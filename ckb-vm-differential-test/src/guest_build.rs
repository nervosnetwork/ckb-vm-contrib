use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::DivergenceError;

const DEFAULT_GUEST_TARGET_TRIPLE: &str = "riscv64imac-unknown-none-elf";
const DEFAULT_GUEST_FEATURE: &str = "__guest";
const ELF_OVERRIDE_ENV: &str = "CKB_VM_DIFFERENTIAL_GUEST_ELF";

/// Knobs for the cargo subprocess that produces the guest ELF.
///
/// Defaults match the canonical ckb-vm guest build:
/// `cargo build --release --target=riscv64imac-unknown-none-elf --features=__guest`.
#[derive(Debug, Clone)]
pub struct BuildConfig {
    pub target_triple: String,
    pub feature: String,
    pub extra_args: Vec<OsString>,
    pub env: Vec<(OsString, OsString)>,
    pub env_remove: Vec<OsString>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            target_triple: DEFAULT_GUEST_TARGET_TRIPLE.into(),
            feature: DEFAULT_GUEST_FEATURE.into(),
            extra_args: Vec::new(),
            env: Vec::new(),
            // Conservative strip list — leakage from the parent cargo can retarget the
            // nested build. TODO: also CARGO_PRIMARY_PACKAGE, CARGO_MANIFEST_DIR, etc.
            env_remove: ["RUSTC_WRAPPER", "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_TARGET"]
                .into_iter()
                .map(OsString::from)
                .collect(),
        }
    }
}

impl BuildConfig {
    pub fn target_triple(mut self, triple: impl Into<String>) -> Self {
        self.target_triple = triple.into();
        self
    }

    pub fn feature(mut self, feature: impl Into<String>) -> Self {
        self.feature = feature.into();
        self
    }

    pub fn arg(mut self, arg: impl Into<OsString>) -> Self {
        self.extra_args.push(arg.into());
        self
    }

    pub fn env(mut self, key: impl Into<OsString>, value: impl Into<OsString>) -> Self {
        self.env.push((key.into(), value.into()));
        self
    }

    pub fn env_remove(mut self, key: impl Into<OsString>) -> Self {
        self.env_remove.push(key.into());
        self
    }
}

/// A fully custom build command for cases where the guest ELF is not produced
/// by a plain `cargo build` invocation.
///
/// The command runs with its working directory set to the crate's manifest
/// directory. After it exits successfully, the framework reads the ELF from
/// `elf_path` (resolved relative to the same manifest directory).
///
/// # Example
///
/// ```ignore
/// build: CustomBuild::new("make", "target/riscv64imac-unknown-none-elf/release/my_contract")
///     .arg("build-guest")
///     .env("RUSTFLAGS", "-C opt-level=z")
/// ```
#[derive(Debug, Clone)]
pub struct CustomBuild {
    pub program: OsString,
    pub args: Vec<OsString>,
    /// Path to the produced ELF, relative to the crate manifest directory.
    pub elf_path: PathBuf,
    pub env: Vec<(OsString, OsString)>,
    pub env_remove: Vec<OsString>,
}

impl CustomBuild {
    pub fn new(program: impl Into<OsString>, elf_path: impl Into<PathBuf>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            elf_path: elf_path.into(),
            env: Vec::new(),
            env_remove: Vec::new(),
        }
    }

    pub fn arg(mut self, arg: impl Into<OsString>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn env(mut self, key: impl Into<OsString>, value: impl Into<OsString>) -> Self {
        self.env.push((key.into(), value.into()));
        self
    }

    pub fn env_remove(mut self, key: impl Into<OsString>) -> Self {
        self.env_remove.push(key.into());
        self
    }
}

/// Selects between the built-in cargo strategy and a fully custom command.
///
/// Both [`BuildConfig`] and [`CustomBuild`] implement `Into<GuestBuild>`, so
/// the `build:` arm of [`harness!`] accepts either type transparently.
#[derive(Debug, Clone)]
pub enum GuestBuild {
    Cargo(BuildConfig),
    Custom(CustomBuild),
}

impl From<BuildConfig> for GuestBuild {
    fn from(c: BuildConfig) -> Self {
        Self::Cargo(c)
    }
}

impl From<CustomBuild> for GuestBuild {
    fn from(c: CustomBuild) -> Self {
        Self::Custom(c)
    }
}

/// Compiles the crate at `manifest_dir` with default `BuildConfig`.
///
/// `CKB_VM_DIFFERENTIAL_GUEST_ELF` short-circuits this and loads from disk —
/// CodeLLDB on Windows crashes if the debugged process spawns cargo.
pub fn build_guest_crate(manifest_dir: &str) -> Result<Vec<u8>, DivergenceError> {
    build_guest_crate_with(manifest_dir, &BuildConfig::default())
}

/// Same as [`build_guest_crate`] but threads a user-supplied [`BuildConfig`].
pub fn build_guest_crate_with(manifest_dir: &str, config: &BuildConfig) -> Result<Vec<u8>, DivergenceError> {
    __build_guest(manifest_dir, config.clone())
}

/// Runs a [`CustomBuild`] command to produce the guest ELF.
///
/// `CKB_VM_DIFFERENTIAL_GUEST_ELF` short-circuits the build the same way it
/// does for [`build_guest_crate_with`].
pub fn build_guest_crate_cmd(manifest_dir: &str, custom: &CustomBuild) -> Result<Vec<u8>, DivergenceError> {
    __build_guest(manifest_dir, custom.clone())
}

/// Dispatch entry-point used by the `harness!` macro. Accepts anything that
/// converts into a [`GuestBuild`] — both [`BuildConfig`] and [`CustomBuild`]
/// qualify — so `harness!`'s `build:` arm works with either type.
#[doc(hidden)]
pub fn __build_guest(manifest_dir: &str, build: impl Into<GuestBuild>) -> Result<Vec<u8>, DivergenceError> {
    if let Some(path) = std::env::var_os(ELF_OVERRIDE_ENV) {
        return std::fs::read(&path).map_err(|e| DivergenceError::Build(format!("{ELF_OVERRIDE_ENV}={path:?}: {e}")));
    }

    let manifest_dir = Path::new(manifest_dir);
    match build.into() {
        GuestBuild::Cargo(config) => run_cargo_strategy(manifest_dir, &config),
        GuestBuild::Custom(custom) => run_custom_strategy(manifest_dir, &custom),
    }
}

fn run_cargo_strategy(manifest_dir: &Path, config: &BuildConfig) -> Result<Vec<u8>, DivergenceError> {
    let manifest_path = manifest_dir.join("Cargo.toml");
    let bin_name = read_bin_name(&manifest_path)?;
    let target_dir = guest_target_dir(manifest_dir, &bin_name);

    run_cargo_build(manifest_dir, &manifest_path, &bin_name, &target_dir, config)?;

    let elf_path = target_dir.join(&config.target_triple).join("release").join(&bin_name);
    std::fs::read(&elf_path)
        .map_err(|e| DivergenceError::Build(format!("reading guest ELF at {}: {e}", elf_path.display())))
}

fn run_custom_strategy(manifest_dir: &Path, custom: &CustomBuild) -> Result<Vec<u8>, DivergenceError> {
    let mut cmd = Command::new(&custom.program);
    for arg in &custom.args {
        cmd.arg(arg);
    }
    cmd.current_dir(manifest_dir);
    for key in &custom.env_remove {
        cmd.env_remove::<&OsStr>(key.as_ref());
    }
    for (key, value) in &custom.env {
        cmd.env::<&OsStr, &OsStr>(key.as_ref(), value.as_ref());
    }

    let output = cmd.output().map_err(|e| DivergenceError::Build(format!("spawning {:?}: {e}", custom.program)))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DivergenceError::Build(format!("{:?} failed:\n{stderr}", custom.program)));
    }

    let elf_path = manifest_dir.join(&custom.elf_path);
    std::fs::read(&elf_path)
        .map_err(|e| DivergenceError::Build(format!("reading guest ELF at {}: {e}", elf_path.display())))
}

fn run_cargo_build(
    manifest_dir: &Path,
    manifest_path: &Path,
    bin_name: &str,
    target_dir: &Path,
    config: &BuildConfig,
) -> Result<(), DivergenceError> {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut cmd = Command::new(cargo);
    cmd.arg("build")
        .arg("--release")
        .arg("--target")
        .arg(&config.target_triple)
        .arg("--features")
        .arg(&config.feature)
        .arg("--bin")
        .arg(bin_name)
        .arg("--manifest-path")
        .arg(manifest_path);
    for arg in &config.extra_args {
        cmd.arg(arg);
    }

    // Pin CWD so cargo's `.cargo/config.toml` walk-up starts from the manifest
    // dir regardless of where the parent test ran.
    cmd.current_dir(manifest_dir);
    cmd.env("CARGO_TARGET_DIR", target_dir);
    for key in &config.env_remove {
        cmd.env_remove::<&OsStr>(key.as_ref());
    }
    for (key, value) in &config.env {
        cmd.env::<&OsStr, &OsStr>(key.as_ref(), value.as_ref());
    }

    let output = cmd.output().map_err(|e| DivergenceError::Build(format!("spawning cargo: {e}")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DivergenceError::Build(format!("cargo build failed:\n{stderr}")));
    }
    Ok(())
}

/// Reads the `[[bin]]` name from a guest manifest. TODO: swap in `cargo_metadata`.
fn read_bin_name(manifest_path: &Path) -> Result<String, DivergenceError> {
    let text = std::fs::read_to_string(manifest_path)
        .map_err(|e| DivergenceError::Build(format!("reading {}: {e}", manifest_path.display())))?;
    let bin_section = text.split("[[bin]]").nth(1);
    if let Some(section) = bin_section {
        for line in section.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("name") {
                if let Some(quoted) = rest.split('=').nth(1) {
                    let name = quoted.trim().trim_matches('"').to_string();
                    if !name.is_empty() {
                        return Ok(name);
                    }
                }
            }
            if line.starts_with('[') {
                break;
            }
        }
    }
    Err(DivergenceError::Build(format!("no [[bin]] name found in {}", manifest_path.display())))
}

fn guest_target_dir(manifest_dir: &Path, bin_name: &str) -> PathBuf {
    manifest_dir.join("target").join(format!("{bin_name}-guest"))
}

# ckb-vm-differential-test

Differential testing harness for libraries ported to ckb-vm: each proptest input is fed to a host-side **reference** and a guest-side **port**; outputs are compared, mismatches surface as typed errors, guest panics are forwarded with `PanicInfo`.

## Quick start

A test crate's `src/lib.rs` declares N `harness!`es and exactly one `entry!`. The file is compiled twice:

- **host lib** — each harness implements [`Harness`].
- **guest bin** — `entry!` emits `_start` and dispatches on `argv[0]`.

Minimal example — [opt-lib/sha256/test](../opt-lib/sha256/test):

```rust
// src/lib.rs
#![cfg_attr(target_arch = "riscv64", no_std)]
#![cfg_attr(target_arch = "riscv64", no_main)]

#[cfg(target_arch = "riscv64")]
use alloc::vec::Vec;

ckb_vm_differential_test::harness! {
    name:      Sha256Harness,
    input:     Vec<u8>,
    output:    [u8; 32],
    port:      |m: &Vec<u8>| ckb_opt_sha256::sha256(m),
    reference: |m: &Vec<u8>| {
        use sha2::Digest;
        sha2::Sha256::digest(m.as_slice()).into()
    },
}

ckb_vm_differential_test::entry!(Sha256Harness);
```

```toml
# Cargo.toml
[package]
name = "sha256_differential"
version = "0.1.0"
edition = "2024"

[workspace]
[features]
__guest = []

[lib]
path = "src/lib.rs"

[[bin]]
name = "sha256_differential"
path = "src/lib.rs"
required-features = ["__guest"]

[dependencies]
ckb-vm-differential-test = { path = "../../../ckb-vm-differential-test" }

[target.'cfg(not(target_arch = "riscv64"))'.dependencies]
sha2 = "0.10"
proptest = "1.5"

[target.'cfg(target_arch = "riscv64")'.dependencies]
ckb-opt-sha256 = { path = "../rust" }
```

```rust
// tests/proptest.rs
use ckb_vm_differential_test::warmstart_check;
use proptest::prelude::*;
use sha256_differential::Sha256Harness;

proptest! {
    #[test]
    fn sha256_matches_reference(input in proptest::collection::vec(any::<u8>(), 0..1024)) {
        warmstart_check::<Sha256Harness>(&input)?;
    }
}
```

```bash
cargo test                              # cross-compiles the guest ELF lazily on first call
PROPTEST_CASES=256 cargo test
```

The first invocation shells out to cargo to cross-compile the guest. Cached at `target/<bin>-guest/`. Failing seeds persist under `tests/regressions/`.

## `harness!`

```text
harness! {
    name:      Ident,                    // host struct name; also the dispatch tag
    input:     Ty,                       // serde + Debug + Clone + 'static
    output:    Ty,                       // serde + Debug + PartialEq + 'static
    port:      |&Input| -> Output,       // guest
    reference: |&Input| -> Output,       // host
    build:     BuildConfig (optional),   // overrides cargo invocation
}
```

Constraints:

- `Output: serde::Deserialize` rules out `[T; N]` for `N > 32`. Use `Vec<u8>` or `serde-big-array`.
- The `reference` body is host-only but token-parsed everywhere; don't import guest-only items inside it.

## Multiple harnesses

Libraries with several primitives share one crate. Each `harness!` gets its own input/output types and host `Harness` impl; `entry!` collects them into one guest binary that dispatches on `argv[0]`. See [opt-lib/fips202/test](../opt-lib/fips202/test):

```rust
ckb_vm_differential_test::harness! { name: Sha3_256Harness, input: Vec<u8>,        output: Vec<u8>, … }
ckb_vm_differential_test::harness! { name: Sha3_512Harness, input: Vec<u8>,        output: Vec<u8>, … }
ckb_vm_differential_test::harness! { name: Shake128Harness, input: (Vec<u8>, u16), output: Vec<u8>, … }
ckb_vm_differential_test::harness! { name: Shake256Harness, input: (Vec<u8>, u16), output: Vec<u8>, … }

ckb_vm_differential_test::entry!(Sha3_256Harness, Sha3_512Harness, Shake128Harness, Shake256Harness);
```

The shared guest ELF is built once per test process via `OnceLock`. `cargo test` parallelizes the four `proptest!` fns; per-thread executor caches keep each thread's warm-start snapshot independent.

## Executors

| Helper                 | Strategy                                                    | Use when                              |
| ---------------------- | ----------------------------------------------------------- | ------------------------------------- |
| `oneshot_check::<H>`   | Fresh VM per input                                          | Diagnosing inter-case state pollution |
| `warmstart_check::<H>` | Boot once, snapshot at first `SIGNAL_READY`, clone per case | Default                               |

`WarmStart` skips ELF parse, allocator init, and run-loop preamble per case. Throughput depends on workload; sha256 with small inputs gains ~30% on 2048 cases.

## Toolchain setup

```bash
rustup target add riscv64imac-unknown-none-elf
```

C cross-compilation honors `cc-rs`'s `CC_<target>` / `AR_<target>` / `CFLAGS_<target>` env vars. Wire per-crate via `.cargo/config.toml`:

```toml
# RISC-V GCC toolchain (xPack)
[env]
CC_riscv64imac_unknown_none_elf = "riscv-none-elf-gcc.exe"
AR_riscv64imac_unknown_none_elf = "riscv-none-elf-ar.exe"

# clang alternative
# CC_…  = "clang"
# AR_…  = "llvm-ar"
# CFLAGS_… = "--target=riscv64-unknown-none-elf"
```

Library crates that hardcode `clang_finder::find()` need a small build.rs guard so they fall back to `cc-rs`'s env-var path:

```rust
let cc_env = format!("CC_{}", env::var("TARGET")?.replace('-', "_"));
if env::var_os(&cc_env).is_none() && env::var_os("CC").is_none() {
    builder.compiler(clang_finder::find());
}
```

## Debugging

CodeLLDB on Windows crashes on the cargo/rustc/linker child process tree. Pre-build the guest and bypass the subprocess:

```bash
cargo build --release --target=riscv64imac-unknown-none-elf --features=__guest --bin=<name>
```

```jsonc
// .vscode/launch.json
"env": { "CKB_VM_DIFFERENTIAL_GUEST_ELF": "${workspaceFolder}/.../<name>" }
```

Host-side breakpoints hit: proptest body, `reference` closure, `OneShot` / `WarmStart` / `DifferentialSyscalls::handle_*`, `build_guest_crate`. They do **not** hit the `port` closure or [src/guest.rs](src/guest.rs) — those run inside the interpreter as RISC-V instructions. To inspect guest state at a syscall, break in the relevant `handle_*` and read `machine.registers()` / `machine.memory_mut()`.

## Protocol

| a7       | Name                   | a0 in/out             | a1       | Semantics                                                       |
| -------- | ---------------------- | --------------------- | -------- | --------------------------------------------------------------- |
| `0xF000` | `SYSCALL_READ_INPUT`   | buf ptr / written len | capacity | Two-step: `cap=0` probes size; `cap >= size` copies bytes.      |
| `0xF001` | `SYSCALL_WRITE_OUTPUT` | buf ptr               | length   | Stores bytes, calls `set_running(false)`.                       |
| `0xF002` | `SYSCALL_SIGNAL_READY` | —                     | —        | No-op for `OneShot`; snapshot point for `WarmStart`.            |
| `0xF003` | `SYSCALL_PANIC`        | utf-8 ptr             | length   | Stores formatted panic, ends run. Priority over `WRITE_OUTPUT`. |

Payloads are `postcard`-encoded; both sides fix serde features to `derive + alloc`. Format is unversioned — coordinated edits across `protocol.rs`, `guest.rs`, and `executor.rs` are required for ABI changes.

## `DivergenceError`

| Variant           | Trigger                                           | Carries                                   |
| ----------------- | ------------------------------------------------- | ----------------------------------------- |
| `OutputMismatch`  | reference ≠ guest                                 | Debug-formatted input / reference / guest |
| `GuestExited`     | VM terminated without `WRITE_OUTPUT` and no panic | exit code                                 |
| `GuestPanicked`   | guest issued `SYSCALL_PANIC`                      | formatted `PanicInfo`                     |
| `Vm`              | ckb-vm error                                      | `ckb_vm::Error`                           |
| `Decode`          | guest output couldn't deserialize                 | `postcard::Error`                         |
| `PayloadTooLarge` | input/output exceeds `MAX_PAYLOAD_LEN`            | sizes                                     |
| `Build`           | cargo subprocess failed                           | stderr                                    |

Forwarded to proptest as `TestCaseError::fail(e.to_string())`.

## Environment variables

| Name                                            | Effect                                                                                                                                    |
| ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `CKB_VM_DIFFERENTIAL_GUEST_ELF`                 | Path to a pre-built ELF; skips the cargo subprocess. Stale values cause misleading "path not found" errors — pin per-crate, not globally. |
| `PROPTEST_CASES`                                | Standard proptest knob.                                                                                                                   |
| `RUST_BACKTRACE`                                | Surfaces backtraces on host panics; not on guest ones (those route through `SYSCALL_PANIC`).                                              |
| `CC_<target>`, `AR_<target>`, `CFLAGS_<target>` | `cc-rs` overrides; set per-crate via `.cargo/config.toml`'s `[env]`.                                                                      |

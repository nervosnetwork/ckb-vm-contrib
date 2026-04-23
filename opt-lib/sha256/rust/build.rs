use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let target_underscored = target.replace('-', "_");
    let cc_env = format!("CC_{target_underscored}");

    let mut builder = cc::Build::new();
    if env::var_os(&cc_env).is_none() && env::var_os("CC").is_none() {
        builder.compiler(clang_finder::find());
    }
    builder.file("src/sha256.c").include("src").opt_level(3);
    if target.contains("riscv64") {
        builder.flag("-march=rv64imc_zba_zbb_zbc_zbs");
    }
    builder.compile("sha256");

    println!("cargo:rerun-if-changed=src/sha256.c");
    println!("cargo:rerun-if-changed=src/sha256.h");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed={cc_env}");
    println!("cargo:rerun-if-env-changed=CC");
}

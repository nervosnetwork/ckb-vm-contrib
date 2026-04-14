use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    let mut builder = cc::Build::new();
    builder.compiler(clang_finder::find());
    builder.file("src/fips202.c").include("src").opt_level(3);
    if target.contains("riscv64") {
        builder.flag("-march=rv64imc_zba_zbb_zbc_zbs");
    }
    builder.compile("fips202");

    println!("cargo:rerun-if-changed=src/fips202.c");
    println!("cargo:rerun-if-changed=src/fips202.h");
    println!("cargo:rerun-if-changed=build.rs");
}

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    let mut builder = cc::Build::new();
    builder.compiler(clang_finder::find());
    builder.file("src/blake2b-ref.c").include("src").opt_level(3);
    builder.define("NATIVE_LITTLE_ENDIAN", None);
    if target.contains("riscv64") {
        builder.flag("-march=rv64imc_zba_zbb_zbc_zbs");
    }
    builder.compile("blake2b");

    println!("cargo:rerun-if-changed=src/blake2b-ref.c");
    println!("cargo:rerun-if-changed=src/blake2.h");
    println!("cargo:rerun-if-changed=src/blake2-impl.h");
    println!("cargo:rerun-if-changed=build.rs");
}

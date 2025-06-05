#[macro_use]
extern crate criterion;

use ckb_vm::{
    Bytes, ISA_B, ISA_IMC, ISA_MOP, RISCV_MAX_MEMORY, SparseMemory,
    machine::{
        DefaultMachineBuilder, VERSION2,
        asm::{AsmCoreMachine, AsmMachine},
    },
    run,
};
use criterion::Criterion;
use std::fs;
use std::path::Path;
use std::process::Command;

const BINARY_PATH_ED25519: &str = "../ckb-vm-bench-scripts/build/release/ed25519_ckbvm";
const BINARY_PATH_K256_ECDSA: &str = "../ckb-vm-bench-scripts/build/release/k256_ecdsa_ckbvm";
const BINARY_PATH_K256_SCHNORR: &str = "../ckb-vm-bench-scripts/build/release/k256_schnorr_ckbvm";
const BINARY_PATH_P256: &str = "../ckb-vm-bench-scripts/build/release/p256_ckbvm";
const BINARY_PATH_RSA: &str = "../ckb-vm-bench-scripts/build/release/rsa_ckbvm";
const BINARY_PATH_SECP256K1_ECDSA: &str = "../ckb-vm-bench-scripts/build/release/secp256k1_ecdsa_ckbvm";
const BINARY_PATH_SECP256K1_SCHNORR: &str = "../ckb-vm-bench-scripts/build/release/secp256k1_schnorr_ckbvm";
const BINARY_PATH_SPHINCSPLUS_REF: &str = "../ckb-vm-bench-scripts/build/release/sphincsplus_ref_ckbvm";
const NATIVE_PATH_ED25519: &str = "../ckb-vm-bench-scripts/build/release/ed25519_native";
const NATIVE_PATH_K256_ECDSA: &str = "../ckb-vm-bench-scripts/build/release/k256_ecdsa_native";
const NATIVE_PATH_K256_SCHNORR: &str = "../ckb-vm-bench-scripts/build/release/k256_schnorr_native";
const NATIVE_PATH_P256: &str = "../ckb-vm-bench-scripts/build/release/p256_native";
const NATIVE_PATH_RSA: &str = "../ckb-vm-bench-scripts/build/release/rsa_native";
const NATIVE_PATH_SECP256K1_ECDSA: &str = "../ckb-vm-bench-scripts/build/release/secp256k1_ecdsa_native";
const NATIVE_PATH_SECP256K1_SCHNORR: &str = "../ckb-vm-bench-scripts/build/release/secp256k1_schnorr_native";
const NATIVE_PATH_SPHINCSPLUS_REF: &str = "../ckb-vm-bench-scripts/build/release/sphincsplus_ref_native";

fn asm_ed25519(c: &mut Criterion) {
    c.bench_function("asm_ed25519", |b| {
        let buffer = fs::read(BINARY_PATH_ED25519).unwrap().into();
        b.iter(|| run_asm(&buffer));
    });
}

fn asm_k256_ecdsa(c: &mut Criterion) {
    c.bench_function("asm_k256_ecdsa", |b| {
        let buffer = fs::read(BINARY_PATH_K256_ECDSA).unwrap().into();
        b.iter(|| run_asm(&buffer));
    });
}

fn asm_k256_schnorr(c: &mut Criterion) {
    c.bench_function("asm_k256_schnorr", |b| {
        let buffer = fs::read(BINARY_PATH_K256_SCHNORR).unwrap().into();
        b.iter(|| run_asm(&buffer));
    });
}

fn asm_p256(c: &mut Criterion) {
    c.bench_function("asm_p256", |b| {
        let buffer = fs::read(BINARY_PATH_P256).unwrap().into();
        b.iter(|| run_asm(&buffer));
    });
}

fn asm_rsa(c: &mut Criterion) {
    c.bench_function("asm_rsa", |b| {
        let buffer = fs::read(BINARY_PATH_RSA).unwrap().into();
        b.iter(|| run_asm(&buffer));
    });
}

fn asm_secp256k1_ecdsa(c: &mut Criterion) {
    c.bench_function("asm_secp256k1_ecdsa", |b| {
        let buffer = fs::read(BINARY_PATH_SECP256K1_ECDSA).unwrap().into();
        b.iter(|| run_asm(&buffer));
    });
}

fn asm_secp256k1_schnorr(c: &mut Criterion) {
    c.bench_function("asm_secp256k1_schnorr", |b| {
        let buffer = fs::read(BINARY_PATH_SECP256K1_SCHNORR).unwrap().into();
        b.iter(|| run_asm(&buffer));
    });
}

fn asm_sphincsplus_ref(c: &mut Criterion) {
    c.bench_function("asm_sphincsplus_ref", |b| {
        let buffer = fs::read(BINARY_PATH_SPHINCSPLUS_REF).unwrap().into();
        b.iter(|| run_asm(&buffer));
    });
}

fn interpret_ed25519(c: &mut Criterion) {
    c.bench_function("interpret_ed25519", |b| {
        let buffer = fs::read(BINARY_PATH_ED25519).unwrap().into();
        b.iter(|| run_interpret(&buffer));
    });
}

fn interpret_k256_ecdsa(c: &mut Criterion) {
    c.bench_function("interpret_k256_ecdsa", |b| {
        let buffer = fs::read(BINARY_PATH_K256_ECDSA).unwrap().into();
        b.iter(|| run_interpret(&buffer));
    });
}

fn interpret_k256_schnorr(c: &mut Criterion) {
    c.bench_function("interpret_k256_schnorr", |b| {
        let buffer = fs::read(BINARY_PATH_K256_SCHNORR).unwrap().into();
        b.iter(|| run_interpret(&buffer));
    });
}

fn interpret_p256(c: &mut Criterion) {
    c.bench_function("interpret_p256", |b| {
        let buffer = fs::read(BINARY_PATH_P256).unwrap().into();
        b.iter(|| run_interpret(&buffer));
    });
}

fn interpret_rsa(c: &mut Criterion) {
    c.bench_function("interpret_rsa", |b| {
        let buffer = fs::read(BINARY_PATH_RSA).unwrap().into();
        b.iter(|| run_interpret(&buffer));
    });
}

fn interpret_secp256k1_ecdsa(c: &mut Criterion) {
    c.bench_function("interpret_secp256k1_ecdsa", |b| {
        let buffer = fs::read(BINARY_PATH_SECP256K1_ECDSA).unwrap().into();
        b.iter(|| run_interpret(&buffer));
    });
}

fn interpret_secp256k1_schnorr(c: &mut Criterion) {
    c.bench_function("interpret_secp256k1_schnorr", |b| {
        let buffer = fs::read(BINARY_PATH_SECP256K1_SCHNORR).unwrap().into();
        b.iter(|| run_interpret(&buffer));
    });
}

fn interpret_sphincsplus_ref(c: &mut Criterion) {
    c.bench_function("interpret_sphincsplus_ref", |b| {
        let buffer = fs::read(BINARY_PATH_SPHINCSPLUS_REF).unwrap().into();
        b.iter(|| run_interpret(&buffer));
    });
}

fn mop_ed25519(c: &mut Criterion) {
    c.bench_function("mop_ed25519", |b| {
        let buffer = fs::read(BINARY_PATH_ED25519).unwrap().into();
        b.iter(|| run_mop(&buffer));
    });
}

fn mop_k256_ecdsa(c: &mut Criterion) {
    c.bench_function("mop_k256_ecdsa", |b| {
        let buffer = fs::read(BINARY_PATH_K256_ECDSA).unwrap().into();
        b.iter(|| run_mop(&buffer));
    });
}

fn mop_k256_schnorr(c: &mut Criterion) {
    c.bench_function("mop_k256_schnorr", |b| {
        let buffer = fs::read(BINARY_PATH_K256_SCHNORR).unwrap().into();
        b.iter(|| run_mop(&buffer));
    });
}

fn mop_p256(c: &mut Criterion) {
    c.bench_function("mop_p256", |b| {
        let buffer = fs::read(BINARY_PATH_P256).unwrap().into();
        b.iter(|| run_mop(&buffer));
    });
}

fn mop_rsa(c: &mut Criterion) {
    c.bench_function("mop_rsa", |b| {
        let buffer = fs::read(BINARY_PATH_RSA).unwrap().into();
        b.iter(|| run_mop(&buffer));
    });
}

fn mop_secp256k1_ecdsa(c: &mut Criterion) {
    c.bench_function("mop_secp256k1_ecdsa", |b| {
        let buffer = fs::read(BINARY_PATH_SECP256K1_ECDSA).unwrap().into();
        b.iter(|| run_mop(&buffer));
    });
}

fn mop_secp256k1_schnorr(c: &mut Criterion) {
    c.bench_function("mop_secp256k1_schnorr", |b| {
        let buffer = fs::read(BINARY_PATH_SECP256K1_SCHNORR).unwrap().into();
        b.iter(|| run_mop(&buffer));
    });
}

fn mop_sphincsplus_ref(c: &mut Criterion) {
    c.bench_function("mop_sphincsplus_ref", |b| {
        let buffer = fs::read(BINARY_PATH_SPHINCSPLUS_REF).unwrap().into();
        b.iter(|| run_mop(&buffer));
    });
}

fn native_ed25519(c: &mut Criterion) {
    c.bench_function("native_ed25519", |b| {
        b.iter(|| run_native(NATIVE_PATH_ED25519));
    });
}

fn native_k256_ecdsa(c: &mut Criterion) {
    c.bench_function("native_k256_ecdsa", |b| {
        b.iter(|| run_native(NATIVE_PATH_K256_ECDSA));
    });
}

fn native_k256_schnorr(c: &mut Criterion) {
    c.bench_function("native_k256_schnorr", |b| {
        b.iter(|| run_native(NATIVE_PATH_K256_SCHNORR));
    });
}

fn native_p256(c: &mut Criterion) {
    c.bench_function("native_p256", |b| {
        b.iter(|| run_native(NATIVE_PATH_P256));
    });
}

fn native_rsa(c: &mut Criterion) {
    c.bench_function("native_rsa", |b| {
        b.iter(|| run_native(NATIVE_PATH_RSA));
    });
}

fn native_secp256k1_ecdsa(c: &mut Criterion) {
    c.bench_function("native_secp256k1_ecdsa", |b| {
        b.iter(|| run_native(NATIVE_PATH_SECP256K1_ECDSA));
    });
}

fn native_secp256k1_schnorr(c: &mut Criterion) {
    c.bench_function("native_secp256k1_schnorr", |b| {
        b.iter(|| run_native(NATIVE_PATH_SECP256K1_SCHNORR));
    });
}

fn native_sphincsplus_ref(c: &mut Criterion) {
    c.bench_function("native_sphincsplus_ref", |b| {
        b.iter(|| run_native(NATIVE_PATH_SPHINCSPLUS_REF));
    });
}

fn run_asm(program: &Bytes) {
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B, VERSION2, u64::MAX);
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine.load_program(&program, [].into_iter()).unwrap();
    let ret = machine.run().unwrap();
    assert_eq!(ret, 0);
}

fn run_interpret(program: &Bytes) {
    let ret = run::<u64, SparseMemory<u64>>(&program, &[], RISCV_MAX_MEMORY).unwrap();
    assert_eq!(ret, 0);
}

fn run_mop(program: &Bytes) {
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B | ISA_MOP, VERSION2, u64::MAX);
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine.load_program(&program, [].into_iter()).unwrap();
    let ret = machine.run().unwrap();
    assert_eq!(ret, 0);
}

fn run_native<P: AsRef<Path>>(path: P) {
    assert!(Command::new(path.as_ref()).status().unwrap().success());
}

criterion_group!(
    benches,
    asm_ed25519,
    asm_k256_ecdsa,
    asm_k256_schnorr,
    asm_p256,
    asm_rsa,
    asm_secp256k1_ecdsa,
    asm_secp256k1_schnorr,
    asm_sphincsplus_ref,
    interpret_ed25519,
    interpret_k256_ecdsa,
    interpret_k256_schnorr,
    interpret_p256,
    interpret_rsa,
    interpret_secp256k1_ecdsa,
    interpret_secp256k1_schnorr,
    interpret_sphincsplus_ref,
    mop_ed25519,
    mop_k256_ecdsa,
    mop_k256_schnorr,
    mop_p256,
    mop_rsa,
    mop_secp256k1_ecdsa,
    mop_secp256k1_schnorr,
    mop_sphincsplus_ref,
    native_ed25519,
    native_k256_ecdsa,
    native_k256_schnorr,
    native_p256,
    native_rsa,
    native_secp256k1_ecdsa,
    native_secp256k1_schnorr,
    native_sphincsplus_ref,
);
criterion_main!(benches);

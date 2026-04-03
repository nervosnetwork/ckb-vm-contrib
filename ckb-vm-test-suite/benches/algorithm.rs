use ckb_vm_test_suite::{
    BINARY_PATH_ED25519, BINARY_PATH_K256_ECDSA, BINARY_PATH_K256_SCHNORR, BINARY_PATH_P256, BINARY_PATH_RSA,
    BINARY_PATH_SECP256K1_ECDSA, BINARY_PATH_SECP256K1_SCHNORR, BINARY_PATH_SPHINCSPLUS_REF, NATIVE_PATH_ED25519,
    NATIVE_PATH_K256_ECDSA, NATIVE_PATH_K256_SCHNORR, NATIVE_PATH_P256, NATIVE_PATH_RSA, NATIVE_PATH_SECP256K1_ECDSA,
    NATIVE_PATH_SECP256K1_SCHNORR, NATIVE_PATH_SPHINCSPLUS_REF, run_asm, run_interpret, run_mop, run_native,
};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use criterion_cycles_per_byte::CyclesPerByte;
use std::fs;

const VM_ALGORITHMS: &[(&str, &str)] = &[
    ("ed25519", BINARY_PATH_ED25519),
    ("k256_ecdsa", BINARY_PATH_K256_ECDSA),
    ("k256_schnorr", BINARY_PATH_K256_SCHNORR),
    ("p256", BINARY_PATH_P256),
    ("rsa", BINARY_PATH_RSA),
    ("secp256k1_ecdsa", BINARY_PATH_SECP256K1_ECDSA),
    ("secp256k1_schnorr", BINARY_PATH_SECP256K1_SCHNORR),
    ("sphincsplus_ref", BINARY_PATH_SPHINCSPLUS_REF),
];

const NATIVE_ALGORITHMS: &[(&str, &str)] = &[
    ("ed25519", NATIVE_PATH_ED25519),
    ("k256_ecdsa", NATIVE_PATH_K256_ECDSA),
    ("k256_schnorr", NATIVE_PATH_K256_SCHNORR),
    ("p256", NATIVE_PATH_P256),
    ("rsa", NATIVE_PATH_RSA),
    ("secp256k1_ecdsa", NATIVE_PATH_SECP256K1_ECDSA),
    ("secp256k1_schnorr", NATIVE_PATH_SECP256K1_SCHNORR),
    ("sphincsplus_ref", NATIVE_PATH_SPHINCSPLUS_REF),
];

fn asm_benchmarks(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("asm");
    for &(name, path) in VM_ALGORITHMS {
        let buffer = fs::read(path).unwrap();
        group.throughput(Throughput::Bytes(buffer.len() as u64));
        let buffer = buffer.into();
        group.bench_function(name, |b| {
            b.iter(|| run_asm(&buffer));
        });
    }
    group.finish();
}

fn interpret_benchmarks(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("interpret");
    for &(name, path) in VM_ALGORITHMS {
        let buffer = fs::read(path).unwrap();
        group.throughput(Throughput::Bytes(buffer.len() as u64));
        let buffer = buffer.into();
        group.bench_function(name, |b| {
            b.iter(|| run_interpret(&buffer));
        });
    }
    group.finish();
}

fn mop_benchmarks(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("mop");
    for &(name, path) in VM_ALGORITHMS {
        let buffer = fs::read(path).unwrap();
        group.throughput(Throughput::Bytes(buffer.len() as u64));
        let buffer = buffer.into();
        group.bench_function(name, |b| {
            b.iter(|| run_mop(&buffer));
        });
    }
    group.finish();
}

fn native_benchmarks(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("native");
    for &(name, path) in NATIVE_ALGORITHMS {
        let size = fs::metadata(path).unwrap().len();
        group.throughput(Throughput::Bytes(size));
        group.bench_function(name, |b| {
            b.iter(|| run_native(path));
        });
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = asm_benchmarks, interpret_benchmarks, mop_benchmarks, native_benchmarks
);
criterion_main!(benches);

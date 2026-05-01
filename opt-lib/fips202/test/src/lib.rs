#![cfg_attr(target_arch = "riscv64", no_std)]
#![cfg_attr(target_arch = "riscv64", no_main)]

#[cfg(target_arch = "riscv64")]
use alloc::vec;
#[cfg(target_arch = "riscv64")]
use alloc::vec::Vec;

ckb_vm_differential_test::harness! {
    name:      Sha3_256Harness,
    input:     Vec<u8>,
    output:    Vec<u8>,
    port:      |m: &Vec<u8>| {
        let mut h = ckb_opt_fips202::Sha3_256::new();
        h.update(m);
        let mut out = [0u8; 32];
        h.finalize(&mut out);
        out.to_vec()
    },
    reference: |m: &Vec<u8>| {
        use sha3::digest::Digest;
        let mut h = sha3::Sha3_256::new();
        Digest::update(&mut h, m);
        h.finalize().to_vec()
    },
}

ckb_vm_differential_test::harness! {
    name:      Sha3_512Harness,
    input:     Vec<u8>,
    output:    Vec<u8>,
    port:      |m: &Vec<u8>| {
        let mut h = ckb_opt_fips202::Sha3_512::new();
        h.update(m);
        let mut out = [0u8; 64];
        h.finalize(&mut out);
        out.to_vec()
    },
    reference: |m: &Vec<u8>| {
        use sha3::digest::Digest;
        let mut h = sha3::Sha3_512::new();
        Digest::update(&mut h, m);
        h.finalize().to_vec()
    },
}

ckb_vm_differential_test::harness! {
    name:      Shake128Harness,
    input:     (Vec<u8>, u16),
    output:    Vec<u8>,
    port:      |t: &(Vec<u8>, u16)| {
        let mut h = ckb_opt_fips202::Shake128::new();
        h.absorb(&t.0);
        h.finalize();
        let mut out = vec![0u8; t.1 as usize];
        h.squeeze(&mut out);
        out
    },
    reference: |t: &(Vec<u8>, u16)| {
        use sha3::digest::{ExtendableOutput, Update, XofReader};
        let mut h = sha3::Shake128::default();
        Update::update(&mut h, &t.0);
        let mut reader = h.finalize_xof();
        let mut out = vec![0u8; t.1 as usize];
        reader.read(&mut out);
        out
    },
}

ckb_vm_differential_test::harness! {
    name:      Shake256Harness,
    input:     (Vec<u8>, u16),
    output:    Vec<u8>,
    port:      |t: &(Vec<u8>, u16)| {
        let mut h = ckb_opt_fips202::Shake256::new();
        h.absorb(&t.0);
        h.finalize();
        let mut out = vec![0u8; t.1 as usize];
        h.squeeze(&mut out);
        out
    },
    reference: |t: &(Vec<u8>, u16)| {
        use sha3::digest::{ExtendableOutput, Update, XofReader};
        let mut h = sha3::Shake256::default();
        Update::update(&mut h, &t.0);
        let mut reader = h.finalize_xof();
        let mut out = vec![0u8; t.1 as usize];
        reader.read(&mut out);
        out
    },
}

ckb_vm_differential_test::entry!(Sha3_256Harness, Sha3_512Harness, Shake128Harness, Shake256Harness);

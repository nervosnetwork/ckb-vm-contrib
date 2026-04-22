#![cfg_attr(target_arch = "riscv64", no_std)]
#![cfg_attr(target_arch = "riscv64", no_main)]

#[cfg(target_arch = "riscv64")]
extern crate alloc;

#[cfg(target_arch = "riscv64")]
use alloc::vec::Vec;

ckb_vm_differential::harness! {
    name:      Sha256Harness,
    input:     Vec<u8>,
    output:    [u8; 32],
    port:      |m: &Vec<u8>| ckb_opt_sha256::sha256(m),
    reference: |m: &Vec<u8>| {
        use sha2::Digest;
        sha2::Sha256::digest(m.as_slice()).into()
    },
}

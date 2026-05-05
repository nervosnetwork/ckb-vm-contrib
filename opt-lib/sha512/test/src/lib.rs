#![cfg_attr(target_arch = "riscv64", no_std)]
#![cfg_attr(target_arch = "riscv64", no_main)]

#[cfg(target_arch = "riscv64")]
use alloc::vec::Vec;

ckb_vm_differential_test::harness! {
    name:      Sha512Harness,
    input:     Vec<u8>,
    output:    Vec<u8>,
    port:      |m: &Vec<u8>| ckb_opt_sha512::sha512(m).to_vec(),
    reference: |m: &Vec<u8>| {
        use sha2::Digest;
        sha2::Sha512::digest(m.as_slice()).to_vec()
    },
}

ckb_vm_differential_test::entry!(Sha512Harness);

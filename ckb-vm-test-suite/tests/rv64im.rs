use ckb_vm_test_suite::{
    BINARY_PATH_ED25519, BINARY_PATH_K256_ECDSA, BINARY_PATH_K256_SCHNORR, BINARY_PATH_P256, BINARY_PATH_RSA,
    BINARY_PATH_SECP256K1_ECDSA, BINARY_PATH_SECP256K1_SCHNORR, BINARY_PATH_SPHINCSPLUS_REF, run_asm_rv64im,
};
use std::fs;

#[test]
pub fn test_rv64im_ed25519() {
    let buffer = fs::read(BINARY_PATH_ED25519).unwrap().into();
    run_asm_rv64im(&buffer);
}

#[test]
pub fn test_rv64im_k256_ecdsa() {
    let buffer = fs::read(BINARY_PATH_K256_ECDSA).unwrap().into();
    run_asm_rv64im(&buffer);
}

#[test]
pub fn test_rv64im_k256_schnorr() {
    let buffer = fs::read(BINARY_PATH_K256_SCHNORR).unwrap().into();
    run_asm_rv64im(&buffer);
}

#[test]
pub fn test_rv64im_p256() {
    let buffer = fs::read(BINARY_PATH_P256).unwrap().into();
    run_asm_rv64im(&buffer);
}

#[test]
pub fn test_rv64im_rsa() {
    let buffer = fs::read(BINARY_PATH_RSA).unwrap().into();
    run_asm_rv64im(&buffer);
}

#[test]
pub fn test_rv64im_secp256k1_ecdsa() {
    let buffer = fs::read(BINARY_PATH_SECP256K1_ECDSA).unwrap().into();
    run_asm_rv64im(&buffer);
}

#[test]
pub fn test_rv64im_secp256k1_schnorr() {
    let buffer = fs::read(BINARY_PATH_SECP256K1_SCHNORR).unwrap().into();
    run_asm_rv64im(&buffer);
}

#[test]
pub fn test_rv64im_sphincsplus_ref() {
    let buffer = fs::read(BINARY_PATH_SPHINCSPLUS_REF).unwrap().into();
    run_asm_rv64im(&buffer);
}

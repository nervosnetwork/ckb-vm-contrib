#![no_main]
sp1_zkvm::entrypoint!(main);

mod k256_ecdsa;
use k256_ecdsa::execute;

fn main() {
    let n = 1;
    for _ in 0..n {
        assert_eq!(execute(), 0);
    }
    // dummy
    let exit_code: i8 = 0;
    let cycles: u64 = 0;
    sp1_zkvm::io::commit(&exit_code);
    sp1_zkvm::io::commit(&cycles);
}

mod k256_ecdsa;
use k256_ecdsa::program_entry;

fn main() {
    assert_eq!(program_entry(), 0);
}

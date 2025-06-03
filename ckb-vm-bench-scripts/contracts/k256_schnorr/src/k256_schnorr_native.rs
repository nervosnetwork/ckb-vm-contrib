mod k256_schnorr;
use k256_schnorr::program_entry;

fn main() {
    assert_eq!(program_entry(), 0);
}

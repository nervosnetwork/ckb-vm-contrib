mod p256;
use p256::program_entry;

fn main() {
    assert_eq!(program_entry(), 0);
}

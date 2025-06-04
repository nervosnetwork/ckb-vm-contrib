mod ed25519;
use ed25519::program_entry;

fn main() {
    assert_eq!(program_entry(), 0);
}

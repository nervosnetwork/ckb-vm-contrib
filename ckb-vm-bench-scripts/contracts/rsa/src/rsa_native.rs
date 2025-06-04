mod rsa;
use rsa::program_entry;

fn main() {
    assert_eq!(program_entry(), 0);
}

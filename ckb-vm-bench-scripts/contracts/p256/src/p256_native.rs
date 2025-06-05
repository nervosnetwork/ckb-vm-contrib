mod p256;
use p256::execute;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n = if args.len() == 1 { 1 } else { args[1].parse().unwrap() };
    for _ in 0..n {
        assert_eq!(execute(), 0);
    }
}

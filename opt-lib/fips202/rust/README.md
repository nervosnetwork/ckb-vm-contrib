# CKB-FIPS202 Rust Bindings

Rust bindings for the optimized FIPS202 implementation for CKB-VM.

## Supported APIs

- `Shake128`
- `Shake256`

## Performance benchmark

The project we compared is <https://crates.io/crates/sha3>


```sh
# Shake128
$ cargo run --release --example ckb_opt_fips202_shake128 # All cycles: 206253618(196.7M)
$ cargo run --release --example sha3_shake128            # All cycles: 496003644(473.0M)

# Shake256
$ cargo run --release --example ckb_opt_fips202_shake256 # All cycles: 206253627(196.7M)
$ cargo run --release --example sha3_shake256            # All cycles: 487403648(464.8M)
```

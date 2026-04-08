# CKB-FIPS202 Rust Bindings

Rust bindings for the optimized FIPS202 implementation for CKB-VM.


## Performance benchmark

The project we compared is <https://crates.io/crates/sha3>

```sh
$ cargo run --release --example ckb_opt_fips202_shake256 # All cycles: 206003627(196.5M)
$ cargo run --release --example sha3_shake256            # All cycles: 487403648(464.8M)
```

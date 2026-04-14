# CKB-FIPS202 Rust Bindings

Rust bindings for the optimized FIPS202 implementation for CKB-VM.

## Performance benchmark

The project we compared is [sha3](https://crates.io/crates/sha3).

```sh
# Shake128
$ cargo run --release --example ckb_opt_fips202_shake128 # All cycles: 149207038(142.3M)
$ cargo run --release --example sha3_shake128            # All cycles: 496003644(473.0M)

# Shake256
$ cargo run --release --example ckb_opt_fips202_shake256 # All cycles: 149207046(142.3M)
$ cargo run --release --example sha3_shake256            # All cycles: 487403648(464.8M)
```

Our research found that in the sha3 crate's Shake128 and Shake256 implementations, the performance bottleneck is in the [finalize_xof](https://docs.rs/sha3/0.11.0/sha3/struct.Shake128.html#method.finalize_xof) method, while the rest of the implementation performs similarly to ckb_opt_fips202. In typical hash-tree scenarios, ckb_opt_fips202 has a clear advantage, delivering nearly a 3.3x performance improvement.

# CKB-FIPS202 Rust Bindings

Rust bindings for the optimized FIPS202 implementation for CKB-VM.

## Currently Supported

- SHAKE256 absorb/squeeze API

## Running Tests

```sh
cargo test
```

## Running Benchmarks

```sh
cargo bench
```

The benchmark includes a baseline implementation from the `sha3` crate.

## License

MIT

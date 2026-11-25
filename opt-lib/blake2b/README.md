# BLAKE2b for CKB-VM

This folder contains the BLAKE2b reference implementation for CKB-VM.

The C source (`blake2b-ref.c`) is taken from the official
[BLAKE2 reference implementation](https://github.com/BLAKE2/BLAKE2) by Samuel Neves,
licensed under CC0 / OpenSSL / Apache 2.0 at your option.

## Build & Run

### Setup Dependencies

```sh
$ cd $ROOT/deps/musl
$ CLANG=clang-19 bash ckb/build.sh

$ cd $ROOT/deps/compiler-rt-builtins-riscv
$ make CC=clang-19 AR=llvm-ar-19
```

### Run Tests

```sh
$ make run
```

## Rust Bindings

See [`rust/`](rust/) for the Rust crate with safe bindings.

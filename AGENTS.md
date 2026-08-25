# AGENTS.md

## Project Overview

`ckb-vm-contrib` is a Rust workspace focused on community tools, testing frameworks, analysis scripts, and experimental extensions for CKB-VM. It does not implement the core CKB-VM runtime itself; instead, it provides supporting components for VM development, validation, debugging, and performance analysis.

The current workspace mainly includes:

- `ckb-mock-tx-types`: mock CKB transaction environments and script execution contexts
- `ckb-script-size-analyzer`: analyze and optimize smart contract binary size
- `ckb-vm-differential-test`: differential testing across different VM implementations or execution paths
- `ckb-vm-fuzzing-utils`: fuzzing utilities and shared abstractions
- `ckb-vm-syscall-tracer`: trace and analyze syscall behavior in CKB scripts
- `ckb-vm-test-suite`: VM test, compatibility, and benchmark entry points
- `ckb-x64-simulator`: simulate CKB smart contract execution in an x64 environment
- `protobuf-ckb-syscalls`: protobuf definitions and generated code for syscalls
- `spike-sys`: Rust FFI bindings for the RISC-V ISA simulator

This repository depends on Rust 1.95.0, the RISC-V toolchain, and submodule dependencies under `deps/`; many builds and tests require these environments to be prepared in advance.

## Development and Build Commands

**ckb-mock-tx-types**

```bash
# Build
$ cargo build
```

**ckb-script-size-analyzer**

```bash
# Build
$ cargo build

# Usage: sort by name (default)
$ cargo run -- --input <path-to-binary-with-debug-info>
# Symbol: _Exit, size: 50
# Symbol: ___errno_location, size: 8
# Symbol: __addtf3, size: 1050

# Usage: sort by size
$ cargo run -- --input <path-to-binary-with-debug-info> --sort size
# Symbol: sha2::sha512::compress512::hf28740838ff5f6b3, size: 13102
# Symbol: __ckb_std_main, size: 4398
# Symbol: curve25519_dalek::field::<impl curve25519_dalek::backend::serial::u64::field::FieldElement51>::sqrt_ratio_i::hcdb99974122f8072, size: 2990
# ...
```

**ckb-vm-differential-test**

```bash
# Build
$ cargo build
```

**ckb-vm-fuzzing-utils**

```bash
# Build
$ cargo build
```

**ckb-vm-syscall-tracer**

```bash
# Build
$ cargo build

# Example tracing workflow
$ cargo run --bin ckb-vm-syscall-tracer -- --tx-file mock_tx.json --script-hash <hash> --output /tmp/out
$ cargo run --bin ckb-vm-syscall-reader -- /tmp/out/vm_0_0.traces
```

**ckb-vm-test-suite**

```bash
# Build
$ cargo build

# Build test programs. Required before many VM tests
$ cd programs && make build

# Full suite
$ bash test.sh

# Benchmarks
$ cargo bench
$ make report-bench.txt
```

**ckb-x64-simulator**

```bash
$ cargo build
```

**opt-lib/fips202**

```bash
# Build
$ make
# Run test and show cycles
$ make run
```

**opt-lib/sha256**

```bash
# Build
$ make
# Run test and show cycles
$ make run
```

**opt-lib/sha512**

```bash
# Build
$ make
# Run test and show cycles
$ make run
```

**protobuf-ckb-syscalls**

```bash
# Build
$ cargo build
```

**spike-sys**

```bash
# Build
$ cargo build
```

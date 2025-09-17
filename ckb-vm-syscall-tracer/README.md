# CKB-VM Syscall Tracer

A utility for tracing and analyzing system call patterns in CKB scripts running on the CKB Virtual Machine.

## Overview

The CKB-VM Syscall Tracer is a powerful debugging and analysis tool that runs CKB transactions and captures detailed information about syscall input/output for each VM instance. This tool is essential for developers who need to understand the runtime behavior of their smart contracts, debug issues, or optimize performance.

## Usage

This package is available as a binary or a library.

The main tracer that executes transactions and captures syscall data:

```bash
# Download a sample mock transaction.
$ wget https://raw.githubusercontent.com/nervosnetwork/ckb-standalone-debugger/refs/heads/develop/ckb-debugger/examples/mock_tx.json

# Basic usage - trace all syscalls in a transaction.
$ cargo run --bin ckb-vm-syscall-tracer -- --tx-file mock_tx.json --script-hash 0xa52337eabfc2571aa165a8c45d07c06125c4b43497ab7954d8a180fd596b3fb1 --output /tmp/out

# Read the generated trace files.
$ cargo run --bin ckb-vm-syscall-reader -- /tmp/out/vm_0_0.traces
```

or captures tx-parts. tx-parts collector analyzes the transaction structure and components, tracking cell dependencies and script relationships.

```bash
$ cargo run --bin ckb-vm-syscall-tracer -- --collector tx-parts --tx-file mock_tx.json --script-hash 0xa52337eabfc2571aa165a8c45d07c06125c4b43497ab7954d8a180fd596b3fb1 --output /tmp/out

$ cargo run --bin ckb-vm-syscall-reader -- --collector tx-parts /tmp/out/vm_0_0.traces
```

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

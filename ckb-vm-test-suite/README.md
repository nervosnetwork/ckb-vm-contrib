# CKB VM Test Suite

Test suite for CKB VM, kept in a separate project to avoid polluting the vm repo with submodules

# How to run this

First, make sure you have [RISCV GNU toolchain](https://github.com/riscv/riscv-gnu-toolchain) installed. The environment variable `RISCV` should point to the path where you install RISCV GNU toolchain. To test this, make sure the following command works:

```bash
$ ls $RISCV/bin/riscv64-unknown-elf-gcc
```

Now you can run the test suite with the following steps:

```bash
$ git clone https://github.com/nervosnetwork/ckb-vm
$ cargo build --features=asm --example=ckb_vm_runner --release

$ make CKB_VM_RUNNER="$(pwd)/ckb-vm/target/release/examples/ckb_vm_runner"
```

# Get Coverage Report By Kcov

```sh
$ git clone https://github.com/nervosnetwork/ckb-vm
$ cargo build --features=asm --example=ckb_vm_runner

$ make kcov # Kcov will only be installed in the current directory
$ make CKB_VM_RUNNER="$(pwd)/kcov/build/usr/local/bin/kcov $(pwd)/coverage $(pwd)/ckb-vm/target/release/examples/ckb_vm_runner"
```

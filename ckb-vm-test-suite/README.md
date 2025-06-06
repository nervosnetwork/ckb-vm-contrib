# CKB VM Test Suite

Test suite for CKB VM, kept in a separate project to avoid polluting the vm repo with submodules

# How to run this

First, make sure you have [RISCV GNU toolchain](https://github.com/riscv/riscv-gnu-toolchain) installed. The environment variable `RISCV` should point to the path where you install RISCV GNU toolchain. To test this, make sure the following command works:

```bash
$ ls $RISCV/bin/riscv64-unknown-elf-gcc
```

Now you can run the test suite with the following steps:

```bash
$ make ckb-vm
$ RISCV=/opt/riscv ./test.sh
```

# Get Coverage Report By Kcov

First install kcov by <https://github.com/SimonKagstrom/kcov/blob/master/INSTALL.md>

```sh
$ RISCV=/opt/riscv ./test.sh --coverage
```

# Benchmark

Get a full performance review of ckb-vm:

```sh
$ make report-bench.txt
```

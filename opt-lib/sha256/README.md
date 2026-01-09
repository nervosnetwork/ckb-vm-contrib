## Setup

```sh
$ cd $ROOT/deps/musl
$ CLANG=clang-19 bash ckb/build.sh

$ cd $ROOT/deps/compiler-rt-builtins-riscv
$ make CC=clang-19 AR=llvm-ar-19
```

all:
	clang-19 --target=riscv64 -march=rv64imc_zba_zbb_zbc_zbs \
		-O3 \
		-isystem ../../deps/musl/release/include \
		-S -o sha256.asm sha256.c
	clang-19 --target=riscv64 -march=rv64imc_zba_zbb_zbc_zbs \
		-c -o sha256.o sha256.asm
	clang-19 --target=riscv64 -march=rv64imc_zba_zbb_zbc_zbs \
		-O3 \
		-isystem ../../deps/musl/release/include \
		-L../../deps/musl/release/lib -L../../deps/compiler-rt-builtins-riscv/build \
		-lc -lgcc -lcompiler-rt \
		-o main main.c sha256.o

run:
	ckb-debugger --bin main

CC = clang-19
LD = ld.lld-19


CFLAGS = --target=riscv64 -march=rv64imc_zba_zbb_zbc_zbs \
		-O3 -g \
		-nodefaultlibs \
		-isystem ../../deps/musl/release/include \
		-fdata-sections -ffunction-sections

LDFLAGS = -static -L../../deps/musl/release/lib -L../../deps/compiler-rt-builtins-riscv/build \
		-lc -lgcc -lcompiler-rt 

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

all: main.o sha256.o
	$(LD) $(LDFLAGS) -o main $^
	llvm-strip main

run:
	ckb-debugger --bin main

asm: sha256.o
	llvm-objdump-19 -d -S sha256.o

fmt:
	clang-format --style="{BasedOnStyle: Google, IndentWidth: 4}" -i *.c

clean:
	rm -f main.o sha256.o main 
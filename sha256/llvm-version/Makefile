CC = clang-19
LD = ld.lld-19


CFLAGS = --target=riscv64 -march=rv64imc_zba_zbb_zbc_zbs \
		-O3 \
		-nodefaultlibs \
		-isystem ../../deps/musl/release/include \
		-fdata-sections -ffunction-sections

LDFLAGS = -static -L../../deps/musl/release/lib -L../../deps/compiler-rt-builtins-riscv/build \
		-lc -lgcc -lcompiler-rt 

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

all: main.o sha256.o
	$(LD) $(LDFLAGS) -o main $^

run:
	ckb-debugger --bin main

clean:
	rm main.o sha256.o main
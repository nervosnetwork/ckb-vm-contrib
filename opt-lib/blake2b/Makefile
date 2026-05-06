CLANG_VERSION ?= 19
CLANG = clang-$(CLANG_VERSION)
CLANG_FORMAT = clang-format-$(CLANG_VERSION)
LD = ld.lld-$(CLANG_VERSION)
LLVM_OBJDUMP = llvm-objdump-$(CLANG_VERSION)
LLVM_STRIP = llvm-strip-$(CLANG_VERSION)

CFLAGS = --target=riscv64 -march=rv64imc_zba_zbb_zbc_zbs \
		-O3 -g \
		-DNATIVE_LITTLE_ENDIAN \
		-nodefaultlibs \
		-isystem ../../deps/musl/release/include \
		-fdata-sections -ffunction-sections

LDFLAGS = -static --gc-sections -L../../deps/musl/release/lib -L../../deps/compiler-rt-builtins-riscv/build \
		-lc -lgcc -lcompiler-rt

%.o: %.c
	$(CLANG) $(CFLAGS) -c -o $@ $<

all: main.o blake2b-ref.o
	$(LD) $(LDFLAGS) -o main $^
	$(LLVM_STRIP) -s main

run:
	ckb-debugger --bin main

asm: blake2b-ref.o
	$(LLVM_OBJDUMP) -d -S blake2b-ref.o

fmt:
	$(CLANG_FORMAT) --style="{BasedOnStyle: Google, IndentWidth: 4}" -i *.c *.h

clean:
	rm -f blake2b-ref.o main.o main

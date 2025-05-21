cur_dir = $(dir $(abspath $(firstword $(MAKEFILE_LIST))))

TOP := $(cur_dir)
CKB_VM_RUNNER := $(TOP)/ckb-vm/target/release/examples/ckb_vm_runner

all: riscv-tests-run ckb-vm-arch-test-run
	@echo "All tests are passed!"

riscv-tests:
	ln -s ../deps/riscv-tests .
	cd riscv-tests && autoconf && ./configure && make isa

riscv-tests-run: riscv-tests
	@for i in $(shell find riscv-tests/ -regex ".*/rv32u[imac]-u-[a-z0-9_]*" | grep -v "fence_i"); do \
		$(CKB_VM_RUNNER) --mode interpreter32 $$i; \
	done
	@for i in $(shell find riscv-tests/ -regex ".*/rv64u[imac]-u-[a-z0-9_]*" | grep -v "fence_i"); do \
		$(CKB_VM_RUNNER) --mode interpreter64 $$i; \
	done
	@for i in $(shell find riscv-tests/ -regex ".*/rv64u[imac]-u-[a-z0-9_]*" | grep -v "fence_i" | grep -v "rv64ui-u-jalr"); do \
		$(CKB_VM_RUNNER) --mode asm64 $$i; \
	done

riscv-arch-test:
	ln -s ../deps/riscv-arch-test .

COMPLIANCE_TARGET := simulate # Or compile
ckb-vm-arch-test-run: riscv-arch-test
	cd ckb-vm-arch-test && mkdir -p work
	cd ckb-vm-arch-test && find work -name "*.log" -delete && make RISCV_TARGET=ckb-vm XLEN=64 RISCV_DEVICE=I TARGET_SIM="$(CKB_VM_RUNNER) --mode interpreter64" $(COMPLIANCE_TARGET)
	cd ckb-vm-arch-test && find work -name "*.log" -delete && make RISCV_TARGET=ckb-vm XLEN=64 RISCV_DEVICE=M TARGET_SIM="$(CKB_VM_RUNNER) --mode interpreter64" $(COMPLIANCE_TARGET)
	cd ckb-vm-arch-test && find work -name "*.log" -delete && make RISCV_TARGET=ckb-vm XLEN=64 RISCV_DEVICE=C TARGET_SIM="$(CKB_VM_RUNNER) --mode interpreter64" $(COMPLIANCE_TARGET)
	cd ckb-vm-arch-test && find work -name "*.log" -delete && make RISCV_TARGET=ckb-vm XLEN=64 RISCV_DEVICE=B TARGET_SIM="$(CKB_VM_RUNNER) --mode interpreter64" $(COMPLIANCE_TARGET)
	cd ckb-vm-arch-test && find work -name "*.log" -delete && make RISCV_TARGET=ckb-vm XLEN=64 RISCV_DEVICE=I TARGET_SIM="$(CKB_VM_RUNNER) --mode asm64" $(COMPLIANCE_TARGET)
	cd ckb-vm-arch-test && find work -name "*.log" -delete && make RISCV_TARGET=ckb-vm XLEN=64 RISCV_DEVICE=M TARGET_SIM="$(CKB_VM_RUNNER) --mode asm64" $(COMPLIANCE_TARGET)
	cd ckb-vm-arch-test && find work -name "*.log" -delete && make RISCV_TARGET=ckb-vm XLEN=64 RISCV_DEVICE=C TARGET_SIM="$(CKB_VM_RUNNER) --mode asm64" $(COMPLIANCE_TARGET)
	cd ckb-vm-arch-test && find work -name "*.log" -delete && make RISCV_TARGET=ckb-vm XLEN=64 RISCV_DEVICE=B TARGET_SIM="$(CKB_VM_RUNNER) --mode asm64" $(COMPLIANCE_TARGET)

# See https://github.com/SimonKagstrom/kcov/blob/master/INSTALL.md
kcov:
	git clone https://github.com/SimonKagstrom/kcov
	mkdir -p kcov/build
	cd kcov/build && cmake .. && make && make install DESTDIR=.


.PHONY: all riscv-tests-run ckb-vm-arch-test-run

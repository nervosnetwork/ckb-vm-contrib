# Benchmark for ckb-vm RV64IM implementation running on QEMU.
#
# Prerequisites:
#   sudo apt install -y gcc-riscv64-linux-gnu qemu-user-static
#   rustup target add riscv64gc-unknown-linux-gnu

GCC_VERSION=$(ls /usr/lib/gcc-cross/riscv64-linux-gnu/ 2>/dev/null | sort -V | tail -1)
if [ -z "$GCC_VERSION" ]; then
    echo "Error: Could not detect riscv64-linux-gnu GCC version" >&2
    return 1
fi

export CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_LINKER=riscv64-linux-gnu-gcc
export CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C link-args=-L -C link-args=/usr/lib/gcc-cross/riscv64-linux-gnu/${GCC_VERSION}"
export CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_RUNNER="qemu-riscv64-static -L /usr/riscv64-linux-gnu"

OUTPUT=$(cargo test --release --target riscv64gc-unknown-linux-gnu test_rv64im -- --nocapture --test-threads=1 2>&1)

echo ""
echo "| Test | QEMU Instructions |"
echo "|------|-------------------|"
echo "$OUTPUT" | grep "QEMU instructions executed" | while read -r line; do
    test_name=$(echo "$line" | sed 's/test test_rv64im_\([^ ]*\) .*/\1/')
    instructions=$(echo "$line" | sed 's/.*QEMU instructions executed: \([0-9.]*\) M.*/\1 M/')
    echo "| $test_name | $instructions |"
done


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

NATIVE_OUTPUT=$(cargo test --release test_rv64im -- --nocapture --test-threads=1 2>&1)
QEMU_OUTPUT=$(cargo test --release --target riscv64gc-unknown-linux-gnu test_rv64im -- --nocapture --test-threads=1 2>&1)

declare -A native_instructions
declare -A qemu_instructions

while read -r line; do
    test_name=$(echo "$line" | sed 's/test test_rv64im_\([^ ]*\) .*/\1/')
    cycles=$(echo "$line" | sed 's/.*CKB-VM consumed instructions: \([0-9.]*\) M.*/\1/')
    native_instructions["$test_name"]="$cycles"
done < <(echo "$NATIVE_OUTPUT" | grep "CKB-VM consumed instructions")

while read -r line; do
    test_name=$(echo "$line" | sed 's/test test_rv64im_\([^ ]*\) .*/\1/')
    instructions=$(echo "$line" | sed 's/.*QEMU instructions executed: \([0-9.]*\) M.*/\1/')
    qemu_instructions["$test_name"]="$instructions"
done < <(echo "$QEMU_OUTPUT" | grep "QEMU instructions executed")

echo ""
echo "| Test | Raw CKB-VM Instructions | QEMU Instructions | Emulation Overhead |"
echo "|------|-------------------------|-------------------|-------------------|"
for test_name in "${!native_instructions[@]}"; do
    native_val="${native_instructions[$test_name]}"
    qemu_val="${qemu_instructions[$test_name]}"
    overhead=$(awk "BEGIN {printf \"%.0fx\", $qemu_val / $native_val}")
    echo "| $test_name | ${native_val} M | ${qemu_val} M | ${overhead} |"
done | sort


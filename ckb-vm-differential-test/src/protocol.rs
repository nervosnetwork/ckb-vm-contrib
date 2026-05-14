/// Read the next input payload into a guest buffer.
pub const SYSCALL_READ_INPUT: u64 = 0xF000;

/// Emit an output payload back to the host.
pub const SYSCALL_WRITE_OUTPUT: u64 = 0xF001;

/// Marker for the host: `OneShot` ignores it, `WarmStart` snapshots here.
pub const SYSCALL_SIGNAL_READY: u64 = 0xF002;

/// Forward guest panic info to the host before halting.
pub const SYSCALL_PANIC: u64 = 0xF003;

pub const DEFAULT_MAX_PAYLOAD_LEN: usize = 1 << 20;

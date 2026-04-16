#![no_std]

/// Reads the next input payload into a guest buffer.
///
/// Calling convention: `a0` = buffer ptr, `a1` = capacity, `a7` = this code.
/// Returns the number of bytes written into `a0`.
pub const SYSCALL_READ_INPUT: u64 = 0xF000;

/// Emits an output payload back to the host.
///
/// Calling convention: `a0` = buffer ptr, `a1` = length, `a7` = this code.
pub const SYSCALL_WRITE_OUTPUT: u64 = 0xF001;

/// Marks the guest as ready for its next input.
///
/// Stage A: host ignores. Stages B/C: host snapshots or resumes here.
pub const SYSCALL_SIGNAL_READY: u64 = 0xF002;

pub const DEFAULT_MAX_PAYLOAD_LEN: usize = 1 << 20;

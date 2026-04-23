#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use serde::{Serialize, de::DeserializeOwned};

pub use ckb_vm_differential_protocol as protocol;

pub fn read_input<I: DeserializeOwned>() -> I {
    let bytes = read_input_raw();
    postcard::from_bytes(&bytes).expect("decode harness input")
}

pub fn write_output<O: Serialize>(output: &O) {
    let bytes = postcard::to_allocvec(output).expect("encode harness output");
    write_output_raw(&bytes);
}

pub fn run<I, O, F>(mut f: F) -> !
where
    I: DeserializeOwned,
    O: Serialize,
    F: FnMut(I) -> O,
{
    loop {
        signal_ready();
        let input: I = read_input();
        let output = f(input);
        write_output(&output);
    }
}

pub fn read_input_raw() -> Vec<u8> {
    // First call with capacity 0 to probe the required size
    let needed = unsafe { syscall_read_input(core::ptr::null_mut(), 0) };
    let mut buf: Vec<u8> = Vec::with_capacity(needed);
    let written = unsafe { syscall_read_input(buf.as_mut_ptr(), buf.capacity()) };
    assert_eq!(written, needed, "host changed input size between probe and read");
    unsafe { buf.set_len(written) };
    buf
}

pub fn write_output_raw(bytes: &[u8]) {
    unsafe { syscall_write_output(bytes.as_ptr(), bytes.len()) };
}

#[inline]
pub fn signal_ready() {
    unsafe { syscall_signal_ready() };
}

#[cfg(target_arch = "riscv64")]
unsafe fn syscall_read_input(buf: *mut u8, cap: usize) -> usize {
    let written: usize;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") buf as usize => written,
            in("a1") cap,
            in("a7") protocol::SYSCALL_READ_INPUT,
            options(nostack),
        );
    }
    written
}

#[cfg(target_arch = "riscv64")]
unsafe fn syscall_write_output(buf: *const u8, len: usize) {
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a0") buf as usize,
            in("a1") len,
            in("a7") protocol::SYSCALL_WRITE_OUTPUT,
            options(nostack),
        );
    }
}

#[cfg(target_arch = "riscv64")]
unsafe fn syscall_signal_ready() {
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a7") protocol::SYSCALL_SIGNAL_READY,
            options(nostack),
        );
    }
}

#[cfg(not(target_arch = "riscv64"))]
unsafe fn syscall_read_input(_buf: *mut u8, _cap: usize) -> usize {
    unimplemented!("guest syscalls only compile on riscv64")
}

#[cfg(not(target_arch = "riscv64"))]
unsafe fn syscall_write_output(_buf: *const u8, _len: usize) {
    unimplemented!("guest syscalls only compile on riscv64")
}

#[cfg(not(target_arch = "riscv64"))]
unsafe fn syscall_signal_ready() {
    unimplemented!("guest syscalls only compile on riscv64")
}

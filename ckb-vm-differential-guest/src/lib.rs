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

#[inline]
pub fn signal_ready() {
    let _ = protocol::SYSCALL_SIGNAL_READY;
    todo!()
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
    let _ = protocol::SYSCALL_READ_INPUT;
    todo!()
}

pub fn write_output_raw(bytes: &[u8]) {
    let _ = (protocol::SYSCALL_WRITE_OUTPUT, bytes);
    todo!()
}

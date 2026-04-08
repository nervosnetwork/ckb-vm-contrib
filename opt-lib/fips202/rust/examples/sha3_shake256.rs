#![no_std]
#![no_main]

use core::arch::asm;
use sha3::{digest::{Update, ExtendableOutput, XofReader}};

fn exit(code: i8) -> ! {
    unsafe {
        asm!("mv a0, {0}",
             "li a7, 93",
             "ecall",
             in(reg) code,
        )
    }
    loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    exit(-128);
}

#[unsafe(no_mangle)]
fn abort() -> ! {
    panic!("abort!")
}

#[unsafe(no_mangle)]
fn _start() -> ! {
    let mut message = [0u8; 32];
    for _ in 0..25_000 {
        let mut hasher = sha3::Shake256::default();
        hasher.update(&message);
        let mut reader = hasher.finalize_xof();
        reader.read(&mut message);
    }
    let want: [u8; 32] = [
        0x41, 0xa6, 0x43, 0xad, 0xe0, 0x13, 0xc0, 0xe9, 0x9b, 0x31, 0x48, 0x42, 0x40, 0xc3, 0xd1, 0x15, 0x94, 0x5d,
        0x0e, 0xae, 0xed, 0xda, 0xc9, 0x2f, 0xfe, 0xf0, 0x47, 0xf2, 0xea, 0x87, 0x32, 0x3f,
    ];
    assert_eq!(message, want);
    exit(0)
}

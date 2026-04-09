#![no_std]
#![no_main]

use core::arch::asm;
use sha3::digest::{ExtendableOutput, Update, XofReader};

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
        let mut hasher = sha3::Shake128::default();
        hasher.update(&message);
        let mut reader = hasher.finalize_xof();
        reader.read(&mut message);
    }
    let want: [u8; 32] = [
        0xa0, 0x65, 0x00, 0xf1, 0xf0, 0xd8, 0xa6, 0x38, 0xb4, 0xbb, 0x41, 0x2f, 0xed, 0x1e, 0x71, 0xf1, 0x9f, 0x68,
        0x0b, 0xc0, 0xf8, 0xea, 0x21, 0x3e, 0x34, 0x17, 0x06, 0x46, 0x4f, 0x89, 0xa5, 0x9a,
    ];
    assert_eq!(message, want);
    exit(0)
}

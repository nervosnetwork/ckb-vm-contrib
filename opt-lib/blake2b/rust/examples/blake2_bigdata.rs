#![no_std]
#![no_main]

use blake2::digest::consts::U32;
use blake2::{Blake2b, Digest};
use core::arch::asm;

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
    let buf = [0u8; 1024];
    let mut hasher = Blake2b::<U32>::new();
    for _ in 0..1024 {
        hasher.update(&buf);
    }
    let hash = hasher.finalize();
    let want: [u8; 32] = [
        0xc7, 0x48, 0x60, 0xdd, 0x74, 0x80, 0xe7, 0xf4, 0xb5, 0xae, 0x70, 0x5f, 0x91, 0x37, 0xe9, 0x0a, 0x0a, 0xa0,
        0xbc, 0x67, 0xd6, 0xe9, 0x0c, 0xf8, 0x07, 0x8d, 0xd6, 0x69, 0x7d, 0xbd, 0xb6, 0xad,
    ];
    assert_eq!(hash, want.into());
    exit(0)
}

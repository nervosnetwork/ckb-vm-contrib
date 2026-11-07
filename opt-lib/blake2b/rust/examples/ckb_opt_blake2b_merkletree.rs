#![no_std]
#![no_main]

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
    let mut hash = [0u8; 32];
    let mut p = ckb_opt_blake2b::blake2b_params();
    p.digest(32);
    for _ in 0..1000 {
        let mut h = ckb_opt_blake2b::blake2b(p.clone());
        h.update(&hash);
        h.digest(&mut hash);
    }
    let want: [u8; 32] = [
        0x65, 0x11, 0x43, 0x6e, 0xd9, 0xa4, 0x38, 0x6a, 0xc9, 0xb2, 0xc5, 0x2d, 0xaf, 0x9c, 0x64, 0x9f, 0xc6, 0x61,
        0xb6, 0x78, 0xfb, 0x90, 0xd3, 0x97, 0x7c, 0x9c, 0x18, 0x79, 0xc5, 0xa5, 0xd9, 0xeb,
    ];
    assert_eq!(hash, want);
    exit(0)
}

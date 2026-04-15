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
    let mut message = [0u8; 32];
    for _ in 0..25_000 {
        let mut hasher = ckb_opt_fips202::Sha3_256::new();
        hasher.update(&message);
        hasher.finalize(&mut message);
    }
    let want: [u8; 32] = [
        0x22, 0xcf, 0x63, 0x05, 0x01, 0xa7, 0xbd, 0x22, 0xfb, 0x5c, 0xe9, 0x18, 0xf0, 0x9c, 0x70, 0xd0, 0x9e, 0xe0,
        0x83, 0x73, 0xb3, 0x39, 0x7c, 0x31, 0x83, 0x0a, 0x1c, 0x38, 0x4b, 0xc8, 0x3b, 0x33,
    ];
    assert_eq!(message, want);
    exit(0)
}

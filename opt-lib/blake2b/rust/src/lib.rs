//! BLAKE2b implementation for CKB-VM with Rust bindings.
//!
//! This crate provides a Rust interface to the BLAKE2b reference implementation
//! designed for CKB-VM (RISC-V). The default output size is 32 bytes (BLAKE2b-256),
//! which matches the CKB convention.
//!
//! # Example
//!
//! ```
//! use ckb_opt_blake2b::{Blake2b, blake2b};
//!
//! // One-shot hashing (32-byte output)
//! let hash = blake2b(b"hello world");
//! assert_eq!(hash.len(), 32);
//!
//! // Incremental hashing
//! let mut hasher = Blake2b::new();
//! hasher.update(b"hello ");
//! hasher.update(b"world");
//! let hash = hasher.finalize();
//! ```

#![no_std]

/// Default output size: 32 bytes (BLAKE2b-256), matching CKB convention.
pub const BLAKE2B_OUTBYTES: usize = 32;

/// Maximum output size supported by BLAKE2b.
pub const BLAKE2B_MAX_OUTBYTES: usize = 64;

pub type Digest = [u8; BLAKE2B_OUTBYTES];

mod ffi {
    use core::ffi::{c_int, c_uchar};

    /// Mirror of `blake2b_state` from blake2.h.
    ///
    /// Layout (64-bit target):
    ///   h[8]       : 8 × u64  = 64 bytes  (offset   0)
    ///   t[2]       : 2 × u64  = 16 bytes  (offset  64)
    ///   f[2]       : 2 × u64  = 16 bytes  (offset  80)
    ///   buf[128]   : 128 × u8 = 128 bytes (offset  96)
    ///   buflen     : usize    =  8 bytes  (offset 224)
    ///   outlen     : usize    =  8 bytes  (offset 232)
    ///   last_node  : u8       =  1 byte   (offset 240)
    ///   (7 bytes padding)
    #[repr(C)]
    pub struct Blake2bState {
        pub h: [u64; 8],
        pub t: [u64; 2],
        pub f: [u64; 2],
        pub buf: [c_uchar; 128],
        pub buflen: usize,
        pub outlen: usize,
        pub last_node: c_uchar,
    }

    extern "C" {
        pub fn blake2b_init(S: *mut Blake2bState, outlen: usize) -> c_int;
        pub fn blake2b_update(S: *mut Blake2bState, pin: *const c_uchar, inlen: usize) -> c_int;
        pub fn blake2b_final(S: *mut Blake2bState, out: *mut c_uchar, outlen: usize) -> c_int;
    }
}

/// BLAKE2b hasher with 32-byte (256-bit) output.
pub struct Blake2b {
    ctx: ffi::Blake2bState,
}

impl Default for Blake2b {
    fn default() -> Self {
        Self::new()
    }
}

impl Blake2b {
    /// Creates a new BLAKE2b hasher with 32-byte output (BLAKE2b-256).
    #[inline]
    pub fn new() -> Self {
        let mut ctx = ffi::Blake2bState {
            h: [0u64; 8],
            t: [0u64; 2],
            f: [0u64; 2],
            buf: [0u8; 128],
            buflen: 0,
            outlen: 0,
            last_node: 0,
        };
        unsafe {
            ffi::blake2b_init(&mut ctx, BLAKE2B_OUTBYTES);
        }
        Self { ctx }
    }

    /// Updates the hasher with input data.
    #[inline]
    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            ffi::blake2b_update(&mut self.ctx, data.as_ptr(), data.len());
        }
    }

    /// Finalizes the hash computation and returns the 32-byte digest.
    #[inline]
    pub fn finalize(mut self) -> Digest {
        let mut hash = [0u8; BLAKE2B_OUTBYTES];
        unsafe {
            ffi::blake2b_final(&mut self.ctx, hash.as_mut_ptr(), BLAKE2B_OUTBYTES);
        }
        hash
    }
}

/// Computes the BLAKE2b-256 hash of the input data (32-byte output, no key).
#[inline]
pub fn blake2b(data: &[u8]) -> Digest {
    let mut hasher = Blake2b::new();
    hasher.update(data);
    hasher.finalize()
}

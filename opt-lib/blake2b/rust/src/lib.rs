#![no_std]

//! BLAKE2b C reference implementation for CKB-VM with Rust bindings.
//!
//! The interface mirrors [blake2ya](https://github.com/mohanson/blake2ya).
//!
//! # Example
//!
//! ```rust
//! let mut p = ckb_opt_blake2b::blake2b_params();
//! p.digest(32);
//! let mut h = ckb_opt_blake2b::blake2b(p);
//! h.update(b"abc");
//! let mut r = [0u8; 32];
//! h.digest(&mut r);
//! let e = [
//!     0xbd, 0xdd, 0x81, 0x3c, 0x63, 0x42, 0x39, 0x72, 0x31, 0x71, 0xef, 0x3f, 0xee, 0x98, 0x57,
//!     0x9b, 0x94, 0x96, 0x4e, 0x3b, 0xb1, 0xcb, 0x3e, 0x42, 0x72, 0x62, 0xc8, 0xc0, 0x68, 0xd5,
//!     0x23, 0x19,
//! ];
//! assert_eq!(r, e);
//! ```

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

    /// Mirror of `blake2b_param` from blake2.h (packed, 64 bytes total).
    ///
    /// Byte layout:
    ///   [0x00]      digest_length (1)
    ///   [0x01]      key_length    (1)
    ///   [0x02]      fanout        (1)
    ///   [0x03]      depth         (1)
    ///   [0x04-0x07] leaf_length   (4)
    ///   [0x08-0x0b] node_offset   (4)
    ///   [0x0c-0x0f] xof_length    (4)
    ///   [0x10]      node_depth    (1)
    ///   [0x11]      inner_length  (1)
    ///   [0x12-0x1f] reserved      (14)
    ///   [0x20-0x2f] salt          (16)
    ///   [0x30-0x3f] personal      (16)
    #[repr(C, packed)]
    pub struct Blake2bParam {
        pub digest_length: c_uchar,
        pub key_length: c_uchar,
        pub fanout: c_uchar,
        pub depth: c_uchar,
        pub leaf_length: u32,
        pub node_offset: u32,
        pub xof_length: u32,
        pub node_depth: c_uchar,
        pub inner_length: c_uchar,
        pub reserved: [c_uchar; 14],
        pub salt: [c_uchar; 16],
        pub personal: [c_uchar; 16],
    }

    extern "C" {
        pub fn blake2b_init_param(S: *mut Blake2bState, P: *const Blake2bParam) -> c_int;
        pub fn blake2b_update(S: *mut Blake2bState, pin: *const c_uchar, inlen: usize) -> c_int;
        pub fn blake2b_final(S: *mut Blake2bState, out: *mut c_uchar, outlen: usize) -> c_int;
    }
}

/// BLAKE2b parameter block structure.
///
/// The byte layout of the internal buffer mirrors `blake2b_param` in blake2.h exactly,
/// so the buffer can be cast directly to a `*const Blake2bParam` pointer for FFI.
#[derive(Clone)]
pub struct Blake2bParams {
    /// Packed parameter block (64 bytes), layout identical to C `blake2b_param`.
    buf: [u8; 64],
    /// Key bytes (up to 64), zeroed beyond key_length.
    key: [u8; 64],
}

impl Blake2bParams {
    /// Set digest byte length. Must be in [1, 64] for BLAKE2b.
    pub fn digest(&mut self, n: u8) {
        assert!(1 <= n && n <= 64);
        self.buf[0x00] = n;
    }

    /// Set key. Key length must be in [0, 64] for BLAKE2b.
    pub fn key(&mut self, n: &[u8]) {
        assert!(n.len() <= 64);
        self.buf[0x01] = n.len() as u8;
        self.key[..n.len()].copy_from_slice(n);
    }

    /// Set salt. An arbitrary string of up to 16 bytes.
    pub fn salt(&mut self, n: &[u8]) {
        assert!(n.len() <= 16);
        self.buf[0x20..0x20 + n.len()].copy_from_slice(n);
    }

    /// Set personalization. An arbitrary string of up to 16 bytes.
    pub fn person(&mut self, n: &[u8]) {
        assert!(n.len() <= 16);
        self.buf[0x30..0x30 + n.len()].copy_from_slice(n);
    }
}

/// A context for computing the BLAKE2b checksum.
pub struct Blake2b {
    ctx: ffi::Blake2bState,
    outlen: usize,
}

impl Blake2b {
    /// Update this hash object's state with the provided data.
    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            ffi::blake2b_update(&mut self.ctx, data.as_ptr(), data.len());
        }
    }

    /// Return the digest value into the provided buffer.
    ///
    /// The length of `d` must equal the digest length set in [`Blake2bParams::digest`].
    pub fn digest(&mut self, d: &mut [u8]) {
        unsafe {
            ffi::blake2b_final(&mut self.ctx, d.as_mut_ptr(), self.outlen);
        }
    }
}

/// Create the parameter block of BLAKE2b. All general parameters are supported.
///
/// The caller must call [`Blake2bParams::digest`] to set the output length before
/// passing the params to [`blake2b`].
pub fn blake2b_params() -> Blake2bParams {
    let mut r = Blake2bParams { buf: [0u8; 64], key: [0u8; 64] };
    r.buf[0x02] = 0x01; // fanout  = 1
    r.buf[0x03] = 0x01; // depth   = 1
    r
}

/// Core hasher state of BLAKE2b.
pub fn blake2b(params: Blake2bParams) -> Blake2b {
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
        ffi::blake2b_init_param(&mut ctx, params.buf.as_ptr() as *const ffi::Blake2bParam);
    }
    let outlen = params.buf[0x00] as usize;
    let mut hasher = Blake2b { ctx, outlen };
    // If a key was set, feed it as the first padded block (matching blake2b_init_key behavior).
    if params.buf[0x01] != 0 {
        let mut key_block = [0u8; 128];
        key_block[..params.key.len()].copy_from_slice(&params.key);
        hasher.update(&key_block);
    }
    hasher
}

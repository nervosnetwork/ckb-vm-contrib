//! Optimized SHAKE128/SHAKE256 implementation for CKB-VM with Rust bindings.
//!
//! This crate exposes minimal APIs around SHAKE128 and SHAKE256 absorb/squeeze operations.

#![no_std]

pub const SHAKE128_RATE: usize = 168;
pub const SHAKE256_RATE: usize = 136;

mod ffi {
    use core::ffi::c_uchar;

    extern "C" {
        pub fn shake128_squeezeblocks(output: *mut c_uchar, nblocks: usize, s: *mut u64);
        pub fn shake128_inc_absorb(s_inc: *mut u64, input: *const c_uchar, inlen: usize);
        pub fn shake128_inc_finalize(s_inc: *mut u64);
        pub fn shake128_inc_squeeze(output: *mut c_uchar, outlen: usize, s_inc: *mut u64);

        pub fn shake256_squeezeblocks(output: *mut c_uchar, nblocks: usize, s: *mut u64);
        pub fn shake256_inc_absorb(s_inc: *mut u64, input: *const c_uchar, inlen: usize);
        pub fn shake256_inc_finalize(s_inc: *mut u64);
        pub fn shake256_inc_squeeze(output: *mut c_uchar, outlen: usize, s_inc: *mut u64);
    }
}

/// SHAKE128 hasher.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Shake128 {
    state: [u64; 26],
}

impl Shake128 {
    /// Creates a new SHAKE128 hasher instance.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Absorbs input bytes into the hasher state.
    pub fn absorb(&mut self, data: &[u8]) {
        unsafe {
            ffi::shake128_inc_absorb(self.state.as_mut_ptr(), data.as_ptr(), data.len());
        }
    }

    /// Finalizes the absorb phase, preparing the state for squeezing.
    pub fn finalize(&mut self) {
        unsafe {
            ffi::shake128_inc_finalize(self.state.as_mut_ptr());
        }
    }

    /// Squeezes bytes from the finalized state.
    pub fn squeeze(&mut self, output: &mut [u8]) {
        unsafe {
            ffi::shake128_inc_squeeze(output.as_mut_ptr(), output.len(), self.state.as_mut_ptr());
        }
    }

    /// Squeezes full SHAKE128 blocks from the finalized state. Output length must be a multiple
    /// of SHAKE128_RATE (168 bytes). Must not be called after squeeze().
    pub fn squeeze_blocks(&mut self, output: &mut [u8]) {
        debug_assert_eq!(output.len() % SHAKE128_RATE, 0);
        debug_assert_eq!(self.state[25], 0);
        unsafe {
            ffi::shake128_squeezeblocks(output.as_mut_ptr(), output.len() / SHAKE128_RATE, self.state.as_mut_ptr());
        }
    }
}

/// SHAKE256 hasher.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Shake256 {
    state: [u64; 26],
}

impl Shake256 {
    /// Creates a new SHAKE256 hasher instance.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Absorbs input bytes into the hasher state.
    pub fn absorb(&mut self, data: &[u8]) {
        unsafe {
            ffi::shake256_inc_absorb(self.state.as_mut_ptr(), data.as_ptr(), data.len());
        }
    }

    /// Finalizes the absorb phase, preparing the state for squeezing.
    pub fn finalize(&mut self) {
        unsafe {
            ffi::shake256_inc_finalize(self.state.as_mut_ptr());
        }
    }

    /// Squeezes bytes from the finalized state.
    pub fn squeeze(&mut self, output: &mut [u8]) {
        unsafe {
            ffi::shake256_inc_squeeze(output.as_mut_ptr(), output.len(), self.state.as_mut_ptr());
        }
    }

    /// Squeezes full SHAKE256 blocks from the finalized state. Output length must be a multiple
    /// of SHAKE256_RATE (136 bytes). Must not be called after squeeze().
    pub fn squeeze_blocks(&mut self, output: &mut [u8]) {
        debug_assert_eq!(output.len() % SHAKE256_RATE, 0);
        debug_assert_eq!(self.state[25], 0);
        unsafe {
            ffi::shake256_squeezeblocks(output.as_mut_ptr(), output.len() / SHAKE256_RATE, self.state.as_mut_ptr());
        }
    }
}

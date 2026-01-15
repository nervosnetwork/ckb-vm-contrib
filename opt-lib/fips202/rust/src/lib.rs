//! Optimized SHAKE256 implementation for CKB-VM with Rust bindings.
//!
//! This crate exposes a minimal API around SHAKE256 absorb/squeeze operations.

#![no_std]

pub const SHAKE256_RATE: usize = 136;

mod ffi {
    use core::ffi::c_uchar;

    extern "C" {
        pub fn shake256_inc_init(s_inc: *mut u64);
        pub fn shake256_inc_absorb(s_inc: *mut u64, input: *const c_uchar, inlen: usize);
        pub fn shake256_inc_finalize(s_inc: *mut u64);
        pub fn shake256_inc_squeeze(output: *mut c_uchar, outlen: usize, s_inc: *mut u64);
    }
}

/// Internal SHAKE256 state used between absorb and squeeze operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Shake256State {
    state: [u64; 26],
}

/// SHAKE256 hasher.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Shake256 {
    state: [u64; 26],
}

impl Default for Shake256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Shake256 {
    /// Creates a new SHAKE256 hasher instance.
    #[inline]
    pub fn new() -> Self {
        let mut state = [0u64; 26];
        unsafe {
            ffi::shake256_inc_init(state.as_mut_ptr());
        }
        Self { state }
    }

    /// Absorbs input bytes and returns an initialized SHAKE256 state.
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

    /// Squeezes bytes from an absorbed SHAKE256 state.
    pub fn squeeze(&mut self, output: &mut [u8]) {
        unsafe {
            ffi::shake256_inc_squeeze(output.as_mut_ptr(), output.len(), self.state.as_mut_ptr());
        }
    }
}

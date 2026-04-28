#![cfg_attr(target_arch = "riscv64", no_std)]
#![cfg_attr(not(target_arch = "riscv64"), forbid(unsafe_op_in_unsafe_fn))]

#[cfg(target_arch = "riscv64")]
extern crate alloc;

pub mod protocol;

#[cfg(target_arch = "riscv64")]
pub mod guest;

// Re-exported so `harness!` can invoke `default_alloc!` without the user crate depending on ckb-std directly.
#[cfg(target_arch = "riscv64")]
pub use ckb_std;

#[cfg(not(target_arch = "riscv64"))]
mod executor;
#[cfg(not(target_arch = "riscv64"))]
mod guest_build;

#[cfg(not(target_arch = "riscv64"))]
pub use executor::{DifferentialSyscalls, OneShot, WarmStart};
#[cfg(not(target_arch = "riscv64"))]
pub use guest_build::{BuildConfig, build_guest_crate, build_guest_crate_with};
#[cfg(not(target_arch = "riscv64"))]
pub use proptest;

#[cfg(not(target_arch = "riscv64"))]
mod host_api {
    use core::fmt::Debug;

    use crate::protocol;

    pub trait Harness: 'static {
        type Input: serde::Serialize + serde::de::DeserializeOwned + Clone + Debug + 'static;
        type Output: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static;

        const MAX_PAYLOAD_LEN: usize = protocol::DEFAULT_MAX_PAYLOAD_LEN;

        fn guest_elf() -> &'static [u8];
        fn reference(input: &Self::Input) -> Self::Output;
    }

    #[derive(Debug, thiserror::Error)]
    pub enum DivergenceError {
        #[error("output mismatch:\n  reference = {reference}\n  guest     = {guest}\n  input     = {input}")]
        OutputMismatch { input: String, reference: String, guest: String },

        #[error("guest exited early: {reason}")]
        GuestExited { reason: String },

        #[error("guest panicked: {message}")]
        GuestPanicked { message: String },

        #[error("vm error: {0}")]
        Vm(#[from] ckb_vm::Error),

        #[error("malformed guest output: {0}")]
        Decode(#[from] postcard::Error),

        #[error("payload exceeds MAX_PAYLOAD_LEN ({limit} bytes): saw {actual}")]
        PayloadTooLarge { limit: usize, actual: usize },

        #[error("guest build failed: {0}")]
        Build(String),
    }

    pub trait Executor<H: Harness> {
        fn check(&mut self, input: &H::Input) -> Result<(), DivergenceError>;
    }

    pub fn oneshot_check<H: Harness>(input: &H::Input) -> Result<(), proptest::test_runner::TestCaseError> {
        use crate::OneShot;
        let mut executor = OneShot::<H>::new();
        executor.check(input).map_err(|e| proptest::test_runner::TestCaseError::fail(e.to_string()))
    }

    /// proptest helper backed by [`crate::WarmStart`]. The per-harness executor
    /// is held in a thread-local `TypeId`-keyed map so the snapshot survives
    /// across cases (proptest re-enters the test fn per case).
    pub fn warmstart_check<H: Harness>(input: &H::Input) -> Result<(), proptest::test_runner::TestCaseError> {
        use crate::WarmStart;
        use std::any::{Any, TypeId};
        use std::cell::RefCell;
        use std::collections::HashMap;

        thread_local! {
            static EXECUTORS: RefCell<HashMap<TypeId, Box<dyn Any>>> = RefCell::new(HashMap::new());
        }

        EXECUTORS.with(|cell| {
            let mut map = cell.borrow_mut();
            let entry = map.entry(TypeId::of::<H>()).or_insert_with(|| Box::new(WarmStart::<H>::new()));
            let executor = entry.downcast_mut::<WarmStart<H>>().expect("WarmStart<H> downcast");
            executor.check(input).map_err(|e| proptest::test_runner::TestCaseError::fail(e.to_string()))
        })
    }
}

#[cfg(not(target_arch = "riscv64"))]
pub use host_api::{DivergenceError, Executor, Harness, oneshot_check, warmstart_check};

/// Defines a harness for a single library port. Expands to:
///
/// * On riscv64 — the guest entry point + panic handler that runs the `port`
///   function in a serve-input/return-output loop.
/// * On any other target — a unit struct named `$name` implementing
///   [`Harness`], with `guest_elf()` lazily built (or supplied via the
///   `CKB_VM_DIFFERENTIAL_GUEST_ELF` env var) and `reference()` invoking the
///   host-side closure.
///
/// The optional `build:` arm threads a custom [`BuildConfig`] through to the
/// cargo subprocess that produces the guest ELF.
#[macro_export]
macro_rules! harness {
    (
        name:      $name:ident,
        input:     $input:ty,
        output:    $output:ty,
        port:      $port:expr,
        reference: $reference:expr $(,)?
    ) => {
        $crate::harness! {
            name:      $name,
            input:     $input,
            output:    $output,
            port:      $port,
            reference: $reference,
            build:     $crate::__default_build_config(),
        }
    };

    (
        name:      $name:ident,
        input:     $input:ty,
        output:    $output:ty,
        port:      $port:expr,
        reference: $reference:expr,
        build:     $build:expr $(,)?
    ) => {
        /* #region Guest-side expansion */
        #[cfg(target_arch = "riscv64")]
        extern crate alloc;

        // 16KB fixed + 1.2MB dynamic. Override by hand-rolling `default_alloc!` for harnesses needing more.
        #[cfg(target_arch = "riscv64")]
        $crate::ckb_std::default_alloc!(16384, 1258306, 64);

        // Replaces `ckb_std::entry!` so guest panics surface as `DivergenceError::GuestPanicked`.
        #[cfg(target_arch = "riscv64")]
        $crate::guest_main!(__ckb_vm_differential_guest_main);

        #[cfg(target_arch = "riscv64")]
        fn __ckb_vm_differential_guest_main() -> i8 {
            $crate::guest::run(|input: $input| -> $output { ($port)(&input) })
        }
        /* #endregion */

        /* #region Host-side expansion */
        #[cfg(not(target_arch = "riscv64"))]
        pub struct $name;

        #[cfg(not(target_arch = "riscv64"))]
        impl $crate::Harness for $name {
            type Input = $input;
            type Output = $output;

            fn guest_elf() -> &'static [u8] {
                static ELF: ::std::sync::OnceLock<::std::vec::Vec<u8>> = ::std::sync::OnceLock::new();
                ELF.get_or_init(|| {
                    let config: $crate::BuildConfig = $build;
                    $crate::build_guest_crate_with(env!("CARGO_MANIFEST_DIR"), &config).expect("build guest crate")
                })
                .as_slice()
            }

            fn reference(input: &Self::Input) -> Self::Output {
                ($reference)(input)
            }
        }
        /* #endregion */
    };
}

#[cfg(not(target_arch = "riscv64"))]
#[doc(hidden)]
pub fn __default_build_config() -> BuildConfig {
    BuildConfig::default()
}

#[cfg(target_arch = "riscv64")]
#[doc(hidden)]
pub fn __default_build_config() {}

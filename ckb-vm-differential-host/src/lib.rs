#![forbid(unsafe_op_in_unsafe_fn)]

use core::fmt::Debug;

pub use ckb_vm_differential_protocol as protocol;
pub use proptest;

pub mod executor;
pub use executor::OneShot;

/// Pairs a host-side reference implementation with a ckb-vm guest ELF that
/// exercises the ported implementation. Implement once per library.
pub trait Harness: 'static {
    type Input: serde::Serialize + serde::de::DeserializeOwned + Clone + Debug + 'static;
    type Output: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static;

    const GUEST_ELF: &'static [u8];
    const MAX_PAYLOAD_LEN: usize = protocol::DEFAULT_MAX_PAYLOAD_LEN;

    fn reference(input: &Self::Input) -> Self::Output;
}

#[derive(Debug, thiserror::Error)]
pub enum DivergenceError {
    #[error("output mismatch:\n  reference = {reference}\n  guest     = {guest}\n  input     = {input}")]
    OutputMismatch { input: String, reference: String, guest: String },

    #[error("guest exited early: {reason}")]
    GuestExited { reason: String },

    #[error("vm error: {0}")]
    Vm(#[from] ckb_vm::Error),

    #[error("malformed guest output: {0}")]
    Decode(#[from] postcard::Error),

    #[error("payload exceeds MAX_PAYLOAD_LEN ({limit} bytes): saw {actual}")]
    PayloadTooLarge { limit: usize, actual: usize },
}

pub trait Executor<H: Harness> {
    fn check(&mut self, input: &H::Input) -> Result<(), DivergenceError>;
}

pub fn oneshot_check<H: Harness>(
    input: &H::Input,
) -> Result<(), proptest::test_runner::TestCaseError> {
    let mut executor = OneShot::<H>::new();
    executor
        .check(input)
        .map_err(|e| proptest::test_runner::TestCaseError::fail(e.to_string()))
}

use core::marker::PhantomData;

use crate::{DivergenceError, Executor, Harness};

type Payload = Vec<u8>;

/// Boots a fresh ckb-vm for every input
pub struct OneShot<H: Harness> {
    _marker: PhantomData<H>,
}

impl<H: Harness> Default for OneShot<H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<H: Harness> OneShot<H> {
    pub fn new() -> Self {
        Self { _marker: PhantomData }
    }

    pub fn check(&mut self, input: &H::Input) -> Result<(), DivergenceError> {
        let expected = H::reference(input);
        let actual = self.run_guest(input)?;
        if expected != actual {
            return Err(DivergenceError::OutputMismatch {
                input: format!("{input:?}"),
                reference: format!("{expected:?}"),
                guest: format!("{actual:?}"),
            });
        }
        Ok(())
    }

    fn run_guest(&mut self, input: &H::Input) -> Result<H::Output, DivergenceError> {
        let _ = input;
        todo!()
    }
}

impl<H: Harness> Executor<H> for OneShot<H> {
    fn check(&mut self, input: &H::Input) -> Result<(), DivergenceError> {
        OneShot::check(self, input)
    }
}

#[doc(hidden)]
pub struct DifferentialSyscalls {
    pub input_bytes: Payload,
    pub output_bytes: Option<Payload>,
    pub ready_count: u32,
}

impl DifferentialSyscalls {
    pub fn new(input_bytes: Payload) -> Self {
        Self { input_bytes, output_bytes: None, ready_count: 0 }
    }

    pub fn handle_read_input<Mac>(&self, _machine: &mut Mac) -> Result<(), ckb_vm::Error> {
        todo!()
    }

    pub fn handle_write_output<Mac>(&mut self, _machine: &mut Mac) -> Result<(), ckb_vm::Error> {
        todo!()
    }

    pub fn handle_signal_ready<Mac>(&mut self, _machine: &mut Mac) -> Result<(), ckb_vm::Error> {
        todo!()
    }
}

use core::marker::PhantomData;
use std::sync::{Arc, Mutex};

use ckb_vm::{
    Bytes, DefaultCoreMachine, DefaultMachineBuilder, Error as VmError, Memory, Register, SparseMemory, SupportMachine,
    Syscalls,
    cost_model::estimate_cycles,
    registers::{A0, A1, A7},
};
use ckb_vm_differential_protocol as protocol;

use crate::{DivergenceError, Executor, Harness};

type Payload = Vec<u8>;
type OutputSlot = Arc<Mutex<Option<Payload>>>;

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
        let input_bytes = postcard::to_allocvec(input)?;
        if input_bytes.len() > H::MAX_PAYLOAD_LEN {
            return Err(DivergenceError::PayloadTooLarge { limit: H::MAX_PAYLOAD_LEN, actual: input_bytes.len() });
        }

        let (syscalls, output_slot) = DifferentialSyscalls::new(input_bytes, H::MAX_PAYLOAD_LEN);

        let core = DefaultCoreMachine::<u64, SparseMemory<u64>>::new(
            ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_A | ckb_vm::ISA_MOP,
            ckb_vm::machine::VERSION2,
            u64::MAX,
        );
        let mut machine = DefaultMachineBuilder::new(core)
            .instruction_cycle_func(Box::new(estimate_cycles))
            .syscall(Box::new(syscalls))
            .build();

        let elf = Bytes::copy_from_slice(H::guest_elf());
        machine.load_program(&elf, std::iter::empty::<Result<Bytes, VmError>>())?;

        let exit_code = machine.run()?;

        let output_bytes =
            output_slot.lock().expect("output slot poisoned").take().ok_or_else(|| DivergenceError::GuestExited {
                reason: format!("guest exited (code={exit_code}) without calling SYSCALL_WRITE_OUTPUT"),
            })?;

        let output = postcard::from_bytes(&output_bytes)?;
        Ok(output)
    }
}

impl<H: Harness> Executor<H> for OneShot<H> {
    fn check(&mut self, input: &H::Input) -> Result<(), DivergenceError> {
        OneShot::check(self, input)
    }
}

#[doc(hidden)]
pub struct DifferentialSyscalls {
    input_bytes: Payload,
    output_slot: OutputSlot,
    max_payload: usize,
    ready_count: u32,
}

impl DifferentialSyscalls {
    pub fn new(input_bytes: Payload, max_payload: usize) -> (Self, OutputSlot) {
        let slot: OutputSlot = Arc::new(Mutex::new(None));
        let this = Self { input_bytes, output_slot: slot.clone(), max_payload, ready_count: 0 };
        (this, slot)
    }

    pub fn handle_read_input<Mac: SupportMachine>(&self, machine: &mut Mac) -> Result<(), VmError> {
        let buf_addr = machine.registers()[A0].to_u64();
        let cap = machine.registers()[A1].to_u64() as usize;
        if self.input_bytes.len() > cap {
            return Err(VmError::External(format!(
                "guest buffer capacity {cap} < input payload {}",
                self.input_bytes.len()
            )));
        }
        machine.memory_mut().store_bytes(buf_addr, &self.input_bytes)?;
        machine.set_register(A0, Mac::REG::from_u64(self.input_bytes.len() as u64));
        Ok(())
    }

    pub fn handle_write_output<Mac: SupportMachine>(&mut self, machine: &mut Mac) -> Result<(), VmError> {
        let buf_addr = machine.registers()[A0].to_u64();
        let len = machine.registers()[A1].to_u64() as usize;
        if len > self.max_payload {
            return Err(VmError::External(format!(
                "guest output length {len} exceeds MAX_PAYLOAD_LEN {}",
                self.max_payload
            )));
        }
        let bytes = machine.memory_mut().load_bytes(buf_addr, len as u64)?;
        *self.output_slot.lock().expect("output slot poisoned") = Some(bytes.to_vec());
        machine.set_running(false);
        Ok(())
    }

    pub fn handle_signal_ready<Mac: SupportMachine>(&mut self, _machine: &mut Mac) -> Result<(), VmError> {
        self.ready_count = self.ready_count.saturating_add(1);
        Ok(())
    }
}

impl<Mac: SupportMachine> Syscalls<Mac> for DifferentialSyscalls {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), VmError> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, VmError> {
        let code = machine.registers()[A7].to_u64();
        match code {
            protocol::SYSCALL_READ_INPUT => {
                self.handle_read_input(machine)?;
                Ok(true)
            }
            protocol::SYSCALL_WRITE_OUTPUT => {
                self.handle_write_output(machine)?;
                Ok(true)
            }
            protocol::SYSCALL_SIGNAL_READY => {
                self.handle_signal_ready(machine)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

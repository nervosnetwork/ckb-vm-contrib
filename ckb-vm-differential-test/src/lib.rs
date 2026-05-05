#![cfg_attr(target_arch = "riscv64", no_std)]
#![cfg_attr(not(target_arch = "riscv64"), forbid(unsafe_op_in_unsafe_fn))]

#[cfg(target_arch = "riscv64")]
extern crate alloc;

pub mod protocol;

#[cfg(target_arch = "riscv64")]
pub mod guest;

// Re-exported so `harness!`/`entry!` can invoke `default_alloc!` etc. without the user crate
// taking a direct dependency on ckb-std.
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
        /// Identifies this harness inside a multi-harness guest binary. The
        /// host passes it as `argv[0]`; the `entry!`-generated dispatcher
        /// matches on it to pick which harness to run.
        const NAME: &'static str;

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

/// Defines one harness. Expands to:
///
/// * On riscv64 — a unit struct `$name` carrying an inherent `__guest_run`
///   method that drives the `port` closure in a serve-input/return-output
///   loop.
/// * On any other target — a unit struct `$name` implementing [`Harness`],
///   with `guest_elf()` lazily built and `reference()` invoking the
///   host-side closure.
///
/// `harness!` does **not** emit boot code; that lives in [`entry!`]. A user
/// crate must invoke `entry!` exactly once with the list of all harnesses
/// it defines, otherwise the guest binary will fail to link.
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
        pub struct $name;

        impl $name {
            /// Identifier matched against `argv[0]` by the `entry!` dispatcher.
            pub const NAME: &'static str = stringify!($name);
        }

        #[cfg(target_arch = "riscv64")]
        impl $name {
            #[doc(hidden)]
            pub fn __guest_run() -> i8 {
                $crate::guest::run(|input: $input| -> $output { ($port)(&input) })
            }
        }

        #[cfg(not(target_arch = "riscv64"))]
        impl $crate::Harness for $name {
            type Input = $input;
            type Output = $output;
            const NAME: &'static str = <$name>::NAME;

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
    };
}

/// Emits the guest binary's boot code and the dispatcher that picks which harness to run based on `argv[0]`.
/// Required exactly once per user crate when one or more `harness!`es are defined.
///
/// ```ignore
/// ckb_vm_differential_test::harness! { name: H1, ... }
/// ckb_vm_differential_test::harness! { name: H2, ... }
/// ckb_vm_differential_test::entry!(H1, H2);
/// ```
///
/// Expands to nothing on non-riscv64 targets.
#[macro_export]
macro_rules! entry {
    ($($harness:ident),+ $(,)?) => {
        #[cfg(target_arch = "riscv64")]
        extern crate alloc;

        // 16KB fixed + 1.2MB dynamic. Hand-roll your own `default_alloc!`
        // before invoking `entry!` if a particular harness needs more.
        #[cfg(target_arch = "riscv64")]
        $crate::ckb_std::default_alloc!(16384, 1258306, 64);

        #[cfg(target_arch = "riscv64")]
        #[unsafe(no_mangle)]
        unsafe extern "C" fn __ckb_vm_differential_main(
            argc: core::ffi::c_int,
            argv: *const $crate::ckb_std::env::Arg,
        ) -> i8 {
            let argv = unsafe { core::slice::from_raw_parts(argv, argc as usize) };
            unsafe { $crate::ckb_std::env::set_argv(argv) };
            let argv = $crate::ckb_std::env::argv();
            assert!(!argv.is_empty(), "differential test: no harness name in argv");
            let name = argv[0].to_bytes();
            $(
                if name == <$harness>::NAME.as_bytes() {
                    return $harness::__guest_run();
                }
            )+
            panic!("differential test: unknown harness in argv[0]")
        }

        #[cfg(target_arch = "riscv64")]
        core::arch::global_asm!(
            ".global _start",
            "_start:",
            "lw a0, 0(sp)",
            "addi a1, sp, 8",
            "li a2, 0",
            "call __ckb_vm_differential_main",
            "li a7, 93",
            "ecall",
        );

        #[cfg(target_arch = "riscv64")]
        #[panic_handler]
        fn __ckb_vm_differential_panic(info: &core::panic::PanicInfo) -> ! {
            use core::fmt::Write as _;
            let mut storage = [0u8; 1024];
            let mut sink = $crate::guest::PanicBuffer::new(&mut storage);
            let _ = write!(sink, "{info}");
            $crate::guest::report_panic(sink.as_slice());
            unreachable!("reported panic and halted");
        }
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

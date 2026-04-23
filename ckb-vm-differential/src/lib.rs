#![cfg_attr(target_arch = "riscv64", no_std)]

pub use ckb_vm_differential_protocol as protocol;

#[cfg(target_arch = "riscv64")]
pub use ckb_vm_differential_guest as guest;

#[cfg(not(target_arch = "riscv64"))]
pub use ckb_vm_differential_host as host;

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
        /* #region guest-side expansion */
        #[cfg(target_arch = "riscv64")]
        ::ckb_std::entry!(__ckb_vm_differential_guest_main);

        // Override by hand-rolling `default_alloc!` at the user-crate level if a particular harness needs more.
        #[cfg(target_arch = "riscv64")]
        ::ckb_std::default_alloc!(16384, 1258306, 64);

        #[cfg(target_arch = "riscv64")]
        fn __ckb_vm_differential_guest_main() -> i8 {
            $crate::guest::run(|input: $input| -> $output { ($port)(&input) })
        }
        /* #endregion */

        /* #region host-side expansion */
        #[cfg(not(target_arch = "riscv64"))]
        pub struct $name;

        #[cfg(not(target_arch = "riscv64"))]
        impl $crate::host::Harness for $name {
            type Input = $input;
            type Output = $output;

            fn guest_elf() -> &'static [u8] {
                static ELF: ::std::sync::OnceLock<::std::vec::Vec<u8>> = ::std::sync::OnceLock::new();
                ELF.get_or_init(|| {
                    let config: $crate::host::BuildConfig = $build;
                    $crate::host::build_guest_crate_with(env!("CARGO_MANIFEST_DIR"), &config)
                        .expect("build guest crate")
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
pub fn __default_build_config() -> ckb_vm_differential_host::BuildConfig {
    ckb_vm_differential_host::BuildConfig::default()
}

#[cfg(target_arch = "riscv64")]
#[doc(hidden)]
pub fn __default_build_config() {}

use ckb_vm_differential_test::warmstart_check;
use fips202_differential::{Sha3_256Harness, Sha3_512Harness, Shake128Harness, Shake256Harness};
use proptest::prelude::*;
use proptest::test_runner::FileFailurePersistence;

proptest! {
    #![proptest_config(ProptestConfig {
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("regressions"))),
        ..ProptestConfig::default()
    })]

    #[test]
    fn sha3_256_matches_reference(msg in proptest::collection::vec(any::<u8>(), 0..1024)) {
        warmstart_check::<Sha3_256Harness>(&msg)?;
    }

    #[test]
    fn sha3_512_matches_reference(msg in proptest::collection::vec(any::<u8>(), 0..1024)) {
        warmstart_check::<Sha3_512Harness>(&msg)?;
    }

    #[test]
    fn shake128_matches_reference(
        msg in proptest::collection::vec(any::<u8>(), 0..1024),
        out_len in 1u16..=512,
    ) {
        warmstart_check::<Shake128Harness>(&(msg, out_len))?;
    }

    #[test]
    fn shake256_matches_reference(
        msg in proptest::collection::vec(any::<u8>(), 0..1024),
        out_len in 1u16..=512,
    ) {
        warmstart_check::<Shake256Harness>(&(msg, out_len))?;
    }
}

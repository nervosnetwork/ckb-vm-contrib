use ckb_vm_differential::host::oneshot_check;
use proptest::prelude::*;
use proptest::test_runner::FileFailurePersistence;
use sha256_differential::Sha256Harness;

proptest! {
    #![proptest_config(ProptestConfig {
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("regressions"))),
        ..ProptestConfig::default()
    })]

    #[test]
    fn sha256_matches_reference(input in proptest::collection::vec(any::<u8>(), 0..1024)) {
        oneshot_check::<Sha256Harness>(&input)?;
    }
}

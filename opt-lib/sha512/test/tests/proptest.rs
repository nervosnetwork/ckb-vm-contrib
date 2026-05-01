use ckb_vm_differential_test::warmstart_check;
use proptest::prelude::*;
use proptest::test_runner::FileFailurePersistence;
use sha512_differential::Sha512Harness;

proptest! {
    #![proptest_config(ProptestConfig {
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("regressions"))),
        ..ProptestConfig::default()
    })]

    #[test]
    fn sha512_matches_reference(input in proptest::collection::vec(any::<u8>(), 0..1024)) {
        warmstart_check::<Sha512Harness>(&input)?;
    }
}

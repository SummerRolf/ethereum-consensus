// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::ssz_static::Eth1DataHandler as TestRunner;
use crate::test_utils::TestCase;

#[test]
fn test_case_0() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/phase0/ssz_static/Eth1Data/ssz_nil/case_0",
    );
    test_case.execute();
}
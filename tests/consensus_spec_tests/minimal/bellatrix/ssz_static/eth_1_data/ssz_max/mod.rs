// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::ssz_static::Eth1DataTestCase;
use crate::test_utils::TestCase;

#[test]
fn test_case_0() {
    let test_case = Eth1DataTestCase::from(
        "consensus-spec-tests/tests/minimal/bellatrix/ssz_static/Eth1Data/ssz_max/case_0",
    );

    test_case.execute();
}

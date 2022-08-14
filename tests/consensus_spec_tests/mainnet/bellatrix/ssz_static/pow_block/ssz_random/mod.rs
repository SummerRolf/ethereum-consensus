// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

#![cfg(not(feature = "bls"))]
use crate::spec_test_runners::ssz_static::PowBlockTestCase;
use ethereum_consensus::bellatrix::mainnet as spec;
use ssz_rs::prelude::*;

#[test]
fn test_case_0() {
    let test_case = PowBlockTestCase::from(
        "consensus-spec-tests/tests/mainnet/bellatrix/ssz_static/PowBlock/ssz_random/case_0",
    );

    test_case.execute(|encoding| {
        let mut data: spec::PowBlock = ssz_rs::deserialize(encoding).unwrap();
        let serialized = ssz_rs::serialize(&data).unwrap();
        let root = data.hash_tree_root().unwrap();
        (serialized, root)
    });
}

#[test]
fn test_case_1() {
    let test_case = PowBlockTestCase::from(
        "consensus-spec-tests/tests/mainnet/bellatrix/ssz_static/PowBlock/ssz_random/case_1",
    );

    test_case.execute(|encoding| {
        let mut data: spec::PowBlock = ssz_rs::deserialize(encoding).unwrap();
        let serialized = ssz_rs::serialize(&data).unwrap();
        let root = data.hash_tree_root().unwrap();
        (serialized, root)
    });
}

#[test]
fn test_case_2() {
    let test_case = PowBlockTestCase::from(
        "consensus-spec-tests/tests/mainnet/bellatrix/ssz_static/PowBlock/ssz_random/case_2",
    );

    test_case.execute(|encoding| {
        let mut data: spec::PowBlock = ssz_rs::deserialize(encoding).unwrap();
        let serialized = ssz_rs::serialize(&data).unwrap();
        let root = data.hash_tree_root().unwrap();
        (serialized, root)
    });
}

#[test]
fn test_case_3() {
    let test_case = PowBlockTestCase::from(
        "consensus-spec-tests/tests/mainnet/bellatrix/ssz_static/PowBlock/ssz_random/case_3",
    );

    test_case.execute(|encoding| {
        let mut data: spec::PowBlock = ssz_rs::deserialize(encoding).unwrap();
        let serialized = ssz_rs::serialize(&data).unwrap();
        let root = data.hash_tree_root().unwrap();
        (serialized, root)
    });
}

#[test]
fn test_case_4() {
    let test_case = PowBlockTestCase::from(
        "consensus-spec-tests/tests/mainnet/bellatrix/ssz_static/PowBlock/ssz_random/case_4",
    );

    test_case.execute(|encoding| {
        let mut data: spec::PowBlock = ssz_rs::deserialize(encoding).unwrap();
        let serialized = ssz_rs::serialize(&data).unwrap();
        let root = data.hash_tree_root().unwrap();
        (serialized, root)
    });
}

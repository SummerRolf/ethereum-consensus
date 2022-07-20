// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::epoch_processing::SlashingsTestCase;
use crate::test_utils::TestCase;
use ethereum_consensus::bellatrix::mainnet as spec;

#[test]
fn test_low_penalty() {
    let mut test_case = SlashingsTestCase::<spec::BeaconState>::from("consensus-spec-tests/tests/mainnet/bellatrix/epoch_processing/slashings/pyspec_tests/low_penalty");

    test_case.execute(|state, context| spec::process_slashings(state, context));
}

#[test]
fn test_max_penalties() {
    let mut test_case = SlashingsTestCase::<spec::BeaconState>::from("consensus-spec-tests/tests/mainnet/bellatrix/epoch_processing/slashings/pyspec_tests/max_penalties");

    test_case.execute(|state, context| spec::process_slashings(state, context));
}

#[test]
fn test_minimal_penalty() {
    let mut test_case = SlashingsTestCase::<spec::BeaconState>::from("consensus-spec-tests/tests/mainnet/bellatrix/epoch_processing/slashings/pyspec_tests/minimal_penalty");

    test_case.execute(|state, context| spec::process_slashings(state, context));
}

#[test]
fn test_scaled_penalties() {
    let mut test_case = SlashingsTestCase::<spec::BeaconState>::from("consensus-spec-tests/tests/mainnet/bellatrix/epoch_processing/slashings/pyspec_tests/scaled_penalties");

    test_case.execute(|state, context| spec::process_slashings(state, context));
}

#[test]
fn test_slashings_with_random_state() {
    let mut test_case = SlashingsTestCase::<spec::BeaconState>::from("consensus-spec-tests/tests/mainnet/bellatrix/epoch_processing/slashings/pyspec_tests/slashings_with_random_state");

    test_case.execute(|state, context| spec::process_slashings(state, context));
}

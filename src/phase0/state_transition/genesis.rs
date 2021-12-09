use crate::phase0::beacon_block::{BeaconBlock, BeaconBlockBody, BeaconBlockHeader};
use crate::phase0::beacon_state::BeaconState;
use crate::phase0::fork::Fork;
use crate::phase0::operations::{Deposit, DepositData, Eth1Data};
use crate::phase0::state_transition::block_processing::process_deposit;
use crate::phase0::state_transition::{get_active_validator_indices, Context, Error};
use crate::phase0::DEPOSIT_CONTRACT_TREE_DEPTH;
use crate::primitives::{Bytes32, Gwei, Hash32, GENESIS_EPOCH};
use ssz_rs::prelude::*;

const DEPOSIT_DATA_LIST_BOUND: usize = 2usize.pow(DEPOSIT_CONTRACT_TREE_DEPTH as u32);

pub fn initialize_beacon_state_from_eth1<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
>(
    eth1_block_hash: Hash32,
    eth1_timestamp: u64,
    deposits: &[Deposit],
    context: &Context,
) -> Result<
    BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    Error,
> {
    let merkleization_context = context.for_merkleization();
    let fork = Fork {
        previous_version: context.genesis_fork_version,
        current_version: context.genesis_fork_version,
        epoch: GENESIS_EPOCH,
    };
    let eth1_data = Eth1Data {
        block_hash: eth1_block_hash,
        deposit_count: deposits.len() as u64,
        ..Default::default()
    };
    let latest_block_body = BeaconBlockBody::<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
    >::default();
    let body_root = latest_block_body.hash_tree_root(merkleization_context)?;
    let latest_block_header = BeaconBlockHeader {
        body_root,
        ..Default::default()
    };
    let eth1_block_hash = Bytes32(Vector::from_iter(eth1_block_hash.0.into_iter()));
    let randao_mixes =
        Vector::from_iter(vec![eth1_block_hash; context.epochs_per_historical_vector]);
    let mut state = BeaconState {
        genesis_time: eth1_timestamp + context.genesis_delay,
        fork,
        eth1_data,
        latest_block_header,
        randao_mixes,
        ..Default::default()
    };

    let mut leaves = List::<DepositData, DEPOSIT_DATA_LIST_BOUND>::default();
    for deposit in deposits.iter() {
        leaves.push(deposit.data.clone());
        state.eth1_data.deposit_root = leaves.hash_tree_root(merkleization_context)?;
        process_deposit(&mut state, deposit, context)?;
    }

    for (i, validator) in state.validators.iter_mut().enumerate() {
        let balance = state.balances[i];
        let effective_balance = Gwei::min(
            balance - balance % context.effective_balance_increment,
            context.max_effective_balance,
        );
        validator.effective_balance = effective_balance;
        if validator.effective_balance == context.max_effective_balance {
            validator.activation_eligibility_epoch = GENESIS_EPOCH;
            validator.activation_epoch = GENESIS_EPOCH;
        }
    }

    state.genesis_validators_root = state.validators.hash_tree_root(merkleization_context)?;

    Ok(state)
}

pub fn is_valid_genesis_state<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    context: &Context,
) -> bool {
    if state.genesis_time < context.min_genesis_time {
        return false;
    }

    if get_active_validator_indices(state, GENESIS_EPOCH).len()
        < context.min_genesis_active_validator_count
    {
        return false;
    }

    true
}

pub fn get_genesis_block<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
>(
    genesis_state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    context: &Context,
) -> Result<
    BeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
    >,
    Error,
> {
    Ok(BeaconBlock {
        state_root: genesis_state.hash_tree_root(context.for_merkleization())?,
        ..Default::default()
    })
}

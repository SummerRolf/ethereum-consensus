pub use crate::domains::DomainType;
use crate::phase0::beacon_block::{BeaconBlock, BeaconBlockBody, BeaconBlockHeader};
use crate::phase0::beacon_state::BeaconState;
use crate::phase0::operations::{
    Attestation, AttesterSlashing, Deposit, ProposerSlashing, SignedVoluntaryExit,
};
use crate::phase0::state_transition::{
    compute_epoch_at_slot, compute_signing_root, get_beacon_proposer_index, get_current_epoch,
    get_domain, get_randao_mix, hash, invalid_header_error, invalid_operation_error,
    is_slashable_validator, slash_validator, Context, Error, InvalidBeaconBlockHeader,
    InvalidDeposit, InvalidOperation, InvalidProposerSlashing,
};
use ssz_rs::prelude::*;

pub fn process_proposer_slashing<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    proposer_slashing: &mut ProposerSlashing,
    context: &Context,
) -> Result<(), Error> {
    let header_1 = &proposer_slashing.signed_header_1.message;
    let header_2 = &proposer_slashing.signed_header_2.message;

    if header_1.slot != header_2.slot {
        return Err(invalid_operation_error(InvalidOperation::ProposerSlashing(
            InvalidProposerSlashing::SlotMismatch(header_1.slot, header_2.slot),
        )));
    }

    if header_1.proposer_index != header_2.proposer_index {
        return Err(invalid_operation_error(InvalidOperation::ProposerSlashing(
            InvalidProposerSlashing::ProposerMismatch(
                header_1.proposer_index,
                header_2.proposer_index,
            ),
        )));
    }

    if header_1 == header_2 {
        return Err(invalid_operation_error(InvalidOperation::ProposerSlashing(
            InvalidProposerSlashing::HeadersAreEqual(header_1.clone()),
        )));
    }

    let proposer_index = header_1.proposer_index;
    let proposer = &state.validators[proposer_index];
    if !is_slashable_validator(proposer, get_current_epoch(state, context)) {
        return Err(invalid_operation_error(InvalidOperation::ProposerSlashing(
            InvalidProposerSlashing::ProposerIsNotSlashable(header_1.proposer_index),
        )));
    }

    let epoch = compute_epoch_at_slot(header_1.slot, context);
    let domain = get_domain(state, DomainType::BeaconProposer, Some(epoch), context)?;
    for signed_header in [
        &mut proposer_slashing.signed_header_1,
        &mut proposer_slashing.signed_header_2,
    ] {
        let signing_root = compute_signing_root(&mut signed_header.message, domain)?;
        let valid_signature = proposer
            .pubkey
            .verify_signature(signing_root.as_bytes(), &signed_header.signature);

        if !valid_signature {
            return Err(invalid_operation_error(InvalidOperation::ProposerSlashing(
                InvalidProposerSlashing::InvalidSignature(signed_header.signature.clone()),
            )));
        }
    }

    slash_validator(state, proposer_index, None, context)
}

pub fn process_attester_slashing<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    _state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    _attester_slashing: &AttesterSlashing<MAX_VALIDATORS_PER_COMMITTEE>,
    _context: &Context,
) -> Result<(), Error> {
    Ok(())
}

pub fn process_attestation<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    _state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    _attestation: &Attestation<MAX_VALIDATORS_PER_COMMITTEE>,
    _context: &Context,
) -> Result<(), Error> {
    Ok(())
}

pub fn process_deposit<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    _state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    _deposit: &Deposit,
    _context: &Context,
) -> Result<(), Error> {
    Ok(())
}

pub fn process_voluntary_exit<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    _state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    _signed_voluntary_exit: &SignedVoluntaryExit,
    _context: &Context,
) -> Result<(), Error> {
    Ok(())
}

fn process_block_header<
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
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    block: &mut BeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
    >,
    context: &Context,
) -> Result<(), Error> {
    if block.slot != state.slot {
        return Err(invalid_header_error(
            InvalidBeaconBlockHeader::StateSlotMismatch {
                state_slot: state.slot,
                block_slot: block.slot,
            },
        ));
    }

    if block.slot <= state.latest_block_header.slot {
        return Err(invalid_header_error(
            InvalidBeaconBlockHeader::OlderThanLatestBlockHeader {
                block_slot: block.slot,
                latest_block_header_slot: state.latest_block_header.slot,
            },
        ));
    }

    let proposer_index = get_beacon_proposer_index(state, context)?;
    if block.proposer_index != proposer_index {
        return Err(invalid_header_error(
            InvalidBeaconBlockHeader::ProposerIndexMismatch {
                block_proposer_index: block.proposer_index,
                proposer_index,
            },
        ));
    }

    let expected_parent_root = state.latest_block_header.hash_tree_root()?;
    if block.parent_root != expected_parent_root {
        return Err(invalid_header_error(
            InvalidBeaconBlockHeader::ParentBlockRootMismatch {
                expected: expected_parent_root,
                provided: block.parent_root,
            },
        ));
    }

    state.latest_block_header = BeaconBlockHeader {
        slot: block.slot,
        proposer_index: block.proposer_index,
        parent_root: block.parent_root,
        body_root: block.body.hash_tree_root()?,
        ..Default::default()
    };

    let proposer = &state.validators[block.proposer_index];
    if proposer.slashed {
        return Err(invalid_header_error(
            InvalidBeaconBlockHeader::ProposerSlashed(proposer_index),
        ));
    }

    Ok(())
}

fn process_randao<
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
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    body: &BeaconBlockBody<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
    >,
    context: &Context,
) -> Result<(), Error> {
    let mut epoch = get_current_epoch(state, context);

    let proposer_index = get_beacon_proposer_index(state, context)?;
    let proposer = &state.validators[proposer_index];

    let domain = get_domain(state, DomainType::Randao, Some(epoch), context)?;
    let signing_root = compute_signing_root(&mut epoch, domain)?;

    if !body
        .randao_reveal
        .verify(&proposer.pubkey, signing_root.as_bytes())
    {
        return Err(invalid_operation_error(InvalidOperation::Randao(
            body.randao_reveal.clone(),
        )));
    }

    let mix = get_randao_mix(state, epoch).xor(hash(body.randao_reveal.as_bytes()));
    let mix_index = epoch % context.epochs_per_historical_vector;
    state.randao_mixes[mix_index as usize] = mix;
    Ok(())
}

fn process_eth1_data<
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
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    body: &BeaconBlockBody<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
    >,
    context: &Context,
) {
    state.eth1_data_votes.push(body.eth1_data.clone());

    let votes_count = state
        .eth1_data_votes
        .iter()
        .filter(|&vote| *vote == body.eth1_data)
        .count() as u64;

    if votes_count * 2 > context.epochs_per_eth1_voting_period * context.slots_per_epoch {
        state.eth1_data = body.eth1_data.clone();
    }
}

fn process_operations<
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
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    body: &mut BeaconBlockBody<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
    >,
    context: &Context,
) -> Result<(), Error> {
    let expected_deposit_count = usize::min(
        context.max_deposits,
        (state.eth1_data.deposit_count - state.eth1_deposit_index) as usize,
    );

    if body.deposits.len() != expected_deposit_count {
        return Err(invalid_operation_error(InvalidOperation::Deposit(
            InvalidDeposit::IncorrectCount {
                expected: expected_deposit_count,
                count: body.deposits.len(),
            },
        )));
    }

    body.proposer_slashings
        .iter_mut()
        .try_for_each(|op| process_proposer_slashing(state, op, context))?;
    body.attester_slashings
        .iter()
        .try_for_each(|op| process_attester_slashing(state, op, context))?;
    body.attestations
        .iter()
        .try_for_each(|op| process_attestation(state, op, context))?;
    body.deposits
        .iter()
        .try_for_each(|op| process_deposit(state, op, context))?;
    body.voluntary_exits
        .iter()
        .try_for_each(|op| process_voluntary_exit(state, op, context))?;
    Ok(())
}

pub fn process_block<
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
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    block: &mut BeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
    >,
    context: &Context,
) -> Result<(), Error> {
    process_block_header(state, block, context)?;
    process_randao(state, &block.body, context)?;
    process_eth1_data(state, &block.body, context);
    process_operations(state, &mut block.body, context)?;
    Ok(())
}

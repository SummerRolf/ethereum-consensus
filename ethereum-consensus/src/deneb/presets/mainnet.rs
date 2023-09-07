use crate::deneb::spec;
pub use crate::{
    capella::presets::mainnet::{
        AggregateAndProof, Attestation, AttesterSlashing, ContributionAndProof, HistoricalBatch,
        IndexedAttestation, LightClientUpdate, PendingAttestation, SignedAggregateAndProof,
        SignedContributionAndProof, SyncAggregate, SyncCommittee, SyncCommitteeContribution,
        BYTES_PER_LOGS_BLOOM, EPOCHS_PER_HISTORICAL_VECTOR, EPOCHS_PER_SLASHINGS_VECTOR,
        ETH1_DATA_VOTES_BOUND, HISTORICAL_ROOTS_LIMIT, MAX_ATTESTATIONS, MAX_ATTESTER_SLASHINGS,
        MAX_BLS_TO_EXECUTION_CHANGES, MAX_BYTES_PER_TRANSACTION, MAX_DEPOSITS,
        MAX_EXTRA_DATA_BYTES, MAX_PROPOSER_SLASHINGS, MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_VALIDATORS_PER_COMMITTEE, MAX_VOLUNTARY_EXITS, MAX_WITHDRAWALS_PER_PAYLOAD,
        SLOTS_PER_HISTORICAL_ROOT, SYNC_COMMITTEE_SIZE, VALIDATOR_REGISTRY_LIMIT,
    },
    deneb::{networking::MAX_REQUEST_BLOCKS_DENEB, presets::Preset},
};

pub use spec::*;

pub const FIELD_ELEMENTS_PER_BLOB: usize = 4096;
pub const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize = 4096;
pub const MAX_BLOBS_PER_BLOCK: usize = 6;

pub const BYTES_PER_BLOB: usize =
    crate::deneb::polynomial_commitments::BYTES_PER_FIELD_ELEMENT * FIELD_ELEMENTS_PER_BLOB;

pub const MAX_REQUEST_BLOB_SIDECARS: usize = MAX_REQUEST_BLOCKS_DENEB * MAX_BLOBS_PER_BLOCK;

pub const PRESET: Preset = Preset {
    field_elements_per_blob: FIELD_ELEMENTS_PER_BLOB,
    max_blob_commitments_per_block: MAX_BLOB_COMMITMENTS_PER_BLOCK,
    max_blobs_per_block: MAX_BLOBS_PER_BLOCK,
};

pub type ExecutionPayload = spec::ExecutionPayload<
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
>;

pub type ExecutionPayloadHeader =
    spec::ExecutionPayloadHeader<BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES>;

pub type BlindedBeaconBlockBody = spec::BlindedBeaconBlockBody<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOB_COMMITMENTS_PER_BLOCK,
>;

pub type BlindedBeaconBlock = spec::BlindedBeaconBlock<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOB_COMMITMENTS_PER_BLOCK,
>;

pub type SignedBlindedBeaconBlock = spec::SignedBlindedBeaconBlock<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOB_COMMITMENTS_PER_BLOCK,
>;

pub type BeaconState = spec::BeaconState<
    SLOTS_PER_HISTORICAL_ROOT,
    HISTORICAL_ROOTS_LIMIT,
    ETH1_DATA_VOTES_BOUND,
    VALIDATOR_REGISTRY_LIMIT,
    EPOCHS_PER_HISTORICAL_VECTOR,
    EPOCHS_PER_SLASHINGS_VECTOR,
    MAX_VALIDATORS_PER_COMMITTEE,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
>;

pub type BeaconBlockBody = spec::BeaconBlockBody<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOBS_PER_BLOCK,
>;

pub type BeaconBlock = spec::BeaconBlock<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOBS_PER_BLOCK,
>;

pub type SignedBeaconBlock = spec::SignedBeaconBlock<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOBS_PER_BLOCK,
>;

pub type Blob = spec::Blob<BYTES_PER_BLOB>;
pub type BlobSidecar = spec::BlobSidecar<BYTES_PER_BLOB>;
pub type SignedBlobSidecar = spec::SignedBlobSidecar<BYTES_PER_BLOB>;

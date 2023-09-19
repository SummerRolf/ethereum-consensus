use crate::{
    configs::Config,
    networks::Network,
    primitives::{Epoch, ExecutionAddress, Gwei, Version, FAR_FUTURE_EPOCH, U256},
};

pub const MIN_GENESIS_ACTIVE_VALIDATOR_COUNT: usize = 16384;
pub const MIN_GENESIS_TIME: u64 = 1694786400;
pub const GENESIS_FORK_VERSION: Version = [0, 1, 112, 0];
pub const GENESIS_DELAY: u64 = 300;
pub const SECONDS_PER_SLOT: u64 = 12;
pub const SECONDS_PER_ETH1_BLOCK: u64 = 14;
pub const MIN_VALIDATOR_WITHDRAWABILITY_DELAY: Epoch = 256;
pub const SHARD_COMMITTEE_PERIOD: Epoch = 256;
pub const ETH1_FOLLOW_DISTANCE: u64 = 2048;
pub const EJECTION_BALANCE: Gwei = 28 * 10u64.pow(9);
pub const MIN_PER_EPOCH_CHURN_LIMIT: u64 = 4;
pub const MAX_PER_EPOCH_ACTIVATION_CHURN_LIMIT: u64 = 8;
pub const CHURN_LIMIT_QUOTIENT: u64 = 65536;
pub const TERMINAL_BLOCK_HASH_ACTIVATION_EPOCH: Epoch = FAR_FUTURE_EPOCH;
pub const ALTAIR_FORK_VERSION: Version = [16, 1, 112, 0];
pub const ALTAIR_FORK_EPOCH: Epoch = 0;
pub const BELLATRIX_FORK_VERSION: Version = [32, 1, 112, 0];
pub const BELLATRIX_FORK_EPOCH: Epoch = 0;
pub const CAPELLA_FORK_VERSION: Version = [48, 1, 112, 0];
pub const CAPELLA_FORK_EPOCH: Epoch = 256;
pub const DENEB_FORK_VERSION: Version = [64, 1, 112, 0];
pub const DENEB_FORK_EPOCH: Epoch = FAR_FUTURE_EPOCH;
pub const INACTIVITY_SCORE_BIAS: u64 = 4;
pub const INACTIVITY_SCORE_RECOVERY_RATE: u64 = 16;
pub const PROPOSER_SCORE_BOOST: u64 = 40;
pub const DEPOSIT_CHAIN_ID: usize = 17000;
pub const DEPOSIT_NETWORK_ID: usize = 17000;

pub fn config() -> Config {
    let terminal_total_difficulty = U256::zero();
    let terminal_block_hash = Default::default();
    let deposit_contract_address = ExecutionAddress::try_from(
        [
            // 0x4242424242424242424242424242424242424242
            66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
        ]
        .as_ref(),
    )
    .unwrap();

    Config {
        preset_base: "mainnet".to_string(),
        name: Network::Holesky,
        terminal_total_difficulty,
        terminal_block_hash,
        terminal_block_hash_activation_epoch: TERMINAL_BLOCK_HASH_ACTIVATION_EPOCH,
        min_genesis_active_validator_count: MIN_GENESIS_ACTIVE_VALIDATOR_COUNT,
        min_genesis_time: MIN_GENESIS_TIME,
        genesis_fork_version: GENESIS_FORK_VERSION,
        genesis_delay: GENESIS_DELAY,
        altair_fork_version: ALTAIR_FORK_VERSION,
        altair_fork_epoch: ALTAIR_FORK_EPOCH,
        bellatrix_fork_version: BELLATRIX_FORK_VERSION,
        bellatrix_fork_epoch: BELLATRIX_FORK_EPOCH,
        capella_fork_version: CAPELLA_FORK_VERSION,
        capella_fork_epoch: CAPELLA_FORK_EPOCH,
        deneb_fork_version: DENEB_FORK_VERSION,
        deneb_fork_epoch: DENEB_FORK_EPOCH,
        seconds_per_slot: SECONDS_PER_SLOT,
        seconds_per_eth1_block: SECONDS_PER_ETH1_BLOCK,
        min_validator_withdrawability_delay: MIN_VALIDATOR_WITHDRAWABILITY_DELAY,
        shard_committee_period: SHARD_COMMITTEE_PERIOD,
        eth1_follow_distance: ETH1_FOLLOW_DISTANCE,
        inactivity_score_bias: INACTIVITY_SCORE_BIAS,
        inactivity_score_recovery_rate: INACTIVITY_SCORE_RECOVERY_RATE,
        ejection_balance: EJECTION_BALANCE,
        min_per_epoch_churn_limit: MIN_PER_EPOCH_CHURN_LIMIT,
        max_per_epoch_activation_churn_limit: MAX_PER_EPOCH_ACTIVATION_CHURN_LIMIT,
        churn_limit_quotient: CHURN_LIMIT_QUOTIENT,
        proposer_score_boost: PROPOSER_SCORE_BOOST,
        deposit_chain_id: DEPOSIT_CHAIN_ID,
        deposit_network_id: DEPOSIT_NETWORK_ID,
        deposit_contract_address,
    }
}

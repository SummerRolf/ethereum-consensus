use crate::configs::Config;
use crate::primitives::{Epoch, /*ExecutionAddress,*/ Gwei, /*Hash32,*/ Version};
// use ssz_rs::prelude::U256;

pub const MIN_GENESIS_ACTIVE_VALIDATOR_COUNT: usize = 16384;
pub const MIN_GENESIS_TIME: u64 = 1606824000;
pub const GENESIS_FORK_VERSION: Version = [0u8; 4];
pub const GENESIS_DELAY: u64 = 604800;
pub const SECONDS_PER_SLOT: u64 = 12;
pub const SECONDS_PER_ETH1_BLOCK: u64 = 14;
pub const MIN_VALIDATOR_WITHDRAWABILITY_DELAY: Epoch = 256;
pub const SHARD_COMMITTEE_PERIOD: Epoch = 256;
pub const ETH1_FOLLOW_DISTANCE: u64 = 2048;
pub const EJECTION_BALANCE: Gwei = 16 * 10u64.pow(9);
pub const MIN_PER_EPOCH_CHURN_LIMIT: u64 = 4;
pub const CHURN_LIMIT_QUOTIENT: u64 = 65536;
// pub const TERMINAL_TOTAL_DIFFICULTY: U256 = Default::default();
// pub const TERMINAL_BLOCK_HASH: Hash32 = Default::default();
pub const TERMINAL_BLOCK_HASH_ACTIVATION_EPOCH: Epoch = 18446744073709551615;
pub const ALTAIR_FORK_VERSION: Version = [1, 0, 0, 0];
pub const ALTAIR_FORK_EPOCH: Epoch = 74240;
pub const BELLATRIX_FORK_VERSION: Version = [2, 0, 0, 0];
pub const BELLATRIX_FORK_EPOCH: Epoch = 18446744073709551615;
pub const CAPELLA_FORK_VERSION: Version = [3, 0, 0, 0];
pub const CAPELLA_FORK_EPOCH: Epoch = 18446744073709551615;
pub const SHARDING_FORK_VERSION: Version = [4, 0, 0, 0];
pub const SHARDING_FORK_EPOCH: Epoch = 18446744073709551615;
pub const INACTIVITY_SCORE_BIAS: u64 = 4;
pub const INACTIVITY_SCORE_RECOVERY_RATE: u64 = 16;
pub const PROPOSER_SCORE_BOOST: u64 = 70;
pub const DEPOSIT_CHAIN_ID: usize = 1;
pub const DEPOSIT_NETWORK_ID: usize = 1;
// pub const DEPOSIT_CONTRACT_ADDRESS: ExecutionAddress = Default::default();

pub fn config() -> Config {
    Config {
        name: "mainnet",
        terminal_total_difficulty: Default::default(),
        terminal_block_hash: Default::default(),
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
        sharding_fork_version: SHARDING_FORK_VERSION,
        sharding_fork_epoch: SHARDING_FORK_EPOCH,
        seconds_per_slot: SECONDS_PER_SLOT,
        seconds_per_eth1_block: SECONDS_PER_ETH1_BLOCK,
        min_validator_withdrawability_delay: MIN_VALIDATOR_WITHDRAWABILITY_DELAY,
        shard_committee_period: SHARD_COMMITTEE_PERIOD,
        eth1_follow_distance: ETH1_FOLLOW_DISTANCE,
        inactivity_score_bias: INACTIVITY_SCORE_BIAS,
        inactivity_score_recovery_rate: INACTIVITY_SCORE_RECOVERY_RATE,
        ejection_balance: EJECTION_BALANCE,
        min_per_epoch_churn_limit: MIN_PER_EPOCH_CHURN_LIMIT,
        churn_limit_quotient: CHURN_LIMIT_QUOTIENT,
        proposer_score_boost: PROPOSER_SCORE_BOOST,
        deposit_chain_id: DEPOSIT_CHAIN_ID,
        deposit_network_id: DEPOSIT_NETWORK_ID,
        deposit_contract_address: Default::default(),
    }
}

use near_sdk::NearToken;

pub const MINER_1: &str = "miner1.near";
pub const MINER_2: &str = "miner2.near";
pub const VALIDATOR_1: &str = "validator1.near";
pub const VALIDATOR_2: &str = "validator2.near";
pub const DEFAULT_MINER_ACCOUNT_ID: &str = MINER_1;
pub const DEFAULT_VALIDATOR_ACCOUNT_ID: &str = VALIDATOR_1;
pub const DEFAULT_TIMESTAMP: u64 = 100_000_000_000;
pub const DEFAULT_DEPOSIT: NearToken = NearToken::from_yoctonear(10u128.pow(24));
pub const DEFAULT_MESSAGE_TO_REQUEST: &str = "Should we add this new NFT to our protocol?";
pub const DEFAULT_REQUEST_ID: &str = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726";
pub const DEFAULT_MINER_ANSWER: &str = "83a297c4156180a209ab3b4be1f9bb55fe692dd02826a0265431d60c6e2ac871";
pub const DEFAULT_VALIDATOR_ANSWER: &str = "bf3250b68ca58d084d4898561d98d6fa9c97863ee644ff49f211ca425b0d6bf5";
pub const COMMIT_VALIDATOR_TIME: u64 = 100000000 + (7 * 60 * 1_000_000_000);
pub const REVEAL_MINER_TIME: u64 = 100000000 + (4 * 60 * 1_000_000_000);
pub const REVEAL_VALIDATOR_TIME: u64 = 100000000 + (8 * 60 * 1_000_000_000);

use near_sdk::NearToken;

pub const THIRTY_SECONDS: u64 = 30 * 1_000_000_000; // 30 seconds in nanoseconds

pub const COMMIT_MINER_DURATION: u64 = THIRTY_SECONDS;
pub const REVEAL_MINER_DURATION: u64 = THIRTY_SECONDS;
pub const COMMIT_VALIDATOR_DURATION: u64 = THIRTY_SECONDS;
pub const REVEAL_VALIDATOR_DURATION: u64 = THIRTY_SECONDS;
pub const MIN_MINER_STAKE: NearToken = NearToken::from_near(1); // 1 NEAR
pub const MIN_VALIDATOR_STAKE: NearToken = NearToken::from_near(2); // 10 NEAR
pub const PROTOCOL_REGISTRATION_FEE: NearToken = NearToken::from_near(1);

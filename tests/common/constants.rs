use near_sdk::NearToken;

pub const MINER_1: &str = "miner1.near";
pub const MINER_2: &str = "miner2.near";
pub const DEFAULT_MINER_ACCOUNT_ID: &str = MINER_1;
pub const DEFAULT_TIMESTAMP: u64 = 100_000_000_000;
pub const DEFAULT_DEPOSIT: NearToken = NearToken::from_yoctonear(10u128.pow(24));

use super::{
    constants::{DEFAULT_MINER_ACCOUNT_ID, DEFAULT_VALIDATOR_ACCOUNT_ID},
    types::Log,
};
use near_sdk::{test_utils::get_logs, AccountId};
use serde_json::{json, Value};

pub fn get_account_for_miner(miner: &str) -> AccountId {
    miner.parse().unwrap()
}

pub fn get_default_miner_account() -> AccountId {
    DEFAULT_MINER_ACCOUNT_ID.parse().unwrap()
}

pub fn get_account_for_validator(validator: &str) -> AccountId {
    validator.parse().unwrap()
}

pub fn get_default_validator_account() -> AccountId {
    DEFAULT_VALIDATOR_ACCOUNT_ID.parse().unwrap()
}
pub fn generate_validator_answer() -> Vec<AccountId> {
    let value = vec![
        "miner1.near".parse().unwrap(),
        "miner2.near".parse().unwrap(),
        "miner3.near".parse().unwrap(),
        "miner4.near".parse().unwrap(),
        "miner5.near".parse().unwrap(),
        "miner6.near".parse().unwrap(),
        "miner7.near".parse().unwrap(),
        "miner8.near".parse().unwrap(),
        "miner9.near".parse().unwrap(),
        "miner10.near".parse().unwrap(),
    ];
    value
}

pub fn group_registered_miners() -> Vec<AccountId> {
    let value = vec![
        "miner1.near".parse().unwrap(),
        "miner2.near".parse().unwrap(),
        "miner3.near".parse().unwrap(),
        "miner4.near".parse().unwrap(),
        "miner5.near".parse().unwrap(),
        "miner6.near".parse().unwrap(),
        "miner7.near".parse().unwrap(),
        "miner8.near".parse().unwrap(),
        "miner9.near".parse().unwrap(),
        "miner10.near".parse().unwrap(),
    ];
    value
}

pub fn assert_log(event_name: &str, data: Vec<(&str, &str)>) {
    let logs = get_logs();
    assert_eq!(logs.len(), 1);

    let mut data_map = serde_json::Map::new();
    for (key, value) in data {
        data_map.insert(key.to_string(), json!(value));
    }

    let expected_event = json!({
        "standard": "emip001",
        "version": "1.0.0",
        "event": event_name,
        "data": [data_map]
    });

    // Deserialize both JSON strings into `Value` objects for comparison
    let log_event: Value = serde_json::from_str(logs[0].trim_start_matches("EVENT_JSON:")).unwrap();
    let expected_event: Value = expected_event;

    // Compare json objects
    assert_eq!(log_event, expected_event);
}

pub fn assert_logs(expected_logs: Vec<Log>) {
    let logs = get_logs();
    assert_eq!(logs.len(), expected_logs.len());

    for (i, expected_log) in expected_logs.iter().enumerate() {
        match expected_log {
            Log::Event { event_name, data } => {
                let mut data_map = serde_json::Map::new();
                for (key, value) in data {
                    data_map.insert(key.to_string(), value.clone());
                }

                let expected_event = json!({
                    "standard": "emip001",
                    "version": "1.0.0",
                    "event": event_name,
                    "data": [data_map]
                });

                // Deserialize both JSON strings into `Value` objects for comparison
                let log_event: Value = serde_json::from_str(logs[i].trim_start_matches("EVENT_JSON:")).unwrap();
                let expected_event: Value = expected_event;

                // Compare json objects
                assert_eq!(log_event, expected_event);
            }
            Log::Message(expected_text) => {
                assert_eq!(logs[i], *expected_text);
            }
        }
    }
}

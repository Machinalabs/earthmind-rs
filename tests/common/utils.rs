use super::{
    constants::{DEFAULT_MINER_ACCOUNT_ID, DEFAULT_PROTOCOL_ACCOUNT_ID, DEFAULT_VALIDATOR_ACCOUNT_ID},
    types::Log,
};
use near_sdk::{test_utils::get_logs, AccountId};
use serde_json::{json, Value};

pub fn get_account_for_protocol(account: &str) -> AccountId {
    account.parse().unwrap()
}

pub fn get_default_protocol_account() -> AccountId {
    DEFAULT_PROTOCOL_ACCOUNT_ID.parse().unwrap()
}

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

pub fn default_miners_commit_answer() -> Vec<String> {
    let value = vec![
        "83a297c4156180a209ab3b4be1f9bb55fe692dd02826a0265431d60c6e2ac871".to_string(),
        "b6ddf58b65f644181e552accafe9518ba9492508ef3ed11f4ea3169020837cb6".to_string(),
        "9c46c81435ee97dd838114edc911ba8620cf48e496245ee8741aa2c16747e7dd".to_string(),
        "2c67d7f748b540a6fe41a6ac0cd538d422a895d9b9e416216136e2238d390f35".to_string(),
        "f5f5af5e2a852672616eccee25fc5ab651f4e408a45f2307ac7b318b3d48233b".to_string(),
        "6d30f164f418a39e0a767edddeb880e132d9f110d13817409cf0c484325ef570".to_string(),
        "bcae6fc0f564c0cec80344e8faf9674f91ab7130e6f92f770fdba64f977544b4".to_string(),
        "858416fce42f6d5f0fb3ee210a2fd98220d09c3cf698ed6c2e013c42c990cda9".to_string(),
        "0d88f6c7fac55bfedd5b9bc541f4642ae71eb413825606653d387eea52f98913".to_string(),
        "81db51808dc7c63a041cbb2dda9aa123d5132f4a6015e5aa2f25decdebd007d8".to_string(),
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

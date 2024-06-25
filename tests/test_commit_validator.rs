use near_sdk::NearToken;
use serde_json::json;

use common::constants::{COMMIT_VALIDATOR_TIME, DEFAULT_MESSAGE_TO_REQUEST, DEFAULT_REQUEST_ID, DEFAULT_VALIDATOR_ANSWER, VALIDATOR_1};
use common::environment::Environment;
use common::types::Log;
use common::utils::{assert_logs, get_default_validator_account};

use earthmind_rs::CommitValidatorResult;
use earthmind_rs::Contract;

pub mod common;

#[test]
fn test_commit_by_validator_when_validator_and_request_exist() {
    let validator = get_default_validator_account();
    let custom_deposit = NearToken::from_yoctonear(10u128.pow(25));

    Environment::with_account(validator.clone()).with_attached_deposit(custom_deposit).create();
    let mut contract = Contract::new();

    contract.register_validator();
    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());

    assert_logs(vec![
        Log::Event {
            event_name: "register_validator".to_string(),
            data: vec![("validator", json![VALIDATOR_1])],
        },
        Log::Event {
            event_name: "register_request".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID])],
        },
    ]);

    Environment::with_account(validator)
        .with_attached_deposit(custom_deposit)
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    let result = contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_eq!(result, CommitValidatorResult::Success);

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);
}

#[test]
#[should_panic]
fn test_commit_by_validator_when_validator_dont_registered_and_request_exist() {
    let validator = get_default_validator_account();
    let custom_deposit = NearToken::from_yoctonear(10u128.pow(25));

    Environment::with_account(validator.clone()).with_attached_deposit(custom_deposit).create();
    let mut contract = Contract::new();

    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());

    assert_logs(vec![Log::Event {
        event_name: "register_request".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID])],
    }]);

    Environment::with_account(validator)
        .with_attached_deposit(custom_deposit)
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    let result = contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_eq!(result, CommitValidatorResult::Fail);

    assert_logs(vec![Log::Message("Validator is not registered: validator1.near".to_string())]);
}

#[test]
fn test_commit_by_validator_when_validator_registered_and_request_dont_exist() {
    let validator = get_default_validator_account();
    let custom_deposit = NearToken::from_yoctonear(10u128.pow(25));

    Environment::with_account(validator).with_attached_deposit(custom_deposit).create();
    let mut contract = Contract::new();

    contract.register_validator();

    let result = contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_eq!(result, CommitValidatorResult::Fail);

    assert_logs(vec![
        Log::Event {
            event_name: "register_validator".to_string(),
            data: vec![("validator", json![VALIDATOR_1])],
        },
        Log::Message("Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string()),
    ]);
}

#[test]
fn test_commit_by_validator_when_miner_and_request_exist_and_commit_already() {
    let validator = get_default_validator_account();
    let custom_deposit = NearToken::from_yoctonear(10u128.pow(25));

    Environment::with_account(validator.clone()).with_attached_deposit(custom_deposit).create();
    let mut contract = Contract::new();

    contract.register_validator();

    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());

    assert_logs(vec![
        Log::Event {
            event_name: "register_validator".to_string(),
            data: vec![("validator", json![VALIDATOR_1])],
        },
        Log::Event {
            event_name: "register_request".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID])],
        },
    ]);

    Environment::with_account(validator)
        .with_attached_deposit(custom_deposit)
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    let result = contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_eq!(result, CommitValidatorResult::Fail);

    assert_logs(vec![
        Log::Event {
            event_name: "commit_validator".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
        },
        Log::Message("This validator have a commit answer: validator1.near".to_string()),
    ]);
}

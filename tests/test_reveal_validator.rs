use near_sdk::{AccountId, NearToken};
use serde_json::json;

use crate::common::utils::generate_validator_answer;
use common::constants::{
    COMMIT_VALIDATOR_TIME, DEFAULT_MESSAGE_TO_REQUEST, DEFAULT_REQUEST_ID, DEFAULT_VALIDATOR_ANSWER, REVEAL_VALIDATOR_TIME, VALIDATOR_1, VALIDATOR_2,
};
use common::environment::Environment;
use common::types::Log;
use common::utils::{assert_logs, get_account_for_validator, get_default_validator_account};

use earthmind_rs::{Contract, RevealValidatorResult};

pub mod common;

#[test]
fn test_reveal_by_validator() {
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

    Environment::with_account(validator.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    Environment::with_account(validator).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer.clone(), message.clone());

    assert_eq!(result, RevealValidatorResult::Success);
    assert_logs(vec![Log::Event {
        event_name: "reveal_validator".to_string(),
        data: vec![
            ("request_id", json![DEFAULT_REQUEST_ID]),
            ("answer", json![answer]),
            ("message", json![message]),
        ],
    }]);
}

#[test]
fn test_reveal_by_validator_when_validator_is_not_registered() {
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

    Environment::with_account(validator).with_block_timestamp(COMMIT_VALIDATOR_TIME).create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    let unregistered_validator = get_account_for_validator(VALIDATOR_2);
    Environment::with_account(unregistered_validator)
        .with_block_timestamp(REVEAL_VALIDATOR_TIME)
        .create();

    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer, message);

    assert_eq!(result, RevealValidatorResult::Fail);

    assert_logs(vec![Log::Message("Validator is not registered: validator2.near".to_string())]);
}

#[test]
fn test_reveal_by_validator_when_request_is_not_registered() {
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

    Environment::with_account(validator.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    Environment::with_account(validator).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let request_id_unregistered = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725".to_string();
    let answer: Vec<AccountId> = generate_validator_answer();
    let message = "It's a cool NFT".to_string();

    let result = contract.reveal_by_validator(request_id_unregistered, answer, message);

    assert_eq!(result, RevealValidatorResult::Fail);

    assert_logs(vec![Log::Message(
        "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725".to_string(),
    )]);
}

#[test]
fn test_reveal_by_validator_when_proposal_is_already_reveal() {
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

    Environment::with_account(validator.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    Environment::with_account(validator).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let answer: Vec<AccountId> = generate_validator_answer();
    let message = "It's a cool NFT".to_string();

    contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer.clone(), message.clone());

    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer.clone(), message.clone());

    assert_eq!(result, RevealValidatorResult::Fail);

    assert_logs(vec![
        Log::Event {
            event_name: "reveal_validator".to_string(),
            data: vec![
                ("request_id", json![DEFAULT_REQUEST_ID]),
                ("answer", json![answer]),
                ("message", json![message]),
            ],
        },
        Log::Message("Proposal already revealed".to_string()),
    ]);
}

#[test]
fn test_reveal_by_validator_when_answer_not_equal() {
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

    Environment::with_account(validator.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    Environment::with_account(validator).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let mut answer: Vec<AccountId> = generate_validator_answer();
    answer[9] = "jane.near".parse().unwrap();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer.clone(), message);

    assert_eq!(result, RevealValidatorResult::Fail);

    assert_logs(vec![Log::Message("Answer don't match".to_string())]);
}

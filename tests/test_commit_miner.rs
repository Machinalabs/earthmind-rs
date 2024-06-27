use near_sdk::test_utils::get_logs;
use near_sdk::NearToken;
use serde_json::json;

use common::constants::{DEFAULT_MESSAGE_TO_REQUEST, DEFAULT_MINER_ANSWER, DEFAULT_REQUEST_ID, MINER_1, MINER_2};
use common::environment::Environment;
use common::types::Log;
use common::utils::{assert_logs, get_account_for_miner, get_default_miner_account};

use earthmind_rs::{CommitMinerResult, Contract};

pub mod common;

#[test]
fn test_commit_by_miner_when_miner_and_request_exist() {
    let miner = get_default_miner_account();
    let deposit = NearToken::from_near(5);
    Environment::with_account(miner).with_attached_deposit(deposit).create();

    let mut contract = Contract::new();
    contract.register_protocol();
    contract.register_miner();
    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());

    let result = contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

    assert_eq!(result, CommitMinerResult::Success);

    assert_logs(vec![
        Log::Event {
            event_name: "register_protocol".to_string(),
            data: vec![("account", json![MINER_1])],
        },
        Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![MINER_1])],
        },
        Log::Event {
            event_name: "register_request".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID])],
        },
        Log::Event {
            event_name: "commit_miner".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_MINER_ANSWER])],
        },
    ]);
}

#[test]
fn test_commit_by_miner_when_miner_dont_registered_and_request_exist() {
    let miner = get_default_miner_account();
    let deposit =  NearToken::from_near(5);

    Environment::with_account(miner).with_attached_deposit(deposit).create();

    let mut contract = Contract::new();
    contract.register_protocol();
    contract.register_miner();
    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());

    let unregistered_miner = get_account_for_miner(MINER_2);
    Environment::with_account(unregistered_miner).create();

    let result = contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

    assert_eq!(result, CommitMinerResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "Miner not registered: miner2.near");
}

#[test]
fn test_commit_by_miner_when_miner_registered_and_request_dont_exist() {
    let miner = get_default_miner_account();
    let deposit = NearToken::from_near(5);

    Environment::with_account(miner).with_attached_deposit(deposit).create();
    let mut contract = Contract::new();

    contract.register_protocol();
    contract.register_miner();

    let result = contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

    assert_eq!(result, CommitMinerResult::Fail);

    assert_logs(vec![
        Log::Event {
            event_name: "register_protocol".to_string(),
            data: vec![("account", json![MINER_1])],
        },
        Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![MINER_1])],
        },
        Log::Message("Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string()),
    ]);
}

#[test]
fn test_commit_by_miner_when_miner_and_request_exist_and_commit_already() {
    let miner = get_default_miner_account();
    let deposit = NearToken::from_near(5);

    Environment::with_account(miner.clone()).with_attached_deposit(deposit).create();
    let mut contract = Contract::new();

    contract.register_protocol();
    contract.register_miner();
    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());
    contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

    Environment::with_account(miner).create();

    let result = contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

    assert_eq!(result, CommitMinerResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "This miner have a commit answer: miner1.near");
}

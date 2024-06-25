use near_workspaces::AccountId;
use serde_json::json;

use common::constants::{DEFAULT_MESSAGE_TO_REQUEST, DEFAULT_REQUEST_ID};
use common::environment::Environment;
use common::types::Log;
use common::utils::{assert_logs, generate_validator_answer, get_default_miner_account, get_default_validator_account};

use earthmind_rs::{Contract, RegisterRequestResult};

pub mod common;

#[test]
fn test_request_governance_decision_when_is_registered_returns_already_registered() {
    let miner = get_default_miner_account();

    Environment::with_account(miner.clone()).create();

    let mut contract = Contract::new();
    contract.register_miner();
    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());

    let result = contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());

    assert_eq!(result, RegisterRequestResult::AlreadyRegistered);

    assert_logs(vec![
        Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![miner])],
        },
        Log::Event {
            event_name: "register_request".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID])],
        },
        Log::Message("Attempted to register an already registered request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string()),
    ]);
}

// Hash miner answer

#[test]
fn test_hash_miner_answer() {
    let miner = get_default_miner_account();

    Environment::with_account(miner).create();

    let mut contract = Contract::new();
    contract.register_miner();
    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());

    let request_id = DEFAULT_REQUEST_ID.to_string();
    let answer = true;
    let message = "It's a cool NFT".to_string();

    let result = contract.hash_miner_answer(request_id, answer, message);

    assert_eq!(result, "83a297c4156180a209ab3b4be1f9bb55fe692dd02826a0265431d60c6e2ac871");
}

// Hash validator answer
#[test]
fn test_hash_validator_answer() {
    let validator = get_default_validator_account();

    Environment::with_account(validator).create();
    let mut contract = Contract::new();
    contract.register_miner();

    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());

    let request_id = DEFAULT_REQUEST_ID.to_string();
    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();

    let result = contract.hash_validator_answer(request_id, answer, message);

    assert_eq!(result, "bf3250b68ca58d084d4898561d98d6fa9c97863ee644ff49f211ca425b0d6bf5");
}

#[test]
#[should_panic]
fn test_hash_validator_answer_when_answer_is_not_complete() {
    let validator = get_default_validator_account();

    Environment::with_account(validator).create();

    let mut contract = Contract::new();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = generate_validator_answer();
    let answer: Vec<AccountId> = answer[0..answer.len() - 1].to_vec();
    let message = "It's a cool NFT".to_string();

    contract.hash_validator_answer(request_id, answer, message);
}

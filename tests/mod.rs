extern crate earthmind_rs;
use earthmind_rs::models::models::{
    CommitMinerResult, CommitValidatorResult, RegisterMinerResult, RegisterRequestResult,
    RegisterValidatorResult, RevealMinerResult, RevealValidatorResult,
};
use earthmind_rs::Contract;
use near_sdk::{
    env,
    test_utils::{get_logs, VMContextBuilder},
    testing_env, AccountId,
};

fn generate_validator_answer() -> Vec<AccountId> {
    let value = vec![
        "hassel.near".parse().unwrap(),
        "edson.near".parse().unwrap(),
        "anne.near".parse().unwrap(),
        "bob.near".parse().unwrap(),
        "alice.near".parse().unwrap(),
        "john.near".parse().unwrap(),
        "harry.near".parse().unwrap(),
        "scott.near".parse().unwrap(),
        "felix.near".parse().unwrap(),
        "margaret.near".parse().unwrap(),
    ];
    return value;
}

fn get_context(predecessor_account_id: AccountId, block_timestamp: u64) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .predecessor_account_id(predecessor_account_id)
        .block_timestamp(block_timestamp);
    builder
}

// Register Miner

#[test]
fn test_register_miner() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    let result_1 = contract.register_miner();
    assert_eq!(result_1, RegisterMinerResult::Success);

    let miner_1: AccountId = "hassel.near".parse().unwrap();
    assert!(contract.get_register_miner(miner_1).is_some());

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "Registered new miner: hassel.near");

    let context = get_context("edson.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let result_2 = contract.register_miner();
    assert_eq!(result_2, RegisterMinerResult::Success);

    let miner_2: AccountId = "edson.near".parse().unwrap();
    assert!(contract.get_register_miner(miner_2).is_some());

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "Registered new miner: edson.near");
}

#[test]
fn test_register_miner_when_is_registered_returns_already_registered() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();
    let result = contract.register_miner();

    assert_eq!(result, RegisterMinerResult::AlreadyRegistered);

    let logs = get_logs();

    assert_eq!(logs.len(), 2);

    assert_eq!(logs[0], "Registered new miner: hassel.near");
    assert_eq!(
        logs[1],
        "Attempted to register an already registered miner: hassel.near"
    );
}

#[test]
fn test_get_register_miner() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();

    let miner: AccountId = "hassel.near".parse().unwrap();
    assert!(contract.get_register_miner(miner).is_some());
}

#[test]
fn test_get_register_miner_when_not_registered() {
    let contract = Contract::new();

    let miner: AccountId = "hassel.near".parse().unwrap();

    assert!(contract.get_register_miner(miner).is_none());
}

// Register Validator

#[test]
fn test_register_validator() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    let result_1 = contract.register_validator();
    assert_eq!(result_1, RegisterValidatorResult::Success);

    let validator_1: AccountId = "hassel.near".parse().unwrap();
    assert!(contract.get_register_validator(validator_1).is_some());

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "Registered new validator: hassel.near");

    let context = get_context("edson.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let result2 = contract.register_validator();
    assert_eq!(result2, RegisterValidatorResult::Success);

    let validator_2: AccountId = "edson.near".parse().unwrap();
    assert!(contract.get_register_validator(validator_2).is_some());

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "Registered new validator: edson.near");
}

#[test]
fn test_register_validator_when_is_registered_returns_already_registered() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let result = contract.register_validator();

    assert_eq!(result, RegisterValidatorResult::AlreadyRegistered);

    let logs = get_logs();

    assert_eq!(logs.len(), 2);

    assert_eq!(logs[0], "Registered new validator: hassel.near");
    assert_eq!(
        logs[1],
        "Attempted to register an already registered validator: hassel.near asd"
    );
}

#[test]
fn test_get_register_validator() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();
    contract.register_validator();

    let validator: AccountId = "hassel.near".parse().unwrap();
    assert!(contract.get_register_validator(validator).is_some());
}

#[test]
fn test_get_register_validator_when_not_registered() {
    let contract = Contract::new();
    let validator: AccountId = "hassel.near".parse().unwrap();

    assert!(contract.get_register_validator(validator).is_none());
}

// Request Governance Decision

#[test]
fn test_request_governance_decision() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    let message = "Should we add this new NFT to our protocol?";

    let result_1 = contract.request_governance_decision(message.to_string());
    assert_eq!(result_1, RegisterRequestResult::Success);

    let request_id = env::keccak256(message.as_bytes());
    let request_id_hex = hex::encode(request_id);
    assert!(contract.get_request_by_id(request_id_hex).is_some());

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(
        logs[0],
        "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
    );

    let context = get_context("edson.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let message_2 = "Should we add this to our protocol?";
    let result_2 = contract.request_governance_decision(message_2.to_string());
    assert_eq!(result_2, RegisterRequestResult::Success);

    let request_id_2 = env::keccak256(message_2.as_bytes());
    let request_id_hex_2 = hex::encode(request_id_2);
    assert!(contract.get_request_by_id(request_id_hex_2).is_some());

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(
        logs[0],
        "Registered new request: 38d15af71379737839e4738066fd4091428081d6a57498b2852337a195bc9f5f"
    );
}

#[test]
fn test_request_governance_decision_when_is_registered_returns_already_registered() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let result = contract.request_governance_decision(message.to_string());

    assert_eq!(result, RegisterRequestResult::AlreadyRegistered);

    let logs = get_logs();
    assert_eq!(logs.len(), 2);

    assert_eq!(
        logs[0],
        "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
    );
    assert_eq!(
            logs[1],
            "Attempted to register an already registered request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
        );
}

// Request by id

#[test]
fn test_get_request_by_id() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726";
    assert!(contract.get_request_by_id(request_id.to_string()).is_some());
}

#[test]
fn test_get_request_by_id_when_not_registered() {
    let mut contract = Contract::new();
    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae727";

    assert!(contract.get_request_by_id(request_id.to_string()).is_none());
}

// Hash miner answer

#[test]
fn test_hash_miner_answer() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = true;
    let message = "It's a cool NFT".to_string();

    let result = contract.hash_miner_answer(request_id, answer, message);

    assert_eq!(
        result,
        "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464"
    );
}

// Hash validator answer
#[test]
fn test_hash_validator_answer() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();

    let result = contract.hash_validator_answer(request_id, answer, message);

    assert_eq!(
        result,
        "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856"
    );
}

#[test]
#[should_panic]
fn test_hash_validator_answer_when_answer_is_not_complete() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = generate_validator_answer();
    let answer: Vec<AccountId> = answer[0..answer.len() - 1].to_vec();
    let message = "It's a cool NFT".to_string();
    contract.hash_validator_answer(request_id, answer.clone(), message);
}

// Commit by miner

#[test]
fn test_commit_by_miner_when_miner_and_request_exist() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

    let result = contract.commit_by_miner(request_id, answer);

    assert_eq!(result, CommitMinerResult::Success);

    let logs = get_logs();
    assert_eq!(logs.len(), 3);

    assert_eq!(logs[0], "Registered new miner: hassel.near");
    assert_eq!(
        logs[1],
        "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
    );
    assert_eq!(logs[2], "Miner proposal registered successfully");
}

#[test]
fn test_commit_by_miner_when_miner_dont_registered_and_request_exist() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let context = get_context("edson.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

    let result = contract.commit_by_miner(request_id, answer);

    assert_eq!(result, CommitMinerResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "Miner not registered: edson.near");
}

#[test]
fn test_commit_by_miner_when_miner_registered_and_request_dont_exist() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

    let result = contract.commit_by_miner(request_id, answer);

    assert_eq!(result, CommitMinerResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0], "Registered new miner: hassel.near");
    assert_eq!(logs[1], "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726")
}

#[test]
fn test_commit_by_miner_when_miner_and_request_exist_and_commit_already() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();
    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

    contract.commit_by_miner(request_id.clone(), answer.clone());

    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let result = contract.commit_by_miner(request_id, answer);

    assert_eq!(result, CommitMinerResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "This miner have a commit answer: hassel.near");
}

// Commit by validator

#[test]
fn test_commit_by_validator_when_validator_and_request_exist() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let logs = get_logs();
    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0], "Registered new validator: hassel.near");
    assert_eq!(
        logs[1],
        "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
    );

    let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), commit_validator_time);
    testing_env!(context.build());

    let result = contract.commit_by_validator(request_id, answer);

    assert_eq!(result, CommitValidatorResult::Success);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "Validator proposal registered successfully");
}

#[test]
fn test_commit_by_validator_when_validator_dont_registered_and_request_exist() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let context = get_context("edson.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let result = contract.commit_by_validator(request_id, answer);

    assert_eq!(result, CommitValidatorResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "Validator is not registered: edson.near");
}

#[test]
fn test_commit_by_validator_when_validator_registered_and_request_dont_exist() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let result = contract.commit_by_validator(request_id, answer);

    assert_eq!(result, CommitValidatorResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0], "Registered new validator: hassel.near");
    assert_eq!(logs[1], "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726")
}

#[test]
fn test_commit_by_validator_when_miner_and_request_exist_and_commit_already() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), commit_validator_time);
    testing_env!(context.build());

    contract.commit_by_validator(request_id.clone(), answer.clone());

    let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000) + 1;
    let context = get_context("hassel.near".parse().unwrap(), commit_validator_time);
    testing_env!(context.build());

    let result = contract.commit_by_validator(request_id, answer);

    assert_eq!(result, CommitValidatorResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "This validator have a commit answer: hassel.near");
}

// Reveal by miner

#[test]
fn test_reveal_by_miner() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

    contract.commit_by_miner(request_id.clone(), answer);

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = true;
    let message = "It's a cool NFT".to_string();

    let logs = get_logs();
    assert_eq!(logs.len(), 3);

    assert_eq!(logs[0], "Registered new miner: hassel.near");
    assert_eq!(
        logs[1],
        "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
    );
    assert_eq!(logs[2], "Miner proposal registered successfully");

    let reveal_miner_time = 100000000 + (3 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), reveal_miner_time);
    testing_env!(context.build());
    let result = contract.reveal_by_miner(request_id, answer, message);

    assert_eq!(result, RevealMinerResult::Success);

    let logs = get_logs();
    assert_eq!(logs.len(), 0);
}

#[test]
fn test_reveal_by_miner_when_miner_is_not_registered() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

    contract.commit_by_miner(request_id.clone(), answer);

    let context = get_context("edson.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = true;
    let message = "It's a cool NFT".to_string();

    let result = contract.reveal_by_miner(request_id, answer, message);

    assert_eq!(result, RevealMinerResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);

    assert_eq!(logs[0], "Miner not registered: edson.near");
}

#[test]
fn test_reveal_by_miner_when_request_is_not_registered() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

    contract.commit_by_miner(request_id.clone(), answer);
    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725".to_string();

    let answer = true;
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_miner(request_id, answer, message);

    assert_eq!(result, RevealMinerResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 4);

    assert_eq!(logs[0], "Registered new miner: hassel.near");
    assert_eq!(
        logs[1],
        "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
    );
    assert_eq!(logs[2], "Miner proposal registered successfully");
    assert_eq!(logs[3], "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725");
}

#[test]
fn test_reveal_by_miner_when_proposal_is_already_reveal() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

    contract.commit_by_miner(request_id.clone(), answer);

    let logs = get_logs();
    assert_eq!(logs.len(), 3);

    assert_eq!(logs[0], "Registered new miner: hassel.near");
    assert_eq!(
        logs[1],
        "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
    );
    assert_eq!(logs[2], "Miner proposal registered successfully");

    let answer = true;
    let message = "It's a cool NFT".to_string();

    let reveal_miner_time = 100000000 + (3 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), reveal_miner_time);
    testing_env!(context.build());

    contract.reveal_by_miner(request_id.clone(), answer, message.clone());

    let result = contract.reveal_by_miner(request_id, answer, message);

    assert_eq!(result, RevealMinerResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);

    assert_eq!(logs[0], "Proposal already revealed");
}

#[test]
fn test_reveal_by_miner_when_answer_not_equal() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

    contract.commit_by_miner(request_id.clone(), answer);
    let logs = get_logs();
    assert_eq!(logs.len(), 3);

    assert_eq!(logs[0], "Registered new miner: hassel.near");
    assert_eq!(
        logs[1],
        "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
    );
    assert_eq!(logs[2], "Miner proposal registered successfully");

    let answer = false;
    let message = "It's a cool NFT".to_string();

    let reveal_miner_time = 100000000 + (3 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), reveal_miner_time);
    testing_env!(context.build());
    let result = contract.reveal_by_miner(request_id, answer, message);

    assert_eq!(result, RevealMinerResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);

    assert_eq!(logs[0], "Answer don't match");
}

// Reveal by validator

#[test]
fn test_reveal_by_validator() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let logs = get_logs();
    assert_eq!(logs.len(), 2);

    assert_eq!(logs[0], "Registered new validator: hassel.near");
    assert_eq!(
        logs[1],
        "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
    );

    let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), commit_validator_time);
    testing_env!(context.build());

    contract.commit_by_validator(request_id.clone(), answer);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "Validator proposal registered successfully");

    let reveal_validator_time = 100000000 + (7 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), reveal_validator_time);
    testing_env!(context.build());

    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(request_id, answer, message);

    assert_eq!(result, RevealValidatorResult::Success);
}

#[test]
fn test_reveal_by_validator_when_validator_is_not_registered() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), commit_validator_time);
    testing_env!(context.build());

    contract.commit_by_validator(request_id.clone(), answer.clone());

    let context = get_context("edson.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(request_id, answer, message);

    assert_eq!(result, RevealValidatorResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);

    assert_eq!(logs[0], "Validator is not registered: edson.near");
}

#[test]
fn test_reveal_by_validator_when_request_is_not_registered() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let logs = get_logs();
    assert_eq!(logs.len(), 2);

    assert_eq!(logs[0], "Registered new validator: hassel.near");
    assert_eq!(
        logs[1],
        "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
    );

    let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), commit_validator_time);
    testing_env!(context.build());

    contract.commit_by_validator(request_id.clone(), answer.clone());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725".to_string();
    let answer: Vec<AccountId> = generate_validator_answer();
    let message = "It's a cool NFT".to_string();

    let result = contract.reveal_by_validator(request_id, answer, message);

    assert_eq!(result, RevealValidatorResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 2);

    assert_eq!(logs[0], "Validator proposal registered successfully");
    assert_eq!(logs[1], "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725");
}

#[test]
fn test_reveal_by_validator_when_proposal_is_already_reveal() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let logs = get_logs();
    assert_eq!(logs.len(), 2);

    assert_eq!(logs[0], "Registered new validator: hassel.near");
    assert_eq!(
        logs[1],
        "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
    );

    let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), commit_validator_time);
    testing_env!(context.build());

    contract.commit_by_validator(request_id.clone(), answer.clone());

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "Validator proposal registered successfully");

    let reveal_validator_time = 100000000 + (7 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), reveal_validator_time);
    testing_env!(context.build());

    let answer: Vec<AccountId> = generate_validator_answer();
    let message = "It's a cool NFT".to_string();

    contract.reveal_by_validator(request_id.clone(), answer.clone(), message.clone());

    let result = contract.reveal_by_validator(request_id, answer, message);

    assert_eq!(result, RevealValidatorResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "Proposal already revealed");
}

#[test]
fn test_reveal_by_validator_when_answer_not_equal() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000);
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let logs = get_logs();
    assert_eq!(logs.len(), 2);

    assert_eq!(logs[0], "Registered new validator: hassel.near");
    assert_eq!(
        logs[1],
        "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
    );

    let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), commit_validator_time);
    testing_env!(context.build());

    contract.commit_by_validator(request_id.clone(), answer.clone());

    let mut answer: Vec<AccountId> = generate_validator_answer();
    answer[9] = "jane.near".parse().unwrap();

    let message = "It's a cool NFT".to_string();

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "Validator proposal registered successfully");

    let reveal_validator_time = 100000000 + (7 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), reveal_validator_time);
    testing_env!(context.build());
    let result = contract.reveal_by_validator(request_id.clone(), answer.clone(), message.clone());

    assert_eq!(result, RevealValidatorResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "Answer don't match");
}

use earthmind_rs::Contract;
use earthmind_rs::{
    CommitMinerResult, CommitValidatorResult, RegisterMinerResult, RegisterRequestResult, RegisterValidatorResult, RevealMinerResult, RevealValidatorResult,
};

use near_sdk::{
    test_utils::{get_logs, VMContextBuilder},
    testing_env, AccountId, NearToken,
};

// 1 Near = NearToken::from_yoctonear(10u128.pow(24)).as_near()

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
    value
}

fn get_context(predecessor_account_id: AccountId, block_timestamp: u64, attached_deposit: NearToken) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .predecessor_account_id(predecessor_account_id)
        .block_timestamp(block_timestamp)
        .attached_deposit(attached_deposit);
    builder
}

// Register Miner

#[test]
fn test_register_miner() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    let result_1 = contract.register_miner();
    assert_eq!(result_1, RegisterMinerResult::Success);

    let miner_1: AccountId = "hassel.near".parse().unwrap();
    assert!(contract.is_miner_registered(miner_1));

    let logs = get_logs();
    assert_eq!(logs.len(), 1);

    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_miner","data":[{"miner":"hassel.near"}]}"#
    );

    let context = get_context("edson.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());

    let result_2 = contract.register_miner();
    assert_eq!(result_2, RegisterMinerResult::Success);

    let miner_2: AccountId = "edson.near".parse().unwrap();
    assert!(contract.is_miner_registered(miner_2));

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_miner","data":[{"miner":"edson.near"}]}"#
    );
}

#[test]
fn test_register_miner_when_is_registered_returns_already_registered() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();
    let result = contract.register_miner();

    assert_eq!(result, RegisterMinerResult::AlreadyRegistered);

    let logs = get_logs();

    assert_eq!(logs.len(), 2);

    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_miner","data":[{"miner":"hassel.near"}]}"#
    );
    assert_eq!(logs[1], "Attempted to register an already registered miner: hassel.near");
}

#[test]
#[should_panic]
fn test_register_miner_when_deposit_is_less_min_stake() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(23)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();
}

#[test]
fn test_is_miner_registered() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();

    let miner: AccountId = "hassel.near".parse().unwrap();
    assert!(contract.is_miner_registered(miner));
}

#[test]
fn test_is_miner_registered_when_not_registered() {
    let contract = Contract::new();

    let miner: AccountId = "hassel.near".parse().unwrap();

    assert!(!contract.is_miner_registered(miner));
}

// Register Validator

#[test]
fn test_register_validator() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    let result_1 = contract.register_validator();
    assert_eq!(result_1, RegisterValidatorResult::Success);

    let validator_1: AccountId = "hassel.near".parse().unwrap();
    assert!(contract.is_validator_registered(validator_1));

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"hassel.near"}]}"#
    );

    let context = get_context("edson.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    let result2 = contract.register_validator();
    assert_eq!(result2, RegisterValidatorResult::Success);

    let validator_2: AccountId = "edson.near".parse().unwrap();
    assert!(contract.is_validator_registered(validator_2));

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"edson.near"}]}"#
    );
}

#[test]
fn test_register_validator_when_is_registered_returns_already_registered() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let result = contract.register_validator();

    assert_eq!(result, RegisterValidatorResult::AlreadyRegistered);

    let logs = get_logs();

    assert_eq!(logs.len(), 2);

    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"hassel.near"}]}"#
    );
    assert_eq!(logs[1], "Attempted to register an already registered validator: hassel.near");
}

#[test]
#[should_panic]
fn test_register_validator_when_deposit_is_less_min_stake() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(23)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();
}

#[test]
fn test_is_validator_registered() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    let mut contract = Contract::new();
    contract.register_validator();

    let validator: AccountId = "hassel.near".parse().unwrap();
    assert!(contract.is_validator_registered(validator));
}

#[test]
fn test_is_validator_registered_when_not_registered() {
    let contract = Contract::new();
    let validator: AccountId = "hassel.near".parse().unwrap();

    assert!(!contract.is_validator_registered(validator));
}

// Request Governance Decision

#[test]
fn test_request_governance_decision_when_is_registered_returns_already_registered() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
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
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
    );
    assert_eq!(
        logs[1],
        "Attempted to register an already registered request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
    );
}

// Hash miner answer

#[test]
fn test_hash_miner_answer() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = true;
    let message = "It's a cool NFT".to_string();

    let result = contract.hash_miner_answer(request_id, answer, message);

    assert_eq!(result, "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464");
}

// Hash validator answer
#[test]
fn test_hash_validator_answer() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();

    let result = contract.hash_validator_answer(request_id, answer, message);

    assert_eq!(result, "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856");
}

#[test]
#[should_panic]
fn test_hash_validator_answer_when_answer_is_not_complete() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = generate_validator_answer();
    let answer: Vec<AccountId> = answer[0..answer.len() - 1].to_vec();
    let message = "It's a cool NFT".to_string();

    contract.hash_validator_answer(request_id, answer, message);
}

// Commit by miner

#[test]
fn test_commit_by_miner_when_miner_and_request_exist() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
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

    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_miner","data":[{"miner":"hassel.near"}]}"#
    );

    assert_eq!(
        logs[1],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
    );
    assert_eq!(
        logs[2],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_miner","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464"}]}"#
    );
}

#[test]
fn test_commit_by_miner_when_miner_dont_registered_and_request_exist() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let context = get_context("edson.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
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
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

    let result = contract.commit_by_miner(request_id, answer);

    assert_eq!(result, CommitMinerResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 2);
    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_miner","data":[{"miner":"hassel.near"}]}"#
    );
    assert_eq!(
        logs[1],
        "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
    );
}

#[test]
fn test_commit_by_miner_when_miner_and_request_exist_and_commit_already() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();
    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

    contract.commit_by_miner(request_id.clone(), answer.clone());

    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
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
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let logs = get_logs();
    assert_eq!(logs.len(), 2);
    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"hassel.near"}]}"#
    );
    assert_eq!(
        logs[1],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
    );

    let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), commit_validator_time, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    let result = contract.commit_by_validator(request_id, answer);

    assert_eq!(result, CommitValidatorResult::Success);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_validator","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856"}]}"#
    );
}

#[test]
fn test_commit_by_validator_when_validator_dont_registered_and_request_exist() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let context = get_context("edson.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
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
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let result = contract.commit_by_validator(request_id, answer);

    assert_eq!(result, CommitValidatorResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 2);
    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"hassel.near"}]}"#
    );
    assert_eq!(
        logs[1],
        "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
    )
}

#[test]
fn test_commit_by_validator_when_miner_and_request_exist_and_commit_already() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), commit_validator_time, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    contract.commit_by_validator(request_id.clone(), answer.clone());

    let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000) + 1;
    let context = get_context("hassel.near".parse().unwrap(), commit_validator_time, NearToken::from_yoctonear(10u128.pow(24)));
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
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

    contract.commit_by_miner(request_id, answer);

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = true;
    let message = "It's a cool NFT".to_string();

    let logs = get_logs();
    assert_eq!(logs.len(), 3);

    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_miner","data":[{"miner":"hassel.near"}]}"#
    );
    assert_eq!(
        logs[1],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
    );
    assert_eq!(
        logs[2],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_miner","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464"}]}"#
    );

    let reveal_miner_time = 100000000 + (3 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), reveal_miner_time, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());
    let result = contract.reveal_by_miner(request_id, answer, message);

    assert_eq!(result, RevealMinerResult::Success);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"reveal_miner","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":true,"message":"It's a cool NFT"}]}"#
    );
}

#[test]
fn test_reveal_by_miner_when_miner_is_not_registered() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

    contract.commit_by_miner(request_id, answer);

    let context = get_context("edson.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
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
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_miner();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

    contract.commit_by_miner(request_id, answer);
    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725".to_string();

    let answer = true;
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_miner(request_id, answer, message);

    assert_eq!(result, RevealMinerResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 4);

    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_miner","data":[{"miner":"hassel.near"}]}"#
    );
    assert_eq!(
        logs[1],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
    );
    assert_eq!(
        logs[2],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_miner","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464"}]}"#
    );
    assert_eq!(
        logs[3],
        "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725"
    );
}

#[test]
fn test_reveal_by_miner_when_proposal_is_already_reveal() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
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

    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_miner","data":[{"miner":"hassel.near"}]}"#
    );

    assert_eq!(
        logs[1],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
    );
    assert_eq!(
        logs[2],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_miner","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464"}]}"#
    );

    let answer = true;
    let message = "It's a cool NFT".to_string();

    let reveal_miner_time = 100000000 + (3 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), reveal_miner_time, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());

    contract.reveal_by_miner(request_id.clone(), answer, message.clone());

    let result = contract.reveal_by_miner(request_id, answer, message);

    assert_eq!(result, RevealMinerResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 2);

    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"reveal_miner","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":true,"message":"It's a cool NFT"}]}"#
    );
    assert_eq!(logs[1], "Proposal already revealed");
}

#[test]
fn test_reveal_by_miner_when_answer_not_equal() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
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

    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_miner","data":[{"miner":"hassel.near"}]}"#
    );
    assert_eq!(
        logs[1],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
    );
    assert_eq!(
        logs[2],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_miner","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464"}]}"#
    );

    let answer = false;
    let message = "It's a cool NFT".to_string();

    let reveal_miner_time = 100000000 + (3 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), reveal_miner_time, NearToken::from_yoctonear(10u128.pow(24)));
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
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let logs = get_logs();
    assert_eq!(logs.len(), 2);

    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"hassel.near"}]}"#
    );
    assert_eq!(
        logs[1],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
    );

    let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), commit_validator_time, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    contract.commit_by_validator(request_id.clone(), answer);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_validator","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856"}]}"#
    );
    let reveal_validator_time = 100000000 + (7 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), reveal_validator_time, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(request_id, answer, message);

    assert_eq!(result, RevealValidatorResult::Success);
}

#[test]
fn test_reveal_by_validator_when_validator_is_not_registered() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), commit_validator_time, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    contract.commit_by_validator(request_id.clone(), answer);

    let context = get_context("edson.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
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
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let logs = get_logs();
    assert_eq!(logs.len(), 2);

    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"hassel.near"}]}"#
    );

    assert_eq!(
        logs[1],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
    );

    let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), commit_validator_time, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    contract.commit_by_validator(request_id, answer);

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725".to_string();
    let answer: Vec<AccountId> = generate_validator_answer();
    let message = "It's a cool NFT".to_string();

    let result = contract.reveal_by_validator(request_id, answer, message);

    assert_eq!(result, RevealValidatorResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 2);

    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_validator","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856"}]}"#
    );
    assert_eq!(
        logs[1],
        "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725"
    );
}

#[test]
fn test_reveal_by_validator_when_proposal_is_already_reveal() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let logs = get_logs();
    assert_eq!(logs.len(), 2);

    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"hassel.near"}]}"#
    );
    assert_eq!(
        logs[1],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
    );

    let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), commit_validator_time, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());

    contract.commit_by_validator(request_id.clone(), answer);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_validator","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856"}]}"#
    );

    let reveal_validator_time = 100000000 + (7 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), reveal_validator_time, NearToken::from_yoctonear(10u128.pow(24)));
    testing_env!(context.build());

    let answer: Vec<AccountId> = generate_validator_answer();
    let message = "It's a cool NFT".to_string();

    contract.reveal_by_validator(request_id.clone(), answer.clone(), message.clone());

    let result = contract.reveal_by_validator(request_id, answer, message);

    assert_eq!(result, RevealValidatorResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 2);
    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"reveal_validator","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":["hassel.near","edson.near","anne.near","bob.near","alice.near","john.near","harry.near","scott.near","felix.near","margaret.near"],"message":"It's a cool NFT"}]}"#
    );
    assert_eq!(logs[1], "Proposal already revealed");
}

#[test]
fn test_reveal_by_validator_when_answer_not_equal() {
    let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    let mut contract = Contract::new();

    contract.register_validator();

    let message = "Should we add this new NFT to our protocol?";
    contract.request_governance_decision(message.to_string());

    let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
    let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

    let logs = get_logs();
    assert_eq!(logs.len(), 2);

    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"hassel.near"}]}"#
    );
    assert_eq!(
        logs[1],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
    );

    let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), commit_validator_time, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());

    contract.commit_by_validator(request_id.clone(), answer);

    let mut answer: Vec<AccountId> = generate_validator_answer();
    answer[9] = "jane.near".parse().unwrap();

    let message = "It's a cool NFT".to_string();

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(
        logs[0],
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_validator","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856"}]}"#
    );

    let reveal_validator_time = 100000000 + (7 * 60 * 1_000_000_000);
    let context = get_context("hassel.near".parse().unwrap(), reveal_validator_time, NearToken::from_yoctonear(10u128.pow(25)));
    testing_env!(context.build());
    let result = contract.reveal_by_validator(request_id, answer.clone(), message);

    assert_eq!(result, RevealValidatorResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "Answer don't match");
}

// use near_sdk::test_utils::get_logs;
// use serde_json::json;

// use common::constants::{DEFAULT_MESSAGE_TO_REQUEST, DEFAULT_MINER_ANSWER, DEFAULT_REQUEST_ID, MINER_1, MINER_2};
// use common::environment::Environment;
// use common::types::Log;
// use common::utils::{assert_logs, get_account_for_miner, get_default_miner_account};

// use earthmind_rs::{Contract, CommitValidatorResult};

// pub mod common;

// #[test]
// fn test_reveal_by_validator() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     contract.register_validator();

//     let message = "Should we add this new NFT to our protocol?";
//     contract.request_governance_decision(message.to_string());

//     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
//     let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

//     let logs = get_logs();
//     assert_eq!(logs.len(), 2);

//     assert_eq!(
//         logs[0],
//         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"hassel.near"}]}"#
//     );
//     assert_eq!(
//         logs[1],
//         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
//     );

//     let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
//     let context = get_context("hassel.near".parse().unwrap(), commit_validator_time, NearToken::from_yoctonear(10u128.pow(25)));
//     testing_env!(context.build());

//     contract.commit_by_validator(request_id.clone(), answer);

//     let logs = get_logs();
//     assert_eq!(logs.len(), 1);
//     assert_eq!(
//         logs[0],
//         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_validator","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856"}]}"#
//     );
//     let reveal_validator_time = 100000000 + (7 * 60 * 1_000_000_000);
//     let context = get_context("hassel.near".parse().unwrap(), reveal_validator_time, NearToken::from_yoctonear(10u128.pow(25)));
//     testing_env!(context.build());

//     let answer = generate_validator_answer();
//     let message = "It's a cool NFT".to_string();
//     let result = contract.reveal_by_validator(request_id, answer, message);

//     assert_eq!(result, RevealValidatorResult::Success);
// }

// // #[test]
// // fn test_reveal_by_validator_when_validator_is_not_registered() {
// //     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
// //     testing_env!(context.build());

// //     let mut contract = Contract::new();

// //     contract.register_validator();

// //     let message = "Should we add this new NFT to our protocol?";
// //     contract.request_governance_decision(message.to_string());

// //     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
// //     let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

// //     let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
// //     let context = get_context("hassel.near".parse().unwrap(), commit_validator_time, NearToken::from_yoctonear(10u128.pow(25)));
// //     testing_env!(context.build());

// //     contract.commit_by_validator(request_id.clone(), answer);

// //     let context = get_context("edson.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
// //     testing_env!(context.build());

// //     let answer = generate_validator_answer();
// //     let message = "It's a cool NFT".to_string();
// //     let result = contract.reveal_by_validator(request_id, answer, message);

// //     assert_eq!(result, RevealValidatorResult::Fail);

// //     let logs = get_logs();
// //     assert_eq!(logs.len(), 1);

// //     assert_eq!(logs[0], "Validator is not registered: edson.near");
// // }

// // #[test]
// // fn test_reveal_by_validator_when_request_is_not_registered() {
// //     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
// //     testing_env!(context.build());

// //     let mut contract = Contract::new();

// //     contract.register_validator();

// //     let message = "Should we add this new NFT to our protocol?";
// //     contract.request_governance_decision(message.to_string());

// //     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
// //     let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

// //     let logs = get_logs();
// //     assert_eq!(logs.len(), 2);

// //     assert_eq!(
// //         logs[0],
// //         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"hassel.near"}]}"#
// //     );

// //     assert_eq!(
// //         logs[1],
// //         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
// //     );

// //     let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
// //     let context = get_context("hassel.near".parse().unwrap(), commit_validator_time, NearToken::from_yoctonear(10u128.pow(25)));
// //     testing_env!(context.build());

// //     contract.commit_by_validator(request_id, answer);

// //     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725".to_string();
// //     let answer: Vec<AccountId> = generate_validator_answer();
// //     let message = "It's a cool NFT".to_string();

// //     let result = contract.reveal_by_validator(request_id, answer, message);

// //     assert_eq!(result, RevealValidatorResult::Fail);

// //     let logs = get_logs();
// //     assert_eq!(logs.len(), 2);

// //     assert_eq!(
// //         logs[0],
// //         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_validator","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856"}]}"#
// //     );
// //     assert_eq!(
// //         logs[1],
// //         "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725"
// //     );
// // }

// // #[test]
// // fn test_reveal_by_validator_when_proposal_is_already_reveal() {
// //     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
// //     testing_env!(context.build());

// //     let mut contract = Contract::new();

// //     contract.register_validator();

// //     let message = "Should we add this new NFT to our protocol?";
// //     contract.request_governance_decision(message.to_string());

// //     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
// //     let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

// //     let logs = get_logs();
// //     assert_eq!(logs.len(), 2);

// //     assert_eq!(
// //         logs[0],
// //         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"hassel.near"}]}"#
// //     );
// //     assert_eq!(
// //         logs[1],
// //         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
// //     );

// //     let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
// //     let context = get_context("hassel.near".parse().unwrap(), commit_validator_time, NearToken::from_yoctonear(10u128.pow(24)));
// //     testing_env!(context.build());

// //     contract.commit_by_validator(request_id.clone(), answer);

// //     let logs = get_logs();
// //     assert_eq!(logs.len(), 1);
// //     assert_eq!(
// //         logs[0],
// //         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_validator","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856"}]}"#
// //     );

// //     let reveal_validator_time = 100000000 + (7 * 60 * 1_000_000_000);
// //     let context = get_context("hassel.near".parse().unwrap(), reveal_validator_time, NearToken::from_yoctonear(10u128.pow(24)));
// //     testing_env!(context.build());

// //     let answer: Vec<AccountId> = generate_validator_answer();
// //     let message = "It's a cool NFT".to_string();

// //     contract.reveal_by_validator(request_id.clone(), answer.clone(), message.clone());

// //     let result = contract.reveal_by_validator(request_id, answer, message);

// //     assert_eq!(result, RevealValidatorResult::Fail);

// //     let logs = get_logs();
// //     assert_eq!(logs.len(), 2);
// //     assert_eq!(
// //         logs[0],
// //         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"reveal_validator","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":["hassel.near","edson.near","anne.near","bob.near","alice.near","john.near","harry.near","scott.near","felix.near","margaret.near"],"message":"It's a cool NFT"}]}"#
// //     );
// //     assert_eq!(logs[1], "Proposal already revealed");
// // }

// // #[test]
// // fn test_reveal_by_validator_when_answer_not_equal() {
// //     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
// //     testing_env!(context.build());

// //     let mut contract = Contract::new();

// //     contract.register_validator();

// //     let message = "Should we add this new NFT to our protocol?";
// //     contract.request_governance_decision(message.to_string());

// //     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
// //     let answer = "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856".to_string();

// //     let logs = get_logs();
// //     assert_eq!(logs.len(), 2);

// //     assert_eq!(
// //         logs[0],
// //         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"hassel.near"}]}"#
// //     );
// //     assert_eq!(
// //         logs[1],
// //         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
// //     );

// //     let commit_validator_time = 100000000 + (5 * 60 * 1_000_000_000);
// //     let context = get_context("hassel.near".parse().unwrap(), commit_validator_time, NearToken::from_yoctonear(10u128.pow(25)));
// //     testing_env!(context.build());

// //     contract.commit_by_validator(request_id.clone(), answer);

// //     let mut answer: Vec<AccountId> = generate_validator_answer();
// //     answer[9] = "jane.near".parse().unwrap();

// //     let message = "It's a cool NFT".to_string();

// //     let logs = get_logs();
// //     assert_eq!(logs.len(), 1);
// //     assert_eq!(
// //         logs[0],
// //         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_validator","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856"}]}"#
// //     );

// //     let reveal_validator_time = 100000000 + (7 * 60 * 1_000_000_000);
// //     let context = get_context("hassel.near".parse().unwrap(), reveal_validator_time, NearToken::from_yoctonear(10u128.pow(25)));
// //     testing_env!(context.build());
// //     let result = contract.reveal_by_validator(request_id, answer.clone(), message);

// //     assert_eq!(result, RevealValidatorResult::Fail);

// //     let logs = get_logs();
// //     assert_eq!(logs.len(), 1);
// //     assert_eq!(logs[0], "Answer don't match");
// // }

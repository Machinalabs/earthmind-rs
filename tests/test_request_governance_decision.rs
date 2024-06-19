// use crate::common::utils::{generate_validator_answer, get_context};
// use earthmind_rs::Contract;
// use earthmind_rs::{
//     CommitMinerResult, CommitValidatorResult, RegisterMinerResult, RegisterRequestResult, RegisterValidatorResult, RevealMinerResult, RevealValidatorResult,
// };
// use near_sdk::{test_utils::get_logs, testing_env, AccountId, NearToken};

// #[test]
// fn test_request_governance_decision_when_is_registered_returns_already_registered() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     let message = "Should we add this new NFT to our protocol?";
//     contract.request_governance_decision(message.to_string());

//     let result = contract.request_governance_decision(message.to_string());

//     assert_eq!(result, RegisterRequestResult::AlreadyRegistered);

//     let logs = get_logs();
//     assert_eq!(logs.len(), 2);

//     assert_eq!(
//         logs[0],
//         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
//     );
//     assert_eq!(
//         logs[1],
//         "Attempted to register an already registered request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
//     );
// }

// // Hash miner answer

// #[test]
// fn test_hash_miner_answer() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     let message = "Should we add this new NFT to our protocol?";
//     contract.request_governance_decision(message.to_string());

//     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
//     let answer = true;
//     let message = "It's a cool NFT".to_string();

//     let result = contract.hash_miner_answer(request_id, answer, message);

//     assert_eq!(result, "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464");
// }

// // Hash validator answer
// #[test]
// fn test_hash_validator_answer() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     let message = "Should we add this new NFT to our protocol?";
//     contract.request_governance_decision(message.to_string());

//     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
//     let answer = generate_validator_answer();
//     let message = "It's a cool NFT".to_string();

//     let result = contract.hash_validator_answer(request_id, answer, message);

//     assert_eq!(result, "cbc707592325bc03fead86ad6207eabb58a0657fa235f72dc500d5f1965ba856");
// }

// #[test]
// #[should_panic]
// fn test_hash_validator_answer_when_answer_is_not_complete() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     let message = "Should we add this new NFT to our protocol?";
//     contract.request_governance_decision(message.to_string());

//     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
//     let answer = generate_validator_answer();
//     let answer: Vec<AccountId> = answer[0..answer.len() - 1].to_vec();
//     let message = "It's a cool NFT".to_string();

//     contract.hash_validator_answer(request_id, answer, message);
// }

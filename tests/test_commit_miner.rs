// use crate::common::utils::{generate_validator_answer, get_context};
// use earthmind_rs::Contract;
// use earthmind_rs::{
//     CommitMinerResult, CommitValidatorResult, RegisterMinerResult, RegisterRequestResult, RegisterValidatorResult, RevealMinerResult, RevealValidatorResult,
// };
// use near_sdk::{test_utils::get_logs, testing_env, AccountId, NearToken};

// #[test]
// fn test_commit_by_miner_when_miner_and_request_exist() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     contract.register_miner();

//     let message = "Should we add this new NFT to our protocol?";
//     contract.request_governance_decision(message.to_string());

//     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
//     let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

//     let result = contract.commit_by_miner(request_id, answer);

//     assert_eq!(result, CommitMinerResult::Success);

//     let logs = get_logs();
//     assert_eq!(logs.len(), 3);

//     assert_eq!(
//         logs[0],
//         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_miner","data":[{"miner":"hassel.near"}]}"#
//     );

//     assert_eq!(
//         logs[1],
//         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
//     );
//     assert_eq!(
//         logs[2],
//         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_miner","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464"}]}"#
//     );
// }

// #[test]
// fn test_commit_by_miner_when_miner_dont_registered_and_request_exist() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     contract.register_miner();

//     let message = "Should we add this new NFT to our protocol?";
//     contract.request_governance_decision(message.to_string());

//     let context = get_context("edson.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
//     let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

//     let result = contract.commit_by_miner(request_id, answer);

//     assert_eq!(result, CommitMinerResult::Fail);

//     let logs = get_logs();
//     assert_eq!(logs.len(), 1);
//     assert_eq!(logs[0], "Miner not registered: edson.near");
// }

// #[test]
// fn test_commit_by_miner_when_miner_registered_and_request_dont_exist() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     contract.register_miner();

//     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
//     let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

//     let result = contract.commit_by_miner(request_id, answer);

//     assert_eq!(result, CommitMinerResult::Fail);

//     let logs = get_logs();
//     assert_eq!(logs.len(), 2);
//     assert_eq!(
//         logs[0],
//         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_miner","data":[{"miner":"hassel.near"}]}"#
//     );
//     assert_eq!(
//         logs[1],
//         "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
//     );
// }

// #[test]
// fn test_commit_by_miner_when_miner_and_request_exist_and_commit_already() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     contract.register_miner();
//     let message = "Should we add this new NFT to our protocol?";
//     contract.request_governance_decision(message.to_string());

//     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
//     let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

//     contract.commit_by_miner(request_id.clone(), answer.clone());

//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let result = contract.commit_by_miner(request_id, answer);

//     assert_eq!(result, CommitMinerResult::Fail);

//     let logs = get_logs();
//     assert_eq!(logs.len(), 1);
//     assert_eq!(logs[0], "This miner have a commit answer: hassel.near");
// }

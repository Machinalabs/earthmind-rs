// #[test]
// fn test_reveal_by_miner() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     contract.register_miner();

//     let message = "Should we add this new NFT to our protocol?";
//     contract.request_governance_decision(message.to_string());

//     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
//     let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

//     contract.commit_by_miner(request_id, answer);

//     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
//     let answer = true;
//     let message = "It's a cool NFT".to_string();

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

//     let reveal_miner_time = 100000000 + (3 * 60 * 1_000_000_000);
//     let context = get_context("hassel.near".parse().unwrap(), reveal_miner_time, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());
//     let result = contract.reveal_by_miner(request_id, answer, message);

//     assert_eq!(result, RevealMinerResult::Success);

//     let logs = get_logs();
//     assert_eq!(logs.len(), 1);
//     assert_eq!(
//         logs[0],
//         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"reveal_miner","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":true,"message":"It's a cool NFT"}]}"#
//     );
// }

// #[test]
// fn test_reveal_by_miner_when_miner_is_not_registered() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     contract.register_miner();

//     let message = "Should we add this new NFT to our protocol?";
//     contract.request_governance_decision(message.to_string());

//     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
//     let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

//     contract.commit_by_miner(request_id, answer);

//     let context = get_context("edson.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
//     let answer = true;
//     let message = "It's a cool NFT".to_string();

//     let result = contract.reveal_by_miner(request_id, answer, message);

//     assert_eq!(result, RevealMinerResult::Fail);

//     let logs = get_logs();
//     assert_eq!(logs.len(), 1);

//     assert_eq!(logs[0], "Miner not registered: edson.near");
// }

// #[test]
// fn test_reveal_by_miner_when_request_is_not_registered() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     contract.register_miner();

//     let message = "Should we add this new NFT to our protocol?";
//     contract.request_governance_decision(message.to_string());

//     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
//     let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

//     contract.commit_by_miner(request_id, answer);
//     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725".to_string();

//     let answer = true;
//     let message = "It's a cool NFT".to_string();
//     let result = contract.reveal_by_miner(request_id, answer, message);

//     assert_eq!(result, RevealMinerResult::Fail);

//     let logs = get_logs();
//     assert_eq!(logs.len(), 4);

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
//     assert_eq!(
//         logs[3],
//         "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725"
//     );
// }

// #[test]
// fn test_reveal_by_miner_when_proposal_is_already_reveal() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     contract.register_miner();

//     let message = "Should we add this new NFT to our protocol?";
//     contract.request_governance_decision(message.to_string());

//     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
//     let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

//     contract.commit_by_miner(request_id.clone(), answer);

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

//     let answer = true;
//     let message = "It's a cool NFT".to_string();

//     let reveal_miner_time = 100000000 + (3 * 60 * 1_000_000_000);
//     let context = get_context("hassel.near".parse().unwrap(), reveal_miner_time, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     contract.reveal_by_miner(request_id.clone(), answer, message.clone());

//     let result = contract.reveal_by_miner(request_id, answer, message);

//     assert_eq!(result, RevealMinerResult::Fail);

//     let logs = get_logs();
//     assert_eq!(logs.len(), 2);

//     assert_eq!(
//         logs[0],
//         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"reveal_miner","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":true,"message":"It's a cool NFT"}]}"#
//     );
//     assert_eq!(logs[1], "Proposal already revealed");
// }

// #[test]
// fn test_reveal_by_miner_when_answer_not_equal() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     contract.register_miner();

//     let message = "Should we add this new NFT to our protocol?";
//     contract.request_governance_decision(message.to_string());

//     let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
//     let answer = "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string();

//     contract.commit_by_miner(request_id.clone(), answer);
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

//     let answer = false;
//     let message = "It's a cool NFT".to_string();

//     let reveal_miner_time = 100000000 + (3 * 60 * 1_000_000_000);
//     let context = get_context("hassel.near".parse().unwrap(), reveal_miner_time, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());
//     let result = contract.reveal_by_miner(request_id, answer, message);

//     assert_eq!(result, RevealMinerResult::Fail);

//     let logs = get_logs();
//     assert_eq!(logs.len(), 1);

//     assert_eq!(logs[0], "Answer don't match");
// }

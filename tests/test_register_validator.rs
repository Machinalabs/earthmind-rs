// use crate::common::utils::{generate_validator_answer, get_context};
// use earthmind_rs::Contract;
// use earthmind_rs::{
//     CommitMinerResult, CommitValidatorResult, RegisterMinerResult, RegisterRequestResult, RegisterValidatorResult, RevealMinerResult, RevealValidatorResult,
// };
// use near_sdk::{test_utils::get_logs, testing_env, AccountId, NearToken};

// #[test]
// fn test_register_validator() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     let result_1 = contract.register_validator();
//     assert_eq!(result_1, RegisterValidatorResult::Success);

//     let validator_1: AccountId = "hassel.near".parse().unwrap();
//     assert!(contract.is_validator_registered(validator_1));

//     let logs = get_logs();
//     assert_eq!(logs.len(), 1);
//     assert_eq!(
//         logs[0],
//         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"hassel.near"}]}"#
//     );

//     let context = get_context("edson.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
//     testing_env!(context.build());

//     let result2 = contract.register_validator();
//     assert_eq!(result2, RegisterValidatorResult::Success);

//     let validator_2: AccountId = "edson.near".parse().unwrap();
//     assert!(contract.is_validator_registered(validator_2));

//     let logs = get_logs();
//     assert_eq!(logs.len(), 1);
//     assert_eq!(
//         logs[0],
//         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"edson.near"}]}"#
//     );
// }

// #[test]
// fn test_register_validator_when_is_registered_returns_already_registered() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     contract.register_validator();

//     let result = contract.register_validator();

//     assert_eq!(result, RegisterValidatorResult::AlreadyRegistered);

//     let logs = get_logs();

//     assert_eq!(logs.len(), 2);

//     assert_eq!(
//         logs[0],
//         r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"hassel.near"}]}"#
//     );
//     assert_eq!(logs[1], "Attempted to register an already registered validator: hassel.near");
// }

// #[test]
// #[should_panic]
// fn test_register_validator_when_deposit_is_less_min_stake() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(23)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     contract.register_validator();
// }

// #[test]
// fn test_is_validator_registered() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();
//     contract.register_validator();

//     let validator: AccountId = "hassel.near".parse().unwrap();
//     assert!(contract.is_validator_registered(validator));
// }

// #[test]
// fn test_is_validator_registered_when_not_registered() {
//     let contract = Contract::new();
//     let validator: AccountId = "hassel.near".parse().unwrap();

//     assert!(!contract.is_validator_registered(validator));
// }

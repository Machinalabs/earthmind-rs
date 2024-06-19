use crate::common::utils::{assert_log, get_default_miner_account, set_environment_with};
use common::constants::MINER_1;
use common::constants::MINER_2;
use common::types::Log;
use common::utils::assert_logs;
use common::utils::get_account_for_miner;
use earthmind_rs::Contract;
use earthmind_rs::RegisterMinerResult;

pub mod common;

#[test]
fn test_register_miner() {
    let miner_1 = get_default_miner_account();
    set_environment_with(miner_1.clone());

    let mut contract = Contract::new();

    let result_1 = contract.register_miner();

    assert_eq!(result_1, RegisterMinerResult::Success);
    assert_eq!(contract.is_miner_registered(miner_1), true);

    assert_log("register_miner", vec![("miner", MINER_1)]);
}

#[test]
fn test_register_multiple_miners() {
    // register miner 1
    let miner_1 = get_default_miner_account();
    set_environment_with(miner_1.clone());

    let mut contract = Contract::new();
    let result_1 = contract.register_miner();

    assert_eq!(result_1, RegisterMinerResult::Success);
    assert_eq!(contract.is_miner_registered(miner_1), true);

    assert_log("register_miner", vec![("miner", MINER_1)]);

    // register miner 2
    let miner_2: near_sdk::AccountId = get_account_for_miner(MINER_2);
    set_environment_with(miner_2.clone());

    let result_2 = contract.register_miner();

    assert_eq!(result_2, RegisterMinerResult::Success);
    assert_eq!(contract.is_miner_registered(miner_2), true);

    assert_log("register_miner", vec![("miner", MINER_2)]);
}

#[test]
fn test_register_miner_when_is_registered_returns_already_registered() {
    let miner_1 = get_default_miner_account();
    set_environment_with(miner_1.clone());

    let mut contract = Contract::new();

    contract.register_miner();

    let result = contract.register_miner();

    assert_eq!(result, RegisterMinerResult::AlreadyRegistered);

    assert_logs(vec![
        Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", MINER_1)],
        },
        Log::Message("Attempted to register an already registered miner: miner1.near".to_string()),
    ]);
}

// #[test]
// #[should_panic]
// fn test_register_miner_when_deposit_is_less_min_stake() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(23)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     contract.register_miner();
// }

// #[test]
// fn test_is_miner_registered() {
//     let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
//     testing_env!(context.build());

//     let mut contract = Contract::new();

//     contract.register_miner();

//     let miner: AccountId = "hassel.near".parse().unwrap();
//     assert!(contract.is_miner_registered(miner));
// }

// #[test]
// fn test_is_miner_registered_when_not_registered() {
//     let contract = Contract::new();

//     let miner: AccountId = "hassel.near".parse().unwrap();

//     assert!(!contract.is_miner_registered(miner));
// }

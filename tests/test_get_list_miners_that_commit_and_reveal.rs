use common::constants::{
    COMMIT_VALIDATOR_TIME, DEFAULT_CULTURE, DEFAULT_DEPOSIT_MINER, DEFAULT_DEPOSIT_PROTOCOL, DEFAULT_MESSAGE_TO_REQUEST, DEFAULT_MINER_ANSWER,
    DEFAULT_REQUEST_ID, MINER_1, MINER_2, REVEAL_MINER_TIME,
};
use common::environment::Environment;
use common::types::Log;
use common::utils::{assert_logs, get_account_for_miner, get_default_miner_account, get_default_protocol_account};

use earthmind_rs::{Contract, Module, RevealMinerResult};

use serde_json::json;

pub mod common;

#[test]
fn test_get_list_miners_that_commit_and_reveal_when_one_miner_commit_and_reveal() {
    let mut contract = Contract::new();

    // @dev Protocol register to earthmind protocol and request a governance decision
    let protocol = get_default_protocol_account();
    Environment::with_account(protocol.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_PROTOCOL)
        .create();

    let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
    contract.register_protocol(DEFAULT_CULTURE.to_string(), modules);
    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());
    assert_logs(vec![
        Log::Event {
            event_name: "register_protocol".to_string(),
            data: vec![("account", json![protocol])],
        },
        Log::Event {
            event_name: "register_request".to_string(),
            data: vec![
                ("request_id", json![DEFAULT_REQUEST_ID]),
                ("start_time", json![1000000000]),
                ("reveal_miner_time", json![30000000000_i64]),
                ("commit_miner_time", json![30000000000_i64]),
                ("reveal_validator_time", json![30000000000_i64]),
                ("commit_validator_time", json![30000000000_i64]),
            ],
        },
    ]);

    let miner = get_default_miner_account();

    Environment::with_account(miner.clone()).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();
    contract.register_miner();
    contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

    assert_logs(vec![
        Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![MINER_1])],
        },
        Log::Event {
            event_name: "commit_miner".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_MINER_ANSWER])],
        },
    ]);

    Environment::with_account(miner.clone()).with_block_timestamp(REVEAL_MINER_TIME).create();

    let answer = true;
    let message = "It's a cool NFT";

    let result = contract.reveal_by_miner(DEFAULT_REQUEST_ID.to_string(), answer, message.to_string());

    assert_eq!(result, RevealMinerResult::Success);

    assert_logs(vec![Log::Event {
        event_name: "reveal_miner".to_string(),
        data: vec![
            ("request_id", json![DEFAULT_REQUEST_ID]),
            ("answer", json![answer]),
            ("message", json![message]),
        ],
    }]);

    Environment::with_account(miner.clone()).with_block_timestamp(COMMIT_VALIDATOR_TIME).create();

    let result = contract.get_list_miners_that_commit_and_reveal(DEFAULT_REQUEST_ID.to_string());

    let expected = vec![miner];
    assert_eq!(result, expected);
}

#[test]
fn test_get_list_miners_that_commit_and_reveal_with_multiple_miner_commit_and_reveal() {
    let mut contract = Contract::new();

    // @dev Protocol register to earthmind protocol and request a governance decision
    let protocol = get_default_protocol_account();
    Environment::with_account(protocol.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_PROTOCOL)
        .create();

    let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
    contract.register_protocol(DEFAULT_CULTURE.to_string(), modules);
    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());
    assert_logs(vec![
        Log::Event {
            event_name: "register_protocol".to_string(),
            data: vec![("account", json![protocol])],
        },
        Log::Event {
            event_name: "register_request".to_string(),
            data: vec![
                ("request_id", json![DEFAULT_REQUEST_ID]),
                ("start_time", json![1000000000]),
                ("reveal_miner_time", json![30000000000_i64]),
                ("commit_miner_time", json![30000000000_i64]),
                ("reveal_validator_time", json![30000000000_i64]),
                ("commit_validator_time", json![30000000000_i64]),
            ],
        },
    ]);

    let miner = get_default_miner_account();
    let miner_2 = get_account_for_miner(MINER_2);

    Environment::with_account(miner.clone()).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();
    contract.register_miner();
    contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

    assert_logs(vec![
        Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![MINER_1])],
        },
        Log::Event {
            event_name: "commit_miner".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_MINER_ANSWER])],
        },
    ]);

    let answer_miner_2 = "c06a8aabd77066edbee09e50289c3cc1a3a57514bea9a9bcbb244559816ccf26";
    Environment::with_account(miner_2.clone()).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();
    contract.register_miner();
    contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), answer_miner_2.to_string());

    assert_logs(vec![
        Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![MINER_2])],
        },
        Log::Event {
            event_name: "commit_miner".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![answer_miner_2])],
        },
    ]);

    Environment::with_account(miner.clone()).with_block_timestamp(REVEAL_MINER_TIME).create();

    let answer = true;
    let message = "It's a cool NFT";

    let result = contract.reveal_by_miner(DEFAULT_REQUEST_ID.to_string(), answer, message.to_string());

    assert_eq!(result, RevealMinerResult::Success);

    assert_logs(vec![Log::Event {
        event_name: "reveal_miner".to_string(),
        data: vec![
            ("request_id", json![DEFAULT_REQUEST_ID]),
            ("answer", json![answer]),
            ("message", json![message]),
        ],
    }]);

    Environment::with_account(miner_2.clone()).with_block_timestamp(REVEAL_MINER_TIME).create();

    let answer = true;
    let message = "It's a cool NFT";

    let result = contract.reveal_by_miner(DEFAULT_REQUEST_ID.to_string(), answer, message.to_string());

    assert_eq!(result, RevealMinerResult::Success);

    assert_logs(vec![Log::Event {
        event_name: "reveal_miner".to_string(),
        data: vec![
            ("request_id", json![DEFAULT_REQUEST_ID]),
            ("answer", json![answer]),
            ("message", json![message]),
        ],
    }]);

    Environment::with_account(miner.clone()).with_block_timestamp(COMMIT_VALIDATOR_TIME).create();

    let result = contract.get_list_miners_that_commit_and_reveal(DEFAULT_REQUEST_ID.to_string());

    let expected = vec![miner, miner_2];
    assert_eq!(result, expected);
}

#[test]
fn test_get_list_miners_that_commit_and_reveal_when_a_miner_commit_but_not_reveal() {
    let mut contract = Contract::new();

    // @dev Protocol register to earthmind protocol and request a governance decision
    let protocol = get_default_protocol_account();
    Environment::with_account(protocol.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_PROTOCOL)
        .create();

    let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
    contract.register_protocol(DEFAULT_CULTURE.to_string(), modules);
    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());
    assert_logs(vec![
        Log::Event {
            event_name: "register_protocol".to_string(),
            data: vec![("account", json![protocol])],
        },
        Log::Event {
            event_name: "register_request".to_string(),
            data: vec![
                ("request_id", json![DEFAULT_REQUEST_ID]),
                ("start_time", json![1000000000]),
                ("reveal_miner_time", json![30000000000_i64]),
                ("commit_miner_time", json![30000000000_i64]),
                ("reveal_validator_time", json![30000000000_i64]),
                ("commit_validator_time", json![30000000000_i64]),
            ],
        },
    ]);

    let miner = get_default_miner_account();
    let miner_2 = get_account_for_miner(MINER_2);

    Environment::with_account(miner.clone()).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();
    contract.register_miner();
    contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

    assert_logs(vec![
        Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![MINER_1])],
        },
        Log::Event {
            event_name: "commit_miner".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_MINER_ANSWER])],
        },
    ]);

    let answer_miner_2 = "c06a8aabd77066edbee09e50289c3cc1a3a57514bea9a9bcbb244559816ccf26";
    Environment::with_account(miner_2).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();
    contract.register_miner();
    contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), answer_miner_2.to_string());

    assert_logs(vec![
        Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![MINER_2])],
        },
        Log::Event {
            event_name: "commit_miner".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![answer_miner_2])],
        },
    ]);

    Environment::with_account(miner.clone()).with_block_timestamp(REVEAL_MINER_TIME).create();

    let answer = true;
    let message = "It's a cool NFT";

    let result = contract.reveal_by_miner(DEFAULT_REQUEST_ID.to_string(), answer, message.to_string());

    assert_eq!(result, RevealMinerResult::Success);

    assert_logs(vec![Log::Event {
        event_name: "reveal_miner".to_string(),
        data: vec![
            ("request_id", json![DEFAULT_REQUEST_ID]),
            ("answer", json![answer]),
            ("message", json![message]),
        ],
    }]);

    Environment::with_account(miner.clone()).with_block_timestamp(COMMIT_VALIDATOR_TIME).create();

    let result = contract.get_list_miners_that_commit_and_reveal(DEFAULT_REQUEST_ID.to_string());

    let expected = vec![miner];
    assert_eq!(result, expected);
}

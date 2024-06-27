use near_sdk::NearToken;
use serde_json::json;

use common::constants::{
    COMMIT_VALIDATOR_TIME, DEFAULT_DEPOSIT, DEFAULT_MESSAGE_TO_REQUEST, DEFAULT_REQUEST_ID, DEFAULT_VALIDATOR_ANSWER, REVEAL_MINER_TIME, REVEAL_TOPTEN_TIME,
    REVEAL_VALIDATOR_TIME, VALIDATOR_1, VALIDATOR_2, VALIDATOR_3,
};
use common::environment::Environment;
use common::types::Log;
use common::utils::{
    assert_logs, default_miners_commit_answer, generate_validator_answer, get_account_for_validator, get_default_validator_account, group_registered_miners,
};

use earthmind_rs::{Contract, RevealMinerResult, RevealValidatorResult};

pub mod common;

#[test]
fn test_votes_for_miner_using_one_validator() {
    let mut contract = Contract::new();

    let validator = get_default_validator_account();
    let deposit = NearToken::from_near(15);

    Environment::with_account(validator.clone()).with_attached_deposit(deposit).create();
    contract.register_protocol();
    contract.register_validator();
    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());

    assert_logs(vec![
        Log::Event {
            event_name: "register_protocol".to_string(),
            data: vec![("account", json![VALIDATOR_1])],
        },
        Log::Event {
            event_name: "register_validator".to_string(),
            data: vec![("validator", json![VALIDATOR_1])],
        },
        Log::Event {
            event_name: "register_request".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID])],
        },
    ]);

    let registered_miners = group_registered_miners();
    let default_answer_miners = default_miners_commit_answer();

    for (index, miners) in registered_miners.clone().into_iter().enumerate() {
        Environment::with_account(miners.clone()).with_attached_deposit(DEFAULT_DEPOSIT).create();
        contract.register_protocol();
        contract.register_miner();
        contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), default_answer_miners[index].clone());

        assert_logs(vec![
            Log::Event {
                event_name: "register_protocol".to_string(),
                data: vec![("account", json![miners])],
            },
            Log::Event {
                event_name: "register_miner".to_string(),
                data: vec![("miner", json![miners])],
            },
            Log::Event {
                event_name: "commit_miner".to_string(),
                data: vec![
                    ("request_id", json![DEFAULT_REQUEST_ID]),
                    ("answer", json![default_answer_miners[index].clone()]),
                ],
            },
        ]);
    }

    for miners in registered_miners {
        Environment::with_account(miners.clone()).with_block_timestamp(REVEAL_MINER_TIME).create();
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
    }

    Environment::with_account(validator.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    Environment::with_account(validator).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer.clone(), message.clone());

    assert_eq!(result, RevealValidatorResult::Success);
    assert_logs(vec![Log::Event {
        event_name: "reveal_validator".to_string(),
        data: vec![
            ("request_id", json![DEFAULT_REQUEST_ID]),
            ("answer", json![answer]),
            ("message", json![message]),
        ],
    }]);

    contract.votes_for_miner(DEFAULT_REQUEST_ID.to_string(), "miner1.near".parse().unwrap());

    assert_logs(vec![
        Log::Event {
            event_name: "reveal_validator".to_string(),
            data: vec![
                ("request_id", json![DEFAULT_REQUEST_ID]),
                ("answer", json![answer]),
                ("message", json![message]),
            ],
        },
        Log::Message("miner1.near have 1 votes".to_string()),
    ]);
}

#[test]
fn test_vote_for_miners_with_multiple_validators() {
    let mut contract = Contract::new();
    let validator_1 = get_default_validator_account();
    let deposit = NearToken::from_yoctonear(10u128.pow(25));

    Environment::with_account(validator_1.clone()).with_attached_deposit(deposit).create();
    contract.register_protocol();
    contract.register_validator();

    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());

    assert_logs(vec![
        Log::Event {
            event_name: "register_protocol".to_string(),
            data: vec![("account", json![VALIDATOR_1])],
        },
        Log::Event {
            event_name: "register_validator".to_string(),
            data: vec![("validator", json![VALIDATOR_1])],
        },
        Log::Event {
            event_name: "register_request".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID])],
        },
    ]);

    let registered_miners = group_registered_miners();
    let default_answer_miners = default_miners_commit_answer();

    for (index, miners) in registered_miners.clone().into_iter().enumerate() {
        Environment::with_account(miners.clone()).with_attached_deposit(DEFAULT_DEPOSIT).create();
        contract.register_protocol();
        contract.register_miner();
        contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), default_answer_miners[index].clone());

        assert_logs(vec![
            Log::Event {
                event_name: "register_protocol".to_string(),
                data: vec![("account", json![miners])],
            },
            Log::Event {
                event_name: "register_miner".to_string(),
                data: vec![("miner", json![miners])],
            },
            Log::Event {
                event_name: "commit_miner".to_string(),
                data: vec![
                    ("request_id", json![DEFAULT_REQUEST_ID]),
                    ("answer", json![default_answer_miners[index].clone()]),
                ],
            },
        ]);
    }

    for miners in registered_miners {
        Environment::with_account(miners.clone()).with_block_timestamp(REVEAL_MINER_TIME).create();
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
    }
    //@dev First validator
    Environment::with_account(validator_1.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    Environment::with_account(validator_1).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer.clone(), message.clone());

    assert_eq!(result, RevealValidatorResult::Success);

    contract.votes_for_miner(DEFAULT_REQUEST_ID.to_string(), "miner1.near".parse().unwrap());

    assert_logs(vec![
        Log::Event {
            event_name: "reveal_validator".to_string(),
            data: vec![
                ("request_id", json![DEFAULT_REQUEST_ID]),
                ("answer", json![answer]),
                ("message", json![message]),
            ],
        },
        Log::Message("miner1.near have 1 votes".to_string()),
    ]);

    //@dev Second validator
    let validator_2 = get_account_for_validator(VALIDATOR_2);
    let deposit = NearToken::from_yoctonear(10u128.pow(25));

    Environment::with_account(validator_2.clone()).with_attached_deposit(deposit).create();
    contract.register_protocol();
    contract.register_validator();

    assert_logs(vec![Log::Event {
        event_name: "register_protocol".to_string(),
        data: vec![("account", json![VALIDATOR_2])],
    },Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_2])],
    }]);

    Environment::with_account(validator_2.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    let validator2_answer = "5f79db20f99cc564b54feb8cbfc08150032e31886c8dc03c93e093006feb1a06";
    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), validator2_answer.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![validator2_answer])],
    }]);

    Environment::with_account(validator_2).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer.clone(), message.clone());

    assert_eq!(result, RevealValidatorResult::Success);

    contract.votes_for_miner(DEFAULT_REQUEST_ID.to_string(), "miner1.near".parse().unwrap());

    assert_logs(vec![
        Log::Event {
            event_name: "reveal_validator".to_string(),
            data: vec![
                ("request_id", json![DEFAULT_REQUEST_ID]),
                ("answer", json![answer]),
                ("message", json![message]),
            ],
        },
        Log::Message("miner1.near have 2 votes".to_string()),
    ]);
}

#[test]
fn test_get_top_10_voters() {
    let mut contract = Contract::new();

    // @dev Register 3 validators
    let validator_1 = get_default_validator_account();
    let deposit = NearToken::from_yoctonear(10u128.pow(25));
    Environment::with_account(validator_1.clone()).with_attached_deposit(deposit).create();
    contract.register_protocol();
    contract.register_validator();

    assert_logs(vec![
        Log::Event {
            event_name: "register_protocol".to_string(),
            data: vec![("account", json![VALIDATOR_1])],
        }, 
        Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_1])],
    }]);

    let validator_2 = get_account_for_validator(VALIDATOR_2);
    let deposit = NearToken::from_yoctonear(10u128.pow(25));
    Environment::with_account(validator_2.clone()).with_attached_deposit(deposit).create();
    contract.register_protocol();
    contract.register_validator();

    assert_logs(vec![
        Log::Event {
            event_name: "register_protocol".to_string(),
            data: vec![("account", json![VALIDATOR_2])],
        },
        Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_2])],
    }
    ]);

    let validator_3 = get_account_for_validator(VALIDATOR_3);
    let deposit = NearToken::from_yoctonear(10u128.pow(25));
    Environment::with_account(validator_3.clone()).with_attached_deposit(deposit).create();
    contract.register_protocol();
    contract.register_validator();

    //@dev Validator 3 request a governance decision
    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());

    assert_logs(vec![
        Log::Event {
            event_name: "register_protocol".to_string(),
            data: vec![("account", json![VALIDATOR_3])],
        },
        Log::Event {
            event_name: "register_validator".to_string(),
            data: vec![("validator", json![VALIDATOR_3])],
        },
        Log::Event {
            event_name: "register_request".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID])],
        },
    ]);

    let registered_miners = group_registered_miners();
    let default_answer_miners = default_miners_commit_answer();

    for (index, miners) in registered_miners.clone().into_iter().enumerate() {
        Environment::with_account(miners.clone()).with_attached_deposit(DEFAULT_DEPOSIT).create();
        contract.register_protocol();
        contract.register_miner();
        contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), default_answer_miners[index].clone());

        assert_logs(vec![
            Log::Event {
                event_name: "register_protocol".to_string(),
                data: vec![("account", json![miners])],
            },
            Log::Event {
                event_name: "register_miner".to_string(),
                data: vec![("miner", json![miners])],
            },
            Log::Event {
                event_name: "commit_miner".to_string(),
                data: vec![
                    ("request_id", json![DEFAULT_REQUEST_ID]),
                    ("answer", json![default_answer_miners[index].clone()]),
                ],
            },
        ]);
    }

    for miners in registered_miners {
        Environment::with_account(miners.clone()).with_block_timestamp(REVEAL_MINER_TIME).create();
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
    }

    // @dev Validators commit an answer
    Environment::with_account(validator_1.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    Environment::with_account(validator_2.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    let validator2_answer = "5f79db20f99cc564b54feb8cbfc08150032e31886c8dc03c93e093006feb1a06";
    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), validator2_answer.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![validator2_answer])],
    }]);

    Environment::with_account(validator_3.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    let answer_validator3 = "08f998c99f900b1cb1899f75e9a86e5df44bf59a1d70648f4b62bc2262672cbe";
    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), answer_validator3.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![answer_validator3])],
    }]);

    //@dev validators reveal their answer
    Environment::with_account(validator_1.clone())
        .with_block_timestamp(REVEAL_VALIDATOR_TIME)
        .create();

    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer.clone(), message.clone());

    assert_eq!(result, RevealValidatorResult::Success);

    assert_logs(vec![Log::Event {
        event_name: "reveal_validator".to_string(),
        data: vec![
            ("request_id", json![DEFAULT_REQUEST_ID]),
            ("answer", json![answer]),
            ("message", json![message]),
        ],
    }]);

    Environment::with_account(validator_2).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer.clone(), message.clone());

    assert_eq!(result, RevealValidatorResult::Success);

    assert_logs(vec![Log::Event {
        event_name: "reveal_validator".to_string(),
        data: vec![
            ("request_id", json![DEFAULT_REQUEST_ID]),
            ("answer", json![answer]),
            ("message", json![message]),
        ],
    }]);

    Environment::with_account(validator_3).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer.clone(), message.clone());

    assert_eq!(result, RevealValidatorResult::Success);

    assert_logs(vec![Log::Event {
        event_name: "reveal_validator".to_string(),
        data: vec![
            ("request_id", json![DEFAULT_REQUEST_ID]),
            ("answer", json![answer]),
            ("message", json![message]),
        ],
    }]);

    Environment::with_account(validator_1).with_block_timestamp(REVEAL_TOPTEN_TIME).create();

    let top_ten = contract.get_top_10_voters(DEFAULT_REQUEST_ID.to_string());

    assert_logs(vec![Log::Event {
        event_name: "topten_miners".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("topten", json![top_ten])],
    }]);
}

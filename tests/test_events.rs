use earthmind_rs::{
    CommitMinerLog, CommitValidatorLog, EventLog, EventLogVariant, RegisterMinerLog, RegisterRequestLog, RegisterValidatorLog, RevealMinerLog,
    RevealValidatorLog,
};

#[test]
fn test_format_register_miner() {
    let expected = r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_miner","data":[{"miner":"hassel.near"},{"miner":"edson.near"}]}"#;
    let log = EventLog {
        standard: "emip001".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::RegisterMiner(vec![
            RegisterMinerLog {
                miner: "hassel.near".parse().unwrap(),
            },
            RegisterMinerLog {
                miner: "edson.near".parse().unwrap(),
            },
        ]),
    };
    assert_eq!(expected, log.to_string());
}

#[test]
fn test_format_register_validator() {
    let expected =
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"hassel.near"},{"validator":"edson.near"}]}"#;
    let log = EventLog {
        standard: "emip001".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::RegisterValidator(vec![
            RegisterValidatorLog {
                validator: "hassel.near".parse().unwrap(),
            },
            RegisterValidatorLog {
                validator: "edson.near".parse().unwrap(),
            },
        ]),
    };
    assert_eq!(expected, log.to_string());
}

#[test]
fn test_format_register_request() {
    let expected = r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"},{"request_id":"38d15af71379737839e4738066fd4091428081d6a57498b2852337a195bc9f5f"}]}"#;
    let log = EventLog {
        standard: "emip001".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::RegisterRequest(vec![
            RegisterRequestLog {
                request_id: "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string(),
            },
            RegisterRequestLog {
                request_id: "38d15af71379737839e4738066fd4091428081d6a57498b2852337a195bc9f5f".to_string(),
            },
        ]),
    };
    assert_eq!(expected, log.to_string());
}

#[test]
fn test_format_commit_miner() {
    let expected = r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_miner","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464"}]}"#;
    let log = EventLog {
        standard: "emip001".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::CommitMiner(vec![CommitMinerLog {
            request_id: "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string(),
            answer: "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string(),
        }]),
    };

    assert_eq!(expected, log.to_string());
}

#[test]
fn test_format_commit_validator() {
    let expected = r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_validator","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464"}]}"#;
    let log = EventLog {
        standard: "emip001".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::CommitValidator(vec![CommitValidatorLog {
            request_id: "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string(),
            answer: "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string(),
        }]),
    };

    assert_eq!(expected, log.to_string());
}

#[test]
fn test_format_reveal_miner() {
    let expected = r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"reveal_miner","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":true,"message":"It's a cool NFT"}]}"#;
    let log = EventLog {
        standard: "emip001".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::RevealMiner(vec![RevealMinerLog {
            request_id: "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string(),
            answer: true,
            message: "It's a cool NFT".to_string(),
        }]),
    };

    assert_eq!(expected, log.to_string());
}

#[test]
fn test_format_reveal_validator() {
    let expected = r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"reveal_validator","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":["hassel.near","edson.near","anne.near","bob.near","alice.near","john.near","harry.near","scott.near","felix.near","margaret.near"],"message":"It's a cool NFT"}]}"#;
    let log = EventLog {
        standard: "emip001".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::RevealValidator(vec![RevealValidatorLog {
            request_id: "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string(),
            answer: vec![
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
            ],
            message: "It's a cool NFT".to_string(),
        }]),
    };

    assert_eq!(expected, log.to_string());
}

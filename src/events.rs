use std::fmt;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

// Enum that represents data type of the eventlog

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant {
    RegisterMiner(Vec<RegisterMinerLog>),
    RegisterValidator(Vec<RegisterValidatorLog>),
    RegisterRequest(Vec<RegisterRequestLog>),
}

//Interface to capture data about an event
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
    pub standard: String,
    pub version: String,

    // `flatten` to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>}.
    #[serde(flatten)]
    pub event: EventLogVariant,
}

impl fmt::Display for EventLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "EVENT_JSON:{}",
            &serde_json::to_string(self).map_err(|_| fmt::Error)?
        ))
    }
}

//An event log to capture miner registered
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RegisterMinerLog {
    pub miner : AccountId,
}

//An event log to capture validator registeres
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RegisterValidatorLog {
    pub validator : AccountId,
}

//An event log to capture validator registeres
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RegisterRequestLog {
    pub request_id: String,
}

// TODO: Add tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nep_format_register_miner() {
        let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"register_miner","data":[{"miner":"hassel.near"},{"miner":"edson.near"}]}"#;
        let log = EventLog {
            standard: "nep171".to_string(),
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
    fn nep_format_register_validator(){
        let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"register_validator","data":[{"validator":"hassel.near"},{"validator":"edson.near"}]}"#;
        let log = EventLog {
            standard: "nep171".to_string(),
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
    fn nep_format_register_request() {
        let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"},{"request_id":"38d15af71379737839e4738066fd4091428081d6a57498b2852337a195bc9f5f"}]}"#;
        let log = EventLog {
            standard: "nep171".to_string(),
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
}
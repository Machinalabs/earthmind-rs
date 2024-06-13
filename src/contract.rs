use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::LookupMap;
use near_sdk::{env, log, near_bindgen, require, AccountId, PanicOnDefault};

pub use crate::constants::*;
pub use crate::events::*;
pub use crate::models::*;

mod constants;
mod events;
mod models;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    requests: LookupMap<Hash, Request>,
    miners: LookupMap<AccountId, Stake>,
    validators: LookupMap<AccountId, Stake>,
}

#[near_bindgen]
impl Contract {
    #[allow(clippy::use_self)]
    #[init]
    pub fn new() -> Self {
        Self {
            requests: LookupMap::new(b"r"),
            miners: LookupMap::new(b"m"),
            validators: LookupMap::new(b"v"),
        }
    }

    pub fn register_miner(&mut self) -> RegisterMinerResult {
        let new_miner_id = env::predecessor_account_id();
        let deposit = env::attached_deposit();

        if deposit < MIN_MINER_STAKE {
            panic!("Miner deposit is less than the minimum stake");
        }

        // @dev Validate the miner is not already registered
        if self.is_miner_registered(new_miner_id.clone()) {
            log!("Attempted to register an already registered miner: {}", new_miner_id);
            return RegisterMinerResult::AlreadyRegistered;
        }

        self.miners.insert(new_miner_id.clone(), deposit);

        let register_miner_log = EventLog {
            standard: "emip001".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::RegisterMiner(vec![RegisterMinerLog { miner: new_miner_id }]),
        };

        log!(&register_miner_log.to_string());

        RegisterMinerResult::Success
    }

    pub fn is_miner_registered(&self, miner_id: AccountId) -> bool {
        self.miners.contains_key(&miner_id)
    }

    pub fn register_validator(&mut self) -> RegisterValidatorResult {
        let new_validator_id = env::predecessor_account_id();
        let deposit = env::attached_deposit();

        if deposit < MIN_VALIDATOR_STAKE {
            panic!("Validator deposit is less than the minimum stake");
        }

        if self.is_validator_registered(new_validator_id.clone()) {
            log!("Attempted to register an already registered validator: {}", new_validator_id);
            return RegisterValidatorResult::AlreadyRegistered;
        }

        self.validators.insert(new_validator_id.clone(), deposit);

        let register_validator_log = EventLog {
            standard: "emip001".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::RegisterValidator(vec![RegisterValidatorLog { validator: new_validator_id }]),
        };

        log!(&register_validator_log.to_string());

        RegisterValidatorResult::Success
    }

    pub fn is_validator_registered(&self, validator_id: AccountId) -> bool {
        self.validators.contains_key(&validator_id)
    }

    pub fn request_governance_decision(&mut self, message: String) -> RegisterRequestResult {
        let new_request_id = env::keccak256(message.as_bytes());
        let new_request_id_hex = hex::encode(new_request_id);

        //@dev Validate the request is not already registered
        if self.get_request_by_id(new_request_id_hex.clone()) {
            log!("Attempted to register an already registered request: {}", new_request_id_hex);
            return RegisterRequestResult::AlreadyRegistered;
        }

        let new_request = Request {
            sender: env::predecessor_account_id(),
            request_id: new_request_id_hex.clone(),
            start_time: env::block_timestamp(),
            miners_proposals: LookupMap::new(b"m"),
            validators_proposals: LookupMap::new(b"v"),
        };

        // @dev We store the key of the request as the hash of the message
        self.requests.insert(new_request_id_hex.clone(), new_request);

        let register_request_log = EventLog {
            standard: "emip001".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::RegisterRequest(vec![RegisterRequestLog {
                request_id: new_request_id_hex,
            }]),
        };

        log!(&register_request_log.to_string());

        RegisterRequestResult::Success
    }

    pub fn get_request_by_id(&self, request_id: Hash) -> bool {
        self.requests.contains_key(&request_id)
    }

    fn get_request_by_id_mut(&mut self, request_id: Hash) -> Option<&mut Request> {
        self.requests.get_mut(&request_id)
    }

    fn get_stage(start_time: u64) -> RequestState {
        let elapsed = env::block_timestamp() - start_time;

        if start_time == 0 {
            RequestState::NonStarted
        } else if elapsed < COMMIT_MINER_DURATION {
            RequestState::CommitMiners
        } else if elapsed < COMMIT_MINER_DURATION + REVEAL_MINER_DURATION {
            RequestState::RevealMiners
        } else if elapsed < COMMIT_MINER_DURATION + REVEAL_MINER_DURATION + COMMIT_VALIDATOR_DURATION {
            RequestState::CommitValidators
        } else if elapsed < COMMIT_MINER_DURATION + REVEAL_MINER_DURATION + COMMIT_VALIDATOR_DURATION + REVEAL_VALIDATOR_DURATION {
            RequestState::RevealValidators
        } else {
            RequestState::Ended
        }
    }

    pub fn hash_miner_answer(self, request_id: Hash, answer: bool, message: String) -> Hash {
        let miner = env::predecessor_account_id();

        let concatenated_answer = format!("{}{}{}{}", request_id, miner, answer, message);
        let value = env::keccak256(concatenated_answer.as_bytes());

        // Return the hash of the answer
        hex::encode(value)
    }

    pub fn commit_by_miner(&mut self, request_id: Hash, answer: Hash) -> CommitMinerResult {
        let miner = env::predecessor_account_id();

        if !self.is_miner_registered(miner.clone()) {
            log!("Miner not registered: {}", miner);
            return CommitMinerResult::Fail;
        }

        match self.get_request_by_id_mut(request_id.clone()) {
            Some(request) => {
                assert_eq!(Self::get_stage(request.start_time), RequestState::CommitMiners, "Not at CommitMiners stage");

                if request.miners_proposals.get(&miner).is_some() {
                    log!("This miner have a commit answer: {}", miner);
                    return CommitMinerResult::Fail;
                }

                let proposal = MinerProposal {
                    proposal_hash: answer.clone(),
                    answer: false,
                    is_revealed: false,
                };

                // @dev Insert miners_proposals using a mut reference
                request.miners_proposals.insert(miner, proposal);

                let commit_miner_log = EventLog {
                    standard: "emip001".to_string(),
                    version: "1.0.0".to_string(),
                    event: EventLogVariant::CommitMiner(vec![CommitMinerLog { request_id, answer }]),
                };

                log!(&commit_miner_log.to_string());

                CommitMinerResult::Success
            }
            None => {
                log!("Request is not registered: {}", request_id);
                CommitMinerResult::Fail
            }
        }
    }

    pub fn hash_validator_answer(self, request_id: String, answer: Vec<AccountId>, message: String) -> Hash {
        let validator = env::predecessor_account_id();

        require!(answer.len() == 10, "Invalid answer");

        let mut concatenated_answer: Vec<u8> = Vec::new();

        concatenated_answer.extend_from_slice(request_id.as_bytes());
        concatenated_answer.extend_from_slice(validator.as_bytes());

        let value: Vec<u8> = answer.iter().flat_map(|id| id.as_bytes()).copied().collect();
        concatenated_answer.extend_from_slice(&value);
        concatenated_answer.extend_from_slice(message.as_bytes());

        let value = env::keccak256(&concatenated_answer);

        // Return the hash of the answer
        hex::encode(value)
    }

    pub fn commit_by_validator(&mut self, request_id: String, answer: Hash) -> CommitValidatorResult {
        let validator = env::predecessor_account_id();

        if !self.is_validator_registered(validator.clone()) {
            log!("Validator is not registered: {}", validator);
            return CommitValidatorResult::Fail;
        }

        match self.get_request_by_id_mut(request_id.clone()) {
            Some(request) => {
                assert_eq!(
                    Self::get_stage(request.start_time),
                    RequestState::CommitValidators,
                    "Not at CommitValidator stage"
                );

                if request.validators_proposals.get(&validator).is_some() {
                    log!("This validator have a commit answer: {}", validator);
                    return CommitValidatorResult::Fail;
                }

                let proposal = ValidatorProposal {
                    proposal_hash: answer.clone(),
                    is_revealed: false,
                    miner_addresses: Vec::new(),
                };

                // @dev Insert miners_proposals using a mut reference
                request.validators_proposals.insert(validator, proposal);

                let commit_validator_log = EventLog {
                    standard: "emip001".to_string(),
                    version: "1.0.0".to_string(),
                    event: EventLogVariant::CommitValidator(vec![CommitValidatorLog { request_id, answer }]),
                };

                log!(&commit_validator_log.to_string());

                CommitValidatorResult::Success
            }
            None => {
                log!("Request is not registered: {}", request_id);
                CommitValidatorResult::Fail
            }
        }
    }

    pub fn reveal_by_miner(&mut self, request_id: String, answer: bool, message: String) -> RevealMinerResult {
        let miner = env::predecessor_account_id();

        if !self.is_miner_registered(miner.clone()) {
            log!("Miner not registered: {}", miner);
            return RevealMinerResult::Fail;
        }

        if self.get_request_by_id_mut(request_id.clone()).is_none() {
            log!("Request is not registered: {}", request_id);
            return RevealMinerResult::Fail;
        }

        let complete_request = self
            .get_request_by_id_mut(request_id.clone())
            .map_or_else(|| panic!("Request not found"), |request| request);

        assert_eq!(
            Self::get_stage(complete_request.start_time),
            RequestState::RevealMiners,
            "Not at RevealMiners stage"
        );

        let save_proposal = complete_request
            .miners_proposals
            .get_mut(&miner)
            .map_or_else(|| panic!("proposal not found"), |proposal| proposal);

        if save_proposal.is_revealed {
            log!("Proposal already revealed");
            return RevealMinerResult::Fail;
        }

        let concatenated_answer = format!("{}{}{}{}", request_id, miner, answer, message);
        let hash_value = env::keccak256(concatenated_answer.as_bytes());
        let answer_to_verify = hex::encode(hash_value);

        if save_proposal.proposal_hash != answer_to_verify {
            log!("Answer don't match");
            return RevealMinerResult::Fail;
        }

        save_proposal.answer = answer;
        save_proposal.is_revealed = true;

        let reveal_miner_log = EventLog {
            standard: "emip001".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::RevealMiner(vec![RevealMinerLog { request_id, answer, message }]),
        };

        env::log_str(&reveal_miner_log.to_string());

        RevealMinerResult::Success
    }

    pub fn reveal_by_validator(&mut self, request_id: String, answer: Vec<AccountId>, message: String) -> RevealValidatorResult {
        let validator = env::predecessor_account_id();

        if !self.is_validator_registered(validator.clone()) {
            log!("Validator is not registered: {}", validator);
            return RevealValidatorResult::Fail;
        }

        if self.get_request_by_id_mut(request_id.clone()).is_none() {
            log!("Request is not registered: {}", request_id);
            return RevealValidatorResult::Fail;
        }

        let complete_request = self
            .get_request_by_id_mut(request_id.clone())
            .map_or_else(|| panic!("Request not found"), |request| request);

        assert_eq!(
            Self::get_stage(complete_request.start_time),
            RequestState::RevealValidators,
            "Not at RevealValidators stage"
        );

        let save_proposal = complete_request
            .validators_proposals
            .get_mut(&validator)
            .map_or_else(|| panic!("proposal not found"), |proposal| proposal);

        if save_proposal.is_revealed {
            log!("Proposal already revealed");
            return RevealValidatorResult::Fail;
        }

        if answer.len() != 10 {
            log!("Invalid answer");
            return RevealValidatorResult::Fail;
        }

        let mut concatenated_answer: Vec<u8> = Vec::new();

        concatenated_answer.extend_from_slice(request_id.as_bytes());
        concatenated_answer.extend_from_slice(validator.as_bytes());

        let value: Vec<u8> = answer.iter().flat_map(|id| id.as_bytes()).copied().collect();
        concatenated_answer.extend_from_slice(&value);
        concatenated_answer.extend_from_slice(message.as_bytes());

        let value = env::keccak256(&concatenated_answer);
        let hash_answer = hex::encode(value);

        if save_proposal.proposal_hash != hash_answer {
            log!("Answer don't match");
            return RevealValidatorResult::Fail;
        }

        save_proposal.is_revealed = true;
        let answer_for_log = answer.clone();

        for addresses in answer {
            save_proposal.miner_addresses.push(addresses);
        }

        let reveal_validator_log = EventLog {
            standard: "emip001".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::RevealValidator(vec![RevealValidatorLog {
                request_id,
                answer: answer_for_log,
                message,
            }]),
        };

        env::log_str(&reveal_validator_log.to_string());
        RevealValidatorResult::Success
    }
}

// Test private function "get_request_by_id_mut"
#[cfg(test)]
mod test {
    use super::*;
    use near_sdk::{
        env,
        test_utils::{get_logs, VMContextBuilder},
        testing_env, AccountId, NearToken,
    };
    fn get_context(predecessor_account_id: AccountId, block_timestamp: u64, attached_deposit: NearToken) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .predecessor_account_id(predecessor_account_id)
            .block_timestamp(block_timestamp)
            .attached_deposit(attached_deposit);
        builder
    }

    #[test]
    fn test_request_governance_decision() {
        let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
        testing_env!(context.build());

        let mut contract = Contract::new();

        let message = "Should we add this new NFT to our protocol?";

        let result_1 = contract.request_governance_decision(message.to_string());
        assert_eq!(result_1, RegisterRequestResult::Success);

        let request_id = env::keccak256(message.as_bytes());
        let request_id_hex = hex::encode(request_id);
        assert!(contract.get_request_by_id_mut(request_id_hex).is_some());

        let logs = get_logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(
            logs[0],
            r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"}]}"#
        );

        let context = get_context("edson.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
        testing_env!(context.build());

        let message_2 = "Should we add this to our protocol?";
        let result_2 = contract.request_governance_decision(message_2.to_string());
        assert_eq!(result_2, RegisterRequestResult::Success);

        let request_id_2 = env::keccak256(message_2.as_bytes());
        let request_id_hex_2 = hex::encode(request_id_2);
        assert!(contract.get_request_by_id_mut(request_id_hex_2).is_some());

        let logs = get_logs();
        assert_eq!(logs.len(), 1);

        assert_eq!(
            logs[0],
            r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"38d15af71379737839e4738066fd4091428081d6a57498b2852337a195bc9f5f"}]}"#
        );
    }

    #[test]
    fn test_get_request_by_id_mut() {
        let context = get_context("hassel.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
        testing_env!(context.build());

        let mut contract = Contract::new();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726";
        assert!(contract.get_request_by_id_mut(request_id.to_string()).is_some());
    }

    #[test]
    fn test_get_request_by_id_mut_when_not_registered() {
        let mut contract = Contract::new();
        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae727";

        assert!(contract.get_request_by_id_mut(request_id.to_string()).is_none());
    }
}

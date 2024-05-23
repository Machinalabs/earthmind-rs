use hex;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::store::{LookupMap, Vector};
use near_sdk::{env, log, near_bindgen, require, AccountId, PanicOnDefault};

const COMMIT_MINER_DURATION: u64 = 5; // 2 minutes
const REVEAL_MINER_DURATION: u64 = 1; // 2 minutes
const COMMIT_VALIDATOR_DURATION: u64 = 5; // 2 minutes
const REVEAL_VALIDATOR_DURATION: u64 = 1; // 2 minutes

type Hash = String;

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum RegisterMinerResult {
    Success,
    AlreadyRegistered,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum RegisterValidatorResult {
    Success,
    AlreadyRegistered,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum RegisterRequestResult {
    Success,
    AlreadyRegistered,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum CommitMinerResult {
    Success,
    Fail,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum RevealMinerResult {
    Success,
    Fail,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum CommitValidatorResult {
    Success,
    Fail,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum RevealValidatorResult {
    Success,
    Fail,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MinerProposal {
    proposal_hash: Hash,
    is_revealed: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ValidatorProposal {
    proposal_hash: Hash,
    is_revealed: bool,
    miner_addresses: Vec<AccountId>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Request {
    sender: AccountId,
    request_id: String,
    start_time: u64,
    miners_proposals: LookupMap<AccountId, MinerProposal>,
    validators_proposals: LookupMap<AccountId, ValidatorProposal>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    requests: Vector<Request>,
    miners: Vector<AccountId>,
    validators: Vector<AccountId>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            requests: Vector::new(b"r"),
            miners: Vector::new(b"m"),
            validators: Vector::new(b"v"),
        }
    }

    pub fn register_miner(&mut self) -> RegisterMinerResult {
        let new_miner_id = env::predecessor_account_id();

        // @dev Validate the miner is not already registered
        if self.get_register_miner(new_miner_id.clone()).is_some() {
            log!(
                "Attempted to register an already registered miner: {}",
                new_miner_id
            );
            return RegisterMinerResult::AlreadyRegistered;
        }

        self.miners.push(new_miner_id.clone());

        log!("Registered new miner: {}", new_miner_id);

        RegisterMinerResult::Success
    }

    pub fn get_register_miner(&self, miner_id: AccountId) -> Option<&AccountId> {
        for miner in self.miners.iter() {
            if *miner == miner_id {
                return Some(miner);
            }
        }
        None
    }

    pub fn register_validator(&mut self) -> RegisterValidatorResult {
        let new_validator_id = env::predecessor_account_id();

        if self
            .get_register_validator(new_validator_id.clone())
            .is_some()
        {
            log!(
                "Attempted to register an already registered validator: {}",
                new_validator_id
            );
            return RegisterValidatorResult::AlreadyRegistered;
        }

        self.validators.push(new_validator_id.clone());

        log!("Registered new validator: {}", new_validator_id);

        RegisterValidatorResult::Success
    }

    pub fn get_register_validator(&self, validator_id: AccountId) -> Option<&AccountId> {
        for validator in self.validators.iter() {
            if *validator == validator_id {
                return Some(validator);
            }
        }
        None
    }

    pub fn request_governance_decision(&mut self, message: String) -> RegisterRequestResult {
        let new_request_id = env::keccak256(message.as_bytes());
        let new_request_id_hex = hex::encode(new_request_id);

        //@dev Validate the request is not already registered
        if self.get_request_by_id(new_request_id_hex.clone()).is_some() {
            log!(
                "Attempted to register an already registered request: {}",
                new_request_id_hex
            );
            return RegisterRequestResult::AlreadyRegistered;
        }

        let new_request = Request {
            sender: env::predecessor_account_id(),
            request_id: new_request_id_hex.clone(),
            start_time: env::epoch_height(),
            miners_proposals: LookupMap::new(b"m"),
            validators_proposals: LookupMap::new(b"v"),
        };

        self.requests.push(new_request);

        log!("Registered new request: {}", new_request_id_hex);
        RegisterRequestResult::Success
    }

    fn get_request_by_id(&mut self, request_id: String) -> Option<&mut Request> {
        for request in self.requests.iter_mut() {
            if request.request_id == request_id {
                return Some(request);
            }
        }
        None
    }

    pub fn hash_miner_answer(self, answer: bool, message: String) -> Hash {

        let concatenated_answer = format!("{}{}", answer, message);
        let value = env::keccak256(concatenated_answer.as_bytes());
        let hash_answer = hex::encode(value);

        return hash_answer;
    }

    pub fn commit_by_miner( &mut self, request_id: String, answer : Hash) -> CommitMinerResult {

        let miner = env::predecessor_account_id();

        if self.get_register_miner(miner.clone()).is_none() {
            log!("Miner is not registered: {}", miner);
            return CommitMinerResult::Fail;
        }

        if self.get_request_by_id(request_id.clone()).is_none() {
            log!("Request is not registered: {}", request_id);
            return CommitMinerResult::Fail;
        }

        let complete_request: &mut Request = match self.get_request_by_id(request_id) {
            Some(request) => request,
            None => panic!("Request not found"),
        };

        if complete_request.miners_proposals.get(&miner).is_some() {
            log!("This miner have a commit answer: {}", miner);
            return CommitMinerResult::Fail;
        }

        let proposal = MinerProposal {
            proposal_hash: answer,
            is_revealed: false,
        };

        complete_request.miners_proposals.insert(miner, proposal);

        log!("Miner proposal registered successfully");
        return CommitMinerResult::Success;

    }

    pub fn hash_validator_answer(self, answer: Vec<AccountId>, message: String) -> Hash {

        require!(answer.len() == 10, "Give 10 miners");

        let mut concatenated_answer: Vec<u8> = answer.iter().flat_map(|id| id.as_bytes()).copied().collect();
        concatenated_answer.extend_from_slice(message.as_bytes());
        let value = env::keccak256(&concatenated_answer);
        let hash_answer = hex::encode(value);

        return hash_answer;
    }

    pub fn commit_by_validator(&mut self, request_id: String, answer: Hash) -> CommitValidatorResult {
        let validator = env::predecessor_account_id();

        if self.get_register_validator(validator.clone()).is_none() {
            log!("Validator is not registered: {}", validator);
            return CommitValidatorResult::Fail;
        }

        if self.get_request_by_id(request_id.clone()).is_none() {
            log!("Request is not registered: {}", request_id);
            return CommitValidatorResult::Fail;
        }

        let complete_request: &mut Request = match self.get_request_by_id(request_id) {
            Some(request) => request,
            None => panic!("Request not found"),
        };

        if complete_request
            .validators_proposals
            .get(&validator)
            .is_some()
        {
            log!("This validator have a commit answer: {}", validator);
            return CommitValidatorResult::Fail;
        }

        let proposal = ValidatorProposal {
            proposal_hash: answer,
            is_revealed: false,
            miner_addresses: Vec::new(),
        };

        complete_request
            .validators_proposals
            .insert(validator, proposal);

        log!("Validator proposal registered successfully");
        return CommitValidatorResult::Success;

    }

    pub fn reveal_by_miner(&mut self, request_id: String, answer: bool, message: String) -> RevealMinerResult {
        let miner = env::predecessor_account_id();

        if self.get_register_miner(miner.clone()).is_none() {
            log!("Miner is not registered: {}", miner);
            return RevealMinerResult::Fail;
        }

        if self.get_request_by_id(request_id.clone()).is_none() {
            log!("Request is not registered: {}", request_id);
            return RevealMinerResult::Fail;
        }

        let complete_request = match self.get_request_by_id(request_id) {
            Some(request) => request,
            None => panic!("Request not found"),
        };

        let save_proposal = match complete_request.miners_proposals.get_mut(&miner) {
            Some(proposal) => proposal,
            None => panic!("proposal not found"),
        };

        if save_proposal.is_revealed == true {
            log!("Proposal already revealed");
            return RevealMinerResult::Fail;
        }

        let concatenated_answer = format!("{}{}", answer, message);
        let hash_value = env::keccak256(concatenated_answer.as_bytes());
        let answer_to_verify = hex::encode(hash_value);

        if save_proposal.proposal_hash != answer_to_verify {
            log!("Answer don't match");
            return RevealMinerResult::Fail;
        }

        save_proposal.is_revealed = true;
        return RevealMinerResult::Success;
    }

    pub fn reveal_by_validator( &mut self, request_id: String, answer: Vec<AccountId>, message: String) -> RevealValidatorResult {
        let validator = env::predecessor_account_id();

        if self.get_register_validator(validator.clone()).is_none() {
            log!("Validator is not registered: {}", validator);
            return RevealValidatorResult::Fail;
        }

        if self.get_request_by_id(request_id.clone()).is_none() {
            log!("Request is not registered: {}", request_id);
            return RevealValidatorResult::Fail;
        }

        let complete_request = match self.get_request_by_id(request_id) {
            Some(request) => request,
            None => panic!("Request not found"),
        };

        let save_proposal = match complete_request.validators_proposals.get_mut(&validator) {
            Some(proposal) => proposal,
            None => panic!("proposal not found"),
        };

        if save_proposal.is_revealed == true {
            log!("Proposal already revealed");
            return RevealValidatorResult::Fail;
        }

        if answer.len() != 10 {
            log!("Vote for 10 miners");
            return RevealValidatorResult::Fail;
        }

        let mut concatenated_answer: Vec<u8> = answer.iter().flat_map(|id| id.as_bytes()).copied().collect();
        concatenated_answer.extend_from_slice(message.as_bytes());
        let hash_value = env::keccak256(&concatenated_answer);
        let answer_to_verify = hex::encode(hash_value);

        if save_proposal.proposal_hash != answer_to_verify {
            log!("Answer don't match");
            return RevealValidatorResult::Fail;
        }

        save_proposal.is_revealed = true;

        for addresses  in answer {
            save_proposal.miner_addresses.push(addresses);
        }
        
        return RevealValidatorResult::Success;
    } 
} 

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{
        test_utils::{get_logs, VMContextBuilder},
        testing_env,
    };
    use near_workspaces::result;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor_account_id);
        builder
    }

    // Register Miner

    #[test]
    fn test_register_miner() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        let result_1 = contract.register_miner();
        assert_eq!(result_1, RegisterMinerResult::Success);

        let miner_1: AccountId = "hassel.near".parse().unwrap();
        assert!(contract.get_register_miner(miner_1).is_some());

        let logs = get_logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0], "Registered new miner: hassel.near");

        let context = get_context("edson.near".parse().unwrap());
        testing_env!(context.build());

        let result_2 = contract.register_miner();
        assert_eq!(result_2, RegisterMinerResult::Success);

        let miner_2: AccountId = "edson.near".parse().unwrap();
        assert!(contract.get_register_miner(miner_2).is_some());

        let logs = get_logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0], "Registered new miner: edson.near");
    }

    #[test]
    fn test_register_miner_when_is_registered_returns_already_registered() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_miner();
        let result = contract.register_miner();

        assert_eq!(result, RegisterMinerResult::AlreadyRegistered);

        let logs = get_logs();

        assert_eq!(logs.len(), 2);

        assert_eq!(logs[0], "Registered new miner: hassel.near");
        assert_eq!(
            logs[1],
            "Attempted to register an already registered miner: hassel.near"
        );
    }

    #[test]
    fn test_get_register_miner() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_miner();

        let miner: AccountId = "hassel.near".parse().unwrap();
        assert!(contract.get_register_miner(miner).is_some());
    }

    #[test]
    fn test_get_register_miner_when_not_registered() {
        let contract = Contract::new();

        let miner: AccountId = "hassel.near".parse().unwrap();

        assert!(contract.get_register_miner(miner).is_none());
    }

    // Register Validator

    #[test]
    fn test_register_validator() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        let result_1 = contract.register_validator();
        assert_eq!(result_1, RegisterValidatorResult::Success);

        let validator_1: AccountId = "hassel.near".parse().unwrap();
        assert!(contract.get_register_validator(validator_1).is_some());

        let logs = get_logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0], "Registered new validator: hassel.near");

        let context = get_context("edson.near".parse().unwrap());
        testing_env!(context.build());

        let result2 = contract.register_validator();
        assert_eq!(result2, RegisterValidatorResult::Success);

        let validator_2: AccountId = "edson.near".parse().unwrap();
        assert!(contract.get_register_validator(validator_2).is_some());

        let logs = get_logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0], "Registered new validator: edson.near");
    }

    #[test]
    fn test_register_validator_when_is_registered_returns_already_registered() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_validator();

        let result = contract.register_validator();

        assert_eq!(result, RegisterValidatorResult::AlreadyRegistered);

        let logs = get_logs();

        assert_eq!(logs.len(), 2);

        assert_eq!(logs[0], "Registered new validator: hassel.near");
        assert_eq!(
            logs[1],
            "Attempted to register an already registered validator: hassel.near"
        );
    }

    #[test]
    fn test_get_register_validator() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();
        contract.register_validator();

        let validator: AccountId = "hassel.near".parse().unwrap();
        assert!(contract.get_register_validator(validator).is_some());
    }

    #[test]
    fn test_get_register_validator_when_not_registered() {
        let contract = Contract::new();
        let validator: AccountId = "hassel.near".parse().unwrap();

        assert!(contract.get_register_validator(validator).is_none());
    }

    // Request Governance Decision

    #[test]
    fn test_request_governance_decision() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        let message = "Should we add this new NFT to our protocol?";

        let result_1 = contract.request_governance_decision(message.to_string());
        assert_eq!(result_1, RegisterRequestResult::Success);

        let request_id = env::keccak256(message.as_bytes());
        let request_id_hex = hex::encode(request_id);
        assert!(contract.get_request_by_id(request_id_hex).is_some());

        let logs = get_logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0], "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726");

        let context = get_context("edson.near".parse().unwrap());
        testing_env!(context.build());

        let message_2 = "Should we add this to our protocol?";
        let result_2 = contract.request_governance_decision(message_2.to_string());
        assert_eq!(result_2, RegisterRequestResult::Success);

        let request_id_2 = env::keccak256(message_2.as_bytes());
        let request_id_hex_2 = hex::encode(request_id_2);
        assert!(contract.get_request_by_id(request_id_hex_2).is_some());

        let logs = get_logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0], "Registered new request: 38d15af71379737839e4738066fd4091428081d6a57498b2852337a195bc9f5f");
    }

    #[test]
    fn test_request_governance_decision_when_is_registered_returns_already_registered() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let result = contract.request_governance_decision(message.to_string());

        assert_eq!(result, RegisterRequestResult::AlreadyRegistered);

        let logs = get_logs();
        assert_eq!(logs.len(), 2);

        assert_eq!(logs[0], "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726");
        assert_eq!(
            logs[1],
            "Attempted to register an already registered request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
        );
    }

    // Request by id

    #[test]
    fn test_get_request_by_id() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726";
        assert!(contract.get_request_by_id(request_id.to_string()).is_some());
    }

    #[test]
    fn test_get_request_by_id_when_not_registered() {
        let mut contract = Contract::new();
        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae727";

        assert!(contract.get_request_by_id(request_id.to_string()).is_none());
    }

    // Hash miner answer

    #[test]
    fn test_hash_miner_answer(){

        let contract = Contract::new();

        let answer = true;
        let message = "It's a cool NFT".to_string(); 

        let result = contract.hash_miner_answer(answer, message);

        assert_eq!(result, "436ec6188940d3fd6481f3225f367a0dd8d6715582867ef92aa7182d9916d8b4");
    }

    // Hash validator answer 
    #[test]
    fn test_hash_validator_answer(){
        let contract = Contract::new();

        let answer = generate_validator_answer() ;
        let message = "It's a cool NFT".to_string(); 

        let result = contract.hash_validator_answer(answer, message);

        assert_eq!(result, "bfe7c4557ab44dc0362d32ef844fe50523f00d03d571d9ff1a689c870f54fbc4");
    }

    #[test]
    #[should_panic]
    fn test_hash_validator_answer_when_answer_is_not_complete() {
      
        let contract = Contract::new();
        let message = "It's a cool NFT".to_string(); 

        let answer = generate_validator_answer() ;
        let answer: Vec<AccountId> = answer[0..answer.len()-1].to_vec();
        
        contract.hash_validator_answer(answer.clone(),message);

    }

    // Commit by miner

    #[test]
    fn test_commit_by_miner_when_miner_and_request_exist() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_miner();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "436ec6188940d3fd6481f3225f367a0dd8d6715582867ef92aa7182d9916d8b4".to_string();

        let result = contract.commit_by_miner(request_id, answer);

        assert_eq!(result, CommitMinerResult::Success);

        let logs = get_logs();
        assert_eq!(logs.len(), 3);

        assert_eq!(logs[0], "Registered new miner: hassel.near");
        assert_eq!(logs[1], "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726");
        assert_eq!(logs[2], "Miner proposal registered successfully");
    }

    #[test]
    fn test_commit_by_miner_when_miner_dont_registered_and_request_exist() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_miner();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let context = get_context("edson.near".parse().unwrap());
        testing_env!(context.build());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "436ec6188940d3fd6481f3225f367a0dd8d6715582867ef92aa7182d9916d8b4".to_string();

        let result = contract.commit_by_miner(request_id, answer);

        assert_eq!(result, CommitMinerResult::Fail);

        let logs = get_logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0], "Miner is not registered: edson.near");
    }

    #[test]
    fn test_commit_by_miner_when_miner_registered_and_request_dont_exist() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_miner();

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "436ec6188940d3fd6481f3225f367a0dd8d6715582867ef92aa7182d9916d8b4".to_string();

        let result = contract.commit_by_miner(request_id, answer);

        assert_eq!(result, CommitMinerResult::Fail);

        let logs = get_logs();
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0], "Registered new miner: hassel.near");
        assert_eq!(logs[1], "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726")
    }

    #[test]
    fn test_commit_by_miner_when_miner_and_request_exist_and_commit_already() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_miner();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "436ec6188940d3fd6481f3225f367a0dd8d6715582867ef92aa7182d9916d8b4".to_string();

        contract.commit_by_miner(request_id.clone(), answer.clone());

        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let result = contract.commit_by_miner(request_id, answer);

        assert_eq!(result, CommitMinerResult::Fail);

        let logs = get_logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0], "This miner have a commit answer: hassel.near");
    }


    // Commit by validator

    #[test]
    fn test_commit_by_validator_when_validator_and_request_exist() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_validator();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "bfe7c4557ab44dc0362d32ef844fe50523f00d03d571d9ff1a689c870f54fbc4".to_string();

        let result = contract.commit_by_validator(request_id, answer);

        assert_eq!(result, CommitValidatorResult::Success);

        let logs = get_logs();
        assert_eq!(logs.len(), 3);
        assert_eq!(logs[0], "Registered new validator: hassel.near");
        assert_eq!(logs[1], "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726");
        assert_eq!(logs[2], "Validator proposal registered successfully");
    }

    #[test]
    fn test_commit_by_validator_when_validator_dont_registered_and_request_exist() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_validator();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let context = get_context("edson.near".parse().unwrap());
        testing_env!(context.build());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "bfe7c4557ab44dc0362d32ef844fe50523f00d03d571d9ff1a689c870f54fbc4".to_string();

        let result = contract.commit_by_validator(request_id, answer);

        assert_eq!(result, CommitValidatorResult::Fail);

        let logs = get_logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0], "Validator is not registered: edson.near");
    }

    
    #[test]
    fn test_commit_by_validator_when_validator_registered_and_request_dont_exist() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_validator();

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "bfe7c4557ab44dc0362d32ef844fe50523f00d03d571d9ff1a689c870f54fbc4".to_string();

        let result = contract.commit_by_validator(request_id, answer);

        assert_eq!(result, CommitValidatorResult::Fail);

        let logs = get_logs();
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0], "Registered new validator: hassel.near");
        assert_eq!(logs[1], "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726")
    }

    #[test]
    fn test_commit_by_validator_when_miner_and_request_exist_and_commit_already() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_validator();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "bfe7c4557ab44dc0362d32ef844fe50523f00d03d571d9ff1a689c870f54fbc4".to_string();

        contract.commit_by_validator(request_id.clone(), answer.clone());

        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let result = contract.commit_by_validator(request_id, answer);

        assert_eq!(result, CommitValidatorResult::Fail);

        let logs = get_logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0], "This validator have a commit answer: hassel.near");
    }

  

    // Reveal by miner

    #[test]
    fn test_reveal_by_miner() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_miner();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "436ec6188940d3fd6481f3225f367a0dd8d6715582867ef92aa7182d9916d8b4".to_string();

        contract.commit_by_miner(request_id.clone(), answer);

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = true;
        let message = "It's a cool NFT".to_string(); 

        let result = contract.reveal_by_miner(request_id, answer, message);

        assert_eq!(result, RevealMinerResult::Success);

        let logs = get_logs();
        assert_eq!(logs.len(), 3);

        assert_eq!(logs[0], "Registered new miner: hassel.near");
        assert_eq!(logs[1], "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726");
        assert_eq!(logs[2], "Miner proposal registered successfully");
    }

    #[test]
    fn test_reveal_by_miner_when_miner_is_not_registered() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_miner();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "436ec6188940d3fd6481f3225f367a0dd8d6715582867ef92aa7182d9916d8b4".to_string();

        contract.commit_by_miner(request_id.clone(), answer);

        let context = get_context("edson.near".parse().unwrap());
        testing_env!(context.build());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = true;
        let message = "It's a cool NFT".to_string(); 

        let result = contract.reveal_by_miner(request_id, answer, message);

        assert_eq!(result, RevealMinerResult::Fail);

        let logs = get_logs();
        assert_eq!(logs.len(), 1);

        assert_eq!(logs[0], "Miner is not registered: edson.near");
    }

    #[test]
    fn test_reveal_by_miner_when_request_is_not_registered() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_miner();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "436ec6188940d3fd6481f3225f367a0dd8d6715582867ef92aa7182d9916d8b4".to_string();

        contract.commit_by_miner(request_id.clone(), answer);
        let request_id ="0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725".to_string();
        
        let answer = true;
        let message = "It's a cool NFT".to_string(); 
        let result = contract.reveal_by_miner(request_id, answer, message);

        assert_eq!(result, RevealMinerResult::Fail);

        let logs = get_logs();
        assert_eq!(logs.len(), 4);

        assert_eq!(logs[0], "Registered new miner: hassel.near");
        assert_eq!(logs[1], "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726");
        assert_eq!(logs[2], "Miner proposal registered successfully");
        assert_eq!(logs[3], "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725");
    }

    
    #[test]
    fn test_reveal_by_miner_when_proposal_is_already_reveal() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_miner();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "436ec6188940d3fd6481f3225f367a0dd8d6715582867ef92aa7182d9916d8b4".to_string();

        contract.commit_by_miner(request_id.clone(), answer);

        let answer = true;
        let message = "It's a cool NFT".to_string(); 

        contract.reveal_by_miner(request_id.clone(), answer, message.clone());

        let result = contract.reveal_by_miner(request_id, answer, message);

        assert_eq!(result, RevealMinerResult::Fail);

        let logs = get_logs();
        assert_eq!(logs.len(), 4);

        assert_eq!(logs[0], "Registered new miner: hassel.near");
        assert_eq!(logs[1], "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726");
        assert_eq!(logs[2], "Miner proposal registered successfully");
        assert_eq!(logs[3], "Proposal already revealed");
    }

    #[test]
    fn test_reveal_by_miner_when_answer_not_equal() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_miner();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "436ec6188940d3fd6481f3225f367a0dd8d6715582867ef92aa7182d9916d8b4".to_string();

        contract.commit_by_miner(request_id.clone(), answer);

        let answer = false;
        let message = "It's a cool NFT".to_string();

        let result = contract.reveal_by_miner(request_id, answer, message);

        assert_eq!(result, RevealMinerResult::Fail);

        let logs = get_logs();
        assert_eq!(logs.len(), 4);

        assert_eq!(logs[0], "Registered new miner: hassel.near");
        assert_eq!(logs[1], "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726");
        assert_eq!(logs[2], "Miner proposal registered successfully");
        assert_eq!(logs[3], "Answer don't match");
    }

    // Reveal by validator

    #[test]
    fn test_reveal_by_validator() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_validator();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "bfe7c4557ab44dc0362d32ef844fe50523f00d03d571d9ff1a689c870f54fbc4".to_string();

        contract.commit_by_validator(request_id.clone(), answer);

        let answer = generate_validator_answer() ;
        let message = "It's a cool NFT".to_string(); 
        let result = contract.reveal_by_validator(request_id, answer, message);

        assert_eq!(result, RevealValidatorResult::Success);

        let logs = get_logs();
        assert_eq!(logs.len(), 3);

        assert_eq!(logs[0], "Registered new validator: hassel.near");
        assert_eq!(logs[1], "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726");
        assert_eq!(logs[2], "Validator proposal registered successfully");
    }

    #[test]
    fn test_reveal_by_validator_when_validator_is_not_registered() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_validator();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());
        
        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "bfe7c4557ab44dc0362d32ef844fe50523f00d03d571d9ff1a689c870f54fbc4".to_string();

        contract.commit_by_validator(request_id.clone(), answer.clone());

        let context = get_context("edson.near".parse().unwrap());
        testing_env!(context.build());

        let answer = generate_validator_answer() ;
        let message = "It's a cool NFT".to_string(); 
        let result = contract.reveal_by_validator(request_id, answer, message);

        assert_eq!(result, RevealValidatorResult::Fail);

        let logs = get_logs();
        assert_eq!(logs.len(), 1);

        assert_eq!(logs[0], "Validator is not registered: edson.near");
    }

    #[test]
    fn test_reveal_by_validator_when_request_is_not_registered() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_validator();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "bfe7c4557ab44dc0362d32ef844fe50523f00d03d571d9ff1a689c870f54fbc4".to_string();


        contract.commit_by_validator(request_id.clone(), answer.clone());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725".to_string();
        let answer: Vec<AccountId> = generate_validator_answer();
        let message = "It's a cool NFT".to_string();
    
        let result = contract.reveal_by_validator(request_id, answer, message);

        assert_eq!(result, RevealValidatorResult::Fail);

        let logs = get_logs();
        assert_eq!(logs.len(), 4);

        assert_eq!(logs[0], "Registered new validator: hassel.near");
        assert_eq!(logs[1], "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726");
        assert_eq!(logs[2], "Validator proposal registered successfully");
        assert_eq!(logs[3], "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725");
    }

    #[test]
    fn test_reveal_by_validator_when_proposal_is_already_reveal() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_validator();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "bfe7c4557ab44dc0362d32ef844fe50523f00d03d571d9ff1a689c870f54fbc4".to_string();

        contract.commit_by_validator(request_id.clone(), answer.clone());
        
        let answer: Vec<AccountId> = generate_validator_answer();
        let message = "It's a cool NFT".to_string();

        contract.reveal_by_validator(request_id.clone(), answer.clone(), message.clone());

        let result = contract.reveal_by_validator(request_id, answer, message);

        assert_eq!(result, RevealValidatorResult::Fail);

        let logs = get_logs();
        assert_eq!(logs.len(), 4);

        assert_eq!(logs[0], "Registered new validator: hassel.near");
        assert_eq!(logs[1], "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726");
        assert_eq!(logs[2], "Validator proposal registered successfully");
        assert_eq!(logs[3], "Proposal already revealed");
    }
    

    #[test]
    fn test_reveal_by_validator_when_answer_not_equal() {
        let context = get_context("hassel.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.register_validator();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string();
        let answer = "bfe7c4557ab44dc0362d32ef844fe50523f00d03d571d9ff1a689c870f54fbc4".to_string();

        contract.commit_by_validator(request_id.clone(), answer.clone());

        let mut answer: Vec<AccountId> = generate_validator_answer();
        answer[9] = "jane.near".parse().unwrap();
  
        let message = "It's a cool NFT".to_string();

        let result = contract.reveal_by_validator(request_id.clone(), answer.clone(), message.clone());

        assert_eq!(result, RevealValidatorResult::Fail);

        let logs = get_logs();
        assert_eq!(logs.len(), 4);

        assert_eq!(logs[0], "Registered new validator: hassel.near");
        assert_eq!(logs[1], "Registered new request: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726");
        assert_eq!(logs[2], "Validator proposal registered successfully");
        assert_eq!(logs[3], "Answer don't match");
    }

    fn generate_validator_answer() -> Vec<AccountId> {
        let value = vec![
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
                    ];
        return value;
        }

}

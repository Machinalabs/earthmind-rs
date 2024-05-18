use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::store::{LookupMap, Vector};
use near_sdk::{env, log, near_bindgen, require, AccountId, PanicOnDefault};

const COMMIT_MINER_DURATION: u64 = 5; // 2 minutes
const REVEAL_MINER_DURATION: u64 = 1; // 2 minutes
const COMMIT_VALIDATOR_DURATION: u64 = 5; // 2 minutes
const REVEAL_VALIDATOR_DURATION: u64 = 1; // 2 minutes

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MinerProposal {
    proposal_hash: Vec<u8>,
    is_revealed: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ValidatorProposal {
    proposal_hash: Vec<u8>,
    is_revealed: bool,
    miner_addresses: Vec<AccountId>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Request {
    sender: AccountId,
    request_id: u64,
    start_time: u64,
    miners_proposals: LookupMap<AccountId, MinerProposal>,
    validators_proposals: LookupMap<AccountId, ValidatorProposal>,
}

/// Main contract structure serialized with Borsh
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    requests: Vector<Request>,
    miners: Vector<AccountId>,
    validators: Vector<AccountId>,
}

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

    pub fn register_miner(&mut self, new_miner_id: AccountId) -> RegisterMinerResult {
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

    pub fn get_register_miner(&mut self, miner_id: AccountId) -> Option<&AccountId> {
        for miner in self.miners.iter() {
            if *miner == miner_id {
                return Some(miner);
            }
        }
        None
    }

    pub fn register_validator(&mut self, new_validator_id: AccountId) -> RegisterValidatorResult {
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

    pub fn get_register_validator(&mut self, miner_id: AccountId) -> Option<&AccountId> {
        for validator in self.validators.iter() {
            if *validator == miner_id {
                return Some(validator);
            }
        }
        None
    }

    pub fn request_governance_decision(&mut self, new_request_id: u64) -> RegisterRequestResult{

        //@dev Validate the request is not already registered
        if self.get_request_by_id(new_request_id.clone()).is_some(){
            log!("Attempted to register an already registered request: {}", new_request_id);
            return RegisterRequestResult::AlreadyRegistered;
        }

        let new_request = Request {
            sender: env::predecessor_account_id(),
            request_id: new_request_id,
            start_time: env::epoch_height(),
            miners_proposals: LookupMap::new(b"m"),
            validators_proposals: LookupMap::new(b"v"),
        };
        self.requests.push(new_request);
        log!("Registered new request: {}", new_request_id);
        RegisterRequestResult::Success
    }

    fn get_request_by_id(&mut self, request_id: u64) -> Option<&Request> {
        for request in self.requests.iter() {
            if request.request_id == request_id {
                return Some(request);
            }
        }
        None
    }

    pub fn commit_by_miner(&mut self, miner: AccountId, request_id: u64, answer: String) {
        
        if self.get_register_miner(miner.clone()).is_none() {
            log!("Miner ")
        }
        let miner_to_commit = self.get_register_miner(miner);
        require!(miner_to_commit.is_some());

        let request_exist = self.get_request_by_id(request_id);
        require!(request_exist.is_some());

        let complete_request = match self.get_request_by_id(request_id) {
            Some(request) => request,
            None => panic!("Request not found"),
        };

        require!(
            env::epoch_height() < complete_request.commit_miner_deadline,
            "No time to commit"
        );

        let proposal = MinerProposal {
            proposal_hash: env::keccak256(answer.as_bytes()),
            is_revealed: false,
        };

        complete_request
            .miners_proposals
            .insert(env::predecessor_account_id(), proposal);
    }

    // //TODO: Answer in this method is a vector with the top ten
    // pub fn commit_by_validator(&mut self, validator: AccountId, request_id: u64, answer: String) {
    //     let validator_to_commit = self.get_register_validator(validator);
    //     require!(validator_to_commit.is_some());

    //     let request_exist = self.get_request_by_id(request_id);
    //     require!(request_exist.is_some());

    //     let complete_request: &mut Request = match self.get_request_by_id(request_id) {
    //         Some(request) => request,
    //         None => panic!("Request not found"),
    //     };

    //     require!(
    //         env::epoch_height() > complete_request.reveal_miner_deadline,
    //         "Miner commit time"
    //     );
    //     require!(
    //         env::epoch_height() < complete_request.commit_validator_deadline,
    //         "No time to commit"
    //     );

    //     //TODO: answer is a vector with the list of miners to vote
    //     let proposal = ValidatorProposal {
    //         proposal_hash: env::keccak256(answer.as_bytes()),
    //         is_revealed: false,
    //         miner_addresses: Vec::new(),
    //     };
    //     complete_request
    //         .validators_proposals
    //         .insert(env::predecessor_account_id(), proposal);
    // }

    // pub fn reveal_by_miner(&mut self, miner: AccountId, request_id: u64, answer: String) {
    //     let request_exist = self.get_request_by_id(request_id);
    //     require!(request_exist.is_some());

    //     let miner_to_reveal = self.get_register_miner(miner.clone());
    //     require!(miner_to_reveal.is_some());

    //     let complete_request = match self.get_request_by_id(request_id) {
    //         Some(request) => request,
    //         None => panic!("Request not found"),
    //     };

    //     require!(
    //         env::epoch_height() > complete_request.commit_miner_deadline,
    //         "commit time"
    //     );
    //     require!(
    //         env::epoch_height() < complete_request.reveal_miner_deadline,
    //         "No time to reveal"
    //     );

    //     let save_proposal = match complete_request.miners_proposals.get_mut(&miner) {
    //         Some(proposal) => proposal,
    //         None => panic!("proposal not found"),
    //     };

    //     require!(
    //         save_proposal.is_revealed == false,
    //         "Proposal already reveal"
    //     );

    //     let answer_to_verify = env::keccak256(answer.as_bytes());
    //     require!(
    //         save_proposal.proposal_hash == answer_to_verify,
    //         "Answer don't match"
    //     );

    //     save_proposal.is_revealed = true;
    // }

    // pub fn reveal_by_validator(&mut self, validator: AccountId, request_id: u64, answer: String) {
    //     let request_exist = self.get_request_by_id(request_id);
    //     require!(request_exist.is_some());

    //     let validator_to_reveal = self.get_register_miner(validator.clone());
    //     require!(validator_to_reveal.is_some());

    //     let complete_request = match self.get_request_by_id(request_id) {
    //         Some(request) => request,
    //         None => panic!("Request not found"),
    //     };

    //     require!(
    //         env::epoch_height() > complete_request.commit_validator_deadline,
    //         "commit time"
    //     );
    //     require!(
    //         env::epoch_height() < complete_request.reveal_validator_deadline,
    //         "No time to reveal"
    //     );

    //     let save_proposal = match complete_request.validators_proposals.get_mut(&validator) {
    //         Some(proposal) => proposal,
    //         None => panic!("proposal not found"),
    //     };

    //     require!(
    //         save_proposal.is_revealed == false,
    //         "Proposal already reveal"
    //     );

    //     let answer_to_verify = env::keccak256(answer.as_bytes());
    //     require!(
    //         save_proposal.proposal_hash == answer_to_verify,
    //         "Answer don't match"
    //     );

    //     save_proposal.is_revealed = true;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::panic;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::testing_env;

    #[test]
    fn test_register_miner() {
        let mut contract = Contract::new();
        let miner_1: AccountId = "hassel.near".parse().unwrap();
        let miner_2: AccountId = "edson.near".parse().unwrap();

        let result_1 = contract.register_miner(miner_1.clone());
        let result_2 = contract.register_miner(miner_2.clone());

        assert_eq!(result_1, RegisterMinerResult::Success);
        assert_eq!(result_2, RegisterMinerResult::Success);

        assert!(contract.get_register_miner(miner_1).is_some());
        assert!(contract.get_register_miner(miner_2).is_some());

        // Assert logs
        let logs = get_logs();

        assert_eq!(logs.len(), 2);

        assert_eq!(logs[0], "Registered new miner: hassel.near");
        assert_eq!(logs[1], "Registered new miner: edson.near");
    }

    #[test]
    fn test_register_miner_when_is_registered_returns_already_registered() {
        let mut contract = Contract::new();
        let miner_1: AccountId = "hassel.near".parse().unwrap();

        contract.register_miner(miner_1.clone());

        let result = contract.register_miner(miner_1.clone());

        assert_eq!(result, RegisterMinerResult::AlreadyRegistered);

        // Assert logs
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
        let mut contract = Contract::new();
        let miner: AccountId = "hassel.near".parse().unwrap();

        contract.register_miner(miner.clone());

        assert!(contract.get_register_miner(miner).is_some());
    }

    #[test]
    fn test_get_register_miner_when_not_registered() {
        let mut contract = Contract::new();
        let miner: AccountId = "hassel.near".parse().unwrap();

        assert!(contract.get_register_miner(miner).is_none());
    }

    #[test]
    fn test_register_validator() {
        let mut contract = Contract::new();
        let validator_1: AccountId = "hassel.near".parse().unwrap();
        let validator_2: AccountId = "edson.near".parse().unwrap();

        let result1 = contract.register_validator(validator_1.clone());
        let result2 = contract.register_validator(validator_2.clone());

        assert_eq!(result1, RegisterValidatorResult::Success);
        assert_eq!(result2, RegisterValidatorResult::Success);

        assert!(contract.get_register_validator(validator_1).is_some());
        assert!(contract.get_register_validator(validator_2).is_some());

        // Assert logs
        let logs = get_logs();

        assert_eq!(logs.len(), 2);

        assert_eq!(logs[0], "Registered new validator: hassel.near");
        assert_eq!(logs[1], "Registered new validator: edson.near");
    }

    #[test]
    fn test_register_validator_when_is_registered_returns_already_registered() {
        let mut contract = Contract::new();
        let validator_1: AccountId = "hassel.near".parse().unwrap();

        contract.register_validator(validator_1.clone());

        let result = contract.register_validator(validator_1.clone());

        assert_eq!(result, RegisterValidatorResult::AlreadyRegistered);

        // Assert logs
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
        let mut contract = Contract::new();
        let validator: AccountId = "hassel.near".parse().unwrap();

        contract.register_validator(validator.clone());

        assert!(contract.get_register_validator(validator).is_some());
    }

    #[test]
    fn test_get_register_validator_when_not_registered() {
        let mut contract = Contract::new();
        let validator: AccountId = "hassel.near".parse().unwrap();

        assert!(contract.get_register_validator(validator).is_none());
    }

    #[test]
    fn test_request_governance_decision(){
        let mut contract = Contract::new();
        let request_id_1 = 100;
        let request_id_2 = 101;

        let result_1 = contract.request_governance_decision(request_id_1.clone());
        let result_2 = contract.request_governance_decision(request_id_2.clone());
        
        assert_eq!(result_1, RegisterRequestResult::Success);
        assert_eq!(result_2, RegisterRequestResult::Success);
        
        assert!(contract.get_request_by_id(request_id_1).is_some());
        assert!(contract.get_request_by_id(request_id_2).is_some());

        let logs = get_logs();

        assert_eq!(logs.len(), 2);

        assert_eq!(logs[0], "Registered new request: 100");
        assert_eq!(logs[1], "Registered new request: 101");
    }

    #[test]
    fn test_request_governance_decision_when_is_registered_returns_already_registered(){
        let mut contract = Contract::new();
        let request_id = 100;

        contract.request_governance_decision(request_id);

        let result = contract.request_governance_decision(request_id);

        assert_eq!(result, RegisterRequestResult::AlreadyRegistered);

        let logs = get_logs();
        assert_eq!(logs.len(), 2);
        
        assert_eq!(logs[0], "Registered new request: 100");
        assert_eq!(logs[1], "Attempted to register an already registered request: 100");
    }

    #[test]
    fn test_get_request_by_id(){
        let mut contract = Contract::new();
        let request_id = 100;

        contract.request_governance_decision(request_id);
        
        assert!(contract.get_request_by_id(request_id).is_some());
    }

    #[test]
    fn test_get_request_by_id_when_not_registered(){
        let mut contract = Contract::new();
        let request_id = 100;

        assert!(contract.get_request_by_id(request_id).is_none());
    }
}
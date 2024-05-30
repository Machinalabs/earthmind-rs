pub mod models;
use crate::models::models::*;
use hex;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::{LookupMap, Vector};
use near_sdk::{env, log, near_bindgen, require, AccountId, PanicOnDefault};

type Hash = String;
const two_minutes : u64 = 2 * 60 * 1_000_000_000; 
const COMMIT_MINER_DURATION: u64 = two_minutes; // 2 minutes in nanoseconds
const REVEAL_MINER_DURATION: u64 = two_minutes; // 2 minutes in nanoseconds
const COMMIT_VALIDATOR_DURATION: u64 = two_minutes; // 2 minutes in nanoseconds
const REVEAL_VALIDATOR_DURATION: u64 = two_minutes; // 2 minutes in nanoseconds

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
            start_time: env::block_timestamp(),
            miners_proposals: LookupMap::new(b"m"),
            validators_proposals: LookupMap::new(b"v"),
        };

        self.requests.push(new_request);

        log!("Registered new request: {}", new_request_id_hex);
        RegisterRequestResult::Success
    }

    pub fn get_request_by_id(&mut self, request_id: String) -> Option<&mut Request> {
        for request in self.requests.iter_mut() {
            if request.request_id == request_id {
                return Some(request);
            }
        }
        None
    }

    fn get_stage(start_time : u64) -> RequestState {

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


    pub fn hash_miner_answer(self, request_id: String, answer: bool, message: String) -> Hash {
        let miner = env::predecessor_account_id();

        let concatenated_answer = format!("{}{}{}{}",request_id, miner, answer, message);
        let value = env::keccak256(concatenated_answer.as_bytes());
        let hash_answer = hex::encode(value);

        return hash_answer;
    }

    pub fn commit_by_miner(&mut self, request_id: String, answer : Hash) -> CommitMinerResult {

        let miner = env::predecessor_account_id();

        if self.get_register_miner(miner.clone()).is_none() {
            log!("Miner not registered: {}", miner);
            return CommitMinerResult::Fail;
        }

        if self.get_request_by_id(request_id.clone()).is_none() {
            log!("Request is not registered: {}", request_id);
            return CommitMinerResult::Fail;
        }

        let complete_request: &mut Request = match self.get_request_by_id(request_id.clone()) {
            Some(request) => request,
            None => panic!("Request not found"),
        };

         assert_eq!(Self::get_stage(complete_request.start_time), RequestState::CommitMiners, "Not at CommitMiners stage");

        if complete_request.miners_proposals.get(&miner).is_some() {
            log!("This miner have a commit answer: {}", miner);
            return CommitMinerResult::Fail;
        }

        let proposal = MinerProposal {
            proposal_hash: answer,
            answer : false, 
            is_revealed: false,
        };

        complete_request.miners_proposals.insert(miner, proposal);

        log!("Miner proposal registered successfully");
        return CommitMinerResult::Success;

    }

    pub fn hash_validator_answer(self, request_id: String, answer: Vec<AccountId>, message: String) -> Hash {
        let validator = env::predecessor_account_id();

        require!(answer.len() == 10, "Invalid answer");

        let mut concatenated_answer : Vec<u8> = Vec::new();  

        concatenated_answer.extend_from_slice(request_id.as_bytes());
        concatenated_answer.extend_from_slice(validator.as_bytes());

        let value : Vec<u8> = answer.iter().flat_map(|id| id.as_bytes()).copied().collect();
        concatenated_answer.extend_from_slice(&value);
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

        assert_eq!(Self::get_stage(complete_request.start_time), RequestState::CommitValidators, "Not at CommitValidator stage");

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
            log!("Miner not registered: {}", miner);
            return RevealMinerResult::Fail;
        }

        if self.get_request_by_id(request_id.clone()).is_none() {
            log!("Request is not registered: {}", request_id);
            return RevealMinerResult::Fail;
        }

        let complete_request = match self.get_request_by_id(request_id.clone()) {
            Some(request) => request,
            None => panic!("Request not found"),
        };

        assert_eq!(Self::get_stage(complete_request.start_time), RequestState::RevealMiners, "Not at RevealMiners stage");

        let save_proposal = match complete_request.miners_proposals.get_mut(&miner) {
            Some(proposal) => proposal,
            None => panic!("proposal not found"),
        };

        if save_proposal.is_revealed == true {
            log!("Proposal already revealed");
            return RevealMinerResult::Fail;
        }

        let concatenated_answer = format!("{}{}{}{}",request_id, miner, answer, message);
        let hash_value = env::keccak256(concatenated_answer.as_bytes());
        let answer_to_verify = hex::encode(hash_value);

        if save_proposal.proposal_hash != answer_to_verify {
            log!("Answer don't match");
            return RevealMinerResult::Fail;
        }

        save_proposal.answer = answer;
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

        let complete_request = match self.get_request_by_id(request_id.clone()) {
            Some(request) => request,
            None => panic!("Request not found"),
        };

        assert_eq!(Self::get_stage(complete_request.start_time), RequestState::RevealValidators, "Not at RevealValidators stage");


        let save_proposal = match complete_request.validators_proposals.get_mut(&validator) {
            Some(proposal) => proposal,
            None => panic!("proposal not found"),
        };

        if save_proposal.is_revealed == true {
            log!("Proposal already revealed");
            return RevealValidatorResult::Fail;
        }

        if answer.len() != 10 {
            log!("Invalid answer");
            return RevealValidatorResult::Fail;
        }

        let mut concatenated_answer : Vec<u8> = Vec::new();  

        concatenated_answer.extend_from_slice(request_id.as_bytes());
        concatenated_answer.extend_from_slice(validator.as_bytes());

        let value : Vec<u8> = answer.iter().flat_map(|id| id.as_bytes()).copied().collect();
        concatenated_answer.extend_from_slice(&value);
        concatenated_answer.extend_from_slice(message.as_bytes());

        let value = env::keccak256(&concatenated_answer);
        let hash_answer = hex::encode(value);

        if save_proposal.proposal_hash != hash_answer {
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

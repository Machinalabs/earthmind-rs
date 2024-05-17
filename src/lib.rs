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

    // pub fn register_validator(&mut self, participant_id: AccountId) {
    //     let validator_to_register = self.get_register_validator(participant_id.clone());
    //     require!(validator_to_register.is_none());

    //     let miner_to_register = self.get_register_miner(participant_id.clone());
    //     require!(miner_to_register.is_none());

    //     self.validators.push(participant_id);
    // }

    // pub fn get_register_validator(&mut self, participant_id: AccountId) -> Option<&AccountId> {
    //     for validator in self.validators.iter() {
    //         if *validator == participant_id {
    //             return Some(validator);
    //         }
    //     }
    //     None
    // }

    // pub fn request_governance_decision(&mut self, request_id: u64) {
    //     let new_request = Request {
    //         sender: env::predecessor_account_id(),
    //         request_id: request_id,
    //         start_time: env::epoch_height(),
    //         commit_miner_deadline: env::epoch_height() + COMMIT_MINER_DURATION,
    //         reveal_miner_deadline: env::epoch_height()
    //             + COMMIT_MINER_DURATION
    //             + REVEAL_MINER_DURATION,
    //         commit_validator_deadline: env::epoch_height()
    //             + COMMIT_MINER_DURATION
    //             + REVEAL_MINER_DURATION
    //             + COMMIT_VALIDATOR_DURATION,
    //         reveal_validator_deadline: env::epoch_height()
    //             + COMMIT_MINER_DURATION
    //             + REVEAL_MINER_DURATION
    //             + COMMIT_VALIDATOR_DURATION
    //             + REVEAL_VALIDATOR_DURATION,
    //         miners_proposals: LookupMap::new(b"m"),
    //         validators_proposals: LookupMap::new(b"v"),
    //     };
    //     self.requests.push(new_request);
    // }

    // fn get_request_by_id(&mut self, request_id: u64) -> Option<&mut Request> {
    //     for request in &mut self.requests {
    //         if request.request_id == request_id {
    //             return Some(request);
    //         }
    //     }
    //     None
    // }

    // pub fn commit_by_miner(&mut self, miner: AccountId, request_id: u64, answer: String) {
    //     let miner_to_commit = self.get_register_miner(miner);
    //     require!(miner_to_commit.is_some());

    //     let request_exist = self.get_request_by_id(request_id);
    //     require!(request_exist.is_some());

    //     let complete_request = match self.get_request_by_id(request_id) {
    //         Some(request) => request,
    //         None => panic!("Request not found"),
    //     };

    //     require!(
    //         env::epoch_height() < complete_request.commit_miner_deadline,
    //         "No time to commit"
    //     );

    //     let proposal = MinerProposal {
    //         proposal_hash: env::keccak256(answer.as_bytes()),
    //         is_revealed: false,
    //     };

    //     complete_request
    //         .miners_proposals
    //         .insert(env::predecessor_account_id(), proposal);
    // }

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

        let result1 = contract.register_miner(miner_1.clone());
        let result2 = contract.register_miner(miner_2.clone());

        assert_eq!(result1, RegisterMinerResult::Success);
        assert_eq!(result2, RegisterMinerResult::Success);

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

    // #[test]
    // fn test_register_validator() {
    //     let mut contract = Contract::new();
    //     let participant_1: AccountId = "alice.near".parse().unwrap();
    //     let participant_2: AccountId = "bob.near".parse().unwrap();

    //     contract.validators.push(participant_1.clone());
    //     contract.validators.push(participant_2.clone());

    //     assert!(contract.validators[0] == participant_1);
    //     assert!(contract.validators[1] == participant_2);
    // }

    // #[test]
    // fn test_get_register_validator() {
    //     let mut contract = Contract::new();
    //     let participant_1: AccountId = "anne.near".parse().unwrap();
    //     let participant_2: AccountId = "bob.near".parse().unwrap();

    //     contract.validators.push(participant_1.clone());
    //     contract.validators.push(participant_2.clone());

    //     let register_validator = contract.get_register_validator(participant_1.clone());

    //     let validator = match register_validator {
    //         Some(register) => register,
    //         None => panic!("Validator not register"),
    //     };

    //     assert_eq!(*validator, participant_1);

    //     let register_validator = contract.get_register_validator(participant_2.clone());

    //     let validator = match register_validator {
    //         Some(register) => register,
    //         None => panic!("Validator not register"),
    //     };

    //     assert_eq!(*validator, participant_2);
    // }

    // #[test]
    // fn test_get_request_by_id() {
    //     let request = Request {
    //         sender: "anne.near".parse().unwrap(),
    //         request_id: 100,
    //         start_time: env::epoch_height(),
    //         commit_miner_deadline: env::epoch_height() + COMMIT_MINER_DURATION,
    //         reveal_miner_deadline: env::epoch_height()
    //             + COMMIT_MINER_DURATION
    //             + REVEAL_MINER_DURATION,
    //         commit_validator_deadline: env::epoch_height()
    //             + COMMIT_MINER_DURATION
    //             + REVEAL_MINER_DURATION
    //             + COMMIT_VALIDATOR_DURATION,
    //         reveal_validator_deadline: env::epoch_height()
    //             + COMMIT_MINER_DURATION
    //             + REVEAL_MINER_DURATION
    //             + COMMIT_VALIDATOR_DURATION
    //             + REVEAL_VALIDATOR_DURATION,
    //         miners_proposals: LookupMap::new(b"m"),
    //         validators_proposals: LookupMap::new(b"v"),
    //     };

    //     let mut contract = Contract::new();
    //     contract.requests.push(request);

    //     let register_request = contract.get_request_by_id(100);

    //     let request = match register_request {
    //         Some(register) => register,
    //         None => panic!("request not register"),
    //     };

    //     assert_eq!(request.request_id, 100);
    // }

    // #[test]
    // fn test_request_governance_decision() {
    //     //TODO: verificar si esta prueba es correcto el planteamiento
    //     let mut contract = Contract::new();
    //     contract.request_governance_decision(100);

    //     let register_request = contract.get_request_by_id(100);

    //     let request = match register_request {
    //         Some(register) => register,
    //         None => panic!("request not register"),
    //     };

    //     assert_eq!(request.request_id, 100);
    // }

    //TODO: Como manejo los tiempos "epoch" para que pueda hacer los siguientes Test
    // #[test]
    // fn test_commit_by_miner() {}

    // #[test]
    // fn test_commit_by_validator() {}

    // #[test]
    // fn test_reveal_by_miner() {}

    // #[test]
    // fn test_reveal_by_validator() {}
}

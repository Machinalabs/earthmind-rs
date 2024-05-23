HELPER function to hash your value 

Miner use a bool and a message 
let concatenated_answer = format!("{}{}", answer, message);

let answer = true;
        let message = "It's a cool NFT".to_string();


Validator is a vector and a message 
answer: Vec<AccountId>, message: String

if answer.len() != 10 {
            log!("Vote for 10 miners");
            return CommitValidatorResult::Fail;
        }
let mut concatenated_answer: Vec<u8> = answer.iter().flat_map(|id| id.as_bytes()).copied().collect();
        concatenated_answer.extend_from_slice(message.as_bytes());


# EarthMind RS





## Pre Requisites

- [Just]
- [Rust]
- [Near]
- [`cargo-near`]

## Getting Started

```bash
$ git clone https://github.com/hasselalcala/consensus_contract.git

$ just test  # run tests

$ just build # build project locally
```

## Deployment

To deploy manually, install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
# Create a new account
cargo near create-dev-account

# Deploy the contract on it
cargo near deploy <account-id>
```

## Useful Links

- [Just]
- [Rust]
- [Near]
- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://docs.near.org/tools/near-cli) - Interact with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)

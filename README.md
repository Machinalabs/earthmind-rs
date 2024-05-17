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

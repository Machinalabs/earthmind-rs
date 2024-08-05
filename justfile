set dotenv-load

test:
    echo "Running tests"
    cargo test --lib

build:
    echo "Building"
    cargo near build

clippy:
    echo "Running clippy"
    cargo clippy --all-targets --all-features -- -D warnings -D clippy::all -D clippy::nursery

deploy:
    echo "deploying contract"
    near deploy "$CONTRACT_ACCOUNT" "$PATH_TO_CONTRACT"

init:
    echo "init contract"
    near call "$CONTRACT_ACCOUNT" new '{}' --accountId "$PROTOCOL_ACCOUNT"

register:   
    echo "Register Protocol"
    near call "$CONTRACT_ACCOUNT" register_protocol '{"culture":"governance!", "modules":["TextPrompting"]}' --accountId "$PROTOCOL_ACCOUNT" --amount 5


request:
    echo "Sending request"
    near call "$CONTRACT_ACCOUNT" request_governance_decision '{"message":"PROVING?????"}' --accountId "$PROTOCOL_ACCOUNT"

register_miner:
    echo "register miner"
    near call "$CONTRACT_ACCOUNT" register_miner --accountId "$MINER_ACCOUNT" --amount 1

register_validator:
    echo "register validator"
    near call "$CONTRACT_ACCOUNT" register_validator --accountId "$VALIDATOR_ACCOUNT" --amount 10




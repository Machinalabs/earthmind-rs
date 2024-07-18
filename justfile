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
    near deploy earthmindprotocol.testnet /Users/hasselalcala/Documents/near_contracts/machinalabs_earthmind/earthmind-rs/target/near/earthmind_rs.wasm
test:
    echo "Running tests"
    cargo test --lib

build:
    echo "Building"
    cargo near build

clippy:
    echo "Running clippy"
    cargo clippy --all-targets --all-features -- -D warnings
    cargo clippy --all-targets -- -D clippy::all -D clippy::nursery
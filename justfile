set shell := ["bash", "-euc"]

check:
    cargo fmt --check --all
    cargo clippy --all-targets --all-features -- -Dwarnings

test:
    cargo test --locked --workspace --all-targets
    cargo test --doc --no-default-features --features async

build-docs:
    cargo doc --no-deps

check-strict DIR:
    pushd {{ DIR }} && cargo clippy --all-targets --all-features -- -D clippy::pedantic -D clippy::nursery

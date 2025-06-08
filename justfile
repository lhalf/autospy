set shell := ["bash", "-euc"]

check:
    cargo fmt --check --all
    cargo clippy --all-targets -- -Dwarnings

test:
    cargo test --locked --workspace --all-targets
    cargo test --doc --no-default-features --features async

build-docs:
    cargo doc --no-deps
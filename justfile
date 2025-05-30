set shell := ["bash", "-euc"]

test:
    cargo fmt --check --all
    cargo clippy --all-targets -- -Dwarnings
    cargo test --locked --workspace --no-default-features
    cargo test --example axum --example rocket

build-docs:
    cargo doc --no-deps
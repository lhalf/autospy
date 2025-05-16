set shell := ["bash", "-euc"]

test:
    cargo fmt --check --all
    cargo clippy --all-targets -- -Dwarnings
    cargo test --locked --workspace

build-docs:
    cargo doc --no-deps
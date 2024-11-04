test:
    cargo nextest run
    cargo clippy 
    cargo fmt --check
    cargo doc --workspace --all-features --no-deps --document-private-items

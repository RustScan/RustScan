test:
    cargo nextest run
    cargo clippy 
    cargo fmt --check
    cargo doc --workspace --all-features --no-deps --document-private-items

fix:
    cargo fmt
    cargo clippy --fix

# release only runs if test runs
release: test
    git checkout master
    git pull
    cargo generate-lockfile
    cargo publish 
    git push

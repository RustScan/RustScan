install:
	cargo install cross

build:
	make build-linux
	make build-mac

	make shasum

build-linux:
	@echo 'Building for Linux... 🐧'
	cross build --release --target=x86_64-unknown-linux-musl
	mkdir -p target/release-archives && tar -czf target/release-archives/rustscan-linux.tar.gz target/x86_64-unknown-linux-musl/release/rustscan 

build-mac:
	@echo 'Building for MacOS... 🍏'
	cross build --release --target=x86_64-apple-darwin
	mkdir -p target/release-archives && tar -czf target/release-archives/rustscan-mac.tar.gz target/x86_64-apple-darwin/release/rustscan

shasum:
	shasum -a 256 target/release-archives/rustscan-*.tar.gz

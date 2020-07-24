#!/bin/bash

cd /sourcefiles

#amd64
cargo build --release

#arm64
rustup target add arm-unknown-linux-gnueabihf
cargo build --release --target=arm-unknown-linux-gnueabihf

#i386
rustup target add i686-unknown-linux-gnu
cargo build --release --target=i686-unknown-linux-gnu

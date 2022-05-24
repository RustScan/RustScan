#!/bin/bash

cd /RustScan
git pull --force

#amd64
cargo deb

#arm64
rustup target add arm-unknown-linux-gnueabihf
cargo deb --target=arm-unknown-linux-gnueabihf

#armv7
rustup target add armv7-unknown-linux-gnueabihf
cargo deb --target=armv7-unknown-linux-gnueabihf

#i386
rustup target add i686-unknown-linux-gnu
cargo deb --target=i686-unknown-linux-gnu

find target/ -name \*.deb -exec cp {} /debs \;

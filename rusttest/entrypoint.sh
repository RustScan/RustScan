#!/bin/bash

cd /RustScan
git pull --force

#amd64
cargo deb

#arm64
rustup target add arm-unknown-linux-gnueabihf
git clone --depth=1 https://github.com/raspberrypi/tools.git /tmp/tools
export PATH=/tmp/tools/arm-bcm2708/arm-linux-gnueabihf/bin:$PATH
export CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc
cargo deb --target=arm-unknown-linux-gnueabihf

#!/bin/sh
set -e

cargo clean
cargo test

cargo lipo --release
# cargo build --target aarch64-linux-android --release
# cargo build --target armv7-linux-androideabi --release
# cargo build --target i686-linux-android --release
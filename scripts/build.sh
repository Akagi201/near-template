#!/bin/sh

set -e

cargo build --workspace --target wasm32-unknown-unknown --release
cp ./target/wasm32-unknown-unknown/release/*.wasm ./res/

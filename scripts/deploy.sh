#!/bin/sh

set -e

near deploy stdcon.$MASTER_ACCOUNT --wasmFile res/stdcon.wasm
near deploy nostdcon.$MASTER_ACCOUNT --wasmFile res/nostdcon.wasm

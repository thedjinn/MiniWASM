#!/bin/bash

set -e

TARGET_PATH="target/wasm32-unknown-unknown/release/miniwasm.wasm"

# Build the wasm32 release target
cargo build --target wasm32-unknown-unknown --release --lib

# Run the wasm optimizer
wasm-opt ${TARGET_PATH} -o ${TARGET_PATH} -Oz --strip-debug --strip-producers --vacuum

# Copy the wasm file to the root directory so it can be served.
cp ${TARGET_PATH} ./miniwasm.wasm

#!/usr/bin/env bash
set -e

echo "Running rust tests"
# export RUST_TEST_THREADS=1
cargo test -- --nocapture

# examples
cargo run --example simple

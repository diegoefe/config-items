@echo off

echo "Running rust tests"
@REM set RUST_TEST_THREADS=1
cargo test -- --nocapture

rem examples
cargo run --example simple

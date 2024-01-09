@echo off

echo "Running rust tests"
@REM set RUST_TEST_THREADS=1
cargo test -- --nocapture

rem examples
@REM cargo run --example myapp
cargo run --example myapp -- -c "./examples/myapp.yaml"

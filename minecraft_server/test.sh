# export RUST_BACKTRACE=1
RUSTFLAGS="-C symbol-mangling-version=v0"
export MY_IP="158.180.88.171:25565"
cargo test client::test_client::test_client -- --exact --ignored --nocapture


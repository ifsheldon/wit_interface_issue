cd ../implementation
echo "Building implementation..."
cargo build --target wasm32-wasip2 --release
cd -
cargo run
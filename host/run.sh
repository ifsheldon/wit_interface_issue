cd ../implementation
echo "Building implementation..."
cargo clean
RUSTFLAGS="-C panic=abort" cargo build --target wasm32-wasip2 --release -Zbuild-std -Zbuild-std-features=panic_immediate_abort
if [ $? -ne 0 ]; then
    exit 1
fi
cd -
cargo run
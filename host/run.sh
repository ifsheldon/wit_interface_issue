cd ../implementation
echo "Building implementation..."
#cargo clean
cargo +nightly build -Zbuild-std=std,panic_abort -Zbuild-std-features=panic_immediate_abort --target wasm32-wasip2 --release
if [ $? -ne 0 ]; then
    exit 1
fi
cd -
cargo run
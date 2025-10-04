#!/bin/bash

set -e

echo "Building library..."
cargo build --release

echo "Generating Kotlin bindings..."
cargo run --features=uniffi/cli --bin uniffi-bindgen generate \
    --library target/release/libelectrum_client_rs.so \
    --language kotlin \
    --out-dir android/src/main/java

echo "Generating Swift bindings..."
cargo run --features=uniffi/cli --bin uniffi-bindgen generate \
    --library target/release/libelectrum_client_rs.dylib \
    --language swift \
    --out-dir ios/

echo "Bindings generated successfully!"

#!/bin/bash

set -e

echo "Building for iOS..."

# iOS targets
IOS_TARGETS=("aarch64-apple-ios" "aarch64-apple-ios-sim" "x86_64-apple-ios")

# Ensure targets are installed
for target in "${IOS_TARGETS[@]}"; do
    rustup target add "$target" 2>/dev/null || true
done

# Build for each target
for target in "${IOS_TARGETS[@]}"; do
    echo "Building for $target..."
    cargo build --release --target "$target"
done

# Create universal library for simulator (combining arm64 and x86_64 simulators)
echo "Creating universal library..."
mkdir -p target/universal/release

lipo -create \
    target/aarch64-apple-ios-sim/release/libelectrum_client_rs.a \
    target/x86_64-apple-ios/release/libelectrum_client_rs.a \
    -output target/universal/release/libelectrum_client_rs_sim.a

# Copy device library
cp target/aarch64-apple-ios/release/libelectrum_client_rs.a target/universal/release/libelectrum_client_rs.a

echo "iOS build complete!"
echo "Device library: target/universal/release/libelectrum_client_rs.a"
echo "Simulator library: target/universal/release/libelectrum_client_rs_sim.a"

#!/bin/bash

set -e

echo "Building for Android..."

# Android targets
TARGETS=("aarch64-linux-android" "armv7-linux-androideabi" "i686-linux-android" "x86_64-linux-android")
OUTPUT_DIRS=("arm64-v8a" "armeabi-v7a" "x86" "x86_64")

# Ensure targets are installed
for target in "${TARGETS[@]}"; do
    rustup target add "$target" 2>/dev/null || true
done

# Build for each target using cargo-ndk
for i in "${!TARGETS[@]}"; do
    target="${TARGETS[$i]}"
    output_dir="${OUTPUT_DIRS[$i]}"
    
    echo "Building for $target..."
    cargo ndk --target $target --platform 21 -- build --release
    
    # Copy to jniLibs
    mkdir -p "android/src/main/jniLibs/$output_dir"
    cp "target/$target/release/libelectrum_client_rs.so" "android/src/main/jniLibs/$output_dir/"
done

echo "Android build complete!"

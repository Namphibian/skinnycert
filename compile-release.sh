#!/bin/bash
set -e

echo "🚀 Starting Rust API build in release mode..."

TARGET="x86_64-unknown-linux-gnu"
echo "🖥️  Build target: $TARGET"
echo "⚙️  Applying optimizations: opt-level=3, target-cpu=native, LTO"
# Unset RUSTFLAGS to avoid embed-bitcode vs LTO conflict
unset RUSTFLAGS
# Set Rust build flags for release optimizations
export RUSTFLAGS="-C opt-level=3 -C target-cpu=native"
# Build
cargo build --release --target "$TARGET"
echo "✅ Build completed successfully!"
echo "📦 Binary location: target/$TARGET/release/$(basename $(pwd))"

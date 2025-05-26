#!/bin/bash
set -euo pipefail

LIB_NAME=mrz_parser
CRATE_DIR=.
OUT_DIR=Sources/MRZParserFFI
UNIVERSAL_DIR=target/universal
XCFRAMEWORK_NAME=MRZParserFFI.xcframework

# Step 1: Build for macOS (universal: arm64 + x86_64)
echo "📦 Building macOS libraries..."
cargo build --release --features uniffi --target x86_64-apple-darwin
cargo build --release --features uniffi --target aarch64-apple-darwin

mkdir -p "$UNIVERSAL_DIR/macos"
lipo -create \
  target/x86_64-apple-darwin/release/lib${LIB_NAME}.dylib \
  target/aarch64-apple-darwin/release/lib${LIB_NAME}.dylib \
  -output "$UNIVERSAL_DIR/macos/lib${LIB_NAME}.dylib"

# Step 2: Build for iOS (simulator + device)
echo "📱 Building iOS libraries..."
cargo build --release --features uniffi --target aarch64-apple-ios
cargo build --release --features uniffi --target x86_64-apple-ios
cargo build --release --features uniffi --target aarch64-apple-ios-sim

# 🧬 Generating Swift bindings via UniFFI...
echo "🧬 Generating Swift bindings via UniFFI..."
rm -rf "$OUT_DIR"

uniffi-bindgen generate \
  --library "target/aarch64-apple-darwin/release/lib${LIB_NAME}.dylib" \
  --language swift \
  --out-dir uniffi-gen \
  --no-format

mkdir -p "$OUT_DIR"
cp uniffi-gen/mrz_parser.swift "$OUT_DIR/mrz_parser.swift"
mkdir -p "$OUT_DIR/include"
cp uniffi-gen/mrz_parserFFI.h "$OUT_DIR/include/mrz_parserFFI.h"
cp uniffi-gen/mrz_parserFFI.modulemap "$OUT_DIR/include/module.modulemap"

mkdir -p "$UNIVERSAL_DIR/ios/device"
cp target/aarch64-apple-ios/release/lib${LIB_NAME}.dylib "$UNIVERSAL_DIR/ios/device/lib${LIB_NAME}.dylib"

mkdir -p "$UNIVERSAL_DIR/ios/simulator"
lipo -create \
  target/x86_64-apple-ios/release/lib${LIB_NAME}.dylib \
  target/aarch64-apple-ios-sim/release/lib${LIB_NAME}.dylib \
  -output "$UNIVERSAL_DIR/ios/simulator/lib${LIB_NAME}.dylib"

# Step 4: Create .xcframework
echo "📦 Creating .xcframework..."
rm -rf "$XCFRAMEWORK_NAME"
xcodebuild -create-xcframework \
  -library "$UNIVERSAL_DIR/macos/lib${LIB_NAME}.dylib" \
  -headers "Sources/MRZParserFFI/include" \
  -library "$UNIVERSAL_DIR/ios/device/lib${LIB_NAME}.dylib" \
  -headers "Sources/MRZParserFFI/include" \
  -library "$UNIVERSAL_DIR/ios/simulator/lib${LIB_NAME}.dylib" \
  -headers "Sources/MRZParserFFI/include" \
  -output "$XCFRAMEWORK_NAME"

echo "✅ Done! Output: $XCFRAMEWORK_NAME"

# Step 5: Copy generated bindings into swift package layout
echo "📦 Updating Swift package directory..."

SWIFT_PKG_DIR="swift"
SWIFT_PKG_SRC="$SWIFT_PKG_DIR/Sources/MRZParserFFI"

mkdir -p "$SWIFT_PKG_SRC/include"

cp uniffi-gen/mrz_parser.swift "$SWIFT_PKG_SRC/mrz_parser.swift"
cp uniffi-gen/mrz_parserFFI.h "$SWIFT_PKG_SRC/include/mrz_parserFFI.h"

# Copy the built xcframework for consumers of the Swift package
cp -R "$XCFRAMEWORK_NAME" "$SWIFT_PKG_DIR/"
echo "✅ Done! Output: $SWIFT_PKG_DIR"

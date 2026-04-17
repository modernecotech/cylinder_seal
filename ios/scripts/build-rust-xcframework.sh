#!/usr/bin/env bash
# Builds `cs-mobile-core` for iOS device + simulator architectures, packages
# them into an xcframework, and drops the UniFFI-generated Swift bindings
# next to the framework. Run this from the repo root before opening the
# Xcode project:
#
#     ./ios/scripts/build-rust-xcframework.sh
#
# Prerequisites:
#   - Xcode command line tools
#   - Rust with `aarch64-apple-ios` + `aarch64-apple-ios-sim` + `x86_64-apple-ios` targets
#     (`rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios`)
#   - `cargo install cargo-swift` OR the manual uniffi-bindgen-swift step below

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
CRATE_DIR="$REPO_ROOT/crates/cs-mobile-core"
OUT_DIR="$REPO_ROOT/ios/CylinderSealCore"
FRAMEWORK_NAME="CylinderSealCore"

echo "--> building cs-mobile-core for iOS targets"
cd "$CRATE_DIR"
cargo build --release --target aarch64-apple-ios
cargo build --release --target aarch64-apple-ios-sim
cargo build --release --target x86_64-apple-ios

TARGET_DIR="$REPO_ROOT/target"
DEVICE_LIB="$TARGET_DIR/aarch64-apple-ios/release/libcs_mobile_core.a"
SIM_ARM_LIB="$TARGET_DIR/aarch64-apple-ios-sim/release/libcs_mobile_core.a"
SIM_X86_LIB="$TARGET_DIR/x86_64-apple-ios/release/libcs_mobile_core.a"

echo "--> combining simulator slices"
SIM_COMBINED="$TARGET_DIR/libcs_mobile_core_sim.a"
lipo -create "$SIM_ARM_LIB" "$SIM_X86_LIB" -output "$SIM_COMBINED"

mkdir -p "$OUT_DIR"
rm -rf "$OUT_DIR/$FRAMEWORK_NAME.xcframework"

echo "--> generating UniFFI Swift bindings"
mkdir -p "$OUT_DIR/bindings"
# cargo-swift writes the bindings alongside the xcframework; adjust paths
# if you use a different bindgen installation.
if command -v uniffi-bindgen-swift >/dev/null 2>&1; then
    uniffi-bindgen-swift \
        "$CRATE_DIR/src/cs_mobile_core.udl" \
        --out-dir "$OUT_DIR/bindings"
else
    echo "warning: uniffi-bindgen-swift not found — install with 'cargo install uniffi-bindgen-swift'"
    echo "falling back to stub bindings directory (xcframework link only)"
fi

echo "--> building xcframework"
xcodebuild -create-xcframework \
    -library "$DEVICE_LIB" \
    -headers "$OUT_DIR/bindings" \
    -library "$SIM_COMBINED" \
    -headers "$OUT_DIR/bindings" \
    -output "$OUT_DIR/$FRAMEWORK_NAME.xcframework"

echo "✓ xcframework at $OUT_DIR/$FRAMEWORK_NAME.xcframework"
echo "✓ Swift bindings at $OUT_DIR/bindings/"
echo "now run: (cd ios && xcodegen generate && open CylinderSeal.xcodeproj)"

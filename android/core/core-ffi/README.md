# core-ffi

Wraps the UniFFI-generated Kotlin bindings for `cs-mobile-core`.

## Generating bindings

The UniFFI Kotlin bindings and the shared-library `libcs_mobile_core.so` for
each Android ABI are produced by the Rust toolchain and copied in:

```bash
# 1. Install UniFFI CLI (once):
cargo install uniffi-cli

# 2. Build the Rust cdylib for each Android ABI (use cargo-ndk):
cargo install cargo-ndk
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -t x86 \
    -o android/core/core-ffi/src/main/jniLibs \
    build --release -p cs-mobile-core

# 3. Generate the Kotlin bindings from the UDL:
uniffi-bindgen generate \
    crates/cs-mobile-core/src/cs_mobile_core.udl \
    --language kotlin \
    --out-dir android/core/core-ffi/src/main/kotlin
```

The generated file lands in `src/main/kotlin/uniffi/cs_mobile_core/cs_mobile_core.kt`
and is consumed by `MobileCore.kt` in this module.

## Why a dedicated module?

Feature modules should never import `uniffi.cs_mobile_core` directly. Instead
they consume the `MobileCore` facade which:

- keeps the generated-code package private to this module;
- exposes Kotlin-idiomatic types (enums for channel/location, Kotlin Result
  on callable sites that can fail);
- gives us one chokepoint to swap the FFI implementation if ever needed.

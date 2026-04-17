//! flutter_rust_bridge-exposed API.
//!
//! The Flutter app consumes the Rust core through this crate.
//! `flutter_rust_bridge_codegen` parses `src/api/*.rs` and emits
//! matching Dart bindings so Flutter calls feel like ordinary Dart
//! async functions. The same code compiles to:
//!
//! - `arm64-apple-ios` / `aarch64-apple-ios-sim` / `x86_64-apple-ios`
//!   → linked into the iOS Runner.app
//! - `aarch64-linux-android` / `armv7-linux-androideabi` /
//!   `x86_64-linux-android` → packaged under `jniLibs/` in the APK
//! - `wasm32-unknown-unknown` → compiled to wasm and served from
//!   `web/` for Flutter Web
//!
//! All the cryptographic primitives live in `cs-core`; this crate is
//! exclusively about shaping the surface for Dart consumers.

pub mod api;
mod frb_generated;

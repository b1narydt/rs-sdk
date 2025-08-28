// BSV Rust SDK (rs-sdk) - high-level module layout
#![cfg_attr(not(feature = "std"), no_std)]

pub mod prelude;
pub mod error;

pub mod util;
pub mod crypto;
pub mod net; // behind trait; concrete impls optional via features
pub mod address; // address + WIF helpers
pub mod script; // top-level script module to mirror ts-sdk
pub mod compat;
pub mod kvstore;
pub mod primitives;
pub mod registry;
pub mod totp;
pub mod overlay_tools;

pub mod auth;
pub mod identity;
pub mod overlay;
pub mod storage;
pub mod wallet;
pub mod transaction;
pub mod messages;

// WASM bindings are optional
#[cfg(feature = "wasm")]
pub mod wasm;

// Version helper
pub fn version() -> &'static str { env!("CARGO_PKG_VERSION") }

// Public re-exports for common types
pub use error::SdkError;

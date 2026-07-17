// SPDX-License-Identifier: MIT or Apache-2.0
//! # multi-codec
//!
//! Implementation of the [Multicodec](https://github.com/multiformats/multicodec)
//! specification for self-describing protocol and encoding identifiers.
//!
//! ## Overview
//!
//! Multicodec is a self-describing multiformat that provides a way to uniquely
//! identify protocols, encodings, cryptographic algorithms, and other systems.
//! Each codec has:
//! - A unique numeric identifier (code)
//! - A human-readable name
//! - A canonical string representation
//!
//! This crate provides the [`Codec`] enum with 570+ variants representing all
//! standardized multicodec identifiers from the official specification.
//!
//! ## Features
//!
//! - **570+ Codec Variants**: All standardized multicodec identifiers
//! - **Type-Safe Conversions**: `TryFrom`/`Into` for all numeric types and strings
//! - **Serde Support**: JSON and binary serialization (feature-gated)
//! - **`no_std` Support**: Works in `no_std` environments with `alloc`
//! - **Zero Unsafe Code**: Completely safe Rust implementation
//! - **Thread-Safe**: All types are `Send + Sync`
//! - **Type Safety**: Optional newtype wrappers ([`CodecCode`], [`CodecName`])
//! - **Rich Errors**: Detailed error messages with context
//! - **Performance**: Optimized for speed with zero-copy operations where possible
//!
//! ## Quick Start
//!
//! ### Basic Usage
//!
//! ```rust
//! use multi_codec::Codec;
//!
//! // Create codec from name
//! let codec = Codec::try_from("ed25519-pub")?;
//! assert_eq!(codec, Codec::Ed25519Pub);
//!
//! // Get codec properties
//! assert_eq!(codec.code(), 0xED);
//! assert_eq!(codec.as_str(), "ed25519-pub");
//! assert_eq!(format!("{:?}", codec), "ed25519-pub (0xed)");
//! # Ok::<(), multi_codec::Error>(())
//! ```
//!
//! ### Encoding and Decoding
//!
//! ```rust
//! use multi_codec::Codec;
//! use multi_trait::TryDecodeFrom;
//!
//! let codec = Codec::Sha2256;
//!
//! // Encode to varint bytes
//! let bytes: Vec<u8> = codec.into();
//! assert_eq!(bytes, vec![0x12]); // Sha2-256 encodes as single byte
//!
//! // Decode from bytes
//! let (decoded, remaining) = Codec::try_decode_from(&bytes)?;
//! assert_eq!(decoded, codec);
//! assert!(remaining.is_empty());
//! # Ok::<(), multi_codec::Error>(())
//! ```
//!
//! ### Working with Codes and Names
//!
//! ```rust
//! use multi_codec::Codec;
//!
//! // From numeric code
//! let codec = Codec::try_from(0x12u64)?;
//! assert_eq!(codec, Codec::Sha2256);
//!
//! // From string name
//! let codec = Codec::try_from("sha2-256")?;
//! assert_eq!(codec, Codec::Sha2256);
//!
//! // Get code and name
//! assert_eq!(codec.code(), 0x12);
//! assert_eq!(codec.as_str(), "sha2-256");
//! # Ok::<(), multi_codec::Error>(())
//! ```
//!
//! ### Serde Integration
//!
//! ```rust
//! use multi_codec::Codec;
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq)]
//! struct SignatureInfo {
//!     algorithm: Codec,
//!     public_key: Vec<u8>,
//! }
//!
//! let info = SignatureInfo {
//!     algorithm: Codec::Ed25519Pub,
//!     public_key: vec![1, 2, 3, 4],
//! };
//!
//! // Serialize to JSON (human-readable)
//! let json = serde_json::to_string(&info)?;
//! // JSON: {"algorithm":"ed25519-pub","public_key":[1,2,3,4]}
//!
//! // Deserialize from JSON
//! let deserialized: SignatureInfo = serde_json::from_str(&json)?;
//! assert_eq!(deserialized, info);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Error Handling
//!
//! ```rust
//! use multi_codec::{Codec, Error};
//!
//! // Handle invalid names
//! match Codec::try_from("unknown-codec") {
//!     Ok(codec) => println!("Valid: {}", codec.as_str()),
//!     Err(Error::InvalidName { name }) => {
//!         eprintln!("Unknown codec name: {}", name);
//!         // Error includes link to valid codec table
//!     }
//!     Err(e) => eprintln!("Other error: {}", e),
//! }
//!
//! // Handle invalid codes
//! match Codec::try_from(0xDEADBEEFu64) {
//!     Ok(codec) => println!("Valid: {}", codec.as_str()),
//!     Err(Error::InvalidValue { code }) => {
//!         eprintln!("Unknown codec value: 0x{:x}", code);
//!     }
//!     Err(e) => eprintln!("Other error: {}", e),
//! }
//!
//! // Handle negative values
//! match Codec::try_from(-1i64) {
//!     Ok(codec) => println!("Valid: {}", codec.as_str()),
//!     Err(Error::NegativeValue { value }) => {
//!         eprintln!("Negative values not allowed: {}", value);
//!     }
//!     Err(e) => eprintln!("Other error: {}", e),
//! }
//! ```
//!
//! ## Type Safety with Newtypes
//!
//! For additional type safety, use the newtype wrappers:
//!
//! ```rust
//! use multi_codec::{Codec, CodecCode, CodecName};
//!
//! // Type-safe code value
//! let code = CodecCode::new(0xED);
//! assert_eq!(code.get(), 0xED);
//! assert_eq!(code.to_string(), "0xed");
//!
//! // Type-safe name
//! let name = CodecName::new("sha2-256");
//! assert_eq!(name.as_str(), "sha2-256");
//! assert!(!name.is_identity());
//! ```
//!
//! ## Thread Safety
//!
//! All types in this crate are `Send + Sync` and can be safely shared between threads:
//!
//! ```rust
//! use std::sync::Arc;
//! use std::thread;
//! use multi_codec::Codec;
//!
//! let codec = Arc::new(Codec::Ed25519Pub);
//! let handle = thread::spawn(move || {
//!     println!("Codec: {} (0x{:x})", codec.as_str(), codec.code());
//! });
//! handle.join().unwrap();
//! ```
//!
//! ## Performance Characteristics
//!
//! - **Codec enum**: Zero-cost abstraction, `Copy` type (no heap allocations)
//! - **String lookups**: O(1) hash table lookups
//! - **Code lookups**: sequential match on the generated enum (the compiler may
//!   optimize large matches to a jump table, but O(log n) is not guaranteed)
//! - **Encoding**: Stack-allocated buffers (no heap allocation)
//! - **Hash**: Direct u64 hashing (no intermediate allocation)
//! - **Serialization**: Stack-allocated arrays (no heap allocation)
//!
//! ## Security
//!
//! This crate follows strict security practices:
//!
//! - **No unsafe code**: Completely safe Rust implementation
//! - **Input validation**: All conversions validate input ranges
//! - **`DoS` protection**: Size limits on deserialization (max 19 bytes)
//! - **Integer overflow protection**: Negative value checks in signed conversions
//! - **Error handling**: All errors return `Result` types, no panics on invalid input
//! - **Thread safety**: All types are `Send + Sync` with no shared mutable state
//!
//! ## Multicodec Table
//!
//! The codec table is generated at build time from the official
//! [Multicodec Table CSV](https://github.com/multiformats/multicodec/blob/master/table.csv).
//! This ensures the crate stays synchronized with the specification.
//!
//! To update the table, replace `table.csv` and rebuild.
//!
//! ## Feature Flags
//!
//! - **`std`** (default): Enables `std` support (pulls in `std`-gated features of dependencies)
//! - **`serde`** (default): Enables serde serialization/deserialization support
//!
//! ### `no_std` Support
//!
//! This crate works in `no_std` environments with `alloc`. Disable the default
//! features:
//!
//! ```toml
//! [dependencies]
//! multi-codec = { version = "1.0", default-features = false }
//! ```
//!
//! This drops the `std` and `serde` features. To use serde under `no_std`,
//! enable only the `serde` feature:
//!
//! ```toml
//! [dependencies]
//! multi-codec = { version = "1.0", default-features = false, features = ["serde"] }
//! ```
//!
//! ## Common Patterns
//!
//! ### Using in Data Structures
//!
//! ```rust
//! use multi_codec::Codec;
//! use std::collections::HashMap;
//!
//! // Codec as HashMap key
//! let mut algorithms = HashMap::new();
//! algorithms.insert(Codec::Sha2256, "SHA-256");
//! algorithms.insert(Codec::Sha2512, "SHA-512");
//! algorithms.insert(Codec::Blake3, "BLAKE3");
//!
//! assert_eq!(algorithms.get(&Codec::Sha2256), Some(&"SHA-256"));
//! ```
//!
//! ### Sequential Codec Handling
//!
//! ```rust
//! use multi_codec::Codec;
//! use multi_trait::{TryDecodeFrom, EncodeIntoBuffer};
//!
//! // Encode multiple codecs sequentially
//! let codecs = vec![Codec::Identity, Codec::Sha2256, Codec::Ed25519Pub];
//!
//! let mut buffer = Vec::new();
//! for codec in &codecs {
//!     let code: u64 = (*codec).into();
//!     code.encode_into_buffer(&mut buffer);
//! }
//!
//! // Decode them back
//! let mut remaining = buffer.as_slice();
//! for expected in &codecs {
//!     let (code, rest) = u64::try_decode_from(remaining)?;
//!     let decoded = Codec::try_from(code)?;
//!     assert_eq!(decoded, *expected);
//!     remaining = rest;
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Examples
//!
//! See the [Multicodec specification](https://github.com/multiformats/multicodec)
//! and the crate-level examples in this file for additional usage.
//!
//! ## Specification Compliance
//!
//! This crate implements the [Multicodec specification](https://github.com/multiformats/multicodec)
//! and maintains compatibility with the official codec table.

#![warn(missing_docs)]
#![deny(
    unsafe_code,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

/// Errors produced by this library
pub mod error;
pub use error::Error;

/// Codec enum definition from the table
pub mod codec;
pub use codec::Codec;

/// Type-safe wrappers for codec identifiers
pub mod types;
pub use types::{CodecCode, CodecName};

/// Serde serialization
#[cfg(feature = "serde")]
pub mod serde;

/// Commonly used items
///
/// ```
/// use multi_codec::prelude::*;
///
/// let codec = Codec::try_from("sha2-256")?;
/// assert_eq!(codec.code(), 0x12);
/// # Ok::<(), multi_codec::Error>(())
/// ```
pub mod prelude {
    pub use super::{codec::*, error::*, types::*};

    // re-exports
    pub use multi_trait::{EncodeInto, TryDecodeFrom};
}

[![](https://img.shields.io/badge/made%20by-Cryptid%20Technologies-gold.svg?style=flat-square)](https://cryptid.tech/)
[![](https://img.shields.io/badge/project-provenance-purple.svg?style=flat-square)](https://github.com/cryptidtech/provenance-specifications/)
[![](https://img.shields.io/badge/project-multiformats-blue.svg?style=flat-square)](https://github.com/multiformats/multiformats/)

[![Build Status](https://github.com/cryptidtech/multi-codec/actions/workflows/rust.yml/badge.svg)](https://github.com/cryptidtech/multi-codec/actions)
[![License](https://img.shields.io/crates/l/multi-codec?style=flat-square)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/multi-codec?style=flat-square)](https://crates.io/crates/multi-codec)
[![Documentation](https://docs.rs/multi-codec/badge.svg?style=flat-square)](https://docs.rs/multi-codec)

# multi-codec

Rust implementation of the [multicodec](https://github.com/multiformats/multicodec)
specification for self-describing protocol and encoding identifiers.

## Overview

Multicodec is a self-describing multiformat that provides a way to uniquely
identify protocols, encodings, cryptographic algorithms, and other systems.
Each codec has a unique numeric identifier (code), a human-readable name, and a
canonical string representation.

This crate provides the `Codec` enum with 570+ variants representing all
standardized multicodec identifiers from the official specification. The enum
and its conversions are generated at build time from `table.csv`, which is kept
in sync with the [official multicodec table](https://github.com/multiformats/multicodec/blob/master/table.csv).

## Table of Contents

- [Features](#features)
- [Install](#install)
- [Usage](#usage)
  - [Basic Usage](#basic-usage)
  - [Encoding and Decoding](#encoding-and-decoding)
  - [Working with Codes and Names](#working-with-codes-and-names)
  - [Serde Integration](#serde-integration)
  - [Error Handling](#error-handling)
  - [Type-Safe Newtypes](#type-safe-newtypes)
  - [Sequential Codec Handling](#sequential-codec-handling)
- [CLI Example](#cli-example)
- [Testing](#testing)
- [Updating the Codec Table](#updating-the-codec-table)
- [Feature Flags](#feature-flags)
- [Security](#security)
- [Maintainers](#maintainers)
- [Contribute](#contribute)
- [License](#license)

## Features

- **570+ Codec Variants**: All standardized multicodec identifiers
- **Type-Safe Conversions**: `TryFrom`/`Into` for all numeric types and strings
- **Serde Support**: JSON and binary serialization (feature-gated)
- **`no_std` Support**: Works in `no_std` environments with `alloc`
- **Zero Unsafe Code**: `#![deny(unsafe_code)]` enforced at compile time
- **Thread-Safe**: All types are `Send + Sync`
- **Type-Safe Newtypes**: `CodecCode` and `CodecName` wrappers
- **Varint Encoding**: Encodes codecs as unsigned varints via `multi-trait`

## Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
multi-codec = "1.0"
```

For `no_std` environments (disable `std` and `serde`):

```toml
[dependencies]
multi-codec = { version = "1.0", default-features = false }
```

To use serde under `no_std`, enable only the `serde` feature:

```toml
[dependencies]
multi-codec = { version = "1.0", default-features = false, features = ["serde"] }
```

**MSRV**: Rust 1.85 (Edition 2024)

## Usage

### Basic Usage

```rust
use multi_codec::Codec;

// Create codec from name
let codec = Codec::try_from("ed25519-pub")?;
assert_eq!(codec, Codec::Ed25519Pub);

// Get codec properties
assert_eq!(codec.code(), 0xED);
assert_eq!(codec.as_str(), "ed25519-pub");
assert_eq!(format!("{:?}", codec), "ed25519-pub (0xed)");
# Ok::<(), multi_codec::Error>(())
```

### Encoding and Decoding

Codecs encode as unsigned varints via the `multi-trait` crate:

```rust
use multi_codec::Codec;
use multi_trait::TryDecodeFrom;

let codec = Codec::Sha2256;

// Encode to varint bytes
let bytes: Vec<u8> = codec.into();
assert_eq!(bytes, vec![0x12]); // Sha2-256 encodes as a single byte

// Decode from bytes (returns remaining slice for streaming)
let (decoded, remaining) = Codec::try_decode_from(&bytes)?;
assert_eq!(decoded, codec);
assert!(remaining.is_empty());
# Ok::<(), multi_codec::Error>(())
```

### Working with Codes and Names

```rust
use multi_codec::Codec;

// From numeric code
let codec = Codec::try_from(0x12u64)?;
assert_eq!(codec, Codec::Sha2256);

// From string name
let codec = Codec::try_from("sha2-256")?;
assert_eq!(codec, Codec::Sha2256);

// Get code and name
assert_eq!(codec.code(), 0x12);
assert_eq!(codec.as_str(), "sha2-256");
# Ok::<(), multi_codec::Error>(())
```

Signed integer conversions reject negative values:

```rust
use multi_codec::{Codec, Error};

match Codec::try_from(-1i64) {
    Err(Error::NegativeValue { value }) => eprintln!("Negative not allowed: {}", value),
    _ => unreachable!(),
}
```

### Serde Integration

With the `serde` feature (enabled by default), `Codec` serializes as a string in
human-readable formats (JSON, TOML) and as varint bytes in binary formats (CBOR,
bincode):

```rust
use multi_codec::Codec;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct SignatureInfo {
    algorithm: Codec,
    public_key: Vec<u8>,
}

let info = SignatureInfo {
    algorithm: Codec::Ed25519Pub,
    public_key: vec![1, 2, 3, 4],
};

// Serialize to JSON (human-readable → codec name string)
let json = serde_json::to_string(&info)?;
assert!(json.contains("ed25519-pub"));

// Deserialize from JSON
let deserialized: SignatureInfo = serde_json::from_str(&json)?;
assert_eq!(deserialized, info);
# Ok::<(), Box<dyn std::error::Error>>(())
```

### Error Handling

All conversion errors return `Result` with a structured `Error` enum:

```rust
use multi_codec::{Codec, Error};

// Handle invalid names
match Codec::try_from("unknown-codec") {
    Err(Error::InvalidName { name }) => {
        eprintln!("Unknown codec name: {}", name);
    }
    Err(e) => eprintln!("Other error: {}", e),
    Ok(_) => unreachable!(),
}

// Handle invalid codes
match Codec::try_from(0xDEADBEEFu64) {
    Err(Error::InvalidValue { code }) => {
        eprintln!("Unknown codec value: 0x{:x}", code);
    }
    Err(e) => eprintln!("Other error: {}", e),
    Ok(_) => unreachable!(),
}
```

### Type-Safe Newtypes

For additional type safety, use the newtype wrappers:

```rust
use multi_codec::{CodecCode, CodecName};

// Type-safe code value
let code = CodecCode::new(0xED);
assert_eq!(code.get(), 0xED);
assert_eq!(code.to_string(), "0xed");

// Type-safe name
let name = CodecName::new("sha2-256");
assert_eq!(name.as_str(), "sha2-256");
assert!(!name.is_identity());
```

### Sequential Codec Handling

Multiple codecs can be encoded into a single buffer and decoded back in order:

```rust
use multi_codec::Codec;
use multi_trait::{TryDecodeFrom, EncodeIntoBuffer};

let codecs = vec![Codec::Identity, Codec::Sha2256, Codec::Ed25519Pub];

// Encode all codecs into one buffer
let mut buffer = Vec::new();
for codec in &codecs {
    let code: u64 = (*codec).into();
    code.encode_into_buffer(&mut buffer);
}

// Decode them back sequentially
let mut remaining = buffer.as_slice();
for expected in &codecs {
    let (code, rest) = u64::try_decode_from(remaining)?;
    let decoded = Codec::try_from(code)?;
    assert_eq!(decoded, *expected);
    remaining = rest;
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

## CLI Example

The crate includes a varuint encode/decode example in `examples/uvi.rs`:

```bash
# Encode a hex number as varuint
cargo run --example uvi -- -e 012c

# Decode a hex-encoded varuint
cargo run --example uvi -- -d ac02
```

## Testing

The crate has 160 tests across unit, integration, property-based, security, and
doc-test suites:

```bash
# Run all tests
cargo test --all-features

# Run specific test suites
cargo test --test edge_case_tests
cargo test --test error_tests
cargo test --test integration_tests
cargo test --test proptest_tests
cargo test --test security_tests

# Run benchmarks
cargo bench

# Run the example
cargo run --example uvi -- -e 012c
```

Linting and formatting:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Updating the Codec Table

The codec enum is generated at build time from `table.csv` at the crate root.
The build script (`build.rs`) parses the CSV, validates that all codes and
names are unique, and emits the `src/table_gen.rs` file that is included into
`src/codec.rs`.

To update the table:

1. Replace `table.csv` with the latest version from the
   [official multicodec repository](https://github.com/multiformats/multicodec/blob/master/table.csv)
   (or apply your local additions).
2. Run `cargo build` — the build script will regenerate `src/table_gen.rs`.
3. Run `cargo test` to verify the new codecs work correctly.

If the CSV contains duplicate codes or names, the build will fail with an error
identifying the offending row.

## Feature Flags

- **`serde`** (default): Enables serde serialization/deserialization. When
  enabled, `Codec` implements `Serialize` and `Deserialize` — strings in
  human-readable formats, varint bytes in binary formats.

### Disabling Default Features

```toml
[dependencies]
multi-codec = { version = "1.0", default-features = false }
```

## Security

- `#![deny(unsafe_code)]` enforced at compile time
- All conversions validate input ranges; negative signed integers are rejected
- Deserialization has a 19-byte size limit on varint input (DoS protection)
- All errors return `Result` types — no panics on invalid input
- All types are `Send + Sync` with no shared mutable state

## Maintainers

This repo: [@dhuseby](https://github.com/dhuseby).

Original author: [@gnunicorn](https://github.com/gnunicorn).

## Contribute

Contributions welcome! Please check out [the issues](https://github.com/cryptidtech/multi-codec/issues).

### Development Guidelines

- Run `cargo fmt` before committing
- Run `cargo clippy -- -D warnings` to check for issues
- Add tests for new features
- Update documentation for API changes
- Run the full test suite: `cargo test --all-features`

## License

[MIT OR Apache-2.0](LICENSE) © Cryptid Technologies
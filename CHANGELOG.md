# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.5] - 2026-07-16

### Security
- Removed unmaintained `serde_cbor` dev-dependency (RUSTSEC-2021-0127). Replaced
  with `ciborium` (a maintained CBOR library) in all test and benchmark code.

### Changed
- Added `cbor_to_vec` helper function in test modules and benchmarks to wrap
  `ciborium::into_writer` (replacing `serde_cbor::to_vec`).
- Replaced `serde_cbor::from_slice` with `ciborium::from_reader` (using
  `bytes.as_slice()` which implements `std::io::Read`).
- Replaced `serde_cbor::to_writer` with `ciborium::into_writer`.
- Replaced `serde_cbor::from_reader` with `ciborium::from_reader` (same API
  name, different crate).

### Dependencies
- Removed `serde_cbor = "0.11"` dev-dependency.
- Added `ciborium = "0.2"` dev-dependency.
- Dependency count reduced from 112 to 110 crates.

## [1.0.4] - 2026-07-16

### Added
- **`no_std` support**: The crate now works in `no_std` environments with
  `alloc`. Added `#![cfg_attr(not(feature = "std"), no_std)]` and
  `#[cfg(not(feature = "std"))] extern crate alloc;` to `src/lib.rs`.
- **`std` feature gate**: `default = ["std", "serde"]`, with `std` enabling
  `thiserror/std`, `serde/std`, and `multi-trait/std`.
- **`no_std` CI job**: `ensure_no_std` job in `.github/workflows/rust.yml`
  that builds for `thumbv6m-none-eabi` with `--no-default-features`.
- **`no_std` documentation**: Updated `README.md` and `src/lib.rs` doc
  comments with `no_std` recipes (both serde-off and serde-on configurations).
- Added `no_std` to crate keywords.

### Changed
- `multi-trait` dependency: `default-features = false` (std enabled via our
  `std` feature).
- `thiserror` dependency: `default-features = false`.
- `serde` feature now uses `dep:serde` syntax.
- Added `#[cfg(not(feature = "std"))] use alloc::...` imports in `src/error.rs`,
  `src/types.rs`, `src/codec.rs`, and `src/serde/de.rs` for `String`, `Vec`,
  `ToString`, and `format` (gated to no_std mode only).

## [1.0.3] - 2026-07-15

### Added
- **`#![deny(unsafe_code)]`** at the crate root and in `codec.rs`.
- **`#[inline]`** and **`#[must_use]`** on `Codec::code()` and `Codec::as_str()`.
- **`#[must_use]`** on `Error::invalid_name()`, `Error::invalid_value()`,
  `Error::negative_value()`, and `Error::kind()`.
- **MSRV declared**: `rust-version = "1.85"` in `Cargo.toml`. CI verifies the
  MSRV with a dedicated job.
- **`cargo audit`** job in CI.
- **`cargo fmt --check`** and **`clippy -D warnings`** steps in CI.
- **Clippy lint configuration**: `[lints.clippy]` with `pedantic`, `nursery`,
  and `cargo` groups (all `warn`), plus `[lints.rust] unsafe_code = "deny"`.
- **README.md** description and examples.

### Changed
- **Edition 2024**: Updated from Rust 2021.
- **Signed integer `TryFrom` impls**: Replaced `as u64`/`as i64` casts with
  `u64::from`/`i64::from` and `u64::try_from` for infallible and fallible
  conversions (clippy::cast_sign_loss, clippy::cast_possible_truncation).
- **`should_panic` tests**: Added reasons to `test_invalid_value` and
  `test_invalid_name`.
- **Clippy pedantic/nursery/cargo warnings** resolved across all source,
  tests, benchmarks, and examples.

## [1.0.2] - 2026-07-13

### Changed
- Updated `table.csv` to the latest multicodec specification (1503 row changes
  across codec additions, recategorizations, and removals).
- Updated `table_gen.rs` build script to handle new table format.
- Updated `multi-trait` dependency version.

## [1.0.1] - 2026-07-13

### Changed
- Updated `table.csv` with corrected codec names and categories.
- Updated `build.rs` for table generation compatibility.

## [1.0.0] - 2026-07-13

### Changed
- Synced from bettersign workspace (bs-multicodec 0.7.0)
- Renamed crate from `bs-multicodec` to `multi-codec`
- Added `types.rs` module with type-safe codec wrappers
- Added test suite (edge cases, error tests, integration, proptest, security)
- Initial published release on crates.io as `multi-codec`

---

## Prior releases as `multicodec` (pre-rename)

## [multicodec 1.0.2] - 2024-05-07

### Changed
- Updated `table.csv` to the latest multicodec specification
- Recategorized `vlad`, `provenance-log`, `provenance-log-entry`, and `nonce`
  codecs to their correct tag types
- Recategorized `es256k`, `bls12_381-g1-sig`, `bls12_381-g2-sig`, and `eddsa`
  from `multisig` to `varsig` tag type
- Recategorized `es256`, `es284`, `es512`, and `rs256` from `multisig` to
  `varsig` tag type
- Removed `bls12_381-g1-sig-share` and `bls12_381-g2-sig-share` entries
- Added `blake3-hashseq` (`0x80`) codec
- Added `es256k-msig`, `bls12_381-g1-msig`, `bls12_381-g2-msig`, `eddsa-msig`,
  `bls12_381-g1-share-msig`, `bls12_381-g2-share-msig`, `lamport-msig`,
  `lamport-share-msig`, `es256-msig`, `es284-msig`, `es512-msig`, and
  `rs256-msig` multisig codecs

## [multicodec 1.0.1] - 2024-05-07

### Changed
- Updated LICENSE copyright notice to "Copyright 2024 Cryptid Technologies, Inc."

## [multicodec 1.0.0] - 2024-04-14

### Added
- `Null` impl for `Codec`
- Lamport key and signature types
- BLS12-381 signature shares, secret key shares, and public key shares
- `Display`, `Default`, `Ord`, `PartialOrd`, `Hash`, `Copy`, and `PartialEq`
  for `Error`
- Multisig sigil support
- Provenance-log and provenance-log-entry codecs
- `CodecInfo` trait
- Serde serialization with human-readable and binary modes
- `TryDecodeFrom` and `EncodeInto` impls
- `Into<u128>` and `Into<u64>` for `Codec`
- `TryFrom<&str>` and `From<Codec> for &str` for canonical string names
- ChaCha20-Poly1305 encryption scheme codec
- Varuint codec support
- Code generation from `table.csv` at build time

### Changed
- Reworked interface to use `multi-trait` traits
- Switched codec numeric type from `u128` to `u64`
- Moved `CodecInfo` and `EncodingInfo` to `multi-util` crate
- Cleaned up imports and exports

[1.0.5]: https://github.com/cryptidtech/multi-codec/compare/v1.0.4...v1.0.5
[1.0.4]: https://github.com/cryptidtech/multi-codec/compare/v1.0.3...v1.0.4
[1.0.3]: https://github.com/cryptidtech/multi-codec/compare/v1.0.0...v1.0.3
[1.0.2]: https://github.com/cryptidtech/multi-codec/releases/tag/v1.0.2
[1.0.1]: https://github.com/cryptidtech/multi-codec/releases/tag/v1.0.1
[1.0.0]: https://github.com/cryptidtech/multi-codec/releases/tag/multi-codec-v1.0.0
[multicodec 1.0.2]: https://github.com/cryptidtech/multi-codec/releases/tag/v1.0.2
[multicodec 1.0.1]: https://github.com/cryptidtech/multi-codec/releases/tag/v1.0.1
[multicodec 1.0.0]: https://github.com/cryptidtech/multi-codec/releases/tag/v1.0.0
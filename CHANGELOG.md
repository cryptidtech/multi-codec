# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[1.0.3]: https://github.com/cryptidtech/multi-codec/compare/v1.0.0...v1.0.3
[multicodec 1.0.2]: https://github.com/cryptidtech/multi-codec/releases/tag/v1.0.2
[multicodec 1.0.1]: https://github.com/cryptidtech/multi-codec/releases/tag/v1.0.1
[multicodec 1.0.0]: https://github.com/cryptidtech/multi-codec/releases/tag/v1.0.0
[1.0.0]: https://github.com/cryptidtech/multi-codec/releases/tag/multi-codec-v1.0.0

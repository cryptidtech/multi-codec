# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.1] - 2026-08-17

- Fix the `multi-trait` dependency to use published crate.

## [1.2.0] - 2026-08-13

### Added

- Synced the multicodec table from the BetterSign workspace. This adds 65 new codec entries: full Lamport signature variants (SHA2-256/384/512, BLAKE2b-512, BLAKE2s-256, BLAKE3-256, SHAKE-128/256 for Pub/Priv/PrivShare/Sig/SigShare), XMSS variants (SHA2-10/16/20-256 for Pub/Priv/Msig), additional McEliece variants, HQC key variants, SLH-DSA key variants, and other PQC codec entries that were in the workspace but not in the published crate.
- The `Codec` enum now has 827 variants (up from 762).

### Notes

- This is a minor version bump. The `Codec` enum is `#[non_exhaustive]`, so new variants are additive and do not break downstream match expressions.
- This sync unblocks Lamport and XMSS signature support in `multi-sig` (Phase 8) and `multi-key` (Phase 9) of the crate extraction plan.

## [1.1.0] - 2026-07-29

### Security

- `TryFrom<&[u8]>` for `Codec` now rejects trailing bytes. The old impl discarded bytes after the codec varint. This could hide parsing bugs when a caller expected the full buffer to be consumed. The impl now returns `Error::TrailingData { consumed, remaining }` when bytes remain after the varint. To parse a stream with trailing data, use `Codec::try_decode_from`. It returns the codec and the remaining slice. That behavior did not change.
- Added the `Error::TrailingData` variant. `Error` is `#[non_exhaustive]`, so the variant is additive.
- Updated `Error::kind()` and `Error::context()` to cover `TrailingData`.

### Changed

- BREAKING. `TryFrom<&[u8]>` for `Codec` no longer discards trailing bytes. It returns `Error::TrailingData`. Code that depended on the discard behavior must switch to `Codec::try_decode_from`.
- The `TryFrom<&[u8]>` doc comment now documents the trailing-data rejection. It points to `try_decode_from` for stream use.
- Rewrote `README.md`, `SECURITY.md`, and `CHANGELOG.md` in ASD-STE100 strict mode. Removed marketing language, passive voice, and long sentences.

### Tests

- Added `test_try_from_bytes_no_trailing_ok`.
- Added `test_try_from_bytes_trailing_rejected`.
- Added `test_try_from_bytes_trailing_single_byte_rejected`.
- Added `test_try_decode_from_still_returns_trailing`. This test confirms `TryDecodeFrom` still returns the trailing slice.

## [1.0.5] - 2026-07-16

### Security

- Removed the unmaintained `serde_cbor` dev-dependency (RUSTSEC-2021-0127). Replaced it with `ciborium` (a maintained CBOR library) in all test and benchmark code.

### Changed

- Added the `cbor_to_vec` helper function in test modules and benchmarks. It wraps `ciborium::into_writer` to replace `serde_cbor::to_vec`.
- Replaced `serde_cbor::from_slice` with `ciborium::from_reader`. The call uses `bytes.as_slice()`, which implements `std::io::Read`.
- Replaced `serde_cbor::to_writer` with `ciborium::into_writer`.
- Replaced `serde_cbor::from_reader` with `ciborium::from_reader`. The API name is the same. The crate is different.

### Dependencies

- Removed the `serde_cbor = "0.11"` dev-dependency.
- Added the `ciborium = "0.2"` dev-dependency.
- The dependency count went from 112 to 110 crates.

## [1.0.4] - 2026-07-16

### Added

- `no_std` support. The crate now works in `no_std` environments with `alloc`. Added `#![cfg_attr(not(feature = "std"), no_std)]` and `#[cfg(not(feature = "std"))] extern crate alloc;` to `src/lib.rs`.
- `std` feature gate. `default = ["std", "serde"]`. The `std` feature enables `thiserror/std`, `serde/std`, and `multi-trait/std`.
- `no_std` CI job. The `ensure_no_std` job in `.github/workflows/rust.yml` builds for `thumbv6m-none-eabi` with `--no-default-features`.
- `no_std` documentation. Updated `README.md` and `src/lib.rs` doc comments with `no_std` recipes for both serde-off and serde-on configurations.
- Added `no_std` to the crate keywords.

### Changed

- `multi-trait` dependency now uses `default-features = false`. The `std` feature enables the `std` features of `multi-trait`.
- `thiserror` dependency now uses `default-features = false`.
- The `serde` feature now uses the `dep:serde` syntax.
- Added `#[cfg(not(feature = "std"))] use alloc::...` imports in `src/error.rs`, `src/types.rs`, `src/codec.rs`, and `src/serde/de.rs`. These imports provide `String`, `Vec`, `ToString`, and `format` in `no_std` mode.

## [1.0.3] - 2026-07-15

### Added

- `#![deny(unsafe_code)]` at the crate root and in `codec.rs`.
- `#[inline]` and `#[must_use]` on `Codec::code()` and `Codec::as_str()`.
- `#[must_use]` on `Error::invalid_name()`, `Error::invalid_value()`, `Error::negative_value()`, and `Error::kind()`.
- MSRV declared. `rust-version = "1.85"` in `Cargo.toml`. CI verifies the MSRV with a dedicated job.
- `cargo audit` job in CI.
- `cargo fmt --check` and `clippy -D warnings` steps in CI.
- Clippy lint configuration. `[lints.clippy]` with `pedantic`, `nursery`, and `cargo` groups set to `warn`. `[lints.rust] unsafe_code = "deny"`.
- `README.md` description and examples.

### Changed

- Edition 2024. Updated from Rust 2021.
- Signed integer `TryFrom` impls. Replaced `as u64` and `as i64` casts with `u64::from`, `i64::from`, and `u64::try_from` for infallible and fallible conversions. This resolves `clippy::cast_sign_loss` and `clippy::cast_possible_truncation`.
- `should_panic` tests. Added reasons to `test_invalid_value` and `test_invalid_name`.
- Resolved all clippy pedantic, nursery, and cargo warnings across source, tests, benchmarks, and examples.

## [1.0.2] - 2026-07-13

### Changed

- Updated `table.csv` to the latest multicodec specification. The update changed 1503 rows across codec additions, recategorizations, and removals.
- Updated `table_gen.rs` build script to handle the new table format.
- Updated the `multi-trait` dependency version.

## [1.0.1] - 2026-07-13

### Changed

- Updated `table.csv` with corrected codec names and categories.
- Updated `build.rs` for table generation compatibility.

## [1.0.0] - 2026-07-13

### Changed

- Synced from the bettersign workspace (`bs-multicodec` 0.7.0).
- Renamed the crate from `bs-multicodec` to `multi-codec`.
- Added the `types.rs` module with type-safe codec wrappers.
- Added a test suite for edge cases, errors, integration, proptests, and security.
- Initial published release on crates.io as `multi-codec`.

---

## Prior releases as `multicodec` (pre-rename)

## [multicodec 1.0.2] - 2024-05-07

### Changed

- Updated `table.csv` to the latest multicodec specification.
- Recategorized `vlad`, `provenance-log`, `provenance-log-entry`, and `nonce` codecs to their correct tag types.
- Recategorized `es256k`, `bls12_381-g1-sig`, `bls12_381-g2-sig`, and `eddsa` from `multisig` to `varsig` tag type.
- Recategorized `es256`, `es284`, `es512`, and `rs256` from `multisig` to `varsig` tag type.
- Removed `bls12_381-g1-sig-share` and `bls12_381-g2-sig-share` entries.
- Added `blake3-hashseq` (`0x80`) codec.
- Added `es256k-msig`, `bls12_381-g1-msig`, `bls12_381-g2-msig`, `eddsa-msig`, `bls12_381-g1-share-msig`, `bls12_381-g2-share-msig`, `lamport-msig`, `lamport-share-msig`, `es256-msig`, `es284-msig`, `es512-msig`, and `rs256-msig` multisig codecs.

## [multicodec 1.0.1] - 2024-05-07

### Changed

- Updated the LICENSE copyright notice to "Copyright 2024 Cryptid Technologies, Inc."

## [multicodec 1.0.0] - 2024-04-14

### Added

- `Null` impl for `Codec`.
- Lamport key and signature types.
- BLS12-381 signature shares, secret key shares, and public key shares.
- `Display`, `Default`, `Ord`, `PartialOrd`, `Hash`, `Copy`, and `PartialEq` for `Error`.
- Multisig sigil support.
- Provenance-log and provenance-log-entry codecs.
- `CodecInfo` trait.
- Serde serialization with human-readable and binary modes.
- `TryDecodeFrom` and `EncodeInto` impls.
- `Into<u128>` and `Into<u64>` for `Codec`.
- `TryFrom<&str>` and `From<Codec> for &str` for canonical string names.
- ChaCha20-Poly1305 encryption scheme codec.
- Varuint codec support.
- Code generation from `table.csv` at build time.

### Changed

- Reworked the interface to use `multi-trait` traits.
- Switched the codec numeric type from `u128` to `u64`.
- Moved `CodecInfo` and `EncodingInfo` to the `multi-util` crate.
- Cleaned up imports and exports.

[1.1.0]: https://github.com/cryptidtech/multi-codec/compare/v1.0.5...v1.1.0
[1.0.5]: https://github.com/cryptidtech/multi-codec/compare/v1.0.4...v1.0.5
[1.0.4]: https://github.com/cryptidtech/multi-codec/compare/v1.0.3...v1.0.4
[1.0.3]: https://github.com/cryptidtech/multi-codec/compare/v1.0.0...v1.0.3
[1.0.2]: https://github.com/cryptidtech/multi-codec/releases/tag/v1.0.2
[1.0.1]: https://github.com/cryptidtech/multi-codec/releases/tag/v1.0.1
[1.0.0]: https://github.com/cryptidtech/multi-codec/releases/tag/multi-codec-v1.0.0
[multicodec 1.0.2]: https://github.com/cryptidtech/multi-codec/releases/tag/v1.0.2
[multicodec 1.0.1]: https://github.com/cryptidtech/multi-codec/releases/tag/v1.0.1
[multicodec 1.0.0]: https://github.com/cryptidtech/multi-codec/releases/tag/v1.0.0

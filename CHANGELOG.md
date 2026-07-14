# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-07-13

### Changed
- Synced from bettersign workspace (bs-multicodec 0.7.0)
- Renamed crate from `bs-multicodec` to `multi-codec`
- Added `types.rs` module with type-safe codec wrappers
- Added comprehensive test suite (edge cases, error tests, integration, proptest, security)
- Initial published release on crates.io as `multi-codec`
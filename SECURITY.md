# Security Policy

## Overview

The `multi-codec` crate implements the [Multicodec](https://github.com/multiformats/multicodec) specification. It provides self-describing protocol and encoding identifiers. This document describes the security properties of the crate.

## `no_std` Support

The crate works in `no_std` environments with `alloc`. Disable the default features to remove the `std` dependency:

```toml
[dependencies]
multi-codec = { version = "1.1", default-features = false }
```

To use serde under `no_std`, enable only the `serde` feature:

```toml
[dependencies]
multi-codec = { version = "1.1", default-features = false, features = ["serde"] }
```

The `std` feature is on by default. It enables `thiserror/std`, `serde/std`, and `multi-trait/std`. A CI `ensure_no_std` job builds the `no_std` target on each push and pull request.

## Memory Safety

- No unsafe code. `#![deny(unsafe_code)]` is set at the crate root. `[lints.rust] unsafe_code = "deny"` in `Cargo.toml` enforces it too.
- Input validation. All conversions check input ranges. Negative signed integers return `Error::NegativeValue`.
- DoS protection. Deserialization rejects varint input longer than 19 bytes. 19 bytes is the maximum for a `u128`. Oversized input returns `Err`. It does not allocate or panic.
- Trailing-data rejection. `TryFrom<&[u8]>` rejects bytes left after the codec varint. It returns `Error::TrailingData`. This prevents silent data loss when a caller expects the full buffer. To parse a stream with trailing data, use `Codec::try_decode_from`. It returns the codec and the remaining slice.
- All errors return `Result`. No path panics on invalid input.

## Reporting Vulnerabilities

Report security issues through the GitHub issue tracker. You can also report them privately to the maintainers.
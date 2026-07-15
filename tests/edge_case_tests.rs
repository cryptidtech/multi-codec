// SPDX-License-Identifier: MIT or Apache-2.0
#![allow(
    clippy::unreadable_literal,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::should_panic_without_expect,
    clippy::items_after_statements,
    clippy::match_same_arms
)]
//! Edge case tests for multi-codec
//!
//! These tests verify behavior at boundary conditions and extreme values.

use multi_codec::Codec;
use multi_trait::{Null, TryDecodeFrom};

/// Test identity codec (0x00) - the default
#[test]
fn test_identity_codec() {
    let codec = Codec::try_from(0x00u64).unwrap();
    assert_eq!(codec, Codec::Identity);
    assert_eq!(codec.code(), 0x00);
    assert_eq!(codec.as_str(), "identity");

    // Should be the default
    assert_eq!(codec, Codec::default());

    // Should be the null value
    assert_eq!(codec, Codec::null());
    assert!(codec.is_null());
}

/// Test maximum valid codec values
#[test]
fn test_maximum_codec_values() {
    // Test some large valid codes
    // Most multicodec values are in lower ranges, but test higher ones if they exist

    // Test that u64::MAX is rejected (not in table)
    let result = Codec::try_from(u64::MAX);
    assert!(result.is_err());

    // Test that values beyond table range are rejected
    let result = Codec::try_from(0xFFFFFFFFu64);
    assert!(result.is_err());
}

/// Test minimum unsigned values across all types
#[test]
fn test_minimum_unsigned_values() {
    // All unsigned types should work with 0 (identity)
    assert!(Codec::try_from(0u8).is_ok());
    assert!(Codec::try_from(0u16).is_ok());
    assert!(Codec::try_from(0u32).is_ok());
    assert!(Codec::try_from(0u64).is_ok());
}

/// Test maximum unsigned values that fit in smaller types
#[test]
fn test_maximum_small_unsigned_values() {
    // These should work if the codes exist in the table
    let _ = Codec::try_from(u8::MAX); // May or may not exist
    let _ = Codec::try_from(u16::MAX); // May or may not exist
    let _ = Codec::try_from(u32::MAX); // Likely doesn't exist
}

/// Test zero values for all signed types
#[test]
fn test_zero_signed_values() {
    assert_eq!(Codec::try_from(0i8).unwrap(), Codec::Identity);
    assert_eq!(Codec::try_from(0i16).unwrap(), Codec::Identity);
    assert_eq!(Codec::try_from(0i32).unwrap(), Codec::Identity);
    assert_eq!(Codec::try_from(0i64).unwrap(), Codec::Identity);
}

/// Test minimum signed values (all negative, should fail)
#[test]
fn test_minimum_signed_values() {
    assert!(Codec::try_from(i8::MIN).is_err());
    assert!(Codec::try_from(i16::MIN).is_err());
    assert!(Codec::try_from(i32::MIN).is_err());
    assert!(Codec::try_from(i64::MIN).is_err());
}

/// Test maximum signed values (positive, may or may not exist)
#[test]
fn test_maximum_signed_values() {
    let _ = Codec::try_from(i8::MAX); // 127
    let _ = Codec::try_from(i16::MAX); // 32767
    let _ = Codec::try_from(i32::MAX); // Likely doesn't exist
    let _ = Codec::try_from(i64::MAX); // Likely doesn't exist
}

/// Test empty string (should fail)
#[test]
fn test_empty_string() {
    let result = Codec::try_from("");
    assert!(result.is_err());
}

/// Test whitespace strings (should fail)
#[test]
fn test_whitespace_strings() {
    assert!(Codec::try_from(" ").is_err());
    assert!(Codec::try_from("  ").is_err());
    assert!(Codec::try_from("\t").is_err());
    assert!(Codec::try_from("\n").is_err());
}

/// Test case sensitivity
#[test]
fn test_case_sensitivity() {
    // Codec names are lowercase with hyphens
    assert!(Codec::try_from("ed25519-pub").is_ok());

    // Different case should fail
    assert!(Codec::try_from("ED25519-PUB").is_err());
    assert!(Codec::try_from("Ed25519-Pub").is_err());
    assert!(Codec::try_from("Ed25519-pub").is_err());
}

/// Test special characters in names
#[test]
fn test_special_characters() {
    // Hyphens are valid
    assert!(Codec::try_from("ed25519-pub").is_ok());

    // Other special chars should fail
    assert!(Codec::try_from("ed25519_pub").is_err());
    assert!(Codec::try_from("ed25519.pub").is_err());
    assert!(Codec::try_from("ed25519/pub").is_err());
}

/// Test encoding of identity (minimal case)
#[test]
fn test_identity_encoding() {
    let codec = Codec::Identity;
    let encoded: Vec<u8> = codec.into();

    // Identity (0x00) encodes as a single byte
    assert_eq!(encoded, vec![0x00]);

    // Decode back
    let (decoded, remaining) = Codec::try_decode_from(&encoded).unwrap();
    assert_eq!(decoded, Codec::Identity);
    assert!(remaining.is_empty());
}

/// Test decoding with no remaining bytes
#[test]
fn test_decode_exact_length() {
    let codec = Codec::Ed25519Pub;
    let encoded: Vec<u8> = codec.into();

    let (decoded, remaining) = Codec::try_decode_from(&encoded).unwrap();
    assert_eq!(decoded, codec);
    assert!(remaining.is_empty());
    assert_eq!(remaining.len(), 0);
}

/// Test decoding with trailing bytes
#[test]
fn test_decode_with_trailing() {
    let codec = Codec::Ed25519Pub;
    let mut encoded: Vec<u8> = codec.into();
    encoded.extend_from_slice(&[0xAA, 0xBB, 0xCC]);

    let (decoded, remaining) = Codec::try_decode_from(&encoded).unwrap();
    assert_eq!(decoded, codec);
    assert_eq!(remaining, &[0xAA, 0xBB, 0xCC]);
    assert_eq!(remaining.len(), 3);
}

/// Test null trait implementation
#[test]
fn test_null_trait() {
    let null_codec = Codec::null();
    assert_eq!(null_codec, Codec::Identity);
    assert!(null_codec.is_null());

    let other = Codec::Ed25519Pub;
    assert!(!other.is_null());
}

/// Test default trait implementation
#[test]
fn test_default_trait() {
    let default_codec = Codec::default();
    assert_eq!(default_codec, Codec::Identity);
    assert_eq!(default_codec, Codec::null());
}

/// Test codec equality is reflexive
#[test]
fn test_equality_reflexive() {
    let codec = Codec::Ed25519Pub;
    assert_eq!(codec, codec);
}

/// Test codec equality is symmetric
#[test]
fn test_equality_symmetric() {
    let c1 = Codec::Ed25519Pub;
    let c2 = Codec::Ed25519Pub;

    assert_eq!(c1, c2);
    assert_eq!(c2, c1);
}

/// Test codec equality is transitive
#[test]
fn test_equality_transitive() {
    let c1 = Codec::Ed25519Pub;
    let c2 = Codec::try_from(0xED).unwrap();
    let c3 = Codec::try_from("ed25519-pub").unwrap();

    assert_eq!(c1, c2);
    assert_eq!(c2, c3);
    assert_eq!(c1, c3);
}

/// Test ordering is consistent
#[test]
fn test_ordering_consistent() {
    let c1 = Codec::Identity; // 0x00
    let c2 = Codec::Sha2256; // 0x12
    let c3 = Codec::Ed25519Pub; // 0xED

    assert!(c1 < c2);
    assert!(c2 < c3);
    assert!(c1 < c3); // Transitivity

    assert!(c3 > c2);
    assert!(c2 > c1);
    assert!(c3 > c1); // Transitivity
}

/// Test copy semantics
#[test]
fn test_copy_semantics() {
    let c1 = Codec::Ed25519Pub;
    let c2 = c1; // Copy, not move

    // Both should be usable
    assert_eq!(c1.code(), 0xED);
    assert_eq!(c2.code(), 0xED);
    assert_eq!(c1, c2);
}

/// Test clone equals copy for Codec
#[test]
fn test_clone_equals_copy() {
    let c1 = Codec::Ed25519Pub;
    let c2 = c1;

    assert_eq!(c1, c2);
    assert_eq!(c1.code(), c2.code());
    assert_eq!(c1.as_str(), c2.as_str());
}

#[cfg(feature = "serde")]
mod serde_edge_cases {
    use super::*;

    /// Test serde with identity codec
    #[test]
    fn test_serde_identity() {
        let codec = Codec::Identity;

        // JSON (human readable)
        let json = serde_json::to_string(&codec).unwrap();
        assert_eq!(json, "\"identity\"");
        let decoded: Codec = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, Codec::Identity);

        // CBOR (binary)
        let bytes = serde_cbor::to_vec(&codec).unwrap();
        let decoded: Codec = serde_cbor::from_slice(&bytes).unwrap();
        assert_eq!(decoded, Codec::Identity);
    }

    /// Test serde with various codecs
    #[test]
    fn test_serde_various_codecs() {
        let codecs = vec![Codec::Identity, Codec::Sha2256, Codec::Ed25519Pub];

        for codec in codecs {
            // JSON roundtrip
            let json = serde_json::to_string(&codec).unwrap();
            let decoded: Codec = serde_json::from_str(&json).unwrap();
            assert_eq!(decoded, codec);

            // CBOR roundtrip
            let bytes = serde_cbor::to_vec(&codec).unwrap();
            let decoded: Codec = serde_cbor::from_slice(&bytes).unwrap();
            assert_eq!(decoded, codec);
        }
    }
}

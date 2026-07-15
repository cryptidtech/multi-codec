// SPDX-License-Identifier: MIT or Apache-2.0
#![allow(
    clippy::unreadable_literal,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::should_panic_without_expect,
    clippy::items_after_statements,
    clippy::match_same_arms
)]
//! Security-focused tests for multi-codec
//!
//! These tests verify that the crate properly handles malicious or
//! pathological inputs without panicking or consuming excessive resources.

use multi_codec::Codec;

/// Test that negative signed integers are rejected with proper error type
#[test]
fn test_negative_i8_rejection() {
    use multi_codec::Error;

    let result = Codec::try_from(-1i8);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::NegativeValue { .. }));

    let result = Codec::try_from(-100i8);
    assert!(result.is_err());

    let result = Codec::try_from(i8::MIN);
    assert!(result.is_err());
}

#[test]
fn test_negative_i16_rejection() {
    use multi_codec::Error;

    let result = Codec::try_from(-1i16);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::NegativeValue { .. }));

    let result = Codec::try_from(-1000i16);
    assert!(result.is_err());

    let result = Codec::try_from(i16::MIN);
    assert!(result.is_err());
}

#[test]
fn test_negative_i32_rejection() {
    use multi_codec::Error;

    let result = Codec::try_from(-1i32);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::NegativeValue { .. }));

    let result = Codec::try_from(-100000i32);
    assert!(result.is_err());

    let result = Codec::try_from(i32::MIN);
    assert!(result.is_err());
}

#[test]
fn test_negative_i64_rejection() {
    use multi_codec::Error;

    let result = Codec::try_from(-1i64);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::NegativeValue { .. }));

    let result = Codec::try_from(-1000000i64);
    assert!(result.is_err());

    let result = Codec::try_from(i64::MIN);
    assert!(result.is_err());
}

/// Test that positive signed integers work correctly
#[test]
fn test_positive_signed_integers() {
    // Identity codec has code 0
    assert!(Codec::try_from(0i8).is_ok());
    assert!(Codec::try_from(0i16).is_ok());
    assert!(Codec::try_from(0i32).is_ok());
    assert!(Codec::try_from(0i64).is_ok());

    // Test some valid positive values
    assert!(Codec::try_from(0xEDi16).is_ok()); // Ed25519Pub
}

/// Test that oversized varint data is rejected during deserialization
#[test]
fn test_dos_protection_oversized_varint() {
    // Create a byte sequence longer than maximum varint size (19 bytes)
    let oversized = vec![0xFF; 20];

    // This should fail gracefully without allocating excessive memory
    let result = Codec::try_from(oversized.as_slice());
    assert!(result.is_err(), "Should reject oversized varint");
}

/// Test that malformed varint data doesn't cause panics
#[test]
fn test_malformed_varint_handling() {
    // Varint with all continuation bits set (invalid)
    let invalid = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];

    // Should either reject or decode without panic
    let _ = Codec::try_from(invalid.as_slice());
}

/// Test that empty byte slices are handled correctly
#[test]
fn test_empty_byte_slice() {
    let empty: &[u8] = &[];
    let result = Codec::try_from(empty);
    assert!(result.is_err(), "Should reject empty byte slice");
}

/// Test that truncated varint data is rejected
#[test]
fn test_truncated_varint() {
    // Single byte with continuation bit set, but no following bytes
    let truncated = vec![0x80];
    let result = Codec::try_from(truncated.as_slice());
    assert!(result.is_err(), "Should reject truncated varint");
}

/// Test that codec values roundtrip correctly after conversion
#[test]
fn test_signed_roundtrip() {
    // Test that positive signed values roundtrip correctly
    let original = 0xEDi32; // Ed25519Pub code
    let codec = Codec::try_from(original).unwrap();
    let code: u64 = codec.into();
    assert_eq!(code, 0xED);
}

/// Test concurrent access doesn't cause issues
#[test]
fn test_concurrent_codec_usage() {
    use std::sync::Arc;
    use std::thread;

    let codec = Arc::new(Codec::Ed25519Pub);
    let mut handles = vec![];

    for _ in 0..4 {
        let codec_clone = Arc::clone(&codec);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let code = codec_clone.code();
                let name = codec_clone.as_str();
                let _encoded: Vec<u8> = (*codec_clone).into();
                assert_eq!(code, 0xED);
                assert_eq!(name, "ed25519-pub");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// Test that Hash implementation works correctly after optimization
#[test]
fn test_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let codec1 = Codec::Ed25519Pub;
    let codec2 = Codec::Ed25519Pub;

    let mut hasher1 = DefaultHasher::new();
    codec1.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    codec2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    assert_eq!(hash1, hash2, "Same codec should produce same hash");
}

/// Test that different codecs produce different hashes
#[test]
fn test_hash_uniqueness() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let codec1 = Codec::Ed25519Pub;
    let codec2 = Codec::Sha2256;

    let mut hasher1 = DefaultHasher::new();
    codec1.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    codec2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    assert_ne!(
        hash1, hash2,
        "Different codecs should produce different hashes"
    );
}

/// Test Send and Sync bounds are satisfied
#[test]
fn test_send_sync_bounds() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<Codec>();
    assert_sync::<Codec>();
}

/// Test that serde deserialization respects size limits
#[cfg(feature = "serde")]
#[test]
fn test_serde_dos_protection() {
    use serde_json::json;

    // Create a JSON array with excessive number of bytes
    let malicious_json = json!({
        "v": [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
    });

    #[derive(serde::Deserialize)]
    struct Wrapper {
        #[serde(rename = "v")]
        _v: Codec,
    }

    let result: Result<Wrapper, _> = serde_json::from_value(malicious_json);
    assert!(result.is_err(), "Should reject oversized varint in serde");
}

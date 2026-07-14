// SPDX-License-Identifier: MIT or Apache-2.0
//! Property-based tests for multi-codec using proptest
//!
//! These tests verify that invariants hold across a wide range of inputs
//! by generating random test cases and checking properties.

use multi_codec::Codec;
use multi_trait::TryDecodeFrom;
use proptest::prelude::*;

/// Property: Encoding and decoding should roundtrip for valid codecs
#[test]
fn test_encode_decode_roundtrip() {
    proptest!(|(code in 0u64..=0x0FFFFFFF)| {
        // Try to create a codec from this code
        if let Ok(codec) = Codec::try_from(code) {
            // Encode it
            let encoded: Vec<u8> = codec.into();

            // Decode it back
            let (decoded, remaining) = Codec::try_decode_from(&encoded).unwrap();

            // Should match original
            prop_assert_eq!(codec, decoded);
            prop_assert!(remaining.is_empty());

            // Code should match
            let decoded_code: u64 = decoded.into();
            prop_assert_eq!(code, decoded_code);
        }
    });
}

/// Property: String conversion should roundtrip for valid codecs
#[test]
fn test_str_conversion_roundtrip() {
    proptest!(|(code in 0u64..=0x0FFFFFFF)| {
        if let Ok(codec) = Codec::try_from(code) {
            // Convert to string
            let name = codec.as_str();

            // Convert back from string
            let decoded = Codec::try_from(name).unwrap();

            // Should match original
            prop_assert_eq!(codec, decoded);
        }
    });
}

/// Property: Code conversion should be bidirectional
#[test]
fn test_code_bidirectional() {
    proptest!(|(code in 0u64..=0x0FFFFFFF)| {
        if let Ok(codec) = Codec::try_from(code) {
            let code2: u64 = codec.into();
            prop_assert_eq!(code, code2);

            // Also test code() method
            prop_assert_eq!(code, codec.code());
        }
    });
}

/// Property: Invalid codes should always be rejected
#[test]
fn test_invalid_codes_rejected() {
    proptest!(|(code in 0u64..=u64::MAX)| {
        match Codec::try_from(code) {
            Ok(codec) => {
                // Valid code - should roundtrip
                let code2: u64 = codec.into();
                prop_assert_eq!(code, code2);
            }
            Err(_) => {
                // Invalid code - expected to fail
                // Just verify it returns an error
            }
        }
    });
}

/// Property: as_str() should always return a non-empty string
#[test]
fn test_str_never_empty() {
    proptest!(|(code in 0u64..=0x0FFFFFFF)| {
        if let Ok(codec) = Codec::try_from(code) {
            let name = codec.as_str();
            prop_assert!(!name.is_empty());
            prop_assert!(!name.is_empty());
        }
    });
}

/// Property: Debug output should contain both name and code
#[test]
fn test_debug_contains_name_and_code() {
    proptest!(|(code in 0u64..=0x0FFFFFFF)| {
        if let Ok(codec) = Codec::try_from(code) {
            let debug = format!("{:?}", codec);
            let name = codec.as_str();

            // Debug should contain the name
            prop_assert!(debug.contains(name));

            // Debug should contain hex representation
            prop_assert!(debug.contains("0x"));
        }
    });
}

/// Property: Display output should equal as_str()
#[test]
fn test_display_equals_as_str() {
    proptest!(|(code in 0u64..=0x0FFFFFFF)| {
        if let Ok(codec) = Codec::try_from(code) {
            let display = format!("{}", codec);
            let as_str = codec.as_str();
            prop_assert_eq!(display, as_str);
        }
    });
}

/// Property: Codec should be deterministic (same input = same output)
#[test]
fn test_deterministic_behavior() {
    proptest!(|(code in 0u64..=0x0FFFFFFF)| {
        if let Ok(codec1) = Codec::try_from(code) {
            let codec2 = Codec::try_from(code).unwrap();

            // Should be equal
            prop_assert_eq!(codec1, codec2);

            // Should have same code
            prop_assert_eq!(codec1.code(), codec2.code());

            // Should have same name
            prop_assert_eq!(codec1.as_str(), codec2.as_str());

            // Should encode identically
            let enc1: Vec<u8> = codec1.into();
            let enc2: Vec<u8> = codec2.into();
            prop_assert_eq!(enc1, enc2);
        }
    });
}

/// Property: Hash should be consistent for equal values
#[test]
fn test_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    proptest!(|(code in 0u64..=0x0FFFFFFF)| {
        if let Ok(codec) = Codec::try_from(code) {
            let mut hasher1 = DefaultHasher::new();
            codec.hash(&mut hasher1);
            let hash1 = hasher1.finish();

            let mut hasher2 = DefaultHasher::new();
            codec.hash(&mut hasher2);
            let hash2 = hasher2.finish();

            prop_assert_eq!(hash1, hash2);
        }
    });
}

/// Property: Ordering should be consistent with code values
#[test]
fn test_ordering_by_code() {
    proptest!(|(code1 in 0u64..=0x0FFFFFFF, code2 in 0u64..=0x0FFFFFFF)| {
        if let (Ok(codec1), Ok(codec2)) = (Codec::try_from(code1), Codec::try_from(code2)) {
            // Ordering should match code ordering
            if code1 < code2 {
                prop_assert!(codec1 < codec2);
            } else if code1 > code2 {
                prop_assert!(codec1 > codec2);
            } else {
                prop_assert_eq!(codec1, codec2);
            }
        }
    });
}

/// Property: Negative signed integers should always fail
#[test]
fn test_negative_always_fails() {
    proptest!(|(value in i64::MIN..0i64)| {
        let result = Codec::try_from(value);
        prop_assert!(result.is_err());
    });
}

/// Property: Non-negative signed integers should behave like unsigned
#[test]
fn test_nonnegative_signed_matches_unsigned() {
    proptest!(|(value in 0i64..=0x0FFFFFFF)| {
        let from_signed = Codec::try_from(value);
        let from_unsigned = Codec::try_from(value as u64);

        match (from_signed, from_unsigned) {
            (Ok(c1), Ok(c2)) => prop_assert_eq!(c1, c2),
            (Err(_), Err(_)) => {}, // Both fail - ok
            _ => prop_assert!(false, "Signed and unsigned should agree"),
        }
    });
}

/// Property: Encoded bytes should have reasonable length (1-10 bytes for u64 varint)
#[test]
fn test_encoded_length_bounded() {
    proptest!(|(code in 0u64..=0x0FFFFFFF)| {
        if let Ok(codec) = Codec::try_from(code) {
            let encoded: Vec<u8> = codec.into();
            prop_assert!(!encoded.is_empty());
            prop_assert!(encoded.len() <= 10); // u64 varint max is 10 bytes
        }
    });
}

/// Property: TryDecodeFrom should consume exactly the encoded bytes
#[test]
fn test_decode_consumes_exact_bytes() {
    proptest!(|(code in 0u64..=0x0FFFFFFF)| {
        if let Ok(codec) = Codec::try_from(code) {
            let encoded: Vec<u8> = codec.into();
            let original_len = encoded.len();

            let (decoded, remaining) = Codec::try_decode_from(&encoded).unwrap();
            prop_assert_eq!(codec, decoded);
            prop_assert_eq!(remaining.len(), 0);
            prop_assert_eq!(original_len, encoded.len() - remaining.len());
        }
    });
}

/// Property: TryDecodeFrom with extra data should leave remainder
#[test]
fn test_decode_with_trailing_data() {
    proptest!(|(code in 0u64..=0x0FFFFFFF, trailing in prop::collection::vec(any::<u8>(), 1..20))| {
        if let Ok(codec) = Codec::try_from(code) {
            let mut encoded: Vec<u8> = codec.into();
            encoded.extend_from_slice(&trailing);

            let (decoded, remaining) = Codec::try_decode_from(&encoded).unwrap();
            prop_assert_eq!(codec, decoded);
            prop_assert_eq!(remaining, trailing.as_slice());
        }
    });
}

#[cfg(feature = "serde")]
mod serde_props {
    use super::*;

    /// Property: JSON serialization should roundtrip
    #[test]
    fn test_json_roundtrip() {
        proptest!(|(code in 0u64..=0x0FFFFFFF)| {
            if let Ok(codec) = Codec::try_from(code) {
                let json = serde_json::to_string(&codec).unwrap();
                let decoded: Codec = serde_json::from_str(&json).unwrap();
                prop_assert_eq!(codec, decoded);
            }
        });
    }

    /// Property: CBOR serialization should roundtrip
    #[test]
    fn test_cbor_roundtrip() {
        proptest!(|(code in 0u64..=0x0FFFFFFF)| {
            if let Ok(codec) = Codec::try_from(code) {
                let bytes = serde_cbor::to_vec(&codec).unwrap();
                let decoded: Codec = serde_cbor::from_slice(&bytes).unwrap();
                prop_assert_eq!(codec, decoded);
            }
        });
    }
}

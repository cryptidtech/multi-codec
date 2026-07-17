// SPDX-License-Identifier: MIT or Apache-2.0
#![allow(
    clippy::unreadable_literal,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::should_panic_without_expect,
    clippy::items_after_statements,
    clippy::match_same_arms
)]
//! Integration tests for multi-codec
//!
//! These tests verify cross-module interactions and real-world usage scenarios.

use multi_codec::{Codec, Error};
use multi_trait::TryDecodeFrom;

/// Serialize a value to CBOR bytes using `ciborium`.
fn cbor_to_vec<T: serde::Serialize>(value: &T) -> Vec<u8> {
    let mut buf = Vec::new();
    ciborium::into_writer(value, &mut buf).expect("CBOR serialize");
    buf
}

/// Test complete encode/decode cycle
#[test]
fn test_full_encode_decode_cycle() {
    let original = Codec::Ed25519Pub;

    // Encode to bytes
    let bytes: Vec<u8> = original.into();
    assert!(!bytes.is_empty());

    // Decode from bytes
    let (decoded, remaining) = Codec::try_decode_from(&bytes).unwrap();
    assert_eq!(decoded, original);
    assert!(remaining.is_empty());

    // Verify code matches
    assert_eq!(decoded.code(), original.code());

    // Verify name matches
    assert_eq!(decoded.as_str(), original.as_str());
}

/// Test conversion chain: name -> codec -> code -> codec -> name
#[test]
fn test_conversion_chain() {
    // Start with name
    let original_name = "sha2-256";

    // Name -> Codec
    let codec1 = Codec::try_from(original_name).unwrap();

    // Codec -> Code
    let code: u64 = codec1.into();

    // Code -> Codec
    let codec2 = Codec::try_from(code).unwrap();

    // Codec -> Name
    let final_name = codec2.as_str();

    // Should match original
    assert_eq!(final_name, original_name);
    assert_eq!(codec1, codec2);
}

/// Test using codec in a struct
#[test]
fn test_codec_in_struct() {
    #[derive(Debug, PartialEq)]
    struct Message {
        algorithm: Codec,
        data: Vec<u8>,
    }

    let msg = Message {
        algorithm: Codec::Sha2256,
        data: vec![1, 2, 3, 4],
    };

    // Verify codec works in struct context
    assert_eq!(msg.algorithm.code(), 0x12);
    assert_eq!(msg.algorithm.as_str(), "sha2-256");

    // Clone the struct
    let msg2 = Message {
        algorithm: msg.algorithm,
        data: msg.data.clone(),
    };

    assert_eq!(msg, msg2);
}

/// Test using codec as `HashMap` key
#[test]
fn test_codec_as_hashmap_key() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(Codec::Sha2256, "SHA-256 hash");
    map.insert(Codec::Ed25519Pub, "Ed25519 public key");
    map.insert(Codec::Identity, "Identity");

    assert_eq!(map.get(&Codec::Sha2256), Some(&"SHA-256 hash"));
    assert_eq!(map.get(&Codec::Ed25519Pub), Some(&"Ed25519 public key"));
    assert_eq!(map.get(&Codec::Identity), Some(&"Identity"));
    assert_eq!(map.len(), 3);
}

/// Test using codec in a `BTreeMap` (requires Ord)
#[test]
fn test_codec_in_btreemap() {
    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();
    map.insert(Codec::Ed25519Pub, "Last");
    map.insert(Codec::Identity, "First");
    map.insert(Codec::Sha2256, "Middle");

    // Should be sorted by code value
    let keys: Vec<_> = map.keys().copied().collect();
    assert_eq!(keys[0], Codec::Identity); // 0x00
    assert_eq!(keys[1], Codec::Sha2256); // 0x12
    assert_eq!(keys[2], Codec::Ed25519Pub); // 0xED
}

/// Test codec in Option
#[test]
fn test_codec_in_option() {
    let some_codec: Option<Codec> = Some(Codec::Sha2256);
    let no_codec: Option<Codec> = None;

    assert_eq!(some_codec.map(|c| c.code()), Some(0x12));
    assert!(no_codec.is_none());

    // Map operations
    let codes: Option<u64> = some_codec.map(|c| c.code());
    assert_eq!(codes, Some(0x12));
}

/// Test codec in Result
#[test]
fn test_codec_in_result() {
    let ok_codec: Result<Codec, Error> = Ok(Codec::Sha2256);
    let err_codec: Result<Codec, Error> = Err(Error::invalid_name("bad"));

    assert_eq!(ok_codec.as_ref().unwrap().code(), 0x12);
    assert!(err_codec.is_err());

    // Map operations
    let codes: Result<u64, Error> = ok_codec.map(|c| c.code());
    assert_eq!(codes.unwrap(), 0x12);
}

/// Test codec in Vec
#[test]
fn test_codec_in_vec() {
    let codec_list = [Codec::Identity, Codec::Sha2256, Codec::Ed25519Pub];

    assert_eq!(codec_list.len(), 3);
    assert_eq!(codec_list[0], Codec::Identity);
    assert_eq!(codec_list[1].code(), 0x12);
    assert_eq!(codec_list[2].as_str(), "ed25519-pub");

    // Iteration
    let codes: Vec<u64> = codec_list.iter().map(multi_codec::Codec::code).collect();
    assert_eq!(codes, vec![0x00, 0x12, 0xED]);
}

/// Test codec with multitrait integration
#[test]
fn test_multitrait_integration() {
    let codec = Codec::Sha2256;

    // Convert to Vec<u8> which uses EncodeInto internally
    let encoded: Vec<u8> = codec.into();
    assert!(!encoded.is_empty());

    // Use TryDecodeFrom from multitrait
    let (decoded, remaining) = Codec::try_decode_from(&encoded).unwrap();
    assert_eq!(decoded, codec);
    assert!(remaining.is_empty());
}

/// Test error types in real-world scenarios
#[test]
fn test_error_handling_scenarios() {
    // Scenario 1: User provides invalid name
    match Codec::try_from("unknown-algorithm") {
        Ok(_) => panic!("Should have failed"),
        Err(e) => {
            assert_eq!(e.kind(), "InvalidName");
            assert!(e.to_string().contains("unknown-algorithm"));
        }
    }

    // Scenario 2: Parsing binary data with invalid code
    match Codec::try_from(0xDEADBEEFu64) {
        Ok(_) => panic!("Should have failed"),
        Err(e) => {
            assert_eq!(e.kind(), "InvalidValue");
            assert!(e.to_string().contains("deadbeef"));
        }
    }

    // Scenario 3: Converting negative value
    match Codec::try_from(-42i64) {
        Ok(_) => panic!("Should have failed"),
        Err(e) => {
            assert_eq!(e.kind(), "NegativeValue");
            assert!(e.to_string().contains("-42"));
        }
    }
}

/// Test building a codec registry
#[test]
fn test_codec_registry() {
    use std::collections::HashMap;

    // Build a registry mapping names to codes
    let mut registry = HashMap::new();
    let codecs = vec![
        Codec::Identity,
        Codec::Sha2256,
        Codec::Sha2512,
        Codec::Ed25519Pub,
    ];

    for codec in codecs {
        registry.insert(codec.as_str().to_string(), codec.code());
    }

    // Lookup works
    assert_eq!(registry.get("sha2-256"), Some(&0x12));
    assert_eq!(registry.get("ed25519-pub"), Some(&0xED));

    // Unknown codec
    assert_eq!(registry.get("unknown"), None);
}

/// Test sequential encoding of multiple codecs
#[test]
fn test_sequential_encoding() {
    use multi_trait::EncodeIntoBuffer;

    let codecs = vec![Codec::Identity, Codec::Sha2256, Codec::Ed25519Pub];

    let mut buffer = Vec::new();
    for codec in &codecs {
        let code: u64 = (*codec).into();
        code.encode_into_buffer(&mut buffer);
    }

    // Decode them back
    let mut remaining = buffer.as_slice();
    for codec in &codecs {
        let (code, rest) = u64::try_decode_from(remaining).unwrap();
        let decoded = Codec::try_from(code).unwrap();
        assert_eq!(decoded, *codec);
        remaining = rest;
    }

    assert!(remaining.is_empty());
}

/// Test codec with pattern matching
#[test]
fn test_pattern_matching() {
    let codec = Codec::Sha2256;

    match codec {
        Codec::Identity => panic!("Wrong variant"),
        Codec::Sha2256 => {
            assert_eq!(codec.code(), 0x12);
        }
        _ => panic!("Wrong variant"),
    }
}

/// Test codec display format
#[test]
fn test_display_format() {
    let codec = Codec::Ed25519Pub;

    // Display should show the name
    let display = format!("{codec}");
    assert_eq!(display, "ed25519-pub");

    // Debug should show both name and code
    let debug = format!("{codec:?}");
    assert!(debug.contains("ed25519-pub"));
    assert!(debug.contains("0xed"));
}

#[cfg(feature = "serde")]
mod serde_integration {
    use super::*;
    use serde::{Deserialize, Serialize};

    /// Test codec in a serde-enabled struct
    #[test]
    fn test_codec_in_serde_struct() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct CryptoOperation {
            algorithm: Codec,
            key_id: u64,
            data: Vec<u8>,
        }

        let op = CryptoOperation {
            algorithm: Codec::Ed25519Pub,
            key_id: 42,
            data: vec![1, 2, 3, 4, 5],
        };

        // JSON roundtrip
        let json = serde_json::to_string(&op).unwrap();
        let decoded: CryptoOperation = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, op);

        // Verify algorithm preserved
        assert_eq!(decoded.algorithm, Codec::Ed25519Pub);
        assert_eq!(decoded.algorithm.code(), 0xED);

        // CBOR roundtrip
        let bytes = cbor_to_vec(&op);
        let decoded: CryptoOperation = ciborium::from_reader(bytes.as_slice()).unwrap();
        assert_eq!(decoded, op);
    }

    /// Test codec in nested structures
    #[test]
    fn test_codec_in_nested_struct() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Inner {
            hash_algorithm: Codec,
        }

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Outer {
            signature_algorithm: Codec,
            hash_config: Inner,
        }

        let config = Outer {
            signature_algorithm: Codec::Ed25519Pub,
            hash_config: Inner {
                hash_algorithm: Codec::Sha2256,
            },
        };

        // JSON roundtrip
        let json = serde_json::to_string(&config).unwrap();
        let decoded: Outer = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, config);

        // CBOR roundtrip
        let bytes = cbor_to_vec(&config);
        let decoded: Outer = ciborium::from_reader(bytes.as_slice()).unwrap();
        assert_eq!(decoded, config);
    }
}

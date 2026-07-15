// SPDX-License-Identifier: MIT or Apache-2.0
#![allow(
    clippy::unreadable_literal,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::should_panic_without_expect,
    clippy::items_after_statements,
    clippy::match_same_arms
)]
//! Tests for error handling and error types

use multi_codec::{Codec, Error};

/// Test `InvalidName` error creation and properties
#[test]
fn test_invalid_name_error_properties() {
    let err = Error::invalid_name("bad-codec");

    // Check error kind
    assert_eq!(err.kind(), "InvalidName");

    // Check context contains the name
    let context = err.context();
    assert!(context.contains("bad-codec"));

    // Check error message
    let msg = err.to_string();
    assert!(msg.contains("bad-codec"));
    assert!(msg.contains("multicodec"));
}

/// Test `InvalidValue` error creation and properties
#[test]
fn test_invalid_value_error_properties() {
    let err = Error::invalid_value(0xDEADBEEF);

    // Check error kind
    assert_eq!(err.kind(), "InvalidValue");

    // Check context contains the value
    let context = err.context();
    assert!(context.contains("deadbeef"));

    // Check error message
    let msg = err.to_string();
    assert!(msg.contains("0xdeadbeef"));
    assert!(msg.contains("multicodec"));
}

/// Test `NegativeValue` error creation and properties
#[test]
fn test_negative_value_error_properties() {
    let err = Error::negative_value(-42);

    // Check error kind
    assert_eq!(err.kind(), "NegativeValue");

    // Check context contains the value
    let context = err.context();
    assert!(context.contains("-42"));

    // Check error message
    let msg = err.to_string();
    assert!(msg.contains("-42"));
    assert!(msg.contains("negative"));
}

/// Test that errors from `TryFrom` implementations have correct types
#[test]
fn test_tryfrom_str_error_type() {
    let result = Codec::try_from("nonexistent-codec");
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(matches!(err, Error::InvalidName { .. }));
    assert_eq!(err.kind(), "InvalidName");
}

#[test]
fn test_tryfrom_u64_error_type() {
    let result = Codec::try_from(0xDEADBEEFu64);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(matches!(err, Error::InvalidValue { .. }));
    assert_eq!(err.kind(), "InvalidValue");
}

#[test]
fn test_tryfrom_negative_error_type() {
    let result = Codec::try_from(-1i64);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(matches!(err, Error::NegativeValue { .. }));
    assert_eq!(err.kind(), "NegativeValue");
}

/// Test error messages provide helpful guidance
#[test]
fn test_error_messages_provide_guidance() {
    // InvalidName should reference the multicodec table
    let err = Error::invalid_name("unknown");
    let msg = err.to_string();
    assert!(msg.contains("github.com") || msg.contains("table.csv"));

    // InvalidValue should reference the multicodec table
    let err = Error::invalid_value(999999);
    let msg = err.to_string();
    assert!(msg.contains("github.com") || msg.contains("table.csv"));

    // NegativeValue should suggest using unsigned types
    let err = Error::negative_value(-1);
    let msg = err.to_string();
    assert!(msg.contains("unsigned") || msg.contains("non-negative"));
}

/// Test error kind uniqueness
#[test]
fn test_error_kinds_are_unique() {
    let errors = [
        Error::invalid_name("test"),
        Error::invalid_value(123),
        Error::negative_value(-1),
    ];

    let kinds: Vec<_> = errors.iter().map(multi_codec::Error::kind).collect();

    // All kinds should be unique
    for (i, kind1) in kinds.iter().enumerate() {
        for (j, kind2) in kinds.iter().enumerate() {
            if i != j {
                assert_ne!(kind1, kind2, "Error kinds should be unique");
            }
        }
    }
}

/// Test error context is useful for debugging
#[test]
fn test_error_context_for_debugging() {
    let err = Error::invalid_name("test-codec");
    let context = err.context();
    assert!(context.contains("test-codec"));
    assert!(!context.is_empty());

    let err = Error::invalid_value(0x123);
    let context = err.context();
    assert!(context.contains("0x123") || context.contains("123") || context.contains("291"));
    assert!(!context.is_empty());

    let err = Error::negative_value(-100);
    let context = err.context();
    assert!(context.contains("-100"));
    assert!(!context.is_empty());
}

/// Test that error types can be matched on
#[test]
fn test_error_pattern_matching() {
    let err = Error::invalid_name("test");
    match err {
        Error::InvalidName { name } => assert_eq!(name, "test"),
        _ => panic!("Wrong error variant"),
    }

    let err = Error::invalid_value(42);
    match err {
        Error::InvalidValue { code } => assert_eq!(code, 42),
        _ => panic!("Wrong error variant"),
    }

    let err = Error::negative_value(-5);
    match err {
        Error::NegativeValue { value } => assert_eq!(value, -5),
        _ => panic!("Wrong error variant"),
    }
}

/// Test error is Send + Sync
#[test]
fn test_error_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<Error>();
    assert_sync::<Error>();
}

/// Test error implements `std::error::Error`
#[test]
fn test_error_trait_impl() {
    use std::error::Error as StdError;

    let err = Error::invalid_name("test");
    let _: &dyn StdError = &err;

    // Multitrait errors are transparent, so they are the source themselves
    let mt_err = multi_trait::Error::InsufficientData {
        expected: 10,
        actual: 5,
    };
    let err = Error::Multitrait(mt_err);
    // Transparent errors don't have a separate source, they ARE the source
    let _: &dyn StdError = &err;
}

/// Test that error display is informative
#[test]
fn test_error_display_informative() {
    let err = Error::invalid_name("bad-name");
    let display = format!("{err}");
    assert!(!display.is_empty());
    assert!(display.len() > 20); // Should be a full sentence, not just a few words

    let err = Error::invalid_value(999);
    let display = format!("{err}");
    assert!(!display.is_empty());
    assert!(display.len() > 20);

    let err = Error::negative_value(-10);
    let display = format!("{err}");
    assert!(!display.is_empty());
    assert!(display.len() > 20);
}

/// Test error debug output
#[test]
fn test_error_debug_output() {
    let err = Error::invalid_name("test");
    let debug = format!("{err:?}");
    assert!(!debug.is_empty());
    assert!(debug.contains("InvalidName") || debug.contains("test"));
}

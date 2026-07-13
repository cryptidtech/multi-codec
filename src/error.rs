// SPDX-License-Identifier: MIT or Apache-2.0
//! Error types for multi-codec
//!
//! This module provides comprehensive error types with backtrace support
//! for debugging and detailed error context for better user experience.

/// Errors produced by the multi-codec crate
///
/// All error variants include contextual information to help with debugging
/// and provide actionable error messages.
///
/// # Backtraces
///
/// When the `RUST_BACKTRACE=1` environment variable is set, errors will
/// capture and display stack backtraces showing where the error occurred.
///
/// # Examples
///
/// ```
/// use multi_codec::{Codec, Error};
///
/// // Invalid name error includes the provided name
/// let result = Codec::try_from("invalid-codec-name");
/// assert!(matches!(result, Err(Error::InvalidName { .. })));
///
/// // Invalid value error includes the provided code
/// let result = Codec::try_from(0xDEADBEEFu64);
/// assert!(matches!(result, Err(Error::InvalidValue { .. })));
/// ```
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Error from bs-multitrait crate
    ///
    /// This error occurs when encoding or decoding operations from the
    /// bs-multitrait crate fail, typically due to malformed varint data.
    #[error(transparent)]
    Multitrait(#[from] multi_trait::Error),

    /// Invalid codec name provided
    ///
    /// The provided name does not match any known multicodec identifier.
    ///
    /// # Context
    ///
    /// - `name`: The invalid name that was provided
    ///
    /// # Resolution
    ///
    /// Check the [Multicodec Table](https://github.com/multiformats/multicodec/blob/master/table.csv)
    /// for valid codec names. Names are case-sensitive and must match exactly.
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::Codec;
    ///
    /// // This will fail - name doesn't exist
    /// let result = Codec::try_from("invalid-name");
    /// assert!(result.is_err());
    ///
    /// // This works - correct name
    /// let codec = Codec::try_from("ed25519-pub").unwrap();
    /// ```
    #[error(
        "Invalid multicodec name: '{name}'\n\
             The name '{name}' is not a recognized multicodec identifier.\n\
             See https://github.com/multiformats/multicodec/blob/master/table.csv for valid names."
    )]
    InvalidName {
        /// The invalid codec name that was provided
        name: String,
    },

    /// Invalid codec value provided
    ///
    /// The provided numeric code does not correspond to any known multicodec.
    ///
    /// # Context
    ///
    /// - `code`: The invalid code value that was provided
    ///
    /// # Resolution
    ///
    /// Check the [Multicodec Table](https://github.com/multiformats/multicodec/blob/master/table.csv)
    /// for valid codec codes. Codes must match exactly.
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::Codec;
    ///
    /// // This will fail - code doesn't exist
    /// let result = Codec::try_from(0xDEADBEEFu64);
    /// assert!(result.is_err());
    ///
    /// // This works - valid code
    /// let codec = Codec::try_from(0xEDu64).unwrap();
    /// assert_eq!(codec, Codec::Ed25519Pub);
    /// ```
    #[error(
        "Invalid multicodec code: 0x{code:x} ({code})\n\
             The code 0x{code:x} is not a recognized multicodec value.\n\
             See https://github.com/multiformats/multicodec/blob/master/table.csv for valid codes."
    )]
    InvalidValue {
        /// The invalid code value that was provided
        code: u64,
    },

    /// Negative value provided for signed integer conversion
    ///
    /// Codec codes must be non-negative. Signed integers with negative
    /// values cannot be converted to codec identifiers.
    ///
    /// # Context
    ///
    /// - `value`: The negative value that was provided
    ///
    /// # Resolution
    ///
    /// Use only non-negative values (>= 0) when converting from signed
    /// integer types to Codec. Consider using unsigned integer types
    /// (u8, u16, u32, u64) instead if the value is always positive.
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::Codec;
    ///
    /// // This will fail - negative value
    /// let result = Codec::try_from(-1i64);
    /// assert!(result.is_err());
    ///
    /// // This works - positive value
    /// let codec = Codec::try_from(0xEDi64).unwrap();
    /// assert_eq!(codec, Codec::Ed25519Pub);
    /// ```
    #[error(
        "Cannot convert negative value {value} to multicodec\n\
             Codec codes must be non-negative integers. The value {value} is negative.\n\
             Use a non-negative value or an unsigned integer type (u8, u16, u32, u64)."
    )]
    NegativeValue {
        /// The negative value that was provided
        value: i64,
    },
}

impl Error {
    /// Create an InvalidName error with the given name
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::Error;
    ///
    /// let err = Error::invalid_name("unknown-codec");
    /// assert!(matches!(err, Error::InvalidName { .. }));
    /// ```
    pub fn invalid_name(name: impl Into<String>) -> Self {
        Self::InvalidName { name: name.into() }
    }

    /// Create an InvalidValue error with the given code
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::Error;
    ///
    /// let err = Error::invalid_value(0xDEADBEEF);
    /// assert!(matches!(err, Error::InvalidValue { .. }));
    /// ```
    pub fn invalid_value(code: u64) -> Self {
        Self::InvalidValue { code }
    }

    /// Create a NegativeValue error with the given value
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::Error;
    ///
    /// let err = Error::negative_value(-100);
    /// assert!(matches!(err, Error::NegativeValue { .. }));
    /// ```
    pub fn negative_value(value: i64) -> Self {
        Self::NegativeValue { value }
    }

    /// Get the error kind as a string
    ///
    /// Returns a short identifier for the error type, useful for
    /// programmatic error handling and logging.
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::Error;
    ///
    /// let err = Error::invalid_name("bad");
    /// assert_eq!(err.kind(), "InvalidName");
    ///
    /// let err = Error::invalid_value(999);
    /// assert_eq!(err.kind(), "InvalidValue");
    ///
    /// let err = Error::negative_value(-1);
    /// assert_eq!(err.kind(), "NegativeValue");
    /// ```
    pub fn kind(&self) -> &str {
        match self {
            Self::Multitrait(_) => "Multitrait",
            Self::InvalidName { .. } => "InvalidName",
            Self::InvalidValue { .. } => "InvalidValue",
            Self::NegativeValue { .. } => "NegativeValue",
        }
    }

    /// Get additional context about the error
    ///
    /// Returns human-readable context information that can help
    /// diagnose the error.
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::Error;
    ///
    /// let err = Error::invalid_name("bad-codec");
    /// let context = err.context();
    /// assert!(context.contains("bad-codec"));
    /// ```
    pub fn context(&self) -> String {
        match self {
            Self::Multitrait(e) => format!("Multitrait error: {}", e),
            Self::InvalidName { name } => format!("Invalid name: '{}'", name),
            Self::InvalidValue { code } => format!("Invalid code: 0x{:x} ({})", code, code),
            Self::NegativeValue { value } => format!("Negative value: {}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_name_error() {
        let err = Error::invalid_name("bad-codec");
        assert!(matches!(err, Error::InvalidName { .. }));
        assert_eq!(err.kind(), "InvalidName");
        assert!(err.context().contains("bad-codec"));
        assert!(err.to_string().contains("bad-codec"));
    }

    #[test]
    fn test_invalid_value_error() {
        let err = Error::invalid_value(0xDEAD);
        assert!(matches!(err, Error::InvalidValue { .. }));
        assert_eq!(err.kind(), "InvalidValue");
        assert!(err.context().contains("dead"));
        assert!(err.to_string().contains("0xdead"));
    }

    #[test]
    fn test_negative_value_error() {
        let err = Error::negative_value(-42);
        assert!(matches!(err, Error::NegativeValue { .. }));
        assert_eq!(err.kind(), "NegativeValue");
        assert!(err.context().contains("-42"));
        assert!(err.to_string().contains("-42"));
    }

    #[test]
    fn test_error_messages_have_context() {
        let err = Error::invalid_name("unknown");
        let msg = err.to_string();
        assert!(msg.contains("unknown"));
        assert!(msg.contains("multicodec"));

        let err = Error::invalid_value(99999);
        let msg = err.to_string();
        assert!(msg.contains("99999"));
        assert!(msg.contains("0x"));
    }

    #[test]
    fn test_error_kind_uniqueness() {
        let errors = [
            Error::invalid_name("test"),
            Error::invalid_value(123),
            Error::negative_value(-1),
        ];

        let kinds: Vec<_> = errors.iter().map(|e| e.kind()).collect();
        assert_eq!(kinds.len(), 3);
        assert!(kinds
            .iter()
            .all(|k| kinds.iter().filter(|x| *x == k).count() == 1));
    }
}

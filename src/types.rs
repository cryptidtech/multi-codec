// SPDX-License-Identifier: MIT or Apache-2.0
//! Type-safe wrappers for multicodec identifiers
//!
//! This module provides newtype wrappers that prevent mixing up raw numeric
//! codes and string names, following Rust best practices for type safety.
//!
//! # Overview
//!
//! Instead of using raw `u64` values and `String`s directly, these newtypes
//! provide:
//! - **Type safety**: Can't accidentally use a code where a name is expected
//! - **Self-documentation**: Code is clearer about intent
//! - **Validation**: Can add validation logic in constructors
//! - **API evolution**: Can add methods without changing function signatures
//!
//! # Examples
//!
//! ```
//! use multi_codec::types::{CodecCode, CodecName};
//!
//! // Type-safe codec code
//! let code = CodecCode::new(0xED);
//! assert_eq!(code.get(), 0xED);
//! assert_eq!(code.to_string(), "0xed");
//!
//! // Type-safe codec name
//! let name = CodecName::new("ed25519-pub");
//! assert_eq!(name.as_str(), "ed25519-pub");
//! ```

#[cfg(not(feature = "std"))]
use alloc::format;
#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};
use core::fmt;

/// A codec code value (0x00 - 0xFFFFFFFFFFFFFFFF)
///
/// This newtype wrapper provides type safety when working with raw codec values.
/// It prevents accidentally mixing up codes with other numeric types and provides
/// a clear, self-documenting API.
///
/// # Examples
///
/// ```
/// use multi_codec::types::CodecCode;
///
/// // Create a code
/// let code = CodecCode::new(0xED);
/// assert_eq!(code.get(), 0xED);
///
/// // Display as hex
/// assert_eq!(code.to_string(), "0xed");
///
/// // Conversions
/// let code: CodecCode = 237u64.into();
/// assert_eq!(u64::from(code), 237);
/// ```
///
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CodecCode(u64);

impl CodecCode {
    /// Create a new `CodecCode` from a u64 value
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::types::CodecCode;
    ///
    /// let code = CodecCode::new(0xED);
    /// assert_eq!(code.get(), 0xED);
    /// ```
    #[must_use]
    pub const fn new(code: u64) -> Self {
        Self(code)
    }

    /// Get the raw u64 value
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::types::CodecCode;
    ///
    /// let code = CodecCode::new(0x12);
    /// assert_eq!(code.get(), 0x12);
    /// ```
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Get the value as a hexadecimal string
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::types::CodecCode;
    ///
    /// let code = CodecCode::new(0xED);
    /// assert_eq!(code.as_hex(), "0xed");
    /// ```
    #[must_use]
    pub fn as_hex(&self) -> String {
        format!("0x{:x}", self.0)
    }

    /// Check if this is the null/identity codec (0x00)
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::types::CodecCode;
    ///
    /// let identity = CodecCode::new(0x00);
    /// assert!(identity.is_identity());
    ///
    /// let other = CodecCode::new(0xED);
    /// assert!(!other.is_identity());
    /// ```
    #[must_use]
    pub const fn is_identity(&self) -> bool {
        self.0 == 0
    }
}

impl From<u64> for CodecCode {
    fn from(code: u64) -> Self {
        Self(code)
    }
}

impl From<CodecCode> for u64 {
    fn from(code: CodecCode) -> Self {
        code.0
    }
}

impl fmt::Display for CodecCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

impl fmt::LowerHex for CodecCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl fmt::UpperHex for CodecCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

/// A codec name string
///
/// This newtype wrapper provides type safety when working with codec names.
/// It prevents accidentally using arbitrary strings where codec names are expected
/// and provides a clear, self-documenting API.
///
/// # Examples
///
/// ```
/// use multi_codec::types::CodecName;
///
/// // Create a name
/// let name = CodecName::new("ed25519-pub");
/// assert_eq!(name.as_str(), "ed25519-pub");
///
/// // Display
/// assert_eq!(name.to_string(), "ed25519-pub");
///
/// // Conversions
/// let name: CodecName = "sha2-256".to_string().into();
/// assert_eq!(name.as_str(), "sha2-256");
/// ```
///
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CodecName(String);

impl CodecName {
    /// Create a new `CodecName` from a string
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::types::CodecName;
    ///
    /// let name = CodecName::new("ed25519-pub");
    /// assert_eq!(name.as_str(), "ed25519-pub");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Get the name as a string slice
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::types::CodecName;
    ///
    /// let name = CodecName::new("sha2-256");
    /// assert_eq!(name.as_str(), "sha2-256");
    /// ```
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if this is the identity codec name
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::types::CodecName;
    ///
    /// let identity = CodecName::new("identity");
    /// assert!(identity.is_identity());
    ///
    /// let other = CodecName::new("sha2-256");
    /// assert!(!other.is_identity());
    /// ```
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.0 == "identity"
    }

    /// Get the length of the name in bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::types::CodecName;
    ///
    /// let name = CodecName::new("sha2-256");
    /// assert_eq!(name.len(), 8);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if the name is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_codec::types::CodecName;
    ///
    /// let name = CodecName::new("");
    /// assert!(name.is_empty());
    ///
    /// let name = CodecName::new("sha2-256");
    /// assert!(!name.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<String> for CodecName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl From<&str> for CodecName {
    fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl From<CodecName> for String {
    fn from(name: CodecName) -> Self {
        name.0
    }
}

impl AsRef<str> for CodecName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CodecName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::items_after_statements)]
    use super::*;

    #[test]
    fn test_codec_code_new() {
        let code = CodecCode::new(0xED);
        assert_eq!(code.get(), 0xED);
    }

    #[test]
    fn test_codec_code_conversions() {
        let code = CodecCode::from(0x12u64);
        assert_eq!(u64::from(code), 0x12);
    }

    #[test]
    fn test_codec_code_display() {
        let code = CodecCode::new(0xED);
        assert_eq!(code.to_string(), "0xed");
        assert_eq!(format!("{code:x}"), "ed");
        assert_eq!(format!("{code:X}"), "ED");
    }

    #[test]
    fn test_codec_code_as_hex() {
        let code = CodecCode::new(0xABCD);
        assert_eq!(code.as_hex(), "0xabcd");
    }

    #[test]
    fn test_codec_code_is_identity() {
        let identity = CodecCode::new(0x00);
        assert!(identity.is_identity());

        let other = CodecCode::new(0x12);
        assert!(!other.is_identity());
    }

    #[test]
    fn test_codec_code_equality() {
        let code1 = CodecCode::new(0xED);
        let code2 = CodecCode::new(0xED);
        let code3 = CodecCode::new(0x12);

        assert_eq!(code1, code2);
        assert_ne!(code1, code3);
    }

    #[test]
    fn test_codec_code_ordering() {
        let code1 = CodecCode::new(0x10);
        let code2 = CodecCode::new(0x20);

        assert!(code1 < code2);
        assert!(code2 > code1);
    }

    #[test]
    fn test_codec_code_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let code = CodecCode::new(0xED);
        map.insert(code, "ed25519-pub");

        assert_eq!(map.get(&code), Some(&"ed25519-pub"));
    }

    #[test]
    fn test_codec_name_new() {
        let name = CodecName::new("ed25519-pub");
        assert_eq!(name.as_str(), "ed25519-pub");
    }

    #[test]
    fn test_codec_name_conversions() {
        let name = CodecName::from("sha2-256".to_string());
        assert_eq!(String::from(name), "sha2-256");

        let name = CodecName::from("blake3");
        assert_eq!(name.as_str(), "blake3");
    }

    #[test]
    fn test_codec_name_display() {
        let name = CodecName::new("ed25519-pub");
        assert_eq!(name.to_string(), "ed25519-pub");
    }

    #[test]
    fn test_codec_name_as_ref() {
        let name = CodecName::new("sha2-256");
        let s: &str = name.as_ref();
        assert_eq!(s, "sha2-256");
    }

    #[test]
    fn test_codec_name_is_identity() {
        let identity = CodecName::new("identity");
        assert!(identity.is_identity());

        let other = CodecName::new("sha2-256");
        assert!(!other.is_identity());
    }

    #[test]
    fn test_codec_name_len() {
        let name = CodecName::new("ed25519-pub");
        assert_eq!(name.len(), 11);

        let empty = CodecName::new("");
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn test_codec_name_is_empty() {
        let empty = CodecName::new("");
        assert!(empty.is_empty());

        let name = CodecName::new("sha2-256");
        assert!(!name.is_empty());
    }

    #[test]
    fn test_codec_name_equality() {
        let name1 = CodecName::new("ed25519-pub");
        let name2 = CodecName::new("ed25519-pub");
        let name3 = CodecName::new("sha2-256");

        assert_eq!(name1, name2);
        assert_ne!(name1, name3);
    }

    #[test]
    fn test_codec_name_ordering() {
        let name1 = CodecName::new("aaa");
        let name2 = CodecName::new("bbb");

        assert!(name1 < name2);
        assert!(name2 > name1);
    }

    #[test]
    fn test_codec_name_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let name = CodecName::new("ed25519-pub");
        map.insert(name.clone(), 0xED);

        assert_eq!(map.get(&name), Some(&0xED));
    }

    #[test]
    fn test_newtypes_are_copy_or_clone() {
        // CodecCode should be Copy (contains only u64)
        fn assert_copy<T: Copy>() {}
        assert_copy::<CodecCode>();

        // CodecName should be Clone (contains String)
        fn assert_clone<T: Clone>() {}
        assert_clone::<CodecName>();
    }

    #[test]
    fn test_newtypes_are_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<CodecCode>();
        assert_sync::<CodecCode>();
        assert_send::<CodecName>();
        assert_sync::<CodecName>();
    }
}

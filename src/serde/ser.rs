// SPDX-License-Identifier: MIT or Apache-2.0
use crate::Codec;
use multi_trait::EncodeIntoArray;
use serde::ser;

/// Serialize instances of [`crate::prelude::Codec`] into varuint encoded bytes
impl ser::Serialize for Codec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if serializer.is_human_readable() {
            let s: &str = (*self).into();
            serializer.serialize_str(s)
        } else {
            // Use stack allocation instead of heap allocation for better performance
            let code: u64 = (*self).into();
            let (buf, len) = code.encode_into_array();
            serializer.serialize_bytes(&buf[..len])
        }
    }
}

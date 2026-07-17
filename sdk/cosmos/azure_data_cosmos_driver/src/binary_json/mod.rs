// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos **binary JSON** codec.
//!
//! Cosmos binary JSON is a tagged byte stream that the service can persist and
//! transmit in place of UTF-8 text JSON. A buffer begins with the preamble byte
//! [`PREAMBLE`] (`0x80`); because no valid UTF-8 text JSON document starts with
//! `0x80`, the first byte unambiguously distinguishes binary from text. Each
//! value is introduced by a single **type-marker** byte (see [`markers`]) that
//! selects how the following bytes are interpreted.
//!
//! This module is schema-agnostic: it operates purely on bytes and either
//! [`serde_json::Value`] (via [`decode`] / [`encode`]) or `serde` types
//! directly (via [`from_slice`] / [`to_vec`]).
//!
//! # Reading and writing
//!
//! - Read binary bytes into a typed value with [`from_slice`], or into a
//!   [`serde_json::Value`] with [`decode`].
//! - Write a typed value to binary bytes with [`to_vec`], or a
//!   [`serde_json::Value`] with [`encode`].
//!
//! The decoder accepts every wire form the service can emit. The encoder emits
//! a valid subset of those forms rather than the most compact encoding; because
//! the service accepts the verbose form, an encode/decode round-trip preserves
//! the original value.
//!
//! # Reference
//!
//! The wire constants in [`markers`] match the service byte-for-byte and are
//! transcribed from the .NET reference implementation
//! `Microsoft.Azure.Cosmos/src/Json/JsonBinaryEncoding.TypeMarker.cs`.

pub mod de;
pub mod error;
pub mod markers;
pub mod reader;
pub mod ser;
pub mod system_strings;
pub mod writer;

#[cfg(test)]
mod fuzz_tests;
#[cfg(test)]
mod vectors;

pub use de::from_slice;
pub use error::{BinaryError, Result};
pub use reader::decode;
pub use ser::to_vec;
pub use writer::encode;

/// The Cosmos binary JSON preamble byte.
///
/// Every binary JSON buffer starts with this byte. It is the basis for
/// first-byte auto-detection ([`is_binary`]): no UTF-8 text JSON document can
/// begin with `0x80` (it is a continuation byte), so its presence reliably
/// distinguishes a binary buffer from a text one.
pub const PREAMBLE: u8 = 0x80;

/// Returns `true` if `buffer` appears to be Cosmos binary JSON.
///
/// Detection is the single-byte test described in the spec: a buffer is binary
/// iff its first byte is the [`PREAMBLE`]. An empty buffer is not binary.
///
/// This is intentionally independent of any HTTP content negotiation so the
/// response path can decode binary bodies even when headers are absent or
/// unexpected.
///
/// # Examples
///
/// ```
/// use azure_data_cosmos_driver::binary_json::{is_binary, PREAMBLE};
///
/// assert!(is_binary(&[PREAMBLE, 0xD2])); // binary `true`
/// assert!(!is_binary(b"{\"id\":\"1\"}")); // text JSON
/// assert!(!is_binary(&[])); // empty
/// ```
pub fn is_binary(buffer: &[u8]) -> bool {
    buffer.first() == Some(&PREAMBLE)
}

/// Transcodes a Cosmos binary JSON buffer to UTF-8 **text** JSON.
///
/// This is the driver-side conversion used when an upstream SDK/app wants to
/// deal only with text JSON while still keeping the wire binary (efficient RUs
/// and network bandwidth): the request and the service response stay binary,
/// and the driver converts the binary response to text before handing it back.
///
/// Behavior:
///
/// - If `buffer` is Cosmos binary JSON (begins with the [`PREAMBLE`]), it is
///   decoded to a [`serde_json::Value`] and re-serialized as compact UTF-8 text
///   JSON (matching `serde_json::to_vec`).
/// - If `buffer` is already text JSON (or empty), it is returned **unchanged**
///   so the conversion is safe to apply unconditionally on a response whose
///   format was negotiated but not guaranteed.
///
/// # Errors
///
/// Returns a [`BinaryError`] if `buffer` is binary but malformed, or if the
/// decoded value cannot be re-serialized as JSON.
pub fn transcode_to_text(buffer: &[u8]) -> Result<Vec<u8>> {
    if !is_binary(buffer) {
        // Already text (or empty): nothing to convert.
        return Ok(buffer.to_vec());
    }
    let value = decode(buffer)?;
    serde_json::to_vec(&value)
        .map_err(|e| BinaryError::Custom(format!("failed to re-serialize decoded value: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preamble_is_0x80() {
        assert_eq!(PREAMBLE, 0x80);
        // The preamble shares its value with the start of the encoded-length
        // string range; that range begins at the same byte by design.
        assert_eq!(PREAMBLE, markers::ENCODED_STRING_LENGTH_MIN);
    }

    #[test]
    fn detects_binary_by_preamble() {
        assert!(is_binary(&[PREAMBLE]));
        assert!(is_binary(&[PREAMBLE, markers::TRUE]));
    }

    #[test]
    fn rejects_text_and_empty() {
        assert!(!is_binary(b"{}"));
        assert!(!is_binary(b"[1,2,3]"));
        assert!(!is_binary(b"\"hello\""));
        assert!(!is_binary(&[]));
        // A different leading byte is not binary even if 0x80 appears later.
        assert!(!is_binary(&[0x00, PREAMBLE]));
    }

    #[test]
    fn transcode_binary_to_text_produces_equivalent_json() {
        // A binary buffer transcodes to the same JSON serde_json would emit.
        let value = serde_json::json!({
            "id": "doc-1",
            "n": 42,
            "flag": true,
            "nested": { "arr": [1, 2, 3], "s": "café" },
        });
        let binary = encode(&value);
        assert!(is_binary(&binary));

        let text = transcode_to_text(&binary).unwrap();
        assert!(!is_binary(&text), "transcoded output must be text");

        // Bytes match serde_json::to_vec of the same value, and re-parse equal.
        assert_eq!(text, serde_json::to_vec(&value).unwrap());
        let reparsed: serde_json::Value = serde_json::from_slice(&text).unwrap();
        assert_eq!(reparsed, value);
    }

    #[test]
    fn transcode_passes_text_through_unchanged() {
        // A text buffer is returned byte-for-byte unchanged.
        let text = br#"{"id":"1","n":7}"#;
        assert_eq!(transcode_to_text(text).unwrap(), text);
    }

    #[test]
    fn transcode_passes_empty_through_unchanged() {
        assert_eq!(transcode_to_text(&[]).unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn transcode_errors_on_malformed_binary() {
        // A lone preamble is not a complete value.
        assert!(transcode_to_text(&[PREAMBLE]).is_err());
    }
}

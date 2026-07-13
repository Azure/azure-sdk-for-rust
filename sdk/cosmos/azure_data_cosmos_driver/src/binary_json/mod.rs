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
//! This module is **schema-agnostic**: it operates purely on bytes and either
//! `serde_json::Value` (via [`decode`] / [`encode`]) or `serde` types directly
//! (via [`from_slice`] / [`to_vec`]). It does not know about Cosmos item
//! schemas, matching the driver's schema-agnostic data-plane principle.
//!
//! # Encode/decode asymmetry
//!
//! The **decoder must be complete** — it parses untrusted service output and
//! must handle every form the service can emit (literal ints, system *and*
//! user strings, reference strings, base64/GUID/compressed strings, every
//! number width, and uniform number arrays). The **encoder may be
//! minimal-but-valid** — to produce a correct (not size-optimal) buffer it only
//! needs encoded-length / length-prefixed strings, a few number forms,
//! length+count containers, and the null/bool singletons; the service accepts
//! the verbose form.
//!
//! # Reference
//!
//! The wire constants in [`markers`] are transcribed from the .NET reference
//! implementation
//! `Microsoft.Azure.Cosmos/src/Json/JsonBinaryEncoding.TypeMarker.cs` and must
//! match the service byte-for-byte. See the binary-encoding spec
//! (`docs/BINARY_ENCODING_SPEC.md`) for the full design and phased plan.
//!
//! > **Status:** binary encoding is **implemented in both directions**.
//! > - **Read path:** the driver's `ResponseBody::into_single` / `into_items`
//! >   auto-detect the `0x80` preamble and decode binary buffers through the
//! >   native serde deserializer [`from_slice`] (no intermediate
//! >   `serde_json::Value`), leaving the text path unchanged. [`decode`]
//! >   (binary → [`serde_json::Value`]) remains the complete reference decoder
//! >   and the fuzz / parity oracle, and is the fallback `from_slice` uses for
//! >   the rare exotic wire forms.
//! > - **Write path:** the SDK encodes item write bodies through the native
//! >   serde serializer [`to_vec`] and advertises binary-response support via
//! >   the `x-ms-cosmos-supported-serialization-formats` request header.
//! >   [`encode`] (the minimal-but-valid `&Value` encoder) round-trips with
//! >   [`decode`] and backs the parity tests.
//! >
//! > All of this stays inert on the wire until binary encoding is enabled
//! > (`AZURE_COSMOS_BINARY_ENCODING_ENABLED`); with it off, requests and
//! > responses are byte-for-byte unchanged. Query/feed binary negotiation is
//! > still deferred to a later phase.

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
}

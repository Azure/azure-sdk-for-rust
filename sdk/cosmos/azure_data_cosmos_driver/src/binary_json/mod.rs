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
//! This module is **schema-agnostic**: it operates purely on bytes and
//! `serde_json::Value` (and, in a later phase, directly on `serde` types). It
//! does not know about Cosmos item schemas, matching the driver's
//! schema-agnostic data-plane principle.
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
//! > **Status:** the decoder is being built incrementally (phase P1). Phase
//! > P1a — the [`Reader`](reader)/[`decode`] scalar decode path — is in place;
//! > containers, user/reference strings, and the exotic forms follow in P1b–P1c,
//! > and wiring the decoder into the response path lands in P1e. Nothing is
//! > wired into the request/response path yet, so there is no behavior change.

pub mod error;
pub mod markers;
pub mod reader;
pub mod system_strings;

#[cfg(test)]
mod vectors;

pub use error::{BinaryError, Result};
pub use reader::decode;

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

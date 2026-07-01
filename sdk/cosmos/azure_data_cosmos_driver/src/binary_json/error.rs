// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Error types for the Cosmos binary JSON codec.
//!
//! [`BinaryError`] is the failure vocabulary the decoder (and, where relevant,
//! the encoder) produces. The decoder parses **untrusted** service bytes, so
//! every fallible step returns one of these variants rather than panicking —
//! malformed, truncated, or adversarial buffers must fail gracefully (see the
//! decoder-fuzzing requirement in the binary-encoding spec).

use std::fmt;

/// A specialized [`Result`](std::result::Result) for binary JSON codec
/// operations.
pub type Result<T> = std::result::Result<T, BinaryError>;

/// An error produced while decoding or encoding Cosmos binary JSON.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum BinaryError {
    /// The buffer ended before a value could be fully read.
    ///
    /// Carries the number of additional bytes the reader needed at the point it
    /// ran out of input.
    UnexpectedEof {
        /// How many more bytes were required to continue.
        needed: usize,
    },

    /// A type-marker byte was encountered that is not valid in the position it
    /// appeared (for example a reserved marker, or [`crate::binary_json::markers::INVALID`]).
    InvalidMarker {
        /// The offending marker byte.
        marker: u8,
        /// Byte offset of the marker within the buffer.
        offset: usize,
    },

    /// A length or count prefix was malformed or describes a region that does
    /// not fit within the remaining buffer.
    InvalidLength {
        /// Human-readable detail about which length was invalid.
        detail: &'static str,
    },

    /// A string's bytes are not valid UTF-8.
    InvalidUtf8 {
        /// Byte offset of the string payload within the buffer.
        offset: usize,
    },

    /// A decoded number cannot be represented as JSON (for example a non-finite
    /// `double` such as NaN or infinity, which JSON does not permit).
    InvalidNumber {
        /// Human-readable detail about why the number is not representable.
        detail: &'static str,
    },

    /// A reference string ([`StrR1`](crate::binary_json::markers::STR_R1)–[`StrR4`](crate::binary_json::markers::STR_R4))
    /// pointed at an offset that does not correspond to an earlier string.
    UnresolvedReference {
        /// The byte offset the reference attempted to resolve.
        target: usize,
    },

    /// A user string ([`UserString1ByteLengthMin`](crate::binary_json::markers::USER_STRING_1BYTE_MIN)–`0x67`)
    /// was encountered. User strings are encoded against an external string
    /// dictionary that the Cosmos data plane does not provide, so the string
    /// cannot be resolved.
    UnsupportedUserString {
        /// The decoded user-string dictionary id.
        id: usize,
    },

    /// The buffer nests containers more deeply than the decoder's configured
    /// limit. A depth bound prevents stack exhaustion from adversarial input.
    DepthLimitExceeded {
        /// The configured maximum nesting depth.
        limit: usize,
    },

    /// The buffer did not begin with the expected
    /// [`PREAMBLE`](crate::binary_json::PREAMBLE) byte.
    MissingPreamble {
        /// The first byte that was found instead.
        found: u8,
    },

    /// Extra bytes remained after a complete top-level value was decoded.
    TrailingBytes {
        /// Number of unconsumed bytes.
        remaining: usize,
    },
}

impl fmt::Display for BinaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryError::UnexpectedEof { needed } => {
                write!(
                    f,
                    "unexpected end of binary JSON buffer (needed {needed} more byte(s))"
                )
            }
            BinaryError::InvalidMarker { marker, offset } => {
                write!(
                    f,
                    "invalid binary JSON type marker {marker:#04x} at offset {offset}"
                )
            }
            BinaryError::InvalidLength { detail } => {
                write!(f, "invalid binary JSON length prefix: {detail}")
            }
            BinaryError::InvalidUtf8 { offset } => {
                write!(f, "invalid UTF-8 in binary JSON string at offset {offset}")
            }
            BinaryError::InvalidNumber { detail } => {
                write!(
                    f,
                    "binary JSON number is not representable as JSON: {detail}"
                )
            }
            BinaryError::UnresolvedReference { target } => {
                write!(
                    f,
                    "binary JSON reference string targets unresolved offset {target}"
                )
            }
            BinaryError::UnsupportedUserString { id } => {
                write!(
                    f,
                    "binary JSON user string (id {id}) requires a string dictionary that is not available"
                )
            }
            BinaryError::DepthLimitExceeded { limit } => {
                write!(
                    f,
                    "binary JSON nesting exceeds the maximum depth of {limit}"
                )
            }
            BinaryError::MissingPreamble { found } => {
                write!(
                    f,
                    "binary JSON buffer does not start with the 0x80 preamble (found {found:#04x})"
                )
            }
            BinaryError::TrailingBytes { remaining } => {
                write!(
                    f,
                    "binary JSON buffer has {remaining} trailing byte(s) after the top-level value"
                )
            }
        }
    }
}

impl std::error::Error for BinaryError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_messages_are_informative() {
        assert!(BinaryError::UnexpectedEof { needed: 4 }
            .to_string()
            .contains("needed 4"));
        assert!(BinaryError::InvalidMarker {
            marker: 0xFF,
            offset: 7
        }
        .to_string()
        .contains("0xff"));
        assert!(BinaryError::MissingPreamble { found: 0x7B }
            .to_string()
            .contains("0x7b"));
    }

    #[test]
    fn implements_std_error() {
        // Compile-time check that BinaryError satisfies the std error trait so
        // it can participate in `?`/`Box<dyn Error>` flows.
        fn assert_error<E: std::error::Error>(_: &E) {}
        assert_error(&BinaryError::TrailingBytes { remaining: 1 });
    }
}

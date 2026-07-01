// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Golden binary-JSON test vectors (test-support).
//!
//! Each [`BinaryVector`] pairs a complete Cosmos binary JSON buffer with the
//! text JSON it represents. The buffers here are the **minimal-valid** encoding
//! the spec's encoder section produces for trivial scalars (the `0x80` preamble
//! followed by the value's single-byte marker, or an encoded-length string),
//! so they are well-defined rather than guessed.
//!
//! At phase P0 these vectors anchor a structural sanity check that the
//! foundation primitives — the [`PREAMBLE`](super::PREAMBLE),
//! [`markers`](super::markers), and the
//! [`system_strings`](super::system_strings) table — are wired correctly and
//! are sufficient to dispatch scalar values. When the P1 decoder lands, the
//! same corpus becomes its decode-parity bar (decode `binary` and assert it
//! equals `json`), and the P2 encoder must reproduce `binary` from `json`.
//!
//! The module is compiled only under `cfg(test)`.

use super::markers;
use super::{system_strings, PREAMBLE};

/// A single golden vector: a complete binary JSON buffer and the canonical text
/// JSON it encodes.
pub(crate) struct BinaryVector {
    /// Human-readable case name (used in assertion messages).
    pub name: &'static str,
    /// The complete binary JSON buffer (including the `0x80` preamble).
    pub binary: &'static [u8],
    /// The text JSON the buffer decodes to.
    pub json: &'static str,
}

/// Scalar golden vectors, hand-encoded from the single-byte type markers plus a
/// short encoded-length string.
///
/// Restricted to forms that are unambiguous from the marker constants alone
/// (no length back-patching), so they can be asserted byte-for-byte today.
pub(crate) const SCALAR_VECTORS: &[BinaryVector] = &[
    BinaryVector {
        name: "null",
        binary: &[PREAMBLE, markers::NULL],
        json: "null",
    },
    BinaryVector {
        name: "false",
        binary: &[PREAMBLE, markers::FALSE],
        json: "false",
    },
    BinaryVector {
        name: "true",
        binary: &[PREAMBLE, markers::TRUE],
        json: "true",
    },
    BinaryVector {
        name: "literal_int_zero",
        // A literal integer whose value IS the marker (0x00 == 0).
        binary: &[PREAMBLE, markers::LITERAL_INT_MIN],
        json: "0",
    },
    BinaryVector {
        name: "literal_int_max",
        // The largest literal integer (0x1F == 31).
        binary: &[PREAMBLE, markers::LITERAL_INT_MAX - 1],
        json: "31",
    },
    BinaryVector {
        name: "system_string_id",
        // System string at index 12 ("id"): marker == SYSTEM_STRING_1BYTE_MIN + 12.
        binary: &[PREAMBLE, markers::SYSTEM_STRING_1BYTE_MIN + 12],
        json: "\"id\"",
    },
    BinaryVector {
        name: "encoded_length_string_id",
        // Encoded-length string "id": marker carries the length (0x80 | 2),
        // followed by the two UTF-8 bytes.
        binary: &[PREAMBLE, markers::ENCODED_STRING_LENGTH_MIN | 2, b'i', b'd'],
        json: "\"id\"",
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_json::is_binary;

    /// Every golden buffer is detected as binary and starts with the preamble.
    #[test]
    fn vectors_are_well_formed_binary() {
        for v in SCALAR_VECTORS {
            assert!(is_binary(v.binary), "{}: not detected as binary", v.name);
            assert_eq!(v.binary[0], PREAMBLE, "{}: missing preamble", v.name);
            assert!(!v.json.is_empty(), "{}: empty expected json", v.name);
        }
    }

    /// A trivial structural decoder built **only** from the P0 foundation
    /// (preamble + marker ranges + the system-string table). Proves the
    /// constants are wired correctly and that the foundation can already
    /// dispatch every scalar form in the corpus. P1's real decoder generalizes
    /// this to containers, all number widths, references, etc.
    fn decode_scalar(binary: &[u8]) -> String {
        assert_eq!(binary[0], PREAMBLE, "buffer must start with the preamble");
        let marker = binary[1];
        match marker {
            markers::NULL => "null".to_string(),
            markers::FALSE => "false".to_string(),
            markers::TRUE => "true".to_string(),
            m if (markers::LITERAL_INT_MIN..markers::LITERAL_INT_MAX).contains(&m) => {
                // The literal integer's value is the marker itself.
                m.to_string()
            }
            m if (markers::SYSTEM_STRING_1BYTE_MIN..markers::SYSTEM_STRING_1BYTE_MAX)
                .contains(&m) =>
            {
                let s = system_strings::system_string_for_marker(m)
                    .expect("system-string marker resolves to a table entry");
                // `{:?}` on a simple ASCII &str yields the JSON-quoted form.
                format!("{s:?}")
            }
            m if (markers::ENCODED_STRING_LENGTH_MIN..markers::ENCODED_STRING_LENGTH_MAX)
                .contains(&m) =>
            {
                let len = usize::from(m & markers::ENCODED_STRING_LENGTH_MASK);
                let s = std::str::from_utf8(&binary[2..2 + len])
                    .expect("encoded-length string is valid UTF-8");
                format!("{s:?}")
            }
            other => panic!("unhandled scalar marker {other:#04x}"),
        }
    }

    /// The foundation decodes every scalar vector to its expected JSON text.
    /// This also pins the byte-exactness of each hand-encoded buffer.
    #[test]
    fn foundation_decodes_scalar_corpus() {
        for v in SCALAR_VECTORS {
            assert_eq!(decode_scalar(v.binary), v.json, "case {}", v.name);
        }
    }

    /// The corpus covers each distinct scalar marker class exactly so a future
    /// edit can't silently drop coverage.
    #[test]
    fn corpus_covers_expected_cases() {
        let names: Vec<&str> = SCALAR_VECTORS.iter().map(|v| v.name).collect();
        assert_eq!(
            names,
            vec![
                "null",
                "false",
                "true",
                "literal_int_zero",
                "literal_int_max",
                "system_string_id",
                "encoded_length_string_id",
            ],
        );
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Golden binary-JSON decode vectors (test-support).
//!
//! Each [`BinaryVector`] pairs a complete Cosmos binary JSON buffer with the
//! text JSON it decodes to. The corpus lives in a shared, human-reviewable JSON
//! data file (`testdata/binary_json_vectors.json`) embedded via [`include_str!`]
//! so it can be reviewed and shared across language SDKs.
//!
//! The module is compiled only under `cfg(test)`.

use serde::Deserialize;

/// The embedded corpus data file (see the module docs for the format).
const CORPUS: &str = include_str!("../../testdata/binary_json_vectors.json");

/// A single golden vector: a complete binary JSON buffer and the canonical text
/// JSON it decodes to.
pub(crate) struct BinaryVector {
    /// Human-readable case name (used in assertion messages).
    pub name: String,
    /// The complete binary JSON buffer (including the `0x80` preamble).
    pub binary: Vec<u8>,
    /// The text JSON the buffer decodes to.
    pub json: String,
}

/// The on-disk shape of a corpus record: a case name, the binary buffer as a
/// space-separated hex string, and the expected JSON value written inline.
#[derive(Deserialize)]
struct RawVector {
    name: String,
    binary: String,
    json: serde_json::Value,
}

/// Parses the embedded JSON corpus into golden vectors.
///
/// The file is a JSON array of objects with `name`, `binary` (space-separated
/// hex bytes), and `json` (the expected value inline). Panics on a malformed
/// corpus, since it is a compile-time-embedded test fixture.
pub(crate) fn golden_vectors() -> Vec<BinaryVector> {
    let raw: Vec<RawVector> =
        serde_json::from_str(CORPUS).expect("corpus must be a valid JSON array of vectors");

    let vectors: Vec<BinaryVector> = raw
        .into_iter()
        .map(|r| BinaryVector {
            name: r.name,
            binary: parse_hex(&r.binary),
            json: r.json.to_string(),
        })
        .collect();

    assert!(!vectors.is_empty(), "corpus must not be empty");
    vectors
}

/// Parses space-separated 2-digit hex bytes (e.g. `"80 D0"`) into bytes.
fn parse_hex(text: &str) -> Vec<u8> {
    text.split_whitespace()
        .map(|byte| u8::from_str_radix(byte, 16).expect("corpus binary is valid hex"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_json::{is_binary, PREAMBLE};

    /// Every golden buffer is detected as binary, starts with the preamble, and
    /// carries non-empty name/json fields, with unique names.
    #[test]
    fn vectors_are_well_formed() {
        let vectors = golden_vectors();
        for v in &vectors {
            assert!(is_binary(&v.binary), "{}: not detected as binary", v.name);
            assert_eq!(v.binary[0], PREAMBLE, "{}: missing preamble", v.name);
            assert!(!v.json.is_empty(), "{}: empty expected json", v.name);
        }
        let mut names: Vec<&str> = vectors.iter().map(|v| v.name.as_str()).collect();
        names.sort_unstable();
        let count = names.len();
        names.dedup();
        assert_eq!(names.len(), count, "corpus contains duplicate vector names");
    }
}

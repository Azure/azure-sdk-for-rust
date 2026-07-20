// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Golden binary-JSON decode vectors (test-support).
//!
//! Each [`BinaryVector`] pairs a complete Cosmos binary JSON buffer with the
//! text JSON it decodes to. The corpus lives in a shared, human-reviewable data
//! file (`testdata/binary_json_vectors.txt`) embedded via [`include_str!`] so it
//! can be reviewed as spaced-hex bytes and shared across language SDKs.
//!
//! The module is compiled only under `cfg(test)`.

/// The embedded corpus data file (see the module docs for the format).
const CORPUS: &str = include_str!("../../testdata/binary_json_vectors.txt");

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

/// Parses the embedded corpus into golden vectors.
///
/// The format is a sequence of blank-line-separated records, each with `name:`,
/// `binary:` (space-separated hex bytes), and `json:` fields. Lines starting
/// with `#` are comments. Panics on a malformed corpus, since it is a
/// compile-time-embedded test fixture.
pub(crate) fn golden_vectors() -> Vec<BinaryVector> {
    let mut vectors = Vec::new();
    let mut name: Option<String> = None;
    let mut binary: Option<Vec<u8>> = None;
    let mut json: Option<String> = None;

    let flush = |name: &mut Option<String>,
                 binary: &mut Option<Vec<u8>>,
                 json: &mut Option<String>,
                 vectors: &mut Vec<BinaryVector>| {
        let (name, binary, json) = (name.take(), binary.take(), json.take());
        match (name, binary, json) {
            (Some(name), Some(binary), Some(json)) => {
                vectors.push(BinaryVector { name, binary, json });
            }
            // A completely empty record (e.g. consecutive blank lines) is fine;
            // a partial record means the corpus is malformed — fail fast so the
            // corruption is surfaced deterministically rather than silently
            // dropped.
            (None, None, None) => {}
            (name, binary, json) => panic!(
                "incomplete corpus record: name={name:?} binary_present={} json={json:?}",
                binary.is_some()
            ),
        }
    };

    for line in CORPUS.lines() {
        let line = line.trim();
        if line.is_empty() {
            flush(&mut name, &mut binary, &mut json, &mut vectors);
            continue;
        }
        if line.starts_with('#') {
            continue;
        }
        if let Some(value) = line.strip_prefix("name:") {
            name = Some(value.trim().to_owned());
        } else if let Some(value) = line.strip_prefix("binary:") {
            binary = Some(parse_hex(value.trim()));
        } else if let Some(value) = line.strip_prefix("json:") {
            json = Some(value.trim().to_owned());
        } else {
            panic!("unrecognized corpus line: {line:?}");
        }
    }
    // Flush a trailing record without a terminating blank line.
    flush(&mut name, &mut binary, &mut json, &mut vectors);

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

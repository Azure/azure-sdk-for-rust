// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Decoder robustness ("fuzz") tests for the Cosmos binary JSON codec.
//!
//! The decoder parses **untrusted** service bytes, so its security contract is:
//! for *any* input buffer it must terminate and either succeed or return a
//! [`BinaryError`](super::BinaryError) — never panic, never hang, and never
//! allocate based on an attacker-controlled length prefix beyond what the
//! buffer can back. These tests assert that contract by throwing large numbers
//! of random, truncated, corrupted, and adversarial buffers at
//! [`decode`](super::decode).
//!
//! Randomness is deterministic (a seeded SplitMix64, matching the crate's
//! dependency-free PRNG convention) so failures reproduce exactly.

use super::vectors::SCALAR_VECTORS;
use super::{decode, encode, markers, PREAMBLE};
use serde_json::json;

/// A tiny deterministic SplitMix64 PRNG.
///
/// Dependency-free (the crate avoids `rand`); the finalizer matches the
/// SplitMix64 mixing used elsewhere in the driver. Deterministic so a failing
/// case always reproduces from the same seed.
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Returns a `u64` in `[0, bound)` (`bound` must be non-zero).
    fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }

    fn byte(&mut self) -> u8 {
        self.next_u64() as u8
    }
}

/// A set of value-producing markers worth biasing random buffers toward, so the
/// generator spends time on the structurally-interesting forms (containers,
/// length-prefixed strings, numbers) rather than mostly-invalid markers.
const INTERESTING_MARKERS: &[u8] = &[
    markers::NULL,
    markers::FALSE,
    markers::TRUE,
    markers::NUMBER_INT64,
    markers::NUMBER_DOUBLE,
    markers::STR_L1,
    markers::STR_L2,
    markers::STR_L4,
    markers::STR_R1,
    markers::ARR_L1,
    markers::ARR_LC1,
    markers::OBJ_L1,
    markers::OBJ_LC1,
    markers::ARR_NUM_C1,
    markers::BINARY_1BYTE_LENGTH,
    markers::BINARY_4BYTE_LENGTH,
    markers::LOWERCASE_GUID_STRING,
    markers::BASE64_STRING_LENGTH1,
    markers::PACKED_7BIT_STRING_LENGTH1,
    markers::INVALID,
];

/// A representative spread of JSON values used to produce *valid* binary buffers
/// (via the encoder) that the corruption/truncation sweeps then mutate.
fn sample_values() -> Vec<serde_json::Value> {
    vec![
        json!(null),
        json!(true),
        json!(0),
        json!(31),
        json!(-5_000_000_000i64),
        json!(u64::MAX),
        json!(1.5),
        json!(""),
        json!("id"),
        json!("x".repeat(300)),
        json!([]),
        json!([1, 2, 3]),
        json!({}),
        json!({ "id": "1", "n": 7, "nested": { "a": [true, null] } }),
        json!([[1, 2], [3, [4, 5]]]),
    ]
}

#[test]
fn decode_never_panics_on_random_bytes() {
    let mut rng = SplitMix64::new(0x5eed_1234_5678_9abc);
    for _ in 0..20_000 {
        // Lengths up to 64 bytes; sometimes force the preamble so the decoder
        // proceeds past the first-byte check into the value parser.
        let len = rng.below(65) as usize;
        let mut buf = Vec::with_capacity(len);
        let force_preamble = rng.below(2) == 0;
        for i in 0..len {
            if i == 0 && force_preamble {
                buf.push(PREAMBLE);
            } else if rng.below(3) == 0 {
                // Bias toward interesting markers to reach deeper code paths.
                let idx = rng.below(INTERESTING_MARKERS.len() as u64) as usize;
                buf.push(INTERESTING_MARKERS[idx]);
            } else {
                buf.push(rng.byte());
            }
        }
        // The contract: terminate with Ok or Err, never panic.
        let _ = decode(&buf);
    }
}

#[test]
fn decode_never_panics_on_truncated_valid_buffers() {
    // Every prefix of a valid buffer (golden corpus + encoder output for the
    // sample values) must decode or error without panicking.
    let mut buffers: Vec<Vec<u8>> = SCALAR_VECTORS.iter().map(|v| v.binary.to_vec()).collect();
    buffers.extend(sample_values().iter().map(encode));

    for buf in &buffers {
        for cut in 0..=buf.len() {
            let _ = decode(&buf[..cut]);
        }
    }
}

#[test]
fn decode_never_panics_on_single_byte_corruption() {
    let mut rng = SplitMix64::new(0xc0ff_ee00_d00d_1010);
    for value in sample_values() {
        let valid = encode(&value);
        for index in 0..valid.len() {
            // Try a handful of replacement bytes at each position, including
            // boundary marker values that flip the parse down a different arm.
            for replacement in [0x00, 0x80, 0xC0, 0xE0, 0xFF, rng.byte()] {
                let mut corrupted = valid.clone();
                corrupted[index] = replacement;
                let _ = decode(&corrupted);
            }
        }
    }
}

#[test]
fn adversarial_length_prefixes_do_not_over_allocate() {
    // Buffers that declare an enormous payload but carry almost none must fail
    // with a bounds error rather than panicking, hanging, or attempting a
    // multi-gigabyte allocation. `read_bytes` only ever slices the existing
    // buffer, so these resolve in O(1) without allocating the declared size.
    let huge = u32::MAX; // ~4 GiB declared

    // StrL4 with a 4-byte length of u32::MAX but no payload.
    let mut str_l4 = vec![PREAMBLE, markers::STR_L4];
    str_l4.extend_from_slice(&huge.to_le_bytes());
    assert!(decode(&str_l4).is_err());

    // ArrL4 / ObjL4 with a giant declared body length.
    for marker in [markers::ARR_L4, markers::OBJ_L4] {
        let mut buf = vec![PREAMBLE, marker];
        buf.extend_from_slice(&huge.to_le_bytes());
        assert!(decode(&buf).is_err());
    }

    // Binary4ByteLength with a giant declared blob length.
    let mut bin = vec![PREAMBLE, markers::BINARY_4BYTE_LENGTH];
    bin.extend_from_slice(&huge.to_le_bytes());
    assert!(decode(&bin).is_err());

    // A uniform Int64 array claiming u32::MAX items: must error, not try to
    // build a 4-billion-element vector.
    let mut uniform = vec![PREAMBLE, markers::ARR_NUM_C2, markers::INT64];
    uniform.extend_from_slice(&(u16::MAX).to_le_bytes());
    assert!(decode(&uniform).is_err());
}

#[test]
fn deeply_nested_input_errors_without_stack_overflow() {
    // A pathologically deep nesting of single-item arrays must hit the depth
    // guard (DepthLimitExceeded) rather than overflowing the stack. 10_000 is
    // far beyond MAX_DEPTH, so this exercises the guard, not a valid document.
    let mut buf = vec![PREAMBLE];
    buf.extend(std::iter::repeat_n(markers::ARR1, 10_000));
    buf.push(0x00); // a literal-int leaf (never reached past the guard)
    assert!(decode(&buf).is_err());
}

#[test]
fn all_two_byte_inputs_terminate() {
    // Exhaustively decode every `[0x80, b]` two-byte buffer: every single-byte
    // value form (and every invalid marker) must resolve without panicking.
    for b in 0u16..=255 {
        let _ = decode(&[PREAMBLE, b as u8]);
    }
}

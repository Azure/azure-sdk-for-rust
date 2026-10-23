// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Decoder robustness ("fuzz") tests for the Cosmos binary JSON codec.
//!
//! For any input buffer the decoder must terminate and either succeed or return
//! a [`BinaryError`](super::BinaryError) — never panic, never hang, and never
//! allocate based on a length prefix beyond what the buffer can back. These
//! tests assert that by throwing random, truncated, corrupted, and adversarial
//! buffers at [`decode`](super::decode). Randomness is deterministic so failures
//! reproduce exactly.

use super::vectors::golden_vectors;
use super::{decode, encode, from_slice, markers, PREAMBLE};
use super::{reader::BinaryCursor, test_samples::SplitMix64};
use serde_json::json;

fn scan(buffer: &[u8]) -> super::Result<()> {
    let mut cursor = BinaryCursor::new(buffer)?;
    let range = cursor.scan_value(0)?;
    cursor.finish()?;
    assert_eq!(range, 1..buffer.len());
    Ok(())
}

fn assert_scanner_agrees(buffer: &[u8]) {
    let decoded = decode(buffer);
    let scanned = scan(buffer);
    assert_eq!(
        scanned.is_ok(),
        decoded.is_ok(),
        "scanner/decoder acceptance differs: {buffer:02x?}"
    );
}

#[test]
fn scanner_matches_decoder_on_generated_values() {
    for seed in [0, 1, 0x5eed, u64::MAX] {
        let mut rng = SplitMix64::new(seed);
        for _ in 0..500 {
            assert_scanner_agrees(&encode(&rng.document()));
        }
    }
}

#[test]
fn scanner_matches_decoder_on_golden_truncations_and_corruptions() {
    for vector in golden_vectors() {
        for cut in 0..=vector.binary.len() {
            assert_scanner_agrees(&vector.binary[..cut]);
        }
        for index in 0..vector.binary.len() {
            for replacement in [0, 0x80, 0xc0, 0xe0, 0xff] {
                let mut mutated = vector.binary.clone();
                mutated[index] = replacement;
                assert_scanner_agrees(&mutated);
            }
        }
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
        assert_scanner_agrees(&buf);
    }
}

#[test]
fn decode_never_panics_on_truncated_valid_buffers() {
    // Every prefix of a valid buffer (golden corpus + encoder output for the
    // sample values) must decode or error without panicking.
    let mut buffers: Vec<Vec<u8>> = golden_vectors().into_iter().map(|v| v.binary).collect();
    buffers.extend(sample_values().iter().map(encode));

    for buf in &buffers {
        for cut in 0..=buf.len() {
            assert_scanner_agrees(&buf[..cut]);
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
                assert_scanner_agrees(&corrupted);
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
    assert_scanner_agrees(&str_l4);

    // ArrL4 / ObjL4 with a giant declared body length.
    for marker in [markers::ARR_L4, markers::OBJ_L4] {
        let mut buf = vec![PREAMBLE, marker];
        buf.extend_from_slice(&huge.to_le_bytes());
        assert!(decode(&buf).is_err());
        assert_scanner_agrees(&buf);
    }

    // Binary4ByteLength with a giant declared blob length.
    let mut bin = vec![PREAMBLE, markers::BINARY_4BYTE_LENGTH];
    bin.extend_from_slice(&huge.to_le_bytes());
    assert!(decode(&bin).is_err());
    assert_scanner_agrees(&bin);

    // A uniform Int64 array claiming u16::MAX items (the max an ArrNumC2
    // count field can express): must error, not try to build a 65,535-element
    // vector from a buffer that carries almost no payload.
    let mut uniform = vec![PREAMBLE, markers::ARR_NUM_C2, markers::INT64];
    uniform.extend_from_slice(&(u16::MAX).to_le_bytes());
    assert!(decode(&uniform).is_err());
    assert_scanner_agrees(&uniform);
}

#[test]
fn deeply_nested_input_errors_without_stack_overflow() {
    // A pathologically deep nesting of single-item arrays must hit the depth
    // guard (DepthLimitExceeded) rather than overflowing the stack. 10_000 is
    // far beyond MAX_DEPTH, so this exercises the guard, not a valid document.
    // The recursive descent keeps a small per-level frame (leaf decoding lives
    // in a separate non-inlined frame), so reaching the guard stays within an
    // ordinary thread stack.
    let mut buf = vec![PREAMBLE];
    buf.extend(std::iter::repeat_n(markers::ARR1, 10_000));
    buf.push(0x00); // a literal-int leaf (never reached past the guard)
    assert!(decode(&buf).is_err());
    assert_scanner_agrees(&buf);
}

#[test]
fn all_two_byte_inputs_terminate() {
    // Exhaustively decode every `[0x80, b]` two-byte buffer: every single-byte
    // value form (and every invalid marker) must resolve without panicking.
    for b in 0u16..=255 {
        assert_scanner_agrees(&[PREAMBLE, b as u8]);
    }
}

/// Runs the same buffer through both parsers of untrusted bytes and asserts
/// their agreement contract, returning the pair of results for the caller to
/// spot-check counts.
///
/// The reference [`decode`] and the native streaming [`from_slice`] are two
/// independent parsers, and `from_slice` is the one actually wired into item
/// reads. The contract: neither may panic on any input, and whenever **both**
/// succeed they must produce the identical [`serde_json::Value`]. (One may
/// legitimately reject a buffer the other accepts only in the direction where
/// `from_slice` is stricter — e.g. exotic forms it defers to `decode` for — so
/// disagreement is asserted only when both return `Ok`.)
///
/// Compared **modulo the integral-`Double`→integer rule**, because the two sit
/// at different layers: `decode` stays faithful to the wire, while `from_slice`
/// is the caller boundary and renders an integral `Double` as an integer. A
/// structural or value disagreement still fails.
///
/// The contract is at the **`Value`** (untyped) level. It does not cover typed
/// integer targets, where `from_slice` additionally coerces out-of-`Value`-range
/// forms (see [`BinaryDeserializer::deserialize_integer`]).
fn assert_decoders_agree(buf: &[u8]) {
    let decoded = decode(buf);
    let streamed = from_slice::<serde_json::Value>(buf);
    if let (Ok(a), Ok(b)) = (&decoded, &streamed) {
        let mut a = a.clone();
        super::normalize_integral_floats(&mut a);
        assert_eq!(
            &a, b,
            "decode and from_slice disagreed on a buffer both accepted: {buf:02x?}"
        );
    }
}

#[test]
fn decode_and_from_slice_agree_on_random_bytes() {
    // Mirror `decode_never_panics_on_random_bytes`, but drive both parsers so
    // the native streaming path (SeqStream/MapStream termination) receives the
    // same adversarial coverage as the reference decoder.
    let mut rng = SplitMix64::new(0x0d1f_f00d_face_b00c);
    for _ in 0..20_000 {
        let len = rng.below(65) as usize;
        let mut buf = Vec::with_capacity(len);
        let force_preamble = rng.below(2) == 0;
        for i in 0..len {
            if i == 0 && force_preamble {
                buf.push(PREAMBLE);
            } else if rng.below(3) == 0 {
                let idx = rng.below(INTERESTING_MARKERS.len() as u64) as usize;
                buf.push(INTERESTING_MARKERS[idx]);
            } else {
                buf.push(rng.byte());
            }
        }
        assert_decoders_agree(&buf);
    }
}

#[test]
fn decode_and_from_slice_agree_on_truncated_and_corrupted_buffers() {
    // Both parsers over every prefix and every single-byte corruption of the
    // encoder's output for the sample values.
    let mut rng = SplitMix64::new(0xabad_1dea_1234_5678);
    for value in sample_values() {
        let valid = encode(&value);

        for cut in 0..=valid.len() {
            assert_decoders_agree(&valid[..cut]);
        }

        for index in 0..valid.len() {
            for replacement in [0x00, 0x80, 0xC0, 0xE0, 0xFF, rng.byte()] {
                let mut corrupted = valid.clone();
                corrupted[index] = replacement;
                assert_decoders_agree(&corrupted);
            }
        }
    }
}

#[test]
fn many_references_to_one_large_string_stay_bounded() {
    // One large `StrL2` string followed by many `StrR2` refs must be rejected
    // by the reference-expansion budget, not amplified into O(S²) `String`s.
    let payload_len = 4096usize;
    let reference_count = 20_000usize;
    let element_count = reference_count + 1;

    // Container prefix is ArrLC4: PREAMBLE + marker + 4-byte length + 4-byte
    // count, so the first element (the big string) begins at offset 10.
    let string_offset: u16 = 1 + 1 + 4 + 4;

    // Body: the big StrL2 string, then `reference_count` StrR2 refs to it.
    let mut body = Vec::new();
    body.push(markers::STR_L2);
    body.extend_from_slice(&(payload_len as u16).to_le_bytes());
    body.extend(std::iter::repeat_n(b'a', payload_len));
    for _ in 0..reference_count {
        body.push(markers::STR_R2);
        body.extend_from_slice(&string_offset.to_le_bytes());
    }

    let mut buf = vec![PREAMBLE, markers::ARR_LC4];
    buf.extend_from_slice(&(body.len() as u32).to_le_bytes());
    buf.extend_from_slice(&(element_count as u32).to_le_bytes());
    buf.extend_from_slice(&body);

    // ~80 MB of expansion exceeds the budget floor, so decode must error.
    assert!(
        decode(&buf).is_err(),
        "expected the reference-expansion budget to reject the amplified buffer"
    );
    // The native path must also terminate without panicking.
    let _ = from_slice::<serde_json::Value>(&buf);
    assert_scanner_agrees(&buf);
}

#[test]
fn from_slice_rejects_undersized_container_count() {
    // A count-framed object whose declared count (1) is smaller than the number
    // of members its byte length spans (2). The reference decoder rejects this
    // via its member-count validation; the native streaming path must agree
    // rather than silently under-reading and reinterpreting the leftover bytes.
    //
    // Build the well-formed 2-member object first, then rewrite the count field
    // to 1, leaving the byte length untouched.
    let valid = encode(&json!({ "a": 1, "b": 2 }));

    // Locate the object marker (first byte after the preamble). It must be one
    // of the count-framed `ObjLC*` forms for this rewrite to apply.
    let obj_marker = valid[1];
    assert_eq!(
        obj_marker,
        markers::OBJ_LC1,
        "encoder is expected to emit a 1-byte length+count object here"
    );

    // Layout for ObjLC1: [PREAMBLE, OBJ_LC1, length(1), count(1), ...members].
    // The count byte sits at index 3; force it to 1.
    let mut undersized = valid.clone();
    undersized[3] = 1;

    // The reference decoder rejects the mismatched count...
    assert!(
        decode(&undersized).is_err(),
        "decode should reject an object whose declared count under-counts its members"
    );
    // ...and so must the native streaming deserializer.
    assert!(
        from_slice::<serde_json::Value>(&undersized).is_err(),
        "from_slice should reject an object whose declared count under-counts its members"
    );
}

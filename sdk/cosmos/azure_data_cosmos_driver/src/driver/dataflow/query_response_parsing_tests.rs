// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{parse_envelope_page, EnvelopePayload, PageAggregator};
use crate::{
    binary_json::{
        self, markers,
        reader::MAX_DEPTH,
        test_samples::SplitMix64,
        test_support::{array, envelope, object, reference_rows, string},
        vectors::golden_vectors,
        PREAMBLE,
    },
    error::status_codes,
    models::ResponseBody,
};
use bytes::Bytes;
use serde_json::{json, Value};

fn page(root: Vec<u8>) -> Bytes {
    let mut bytes = vec![PREAMBLE];
    bytes.extend(root);
    Bytes::from(bytes)
}

fn check_against_full_decode(bytes: Bytes) {
    let Ok(rows) = parse_envelope_page(&ResponseBody::from_bytes(bytes.clone()), 1) else {
        return;
    };
    let mut decoded = binary_json::decode(&bytes).expect("scanner accepted invalid binary");
    binary_json::normalize_integral_floats(&mut decoded);
    let text = serde_json::to_vec(&decoded).unwrap();
    let reference = parse_envelope_page(&ResponseBody::from_bytes(text), 1).unwrap();
    assert_eq!(rows.len(), reference.len());
    let aggregator = PageAggregator::new(true);
    for (row, expected) in rows.iter().zip(reference) {
        assert_eq!(row.keys, expected.keys);
        assert_eq!(row.rid, expected.rid);
        let EnvelopePayload::Binary(item) = &row.payload else {
            panic!("binary payload must retain page context");
        };
        let EnvelopePayload::Text(raw) = expected.payload else {
            panic!("expected text oracle");
        };
        let value: Value = serde_json::from_str(raw.get()).unwrap();
        assert_eq!(item.source_page().as_ptr(), bytes.as_ptr());
        assert_eq!(item.raw_value(), &bytes[item.value_range()]);
        assert_eq!(item.deserialize::<Value>().unwrap(), value);
        let mut standalone = binary_json::decode(&item.to_standalone().unwrap()).unwrap();
        binary_json::normalize_integral_floats(&mut standalone);
        assert_eq!(standalone, value);
        let emitted = aggregator.encode_item(0, &row.payload).unwrap();
        assert_eq!(emitted.source_page().as_ptr(), bytes.as_ptr());
    }
}

#[test]
fn generated_envelopes_match_full_decode_and_text_parser() {
    for seed in [0, 1, 0x5eed, u64::MAX] {
        let mut rng = SplitMix64::new(seed);
        for _ in 0..250 {
            let payloads: Vec<_> = (0..1 + rng.below(5))
                .map(|_| binary_json::encode(&rng.document())[1..].to_vec())
                .collect();
            let bytes = envelope(&payloads);
            assert_eq!(
                parse_envelope_page(&ResponseBody::from_bytes(bytes.clone()), 1)
                    .unwrap()
                    .len(),
                payloads.len()
            );
            check_against_full_decode(bytes);
        }
    }
}

#[test]
fn generated_sort_metadata_matches_full_decode() {
    let mut rng = SplitMix64::new(0x5283);
    for _ in 0..500 {
        let key = rng.value(2);
        let envelope = json!({
            "Documents": [{
                "unknown": rng.value(2),
                "orderByItems": [{"item": key}],
                "payload": rng.document(),
                "_rid": "metadata",
            }],
            "unknown": rng.value(2),
        });
        let bytes = Bytes::from(binary_json::encode(&envelope));
        let parsed = parse_envelope_page(&ResponseBody::from_bytes(bytes.clone()), 1);
        if key.is_array() || key.is_object() {
            let text = parse_envelope_page(
                &ResponseBody::from_bytes(serde_json::to_vec(&envelope).unwrap()),
                1,
            );
            assert_eq!(parsed.unwrap_err().status(), text.unwrap_err().status());
        } else {
            assert_eq!(parsed.unwrap().len(), 1);
        }
        check_against_full_decode(bytes);
    }
}

#[test]
fn envelope_truncations_corruptions_and_random_bytes_never_panic() {
    let bytes = reference_rows(&[("a", 1), ("b", 2)], "referenced string");
    for cut in 1..bytes.len() {
        assert!(
            parse_envelope_page(&ResponseBody::from_bytes(bytes.slice(..cut)), 1).is_err(),
            "cut {cut}"
        );
    }
    let mut rng = SplitMix64::new(0xc0ff_ee);
    for index in 0..bytes.len() {
        for replacement in [0, 0x80, 0xc0, 0xe0, 0xff, rng.byte()] {
            let mut mutated = bytes.to_vec();
            mutated[index] = replacement;
            check_against_full_decode(Bytes::from(mutated));
        }
    }
    for _ in 0..10_000 {
        let length = rng.below(128);
        let mut mutated = vec![PREAMBLE];
        mutated.extend((0..length).map(|_| rng.byte()));
        check_against_full_decode(Bytes::from(mutated));
    }
}

#[test]
fn golden_wire_forms_decode_in_payloads_and_unknown_fields() {
    for vector in golden_vectors() {
        // Reference targets belong to their original page; relocation is covered separately.
        if vector.name.starts_with("reference_string_") {
            continue;
        }
        let payload = vector.binary[1..].to_vec();
        let bytes = envelope(std::slice::from_ref(&payload));
        parse_envelope_page(&ResponseBody::from_bytes(bytes.clone()), 1)
            .unwrap_or_else(|e| panic!("{}: {e}", vector.name));
        check_against_full_decode(bytes);
        let bytes = page(object(&[
            ("unknown", payload.clone()),
            (
                "Documents",
                array(&[object(&[
                    ("_rid", string("row")),
                    ("unknown", payload),
                    ("payload", vec![markers::NULL]),
                    ("orderByItems", array(&[object(&[("item", vec![0])])])),
                ])]),
            ),
        ]));
        parse_envelope_page(&ResponseBody::from_bytes(bytes.clone()), 1)
            .unwrap_or_else(|e| panic!("unknown {}: {e}", vector.name));
        check_against_full_decode(bytes);
    }
}

#[test]
fn reference_offsets_cross_width_boundaries_and_can_point_forward() {
    for target in [255_u32, 256, 65_535, 65_536] {
        for (marker, width) in [
            (markers::STR_R1, 1),
            (markers::STR_R2, 2),
            (markers::STR_R3, 3),
            (markers::STR_R4, 4),
        ] {
            if u64::from(target) >= 1_u64 << (width * 8) {
                continue;
            }
            let mut reference = vec![marker];
            reference.extend_from_slice(&target.to_le_bytes()[..width]);
            let padding_length =
                target as usize - (10 + string("pad").len() + 5 + string("target").len());
            let mut padding = vec![markers::BINARY_4BYTE_LENGTH];
            padding.extend_from_slice(&(padding_length as u32).to_le_bytes());
            padding.extend(vec![0; padding_length]);
            let rows = array(&[object(&[
                ("_rid", string("row")),
                ("orderByItems", array(&[object(&[("item", vec![1])])])),
                ("payload", object(&[("value", reference)])),
            ])]);
            let bytes = page(object(&[
                ("pad", padding),
                ("target", string("needle")),
                ("Documents", rows),
            ]));
            assert_eq!(
                &bytes[target as usize..target as usize + string("needle").len()],
                string("needle")
            );
            check_against_full_decode(bytes.clone());
            let rows = parse_envelope_page(&ResponseBody::from_bytes(bytes), 1).unwrap();
            let EnvelopePayload::Binary(item) = &rows[0].payload else {
                panic!("binary")
            };
            assert_eq!(
                item.deserialize::<Value>().unwrap(),
                json!({"value": "needle"})
            );
        }
    }
    for forward in [false, true] {
        let mut reference = vec![markers::STR_R4, 0, 0, 0, 0];
        let make = |reference: Vec<u8>| {
            let documents = array(&[object(&[
                (
                    "payload",
                    vec![markers::OBJ1]
                        .into_iter()
                        .chain(reference)
                        .chain([markers::NULL])
                        .collect(),
                ),
                ("_rid", string("row")),
                ("orderByItems", array(&[object(&[("item", vec![1])])])),
            ])]);
            page(if forward {
                object(&[
                    ("Documents", documents),
                    ("target", string("unique-field-name")),
                ])
            } else {
                object(&[
                    ("target", string("unique-field-name")),
                    ("Documents", documents),
                ])
            })
        };
        let bytes = make(reference.clone());
        let needle = string("unique-field-name");
        let target = bytes
            .windows(needle.len())
            .position(|window| window == needle)
            .unwrap();
        reference[1..].copy_from_slice(&(target as u32).to_le_bytes());
        let bytes = make(reference);
        check_against_full_decode(bytes.clone());
        let rows = parse_envelope_page(&ResponseBody::from_bytes(bytes), 1).unwrap();
        let EnvelopePayload::Binary(item) = &rows[0].payload else {
            panic!("binary")
        };
        assert_eq!(
            item.deserialize::<Value>().unwrap(),
            json!({"unique-field-name": null})
        );
    }
}

#[test]
fn payload_references_can_target_another_document() {
    let needle = string("cross-document-target");
    let first = object(&[("target", needle.clone())]);
    let second = vec![markers::STR_R4, 0, 0, 0, 0];
    let initial = envelope(&[first.clone(), second.clone()]);
    let target = initial
        .windows(needle.len())
        .position(|window| window == needle)
        .unwrap();
    let mut reference = second;
    reference[1..].copy_from_slice(&(target as u32).to_le_bytes());
    let bytes = envelope(&[first, reference]);
    check_against_full_decode(bytes.clone());
    let rows = parse_envelope_page(&ResponseBody::from_bytes(bytes), 1).unwrap();
    let EnvelopePayload::Binary(item) = &rows[1].payload else {
        panic!("binary")
    };
    assert_eq!(
        item.deserialize::<String>().unwrap(),
        "cross-document-target"
    );
    drop(rows);
}

#[test]
fn structural_field_permutations_and_null_payloads_are_valid() {
    let fields = [
        ("_rid", string("row")),
        ("orderByItems", array(&[object(&[("item", vec![1])])])),
        ("payload", vec![markers::NULL]),
    ];
    for order in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        for name in ["Documents", "documents"] {
            let item = object(&order.map(|i| fields[i].clone()));
            let bytes = page(object(&[(name, array(&[item]))]));
            assert_eq!(
                parse_envelope_page(&ResponseBody::from_bytes(bytes.clone()), 1)
                    .unwrap()
                    .len(),
                1
            );
            check_against_full_decode(bytes);
        }
    }
}

#[test]
fn malformed_structural_fields_are_envelope_errors() {
    let valid = [
        ("_rid", string("row")),
        ("orderByItems", array(&[object(&[("item", vec![1])])])),
        ("payload", vec![markers::NULL]),
    ];
    for index in 0..valid.len() {
        let mut missing = valid.to_vec();
        missing.remove(index);
        let mut duplicate = valid.to_vec();
        duplicate.push(valid[index].clone());
        for fields in [missing, duplicate] {
            let bytes = page(object(&[("Documents", array(&[object(&fields)]))]));
            assert_eq!(
                parse_envelope_page(&ResponseBody::from_bytes(bytes), 1)
                    .unwrap_err()
                    .status(),
                status_codes::SERVICE_ORDER_BY_ENVELOPE_INVALID
            );
        }
    }
    for (field, invalid) in [
        ("_rid", vec![markers::NULL]),
        ("_rid", string("")),
        ("_rid", vec![1]),
        ("orderByItems", vec![markers::NULL]),
        ("orderByItems", array(&[])),
        (
            "orderByItems",
            array(&[object(&[("item", vec![1])]), object(&[("item", vec![2])])]),
        ),
    ] {
        let mut fields = valid.to_vec();
        fields
            .iter_mut()
            .find(|(name, _)| *name == field)
            .unwrap()
            .1 = invalid;
        let bytes = page(object(&[("Documents", array(&[object(&fields)]))]));
        assert_eq!(
            parse_envelope_page(&ResponseBody::from_bytes(bytes), 1)
                .unwrap_err()
                .status(),
            status_codes::SERVICE_ORDER_BY_ENVELOPE_INVALID
        );
    }
    for root in [
        object(&[]),
        object(&[("Documents", vec![markers::NULL])]),
        object(&[("Documents", array(&[vec![markers::NULL]]))]),
        object(&[("Documents", array(&[])), ("documents", array(&[]))]),
    ] {
        assert_eq!(
            parse_envelope_page(&ResponseBody::from_bytes(page(root)), 1)
                .unwrap_err()
                .status(),
            status_codes::SERVICE_ORDER_BY_ENVELOPE_INVALID
        );
    }
}

#[test]
fn invalid_references_and_unknown_values_are_serialization_errors() {
    for invalid in [
        vec![markers::STR_R4, 255, 255, 255, 255],
        vec![markers::STR_R1, 1],
        vec![markers::STR_L1, 1, 255],
        vec![markers::ARR_LC1, 1, 0, markers::NULL],
        vec![markers::OBJ_LC1, 1, 1, markers::NULL],
        vec![markers::NUMBER_DOUBLE]
            .into_iter()
            .chain(f64::NAN.to_le_bytes())
            .collect(),
        vec![markers::ARR_NUM_C1, markers::FLOAT64, 1]
            .into_iter()
            .chain(f64::INFINITY.to_le_bytes())
            .collect(),
    ] {
        for bytes in [
            envelope(std::slice::from_ref(&invalid)),
            page(object(&[
                ("unknown", invalid.clone()),
                ("Documents", array(&[])),
            ])),
            page(object(&[
                ("Documents", array(&[])),
                ("unknown", invalid.clone()),
            ])),
            page(object(&[(
                "Documents",
                array(&[object(&[
                    ("_rid", string("row")),
                    ("orderByItems", array(&[object(&[("item", vec![1])])])),
                    ("payload", vec![markers::NULL]),
                    ("unknown", invalid.clone()),
                ])]),
            )])),
        ] {
            let error = parse_envelope_page(&ResponseBody::from_bytes(bytes), 1)
                .expect_err(&format!("malformed value must be rejected: {invalid:02x?}"));
            assert_eq!(
                error.status(),
                status_codes::SERIALIZATION_RESPONSE_BODY_INVALID
            );
        }
    }
}

#[test]
fn envelope_depth_and_trailing_bytes_are_checked() {
    for (depth, valid) in [(MAX_DEPTH - 3, true), (MAX_DEPTH - 2, false)] {
        let mut payload = vec![markers::ARR1; depth];
        payload.push(markers::NULL);
        let result = parse_envelope_page(&ResponseBody::from_bytes(envelope(&[payload])), 1);
        assert_eq!(result.is_ok(), valid, "depth {depth}");
    }
    let mut bytes = envelope(&[vec![markers::NULL]]).to_vec();
    bytes.push(markers::NULL);
    assert_eq!(
        parse_envelope_page(&ResponseBody::from_bytes(bytes), 1)
            .unwrap_err()
            .status(),
        status_codes::SERIALIZATION_RESPONSE_BODY_INVALID
    );
}

#[test]
fn string_and_container_length_forms_keep_exact_item_spans() {
    for length in [0, 1, 63, 64, 255, 256, 65_535, 65_536] {
        let text = "x".repeat(length);
        for (marker, width) in [
            (markers::STR_L1, 1),
            (markers::STR_L2, 2),
            (markers::STR_L4, 4),
        ] {
            if length as u64 >= 1_u64 << (width * 8) {
                continue;
            }
            let mut encoded = vec![marker];
            encoded.extend_from_slice(&(length as u32).to_le_bytes()[..width]);
            encoded.extend_from_slice(text.as_bytes());
            let bytes = envelope(&[encoded.clone(), object(&[(&text, encoded)])]);
            assert_eq!(
                parse_envelope_page(&ResponseBody::from_bytes(bytes.clone()), 1)
                    .unwrap()
                    .len(),
                2
            );
            check_against_full_decode(bytes);
        }
    }
    for (marker, width, counted) in [
        (markers::ARR_L1, 1, false),
        (markers::ARR_L2, 2, false),
        (markers::ARR_L4, 4, false),
        (markers::ARR_LC1, 1, true),
        (markers::ARR_LC2, 2, true),
        (markers::ARR_LC4, 4, true),
        (markers::OBJ_L1, 1, false),
        (markers::OBJ_L2, 2, false),
        (markers::OBJ_L4, 4, false),
        (markers::OBJ_LC1, 1, true),
        (markers::OBJ_LC2, 2, true),
        (markers::OBJ_LC4, 4, true),
    ] {
        let is_object = marker >= markers::OBJ_L1;
        for count in [0_u32, 1, 2] {
            let mut body = Vec::new();
            for index in 0..count {
                if is_object {
                    body.extend(string(&format!("key-{index}")));
                }
                body.push(markers::NULL);
            }
            let mut encoded = vec![marker];
            encoded.extend_from_slice(&(body.len() as u32).to_le_bytes()[..width]);
            if counted {
                encoded.extend_from_slice(&count.to_le_bytes()[..width]);
            }
            encoded.extend(body);
            check_against_full_decode(envelope(std::slice::from_ref(&encoded)));
            assert!(parse_envelope_page(
                &ResponseBody::from_bytes(envelope(std::slice::from_ref(&encoded))),
                1
            )
            .is_ok());
            if counted {
                encoded[1 + width] = (count + 1) as u8;
                assert!(
                    parse_envelope_page(&ResponseBody::from_bytes(envelope(&[encoded])), 1)
                        .is_err()
                );
            }
        }
    }
}

#[test]
fn reference_chains_and_cycles_are_rejected_in_context() {
    for self_reference in [false, true] {
        let first = vec![markers::STR_R4, 0, 0, 0, 0];
        let mut bytes = envelope(&[first.clone(), first]).to_vec();
        let positions: Vec<_> = bytes
            .windows(5)
            .enumerate()
            .filter_map(|(index, window)| {
                (window == [markers::STR_R4, 0, 0, 0, 0]).then_some(index)
            })
            .collect();
        assert_eq!(positions.len(), 2);
        for index in 0..2 {
            let target = positions[if self_reference { index } else { 1 - index }];
            bytes[positions[index] + 1..positions[index] + 5]
                .copy_from_slice(&(target as u32).to_le_bytes());
        }
        assert!(binary_json::decode(&bytes).is_err());
        assert_eq!(
            parse_envelope_page(&ResponseBody::from_bytes(bytes), 1)
                .unwrap_err()
                .status(),
            status_codes::SERIALIZATION_RESPONSE_BODY_INVALID
        );
    }
}

#[test]
fn contextual_typed_numbers_match_standalone_at_integer_boundaries() {
    for value in [
        json!(i64::MIN),
        json!(i64::MAX),
        json!(u64::MAX),
        json!(9_007_199_254_740_992_u64),
    ] {
        let bytes = envelope(&[binary_json::encode(&value)[1..].to_vec()]);
        let rows = parse_envelope_page(&ResponseBody::from_bytes(bytes), 1).unwrap();
        let EnvelopePayload::Binary(item) = &rows[0].payload else {
            panic!("binary")
        };
        if let Some(expected) = value.as_u64() {
            assert_eq!(item.deserialize::<u64>().unwrap(), expected);
            assert_eq!(
                binary_json::from_slice::<u64>(&item.to_standalone().unwrap()).unwrap(),
                expected
            );
        } else {
            assert_eq!(item.deserialize::<i64>().unwrap(), value.as_i64().unwrap());
            assert!(item.deserialize::<u64>().is_err());
        }
        assert!(item.deserialize::<bool>().is_err());
    }
}

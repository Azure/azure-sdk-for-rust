// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Binary fixtures with fixed-width framing so offsets can be assigned directly.

use super::{encode, markers, PREAMBLE};
use bytes::Bytes;
use serde_json::json;

pub(crate) fn string(value: &str) -> Vec<u8> {
    encode(&json!(value))[1..].to_vec()
}

fn container(marker: u8, count: usize, body: Vec<u8>) -> Vec<u8> {
    let mut bytes = vec![marker];
    bytes.extend_from_slice(&u32::try_from(body.len()).unwrap().to_le_bytes());
    bytes.extend_from_slice(&u32::try_from(count).unwrap().to_le_bytes());
    bytes.extend(body);
    bytes
}

pub(crate) fn array(items: &[Vec<u8>]) -> Vec<u8> {
    container(
        markers::ARR_LC4,
        items.len(),
        items.iter().flatten().copied().collect(),
    )
}

pub(crate) fn object(fields: &[(&str, Vec<u8>)]) -> Vec<u8> {
    let mut body = Vec::new();
    for (name, value) in fields {
        body.extend(string(name));
        body.extend(value);
    }
    container(markers::OBJ_LC4, fields.len(), body)
}

pub(crate) fn envelope(payloads: &[Vec<u8>]) -> Bytes {
    let rows: Vec<_> = payloads
        .iter()
        .enumerate()
        .map(|(i, payload)| {
            object(&[
                ("payload", payload.clone()),
                ("extra", vec![markers::NULL]),
                (
                    "orderByItems",
                    array(&[object(&[("item", encode(&json!(i))[1..].to_vec())])]),
                ),
                ("_rid", string(&format!("rid-{i}"))),
            ])
        })
        .collect();
    let mut page = vec![PREAMBLE];
    page.extend(object(&[("Documents", array(&rows))]));
    Bytes::from(page)
}

pub(crate) fn reference_rows(rows: &[(&str, i64)], target: &str) -> Bytes {
    // The first value begins after the fixed root frame and its property name.
    let offset = 1 + 9 + string("shared").len();
    let mut reference = vec![markers::STR_R4];
    reference.extend_from_slice(&u32::try_from(offset).unwrap().to_le_bytes());
    let documents: Vec<_> = rows
        .iter()
        .map(|(id, rank)| {
            let mut rid = [0u8; 16];
            rid[..8].copy_from_slice(&[0x0a, 0x0b, 0x0c, 0x0d, 0x80, 1, 2, 3]);
            rid[8..].copy_from_slice(&rank.to_le_bytes());
            object(&[
                (
                    "_rid",
                    string(&crate::models::resource_id::encode_rid(&rid)),
                ),
                (
                    "orderByItems",
                    array(&[object(&[("item", encode(&json!(rank))[1..].to_vec())])]),
                ),
                (
                    "payload",
                    object(&[("id", string(id)), ("shared", reference.clone())]),
                ),
            ])
        })
        .collect();
    let mut page = vec![PREAMBLE];
    page.extend(object(&[
        ("shared", string(target)),
        ("Documents", array(&documents)),
    ]));
    Bytes::from(page)
}

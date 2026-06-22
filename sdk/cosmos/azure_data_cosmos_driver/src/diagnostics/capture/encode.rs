// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! A compact binary serialization of an [`EventLogStorage`]'s two lists — a **cold-path** concern.
//!
//! The hot path stores typed structs (see [`super::event`]); it never touches this codec. When a
//! captured log needs to cross a process or storage boundary in compact form, [`encode`] flattens
//! the spans and attrs into a varint/TLV byte stream and [`decode`] restores them. Keeping the
//! variable-length-integer machinery here (rather than on the append path) is the whole point: the
//! binary format is just a compact way of storing the two lists, paid only when something actually
//! asks for the bytes.

use super::event::{Attr, AttrKey, AttrValue, EventLogStorage, Span, SpanId, SpanKind, TimeOffset};
use crate::error::{CosmosStatus, SubStatusCode};
use azure_core::http::StatusCode;

// LEB128 varint helpers (used only by this cold-path codec).

fn write_varint(out: &mut Vec<u8>, mut value: u64) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

fn read_varint(input: &[u8], pos: &mut usize) -> Option<u64> {
    let mut result: u64 = 0;
    let mut shift = 0;
    loop {
        let byte = *input.get(*pos)?;
        *pos += 1;
        result |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= 64 {
            return None;
        }
    }
    Some(result)
}

fn write_str(out: &mut Vec<u8>, value: &str) {
    write_varint(out, value.len() as u64);
    out.extend_from_slice(value.as_bytes());
}

fn read_str(input: &[u8], pos: &mut usize) -> Option<Box<str>> {
    let len = usize::try_from(read_varint(input, pos)?).ok()?;
    let end = pos.checked_add(len)?;
    let bytes = input.get(*pos..end)?;
    let s = std::str::from_utf8(bytes).ok()?.to_string();
    *pos = end;
    Some(s.into_boxed_str())
}

// Value tags for `AttrValue`. The three string flavors (owned / static / shared) all serialize as
// a string and decode to the owned `Str` variant — the distinction is a hot-path allocation
// optimization, not a semantic difference the byte format needs to preserve.
const VALUE_U64: u8 = 0;
const VALUE_F64: u8 = 1;
const VALUE_STR: u8 = 2;
const VALUE_STATUS: u8 = 3;

fn write_parent(out: &mut Vec<u8>, parent: Option<SpanId>) {
    // `None` (the root) is `0`; a real parent is its non-zero wire id.
    write_varint(out, parent.map_or(0, SpanId::get).into());
}

/// Serializes an [`EventLogStorage`] into a compact binary form.
pub(crate) fn encode(log: &EventLogStorage) -> Vec<u8> {
    let spans = log.spans();
    let attrs = log.attrs();
    let mut out = Vec::with_capacity(8 + spans.len() * 8 + attrs.len() * 6);
    write_varint(&mut out, spans.len() as u64);
    write_varint(&mut out, attrs.len() as u64);

    for span in spans {
        out.push(span.kind as u8);
        write_parent(&mut out, span.parent);
        write_varint(&mut out, span.start.as_nanos());
        write_varint(&mut out, span.end.as_nanos());
    }

    for attr in attrs {
        write_varint(&mut out, u64::from(attr.span.get()));
        out.push(attr.key as u8);
        match &attr.value {
            AttrValue::U64(v) => {
                out.push(VALUE_U64);
                write_varint(&mut out, *v);
            }
            AttrValue::F64(v) => {
                out.push(VALUE_F64);
                out.extend_from_slice(&v.to_le_bytes());
            }
            AttrValue::Status(s) => {
                out.push(VALUE_STATUS);
                write_varint(&mut out, u64::from(u16::from(s.status_code())));
                // Store sub-status as `value + 1` so `0` means "no sub-status".
                write_varint(
                    &mut out,
                    s.sub_status().map_or(0, |s| u64::from(s.value()) + 1),
                );
            }
            AttrValue::StaticStr(v) => {
                out.push(VALUE_STR);
                write_str(&mut out, v);
            }
            AttrValue::SharedStr(v) => {
                out.push(VALUE_STR);
                write_str(&mut out, v);
            }
            AttrValue::Str(v) => {
                out.push(VALUE_STR);
                write_str(&mut out, v);
            }
        }
    }

    out
}

/// Restores an [`EventLogStorage`] from [`encode`] output. Returns `None` on malformed input.
pub(crate) fn decode(input: &[u8]) -> Option<EventLogStorage> {
    let mut pos = 0usize;
    let span_count = usize::try_from(read_varint(input, &mut pos)?).ok()?;
    let attr_count = usize::try_from(read_varint(input, &mut pos)?).ok()?;

    // Cap the pre-allocation against the remaining input: every span/attr needs at least one byte,
    // so a claimed count larger than the remaining bytes is malformed and must not drive a giant
    // allocation. `with_capacity` is then bounded by real data, not an attacker-supplied length.
    let remaining = input.len().saturating_sub(pos);
    let mut spans = Vec::with_capacity(span_count.min(remaining));
    for _ in 0..span_count {
        let kind = SpanKind::from_u8(*input.get(pos)?);
        pos += 1;
        let raw_parent = u32::try_from(read_varint(input, &mut pos)?).ok()?;
        let parent = if raw_parent == 0 {
            None
        } else {
            Some(SpanId::from_raw(raw_parent)?)
        };
        let start = TimeOffset::from_nanos(read_varint(input, &mut pos)?);
        let end = TimeOffset::from_nanos(read_varint(input, &mut pos)?);
        spans.push(Span {
            kind,
            parent,
            start,
            end,
        });
    }

    let mut attrs = Vec::with_capacity(attr_count.min(input.len().saturating_sub(pos)));
    for _ in 0..attr_count {
        let span = SpanId::from_raw(u32::try_from(read_varint(input, &mut pos)?).ok()?)?;
        let key = AttrKey::from_u8(*input.get(pos)?)?;
        pos += 1;
        let tag = *input.get(pos)?;
        pos += 1;
        let value = match tag {
            VALUE_U64 => AttrValue::U64(read_varint(input, &mut pos)?),
            VALUE_F64 => {
                let mut bytes = [0u8; 8];
                let end = pos.checked_add(8)?;
                bytes.copy_from_slice(input.get(pos..end)?);
                pos = end;
                AttrValue::F64(f64::from_le_bytes(bytes))
            }
            VALUE_STATUS => {
                let code = u16::try_from(read_varint(input, &mut pos)?).ok()?;
                let raw_sub = read_varint(input, &mut pos)?;
                let sub = (raw_sub != 0)
                    .then(|| u16::try_from(raw_sub - 1).map(SubStatusCode::new))
                    .transpose()
                    .ok()?;
                AttrValue::Status(CosmosStatus::from_parts(StatusCode::from(code), sub))
            }
            VALUE_STR => AttrValue::Str(read_str(input, &mut pos)?),
            _ => return None,
        };
        attrs.push(Attr { span, key, value });
    }

    Some(EventLogStorage::from_parts(spans, attrs))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn sample() -> EventLogStorage {
        let mut log = EventLogStorage::new();
        let op = log.push_span(
            SpanKind::Operation,
            None,
            TimeOffset::ZERO,
            TimeOffset::from_nanos(7_000_000),
        );
        log.attr_str(op, AttrKey::OperationName, "read_item");
        log.attr_str(op, AttrKey::ActivityId, "act-2");
        log.attr_status(
            op,
            AttrKey::FinalStatus,
            CosmosStatus::from_parts(StatusCode::from(200), None),
        );

        let a0 = log.push_span(
            SpanKind::Attempt,
            Some(op),
            TimeOffset::ZERO,
            TimeOffset::from_nanos(3_000_000),
        );
        log.attr_u64(a0, AttrKey::ExecutionContext, 0);
        log.attr_str(a0, AttrKey::Region, "eastus");
        log.attr_status(
            a0,
            AttrKey::Status,
            CosmosStatus::from_parts(StatusCode::from(429), Some(SubStatusCode::new(3200))),
        );
        log.attr_f64(a0, AttrKey::RequestCharge, 4.2);

        let a1 = log.push_span(
            SpanKind::Attempt,
            Some(op),
            TimeOffset::from_nanos(3_000_000),
            TimeOffset::from_nanos(7_000_000),
        );
        log.attr_u64(a1, AttrKey::ExecutionContext, 1);
        log.attr_status(
            a1,
            AttrKey::Status,
            CosmosStatus::from_parts(StatusCode::from(200), None),
        );
        log
    }

    #[test]
    fn round_trips() {
        let log = sample();
        let bytes = encode(&log);
        let decoded = decode(&bytes).expect("decode succeeds");
        assert_eq!(decoded, log, "decode(encode(log)) == log");
    }

    #[test]
    fn string_flavors_decode_to_owned() {
        let mut log = EventLogStorage::new();
        let op = log.push_span(
            SpanKind::Operation,
            None,
            TimeOffset::ZERO,
            TimeOffset::ZERO,
        );
        log.attr_static_str(op, AttrKey::Region, "eastus");
        log.attr_shared_str(op, AttrKey::PrimaryRegion, Arc::from("westus"));
        let decoded = decode(&encode(&log)).expect("decode succeeds");
        // Static/shared strings come back as owned strings with the same content.
        assert_eq!(decoded.attr_str_of(op, AttrKey::Region), Some("eastus"));
        assert_eq!(
            decoded.attr_str_of(op, AttrKey::PrimaryRegion),
            Some("westus")
        );
    }

    #[test]
    fn empty_round_trips() {
        let log = EventLogStorage::new();
        let bytes = encode(&log);
        let decoded = decode(&bytes).expect("decode succeeds");
        assert!(decoded.is_empty());
    }

    #[test]
    fn truncated_input_is_rejected_not_panicked() {
        let bytes = encode(&sample());
        // Lopping off the tail must fail gracefully rather than panic.
        assert!(decode(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn unknown_attr_key_is_rejected() {
        // span_count=0, attr_count=1, span=1, key=250 (unknown).
        let bytes = [0u8, 1u8, 1u8, 250u8, VALUE_U64, 0u8];
        assert!(decode(&bytes).is_none());
    }

    #[test]
    fn oversized_declared_counts_do_not_overallocate_or_panic() {
        // A malformed header claiming a huge span/attr count must not drive a giant `with_capacity`
        // allocation; decode caps the pre-allocation against the remaining bytes and then fails
        // cleanly when the promised entries aren't actually present.
        // span_count = u64::MAX (varint), attr_count = 0, then no span bytes.
        let mut bytes = Vec::new();
        write_varint(&mut bytes, u64::MAX);
        write_varint(&mut bytes, 0);
        assert!(decode(&bytes).is_none());

        // attr_count enormous with no attr bytes following.
        let mut bytes = Vec::new();
        write_varint(&mut bytes, 0);
        write_varint(&mut bytes, 1_000_000_000);
        assert!(decode(&bytes).is_none());
    }
}

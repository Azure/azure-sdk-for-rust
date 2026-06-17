// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! A compact binary serialization of the [`EventLog`]'s two lists — a **cold-path** concern.
//!
//! The hot path stores typed structs (see [`super::event`]); it never touches this codec. When a
//! captured log needs to cross a process or storage boundary in compact form, [`encode`] flattens
//! the spans and attrs into a varint/TLV byte stream and [`decode`] restores them. Keeping the
//! variable-length-integer machinery here (rather than on the append path) is the whole point: the
//! binary format is just a compact way of storing the two lists, paid only when something actually
//! asks for the bytes.

use super::event::{Attr, AttrKey, AttrValue, EventLog, Span, SpanKind, NO_PARENT};

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

// Value tags for `AttrValue`.
const VALUE_U64: u8 = 0;
const VALUE_F64: u8 = 1;
const VALUE_STR: u8 = 2;

/// Serializes an [`EventLog`] into a compact binary form.
pub(crate) fn encode(log: &EventLog) -> Vec<u8> {
    let spans = log.spans();
    let attrs = log.attrs();
    let mut out = Vec::with_capacity(8 + spans.len() * 8 + attrs.len() * 6);
    write_varint(&mut out, spans.len() as u64);
    write_varint(&mut out, attrs.len() as u64);

    for span in spans {
        out.push(span.kind as u8);
        // Store the root's `NO_PARENT` as 0 and every real parent as `parent + 1`, so parents
        // varint-encode small.
        let parent = if span.parent == NO_PARENT {
            0
        } else {
            u64::from(span.parent) + 1
        };
        write_varint(&mut out, parent);
        write_varint(&mut out, span.start_ns);
        write_varint(&mut out, span.end_ns);
    }

    for attr in attrs {
        write_varint(&mut out, u64::from(attr.span));
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
            AttrValue::Str(v) => {
                out.push(VALUE_STR);
                write_str(&mut out, v);
            }
        }
    }

    out
}

/// Restores an [`EventLog`] from [`encode`] output. Returns `None` on malformed input.
pub(crate) fn decode(input: &[u8]) -> Option<EventLog> {
    let mut pos = 0usize;
    let span_count = usize::try_from(read_varint(input, &mut pos)?).ok()?;
    let attr_count = usize::try_from(read_varint(input, &mut pos)?).ok()?;

    let mut spans = Vec::with_capacity(span_count);
    for _ in 0..span_count {
        let kind = SpanKind::from_u8(*input.get(pos)?);
        pos += 1;
        let raw_parent = read_varint(input, &mut pos)?;
        let parent = if raw_parent == 0 {
            NO_PARENT
        } else {
            u32::try_from(raw_parent - 1).ok()?
        };
        let start_ns = read_varint(input, &mut pos)?;
        let end_ns = read_varint(input, &mut pos)?;
        spans.push(Span {
            kind,
            parent,
            start_ns,
            end_ns,
        });
    }

    let mut attrs = Vec::with_capacity(attr_count);
    for _ in 0..attr_count {
        let span = u32::try_from(read_varint(input, &mut pos)?).ok()?;
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
            VALUE_STR => AttrValue::Str(read_str(input, &mut pos)?),
            _ => return None,
        };
        attrs.push(Attr { span, key, value });
    }

    Some(EventLog::from_parts(spans, attrs))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> EventLog {
        let mut log = EventLog::new();
        let op = log.push_span(SpanKind::Operation, NO_PARENT, 0, 7_000_000);
        log.attr_str(op, AttrKey::OperationName, "read_item");
        log.attr_str(op, AttrKey::ActivityId, "act-2");
        log.attr_u64(op, AttrKey::FinalStatus, 200);

        let a0 = log.push_span(SpanKind::Attempt, op, 0, 3_000_000);
        log.attr_u64(a0, AttrKey::ExecutionContext, 0);
        log.attr_str(a0, AttrKey::Region, "eastus");
        log.attr_u64(a0, AttrKey::Status, 429);
        log.attr_f64(a0, AttrKey::RequestCharge, 4.2);

        let a1 = log.push_span(SpanKind::Attempt, op, 3_000_000, 7_000_000);
        log.attr_u64(a1, AttrKey::ExecutionContext, 1);
        log.attr_u64(a1, AttrKey::Status, 200);
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
    fn empty_round_trips() {
        let log = EventLog::new();
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
        // span_count=0, attr_count=1, span=0, key=250 (unknown).
        let bytes = [0u8, 1u8, 0u8, 250u8, VALUE_U64, 0u8];
        assert!(decode(&bytes).is_none());
    }
}

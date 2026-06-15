// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The diagnostics **wire model** and a compact hand-rolled binary codec (`AZD1`).
//!
//! This is the opt-in detail tier of the deferred, threshold-gated diagnostics design. The
//! summary tier is enough for most investigations; when more detail is wanted, the captured
//! per-operation log is projected into a [`WireTree`] and encoded to the compact `AZD1` binary
//! format, which a shared `diag-decode` tool can turn back into JSON.
//!
//! The format is deterministic (the same input always yields the same bytes), self-describing
//! via a magic + version header, and optionally DEFLATE-compressed. The version byte is the
//! evolution anchor: a decoder rejects an unknown version rather than guessing.
//!
//! # Wire format (`AZD1`)
//!
//! ```text
//! magic   : 4 bytes  = b"AZD1"
//! version : u8       = 1
//! flags   : u8       = bit0 set => payload is DEFLATE-compressed
//! payload : the (optionally compressed) node stream:
//!   operation : varint len + utf8
//!   node_count: varint
//!   per node:
//!     parent   : varint (0 => root, else parent_index + 1)
//!     kind     : u8
//!     start_ns : varint
//!     duration : varint
//!     status   : varint (0 => not applicable)
//!     attr_cnt : varint
//!     per attr : varint len + utf8 key, varint len + utf8 value
//! ```

use flate2::{read::DeflateDecoder, write::DeflateEncoder, Compression};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// Magic bytes identifying an Azure diagnostics blob.
pub const MAGIC: &[u8; 4] = b"AZD1";
/// Current wire format version.
pub const VERSION: u8 = 1;
/// `flags` bit indicating the payload is DEFLATE-compressed.
pub const FLAG_COMPRESSED: u8 = 0b0000_0001;
/// Default size (in bytes of the uncompressed payload) above which [`encode_auto`] compresses.
///
/// Tuned so small point operations skip DEFLATE entirely — compressing a ~300 byte blob costs
/// far more CPU than it saves — while wide fan-out crosses the threshold and benefits. This is
/// the single compression CPU/size tuning knob.
pub const AUTO_COMPRESS_THRESHOLD: usize = 512;

/// The kind of a span node. Stored on the wire as a single byte.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum NodeKind {
    /// The root operation span (wraps all retries and fan-out).
    Operation = 0,
    /// A single HTTP attempt span.
    Attempt = 1,
    /// A transport-level span beneath an attempt.
    Transport = 2,
    /// A routing / query-plan child span (fan-out).
    Routing = 3,
    /// A backend query span.
    Query = 4,
}

impl NodeKind {
    /// Converts a raw wire byte into a [`NodeKind`], defaulting unknown values to [`NodeKind::Operation`].
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => NodeKind::Attempt,
            2 => NodeKind::Transport,
            3 => NodeKind::Routing,
            4 => NodeKind::Query,
            _ => NodeKind::Operation,
        }
    }
}

/// A single node in the flat wire tree. Children reference their parent by index.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WireNode {
    /// Parent node index, or `None` for the root.
    pub parent: Option<u32>,
    /// The node kind (see [`NodeKind`]).
    #[serde(default)]
    pub kind: u8,
    /// Start tick (nanoseconds, relative to the operation start).
    pub start_ns: u64,
    /// Duration in nanoseconds.
    pub duration_ns: u64,
    /// HTTP status code, or `0` when not applicable.
    pub status: u16,
    /// Attribute key/value pairs.
    pub attrs: Vec<(String, String)>,
}

impl WireNode {
    /// Returns the typed [`NodeKind`] for this node.
    pub fn node_kind(&self) -> NodeKind {
        NodeKind::from_u8(self.kind)
    }

    /// Looks up an attribute value by key.
    pub fn attr(&self, key: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

/// A full diagnostics tree as a flat, index-linked node list.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WireTree {
    /// The operation name.
    pub operation: String,
    /// All nodes, root first. Children appear after their parents.
    pub nodes: Vec<WireNode>,
}

impl WireTree {
    /// Returns the indices of the direct children of `parent`.
    pub fn children_of(&self, parent: u32) -> Vec<u32> {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.parent == Some(parent))
            .map(|(i, _)| i as u32)
            .collect()
    }
}

/// Appends `value` to `out` as an LEB128 unsigned varint.
///
/// Shared by the wire codec and by the append-only capture log so the two never diverge.
pub fn write_varint(out: &mut Vec<u8>, mut value: u64) {
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

/// Reads an LEB128 unsigned varint from `input` at `pos`, advancing `pos`.
pub fn read_varint(input: &[u8], pos: &mut usize) -> Result<u64, DecodeError> {
    let mut result: u64 = 0;
    let mut shift = 0;
    loop {
        let byte = *input.get(*pos).ok_or(DecodeError::UnexpectedEof)?;
        *pos += 1;
        result |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= 64 {
            return Err(DecodeError::Malformed("varint too long"));
        }
    }
    Ok(result)
}

/// Appends `value` to `out` as a varint length prefix followed by its UTF-8 bytes.
pub fn write_str(out: &mut Vec<u8>, value: &str) {
    write_varint(out, value.len() as u64);
    out.extend_from_slice(value.as_bytes());
}

/// Reads a length-prefixed UTF-8 string from `input` at `pos`, advancing `pos`.
pub fn read_str(input: &[u8], pos: &mut usize) -> Result<String, DecodeError> {
    let len = usize::try_from(read_varint(input, pos)?)
        .map_err(|_| DecodeError::Malformed("string length exceeds platform limits"))?;
    let end = pos.checked_add(len).ok_or(DecodeError::UnexpectedEof)?;
    let bytes = input.get(*pos..end).ok_or(DecodeError::UnexpectedEof)?;
    let s = std::str::from_utf8(bytes)
        .map_err(|_| DecodeError::Malformed("invalid utf8"))?
        .to_string();
    *pos = end;
    Ok(s)
}

fn write_payload(tree: &WireTree) -> Vec<u8> {
    let mut payload = Vec::new();
    write_str(&mut payload, &tree.operation);
    write_varint(&mut payload, tree.nodes.len() as u64);
    for node in &tree.nodes {
        match node.parent {
            Some(p) => write_varint(&mut payload, u64::from(p) + 1),
            None => write_varint(&mut payload, 0),
        }
        payload.push(node.kind);
        write_varint(&mut payload, node.start_ns);
        write_varint(&mut payload, node.duration_ns);
        write_varint(&mut payload, u64::from(node.status));
        write_varint(&mut payload, node.attrs.len() as u64);
        for (k, v) in &node.attrs {
            write_str(&mut payload, k);
            write_str(&mut payload, v);
        }
    }
    payload
}

fn read_payload(payload: &[u8]) -> Result<WireTree, DecodeError> {
    // Cap pre-allocation so a malformed blob with a huge node/attr count cannot force a giant
    // up-front allocation (DoS). The Vec still grows as real, EOF-bounded content is decoded.
    const PREALLOC_CAP: usize = 1024;
    let mut pos = 0usize;
    let operation = read_str(payload, &mut pos)?;
    let count = usize::try_from(read_varint(payload, &mut pos)?)
        .map_err(|_| DecodeError::Malformed("node count exceeds platform limits"))?;
    let mut nodes = Vec::with_capacity(count.min(PREALLOC_CAP));
    for _ in 0..count {
        let raw_parent = read_varint(payload, &mut pos)?;
        let parent = if raw_parent == 0 {
            None
        } else {
            Some(
                u32::try_from(raw_parent - 1)
                    .map_err(|_| DecodeError::Malformed("parent index out of range"))?,
            )
        };
        let kind = *payload.get(pos).ok_or(DecodeError::UnexpectedEof)?;
        pos += 1;
        let start_ns = read_varint(payload, &mut pos)?;
        let duration_ns = read_varint(payload, &mut pos)?;
        let status = u16::try_from(read_varint(payload, &mut pos)?)
            .map_err(|_| DecodeError::Malformed("status code out of range"))?;
        let attr_count = usize::try_from(read_varint(payload, &mut pos)?)
            .map_err(|_| DecodeError::Malformed("attribute count exceeds platform limits"))?;
        let mut attrs = Vec::with_capacity(attr_count.min(PREALLOC_CAP));
        for _ in 0..attr_count {
            let key = read_str(payload, &mut pos)?;
            let value = read_str(payload, &mut pos)?;
            attrs.push((key, value));
        }
        nodes.push(WireNode {
            parent,
            kind,
            start_ns,
            duration_ns,
            status,
            attrs,
        });
    }
    Ok(WireTree { operation, nodes })
}

/// Encodes a [`WireTree`] to the `AZD1` binary format, compressing when `compress` is set.
pub fn encode(tree: &WireTree, compress: bool) -> Vec<u8> {
    let payload = write_payload(tree);
    let mut out = Vec::with_capacity(payload.len() + 6);
    out.extend_from_slice(MAGIC);
    out.push(VERSION);
    if compress {
        out.push(FLAG_COMPRESSED);
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::new(6));
        encoder
            .write_all(&payload)
            .expect("writing to an in-memory DEFLATE encoder never fails");
        let compressed = encoder
            .finish()
            .expect("finishing an in-memory DEFLATE encoder never fails");
        out.extend_from_slice(&compressed);
    } else {
        out.push(0);
        out.extend_from_slice(&payload);
    }
    out
}

/// Encodes a [`WireTree`], compressing only when the uncompressed payload exceeds
/// [`AUTO_COMPRESS_THRESHOLD`]. Keeps small blobs cheap and large ones small.
pub fn encode_auto(tree: &WireTree) -> Vec<u8> {
    let payload_len = write_payload(tree).len();
    encode(tree, payload_len > AUTO_COMPRESS_THRESHOLD)
}

/// An error encountered while decoding an `AZD1` blob.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// The blob did not start with the expected magic bytes.
    BadMagic,
    /// The blob version is not supported by this decoder (forward-compat guard).
    UnsupportedVersion(u8),
    /// The blob ended before decoding finished.
    UnexpectedEof,
    /// The blob was structurally invalid.
    Malformed(&'static str),
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::BadMagic => write!(f, "not an AZD1 diagnostics blob"),
            DecodeError::UnsupportedVersion(v) => write!(f, "unsupported AZD1 version {v}"),
            DecodeError::UnexpectedEof => write!(f, "unexpected end of blob"),
            DecodeError::Malformed(why) => write!(f, "malformed blob: {why}"),
        }
    }
}

impl std::error::Error for DecodeError {}

/// Decodes an `AZD1` binary blob (compressed or not) back into a [`WireTree`].
pub fn decode(blob: &[u8]) -> Result<WireTree, DecodeError> {
    if blob.len() < 6 {
        return Err(DecodeError::UnexpectedEof);
    }
    if &blob[0..4] != MAGIC {
        return Err(DecodeError::BadMagic);
    }
    let version = blob[4];
    if version != VERSION {
        return Err(DecodeError::UnsupportedVersion(version));
    }
    let flags = blob[5];
    let body = &blob[6..];
    if flags & FLAG_COMPRESSED != 0 {
        let mut decoder = DeflateDecoder::new(body);
        let mut payload = Vec::new();
        decoder
            .read_to_end(&mut payload)
            .map_err(|_| DecodeError::Malformed("DEFLATE inflate failed"))?;
        read_payload(&payload)
    } else {
        read_payload(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> WireTree {
        WireTree {
            operation: "read_item".to_string(),
            nodes: vec![
                WireNode {
                    parent: None,
                    kind: NodeKind::Operation as u8,
                    start_ns: 0,
                    duration_ns: 250,
                    status: 0,
                    attrs: vec![("az.operation".into(), "read_item".into())],
                },
                WireNode {
                    parent: Some(0),
                    kind: NodeKind::Attempt as u8,
                    start_ns: 0,
                    duration_ns: 250,
                    status: 200,
                    attrs: vec![("az.service_request_id".into(), "svc-200".into())],
                },
            ],
        }
    }

    #[test]
    fn round_trips_uncompressed() {
        let tree = sample_tree();
        let blob = encode(&tree, false);
        assert_eq!(&blob[0..4], MAGIC);
        assert_eq!(blob[4], VERSION);
        assert_eq!(blob[5] & FLAG_COMPRESSED, 0);
        assert_eq!(decode(&blob).unwrap(), tree);
    }

    #[test]
    fn round_trips_compressed() {
        let tree = sample_tree();
        let blob = encode(&tree, true);
        assert_eq!(blob[5] & FLAG_COMPRESSED, FLAG_COMPRESSED);
        assert_eq!(decode(&blob).unwrap(), tree);
    }

    #[test]
    fn rejects_bad_magic() {
        assert_eq!(decode(b"NOPE\x01\x00"), Err(DecodeError::BadMagic));
    }

    #[test]
    fn rejects_unknown_version() {
        let mut blob = encode(&sample_tree(), false);
        blob[4] = 99;
        assert_eq!(decode(&blob), Err(DecodeError::UnsupportedVersion(99)));
    }

    #[test]
    fn rejects_truncated_payload() {
        let blob = encode(&sample_tree(), false);
        // Lop off the back half of the payload: decode must error, not panic.
        let truncated = &blob[..blob.len() - 5];
        assert!(matches!(
            decode(truncated),
            Err(DecodeError::UnexpectedEof) | Err(DecodeError::Malformed(_))
        ));
    }

    #[test]
    fn rejects_oversized_string_length_without_allocating() {
        // magic + version + flags(uncompressed) + operation length = u64::MAX varint.
        let mut blob = Vec::new();
        blob.extend_from_slice(MAGIC);
        blob.push(VERSION);
        blob.push(0);
        write_varint(&mut blob, u64::MAX);
        // No string bytes follow; the decoder must reject rather than try to read u64::MAX bytes.
        assert!(matches!(
            decode(&blob),
            Err(DecodeError::Malformed(_)) | Err(DecodeError::UnexpectedEof)
        ));
    }

    #[test]
    fn read_varint_rejects_overlong() {
        // 10 continuation bytes overflow the 64-bit accumulator.
        let bytes = [0x80u8; 11];
        let mut pos = 0;
        assert_eq!(
            read_varint(&bytes, &mut pos),
            Err(DecodeError::Malformed("varint too long"))
        );
    }

    #[test]
    fn children_of_finds_direct_children() {
        let tree = sample_tree();
        assert_eq!(tree.children_of(0), vec![1]);
    }
}

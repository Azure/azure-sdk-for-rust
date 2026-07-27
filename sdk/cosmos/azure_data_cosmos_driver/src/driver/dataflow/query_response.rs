// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Helpers for trimming cross-partition query response pages.
//!
//! A query page from the backend is a JSON envelope of the shape
//! `{"_rid":"...","Documents":[ ... ],"_count":N}`. The [`SkipTake`] node
//! (`OFFSET` / `LIMIT` / `TOP`) needs to drop and truncate documents *across*
//! pages while leaving each surviving document byte-for-byte identical (so that
//! numeric precision and key ordering are preserved). To do that we deserialize
//! the `Documents` array as a list of [`RawValue`]s — which borrow the original
//! bytes verbatim — trim the list, and re-serialize a new envelope.
//!
//! [`SkipTake`]: super::SkipTake

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

/// The result of applying skip/take to a single page body.
pub(crate) struct SkipTakeOutcome {
    /// Number of documents dropped to satisfy the outstanding `OFFSET`.
    pub dropped: u64,
    /// Number of documents kept in the re-serialized body.
    pub emitted: u64,
    /// The re-serialized `{"Documents":[...],"_count":emitted}` envelope.
    pub body: Vec<u8>,
}

/// Incoming page envelope. Only `Documents` and `_rid` are retained; every
/// document is kept as raw bytes so re-serialization is lossless.
#[derive(Deserialize)]
struct RawQueryPage<'a> {
    #[serde(rename = "Documents", borrow, default)]
    documents: Vec<&'a RawValue>,
    #[serde(rename = "_rid", borrow, default)]
    rid: Option<&'a RawValue>,
}

/// Outgoing page envelope, mirroring the wire shape the backend produces.
#[derive(Serialize)]
struct OutQueryPage<'a> {
    #[serde(rename = "_rid", skip_serializing_if = "Option::is_none")]
    rid: Option<&'a RawValue>,
    #[serde(rename = "Documents")]
    documents: &'a [&'a RawValue],
    #[serde(rename = "_count")]
    count: u64,
}

/// Drops up to `skip` documents from `body`, then keeps up to `take`
/// (`None` = unbounded) of the remainder, returning the re-serialized envelope
/// and the counts consumed.
///
/// An empty (`NoPayload`) body is treated as a zero-document page.
pub(crate) fn skip_take_page(
    body: &[u8],
    skip: u64,
    take: Option<u64>,
) -> crate::error::Result<SkipTakeOutcome> {
    let page: RawQueryPage = if body.is_empty() {
        RawQueryPage {
            documents: Vec::new(),
            rid: None,
        }
    } else {
        serde_json::from_slice(body).map_err(|e| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("failed to parse cross-partition query page envelope")
                .with_source(e)
                .build()
        })?
    };

    let available = page.documents.len() as u64;
    let dropped = skip.min(available);
    let remaining = available - dropped;
    let emitted = match take {
        Some(t) => t.min(remaining),
        None => remaining,
    };

    let start = dropped as usize;
    let end = start + emitted as usize;
    let kept = &page.documents[start..end];

    let out = OutQueryPage {
        rid: page.rid,
        documents: kept,
        count: emitted,
    };
    let body = serde_json::to_vec(&out).map_err(|e| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
            .with_message("failed to re-serialize trimmed query page envelope")
            .with_source(e)
            .build()
    })?;

    Ok(SkipTakeOutcome {
        dropped,
        emitted,
        body,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn docs(body: &[u8]) -> Vec<serde_json::Value> {
        let v: serde_json::Value = serde_json::from_slice(body).unwrap();
        v["Documents"].as_array().unwrap().clone()
    }

    fn count(body: &[u8]) -> u64 {
        let v: serde_json::Value = serde_json::from_slice(body).unwrap();
        v["_count"].as_u64().unwrap()
    }

    #[test]
    fn take_only_truncates() {
        let body = br#"{"Documents":[{"id":1},{"id":2},{"id":3}],"_count":3}"#;
        let out = skip_take_page(body, 0, Some(2)).unwrap();
        assert_eq!(out.dropped, 0);
        assert_eq!(out.emitted, 2);
        assert_eq!(count(&out.body), 2);
        let d = docs(&out.body);
        assert_eq!(d.len(), 2);
        assert_eq!(d[0]["id"], 1);
        assert_eq!(d[1]["id"], 2);
    }

    #[test]
    fn skip_then_take() {
        let body = br#"{"Documents":[{"id":1},{"id":2},{"id":3},{"id":4}],"_count":4}"#;
        let out = skip_take_page(body, 1, Some(2)).unwrap();
        assert_eq!(out.dropped, 1);
        assert_eq!(out.emitted, 2);
        let d = docs(&out.body);
        assert_eq!(d[0]["id"], 2);
        assert_eq!(d[1]["id"], 3);
    }

    #[test]
    fn skip_exceeds_page() {
        let body = br#"{"Documents":[{"id":1},{"id":2}],"_count":2}"#;
        let out = skip_take_page(body, 5, Some(3)).unwrap();
        assert_eq!(out.dropped, 2);
        assert_eq!(out.emitted, 0);
        assert_eq!(count(&out.body), 0);
        assert!(docs(&out.body).is_empty());
    }

    #[test]
    fn unbounded_take_keeps_remainder() {
        let body = br#"{"Documents":[{"id":1},{"id":2},{"id":3}],"_count":3}"#;
        let out = skip_take_page(body, 1, None).unwrap();
        assert_eq!(out.emitted, 2);
        let d = docs(&out.body);
        assert_eq!(d[0]["id"], 2);
        assert_eq!(d[1]["id"], 3);
    }

    #[test]
    fn empty_body_is_zero_documents() {
        let out = skip_take_page(b"", 3, Some(5)).unwrap();
        assert_eq!(out.dropped, 0);
        assert_eq!(out.emitted, 0);
        assert_eq!(count(&out.body), 0);
    }

    #[test]
    fn preserves_numeric_precision() {
        // A high-precision number must survive the round-trip unchanged.
        let body = br#"{"Documents":[{"v":1.7976931348623157e308},{"v":100000000000000000001}],"_count":2}"#;
        let out = skip_take_page(body, 0, Some(2)).unwrap();
        let s = String::from_utf8(out.body).unwrap();
        assert!(s.contains("1.7976931348623157e308"));
        assert!(s.contains("100000000000000000001"));
    }

    #[test]
    fn preserves_rid() {
        let body = br#"{"_rid":"abc==","Documents":[{"id":1}],"_count":1}"#;
        let out = skip_take_page(body, 0, Some(1)).unwrap();
        let v: serde_json::Value = serde_json::from_slice(&out.body).unwrap();
        assert_eq!(v["_rid"], "abc==");
    }

    #[test]
    fn malformed_body_errors() {
        let out = skip_take_page(b"not json", 0, None);
        assert!(out.is_err());
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Helpers for trimming cross-partition query response pages.
//!
//! A query page from the backend is a JSON envelope of the shape
//! `{"_rid":"...","Documents":[ ... ],"_count":N}`. The [`SkipTake`] node
//! (`OFFSET` / `LIMIT` / `TOP`) needs to drop and truncate documents *across*
//! pages while leaving each surviving document byte-for-byte identical (so that
//! numeric precision and key ordering are preserved).
//!
//! Rather than re-serialize a trimmed envelope back to bytes (which the calling
//! SDK would then have to re-parse), these helpers split the `Documents` array
//! into a list of per-document [`Bytes`] — each a zero-copy
//! [`slice_ref`](bytes::Bytes::slice_ref) of the original page buffer — and
//! trim that list. [`SkipTake`] emits the surviving slices directly as a
//! [`ResponseBody::Items`](crate::models::ResponseBody::Items) body.
//!
//! [`SkipTake`]: super::SkipTake

use bytes::Bytes;
use serde::Deserialize;
use serde_json::value::RawValue;

/// The result of applying skip/take to a page's documents.
pub(crate) struct SkipTakeOutcome {
    /// Number of documents dropped to satisfy the outstanding `OFFSET`.
    pub dropped: u64,
    /// Number of documents kept (equal to `items.len()`).
    pub emitted: u64,
    /// The surviving per-document payloads, each an unmodified slice of the
    /// original page bytes.
    pub items: Vec<Bytes>,
}

/// Incoming page envelope. Only `Documents` is retained; every document is kept
/// as a borrowed [`RawValue`] so the surviving slices are byte-for-byte
/// identical to the backend bytes.
#[derive(Deserialize)]
struct RawQueryPage<'a> {
    #[serde(rename = "Documents", borrow, default)]
    documents: Vec<&'a RawValue>,
}

/// Splits a backend query-page envelope into a list of per-document payloads,
/// each a zero-copy [`slice_ref`](bytes::Bytes::slice_ref) of `body`.
///
/// An empty (`NoPayload`) body is treated as a zero-document page.
pub(crate) fn split_feed_envelope(body: &Bytes) -> crate::error::Result<Vec<Bytes>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let page: RawQueryPage = serde_json::from_slice(body).map_err(|e| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
            .with_message("failed to parse cross-partition query page envelope")
            .with_source(e)
            .build()
    })?;
    Ok(page
        .documents
        .iter()
        // `slice_ref` is zero-copy: the `RawValue` borrows from `body`, so its
        // bytes lie within the same allocation and no document is re-serialized.
        .map(|raw| body.slice_ref(raw.get().as_bytes()))
        .collect())
}

/// Drops up to `skip` documents from `items`, then keeps up to `take`
/// (`None` = unbounded) of the remainder, returning the surviving slices and
/// the counts consumed. Each surviving [`Bytes`] is returned unmodified.
pub(crate) fn skip_take_items(items: Vec<Bytes>, skip: u64, take: Option<u64>) -> SkipTakeOutcome {
    let available = items.len() as u64;
    let dropped = skip.min(available);
    let remaining = available - dropped;
    let emitted = match take {
        Some(t) => t.min(remaining),
        None => remaining,
    };

    let start = dropped as usize;
    let end = start + emitted as usize;
    let kept: Vec<Bytes> = items[start..end].to_vec();

    SkipTakeOutcome {
        dropped,
        emitted,
        items: kept,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(items: &[Bytes]) -> Vec<serde_json::Value> {
        items
            .iter()
            .map(|b| serde_json::from_slice(b).unwrap())
            .collect()
    }

    /// The exact JSON text of each surviving slice, so tests assert the output
    /// bytes directly rather than only a re-parsed shape.
    fn raw(items: &[Bytes]) -> Vec<String> {
        items
            .iter()
            .map(|b| String::from_utf8(b.to_vec()).unwrap())
            .collect()
    }

    fn envelope(body: &[u8]) -> Bytes {
        Bytes::copy_from_slice(body)
    }

    #[test]
    fn take_only_truncates() {
        let body = envelope(br#"{"Documents":[{"id":1},{"id":2},{"id":3}],"_count":3}"#);
        let items = split_feed_envelope(&body).unwrap();
        let out = skip_take_items(items, 0, Some(2));
        assert_eq!(out.dropped, 0);
        assert_eq!(out.emitted, 2);
        // Assert the exact surviving bytes, not just the parsed shape.
        assert_eq!(raw(&out.items), vec![r#"{"id":1}"#, r#"{"id":2}"#]);
    }

    #[test]
    fn skip_then_take() {
        let body = envelope(br#"{"Documents":[{"id":1},{"id":2},{"id":3},{"id":4}],"_count":4}"#);
        let items = split_feed_envelope(&body).unwrap();
        let out = skip_take_items(items, 1, Some(2));
        assert_eq!(out.dropped, 1);
        assert_eq!(out.emitted, 2);
        assert_eq!(raw(&out.items), vec![r#"{"id":2}"#, r#"{"id":3}"#]);
    }

    #[test]
    fn skip_exceeds_page() {
        let body = envelope(br#"{"Documents":[{"id":1},{"id":2}],"_count":2}"#);
        let items = split_feed_envelope(&body).unwrap();
        let out = skip_take_items(items, 5, Some(3));
        assert_eq!(out.dropped, 2);
        assert_eq!(out.emitted, 0);
        assert!(out.items.is_empty());
    }

    #[test]
    fn unbounded_take_keeps_remainder() {
        let body = envelope(br#"{"Documents":[{"id":1},{"id":2},{"id":3}],"_count":3}"#);
        let items = split_feed_envelope(&body).unwrap();
        let out = skip_take_items(items, 1, None);
        assert_eq!(out.emitted, 2);
        assert_eq!(ids(&out.items)[0]["id"], 2);
        assert_eq!(ids(&out.items)[1]["id"], 3);
    }

    #[test]
    fn empty_body_is_zero_documents() {
        let out = skip_take_items(split_feed_envelope(&Bytes::new()).unwrap(), 3, Some(5));
        assert_eq!(out.dropped, 0);
        assert_eq!(out.emitted, 0);
        assert!(out.items.is_empty());
    }

    #[test]
    fn preserves_numeric_precision() {
        // A high-precision number must survive the split unchanged, byte-for-byte.
        let body = envelope(
            br#"{"Documents":[{"v":1.7976931348623157e308},{"v":100000000000000000001}],"_count":2}"#,
        );
        let items = split_feed_envelope(&body).unwrap();
        let out = skip_take_items(items, 0, Some(2));
        assert_eq!(
            raw(&out.items),
            vec![
                r#"{"v":1.7976931348623157e308}"#,
                r#"{"v":100000000000000000001}"#,
            ]
        );
    }

    #[test]
    fn slices_are_zero_copy_verbatim() {
        // A document with unusual (but valid) internal spacing must be returned
        // exactly as it appeared in the source envelope, proving no
        // re-serialization occurred.
        let body = envelope(br#"{"Documents":[{"id":1,  "a":  [ 1, 2 ]}],"_count":1}"#);
        let items = split_feed_envelope(&body).unwrap();
        let out = skip_take_items(items, 0, None);
        assert_eq!(raw(&out.items), vec![r#"{"id":1,  "a":  [ 1, 2 ]}"#]);
    }

    #[test]
    fn malformed_body_errors() {
        let out = split_feed_envelope(&envelope(b"not json"));
        assert!(out.is_err());
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Helpers for trimming cross-partition query response pages.
//!
//! A query page from the backend is a JSON envelope of the shape
//! `{"_rid":"...","Documents":[ ... ],"_count":N}`. The [`SkipTake`] node
//! (`OFFSET` / `LIMIT` / `TOP`) needs to drop and truncate documents *across*
//! pages while preserving each surviving document's numeric precision and key
//! ordering.
//!
//! Rather than re-serialize a trimmed envelope back to bytes (which the calling
//! SDK would then have to re-parse), these helpers split the `Documents` array
//! into a list of per-document [`Bytes`] and trim that list. [`SkipTake`] emits
//! the surviving slices directly as a
//! [`ResponseBody::Items`](crate::models::ResponseBody::Items) body.
//!
//! A **text**-negotiated query splits with
//! [`slice_ref`](bytes::Bytes::slice_ref), so each surviving document is a
//! zero-copy view of the original buffer and is byte-for-byte identical to what
//! the service sent. A **binary**-negotiated query cannot be scanned in place:
//! the page is decoded to text to find the document boundaries, and each
//! surviving document is then re-encoded to standalone binary so the SDK's
//! per-slice `0x80` auto-detection still applies. That round trip is
//! value-preserving but not byte-preserving — the re-encoded document need not
//! match the bytes the service sent.
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
/// as a borrowed [`RawValue`] so a text-negotiated query's surviving slices are
/// byte-for-byte identical to the backend bytes.
#[derive(Deserialize)]
struct RawQueryPage<'a> {
    #[serde(rename = "Documents", borrow, default)]
    documents: Vec<&'a RawValue>,
}

/// Splits a backend query-page envelope into a list of per-document payloads.
///
/// An empty (`NoPayload`) body is treated as a zero-document page.
///
/// Payloads are always **text** here, regardless of what the operation
/// negotiated: a binary envelope is decoded to text so the `Documents` array
/// can be split, and each surviving document is re-encoded later by
/// [`encode_items`] — after the skip/take window has discarded the documents
/// this page does not contribute, so no discarded document costs a transcode or
/// can fail the query.
///
/// When the page arrived as text each payload is a zero-copy
/// [`slice_ref`](bytes::Bytes::slice_ref) of `body`.
pub(crate) fn split_feed_envelope(body: &Bytes) -> crate::error::Result<Vec<Bytes>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }

    // Decode binary pages so the scan below stays a plain text-JSON split.
    let body = &super::query_response::normalize_page_body(body)?;

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

/// Re-encodes surviving text payloads into the encoding the operation emits.
///
/// When `emit_binary` is false the payloads pass through unchanged, so a
/// text-negotiated query's slices stay byte-for-byte identical to the backend
/// bytes. When it is true each document is re-encoded to **standalone** binary
/// so the SDK's per-slice `0x80` auto-detection still applies; those payloads
/// are freshly allocated and value-preserving rather than byte-preserving.
///
/// `emit_binary` comes from the operation, not from the received bytes, so a
/// page the service answered in text on a binary-negotiated query still emits
/// the same format as every other node in the pipeline.
///
/// Called on the skip/take *survivors* only, so a document the window discards
/// cannot fail the query. That is the only property this ordering buys: the
/// caller has already consumed a page from its child by this point, so a
/// failure here leaves the window counters and the child's resume position
/// disagreeing and is not recoverable in place. See the encode branch in
/// [`SkipTake::next_page`](super::skip_take::SkipTake).
///
/// # Errors
///
/// Returns an error if a payload cannot be encoded to Cosmos binary JSON.
pub(crate) fn encode_items(
    items: Vec<Bytes>,
    emit_binary: bool,
) -> crate::error::Result<Vec<Bytes>> {
    if !emit_binary {
        return Ok(items);
    }

    items
        .iter()
        .map(|raw| {
            crate::binary_json::transcode_to_binary(raw)
                .map(Bytes::from)
                .map_err(|e| {
                    crate::error::CosmosError::builder()
                        .with_status(
                            crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID,
                        )
                        .with_message(
                            "failed to re-encode cross-partition query document to binary",
                        )
                        .with_source(e)
                        .build()
                })
        })
        .collect()
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

    /// Split then encode, the order `SkipTake` uses when a page contributes
    /// every document. Tests that exercise a skip/take window deliberately do
    /// not use this — they encode the survivors, which is the point.
    fn split_and_encode(body: &Bytes, emit_binary: bool) -> crate::error::Result<Vec<Bytes>> {
        encode_items(split_feed_envelope(body)?, emit_binary)
    }

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

    /// Splitting yields **text**, even for a binary page: encoding is deferred
    /// to `encode_items` so it runs on the skip/take survivors rather than on
    /// every document the page happened to carry.
    ///
    /// This pins the ordering that keeps a discarded document from paying a
    /// transcode — and, if the encoder ever grows a failure mode the envelope
    /// parse does not already reject, from failing a query it contributes
    /// nothing to.
    #[test]
    fn splitting_defers_encoding_so_discarded_documents_are_never_encoded() {
        let text = br#"{"Documents":[{"id":1},{"id":2},{"id":3}],"_count":3}"#;
        let binary = crate::binary_json::transcode_to_binary(text).unwrap();

        let items = split_feed_envelope(&Bytes::from(binary)).unwrap();
        assert_eq!(items.len(), 3);
        for item in &items {
            assert!(
                !crate::binary_json::is_binary(item),
                "split must hand back text; encoding belongs to encode_items",
            );
        }

        // Only the survivor is encoded.
        let out = skip_take_items(items, 2, Some(1));
        let encoded = encode_items(out.items, true).unwrap();
        assert_eq!(encoded.len(), 1);
        let doc: serde_json::Value = crate::binary_json::from_slice(&encoded[0]).unwrap();
        assert_eq!(doc, serde_json::json!({ "id": 3 }));
    }

    /// A page fully consumed by an outstanding skip encodes nothing at all.
    #[test]
    fn fully_skipped_page_encodes_nothing() {
        let body = envelope(br#"{"Documents":[{"id":1},{"id":2}],"_count":2}"#);
        let items = split_feed_envelope(&body).unwrap();
        let out = skip_take_items(items, 2, None);
        assert_eq!(out.emitted, 0);
        assert!(encode_items(out.items, true).unwrap().is_empty());
    }

    #[test]
    fn binary_page_splits_into_binary_items() {
        let text = br#"{"Documents":[{"id":1},{"id":2},{"id":3}],"_count":3}"#;
        let binary = crate::binary_json::transcode_to_binary(text).unwrap();
        assert!(crate::binary_json::is_binary(&binary));

        let items = split_feed_envelope(&Bytes::from(binary)).unwrap();
        let out = skip_take_items(items, 1, Some(1));
        let out_items = encode_items(out.items, true).unwrap();
        assert_eq!(out.dropped, 1);
        assert_eq!(out.emitted, 1);
        assert!(crate::binary_json::is_binary(&out_items[0]));
        let doc: serde_json::Value = crate::binary_json::from_slice(&out_items[0]).unwrap();
        assert_eq!(doc, serde_json::json!({ "id": 2 }));
    }

    #[test]
    fn binary_page_preserves_wide_integer_for_typed_decode() {
        // #5028: a text split surfaces the integral `Double` as a float that a
        // `u64` target rejects; the binary path re-encodes so it coerces back.
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct Doc {
            wide: u64,
        }
        let wide = serde_json::Number::from_f64(9_007_199_254_740_992.0).unwrap();
        let text = serde_json::to_vec(&serde_json::json!({
            "Documents": [{ "wide": wide }],
            "_count": 1,
        }))
        .unwrap();
        let binary = crate::binary_json::transcode_to_binary(&text).unwrap();

        let items = split_and_encode(&Bytes::from(binary), true).unwrap();
        assert_eq!(items.len(), 1);
        let doc: Doc = crate::binary_json::from_slice(&items[0]).unwrap();
        assert_eq!(
            doc,
            Doc {
                wide: 9_007_199_254_740_992
            }
        );
    }

    /// The emitted encoding follows the negotiated operation, not the bytes that
    /// happened to arrive. A binary-negotiated query whose page came back as
    /// text must still emit binary, so `SkipTake` and the ordered merge agree on
    /// the same query rather than each sniffing its own input.
    #[test]
    fn emitted_encoding_follows_negotiation_not_received_bytes() {
        let text = envelope(br#"{"Documents":[{"id":1},{"id":2}],"_count":2}"#);

        let negotiated_binary = split_and_encode(&text, true).unwrap();
        assert_eq!(negotiated_binary.len(), 2);
        for item in &negotiated_binary {
            assert!(
                crate::binary_json::is_binary(item),
                "a binary-negotiated query must emit binary even from a text page"
            );
        }

        let negotiated_text = split_feed_envelope(&text).unwrap();
        for item in &negotiated_text {
            assert!(
                !crate::binary_json::is_binary(item),
                "a text-negotiated query must stay text"
            );
        }
    }

    /// The binary counterpart of [`slices_are_zero_copy_verbatim`]: the text
    /// path proves items are handed back byte-for-byte, and the binary path
    /// must be pinned just as tightly. Comparing decoded values instead would
    /// accept any re-encoding that happens to mean the same thing, which is
    /// exactly the shape this file's tests were changed to stop accepting.
    #[test]
    fn binary_items_match_the_canonical_encoding_byte_for_byte() {
        let text = envelope(br#"{"Documents":[{"id":1,  "a":  [ 1, 2 ]}],"_count":1}"#);

        let items = split_and_encode(&text, true).unwrap();

        assert_eq!(items.len(), 1);
        let expected = crate::binary_json::encode(&serde_json::json!({ "id": 1, "a": [1, 2] }));
        assert_eq!(
            items[0].as_ref(),
            expected.as_slice(),
            "a binary-negotiated split must emit the canonical binary encoding, so a change \
             in marker width or key order is caught rather than absorbed",
        );
    }

    /// The binary counterpart of [`preserves_numeric_precision`]. Binary JSON
    /// carries every number as an IEEE-754 double, so the wide-integer literal
    /// the text path preserves verbatim cannot survive here — this pins what
    /// the binary path *does* do rather than leaving it unstated: doubles are
    /// bit-exact, and an integer beyond `u64` is representable only to double
    /// precision.
    #[test]
    fn binary_split_preserves_double_bits_and_documents_integer_widening() {
        let text = envelope(
            br#"{"Documents":[{"v":1.7976931348623157e308},{"v":100000000000000000001}],"_count":2}"#,
        );

        let items = split_and_encode(&text, true).unwrap();

        assert_eq!(items.len(), 2);
        let first: serde_json::Value = crate::binary_json::from_slice(&items[0]).unwrap();
        assert_eq!(
            first["v"].as_f64().unwrap().to_bits(),
            f64::MAX.to_bits(),
            "the largest finite double must survive the re-encode bit-exactly",
        );

        // `100000000000000000001` exceeds `u64`, so binary stores it as the
        // nearest double (1e20) — a real and deliberate difference from the
        // verbatim text path, recorded here so it is a decision rather than a
        // surprise.
        let second: serde_json::Value = crate::binary_json::from_slice(&items[1]).unwrap();
        assert_eq!(second["v"].as_f64().unwrap(), 1e20);
    }
}

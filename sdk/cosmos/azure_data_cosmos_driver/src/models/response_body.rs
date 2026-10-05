// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Response body container for Cosmos DB operations.
//!
//! Provides [`ResponseBody`], a typed response body that distinguishes between
//! single-payload responses (point reads/writes, batches) and feed-style
//! responses (Query / ChangeFeed) that carry one element per document.

use std::ops::Range;

use azure_core::{fmt::SafeDebug, Bytes};
use serde::de::DeserializeOwned;

/// A document backed by its original binary page or a standalone body.
///
/// A binary page's value bytes may contain references to strings elsewhere in
/// the [`source_page()`](Self::source_page). The
/// [`value_range()`](Self::value_range) is an absolute byte range within that
/// page; its slice might decode incorrectly without the rest of the page.
/// Use [`deserialize()`](Self::deserialize) for typed values, or
/// [`to_standalone()`](Self::to_standalone) for an independent byte buffer.
#[derive(Clone, SafeDebug)]
pub struct ItemView {
    source_page: Bytes,
    value_range: Range<usize>,
}

impl ItemView {
    pub(crate) fn standalone(bytes: Bytes) -> Self {
        Self {
            value_range: 0..bytes.len(),
            source_page: bytes,
        }
    }

    pub(crate) fn from_binary_page(
        page: Bytes,
        value_range: Range<usize>,
    ) -> crate::error::Result<Self> {
        if !crate::binary_json::is_binary(&page)
            || value_range.start == 0
            || value_range.start > value_range.end
            || page.get(value_range.clone()).is_none()
        {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("binary item range lies outside its source page")
                .build());
        }
        Ok(Self {
            source_page: page,
            value_range,
        })
    }

    /// Returns the full source buffer, including any string targets outside
    /// this item's value range.
    ///
    /// For standalone items, the source is the item itself, in text or binary.
    pub fn source_page(&self) -> &[u8] {
        &self.source_page
    }

    /// Returns this item's absolute byte offsets within
    /// [`source_page()`](Self::source_page), with an exclusive end.
    pub fn value_range(&self) -> Range<usize> {
        self.value_range.clone()
    }

    /// Returns the original value bytes.
    ///
    /// A binary value may refer outside this slice. Keep the source page when
    /// decoding it, or call [`to_standalone()`](Self::to_standalone).
    pub fn raw_value(&self) -> &[u8] {
        &self.source_page[self.value_range.clone()]
    }

    pub(crate) fn is_page_backed(&self) -> bool {
        self.value_range.start != 0
    }

    /// Returns a standalone document.
    ///
    /// A standalone item returns a shared clone of its original buffer. A
    /// page-backed item is decoded in context and re-encoded with a binary
    /// preamble. Its value is preserved, but its encoding can differ from the
    /// original bytes; use [`raw_value()`](Self::raw_value) for exact bytes.
    ///
    /// # Errors
    ///
    /// Returns a response-body error if the binary item cannot be decoded.
    pub fn to_standalone(&self) -> crate::error::Result<Bytes> {
        if !self.is_page_backed() {
            return Ok(self.source_page.clone());
        }
        let mut value =
            crate::binary_json::reader::decode_at(&self.source_page, self.value_range.clone())
                .map_err(|e| invalid_body_error("failed to materialize binary item", e))?;
        crate::binary_json::normalize_integral_floats(&mut value);
        Ok(Bytes::from(crate::binary_json::encode(&value)))
    }

    /// Deserializes an item using the whole source page for binary references.
    ///
    /// # Errors
    ///
    /// Returns a response-body error if the item is invalid or `T` rejects it.
    pub fn deserialize<T: DeserializeOwned>(&self) -> crate::error::Result<T> {
        if !self.is_page_backed() {
            return deserialize_response(&self.source_page, "failed to deserialize feed item");
        }
        match crate::binary_json::de::from_slice_at(&self.source_page, self.value_range.clone()) {
            Ok(value) => Ok(value),
            Err(crate::binary_json::BinaryError::Custom(message))
                if message.starts_with("duplicate field ") =>
            {
                let standalone = self.to_standalone()?;
                deserialize_response(&standalone, "failed to deserialize feed item")
            }
            Err(e) => Err(invalid_body_error("failed to deserialize feed item", e)),
        }
    }

    pub(crate) fn decoded_value(&self) -> crate::error::Result<serde_json::Value> {
        if !self.is_page_backed() {
            return deserialize_response(&self.source_page, "failed to decode feed item");
        }
        let mut value =
            crate::binary_json::reader::decode_at(&self.source_page, self.value_range.clone())
                .map_err(|e| invalid_body_error("failed to decode feed item", e))?;
        crate::binary_json::normalize_integral_floats(&mut value);
        Ok(value)
    }
}

/// The body of a [`CosmosResponse`](super::CosmosResponse).
///
/// Explicitly distinguishes between the four response shapes the driver
/// returns:
///
/// * [`ResponseBody::NoPayload`] — the service returned no body (e.g. HTTP
///   204 on a successful delete, or any other empty-body response).
/// * [`ResponseBody::Bytes`] — a single payload buffer. Used for point reads,
///   writes, batches, and any other operation that returns one document or
///   envelope.
/// * [`ResponseBody::Items`] — a list of pre-sliced per-document buffers. Used
///   for feed responses (Query / ChangeFeed) where the driver pipeline splits
///   the `Documents` array once via zero-copy [`Bytes::slice`](bytes::Bytes::slice)
///   so the SDK never needs to re-parse the envelope.
/// * [`ResponseBody::ContextualItems`] — documents with their complete binary
///   source pages, needed when a value refers to a string outside its span.
///
/// The payload variants carry shared ownership via reference-counted
/// [`bytes::Bytes`].
#[derive(Clone, Default, SafeDebug)]
pub enum ResponseBody {
    /// The service returned no response body.
    #[default]
    NoPayload,

    /// A single response payload (point read/write, batch, metadata, etc.).
    Bytes(Bytes),

    /// A list of per-document slices produced by the feed/query pipeline.
    Items(Vec<Bytes>),

    /// Original page-backed documents (and any standalone items interleaved
    /// with them) in the order returned by the query pipeline.
    ContextualItems(Vec<ItemView>),
}

impl ResponseBody {
    /// Creates an empty body (a [`NoPayload`](Self::NoPayload) response).
    pub fn empty() -> Self {
        Self::NoPayload
    }

    /// Builds a single-payload [`Bytes`](Self::Bytes) body, or
    /// [`NoPayload`](Self::NoPayload) if the input is empty.
    ///
    /// Use this for point reads/writes, batches, and any other operation that
    /// returns a single document or envelope.
    pub fn from_bytes(bytes: impl Into<Bytes>) -> Self {
        let bytes = bytes.into();
        if bytes.is_empty() {
            Self::NoPayload
        } else {
            Self::Bytes(bytes)
        }
    }

    /// Builds a feed-style [`Items`](Self::Items) body from pre-sliced
    /// per-document buffers.
    ///
    /// Use this for Query / ChangeFeed responses where the pipeline has
    /// already split the `Documents` array via zero-copy
    /// [`Bytes::slice`](bytes::Bytes::slice).
    pub fn from_items(items: Vec<Bytes>) -> Self {
        Self::Items(items)
    }

    pub(crate) fn from_item_views(items: Vec<ItemView>) -> Self {
        if items.iter().any(ItemView::is_page_backed) {
            Self::ContextualItems(items)
        } else {
            Self::Items(items.into_iter().map(|item| item.source_page).collect())
        }
    }

    /// Returns `true` if the body carries no readable content.
    ///
    /// * [`NoPayload`](Self::NoPayload) is always empty.
    /// * [`Bytes`](Self::Bytes) is empty when the single buffer has zero bytes.
    /// * [`Items`](Self::Items) is empty when the feed envelope contains zero
    ///   documents.
    /// * [`ContextualItems`](Self::ContextualItems) is empty when it contains
    ///   no item views.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::NoPayload => true,
            Self::Bytes(b) => b.is_empty(),
            Self::Items(items) => items.is_empty(),
            Self::ContextualItems(items) => items.is_empty(),
        }
    }

    /// Returns the single payload, or an error if the body is a feed response.
    /// A [`NoPayload`](Self::NoPayload) body yields an empty [`Bytes`].
    ///
    /// Used by single-document response paths (point reads/writes, batch, etc.).
    pub fn single(self) -> crate::error::Result<Bytes> {
        match self {
            Self::NoPayload => Ok(Bytes::new()),
            Self::Bytes(b) => Ok(b),
            Self::ContextualItems(items) => Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!(
                    "expected single response body, found feed response with {} item(s)",
                    items.len()
                ))
                .build()),
            Self::Items(items) => Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!(
                    "expected single response body, found feed response with {} item(s)",
                    items.len()
                ))
                .build()),
        }
    }

    /// Returns standalone per-item raw buffers of a feed response, or wraps a
    /// single-payload body as a one-element vector. A
    /// [`NoPayload`](Self::NoPayload) body yields an empty `Vec`.
    ///
    /// For [`ContextualItems`](Self::ContextualItems), each binary item is
    /// decoded in its full source page and re-encoded. These buffers are
    /// independently decodable but not byte-identical to the original
    /// document; use the views for exact source bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if a contextual item cannot be materialized.
    pub fn items(self) -> crate::error::Result<Vec<Bytes>> {
        match self {
            Self::NoPayload => Ok(Vec::new()),
            Self::Bytes(b) => Ok(vec![b]),
            Self::Items(items) => Ok(items),
            Self::ContextualItems(items) => items.iter().map(ItemView::to_standalone).collect(),
        }
    }

    /// Deserializes a single-payload body as type `T`, transparently accepting
    /// either Cosmos binary JSON or UTF-8 text JSON (auto-detected by the
    /// `0x80` preamble).
    ///
    /// Owned `serde_json::value::RawValue` is supported; on the binary path its
    /// raw text is the codec's normalized rendering, not the service bytes
    /// verbatim.
    ///
    /// Returns an error if the body is a feed [`Items`](Self::Items) response
    /// or if the body is [`NoPayload`](Self::NoPayload) (nothing to parse).
    pub fn into_single<T: DeserializeOwned>(self) -> crate::error::Result<T> {
        let bytes = self.single()?;
        deserialize_response(&bytes, "failed to deserialize response body")
    }

    /// Deserializes every item in a feed response, or the single payload, as
    /// type `T`. A [`NoPayload`](Self::NoPayload) body yields an empty `Vec`.
    ///
    /// Each standalone buffer is decoded as Cosmos binary JSON or UTF-8 text
    /// JSON (detected by the `0x80` preamble). Contextual binary items use the
    /// complete source page for reference strings.
    pub fn into_items<T: DeserializeOwned>(self) -> crate::error::Result<Vec<T>> {
        match self {
            Self::NoPayload => Ok(Vec::new()),
            Self::Bytes(b) => {
                let item = deserialize_response(&b, "failed to deserialize response body")?;
                Ok(vec![item])
            }
            Self::Items(items) => items
                .into_iter()
                // Items are decoded independently, auto-detecting binary per
                // slice via the `0x80` preamble. Safe because every producer
                // (`skip_take_page::split_feed_envelope`, `build_page`) emits
                // each document already standalone-encoded. A future splitter
                // that slices a single-preamble envelope by scanning text would
                // yield preamble-less sub-documents misrouted to the text path,
                // so it must re-prefix per document first.
                .map(|b| deserialize_response(&b, "failed to deserialize feed item"))
                .collect(),
            Self::ContextualItems(items) => items.iter().map(ItemView::deserialize).collect(),
        }
    }

    /// Transcodes any Cosmos **binary** JSON payload(s) to UTF-8 **text** JSON
    /// in place, leaving text payloads unchanged.
    ///
    /// Used by the driver when the operation requested a text response while
    /// keeping the wire binary (see
    /// [`BinaryEncodingOptions::request_text_response`](crate::options::BinaryEncodingOptions)).
    /// The conversion is schema-agnostic: each buffer is decoded with
    /// [`binary_json::decode`](crate::binary_json::decode) and re-serialized as
    /// compact text JSON. A [`NoPayload`](Self::NoPayload) body is a no-op.
    ///
    /// # Errors
    ///
    /// Returns an error if a binary payload is malformed.
    pub(crate) fn transcode_to_text(self) -> crate::error::Result<Self> {
        fn convert(bytes: &Bytes) -> crate::error::Result<Bytes> {
            // Already-text payloads are left unchanged: return a cheap refcount
            // clone instead of round-tripping through `transcode_to_text` (which
            // would copy the buffer into a fresh `Vec<u8>`).
            if !crate::binary_json::is_binary(bytes) {
                return Ok(bytes.clone());
            }
            let text = crate::binary_json::transcode_to_text(bytes).map_err(|e| {
                crate::error::CosmosError::builder()
                    .with_status(crate::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message(format!("failed to transcode binary response to text: {e}"))
                    .with_source(e)
                    .build()
            })?;
            Ok(Bytes::from(text))
        }

        match self {
            Self::NoPayload => Ok(Self::NoPayload),
            Self::Bytes(b) => Ok(Self::Bytes(convert(&b)?)),
            Self::Items(items) => {
                let converted = items
                    .into_iter()
                    .map(|b| convert(&b))
                    .collect::<crate::error::Result<Vec<_>>>()?;
                Ok(Self::Items(converted))
            }
            Self::ContextualItems(items) => {
                let converted = items
                    .iter()
                    .map(|item| {
                        if item.is_page_backed() {
                            serde_json::to_vec(&item.decoded_value()?)
                                .map(Bytes::from)
                                .map_err(|e| invalid_body_error("failed to transcode item", e))
                        } else {
                            convert(&item.source_page)
                        }
                    })
                    .collect::<crate::error::Result<Vec<_>>>()?;
                Ok(Self::Items(converted))
            }
        }
    }

    /// Transcodes text JSON payloads to standalone Cosmos binary JSON buffers.
    ///
    /// Each [`Items`](Self::Items) slice is encoded independently, so every
    /// resulting item carries the `0x80` preamble required for auto-detection.
    #[allow(dead_code)] // Defensive mirror for direct body conversion; pipeline nodes encode today.
    pub(crate) fn transcode_to_binary(self) -> crate::error::Result<Self> {
        fn convert(bytes: Bytes) -> crate::error::Result<Bytes> {
            if crate::binary_json::is_binary(&bytes) {
                return Ok(bytes);
            }
            crate::binary_json::transcode_to_binary(&bytes)
                .map(Bytes::from)
                .map_err(|e| invalid_body_error("failed to transcode response body to binary", e))
        }

        match self {
            Self::NoPayload => Ok(Self::NoPayload),
            Self::Bytes(bytes) => convert(bytes).map(Self::Bytes),
            Self::Items(items) => items
                .into_iter()
                .map(convert)
                .collect::<crate::error::Result<Vec<_>>>()
                .map(Self::Items),
            Self::ContextualItems(items) => items
                .iter()
                .map(ItemView::to_standalone)
                .collect::<crate::error::Result<Vec<_>>>()
                .map(Self::Items),
        }
    }
}

impl From<Bytes> for ResponseBody {
    fn from(bytes: Bytes) -> Self {
        Self::from_bytes(bytes)
    }
}

impl From<Vec<u8>> for ResponseBody {
    fn from(bytes: Vec<u8>) -> Self {
        Self::from_bytes(Bytes::from(bytes))
    }
}

/// Builds a `SERIALIZATION_RESPONSE_BODY_INVALID` error carrying `message` and
/// the underlying `source`.
fn invalid_body_error<E>(message: &'static str, source: E) -> crate::error::CosmosError
where
    E: std::error::Error + Send + Sync + 'static,
{
    crate::error::CosmosError::builder()
        .with_status(crate::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
        .with_message(message)
        .with_source(source)
        .build()
}

/// Deserializes a response buffer as `T`, transparently accepting either Cosmos
/// binary JSON or UTF-8 text JSON.
///
/// A buffer that begins with the binary preamble (`0x80`, detected by
/// [`is_binary`](crate::binary_json::is_binary)) is deserialized directly by the
/// binary-JSON codec's native serde deserializer
/// ([`from_slice`](crate::binary_json::from_slice)) — driving `T::deserialize`
/// straight off the bytes with no intermediate [`serde_json::Value`]; any other
/// buffer is parsed directly as text JSON. Because no UTF-8 text JSON document
/// can begin with `0x80` (it is a UTF-8 continuation byte), the detection is
/// unambiguous and the text path is byte-for-byte unchanged. `message` is the
/// error context attached on failure.
fn deserialize_response<T: DeserializeOwned>(
    bytes: &[u8],
    message: &'static str,
) -> crate::error::Result<T> {
    if crate::binary_json::is_binary(bytes) {
        crate::binary_json::from_slice(bytes).map_err(|e| invalid_body_error(message, e))
    } else {
        serde_json::from_slice(bytes).map_err(|e| invalid_body_error(message, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_no_payload() {
        let body = ResponseBody::default();
        assert!(matches!(body, ResponseBody::NoPayload));
        assert!(body.is_empty());
    }

    #[test]
    fn empty_constructor_is_no_payload() {
        let body = ResponseBody::empty();
        assert!(matches!(body, ResponseBody::NoPayload));
        assert!(body.is_empty());
    }

    #[test]
    fn no_payload_single_yields_empty_bytes() {
        let body = ResponseBody::NoPayload;
        let bytes = body.single().expect("NoPayload should yield empty Bytes");
        assert!(bytes.is_empty());
    }

    #[test]
    fn no_payload_into_items_yields_empty_vec() {
        let items: Vec<serde_json::Value> = ResponseBody::NoPayload.into_items().unwrap();
        assert!(items.is_empty());
    }

    #[test]
    fn transcode_items_to_binary_prefixes_every_item() {
        let body = ResponseBody::from_items(vec![
            Bytes::from_static(br#"{"id":"1"}"#),
            Bytes::from_static(br#"{"id":"2"}"#),
        ]);
        let ResponseBody::Items(items) = body.transcode_to_binary().unwrap() else {
            panic!("expected item body");
        };

        assert_eq!(items.len(), 2);
        assert!(items.iter().all(|item| crate::binary_json::is_binary(item)));
        let decoded: Vec<serde_json::Value> = ResponseBody::from_items(items).into_items().unwrap();
        assert_eq!(
            decoded,
            vec![
                serde_json::json!({"id": "1"}),
                serde_json::json!({"id": "2"})
            ]
        );
    }

    #[test]
    fn no_payload_into_item_errors() {
        // No bytes to deserialize.
        let body = ResponseBody::NoPayload;
        let result: crate::error::Result<serde_json::Value> = body.into_single();
        assert!(result.is_err());
    }

    #[test]
    fn from_empty_bytes_becomes_no_payload() {
        let body: ResponseBody = Bytes::new().into();
        assert!(matches!(body, ResponseBody::NoPayload));
    }

    #[test]
    fn from_empty_vec_u8_becomes_no_payload() {
        let body: ResponseBody = Vec::<u8>::new().into();
        assert!(matches!(body, ResponseBody::NoPayload));
    }

    #[test]
    fn from_bytes_roundtrip() {
        let body: ResponseBody = Bytes::from_static(b"hello").into();
        match &body {
            ResponseBody::Bytes(b) => assert_eq!(&b[..], b"hello"),
            _ => panic!("expected Bytes variant"),
        }
        let bytes = body.single().expect("single");
        assert_eq!(&bytes[..], b"hello");
    }

    #[test]
    fn items_roundtrip() {
        let body = ResponseBody::from_items(vec![
            Bytes::from_static(b"a"),
            Bytes::from_static(b"bc"),
            Bytes::from_static(b"def"),
        ]);
        assert!(!body.is_empty());
        let items: Vec<Bytes> = match body {
            ResponseBody::Items(items) => items,
            _ => panic!("expected Items variant"),
        };
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn single_errors_on_items() {
        let body =
            ResponseBody::from_items(vec![Bytes::from_static(b"a"), Bytes::from_static(b"b")]);
        assert!(body.single().is_err());
    }

    #[test]
    fn into_item_deserializes() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct Foo {
            id: u32,
        }
        let body = ResponseBody::Bytes(Bytes::from_static(br#"{"id":7}"#));
        let foo: Foo = body.into_single().unwrap();
        assert_eq!(foo, Foo { id: 7 });
    }

    #[test]
    fn into_items_from_items_variant() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct Foo {
            id: u32,
        }
        let body = ResponseBody::from_items(vec![
            Bytes::from_static(br#"{"id":1}"#),
            Bytes::from_static(br#"{"id":2}"#),
        ]);
        let items: Vec<Foo> = body.into_items().unwrap();
        assert_eq!(items, vec![Foo { id: 1 }, Foo { id: 2 }]);
    }

    #[test]
    fn into_items_from_bytes_variant_yields_one() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct Foo {
            id: u32,
        }
        let body = ResponseBody::Bytes(Bytes::from_static(br#"{"id":42}"#));
        let items: Vec<Foo> = body.into_items().unwrap();
        assert_eq!(items, vec![Foo { id: 42 }]);
    }

    #[test]
    fn from_vec_u8_via_into() {
        let body: ResponseBody = vec![1u8, 2, 3].into();
        match &body {
            ResponseBody::Bytes(b) => assert_eq!(&b[..], &[1u8, 2, 3]),
            _ => panic!("expected Bytes variant"),
        }
    }

    #[test]
    fn from_bytes_constructor() {
        let body = ResponseBody::from_bytes(Bytes::from_static(b"abc"));
        match &body {
            ResponseBody::Bytes(b) => assert_eq!(&b[..], b"abc"),
            _ => panic!("expected Bytes variant"),
        }
    }

    #[test]
    fn from_items_constructor() {
        let body = ResponseBody::from_items(vec![Bytes::from_static(b"a")]);
        match &body {
            ResponseBody::Items(v) => assert_eq!(v.len(), 1),
            _ => panic!("expected Items variant"),
        }
    }

    /// A query page arrives as pre-split `Items`, so honoring
    /// `request_text_response` for a query means transcoding every item, not
    /// just a single body. Mixed input also proves the per-item binary sniff:
    /// an already-text item must survive untouched rather than being re-parsed.
    #[test]
    fn transcode_to_text_converts_every_item_of_a_query_page() {
        let binary_a = crate::binary_json::transcode_to_binary(br#"{"id":"a","n":1}"#).unwrap();
        let binary_b = crate::binary_json::transcode_to_binary(br#"{"id":"b","n":2}"#).unwrap();
        let already_text = Bytes::from_static(br#"{"id":"c"}"#);

        let body = ResponseBody::from_items(vec![
            Bytes::from(binary_a),
            Bytes::from(binary_b),
            already_text.clone(),
        ]);
        let text = body.transcode_to_text().unwrap();

        match &text {
            ResponseBody::Items(items) => {
                assert_eq!(items.len(), 3);
                for item in items {
                    assert!(
                        !crate::binary_json::is_binary(item),
                        "every item must be text after transcoding",
                    );
                }
                assert_eq!(&items[0][..], br#"{"id":"a","n":1}"#);
                assert_eq!(&items[1][..], br#"{"id":"b","n":2}"#);
                assert_eq!(&items[2][..], &already_text[..]);
            }
            _ => panic!("expected Items variant"),
        }
    }

    #[test]
    fn is_empty_true_for_empty_items_vec() {
        assert!(ResponseBody::from_items(Vec::new()).is_empty());
    }

    #[test]
    fn is_empty_false_for_items_with_entry() {
        let body = ResponseBody::from_items(vec![Bytes::new()]);
        assert!(!body.is_empty());
    }

    #[test]
    fn is_empty_true_for_no_payload() {
        assert!(ResponseBody::NoPayload.is_empty());
    }

    #[test]
    fn is_empty_false_for_non_empty_bytes() {
        assert!(!ResponseBody::Bytes(Bytes::from_static(b"x")).is_empty());
    }

    // ── Binary JSON auto-detection (P1e) ────────────────────────────────────
    //
    // The encoder is a later phase, so these vectors are hand-encoded. Each
    // buffer starts with the `0x80` preamble; `into_single` / `into_items`
    // detect it and route through the binary-JSON decoder.

    use crate::binary_json::{markers, PREAMBLE};

    /// Wraps already-encoded value bytes in a complete binary buffer (preamble
    /// prefix), returning shared [`Bytes`].
    fn binary(value_bytes: &[u8]) -> Bytes {
        let mut buf = vec![PREAMBLE];
        buf.extend_from_slice(value_bytes);
        Bytes::from(buf)
    }

    /// Encodes a `{"id": n}` object: `OBJ1` (single property), the 1-byte
    /// system string for `id` (index 12), then the literal-int value `n`
    /// (`n` must be < 32 to use the literal-int form).
    fn id_object(n: u8) -> Vec<u8> {
        assert!(n < 32, "literal-int form requires n < 32");
        vec![markers::OBJ1, markers::SYSTEM_STRING_1BYTE_MIN + 12, n]
    }

    /// Encodes an encoded-length string value (the marker carries the length).
    fn enc_str(s: &str) -> Vec<u8> {
        let mut v = vec![markers::ENCODED_STRING_LENGTH_MIN | (s.len() as u8)];
        v.extend_from_slice(s.as_bytes());
        v
    }

    #[derive(serde::Deserialize, PartialEq, Debug)]
    struct Item {
        id: u32,
    }

    #[test]
    fn into_single_decodes_binary_object() {
        // Binary `{"id": 7}` decodes through the typed point-read path.
        let body = ResponseBody::Bytes(binary(&id_object(7)));
        let item: Item = body.into_single().unwrap();
        assert_eq!(item, Item { id: 7 });
    }

    #[test]
    fn into_single_reads_raw_value_from_binary_body() {
        use serde_json::value::RawValue;

        // Regression for issue #5328: a `RawValue` caller (raw passthrough) hit
        // the binary body straight through `into_single`, which failed with a
        // misclassified serialization error once binary encoding became the
        // default. It must now succeed, matching the text path.
        let text = ResponseBody::Bytes(Bytes::from_static(br#"{"id":7}"#));
        let text_raw: Box<RawValue> = text.into_single().unwrap();

        let binary_body = ResponseBody::Bytes(binary(&id_object(7)));
        let binary_raw: Box<RawValue> = binary_body
            .into_single()
            .expect("RawValue must deserialize from a binary body");

        assert_eq!(binary_raw.get(), text_raw.get());
    }

    #[test]
    fn into_items_reads_raw_value_from_binary_items() {
        use serde_json::value::RawValue;

        let body = ResponseBody::from_items(vec![binary(&id_object(1)), binary(&id_object(2))]);
        let items: Vec<Box<RawValue>> = body
            .into_items()
            .expect("RawValue items must deserialize from binary slices");
        let rendered: Vec<&str> = items.iter().map(|r| r.get()).collect();
        assert_eq!(rendered, vec![r#"{"id":1}"#, r#"{"id":2}"#]);
    }

    #[test]
    fn into_items_decodes_binary_bytes_variant() {
        let body = ResponseBody::Bytes(binary(&id_object(9)));
        let items: Vec<Item> = body.into_items().unwrap();
        assert_eq!(items, vec![Item { id: 9 }]);
    }

    #[test]
    fn into_items_decodes_binary_items_variant() {
        let body = ResponseBody::from_items(vec![binary(&id_object(1)), binary(&id_object(2))]);
        let items: Vec<Item> = body.into_items().unwrap();
        assert_eq!(items, vec![Item { id: 1 }, Item { id: 2 }]);
    }

    #[test]
    fn into_single_decodes_binary_feed_envelope() {
        // Proves the query path: the whole `{"Documents":[…],"_count":N}`
        // envelope is decoded from binary and deserialized into a typed feed
        // body in one pass (as the SDK does via `into_single::<FeedBody<T>>`).
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct Feed {
            #[serde(rename = "Documents")]
            documents: Vec<Item>,
            #[serde(rename = "_count")]
            count: u32,
        }

        // Documents: ARR1 wrapping a single `{"id": 1}`.
        let mut documents = vec![markers::ARR1];
        documents.extend_from_slice(&id_object(1));

        // OBJ_L1 envelope with two members.
        let mut payload = Vec::new();
        payload.extend_from_slice(&enc_str("Documents"));
        payload.extend_from_slice(&documents);
        payload.extend_from_slice(&enc_str("_count"));
        payload.push(0x01); // literal int 1
        let mut envelope = vec![markers::OBJ_L1, payload.len() as u8];
        envelope.extend_from_slice(&payload);

        let body = ResponseBody::Bytes(binary(&envelope));
        let feed: Feed = body.into_single().unwrap();
        assert_eq!(
            feed,
            Feed {
                documents: vec![Item { id: 1 }],
                count: 1,
            }
        );
    }

    #[test]
    fn text_json_still_deserializes_unchanged() {
        // A text buffer never begins with `0x80`, so it takes the unchanged
        // text path even with auto-detection in place.
        let body = ResponseBody::Bytes(Bytes::from_static(br#"{"id":5}"#));
        let item: Item = body.into_single().unwrap();
        assert_eq!(item, Item { id: 5 });
    }

    #[test]
    fn malformed_binary_body_errors() {
        // A lone preamble is a truncated binary buffer; decoding fails and the
        // error surfaces as a response-body deserialization error.
        let body = ResponseBody::Bytes(Bytes::from_static(&[PREAMBLE]));
        let result: crate::error::Result<Item> = body.into_single();
        assert!(result.is_err());
    }

    // ── Binary → text transcoding ───────────────────────────────────────────

    #[test]
    fn transcode_bytes_binary_to_text() {
        // A binary `{"id": 7}` body transcodes to text bytes (no `0x80`) that
        // deserialize to the same value.
        let body = ResponseBody::Bytes(binary(&id_object(7)));
        let text = body.transcode_to_text().unwrap();
        match &text {
            ResponseBody::Bytes(b) => {
                assert!(!crate::binary_json::is_binary(b), "must be text now");
                let item: Item = serde_json::from_slice(b).unwrap();
                assert_eq!(item, Item { id: 7 });
            }
            _ => panic!("expected Bytes variant"),
        }
    }

    #[test]
    fn transcode_items_binary_to_text() {
        let body = ResponseBody::from_items(vec![binary(&id_object(1)), binary(&id_object(2))]);
        let text = body.transcode_to_text().unwrap();
        match &text {
            ResponseBody::Items(items) => {
                assert_eq!(items.len(), 2);
                for b in items {
                    assert!(!crate::binary_json::is_binary(b));
                }
            }
            _ => panic!("expected Items variant"),
        }
    }

    #[test]
    fn transcode_text_body_unchanged() {
        // Text passes through byte-for-byte.
        let body = ResponseBody::Bytes(Bytes::from_static(br#"{"id":5}"#));
        let text = body.transcode_to_text().unwrap();
        match &text {
            ResponseBody::Bytes(b) => assert_eq!(&b[..], br#"{"id":5}"#),
            _ => panic!("expected Bytes variant"),
        }
    }

    #[test]
    fn transcode_no_payload_is_noop() {
        let body = ResponseBody::NoPayload;
        assert!(matches!(
            body.transcode_to_text().unwrap(),
            ResponseBody::NoPayload
        ));
    }

    #[test]
    fn transcode_malformed_binary_errors() {
        let body = ResponseBody::Bytes(Bytes::from_static(&[PREAMBLE]));
        assert!(body.transcode_to_text().is_err());
    }
}

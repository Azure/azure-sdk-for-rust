// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Response body container for Cosmos DB operations.
//!
//! Provides [`ResponseBody`], a typed response body that distinguishes between
//! single-payload responses (point reads/writes, batches) and feed-style
//! responses (Query / ChangeFeed) that carry one element per document.

use azure_core::{error::ErrorKind, fmt::SafeDebug, Bytes};
use serde::de::DeserializeOwned;

/// The body of a [`CosmosResponse`](super::CosmosResponse).
///
/// Explicitly distinguishes between the two response shapes the driver returns:
///
/// * [`ResponseBody::Bytes`] — a single payload buffer. Used for point reads,
///   writes, batches, and any other operation that returns one document or
///   envelope.
/// * [`ResponseBody::Items`] — a list of pre-sliced per-document buffers. Used
///   for feed responses (Query / ChangeFeed) where the driver pipeline splits
///   the `Documents` array once via zero-copy [`Bytes::slice`](bytes::Bytes::slice)
///   so the SDK never needs to re-parse the envelope.
///
/// Each variant carries shared ownership via reference-counted
/// [`bytes::Bytes`].
#[derive(Clone, SafeDebug)]
pub enum ResponseBody {
    /// A single response payload (point read/write, batch, metadata, etc.).
    Bytes(Bytes),

    /// A list of per-document slices produced by the feed/query pipeline.
    Items(Vec<Bytes>),
}

impl Default for ResponseBody {
    fn default() -> Self {
        Self::Bytes(Bytes::new())
    }
}

impl ResponseBody {
    /// Creates an empty body (a single empty `Bytes` payload).
    pub fn empty() -> Self {
        Self::Bytes(Bytes::new())
    }

    /// Builds a single-payload [`Bytes`](Self::Bytes) body.
    ///
    /// Use this for point reads/writes, batches, and any other operation that
    /// returns a single document or envelope.
    pub fn from_bytes(bytes: impl Into<Bytes>) -> Self {
        Self::Bytes(bytes.into())
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

    /// Returns `true` if the body carries no response payload from the
    /// service.
    ///
    /// * [`Bytes`](Self::Bytes) is empty when the single buffer is empty.
    /// * [`Items`](Self::Items) is empty when the feed contains zero
    ///   documents. Note: individual document slices are not inspected; a
    ///   feed of one or more (possibly empty) items is *not* empty.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Bytes(b) => b.is_empty(),
            Self::Items(items) => items.is_empty(),
        }
    }

    /// Returns the single payload, or an error if the body is a feed
    /// [`Items`](Self::Items) response.
    ///
    /// Used by single-document response paths (point reads/writes, batch, etc.).
    pub fn single(self) -> azure_core::Result<Bytes> {
        match self {
            Self::Bytes(b) => Ok(b),
            Self::Items(items) => Err(azure_core::Error::with_message(
                ErrorKind::DataConversion,
                format!(
                    "expected single response body, found feed response with {} item(s)",
                    items.len()
                ),
            )),
        }
    }

    /// Deserializes a single-payload body as JSON of type `T`.
    ///
    /// Returns an error if the body is a feed [`Items`](Self::Items) response.
    pub fn single_item<T: DeserializeOwned>(self) -> azure_core::Result<T> {
        let bytes = self.single()?;
        serde_json::from_slice(&bytes).map_err(azure_core::Error::from)
    }

    /// Explicit alias for [`single_item`](Self::single_item). Errors on feed
    /// [`Items`](Self::Items) responses; use [`into_items`](Self::into_items) for those.
    pub fn json_single<T: DeserializeOwned>(self) -> azure_core::Result<T> {
        self.single_item()
    }

    /// Deserializes every item in a feed response, or the single payload, as
    /// JSON of type `T`.
    pub fn into_items<T: DeserializeOwned>(self) -> azure_core::Result<Vec<T>> {
        match self {
            Self::Bytes(b) => {
                let item = serde_json::from_slice(&b).map_err(azure_core::Error::from)?;
                Ok(vec![item])
            }
            Self::Items(items) => items
                .into_iter()
                .map(|b| serde_json::from_slice(&b).map_err(azure_core::Error::from))
                .collect(),
        }
    }

    /// Decodes a single-payload body as a UTF-8 string.
    ///
    /// Returns an error if the body is a feed [`Items`](Self::Items) response.
    pub fn into_string(self) -> azure_core::Result<String> {
        let bytes = self.single()?;
        String::from_utf8(bytes.to_vec()).map_err(|e| {
            azure_core::Error::with_message(
                ErrorKind::DataConversion,
                format!("response body was not valid UTF-8: {e}"),
            )
        })
    }
}

impl From<Bytes> for ResponseBody {
    fn from(bytes: Bytes) -> Self {
        Self::Bytes(bytes)
    }
}

impl From<Vec<u8>> for ResponseBody {
    fn from(bytes: Vec<u8>) -> Self {
        Self::Bytes(Bytes::from(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_empty_bytes() {
        let body = ResponseBody::default();
        assert!(body.is_empty());
        assert_eq!(body.single().unwrap(), Bytes::new());
    }

    #[test]
    fn empty_constructor() {
        let body = ResponseBody::empty();
        assert!(body.is_empty());
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
        let body = ResponseBody::Items(vec![
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
        let body = ResponseBody::Items(vec![Bytes::from_static(b"a"), Bytes::from_static(b"b")]);
        assert!(body.single().is_err());
    }

    #[test]
    fn single_item_deserializes() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct Foo {
            id: u32,
        }
        let body = ResponseBody::Bytes(Bytes::from_static(br#"{"id":7}"#));
        let foo: Foo = body.single_item().unwrap();
        assert_eq!(foo, Foo { id: 7 });
    }

    #[test]
    fn into_items_from_items_variant() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct Foo {
            id: u32,
        }
        let body = ResponseBody::Items(vec![
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

    #[test]
    fn is_empty_for_empty_items_vec() {
        assert!(ResponseBody::from_items(Vec::new()).is_empty());
    }

    #[test]
    fn is_empty_false_for_items_with_any_entry() {
        // Even an empty payload counts as "the feed contained a document".
        let body = ResponseBody::from_items(vec![Bytes::new()]);
        assert!(!body.is_empty());
    }
}

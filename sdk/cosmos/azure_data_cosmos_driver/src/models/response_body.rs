// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Response body container for Cosmos DB operations.
//!
//! Provides [`ResponseBody`], a typed response body that distinguishes between
//! single-payload responses (point reads/writes, batches) and feed-style
//! responses (Query / ChangeFeed) that carry one element per document.

use azure_core::{fmt::SafeDebug, Bytes};
use serde::de::DeserializeOwned;

/// The body of a [`CosmosResponse`](super::CosmosResponse).
///
/// Explicitly distinguishes between the three response shapes the driver
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

    /// Returns `true` if the body carries no readable content.
    ///
    /// * [`NoPayload`](Self::NoPayload) is always empty.
    /// * [`Bytes`](Self::Bytes) is empty when the single buffer has zero bytes.
    /// * [`Items`](Self::Items) is empty when the feed envelope contains zero
    ///   documents.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::NoPayload => true,
            Self::Bytes(b) => b.is_empty(),
            Self::Items(items) => items.is_empty(),
        }
    }

    /// Returns the single payload, or an error if the body is a feed
    /// [`Items`](Self::Items) response. A [`NoPayload`](Self::NoPayload) body
    /// yields an empty [`Bytes`].
    ///
    /// Used by single-document response paths (point reads/writes, batch, etc.).
    pub fn single(self) -> crate::error::Result<Bytes> {
        match self {
            Self::NoPayload => Ok(Bytes::new()),
            Self::Bytes(b) => Ok(b),
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

    /// Returns the per-item raw buffers of a feed response, or wraps a
    /// single-payload body as a one-element vector. A
    /// [`NoPayload`](Self::NoPayload) body yields an empty `Vec`.
    ///
    /// This is the raw-bytes counterpart to
    /// [`into_items`](Self::into_items); use it when callers want to decode
    /// each item themselves instead of going through JSON.
    pub fn items(self) -> crate::error::Result<Vec<Bytes>> {
        match self {
            Self::NoPayload => Ok(Vec::new()),
            Self::Bytes(b) => Ok(vec![b]),
            Self::Items(items) => Ok(items),
        }
    }

    /// Deserializes a single-payload body as JSON of type `T`.
    ///
    /// Returns an error if the body is a feed [`Items`](Self::Items) response
    /// or if the body is [`NoPayload`](Self::NoPayload) (nothing to parse).
    pub fn into_single<T: DeserializeOwned>(self) -> crate::error::Result<T> {
        let bytes = self.single()?;
        serde_json::from_slice(&bytes).map_err(|e| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("failed to deserialize response body")
                .with_source(e)
                .build()
        })
    }

    /// Deserializes every item in a feed response, or the single payload, as
    /// JSON of type `T`. A [`NoPayload`](Self::NoPayload) body yields an empty
    /// `Vec`.
    pub fn into_items<T: DeserializeOwned>(self) -> crate::error::Result<Vec<T>> {
        match self {
            Self::NoPayload => Ok(Vec::new()),
            Self::Bytes(b) => {
                let item = serde_json::from_slice(&b).map_err(|e| {
                    crate::error::CosmosError::builder()
                        .with_status(
                            crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID,
                        )
                        .with_message("failed to deserialize response body")
                        .with_source(e)
                        .build()
                })?;
                Ok(vec![item])
            }
            Self::Items(items) => items
                .into_iter()
                .map(|b| {
                    serde_json::from_slice(&b).map_err(|e| {
                        crate::error::CosmosError::builder()
                            .with_status(
                                crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID,
                            )
                            .with_message("failed to deserialize feed item")
                            .with_source(e)
                            .build()
                    })
                })
                .collect(),
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
}

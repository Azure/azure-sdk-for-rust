// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! SDK-owned wrapper around the driver's response body type.

use azure_core::{fmt::SafeDebug, Bytes};
use azure_data_cosmos_driver::models::ResponseBody as DriverResponseBody;
use serde::de::DeserializeOwned;

use crate::feed::FeedBody;

/// The body of a Cosmos DB operation response.
///
/// Returned by [`ItemResponse::into_body`](crate::models::ItemResponse::into_body),
/// [`ResourceResponse::into_body`](crate::models::ResourceResponse::into_body), and
/// [`BatchResponse::into_body`](crate::models::BatchResponse::into_body). Internally
/// the body may be a single payload (point reads/writes, batches) or a list of
/// per-document slices (feed responses); use the helpers below to consume it.
#[derive(Clone, Default, SafeDebug)]
#[non_exhaustive]
pub struct ResponseBody(DriverResponseBody);

impl ResponseBody {
    /// Returns `true` if the body carries no readable content.
    ///
    /// True for the no-payload response shape, for a single-payload body of
    /// zero bytes, and for a feed envelope with zero items.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the single payload, or an error if the body is a feed response.
    pub fn single(self) -> crate::Result<Bytes> {
        self.0.single().map_err(Into::into)
    }

    /// Returns the per-item raw buffers of a feed response, or wraps a
    /// single-payload body as a one-element vector. A no-payload body yields
    /// an empty `Vec`.
    pub fn items(self) -> crate::Result<Vec<Bytes>> {
        self.0.items().map_err(Into::into)
    }

    /// Deserializes a single-payload body as JSON of type `T`.
    pub fn into_single<T: DeserializeOwned>(self) -> crate::Result<T> {
        self.0.into_single().map_err(Into::into)
    }

    /// Deserializes the items of a feed response as JSON of type `T`.
    ///
    /// * An [`Items`](DriverResponseBody::Items) body — pre-split by the
    ///   cross-partition pipeline (skip/take, streaming ORDER BY merge) — is
    ///   decoded slice-by-slice.
    /// * A single-partition page that never went through those nodes arrives as
    ///   a raw `{"Documents":[...]}` envelope in a single
    ///   [`Bytes`](DriverResponseBody::Bytes) payload, parsed here via
    ///   [`FeedBody`].
    /// * A [`NoPayload`](DriverResponseBody::NoPayload) body yields an empty `Vec`.
    pub(crate) fn into_items<T: DeserializeOwned>(self) -> crate::Result<Vec<T>> {
        match self.0 {
            bytes @ DriverResponseBody::Bytes(_) => {
                let body: FeedBody<T> = bytes.into_single()?;
                Ok(body.items)
            }
            pre_split => pre_split.into_items().map_err(Into::into),
        }
    }
}

impl From<DriverResponseBody> for ResponseBody {
    fn from(inner: DriverResponseBody) -> Self {
        Self(inner)
    }
}

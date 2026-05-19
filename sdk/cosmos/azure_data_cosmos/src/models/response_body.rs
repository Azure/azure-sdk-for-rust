// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! SDK-owned wrapper around the driver's response body type.

use azure_core::{fmt::SafeDebug, Bytes};
use azure_data_cosmos_driver::models::ResponseBody as DriverResponseBody;
use serde::de::DeserializeOwned;

/// The body of a Cosmos DB operation response.
///
/// Returned by [`ItemResponse::into_body`](crate::ItemResponse::into_body),
/// [`ResourceResponse::into_body`](crate::ResourceResponse::into_body), and
/// [`BatchResponse::into_body`](crate::BatchResponse::into_body). Internally
/// the body may be a single payload (point reads/writes, batches) or a list of
/// per-document slices (feed responses); use the helpers below to consume it.
#[derive(Clone, Default, SafeDebug)]
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
    pub fn single(self) -> azure_core::Result<Bytes> {
        self.0.single()
    }

    /// Returns the per-item raw buffers of a feed response, or wraps a
    /// single-payload body as a one-element vector. A no-payload body yields
    /// an empty `Vec`.
    pub fn items(self) -> azure_core::Result<Vec<Bytes>> {
        self.0.items()
    }

    /// Deserializes a single-payload body as JSON of type `T`.
    pub fn into_single<T: DeserializeOwned>(self) -> azure_core::Result<T> {
        self.0.into_single()
    }

    /// Deserializes every item in a feed response, or the single payload, as
    /// JSON of type `T`.
    pub fn into_items<T: DeserializeOwned>(self) -> azure_core::Result<Vec<T>> {
        self.0.into_items()
    }
}

impl From<DriverResponseBody> for ResponseBody {
    fn from(inner: DriverResponseBody) -> Self {
        Self(inner)
    }
}

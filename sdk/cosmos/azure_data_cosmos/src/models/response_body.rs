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
    /// Returns `true` if the body carries no bytes.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the single payload, or an error if the body is a feed response.
    pub fn single(self) -> azure_core::Result<Bytes> {
        self.0.single()
    }

    /// Deserializes a single-payload body as JSON of type `T`.
    pub fn single_item<T: DeserializeOwned>(self) -> azure_core::Result<T> {
        self.0.single_item()
    }

    /// Explicit alias for [`single_item`](Self::single_item). Errors on feed responses; use [`into_items`](Self::into_items) for those.
    pub fn json_single<T: DeserializeOwned>(self) -> azure_core::Result<T> {
        self.0.json_single()
    }

    /// Deserializes every item in a feed response, or the single payload, as
    /// JSON of type `T`.
    pub fn into_items<T: DeserializeOwned>(self) -> azure_core::Result<Vec<T>> {
        self.0.into_items()
    }

    /// Decodes a single-payload body as a UTF-8 string. Returns an error if
    /// the body is a feed response.
    pub fn into_string(self) -> azure_core::Result<String> {
        self.0.into_string()
    }
}

impl From<DriverResponseBody> for ResponseBody {
    fn from(inner: DriverResponseBody) -> Self {
        Self(inner)
    }
}

impl From<ResponseBody> for DriverResponseBody {
    fn from(body: ResponseBody) -> Self {
        body.0
    }
}

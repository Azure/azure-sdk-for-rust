// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! SDK-owned wrapper around the driver's `CosmosResponseHeaders`.

use azure_data_cosmos_driver::models::{
    ActivityId, CosmosResponseHeaders as DriverCosmosResponseHeaders, ETag, RequestCharge,
    SessionToken, SubStatusCode,
};

/// Cosmos DB response headers parsed from the wire.
///
/// This is the SDK-owned view of the per-response headers. Callers reach the
/// individual values through dedicated accessor methods (`etag()`,
/// `request_charge()`, `session_token()`, …) instead of touching the driver
/// header struct directly, so the driver remains free to evolve its internal
/// header representation without breaking SDK consumers.
///
/// Conversions to/from the driver type are provided via `From` for the few
/// internal sites (tests, diagnostics bridges) that need full access; they are
/// not required for normal SDK usage.
#[derive(Clone, Debug, Default)]
pub struct ResponseHeaders(DriverCosmosResponseHeaders);

impl ResponseHeaders {
    /// ETag for optimistic concurrency (`etag`).
    pub fn etag(&self) -> Option<&ETag> {
        self.0.etag.as_ref()
    }

    /// Request charge in Request Units (`x-ms-request-charge`).
    pub fn request_charge(&self) -> Option<&RequestCharge> {
        self.0.request_charge.as_ref()
    }

    /// Session token for session consistency (`x-ms-session-token`).
    pub fn session_token(&self) -> Option<&SessionToken> {
        self.0.session_token.as_ref()
    }

    /// Continuation token for pagination (`x-ms-continuation`).
    pub fn continuation(&self) -> Option<&str> {
        self.0.continuation.as_deref()
    }

    /// Activity ID for request correlation (`x-ms-activity-id`).
    pub fn activity_id(&self) -> Option<&ActivityId> {
        self.0.activity_id.as_ref()
    }

    /// Cosmos substatus code (`x-ms-substatus`).
    pub fn substatus(&self) -> Option<&SubStatusCode> {
        self.0.substatus.as_ref()
    }

    /// Decoded JSON for index utilization metrics (`x-ms-cosmos-index-utilization`).
    pub fn index_metrics(&self) -> Option<&str> {
        self.0.index_metrics.as_deref()
    }

    /// Raw query execution metrics (`x-ms-documentdb-query-metrics`).
    pub fn query_metrics(&self) -> Option<&str> {
        self.0.query_metrics.as_deref()
    }

    /// Whether an offer replace is still pending (`x-ms-offer-replace-pending`).
    pub fn offer_replace_pending(&self) -> Option<bool> {
        self.0.offer_replace_pending
    }

    /// Server-side request processing time in milliseconds
    /// (`x-ms-request-duration-ms`). Non-finite / negative values are filtered
    /// to `None` during parsing.
    pub fn server_duration_ms(&self) -> Option<f64> {
        self.0.server_duration_ms
    }

    /// Logical sequence number of the partition replica that served this request
    /// (`lsn`). Advances with every write on the partition.
    pub fn lsn(&self) -> Option<u64> {
        self.0.lsn
    }

    /// Logical sequence number of the specific item operated on (`x-ms-item-lsn`).
    pub fn item_lsn(&self) -> Option<u64> {
        self.0.item_lsn
    }

    /// Item count in response (`x-ms-item-count`).
    pub fn item_count(&self) -> Option<u32> {
        self.0.item_count
    }
}

impl From<DriverCosmosResponseHeaders> for ResponseHeaders {
    fn from(inner: DriverCosmosResponseHeaders) -> Self {
        Self(inner)
    }
}

impl From<ResponseHeaders> for DriverCosmosResponseHeaders {
    fn from(h: ResponseHeaders) -> Self {
        h.0
    }
}

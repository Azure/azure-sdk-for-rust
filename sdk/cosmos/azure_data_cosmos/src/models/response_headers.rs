// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore activityid llsn gatewayversion

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
/// Note that the *value* types returned by the accessors
/// (`ETag`, `RequestCharge`, `SessionToken`, `SubStatusCode`, `ActivityId`)
/// are intentionally re-exported from the driver as canonical Cosmos types.
/// They are stable, narrow primitives with no SDK-specific behavior, so the
/// SDK does not maintain parallel wrappers for them.
///
/// Construction from the driver type is `From`-based for the bridge layer; the
/// reverse direction is intentionally `pub(crate)`-scoped (the crate-internal
/// `into_driver_headers` helper) so the driver representation is not part of
/// the SDK's public surface.
#[derive(Clone, Debug, Default)]
pub struct ResponseHeaders(DriverCosmosResponseHeaders);

impl ResponseHeaders {
    /// Clones the supplied driver-owned `CosmosResponseHeaders` into a
    /// fresh `ResponseHeaders` wrapper.
    ///
    /// Constructs the SDK [`ResponseHeaders`] wrapper from the driver's
    /// canonical [`CosmosResponseHeaders`](DriverCosmosResponseHeaders).
    /// The driver type is already part of the public surface (re-exported
    /// from `crate::models`); this is the no-cost bridge for code that
    /// already has a driver headers value in hand (e.g. via
    /// [`CosmosError::response`](crate::error::CosmosError::response) →
    /// `CosmosResponse::headers`).
    ///
    /// Cosmos response headers are a small bag of `Option<…>` primitives,
    /// so the clone is a handful of `Option<String>` deep copies — cheap
    /// relative to constructing the originating error or response.
    pub fn from_driver(driver: &DriverCosmosResponseHeaders) -> Self {
        Self(driver.clone())
    }

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

    /// Server-suggested retry delay in milliseconds on throttling
    /// (`x-ms-retry-after-ms`).
    pub fn retry_after_ms(&self) -> Option<u64> {
        self.0.retry_after_ms
    }

    /// Caller-supplied correlation ID echoed back by the server
    /// (`x-ms-cosmos-correlated-activityid`).
    pub fn correlated_activity_id(&self) -> Option<&str> {
        self.0.correlated_activity_id.as_deref()
    }

    /// Server-side transport request ID (`x-ms-transport-request-id`). Useful
    /// for correlating client-side diagnostics with backend logs.
    pub fn transport_request_id(&self) -> Option<u32> {
        self.0.transport_request_id
    }

    /// Latest globally committed LSN across all regions for the partition
    /// (`x-ms-global-committed-lsn`).
    pub fn global_committed_lsn(&self) -> Option<i64> {
        self.0.global_committed_lsn
    }

    /// Local (per-replica) LSN that served the request (`x-ms-cosmos-llsn`).
    pub fn local_lsn(&self) -> Option<u64> {
        self.0.local_lsn
    }

    /// Local (per-replica) LSN of the specific item operated on
    /// (`x-ms-cosmos-item-llsn`).
    pub fn item_local_lsn(&self) -> Option<u64> {
        self.0.item_local_lsn
    }

    /// Gateway version that served the request (`x-ms-gatewayversion`).
    pub fn gateway_version(&self) -> Option<&str> {
        self.0.gateway_version.as_deref()
    }

    /// Resource-level quota information (`x-ms-resource-quota`). Semicolon-
    /// delimited `key=value` pairs (e.g. `documentSize=…;collections=…`).
    pub fn resource_quota(&self) -> Option<&str> {
        self.0.resource_quota.as_deref()
    }

    /// Resource-level usage information (`x-ms-resource-usage`). Semicolon-
    /// delimited `key=value` pairs, paired with `resource_quota`.
    pub fn resource_usage(&self) -> Option<&str> {
        self.0.resource_usage.as_deref()
    }

    /// Partition key range that served the request
    /// (`x-ms-documentdb-partitionkeyrangeid`).
    pub fn partition_key_range_id(&self) -> Option<&str> {
        self.0.partition_key_range_id.as_deref()
    }

    /// Internal partition ID (`x-ms-cosmos-internal-partition-id`). Backend
    /// routing identifier; primarily useful for diagnostics.
    pub fn internal_partition_id(&self) -> Option<&str> {
        self.0.internal_partition_id.as_deref()
    }

    /// Collection index transformation progress, 0\u2013100
    /// (`x-ms-documentdb-collection-index-transformation-progress`). Reported
    /// while the service is rebuilding the index after a policy change.
    pub fn collection_index_transformation_progress(&self) -> Option<i64> {
        self.0.collection_index_transformation_progress
    }

    /// Collection lazy-indexing progress, 0\u2013100
    /// (`x-ms-documentdb-collection-lazy-indexing-progress`). Reported for
    /// containers using the lazy indexing mode.
    pub fn collection_lazy_indexing_progress(&self) -> Option<i64> {
        self.0.collection_lazy_indexing_progress
    }
}

impl ResponseHeaders {
    /// Test-only escape hatch to recover the underlying driver headers.
    ///
    /// Gated behind the unstable `__internal_in_memory_emulator` feature so
    /// the conversion is not part of the SDK's public API. Used only by the
    /// integration-test validation framework that compares against driver-side
    /// snapshots.
    #[cfg(feature = "__internal_in_memory_emulator")]
    #[doc(hidden)]
    pub fn __into_driver_headers(self) -> DriverCosmosResponseHeaders {
        self.0
    }
}

impl From<DriverCosmosResponseHeaders> for ResponseHeaders {
    fn from(inner: DriverCosmosResponseHeaders) -> Self {
        Self(inner)
    }
}

/// Crate-internal escape hatch to recover the underlying driver headers.
///
/// Used by `feed.rs` (which needs to read several parsed fields at once for
/// page wire-up) and validation tests. Intentionally not `pub` — external
/// callers must go through the typed accessors.
pub(crate) fn into_driver_headers(h: ResponseHeaders) -> DriverCosmosResponseHeaders {
    h.0
}

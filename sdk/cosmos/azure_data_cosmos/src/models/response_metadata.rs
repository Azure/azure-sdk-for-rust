// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation-specific response metadata types.

use azure_data_cosmos_driver::models::CosmosResponseHeaders;

/// Diagnostics for a Cosmos DB operation.
///
/// Provides operation-level metadata for debugging and performance analysis.
///
/// This is a lightweight diagnostics type populated from HTTP response headers.
/// It will be expanded to include full driver diagnostics (retry tracking,
/// regions contacted, pipeline events) once the SDK pipeline is ported to the
/// driver's transport pipeline.
#[derive(Debug, Clone, Default)]
pub struct CosmosDiagnostics {
    activity_id: Option<String>,
    server_duration_ms: Option<f64>,
}

impl CosmosDiagnostics {
    pub(crate) fn from_headers(headers: &CosmosResponseHeaders) -> Self {
        Self {
            activity_id: headers.activity_id.as_ref().map(|a| a.as_str().to_owned()),
            server_duration_ms: headers.server_duration_ms,
        }
    }

    /// Returns the activity ID for request correlation, if available.
    pub fn activity_id(&self) -> Option<&str> {
        self.activity_id.as_deref()
    }

    /// Returns the server-side request processing duration in milliseconds, if available.
    pub fn server_duration_ms(&self) -> Option<f64> {
        self.server_duration_ms
    }
}

/// Metadata specific to point item operations (create, read, replace, upsert, delete).
#[derive(Debug, Clone, Default)]
pub struct ItemMetadata {
    etag: Option<String>,
}

impl ItemMetadata {
    pub(crate) fn from_headers(headers: &CosmosResponseHeaders) -> Self {
        Self {
            etag: headers.etag.as_ref().map(|e| e.as_str().to_owned()),
        }
    }

    /// Returns the ETag for optimistic concurrency control, if available.
    pub fn etag(&self) -> Option<&str> {
        self.etag.as_deref()
    }
}

/// Metadata specific to query operations.
#[derive(Debug, Clone, Default)]
pub struct QueryMetadata {
    index_metrics: Option<String>,
    query_metrics: Option<String>,
}

impl QueryMetadata {
    pub(crate) fn from_headers(headers: &CosmosResponseHeaders) -> Self {
        Self {
            index_metrics: headers.index_metrics.clone(),
            query_metrics: headers.query_metrics.clone(),
        }
    }

    /// Returns the index utilization metrics as a decoded JSON string, if available.
    ///
    /// The service returns this header as a base64-encoded JSON string. This method
    /// returns the decoded JSON. Only populated when the request included the
    /// `x-ms-cosmos-populateindexmetrics` header.
    pub fn index_metrics(&self) -> Option<&str> {
        self.index_metrics.as_deref()
    }

    /// Returns the query execution metrics, if available.
    ///
    /// The value is a semicolon-delimited string of key=value pairs.
    /// Only populated when the request included the `x-ms-documentdb-populatequerymetrics` header.
    pub fn query_metrics(&self) -> Option<&str> {
        self.query_metrics.as_deref()
    }
}

/// Metadata for resource management operations (databases, containers, throughput).
///
/// Currently empty — reserved for future fields without breaking changes.
#[derive(Debug, Clone, Default)]
pub struct ResourceMetadata {}

impl ResourceMetadata {
    pub(crate) fn from_headers(_headers: &CosmosResponseHeaders) -> Self {
        Self {}
    }
}

/// Metadata specific to transactional batch operations.
///
/// Structurally identical to [`ItemMetadata`] but kept separate because batch-level
/// ETags have different semantics than single-item ETags, and the types may diverge
/// in the future.
///
/// Note: The batch-level ETag differs from a single-item ETag. It represents
/// the ETag for the entire batch operation, not an individual item's concurrency token.
/// Use individual `TransactionalBatchOperationResult` entries for per-item ETags.
#[derive(Debug, Clone, Default)]
pub struct BatchMetadata {
    etag: Option<String>,
}

impl BatchMetadata {
    pub(crate) fn from_headers(headers: &CosmosResponseHeaders) -> Self {
        Self {
            etag: headers.etag.as_ref().map(|e| e.as_str().to_owned()),
        }
    }

    /// Returns the batch-level ETag, if available.
    ///
    /// This is the ETag for the entire batch operation, not an individual item's
    /// concurrency token.
    pub fn etag(&self) -> Option<&str> {
        self.etag.as_deref()
    }
}

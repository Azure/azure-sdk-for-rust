// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation-specific response metadata types.

use azure_data_cosmos_driver::models::CosmosResponseHeaders;

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

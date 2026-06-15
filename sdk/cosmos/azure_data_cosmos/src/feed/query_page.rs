// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`QueryFeedPage`] — a single page of query results with query-specific metadata.

use std::sync::Arc;

use azure_data_cosmos_driver::models::CosmosResponseHeaders;
use serde::de::DeserializeOwned;

use crate::{
    diagnostics::DiagnosticsContext,
    feed::page::{FeedBody, FeedPage},
    models::{CosmosResponse, ResponseHeaders},
};

/// Represents a single page of results from a Cosmos DB query.
///
/// Wraps a [`FeedPage`] and adds query-specific metadata such as
/// [`index_metrics()`](Self::index_metrics) and [`query_metrics()`](Self::query_metrics).
///
/// This type is yielded by [`QueryItemIterator`](crate::feed::QueryItemIterator)
/// and [`QueryPageIterator`](crate::feed::QueryPageIterator) for query operations.
#[derive(Debug)]
pub struct QueryFeedPage<T> {
    /// The underlying feed page with common fields.
    page: FeedPage<T>,

    /// Index utilization metrics (decoded from base64 JSON).
    index_metrics: Option<String>,

    /// Query execution metrics (semicolon-delimited key=value pairs).
    query_metrics: Option<String>,
}

impl<T> QueryFeedPage<T> {
    /// Returns a reference to the underlying [`FeedPage`].
    ///
    /// Use this when passing a query page to APIs that accept the more general
    /// [`FeedPage`] type. The query-specific metadata (index metrics, query
    /// metrics) is not visible through the returned reference; access it via
    /// [`index_metrics`](Self::index_metrics) and
    /// [`query_metrics`](Self::query_metrics) on this `QueryFeedPage` instead.
    pub fn as_feed_page(&self) -> &FeedPage<T> {
        &self.page
    }

    /// Gets the items in this page of results.
    pub fn items(&self) -> &[T] {
        self.page.items()
    }

    /// Consumes the page and returns a vector of the items.
    pub fn into_items(self) -> Vec<T> {
        self.page.into_items()
    }

    /// Returns the parsed Cosmos-specific response headers for this page.
    pub fn headers(&self) -> &ResponseHeaders {
        self.page.headers()
    }

    /// Returns the diagnostics for this page.
    ///
    /// The returned [`DiagnosticsContext`] surfaces the full per-operation
    /// diagnostics produced by the driver pipeline (request tracking, retries,
    /// regions contacted, RU charges, status, etc.).
    pub fn diagnostics(&self) -> Arc<DiagnosticsContext> {
        self.page.diagnostics()
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

impl<T: DeserializeOwned> QueryFeedPage<T> {
    pub(crate) fn from_response(response: CosmosResponse) -> crate::Result<Self> {
        // Convert once to the driver header struct: this module owns the
        // FeedPage wire-up and needs every parsed field, so reaching for the
        // SDK wrapper accessors here would be pure ceremony.
        let cosmos_headers: CosmosResponseHeaders =
            crate::models::into_driver_headers(response.cosmos_headers().clone());
        let index_metrics = cosmos_headers.index_metrics.clone();
        let query_metrics = cosmos_headers.query_metrics.clone();
        let diagnostics = response.diagnostics();
        let body: FeedBody<T> = response.into_model()?;

        Ok(Self {
            page: FeedPage::new(
                body.items,
                ResponseHeaders::from(cosmos_headers),
                diagnostics,
            ),
            index_metrics,
            query_metrics,
        })
    }
}

#[cfg(test)]
impl<T> QueryFeedPage<T> {
    /// Test-only constructor used by the iterator unit tests in this crate.
    pub(crate) fn new_for_testing(
        items: Vec<T>,
        headers: ResponseHeaders,
        diagnostics: Arc<DiagnosticsContext>,
    ) -> Self {
        Self {
            page: FeedPage::new(items, headers, diagnostics),
            index_metrics: None,
            query_metrics: None,
        }
    }
}

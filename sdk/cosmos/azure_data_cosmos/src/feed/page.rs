// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`FeedPage`] — a single page of results from a Cosmos DB feed.

use std::sync::Arc;

use serde::Deserialize;

use crate::{diagnostics::DiagnosticsContext, models::ResponseHeaders};

/// Represents a single page of results from a Cosmos DB feed.
///
/// A feed could be a list of items, databases, containers, etc.
/// The feed may represent a single-partition or cross-partition query.
///
/// Cosmos DB queries can be executed using non-HTTP transports, depending on the circumstances.
/// They may also produce results that don't directly correlate to specific HTTP responses (as in the case of cross-partition queries).
/// Because of this, Cosmos DB feed responses use `FeedPage` to represent the results, rather than a more generic type like [`Response`](azure_core::http::Response).
#[derive(Debug)]
pub struct FeedPage<T> {
    /// The items in the response.
    items: Vec<T>,

    /// Parsed Cosmos-specific response headers.
    headers: ResponseHeaders,

    /// Diagnostics for this page.
    diagnostics: Arc<DiagnosticsContext>,
}

impl<T> FeedPage<T> {
    /// Creates a new `FeedPage` instance.
    pub(crate) fn new(
        items: Vec<T>,
        headers: ResponseHeaders,
        diagnostics: Arc<DiagnosticsContext>,
    ) -> Self {
        Self {
            items,
            headers,
            diagnostics,
        }
    }

    /// Gets the items in this page of results.
    pub fn items(&self) -> &[T] {
        &self.items
    }

    /// Consumes the page and returns a vector of the items.
    pub fn into_items(self) -> Vec<T> {
        self.items
    }

    /// Returns the parsed Cosmos-specific response headers for this page.
    pub fn headers(&self) -> &ResponseHeaders {
        &self.headers
    }

    /// Returns the diagnostics for this page.
    ///
    /// The returned [`DiagnosticsContext`] surfaces the full per-operation
    /// diagnostics produced by the driver pipeline (request tracking, retries,
    /// regions contacted, RU charges, status, etc.).
    pub fn diagnostics(&self) -> Arc<DiagnosticsContext> {
        Arc::clone(&self.diagnostics)
    }
}

/// Internal wire-format wrapper for a feed body returned by the service.
#[derive(Deserialize)]
pub(crate) struct FeedBody<T> {
    #[serde(alias = "Documents")]
    #[serde(alias = "DocumentCollections")]
    #[serde(alias = "Databases")]
    #[serde(alias = "Offers")]
    pub(crate) items: Vec<T>,
}

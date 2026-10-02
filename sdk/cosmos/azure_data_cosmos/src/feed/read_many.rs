// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    pin::Pin,
    str::FromStr,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

use azure_core::fmt::SafeDebug;
use azure_data_cosmos_driver::{read_many as driver, RequestCharge};
use futures::{Stream, StreamExt};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    diagnostics::DiagnosticsContext,
    feed::{QueryFeedPage, QueryPageIterator},
    PartitionKey,
};

/// Items or complete logical partitions selected for a read-many operation.
///
/// Empty selections read nothing. Duplicate selections are removed.
#[derive(Clone, SafeDebug)]
#[non_exhaustive]
pub enum ReadManySelection {
    /// Exact `(partition key, item id)` pairs.
    Items(Vec<(PartitionKey, String)>),
    /// Complete logical partition keys, not hierarchical prefixes.
    Partitions(Vec<PartitionKey>),
}

impl ReadManySelection {
    pub(crate) fn into_driver(self) -> driver::ReadManySelection {
        match self {
            Self::Items(items) => driver::ReadManySelection::Items(items),
            Self::Partitions(keys) => driver::ReadManySelection::Partitions(keys),
        }
    }
}

/// A parameterized SQL predicate that narrows a read-many selection.
///
/// Parse a WHERE expression using alias `c`, without the `WHERE` keyword.
/// SELECT statements, subqueries, and user-defined functions are not supported.
/// Use [`crate::clients::ContainerClient::query_items()`] for full queries.
/// Filters support at most 256 lexical tokens. Bind escaped values as parameters
/// or use JSON double-quoted string literals.
///
/// ```
/// use azure_data_cosmos::feed::ReadManyFilter;
/// let filter = "c.active = @active".parse::<ReadManyFilter>()?
///     .with_parameter("@active", true)?;
/// # Ok::<(), azure_data_cosmos::CosmosError>(())
/// ```
#[derive(Clone, SafeDebug)]
pub struct ReadManyFilter(pub(crate) driver::ReadManyFilter);

impl FromStr for ReadManyFilter {
    type Err = crate::CosmosError;
    fn from_str(value: &str) -> crate::Result<Self> {
        value.parse().map(Self)
    }
}

impl ReadManyFilter {
    /// Binds a serializable value to a referenced parameter, including `@`.
    ///
    /// Returns an error for duplicate/unreferenced names or serialization
    /// failures. Bind every parameter before starting read-many.
    pub fn with_parameter(
        self,
        name: impl Into<String>,
        value: impl Serialize,
    ) -> crate::Result<Self> {
        self.0
            .with_parameter(name, serde_json::to_value(value)?)
            .map(Self)
    }
}

/// A finite stream of read-many pages without durable continuation tokens.
///
/// Items are unordered; missing and filtered-out items are omitted.
/// Earlier pages remain valid if a later page fails. There is no cross-partition
/// snapshot guarantee.
#[pin_project::pin_project]
pub struct ReadManyIterator<T: Send> {
    #[pin]
    pages: QueryPageIterator<T>,
    started: bool,
    timeout: Option<Duration>,
}

impl<T: DeserializeOwned + Send + 'static> ReadManyIterator<T> {
    pub(crate) fn new(pages: QueryPageIterator<T>, timeout: Option<Duration>) -> Self {
        Self {
            pages,
            started: false,
            timeout,
        }
    }

    /// Collects all pages into one response with total charge and diagnostics.
    ///
    /// Memory usage grows with all returned items. This consumes the iterator;
    /// call it before polling any pages. The configured operation timeout
    /// bounds the whole collection, rather than each page independently.
    ///
    /// # Errors
    ///
    /// Returns an error if iteration already started, deserialization fails,
    /// or any request fails. Partial items are not returned as success.
    pub async fn collect_all(mut self) -> crate::Result<ReadManyResponse<T>> {
        if self.started {
            return Err(crate::CosmosError::builder()
                .with_status(crate::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("collect_all must be called before polling read-many")
                .build());
        }
        if let Some(timeout) = self.timeout {
            self.pages
                .set_collection_deadline(std::time::Instant::now() + timeout)?;
        }
        let mut items = Vec::new();
        let mut diagnostics: Option<Arc<DiagnosticsContext>> = None;
        while let Some(page) = self.pages.next().await {
            match page {
                Ok(page) => {
                    diagnostics = merge_diagnostics(diagnostics, Some(page.diagnostics()));
                    items.extend(page.into_items());
                }
                Err(error) => {
                    let diagnostics = merge_diagnostics(diagnostics, error.diagnostics());
                    let mut builder =
                        azure_data_cosmos_driver::CosmosErrorBuilder::from_error(error);
                    if let Some(diagnostics) = diagnostics {
                        builder = builder.with_diagnostics(diagnostics);
                    }
                    return Err(builder.build());
                }
            }
        }
        Ok(ReadManyResponse {
            items,
            request_charge: diagnostics
                .as_ref()
                .map(|context| context.total_request_charge())
                .unwrap_or_default(),
            diagnostics,
        })
    }
}

fn merge_diagnostics(
    prior: Option<Arc<DiagnosticsContext>>,
    current: Option<Arc<DiagnosticsContext>>,
) -> Option<Arc<DiagnosticsContext>> {
    match (prior, current) {
        (Some(prior), Some(current)) => {
            DiagnosticsContext::aggregate_sub_operations(&[prior, current]).map(Arc::new)
        }
        (prior, current) => prior.or(current),
    }
}

impl<T: DeserializeOwned + Send + 'static> Stream for ReadManyIterator<T> {
    type Item = crate::Result<QueryFeedPage<T>>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        *this.started = true;
        this.pages.poll_next(cx)
    }
}

/// All collected read-many items and their aggregate operation metadata.
pub struct ReadManyResponse<T> {
    items: Vec<T>,
    request_charge: RequestCharge,
    diagnostics: Option<Arc<DiagnosticsContext>>,
}

// SafeDebug's debug feature formats T without generating a bound; keep items redacted.
impl<T> std::fmt::Debug for ReadManyResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReadManyResponse").finish_non_exhaustive()
    }
}

impl<T> ReadManyResponse<T> {
    /// Returns the matching items in unspecified order.
    pub fn items(&self) -> &[T] {
        &self.items
    }
    /// Consumes the response and returns its items.
    pub fn into_items(self) -> Vec<T> {
        self.items
    }
    /// Returns the total request charge, including requests for missing items.
    pub fn request_charge(&self) -> RequestCharge {
        self.request_charge
    }
    /// Returns aggregate diagnostics, or `None` when no requests were needed.
    pub fn diagnostics(&self) -> Option<Arc<DiagnosticsContext>> {
        self.diagnostics.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{ReadManyResponse, RequestCharge};

    #[test]
    fn response_debug_supports_non_debug_items_without_exposing_contents() {
        struct Item;
        let opaque = ReadManyResponse {
            items: vec![Item],
            request_charge: RequestCharge::default(),
            diagnostics: None,
        };
        assert_eq!(format!("{opaque:?}"), "ReadManyResponse { .. }");

        let sensitive = ReadManyResponse {
            items: vec!["private item content"],
            request_charge: RequestCharge::default(),
            diagnostics: None,
        };
        assert_eq!(format!("{sensitive:?}"), "ReadManyResponse { .. }");
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Query execution implementation.

use std::{collections::HashMap, sync::Arc};

use azure_core::http::headers::{HeaderName, HeaderValue};
use azure_data_cosmos_driver::{
    models::{CosmosOperation, SessionToken},
    options::OperationOptions as DriverOperationOptions,
    CosmosDriver,
};
use serde::de::DeserializeOwned;

use crate::{constants, driver_bridge, options::MaxItemCountHint, Query, QueryFeedPage};

/// Per-execution configuration for [`QueryExecutor`].
///
/// Grouping the knobs here keeps the constructor's argument list short and
/// makes adding future per-query options non-breaking. `Default::default()`
/// is appropriate for metadata queries (databases / containers) that don't
/// expose any of these knobs to the caller.
#[derive(Clone, Default)]
pub(crate) struct QueryExecutorConfig {
    /// Optional initial session-token hint.
    pub session_token: Option<SessionToken>,
    /// Request `x-ms-cosmos-populateindexmetrics` on each page.
    pub populate_index_metrics: Option<bool>,
    /// Request `x-ms-documentdb-populatequerymetrics` on each page.
    pub populate_query_metrics: Option<bool>,
    /// Page size for `x-ms-max-item-count`.
    pub max_item_count: Option<MaxItemCountHint>,
}

/// A query executor that sends queries through the Cosmos driver.
///
/// This executor handles pagination via continuation tokens and works for
/// item queries (with partition key), database queries, and container queries.
/// The `operation_factory` closure produces the appropriate `CosmosOperation`
/// for each page request.
pub struct QueryExecutor<T: DeserializeOwned + Send> {
    driver: Arc<CosmosDriver>,
    operation_factory: Box<dyn Fn() -> CosmosOperation + Send>,
    query: Query,
    query_body: Option<Vec<u8>>,
    base_options: DriverOperationOptions,
    base_headers: HashMap<HeaderName, HeaderValue>,
    config: QueryExecutorConfig,
    continuation: Option<String>,
    complete: bool,
    // Why is our phantom type a function? Because that represents how we _use_ the type T.
    // Normally, PhantomData<T> is only Send/Sync if T is, because PhantomData is indicating that while we don't _name_ T in a field, we should act as though we have a field of type T.
    // However, we don't store any T values in this, we only RETURN them.
    // That means we use a function pointer to indicate that we don't actually operate on T directly, we just return it.
    // Because of this, PhantomData<fn() -> T> is Send/Sync even if T isn't (see https://doc.rust-lang.org/stable/nomicon/phantom-data.html#table-of-phantomdata-patterns)
    phantom: std::marker::PhantomData<fn() -> T>,
}

impl<T: DeserializeOwned + Send + 'static> QueryExecutor<T> {
    pub(crate) fn new(
        driver: Arc<CosmosDriver>,
        operation_factory: impl Fn() -> CosmosOperation + Send + 'static,
        query: Query,
        base_options: DriverOperationOptions,
        config: QueryExecutorConfig,
    ) -> Self {
        // Pre-build the static custom headers (user-provided only). The
        // driver pipeline auto-emits `x-ms-documentdb-isquery: True` and
        // `Content-Type: application/query+json` for every OperationType::Query,
        // so we no longer push them into `custom_headers` here.
        let base_headers = base_options.custom_headers().cloned().unwrap_or_default();

        Self {
            driver,
            operation_factory: Box::new(operation_factory),
            query,
            query_body: None,
            base_options,
            base_headers,
            config,
            continuation: None,
            complete: false,
            phantom: std::marker::PhantomData,
        }
    }

    /// Consumes the executor and converts it into a stream of pages.
    pub fn into_stream(self) -> azure_core::Result<crate::FeedItemIterator<T>> {
        Ok(crate::FeedItemIterator::new(futures::stream::try_unfold(
            self,
            |mut state| async move {
                let val = state.next_page().await?;
                Ok(val.map(|item| (item, state)))
            },
        )))
    }

    /// Fetches the next page of query results.
    ///
    /// Returns `None` if there are no more pages to fetch.
    pub async fn next_page(&mut self) -> azure_core::Result<Option<QueryFeedPage<T>>> {
        if self.complete {
            return Ok(None);
        }

        // Build a fresh operation for this page
        let mut operation = (self.operation_factory)();

        // Serialize the query body on the first page and cache it for subsequent pages.
        if self.query_body.is_none() {
            self.query_body = Some(serde_json::to_vec(&self.query)?);
        }
        operation = operation.with_body(self.query_body.clone().unwrap());

        // The explicit session token serves as an initial hint; the driver's
        // internal session manager captures response tokens and applies them
        // to subsequent requests automatically.
        if let Some(session_token) = &self.config.session_token {
            operation = operation.with_session_token(session_token.clone());
        }

        // Typed per-query request headers, applied additively with consistent
        // precedence: anything the caller has already set on the operation
        // (`is_some()` for `Option` fields) wins, and the executor only fills
        // in fields that are not already set.
        if self.config.populate_index_metrics.is_some()
            || self.config.populate_query_metrics.is_some()
            || self.config.max_item_count.is_some()
        {
            let mut request_headers = operation.request_headers().clone();
            if request_headers.populate_index_metrics.is_none() {
                request_headers.populate_index_metrics = self.config.populate_index_metrics;
            }
            if request_headers.populate_query_metrics.is_none() {
                request_headers.populate_query_metrics = self.config.populate_query_metrics;
            }
            if request_headers.max_item_count.is_none() {
                request_headers.max_item_count =
                    self.config.max_item_count.map(|c| c.to_header_value());
            }
            operation = operation.with_request_headers(request_headers);
        }

        // Clone the pre-built static headers and add the continuation token
        // (the only header that changes between pages).
        let mut headers = self.base_headers.clone();
        if let Some(continuation) = &self.continuation {
            headers.insert(
                constants::CONTINUATION.clone(),
                HeaderValue::from(continuation.clone()),
            );
        }

        let op_options = self.base_options.clone().with_custom_headers(headers);

        // Execute through the driver
        let driver_response = self.driver.execute_operation(operation, op_options).await?;

        // Bridge driver response to SDK types
        let cosmos_response = driver_bridge::driver_response_to_cosmos_response(driver_response);

        let page = QueryFeedPage::<T>::from_response(cosmos_response).await?;

        match page.continuation() {
            Some(token) => self.continuation = Some(token.to_string()),
            None => self.complete = true,
        }

        Ok(Some(page))
    }
}

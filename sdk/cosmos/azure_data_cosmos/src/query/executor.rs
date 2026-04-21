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

use crate::{constants, driver_bridge, feed::FeedBody, Query, QueryFeedPage};

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
    session_token: Option<SessionToken>,
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
        session_token: Option<SessionToken>,
    ) -> Self {
        // Pre-build the static headers that are the same for every page:
        // user-provided custom headers + query-specific constants.
        let mut base_headers = base_options.custom_headers().cloned().unwrap_or_default();
        base_headers.insert(constants::QUERY.clone(), HeaderValue::from_static("True"));
        base_headers.insert(
            azure_core::http::headers::CONTENT_TYPE,
            HeaderValue::from_static("application/query+json"),
        );

        Self {
            driver,
            operation_factory: Box::new(operation_factory),
            query,
            query_body: None,
            base_options,
            base_headers,
            session_token,
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
        if let Some(session_token) = &self.session_token {
            operation = operation.with_session_token(session_token.clone());
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
        let cosmos_response =
            driver_bridge::driver_response_to_cosmos_response::<FeedBody<T>>(driver_response);

        let page = QueryFeedPage::<T>::from_response(cosmos_response).await?;

        match page.continuation() {
            Some(token) => self.continuation = Some(token.to_string()),
            None => self.complete = true,
        }

        Ok(Some(page))
    }
}

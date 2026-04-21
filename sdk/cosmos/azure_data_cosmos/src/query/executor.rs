// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Query execution implementation.

use std::sync::Arc;

use azure_core::http::headers::{Header, HeaderValue};
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
    base_options: DriverOperationOptions,
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
        Self {
            driver,
            operation_factory: Box::new(operation_factory),
            query,
            base_options,
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

        // Serialize query body
        let body = serde_json::to_vec(&self.query)?;
        operation = operation.with_body(body);

        // Set session token if provided
        if let Some(session_token) = &self.session_token {
            operation = operation.with_session_token(session_token.clone());
        }

        // Build custom headers for the driver operation options
        let mut headers = self
            .base_options
            .custom_headers()
            .cloned()
            .unwrap_or_default();

        // Query-specific headers
        headers.insert(constants::QUERY.clone(), HeaderValue::from_static("True"));
        headers.insert(
            constants::QUERY_CONTENT_TYPE.name().clone(),
            HeaderValue::from(constants::QUERY_CONTENT_TYPE.value().as_str().to_owned()),
        );

        // Continuation token
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

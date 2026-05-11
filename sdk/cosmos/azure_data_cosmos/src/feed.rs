// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{marker::PhantomData, pin::Pin, sync::Arc, task};

use azure_core::http::{
    headers::Headers,
    pager::{PagerContinuation, PagerResult},
};
use azure_data_cosmos_driver::{
    models::{ContainerReference, CosmosResponse as DriverResponse, CosmosResponseHeaders},
    options::OperationOptions,
    CosmosDriver, OperationPlan,
};
use futures::future::BoxFuture;
use futures::Stream;
use serde::{de::DeserializeOwned, Deserialize};

use crate::{
    constants, driver_bridge,
    models::{CosmosDiagnostics, CosmosResponse},
    ContinuationToken, SessionToken,
};

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

    /// The continuation token for the next page of results.
    continuation: Option<String>,

    /// Raw HTTP response headers.
    raw_headers: Headers,

    /// Parsed Cosmos-specific response headers.
    headers: CosmosResponseHeaders,

    /// Diagnostics for this page.
    diagnostics: CosmosDiagnostics,
}

impl<T> FeedPage<T> {
    /// Creates a new `FeedPage` instance.
    pub(crate) fn new(
        items: Vec<T>,
        continuation: Option<String>,
        raw_headers: Headers,
        headers: CosmosResponseHeaders,
        diagnostics: CosmosDiagnostics,
    ) -> Self {
        Self {
            items,
            continuation,
            raw_headers,
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

    /// Gets the continuation token for the next page of results, if any.
    pub fn continuation(&self) -> Option<&str> {
        self.continuation.as_deref()
    }

    /// Gets any headers returned by the server for this page of results.
    pub fn headers(&self) -> &Headers {
        &self.raw_headers
    }

    /// Returns the request charge (RU consumption) for this page, if available.
    pub fn request_charge(&self) -> Option<f64> {
        self.headers.request_charge.as_ref().map(|rc| rc.value())
    }

    /// Returns the session token from this page, if available.
    pub fn session_token(&self) -> Option<SessionToken> {
        self.headers
            .session_token
            .as_ref()
            .map(|st| SessionToken::from(st.as_str().to_string()))
    }

    /// Returns the diagnostics for this page.
    ///
    /// Provides access to the activity ID, server-side duration, and other
    /// diagnostic information for debugging and performance analysis.
    pub fn diagnostics(&self) -> &CosmosDiagnostics {
        &self.diagnostics
    }
}

impl<T> From<FeedPage<T>> for PagerResult<FeedPage<T>> {
    fn from(value: FeedPage<T>) -> Self {
        let continuation = value.continuation.clone();
        match continuation {
            Some(continuation) => PagerResult::More {
                response: value,
                continuation: PagerContinuation::Token(continuation),
            },
            None => PagerResult::Done { response: value },
        }
    }
}

impl<T: DeserializeOwned> FeedPage<T> {
    #[allow(dead_code)] // Will be used by future read-many and change feed operations
    pub(crate) async fn from_response(
        response: CosmosResponse<FeedBody<T>>,
    ) -> azure_core::Result<Self> {
        let raw_headers = response.headers().clone();
        let continuation = raw_headers.get_optional_string(&constants::CONTINUATION);
        let cosmos_headers = response.cosmos_headers().clone();
        let diagnostics = response.diagnostics().clone();
        let body: FeedBody<T> = response.into_model()?;

        Ok(Self::new(
            body.items,
            continuation,
            raw_headers,
            cosmos_headers,
            diagnostics,
        ))
    }
}

/// Represents a single page of results from a Cosmos DB query.
///
/// Wraps a [`FeedPage`] and adds query-specific metadata such as
/// [`index_metrics()`](Self::index_metrics) and [`query_metrics()`](Self::query_metrics).
///
/// This type is yielded by [`FeedItemIterator`] and [`FeedPageIterator`] for query operations.
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
    /// Gets the items in this page of results.
    pub fn items(&self) -> &[T] {
        self.page.items()
    }

    /// Consumes the page and returns a vector of the items.
    pub fn into_items(self) -> Vec<T> {
        self.page.into_items()
    }

    /// Gets the continuation token for the next page of results, if any.
    pub fn continuation(&self) -> Option<&str> {
        self.page.continuation()
    }

    /// Gets any headers returned by the server for this page of results.
    pub fn headers(&self) -> &Headers {
        self.page.headers()
    }

    /// Returns the request charge (RU consumption) for this page, if available.
    pub fn request_charge(&self) -> Option<f64> {
        self.page.request_charge()
    }

    /// Returns the session token from this page, if available.
    pub fn session_token(&self) -> Option<SessionToken> {
        self.page.session_token()
    }

    /// Returns the diagnostics for this page.
    ///
    /// Provides access to the activity ID, server-side duration, and other
    /// diagnostic information for debugging and performance analysis.
    pub fn diagnostics(&self) -> &CosmosDiagnostics {
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

impl<T> From<QueryFeedPage<T>> for PagerResult<QueryFeedPage<T>> {
    fn from(value: QueryFeedPage<T>) -> Self {
        let continuation = value.page.continuation.clone();
        match continuation {
            Some(continuation) => PagerResult::More {
                response: value,
                continuation: PagerContinuation::Token(continuation),
            },
            None => PagerResult::Done { response: value },
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct FeedBody<T> {
    #[serde(alias = "Documents")]
    #[serde(alias = "DocumentCollections")]
    #[serde(alias = "Databases")]
    #[serde(alias = "Offers")]
    pub(crate) items: Vec<T>,
}

impl<T: DeserializeOwned> QueryFeedPage<T> {
    pub(crate) fn from_response(response: CosmosResponse<FeedBody<T>>) -> azure_core::Result<Self> {
        let raw_headers = response.headers().clone();
        let continuation = raw_headers.get_optional_string(&constants::CONTINUATION);
        let cosmos_headers = response.cosmos_headers().clone();
        let index_metrics = cosmos_headers.index_metrics.clone();
        let query_metrics = cosmos_headers.query_metrics.clone();
        let diagnostics = response.diagnostics().clone();
        let body: FeedBody<T> = response.into_model()?;

        Ok(Self {
            page: FeedPage::new(
                body.items,
                continuation,
                raw_headers,
                cosmos_headers,
                diagnostics,
            ),
            index_metrics,
            query_metrics,
        })
    }
}

type DriverPageFuture =
    BoxFuture<'static, (OperationPlan, azure_core::Result<Option<DriverResponse>>)>;

/// Live pipeline state held by [`FeedPageIterator`] / [`FeedItemIterator`].
///
/// Owns the [`OperationPlan`] directly (rather than burying it inside an
/// `unfold` closure) so that
/// [`FeedPageIterator::to_continuation_token`] can snapshot it between polls.
struct LiveState {
    driver: Arc<CosmosDriver>,
    container: Option<ContainerReference>,
    options: OperationOptions,
    /// Always `Some` while no page fetch is in flight.
    plan: Option<OperationPlan>,
    /// `Some` while a page fetch is pending.
    in_flight: Option<DriverPageFuture>,
    exhausted: bool,
}

impl LiveState {
    fn new(
        driver: Arc<CosmosDriver>,
        container: Option<ContainerReference>,
        plan: OperationPlan,
        options: OperationOptions,
    ) -> Self {
        Self {
            driver,
            container,
            options,
            plan: Some(plan),
            in_flight: None,
            exhausted: false,
        }
    }

    fn poll_next_page<T: DeserializeOwned + Send + 'static>(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<azure_core::Result<QueryFeedPage<T>>>> {
        if self.exhausted {
            return task::Poll::Ready(None);
        }

        if self.in_flight.is_none() {
            // Move the plan into a future. The future returns the plan back so
            // we can store it again between polls.
            let mut plan = self
                .plan
                .take()
                .expect("plan must be present between polls");
            let driver = Arc::clone(&self.driver);
            let container = self.container.clone();
            let options = self.options.clone();
            let fut: DriverPageFuture = Box::pin(async move {
                let result = driver.execute_plan(&mut plan, container, options).await;
                (plan, result)
            });
            self.in_flight = Some(fut);
        }

        let fut = self.in_flight.as_mut().expect("future just installed");
        let (plan, result) = match fut.as_mut().poll(cx) {
            task::Poll::Pending => return task::Poll::Pending,
            task::Poll::Ready(out) => out,
        };
        self.in_flight = None;
        self.plan = Some(plan);

        match result {
            Ok(None) => {
                self.exhausted = true;
                task::Poll::Ready(None)
            }
            Err(err) => {
                self.exhausted = true;
                task::Poll::Ready(Some(Err(err)))
            }
            Ok(Some(driver_response)) => {
                let response = driver_bridge::driver_response_to_cosmos_response::<FeedBody<T>>(
                    driver_response,
                );
                match QueryFeedPage::from_response(response) {
                    Ok(page) => task::Poll::Ready(Some(Ok(page))),
                    Err(err) => {
                        self.exhausted = true;
                        task::Poll::Ready(Some(Err(err)))
                    }
                }
            }
        }
    }

    fn to_continuation_token(&self) -> azure_core::Result<ContinuationToken> {
        let plan = self.plan.as_ref().ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "to_continuation_token called while a page fetch is in flight",
            )
        })?;
        plan.to_continuation_token()
    }
}

/// Internal source of pages for [`FeedPageIterator`] and [`FeedItemIterator`].
///
/// Production iterators use the [`Live`](Self::Live) variant which drives the
/// underlying [`OperationPlan`]. Unit tests use [`Synthetic`](Self::Synthetic)
/// to inject a pre-built sequence of pages.
enum PageSource<T: Send> {
    Live(Box<LiveState>),
    #[cfg(test)]
    Synthetic(std::collections::VecDeque<azure_core::Result<QueryFeedPage<T>>>),
    #[cfg(not(test))]
    #[allow(dead_code)]
    _Phantom(PhantomData<fn() -> T>),
}

impl<T: Send + DeserializeOwned + 'static> PageSource<T> {
    fn poll_next_page(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<azure_core::Result<QueryFeedPage<T>>>> {
        match self {
            PageSource::Live(state) => state.poll_next_page::<T>(cx),
            #[cfg(test)]
            PageSource::Synthetic(pages) => task::Poll::Ready(pages.pop_front()),
            #[cfg(not(test))]
            PageSource::_Phantom(_) => task::Poll::Ready(None),
        }
    }
}

/// Represents a stream of items from a Cosmos DB query.
///
/// See [`QueryFeedPage`] for more details on Cosmos DB feeds.
pub struct FeedItemIterator<T: Send> {
    source: PageSource<T>,
    current: Option<std::vec::IntoIter<T>>,
    _marker: PhantomData<fn() -> T>,
}

impl<T: Send + DeserializeOwned + 'static> FeedItemIterator<T> {
    /// Creates a new `FeedItemIterator` backed by the given operation plan.
    pub(crate) fn new(
        driver: Arc<CosmosDriver>,
        container: Option<ContainerReference>,
        plan: OperationPlan,
        options: OperationOptions,
    ) -> Self {
        Self {
            source: PageSource::Live(Box::new(LiveState::new(driver, container, plan, options))),
            current: None,
            _marker: PhantomData,
        }
    }

    /// Converts this item iterator into a page iterator, yielding full pages
    /// instead of individual items.
    ///
    /// IMPORTANT: This will DISCARD any items from the current page that have
    /// not yet been yielded by the item iterator. Use this method before
    /// consuming any items if you want to switch to page-based iteration.
    pub fn into_pages(self) -> FeedPageIterator<T> {
        FeedPageIterator {
            source: self.source,
            _marker: PhantomData,
        }
    }
}

impl<T: Send + DeserializeOwned + 'static> Stream for FeedItemIterator<T> {
    type Item = azure_core::Result<T>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        // Safety: we never move the inner source/current out via Pin.
        let this = unsafe { self.get_unchecked_mut() };
        loop {
            if let Some(current) = this.current.as_mut() {
                if let Some(item) = current.next() {
                    return task::Poll::Ready(Some(Ok(item)));
                }
                this.current = None;
            }

            match this.source.poll_next_page(cx) {
                task::Poll::Ready(Some(Ok(page))) => {
                    this.current = Some(page.into_items().into_iter());
                    continue;
                }
                task::Poll::Ready(Some(Err(err))) => return task::Poll::Ready(Some(Err(err))),
                task::Poll::Ready(None) => return task::Poll::Ready(None),
                task::Poll::Pending => return task::Poll::Pending,
            }
        }
    }
}

/// A stream of pages from a Cosmos DB feed operation.
///
/// In addition to yielding [`QueryFeedPage`]s like a regular `Stream`, this
/// iterator can be snapshotted into a [`ContinuationToken`] for later
/// resumption via
/// [`to_continuation_token`](Self::to_continuation_token).
pub struct FeedPageIterator<T: Send> {
    source: PageSource<T>,
    _marker: PhantomData<fn() -> T>,
}

impl<T: Send + DeserializeOwned + 'static> FeedPageIterator<T> {
    /// Captures the current iterator position as a [`ContinuationToken`].
    ///
    /// Pass the returned token to a subsequent
    /// [`ContainerClient::query_items`](crate::clients::ContainerClient::query_items)
    /// call (via [`QueryOptions::with_continuation_token`](crate::QueryOptions::with_continuation_token))
    /// to resume the query at the same position.
    ///
    /// Snapshotting is non-mutating; the iterator may continue to be used
    /// afterwards.
    ///
    /// # Errors
    ///
    /// Returns an error if a page fetch is currently in flight (the plan
    /// state is being mutated and cannot be safely snapshotted).
    pub fn to_continuation_token(&self) -> azure_core::Result<ContinuationToken> {
        match &self.source {
            PageSource::Live(state) => state.to_continuation_token(),
            #[cfg(test)]
            PageSource::Synthetic(_) => Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "synthetic test iterator does not support to_continuation_token",
            )),
            #[cfg(not(test))]
            PageSource::_Phantom(_) => unreachable!(),
        }
    }
}

impl<T: Send + DeserializeOwned + 'static> Stream for FeedPageIterator<T> {
    type Item = azure_core::Result<QueryFeedPage<T>>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        // Safety: we never move source out via Pin.
        let this = unsafe { self.get_unchecked_mut() };
        this.source.poll_next_page(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    fn create_test_page<T>(items: Vec<T>, continuation: Option<String>) -> QueryFeedPage<T> {
        QueryFeedPage {
            page: FeedPage::new(
                items,
                continuation,
                Headers::new(),
                CosmosResponseHeaders::default(),
                CosmosDiagnostics::default(),
            ),
            index_metrics: None,
            query_metrics: None,
        }
    }

    fn synthetic_item_iter<T: Send + DeserializeOwned + 'static>(
        pages: Vec<azure_core::Result<QueryFeedPage<T>>>,
    ) -> FeedItemIterator<T> {
        FeedItemIterator {
            source: PageSource::Synthetic(pages.into()),
            current: None,
            _marker: PhantomData,
        }
    }

    #[tokio::test]
    async fn item_iterator_yields_all_items_from_multiple_pages() {
        let pages = vec![
            Ok(create_test_page(vec![1, 2, 3], Some("token1".to_string()))),
            Ok(create_test_page(vec![4, 5], Some("token2".to_string()))),
            Ok(create_test_page(vec![6], None)),
        ];

        let item_iter = synthetic_item_iter(pages);
        let items: Vec<_> = item_iter
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(items, vec![1, 2, 3, 4, 5, 6]);
    }

    #[tokio::test]
    async fn page_iterator_yields_all_pages() {
        let pages = vec![
            Ok(create_test_page(vec![1, 2], Some("token1".to_string()))),
            Ok(create_test_page(vec![3], None)),
        ];

        let page_iter = synthetic_item_iter(pages).into_pages();
        let page_items: Vec<_> = page_iter
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .map(|r| r.unwrap().into_items())
            .collect();
        assert_eq!(page_items, vec![vec![1, 2], vec![3]]);
    }

    #[tokio::test]
    async fn item_iterator_propagates_errors() {
        let pages = vec![
            Ok(create_test_page(vec![1, 2], Some("token".to_string()))),
            Err(azure_core::Error::new(
                azure_core::error::ErrorKind::Other,
                "test error",
            )),
        ];

        let mut item_iter = synthetic_item_iter(pages);

        // First two items should succeed
        assert_eq!(item_iter.next().await.unwrap().unwrap(), 1);
        assert_eq!(item_iter.next().await.unwrap().unwrap(), 2);

        // Third item should be an error
        assert!(item_iter.next().await.unwrap().is_err());
    }

    #[tokio::test]
    async fn item_iterator_handles_empty_pages() {
        let pages = vec![
            Ok(create_test_page(vec![1], Some("token1".to_string()))),
            Ok(create_test_page(vec![], Some("token2".to_string()))),
            Ok(create_test_page(vec![2], None)),
        ];

        let item_iter = synthetic_item_iter(pages);
        let items: Vec<_> = item_iter
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .map(|r| r.unwrap())
            .collect();

        assert_eq!(items, vec![1, 2]);
    }
}

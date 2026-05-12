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
    models::{CosmosDiagnosticsContext, CosmosResponse},
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
    diagnostics: Arc<CosmosDiagnosticsContext>,
}

impl<T> FeedPage<T> {
    /// Creates a new `FeedPage` instance.
    pub(crate) fn new(
        items: Vec<T>,
        continuation: Option<String>,
        raw_headers: Headers,
        headers: CosmosResponseHeaders,
        diagnostics: Arc<CosmosDiagnosticsContext>,
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
    /// The returned [`CosmosDiagnosticsContext`] surfaces the full per-operation
    /// diagnostics produced by the driver pipeline (request tracking, retries,
    /// regions contacted, RU charges, status, etc.).
    pub fn diagnostics(&self) -> Arc<CosmosDiagnosticsContext> {
        Arc::clone(&self.diagnostics)
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
    /// The returned [`CosmosDiagnosticsContext`] surfaces the full per-operation
    /// diagnostics produced by the driver pipeline (request tracking, retries,
    /// regions contacted, RU charges, status, etc.).
    pub fn diagnostics(&self) -> Arc<CosmosDiagnosticsContext> {
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
        let diagnostics = response.diagnostics();
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
#[pin_project::pin_project]
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
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<azure_core::Result<QueryFeedPage<T>>>> {
        // Because we want to be able to use the OperationPlan to generate a continuation token on-demand (generating the token has a perf cost),
        // we can't use a utility like `futures::stream::unfold` to drive the pagination, since that would move the plan into the future and make it inaccessible for token generation until the future completes.
        // So, instead, we have to manually drive the pagination loop here in `poll_next_page`, which allows us to keep the plan in `self` and only move it into the future when we actually need to fetch the next page.
        // Since poll_next_page holds a mutable reference to self, we can safely move the plan in and out of the future as long as we bring it back into self before returning Poll::Ready.

        let this = self.project();

        // Early exit if we're done.
        if *this.exhausted {
            return task::Poll::Ready(None);
        }

        // Is there a current in-flight page fetch future? If not, start one.
        let in_flight = match this.in_flight.as_mut() {
            Some(fut) => fut,
            None => {
                // Move the plan into a future. The future returns the plan back so
                // we can store it again between polls.
                let mut plan = this
                    .plan
                    .take()
                    .expect("plan must be present between polls");
                let driver = Arc::clone(this.driver);
                let container = this.container.clone();
                let options = this.options.clone();
                let fut: DriverPageFuture = Box::pin(async move {
                    let result = driver.execute_plan(&mut plan, container, options).await;
                    (plan, result)
                });
                this.in_flight.insert(fut)
            }
        };

        // Poll the in-flight future.
        let (plan, result) = match in_flight.as_mut().poll(cx) {
            // It's not done yet, so we're not ready to yield a page.
            // We haven't returned the plan back to self yet since it's still being used by the future.
            // That means that if the user tries to generate a continuation token while a page fetch is in-flight, we'll get an error since the plan is not currently available.
            // This is intentional since generating a continuation token while a page fetch is in-flight would be racy (the internal state is being mutated by the in-flight future and we might capture a token that's inconsistent with the actual state of the iteration).
            task::Poll::Pending => return task::Poll::Pending,

            // It's done. Take the the result out.
            task::Poll::Ready(out) => out,
        };

        // Restore the plan back into self so the user can capture a continuation token, and clear the in-flight future since it's done.
        this.in_flight.take();
        *this.plan = Some(plan);

        match result {
            Ok(None) => {
                // The driver returning Ok(None) indicates that there are no more pages to fetch. Mark ourselves as exhausted and return None to end the stream.
                *this.exhausted = true;
                task::Poll::Ready(None)
            }
            Err(err) => {
                // An error from the driver indicates a failure to fetch the next page. Mark ourselves as exhausted and return the error.
                *this.exhausted = true;
                task::Poll::Ready(Some(Err(err)))
            }
            Ok(Some(driver_response)) => {
                // Successfully got a response from the driver. Convert it into a QueryFeedPage and yield it.
                let response = driver_bridge::driver_response_to_cosmos_response::<FeedBody<T>>(
                    driver_response,
                );
                match QueryFeedPage::from_response(response) {
                    Ok(page) => task::Poll::Ready(Some(Ok(page))),
                    Err(err) => {
                        *this.exhausted = true;
                        task::Poll::Ready(Some(Err(err)))
                    }
                }
            }
        }
    }

    /// Captures the current iterator position as a [`ContinuationToken`].
    ///
    /// This can ONLY be called when there is no page fetch currently in-flight.
    /// Attempting to call this method while a page fetch is in-flight will result in an error, since the internal state is being mutated and cannot be safely snapshotted.
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
#[pin_project::pin_project(project = PageSourceProj)]
enum PageSource<T: Send> {
    Live(Pin<Box<LiveState>>),
    #[cfg(test)]
    Synthetic(std::collections::VecDeque<azure_core::Result<QueryFeedPage<T>>>),
    #[cfg(not(test))]
    #[allow(dead_code)]
    _Phantom(PhantomData<fn() -> T>),
}

impl<T: Send + DeserializeOwned + 'static> PageSource<T> {
    fn poll_next_page(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<azure_core::Result<QueryFeedPage<T>>>> {
        match self.project() {
            PageSourceProj::Live(state) => state.as_mut().poll_next_page::<T>(cx),
            #[cfg(test)]
            PageSourceProj::Synthetic(pages) => task::Poll::Ready(pages.pop_front()),
            #[cfg(not(test))]
            PageSourceProj::_Phantom(_) => task::Poll::Ready(None),
        }
    }
}

/// Represents a stream of items from a Cosmos DB query.
///
/// See [`QueryFeedPage`] for more details on Cosmos DB feeds.
#[pin_project::pin_project]
pub struct FeedItemIterator<T: Send> {
    #[pin]
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
            source: PageSource::Live(Box::pin(LiveState::new(driver, container, plan, options))),
            current: None,
            _marker: PhantomData,
        }
    }

    /// Converts this item iterator into a page iterator, yielding full pages
    /// instead of individual items.
    ///
    /// IMPORTANT: This will DISCARD any items from the current page that have
    /// not yet been yielded by the item iterator. Use this method before
    /// consuming any items to cleanly switch to page-based iteration.
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
        let mut this = self.project();
        loop {
            if let Some(current) = this.current {
                if let Some(item) = current.next() {
                    return task::Poll::Ready(Some(Ok(item)));
                }
                this.current.take();
            }

            match this.source.as_mut().poll_next_page(cx) {
                task::Poll::Ready(Some(Ok(page))) => {
                    *this.current = Some(page.into_items().into_iter());
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
#[pin_project::pin_project]
pub struct FeedPageIterator<T: Send> {
    #[pin]
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
    /// afterwards. However, the captured token will resume from the position of the iterator at the time of capture,
    /// so any subsequent calls to `poll_next` after token capture will not affect the captured token's position.
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
        let this = self.project();
        this.source.poll_next_page(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_data_cosmos_driver::models::ActivityId;
    use futures::StreamExt;

    fn create_test_page<T>(items: Vec<T>, continuation: Option<String>) -> QueryFeedPage<T> {
        QueryFeedPage {
            page: FeedPage::new(
                items,
                continuation,
                Headers::new(),
                CosmosResponseHeaders::default(),
                Arc::new(CosmosDiagnosticsContext::for_testing(ActivityId::new_uuid())),
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

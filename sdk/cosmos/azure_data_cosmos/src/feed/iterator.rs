// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Async iterators for Cosmos DB feed and query operations.
//!
//! [`QueryItemIterator`] yields individual items; [`QueryPageIterator`] yields
//! whole [`QueryFeedPage`]s and supports continuation-token snapshotting.

use std::{marker::PhantomData, pin::Pin, sync::Arc, task};

use azure_data_cosmos_driver::{
    models::{ContainerReference, ContinuationToken, CosmosResponse as DriverResponse},
    options::OperationOptions,
    CosmosDriver, OperationPlan,
};
use futures::future::BoxFuture;
use futures::Stream;
use serde::de::DeserializeOwned;

use crate::{driver_bridge, feed::query_page::QueryFeedPage};

type DriverPageFuture = BoxFuture<'static, (OperationPlan, crate::Result<Option<DriverResponse>>)>;

/// Live pipeline state held by [`QueryPageIterator`] / [`QueryItemIterator`].
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
    ) -> task::Poll<Option<crate::Result<QueryFeedPage<T>>>> {
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
                    let result = driver
                        .execute_plan(&mut plan, container, options)
                        .await
                        .map_err(Into::into);
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
                let response = driver_bridge::driver_response_to_cosmos_response(driver_response);
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
    fn to_continuation_token(&self) -> crate::Result<ContinuationToken> {
        let plan = self.plan.as_ref().ok_or_else(|| {
            crate::DriverCosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_FETCH_IN_FLIGHT)
                .with_message("to_continuation_token called while a page fetch is in flight")
                .build()
        })?;
        plan.to_continuation_token().map_err(Into::into)
    }
}

/// Internal source of pages for [`QueryPageIterator`] and [`QueryItemIterator`].
///
/// Production iterators use the [`Live`](Self::Live) variant which drives the
/// underlying [`OperationPlan`]. Unit tests use [`Synthetic`](Self::Synthetic)
/// to inject a pre-built sequence of pages.
#[pin_project::pin_project(project = PageSourceProj)]
enum PageSource<T: Send> {
    Live(Pin<Box<LiveState>>),
    #[cfg(test)]
    Synthetic(std::collections::VecDeque<crate::Result<QueryFeedPage<T>>>),
    #[cfg(not(test))]
    #[allow(dead_code)]
    _Phantom(PhantomData<fn() -> T>),
}

impl<T: Send + DeserializeOwned + 'static> PageSource<T> {
    fn poll_next_page(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<crate::Result<QueryFeedPage<T>>>> {
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
pub struct QueryItemIterator<T: Send> {
    #[pin]
    source: PageSource<T>,
    current: Option<std::vec::IntoIter<T>>,
    _marker: PhantomData<fn() -> T>,
}

impl<T: Send + DeserializeOwned + 'static> QueryItemIterator<T> {
    /// Creates a new `QueryItemIterator` backed by the given operation plan.
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
    pub fn into_pages(self) -> QueryPageIterator<T> {
        QueryPageIterator {
            source: self.source,
            _marker: PhantomData,
        }
    }
}

impl<T: Send + DeserializeOwned + 'static> Stream for QueryItemIterator<T> {
    type Item = crate::Result<T>;

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
pub struct QueryPageIterator<T: Send> {
    #[pin]
    source: PageSource<T>,
    _marker: PhantomData<fn() -> T>,
}

impl<T: Send + DeserializeOwned + 'static> QueryPageIterator<T> {
    /// Captures the current iterator position as a [`ContinuationToken`].
    ///
    /// Pass the returned token to a subsequent
    /// [`ContainerClient::query_items`](crate::clients::ContainerClient::query_items)
    /// call (via [`QueryOptions::with_continuation_token`](crate::options::QueryOptions::with_continuation_token))
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
    pub fn to_continuation_token(&self) -> crate::Result<ContinuationToken> {
        match &self.source {
            PageSource::Live(state) => state.to_continuation_token(),
            #[cfg(test)]
            PageSource::Synthetic(_) => Err(crate::DriverCosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("synthetic test iterator does not support to_continuation_token")
                .build()
                .into()),
            #[cfg(not(test))]
            PageSource::_Phantom(_) => unreachable!(),
        }
    }
}

impl<T: Send + DeserializeOwned + 'static> Stream for QueryPageIterator<T> {
    type Item = crate::Result<QueryFeedPage<T>>;

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

    use crate::{diagnostics::DiagnosticsContext, models::ResponseHeaders};

    fn create_test_page<T>(items: Vec<T>) -> QueryFeedPage<T> {
        QueryFeedPage::new_for_testing(
            items,
            ResponseHeaders::default(),
            Arc::new(DiagnosticsContext::for_testing(ActivityId::new_uuid())),
        )
    }

    fn synthetic_item_iter<T: Send + DeserializeOwned + 'static>(
        pages: Vec<crate::Result<QueryFeedPage<T>>>,
    ) -> QueryItemIterator<T> {
        QueryItemIterator {
            source: PageSource::Synthetic(pages.into()),
            current: None,
            _marker: PhantomData,
        }
    }

    #[tokio::test]
    async fn item_iterator_yields_all_items_from_multiple_pages() {
        let pages = vec![
            Ok(create_test_page(vec![1, 2, 3])),
            Ok(create_test_page(vec![4, 5])),
            Ok(create_test_page(vec![6])),
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
            Ok(create_test_page(vec![1, 2])),
            Ok(create_test_page(vec![3])),
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
            Ok(create_test_page(vec![1, 2])),
            Err(crate::DriverCosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("test error")
                .build()
                .into()),
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
            Ok(create_test_page(vec![1])),
            Ok(create_test_page(vec![])),
            Ok(create_test_page(vec![2])),
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

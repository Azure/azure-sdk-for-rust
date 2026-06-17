// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`ChangeFeedPageIterator`] — async page-level stream for change feed operations.

use std::{marker::PhantomData, pin::Pin, sync::Arc, task};

use azure_data_cosmos_driver::{
    models::{ContainerReference, ContinuationToken, CosmosResponse as DriverResponse},
    options::OperationOptions,
    CosmosDriver, OperationPlan,
};
use futures::future::BoxFuture;
use futures::Stream;
use serde::de::DeserializeOwned;

use crate::{driver_bridge, feed::page::FeedBody, feed::FeedPage};

type DriverPageFuture = BoxFuture<'static, (OperationPlan, crate::Result<Option<DriverResponse>>)>;

/// Internal pipeline state for [`ChangeFeedPageIterator`].
#[pin_project::pin_project]
struct LiveState {
    driver: Arc<CosmosDriver>,
    container: Option<ContainerReference>,
    options: OperationOptions,
    plan: Option<OperationPlan>,
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
    ) -> task::Poll<Option<crate::Result<FeedPage<T>>>> {
        let this = self.project();

        if *this.exhausted {
            return task::Poll::Ready(None);
        }

        let in_flight = match this.in_flight.as_mut() {
            Some(fut) => fut,
            None => {
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

        let (plan, result) = match in_flight.as_mut().poll(cx) {
            task::Poll::Pending => return task::Poll::Pending,
            task::Poll::Ready(out) => out,
        };

        this.in_flight.take();
        *this.plan = Some(plan);

        match result {
            Ok(None) => {
                *this.exhausted = true;
                task::Poll::Ready(None)
            }
            Err(err) => {
                *this.exhausted = true;
                task::Poll::Ready(Some(Err(err)))
            }
            Ok(Some(driver_response)) => {
                let response = driver_bridge::driver_response_to_cosmos_response(driver_response);
                let headers = response.cosmos_headers().clone();
                let diagnostics = response.diagnostics();
                match response.into_model::<FeedBody<T>>() {
                    Ok(body) => {
                        let page = FeedPage::new(body.items, headers, diagnostics);
                        task::Poll::Ready(Some(Ok(page)))
                    }
                    Err(err) => {
                        *this.exhausted = true;
                        task::Poll::Ready(Some(Err(err)))
                    }
                }
            }
        }
    }

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

/// A stream of pages from a Cosmos DB change feed operation.
///
/// Yields [`FeedPage<T>`] instances, where `T` is the user's document type
/// (for LatestVersion mode).
///
/// Use [`to_continuation_token()`](Self::to_continuation_token) to capture
/// the current position for later resumption.
///
/// # Examples
///
/// ```rust,no_run
/// use azure_data_cosmos::{clients::ContainerClient, feed::FeedScope};
/// use futures::StreamExt;
/// use serde::Deserialize;
///
/// #[derive(Debug, Deserialize)]
/// struct MyItem { id: String }
///
/// # async fn example(container: ContainerClient) -> Result<(), Box<dyn std::error::Error>> {
/// let mut pages = container
///     .read_change_feed::<MyItem>(FeedScope::full_container(), None)
///     .await?;
///
/// while let Some(page) = pages.next().await {
///     let page = page?;
///     for item in page.items() {
///         println!("changed: {:?}", item);
///     }
///     // Checkpoint
///     let token = pages.to_continuation_token()?;
///     // save_checkpoint(token).await;
/// }
/// # Ok(())
/// # }
/// ```
#[pin_project::pin_project]
pub struct ChangeFeedPageIterator<T: Send> {
    #[pin]
    state: Pin<Box<LiveState>>,
    _marker: PhantomData<fn() -> T>,
}

impl<T: Send + DeserializeOwned + 'static> ChangeFeedPageIterator<T> {
    /// Creates a new `ChangeFeedPageIterator` backed by the given operation plan.
    pub(crate) fn new(
        driver: Arc<CosmosDriver>,
        container: Option<ContainerReference>,
        plan: OperationPlan,
        options: OperationOptions,
    ) -> Self {
        Self {
            state: Box::pin(LiveState::new(driver, container, plan, options)),
            _marker: PhantomData,
        }
    }

    /// Captures the current iterator position as a [`ContinuationToken`].
    ///
    /// Pass the returned token to a subsequent
    /// [`ContainerClient::read_change_feed`](crate::clients::ContainerClient::read_change_feed)
    /// call (via [`ChangeFeedOptions::with_continuation_token`](crate::options::ChangeFeedOptions::with_continuation_token))
    /// to resume the change feed at the same position.
    ///
    /// Snapshotting is non-mutating; the iterator may continue to be used
    /// afterwards.
    ///
    /// # Errors
    ///
    /// Returns an error if a page fetch is currently in flight.
    pub fn to_continuation_token(&self) -> crate::Result<ContinuationToken> {
        self.state.to_continuation_token()
    }
}

impl<T: Send + DeserializeOwned + 'static> Stream for ChangeFeedPageIterator<T> {
    type Item = crate::Result<FeedPage<T>>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        let this = self.project();
        this.state.as_mut().poll_next_page(cx)
    }
}

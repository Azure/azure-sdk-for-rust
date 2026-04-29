// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`ThroughputPoller`] type for polling asynchronous throughput replacement operations.
//!
//! Cosmos DB throughput LROs differ from standard ARM LROs: instead of returning an
//! `Operation-Location` URL, the service signals async processing via the
//! `x-ms-offer-replace-pending` response header (or HTTP 202 status). Polling is done by
//! re-reading the offer resource by its RID. Because of this non-standard pattern,
//! [`ThroughputPoller`] is a custom type rather than using [`azure_core::http::Poller`], which
//! expects URL-based [`StatusMonitor`](azure_core::http::poller::StatusMonitor) continuation.

use crate::{
    clients::offers_client,
    constants,
    models::{CosmosResponse, ResourceResponse, ThroughputProperties},
};
use azure_core::http::StatusCode;
use azure_core::time::Duration;
use azure_data_cosmos_driver::models::AccountReference;
use azure_data_cosmos_driver::CosmosDriver;
use futures::{stream::BoxStream, Stream, StreamExt};
use std::{
    future::{Future, IntoFuture},
    pin::Pin,
    sync::Arc,
    task,
};

/// Default polling interval for throughput replacement operations.
const DEFAULT_POLLING_INTERVAL: Duration = Duration::seconds(5);

/// A poller for an asynchronous throughput replacement operation.
///
/// When Cosmos DB processes a throughput change, it may complete synchronously (HTTP 200) or
/// asynchronously (HTTP 202 with `x-ms-offer-replace-pending: true`). This type abstracts
/// that distinction, allowing callers to simply `await` the final result or poll for progress.
///
/// Dropping the poller does not cancel the server-side throughput change. The operation
/// continues to completion on the server regardless.
///
/// # Usage
///
/// ```rust,no_run
/// # use azure_data_cosmos::models::ThroughputProperties;
/// # async fn example(container_client: azure_data_cosmos::clients::ContainerClient) -> azure_core::Result<()> {
/// // Simple: just await the final result
/// let throughput = container_client
///     .begin_replace_throughput(ThroughputProperties::manual(500), None)
///     .await? // start the replace operation
///     .await? // wait for completion (polls if async)
///     .into_model()?;
///
/// // Advanced: poll for progress via Stream
/// use futures::TryStreamExt;
/// let mut poller = container_client
///     .begin_replace_throughput(ThroughputProperties::manual(500), None)
///     .await?;
/// while let Some(status) = poller.try_next().await? {
///     if let Some(charge) = status.request_charge() {
///         println!("Request charge: {charge}");
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub struct ThroughputPoller {
    stream: BoxStream<'static, azure_core::Result<CosmosResponse<ThroughputProperties>>>,
}

impl ThroughputPoller {
    /// Creates a new `ThroughputPoller` from the initial replace response.
    ///
    /// The `offer_id` is provided by the caller (extracted before the replace request)
    /// to enable efficient single-GET polling without re-querying.
    pub(crate) fn new(
        initial_response: CosmosResponse<ThroughputProperties>,
        driver: Arc<CosmosDriver>,
        account: AccountReference,
        offer_id: String,
    ) -> Self {
        let is_pending = is_offer_replace_pending(&initial_response);

        if is_pending {
            Self::pending(initial_response, driver, account, offer_id)
        } else {
            Self::completed(initial_response)
        }
    }

    /// Creates a poller for an operation that completed synchronously.
    fn completed(response: CosmosResponse<ThroughputProperties>) -> Self {
        let stream = futures::stream::once(async { Ok(response) });
        Self {
            stream: Box::pin(stream),
        }
    }

    /// Creates a poller for an operation that is still pending.
    fn pending(
        initial_response: CosmosResponse<ThroughputProperties>,
        driver: Arc<CosmosDriver>,
        account: AccountReference,
        offer_id: String,
    ) -> Self {
        let polling_interval = DEFAULT_POLLING_INTERVAL;

        let stream = futures::stream::unfold(
            Some(PollState::Initial(Box::new(initial_response))),
            move |state| {
                let driver = driver.clone();
                let account = account.clone();
                let offer_id = offer_id.clone();
                async move {
                    let state = state?;
                    match state {
                        PollState::Initial(response) => {
                            Some((Ok(*response), Some(PollState::Polling)))
                        }
                        PollState::Polling => {
                            azure_core::sleep::sleep(polling_interval).await;
                            let result =
                                offers_client::read_offer_by_id(&driver, &account, &offer_id).await;
                            match result {
                                Ok(response) => {
                                    if is_offer_replace_pending(&response) {
                                        Some((Ok(response), Some(PollState::Polling)))
                                    } else {
                                        Some((Ok(response), None))
                                    }
                                }
                                Err(e) => Some((Err(e), None)),
                            }
                        }
                    }
                }
            },
        );

        Self {
            stream: Box::pin(stream),
        }
    }
}

/// Internal state for the polling stream.
enum PollState {
    /// The initial replace response has been received but not yet yielded.
    Initial(Box<CosmosResponse<ThroughputProperties>>),
    /// Polling for completion.
    Polling,
}

impl Stream for ThroughputPoller {
    type Item = azure_core::Result<ResourceResponse<ThroughputProperties>>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        self.stream
            .poll_next_unpin(cx)
            .map(|opt| opt.map(|res| res.map(ResourceResponse::new)))
    }
}

impl IntoFuture for ThroughputPoller {
    type Output = azure_core::Result<ResourceResponse<ThroughputProperties>>;
    type IntoFuture = Pin<
        Box<dyn Future<Output = azure_core::Result<ResourceResponse<ThroughputProperties>>> + Send>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let mut stream = self.stream;
            let mut last_response = None;
            while let Some(result) = stream.next().await {
                last_response = Some(result?);
            }
            last_response.map(ResourceResponse::new).ok_or_else(|| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "throughput poller stream ended without yielding a response",
                )
            })
        })
    }
}

/// Checks whether the `x-ms-offer-replace-pending` header indicates a pending operation.
fn is_offer_replace_pending<T>(response: &CosmosResponse<T>) -> bool {
    if let Some(pending) = response
        .headers()
        .get_optional_str(&constants::OFFER_REPLACE_PENDING)
    {
        return pending.eq_ignore_ascii_case("true");
    }
    // Also treat HTTP 202 as pending (even without the header).
    response.status() == StatusCode::Accepted
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::TryStreamExt;

    #[test]
    fn is_offer_replace_pending_returns_false_for_ok() {
        let response = create_mock_response(StatusCode::Ok, None);
        assert!(!is_offer_replace_pending(&response));
    }

    #[test]
    fn is_offer_replace_pending_returns_true_for_accepted() {
        let response = create_mock_response(StatusCode::Accepted, None);
        assert!(is_offer_replace_pending(&response));
    }

    #[test]
    fn is_offer_replace_pending_returns_true_for_header() {
        let response = create_mock_response(StatusCode::Ok, Some("true"));
        assert!(is_offer_replace_pending(&response));
    }

    #[test]
    fn is_offer_replace_pending_returns_false_for_header_false() {
        let response = create_mock_response(StatusCode::Ok, Some("false"));
        assert!(!is_offer_replace_pending(&response));
    }

    #[tokio::test]
    async fn completed_poller_yields_one_item() {
        let response = create_mock_response(StatusCode::Ok, None);
        let mut poller = ThroughputPoller::completed(response);

        let first = poller.try_next().await.expect("should yield Ok");
        assert!(first.is_some(), "should yield one item");

        let second = poller.try_next().await.expect("should yield Ok");
        assert!(second.is_none(), "should end after one item");
    }

    #[tokio::test]
    async fn completed_poller_into_future_returns_response() {
        let response = create_mock_response(StatusCode::Ok, None);
        let poller = ThroughputPoller::completed(response);

        let result = poller.await;
        assert!(result.is_ok(), "into_future should return Ok");
        assert_eq!(result.unwrap().status(), StatusCode::Ok);
    }

    fn create_mock_response(
        status: StatusCode,
        offer_replace_pending: Option<&str>,
    ) -> CosmosResponse<ThroughputProperties> {
        use crate::cosmos_request::CosmosRequest;
        use crate::operation_context::OperationType;
        use crate::resource_context::{ResourceLink, ResourceType};
        use azure_core::{
            http::{headers::Headers, response::Response, RawResponse},
            Bytes,
        };

        let mut headers = Headers::new();
        if let Some(value) = offer_replace_pending {
            headers.insert(constants::OFFER_REPLACE_PENDING, value.to_owned());
        }

        let body = Bytes::from_static(b"{}");
        let raw = RawResponse::from_bytes(status, headers, body);
        let typed: Response<ThroughputProperties> = raw.into();
        let resource_link = ResourceLink::root(ResourceType::Offers);
        let request = CosmosRequest::builder(OperationType::Replace, resource_link)
            .build()
            .unwrap();
        CosmosResponse::new(typed, request)
    }
}

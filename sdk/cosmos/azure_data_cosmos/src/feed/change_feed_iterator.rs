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

use crate::{driver_bridge, feed::page::FeedBody, feed::FeedPage, models::CosmosResponse};

type DriverPageFuture = BoxFuture<'static, (OperationPlan, crate::Result<Option<DriverResponse>>)>;

/// Internal pipeline state for [`ChangeFeedPageIterator`].
#[pin_project::pin_project]
struct LiveState {
    driver: Arc<CosmosDriver>,
    container: Option<ContainerReference>,
    options: OperationOptions,
    plan: Option<OperationPlan>,
    in_flight: Option<DriverPageFuture>,
    errored: bool,
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
            errored: false,
        }
    }

    fn poll_next_page<T: DeserializeOwned + Send + 'static>(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<crate::Result<FeedPage<T>>>> {
        let this = self.project();

        if *this.errored {
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
                // The change feed is a conceptually infinite stream: "no
                // changes" surfaces as a 304 / empty page (handled below),
                // never as a drained pipeline. A `None` here therefore means
                // the pipeline drained unexpectedly — an internal invariant
                // violation. Surface it as an error rather than silently
                // ending the caller's polling loop.
                *this.errored = true;
                let err = crate::DriverCosmosError::builder()
                    .with_status(
                        crate::error::CosmosStatus::CLIENT_CHANGE_FEED_PIPELINE_UNEXPECTEDLY_DRAINED,
                    )
                    .with_message(
                        "change feed pipeline drained unexpectedly; the change feed stream is \
                         infinite and should surface empty pages (304) rather than terminating",
                    )
                    .build();
                task::Poll::Ready(Some(Err(err.into())))
            }
            Err(err) => {
                *this.errored = true;
                task::Poll::Ready(Some(Err(err)))
            }
            Ok(Some(driver_response)) => {
                let response = driver_bridge::driver_response_to_cosmos_response(driver_response);
                let status = response.status();
                let headers = response.cosmos_headers().clone();
                let diagnostics = response.diagnostics();

                // 304 Not Modified means no changes for this partition.
                // Return an empty page — do not try to deserialize the
                // (potentially empty) body.
                if status.status_code() == azure_core::http::StatusCode::NotModified {
                    let page = FeedPage::new(Vec::new(), headers, diagnostics);
                    return task::Poll::Ready(Some(Ok(page)));
                }

                match unwrap_change_feed_items::<T>(response) {
                    Ok(items) => {
                        let page = FeedPage::new(items, headers, diagnostics);
                        task::Poll::Ready(Some(Ok(page)))
                    }
                    Err(err) => {
                        *this.errored = true;
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

/// Deserializes a change feed response body into the caller's document type,
/// unwrapping the structured wire-format envelope when present.
///
/// The change feed is always read with the
/// `x-ms-cosmos-changefeed-wire-format-version` header set (see
/// [`CosmosOperation::change_feed`]), so a conforming service wraps every item
/// as `{ "current": <document>, "metadata": { ... }, ... }`. LatestVersion has
/// no pre-image, but `current` always carries the document.
///
/// The unwrap is deliberately **tolerant**: it strips `current` when an item is
/// enveloped and otherwise passes the item through unchanged. We send the
/// header on every request, so the enveloped shape is the expected one — but
/// the SDK cannot guarantee the service honors it. A backend that does not
/// support the wire-format-version header (an older gateway, a region where the
/// feature has not rolled out, or an emulator build without LatestVersion
/// enveloping) returns the legacy flat `Documents[]` shape instead. Accepting
/// both keeps the iterator working in either case rather than failing the read.
/// See [`unwrap_change_feed_item`] for the one documented edge case.
///
/// [`CosmosOperation::change_feed`]: azure_data_cosmos_driver::models::CosmosOperation
fn unwrap_change_feed_items<T: DeserializeOwned>(
    response: CosmosResponse,
) -> crate::Result<Vec<T>> {
    let body: FeedBody<serde_json::Value> = response.into_model()?;
    body.items
        .into_iter()
        .map(|item| serde_json::from_value(unwrap_change_feed_item(item)).map_err(Into::into))
        .collect()
}

/// Unwraps the `current` document from a single change feed wire-format
/// envelope, or returns the value unchanged when it is not enveloped.
///
/// An item is treated as enveloped when it is a JSON object that carries a
/// `current` field, in which case that field's value is returned. Any other
/// item — including a flat document with no `current` field, or a non-object
/// value — is returned as-is. This lets both the structured wire format and the
/// legacy flat shape deserialize into the caller's type (see
/// [`unwrap_change_feed_items`] for why both are tolerated).
///
/// Edge case: a flat user document that itself has a top-level field named
/// `current` would be unwrapped to that field's value, as though it were an
/// envelope. This is accepted as a documented limitation — the structured wire
/// format is the contract, and tolerating the flat shape is a compatibility
/// fallback — so a top-level `current` field is reserved by the wire format.
fn unwrap_change_feed_item(item: serde_json::Value) -> serde_json::Value {
    match item {
        serde_json::Value::Object(mut fields) => match fields.remove("current") {
            Some(current) => current,
            None => serde_json::Value::Object(fields),
        },
        other => other,
    }
}

/// A stream of pages from a Cosmos DB change feed operation.
///
/// Yields [`FeedPage<T>`] instances, where `T` is the user's document type
/// (for LatestVersion mode). The stream is conceptually infinite: when a
/// partition has no new changes (304 Not Modified), an empty page is
/// returned instead of terminating the stream. The consumer decides when
/// to stop polling.
///
/// Use [`to_continuation_token()`](Self::to_continuation_token) to capture
/// the current position for later resumption.
///
/// # Examples
///
/// ```rust,no_run
/// use azure_data_cosmos::{clients::ContainerClient, feed::FeedScope, options::ChangeFeedStartFrom};
/// use futures::StreamExt;
/// use serde::Deserialize;
///
/// #[derive(Debug, Deserialize)]
/// struct MyItem { id: String }
///
/// # async fn example(container: ContainerClient) -> Result<(), Box<dyn std::error::Error>> {
/// let mut pages = container
///     .query_change_feed::<MyItem>(FeedScope::full_container(), ChangeFeedStartFrom::Beginning, None)
///     .await?;
///
/// while let Some(page) = pages.next().await {
///     let page = page?;
///     if page.items().is_empty() {
///         // No changes right now — checkpoint and wait before retrying.
///         let _token = pages.to_continuation_token()?;
///         break;
///     }
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
    // `state` is already heap-pinned (`Pin<Box<_>>`), so it is not a
    // structurally-pinned field of this struct. Projecting it as `&mut`
    // lets us re-derive `Pin<&mut LiveState>` via `Pin::as_mut` on each poll.
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
    /// [`ContainerClient::query_change_feed`](crate::clients::ContainerClient::query_change_feed)
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

#[cfg(test)]
mod tests {
    use super::{unwrap_change_feed_item, unwrap_change_feed_items};
    use crate::models::CosmosResponse;
    use azure_core::http::StatusCode;
    use azure_data_cosmos_driver::diagnostics::DiagnosticsContext;
    use azure_data_cosmos_driver::models::{
        ActivityId, CosmosResponseHeaders, CosmosStatus, ResponseBody,
    };
    use serde::Deserialize;
    use serde_json::json;
    use std::sync::Arc;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Doc {
        id: String,
    }

    #[test]
    fn unwraps_current_from_envelope() {
        let item = json!({
            "current": { "id": "1" },
            "metadata": { "operationType": "create" }
        });
        assert_eq!(unwrap_change_feed_item(item), json!({ "id": "1" }));
    }

    #[test]
    fn passes_through_flat_document() {
        // A backend that does not honor the wire-format header returns flat
        // documents; the unwrap must pass those through unchanged.
        let item = json!({ "id": "2", "value": 7 });
        assert_eq!(
            unwrap_change_feed_item(item),
            json!({ "id": "2", "value": 7 })
        );
    }

    #[test]
    fn deserializes_enveloped_change_feed_page() {
        let body = json!({
             "Documents": [
                 { "current": { "id": "1" }, "metadata": {} },
                 { "current": { "id": "2" }, "metadata": {} }
             ],
             "_count": 2
        });
        let items: Vec<Doc> = unwrap_change_feed_items(make_response(body)).unwrap();
        assert_eq!(items, vec![Doc { id: "1".into() }, Doc { id: "2".into() }]);
    }

    #[test]
    fn deserializes_flat_change_feed_page() {
        // The legacy flat shape (no envelope) still deserializes correctly,
        // so the iterator tolerates a service that ignores the header.
        let body = json!({
             "Documents": [
                 { "id": "1" },
                 { "id": "2" }
             ],
             "_count": 2
        });
        let items: Vec<Doc> = unwrap_change_feed_items(make_response(body)).unwrap();
        assert_eq!(items, vec![Doc { id: "1".into() }, Doc { id: "2".into() }]);
    }

    /// Builds an SDK [`CosmosResponse`] wrapping the given JSON body so the
    /// unwrap helper can be exercised end to end.
    fn make_response(body: serde_json::Value) -> CosmosResponse {
        let bytes = azure_core::Bytes::from(serde_json::to_vec(&body).unwrap());
        let driver_body = ResponseBody::Bytes(bytes);
        let status = CosmosStatus::new(StatusCode::Ok);
        let diagnostics = Arc::new(DiagnosticsContext::for_testing(ActivityId::new_uuid()));
        CosmosResponse::from_driver_parts(
            driver_body.into(),
            CosmosResponseHeaders::new().into(),
            status,
            diagnostics,
        )
    }
}

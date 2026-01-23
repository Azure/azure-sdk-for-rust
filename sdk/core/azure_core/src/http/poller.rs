// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types and methods for long-running operations (LROs).

use crate::{
    conditional_send::ConditionalSend,
    error::{ErrorKind, ErrorResponse},
    http::{
        headers::{HeaderName, Headers},
        policies::create_public_api_span,
        Context, Format, JsonFormat, Response, StatusCode, Url,
    },
    sleep,
    time::{Duration, OffsetDateTime},
    tracing::{Span, SpanStatus},
};
use futures::{channel::oneshot, stream::unfold, Stream, StreamExt};
use serde::Deserialize;
use std::{
    convert::Infallible,
    fmt,
    future::{Future, IntoFuture},
    pin::Pin,
    str::FromStr,
    sync::Arc,
    task::{Context as TaskContext, Poll},
};

/// Default retry time for long-running operations if no retry-after header is present
///
/// This value is the same as the default used in other Azure SDKs e.g.,
/// <https://github.com/Azure/azure-sdk-for-python/blob/azure-core_1.35.0/sdk/core/azure-core/azure/core/polling/base_polling.py#L586>
const DEFAULT_RETRY_TIME: Duration = Duration::seconds(30);

/// Minimum retry time for long-running operations
const MIN_RETRY_TIME: Duration = Duration::seconds(1);

/// Represents the state of a [`Poller`].
#[derive(Debug, Default, PartialEq, Eq)]
pub enum PollerState<C = Url> {
    /// The poller should fetch the initial status.
    #[default]
    Initial,
    /// The poller should fetch subsequent status.
    More(C),
}

impl<C> PollerState<C> {
    /// Maps a [`PollerState<C>`] to a [`PollerState<U>`] by applying a function to a next link `C` (if `PollerState::More`) or returns `PollerState::Initial` (if `PollerState::Initial`).
    #[inline]
    pub fn map<U, F>(self, f: F) -> PollerState<U>
    where
        F: FnOnce(C) -> U,
    {
        match self {
            PollerState::Initial => PollerState::Initial,
            PollerState::More(c) => PollerState::More(f(c)),
        }
    }
}

impl<C: Clone> Clone for PollerState<C> {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            PollerState::Initial => PollerState::Initial,
            PollerState::More(c) => PollerState::More(c.clone()),
        }
    }
}

/// Long-running operation (LRO) status.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum PollerStatus {
    /// The LRO is still in progress.
    #[default]
    InProgress,

    /// The LRO completed successfully.
    Succeeded,

    /// The LRO failed.
    Failed,

    /// The LRO was canceled.
    Canceled,

    /// Another status not otherwise defined.
    UnknownValue(String),
}

impl From<&str> for PollerStatus {
    fn from(value: &str) -> Self {
        // LRO status should be compared case-insensitively:
        // https://github.com/Azure/azure-sdk-for-rust/issues/2482

        // cspell:words inprogress
        if "inprogress".eq_ignore_ascii_case(value) {
            return PollerStatus::InProgress;
        }

        if "succeeded".eq_ignore_ascii_case(value) {
            return PollerStatus::Succeeded;
        }

        if "failed".eq_ignore_ascii_case(value) {
            return PollerStatus::Failed;
        }

        // While the specification recommends "Canceled", in practice
        // numerous services use "Cancelled".
        if "canceled".eq_ignore_ascii_case(value) || "cancelled".eq_ignore_ascii_case(value) {
            return PollerStatus::Canceled;
        }

        PollerStatus::UnknownValue(value.to_owned())
    }
}

impl FromStr for PollerStatus {
    type Err = Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(value.into())
    }
}

impl<'de> Deserialize<'de> for PollerStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PollerStatusVisitor;
        impl serde::de::Visitor<'_> for PollerStatusVisitor {
            type Value = PollerStatus;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a PollerStatus")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                FromStr::from_str(s).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_str(PollerStatusVisitor)
    }
}

/// Options to create the [`Poller`].
#[derive(Debug, Clone)]
pub struct PollerOptions<'a> {
    /// Allows customization of the method call.
    pub context: Context<'a>,
    /// The time to wait between polling intervals in absence of a `retry-after` header.
    ///
    /// The default is 30 seconds. The minimum time enforced by [`Poller::new`] is 1 second.
    pub frequency: Duration,
}

impl Default for PollerOptions<'_> {
    fn default() -> Self {
        Self {
            frequency: DEFAULT_RETRY_TIME,
            context: Context::new(),
        }
    }
}

impl<'a> PollerOptions<'a> {
    /// Converts these poller options into an owned form so they can outlive the current scope.
    #[must_use]
    pub fn into_owned(self) -> PollerOptions<'static> {
        PollerOptions {
            context: self.context.into_owned(),
            frequency: self.frequency,
        }
    }
}

/// The result of fetching the status monitor from a [`Poller`], whether the long-running operation (LRO) is in progress or done.
pub enum PollerResult<M, C, F = JsonFormat>
where
    M: StatusMonitor,
    F: Format,
{
    /// The long-running operation (LRO) is in progress and the next status monitor update may be fetched from `continuation_token`.
    ///
    /// # Fields
    ///
    /// * `response` contains the HTTP response with the status monitor.
    /// * `retry_after` is the optional client-specified [`Duration`] to wait. The default is 30 seconds.
    /// * `continuation_token` is the next link / continuation token.
    InProgress {
        /// The HTTP response with the status monitor.
        response: Response<M, F>,
        /// The optional client-specified [`Duration`] to wait before polling again.
        retry_after: Duration,
        /// The next link / continuation token.
        continuation_token: C,
    },

    /// The long-running operation (LRO) succeeded and contains the final output.
    ///
    /// # Fields
    ///
    /// * `response` contains the HTTP response with the status monitor in a terminal state.
    Done {
        /// The HTTP response with the status monitor in a terminal state.
        response: Response<M, F>,
    },

    /// The long-running operation (LRO) succeeded and contains the final status.
    ///
    /// # Fields
    ///
    /// * `response` contains the HTTP response with the final status monitor.
    /// * `get_target` is an async function that fetches the final output.
    Succeeded {
        /// The HTTP response with the final status monitor.
        response: Response<M, F>,
        /// An async function that fetches the final output.
        target: BoxedCallback<M>,
    },
}

impl<M, C, F> fmt::Debug for PollerResult<M, C, F>
where
    M: StatusMonitor,
    C: fmt::Debug,
    F: Format,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InProgress {
                retry_after,
                continuation_token,
                ..
            } => f
                .debug_struct("InProgress")
                .field("retry_after", &retry_after)
                .field("continuation_token", &continuation_token)
                .finish_non_exhaustive(),
            Self::Done { .. } => f.debug_struct("Done").finish_non_exhaustive(),
            Self::Succeeded { .. } => f.debug_struct("Succeeded").finish_non_exhaustive(),
        }
    }
}

/// Represents a status monitor for a long-running operation (LRO).
pub trait StatusMonitor {
    /// The model type returned after the long-running operation (LRO) has completed successfully.
    ///
    /// Set this to the unit type `()` if no final resource is expected.
    type Output;

    /// The format used to deserialize the `Output`.
    ///
    /// Set this to [`NoFormat`](crate::http::NoFormat) if no final resource is expected.
    type Format: Format + ConditionalSend;

    /// Gets the [`PollerStatus`] from the status monitor.
    fn status(&self) -> PollerStatus;
}

#[cfg(not(target_arch = "wasm32"))]
mod types {
    use super::{PollerResult, Response, StatusMonitor, Stream};
    use std::{future::Future, pin::Pin};

    pub type BoxedStream<M, F> = Box<dyn Stream<Item = crate::Result<Response<M, F>>> + Send>;
    pub type BoxedFuture<M> = Box<
        dyn Future<
                Output = crate::Result<
                    Response<<M as StatusMonitor>::Output, <M as StatusMonitor>::Format>,
                >,
            > + Send,
    >;
    pub type BoxedCallback<M> = Box<dyn FnOnce() -> Pin<BoxedFuture<M>> + Send>;

    /// A pinned boxed [`Future`] that can be stored and called dynamically.
    pub type PollerResultFuture<M, C, F> =
        Pin<Box<dyn Future<Output = crate::Result<PollerResult<M, C, F>>> + Send + 'static>>;
}

#[cfg(target_arch = "wasm32")]
mod types {
    use super::{PollerResult, Response, StatusMonitor, Stream};
    use std::{future::Future, pin::Pin};

    pub type BoxedStream<M, F> = Box<dyn Stream<Item = crate::Result<Response<M, F>>>>;
    pub type BoxedFuture<M> = Box<
        dyn Future<
            Output = crate::Result<
                Response<<M as StatusMonitor>::Output, <M as StatusMonitor>::Format>,
            >,
        >,
    >;
    pub type BoxedCallback<M> = Box<dyn FnOnce() -> Pin<BoxedFuture<M>>>;

    /// A pinned boxed [`Future`] that can be stored and called dynamically.
    pub type PollerResultFuture<M, C, F> =
        Pin<Box<dyn Future<Output = crate::Result<PollerResult<M, C, F>>> + 'static>>;
}

pub use types::PollerResultFuture;
use types::{BoxedCallback, BoxedFuture, BoxedStream};

/// Represents a long-running operation (LRO)
///
/// A `Poller` implements both [`IntoFuture`] and [`Stream`].
/// You can `await` a `Poller` to get the final model upon successful completion; or,
/// you can call [`next`](StreamExt::next) or [`try_next`](futures::stream::TryStreamExt::try_next) on a mutable `Poller` to poll status manually.
///
/// # Examples
///
/// For clients that return a `Poller`, you can await it to get the final result:
///
/// ```no_run
/// # use azure_core::credentials::TokenCredential;
/// # use azure_security_keyvault_certificates::{CertificateClient, models::CreateCertificateParameters};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let credential: std::sync::Arc<dyn TokenCredential> = unimplemented!();
/// let client = CertificateClient::new(
///     "https://my-vault.vault.azure.net",
///     credential.clone(),
///     None,
/// )?;
///
/// let params = CreateCertificateParameters::default();
///
/// // Await the poller to get the final certificate.
/// let certificate = client
///     .create_certificate("my-cert", params.try_into()?, None)?
///     .await?
///     .into_model()?;
/// # Ok(()) }
/// ```
///
/// If you want to manually poll status updates, you can use the `Poller` as a stream:
///
/// ```no_run
/// # use azure_core::credentials::TokenCredential;
/// # use azure_security_keyvault_certificates::{CertificateClient, models::CreateCertificateParameters};
/// # use futures::TryStreamExt;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let credential: std::sync::Arc<dyn TokenCredential> = unimplemented!();
/// let client = CertificateClient::new(
///     "https://my-vault.vault.azure.net",
///     credential.clone(),
///     None,
/// )?;
///
/// let params = CreateCertificateParameters::default();
///
/// // Manually poll status updates.
/// let mut poller = client
///     .create_certificate("my-cert", params.try_into()?, None)?;
///
/// while let Some(status) = poller.try_next().await? {
///     let status = status.into_model()?;
///     println!("Status: {:?}", status.status);
/// }
///
/// // After the stream ends, await to get the final certificate.
/// let certificate = poller.await?.into_model()?;
/// # Ok(()) }
/// ```
#[pin_project::pin_project]
pub struct Poller<M, F = JsonFormat>
where
    M: StatusMonitor,
    F: Format,
{
    #[pin]
    stream: Pin<BoxedStream<M, F>>,
    target: Option<BoxedFuture<M>>,
}

impl<M, F> Poller<M, F>
where
    M: StatusMonitor,
    F: Format + Send,
{
    /// Creates a [`Poller`] from a callback that will be called repeatedly to monitor a long-running operation (LRO).
    ///
    /// This method expects a callback that accepts a single [`PollerState`] parameter, and returns a [`PollerResult`] value asynchronously.
    /// The `N` type parameter is the type of the next link/continuation token. It may be any [`Send`]able type.
    /// The `M` type parameter must implement [`StatusMonitor`].
    ///
    /// The stream will yield [`Response`] values for each intermediate response while the operation is in progress
    /// i.e., while `M::status()` returns [`PollerStatus::InProgress`]. The stream ends when the operation completes
    /// successfully, fails, or is canceled.
    ///
    /// # Panics
    ///
    /// Panics if [`PollerOptions::frequency`] is less than 1 second.
    ///
    /// # Examples
    ///
    /// To poll a long-running operation:
    ///
    /// ```rust,no_run
    /// # use azure_core::{Result, json, http::{Context, JsonFormat, Pipeline, RawResponse, Request, Response, Method, Url, poller::{Poller, PollerResult, PollerState, PollerStatus, StatusMonitor}}};
    /// # use serde::Deserialize;
    /// # let api_version = "2025-06-04".to_string();
    /// # let pipeline: Pipeline = panic!("Not a runnable example");
    /// #[derive(Deserialize)]
    /// struct OperationResult {
    ///     id: String,
    ///     status: Option<PollerStatus>,
    ///     result: Option<String>,
    /// }
    ///
    /// impl StatusMonitor for OperationResult {
    ///     type Output = OperationResult;
    ///     type Format = JsonFormat;
    ///
    ///     fn status(&self) -> PollerStatus {
    ///         self.status.clone().unwrap_or_default()
    ///     }
    /// }
    ///
    /// let url = "https://example.com/my_operation".parse().unwrap();
    /// let mut req = Request::new(url, Method::Post);
    ///
    /// let poller = Poller::new(move |operation_url: PollerState<Url>,  poller_options| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let api_version = api_version.clone();
    ///     let mut req = req.clone();
    ///     Box::pin(async move {
    ///         if let PollerState::More(operation_url) = operation_url {
    ///             // Use the operation URL for polling
    ///             *req.url_mut() = operation_url;
    ///             req.set_method(Method::Get);
    ///         }
    ///
    ///         req.url_mut()
    ///             .query_pairs_mut()
    ///             .append_pair("api-version", &api_version);
    ///
    ///         let resp = pipeline
    ///             .send(&poller_options.context, &mut req, None)
    ///             .await?;
    ///         let (status, headers, body) = resp.deconstruct();
    ///         let result: OperationResult = json::from_json(&body)?;
    ///         let final_body = body.clone(); // Clone before moving into Response
    ///         let resp: Response<OperationResult> = RawResponse::from_bytes(status, headers, body).into();
    ///
    ///         match result.status() {
    ///             PollerStatus::InProgress => {
    ///                 // Continue polling with the operation URL from the response
    ///                 let operation_url = format!("https://example.com/operations/{}", result.id).parse()?;
    ///                 Ok(PollerResult::InProgress {
    ///                     response: resp,
    ///                     retry_after: poller_options.frequency,
    ///                     continuation_token: operation_url
    ///                 })
    ///             }
    ///             PollerStatus::Succeeded => {
    ///                 // The result is in the operation response; otherwise, get the target URL
    ///                 // from the response headers or body and asynchronously fetch the operation target.
    ///                 Ok(PollerResult::Succeeded {
    ///                     response: resp,
    ///                     target: Box::new(move || {
    ///                         Box::pin(async move {
    ///                             // In this example, the final result is already in the status response
    ///                             // In other cases, you might fetch from a target URL
    ///                             use azure_core::http::headers::Headers;
    ///                             Ok(RawResponse::from_bytes(status, Headers::new(), final_body).into())
    ///                         })
    ///                     }),
    ///                 })
    ///             }
    ///             _ => Ok(PollerResult::Done { response: resp })
    ///         }
    ///     })
    /// }, None);
    /// ```
    pub fn new<C, Fun>(make_request: Fun, options: Option<PollerOptions<'static>>) -> Self
    where
        M: Send + 'static,
        M::Output: Send + 'static,
        M::Format: Send + 'static,
        C: AsRef<str> + ConditionalSend + 'static,
        Fun: Fn(PollerState<C>, PollerOptions<'static>) -> PollerResultFuture<M, C, F>
            + ConditionalSend
            + 'static,
    {
        let options = options.unwrap_or_default();
        let (stream, target) = create_poller_stream(make_request, options);
        Self {
            stream: Box::pin(stream),
            target: Some(target),
        }
    }
}

impl<M, F> Stream for Poller<M, F>
where
    M: StatusMonitor,
    F: Format,
{
    type Item = crate::Result<Response<M, F>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<Self::Item>> {
        let state = self.project().stream.poll_next(cx);
        if let Poll::Ready(Some(Ok(ref response))) = state {
            check_status_code(response)?;
        }

        state
    }
}

impl<M, F> IntoFuture for Poller<M, F>
where
    M: StatusMonitor + 'static,
    M::Output: ConditionalSend + 'static,
    M::Format: ConditionalSend + 'static,
    F: Format + 'static,
{
    type Output = crate::Result<Response<M::Output, M::Format>>;

    #[cfg(not(target_arch = "wasm32"))]
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    #[cfg(target_arch = "wasm32")]
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn into_future(mut self) -> Self::IntoFuture {
        Box::pin(async move {
            // Poll the stream until completion
            while let Some(result) = self.stream.next().await {
                // Check if we got an error from the stream
                result?;
            }

            // Extract the target future
            let target = self.target.ok_or_else(|| {
                crate::Error::new(
                    ErrorKind::Other,
                    "poller completed without a target response",
                )
            })?;

            // Pin and await the target future to get the final response
            Box::into_pin(target).await
        })
    }
}

impl<M, F> fmt::Debug for Poller<M, F>
where
    M: StatusMonitor,
    F: Format,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Poller")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum State<C> {
    Init,
    InProgress(C),
    Done,
}

/// The type of the oneshot channel sender for the target future.
type TargetTransmitterType<'a, M> = (Pin<BoxedFuture<M>>, Option<Context<'a>>);

/// Represents the state used for each iteration through the poller stream.
struct StreamState<'a, M, C, Fun>
where
    M: StatusMonitor,
{
    /// The current polling state (Init, InProgress, or Done)
    state: State<C>,
    /// The callback function to make requests
    make_request: Fun,
    /// Optional channel sender for the target future
    target_tx: Option<oneshot::Sender<TargetTransmitterType<'a, M>>>,
    /// The poller options
    options: PollerOptions<'a>,
    /// Whether a span was added to the context
    added_span: bool,
}

fn create_poller_stream<
    M,
    F: Format,
    C: AsRef<str> + ConditionalSend + 'static,
    Fun: Fn(PollerState<C>, PollerOptions<'static>) -> PollerResultFuture<M, C, F>
        + ConditionalSend
        + 'static,
>(
    make_request: Fun,
    options: PollerOptions<'static>,
) -> (
    impl Stream<Item = crate::Result<Response<M, F>>> + 'static,
    BoxedFuture<M>,
)
where
    M: StatusMonitor + 'static,
    M::Output: ConditionalSend + 'static,
    M::Format: ConditionalSend + 'static,
{
    let (target_tx, target_rx) = oneshot::channel();

    assert!(
        options.frequency >= MIN_RETRY_TIME,
        "minimum polling frequency is 1 second"
    );
    let stream = unfold(
        // We flow the `make_request` callback through the state value to avoid cloning.
        StreamState::<M, C, Fun> {
            state: State::Init,
            make_request,
            target_tx: Some(target_tx),
            options,
            added_span: false,
        },
        move |mut poller_stream_state| async move {
            let result = match poller_stream_state.state {
                State::Init => {
                    // At the very start of polling, create a span for the entire request, and attach it to the context
                    let span =
                        create_public_api_span(&poller_stream_state.options.context, None, None);
                    if let Some(ref s) = span {
                        poller_stream_state.added_span = true;
                        poller_stream_state.options.context =
                            poller_stream_state.options.context.with_value(s.clone());
                    }
                    (poller_stream_state.make_request)(
                        PollerState::Initial,
                        poller_stream_state.options.clone(),
                    )
                    .await
                }
                State::InProgress(n) => {
                    tracing::debug!(
                        "subsequent operation request to {:?}",
                        AsRef::<str>::as_ref(&n)
                    );
                    (poller_stream_state.make_request)(
                        PollerState::More(n),
                        poller_stream_state.options.clone(),
                    )
                    .await
                }
                State::Done => {
                    tracing::debug!("done");
                    return None;
                }
            };
            let (item, next_state) = match result {
                Err(e) => {
                    if poller_stream_state.added_span {
                        if let Some(span) =
                            poller_stream_state.options.context.value::<Arc<dyn Span>>()
                        {
                            // Mark the span as an error with an appropriate description.
                            span.set_status(SpanStatus::Error {
                                description: e.to_string(),
                            });
                            span.set_attribute("error.type", e.kind().to_string().into());
                            span.end();
                        }
                    }

                    poller_stream_state.state = State::Done;
                    return Some((Err(e), poller_stream_state));
                }
                Ok(PollerResult::InProgress {
                    response,
                    retry_after,
                    continuation_token: n,
                }) => {
                    // Note that test-proxy automatically adds a transform that zeroes an existing `after-retry` header during playback, so don't check at runtime:
                    // <https://github.com/Azure/azure-sdk-tools/blob/a80b559d7682891f36a491b73f52fcb679d40923/tools/test-proxy/Azure.Sdk.Tools.TestProxy/RecordingHandler.cs#L1175>
                    tracing::trace!("retry poller in {}s", retry_after.whole_seconds());
                    sleep(retry_after).await;

                    (Ok(response), State::InProgress(n))
                }
                // Note that we will normally never reach this state. The normal progression of the `make_request` callback is to return `Succeeded` with a target future,
                // and then the stream yields the final response and transitions to `Done` state.
                // The only time that the `make_request` callback will normally enter the `Done` state directly is if the LRO fails or is canceled.
                Ok(PollerResult::Done { response }) => (Ok(response), State::Done),
                Ok(PollerResult::Succeeded {
                    response,
                    target: get_target,
                }) => {
                    // Send the target callback through the channel
                    if let Some(tx) = poller_stream_state.target_tx.take() {
                        let _ = tx.send((
                            get_target(),
                            if poller_stream_state.added_span {
                                Some(poller_stream_state.options.context.clone())
                            } else {
                                None
                            },
                        ));
                    }
                    // Also yield the final status response
                    poller_stream_state.state = State::Done;
                    return Some((Ok(response), poller_stream_state));
                }
            };

            // Update state and return
            poller_stream_state.state = next_state;
            Some((item, poller_stream_state))
        },
    );

    let target = Box::new(async move {
        match target_rx.await {
            Ok(target_state) => {
                // Await the target future to get the final response from the poller.
                let res = target_state.0.await;
                // If we added a span to the target, take the result of the final target future to finalize the span.

                if let Some(ctx) = target_state.1 {
                    match &res {
                        Ok(response) => {
                            // When the result is done, finalize the span. Note that we only do that if we created the span in the first place,
                            // otherwise it is the responsibility of the caller to end their span.
                            if let Some(span) = ctx.value::<Arc<dyn Span>>() {
                                // 5xx status codes SHOULD set status to Error.
                                // The description should not be set because it can be inferred from "http.response.status_code".
                                if response.status().is_server_error() {
                                    span.set_status(SpanStatus::Error {
                                        description: "".to_string(),
                                    });
                                }
                                if response.status().is_client_error()
                                    || response.status().is_server_error()
                                {
                                    span.set_attribute(
                                        "error.type",
                                        response.status().to_string().into(),
                                    );
                                }

                                span.end();
                            }
                        }
                        Err(err) => {
                            if let Some(span) = ctx.value::<Arc<dyn Span>>() {
                                span.set_status(SpanStatus::Error {
                                    description: err.to_string(),
                                });
                                span.set_attribute("error.type", err.kind().to_string().into());
                                span.end();
                            }
                        }
                    }
                }
                res
            }
            Err(err) => Err(crate::Error::with_error(
                ErrorKind::Other,
                err,
                "poller completed without defining a target",
            )),
        }
    });

    (stream, target)
}

/// Get the retry duration from the operation response or [`PollerOptions`].
pub fn get_retry_after(
    headers: &Headers,
    retry_headers: &[HeaderName],
    options: &PollerOptions,
) -> Duration {
    #[cfg_attr(feature = "test", allow(unused_mut))]
    let duration =
        crate::http::policies::get_retry_after(headers, OffsetDateTime::now_utc, retry_headers)
            .unwrap_or(options.frequency);

    #[cfg(feature = "test")]
    {
        use crate::test::RecordingMode;

        // Even though test-proxy will zero an existing `after-retry` (or similar proprietary) header during playback,
        // we need to override the frequency for services which do not send back supported headers in their response.
        if matches!(headers.get_optional::<RecordingMode>(), Ok(Some(mode)) if mode == RecordingMode::Playback)
        {
            if duration > Duration::ZERO {
                tracing::debug!(
                    "overriding {}s poller retry in playback",
                    duration.whole_seconds()
                );
            }

            return Duration::ZERO;
        }
    }

    duration
}

fn check_status_code<T, F: Format>(response: &Response<T, F>) -> crate::Result<()> {
    let status = response.status();
    match status {
        StatusCode::Ok | StatusCode::Accepted | StatusCode::Created | StatusCode::NoContent => {
            Ok(())
        }
        _ => {
            // Ideally we could take an owned `Response` and move data to avoid cloning the `RawResponse`.
            let raw_response = Box::new(response.to_raw_response());
            let error_code = F::deserialize(raw_response.body())
                .ok()
                .and_then(|err: ErrorResponse| err.error)
                .and_then(|details| details.code);
            Err(ErrorKind::HttpResponse {
                status,
                error_code,
                raw_response: Some(raw_response),
            }
            .into_error())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "xml")]
    use crate::http::XmlFormat;
    use crate::http::{
        headers::Headers, AsyncRawResponse, HttpClient, Method, NoFormat, RawResponse, Request,
    };
    use azure_core_test::http::MockHttpClient;
    use futures::{FutureExt as _, TryStreamExt as _};
    use std::sync::{Arc, Mutex};

    #[derive(Debug, serde::Deserialize)]
    struct TestStatus {
        status: String,
        #[serde(default)]
        target: Option<String>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct TestOutput {
        #[serde(default)]
        id: Option<String>,
        #[serde(default)]
        name: Option<String>,
    }

    impl StatusMonitor for TestStatus {
        type Output = TestOutput;
        type Format = JsonFormat;

        fn status(&self) -> PollerStatus {
            self.status.parse().unwrap_or_default()
        }
    }

    #[cfg(feature = "xml")]
    #[derive(Debug, serde::Deserialize)]
    struct XmlTestStatus {
        status: String,
    }

    #[cfg(feature = "xml")]
    impl StatusMonitor for XmlTestStatus {
        type Output = TestOutput;
        type Format = XmlFormat;

        fn status(&self) -> PollerStatus {
            self.status.parse().unwrap_or_default()
        }
    }

    #[tokio::test]
    async fn poller_succeeded() {
        let call_count = Arc::new(Mutex::new(0));

        let mock_client = {
            let call_count = call_count.clone();
            Arc::new(MockHttpClient::new(move |_| {
                let call_count = call_count.clone();
                async move {
                    let mut count = call_count.lock().unwrap();
                    *count += 1;

                    if *count == 1 {
                        // First call returns 201 Created with InProgress status
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Created,
                            Headers::new(),
                            br#"{"status":"InProgress"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Succeeded status
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"Succeeded"}"#.to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };

        let mut poller = Poller::new(
            move |_, _| {
                let client = mock_client.clone();
                Box::pin(async move {
                    let req = Request::new("https://example.com".parse().unwrap(), Method::Get);
                    let raw_response = client.execute_request(&req).await?;
                    let (status, headers, body) = raw_response.deconstruct();
                    let bytes = body.collect().await?;

                    let test_status: TestStatus = crate::json::from_json(&bytes)?;
                    let response: Response<TestStatus> =
                        RawResponse::from_bytes(status, headers, bytes).into();

                    match test_status.status() {
                        PollerStatus::InProgress => Ok(PollerResult::InProgress {
                            response,
                            retry_after: Duration::ZERO,
                            continuation_token: "",
                        }),
                        _ => Ok(PollerResult::Done { response }),
                    }
                })
            },
            None,
        );

        // First poll should succeed (201 Created with InProgress)
        let first_result = poller.next().await;
        assert!(first_result.is_some());
        let first_response = first_result.unwrap().unwrap();
        assert_eq!(first_response.status(), StatusCode::Created);
        let first_body = first_response.into_model().unwrap();
        assert_eq!(first_body.status(), PollerStatus::InProgress);

        // Second poll should succeed (200 OK with Succeeded)
        let second_result = poller.next().await;
        assert!(second_result.is_some());
        let second_response = second_result.unwrap().unwrap();
        assert_eq!(second_response.status(), StatusCode::Ok);
        let second_body = second_response.into_model().unwrap();
        assert_eq!(second_body.status(), PollerStatus::Succeeded);

        // Third poll should return None (end of stream)
        let third_result = poller.next().await;
        assert!(third_result.is_none());

        // Verify both calls were made
        assert_eq!(*call_count.lock().unwrap(), 2);
    }

    #[tokio::test]
    async fn poller_failed() {
        let call_count = Arc::new(Mutex::new(0));

        let mock_client = {
            let call_count = call_count.clone();
            Arc::new(MockHttpClient::new(move |_| {
                let call_count = call_count.clone();
                async move {
                    let mut count = call_count.lock().unwrap();
                    *count += 1;

                    if *count == 1 {
                        // First call returns 201 Created with InProgress status
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Created,
                            Headers::new(),
                            br#"{"status":"InProgress"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Failed status
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"Failed"}"#.to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };
        let mut poller = Poller::new(
            move |_, _| {
                let client = mock_client.clone();
                Box::pin(async move {
                    let req = Request::new("https://example.com".parse().unwrap(), Method::Get);
                    let raw_response = client
                        .execute_request(&req)
                        .await?
                        .try_into_raw_response()
                        .await?;
                    let (status, headers, body) = raw_response.deconstruct();

                    let test_status: TestStatus = crate::json::from_json(&body)?;
                    let response: Response<TestStatus> =
                        RawResponse::from_bytes(status, headers, body).into();

                    match test_status.status() {
                        PollerStatus::InProgress => Ok(PollerResult::InProgress {
                            response,
                            retry_after: Duration::ZERO,
                            continuation_token: "",
                        }),
                        _ => Ok(PollerResult::Done { response }),
                    }
                })
            },
            None,
        );

        // First poll should succeed (201 Created with InProgress)
        let first_result = poller.next().await;
        assert!(first_result.is_some());
        let first_response = first_result.unwrap().unwrap();
        assert_eq!(first_response.status(), StatusCode::Created);
        let first_body = first_response.into_model().unwrap();
        assert_eq!(first_body.status(), PollerStatus::InProgress);

        // Second poll should succeed (200 OK with Succeeded)
        let second_result = poller.next().await;
        assert!(second_result.is_some());
        let second_response = second_result.unwrap().unwrap();
        assert_eq!(second_response.status(), StatusCode::Ok);
        let second_body = second_response.into_model().unwrap();
        assert_eq!(second_body.status(), PollerStatus::Failed);

        // Third poll should return None (end of stream)
        let third_result = poller.next().await;
        assert!(third_result.is_none());

        // Verify both calls were made
        assert_eq!(*call_count.lock().unwrap(), 2);
    }

    #[tokio::test]
    async fn poller_failed_with_http_429() {
        let call_count = Arc::new(Mutex::new(0));

        let mock_client = {
            let call_count = call_count.clone();
            Arc::new(MockHttpClient::new(move |_| {
                let call_count = call_count.clone();
                async move {
                    let mut count = call_count.lock().unwrap();
                    *count += 1;

                    if *count == 1 {
                        // First call returns 200 OK with InProgress status
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"InProgress"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 429 Too Many Requests
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::TooManyRequests,
                            Headers::new(),
                            vec![],
                        ))
                    }
                }
                .boxed()
            }))
        };

        let mut poller = Poller::new(
            move |_, _| {
                let client = mock_client.clone();
                Box::pin(async move {
                    let req = Request::new("https://example.com".parse().unwrap(), Method::Get);
                    let raw_response = client
                        .execute_request(&req)
                        .await?
                        .try_into_raw_response()
                        .await?;
                    let (status, headers, body) = raw_response.deconstruct();

                    if status == StatusCode::Ok {
                        let test_status: TestStatus = crate::json::from_json(&body)?;
                        let response: Response<TestStatus> =
                            RawResponse::from_bytes(status, headers, body).into();

                        match test_status.status() {
                            PollerStatus::InProgress => Ok(PollerResult::InProgress {
                                response,
                                retry_after: Duration::ZERO,
                                continuation_token: "",
                            }),
                            _ => Ok(PollerResult::Done { response }),
                        }
                    } else {
                        // Return the error response which should trigger check_status_code
                        let response: Response<TestStatus> =
                            RawResponse::from_bytes(status, headers, body).into();
                        Ok(PollerResult::Done { response })
                    }
                })
            },
            None,
        );

        // First poll should succeed (200 OK with InProgress)
        let first_result = poller.next().await;
        assert!(first_result.is_some());
        assert!(first_result.unwrap().is_ok());

        // Second poll should fail due to 429 status code being rejected by check_status_code
        let second_result = poller.next().await;
        assert!(second_result.is_some());
        let error = second_result.unwrap().unwrap_err();

        // Verify the error is an HttpResponse error with 429 status
        match error.kind() {
            ErrorKind::HttpResponse { status, .. } => {
                assert_eq!(*status, StatusCode::TooManyRequests);
            }
            _ => panic!("Expected HttpResponse error, got {:?}", error.kind()),
        }

        // Verify both calls were made
        assert_eq!(*call_count.lock().unwrap(), 2);
    }

    #[tokio::test]
    async fn poller_into_future_succeeds() {
        let call_count = Arc::new(Mutex::new(0));

        let mock_client = {
            let call_count = call_count.clone();
            Arc::new(MockHttpClient::new(move |_| {
                let call_count = call_count.clone();
                async move {
                    let mut count = call_count.lock().unwrap();
                    *count += 1;

                    if *count == 1 {
                        // First call returns 201 Created with InProgress status
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Created,
                            Headers::new(),
                            br#"{"status":"InProgress"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Succeeded status and final result
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"Succeeded","id":"op1","name":"Operation completed successfully"}"#.to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };

        let poller = Poller::new(
            move |_, _| {
                let client = mock_client.clone();
                Box::pin(async move {
                    let req = Request::new("https://example.com".parse().unwrap(), Method::Get);
                    let raw_response = client.execute_request(&req).await?;
                    let (status, headers, body) = raw_response.deconstruct();
                    let bytes = body.collect().await?;

                    let test_status: TestStatus = crate::json::from_json(&bytes)?;
                    let response: Response<TestStatus> =
                        RawResponse::from_bytes(status, headers.clone(), bytes.clone()).into();

                    match test_status.status() {
                        PollerStatus::InProgress => Ok(PollerResult::InProgress {
                            response,
                            retry_after: Duration::ZERO,
                            continuation_token: "",
                        }),
                        PollerStatus::Succeeded => {
                            // Return the status response with a callback to fetch the final resource
                            Ok(PollerResult::Succeeded {
                                response,
                                target: Box::new(|| {
                                    Box::pin(async {
                                        // In a real scenario, this would fetch the final resource
                                        // For this test, the final status already contains the result
                                        use crate::http::headers::Headers;
                                        let headers = Headers::new();
                                        let bytes = bytes::Bytes::from(
                                            r#"{"id": "op1", "name": "Operation completed successfully"}"#,
                                        );
                                        Ok(RawResponse::from_bytes(StatusCode::Ok, headers, bytes)
                                            .into())
                                    })
                                }),
                            })
                        }
                        _ => Ok(PollerResult::Done { response }),
                    }
                })
            },
            None,
        );

        // Use IntoFuture to await completion
        let result = poller.await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::Ok);
        let output = response.into_model().unwrap();
        assert_eq!(output.id.as_deref(), Some("op1"));
        assert_eq!(
            output.name.as_deref(),
            Some("Operation completed successfully")
        );

        // Verify both calls were made
        assert_eq!(*call_count.lock().unwrap(), 2);
    }

    #[tokio::test]
    async fn poller_into_future_with_target_url() {
        let call_count = Arc::new(Mutex::new(0));

        let mock_client = {
            let call_count = call_count.clone();
            Arc::new(MockHttpClient::new(move |req: &Request| {
                let call_count = call_count.clone();
                let url = req.url().to_string();
                async move {
                    let mut count = call_count.lock().unwrap();
                    *count += 1;

                    if *count == 1 {
                        // First call to operation URL returns InProgress status
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Accepted,
                            Headers::new(),
                            br#"{"status":"InProgress"}"#.to_vec(),
                        ))
                    } else if *count == 2 {
                        // Second call to operation URL returns Succeeded with target URL
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"Succeeded","target":"https://example.com/resources/123"}"#.to_vec(),
                        ))
                    } else {
                        // Third call fetches the final resource from target URL
                        assert_eq!(url, "https://example.com/resources/123");
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"id":"123","name":"Test Resource"}"#.to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };

        let poller = Poller::new(
            move |_, _| {
                let client = mock_client.clone();
                Box::pin(async move {
                    let req = Request::new(
                        "https://example.com/operations/op1".parse().unwrap(),
                        Method::Get,
                    );
                    let raw_response = client.execute_request(&req).await?;
                    let (status, headers, body) = raw_response.deconstruct();
                    let bytes = body.collect().await?;

                    let operation_status: TestStatus = crate::json::from_json(&bytes)?;
                    let response: Response<TestStatus> =
                        RawResponse::from_bytes(status, headers.clone(), bytes.clone()).into();

                    match operation_status.status() {
                        PollerStatus::InProgress => Ok(PollerResult::InProgress {
                            response,
                            retry_after: Duration::ZERO,
                            continuation_token: "",
                        }),
                        PollerStatus::Succeeded => {
                            // Return the status response with a callback to fetch the final resource
                            if let Some(target_url) = operation_status.target {
                                let client_clone = client.clone();
                                Ok(PollerResult::Succeeded {
                                    response,
                                    target: Box::new(move || {
                                        Box::pin(async move {
                                            let target_req = Request::new(
                                                target_url.parse().unwrap(),
                                                Method::Get,
                                            );
                                            let target_response =
                                                client_clone.execute_request(&target_req).await?;
                                            let (target_status, target_headers, target_body) =
                                                target_response.deconstruct();
                                            let target_bytes = target_body.collect().await?;

                                            Ok(RawResponse::from_bytes(
                                                target_status,
                                                target_headers,
                                                target_bytes,
                                            )
                                            .into())
                                        })
                                    }),
                                })
                            } else {
                                Err(crate::Error::new(
                                    ErrorKind::Other,
                                    "no target URL in succeeded response",
                                ))
                            }
                        }
                        _ => Ok(PollerResult::Done { response }),
                    }
                })
            },
            None,
        );

        // Use IntoFuture to await completion
        let result = poller.await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::Ok);
        let resource = response.into_model().unwrap();
        assert_eq!(resource.id.as_deref(), Some("123"));
        assert_eq!(resource.name.as_deref(), Some("Test Resource"));

        // Verify all three calls were made
        assert_eq!(*call_count.lock().unwrap(), 3);
    }

    #[tokio::test]
    async fn poller_into_future_no_response_body() {
        #[derive(Debug, serde::Deserialize)]
        struct NoBodyStatus {
            status: String,
        }

        impl StatusMonitor for NoBodyStatus {
            type Output = ();
            type Format = NoFormat;

            fn status(&self) -> PollerStatus {
                self.status.parse().unwrap_or_default()
            }
        }

        let call_count = Arc::new(Mutex::new(0));

        let mock_client = {
            let call_count = call_count.clone();
            Arc::new(MockHttpClient::new(move |_| {
                let call_count = call_count.clone();
                async move {
                    let mut count = call_count.lock().unwrap();
                    *count += 1;

                    if *count == 1 {
                        // First call returns 202 Accepted with InProgress status
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Accepted,
                            Headers::new(),
                            br#"{"status":"InProgress"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Succeeded status
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"Succeeded"}"#.to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };

        let poller = Poller::new(
            move |_, _| {
                let client = mock_client.clone();
                Box::pin(async move {
                    let req = Request::new("https://example.com".parse().unwrap(), Method::Get);
                    let raw_response = client.execute_request(&req).await?;
                    let (status, headers, body) = raw_response.deconstruct();
                    let bytes = body.collect().await?;

                    let no_body_status: NoBodyStatus = crate::json::from_json(&bytes)?;
                    let response: Response<NoBodyStatus> =
                        RawResponse::from_bytes(status, headers.clone(), bytes.clone()).into();

                    match no_body_status.status() {
                        PollerStatus::InProgress => Ok(PollerResult::InProgress {
                            response,
                            retry_after: Duration::ZERO,
                            continuation_token: "",
                        }),
                        PollerStatus::Succeeded => {
                            // Return the status response with a callback
                            Ok(PollerResult::Succeeded {
                                response,
                                target: Box::new(move || {
                                    Box::pin(async move {
                                        // Return a Response<()> with no body for operations that don't return data
                                        use crate::http::headers::Headers;
                                        let headers = Headers::new();
                                        Ok(RawResponse::from_bytes(status, headers, Vec::new())
                                            .into())
                                    })
                                }),
                            })
                        }
                        _ => Ok(PollerResult::Done { response }),
                    }
                })
            },
            None,
        );

        // Use IntoFuture to await completion
        let result = poller.await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::Ok);
        // For operations with no response body, we don't need to call into_model()
        // The important thing is that the poller completed successfully and returned Response<()>

        // Verify both calls were made
        assert_eq!(*call_count.lock().unwrap(), 2);
    }

    #[cfg(feature = "xml")]
    #[tokio::test]
    async fn poller_succeeded_xml() {
        let call_count = Arc::new(Mutex::new(0));

        let mock_client = {
            let call_count = call_count.clone();
            Arc::new(MockHttpClient::new(move |_| {
                let call_count = call_count.clone();
                async move {
                    let mut count = call_count.lock().unwrap();
                    *count += 1;

                    if *count == 1 {
                        // First call returns 201 Created with InProgress status
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Created,
                            Headers::new(),
                            b"<XmlTestStatus><status>InProgress</status></XmlTestStatus>".to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Succeeded status
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            b"<XmlTestStatus><status>Succeeded</status></XmlTestStatus>".to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };

        let mut poller = Poller::new(
            move |_, _| {
                let client = mock_client.clone();
                Box::pin(async move {
                    let req = Request::new("https://example.com".parse().unwrap(), Method::Get);
                    let raw_response = client.execute_request(&req).await?;
                    let (status, headers, body) = raw_response.deconstruct();
                    let bytes = body.collect().await?;

                    let test_status: XmlTestStatus = crate::xml::from_xml(&bytes)?;
                    let response: Response<XmlTestStatus, XmlFormat> =
                        RawResponse::from_bytes(status, headers, bytes).into();

                    match test_status.status() {
                        PollerStatus::InProgress => Ok(PollerResult::InProgress {
                            response,
                            retry_after: Duration::ZERO,
                            continuation_token: "",
                        }),
                        _ => Ok(PollerResult::Done { response }),
                    }
                })
            },
            None,
        );

        // First poll should succeed (201 Created with InProgress)
        let first_result = poller.next().await;
        assert!(first_result.is_some());
        let first_response = first_result.unwrap().unwrap();
        assert_eq!(first_response.status(), StatusCode::Created);
        let first_body = first_response.into_model().unwrap();
        assert_eq!(first_body.status(), PollerStatus::InProgress);

        // Second poll should succeed (200 OK with Succeeded)
        let second_result = poller.next().await;
        assert!(second_result.is_some());
        let second_response = second_result.unwrap().unwrap();
        assert_eq!(second_response.status(), StatusCode::Ok);
        let second_body = second_response.into_model().unwrap();
        assert_eq!(second_body.status(), PollerStatus::Succeeded);

        // Third poll should return None (end of stream)
        let third_result = poller.next().await;
        assert!(third_result.is_none());

        // Verify both calls were made
        assert_eq!(*call_count.lock().unwrap(), 2);
    }

    #[cfg(feature = "xml")]
    #[tokio::test]
    async fn poller_into_future_succeeds_xml() {
        let call_count = Arc::new(Mutex::new(0));

        let mock_client = {
            let call_count = call_count.clone();
            Arc::new(MockHttpClient::new(move |_| {
                let call_count = call_count.clone();
                async move {
                    let mut count = call_count.lock().unwrap();
                    *count += 1;

                    if *count == 1 {
                        // First call returns 201 Created with InProgress status
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Created,
                            Headers::new(),
                            b"<XmlTestStatus><status>InProgress</status></XmlTestStatus>"
                                .to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Succeeded status and final result
                        // Note: The response contains both status and the final output fields
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            b"<XmlTestStatus><status>Succeeded</status><id>op1</id><name>Operation completed successfully</name></XmlTestStatus>"
                                .to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };

        let poller = Poller::new(
            move |_, _| {
                let client = mock_client.clone();
                Box::pin(async move {
                    let req = Request::new("https://example.com".parse().unwrap(), Method::Get);
                    let raw_response = client.execute_request(&req).await?;
                    let (status, headers, body) = raw_response.deconstruct();
                    let bytes = body.collect().await?;

                    let test_status: XmlTestStatus = crate::xml::from_xml(&bytes)?;
                    let response: Response<XmlTestStatus, XmlFormat> =
                        RawResponse::from_bytes(status, headers.clone(), bytes.clone()).into();

                    match test_status.status() {
                        PollerStatus::InProgress => Ok(PollerResult::InProgress {
                            response,
                            retry_after: Duration::ZERO,
                            continuation_token: "",
                        }),
                        PollerStatus::Succeeded => {
                            // Return the status response with a callback
                            Ok(PollerResult::Succeeded {
                                response,
                                target: Box::new(move || {
                                    Box::pin(async move {
                                        // For XML format, return the final response
                                        let headers = Headers::new();
                                        let bytes = bytes::Bytes::from(
                                            r#"<TestOutput><id>op1</id><name>Operation completed successfully</name></TestOutput>"#,
                                        );
                                        Ok(RawResponse::from_bytes(StatusCode::Ok, headers, bytes)
                                            .into())
                                    })
                                }),
                            })
                        }
                        _ => Ok(PollerResult::Done { response }),
                    }
                })
            },
            None,
        );

        // Use IntoFuture to await completion
        let result = poller.await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::Ok);
        let output = response.into_model().unwrap();
        assert_eq!(output.id.as_deref(), Some("op1"));
        assert_eq!(
            output.name.as_deref(),
            Some("Operation completed successfully")
        );

        // Verify both calls were made
        assert_eq!(*call_count.lock().unwrap(), 2);
    }

    #[tokio::test]
    async fn poller_into_future_output_is_self() {
        // Test case where StatusMonitor::Output is the same type as the status monitor itself
        #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
        struct SelfContainedStatus {
            status: String,
            id: Option<String>,
            result: Option<String>,
        }

        impl StatusMonitor for SelfContainedStatus {
            type Output = Self; // Output is the same type as the monitor
            type Format = JsonFormat;

            fn status(&self) -> PollerStatus {
                self.status.parse().unwrap_or_default()
            }
        }

        let call_count = Arc::new(Mutex::new(0));

        let mock_client = {
            let call_count = call_count.clone();
            Arc::new(MockHttpClient::new(move |_| {
                let call_count = call_count.clone();
                async move {
                    let mut count = call_count.lock().unwrap();
                    *count += 1;

                    if *count == 1 {
                        // First call returns 201 Created with InProgress status
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Created,
                            Headers::new(),
                            br#"{"status":"InProgress","id":"op1"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Succeeded status and final result in the same object
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"Succeeded","id":"op1","result":"Operation completed successfully"}"#.to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };

        let poller = Poller::new(
            move |_, _| {
                let client = mock_client.clone();
                Box::pin(async move {
                    let req = Request::new("https://example.com".parse().unwrap(), Method::Get);
                    let raw_response = client.execute_request(&req).await?;
                    let (status, headers, body) = raw_response.deconstruct();
                    let bytes = body.collect().await?;

                    let self_status: SelfContainedStatus = crate::json::from_json(&bytes)?;
                    let response: Response<SelfContainedStatus> =
                        RawResponse::from_bytes(status, headers.clone(), bytes.clone()).into();

                    match self_status.status() {
                        PollerStatus::InProgress => Ok(PollerResult::InProgress {
                            response,
                            retry_after: Duration::ZERO,
                            continuation_token: "",
                        }),
                        PollerStatus::Succeeded => {
                            // The final result is already in the status response itself
                            // No separate fetch needed - just return the same response in the callback
                            let final_bytes = bytes.clone();
                            Ok(PollerResult::Succeeded {
                                response,
                                target: Box::new(move || {
                                    Box::pin(async move {
                                        // Return the same data - no additional fetch needed
                                        let headers = Headers::new();
                                        Ok(RawResponse::from_bytes(
                                            StatusCode::Ok,
                                            headers,
                                            final_bytes,
                                        )
                                        .into())
                                    })
                                }),
                            })
                        }
                        _ => Ok(PollerResult::Done { response }),
                    }
                })
            },
            None,
        );

        // Use IntoFuture to await completion
        let result = poller.await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::Ok);
        let output = response.into_model().unwrap();
        assert_eq!(output.id.as_deref(), Some("op1"));
        assert_eq!(
            output.result.as_deref(),
            Some("Operation completed successfully")
        );

        // Verify both calls were made
        assert_eq!(*call_count.lock().unwrap(), 2);
    }

    #[tokio::test]
    async fn poller_stream_output_is_self() {
        // Test case where StatusMonitor::Output is the same type as the status monitor itself
        // Used as a stream to monitor progress
        #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
        struct SelfContainedStatus {
            status: String,
            id: Option<String>,
            result: Option<String>,
        }

        impl StatusMonitor for SelfContainedStatus {
            type Output = Self; // Output is the same type as the monitor
            type Format = JsonFormat;

            fn status(&self) -> PollerStatus {
                self.status.parse().unwrap_or_default()
            }
        }

        let call_count = Arc::new(Mutex::new(0));

        let mock_client = {
            let call_count = call_count.clone();
            Arc::new(MockHttpClient::new(move |_| {
                let call_count = call_count.clone();
                async move {
                    let mut count = call_count.lock().unwrap();
                    *count += 1;

                    if *count == 1 {
                        // First call returns 201 Created with InProgress status
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Created,
                            Headers::new(),
                            br#"{"status":"InProgress","id":"op1"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Succeeded status and final result in the same object
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"Succeeded","id":"op1","result":"Operation completed successfully"}"#.to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };

        let mut poller = Poller::new(
            move |_, _| {
                let client = mock_client.clone();
                Box::pin(async move {
                    let req = Request::new("https://example.com".parse().unwrap(), Method::Get);
                    let raw_response = client.execute_request(&req).await?;
                    let (status, headers, body) = raw_response.deconstruct();
                    let bytes = body.collect().await?;

                    let self_status: SelfContainedStatus = crate::json::from_json(&bytes)?;
                    let response: Response<SelfContainedStatus> =
                        RawResponse::from_bytes(status, headers.clone(), bytes.clone()).into();

                    match self_status.status() {
                        PollerStatus::InProgress => Ok(PollerResult::InProgress {
                            response,
                            retry_after: Duration::ZERO,
                            continuation_token: "",
                        }),
                        PollerStatus::Succeeded => {
                            // The final result is already in the status response itself
                            let final_bytes = bytes.clone();
                            Ok(PollerResult::Succeeded {
                                response,
                                target: Box::new(move || {
                                    Box::pin(async move {
                                        use crate::http::headers::Headers;
                                        let headers = Headers::new();
                                        Ok(RawResponse::from_bytes(
                                            StatusCode::Ok,
                                            headers,
                                            final_bytes,
                                        )
                                        .into())
                                    })
                                }),
                            })
                        }
                        _ => Ok(PollerResult::Done { response }),
                    }
                })
            },
            None,
        );

        // Use as a stream to monitor progress
        let mut statuses = Vec::new();
        while let Some(status_response) = poller.try_next().await.unwrap() {
            let status = status_response.into_model().unwrap();
            statuses.push(status);
        }

        // Should have received both InProgress and Succeeded statuses
        assert_eq!(statuses.len(), 2);
        assert_eq!(statuses[0].status, "InProgress");
        assert_eq!(statuses[0].id.as_deref(), Some("op1"));
        assert_eq!(statuses[0].result, None);

        assert_eq!(statuses[1].status, "Succeeded");
        assert_eq!(statuses[1].id.as_deref(), Some("op1"));
        assert_eq!(
            statuses[1].result.as_deref(),
            Some("Operation completed successfully")
        );

        // Verify both calls were made
        assert_eq!(*call_count.lock().unwrap(), 2);
    }
}

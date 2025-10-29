// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types and methods for long-running operations (LROs).

use crate::{
    error::{ErrorKind, ErrorResponse},
    http::{
        headers::{HeaderName, Headers},
        Format, JsonFormat, Response, StatusCode,
    },
    sleep,
    time::{Duration, OffsetDateTime},
};
use futures::{stream::unfold, Stream, StreamExt};
use serde::Deserialize;
use std::{
    convert::Infallible,
    fmt,
    future::{Future, IntoFuture},
    pin::Pin,
    str::FromStr,
    task::{Context, Poll},
};

/// Default retry time for long-running operations if no retry-after header is present
///
/// This value is the same as the default used in other Azure SDKs e.g.,
/// <https://github.com/Azure/azure-sdk-for-python/blob/azure-core_1.35.0/sdk/core/azure-core/azure/core/polling/base_polling.py#L586>
const DEFAULT_RETRY_TIME: Duration = Duration::seconds(30);
const MIN_RETRY_TIME: Duration = Duration::seconds(1);

/// Represents the state of a [`Poller`].
#[derive(Debug, Default, PartialEq, Eq)]
pub enum PollerState<N> {
    /// The poller should fetch the initial status.
    #[default]
    Initial,
    /// The poller should fetch subsequent status.
    More(N),
}

impl<N> PollerState<N> {
    /// Maps a [`PollerState<N>`] to a [`PollerState<U>`] by applying a function to a next link `N` (if `PollerState::More`) or returns `PollerState::Initial` (if `PollerState::Initial`).
    #[inline]
    pub fn map<U, F>(self, f: F) -> PollerState<U>
    where
        F: FnOnce(N) -> U,
    {
        match self {
            PollerState::Initial => PollerState::Initial,
            PollerState::More(c) => PollerState::More(f(c)),
        }
    }
}

impl<N: Clone> Clone for PollerState<N> {
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
#[derive(Debug, Clone, Copy)]
pub struct PollerOptions {
    /// The time to wait between polling intervals in absence of a `retry-after` header.
    ///
    /// The default is 30 seconds. The minimum time enforced by [`Poller::from_callback`] is 1 second.
    pub frequency: Option<Duration>,
}

impl Default for PollerOptions {
    fn default() -> Self {
        Self {
            frequency: Some(DEFAULT_RETRY_TIME),
        }
    }
}

/// The result of fetching the status monitor from a [`Poller`], whether the long-running operation (LRO) is in progress or done.
pub enum PollerResult<M: StatusMonitor, N, F: Format = JsonFormat> {
    /// The long-running operation (LRO) is in progress and the next status monitor update may be fetched from `next`.
    ///
    /// # Fields
    ///
    /// * `response` contains the HTTP response with the status monitor.
    /// * `retry_after` is the optional client-specified [`Duration`] to wait. The default is 30 seconds.
    /// * `next` is the next link / continuation token.
    InProgress {
        /// The HTTP response with the status monitor.
        response: Response<M, F>,
        /// The optional client-specified [`Duration`] to wait before polling again.
        retry_after: Option<Duration>,
        /// The next link / continuation token.
        next: N,
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

impl<M: StatusMonitor, N: fmt::Debug, F: Format> fmt::Debug for PollerResult<M, N, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InProgress {
                retry_after, next, ..
            } => f
                .debug_struct("InProgress")
                .field("retry_after", &retry_after)
                .field("next", &next)
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
    #[cfg(not(target_arch = "wasm32"))]
    type Format: Format + Send;

    /// The format used to deserialize the `Output`.
    ///
    /// Set this to [`NoFormat`](crate::http::NoFormat) if no final resource is expected.
    #[cfg(target_arch = "wasm32")]
    type Format: Format;

    /// Gets the [`PollerStatus`] from the status monitor.
    fn status(&self) -> PollerStatus;
}

#[cfg(not(target_arch = "wasm32"))]
type BoxedStream<M, F> = Box<dyn Stream<Item = crate::Result<Response<M, F>>> + Send>;

#[cfg(target_arch = "wasm32")]
type BoxedStream<M, F> = Box<dyn Stream<Item = crate::Result<Response<M, F>>>>;

#[cfg(not(target_arch = "wasm32"))]
type BoxedFuture<M> = Box<
    dyn Future<
            Output = crate::Result<
                Response<<M as StatusMonitor>::Output, <M as StatusMonitor>::Format>,
            >,
        > + Send,
>;

#[cfg(target_arch = "wasm32")]
type BoxedFuture<M> = Box<
    dyn Future<
        Output = crate::Result<
            Response<<M as StatusMonitor>::Output, <M as StatusMonitor>::Format>,
        >,
    >,
>;

#[cfg(not(target_arch = "wasm32"))]
type BoxedCallback<M> = Box<dyn FnOnce() -> Pin<BoxedFuture<M>> + Send>;

#[cfg(target_arch = "wasm32")]
type BoxedCallback<M> = Box<dyn FnOnce() -> Pin<BoxedFuture<M>>>;

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
///     .into_body()?;
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
///     let status = status.into_body()?;
///     println!("Status: {:?}", status.status);
/// }
///
/// // After the stream ends, await to get the final certificate.
/// let certificate = poller.await?.into_body()?;
/// # Ok(()) }
/// ```
#[pin_project::pin_project]
pub struct Poller<M, F: Format = JsonFormat>
where
    M: StatusMonitor,
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
    /// Creates a [`Poller<M>`] from a callback that will be called repeatedly to monitor a long-running operation (LRO).
    ///
    /// This method expects a callback that accepts a single [`PollerState<N>`] parameter, and returns a [`PollerResult<M, N>`] value asynchronously.
    /// The `N` type parameter is the type of the next link/continuation token. It may be any [`Send`]able type.
    /// The `M` type parameter must implement [`StatusMonitor`].
    ///
    /// The stream will yield [`Response<M>`] values for each intermediate response while the operation is in progress
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
    /// let poller = Poller::from_callback(move |operation_url: PollerState<Url>| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let api_version = api_version.clone();
    ///     let mut req = req.clone();
    ///     async move {
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
    ///             .send(&Context::new(), &mut req, None)
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
    ///                     retry_after: None,
    ///                     next: operation_url
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
    ///     }
    /// }, None);
    /// ```
    pub fn from_callback<
        #[cfg(not(target_arch = "wasm32"))] N: Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] Fun: Fn(PollerState<N>) -> Fut + Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = crate::Result<PollerResult<M, N, F>>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] N: 'static,
        #[cfg(target_arch = "wasm32")] Fun: Fn(PollerState<N>) -> Fut + 'static,
        #[cfg(target_arch = "wasm32")] Fut: Future<Output = crate::Result<PollerResult<M, N, F>>> + 'static,
    >(
        make_request: Fun,
        options: Option<PollerOptions>,
    ) -> Self
    where
        M: Send + 'static,
        M::Output: Send + 'static,
        M::Format: Send + 'static,
    {
        let (stream, target) = create_poller_stream(make_request, options);
        Self {
            stream: Box::pin(stream),
            target: Some(target),
        }
    }

    /// Creates a [`Poller<M>`] from a raw stream of [`Result<Response<M>>`](crate::Result<Response<M>>) values.
    ///
    /// # Polling frequency
    ///
    /// Streams should take into consideration the polling frequency and retries.
    /// [`Poller::from_callback`] takes a [`PollerOptions::frequency`] that it uses to compute the frequency,
    /// also taking into account any `retry-after` header.
    pub fn from_stream<
        // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
        #[cfg(not(target_arch = "wasm32"))] S: Stream<Item = crate::Result<Response<M, F>>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] S: Stream<Item = crate::Result<Response<M, F>>> + 'static,
    >(
        stream: S,
    ) -> Self {
        Self {
            stream: Box::pin(stream),
            target: None,
        }
    }
}

impl<M, F: Format> Stream for Poller<M, F>
where
    M: StatusMonitor,
{
    type Item = crate::Result<Response<M, F>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let state = self.project().stream.poll_next(cx);
        if let Poll::Ready(Some(Ok(ref response))) = state {
            check_status_code(response)?;
        }

        state
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<M, F: Format + 'static> IntoFuture for Poller<M, F>
where
    M: StatusMonitor + 'static,
    M::Output: Send + 'static,
    M::Format: Send + 'static,
{
    type Output = crate::Result<Response<M::Output, M::Format>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

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

#[cfg(target_arch = "wasm32")]
impl<M> IntoFuture for Poller<M>
where
    M: StatusMonitor + 'static,
    M::Output: 'static,
    M::Format: 'static,
{
    type Output = crate::Result<Response<M::Output, M::Format>>;
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

impl<M: StatusMonitor, F: Format> fmt::Debug for Poller<M, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Poller")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum State<N> {
    Init,
    InProgress(N),
    Done,
}

fn create_poller_stream<
    M,
    F: Format,
    #[cfg(not(target_arch = "wasm32"))] N: Send + 'static,
    #[cfg(not(target_arch = "wasm32"))] Fun: Fn(PollerState<N>) -> Fut + Send + 'static,
    #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = crate::Result<PollerResult<M, N, F>>> + Send + 'static,
    #[cfg(target_arch = "wasm32")] N: 'static,
    #[cfg(target_arch = "wasm32")] Fun: Fn(PollerState<N>) -> Fut + 'static,
    #[cfg(target_arch = "wasm32")] Fut: Future<Output = crate::Result<PollerResult<M, N, F>>> + 'static,
>(
    make_request: Fun,
    options: Option<PollerOptions>,
) -> (
    impl Stream<Item = crate::Result<Response<M, F>>> + 'static,
    BoxedFuture<M>,
)
where
    M: StatusMonitor + 'static,
    M::Output: Send + 'static,
    M::Format: Send + 'static,
{
    use futures::channel::oneshot;

    let (target_tx, target_rx) = oneshot::channel();
    let frequency = options
        .unwrap_or_default()
        .frequency
        .unwrap_or(DEFAULT_RETRY_TIME);
    assert!(
        frequency >= MIN_RETRY_TIME,
        "minimum polling frequency is 1 second"
    );

    let stream = unfold(
        // We flow the `make_request` callback through the state value to avoid cloning.
        (State::Init, make_request, Some(target_tx)),
        move |(state, make_request, target_tx)| async move {
            let result = match state {
                State::Init => make_request(PollerState::Initial).await,
                State::InProgress(n) => make_request(PollerState::More(n)).await,
                State::Done => return None,
            };
            let (item, next_state) = match result {
                Err(e) => return Some((Err(e), (State::Done, make_request, target_tx))),
                Ok(PollerResult::InProgress {
                    response,
                    retry_after,
                    next: n,
                }) => {
                    // Note that test-proxy automatically adds a transform that zeroes an existing `after-retry` header during playback, so don't check at runtime:
                    // <https://github.com/Azure/azure-sdk-tools/blob/a80b559d7682891f36a491b73f52fcb679d40923/tools/test-proxy/Azure.Sdk.Tools.TestProxy/RecordingHandler.cs#L1175>
                    let duration = retry_after.unwrap_or(frequency);

                    tracing::trace!("retry poller in {}s", duration.whole_seconds());
                    sleep(duration).await;

                    (Ok(response), State::InProgress(n))
                }
                Ok(PollerResult::Done { response }) => (Ok(response), State::Done),
                Ok(PollerResult::Succeeded {
                    response,
                    target: get_target,
                }) => {
                    // Send the target callback through the channel
                    if let Some(tx) = target_tx {
                        let _ = tx.send(get_target());
                    }
                    // Also yield the final status response and set target_tx to None since we consumed it
                    return Some((Ok(response), (State::Done, make_request, None)));
                }
            };

            // Flow 'make_request' and target_tx through to avoid cloning
            Some((item, (next_state, make_request, target_tx)))
        },
    );

    let target = Box::new(async move {
        match target_rx.await {
            Ok(fut) => fut.await,
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
) -> Option<Duration> {
    #[cfg_attr(feature = "test", allow(unused_mut))]
    let duration =
        crate::http::policies::get_retry_after(headers, OffsetDateTime::now_utc, retry_headers)
            .or(options.frequency);

    #[cfg(feature = "test")]
    {
        use crate::test::RecordingMode;

        // Even though test-proxy will zero an existing `after-retry` (or similar proprietary) header during playback,
        // we need to override the frequency for services which do not send back supported headers in their response.
        if matches!(headers.get_optional::<RecordingMode>(), Ok(Some(mode)) if mode == RecordingMode::Playback)
        {
            match duration {
                Some(duration) if duration > Duration::ZERO => {
                    tracing::debug!(
                        "overriding {}s poller retry in playback",
                        duration.whole_seconds()
                    );
                }
                _ => {}
            }

            return Some(Duration::ZERO);
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
        headers::Headers, BufResponse, HttpClient, Method, NoFormat, RawResponse, Request,
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
                        Ok(BufResponse::from_bytes(
                            StatusCode::Created,
                            Headers::new(),
                            br#"{"status":"InProgress"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Succeeded status
                        Ok(BufResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"Succeeded"}"#.to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };

        let mut poller = Poller::from_callback(
            move |_| {
                let client = mock_client.clone();
                async move {
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
                            retry_after: Some(Duration::ZERO),
                            next: (),
                        }),
                        _ => Ok(PollerResult::Done { response }),
                    }
                }
            },
            None,
        );

        // First poll should succeed (201 Created with InProgress)
        let first_result = poller.next().await;
        assert!(first_result.is_some());
        let first_response = first_result.unwrap().unwrap();
        assert_eq!(first_response.status(), StatusCode::Created);
        let first_body = first_response.into_body().unwrap();
        assert_eq!(first_body.status(), PollerStatus::InProgress);

        // Second poll should succeed (200 OK with Succeeded)
        let second_result = poller.next().await;
        assert!(second_result.is_some());
        let second_response = second_result.unwrap().unwrap();
        assert_eq!(second_response.status(), StatusCode::Ok);
        let second_body = second_response.into_body().unwrap();
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
                        Ok(BufResponse::from_bytes(
                            StatusCode::Created,
                            Headers::new(),
                            br#"{"status":"InProgress"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Failed status
                        Ok(BufResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"Failed"}"#.to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };

        let mut poller = Poller::from_callback(
            move |_| {
                let client = mock_client.clone();
                async move {
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
                            retry_after: Some(Duration::ZERO),
                            next: (),
                        }),
                        _ => Ok(PollerResult::Done { response }),
                    }
                }
            },
            None,
        );

        // First poll should succeed (201 Created with InProgress)
        let first_result = poller.next().await;
        assert!(first_result.is_some());
        let first_response = first_result.unwrap().unwrap();
        assert_eq!(first_response.status(), StatusCode::Created);
        let first_body = first_response.into_body().unwrap();
        assert_eq!(first_body.status(), PollerStatus::InProgress);

        // Second poll should succeed (200 OK with Succeeded)
        let second_result = poller.next().await;
        assert!(second_result.is_some());
        let second_response = second_result.unwrap().unwrap();
        assert_eq!(second_response.status(), StatusCode::Ok);
        let second_body = second_response.into_body().unwrap();
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
                        Ok(BufResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"InProgress"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 429 Too Many Requests
                        Ok(BufResponse::from_bytes(
                            StatusCode::TooManyRequests,
                            Headers::new(),
                            vec![],
                        ))
                    }
                }
                .boxed()
            }))
        };

        let mut poller = Poller::from_callback(
            move |_| {
                let client = mock_client.clone();
                async move {
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
                                retry_after: Some(Duration::ZERO),
                                next: (),
                            }),
                            _ => Ok(PollerResult::Done { response }),
                        }
                    } else {
                        // Return the error response which should trigger check_status_code
                        let response: Response<TestStatus> =
                            RawResponse::from_bytes(status, headers, body).into();
                        Ok(PollerResult::Done { response })
                    }
                }
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
                        Ok(BufResponse::from_bytes(
                            StatusCode::Created,
                            Headers::new(),
                            br#"{"status":"InProgress"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Succeeded status and final result
                        Ok(BufResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"Succeeded","id":"op1","name":"Operation completed successfully"}"#.to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };

        let poller = Poller::from_callback(
            move |_| {
                let client = mock_client.clone();
                async move {
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
                            retry_after: Some(Duration::ZERO),
                            next: (),
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
                }
            },
            None,
        );

        // Use IntoFuture to await completion
        let result = poller.await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::Ok);
        let output = response.into_body().unwrap();
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
                        Ok(BufResponse::from_bytes(
                            StatusCode::Accepted,
                            Headers::new(),
                            br#"{"status":"InProgress"}"#.to_vec(),
                        ))
                    } else if *count == 2 {
                        // Second call to operation URL returns Succeeded with target URL
                        Ok(BufResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"Succeeded","target":"https://example.com/resources/123"}"#.to_vec(),
                        ))
                    } else {
                        // Third call fetches the final resource from target URL
                        assert_eq!(url, "https://example.com/resources/123");
                        Ok(BufResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"id":"123","name":"Test Resource"}"#.to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };

        let poller = Poller::from_callback(
            move |_| {
                let client = mock_client.clone();
                async move {
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
                            retry_after: Some(Duration::ZERO),
                            next: (),
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
                }
            },
            None,
        );

        // Use IntoFuture to await completion
        let result = poller.await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::Ok);
        let resource = response.into_body().unwrap();
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
                        Ok(BufResponse::from_bytes(
                            StatusCode::Accepted,
                            Headers::new(),
                            br#"{"status":"InProgress"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Succeeded status
                        Ok(BufResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"Succeeded"}"#.to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };

        let poller = Poller::from_callback(
            move |_| {
                let client = mock_client.clone();
                async move {
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
                            retry_after: Some(Duration::ZERO),
                            next: (),
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
                }
            },
            None,
        );

        // Use IntoFuture to await completion
        let result = poller.await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::Ok);
        // For operations with no response body, we don't need to call into_body()
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
                        Ok(BufResponse::from_bytes(
                            StatusCode::Created,
                            Headers::new(),
                            b"<XmlTestStatus><status>InProgress</status></XmlTestStatus>".to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Succeeded status
                        Ok(BufResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            b"<XmlTestStatus><status>Succeeded</status></XmlTestStatus>".to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };

        let mut poller = Poller::from_callback(
            move |_| {
                let client = mock_client.clone();
                async move {
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
                            retry_after: Some(Duration::ZERO),
                            next: (),
                        }),
                        _ => Ok(PollerResult::Done { response }),
                    }
                }
            },
            None,
        );

        // First poll should succeed (201 Created with InProgress)
        let first_result = poller.next().await;
        assert!(first_result.is_some());
        let first_response = first_result.unwrap().unwrap();
        assert_eq!(first_response.status(), StatusCode::Created);
        let first_body = first_response.into_body().unwrap();
        assert_eq!(first_body.status(), PollerStatus::InProgress);

        // Second poll should succeed (200 OK with Succeeded)
        let second_result = poller.next().await;
        assert!(second_result.is_some());
        let second_response = second_result.unwrap().unwrap();
        assert_eq!(second_response.status(), StatusCode::Ok);
        let second_body = second_response.into_body().unwrap();
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
                        Ok(BufResponse::from_bytes(
                            StatusCode::Created,
                            Headers::new(),
                            b"<XmlTestStatus><status>InProgress</status></XmlTestStatus>"
                                .to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Succeeded status and final result
                        // Note: The response contains both status and the final output fields
                        Ok(BufResponse::from_bytes(
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

        let poller = Poller::from_callback(
            move |_| {
                let client = mock_client.clone();
                async move {
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
                            retry_after: Some(Duration::ZERO),
                            next: (),
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
                }
            },
            None,
        );

        // Use IntoFuture to await completion
        let result = poller.await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::Ok);
        let output = response.into_body().unwrap();
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
                        Ok(BufResponse::from_bytes(
                            StatusCode::Created,
                            Headers::new(),
                            br#"{"status":"InProgress","id":"op1"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Succeeded status and final result in the same object
                        Ok(BufResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"Succeeded","id":"op1","result":"Operation completed successfully"}"#.to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };

        let poller = Poller::from_callback(
            move |_| {
                let client = mock_client.clone();
                async move {
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
                            retry_after: Some(Duration::ZERO),
                            next: (),
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
                }
            },
            None,
        );

        // Use IntoFuture to await completion
        let result = poller.await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::Ok);
        let output = response.into_body().unwrap();
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
                        Ok(BufResponse::from_bytes(
                            StatusCode::Created,
                            Headers::new(),
                            br#"{"status":"InProgress","id":"op1"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Succeeded status and final result in the same object
                        Ok(BufResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"Succeeded","id":"op1","result":"Operation completed successfully"}"#.to_vec(),
                        ))
                    }
                }
                .boxed()
            }))
        };

        let mut poller = Poller::from_callback(
            move |_| {
                let client = mock_client.clone();
                async move {
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
                            retry_after: Some(Duration::ZERO),
                            next: (),
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
                }
            },
            None,
        );

        // Use as a stream to monitor progress
        let mut statuses = Vec::new();
        while let Some(status_response) = poller.try_next().await.unwrap() {
            let status = status_response.into_body().unwrap();
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

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types and methods for long-running operations (LROs).

use crate::{
    error::ErrorKind,
    http::{headers::Headers, Format, Response, StatusCode},
    sleep,
    time::{Duration, OffsetDateTime},
};
use futures::{stream::unfold, Stream, StreamExt};
use serde::Deserialize;
use std::{
    convert::Infallible,
    fmt,
    future::Future,
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
pub enum PollerResult<M, N> {
    /// The long-running operation (LRO) is in progress and the next status monitor update may be fetched from `next`.
    ///
    /// # Fields
    ///
    /// * `response` contains the HTTP response with the status monitor.
    /// * `retry_after` is the optional client-specified [`Duration`] to wait. The default is 30 seconds.
    /// * `next` is the next link / continuation token.
    InProgress {
        response: Response<M>,
        retry_after: Option<Duration>,
        next: N,
    },

    /// The long-running operation (LRO) succeeded and contains the final output.
    ///
    /// # Fields
    ///
    /// * `response` contains the HTTP response with the status monitor in a terminal state.
    Done { response: Response<M> },
}

impl<M: StatusMonitor, N: fmt::Debug> fmt::Debug for PollerResult<M, N> {
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
        }
    }
}

/// Represents a status monitor for a long-running operation (LRO).
pub trait StatusMonitor {
    /// The model type returned after the long-running operation (LRO) has completed successfully.
    ///
    /// Set this to the unit type `()` if no final resource is expected.
    type Output;

    /// Gets the [`PollerStatus`] from the status monitor.
    fn status(&self) -> PollerStatus;
}

#[cfg(not(target_arch = "wasm32"))]
type BoxedStream<M> = Box<dyn Stream<Item = crate::Result<Response<M>>> + Send>;

#[cfg(target_arch = "wasm32")]
type BoxedStream<M> = Box<dyn Stream<Item = crate::Result<Response<M>>>>;

/// Represents a long-running operation (LRO)
#[pin_project::pin_project]
pub struct Poller<M> {
    #[pin]
    stream: Pin<BoxedStream<M>>,
    frequency: Duration,
}

impl<M> Poller<M>
where
    M: StatusMonitor,
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
    /// ## Panics
    ///
    /// Panics if [`PollerOptions::frequency`] is less than 1 second.
    ///
    /// ## Examples
    ///
    /// To poll a long-running operation:
    ///
    /// ```rust,no_run
    /// # use azure_core::{Result, http::{Context, Pipeline, RawResponse, Request, Response, Method, Url, poller::{Poller, PollerResult, PollerState, PollerStatus, StatusMonitor}}, json};
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
    ///             .send(&Context::new(), &mut req)
    ///             .await?;
    ///         let (status, headers, body) = resp.deconstruct();
    ///         let bytes = body.collect().await?;
    ///         let result: OperationResult = json::from_json(&bytes)?;
    ///         let resp: Response<OperationResult> = RawResponse::from_bytes(status, headers, bytes).into();
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
    ///             _ => {
    ///                 Ok(PollerResult::Done { response: resp })
    ///             }
    ///         }
    ///     }
    /// }, None);
    /// ```
    pub fn from_callback<
        #[cfg(not(target_arch = "wasm32"))] N: Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] F: Fn(PollerState<N>) -> Fut + Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = crate::Result<PollerResult<M, N>>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] N: 'static,
        #[cfg(target_arch = "wasm32")] F: Fn(PollerState<N>) -> Fut + 'static,
        #[cfg(target_arch = "wasm32")] Fut: Future<Output = crate::Result<PollerResult<M, N>>> + 'static,
    >(
        make_request: F,
        options: Option<PollerOptions>,
    ) -> Self
    where
        M: Send + 'static,
    {
        Self::from_stream(create_poller_stream(make_request), options)
    }

    /// Creates a [`Poller<M>`] from a raw stream of [`Result<Response<M>>`](crate::Result<Response<M>>) values.
    ///
    /// ## Panics
    ///
    /// Panics if [`PollerOptions::frequency`] is less than 1 second.
    ///
    pub fn from_stream<
        // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
        #[cfg(not(target_arch = "wasm32"))] S: Stream<Item = crate::Result<Response<M>>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] S: Stream<Item = crate::Result<Response<M>>> + 'static,
    >(
        stream: S,
        options: Option<PollerOptions>,
    ) -> Self {
        let frequency = options
            .unwrap_or_default()
            .frequency
            .unwrap_or(DEFAULT_RETRY_TIME);
        assert!(
            frequency >= MIN_RETRY_TIME,
            "minimum polling frequency is 1 second"
        );

        Self {
            stream: Box::pin(stream),
            frequency,
        }
    }

    /// Asynchronously waits until the [`Poller<M>`] reaches a terminal state.
    ///
    /// For the terminal state [`PollerStatus::Succeeded`], the final [`Response<M>`] is returned;
    /// otherwise, any other terminal state will return an [`Error`](crate::Error).
    pub async fn wait(&mut self) -> crate::Result<Response<M>> {
        use crate::Result;

        #[pin_project::pin_project]
        struct Last<M>
        where
            M: StatusMonitor,
        {
            monitor: Option<Response<M>>,
        }

        impl<M: StatusMonitor> futures::Sink<Response<M>> for Last<M> {
            type Error = crate::Error;

            fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
                Poll::Ready(Ok(()))
            }

            fn start_send(self: Pin<&mut Self>, item: Response<M>) -> Result<()> {
                let pinned = self.project();
                *pinned.monitor = Some(item);

                Ok(())
            }

            fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
                Poll::Ready(Ok(()))
            }

            fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
                Poll::Ready(Ok(()))
            }
        }

        let mut last = Last { monitor: None };
        self.forward(&mut last).await?;

        last.monitor
            .ok_or_else(|| crate::Error::new(ErrorKind::Other, "end of stream"))
    }
}

impl<M> Stream for Poller<M>
where
    M: StatusMonitor,
{
    type Item = crate::Result<Response<M>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let state = self.project().stream.poll_next(cx);
        if let Poll::Ready(Some(Ok(ref response))) = state {
            check_status_code(response)?;
        }

        state
    }
}

impl<T> fmt::Debug for Poller<T> {
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
    #[cfg(not(target_arch = "wasm32"))] N: Send + 'static,
    #[cfg(not(target_arch = "wasm32"))] F: Fn(PollerState<N>) -> Fut + Send + 'static,
    #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = crate::Result<PollerResult<M, N>>> + Send + 'static,
    #[cfg(target_arch = "wasm32")] N: 'static,
    #[cfg(target_arch = "wasm32")] F: Fn(PollerState<N>) -> Fut + 'static,
    #[cfg(target_arch = "wasm32")] Fut: Future<Output = crate::Result<PollerResult<M, N>>> + 'static,
>(
    make_request: F,
) -> impl Stream<Item = crate::Result<Response<M>>> + 'static
where
    M: StatusMonitor + 'static,
{
    unfold(
        // We flow the `make_request` callback through the state value to avoid cloning.
        (State::Init, make_request),
        |(state, make_request)| async move {
            let result = match state {
                State::Init => make_request(PollerState::Initial).await,
                State::InProgress(n) => make_request(PollerState::More(n)).await,
                State::Done => return None,
            };
            let (item, next_state) = match result {
                Err(e) => return Some((Err(e), (State::Done, make_request))),
                Ok(PollerResult::InProgress {
                    response,
                    retry_after,
                    next: n,
                }) => {
                    // Note that test-proxy automatically adds a transform that zeroes an existing `after-retry` header during playback, so don't check at runtime:
                    // <https://github.com/Azure/azure-sdk-tools/blob/a80b559d7682891f36a491b73f52fcb679d40923/tools/test-proxy/Azure.Sdk.Tools.TestProxy/RecordingHandler.cs#L1175>
                    let duration = retry_after.unwrap_or(DEFAULT_RETRY_TIME);

                    tracing::trace!("retry poller in {}s", duration.whole_seconds());
                    sleep(duration).await;

                    (Ok(response), State::InProgress(n))
                }
                Ok(PollerResult::Done { response }) => (Ok(response), State::Done),
            };

            // Flow 'make_request' through to avoid cloning
            Some((item, (next_state, make_request)))
        },
    )
}

/// Get the retry duration from the operation response or [`PollerOptions`].
pub fn get_retry_after(headers: &Headers, options: &PollerOptions) -> Option<Duration> {
    #[cfg_attr(feature = "test", allow(unused_mut))]
    let duration = crate::http::policies::get_retry_after(headers, OffsetDateTime::now_utc)
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

#[inline]
fn check_status_code<T, F: Format>(response: &Response<T, F>) -> crate::Result<()> {
    let status = response.status();
    match status {
        StatusCode::Ok | StatusCode::Accepted | StatusCode::Created | StatusCode::NoContent => {
            Ok(())
        }
        _ => Err(ErrorKind::HttpResponse {
            status,
            error_code: None,
        }
        .into_error()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{headers::Headers, HttpClient, Method, RawResponse, Request};
    use azure_core_test::http::MockHttpClient;
    use futures::FutureExt as _;
    use std::sync::{Arc, Mutex};

    #[derive(Debug, serde::Deserialize)]
    struct TestStatus {
        status: String,
    }

    impl StatusMonitor for TestStatus {
        type Output = TestStatus;

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
                        Ok(RawResponse::from_bytes(
                            StatusCode::Created,
                            Headers::new(),
                            br#"{"status":"InProgress"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Succeeded status
                        Ok(RawResponse::from_bytes(
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
        let first_body = first_response.into_body().await.unwrap();
        assert_eq!(first_body.status(), PollerStatus::InProgress);

        // Second poll should succeed (200 OK with Succeeded)
        let second_result = poller.next().await;
        assert!(second_result.is_some());
        let second_response = second_result.unwrap().unwrap();
        assert_eq!(second_response.status(), StatusCode::Ok);
        let second_body = second_response.into_body().await.unwrap();
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
                        Ok(RawResponse::from_bytes(
                            StatusCode::Created,
                            Headers::new(),
                            br#"{"status":"InProgress"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 200 OK with Failed status
                        Ok(RawResponse::from_bytes(
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
        let first_body = first_response.into_body().await.unwrap();
        assert_eq!(first_body.status(), PollerStatus::InProgress);

        // Second poll should succeed (200 OK with Succeeded)
        let second_result = poller.next().await;
        assert!(second_result.is_some());
        let second_response = second_result.unwrap().unwrap();
        assert_eq!(second_response.status(), StatusCode::Ok);
        let second_body = second_response.into_body().await.unwrap();
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
                        Ok(RawResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            br#"{"status":"InProgress"}"#.to_vec(),
                        ))
                    } else {
                        // Second call returns 429 Too Many Requests
                        Ok(RawResponse::from_bytes(
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
                    let raw_response = client.execute_request(&req).await?;
                    let (status, headers, body) = raw_response.deconstruct();
                    let bytes = body.collect().await?;

                    if status == StatusCode::Ok {
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
                    } else {
                        // Return the error response which should trigger check_status_code
                        let response: Response<TestStatus> =
                            RawResponse::from_bytes(status, headers, bytes).into();
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
}

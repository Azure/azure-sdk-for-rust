// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types and methods for long-running operations (LROs).

use crate::{
    error::ErrorKind,
    http::{headers::Headers, Response},
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
/// This value is the same as the default used in the Azure SDK for Python:
/// <https://github.com/Azure/azure-sdk-for-python/blob/azure-mgmt-core_1.5.0/sdk/core/azure-mgmt-core/azure/mgmt/core/polling/arm_polling.py#L191>
const DEFAULT_RETRY_TIME: Duration = Duration::seconds(30);

/// long-running operation (LRO) status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PollerStatus {
    InProgress,
    Succeeded,
    Failed,
    Canceled,
    Other(String),
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

        PollerStatus::Other(value.to_owned())
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
        impl<'de> serde::de::Visitor<'de> for PollerStatusVisitor {
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

/// The result of fetching the status monitor from a [`Poller`], whether the long-running operation (LRO) is in progress or done.
pub enum PollerResult<M, N> {
    /// The long-running operation (LRO) is in progress and the next status monitor update may be fetched from `next`.
    /// The `response` contains the HTTP response with the status monitor.
    InProgress { response: Response<M>, next: N },

    /// The long-running operation (LRO) succeeded and contains the final output.
    Done { response: Response<M> },
}

impl<M: StatusMonitor, N: fmt::Debug> fmt::Debug for PollerResult<M, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InProgress { next, .. } => f
                .debug_struct("InProgress")
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

#[pin_project::pin_project]
pub struct Poller<M> {
    #[pin]
    stream: Pin<BoxedStream<M>>,
}

impl<M> Poller<M>
where
    M: StatusMonitor,
{
    /// Creates a [`Poller<M>`] from a callback that will be called repeatedly to monitor a long-running operation (LRO).
    ///
    /// This method expects a callback that accepts a single `Option<N>` parameter, and returns a [`PollerResult<M, N>`] value asynchronously.
    /// The `N` type parameter is the type of the next link/token. It may be any [`Send`]able type.
    /// The `M` type parameter must implement [`StatusMonitor`].
    ///
    /// The stream will yield [`Response<M>`] values for each intermediate response while the operation is in progress
    /// (i.e., while `M::status()` returns [`PollerStatus::InProgress`]). The stream ends when the operation completes
    /// successfully, fails, or is canceled.
    pub fn from_callback<
        #[cfg(not(target_arch = "wasm32"))] N: Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] F: Fn(Option<N>) -> Fut + Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = crate::Result<PollerResult<M, N>>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] N: 'static,
        #[cfg(target_arch = "wasm32")] F: Fn(Option<N>) -> Fut + 'static,
        #[cfg(target_arch = "wasm32")] Fut: Future<Output = crate::Result<PollerResult<M, N>>> + 'static,
    >(
        make_request: F,
    ) -> Self
    where
        M: Send + 'static,
    {
        Self::from_stream(create_poller_stream(make_request))
    }

    /// Creates a [`Poller<M>`] from a raw stream of [`Result<Response<M>>`](crate::Result<Response<M>>) values.
    pub fn from_stream<
        // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
        #[cfg(not(target_arch = "wasm32"))] S: Stream<Item = crate::Result<Response<M>>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] S: Stream<Item = crate::Result<Response<M>>> + 'static,
    >(
        stream: S,
    ) -> Self {
        Self {
            stream: Box::pin(stream),
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

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().stream.poll_next(cx)
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
    T,
    #[cfg(not(target_arch = "wasm32"))] N: Send + 'static,
    #[cfg(not(target_arch = "wasm32"))] F: Fn(Option<N>) -> Fut + Send + 'static,
    #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = crate::Result<PollerResult<T, N>>> + Send + 'static,
    #[cfg(target_arch = "wasm32")] N: 'static,
    #[cfg(target_arch = "wasm32")] F: Fn(Option<N>) -> Fut + 'static,
    #[cfg(target_arch = "wasm32")] Fut: Future<Output = crate::Result<PollerResult<T, N>>> + 'static,
>(
    make_request: F,
) -> impl Stream<Item = crate::Result<Response<T>>> + 'static
where
    T: StatusMonitor + 'static,
{
    unfold(
        // We flow the `make_request` callback through the state value to avoid cloning.
        (State::Init, make_request),
        |(state, make_request)| async move {
            let result = match state {
                State::Init => make_request(None).await,
                State::InProgress(n) => make_request(Some(n)).await,
                State::Done => return None,
            };
            let (item, next_state) = match result {
                Err(e) => return Some((Err(e), (State::Done, make_request))),
                Ok(PollerResult::InProgress { response, next: n }) => {
                    (Ok(response), State::InProgress(n))
                }
                Ok(PollerResult::Done { response }) => (Ok(response), State::Done),
            };

            // Flow 'make_request' through to avoid cloning
            Some((item, (next_state, make_request)))
        },
    )
}

/// Get the retry duration from the operation response.
pub fn get_retry_after(headers: &Headers) -> Duration {
    crate::http::policies::get_retry_after(headers, OffsetDateTime::now_utc)
        .unwrap_or(DEFAULT_RETRY_TIME)
}

/// Types and methods for getting long-running operation (LRO) resource locations.
///
/// This is more common for data plane operations.
pub mod location {
    use crate::{
        http::{
            headers::{Headers, AZURE_ASYNCOPERATION, LOCATION, OPERATION_LOCATION},
            poller::PollerStatus,
            Url,
        },
        json::from_json,
    };

    /// How to find the final resource URL.
    #[derive(Debug, Clone, Copy)]
    pub enum FinalState {
        /// The final resource URL is found in the `azure-asyncoperation` header.
        AzureAsyncOperation,

        /// The final resource URL is found in the `location` header.
        Location,

        /// The final resource URL is found in the `operation-location` header.
        OperationLocation,
    }

    /// Get the location from the `headers` based on the `final_state` location.
    pub fn get_location(headers: &Headers, final_state: FinalState) -> crate::Result<Option<Url>> {
        match final_state {
            FinalState::AzureAsyncOperation => headers.get_optional_as(&AZURE_ASYNCOPERATION),
            FinalState::Location => headers.get_optional_as(&LOCATION),
            FinalState::OperationLocation => headers.get_optional_as(&OPERATION_LOCATION),
        }
    }

    /// Get the [`PollerStatus`] from the response body.
    pub fn get_operation_state(body: &[u8]) -> Option<PollerStatus> {
        #[derive(serde::Deserialize)]
        struct Body {
            status: String,
        }

        let body: Body = from_json(body).ok()?;
        body.status.parse().ok()
    }
}

/// Types and methods for getting operation status from the body.
///
/// This is more common for control plane (management) operations.
pub mod body {
    use crate::http::{poller::PollerStatus, StatusCode};
    use crate::json::{from_json, to_json};
    use serde::{Deserialize, Serialize};

    /// Extract the long-running operation (LRO) state based on the status code and response body.
    pub fn get_operation_state<S>(status_code: StatusCode, body: &S) -> crate::Result<PollerStatus>
    where
        S: Serialize,
    {
        match status_code {
            StatusCode::Accepted => Ok(PollerStatus::InProgress),
            StatusCode::Created => {
                Ok(get_provisioning_state_from_body(body).unwrap_or(PollerStatus::InProgress))
            }
            StatusCode::Ok => {
                Ok(get_provisioning_state_from_body(body).unwrap_or(PollerStatus::Succeeded))
            }
            StatusCode::NoContent => Ok(PollerStatus::Succeeded),
            _ => Err(crate::error::Error::from(
                crate::error::ErrorKind::HttpResponse {
                    status: status_code,
                    error_code: Some("invalid status found in LRO response".to_owned()),
                },
            )),
        }
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "snake_case")]
    struct Properties {
        provisioning_state: String,
    }

    #[derive(Deserialize)]
    struct Body {
        properties: Properties,
    }

    // TODO: Can we use the StatusMonitor here to avoid re-serializing and deserializing; or, do we even need this anymore?
    fn get_provisioning_state_from_body<S>(body: &S) -> Option<PollerStatus>
    where
        S: Serialize,
    {
        let json = to_json(&body).ok()?;
        let body: Body = from_json(json).ok()?;
        body.properties.provisioning_state.parse().ok()
    }
}

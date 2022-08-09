use crate::error::{Error, ErrorKind, HttpError};
use crate::policies::{Policy, PolicyResult, Request};
use crate::sleep::sleep;
use crate::{Context, StatusCode};

use time::OffsetDateTime;

use std::sync::Arc;
use std::time::Duration;

/// A retry policy.
///
/// All retry policies follow a similar pattern only differing in how
/// they determine if the retry has expired and for how long they should
/// sleep between retries.
pub trait RetryPolicy {
    /// Determine if no more retries should be performed.
    ///
    /// Must return true if no more retries should be attempted.
    fn is_expired(&self, duration_since_start: Duration, retry_count: u32) -> bool;
    /// Determine how long before the next retry should be attempted.
    fn sleep_duration(&self, retry_count: u32) -> Duration;
}

/// The status codes where a retry should be attempted.
///
/// On all other 4xx and 5xx status codes no retry is attempted.
const RETRY_STATUSES: &[StatusCode] = &[
    StatusCode::RequestTimeout,
    StatusCode::TooManyRequests,
    StatusCode::InternalServerError,
    StatusCode::BadGateway,
    StatusCode::ServiceUnavailable,
    StatusCode::GatewayTimeout,
];

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl<T> Policy for T
where
    T: RetryPolicy + std::fmt::Debug + Send + Sync,
{
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let mut retry_count = 0;
        let mut start = None;

        loop {
            let result = next[0].send(ctx, request, &next[1..]).await;
            // only start keeping track of time after the first request is made
            let start = start.get_or_insert_with(OffsetDateTime::now_utc);
            let error = match result {
                Ok(response) if response.status().is_success() => {
                    log::trace!(
                        "Successful response. Request={:?} response={:?}",
                        request,
                        response
                    );
                    return Ok(response);
                }
                Ok(response) => {
                    // Error status code
                    let status = response.status();
                    let http_error = HttpError::new(response).await;

                    let error = Error::full(
                        ErrorKind::http_response(
                            status,
                            http_error.error_code().map(std::borrow::ToOwned::to_owned),
                        ),
                        http_error,
                        format!("server returned error status which will not be retried: {status}"),
                    );

                    if !RETRY_STATUSES.contains(&status) {
                        log::error!(
                            "server returned error status which will not be retried: {}",
                            status
                        );
                        // Server didn't return a status we retry on so return early
                        return Err(error);
                    }
                    log::debug!(
                        "server returned error status which requires retry: {}",
                        status
                    );
                    error
                }
                Err(error) => {
                    if error.kind() == &ErrorKind::Io {
                        log::debug!(
                            "io error occurred when making request which will be retried: {}",
                            error
                        );
                        error
                    } else {
                        log::error!("non-io error occurred which will not be retried: {}", error);
                        return Err(error);
                    }
                }
            };

            let time_since_start = (OffsetDateTime::now_utc() - *start)
                .try_into()
                .unwrap_or_default();
            if self.is_expired(time_since_start, retry_count) {
                return Err(error);
            }
            retry_count += 1;

            sleep(self.sleep_duration(retry_count)).await;
        }
    }
}

use crate::policies::{Policy, PolicyResult, Request, Response};
use crate::sleep::sleep;
use crate::{HttpError, PipelineContext};
use chrono::{DateTime, Local};
use http::StatusCode;
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
    fn is_expired(&self, first_retry_time: &mut Option<DateTime<Local>>, retry_count: u32) -> bool;
    /// Determine how long before the next retry should be attempted.
    fn sleep_duration(&self, retry_count: u32) -> Duration;
}

/// The status codes where a retry should be attempted.
///
/// On all other 4xx and 5xx status codes no retry is attempted.
const RETRY_STATUSES: &[StatusCode] = &[
    StatusCode::REQUEST_TIMEOUT,
    StatusCode::TOO_MANY_REQUESTS,
    StatusCode::INTERNAL_SERVER_ERROR,
    StatusCode::BAD_GATEWAY,
    StatusCode::SERVICE_UNAVAILABLE,
    StatusCode::GATEWAY_TIMEOUT,
];

#[async_trait::async_trait]
impl<T, C> Policy<C> for T
where
    T: RetryPolicy + std::fmt::Debug + Send + Sync,
    C: Send + Sync,
{
    async fn send(
        &self,
        ctx: &mut PipelineContext<C>,
        request: &mut Request,
        next: &[Arc<dyn Policy<C>>],
    ) -> PolicyResult<Response> {
        let mut first_retry_time = None;
        let mut retry_count = 0;

        loop {
            let error = match next[0].send(ctx, request, &next[1..]).await {
                Ok(response) => {
                    let status = response.status();
                    if status.as_u16() < 400 {
                        // Successful status code
                        return Ok(response);
                    }

                    let body = response.into_body_string().await;
                    let error = Box::new(HttpError::ErrorStatusCode { status, body });
                    if !RETRY_STATUSES.contains(&status) {
                        // Server didn't return a status we retry on so return early
                        return Err(error);
                    }
                    log::error!(
                        "server returned error status which requires retry: {}",
                        status
                    );
                    error
                }
                Err(error) => {
                    log::error!("error occurred when making request: {}", error);
                    error
                }
            };

            if self.is_expired(&mut first_retry_time, retry_count) {
                return Err(error);
            }
            retry_count += 1;

            sleep(self.sleep_duration(retry_count)).await;
        }
    }
}

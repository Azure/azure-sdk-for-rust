// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod exponential;
mod fixed;
mod none;

pub use exponential::*;
pub use fixed::*;
pub use none::*;

use crate::{
    error::HttpError,
    http::{
        headers::{Headers, RETRY_AFTER, RETRY_AFTER_MS, X_MS_RETRY_AFTER_MS},
        policies::{Policy, PolicyResult},
        Context, Request, StatusCode,
    },
    sleep::sleep,
    time::{self, Duration, OffsetDateTime},
};
use async_trait::async_trait;
use std::{ops::Deref, sync::Arc};
use tracing::{debug, trace};
use typespec::error::{Error, ErrorKind, ResultExt};

/// Attempts to parse the supplied string as an HTTP date, of the form defined by RFC 7231 (e.g. `Fri, 01 Jan 2021 00:00:00 GMT`).
/// Returns `None` if the string is not a valid HTTP date.
fn try_parse_retry_after_http_date(http_date: &str) -> Option<OffsetDateTime> {
    crate::time::parse_rfc7231(http_date).ok()
}

/// A function that returns an `OffsetDateTime`.
type DateTimeFn = fn() -> OffsetDateTime;

/// Get the duration to delay between retry attempts, provided by the headers from the response.
///
/// This function checks for retry-after headers in the following order:
///
/// 1. `retry-after-ms`
/// 2. `x-ms-retry-after-ms`
/// 3. `retry-after`
///
/// If no header is provided, `None` is returned.
pub fn get_retry_after(headers: &Headers, now: DateTimeFn) -> Option<Duration> {
    // TODO: Only check Microsoft headers when constructed from azure_core (https://github.com/Azure/azure-sdk-for-rust/issues/1753)
    [RETRY_AFTER_MS, X_MS_RETRY_AFTER_MS, RETRY_AFTER]
        .iter()
        .find_map(|header| {
            headers.get_str(header).ok().and_then(|v| {
                if header == &RETRY_AFTER {
                    // RETRY_AFTER values are either in seconds or a HTTP date
                    v.parse::<i64>().ok().map(Duration::seconds).or_else(|| {
                        try_parse_retry_after_http_date(v).map(|retry_after_datetime| {
                            let now = now();
                            if retry_after_datetime < now {
                                Duration::seconds(0)
                            } else {
                                time::diff(retry_after_datetime, now)
                            }
                        })
                    })
                } else {
                    // RETRY_AFTER_MS or X_MS_RETRY_AFTER_MS values are in milliseconds
                    v.parse::<i64>().ok().map(Duration::milliseconds)
                }
            })
        })
}

/// A wrapper around a retry count to be used in the context of a retry policy.
///
/// This allows a post-retry policy to access the retry count
pub struct RetryPolicyCount(u32);

impl Deref for RetryPolicyCount {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A retry policy.
///
/// In the simple form, the policies need only differ in how
/// they determine if the retry has expired and for how long they should
/// sleep between retries.
///
/// `wait` can be implemented in more complex cases where a simple test of time
/// is not enough.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait RetryPolicy: std::fmt::Debug + Send + Sync {
    /// Determine if no more retries should be performed.
    ///
    /// Must return true if no more retries should be attempted.
    fn is_expired(&self, duration_since_start: Duration, retry_count: u32) -> bool;
    /// Determine how long before the next retry should be attempted.
    fn sleep_duration(&self, retry_count: u32) -> Duration;
    /// A Future that will wait until the request can be retried.
    /// `error` is the [`Error`] value the led to a retry attempt.
    /// `retry_after` is the duration to wait before retrying, if provided by the server response.
    async fn wait(&self, _error: &Error, retry_count: u32, retry_after: Option<Duration>) {
        let policy_sleep_duration = self.sleep_duration(retry_count);
        // If the server provided a retry-after header, use the max of that and the policy sleep duration
        let sleep_duration = retry_after.map_or(policy_sleep_duration, |retry_after| {
            std::cmp::max(retry_after, policy_sleep_duration)
        });
        sleep(sleep_duration).await;
    }
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
    T: RetryPolicy,
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
            if retry_count > 0 {
                request.body.reset().await.context(
                    ErrorKind::Other,
                    "failed to reset body stream before retrying request",
                )?;
            }
            let ctx = ctx.clone().with_value(RetryPolicyCount(retry_count));
            let result = next[0].send(&ctx, request, &next[1..]).await;
            // only start keeping track of time after the first request is made
            let start = start.get_or_insert_with(OffsetDateTime::now_utc);
            let (last_error, retry_after) = match result {
                Ok(response) => {
                    let status = response.status();
                    if !RETRY_STATUSES.contains(&status) {
                        if status.is_success() {
                            trace!(
                                // ?request,
                                // ?response,
                                "server returned success status {}",
                                status,
                            );
                        } else {
                            debug!(
                                "server returned status which will not be retried: {}",
                                status
                            );
                        }
                        return Ok(response);
                    }

                    // For a 429 response (TooManyRequests) or 503 (ServiceUnavailable),
                    // use any "retry-after" headers returned by the server to determine how long to wait before retrying.
                    // https://learn.microsoft.com/en-us/azure/architecture/best-practices/retry-service-specific#retry-usage-guidance
                    let retry_after = match status {
                        StatusCode::TooManyRequests | StatusCode::ServiceUnavailable => {
                            get_retry_after(response.headers(), OffsetDateTime::now_utc)
                        }
                        _ => None,
                    };

                    let http_error = HttpError::new(response).await;

                    let error_kind = ErrorKind::http_response(
                        status,
                        http_error.error_code().map(std::borrow::ToOwned::to_owned),
                    );

                    debug!(
                        "server returned error status which requires retry: {}",
                        status
                    );
                    (Error::new(error_kind, http_error), retry_after)
                }
                Err(error) => {
                    if error.kind() == &ErrorKind::Io {
                        debug!(
                            "io error occurred when making request which will be retried: {}",
                            error
                        );
                        // IO error so no Retry-After headers - leave the retry period up to the policy
                        let retry_after = None;
                        (error, retry_after)
                    } else {
                        return Err(
                            error.context("non-io error occurred which will not be retried")
                        );
                    }
                }
            };

            let time_since_start = OffsetDateTime::now_utc() - *start;
            if self.is_expired(time_since_start, retry_count) {
                return Err(last_error
                    .context("retry policy expired and the request will no longer be retried"));
            }
            retry_count += 1;

            self.wait(&last_error, retry_count, retry_after).await;
        }
    }
}

// Unit tests
#[cfg(test)]
mod test {
    use super::*;
    use crate::http::{
        headers::Headers, Context, FixedRetryOptions, Method, RawResponse, Request, RetryOptions,
        Url,
    };
    use ::time::macros::datetime;
    use std::sync::{Arc, Mutex};

    // Policy that counts the requests it receives and returns responses having a given status code
    #[derive(Debug)]
    struct StatusResponder {
        request_count: Arc<Mutex<u32>>,
        status: StatusCode,
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl Policy for StatusResponder {
        async fn send(&self, _: &Context, _: &mut Request, _: &[Arc<dyn Policy>]) -> PolicyResult {
            let mut count = self.request_count.lock().unwrap();
            *count += 1;
            Ok(RawResponse::from_bytes(self.status, Headers::new(), ""))
        }
    }

    // A function that returns a fixed "now" value for testing.
    fn datetime_now() -> OffsetDateTime {
        datetime!(2021-01-01 0:00:00 UTC)
    }

    #[test]
    fn test_try_parse_retry_after_http_date() {
        // Test parsing a valid HTTP date
        let retry_after = try_parse_retry_after_http_date("Fri, 01 Jan 2021 00:00:00 GMT");
        assert_eq!(retry_after, Some(datetime!(2021-01-01 0:00:00 UTC)));

        // Test parsing an invalid HTTP date (missing day of week)
        let retry_after = try_parse_retry_after_http_date("01 Jan 2021 00:00:00 GMT");
        assert_eq!(retry_after, None);

        // Test parsing an invalid HTTP date (complete garbage)
        let retry_after = try_parse_retry_after_http_date("invalid");
        assert_eq!(retry_after, None);

        // Test parsing an integer value fails
        let retry_after = try_parse_retry_after_http_date("123");
        assert_eq!(retry_after, None);
    }

    #[test]
    fn test_get_retry_after() {
        // Test parsing a valid HTTP date that is 10 secs in the future
        let mut headers = Headers::new();
        headers.insert(RETRY_AFTER, "Fri, 01 Jan 2021 00:00:10 GMT");
        let retry_after = get_retry_after(&headers, datetime_now);
        assert_eq!(retry_after, Some(Duration::seconds(10)));

        // Test parsing a valid HTTP date that is in the past returns 0
        let mut headers = Headers::new();
        headers.insert(RETRY_AFTER, "Thu, 31 Dec 2020 23:59:50 GMT");
        let retry_after = get_retry_after(&headers, datetime_now);
        assert_eq!(retry_after, Some(Duration::seconds(0)));

        // Test that when no retry headers are present, None is returned
        let headers = Headers::new();
        let retry_after = get_retry_after(&headers, datetime_now);
        assert_eq!(retry_after, None);

        // Test parsing an invalid HTTP date
        let mut headers = Headers::new();
        headers.insert(RETRY_AFTER, "invalid");
        let retry_after = get_retry_after(&headers, datetime_now);
        assert_eq!(retry_after, None);

        // Test `RETRY_AFTER` parsing an integer value
        let mut headers = Headers::new();
        headers.insert(RETRY_AFTER, "123");
        let retry_after = get_retry_after(&headers, datetime_now);
        assert_eq!(retry_after, Some(Duration::seconds(123)));
    }

    #[tokio::test]
    async fn test_retry_statuses() {
        let retries = 2u32;
        let retry_policy = RetryOptions::fixed(FixedRetryOptions {
            delay: Duration::nanoseconds(1),
            max_retries: retries,
            ..Default::default()
        })
        .to_policy();
        let ctx = Context::new();
        let url = Url::parse("http://localhost").unwrap();

        for &status in RETRY_STATUSES {
            let mut request = Request::new(url.clone(), Method::Get);
            let count = Arc::new(Mutex::new(0));
            let mock = StatusResponder {
                request_count: count.clone(),
                status,
            };
            let next = vec![Arc::new(mock) as Arc<dyn Policy>];

            retry_policy
                .send(&ctx, &mut request, &next)
                .await
                .expect_err("Policy should return an error after exhausting retries");

            assert_eq!(
                retries + 1,
                *count.lock().unwrap(),
                "Policy should retry {status}"
            );
        }

        let mut request = Request::new(url.clone(), Method::Get);
        let count = Arc::new(Mutex::new(0));
        let next = vec![Arc::new(StatusResponder {
            request_count: count.clone(),
            status: StatusCode::Unauthorized,
        }) as Arc<dyn Policy>];

        let response = retry_policy
            .send(&ctx, &mut request, &next)
            .await
            .expect("Policy should return a response whose status isn't in RETRY_STATUSES");

        assert_eq!(response.status(), StatusCode::Unauthorized);
        assert_eq!(
            1,
            *count.lock().unwrap(),
            "Policy shouldn't retry after receiving a response whose status isn't in RETRY_STATUSES"
        );
    }
}

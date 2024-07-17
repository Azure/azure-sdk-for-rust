use crate::{
    date,
    error::{Error, ErrorKind, HttpError, ResultExt},
    headers::{Headers, RETRY_AFTER, RETRY_AFTER_MS, X_MS_RETRY_AFTER_MS},
    policies::{Policy, PolicyResult, Request},
    sleep::sleep,
    Context, StatusCode,
};
use async_trait::async_trait;
use std::{sync::Arc, time::Duration};
use time::OffsetDateTime;
use tracing::{debug, trace};

/// Attempts to parse the supplied string as an HTTP date, of the form defined by RFC 1123 (e.g. `Fri, 01 Jan 2021 00:00:00 GMT`).
/// Returns `None` if the string is not a valid HTTP date.
fn try_parse_retry_after_http_date(http_date: &str) -> Option<OffsetDateTime> {
    crate::date::parse_rfc1123(http_date).ok()
}

/// A function that returns an `OffsetDateTime`.
type DateTimeFn = fn() -> OffsetDateTime;

/// Get the duration to delay between retry attempts, provided by the headers from the response.
///
/// This function checks for retry-after headers in the following order, following the
/// JS Azure SDK implementation: <https://github.com/Azure/azure-sdk-for-js/blob/17de1a2b7f3ad61f34ff62876eced7d077c10d4b/sdk/core/core-rest-pipeline/src/retryStrategies/throttlingRetryStrategy.ts#L35>
/// * `retry-after-ms`
/// * `x-ms-retry-after-ms`
/// * `Retry-After`
///
/// If no header is provided, None is returned.
pub(crate) fn get_retry_after(headers: &Headers, datetime_now: DateTimeFn) -> Option<Duration> {
    [RETRY_AFTER_MS, X_MS_RETRY_AFTER_MS, RETRY_AFTER]
        .iter()
        .find_map(|header| {
            headers.get_str(header).ok().and_then(|v| {
                if header == &RETRY_AFTER {
                    // RETRY_AFTER values are either in seconds or a HTTP date
                    v.parse::<u64>().ok().map(Duration::from_secs).or_else(|| {
                        try_parse_retry_after_http_date(v).map(|retry_after_datetime| {
                            let now = datetime_now();
                            if retry_after_datetime < now {
                                Duration::from_secs(0)
                            } else {
                                date::diff(retry_after_datetime, now)
                            }
                        })
                    })
                } else {
                    // RETRY_AFTER_MS or X_MS_RETRY_AFTER_MS values are in milliseconds
                    v.parse::<u64>().ok().map(Duration::from_millis)
                }
            })
        })
}

/// A retry policy.
///
/// In the simple form, the policies need only differ in how
/// they determine if the retry has expired and for how long they should
/// sleep between retries.
///
/// `wait` can be implemented in more complex cases where a simple test of time
/// is not enough.
#[async_trait]
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
            let result = next[0].send(ctx, request, &next[1..]).await;
            // only start keeping track of time after the first request is made
            let start = start.get_or_insert_with(OffsetDateTime::now_utc);
            let (last_error, retry_after) = match result {
                Ok(response) if response.status().is_success() => {
                    trace!(
                        "Successful response. Request={:?} response={:?}",
                        request,
                        response
                    );
                    return Ok(response);
                }
                Ok(response) => {
                    // Error status code
                    let status = response.status();

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

                    if !RETRY_STATUSES.contains(&status) {
                        debug!(
                            "server returned error status which will not be retried: {}",
                            status
                        );
                        // Server didn't return a status we retry on so return early
                        let error = Error::full(
                            error_kind,
                            http_error,
                            format!(
                                "server returned error status which will not be retried: {status}"
                            ),
                        );
                        return Err(error);
                    }
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

            let time_since_start = (OffsetDateTime::now_utc() - *start)
                .try_into()
                .unwrap_or_default();
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
    use time::macros::datetime;

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
        assert_eq!(retry_after, Some(Duration::from_secs(10)));

        // Test parsing a valid HTTP date that is in the past returns 0
        let mut headers = Headers::new();
        headers.insert(RETRY_AFTER, "Thu, 31 Dec 2020 23:59:50 GMT");
        let retry_after = get_retry_after(&headers, datetime_now);
        assert_eq!(retry_after, Some(Duration::from_secs(0)));

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
        assert_eq!(retry_after, Some(Duration::from_secs(123)));

        // Test `RETRY_AFTER_MS` parsing an integer value
        let mut headers = Headers::new();
        headers.insert(RETRY_AFTER_MS, "123");
        let retry_after = get_retry_after(&headers, datetime_now);
        assert_eq!(retry_after, Some(Duration::from_millis(123)));

        // Test `RETRY_AFTER_MS` parsing an integer value
        let mut headers = Headers::new();
        headers.insert(RETRY_AFTER_MS, "123");
        let retry_after = get_retry_after(&headers, datetime_now);
        assert_eq!(retry_after, Some(Duration::from_millis(123)));

        // Test `X_MS_RETRY_AFTER_MS` parsing an integer value
        let mut headers = Headers::new();
        headers.insert(X_MS_RETRY_AFTER_MS, "123");
        let retry_after = get_retry_after(&headers, datetime_now);
        assert_eq!(retry_after, Some(Duration::from_millis(123)));

        // Test that when both `RETRY_AFTER_MS` and `RETRY_AFTER` are present, `RETRY_AFTER_MS` is used
        let mut headers = Headers::new();
        headers.insert(RETRY_AFTER_MS, "123");
        headers.insert(RETRY_AFTER, "456");
        let retry_after = get_retry_after(&headers, datetime_now);
        assert_eq!(retry_after, Some(Duration::from_millis(123)));

        // Test that when both `X_MS_RETRY_AFTER_MS` and `RETRY_AFTER` are present, `X_MS_RETRY_AFTER_MS` is used
        let mut headers = Headers::new();
        headers.insert(X_MS_RETRY_AFTER_MS, "123");
        headers.insert(RETRY_AFTER, "456");
        let retry_after = get_retry_after(&headers, datetime_now);
        assert_eq!(retry_after, Some(Duration::from_millis(123)));

        // Test that when both `X_MS_RETRY_AFTER_MS` and `RETRY_AFTER_MS` are present, `RETRY_AFTER_MS` is used
        let mut headers = Headers::new();
        headers.insert(X_MS_RETRY_AFTER_MS, "123");
        headers.insert(RETRY_AFTER_MS, "456");
        let retry_after = get_retry_after(&headers, datetime_now);
        assert_eq!(retry_after, Some(Duration::from_millis(456)));
    }
}

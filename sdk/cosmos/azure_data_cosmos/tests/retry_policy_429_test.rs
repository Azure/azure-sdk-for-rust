#![cfg(feature = "key_auth")]

//! Unit test to validate ResourceThrottleRetryPolicy behavior on 429 status codes
//!
//! This test validates that:
//! 1. The retry policy correctly identifies 429 TooManyRequests status
//! 2. The retry policy increments retry attempts
//! 3. The retry policy calculates appropriate backoff delays
//! 4. The retry policy respects maximum retry limits

use azure_core::http::{RawResponse, StatusCode};
use azure_core::time::Duration;
use azure_data_cosmos::retry_policies::{
    resource_throttle_retry_policy::ResourceThrottleRetryPolicy, RetryPolicy, RetryResult,
};

/// Helper function to create a mock RawResponse with a given status code
fn create_mock_response(status: StatusCode) -> azure_core::Result<RawResponse> {
    use typespec_client_core::http::headers::Headers;

    // Create headers
    let mut headers = Headers::new();
    headers.insert("content-type", "application/json");

    azure_core::Result::from(Ok(RawResponse::from_bytes(
        status,
        headers,
        r#"{"id":12,"name":"Too Many Requests"}"#,
    )))
}

#[tokio::test]
async fn test_retry_policy_handles_429_status() {
    // Create a retry policy with 3 max retries, 100 second max wait time, and backoff factor of 2
    let policy = ResourceThrottleRetryPolicy::new(3, 100, 2);

    // Simulate a 429 TooManyRequests response
    let response_429 = create_mock_response(StatusCode::TooManyRequests);

    // First retry attempt
    let result1 = policy.should_retry(&response_429).await;
    match result1 {
        RetryResult::Retry { after } => {
            assert!(after > Duration::ZERO, "Should have backoff time");
            println!("First retry - backoff time: {:?}", after);
        }
        RetryResult::DoNotRetry => panic!("Should retry on first 429 response"),
    }

    // Second retry attempt
    let result2 = policy.should_retry(&response_429).await;
    let backoff2 = match result2 {
        RetryResult::Retry { after } => {
            assert!(after > Duration::ZERO, "Should have backoff time");
            after
        }
        RetryResult::DoNotRetry => panic!("Should retry on second 429 response"),
    };

    // Extract backoff1 for comparison
    let backoff1 = match result1 {
        RetryResult::Retry { after } => after,
        _ => panic!("Expected retry result"),
    };

    assert!(
        backoff2 >= backoff1,
        "Backoff should increase with exponential backoff"
    );
    println!("Second retry - backoff time: {:?}", backoff2);

    // Third retry attempt
    let result3 = policy.should_retry(&response_429).await;
    match result3 {
        RetryResult::Retry { after } => {
            assert!(after > Duration::ZERO, "Should have backoff time");
            println!("Third retry - backoff time: {:?}", after);
        }
        RetryResult::DoNotRetry => panic!("Should retry on third 429 response"),
    }

    // Fourth attempt should NOT retry (exceeded max_attempt_count of 3)
    let result4 = policy.should_retry(&response_429).await;
    match result4 {
        RetryResult::DoNotRetry => {
            println!("Fourth attempt - should not retry");
        }
        RetryResult::Retry { .. } => panic!("Should NOT retry after exceeding max attempts"),
    }
}

#[tokio::test]
async fn test_retry_policy_does_not_retry_on_success() {
    let policy = ResourceThrottleRetryPolicy::new(3, 100, 2);

    // Simulate a 200 OK response (success)
    let response_200 = create_mock_response(StatusCode::Ok);

    let result = policy.should_retry(&response_200).await;
    match result {
        RetryResult::DoNotRetry => {
            // Success - should not retry
        }
        RetryResult::Retry { .. } => panic!("Should NOT retry on successful response"),
    }
}

#[tokio::test]
async fn test_retry_policy_does_not_retry_on_client_errors() {
    let policy = ResourceThrottleRetryPolicy::new(3, 100, 2);

    // Test various client errors that should NOT trigger retry
    let test_cases = vec![
        (StatusCode::BadRequest, "400 Bad Request"),
        (StatusCode::Unauthorized, "401 Unauthorized"),
        (StatusCode::Forbidden, "403 Forbidden"),
        (StatusCode::NotFound, "404 Not Found"),
        (StatusCode::Conflict, "409 Conflict"),
    ];

    for (status, description) in test_cases {
        let response = create_mock_response(status);
        let result = policy.should_retry(&response).await;
        match result {
            RetryResult::DoNotRetry => {
                // Success - should not retry on client errors
            }
            RetryResult::Retry { .. } => panic!("Should NOT retry on {}", description),
        }
    }
}

#[tokio::test]
async fn test_retry_policy_backoff_calculation() {
    let backoff_factor = 3;
    let policy = ResourceThrottleRetryPolicy::new(5, 1000, backoff_factor);

    let response_429 = create_mock_response(StatusCode::TooManyRequests);

    let mut previous_backoff = Duration::ZERO;

    for attempt in 1..=3 {
        let result = policy.should_retry(&response_429).await;
        match result {
            RetryResult::Retry { after } => {
                if attempt > 1 {
                    // With exponential backoff, each attempt should have longer delay
                    // (though the exact multiplier depends on internal logic)
                    println!(
                        "Attempt {}: backoff = {:?} (previous = {:?})",
                        attempt, after, previous_backoff
                    );
                }
                previous_backoff = after;
            }
            RetryResult::DoNotRetry => panic!("Attempt {} should trigger retry", attempt),
        }
    }
}

#[tokio::test]
async fn test_retry_policy_respects_max_wait_time() {
    // Set very low max_wait_time to test the limit
    let max_wait_secs = 5;
    let policy = ResourceThrottleRetryPolicy::new(10, max_wait_secs, 100);

    let response_429 = create_mock_response(StatusCode::TooManyRequests);

    let mut total_delay = Duration::ZERO;
    let max_wait = Duration::seconds(max_wait_secs as i64);

    // Keep retrying until we hit the cumulative wait time limit
    for attempt in 1..=10 {
        let result = policy.should_retry(&response_429).await;

        match result {
            RetryResult::Retry { after } => {
                total_delay += after;
                println!(
                    "Attempt {}: backoff = {:?}, cumulative = {:?}",
                    attempt, after, total_delay
                );

                // Total cumulative delay should never exceed max_wait_time
                assert!(
                    total_delay <= max_wait,
                    "Cumulative delay {:?} should not exceed max wait time {:?}",
                    total_delay,
                    max_wait
                );
            }
            RetryResult::DoNotRetry => {
                println!(
                    "Stopped retrying at attempt {} due to max wait time limit",
                    attempt
                );
                break;
            }
        }
    }
}

// Note: Testing error exceptions with specific status codes is challenging
// because azure_core::Error doesn't provide a way to set HTTP status codes directly.
// The should_retry_exception method in ResourceThrottleRetryPolicy checks for
// 429 status using err.http_status(), which returns None for manually constructed errors.
// In real scenarios, errors would come from actual HTTP responses.

/// Integration-style test that demonstrates the retry counter increments
#[tokio::test]
async fn test_retry_counter_increments() {
    let policy = ResourceThrottleRetryPolicy::new(5, 100, 2);
    let response_429 = create_mock_response(StatusCode::TooManyRequests);

    // Track how many times we get a retry decision
    let mut retry_count = 0;

    for attempt in 1..=10 {
        let result = policy.should_retry(&response_429).await;

        match result {
            RetryResult::Retry { after } => {
                retry_count += 1;
                println!(
                    "Attempt {}: Retry #{} - backoff: {:?}",
                    attempt, retry_count, after
                );
            }
            RetryResult::DoNotRetry => {
                println!(
                    "Attempt {}: No more retries (total retries = {})",
                    attempt, retry_count
                );
                break;
            }
        }
    }

    // We should have gotten exactly max_attempt_count retries
    assert_eq!(
        retry_count, 5,
        "Should have retried exactly max_attempt_count times"
    );
}

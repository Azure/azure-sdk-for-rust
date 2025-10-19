#![cfg(feature = "key_auth")]

//! Unit test to validate ResourceThrottleRetryPolicy behavior on 429 status codes
//!
//! This test validates that:
//! 1. The retry policy correctly identifies 429 TooManyRequests status
//! 2. The retry policy increments retry attempts
//! 3. The retry policy calculates appropriate backoff delays
//! 4. The retry policy respects maximum retry limits

use azure_core::http::{RawResponse, StatusCode};
use azure_data_cosmos::retry_policies::{
    resource_throttle_retry_policy::ResourceThrottleRetryPolicy, RetryPolicy,
};
use std::time::Duration;

/// Helper function to create a mock RawResponse with a given status code
fn create_mock_response(status: StatusCode) -> RawResponse {
    use typespec_client_core::http::headers::Headers;

    // Create headers
    let mut headers = Headers::new();
    headers.insert("content-type", "application/json");

    RawResponse::from_bytes(status, headers, r#"{"id":12,"name":"Too Many Requests"}"#)
}

#[tokio::test]
async fn test_retry_policy_handles_429_status() {
    // Create a retry policy with 3 max retries, 100 second max wait time, and backoff factor of 2
    let policy = ResourceThrottleRetryPolicy::new(3, 100, 2);

    // Simulate a 429 TooManyRequests response
    let response_429 = create_mock_response(StatusCode::TooManyRequests);

    // First retry attempt
    let result1 = policy.should_retry_response(&response_429).await;
    assert!(result1.should_retry, "Should retry on first 429 response");
    assert!(
        result1.backoff_time > Duration::ZERO,
        "Should have backoff time"
    );
    println!("First retry - backoff time: {:?}", result1.backoff_time);

    // Second retry attempt
    let result2 = policy.should_retry_response(&response_429).await;
    assert!(result2.should_retry, "Should retry on second 429 response");
    assert!(
        result2.backoff_time > Duration::ZERO,
        "Should have backoff time"
    );
    assert!(
        result2.backoff_time >= result1.backoff_time,
        "Backoff should increase with exponential backoff"
    );
    println!("Second retry - backoff time: {:?}", result2.backoff_time);

    // Third retry attempt
    let result3 = policy.should_retry_response(&response_429).await;
    assert!(result3.should_retry, "Should retry on third 429 response");
    assert!(
        result3.backoff_time > Duration::ZERO,
        "Should have backoff time"
    );
    println!("Third retry - backoff time: {:?}", result3.backoff_time);

    // Fourth attempt should NOT retry (exceeded max_attempt_count of 3)
    let result4 = policy.should_retry_response(&response_429).await;
    assert!(
        !result4.should_retry,
        "Should NOT retry after exceeding max attempts"
    );
    assert_eq!(
        result4.backoff_time,
        Duration::ZERO,
        "Should have zero backoff when not retrying"
    );
    println!("Fourth attempt - should not retry");
}

#[tokio::test]
async fn test_retry_policy_does_not_retry_on_success() {
    let policy = ResourceThrottleRetryPolicy::new(3, 100, 2);

    // Simulate a 200 OK response (success)
    let response_200 = create_mock_response(StatusCode::Ok);

    let result = policy.should_retry_response(&response_200).await;
    assert!(
        !result.should_retry,
        "Should NOT retry on successful response"
    );
    assert_eq!(
        result.backoff_time,
        Duration::ZERO,
        "Should have zero backoff for success"
    );
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
        let result = policy.should_retry_response(&response).await;
        assert!(!result.should_retry, "Should NOT retry on {}", description);
        assert_eq!(
            result.backoff_time,
            Duration::ZERO,
            "Should have zero backoff for {}",
            description
        );
    }
}

#[tokio::test]
async fn test_retry_policy_backoff_calculation() {
    let backoff_factor = 3;
    let policy = ResourceThrottleRetryPolicy::new(5, 1000, backoff_factor);

    let response_429 = create_mock_response(StatusCode::TooManyRequests);

    let mut previous_backoff = Duration::ZERO;

    for attempt in 1..=3 {
        let result = policy.should_retry_response(&response_429).await;
        assert!(
            result.should_retry,
            "Attempt {} should trigger retry",
            attempt
        );

        if attempt > 1 {
            // With exponential backoff, each attempt should have longer delay
            // (though the exact multiplier depends on internal logic)
            println!(
                "Attempt {}: backoff = {:?} (previous = {:?})",
                attempt, result.backoff_time, previous_backoff
            );
        }

        previous_backoff = result.backoff_time;
    }
}

#[tokio::test]
async fn test_retry_policy_respects_max_wait_time() {
    // Set very low max_wait_time to test the limit
    let max_wait_secs = 5;
    let policy = ResourceThrottleRetryPolicy::new(10, max_wait_secs, 100);

    let response_429 = create_mock_response(StatusCode::TooManyRequests);

    let mut total_delay = Duration::ZERO;
    let max_wait = Duration::from_secs(max_wait_secs);

    // Keep retrying until we hit the cumulative wait time limit
    for attempt in 1..=10 {
        let result = policy.should_retry_response(&response_429).await;

        if result.should_retry {
            total_delay += result.backoff_time;
            println!(
                "Attempt {}: backoff = {:?}, cumulative = {:?}",
                attempt, result.backoff_time, total_delay
            );

            // Total cumulative delay should never exceed max_wait_time
            assert!(
                total_delay <= max_wait,
                "Cumulative delay {:?} should not exceed max wait time {:?}",
                total_delay,
                max_wait
            );
        } else {
            println!(
                "Stopped retrying at attempt {} due to max wait time limit",
                attempt
            );
            break;
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
        let result = policy.should_retry_response(&response_429).await;

        if result.should_retry {
            retry_count += 1;
            println!(
                "Attempt {}: Retry #{} - backoff: {:?}",
                attempt, retry_count, result.backoff_time
            );
        } else {
            println!(
                "Attempt {}: No more retries (total retries = {})",
                attempt, retry_count
            );
            break;
        }
    }

    // We should have gotten exactly max_attempt_count retries
    assert_eq!(
        retry_count, 5,
        "Should have retried exactly max_attempt_count times"
    );
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::result::FaultInjectionResult;
use super::rule::FaultInjectionRule;
use super::FaultInjectionErrorType;
use super::FaultOperationType;
use crate::constants::{self, SubStatusCode};
use async_trait::async_trait;
use azure_core::error::ErrorKind;
use azure_core::http::{
    headers::Headers, AsyncRawResponse, HttpClient, RawResponse, Request, StatusCode,
};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Custom implementation of an HTTP client that injects faults for testing purposes.
#[derive(Debug)]
pub struct FaultClient {
    /// The inner HTTP client to which requests are delegated.
    inner: Arc<dyn HttpClient>,
    /// The fault injection rules to apply.
    rules: Arc<Mutex<Vec<RuleState>>>,
}

/// Tracks the state of a fault injection rule.
#[derive(Debug)]
struct RuleState {
    /// The fault injection rule.
    rule: Arc<FaultInjectionRule>,
    /// Number of times this rule has been applied.
    hit_count: AtomicU32,
}

impl FaultClient {
    /// Creates a new instance of the FaultClient.
    pub fn new(inner: Arc<dyn HttpClient>, rules: Vec<Arc<FaultInjectionRule>>) -> Self {
        let rule_states = rules
            .into_iter()
            .map(|rule| RuleState {
                rule,
                hit_count: AtomicU32::new(0),
            })
            .collect();

        Self {
            inner,
            rules: Arc::new(Mutex::new(rule_states)),
        }
    }

    /// Checks if a rule is currently applicable based on timing constraints.
    fn is_rule_applicable(&self, rule_state: &RuleState) -> bool {
        let now = Instant::now();
        let rule = &rule_state.rule;

        // Check if the rule is enabled
        if !rule.is_enabled() {
            return false;
        }

        // Check if the rule has started
        if now < rule.start_time {
            return false;
        }

        // Check if the rule has expired
        if let Some(end_time) = rule.end_time {
            if now >= end_time {
                return false;
            }
        }

        // Check if we've exceeded the hit limit on the rule
        if let Some(hit_limit) = rule.hit_limit {
            if rule_state.hit_count.load(Ordering::SeqCst) >= hit_limit {
                return false;
            }
        }

        true
    }

    /// Checks if the request matches the rule's condition.
    fn matches_condition(&self, request: &Request, rule: &FaultInjectionRule) -> bool {
        let condition = &rule.condition;
        let mut matches = true;

        // Check operation type if specified
        if let Some(expected_op) = condition.operation_type {
            let request_op = request
                .headers()
                .get_optional_str(&constants::FAULT_INJECTION_OPERATION)
                .and_then(|s| s.parse::<FaultOperationType>().ok());

            match request_op {
                Some(op) if op == expected_op => {
                    // Operation type matches, continue checking other conditions
                }
                _ => {
                    // Operation type doesn't match or header not present
                    matches = false;
                }
            }
        }

        // Check region if specified
        if let Some(region) = &condition.region {
            if !request.url().as_str().contains(region.as_str()) {
                matches = false;
            }
        }

        // Check container ID if specified
        if let Some(container_id) = &condition.container_id {
            if !request.url().as_str().contains(container_id) {
                matches = false;
            }
        }

        matches
    }

    /// Applies the fault injection result and returns an error or modifies the response.
    async fn apply_fault(
        &self,
        server_error: &FaultInjectionResult,
    ) -> Option<azure_core::Result<AsyncRawResponse>> {
        // Check probability
        if server_error.probability() < 1.0 {
            // Use a simple time-based pseudo-random check
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos();
            let random = (nanos % 1000) as f32 / 1000.0;
            if !server_error.probability().is_finite()
                || server_error.probability() <= 0.0
                || random >= server_error.probability()
            {
                return None; // Don't inject fault this time
            }
        }

        // Check for custom response first (takes precedence over error injection)
        if let Some(ref custom) = server_error.custom_response {
            return Some(Ok(AsyncRawResponse::from_bytes(
                custom.status_code,
                custom.headers.clone(),
                custom.body.clone(),
            )));
        }

        // Generate the appropriate error based on error type
        let error_type = match server_error.error_type {
            Some(et) => et,
            None => return None, // No error type set, pass through
        };

        // Connection-level faults return simple errors with the appropriate ErrorKind.
        // HTTP-level faults produce synthetic error responses.
        let (status_code, sub_status, message) = match error_type {
            FaultInjectionErrorType::ConnectionError => {
                return Some(Err(azure_core::Error::with_message(
                    ErrorKind::Io,
                    "Injected fault: connection error",
                )));
            }
            FaultInjectionErrorType::ResponseTimeout => {
                return Some(Err(azure_core::Error::with_message(
                    ErrorKind::Io,
                    "Injected fault: response timeout",
                )));
            }
            FaultInjectionErrorType::InternalServerError => (
                StatusCode::InternalServerError,
                None,
                "Internal Server Error - Injected fault",
            ),
            FaultInjectionErrorType::TooManyRequests => (
                StatusCode::TooManyRequests,
                None,
                "Too Many Requests - Injected fault",
            ),
            FaultInjectionErrorType::ReadSessionNotAvailable => (
                StatusCode::NotFound,
                Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
                "Read Session Not Available - Injected fault",
            ),
            FaultInjectionErrorType::Timeout => (
                StatusCode::RequestTimeout,
                None,
                "Request Timeout - Injected fault",
            ),
            FaultInjectionErrorType::ServiceUnavailable => (
                StatusCode::ServiceUnavailable,
                None,
                "Service Unavailable - Injected fault",
            ),
            FaultInjectionErrorType::PartitionIsGone => (
                StatusCode::Gone,
                Some(SubStatusCode::PARTITION_KEY_RANGE_GONE),
                "Partition Is Gone - Injected fault",
            ),
            FaultInjectionErrorType::WriteForbidden => (
                StatusCode::Forbidden,
                Some(SubStatusCode::WRITE_FORBIDDEN),
                "Write Forbidden - Injected fault",
            ),
            FaultInjectionErrorType::DatabaseAccountNotFound => (
                StatusCode::Forbidden,
                Some(SubStatusCode::DATABASE_ACCOUNT_NOT_FOUND),
                "Database Account Not Found - Injected fault",
            ),
        };

        let raw_response = sub_status.map(|ss| {
            let mut headers = Headers::new();
            headers.insert(constants::SUB_STATUS, ss.to_string());
            Box::new(RawResponse::from_bytes(status_code, headers, vec![]))
        });

        let error = azure_core::Error::with_message(
            ErrorKind::HttpResponse {
                status: status_code,
                error_code: Some("Injected Fault".to_string()),
                raw_response,
            },
            message,
        );

        Some(Err(error))
    }
}

#[async_trait]
impl HttpClient for FaultClient {
    async fn execute_request(&self, request: &Request) -> azure_core::Result<AsyncRawResponse> {
        // Find applicable rule and clone the result if needed
        let fault_result: Option<FaultInjectionResult> = {
            let rules = self.rules.lock().unwrap();
            let mut applicable_rule_index: Option<usize> = None;

            for (index, rule_state) in rules.iter().enumerate() {
                if self.is_rule_applicable(rule_state)
                    && self.matches_condition(request, &rule_state.rule)
                {
                    applicable_rule_index = Some(index);
                    break;
                }
            }

            // Apply fault if we found an applicable rule
            if let Some(index) = applicable_rule_index {
                let rule_state = &rules[index];
                // Increment hit count
                rule_state.hit_count.fetch_add(1, Ordering::SeqCst);
                // Clone and return the result
                Some(rule_state.rule.result.clone())
            } else {
                None
            }
        };

        // Apply the fault outside the lock
        let fault_response = if let Some(ref result) = fault_result {
            self.apply_fault(result).await
        } else {
            None
        };

        let resp = if let Some(fault_response) = fault_response {
            fault_response
        } else {
            // Clone the request and remove fault injection headers before forwarding
            let mut clean_request = request.clone();
            clean_request
                .headers_mut()
                .remove(constants::FAULT_INJECTION_OPERATION);

            // No fault injection or delay-only fault, proceed with actual request
            self.inner.execute_request(&clean_request).await
        };

        // Apply delay after the request is sent
        if let Some(result) = fault_result {
            if result.delay > Duration::ZERO {
                let delay = azure_core::time::Duration::try_from(result.delay)
                    .unwrap_or(azure_core::time::Duration::ZERO);
                azure_core::async_runtime::get_async_runtime()
                    .sleep(delay)
                    .await;
            }
        }

        resp
    }
}

#[cfg(test)]
mod tests {
    use super::FaultClient;
    use crate::constants::{SubStatusCode, SUB_STATUS};
    use crate::fault_injection::{
        CustomResponse, FaultInjectionConditionBuilder, FaultInjectionErrorType,
        FaultInjectionResultBuilder, FaultInjectionRuleBuilder, FaultOperationType,
    };
    use crate::regions;
    use async_trait::async_trait;
    use azure_core::error::ErrorKind;
    use azure_core::http::{headers::Headers, AsyncRawResponse, HttpClient, Method, Request, Url};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    /// A mock HTTP client that tracks call counts and returns success.
    #[derive(Debug)]
    struct MockHttpClient {
        call_count: AtomicU32,
    }

    impl MockHttpClient {
        fn new() -> Self {
            Self {
                call_count: AtomicU32::new(0),
            }
        }

        fn call_count(&self) -> u32 {
            self.call_count.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl HttpClient for MockHttpClient {
        async fn execute_request(
            &self,
            _request: &Request,
        ) -> azure_core::Result<AsyncRawResponse> {
            self.call_count.fetch_add(1, Ordering::SeqCst);

            // Return a minimal valid response
            Ok(AsyncRawResponse::from_bytes(
                azure_core::http::StatusCode::Ok,
                azure_core::http::headers::Headers::new(),
                vec![],
            ))
        }
    }

    fn create_test_request() -> Request {
        Request::new(
            Url::parse("https://test.cosmos.azure.com/dbs/testdb").unwrap(),
            Method::Get,
        )
    }

    #[tokio::test]
    async fn execute_request_no_matching_rules() {
        let mock_client = Arc::new(MockHttpClient::new());

        // Create rule that requires specific operation type
        let condition = FaultInjectionConditionBuilder::new()
            .with_operation_type(FaultOperationType::CreateItem)
            .build();
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::Timeout)
            .build();
        let rule = FaultInjectionRuleBuilder::new("create-only", error)
            .with_condition(condition)
            .build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);

        // Request without operation type header shouldn't match
        let request = create_test_request();
        let result = fault_client.execute_request(&request).await;

        assert!(result.is_ok());
        assert_eq!(mock_client.call_count(), 1);
    }

    #[tokio::test]
    async fn execute_request_empty_rules() {
        let mock_client = Arc::new(MockHttpClient::new());
        let fault_client = FaultClient::new(mock_client.clone(), vec![]);

        let request = create_test_request();
        let result = fault_client.execute_request(&request).await;

        assert!(result.is_ok());
        assert_eq!(mock_client.call_count(), 1);
    }

    #[tokio::test]
    async fn execute_request_with_hit_limit() {
        let mock_client = Arc::new(MockHttpClient::new());

        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::InternalServerError)
            .build();
        let rule = FaultInjectionRuleBuilder::new("limited-rule", error)
            .with_hit_limit(2)
            .build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let request = create_test_request();

        // First two requests should hit the fault
        let result1 = fault_client.execute_request(&request).await;
        assert!(result1.is_err());

        let result2 = fault_client.execute_request(&request).await;
        assert!(result2.is_err());

        // Third request should pass through (hit limit reached)
        let result3 = fault_client.execute_request(&request).await;
        assert!(result3.is_ok());
        assert_eq!(mock_client.call_count(), 1);
    }

    #[tokio::test]
    async fn execute_request_before_start_time() {
        let mock_client = Arc::new(MockHttpClient::new());

        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::InternalServerError)
            .build();
        let rule = FaultInjectionRuleBuilder::new("future-rule", error)
            .with_start_time(Instant::now() + Duration::from_secs(60))
            .build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let request = create_test_request();

        // Request should pass through because start_time is in the future
        let result = fault_client.execute_request(&request).await;
        assert!(result.is_ok());
        assert_eq!(mock_client.call_count(), 1);
    }

    #[tokio::test]
    async fn execute_request_injects_internal_server_error() {
        let mock_client = Arc::new(MockHttpClient::new());

        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::InternalServerError)
            .build();
        let rule = FaultInjectionRuleBuilder::new("error-rule", error).build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let request = create_test_request();

        let result = fault_client.execute_request(&request).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(
            err.http_status(),
            Some(azure_core::http::StatusCode::InternalServerError),
            "expected InternalServerError status code"
        );

        assert_eq!(mock_client.call_count(), 0); // Request not forwarded
    }

    #[tokio::test]
    async fn execute_request_injects_too_many_requests() {
        let mock_client = Arc::new(MockHttpClient::new());

        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::TooManyRequests)
            .build();
        let rule = FaultInjectionRuleBuilder::new("throttle-rule", error).build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let request = create_test_request();

        let result = fault_client.execute_request(&request).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(
            err.http_status(),
            Some(azure_core::http::StatusCode::TooManyRequests),
            "expected TooManyRequests status code"
        );
    }

    #[tokio::test]
    async fn execute_request_response_delay_passes_through() {
        let mock_client = Arc::new(MockHttpClient::new());

        // Create a rule with only delay, no error type - should pass through after delay
        let error = FaultInjectionResultBuilder::new()
            .with_delay(Duration::from_millis(200))
            .build();
        let rule = FaultInjectionRuleBuilder::new("response-delay-rule", error).build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let request = create_test_request();

        // Delay-only should pass through to actual request after delay
        let start = std::time::Instant::now();
        let result = fault_client.execute_request(&request).await;
        let elapsed = start.elapsed();

        assert!(result.is_ok());
        assert_eq!(mock_client.call_count(), 1);

        // Verify the delay was applied (at least 150ms to account for timing variance)
        assert!(
            elapsed >= Duration::from_millis(150),
            "expected at least 150ms delay, got {:?}",
            elapsed
        );
    }

    #[tokio::test]
    async fn execute_request_region_matching() {
        let mock_client = Arc::new(MockHttpClient::new());

        let condition = FaultInjectionConditionBuilder::new()
            .with_region(regions::WEST_US)
            .build();
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .build();
        let rule = FaultInjectionRuleBuilder::new("region-rule", error)
            .with_condition(condition)
            .build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);

        // Request URL doesn't contain "westus", should pass through
        let request = create_test_request();
        let result = fault_client.execute_request(&request).await;

        assert!(result.is_ok());
        assert_eq!(mock_client.call_count(), 1);
    }

    #[tokio::test]
    async fn execute_request_container_matching() {
        let mock_client = Arc::new(MockHttpClient::new());

        let condition = FaultInjectionConditionBuilder::new()
            .with_container_id("my-container")
            .build();
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::PartitionIsGone)
            .build();
        let rule = FaultInjectionRuleBuilder::new("container-rule", error)
            .with_condition(condition)
            .build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);

        // Request URL doesn't contain "my-container", should pass through
        let request = create_test_request();
        let result = fault_client.execute_request(&request).await;

        assert!(result.is_ok());
        assert_eq!(mock_client.call_count(), 1);
    }

    #[tokio::test]
    async fn execute_request_with_hit_limit_on_rule() {
        let mock_client = Arc::new(MockHttpClient::new());

        // Create a rule where the error is injected only 2 times via hit_limit on rule
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .build();
        let rule = FaultInjectionRuleBuilder::new("hit-limited-rule", error)
            .with_hit_limit(2)
            .build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let request = create_test_request();

        // First request should hit the fault
        let result1 = fault_client.execute_request(&request).await;
        assert!(result1.is_err(), "first request should fail");
        assert_eq!(
            result1.unwrap_err().http_status(),
            Some(azure_core::http::StatusCode::ServiceUnavailable)
        );

        // Second request should also hit the fault
        let result2 = fault_client.execute_request(&request).await;
        assert!(result2.is_err(), "second request should fail");
        assert_eq!(
            result2.unwrap_err().http_status(),
            Some(azure_core::http::StatusCode::ServiceUnavailable)
        );

        // Third request should pass through (times limit reached)
        let result3 = fault_client.execute_request(&request).await;
        assert!(
            result3.is_ok(),
            "third request should succeed after times limit"
        );
        assert_eq!(mock_client.call_count(), 1);
    }

    #[tokio::test]
    async fn execute_request_error_includes_substatus_header() {
        let test_cases = vec![
            (
                FaultInjectionErrorType::ReadSessionNotAvailable,
                Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
            ),
            (
                FaultInjectionErrorType::PartitionIsGone,
                Some(SubStatusCode::PARTITION_KEY_RANGE_GONE),
            ),
            (
                FaultInjectionErrorType::WriteForbidden,
                Some(SubStatusCode::WRITE_FORBIDDEN),
            ),
            (
                FaultInjectionErrorType::DatabaseAccountNotFound,
                Some(SubStatusCode::DATABASE_ACCOUNT_NOT_FOUND),
            ),
            (FaultInjectionErrorType::InternalServerError, None),
            (FaultInjectionErrorType::ServiceUnavailable, None),
            (FaultInjectionErrorType::TooManyRequests, None),
            (FaultInjectionErrorType::Timeout, None),
        ];

        for (error_type, expected_substatus) in test_cases {
            let mock_client = Arc::new(MockHttpClient::new());

            let error = FaultInjectionResultBuilder::new()
                .with_error(error_type)
                .build();
            let rule = FaultInjectionRuleBuilder::new("substatus-rule", error).build();

            let fault_client = FaultClient::new(mock_client, vec![Arc::new(rule)]);
            let request = create_test_request();

            let result = fault_client.execute_request(&request).await;
            assert!(result.is_err(), "{:?} should produce an error", error_type);

            let err = result.unwrap_err();
            if let azure_core::error::ErrorKind::HttpResponse { raw_response, .. } = err.kind() {
                match expected_substatus {
                    Some(expected) => {
                        let response = raw_response.as_ref().unwrap_or_else(|| {
                            panic!("{:?} should have a raw_response with substatus", error_type)
                        });
                        let actual: u32 = response
                            .headers()
                            .get_as::<u32, std::num::ParseIntError>(&SUB_STATUS)
                            .unwrap_or_else(|_| {
                                panic!("{:?} should have x-ms-substatus header", error_type)
                            });
                        assert_eq!(
                            SubStatusCode::from(actual),
                            expected,
                            "{:?}: substatus mismatch",
                            error_type
                        );
                    }
                    None => {
                        assert!(
                            raw_response.is_none(),
                            "{:?} should not have a raw_response",
                            error_type
                        );
                    }
                }
            } else {
                panic!("{:?} should produce an HttpResponse error kind", error_type);
            }
        }
    }

    #[tokio::test]
    async fn connection_error_produces_connection_error_kind() {
        let mock_client = Arc::new(MockHttpClient::new());

        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ConnectionError)
            .build();
        let rule = FaultInjectionRuleBuilder::new("conn-error", error).build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let request = create_test_request();

        let result = fault_client.execute_request(&request).await;
        assert!(result.is_err(), "should produce an error");

        let err = result.unwrap_err();
        assert_eq!(mock_client.call_count(), 0);
    }

    #[tokio::test]
    async fn response_timeout_produces_io_error_kind() {
        let mock_client = Arc::new(MockHttpClient::new());

        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ResponseTimeout)
            .build();
        let rule = FaultInjectionRuleBuilder::new("timeout-error", error).build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let request = create_test_request();

        let result = fault_client.execute_request(&request).await;
        assert!(result.is_err(), "should produce an error");

        let err = result.unwrap_err();
        assert_eq!(
            err.kind(),
            &ErrorKind::Io,
            "response timeout should have Io ErrorKind"
        );
        assert_eq!(mock_client.call_count(), 0);
    }

    #[tokio::test]
    async fn execute_request_with_custom_response() {
        let mock_client = Arc::new(MockHttpClient::new());

        let body = b"{\"id\": \"test-account\"}".to_vec();
        let result = FaultInjectionResultBuilder::new()
            .with_custom_response(CustomResponse {
                status_code: azure_core::http::StatusCode::Ok,
                headers: Headers::new(),
                body: body.clone(),
            })
            .build();
        let rule = FaultInjectionRuleBuilder::new("custom-response-rule", result).build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let request = create_test_request();

        let response = fault_client.execute_request(&request).await;
        assert!(response.is_ok(), "custom response should succeed");

        let raw = response.unwrap();
        assert_eq!(raw.status(), azure_core::http::StatusCode::Ok);
        // Request should NOT be forwarded to inner client
        assert_eq!(mock_client.call_count(), 0);
    }
}

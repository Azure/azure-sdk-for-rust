use super::fault_injection_condition::FaultOperationType;
use super::fault_injection_result::{FaultInjectionErrorType, FaultInjectionResult};
use super::fault_injection_rule::FaultInjectionRule;
use crate::constants::{self, SubStatusCode};
use async_trait::async_trait;
use azure_core::error::ErrorKind;
use azure_core::http::{AsyncRawResponse, HttpClient, Request, StatusCode};
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
    /// The time when this client was created (for tracking rule start delays and durations).
    created_at: Instant,
}

/// Tracks the state of a fault injection rule.
#[derive(Debug)]
struct RuleState {
    /// The fault injection rule.
    rule: FaultInjectionRule,
    /// Number of times this rule has been applied.
    hit_count: AtomicU32,
    /// The time when this rule was first activated (after start_delay).
    activated_at: Option<Instant>,
}

impl FaultClient {
    /// Creates a new instance of the FaultClient.
    pub fn new(inner: Arc<dyn HttpClient>, rules: Vec<FaultInjectionRule>) -> Self {
        let rule_states = rules
            .into_iter()
            .map(|rule| RuleState {
                rule,
                hit_count: AtomicU32::new(0),
                activated_at: None,
            })
            .collect();

        Self {
            inner,
            rules: Arc::new(Mutex::new(rule_states)),
            created_at: Instant::now(),
        }
    }

    /// Checks if a rule is currently applicable based on timing constraints.
    fn is_rule_applicable(&self, rule_state: &mut RuleState) -> bool {
        let elapsed = self.created_at.elapsed();
        let rule = &rule_state.rule;

        // Check if we've passed the start delay
        if elapsed < rule.start_delay {
            return false;
        }

        // Activate the rule if not already activated
        if rule_state.activated_at.is_none() {
            rule_state.activated_at = Some(Instant::now());
        }

        // Check if the rule has exceeded its duration
        if let Some(activated_at) = rule_state.activated_at {
            if activated_at.elapsed() > rule.duration {
                return false;
            }
        }

        // Check if we've exceeded the hit limit
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
        if let Some(ref expected_op) = condition.operation_type {
            let request_op = request
                .headers()
                .get_optional_str(&constants::FAULT_INJECTION_OPERATION)
                .and_then(FaultOperationType::parse);

            match request_op {
                Some(op) if op == *expected_op => {
                    // Operation type matches, continue checking other conditions
                }
                _ => {
                    // Operation type doesn't match or header not present
                    matches = false;
                }
            }
        }

        // Check region if specified
        if let Some(ref region) = condition.region {
            if !request.url().as_str().contains(region.as_str()) {
                matches = false;
            }
        }

        // Check container ID if specified
        // in the future
        if let Some(ref container_id) = condition.container_id {
            if !request.url().as_str().contains(container_id) {
                matches = false;
            }
        }

        matches
    }

    /// Applies the fault injection result and returns an error or modifies the response.
    async fn apply_fault(
        &self,
        server_error: FaultInjectionResult,
    ) -> Option<azure_core::Result<AsyncRawResponse>> {
        // Check probability
        if server_error.probability < 1.0 {
            // Use a simple time-based pseudo-random check
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos();
            let random = (nanos % 1000) as f32 / 1000.0;
            if !server_error.probability.is_finite()
                || server_error.probability <= 0.0
                || random >= server_error.probability
            {
                return None; // Don't inject fault this time
            }
        }

        // Apply delay before sending request or injecting error
        if server_error.delay > Duration::ZERO {
            // Convert std::time::Duration to azure_core::time::Duration for sleep
            let delay = azure_core::time::Duration::try_from(server_error.delay)
                .unwrap_or(azure_core::time::Duration::ZERO);
            azure_core::async_runtime::get_async_runtime()
                .sleep(delay)
                .await;
        }

        // Generate the appropriate error based on error type
        let error_type = match server_error.error_type {
            Some(et) => et,
            None => return None, // No error type set, pass through
        };

        let (status_code, _sub_status, message) = match error_type {
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

        let error = azure_core::Error::with_message(
            ErrorKind::HttpResponse {
                status: status_code,
                error_code: Some("Injected Fault".to_string()),
                raw_response: None,
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
            let mut rules = self.rules.lock().unwrap();
            let mut applicable_rule_index: Option<usize> = None;

            for (index, rule_state) in rules.iter_mut().enumerate() {
                if self.is_rule_applicable(rule_state)
                    && self.matches_condition(request, &rule_state.rule)
                {
                    applicable_rule_index = Some(index);
                    break;
                }
            }

            // Apply fault if we found an applicable rule
            if let Some(index) = applicable_rule_index {
                let rule_state = &mut rules[index];
                // Increment hit count
                rule_state.hit_count.fetch_add(1, Ordering::SeqCst);
                // Clone and return the result (dereference the Box)
                Some((*rule_state.rule.result).clone())
            } else {
                None
            }
        };

        // Apply the fault outside the lock
        if let Some(ref result) = fault_result {
            if let Some(fault_response) = self.apply_fault(result.clone()).await {
                return fault_response;
            }
        }

        // Clone the request and remove fault injection headers before forwarding
        let mut clean_request = request.clone();
        clean_request
            .headers_mut()
            .remove(constants::FAULT_INJECTION_OPERATION);

        // No fault injection or delay-only fault, proceed with actual request
        let resp = self.inner.execute_request(&clean_request).await;

        // Apply response delay if configured
        if let Some(result) = fault_result {
            // Convert std::time::Duration to azure_core::time::Duration for sleep
            let delay = azure_core::time::Duration::try_from(result.delay)
                .unwrap_or(azure_core::time::Duration::ZERO);
            azure_core::async_runtime::get_async_runtime()
                .sleep(delay)
                .await;
        }

        resp
    }
}

#[cfg(test)]
mod tests {
    use super::FaultClient;
    use crate::fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
        FaultInjectionRuleBuilder, FaultOperationType,
    };
    use crate::regions;
    use async_trait::async_trait;
    use azure_core::http::{AsyncRawResponse, HttpClient, Method, Request, Url};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

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

        let fault_client = FaultClient::new(mock_client.clone(), vec![rule]);

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

        let fault_client = FaultClient::new(mock_client.clone(), vec![rule]);
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
    async fn execute_request_with_start_delay() {
        let mock_client = Arc::new(MockHttpClient::new());

        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::InternalServerError)
            .build();
        let rule = FaultInjectionRuleBuilder::new("delayed-rule", error)
            .with_start_delay(Duration::from_secs(60)) // Long delay
            .build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![rule]);
        let request = create_test_request();

        // Request should pass through because start delay hasn't elapsed
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

        let fault_client = FaultClient::new(mock_client.clone(), vec![rule]);
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

        let fault_client = FaultClient::new(mock_client.clone(), vec![rule]);
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

        let fault_client = FaultClient::new(mock_client.clone(), vec![rule]);
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

        let fault_client = FaultClient::new(mock_client.clone(), vec![rule]);

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

        let fault_client = FaultClient::new(mock_client.clone(), vec![rule]);

        // Request URL doesn't contain "my-container", should pass through
        let request = create_test_request();
        let result = fault_client.execute_request(&request).await;

        assert!(result.is_ok());
        assert_eq!(mock_client.call_count(), 1);
    }
}

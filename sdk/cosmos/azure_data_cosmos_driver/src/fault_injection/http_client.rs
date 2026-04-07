// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP client wrapper that evaluates fault injection rules on each request.

use super::result::FaultInjectionResult;
use super::rule::FaultInjectionRule;
use super::FaultInjectionErrorType;
use super::FaultInjectionEvaluation;
use super::FaultOperationType;
use crate::diagnostics::RequestSentStatus;
use crate::driver::transport::cosmos_transport_client::{
    HttpRequest, HttpResponse, TransportClient, TransportError,
};
use crate::models::cosmos_headers::fault_injection_header_names::FAULT_INJECTION_OPERATION;
use crate::models::cosmos_headers::response_header_names::SUBSTATUS;
use crate::models::SubStatusCode;
use async_trait::async_trait;
use azure_core::error::ErrorKind;
use azure_core::http::headers::Headers;
use azure_core::http::{RawResponse, StatusCode};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Result of attempting to apply a fault injection rule.
enum ApplyResult {
    /// Fault was injected — return this response/error to the caller.
    Injected(Result<HttpResponse, TransportError>),
    /// Rule matched but the probability check failed.
    ProbabilityMiss,
    /// Rule had no error_type and no custom_response — effectively no-op.
    /// This happens when a rule only has delay (delay was already applied).
    NoEffect,
}

/// Custom implementation of a transport client that injects faults for testing purposes.
#[derive(Debug)]
pub struct FaultClient {
    /// The inner transport client to which requests are delegated.
    inner: Arc<dyn TransportClient>,
    /// The fault injection rules to apply.
    rules: Arc<Vec<Arc<FaultInjectionRule>>>,
}

impl FaultClient {
    /// Creates a new instance of the FaultClient.
    pub(crate) fn new(
        inner: Arc<dyn TransportClient>,
        rules: Vec<Arc<FaultInjectionRule>>,
    ) -> Self {
        Self {
            inner,
            rules: Arc::new(rules),
        }
    }

    /// Checks if a rule is currently applicable based on timing constraints.
    /// Returns `None` if applicable, or `Some(evaluation)` with the skip reason.
    fn evaluate_applicability(
        &self,
        rule: &FaultInjectionRule,
    ) -> Option<FaultInjectionEvaluation> {
        let now = Instant::now();

        if !rule.is_enabled() {
            return Some(FaultInjectionEvaluation::Disabled {
                rule_id: rule.id().to_owned(),
            });
        }

        if let Some(start_time) = rule.start_time() {
            if now < start_time {
                return Some(FaultInjectionEvaluation::BeforeStartTime {
                    rule_id: rule.id().to_owned(),
                });
            }
        }

        if let Some(end_time) = rule.end_time() {
            if now >= end_time {
                return Some(FaultInjectionEvaluation::AfterEndTime {
                    rule_id: rule.id().to_owned(),
                });
            }
        }

        if let Some(hit_limit) = rule.hit_limit() {
            let hit_count = rule.hit_count();
            if hit_count >= hit_limit {
                return Some(FaultInjectionEvaluation::HitLimitExhausted {
                    rule_id: rule.id().to_owned(),
                    hit_count,
                    hit_limit,
                });
            }
        }

        None // Rule is applicable
    }

    /// Checks if the request matches the rule's condition.
    /// Returns `None` if it matches, or `Some(evaluation)` with the mismatch reason.
    fn evaluate_condition(
        &self,
        request: &HttpRequest,
        rule: &FaultInjectionRule,
    ) -> Option<FaultInjectionEvaluation> {
        let condition = rule.condition();

        if let Some(expected_op) = condition.operation_type() {
            let request_op = request
                .headers
                .get_optional_str(&FAULT_INJECTION_OPERATION)
                .and_then(|s| s.parse::<FaultOperationType>().ok());
            if request_op != Some(expected_op) {
                return Some(FaultInjectionEvaluation::OperationMismatch {
                    rule_id: rule.id().to_owned(),
                });
            }
        }

        if let Some(region) = condition.region() {
            if !request.url.as_str().contains(region.as_str()) {
                return Some(FaultInjectionEvaluation::RegionMismatch {
                    rule_id: rule.id().to_owned(),
                });
            }
        }

        if let Some(container_id) = condition.container_id() {
            if !request.url.as_str().contains(container_id) {
                return Some(FaultInjectionEvaluation::ContainerMismatch {
                    rule_id: rule.id().to_owned(),
                });
            }
        }

        None // Condition matches
    }

    /// Applies the fault injection result and returns an error or modifies the response.
    ///
    /// This method handles the full fault lifecycle: probability check, delay application,
    /// and fault response generation. If the probability check fails, no fault is injected
    /// and no delay is applied.
    ///
    /// When a fault is injected, pushes an `Applied` evaluation to the provided
    /// `evaluations` list. The caller writes these into the evaluation collector
    /// attached to the request, which the transport pipeline reads after the
    /// request completes.
    async fn apply_fault(
        &self,
        server_error: &FaultInjectionResult,
        rule: &FaultInjectionRule,
        evaluations: &mut Vec<FaultInjectionEvaluation>,
    ) -> ApplyResult {
        // Check probability
        if server_error.probability() < 1.0 {
            let random: f32 = rand::random();
            if !server_error.probability().is_finite()
                || server_error.probability() <= 0.0
                || random >= server_error.probability()
            {
                return ApplyResult::ProbabilityMiss;
            }
        }

        // Probability check passed — count this as a hit.
        rule.increment_hit_count();
        let rule_id = rule.id();

        // Record that the rule was applied before serializing evaluations.
        evaluations.push(FaultInjectionEvaluation::Applied {
            rule_id: rule_id.to_owned(),
        });

        // Apply delay if configured (only when fault is actually injected).
        if let Some(delay) = server_error.delay() {
            if delay > Duration::ZERO {
                let delay = azure_core::time::Duration::try_from(delay)
                    .unwrap_or(azure_core::time::Duration::ZERO);
                azure_core::sleep(delay).await;
            }
        }

        // Check for custom response first (takes precedence over error injection)
        if let Some(custom) = server_error.custom_response() {
            let headers = custom.headers().clone();
            return ApplyResult::Injected(Ok(HttpResponse {
                status: u16::from(custom.status_code()),
                headers,
                body: custom.body().to_vec(),
            }));
        }

        // Generate the appropriate error based on error type
        let error_type = match server_error.error_type() {
            Some(et) => et,
            None => return ApplyResult::NoEffect,
        };

        // Connection-level faults return simple errors with the appropriate ErrorKind.
        // Evaluations are propagated via the evaluation collector attached to the request for all paths.
        let (status_code, sub_status, message) = match error_type {
            FaultInjectionErrorType::ConnectionError => {
                return ApplyResult::Injected(Err(TransportError::new(
                    azure_core::Error::with_message(
                        ErrorKind::Connection,
                        "Injected fault: connection error",
                    ),
                    RequestSentStatus::NotSent,
                )));
            }
            FaultInjectionErrorType::ResponseTimeout => {
                return ApplyResult::Injected(Err(TransportError::new(
                    azure_core::Error::with_message(
                        ErrorKind::Io,
                        "Injected fault: response timeout",
                    ),
                    RequestSentStatus::Unknown,
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

        let mut headers = Headers::new();
        if let Some(ss) = sub_status {
            headers.insert(SUBSTATUS.clone(), ss.value().to_string());
        }
        let raw_response = Box::new(RawResponse::from_bytes(
            status_code,
            headers.clone(),
            vec![],
        ));

        let error = azure_core::Error::with_message(
            ErrorKind::HttpResponse {
                status: status_code,
                error_code: Some("Injected Fault".to_string()),
                raw_response: Some(raw_response),
            },
            message,
        );

        ApplyResult::Injected(Err(TransportError::new(error, RequestSentStatus::Sent)))
    }
}

#[async_trait]
impl TransportClient for FaultClient {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        let mut evaluations: Vec<FaultInjectionEvaluation> = Vec::new();
        let mut matched_rule: Option<Arc<FaultInjectionRule>> = None;

        for rule in self.rules.iter() {
            // Check applicability (timing, enabled, hit limit)
            if let Some(skip_reason) = self.evaluate_applicability(rule) {
                evaluations.push(skip_reason);
                continue;
            }

            // Check condition match (operation, region, container)
            if let Some(mismatch) = self.evaluate_condition(request, rule) {
                evaluations.push(mismatch);
                continue;
            }

            // Rule is applicable and matches — use it (first match wins)
            if matched_rule.is_none() {
                matched_rule = Some(Arc::clone(rule));
            } else {
                evaluations.push(FaultInjectionEvaluation::Superseded {
                    rule_id: rule.id().to_owned(),
                });
            }
        }

        // Apply the fault if we found a matching rule.
        // `apply_fault` pushes the Applied evaluation; all evaluations are then
        // written into the request's evaluation collector.
        let fault_response = if let Some(ref rule) = matched_rule {
            match self
                .apply_fault(rule.result(), rule, &mut evaluations)
                .await
            {
                ApplyResult::Injected(response) => Some(response),
                ApplyResult::ProbabilityMiss => {
                    evaluations.push(FaultInjectionEvaluation::ProbabilityMiss {
                        rule_id: rule.id().to_owned(),
                        probability: rule.result().probability(),
                    });
                    None
                }
                ApplyResult::NoEffect => None,
            }
        } else {
            None
        };

        // Write evaluations into the collector and log them.
        if !evaluations.is_empty() {
            tracing::trace!(
                evaluations = ?evaluations,
                "fault injection rule evaluation"
            );

            if let Some(ref collector) = request.evaluation_collector {
                collector.push_all(&mut evaluations);
            }
        }

        if let Some(fault_response) = fault_response {
            fault_response
        } else {
            // Build a clean request without the fault injection operation header
            // before forwarding to the real transport.
            let mut clean_headers = request.headers.clone();
            clean_headers.remove(FAULT_INJECTION_OPERATION.clone());

            // Collector intentionally omitted: evaluations already captured above.
            let clean_request = HttpRequest {
                url: request.url.clone(),
                method: request.method,
                headers: clean_headers,
                body: request.body.clone(),
                timeout: request.timeout,
                evaluation_collector: None,
            };

            // No fault injection, proceed with actual request
            self.inner.send(&clean_request).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FaultClient;
    use crate::driver::transport::cosmos_transport_client::{
        HttpRequest, HttpResponse, TransportClient, TransportError,
    };
    use crate::fault_injection::{
        CustomResponseBuilder, EvaluationCollector, FaultInjectionConditionBuilder,
        FaultInjectionErrorType, FaultInjectionEvaluation, FaultInjectionResultBuilder,
        FaultInjectionRuleBuilder, FaultOperationType,
    };
    use crate::models::cosmos_headers::fault_injection_header_names::FAULT_INJECTION_OPERATION;
    use crate::models::cosmos_headers::response_header_names::SUBSTATUS;
    use crate::models::SubStatusCode;
    use crate::options::Region;
    use async_trait::async_trait;
    use azure_core::error::ErrorKind;
    use azure_core::http::{headers::Headers, Method, Url};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    /// A mock transport client that tracks call counts and returns success.
    #[derive(Debug)]
    struct MockTransportClient {
        call_count: AtomicU32,
    }

    impl MockTransportClient {
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
    impl TransportClient for MockTransportClient {
        async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            self.call_count.fetch_add(1, Ordering::SeqCst);

            // Return a minimal valid response
            Ok(HttpResponse {
                status: 200,
                headers: Headers::new(),
                body: vec![],
            })
        }
    }

    fn create_test_request() -> (HttpRequest, EvaluationCollector) {
        let collector = EvaluationCollector::default();
        let request = HttpRequest {
            url: Url::parse("https://test.cosmos.azure.com/dbs/testdb").unwrap(),
            method: Method::Get,
            headers: Headers::new(),
            body: None,
            timeout: None,
            evaluation_collector: Some(collector.clone()),
        };
        (request, collector)
    }

    #[tokio::test]
    async fn execute_request_no_matching_rules() {
        let mock_client = Arc::new(MockTransportClient::new());

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
        let (request, _collector) = create_test_request();
        let result = fault_client.send(&request).await;

        assert!(result.is_ok());
        assert_eq!(mock_client.call_count(), 1);
    }

    #[tokio::test]
    async fn execute_request_empty_rules() {
        let mock_client = Arc::new(MockTransportClient::new());
        let fault_client = FaultClient::new(mock_client.clone(), vec![]);

        let (request, _collector) = create_test_request();
        let result = fault_client.send(&request).await;

        assert!(result.is_ok());
        assert_eq!(mock_client.call_count(), 1);
    }

    #[tokio::test]
    async fn execute_request_with_hit_limit() {
        let mock_client = Arc::new(MockTransportClient::new());

        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::InternalServerError)
            .build();
        let rule = FaultInjectionRuleBuilder::new("limited-rule", error)
            .with_hit_limit(2)
            .build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let (request, _collector) = create_test_request();

        // First two requests should hit the fault
        let result1 = fault_client.send(&request).await;
        assert!(result1.is_err());

        let result2 = fault_client.send(&request).await;
        assert!(result2.is_err());

        // Third request should pass through (hit limit reached)
        let result3 = fault_client.send(&request).await;
        assert!(result3.is_ok());
        assert_eq!(mock_client.call_count(), 1);
    }

    #[tokio::test]
    async fn execute_request_before_start_time() {
        let mock_client = Arc::new(MockTransportClient::new());

        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::InternalServerError)
            .build();
        let rule = FaultInjectionRuleBuilder::new("future-rule", error)
            .with_start_time(Instant::now() + Duration::from_secs(60))
            .build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let (request, _collector) = create_test_request();

        // Request should pass through because start_time is in the future
        let result = fault_client.send(&request).await;
        assert!(result.is_ok());
        assert_eq!(mock_client.call_count(), 1);
    }

    #[tokio::test]
    async fn execute_request_injects_internal_server_error() {
        let mock_client = Arc::new(MockTransportClient::new());

        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::InternalServerError)
            .build();
        let rule = FaultInjectionRuleBuilder::new("error-rule", error).build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let (request, _collector) = create_test_request();

        let result = fault_client.send(&request).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(
            err.error.http_status(),
            Some(azure_core::http::StatusCode::InternalServerError),
            "expected InternalServerError status code"
        );

        assert_eq!(mock_client.call_count(), 0); // Request not forwarded
    }

    #[tokio::test]
    async fn execute_request_injects_too_many_requests() {
        let mock_client = Arc::new(MockTransportClient::new());

        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::TooManyRequests)
            .build();
        let rule = FaultInjectionRuleBuilder::new("throttle-rule", error).build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let (request, _collector) = create_test_request();

        let result = fault_client.send(&request).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(
            err.error.http_status(),
            Some(azure_core::http::StatusCode::TooManyRequests),
            "expected TooManyRequests status code"
        );
    }

    #[tokio::test]
    async fn execute_request_response_delay_passes_through() {
        let mock_client = Arc::new(MockTransportClient::new());

        // Create a rule with only delay, no error type - should pass through after delay
        let error = FaultInjectionResultBuilder::new()
            .with_delay(Duration::from_millis(200))
            .build();
        let rule = FaultInjectionRuleBuilder::new("response-delay-rule", error).build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let (request, _collector) = create_test_request();

        // Delay-only should pass through to actual request after delay
        let start = std::time::Instant::now();
        let result = fault_client.send(&request).await;
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
        let mock_client = Arc::new(MockTransportClient::new());

        let condition = FaultInjectionConditionBuilder::new()
            .with_region(Region::WEST_US)
            .build();
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .build();
        let rule = FaultInjectionRuleBuilder::new("region-rule", error)
            .with_condition(condition)
            .build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);

        // Request URL doesn't contain "westus", should pass through
        let (request, _collector) = create_test_request();
        let result = fault_client.send(&request).await;

        assert!(result.is_ok());
        assert_eq!(mock_client.call_count(), 1);
    }

    #[tokio::test]
    async fn execute_request_container_matching() {
        let mock_client = Arc::new(MockTransportClient::new());

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
        let (request, _collector) = create_test_request();
        let result = fault_client.send(&request).await;

        assert!(result.is_ok());
        assert_eq!(mock_client.call_count(), 1);
    }

    #[tokio::test]
    async fn execute_request_with_hit_limit_on_rule() {
        let mock_client = Arc::new(MockTransportClient::new());

        // Create a rule where the error is injected only 2 times via hit_limit on rule
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .build();
        let rule = FaultInjectionRuleBuilder::new("hit-limited-rule", error)
            .with_hit_limit(2)
            .build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let (request, _collector) = create_test_request();

        // First request should hit the fault
        let result1 = fault_client.send(&request).await;
        assert!(result1.is_err(), "first request should fail");
        assert_eq!(
            result1.unwrap_err().error.http_status(),
            Some(azure_core::http::StatusCode::ServiceUnavailable)
        );

        // Second request should also hit the fault
        let result2 = fault_client.send(&request).await;
        assert!(result2.is_err(), "second request should fail");
        assert_eq!(
            result2.unwrap_err().error.http_status(),
            Some(azure_core::http::StatusCode::ServiceUnavailable)
        );

        // Third request should pass through (times limit reached)
        let result3 = fault_client.send(&request).await;
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
            let mock_client = Arc::new(MockTransportClient::new());

            let error = FaultInjectionResultBuilder::new()
                .with_error(error_type)
                .build();
            let rule = FaultInjectionRuleBuilder::new("substatus-rule", error).build();

            let fault_client = FaultClient::new(mock_client, vec![Arc::new(rule)]);
            let (request, _collector) = create_test_request();

            let result = fault_client.send(&request).await;
            assert!(result.is_err(), "{:?} should produce an error", error_type);

            let err = result.unwrap_err();
            if let azure_core::error::ErrorKind::HttpResponse { raw_response, .. } =
                err.error.kind()
            {
                let response = raw_response
                    .as_ref()
                    .unwrap_or_else(|| panic!("{:?} should have a raw_response", error_type));

                match expected_substatus {
                    Some(expected) => {
                        let actual: u32 = response
                            .headers()
                            .get_as::<u32, std::num::ParseIntError>(&SUBSTATUS)
                            .unwrap_or_else(|_| {
                                panic!("{:?} should have x-ms-substatus header", error_type)
                            });
                        assert_eq!(
                            SubStatusCode::new(actual),
                            expected,
                            "{:?}: substatus mismatch",
                            error_type
                        );
                    }
                    None => {
                        let substatus_header = response.headers().get_optional_str(&SUBSTATUS);
                        assert!(
                            substatus_header.is_none(),
                            "{:?} should not have x-ms-substatus header",
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
        let mock_client = Arc::new(MockTransportClient::new());

        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ConnectionError)
            .build();
        let rule = FaultInjectionRuleBuilder::new("conn-error", error).build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let (request, _collector) = create_test_request();

        let result = fault_client.send(&request).await;
        assert!(result.is_err(), "should produce an error");

        let err = result.unwrap_err();
        assert_eq!(
            err.error.kind(),
            &ErrorKind::Connection,
            "connection error should have Connection ErrorKind"
        );
        assert_eq!(mock_client.call_count(), 0);
    }

    #[tokio::test]
    async fn response_timeout_produces_io_error_kind() {
        let mock_client = Arc::new(MockTransportClient::new());

        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ResponseTimeout)
            .build();
        let rule = FaultInjectionRuleBuilder::new("timeout-error", error).build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let (request, _collector) = create_test_request();

        let result = fault_client.send(&request).await;
        assert!(result.is_err(), "should produce an error");

        let err = result.unwrap_err();
        assert_eq!(
            err.error.kind(),
            &ErrorKind::Io,
            "response timeout should have Io ErrorKind"
        );
        assert_eq!(mock_client.call_count(), 0);
    }

    #[tokio::test]
    async fn execute_request_with_custom_response() {
        let mock_client = Arc::new(MockTransportClient::new());

        let body = b"{\"id\": \"test-account\"}".to_vec();
        let result = FaultInjectionResultBuilder::new()
            .with_custom_response(
                CustomResponseBuilder::new(azure_core::http::StatusCode::Ok)
                    .with_body(body.clone())
                    .build(),
            )
            .build();
        let rule = FaultInjectionRuleBuilder::new("custom-response-rule", result).build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);
        let (request, _collector) = create_test_request();

        let response = fault_client.send(&request).await;
        assert!(response.is_ok(), "custom response should succeed");

        let raw = response.unwrap();
        assert_eq!(raw.status, 200);
        // Request should NOT be forwarded to inner client
        assert_eq!(mock_client.call_count(), 0);
    }

    #[tokio::test]
    async fn execute_request_with_matching_operation_header() {
        let mock_client = Arc::new(MockTransportClient::new());

        let condition = FaultInjectionConditionBuilder::new()
            .with_operation_type(FaultOperationType::ReadItem)
            .build();
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .build();
        let rule = FaultInjectionRuleBuilder::new("op-match-rule", error)
            .with_condition(condition)
            .build();

        let fault_client = FaultClient::new(mock_client.clone(), vec![Arc::new(rule)]);

        let (mut request, _collector) = create_test_request();
        request
            .headers
            .insert(FAULT_INJECTION_OPERATION.clone(), "ReadItem");

        let result = fault_client.send(&request).await;
        assert!(
            result.is_err(),
            "should inject fault for matching operation"
        );
        assert_eq!(mock_client.call_count(), 0);
    }

    #[tokio::test]
    async fn custom_response_evaluation_propagated() {
        let mock_client = Arc::new(MockTransportClient::new());

        let result = FaultInjectionResultBuilder::new()
            .with_custom_response(
                CustomResponseBuilder::new(azure_core::http::StatusCode::Ok)
                    .with_body(b"test")
                    .build(),
            )
            .build();
        let rule = FaultInjectionRuleBuilder::new("header-test-rule", result).build();

        let fault_client = FaultClient::new(mock_client, vec![Arc::new(rule)]);
        let (request, collector) = create_test_request();

        let response = fault_client.send(&request).await;
        assert!(response.is_ok());

        let evals = collector.take();
        assert_eq!(evals.len(), 1);
        assert!(evals[0].was_applied());
        assert_eq!(evals[0].rule_id(), "header-test-rule");
    }

    #[tokio::test]
    async fn evaluation_records_disabled_rule() {
        let mock_client = Arc::new(MockTransportClient::new());
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .build();
        let rule = Arc::new(FaultInjectionRuleBuilder::new("disabled-rule", error).build());
        rule.disable();

        let fault_client = FaultClient::new(mock_client, vec![rule]);
        let (request, collector) = create_test_request();
        let result = fault_client.send(&request).await;
        assert!(result.is_ok(), "Request should succeed with disabled rule");

        // Verify evaluation was recorded
        let evals = collector.take();
        assert_eq!(
            evals.len(),
            1,
            "Should have one evaluation for the disabled rule"
        );
        assert!(
            matches!(&evals[0], FaultInjectionEvaluation::Disabled { rule_id } if rule_id == "disabled-rule"),
            "Should record Disabled evaluation, got: {:?}",
            evals[0]
        );
    }

    #[tokio::test]
    async fn evaluations_propagated_for_http_fault() {
        let mock_client = Arc::new(MockTransportClient::new());
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .build();
        let rule = FaultInjectionRuleBuilder::new("test-rule", error).build();
        let fault_client = FaultClient::new(mock_client, vec![Arc::new(rule)]);

        let (request, collector) = create_test_request();
        let _ = fault_client.send(&request).await;

        let evals = collector.take();
        assert_eq!(evals.len(), 1);
        assert!(evals[0].was_applied());
        assert_eq!(evals[0].rule_id(), "test-rule");
    }

    #[tokio::test]
    async fn evaluations_propagated_for_connection_error() {
        let mock_client = Arc::new(MockTransportClient::new());
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ConnectionError)
            .build();
        let rule = FaultInjectionRuleBuilder::new("conn-rule", error).build();
        let fault_client = FaultClient::new(mock_client, vec![Arc::new(rule)]);

        let (request, collector) = create_test_request();
        let _ = fault_client.send(&request).await;

        let evals = collector.take();
        assert_eq!(evals.len(), 1);
        assert!(evals[0].was_applied());
        assert_eq!(evals[0].rule_id(), "conn-rule");
    }

    #[tokio::test]
    async fn evaluations_propagated_for_response_timeout() {
        let mock_client = Arc::new(MockTransportClient::new());
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ResponseTimeout)
            .build();
        let rule = FaultInjectionRuleBuilder::new("timeout-rule", error).build();
        let fault_client = FaultClient::new(mock_client, vec![Arc::new(rule)]);

        let (request, collector) = create_test_request();
        let _ = fault_client.send(&request).await;

        let evals = collector.take();
        assert_eq!(evals.len(), 1);
        assert!(evals[0].was_applied());
        assert_eq!(evals[0].rule_id(), "timeout-rule");
    }

    #[tokio::test]
    async fn evaluations_include_disabled_and_superseded() {
        let mock_client = Arc::new(MockTransportClient::new());

        let error1 = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .build();
        let error2 = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::InternalServerError)
            .build();
        let error3 = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::Timeout)
            .build();

        let rule1 = Arc::new(FaultInjectionRuleBuilder::new("disabled-rule", error1).build());
        rule1.disable();
        let rule2 = Arc::new(FaultInjectionRuleBuilder::new("active-rule", error2).build());
        let rule3 = Arc::new(FaultInjectionRuleBuilder::new("superseded-rule", error3).build());

        let fault_client = FaultClient::new(mock_client, vec![rule1, rule2, rule3]);
        let (request, collector) = create_test_request();
        let _ = fault_client.send(&request).await;

        let evals = collector.take();
        assert_eq!(evals.len(), 3);

        // Iteration order: Disabled first, then Superseded (third rule), then Applied (second rule).
        assert!(matches!(
            &evals[0],
            super::FaultInjectionEvaluation::Disabled { rule_id } if rule_id == "disabled-rule"
        ));
        assert!(matches!(
            &evals[1],
            super::FaultInjectionEvaluation::Superseded { rule_id } if rule_id == "superseded-rule"
        ));
        assert!(matches!(
            &evals[2],
            super::FaultInjectionEvaluation::Applied { rule_id } if rule_id == "active-rule"
        ));
    }

    #[tokio::test]
    async fn evaluations_propagated_for_no_match() {
        let mock_client = Arc::new(MockTransportClient::new());

        let condition = FaultInjectionConditionBuilder::new()
            .with_operation_type(FaultOperationType::CreateItem)
            .build();
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .build();
        let rule = FaultInjectionRuleBuilder::new("no-match-rule", error)
            .with_condition(condition)
            .build();

        let fault_client = FaultClient::new(mock_client, vec![Arc::new(rule)]);

        // Request without matching operation header
        let (request, collector) = create_test_request();
        let _ = fault_client.send(&request).await;

        let evals = collector.take();
        assert_eq!(evals.len(), 1);
        assert!(matches!(
            &evals[0],
            super::FaultInjectionEvaluation::OperationMismatch { rule_id } if rule_id == "no-match-rule"
        ));
    }
}

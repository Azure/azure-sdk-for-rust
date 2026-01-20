use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use async_trait::async_trait;
use rand::Rng;
use azure_core::http::{AsyncRawResponse, HttpClient, Request, StatusCode};
use azure_core::error::ErrorKind;
use crate::constants::{FAULT_INJECTION_CONTAINER_ID, FAULT_INJECTION_OPERATION, PARTITION_KEY_RANGE_ID};
use crate::fault_injection::fault_injection_utils::{
    FaultInjectionRule, FaultInjectionServerErrorType, FaultInjectionResult,
};

/// Custom implementation of an HTTP client that injects faults for testing purposes.
pub struct FaultClient {
    /// The inner HTTP client to which requests are delegated.
    inner: Arc<dyn HttpClient>,
    /// The fault injection rules to apply.
    rules: Arc<Mutex<Vec<RuleState>>>,
    /// The time when this client was created (for tracking rule start delays and durations).
    created_at: Instant,
}

/// Tracks the state of a fault injection rule.
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
    pub fn new(
        inner: Arc<dyn HttpClient>,
        rules: VecDeque<FaultInjectionRule>,
    ) -> Self {
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
    fn matches_condition(&self, _request: &Request, rule: &FaultInjectionRule) -> bool {
        let condition = &rule.condition;
        if condition.endpoints.unwrap().contains(_request.url()) {
            return true;
        }
        if _request.url().as_str().contains(condition.region) {
            return true;
        }
        if _request.headers().get_str(FAULT_INJECTION_OPERATION) == Some(&condition.operation_type) {
            return true;
        }
        if _request.headers().get_str(FAULT_INJECTION_CONTAINER_ID) == Some(&condition.container_id) {
            return true;
        }
        if _request.headers().get_str(PARTITION_KEY_RANGE_ID) == Some(&condition.partition_key_range_id) {
            return true;
        }
        return false;
    }

    /// Applies the fault injection result and returns an error or modifies the response.
    async fn apply_fault(
        &self,
        result: &dyn FaultInjectionResult,
    ) -> Option<azure_core::Result<AsyncRawResponse>> {
        // Check if it's a server error type
        if let Some(server_error) = result.as_server_error() {
            // Check probability
            if server_error.probability < 1.0 {
                let random: f32 = rand::rng().random();
                if random > server_error.probability {
                    return None; // Don't inject fault this time
                }
            }

            // Apply delay before injecting the error
            if server_error.delay > Duration::ZERO {
                tokio::time::sleep(server_error.delay).await;
            }

            // Generate the appropriate error based on error type
            let (status_code, sub_status, message) = match server_error.error_type {
                FaultInjectionServerErrorType::Gone => {
                    (StatusCode::Gone, Some(0), "Gone - Injected fault")
                }
                FaultInjectionServerErrorType::RetryWith => {
                    (StatusCode::from(449), None, "Retry With - Injected fault")
                }
                FaultInjectionServerErrorType::InternalServerError => {
                    (StatusCode::InternalServerError, None, "Internal Server Error - Injected fault")
                }
                FaultInjectionServerErrorType::TooManyRequests => {
                    (StatusCode::TooManyRequests, None, "Too Many Requests - Injected fault")
                }
                FaultInjectionServerErrorType::ReadSessionNotAvailable => {
                    (StatusCode::NotFound, Some(1002), "Read Session Not Available - Injected fault")
                }
                FaultInjectionServerErrorType::Timeout => {
                    (StatusCode::RequestTimeout, None, "Request Timeout - Injected fault")
                }
                FaultInjectionServerErrorType::PartitionIsMigrating => {
                    (StatusCode::Gone, Some(1008), "Partition Is Migrating - Injected fault")
                }
                FaultInjectionServerErrorType::PartitionIsSplitting => {
                    (StatusCode::Gone, Some(1007), "Partition Is Splitting - Injected fault")
                }
                FaultInjectionServerErrorType::ResponseDelay => {
                    // For response delay, we just add a delay but don't return an error
                    // The delay was already applied above
                    return None;
                }
                FaultInjectionServerErrorType::ConnectionDelay => {
                    // For connection delay, we add a delay but don't return an error
                    // The delay was already applied above
                    return None;
                }
                FaultInjectionServerErrorType::ServiceUnavailable => {
                    (StatusCode::ServiceUnavailable, None, "Service Unavailable - Injected fault")
                }
                FaultInjectionServerErrorType::StaledAddressesServerGone => {
                    (StatusCode::Gone, Some(0), "Staled Addresses Server Gone - Injected fault")
                }
                FaultInjectionServerErrorType::NameCacheIsStale => {
                    (StatusCode::Gone, Some(1000), "Name Cache Is Stale - Injected fault")
                }
                FaultInjectionServerErrorType::PartitionIsGone => {
                    (StatusCode::Gone, Some(1002), "Partition Is Gone - Injected fault")
                }
                FaultInjectionServerErrorType::LeaseNotFound => {
                    (StatusCode::Gone, Some(1022), "Lease Not Found - Injected fault")
                }
            };

            let error = azure_core::Error::new(ErrorKind::HttpResponse {
                status: status_code,
                error_code: sub_status.map(|s| s.to_string()),
            })
            .context(message);

            return Some(Err(error));
        }

        None
    }
}

#[async_trait]
impl HttpClient for FaultClient {
    async fn execute_request(&self, request: &Request) -> azure_core::Result<AsyncRawResponse> {
        // Find applicable rule
        let mut rules = self.rules.lock().unwrap();
        let mut applicable_rule_index: Option<usize> = None;
        request.headers().remove("x-ms-fault-injection-operation-type");

        for (index, rule_state) in rules.iter_mut().enumerate() {
            if self.is_rule_applicable(rule_state) && self.matches_condition(request, &rule_state.rule) {
                applicable_rule_index = Some(index);
                break;
            }
        }

        // Apply fault if we found an applicable rule
        if let Some(index) = applicable_rule_index {
            let rule_state = &mut rules[index];

            // Increment hit count
            rule_state.hit_count.fetch_add(1, Ordering::SeqCst);

            // Get a reference to the result for applying the fault
            let result = &rule_state.rule.result;

            // Drop the lock before async operations
            drop(rules);

            // Re-acquire to get the result (we need to handle this differently)
            let rules = self.rules.lock().unwrap();
            if let Some(rule_state) = rules.get(index) {
                let result = &rule_state.rule.result;

                // Drop lock before async call
                let result_clone = result.clone_box();
                drop(rules);

                if let Some(fault_result) = self.apply_fault(result_clone.as_ref()).await {
                    return fault_result;
                }
            }
        } else {
            drop(rules);
        }

        // No fault injection or delay-only fault, proceed with actual request
        self.inner.execute_request(request).await
    }
}
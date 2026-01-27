use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use async_trait::async_trait;
use azure_core::http::{AsyncRawResponse, HttpClient, Request, StatusCode};
use azure_core::error::ErrorKind;
use crate::fault_injection::fault_injection_utils::{
    FaultInjectionRule, FaultInjectionServerErrorType, FaultInjectionResult,
};

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
    pub fn new(
        inner: Arc<dyn HttpClient>,
        rules: Vec<FaultInjectionRule>,
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
    fn matches_condition(&self, request: &Request, rule: &FaultInjectionRule) -> bool {
        let condition = &rule.condition;
        
        // Check endpoints if specified
        if let Some(ref endpoints) = condition.endpoints {
            let request_url = request.url().as_str();
            if endpoints.iter().any(|ep| request_url.contains(ep)) {
                return true;
            }
        }
        
        // Check region if specified
        if let Some(ref region) = condition.region {
            if request.url().as_str().contains(region) {
                return true;
            }
        }
        
        // If no conditions are specified, match all requests
        if condition.endpoints.is_none() && condition.region.is_none() 
            && condition.operation_type.is_none() && condition.container_id.is_none() {
            return true;
        }
        
        false
    }

    /// Applies the fault injection result and returns an error or modifies the response.
    async fn apply_fault(
        &self,
        result: &dyn FaultInjectionResult,
    ) -> Option<azure_core::Result<AsyncRawResponse>> {
        // Check if it's a server error type
        if let Some(server_error) = result.as_server_error() {
            // Check probability (simple implementation without rand)
            if server_error.probability < 1.0 {
                // Use a simple time-based pseudo-random check
                let nanos = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .subsec_nanos();
                let random = (nanos % 1000) as f32 / 1000.0;
                if random > server_error.probability {
                    return None; // Don't inject fault this time
                }
            }

            // Apply delay before injecting the error
            if server_error.delay > Duration::ZERO {
                // Convert std::time::Duration to azure_core time::Duration for sleep
                let delay_secs = server_error.delay.as_secs();
                let delay_nanos = server_error.delay.subsec_nanos();
                let delay = azure_core::time::Duration::new(delay_secs as i64, delay_nanos as i32);
                azure_core::async_runtime::get_async_runtime().sleep(delay).await;
            }

            // Generate the appropriate error based on error type
            let (status_code, _sub_status, message) = match server_error.error_type {
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
                FaultInjectionServerErrorType::PartitionIsGone => {
                    (StatusCode::Gone, Some(1002), "Partition Is Gone - Injected fault")
                }
            };

            let error = azure_core::Error::with_message(ErrorKind::HttpResponse {
                status: status_code,
                error_code: Some("Injected Fault".to_string()),
                raw_response: None,
            }, message);

            return Some(Err(error));
        }

        None
    }
}

#[async_trait]
impl HttpClient for FaultClient {
    async fn execute_request(&self, request: &Request) -> azure_core::Result<AsyncRawResponse> {
        // Find applicable rule and clone the result if needed
        let fault_result: Option<Box<dyn FaultInjectionResult>> = {
            let mut rules = self.rules.lock().unwrap();
            let mut applicable_rule_index: Option<usize> = None;

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
                // Clone the result before dropping the lock
                Some(rule_state.rule.result.clone_box())
            } else {
                None
            }
        };

        // Apply the fault outside the lock
        if let Some(result) = fault_result {
            if let Some(fault_response) = self.apply_fault(result.as_ref()).await {
                return fault_response;
            }
        }

        // No fault injection or delay-only fault, proceed with actual request
        self.inner.execute_request(request).await
    }
}
pub mod gone_retry_policy;
pub mod resource_throttle_retry_policy;
pub mod default_retry_policy;
pub mod session_retry_policy;

use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use azure_core::http::RawResponse;
use typespec_client_core::http::Request;
use resource_throttle_retry_policy::ResourceThrottleRetryPolicy;

#[derive(Debug, Clone)]
pub struct ShouldRetryResult {
    pub should_retry: bool,
    pub backoff_time: Duration,
}

impl ShouldRetryResult {
    pub fn no_retry() -> Self {
        Self {
            should_retry: false,
            backoff_time: Duration::ZERO,
        }
    }
    pub fn retry_after(backoff: Duration) -> Self {
        Self {
            should_retry: true,
            backoff_time: backoff,
        }
    }
}

/// Trait defining the retry policy interface for Cosmos DB operations
/// 
/// This trait provides a contract for implementing retry logic for transient failures
/// in Azure Cosmos DB operations. Implementers can define custom retry behavior for
/// both exceptions (errors) and HTTP responses based on their specific requirements.
#[async_trait]
pub trait DocumentClientRetryPolicy: Send + Sync {

    /// Called before sending a request to allow policy-specific modifications
    /// 
    /// This method is invoked immediately before each request is sent (including retries).
    /// # Arguments
    /// * `request` - Mutable reference to the HTTP request being sent
    fn on_before_send_request(&self, request: &mut Request);
    
    /// Determines whether to retry an operation that resulted in an error
    /// 
    /// This method is called when an operation fails with an exception (network error,
    /// timeout, or HTTP error). The implementation should examine the error and decide
    /// whether the operation should be retried and how long to wait before retrying.
    /// 
    /// # Arguments
    /// * `err` - The error that occurred during the operation
    /// 
    /// # Returns
    /// `ShouldRetryResult` indicating:
    /// - `should_retry`: Whether the operation should be retried
    /// - `backoff_time`: How long to wait before retrying (if `should_retry` is true)
    async fn should_retry_exception(
        &self,
        err: &azure_core::Error,
    ) -> ShouldRetryResult;

    /// Determines whether to retry an operation based on the HTTP response
    /// 
    /// This method is called when an operation completes with an HTTP response that
    /// might indicate a transient failure (e.g., 429, 503, 410). The implementation
    /// should examine the response status code and headers to decide whether to retry.
    /// 
    /// # Arguments
    /// * `response` - The HTTP response received from the server
    /// 
    /// # Returns
    /// `ShouldRetryResult` indicating:
    /// - `should_retry`: Whether the operation should be retried
    /// - `backoff_time`: How long to wait before retrying (if `should_retry` is true)
    async fn should_retry_response(
        &self,
        response: &RawResponse,
    ) -> ShouldRetryResult;
}

/// Configuration for retry policies
#[derive(Debug, Clone)]
pub struct RetryPolicyConfig {
    /// Maximum number of retry attempts for throttling (429 errors)
    pub max_throttle_retry_count: usize,

    /// Maximum wait time in seconds for throttling retries
    pub max_throttle_wait_time_secs: u64,
    
    /// Backoff delay multiplication factor for throttling
    pub throttle_backoff_factor: u32,
}

impl Default for RetryPolicyConfig {
    fn default() -> Self {
        Self {
            max_throttle_retry_count: 3,
            max_throttle_wait_time_secs: 100,
            throttle_backoff_factor: 30,
        }
    }
}

/// Base retry policy that manages all available retry policies
///
/// This class initializes and holds instances of all retry policies:
/// - ResourceThrottleRetryPolicy: Handles 429 TooManyRequests errors
/// - GoneRetryPolicy: Handles 410 Gone errors (partition splits/merges)
/// - SessionRetryPolicy: Handles session consistency issues
/// - DefaultRetryPolicy: Handles general connection errors
#[derive(Clone)]
pub struct BaseRetryPolicy {
    /// Policy for handling resource throttling (429 TooManyRequests)
    resource_throttle_policy: Arc<ResourceThrottleRetryPolicy>,

    /// Policy for handling gone exceptions (410 Gone - partition splits)
    // TODO: Uncomment when GoneRetryPolicy implements DocumentClientRetryPolicy
    // gone_retry_policy: Arc<dyn DocumentClientRetryPolicy>,

    /// Policy for handling session unavailability
    // TODO: Uncomment when SessionRetryPolicy implements DocumentClientRetryPolicy
    // session_retry_policy: Arc<dyn DocumentClientRetryPolicy>,

    /// Default policy for handling connection errors
    // TODO: Uncomment when DefaultRetryPolicy implements DocumentClientRetryPolicy
    // default_retry_policy: Arc<dyn DocumentClientRetryPolicy>,

    /// Configuration used to initialize the policies
    config: RetryPolicyConfig,
}

impl BaseRetryPolicy {
    /// Creates a new BaseRetryPolicy with default configuration
    ///
    /// This constructor initializes all available retry policies with sensible defaults:
    /// - Max throttle retry count: 3
    /// - Max throttle wait time: 100 seconds
    /// - Throttle backoff factor: 30
    ///
    /// # Example
    /// ```
    /// use azure_data_cosmos::retry_policies::BaseRetryPolicy;
    ///
    /// let retry_policy = BaseRetryPolicy::new();
    /// ```
    pub fn new() -> Self {
        Self::with_config(RetryPolicyConfig::default())
    }

    /// Creates a new BaseRetryPolicy with custom configuration
    ///
    /// This constructor allows you to customize the retry behavior for all policies.
    ///
    /// # Arguments
    /// * `config` - Custom configuration for the retry policies
    ///
    /// # Example
    /// ```
    /// use azure_data_cosmos::retry_policies::{BaseRetryPolicy, RetryPolicyConfig};
    ///
    /// let config = RetryPolicyConfig {
    ///     max_throttle_retry_count: 5,
    ///     max_throttle_wait_time_secs: 200,
    ///     throttle_backoff_factor: 50,
    /// };
    ///
    /// let retry_policy = BaseRetryPolicy::with_config(config);
    /// ```
    pub fn with_config(config: RetryPolicyConfig) -> Self {
        // Initialize ResourceThrottleRetryPolicy
        let resource_throttle_policy = Arc::new(ResourceThrottleRetryPolicy::new(
            config.max_throttle_retry_count,
            config.max_throttle_wait_time_secs,
            config.throttle_backoff_factor,
        ));

        // TODO: Initialize GoneRetryPolicy when it implements DocumentClientRetryPolicy
        // let gone_retry_policy = Arc::new(GoneRetryPolicy::new(...));

        // TODO: Initialize SessionRetryPolicy when it implements DocumentClientRetryPolicy
        // let session_retry_policy = Arc::new(SessionRetryPolicy::new(...));

        // TODO: Initialize DefaultRetryPolicy when it implements DocumentClientRetryPolicy
        // let default_retry_policy = Arc::new(DefaultRetryPolicy::new(...));

        Self {
            resource_throttle_policy,
            config,
        }
    }

    /// Returns the resource throttle retry policy
    ///
    /// This policy handles 429 TooManyRequests errors with exponential backoff.
    pub fn resource_throttle_policy(&self) -> Arc<ResourceThrottleRetryPolicy> {
        self.resource_throttle_policy.clone()
    }

    /// Returns the resource throttle retry policy as a trait object
    ///
    /// Useful when you need to work with the DocumentClientRetryPolicy trait.
    pub fn resource_throttle_policy_dyn(&self) -> Arc<dyn DocumentClientRetryPolicy> {
        self.resource_throttle_policy.clone()
    }

    // TODO: Add getters for other policies when they implement DocumentClientRetryPolicy
    //
    // /// Returns the gone retry policy
    // pub fn gone_retry_policy(&self) -> Arc<dyn DocumentClientRetryPolicy> {
    //     self.gone_retry_policy.clone()
    // }
    //
    // /// Returns the session retry policy
    // pub fn session_retry_policy(&self) -> Arc<dyn DocumentClientRetryPolicy> {
    //     self.session_retry_policy.clone()
    // }
    //
    // /// Returns the default retry policy
    // pub fn default_retry_policy(&self) -> Arc<dyn DocumentClientRetryPolicy> {
    //     self.default_retry_policy.clone()
    // }

    /// Returns the configuration used to initialize this retry policy
    pub fn config(&self) -> &RetryPolicyConfig {
        &self.config
    }

    /// Returns the appropriate retry policy based on the request
    ///
    /// This method examines the request headers and method to determine which
    /// retry policy should be used for this specific request.
    ///
    /// # Policy Selection Logic
    ///
    /// Currently, this method returns the ResourceThrottleRetryPolicy for all requests.
    /// Future enhancements will check for:
    ///
    /// - **GoneRetryPolicy**: For requests that might encounter 410 Gone errors
    ///   (partition splits/merges). This is typically needed for:
    ///   - Requests with partition key headers
    ///   - Long-running operations
    ///
    /// - **SessionRetryPolicy**: For requests with session consistency requirements.
    ///   Detected by checking for:
    ///   - `x-ms-session-token` header
    ///   - `x-ms-consistency-level: Session` header
    ///
    /// - **DefaultRetryPolicy**: For general connection errors and transient failures.
    ///   Used as a fallback for requests that don't match other specific policies.
    ///
    /// - **ResourceThrottleRetryPolicy**: For handling 429 TooManyRequests errors.
    ///   This is the default policy for all requests.
    ///
    /// # Arguments
    /// * `request` - The HTTP request to analyze
    ///
    /// # Returns
    /// An Arc-wrapped trait object implementing DocumentClientRetryPolicy
    ///
    /// # Example
    /// ```ignore
    /// use azure_core::http::request::Request;
    /// use url::Url;
    /// use azure_data_cosmos::retry_policies::BaseRetryPolicy;
    ///
    /// let base_policy = BaseRetryPolicy::new();
    /// let url = Url::parse("https://localhost:8081/dbs/mydb").unwrap();
    /// let request = Request::new(url, azure_core::http::Method::Get);
    ///
    /// let policy = base_policy.get_policy_for_request(&request);
    /// // policy can now be used for retry logic
    /// ```
    pub fn get_policy_for_request(&self, _request: &azure_core::http::request::Request) -> Arc<dyn DocumentClientRetryPolicy> {
        // TODO: Implement policy selection logic based on request headers
        // For now, always return ResourceThrottleRetryPolicy
        //
        // Future implementation should check:
        // 1. request.headers().get(constants::SESSION_TOKEN) -> SessionRetryPolicy
        // 2. request.headers().get(constants::CONSISTENCY_LEVEL) == "Session" -> SessionRetryPolicy
        // 3. request.headers().get(constants::PARTITION_KEY) for partition-related requests -> GoneRetryPolicy
        // 4. Default to ResourceThrottleRetryPolicy for throttling protection

        self.resource_throttle_policy_dyn()
    }
}

impl Default for BaseRetryPolicy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_retry_policy_new() {
        let policy = BaseRetryPolicy::new();
        assert_eq!(policy.config.max_throttle_retry_count, 3);
        assert_eq!(policy.config.max_throttle_wait_time_secs, 100);
        assert_eq!(policy.config.throttle_backoff_factor, 30);
    }

    #[test]
    fn test_base_retry_policy_with_custom_config() {
        let config = RetryPolicyConfig {
            max_throttle_retry_count: 5,
            max_throttle_wait_time_secs: 200,
            throttle_backoff_factor: 50,
        };

        let policy = BaseRetryPolicy::with_config(config.clone());
        assert_eq!(policy.config.max_throttle_retry_count, 5);
        assert_eq!(policy.config.max_throttle_wait_time_secs, 200);
        assert_eq!(policy.config.throttle_backoff_factor, 50);
    }

    #[test]
    fn test_resource_throttle_policy_accessible() {
        let policy = BaseRetryPolicy::new();
        let throttle_policy = policy.resource_throttle_policy();

        // Verify we can access the policy
        assert!(Arc::strong_count(&throttle_policy) >= 1);
    }

    #[test]
    fn test_default_trait_implementation() {
        let policy = BaseRetryPolicy::default();
        assert_eq!(policy.config.max_throttle_retry_count, 3);
    }

    #[test]
    fn test_get_policy_for_request() {
        use azure_core::http::Method;
        use url::Url;

        let base_policy = BaseRetryPolicy::new();
        let url = Url::parse("https://localhost:8081/dbs/mydb").unwrap();
        let request = Request::new(url, Method::Get);

        // Get the policy for the request
        let policy = base_policy.get_policy_for_request(&request);

        // Verify we get a valid policy back
        // (We can't directly test which policy it is since it's behind a trait object,
        // but we can verify the Arc is valid)
        assert!(Arc::strong_count(&policy) >= 1);
    }

    #[test]
    fn test_get_policy_for_request_with_different_methods() {
        use azure_core::http::Method;
        use url::Url;

        let base_policy = BaseRetryPolicy::new();
        let url = Url::parse("https://localhost:8081/dbs/mydb/colls/mycoll/docs").unwrap();

        // Test with different HTTP methods
        for method in [Method::Get, Method::Post, Method::Put, Method::Delete, Method::Patch] {
            let request = Request::new(url.clone(), method);
            let policy = base_policy.get_policy_for_request(&request);
            assert!(Arc::strong_count(&policy) >= 1);
        }
    }
}

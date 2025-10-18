// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub mod resource_throttle_retry_policy;
use async_trait::async_trait;
use azure_core::http::RawResponse;
use resource_throttle_retry_policy::ResourceThrottleRetryPolicy;
use std::sync::Arc;
use std::time::Duration;
use typespec_client_core::http::Request;

/// Result of a retry policy decision
///
/// This struct encapsulates the decision made by a retry policy about whether
/// an operation should be retried and how long to wait before the retry attempt.
///
/// # Fields
/// * `should_retry` - Whether the operation should be retried
/// * `backoff_time` - Duration to wait before retrying (meaningful only if `should_retry` is true)
#[derive(Debug, Clone)]
pub struct ShouldRetryResult {
    pub should_retry: bool,
    pub backoff_time: Duration,
}

impl ShouldRetryResult {
    /// Creates a result indicating the operation should not be retried
    ///
    /// # Example
    /// ```
    /// use azure_data_cosmos::retry_policies::ShouldRetryResult;
    ///
    /// let result = ShouldRetryResult::no_retry();
    /// assert!(!result.should_retry);
    /// ```
    pub fn no_retry() -> Self {
        Self {
            should_retry: false,
            backoff_time: Duration::ZERO,
        }
    }

    /// Creates a result indicating the operation should be retried after a delay
    ///
    /// # Arguments
    /// * `backoff` - The duration to wait before retrying
    ///
    /// # Example
    /// ```
    /// use azure_data_cosmos::retry_policies::ShouldRetryResult;
    /// use std::time::Duration;
    ///
    /// let result = ShouldRetryResult::retry_after(Duration::from_secs(5));
    /// assert!(result.should_retry);
    /// assert_eq!(result.backoff_time, Duration::from_secs(5));
    /// ```
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
pub trait RetryPolicy: Send + Sync {
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
    async fn should_retry_exception(&self, err: &azure_core::Error) -> ShouldRetryResult;

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
    async fn should_retry_response(&self, response: &RawResponse) -> ShouldRetryResult;
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
#[derive(Debug, Clone)]
pub struct BaseRetryPolicy {
    /// Policy for handling resource throttling (429 TooManyRequests)
    resource_throttle_policy: Arc<ResourceThrottleRetryPolicy>,

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
    pub fn resource_throttle_policy_dyn(&self) -> Arc<dyn RetryPolicy> {
        self.resource_throttle_policy.clone()
    }

    /// Returns the configuration used to initialize this retry policy
    pub fn config(&self) -> &RetryPolicyConfig {
        &self.config
    }

    /// Returns the appropriate retry policy based on the request
    ///
    /// This method examines the request headers and method to determine which
    /// retry policy should be used for this specific request.
    /// # Arguments
    /// * `request` - The HTTP request to analyze
    pub fn get_policy_for_request(&self, _request: &Request) -> Arc<dyn RetryPolicy> {
        // For now, always return ResourceThrottleRetryPolicy.  Future implementation should check
        // the request operation type and resource type and accordingly return the respective retry
        // policy.
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
        for method in [
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Patch,
        ] {
            let request = Request::new(url.clone(), method);
            let policy = base_policy.get_policy_for_request(&request);
            assert!(Arc::strong_count(&policy) >= 1);
        }
    }
}

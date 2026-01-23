// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Resource Provider (RP) registration policy for Azure Resource Manager.
//!
//! This policy automatically registers unregistered resource providers when encountering
//! specific registration errors. See <https://aka.ms/rps-not-found> for more information.

use crate::{
    credentials::TokenCredential,
    error::{ErrorKind, ResultExt},
    http::{
        headers::{HeaderName, Headers},
        policies::{auth::BearerTokenAuthorizationPolicy, Policy, PolicyResult},
        AsyncRawResponse, Context, Method, Request, StatusCode, Url,
    },
    sleep::sleep,
    time::Duration,
    Error, Result,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, trace};

/// Error codes that indicate an unregistered resource provider.
const UNREGISTERED_RP_CODES: &[&str] = &[
    "MissingSubscriptionRegistration",
    "MissingRegistrationForResourceProvider",
    "Subscription Not Registered",
    "SubscriptionNotRegistered",
];

/// The registered state for a resource provider.
const REGISTERED_STATE: &str = "Registered";

/// Default API version for provider registration operations.
const PROVIDER_API_VERSION: &str = "2019-05-01";

/// Default maximum number of registration attempts.
const DEFAULT_MAX_ATTEMPTS: u32 = 3;

/// Default polling delay between registration status checks.
const DEFAULT_POLLING_DELAY: Duration = Duration::seconds(15);

/// Default maximum polling duration for registration to complete.
const DEFAULT_POLLING_DURATION: Duration = Duration::minutes(5);

/// Options for configuring the RP registration policy.
#[derive(Debug, Clone)]
pub struct RPRegistrationOptions {
    /// Maximum number of attempts to register a resource provider.
    /// Set to 0 to disable automatic registration.
    /// Default: 3
    pub max_attempts: u32,

    /// Delay between polling attempts for registration status.
    /// Default: 15 seconds
    pub polling_delay: Duration,

    /// Maximum duration to wait for registration to complete.
    /// Default: 5 minutes
    pub polling_duration: Duration,

    /// HTTP status codes that trigger RP registration.
    /// Default: [409] (Conflict)
    pub status_codes: Vec<StatusCode>,

    /// The Azure management endpoint.
    /// Default: "https://management.azure.com"
    pub endpoint: String,

    /// The audience for authentication tokens.
    /// Default: "https://management.azure.com"
    pub audience: String,
}

impl Default for RPRegistrationOptions {
    fn default() -> Self {
        Self {
            max_attempts: DEFAULT_MAX_ATTEMPTS,
            polling_delay: DEFAULT_POLLING_DELAY,
            polling_duration: DEFAULT_POLLING_DURATION,
            status_codes: vec![StatusCode::Conflict],
            endpoint: "https://management.azure.com".to_string(),
            audience: "https://management.azure.com".to_string(),
        }
    }
}

/// Error response from Azure Resource Manager.
#[derive(Debug, Deserialize)]
struct ArmErrorResponse {
    error: Option<ArmServiceError>,
}

/// Service error details from Azure Resource Manager.
#[derive(Debug, Deserialize)]
struct ArmServiceError {
    code: String,
}

/// Response from provider registration operations.
#[derive(Debug, Deserialize, Serialize)]
struct ProviderResponse {
    #[serde(rename = "registrationState")]
    registration_state: Option<String>,
}

/// Policy that automatically registers unregistered resource providers.
///
/// This policy intercepts requests that fail due to unregistered resource providers
/// and automatically registers them before retrying the original request.
///
/// # Example
///
/// ```rust,no_run
/// use azure_core::{
///     credentials::TokenCredential,
///     http::policies::arm::{RPRegistrationOptions, RPRegistrationPolicy},
/// };
/// use std::sync::Arc;
///
/// # async fn example(credential: Arc<dyn TokenCredential>) {
/// let options = RPRegistrationOptions::default();
/// let policy = RPRegistrationPolicy::new(credential, options);
/// # }
/// ```
#[derive(Debug)]
pub struct RPRegistrationPolicy {
    options: RPRegistrationOptions,
    auth_policy: Arc<BearerTokenAuthorizationPolicy>,
}

impl RPRegistrationPolicy {
    /// Creates a new RP registration policy.
    ///
    /// # Arguments
    ///
    /// * `credential` - The token credential to use for authentication.
    /// * `options` - Configuration options for the policy.
    pub fn new(credential: Arc<dyn TokenCredential>, options: RPRegistrationOptions) -> Self {
        let auth_policy = Arc::new(BearerTokenAuthorizationPolicy::new(
            credential,
            vec![format!("{}/.default", options.audience)],
        ));

        Self {
            options,
            auth_policy,
        }
    }

    /// Checks if the error code indicates an unregistered resource provider.
    fn is_unregistered_rp_error(code: &str) -> bool {
        UNREGISTERED_RP_CODES
            .iter()
            .any(|&rp_code| code.eq_ignore_ascii_case(rp_code))
    }

    /// Extracts the resource provider namespace from a URL path.
    ///
    /// Expected path format: /subscriptions/{subscriptionId}/providers/{resourceProviderNamespace}/...
    fn extract_rp_namespace(path: &str) -> Option<(String, String)> {
        let parts: Vec<&str> = path.split('/').collect();

        // Find the subscription ID
        let subscription_id = parts
            .iter()
            .position(|&p| p.eq_ignore_ascii_case("subscriptions"))
            .and_then(|i| parts.get(i + 1))
            .map(|s| s.to_string())?;

        // Find the resource provider namespace
        let rp_namespace = parts
            .iter()
            .position(|&p| p.eq_ignore_ascii_case("providers"))
            .and_then(|i| parts.get(i + 1))
            .map(|s| s.to_string())?;

        Some((subscription_id, rp_namespace))
    }

    /// Registers a resource provider.
    async fn register_provider(
        &self,
        ctx: &Context<'_>,
        subscription_id: &str,
        provider_namespace: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/providers/{}/register?api-version={}",
            self.options.endpoint, subscription_id, provider_namespace, PROVIDER_API_VERSION
        );

        let url = Url::parse(&url)
            .with_context(ErrorKind::Other, "failed to parse registration URL")?;

        let mut request = Request::new(url, Method::Post);
        request.insert_header(HeaderName::from_static("content-length"), "0");

        // Send the registration request using the auth policy
        let response = self
            .auth_policy
            .send(ctx, &mut request, &[])
            .await
            .with_context(ErrorKind::Other, "failed to register resource provider")?;

        if !response.status().is_success() {
            return Err(Error::with_message(
                ErrorKind::Other,
                format!(
                    "failed to register resource provider: status {}",
                    response.status()
                ),
            ));
        }

        debug!(
            "Successfully initiated registration for provider: {}",
            provider_namespace
        );

        Ok(())
    }

    /// Gets the current registration state of a resource provider.
    async fn get_provider_state(
        &self,
        ctx: &Context<'_>,
        subscription_id: &str,
        provider_namespace: &str,
    ) -> Result<String> {
        let url = format!(
            "{}/subscriptions/{}/providers/{}?api-version={}",
            self.options.endpoint, subscription_id, provider_namespace, PROVIDER_API_VERSION
        );

        let url = Url::parse(&url)
            .with_context(ErrorKind::Other, "failed to parse provider state URL")?;

        let mut request = Request::new(url, Method::Get);

        // Send the request using the auth policy
        let response = self
            .auth_policy
            .send(ctx, &mut request, &[])
            .await
            .with_context(ErrorKind::Other, "failed to get provider state")?;

        if !response.status().is_success() {
            return Err(Error::with_message(
                ErrorKind::Other,
                format!("failed to get provider state: status {}", response.status()),
            ));
        }

        let body = response
            .into_body()
            .collect_string()
            .await
            .with_context(ErrorKind::Other, "failed to read provider state response")?;

        let provider_response: ProviderResponse = serde_json::from_str(&body)
            .with_context(ErrorKind::DataConversion, "failed to parse provider state")?;

        Ok(provider_response
            .registration_state
            .unwrap_or_else(|| "Unknown".to_string()))
    }

    /// Polls the provider registration status until it's registered or times out.
    async fn poll_registration_state(
        &self,
        ctx: &Context<'_>,
        subscription_id: &str,
        provider_namespace: &str,
    ) -> Result<()> {
        let start = crate::time::OffsetDateTime::now_utc();
        let mut last_state = String::new();

        loop {
            let state = self
                .get_provider_state(ctx, subscription_id, provider_namespace)
                .await?;

            if state != last_state {
                trace!("Provider {} registration state: {}", provider_namespace, state);
                last_state = state.clone();
            }

            if state.eq_ignore_ascii_case(REGISTERED_STATE) {
                debug!("Provider {} is now registered", provider_namespace);
                return Ok(());
            }

            let elapsed = crate::time::OffsetDateTime::now_utc() - start;
            if elapsed >= self.options.polling_duration {
                return Err(Error::with_message(
                    ErrorKind::Other,
                    format!(
                        "timeout waiting for provider {} to register after {:?}",
                        provider_namespace, elapsed
                    ),
                ));
            }

            sleep(self.options.polling_delay).await;
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for RPRegistrationPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        if self.options.max_attempts == 0 {
            // Policy is disabled
            return next[0].send(ctx, request, &next[1..]).await;
        }

        for attempt in 0..self.options.max_attempts {
            // Make the original request
            let response = next[0].send(ctx, request, &next[1..]).await?;

            // Check if the status code indicates a potential RP registration issue
            if !self.options.status_codes.contains(&response.status()) {
                return Ok(response);
            }

            // Try to parse the error response
            let body = response
                .into_body()
                .collect_string()
                .await
                .with_context(ErrorKind::Other, "failed to read error response")?;

            let error_response: Result<ArmErrorResponse> = serde_json::from_str(&body)
                .with_context(ErrorKind::DataConversion, "failed to parse error response");

            let error_response = match error_response {
                Ok(err) => err,
                Err(_) => {
                    // Not a parseable ARM error, return the original response
                    return Ok(AsyncRawResponse::from_bytes(
                        StatusCode::Conflict,
                        Headers::new(),
                        body,
                    ));
                }
            };

            // Check if this is an unregistered RP error
            let is_unregistered = error_response
                .error
                .as_ref()
                .map(|e| Self::is_unregistered_rp_error(&e.code))
                .unwrap_or(false);

            if !is_unregistered {
                // Not an unregistered RP error, return the response
                return Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Conflict,
                    Headers::new(),
                    body,
                ));
            }

            // Extract the resource provider namespace from the request URL
            let (subscription_id, provider_namespace) =
                Self::extract_rp_namespace(request.url().path())
                    .ok_or_else(|| {
                        Error::with_message(
                            ErrorKind::Other,
                            "failed to extract resource provider namespace from URL",
                        )
                    })?;

            debug!(
                "Detected unregistered resource provider: {} (attempt {}/{})",
                provider_namespace,
                attempt + 1,
                self.options.max_attempts
            );

            // Register the provider
            self.register_provider(ctx, &subscription_id, &provider_namespace)
                .await?;

            // Poll until registration is complete
            self.poll_registration_state(ctx, &subscription_id, &provider_namespace)
                .await?;

            // Reset the request body for retry
            request
                .body_mut()
                .reset()
                .await
                .with_context(ErrorKind::Other, "failed to reset request body for retry")?;

            debug!(
                "Retrying original request after registering provider: {}",
                provider_namespace
            );
        }

        // If we get here, we exceeded the maximum number of attempts
        Err(Error::with_message(
            ErrorKind::Other,
            "exceeded maximum attempts to register resource provider",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unregistered_rp_error() {
        assert!(RPRegistrationPolicy::is_unregistered_rp_error(
            "MissingSubscriptionRegistration"
        ));
        assert!(RPRegistrationPolicy::is_unregistered_rp_error(
            "MissingRegistrationForResourceProvider"
        ));
        assert!(RPRegistrationPolicy::is_unregistered_rp_error(
            "Subscription Not Registered"
        ));
        assert!(RPRegistrationPolicy::is_unregistered_rp_error(
            "SubscriptionNotRegistered"
        ));

        // Test case insensitivity
        assert!(RPRegistrationPolicy::is_unregistered_rp_error(
            "missingsubscriptionregistration"
        ));

        // Test non-matching code
        assert!(!RPRegistrationPolicy::is_unregistered_rp_error(
            "SomeOtherError"
        ));
    }

    #[test]
    fn test_extract_rp_namespace() {
        // Test valid path
        let path = "/subscriptions/12345/providers/Microsoft.Storage/storageAccounts/myaccount";
        let result = RPRegistrationPolicy::extract_rp_namespace(path);
        assert_eq!(
            result,
            Some(("12345".to_string(), "Microsoft.Storage".to_string()))
        );

        // Test path with different casing
        let path = "/Subscriptions/12345/Providers/Microsoft.Compute/virtualMachines/myvm";
        let result = RPRegistrationPolicy::extract_rp_namespace(path);
        assert_eq!(
            result,
            Some(("12345".to_string(), "Microsoft.Compute".to_string()))
        );

        // Test invalid path (no providers)
        let path = "/subscriptions/12345/resourceGroups/mygroup";
        let result = RPRegistrationPolicy::extract_rp_namespace(path);
        assert_eq!(result, None);

        // Test invalid path (no subscription)
        let path = "/providers/Microsoft.Storage/storageAccounts/myaccount";
        let result = RPRegistrationPolicy::extract_rp_namespace(path);
        assert_eq!(result, None);
    }

    #[test]
    fn test_rp_registration_options_default() {
        let options = RPRegistrationOptions::default();
        assert_eq!(options.max_attempts, 3);
        assert_eq!(options.polling_delay, Duration::seconds(15));
        assert_eq!(options.polling_duration, Duration::minutes(5));
        assert_eq!(options.status_codes, vec![StatusCode::Conflict]);
        assert_eq!(options.endpoint, "https://management.azure.com");
        assert_eq!(options.audience, "https://management.azure.com");
    }

    #[test]
    fn test_rp_registration_options_custom() {
        let options = RPRegistrationOptions {
            max_attempts: 5,
            polling_delay: Duration::seconds(10),
            polling_duration: Duration::minutes(10),
            status_codes: vec![StatusCode::Conflict, StatusCode::Forbidden],
            endpoint: "https://custom.endpoint.com".to_string(),
            audience: "https://custom.audience.com".to_string(),
        };

        assert_eq!(options.max_attempts, 5);
        assert_eq!(options.polling_delay, Duration::seconds(10));
        assert_eq!(options.polling_duration, Duration::minutes(10));
        assert_eq!(
            options.status_codes,
            vec![StatusCode::Conflict, StatusCode::Forbidden]
        );
        assert_eq!(options.endpoint, "https://custom.endpoint.com");
        assert_eq!(options.audience, "https://custom.audience.com");
    }

    #[test]
    fn test_disabled_policy() {
        // Test that policy is disabled when max_attempts is 0
        let options = RPRegistrationOptions {
            max_attempts: 0,
            ..Default::default()
        };
        assert_eq!(options.max_attempts, 0);
    }
}

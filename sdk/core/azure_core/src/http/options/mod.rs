// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod instrumentation;
mod user_agent;

pub use instrumentation::*;
use crate::cloud::CloudConfiguration;
use std::sync::Arc;
use typespec_client_core::http::policies::Policy;
pub use typespec_client_core::http::{
    ClientMethodOptions, ExponentialRetryOptions, FixedRetryOptions, RetryOptions, TransportOptions,
};
pub use user_agent::*;

/// Client options allow customization of general client policies, retry options, and more.
///
/// # Examples
///
/// ## Basic usage with default (Public Cloud) configuration:
/// ```
/// use azure_core::http::ClientOptions;
/// 
/// let options = ClientOptions::default();
/// ```
///
/// ## Using a specific cloud configuration:
/// ```
/// use azure_core::http::ClientOptions;
/// use azure_core::cloud::configurations;
/// 
/// // Configure for Azure China Cloud
/// let options = ClientOptions::default()
///     .with_cloud(configurations::azure_china_cloud().clone())
///     .with_audience("https://storage.core.chinacloudapi.cn");
///     
/// // Get the OAuth scope for authentication
/// let scope = options.get_auth_scope(Some("storage"));
/// assert_eq!(scope, Some("https://storage.core.chinacloudapi.cn/.default".to_string()));
/// ```
#[derive(Clone, Debug, Default)]
pub struct ClientOptions {
    /// Policies called per call.
    pub per_call_policies: Vec<Arc<dyn Policy>>,

    /// Policies called per try.
    pub per_try_policies: Vec<Arc<dyn Policy>>,

    /// Retry options.
    pub retry: Option<RetryOptions>,

    /// Transport options.
    pub transport: Option<TransportOptions>,

    /// User-Agent telemetry options.
    pub user_agent: Option<UserAgentOptions>,

    /// Options for request instrumentation, such as distributed tracing.
    ///
    /// If not specified, defaults to no instrumentation.
    ///
    pub instrumentation: Option<InstrumentationOptions>,

    /// Cloud configuration for determining endpoints and audiences.
    ///
    /// If not specified, defaults to Azure Public Cloud.
    pub cloud: Option<CloudConfiguration>,

    /// Service audience for token requests.
    ///
    /// This is typically the base URI of the service being accessed.
    /// If not specified, the audience will be derived from the cloud configuration
    /// for known services, or default to the resource manager audience.
    pub audience: Option<String>,
}

pub(crate) struct CoreClientOptions {
    pub(crate) user_agent: UserAgentOptions,
    pub(crate) instrumentation: InstrumentationOptions,
    pub(crate) cloud: Option<CloudConfiguration>,
    pub(crate) audience: Option<String>,
}

impl ClientOptions {
    /// Set the cloud configuration.
    ///
    /// This determines the endpoints and audiences used for authentication
    /// and service requests.
    pub fn with_cloud(mut self, cloud: CloudConfiguration) -> Self {
        self.cloud = Some(cloud);
        self
    }

    /// Set the service audience for token requests.
    ///
    /// The audience should be the base URI of the service being accessed.
    /// For example, for Azure Storage, use "https://storage.azure.com".
    pub fn with_audience(mut self, audience: impl Into<String>) -> Self {
        self.audience = Some(audience.into());
        self
    }

    /// Get the scope for authentication based on audience and cloud configuration.
    ///
    /// This is a convenience method that derives the OAuth scope from the audience.
    /// If no audience is explicitly set, it will try to derive one from the cloud
    /// configuration for known services.
    pub fn get_auth_scope(&self, service_name: Option<&str>) -> Option<String> {
        let cloud = self.cloud.as_ref().unwrap_or_else(|| crate::cloud::configurations::azure_public_cloud());
        
        if let Some(audience) = &self.audience {
            Some(CloudConfiguration::audience_to_scope(audience))
        } else if let Some(service) = service_name {
            cloud.service_audience(service)
                .map(CloudConfiguration::audience_to_scope)
        } else {
            Some(CloudConfiguration::audience_to_scope(cloud.resource_manager_audience()))
        }
    }

    /// Efficiently deconstructs into owned [`typespec_client_core::http::ClientOptions`] as well as unwrapped or default Azure-specific options.
    ///
    /// If instead we implemented [`Into`], we'd have to clone Azure-specific options instead of moving memory of [`Some`] values.
    pub(in crate::http) fn deconstruct(
        self,
    ) -> (CoreClientOptions, typespec_client_core::http::ClientOptions) {
        let options = typespec_client_core::http::ClientOptions {
            per_call_policies: self.per_call_policies,
            per_try_policies: self.per_try_policies,
            retry: self.retry,
            transport: self.transport,
        };

        (
            CoreClientOptions {
                user_agent: self.user_agent.unwrap_or_default(),
                instrumentation: self.instrumentation.unwrap_or_default(),
                cloud: self.cloud,
                audience: self.audience,
            },
            options,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud::configurations;

    #[test]
    fn test_get_auth_scope_with_explicit_audience() {
        let options = ClientOptions::default()
            .with_audience("https://storage.azure.com");
        
        let scope = options.get_auth_scope(None);
        assert_eq!(scope, Some("https://storage.azure.com/.default".to_string()));
    }

    #[test]
    fn test_get_auth_scope_with_service_name() {
        let options = ClientOptions::default()
            .with_cloud(configurations::azure_public_cloud().clone());
        
        let scope = options.get_auth_scope(Some("storage"));
        assert_eq!(scope, Some("https://storage.azure.com/.default".to_string()));
        
        let scope = options.get_auth_scope(Some("keyvault"));
        assert_eq!(scope, Some("https://vault.azure.net/.default".to_string()));
    }

    #[test]
    fn test_get_auth_scope_default_resource_manager() {
        let options = ClientOptions::default();
        
        let scope = options.get_auth_scope(None);
        assert_eq!(scope, Some("https://management.azure.com/.default".to_string()));
    }

    #[test]
    fn test_get_auth_scope_china_cloud() {
        let options = ClientOptions::default()
            .with_cloud(configurations::azure_china_cloud().clone());
        
        let scope = options.get_auth_scope(Some("keyvault"));
        assert_eq!(scope, Some("https://vault.azure.cn/.default".to_string()));
    }

    #[test]
    fn test_explicit_audience_overrides_service_name() {
        let options = ClientOptions::default()
            .with_cloud(configurations::azure_public_cloud().clone())
            .with_audience("https://custom.service.com");
        
        let scope = options.get_auth_scope(Some("storage"));
        assert_eq!(scope, Some("https://custom.service.com/.default".to_string()));
    }

    #[test]
    fn test_scope_derivation_for_different_services() {
        // Test that the scope derivation works correctly for different Azure services
        let options = ClientOptions::default()
            .with_cloud(configurations::azure_public_cloud().clone());

        // Test KeyVault service
        let keyvault_scope = options.get_auth_scope(Some("keyvault"));
        assert_eq!(keyvault_scope, Some("https://vault.azure.net/.default".to_string()));

        // Test Storage service
        let storage_scope = options.get_auth_scope(Some("storage"));
        assert_eq!(storage_scope, Some("https://storage.azure.com/.default".to_string()));

        // Test unknown service (falls back to None for unknown services)
        let unknown_scope = options.get_auth_scope(Some("unknown"));
        assert_eq!(unknown_scope, None);

        // Test no service specified (uses resource manager)
        let default_scope = options.get_auth_scope(None);
        assert_eq!(default_scope, Some("https://management.azure.com/.default".to_string()));
    }
}

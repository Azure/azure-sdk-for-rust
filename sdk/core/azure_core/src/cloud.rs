// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cloud configuration for Azure services.
//!
//! This module provides cloud configurations for different Azure environments,
//! allowing services to operate across Azure Public Cloud, Azure China Cloud,
//! Azure Germany Cloud, and Azure US Government Cloud.

use crate::http::Url;
use std::collections::HashMap;

/// Configuration for a specific Azure cloud environment.
///
/// This struct contains the endpoints and settings needed to connect to
/// a specific Azure cloud environment. It includes authority hosts for
/// authentication, resource manager endpoints, and service-specific
/// audience URIs for token requests.
#[derive(Debug, Clone)]
pub struct CloudConfiguration {
    /// The authority host URL for authentication requests.
    pub authority_host: Url,
    
    /// The resource manager endpoint for management operations.
    pub resource_manager_endpoint: Url,
    
    /// Default audience for Azure Resource Manager.
    pub resource_manager_audience: String,
    
    /// Map of service names to their audience URIs.
    pub service_audiences: HashMap<String, String>,
}

impl CloudConfiguration {
    /// Creates a new cloud configuration.
    pub fn new(
        authority_host: Url,
        resource_manager_endpoint: Url,
        resource_manager_audience: String,
    ) -> Self {
        Self {
            authority_host,
            resource_manager_endpoint,
            resource_manager_audience,
            service_audiences: HashMap::new(),
        }
    }

    /// Adds a service audience to the cloud configuration.
    pub fn with_service_audience(mut self, service: impl Into<String>, audience: impl Into<String>) -> Self {
        self.service_audiences.insert(service.into(), audience.into());
        self
    }

    /// Gets the audience for a specific service.
    pub fn service_audience(&self, service: &str) -> Option<&str> {
        self.service_audiences.get(service).map(|s| s.as_str())
    }

    /// Gets the audience for Azure Resource Manager.
    pub fn resource_manager_audience(&self) -> &str {
        &self.resource_manager_audience
    }

    /// Derives a scope from an audience URI.
    ///
    /// Azure OAuth 2.0 scopes are typically the audience URI with "/.default" appended.
    pub fn audience_to_scope(audience: &str) -> String {
        if audience.ends_with("/.default") {
            audience.to_string()
        } else {
            format!("{}/.default", audience.trim_end_matches('/'))
        }
    }
}

/// Well-known cloud configurations for Azure environments.
pub mod configurations {
    use super::*;
    use std::sync::OnceLock;

    /// Azure Public Cloud configuration.
    pub fn azure_public_cloud() -> &'static CloudConfiguration {
        static CONFIG: OnceLock<CloudConfiguration> = OnceLock::new();
        CONFIG.get_or_init(|| {
            CloudConfiguration::new(
                Url::parse("https://login.microsoftonline.com").unwrap(),
                Url::parse("https://management.azure.com").unwrap(),
                "https://management.azure.com".to_string(),
            )
            .with_service_audience("storage", "https://storage.azure.com")
            .with_service_audience("keyvault", "https://vault.azure.net")
        })
    }

    /// Azure China Cloud configuration.
    pub fn azure_china_cloud() -> &'static CloudConfiguration {
        static CONFIG: OnceLock<CloudConfiguration> = OnceLock::new();
        CONFIG.get_or_init(|| {
            CloudConfiguration::new(
                Url::parse("https://login.chinacloudapi.cn").unwrap(),
                Url::parse("https://management.chinacloudapi.cn").unwrap(),
                "https://management.chinacloudapi.cn".to_string(),
            )
            .with_service_audience("storage", "https://storage.azure.com")
            .with_service_audience("keyvault", "https://vault.azure.cn")
        })
    }

    /// Azure Germany Cloud configuration.
    pub fn azure_germany_cloud() -> &'static CloudConfiguration {
        static CONFIG: OnceLock<CloudConfiguration> = OnceLock::new();
        CONFIG.get_or_init(|| {
            CloudConfiguration::new(
                Url::parse("https://login.microsoftonline.de").unwrap(),
                Url::parse("https://management.microsoftazure.de").unwrap(),
                "https://management.microsoftazure.de".to_string(),
            )
            .with_service_audience("storage", "https://storage.azure.com")
            .with_service_audience("keyvault", "https://vault.microsoftazure.de")
        })
    }

    /// Azure US Government Cloud configuration.
    pub fn azure_us_government_cloud() -> &'static CloudConfiguration {
        static CONFIG: OnceLock<CloudConfiguration> = OnceLock::new();
        CONFIG.get_or_init(|| {
            CloudConfiguration::new(
                Url::parse("https://login.microsoftonline.us").unwrap(),
                Url::parse("https://management.usgovcloudapi.net").unwrap(),
                "https://management.usgovcloudapi.net".to_string(),
            )
            .with_service_audience("storage", "https://storage.azure.com")
            .with_service_audience("keyvault", "https://vault.usgovcloudapi.net")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audience_to_scope() {
        assert_eq!(
            CloudConfiguration::audience_to_scope("https://management.azure.com"),
            "https://management.azure.com/.default"
        );
        assert_eq!(
            CloudConfiguration::audience_to_scope("https://management.azure.com/"),
            "https://management.azure.com/.default"
        );
        assert_eq!(
            CloudConfiguration::audience_to_scope("https://management.azure.com/.default"),
            "https://management.azure.com/.default"
        );
    }

    #[test]
    fn test_cloud_configurations() {
        let public = configurations::azure_public_cloud();
        assert_eq!(public.authority_host.as_str(), "https://login.microsoftonline.com/");
        assert_eq!(public.resource_manager_endpoint.as_str(), "https://management.azure.com/");
        assert_eq!(public.service_audience("storage"), Some("https://storage.azure.com"));
        assert_eq!(public.service_audience("tables"), None);

        let china = configurations::azure_china_cloud();
        assert_eq!(china.authority_host.as_str(), "https://login.chinacloudapi.cn/");
        assert_eq!(china.resource_manager_endpoint.as_str(), "https://management.chinacloudapi.cn/");

        let germany = configurations::azure_germany_cloud();
        assert_eq!(germany.authority_host.as_str(), "https://login.microsoftonline.de/");
        assert_eq!(germany.resource_manager_endpoint.as_str(), "https://management.microsoftazure.de/");

        let us_gov = configurations::azure_us_government_cloud();
        assert_eq!(us_gov.authority_host.as_str(), "https://login.microsoftonline.us/");
        assert_eq!(us_gov.resource_manager_endpoint.as_str(), "https://management.usgovcloudapi.net/");
    }

    #[test]
    fn test_service_audience() {
        let mut config = CloudConfiguration::new(
            Url::parse("https://login.microsoftonline.com").unwrap(),
            Url::parse("https://management.azure.com").unwrap(),
            "https://management.azure.com".to_string(),
        );
        
        assert_eq!(config.service_audience("storage"), None);
        
        config = config.with_service_audience("storage", "https://storage.azure.com");
        assert_eq!(config.service_audience("storage"), Some("https://storage.azure.com"));
    }
}
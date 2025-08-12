// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::env::Env;
#[cfg(not(target_arch = "wasm32"))]
use crate::process::{new_executor, Executor};
use azure_core::{
    cloud::CloudConfiguration,
    error::{ErrorKind, Result, ResultExt},
    http::{new_http_client, HttpClient, Url},
};
use std::sync::Arc;

const AZURE_AUTHORITY_HOST_ENV_KEY: &str = "AZURE_AUTHORITY_HOST";
const AZURE_PUBLIC_CLOUD: &str = "https://login.microsoftonline.com";

/// Provides options to configure how the Identity library makes authentication
/// requests to Azure Active Directory.
#[derive(Debug, Clone)]
pub struct TokenCredentialOptions {
    pub(crate) env: Env,
    pub(crate) http_client: Arc<dyn HttpClient>,
    pub(crate) authority_host: String,
    pub(crate) cloud_config: Option<&'static CloudConfiguration>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) executor: Arc<dyn Executor>,
}

/// The default token credential options.
///
/// The authority host is taken from the `AZURE_AUTHORITY_HOST` environment variable if set and a valid URL.
/// If not, the default authority host is `https://login.microsoftonline.com` for the Azure public cloud.
/// The cloud configuration can be set explicitly using `set_cloud_config()` for non-public clouds.
impl Default for TokenCredentialOptions {
    fn default() -> Self {
        let env = Env::default();
        let authority_host = env
            .var(AZURE_AUTHORITY_HOST_ENV_KEY)
            .unwrap_or_else(|_| AZURE_PUBLIC_CLOUD.to_owned());
        Self {
            env: Env::default(),
            http_client: new_http_client(),
            authority_host,
            cloud_config: None,
            #[cfg(not(target_arch = "wasm32"))]
            executor: new_executor(),
        }
    }
}

impl TokenCredentialOptions {
    /// Create options for the Azure Public Cloud.
    pub fn new_for_public_cloud() -> Self {
        let mut options = Self::default();
        options.set_cloud_config(azure_core::cloud::configurations::azure_public_cloud());
        options
    }

    /// Create options for the Azure China Cloud.
    pub fn new_for_china_cloud() -> Self {
        let mut options = Self::default();
        options.set_cloud_config(azure_core::cloud::configurations::azure_china_cloud());
        options
    }

    /// Create options for the Azure Germany Cloud.
    pub fn new_for_germany_cloud() -> Self {
        let mut options = Self::default();
        options.set_cloud_config(azure_core::cloud::configurations::azure_germany_cloud());
        options
    }

    /// Create options for the Azure US Government Cloud.
    pub fn new_for_us_government_cloud() -> Self {
        let mut options = Self::default();
        options.set_cloud_config(azure_core::cloud::configurations::azure_us_government_cloud());
        options
    }

    /// Set the cloud configuration for authentication requests.
    ///
    /// This allows credentials to work with different Azure clouds
    /// (Public, China, Germany, US Government) by setting the appropriate
    /// authority host and other cloud-specific settings.
    pub fn set_cloud_config(&mut self, cloud_config: &'static CloudConfiguration) {
        self.cloud_config = Some(cloud_config);
        self.authority_host = cloud_config.authority_host.to_string();
    }

    /// Set the authority host for authentication requests.
    pub fn set_authority_host(&mut self, authority_host: String) {
        self.authority_host = authority_host;
    }

    /// The authority host to use for authentication requests.
    ///
    /// The default is `https://login.microsoftonline.com`.
    pub fn authority_host(&self) -> Result<Url> {
        // If cloud config is set, use it; otherwise use the explicit authority_host
        let host = if let Some(config) = self.cloud_config {
            config.authority_host.clone()
        } else {
            Url::parse(&self.authority_host).with_context(ErrorKind::DataConversion, || {
                format!("invalid authority host URL {}", &self.authority_host)
            })?
        };
        Ok(host)
    }

    /// Gets the cloud configuration if set.
    pub fn cloud_config(&self) -> Option<&'static CloudConfiguration> {
        self.cloud_config
    }

    /// The [`HttpClient`] to make requests.
    pub fn http_client(&self) -> Arc<dyn HttpClient> {
        self.http_client.clone()
    }

    /// The [`Executor`] to run commands.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn executor(&self) -> Arc<dyn Executor> {
        self.executor.clone()
    }

    pub(crate) fn env(&self) -> &Env {
        &self.env
    }
}

impl From<Arc<dyn HttpClient>> for TokenCredentialOptions {
    fn from(http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            http_client,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::cloud::configurations;

    #[test]
    fn test_default_options() {
        let options = TokenCredentialOptions::default();
        assert_eq!(options.authority_host, AZURE_PUBLIC_CLOUD);
        assert!(options.cloud_config.is_none());
    }

    #[test]
    fn test_set_cloud_config() {
        let mut options = TokenCredentialOptions::default();
        options.set_cloud_config(configurations::azure_china_cloud());

        assert_eq!(
            options.cloud_config.unwrap().authority_host.as_str(),
            "https://login.chinacloudapi.cn/"
        );
        assert_eq!(options.authority_host, "https://login.chinacloudapi.cn/");
    }

    #[test]
    fn test_authority_host_with_cloud_config() {
        let mut options = TokenCredentialOptions::default();
        options.set_cloud_config(configurations::azure_us_government_cloud());

        let authority_host = options.authority_host().unwrap();
        assert_eq!(authority_host.as_str(), "https://login.microsoftonline.us/");
    }

    #[test]
    fn test_authority_host_without_cloud_config() {
        let options = TokenCredentialOptions::default();
        let authority_host = options.authority_host().unwrap();
        assert_eq!(authority_host.as_str(), "https://login.microsoftonline.com/");
    }

    #[test]
    fn test_convenience_methods() {
        let public = TokenCredentialOptions::new_for_public_cloud();
        assert_eq!(
            public.cloud_config.unwrap().authority_host.as_str(),
            "https://login.microsoftonline.com/"
        );

        let china = TokenCredentialOptions::new_for_china_cloud();
        assert_eq!(
            china.cloud_config.unwrap().authority_host.as_str(),
            "https://login.chinacloudapi.cn/"
        );

        let germany = TokenCredentialOptions::new_for_germany_cloud();
        assert_eq!(
            germany.cloud_config.unwrap().authority_host.as_str(),
            "https://login.microsoftonline.de/"
        );

        let us_gov = TokenCredentialOptions::new_for_us_government_cloud();
        assert_eq!(
            us_gov.cloud_config.unwrap().authority_host.as_str(),
            "https://login.microsoftonline.us/"
        );
    }
}

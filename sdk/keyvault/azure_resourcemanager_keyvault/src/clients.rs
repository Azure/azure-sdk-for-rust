// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.

/// Clients used to communicate with the service.
#[allow(clippy::wildcard_imports)]
pub use crate::generated::clients::*;
mod key_vault_clients;

use azure_core::{
    cloud::{CloudConfiguration, CustomConfiguration},
    error::ErrorKind,
    Error, Result,
};
pub use key_vault_clients::KeyVaultClientOptions;

/// Audience for Azure Key Vault Resource Manager requests.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Audience;

/// Returns the default audience for the specified cloud configuration.
///
/// Use this when requesting tokens for Azure Resource Manager. Custom clouds must define an
/// audience for `Audience` in the provided [`CustomConfiguration`].
fn audience(cloud: &CloudConfiguration) -> Result<&str> {
    match cloud {
        CloudConfiguration::AzurePublic => Ok("https://management.core.windows.net/"),
        CloudConfiguration::AzureGovernment => Ok("https://management.core.usgovcloudapi.net/"),
        CloudConfiguration::AzureChina => Ok("https://management.core.chinacloudapi.cn/"),
        CloudConfiguration::Custom(CustomConfiguration { audiences, .. }) => {
            audiences.get::<Audience>().ok_or_else(|| {
                Error::with_message(
                    ErrorKind::Other,
                    "cloud CustomConfiguration doesn't have a value for audience",
                )
            })
        }
        _ => Err(Error::with_message(
            ErrorKind::Other,
            "cloud configuration is not supported",
        )),
    }
}

/// Returns the default Azure Resource Manager endpoint for the specified cloud configuration.
///
/// Custom clouds must supply their own endpoint when constructing clients.
fn endpoint(cloud: &CloudConfiguration) -> Result<&'static str> {
    match cloud {
        CloudConfiguration::AzurePublic => Ok("https://management.azure.com"),
        CloudConfiguration::AzureGovernment => Ok("https://management.usgovcloudapi.net"),
        CloudConfiguration::AzureChina => Ok("https://management.chinacloudapi.cn"),
        CloudConfiguration::Custom(_) => Err(Error::with_message(
            ErrorKind::Other,
            "cloud CustomConfiguration doesn't have a value for endpoint",
        )),
        _ => Err(Error::with_message(
            ErrorKind::Other,
            "cloud configuration is not supported",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::cloud::Audiences;

    #[test]
    fn test_audience_azure_public() {
        let cloud = CloudConfiguration::AzurePublic;
        let result = audience(&cloud);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "https://management.core.windows.net/.default"
        );
    }

    #[test]
    fn test_audience_azure_government() {
        let cloud = CloudConfiguration::AzureGovernment;
        let result = audience(&cloud);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "https://management.core.usgovcloudapi.net/.default"
        );
    }

    #[test]
    fn test_audience_azure_china() {
        let cloud = CloudConfiguration::AzureChina;
        let result = audience(&cloud);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "https://management.core.chinacloudapi.cn/.default"
        );
    }

    #[test]
    fn test_audience_custom_with_audience() {
        let mut custom = CustomConfiguration::default();
        custom.audiences =
            Audiences::new().with::<Audience>("https://custom.audience.local/".to_string());
        let cloud = CloudConfiguration::Custom(custom);
        let result = audience(&cloud);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://custom.audience.local/.default");
    }

    #[test]
    fn test_audience_custom_without_audience() {
        let custom = CustomConfiguration::default();
        let cloud = CloudConfiguration::Custom(custom);
        let result = audience(&cloud);
        assert!(result.is_err());
    }

    #[test]
    fn test_endpoint_azure_public() {
        let cloud = CloudConfiguration::AzurePublic;
        let result = endpoint(&cloud);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://management.azure.com");
    }

    #[test]
    fn test_endpoint_azure_government() {
        let cloud = CloudConfiguration::AzureGovernment;
        let result = endpoint(&cloud);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://management.usgovcloudapi.net");
    }

    #[test]
    fn test_endpoint_azure_china() {
        let cloud = CloudConfiguration::AzureChina;
        let result = endpoint(&cloud);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://management.chinacloudapi.cn");
    }

    #[test]
    fn test_endpoint_custom_with_endpoint() {
        let mut custom = CustomConfiguration::default();
        custom.authority_host = "https://login.custom.local".to_string();
        custom.audiences =
            Audiences::new().with::<Audience>("https://custom.audience.local/".to_string());
        let cloud = CloudConfiguration::Custom(custom);
        let result = endpoint(&cloud);
        assert!(result.is_err());
    }

    #[test]
    fn test_endpoint_custom_without_endpoint() {
        let custom = CustomConfiguration::default();
        let cloud = CloudConfiguration::Custom(custom);
        let result = endpoint(&cloud);
        assert!(result.is_err());
    }
}

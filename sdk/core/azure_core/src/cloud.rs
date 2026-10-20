// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Azure cloud configuration.

use std::{any::TypeId, borrow::Cow, collections::HashMap};

/// Configurations for different Azure clouds.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum CloudConfiguration {
    /// Azure Public Cloud
    #[default]
    AzurePublic,

    /// Azure Government
    AzureGovernment,

    /// Azure in China
    AzureChina,

    /// A custom cloud.
    ///
    /// # Example
    ///
    /// ```
    /// # mod azure_service_module {
    /// #   pub struct Audience;
    /// # }
    /// use azure_core::cloud::{Audiences, CloudConfiguration, CustomConfiguration};
    ///
    /// let mut custom = CustomConfiguration::default();
    /// custom.audiences = Audiences::new()
    ///     .with::<azure_service_module::Audience>("https://service.mycloud.local".to_string());
    /// custom.authority_host = "https://login.mycloud.local".to_string();
    /// let cloud: CloudConfiguration = custom.into();
    /// ```
    Custom(CustomConfiguration),
}

impl CloudConfiguration {
    /// Gets the default credential scope.
    ///
    /// # Examples
    ///
    /// Get the default scope for a known cloud.
    ///
    /// ```
    /// # mod azure_service_module {
    /// #   pub struct Audience;
    /// # }
    /// use azure_core::cloud::CloudConfiguration;
    ///
    /// let cloud = CloudConfiguration::AzurePublic;
    /// assert!(matches!(
    ///     cloud.default_scope::<azure_service_module::Audience>(),
    ///     Ok(scope) if &scope == "https://management.azure.com/.default",
    /// ));
    /// ```
    ///
    /// For [`CloudConfiguration::Custom`] this returns the service-specific audience + "/.default".
    ///
    /// ```
    /// # mod azure_service_module {
    /// #   pub struct Audience;
    /// # }
    /// use azure_core::cloud::{Audiences, CloudConfiguration, CustomConfiguration};
    ///
    /// let mut custom = CustomConfiguration::default();
    /// custom.audiences = Audiences::new()
    ///     .with::<azure_service_module::Audience>("https://service.mycloud.local".to_string());
    /// let cloud: CloudConfiguration = custom.into();
    /// assert!(matches!(
    ///     cloud.default_scope::<azure_service_module::Audience>(),
    ///     Ok(scope) if &scope == "https://service.mycloud.local/.default",
    /// ));
    /// ```
    pub fn default_scope<T: 'static>(&self) -> crate::Result<Cow<'static, str>> {
        match self {
            Self::AzurePublic => Ok(Cow::Borrowed("https://management.azure.com/.default")),
            Self::AzureGovernment => Ok(Cow::Borrowed(
                "https://management.usgovcloudapi.net/.default",
            )),
            Self::AzureChina => Ok(Cow::Borrowed(
                "https://management.chinacloudapi.cn/.default",
            )),
            Self::Custom(custom) => custom.audiences.get::<T>().map_or_else(
                || {
                    Err(crate::Error::new(
                        crate::error::ErrorKind::Other,
                        "missing required audience",
                    ))
                },
                |audience| Ok(Cow::Owned(audience.to_string() + "/.default")),
            ),
        }
    }

    /// Gets the resource manager (ARM) endpoint.
    ///
    /// For [`CloudConfiguration::Custom`] this returns [`CustomConfiguration::endpoint`].
    #[cfg(feature = "resourcemanager")]
    pub fn endpoint(&self) -> Option<&str> {
        match self {
            Self::AzurePublic => Some("https://management.azure.com"),
            Self::AzureGovernment => Some("https://management.usgovcloudapi.net"),
            Self::AzureChina => Some("https://management.chinacloudapi.cn"),
            Self::Custom(custom) => custom.endpoint.as_deref(),
        }
    }
}

/// Configuration for a custom cloud.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct CustomConfiguration {
    /// Base URL for authentication, for example "https://login.microsoftonline.com"
    pub authority_host: String,

    /// Map of SDK modules to their Entra ID audiences.
    pub audiences: Audiences,

    /// Resource management endpoint.
    #[cfg(feature = "resourcemanager")]
    pub endpoint: Option<String>,
}

impl From<CustomConfiguration> for CloudConfiguration {
    fn from(config: CustomConfiguration) -> Self {
        Self::Custom(config)
    }
}

/// Collection of audiences for an Azure cloud's services
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Audiences(HashMap<TypeId, String>);

impl Audiences {
    /// Create an empty `Audiences` map.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Get a module's audience.
    pub fn get<T: 'static>(&self) -> Option<&str> {
        self.0.get(&TypeId::of::<T>()).map(|s| s.as_str())
    }

    /// Insert or replace an audience.
    pub fn insert<T: 'static>(&mut self, audience: String) {
        self.0.insert(TypeId::of::<T>(), audience);
    }

    /// Insert or replace an audience and return `Self` to allow chaining.
    pub fn with<T: 'static>(mut self, audience: String) -> Self {
        self.0.insert(TypeId::of::<T>(), audience);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom() {
        struct A;
        struct B;
        struct C;

        let cloud = CustomConfiguration {
            authority_host: "https://login.mycloud.local".to_string(),
            audiences: Audiences::new()
                .with::<A>("A".to_string())
                .with::<B>("B".to_string()),
            endpoint: None,
        }
        .into();

        let CloudConfiguration::Custom(mut custom) = cloud else {
            unreachable!();
        };

        assert_eq!(custom.authority_host, "https://login.mycloud.local");
        assert_eq!(custom.audiences.get::<A>(), Some("A"));
        assert_eq!(custom.audiences.get::<B>(), Some("B"));
        assert_eq!(custom.audiences.get::<C>(), None);

        custom.audiences.insert::<C>("C".to_string());
        assert_eq!(custom.audiences.get::<C>(), Some("C"));
    }

    #[test]
    fn default() {
        assert_eq!(
            CloudConfiguration::AzurePublic,
            CloudConfiguration::default()
        );
    }
}

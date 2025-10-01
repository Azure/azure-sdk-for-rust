// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Azure cloud configuration.

use std::{any::TypeId, collections::HashMap};

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
    ///
    /// use azure_core::{
    ///     cloud::{Audiences, CloudConfiguration, CustomConfiguration},
    ///     http::ClientOptions,
    /// };
    ///
    /// let mut custom = CustomConfiguration::default();
    /// custom.audiences = Audiences::new().with::<azure_service_module::Audience>("https://service.mycloud.local".to_string());
    /// custom.authority_host = "https://login.mycloud.local".to_string();
    /// let cloud: CloudConfiguration = custom.into();
    /// ```
    Custom(CustomConfiguration),
}

/// Configuration for a custom cloud.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct CustomConfiguration {
    /// Base URL for authentication, for example "https://login.microsoftonline.com"
    pub authority_host: String,

    /// Map of SDK modules to their Entra ID audiences.
    pub audiences: Audiences,
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
    pub fn audience<T: 'static>(&self) -> Option<&str> {
        self.0.get(&TypeId::of::<T>()).map(|s| s.as_str())
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
        }
        .into();

        let CloudConfiguration::Custom(custom) = cloud else {
            unreachable!();
        };

        assert_eq!(custom.authority_host, "https://login.mycloud.local");
        assert_eq!(custom.audiences.audience::<A>(), Some("A"));
        assert_eq!(custom.audiences.audience::<B>(), Some("B"));
        assert_eq!(custom.audiences.audience::<C>(), None);
    }

    #[test]
    fn default() {
        assert_eq!(
            CloudConfiguration::AzurePublic,
            CloudConfiguration::default()
        );
    }
}

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

impl Default for &CloudConfiguration {
    fn default() -> Self {
        &CloudConfiguration::AzurePublic
    }
}

/// Configuration for a custom cloud.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct CustomConfiguration {
    /// Base URL for authentication, for example "https://login.microsoftonline.com".
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

/// Cloud configuration for Azure Resource Manager (ARM).
#[cfg(feature = "resourcemanager")]
pub mod resourcemanager {
    use super::{CloudConfiguration, CustomConfiguration};
    use std::borrow::Cow;

    /// Marker type for Azure Resource Manager (ARM) audience configuration.
    #[allow(missing_debug_implementations)]
    pub struct Audience;

    /// Helper functions for Azure Resource Manager (ARM) client libraries.
    pub trait CloudConfigurationExt: crate::private::Sealed {
        /// Gets the Azure Resource Manager (ARM) authority.
        ///
        /// For [`CloudConfiguration::Custom`] this returns and audience from [`CustomConfiguration::audiences`] using [`Audience`].
        ///
        /// # Errors
        ///
        /// Returns an error if a custom cloud has no [`Audience`] configured.
        ///
        /// # Examples
        ///
        /// Get the audience for a known cloud.
        ///
        /// ```
        /// use azure_core::cloud::{
        ///     CloudConfiguration,
        ///     resourcemanager::CloudConfigurationExt,
        /// };
        ///
        /// let cloud = CloudConfiguration::AzurePublic;
        /// assert!(matches!(
        ///     cloud.audience(),
        ///     Ok(audience) if audience == "https://management.azure.com",
        /// ));
        /// ```
        ///
        /// For [`CloudConfiguration::Custom`], this returns the service-specific audience.
        ///
        /// ```
        /// use azure_core::cloud::{
        ///     Audiences, CloudConfiguration, CustomConfiguration,
        ///     resourcemanager::{Audience, CloudConfigurationExt},
        /// };
        ///
        /// let mut custom = CustomConfiguration::default();
        /// custom.audiences = Audiences::new()
        ///     .with::<Audience>("https://service.mycloud.local".to_string());
        /// let cloud: CloudConfiguration = custom.into();
        /// assert!(matches!(
        ///     cloud.audience(),
        ///     Ok(audience) if audience == "https://service.mycloud.local",
        /// ));
        /// ```
        fn audience(&self) -> crate::Result<&str>;

        /// Gets the default Azure Resource Manager (ARM) credential scope.
        ///
        /// # Errors
        ///
        /// Returns an error if a custom cloud has no [`Audience`] configured.
        ///
        /// # Examples
        ///
        /// Get the default scope for a known cloud.
        ///
        /// ```
        /// use azure_core::cloud::{
        ///     CloudConfiguration,
        ///     resourcemanager::CloudConfigurationExt,
        /// };
        ///
        /// let cloud = CloudConfiguration::AzurePublic;
        /// assert!(matches!(
        ///     cloud.default_scope(),
        ///     Ok(scope) if &scope == "https://management.azure.com/.default",
        /// ));
        /// ```
        ///
        /// For [`CloudConfiguration::Custom`] this returns the service-specific audience + "/.default".
        ///
        /// ```
        /// use azure_core::cloud::{
        ///     Audiences, CloudConfiguration, CustomConfiguration,
        ///     resourcemanager::{Audience, CloudConfigurationExt},
        /// };
        ///
        /// let mut custom = CustomConfiguration::default();
        /// custom.audiences = Audiences::new()
        ///     .with::<Audience>("https://service.mycloud.local".to_string());
        /// let cloud: CloudConfiguration = custom.into();
        /// assert!(matches!(
        ///     cloud.default_scope(),
        ///     Ok(scope) if &scope == "https://service.mycloud.local/.default",
        /// ));
        /// ```
        fn default_scope(&self) -> crate::Result<Cow<'static, str>>;

        /// Gets the Azure Resource Manager (ARM) endpoint.
        ///
        /// For [`CloudConfiguration::Custom`] this returns [`CustomConfiguration::endpoint`].
        fn endpoint(&self) -> Option<&str>;
    }

    impl CloudConfigurationExt for CloudConfiguration {
        fn audience(&self) -> crate::Result<&str> {
            match self {
                Self::AzurePublic => Ok("https://management.azure.com"),
                Self::AzureGovernment => Ok("https://management.usgovcloudapi.net"),
                Self::AzureChina => Ok("https://management.chinacloudapi.cn"),
                Self::Custom(custom) => custom.audience::<Audience>(),
            }
        }

        fn default_scope(&self) -> crate::Result<Cow<'static, str>> {
            match self {
                Self::AzurePublic => Ok(Cow::Borrowed("https://management.azure.com/.default")),
                Self::AzureGovernment => Ok(Cow::Borrowed(
                    "https://management.usgovcloudapi.net/.default",
                )),
                Self::AzureChina => Ok(Cow::Borrowed(
                    "https://management.chinacloudapi.cn/.default",
                )),
                Self::Custom(custom) => custom
                    .audience::<Audience>()
                    .map(|audience| Cow::Owned(audience.to_string() + "/.default")),
            }
        }

        fn endpoint(&self) -> Option<&str> {
            match self {
                Self::AzurePublic => Some("https://management.azure.com"),
                Self::AzureGovernment => Some("https://management.usgovcloudapi.net"),
                Self::AzureChina => Some("https://management.chinacloudapi.cn"),
                Self::Custom(custom) => custom.endpoint.as_deref(),
            }
        }
    }

    impl crate::private::Sealed for CloudConfiguration {}

    impl CustomConfiguration {
        #[inline]
        fn audience<T: 'static>(&self) -> crate::Result<&str> {
            self.audiences.get::<T>().ok_or_else(|| {
                crate::Error::new(
                    crate::error::ErrorKind::Credential,
                    "missing custom cloud configuration audience",
                )
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Audiences, CloudConfiguration, CustomConfiguration};

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
            #[cfg(feature = "resourcemanager")]
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
            CloudConfiguration::default(),
            CloudConfiguration::AzurePublic,
        );
        let cloud: Option<&CloudConfiguration> = None;
        assert_eq!(
            {
                #[allow(clippy::unnecessary_literal_unwrap)]
                cloud.unwrap_or_default()
            },
            &CloudConfiguration::AzurePublic,
        );
    }
}

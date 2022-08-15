#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Defines the parameters for the cache expiration action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheExpirationActionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: cache_expiration_action_parameters::OdataType,
    #[doc = "Caching behavior for the requests"]
    #[serde(rename = "cacheBehavior")]
    pub cache_behavior: cache_expiration_action_parameters::CacheBehavior,
    #[doc = "The level at which the content needs to be cached."]
    #[serde(rename = "cacheType")]
    pub cache_type: cache_expiration_action_parameters::CacheType,
    #[doc = "The duration for which the content needs to be cached. Allowed format is [d.]hh:mm:ss"]
    #[serde(rename = "cacheDuration", default, skip_serializing_if = "Option::is_none")]
    pub cache_duration: Option<String>,
}
impl CacheExpirationActionParameters {
    pub fn new(
        odata_type: cache_expiration_action_parameters::OdataType,
        cache_behavior: cache_expiration_action_parameters::CacheBehavior,
        cache_type: cache_expiration_action_parameters::CacheType,
    ) -> Self {
        Self {
            odata_type,
            cache_behavior,
            cache_type,
            cache_duration: None,
        }
    }
}
pub mod cache_expiration_action_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleCacheExpirationActionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleCacheExpirationActionParameters,
    }
    #[doc = "Caching behavior for the requests"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CacheBehavior")]
    pub enum CacheBehavior {
        BypassCache,
        Override,
        SetIfMissing,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CacheBehavior {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CacheBehavior {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CacheBehavior {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BypassCache => serializer.serialize_unit_variant("CacheBehavior", 0u32, "BypassCache"),
                Self::Override => serializer.serialize_unit_variant("CacheBehavior", 1u32, "Override"),
                Self::SetIfMissing => serializer.serialize_unit_variant("CacheBehavior", 2u32, "SetIfMissing"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The level at which the content needs to be cached."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CacheType")]
    pub enum CacheType {
        All,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CacheType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CacheType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CacheType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::All => serializer.serialize_unit_variant("CacheType", 0u32, "All"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameters for the cache-key query string action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheKeyQueryStringActionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: cache_key_query_string_action_parameters::OdataType,
    #[doc = "Caching behavior for the requests"]
    #[serde(rename = "queryStringBehavior")]
    pub query_string_behavior: cache_key_query_string_action_parameters::QueryStringBehavior,
    #[doc = "query parameters to include or exclude (comma separated)."]
    #[serde(rename = "queryParameters", default, skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<String>,
}
impl CacheKeyQueryStringActionParameters {
    pub fn new(
        odata_type: cache_key_query_string_action_parameters::OdataType,
        query_string_behavior: cache_key_query_string_action_parameters::QueryStringBehavior,
    ) -> Self {
        Self {
            odata_type,
            query_string_behavior,
            query_parameters: None,
        }
    }
}
pub mod cache_key_query_string_action_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleCacheKeyQueryStringBehaviorActionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleCacheKeyQueryStringBehaviorActionParameters,
    }
    #[doc = "Caching behavior for the requests"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "QueryStringBehavior")]
    pub enum QueryStringBehavior {
        Include,
        IncludeAll,
        Exclude,
        ExcludeAll,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for QueryStringBehavior {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for QueryStringBehavior {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for QueryStringBehavior {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Include => serializer.serialize_unit_variant("QueryStringBehavior", 0u32, "Include"),
                Self::IncludeAll => serializer.serialize_unit_variant("QueryStringBehavior", 1u32, "IncludeAll"),
                Self::Exclude => serializer.serialize_unit_variant("QueryStringBehavior", 2u32, "Exclude"),
                Self::ExcludeAll => serializer.serialize_unit_variant("QueryStringBehavior", 3u32, "ExcludeAll"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameters for using CDN managed certificate for securing custom domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CdnCertificateSourceParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: cdn_certificate_source_parameters::OdataType,
    #[doc = "Type of certificate used"]
    #[serde(rename = "certificateType")]
    pub certificate_type: cdn_certificate_source_parameters::CertificateType,
}
impl CdnCertificateSourceParameters {
    pub fn new(
        odata_type: cdn_certificate_source_parameters::OdataType,
        certificate_type: cdn_certificate_source_parameters::CertificateType,
    ) -> Self {
        Self {
            odata_type,
            certificate_type,
        }
    }
}
pub mod cdn_certificate_source_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.CdnCertificateSourceParameters")]
        MicrosoftAzureCdnModelsCdnCertificateSourceParameters,
    }
    #[doc = "Type of certificate used"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CertificateType")]
    pub enum CertificateType {
        Shared,
        Dedicated,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CertificateType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CertificateType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CertificateType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Shared => serializer.serialize_unit_variant("CertificateType", 0u32, "Shared"),
                Self::Dedicated => serializer.serialize_unit_variant("CertificateType", 1u32, "Dedicated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the certificate source parameters using CDN managed certificate for enabling SSL."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CdnManagedHttpsParameters {
    #[serde(flatten)]
    pub custom_domain_https_parameters: CustomDomainHttpsParameters,
    #[doc = "Defines the parameters for using CDN managed certificate for securing custom domain."]
    #[serde(rename = "certificateSourceParameters")]
    pub certificate_source_parameters: CdnCertificateSourceParameters,
}
impl CdnManagedHttpsParameters {
    pub fn new(
        custom_domain_https_parameters: CustomDomainHttpsParameters,
        certificate_source_parameters: CdnCertificateSourceParameters,
    ) -> Self {
        Self {
            custom_domain_https_parameters,
            certificate_source_parameters,
        }
    }
}
#[doc = "Input of CheckNameAvailability API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityInput {
    #[doc = "The resource name to validate."]
    pub name: String,
    #[doc = "Type of CDN resource used in CheckNameAvailability."]
    #[serde(rename = "type")]
    pub type_: ResourceType,
}
impl CheckNameAvailabilityInput {
    pub fn new(name: String, type_: ResourceType) -> Self {
        Self { name, type_ }
    }
}
#[doc = "Output of check name availability API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityOutput {
    #[doc = "Indicates whether the name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason why the name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The detailed error message describing why the name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the parameters for Cookies match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CookiesMatchConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: cookies_match_condition_parameters::OdataType,
    #[doc = "Name of Cookies to be matched"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[doc = "Describes operator to be matched"]
    pub operator: cookies_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(rename = "matchValues", default, skip_serializing_if = "Vec::is_empty")]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transforms: Vec<Transform>,
}
impl CookiesMatchConditionParameters {
    pub fn new(odata_type: cookies_match_condition_parameters::OdataType, operator: cookies_match_condition_parameters::Operator) -> Self {
        Self {
            odata_type,
            selector: None,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod cookies_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleCookiesConditionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleCookiesConditionParameters,
    }
    #[doc = "Describes operator to be matched"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Any,
        Equal,
        Contains,
        BeginsWith,
        EndsWith,
        LessThan,
        LessThanOrEqual,
        GreaterThan,
        GreaterThanOrEqual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Any => serializer.serialize_unit_variant("Operator", 0u32, "Any"),
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Friendly domain name mapping to the endpoint hostname that the customer provides for branding purposes, e.g. www.contoso.com."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomDomain {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The JSON object that contains the properties of the custom domain to create."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomDomainProperties>,
}
impl CustomDomain {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties to secure a custom domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainHttpsParameters {
    #[doc = "Defines the source of the SSL certificate."]
    #[serde(rename = "certificateSource")]
    pub certificate_source: custom_domain_https_parameters::CertificateSource,
    #[doc = "Defines the TLS extension protocol that is used for secure delivery."]
    #[serde(rename = "protocolType")]
    pub protocol_type: custom_domain_https_parameters::ProtocolType,
    #[doc = "TLS protocol version that will be used for Https"]
    #[serde(rename = "minimumTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimum_tls_version: Option<custom_domain_https_parameters::MinimumTlsVersion>,
}
impl CustomDomainHttpsParameters {
    pub fn new(
        certificate_source: custom_domain_https_parameters::CertificateSource,
        protocol_type: custom_domain_https_parameters::ProtocolType,
    ) -> Self {
        Self {
            certificate_source,
            protocol_type,
            minimum_tls_version: None,
        }
    }
}
pub mod custom_domain_https_parameters {
    use super::*;
    #[doc = "Defines the source of the SSL certificate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CertificateSource")]
    pub enum CertificateSource {
        AzureKeyVault,
        Cdn,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CertificateSource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CertificateSource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CertificateSource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureKeyVault => serializer.serialize_unit_variant("CertificateSource", 0u32, "AzureKeyVault"),
                Self::Cdn => serializer.serialize_unit_variant("CertificateSource", 1u32, "Cdn"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Defines the TLS extension protocol that is used for secure delivery."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProtocolType")]
    pub enum ProtocolType {
        ServerNameIndication,
        #[serde(rename = "IPBased")]
        IpBased,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProtocolType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProtocolType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProtocolType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ServerNameIndication => serializer.serialize_unit_variant("ProtocolType", 0u32, "ServerNameIndication"),
                Self::IpBased => serializer.serialize_unit_variant("ProtocolType", 1u32, "IPBased"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "TLS protocol version that will be used for Https"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MinimumTlsVersion {
        None,
        #[serde(rename = "TLS10")]
        Tls10,
        #[serde(rename = "TLS12")]
        Tls12,
    }
}
#[doc = "Result of the request to list custom domains. It contains a list of custom domain objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomDomainListResult {
    #[doc = "List of CDN CustomDomains within an endpoint."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomDomain>,
    #[doc = "URL to get the next set of custom domain objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CustomDomainListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CustomDomainListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The customDomain JSON object required for custom domain creation or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomDomainParameters {
    #[doc = "The JSON object that contains the properties of the custom domain to create."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomDomainPropertiesParameters>,
}
impl CustomDomainParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties of the custom domain to create."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainProperties {
    #[doc = "The host name of the custom domain. Must be a domain name."]
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[doc = "Resource status of the custom domain."]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<custom_domain_properties::ResourceState>,
    #[doc = "Provisioning status of Custom Https of the custom domain."]
    #[serde(rename = "customHttpsProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub custom_https_provisioning_state: Option<custom_domain_properties::CustomHttpsProvisioningState>,
    #[doc = "Provisioning substate shows the progress of custom HTTPS enabling/disabling process step by step."]
    #[serde(rename = "customHttpsProvisioningSubstate", default, skip_serializing_if = "Option::is_none")]
    pub custom_https_provisioning_substate: Option<custom_domain_properties::CustomHttpsProvisioningSubstate>,
    #[doc = "The JSON object that contains the properties to secure a custom domain."]
    #[serde(rename = "customHttpsParameters", default, skip_serializing_if = "Option::is_none")]
    pub custom_https_parameters: Option<CustomDomainHttpsParameters>,
    #[doc = "Special validation or data may be required when delivering CDN to some regions due to local compliance reasons. E.g. ICP license number of a custom domain is required to deliver content in China."]
    #[serde(rename = "validationData", default, skip_serializing_if = "Option::is_none")]
    pub validation_data: Option<String>,
    #[doc = "Provisioning status of the custom domain."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl CustomDomainProperties {
    pub fn new(host_name: String) -> Self {
        Self {
            host_name,
            resource_state: None,
            custom_https_provisioning_state: None,
            custom_https_provisioning_substate: None,
            custom_https_parameters: None,
            validation_data: None,
            provisioning_state: None,
        }
    }
}
pub mod custom_domain_properties {
    use super::*;
    #[doc = "Resource status of the custom domain."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceState")]
    pub enum ResourceState {
        Creating,
        Active,
        Deleting,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Creating => serializer.serialize_unit_variant("ResourceState", 0u32, "Creating"),
                Self::Active => serializer.serialize_unit_variant("ResourceState", 1u32, "Active"),
                Self::Deleting => serializer.serialize_unit_variant("ResourceState", 2u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning status of Custom Https of the custom domain."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CustomHttpsProvisioningState")]
    pub enum CustomHttpsProvisioningState {
        Enabling,
        Enabled,
        Disabling,
        Disabled,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CustomHttpsProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CustomHttpsProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CustomHttpsProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabling => serializer.serialize_unit_variant("CustomHttpsProvisioningState", 0u32, "Enabling"),
                Self::Enabled => serializer.serialize_unit_variant("CustomHttpsProvisioningState", 1u32, "Enabled"),
                Self::Disabling => serializer.serialize_unit_variant("CustomHttpsProvisioningState", 2u32, "Disabling"),
                Self::Disabled => serializer.serialize_unit_variant("CustomHttpsProvisioningState", 3u32, "Disabled"),
                Self::Failed => serializer.serialize_unit_variant("CustomHttpsProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning substate shows the progress of custom HTTPS enabling/disabling process step by step."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CustomHttpsProvisioningSubstate")]
    pub enum CustomHttpsProvisioningSubstate {
        SubmittingDomainControlValidationRequest,
        PendingDomainControlValidationREquestApproval,
        DomainControlValidationRequestApproved,
        DomainControlValidationRequestRejected,
        DomainControlValidationRequestTimedOut,
        IssuingCertificate,
        DeployingCertificate,
        CertificateDeployed,
        DeletingCertificate,
        CertificateDeleted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CustomHttpsProvisioningSubstate {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CustomHttpsProvisioningSubstate {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CustomHttpsProvisioningSubstate {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SubmittingDomainControlValidationRequest => {
                    serializer.serialize_unit_variant("CustomHttpsProvisioningSubstate", 0u32, "SubmittingDomainControlValidationRequest")
                }
                Self::PendingDomainControlValidationREquestApproval => serializer.serialize_unit_variant(
                    "CustomHttpsProvisioningSubstate",
                    1u32,
                    "PendingDomainControlValidationREquestApproval",
                ),
                Self::DomainControlValidationRequestApproved => {
                    serializer.serialize_unit_variant("CustomHttpsProvisioningSubstate", 2u32, "DomainControlValidationRequestApproved")
                }
                Self::DomainControlValidationRequestRejected => {
                    serializer.serialize_unit_variant("CustomHttpsProvisioningSubstate", 3u32, "DomainControlValidationRequestRejected")
                }
                Self::DomainControlValidationRequestTimedOut => {
                    serializer.serialize_unit_variant("CustomHttpsProvisioningSubstate", 4u32, "DomainControlValidationRequestTimedOut")
                }
                Self::IssuingCertificate => {
                    serializer.serialize_unit_variant("CustomHttpsProvisioningSubstate", 5u32, "IssuingCertificate")
                }
                Self::DeployingCertificate => {
                    serializer.serialize_unit_variant("CustomHttpsProvisioningSubstate", 6u32, "DeployingCertificate")
                }
                Self::CertificateDeployed => {
                    serializer.serialize_unit_variant("CustomHttpsProvisioningSubstate", 7u32, "CertificateDeployed")
                }
                Self::DeletingCertificate => {
                    serializer.serialize_unit_variant("CustomHttpsProvisioningSubstate", 8u32, "DeletingCertificate")
                }
                Self::CertificateDeleted => {
                    serializer.serialize_unit_variant("CustomHttpsProvisioningSubstate", 9u32, "CertificateDeleted")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The JSON object that contains the properties of the custom domain to create."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainPropertiesParameters {
    #[doc = "The host name of the custom domain. Must be a domain name."]
    #[serde(rename = "hostName")]
    pub host_name: String,
}
impl CustomDomainPropertiesParameters {
    pub fn new(host_name: String) -> Self {
        Self { host_name }
    }
}
#[doc = "The main origin of CDN content which is added when creating a CDN endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeepCreatedOrigin {
    #[doc = "Origin name which must be unique within the endpoint. "]
    pub name: String,
    #[doc = "Properties of the origin created on the CDN endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeepCreatedOriginProperties>,
}
impl DeepCreatedOrigin {
    pub fn new(name: String) -> Self {
        Self { name, properties: None }
    }
}
#[doc = "The origin group for CDN content which is added when creating a CDN endpoint. Traffic is sent to the origins within the origin group based on origin health."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeepCreatedOriginGroup {
    #[doc = "Origin group name which must be unique within the endpoint."]
    pub name: String,
    #[doc = "Properties of the origin group created on the CDN endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeepCreatedOriginGroupProperties>,
}
impl DeepCreatedOriginGroup {
    pub fn new(name: String) -> Self {
        Self { name, properties: None }
    }
}
#[doc = "Properties of the origin group created on the CDN endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeepCreatedOriginGroupProperties {
    #[doc = "The JSON object that contains the properties to send health probes to origin."]
    #[serde(rename = "healthProbeSettings", default, skip_serializing_if = "Option::is_none")]
    pub health_probe_settings: Option<HealthProbeParameters>,
    #[doc = "The source of the content being delivered via CDN within given origin group."]
    pub origins: Vec<ResourceReference>,
    #[doc = "Time in minutes to shift the traffic to the endpoint gradually when an unhealthy endpoint comes healthy or a new endpoint is added. Default is 10 mins. This property is currently not supported."]
    #[serde(
        rename = "trafficRestorationTimeToHealedOrNewEndpointsInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_restoration_time_to_healed_or_new_endpoints_in_minutes: Option<i64>,
    #[doc = "The JSON object that contains the properties to determine origin health using real requests/responses."]
    #[serde(
        rename = "responseBasedOriginErrorDetectionSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_based_origin_error_detection_settings: Option<ResponseBasedOriginErrorDetectionParameters>,
}
impl DeepCreatedOriginGroupProperties {
    pub fn new(origins: Vec<ResourceReference>) -> Self {
        Self {
            health_probe_settings: None,
            origins,
            traffic_restoration_time_to_healed_or_new_endpoints_in_minutes: None,
            response_based_origin_error_detection_settings: None,
        }
    }
}
#[doc = "Properties of the origin created on the CDN endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeepCreatedOriginProperties {
    #[doc = "The address of the origin. It can be a domain name, IPv4 address, or IPv6 address. This should be unique across all origins in an endpoint."]
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[doc = "The value of the HTTP port. Must be between 1 and 65535."]
    #[serde(rename = "httpPort", default, skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,
    #[doc = "The value of the HTTPS port. Must be between 1 and 65535."]
    #[serde(rename = "httpsPort", default, skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,
    #[doc = "The host header value sent to the origin with each request. If you leave this blank, the request hostname determines this value. Azure CDN origins, such as Web Apps, Blob Storage, and Cloud Services require this host header value to match the origin hostname by default. If endpoint uses multiple origins for load balancing, then the host header at endpoint is ignored and this one is considered."]
    #[serde(rename = "originHostHeader", default, skip_serializing_if = "Option::is_none")]
    pub origin_host_header: Option<String>,
    #[doc = "Priority of origin in given origin group for load balancing. Higher priorities will not be used for load balancing if any lower priority origin is healthy.Must be between 1 and 5."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[doc = "Weight of the origin in given origin group for load balancing. Must be between 1 and 1000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
    #[doc = "Origin is enabled for load balancing or not. By default, origin is always enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl DeepCreatedOriginProperties {
    pub fn new(host_name: String) -> Self {
        Self {
            host_name,
            http_port: None,
            https_port: None,
            origin_host_header: None,
            priority: None,
            weight: None,
            enabled: None,
        }
    }
}
#[doc = "A rule that specifies a set of actions and conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRule {
    #[doc = "Name of the rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The order in which the rules are applied for the endpoint. Possible values {0,1,2,3,………}. A rule with a lesser order will be applied before a rule with a greater order. Rule with order 0 is a special rule. It does not require any condition and actions listed in it will always be applied."]
    pub order: i64,
    #[doc = "A list of conditions that must be matched for the actions to be executed"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<DeliveryRuleCondition>,
    #[doc = "A list of actions that are executed when all the conditions of a rule are satisfied."]
    pub actions: Vec<DeliveryRuleAction>,
}
impl DeliveryRule {
    pub fn new(order: i64, actions: Vec<DeliveryRuleAction>) -> Self {
        Self {
            name: None,
            order,
            conditions: Vec::new(),
            actions,
        }
    }
}
#[doc = "An action for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleAction {
    #[doc = "The name of the action for the delivery rule."]
    pub name: delivery_rule_action::Name,
}
impl DeliveryRuleAction {
    pub fn new(name: delivery_rule_action::Name) -> Self {
        Self { name }
    }
}
pub mod delivery_rule_action {
    use super::*;
    #[doc = "The name of the action for the delivery rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        CacheExpiration,
        CacheKeyQueryString,
        ModifyRequestHeader,
        ModifyResponseHeader,
        UrlRedirect,
        UrlRewrite,
        OriginGroupOverride,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CacheExpiration => serializer.serialize_unit_variant("Name", 0u32, "CacheExpiration"),
                Self::CacheKeyQueryString => serializer.serialize_unit_variant("Name", 1u32, "CacheKeyQueryString"),
                Self::ModifyRequestHeader => serializer.serialize_unit_variant("Name", 2u32, "ModifyRequestHeader"),
                Self::ModifyResponseHeader => serializer.serialize_unit_variant("Name", 3u32, "ModifyResponseHeader"),
                Self::UrlRedirect => serializer.serialize_unit_variant("Name", 4u32, "UrlRedirect"),
                Self::UrlRewrite => serializer.serialize_unit_variant("Name", 5u32, "UrlRewrite"),
                Self::OriginGroupOverride => serializer.serialize_unit_variant("Name", 6u32, "OriginGroupOverride"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the cache expiration action for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleCacheExpirationAction {
    #[serde(flatten)]
    pub delivery_rule_action: DeliveryRuleAction,
    #[doc = "Defines the parameters for the cache expiration action."]
    pub parameters: CacheExpirationActionParameters,
}
impl DeliveryRuleCacheExpirationAction {
    pub fn new(delivery_rule_action: DeliveryRuleAction, parameters: CacheExpirationActionParameters) -> Self {
        Self {
            delivery_rule_action,
            parameters,
        }
    }
}
#[doc = "Defines the cache-key query string action for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleCacheKeyQueryStringAction {
    #[serde(flatten)]
    pub delivery_rule_action: DeliveryRuleAction,
    #[doc = "Defines the parameters for the cache-key query string action."]
    pub parameters: CacheKeyQueryStringActionParameters,
}
impl DeliveryRuleCacheKeyQueryStringAction {
    pub fn new(delivery_rule_action: DeliveryRuleAction, parameters: CacheKeyQueryStringActionParameters) -> Self {
        Self {
            delivery_rule_action,
            parameters,
        }
    }
}
#[doc = "A condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleCondition {
    #[doc = "The name of the condition for the delivery rule."]
    pub name: delivery_rule_condition::Name,
}
impl DeliveryRuleCondition {
    pub fn new(name: delivery_rule_condition::Name) -> Self {
        Self { name }
    }
}
pub mod delivery_rule_condition {
    use super::*;
    #[doc = "The name of the condition for the delivery rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        RemoteAddress,
        RequestMethod,
        QueryString,
        PostArgs,
        RequestUri,
        RequestHeader,
        RequestBody,
        RequestScheme,
        UrlPath,
        UrlFileExtension,
        UrlFileName,
        HttpVersion,
        Cookies,
        IsDevice,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::RemoteAddress => serializer.serialize_unit_variant("Name", 0u32, "RemoteAddress"),
                Self::RequestMethod => serializer.serialize_unit_variant("Name", 1u32, "RequestMethod"),
                Self::QueryString => serializer.serialize_unit_variant("Name", 2u32, "QueryString"),
                Self::PostArgs => serializer.serialize_unit_variant("Name", 3u32, "PostArgs"),
                Self::RequestUri => serializer.serialize_unit_variant("Name", 4u32, "RequestUri"),
                Self::RequestHeader => serializer.serialize_unit_variant("Name", 5u32, "RequestHeader"),
                Self::RequestBody => serializer.serialize_unit_variant("Name", 6u32, "RequestBody"),
                Self::RequestScheme => serializer.serialize_unit_variant("Name", 7u32, "RequestScheme"),
                Self::UrlPath => serializer.serialize_unit_variant("Name", 8u32, "UrlPath"),
                Self::UrlFileExtension => serializer.serialize_unit_variant("Name", 9u32, "UrlFileExtension"),
                Self::UrlFileName => serializer.serialize_unit_variant("Name", 10u32, "UrlFileName"),
                Self::HttpVersion => serializer.serialize_unit_variant("Name", 11u32, "HttpVersion"),
                Self::Cookies => serializer.serialize_unit_variant("Name", 12u32, "Cookies"),
                Self::IsDevice => serializer.serialize_unit_variant("Name", 13u32, "IsDevice"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the Cookies condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleCookiesCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for Cookies match conditions"]
    pub parameters: CookiesMatchConditionParameters,
}
impl DeliveryRuleCookiesCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: CookiesMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[doc = "Defines the HttpVersion condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleHttpVersionCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for HttpVersion match conditions"]
    pub parameters: HttpVersionMatchConditionParameters,
}
impl DeliveryRuleHttpVersionCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: HttpVersionMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[doc = "Defines the IsDevice condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleIsDeviceCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for IsDevice match conditions"]
    pub parameters: IsDeviceMatchConditionParameters,
}
impl DeliveryRuleIsDeviceCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: IsDeviceMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[doc = "Defines the PostArgs condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRulePostArgsCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for PostArgs match conditions"]
    pub parameters: PostArgsMatchConditionParameters,
}
impl DeliveryRulePostArgsCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: PostArgsMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[doc = "Defines the QueryString condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleQueryStringCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for QueryString match conditions"]
    pub parameters: QueryStringMatchConditionParameters,
}
impl DeliveryRuleQueryStringCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: QueryStringMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[doc = "Defines the RemoteAddress condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleRemoteAddressCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for RemoteAddress match conditions"]
    pub parameters: RemoteAddressMatchConditionParameters,
}
impl DeliveryRuleRemoteAddressCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: RemoteAddressMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[doc = "Defines the RequestBody condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleRequestBodyCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for RequestBody match conditions"]
    pub parameters: RequestBodyMatchConditionParameters,
}
impl DeliveryRuleRequestBodyCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: RequestBodyMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[doc = "Defines the request header action for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleRequestHeaderAction {
    #[serde(flatten)]
    pub delivery_rule_action: DeliveryRuleAction,
    #[doc = "Defines the parameters for the request header action."]
    pub parameters: HeaderActionParameters,
}
impl DeliveryRuleRequestHeaderAction {
    pub fn new(delivery_rule_action: DeliveryRuleAction, parameters: HeaderActionParameters) -> Self {
        Self {
            delivery_rule_action,
            parameters,
        }
    }
}
#[doc = "Defines the RequestHeader condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleRequestHeaderCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for RequestHeader match conditions"]
    pub parameters: RequestHeaderMatchConditionParameters,
}
impl DeliveryRuleRequestHeaderCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: RequestHeaderMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[doc = "Defines the RequestMethod condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleRequestMethodCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for RequestMethod match conditions"]
    pub parameters: RequestMethodMatchConditionParameters,
}
impl DeliveryRuleRequestMethodCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: RequestMethodMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[doc = "Defines the RequestScheme condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleRequestSchemeCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for RequestScheme match conditions "]
    pub parameters: RequestSchemeMatchConditionParameters,
}
impl DeliveryRuleRequestSchemeCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: RequestSchemeMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[doc = "Defines the RequestUri condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleRequestUriCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for RequestUri match conditions"]
    pub parameters: RequestUriMatchConditionParameters,
}
impl DeliveryRuleRequestUriCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: RequestUriMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[doc = "Defines the response header action for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleResponseHeaderAction {
    #[serde(flatten)]
    pub delivery_rule_action: DeliveryRuleAction,
    #[doc = "Defines the parameters for the request header action."]
    pub parameters: HeaderActionParameters,
}
impl DeliveryRuleResponseHeaderAction {
    pub fn new(delivery_rule_action: DeliveryRuleAction, parameters: HeaderActionParameters) -> Self {
        Self {
            delivery_rule_action,
            parameters,
        }
    }
}
#[doc = "Defines the UrlFileExtension condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleUrlFileExtensionCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for UrlFileExtension match conditions"]
    pub parameters: UrlFileExtensionMatchConditionParameters,
}
impl DeliveryRuleUrlFileExtensionCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: UrlFileExtensionMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[doc = "Defines the UrlFileName condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleUrlFileNameCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for UrlFilename match conditions"]
    pub parameters: UrlFileNameMatchConditionParameters,
}
impl DeliveryRuleUrlFileNameCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: UrlFileNameMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[doc = "Defines the UrlPath condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleUrlPathCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for UrlPath match conditions"]
    pub parameters: UrlPathMatchConditionParameters,
}
impl DeliveryRuleUrlPathCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: UrlPathMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[doc = "Edgenode is a global Point of Presence (POP) location used to deliver CDN content to end users."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgeNode {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The JSON object that contains the properties required to create an edgenode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EdgeNodeProperties>,
}
impl EdgeNode {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties required to create an edgenode."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeNodeProperties {
    #[doc = "List of ip address groups."]
    #[serde(rename = "ipAddressGroups")]
    pub ip_address_groups: Vec<IpAddressGroup>,
}
impl EdgeNodeProperties {
    pub fn new(ip_address_groups: Vec<IpAddressGroup>) -> Self {
        Self { ip_address_groups }
    }
}
#[doc = "Result of the request to list CDN edgenodes. It contains a list of ip address group and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgenodeResult {
    #[doc = "Edge node of CDN service."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EdgeNode>,
    #[doc = "URL to get the next set of edgenode list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EdgenodeResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EdgenodeResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CDN endpoint is the entity within a CDN profile containing configuration information such as origin, protocol, content caching and delivery behavior. The CDN endpoint uses the URL format <endpointname>.azureedge.net."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Endpoint {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The JSON object that contains the properties required to create an endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EndpointProperties>,
}
impl Endpoint {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Result of the request to list endpoints. It contains a list of endpoint objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointListResult {
    #[doc = "List of CDN endpoints within a profile"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Endpoint>,
    #[doc = "URL to get the next set of endpoint objects if there is any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EndpointListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EndpointListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties required to create an endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointProperties {
    #[serde(flatten)]
    pub endpoint_properties_update_parameters: EndpointPropertiesUpdateParameters,
    #[doc = "The host name of the endpoint structured as {endpointName}.{DNSZone}, e.g. contoso.azureedge.net"]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "The source of the content being delivered via CDN."]
    pub origins: Vec<DeepCreatedOrigin>,
    #[doc = "The origin groups comprising of origins that are used for load balancing the traffic based on availability."]
    #[serde(rename = "originGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub origin_groups: Vec<DeepCreatedOriginGroup>,
    #[doc = "Resource status of the endpoint."]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<endpoint_properties::ResourceState>,
    #[doc = "Provisioning status of the endpoint."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl EndpointProperties {
    pub fn new(origins: Vec<DeepCreatedOrigin>) -> Self {
        Self {
            endpoint_properties_update_parameters: EndpointPropertiesUpdateParameters::default(),
            host_name: None,
            origins,
            origin_groups: Vec::new(),
            resource_state: None,
            provisioning_state: None,
        }
    }
}
pub mod endpoint_properties {
    use super::*;
    #[doc = "Resource status of the endpoint."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceState")]
    pub enum ResourceState {
        Creating,
        Deleting,
        Running,
        Starting,
        Stopped,
        Stopping,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Creating => serializer.serialize_unit_variant("ResourceState", 0u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ResourceState", 1u32, "Deleting"),
                Self::Running => serializer.serialize_unit_variant("ResourceState", 2u32, "Running"),
                Self::Starting => serializer.serialize_unit_variant("ResourceState", 3u32, "Starting"),
                Self::Stopped => serializer.serialize_unit_variant("ResourceState", 4u32, "Stopped"),
                Self::Stopping => serializer.serialize_unit_variant("ResourceState", 5u32, "Stopping"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The JSON object containing endpoint update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointPropertiesUpdateParameters {
    #[doc = "A directory path on the origin that CDN can use to retrieve content from, e.g. contoso.cloudapp.net/originpath."]
    #[serde(rename = "originPath", default, skip_serializing_if = "Option::is_none")]
    pub origin_path: Option<String>,
    #[doc = "List of content types on which compression applies. The value should be a valid MIME type."]
    #[serde(rename = "contentTypesToCompress", default, skip_serializing_if = "Vec::is_empty")]
    pub content_types_to_compress: Vec<String>,
    #[doc = "The host header value sent to the origin with each request. This property at Endpoint can only be set allowed when endpoint uses single origin. If you leave this blank, the request hostname determines this value. Azure CDN origins, such as Web Apps, Blob Storage, and Cloud Services require this host header value to match the origin hostname by default."]
    #[serde(rename = "originHostHeader", default, skip_serializing_if = "Option::is_none")]
    pub origin_host_header: Option<String>,
    #[doc = "Indicates whether content compression is enabled on CDN. Default value is false. If compression is enabled, content will be served as compressed if user requests for a compressed version. Content won't be compressed on CDN when requested content is smaller than 1 byte or larger than 1 MB."]
    #[serde(rename = "isCompressionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_compression_enabled: Option<bool>,
    #[doc = "Indicates whether HTTP traffic is allowed on the endpoint. Default value is true. At least one protocol (HTTP or HTTPS) must be allowed."]
    #[serde(rename = "isHttpAllowed", default, skip_serializing_if = "Option::is_none")]
    pub is_http_allowed: Option<bool>,
    #[doc = "Indicates whether HTTPS traffic is allowed on the endpoint. Default value is true. At least one protocol (HTTP or HTTPS) must be allowed."]
    #[serde(rename = "isHttpsAllowed", default, skip_serializing_if = "Option::is_none")]
    pub is_https_allowed: Option<bool>,
    #[doc = "Defines how CDN caches requests that include query strings. You can ignore any query strings when caching, bypass caching to prevent requests that contain query strings from being cached, or cache every request with a unique URL."]
    #[serde(rename = "queryStringCachingBehavior", default, skip_serializing_if = "Option::is_none")]
    pub query_string_caching_behavior: Option<QueryStringCachingBehavior>,
    #[doc = "Specifies what scenario the customer wants this CDN endpoint to optimize, e.g. Download, Media services. With this information we can apply scenario driven optimization."]
    #[serde(rename = "optimizationType", default, skip_serializing_if = "Option::is_none")]
    pub optimization_type: Option<OptimizationType>,
    #[doc = "Path to a file hosted on the origin which helps accelerate delivery of the dynamic content and calculate the most optimal routes for the CDN. This is relative to the origin path. This property is only relevant when using a single origin."]
    #[serde(rename = "probePath", default, skip_serializing_if = "Option::is_none")]
    pub probe_path: Option<String>,
    #[doc = "List of rules defining the user's geo access within a CDN endpoint. Each geo filter defines an access rule to a specified path or content, e.g. block APAC for path /pictures/"]
    #[serde(rename = "geoFilters", default, skip_serializing_if = "Vec::is_empty")]
    pub geo_filters: Vec<GeoFilter>,
    #[doc = "Reference to another resource."]
    #[serde(rename = "defaultOriginGroup", default, skip_serializing_if = "Option::is_none")]
    pub default_origin_group: Option<ResourceReference>,
    #[doc = "A policy that specifies the delivery rules to be used for an endpoint."]
    #[serde(rename = "deliveryPolicy", default, skip_serializing_if = "Option::is_none")]
    pub delivery_policy: Option<endpoint_properties_update_parameters::DeliveryPolicy>,
}
impl EndpointPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod endpoint_properties_update_parameters {
    use super::*;
    #[doc = "A policy that specifies the delivery rules to be used for an endpoint."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DeliveryPolicy {
        #[doc = "User-friendly description of the policy."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "A list of the delivery rules."]
        pub rules: Vec<DeliveryRule>,
    }
    impl DeliveryPolicy {
        pub fn new(rules: Vec<DeliveryRule>) -> Self {
            Self { description: None, rules }
        }
    }
}
#[doc = "Properties required to create or update an endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointUpdateParameters {
    #[doc = "Endpoint tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The JSON object containing endpoint update parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EndpointPropertiesUpdateParameters>,
}
impl EndpointUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates CDN service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rules defining user's geo access within a CDN endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoFilter {
    #[doc = "Relative path applicable to geo filter. (e.g. '/mypictures', '/mypicture/kitty.jpg', and etc.)"]
    #[serde(rename = "relativePath")]
    pub relative_path: String,
    #[doc = "Action of the geo filter, i.e. allow or block access."]
    pub action: geo_filter::Action,
    #[doc = "Two letter country codes defining user country access in a geo filter, e.g. AU, MX, US."]
    #[serde(rename = "countryCodes")]
    pub country_codes: Vec<String>,
}
impl GeoFilter {
    pub fn new(relative_path: String, action: geo_filter::Action, country_codes: Vec<String>) -> Self {
        Self {
            relative_path,
            action,
            country_codes,
        }
    }
}
pub mod geo_filter {
    use super::*;
    #[doc = "Action of the geo filter, i.e. allow or block access."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Block,
        Allow,
    }
}
#[doc = "Defines the parameters for the request header action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeaderActionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: header_action_parameters::OdataType,
    #[doc = "Action to perform"]
    #[serde(rename = "headerAction")]
    pub header_action: header_action_parameters::HeaderAction,
    #[doc = "Name of the header to modify"]
    #[serde(rename = "headerName")]
    pub header_name: String,
    #[doc = "Value for the specified action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl HeaderActionParameters {
    pub fn new(
        odata_type: header_action_parameters::OdataType,
        header_action: header_action_parameters::HeaderAction,
        header_name: String,
    ) -> Self {
        Self {
            odata_type,
            header_action,
            header_name,
            value: None,
        }
    }
}
pub mod header_action_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleHeaderActionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleHeaderActionParameters,
    }
    #[doc = "Action to perform"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HeaderAction")]
    pub enum HeaderAction {
        Append,
        Overwrite,
        Delete,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HeaderAction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HeaderAction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HeaderAction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Append => serializer.serialize_unit_variant("HeaderAction", 0u32, "Append"),
                Self::Overwrite => serializer.serialize_unit_variant("HeaderAction", 1u32, "Overwrite"),
                Self::Delete => serializer.serialize_unit_variant("HeaderAction", 2u32, "Delete"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The JSON object that contains the properties to send health probes to origin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthProbeParameters {
    #[doc = "The path relative to the origin that is used to determine the health of the origin."]
    #[serde(rename = "probePath", default, skip_serializing_if = "Option::is_none")]
    pub probe_path: Option<String>,
    #[doc = "The type of health probe request that is made."]
    #[serde(rename = "probeRequestType", default, skip_serializing_if = "Option::is_none")]
    pub probe_request_type: Option<health_probe_parameters::ProbeRequestType>,
    #[doc = "Protocol to use for health probe."]
    #[serde(rename = "probeProtocol", default, skip_serializing_if = "Option::is_none")]
    pub probe_protocol: Option<health_probe_parameters::ProbeProtocol>,
    #[doc = "The number of seconds between health probes.Default is 240sec."]
    #[serde(rename = "probeIntervalInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub probe_interval_in_seconds: Option<i64>,
}
impl HealthProbeParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod health_probe_parameters {
    use super::*;
    #[doc = "The type of health probe request that is made."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProbeRequestType {
        NotSet,
        #[serde(rename = "GET")]
        Get,
        #[serde(rename = "HEAD")]
        Head,
    }
    #[doc = "Protocol to use for health probe."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProbeProtocol {
        NotSet,
        Http,
        Https,
    }
}
#[doc = "The JSON object that represents the range for http status codes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpErrorRangeParameters {
    #[doc = "The inclusive start of the http status code range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub begin: Option<i64>,
    #[doc = "The inclusive end of the http status code range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
}
impl HttpErrorRangeParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the parameters for HttpVersion match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpVersionMatchConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: http_version_match_condition_parameters::OdataType,
    #[doc = "Describes operator to be matched"]
    pub operator: http_version_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(rename = "matchValues", default, skip_serializing_if = "Vec::is_empty")]
    pub match_values: Vec<String>,
}
impl HttpVersionMatchConditionParameters {
    pub fn new(
        odata_type: http_version_match_condition_parameters::OdataType,
        operator: http_version_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            odata_type,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
        }
    }
}
pub mod http_version_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleHttpVersionConditionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleHttpVersionConditionParameters,
    }
    #[doc = "Describes operator to be matched"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Equal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Equal => serializer.serialize_unit_variant("Operator", 0u32, "Equal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "CDN Ip address group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpAddressGroup {
    #[doc = "The delivery region of the ip address group"]
    #[serde(rename = "deliveryRegion", default, skip_serializing_if = "Option::is_none")]
    pub delivery_region: Option<String>,
    #[doc = "The list of ip v4 addresses."]
    #[serde(rename = "ipv4Addresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ipv4_addresses: Vec<CidrIpAddress>,
    #[doc = "The list of ip v6 addresses."]
    #[serde(rename = "ipv6Addresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ipv6_addresses: Vec<CidrIpAddress>,
}
impl IpAddressGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the parameters for IsDevice match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IsDeviceMatchConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: is_device_match_condition_parameters::OdataType,
    #[doc = "Describes operator to be matched"]
    pub operator: is_device_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(rename = "matchValues", default, skip_serializing_if = "Vec::is_empty")]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transforms: Vec<Transform>,
}
impl IsDeviceMatchConditionParameters {
    pub fn new(
        odata_type: is_device_match_condition_parameters::OdataType,
        operator: is_device_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            odata_type,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod is_device_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleIsDeviceConditionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleIsDeviceConditionParameters,
    }
    #[doc = "Describes operator to be matched"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Equal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Equal => serializer.serialize_unit_variant("Operator", 0u32, "Equal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the parameters for using a user's KeyVault certificate for securing custom domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultCertificateSourceParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: key_vault_certificate_source_parameters::OdataType,
    #[doc = "Subscription Id of the user's Key Vault containing the SSL certificate"]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
    #[doc = "Resource group of the user's Key Vault containing the SSL certificate"]
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
    #[doc = "The name of the user's Key Vault containing the SSL certificate"]
    #[serde(rename = "vaultName")]
    pub vault_name: String,
    #[doc = "The name of Key Vault Secret (representing the full certificate PFX) in Key Vault."]
    #[serde(rename = "secretName")]
    pub secret_name: String,
    #[doc = "The version(GUID) of Key Vault Secret in Key Vault."]
    #[serde(rename = "secretVersion", default, skip_serializing_if = "Option::is_none")]
    pub secret_version: Option<String>,
    #[doc = "Describes the action that shall be taken when the certificate is updated in Key Vault."]
    #[serde(rename = "updateRule")]
    pub update_rule: key_vault_certificate_source_parameters::UpdateRule,
    #[doc = "Describes the action that shall be taken when the certificate is removed from Key Vault."]
    #[serde(rename = "deleteRule")]
    pub delete_rule: key_vault_certificate_source_parameters::DeleteRule,
}
impl KeyVaultCertificateSourceParameters {
    pub fn new(
        odata_type: key_vault_certificate_source_parameters::OdataType,
        subscription_id: String,
        resource_group_name: String,
        vault_name: String,
        secret_name: String,
        update_rule: key_vault_certificate_source_parameters::UpdateRule,
        delete_rule: key_vault_certificate_source_parameters::DeleteRule,
    ) -> Self {
        Self {
            odata_type,
            subscription_id,
            resource_group_name,
            vault_name,
            secret_name,
            secret_version: None,
            update_rule,
            delete_rule,
        }
    }
}
pub mod key_vault_certificate_source_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.KeyVaultCertificateSourceParameters")]
        MicrosoftAzureCdnModelsKeyVaultCertificateSourceParameters,
    }
    #[doc = "Describes the action that shall be taken when the certificate is updated in Key Vault."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UpdateRule")]
    pub enum UpdateRule {
        NoAction,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UpdateRule {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UpdateRule {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UpdateRule {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NoAction => serializer.serialize_unit_variant("UpdateRule", 0u32, "NoAction"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Describes the action that shall be taken when the certificate is removed from Key Vault."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeleteRule")]
    pub enum DeleteRule {
        NoAction,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeleteRule {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeleteRule {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeleteRule {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NoAction => serializer.serialize_unit_variant("DeleteRule", 0u32, "NoAction"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Parameters required for content load."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadParameters {
    #[doc = "The path to the content to be loaded. Path should be a relative file URL of the origin."]
    #[serde(rename = "contentPaths")]
    pub content_paths: Vec<String>,
}
impl LoadParameters {
    pub fn new(content_paths: Vec<String>) -> Self {
        Self { content_paths }
    }
}
#[doc = "CDN REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.Cdn"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Profile, endpoint, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list CDN operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsListResult {
    #[doc = "List of CDN operations supported by the CDN resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies what scenario the customer wants this CDN endpoint to optimize, e.g. Download, Media services. With this information we can apply scenario driven optimization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OptimizationType")]
pub enum OptimizationType {
    GeneralWebDelivery,
    GeneralMediaStreaming,
    VideoOnDemandMediaStreaming,
    LargeFileDownload,
    DynamicSiteAcceleration,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OptimizationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OptimizationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OptimizationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::GeneralWebDelivery => serializer.serialize_unit_variant("OptimizationType", 0u32, "GeneralWebDelivery"),
            Self::GeneralMediaStreaming => serializer.serialize_unit_variant("OptimizationType", 1u32, "GeneralMediaStreaming"),
            Self::VideoOnDemandMediaStreaming => serializer.serialize_unit_variant("OptimizationType", 2u32, "VideoOnDemandMediaStreaming"),
            Self::LargeFileDownload => serializer.serialize_unit_variant("OptimizationType", 3u32, "LargeFileDownload"),
            Self::DynamicSiteAcceleration => serializer.serialize_unit_variant("OptimizationType", 4u32, "DynamicSiteAcceleration"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "CDN origin is the source of the content being delivered via CDN. When the edge nodes represented by an endpoint do not have the requested content cached, they attempt to fetch it from one or more of the configured origins."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Origin {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The JSON object that contains the properties of the origin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OriginProperties>,
}
impl Origin {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Origin group comprising of origins is used for load balancing to origins when the content cannot be served from CDN."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OriginGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The JSON object that contains the properties of the origin group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OriginGroupProperties>,
}
impl OriginGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list origin groups. It contains a list of origin groups objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OriginGroupListResult {
    #[doc = "List of CDN origin groups within an endpoint"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OriginGroup>,
    #[doc = "URL to get the next set of origin objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OriginGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OriginGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the Origin Group override action for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OriginGroupOverrideAction {
    #[serde(flatten)]
    pub delivery_rule_action: DeliveryRuleAction,
    #[doc = "Defines the parameters for the Origin Group override action."]
    pub parameters: OriginGroupOverrideActionParameters,
}
impl OriginGroupOverrideAction {
    pub fn new(delivery_rule_action: DeliveryRuleAction, parameters: OriginGroupOverrideActionParameters) -> Self {
        Self {
            delivery_rule_action,
            parameters,
        }
    }
}
#[doc = "Defines the parameters for the Origin Group override action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OriginGroupOverrideActionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: origin_group_override_action_parameters::OdataType,
    #[doc = "Reference to another resource."]
    #[serde(rename = "originGroup")]
    pub origin_group: ResourceReference,
}
impl OriginGroupOverrideActionParameters {
    pub fn new(odata_type: origin_group_override_action_parameters::OdataType, origin_group: ResourceReference) -> Self {
        Self { odata_type, origin_group }
    }
}
pub mod origin_group_override_action_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleOriginGroupOverrideActionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleOriginGroupOverrideActionParameters,
    }
}
#[doc = "The JSON object that contains the properties of the origin group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OriginGroupProperties {
    #[serde(flatten)]
    pub origin_group_update_properties_parameters: OriginGroupUpdatePropertiesParameters,
    #[doc = "Resource status of the origin group."]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<origin_group_properties::ResourceState>,
    #[doc = "Provisioning status of the origin group."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl OriginGroupProperties {
    pub fn new() -> Self {
        Self {
            origin_group_update_properties_parameters: OriginGroupUpdatePropertiesParameters::default(),
            resource_state: None,
            provisioning_state: None,
        }
    }
}
pub mod origin_group_properties {
    use super::*;
    #[doc = "Resource status of the origin group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceState")]
    pub enum ResourceState {
        Creating,
        Active,
        Deleting,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Creating => serializer.serialize_unit_variant("ResourceState", 0u32, "Creating"),
                Self::Active => serializer.serialize_unit_variant("ResourceState", 1u32, "Active"),
                Self::Deleting => serializer.serialize_unit_variant("ResourceState", 2u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Origin group properties needed for origin group creation or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OriginGroupUpdateParameters {
    #[doc = "The JSON object that contains the properties of the origin group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OriginGroupUpdatePropertiesParameters>,
}
impl OriginGroupUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties of the origin group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OriginGroupUpdatePropertiesParameters {
    #[doc = "The JSON object that contains the properties to send health probes to origin."]
    #[serde(rename = "healthProbeSettings", default, skip_serializing_if = "Option::is_none")]
    pub health_probe_settings: Option<HealthProbeParameters>,
    #[doc = "The source of the content being delivered via CDN within given origin group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origins: Vec<ResourceReference>,
    #[doc = "Time in minutes to shift the traffic to the endpoint gradually when an unhealthy endpoint comes healthy or a new endpoint is added. Default is 10 mins. This property is currently not supported."]
    #[serde(
        rename = "trafficRestorationTimeToHealedOrNewEndpointsInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_restoration_time_to_healed_or_new_endpoints_in_minutes: Option<i64>,
    #[doc = "The JSON object that contains the properties to determine origin health using real requests/responses."]
    #[serde(
        rename = "responseBasedOriginErrorDetectionSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_based_origin_error_detection_settings: Option<ResponseBasedOriginErrorDetectionParameters>,
}
impl OriginGroupUpdatePropertiesParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list origins. It contains a list of origin objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OriginListResult {
    #[doc = "List of CDN origins within an endpoint"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Origin>,
    #[doc = "URL to get the next set of origin objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OriginListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OriginListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties of the origin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OriginProperties {
    #[serde(flatten)]
    pub origin_update_properties_parameters: OriginUpdatePropertiesParameters,
    #[doc = "Resource status of the origin."]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<origin_properties::ResourceState>,
    #[doc = "Provisioning status of the origin."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl OriginProperties {
    pub fn new() -> Self {
        Self {
            origin_update_properties_parameters: OriginUpdatePropertiesParameters::default(),
            resource_state: None,
            provisioning_state: None,
        }
    }
}
pub mod origin_properties {
    use super::*;
    #[doc = "Resource status of the origin."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceState")]
    pub enum ResourceState {
        Creating,
        Active,
        Deleting,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Creating => serializer.serialize_unit_variant("ResourceState", 0u32, "Creating"),
                Self::Active => serializer.serialize_unit_variant("ResourceState", 1u32, "Active"),
                Self::Deleting => serializer.serialize_unit_variant("ResourceState", 2u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Origin properties needed for origin update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OriginUpdateParameters {
    #[doc = "The JSON object that contains the properties of the origin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OriginUpdatePropertiesParameters>,
}
impl OriginUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties of the origin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OriginUpdatePropertiesParameters {
    #[doc = "The address of the origin. Domain names, IPv4 addresses, and IPv6 addresses are supported.This should be unique across all origins in an endpoint."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "The value of the HTTP port. Must be between 1 and 65535."]
    #[serde(rename = "httpPort", default, skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,
    #[doc = "The value of the HTTPS port. Must be between 1 and 65535."]
    #[serde(rename = "httpsPort", default, skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,
    #[doc = "The host header value sent to the origin with each request. If you leave this blank, the request hostname determines this value. Azure CDN origins, such as Web Apps, Blob Storage, and Cloud Services require this host header value to match the origin hostname by default. If endpoint uses multiple origins for load balancing, then the host header at endpoint is ignored and this one is considered."]
    #[serde(rename = "originHostHeader", default, skip_serializing_if = "Option::is_none")]
    pub origin_host_header: Option<String>,
    #[doc = "Priority of origin in given origin group for load balancing. Higher priorities will not be used for load balancing if any lower priority origin is healthy.Must be between 1 and 5"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[doc = "Weight of the origin in given origin group for load balancing. Must be between 1 and 1000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
    #[doc = "Origin is enabled for load balancing or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl OriginUpdatePropertiesParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the parameters for PostArgs match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostArgsMatchConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: post_args_match_condition_parameters::OdataType,
    #[doc = "Name of PostArg to be matched"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[doc = "Describes operator to be matched"]
    pub operator: post_args_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(rename = "matchValues", default, skip_serializing_if = "Vec::is_empty")]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transforms: Vec<Transform>,
}
impl PostArgsMatchConditionParameters {
    pub fn new(
        odata_type: post_args_match_condition_parameters::OdataType,
        operator: post_args_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            odata_type,
            selector: None,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod post_args_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRulePostArgsConditionParameters")]
        MicrosoftAzureCdnModelsDeliveryRulePostArgsConditionParameters,
    }
    #[doc = "Describes operator to be matched"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Any,
        Equal,
        Contains,
        BeginsWith,
        EndsWith,
        LessThan,
        LessThanOrEqual,
        GreaterThan,
        GreaterThanOrEqual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Any => serializer.serialize_unit_variant("Operator", 0u32, "Any"),
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "CDN profile is a logical grouping of endpoints that share the same settings, such as CDN provider and pricing tier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The pricing tier (defines a CDN provider, feature list and rate) of the CDN profile."]
    pub sku: Sku,
    #[doc = "The JSON object that contains the properties required to create a profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProfileProperties>,
}
impl Profile {
    pub fn new(tracked_resource: TrackedResource, sku: Sku) -> Self {
        Self {
            tracked_resource,
            sku,
            properties: None,
        }
    }
}
#[doc = "Result of the request to list profiles. It contains a list of profile objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileListResult {
    #[doc = "List of CDN profiles within a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Profile>,
    #[doc = "URL to get the next set of profile objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProfileListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProfileListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties required to create a profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileProperties {
    #[doc = "Resource status of the profile."]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<profile_properties::ResourceState>,
    #[doc = "Provisioning status of the profile."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ProfileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod profile_properties {
    use super::*;
    #[doc = "Resource status of the profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceState")]
    pub enum ResourceState {
        Creating,
        Active,
        Deleting,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Creating => serializer.serialize_unit_variant("ResourceState", 0u32, "Creating"),
                Self::Active => serializer.serialize_unit_variant("ResourceState", 1u32, "Active"),
                Self::Deleting => serializer.serialize_unit_variant("ResourceState", 2u32, "Deleting"),
                Self::Disabled => serializer.serialize_unit_variant("ResourceState", 3u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties required to update a profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileUpdateParameters {
    #[doc = "Profile tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ProfileUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a ARM proxy resource. It will have everything other than required location and tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters required for content purge."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurgeParameters {
    #[doc = "The path to the content to be purged. Can describe a file path or a wild card directory."]
    #[serde(rename = "contentPaths")]
    pub content_paths: Vec<String>,
}
impl PurgeParameters {
    pub fn new(content_paths: Vec<String>) -> Self {
        Self { content_paths }
    }
}
#[doc = "Defines how CDN caches requests that include query strings. You can ignore any query strings when caching, bypass caching to prevent requests that contain query strings from being cached, or cache every request with a unique URL."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum QueryStringCachingBehavior {
    IgnoreQueryString,
    BypassCaching,
    UseQueryString,
    NotSet,
}
#[doc = "Defines the parameters for QueryString match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryStringMatchConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: query_string_match_condition_parameters::OdataType,
    #[doc = "Describes operator to be matched"]
    pub operator: query_string_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(rename = "matchValues", default, skip_serializing_if = "Vec::is_empty")]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transforms: Vec<Transform>,
}
impl QueryStringMatchConditionParameters {
    pub fn new(
        odata_type: query_string_match_condition_parameters::OdataType,
        operator: query_string_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            odata_type,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod query_string_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleQueryStringConditionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleQueryStringConditionParameters,
    }
    #[doc = "Describes operator to be matched"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Any,
        Equal,
        Contains,
        BeginsWith,
        EndsWith,
        LessThan,
        LessThanOrEqual,
        GreaterThan,
        GreaterThanOrEqual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Any => serializer.serialize_unit_variant("Operator", 0u32, "Any"),
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameters for RemoteAddress match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteAddressMatchConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: remote_address_match_condition_parameters::OdataType,
    #[doc = "Describes operator to be matched"]
    pub operator: remote_address_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "Match values to match against. The operator will apply to each value in here with OR semantics. If any of them match the variable with the given operator this match condition is considered a match."]
    #[serde(rename = "matchValues", default, skip_serializing_if = "Vec::is_empty")]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transforms: Vec<Transform>,
}
impl RemoteAddressMatchConditionParameters {
    pub fn new(
        odata_type: remote_address_match_condition_parameters::OdataType,
        operator: remote_address_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            odata_type,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod remote_address_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleRemoteAddressConditionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleRemoteAddressConditionParameters,
    }
    #[doc = "Describes operator to be matched"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Any,
        #[serde(rename = "IPMatch")]
        IpMatch,
        GeoMatch,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Any => serializer.serialize_unit_variant("Operator", 0u32, "Any"),
                Self::IpMatch => serializer.serialize_unit_variant("Operator", 1u32, "IPMatch"),
                Self::GeoMatch => serializer.serialize_unit_variant("Operator", 2u32, "GeoMatch"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameters for RequestBody match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestBodyMatchConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: request_body_match_condition_parameters::OdataType,
    #[doc = "Describes operator to be matched"]
    pub operator: request_body_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(rename = "matchValues", default, skip_serializing_if = "Vec::is_empty")]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transforms: Vec<Transform>,
}
impl RequestBodyMatchConditionParameters {
    pub fn new(
        odata_type: request_body_match_condition_parameters::OdataType,
        operator: request_body_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            odata_type,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod request_body_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleRequestBodyConditionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleRequestBodyConditionParameters,
    }
    #[doc = "Describes operator to be matched"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Any,
        Equal,
        Contains,
        BeginsWith,
        EndsWith,
        LessThan,
        LessThanOrEqual,
        GreaterThan,
        GreaterThanOrEqual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Any => serializer.serialize_unit_variant("Operator", 0u32, "Any"),
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameters for RequestHeader match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestHeaderMatchConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: request_header_match_condition_parameters::OdataType,
    #[doc = "Name of Header to be matched"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[doc = "Describes operator to be matched"]
    pub operator: request_header_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(rename = "matchValues", default, skip_serializing_if = "Vec::is_empty")]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transforms: Vec<Transform>,
}
impl RequestHeaderMatchConditionParameters {
    pub fn new(
        odata_type: request_header_match_condition_parameters::OdataType,
        operator: request_header_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            odata_type,
            selector: None,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod request_header_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleRequestHeaderConditionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleRequestHeaderConditionParameters,
    }
    #[doc = "Describes operator to be matched"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Any,
        Equal,
        Contains,
        BeginsWith,
        EndsWith,
        LessThan,
        LessThanOrEqual,
        GreaterThan,
        GreaterThanOrEqual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Any => serializer.serialize_unit_variant("Operator", 0u32, "Any"),
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameters for RequestMethod match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestMethodMatchConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: request_method_match_condition_parameters::OdataType,
    #[doc = "Describes operator to be matched"]
    pub operator: request_method_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(rename = "matchValues", default, skip_serializing_if = "Vec::is_empty")]
    pub match_values: Vec<String>,
}
impl RequestMethodMatchConditionParameters {
    pub fn new(
        odata_type: request_method_match_condition_parameters::OdataType,
        operator: request_method_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            odata_type,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
        }
    }
}
pub mod request_method_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleRequestMethodConditionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleRequestMethodConditionParameters,
    }
    #[doc = "Describes operator to be matched"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Equal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Equal => serializer.serialize_unit_variant("Operator", 0u32, "Equal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameters for RequestScheme match conditions "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestSchemeMatchConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: request_scheme_match_condition_parameters::OdataType,
    #[doc = "Describes operator to be matched"]
    pub operator: request_scheme_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(rename = "matchValues", default, skip_serializing_if = "Vec::is_empty")]
    pub match_values: Vec<String>,
}
impl RequestSchemeMatchConditionParameters {
    pub fn new(
        odata_type: request_scheme_match_condition_parameters::OdataType,
        operator: request_scheme_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            odata_type,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
        }
    }
}
pub mod request_scheme_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleRequestSchemeConditionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleRequestSchemeConditionParameters,
    }
    #[doc = "Describes operator to be matched"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        Equal,
    }
}
#[doc = "Defines the parameters for RequestUri match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestUriMatchConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: request_uri_match_condition_parameters::OdataType,
    #[doc = "Describes operator to be matched"]
    pub operator: request_uri_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(rename = "matchValues", default, skip_serializing_if = "Vec::is_empty")]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transforms: Vec<Transform>,
}
impl RequestUriMatchConditionParameters {
    pub fn new(
        odata_type: request_uri_match_condition_parameters::OdataType,
        operator: request_uri_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            odata_type,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod request_uri_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleRequestUriConditionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleRequestUriConditionParameters,
    }
    #[doc = "Describes operator to be matched"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Any,
        Equal,
        Contains,
        BeginsWith,
        EndsWith,
        LessThan,
        LessThanOrEqual,
        GreaterThan,
        GreaterThanOrEqual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Any => serializer.serialize_unit_variant("Operator", 0u32, "Any"),
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The core properties of ARM resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to another resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceReference {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of CDN resource used in CheckNameAvailability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    #[serde(rename = "Microsoft.Cdn/Profiles/Endpoints")]
    MicrosoftCdnProfilesEndpoints,
}
#[doc = "Output of check resource usage API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceUsage {
    #[doc = "Resource type for which the usage is provided."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Unit of the usage. e.g. Count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Actual value of usage on the specified resource type."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "Quota of the specified resource type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}
impl ResourceUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Output of check resource usage API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceUsageListResult {
    #[doc = "List of resource usages."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceUsage>,
    #[doc = "URL to get the next set of custom domain objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceUsageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceUsageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties to determine origin health using real requests/responses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseBasedOriginErrorDetectionParameters {
    #[doc = "Type of response errors for real user requests for which origin will be deemed unhealthy"]
    #[serde(rename = "responseBasedDetectedErrorTypes", default, skip_serializing_if = "Option::is_none")]
    pub response_based_detected_error_types: Option<response_based_origin_error_detection_parameters::ResponseBasedDetectedErrorTypes>,
    #[doc = "The percentage of failed requests in the sample where failover should trigger."]
    #[serde(
        rename = "responseBasedFailoverThresholdPercentage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_based_failover_threshold_percentage: Option<i64>,
    #[doc = "The list of Http status code ranges that are considered as server errors for origin and it is marked as unhealthy."]
    #[serde(rename = "httpErrorRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub http_error_ranges: Vec<HttpErrorRangeParameters>,
}
impl ResponseBasedOriginErrorDetectionParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod response_based_origin_error_detection_parameters {
    use super::*;
    #[doc = "Type of response errors for real user requests for which origin will be deemed unhealthy"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResponseBasedDetectedErrorTypes {
        None,
        TcpErrorsOnly,
        TcpAndHttpErrors,
    }
}
#[doc = "The pricing tier (defines a CDN provider, feature list and rate) of the CDN profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "Name of the pricing tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<sku::Name>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku {
    use super::*;
    #[doc = "Name of the pricing tier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        #[serde(rename = "Standard_Verizon")]
        StandardVerizon,
        #[serde(rename = "Premium_Verizon")]
        PremiumVerizon,
        #[serde(rename = "Custom_Verizon")]
        CustomVerizon,
        #[serde(rename = "Standard_Akamai")]
        StandardAkamai,
        #[serde(rename = "Standard_ChinaCdn")]
        StandardChinaCdn,
        #[serde(rename = "Standard_Microsoft")]
        StandardMicrosoft,
        #[serde(rename = "Premium_ChinaCdn")]
        PremiumChinaCdn,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardVerizon => serializer.serialize_unit_variant("Name", 0u32, "Standard_Verizon"),
                Self::PremiumVerizon => serializer.serialize_unit_variant("Name", 1u32, "Premium_Verizon"),
                Self::CustomVerizon => serializer.serialize_unit_variant("Name", 2u32, "Custom_Verizon"),
                Self::StandardAkamai => serializer.serialize_unit_variant("Name", 3u32, "Standard_Akamai"),
                Self::StandardChinaCdn => serializer.serialize_unit_variant("Name", 4u32, "Standard_ChinaCdn"),
                Self::StandardMicrosoft => serializer.serialize_unit_variant("Name", 5u32, "Standard_Microsoft"),
                Self::PremiumChinaCdn => serializer.serialize_unit_variant("Name", 6u32, "Premium_ChinaCdn"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The URI required to login to the supplemental portal from the Azure portal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SsoUri {
    #[doc = "The URI used to login to the supplemental portal."]
    #[serde(rename = "ssoUriValue", default, skip_serializing_if = "Option::is_none")]
    pub sso_uri_value: Option<String>,
}
impl SsoUri {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of the GetSupportedOptimizationTypes API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedOptimizationTypesListResult {
    #[doc = "Supported optimization types for a profile."]
    #[serde(rename = "supportedOptimizationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_optimization_types: Vec<OptimizationType>,
}
impl SupportedOptimizationTypesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a ARM tracked top level resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource location."]
    pub location: String,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            location,
            tags: None,
        }
    }
}
#[doc = "Defines the parameters for UrlFileExtension match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlFileExtensionMatchConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: url_file_extension_match_condition_parameters::OdataType,
    #[doc = "Describes operator to be matched"]
    pub operator: url_file_extension_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(rename = "matchValues", default, skip_serializing_if = "Vec::is_empty")]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transforms: Vec<Transform>,
}
impl UrlFileExtensionMatchConditionParameters {
    pub fn new(
        odata_type: url_file_extension_match_condition_parameters::OdataType,
        operator: url_file_extension_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            odata_type,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod url_file_extension_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleUrlFileExtensionMatchConditionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleUrlFileExtensionMatchConditionParameters,
    }
    #[doc = "Describes operator to be matched"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Any,
        Equal,
        Contains,
        BeginsWith,
        EndsWith,
        LessThan,
        LessThanOrEqual,
        GreaterThan,
        GreaterThanOrEqual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Any => serializer.serialize_unit_variant("Operator", 0u32, "Any"),
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameters for UrlFilename match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlFileNameMatchConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: url_file_name_match_condition_parameters::OdataType,
    #[doc = "Describes operator to be matched"]
    pub operator: url_file_name_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(rename = "matchValues", default, skip_serializing_if = "Vec::is_empty")]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transforms: Vec<Transform>,
}
impl UrlFileNameMatchConditionParameters {
    pub fn new(
        odata_type: url_file_name_match_condition_parameters::OdataType,
        operator: url_file_name_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            odata_type,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod url_file_name_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleUrlFilenameConditionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleUrlFilenameConditionParameters,
    }
    #[doc = "Describes operator to be matched"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Any,
        Equal,
        Contains,
        BeginsWith,
        EndsWith,
        LessThan,
        LessThanOrEqual,
        GreaterThan,
        GreaterThanOrEqual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Any => serializer.serialize_unit_variant("Operator", 0u32, "Any"),
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameters for UrlPath match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlPathMatchConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: url_path_match_condition_parameters::OdataType,
    #[doc = "Describes operator to be matched"]
    pub operator: url_path_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(rename = "matchValues", default, skip_serializing_if = "Vec::is_empty")]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transforms: Vec<Transform>,
}
impl UrlPathMatchConditionParameters {
    pub fn new(
        odata_type: url_path_match_condition_parameters::OdataType,
        operator: url_path_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            odata_type,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod url_path_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleUrlPathMatchConditionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleUrlPathMatchConditionParameters,
    }
    #[doc = "Describes operator to be matched"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Any,
        Equal,
        Contains,
        BeginsWith,
        EndsWith,
        LessThan,
        LessThanOrEqual,
        GreaterThan,
        GreaterThanOrEqual,
        Wildcard,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Any => serializer.serialize_unit_variant("Operator", 0u32, "Any"),
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::Wildcard => serializer.serialize_unit_variant("Operator", 9u32, "Wildcard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the url redirect action for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlRedirectAction {
    #[serde(flatten)]
    pub delivery_rule_action: DeliveryRuleAction,
    #[doc = "Defines the parameters for the url redirect action."]
    pub parameters: UrlRedirectActionParameters,
}
impl UrlRedirectAction {
    pub fn new(delivery_rule_action: DeliveryRuleAction, parameters: UrlRedirectActionParameters) -> Self {
        Self {
            delivery_rule_action,
            parameters,
        }
    }
}
#[doc = "Defines the parameters for the url redirect action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlRedirectActionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: url_redirect_action_parameters::OdataType,
    #[doc = "The redirect type the rule will use when redirecting traffic."]
    #[serde(rename = "redirectType")]
    pub redirect_type: url_redirect_action_parameters::RedirectType,
    #[doc = "Protocol to use for the redirect. The default value is MatchRequest"]
    #[serde(rename = "destinationProtocol", default, skip_serializing_if = "Option::is_none")]
    pub destination_protocol: Option<url_redirect_action_parameters::DestinationProtocol>,
    #[doc = "The full path to redirect. Path cannot be empty and must start with /. Leave empty to use the incoming path as destination path."]
    #[serde(rename = "customPath", default, skip_serializing_if = "Option::is_none")]
    pub custom_path: Option<String>,
    #[doc = "Host to redirect. Leave empty to use the incoming host as the destination host."]
    #[serde(rename = "customHostname", default, skip_serializing_if = "Option::is_none")]
    pub custom_hostname: Option<String>,
    #[doc = "The set of query strings to be placed in the redirect URL. Setting this value would replace any existing query string; leave empty to preserve the incoming query string. Query string must be in <key>=<value> format. ? and & will be added automatically so do not include them."]
    #[serde(rename = "customQueryString", default, skip_serializing_if = "Option::is_none")]
    pub custom_query_string: Option<String>,
    #[doc = "Fragment to add to the redirect URL. Fragment is the part of the URL that comes after #. Do not include the #."]
    #[serde(rename = "customFragment", default, skip_serializing_if = "Option::is_none")]
    pub custom_fragment: Option<String>,
}
impl UrlRedirectActionParameters {
    pub fn new(odata_type: url_redirect_action_parameters::OdataType, redirect_type: url_redirect_action_parameters::RedirectType) -> Self {
        Self {
            odata_type,
            redirect_type,
            destination_protocol: None,
            custom_path: None,
            custom_hostname: None,
            custom_query_string: None,
            custom_fragment: None,
        }
    }
}
pub mod url_redirect_action_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleUrlRedirectActionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleUrlRedirectActionParameters,
    }
    #[doc = "The redirect type the rule will use when redirecting traffic."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RedirectType")]
    pub enum RedirectType {
        Moved,
        Found,
        TemporaryRedirect,
        PermanentRedirect,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RedirectType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RedirectType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RedirectType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Moved => serializer.serialize_unit_variant("RedirectType", 0u32, "Moved"),
                Self::Found => serializer.serialize_unit_variant("RedirectType", 1u32, "Found"),
                Self::TemporaryRedirect => serializer.serialize_unit_variant("RedirectType", 2u32, "TemporaryRedirect"),
                Self::PermanentRedirect => serializer.serialize_unit_variant("RedirectType", 3u32, "PermanentRedirect"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Protocol to use for the redirect. The default value is MatchRequest"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DestinationProtocol")]
    pub enum DestinationProtocol {
        MatchRequest,
        Http,
        Https,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DestinationProtocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DestinationProtocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DestinationProtocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MatchRequest => serializer.serialize_unit_variant("DestinationProtocol", 0u32, "MatchRequest"),
                Self::Http => serializer.serialize_unit_variant("DestinationProtocol", 1u32, "Http"),
                Self::Https => serializer.serialize_unit_variant("DestinationProtocol", 2u32, "Https"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the url rewrite action for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlRewriteAction {
    #[serde(flatten)]
    pub delivery_rule_action: DeliveryRuleAction,
    #[doc = "Defines the parameters for the url rewrite action."]
    pub parameters: UrlRewriteActionParameters,
}
impl UrlRewriteAction {
    pub fn new(delivery_rule_action: DeliveryRuleAction, parameters: UrlRewriteActionParameters) -> Self {
        Self {
            delivery_rule_action,
            parameters,
        }
    }
}
#[doc = "Defines the parameters for the url rewrite action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlRewriteActionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: url_rewrite_action_parameters::OdataType,
    #[doc = "define a request URI pattern that identifies the type of requests that may be rewritten. If value is blank, all strings are matched."]
    #[serde(rename = "sourcePattern")]
    pub source_pattern: String,
    #[doc = "Define the relative URL to which the above requests will be rewritten by."]
    pub destination: String,
    #[doc = "Whether to preserve unmatched path. Default value is true."]
    #[serde(rename = "preserveUnmatchedPath", default, skip_serializing_if = "Option::is_none")]
    pub preserve_unmatched_path: Option<bool>,
}
impl UrlRewriteActionParameters {
    pub fn new(odata_type: url_rewrite_action_parameters::OdataType, source_pattern: String, destination: String) -> Self {
        Self {
            odata_type,
            source_pattern,
            destination,
            preserve_unmatched_path: None,
        }
    }
}
pub mod url_rewrite_action_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.DeliveryRuleUrlRewriteActionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleUrlRewriteActionParameters,
    }
}
#[doc = "Defines the certificate source parameters using user's keyvault certificate for enabling SSL."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserManagedHttpsParameters {
    #[serde(flatten)]
    pub custom_domain_https_parameters: CustomDomainHttpsParameters,
    #[doc = "Describes the parameters for using a user's KeyVault certificate for securing custom domain."]
    #[serde(rename = "certificateSourceParameters")]
    pub certificate_source_parameters: KeyVaultCertificateSourceParameters,
}
impl UserManagedHttpsParameters {
    pub fn new(
        custom_domain_https_parameters: CustomDomainHttpsParameters,
        certificate_source_parameters: KeyVaultCertificateSourceParameters,
    ) -> Self {
        Self {
            custom_domain_https_parameters,
            certificate_source_parameters,
        }
    }
}
#[doc = "Input of the custom domain to be validated for DNS mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateCustomDomainInput {
    #[doc = "The host name of the custom domain. Must be a domain name."]
    #[serde(rename = "hostName")]
    pub host_name: String,
}
impl ValidateCustomDomainInput {
    pub fn new(host_name: String) -> Self {
        Self { host_name }
    }
}
#[doc = "Output of custom domain validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateCustomDomainOutput {
    #[doc = "Indicates whether the custom domain is valid or not."]
    #[serde(rename = "customDomainValidated", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain_validated: Option<bool>,
    #[doc = "The reason why the custom domain is not valid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Error message describing why the custom domain is not valid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ValidateCustomDomainOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input of the validate probe API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateProbeInput {
    #[doc = "The probe URL to validate."]
    #[serde(rename = "probeURL")]
    pub probe_url: String,
}
impl ValidateProbeInput {
    pub fn new(probe_url: String) -> Self {
        Self { probe_url }
    }
}
#[doc = "Output of the validate probe API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateProbeOutput {
    #[doc = "Indicates whether the probe URL is accepted or not."]
    #[serde(rename = "isValid", default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[doc = "Specifies the error code when the probe url is not accepted."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "The detailed error message describing why the probe URL is not accepted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ValidateProbeOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CIDR Ip address"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CidrIpAddress {
    #[doc = "Ip address itself."]
    #[serde(rename = "baseIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub base_ip_address: Option<String>,
    #[doc = "The length of the prefix of the ip address."]
    #[serde(rename = "prefixLength", default, skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i64>,
}
impl CidrIpAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes what transforms are applied before matching"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Transform")]
pub enum Transform {
    Lowercase,
    Uppercase,
    Trim,
    UrlDecode,
    UrlEncode,
    RemoveNulls,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Transform {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Transform {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Transform {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Lowercase => serializer.serialize_unit_variant("Transform", 0u32, "Lowercase"),
            Self::Uppercase => serializer.serialize_unit_variant("Transform", 1u32, "Uppercase"),
            Self::Trim => serializer.serialize_unit_variant("Transform", 2u32, "Trim"),
            Self::UrlDecode => serializer.serialize_unit_variant("Transform", 3u32, "UrlDecode"),
            Self::UrlEncode => serializer.serialize_unit_variant("Transform", 4u32, "UrlEncode"),
            Self::RemoveNulls => serializer.serialize_unit_variant("Transform", 5u32, "RemoveNulls"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}

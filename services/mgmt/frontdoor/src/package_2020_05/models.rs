#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Defines the action to take on rule match."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ActionType")]
pub enum ActionType {
    Allow,
    Block,
    Log,
    Redirect,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ActionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ActionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ActionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Allow => serializer.serialize_unit_variant("ActionType", 0u32, "Allow"),
            Self::Block => serializer.serialize_unit_variant("ActionType", 1u32, "Block"),
            Self::Log => serializer.serialize_unit_variant("ActionType", 2u32, "Log"),
            Self::Redirect => serializer.serialize_unit_variant("ActionType", 3u32, "Redirect"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response body contains the status of the specified asynchronous operation, indicating whether it has succeeded, is in progress, or has failed. Note that this status is distinct from the HTTP status code returned for the Get Operation Status operation itself. If the asynchronous operation succeeded, the response body includes the HTTP status code for the successful request. If the asynchronous operation failed, the response body includes the HTTP status code for the failed request and error information regarding the failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureAsyncOperationResult {
    #[doc = "Status of the Azure async operation. Possible values are: 'InProgress', 'Succeeded', and 'Failed'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<azure_async_operation_result::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
impl AzureAsyncOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod azure_async_operation_result {
    use super::*;
    #[doc = "Status of the Azure async operation. Possible values are: 'InProgress', 'Succeeded', and 'Failed'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        InProgress,
        Succeeded,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::InProgress => serializer.serialize_unit_variant("Status", 0u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Backend address of a frontDoor load balancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Backend {
    #[doc = "Location of the backend (IP address or FQDN)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "The Alias of the Private Link resource. Populating this optional field indicates that this backend is 'Private'"]
    #[serde(rename = "privateLinkAlias", default, skip_serializing_if = "Option::is_none")]
    pub private_link_alias: Option<String>,
    #[doc = "The Resource Id of the Private Link resource. Populating this optional field indicates that this backend is 'Private'"]
    #[serde(rename = "privateLinkResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_resource_id: Option<String>,
    #[doc = "The location of the Private Link resource. Required only if 'privateLinkResourceId' is populated"]
    #[serde(rename = "privateLinkLocation", default, skip_serializing_if = "Option::is_none")]
    pub private_link_location: Option<String>,
    #[doc = "The Approval status for the connection to the Private Link"]
    #[serde(rename = "privateEndpointStatus", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint_status: Option<backend::PrivateEndpointStatus>,
    #[doc = "A custom message to be included in the approval request to connect to the Private Link"]
    #[serde(rename = "privateLinkApprovalMessage", default, skip_serializing_if = "Option::is_none")]
    pub private_link_approval_message: Option<String>,
    #[doc = "The HTTP TCP port number. Must be between 1 and 65535."]
    #[serde(rename = "httpPort", default, skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,
    #[doc = "The HTTPS TCP port number. Must be between 1 and 65535."]
    #[serde(rename = "httpsPort", default, skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,
    #[doc = "Whether to enable use of this backend. Permitted values are 'Enabled' or 'Disabled'"]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<backend::EnabledState>,
    #[doc = "Priority to use for load balancing. Higher priorities will not be used for load balancing if any lower priority backend is healthy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[doc = "Weight of this endpoint for load balancing purposes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
    #[doc = "The value to use as the host header sent to the backend. If blank or unspecified, this defaults to the incoming host."]
    #[serde(rename = "backendHostHeader", default, skip_serializing_if = "Option::is_none")]
    pub backend_host_header: Option<String>,
}
impl Backend {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backend {
    use super::*;
    #[doc = "The Approval status for the connection to the Private Link"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrivateEndpointStatus")]
    pub enum PrivateEndpointStatus {
        Pending,
        Approved,
        Rejected,
        Disconnected,
        Timeout,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrivateEndpointStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrivateEndpointStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrivateEndpointStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pending => serializer.serialize_unit_variant("PrivateEndpointStatus", 0u32, "Pending"),
                Self::Approved => serializer.serialize_unit_variant("PrivateEndpointStatus", 1u32, "Approved"),
                Self::Rejected => serializer.serialize_unit_variant("PrivateEndpointStatus", 2u32, "Rejected"),
                Self::Disconnected => serializer.serialize_unit_variant("PrivateEndpointStatus", 3u32, "Disconnected"),
                Self::Timeout => serializer.serialize_unit_variant("PrivateEndpointStatus", 4u32, "Timeout"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether to enable use of this backend. Permitted values are 'Enabled' or 'Disabled'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnabledState")]
    pub enum EnabledState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnabledState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnabledState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnabledState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("EnabledState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("EnabledState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A backend pool is a collection of backends that can be routed to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendPool {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The JSON object that contains the properties required to create a Backend Pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackendPoolProperties>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl BackendPool {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list Backend Pools. It contains a list of Backend Pools objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendPoolListResult {
    #[doc = "List of Backend Pools within a Front Door."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BackendPool>,
    #[doc = "URL to get the next set of BackendPool objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl BackendPoolListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties required to create a Backend Pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendPoolProperties {
    #[serde(flatten)]
    pub backend_pool_update_parameters: BackendPoolUpdateParameters,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
}
impl BackendPoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of backends that can be routed to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendPoolUpdateParameters {
    #[doc = "The set of backends for this pool"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub backends: Vec<Backend>,
    #[doc = "Reference to another subresource."]
    #[serde(rename = "loadBalancingSettings", default, skip_serializing_if = "Option::is_none")]
    pub load_balancing_settings: Option<SubResource>,
    #[doc = "Reference to another subresource."]
    #[serde(rename = "healthProbeSettings", default, skip_serializing_if = "Option::is_none")]
    pub health_probe_settings: Option<SubResource>,
}
impl BackendPoolUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Settings that apply to all backend pools."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendPoolsSettings {
    #[doc = "Whether to enforce certificate name check on HTTPS requests to all backend pools. No effect on non-HTTPS requests."]
    #[serde(rename = "enforceCertificateNameCheck", default, skip_serializing_if = "Option::is_none")]
    pub enforce_certificate_name_check: Option<backend_pools_settings::EnforceCertificateNameCheck>,
    #[doc = "Send and receive timeout on forwarding request to the backend. When timeout is reached, the request fails and returns."]
    #[serde(rename = "sendRecvTimeoutSeconds", default, skip_serializing_if = "Option::is_none")]
    pub send_recv_timeout_seconds: Option<i64>,
}
impl BackendPoolsSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backend_pools_settings {
    use super::*;
    #[doc = "Whether to enforce certificate name check on HTTPS requests to all backend pools. No effect on non-HTTPS requests."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnforceCertificateNameCheck")]
    pub enum EnforceCertificateNameCheck {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnforceCertificateNameCheck {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnforceCertificateNameCheck {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnforceCertificateNameCheck {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("EnforceCertificateNameCheck", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("EnforceCertificateNameCheck", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for EnforceCertificateNameCheck {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "Caching settings for a caching-type route. To disable caching, do not provide a cacheConfiguration object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheConfiguration {
    #[doc = "Treatment of URL query terms when forming the cache key."]
    #[serde(rename = "queryParameterStripDirective", default, skip_serializing_if = "Option::is_none")]
    pub query_parameter_strip_directive: Option<cache_configuration::QueryParameterStripDirective>,
    #[doc = "query parameters to include or exclude (comma separated)."]
    #[serde(rename = "queryParameters", default, skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<String>,
    #[doc = "Whether to use dynamic compression for cached content"]
    #[serde(rename = "dynamicCompression", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_compression: Option<cache_configuration::DynamicCompression>,
    #[doc = "The duration for which the content needs to be cached. Allowed format is in ISO 8601 format (http://en.wikipedia.org/wiki/ISO_8601#Durations). HTTP requires the value to be no more than a year"]
    #[serde(rename = "cacheDuration", default, skip_serializing_if = "Option::is_none")]
    pub cache_duration: Option<String>,
}
impl CacheConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cache_configuration {
    use super::*;
    #[doc = "Treatment of URL query terms when forming the cache key."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "QueryParameterStripDirective")]
    pub enum QueryParameterStripDirective {
        StripNone,
        StripAll,
        StripOnly,
        StripAllExcept,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for QueryParameterStripDirective {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for QueryParameterStripDirective {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for QueryParameterStripDirective {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StripNone => serializer.serialize_unit_variant("QueryParameterStripDirective", 0u32, "StripNone"),
                Self::StripAll => serializer.serialize_unit_variant("QueryParameterStripDirective", 1u32, "StripAll"),
                Self::StripOnly => serializer.serialize_unit_variant("QueryParameterStripDirective", 2u32, "StripOnly"),
                Self::StripAllExcept => serializer.serialize_unit_variant("QueryParameterStripDirective", 3u32, "StripAllExcept"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether to use dynamic compression for cached content"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DynamicCompression")]
    pub enum DynamicCompression {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DynamicCompression {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DynamicCompression {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DynamicCompression {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("DynamicCompression", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("DynamicCompression", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Input of CheckNameAvailability API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityInput {
    #[doc = "The resource name to validate."]
    pub name: String,
    #[doc = "Type of Front Door resource used in CheckNameAvailability."]
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
    #[serde(rename = "nameAvailability", default, skip_serializing_if = "Option::is_none")]
    pub name_availability: Option<check_name_availability_output::NameAvailability>,
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
pub mod check_name_availability_output {
    use super::*;
    #[doc = "Indicates whether the name is available."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NameAvailability")]
    pub enum NameAvailability {
        Available,
        Unavailable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NameAvailability {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NameAvailability {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NameAvailability {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Available => serializer.serialize_unit_variant("NameAvailability", 0u32, "Available"),
                Self::Unavailable => serializer.serialize_unit_variant("NameAvailability", 1u32, "Unavailable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Https settings for a domain"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomHttpsConfiguration {
    #[doc = "Defines the source of the SSL certificate"]
    #[serde(rename = "certificateSource")]
    pub certificate_source: custom_https_configuration::CertificateSource,
    #[doc = "Defines the TLS extension protocol that is used for secure delivery"]
    #[serde(rename = "protocolType")]
    pub protocol_type: custom_https_configuration::ProtocolType,
    #[doc = "The minimum TLS version required from the clients to establish an SSL handshake with Front Door."]
    #[serde(rename = "minimumTlsVersion")]
    pub minimum_tls_version: custom_https_configuration::MinimumTlsVersion,
    #[doc = "Parameters required for bring-your-own-certification via Key Vault"]
    #[serde(rename = "keyVaultCertificateSourceParameters", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_certificate_source_parameters: Option<KeyVaultCertificateSourceParameters>,
    #[doc = "Parameters required for enabling SSL with Front Door-managed certificates"]
    #[serde(rename = "frontDoorCertificateSourceParameters", default, skip_serializing_if = "Option::is_none")]
    pub front_door_certificate_source_parameters: Option<FrontDoorCertificateSourceParameters>,
}
impl CustomHttpsConfiguration {
    pub fn new(
        certificate_source: custom_https_configuration::CertificateSource,
        protocol_type: custom_https_configuration::ProtocolType,
        minimum_tls_version: custom_https_configuration::MinimumTlsVersion,
    ) -> Self {
        Self {
            certificate_source,
            protocol_type,
            minimum_tls_version,
            key_vault_certificate_source_parameters: None,
            front_door_certificate_source_parameters: None,
        }
    }
}
pub mod custom_https_configuration {
    use super::*;
    #[doc = "Defines the source of the SSL certificate"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CertificateSource")]
    pub enum CertificateSource {
        AzureKeyVault,
        FrontDoor,
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
                Self::FrontDoor => serializer.serialize_unit_variant("CertificateSource", 1u32, "FrontDoor"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Defines the TLS extension protocol that is used for secure delivery"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProtocolType")]
    pub enum ProtocolType {
        ServerNameIndication,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The minimum TLS version required from the clients to establish an SSL handshake with Front Door."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MinimumTlsVersion")]
    pub enum MinimumTlsVersion {
        #[serde(rename = "1.0")]
        N1_0,
        #[serde(rename = "1.2")]
        N1_2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MinimumTlsVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MinimumTlsVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MinimumTlsVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N1_0 => serializer.serialize_unit_variant("MinimumTlsVersion", 0u32, "1.0"),
                Self::N1_2 => serializer.serialize_unit_variant("MinimumTlsVersion", 1u32, "1.2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines contents of a web application rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomRule {
    #[doc = "Describes the name of the rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes priority of the rule. Rules with a lower value will be evaluated before rules with a higher value."]
    pub priority: i64,
    #[doc = "Describes if the custom rule is in enabled or disabled state. Defaults to Enabled if not specified."]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<custom_rule::EnabledState>,
    #[doc = "Describes type of rule."]
    #[serde(rename = "ruleType")]
    pub rule_type: custom_rule::RuleType,
    #[doc = "Time window for resetting the rate limit count. Default is 1 minute."]
    #[serde(rename = "rateLimitDurationInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub rate_limit_duration_in_minutes: Option<i64>,
    #[doc = "Number of allowed requests per client within the time window."]
    #[serde(rename = "rateLimitThreshold", default, skip_serializing_if = "Option::is_none")]
    pub rate_limit_threshold: Option<i64>,
    #[doc = "List of match conditions."]
    #[serde(rename = "matchConditions")]
    pub match_conditions: Vec<MatchCondition>,
    #[doc = "Defines the action to take on rule match."]
    pub action: ActionType,
}
impl CustomRule {
    pub fn new(priority: i64, rule_type: custom_rule::RuleType, match_conditions: Vec<MatchCondition>, action: ActionType) -> Self {
        Self {
            name: None,
            priority,
            enabled_state: None,
            rule_type,
            rate_limit_duration_in_minutes: None,
            rate_limit_threshold: None,
            match_conditions,
            action,
        }
    }
}
pub mod custom_rule {
    use super::*;
    #[doc = "Describes if the custom rule is in enabled or disabled state. Defaults to Enabled if not specified."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnabledState")]
    pub enum EnabledState {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnabledState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnabledState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnabledState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("EnabledState", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("EnabledState", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Describes type of rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RuleType")]
    pub enum RuleType {
        MatchRule,
        RateLimitRule,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RuleType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RuleType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RuleType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MatchRule => serializer.serialize_unit_variant("RuleType", 0u32, "MatchRule"),
                Self::RateLimitRule => serializer.serialize_unit_variant("RuleType", 1u32, "RateLimitRule"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines contents of custom rules"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomRuleList {
    #[doc = "List of rules"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<CustomRule>,
}
impl CustomRuleList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the endpoint properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Endpoint {
    #[doc = "The name of the endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The endpoint URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl Endpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetails>,
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<String>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates Front Door service is not able to process the incoming request. The reason is provided in the error message."]
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
#[doc = "Defines the properties of an Experiment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Experiment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The name of the Experiment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Defines the properties of an experiment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExperimentProperties>,
}
impl Experiment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a list of Experiments. It contains a list of Experiment objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExperimentList {
    #[doc = "List of Experiments within a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Experiment>,
    #[doc = "URL to get the next set of Experiment objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExperimentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ExperimentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the properties of an experiment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExperimentProperties {
    #[doc = "The description of the details or intents of the Experiment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Defines the endpoint properties"]
    #[serde(rename = "endpointA", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_a: Option<Endpoint>,
    #[doc = "Defines the endpoint properties"]
    #[serde(rename = "endpointB", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_b: Option<Endpoint>,
    #[doc = "The state of the Experiment"]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<experiment_properties::EnabledState>,
    #[doc = "Defines the server side resource status"]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<NetworkExperimentResourceState>,
    #[doc = "The description of Experiment status from the server side"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The uri to the Script used in the Experiment"]
    #[serde(rename = "scriptFileUri", default, skip_serializing_if = "Option::is_none")]
    pub script_file_uri: Option<String>,
}
impl ExperimentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod experiment_properties {
    use super::*;
    #[doc = "The state of the Experiment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnabledState")]
    pub enum EnabledState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnabledState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnabledState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnabledState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("EnabledState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("EnabledState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines modifiable attributes of an Experiment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExperimentUpdateModel {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Defines the properties of an experiment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExperimentUpdateProperties>,
}
impl ExperimentUpdateModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the properties of an experiment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExperimentUpdateProperties {
    #[doc = "The description of the intent or details of the Experiment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The state of the Experiment"]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<experiment_update_properties::EnabledState>,
}
impl ExperimentUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod experiment_update_properties {
    use super::*;
    #[doc = "The state of the Experiment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnabledState")]
    pub enum EnabledState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnabledState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnabledState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnabledState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("EnabledState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("EnabledState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes Forwarding Route."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForwardingConfiguration {
    #[serde(flatten)]
    pub route_configuration: RouteConfiguration,
    #[doc = "A custom path used to rewrite resource paths matched by this rule. Leave empty to use incoming path."]
    #[serde(rename = "customForwardingPath", default, skip_serializing_if = "Option::is_none")]
    pub custom_forwarding_path: Option<String>,
    #[doc = "Protocol this rule will use when forwarding traffic to backends."]
    #[serde(rename = "forwardingProtocol", default, skip_serializing_if = "Option::is_none")]
    pub forwarding_protocol: Option<forwarding_configuration::ForwardingProtocol>,
    #[doc = "Caching settings for a caching-type route. To disable caching, do not provide a cacheConfiguration object."]
    #[serde(rename = "cacheConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cache_configuration: Option<CacheConfiguration>,
    #[doc = "Reference to another subresource."]
    #[serde(rename = "backendPool", default, skip_serializing_if = "Option::is_none")]
    pub backend_pool: Option<SubResource>,
}
impl ForwardingConfiguration {
    pub fn new(route_configuration: RouteConfiguration) -> Self {
        Self {
            route_configuration,
            custom_forwarding_path: None,
            forwarding_protocol: None,
            cache_configuration: None,
            backend_pool: None,
        }
    }
}
pub mod forwarding_configuration {
    use super::*;
    #[doc = "Protocol this rule will use when forwarding traffic to backends."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ForwardingProtocol")]
    pub enum ForwardingProtocol {
        HttpOnly,
        HttpsOnly,
        MatchRequest,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ForwardingProtocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ForwardingProtocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ForwardingProtocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::HttpOnly => serializer.serialize_unit_variant("ForwardingProtocol", 0u32, "HttpOnly"),
                Self::HttpsOnly => serializer.serialize_unit_variant("ForwardingProtocol", 1u32, "HttpsOnly"),
                Self::MatchRequest => serializer.serialize_unit_variant("ForwardingProtocol", 2u32, "MatchRequest"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Front Door represents a collection of backend endpoints to route traffic to along with rules that specify how traffic is sent there."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontDoor {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The JSON object that contains the properties required to create an endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FrontDoorProperties>,
}
impl FrontDoor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters required for enabling SSL with Front Door-managed certificates"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontDoorCertificateSourceParameters {
    #[doc = "Defines the type of the certificate used for secure connections to a frontendEndpoint"]
    #[serde(rename = "certificateType", default, skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<front_door_certificate_source_parameters::CertificateType>,
}
impl FrontDoorCertificateSourceParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod front_door_certificate_source_parameters {
    use super::*;
    #[doc = "Defines the type of the certificate used for secure connections to a frontendEndpoint"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CertificateType")]
    pub enum CertificateType {
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
                Self::Dedicated => serializer.serialize_unit_variant("CertificateType", 0u32, "Dedicated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of the request to list Front Doors. It contains a list of Front Door objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontDoorListResult {
    #[doc = "List of Front Doors within a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FrontDoor>,
    #[doc = "URL to get the next set of Front Door objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FrontDoorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FrontDoorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties required to create an endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontDoorProperties {
    #[serde(flatten)]
    pub front_door_update_parameters: FrontDoorUpdateParameters,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
    #[doc = "Provisioning state of the Front Door."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The host that each frontendEndpoint must CNAME to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    #[doc = "The Id of the frontdoor."]
    #[serde(rename = "frontdoorId", default, skip_serializing_if = "Option::is_none")]
    pub frontdoor_id: Option<String>,
    #[doc = "Rules Engine Configurations available to routing rules."]
    #[serde(rename = "rulesEngines", default, skip_serializing_if = "Vec::is_empty")]
    pub rules_engines: Vec<RulesEngine>,
}
impl FrontDoorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties needed to update a Front Door"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontDoorUpdateParameters {
    #[doc = "A friendly name for the frontDoor"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Routing rules associated with this Front Door."]
    #[serde(rename = "routingRules", default, skip_serializing_if = "Vec::is_empty")]
    pub routing_rules: Vec<RoutingRule>,
    #[doc = "Load balancing settings associated with this Front Door instance."]
    #[serde(rename = "loadBalancingSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancing_settings: Vec<LoadBalancingSettingsModel>,
    #[doc = "Health probe settings associated with this Front Door instance."]
    #[serde(rename = "healthProbeSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub health_probe_settings: Vec<HealthProbeSettingsModel>,
    #[doc = "Backend pools available to routing rules."]
    #[serde(rename = "backendPools", default, skip_serializing_if = "Vec::is_empty")]
    pub backend_pools: Vec<BackendPool>,
    #[doc = "Frontend endpoints available to routing rules."]
    #[serde(rename = "frontendEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub frontend_endpoints: Vec<FrontendEndpoint>,
    #[doc = "Settings that apply to all backend pools."]
    #[serde(rename = "backendPoolsSettings", default, skip_serializing_if = "Option::is_none")]
    pub backend_pools_settings: Option<BackendPoolsSettings>,
    #[doc = "Operational status of the Front Door load balancer. Permitted values are 'Enabled' or 'Disabled'"]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<front_door_update_parameters::EnabledState>,
}
impl FrontDoorUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod front_door_update_parameters {
    use super::*;
    #[doc = "Operational status of the Front Door load balancer. Permitted values are 'Enabled' or 'Disabled'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnabledState")]
    pub enum EnabledState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnabledState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnabledState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnabledState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("EnabledState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("EnabledState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A frontend endpoint used for routing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontendEndpoint {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The JSON object that contains the properties required to create a frontend endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FrontendEndpointProperties>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl FrontendEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the Resource ID for a Frontend Endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontendEndpointLink {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl FrontendEndpointLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties required to create a frontend endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontendEndpointProperties {
    #[serde(flatten)]
    pub frontend_endpoint_update_parameters: FrontendEndpointUpdateParameters,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
    #[doc = "Provisioning status of Custom Https of the frontendEndpoint."]
    #[serde(rename = "customHttpsProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub custom_https_provisioning_state: Option<frontend_endpoint_properties::CustomHttpsProvisioningState>,
    #[doc = "Provisioning substate shows the progress of custom HTTPS enabling/disabling process step by step."]
    #[serde(rename = "customHttpsProvisioningSubstate", default, skip_serializing_if = "Option::is_none")]
    pub custom_https_provisioning_substate: Option<frontend_endpoint_properties::CustomHttpsProvisioningSubstate>,
    #[doc = "Https settings for a domain"]
    #[serde(rename = "customHttpsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub custom_https_configuration: Option<CustomHttpsConfiguration>,
}
impl FrontendEndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod frontend_endpoint_properties {
    use super::*;
    #[doc = "Provisioning status of Custom Https of the frontendEndpoint."]
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
#[doc = "Frontend endpoint used in routing rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontendEndpointUpdateParameters {
    #[doc = "The host name of the frontendEndpoint. Must be a domain name."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "Whether to allow session affinity on this host. Valid options are 'Enabled' or 'Disabled'"]
    #[serde(rename = "sessionAffinityEnabledState", default, skip_serializing_if = "Option::is_none")]
    pub session_affinity_enabled_state: Option<frontend_endpoint_update_parameters::SessionAffinityEnabledState>,
    #[doc = "UNUSED. This field will be ignored. The TTL to use in seconds for session affinity, if applicable."]
    #[serde(rename = "sessionAffinityTtlSeconds", default, skip_serializing_if = "Option::is_none")]
    pub session_affinity_ttl_seconds: Option<i64>,
    #[doc = "Defines the Web Application Firewall policy for each host (if applicable)"]
    #[serde(rename = "webApplicationFirewallPolicyLink", default, skip_serializing_if = "Option::is_none")]
    pub web_application_firewall_policy_link: Option<frontend_endpoint_update_parameters::WebApplicationFirewallPolicyLink>,
}
impl FrontendEndpointUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod frontend_endpoint_update_parameters {
    use super::*;
    #[doc = "Whether to allow session affinity on this host. Valid options are 'Enabled' or 'Disabled'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SessionAffinityEnabledState")]
    pub enum SessionAffinityEnabledState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SessionAffinityEnabledState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SessionAffinityEnabledState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SessionAffinityEnabledState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("SessionAffinityEnabledState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("SessionAffinityEnabledState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Defines the Web Application Firewall policy for each host (if applicable)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct WebApplicationFirewallPolicyLink {
        #[doc = "Resource ID."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    impl WebApplicationFirewallPolicyLink {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list frontend endpoints. It contains a list of Frontend endpoint objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontendEndpointsListResult {
    #[doc = "List of Frontend endpoints within a Front Door."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FrontendEndpoint>,
    #[doc = "URL to get the next set of frontend endpoints if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FrontendEndpointsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FrontendEndpointsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An action that can manipulate an http header."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeaderAction {
    #[doc = "Which type of manipulation to apply to the header."]
    #[serde(rename = "headerActionType")]
    pub header_action_type: header_action::HeaderActionType,
    #[doc = "The name of the header this action will apply to."]
    #[serde(rename = "headerName")]
    pub header_name: String,
    #[doc = "The value to update the given header name with. This value is not used if the actionType is Delete."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl HeaderAction {
    pub fn new(header_action_type: header_action::HeaderActionType, header_name: String) -> Self {
        Self {
            header_action_type,
            header_name,
            value: None,
        }
    }
}
pub mod header_action {
    use super::*;
    #[doc = "Which type of manipulation to apply to the header."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HeaderActionType")]
    pub enum HeaderActionType {
        Append,
        Delete,
        Overwrite,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HeaderActionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HeaderActionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HeaderActionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Append => serializer.serialize_unit_variant("HeaderActionType", 0u32, "Append"),
                Self::Delete => serializer.serialize_unit_variant("HeaderActionType", 1u32, "Delete"),
                Self::Overwrite => serializer.serialize_unit_variant("HeaderActionType", 2u32, "Overwrite"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of the request to list HealthProbeSettings. It contains a list of HealthProbeSettings objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthProbeSettingsListResult {
    #[doc = "List of HealthProbeSettings within a Front Door."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HealthProbeSettingsModel>,
    #[doc = "URL to get the next set of HealthProbeSettings objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl HealthProbeSettingsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Load balancing settings for a backend pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthProbeSettingsModel {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The JSON object that contains the properties required to create a health probe settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HealthProbeSettingsProperties>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl HealthProbeSettingsModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties required to create a health probe settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthProbeSettingsProperties {
    #[serde(flatten)]
    pub health_probe_settings_update_parameters: HealthProbeSettingsUpdateParameters,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
}
impl HealthProbeSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "L7 health probe settings for a backend pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthProbeSettingsUpdateParameters {
    #[doc = "The path to use for the health probe. Default is /"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Protocol scheme to use for this probe"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<health_probe_settings_update_parameters::Protocol>,
    #[doc = "The number of seconds between health probes."]
    #[serde(rename = "intervalInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i64>,
    #[doc = "Configures which HTTP method to use to probe the backends defined under backendPools."]
    #[serde(rename = "healthProbeMethod", default, skip_serializing_if = "Option::is_none")]
    pub health_probe_method: Option<health_probe_settings_update_parameters::HealthProbeMethod>,
    #[doc = "Whether to enable health probes to be made against backends defined under backendPools. Health probes can only be disabled if there is a single enabled backend in single enabled backend pool."]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<health_probe_settings_update_parameters::EnabledState>,
}
impl HealthProbeSettingsUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod health_probe_settings_update_parameters {
    use super::*;
    #[doc = "Protocol scheme to use for this probe"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        Http,
        Https,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Protocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Protocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Protocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Http => serializer.serialize_unit_variant("Protocol", 0u32, "Http"),
                Self::Https => serializer.serialize_unit_variant("Protocol", 1u32, "Https"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Configures which HTTP method to use to probe the backends defined under backendPools."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HealthProbeMethod")]
    pub enum HealthProbeMethod {
        #[serde(rename = "GET")]
        Get,
        #[serde(rename = "HEAD")]
        Head,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HealthProbeMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HealthProbeMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HealthProbeMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Get => serializer.serialize_unit_variant("HealthProbeMethod", 0u32, "GET"),
                Self::Head => serializer.serialize_unit_variant("HealthProbeMethod", 1u32, "HEAD"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for HealthProbeMethod {
        fn default() -> Self {
            Self::Head
        }
    }
    #[doc = "Whether to enable health probes to be made against backends defined under backendPools. Health probes can only be disabled if there is a single enabled backend in single enabled backend pool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnabledState")]
    pub enum EnabledState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnabledState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnabledState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnabledState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("EnabledState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("EnabledState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Parameters required for bring-your-own-certification via Key Vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultCertificateSourceParameters {
    #[doc = "The Key Vault containing the SSL certificate"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vault: Option<key_vault_certificate_source_parameters::Vault>,
    #[doc = "The name of the Key Vault secret representing the full certificate PFX"]
    #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    #[doc = "The version of the Key Vault secret representing the full certificate PFX"]
    #[serde(rename = "secretVersion", default, skip_serializing_if = "Option::is_none")]
    pub secret_version: Option<String>,
}
impl KeyVaultCertificateSourceParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod key_vault_certificate_source_parameters {
    use super::*;
    #[doc = "The Key Vault containing the SSL certificate"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Vault {
        #[doc = "Resource ID."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    impl Vault {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Defines the properties of a latency metric used in the latency scorecard"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LatencyMetric {
    #[doc = "The name of the Latency Metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The end time of the Latency Scorecard in UTC"]
    #[serde(rename = "endDateTimeUTC", default, skip_serializing_if = "Option::is_none")]
    pub end_date_time_utc: Option<String>,
    #[doc = "The metric value of the A endpoint"]
    #[serde(rename = "aValue", default, skip_serializing_if = "Option::is_none")]
    pub a_value: Option<f64>,
    #[doc = "The metric value of the B endpoint"]
    #[serde(rename = "bValue", default, skip_serializing_if = "Option::is_none")]
    pub b_value: Option<f64>,
    #[doc = "The difference in value between endpoint A and B"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta: Option<f64>,
    #[doc = "The percent difference between endpoint A and B"]
    #[serde(rename = "deltaPercent", default, skip_serializing_if = "Option::is_none")]
    pub delta_percent: Option<f64>,
    #[doc = "The lower end of the 95% confidence interval for endpoint A"]
    #[serde(rename = "aCLower95CI", default, skip_serializing_if = "Option::is_none")]
    pub a_c_lower95_ci: Option<f64>,
    #[doc = "The upper end of the 95% confidence interval for endpoint A"]
    #[serde(rename = "aHUpper95CI", default, skip_serializing_if = "Option::is_none")]
    pub a_h_upper95_ci: Option<f64>,
    #[doc = "The lower end of the 95% confidence interval for endpoint B"]
    #[serde(rename = "bCLower95CI", default, skip_serializing_if = "Option::is_none")]
    pub b_c_lower95_ci: Option<f64>,
    #[doc = "The upper end of the 95% confidence interval for endpoint B"]
    #[serde(rename = "bUpper95CI", default, skip_serializing_if = "Option::is_none")]
    pub b_upper95_ci: Option<f64>,
}
impl LatencyMetric {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the LatencyScorecard"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LatencyScorecard {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Defines a the properties of a Latency Scorecard"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LatencyScorecardProperties>,
}
impl LatencyScorecard {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a the properties of a Latency Scorecard"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LatencyScorecardProperties {
    #[doc = "The unique identifier of the Latency Scorecard"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the Latency Scorecard"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The description of the Latency Scorecard"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The A endpoint in the scorecard"]
    #[serde(rename = "endpointA", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_a: Option<String>,
    #[doc = "The B endpoint in the scorecard"]
    #[serde(rename = "endpointB", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_b: Option<String>,
    #[doc = "The start time of the Latency Scorecard in UTC"]
    #[serde(rename = "startDateTimeUTC", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The end time of the Latency Scorecard in UTC"]
    #[serde(rename = "endDateTimeUTC", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The country associated with the Latency Scorecard. Values are country ISO codes as specified here- https://www.iso.org/iso-3166-country-codes.html"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "The latency metrics of the Latency Scorecard"]
    #[serde(rename = "latencyMetrics", default, skip_serializing_if = "Vec::is_empty")]
    pub latency_metrics: Vec<LatencyMetric>,
}
impl LatencyScorecardProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list load balancing settings. It contains a list of load balancing settings objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancingSettingsListResult {
    #[doc = "List of Backend Pools within a Front Door."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LoadBalancingSettingsModel>,
    #[doc = "URL to get the next set of LoadBalancingSettings objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl LoadBalancingSettingsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Load balancing settings for a backend pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancingSettingsModel {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The JSON object that contains the properties required to create load balancing settings"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LoadBalancingSettingsProperties>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl LoadBalancingSettingsModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties required to create load balancing settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancingSettingsProperties {
    #[serde(flatten)]
    pub load_balancing_settings_update_parameters: LoadBalancingSettingsUpdateParameters,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
}
impl LoadBalancingSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Round-Robin load balancing settings for a backend pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancingSettingsUpdateParameters {
    #[doc = "The number of samples to consider for load balancing decisions"]
    #[serde(rename = "sampleSize", default, skip_serializing_if = "Option::is_none")]
    pub sample_size: Option<i64>,
    #[doc = "The number of samples within the sample period that must succeed"]
    #[serde(rename = "successfulSamplesRequired", default, skip_serializing_if = "Option::is_none")]
    pub successful_samples_required: Option<i64>,
    #[doc = "The additional latency in milliseconds for probes to fall into the lowest latency bucket"]
    #[serde(rename = "additionalLatencyMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub additional_latency_milliseconds: Option<i64>,
}
impl LoadBalancingSettingsUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a managed rule definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedRuleDefinition {
    #[doc = "Identifier for the managed rule."]
    #[serde(rename = "ruleId", default, skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[doc = "Describes if the managed rule is in enabled or disabled state."]
    #[serde(rename = "defaultState", default, skip_serializing_if = "Option::is_none")]
    pub default_state: Option<ManagedRuleEnabledState>,
    #[doc = "Defines the action to take on rule match."]
    #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
    pub default_action: Option<ActionType>,
    #[doc = "Describes the functionality of the managed rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ManagedRuleDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes if the managed rule is in enabled or disabled state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedRuleEnabledState")]
pub enum ManagedRuleEnabledState {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedRuleEnabledState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedRuleEnabledState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedRuleEnabledState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("ManagedRuleEnabledState", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("ManagedRuleEnabledState", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Exclude variables from managed rule evaluation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedRuleExclusion {
    #[doc = "The variable type to be excluded."]
    #[serde(rename = "matchVariable")]
    pub match_variable: managed_rule_exclusion::MatchVariable,
    #[doc = "Comparison operator to apply to the selector when specifying which elements in the collection this exclusion applies to."]
    #[serde(rename = "selectorMatchOperator")]
    pub selector_match_operator: managed_rule_exclusion::SelectorMatchOperator,
    #[doc = "Selector value for which elements in the collection this exclusion applies to."]
    pub selector: String,
}
impl ManagedRuleExclusion {
    pub fn new(
        match_variable: managed_rule_exclusion::MatchVariable,
        selector_match_operator: managed_rule_exclusion::SelectorMatchOperator,
        selector: String,
    ) -> Self {
        Self {
            match_variable,
            selector_match_operator,
            selector,
        }
    }
}
pub mod managed_rule_exclusion {
    use super::*;
    #[doc = "The variable type to be excluded."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MatchVariable")]
    pub enum MatchVariable {
        RequestHeaderNames,
        RequestCookieNames,
        QueryStringArgNames,
        RequestBodyPostArgNames,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MatchVariable {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MatchVariable {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MatchVariable {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::RequestHeaderNames => serializer.serialize_unit_variant("MatchVariable", 0u32, "RequestHeaderNames"),
                Self::RequestCookieNames => serializer.serialize_unit_variant("MatchVariable", 1u32, "RequestCookieNames"),
                Self::QueryStringArgNames => serializer.serialize_unit_variant("MatchVariable", 2u32, "QueryStringArgNames"),
                Self::RequestBodyPostArgNames => serializer.serialize_unit_variant("MatchVariable", 3u32, "RequestBodyPostArgNames"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Comparison operator to apply to the selector when specifying which elements in the collection this exclusion applies to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SelectorMatchOperator")]
    pub enum SelectorMatchOperator {
        Equals,
        Contains,
        StartsWith,
        EndsWith,
        EqualsAny,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SelectorMatchOperator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SelectorMatchOperator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SelectorMatchOperator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Equals => serializer.serialize_unit_variant("SelectorMatchOperator", 0u32, "Equals"),
                Self::Contains => serializer.serialize_unit_variant("SelectorMatchOperator", 1u32, "Contains"),
                Self::StartsWith => serializer.serialize_unit_variant("SelectorMatchOperator", 2u32, "StartsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("SelectorMatchOperator", 3u32, "EndsWith"),
                Self::EqualsAny => serializer.serialize_unit_variant("SelectorMatchOperator", 4u32, "EqualsAny"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a managed rule group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedRuleGroupDefinition {
    #[doc = "Name of the managed rule group."]
    #[serde(rename = "ruleGroupName", default, skip_serializing_if = "Option::is_none")]
    pub rule_group_name: Option<String>,
    #[doc = "Description of the managed rule group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "List of rules within the managed rule group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<ManagedRuleDefinition>,
}
impl ManagedRuleGroupDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a managed rule group override setting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedRuleGroupOverride {
    #[doc = "Describes the managed rule group to override."]
    #[serde(rename = "ruleGroupName")]
    pub rule_group_name: String,
    #[doc = "Describes the exclusions that are applied to all rules in the group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclusions: Vec<ManagedRuleExclusion>,
    #[doc = "List of rules that will be disabled. If none specified, all rules in the group will be disabled."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<ManagedRuleOverride>,
}
impl ManagedRuleGroupOverride {
    pub fn new(rule_group_name: String) -> Self {
        Self {
            rule_group_name,
            exclusions: Vec::new(),
            rules: Vec::new(),
        }
    }
}
#[doc = "Defines a managed rule group override setting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedRuleOverride {
    #[doc = "Identifier for the managed rule."]
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    #[doc = "Describes if the managed rule is in enabled or disabled state."]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<ManagedRuleEnabledState>,
    #[doc = "Defines the action to take on rule match."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionType>,
    #[doc = "Describes the exclusions that are applied to this specific rule."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclusions: Vec<ManagedRuleExclusion>,
}
impl ManagedRuleOverride {
    pub fn new(rule_id: String) -> Self {
        Self {
            rule_id,
            enabled_state: None,
            action: None,
            exclusions: Vec::new(),
        }
    }
}
#[doc = "Defines a managed rule set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedRuleSet {
    #[doc = "Defines the rule set type to use."]
    #[serde(rename = "ruleSetType")]
    pub rule_set_type: String,
    #[doc = "Defines the version of the rule set to use."]
    #[serde(rename = "ruleSetVersion")]
    pub rule_set_version: String,
    #[doc = "Describes the exclusions that are applied to all rules in the set."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclusions: Vec<ManagedRuleExclusion>,
    #[doc = "Defines the rule group overrides to apply to the rule set."]
    #[serde(rename = "ruleGroupOverrides", default, skip_serializing_if = "Vec::is_empty")]
    pub rule_group_overrides: Vec<ManagedRuleGroupOverride>,
}
impl ManagedRuleSet {
    pub fn new(rule_set_type: String, rule_set_version: String) -> Self {
        Self {
            rule_set_type,
            rule_set_version,
            exclusions: Vec::new(),
            rule_group_overrides: Vec::new(),
        }
    }
}
#[doc = "Describes the a managed rule set definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedRuleSetDefinition {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties for a managed rule set definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedRuleSetDefinitionProperties>,
}
impl ManagedRuleSetDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of managed rule set definitions available for use in a policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedRuleSetDefinitionList {
    #[doc = "List of managed rule set definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedRuleSetDefinition>,
    #[doc = "URL to retrieve next set of managed rule set definitions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedRuleSetDefinitionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagedRuleSetDefinitionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for a managed rule set definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedRuleSetDefinitionProperties {
    #[doc = "Provisioning state of the managed rule set."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Id of the managed rule set."]
    #[serde(rename = "ruleSetId", default, skip_serializing_if = "Option::is_none")]
    pub rule_set_id: Option<String>,
    #[doc = "Type of the managed rule set."]
    #[serde(rename = "ruleSetType", default, skip_serializing_if = "Option::is_none")]
    pub rule_set_type: Option<String>,
    #[doc = "Version of the managed rule set type."]
    #[serde(rename = "ruleSetVersion", default, skip_serializing_if = "Option::is_none")]
    pub rule_set_version: Option<String>,
    #[doc = "Rule groups of the managed rule set."]
    #[serde(rename = "ruleGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub rule_groups: Vec<ManagedRuleGroupDefinition>,
}
impl ManagedRuleSetDefinitionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the list of managed rule sets for the policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedRuleSetList {
    #[doc = "List of rule sets."]
    #[serde(rename = "managedRuleSets", default, skip_serializing_if = "Vec::is_empty")]
    pub managed_rule_sets: Vec<ManagedRuleSet>,
}
impl ManagedRuleSetList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Define a match condition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchCondition {
    #[doc = "Request variable to compare with."]
    #[serde(rename = "matchVariable")]
    pub match_variable: match_condition::MatchVariable,
    #[doc = "Match against a specific key from the QueryString, PostArgs, RequestHeader or Cookies variables. Default is null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[doc = "Comparison type to use for matching with the variable value."]
    pub operator: match_condition::Operator,
    #[doc = "Describes if the result of this condition should be negated."]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "List of possible match values."]
    #[serde(rename = "matchValue")]
    pub match_value: Vec<String>,
    #[doc = "List of transforms."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transforms: Vec<TransformType>,
}
impl MatchCondition {
    pub fn new(match_variable: match_condition::MatchVariable, operator: match_condition::Operator, match_value: Vec<String>) -> Self {
        Self {
            match_variable,
            selector: None,
            operator,
            negate_condition: None,
            match_value,
            transforms: Vec::new(),
        }
    }
}
pub mod match_condition {
    use super::*;
    #[doc = "Request variable to compare with."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MatchVariable")]
    pub enum MatchVariable {
        RemoteAddr,
        RequestMethod,
        QueryString,
        PostArgs,
        RequestUri,
        RequestHeader,
        RequestBody,
        Cookies,
        SocketAddr,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MatchVariable {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MatchVariable {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MatchVariable {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::RemoteAddr => serializer.serialize_unit_variant("MatchVariable", 0u32, "RemoteAddr"),
                Self::RequestMethod => serializer.serialize_unit_variant("MatchVariable", 1u32, "RequestMethod"),
                Self::QueryString => serializer.serialize_unit_variant("MatchVariable", 2u32, "QueryString"),
                Self::PostArgs => serializer.serialize_unit_variant("MatchVariable", 3u32, "PostArgs"),
                Self::RequestUri => serializer.serialize_unit_variant("MatchVariable", 4u32, "RequestUri"),
                Self::RequestHeader => serializer.serialize_unit_variant("MatchVariable", 5u32, "RequestHeader"),
                Self::RequestBody => serializer.serialize_unit_variant("MatchVariable", 6u32, "RequestBody"),
                Self::Cookies => serializer.serialize_unit_variant("MatchVariable", 7u32, "Cookies"),
                Self::SocketAddr => serializer.serialize_unit_variant("MatchVariable", 8u32, "SocketAddr"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Comparison type to use for matching with the variable value."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Any,
        #[serde(rename = "IPMatch")]
        IpMatch,
        GeoMatch,
        Equal,
        Contains,
        LessThan,
        GreaterThan,
        LessThanOrEqual,
        GreaterThanOrEqual,
        BeginsWith,
        EndsWith,
        RegEx,
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
                Self::Equal => serializer.serialize_unit_variant("Operator", 3u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 4u32, "Contains"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 6u32, "GreaterThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 7u32, "LessThanOrEqual"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 9u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 10u32, "EndsWith"),
                Self::RegEx => serializer.serialize_unit_variant("Operator", 11u32, "RegEx"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the server side resource status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NetworkExperimentResourceState")]
pub enum NetworkExperimentResourceState {
    Creating,
    Enabling,
    Enabled,
    Disabling,
    Disabled,
    Deleting,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NetworkExperimentResourceState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NetworkExperimentResourceState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NetworkExperimentResourceState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Creating => serializer.serialize_unit_variant("NetworkExperimentResourceState", 0u32, "Creating"),
            Self::Enabling => serializer.serialize_unit_variant("NetworkExperimentResourceState", 1u32, "Enabling"),
            Self::Enabled => serializer.serialize_unit_variant("NetworkExperimentResourceState", 2u32, "Enabled"),
            Self::Disabling => serializer.serialize_unit_variant("NetworkExperimentResourceState", 3u32, "Disabling"),
            Self::Disabled => serializer.serialize_unit_variant("NetworkExperimentResourceState", 4u32, "Disabled"),
            Self::Deleting => serializer.serialize_unit_variant("NetworkExperimentResourceState", 5u32, "Deleting"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines top-level WebApplicationFirewallPolicy configuration settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicySettings {
    #[doc = "Describes if the policy is in enabled or disabled state. Defaults to Enabled if not specified."]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<policy_settings::EnabledState>,
    #[doc = "Describes if it is in detection mode or prevention mode at policy level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<policy_settings::Mode>,
    #[doc = "If action type is redirect, this field represents redirect URL for the client."]
    #[serde(rename = "redirectUrl", default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[doc = "If the action type is block, customer can override the response status code."]
    #[serde(rename = "customBlockResponseStatusCode", default, skip_serializing_if = "Option::is_none")]
    pub custom_block_response_status_code: Option<i64>,
    #[doc = "If the action type is block, customer can override the response body. The body must be specified in base64 encoding."]
    #[serde(rename = "customBlockResponseBody", default, skip_serializing_if = "Option::is_none")]
    pub custom_block_response_body: Option<String>,
}
impl PolicySettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_settings {
    use super::*;
    #[doc = "Describes if the policy is in enabled or disabled state. Defaults to Enabled if not specified."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnabledState")]
    pub enum EnabledState {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnabledState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnabledState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnabledState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("EnabledState", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("EnabledState", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Describes if it is in detection mode or prevention mode at policy level."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        Prevention,
        Detection,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Mode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Mode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Mode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Prevention => serializer.serialize_unit_variant("Mode", 0u32, "Prevention"),
                Self::Detection => serializer.serialize_unit_variant("Mode", 1u32, "Detection"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the properties of a preconfigured endpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreconfiguredEndpoint {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The name of the endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Defines the properties of a preconfigured endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PreconfiguredEndpointProperties>,
}
impl PreconfiguredEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a list of preconfigured endpoints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreconfiguredEndpointList {
    #[doc = "List of PreconfiguredEndpoints supported by NetworkExperiment."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PreconfiguredEndpoint>,
    #[doc = "URL to get the next set of PreconfiguredEndpoints if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PreconfiguredEndpointList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PreconfiguredEndpointList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the properties of a preconfigured endpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreconfiguredEndpointProperties {
    #[doc = "The description of the endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The endpoint that is preconfigured"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "The type of endpoint"]
    #[serde(rename = "endpointType", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<preconfigured_endpoint_properties::EndpointType>,
    #[doc = "The preconfigured endpoint backend"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend: Option<String>,
}
impl PreconfiguredEndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod preconfigured_endpoint_properties {
    use super::*;
    #[doc = "The type of endpoint"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EndpointType")]
    pub enum EndpointType {
        #[serde(rename = "AFD")]
        Afd,
        AzureRegion,
        #[serde(rename = "CDN")]
        Cdn,
        #[serde(rename = "ATM")]
        Atm,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EndpointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EndpointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EndpointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Afd => serializer.serialize_unit_variant("EndpointType", 0u32, "AFD"),
                Self::AzureRegion => serializer.serialize_unit_variant("EndpointType", 1u32, "AzureRegion"),
                Self::Cdn => serializer.serialize_unit_variant("EndpointType", 2u32, "CDN"),
                Self::Atm => serializer.serialize_unit_variant("EndpointType", 3u32, "ATM"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines an Network Experiment Profile and lists of Experiments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Profile {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The name of the Profile"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Defines the properties of an experiment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProfileProperties>,
    #[doc = "Gets a unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Profile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a list of Profiles. It contains a list of Profile objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileList {
    #[doc = "List of Profiles within a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Profile>,
    #[doc = "URL to get the next set of Profile objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProfileList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProfileList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the properties of an experiment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileProperties {
    #[doc = "Defines the server side resource status"]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<NetworkExperimentResourceState>,
    #[doc = "The state of the Experiment"]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<profile_properties::EnabledState>,
}
impl ProfileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod profile_properties {
    use super::*;
    #[doc = "The state of the Experiment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnabledState")]
    pub enum EnabledState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnabledState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnabledState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnabledState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("EnabledState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("EnabledState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines modifiable attributes of a Profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileUpdateModel {
    #[doc = "Defines the properties of an experiment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProfileUpdateProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ProfileUpdateModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the properties of an experiment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileUpdateProperties {
    #[doc = "The enabled state of the Profile"]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<profile_update_properties::EnabledState>,
}
impl ProfileUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod profile_update_properties {
    use super::*;
    #[doc = "The enabled state of the Profile"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnabledState")]
    pub enum EnabledState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnabledState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnabledState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnabledState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("EnabledState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("EnabledState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "Describes Redirect Route."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedirectConfiguration {
    #[serde(flatten)]
    pub route_configuration: RouteConfiguration,
    #[doc = "The redirect type the rule will use when redirecting traffic."]
    #[serde(rename = "redirectType", default, skip_serializing_if = "Option::is_none")]
    pub redirect_type: Option<redirect_configuration::RedirectType>,
    #[doc = "The protocol of the destination to where the traffic is redirected"]
    #[serde(rename = "redirectProtocol", default, skip_serializing_if = "Option::is_none")]
    pub redirect_protocol: Option<redirect_configuration::RedirectProtocol>,
    #[doc = "Host to redirect. Leave empty to use the incoming host as the destination host."]
    #[serde(rename = "customHost", default, skip_serializing_if = "Option::is_none")]
    pub custom_host: Option<String>,
    #[doc = "The full path to redirect. Path cannot be empty and must start with /. Leave empty to use the incoming path as destination path."]
    #[serde(rename = "customPath", default, skip_serializing_if = "Option::is_none")]
    pub custom_path: Option<String>,
    #[doc = "Fragment to add to the redirect URL. Fragment is the part of the URL that comes after #. Do not include the #."]
    #[serde(rename = "customFragment", default, skip_serializing_if = "Option::is_none")]
    pub custom_fragment: Option<String>,
    #[doc = "The set of query strings to be placed in the redirect URL. Setting this value would replace any existing query string; leave empty to preserve the incoming query string. Query string must be in <key>=<value> format. The first ? and & will be added automatically so do not include them in the front, but do separate multiple query strings with &."]
    #[serde(rename = "customQueryString", default, skip_serializing_if = "Option::is_none")]
    pub custom_query_string: Option<String>,
}
impl RedirectConfiguration {
    pub fn new(route_configuration: RouteConfiguration) -> Self {
        Self {
            route_configuration,
            redirect_type: None,
            redirect_protocol: None,
            custom_host: None,
            custom_path: None,
            custom_fragment: None,
            custom_query_string: None,
        }
    }
}
pub mod redirect_configuration {
    use super::*;
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
    #[doc = "The protocol of the destination to where the traffic is redirected"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RedirectProtocol")]
    pub enum RedirectProtocol {
        HttpOnly,
        HttpsOnly,
        MatchRequest,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RedirectProtocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RedirectProtocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RedirectProtocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::HttpOnly => serializer.serialize_unit_variant("RedirectProtocol", 0u32, "HttpOnly"),
                Self::HttpsOnly => serializer.serialize_unit_variant("RedirectProtocol", 1u32, "HttpsOnly"),
                Self::MatchRequest => serializer.serialize_unit_variant("RedirectProtocol", 2u32, "MatchRequest"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Common resource representation."]
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
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceState")]
pub enum ResourceState {
    Creating,
    Enabling,
    Enabled,
    Disabling,
    Disabled,
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
            Self::Enabling => serializer.serialize_unit_variant("ResourceState", 1u32, "Enabling"),
            Self::Enabled => serializer.serialize_unit_variant("ResourceState", 2u32, "Enabled"),
            Self::Disabling => serializer.serialize_unit_variant("ResourceState", 3u32, "Disabling"),
            Self::Disabled => serializer.serialize_unit_variant("ResourceState", 4u32, "Disabled"),
            Self::Deleting => serializer.serialize_unit_variant("ResourceState", 5u32, "Deleting"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Type of Front Door resource used in CheckNameAvailability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    #[serde(rename = "Microsoft.Network/frontDoors")]
    MicrosoftNetworkFrontDoors,
    #[serde(rename = "Microsoft.Network/frontDoors/frontendEndpoints")]
    MicrosoftNetworkFrontDoorsFrontendEndpoints,
}
#[doc = "Base class for all types of Route."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteConfiguration {
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
}
impl RouteConfiguration {
    pub fn new(odata_type: String) -> Self {
        Self { odata_type }
    }
}
#[doc = "A routing rule represents a specification for traffic to treat and where to send it, along with health probe information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutingRule {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The JSON object that contains the properties required to create a routing rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoutingRuleProperties>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl RoutingRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the Resource ID for a Routing Rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutingRuleLink {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl RoutingRuleLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list Routing Rules. It contains a list of Routing Rule objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutingRuleListResult {
    #[doc = "List of Routing Rules within a Front Door."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoutingRule>,
    #[doc = "URL to get the next set of RoutingRule objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl RoutingRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties required to create a routing rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutingRuleProperties {
    #[serde(flatten)]
    pub routing_rule_update_parameters: RoutingRuleUpdateParameters,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
}
impl RoutingRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Routing rules to apply to an endpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutingRuleUpdateParameters {
    #[doc = "Frontend endpoints associated with this rule"]
    #[serde(rename = "frontendEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub frontend_endpoints: Vec<SubResource>,
    #[doc = "Protocol schemes to match for this rule"]
    #[serde(rename = "acceptedProtocols", default, skip_serializing_if = "Vec::is_empty")]
    pub accepted_protocols: Vec<String>,
    #[doc = "The route patterns of the rule."]
    #[serde(rename = "patternsToMatch", default, skip_serializing_if = "Vec::is_empty")]
    pub patterns_to_match: Vec<String>,
    #[doc = "Whether to enable use of this rule. Permitted values are 'Enabled' or 'Disabled'"]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<routing_rule_update_parameters::EnabledState>,
    #[doc = "Base class for all types of Route."]
    #[serde(rename = "routeConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub route_configuration: Option<RouteConfiguration>,
    #[doc = "Reference to another subresource."]
    #[serde(rename = "rulesEngine", default, skip_serializing_if = "Option::is_none")]
    pub rules_engine: Option<SubResource>,
    #[doc = "Defines the Web Application Firewall policy for each routing rule (if applicable)"]
    #[serde(rename = "webApplicationFirewallPolicyLink", default, skip_serializing_if = "Option::is_none")]
    pub web_application_firewall_policy_link: Option<routing_rule_update_parameters::WebApplicationFirewallPolicyLink>,
}
impl RoutingRuleUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod routing_rule_update_parameters {
    use super::*;
    #[doc = "Whether to enable use of this rule. Permitted values are 'Enabled' or 'Disabled'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnabledState")]
    pub enum EnabledState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnabledState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnabledState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnabledState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("EnabledState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("EnabledState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Defines the Web Application Firewall policy for each routing rule (if applicable)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct WebApplicationFirewallPolicyLink {
        #[doc = "Resource ID."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    impl WebApplicationFirewallPolicyLink {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A rules engine configuration containing a list of rules that will run to modify the runtime behavior of the request and response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RulesEngine {
    #[doc = "The JSON object that contains the properties required to create a Rules Engine Configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RulesEngineProperties>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl RulesEngine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "One or more actions that will execute, modifying the request and/or response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RulesEngineAction {
    #[doc = "A list of header actions to apply from the request from AFD to the origin."]
    #[serde(rename = "requestHeaderActions", default, skip_serializing_if = "Vec::is_empty")]
    pub request_header_actions: Vec<HeaderAction>,
    #[doc = "A list of header actions to apply from the response from AFD to the client."]
    #[serde(rename = "responseHeaderActions", default, skip_serializing_if = "Vec::is_empty")]
    pub response_header_actions: Vec<HeaderAction>,
    #[doc = "Base class for all types of Route."]
    #[serde(rename = "routeConfigurationOverride", default, skip_serializing_if = "Option::is_none")]
    pub route_configuration_override: Option<RouteConfiguration>,
}
impl RulesEngineAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list Rules Engine Configurations. It contains a list of RulesEngine objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RulesEngineListResult {
    #[doc = "List of rulesEngines within a Front Door."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RulesEngine>,
    #[doc = "URL to get the next set of RulesEngine objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RulesEngineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RulesEngineListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Define a match condition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RulesEngineMatchCondition {
    #[doc = "Match Variable"]
    #[serde(rename = "rulesEngineMatchVariable")]
    pub rules_engine_match_variable: rules_engine_match_condition::RulesEngineMatchVariable,
    #[doc = "Name of selector in RequestHeader or RequestBody to be matched"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[doc = "Describes operator to apply to the match condition."]
    #[serde(rename = "rulesEngineOperator")]
    pub rules_engine_operator: rules_engine_match_condition::RulesEngineOperator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "Match values to match against. The operator will apply to each value in here with OR semantics. If any of them match the variable with the given operator this match condition is considered a match."]
    #[serde(rename = "rulesEngineMatchValue")]
    pub rules_engine_match_value: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transforms: Vec<Transform>,
}
impl RulesEngineMatchCondition {
    pub fn new(
        rules_engine_match_variable: rules_engine_match_condition::RulesEngineMatchVariable,
        rules_engine_operator: rules_engine_match_condition::RulesEngineOperator,
        rules_engine_match_value: Vec<String>,
    ) -> Self {
        Self {
            rules_engine_match_variable,
            selector: None,
            rules_engine_operator,
            negate_condition: None,
            rules_engine_match_value,
            transforms: Vec::new(),
        }
    }
}
pub mod rules_engine_match_condition {
    use super::*;
    #[doc = "Match Variable"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RulesEngineMatchVariable")]
    pub enum RulesEngineMatchVariable {
        IsMobile,
        RemoteAddr,
        RequestMethod,
        QueryString,
        PostArgs,
        RequestUri,
        RequestPath,
        RequestFilename,
        RequestFilenameExtension,
        RequestHeader,
        RequestBody,
        RequestScheme,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RulesEngineMatchVariable {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RulesEngineMatchVariable {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RulesEngineMatchVariable {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IsMobile => serializer.serialize_unit_variant("RulesEngineMatchVariable", 0u32, "IsMobile"),
                Self::RemoteAddr => serializer.serialize_unit_variant("RulesEngineMatchVariable", 1u32, "RemoteAddr"),
                Self::RequestMethod => serializer.serialize_unit_variant("RulesEngineMatchVariable", 2u32, "RequestMethod"),
                Self::QueryString => serializer.serialize_unit_variant("RulesEngineMatchVariable", 3u32, "QueryString"),
                Self::PostArgs => serializer.serialize_unit_variant("RulesEngineMatchVariable", 4u32, "PostArgs"),
                Self::RequestUri => serializer.serialize_unit_variant("RulesEngineMatchVariable", 5u32, "RequestUri"),
                Self::RequestPath => serializer.serialize_unit_variant("RulesEngineMatchVariable", 6u32, "RequestPath"),
                Self::RequestFilename => serializer.serialize_unit_variant("RulesEngineMatchVariable", 7u32, "RequestFilename"),
                Self::RequestFilenameExtension => {
                    serializer.serialize_unit_variant("RulesEngineMatchVariable", 8u32, "RequestFilenameExtension")
                }
                Self::RequestHeader => serializer.serialize_unit_variant("RulesEngineMatchVariable", 9u32, "RequestHeader"),
                Self::RequestBody => serializer.serialize_unit_variant("RulesEngineMatchVariable", 10u32, "RequestBody"),
                Self::RequestScheme => serializer.serialize_unit_variant("RulesEngineMatchVariable", 11u32, "RequestScheme"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Describes operator to apply to the match condition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RulesEngineOperator")]
    pub enum RulesEngineOperator {
        Any,
        #[serde(rename = "IPMatch")]
        IpMatch,
        GeoMatch,
        Equal,
        Contains,
        LessThan,
        GreaterThan,
        LessThanOrEqual,
        GreaterThanOrEqual,
        BeginsWith,
        EndsWith,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RulesEngineOperator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RulesEngineOperator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RulesEngineOperator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Any => serializer.serialize_unit_variant("RulesEngineOperator", 0u32, "Any"),
                Self::IpMatch => serializer.serialize_unit_variant("RulesEngineOperator", 1u32, "IPMatch"),
                Self::GeoMatch => serializer.serialize_unit_variant("RulesEngineOperator", 2u32, "GeoMatch"),
                Self::Equal => serializer.serialize_unit_variant("RulesEngineOperator", 3u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("RulesEngineOperator", 4u32, "Contains"),
                Self::LessThan => serializer.serialize_unit_variant("RulesEngineOperator", 5u32, "LessThan"),
                Self::GreaterThan => serializer.serialize_unit_variant("RulesEngineOperator", 6u32, "GreaterThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("RulesEngineOperator", 7u32, "LessThanOrEqual"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("RulesEngineOperator", 8u32, "GreaterThanOrEqual"),
                Self::BeginsWith => serializer.serialize_unit_variant("RulesEngineOperator", 9u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("RulesEngineOperator", 10u32, "EndsWith"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The JSON object that contains the properties required to create a Rules Engine Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RulesEngineProperties {
    #[serde(flatten)]
    pub rules_engine_update_parameters: RulesEngineUpdateParameters,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
}
impl RulesEngineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains a list of match conditions, and an action on how to modify the request/response. If multiple rules match, the actions from one rule that conflict with a previous rule overwrite for a singular action, or append in the case of headers manipulation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RulesEngineRule {
    #[doc = "A name to refer to this specific rule."]
    pub name: String,
    #[doc = "A priority assigned to this rule. "]
    pub priority: i64,
    #[doc = "One or more actions that will execute, modifying the request and/or response."]
    pub action: RulesEngineAction,
    #[doc = "A list of match conditions that must meet in order for the actions of this rule to run. Having no match conditions means the actions will always run."]
    #[serde(rename = "matchConditions", default, skip_serializing_if = "Vec::is_empty")]
    pub match_conditions: Vec<RulesEngineMatchCondition>,
    #[doc = "If this rule is a match should the rules engine continue running the remaining rules or stop. If not present, defaults to Continue."]
    #[serde(rename = "matchProcessingBehavior", default, skip_serializing_if = "Option::is_none")]
    pub match_processing_behavior: Option<rules_engine_rule::MatchProcessingBehavior>,
}
impl RulesEngineRule {
    pub fn new(name: String, priority: i64, action: RulesEngineAction) -> Self {
        Self {
            name,
            priority,
            action,
            match_conditions: Vec::new(),
            match_processing_behavior: None,
        }
    }
}
pub mod rules_engine_rule {
    use super::*;
    #[doc = "If this rule is a match should the rules engine continue running the remaining rules or stop. If not present, defaults to Continue."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MatchProcessingBehavior")]
    pub enum MatchProcessingBehavior {
        Continue,
        Stop,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MatchProcessingBehavior {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MatchProcessingBehavior {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MatchProcessingBehavior {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Continue => serializer.serialize_unit_variant("MatchProcessingBehavior", 0u32, "Continue"),
                Self::Stop => serializer.serialize_unit_variant("MatchProcessingBehavior", 1u32, "Stop"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Rules Engine Configuration to apply to a Routing Rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RulesEngineUpdateParameters {
    #[doc = "A list of rules that define a particular Rules Engine Configuration."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<RulesEngineRule>,
}
impl RulesEngineUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to another subresource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tags object for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsObject {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the Timeseries"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Timeseries {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Defines the properties of a timeseries"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TimeseriesProperties>,
}
impl Timeseries {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a timeseries datapoint used in a timeseries"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeseriesDataPoint {
    #[doc = "The DateTime of the Timeseries data point in UTC"]
    #[serde(rename = "dateTimeUTC", default, skip_serializing_if = "Option::is_none")]
    pub date_time_utc: Option<String>,
    #[doc = "The Value of the Timeseries data point"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl TimeseriesDataPoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the properties of a timeseries"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeseriesProperties {
    #[doc = "The endpoint associated with the Timeseries data point"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "The start DateTime of the Timeseries in UTC"]
    #[serde(rename = "startDateTimeUTC", default, skip_serializing_if = "Option::is_none")]
    pub start_date_time_utc: Option<String>,
    #[doc = "The end DateTime of the Timeseries in UTC"]
    #[serde(rename = "endDateTimeUTC", default, skip_serializing_if = "Option::is_none")]
    pub end_date_time_utc: Option<String>,
    #[doc = "The aggregation interval of the Timeseries"]
    #[serde(rename = "aggregationInterval", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_interval: Option<timeseries_properties::AggregationInterval>,
    #[doc = "The type of Timeseries"]
    #[serde(rename = "timeseriesType", default, skip_serializing_if = "Option::is_none")]
    pub timeseries_type: Option<timeseries_properties::TimeseriesType>,
    #[doc = "The country associated with the Timeseries. Values are country ISO codes as specified here- https://www.iso.org/iso-3166-country-codes.html"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "The set of data points for the timeseries"]
    #[serde(rename = "timeseriesData", default, skip_serializing_if = "Vec::is_empty")]
    pub timeseries_data: Vec<TimeseriesDataPoint>,
}
impl TimeseriesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod timeseries_properties {
    use super::*;
    #[doc = "The aggregation interval of the Timeseries"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AggregationInterval")]
    pub enum AggregationInterval {
        Hourly,
        Daily,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AggregationInterval {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AggregationInterval {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AggregationInterval {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Hourly => serializer.serialize_unit_variant("AggregationInterval", 0u32, "Hourly"),
                Self::Daily => serializer.serialize_unit_variant("AggregationInterval", 1u32, "Daily"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of Timeseries"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TimeseriesType")]
    pub enum TimeseriesType {
        MeasurementCounts,
        LatencyP50,
        LatencyP75,
        LatencyP95,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TimeseriesType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TimeseriesType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TimeseriesType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MeasurementCounts => serializer.serialize_unit_variant("TimeseriesType", 0u32, "MeasurementCounts"),
                Self::LatencyP50 => serializer.serialize_unit_variant("TimeseriesType", 1u32, "LatencyP50"),
                Self::LatencyP75 => serializer.serialize_unit_variant("TimeseriesType", 2u32, "LatencyP75"),
                Self::LatencyP95 => serializer.serialize_unit_variant("TimeseriesType", 3u32, "LatencyP95"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes what transforms applied before matching."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TransformType")]
pub enum TransformType {
    Lowercase,
    Uppercase,
    Trim,
    UrlDecode,
    UrlEncode,
    RemoveNulls,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TransformType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TransformType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TransformType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Lowercase => serializer.serialize_unit_variant("TransformType", 0u32, "Lowercase"),
            Self::Uppercase => serializer.serialize_unit_variant("TransformType", 1u32, "Uppercase"),
            Self::Trim => serializer.serialize_unit_variant("TransformType", 2u32, "Trim"),
            Self::UrlDecode => serializer.serialize_unit_variant("TransformType", 3u32, "UrlDecode"),
            Self::UrlEncode => serializer.serialize_unit_variant("TransformType", 4u32, "UrlEncode"),
            Self::RemoveNulls => serializer.serialize_unit_variant("TransformType", 5u32, "RemoveNulls"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "Defines web application firewall policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplicationFirewallPolicy {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Defines web application firewall policy properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebApplicationFirewallPolicyProperties>,
    #[doc = "Gets a unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl WebApplicationFirewallPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a list of WebApplicationFirewallPolicies. It contains a list of WebApplicationFirewallPolicy objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplicationFirewallPolicyList {
    #[doc = "List of WebApplicationFirewallPolicies within a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WebApplicationFirewallPolicy>,
    #[doc = "URL to get the next set of WebApplicationFirewallPolicy objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WebApplicationFirewallPolicyList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WebApplicationFirewallPolicyList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines web application firewall policy properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplicationFirewallPolicyProperties {
    #[doc = "Defines top-level WebApplicationFirewallPolicy configuration settings."]
    #[serde(rename = "policySettings", default, skip_serializing_if = "Option::is_none")]
    pub policy_settings: Option<PolicySettings>,
    #[doc = "Defines contents of custom rules"]
    #[serde(rename = "customRules", default, skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<CustomRuleList>,
    #[doc = "Defines the list of managed rule sets for the policy."]
    #[serde(rename = "managedRules", default, skip_serializing_if = "Option::is_none")]
    pub managed_rules: Option<ManagedRuleSetList>,
    #[doc = "Describes Frontend Endpoints associated with this Web Application Firewall policy."]
    #[serde(rename = "frontendEndpointLinks", default, skip_serializing_if = "Vec::is_empty")]
    pub frontend_endpoint_links: Vec<FrontendEndpointLink>,
    #[doc = "Describes Routing Rules associated with this Web Application Firewall policy."]
    #[serde(rename = "routingRuleLinks", default, skip_serializing_if = "Vec::is_empty")]
    pub routing_rule_links: Vec<RoutingRuleLink>,
    #[doc = "Provisioning state of the policy."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<web_application_firewall_policy_properties::ResourceState>,
}
impl WebApplicationFirewallPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod web_application_firewall_policy_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceState")]
    pub enum ResourceState {
        Creating,
        Enabling,
        Enabled,
        Disabling,
        Disabled,
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
                Self::Enabling => serializer.serialize_unit_variant("ResourceState", 1u32, "Enabling"),
                Self::Enabled => serializer.serialize_unit_variant("ResourceState", 2u32, "Enabled"),
                Self::Disabling => serializer.serialize_unit_variant("ResourceState", 3u32, "Disabling"),
                Self::Disabled => serializer.serialize_unit_variant("ResourceState", 4u32, "Disabled"),
                Self::Deleting => serializer.serialize_unit_variant("ResourceState", 5u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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

#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Friendly domain name mapping to the endpoint hostname that the customer provides for branding purposes, e.g. www.contoso.com."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdDomain {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The JSON object that contains the properties of the domain to create."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AfdDomainProperties>,
}
impl AfdDomain {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties to secure a domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AfdDomainHttpsParameters {
    #[doc = "Defines the source of the SSL certificate."]
    #[serde(rename = "certificateType")]
    pub certificate_type: afd_domain_https_parameters::CertificateType,
    #[doc = "TLS protocol version that will be used for Https"]
    #[serde(rename = "minimumTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimum_tls_version: Option<afd_domain_https_parameters::MinimumTlsVersion>,
    #[doc = "Reference to another resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<ResourceReference>,
}
impl AfdDomainHttpsParameters {
    pub fn new(certificate_type: afd_domain_https_parameters::CertificateType) -> Self {
        Self {
            certificate_type,
            minimum_tls_version: None,
            secret: None,
        }
    }
}
pub mod afd_domain_https_parameters {
    use super::*;
    #[doc = "Defines the source of the SSL certificate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CertificateType")]
    pub enum CertificateType {
        CustomerCertificate,
        ManagedCertificate,
        AzureFirstPartyManagedCertificate,
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
                Self::CustomerCertificate => serializer.serialize_unit_variant("CertificateType", 0u32, "CustomerCertificate"),
                Self::ManagedCertificate => serializer.serialize_unit_variant("CertificateType", 1u32, "ManagedCertificate"),
                Self::AzureFirstPartyManagedCertificate => {
                    serializer.serialize_unit_variant("CertificateType", 2u32, "AzureFirstPartyManagedCertificate")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "TLS protocol version that will be used for Https"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MinimumTlsVersion {
        #[serde(rename = "TLS10")]
        Tls10,
        #[serde(rename = "TLS12")]
        Tls12,
    }
}
#[doc = "Result of the request to list domains. It contains a list of domain objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdDomainListResult {
    #[doc = "List of AzureFrontDoor domains within a profile."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AfdDomain>,
    #[doc = "URL to get the next set of domain objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AfdDomainListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AfdDomainListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties of the domain to create."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AfdDomainProperties {
    #[serde(flatten)]
    pub afd_domain_update_properties_parameters: AfdDomainUpdatePropertiesParameters,
    #[serde(flatten)]
    pub afd_state_properties: AfdStateProperties,
    #[doc = "Provisioning substate shows the progress of custom HTTPS enabling/disabling process step by step. DCV stands for DomainControlValidation."]
    #[serde(rename = "domainValidationState", default, skip_serializing_if = "Option::is_none")]
    pub domain_validation_state: Option<afd_domain_properties::DomainValidationState>,
    #[doc = "The host name of the domain. Must be a domain name."]
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[doc = "Key-Value pair representing migration properties for domains."]
    #[serde(rename = "extendedProperties", default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<serde_json::Value>,
    #[doc = "The JSON object that contains the properties to validate a domain."]
    #[serde(rename = "validationProperties", default, skip_serializing_if = "Option::is_none")]
    pub validation_properties: Option<DomainValidationProperties>,
}
impl AfdDomainProperties {
    pub fn new(host_name: String) -> Self {
        Self {
            afd_domain_update_properties_parameters: AfdDomainUpdatePropertiesParameters::default(),
            afd_state_properties: AfdStateProperties::default(),
            domain_validation_state: None,
            host_name,
            extended_properties: None,
            validation_properties: None,
        }
    }
}
pub mod afd_domain_properties {
    use super::*;
    #[doc = "Provisioning substate shows the progress of custom HTTPS enabling/disabling process step by step. DCV stands for DomainControlValidation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DomainValidationState")]
    pub enum DomainValidationState {
        Unknown,
        Submitting,
        Pending,
        Rejected,
        TimedOut,
        PendingRevalidation,
        Approved,
        RefreshingValidationToken,
        InternalError,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DomainValidationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DomainValidationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DomainValidationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("DomainValidationState", 0u32, "Unknown"),
                Self::Submitting => serializer.serialize_unit_variant("DomainValidationState", 1u32, "Submitting"),
                Self::Pending => serializer.serialize_unit_variant("DomainValidationState", 2u32, "Pending"),
                Self::Rejected => serializer.serialize_unit_variant("DomainValidationState", 3u32, "Rejected"),
                Self::TimedOut => serializer.serialize_unit_variant("DomainValidationState", 4u32, "TimedOut"),
                Self::PendingRevalidation => serializer.serialize_unit_variant("DomainValidationState", 5u32, "PendingRevalidation"),
                Self::Approved => serializer.serialize_unit_variant("DomainValidationState", 6u32, "Approved"),
                Self::RefreshingValidationToken => {
                    serializer.serialize_unit_variant("DomainValidationState", 7u32, "RefreshingValidationToken")
                }
                Self::InternalError => serializer.serialize_unit_variant("DomainValidationState", 8u32, "InternalError"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The domain JSON object required for domain creation or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdDomainUpdateParameters {
    #[doc = "The JSON object that contains the properties of the domain to create."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AfdDomainUpdatePropertiesParameters>,
}
impl AfdDomainUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties of the domain to create."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdDomainUpdatePropertiesParameters {
    #[doc = "The name of the profile which holds the domain."]
    #[serde(rename = "profileName", default, skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    #[doc = "The JSON object that contains the properties to secure a domain."]
    #[serde(rename = "tlsSettings", default, skip_serializing_if = "Option::is_none")]
    pub tls_settings: Option<AfdDomainHttpsParameters>,
    #[doc = "Reference to another resource."]
    #[serde(rename = "azureDnsZone", default, skip_serializing_if = "Option::is_none")]
    pub azure_dns_zone: Option<ResourceReference>,
    #[doc = "Reference to another resource."]
    #[serde(rename = "preValidatedCustomDomainResourceId", default, skip_serializing_if = "Option::is_none")]
    pub pre_validated_custom_domain_resource_id: Option<ResourceReference>,
}
impl AfdDomainUpdatePropertiesParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Front Door endpoint is the entity within a Azure Front Door profile containing configuration information such as origin, protocol, content caching and delivery behavior. The AzureFrontDoor endpoint uses the URL format <endpointname>.azureedge.net."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AfdEndpoint {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The JSON object that contains the properties required to create an endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AfdEndpointProperties>,
}
impl AfdEndpoint {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Result of the request to list endpoints. It contains a list of endpoint objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdEndpointListResult {
    #[doc = "List of AzureFrontDoor endpoints within a profile"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AfdEndpoint>,
    #[doc = "URL to get the next set of endpoint objects if there is any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AfdEndpointListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AfdEndpointListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties required to create an endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdEndpointProperties {
    #[serde(flatten)]
    pub afd_endpoint_properties_update_parameters: AfdEndpointPropertiesUpdateParameters,
    #[serde(flatten)]
    pub afd_state_properties: AfdStateProperties,
    #[doc = "The host name of the endpoint structured as {endpointName}.{DNSZone}, e.g. contoso.azureedge.net"]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "Indicates the endpoint name reuse scope. The default value is TenantReuse."]
    #[serde(rename = "autoGeneratedDomainNameLabelScope", default, skip_serializing_if = "Option::is_none")]
    pub auto_generated_domain_name_label_scope: Option<AutoGeneratedDomainNameLabelScope>,
}
impl AfdEndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object containing endpoint update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdEndpointPropertiesUpdateParameters {
    #[doc = "The name of the profile which holds the endpoint."]
    #[serde(rename = "profileName", default, skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    #[doc = "Whether to enable use of this rule. Permitted values are 'Enabled' or 'Disabled'"]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<afd_endpoint_properties_update_parameters::EnabledState>,
}
impl AfdEndpointPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod afd_endpoint_properties_update_parameters {
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
}
#[doc = "Supported protocols for the customer's endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AfdEndpointProtocols")]
pub enum AfdEndpointProtocols {
    Http,
    Https,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AfdEndpointProtocols {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AfdEndpointProtocols {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AfdEndpointProtocols {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Http => serializer.serialize_unit_variant("AfdEndpointProtocols", 0u32, "Http"),
            Self::Https => serializer.serialize_unit_variant("AfdEndpointProtocols", 1u32, "Https"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties required to create or update an endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdEndpointUpdateParameters {
    #[doc = "Endpoint tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The JSON object containing endpoint update parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AfdEndpointPropertiesUpdateParameters>,
}
impl AfdEndpointUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Front Door origin is the source of the content being delivered via Azure Front Door. When the edge nodes represented by an endpoint do not have the requested content cached, they attempt to fetch it from one or more of the configured origins."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdOrigin {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The JSON object that contains the properties of the origin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AfdOriginProperties>,
}
impl AfdOrigin {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AFDOrigin group comprising of origins is used for load balancing to origins when the content cannot be served from Azure Front Door."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdOriginGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The JSON object that contains the properties of the origin group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AfdOriginGroupProperties>,
}
impl AfdOriginGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list origin groups. It contains a list of origin groups objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdOriginGroupListResult {
    #[doc = "List of Azure Front Door origin groups within an Azure Front Door endpoint"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AfdOriginGroup>,
    #[doc = "URL to get the next set of origin objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AfdOriginGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AfdOriginGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties of the origin group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdOriginGroupProperties {
    #[serde(flatten)]
    pub afd_origin_group_update_properties_parameters: AfdOriginGroupUpdatePropertiesParameters,
    #[serde(flatten)]
    pub afd_state_properties: AfdStateProperties,
}
impl AfdOriginGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AFDOrigin group properties needed for origin group creation or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdOriginGroupUpdateParameters {
    #[doc = "The JSON object that contains the properties of the origin group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AfdOriginGroupUpdatePropertiesParameters>,
}
impl AfdOriginGroupUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties of the origin group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdOriginGroupUpdatePropertiesParameters {
    #[doc = "The name of the profile which holds the origin group."]
    #[serde(rename = "profileName", default, skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    #[doc = "Round-Robin load balancing settings for a backend pool"]
    #[serde(rename = "loadBalancingSettings", default, skip_serializing_if = "Option::is_none")]
    pub load_balancing_settings: Option<LoadBalancingSettingsParameters>,
    #[doc = "The JSON object that contains the properties to send health probes to origin."]
    #[serde(rename = "healthProbeSettings", default, skip_serializing_if = "Option::is_none")]
    pub health_probe_settings: Option<HealthProbeParameters>,
    #[doc = "Time in minutes to shift the traffic to the endpoint gradually when an unhealthy endpoint comes healthy or a new endpoint is added. Default is 10 mins. This property is currently not supported."]
    #[serde(
        rename = "trafficRestorationTimeToHealedOrNewEndpointsInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_restoration_time_to_healed_or_new_endpoints_in_minutes: Option<i32>,
    #[doc = "Whether to allow session affinity on this host. Valid options are 'Enabled' or 'Disabled'"]
    #[serde(rename = "sessionAffinityState", default, skip_serializing_if = "Option::is_none")]
    pub session_affinity_state: Option<afd_origin_group_update_properties_parameters::SessionAffinityState>,
}
impl AfdOriginGroupUpdatePropertiesParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod afd_origin_group_update_properties_parameters {
    use super::*;
    #[doc = "Whether to allow session affinity on this host. Valid options are 'Enabled' or 'Disabled'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SessionAffinityState")]
    pub enum SessionAffinityState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SessionAffinityState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SessionAffinityState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SessionAffinityState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("SessionAffinityState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("SessionAffinityState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of the request to list origins. It contains a list of origin objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdOriginListResult {
    #[doc = "List of Azure Front Door origins within an Azure Front Door endpoint"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AfdOrigin>,
    #[doc = "URL to get the next set of origin objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AfdOriginListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AfdOriginListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties of the origin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AfdOriginProperties {
    #[serde(flatten)]
    pub afd_origin_update_properties_parameters: AfdOriginUpdatePropertiesParameters,
    #[serde(flatten)]
    pub afd_state_properties: AfdStateProperties,
}
impl AfdOriginProperties {
    pub fn new() -> Self {
        Self {
            afd_origin_update_properties_parameters: AfdOriginUpdatePropertiesParameters::default(),
            afd_state_properties: AfdStateProperties::default(),
        }
    }
}
#[doc = "AFDOrigin properties needed for origin update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdOriginUpdateParameters {
    #[doc = "The JSON object that contains the properties of the origin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AfdOriginUpdatePropertiesParameters>,
}
impl AfdOriginUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties of the origin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdOriginUpdatePropertiesParameters {
    #[doc = "The name of the origin group which contains this origin."]
    #[serde(rename = "originGroupName", default, skip_serializing_if = "Option::is_none")]
    pub origin_group_name: Option<String>,
    #[doc = "Reference to another resource."]
    #[serde(rename = "azureOrigin", default, skip_serializing_if = "Option::is_none")]
    pub azure_origin: Option<ResourceReference>,
    #[doc = "The address of the origin. Domain names, IPv4 addresses, and IPv6 addresses are supported.This should be unique across all origins in an endpoint."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "The value of the HTTP port. Must be between 1 and 65535."]
    #[serde(rename = "httpPort", default, skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i32>,
    #[doc = "The value of the HTTPS port. Must be between 1 and 65535."]
    #[serde(rename = "httpsPort", default, skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i32>,
    #[doc = "The host header value sent to the origin with each request. If you leave this blank, the request hostname determines this value. Azure Front Door origins, such as Web Apps, Blob Storage, and Cloud Services require this host header value to match the origin hostname by default. This overrides the host header defined at Endpoint"]
    #[serde(rename = "originHostHeader", default, skip_serializing_if = "Option::is_none")]
    pub origin_host_header: Option<String>,
    #[doc = "Priority of origin in given origin group for load balancing. Higher priorities will not be used for load balancing if any lower priority origin is healthy.Must be between 1 and 5"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "Weight of the origin in given origin group for load balancing. Must be between 1 and 1000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[doc = "Describes the properties of an existing Shared Private Link Resource to use when connecting to a private origin."]
    #[serde(rename = "sharedPrivateLinkResource", default, skip_serializing_if = "Option::is_none")]
    pub shared_private_link_resource: Option<SharedPrivateLinkResourceProperties>,
    #[doc = "Whether to enable health probes to be made against backends defined under backendPools. Health probes can only be disabled if there is a single enabled backend in single enabled backend pool."]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<afd_origin_update_properties_parameters::EnabledState>,
    #[doc = "Whether to enable certificate name check at origin level"]
    #[serde(rename = "enforceCertificateNameCheck", default, skip_serializing_if = "Option::is_none")]
    pub enforce_certificate_name_check: Option<bool>,
}
impl AfdOriginUpdatePropertiesParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod afd_origin_update_properties_parameters {
    use super::*;
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
#[doc = "The tracking states for afd resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdStateProperties {
    #[doc = "Provisioning status"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<afd_state_properties::ProvisioningState>,
    #[serde(rename = "deploymentStatus", default, skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<afd_state_properties::DeploymentStatus>,
}
impl AfdStateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod afd_state_properties {
    use super::*;
    #[doc = "Provisioning status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Updating,
        Deleting,
        Creating,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeploymentStatus")]
    pub enum DeploymentStatus {
        NotStarted,
        InProgress,
        Succeeded,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeploymentStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeploymentStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeploymentStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotStarted => serializer.serialize_unit_variant("DeploymentStatus", 0u32, "NotStarted"),
                Self::InProgress => serializer.serialize_unit_variant("DeploymentStatus", 1u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("DeploymentStatus", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("DeploymentStatus", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
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
#[doc = "Reference to another resource along with its state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivatedResourceReference {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Whether the resource is active or inactive"]
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}
impl ActivatedResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl azure_core::Continuable for AfdErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AfdErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters required for content purge."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AfdPurgeParameters {
    #[doc = "The path to the content to be purged. Can describe a file path or a wild card directory."]
    #[serde(rename = "contentPaths")]
    pub content_paths: Vec<String>,
    #[doc = "List of domains."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub domains: Vec<String>,
}
impl AfdPurgeParameters {
    pub fn new(content_paths: Vec<String>) -> Self {
        Self {
            content_paths,
            domains: Vec::new(),
        }
    }
}
#[doc = "Caching settings for a caching-type route. To disable caching, do not provide a cacheConfiguration object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AfdRouteCacheConfiguration {
    #[doc = "Defines how Frontdoor caches requests that include query strings. You can ignore any query strings when caching, ignore specific query strings, cache every request with a unique URL, or cache specific query strings."]
    #[serde(rename = "queryStringCachingBehavior", default, skip_serializing_if = "Option::is_none")]
    pub query_string_caching_behavior: Option<afd_route_cache_configuration::QueryStringCachingBehavior>,
    #[doc = "query parameters to include or exclude (comma separated)."]
    #[serde(rename = "queryParameters", default, skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<String>,
    #[doc = "settings for compression."]
    #[serde(rename = "compressionSettings", default, skip_serializing_if = "Option::is_none")]
    pub compression_settings: Option<CompressionSettings>,
}
impl AfdRouteCacheConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod afd_route_cache_configuration {
    use super::*;
    #[doc = "Defines how Frontdoor caches requests that include query strings. You can ignore any query strings when caching, ignore specific query strings, cache every request with a unique URL, or cache specific query strings."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "QueryStringCachingBehavior")]
    pub enum QueryStringCachingBehavior {
        IgnoreQueryString,
        UseQueryString,
        IgnoreSpecifiedQueryStrings,
        IncludeSpecifiedQueryStrings,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for QueryStringCachingBehavior {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for QueryStringCachingBehavior {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for QueryStringCachingBehavior {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IgnoreQueryString => serializer.serialize_unit_variant("QueryStringCachingBehavior", 0u32, "IgnoreQueryString"),
                Self::UseQueryString => serializer.serialize_unit_variant("QueryStringCachingBehavior", 1u32, "UseQueryString"),
                Self::IgnoreSpecifiedQueryStrings => {
                    serializer.serialize_unit_variant("QueryStringCachingBehavior", 2u32, "IgnoreSpecifiedQueryStrings")
                }
                Self::IncludeSpecifiedQueryStrings => {
                    serializer.serialize_unit_variant("QueryStringCachingBehavior", 3u32, "IncludeSpecifiedQueryStrings")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the url signing action for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AfdUrlSigningAction {
    #[serde(flatten)]
    pub delivery_rule_action: DeliveryRuleAction,
    #[doc = "Defines the parameters for the Url Signing action."]
    pub parameters: AfdUrlSigningActionParameters,
}
impl AfdUrlSigningAction {
    pub fn new(delivery_rule_action: DeliveryRuleAction, parameters: AfdUrlSigningActionParameters) -> Self {
        Self {
            delivery_rule_action,
            parameters,
        }
    }
}
#[doc = "Defines the parameters for the Url Signing action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AfdUrlSigningActionParameters {
    #[serde(rename = "typeName")]
    pub type_name: afd_url_signing_action_parameters::TypeName,
    #[doc = "Reference to another resource."]
    #[serde(rename = "keyGroupReference")]
    pub key_group_reference: ResourceReference,
    #[doc = "Algorithm to use for URL signing"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<afd_url_signing_action_parameters::Algorithm>,
    #[doc = "Defines which query string parameters in the url to be considered for expires, key id etc. "]
    #[serde(
        rename = "parameterNameOverride",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parameter_name_override: Vec<UrlSigningParamIdentifier>,
}
impl AfdUrlSigningActionParameters {
    pub fn new(type_name: afd_url_signing_action_parameters::TypeName, key_group_reference: ResourceReference) -> Self {
        Self {
            type_name,
            key_group_reference,
            algorithm: None,
            parameter_name_override: Vec::new(),
        }
    }
}
pub mod afd_url_signing_action_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TypeName")]
    pub enum TypeName {
        DeliveryRuleAfdUrlSigningActionParameters,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TypeName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TypeName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TypeName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::DeliveryRuleAfdUrlSigningActionParameters => {
                    serializer.serialize_unit_variant("TypeName", 0u32, "DeliveryRuleAfdUrlSigningActionParameters")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Algorithm to use for URL signing"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Algorithm")]
    pub enum Algorithm {
        #[serde(rename = "SHA256")]
        Sha256,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Algorithm {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Algorithm {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Algorithm {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Sha256 => serializer.serialize_unit_variant("Algorithm", 0u32, "SHA256"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Indicates the endpoint name reuse scope. The default value is TenantReuse."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutoGeneratedDomainNameLabelScope")]
pub enum AutoGeneratedDomainNameLabelScope {
    TenantReuse,
    SubscriptionReuse,
    ResourceGroupReuse,
    NoReuse,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutoGeneratedDomainNameLabelScope {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutoGeneratedDomainNameLabelScope {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutoGeneratedDomainNameLabelScope {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::TenantReuse => serializer.serialize_unit_variant("AutoGeneratedDomainNameLabelScope", 0u32, "TenantReuse"),
            Self::SubscriptionReuse => serializer.serialize_unit_variant("AutoGeneratedDomainNameLabelScope", 1u32, "SubscriptionReuse"),
            Self::ResourceGroupReuse => serializer.serialize_unit_variant("AutoGeneratedDomainNameLabelScope", 2u32, "ResourceGroupReuse"),
            Self::NoReuse => serializer.serialize_unit_variant("AutoGeneratedDomainNameLabelScope", 3u32, "NoReuse"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Azure FirstParty Managed Certificate provided by other first party resource providers to enable HTTPS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureFirstPartyManagedCertificate {
    #[serde(flatten)]
    pub certificate: Certificate,
}
impl AzureFirstPartyManagedCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure FirstParty Managed Certificate provided by other first party resource providers to enable HTTPS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFirstPartyManagedCertificateParameters {
    #[serde(flatten)]
    pub secret_parameters: SecretParameters,
    #[doc = "Reference to another resource."]
    #[serde(rename = "secretSource", default, skip_serializing_if = "Option::is_none")]
    pub secret_source: Option<ResourceReference>,
    #[doc = "Subject name in the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Certificate expiration date."]
    #[serde(rename = "expirationDate", default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[doc = "Certificate issuing authority."]
    #[serde(rename = "certificateAuthority", default, skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<String>,
    #[doc = "The list of SANs."]
    #[serde(
        rename = "subjectAlternativeNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subject_alternative_names: Vec<String>,
    #[doc = "Certificate thumbprint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}
impl AzureFirstPartyManagedCertificateParameters {
    pub fn new(secret_parameters: SecretParameters) -> Self {
        Self {
            secret_parameters,
            secret_source: None,
            subject: None,
            expiration_date: None,
            certificate_authority: None,
            subject_alternative_names: Vec::new(),
            thumbprint: None,
        }
    }
}
#[doc = "Caching settings for a caching-type route. To disable caching, do not provide a cacheConfiguration object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheConfiguration {
    #[doc = "Defines how Frontdoor caches requests that include query strings. You can ignore any query strings when caching, ignore specific query strings, cache every request with a unique URL, or cache specific query strings."]
    #[serde(rename = "queryStringCachingBehavior", default, skip_serializing_if = "Option::is_none")]
    pub query_string_caching_behavior: Option<cache_configuration::QueryStringCachingBehavior>,
    #[doc = "query parameters to include or exclude (comma separated)."]
    #[serde(rename = "queryParameters", default, skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<String>,
    #[doc = "Indicates whether content compression is enabled. If compression is enabled, content will be served as compressed if user requests for a compressed version. Content won't be compressed on AzureFrontDoor when requested content is smaller than 1 byte or larger than 1 MB."]
    #[serde(rename = "isCompressionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_compression_enabled: Option<cache_configuration::IsCompressionEnabled>,
    #[doc = "Caching behavior for the requests"]
    #[serde(rename = "cacheBehavior", default, skip_serializing_if = "Option::is_none")]
    pub cache_behavior: Option<cache_configuration::CacheBehavior>,
    #[doc = "The duration for which the content needs to be cached. Allowed format is [d.]hh:mm:ss"]
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
    #[doc = "Defines how Frontdoor caches requests that include query strings. You can ignore any query strings when caching, ignore specific query strings, cache every request with a unique URL, or cache specific query strings."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "QueryStringCachingBehavior")]
    pub enum QueryStringCachingBehavior {
        IgnoreQueryString,
        UseQueryString,
        IgnoreSpecifiedQueryStrings,
        IncludeSpecifiedQueryStrings,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for QueryStringCachingBehavior {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for QueryStringCachingBehavior {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for QueryStringCachingBehavior {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IgnoreQueryString => serializer.serialize_unit_variant("QueryStringCachingBehavior", 0u32, "IgnoreQueryString"),
                Self::UseQueryString => serializer.serialize_unit_variant("QueryStringCachingBehavior", 1u32, "UseQueryString"),
                Self::IgnoreSpecifiedQueryStrings => {
                    serializer.serialize_unit_variant("QueryStringCachingBehavior", 2u32, "IgnoreSpecifiedQueryStrings")
                }
                Self::IncludeSpecifiedQueryStrings => {
                    serializer.serialize_unit_variant("QueryStringCachingBehavior", 3u32, "IncludeSpecifiedQueryStrings")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates whether content compression is enabled. If compression is enabled, content will be served as compressed if user requests for a compressed version. Content won't be compressed on AzureFrontDoor when requested content is smaller than 1 byte or larger than 1 MB."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsCompressionEnabled")]
    pub enum IsCompressionEnabled {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsCompressionEnabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsCompressionEnabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsCompressionEnabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("IsCompressionEnabled", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("IsCompressionEnabled", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Caching behavior for the requests"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CacheBehavior")]
    pub enum CacheBehavior {
        HonorOrigin,
        OverrideAlways,
        OverrideIfOriginMissing,
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
                Self::HonorOrigin => serializer.serialize_unit_variant("CacheBehavior", 0u32, "HonorOrigin"),
                Self::OverrideAlways => serializer.serialize_unit_variant("CacheBehavior", 1u32, "OverrideAlways"),
                Self::OverrideIfOriginMissing => serializer.serialize_unit_variant("CacheBehavior", 2u32, "OverrideIfOriginMissing"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameters for the cache expiration action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheExpirationActionParameters {
    #[serde(rename = "typeName")]
    pub type_name: cache_expiration_action_parameters::TypeName,
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
        type_name: cache_expiration_action_parameters::TypeName,
        cache_behavior: cache_expiration_action_parameters::CacheBehavior,
        cache_type: cache_expiration_action_parameters::CacheType,
    ) -> Self {
        Self {
            type_name,
            cache_behavior,
            cache_type,
            cache_duration: None,
        }
    }
}
pub mod cache_expiration_action_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        DeliveryRuleCacheExpirationActionParameters,
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
    #[serde(rename = "typeName")]
    pub type_name: cache_key_query_string_action_parameters::TypeName,
    #[doc = "Caching behavior for the requests"]
    #[serde(rename = "queryStringBehavior")]
    pub query_string_behavior: cache_key_query_string_action_parameters::QueryStringBehavior,
    #[doc = "query parameters to include or exclude (comma separated)."]
    #[serde(rename = "queryParameters", default, skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<String>,
}
impl CacheKeyQueryStringActionParameters {
    pub fn new(
        type_name: cache_key_query_string_action_parameters::TypeName,
        query_string_behavior: cache_key_query_string_action_parameters::QueryStringBehavior,
    ) -> Self {
        Self {
            type_name,
            query_string_behavior,
            query_parameters: None,
        }
    }
}
pub mod cache_key_query_string_action_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        DeliveryRuleCacheKeyQueryStringBehaviorActionParameters,
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
#[doc = "Request body for CanMigrate operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CanMigrateParameters {
    #[doc = "Reference to another resource."]
    #[serde(rename = "classicResourceReference")]
    pub classic_resource_reference: ResourceReference,
}
impl CanMigrateParameters {
    pub fn new(classic_resource_reference: ResourceReference) -> Self {
        Self {
            classic_resource_reference,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CanMigrateProperties {
    #[doc = "Flag that says if the profile can be migrated"]
    #[serde(rename = "canMigrate", default, skip_serializing_if = "Option::is_none")]
    pub can_migrate: Option<bool>,
    #[doc = "Recommended sku for the migration"]
    #[serde(rename = "defaultSku", default, skip_serializing_if = "Option::is_none")]
    pub default_sku: Option<can_migrate_properties::DefaultSku>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<MigrationErrorType>,
}
impl CanMigrateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod can_migrate_properties {
    use super::*;
    #[doc = "Recommended sku for the migration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DefaultSku")]
    pub enum DefaultSku {
        #[serde(rename = "Standard_AzureFrontDoor")]
        StandardAzureFrontDoor,
        #[serde(rename = "Premium_AzureFrontDoor")]
        PremiumAzureFrontDoor,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DefaultSku {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DefaultSku {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DefaultSku {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardAzureFrontDoor => serializer.serialize_unit_variant("DefaultSku", 0u32, "Standard_AzureFrontDoor"),
                Self::PremiumAzureFrontDoor => serializer.serialize_unit_variant("DefaultSku", 1u32, "Premium_AzureFrontDoor"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result for canMigrate operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CanMigrateResult {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CanMigrateProperties>,
}
impl CanMigrateResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the parameters for using CDN managed certificate for securing custom domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CdnCertificateSourceParameters {
    #[serde(rename = "typeName")]
    pub type_name: cdn_certificate_source_parameters::TypeName,
    #[doc = "Type of certificate used"]
    #[serde(rename = "certificateType")]
    pub certificate_type: cdn_certificate_source_parameters::CertificateType,
}
impl CdnCertificateSourceParameters {
    pub fn new(
        type_name: cdn_certificate_source_parameters::TypeName,
        certificate_type: cdn_certificate_source_parameters::CertificateType,
    ) -> Self {
        Self {
            type_name,
            certificate_type,
        }
    }
}
pub mod cdn_certificate_source_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        CdnCertificateSourceParameters,
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
#[doc = "Defines the ARM Resource ID for the linked endpoints"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CdnEndpoint {
    #[doc = "ARM Resource ID string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl CdnEndpoint {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Defines web application firewall policy for Azure CDN."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CdnWebApplicationFirewallPolicy {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Defines CDN web application firewall policy properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CdnWebApplicationFirewallPolicyProperties>,
    #[doc = "Gets a unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Standard_Verizon = The SKU name for a Standard Verizon CDN profile.\nPremium_Verizon = The SKU name for a Premium Verizon CDN profile.\nCustom_Verizon = The SKU name for a Custom Verizon CDN profile.\nStandard_Akamai = The SKU name for an Akamai CDN profile.\nStandard_ChinaCdn = The SKU name for a China CDN profile for VOD, Web and download scenarios using GB based billing model.\nStandard_Microsoft = The SKU name for a Standard Microsoft CDN profile.\nStandard_AzureFrontDoor =  The SKU name for an Azure Front Door Standard profile.\nPremium_AzureFrontDoor = The SKU name for an Azure Front Door Premium profile.\nStandard_955BandWidth_ChinaCdn = The SKU name for a China CDN profile for VOD, Web and download scenarios using 95-5 peak bandwidth billing model.\nStandard_AvgBandWidth_ChinaCdn = The SKU name for a China CDN profile for VOD, Web and download scenarios using monthly average peak bandwidth billing model.\nStandardPlus_ChinaCdn = The SKU name for a China CDN profile for live-streaming using GB based billing model.\nStandardPlus_955BandWidth_ChinaCdn = The SKU name for a China CDN live-streaming profile using 95-5 peak bandwidth billing model.\nStandardPlus_AvgBandWidth_ChinaCdn = The SKU name for a China CDN live-streaming profile using monthly average peak bandwidth billing model.\n"]
    pub sku: Sku,
}
impl CdnWebApplicationFirewallPolicy {
    pub fn new(tracked_resource: TrackedResource, sku: Sku) -> Self {
        Self {
            tracked_resource,
            properties: None,
            etag: None,
            sku,
        }
    }
}
#[doc = "Defines a list of WebApplicationFirewallPolicies for Azure CDN. It contains a list of WebApplicationFirewallPolicy objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CdnWebApplicationFirewallPolicyList {
    #[doc = "List of Azure CDN WebApplicationFirewallPolicies within a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CdnWebApplicationFirewallPolicy>,
    #[doc = "URL to get the next set of WebApplicationFirewallPolicy objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CdnWebApplicationFirewallPolicyList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CdnWebApplicationFirewallPolicyList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties required to update a CdnWebApplicationFirewallPolicy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CdnWebApplicationFirewallPolicyPatchParameters {
    #[doc = "CdnWebApplicationFirewallPolicy tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CdnWebApplicationFirewallPolicyPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines CDN web application firewall policy properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CdnWebApplicationFirewallPolicyProperties {
    #[doc = "Defines contents of a web application firewall global configuration"]
    #[serde(rename = "policySettings", default, skip_serializing_if = "Option::is_none")]
    pub policy_settings: Option<PolicySettings>,
    #[doc = "Defines contents of rate limit rules"]
    #[serde(rename = "rateLimitRules", default, skip_serializing_if = "Option::is_none")]
    pub rate_limit_rules: Option<RateLimitRuleList>,
    #[doc = "Defines contents of custom rules"]
    #[serde(rename = "customRules", default, skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<CustomRuleList>,
    #[doc = "Defines the list of managed rule sets for the policy."]
    #[serde(rename = "managedRules", default, skip_serializing_if = "Option::is_none")]
    pub managed_rules: Option<ManagedRuleSetList>,
    #[doc = "Describes Azure CDN endpoints associated with this Web Application Firewall policy."]
    #[serde(
        rename = "endpointLinks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endpoint_links: Vec<CdnEndpoint>,
    #[doc = "Key-Value pair representing additional properties for Web Application Firewall policy."]
    #[serde(rename = "extendedProperties", default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<serde_json::Value>,
    #[doc = "Provisioning state of the WebApplicationFirewallPolicy."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<cdn_web_application_firewall_policy_properties::ProvisioningState>,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<cdn_web_application_firewall_policy_properties::ResourceState>,
}
impl CdnWebApplicationFirewallPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cdn_web_application_firewall_policy_properties {
    use super::*;
    #[doc = "Provisioning state of the WebApplicationFirewallPolicy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Succeeded,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
}
#[doc = "Certificate used for https"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Certificate {
    #[doc = "The type of the secret resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<SecretType>,
    #[doc = "Subject name in the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Certificate expiration date."]
    #[serde(rename = "expirationDate", default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
}
impl Certificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input of CheckNameAvailability API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckEndpointNameAvailabilityInput {
    #[doc = "The resource name to validate."]
    pub name: String,
    #[doc = "Type of CDN resource used in CheckNameAvailability."]
    #[serde(rename = "type")]
    pub type_: ResourceType,
    #[doc = "Indicates the endpoint name reuse scope. The default value is TenantReuse."]
    #[serde(rename = "autoGeneratedDomainNameLabelScope", default, skip_serializing_if = "Option::is_none")]
    pub auto_generated_domain_name_label_scope: Option<AutoGeneratedDomainNameLabelScope>,
}
impl CheckEndpointNameAvailabilityInput {
    pub fn new(name: String, type_: ResourceType) -> Self {
        Self {
            name,
            type_,
            auto_generated_domain_name_label_scope: None,
        }
    }
}
#[doc = "Output of check name availability API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckEndpointNameAvailabilityOutput {
    #[doc = "Indicates whether the name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Returns the available hostname generated based on the AutoGeneratedDomainNameLabelScope when the name is available, otherwise it returns empty string"]
    #[serde(rename = "availableHostname", default, skip_serializing_if = "Option::is_none")]
    pub available_hostname: Option<String>,
    #[doc = "The reason why the name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The detailed error message describing why the name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckEndpointNameAvailabilityOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input of CheckHostNameAvailability API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckHostNameAvailabilityInput {
    #[doc = "The host name to validate."]
    #[serde(rename = "hostName")]
    pub host_name: String,
}
impl CheckHostNameAvailabilityInput {
    pub fn new(host_name: String) -> Self {
        Self { host_name }
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
#[doc = "Defines the parameters for ClientPort match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientPortMatchConditionParameters {
    #[serde(rename = "typeName")]
    pub type_name: client_port_match_condition_parameters::TypeName,
    #[doc = "Describes operator to be matched"]
    pub operator: client_port_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl ClientPortMatchConditionParameters {
    pub fn new(
        type_name: client_port_match_condition_parameters::TypeName,
        operator: client_port_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod client_port_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        DeliveryRuleClientPortConditionParameters,
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
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::RegEx => serializer.serialize_unit_variant("Operator", 9u32, "RegEx"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "settings for compression."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CompressionSettings {
    #[doc = "List of content types on which compression applies. The value should be a valid MIME type."]
    #[serde(
        rename = "contentTypesToCompress",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub content_types_to_compress: Vec<String>,
    #[doc = "Indicates whether content compression is enabled on AzureFrontDoor. Default value is false. If compression is enabled, content will be served as compressed if user requests for a compressed version. Content won't be compressed on AzureFrontDoor when requested content is smaller than 1 byte or larger than 1 MB."]
    #[serde(rename = "isCompressionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_compression_enabled: Option<bool>,
}
impl CompressionSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Continents Response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContinentsResponse {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub continents: Vec<serde_json::Value>,
    #[serde(
        rename = "countryOrRegions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub country_or_regions: Vec<serde_json::Value>,
}
impl ContinentsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the parameters for Cookies match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CookiesMatchConditionParameters {
    #[serde(rename = "typeName")]
    pub type_name: cookies_match_condition_parameters::TypeName,
    #[doc = "Name of Cookies to be matched"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[doc = "Describes operator to be matched"]
    pub operator: cookies_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl CookiesMatchConditionParameters {
    pub fn new(type_name: cookies_match_condition_parameters::TypeName, operator: cookies_match_condition_parameters::Operator) -> Self {
        Self {
            type_name,
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
    pub enum TypeName {
        DeliveryRuleCookiesConditionParameters,
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
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::RegEx => serializer.serialize_unit_variant("Operator", 9u32, "RegEx"),
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "certificateSource")]
pub enum CustomDomainHttpsParametersUnion {
    Cdn(CdnManagedHttpsParameters),
    AzureKeyVault(UserManagedHttpsParameters),
}
#[doc = "Result of the request to list custom domains. It contains a list of custom domain objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomDomainListResult {
    #[doc = "List of CDN CustomDomains within an endpoint."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CustomDomain>,
    #[doc = "URL to get the next set of custom domain objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CustomDomainListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
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
    #[doc = "Provisioning status of the custom domain."]
    #[serde(rename = "customHttpsProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub custom_https_provisioning_state: Option<custom_domain_properties::CustomHttpsProvisioningState>,
    #[doc = "Provisioning substate shows the progress of custom HTTPS enabling/disabling process step by step."]
    #[serde(rename = "customHttpsProvisioningSubstate", default, skip_serializing_if = "Option::is_none")]
    pub custom_https_provisioning_substate: Option<custom_domain_properties::CustomHttpsProvisioningSubstate>,
    #[doc = "The JSON object that contains the properties to secure a custom domain."]
    #[serde(rename = "customHttpsParameters", default, skip_serializing_if = "Option::is_none")]
    pub custom_https_parameters: Option<CustomDomainHttpsParametersUnion>,
    #[doc = "Special validation or data may be required when delivering CDN to some regions due to local compliance reasons. E.g. ICP license number of a custom domain is required to deliver content in China."]
    #[serde(rename = "validationData", default, skip_serializing_if = "Option::is_none")]
    pub validation_data: Option<String>,
    #[doc = "Provisioning status of Custom Https of the custom domain."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<custom_domain_properties::ProvisioningState>,
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
    #[doc = "Provisioning status of the custom domain."]
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
    #[doc = "Provisioning status of Custom Https of the custom domain."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Enabling,
        Enabled,
        Disabling,
        Disabled,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabling => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Enabling"),
                Self::Enabled => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Enabled"),
                Self::Disabling => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Disabling"),
                Self::Disabled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Disabled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
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
#[doc = "Defines the common attributes for a custom rule that can be included in a waf policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomRule {
    #[doc = "Defines the name of the custom rule"]
    pub name: String,
    #[doc = "Describes if the custom rule is in enabled or disabled state. Defaults to Enabled if not specified."]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<custom_rule::EnabledState>,
    #[doc = "Defines in what order this rule be evaluated in the overall list of custom rules"]
    pub priority: i32,
    #[doc = "List of match conditions."]
    #[serde(rename = "matchConditions")]
    pub match_conditions: Vec<MatchCondition>,
    #[doc = "Defines the action to take on rule match."]
    pub action: ActionType,
}
impl CustomRule {
    pub fn new(name: String, priority: i32, match_conditions: Vec<MatchCondition>, action: ActionType) -> Self {
        Self {
            name,
            enabled_state: None,
            priority,
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
}
#[doc = "Defines contents of custom rules"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomRuleList {
    #[doc = "List of rules"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rules: Vec<CustomRule>,
}
impl CustomRuleList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Customer Certificate used for https"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomerCertificate {
    #[serde(flatten)]
    pub certificate: Certificate,
    #[doc = "Reference to another resource."]
    #[serde(rename = "secretSource", default, skip_serializing_if = "Option::is_none")]
    pub secret_source: Option<ResourceReference>,
    #[doc = "Certificate version."]
    #[serde(rename = "secretVersion", default, skip_serializing_if = "Option::is_none")]
    pub secret_version: Option<String>,
    #[doc = "Certificate issuing authority."]
    #[serde(rename = "certificateAuthority", default, skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<String>,
    #[doc = "Whether to use the latest version for the certificate"]
    #[serde(rename = "useLatestVersion", default, skip_serializing_if = "Option::is_none")]
    pub use_latest_version: Option<bool>,
    #[doc = "The list of SANs."]
    #[serde(
        rename = "subjectAlternativeNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subject_alternative_names: Vec<String>,
    #[doc = "Certificate thumbprint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}
impl CustomerCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Customer Certificate used for https"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerCertificateParameters {
    #[serde(flatten)]
    pub secret_parameters: SecretParameters,
    #[doc = "Reference to another resource."]
    #[serde(rename = "secretSource")]
    pub secret_source: ResourceReference,
    #[doc = "Version of the secret to be used"]
    #[serde(rename = "secretVersion", default, skip_serializing_if = "Option::is_none")]
    pub secret_version: Option<String>,
    #[doc = "Whether to use the latest version for the certificate"]
    #[serde(rename = "useLatestVersion", default, skip_serializing_if = "Option::is_none")]
    pub use_latest_version: Option<bool>,
    #[doc = "Subject name in the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Certificate expiration date."]
    #[serde(rename = "expirationDate", default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[doc = "Certificate issuing authority."]
    #[serde(rename = "certificateAuthority", default, skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<String>,
    #[doc = "The list of SANs."]
    #[serde(
        rename = "subjectAlternativeNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subject_alternative_names: Vec<String>,
    #[doc = "Certificate thumbprint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}
impl CustomerCertificateParameters {
    pub fn new(secret_parameters: SecretParameters, secret_source: ResourceReference) -> Self {
        Self {
            secret_parameters,
            secret_source,
            secret_version: None,
            use_latest_version: None,
            subject: None,
            expiration_date: None,
            certificate_authority: None,
            subject_alternative_names: Vec::new(),
            thumbprint: None,
        }
    }
}
#[doc = "Custom domains created on the CDN endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeepCreatedCustomDomain {
    #[doc = "Custom domain name."]
    pub name: String,
    #[doc = "Properties of the custom domain created on the CDN endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeepCreatedCustomDomainProperties>,
}
impl DeepCreatedCustomDomain {
    pub fn new(name: String) -> Self {
        Self { name, properties: None }
    }
}
#[doc = "Properties of the custom domain created on the CDN endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeepCreatedCustomDomainProperties {
    #[doc = "The host name of the custom domain. Must be a domain name."]
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[doc = "Special validation or data may be required when delivering CDN to some regions due to local compliance reasons. E.g. ICP license number of a custom domain is required to deliver content in China."]
    #[serde(rename = "validationData", default, skip_serializing_if = "Option::is_none")]
    pub validation_data: Option<String>,
}
impl DeepCreatedCustomDomainProperties {
    pub fn new(host_name: String) -> Self {
        Self {
            host_name,
            validation_data: None,
        }
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
    pub traffic_restoration_time_to_healed_or_new_endpoints_in_minutes: Option<i32>,
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
    pub http_port: Option<i32>,
    #[doc = "The value of the HTTPS port. Must be between 1 and 65535."]
    #[serde(rename = "httpsPort", default, skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i32>,
    #[doc = "The host header value sent to the origin with each request. If you leave this blank, the request hostname determines this value. Azure CDN origins, such as Web Apps, Blob Storage, and Cloud Services require this host header value to match the origin hostname by default."]
    #[serde(rename = "originHostHeader", default, skip_serializing_if = "Option::is_none")]
    pub origin_host_header: Option<String>,
    #[doc = "Priority of origin in given origin group for load balancing. Higher priorities will not be used for load balancing if any lower priority origin is healthy.Must be between 1 and 5."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "Weight of the origin in given origin group for load balancing. Must be between 1 and 1000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[doc = "Origin is enabled for load balancing or not. By default, origin is always enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The Alias of the Private Link resource. Populating this optional field indicates that this origin is 'Private'"]
    #[serde(rename = "privateLinkAlias", default, skip_serializing_if = "Option::is_none")]
    pub private_link_alias: Option<String>,
    #[doc = "The Resource Id of the Private Link resource. Populating this optional field indicates that this backend is 'Private'"]
    #[serde(rename = "privateLinkResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_resource_id: Option<String>,
    #[doc = "The location of the Private Link resource. Required only if 'privateLinkResourceId' is populated"]
    #[serde(rename = "privateLinkLocation", default, skip_serializing_if = "Option::is_none")]
    pub private_link_location: Option<String>,
    #[doc = "A custom message to be included in the approval request to connect to the Private Link."]
    #[serde(rename = "privateLinkApprovalMessage", default, skip_serializing_if = "Option::is_none")]
    pub private_link_approval_message: Option<String>,
    #[doc = "The approval status for the connection to the Private Link"]
    #[serde(rename = "privateEndpointStatus", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint_status: Option<PrivateEndpointStatus>,
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
            private_link_alias: None,
            private_link_resource_id: None,
            private_link_location: None,
            private_link_approval_message: None,
            private_endpoint_status: None,
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
    pub order: i32,
    #[doc = "A list of conditions that must be matched for the actions to be executed"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions: Vec<DeliveryRuleConditionUnion>,
    #[doc = "A list of actions that are executed when all the conditions of a rule are satisfied."]
    pub actions: Vec<DeliveryRuleActionUnion>,
}
impl DeliveryRule {
    pub fn new(order: i32, actions: Vec<DeliveryRuleActionUnion>) -> Self {
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
        UrlSigning,
        OriginGroupOverride,
        RouteConfigurationOverride,
        AfdUrlSigning,
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
                Self::UrlSigning => serializer.serialize_unit_variant("Name", 6u32, "UrlSigning"),
                Self::OriginGroupOverride => serializer.serialize_unit_variant("Name", 7u32, "OriginGroupOverride"),
                Self::RouteConfigurationOverride => serializer.serialize_unit_variant("Name", 8u32, "RouteConfigurationOverride"),
                Self::AfdUrlSigning => serializer.serialize_unit_variant("Name", 9u32, "AfdUrlSigning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum DeliveryRuleActionUnion {
    AfdUrlSigning(AfdUrlSigningAction),
    CacheExpiration(DeliveryRuleCacheExpirationAction),
    CacheKeyQueryString(DeliveryRuleCacheKeyQueryStringAction),
    ModifyRequestHeader(DeliveryRuleRequestHeaderAction),
    ModifyResponseHeader(DeliveryRuleResponseHeaderAction),
    RouteConfigurationOverride(DeliveryRuleRouteConfigurationOverrideAction),
    OriginGroupOverride(OriginGroupOverrideAction),
    UrlRedirect(UrlRedirectAction),
    UrlRewrite(UrlRewriteAction),
    UrlSigning(UrlSigningAction),
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
#[doc = "Defines the ClientPort condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleClientPortCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for ClientPort match conditions"]
    pub parameters: ClientPortMatchConditionParameters,
}
impl DeliveryRuleClientPortCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: ClientPortMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
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
        SocketAddr,
        ClientPort,
        ServerPort,
        HostName,
        SslProtocol,
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
                Self::SocketAddr => serializer.serialize_unit_variant("Name", 14u32, "SocketAddr"),
                Self::ClientPort => serializer.serialize_unit_variant("Name", 15u32, "ClientPort"),
                Self::ServerPort => serializer.serialize_unit_variant("Name", 16u32, "ServerPort"),
                Self::HostName => serializer.serialize_unit_variant("Name", 17u32, "HostName"),
                Self::SslProtocol => serializer.serialize_unit_variant("Name", 18u32, "SslProtocol"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum DeliveryRuleConditionUnion {
    ClientPort(DeliveryRuleClientPortCondition),
    Cookies(DeliveryRuleCookiesCondition),
    HostName(DeliveryRuleHostNameCondition),
    HttpVersion(DeliveryRuleHttpVersionCondition),
    IsDevice(DeliveryRuleIsDeviceCondition),
    PostArgs(DeliveryRulePostArgsCondition),
    QueryString(DeliveryRuleQueryStringCondition),
    RemoteAddress(DeliveryRuleRemoteAddressCondition),
    RequestBody(DeliveryRuleRequestBodyCondition),
    RequestHeader(DeliveryRuleRequestHeaderCondition),
    RequestMethod(DeliveryRuleRequestMethodCondition),
    RequestScheme(DeliveryRuleRequestSchemeCondition),
    RequestUri(DeliveryRuleRequestUriCondition),
    ServerPort(DeliveryRuleServerPortCondition),
    SocketAddr(DeliveryRuleSocketAddrCondition),
    SslProtocol(DeliveryRuleSslProtocolCondition),
    UrlFileExtension(DeliveryRuleUrlFileExtensionCondition),
    UrlFileName(DeliveryRuleUrlFileNameCondition),
    UrlPath(DeliveryRuleUrlPathCondition),
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
#[doc = "Defines the HostName condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleHostNameCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for HostName match conditions"]
    pub parameters: HostNameMatchConditionParameters,
}
impl DeliveryRuleHostNameCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: HostNameMatchConditionParameters) -> Self {
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
#[doc = "Defines the route configuration override action for the delivery rule. Only applicable to Frontdoor Standard/Premium Profiles."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleRouteConfigurationOverrideAction {
    #[serde(flatten)]
    pub delivery_rule_action: DeliveryRuleAction,
    #[doc = "Defines the parameters for the route configuration override action."]
    pub parameters: RouteConfigurationOverrideActionParameters,
}
impl DeliveryRuleRouteConfigurationOverrideAction {
    pub fn new(delivery_rule_action: DeliveryRuleAction, parameters: RouteConfigurationOverrideActionParameters) -> Self {
        Self {
            delivery_rule_action,
            parameters,
        }
    }
}
#[doc = "Defines the ServerPort condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleServerPortCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for ServerPort match conditions"]
    pub parameters: ServerPortMatchConditionParameters,
}
impl DeliveryRuleServerPortCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: ServerPortMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[doc = "Defines the SocketAddress condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleSocketAddrCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for SocketAddress match conditions"]
    pub parameters: SocketAddrMatchConditionParameters,
}
impl DeliveryRuleSocketAddrCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: SocketAddrMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[doc = "Defines the SslProtocol condition for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleSslProtocolCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    #[doc = "Defines the parameters for SslProtocol match conditions"]
    pub parameters: SslProtocolMatchConditionParameters,
}
impl DeliveryRuleSslProtocolCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: SslProtocolMatchConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
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
#[doc = "Type of operation: get, read, delete, etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DimensionProperties {
    #[doc = "Name of dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of dimension."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Internal name of dimension."]
    #[serde(rename = "internalName", default, skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
}
impl DimensionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties to validate a domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainValidationProperties {
    #[doc = "Challenge used for DNS TXT record or file based validation"]
    #[serde(rename = "validationToken", default, skip_serializing_if = "Option::is_none")]
    pub validation_token: Option<String>,
    #[doc = "The date time that the token expires"]
    #[serde(rename = "expirationDate", default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
}
impl DomainValidationProperties {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EdgeNode>,
    #[doc = "URL to get the next set of edgenode list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EdgenodeResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Endpoint>,
    #[doc = "URL to get the next set of endpoint objects if there is any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EndpointListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
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
    #[serde(
        rename = "originGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub origin_groups: Vec<DeepCreatedOriginGroup>,
    #[doc = "The custom domains under the endpoint."]
    #[serde(
        rename = "customDomains",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_domains: Vec<DeepCreatedCustomDomain>,
    #[doc = "Resource status of the endpoint."]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<endpoint_properties::ResourceState>,
    #[doc = "Provisioning status of the endpoint."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<endpoint_properties::ProvisioningState>,
}
impl EndpointProperties {
    pub fn new(origins: Vec<DeepCreatedOrigin>) -> Self {
        Self {
            endpoint_properties_update_parameters: EndpointPropertiesUpdateParameters::default(),
            host_name: None,
            origins,
            origin_groups: Vec::new(),
            custom_domains: Vec::new(),
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
    #[doc = "Provisioning status of the endpoint."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Updating,
        Deleting,
        Creating,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
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
    #[serde(
        rename = "contentTypesToCompress",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub content_types_to_compress: Vec<String>,
    #[doc = "The host header value sent to the origin with each request. This property at Endpoint is only allowed when endpoint uses single origin and can be overridden by the same property specified at origin.If you leave this blank, the request hostname determines this value. Azure CDN origins, such as Web Apps, Blob Storage, and Cloud Services require this host header value to match the origin hostname by default."]
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
    #[serde(
        rename = "geoFilters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub geo_filters: Vec<GeoFilter>,
    #[doc = "Reference to another resource."]
    #[serde(rename = "defaultOriginGroup", default, skip_serializing_if = "Option::is_none")]
    pub default_origin_group: Option<ResourceReference>,
    #[doc = "List of keys used to validate the signed URL hashes."]
    #[serde(
        rename = "urlSigningKeys",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub url_signing_keys: Vec<UrlSigningKey>,
    #[doc = "A policy that specifies the delivery rules to be used for an endpoint."]
    #[serde(rename = "deliveryPolicy", default, skip_serializing_if = "Option::is_none")]
    pub delivery_policy: Option<endpoint_properties_update_parameters::DeliveryPolicy>,
    #[doc = "Defines the Web Application Firewall policy for the endpoint (if applicable)"]
    #[serde(rename = "webApplicationFirewallPolicyLink", default, skip_serializing_if = "Option::is_none")]
    pub web_application_firewall_policy_link: Option<endpoint_properties_update_parameters::WebApplicationFirewallPolicyLink>,
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
    #[doc = "Defines the Web Application Firewall policy for the endpoint (if applicable)"]
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
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates Azure Front Door Standard or Azure Front Door Premium or CDN service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
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
    #[doc = "Two letter country or region codes defining user country or region access in a geo filter, e.g. AU, MX, US."]
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
    #[serde(rename = "typeName")]
    pub type_name: header_action_parameters::TypeName,
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
        type_name: header_action_parameters::TypeName,
        header_action: header_action_parameters::HeaderAction,
        header_name: String,
    ) -> Self {
        Self {
            type_name,
            header_action,
            header_name,
            value: None,
        }
    }
}
pub mod header_action_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        DeliveryRuleHeaderActionParameters,
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
    pub probe_interval_in_seconds: Option<i32>,
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
#[doc = "Defines the parameters for HostName match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostNameMatchConditionParameters {
    #[serde(rename = "typeName")]
    pub type_name: host_name_match_condition_parameters::TypeName,
    #[doc = "Describes operator to be matched"]
    pub operator: host_name_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl HostNameMatchConditionParameters {
    pub fn new(
        type_name: host_name_match_condition_parameters::TypeName,
        operator: host_name_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod host_name_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        DeliveryRuleHostNameConditionParameters,
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
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::RegEx => serializer.serialize_unit_variant("Operator", 9u32, "RegEx"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The JSON object that represents the range for http status codes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpErrorRangeParameters {
    #[doc = "The inclusive start of the http status code range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub begin: Option<i32>,
    #[doc = "The inclusive end of the http status code range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
}
impl HttpErrorRangeParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the parameters for HttpVersion match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpVersionMatchConditionParameters {
    #[serde(rename = "typeName")]
    pub type_name: http_version_match_condition_parameters::TypeName,
    #[doc = "Describes operator to be matched"]
    pub operator: http_version_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl HttpVersionMatchConditionParameters {
    pub fn new(
        type_name: http_version_match_condition_parameters::TypeName,
        operator: http_version_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod http_version_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        DeliveryRuleHttpVersionConditionParameters,
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
#[doc = "The type of identity that creates/modifies resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IdentityType")]
pub enum IdentityType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "application")]
    Application,
    #[serde(rename = "managedIdentity")]
    ManagedIdentity,
    #[serde(rename = "key")]
    Key,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::User => serializer.serialize_unit_variant("IdentityType", 0u32, "user"),
            Self::Application => serializer.serialize_unit_variant("IdentityType", 1u32, "application"),
            Self::ManagedIdentity => serializer.serialize_unit_variant("IdentityType", 2u32, "managedIdentity"),
            Self::Key => serializer.serialize_unit_variant("IdentityType", 3u32, "key"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
    #[serde(
        rename = "ipv4Addresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ipv4_addresses: Vec<CidrIpAddress>,
    #[doc = "The list of ip v6 addresses."]
    #[serde(
        rename = "ipv6Addresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[serde(rename = "typeName")]
    pub type_name: is_device_match_condition_parameters::TypeName,
    #[doc = "Describes operator to be matched"]
    pub operator: is_device_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl IsDeviceMatchConditionParameters {
    pub fn new(
        type_name: is_device_match_condition_parameters::TypeName,
        operator: is_device_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
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
    pub enum TypeName {
        DeliveryRuleIsDeviceConditionParameters,
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
#[doc = "Contains a list of references of UrlSigningKey type secret objects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The JSON object that contains the properties of the key group to create."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<KeyGroupProperties>,
}
impl KeyGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list keyGroups. It contains a list of KeyGroup objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyGroupListResult {
    #[doc = "List of AzureFrontDoor KeyGroup within a profile."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<KeyGroup>,
    #[doc = "URL to get the next set of KeyGroup objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for KeyGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl KeyGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties of the key group to create."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyGroupProperties {
    #[serde(flatten)]
    pub key_group_update_properties_parameters: KeyGroupUpdatePropertiesParameters,
    #[serde(flatten)]
    pub afd_state_properties: AfdStateProperties,
}
impl KeyGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object containing properties of key group to create or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyGroupUpdatePropertiesParameters {
    #[doc = "Names of UrlSigningKey type secret objects"]
    #[serde(
        rename = "keyReferences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub key_references: Vec<ResourceReference>,
}
impl KeyGroupUpdatePropertiesParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the parameters for using a user's KeyVault certificate for securing custom domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultCertificateSourceParameters {
    #[serde(rename = "typeName")]
    pub type_name: key_vault_certificate_source_parameters::TypeName,
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
        type_name: key_vault_certificate_source_parameters::TypeName,
        subscription_id: String,
        resource_group_name: String,
        vault_name: String,
        secret_name: String,
        update_rule: key_vault_certificate_source_parameters::UpdateRule,
        delete_rule: key_vault_certificate_source_parameters::DeleteRule,
    ) -> Self {
        Self {
            type_name,
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
    pub enum TypeName {
        KeyVaultCertificateSourceParameters,
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
#[doc = "Describes the parameters for using a user's KeyVault for URL Signing Key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultSigningKeyParameters {
    #[serde(rename = "typeName")]
    pub type_name: key_vault_signing_key_parameters::TypeName,
    #[doc = "Subscription Id of the user's Key Vault containing the secret"]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
    #[doc = "Resource group of the user's Key Vault containing the secret"]
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
    #[doc = "The name of the user's Key Vault containing the secret"]
    #[serde(rename = "vaultName")]
    pub vault_name: String,
    #[doc = "The name of secret in Key Vault."]
    #[serde(rename = "secretName")]
    pub secret_name: String,
    #[doc = "The version(GUID) of secret in Key Vault."]
    #[serde(rename = "secretVersion")]
    pub secret_version: String,
}
impl KeyVaultSigningKeyParameters {
    pub fn new(
        type_name: key_vault_signing_key_parameters::TypeName,
        subscription_id: String,
        resource_group_name: String,
        vault_name: String,
        secret_name: String,
        secret_version: String,
    ) -> Self {
        Self {
            type_name,
            subscription_id,
            resource_group_name,
            vault_name,
            secret_name,
            secret_version,
        }
    }
}
pub mod key_vault_signing_key_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        KeyVaultSigningKeyParameters,
    }
}
#[doc = "Round-Robin load balancing settings for a backend pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancingSettingsParameters {
    #[doc = "The number of samples to consider for load balancing decisions"]
    #[serde(rename = "sampleSize", default, skip_serializing_if = "Option::is_none")]
    pub sample_size: Option<i32>,
    #[doc = "The number of samples within the sample period that must succeed"]
    #[serde(rename = "successfulSamplesRequired", default, skip_serializing_if = "Option::is_none")]
    pub successful_samples_required: Option<i32>,
    #[doc = "The additional latency in milliseconds for probes to fall into the lowest latency bucket"]
    #[serde(rename = "additionalLatencyInMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub additional_latency_in_milliseconds: Option<i32>,
}
impl LoadBalancingSettingsParameters {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Log specification of operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[doc = "Name of log specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of log specification."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Blob duration of specification."]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
    #[doc = "Pattern to filter based on name"]
    #[serde(rename = "logFilterPattern", default, skip_serializing_if = "Option::is_none")]
    pub log_filter_pattern: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed Certificate used for https"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedCertificate {
    #[serde(flatten)]
    pub certificate: Certificate,
}
impl ManagedCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed Certificate used for https"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedCertificateParameters {
    #[serde(flatten)]
    pub secret_parameters: SecretParameters,
    #[doc = "Subject name in the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Certificate expiration date."]
    #[serde(rename = "expirationDate", default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
}
impl ManagedCertificateParameters {
    pub fn new(secret_parameters: SecretParameters) -> Self {
        Self {
            secret_parameters,
            subject: None,
            expiration_date: None,
        }
    }
}
#[doc = "Describes a managed rule definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedRuleDefinition {
    #[doc = "Identifier for the managed rule."]
    #[serde(rename = "ruleId", default, skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[doc = "Describes the functionality of the managed rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ManagedRuleDefinition {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[doc = "Describes the managed rule group within the rule set to override"]
    #[serde(rename = "ruleGroupName")]
    pub rule_group_name: String,
    #[doc = "List of rules that will be enabled. If none specified, all rules in the group will be disabled."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rules: Vec<ManagedRuleOverride>,
}
impl ManagedRuleGroupOverride {
    pub fn new(rule_group_name: String) -> Self {
        Self {
            rule_group_name,
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
    #[doc = "Describes if the managed rule is in enabled or disabled state. Defaults to Disabled if not specified."]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<managed_rule_override::EnabledState>,
    #[doc = "Defines the action to take on rule match."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionType>,
}
impl ManagedRuleOverride {
    pub fn new(rule_id: String) -> Self {
        Self {
            rule_id,
            enabled_state: None,
            action: None,
        }
    }
}
pub mod managed_rule_override {
    use super::*;
    #[doc = "Describes if the managed rule is in enabled or disabled state. Defaults to Disabled if not specified."]
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
    #[doc = "Verizon only : If the rule set supports anomaly detection mode, this describes the threshold for blocking requests."]
    #[serde(rename = "anomalyScore", default, skip_serializing_if = "Option::is_none")]
    pub anomaly_score: Option<i32>,
    #[doc = "Defines the rule overrides to apply to the rule set."]
    #[serde(
        rename = "ruleGroupOverrides",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rule_group_overrides: Vec<ManagedRuleGroupOverride>,
}
impl ManagedRuleSet {
    pub fn new(rule_set_type: String, rule_set_version: String) -> Self {
        Self {
            rule_set_type,
            rule_set_version,
            anomaly_score: None,
            rule_group_overrides: Vec::new(),
        }
    }
}
#[doc = "Describes a managed rule set definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedRuleSetDefinition {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties for a managed rule set definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedRuleSetDefinitionProperties>,
    #[doc = "Standard_Verizon = The SKU name for a Standard Verizon CDN profile.\nPremium_Verizon = The SKU name for a Premium Verizon CDN profile.\nCustom_Verizon = The SKU name for a Custom Verizon CDN profile.\nStandard_Akamai = The SKU name for an Akamai CDN profile.\nStandard_ChinaCdn = The SKU name for a China CDN profile for VOD, Web and download scenarios using GB based billing model.\nStandard_Microsoft = The SKU name for a Standard Microsoft CDN profile.\nStandard_AzureFrontDoor =  The SKU name for an Azure Front Door Standard profile.\nPremium_AzureFrontDoor = The SKU name for an Azure Front Door Premium profile.\nStandard_955BandWidth_ChinaCdn = The SKU name for a China CDN profile for VOD, Web and download scenarios using 95-5 peak bandwidth billing model.\nStandard_AvgBandWidth_ChinaCdn = The SKU name for a China CDN profile for VOD, Web and download scenarios using monthly average peak bandwidth billing model.\nStandardPlus_ChinaCdn = The SKU name for a China CDN profile for live-streaming using GB based billing model.\nStandardPlus_955BandWidth_ChinaCdn = The SKU name for a China CDN live-streaming profile using 95-5 peak bandwidth billing model.\nStandardPlus_AvgBandWidth_ChinaCdn = The SKU name for a China CDN live-streaming profile using monthly average peak bandwidth billing model.\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedRuleSetDefinition>,
    #[doc = "URL to retrieve next set of managed rule set definitions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedRuleSetDefinitionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
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
    #[doc = "Type of the managed rule set."]
    #[serde(rename = "ruleSetType", default, skip_serializing_if = "Option::is_none")]
    pub rule_set_type: Option<String>,
    #[doc = "Version of the managed rule set type."]
    #[serde(rename = "ruleSetVersion", default, skip_serializing_if = "Option::is_none")]
    pub rule_set_version: Option<String>,
    #[doc = "Rule groups of the managed rule set."]
    #[serde(
        rename = "ruleGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[serde(
        rename = "managedRuleSets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub managed_rule_sets: Vec<ManagedRuleSet>,
}
impl ManagedRuleSetList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: ManagedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedServiceIdentityType")]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned, UserAssigned")]
    SystemAssignedUserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 2u32, "UserAssigned"),
            Self::SystemAssignedUserAssigned => {
                serializer.serialize_unit_variant("ManagedServiceIdentityType", 3u32, "SystemAssigned, UserAssigned")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Define match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchCondition {
    #[doc = "Match variable to compare against."]
    #[serde(rename = "matchVariable")]
    pub match_variable: match_condition::MatchVariable,
    #[doc = "Selector can used to match a specific key for QueryString, Cookies, RequestHeader or PostArgs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[doc = "Describes operator to be matched"]
    pub operator: match_condition::Operator,
    #[doc = "Describes if the result of this condition should be negated."]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "List of possible match values."]
    #[serde(rename = "matchValue")]
    pub match_value: Vec<String>,
    #[doc = "List of transforms."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[doc = "Match variable to compare against."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MatchVariable")]
    pub enum MatchVariable {
        RemoteAddr,
        SocketAddr,
        RequestMethod,
        RequestHeader,
        RequestUri,
        QueryString,
        RequestBody,
        Cookies,
        PostArgs,
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
                Self::SocketAddr => serializer.serialize_unit_variant("MatchVariable", 1u32, "SocketAddr"),
                Self::RequestMethod => serializer.serialize_unit_variant("MatchVariable", 2u32, "RequestMethod"),
                Self::RequestHeader => serializer.serialize_unit_variant("MatchVariable", 3u32, "RequestHeader"),
                Self::RequestUri => serializer.serialize_unit_variant("MatchVariable", 4u32, "RequestUri"),
                Self::QueryString => serializer.serialize_unit_variant("MatchVariable", 5u32, "QueryString"),
                Self::RequestBody => serializer.serialize_unit_variant("MatchVariable", 6u32, "RequestBody"),
                Self::Cookies => serializer.serialize_unit_variant("MatchVariable", 7u32, "Cookies"),
                Self::PostArgs => serializer.serialize_unit_variant("MatchVariable", 8u32, "PostArgs"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Describes operator to be matched"]
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
#[doc = "Retention policy of a resource metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAvailability {
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl MetricAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metric specification of operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[doc = "Name of metric specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of metric specification."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Display description of metric specification."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "The metric unit. Possible values include: 'Bytes', 'Count', 'Milliseconds'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The metric aggregation type. Possible values include: 'Average', 'Count', 'Total'."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "Retention policies of a resource metric."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub availabilities: Vec<MetricAvailability>,
    #[doc = "The supported time grain types for the metrics."]
    #[serde(
        rename = "supportedTimeGrainTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_time_grain_types: Vec<String>,
    #[doc = "The dimensions of metric"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<DimensionProperties>,
    #[doc = "Property to specify whether to fill gap with zero."]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[doc = "Pattern to filter based on name"]
    #[serde(rename = "metricFilterPattern", default, skip_serializing_if = "Option::is_none")]
    pub metric_filter_pattern: Option<String>,
    #[doc = "Property to specify metric is internal or not."]
    #[serde(rename = "isInternal", default, skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metrics Response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricsResponse {
    #[serde(rename = "dateTimeBegin", default, with = "azure_core::date::rfc3339::option")]
    pub date_time_begin: Option<time::OffsetDateTime>,
    #[serde(rename = "dateTimeEnd", default, with = "azure_core::date::rfc3339::option")]
    pub date_time_end: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<metrics_response::Granularity>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub series: Vec<serde_json::Value>,
}
impl MetricsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod metrics_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Granularity")]
    pub enum Granularity {
        #[serde(rename = "PT5M")]
        Pt5m,
        #[serde(rename = "PT1H")]
        Pt1h,
        #[serde(rename = "P1D")]
        P1d,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Granularity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Granularity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Granularity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pt5m => serializer.serialize_unit_variant("Granularity", 0u32, "PT5M"),
                Self::Pt1h => serializer.serialize_unit_variant("Granularity", 1u32, "PT1H"),
                Self::P1d => serializer.serialize_unit_variant("Granularity", 2u32, "P1D"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result for migrate operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateResult {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MigrateResultProperties>,
}
impl MigrateResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateResultProperties {
    #[doc = "Reference to another resource."]
    #[serde(rename = "migratedProfileResourceId", default, skip_serializing_if = "Option::is_none")]
    pub migrated_profile_resource_id: Option<ResourceReference>,
}
impl MigrateResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates CDN service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationErrorType {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Resource which has the problem."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Describes what needs to be done to fix the problem"]
    #[serde(rename = "nextSteps", default, skip_serializing_if = "Option::is_none")]
    pub next_steps: Option<String>,
}
impl MigrationErrorType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request body for Migrate operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrationParameters {
    #[doc = "Standard_Verizon = The SKU name for a Standard Verizon CDN profile.\nPremium_Verizon = The SKU name for a Premium Verizon CDN profile.\nCustom_Verizon = The SKU name for a Custom Verizon CDN profile.\nStandard_Akamai = The SKU name for an Akamai CDN profile.\nStandard_ChinaCdn = The SKU name for a China CDN profile for VOD, Web and download scenarios using GB based billing model.\nStandard_Microsoft = The SKU name for a Standard Microsoft CDN profile.\nStandard_AzureFrontDoor =  The SKU name for an Azure Front Door Standard profile.\nPremium_AzureFrontDoor = The SKU name for an Azure Front Door Premium profile.\nStandard_955BandWidth_ChinaCdn = The SKU name for a China CDN profile for VOD, Web and download scenarios using 95-5 peak bandwidth billing model.\nStandard_AvgBandWidth_ChinaCdn = The SKU name for a China CDN profile for VOD, Web and download scenarios using monthly average peak bandwidth billing model.\nStandardPlus_ChinaCdn = The SKU name for a China CDN profile for live-streaming using GB based billing model.\nStandardPlus_955BandWidth_ChinaCdn = The SKU name for a China CDN live-streaming profile using 95-5 peak bandwidth billing model.\nStandardPlus_AvgBandWidth_ChinaCdn = The SKU name for a China CDN live-streaming profile using monthly average peak bandwidth billing model.\n"]
    pub sku: Sku,
    #[doc = "Reference to another resource."]
    #[serde(rename = "classicResourceReference")]
    pub classic_resource_reference: ResourceReference,
    #[doc = "Name of the new profile that need to be created."]
    #[serde(rename = "profileName")]
    pub profile_name: String,
    #[doc = "Waf mapping for the migrated profile"]
    #[serde(
        rename = "migrationWebApplicationFirewallMappings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub migration_web_application_firewall_mappings: Vec<MigrationWebApplicationFirewallMapping>,
}
impl MigrationParameters {
    pub fn new(sku: Sku, classic_resource_reference: ResourceReference, profile_name: String) -> Self {
        Self {
            sku,
            classic_resource_reference,
            profile_name,
            migration_web_application_firewall_mappings: Vec::new(),
        }
    }
}
#[doc = "Web Application Firewall Mapping "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationWebApplicationFirewallMapping {
    #[doc = "Reference to another resource."]
    #[serde(rename = "migratedFrom", default, skip_serializing_if = "Option::is_none")]
    pub migrated_from: Option<ResourceReference>,
    #[doc = "Reference to another resource."]
    #[serde(rename = "migratedTo", default, skip_serializing_if = "Option::is_none")]
    pub migrated_to: Option<ResourceReference>,
}
impl MigrationWebApplicationFirewallMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CDN REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The origin of operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Properties of operation, include metric specifications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
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
        #[doc = "Description of operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Properties of operation, include metric specifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "One property of operation, include log specifications."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list CDN operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsListResult {
    #[doc = "List of CDN operations supported by the CDN resource provider."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OriginGroup>,
    #[doc = "URL to get the next set of origin objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OriginGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OriginGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the parameters for the origin group override configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OriginGroupOverride {
    #[doc = "Reference to another resource."]
    #[serde(rename = "originGroup", default, skip_serializing_if = "Option::is_none")]
    pub origin_group: Option<ResourceReference>,
    #[doc = "Protocol this rule will use when forwarding traffic to backends."]
    #[serde(rename = "forwardingProtocol", default, skip_serializing_if = "Option::is_none")]
    pub forwarding_protocol: Option<origin_group_override::ForwardingProtocol>,
}
impl OriginGroupOverride {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod origin_group_override {
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
#[doc = "Defines the origin group override action for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OriginGroupOverrideAction {
    #[serde(flatten)]
    pub delivery_rule_action: DeliveryRuleAction,
    #[doc = "Defines the parameters for the origin group override action."]
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
#[doc = "Defines the parameters for the origin group override action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OriginGroupOverrideActionParameters {
    #[serde(rename = "typeName")]
    pub type_name: origin_group_override_action_parameters::TypeName,
    #[doc = "Reference to another resource."]
    #[serde(rename = "originGroup")]
    pub origin_group: ResourceReference,
}
impl OriginGroupOverrideActionParameters {
    pub fn new(type_name: origin_group_override_action_parameters::TypeName, origin_group: ResourceReference) -> Self {
        Self { type_name, origin_group }
    }
}
pub mod origin_group_override_action_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        DeliveryRuleOriginGroupOverrideActionParameters,
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
    pub provisioning_state: Option<origin_group_properties::ProvisioningState>,
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
    #[doc = "Provisioning status of the origin group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Updating,
        Deleting,
        Creating,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub origins: Vec<ResourceReference>,
    #[doc = "Time in minutes to shift the traffic to the endpoint gradually when an unhealthy endpoint comes healthy or a new endpoint is added. Default is 10 mins. This property is currently not supported."]
    #[serde(
        rename = "trafficRestorationTimeToHealedOrNewEndpointsInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_restoration_time_to_healed_or_new_endpoints_in_minutes: Option<i32>,
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Origin>,
    #[doc = "URL to get the next set of origin objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OriginListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
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
    pub provisioning_state: Option<origin_properties::ProvisioningState>,
    #[doc = "The approval status for the connection to the Private Link"]
    #[serde(rename = "privateEndpointStatus", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint_status: Option<PrivateEndpointStatus>,
}
impl OriginProperties {
    pub fn new() -> Self {
        Self {
            origin_update_properties_parameters: OriginUpdatePropertiesParameters::default(),
            resource_state: None,
            provisioning_state: None,
            private_endpoint_status: None,
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
    #[doc = "Provisioning status of the origin."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Updating,
        Deleting,
        Creating,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
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
    pub http_port: Option<i32>,
    #[doc = "The value of the HTTPS port. Must be between 1 and 65535."]
    #[serde(rename = "httpsPort", default, skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i32>,
    #[doc = "The host header value sent to the origin with each request. If you leave this blank, the request hostname determines this value. Azure CDN origins, such as Web Apps, Blob Storage, and Cloud Services require this host header value to match the origin hostname by default. This overrides the host header defined at Endpoint"]
    #[serde(rename = "originHostHeader", default, skip_serializing_if = "Option::is_none")]
    pub origin_host_header: Option<String>,
    #[doc = "Priority of origin in given origin group for load balancing. Higher priorities will not be used for load balancing if any lower priority origin is healthy.Must be between 1 and 5"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "Weight of the origin in given origin group for load balancing. Must be between 1 and 1000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[doc = "Origin is enabled for load balancing or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The Alias of the Private Link resource. Populating this optional field indicates that this origin is 'Private'"]
    #[serde(rename = "privateLinkAlias", default, skip_serializing_if = "Option::is_none")]
    pub private_link_alias: Option<String>,
    #[doc = "The Resource Id of the Private Link resource. Populating this optional field indicates that this backend is 'Private'"]
    #[serde(rename = "privateLinkResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_resource_id: Option<String>,
    #[doc = "The location of the Private Link resource. Required only if 'privateLinkResourceId' is populated"]
    #[serde(rename = "privateLinkLocation", default, skip_serializing_if = "Option::is_none")]
    pub private_link_location: Option<String>,
    #[doc = "A custom message to be included in the approval request to connect to the Private Link."]
    #[serde(rename = "privateLinkApprovalMessage", default, skip_serializing_if = "Option::is_none")]
    pub private_link_approval_message: Option<String>,
}
impl OriginUpdatePropertiesParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the parameters for PostArgs match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostArgsMatchConditionParameters {
    #[serde(rename = "typeName")]
    pub type_name: post_args_match_condition_parameters::TypeName,
    #[doc = "Name of PostArg to be matched"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[doc = "Describes operator to be matched"]
    pub operator: post_args_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl PostArgsMatchConditionParameters {
    pub fn new(
        type_name: post_args_match_condition_parameters::TypeName,
        operator: post_args_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
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
    pub enum TypeName {
        DeliveryRulePostArgsConditionParameters,
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
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::RegEx => serializer.serialize_unit_variant("Operator", 9u32, "RegEx"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The approval status for the connection to the Private Link"]
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
#[doc = "A profile is a logical grouping of endpoints that share the same settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Standard_Verizon = The SKU name for a Standard Verizon CDN profile.\nPremium_Verizon = The SKU name for a Premium Verizon CDN profile.\nCustom_Verizon = The SKU name for a Custom Verizon CDN profile.\nStandard_Akamai = The SKU name for an Akamai CDN profile.\nStandard_ChinaCdn = The SKU name for a China CDN profile for VOD, Web and download scenarios using GB based billing model.\nStandard_Microsoft = The SKU name for a Standard Microsoft CDN profile.\nStandard_AzureFrontDoor =  The SKU name for an Azure Front Door Standard profile.\nPremium_AzureFrontDoor = The SKU name for an Azure Front Door Premium profile.\nStandard_955BandWidth_ChinaCdn = The SKU name for a China CDN profile for VOD, Web and download scenarios using 95-5 peak bandwidth billing model.\nStandard_AvgBandWidth_ChinaCdn = The SKU name for a China CDN profile for VOD, Web and download scenarios using monthly average peak bandwidth billing model.\nStandardPlus_ChinaCdn = The SKU name for a China CDN profile for live-streaming using GB based billing model.\nStandardPlus_955BandWidth_ChinaCdn = The SKU name for a China CDN live-streaming profile using 95-5 peak bandwidth billing model.\nStandardPlus_AvgBandWidth_ChinaCdn = The SKU name for a China CDN live-streaming profile using monthly average peak bandwidth billing model.\n"]
    pub sku: Sku,
    #[doc = "Kind of the profile. Used by portal to differentiate traditional CDN profile and new AFD profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "The JSON object that contains the properties required to create a profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProfileProperties>,
}
impl Profile {
    pub fn new(tracked_resource: TrackedResource, sku: Sku) -> Self {
        Self {
            tracked_resource,
            sku,
            kind: None,
            identity: None,
            properties: None,
        }
    }
}
#[doc = "Parameters required for profile upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileChangeSkuWafMapping {
    #[doc = "The security policy name."]
    #[serde(rename = "securityPolicyName")]
    pub security_policy_name: String,
    #[doc = "Reference to another resource."]
    #[serde(rename = "changeToWafPolicy")]
    pub change_to_waf_policy: ResourceReference,
}
impl ProfileChangeSkuWafMapping {
    pub fn new(security_policy_name: String, change_to_waf_policy: ResourceReference) -> Self {
        Self {
            security_policy_name,
            change_to_waf_policy,
        }
    }
}
#[doc = "Result of the request to list profiles. It contains a list of profile objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileListResult {
    #[doc = "List of CDN profiles within a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Profile>,
    #[doc = "URL to get the next set of profile objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProfileListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
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
    pub provisioning_state: Option<profile_properties::ProvisioningState>,
    #[doc = "Key-Value pair representing additional properties for profiles."]
    #[serde(rename = "extendedProperties", default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<serde_json::Value>,
    #[doc = "The Id of the frontdoor."]
    #[serde(rename = "frontDoorId", default, skip_serializing_if = "Option::is_none")]
    pub front_door_id: Option<String>,
    #[doc = "Send and receive timeout on forwarding request to the origin. When timeout is reached, the request fails and returns."]
    #[serde(rename = "originResponseTimeoutSeconds", default, skip_serializing_if = "Option::is_none")]
    pub origin_response_timeout_seconds: Option<i32>,
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
        Migrating,
        Migrated,
        PendingMigrationCommit,
        CommittingMigration,
        AbortingMigration,
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
                Self::Migrating => serializer.serialize_unit_variant("ResourceState", 4u32, "Migrating"),
                Self::Migrated => serializer.serialize_unit_variant("ResourceState", 5u32, "Migrated"),
                Self::PendingMigrationCommit => serializer.serialize_unit_variant("ResourceState", 6u32, "PendingMigrationCommit"),
                Self::CommittingMigration => serializer.serialize_unit_variant("ResourceState", 7u32, "CommittingMigration"),
                Self::AbortingMigration => serializer.serialize_unit_variant("ResourceState", 8u32, "AbortingMigration"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning status of the profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Updating,
        Deleting,
        Creating,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The JSON object containing profile update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfilePropertiesUpdateParameters {
    #[doc = "Send and receive timeout on forwarding request to the origin. When timeout is reached, the request fails and returns."]
    #[serde(rename = "originResponseTimeoutSeconds", default, skip_serializing_if = "Option::is_none")]
    pub origin_response_timeout_seconds: Option<i32>,
}
impl ProfilePropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties required to update a profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileUpdateParameters {
    #[doc = "Profile tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "The JSON object containing profile update parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProfilePropertiesUpdateParameters>,
}
impl ProfileUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters required for profile upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileUpgradeParameters {
    #[doc = "Web Application Firewall (WAF) and security policy mapping for the profile upgrade"]
    #[serde(rename = "wafMappingList")]
    pub waf_mapping_list: Vec<ProfileChangeSkuWafMapping>,
}
impl ProfileUpgradeParameters {
    pub fn new(waf_mapping_list: Vec<ProfileChangeSkuWafMapping>) -> Self {
        Self { waf_mapping_list }
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
    #[serde(rename = "typeName")]
    pub type_name: query_string_match_condition_parameters::TypeName,
    #[doc = "Describes operator to be matched"]
    pub operator: query_string_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl QueryStringMatchConditionParameters {
    pub fn new(
        type_name: query_string_match_condition_parameters::TypeName,
        operator: query_string_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
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
    pub enum TypeName {
        DeliveryRuleQueryStringConditionParameters,
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
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::RegEx => serializer.serialize_unit_variant("Operator", 9u32, "RegEx"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Rankings Response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RankingsResponse {
    #[serde(rename = "dateTimeBegin", default, with = "azure_core::date::rfc3339::option")]
    pub date_time_begin: Option<time::OffsetDateTime>,
    #[serde(rename = "dateTimeEnd", default, with = "azure_core::date::rfc3339::option")]
    pub date_time_end: Option<time::OffsetDateTime>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tables: Vec<serde_json::Value>,
}
impl RankingsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a rate limiting rule that can be included in a waf policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RateLimitRule {
    #[serde(flatten)]
    pub custom_rule: CustomRule,
    #[doc = "Defines rate limit threshold."]
    #[serde(rename = "rateLimitThreshold")]
    pub rate_limit_threshold: i32,
    #[doc = "Defines rate limit duration. Default is 1 minute."]
    #[serde(rename = "rateLimitDurationInMinutes")]
    pub rate_limit_duration_in_minutes: i32,
}
impl RateLimitRule {
    pub fn new(custom_rule: CustomRule, rate_limit_threshold: i32, rate_limit_duration_in_minutes: i32) -> Self {
        Self {
            custom_rule,
            rate_limit_threshold,
            rate_limit_duration_in_minutes,
        }
    }
}
#[doc = "Defines contents of rate limit rules"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RateLimitRuleList {
    #[doc = "List of rules"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rules: Vec<RateLimitRule>,
}
impl RateLimitRuleList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the parameters for RemoteAddress match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteAddressMatchConditionParameters {
    #[serde(rename = "typeName")]
    pub type_name: remote_address_match_condition_parameters::TypeName,
    #[doc = "Describes operator to be matched"]
    pub operator: remote_address_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "Match values to match against. The operator will apply to each value in here with OR semantics. If any of them match the variable with the given operator this match condition is considered a match."]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl RemoteAddressMatchConditionParameters {
    pub fn new(
        type_name: remote_address_match_condition_parameters::TypeName,
        operator: remote_address_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
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
    pub enum TypeName {
        DeliveryRuleRemoteAddressConditionParameters,
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
    #[serde(rename = "typeName")]
    pub type_name: request_body_match_condition_parameters::TypeName,
    #[doc = "Describes operator to be matched"]
    pub operator: request_body_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl RequestBodyMatchConditionParameters {
    pub fn new(
        type_name: request_body_match_condition_parameters::TypeName,
        operator: request_body_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
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
    pub enum TypeName {
        DeliveryRuleRequestBodyConditionParameters,
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
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::RegEx => serializer.serialize_unit_variant("Operator", 9u32, "RegEx"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameters for RequestHeader match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestHeaderMatchConditionParameters {
    #[serde(rename = "typeName")]
    pub type_name: request_header_match_condition_parameters::TypeName,
    #[doc = "Name of Header to be matched"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[doc = "Describes operator to be matched"]
    pub operator: request_header_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl RequestHeaderMatchConditionParameters {
    pub fn new(
        type_name: request_header_match_condition_parameters::TypeName,
        operator: request_header_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
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
    pub enum TypeName {
        DeliveryRuleRequestHeaderConditionParameters,
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
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::RegEx => serializer.serialize_unit_variant("Operator", 9u32, "RegEx"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameters for RequestMethod match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestMethodMatchConditionParameters {
    #[serde(rename = "typeName")]
    pub type_name: request_method_match_condition_parameters::TypeName,
    #[doc = "Describes operator to be matched"]
    pub operator: request_method_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
}
impl RequestMethodMatchConditionParameters {
    pub fn new(
        type_name: request_method_match_condition_parameters::TypeName,
        operator: request_method_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
            operator,
            negate_condition: None,
            transforms: Vec::new(),
            match_values: Vec::new(),
        }
    }
}
pub mod request_method_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        DeliveryRuleRequestMethodConditionParameters,
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
    #[serde(rename = "typeName")]
    pub type_name: request_scheme_match_condition_parameters::TypeName,
    #[doc = "Describes operator to be matched"]
    pub operator: request_scheme_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
}
impl RequestSchemeMatchConditionParameters {
    pub fn new(
        type_name: request_scheme_match_condition_parameters::TypeName,
        operator: request_scheme_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
            operator,
            negate_condition: None,
            transforms: Vec::new(),
            match_values: Vec::new(),
        }
    }
}
pub mod request_scheme_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        DeliveryRuleRequestSchemeConditionParameters,
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
    #[serde(rename = "typeName")]
    pub type_name: request_uri_match_condition_parameters::TypeName,
    #[doc = "Describes operator to be matched"]
    pub operator: request_uri_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl RequestUriMatchConditionParameters {
    pub fn new(
        type_name: request_uri_match_condition_parameters::TypeName,
        operator: request_uri_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
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
    pub enum TypeName {
        DeliveryRuleRequestUriConditionParameters,
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
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::RegEx => serializer.serialize_unit_variant("Operator", 9u32, "RegEx"),
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
    #[doc = "Read only system data"]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
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
#[serde(remote = "ResourceType")]
pub enum ResourceType {
    #[serde(rename = "Microsoft.Cdn/Profiles/Endpoints")]
    MicrosoftCdnProfilesEndpoints,
    #[serde(rename = "Microsoft.Cdn/Profiles/AfdEndpoints")]
    MicrosoftCdnProfilesAfdEndpoints,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::MicrosoftCdnProfilesEndpoints => {
                serializer.serialize_unit_variant("ResourceType", 0u32, "Microsoft.Cdn/Profiles/Endpoints")
            }
            Self::MicrosoftCdnProfilesAfdEndpoints => {
                serializer.serialize_unit_variant("ResourceType", 1u32, "Microsoft.Cdn/Profiles/AfdEndpoints")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Output of check resource usage API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceUsage {
    #[doc = "Resource type for which the usage is provided."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Unit of the usage. e.g. count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<resource_usage::Unit>,
    #[doc = "Actual value of usage on the specified resource type."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i32>,
    #[doc = "Quota of the specified resource type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}
impl ResourceUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_usage {
    use super::*;
    #[doc = "Unit of the usage. e.g. count."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        #[serde(rename = "count")]
        Count,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Count => serializer.serialize_unit_variant("Unit", 0u32, "count"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Output of check resource usage API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceUsageListResult {
    #[doc = "List of resource usages."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ResourceUsage>,
    #[doc = "URL to get the next set of custom domain objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceUsageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ResourceUsageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resources Response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcesResponse {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endpoints: Vec<serde_json::Value>,
    #[serde(
        rename = "customDomains",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_domains: Vec<serde_json::Value>,
}
impl ResourcesResponse {
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
    pub response_based_failover_threshold_percentage: Option<i32>,
    #[doc = "The list of Http status code ranges that are considered as server errors for origin and it is marked as unhealthy."]
    #[serde(
        rename = "httpErrorRanges",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "Friendly Routes name mapping to the any Routes or secret related information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Route {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The JSON object that contains the properties of the Routes to create."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RouteProperties>,
}
impl Route {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the parameters for the route configuration override action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteConfigurationOverrideActionParameters {
    #[serde(rename = "typeName")]
    pub type_name: route_configuration_override_action_parameters::TypeName,
    #[doc = "Defines the parameters for the origin group override configuration."]
    #[serde(rename = "originGroupOverride", default, skip_serializing_if = "Option::is_none")]
    pub origin_group_override: Option<OriginGroupOverride>,
    #[doc = "Caching settings for a caching-type route. To disable caching, do not provide a cacheConfiguration object."]
    #[serde(rename = "cacheConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cache_configuration: Option<CacheConfiguration>,
}
impl RouteConfigurationOverrideActionParameters {
    pub fn new(type_name: route_configuration_override_action_parameters::TypeName) -> Self {
        Self {
            type_name,
            origin_group_override: None,
            cache_configuration: None,
        }
    }
}
pub mod route_configuration_override_action_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        DeliveryRuleRouteConfigurationOverrideActionParameters,
    }
}
#[doc = "Result of the request to list routes. It contains a list of route objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RouteListResult {
    #[doc = "List of AzureFrontDoor routes within a profile."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Route>,
    #[doc = "URL to get the next set of route objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RouteListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RouteListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties of the Routes to create."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteProperties {
    #[serde(flatten)]
    pub route_update_properties_parameters: RouteUpdatePropertiesParameters,
    #[serde(flatten)]
    pub afd_state_properties: AfdStateProperties,
}
impl RouteProperties {
    pub fn new() -> Self {
        Self {
            route_update_properties_parameters: RouteUpdatePropertiesParameters::default(),
            afd_state_properties: AfdStateProperties::default(),
        }
    }
}
#[doc = "The domain JSON object required for domain creation or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RouteUpdateParameters {
    #[doc = "The JSON object that contains the properties of the domain to create."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RouteUpdatePropertiesParameters>,
}
impl RouteUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties of the domain to create."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RouteUpdatePropertiesParameters {
    #[doc = "The name of the endpoint which holds the route."]
    #[serde(rename = "endpointName", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    #[doc = "Domains referenced by this endpoint."]
    #[serde(
        rename = "customDomains",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_domains: Vec<ActivatedResourceReference>,
    #[doc = "Reference to another resource."]
    #[serde(rename = "originGroup", default, skip_serializing_if = "Option::is_none")]
    pub origin_group: Option<ResourceReference>,
    #[doc = "A directory path on the origin that AzureFrontDoor can use to retrieve content from, e.g. contoso.cloudapp.net/originpath."]
    #[serde(rename = "originPath", default, skip_serializing_if = "Option::is_none")]
    pub origin_path: Option<String>,
    #[doc = "rule sets referenced by this endpoint."]
    #[serde(
        rename = "ruleSets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rule_sets: Vec<ResourceReference>,
    #[doc = "List of supported protocols for this route."]
    #[serde(
        rename = "supportedProtocols",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_protocols: Vec<AfdEndpointProtocols>,
    #[doc = "The route patterns of the rule."]
    #[serde(
        rename = "patternsToMatch",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub patterns_to_match: Vec<String>,
    #[doc = "Caching settings for a caching-type route. To disable caching, do not provide a cacheConfiguration object."]
    #[serde(rename = "cacheConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cache_configuration: Option<AfdRouteCacheConfiguration>,
    #[doc = "Protocol this rule will use when forwarding traffic to backends."]
    #[serde(rename = "forwardingProtocol", default, skip_serializing_if = "Option::is_none")]
    pub forwarding_protocol: Option<route_update_properties_parameters::ForwardingProtocol>,
    #[doc = "whether this route will be linked to the default endpoint domain."]
    #[serde(rename = "linkToDefaultDomain", default, skip_serializing_if = "Option::is_none")]
    pub link_to_default_domain: Option<route_update_properties_parameters::LinkToDefaultDomain>,
    #[doc = "Whether to automatically redirect HTTP traffic to HTTPS traffic. Note that this is a easy way to set up this rule and it will be the first rule that gets executed."]
    #[serde(rename = "httpsRedirect", default, skip_serializing_if = "Option::is_none")]
    pub https_redirect: Option<route_update_properties_parameters::HttpsRedirect>,
    #[doc = "Whether to enable use of this rule. Permitted values are 'Enabled' or 'Disabled'"]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<route_update_properties_parameters::EnabledState>,
}
impl RouteUpdatePropertiesParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod route_update_properties_parameters {
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
    impl Default for ForwardingProtocol {
        fn default() -> Self {
            Self::MatchRequest
        }
    }
    #[doc = "whether this route will be linked to the default endpoint domain."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LinkToDefaultDomain")]
    pub enum LinkToDefaultDomain {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LinkToDefaultDomain {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LinkToDefaultDomain {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LinkToDefaultDomain {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("LinkToDefaultDomain", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("LinkToDefaultDomain", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for LinkToDefaultDomain {
        fn default() -> Self {
            Self::Disabled
        }
    }
    #[doc = "Whether to automatically redirect HTTP traffic to HTTPS traffic. Note that this is a easy way to set up this rule and it will be the first rule that gets executed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HttpsRedirect")]
    pub enum HttpsRedirect {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HttpsRedirect {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HttpsRedirect {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HttpsRedirect {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("HttpsRedirect", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("HttpsRedirect", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for HttpsRedirect {
        fn default() -> Self {
            Self::Disabled
        }
    }
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
}
#[doc = "Friendly Rules name mapping to the any Rules or secret related information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Rule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The JSON object that contains the properties of the Rules to create."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RuleProperties>,
}
impl Rule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list rules. It contains a list of rule objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleListResult {
    #[doc = "List of AzureFrontDoor rules within a rule set."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Rule>,
    #[doc = "URL to get the next set of rule objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties of the Rules to create."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleProperties {
    #[serde(flatten)]
    pub rule_update_properties_parameters: RuleUpdatePropertiesParameters,
    #[serde(flatten)]
    pub afd_state_properties: AfdStateProperties,
}
impl RuleProperties {
    pub fn new() -> Self {
        Self {
            rule_update_properties_parameters: RuleUpdatePropertiesParameters::default(),
            afd_state_properties: AfdStateProperties::default(),
        }
    }
}
#[doc = "Friendly RuleSet name mapping to the any RuleSet or secret related information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleSet {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The JSON object that contains the properties of the Rule Set to create."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RuleSetProperties>,
}
impl RuleSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list rule sets. It contains a list of rule set objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleSetListResult {
    #[doc = "List of AzureFrontDoor rule sets within a profile."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RuleSet>,
    #[doc = "URL to get the next set of rule set objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RuleSetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RuleSetListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties of the Rule Set to create."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleSetProperties {
    #[serde(flatten)]
    pub afd_state_properties: AfdStateProperties,
    #[doc = "The name of the profile which holds the rule set."]
    #[serde(rename = "profileName", default, skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
}
impl RuleSetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The domain JSON object required for domain creation or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleUpdateParameters {
    #[doc = "The JSON object that contains the properties of the rule to update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RuleUpdatePropertiesParameters>,
}
impl RuleUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties of the rule to update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleUpdatePropertiesParameters {
    #[doc = "The name of the rule set containing the rule."]
    #[serde(rename = "ruleSetName", default, skip_serializing_if = "Option::is_none")]
    pub rule_set_name: Option<String>,
    #[doc = "The order in which the rules are applied for the endpoint. Possible values {0,1,2,3,………}. A rule with a lesser order will be applied before a rule with a greater order. Rule with order 0 is a special rule. It does not require any condition and actions listed in it will always be applied."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "A list of conditions that must be matched for the actions to be executed"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions: Vec<DeliveryRuleConditionUnion>,
    #[doc = "A list of actions that are executed when all the conditions of a rule are satisfied."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<DeliveryRuleActionUnion>,
    #[doc = "If this rule is a match should the rules engine continue running the remaining rules or stop. If not present, defaults to Continue."]
    #[serde(rename = "matchProcessingBehavior", default, skip_serializing_if = "Option::is_none")]
    pub match_processing_behavior: Option<rule_update_properties_parameters::MatchProcessingBehavior>,
}
impl RuleUpdatePropertiesParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod rule_update_properties_parameters {
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
    impl Default for MatchProcessingBehavior {
        fn default() -> Self {
            Self::Continue
        }
    }
}
#[doc = "Friendly Secret name mapping to the any Secret or secret related information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Secret {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The JSON object that contains the properties of the Secret to create."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecretProperties>,
}
impl Secret {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list secrets. It contains a list of Secret objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretListResult {
    #[doc = "List of AzureFrontDoor secrets within a profile."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Secret>,
    #[doc = "URL to get the next set of Secret objects if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecretListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SecretListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The json object containing secret parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretParameters {
    #[doc = "The type of the secret resource."]
    #[serde(rename = "type")]
    pub type_: SecretType,
}
impl SecretParameters {
    pub fn new(type_: SecretType) -> Self {
        Self { type_ }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SecretParametersUnion {
    AzureFirstPartyManagedCertificate(AzureFirstPartyManagedCertificateParameters),
    CustomerCertificate(CustomerCertificateParameters),
    ManagedCertificate(ManagedCertificateParameters),
    UrlSigningKey(UrlSigningKeyParameters),
}
#[doc = "The JSON object that contains the properties of the Secret to create."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretProperties {
    #[serde(flatten)]
    pub afd_state_properties: AfdStateProperties,
    #[doc = "The name of the profile which holds the secret."]
    #[serde(rename = "profileName", default, skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    #[doc = "The json object containing secret parameters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<SecretParametersUnion>,
}
impl SecretProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of the secret resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SecretType")]
pub enum SecretType {
    UrlSigningKey,
    CustomerCertificate,
    ManagedCertificate,
    AzureFirstPartyManagedCertificate,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SecretType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SecretType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SecretType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UrlSigningKey => serializer.serialize_unit_variant("SecretType", 0u32, "UrlSigningKey"),
            Self::CustomerCertificate => serializer.serialize_unit_variant("SecretType", 1u32, "CustomerCertificate"),
            Self::ManagedCertificate => serializer.serialize_unit_variant("SecretType", 2u32, "ManagedCertificate"),
            Self::AzureFirstPartyManagedCertificate => {
                serializer.serialize_unit_variant("SecretType", 3u32, "AzureFirstPartyManagedCertificate")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SecurityPolicy association for AzureFrontDoor profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The json object that contains properties required to create a security policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityPolicyProperties>,
}
impl SecurityPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list security policies. It contains a list of security policy objects and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityPolicyListResult {
    #[doc = "List of Security policies within a profile"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SecurityPolicy>,
    #[doc = "URL to get the next set of security policy objects if there is any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecurityPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SecurityPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The json object that contains properties required to create a security policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityPolicyProperties {
    #[serde(flatten)]
    pub afd_state_properties: AfdStateProperties,
    #[doc = "The name of the profile which holds the security policy."]
    #[serde(rename = "profileName", default, skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    #[doc = "The json object containing security policy parameters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<SecurityPolicyPropertiesParametersUnion>,
}
impl SecurityPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The json object containing security policy parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityPolicyPropertiesParameters {
    #[doc = "The type of the Security policy to create."]
    #[serde(rename = "type")]
    pub type_: security_policy_properties_parameters::Type,
}
impl SecurityPolicyPropertiesParameters {
    pub fn new(type_: security_policy_properties_parameters::Type) -> Self {
        Self { type_ }
    }
}
pub mod security_policy_properties_parameters {
    use super::*;
    #[doc = "The type of the Security policy to create."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        WebApplicationFirewall,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::WebApplicationFirewall => serializer.serialize_unit_variant("Type", 0u32, "WebApplicationFirewall"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SecurityPolicyPropertiesParametersUnion {
    WebApplicationFirewall(SecurityPolicyWebApplicationFirewallParameters),
}
#[doc = "The JSON object containing security policy update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityPolicyUpdateParameters {
    #[doc = "The json object that contains properties required to update a security policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityPolicyUpdateProperties>,
}
impl SecurityPolicyUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The json object that contains properties required to update a security policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityPolicyUpdateProperties {
    #[doc = "The json object containing security policy parameters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<SecurityPolicyPropertiesParametersUnion>,
}
impl SecurityPolicyUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "settings for security policy patterns to match"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityPolicyWebApplicationFirewallAssociation {
    #[doc = "List of domains."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub domains: Vec<ActivatedResourceReference>,
    #[doc = "List of paths"]
    #[serde(
        rename = "patternsToMatch",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub patterns_to_match: Vec<String>,
}
impl SecurityPolicyWebApplicationFirewallAssociation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The json object containing security policy waf parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityPolicyWebApplicationFirewallParameters {
    #[serde(flatten)]
    pub security_policy_properties_parameters: SecurityPolicyPropertiesParameters,
    #[doc = "Reference to another resource."]
    #[serde(rename = "wafPolicy", default, skip_serializing_if = "Option::is_none")]
    pub waf_policy: Option<ResourceReference>,
    #[doc = "Waf associations"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub associations: Vec<SecurityPolicyWebApplicationFirewallAssociation>,
}
impl SecurityPolicyWebApplicationFirewallParameters {
    pub fn new(security_policy_properties_parameters: SecurityPolicyPropertiesParameters) -> Self {
        Self {
            security_policy_properties_parameters,
            waf_policy: None,
            associations: Vec::new(),
        }
    }
}
#[doc = "Defines the parameters for ServerPort match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPortMatchConditionParameters {
    #[serde(rename = "typeName")]
    pub type_name: server_port_match_condition_parameters::TypeName,
    #[doc = "Describes operator to be matched"]
    pub operator: server_port_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl ServerPortMatchConditionParameters {
    pub fn new(
        type_name: server_port_match_condition_parameters::TypeName,
        operator: server_port_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod server_port_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        DeliveryRuleServerPortConditionParameters,
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
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::RegEx => serializer.serialize_unit_variant("Operator", 9u32, "RegEx"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "One property of operation, include log specifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Log specifications of operation."]
    #[serde(
        rename = "logSpecifications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub log_specifications: Vec<LogSpecification>,
    #[doc = "Metric specifications of operation."]
    #[serde(
        rename = "metricSpecifications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metric_specifications: Vec<MetricSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of an existing Shared Private Link Resource to use when connecting to a private origin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPrivateLinkResourceProperties {
    #[doc = "Reference to another resource."]
    #[serde(rename = "privateLink", default, skip_serializing_if = "Option::is_none")]
    pub private_link: Option<ResourceReference>,
    #[doc = "The location of the shared private link resource"]
    #[serde(rename = "privateLinkLocation", default, skip_serializing_if = "Option::is_none")]
    pub private_link_location: Option<String>,
    #[doc = "The group id from the provider of resource the shared private link resource is for."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The request message for requesting approval of the shared private link resource."]
    #[serde(rename = "requestMessage", default, skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,
    #[doc = "Status of the shared private link resource. Can be Pending, Approved, Rejected, Disconnected, or Timeout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<shared_private_link_resource_properties::Status>,
}
impl SharedPrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod shared_private_link_resource_properties {
    use super::*;
    #[doc = "Status of the shared private link resource. Can be Pending, Approved, Rejected, Disconnected, or Timeout."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Pending,
        Approved,
        Rejected,
        Disconnected,
        Timeout,
    }
}
#[doc = "Standard_Verizon = The SKU name for a Standard Verizon CDN profile.\nPremium_Verizon = The SKU name for a Premium Verizon CDN profile.\nCustom_Verizon = The SKU name for a Custom Verizon CDN profile.\nStandard_Akamai = The SKU name for an Akamai CDN profile.\nStandard_ChinaCdn = The SKU name for a China CDN profile for VOD, Web and download scenarios using GB based billing model.\nStandard_Microsoft = The SKU name for a Standard Microsoft CDN profile.\nStandard_AzureFrontDoor =  The SKU name for an Azure Front Door Standard profile.\nPremium_AzureFrontDoor = The SKU name for an Azure Front Door Premium profile.\nStandard_955BandWidth_ChinaCdn = The SKU name for a China CDN profile for VOD, Web and download scenarios using 95-5 peak bandwidth billing model.\nStandard_AvgBandWidth_ChinaCdn = The SKU name for a China CDN profile for VOD, Web and download scenarios using monthly average peak bandwidth billing model.\nStandardPlus_ChinaCdn = The SKU name for a China CDN profile for live-streaming using GB based billing model.\nStandardPlus_955BandWidth_ChinaCdn = The SKU name for a China CDN live-streaming profile using 95-5 peak bandwidth billing model.\nStandardPlus_AvgBandWidth_ChinaCdn = The SKU name for a China CDN live-streaming profile using monthly average peak bandwidth billing model.\n"]
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
        #[serde(rename = "Standard_AzureFrontDoor")]
        StandardAzureFrontDoor,
        #[serde(rename = "Premium_AzureFrontDoor")]
        PremiumAzureFrontDoor,
        #[serde(rename = "Standard_955BandWidth_ChinaCdn")]
        Standard955bandWidthChinaCdn,
        #[serde(rename = "Standard_AvgBandWidth_ChinaCdn")]
        StandardAvgBandWidthChinaCdn,
        #[serde(rename = "StandardPlus_ChinaCdn")]
        StandardPlusChinaCdn,
        #[serde(rename = "StandardPlus_955BandWidth_ChinaCdn")]
        StandardPlus955bandWidthChinaCdn,
        #[serde(rename = "StandardPlus_AvgBandWidth_ChinaCdn")]
        StandardPlusAvgBandWidthChinaCdn,
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
                Self::StandardAzureFrontDoor => serializer.serialize_unit_variant("Name", 6u32, "Standard_AzureFrontDoor"),
                Self::PremiumAzureFrontDoor => serializer.serialize_unit_variant("Name", 7u32, "Premium_AzureFrontDoor"),
                Self::Standard955bandWidthChinaCdn => serializer.serialize_unit_variant("Name", 8u32, "Standard_955BandWidth_ChinaCdn"),
                Self::StandardAvgBandWidthChinaCdn => serializer.serialize_unit_variant("Name", 9u32, "Standard_AvgBandWidth_ChinaCdn"),
                Self::StandardPlusChinaCdn => serializer.serialize_unit_variant("Name", 10u32, "StandardPlus_ChinaCdn"),
                Self::StandardPlus955bandWidthChinaCdn => {
                    serializer.serialize_unit_variant("Name", 11u32, "StandardPlus_955BandWidth_ChinaCdn")
                }
                Self::StandardPlusAvgBandWidthChinaCdn => {
                    serializer.serialize_unit_variant("Name", 12u32, "StandardPlus_AvgBandWidth_ChinaCdn")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameters for SocketAddress match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocketAddrMatchConditionParameters {
    #[serde(rename = "typeName")]
    pub type_name: socket_addr_match_condition_parameters::TypeName,
    #[doc = "Describes operator to be matched"]
    pub operator: socket_addr_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl SocketAddrMatchConditionParameters {
    pub fn new(
        type_name: socket_addr_match_condition_parameters::TypeName,
        operator: socket_addr_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod socket_addr_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        DeliveryRuleSocketAddrConditionParameters,
    }
    #[doc = "Describes operator to be matched"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Any,
        #[serde(rename = "IPMatch")]
        IpMatch,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The protocol of an established TLS connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SslProtocol")]
pub enum SslProtocol {
    #[serde(rename = "TLSv1")]
    TlSv1,
    #[serde(rename = "TLSv1.1")]
    TlSv11,
    #[serde(rename = "TLSv1.2")]
    TlSv12,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SslProtocol {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SslProtocol {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SslProtocol {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::TlSv1 => serializer.serialize_unit_variant("SslProtocol", 0u32, "TLSv1"),
            Self::TlSv11 => serializer.serialize_unit_variant("SslProtocol", 1u32, "TLSv1.1"),
            Self::TlSv12 => serializer.serialize_unit_variant("SslProtocol", 2u32, "TLSv1.2"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the parameters for SslProtocol match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SslProtocolMatchConditionParameters {
    #[serde(rename = "typeName")]
    pub type_name: ssl_protocol_match_condition_parameters::TypeName,
    #[doc = "Describes operator to be matched"]
    pub operator: ssl_protocol_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<SslProtocol>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl SslProtocolMatchConditionParameters {
    pub fn new(
        type_name: ssl_protocol_match_condition_parameters::TypeName,
        operator: ssl_protocol_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
            operator,
            negate_condition: None,
            match_values: Vec::new(),
            transforms: Vec::new(),
        }
    }
}
pub mod ssl_protocol_match_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        DeliveryRuleSslProtocolConditionParameters,
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
    #[serde(
        rename = "supportedOptimizationTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_optimization_types: Vec<OptimizationType>,
}
impl SupportedOptimizationTypesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Read only system data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "An identifier for the identity that created the resource"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that creates/modifies resources"]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<IdentityType>,
    #[doc = "The timestamp of resource creation (UTC)"]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "An identifier for the identity that last modified the resource"]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that creates/modifies resources"]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<IdentityType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
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
#[doc = "Describes what transforms were applied before matching."]
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
#[doc = "Defines the parameters for UrlFileExtension match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlFileExtensionMatchConditionParameters {
    #[serde(rename = "typeName")]
    pub type_name: url_file_extension_match_condition_parameters::TypeName,
    #[doc = "Describes operator to be matched"]
    pub operator: url_file_extension_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl UrlFileExtensionMatchConditionParameters {
    pub fn new(
        type_name: url_file_extension_match_condition_parameters::TypeName,
        operator: url_file_extension_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
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
    pub enum TypeName {
        DeliveryRuleUrlFileExtensionMatchConditionParameters,
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
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::RegEx => serializer.serialize_unit_variant("Operator", 9u32, "RegEx"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameters for UrlFilename match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlFileNameMatchConditionParameters {
    #[serde(rename = "typeName")]
    pub type_name: url_file_name_match_condition_parameters::TypeName,
    #[doc = "Describes operator to be matched"]
    pub operator: url_file_name_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl UrlFileNameMatchConditionParameters {
    pub fn new(
        type_name: url_file_name_match_condition_parameters::TypeName,
        operator: url_file_name_match_condition_parameters::Operator,
    ) -> Self {
        Self {
            type_name,
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
    pub enum TypeName {
        DeliveryRuleUrlFilenameConditionParameters,
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
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::RegEx => serializer.serialize_unit_variant("Operator", 9u32, "RegEx"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameters for UrlPath match conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlPathMatchConditionParameters {
    #[serde(rename = "typeName")]
    pub type_name: url_path_match_condition_parameters::TypeName,
    #[doc = "Describes operator to be matched"]
    pub operator: url_path_match_condition_parameters::Operator,
    #[doc = "Describes if this is negate condition or not"]
    #[serde(rename = "negateCondition", default, skip_serializing_if = "Option::is_none")]
    pub negate_condition: Option<bool>,
    #[doc = "The match value for the condition of the delivery rule"]
    #[serde(
        rename = "matchValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_values: Vec<String>,
    #[doc = "List of transforms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transforms: Vec<Transform>,
}
impl UrlPathMatchConditionParameters {
    pub fn new(type_name: url_path_match_condition_parameters::TypeName, operator: url_path_match_condition_parameters::Operator) -> Self {
        Self {
            type_name,
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
    pub enum TypeName {
        DeliveryRuleUrlPathMatchConditionParameters,
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
                Self::Equal => serializer.serialize_unit_variant("Operator", 1u32, "Equal"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::BeginsWith => serializer.serialize_unit_variant("Operator", 3u32, "BeginsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 4u32, "EndsWith"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 5u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 6u32, "LessThanOrEqual"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 7u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 8u32, "GreaterThanOrEqual"),
                Self::Wildcard => serializer.serialize_unit_variant("Operator", 9u32, "Wildcard"),
                Self::RegEx => serializer.serialize_unit_variant("Operator", 10u32, "RegEx"),
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
    #[serde(rename = "typeName")]
    pub type_name: url_redirect_action_parameters::TypeName,
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
    pub fn new(type_name: url_redirect_action_parameters::TypeName, redirect_type: url_redirect_action_parameters::RedirectType) -> Self {
        Self {
            type_name,
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
    pub enum TypeName {
        DeliveryRuleUrlRedirectActionParameters,
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
    #[serde(rename = "typeName")]
    pub type_name: url_rewrite_action_parameters::TypeName,
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
    pub fn new(type_name: url_rewrite_action_parameters::TypeName, source_pattern: String, destination: String) -> Self {
        Self {
            type_name,
            source_pattern,
            destination,
            preserve_unmatched_path: None,
        }
    }
}
pub mod url_rewrite_action_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        DeliveryRuleUrlRewriteActionParameters,
    }
}
#[doc = "Defines the url signing action for the delivery rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlSigningAction {
    #[serde(flatten)]
    pub delivery_rule_action: DeliveryRuleAction,
    #[doc = "Defines the parameters for the Url Signing action."]
    pub parameters: UrlSigningActionParameters,
}
impl UrlSigningAction {
    pub fn new(delivery_rule_action: DeliveryRuleAction, parameters: UrlSigningActionParameters) -> Self {
        Self {
            delivery_rule_action,
            parameters,
        }
    }
}
#[doc = "Defines the parameters for the Url Signing action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlSigningActionParameters {
    #[serde(rename = "typeName")]
    pub type_name: url_signing_action_parameters::TypeName,
    #[doc = "Algorithm to use for URL signing"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<url_signing_action_parameters::Algorithm>,
    #[doc = "Defines which query string parameters in the url to be considered for expires, key id etc. "]
    #[serde(
        rename = "parameterNameOverride",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parameter_name_override: Vec<UrlSigningParamIdentifier>,
}
impl UrlSigningActionParameters {
    pub fn new(type_name: url_signing_action_parameters::TypeName) -> Self {
        Self {
            type_name,
            algorithm: None,
            parameter_name_override: Vec::new(),
        }
    }
}
pub mod url_signing_action_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TypeName {
        DeliveryRuleUrlSigningActionParameters,
    }
    #[doc = "Algorithm to use for URL signing"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Algorithm")]
    pub enum Algorithm {
        #[serde(rename = "SHA256")]
        Sha256,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Algorithm {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Algorithm {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Algorithm {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Sha256 => serializer.serialize_unit_variant("Algorithm", 0u32, "SHA256"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Url signing key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlSigningKey {
    #[doc = "Defines the customer defined key Id. This id will exist in the incoming request to indicate the key used to form the hash."]
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[doc = "Describes the parameters for using a user's KeyVault for URL Signing Key."]
    #[serde(rename = "keySourceParameters")]
    pub key_source_parameters: KeyVaultSigningKeyParameters,
}
impl UrlSigningKey {
    pub fn new(key_id: String, key_source_parameters: KeyVaultSigningKeyParameters) -> Self {
        Self {
            key_id,
            key_source_parameters,
        }
    }
}
#[doc = "Url signing key parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlSigningKeyParameters {
    #[serde(flatten)]
    pub secret_parameters: SecretParameters,
    #[doc = "Defines the customer defined key Id. This id will exist in the incoming request to indicate the key used to form the hash."]
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[doc = "Reference to another resource."]
    #[serde(rename = "secretSource")]
    pub secret_source: ResourceReference,
    #[doc = "Version of the secret to be used"]
    #[serde(rename = "secretVersion", default, skip_serializing_if = "Option::is_none")]
    pub secret_version: Option<String>,
}
impl UrlSigningKeyParameters {
    pub fn new(secret_parameters: SecretParameters, key_id: String, secret_source: ResourceReference) -> Self {
        Self {
            secret_parameters,
            key_id,
            secret_source,
            secret_version: None,
        }
    }
}
#[doc = "Defines how to identify a parameter for a specific purpose e.g. expires"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlSigningParamIdentifier {
    #[doc = "Indicates the purpose of the parameter"]
    #[serde(rename = "paramIndicator")]
    pub param_indicator: url_signing_param_identifier::ParamIndicator,
    #[doc = "Parameter name"]
    #[serde(rename = "paramName")]
    pub param_name: String,
}
impl UrlSigningParamIdentifier {
    pub fn new(param_indicator: url_signing_param_identifier::ParamIndicator, param_name: String) -> Self {
        Self {
            param_indicator,
            param_name,
        }
    }
}
pub mod url_signing_param_identifier {
    use super::*;
    #[doc = "Indicates the purpose of the parameter"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ParamIndicator")]
    pub enum ParamIndicator {
        Expires,
        KeyId,
        Signature,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ParamIndicator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ParamIndicator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ParamIndicator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Expires => serializer.serialize_unit_variant("ParamIndicator", 0u32, "Expires"),
                Self::KeyId => serializer.serialize_unit_variant("ParamIndicator", 1u32, "KeyId"),
                Self::Signature => serializer.serialize_unit_variant("ParamIndicator", 2u32, "Signature"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes resource usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    #[doc = "Resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "An enum describing the unit of measurement."]
    pub unit: usage::Unit,
    #[doc = "The current value of the usage."]
    #[serde(rename = "currentValue")]
    pub current_value: i64,
    #[doc = "The limit of usage."]
    pub limit: i64,
    #[doc = "The usage names."]
    pub name: UsageName,
}
impl Usage {
    pub fn new(unit: usage::Unit, current_value: i64, limit: i64, name: UsageName) -> Self {
        Self {
            id: None,
            unit,
            current_value,
            limit,
            name,
        }
    }
}
pub mod usage {
    use super::*;
    #[doc = "An enum describing the unit of measurement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Count,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Count => serializer.serialize_unit_variant("Unit", 0u32, "Count"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The usage names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageName {
    #[doc = "A string describing the resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "A localized string describing the resource name."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl UsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list usages operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsagesListResult {
    #[doc = "The list of resource usages."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Usage>,
    #[doc = "URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UsagesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl UsagesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Input of the secret to be validated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateSecretInput {
    #[doc = "The type of the secret resource."]
    #[serde(rename = "secretType")]
    pub secret_type: SecretType,
    #[doc = "Reference to another resource."]
    #[serde(rename = "secretSource")]
    pub secret_source: ResourceReference,
    #[doc = "Secret version, if customer is using a specific version."]
    #[serde(rename = "secretVersion", default, skip_serializing_if = "Option::is_none")]
    pub secret_version: Option<String>,
}
impl ValidateSecretInput {
    pub fn new(secret_type: SecretType, secret_source: ResourceReference) -> Self {
        Self {
            secret_type,
            secret_source,
            secret_version: None,
        }
    }
}
#[doc = "Output of the validated secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateSecretOutput {
    #[doc = "The validation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<validate_secret_output::Status>,
    #[doc = "Detailed error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ValidateSecretOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod validate_secret_output {
    use super::*;
    #[doc = "The validation status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Valid,
        Invalid,
        AccessDenied,
        CertificateExpired,
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
                Self::Valid => serializer.serialize_unit_variant("Status", 0u32, "Valid"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 1u32, "Invalid"),
                Self::AccessDenied => serializer.serialize_unit_variant("Status", 2u32, "AccessDenied"),
                Self::CertificateExpired => serializer.serialize_unit_variant("Status", 3u32, "CertificateExpired"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The validation token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationToken {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl ValidationToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Waf Metrics Response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WafMetricsResponse {
    #[serde(rename = "dateTimeBegin", default, with = "azure_core::date::rfc3339::option")]
    pub date_time_begin: Option<time::OffsetDateTime>,
    #[serde(rename = "dateTimeEnd", default, with = "azure_core::date::rfc3339::option")]
    pub date_time_end: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<waf_metrics_response::Granularity>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub series: Vec<serde_json::Value>,
}
impl WafMetricsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod waf_metrics_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Granularity")]
    pub enum Granularity {
        #[serde(rename = "PT5M")]
        Pt5m,
        #[serde(rename = "PT1H")]
        Pt1h,
        #[serde(rename = "P1D")]
        P1d,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Granularity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Granularity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Granularity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pt5m => serializer.serialize_unit_variant("Granularity", 0u32, "PT5M"),
                Self::Pt1h => serializer.serialize_unit_variant("Granularity", 1u32, "PT1H"),
                Self::P1d => serializer.serialize_unit_variant("Granularity", 2u32, "P1D"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Waf Rankings Response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WafRankingsResponse {
    #[serde(rename = "dateTimeBegin", default, with = "azure_core::date::rfc3339::option")]
    pub date_time_begin: Option<time::OffsetDateTime>,
    #[serde(rename = "dateTimeEnd", default, with = "azure_core::date::rfc3339::option")]
    pub date_time_end: Option<time::OffsetDateTime>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub groups: Vec<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data: Vec<serde_json::Value>,
}
impl WafRankingsResponse {
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
    pub prefix_length: Option<i32>,
}
impl CidrIpAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines contents of a web application firewall global configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicySettings {
    #[doc = "describes if the policy is in enabled state or disabled state"]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<policy_settings::EnabledState>,
    #[doc = "Describes if it is in detection mode or prevention mode at policy level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<policy_settings::Mode>,
    #[doc = "If action type is redirect, this field represents the default redirect URL for the client."]
    #[serde(rename = "defaultRedirectUrl", default, skip_serializing_if = "Option::is_none")]
    pub default_redirect_url: Option<String>,
    #[doc = "If the action type is block, this field defines the default customer overridable http response status code."]
    #[serde(rename = "defaultCustomBlockResponseStatusCode", default, skip_serializing_if = "Option::is_none")]
    pub default_custom_block_response_status_code: Option<policy_settings::DefaultCustomBlockResponseStatusCode>,
    #[doc = "If the action type is block, customer can override the response body. The body must be specified in base64 encoding."]
    #[serde(rename = "defaultCustomBlockResponseBody", default, skip_serializing_if = "Option::is_none")]
    pub default_custom_block_response_body: Option<String>,
}
impl PolicySettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_settings {
    use super::*;
    #[doc = "describes if the policy is in enabled state or disabled state"]
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
    #[doc = "If the action type is block, this field defines the default customer overridable http response status code."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DefaultCustomBlockResponseStatusCode {}
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

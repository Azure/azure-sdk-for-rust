#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ActionEnum")]
pub enum ActionEnum {
    Allow,
    DenySilent,
    DenyResetServer,
    DenyResetBoth,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ActionEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ActionEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ActionEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Allow => serializer.serialize_unit_variant("ActionEnum", 0u32, "Allow"),
            Self::DenySilent => serializer.serialize_unit_variant("ActionEnum", 1u32, "DenySilent"),
            Self::DenyResetServer => serializer.serialize_unit_variant("ActionEnum", 2u32, "DenyResetServer"),
            Self::DenyResetBoth => serializer.serialize_unit_variant("ActionEnum", 3u32, "DenyResetBoth"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AdvSecurityObjectTypeEnum")]
pub enum AdvSecurityObjectTypeEnum {
    #[serde(rename = "urlCustom")]
    UrlCustom,
    #[serde(rename = "feeds")]
    Feeds,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AdvSecurityObjectTypeEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AdvSecurityObjectTypeEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AdvSecurityObjectTypeEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UrlCustom => serializer.serialize_unit_variant("AdvSecurityObjectTypeEnum", 0u32, "urlCustom"),
            Self::Feeds => serializer.serialize_unit_variant("AdvSecurityObjectTypeEnum", 1u32, "feeds"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Data Type for App Seen"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSeenData {
    #[doc = "number of rows"]
    pub count: i32,
    #[doc = "array of appSeen"]
    #[serde(rename = "appSeenList")]
    pub app_seen_list: Vec<AppSeenInfo>,
}
impl AppSeenData {
    pub fn new(count: i32, app_seen_list: Vec<AppSeenInfo>) -> Self {
        Self { count, app_seen_list }
    }
}
#[doc = "Definition for App Seen"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSeenInfo {
    #[doc = "title"]
    pub title: String,
    #[doc = "category"]
    pub category: String,
    #[doc = "subCategory"]
    #[serde(rename = "subCategory")]
    pub sub_category: String,
    #[doc = "risk"]
    pub risk: String,
    #[doc = "tag"]
    pub tag: String,
    #[doc = "technology"]
    pub technology: String,
    #[doc = "standardPorts"]
    #[serde(rename = "standardPorts")]
    pub standard_ports: String,
}
impl AppSeenInfo {
    pub fn new(
        title: String,
        category: String,
        sub_category: String,
        risk: String,
        tag: String,
        technology: String,
        standard_ports: String,
    ) -> Self {
        Self {
            title,
            category,
            sub_category,
            risk,
            tag,
            technology,
            standard_ports,
        }
    }
}
#[doc = "Application Insights key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsights {
    #[doc = "Resource id for Application Insights"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Application Insights key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl ApplicationInsights {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the managed service identities assigned to this resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceManagerManagedIdentityProperties {
    #[doc = "The Active Directory tenant id of the principal."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The active directory identifier of this principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The kind of managed identity assigned to this resource."]
    #[serde(rename = "type")]
    pub type_: AzureResourceManagerManagedIdentityType,
    #[doc = "The identities assigned to this resource by the user."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl AzureResourceManagerManagedIdentityProperties {
    pub fn new(type_: AzureResourceManagerManagedIdentityType) -> Self {
        Self {
            tenant_id: None,
            principal_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "The kind of managed identity assigned to this resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureResourceManagerManagedIdentityType")]
pub enum AzureResourceManagerManagedIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned,UserAssigned")]
    SystemAssignedUserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureResourceManagerManagedIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureResourceManagerManagedIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureResourceManagerManagedIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("AzureResourceManagerManagedIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("AzureResourceManagerManagedIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("AzureResourceManagerManagedIdentityType", 2u32, "UserAssigned"),
            Self::SystemAssignedUserAssigned => {
                serializer.serialize_unit_variant("AzureResourceManagerManagedIdentityType", 3u32, "SystemAssigned,UserAssigned")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A managed identity assigned by the user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceManagerUserAssignedIdentity {
    #[doc = "The active directory client identifier for this principal."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The active directory identifier for this principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
}
impl AzureResourceManagerUserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Billing cycle"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BillingCycle")]
pub enum BillingCycle {
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "MONTHLY")]
    Monthly,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BillingCycle {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BillingCycle {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BillingCycle {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Weekly => serializer.serialize_unit_variant("BillingCycle", 0u32, "WEEKLY"),
            Self::Monthly => serializer.serialize_unit_variant("BillingCycle", 1u32, "MONTHLY"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Boolean Enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BooleanEnum")]
pub enum BooleanEnum {
    #[serde(rename = "TRUE")]
    True,
    #[serde(rename = "FALSE")]
    False,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BooleanEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BooleanEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BooleanEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::True => serializer.serialize_unit_variant("BooleanEnum", 0u32, "TRUE"),
            Self::False => serializer.serialize_unit_variant("BooleanEnum", 1u32, "FALSE"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "URL/EDL to match"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Category {
    #[doc = "custom URL"]
    #[serde(rename = "urlCustom")]
    pub url_custom: Vec<String>,
    #[doc = "feed list"]
    pub feeds: Vec<String>,
}
impl Category {
    pub fn new(url_custom: Vec<String>, feeds: Vec<String>) -> Self {
        Self { url_custom, feeds }
    }
}
#[doc = "certificate used for inbound and outbound decryption"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateObject {
    #[doc = "Resource Id of certificate signer, to be populated only when certificateSelfSigned is false"]
    #[serde(rename = "certificateSignerResourceId", default, skip_serializing_if = "Option::is_none")]
    pub certificate_signer_resource_id: Option<String>,
    #[doc = "Boolean Enum"]
    #[serde(rename = "certificateSelfSigned")]
    pub certificate_self_signed: BooleanEnum,
    #[doc = "comment for this object"]
    #[serde(rename = "auditComment", default, skip_serializing_if = "Option::is_none")]
    pub audit_comment: Option<String>,
    #[doc = "user description for this object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "read only string representing last create or update"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Provisioning state of the firewall resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl CertificateObject {
    pub fn new(certificate_self_signed: BooleanEnum) -> Self {
        Self {
            certificate_signer_resource_id: None,
            certificate_self_signed,
            audit_comment: None,
            description: None,
            etag: None,
            provisioning_state: None,
        }
    }
}
#[doc = "GlobalRulestack Certificate Object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateObjectGlobalRulestackResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "certificate used for inbound and outbound decryption"]
    pub properties: CertificateObject,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl CertificateObjectGlobalRulestackResource {
    pub fn new(properties: CertificateObject) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response of a CertificateObjectGlobalRulestackResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateObjectGlobalRulestackResourceListResult {
    #[doc = "The items on this page"]
    pub value: Vec<CertificateObjectGlobalRulestackResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CertificateObjectGlobalRulestackResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CertificateObjectGlobalRulestackResourceListResult {
    pub fn new(value: Vec<CertificateObjectGlobalRulestackResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "LocalRulestack Certificate Object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateObjectLocalRulestackResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "certificate used for inbound and outbound decryption"]
    pub properties: CertificateObject,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl CertificateObjectLocalRulestackResource {
    pub fn new(properties: CertificateObject) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response of a CertificateObjectLocalRulestackResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateObjectLocalRulestackResourceListResult {
    #[doc = "The items on this page"]
    pub value: Vec<CertificateObjectLocalRulestackResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CertificateObjectLocalRulestackResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CertificateObjectLocalRulestackResourceListResult {
    pub fn new(value: Vec<CertificateObjectLocalRulestackResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Changelog list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Changelog {
    #[doc = "list of changes"]
    pub changes: Vec<String>,
    #[doc = "lastCommitted timestamp"]
    #[serde(rename = "lastCommitted", default, with = "azure_core::date::rfc3339::option")]
    pub last_committed: Option<time::OffsetDateTime>,
    #[doc = "lastModified timestamp"]
    #[serde(rename = "lastModified", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
}
impl Changelog {
    pub fn new(changes: Vec<String>) -> Self {
        Self {
            changes,
            last_committed: None,
            last_modified: None,
        }
    }
}
#[doc = "Countries Response Object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CountriesResponse {
    #[doc = "List of countries"]
    pub value: Vec<Country>,
    #[doc = "next link"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CountriesResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CountriesResponse {
    pub fn new(value: Vec<Country>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Country Description"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Country {
    #[doc = "country code"]
    pub code: String,
    #[doc = "code description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl Country {
    pub fn new(code: String) -> Self {
        Self { code, description: None }
    }
}
#[doc = "DNS Proxy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DnsProxy")]
pub enum DnsProxy {
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DnsProxy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DnsProxy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DnsProxy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("DnsProxy", 0u32, "DISABLED"),
            Self::Enabled => serializer.serialize_unit_variant("DnsProxy", 1u32, "ENABLED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "DNS Proxy settings for Firewall"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsSettings {
    #[doc = "DNS Proxy"]
    #[serde(rename = "enableDnsProxy", default, skip_serializing_if = "Option::is_none")]
    pub enable_dns_proxy: Option<DnsProxy>,
    #[doc = "Enabled DNS type values"]
    #[serde(rename = "enabledDnsType", default, skip_serializing_if = "Option::is_none")]
    pub enabled_dns_type: Option<EnabledDnsType>,
    #[doc = "List of IPs associated with the Firewall"]
    #[serde(
        rename = "dnsServers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dns_servers: Vec<IpAddress>,
}
impl DnsSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DecryptionRuleTypeEnum")]
pub enum DecryptionRuleTypeEnum {
    #[serde(rename = "SSLOutboundInspection")]
    SslOutboundInspection,
    #[serde(rename = "SSLInboundInspection")]
    SslInboundInspection,
    None,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DecryptionRuleTypeEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DecryptionRuleTypeEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DecryptionRuleTypeEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SslOutboundInspection => serializer.serialize_unit_variant("DecryptionRuleTypeEnum", 0u32, "SSLOutboundInspection"),
            Self::SslInboundInspection => serializer.serialize_unit_variant("DecryptionRuleTypeEnum", 1u32, "SSLInboundInspection"),
            Self::None => serializer.serialize_unit_variant("DecryptionRuleTypeEnum", 2u32, "None"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Type for Default Mode for rules creation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DefaultMode")]
pub enum DefaultMode {
    #[serde(rename = "IPS")]
    Ips,
    #[serde(rename = "FIREWALL")]
    Firewall,
    #[serde(rename = "NONE")]
    None,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DefaultMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DefaultMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DefaultMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ips => serializer.serialize_unit_variant("DefaultMode", 0u32, "IPS"),
            Self::Firewall => serializer.serialize_unit_variant("DefaultMode", 1u32, "FIREWALL"),
            Self::None => serializer.serialize_unit_variant("DefaultMode", 2u32, "NONE"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "destination address"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DestinationAddr {
    #[doc = "special value 'any'"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cidrs: Vec<String>,
    #[doc = "list of countries"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub countries: Vec<String>,
    #[doc = "list of feeds"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub feeds: Vec<String>,
    #[doc = "prefix list"]
    #[serde(
        rename = "prefixLists",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub prefix_lists: Vec<String>,
    #[doc = "fqdn list"]
    #[serde(
        rename = "fqdnLists",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fqdn_lists: Vec<String>,
}
impl DestinationAddr {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Egress NAT"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EgressNat")]
pub enum EgressNat {
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EgressNat {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EgressNat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EgressNat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("EgressNat", 0u32, "DISABLED"),
            Self::Enabled => serializer.serialize_unit_variant("EgressNat", 1u32, "ENABLED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enabled DNS type values"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnabledDnsType")]
pub enum EnabledDnsType {
    #[serde(rename = "CUSTOM")]
    Custom,
    #[serde(rename = "AZURE")]
    Azure,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnabledDnsType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnabledDnsType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnabledDnsType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Custom => serializer.serialize_unit_variant("EnabledDnsType", 0u32, "CUSTOM"),
            Self::Azure => serializer.serialize_unit_variant("EnabledDnsType", 1u32, "AZURE"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Endpoint Configuration for frontend and backend"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointConfiguration {
    #[doc = "port ID"]
    pub port: String,
    #[doc = "IP Address"]
    pub address: IpAddress,
}
impl EndpointConfiguration {
    pub fn new(port: String, address: IpAddress) -> Self {
        Self { port, address }
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
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
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
#[doc = "EventHub configurations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHub {
    #[doc = "Resource ID of EventHub"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Subscription Id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "EventHub name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "EventHub namespace"]
    #[serde(rename = "nameSpace", default, skip_serializing_if = "Option::is_none")]
    pub name_space: Option<String>,
    #[doc = "EventHub policy name"]
    #[serde(rename = "policyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}
impl EventHub {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties specific to the Firewall resource deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallDeploymentProperties {
    #[doc = "panEtag info"]
    #[serde(rename = "panEtag", default, skip_serializing_if = "Option::is_none")]
    pub pan_etag: Option<String>,
    #[doc = "Network settings for Firewall"]
    #[serde(rename = "networkProfile")]
    pub network_profile: NetworkProfile,
    #[doc = "Boolean Enum"]
    #[serde(rename = "isPanoramaManaged", default, skip_serializing_if = "Option::is_none")]
    pub is_panorama_managed: Option<BooleanEnum>,
    #[doc = "Panorama Config"]
    #[serde(rename = "panoramaConfig", default, skip_serializing_if = "Option::is_none")]
    pub panorama_config: Option<PanoramaConfig>,
    #[doc = "Associated rulestack details"]
    #[serde(rename = "associatedRulestack", default, skip_serializing_if = "Option::is_none")]
    pub associated_rulestack: Option<RulestackDetails>,
    #[doc = "DNS Proxy settings for Firewall"]
    #[serde(rename = "dnsSettings")]
    pub dns_settings: DnsSettings,
    #[doc = "Frontend settings for Firewall"]
    #[serde(
        rename = "frontEndSettings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub front_end_settings: Vec<FrontendSetting>,
    #[doc = "Provisioning state of the firewall resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Billing plan information."]
    #[serde(rename = "planData")]
    pub plan_data: PlanData,
    #[doc = "MarketplaceDetails of PAN Firewall resource"]
    #[serde(rename = "marketplaceDetails")]
    pub marketplace_details: MarketplaceDetails,
}
impl FirewallDeploymentProperties {
    pub fn new(
        network_profile: NetworkProfile,
        dns_settings: DnsSettings,
        plan_data: PlanData,
        marketplace_details: MarketplaceDetails,
    ) -> Self {
        Self {
            pan_etag: None,
            network_profile,
            is_panorama_managed: None,
            panorama_config: None,
            associated_rulestack: None,
            dns_settings,
            front_end_settings: Vec::new(),
            provisioning_state: None,
            plan_data,
            marketplace_details,
        }
    }
}
#[doc = "PaloAltoNetworks Firewall"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties specific to the Firewall resource deployment."]
    pub properties: FirewallDeploymentProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of the managed service identities assigned to this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<AzureResourceManagerManagedIdentityProperties>,
}
impl FirewallResource {
    pub fn new(tracked_resource: TrackedResource, properties: FirewallDeploymentProperties) -> Self {
        Self {
            tracked_resource,
            properties,
            system_data: None,
            identity: None,
        }
    }
}
#[doc = "The response of a FirewallResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallResourceListResult {
    #[doc = "The items on this page"]
    pub value: Vec<FirewallResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FirewallResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FirewallResourceListResult {
    pub fn new(value: Vec<FirewallResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the FirewallResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallResourceUpdate {
    #[doc = "The properties of the managed service identities assigned to this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<AzureResourceManagerManagedIdentityProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the FirewallResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FirewallResourceUpdateProperties>,
}
impl FirewallResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the FirewallResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallResourceUpdateProperties {
    #[doc = "panEtag info"]
    #[serde(rename = "panEtag", default, skip_serializing_if = "Option::is_none")]
    pub pan_etag: Option<String>,
    #[doc = "Network settings for Firewall"]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    #[doc = "Boolean Enum"]
    #[serde(rename = "isPanoramaManaged", default, skip_serializing_if = "Option::is_none")]
    pub is_panorama_managed: Option<BooleanEnum>,
    #[doc = "Panorama Config"]
    #[serde(rename = "panoramaConfig", default, skip_serializing_if = "Option::is_none")]
    pub panorama_config: Option<PanoramaConfig>,
    #[doc = "Associated rulestack details"]
    #[serde(rename = "associatedRulestack", default, skip_serializing_if = "Option::is_none")]
    pub associated_rulestack: Option<RulestackDetails>,
    #[doc = "DNS Proxy settings for Firewall"]
    #[serde(rename = "dnsSettings", default, skip_serializing_if = "Option::is_none")]
    pub dns_settings: Option<DnsSettings>,
    #[doc = "Frontend settings for Firewall"]
    #[serde(
        rename = "frontEndSettings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub front_end_settings: Vec<FrontendSetting>,
    #[doc = "Billing plan information."]
    #[serde(rename = "planData", default, skip_serializing_if = "Option::is_none")]
    pub plan_data: Option<PlanData>,
    #[doc = "MarketplaceDetails of PAN Firewall resource"]
    #[serde(rename = "marketplaceDetails", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_details: Option<MarketplaceDetails>,
}
impl FirewallResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Firewall Status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallStatusProperty {
    #[doc = "Boolean Enum"]
    #[serde(rename = "isPanoramaManaged", default, skip_serializing_if = "Option::is_none")]
    pub is_panorama_managed: Option<BooleanEnum>,
    #[doc = "Status Codes for the Firewall"]
    #[serde(rename = "healthStatus", default, skip_serializing_if = "Option::is_none")]
    pub health_status: Option<HealthStatus>,
    #[doc = "Detail description of current health of the Firewall"]
    #[serde(rename = "healthReason", default, skip_serializing_if = "Option::is_none")]
    pub health_reason: Option<String>,
    #[doc = "Panorama connectivity information"]
    #[serde(rename = "panoramaStatus", default, skip_serializing_if = "Option::is_none")]
    pub panorama_status: Option<PanoramaStatus>,
    #[doc = "Provisioning state of the firewall resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ReadOnlyProvisioningState>,
}
impl FirewallStatusProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Firewall Status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallStatusResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Firewall Status"]
    pub properties: FirewallStatusProperty,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl FirewallStatusResource {
    pub fn new(properties: FirewallStatusProperty) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response of a FirewallStatusResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallStatusResourceListResult {
    #[doc = "The items on this page"]
    pub value: Vec<FirewallStatusResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FirewallStatusResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FirewallStatusResourceListResult {
    pub fn new(value: Vec<FirewallStatusResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "GlobalRulestack fqdnList"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FqdnListGlobalRulestackResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "fqdn object"]
    pub properties: FqdnObject,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl FqdnListGlobalRulestackResource {
    pub fn new(properties: FqdnObject) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response of a FqdnListGlobalRulestackResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FqdnListGlobalRulestackResourceListResult {
    #[doc = "The items on this page"]
    pub value: Vec<FqdnListGlobalRulestackResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FqdnListGlobalRulestackResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FqdnListGlobalRulestackResourceListResult {
    pub fn new(value: Vec<FqdnListGlobalRulestackResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "LocalRulestack fqdnList"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FqdnListLocalRulestackResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "fqdn object"]
    pub properties: FqdnObject,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl FqdnListLocalRulestackResource {
    pub fn new(properties: FqdnObject) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response of a FqdnListLocalRulestackResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FqdnListLocalRulestackResourceListResult {
    #[doc = "The items on this page"]
    pub value: Vec<FqdnListLocalRulestackResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FqdnListLocalRulestackResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FqdnListLocalRulestackResourceListResult {
    pub fn new(value: Vec<FqdnListLocalRulestackResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "fqdn object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FqdnObject {
    #[doc = "fqdn object description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "fqdn list"]
    #[serde(rename = "fqdnList")]
    pub fqdn_list: Vec<String>,
    #[doc = "etag info"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "comment for this object"]
    #[serde(rename = "auditComment", default, skip_serializing_if = "Option::is_none")]
    pub audit_comment: Option<String>,
    #[doc = "Provisioning state of the firewall resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl FqdnObject {
    pub fn new(fqdn_list: Vec<String>) -> Self {
        Self {
            description: None,
            fqdn_list,
            etag: None,
            audit_comment: None,
            provisioning_state: None,
        }
    }
}
#[doc = "Frontend setting for Firewall"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FrontendSetting {
    #[doc = "Settings name"]
    pub name: String,
    #[doc = "Protocol Enum"]
    pub protocol: ProtocolType,
    #[doc = "Endpoint Configuration for frontend and backend"]
    #[serde(rename = "frontendConfiguration")]
    pub frontend_configuration: EndpointConfiguration,
    #[doc = "Endpoint Configuration for frontend and backend"]
    #[serde(rename = "backendConfiguration")]
    pub backend_configuration: EndpointConfiguration,
}
impl FrontendSetting {
    pub fn new(
        name: String,
        protocol: ProtocolType,
        frontend_configuration: EndpointConfiguration,
        backend_configuration: EndpointConfiguration,
    ) -> Self {
        Self {
            name,
            protocol,
            frontend_configuration,
            backend_configuration,
        }
    }
}
#[doc = "PAN Rulestack Describe Object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalRulestackInfo {
    #[doc = "rulestack description"]
    #[serde(rename = "azureId")]
    pub azure_id: String,
}
impl GlobalRulestackInfo {
    pub fn new(azure_id: String) -> Self {
        Self { azure_id }
    }
}
#[doc = "PaloAltoNetworks GlobalRulestack"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalRulestackResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "PAN Rulestack Describe Object"]
    pub properties: RulestackProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Global Location"]
    pub location: String,
    #[doc = "The properties of the managed service identities assigned to this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<AzureResourceManagerManagedIdentityProperties>,
}
impl GlobalRulestackResource {
    pub fn new(properties: RulestackProperties, location: String) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
            location,
            identity: None,
        }
    }
}
#[doc = "The response of a GlobalRulestackResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalRulestackResourceListResult {
    #[doc = "The items on this page"]
    pub value: Vec<GlobalRulestackResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GlobalRulestackResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GlobalRulestackResourceListResult {
    pub fn new(value: Vec<GlobalRulestackResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the GlobalRulestackResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalRulestackResourceUpdate {
    #[doc = "Global Location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The properties of the managed service identities assigned to this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<AzureResourceManagerManagedIdentityProperties>,
    #[doc = "The updatable properties of the GlobalRulestackResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GlobalRulestackResourceUpdateProperties>,
}
impl GlobalRulestackResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the GlobalRulestackResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalRulestackResourceUpdateProperties {
    #[doc = "PanEtag info"]
    #[serde(rename = "panEtag", default, skip_serializing_if = "Option::is_none")]
    pub pan_etag: Option<String>,
    #[doc = "Rulestack Location, Required for GlobalRulestacks, Not for LocalRulestacks"]
    #[serde(rename = "panLocation", default, skip_serializing_if = "Option::is_none")]
    pub pan_location: Option<String>,
    #[doc = "Rulestack Type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeType>,
    #[doc = "subscription scope of global rulestack"]
    #[serde(
        rename = "associatedSubscriptions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub associated_subscriptions: Vec<String>,
    #[doc = "rulestack description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Type for Default Mode for rules creation"]
    #[serde(rename = "defaultMode", default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<DefaultMode>,
    #[doc = "minimum version"]
    #[serde(rename = "minAppIdVersion", default, skip_serializing_if = "Option::is_none")]
    pub min_app_id_version: Option<String>,
    #[doc = "security services"]
    #[serde(rename = "securityServices", default, skip_serializing_if = "Option::is_none")]
    pub security_services: Option<SecurityServices>,
}
impl GlobalRulestackResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status Codes for the Firewall"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HealthStatus")]
pub enum HealthStatus {
    #[serde(rename = "GREEN")]
    Green,
    #[serde(rename = "YELLOW")]
    Yellow,
    #[serde(rename = "RED")]
    Red,
    #[serde(rename = "INITIALIZING")]
    Initializing,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HealthStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HealthStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HealthStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Green => serializer.serialize_unit_variant("HealthStatus", 0u32, "GREEN"),
            Self::Yellow => serializer.serialize_unit_variant("HealthStatus", 1u32, "YELLOW"),
            Self::Red => serializer.serialize_unit_variant("HealthStatus", 2u32, "RED"),
            Self::Initializing => serializer.serialize_unit_variant("HealthStatus", 3u32, "INITIALIZING"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "IP Address"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpAddress {
    #[doc = "Resource Id"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Address value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}
impl IpAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IP Address Space"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpAddressSpace {
    #[doc = "Resource Id"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Address Space"]
    #[serde(rename = "addressSpace", default, skip_serializing_if = "Option::is_none")]
    pub address_space: Option<String>,
}
impl IpAddressSpace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListAppIdResponse {
    #[doc = "List of AppIds"]
    pub value: Vec<String>,
    #[doc = "next Link"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListAppIdResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ListAppIdResponse {
    pub fn new(value: Vec<String>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "List firewalls response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListFirewallsResponse {
    #[doc = "firewalls list"]
    pub value: Vec<String>,
    #[doc = "next link"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ListFirewallsResponse {
    pub fn new(value: Vec<String>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "LocalRulestack rule list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalRulesResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "definition of rule"]
    pub properties: RuleEntry,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl LocalRulesResource {
    pub fn new(properties: RuleEntry) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response of a LocalRulesResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalRulesResourceListResult {
    #[doc = "The items on this page"]
    pub value: Vec<LocalRulesResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LocalRulesResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LocalRulesResourceListResult {
    pub fn new(value: Vec<LocalRulesResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "PaloAltoNetworks LocalRulestack"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalRulestackResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "PAN Rulestack Describe Object"]
    pub properties: RulestackProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of the managed service identities assigned to this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<AzureResourceManagerManagedIdentityProperties>,
}
impl LocalRulestackResource {
    pub fn new(tracked_resource: TrackedResource, properties: RulestackProperties) -> Self {
        Self {
            tracked_resource,
            properties,
            system_data: None,
            identity: None,
        }
    }
}
#[doc = "The response of a LocalRulestackResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalRulestackResourceListResult {
    #[doc = "The items on this page"]
    pub value: Vec<LocalRulestackResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LocalRulestackResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LocalRulestackResourceListResult {
    pub fn new(value: Vec<LocalRulestackResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the LocalRulestackResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalRulestackResourceUpdate {
    #[doc = "The properties of the managed service identities assigned to this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<AzureResourceManagerManagedIdentityProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the LocalRulestackResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LocalRulestackResourceUpdateProperties>,
}
impl LocalRulestackResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the LocalRulestackResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalRulestackResourceUpdateProperties {
    #[doc = "PanEtag info"]
    #[serde(rename = "panEtag", default, skip_serializing_if = "Option::is_none")]
    pub pan_etag: Option<String>,
    #[doc = "Rulestack Location, Required for GlobalRulestacks, Not for LocalRulestacks"]
    #[serde(rename = "panLocation", default, skip_serializing_if = "Option::is_none")]
    pub pan_location: Option<String>,
    #[doc = "Rulestack Type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeType>,
    #[doc = "subscription scope of global rulestack"]
    #[serde(
        rename = "associatedSubscriptions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub associated_subscriptions: Vec<String>,
    #[doc = "rulestack description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Type for Default Mode for rules creation"]
    #[serde(rename = "defaultMode", default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<DefaultMode>,
    #[doc = "minimum version"]
    #[serde(rename = "minAppIdVersion", default, skip_serializing_if = "Option::is_none")]
    pub min_app_id_version: Option<String>,
    #[doc = "security services"]
    #[serde(rename = "securityServices", default, skip_serializing_if = "Option::is_none")]
    pub security_services: Option<SecurityServices>,
}
impl LocalRulestackResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Log Destination"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogDestination {
    #[doc = "Storage Account configurations"]
    #[serde(rename = "storageConfigurations", default, skip_serializing_if = "Option::is_none")]
    pub storage_configurations: Option<StorageAccount>,
    #[doc = "EventHub configurations"]
    #[serde(rename = "eventHubConfigurations", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_configurations: Option<EventHub>,
    #[doc = "MonitorLog configurations"]
    #[serde(rename = "monitorConfigurations", default, skip_serializing_if = "Option::is_none")]
    pub monitor_configurations: Option<MonitorLog>,
}
impl LogDestination {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Log options possible"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LogOption")]
pub enum LogOption {
    #[serde(rename = "SAME_DESTINATION")]
    SameDestination,
    #[serde(rename = "INDIVIDUAL_DESTINATION")]
    IndividualDestination,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LogOption {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LogOption {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LogOption {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SameDestination => serializer.serialize_unit_variant("LogOption", 0u32, "SAME_DESTINATION"),
            Self::IndividualDestination => serializer.serialize_unit_variant("LogOption", 1u32, "INDIVIDUAL_DESTINATION"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Log Settings for Firewall"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSettings {
    #[doc = "Possible log types"]
    #[serde(rename = "logType", default, skip_serializing_if = "Option::is_none")]
    pub log_type: Option<LogType>,
    #[doc = "Log options possible"]
    #[serde(rename = "logOption", default, skip_serializing_if = "Option::is_none")]
    pub log_option: Option<LogOption>,
    #[doc = "Application Insights key"]
    #[serde(rename = "applicationInsights", default, skip_serializing_if = "Option::is_none")]
    pub application_insights: Option<ApplicationInsights>,
    #[doc = "Log Destination"]
    #[serde(rename = "commonDestination", default, skip_serializing_if = "Option::is_none")]
    pub common_destination: Option<LogDestination>,
    #[doc = "Log Destination"]
    #[serde(rename = "trafficLogDestination", default, skip_serializing_if = "Option::is_none")]
    pub traffic_log_destination: Option<LogDestination>,
    #[doc = "Log Destination"]
    #[serde(rename = "threatLogDestination", default, skip_serializing_if = "Option::is_none")]
    pub threat_log_destination: Option<LogDestination>,
    #[doc = "Log Destination"]
    #[serde(rename = "decryptLogDestination", default, skip_serializing_if = "Option::is_none")]
    pub decrypt_log_destination: Option<LogDestination>,
}
impl LogSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Possible log types"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LogType")]
pub enum LogType {
    #[serde(rename = "TRAFFIC")]
    Traffic,
    #[serde(rename = "THREAT")]
    Threat,
    #[serde(rename = "DECRYPTION")]
    Decryption,
    #[serde(rename = "WILDFIRE")]
    Wildfire,
    #[serde(rename = "DLP")]
    Dlp,
    #[serde(rename = "AUDIT")]
    Audit,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LogType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LogType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LogType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Traffic => serializer.serialize_unit_variant("LogType", 0u32, "TRAFFIC"),
            Self::Threat => serializer.serialize_unit_variant("LogType", 1u32, "THREAT"),
            Self::Decryption => serializer.serialize_unit_variant("LogType", 2u32, "DECRYPTION"),
            Self::Wildfire => serializer.serialize_unit_variant("LogType", 3u32, "WILDFIRE"),
            Self::Dlp => serializer.serialize_unit_variant("LogType", 4u32, "DLP"),
            Self::Audit => serializer.serialize_unit_variant("LogType", 5u32, "AUDIT"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "MarketplaceDetails of PAN Firewall resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceDetails {
    #[doc = "Marketplace Subscription Id"]
    #[serde(rename = "marketplaceSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_subscription_id: Option<String>,
    #[doc = "Offer Id"]
    #[serde(rename = "offerId")]
    pub offer_id: String,
    #[doc = "Publisher Id"]
    #[serde(rename = "publisherId")]
    pub publisher_id: String,
    #[doc = "Marketplace Subscription Status"]
    #[serde(rename = "marketplaceSubscriptionStatus", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_subscription_status: Option<MarketplaceSubscriptionStatus>,
}
impl MarketplaceDetails {
    pub fn new(offer_id: String, publisher_id: String) -> Self {
        Self {
            marketplace_subscription_id: None,
            offer_id,
            publisher_id,
            marketplace_subscription_status: None,
        }
    }
}
#[doc = "Marketplace Subscription Status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MarketplaceSubscriptionStatus")]
pub enum MarketplaceSubscriptionStatus {
    PendingFulfillmentStart,
    Subscribed,
    Suspended,
    Unsubscribed,
    NotStarted,
    FulfillmentRequested,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MarketplaceSubscriptionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MarketplaceSubscriptionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MarketplaceSubscriptionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::PendingFulfillmentStart => {
                serializer.serialize_unit_variant("MarketplaceSubscriptionStatus", 0u32, "PendingFulfillmentStart")
            }
            Self::Subscribed => serializer.serialize_unit_variant("MarketplaceSubscriptionStatus", 1u32, "Subscribed"),
            Self::Suspended => serializer.serialize_unit_variant("MarketplaceSubscriptionStatus", 2u32, "Suspended"),
            Self::Unsubscribed => serializer.serialize_unit_variant("MarketplaceSubscriptionStatus", 3u32, "Unsubscribed"),
            Self::NotStarted => serializer.serialize_unit_variant("MarketplaceSubscriptionStatus", 4u32, "NotStarted"),
            Self::FulfillmentRequested => serializer.serialize_unit_variant("MarketplaceSubscriptionStatus", 5u32, "FulfillmentRequested"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "MonitorLog configurations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorLog {
    #[doc = "Resource ID of MonitorLog"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Subscription Id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "MonitorLog workspace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    #[doc = "Primary Key value for Monitor"]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "Secondary Key value for Monitor"]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
}
impl MonitorLog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "object type info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameDescriptionObject {
    #[doc = "name value"]
    pub name: String,
    #[doc = "description value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl NameDescriptionObject {
    pub fn new(name: String) -> Self {
        Self { name, description: None }
    }
}
#[doc = "Network settings for Firewall"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkProfile {
    #[doc = "VnetInfo for Firewall Networking"]
    #[serde(rename = "vnetConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub vnet_configuration: Option<VnetConfiguration>,
    #[doc = "VwanInfo for Firewall Networking"]
    #[serde(rename = "vwanConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub vwan_configuration: Option<VwanConfiguration>,
    #[doc = "NetworkType Enum"]
    #[serde(rename = "networkType")]
    pub network_type: NetworkType,
    #[doc = "List of IPs associated with the Firewall"]
    #[serde(rename = "publicIps")]
    pub public_ips: Vec<IpAddress>,
    #[doc = "Egress NAT"]
    #[serde(rename = "enableEgressNat")]
    pub enable_egress_nat: EgressNat,
    #[doc = "Egress nat IP to use"]
    #[serde(
        rename = "egressNatIp",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub egress_nat_ip: Vec<IpAddress>,
    #[doc = "Non-RFC 1918 address"]
    #[serde(
        rename = "trustedRanges",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trusted_ranges: Vec<String>,
    #[doc = "Array of ipv4 destination address for which source NAT is to be performed"]
    #[serde(
        rename = "privateSourceNatRulesDestination",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_source_nat_rules_destination: Vec<String>,
}
impl NetworkProfile {
    pub fn new(network_type: NetworkType, public_ips: Vec<IpAddress>, enable_egress_nat: EgressNat) -> Self {
        Self {
            vnet_configuration: None,
            vwan_configuration: None,
            network_type,
            public_ips,
            enable_egress_nat,
            egress_nat_ip: Vec::new(),
            trusted_ranges: Vec::new(),
            private_source_nat_rules_destination: Vec::new(),
        }
    }
}
#[doc = "NetworkType Enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NetworkType")]
pub enum NetworkType {
    #[serde(rename = "VNET")]
    Vnet,
    #[serde(rename = "VWAN")]
    Vwan,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NetworkType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NetworkType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NetworkType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Vnet => serializer.serialize_unit_variant("NetworkType", 0u32, "VNET"),
            Self::Vwan => serializer.serialize_unit_variant("NetworkType", 1u32, "VWAN"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Details of a REST API operation, returned from the Resource Provider Operations API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation, as per Resource-Based Access Control (RBAC). Examples: \"Microsoft.Compute/virtualMachines/write\", \"Microsoft.Compute/virtualMachines/capture/action\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether the operation applies to data-plane. This is \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Localized display information for this particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Localized display information for this particular operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name, e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly name of the resource type related to this operation. E.g. \"Virtual Machines\" or \"Job Schedule Collections\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The concise, localized friendly name for the operation; suitable for dropdowns. E.g. \"Create or Update Virtual Machine\", \"Restart Virtual Machine\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The short, localized friendly description of the operation; suitable for tool tips and detailed views."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Origin {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Origin {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Origin {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
                Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
                Self::UserSystem => serializer.serialize_unit_variant("Origin", 2u32, "user,system"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        Internal,
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
                Self::Internal => serializer.serialize_unit_variant("ActionType", 0u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of REST API operations supported by an Azure Resource Provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Panorama Config"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PanoramaConfig {
    #[doc = "Base64 encoded string representing Panorama parameters to be used by Firewall to connect to Panorama. This string is generated via azure plugin in Panorama"]
    #[serde(rename = "configString")]
    pub config_string: String,
    #[doc = "VM auth key for panorama connectivity"]
    #[serde(rename = "vmAuthKey", default, skip_serializing_if = "Option::is_none")]
    pub vm_auth_key: Option<String>,
    #[doc = "Primary Panorama Server IP address value in dotted format for IPv4"]
    #[serde(rename = "panoramaServer", default, skip_serializing_if = "Option::is_none")]
    pub panorama_server: Option<String>,
    #[doc = "Secondary Panorama Server IP address value in dotted format for IPv4"]
    #[serde(rename = "panoramaServer2", default, skip_serializing_if = "Option::is_none")]
    pub panorama_server2: Option<String>,
    #[doc = "Panorama Device Group to join"]
    #[serde(rename = "dgName", default, skip_serializing_if = "Option::is_none")]
    pub dg_name: Option<String>,
    #[doc = "Panorama Template Stack to join - (Once configured we can not edit the value)"]
    #[serde(rename = "tplName", default, skip_serializing_if = "Option::is_none")]
    pub tpl_name: Option<String>,
    #[doc = "Panorama Collector Group to join - (Once configured we can not edit the value)"]
    #[serde(rename = "cgName", default, skip_serializing_if = "Option::is_none")]
    pub cg_name: Option<String>,
    #[doc = "Resource name(may be unique) for PN admin"]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
}
impl PanoramaConfig {
    pub fn new(config_string: String) -> Self {
        Self {
            config_string,
            vm_auth_key: None,
            panorama_server: None,
            panorama_server2: None,
            dg_name: None,
            tpl_name: None,
            cg_name: None,
            host_name: None,
        }
    }
}
#[doc = "Panorama connectivity information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PanoramaStatus {
    #[doc = "Connectivity Status for Panorama Server"]
    #[serde(rename = "panoramaServerStatus", default, skip_serializing_if = "Option::is_none")]
    pub panorama_server_status: Option<ServerStatus>,
    #[doc = "Connectivity Status for Panorama Server"]
    #[serde(rename = "panoramaServer2Status", default, skip_serializing_if = "Option::is_none")]
    pub panorama_server2_status: Option<ServerStatus>,
}
impl PanoramaStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Billing plan information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanData {
    #[doc = "Usage Type"]
    #[serde(rename = "usageType", default, skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<UsageType>,
    #[doc = "Billing cycle"]
    #[serde(rename = "billingCycle")]
    pub billing_cycle: BillingCycle,
    #[doc = "plan id as published by Liftr.PAN"]
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[doc = "date when plan was applied"]
    #[serde(rename = "effectiveDate", default, with = "azure_core::date::rfc3339::option")]
    pub effective_date: Option<time::OffsetDateTime>,
}
impl PlanData {
    pub fn new(billing_cycle: BillingCycle, plan_id: String) -> Self {
        Self {
            usage_type: None,
            billing_cycle,
            plan_id,
            effective_date: None,
        }
    }
}
#[doc = "PostRulestack rule list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostRulesResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "definition of rule"]
    pub properties: RuleEntry,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PostRulesResource {
    pub fn new(properties: RuleEntry) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response of a PostRulesResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostRulesResourceListResult {
    #[doc = "The items on this page"]
    pub value: Vec<PostRulesResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PostRulesResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PostRulesResourceListResult {
    pub fn new(value: Vec<PostRulesResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "PreRulestack rule list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreRulesResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "definition of rule"]
    pub properties: RuleEntry,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PreRulesResource {
    pub fn new(properties: RuleEntry) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response of a PreRulesResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreRulesResourceListResult {
    #[doc = "The items on this page"]
    pub value: Vec<PreRulesResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PreRulesResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PreRulesResourceListResult {
    pub fn new(value: Vec<PreRulesResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "predefined url categories response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PredefinedUrlCategoriesResponse {
    #[doc = "predefined url categories"]
    pub value: Vec<PredefinedUrlCategory>,
    #[doc = "next link"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PredefinedUrlCategoriesResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PredefinedUrlCategoriesResponse {
    pub fn new(value: Vec<PredefinedUrlCategory>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Predefined URL category object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PredefinedUrlCategory {
    pub action: String,
    pub name: String,
}
impl PredefinedUrlCategory {
    pub fn new(action: String, name: String) -> Self {
        Self { action, name }
    }
}
#[doc = "GlobalRulestack prefixList"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrefixListGlobalRulestackResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "prefix entry"]
    pub properties: PrefixObject,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PrefixListGlobalRulestackResource {
    pub fn new(properties: PrefixObject) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response of a PrefixListGlobalRulestackResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrefixListGlobalRulestackResourceListResult {
    #[doc = "The items on this page"]
    pub value: Vec<PrefixListGlobalRulestackResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrefixListGlobalRulestackResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PrefixListGlobalRulestackResourceListResult {
    pub fn new(value: Vec<PrefixListGlobalRulestackResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "LocalRulestack prefixList"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrefixListResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "prefix entry"]
    pub properties: PrefixObject,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PrefixListResource {
    pub fn new(properties: PrefixObject) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response of a PrefixListResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrefixListResourceListResult {
    #[doc = "The items on this page"]
    pub value: Vec<PrefixListResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrefixListResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PrefixListResourceListResult {
    pub fn new(value: Vec<PrefixListResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "prefix entry"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrefixObject {
    #[doc = "prefix description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "prefix list"]
    #[serde(rename = "prefixList")]
    pub prefix_list: Vec<String>,
    #[doc = "etag info"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "comment for this object"]
    #[serde(rename = "auditComment", default, skip_serializing_if = "Option::is_none")]
    pub audit_comment: Option<String>,
    #[doc = "Provisioning state of the firewall resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl PrefixObject {
    pub fn new(prefix_list: Vec<String>) -> Self {
        Self {
            description: None,
            prefix_list,
            etag: None,
            audit_comment: None,
            provisioning_state: None,
        }
    }
}
#[doc = "Protocol Enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProtocolType")]
pub enum ProtocolType {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
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
            Self::Tcp => serializer.serialize_unit_variant("ProtocolType", 0u32, "TCP"),
            Self::Udp => serializer.serialize_unit_variant("ProtocolType", 1u32, "UDP"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Provisioning state of the firewall resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Accepted,
    Creating,
    Updating,
    Deleting,
    Succeeded,
    Failed,
    Canceled,
    Deleted,
    NotSpecified,
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
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Accepted"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Canceled"),
            Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleted"),
            Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 8u32, "NotSpecified"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition for a Azure Resource Manager proxy resource. It will not have tags and a location"]
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
#[doc = "Provisioning state of the firewall resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReadOnlyProvisioningState")]
pub enum ReadOnlyProvisioningState {
    Succeeded,
    Failed,
    Deleted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReadOnlyProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReadOnlyProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReadOnlyProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("ReadOnlyProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ReadOnlyProvisioningState", 1u32, "Failed"),
            Self::Deleted => serializer.serialize_unit_variant("ReadOnlyProvisioningState", 2u32, "Deleted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rule counter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleCounter {
    #[doc = "priority number"]
    pub priority: String,
    #[doc = "rule Stack Name"]
    #[serde(rename = "ruleStackName", default, skip_serializing_if = "Option::is_none")]
    pub rule_stack_name: Option<String>,
    #[doc = "rule list name"]
    #[serde(rename = "ruleListName", default, skip_serializing_if = "Option::is_none")]
    pub rule_list_name: Option<String>,
    #[doc = "firewall name"]
    #[serde(rename = "firewallName", default, skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[doc = "rule name"]
    #[serde(rename = "ruleName")]
    pub rule_name: String,
    #[doc = "hit count"]
    #[serde(rename = "hitCount", default, skip_serializing_if = "Option::is_none")]
    pub hit_count: Option<i32>,
    #[doc = "Data Type for App Seen"]
    #[serde(rename = "appSeen", default, skip_serializing_if = "Option::is_none")]
    pub app_seen: Option<AppSeenData>,
    #[doc = "timestamp of response"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "timestamp of request"]
    #[serde(rename = "requestTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub request_timestamp: Option<time::OffsetDateTime>,
    #[doc = "last updated timestamp"]
    #[serde(rename = "lastUpdatedTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_timestamp: Option<time::OffsetDateTime>,
}
impl RuleCounter {
    pub fn new(priority: String, rule_name: String) -> Self {
        Self {
            priority,
            rule_stack_name: None,
            rule_list_name: None,
            firewall_name: None,
            rule_name,
            hit_count: None,
            app_seen: None,
            timestamp: None,
            request_timestamp: None,
            last_updated_timestamp: None,
        }
    }
}
#[doc = "Rule counter reset"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleCounterReset {
    #[doc = "priority number"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[doc = "rule Stack Name"]
    #[serde(rename = "ruleStackName", default, skip_serializing_if = "Option::is_none")]
    pub rule_stack_name: Option<String>,
    #[doc = "rule list name"]
    #[serde(rename = "ruleListName", default, skip_serializing_if = "Option::is_none")]
    pub rule_list_name: Option<String>,
    #[doc = "firewall name"]
    #[serde(rename = "firewallName", default, skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[doc = "rule name"]
    #[serde(rename = "ruleName", default, skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}
impl RuleCounterReset {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "definition of rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleEntry {
    #[doc = "etag info"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "rule name"]
    #[serde(rename = "ruleName")]
    pub rule_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "rule description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Enabled or Disabled Enum"]
    #[serde(rename = "ruleState", default, skip_serializing_if = "Option::is_none")]
    pub rule_state: Option<StateEnum>,
    #[doc = "Address properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceAddr>,
    #[doc = "Boolean Enum"]
    #[serde(rename = "negateSource", default, skip_serializing_if = "Option::is_none")]
    pub negate_source: Option<BooleanEnum>,
    #[doc = "destination address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<DestinationAddr>,
    #[doc = "Boolean Enum"]
    #[serde(rename = "negateDestination", default, skip_serializing_if = "Option::is_none")]
    pub negate_destination: Option<BooleanEnum>,
    #[doc = "array of rule applications"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub applications: Vec<String>,
    #[doc = "URL/EDL to match"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    #[doc = "any, application-default, TCP:number, UDP:number"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[doc = "prot port list"]
    #[serde(
        rename = "protocolPortList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub protocol_port_list: Vec<String>,
    #[doc = "inbound Inspection Certificate"]
    #[serde(rename = "inboundInspectionCertificate", default, skip_serializing_if = "Option::is_none")]
    pub inbound_inspection_certificate: Option<String>,
    #[doc = "rule comment"]
    #[serde(rename = "auditComment", default, skip_serializing_if = "Option::is_none")]
    pub audit_comment: Option<String>,
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<ActionEnum>,
    #[doc = "Enabled or Disabled Enum"]
    #[serde(rename = "enableLogging", default, skip_serializing_if = "Option::is_none")]
    pub enable_logging: Option<StateEnum>,
    #[serde(rename = "decryptionRuleType", default, skip_serializing_if = "Option::is_none")]
    pub decryption_rule_type: Option<DecryptionRuleTypeEnum>,
    #[doc = "tag for rule"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<TagInfo>,
    #[doc = "Provisioning state of the firewall resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl RuleEntry {
    pub fn new(rule_name: String) -> Self {
        Self {
            etag: None,
            rule_name,
            priority: None,
            description: None,
            rule_state: None,
            source: None,
            negate_source: None,
            destination: None,
            negate_destination: None,
            applications: Vec::new(),
            category: None,
            protocol: None,
            protocol_port_list: Vec::new(),
            inbound_inspection_certificate: None,
            audit_comment: None,
            action_type: None,
            enable_logging: None,
            decryption_rule_type: None,
            tags: Vec::new(),
            provisioning_state: None,
        }
    }
}
#[doc = "Associated rulestack details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RulestackDetails {
    #[doc = "Resource Id"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Associated rulestack Id"]
    #[serde(rename = "rulestackId", default, skip_serializing_if = "Option::is_none")]
    pub rulestack_id: Option<String>,
    #[doc = "Rulestack location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl RulestackDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "PAN Rulestack Describe Object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RulestackProperties {
    #[doc = "PanEtag info"]
    #[serde(rename = "panEtag", default, skip_serializing_if = "Option::is_none")]
    pub pan_etag: Option<String>,
    #[doc = "Rulestack Location, Required for GlobalRulestacks, Not for LocalRulestacks"]
    #[serde(rename = "panLocation", default, skip_serializing_if = "Option::is_none")]
    pub pan_location: Option<String>,
    #[doc = "Rulestack Type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeType>,
    #[doc = "subscription scope of global rulestack"]
    #[serde(
        rename = "associatedSubscriptions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub associated_subscriptions: Vec<String>,
    #[doc = "rulestack description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Type for Default Mode for rules creation"]
    #[serde(rename = "defaultMode", default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<DefaultMode>,
    #[doc = "minimum version"]
    #[serde(rename = "minAppIdVersion", default, skip_serializing_if = "Option::is_none")]
    pub min_app_id_version: Option<String>,
    #[doc = "Provisioning state of the firewall resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "security services"]
    #[serde(rename = "securityServices", default, skip_serializing_if = "Option::is_none")]
    pub security_services: Option<SecurityServices>,
}
impl RulestackProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rulestack Type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScopeType")]
pub enum ScopeType {
    #[serde(rename = "LOCAL")]
    Local,
    #[serde(rename = "GLOBAL")]
    Global,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScopeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScopeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScopeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Local => serializer.serialize_unit_variant("ScopeType", 0u32, "LOCAL"),
            Self::Global => serializer.serialize_unit_variant("ScopeType", 1u32, "GLOBAL"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SecurityServicesTypeEnum")]
pub enum SecurityServicesTypeEnum {
    #[serde(rename = "antiSpyware")]
    AntiSpyware,
    #[serde(rename = "antiVirus")]
    AntiVirus,
    #[serde(rename = "ipsVulnerability")]
    IpsVulnerability,
    #[serde(rename = "urlFiltering")]
    UrlFiltering,
    #[serde(rename = "fileBlocking")]
    FileBlocking,
    #[serde(rename = "dnsSubscription")]
    DnsSubscription,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SecurityServicesTypeEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SecurityServicesTypeEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SecurityServicesTypeEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AntiSpyware => serializer.serialize_unit_variant("SecurityServicesTypeEnum", 0u32, "antiSpyware"),
            Self::AntiVirus => serializer.serialize_unit_variant("SecurityServicesTypeEnum", 1u32, "antiVirus"),
            Self::IpsVulnerability => serializer.serialize_unit_variant("SecurityServicesTypeEnum", 2u32, "ipsVulnerability"),
            Self::UrlFiltering => serializer.serialize_unit_variant("SecurityServicesTypeEnum", 3u32, "urlFiltering"),
            Self::FileBlocking => serializer.serialize_unit_variant("SecurityServicesTypeEnum", 4u32, "fileBlocking"),
            Self::DnsSubscription => serializer.serialize_unit_variant("SecurityServicesTypeEnum", 5u32, "dnsSubscription"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Connectivity Status for Panorama Server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServerStatus")]
pub enum ServerStatus {
    #[serde(rename = "UP")]
    Up,
    #[serde(rename = "DOWN")]
    Down,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServerStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServerStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServerStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Up => serializer.serialize_unit_variant("ServerStatus", 0u32, "UP"),
            Self::Down => serializer.serialize_unit_variant("ServerStatus", 1u32, "DOWN"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Address properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceAddr {
    #[doc = "special value 'any'"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cidrs: Vec<String>,
    #[doc = "list of countries"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub countries: Vec<String>,
    #[doc = "list of feeds"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub feeds: Vec<String>,
    #[doc = "prefix list"]
    #[serde(
        rename = "prefixLists",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub prefix_lists: Vec<String>,
}
impl SourceAddr {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enabled or Disabled Enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StateEnum")]
pub enum StateEnum {
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StateEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StateEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StateEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("StateEnum", 0u32, "DISABLED"),
            Self::Enabled => serializer.serialize_unit_variant("StateEnum", 1u32, "ENABLED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Storage Account configurations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccount {
    #[doc = "Resource ID of storage account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Subscription Id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Storage account name"]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
}
impl StorageAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Support information for the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportInfo {
    #[doc = "product SKU associated with given resource"]
    #[serde(rename = "productSku", default, skip_serializing_if = "Option::is_none")]
    pub product_sku: Option<String>,
    #[doc = "product Serial associated with given resource"]
    #[serde(rename = "productSerial", default, skip_serializing_if = "Option::is_none")]
    pub product_serial: Option<String>,
    #[doc = "Boolean Enum"]
    #[serde(rename = "accountRegistered", default, skip_serializing_if = "Option::is_none")]
    pub account_registered: Option<BooleanEnum>,
    #[doc = "Support account associated with given resource"]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "Boolean Enum"]
    #[serde(rename = "userDomainSupported", default, skip_serializing_if = "Option::is_none")]
    pub user_domain_supported: Option<BooleanEnum>,
    #[doc = "Boolean Enum"]
    #[serde(rename = "userRegistered", default, skip_serializing_if = "Option::is_none")]
    pub user_registered: Option<BooleanEnum>,
    #[doc = "Boolean Enum"]
    #[serde(rename = "freeTrial", default, skip_serializing_if = "Option::is_none")]
    pub free_trial: Option<BooleanEnum>,
    #[doc = "Free trial days remaining"]
    #[serde(rename = "freeTrialDaysLeft", default, skip_serializing_if = "Option::is_none")]
    pub free_trial_days_left: Option<i32>,
    #[doc = "Free trial credit remaining"]
    #[serde(rename = "freeTrialCreditLeft", default, skip_serializing_if = "Option::is_none")]
    pub free_trial_credit_left: Option<i32>,
    #[doc = "URL for paloaltonetworks live community"]
    #[serde(rename = "helpURL", default, skip_serializing_if = "Option::is_none")]
    pub help_url: Option<String>,
    #[doc = "URL for paloaltonetworks Customer Service Portal"]
    #[serde(rename = "supportURL", default, skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
    #[doc = "URL for registering product in paloaltonetworks Customer Service Portal"]
    #[serde(rename = "registerURL", default, skip_serializing_if = "Option::is_none")]
    pub register_url: Option<String>,
}
impl SupportInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagInfo {
    #[doc = "tag name"]
    pub key: String,
    #[doc = "tag value"]
    pub value: String,
}
impl TagInfo {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[doc = "Usage Type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UsageType")]
pub enum UsageType {
    #[serde(rename = "PAYG")]
    Payg,
    #[serde(rename = "COMMITTED")]
    Committed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UsageType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UsageType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UsageType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Payg => serializer.serialize_unit_variant("UsageType", 0u32, "PAYG"),
            Self::Committed => serializer.serialize_unit_variant("UsageType", 1u32, "COMMITTED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "VnetInfo for Firewall Networking"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VnetConfiguration {
    #[doc = "IP Address Space"]
    pub vnet: IpAddressSpace,
    #[doc = "IP Address Space"]
    #[serde(rename = "trustSubnet")]
    pub trust_subnet: IpAddressSpace,
    #[doc = "IP Address Space"]
    #[serde(rename = "unTrustSubnet")]
    pub un_trust_subnet: IpAddressSpace,
    #[doc = "IP Address"]
    #[serde(rename = "ipOfTrustSubnetForUdr", default, skip_serializing_if = "Option::is_none")]
    pub ip_of_trust_subnet_for_udr: Option<IpAddress>,
}
impl VnetConfiguration {
    pub fn new(vnet: IpAddressSpace, trust_subnet: IpAddressSpace, un_trust_subnet: IpAddressSpace) -> Self {
        Self {
            vnet,
            trust_subnet,
            un_trust_subnet,
            ip_of_trust_subnet_for_udr: None,
        }
    }
}
#[doc = "VwanInfo for Firewall Networking"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VwanConfiguration {
    #[doc = "Network Virtual Appliance resource ID "]
    #[serde(rename = "networkVirtualApplianceId", default, skip_serializing_if = "Option::is_none")]
    pub network_virtual_appliance_id: Option<String>,
    #[doc = "IP Address Space"]
    #[serde(rename = "vHub")]
    pub v_hub: IpAddressSpace,
    #[doc = "IP Address Space"]
    #[serde(rename = "trustSubnet", default, skip_serializing_if = "Option::is_none")]
    pub trust_subnet: Option<IpAddressSpace>,
    #[doc = "IP Address Space"]
    #[serde(rename = "unTrustSubnet", default, skip_serializing_if = "Option::is_none")]
    pub un_trust_subnet: Option<IpAddressSpace>,
    #[doc = "IP Address"]
    #[serde(rename = "ipOfTrustSubnetForUdr", default, skip_serializing_if = "Option::is_none")]
    pub ip_of_trust_subnet_for_udr: Option<IpAddress>,
}
impl VwanConfiguration {
    pub fn new(v_hub: IpAddressSpace) -> Self {
        Self {
            network_virtual_appliance_id: None,
            v_hub,
            trust_subnet: None,
            un_trust_subnet: None,
            ip_of_trust_subnet_for_udr: None,
        }
    }
}
#[doc = "advanced security object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdvSecurityObjectListResponse {
    #[doc = "List of custom and predefined url category"]
    pub value: AdvSecurityObjectModel,
    #[doc = "next link"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AdvSecurityObjectListResponse {
    pub fn new(value: AdvSecurityObjectModel) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "List of custom and predefined url category"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdvSecurityObjectModel {
    #[doc = "type of object"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "URL entry"]
    pub entry: Vec<NameDescriptionObject>,
}
impl AdvSecurityObjectModel {
    pub fn new(entry: Vec<NameDescriptionObject>) -> Self {
        Self { type_: None, entry }
    }
}
#[doc = "security services"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityServices {
    #[doc = "IPs Vulnerability Profile Data"]
    #[serde(rename = "vulnerabilityProfile", default, skip_serializing_if = "Option::is_none")]
    pub vulnerability_profile: Option<String>,
    #[doc = "Anti spyware Profile data"]
    #[serde(rename = "antiSpywareProfile", default, skip_serializing_if = "Option::is_none")]
    pub anti_spyware_profile: Option<String>,
    #[doc = "anti virus profile data"]
    #[serde(rename = "antiVirusProfile", default, skip_serializing_if = "Option::is_none")]
    pub anti_virus_profile: Option<String>,
    #[doc = "URL filtering profile data"]
    #[serde(rename = "urlFilteringProfile", default, skip_serializing_if = "Option::is_none")]
    pub url_filtering_profile: Option<String>,
    #[doc = "File blocking profile data"]
    #[serde(rename = "fileBlockingProfile", default, skip_serializing_if = "Option::is_none")]
    pub file_blocking_profile: Option<String>,
    #[doc = "DNS Subscription profile data"]
    #[serde(rename = "dnsSubscription", default, skip_serializing_if = "Option::is_none")]
    pub dns_subscription: Option<String>,
    #[doc = "Untrusted Egress Decryption profile data"]
    #[serde(rename = "outboundUnTrustCertificate", default, skip_serializing_if = "Option::is_none")]
    pub outbound_un_trust_certificate: Option<String>,
    #[doc = "Trusted Egress Decryption profile data"]
    #[serde(rename = "outboundTrustCertificate", default, skip_serializing_if = "Option::is_none")]
    pub outbound_trust_certificate: Option<String>,
}
impl SecurityServices {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Security services list response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityServicesResponse {
    #[doc = "Security services type list"]
    pub value: SecurityServicesTypeList,
    #[doc = "next link"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SecurityServicesResponse {
    pub fn new(value: SecurityServicesTypeList) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Security services type list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityServicesTypeList {
    #[doc = "security services type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "list"]
    pub entry: Vec<NameDescriptionObject>,
}
impl SecurityServicesTypeList {
    pub fn new(entry: Vec<NameDescriptionObject>) -> Self {
        Self { type_: None, entry }
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreatedByType")]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreatedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreatedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreatedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("CreatedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("CreatedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("CreatedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("CreatedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastModifiedByType")]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastModifiedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastModifiedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastModifiedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("LastModifiedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("LastModifiedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("LastModifiedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("LastModifiedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}

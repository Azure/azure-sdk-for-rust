#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Domain Security Settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainSecuritySettings {
    #[doc = "A flag to determine whether or not NtlmV1 is enabled or disabled."]
    #[serde(rename = "ntlmV1", default, skip_serializing_if = "Option::is_none")]
    pub ntlm_v1: Option<domain_security_settings::NtlmV1>,
    #[doc = "A flag to determine whether or not TlsV1 is enabled or disabled."]
    #[serde(rename = "tlsV1", default, skip_serializing_if = "Option::is_none")]
    pub tls_v1: Option<domain_security_settings::TlsV1>,
    #[doc = "A flag to determine whether or not SyncNtlmPasswords is enabled or disabled."]
    #[serde(rename = "syncNtlmPasswords", default, skip_serializing_if = "Option::is_none")]
    pub sync_ntlm_passwords: Option<domain_security_settings::SyncNtlmPasswords>,
}
impl DomainSecuritySettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod domain_security_settings {
    use super::*;
    #[doc = "A flag to determine whether or not NtlmV1 is enabled or disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NtlmV1")]
    pub enum NtlmV1 {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NtlmV1 {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NtlmV1 {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NtlmV1 {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("NtlmV1", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("NtlmV1", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A flag to determine whether or not TlsV1 is enabled or disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TlsV1")]
    pub enum TlsV1 {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TlsV1 {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TlsV1 {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TlsV1 {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("TlsV1", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("TlsV1", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A flag to determine whether or not SyncNtlmPasswords is enabled or disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SyncNtlmPasswords")]
    pub enum SyncNtlmPasswords {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SyncNtlmPasswords {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SyncNtlmPasswords {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SyncNtlmPasswords {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("SyncNtlmPasswords", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("SyncNtlmPasswords", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Domain service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainService {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the Domain Service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DomainServiceProperties>,
}
impl DomainService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from the List Domain Services operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainServiceListResult {
    #[doc = "the list of domain services."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DomainService>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DomainServiceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DomainServiceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the Domain Service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainServiceProperties {
    #[doc = "Azure Active Directory tenant id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The name of the Azure domain that the user would like to deploy Domain Services to."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "Deployment Id"]
    #[serde(rename = "deploymentId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[doc = "Virtual network site id"]
    #[serde(rename = "vnetSiteId", default, skip_serializing_if = "Option::is_none")]
    pub vnet_site_id: Option<String>,
    #[doc = "The name of the virtual network that Domain Services will be deployed on. The id of the subnet that Domain Services will be deployed on. /virtualNetwork/vnetName/subnets/subnetName."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "Secure LDAP Settings"]
    #[serde(rename = "ldapsSettings", default, skip_serializing_if = "Option::is_none")]
    pub ldaps_settings: Option<LdapsSettings>,
    #[doc = "Last domain evaluation run DateTime"]
    #[serde(rename = "healthLastEvaluated", default, with = "azure_core::date::rfc1123::option")]
    pub health_last_evaluated: Option<time::OffsetDateTime>,
    #[doc = "List of Domain Health Monitors"]
    #[serde(
        rename = "healthMonitors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub health_monitors: Vec<HealthMonitor>,
    #[doc = "List of Domain Health Alerts"]
    #[serde(
        rename = "healthAlerts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub health_alerts: Vec<HealthAlert>,
    #[doc = "Settings for notification"]
    #[serde(rename = "notificationSettings", default, skip_serializing_if = "Option::is_none")]
    pub notification_settings: Option<NotificationSettings>,
    #[doc = "Domain Security Settings"]
    #[serde(rename = "domainSecuritySettings", default, skip_serializing_if = "Option::is_none")]
    pub domain_security_settings: Option<DomainSecuritySettings>,
    #[doc = "Enabled or Disabled flag to turn on Group-based filtered sync"]
    #[serde(rename = "filteredSync", default, skip_serializing_if = "Option::is_none")]
    pub filtered_sync: Option<domain_service_properties::FilteredSync>,
    #[doc = "List of Domain Controller IP Address"]
    #[serde(
        rename = "domainControllerIpAddress",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub domain_controller_ip_address: Vec<String>,
    #[doc = "Status of Domain Service instance"]
    #[serde(rename = "serviceStatus", default, skip_serializing_if = "Option::is_none")]
    pub service_status: Option<String>,
    #[doc = "the current deployment or provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl DomainServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod domain_service_properties {
    use super::*;
    #[doc = "Enabled or Disabled flag to turn on Group-based filtered sync"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FilteredSync")]
    pub enum FilteredSync {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FilteredSync {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FilteredSync {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FilteredSync {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("FilteredSync", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("FilteredSync", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Health Alert Description"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthAlert {
    #[doc = "Health Alert Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Health Alert Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Health Alert Issue"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<String>,
    #[doc = "Health Alert Severity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "Health Alert Raised DateTime"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub raised: Option<time::OffsetDateTime>,
    #[doc = "Health Alert Last Detected DateTime"]
    #[serde(rename = "lastDetected", default, with = "azure_core::date::rfc3339::option")]
    pub last_detected: Option<time::OffsetDateTime>,
    #[doc = "Health Alert TSG Link"]
    #[serde(rename = "resolutionUri", default, skip_serializing_if = "Option::is_none")]
    pub resolution_uri: Option<String>,
}
impl HealthAlert {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Health Monitor Description"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthMonitor {
    #[doc = "Health Monitor Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Health Monitor Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Health Monitor Details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl HealthMonitor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Secure LDAP Settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LdapsSettings {
    #[doc = "A flag to determine whether or not Secure LDAP is enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ldaps: Option<ldaps_settings::Ldaps>,
    #[doc = "The certificate required to configure Secure LDAP. The parameter passed here should be a base64encoded representation of the certificate pfx file."]
    #[serde(rename = "pfxCertificate", default, skip_serializing_if = "Option::is_none")]
    pub pfx_certificate: Option<String>,
    #[doc = "The password to decrypt the provided Secure LDAP certificate pfx file."]
    #[serde(rename = "pfxCertificatePassword", default, skip_serializing_if = "Option::is_none")]
    pub pfx_certificate_password: Option<String>,
    #[doc = "Public certificate used to configure secure ldap."]
    #[serde(rename = "publicCertificate", default, skip_serializing_if = "Option::is_none")]
    pub public_certificate: Option<String>,
    #[doc = "Thumbprint of configure ldaps certificate."]
    #[serde(rename = "certificateThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub certificate_thumbprint: Option<String>,
    #[doc = "NotAfter DateTime of configure ldaps certificate."]
    #[serde(rename = "certificateNotAfter", default, with = "azure_core::date::rfc3339::option")]
    pub certificate_not_after: Option<time::OffsetDateTime>,
    #[doc = "A flag to determine whether or not Secure LDAP access over the internet is enabled or disabled."]
    #[serde(rename = "externalAccess", default, skip_serializing_if = "Option::is_none")]
    pub external_access: Option<ldaps_settings::ExternalAccess>,
    #[doc = "External access ip address."]
    #[serde(rename = "externalAccessIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub external_access_ip_address: Option<String>,
}
impl LdapsSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ldaps_settings {
    use super::*;
    #[doc = "A flag to determine whether or not Secure LDAP is enabled or disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Ldaps")]
    pub enum Ldaps {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Ldaps {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Ldaps {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Ldaps {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("Ldaps", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Ldaps", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A flag to determine whether or not Secure LDAP access over the internet is enabled or disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ExternalAccess")]
    pub enum ExternalAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ExternalAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ExternalAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ExternalAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("ExternalAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("ExternalAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Settings for notification"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationSettings {
    #[doc = "Should global admins be notified"]
    #[serde(rename = "notifyGlobalAdmins", default, skip_serializing_if = "Option::is_none")]
    pub notify_global_admins: Option<notification_settings::NotifyGlobalAdmins>,
    #[doc = "Should domain controller admins be notified"]
    #[serde(rename = "notifyDcAdmins", default, skip_serializing_if = "Option::is_none")]
    pub notify_dc_admins: Option<notification_settings::NotifyDcAdmins>,
    #[doc = "The list of additional recipients"]
    #[serde(
        rename = "additionalRecipients",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_recipients: Vec<String>,
}
impl NotificationSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod notification_settings {
    use super::*;
    #[doc = "Should global admins be notified"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NotifyGlobalAdmins")]
    pub enum NotifyGlobalAdmins {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NotifyGlobalAdmins {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NotifyGlobalAdmins {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NotifyGlobalAdmins {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("NotifyGlobalAdmins", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("NotifyGlobalAdmins", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Should domain controller admins be notified"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NotifyDcAdmins")]
    pub enum NotifyDcAdmins {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NotifyDcAdmins {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NotifyDcAdmins {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NotifyDcAdmins {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("NotifyDcAdmins", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("NotifyDcAdmins", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The operation supported by Domain Services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayInfo {
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The action that users can perform, based on their permission level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Service provider: Domain Services."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl OperationDisplayInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operation supported by Domain Services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntity {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The operation supported by Domain Services."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
    #[doc = "The origin of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl OperationEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of domain service operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntityListResult {
    #[doc = "The list of operations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OperationEntity>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationEntityListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationEntityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Resource etag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
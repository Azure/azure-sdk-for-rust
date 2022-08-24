#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An error response from the Domain Services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the Domain Services."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl azure_core::Continuable for CloudError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the Domain Services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration Diagnostics"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigDiagnostics {
    #[doc = "Last domain configuration diagnostics DateTime"]
    #[serde(rename = "lastExecuted", default, with = "azure_core::date::rfc1123::option")]
    pub last_executed: Option<time::OffsetDateTime>,
    #[doc = "List of Configuration Diagnostics validator results."]
    #[serde(rename = "validatorResults", default, skip_serializing_if = "Vec::is_empty")]
    pub validator_results: Vec<ConfigDiagnosticsValidatorResult>,
}
impl ConfigDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Config Diagnostics validator result data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigDiagnosticsValidatorResult {
    #[doc = "Validator identifier"]
    #[serde(rename = "validatorId", default, skip_serializing_if = "Option::is_none")]
    pub validator_id: Option<String>,
    #[doc = "Replica set location and subnet name"]
    #[serde(rename = "replicaSetSubnetDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub replica_set_subnet_display_name: Option<String>,
    #[doc = "Status for individual validator after running diagnostics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<config_diagnostics_validator_result::Status>,
    #[doc = "List of resource config validation issues."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issues: Vec<ConfigDiagnosticsValidatorResultIssue>,
}
impl ConfigDiagnosticsValidatorResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod config_diagnostics_validator_result {
    use super::*;
    #[doc = "Status for individual validator after running diagnostics."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        None,
        Running,
        #[serde(rename = "OK")]
        Ok,
        Failure,
        Warning,
        Skipped,
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
                Self::None => serializer.serialize_unit_variant("Status", 0u32, "None"),
                Self::Running => serializer.serialize_unit_variant("Status", 1u32, "Running"),
                Self::Ok => serializer.serialize_unit_variant("Status", 2u32, "OK"),
                Self::Failure => serializer.serialize_unit_variant("Status", 3u32, "Failure"),
                Self::Warning => serializer.serialize_unit_variant("Status", 4u32, "Warning"),
                Self::Skipped => serializer.serialize_unit_variant("Status", 5u32, "Skipped"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::None
        }
    }
}
#[doc = "Specific issue for a particular config diagnostics validator"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigDiagnosticsValidatorResultIssue {
    #[doc = "Validation issue identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "List of domain resource property name or values used to compose a rich description."]
    #[serde(rename = "descriptionParams", default, skip_serializing_if = "Vec::is_empty")]
    pub description_params: Vec<String>,
}
impl ConfigDiagnosticsValidatorResultIssue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container Account Description"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerAccount {
    #[doc = "The account name"]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The account spn"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spn: Option<String>,
    #[doc = "The account password"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl ContainerAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
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
    #[doc = "A flag to determine whether or not SyncKerberosPasswords is enabled or disabled."]
    #[serde(rename = "syncKerberosPasswords", default, skip_serializing_if = "Option::is_none")]
    pub sync_kerberos_passwords: Option<domain_security_settings::SyncKerberosPasswords>,
    #[doc = "A flag to determine whether or not SyncOnPremPasswords is enabled or disabled."]
    #[serde(rename = "syncOnPremPasswords", default, skip_serializing_if = "Option::is_none")]
    pub sync_on_prem_passwords: Option<domain_security_settings::SyncOnPremPasswords>,
    #[doc = "A flag to determine whether or not KerberosRc4Encryption is enabled or disabled."]
    #[serde(rename = "kerberosRc4Encryption", default, skip_serializing_if = "Option::is_none")]
    pub kerberos_rc4_encryption: Option<domain_security_settings::KerberosRc4Encryption>,
    #[doc = "A flag to determine whether or not KerberosArmoring is enabled or disabled."]
    #[serde(rename = "kerberosArmoring", default, skip_serializing_if = "Option::is_none")]
    pub kerberos_armoring: Option<domain_security_settings::KerberosArmoring>,
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
    impl Default for NtlmV1 {
        fn default() -> Self {
            Self::Enabled
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
    impl Default for TlsV1 {
        fn default() -> Self {
            Self::Enabled
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
    impl Default for SyncNtlmPasswords {
        fn default() -> Self {
            Self::Enabled
        }
    }
    #[doc = "A flag to determine whether or not SyncKerberosPasswords is enabled or disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SyncKerberosPasswords")]
    pub enum SyncKerberosPasswords {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SyncKerberosPasswords {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SyncKerberosPasswords {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SyncKerberosPasswords {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("SyncKerberosPasswords", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("SyncKerberosPasswords", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for SyncKerberosPasswords {
        fn default() -> Self {
            Self::Enabled
        }
    }
    #[doc = "A flag to determine whether or not SyncOnPremPasswords is enabled or disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SyncOnPremPasswords")]
    pub enum SyncOnPremPasswords {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SyncOnPremPasswords {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SyncOnPremPasswords {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SyncOnPremPasswords {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("SyncOnPremPasswords", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("SyncOnPremPasswords", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for SyncOnPremPasswords {
        fn default() -> Self {
            Self::Enabled
        }
    }
    #[doc = "A flag to determine whether or not KerberosRc4Encryption is enabled or disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KerberosRc4Encryption")]
    pub enum KerberosRc4Encryption {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KerberosRc4Encryption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KerberosRc4Encryption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KerberosRc4Encryption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("KerberosRc4Encryption", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("KerberosRc4Encryption", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for KerberosRc4Encryption {
        fn default() -> Self {
            Self::Enabled
        }
    }
    #[doc = "A flag to determine whether or not KerberosArmoring is enabled or disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KerberosArmoring")]
    pub enum KerberosArmoring {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KerberosArmoring {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KerberosArmoring {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KerberosArmoring {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("KerberosArmoring", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("KerberosArmoring", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for KerberosArmoring {
        fn default() -> Self {
            Self::Disabled
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[doc = "Data Model Version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[doc = "Azure Active Directory Tenant Id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The name of the Azure domain that the user would like to deploy Domain Services to."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "Deployment Id"]
    #[serde(rename = "deploymentId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[doc = "SyncOwner ReplicaSet Id"]
    #[serde(rename = "syncOwner", default, skip_serializing_if = "Option::is_none")]
    pub sync_owner: Option<String>,
    #[doc = "List of ReplicaSets"]
    #[serde(rename = "replicaSets", default, skip_serializing_if = "Vec::is_empty")]
    pub replica_sets: Vec<ReplicaSet>,
    #[doc = "Secure LDAP Settings"]
    #[serde(rename = "ldapsSettings", default, skip_serializing_if = "Option::is_none")]
    pub ldaps_settings: Option<LdapsSettings>,
    #[doc = "Settings for Resource Forest"]
    #[serde(rename = "resourceForestSettings", default, skip_serializing_if = "Option::is_none")]
    pub resource_forest_settings: Option<ResourceForestSettings>,
    #[doc = "Domain Security Settings"]
    #[serde(rename = "domainSecuritySettings", default, skip_serializing_if = "Option::is_none")]
    pub domain_security_settings: Option<DomainSecuritySettings>,
    #[doc = "Domain Configuration Type"]
    #[serde(rename = "domainConfigurationType", default, skip_serializing_if = "Option::is_none")]
    pub domain_configuration_type: Option<String>,
    #[doc = "Sku Type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "Enabled or Disabled flag to turn on Group-based filtered sync"]
    #[serde(rename = "filteredSync", default, skip_serializing_if = "Option::is_none")]
    pub filtered_sync: Option<domain_service_properties::FilteredSync>,
    #[doc = "Settings for notification"]
    #[serde(rename = "notificationSettings", default, skip_serializing_if = "Option::is_none")]
    pub notification_settings: Option<NotificationSettings>,
    #[doc = "Migration Properties"]
    #[serde(rename = "migrationProperties", default, skip_serializing_if = "Option::is_none")]
    pub migration_properties: Option<MigrationProperties>,
    #[doc = "the current deployment or provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Configuration Diagnostics"]
    #[serde(rename = "configDiagnostics", default, skip_serializing_if = "Option::is_none")]
    pub config_diagnostics: Option<ConfigDiagnostics>,
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
#[doc = "Forest Trust Setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForestTrust {
    #[doc = "Trusted Domain FQDN"]
    #[serde(rename = "trustedDomainFqdn", default, skip_serializing_if = "Option::is_none")]
    pub trusted_domain_fqdn: Option<String>,
    #[doc = "Trust Direction"]
    #[serde(rename = "trustDirection", default, skip_serializing_if = "Option::is_none")]
    pub trust_direction: Option<String>,
    #[doc = "Friendly Name"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Remote Dns ips"]
    #[serde(rename = "remoteDnsIps", default, skip_serializing_if = "Option::is_none")]
    pub remote_dns_ips: Option<String>,
    #[doc = "Trust Password"]
    #[serde(rename = "trustPassword", default, skip_serializing_if = "Option::is_none")]
    pub trust_password: Option<String>,
}
impl ForestTrust {
    pub fn new() -> Self {
        Self::default()
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
    impl Default for Ldaps {
        fn default() -> Self {
            Self::Disabled
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
    impl Default for ExternalAccess {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[doc = "Migration Progress"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationProgress {
    #[doc = "Completion Percentage"]
    #[serde(rename = "completionPercentage", default, skip_serializing_if = "Option::is_none")]
    pub completion_percentage: Option<f64>,
    #[doc = "Progress Message"]
    #[serde(rename = "progressMessage", default, skip_serializing_if = "Option::is_none")]
    pub progress_message: Option<String>,
}
impl MigrationProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Migration Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationProperties {
    #[doc = "Old Subnet Id"]
    #[serde(rename = "oldSubnetId", default, skip_serializing_if = "Option::is_none")]
    pub old_subnet_id: Option<String>,
    #[doc = "Old Vnet Site Id"]
    #[serde(rename = "oldVnetSiteId", default, skip_serializing_if = "Option::is_none")]
    pub old_vnet_site_id: Option<String>,
    #[doc = "Migration Progress"]
    #[serde(rename = "migrationProgress", default, skip_serializing_if = "Option::is_none")]
    pub migration_progress: Option<MigrationProgress>,
}
impl MigrationProperties {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(rename = "additionalRecipients", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Resource for OuContainer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OuContainer {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the OuContainer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OuContainerProperties>,
}
impl OuContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from the List OuContainer operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OuContainerListResult {
    #[doc = "The list of OuContainer."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OuContainer>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OuContainerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OuContainerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the OuContainer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OuContainerProperties {
    #[doc = "Azure Active Directory tenant id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The domain name of Domain Services."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "The Deployment id"]
    #[serde(rename = "deploymentId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[doc = "The OuContainer name"]
    #[serde(rename = "containerId", default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[doc = "The list of container accounts"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<ContainerAccount>,
    #[doc = "Status of OuContainer instance"]
    #[serde(rename = "serviceStatus", default, skip_serializing_if = "Option::is_none")]
    pub service_status: Option<String>,
    #[doc = "Distinguished Name of OuContainer instance"]
    #[serde(rename = "distinguishedName", default, skip_serializing_if = "Option::is_none")]
    pub distinguished_name: Option<String>,
    #[doc = "The current deployment or provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl OuContainerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replica Set Definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicaSet {
    #[doc = "ReplicaSet Id"]
    #[serde(rename = "replicaSetId", default, skip_serializing_if = "Option::is_none")]
    pub replica_set_id: Option<String>,
    #[doc = "Virtual network location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Virtual network site id"]
    #[serde(rename = "vnetSiteId", default, skip_serializing_if = "Option::is_none")]
    pub vnet_site_id: Option<String>,
    #[doc = "The name of the virtual network that Domain Services will be deployed on. The id of the subnet that Domain Services will be deployed on. /virtualNetwork/vnetName/subnets/subnetName."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "List of Domain Controller IP Address"]
    #[serde(rename = "domainControllerIpAddress", default, skip_serializing_if = "Vec::is_empty")]
    pub domain_controller_ip_address: Vec<String>,
    #[doc = "External access ip address."]
    #[serde(rename = "externalAccessIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub external_access_ip_address: Option<String>,
    #[doc = "Status of Domain Service instance"]
    #[serde(rename = "serviceStatus", default, skip_serializing_if = "Option::is_none")]
    pub service_status: Option<String>,
    #[doc = "Last domain evaluation run DateTime"]
    #[serde(rename = "healthLastEvaluated", default, with = "azure_core::date::rfc1123::option")]
    pub health_last_evaluated: Option<time::OffsetDateTime>,
    #[doc = "List of Domain Health Monitors"]
    #[serde(rename = "healthMonitors", default, skip_serializing_if = "Vec::is_empty")]
    pub health_monitors: Vec<HealthMonitor>,
    #[doc = "List of Domain Health Alerts"]
    #[serde(rename = "healthAlerts", default, skip_serializing_if = "Vec::is_empty")]
    pub health_alerts: Vec<HealthAlert>,
}
impl ReplicaSet {
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Settings for Resource Forest"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceForestSettings {
    #[doc = "List of settings for Resource Forest"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub settings: Vec<ForestTrust>,
    #[doc = "Resource Forest"]
    #[serde(rename = "resourceForest", default, skip_serializing_if = "Option::is_none")]
    pub resource_forest: Option<String>,
}
impl ResourceForestSettings {
    pub fn new() -> Self {
        Self::default()
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

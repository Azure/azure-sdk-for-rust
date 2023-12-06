#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "AAD based security principal with associated Ledger RoleName"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AadBasedSecurityPrincipal {
    #[doc = "UUID/GUID based Principal Id of the Security Principal"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "UUID/GUID based Tenant Id of the Security Principal"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "LedgerRole associated with the Security Principal of Ledger"]
    #[serde(rename = "ledgerRoleName", default, skip_serializing_if = "Option::is_none")]
    pub ledger_role_name: Option<LedgerRoleName>,
}
impl AadBasedSecurityPrincipal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cert based security principal with Ledger RoleName"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertBasedSecurityPrincipal {
    #[doc = "Public key of the user cert (.pem or .cer)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    #[doc = "LedgerRole associated with the Security Principal of Ledger"]
    #[serde(rename = "ledgerRoleName", default, skip_serializing_if = "Option::is_none")]
    pub ledger_role_name: Option<LedgerRoleName>,
}
impl CertBasedSecurityPrincipal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tags for Managed CCF Certificates"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateTags {
    #[doc = "Additional tags for Managed CCF Certificates"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CertificateTags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The check availability request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityRequest {
    #[doc = "The name of the resource for which availability needs to be checked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckNameAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The check availability result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResponse {
    #[doc = "Indicates if the resource name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason why the given name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_response::Reason>,
    #[doc = "Detailed reason why the given name is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_response {
    use super::*;
    #[doc = "The reason why the given name is not available."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        Invalid,
        AlreadyExists,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Reason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Reason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Reason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("Reason", 0u32, "Invalid"),
                Self::AlreadyExists => serializer.serialize_unit_variant("Reason", 1u32, "AlreadyExists"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Confidential Ledger. Contains the properties of Confidential Ledger Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfidentialLedger {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Additional Confidential Ledger properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LedgerProperties>,
}
impl ConfidentialLedger {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Object representing Backup properties of a Confidential Ledger Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfidentialLedgerBackup {
    #[doc = "The region where the backup of the ledger will eventually be restored to."]
    #[serde(rename = "restoreRegion", default, skip_serializing_if = "Option::is_none")]
    pub restore_region: Option<String>,
    #[doc = "SAS URI used to access the backup Fileshare."]
    pub uri: String,
}
impl ConfidentialLedgerBackup {
    pub fn new(uri: String) -> Self {
        Self { restore_region: None, uri }
    }
}
#[doc = "Object representing the backup response of a Confidential Ledger Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfidentialLedgerBackupResponse {
    #[doc = "Response body stating if the ledger is being backed up."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ConfidentialLedgerBackupResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of Confidential Ledgers and a possible link for next set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfidentialLedgerList {
    #[doc = "List of Confidential Ledgers"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ConfidentialLedger>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConfidentialLedgerList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ConfidentialLedgerList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object representing Restore properties of a Confidential Ledger Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfidentialLedgerRestore {
    #[doc = "Fileshare where the ledger backup is stored."]
    #[serde(rename = "fileShareName")]
    pub file_share_name: String,
    #[doc = "The region the ledger is being restored to."]
    #[serde(rename = "restoreRegion")]
    pub restore_region: String,
    #[doc = "SAS URI used to access the backup fileshare."]
    pub uri: String,
}
impl ConfidentialLedgerRestore {
    pub fn new(file_share_name: String, restore_region: String, uri: String) -> Self {
        Self {
            file_share_name,
            restore_region,
            uri,
        }
    }
}
#[doc = "Object representing the restore response of a Confidential Ledger Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfidentialLedgerRestoreResponse {
    #[doc = "Response body stating if the ledger is being restored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ConfidentialLedgerRestoreResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the ledger. Private means transaction data is encrypted."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ConfidentialLedgerType")]
pub enum ConfidentialLedgerType {
    Unknown,
    Public,
    Private,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ConfidentialLedgerType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ConfidentialLedgerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ConfidentialLedgerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ConfidentialLedgerType", 0u32, "Unknown"),
            Self::Public => serializer.serialize_unit_variant("ConfidentialLedgerType", 1u32, "Public"),
            Self::Private => serializer.serialize_unit_variant("ConfidentialLedgerType", 2u32, "Private"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Object representing DeploymentType for Managed CCF."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentType {
    #[doc = "Object representing LanguageRuntime for Manged CCF."]
    #[serde(rename = "languageRuntime", default, skip_serializing_if = "Option::is_none")]
    pub language_runtime: Option<LanguageRuntime>,
    #[doc = "Source Uri containing ManagedCCF code"]
    #[serde(rename = "appSourceUri", default, skip_serializing_if = "Option::is_none")]
    pub app_source_uri: Option<String>,
}
impl DeploymentType {
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
#[doc = "Object representing LanguageRuntime for Manged CCF."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LanguageRuntime")]
pub enum LanguageRuntime {
    #[serde(rename = "CPP")]
    Cpp,
    #[serde(rename = "JS")]
    Js,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LanguageRuntime {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LanguageRuntime {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LanguageRuntime {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Cpp => serializer.serialize_unit_variant("LanguageRuntime", 0u32, "CPP"),
            Self::Js => serializer.serialize_unit_variant("LanguageRuntime", 1u32, "JS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Additional Confidential Ledger properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LedgerProperties {
    #[doc = "Unique name for the Confidential Ledger."]
    #[serde(rename = "ledgerName", default, skip_serializing_if = "Option::is_none")]
    pub ledger_name: Option<String>,
    #[doc = "Endpoint for calling Ledger Service."]
    #[serde(rename = "ledgerUri", default, skip_serializing_if = "Option::is_none")]
    pub ledger_uri: Option<String>,
    #[doc = "Endpoint for accessing network identity."]
    #[serde(rename = "identityServiceUri", default, skip_serializing_if = "Option::is_none")]
    pub identity_service_uri: Option<String>,
    #[doc = "Internal namespace for the Ledger"]
    #[serde(rename = "ledgerInternalNamespace", default, skip_serializing_if = "Option::is_none")]
    pub ledger_internal_namespace: Option<String>,
    #[doc = "Object representing RunningState for Confidential Ledger."]
    #[serde(rename = "runningState", default, skip_serializing_if = "Option::is_none")]
    pub running_state: Option<RunningState>,
    #[doc = "Type of the ledger. Private means transaction data is encrypted."]
    #[serde(rename = "ledgerType", default, skip_serializing_if = "Option::is_none")]
    pub ledger_type: Option<ConfidentialLedgerType>,
    #[doc = "Object representing ProvisioningState for Confidential Ledger."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "SKU associated with the ledger resource"]
    #[serde(rename = "ledgerSku", default, skip_serializing_if = "Option::is_none")]
    pub ledger_sku: Option<LedgerSku>,
    #[doc = "Array of all AAD based Security Principals."]
    #[serde(
        rename = "aadBasedSecurityPrincipals",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub aad_based_security_principals: Vec<AadBasedSecurityPrincipal>,
    #[doc = "Array of all cert based Security Principals."]
    #[serde(
        rename = "certBasedSecurityPrincipals",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cert_based_security_principals: Vec<CertBasedSecurityPrincipal>,
}
impl LedgerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "LedgerRole associated with the Security Principal of Ledger"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LedgerRoleName")]
pub enum LedgerRoleName {
    Reader,
    Contributor,
    Administrator,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LedgerRoleName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LedgerRoleName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LedgerRoleName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Reader => serializer.serialize_unit_variant("LedgerRoleName", 0u32, "Reader"),
            Self::Contributor => serializer.serialize_unit_variant("LedgerRoleName", 1u32, "Contributor"),
            Self::Administrator => serializer.serialize_unit_variant("LedgerRoleName", 2u32, "Administrator"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SKU associated with the ledger resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LedgerSku")]
pub enum LedgerSku {
    Standard,
    Basic,
    Unknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LedgerSku {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LedgerSku {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LedgerSku {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Standard => serializer.serialize_unit_variant("LedgerSku", 0u32, "Standard"),
            Self::Basic => serializer.serialize_unit_variant("LedgerSku", 1u32, "Basic"),
            Self::Unknown => serializer.serialize_unit_variant("LedgerSku", 2u32, "Unknown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Managed CCF. Contains the properties of Managed CCF Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedCcf {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Additional Managed CCF properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedCcfProperties>,
}
impl ManagedCcf {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Object representing Backup properties of a Managed CCF Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedCcfBackup {
    #[doc = "The region where the backup of the managed CCF resource will eventually be restored to."]
    #[serde(rename = "restoreRegion", default, skip_serializing_if = "Option::is_none")]
    pub restore_region: Option<String>,
    #[doc = "SAS URI used to access the backup Fileshare."]
    pub uri: String,
}
impl ManagedCcfBackup {
    pub fn new(uri: String) -> Self {
        Self { restore_region: None, uri }
    }
}
#[doc = "Object representing the backup response of a Managed CCF Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedCcfBackupResponse {
    #[doc = "Response body stating if the managed CCF resource is being backed up."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ManagedCcfBackupResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of Managed CCF and a possible link for next set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedCcfList {
    #[doc = "List of Managed CCF"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedCcf>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedCcfList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedCcfList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional Managed CCF properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedCcfProperties {
    #[doc = "Unique name for the Managed CCF."]
    #[serde(rename = "appName", default, skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[doc = "Endpoint for calling Managed CCF Service."]
    #[serde(rename = "appUri", default, skip_serializing_if = "Option::is_none")]
    pub app_uri: Option<String>,
    #[doc = "Endpoint for accessing network identity."]
    #[serde(rename = "identityServiceUri", default, skip_serializing_if = "Option::is_none")]
    pub identity_service_uri: Option<String>,
    #[doc = "List of member identity certificates for  Managed CCF"]
    #[serde(
        rename = "memberIdentityCertificates",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub member_identity_certificates: Vec<MemberIdentityCertificate>,
    #[doc = "Object representing DeploymentType for Managed CCF."]
    #[serde(rename = "deploymentType", default, skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<DeploymentType>,
    #[doc = "Object representing RunningState for Managed CCF."]
    #[serde(rename = "runningState", default, skip_serializing_if = "Option::is_none")]
    pub running_state: Option<RunningState>,
    #[doc = "Object representing ProvisioningState for Managed CCF."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Number of CCF nodes in the Managed CCF."]
    #[serde(rename = "nodeCount", default, skip_serializing_if = "Option::is_none")]
    pub node_count: Option<NodeCount>,
}
impl ManagedCcfProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object representing Restore properties of Managed CCF Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedCcfRestore {
    #[doc = "Fileshare where the managed CCF resource backup is stored."]
    #[serde(rename = "fileShareName")]
    pub file_share_name: String,
    #[doc = "The region the managed CCF resource is being restored to."]
    #[serde(rename = "restoreRegion")]
    pub restore_region: String,
    #[doc = "SAS URI used to access the backup Fileshare."]
    pub uri: String,
}
impl ManagedCcfRestore {
    pub fn new(file_share_name: String, restore_region: String, uri: String) -> Self {
        Self {
            file_share_name,
            restore_region,
            uri,
        }
    }
}
#[doc = "Object representing the restore response of a Managed CCF Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedCcfRestoreResponse {
    #[doc = "Response body stating if the managed CCF resource is being restored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ManagedCcfRestoreResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object representing MemberIdentityCertificate for Managed CCF."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MemberIdentityCertificate {
    #[doc = "Member Identity Certificate"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[doc = "Member Identity Certificate Encryption Key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryptionkey: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl MemberIdentityCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type NodeCount = i32;
#[doc = "Object representing ProvisioningState for Confidential Ledger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Unknown,
    Succeeded,
    Failed,
    Canceled,
    Creating,
    Deleting,
    Updating,
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
            Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Unknown"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
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
#[doc = "Describes the Resource Provider Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationDefinition {
    #[doc = "Resource provider operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is data action or not."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Describes the properties of the Operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<ResourceProviderOperationDisplay>,
}
impl ResourceProviderOperationDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of the Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationDisplay {
    #[doc = "Name of the resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Name of the resource type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Name of the resource provider operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the resource provider operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ResourceProviderOperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List containing this Resource Provider's available operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationList {
    #[doc = "Resource provider operations list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ResourceProviderOperationDefinition>,
    #[doc = "The URI that can be used to request the next page for list of Azure operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceProviderOperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ResourceProviderOperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object representing RunningState for Confidential Ledger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RunningState")]
pub enum RunningState {
    Active,
    Paused,
    Unknown,
    Pausing,
    Resuming,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RunningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RunningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RunningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("RunningState", 0u32, "Active"),
            Self::Paused => serializer.serialize_unit_variant("RunningState", 1u32, "Paused"),
            Self::Unknown => serializer.serialize_unit_variant("RunningState", 2u32, "Unknown"),
            Self::Pausing => serializer.serialize_unit_variant("RunningState", 3u32, "Pausing"),
            Self::Resuming => serializer.serialize_unit_variant("RunningState", 4u32, "Resuming"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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

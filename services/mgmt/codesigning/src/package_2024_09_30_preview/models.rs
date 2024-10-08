#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "SKU of the trusted signing account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountSku {
    #[doc = "Name of the sku."]
    pub name: SkuName,
}
impl AccountSku {
    pub fn new(name: SkuName) -> Self {
        Self { name }
    }
}
#[doc = "SKU of the trusted signing account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountSkuPatch {
    #[doc = "Name of the sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<SkuName>,
}
impl AccountSkuPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Certificate {
    #[doc = "Serial number of the certificate."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Enhanced key usage of the certificate."]
    #[serde(rename = "enhancedKeyUsage", default, skip_serializing_if = "Option::is_none")]
    pub enhanced_key_usage: Option<String>,
    #[doc = "Subject name of the certificate."]
    #[serde(rename = "subjectName", default, skip_serializing_if = "Option::is_none")]
    pub subject_name: Option<String>,
    #[doc = "Thumbprint of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "Certificate created date."]
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[doc = "Certificate expiry date."]
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[doc = "Status of the certificate"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CertificateStatus>,
    #[doc = "Revocation details of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revocation: Option<Revocation>,
}
impl Certificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Certificate profile resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateProfile {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the certificate profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificateProfileProperties>,
}
impl CertificateProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a CertificateProfile list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateProfileListResult {
    #[doc = "The CertificateProfile items on this page"]
    pub value: Vec<CertificateProfile>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CertificateProfileListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CertificateProfileListResult {
    pub fn new(value: Vec<CertificateProfile>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties of the certificate profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateProfileProperties {
    #[doc = "Type of the certificate"]
    #[serde(rename = "profileType")]
    pub profile_type: ProfileType,
    #[doc = "Whether to include STREET in the certificate subject name."]
    #[serde(rename = "includeStreetAddress", default, skip_serializing_if = "Option::is_none")]
    pub include_street_address: Option<bool>,
    #[doc = "Whether to include L in the certificate subject name. Applicable only for private trust, private trust ci profile types"]
    #[serde(rename = "includeCity", default, skip_serializing_if = "Option::is_none")]
    pub include_city: Option<bool>,
    #[doc = "Whether to include S in the certificate subject name. Applicable only for private trust, private trust ci profile types"]
    #[serde(rename = "includeState", default, skip_serializing_if = "Option::is_none")]
    pub include_state: Option<bool>,
    #[doc = "Whether to include C in the certificate subject name. Applicable only for private trust, private trust ci profile types"]
    #[serde(rename = "includeCountry", default, skip_serializing_if = "Option::is_none")]
    pub include_country: Option<bool>,
    #[doc = "Whether to include PC in the certificate subject name."]
    #[serde(rename = "includePostalCode", default, skip_serializing_if = "Option::is_none")]
    pub include_postal_code: Option<bool>,
    #[doc = "Identity validation id used for the certificate subject name."]
    #[serde(rename = "identityValidationId")]
    pub identity_validation_id: String,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Status of the certificate profiles."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CertificateProfileStatus>,
    #[doc = "List of renewed certificates."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub certificates: Vec<Certificate>,
}
impl CertificateProfileProperties {
    pub fn new(profile_type: ProfileType, identity_validation_id: String) -> Self {
        Self {
            profile_type,
            include_street_address: None,
            include_city: None,
            include_state: None,
            include_country: None,
            include_postal_code: None,
            identity_validation_id,
            provisioning_state: None,
            status: None,
            certificates: Vec::new(),
        }
    }
}
#[doc = "Status of the certificate profiles."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CertificateProfileStatus")]
pub enum CertificateProfileStatus {
    Active,
    Disabled,
    Suspended,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CertificateProfileStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CertificateProfileStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CertificateProfileStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("CertificateProfileStatus", 0u32, "Active"),
            Self::Disabled => serializer.serialize_unit_variant("CertificateProfileStatus", 1u32, "Disabled"),
            Self::Suspended => serializer.serialize_unit_variant("CertificateProfileStatus", 2u32, "Suspended"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Status of the certificate"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CertificateStatus")]
pub enum CertificateStatus {
    Active,
    Expired,
    Revoked,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CertificateStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CertificateStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CertificateStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("CertificateStatus", 0u32, "Active"),
            Self::Expired => serializer.serialize_unit_variant("CertificateStatus", 1u32, "Expired"),
            Self::Revoked => serializer.serialize_unit_variant("CertificateStatus", 2u32, "Revoked"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The parameters used to check the availability of the trusted signing account name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailability {
    #[doc = "Trusted signing account name."]
    pub name: String,
}
impl CheckNameAvailability {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "The CheckNameAvailability operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "A boolean value that indicates whether the name is available for you to use. If true, the name is available. If false, the name has already been taken or is invalid and cannot be used."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason that a trusted signing account name could not be used. The Reason element is only returned if nameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<NameUnavailabilityReason>,
    #[doc = "An error message explaining the Reason value in more detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Trusted signing account resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeSigningAccount {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of the trusted signing account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CodeSigningAccountProperties>,
}
impl CodeSigningAccount {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a CodeSigningAccount list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeSigningAccountListResult {
    #[doc = "The CodeSigningAccount items on this page"]
    pub value: Vec<CodeSigningAccount>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CodeSigningAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CodeSigningAccountListResult {
    pub fn new(value: Vec<CodeSigningAccount>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Parameters for creating or updating a trusted signing account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodeSigningAccountPatch {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of the trusted signing account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CodeSigningAccountPatchProperties>,
}
impl CodeSigningAccountPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the trusted signing account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodeSigningAccountPatchProperties {
    #[doc = "SKU of the trusted signing account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<AccountSkuPatch>,
}
impl CodeSigningAccountPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the trusted signing account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodeSigningAccountProperties {
    #[doc = "The URI of the trusted signing account which is used during signing files."]
    #[serde(rename = "accountUri", default, skip_serializing_if = "Option::is_none")]
    pub account_uri: Option<String>,
    #[doc = "SKU of the trusted signing account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<AccountSku>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl CodeSigningAccountProperties {
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
#[doc = "The reason that a trusted signing account name could not be used. The Reason element is only returned if nameAvailable is false."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NameUnavailabilityReason")]
pub enum NameUnavailabilityReason {
    AccountNameInvalid,
    AlreadyExists,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NameUnavailabilityReason {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NameUnavailabilityReason {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NameUnavailabilityReason {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AccountNameInvalid => serializer.serialize_unit_variant("NameUnavailabilityReason", 0u32, "AccountNameInvalid"),
            Self::AlreadyExists => serializer.serialize_unit_variant("NameUnavailabilityReason", 1u32, "AlreadyExists"),
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
#[doc = "Type of the certificate"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProfileType")]
pub enum ProfileType {
    PublicTrust,
    PrivateTrust,
    #[serde(rename = "PrivateTrustCIPolicy")]
    PrivateTrustCiPolicy,
    #[serde(rename = "VBSEnclave")]
    VbsEnclave,
    PublicTrustTest,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProfileType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProfileType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProfileType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::PublicTrust => serializer.serialize_unit_variant("ProfileType", 0u32, "PublicTrust"),
            Self::PrivateTrust => serializer.serialize_unit_variant("ProfileType", 1u32, "PrivateTrust"),
            Self::PrivateTrustCiPolicy => serializer.serialize_unit_variant("ProfileType", 2u32, "PrivateTrustCIPolicy"),
            Self::VbsEnclave => serializer.serialize_unit_variant("ProfileType", 3u32, "VBSEnclave"),
            Self::PublicTrustTest => serializer.serialize_unit_variant("ProfileType", 4u32, "PublicTrustTest"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The status of the current operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Updating,
    Deleting,
    Accepted,
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
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Accepted"),
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
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. E.g. \"/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}\""]
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
#[doc = "Revocation details of the certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Revocation {
    #[doc = "The timestamp when the revocation is requested."]
    #[serde(rename = "requestedAt", default, with = "azure_core::date::rfc3339::option")]
    pub requested_at: Option<::time::OffsetDateTime>,
    #[doc = "The timestamp when the revocation is effective."]
    #[serde(rename = "effectiveAt", default, with = "azure_core::date::rfc3339::option")]
    pub effective_at: Option<::time::OffsetDateTime>,
    #[doc = "Reason for revocation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Remarks for the revocation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    #[doc = "Revocation status of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RevocationStatus>,
    #[doc = "Reason for the revocation failure."]
    #[serde(rename = "failureReason", default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}
impl Revocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Revocation status of the certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RevocationStatus")]
pub enum RevocationStatus {
    Succeeded,
    InProgress,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RevocationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RevocationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RevocationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("RevocationStatus", 0u32, "Succeeded"),
            Self::InProgress => serializer.serialize_unit_variant("RevocationStatus", 1u32, "InProgress"),
            Self::Failed => serializer.serialize_unit_variant("RevocationStatus", 2u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the certificate revocation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RevokeCertificate {
    #[doc = "Serial number of the certificate."]
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[doc = "Thumbprint of the certificate."]
    pub thumbprint: String,
    #[doc = "The timestamp when the revocation is effective."]
    #[serde(rename = "effectiveAt", with = "azure_core::date::rfc3339")]
    pub effective_at: ::time::OffsetDateTime,
    #[doc = "Reason for the revocation."]
    pub reason: String,
    #[doc = "Remarks for the revocation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}
impl RevokeCertificate {
    pub fn new(serial_number: String, thumbprint: String, effective_at: ::time::OffsetDateTime, reason: String) -> Self {
        Self {
            serial_number,
            thumbprint,
            effective_at,
            reason,
            remarks: None,
        }
    }
}
#[doc = "Name of the sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SkuName")]
pub enum SkuName {
    Basic,
    Premium,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SkuName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SkuName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SkuName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Basic => serializer.serialize_unit_variant("SkuName", 0u32, "Basic"),
            Self::Premium => serializer.serialize_unit_variant("SkuName", 1u32, "Premium"),
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
    pub created_at: Option<::time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<::time::OffsetDateTime>,
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

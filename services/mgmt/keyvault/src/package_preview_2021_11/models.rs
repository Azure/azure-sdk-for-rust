#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An identity that have access to the key vault. All identities in the array must use the same tenant ID as the key vault's tenant ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyEntry {
    #[doc = "The Azure Active Directory tenant ID that should be used for authenticating requests to the key vault."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[doc = "The object ID of a user, service principal or security group in the Azure Active Directory tenant for the vault. The object ID must be unique for the list of access policies."]
    #[serde(rename = "objectId")]
    pub object_id: String,
    #[doc = " Application ID of the client making request on behalf of a principal"]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "Permissions the identity has for keys, secrets, certificates and storage."]
    pub permissions: Permissions,
}
impl AccessPolicyEntry {
    pub fn new(tenant_id: String, object_id: String, permissions: Permissions) -> Self {
        Self {
            tenant_id,
            object_id,
            application_id: None,
            permissions,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Action {
    #[doc = "The type of action."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<action::Type>,
}
impl Action {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod action {
    use super::*;
    #[doc = "The type of action."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "rotate")]
        Rotate,
        #[serde(rename = "notify")]
        Notify,
    }
}
#[doc = "The object attributes managed by the KeyVault service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Attributes {
    #[doc = "Determines whether the object is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Not before date in seconds since 1970-01-01T00:00:00Z."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbf: Option<i64>,
    #[doc = "Expiry date in seconds since 1970-01-01T00:00:00Z."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    #[doc = "Creation time in seconds since 1970-01-01T00:00:00Z."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[doc = "Last updated time in seconds since 1970-01-01T00:00:00Z."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
}
impl Attributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The CheckNameAvailability operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "A boolean value that indicates whether the name is available for you to use. If true, the name is available. If false, the name has already been taken or is invalid and cannot be used."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason that a vault name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[doc = "An error message explaining the Reason value in more detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_result {
    use super::*;
    #[doc = "The reason that a vault name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        AccountNameInvalid,
        AlreadyExists,
    }
}
#[doc = "An error response from Key Vault resource provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from Key Vault resource provider"]
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
#[doc = "An error response from Key Vault resource provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "Error code. This is a mnemonic that can be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "User friendly error message. The message is typically localized and may vary with service version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedManagedHsm {
    #[doc = "The Azure Resource Manager resource ID for the deleted managed HSM Pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the managed HSM Pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type of the managed HSM Pool."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of the deleted managed HSM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeletedManagedHsmProperties>,
}
impl DeletedManagedHsm {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of deleted managed HSM Pools"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedManagedHsmListResult {
    #[doc = "The list of deleted managed HSM Pools."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeletedManagedHsm>,
    #[doc = "The URL to get the next set of deleted managed HSM Pools."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeletedManagedHsmListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeletedManagedHsmListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the deleted managed HSM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedManagedHsmProperties {
    #[doc = "The resource id of the original managed HSM."]
    #[serde(rename = "mhsmId", default, skip_serializing_if = "Option::is_none")]
    pub mhsm_id: Option<String>,
    #[doc = "The location of the original managed HSM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The deleted date."]
    #[serde(rename = "deletionDate", with = "azure_core::date::rfc3339::option")]
    pub deletion_date: Option<time::OffsetDateTime>,
    #[doc = "The scheduled purged date."]
    #[serde(rename = "scheduledPurgeDate", with = "azure_core::date::rfc3339::option")]
    pub scheduled_purge_date: Option<time::OffsetDateTime>,
    #[doc = "Purge protection status of the original managed HSM."]
    #[serde(rename = "purgeProtectionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub purge_protection_enabled: Option<bool>,
    #[doc = "Tags of the original managed HSM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DeletedManagedHsmProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deleted vault information with extended details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedVault {
    #[doc = "The resource ID for the deleted key vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the key vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type of the key vault."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of the deleted vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeletedVaultProperties>,
}
impl DeletedVault {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of vaults"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedVaultListResult {
    #[doc = "The list of deleted vaults."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeletedVault>,
    #[doc = "The URL to get the next set of deleted vaults."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeletedVaultListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeletedVaultListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the deleted vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedVaultProperties {
    #[doc = "The resource id of the original vault."]
    #[serde(rename = "vaultId", default, skip_serializing_if = "Option::is_none")]
    pub vault_id: Option<String>,
    #[doc = "The location of the original vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The deleted date."]
    #[serde(rename = "deletionDate", with = "azure_core::date::rfc3339::option")]
    pub deletion_date: Option<time::OffsetDateTime>,
    #[doc = "The scheduled purged date."]
    #[serde(rename = "scheduledPurgeDate", with = "azure_core::date::rfc3339::option")]
    pub scheduled_purge_date: Option<time::OffsetDateTime>,
    #[doc = "Tags of the original vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Purge protection status of the original vault."]
    #[serde(rename = "purgeProtectionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub purge_protection_enabled: Option<bool>,
}
impl DeletedVaultProperties {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Property to specify whether the dimension should be exported for Shoebox."]
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl DimensionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The server error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The server error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Box<Option<Error>>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A rule governing the accessibility of a vault from a specific ip address or ip range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpRule {
    #[doc = "An IPv4 address range in CIDR notation, such as '124.56.78.91' (simple IP address) or '124.56.78.0/24' (all addresses that start with 124.56.78)."]
    pub value: String,
}
impl IpRule {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
#[doc = "The type of identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IdentityType")]
pub enum IdentityType {
    User,
    Application,
    ManagedIdentity,
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
            Self::User => serializer.serialize_unit_variant("IdentityType", 0u32, "User"),
            Self::Application => serializer.serialize_unit_variant("IdentityType", 1u32, "Application"),
            Self::ManagedIdentity => serializer.serialize_unit_variant("IdentityType", 2u32, "ManagedIdentity"),
            Self::Key => serializer.serialize_unit_variant("IdentityType", 3u32, "Key"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The key resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Key {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the key."]
    pub properties: KeyProperties,
}
impl Key {
    pub fn new(properties: KeyProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "The object attributes managed by the Azure Key Vault service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyAttributes {
    #[doc = "Determines whether or not the object is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Not before date in seconds since 1970-01-01T00:00:00Z."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbf: Option<i64>,
    #[doc = "Expiry date in seconds since 1970-01-01T00:00:00Z."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    #[doc = "Creation time in seconds since 1970-01-01T00:00:00Z."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[doc = "Last updated time in seconds since 1970-01-01T00:00:00Z."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
    #[doc = "The deletion recovery level currently in effect for the object. If it contains 'Purgeable', then the object can be permanently deleted by a privileged user; otherwise, only the system can purge the object at the end of the retention interval."]
    #[serde(rename = "recoveryLevel", default, skip_serializing_if = "Option::is_none")]
    pub recovery_level: Option<key_attributes::RecoveryLevel>,
    #[doc = "Indicates if the private key can be exported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exportable: Option<bool>,
}
impl KeyAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod key_attributes {
    use super::*;
    #[doc = "The deletion recovery level currently in effect for the object. If it contains 'Purgeable', then the object can be permanently deleted by a privileged user; otherwise, only the system can purge the object at the end of the retention interval."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryLevel")]
    pub enum RecoveryLevel {
        Purgeable,
        #[serde(rename = "Recoverable+Purgeable")]
        RecoverablePurgeable,
        Recoverable,
        #[serde(rename = "Recoverable+ProtectedSubscription")]
        RecoverableProtectedSubscription,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Purgeable => serializer.serialize_unit_variant("RecoveryLevel", 0u32, "Purgeable"),
                Self::RecoverablePurgeable => serializer.serialize_unit_variant("RecoveryLevel", 1u32, "Recoverable+Purgeable"),
                Self::Recoverable => serializer.serialize_unit_variant("RecoveryLevel", 2u32, "Recoverable"),
                Self::RecoverableProtectedSubscription => {
                    serializer.serialize_unit_variant("RecoveryLevel", 3u32, "Recoverable+ProtectedSubscription")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters used to create a key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyCreateParameters {
    #[doc = "The tags that will be assigned to the key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The properties of the key."]
    pub properties: KeyProperties,
}
impl KeyCreateParameters {
    pub fn new(properties: KeyProperties) -> Self {
        Self { tags: None, properties }
    }
}
#[doc = "The page of keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyListResult {
    #[doc = "The key resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Key>,
    #[doc = "The URL to get the next page of keys."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for KeyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl KeyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyProperties {
    #[doc = "The object attributes managed by the Azure Key Vault service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<KeyAttributes>,
    #[doc = "The type of the key. For valid values, see JsonWebKeyType."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kty: Option<key_properties::Kty>,
    #[serde(rename = "keyOps", default, skip_serializing_if = "Vec::is_empty")]
    pub key_ops: Vec<String>,
    #[doc = "The key size in bits. For example: 2048, 3072, or 4096 for RSA."]
    #[serde(rename = "keySize", default, skip_serializing_if = "Option::is_none")]
    pub key_size: Option<i32>,
    #[doc = "The elliptic curve name. For valid values, see JsonWebKeyCurveName."]
    #[serde(rename = "curveName", default, skip_serializing_if = "Option::is_none")]
    pub curve_name: Option<key_properties::CurveName>,
    #[doc = "The URI to retrieve the current version of the key."]
    #[serde(rename = "keyUri", default, skip_serializing_if = "Option::is_none")]
    pub key_uri: Option<String>,
    #[doc = "The URI to retrieve the specific version of the key."]
    #[serde(rename = "keyUriWithVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_uri_with_version: Option<String>,
    #[serde(rename = "rotationPolicy", default, skip_serializing_if = "Option::is_none")]
    pub rotation_policy: Option<RotationPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_policy: Option<KeyReleasePolicy>,
}
impl KeyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod key_properties {
    use super::*;
    #[doc = "The type of the key. For valid values, see JsonWebKeyType."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kty")]
    pub enum Kty {
        #[serde(rename = "EC")]
        Ec,
        #[serde(rename = "EC-HSM")]
        EcHsm,
        #[serde(rename = "RSA")]
        Rsa,
        #[serde(rename = "RSA-HSM")]
        RsaHsm,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kty {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kty {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kty {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ec => serializer.serialize_unit_variant("Kty", 0u32, "EC"),
                Self::EcHsm => serializer.serialize_unit_variant("Kty", 1u32, "EC-HSM"),
                Self::Rsa => serializer.serialize_unit_variant("Kty", 2u32, "RSA"),
                Self::RsaHsm => serializer.serialize_unit_variant("Kty", 3u32, "RSA-HSM"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The elliptic curve name. For valid values, see JsonWebKeyCurveName."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CurveName")]
    pub enum CurveName {
        #[serde(rename = "P-256")]
        P256,
        #[serde(rename = "P-384")]
        P384,
        #[serde(rename = "P-521")]
        P521,
        #[serde(rename = "P-256K")]
        P256k,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CurveName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CurveName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CurveName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::P256 => serializer.serialize_unit_variant("CurveName", 0u32, "P-256"),
                Self::P384 => serializer.serialize_unit_variant("CurveName", 1u32, "P-384"),
                Self::P521 => serializer.serialize_unit_variant("CurveName", 2u32, "P-521"),
                Self::P256k => serializer.serialize_unit_variant("CurveName", 3u32, "P-256K"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyReleasePolicy {
    #[doc = "Content type and version of key release policy"]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "Blob encoding the policy rules under which the key can be released."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}
impl KeyReleasePolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyRotationPolicyAttributes {
    #[doc = "Creation time in seconds since 1970-01-01T00:00:00Z."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[doc = "Last updated time in seconds since 1970-01-01T00:00:00Z."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
    #[doc = "The expiration time for the new key version. It should be in ISO8601 format. Eg: 'P90D', 'P1Y'."]
    #[serde(rename = "expiryTime", default, skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
}
impl KeyRotationPolicyAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LifetimeAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
}
impl LifetimeAction {
    pub fn new() -> Self {
        Self::default()
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
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A rule governing the accessibility of a managed hsm pool from a specific ip address or ip range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MhsmipRule {
    #[doc = "An IPv4 address range in CIDR notation, such as '124.56.78.91' (simple IP address) or '124.56.78.0/24' (all addresses that start with 124.56.78)."]
    pub value: String,
}
impl MhsmipRule {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
#[doc = "A set of rules governing the network accessibility of a managed hsm pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MhsmNetworkRuleSet {
    #[doc = "Tells what traffic can bypass network rules. This can be 'AzureServices' or 'None'.  If not specified the default is 'AzureServices'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bypass: Option<mhsm_network_rule_set::Bypass>,
    #[doc = "The default action when no rule from ipRules and from virtualNetworkRules match. This is only used after the bypass property has been evaluated."]
    #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
    pub default_action: Option<mhsm_network_rule_set::DefaultAction>,
    #[doc = "The list of IP address rules."]
    #[serde(rename = "ipRules", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_rules: Vec<MhsmipRule>,
    #[doc = "The list of virtual network rules."]
    #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<MhsmVirtualNetworkRule>,
}
impl MhsmNetworkRuleSet {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod mhsm_network_rule_set {
    use super::*;
    #[doc = "Tells what traffic can bypass network rules. This can be 'AzureServices' or 'None'.  If not specified the default is 'AzureServices'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Bypass")]
    pub enum Bypass {
        AzureServices,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Bypass {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Bypass {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Bypass {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureServices => serializer.serialize_unit_variant("Bypass", 0u32, "AzureServices"),
                Self::None => serializer.serialize_unit_variant("Bypass", 1u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The default action when no rule from ipRules and from virtualNetworkRules match. This is only used after the bypass property has been evaluated."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DefaultAction")]
    pub enum DefaultAction {
        Allow,
        Deny,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DefaultAction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DefaultAction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DefaultAction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("DefaultAction", 0u32, "Allow"),
                Self::Deny => serializer.serialize_unit_variant("DefaultAction", 1u32, "Deny"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Private endpoint object properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MhsmPrivateEndpoint {
    #[doc = "Full identifier of the private endpoint resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl MhsmPrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MhsmPrivateEndpointConnection {
    #[serde(flatten)]
    pub managed_hsm_resource: ManagedHsmResource,
    #[doc = "Properties of the private endpoint connection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MhsmPrivateEndpointConnectionProperties>,
    #[doc = "Modified whenever there is a change in the state of private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl MhsmPrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint connection item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MhsmPrivateEndpointConnectionItem {
    #[doc = "Id of private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Modified whenever there is a change in the state of private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Properties of the private endpoint connection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MhsmPrivateEndpointConnectionProperties>,
}
impl MhsmPrivateEndpointConnectionItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the private endpoint connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MhsmPrivateEndpointConnectionProperties {
    #[doc = "Private endpoint object properties."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<MhsmPrivateEndpoint>,
    #[doc = "An object that represents the approval state of the private link connection."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<MhsmPrivateLinkServiceConnectionState>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<MhsmPrivateEndpointConnectionProvisioningState>,
}
impl MhsmPrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MhsmPrivateEndpointConnectionProvisioningState")]
pub enum MhsmPrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Updating,
    Deleting,
    Failed,
    Disconnected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MhsmPrivateEndpointConnectionProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MhsmPrivateEndpointConnectionProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MhsmPrivateEndpointConnectionProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("MhsmPrivateEndpointConnectionProvisioningState", 0u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("MhsmPrivateEndpointConnectionProvisioningState", 1u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("MhsmPrivateEndpointConnectionProvisioningState", 2u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("MhsmPrivateEndpointConnectionProvisioningState", 3u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("MhsmPrivateEndpointConnectionProvisioningState", 4u32, "Failed"),
            Self::Disconnected => serializer.serialize_unit_variant("MhsmPrivateEndpointConnectionProvisioningState", 5u32, "Disconnected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "List of private endpoint connections associated with a managed HSM Pools"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MhsmPrivateEndpointConnectionsListResult {
    #[doc = "The private endpoint connection associated with a managed HSM Pools."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MhsmPrivateEndpointConnection>,
    #[doc = "The URL to get the next set of managed HSM Pools."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MhsmPrivateEndpointConnectionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MhsmPrivateEndpointConnectionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The private endpoint connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MhsmPrivateEndpointServiceConnectionStatus")]
pub enum MhsmPrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    Disconnected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MhsmPrivateEndpointServiceConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MhsmPrivateEndpointServiceConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MhsmPrivateEndpointServiceConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("MhsmPrivateEndpointServiceConnectionStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("MhsmPrivateEndpointServiceConnectionStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("MhsmPrivateEndpointServiceConnectionStatus", 2u32, "Rejected"),
            Self::Disconnected => serializer.serialize_unit_variant("MhsmPrivateEndpointServiceConnectionStatus", 3u32, "Disconnected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MhsmPrivateLinkResource {
    #[serde(flatten)]
    pub managed_hsm_resource: ManagedHsmResource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MhsmPrivateLinkResourceProperties>,
}
impl MhsmPrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MhsmPrivateLinkResourceListResult {
    #[doc = "Array of private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MhsmPrivateLinkResource>,
}
impl MhsmPrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MhsmPrivateLinkResourceProperties {
    #[doc = "Group identifier of private link resource."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Required member names of private link resource."]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "Required DNS zone names of the the private link resource."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl MhsmPrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents the approval state of the private link connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MhsmPrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<MhsmPrivateEndpointServiceConnectionStatus>,
    #[doc = "The reason for approval or rejection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<mhsm_private_link_service_connection_state::ActionsRequired>,
}
impl MhsmPrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod mhsm_private_link_service_connection_state {
    use super::*;
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionsRequired")]
    pub enum ActionsRequired {
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionsRequired {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionsRequired {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionsRequired {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ActionsRequired", 0u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A rule governing the accessibility of a managed hsm pool from a specific virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MhsmVirtualNetworkRule {
    #[doc = "Full resource id of a vnet subnet, such as '/subscriptions/subid/resourceGroups/rg1/providers/Microsoft.Network/virtualNetworks/test-vnet/subnets/subnet1'."]
    pub id: String,
}
impl MhsmVirtualNetworkRule {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Resource information with extended details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedHsm {
    #[serde(flatten)]
    pub managed_hsm_resource: ManagedHsmResource,
    #[doc = "Properties of the managed HSM Pool"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedHsmProperties>,
}
impl ManagedHsm {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error exception."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedHsmError {
    #[doc = "The server error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
impl azure_core::Continuable for ManagedHsmError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ManagedHsmError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of managed HSM Pools"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedHsmListResult {
    #[doc = "The list of managed HSM Pools."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedHsm>,
    #[doc = "The URL to get the next set of managed HSM Pools."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedHsmListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagedHsmListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the managed HSM Pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedHsmProperties {
    #[doc = "The Azure Active Directory tenant ID that should be used for authenticating requests to the managed HSM pool."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Array of initial administrators object ids for this managed hsm pool."]
    #[serde(rename = "initialAdminObjectIds", default, skip_serializing_if = "Vec::is_empty")]
    pub initial_admin_object_ids: Vec<String>,
    #[doc = "The URI of the managed hsm pool for performing operations on keys."]
    #[serde(rename = "hsmUri", default, skip_serializing_if = "Option::is_none")]
    pub hsm_uri: Option<String>,
    #[doc = "Property to specify whether the 'soft delete' functionality is enabled for this managed HSM pool. If it's not set to any value(true or false) when creating new managed HSM pool, it will be set to true by default. Once set to true, it cannot be reverted to false."]
    #[serde(rename = "enableSoftDelete", default, skip_serializing_if = "Option::is_none")]
    pub enable_soft_delete: Option<bool>,
    #[doc = "softDelete data retention days. It accepts >=7 and <=90."]
    #[serde(rename = "softDeleteRetentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_retention_in_days: Option<i32>,
    #[doc = "Property specifying whether protection against purge is enabled for this managed HSM pool. Setting this property to true activates protection against purge for this managed HSM pool and its content - only the Managed HSM service may initiate a hard, irrecoverable deletion. The setting is effective only if soft delete is also enabled. Enabling this functionality is irreversible."]
    #[serde(rename = "enablePurgeProtection", default, skip_serializing_if = "Option::is_none")]
    pub enable_purge_protection: Option<bool>,
    #[doc = "The create mode to indicate whether the resource is being created or is being recovered from a deleted resource."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<managed_hsm_properties::CreateMode>,
    #[doc = "Resource Status Message."]
    #[serde(rename = "statusMessage", default, skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[doc = "Provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<managed_hsm_properties::ProvisioningState>,
    #[doc = "A set of rules governing the network accessibility of a managed hsm pool."]
    #[serde(rename = "networkAcls", default, skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<MhsmNetworkRuleSet>,
    #[doc = "List of private endpoint connections associated with the managed hsm pool."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<MhsmPrivateEndpointConnectionItem>,
    #[doc = "Control permission for data plane traffic coming from public networks while private endpoint is enabled."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<managed_hsm_properties::PublicNetworkAccess>,
    #[doc = "The scheduled purge date in UTC."]
    #[serde(rename = "scheduledPurgeDate", with = "azure_core::date::rfc3339::option")]
    pub scheduled_purge_date: Option<time::OffsetDateTime>,
}
impl ManagedHsmProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_hsm_properties {
    use super::*;
    #[doc = "The create mode to indicate whether the resource is being created or is being recovered from a deleted resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateMode {
        #[serde(rename = "recover")]
        Recover,
        #[serde(rename = "default")]
        Default,
    }
    #[doc = "Provisioning state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Provisioning,
        Failed,
        Updating,
        Deleting,
        Activated,
        SecurityDomainRestore,
        Restoring,
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
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Provisioning"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::Activated => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Activated"),
                Self::SecurityDomainRestore => serializer.serialize_unit_variant("ProvisioningState", 6u32, "SecurityDomainRestore"),
                Self::Restoring => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Restoring"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Control permission for data plane traffic coming from public networks while private endpoint is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Managed HSM resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedHsmResource {
    #[doc = "The Azure Resource Manager resource ID for the managed HSM Pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the managed HSM Pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type of the managed HSM Pool."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The supported Azure location where the managed HSM Pool should be created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "SKU details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ManagedHsmSku>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the key vault resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ManagedHsmResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedHsmSku {
    #[doc = "SKU Family of the managed HSM Pool"]
    pub family: managed_hsm_sku::Family,
    #[doc = "SKU of the managed HSM Pool"]
    pub name: managed_hsm_sku::Name,
}
impl ManagedHsmSku {
    pub fn new(family: managed_hsm_sku::Family, name: managed_hsm_sku::Name) -> Self {
        Self { family, name }
    }
}
pub mod managed_hsm_sku {
    use super::*;
    #[doc = "SKU Family of the managed HSM Pool"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Family")]
    pub enum Family {
        B,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Family {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Family {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Family {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::B => serializer.serialize_unit_variant("Family", 0u32, "B"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "SKU of the managed HSM Pool"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_B1")]
        StandardB1,
        #[serde(rename = "Custom_B32")]
        CustomB32,
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
    #[doc = "The supported aggregation types for the metrics."]
    #[serde(rename = "supportedAggregationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
    #[doc = "The supported time grain types for the metrics."]
    #[serde(rename = "supportedTimeGrainTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_time_grain_types: Vec<String>,
    #[doc = "The metric lock aggregation type."]
    #[serde(rename = "lockAggregationType", default, skip_serializing_if = "Option::is_none")]
    pub lock_aggregation_type: Option<String>,
    #[doc = "The dimensions of metric"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<DimensionProperties>,
    #[doc = "Property to specify whether to fill gap with zero."]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[doc = "The internal metric name."]
    #[serde(rename = "internalMetricName", default, skip_serializing_if = "Option::is_none")]
    pub internal_metric_name: Option<String>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A set of rules governing the network accessibility of a vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRuleSet {
    #[doc = "Tells what traffic can bypass network rules. This can be 'AzureServices' or 'None'.  If not specified the default is 'AzureServices'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bypass: Option<network_rule_set::Bypass>,
    #[doc = "The default action when no rule from ipRules and from virtualNetworkRules match. This is only used after the bypass property has been evaluated."]
    #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
    pub default_action: Option<network_rule_set::DefaultAction>,
    #[doc = "The list of IP address rules."]
    #[serde(rename = "ipRules", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_rules: Vec<IpRule>,
    #[doc = "The list of virtual network rules."]
    #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
}
impl NetworkRuleSet {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_rule_set {
    use super::*;
    #[doc = "Tells what traffic can bypass network rules. This can be 'AzureServices' or 'None'.  If not specified the default is 'AzureServices'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Bypass")]
    pub enum Bypass {
        AzureServices,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Bypass {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Bypass {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Bypass {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureServices => serializer.serialize_unit_variant("Bypass", 0u32, "AzureServices"),
                Self::None => serializer.serialize_unit_variant("Bypass", 1u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The default action when no rule from ipRules and from virtualNetworkRules match. This is only used after the bypass property has been evaluated."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DefaultAction")]
    pub enum DefaultAction {
        Allow,
        Deny,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DefaultAction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DefaultAction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DefaultAction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("DefaultAction", 0u32, "Allow"),
                Self::Deny => serializer.serialize_unit_variant("DefaultAction", 1u32, "Deny"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Key Vault REST API operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The origin of operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Properties of operation, include metric specifications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
    #[doc = "Property to specify whether the action is a data action."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft Key Vault."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
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
#[doc = "Result of the request to list Storage operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Storage operations supported by the Storage resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The URL to get the next set of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Permissions the identity has for keys, secrets, certificates and storage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Permissions {
    #[doc = "Permissions to keys"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<String>,
    #[doc = "Permissions to secrets"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<String>,
    #[doc = "Permissions to certificates"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certificates: Vec<String>,
    #[doc = "Permissions to storage accounts"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub storage: Vec<String>,
}
impl Permissions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint object properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "Full identifier of the private endpoint resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the private endpoint connection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
    #[doc = "Modified whenever there is a change in the state of private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint connection item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionItem {
    #[doc = "Id of private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Modified whenever there is a change in the state of private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Properties of the private endpoint connection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnectionItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of private endpoint connections."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "The list of private endpoint connections."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "The URL to get the next set of private endpoint connections."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the private endpoint connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "Private endpoint object properties."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "An object that represents the approval state of the private link connection."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointConnectionProvisioningState")]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Updating,
    Deleting,
    Failed,
    Disconnected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointConnectionProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointConnectionProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointConnectionProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 0u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 1u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 2u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 3u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 4u32, "Failed"),
            Self::Disconnected => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 5u32, "Disconnected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The private endpoint connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointServiceConnectionStatus")]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    Disconnected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointServiceConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointServiceConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointServiceConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 2u32, "Rejected"),
            Self::Disconnected => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 3u32, "Disconnected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "Array of private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "Group identifier of private link resource."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Required member names of private link resource."]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "Required DNS zone names of the the private link resource."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents the approval state of the private link connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[doc = "The reason for approval or rejection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<private_link_service_connection_state::ActionsRequired>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_link_service_connection_state {
    use super::*;
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionsRequired")]
    pub enum ActionsRequired {
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionsRequired {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionsRequired {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionsRequired {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ActionsRequired", 0u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Key Vault resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified identifier of the key vault resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the key vault resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type of the key vault resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Azure location of the key vault resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Tags assigned to the key vault resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of vault resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceListResult {
    #[doc = "The list of vault resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Resource>,
    #[doc = "The URL to get the next set of vault resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RotationPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<KeyRotationPolicyAttributes>,
    #[doc = "The lifetimeActions for key rotation action."]
    #[serde(rename = "lifetimeActions", default, skip_serializing_if = "Vec::is_empty")]
    pub lifetime_actions: Vec<LifetimeAction>,
}
impl RotationPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource information with extended details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Secret {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the secret"]
    pub properties: SecretProperties,
}
impl Secret {
    pub fn new(properties: SecretProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "The secret management attributes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretAttributes {
    #[serde(flatten)]
    pub attributes: Attributes,
}
impl SecretAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for creating or updating a secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretCreateOrUpdateParameters {
    #[doc = "The tags that will be assigned to the secret. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of the secret"]
    pub properties: SecretProperties,
}
impl SecretCreateOrUpdateParameters {
    pub fn new(properties: SecretProperties) -> Self {
        Self { tags: None, properties }
    }
}
#[doc = "List of secrets"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretListResult {
    #[doc = "The list of secrets."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Secret>,
    #[doc = "The URL to get the next set of secrets."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecretListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SecretListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for patching a secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretPatchParameters {
    #[doc = "The tags that will be assigned to the secret. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of the secret"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecretPatchProperties>,
}
impl SecretPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretPatchProperties {
    #[doc = "The value of the secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The content type of the secret."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The secret management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SecretAttributes>,
}
impl SecretPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretProperties {
    #[doc = "The value of the secret. NOTE: 'value' will never be returned from the service, as APIs using this model are is intended for internal use in ARM deployments. Users should use the data-plane REST service for interaction with vault secrets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The content type of the secret."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The secret management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SecretAttributes>,
    #[doc = "The URI to retrieve the current version of the secret."]
    #[serde(rename = "secretUri", default, skip_serializing_if = "Option::is_none")]
    pub secret_uri: Option<String>,
    #[doc = "The URI to retrieve the specific version of the secret."]
    #[serde(rename = "secretUriWithVersion", default, skip_serializing_if = "Option::is_none")]
    pub secret_uri_with_version: Option<String>,
}
impl SecretProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "One property of operation, include log specifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Log specifications of operation."]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
    #[doc = "Metric specifications of operation."]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "SKU family name"]
    pub family: sku::Family,
    #[doc = "SKU name to specify whether the key vault is a standard vault or a premium vault."]
    pub name: sku::Name,
}
impl Sku {
    pub fn new(family: sku::Family, name: sku::Name) -> Self {
        Self { family, name }
    }
}
pub mod sku {
    use super::*;
    #[doc = "SKU family name"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Family")]
    pub enum Family {
        A,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Family {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Family {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Family {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::A => serializer.serialize_unit_variant("Family", 0u32, "A"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "SKU name to specify whether the key vault is a standard vault or a premium vault."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "standard")]
        Standard,
        #[serde(rename = "premium")]
        Premium,
    }
}
#[doc = "Metadata pertaining to creation and last modification of the key vault resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the key vault resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<IdentityType>,
    #[doc = "The timestamp of the key vault resource creation (UTC)."]
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the key vault resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<IdentityType>,
    #[doc = "The timestamp of the key vault resource last modification (UTC)."]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Trigger {
    #[doc = "The time duration after key creation to rotate the key. It only applies to rotate. It will be in ISO 8601 duration format. Eg: 'P90D', 'P1Y'."]
    #[serde(rename = "timeAfterCreate", default, skip_serializing_if = "Option::is_none")]
    pub time_after_create: Option<String>,
    #[doc = "The time duration before key expiring to rotate or notify. It will be in ISO 8601 duration format. Eg: 'P90D', 'P1Y'."]
    #[serde(rename = "timeBeforeExpiry", default, skip_serializing_if = "Option::is_none")]
    pub time_before_expiry: Option<String>,
}
impl Trigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource information with extended details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vault {
    #[doc = "Fully qualified identifier of the key vault resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the key vault resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type of the key vault resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Azure location of the key vault resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Tags assigned to the key vault resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the key vault resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of the vault"]
    pub properties: VaultProperties,
}
impl Vault {
    pub fn new(properties: VaultProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location: None,
            tags: None,
            system_data: None,
            properties,
        }
    }
}
#[doc = "Parameters for updating the access policy in a vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultAccessPolicyParameters {
    #[doc = "The resource id of the access policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name of the access policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource name of the access policy."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource type of the access policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties of the vault access policy"]
    pub properties: VaultAccessPolicyProperties,
}
impl VaultAccessPolicyParameters {
    pub fn new(properties: VaultAccessPolicyProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location: None,
            properties,
        }
    }
}
#[doc = "Properties of the vault access policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultAccessPolicyProperties {
    #[doc = "An array of 0 to 16 identities that have access to the key vault. All identities in the array must use the same tenant ID as the key vault's tenant ID."]
    #[serde(rename = "accessPolicies")]
    pub access_policies: Vec<AccessPolicyEntry>,
}
impl VaultAccessPolicyProperties {
    pub fn new(access_policies: Vec<AccessPolicyEntry>) -> Self {
        Self { access_policies }
    }
}
#[doc = "The parameters used to check the availability of the vault name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultCheckNameAvailabilityParameters {
    #[doc = "The vault name."]
    pub name: String,
    #[doc = "The type of resource, Microsoft.KeyVault/vaults"]
    #[serde(rename = "type")]
    pub type_: vault_check_name_availability_parameters::Type,
}
impl VaultCheckNameAvailabilityParameters {
    pub fn new(name: String, type_: vault_check_name_availability_parameters::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod vault_check_name_availability_parameters {
    use super::*;
    #[doc = "The type of resource, Microsoft.KeyVault/vaults"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.KeyVault/vaults")]
        MicrosoftKeyVaultVaults,
    }
}
#[doc = "Parameters for creating or updating a vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultCreateOrUpdateParameters {
    #[doc = "The supported Azure location where the key vault should be created."]
    pub location: String,
    #[doc = "The tags that will be assigned to the key vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of the vault"]
    pub properties: VaultProperties,
}
impl VaultCreateOrUpdateParameters {
    pub fn new(location: String, properties: VaultProperties) -> Self {
        Self {
            location,
            tags: None,
            properties,
        }
    }
}
#[doc = "List of vaults"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultListResult {
    #[doc = "The list of vaults."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Vault>,
    #[doc = "The URL to get the next set of vaults."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VaultListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VaultListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for creating or updating a vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultPatchParameters {
    #[doc = "The tags that will be assigned to the key vault. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of the vault"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultPatchProperties>,
}
impl VaultPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultPatchProperties {
    #[doc = "The Azure Active Directory tenant ID that should be used for authenticating requests to the key vault."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "SKU details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "An array of 0 to 16 identities that have access to the key vault. All identities in the array must use the same tenant ID as the key vault's tenant ID."]
    #[serde(rename = "accessPolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub access_policies: Vec<AccessPolicyEntry>,
    #[doc = "Property to specify whether Azure Virtual Machines are permitted to retrieve certificates stored as secrets from the key vault."]
    #[serde(rename = "enabledForDeployment", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_deployment: Option<bool>,
    #[doc = "Property to specify whether Azure Disk Encryption is permitted to retrieve secrets from the vault and unwrap keys."]
    #[serde(rename = "enabledForDiskEncryption", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_disk_encryption: Option<bool>,
    #[doc = "Property to specify whether Azure Resource Manager is permitted to retrieve secrets from the key vault."]
    #[serde(rename = "enabledForTemplateDeployment", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_template_deployment: Option<bool>,
    #[doc = "Property to specify whether the 'soft delete' functionality is enabled for this key vault. Once set to true, it cannot be reverted to false."]
    #[serde(rename = "enableSoftDelete", default, skip_serializing_if = "Option::is_none")]
    pub enable_soft_delete: Option<bool>,
    #[doc = "Property that controls how data actions are authorized. When true, the key vault will use Role Based Access Control (RBAC) for authorization of data actions, and the access policies specified in vault properties will be  ignored (warning: this is a preview feature). When false, the key vault will use the access policies specified in vault properties, and any policy stored on Azure Resource Manager will be ignored. If null or not specified, the value of this property will not change."]
    #[serde(rename = "enableRbacAuthorization", default, skip_serializing_if = "Option::is_none")]
    pub enable_rbac_authorization: Option<bool>,
    #[doc = "softDelete data retention days. It accepts >=7 and <=90."]
    #[serde(rename = "softDeleteRetentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_retention_in_days: Option<i32>,
    #[doc = "The vault's create mode to indicate whether the vault need to be recovered or not."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<vault_patch_properties::CreateMode>,
    #[doc = "Property specifying whether protection against purge is enabled for this vault. Setting this property to true activates protection against purge for this vault and its content - only the Key Vault service may initiate a hard, irrecoverable deletion. The setting is effective only if soft delete is also enabled. Enabling this functionality is irreversible - that is, the property does not accept false as its value."]
    #[serde(rename = "enablePurgeProtection", default, skip_serializing_if = "Option::is_none")]
    pub enable_purge_protection: Option<bool>,
    #[doc = "A set of rules governing the network accessibility of a vault."]
    #[serde(rename = "networkAcls", default, skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
    #[doc = "Property to specify whether the vault will accept traffic from public internet. If set to 'disabled' all traffic except private endpoint traffic and that that originates from trusted services will be blocked. This will override the set firewall rules, meaning that even if the firewall rules are present we will not honor the rules."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<String>,
}
impl VaultPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vault_patch_properties {
    use super::*;
    #[doc = "The vault's create mode to indicate whether the vault need to be recovered or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateMode {
        #[serde(rename = "recover")]
        Recover,
        #[serde(rename = "default")]
        Default,
    }
}
#[doc = "Properties of the vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultProperties {
    #[doc = "The Azure Active Directory tenant ID that should be used for authenticating requests to the key vault."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[doc = "SKU details"]
    pub sku: Sku,
    #[doc = "An array of 0 to 1024 identities that have access to the key vault. All identities in the array must use the same tenant ID as the key vault's tenant ID. When `createMode` is set to `recover`, access policies are not required. Otherwise, access policies are required."]
    #[serde(rename = "accessPolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub access_policies: Vec<AccessPolicyEntry>,
    #[doc = "The URI of the vault for performing operations on keys and secrets."]
    #[serde(rename = "vaultUri", default, skip_serializing_if = "Option::is_none")]
    pub vault_uri: Option<String>,
    #[doc = "The resource id of HSM Pool."]
    #[serde(rename = "hsmPoolResourceId", default, skip_serializing_if = "Option::is_none")]
    pub hsm_pool_resource_id: Option<String>,
    #[doc = "Property to specify whether Azure Virtual Machines are permitted to retrieve certificates stored as secrets from the key vault."]
    #[serde(rename = "enabledForDeployment", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_deployment: Option<bool>,
    #[doc = "Property to specify whether Azure Disk Encryption is permitted to retrieve secrets from the vault and unwrap keys."]
    #[serde(rename = "enabledForDiskEncryption", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_disk_encryption: Option<bool>,
    #[doc = "Property to specify whether Azure Resource Manager is permitted to retrieve secrets from the key vault."]
    #[serde(rename = "enabledForTemplateDeployment", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_template_deployment: Option<bool>,
    #[doc = "Property to specify whether the 'soft delete' functionality is enabled for this key vault. If it's not set to any value(true or false) when creating new key vault, it will be set to true by default. Once set to true, it cannot be reverted to false."]
    #[serde(rename = "enableSoftDelete", default, skip_serializing_if = "Option::is_none")]
    pub enable_soft_delete: Option<bool>,
    #[doc = "softDelete data retention days. It accepts >=7 and <=90."]
    #[serde(rename = "softDeleteRetentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_retention_in_days: Option<i32>,
    #[doc = "Property that controls how data actions are authorized. When true, the key vault will use Role Based Access Control (RBAC) for authorization of data actions, and the access policies specified in vault properties will be  ignored (warning: this is a preview feature). When false, the key vault will use the access policies specified in vault properties, and any policy stored on Azure Resource Manager will be ignored. If null or not specified, the vault is created with the default value of false. Note that management actions are always authorized with RBAC."]
    #[serde(rename = "enableRbacAuthorization", default, skip_serializing_if = "Option::is_none")]
    pub enable_rbac_authorization: Option<bool>,
    #[doc = "The vault's create mode to indicate whether the vault need to be recovered or not."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<vault_properties::CreateMode>,
    #[doc = "Property specifying whether protection against purge is enabled for this vault. Setting this property to true activates protection against purge for this vault and its content - only the Key Vault service may initiate a hard, irrecoverable deletion. The setting is effective only if soft delete is also enabled. Enabling this functionality is irreversible - that is, the property does not accept false as its value."]
    #[serde(rename = "enablePurgeProtection", default, skip_serializing_if = "Option::is_none")]
    pub enable_purge_protection: Option<bool>,
    #[doc = "A set of rules governing the network accessibility of a vault."]
    #[serde(rename = "networkAcls", default, skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
    #[doc = "Provisioning state of the vault."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<vault_properties::ProvisioningState>,
    #[doc = "List of private endpoint connections associated with the key vault."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnectionItem>,
    #[doc = "Property to specify whether the vault will accept traffic from public internet. If set to 'disabled' all traffic except private endpoint traffic and that that originates from trusted services will be blocked. This will override the set firewall rules, meaning that even if the firewall rules are present we will not honor the rules."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<String>,
}
impl VaultProperties {
    pub fn new(tenant_id: String, sku: Sku) -> Self {
        Self {
            tenant_id,
            sku,
            access_policies: Vec::new(),
            vault_uri: None,
            hsm_pool_resource_id: None,
            enabled_for_deployment: None,
            enabled_for_disk_encryption: None,
            enabled_for_template_deployment: None,
            enable_soft_delete: None,
            soft_delete_retention_in_days: None,
            enable_rbac_authorization: None,
            create_mode: None,
            enable_purge_protection: None,
            network_acls: None,
            provisioning_state: None,
            private_endpoint_connections: Vec::new(),
            public_network_access: None,
        }
    }
}
pub mod vault_properties {
    use super::*;
    #[doc = "The vault's create mode to indicate whether the vault need to be recovered or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateMode {
        #[serde(rename = "recover")]
        Recover,
        #[serde(rename = "default")]
        Default,
    }
    #[doc = "Provisioning state of the vault."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        RegisteringDns,
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
                Self::RegisteringDns => serializer.serialize_unit_variant("ProvisioningState", 1u32, "RegisteringDns"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A rule governing the accessibility of a vault from a specific virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    #[doc = "Full resource id of a vnet subnet, such as '/subscriptions/subid/resourceGroups/rg1/providers/Microsoft.Network/virtualNetworks/test-vnet/subnets/subnet1'."]
    pub id: String,
    #[doc = "Property to specify whether NRP will ignore the check if parent subnet has serviceEndpoints configured."]
    #[serde(rename = "ignoreMissingVnetServiceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
}
impl VirtualNetworkRule {
    pub fn new(id: String) -> Self {
        Self {
            id,
            ignore_missing_vnet_service_endpoint: None,
        }
    }
}

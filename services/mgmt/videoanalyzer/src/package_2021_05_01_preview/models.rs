#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Policy that determines how a video can be accessed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessPolicyEntity {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Application level properties for the access policy resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessPolicyProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl AccessPolicyEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of AccessPolicyEntity items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessPolicyEntityCollection {
    #[doc = "A collection of AccessPolicyEntity items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AccessPolicyEntity>,
    #[doc = "A link to the next page of the collection (when the collection contains too many results to return in one response)."]
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessPolicyEntityCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AccessPolicyEntityCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application level properties for the access policy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessPolicyProperties {
    #[doc = "Defines the access level granted by this policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<access_policy_properties::Role>,
    #[doc = "Base class for access policies authentication methods."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<AuthenticationBase>,
}
impl AccessPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_policy_properties {
    use super::*;
    #[doc = "Defines the access level granted by this policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Role")]
    pub enum Role {
        Reader,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Role {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Role {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Role {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Reader => serializer.serialize_unit_variant("Role", 0u32, "Reader"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines how the Video Analyzer account is (optionally) encrypted."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountEncryption {
    #[doc = "The type of key used to encrypt the Account Key."]
    #[serde(rename = "type")]
    pub type_: account_encryption::Type,
    #[doc = "The details for accessing the encryption keys in Key Vault."]
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
    #[doc = "The user assigned managed identity to use when accessing a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[doc = "The current status of the Key Vault mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl AccountEncryption {
    pub fn new(type_: account_encryption::Type) -> Self {
        Self {
            type_,
            key_vault_properties: None,
            identity: None,
            status: None,
        }
    }
}
pub mod account_encryption {
    use super::*;
    #[doc = "The type of key used to encrypt the Account Key."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        SystemKey,
        CustomerKey,
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
                Self::SystemKey => serializer.serialize_unit_variant("Type", 0u32, "SystemKey"),
                Self::CustomerKey => serializer.serialize_unit_variant("Type", 1u32, "CustomerKey"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Base class for access policies authentication methods."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationBase {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
}
impl AuthenticationBase {
    pub fn new(type_: String) -> Self {
        Self { type_ }
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
#[doc = "Required validation properties for tokens generated with Elliptical Curve algorithm."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EccTokenKey {
    #[serde(flatten)]
    pub token_key: TokenKey,
    #[doc = "Elliptical curve algorithm to be used: ES256, ES384 or ES512."]
    pub alg: ecc_token_key::Alg,
    #[doc = "X coordinate."]
    pub x: String,
    #[doc = "Y coordinate."]
    pub y: String,
}
impl EccTokenKey {
    pub fn new(token_key: TokenKey, alg: ecc_token_key::Alg, x: String, y: String) -> Self {
        Self { token_key, alg, x, y }
    }
}
pub mod ecc_token_key {
    use super::*;
    #[doc = "Elliptical curve algorithm to be used: ES256, ES384 or ES512."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Alg")]
    pub enum Alg {
        #[serde(rename = "ES256")]
        Es256,
        #[serde(rename = "ES384")]
        Es384,
        #[serde(rename = "ES512")]
        Es512,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Alg {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Alg {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Alg {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Es256 => serializer.serialize_unit_variant("Alg", 0u32, "ES256"),
                Self::Es384 => serializer.serialize_unit_variant("Alg", 1u32, "ES384"),
                Self::Es512 => serializer.serialize_unit_variant("Alg", 2u32, "ES512"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The representation of an edge module."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgeModuleEntity {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Application level properties for the edge module resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EdgeModuleProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl EdgeModuleEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of EdgeModuleEntity items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgeModuleEntityCollection {
    #[doc = "A collection of EdgeModuleEntity items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EdgeModuleEntity>,
    #[doc = "A link to the next page of the collection (when the collection contains too many results to return in one response)."]
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EdgeModuleEntityCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EdgeModuleEntityCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application level properties for the edge module resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgeModuleProperties {
    #[doc = "Internal ID generated for the instance of the Video Analyzer edge module."]
    #[serde(rename = "edgeModuleId", default, skip_serializing_if = "Option::is_none")]
    pub edge_module_id: Option<String>,
}
impl EdgeModuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provisioning token properties. A provisioning token allows for a single instance of Azure Video analyzer IoT edge module to be initialized and authorized to the cloud account. The provisioning token itself is short lived and it is only used for the initial handshake between IoT edge module and the cloud. After the initial handshake, the IoT edge module will agree on a set of authentication keys which will be auto-rotated as long as the module is able to periodically connect to the cloud. A new provisioning token can be generated for the same IoT edge module in case the module state lost or reset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgeModuleProvisioningToken {
    #[doc = "The expiration date of the registration token. The Azure Video Analyzer IoT edge module must be initialized and connected to the Internet prior to the token expiration date."]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "The token blob to be provided to the Azure Video Analyzer IoT edge module through the Azure IoT Edge module twin properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl EdgeModuleProvisioningToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The endpoint details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Endpoint {
    #[doc = "The URL of the endpoint."]
    #[serde(rename = "endpointUrl", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[doc = "The type of the endpoint."]
    #[serde(rename = "type")]
    pub type_: endpoint::Type,
}
impl Endpoint {
    pub fn new(type_: endpoint::Type) -> Self {
        Self { endpoint_url: None, type_ }
    }
}
pub mod endpoint {
    use super::*;
    #[doc = "The type of the endpoint."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        ClientApi,
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
                Self::ClientApi => serializer.serialize_unit_variant("Type", 0u32, "ClientApi"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Properties for access validation based on JSON Web Tokens (JWT)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JwtAuthentication {
    #[serde(flatten)]
    pub authentication_base: AuthenticationBase,
    #[doc = "List of expected token issuers. Token issuer is valid if it matches at least one of the given values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issuers: Vec<String>,
    #[doc = "List of expected token audiences. Token audience is valid if it matches at least one of the given values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audiences: Vec<String>,
    #[doc = "List of additional token claims to be validated. Token must contains all claims and respective values for it to be valid."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub claims: Vec<TokenClaim>,
    #[doc = "List of keys which can be used to validate access tokens. Having multiple keys allow for seamless key rotation of the token signing key. Token signature must match exactly one key."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<TokenKey>,
}
impl JwtAuthentication {
    pub fn new(authentication_base: AuthenticationBase) -> Self {
        Self {
            authentication_base,
            issuers: Vec::new(),
            audiences: Vec::new(),
            claims: Vec::new(),
            keys: Vec::new(),
        }
    }
}
#[doc = "The details for accessing the encryption keys in Key Vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultProperties {
    #[doc = "The URL of the Key Vault key used to encrypt the account. The key may either be versioned (for example https://vault/keys/mykey/version1) or reference a key without a version (for example https://vault/keys/mykey)."]
    #[serde(rename = "keyIdentifier")]
    pub key_identifier: String,
    #[doc = "The current key used to encrypt Video Analyzer account, including the key version."]
    #[serde(rename = "currentKeyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub current_key_identifier: Option<String>,
}
impl KeyVaultProperties {
    pub fn new(key_identifier: String) -> Self {
        Self {
            key_identifier,
            current_key_identifier: None,
        }
    }
}
#[doc = "The input parameters to generate registration token for the Azure Video Analyzer IoT edge module."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListProvisioningTokenInput {
    #[doc = "The desired expiration date of the registration token. The Azure Video Analyzer IoT edge module must be initialized and connected to the Internet prior to the token expiration date."]
    #[serde(rename = "expirationDate", with = "azure_core::date::rfc3339")]
    pub expiration_date: time::OffsetDateTime,
}
impl ListProvisioningTokenInput {
    pub fn new(expiration_date: time::OffsetDateTime) -> Self {
        Self { expiration_date }
    }
}
#[doc = "A diagnostic log emitted by service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[doc = "The diagnostic log category name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The diagnostic log category display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The time range for requests in each blob."]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A metric dimension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDimension {
    #[doc = "The metric dimension name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The display name for the dimension."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Whether to export metric to shoebox."]
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl MetricDimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A metric emitted by service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[doc = "The metric name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The metric display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The metric display description."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "The metric unit"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<metric_specification::Unit>,
    #[doc = "The metric aggregation type"]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<metric_specification::AggregationType>,
    #[doc = "The metric lock aggregation type"]
    #[serde(rename = "lockAggregationType", default, skip_serializing_if = "Option::is_none")]
    pub lock_aggregation_type: Option<metric_specification::LockAggregationType>,
    #[doc = "Supported aggregation types."]
    #[serde(rename = "supportedAggregationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
    #[doc = "The metric dimensions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<MetricDimension>,
    #[doc = "Indicates whether regional MDM account is enabled."]
    #[serde(rename = "enableRegionalMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub enable_regional_mdm_account: Option<bool>,
    #[doc = "The source MDM account."]
    #[serde(rename = "sourceMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_account: Option<String>,
    #[doc = "The source MDM namespace."]
    #[serde(rename = "sourceMdmNamespace", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_namespace: Option<String>,
    #[doc = "The supported time grain types."]
    #[serde(rename = "supportedTimeGrainTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_time_grain_types: Vec<String>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod metric_specification {
    use super::*;
    #[doc = "The metric unit"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Bytes,
        Count,
        Milliseconds,
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
                Self::Bytes => serializer.serialize_unit_variant("Unit", 0u32, "Bytes"),
                Self::Count => serializer.serialize_unit_variant("Unit", 1u32, "Count"),
                Self::Milliseconds => serializer.serialize_unit_variant("Unit", 2u32, "Milliseconds"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The metric aggregation type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AggregationType")]
    pub enum AggregationType {
        Average,
        Count,
        Total,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AggregationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AggregationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AggregationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Average => serializer.serialize_unit_variant("AggregationType", 0u32, "Average"),
                Self::Count => serializer.serialize_unit_variant("AggregationType", 1u32, "Count"),
                Self::Total => serializer.serialize_unit_variant("AggregationType", 2u32, "Total"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The metric lock aggregation type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LockAggregationType")]
    pub enum LockAggregationType {
        Average,
        Count,
        Total,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LockAggregationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LockAggregationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LockAggregationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Average => serializer.serialize_unit_variant("LockAggregationType", 0u32, "Average"),
                Self::Count => serializer.serialize_unit_variant("LockAggregationType", 1u32, "Count"),
                Self::Total => serializer.serialize_unit_variant("LockAggregationType", 2u32, "Total"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[doc = "The operation name."]
    pub name: String,
    #[doc = "Operation details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Origin of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Metric properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Properties>,
    #[doc = "Whether the operation applies to data-plane."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Indicates the action type."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
}
impl Operation {
    pub fn new(name: String) -> Self {
        Self {
            name,
            display: None,
            origin: None,
            properties: None,
            is_data_action: None,
            action_type: None,
        }
    }
}
pub mod operation {
    use super::*;
    #[doc = "Indicates the action type."]
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
#[doc = "A collection of Operation items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationCollection {
    #[doc = "A collection of Operation items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl OperationCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "The service provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The operation type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metric properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Properties {
    #[doc = "The service metric specifications."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl Properties {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The user assigned managed identity to use when accessing a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceIdentity {
    #[doc = "The user assigned managed identity's resource identifier to use when accessing a resource."]
    #[serde(rename = "userAssignedIdentity")]
    pub user_assigned_identity: String,
}
impl ResourceIdentity {
    pub fn new(user_assigned_identity: String) -> Self {
        Self { user_assigned_identity }
    }
}
#[doc = "Required validation properties for tokens generated with RSA algorithm."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RsaTokenKey {
    #[serde(flatten)]
    pub token_key: TokenKey,
    #[doc = "RSA algorithm to be used: RS256, RS384 or RS512."]
    pub alg: rsa_token_key::Alg,
    #[doc = "RSA public key modulus."]
    pub n: String,
    #[doc = "RSA public key exponent."]
    pub e: String,
}
impl RsaTokenKey {
    pub fn new(token_key: TokenKey, alg: rsa_token_key::Alg, n: String, e: String) -> Self {
        Self { token_key, alg, n, e }
    }
}
pub mod rsa_token_key {
    use super::*;
    #[doc = "RSA algorithm to be used: RS256, RS384 or RS512."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Alg")]
    pub enum Alg {
        #[serde(rename = "RS256")]
        Rs256,
        #[serde(rename = "RS384")]
        Rs384,
        #[serde(rename = "RS512")]
        Rs512,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Alg {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Alg {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Alg {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Rs256 => serializer.serialize_unit_variant("Alg", 0u32, "RS256"),
                Self::Rs384 => serializer.serialize_unit_variant("Alg", 1u32, "RS384"),
                Self::Rs512 => serializer.serialize_unit_variant("Alg", 2u32, "RS512"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The service metric specifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "List of log specifications."]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
    #[doc = "List of metric specifications."]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details about the associated storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccount {
    #[doc = "The ID of the storage account resource. Video Analyzer relies on tables, queues, and blobs. The primary storage account must be a Standard Storage account (either Microsoft.ClassicStorage or Microsoft.Storage)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The user assigned managed identity to use when accessing a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[doc = "The current status of the storage account mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl StorageAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The input to the sync storage keys request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncStorageKeysInput {
    #[doc = "The ID of the storage account resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SyncStorageKeysInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for expected token claims."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenClaim {
    #[doc = "Name of the claim which must be present on the token."]
    pub name: String,
    #[doc = "Expected value of the claim to be present on the token."]
    pub value: String,
}
impl TokenClaim {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[doc = "Key properties for JWT token validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenKey {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
    #[doc = "JWT token key id. Validation keys are looked up based on the key id present on the JWT token header."]
    pub kid: String,
}
impl TokenKey {
    pub fn new(type_: String, kid: String) -> Self {
        Self { type_, kid }
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
#[doc = "The User Assigned Managed Identities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedManagedIdentities {}
impl UserAssignedManagedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the user assigned managed identity used by the Video Analyzer resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedManagedIdentity {
    #[doc = "The client ID."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The principal ID."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
}
impl UserAssignedManagedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Video Analyzer account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoAnalyzer {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VideoAnalyzerProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The managed identity for the Video Analyzer resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<VideoAnalyzerIdentity>,
}
impl VideoAnalyzer {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            system_data: None,
            identity: None,
        }
    }
}
#[doc = "A collection of VideoAnalyzer items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoAnalyzerCollection {
    #[doc = "A collection of VideoAnalyzer items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VideoAnalyzer>,
}
impl VideoAnalyzerCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed identity for the Video Analyzer resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoAnalyzerIdentity {
    #[doc = "The identity type."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The User Assigned Managed Identities."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedManagedIdentities>,
}
impl VideoAnalyzerIdentity {
    pub fn new(type_: String) -> Self {
        Self {
            type_,
            user_assigned_identities: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoAnalyzerProperties {
    #[serde(flatten)]
    pub video_analyzer_properties_update: VideoAnalyzerPropertiesUpdate,
}
impl VideoAnalyzerProperties {
    pub fn new() -> Self {
        Self {
            video_analyzer_properties_update: VideoAnalyzerPropertiesUpdate::default(),
        }
    }
}
#[doc = "Properties of the Video Analyzer account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoAnalyzerPropertiesUpdate {
    #[doc = "The storage accounts for this resource."]
    #[serde(rename = "storageAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub storage_accounts: Vec<StorageAccount>,
    #[doc = "The list of endpoints associated with this resource."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<Endpoint>,
    #[doc = "Defines how the Video Analyzer account is (optionally) encrypted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<AccountEncryption>,
}
impl VideoAnalyzerPropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The update operation for a Video Analyzer account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoAnalyzerUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of the Video Analyzer account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VideoAnalyzerPropertiesUpdate>,
    #[doc = "The managed identity for the Video Analyzer resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<VideoAnalyzerIdentity>,
}
impl VideoAnalyzerUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The representation of a single video in a Video Analyzer account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoEntity {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Application level properties for the video resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VideoProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl VideoEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of VideoEntity items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoEntityCollection {
    #[doc = "A collection of VideoEntity items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VideoEntity>,
    #[doc = "A link to the next page of the collection (when the collection contains too many results to return in one response)."]
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VideoEntityCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VideoEntityCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Video flags contain information about the available video actions and its dynamic properties based on the current video state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoFlags {
    #[doc = "Value indicating whether or not the video can be streamed. Only \"archive\" type videos can be streamed."]
    #[serde(rename = "canStream")]
    pub can_stream: bool,
    #[doc = "Value indicating whether or not there has ever been data recorded or uploaded into the video. Newly created videos have this value set to false."]
    #[serde(rename = "hasData")]
    pub has_data: bool,
    #[doc = "Value indicating whether or not the video is currently being referenced be an active live pipeline. The fact that is being referenced, doesn't necessarily indicate that data is being received. For example, video recording may be gated on events or camera may not be accessible at the time."]
    #[serde(rename = "isRecording")]
    pub is_recording: bool,
}
impl VideoFlags {
    pub fn new(can_stream: bool, has_data: bool, is_recording: bool) -> Self {
        Self {
            can_stream,
            has_data,
            is_recording,
        }
    }
}
#[doc = "Contains information about the video and audio content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoMediaInfo {
    #[doc = "Video segment length indicates the length of individual video files (segments) which are persisted to storage. Smaller segments provide lower archive playback latency but generate larger volume of storage transactions. Larger segments reduce the amount of storage transactions while increasing the archive playback latency. Value must be specified in ISO8601 duration format (i.e. \"PT30S\" equals 30 seconds) and can vary between 30 seconds to 5 minutes, in 30 seconds increments."]
    #[serde(rename = "segmentLength", default, skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<String>,
}
impl VideoMediaInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application level properties for the video resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoProperties {
    #[doc = "Optional video title provided by the user. Value can be up to 256 characters long."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Optional video description provided by the user. Value can be up to 2048 characters long."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Type of the video archive. Different archive formats provide different capabilities."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<video_properties::Type>,
    #[doc = "Video flags contain information about the available video actions and its dynamic properties based on the current video state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<VideoFlags>,
    #[doc = "Video streaming holds information about video streaming URLs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub streaming: Option<VideoStreaming>,
    #[doc = "Contains information about the video and audio content."]
    #[serde(rename = "mediaInfo", default, skip_serializing_if = "Option::is_none")]
    pub media_info: Option<VideoMediaInfo>,
}
impl VideoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod video_properties {
    use super::*;
    #[doc = "Type of the video archive. Different archive formats provide different capabilities."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Archive,
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
                Self::Archive => serializer.serialize_unit_variant("Type", 0u32, "Archive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Video streaming holds information about video streaming URLs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoStreaming {
    #[doc = "Video streaming base URL for the video archive. When present, archived video can be played through the Azure Video Analyzer player. Alternatively, this URL can be used with compatible DASH or HLS players by appending the following to the base URL:\r\n\r\n  - HLSv4:     /manifest(format=m3u8-aapl).m3u8\r\n  - HLS CMAF:  /manifest(format=m3u8-cmaf)\r\n  - DASH CMAF: /manifest(format=mpd-time-cmaf)\r\n\r\nMoreover, an ongoing video recording can be played in \"live mode\" with latencies which are approximately double of the chosen video segment length."]
    #[serde(rename = "archiveBaseUrl", default, skip_serializing_if = "Option::is_none")]
    pub archive_base_url: Option<String>,
}
impl VideoStreaming {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Video streaming token grants access to the video streaming URLs which can be used by an compatible HLS or DASH player."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoStreamingToken {
    #[doc = "The streaming token expiration date in ISO8601 format (eg. 2021-01-01T00:00:00Z)."]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "The streaming token value to be added to the video streaming URL as the value for a \"token\" query string parameter. The token is specific to a single video."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl VideoStreamingToken {
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

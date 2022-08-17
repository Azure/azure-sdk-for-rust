#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Access policies help define the authentication rules, and control access to specific video resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessPolicyEntity {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Application level properties for the access policy resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessPolicyProperties>,
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
#[doc = "A custom preset for encoding audio with the AAC codec."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioEncoderAac {
    #[serde(flatten)]
    pub audio_encoder_base: AudioEncoderBase,
}
impl AudioEncoderAac {
    pub fn new(audio_encoder_base: AudioEncoderBase) -> Self {
        Self { audio_encoder_base }
    }
}
#[doc = "Base type for all audio encoder presets, which define the recipe or instructions on how audio should be processed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioEncoderBase {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
    #[doc = "Bitrate, in kilobits per second or Kbps, at which audio should be encoded (2-channel stereo audio at a sampling rate of 48 kHz). Allowed values are 96, 112, 128, 160, 192, 224, and 256. If omitted, the bitrate of the input audio is used."]
    #[serde(rename = "bitrateKbps", default, skip_serializing_if = "Option::is_none")]
    pub bitrate_kbps: Option<String>,
}
impl AudioEncoderBase {
    pub fn new(type_: String) -> Self {
        Self { type_, bitrate_kbps: None }
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
#[doc = "Base class for certificate sources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateSource {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
}
impl CertificateSource {
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
#[doc = "Base class for credential objects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialsBase {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
}
impl CredentialsBase {
    pub fn new(type_: String) -> Self {
        Self { type_ }
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
#[doc = "Describes a custom preset for encoding the input content using the encoder processor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncoderCustomPreset {
    #[serde(flatten)]
    pub encoder_preset_base: EncoderPresetBase,
    #[doc = "Base type for all audio encoder presets, which define the recipe or instructions on how audio should be processed."]
    #[serde(rename = "audioEncoder", default, skip_serializing_if = "Option::is_none")]
    pub audio_encoder: Option<AudioEncoderBase>,
    #[doc = "Base type for all video encoding presets, which define the recipe or instructions on how the input video should be processed."]
    #[serde(rename = "videoEncoder", default, skip_serializing_if = "Option::is_none")]
    pub video_encoder: Option<VideoEncoderBase>,
}
impl EncoderCustomPreset {
    pub fn new(encoder_preset_base: EncoderPresetBase) -> Self {
        Self {
            encoder_preset_base,
            audio_encoder: None,
            video_encoder: None,
        }
    }
}
#[doc = "Base type for all encoder presets, which define the recipe or instructions on how the input content should be processed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncoderPresetBase {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
}
impl EncoderPresetBase {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "Encoder processor allows for encoding of the input content. For example, it can used to change the resolution from 4K to 1280x720."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncoderProcessor {
    #[serde(flatten)]
    pub processor_node_base: ProcessorNodeBase,
    #[doc = "Base type for all encoder presets, which define the recipe or instructions on how the input content should be processed."]
    pub preset: EncoderPresetBase,
}
impl EncoderProcessor {
    pub fn new(processor_node_base: ProcessorNodeBase, preset: EncoderPresetBase) -> Self {
        Self {
            processor_node_base,
            preset,
        }
    }
}
#[doc = "Describes a built-in preset for encoding the input content using the encoder processor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncoderSystemPreset {
    #[serde(flatten)]
    pub encoder_preset_base: EncoderPresetBase,
    #[doc = "Name of the built-in encoding preset."]
    pub name: encoder_system_preset::Name,
}
impl EncoderSystemPreset {
    pub fn new(encoder_preset_base: EncoderPresetBase, name: encoder_system_preset::Name) -> Self {
        Self { encoder_preset_base, name }
    }
}
pub mod encoder_system_preset {
    use super::*;
    #[doc = "Name of the built-in encoding preset."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        #[serde(rename = "SingleLayer_540p_H264_AAC")]
        SingleLayer540pH264Aac,
        #[serde(rename = "SingleLayer_720p_H264_AAC")]
        SingleLayer720pH264Aac,
        #[serde(rename = "SingleLayer_1080p_H264_AAC")]
        SingleLayer1080pH264Aac,
        #[serde(rename = "SingleLayer_2160p_H264_AAC")]
        SingleLayer2160pH264Aac,
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
                Self::SingleLayer540pH264Aac => serializer.serialize_unit_variant("Name", 0u32, "SingleLayer_540p_H264_AAC"),
                Self::SingleLayer720pH264Aac => serializer.serialize_unit_variant("Name", 1u32, "SingleLayer_720p_H264_AAC"),
                Self::SingleLayer1080pH264Aac => serializer.serialize_unit_variant("Name", 2u32, "SingleLayer_1080p_H264_AAC"),
                Self::SingleLayer2160pH264Aac => serializer.serialize_unit_variant("Name", 3u32, "SingleLayer_2160p_H264_AAC"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "Base class for endpoints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointBase {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
    #[doc = "Base class for credential objects."]
    pub credentials: CredentialsBase,
    #[doc = "The endpoint URL for Video Analyzer to connect to."]
    pub url: String,
    #[doc = "Base class for tunnel objects."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tunnel: Option<TunnelBase>,
}
impl EndpointBase {
    pub fn new(type_: String, credentials: CredentialsBase, url: String) -> Self {
        Self {
            type_,
            credentials,
            url,
            tunnel: None,
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
#[doc = "Group level network access control."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupLevelAccessControl {
    #[doc = "Whether or not public network access is allowed for specified resources under the Video Analyzer account."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<group_level_access_control::PublicNetworkAccess>,
}
impl GroupLevelAccessControl {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod group_level_access_control {
    use super::*;
    #[doc = "Whether or not public network access is allowed for specified resources under the Video Analyzer account."]
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
#[doc = "The IoT Hub details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHub {
    #[doc = "The IoT Hub resource identifier."]
    pub id: String,
    #[doc = "The user assigned managed identity to use when accessing a resource."]
    pub identity: ResourceIdentity,
    #[doc = "The current status of the Iot Hub mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl IotHub {
    pub fn new(id: String, identity: ResourceIdentity) -> Self {
        Self {
            id,
            identity,
            status: None,
        }
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
#[doc = "Live pipeline represents a unique instance of a live topology, used for real-time ingestion, archiving and publishing of content for a unique RTSP camera."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LivePipeline {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Live pipeline properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LivePipelineProperties>,
}
impl LivePipeline {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of LivePipeline items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LivePipelineCollection {
    #[doc = "A collection of LivePipeline items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LivePipeline>,
    #[doc = "A link to the next page of the collection (when the collection contains too many results to return in one response)."]
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LivePipelineCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl LivePipelineCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Used for tracking the status of an operation on the live pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LivePipelineOperationStatus {
    #[doc = "The name of the live pipeline operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of the live pipeline operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl LivePipelineOperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Live pipeline properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LivePipelineProperties {
    #[doc = "The reference to an existing pipeline topology defined for real-time content processing. When activated, this live pipeline will process content according to the pipeline topology definition."]
    #[serde(rename = "topologyName")]
    pub topology_name: String,
    #[doc = "An optional description for the pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Maximum bitrate capacity in Kbps reserved for the live pipeline. The allowed range is from 500 to 3000 Kbps in increments of 100 Kbps. If the RTSP camera exceeds this capacity, then the service will disconnect temporarily from the camera. It will retry to re-establish connection (with exponential backoff), checking to see if the camera bitrate is now below the reserved capacity. Doing so will ensure that one 'noisy neighbor' does not affect other live pipelines in your account."]
    #[serde(rename = "bitrateKbps")]
    pub bitrate_kbps: i32,
    #[doc = "Current state of the pipeline (read-only)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<live_pipeline_properties::State>,
    #[doc = "List of the instance level parameter values for the user-defined topology parameters. A pipeline can only define or override parameters values for parameters which have been declared in the referenced topology. Topology parameters without a default value must be defined. Topology parameters with a default value can be optionally be overridden."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterDefinition>,
}
impl LivePipelineProperties {
    pub fn new(topology_name: String, bitrate_kbps: i32) -> Self {
        Self {
            topology_name,
            description: None,
            bitrate_kbps,
            state: None,
            parameters: Vec::new(),
        }
    }
}
pub mod live_pipeline_properties {
    use super::*;
    #[doc = "Current state of the pipeline (read-only)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Inactive,
        Activating,
        Active,
        Deactivating,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inactive => serializer.serialize_unit_variant("State", 0u32, "Inactive"),
                Self::Activating => serializer.serialize_unit_variant("State", 1u32, "Activating"),
                Self::Active => serializer.serialize_unit_variant("State", 2u32, "Active"),
                Self::Deactivating => serializer.serialize_unit_variant("State", 3u32, "Deactivating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Live pipeline properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LivePipelinePropertiesUpdate {
    #[doc = "The reference to an existing pipeline topology defined for real-time content processing. When activated, this live pipeline will process content according to the pipeline topology definition."]
    #[serde(rename = "topologyName", default, skip_serializing_if = "Option::is_none")]
    pub topology_name: Option<String>,
    #[doc = "An optional description for the pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Maximum bitrate capacity in Kbps reserved for the live pipeline. The allowed range is from 500 to 3000 Kbps in increments of 100 Kbps. If the RTSP camera exceeds this capacity, then the service will disconnect temporarily from the camera. It will retry to re-establish connection (with exponential backoff), checking to see if the camera bitrate is now below the reserved capacity. Doing so will ensure that one 'noisy neighbor' does not affect other live pipelines in your account."]
    #[serde(rename = "bitrateKbps", default, skip_serializing_if = "Option::is_none")]
    pub bitrate_kbps: Option<i32>,
    #[doc = "Current state of the pipeline (read-only)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<live_pipeline_properties_update::State>,
    #[doc = "List of the instance level parameter values for the user-defined topology parameters. A pipeline can only define or override parameters values for parameters which have been declared in the referenced topology. Topology parameters without a default value must be defined. Topology parameters with a default value can be optionally be overridden."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterDefinition>,
}
impl LivePipelinePropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod live_pipeline_properties_update {
    use super::*;
    #[doc = "Current state of the pipeline (read-only)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Inactive,
        Activating,
        Active,
        Deactivating,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inactive => serializer.serialize_unit_variant("State", 0u32, "Inactive"),
                Self::Activating => serializer.serialize_unit_variant("State", 1u32, "Activating"),
                Self::Active => serializer.serialize_unit_variant("State", 2u32, "Active"),
                Self::Deactivating => serializer.serialize_unit_variant("State", 3u32, "Deactivating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Live pipeline represents a unique instance of a live topology, used for real-time ingestion, archiving and publishing of content for a unique RTSP camera."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LivePipelineUpdate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Live pipeline properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LivePipelinePropertiesUpdate>,
}
impl LivePipelineUpdate {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Network access control for video analyzer account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkAccessControl {
    #[doc = "Group level network access control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration: Option<GroupLevelAccessControl>,
    #[doc = "Group level network access control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingestion: Option<GroupLevelAccessControl>,
    #[doc = "Group level network access control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumption: Option<GroupLevelAccessControl>,
}
impl NetworkAccessControl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for nodes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeBase {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
    #[doc = "Node name. Must be unique within the topology."]
    pub name: String,
}
impl NodeBase {
    pub fn new(type_: String, name: String) -> Self {
        Self { type_, name }
    }
}
#[doc = "Describes an input signal to be used on a pipeline node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeInput {
    #[doc = "The name of the upstream node in the pipeline which output is used as input of the current node."]
    #[serde(rename = "nodeName")]
    pub node_name: String,
}
impl NodeInput {
    pub fn new(node_name: String) -> Self {
        Self { node_name }
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
#[doc = "Single topology parameter declaration. Declared parameters can and must be referenced throughout the topology and can optionally have default values to be used when they are not defined in the pipelines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterDeclaration {
    #[doc = "Name of the parameter."]
    pub name: String,
    #[doc = "Type of the parameter."]
    #[serde(rename = "type")]
    pub type_: parameter_declaration::Type,
    #[doc = "Description of the parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The default value for the parameter to be used if the pipeline does not specify a value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}
impl ParameterDeclaration {
    pub fn new(name: String, type_: parameter_declaration::Type) -> Self {
        Self {
            name,
            type_,
            description: None,
            default: None,
        }
    }
}
pub mod parameter_declaration {
    use super::*;
    #[doc = "Type of the parameter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        String,
        SecretString,
        Int,
        Double,
        Bool,
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
                Self::String => serializer.serialize_unit_variant("Type", 0u32, "String"),
                Self::SecretString => serializer.serialize_unit_variant("Type", 1u32, "SecretString"),
                Self::Int => serializer.serialize_unit_variant("Type", 2u32, "Int"),
                Self::Double => serializer.serialize_unit_variant("Type", 3u32, "Double"),
                Self::Bool => serializer.serialize_unit_variant("Type", 4u32, "Bool"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the parameter value of an specific pipeline topology parameter. See pipeline topology parameters for more information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterDefinition {
    #[doc = "Name of the parameter declared in the pipeline topology."]
    pub name: String,
    #[doc = "Parameter value to be applied on this specific pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ParameterDefinition {
    pub fn new(name: String) -> Self {
        Self { name, value: None }
    }
}
#[doc = "A list of PEM formatted certificates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PemCertificateList {
    #[serde(flatten)]
    pub certificate_source: CertificateSource,
    #[doc = "PEM formatted public certificates. One certificate per entry."]
    pub certificates: Vec<String>,
}
impl PemCertificateList {
    pub fn new(certificate_source: CertificateSource, certificates: Vec<String>) -> Self {
        Self {
            certificate_source,
            certificates,
        }
    }
}
#[doc = "Pipeline job represents a unique instance of a batch topology, used for offline processing of selected portions of archived content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineJob {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Pipeline job properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PipelineJobProperties>,
}
impl PipelineJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of PipelineJob items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineJobCollection {
    #[doc = "A collection of PipelineJob items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PipelineJob>,
    #[doc = "A link to the next page of the collection (when the collection contains too many results to return in one response)."]
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PipelineJobCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PipelineJobCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about the error for a failed pipeline job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineJobError {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl PipelineJobError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Used for tracking the status of an operation on the pipeline job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineJobOperationStatus {
    #[doc = "The name of the pipeline job operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of the pipeline job operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl PipelineJobOperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pipeline job properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineJobProperties {
    #[doc = "Reference to an existing pipeline topology. When activated, this pipeline job will process content according to the pipeline topology definition."]
    #[serde(rename = "topologyName")]
    pub topology_name: String,
    #[doc = "An optional description for the pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Current state of the pipeline (read-only)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<pipeline_job_properties::State>,
    #[doc = "The date-time by when this pipeline job will be automatically deleted from your account."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expiration: Option<time::OffsetDateTime>,
    #[doc = "Details about the error for a failed pipeline job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PipelineJobError>,
    #[doc = "List of the instance level parameter values for the user-defined topology parameters. A pipeline can only define or override parameters values for parameters which have been declared in the referenced topology. Topology parameters without a default value must be defined. Topology parameters with a default value can be optionally be overridden."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterDefinition>,
}
impl PipelineJobProperties {
    pub fn new(topology_name: String) -> Self {
        Self {
            topology_name,
            description: None,
            state: None,
            expiration: None,
            error: None,
            parameters: Vec::new(),
        }
    }
}
pub mod pipeline_job_properties {
    use super::*;
    #[doc = "Current state of the pipeline (read-only)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Processing,
        Canceled,
        Completed,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Processing => serializer.serialize_unit_variant("State", 0u32, "Processing"),
                Self::Canceled => serializer.serialize_unit_variant("State", 1u32, "Canceled"),
                Self::Completed => serializer.serialize_unit_variant("State", 2u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("State", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Pipeline job properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineJobPropertiesUpdate {
    #[doc = "Reference to an existing pipeline topology. When activated, this pipeline job will process content according to the pipeline topology definition."]
    #[serde(rename = "topologyName", default, skip_serializing_if = "Option::is_none")]
    pub topology_name: Option<String>,
    #[doc = "An optional description for the pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Current state of the pipeline (read-only)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<pipeline_job_properties_update::State>,
    #[doc = "The date-time by when this pipeline job will be automatically deleted from your account."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expiration: Option<time::OffsetDateTime>,
    #[doc = "Details about the error for a failed pipeline job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PipelineJobError>,
    #[doc = "List of the instance level parameter values for the user-defined topology parameters. A pipeline can only define or override parameters values for parameters which have been declared in the referenced topology. Topology parameters without a default value must be defined. Topology parameters with a default value can be optionally be overridden."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterDefinition>,
}
impl PipelineJobPropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod pipeline_job_properties_update {
    use super::*;
    #[doc = "Current state of the pipeline (read-only)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Processing,
        Canceled,
        Completed,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Processing => serializer.serialize_unit_variant("State", 0u32, "Processing"),
                Self::Canceled => serializer.serialize_unit_variant("State", 1u32, "Canceled"),
                Self::Completed => serializer.serialize_unit_variant("State", 2u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("State", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Pipeline job represents a unique instance of a batch topology, used for offline processing of selected portions of archived content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineJobUpdate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Pipeline job properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PipelineJobPropertiesUpdate>,
}
impl PipelineJobUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pipeline topology describes the processing steps to be applied when processing content for a particular outcome. The topology should be defined according to the scenario to be achieved and can be reused across many pipeline instances which share the same processing characteristics. For instance, a pipeline topology which captures content from a RTSP camera and archives the content can be reused across many different cameras, as long as the same processing is to be applied across all the cameras. Individual instance properties can be defined through the use of user-defined parameters, which allow for a topology to be parameterized. This allows  individual pipelines refer to different values, such as individual cameras' RTSP endpoints and credentials. Overall a topology is composed of the following:\r\n\r\n  - Parameters: list of user defined parameters that can be references across the topology nodes.\r\n  - Sources: list of one or more data sources nodes such as an RTSP source which allows for content to be ingested from cameras.\r\n  - Processors: list of nodes which perform data analysis or transformations.\r\n  - Sinks: list of one or more data sinks which allow for data to be stored or exported to other destinations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineTopology {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Describes the properties of a pipeline topology."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PipelineTopologyProperties>,
    #[doc = "Topology kind."]
    pub kind: pipeline_topology::Kind,
    #[doc = "The SKU details."]
    pub sku: Sku,
}
impl PipelineTopology {
    pub fn new(kind: pipeline_topology::Kind, sku: Sku) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties: None,
            kind,
            sku,
        }
    }
}
pub mod pipeline_topology {
    use super::*;
    #[doc = "Topology kind."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Live,
        Batch,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Live => serializer.serialize_unit_variant("Kind", 0u32, "Live"),
                Self::Batch => serializer.serialize_unit_variant("Kind", 1u32, "Batch"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A collection of PipelineTopology items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineTopologyCollection {
    #[doc = "A collection of PipelineTopology items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PipelineTopology>,
    #[doc = "A link to the next page of the collection (when the collection contains too many results to return in one response)."]
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PipelineTopologyCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PipelineTopologyCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a pipeline topology."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineTopologyProperties {
    #[doc = "An optional description of the pipeline topology. It is recommended that the expected use of the topology to be described here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "List of the topology parameter declarations. Parameters declared here can be referenced throughout the topology nodes through the use of \"${PARAMETER_NAME}\" string pattern. Parameters can have optional default values and can later be defined in individual instances of the pipeline."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterDeclaration>,
    #[doc = "List of the topology source nodes. Source nodes enable external data to be ingested by the pipeline."]
    pub sources: Vec<SourceNodeBase>,
    #[doc = "List of the topology processor nodes. Processor nodes enable pipeline data to be analyzed, processed or transformed."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processors: Vec<ProcessorNodeBase>,
    #[doc = "List of the topology sink nodes. Sink nodes allow pipeline data to be stored or exported."]
    pub sinks: Vec<SinkNodeBase>,
}
impl PipelineTopologyProperties {
    pub fn new(sources: Vec<SourceNodeBase>, sinks: Vec<SinkNodeBase>) -> Self {
        Self {
            description: None,
            parameters: Vec::new(),
            sources,
            processors: Vec::new(),
            sinks,
        }
    }
}
#[doc = "Describes the properties of a pipeline topology."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineTopologyPropertiesUpdate {
    #[doc = "An optional description of the pipeline topology. It is recommended that the expected use of the topology to be described here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "List of the topology parameter declarations. Parameters declared here can be referenced throughout the topology nodes through the use of \"${PARAMETER_NAME}\" string pattern. Parameters can have optional default values and can later be defined in individual instances of the pipeline."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterDeclaration>,
    #[doc = "List of the topology source nodes. Source nodes enable external data to be ingested by the pipeline."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<SourceNodeBase>,
    #[doc = "List of the topology processor nodes. Processor nodes enable pipeline data to be analyzed, processed or transformed."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processors: Vec<ProcessorNodeBase>,
    #[doc = "List of the topology sink nodes. Sink nodes allow pipeline data to be stored or exported."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sinks: Vec<SinkNodeBase>,
}
impl PipelineTopologyPropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pipeline topology describes the processing steps to be applied when processing content for a particular outcome. The topology should be defined according to the scenario to be achieved and can be reused across many pipeline instances which share the same processing characteristics. For instance, a pipeline topology which captures content from a RTSP camera and archives the content can be reused across many different cameras, as long as the same processing is to be applied across all the cameras. Individual instance properties can be defined through the use of user-defined parameters, which allow for a topology to be parameterized. This allows  individual pipelines refer to different values, such as individual cameras' RTSP endpoints and credentials. Overall a topology is composed of the following:\r\n\r\n  - Parameters: list of user defined parameters that can be references across the topology nodes.\r\n  - Sources: list of one or more data sources nodes such as an RTSP source which allows for content to be ingested from cameras.\r\n  - Processors: list of nodes which perform data analysis or transformations.\r\n  - Sinks: list of one or more data sinks which allow for data to be stored or exported to other destinations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineTopologyUpdate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Describes the properties of a pipeline topology."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PipelineTopologyPropertiesUpdate>,
    #[doc = "Topology kind."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<pipeline_topology_update::Kind>,
    #[doc = "The SKU details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl PipelineTopologyUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod pipeline_topology_update {
    use super::*;
    #[doc = "Topology kind."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Live,
        Batch,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Live => serializer.serialize_unit_variant("Kind", 0u32, "Live"),
                Self::Batch => serializer.serialize_unit_variant("Kind", 1u32, "Batch"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Private Endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for Private Endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint Connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the PrivateEndpointConnectProperties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of private endpoint connection associated with the specified storage account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "Array of private endpoint connections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the PrivateEndpointConnectProperties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The Private Endpoint resource."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "A collection of information about the state of the connection between service consumer and provider."]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            private_endpoint: None,
            private_link_service_connection_state,
            provisioning_state: None,
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointConnectionProvisioningState")]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
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
            Self::Deleting => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 2u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 3u32, "Failed"),
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
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The private link resource Private link DNS zone name."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of information about the state of the connection between service consumer and provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[doc = "The reason for approval/rejection of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for topology processor nodes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessorNodeBase {
    #[serde(flatten)]
    pub node_base: NodeBase,
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
    #[doc = "An array of upstream node references within the topology to be used as inputs for this node."]
    pub inputs: Vec<NodeInput>,
}
impl ProcessorNodeBase {
    pub fn new(node_base: NodeBase, type_: String, inputs: Vec<NodeInput>) -> Self {
        Self { node_base, type_, inputs }
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
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
#[doc = "RTSP source allows for media from an RTSP camera or generic RTSP server to be ingested into a pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RtspSource {
    #[serde(flatten)]
    pub source_node_base: SourceNodeBase,
    #[doc = "Network transport utilized by the RTSP and RTP exchange: TCP or HTTP. When using TCP, the RTP packets are interleaved on the TCP RTSP connection. When using HTTP, the RTSP messages are exchanged through long lived HTTP connections, and the RTP packages are interleaved in the HTTP connections alongside the RTSP messages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<rtsp_source::Transport>,
    #[doc = "Base class for endpoints."]
    pub endpoint: EndpointBase,
}
impl RtspSource {
    pub fn new(source_node_base: SourceNodeBase, endpoint: EndpointBase) -> Self {
        Self {
            source_node_base,
            transport: None,
            endpoint,
        }
    }
}
pub mod rtsp_source {
    use super::*;
    #[doc = "Network transport utilized by the RTSP and RTP exchange: TCP or HTTP. When using TCP, the RTP packets are interleaved on the TCP RTSP connection. When using HTTP, the RTSP messages are exchanged through long lived HTTP connections, and the RTP packages are interleaved in the HTTP connections alongside the RTSP messages."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Transport")]
    pub enum Transport {
        Http,
        Tcp,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Transport {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Transport {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Transport {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Http => serializer.serialize_unit_variant("Transport", 0u32, "Http"),
                Self::Tcp => serializer.serialize_unit_variant("Transport", 1u32, "Tcp"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A remote tunnel securely established using IoT Hub device information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecureIotDeviceRemoteTunnel {
    #[serde(flatten)]
    pub tunnel_base: TunnelBase,
    #[doc = "Name of the IoT Hub."]
    #[serde(rename = "iotHubName")]
    pub iot_hub_name: String,
    #[doc = "The IoT device id to use when establishing the remote tunnel. This string is case-sensitive."]
    #[serde(rename = "deviceId")]
    pub device_id: String,
}
impl SecureIotDeviceRemoteTunnel {
    pub fn new(tunnel_base: TunnelBase, iot_hub_name: String, device_id: String) -> Self {
        Self {
            tunnel_base,
            iot_hub_name,
            device_id,
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
#[doc = "Base class for topology sink nodes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SinkNodeBase {
    #[serde(flatten)]
    pub node_base: NodeBase,
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
    #[doc = "An array of upstream node references within the topology to be used as inputs for this node."]
    pub inputs: Vec<NodeInput>,
}
impl SinkNodeBase {
    pub fn new(node_base: NodeBase, type_: String, inputs: Vec<NodeInput>) -> Self {
        Self { node_base, type_, inputs }
    }
}
#[doc = "The SKU details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The SKU name."]
    pub name: sku::Name,
    #[doc = "The SKU tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self { name, tier: None }
    }
}
pub mod sku {
    use super::*;
    #[doc = "The SKU name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        #[serde(rename = "Live_S1")]
        LiveS1,
        #[serde(rename = "Batch_S1")]
        BatchS1,
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
                Self::LiveS1 => serializer.serialize_unit_variant("Name", 0u32, "Live_S1"),
                Self::BatchS1 => serializer.serialize_unit_variant("Name", 1u32, "Batch_S1"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The SKU tier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Standard,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("Tier", 0u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Base class for topology source nodes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceNodeBase {
    #[serde(flatten)]
    pub node_base: NodeBase,
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
}
impl SourceNodeBase {
    pub fn new(node_base: NodeBase, type_: String) -> Self {
        Self { node_base, type_ }
    }
}
#[doc = "The details about the associated storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccount {
    #[doc = "The ID of the storage account resource. Video Analyzer relies on tables, queues, and blobs. The primary storage account must be a Standard Storage account (either Microsoft.ClassicStorage or Microsoft.Storage)."]
    pub id: String,
    #[doc = "The user assigned managed identity to use when accessing a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[doc = "The current status of the storage account mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl StorageAccount {
    pub fn new(id: String) -> Self {
        Self {
            id,
            identity: None,
            status: None,
        }
    }
}
#[doc = "A sequence of datetime ranges as a string."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSequenceBase {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
}
impl TimeSequenceBase {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "TLS endpoint describes an endpoint that the pipeline can connect to over TLS transport (data is encrypted in transit)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TlsEndpoint {
    #[serde(flatten)]
    pub endpoint_base: EndpointBase,
    #[doc = "Base class for certificate sources."]
    #[serde(rename = "trustedCertificates", default, skip_serializing_if = "Option::is_none")]
    pub trusted_certificates: Option<CertificateSource>,
    #[doc = "Options for controlling the validation of TLS endpoints."]
    #[serde(rename = "validationOptions", default, skip_serializing_if = "Option::is_none")]
    pub validation_options: Option<TlsValidationOptions>,
}
impl TlsEndpoint {
    pub fn new(endpoint_base: EndpointBase) -> Self {
        Self {
            endpoint_base,
            trusted_certificates: None,
            validation_options: None,
        }
    }
}
#[doc = "Options for controlling the validation of TLS endpoints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TlsValidationOptions {
    #[doc = "When set to 'true' causes the certificate subject name validation to be skipped. Default is 'false'."]
    #[serde(rename = "ignoreHostname", default, skip_serializing_if = "Option::is_none")]
    pub ignore_hostname: Option<String>,
    #[doc = "When set to 'true' causes the certificate chain trust validation to be skipped. Default is 'false'."]
    #[serde(rename = "ignoreSignature", default, skip_serializing_if = "Option::is_none")]
    pub ignore_signature: Option<String>,
}
impl TlsValidationOptions {
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
#[doc = "Base class for tunnel objects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TunnelBase {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
}
impl TunnelBase {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "Unsecured endpoint describes an endpoint that the pipeline can connect to over clear transport (no encryption in transit)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnsecuredEndpoint {
    #[serde(flatten)]
    pub endpoint_base: EndpointBase,
}
impl UnsecuredEndpoint {
    pub fn new(endpoint_base: EndpointBase) -> Self {
        Self { endpoint_base }
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
#[doc = "Username and password credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsernamePasswordCredentials {
    #[serde(flatten)]
    pub credentials_base: CredentialsBase,
    #[doc = "Username to be presented as part of the credentials."]
    pub username: String,
    #[doc = "Password to be presented as part of the credentials. It is recommended that this value is parameterized as a secret string in order to prevent this value to be returned as part of the resource on API requests."]
    pub password: String,
}
impl UsernamePasswordCredentials {
    pub fn new(credentials_base: CredentialsBase, username: String, password: String) -> Self {
        Self {
            credentials_base,
            username,
            password,
        }
    }
}
#[doc = "The Video Analyzer account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoAnalyzer {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of the Video Analyzer account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VideoAnalyzerProperties>,
    #[doc = "The managed identity for the Video Analyzer resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<VideoAnalyzerIdentity>,
}
impl VideoAnalyzer {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
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
#[doc = "Status of video analyzer operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoAnalyzerOperationStatus {
    #[doc = "Operation identifier."]
    pub name: String,
    #[doc = "Operation resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Operation start time."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Operation end time."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "Operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl VideoAnalyzerOperationStatus {
    pub fn new(name: String) -> Self {
        Self {
            name,
            id: None,
            start_time: None,
            end_time: None,
            status: None,
            error: None,
        }
    }
}
#[doc = "Status of private endpoint connection operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoAnalyzerPrivateEndpointConnectionOperationStatus {
    #[doc = "Operation identifier."]
    pub name: String,
    #[doc = "Operation resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Operation start time."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Operation end time."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "Operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl VideoAnalyzerPrivateEndpointConnectionOperationStatus {
    pub fn new(name: String) -> Self {
        Self {
            name,
            id: None,
            start_time: None,
            end_time: None,
            status: None,
            error: None,
        }
    }
}
#[doc = "The properties of the Video Analyzer account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoAnalyzerProperties {
    #[doc = "The storage accounts for this resource."]
    #[serde(rename = "storageAccounts")]
    pub storage_accounts: Vec<StorageAccount>,
    #[doc = "The endpoints associated with this resource."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<Endpoint>,
    #[doc = "Defines how the Video Analyzer account is (optionally) encrypted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<AccountEncryption>,
    #[doc = "The IoT Hubs for this resource."]
    #[serde(rename = "iotHubs", default, skip_serializing_if = "Vec::is_empty")]
    pub iot_hubs: Vec<IotHub>,
    #[doc = "Whether or not public network access is allowed for resources under the Video Analyzer account."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<video_analyzer_properties::PublicNetworkAccess>,
    #[doc = "Network access control for video analyzer account."]
    #[serde(rename = "networkAccessControl", default, skip_serializing_if = "Option::is_none")]
    pub network_access_control: Option<NetworkAccessControl>,
    #[doc = "Provisioning state of the Video Analyzer account."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<video_analyzer_properties::ProvisioningState>,
    #[doc = "Private Endpoint Connections created under Video Analyzer account."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
}
impl VideoAnalyzerProperties {
    pub fn new(storage_accounts: Vec<StorageAccount>) -> Self {
        Self {
            storage_accounts,
            endpoints: Vec::new(),
            encryption: None,
            iot_hubs: Vec::new(),
            public_network_access: None,
            network_access_control: None,
            provisioning_state: None,
            private_endpoint_connections: Vec::new(),
        }
    }
}
pub mod video_analyzer_properties {
    use super::*;
    #[doc = "Whether or not public network access is allowed for resources under the Video Analyzer account."]
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
    #[doc = "Provisioning state of the Video Analyzer account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Failed,
        InProgress,
        Succeeded,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Failed"),
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 1u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of the Video Analyzer account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoAnalyzerPropertiesUpdate {
    #[doc = "The storage accounts for this resource."]
    #[serde(rename = "storageAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub storage_accounts: Vec<StorageAccount>,
    #[doc = "The endpoints associated with this resource."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<Endpoint>,
    #[doc = "Defines how the Video Analyzer account is (optionally) encrypted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<AccountEncryption>,
    #[doc = "The IoT Hubs for this resource."]
    #[serde(rename = "iotHubs", default, skip_serializing_if = "Vec::is_empty")]
    pub iot_hubs: Vec<IotHub>,
    #[doc = "Whether or not public network access is allowed for resources under the Video Analyzer account."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<video_analyzer_properties_update::PublicNetworkAccess>,
    #[doc = "Network access control for video analyzer account."]
    #[serde(rename = "networkAccessControl", default, skip_serializing_if = "Option::is_none")]
    pub network_access_control: Option<NetworkAccessControl>,
    #[doc = "Provisioning state of the Video Analyzer account."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<video_analyzer_properties_update::ProvisioningState>,
    #[doc = "Private Endpoint Connections created under Video Analyzer account."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
}
impl VideoAnalyzerPropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod video_analyzer_properties_update {
    use super::*;
    #[doc = "Whether or not public network access is allowed for resources under the Video Analyzer account."]
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
    #[doc = "Provisioning state of the Video Analyzer account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Failed,
        InProgress,
        Succeeded,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Failed"),
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 1u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The update operation for a Video Analyzer account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoAnalyzerUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The properties of the Video Analyzer account."]
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
#[doc = "Video archival properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoArchival {
    #[doc = "Video retention period indicates the maximum age of the video archive segments which are intended to be kept in storage. It must be provided in the ISO8601 duration format in the granularity of days, up to a maximum of 10 years. For example, if this is set to P30D (30 days), content older than 30 days will be periodically deleted. This value can be updated at any time and the new desired retention period will be effective within 24 hours."]
    #[serde(rename = "retentionPeriod", default, skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<String>,
}
impl VideoArchival {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "\"Video content token grants access to the video content URLs.\""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoContentToken {
    #[doc = "The content token expiration date in ISO8601 format (eg. 2021-01-01T00:00:00Z)."]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "The content token value to be added to the video content URL as the value for the \"token\" query string parameter. The token is specific to a single video."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl VideoContentToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Set of URLs to the video content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoContentUrls {
    #[doc = "Video file download URL. This URL can be used in conjunction with the video content authorization token to download the video MP4 file. The resulting MP4 file can be played on any standard media player. It is available when the video type is 'file' and video file is available for consumption."]
    #[serde(rename = "downloadUrl", default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[doc = "Video archive streaming base URL. The archived content can be automatically played by the Azure Video Analyzer player widget. Alternatively, this URL can be used in conjunction with the video content authorization token on any compatible DASH or HLS players by appending the following to the base URL:\r\n\r\n    - HLSv4:     /manifest(format=m3u8-aapl).m3u8\r\n    - HLS CMAF:  /manifest(format=m3u8-cmaf)\r\n    - DASH CMAF: /manifest(format=mpd-time-cmaf)\r\n\r\n    Moreover, an ongoing video recording can be played in \"live mode\" with latencies which are approximately double of the chosen video segment length. It is available when the video type is 'archive' and video archiving is enabled."]
    #[serde(rename = "archiveBaseUrl", default, skip_serializing_if = "Option::is_none")]
    pub archive_base_url: Option<String>,
    #[doc = "Video low-latency streaming URL. The live content can be automatically played by the Azure Video Analyzer player widget. Alternatively, this URL can be used in conjunction with the video content authorization token to expose a WebSocket tunneled RTSP stream. It is available when the video type is 'archive' and a live, low-latency feed is available from the source."]
    #[serde(rename = "rtspTunnelUrl", default, skip_serializing_if = "Option::is_none")]
    pub rtsp_tunnel_url: Option<String>,
    #[doc = "Video preview image URLs. These URLs can be used in conjunction with the video content authorization token to download the most recent still image from the video archive in different resolutions. They are available when the video type is 'archive' and preview images are enabled."]
    #[serde(rename = "previewImageUrls", default, skip_serializing_if = "Option::is_none")]
    pub preview_image_urls: Option<VideoPreviewImageUrls>,
}
impl VideoContentUrls {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Optional properties to be used in case a new video resource needs to be created on the service. These will not take effect if the video already exists."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoCreationProperties {
    #[doc = "Optional title provided by the user. Value can be up to 256 characters long."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Optional description provided by the user. Value can be up to 2048 characters long."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Segment length indicates the length of individual content files (segments) which are persisted to storage. Smaller segments provide lower archive playback latency but generate larger volume of storage transactions. Larger segments reduce the amount of storage transactions while increasing the archive playback latency. Value must be specified in ISO8601 duration format (i.e. \"PT30S\" equals 30 seconds) and can vary between 30 seconds to 5 minutes, in 30 seconds increments. Changing this value after the initial call to create the video resource can lead to errors when uploading content to the archive. Default value is 30 seconds. This property is only allowed for topologies where \"kind\" is set to \"live\"."]
    #[serde(rename = "segmentLength", default, skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<String>,
    #[doc = "Video retention period indicates how long the video is kept in storage. Value must be specified in ISO8601 duration format (i.e. \"P1D\" equals 1 day) and can vary between 1 day to 10 years, in 1 day increments. When absent (null), all video content is retained indefinitely. This property is only allowed for topologies where \"kind\" is set to \"live\"."]
    #[serde(rename = "retentionPeriod", default, skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<String>,
}
impl VideoCreationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base type for all video encoding presets, which define the recipe or instructions on how the input video should be processed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoEncoderBase {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@type")]
    pub type_: String,
    #[doc = "The maximum bitrate, in kilobits per second or Kbps, at which video should be encoded. If omitted, encoder sets it automatically to try and match the quality of the input video."]
    #[serde(rename = "bitrateKbps", default, skip_serializing_if = "Option::is_none")]
    pub bitrate_kbps: Option<String>,
    #[doc = "The frame rate (in frames per second) of the encoded video. The value must be greater than zero, and less than or equal to 300. If omitted, the encoder uses the average frame rate of the input video."]
    #[serde(rename = "frameRate", default, skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<String>,
    #[doc = "The video scaling information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<VideoScale>,
}
impl VideoEncoderBase {
    pub fn new(type_: String) -> Self {
        Self {
            type_,
            bitrate_kbps: None,
            frame_rate: None,
            scale: None,
        }
    }
}
#[doc = "A custom preset for encoding video with the H.264 (AVC) codec."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoEncoderH264 {
    #[serde(flatten)]
    pub video_encoder_base: VideoEncoderBase,
}
impl VideoEncoderH264 {
    pub fn new(video_encoder_base: VideoEncoderBase) -> Self {
        Self { video_encoder_base }
    }
}
#[doc = "Represents a video resource within Azure Video Analyzer. Videos can be ingested from RTSP cameras through live pipelines or can be created by exporting sequences from existing captured video through a pipeline job. Videos ingested through live pipelines can be streamed through Azure Video Analyzer Player Widget or compatible players. Exported videos can be downloaded as MP4 files."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoEntity {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Application level properties for the video resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VideoProperties>,
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
    #[doc = "Value indicating whether or not the video is currently being referenced be an active pipeline. The fact that is being referenced, doesn't necessarily indicate that data is being received. For example, video recording may be gated on events or camera may not be accessible at the time."]
    #[serde(rename = "isInUse")]
    pub is_in_use: bool,
}
impl VideoFlags {
    pub fn new(can_stream: bool, has_data: bool, is_in_use: bool) -> Self {
        Self {
            can_stream,
            has_data,
            is_in_use,
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
#[doc = "Video preview image URLs. These URLs can be used in conjunction with the video content authorization token to download the most recent still image from the video archive in different resolutions. They are available when the video type is 'archive' and preview images are enabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoPreviewImageUrls {
    #[doc = "Low resolution preview image URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub small: Option<String>,
    #[doc = "Medium resolution preview image URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[doc = "High resolution preview image URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large: Option<String>,
}
impl VideoPreviewImageUrls {
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
    #[doc = "Video content type. Different content types are suitable for different applications and scenarios."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<video_properties::Type>,
    #[doc = "Video flags contain information about the available video actions and its dynamic properties based on the current video state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<VideoFlags>,
    #[doc = "Set of URLs to the video content."]
    #[serde(rename = "contentUrls", default, skip_serializing_if = "Option::is_none")]
    pub content_urls: Option<VideoContentUrls>,
    #[doc = "Contains information about the video and audio content."]
    #[serde(rename = "mediaInfo", default, skip_serializing_if = "Option::is_none")]
    pub media_info: Option<VideoMediaInfo>,
    #[doc = "Video archival properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archival: Option<VideoArchival>,
}
impl VideoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod video_properties {
    use super::*;
    #[doc = "Video content type. Different content types are suitable for different applications and scenarios."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Archive,
        File,
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
                Self::File => serializer.serialize_unit_variant("Type", 1u32, "File"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Optional flags used to change how video is published. These are only allowed for topologies where \"kind\" is set to \"live\"."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoPublishingOptions {
    #[doc = "When set to 'true' content will not be archived or recorded. This is used, for example, when the topology is used only for low latency video streaming. Default is 'false'.  If set to 'true', then \"disableRtspPublishing\" must be set to 'false'."]
    #[serde(rename = "disableArchive", default, skip_serializing_if = "Option::is_none")]
    pub disable_archive: Option<String>,
    #[doc = "When set to 'true' the RTSP playback URL will not be published, disabling low latency streaming. This is used, for example, when the topology is used only for archiving content. Default is 'false'.  If set to 'true', then \"disableArchive\" must be set to 'false'."]
    #[serde(rename = "disableRtspPublishing", default, skip_serializing_if = "Option::is_none")]
    pub disable_rtsp_publishing: Option<String>,
}
impl VideoPublishingOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The video scaling information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoScale {
    #[doc = "The desired output video height."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[doc = "The desired output video width."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[doc = "Describes the video scaling mode to be applied. Default mode is 'Pad'. If the mode is 'Pad' or 'Stretch' then both width and height must be specified. Else if the mode is 'PreserveAspectRatio' then only one of width or height need be provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<video_scale::Mode>,
}
impl VideoScale {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod video_scale {
    use super::*;
    #[doc = "Describes the video scaling mode to be applied. Default mode is 'Pad'. If the mode is 'Pad' or 'Stretch' then both width and height must be specified. Else if the mode is 'PreserveAspectRatio' then only one of width or height need be provided."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        Pad,
        PreserveAspectRatio,
        Stretch,
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
                Self::Pad => serializer.serialize_unit_variant("Mode", 0u32, "Pad"),
                Self::PreserveAspectRatio => serializer.serialize_unit_variant("Mode", 1u32, "PreserveAspectRatio"),
                Self::Stretch => serializer.serialize_unit_variant("Mode", 2u32, "Stretch"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A sequence of absolute datetime ranges as a string. The datetime values should follow IS08601, and the sum of the ranges should add up to 24 hours or less. Currently, there can be only one range specified in the sequence."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoSequenceAbsoluteTimeMarkers {
    #[serde(flatten)]
    pub time_sequence_base: TimeSequenceBase,
    #[doc = "The sequence of datetime ranges. Example: '[[\"2021-10-05T03:30:00Z\", \"2021-10-05T03:40:00Z\"]]'."]
    pub ranges: String,
}
impl VideoSequenceAbsoluteTimeMarkers {
    pub fn new(time_sequence_base: TimeSequenceBase, ranges: String) -> Self {
        Self {
            time_sequence_base,
            ranges,
        }
    }
}
#[doc = "Video sink in a live topology allows for video and audio to be captured, optionally archived, and published via a video resource. If archiving is enabled, this results in a video of type 'archive'. If used in a batch topology, this allows for video and audio to be stored as a file, and published via a video resource of type 'file'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoSink {
    #[serde(flatten)]
    pub sink_node_base: SinkNodeBase,
    #[doc = "Name of a new or existing video resource used to capture and publish content. Note: if downstream of RTSP source, and if disableArchive is set to true, then no content is archived."]
    #[serde(rename = "videoName")]
    pub video_name: String,
    #[doc = "Optional properties to be used in case a new video resource needs to be created on the service. These will not take effect if the video already exists."]
    #[serde(rename = "videoCreationProperties", default, skip_serializing_if = "Option::is_none")]
    pub video_creation_properties: Option<VideoCreationProperties>,
    #[doc = "Optional flags used to change how video is published. These are only allowed for topologies where \"kind\" is set to \"live\"."]
    #[serde(rename = "videoPublishingOptions", default, skip_serializing_if = "Option::is_none")]
    pub video_publishing_options: Option<VideoPublishingOptions>,
}
impl VideoSink {
    pub fn new(sink_node_base: SinkNodeBase, video_name: String) -> Self {
        Self {
            sink_node_base,
            video_name,
            video_creation_properties: None,
            video_publishing_options: None,
        }
    }
}
#[doc = "Video source allows for content from a Video Analyzer video resource to be ingested into a pipeline. Currently supported only with batch pipelines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoSource {
    #[serde(flatten)]
    pub source_node_base: SourceNodeBase,
    #[doc = "Name of the Video Analyzer video resource to be used as the source."]
    #[serde(rename = "videoName")]
    pub video_name: String,
    #[doc = "A sequence of datetime ranges as a string."]
    #[serde(rename = "timeSequences")]
    pub time_sequences: TimeSequenceBase,
}
impl VideoSource {
    pub fn new(source_node_base: SourceNodeBase, video_name: String, time_sequences: TimeSequenceBase) -> Self {
        Self {
            source_node_base,
            video_name,
            time_sequences,
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

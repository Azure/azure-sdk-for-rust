#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Describes Advanced Audio Codec (AAC) audio encoding settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AacAudio {
    #[serde(flatten)]
    pub audio: Audio,
    #[doc = "The encoding profile to be used when encoding audio with AAC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<aac_audio::Profile>,
}
impl AacAudio {
    pub fn new(audio: Audio) -> Self {
        Self { audio, profile: None }
    }
}
pub mod aac_audio {
    use super::*;
    #[doc = "The encoding profile to be used when encoding audio with AAC."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Profile")]
    pub enum Profile {
        AacLc,
        HeAacV1,
        HeAacV2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Profile {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Profile {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Profile {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AacLc => serializer.serialize_unit_variant("Profile", 0u32, "AacLc"),
                Self::HeAacV1 => serializer.serialize_unit_variant("Profile", 1u32, "HeAacV1"),
                Self::HeAacV2 => serializer.serialize_unit_variant("Profile", 2u32, "HeAacV2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies the clip time as an absolute time position in the media file.  The absolute time can point to a different position depending on whether the media file starts from a timestamp of zero or not."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AbsoluteClipTime {
    #[serde(flatten)]
    pub clip_time: ClipTime,
    #[doc = "The time position on the timeline of the input media. It is usually specified as an ISO8601 period. e.g PT30S for 30 seconds."]
    pub time: String,
}
impl AbsoluteClipTime {
    pub fn new(clip_time: ClipTime, time: String) -> Self {
        Self { clip_time, time }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessControl {
    #[doc = "The behavior for IP access control in Key Delivery."]
    #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
    pub default_action: Option<access_control::DefaultAction>,
    #[doc = "The IP allow list for access control in Key Delivery. If the default action is set to 'Allow', the IP allow list must be empty."]
    #[serde(rename = "ipAllowList", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_allow_list: Vec<String>,
}
impl AccessControl {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_control {
    use super::*;
    #[doc = "The behavior for IP access control in Key Delivery."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountEncryption {
    #[doc = "The type of key used to encrypt the Account Key."]
    #[serde(rename = "type")]
    pub type_: account_encryption::Type,
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
}
impl AccountEncryption {
    pub fn new(type_: account_encryption::Type) -> Self {
        Self {
            type_,
            key_vault_properties: None,
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
#[doc = "An Account Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountFilter {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The Media Filter properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MediaFilterProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl AccountFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of AccountFilter items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountFilterCollection {
    #[doc = "A collection of AccountFilter items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AccountFilter>,
    #[doc = "A link to the next page of the collection (when the collection contains too many results to return in one response)."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for AccountFilterCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl AccountFilterCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Akamai access control"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AkamaiAccessControl {
    #[doc = "authentication key list"]
    #[serde(
        rename = "akamaiSignatureHeaderAuthenticationKeyList",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub akamai_signature_header_authentication_key_list: Vec<AkamaiSignatureHeaderAuthenticationKey>,
}
impl AkamaiAccessControl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Akamai Signature Header authentication key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AkamaiSignatureHeaderAuthenticationKey {
    #[doc = "identifier of the key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[doc = "authentication key"]
    #[serde(rename = "base64Key", default, skip_serializing_if = "Option::is_none")]
    pub base64_key: Option<String>,
    #[doc = "The expiration time of the authentication key."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub expiration: Option<time::OffsetDateTime>,
}
impl AkamaiSignatureHeaderAuthenticationKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The API error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiError {
    #[doc = "Information about an error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ODataError>,
}
impl azure_core::Continuable for ApiError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ApiError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Asset {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The Asset properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssetProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Asset {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of Asset items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetCollection {
    #[doc = "A collection of Asset items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Asset>,
    #[doc = "A link to the next page of the collection (when the collection contains too many results to return in one response)."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for AssetCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl AssetCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Asset Storage container SAS URLs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetContainerSas {
    #[doc = "The list of Asset container SAS URLs."]
    #[serde(rename = "assetContainerSasUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub asset_container_sas_urls: Vec<String>,
}
impl AssetContainerSas {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Asset File Storage encryption metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetFileEncryptionMetadata {
    #[doc = "The Asset File initialization vector."]
    #[serde(rename = "initializationVector", default, skip_serializing_if = "Option::is_none")]
    pub initialization_vector: Option<String>,
    #[doc = "The Asset File name."]
    #[serde(rename = "assetFileName", default, skip_serializing_if = "Option::is_none")]
    pub asset_file_name: Option<String>,
    #[doc = "The Asset File Id."]
    #[serde(rename = "assetFileId")]
    pub asset_file_id: String,
}
impl AssetFileEncryptionMetadata {
    pub fn new(asset_file_id: String) -> Self {
        Self {
            initialization_vector: None,
            asset_file_name: None,
            asset_file_id,
        }
    }
}
#[doc = "An Asset Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetFilter {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The Media Filter properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MediaFilterProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl AssetFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of AssetFilter items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetFilterCollection {
    #[doc = "A collection of AssetFilter items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AssetFilter>,
    #[doc = "A link to the next page of the collection (when the collection contains too many results to return in one response)."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for AssetFilterCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl AssetFilterCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Asset properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetProperties {
    #[doc = "The Asset ID."]
    #[serde(rename = "assetId", default, skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[doc = "The creation date of the Asset."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The last modified date of the Asset."]
    #[serde(rename = "lastModified", with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
    #[doc = "The alternate ID of the Asset."]
    #[serde(rename = "alternateId", default, skip_serializing_if = "Option::is_none")]
    pub alternate_id: Option<String>,
    #[doc = "The Asset description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The name of the asset blob container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[doc = "The name of the storage account."]
    #[serde(rename = "storageAccountName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_name: Option<String>,
    #[doc = "The Asset encryption format. One of None or MediaStorageEncryption."]
    #[serde(rename = "storageEncryptionFormat", default, skip_serializing_if = "Option::is_none")]
    pub storage_encryption_format: Option<asset_properties::StorageEncryptionFormat>,
}
impl AssetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod asset_properties {
    use super::*;
    #[doc = "The Asset encryption format. One of None or MediaStorageEncryption."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageEncryptionFormat")]
    pub enum StorageEncryptionFormat {
        None,
        MediaStorageClientEncryption,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageEncryptionFormat {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageEncryptionFormat {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageEncryptionFormat {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("StorageEncryptionFormat", 0u32, "None"),
                Self::MediaStorageClientEncryption => {
                    serializer.serialize_unit_variant("StorageEncryptionFormat", 1u32, "MediaStorageClientEncryption")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the Streaming Locator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetStreamingLocator {
    #[doc = "Streaming Locator name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Asset Name."]
    #[serde(rename = "assetName", default, skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    #[doc = "The creation time of the Streaming Locator."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The start time of the Streaming Locator."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the Streaming Locator."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "StreamingLocatorId of the Streaming Locator."]
    #[serde(rename = "streamingLocatorId", default, skip_serializing_if = "Option::is_none")]
    pub streaming_locator_id: Option<String>,
    #[doc = "Name of the Streaming Policy used by this Streaming Locator."]
    #[serde(rename = "streamingPolicyName", default, skip_serializing_if = "Option::is_none")]
    pub streaming_policy_name: Option<String>,
    #[doc = "Name of the default ContentKeyPolicy used by this Streaming Locator."]
    #[serde(rename = "defaultContentKeyPolicyName", default, skip_serializing_if = "Option::is_none")]
    pub default_content_key_policy_name: Option<String>,
}
impl AssetStreamingLocator {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the common properties for all audio codecs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Audio {
    #[serde(flatten)]
    pub codec: Codec,
    #[doc = "The number of channels in the audio."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<i32>,
    #[doc = "The sampling rate to use for encoding in hertz."]
    #[serde(rename = "samplingRate", default, skip_serializing_if = "Option::is_none")]
    pub sampling_rate: Option<i32>,
    #[doc = "The bitrate, in bits per second, of the output encoded audio."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
}
impl Audio {
    pub fn new(codec: Codec) -> Self {
        Self {
            codec,
            channels: None,
            sampling_rate: None,
            bitrate: None,
        }
    }
}
#[doc = "The Audio Analyzer preset applies a pre-defined set of AI-based analysis operations, including speech transcription. Currently, the preset supports processing of content with a single audio track."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioAnalyzerPreset {
    #[serde(flatten)]
    pub preset: Preset,
    #[doc = "The language for the audio payload in the input using the BCP-47 format of 'language tag-region' (e.g: 'en-US').  If you know the language of your content, it is recommended that you specify it. The language must be specified explicitly for AudioAnalysisMode::Basic, since automatic language detection is not included in basic mode. If the language isn't specified or set to null, automatic language detection will choose the first language detected and process with the selected language for the duration of the file. It does not currently support dynamically switching between languages after the first language is detected. The automatic detection works best with audio recordings with clearly discernable speech. If automatic detection fails to find the language, transcription would fallback to 'en-US'.\" The list of supported languages is available here: https://go.microsoft.com/fwlink/?linkid=2109463"]
    #[serde(rename = "audioLanguage", default, skip_serializing_if = "Option::is_none")]
    pub audio_language: Option<String>,
    #[doc = "Determines the set of audio analysis operations to be performed. If unspecified, the Standard AudioAnalysisMode would be chosen."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<audio_analyzer_preset::Mode>,
    #[doc = "Dictionary containing key value pairs for parameters not exposed in the preset itself"]
    #[serde(rename = "experimentalOptions", default, skip_serializing_if = "Option::is_none")]
    pub experimental_options: Option<serde_json::Value>,
}
impl AudioAnalyzerPreset {
    pub fn new(preset: Preset) -> Self {
        Self {
            preset,
            audio_language: None,
            mode: None,
            experimental_options: None,
        }
    }
}
pub mod audio_analyzer_preset {
    use super::*;
    #[doc = "Determines the set of audio analysis operations to be performed. If unspecified, the Standard AudioAnalysisMode would be chosen."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        Standard,
        Basic,
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
                Self::Standard => serializer.serialize_unit_variant("Mode", 0u32, "Standard"),
                Self::Basic => serializer.serialize_unit_variant("Mode", 1u32, "Basic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the properties of an audio overlay."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioOverlay {
    #[serde(flatten)]
    pub overlay: Overlay,
}
impl AudioOverlay {
    pub fn new(overlay: Overlay) -> Self {
        Self { overlay }
    }
}
#[doc = "A TrackSelection to select audio tracks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioTrackDescriptor {
    #[serde(flatten)]
    pub track_descriptor: TrackDescriptor,
    #[doc = "Optional designation for single channel audio tracks.  Can be used to combine the tracks into stereo or multi-channel audio tracks."]
    #[serde(rename = "channelMapping", default, skip_serializing_if = "Option::is_none")]
    pub channel_mapping: Option<audio_track_descriptor::ChannelMapping>,
}
impl AudioTrackDescriptor {
    pub fn new(track_descriptor: TrackDescriptor) -> Self {
        Self {
            track_descriptor,
            channel_mapping: None,
        }
    }
}
pub mod audio_track_descriptor {
    use super::*;
    #[doc = "Optional designation for single channel audio tracks.  Can be used to combine the tracks into stereo or multi-channel audio tracks."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ChannelMapping")]
    pub enum ChannelMapping {
        FrontLeft,
        FrontRight,
        Center,
        LowFrequencyEffects,
        BackLeft,
        BackRight,
        StereoLeft,
        StereoRight,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ChannelMapping {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ChannelMapping {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ChannelMapping {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::FrontLeft => serializer.serialize_unit_variant("ChannelMapping", 0u32, "FrontLeft"),
                Self::FrontRight => serializer.serialize_unit_variant("ChannelMapping", 1u32, "FrontRight"),
                Self::Center => serializer.serialize_unit_variant("ChannelMapping", 2u32, "Center"),
                Self::LowFrequencyEffects => serializer.serialize_unit_variant("ChannelMapping", 3u32, "LowFrequencyEffects"),
                Self::BackLeft => serializer.serialize_unit_variant("ChannelMapping", 4u32, "BackLeft"),
                Self::BackRight => serializer.serialize_unit_variant("ChannelMapping", 5u32, "BackRight"),
                Self::StereoLeft => serializer.serialize_unit_variant("ChannelMapping", 6u32, "StereoLeft"),
                Self::StereoRight => serializer.serialize_unit_variant("ChannelMapping", 7u32, "StereoRight"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a built-in preset for encoding the input video with the Standard Encoder."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuiltInStandardEncoderPreset {
    #[serde(flatten)]
    pub preset: Preset,
    #[doc = "The built-in preset to be used for encoding videos."]
    #[serde(rename = "presetName")]
    pub preset_name: built_in_standard_encoder_preset::PresetName,
}
impl BuiltInStandardEncoderPreset {
    pub fn new(preset: Preset, preset_name: built_in_standard_encoder_preset::PresetName) -> Self {
        Self { preset, preset_name }
    }
}
pub mod built_in_standard_encoder_preset {
    use super::*;
    #[doc = "The built-in preset to be used for encoding videos."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PresetName")]
    pub enum PresetName {
        #[serde(rename = "H264SingleBitrateSD")]
        H264SingleBitrateSd,
        H264SingleBitrate720p,
        H264SingleBitrate1080p,
        AdaptiveStreaming,
        #[serde(rename = "AACGoodQualityAudio")]
        AacGoodQualityAudio,
        ContentAwareEncodingExperimental,
        ContentAwareEncoding,
        CopyAllBitrateNonInterleaved,
        H264MultipleBitrate1080p,
        H264MultipleBitrate720p,
        #[serde(rename = "H264MultipleBitrateSD")]
        H264MultipleBitrateSd,
        H265ContentAwareEncoding,
        H265AdaptiveStreaming,
        H265SingleBitrate720p,
        H265SingleBitrate1080p,
        H265SingleBitrate4K,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PresetName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PresetName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PresetName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::H264SingleBitrateSd => serializer.serialize_unit_variant("PresetName", 0u32, "H264SingleBitrateSD"),
                Self::H264SingleBitrate720p => serializer.serialize_unit_variant("PresetName", 1u32, "H264SingleBitrate720p"),
                Self::H264SingleBitrate1080p => serializer.serialize_unit_variant("PresetName", 2u32, "H264SingleBitrate1080p"),
                Self::AdaptiveStreaming => serializer.serialize_unit_variant("PresetName", 3u32, "AdaptiveStreaming"),
                Self::AacGoodQualityAudio => serializer.serialize_unit_variant("PresetName", 4u32, "AACGoodQualityAudio"),
                Self::ContentAwareEncodingExperimental => {
                    serializer.serialize_unit_variant("PresetName", 5u32, "ContentAwareEncodingExperimental")
                }
                Self::ContentAwareEncoding => serializer.serialize_unit_variant("PresetName", 6u32, "ContentAwareEncoding"),
                Self::CopyAllBitrateNonInterleaved => serializer.serialize_unit_variant("PresetName", 7u32, "CopyAllBitrateNonInterleaved"),
                Self::H264MultipleBitrate1080p => serializer.serialize_unit_variant("PresetName", 8u32, "H264MultipleBitrate1080p"),
                Self::H264MultipleBitrate720p => serializer.serialize_unit_variant("PresetName", 9u32, "H264MultipleBitrate720p"),
                Self::H264MultipleBitrateSd => serializer.serialize_unit_variant("PresetName", 10u32, "H264MultipleBitrateSD"),
                Self::H265ContentAwareEncoding => serializer.serialize_unit_variant("PresetName", 11u32, "H265ContentAwareEncoding"),
                Self::H265AdaptiveStreaming => serializer.serialize_unit_variant("PresetName", 12u32, "H265AdaptiveStreaming"),
                Self::H265SingleBitrate720p => serializer.serialize_unit_variant("PresetName", 13u32, "H265SingleBitrate720p"),
                Self::H265SingleBitrate1080p => serializer.serialize_unit_variant("PresetName", 14u32, "H265SingleBitrate1080p"),
                Self::H265SingleBitrate4K => serializer.serialize_unit_variant("PresetName", 15u32, "H265SingleBitrate4K"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Class to specify DRM configurations of CommonEncryptionCbcs scheme in Streaming Policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CbcsDrmConfiguration {
    #[doc = "Class to specify configurations of FairPlay in Streaming Policy"]
    #[serde(rename = "fairPlay", default, skip_serializing_if = "Option::is_none")]
    pub fair_play: Option<StreamingPolicyFairPlayConfiguration>,
    #[doc = "Class to specify configurations of PlayReady in Streaming Policy"]
    #[serde(rename = "playReady", default, skip_serializing_if = "Option::is_none")]
    pub play_ready: Option<StreamingPolicyPlayReadyConfiguration>,
    #[doc = "Class to specify configurations of Widevine in Streaming Policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widevine: Option<StreamingPolicyWidevineConfiguration>,
}
impl CbcsDrmConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to specify DRM configurations of CommonEncryptionCenc scheme in Streaming Policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CencDrmConfiguration {
    #[doc = "Class to specify configurations of PlayReady in Streaming Policy"]
    #[serde(rename = "playReady", default, skip_serializing_if = "Option::is_none")]
    pub play_ready: Option<StreamingPolicyPlayReadyConfiguration>,
    #[doc = "Class to specify configurations of Widevine in Streaming Policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widevine: Option<StreamingPolicyWidevineConfiguration>,
}
impl CencDrmConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The input to the check name availability request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityInput {
    #[doc = "The account name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The account type. For a Media Services account, this should be 'MediaServices'."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckNameAvailabilityInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for specifying a clip time. Use sub classes of this class to specify the time position in the media."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClipTime {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
}
impl ClipTime {
    pub fn new(odata_type: String) -> Self {
        Self { odata_type }
    }
}
#[doc = "Describes the basic properties of all codecs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Codec {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    #[doc = "An optional label for the codec. The label can be used to control muxing behavior."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
impl Codec {
    pub fn new(odata_type: String) -> Self {
        Self { odata_type, label: None }
    }
}
#[doc = "Class for CommonEncryptionCbcs encryption scheme"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommonEncryptionCbcs {
    #[doc = "Class to specify which protocols are enabled"]
    #[serde(rename = "enabledProtocols", default, skip_serializing_if = "Option::is_none")]
    pub enabled_protocols: Option<EnabledProtocols>,
    #[doc = "Representing which tracks should not be encrypted"]
    #[serde(rename = "clearTracks", default, skip_serializing_if = "Vec::is_empty")]
    pub clear_tracks: Vec<TrackSelection>,
    #[doc = "Class to specify properties of all content keys in Streaming Policy"]
    #[serde(rename = "contentKeys", default, skip_serializing_if = "Option::is_none")]
    pub content_keys: Option<StreamingPolicyContentKeys>,
    #[doc = "Class to specify DRM configurations of CommonEncryptionCbcs scheme in Streaming Policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drm: Option<CbcsDrmConfiguration>,
}
impl CommonEncryptionCbcs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for envelope encryption scheme"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommonEncryptionCenc {
    #[doc = "Class to specify which protocols are enabled"]
    #[serde(rename = "enabledProtocols", default, skip_serializing_if = "Option::is_none")]
    pub enabled_protocols: Option<EnabledProtocols>,
    #[doc = "Representing which tracks should not be encrypted"]
    #[serde(rename = "clearTracks", default, skip_serializing_if = "Vec::is_empty")]
    pub clear_tracks: Vec<TrackSelection>,
    #[doc = "Class to specify properties of all content keys in Streaming Policy"]
    #[serde(rename = "contentKeys", default, skip_serializing_if = "Option::is_none")]
    pub content_keys: Option<StreamingPolicyContentKeys>,
    #[doc = "Class to specify DRM configurations of CommonEncryptionCenc scheme in Streaming Policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drm: Option<CencDrmConfiguration>,
}
impl CommonEncryptionCenc {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Content Key Policy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContentKeyPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of the Content Key Policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ContentKeyPolicyProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ContentKeyPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a configuration for non-DRM keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyClearKeyConfiguration {
    #[serde(flatten)]
    pub content_key_policy_configuration: ContentKeyPolicyConfiguration,
}
impl ContentKeyPolicyClearKeyConfiguration {
    pub fn new(content_key_policy_configuration: ContentKeyPolicyConfiguration) -> Self {
        Self {
            content_key_policy_configuration,
        }
    }
}
#[doc = "A collection of ContentKeyPolicy items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContentKeyPolicyCollection {
    #[doc = "A collection of ContentKeyPolicy items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ContentKeyPolicy>,
    #[doc = "A link to the next page of the collection (when the collection contains too many results to return in one response)."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for ContentKeyPolicyCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl ContentKeyPolicyCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for Content Key Policy configuration. A derived class must be used to create a configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyConfiguration {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
}
impl ContentKeyPolicyConfiguration {
    pub fn new(odata_type: String) -> Self {
        Self { odata_type }
    }
}
#[doc = "Specifies a configuration for FairPlay licenses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyFairPlayConfiguration {
    #[serde(flatten)]
    pub content_key_policy_configuration: ContentKeyPolicyConfiguration,
    #[doc = "The key that must be used as FairPlay Application Secret key."]
    pub ask: String,
    #[doc = "The password encrypting FairPlay certificate in PKCS 12 (pfx) format."]
    #[serde(rename = "fairPlayPfxPassword")]
    pub fair_play_pfx_password: String,
    #[doc = "The Base64 representation of FairPlay certificate in PKCS 12 (pfx) format (including private key)."]
    #[serde(rename = "fairPlayPfx")]
    pub fair_play_pfx: String,
    #[doc = "The rental and lease key type."]
    #[serde(rename = "rentalAndLeaseKeyType")]
    pub rental_and_lease_key_type: content_key_policy_fair_play_configuration::RentalAndLeaseKeyType,
    #[doc = "The rental duration. Must be greater than or equal to 0."]
    #[serde(rename = "rentalDuration")]
    pub rental_duration: i64,
    #[serde(rename = "offlineRentalConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub offline_rental_configuration: Option<ContentKeyPolicyFairPlayOfflineRentalConfiguration>,
}
impl ContentKeyPolicyFairPlayConfiguration {
    pub fn new(
        content_key_policy_configuration: ContentKeyPolicyConfiguration,
        ask: String,
        fair_play_pfx_password: String,
        fair_play_pfx: String,
        rental_and_lease_key_type: content_key_policy_fair_play_configuration::RentalAndLeaseKeyType,
        rental_duration: i64,
    ) -> Self {
        Self {
            content_key_policy_configuration,
            ask,
            fair_play_pfx_password,
            fair_play_pfx,
            rental_and_lease_key_type,
            rental_duration,
            offline_rental_configuration: None,
        }
    }
}
pub mod content_key_policy_fair_play_configuration {
    use super::*;
    #[doc = "The rental and lease key type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RentalAndLeaseKeyType")]
    pub enum RentalAndLeaseKeyType {
        Unknown,
        Undefined,
        DualExpiry,
        PersistentUnlimited,
        PersistentLimited,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RentalAndLeaseKeyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RentalAndLeaseKeyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RentalAndLeaseKeyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("RentalAndLeaseKeyType", 0u32, "Unknown"),
                Self::Undefined => serializer.serialize_unit_variant("RentalAndLeaseKeyType", 1u32, "Undefined"),
                Self::DualExpiry => serializer.serialize_unit_variant("RentalAndLeaseKeyType", 2u32, "DualExpiry"),
                Self::PersistentUnlimited => serializer.serialize_unit_variant("RentalAndLeaseKeyType", 3u32, "PersistentUnlimited"),
                Self::PersistentLimited => serializer.serialize_unit_variant("RentalAndLeaseKeyType", 4u32, "PersistentLimited"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyFairPlayOfflineRentalConfiguration {
    #[doc = "Playback duration"]
    #[serde(rename = "playbackDurationSeconds")]
    pub playback_duration_seconds: i64,
    #[doc = "Storage duration"]
    #[serde(rename = "storageDurationSeconds")]
    pub storage_duration_seconds: i64,
}
impl ContentKeyPolicyFairPlayOfflineRentalConfiguration {
    pub fn new(playback_duration_seconds: i64, storage_duration_seconds: i64) -> Self {
        Self {
            playback_duration_seconds,
            storage_duration_seconds,
        }
    }
}
#[doc = "Represents an open restriction. License or key will be delivered on every request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyOpenRestriction {
    #[serde(flatten)]
    pub content_key_policy_restriction: ContentKeyPolicyRestriction,
}
impl ContentKeyPolicyOpenRestriction {
    pub fn new(content_key_policy_restriction: ContentKeyPolicyRestriction) -> Self {
        Self {
            content_key_policy_restriction,
        }
    }
}
#[doc = "Represents a policy option."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyOption {
    #[doc = "The legacy Policy Option ID."]
    #[serde(rename = "policyOptionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_option_id: Option<String>,
    #[doc = "The Policy Option description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Base class for Content Key Policy configuration. A derived class must be used to create a configuration."]
    pub configuration: ContentKeyPolicyConfiguration,
    #[doc = "Base class for Content Key Policy restrictions. A derived class must be used to create a restriction."]
    pub restriction: ContentKeyPolicyRestriction,
}
impl ContentKeyPolicyOption {
    pub fn new(configuration: ContentKeyPolicyConfiguration, restriction: ContentKeyPolicyRestriction) -> Self {
        Self {
            policy_option_id: None,
            name: None,
            configuration,
            restriction,
        }
    }
}
#[doc = "Specifies a configuration for PlayReady licenses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyPlayReadyConfiguration {
    #[serde(flatten)]
    pub content_key_policy_configuration: ContentKeyPolicyConfiguration,
    #[doc = "The PlayReady licenses."]
    pub licenses: Vec<ContentKeyPolicyPlayReadyLicense>,
    #[doc = "The custom response data."]
    #[serde(rename = "responseCustomData", default, skip_serializing_if = "Option::is_none")]
    pub response_custom_data: Option<String>,
}
impl ContentKeyPolicyPlayReadyConfiguration {
    pub fn new(content_key_policy_configuration: ContentKeyPolicyConfiguration, licenses: Vec<ContentKeyPolicyPlayReadyLicense>) -> Self {
        Self {
            content_key_policy_configuration,
            licenses,
            response_custom_data: None,
        }
    }
}
#[doc = "Specifies that the content key ID is in the PlayReady header."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyPlayReadyContentEncryptionKeyFromHeader {
    #[serde(flatten)]
    pub content_key_policy_play_ready_content_key_location: ContentKeyPolicyPlayReadyContentKeyLocation,
}
impl ContentKeyPolicyPlayReadyContentEncryptionKeyFromHeader {
    pub fn new(content_key_policy_play_ready_content_key_location: ContentKeyPolicyPlayReadyContentKeyLocation) -> Self {
        Self {
            content_key_policy_play_ready_content_key_location,
        }
    }
}
#[doc = "Specifies that the content key ID is specified in the PlayReady configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyPlayReadyContentEncryptionKeyFromKeyIdentifier {
    #[serde(flatten)]
    pub content_key_policy_play_ready_content_key_location: ContentKeyPolicyPlayReadyContentKeyLocation,
    #[doc = "The content key ID."]
    #[serde(rename = "keyId")]
    pub key_id: String,
}
impl ContentKeyPolicyPlayReadyContentEncryptionKeyFromKeyIdentifier {
    pub fn new(content_key_policy_play_ready_content_key_location: ContentKeyPolicyPlayReadyContentKeyLocation, key_id: String) -> Self {
        Self {
            content_key_policy_play_ready_content_key_location,
            key_id,
        }
    }
}
#[doc = "Base class for content key ID location. A derived class must be used to represent the location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyPlayReadyContentKeyLocation {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
}
impl ContentKeyPolicyPlayReadyContentKeyLocation {
    pub fn new(odata_type: String) -> Self {
        Self { odata_type }
    }
}
#[doc = "Configures the Explicit Analog Television Output Restriction control bits. For further details see the PlayReady Compliance Rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyPlayReadyExplicitAnalogTelevisionRestriction {
    #[doc = "Indicates whether this restriction is enforced on a Best Effort basis."]
    #[serde(rename = "bestEffort")]
    pub best_effort: bool,
    #[doc = "Configures the restriction control bits. Must be between 0 and 3 inclusive."]
    #[serde(rename = "configurationData")]
    pub configuration_data: i32,
}
impl ContentKeyPolicyPlayReadyExplicitAnalogTelevisionRestriction {
    pub fn new(best_effort: bool, configuration_data: i32) -> Self {
        Self {
            best_effort,
            configuration_data,
        }
    }
}
#[doc = "The PlayReady license"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyPlayReadyLicense {
    #[doc = "A flag indicating whether test devices can use the license."]
    #[serde(rename = "allowTestDevices")]
    pub allow_test_devices: bool,
    #[doc = "The begin date of license"]
    #[serde(rename = "beginDate", with = "azure_core::date::rfc3339::option")]
    pub begin_date: Option<time::OffsetDateTime>,
    #[doc = "The expiration date of license."]
    #[serde(rename = "expirationDate", with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "The relative begin date of license."]
    #[serde(rename = "relativeBeginDate", default, skip_serializing_if = "Option::is_none")]
    pub relative_begin_date: Option<String>,
    #[doc = "The relative expiration date of license."]
    #[serde(rename = "relativeExpirationDate", default, skip_serializing_if = "Option::is_none")]
    pub relative_expiration_date: Option<String>,
    #[doc = "The grace period of license."]
    #[serde(rename = "gracePeriod", default, skip_serializing_if = "Option::is_none")]
    pub grace_period: Option<String>,
    #[doc = "Configures the Play Right in the PlayReady license."]
    #[serde(rename = "playRight", default, skip_serializing_if = "Option::is_none")]
    pub play_right: Option<ContentKeyPolicyPlayReadyPlayRight>,
    #[doc = "The license type."]
    #[serde(rename = "licenseType")]
    pub license_type: content_key_policy_play_ready_license::LicenseType,
    #[doc = "Base class for content key ID location. A derived class must be used to represent the location."]
    #[serde(rename = "contentKeyLocation")]
    pub content_key_location: ContentKeyPolicyPlayReadyContentKeyLocation,
    #[doc = "The PlayReady content type."]
    #[serde(rename = "contentType")]
    pub content_type: content_key_policy_play_ready_license::ContentType,
}
impl ContentKeyPolicyPlayReadyLicense {
    pub fn new(
        allow_test_devices: bool,
        license_type: content_key_policy_play_ready_license::LicenseType,
        content_key_location: ContentKeyPolicyPlayReadyContentKeyLocation,
        content_type: content_key_policy_play_ready_license::ContentType,
    ) -> Self {
        Self {
            allow_test_devices,
            begin_date: None,
            expiration_date: None,
            relative_begin_date: None,
            relative_expiration_date: None,
            grace_period: None,
            play_right: None,
            license_type,
            content_key_location,
            content_type,
        }
    }
}
pub mod content_key_policy_play_ready_license {
    use super::*;
    #[doc = "The license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        Unknown,
        NonPersistent,
        Persistent,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("LicenseType", 0u32, "Unknown"),
                Self::NonPersistent => serializer.serialize_unit_variant("LicenseType", 1u32, "NonPersistent"),
                Self::Persistent => serializer.serialize_unit_variant("LicenseType", 2u32, "Persistent"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The PlayReady content type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ContentType")]
    pub enum ContentType {
        Unknown,
        Unspecified,
        UltraVioletDownload,
        UltraVioletStreaming,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ContentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ContentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ContentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("ContentType", 0u32, "Unknown"),
                Self::Unspecified => serializer.serialize_unit_variant("ContentType", 1u32, "Unspecified"),
                Self::UltraVioletDownload => serializer.serialize_unit_variant("ContentType", 2u32, "UltraVioletDownload"),
                Self::UltraVioletStreaming => serializer.serialize_unit_variant("ContentType", 3u32, "UltraVioletStreaming"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Configures the Play Right in the PlayReady license."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyPlayReadyPlayRight {
    #[doc = "The amount of time that the license is valid after the license is first used to play content."]
    #[serde(rename = "firstPlayExpiration", default, skip_serializing_if = "Option::is_none")]
    pub first_play_expiration: Option<String>,
    #[doc = "Configures the Serial Copy Management System (SCMS) in the license. Must be between 0 and 3 inclusive."]
    #[serde(rename = "scmsRestriction", default, skip_serializing_if = "Option::is_none")]
    pub scms_restriction: Option<i32>,
    #[doc = "Configures Automatic Gain Control (AGC) and Color Stripe in the license. Must be between 0 and 3 inclusive."]
    #[serde(rename = "agcAndColorStripeRestriction", default, skip_serializing_if = "Option::is_none")]
    pub agc_and_color_stripe_restriction: Option<i32>,
    #[doc = "Configures the Explicit Analog Television Output Restriction control bits. For further details see the PlayReady Compliance Rules."]
    #[serde(
        rename = "explicitAnalogTelevisionOutputRestriction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub explicit_analog_television_output_restriction: Option<ContentKeyPolicyPlayReadyExplicitAnalogTelevisionRestriction>,
    #[doc = "Enables the Image Constraint For Analog Component Video Restriction in the license."]
    #[serde(rename = "digitalVideoOnlyContentRestriction")]
    pub digital_video_only_content_restriction: bool,
    #[doc = "Enables the Image Constraint For Analog Component Video Restriction in the license."]
    #[serde(rename = "imageConstraintForAnalogComponentVideoRestriction")]
    pub image_constraint_for_analog_component_video_restriction: bool,
    #[doc = "Enables the Image Constraint For Analog Component Video Restriction in the license."]
    #[serde(rename = "imageConstraintForAnalogComputerMonitorRestriction")]
    pub image_constraint_for_analog_computer_monitor_restriction: bool,
    #[doc = "Configures Unknown output handling settings of the license."]
    #[serde(rename = "allowPassingVideoContentToUnknownOutput")]
    pub allow_passing_video_content_to_unknown_output: content_key_policy_play_ready_play_right::AllowPassingVideoContentToUnknownOutput,
    #[doc = "Specifies the output protection level for uncompressed digital video."]
    #[serde(rename = "uncompressedDigitalVideoOpl", default, skip_serializing_if = "Option::is_none")]
    pub uncompressed_digital_video_opl: Option<i32>,
    #[doc = "Specifies the output protection level for compressed digital video."]
    #[serde(rename = "compressedDigitalVideoOpl", default, skip_serializing_if = "Option::is_none")]
    pub compressed_digital_video_opl: Option<i32>,
    #[doc = "Specifies the output protection level for compressed digital audio."]
    #[serde(rename = "analogVideoOpl", default, skip_serializing_if = "Option::is_none")]
    pub analog_video_opl: Option<i32>,
    #[doc = "Specifies the output protection level for compressed digital audio."]
    #[serde(rename = "compressedDigitalAudioOpl", default, skip_serializing_if = "Option::is_none")]
    pub compressed_digital_audio_opl: Option<i32>,
    #[doc = "Specifies the output protection level for uncompressed digital audio."]
    #[serde(rename = "uncompressedDigitalAudioOpl", default, skip_serializing_if = "Option::is_none")]
    pub uncompressed_digital_audio_opl: Option<i32>,
}
impl ContentKeyPolicyPlayReadyPlayRight {
    pub fn new(
        digital_video_only_content_restriction: bool,
        image_constraint_for_analog_component_video_restriction: bool,
        image_constraint_for_analog_computer_monitor_restriction: bool,
        allow_passing_video_content_to_unknown_output: content_key_policy_play_ready_play_right::AllowPassingVideoContentToUnknownOutput,
    ) -> Self {
        Self {
            first_play_expiration: None,
            scms_restriction: None,
            agc_and_color_stripe_restriction: None,
            explicit_analog_television_output_restriction: None,
            digital_video_only_content_restriction,
            image_constraint_for_analog_component_video_restriction,
            image_constraint_for_analog_computer_monitor_restriction,
            allow_passing_video_content_to_unknown_output,
            uncompressed_digital_video_opl: None,
            compressed_digital_video_opl: None,
            analog_video_opl: None,
            compressed_digital_audio_opl: None,
            uncompressed_digital_audio_opl: None,
        }
    }
}
pub mod content_key_policy_play_ready_play_right {
    use super::*;
    #[doc = "Configures Unknown output handling settings of the license."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AllowPassingVideoContentToUnknownOutput")]
    pub enum AllowPassingVideoContentToUnknownOutput {
        Unknown,
        NotAllowed,
        Allowed,
        AllowedWithVideoConstriction,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AllowPassingVideoContentToUnknownOutput {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AllowPassingVideoContentToUnknownOutput {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AllowPassingVideoContentToUnknownOutput {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("AllowPassingVideoContentToUnknownOutput", 0u32, "Unknown"),
                Self::NotAllowed => serializer.serialize_unit_variant("AllowPassingVideoContentToUnknownOutput", 1u32, "NotAllowed"),
                Self::Allowed => serializer.serialize_unit_variant("AllowPassingVideoContentToUnknownOutput", 2u32, "Allowed"),
                Self::AllowedWithVideoConstriction => {
                    serializer.serialize_unit_variant("AllowPassingVideoContentToUnknownOutput", 3u32, "AllowedWithVideoConstriction")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of the Content Key Policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyProperties {
    #[doc = "The legacy Policy ID."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "The creation date of the Policy"]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The last modified date of the Policy"]
    #[serde(rename = "lastModified", with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
    #[doc = "A description for the Policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Key Policy options."]
    pub options: Vec<ContentKeyPolicyOption>,
}
impl ContentKeyPolicyProperties {
    pub fn new(options: Vec<ContentKeyPolicyOption>) -> Self {
        Self {
            policy_id: None,
            created: None,
            last_modified: None,
            description: None,
            options,
        }
    }
}
#[doc = "Base class for Content Key Policy restrictions. A derived class must be used to create a restriction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyRestriction {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
}
impl ContentKeyPolicyRestriction {
    pub fn new(odata_type: String) -> Self {
        Self { odata_type }
    }
}
#[doc = "Base class for Content Key Policy key for token validation. A derived class must be used to create a token key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyRestrictionTokenKey {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
}
impl ContentKeyPolicyRestrictionTokenKey {
    pub fn new(odata_type: String) -> Self {
        Self { odata_type }
    }
}
#[doc = "Specifies a RSA key for token validation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyRsaTokenKey {
    #[serde(flatten)]
    pub content_key_policy_restriction_token_key: ContentKeyPolicyRestrictionTokenKey,
    #[doc = "The RSA Parameter exponent"]
    pub exponent: String,
    #[doc = "The RSA Parameter modulus"]
    pub modulus: String,
}
impl ContentKeyPolicyRsaTokenKey {
    pub fn new(content_key_policy_restriction_token_key: ContentKeyPolicyRestrictionTokenKey, exponent: String, modulus: String) -> Self {
        Self {
            content_key_policy_restriction_token_key,
            exponent,
            modulus,
        }
    }
}
#[doc = "Specifies a symmetric key for token validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicySymmetricTokenKey {
    #[serde(flatten)]
    pub content_key_policy_restriction_token_key: ContentKeyPolicyRestrictionTokenKey,
    #[doc = "The key value of the key"]
    #[serde(rename = "keyValue")]
    pub key_value: String,
}
impl ContentKeyPolicySymmetricTokenKey {
    pub fn new(content_key_policy_restriction_token_key: ContentKeyPolicyRestrictionTokenKey, key_value: String) -> Self {
        Self {
            content_key_policy_restriction_token_key,
            key_value,
        }
    }
}
#[doc = "Represents a token claim."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContentKeyPolicyTokenClaim {
    #[doc = "Token claim type."]
    #[serde(rename = "claimType", default, skip_serializing_if = "Option::is_none")]
    pub claim_type: Option<String>,
    #[doc = "Token claim value."]
    #[serde(rename = "claimValue", default, skip_serializing_if = "Option::is_none")]
    pub claim_value: Option<String>,
}
impl ContentKeyPolicyTokenClaim {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a token restriction. Provided token must match these requirements for successful license or key delivery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyTokenRestriction {
    #[serde(flatten)]
    pub content_key_policy_restriction: ContentKeyPolicyRestriction,
    #[doc = "The token issuer."]
    pub issuer: String,
    #[doc = "The audience for the token."]
    pub audience: String,
    #[doc = "Base class for Content Key Policy key for token validation. A derived class must be used to create a token key."]
    #[serde(rename = "primaryVerificationKey")]
    pub primary_verification_key: ContentKeyPolicyRestrictionTokenKey,
    #[doc = "A list of alternative verification keys."]
    #[serde(rename = "alternateVerificationKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub alternate_verification_keys: Vec<ContentKeyPolicyRestrictionTokenKey>,
    #[doc = "A list of required token claims."]
    #[serde(rename = "requiredClaims", default, skip_serializing_if = "Vec::is_empty")]
    pub required_claims: Vec<ContentKeyPolicyTokenClaim>,
    #[doc = "The type of token."]
    #[serde(rename = "restrictionTokenType")]
    pub restriction_token_type: content_key_policy_token_restriction::RestrictionTokenType,
    #[doc = "The OpenID connect discovery document."]
    #[serde(rename = "openIdConnectDiscoveryDocument", default, skip_serializing_if = "Option::is_none")]
    pub open_id_connect_discovery_document: Option<String>,
}
impl ContentKeyPolicyTokenRestriction {
    pub fn new(
        content_key_policy_restriction: ContentKeyPolicyRestriction,
        issuer: String,
        audience: String,
        primary_verification_key: ContentKeyPolicyRestrictionTokenKey,
        restriction_token_type: content_key_policy_token_restriction::RestrictionTokenType,
    ) -> Self {
        Self {
            content_key_policy_restriction,
            issuer,
            audience,
            primary_verification_key,
            alternate_verification_keys: Vec::new(),
            required_claims: Vec::new(),
            restriction_token_type,
            open_id_connect_discovery_document: None,
        }
    }
}
pub mod content_key_policy_token_restriction {
    use super::*;
    #[doc = "The type of token."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RestrictionTokenType")]
    pub enum RestrictionTokenType {
        Unknown,
        Swt,
        Jwt,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RestrictionTokenType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RestrictionTokenType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RestrictionTokenType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("RestrictionTokenType", 0u32, "Unknown"),
                Self::Swt => serializer.serialize_unit_variant("RestrictionTokenType", 1u32, "Swt"),
                Self::Jwt => serializer.serialize_unit_variant("RestrictionTokenType", 2u32, "Jwt"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents a ContentKeyPolicyConfiguration that is unavailable in the current API version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyUnknownConfiguration {
    #[serde(flatten)]
    pub content_key_policy_configuration: ContentKeyPolicyConfiguration,
}
impl ContentKeyPolicyUnknownConfiguration {
    pub fn new(content_key_policy_configuration: ContentKeyPolicyConfiguration) -> Self {
        Self {
            content_key_policy_configuration,
        }
    }
}
#[doc = "Represents a ContentKeyPolicyRestriction that is unavailable in the current API version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyUnknownRestriction {
    #[serde(flatten)]
    pub content_key_policy_restriction: ContentKeyPolicyRestriction,
}
impl ContentKeyPolicyUnknownRestriction {
    pub fn new(content_key_policy_restriction: ContentKeyPolicyRestriction) -> Self {
        Self {
            content_key_policy_restriction,
        }
    }
}
#[doc = "Specifies a configuration for Widevine licenses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyWidevineConfiguration {
    #[serde(flatten)]
    pub content_key_policy_configuration: ContentKeyPolicyConfiguration,
    #[doc = "The Widevine template."]
    #[serde(rename = "widevineTemplate")]
    pub widevine_template: String,
}
impl ContentKeyPolicyWidevineConfiguration {
    pub fn new(content_key_policy_configuration: ContentKeyPolicyConfiguration, widevine_template: String) -> Self {
        Self {
            content_key_policy_configuration,
            widevine_template,
        }
    }
}
#[doc = "Specifies a certificate for token validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentKeyPolicyX509CertificateTokenKey {
    #[serde(flatten)]
    pub content_key_policy_restriction_token_key: ContentKeyPolicyRestrictionTokenKey,
    #[doc = "The raw data field of a certificate in PKCS 12 format (X509Certificate2 in .NET)"]
    #[serde(rename = "rawBody")]
    pub raw_body: String,
}
impl ContentKeyPolicyX509CertificateTokenKey {
    pub fn new(content_key_policy_restriction_token_key: ContentKeyPolicyRestrictionTokenKey, raw_body: String) -> Self {
        Self {
            content_key_policy_restriction_token_key,
            raw_body,
        }
    }
}
#[doc = "A codec flag, which tells the encoder to copy the input audio bitstream."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CopyAudio {
    #[serde(flatten)]
    pub codec: Codec,
}
impl CopyAudio {
    pub fn new(codec: Codec) -> Self {
        Self { codec }
    }
}
#[doc = "A codec flag, which tells the encoder to copy the input video bitstream without re-encoding."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CopyVideo {
    #[serde(flatten)]
    pub codec: Codec,
}
impl CopyVideo {
    pub fn new(codec: Codec) -> Self {
        Self { codec }
    }
}
#[doc = "The client access policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CrossSiteAccessPolicies {
    #[doc = "The content of clientaccesspolicy.xml used by Silverlight."]
    #[serde(rename = "clientAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub client_access_policy: Option<String>,
    #[doc = "The content of crossdomain.xml used by Silverlight."]
    #[serde(rename = "crossDomainPolicy", default, skip_serializing_if = "Option::is_none")]
    pub cross_domain_policy: Option<String>,
}
impl CrossSiteAccessPolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to specify properties of default content key for each encryption scheme"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultKey {
    #[doc = "Label can be used to specify Content Key when creating a Streaming Locator"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Policy used by Default Key"]
    #[serde(rename = "policyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}
impl DefaultKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the de-interlacing settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Deinterlace {
    #[doc = "The field parity for de-interlacing, defaults to Auto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parity: Option<deinterlace::Parity>,
    #[doc = "The deinterlacing mode. Defaults to AutoPixelAdaptive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<deinterlace::Mode>,
}
impl Deinterlace {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deinterlace {
    use super::*;
    #[doc = "The field parity for de-interlacing, defaults to Auto."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Parity")]
    pub enum Parity {
        Auto,
        TopFieldFirst,
        BottomFieldFirst,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Parity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Parity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Parity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Auto => serializer.serialize_unit_variant("Parity", 0u32, "Auto"),
                Self::TopFieldFirst => serializer.serialize_unit_variant("Parity", 1u32, "TopFieldFirst"),
                Self::BottomFieldFirst => serializer.serialize_unit_variant("Parity", 2u32, "BottomFieldFirst"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The deinterlacing mode. Defaults to AutoPixelAdaptive."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        Off,
        AutoPixelAdaptive,
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
                Self::Off => serializer.serialize_unit_variant("Mode", 0u32, "Off"),
                Self::AutoPixelAdaptive => serializer.serialize_unit_variant("Mode", 1u32, "AutoPixelAdaptive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgePolicies {
    #[serde(rename = "usageDataCollectionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub usage_data_collection_policy: Option<EdgeUsageDataCollectionPolicy>,
}
impl EdgePolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgeUsageDataCollectionPolicy {
    #[doc = "Usage data collection frequency in ISO 8601 duration format e.g. PT10M , PT5H."]
    #[serde(rename = "dataCollectionFrequency", default, skip_serializing_if = "Option::is_none")]
    pub data_collection_frequency: Option<String>,
    #[doc = "Usage data reporting frequency in ISO 8601 duration format e.g. PT10M , PT5H."]
    #[serde(rename = "dataReportingFrequency", default, skip_serializing_if = "Option::is_none")]
    pub data_reporting_frequency: Option<String>,
    #[doc = "Maximum time for which the functionality of the device will not be hampered for not reporting the usage data."]
    #[serde(rename = "maxAllowedUnreportedUsageDuration", default, skip_serializing_if = "Option::is_none")]
    pub max_allowed_unreported_usage_duration: Option<String>,
    #[serde(rename = "eventHubDetails", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_details: Option<EdgeUsageDataEventHub>,
}
impl EdgeUsageDataCollectionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgeUsageDataEventHub {
    #[doc = "Name of the Event Hub where usage will be reported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Namespace of the Event Hub where usage will be reported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "SAS token needed to interact with Event Hub."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl EdgeUsageDataEventHub {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to specify which protocols are enabled"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnabledProtocols {
    #[doc = "Enable Download protocol or not"]
    pub download: bool,
    #[doc = "Enable DASH protocol or not"]
    pub dash: bool,
    #[doc = "Enable HLS protocol or not"]
    pub hls: bool,
    #[doc = "Enable SmoothStreaming protocol or not"]
    #[serde(rename = "smoothStreaming")]
    pub smooth_streaming: bool,
}
impl EnabledProtocols {
    pub fn new(download: bool, dash: bool, hls: bool, smooth_streaming: bool) -> Self {
        Self {
            download,
            dash,
            hls,
            smooth_streaming,
        }
    }
}
#[doc = "The response from the check name availability request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityNameAvailabilityCheckOutput {
    #[doc = "Specifies if the name is available."]
    #[serde(rename = "nameAvailable")]
    pub name_available: bool,
    #[doc = "Specifies the reason if the name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Specifies the detailed reason if the name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl EntityNameAvailabilityCheckOutput {
    pub fn new(name_available: bool) -> Self {
        Self {
            name_available,
            reason: None,
            message: None,
        }
    }
}
#[doc = "Class for EnvelopeEncryption encryption scheme"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvelopeEncryption {
    #[doc = "Class to specify which protocols are enabled"]
    #[serde(rename = "enabledProtocols", default, skip_serializing_if = "Option::is_none")]
    pub enabled_protocols: Option<EnabledProtocols>,
    #[doc = "Representing which tracks should not be encrypted"]
    #[serde(rename = "clearTracks", default, skip_serializing_if = "Vec::is_empty")]
    pub clear_tracks: Vec<TrackSelection>,
    #[doc = "Class to specify properties of all content keys in Streaming Policy"]
    #[serde(rename = "contentKeys", default, skip_serializing_if = "Option::is_none")]
    pub content_keys: Option<StreamingPolicyContentKeys>,
    #[doc = "Template for the URL of the custom service delivering keys to end user players.  Not required when using Azure Media Services for issuing keys.  The template supports replaceable tokens that the service will update at runtime with the value specific to the request.  The currently supported token values are {AlternativeMediaId}, which is replaced with the value of StreamingLocatorId.AlternativeMediaId, and {ContentKeyId}, which is replaced with the value of identifier of the key being requested."]
    #[serde(rename = "customKeyAcquisitionUrlTemplate", default, skip_serializing_if = "Option::is_none")]
    pub custom_key_acquisition_url_template: Option<String>,
}
impl EnvelopeEncryption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes all the settings to be used when analyzing a video in order to detect (and optionally redact) all the faces present."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FaceDetectorPreset {
    #[serde(flatten)]
    pub preset: Preset,
    #[doc = "Specifies the maximum resolution at which your video is analyzed. The default behavior is \"SourceResolution,\" which will keep the input video at its original resolution when analyzed. Using \"StandardDefinition\" will resize input videos to standard definition while preserving the appropriate aspect ratio. It will only resize if the video is of higher resolution. For example, a 1920x1080 input would be scaled to 640x360 before processing. Switching to \"StandardDefinition\" will reduce the time it takes to process high resolution video. It may also reduce the cost of using this component (see https://azure.microsoft.com/en-us/pricing/details/media-services/#analytics for details). However, faces that end up being too small in the resized video may not be detected."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<face_detector_preset::Resolution>,
    #[doc = "This mode provides the ability to choose between the following settings: 1) Analyze - For detection only.This mode generates a metadata JSON file marking appearances of faces throughout the video.Where possible, appearances of the same person are assigned the same ID. 2) Combined - Additionally redacts(blurs) detected faces. 3) Redact - This enables a 2-pass process, allowing for selective redaction of a subset of detected faces.It takes in the metadata file from a prior analyze pass, along with the source video, and a user-selected subset of IDs that require redaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<face_detector_preset::Mode>,
    #[doc = "Blur type"]
    #[serde(rename = "blurType", default, skip_serializing_if = "Option::is_none")]
    pub blur_type: Option<face_detector_preset::BlurType>,
    #[doc = "Dictionary containing key value pairs for parameters not exposed in the preset itself"]
    #[serde(rename = "experimentalOptions", default, skip_serializing_if = "Option::is_none")]
    pub experimental_options: Option<serde_json::Value>,
}
impl FaceDetectorPreset {
    pub fn new(preset: Preset) -> Self {
        Self {
            preset,
            resolution: None,
            mode: None,
            blur_type: None,
            experimental_options: None,
        }
    }
}
pub mod face_detector_preset {
    use super::*;
    #[doc = "Specifies the maximum resolution at which your video is analyzed. The default behavior is \"SourceResolution,\" which will keep the input video at its original resolution when analyzed. Using \"StandardDefinition\" will resize input videos to standard definition while preserving the appropriate aspect ratio. It will only resize if the video is of higher resolution. For example, a 1920x1080 input would be scaled to 640x360 before processing. Switching to \"StandardDefinition\" will reduce the time it takes to process high resolution video. It may also reduce the cost of using this component (see https://azure.microsoft.com/en-us/pricing/details/media-services/#analytics for details). However, faces that end up being too small in the resized video may not be detected."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Resolution")]
    pub enum Resolution {
        SourceResolution,
        StandardDefinition,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Resolution {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Resolution {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Resolution {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SourceResolution => serializer.serialize_unit_variant("Resolution", 0u32, "SourceResolution"),
                Self::StandardDefinition => serializer.serialize_unit_variant("Resolution", 1u32, "StandardDefinition"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "This mode provides the ability to choose between the following settings: 1) Analyze - For detection only.This mode generates a metadata JSON file marking appearances of faces throughout the video.Where possible, appearances of the same person are assigned the same ID. 2) Combined - Additionally redacts(blurs) detected faces. 3) Redact - This enables a 2-pass process, allowing for selective redaction of a subset of detected faces.It takes in the metadata file from a prior analyze pass, along with the source video, and a user-selected subset of IDs that require redaction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        Analyze,
        Redact,
        Combined,
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
                Self::Analyze => serializer.serialize_unit_variant("Mode", 0u32, "Analyze"),
                Self::Redact => serializer.serialize_unit_variant("Mode", 1u32, "Redact"),
                Self::Combined => serializer.serialize_unit_variant("Mode", 2u32, "Combined"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Blur type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BlurType")]
    pub enum BlurType {
        Box,
        Low,
        Med,
        High,
        Black,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BlurType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BlurType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BlurType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Box => serializer.serialize_unit_variant("BlurType", 0u32, "Box"),
                Self::Low => serializer.serialize_unit_variant("BlurType", 1u32, "Low"),
                Self::Med => serializer.serialize_unit_variant("BlurType", 2u32, "Med"),
                Self::High => serializer.serialize_unit_variant("BlurType", 3u32, "High"),
                Self::Black => serializer.serialize_unit_variant("BlurType", 4u32, "Black"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The class to specify one track property condition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterTrackPropertyCondition {
    #[doc = "The track property type."]
    pub property: filter_track_property_condition::Property,
    #[doc = "The track property value."]
    pub value: String,
    #[doc = "The track property condition operation."]
    pub operation: filter_track_property_condition::Operation,
}
impl FilterTrackPropertyCondition {
    pub fn new(
        property: filter_track_property_condition::Property,
        value: String,
        operation: filter_track_property_condition::Operation,
    ) -> Self {
        Self {
            property,
            value,
            operation,
        }
    }
}
pub mod filter_track_property_condition {
    use super::*;
    #[doc = "The track property type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Property")]
    pub enum Property {
        Unknown,
        Type,
        Name,
        Language,
        #[serde(rename = "FourCC")]
        FourCc,
        Bitrate,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Property {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Property {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Property {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("Property", 0u32, "Unknown"),
                Self::Type => serializer.serialize_unit_variant("Property", 1u32, "Type"),
                Self::Name => serializer.serialize_unit_variant("Property", 2u32, "Name"),
                Self::Language => serializer.serialize_unit_variant("Property", 3u32, "Language"),
                Self::FourCc => serializer.serialize_unit_variant("Property", 4u32, "FourCC"),
                Self::Bitrate => serializer.serialize_unit_variant("Property", 5u32, "Bitrate"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The track property condition operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operation")]
    pub enum Operation {
        Equal,
        NotEqual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Equal => serializer.serialize_unit_variant("Operation", 0u32, "Equal"),
                Self::NotEqual => serializer.serialize_unit_variant("Operation", 1u32, "NotEqual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Representing a list of FilterTrackPropertyConditions to select a track.  The filters are combined using a logical AND operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterTrackSelection {
    #[doc = "The track selections."]
    #[serde(rename = "trackSelections")]
    pub track_selections: Vec<FilterTrackPropertyCondition>,
}
impl FilterTrackSelection {
    pub fn new(track_selections: Vec<FilterTrackPropertyCondition>) -> Self {
        Self { track_selections }
    }
}
#[doc = "Describes all the filtering operations, such as de-interlacing, rotation etc. that are to be applied to the input media before encoding."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Filters {
    #[doc = "Describes the de-interlacing settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deinterlace: Option<Deinterlace>,
    #[doc = "The rotation, if any, to be applied to the input video, before it is encoded. Default is Auto"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<filters::Rotation>,
    #[doc = "Describes the properties of a rectangular window applied to the input media before processing it."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crop: Option<Rectangle>,
    #[doc = "The properties of overlays to be applied to the input video. These could be audio, image or video overlays."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub overlays: Vec<Overlay>,
}
impl Filters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod filters {
    use super::*;
    #[doc = "The rotation, if any, to be applied to the input video, before it is encoded. Default is Auto"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Rotation")]
    pub enum Rotation {
        Auto,
        None,
        Rotate0,
        Rotate90,
        Rotate180,
        Rotate270,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Rotation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Rotation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Rotation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Auto => serializer.serialize_unit_variant("Rotation", 0u32, "Auto"),
                Self::None => serializer.serialize_unit_variant("Rotation", 1u32, "None"),
                Self::Rotate0 => serializer.serialize_unit_variant("Rotation", 2u32, "Rotate0"),
                Self::Rotate90 => serializer.serialize_unit_variant("Rotation", 3u32, "Rotate90"),
                Self::Rotate180 => serializer.serialize_unit_variant("Rotation", 4u32, "Rotate180"),
                Self::Rotate270 => serializer.serialize_unit_variant("Rotation", 5u32, "Rotate270"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Filter First Quality"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirstQuality {
    #[doc = "The first quality bitrate."]
    pub bitrate: i32,
}
impl FirstQuality {
    pub fn new(bitrate: i32) -> Self {
        Self { bitrate }
    }
}
#[doc = "Base class for output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Format {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    #[doc = "The pattern of the file names for the generated output files. The following macros are supported in the file name: {Basename} - An expansion macro that will use the name of the input video file. If the base name(the file suffix is not included) of the input video file is less than 32 characters long, the base name of input video files will be used. If the length of base name of the input video file exceeds 32 characters, the base name is truncated to the first 32 characters in total length. {Extension} - The appropriate extension for this format. {Label} - The label assigned to the codec/layer. {Index} - A unique index for thumbnails. Only applicable to thumbnails. {Bitrate} - The audio/video bitrate. Not applicable to thumbnails. {Codec} - The type of the audio/video codec. {Resolution} - The video resolution. Any unsubstituted macros will be collapsed and removed from the filename."]
    #[serde(rename = "filenamePattern")]
    pub filename_pattern: String,
}
impl Format {
    pub fn new(odata_type: String, filename_pattern: String) -> Self {
        Self {
            odata_type,
            filename_pattern,
        }
    }
}
#[doc = "An InputDefinition that looks across all of the files provided to select tracks specified by the IncludedTracks property. Generally used with the AudioTrackByAttribute and VideoTrackByAttribute to allow selection of a single track across a set of input files."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FromAllInputFile {
    #[serde(flatten)]
    pub input_definition: InputDefinition,
}
impl FromAllInputFile {
    pub fn new(input_definition: InputDefinition) -> Self {
        Self { input_definition }
    }
}
#[doc = "An InputDefinition that looks at each input file provided to select tracks specified by the IncludedTracks property. Generally used with the AudioTrackByAttribute and VideoTrackByAttribute to select tracks from each file given."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FromEachInputFile {
    #[serde(flatten)]
    pub input_definition: InputDefinition,
}
impl FromEachInputFile {
    pub fn new(input_definition: InputDefinition) -> Self {
        Self { input_definition }
    }
}
#[doc = "Describes the settings to be used when encoding the input video into a desired output bitrate layer with the H.264 video codec."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct H264Layer {
    #[serde(flatten)]
    pub video_layer: VideoLayer,
    #[doc = "We currently support Baseline, Main, High, High422, High444. Default is Auto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<h264_layer::Profile>,
    #[doc = "We currently support Level up to 6.2. The value can be Auto, or a number that matches the H.264 profile. If not specified, the default is Auto, which lets the encoder choose the Level that is appropriate for this layer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[doc = "The VBV buffer window length. The value should be in ISO 8601 format. The value should be in the range [0.1-100] seconds. The default is 5 seconds (for example, PT5S)."]
    #[serde(rename = "bufferWindow", default, skip_serializing_if = "Option::is_none")]
    pub buffer_window: Option<String>,
    #[doc = "The number of reference frames to be used when encoding this layer. If not specified, the encoder determines an appropriate number based on the encoder complexity setting."]
    #[serde(rename = "referenceFrames", default, skip_serializing_if = "Option::is_none")]
    pub reference_frames: Option<i32>,
    #[doc = "The entropy mode to be used for this layer. If not specified, the encoder chooses the mode that is appropriate for the profile and level."]
    #[serde(rename = "entropyMode", default, skip_serializing_if = "Option::is_none")]
    pub entropy_mode: Option<h264_layer::EntropyMode>,
}
impl H264Layer {
    pub fn new(video_layer: VideoLayer) -> Self {
        Self {
            video_layer,
            profile: None,
            level: None,
            buffer_window: None,
            reference_frames: None,
            entropy_mode: None,
        }
    }
}
pub mod h264_layer {
    use super::*;
    #[doc = "We currently support Baseline, Main, High, High422, High444. Default is Auto."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Profile")]
    pub enum Profile {
        Auto,
        Baseline,
        Main,
        High,
        High422,
        High444,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Profile {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Profile {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Profile {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Auto => serializer.serialize_unit_variant("Profile", 0u32, "Auto"),
                Self::Baseline => serializer.serialize_unit_variant("Profile", 1u32, "Baseline"),
                Self::Main => serializer.serialize_unit_variant("Profile", 2u32, "Main"),
                Self::High => serializer.serialize_unit_variant("Profile", 3u32, "High"),
                Self::High422 => serializer.serialize_unit_variant("Profile", 4u32, "High422"),
                Self::High444 => serializer.serialize_unit_variant("Profile", 5u32, "High444"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The entropy mode to be used for this layer. If not specified, the encoder chooses the mode that is appropriate for the profile and level."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EntropyMode")]
    pub enum EntropyMode {
        Cabac,
        Cavlc,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EntropyMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EntropyMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EntropyMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Cabac => serializer.serialize_unit_variant("EntropyMode", 0u32, "Cabac"),
                Self::Cavlc => serializer.serialize_unit_variant("EntropyMode", 1u32, "Cavlc"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes all the properties for encoding a video with the H.264 codec."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct H264Video {
    #[serde(flatten)]
    pub video: Video,
    #[doc = "Whether or not the encoder should insert key frames at scene changes. If not specified, the default is false. This flag should be set to true only when the encoder is being configured to produce a single output video."]
    #[serde(rename = "sceneChangeDetection", default, skip_serializing_if = "Option::is_none")]
    pub scene_change_detection: Option<bool>,
    #[doc = "Tells the encoder how to choose its encoding settings. The default value is Balanced."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complexity: Option<h264_video::Complexity>,
    #[doc = "The collection of output H.264 layers to be produced by the encoder."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<H264Layer>,
}
impl H264Video {
    pub fn new(video: Video) -> Self {
        Self {
            video,
            scene_change_detection: None,
            complexity: None,
            layers: Vec::new(),
        }
    }
}
pub mod h264_video {
    use super::*;
    #[doc = "Tells the encoder how to choose its encoding settings. The default value is Balanced."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Complexity")]
    pub enum Complexity {
        Speed,
        Balanced,
        Quality,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Complexity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Complexity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Complexity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Speed => serializer.serialize_unit_variant("Complexity", 0u32, "Speed"),
                Self::Balanced => serializer.serialize_unit_variant("Complexity", 1u32, "Balanced"),
                Self::Quality => serializer.serialize_unit_variant("Complexity", 2u32, "Quality"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the settings to be used when encoding the input video into a desired output bitrate layer with the H.265 video codec."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct H265Layer {
    #[serde(flatten)]
    pub h265_video_layer: H265VideoLayer,
    #[doc = "We currently support Main. Default is Auto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<h265_layer::Profile>,
    #[doc = "We currently support Level up to 6.2. The value can be Auto, or a number that matches the H.265 profile. If not specified, the default is Auto, which lets the encoder choose the Level that is appropriate for this layer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[doc = "The VBV buffer window length. The value should be in ISO 8601 format. The value should be in the range [0.1-100] seconds. The default is 5 seconds (for example, PT5S)."]
    #[serde(rename = "bufferWindow", default, skip_serializing_if = "Option::is_none")]
    pub buffer_window: Option<String>,
    #[doc = "The number of reference frames to be used when encoding this layer. If not specified, the encoder determines an appropriate number based on the encoder complexity setting."]
    #[serde(rename = "referenceFrames", default, skip_serializing_if = "Option::is_none")]
    pub reference_frames: Option<i32>,
}
impl H265Layer {
    pub fn new(h265_video_layer: H265VideoLayer) -> Self {
        Self {
            h265_video_layer,
            profile: None,
            level: None,
            buffer_window: None,
            reference_frames: None,
        }
    }
}
pub mod h265_layer {
    use super::*;
    #[doc = "We currently support Main. Default is Auto."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Profile")]
    pub enum Profile {
        Auto,
        Main,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Profile {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Profile {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Profile {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Auto => serializer.serialize_unit_variant("Profile", 0u32, "Auto"),
                Self::Main => serializer.serialize_unit_variant("Profile", 1u32, "Main"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes all the properties for encoding a video with the H.265 codec."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct H265Video {
    #[serde(flatten)]
    pub video: Video,
    #[doc = "Specifies whether or not the encoder should insert key frames at scene changes. If not specified, the default is false. This flag should be set to true only when the encoder is being configured to produce a single output video."]
    #[serde(rename = "sceneChangeDetection", default, skip_serializing_if = "Option::is_none")]
    pub scene_change_detection: Option<bool>,
    #[doc = "Tells the encoder how to choose its encoding settings.  Quality will provide for a higher compression ratio but at a higher cost and longer compute time.  Speed will produce a relatively larger file but is faster and more economical. The default value is Balanced."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complexity: Option<h265_video::Complexity>,
    #[doc = "The collection of output H.265 layers to be produced by the encoder."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<H265Layer>,
}
impl H265Video {
    pub fn new(video: Video) -> Self {
        Self {
            video,
            scene_change_detection: None,
            complexity: None,
            layers: Vec::new(),
        }
    }
}
pub mod h265_video {
    use super::*;
    #[doc = "Tells the encoder how to choose its encoding settings.  Quality will provide for a higher compression ratio but at a higher cost and longer compute time.  Speed will produce a relatively larger file but is faster and more economical. The default value is Balanced."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Complexity")]
    pub enum Complexity {
        Speed,
        Balanced,
        Quality,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Complexity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Complexity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Complexity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Speed => serializer.serialize_unit_variant("Complexity", 0u32, "Speed"),
                Self::Balanced => serializer.serialize_unit_variant("Complexity", 1u32, "Balanced"),
                Self::Quality => serializer.serialize_unit_variant("Complexity", 2u32, "Quality"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the settings to be used when encoding the input video into a desired output bitrate layer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct H265VideoLayer {
    #[serde(flatten)]
    pub layer: Layer,
    #[doc = "The average bitrate in bits per second at which to encode the input video when generating this layer. For example: a target bitrate of 3000Kbps or 3Mbps means this value should be 3000000 This is a required field."]
    pub bitrate: i32,
    #[doc = "The maximum bitrate (in bits per second), at which the VBV buffer should be assumed to refill. If not specified, defaults to the same value as bitrate."]
    #[serde(rename = "maxBitrate", default, skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i32>,
    #[doc = "The number of B-frames to be used when encoding this layer.  If not specified, the encoder chooses an appropriate number based on the video profile and level."]
    #[serde(rename = "bFrames", default, skip_serializing_if = "Option::is_none")]
    pub b_frames: Option<i32>,
    #[doc = "The frame rate (in frames per second) at which to encode this layer. The value can be in the form of M/N where M and N are integers (For example, 30000/1001), or in the form of a number (For example, 30, or 29.97). The encoder enforces constraints on allowed frame rates based on the profile and level. If it is not specified, the encoder will use the same frame rate as the input video."]
    #[serde(rename = "frameRate", default, skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<String>,
    #[doc = "The number of slices to be used when encoding this layer. If not specified, default is zero, which means that encoder will use a single slice for each frame."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slices: Option<i32>,
    #[doc = "Specifies whether or not adaptive B-frames are to be used when encoding this layer. If not specified, the encoder will turn it on whenever the video profile permits its use."]
    #[serde(rename = "adaptiveBFrame", default, skip_serializing_if = "Option::is_none")]
    pub adaptive_b_frame: Option<bool>,
}
impl H265VideoLayer {
    pub fn new(layer: Layer, bitrate: i32) -> Self {
        Self {
            layer,
            bitrate,
            max_bitrate: None,
            b_frames: None,
            frame_rate: None,
            slices: None,
            adaptive_b_frame: None,
        }
    }
}
#[doc = "HTTP Live Streaming (HLS) packing setting for the live output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Hls {
    #[doc = "The number of fragments in an HTTP Live Streaming (HLS) TS segment in the output of the live event. This value does not affect the packing ratio for HLS CMAF output."]
    #[serde(rename = "fragmentsPerTsSegment", default, skip_serializing_if = "Option::is_none")]
    pub fragments_per_ts_segment: Option<i32>,
}
impl Hls {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The IP access control."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpAccessControl {
    #[doc = "The IP allow list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allow: Vec<IpRange>,
}
impl IpAccessControl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The IP address range in the CIDR scheme."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpRange {
    #[doc = "The friendly name for the IP address range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The IP address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "The subnet mask prefix length (see CIDR notation)."]
    #[serde(rename = "subnetPrefixLength", default, skip_serializing_if = "Option::is_none")]
    pub subnet_prefix_length: Option<i32>,
}
impl IpRange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the basic properties for generating thumbnails from the input video"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Image {
    #[serde(flatten)]
    pub video: Video,
    #[doc = "The position in the input video from where to start generating thumbnails. The value can be in ISO 8601 format (For example, PT05S to start at 5 seconds), or a frame count (For example, 10 to start at the 10th frame), or a relative value to stream duration (For example, 10% to start at 10% of stream duration). Also supports a macro {Best}, which tells the encoder to select the best thumbnail from the first few seconds of the video and will only produce one thumbnail, no matter what other settings are for Step and Range. The default value is macro {Best}."]
    pub start: String,
    #[doc = "The intervals at which thumbnails are generated. The value can be in ISO 8601 format (For example, PT05S for one image every 5 seconds), or a frame count (For example, 30 for one image every 30 frames), or a relative value to stream duration (For example, 10% for one image every 10% of stream duration). Note: Step value will affect the first generated thumbnail, which may not be exactly the one specified at transform preset start time. This is due to the encoder, which tries to select the best thumbnail between start time and Step position from start time as the first output. As the default value is 10%, it means if stream has long duration, the first generated thumbnail might be far away from the one specified at start time. Try to select reasonable value for Step if the first thumbnail is expected close to start time, or set Range value at 1 if only one thumbnail is needed at start time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<String>,
    #[doc = "The position relative to transform preset start time in the input video at which to stop generating thumbnails. The value can be in ISO 8601 format (For example, PT5M30S to stop at 5 minutes and 30 seconds from start time), or a frame count (For example, 300 to stop at the 300th frame from the frame at start time. If this value is 1, it means only producing one thumbnail at start time), or a relative value to the stream duration (For example, 50% to stop at half of stream duration from start time). The default value is 100%, which means to stop at the end of the stream."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}
impl Image {
    pub fn new(video: Video, start: String) -> Self {
        Self {
            video,
            start,
            step: None,
            range: None,
        }
    }
}
#[doc = "Describes the properties for an output image file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageFormat {
    #[serde(flatten)]
    pub format: Format,
}
impl ImageFormat {
    pub fn new(format: Format) -> Self {
        Self { format }
    }
}
#[doc = "Base class for defining an input. Use sub classes of this class to specify tracks selections and related metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputDefinition {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    #[doc = "The list of TrackDescriptors which define the metadata and selection of tracks in the input."]
    #[serde(rename = "includedTracks", default, skip_serializing_if = "Vec::is_empty")]
    pub included_tracks: Vec<TrackDescriptor>,
}
impl InputDefinition {
    pub fn new(odata_type: String) -> Self {
        Self {
            odata_type,
            included_tracks: Vec::new(),
        }
    }
}
#[doc = "An InputDefinition for a single file.  TrackSelections are scoped to the file specified."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputFile {
    #[serde(flatten)]
    pub input_definition: InputDefinition,
    #[doc = "Name of the file that this input definition applies to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}
impl InputFile {
    pub fn new(input_definition: InputDefinition) -> Self {
        Self {
            input_definition,
            filename: None,
        }
    }
}
#[doc = "A Job resource type. The progress and state can be obtained by polling a Job or subscribing to events using EventGrid."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Job {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the Job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Job {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of Job items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobCollection {
    #[doc = "A collection of Job items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Job>,
    #[doc = "A link to the next page of the collection (when the collection contains too many results to return in one response)."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for JobCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl JobCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of JobOutput errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobError {
    #[doc = "Error code describing the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<job_error::Code>,
    #[doc = "A human-readable language-dependent representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Helps with categorization of errors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<job_error::Category>,
    #[doc = "Indicates that it may be possible to retry the Job. If retry is unsuccessful, please contact Azure support via Azure Portal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry: Option<job_error::Retry>,
    #[doc = "An array of details about specific errors that led to this reported error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<JobErrorDetail>,
}
impl JobError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_error {
    use super::*;
    #[doc = "Error code describing the error."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Code")]
    pub enum Code {
        ServiceError,
        ServiceTransientError,
        DownloadNotAccessible,
        DownloadTransientError,
        UploadNotAccessible,
        UploadTransientError,
        ConfigurationUnsupported,
        ContentMalformed,
        ContentUnsupported,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Code {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Code {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Code {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ServiceError => serializer.serialize_unit_variant("Code", 0u32, "ServiceError"),
                Self::ServiceTransientError => serializer.serialize_unit_variant("Code", 1u32, "ServiceTransientError"),
                Self::DownloadNotAccessible => serializer.serialize_unit_variant("Code", 2u32, "DownloadNotAccessible"),
                Self::DownloadTransientError => serializer.serialize_unit_variant("Code", 3u32, "DownloadTransientError"),
                Self::UploadNotAccessible => serializer.serialize_unit_variant("Code", 4u32, "UploadNotAccessible"),
                Self::UploadTransientError => serializer.serialize_unit_variant("Code", 5u32, "UploadTransientError"),
                Self::ConfigurationUnsupported => serializer.serialize_unit_variant("Code", 6u32, "ConfigurationUnsupported"),
                Self::ContentMalformed => serializer.serialize_unit_variant("Code", 7u32, "ContentMalformed"),
                Self::ContentUnsupported => serializer.serialize_unit_variant("Code", 8u32, "ContentUnsupported"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Helps with categorization of errors."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Category")]
    pub enum Category {
        Service,
        Download,
        Upload,
        Configuration,
        Content,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Category {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Category {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Category {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Service => serializer.serialize_unit_variant("Category", 0u32, "Service"),
                Self::Download => serializer.serialize_unit_variant("Category", 1u32, "Download"),
                Self::Upload => serializer.serialize_unit_variant("Category", 2u32, "Upload"),
                Self::Configuration => serializer.serialize_unit_variant("Category", 3u32, "Configuration"),
                Self::Content => serializer.serialize_unit_variant("Category", 4u32, "Content"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates that it may be possible to retry the Job. If retry is unsuccessful, please contact Azure support via Azure Portal."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Retry")]
    pub enum Retry {
        DoNotRetry,
        MayRetry,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Retry {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Retry {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Retry {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::DoNotRetry => serializer.serialize_unit_variant("Retry", 0u32, "DoNotRetry"),
                Self::MayRetry => serializer.serialize_unit_variant("Retry", 1u32, "MayRetry"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Details of JobOutput errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobErrorDetail {
    #[doc = "Code describing the error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A human-readable representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl JobErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for inputs to a Job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobInput {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
}
impl JobInput {
    pub fn new(odata_type: String) -> Self {
        Self { odata_type }
    }
}
#[doc = "Represents an Asset for input into a Job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobInputAsset {
    #[serde(flatten)]
    pub job_input_clip: JobInputClip,
    #[doc = "The name of the input Asset."]
    #[serde(rename = "assetName")]
    pub asset_name: String,
}
impl JobInputAsset {
    pub fn new(job_input_clip: JobInputClip, asset_name: String) -> Self {
        Self {
            job_input_clip,
            asset_name,
        }
    }
}
#[doc = "Represents input files for a Job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobInputClip {
    #[serde(flatten)]
    pub job_input: JobInput,
    #[doc = "List of files. Required for JobInputHttp. Maximum of 4000 characters each."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    #[doc = "Base class for specifying a clip time. Use sub classes of this class to specify the time position in the media."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<ClipTime>,
    #[doc = "Base class for specifying a clip time. Use sub classes of this class to specify the time position in the media."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<ClipTime>,
    #[doc = "A label that is assigned to a JobInputClip, that is used to satisfy a reference used in the Transform. For example, a Transform can be authored so as to take an image file with the label 'xyz' and apply it as an overlay onto the input video before it is encoded. When submitting a Job, exactly one of the JobInputs should be the image file, and it should have the label 'xyz'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Defines a list of InputDefinitions. For each InputDefinition, it defines a list of track selections and related metadata."]
    #[serde(rename = "inputDefinitions", default, skip_serializing_if = "Vec::is_empty")]
    pub input_definitions: Vec<InputDefinition>,
}
impl JobInputClip {
    pub fn new(job_input: JobInput) -> Self {
        Self {
            job_input,
            files: Vec::new(),
            start: None,
            end: None,
            label: None,
            input_definitions: Vec::new(),
        }
    }
}
#[doc = "Represents HTTPS job input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobInputHttp {
    #[serde(flatten)]
    pub job_input_clip: JobInputClip,
    #[doc = "Base URI for HTTPS job input. It will be concatenated with provided file names. If no base uri is given, then the provided file list is assumed to be fully qualified uris. Maximum length of 4000 characters."]
    #[serde(rename = "baseUri", default, skip_serializing_if = "Option::is_none")]
    pub base_uri: Option<String>,
}
impl JobInputHttp {
    pub fn new(job_input_clip: JobInputClip) -> Self {
        Self {
            job_input_clip,
            base_uri: None,
        }
    }
}
#[doc = "A Sequence contains an ordered list of Clips where each clip is a JobInput.  The Sequence will be treated as a single input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobInputSequence {
    #[serde(flatten)]
    pub job_input: JobInput,
    #[doc = "JobInputs that make up the timeline."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<JobInputClip>,
}
impl JobInputSequence {
    pub fn new(job_input: JobInput) -> Self {
        Self {
            job_input,
            inputs: Vec::new(),
        }
    }
}
#[doc = "Describes a list of inputs to a Job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobInputs {
    #[serde(flatten)]
    pub job_input: JobInput,
    #[doc = "List of inputs to a Job."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<JobInput>,
}
impl JobInputs {
    pub fn new(job_input: JobInput) -> Self {
        Self {
            job_input,
            inputs: Vec::new(),
        }
    }
}
#[doc = "Describes all the properties of a JobOutput."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobOutput {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    #[doc = "Details of JobOutput errors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<JobError>,
    #[doc = "Describes the state of the JobOutput."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<job_output::State>,
    #[doc = "If the JobOutput is in a Processing state, this contains the Job completion percentage. The value is an estimate and not intended to be used to predict Job completion times. To determine if the JobOutput is complete, use the State property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    #[doc = "A label that is assigned to a JobOutput in order to help uniquely identify it. This is useful when your Transform has more than one TransformOutput, whereby your Job has more than one JobOutput. In such cases, when you submit the Job, you will add two or more JobOutputs, in the same order as TransformOutputs in the Transform. Subsequently, when you retrieve the Job, either through events or on a GET request, you can use the label to easily identify the JobOutput. If a label is not provided, a default value of '{presetName}_{outputIndex}' will be used, where the preset name is the name of the preset in the corresponding TransformOutput and the output index is the relative index of the this JobOutput within the Job. Note that this index is the same as the relative index of the corresponding TransformOutput within its Transform."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "The UTC date and time at which this Job Output began processing."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The UTC date and time at which this Job Output finished processing."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}
impl JobOutput {
    pub fn new(odata_type: String) -> Self {
        Self {
            odata_type,
            error: None,
            state: None,
            progress: None,
            label: None,
            start_time: None,
            end_time: None,
        }
    }
}
pub mod job_output {
    use super::*;
    #[doc = "Describes the state of the JobOutput."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Canceled,
        Canceling,
        Error,
        Finished,
        Processing,
        Queued,
        Scheduled,
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
                Self::Canceled => serializer.serialize_unit_variant("State", 0u32, "Canceled"),
                Self::Canceling => serializer.serialize_unit_variant("State", 1u32, "Canceling"),
                Self::Error => serializer.serialize_unit_variant("State", 2u32, "Error"),
                Self::Finished => serializer.serialize_unit_variant("State", 3u32, "Finished"),
                Self::Processing => serializer.serialize_unit_variant("State", 4u32, "Processing"),
                Self::Queued => serializer.serialize_unit_variant("State", 5u32, "Queued"),
                Self::Scheduled => serializer.serialize_unit_variant("State", 6u32, "Scheduled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents an Asset used as a JobOutput."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobOutputAsset {
    #[serde(flatten)]
    pub job_output: JobOutput,
    #[doc = "The name of the output Asset."]
    #[serde(rename = "assetName")]
    pub asset_name: String,
}
impl JobOutputAsset {
    pub fn new(job_output: JobOutput, asset_name: String) -> Self {
        Self { job_output, asset_name }
    }
}
#[doc = "Properties of the Job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobProperties {
    #[doc = "The UTC date and time when the customer has created the Job, in 'YYYY-MM-DDThh:mm:ssZ' format."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The current state of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<job_properties::State>,
    #[doc = "Optional customer supplied description of the Job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Base class for inputs to a Job."]
    pub input: JobInput,
    #[doc = "The UTC date and time when the customer has last updated the Job, in 'YYYY-MM-DDThh:mm:ssZ' format."]
    #[serde(rename = "lastModified", with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
    #[doc = "The outputs for the Job."]
    pub outputs: Vec<JobOutput>,
    #[doc = "Priority with which the job should be processed. Higher priority jobs are processed before lower priority jobs. If not set, the default is normal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<job_properties::Priority>,
    #[doc = "Customer provided key, value pairs that will be returned in Job and JobOutput state events."]
    #[serde(rename = "correlationData", default, skip_serializing_if = "Option::is_none")]
    pub correlation_data: Option<serde_json::Value>,
    #[doc = "The UTC date and time at which this Job began processing."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The UTC date and time at which this Job finished processing."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}
impl JobProperties {
    pub fn new(input: JobInput, outputs: Vec<JobOutput>) -> Self {
        Self {
            created: None,
            state: None,
            description: None,
            input,
            last_modified: None,
            outputs,
            priority: None,
            correlation_data: None,
            start_time: None,
            end_time: None,
        }
    }
}
pub mod job_properties {
    use super::*;
    #[doc = "The current state of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Canceled,
        Canceling,
        Error,
        Finished,
        Processing,
        Queued,
        Scheduled,
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
                Self::Canceled => serializer.serialize_unit_variant("State", 0u32, "Canceled"),
                Self::Canceling => serializer.serialize_unit_variant("State", 1u32, "Canceling"),
                Self::Error => serializer.serialize_unit_variant("State", 2u32, "Error"),
                Self::Finished => serializer.serialize_unit_variant("State", 3u32, "Finished"),
                Self::Processing => serializer.serialize_unit_variant("State", 4u32, "Processing"),
                Self::Queued => serializer.serialize_unit_variant("State", 5u32, "Queued"),
                Self::Scheduled => serializer.serialize_unit_variant("State", 6u32, "Scheduled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Priority with which the job should be processed. Higher priority jobs are processed before lower priority jobs. If not set, the default is normal."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Priority")]
    pub enum Priority {
        Low,
        Normal,
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Priority {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Priority {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Priority {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Low => serializer.serialize_unit_variant("Priority", 0u32, "Low"),
                Self::Normal => serializer.serialize_unit_variant("Priority", 1u32, "Normal"),
                Self::High => serializer.serialize_unit_variant("Priority", 2u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the settings for producing JPEG thumbnails."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JpgFormat {
    #[serde(flatten)]
    pub image_format: ImageFormat,
}
impl JpgFormat {
    pub fn new(image_format: ImageFormat) -> Self {
        Self { image_format }
    }
}
#[doc = "Describes the properties for producing a series of JPEG images from the input video."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JpgImage {
    #[serde(flatten)]
    pub image: Image,
    #[doc = "A collection of output JPEG image layers to be produced by the encoder."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<JpgLayer>,
    #[doc = "Sets the number of columns used in thumbnail sprite image.  The number of rows are automatically calculated and a VTT file is generated with the coordinate mappings for each thumbnail in the sprite. Note: this value should be a positive integer and a proper value is recommended so that the output image resolution will not go beyond JPEG maximum pixel resolution limit 65535x65535."]
    #[serde(rename = "spriteColumn", default, skip_serializing_if = "Option::is_none")]
    pub sprite_column: Option<i32>,
}
impl JpgImage {
    pub fn new(image: Image) -> Self {
        Self {
            image,
            layers: Vec::new(),
            sprite_column: None,
        }
    }
}
#[doc = "Describes the settings to produce a JPEG image from the input video."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JpgLayer {
    #[serde(flatten)]
    pub layer: Layer,
    #[doc = "The compression quality of the JPEG output. Range is from 0-100 and the default is 70."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<i32>,
}
impl JpgLayer {
    pub fn new(layer: Layer) -> Self {
        Self { layer, quality: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyDelivery {
    #[serde(rename = "accessControl", default, skip_serializing_if = "Option::is_none")]
    pub access_control: Option<AccessControl>,
}
impl KeyDelivery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultProperties {
    #[doc = "The URL of the Key Vault key used to encrypt the account. The key may either be versioned (for example https://vault/keys/mykey/version1) or reference a key without a version (for example https://vault/keys/mykey)."]
    #[serde(rename = "keyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub key_identifier: Option<String>,
    #[doc = "The current key used to encrypt the Media Services account, including the key version."]
    #[serde(rename = "currentKeyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub current_key_identifier: Option<String>,
}
impl KeyVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The encoder can be configured to produce video and/or images (thumbnails) at different resolutions, by specifying a layer for each desired resolution. A layer represents the properties for the video or image at a resolution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    #[doc = "The width of the output video for this layer. The value can be absolute (in pixels) or relative (in percentage). For example 50% means the output video has half as many pixels in width as the input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[doc = "The height of the output video for this layer. The value can be absolute (in pixels) or relative (in percentage). For example 50% means the output video has half as many pixels in height as the input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[doc = "The alphanumeric label for this layer, which can be used in multiplexing different video and audio layers, or in naming the output file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
impl Layer {
    pub fn new(odata_type: String) -> Self {
        Self {
            odata_type,
            width: None,
            height: None,
            label: None,
        }
    }
}
#[doc = "The parameters to the list SAS request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListContainerSasInput {
    #[doc = "The permissions to set on the SAS URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<list_container_sas_input::Permissions>,
    #[doc = "The SAS URL expiration time.  This must be less than 24 hours from the current time."]
    #[serde(rename = "expiryTime", with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
}
impl ListContainerSasInput {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod list_container_sas_input {
    use super::*;
    #[doc = "The permissions to set on the SAS URL."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Permissions")]
    pub enum Permissions {
        Read,
        ReadWrite,
        ReadWriteDelete,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Permissions {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Permissions {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Permissions {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Read => serializer.serialize_unit_variant("Permissions", 0u32, "Read"),
                Self::ReadWrite => serializer.serialize_unit_variant("Permissions", 1u32, "ReadWrite"),
                Self::ReadWriteDelete => serializer.serialize_unit_variant("Permissions", 2u32, "ReadWriteDelete"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Class of response for listContentKeys action"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListContentKeysResponse {
    #[doc = "ContentKeys used by current Streaming Locator"]
    #[serde(rename = "contentKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub content_keys: Vec<StreamingLocatorContentKey>,
}
impl ListContentKeysResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListEdgePoliciesInput {
    #[doc = "Unique identifier of the edge device."]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}
impl ListEdgePoliciesInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class of response for listPaths action"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListPathsResponse {
    #[doc = "Streaming Paths supported by current Streaming Locator"]
    #[serde(rename = "streamingPaths", default, skip_serializing_if = "Vec::is_empty")]
    pub streaming_paths: Vec<StreamingPath>,
    #[doc = "Download Paths supported by current Streaming Locator"]
    #[serde(rename = "downloadPaths", default, skip_serializing_if = "Vec::is_empty")]
    pub download_paths: Vec<String>,
}
impl ListPathsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Streaming Locators associated with this Asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListStreamingLocatorsResponse {
    #[doc = "The list of Streaming Locators."]
    #[serde(rename = "streamingLocators", default, skip_serializing_if = "Vec::is_empty")]
    pub streaming_locators: Vec<AssetStreamingLocator>,
}
impl ListStreamingLocatorsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The live event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveEvent {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The live event properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LiveEventProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl LiveEvent {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "The LiveEvent action input parameter definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveEventActionInput {
    #[doc = "The flag indicates whether live outputs are automatically deleted when live event is being stopped. Deleting live outputs do not delete the underlying assets."]
    #[serde(rename = "removeOutputsOnStop", default, skip_serializing_if = "Option::is_none")]
    pub remove_outputs_on_stop: Option<bool>,
}
impl LiveEventActionInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the live event type and optional encoding settings for encoding live events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveEventEncoding {
    #[doc = "Live event type. When encodingType is set to None, the service simply passes through the incoming video and audio layer(s) to the output. When encodingType is set to Standard or Premium1080p, a live encoder transcodes the incoming stream into multiple bitrates or layers. See https://go.microsoft.com/fwlink/?linkid=2095101 for more information. This property cannot be modified after the live event is created."]
    #[serde(rename = "encodingType", default, skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<live_event_encoding::EncodingType>,
    #[doc = "The optional encoding preset name, used when encodingType is not None. This value is specified at creation time and cannot be updated. If the encodingType is set to Standard, then the default preset name is ‘Default720p’. Else if the encodingType is set to Premium1080p, the default preset is ‘Default1080p’."]
    #[serde(rename = "presetName", default, skip_serializing_if = "Option::is_none")]
    pub preset_name: Option<String>,
    #[doc = "Specifies how the input video will be resized to fit the desired output resolution(s). Default is None"]
    #[serde(rename = "stretchMode", default, skip_serializing_if = "Option::is_none")]
    pub stretch_mode: Option<live_event_encoding::StretchMode>,
    #[doc = "Use an ISO 8601 time value between 0.5 to 20 seconds to specify the output fragment length for the video and audio tracks of an encoding live event. For example, use PT2S to indicate 2 seconds. For the video track it also defines the key frame interval, or the length of a GoP (group of pictures).   If this value is not set for an encoding live event, the fragment duration defaults to 2 seconds. The value cannot be set for pass-through live events."]
    #[serde(rename = "keyFrameInterval", default, skip_serializing_if = "Option::is_none")]
    pub key_frame_interval: Option<String>,
}
impl LiveEventEncoding {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod live_event_encoding {
    use super::*;
    #[doc = "Live event type. When encodingType is set to None, the service simply passes through the incoming video and audio layer(s) to the output. When encodingType is set to Standard or Premium1080p, a live encoder transcodes the incoming stream into multiple bitrates or layers. See https://go.microsoft.com/fwlink/?linkid=2095101 for more information. This property cannot be modified after the live event is created."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EncodingType")]
    pub enum EncodingType {
        None,
        Standard,
        Premium1080p,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EncodingType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EncodingType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EncodingType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("EncodingType", 0u32, "None"),
                Self::Standard => serializer.serialize_unit_variant("EncodingType", 1u32, "Standard"),
                Self::Premium1080p => serializer.serialize_unit_variant("EncodingType", 2u32, "Premium1080p"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies how the input video will be resized to fit the desired output resolution(s). Default is None"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StretchMode")]
    pub enum StretchMode {
        None,
        AutoSize,
        AutoFit,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StretchMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StretchMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StretchMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("StretchMode", 0u32, "None"),
                Self::AutoSize => serializer.serialize_unit_variant("StretchMode", 1u32, "AutoSize"),
                Self::AutoFit => serializer.serialize_unit_variant("StretchMode", 2u32, "AutoFit"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The live event endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveEventEndpoint {
    #[doc = "The endpoint protocol."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[doc = "The endpoint URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl LiveEventEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The live event input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveEventInput {
    #[doc = "The input protocol for the live event. This is specified at creation time and cannot be updated."]
    #[serde(rename = "streamingProtocol")]
    pub streaming_protocol: live_event_input::StreamingProtocol,
    #[doc = "The IP access control for live event input."]
    #[serde(rename = "accessControl", default, skip_serializing_if = "Option::is_none")]
    pub access_control: Option<LiveEventInputAccessControl>,
    #[doc = "ISO 8601 time duration of the key frame interval duration of the input. This value sets the EXT-X-TARGETDURATION property in the HLS output. For example, use PT2S to indicate 2 seconds. Leave the value empty for encoding live events."]
    #[serde(rename = "keyFrameIntervalDuration", default, skip_serializing_if = "Option::is_none")]
    pub key_frame_interval_duration: Option<String>,
    #[doc = "A UUID in string form to uniquely identify the stream. This can be specified at creation time but cannot be updated. If omitted, the service will generate a unique value."]
    #[serde(rename = "accessToken", default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[doc = "The input endpoints for the live event."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<LiveEventEndpoint>,
}
impl LiveEventInput {
    pub fn new(streaming_protocol: live_event_input::StreamingProtocol) -> Self {
        Self {
            streaming_protocol,
            access_control: None,
            key_frame_interval_duration: None,
            access_token: None,
            endpoints: Vec::new(),
        }
    }
}
pub mod live_event_input {
    use super::*;
    #[doc = "The input protocol for the live event. This is specified at creation time and cannot be updated."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StreamingProtocol")]
    pub enum StreamingProtocol {
        #[serde(rename = "FragmentedMP4")]
        FragmentedMp4,
        #[serde(rename = "RTMP")]
        Rtmp,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StreamingProtocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StreamingProtocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StreamingProtocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::FragmentedMp4 => serializer.serialize_unit_variant("StreamingProtocol", 0u32, "FragmentedMP4"),
                Self::Rtmp => serializer.serialize_unit_variant("StreamingProtocol", 1u32, "RTMP"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The IP access control for live event input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveEventInputAccessControl {
    #[doc = "The IP access control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<IpAccessControl>,
}
impl LiveEventInputAccessControl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A track selection condition. This property is reserved for future use, any value set on this property will be ignored."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveEventInputTrackSelection {
    #[doc = "Property name to select. This property is reserved for future use, any value set on this property will be ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    #[doc = "Comparing operation. This property is reserved for future use, any value set on this property will be ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Property value to select. This property is reserved for future use, any value set on this property will be ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl LiveEventInputTrackSelection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The LiveEvent list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveEventListResult {
    #[doc = "The result of the List Live Event operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LiveEvent>,
    #[doc = "The number of result."]
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[doc = "The link to the next set of results. Not empty if value contains incomplete list of live outputs."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for LiveEventListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl LiveEventListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a transcription track in the output of a live event, generated using speech-to-text transcription. This property is reserved for future use, any value set on this property will be ignored."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveEventOutputTranscriptionTrack {
    #[doc = "The output track name. This property is reserved for future use, any value set on this property will be ignored."]
    #[serde(rename = "trackName")]
    pub track_name: String,
}
impl LiveEventOutputTranscriptionTrack {
    pub fn new(track_name: String) -> Self {
        Self { track_name }
    }
}
#[doc = "Live event preview settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveEventPreview {
    #[doc = "The endpoints for preview. Do not share the preview URL with the live event audience."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<LiveEventEndpoint>,
    #[doc = "The IP access control for the live event preview endpoint."]
    #[serde(rename = "accessControl", default, skip_serializing_if = "Option::is_none")]
    pub access_control: Option<LiveEventPreviewAccessControl>,
    #[doc = "The identifier of the preview locator in Guid format. Specifying this at creation time allows the caller to know the preview locator url before the event is created. If omitted, the service will generate a random identifier. This value cannot be updated once the live event is created."]
    #[serde(rename = "previewLocator", default, skip_serializing_if = "Option::is_none")]
    pub preview_locator: Option<String>,
    #[doc = "The name of streaming policy used for the live event preview. This value is specified at creation time and cannot be updated."]
    #[serde(rename = "streamingPolicyName", default, skip_serializing_if = "Option::is_none")]
    pub streaming_policy_name: Option<String>,
    #[doc = "An alternative media identifier associated with the streaming locator created for the preview. This value is specified at creation time and cannot be updated. The identifier can be used in the CustomLicenseAcquisitionUrlTemplate or the CustomKeyAcquisitionUrlTemplate of the StreamingPolicy specified in the StreamingPolicyName field."]
    #[serde(rename = "alternativeMediaId", default, skip_serializing_if = "Option::is_none")]
    pub alternative_media_id: Option<String>,
}
impl LiveEventPreview {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The IP access control for the live event preview endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveEventPreviewAccessControl {
    #[doc = "The IP access control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<IpAccessControl>,
}
impl LiveEventPreviewAccessControl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The live event properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveEventProperties {
    #[doc = "A description for the live event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The live event input."]
    pub input: LiveEventInput,
    #[doc = "Live event preview settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<LiveEventPreview>,
    #[doc = "Specifies the live event type and optional encoding settings for encoding live events."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<LiveEventEncoding>,
    #[doc = "Live transcription settings for the live event. See https://go.microsoft.com/fwlink/?linkid=2133742 for more information about the live transcription feature."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transcriptions: Vec<LiveEventTranscription>,
    #[doc = "The provisioning state of the live event."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The resource state of the live event. See https://go.microsoft.com/fwlink/?linkid=2139012 for more information."]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<live_event_properties::ResourceState>,
    #[doc = "The client access policy."]
    #[serde(rename = "crossSiteAccessPolicies", default, skip_serializing_if = "Option::is_none")]
    pub cross_site_access_policies: Option<CrossSiteAccessPolicies>,
    #[doc = "Specifies whether a static hostname would be assigned to the live event preview and ingest endpoints. This value can only be updated if the live event is in Standby state"]
    #[serde(rename = "useStaticHostname", default, skip_serializing_if = "Option::is_none")]
    pub use_static_hostname: Option<bool>,
    #[doc = "When useStaticHostname is set to true, the hostnamePrefix specifies the first part of the hostname assigned to the live event preview and ingest endpoints. The final hostname would be a combination of this prefix, the media service account name and a short code for the Azure Media Services data center."]
    #[serde(rename = "hostnamePrefix", default, skip_serializing_if = "Option::is_none")]
    pub hostname_prefix: Option<String>,
    #[doc = "The options to use for the LiveEvent. This value is specified at creation time and cannot be updated. The valid values for the array entry values are 'Default' and 'LowLatency'."]
    #[serde(rename = "streamOptions", default, skip_serializing_if = "Vec::is_empty")]
    pub stream_options: Vec<String>,
    #[doc = "The creation time for the live event"]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The last modified time of the live event."]
    #[serde(rename = "lastModified", with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
}
impl LiveEventProperties {
    pub fn new(input: LiveEventInput) -> Self {
        Self {
            description: None,
            input,
            preview: None,
            encoding: None,
            transcriptions: Vec::new(),
            provisioning_state: None,
            resource_state: None,
            cross_site_access_policies: None,
            use_static_hostname: None,
            hostname_prefix: None,
            stream_options: Vec::new(),
            created: None,
            last_modified: None,
        }
    }
}
pub mod live_event_properties {
    use super::*;
    #[doc = "The resource state of the live event. See https://go.microsoft.com/fwlink/?linkid=2139012 for more information."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceState")]
    pub enum ResourceState {
        Stopped,
        Allocating,
        StandBy,
        Starting,
        Running,
        Stopping,
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
                Self::Stopped => serializer.serialize_unit_variant("ResourceState", 0u32, "Stopped"),
                Self::Allocating => serializer.serialize_unit_variant("ResourceState", 1u32, "Allocating"),
                Self::StandBy => serializer.serialize_unit_variant("ResourceState", 2u32, "StandBy"),
                Self::Starting => serializer.serialize_unit_variant("ResourceState", 3u32, "Starting"),
                Self::Running => serializer.serialize_unit_variant("ResourceState", 4u32, "Running"),
                Self::Stopping => serializer.serialize_unit_variant("ResourceState", 5u32, "Stopping"),
                Self::Deleting => serializer.serialize_unit_variant("ResourceState", 6u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the transcription tracks in the output of a live event, generated using speech-to-text transcription. This property is reserved for future use, any value set on this property will be ignored."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveEventTranscription {
    #[doc = "Specifies the language (locale) to be used for speech-to-text transcription – it should match the spoken language in the audio track. The value should be in BCP-47 format (e.g: 'en-US'). See https://go.microsoft.com/fwlink/?linkid=2133742 for more information about the live transcription feature and the list of supported languages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[doc = "Provides a mechanism to select the audio track in the input live feed, to which speech-to-text transcription is applied. This property is reserved for future use, any value set on this property will be ignored."]
    #[serde(rename = "inputTrackSelection", default, skip_serializing_if = "Vec::is_empty")]
    pub input_track_selection: Vec<LiveEventInputTrackSelection>,
    #[doc = "Describes a transcription track in the output of a live event, generated using speech-to-text transcription. This property is reserved for future use, any value set on this property will be ignored."]
    #[serde(rename = "outputTranscriptionTrack", default, skip_serializing_if = "Option::is_none")]
    pub output_transcription_track: Option<LiveEventOutputTranscriptionTrack>,
}
impl LiveEventTranscription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Live Output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveOutput {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The JSON object that contains the properties required to create a live output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LiveOutputProperties>,
}
impl LiveOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The LiveOutput list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveOutputListResult {
    #[doc = "The result of the List LiveOutput operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LiveOutput>,
    #[doc = "The number of result."]
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[doc = "The link to the next set of results. Not empty if value contains incomplete list of live outputs."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for LiveOutputListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl LiveOutputListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON object that contains the properties required to create a live output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveOutputProperties {
    #[doc = "The description of the live output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The asset that the live output will write to."]
    #[serde(rename = "assetName")]
    pub asset_name: String,
    #[doc = "ISO 8601 time between 1 minute to 25 hours to indicate the maximum content length that can be archived in the asset for this live output. This also sets the maximum content length for the rewind window. For example, use PT1H30M to indicate 1 hour and 30 minutes of archive window."]
    #[serde(rename = "archiveWindowLength")]
    pub archive_window_length: String,
    #[doc = "The manifest file name. If not provided, the service will generate one automatically."]
    #[serde(rename = "manifestName", default, skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[doc = "HTTP Live Streaming (HLS) packing setting for the live output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hls: Option<Hls>,
    #[doc = "The initial timestamp that the live output will start at, any content before this value will not be archived."]
    #[serde(rename = "outputSnapTime", default, skip_serializing_if = "Option::is_none")]
    pub output_snap_time: Option<i64>,
    #[doc = "The creation time the live output."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The time the live output was last modified."]
    #[serde(rename = "lastModified", with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of the live output."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The resource state of the live output."]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<live_output_properties::ResourceState>,
}
impl LiveOutputProperties {
    pub fn new(asset_name: String, archive_window_length: String) -> Self {
        Self {
            description: None,
            asset_name,
            archive_window_length,
            manifest_name: None,
            hls: None,
            output_snap_time: None,
            created: None,
            last_modified: None,
            provisioning_state: None,
            resource_state: None,
        }
    }
}
pub mod live_output_properties {
    use super::*;
    #[doc = "The resource state of the live output."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceState")]
    pub enum ResourceState {
        Creating,
        Running,
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
                Self::Running => serializer.serialize_unit_variant("ResourceState", 1u32, "Running"),
                Self::Deleting => serializer.serialize_unit_variant("ResourceState", 2u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
}
impl Location {
    pub fn new(name: String) -> Self {
        Self { name }
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
#[doc = "The Media Filter properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaFilterProperties {
    #[doc = "The presentation time range, this is asset related and not recommended for Account Filter."]
    #[serde(rename = "presentationTimeRange", default, skip_serializing_if = "Option::is_none")]
    pub presentation_time_range: Option<PresentationTimeRange>,
    #[doc = "Filter First Quality"]
    #[serde(rename = "firstQuality", default, skip_serializing_if = "Option::is_none")]
    pub first_quality: Option<FirstQuality>,
    #[doc = "The tracks selection conditions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tracks: Vec<FilterTrackSelection>,
}
impl MediaFilterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Media Services account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaService {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of the Media Services account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MediaServiceProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<MediaServiceIdentity>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl MediaService {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
            system_data: None,
        }
    }
}
#[doc = "A collection of MediaService items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaServiceCollection {
    #[doc = "A collection of MediaService items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MediaService>,
    #[doc = "A link to the next page of the collection (when the collection contains too many results to return in one response)."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for MediaServiceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl MediaServiceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaServiceIdentity {
    #[doc = "The identity type."]
    #[serde(rename = "type")]
    pub type_: media_service_identity::Type,
    #[doc = "The Principal ID of the identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The Tenant ID of the identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl MediaServiceIdentity {
    pub fn new(type_: media_service_identity::Type) -> Self {
        Self {
            type_,
            principal_id: None,
            tenant_id: None,
        }
    }
}
pub mod media_service_identity {
    use super::*;
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        SystemAssigned,
        None,
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
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 0u32, "SystemAssigned"),
                Self::None => serializer.serialize_unit_variant("Type", 1u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the Media Services account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaServiceProperties {
    #[doc = "The Media Services account ID."]
    #[serde(rename = "mediaServiceId", default, skip_serializing_if = "Option::is_none")]
    pub media_service_id: Option<String>,
    #[doc = "The storage accounts for this resource."]
    #[serde(rename = "storageAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub storage_accounts: Vec<StorageAccount>,
    #[serde(rename = "storageAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub storage_authentication: Option<media_service_properties::StorageAuthentication>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<AccountEncryption>,
    #[serde(rename = "keyDelivery", default, skip_serializing_if = "Option::is_none")]
    pub key_delivery: Option<KeyDelivery>,
}
impl MediaServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod media_service_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageAuthentication")]
    pub enum StorageAuthentication {
        System,
        ManagedIdentity,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageAuthentication {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageAuthentication {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageAuthentication {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::System => serializer.serialize_unit_variant("StorageAuthentication", 0u32, "System"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("StorageAuthentication", 1u32, "ManagedIdentity"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A Media Services account update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaServiceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of the Media Services account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MediaServiceProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<MediaServiceIdentity>,
}
impl MediaServiceUpdate {
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
#[doc = "Describes the properties for an output ISO MP4 file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mp4Format {
    #[serde(flatten)]
    pub multi_bitrate_format: MultiBitrateFormat,
}
impl Mp4Format {
    pub fn new(multi_bitrate_format: MultiBitrateFormat) -> Self {
        Self { multi_bitrate_format }
    }
}
#[doc = "Describes the properties for producing a collection of GOP aligned multi-bitrate files. The default behavior is to produce one output file for each video layer which is muxed together with all the audios. The exact output files produced can be controlled by specifying the outputFiles collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiBitrateFormat {
    #[serde(flatten)]
    pub format: Format,
    #[doc = "The list of output files to produce.  Each entry in the list is a set of audio and video layer labels to be muxed together ."]
    #[serde(rename = "outputFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub output_files: Vec<OutputFile>,
}
impl MultiBitrateFormat {
    pub fn new(format: Format) -> Self {
        Self {
            format,
            output_files: Vec::new(),
        }
    }
}
#[doc = "Class for NoEncryption scheme"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NoEncryption {
    #[doc = "Class to specify which protocols are enabled"]
    #[serde(rename = "enabledProtocols", default, skip_serializing_if = "Option::is_none")]
    pub enabled_protocols: Option<EnabledProtocols>,
}
impl NoEncryption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about an error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ODataError {
    #[doc = "A language-independent error name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the error (for example, the name of the property in error)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ODataError>,
}
impl ODataError {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "The service specification property."]
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
    #[doc = "A link to the next page of the collection (when the collection contains too many results to return in one response)."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for OperationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
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
#[doc = "Represents an output file produced."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputFile {
    #[doc = "The list of labels that describe how the encoder should multiplex video and audio into an output file. For example, if the encoder is producing two video layers with labels v1 and v2, and one audio layer with label a1, then an array like '[v1, a1]' tells the encoder to produce an output file with the video track represented by v1 and the audio track represented by a1."]
    pub labels: Vec<String>,
}
impl OutputFile {
    pub fn new(labels: Vec<String>) -> Self {
        Self { labels }
    }
}
#[doc = "Base type for all overlays - image, audio or video."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Overlay {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    #[doc = "The label of the job input which is to be used as an overlay. The Input must specify exactly one file. You can specify an image file in JPG, PNG, GIF or BMP format, or an audio file (such as a WAV, MP3, WMA or M4A file), or a video file. See https://aka.ms/mesformats for the complete list of supported audio and video file formats."]
    #[serde(rename = "inputLabel")]
    pub input_label: String,
    #[doc = "The start position, with reference to the input video, at which the overlay starts. The value should be in ISO 8601 format. For example, PT05S to start the overlay at 5 seconds into the input video. If not specified the overlay starts from the beginning of the input video."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[doc = "The end position, with reference to the input video, at which the overlay ends. The value should be in ISO 8601 format. For example, PT30S to end the overlay at 30 seconds into the input video. If not specified or the value is greater than the input video duration, the overlay will be applied until the end of the input video if the overlay media duration is greater than the input video duration, else the overlay will last as long as the overlay media duration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[doc = "The duration over which the overlay fades in onto the input video. The value should be in ISO 8601 duration format. If not specified the default behavior is to have no fade in (same as PT0S)."]
    #[serde(rename = "fadeInDuration", default, skip_serializing_if = "Option::is_none")]
    pub fade_in_duration: Option<String>,
    #[doc = "The duration over which the overlay fades out of the input video. The value should be in ISO 8601 duration format. If not specified the default behavior is to have no fade out (same as PT0S)."]
    #[serde(rename = "fadeOutDuration", default, skip_serializing_if = "Option::is_none")]
    pub fade_out_duration: Option<String>,
    #[doc = "The gain level of audio in the overlay. The value should be in the range [0, 1.0]. The default is 1.0."]
    #[serde(rename = "audioGainLevel", default, skip_serializing_if = "Option::is_none")]
    pub audio_gain_level: Option<f64>,
}
impl Overlay {
    pub fn new(odata_type: String, input_label: String) -> Self {
        Self {
            odata_type,
            input_label,
            start: None,
            end: None,
            fade_in_duration: None,
            fade_out_duration: None,
            audio_gain_level: None,
        }
    }
}
#[doc = "Describes the settings for producing PNG thumbnails."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PngFormat {
    #[serde(flatten)]
    pub image_format: ImageFormat,
}
impl PngFormat {
    pub fn new(image_format: ImageFormat) -> Self {
        Self { image_format }
    }
}
#[doc = "Describes the properties for producing a series of PNG images from the input video."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PngImage {
    #[serde(flatten)]
    pub image: Image,
    #[doc = "A collection of output PNG image layers to be produced by the encoder."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<PngLayer>,
}
impl PngImage {
    pub fn new(image: Image) -> Self {
        Self { image, layers: Vec::new() }
    }
}
#[doc = "Describes the settings to produce a PNG image from the input video."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PngLayer {
    #[serde(flatten)]
    pub layer: Layer,
}
impl PngLayer {
    pub fn new(layer: Layer) -> Self {
        Self { layer }
    }
}
#[doc = "The presentation time range, this is asset related and not recommended for Account Filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PresentationTimeRange {
    #[doc = "The absolute start time boundary."]
    #[serde(rename = "startTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,
    #[doc = "The absolute end time boundary."]
    #[serde(rename = "endTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<i64>,
    #[doc = "The relative to end sliding window."]
    #[serde(rename = "presentationWindowDuration", default, skip_serializing_if = "Option::is_none")]
    pub presentation_window_duration: Option<i64>,
    #[doc = "The relative to end right edge."]
    #[serde(rename = "liveBackoffDuration", default, skip_serializing_if = "Option::is_none")]
    pub live_backoff_duration: Option<i64>,
    #[doc = "The time scale of time stamps."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timescale: Option<i64>,
    #[doc = "The indicator of forcing existing of end time stamp."]
    #[serde(rename = "forceEndTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub force_end_timestamp: Option<bool>,
}
impl PresentationTimeRange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base type for all Presets, which define the recipe or instructions on how the input media files should be processed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Preset {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
}
impl Preset {
    pub fn new(odata_type: String) -> Self {
        Self { odata_type }
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
#[doc = "The service specification property."]
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
#[doc = "A resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Provider {
    #[doc = "The provider name."]
    #[serde(rename = "providerName")]
    pub provider_name: String,
}
impl Provider {
    pub fn new(provider_name: String) -> Self {
        Self { provider_name }
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
#[doc = "Describes the properties of a rectangular window applied to the input media before processing it."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Rectangle {
    #[doc = "The number of pixels from the left-margin. This can be absolute pixel value (e.g 100), or relative to the size of the video (For example, 50%)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,
    #[doc = "The number of pixels from the top-margin. This can be absolute pixel value (e.g 100), or relative to the size of the video (For example, 50%)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<String>,
    #[doc = "The width of the rectangular region in pixels. This can be absolute pixel value (e.g 100), or relative to the size of the video (For example, 50%)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[doc = "The height of the rectangular region in pixels. This can be absolute pixel value (e.g 100), or relative to the size of the video (For example, 50%)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
}
impl Rectangle {
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
#[doc = "Select audio tracks from the input by specifying an attribute and an attribute filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectAudioTrackByAttribute {
    #[serde(flatten)]
    pub audio_track_descriptor: AudioTrackDescriptor,
    #[doc = "The TrackAttribute to filter the tracks by."]
    pub attribute: select_audio_track_by_attribute::Attribute,
    #[doc = "The type of AttributeFilter to apply to the TrackAttribute in order to select the tracks."]
    pub filter: select_audio_track_by_attribute::Filter,
    #[doc = "The value to filter the tracks by.  Only used when AttributeFilter.ValueEquals is specified for the Filter property."]
    #[serde(rename = "filterValue", default, skip_serializing_if = "Option::is_none")]
    pub filter_value: Option<String>,
}
impl SelectAudioTrackByAttribute {
    pub fn new(
        audio_track_descriptor: AudioTrackDescriptor,
        attribute: select_audio_track_by_attribute::Attribute,
        filter: select_audio_track_by_attribute::Filter,
    ) -> Self {
        Self {
            audio_track_descriptor,
            attribute,
            filter,
            filter_value: None,
        }
    }
}
pub mod select_audio_track_by_attribute {
    use super::*;
    #[doc = "The TrackAttribute to filter the tracks by."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Attribute")]
    pub enum Attribute {
        Bitrate,
        Language,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Attribute {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Attribute {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Attribute {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Bitrate => serializer.serialize_unit_variant("Attribute", 0u32, "Bitrate"),
                Self::Language => serializer.serialize_unit_variant("Attribute", 1u32, "Language"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of AttributeFilter to apply to the TrackAttribute in order to select the tracks."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Filter")]
    pub enum Filter {
        All,
        Top,
        Bottom,
        ValueEquals,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Filter {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Filter {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Filter {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::All => serializer.serialize_unit_variant("Filter", 0u32, "All"),
                Self::Top => serializer.serialize_unit_variant("Filter", 1u32, "Top"),
                Self::Bottom => serializer.serialize_unit_variant("Filter", 2u32, "Bottom"),
                Self::ValueEquals => serializer.serialize_unit_variant("Filter", 3u32, "ValueEquals"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Select audio tracks from the input by specifying a track identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectAudioTrackById {
    #[serde(flatten)]
    pub audio_track_descriptor: AudioTrackDescriptor,
    #[doc = "Track identifier to select"]
    #[serde(rename = "trackId")]
    pub track_id: i64,
}
impl SelectAudioTrackById {
    pub fn new(audio_track_descriptor: AudioTrackDescriptor, track_id: i64) -> Self {
        Self {
            audio_track_descriptor,
            track_id,
        }
    }
}
#[doc = "Select video tracks from the input by specifying an attribute and an attribute filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectVideoTrackByAttribute {
    #[serde(flatten)]
    pub video_track_descriptor: VideoTrackDescriptor,
    #[doc = "The TrackAttribute to filter the tracks by."]
    pub attribute: select_video_track_by_attribute::Attribute,
    #[doc = "The type of AttributeFilter to apply to the TrackAttribute in order to select the tracks."]
    pub filter: select_video_track_by_attribute::Filter,
    #[doc = "The value to filter the tracks by.  Only used when AttributeFilter.ValueEquals is specified for the Filter property. For TrackAttribute.Bitrate, this should be an integer value in bits per second (e.g: '1500000').  The TrackAttribute.Language is not supported for video tracks."]
    #[serde(rename = "filterValue", default, skip_serializing_if = "Option::is_none")]
    pub filter_value: Option<String>,
}
impl SelectVideoTrackByAttribute {
    pub fn new(
        video_track_descriptor: VideoTrackDescriptor,
        attribute: select_video_track_by_attribute::Attribute,
        filter: select_video_track_by_attribute::Filter,
    ) -> Self {
        Self {
            video_track_descriptor,
            attribute,
            filter,
            filter_value: None,
        }
    }
}
pub mod select_video_track_by_attribute {
    use super::*;
    #[doc = "The TrackAttribute to filter the tracks by."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Attribute")]
    pub enum Attribute {
        Bitrate,
        Language,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Attribute {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Attribute {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Attribute {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Bitrate => serializer.serialize_unit_variant("Attribute", 0u32, "Bitrate"),
                Self::Language => serializer.serialize_unit_variant("Attribute", 1u32, "Language"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of AttributeFilter to apply to the TrackAttribute in order to select the tracks."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Filter")]
    pub enum Filter {
        All,
        Top,
        Bottom,
        ValueEquals,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Filter {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Filter {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Filter {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::All => serializer.serialize_unit_variant("Filter", 0u32, "All"),
                Self::Top => serializer.serialize_unit_variant("Filter", 1u32, "Top"),
                Self::Bottom => serializer.serialize_unit_variant("Filter", 2u32, "Bottom"),
                Self::ValueEquals => serializer.serialize_unit_variant("Filter", 3u32, "ValueEquals"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Select video tracks from the input by specifying a track identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectVideoTrackById {
    #[serde(flatten)]
    pub video_track_descriptor: VideoTrackDescriptor,
    #[doc = "Track identifier to select"]
    #[serde(rename = "trackId")]
    pub track_id: i64,
}
impl SelectVideoTrackById {
    pub fn new(video_track_descriptor: VideoTrackDescriptor, track_id: i64) -> Self {
        Self {
            video_track_descriptor,
            track_id,
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
#[doc = "Describes all the settings to be used when encoding the input video with the Standard Encoder."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandardEncoderPreset {
    #[serde(flatten)]
    pub preset: Preset,
    #[doc = "Describes all the filtering operations, such as de-interlacing, rotation etc. that are to be applied to the input media before encoding."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filters>,
    #[doc = "The list of codecs to be used when encoding the input video."]
    pub codecs: Vec<Codec>,
    #[doc = "The list of outputs to be produced by the encoder."]
    pub formats: Vec<Format>,
}
impl StandardEncoderPreset {
    pub fn new(preset: Preset, codecs: Vec<Codec>, formats: Vec<Format>) -> Self {
        Self {
            preset,
            filters: None,
            codecs,
            formats,
        }
    }
}
#[doc = "The storage account details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccount {
    #[doc = "The ID of the storage account resource. Media Services relies on tables and queues as well as blobs, so the primary storage account must be a Standard Storage account (either Microsoft.ClassicStorage or Microsoft.Storage). Blob only storage accounts can be added as secondary storage accounts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the storage account."]
    #[serde(rename = "type")]
    pub type_: storage_account::Type,
}
impl StorageAccount {
    pub fn new(type_: storage_account::Type) -> Self {
        Self { id: None, type_ }
    }
}
pub mod storage_account {
    use super::*;
    #[doc = "The type of the storage account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Primary,
        Secondary,
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
                Self::Primary => serializer.serialize_unit_variant("Type", 0u32, "Primary"),
                Self::Secondary => serializer.serialize_unit_variant("Type", 1u32, "Secondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Data needed to decrypt asset files encrypted with legacy storage encryption."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageEncryptedAssetDecryptionData {
    #[doc = "The Asset File storage encryption key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "Asset File encryption metadata."]
    #[serde(rename = "assetFileEncryptionMetadata", default, skip_serializing_if = "Vec::is_empty")]
    pub asset_file_encryption_metadata: Vec<AssetFileEncryptionMetadata>,
}
impl StorageEncryptedAssetDecryptionData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The streaming endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamingEndpoint {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The streaming endpoint properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StreamingEndpointProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl StreamingEndpoint {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "Streaming endpoint access control definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamingEndpointAccessControl {
    #[doc = "Akamai access control"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub akamai: Option<AkamaiAccessControl>,
    #[doc = "The IP access control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<IpAccessControl>,
}
impl StreamingEndpointAccessControl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The streaming endpoint list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamingEndpointListResult {
    #[doc = "The result of the List StreamingEndpoint operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StreamingEndpoint>,
    #[doc = "The number of result."]
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[doc = "The link to the next set of results. Not empty if value contains incomplete list of streaming endpoints."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for StreamingEndpointListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl StreamingEndpointListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The streaming endpoint properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamingEndpointProperties {
    #[doc = "The streaming endpoint description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The number of scale units. Use the Scale operation to adjust this value."]
    #[serde(rename = "scaleUnits")]
    pub scale_units: i32,
    #[doc = "This feature is deprecated, do not set a value for this property."]
    #[serde(rename = "availabilitySetName", default, skip_serializing_if = "Option::is_none")]
    pub availability_set_name: Option<String>,
    #[doc = "Streaming endpoint access control definition."]
    #[serde(rename = "accessControl", default, skip_serializing_if = "Option::is_none")]
    pub access_control: Option<StreamingEndpointAccessControl>,
    #[doc = "Max cache age"]
    #[serde(rename = "maxCacheAge", default, skip_serializing_if = "Option::is_none")]
    pub max_cache_age: Option<i64>,
    #[doc = "The custom host names of the streaming endpoint"]
    #[serde(rename = "customHostNames", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_host_names: Vec<String>,
    #[doc = "The streaming endpoint host name."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "The CDN enabled flag."]
    #[serde(rename = "cdnEnabled", default, skip_serializing_if = "Option::is_none")]
    pub cdn_enabled: Option<bool>,
    #[doc = "The CDN provider name."]
    #[serde(rename = "cdnProvider", default, skip_serializing_if = "Option::is_none")]
    pub cdn_provider: Option<String>,
    #[doc = "The CDN profile name."]
    #[serde(rename = "cdnProfile", default, skip_serializing_if = "Option::is_none")]
    pub cdn_profile: Option<String>,
    #[doc = "The provisioning state of the streaming endpoint."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The resource state of the streaming endpoint."]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<streaming_endpoint_properties::ResourceState>,
    #[doc = "The client access policy."]
    #[serde(rename = "crossSiteAccessPolicies", default, skip_serializing_if = "Option::is_none")]
    pub cross_site_access_policies: Option<CrossSiteAccessPolicies>,
    #[doc = "The free trial expiration time."]
    #[serde(rename = "freeTrialEndTime", with = "azure_core::date::rfc3339::option")]
    pub free_trial_end_time: Option<time::OffsetDateTime>,
    #[doc = "The exact time the streaming endpoint was created."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The exact time the streaming endpoint was last modified."]
    #[serde(rename = "lastModified", with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
}
impl StreamingEndpointProperties {
    pub fn new(scale_units: i32) -> Self {
        Self {
            description: None,
            scale_units,
            availability_set_name: None,
            access_control: None,
            max_cache_age: None,
            custom_host_names: Vec::new(),
            host_name: None,
            cdn_enabled: None,
            cdn_provider: None,
            cdn_profile: None,
            provisioning_state: None,
            resource_state: None,
            cross_site_access_policies: None,
            free_trial_end_time: None,
            created: None,
            last_modified: None,
        }
    }
}
pub mod streaming_endpoint_properties {
    use super::*;
    #[doc = "The resource state of the streaming endpoint."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceState")]
    pub enum ResourceState {
        Stopped,
        Starting,
        Running,
        Stopping,
        Deleting,
        Scaling,
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
                Self::Stopped => serializer.serialize_unit_variant("ResourceState", 0u32, "Stopped"),
                Self::Starting => serializer.serialize_unit_variant("ResourceState", 1u32, "Starting"),
                Self::Running => serializer.serialize_unit_variant("ResourceState", 2u32, "Running"),
                Self::Stopping => serializer.serialize_unit_variant("ResourceState", 3u32, "Stopping"),
                Self::Deleting => serializer.serialize_unit_variant("ResourceState", 4u32, "Deleting"),
                Self::Scaling => serializer.serialize_unit_variant("ResourceState", 5u32, "Scaling"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "scale units definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamingEntityScaleUnit {
    #[doc = "The scale unit number of the streaming endpoint."]
    #[serde(rename = "scaleUnit", default, skip_serializing_if = "Option::is_none")]
    pub scale_unit: Option<i32>,
}
impl StreamingEntityScaleUnit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Streaming Locator resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamingLocator {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the Streaming Locator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StreamingLocatorProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl StreamingLocator {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of StreamingLocator items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamingLocatorCollection {
    #[doc = "A collection of StreamingLocator items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StreamingLocator>,
    #[doc = "A link to the next page of the collection (when the collection contains too many results to return in one response)."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for StreamingLocatorCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl StreamingLocatorCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for content key in Streaming Locator"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamingLocatorContentKey {
    #[doc = "ID of Content Key"]
    pub id: String,
    #[doc = "Encryption type of Content Key"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<streaming_locator_content_key::Type>,
    #[doc = "Label of Content Key as specified in the Streaming Policy"]
    #[serde(rename = "labelReferenceInStreamingPolicy", default, skip_serializing_if = "Option::is_none")]
    pub label_reference_in_streaming_policy: Option<String>,
    #[doc = "Value of Content Key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "ContentKeyPolicy used by Content Key"]
    #[serde(rename = "policyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[doc = "Tracks which use this Content Key"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tracks: Vec<TrackSelection>,
}
impl StreamingLocatorContentKey {
    pub fn new(id: String) -> Self {
        Self {
            id,
            type_: None,
            label_reference_in_streaming_policy: None,
            value: None,
            policy_name: None,
            tracks: Vec::new(),
        }
    }
}
pub mod streaming_locator_content_key {
    use super::*;
    #[doc = "Encryption type of Content Key"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        CommonEncryptionCenc,
        CommonEncryptionCbcs,
        EnvelopeEncryption,
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
                Self::CommonEncryptionCenc => serializer.serialize_unit_variant("Type", 0u32, "CommonEncryptionCenc"),
                Self::CommonEncryptionCbcs => serializer.serialize_unit_variant("Type", 1u32, "CommonEncryptionCbcs"),
                Self::EnvelopeEncryption => serializer.serialize_unit_variant("Type", 2u32, "EnvelopeEncryption"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the Streaming Locator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamingLocatorProperties {
    #[doc = "Asset Name"]
    #[serde(rename = "assetName")]
    pub asset_name: String,
    #[doc = "The creation time of the Streaming Locator."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The start time of the Streaming Locator."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the Streaming Locator."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The StreamingLocatorId of the Streaming Locator."]
    #[serde(rename = "streamingLocatorId", default, skip_serializing_if = "Option::is_none")]
    pub streaming_locator_id: Option<String>,
    #[doc = "Name of the Streaming Policy used by this Streaming Locator. Either specify the name of Streaming Policy you created or use one of the predefined Streaming Policies. The predefined Streaming Policies available are: 'Predefined_DownloadOnly', 'Predefined_ClearStreamingOnly', 'Predefined_DownloadAndClearStreaming', 'Predefined_ClearKey', 'Predefined_MultiDrmCencStreaming' and 'Predefined_MultiDrmStreaming'"]
    #[serde(rename = "streamingPolicyName")]
    pub streaming_policy_name: String,
    #[doc = "Name of the default ContentKeyPolicy used by this Streaming Locator."]
    #[serde(rename = "defaultContentKeyPolicyName", default, skip_serializing_if = "Option::is_none")]
    pub default_content_key_policy_name: Option<String>,
    #[doc = "The ContentKeys used by this Streaming Locator."]
    #[serde(rename = "contentKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub content_keys: Vec<StreamingLocatorContentKey>,
    #[doc = "Alternative Media ID of this Streaming Locator"]
    #[serde(rename = "alternativeMediaId", default, skip_serializing_if = "Option::is_none")]
    pub alternative_media_id: Option<String>,
    #[doc = "A list of asset or account filters which apply to this streaming locator"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<String>,
}
impl StreamingLocatorProperties {
    pub fn new(asset_name: String, streaming_policy_name: String) -> Self {
        Self {
            asset_name,
            created: None,
            start_time: None,
            end_time: None,
            streaming_locator_id: None,
            streaming_policy_name,
            default_content_key_policy_name: None,
            content_keys: Vec::new(),
            alternative_media_id: None,
            filters: Vec::new(),
        }
    }
}
#[doc = "Class of paths for streaming"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamingPath {
    #[doc = "Streaming protocol"]
    #[serde(rename = "streamingProtocol")]
    pub streaming_protocol: streaming_path::StreamingProtocol,
    #[doc = "Encryption scheme"]
    #[serde(rename = "encryptionScheme")]
    pub encryption_scheme: streaming_path::EncryptionScheme,
    #[doc = "Streaming paths for each protocol and encryptionScheme pair"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<String>,
}
impl StreamingPath {
    pub fn new(streaming_protocol: streaming_path::StreamingProtocol, encryption_scheme: streaming_path::EncryptionScheme) -> Self {
        Self {
            streaming_protocol,
            encryption_scheme,
            paths: Vec::new(),
        }
    }
}
pub mod streaming_path {
    use super::*;
    #[doc = "Streaming protocol"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StreamingProtocol")]
    pub enum StreamingProtocol {
        Hls,
        Dash,
        SmoothStreaming,
        Download,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StreamingProtocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StreamingProtocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StreamingProtocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Hls => serializer.serialize_unit_variant("StreamingProtocol", 0u32, "Hls"),
                Self::Dash => serializer.serialize_unit_variant("StreamingProtocol", 1u32, "Dash"),
                Self::SmoothStreaming => serializer.serialize_unit_variant("StreamingProtocol", 2u32, "SmoothStreaming"),
                Self::Download => serializer.serialize_unit_variant("StreamingProtocol", 3u32, "Download"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Encryption scheme"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EncryptionScheme")]
    pub enum EncryptionScheme {
        NoEncryption,
        EnvelopeEncryption,
        CommonEncryptionCenc,
        CommonEncryptionCbcs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EncryptionScheme {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EncryptionScheme {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EncryptionScheme {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NoEncryption => serializer.serialize_unit_variant("EncryptionScheme", 0u32, "NoEncryption"),
                Self::EnvelopeEncryption => serializer.serialize_unit_variant("EncryptionScheme", 1u32, "EnvelopeEncryption"),
                Self::CommonEncryptionCenc => serializer.serialize_unit_variant("EncryptionScheme", 2u32, "CommonEncryptionCenc"),
                Self::CommonEncryptionCbcs => serializer.serialize_unit_variant("EncryptionScheme", 3u32, "CommonEncryptionCbcs"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A Streaming Policy resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamingPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class to specify properties of Streaming Policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StreamingPolicyProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl StreamingPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of StreamingPolicy items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamingPolicyCollection {
    #[doc = "A collection of StreamingPolicy items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StreamingPolicy>,
    #[doc = "A link to the next page of the collection (when the collection contains too many results to return in one response)."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for StreamingPolicyCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl StreamingPolicyCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to specify properties of content key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamingPolicyContentKey {
    #[doc = "Label can be used to specify Content Key when creating a Streaming Locator"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Policy used by Content Key"]
    #[serde(rename = "policyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[doc = "Tracks which use this content key"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tracks: Vec<TrackSelection>,
}
impl StreamingPolicyContentKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to specify properties of all content keys in Streaming Policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamingPolicyContentKeys {
    #[doc = "Class to specify properties of default content key for each encryption scheme"]
    #[serde(rename = "defaultKey", default, skip_serializing_if = "Option::is_none")]
    pub default_key: Option<DefaultKey>,
    #[doc = "Representing tracks needs separate content key"]
    #[serde(rename = "keyToTrackMappings", default, skip_serializing_if = "Vec::is_empty")]
    pub key_to_track_mappings: Vec<StreamingPolicyContentKey>,
}
impl StreamingPolicyContentKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to specify configurations of FairPlay in Streaming Policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamingPolicyFairPlayConfiguration {
    #[doc = "Template for the URL of the custom service delivering licenses to end user players.  Not required when using Azure Media Services for issuing licenses.  The template supports replaceable tokens that the service will update at runtime with the value specific to the request.  The currently supported token values are {AlternativeMediaId}, which is replaced with the value of StreamingLocatorId.AlternativeMediaId, and {ContentKeyId}, which is replaced with the value of identifier of the key being requested."]
    #[serde(rename = "customLicenseAcquisitionUrlTemplate", default, skip_serializing_if = "Option::is_none")]
    pub custom_license_acquisition_url_template: Option<String>,
    #[doc = "All license to be persistent or not"]
    #[serde(rename = "allowPersistentLicense")]
    pub allow_persistent_license: bool,
}
impl StreamingPolicyFairPlayConfiguration {
    pub fn new(allow_persistent_license: bool) -> Self {
        Self {
            custom_license_acquisition_url_template: None,
            allow_persistent_license,
        }
    }
}
#[doc = "Class to specify configurations of PlayReady in Streaming Policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamingPolicyPlayReadyConfiguration {
    #[doc = "Template for the URL of the custom service delivering licenses to end user players.  Not required when using Azure Media Services for issuing licenses.  The template supports replaceable tokens that the service will update at runtime with the value specific to the request.  The currently supported token values are {AlternativeMediaId}, which is replaced with the value of StreamingLocatorId.AlternativeMediaId, and {ContentKeyId}, which is replaced with the value of identifier of the key being requested."]
    #[serde(rename = "customLicenseAcquisitionUrlTemplate", default, skip_serializing_if = "Option::is_none")]
    pub custom_license_acquisition_url_template: Option<String>,
    #[doc = "Custom attributes for PlayReady"]
    #[serde(rename = "playReadyCustomAttributes", default, skip_serializing_if = "Option::is_none")]
    pub play_ready_custom_attributes: Option<String>,
}
impl StreamingPolicyPlayReadyConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to specify properties of Streaming Policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamingPolicyProperties {
    #[doc = "Creation time of Streaming Policy"]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "Default ContentKey used by current Streaming Policy"]
    #[serde(rename = "defaultContentKeyPolicyName", default, skip_serializing_if = "Option::is_none")]
    pub default_content_key_policy_name: Option<String>,
    #[doc = "Class for EnvelopeEncryption encryption scheme"]
    #[serde(rename = "envelopeEncryption", default, skip_serializing_if = "Option::is_none")]
    pub envelope_encryption: Option<EnvelopeEncryption>,
    #[doc = "Class for envelope encryption scheme"]
    #[serde(rename = "commonEncryptionCenc", default, skip_serializing_if = "Option::is_none")]
    pub common_encryption_cenc: Option<CommonEncryptionCenc>,
    #[doc = "Class for CommonEncryptionCbcs encryption scheme"]
    #[serde(rename = "commonEncryptionCbcs", default, skip_serializing_if = "Option::is_none")]
    pub common_encryption_cbcs: Option<CommonEncryptionCbcs>,
    #[doc = "Class for NoEncryption scheme"]
    #[serde(rename = "noEncryption", default, skip_serializing_if = "Option::is_none")]
    pub no_encryption: Option<NoEncryption>,
}
impl StreamingPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to specify configurations of Widevine in Streaming Policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamingPolicyWidevineConfiguration {
    #[doc = "Template for the URL of the custom service delivering licenses to end user players.  Not required when using Azure Media Services for issuing licenses.  The template supports replaceable tokens that the service will update at runtime with the value specific to the request.  The currently supported token values are {AlternativeMediaId}, which is replaced with the value of StreamingLocatorId.AlternativeMediaId, and {ContentKeyId}, which is replaced with the value of identifier of the key being requested."]
    #[serde(rename = "customLicenseAcquisitionUrlTemplate", default, skip_serializing_if = "Option::is_none")]
    pub custom_license_acquisition_url_template: Option<String>,
}
impl StreamingPolicyWidevineConfiguration {
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
#[doc = "Base type for all TrackDescriptor types, which define the metadata and selection for tracks that should be processed by a Job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackDescriptor {
    #[doc = "The discriminator for derived types."]
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
}
impl TrackDescriptor {
    pub fn new(odata_type: String) -> Self {
        Self { odata_type }
    }
}
#[doc = "Class to specify one track property condition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackPropertyCondition {
    #[doc = "Track property type"]
    pub property: track_property_condition::Property,
    #[doc = "Track property condition operation"]
    pub operation: track_property_condition::Operation,
    #[doc = "Track property value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl TrackPropertyCondition {
    pub fn new(property: track_property_condition::Property, operation: track_property_condition::Operation) -> Self {
        Self {
            property,
            operation,
            value: None,
        }
    }
}
pub mod track_property_condition {
    use super::*;
    #[doc = "Track property type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Property")]
    pub enum Property {
        Unknown,
        #[serde(rename = "FourCC")]
        FourCc,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Property {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Property {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Property {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("Property", 0u32, "Unknown"),
                Self::FourCc => serializer.serialize_unit_variant("Property", 1u32, "FourCC"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Track property condition operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operation")]
    pub enum Operation {
        Unknown,
        Equal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("Operation", 0u32, "Unknown"),
                Self::Equal => serializer.serialize_unit_variant("Operation", 1u32, "Equal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Class to select a track"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackSelection {
    #[doc = "TrackSelections is a track property condition list which can specify track(s)"]
    #[serde(rename = "trackSelections", default, skip_serializing_if = "Vec::is_empty")]
    pub track_selections: Vec<TrackPropertyCondition>,
}
impl TrackSelection {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "A Transform encapsulates the rules or instructions for generating desired outputs from input media, such as by transcoding or by extracting insights. After the Transform is created, it can be applied to input media by creating Jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Transform {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "A Transform."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TransformProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Transform {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of Transform items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransformCollection {
    #[doc = "A collection of Transform items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Transform>,
    #[doc = "A link to the next page of the collection (when the collection contains too many results to return in one response)."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for TransformCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl TransformCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a TransformOutput, which are the rules to be applied while generating the desired output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransformOutput {
    #[doc = "A Transform can define more than one outputs. This property defines what the service should do when one output fails - either continue to produce other outputs, or, stop the other outputs. The overall Job state will not reflect failures of outputs that are specified with 'ContinueJob'. The default is 'StopProcessingJob'."]
    #[serde(rename = "onError", default, skip_serializing_if = "Option::is_none")]
    pub on_error: Option<transform_output::OnError>,
    #[doc = "Sets the relative priority of the TransformOutputs within a Transform. This sets the priority that the service uses for processing TransformOutputs. The default priority is Normal."]
    #[serde(rename = "relativePriority", default, skip_serializing_if = "Option::is_none")]
    pub relative_priority: Option<transform_output::RelativePriority>,
    #[doc = "Base type for all Presets, which define the recipe or instructions on how the input media files should be processed."]
    pub preset: Preset,
}
impl TransformOutput {
    pub fn new(preset: Preset) -> Self {
        Self {
            on_error: None,
            relative_priority: None,
            preset,
        }
    }
}
pub mod transform_output {
    use super::*;
    #[doc = "A Transform can define more than one outputs. This property defines what the service should do when one output fails - either continue to produce other outputs, or, stop the other outputs. The overall Job state will not reflect failures of outputs that are specified with 'ContinueJob'. The default is 'StopProcessingJob'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OnError")]
    pub enum OnError {
        StopProcessingJob,
        ContinueJob,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OnError {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OnError {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OnError {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StopProcessingJob => serializer.serialize_unit_variant("OnError", 0u32, "StopProcessingJob"),
                Self::ContinueJob => serializer.serialize_unit_variant("OnError", 1u32, "ContinueJob"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Sets the relative priority of the TransformOutputs within a Transform. This sets the priority that the service uses for processing TransformOutputs. The default priority is Normal."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RelativePriority")]
    pub enum RelativePriority {
        Low,
        Normal,
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RelativePriority {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RelativePriority {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RelativePriority {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Low => serializer.serialize_unit_variant("RelativePriority", 0u32, "Low"),
                Self::Normal => serializer.serialize_unit_variant("RelativePriority", 1u32, "Normal"),
                Self::High => serializer.serialize_unit_variant("RelativePriority", 2u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A Transform."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransformProperties {
    #[doc = "The UTC date and time when the Transform was created, in 'YYYY-MM-DDThh:mm:ssZ' format."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "An optional verbose description of the Transform."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The UTC date and time when the Transform was last updated, in 'YYYY-MM-DDThh:mm:ssZ' format."]
    #[serde(rename = "lastModified", with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
    #[doc = "An array of one or more TransformOutputs that the Transform should generate."]
    pub outputs: Vec<TransformOutput>,
}
impl TransformProperties {
    pub fn new(outputs: Vec<TransformOutput>) -> Self {
        Self {
            created: None,
            description: None,
            last_modified: None,
            outputs,
        }
    }
}
#[doc = "Describes the properties for generating an MPEG-2 Transport Stream (ISO/IEC 13818-1) output video file(s)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransportStreamFormat {
    #[serde(flatten)]
    pub multi_bitrate_format: MultiBitrateFormat,
}
impl TransportStreamFormat {
    pub fn new(multi_bitrate_format: MultiBitrateFormat) -> Self {
        Self { multi_bitrate_format }
    }
}
#[doc = "Specifies the clip time as a Utc time position in the media file.  The Utc time can point to a different position depending on whether the media file starts from a timestamp of zero or not."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UtcClipTime {
    #[serde(flatten)]
    pub clip_time: ClipTime,
    #[doc = "The time position on the timeline of the input media based on Utc time."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub time: time::OffsetDateTime,
}
impl UtcClipTime {
    pub fn new(clip_time: ClipTime, time: time::OffsetDateTime) -> Self {
        Self { clip_time, time }
    }
}
#[doc = "Describes the basic properties for encoding the input video."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Video {
    #[serde(flatten)]
    pub codec: Codec,
    #[doc = "The distance between two key frames. The value should be non-zero in the range [0.5, 20] seconds, specified in ISO 8601 format. The default is 2 seconds(PT2S). Note that this setting is ignored if VideoSyncMode.Passthrough is set, where the KeyFrameInterval value will follow the input source setting."]
    #[serde(rename = "keyFrameInterval", default, skip_serializing_if = "Option::is_none")]
    pub key_frame_interval: Option<String>,
    #[doc = "The resizing mode - how the input video will be resized to fit the desired output resolution(s). Default is AutoSize"]
    #[serde(rename = "stretchMode", default, skip_serializing_if = "Option::is_none")]
    pub stretch_mode: Option<video::StretchMode>,
    #[doc = "The Video Sync Mode"]
    #[serde(rename = "syncMode", default, skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<video::SyncMode>,
}
impl Video {
    pub fn new(codec: Codec) -> Self {
        Self {
            codec,
            key_frame_interval: None,
            stretch_mode: None,
            sync_mode: None,
        }
    }
}
pub mod video {
    use super::*;
    #[doc = "The resizing mode - how the input video will be resized to fit the desired output resolution(s). Default is AutoSize"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StretchMode")]
    pub enum StretchMode {
        None,
        AutoSize,
        AutoFit,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StretchMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StretchMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StretchMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("StretchMode", 0u32, "None"),
                Self::AutoSize => serializer.serialize_unit_variant("StretchMode", 1u32, "AutoSize"),
                Self::AutoFit => serializer.serialize_unit_variant("StretchMode", 2u32, "AutoFit"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The Video Sync Mode"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SyncMode")]
    pub enum SyncMode {
        Auto,
        Passthrough,
        Cfr,
        Vfr,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SyncMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SyncMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SyncMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Auto => serializer.serialize_unit_variant("SyncMode", 0u32, "Auto"),
                Self::Passthrough => serializer.serialize_unit_variant("SyncMode", 1u32, "Passthrough"),
                Self::Cfr => serializer.serialize_unit_variant("SyncMode", 2u32, "Cfr"),
                Self::Vfr => serializer.serialize_unit_variant("SyncMode", 3u32, "Vfr"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A video analyzer preset that extracts insights (rich metadata) from both audio and video, and outputs a JSON format file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoAnalyzerPreset {
    #[serde(flatten)]
    pub audio_analyzer_preset: AudioAnalyzerPreset,
    #[doc = "Defines the type of insights that you want the service to generate. The allowed values are 'AudioInsightsOnly', 'VideoInsightsOnly', and 'AllInsights'. The default is AllInsights. If you set this to AllInsights and the input is audio only, then only audio insights are generated. Similarly if the input is video only, then only video insights are generated. It is recommended that you not use AudioInsightsOnly if you expect some of your inputs to be video only; or use VideoInsightsOnly if you expect some of your inputs to be audio only. Your Jobs in such conditions would error out."]
    #[serde(rename = "insightsToExtract", default, skip_serializing_if = "Option::is_none")]
    pub insights_to_extract: Option<video_analyzer_preset::InsightsToExtract>,
}
impl VideoAnalyzerPreset {
    pub fn new(audio_analyzer_preset: AudioAnalyzerPreset) -> Self {
        Self {
            audio_analyzer_preset,
            insights_to_extract: None,
        }
    }
}
pub mod video_analyzer_preset {
    use super::*;
    #[doc = "Defines the type of insights that you want the service to generate. The allowed values are 'AudioInsightsOnly', 'VideoInsightsOnly', and 'AllInsights'. The default is AllInsights. If you set this to AllInsights and the input is audio only, then only audio insights are generated. Similarly if the input is video only, then only video insights are generated. It is recommended that you not use AudioInsightsOnly if you expect some of your inputs to be video only; or use VideoInsightsOnly if you expect some of your inputs to be audio only. Your Jobs in such conditions would error out."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InsightsToExtract")]
    pub enum InsightsToExtract {
        AudioInsightsOnly,
        VideoInsightsOnly,
        AllInsights,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InsightsToExtract {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InsightsToExtract {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InsightsToExtract {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AudioInsightsOnly => serializer.serialize_unit_variant("InsightsToExtract", 0u32, "AudioInsightsOnly"),
                Self::VideoInsightsOnly => serializer.serialize_unit_variant("InsightsToExtract", 1u32, "VideoInsightsOnly"),
                Self::AllInsights => serializer.serialize_unit_variant("InsightsToExtract", 2u32, "AllInsights"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the settings to be used when encoding the input video into a desired output bitrate layer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoLayer {
    #[serde(flatten)]
    pub layer: Layer,
    #[doc = "The average bitrate in bits per second at which to encode the input video when generating this layer. This is a required field."]
    pub bitrate: i32,
    #[doc = "The maximum bitrate (in bits per second), at which the VBV buffer should be assumed to refill. If not specified, defaults to the same value as bitrate."]
    #[serde(rename = "maxBitrate", default, skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i32>,
    #[doc = "The number of B-frames to be used when encoding this layer.  If not specified, the encoder chooses an appropriate number based on the video profile and level."]
    #[serde(rename = "bFrames", default, skip_serializing_if = "Option::is_none")]
    pub b_frames: Option<i32>,
    #[doc = "The frame rate (in frames per second) at which to encode this layer. The value can be in the form of M/N where M and N are integers (For example, 30000/1001), or in the form of a number (For example, 30, or 29.97). The encoder enforces constraints on allowed frame rates based on the profile and level. If it is not specified, the encoder will use the same frame rate as the input video."]
    #[serde(rename = "frameRate", default, skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<String>,
    #[doc = "The number of slices to be used when encoding this layer. If not specified, default is zero, which means that encoder will use a single slice for each frame."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slices: Option<i32>,
    #[doc = "Whether or not adaptive B-frames are to be used when encoding this layer. If not specified, the encoder will turn it on whenever the video profile permits its use."]
    #[serde(rename = "adaptiveBFrame", default, skip_serializing_if = "Option::is_none")]
    pub adaptive_b_frame: Option<bool>,
}
impl VideoLayer {
    pub fn new(layer: Layer, bitrate: i32) -> Self {
        Self {
            layer,
            bitrate,
            max_bitrate: None,
            b_frames: None,
            frame_rate: None,
            slices: None,
            adaptive_b_frame: None,
        }
    }
}
#[doc = "Describes the properties of a video overlay."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoOverlay {
    #[serde(flatten)]
    pub overlay: Overlay,
    #[doc = "Describes the properties of a rectangular window applied to the input media before processing it."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Rectangle>,
    #[doc = "The opacity of the overlay. This is a value in the range [0 - 1.0]. Default is 1.0 which mean the overlay is opaque."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    #[doc = "Describes the properties of a rectangular window applied to the input media before processing it."]
    #[serde(rename = "cropRectangle", default, skip_serializing_if = "Option::is_none")]
    pub crop_rectangle: Option<Rectangle>,
}
impl VideoOverlay {
    pub fn new(overlay: Overlay) -> Self {
        Self {
            overlay,
            position: None,
            opacity: None,
            crop_rectangle: None,
        }
    }
}
#[doc = "A TrackSelection to select video tracks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoTrackDescriptor {
    #[serde(flatten)]
    pub track_descriptor: TrackDescriptor,
}
impl VideoTrackDescriptor {
    pub fn new(track_descriptor: TrackDescriptor) -> Self {
        Self { track_descriptor }
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
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
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

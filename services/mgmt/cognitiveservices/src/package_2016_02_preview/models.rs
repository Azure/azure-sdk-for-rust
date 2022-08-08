#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Cognitive Services Account is an Azure resource representing the provisioned account, its type, location and SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CognitiveServicesAccount {
    #[doc = "Entity Tag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The id of the created account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Type of cognitive service account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "The location of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The name of the created account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CognitiveServicesAccountProperties>,
    #[doc = "The SKU of the cognitive services account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Gets or sets a list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key no greater than 128 characters and value no greater than 256 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CognitiveServicesAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters to provide for the account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CognitiveServicesAccountCreateParameters {
    #[doc = "The SKU of the cognitive services account."]
    pub sku: Sku,
    #[doc = "Required. Indicates the type of cognitive service account."]
    pub kind: cognitive_services_account_create_parameters::Kind,
    #[doc = "Required. Gets or sets the location of the resource. This will be one of the supported and registered Azure Geo Regions (e.g. West US, East US, Southeast Asia, etc.). The geo region of a resource cannot be changed once it is created, but if an identical geo region is specified on update the request will succeed."]
    pub location: String,
    #[doc = "Gets or sets a list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key no greater than 128 characters and value no greater than 256 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "required empty properties object. Must be an empty object, and must exist in the request."]
    pub properties: CognitiveServicesAccountPropertiesCreateParameters,
}
impl CognitiveServicesAccountCreateParameters {
    pub fn new(
        sku: Sku,
        kind: cognitive_services_account_create_parameters::Kind,
        location: String,
        properties: CognitiveServicesAccountPropertiesCreateParameters,
    ) -> Self {
        Self {
            sku,
            kind,
            location,
            tags: None,
            properties,
        }
    }
}
pub mod cognitive_services_account_create_parameters {
    use super::*;
    #[doc = "Required. Indicates the type of cognitive service account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Academic,
        #[serde(rename = "Bing.Autosuggest")]
        BingAutosuggest,
        #[serde(rename = "Bing.Search")]
        BingSearch,
        #[serde(rename = "Bing.Speech")]
        BingSpeech,
        #[serde(rename = "Bing.SpellCheck")]
        BingSpellCheck,
        ComputerVision,
        ContentModerator,
        Emotion,
        Face,
        #[serde(rename = "LUIS")]
        Luis,
        Recommendations,
        SpeakerRecognition,
        Speech,
        SpeechTranslation,
        TextAnalytics,
        TextTranslation,
        #[serde(rename = "WebLM")]
        WebLm,
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
                Self::Academic => serializer.serialize_unit_variant("Kind", 0u32, "Academic"),
                Self::BingAutosuggest => serializer.serialize_unit_variant("Kind", 1u32, "Bing.Autosuggest"),
                Self::BingSearch => serializer.serialize_unit_variant("Kind", 2u32, "Bing.Search"),
                Self::BingSpeech => serializer.serialize_unit_variant("Kind", 3u32, "Bing.Speech"),
                Self::BingSpellCheck => serializer.serialize_unit_variant("Kind", 4u32, "Bing.SpellCheck"),
                Self::ComputerVision => serializer.serialize_unit_variant("Kind", 5u32, "ComputerVision"),
                Self::ContentModerator => serializer.serialize_unit_variant("Kind", 6u32, "ContentModerator"),
                Self::Emotion => serializer.serialize_unit_variant("Kind", 7u32, "Emotion"),
                Self::Face => serializer.serialize_unit_variant("Kind", 8u32, "Face"),
                Self::Luis => serializer.serialize_unit_variant("Kind", 9u32, "LUIS"),
                Self::Recommendations => serializer.serialize_unit_variant("Kind", 10u32, "Recommendations"),
                Self::SpeakerRecognition => serializer.serialize_unit_variant("Kind", 11u32, "SpeakerRecognition"),
                Self::Speech => serializer.serialize_unit_variant("Kind", 12u32, "Speech"),
                Self::SpeechTranslation => serializer.serialize_unit_variant("Kind", 13u32, "SpeechTranslation"),
                Self::TextAnalytics => serializer.serialize_unit_variant("Kind", 14u32, "TextAnalytics"),
                Self::TextTranslation => serializer.serialize_unit_variant("Kind", 15u32, "TextTranslation"),
                Self::WebLm => serializer.serialize_unit_variant("Kind", 16u32, "WebLM"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The list of cognitive services accounts operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CognitiveServicesAccountEnumerateSkusResult {
    #[doc = "Gets the list of Cognitive Services accounts and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CognitiveServicesResourceAndSku>,
}
impl CognitiveServicesAccountEnumerateSkusResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The access keys for the cognitive services account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CognitiveServicesAccountKeys {
    #[doc = "Gets the value of key 1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key1: Option<String>,
    #[doc = "Gets the value of key 2."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
}
impl CognitiveServicesAccountKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of cognitive services accounts operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CognitiveServicesAccountListResult {
    #[doc = "Gets the list of Cognitive Services accounts and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CognitiveServicesAccount>,
}
impl azure_core::Continuable for CognitiveServicesAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CognitiveServicesAccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CognitiveServicesAccountProperties {
    #[doc = "Gets the status of the cognitive services account at the time the operation was called."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<cognitive_services_account_properties::ProvisioningState>,
    #[doc = "Endpoint of the created account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl CognitiveServicesAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cognitive_services_account_properties {
    use super::*;
    #[doc = "Gets the status of the cognitive services account at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        #[serde(rename = "ResolvingDNS")]
        ResolvingDns,
        Succeeded,
        Failed,
    }
}
#[doc = "required empty properties object. Must be an empty object, and must exist in the request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CognitiveServicesAccountPropertiesCreateParameters {}
impl CognitiveServicesAccountPropertiesCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters to provide for the account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CognitiveServicesAccountUpdateParameters {
    #[doc = "The SKU of the cognitive services account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Gets or sets a list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key no greater than 128 characters and value no greater than 256 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CognitiveServicesAccountUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CognitiveServicesResourceAndSku {
    #[doc = "Resource Namespace and Type"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The SKU of the cognitive services account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl CognitiveServicesResourceAndSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorBody>,
}
impl azure_core::Continuable for Error {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorBody {
    #[doc = "error code"]
    pub code: String,
    #[doc = "error message"]
    pub message: String,
}
impl ErrorBody {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
#[doc = "Regenerate key parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegenerateKeyParameters {
    #[doc = "key name to generate (Key1|Key2)"]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<regenerate_key_parameters::KeyName>,
}
impl RegenerateKeyParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod regenerate_key_parameters {
    use super::*;
    #[doc = "key name to generate (Key1|Key2)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyName {
        Key1,
        Key2,
    }
}
#[doc = "The SKU of the cognitive services account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "Gets or sets the sku name. Required for account creation, optional for update."]
    pub name: sku::Name,
    #[doc = "Gets the sku tier. This is based on the SKU name."]
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
    #[doc = "Gets or sets the sku name. Required for account creation, optional for update."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        F0,
        P0,
        P1,
        P2,
        S0,
        S1,
        S2,
        S3,
        S4,
        S5,
        S6,
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
                Self::F0 => serializer.serialize_unit_variant("Name", 0u32, "F0"),
                Self::P0 => serializer.serialize_unit_variant("Name", 1u32, "P0"),
                Self::P1 => serializer.serialize_unit_variant("Name", 2u32, "P1"),
                Self::P2 => serializer.serialize_unit_variant("Name", 3u32, "P2"),
                Self::S0 => serializer.serialize_unit_variant("Name", 4u32, "S0"),
                Self::S1 => serializer.serialize_unit_variant("Name", 5u32, "S1"),
                Self::S2 => serializer.serialize_unit_variant("Name", 6u32, "S2"),
                Self::S3 => serializer.serialize_unit_variant("Name", 7u32, "S3"),
                Self::S4 => serializer.serialize_unit_variant("Name", 8u32, "S4"),
                Self::S5 => serializer.serialize_unit_variant("Name", 9u32, "S5"),
                Self::S6 => serializer.serialize_unit_variant("Name", 10u32, "S6"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets the sku tier. This is based on the SKU name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Free,
        Standard,
        Premium,
    }
}

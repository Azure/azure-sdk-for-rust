#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
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
    pub details: Vec<CloudErrorBody>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl CloudErrorBody {
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
#[doc = "A plan's extension properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Extension {
    #[doc = "The extension name. Supported values are: <br><br>**AgentlessDiscoveryForKubernetes** - API-based discovery of information about Kubernetes cluster architecture, workload objects, and setup. Required for Kubernetes inventory, identity and network exposure detection, attack path analysis and risk hunting as part of the cloud security explorer.\r\nAvailable for CloudPosture plan.<br><br>**OnUploadMalwareScanning** - Limits the GB to be scanned per month for each storage account within the subscription. Once this limit reached on a given storage account, Blobs won't be scanned during current calendar month.\r\nAvailable for StorageAccounts plan.<br><br>**SensitiveDataDiscovery** - Sensitive data discovery identifies Blob storage container with sensitive data such as credentials, credit cards, and more, to help prioritize and investigate security events.\r\nAvailable for StorageAccounts and CloudPosture plans.<br><br>**ContainerRegistriesVulnerabilityAssessments** - Provides vulnerability management for images stored in your container registries.\r\nAvailable for CloudPosture and Containers plans."]
    pub name: String,
    #[doc = "Indicates whether the extension is enabled."]
    #[serde(rename = "isEnabled")]
    pub is_enabled: extension::IsEnabled,
    #[doc = "Property values associated with the extension."]
    #[serde(rename = "additionalExtensionProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_extension_properties: Option<serde_json::Value>,
    #[doc = "A status describing the success/failure of the extension's enablement/disablement operation."]
    #[serde(rename = "operationStatus", default, skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<OperationStatus>,
}
impl Extension {
    pub fn new(name: String, is_enabled: extension::IsEnabled) -> Self {
        Self {
            name,
            is_enabled,
            additional_extension_properties: None,
            operation_status: None,
        }
    }
}
pub mod extension {
    use super::*;
    #[doc = "Indicates whether the extension is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsEnabled")]
    pub enum IsEnabled {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsEnabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsEnabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsEnabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("IsEnabled", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("IsEnabled", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A status describing the success/failure of the extension's enablement/disablement operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "The operation status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<operation_status::Code>,
    #[doc = "Additional information regarding the success/failure of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_status {
    use super::*;
    #[doc = "The operation status code."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Code")]
    pub enum Code {
        Succeeded,
        Failed,
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
                Self::Succeeded => serializer.serialize_unit_variant("Code", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Code", 1u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Microsoft Defender for Cloud is provided in two pricing tiers: free and standard. The standard tier offers advanced security capabilities, while the free tier offers basic security features."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Pricing {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Pricing properties for the relevant scope"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PricingProperties>,
}
impl Pricing {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of pricing configurations response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PricingList {
    #[doc = "List of pricing configurations"]
    pub value: Vec<Pricing>,
}
impl PricingList {
    pub fn new(value: Vec<Pricing>) -> Self {
        Self { value }
    }
}
#[doc = "Pricing properties for the relevant scope"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PricingProperties {
    #[doc = "The pricing tier value. Microsoft Defender for Cloud is provided in two pricing tiers: free and standard. The standard tier offers advanced security capabilities, while the free tier offers basic security features."]
    #[serde(rename = "pricingTier")]
    pub pricing_tier: pricing_properties::PricingTier,
    #[doc = "The sub-plan selected for a Standard pricing configuration, when more than one sub-plan is available. Each sub-plan enables a set of security features. When not specified, full plan is applied."]
    #[serde(rename = "subPlan", default, skip_serializing_if = "Option::is_none")]
    pub sub_plan: Option<String>,
    #[doc = "The duration left for the subscriptions free trial period - in ISO 8601 format (e.g. P3Y6M4DT12H30M5S)."]
    #[serde(rename = "freeTrialRemainingTime", default, skip_serializing_if = "Option::is_none")]
    pub free_trial_remaining_time: Option<String>,
    #[doc = "Optional. If `pricingTier` is `Standard` then this property holds the date of the last time the `pricingTier` was set to `Standard`, when available (e.g 2023-03-01T12:42:42.1921106Z)."]
    #[serde(rename = "enablementTime", default, with = "azure_core::date::rfc3339::option")]
    pub enablement_time: Option<time::OffsetDateTime>,
    #[doc = "Optional. True if the plan is deprecated. If there are replacing plans they will appear in `replacedBy` property"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[doc = "Optional. List of plans that replace this plan. This property exists only if this plan is deprecated."]
    #[serde(
        rename = "replacedBy",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub replaced_by: Vec<String>,
    #[doc = "Optional. List of extensions offered under a plan."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extensions: Vec<Extension>,
}
impl PricingProperties {
    pub fn new(pricing_tier: pricing_properties::PricingTier) -> Self {
        Self {
            pricing_tier,
            sub_plan: None,
            free_trial_remaining_time: None,
            enablement_time: None,
            deprecated: None,
            replaced_by: Vec::new(),
            extensions: Vec::new(),
        }
    }
}
pub mod pricing_properties {
    use super::*;
    #[doc = "The pricing tier value. Microsoft Defender for Cloud is provided in two pricing tiers: free and standard. The standard tier offers advanced security capabilities, while the free tier offers basic security features."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PricingTier")]
    pub enum PricingTier {
        Free,
        Standard,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PricingTier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PricingTier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PricingTier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Free => serializer.serialize_unit_variant("PricingTier", 0u32, "Free"),
                Self::Standard => serializer.serialize_unit_variant("PricingTier", 1u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes an Azure resource."]
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
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}

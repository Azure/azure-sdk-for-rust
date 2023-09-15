#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
pub type CanonicalSupportPlanInfo = Vec<CanonicalSupportPlanInfoDefinition>;
#[doc = "Definition object with the properties of a canonical plan"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CanonicalSupportPlanInfoDefinition {
    #[doc = "Support plan type."]
    #[serde(rename = "supportPlanType", default, skip_serializing_if = "Option::is_none")]
    pub support_plan_type: Option<canonical_support_plan_info_definition::SupportPlanType>,
    #[doc = "Flag to indicate if this support plan type is currently enabled for the subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The one time charge status for the subscription."]
    #[serde(rename = "oneTimeCharge", default, skip_serializing_if = "Option::is_none")]
    pub one_time_charge: Option<canonical_support_plan_info_definition::OneTimeCharge>,
}
impl CanonicalSupportPlanInfoDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod canonical_support_plan_info_definition {
    use super::*;
    #[doc = "Support plan type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SupportPlanType")]
    pub enum SupportPlanType {
        #[serde(rename = "essential")]
        Essential,
        #[serde(rename = "standard")]
        Standard,
        #[serde(rename = "advanced")]
        Advanced,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SupportPlanType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SupportPlanType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SupportPlanType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Essential => serializer.serialize_unit_variant("SupportPlanType", 0u32, "essential"),
                Self::Standard => serializer.serialize_unit_variant("SupportPlanType", 1u32, "standard"),
                Self::Advanced => serializer.serialize_unit_variant("SupportPlanType", 2u32, "advanced"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The one time charge status for the subscription."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OneTimeCharge")]
    pub enum OneTimeCharge {
        #[serde(rename = "no")]
        No,
        #[serde(rename = "onEnabled")]
        OnEnabled,
        #[serde(rename = "onReenabled")]
        OnReenabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OneTimeCharge {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OneTimeCharge {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OneTimeCharge {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::No => serializer.serialize_unit_variant("OneTimeCharge", 0u32, "no"),
                Self::OnEnabled => serializer.serialize_unit_variant("OneTimeCharge", 1u32, "onEnabled"),
                Self::OnReenabled => serializer.serialize_unit_variant("OneTimeCharge", 2u32, "onReenabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of the Canonical support plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CanonicalSupportPlanProperties {
    #[doc = "The provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<canonical_support_plan_properties::ProvisioningState>,
}
impl CanonicalSupportPlanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod canonical_support_plan_properties {
    use super::*;
    #[doc = "The provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Cancelled,
        Purchasing,
        Downgrading,
        Cancelling,
        Upgrading,
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
                Self::Cancelled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Cancelled"),
                Self::Purchasing => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Purchasing"),
                Self::Downgrading => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Downgrading"),
                Self::Cancelling => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Cancelling"),
                Self::Upgrading => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Upgrading"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The status of the Canonical support plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CanonicalSupportPlanResponseEnvelope {
    #[doc = "The id of the ARM resource, e.g. \"/subscriptions/{id}/providers/Microsoft.Addons/supportProvider/{supportProviderName}/supportPlanTypes/{planTypeName}\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the Canonical support plan, i.e. \"essential\", \"standard\" or \"advanced\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Microsoft.Addons/supportProvider"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The properties of the Canonical support plan."]
    pub properties: CanonicalSupportPlanProperties,
}
impl CanonicalSupportPlanResponseEnvelope {
    pub fn new(properties: CanonicalSupportPlanProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "Error description and code explaining why an operation failed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    #[doc = "Description of the error."]
    pub message: String,
    #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
    pub code: String,
}
impl ErrorDefinition {
    pub fn new(message: String, code: String) -> Self {
        Self { message, code }
    }
}
pub type OperationList = Vec<OperationsDefinition>;
#[doc = "List of supported operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListValue {
    #[doc = "List of supported operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<OperationList>,
}
impl OperationListValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition object with the name and properties of an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsDefinition {
    #[doc = "Name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display object with properties of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationsDisplayDefinition>,
}
impl OperationsDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Display object with properties of the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsDisplayDefinition {
    #[doc = "Resource provider of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Short description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationsDisplayDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}

#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
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
pub type CanonicalSupportPlanStatus = Vec<serde_json::Value>;
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

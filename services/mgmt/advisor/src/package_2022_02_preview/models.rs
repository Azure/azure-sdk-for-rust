#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "ARM error response body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmErrorResponseBody {
    #[doc = "Gets or sets the string that describes the error in detail and provides debugging information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Gets or sets the string that can be used to programmatically identify the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
impl ArmErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmErrorResponse {
    #[doc = "ARM error response body."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ArmErrorResponseBody>,
}
impl azure_core::Continuable for ArmErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ArmErrorResponse {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The list of Advisor operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntityListResult {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationEntityListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationEntityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for predict recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PredictionRequest {
    #[doc = "Properties given for the predictor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PredictionRequestProperties>,
}
impl PredictionRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties given for the predictor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PredictionRequestProperties {
    #[doc = "Type of the prediction."]
    #[serde(rename = "predictionType", default, skip_serializing_if = "Option::is_none")]
    pub prediction_type: Option<prediction_request_properties::PredictionType>,
    #[doc = "Extended properties are arguments specific for each prediction type."]
    #[serde(rename = "extendedProperties", default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<serde_json::Value>,
}
impl PredictionRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod prediction_request_properties {
    use super::*;
    #[doc = "Type of the prediction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PredictionType")]
    pub enum PredictionType {
        PredictiveRightsizing,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PredictionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PredictionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PredictionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PredictiveRightsizing => serializer.serialize_unit_variant("PredictionType", 0u32, "PredictiveRightsizing"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Response used by predictions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PredictionResponse {
    #[doc = "Properties of the prediction"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PredictionResponseProperties>,
}
impl PredictionResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the prediction"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PredictionResponseProperties {
    #[doc = "Extended properties"]
    #[serde(rename = "extendedProperties", default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<serde_json::Value>,
    #[doc = "Type of the prediction."]
    #[serde(rename = "predictionType", default, skip_serializing_if = "Option::is_none")]
    pub prediction_type: Option<prediction_response_properties::PredictionType>,
    #[doc = "The category of the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<prediction_response_properties::Category>,
    #[doc = "The business impact of the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact: Option<prediction_response_properties::Impact>,
    #[doc = "The resource type identified by Advisor."]
    #[serde(rename = "impactedField", default, skip_serializing_if = "Option::is_none")]
    pub impacted_field: Option<String>,
    #[doc = "The most recent time that Advisor checked the validity of the recommendation."]
    #[serde(rename = "lastUpdated", with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = "A summary of the recommendation."]
    #[serde(rename = "shortDescription", default, skip_serializing_if = "Option::is_none")]
    pub short_description: Option<ShortDescription>,
}
impl PredictionResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod prediction_response_properties {
    use super::*;
    #[doc = "Type of the prediction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PredictionType")]
    pub enum PredictionType {
        PredictiveRightsizing,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PredictionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PredictionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PredictionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PredictiveRightsizing => serializer.serialize_unit_variant("PredictionType", 0u32, "PredictiveRightsizing"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The category of the recommendation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Category")]
    pub enum Category {
        HighAvailability,
        Security,
        Performance,
        Cost,
        OperationalExcellence,
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
                Self::HighAvailability => serializer.serialize_unit_variant("Category", 0u32, "HighAvailability"),
                Self::Security => serializer.serialize_unit_variant("Category", 1u32, "Security"),
                Self::Performance => serializer.serialize_unit_variant("Category", 2u32, "Performance"),
                Self::Cost => serializer.serialize_unit_variant("Category", 3u32, "Cost"),
                Self::OperationalExcellence => serializer.serialize_unit_variant("Category", 4u32, "OperationalExcellence"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The business impact of the recommendation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Impact")]
    pub enum Impact {
        High,
        Medium,
        Low,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Impact {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Impact {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Impact {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::High => serializer.serialize_unit_variant("Impact", 0u32, "High"),
                Self::Medium => serializer.serialize_unit_variant("Impact", 1u32, "Medium"),
                Self::Low => serializer.serialize_unit_variant("Impact", 2u32, "Low"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A summary of the recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShortDescription {
    #[doc = "The remediation action suggested by the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solution: Option<String>,
}
impl ShortDescription {
    pub fn new() -> Self {
        Self::default()
    }
}

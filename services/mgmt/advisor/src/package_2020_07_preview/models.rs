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
impl ArmErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operation supported by Advisor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayInfo {
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The action that users can perform, based on their permission level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Service provider: Microsoft Advisor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl OperationDisplayInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operation supported by Advisor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntity {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The operation supported by Advisor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
}
impl OperationEntity {
    pub fn new() -> Self {
        Self::default()
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
    pub value: Vec<OperationEntity>,
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
#[doc = "The details of Advisor score for a single category."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdvisorScoreEntity {
    #[doc = "The ID of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The Advisor score data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<advisor_score_entity::Properties>,
}
impl AdvisorScoreEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod advisor_score_entity {
    use super::*;
    #[doc = "The Advisor score data."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The details of Advisor Score"]
        #[serde(rename = "lastRefreshedScore", default, skip_serializing_if = "Option::is_none")]
        pub last_refreshed_score: Option<ScoreEntity>,
        #[doc = "The historic data at different aggregation levels."]
        #[serde(rename = "timeSeries", default, skip_serializing_if = "Option::is_none")]
        pub time_series: Option<TimeSeriesEntity>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The details of Advisor Score"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScoreEntity {
    #[doc = "The date score was calculated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[doc = "The percentage score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[doc = "The consumption units for the score."]
    #[serde(rename = "consumptionUnits", default, skip_serializing_if = "Option::is_none")]
    pub consumption_units: Option<f64>,
    #[doc = "The number of impacted resources."]
    #[serde(rename = "impactedResourceCount", default, skip_serializing_if = "Option::is_none")]
    pub impacted_resource_count: Option<f64>,
    #[doc = "The potential percentage increase in overall score at subscription level once all recommendations in this scope are implemented."]
    #[serde(rename = "potentialScoreIncrease", default, skip_serializing_if = "Option::is_none")]
    pub potential_score_increase: Option<f64>,
    #[doc = "The count of impacted categories."]
    #[serde(rename = "categoryCount", default, skip_serializing_if = "Option::is_none")]
    pub category_count: Option<f64>,
}
impl ScoreEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type TimeSeriesEntity = Vec<serde_json::Value>;

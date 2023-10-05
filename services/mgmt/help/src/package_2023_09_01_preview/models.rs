#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Only for AutomatedStep type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomatedCheckResult {
    #[doc = "Insight Article Content"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[doc = "Type of Result."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<automated_check_result::Type>,
}
impl AutomatedCheckResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod automated_check_result {
    use super::*;
    #[doc = "Type of Result."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Success,
        Warning,
        Error,
        Information,
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
                Self::Success => serializer.serialize_unit_variant("Type", 0u32, "Success"),
                Self::Warning => serializer.serialize_unit_variant("Type", 1u32, "Warning"),
                Self::Error => serializer.serialize_unit_variant("Type", 2u32, "Error"),
                Self::Information => serializer.serialize_unit_variant("Type", 3u32, "Information"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "Response for whether the requested resource name is available or not."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResponse {
    #[doc = "Returns true or false depending on the availability of the name"]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Reason for why value is not available. This field is returned if nameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Gets an error message explaining the 'reason' value with more details. This field is returned iif nameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Troubleshooter ContinueRequest body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContinueRequestBody {
    #[doc = "Unique id of the result."]
    #[serde(rename = "stepId", default, skip_serializing_if = "Option::is_none")]
    pub step_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub responses: Vec<TroubleshooterResponse>,
}
impl ContinueRequestBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties returned with in an insight."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Diagnostic {
    #[doc = "Solution Id"]
    #[serde(rename = "solutionId", default, skip_serializing_if = "Option::is_none")]
    pub solution_id: Option<String>,
    #[doc = "Denotes the status of the diagnostic resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DiagnosticStatus>,
    #[doc = "The problems (if any) detected by this insight."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub insights: Vec<Insight>,
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
impl Diagnostic {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Solution Invocation with additional params needed for invocation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticInvocation {
    #[doc = "Solution Id to invoke."]
    #[serde(rename = "solutionId", default, skip_serializing_if = "Option::is_none")]
    pub solution_id: Option<String>,
    #[doc = "Additional parameters required to invoke the solutionId."]
    #[serde(rename = "additionalParameters", default, skip_serializing_if = "Option::is_none")]
    pub additional_parameters: Option<serde_json::Value>,
}
impl DiagnosticInvocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Diagnostic resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Diagnostic resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticResourceProperties>,
}
impl DiagnosticResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Diagnostic resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticResourceProperties {
    #[doc = "Global parameters that can be passed to all solutionIds."]
    #[serde(rename = "globalParameters", default, skip_serializing_if = "Option::is_none")]
    pub global_parameters: Option<serde_json::Value>,
    #[doc = "SolutionIds that are needed to be invoked."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub insights: Vec<DiagnosticInvocation>,
    #[doc = "Diagnostic Request Accepted time."]
    #[serde(rename = "acceptedAt", default, skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<String>,
    #[doc = "Status of diagnostic provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<diagnostic_resource_properties::ProvisioningState>,
    #[doc = "Array of Diagnostics."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub diagnostics: Vec<Diagnostic>,
}
impl DiagnosticResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod diagnostic_resource_properties {
    use super::*;
    #[doc = "Status of diagnostic provisioning."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        PartialComplete,
        Failed,
        Canceled,
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
                Self::PartialComplete => serializer.serialize_unit_variant("ProvisioningState", 1u32, "PartialComplete"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Denotes the status of the diagnostic resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DiagnosticStatus")]
pub enum DiagnosticStatus {
    Failed,
    MissingInputs,
    Running,
    Succeeded,
    Timeout,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DiagnosticStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DiagnosticStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DiagnosticStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Failed => serializer.serialize_unit_variant("DiagnosticStatus", 0u32, "Failed"),
            Self::MissingInputs => serializer.serialize_unit_variant("DiagnosticStatus", 1u32, "MissingInputs"),
            Self::Running => serializer.serialize_unit_variant("DiagnosticStatus", 2u32, "Running"),
            Self::Succeeded => serializer.serialize_unit_variant("DiagnosticStatus", 3u32, "Succeeded"),
            Self::Timeout => serializer.serialize_unit_variant("DiagnosticStatus", 4u32, "Timeout"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Discovery response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscoveryResponse {
    #[doc = "The list of metadata."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SolutionMetadataResource>,
    #[doc = "The link used to get the next page of solution metadata."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiscoveryResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DiscoveryResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Service specific error type which serves as additional context for the error herein."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "An array of additional nested error response info objects, as described by this contract."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<Error>,
}
impl Error {
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "Filter criterion"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Filter {
    #[doc = "Filter name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Filter values"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<String>,
    #[doc = "Filter operator"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
}
impl Filter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Filter group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FilterGroup {
    #[doc = "List of filters"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub filter: Vec<Filter>,
}
impl FilterGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Detailed insights(s) obtained via the invocation of an insight diagnostic"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Insight {
    #[doc = "Article id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "This insight's title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Detailed result content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<String>,
    #[doc = "Importance level of the insight."]
    #[serde(rename = "importanceLevel", default, skip_serializing_if = "Option::is_none")]
    pub importance_level: Option<insight::ImportanceLevel>,
}
impl Insight {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod insight {
    use super::*;
    #[doc = "Importance level of the insight."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ImportanceLevel")]
    pub enum ImportanceLevel {
        Critical,
        Warning,
        Information,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ImportanceLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ImportanceLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ImportanceLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Critical => serializer.serialize_unit_variant("ImportanceLevel", 0u32, "Critical"),
                Self::Warning => serializer.serialize_unit_variant("ImportanceLevel", 1u32, "Warning"),
                Self::Information => serializer.serialize_unit_variant("ImportanceLevel", 2u32, "Information"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Solutions metrics based chart"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricsBasedChart {
    #[doc = "Chart name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Allowed values are Sum, Avg, Count, Min, Max. Default is Sum"]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<metrics_based_chart::AggregationType>,
    #[doc = "Time span duration"]
    #[serde(rename = "timeSpanDuration", default, skip_serializing_if = "Option::is_none")]
    pub time_span_duration: Option<String>,
    #[doc = "Chart title"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Filter group"]
    #[serde(rename = "filterGroup", default, skip_serializing_if = "Option::is_none")]
    pub filter_group: Option<FilterGroup>,
    #[doc = "Place holder used in HTML Content replace control with the content"]
    #[serde(rename = "replacementKey", default, skip_serializing_if = "Option::is_none")]
    pub replacement_key: Option<String>,
}
impl MetricsBasedChart {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod metrics_based_chart {
    use super::*;
    #[doc = "Allowed values are Sum, Avg, Count, Min, Max. Default is Sum"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AggregationType")]
    pub enum AggregationType {
        Sum,
        Avg,
        Count,
        Min,
        Max,
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
                Self::Sum => serializer.serialize_unit_variant("AggregationType", 0u32, "Sum"),
                Self::Avg => serializer.serialize_unit_variant("AggregationType", 1u32, "Avg"),
                Self::Count => serializer.serialize_unit_variant("AggregationType", 2u32, "Count"),
                Self::Min => serializer.serialize_unit_variant("AggregationType", 3u32, "Min"),
                Self::Max => serializer.serialize_unit_variant("AggregationType", 4u32, "Max"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "A list of REST API operations supported by an Azure Resource Provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
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
#[doc = "Solution replacement maps."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplacementMaps {
    #[doc = "Solution AzureKB results"]
    #[serde(
        rename = "webResults",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub web_results: Vec<WebResult>,
    #[doc = "Solution diagnostics results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub diagnostics: Vec<SolutionsDiagnostic>,
    #[doc = "Solutions Troubleshooters"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub troubleshooters: Vec<SolutionsTroubleshooters>,
    #[doc = "Solution metrics based charts"]
    #[serde(
        rename = "metricsBasedCharts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metrics_based_charts: Vec<MetricsBasedChart>,
    #[doc = "Video solutions, which have the power to engage the customer by stimulating their senses"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub videos: Vec<Video>,
    #[doc = "Group of Videos"]
    #[serde(
        rename = "videoGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub video_groups: Vec<VideoGroup>,
}
impl ReplacementMaps {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. E.g. \"/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}\""]
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
#[doc = "The status of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseOption {
    #[doc = "Unique string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "Option description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ResponseOption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Troubleshooter step input response validation properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseValidationProperties {
    #[doc = "Regex used for the input validation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[doc = "Default True"]
    #[serde(rename = "isRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[doc = "Validation Error Message."]
    #[serde(rename = "validationErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub validation_error_message: Option<String>,
    #[doc = "Max text input (open Ended Text)."]
    #[serde(rename = "maxLength", default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
}
impl ResponseValidationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Troubleshooter restart response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestartTroubleshooterResponse {
    #[doc = "Updated TroubleshooterResource Name ."]
    #[serde(rename = "troubleshooterResourceName", default, skip_serializing_if = "Option::is_none")]
    pub troubleshooter_resource_name: Option<String>,
}
impl RestartTroubleshooterResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of an AzureKB search result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchResult {
    #[doc = "Unique id of the result."]
    #[serde(rename = "solutionId", default, skip_serializing_if = "Option::is_none")]
    pub solution_id: Option<String>,
    #[doc = "Content of the search result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "Title of the search result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Confidence of the search result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<search_result::Confidence>,
    #[doc = "Source of the search result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Result type of the search result."]
    #[serde(rename = "resultType", default, skip_serializing_if = "Option::is_none")]
    pub result_type: Option<search_result::ResultType>,
    #[doc = "rank of the search result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[doc = "Link to the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
impl SearchResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod search_result {
    use super::*;
    #[doc = "Confidence of the search result."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Confidence")]
    pub enum Confidence {
        Low,
        Medium,
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Confidence {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Confidence {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Confidence {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Low => serializer.serialize_unit_variant("Confidence", 0u32, "Low"),
                Self::Medium => serializer.serialize_unit_variant("Confidence", 1u32, "Medium"),
                Self::High => serializer.serialize_unit_variant("Confidence", 2u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Result type of the search result."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResultType")]
    pub enum ResultType {
        Community,
        Documentation,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResultType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResultType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResultType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Community => serializer.serialize_unit_variant("ResultType", 0u32, "Community"),
                Self::Documentation => serializer.serialize_unit_variant("ResultType", 1u32, "Documentation"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Part of the solution and are dividers in the solution rendering."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Section {
    #[doc = "Solution sections title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Solution sections content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "Solution replacement maps."]
    #[serde(rename = "replacementMaps", default, skip_serializing_if = "Option::is_none")]
    pub replacement_maps: Option<ReplacementMaps>,
}
impl Section {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionMetadataProperties {
    #[doc = "Solution Id."]
    #[serde(rename = "solutionId", default, skip_serializing_if = "Option::is_none")]
    pub solution_id: Option<String>,
    #[doc = "Solution Type."]
    #[serde(rename = "solutionType", default, skip_serializing_if = "Option::is_none")]
    pub solution_type: Option<SolutionType>,
    #[doc = "A detailed description of solution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Required parameters for invoking this particular solution."]
    #[serde(
        rename = "requiredInputs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_inputs: Vec<String>,
}
impl SolutionMetadataProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionMetadataResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "List of solutions"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Solutions>,
}
impl SolutionMetadataResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Solution response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionPatchRequestBody {
    #[doc = "Solution result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SolutionResourceProperties>,
}
impl SolutionPatchRequestBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Solution response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionResource {
    #[doc = "Full resource uri of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Type of resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Solution result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SolutionResourceProperties>,
}
impl SolutionResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Solution result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionResourceProperties {
    #[doc = "Solution request trigger criteria"]
    #[serde(
        rename = "triggerCriteria",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trigger_criteria: Vec<TriggerCriterion>,
    #[doc = "Client input parameters to run Solution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Solution Id to identify single solution."]
    #[serde(rename = "solutionId", default, skip_serializing_if = "Option::is_none")]
    pub solution_id: Option<String>,
    #[doc = "Status of solution provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<solution_resource_properties::ProvisioningState>,
    #[doc = "The title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The HTML content that needs to be rendered and shown to customer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "Solution replacement maps."]
    #[serde(rename = "replacementMaps", default, skip_serializing_if = "Option::is_none")]
    pub replacement_maps: Option<ReplacementMaps>,
    #[doc = "List of section object."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sections: Vec<Section>,
}
impl SolutionResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod solution_resource_properties {
    use super::*;
    #[doc = "Status of solution provisioning."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
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
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Solution Type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SolutionType")]
pub enum SolutionType {
    Diagnostics,
    Solutions,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SolutionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SolutionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SolutionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Diagnostics => serializer.serialize_unit_variant("SolutionType", 0u32, "Diagnostics"),
            Self::Solutions => serializer.serialize_unit_variant("SolutionType", 1u32, "Solutions"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "List of solutions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Solutions {
    #[doc = "List of metadata."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub solutions: Vec<SolutionMetadataProperties>,
}
impl Solutions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Solutions Diagnostic"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionsDiagnostic {
    #[doc = "Solution Id to identify single Solutions Diagnostic"]
    #[serde(rename = "solutionId", default, skip_serializing_if = "Option::is_none")]
    pub solution_id: Option<String>,
    #[doc = "The status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[doc = "Details of the status"]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "Place holder used in HTML Content replace control with the content"]
    #[serde(rename = "replacementKey", default, skip_serializing_if = "Option::is_none")]
    pub replacement_key: Option<String>,
    #[doc = "Required parameters of this item"]
    #[serde(
        rename = "requiredParameters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_parameters: Vec<String>,
    #[doc = "Diagnostic insights"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub insights: Vec<Insight>,
}
impl SolutionsDiagnostic {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Troubleshooters in Solutions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionsTroubleshooters {
    #[doc = "Solution Id to identify single Solutions Troubleshooter"]
    #[serde(rename = "solutionId", default, skip_serializing_if = "Option::is_none")]
    pub solution_id: Option<String>,
    #[doc = "Troubleshooter title"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Troubleshooter summary"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}
impl SolutionsTroubleshooters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Status")]
pub enum Status {
    Failed,
    MissingInputs,
    Running,
    Succeeded,
    Timeout,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Status {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Failed => serializer.serialize_unit_variant("Status", 0u32, "Failed"),
            Self::MissingInputs => serializer.serialize_unit_variant("Status", 1u32, "MissingInputs"),
            Self::Running => serializer.serialize_unit_variant("Status", 2u32, "Running"),
            Self::Succeeded => serializer.serialize_unit_variant("Status", 3u32, "Succeeded"),
            Self::Timeout => serializer.serialize_unit_variant("Status", 4u32, "Timeout"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Troubleshooter step"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Step {
    #[doc = "Unique step id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Step title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Step description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Get or sets the Step guidance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
    #[doc = "Status of Troubleshooter Step execution."]
    #[serde(rename = "executionStatus", default, skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<step::ExecutionStatus>,
    #[doc = "This field has more detailed status description of the execution status."]
    #[serde(rename = "executionStatusDescription", default, skip_serializing_if = "Option::is_none")]
    pub execution_status_description: Option<String>,
    #[doc = "Type of Troubleshooting step."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<step::Type>,
    #[doc = "is this last step of the workflow."]
    #[serde(rename = "isLastStep", default, skip_serializing_if = "Option::is_none")]
    pub is_last_step: Option<bool>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inputs: Vec<StepInput>,
    #[doc = "Only for AutomatedStep type"]
    #[serde(rename = "automatedCheckResults", default, skip_serializing_if = "Option::is_none")]
    pub automated_check_results: Option<AutomatedCheckResult>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub insights: Vec<Insight>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl Step {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod step {
    use super::*;
    #[doc = "Status of Troubleshooter Step execution."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ExecutionStatus")]
    pub enum ExecutionStatus {
        Success,
        Running,
        Failed,
        Warning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ExecutionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ExecutionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ExecutionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Success => serializer.serialize_unit_variant("ExecutionStatus", 0u32, "Success"),
                Self::Running => serializer.serialize_unit_variant("ExecutionStatus", 1u32, "Running"),
                Self::Failed => serializer.serialize_unit_variant("ExecutionStatus", 2u32, "Failed"),
                Self::Warning => serializer.serialize_unit_variant("ExecutionStatus", 3u32, "Warning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of Troubleshooting step."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Decision,
        Solution,
        Insight,
        AutomatedCheck,
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
                Self::Decision => serializer.serialize_unit_variant("Type", 0u32, "Decision"),
                Self::Solution => serializer.serialize_unit_variant("Type", 1u32, "Solution"),
                Self::Insight => serializer.serialize_unit_variant("Type", 2u32, "Insight"),
                Self::AutomatedCheck => serializer.serialize_unit_variant("Type", 3u32, "AutomatedCheck"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Details of step input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StepInput {
    #[doc = "Use Index as QuestionId."]
    #[serde(rename = "questionId", default, skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    #[doc = "Text Input. Will be a single line input."]
    #[serde(rename = "questionType", default, skip_serializing_if = "Option::is_none")]
    pub question_type: Option<String>,
    #[doc = "User question content."]
    #[serde(rename = "questionContent", default, skip_serializing_if = "Option::is_none")]
    pub question_content: Option<String>,
    #[doc = "Default is Text."]
    #[serde(rename = "questionContentType", default, skip_serializing_if = "Option::is_none")]
    pub question_content_type: Option<step_input::QuestionContentType>,
    #[doc = "Place holder text for response hints."]
    #[serde(rename = "responseHint", default, skip_serializing_if = "Option::is_none")]
    pub response_hint: Option<String>,
    #[doc = "Result of Automate step."]
    #[serde(rename = "recommendedOption", default, skip_serializing_if = "Option::is_none")]
    pub recommended_option: Option<String>,
    #[doc = "Text of response that was selected."]
    #[serde(rename = "selectedOptionValue", default, skip_serializing_if = "Option::is_none")]
    pub selected_option_value: Option<String>,
    #[doc = "Troubleshooter step input response validation properties"]
    #[serde(rename = "responseValidationProperties", default, skip_serializing_if = "Option::is_none")]
    pub response_validation_properties: Option<ResponseValidationProperties>,
    #[serde(
        rename = "responseOptions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub response_options: Vec<ResponseOption>,
}
impl StepInput {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod step_input {
    use super::*;
    #[doc = "Default is Text."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "QuestionContentType")]
    pub enum QuestionContentType {
        Text,
        Html,
        Markdown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for QuestionContentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for QuestionContentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for QuestionContentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Text => serializer.serialize_unit_variant("QuestionContentType", 0u32, "Text"),
                Self::Html => serializer.serialize_unit_variant("QuestionContentType", 1u32, "Html"),
                Self::Markdown => serializer.serialize_unit_variant("QuestionContentType", 2u32, "Markdown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Solution request trigger criterion. SolutionId/ProblemClassificationId is the only supported trigger type for Solution PUT request. ReplacementKey is the only supported trigger type for Solution PATCH request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerCriterion {
    #[doc = "Trigger criterion name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<trigger_criterion::Name>,
    #[doc = "Trigger criterion value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl TriggerCriterion {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod trigger_criterion {
    use super::*;
    #[doc = "Trigger criterion name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        SolutionId,
        ProblemClassificationId,
        ReplacementKey,
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
                Self::SolutionId => serializer.serialize_unit_variant("Name", 0u32, "SolutionId"),
                Self::ProblemClassificationId => serializer.serialize_unit_variant("Name", 1u32, "ProblemClassificationId"),
                Self::ReplacementKey => serializer.serialize_unit_variant("Name", 2u32, "ReplacementKey"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Troubleshooter Instance properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TroubleshooterInstanceProperties {
    #[doc = "Solution Id to identify single troubleshooter."]
    #[serde(rename = "solutionId", default, skip_serializing_if = "Option::is_none")]
    pub solution_id: Option<String>,
    #[doc = "Client input parameters to run Troubleshooter Resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Status of troubleshooter provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<troubleshooter_instance_properties::ProvisioningState>,
    #[doc = "List of step object."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub steps: Vec<Step>,
}
impl TroubleshooterInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod troubleshooter_instance_properties {
    use super::*;
    #[doc = "Status of troubleshooter provisioning."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Running,
        AutoContinue,
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
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Running => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Running"),
                Self::AutoContinue => serializer.serialize_unit_variant("ProvisioningState", 4u32, "AutoContinue"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Troubleshooter response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TroubleshooterResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Troubleshooter Instance properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TroubleshooterInstanceProperties>,
}
impl TroubleshooterResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User Response for Troubleshooter continue request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TroubleshooterResponse {
    #[doc = "id of the question."]
    #[serde(rename = "questionId", default, skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    #[doc = "Text Input. Will be a single line input."]
    #[serde(rename = "questionType", default, skip_serializing_if = "Option::is_none")]
    pub question_type: Option<troubleshooter_response::QuestionType>,
    #[doc = "Response key for SingleInput. For Multi-line test/open ended question it is free form text"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
}
impl TroubleshooterResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod troubleshooter_response {
    use super::*;
    #[doc = "Text Input. Will be a single line input."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "QuestionType")]
    pub enum QuestionType {
        RadioButton,
        Dropdown,
        TextInput,
        MultiLineInfoBox,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for QuestionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for QuestionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for QuestionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::RadioButton => serializer.serialize_unit_variant("QuestionType", 0u32, "RadioButton"),
                Self::Dropdown => serializer.serialize_unit_variant("QuestionType", 1u32, "Dropdown"),
                Self::TextInput => serializer.serialize_unit_variant("QuestionType", 2u32, "TextInput"),
                Self::MultiLineInfoBox => serializer.serialize_unit_variant("QuestionType", 3u32, "MultiLineInfoBox"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Video detail"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Video {
    #[serde(flatten)]
    pub video_group_video: VideoGroupVideo,
    #[doc = "Place holder used in HTML Content replace control with the insight content"]
    #[serde(rename = "replacementKey", default, skip_serializing_if = "Option::is_none")]
    pub replacement_key: Option<String>,
}
impl Video {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Video group detail"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoGroup {
    #[doc = "List of videos will be shown to customers"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub videos: Vec<VideoGroupVideo>,
    #[doc = "Place holder used in HTML Content replace control with the insight content"]
    #[serde(rename = "replacementKey", default, skip_serializing_if = "Option::is_none")]
    pub replacement_key: Option<String>,
}
impl VideoGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VideoGroup video detail"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoGroupVideo {
    #[doc = "Link to the video"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[doc = "Title of the video"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl VideoGroupVideo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AzureKB web result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebResult {
    #[doc = "Place holder used in HTML Content replace control with the content"]
    #[serde(rename = "replacementKey", default, skip_serializing_if = "Option::is_none")]
    pub replacement_key: Option<String>,
    #[doc = "AzureKB search results"]
    #[serde(
        rename = "searchResults",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub search_results: Vec<SearchResult>,
}
impl WebResult {
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

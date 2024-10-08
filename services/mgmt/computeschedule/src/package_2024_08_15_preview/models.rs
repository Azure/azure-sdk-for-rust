#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
pub type AzureCoreAzureLocation = String;
#[doc = "This is the request to cancel running operations in scheduled actions using the operation ids"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CancelOperationsRequest {
    #[doc = "The list of operation ids to cancel operations on"]
    #[serde(rename = "operationIds")]
    pub operation_ids: Vec<String>,
    #[doc = "CorrelationId item"]
    pub correlationid: String,
}
impl CancelOperationsRequest {
    pub fn new(operation_ids: Vec<String>, correlationid: String) -> Self {
        Self {
            operation_ids,
            correlationid,
        }
    }
}
#[doc = "This is the response from a cancel operations request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CancelOperationsResponse {
    #[doc = "An array of resource operations that were successfully cancelled"]
    pub results: Vec<ResourceOperation>,
}
impl CancelOperationsResponse {
    pub fn new(results: Vec<ResourceOperation>) -> Self {
        Self { results }
    }
}
#[doc = "The types of deadlines supported by ScheduledActions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeadlineType")]
pub enum DeadlineType {
    Unknown,
    InitiateAt,
    CompleteBy,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeadlineType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeadlineType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeadlineType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("DeadlineType", 0u32, "Unknown"),
            Self::InitiateAt => serializer.serialize_unit_variant("DeadlineType", 1u32, "InitiateAt"),
            Self::CompleteBy => serializer.serialize_unit_variant("DeadlineType", 2u32, "CompleteBy"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response from a deallocate request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeallocateResourceOperationResponse {
    #[doc = "The description of the operation response"]
    pub description: String,
    #[doc = "The type of resources used in the deallocate request eg virtual machines"]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Represents an Azure geography region where supported resource providers live."]
    pub location: AzureCoreAzureLocation,
    #[doc = "The results from the deallocate request if no errors exist"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<ResourceOperation>,
}
impl DeallocateResourceOperationResponse {
    pub fn new(description: String, type_: String, location: AzureCoreAzureLocation) -> Self {
        Self {
            description,
            type_,
            location,
            results: Vec::new(),
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
#[doc = "The ExecuteDeallocateRequest request for executeDeallocate operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecuteDeallocateRequest {
    #[doc = "Extra details needed to run the user's request"]
    #[serde(rename = "executionParameters")]
    pub execution_parameters: ExecutionParameters,
    #[doc = "The resources needed for the user request"]
    pub resources: Resources,
    #[doc = "CorrelationId item"]
    pub correlationid: String,
}
impl ExecuteDeallocateRequest {
    pub fn new(execution_parameters: ExecutionParameters, resources: Resources, correlationid: String) -> Self {
        Self {
            execution_parameters,
            resources,
            correlationid,
        }
    }
}
#[doc = "The ExecuteHibernateRequest request for executeHibernate operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecuteHibernateRequest {
    #[doc = "Extra details needed to run the user's request"]
    #[serde(rename = "executionParameters")]
    pub execution_parameters: ExecutionParameters,
    #[doc = "The resources needed for the user request"]
    pub resources: Resources,
    #[doc = "CorrelationId item"]
    pub correlationid: String,
}
impl ExecuteHibernateRequest {
    pub fn new(execution_parameters: ExecutionParameters, resources: Resources, correlationid: String) -> Self {
        Self {
            execution_parameters,
            resources,
            correlationid,
        }
    }
}
#[doc = "The ExecuteStartRequest request for executeStart operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecuteStartRequest {
    #[doc = "Extra details needed to run the user's request"]
    #[serde(rename = "executionParameters")]
    pub execution_parameters: ExecutionParameters,
    #[doc = "The resources needed for the user request"]
    pub resources: Resources,
    #[doc = "CorrelationId item"]
    pub correlationid: String,
}
impl ExecuteStartRequest {
    pub fn new(execution_parameters: ExecutionParameters, resources: Resources, correlationid: String) -> Self {
        Self {
            execution_parameters,
            resources,
            correlationid,
        }
    }
}
#[doc = "Extra details needed to run the user's request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExecutionParameters {
    #[doc = "The preferences customers can select to optimize their requests to ScheduledActions"]
    #[serde(rename = "optimizationPreference", default, skip_serializing_if = "Option::is_none")]
    pub optimization_preference: Option<OptimizationPreference>,
    #[doc = "The retry policy for the user request"]
    #[serde(rename = "retryPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
}
impl ExecutionParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the request to get errors per vm operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetOperationErrorsRequest {
    #[doc = "The list of operation ids to query errors of"]
    #[serde(rename = "operationIds")]
    pub operation_ids: Vec<String>,
}
impl GetOperationErrorsRequest {
    pub fn new(operation_ids: Vec<String>) -> Self {
        Self { operation_ids }
    }
}
#[doc = "This is the response from a get operations errors request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetOperationErrorsResponse {
    #[doc = "An array of operationids and their corresponding errors if any"]
    pub results: Vec<OperationErrorsResult>,
}
impl GetOperationErrorsResponse {
    pub fn new(results: Vec<OperationErrorsResult>) -> Self {
        Self { results }
    }
}
#[doc = "This is the request to get operation status using operationids"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetOperationStatusRequest {
    #[doc = "The list of operation ids to get the status of"]
    #[serde(rename = "operationIds")]
    pub operation_ids: Vec<String>,
    #[doc = "CorrelationId item"]
    pub correlationid: String,
}
impl GetOperationStatusRequest {
    pub fn new(operation_ids: Vec<String>, correlationid: String) -> Self {
        Self {
            operation_ids,
            correlationid,
        }
    }
}
#[doc = "This is the response from a get operations status request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetOperationStatusResponse {
    #[doc = "An array of resource operations based on their operation ids"]
    pub results: Vec<ResourceOperation>,
}
impl GetOperationStatusResponse {
    pub fn new(results: Vec<ResourceOperation>) -> Self {
        Self { results }
    }
}
#[doc = "The response from a Hibernate request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HibernateResourceOperationResponse {
    #[doc = "The description of the operation response"]
    pub description: String,
    #[doc = "The type of resources used in the Hibernate request eg virtual machines"]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Represents an Azure geography region where supported resource providers live."]
    pub location: AzureCoreAzureLocation,
    #[doc = "The results from the Hibernate request if no errors exist"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<ResourceOperation>,
}
impl HibernateResourceOperationResponse {
    pub fn new(description: String, type_: String, location: AzureCoreAzureLocation) -> Self {
        Self {
            description,
            type_,
            location,
            results: Vec::new(),
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
#[doc = "This defines a list of operation errors associated with a unique operationId"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationErrorDetails {
    #[doc = "The error code of the operation"]
    #[serde(rename = "errorCode")]
    pub error_code: String,
    #[doc = "The error details of the operation"]
    #[serde(rename = "errorDetails", with = "azure_core::date::rfc3339")]
    pub error_details: ::time::OffsetDateTime,
    #[doc = "The timestamp of the error occurence"]
    #[serde(rename = "timeStamp", with = "azure_core::date::rfc3339")]
    pub time_stamp: ::time::OffsetDateTime,
    #[doc = "CRP operationid of the operation for deeper analysis"]
    #[serde(rename = "crpOperationId")]
    pub crp_operation_id: String,
}
impl OperationErrorDetails {
    pub fn new(
        error_code: String,
        error_details: ::time::OffsetDateTime,
        time_stamp: ::time::OffsetDateTime,
        crp_operation_id: String,
    ) -> Self {
        Self {
            error_code,
            error_details,
            time_stamp,
            crp_operation_id,
        }
    }
}
#[doc = "This is the first level of operation errors from the request when clients get errors per vm operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationErrorsResult {
    #[doc = "The operationId identifying a vm operation"]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "The creation time of the error result"]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<::time::OffsetDateTime>,
    #[doc = "The activation time of a vm operation"]
    #[serde(rename = "activationTime", default, with = "azure_core::date::rfc3339::option")]
    pub activation_time: Option<::time::OffsetDateTime>,
    #[doc = "The completion time of the operation if the operation was completed"]
    #[serde(rename = "completedAt", default, with = "azure_core::date::rfc3339::option")]
    pub completed_at: Option<::time::OffsetDateTime>,
    #[doc = "A list of errors associated with the operationid"]
    #[serde(
        rename = "operationErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operation_errors: Vec<OperationErrorDetails>,
    #[doc = "Request level error code"]
    #[serde(rename = "requestErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub request_error_code: Option<String>,
    #[doc = "Request level error details"]
    #[serde(rename = "requestErrorDetails", default, skip_serializing_if = "Option::is_none")]
    pub request_error_details: Option<String>,
}
impl OperationErrorsResult {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Values that define the states of operations in Scheduled Actions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationState")]
pub enum OperationState {
    Unknown,
    PendingScheduling,
    Scheduled,
    PendingExecution,
    Executing,
    Succeeded,
    Failed,
    Cancelled,
    Blocked,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("OperationState", 0u32, "Unknown"),
            Self::PendingScheduling => serializer.serialize_unit_variant("OperationState", 1u32, "PendingScheduling"),
            Self::Scheduled => serializer.serialize_unit_variant("OperationState", 2u32, "Scheduled"),
            Self::PendingExecution => serializer.serialize_unit_variant("OperationState", 3u32, "PendingExecution"),
            Self::Executing => serializer.serialize_unit_variant("OperationState", 4u32, "Executing"),
            Self::Succeeded => serializer.serialize_unit_variant("OperationState", 5u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("OperationState", 6u32, "Failed"),
            Self::Cancelled => serializer.serialize_unit_variant("OperationState", 7u32, "Cancelled"),
            Self::Blocked => serializer.serialize_unit_variant("OperationState", 8u32, "Blocked"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The preferences customers can select to optimize their requests to ScheduledActions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OptimizationPreference")]
pub enum OptimizationPreference {
    Cost,
    Availability,
    CostAvailabilityBalanced,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OptimizationPreference {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OptimizationPreference {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OptimizationPreference {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Cost => serializer.serialize_unit_variant("OptimizationPreference", 0u32, "Cost"),
            Self::Availability => serializer.serialize_unit_variant("OptimizationPreference", 1u32, "Availability"),
            Self::CostAvailabilityBalanced => serializer.serialize_unit_variant("OptimizationPreference", 2u32, "CostAvailabilityBalanced"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "High level response from an operation on a resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceOperation {
    #[doc = "Unique identifier for the resource involved in the operation, eg ArmId"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Resource level error code if it exists"]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Resource level error details if they exist"]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<String>,
    #[doc = "The details of a response from an operation on a resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<ResourceOperationDetails>,
}
impl ResourceOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of a response from an operation on a resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceOperationDetails {
    #[doc = "Operation identifier for the unique operation"]
    #[serde(rename = "operationId")]
    pub operation_id: String,
    #[doc = "Unique identifier for the resource involved in the operation, eg ArmId"]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "Type of operation performed on the resources"]
    #[serde(rename = "opType")]
    pub op_type: resource_operation_details::OpType,
    #[doc = "Subscription id attached to the request"]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
    #[doc = "Deadline for the operation"]
    #[serde(with = "azure_core::date::rfc3339")]
    pub deadline: ::time::OffsetDateTime,
    #[doc = "Type of deadline of the operation"]
    #[serde(rename = "deadlineType")]
    pub deadline_type: resource_operation_details::DeadlineType,
    #[doc = "Current state of the operation"]
    pub state: resource_operation_details::State,
    #[doc = "Timezone for the operation"]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "These describe errors that occur at the resource level"]
    #[serde(rename = "resourceOperationError", default, skip_serializing_if = "Option::is_none")]
    pub resource_operation_error: Option<ResourceOperationError>,
    #[doc = "Time the operation was complete if errors are null"]
    #[serde(rename = "completedAt", default, with = "azure_core::date::rfc3339::option")]
    pub completed_at: Option<::time::OffsetDateTime>,
    #[doc = "The retry policy for the user request"]
    #[serde(rename = "retryPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
}
impl ResourceOperationDetails {
    pub fn new(
        operation_id: String,
        resource_id: String,
        op_type: resource_operation_details::OpType,
        subscription_id: String,
        deadline: ::time::OffsetDateTime,
        deadline_type: resource_operation_details::DeadlineType,
        state: resource_operation_details::State,
    ) -> Self {
        Self {
            operation_id,
            resource_id,
            op_type,
            subscription_id,
            deadline,
            deadline_type,
            state,
            time_zone: None,
            resource_operation_error: None,
            completed_at: None,
            retry_policy: None,
        }
    }
}
pub mod resource_operation_details {
    use super::*;
    #[doc = "Type of operation performed on the resources"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OpType")]
    pub enum OpType {
        Unknown,
        Start,
        Deallocate,
        Hibernate,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OpType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OpType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OpType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("OpType", 0u32, "Unknown"),
                Self::Start => serializer.serialize_unit_variant("OpType", 1u32, "Start"),
                Self::Deallocate => serializer.serialize_unit_variant("OpType", 2u32, "Deallocate"),
                Self::Hibernate => serializer.serialize_unit_variant("OpType", 3u32, "Hibernate"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for OpType {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[doc = "Type of deadline of the operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeadlineType")]
    pub enum DeadlineType {
        Unknown,
        InitiateAt,
        CompleteBy,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeadlineType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeadlineType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeadlineType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("DeadlineType", 0u32, "Unknown"),
                Self::InitiateAt => serializer.serialize_unit_variant("DeadlineType", 1u32, "InitiateAt"),
                Self::CompleteBy => serializer.serialize_unit_variant("DeadlineType", 2u32, "CompleteBy"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for DeadlineType {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[doc = "Current state of the operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Unknown,
        PendingScheduling,
        Scheduled,
        PendingExecution,
        Executing,
        Succeeded,
        Failed,
        Cancelled,
        Blocked,
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
                Self::Unknown => serializer.serialize_unit_variant("State", 0u32, "Unknown"),
                Self::PendingScheduling => serializer.serialize_unit_variant("State", 1u32, "PendingScheduling"),
                Self::Scheduled => serializer.serialize_unit_variant("State", 2u32, "Scheduled"),
                Self::PendingExecution => serializer.serialize_unit_variant("State", 3u32, "PendingExecution"),
                Self::Executing => serializer.serialize_unit_variant("State", 4u32, "Executing"),
                Self::Succeeded => serializer.serialize_unit_variant("State", 5u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("State", 6u32, "Failed"),
                Self::Cancelled => serializer.serialize_unit_variant("State", 7u32, "Cancelled"),
                Self::Blocked => serializer.serialize_unit_variant("State", 8u32, "Blocked"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for State {
        fn default() -> Self {
            Self::Unknown
        }
    }
}
#[doc = "These describe errors that occur at the resource level"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceOperationError {
    #[doc = "Code for the error eg 404, 500"]
    #[serde(rename = "errorCode")]
    pub error_code: String,
    #[doc = "Detailed message about the error"]
    #[serde(rename = "errorDetails")]
    pub error_details: String,
}
impl ResourceOperationError {
    pub fn new(error_code: String, error_details: String) -> Self {
        Self { error_code, error_details }
    }
}
#[doc = "The kind of operation types that can be performed on resources using ScheduledActions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceOperationType")]
pub enum ResourceOperationType {
    Unknown,
    Start,
    Deallocate,
    Hibernate,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceOperationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceOperationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceOperationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ResourceOperationType", 0u32, "Unknown"),
            Self::Start => serializer.serialize_unit_variant("ResourceOperationType", 1u32, "Start"),
            Self::Deallocate => serializer.serialize_unit_variant("ResourceOperationType", 2u32, "Deallocate"),
            Self::Hibernate => serializer.serialize_unit_variant("ResourceOperationType", 3u32, "Hibernate"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resources needed for the user request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resources {
    #[doc = "The resource ids used for the request"]
    pub ids: Vec<String>,
}
impl Resources {
    pub fn new(ids: Vec<String>) -> Self {
        Self { ids }
    }
}
#[doc = "The retry policy for the user request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetryPolicy {
    #[doc = "Retry count for user request"]
    #[serde(rename = "retryCount", default, skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<i32>,
    #[doc = "Retry window in minutes for user request"]
    #[serde(rename = "retryWindowInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub retry_window_in_minutes: Option<i32>,
}
impl RetryPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The schedule details for the user request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    #[doc = "The deadline for the operation"]
    #[serde(rename = "deadLine", with = "azure_core::date::rfc3339")]
    pub dead_line: ::time::OffsetDateTime,
    #[doc = "The timezone for the operation"]
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[doc = "The deadlinetype of the operation, this can either be InitiateAt or CompleteBy"]
    #[serde(rename = "deadlineType")]
    pub deadline_type: schedule::DeadlineType,
}
impl Schedule {
    pub fn new(dead_line: ::time::OffsetDateTime, time_zone: String, deadline_type: schedule::DeadlineType) -> Self {
        Self {
            dead_line,
            time_zone,
            deadline_type,
        }
    }
}
pub mod schedule {
    use super::*;
    #[doc = "The deadlinetype of the operation, this can either be InitiateAt or CompleteBy"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeadlineType")]
    pub enum DeadlineType {
        Unknown,
        InitiateAt,
        CompleteBy,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeadlineType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeadlineType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeadlineType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("DeadlineType", 0u32, "Unknown"),
                Self::InitiateAt => serializer.serialize_unit_variant("DeadlineType", 1u32, "InitiateAt"),
                Self::CompleteBy => serializer.serialize_unit_variant("DeadlineType", 2u32, "CompleteBy"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for DeadlineType {
        fn default() -> Self {
            Self::Unknown
        }
    }
}
#[doc = "The response from a start request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartResourceOperationResponse {
    #[doc = "The description of the operation response"]
    pub description: String,
    #[doc = "The type of resources used in the start request eg virtual machines"]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Represents an Azure geography region where supported resource providers live."]
    pub location: AzureCoreAzureLocation,
    #[doc = "The results from the start request if no errors exist"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<ResourceOperation>,
}
impl StartResourceOperationResponse {
    pub fn new(description: String, type_: String, location: AzureCoreAzureLocation) -> Self {
        Self {
            description,
            type_,
            location,
            results: Vec::new(),
        }
    }
}
#[doc = "The deallocate request for resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitDeallocateRequest {
    #[doc = "The schedule details for the user request"]
    pub schedule: Schedule,
    #[doc = "Extra details needed to run the user's request"]
    #[serde(rename = "executionParameters")]
    pub execution_parameters: ExecutionParameters,
    #[doc = "The resources needed for the user request"]
    pub resources: Resources,
    #[doc = "CorrelationId item"]
    pub correlationid: String,
}
impl SubmitDeallocateRequest {
    pub fn new(schedule: Schedule, execution_parameters: ExecutionParameters, resources: Resources, correlationid: String) -> Self {
        Self {
            schedule,
            execution_parameters,
            resources,
            correlationid,
        }
    }
}
#[doc = "This is the request for hibernate"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitHibernateRequest {
    #[doc = "The schedule details for the user request"]
    pub schedule: Schedule,
    #[doc = "Extra details needed to run the user's request"]
    #[serde(rename = "executionParameters")]
    pub execution_parameters: ExecutionParameters,
    #[doc = "The resources needed for the user request"]
    pub resources: Resources,
    #[doc = "CorrelationId item"]
    pub correlationid: String,
}
impl SubmitHibernateRequest {
    pub fn new(schedule: Schedule, execution_parameters: ExecutionParameters, resources: Resources, correlationid: String) -> Self {
        Self {
            schedule,
            execution_parameters,
            resources,
            correlationid,
        }
    }
}
#[doc = "This is the request for start"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitStartRequest {
    #[doc = "The schedule details for the user request"]
    pub schedule: Schedule,
    #[doc = "Extra details needed to run the user's request"]
    #[serde(rename = "executionParameters")]
    pub execution_parameters: ExecutionParameters,
    #[doc = "The resources needed for the user request"]
    pub resources: Resources,
    #[doc = "CorrelationId item"]
    pub correlationid: String,
}
impl SubmitStartRequest {
    pub fn new(schedule: Schedule, execution_parameters: ExecutionParameters, resources: Resources, correlationid: String) -> Self {
        Self {
            schedule,
            execution_parameters,
            resources,
            correlationid,
        }
    }
}

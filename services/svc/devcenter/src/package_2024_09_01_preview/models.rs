#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreFoundationsError {
    #[doc = "One of a server-defined set of error codes."]
    pub code: String,
    #[doc = "A human-readable representation of the error."]
    pub message: String,
    #[doc = "The target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "An array of details about specific errors that led to this reported error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<AzureCoreFoundationsError>,
    #[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<AzureCoreFoundationsInnerError>,
}
impl AzureCoreFoundationsError {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
            innererror: None,
        }
    }
}
#[doc = "A response containing error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreFoundationsErrorResponse {
    #[doc = "The error object."]
    pub error: AzureCoreFoundationsError,
}
impl azure_core::Continuable for AzureCoreFoundationsErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AzureCoreFoundationsErrorResponse {
    pub fn new(error: AzureCoreFoundationsError) -> Self {
        Self { error }
    }
}
#[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCoreFoundationsInnerError {
    #[doc = "One of a server-defined set of error codes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<Box<AzureCoreFoundationsInnerError>>,
}
impl AzureCoreFoundationsInnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enum describing allowed operation states."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureCoreFoundationsOperationState")]
pub enum AzureCoreFoundationsOperationState {
    NotStarted,
    Running,
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureCoreFoundationsOperationState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureCoreFoundationsOperationState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureCoreFoundationsOperationState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotStarted => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 0u32, "NotStarted"),
            Self::Running => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 1u32, "Running"),
            Self::Succeeded => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 3u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 4u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type AzureCoreAzureLocation = String;
pub type AzureCoreUuid = String;
#[doc = "A catalog."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Catalog {
    #[doc = "The unique URI of the catalog."]
    pub uri: String,
    #[doc = "Name of the catalog."]
    pub name: String,
}
impl Catalog {
    pub fn new(uri: String, name: String) -> Self {
        Self { uri, name }
    }
}
#[doc = "Represents a list of tasks to apply to a Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomizationGroup {
    #[doc = "Tasks to apply. Note by default tasks are excluded from the response when\nlisting customization groups. To include them, use the `include=tasks` query\nparameter."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tasks: Vec<CustomizationTask>,
    #[doc = "The unique URI of the customization group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Name of the customization group."]
    pub name: String,
    #[doc = "Status of a customization group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CustomizationGroupStatus>,
    #[doc = "Start time of the customization group, in RFC3339 format."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "End time of the customization group, in RFC3339 format."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
}
impl CustomizationGroup {
    pub fn new(name: String) -> Self {
        Self {
            tasks: Vec::new(),
            uri: None,
            name,
            status: None,
            start_time: None,
            end_time: None,
        }
    }
}
#[doc = "Status of a customization group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CustomizationGroupStatus")]
pub enum CustomizationGroupStatus {
    NotStarted,
    Running,
    Succeeded,
    Failed,
    ValidationFailed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CustomizationGroupStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CustomizationGroupStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CustomizationGroupStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotStarted => serializer.serialize_unit_variant("CustomizationGroupStatus", 0u32, "NotStarted"),
            Self::Running => serializer.serialize_unit_variant("CustomizationGroupStatus", 1u32, "Running"),
            Self::Succeeded => serializer.serialize_unit_variant("CustomizationGroupStatus", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("CustomizationGroupStatus", 3u32, "Failed"),
            Self::ValidationFailed => serializer.serialize_unit_variant("CustomizationGroupStatus", 4u32, "ValidationFailed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A customization task to run on a Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomizationTask {
    #[doc = "Name of the task."]
    pub name: String,
    #[doc = "Parameters for the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Display name to help differentiate multiple instances of the same task."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Timeout, in seconds. Overrides any timeout provided on the task definition."]
    #[serde(rename = "timeoutInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i32>,
    #[doc = "What account to run the task as."]
    #[serde(rename = "runAs", default, skip_serializing_if = "Option::is_none")]
    pub run_as: Option<CustomizationTaskExecutionAccount>,
    #[doc = "ID of the task instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The unique URI for retrieving the task logs."]
    #[serde(rename = "logUri", default, skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    #[doc = "Status of a customization task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CustomizationTaskStatus>,
    #[doc = "Start time of the task, in RFC3339 format."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "End time of the task, in RFC3339 format."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
}
impl CustomizationTask {
    pub fn new(name: String) -> Self {
        Self {
            name,
            parameters: None,
            display_name: None,
            timeout_in_seconds: None,
            run_as: None,
            id: None,
            log_uri: None,
            status: None,
            start_time: None,
            end_time: None,
        }
    }
}
#[doc = "Represents a task to be used in customizing a Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomizationTaskDefinition {
    #[doc = "Full name of the task: {catalogName}/{taskName}."]
    pub name: String,
    #[doc = "Name of the catalog that the task belongs to."]
    #[serde(rename = "catalogName")]
    pub catalog_name: String,
    #[doc = "The unique URI of the customization task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Description of the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Parameters for the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl CustomizationTaskDefinition {
    pub fn new(name: String, catalog_name: String) -> Self {
        Self {
            name,
            catalog_name,
            uri: None,
            description: None,
            parameters: None,
        }
    }
}
#[doc = "Parameters for a customization task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomizationTaskDefinitionParameter {
    #[doc = "Description of the parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Type of the parameter."]
    #[serde(rename = "type")]
    pub type_: CustomizationTaskDefinitionParameterType,
    #[doc = "Whether or not the parameter is required."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[doc = "Default value for the parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[doc = "Allowed values for the parameter."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed: Vec<String>,
}
impl CustomizationTaskDefinitionParameter {
    pub fn new(type_: CustomizationTaskDefinitionParameterType) -> Self {
        Self {
            description: None,
            type_,
            required: None,
            default: None,
            allowed: Vec::new(),
        }
    }
}
#[doc = "Type of the parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CustomizationTaskDefinitionParameterType")]
pub enum CustomizationTaskDefinitionParameterType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CustomizationTaskDefinitionParameterType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CustomizationTaskDefinitionParameterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CustomizationTaskDefinitionParameterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::String => serializer.serialize_unit_variant("CustomizationTaskDefinitionParameterType", 0u32, "string"),
            Self::Number => serializer.serialize_unit_variant("CustomizationTaskDefinitionParameterType", 1u32, "number"),
            Self::Boolean => serializer.serialize_unit_variant("CustomizationTaskDefinitionParameterType", 2u32, "boolean"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "What account to run the task as."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CustomizationTaskExecutionAccount")]
pub enum CustomizationTaskExecutionAccount {
    System,
    User,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CustomizationTaskExecutionAccount {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CustomizationTaskExecutionAccount {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CustomizationTaskExecutionAccount {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::System => serializer.serialize_unit_variant("CustomizationTaskExecutionAccount", 0u32, "System"),
            Self::User => serializer.serialize_unit_variant("CustomizationTaskExecutionAccount", 1u32, "User"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a list of tasks to apply to a Dev Box"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomizationTaskList {
    #[doc = "Tasks to apply."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tasks: Vec<CustomizationTask>,
}
impl CustomizationTaskList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "All of the validation errors for a customization task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomizationTaskListValidationError {
    #[doc = "A customization task to run on a Dev Box."]
    pub target: CustomizationTask,
    #[doc = "List of validation errors for the task."]
    pub details: Vec<AzureCoreFoundationsError>,
}
impl CustomizationTaskListValidationError {
    pub fn new(target: CustomizationTask, details: Vec<AzureCoreFoundationsError>) -> Self {
        Self { target, details }
    }
}
#[doc = "The operation result of validating a list of customization tasks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomizationTaskListValidationOperationResult {
    #[doc = "Fully qualified ID for the operation status."]
    pub id: String,
    #[doc = "Universally Unique Identifier"]
    pub name: AzureCoreUuid,
    #[doc = "Enum describing allowed operation states."]
    pub status: AzureCoreFoundationsOperationState,
    #[doc = "The id of the resource."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The start time of the operation, in RFC3339 format."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "The end time of the operation, in RFC3339 format."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
    #[doc = "Percent of the operation that is complete."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "Custom operation properties, populated only for a successful operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<AzureCoreFoundationsError>,
    #[doc = "The result of validating a list of customization tasks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<CustomizationTaskListValidationResult>,
}
impl CustomizationTaskListValidationOperationResult {
    pub fn new(id: String, name: AzureCoreUuid, status: AzureCoreFoundationsOperationState) -> Self {
        Self {
            id,
            name,
            status,
            resource_id: None,
            start_time: None,
            end_time: None,
            percent_complete: None,
            properties: None,
            error: None,
            result: None,
        }
    }
}
#[doc = "The result of validating a list of customization tasks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomizationTaskListValidationResult {
    #[doc = "Status of validating a list of customization tasks."]
    #[serde(rename = "validationResult")]
    pub validation_result: CustomizationTaskListValidationStatus,
    #[doc = "List of validation errors. Absent if no errors."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<CustomizationTaskListValidationError>,
}
impl CustomizationTaskListValidationResult {
    pub fn new(validation_result: CustomizationTaskListValidationStatus) -> Self {
        Self {
            validation_result,
            errors: Vec::new(),
        }
    }
}
#[doc = "Status of validating a list of customization tasks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CustomizationTaskListValidationStatus")]
pub enum CustomizationTaskListValidationStatus {
    Succeeded,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CustomizationTaskListValidationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CustomizationTaskListValidationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CustomizationTaskListValidationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("CustomizationTaskListValidationStatus", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("CustomizationTaskListValidationStatus", 1u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Status of a customization task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CustomizationTaskStatus")]
pub enum CustomizationTaskStatus {
    NotStarted,
    Running,
    Succeeded,
    FailedValidation,
    Skipped,
    TimedOut,
    Failed,
    WaitingForUserInputUac,
    WaitingForUserSession,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CustomizationTaskStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CustomizationTaskStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CustomizationTaskStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotStarted => serializer.serialize_unit_variant("CustomizationTaskStatus", 0u32, "NotStarted"),
            Self::Running => serializer.serialize_unit_variant("CustomizationTaskStatus", 1u32, "Running"),
            Self::Succeeded => serializer.serialize_unit_variant("CustomizationTaskStatus", 2u32, "Succeeded"),
            Self::FailedValidation => serializer.serialize_unit_variant("CustomizationTaskStatus", 3u32, "FailedValidation"),
            Self::Skipped => serializer.serialize_unit_variant("CustomizationTaskStatus", 4u32, "Skipped"),
            Self::TimedOut => serializer.serialize_unit_variant("CustomizationTaskStatus", 5u32, "TimedOut"),
            Self::Failed => serializer.serialize_unit_variant("CustomizationTaskStatus", 6u32, "Failed"),
            Self::WaitingForUserInputUac => serializer.serialize_unit_variant("CustomizationTaskStatus", 7u32, "WaitingForUserInputUac"),
            Self::WaitingForUserSession => serializer.serialize_unit_variant("CustomizationTaskStatus", 8u32, "WaitingForUserSession"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBox {
    #[doc = "The unique URI of the dev box."]
    pub uri: String,
    #[doc = "Display name for the Dev Box."]
    pub name: String,
    #[doc = "Name of the project this Dev Box belongs to."]
    #[serde(rename = "projectName", default, skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[doc = "The name of the Dev Box pool this machine belongs to."]
    #[serde(rename = "poolName")]
    pub pool_name: String,
    #[doc = "Indicates whether hibernate is supported and enabled, disabled, or unsupported by the operating system. Unknown hibernate support is represented as null."]
    #[serde(rename = "hibernateSupport", default, skip_serializing_if = "Option::is_none")]
    pub hibernate_support: Option<HibernateSupport>,
    #[doc = "Indicates the provisioning state of the Dev Box."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevBoxProvisioningState>,
    #[doc = "The current action state of the Dev Box. This is state is based on previous\naction performed by user."]
    #[serde(rename = "actionState", default, skip_serializing_if = "Option::is_none")]
    pub action_state: Option<String>,
    #[doc = "The power states of a Dev Box."]
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<PowerState>,
    #[doc = "Universally Unique Identifier"]
    #[serde(rename = "uniqueId", default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<AzureCoreUuid>,
    #[doc = "The error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<AzureCoreFoundationsError>,
    #[doc = "Represents an Azure geography region where supported resource providers live."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<AzureCoreAzureLocation>,
    #[doc = "The operating system type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "Universally Unique Identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<AzureCoreUuid>,
    #[doc = "Hardware specifications for the Dev Box."]
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[doc = "Storage settings for the Dev Box's disks."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "Specifies information about the image used."]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
    #[doc = "Creation time of this Dev Box, in RFC3339 format."]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<::time::OffsetDateTime>,
    #[doc = "Indicates whether owners of Dev Boxes in a pool are local administrators on the Dev Boxes."]
    #[serde(rename = "localAdministrator", default, skip_serializing_if = "Option::is_none")]
    pub local_administrator: Option<LocalAdminStatus>,
}
impl DevBox {
    pub fn new(uri: String, name: String, pool_name: String) -> Self {
        Self {
            uri,
            name,
            project_name: None,
            pool_name,
            hibernate_support: None,
            provisioning_state: None,
            action_state: None,
            power_state: None,
            unique_id: None,
            error: None,
            location: None,
            os_type: None,
            user: None,
            hardware_profile: None,
            storage_profile: None,
            image_reference: None,
            created_time: None,
            local_administrator: None,
        }
    }
}
#[doc = "An action which will take place on a Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBoxAction {
    #[doc = "The unique URI for the Dev Box action."]
    pub uri: String,
    #[doc = "The name of the action."]
    pub name: String,
    #[doc = "The type of action which will take place on a Dev Box."]
    #[serde(rename = "actionType")]
    pub action_type: DevBoxActionType,
    #[doc = "The id of the resource which triggered this action."]
    #[serde(rename = "sourceId")]
    pub source_id: String,
    #[doc = "The URI of the resource which triggered this action."]
    #[serde(rename = "sourceUri")]
    pub source_uri: String,
    #[doc = "The type of the resource which triggered the action."]
    #[serde(rename = "sourceType")]
    pub source_type: DevBoxActionSourceType,
    #[doc = "The earliest time that the action could occur (UTC), in RFC3339 format."]
    #[serde(rename = "suspendedUntil", default, with = "azure_core::date::rfc3339::option")]
    pub suspended_until: Option<::time::OffsetDateTime>,
    #[doc = "Details about the next run of an action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<DevBoxNextAction>,
}
impl DevBoxAction {
    pub fn new(
        uri: String,
        name: String,
        action_type: DevBoxActionType,
        source_id: String,
        source_uri: String,
        source_type: DevBoxActionSourceType,
    ) -> Self {
        Self {
            uri,
            name,
            action_type,
            source_id,
            source_uri,
            source_type,
            suspended_until: None,
            next: None,
        }
    }
}
#[doc = "The action delay result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBoxActionDelayResult {
    #[doc = "The unique URI of the action."]
    pub uri: String,
    #[doc = "The name of the action."]
    pub name: String,
    #[doc = "The result of the delay operation on this action."]
    pub result: DevBoxActionDelayResultStatus,
    #[doc = "An action which will take place on a Dev Box."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<DevBoxAction>,
    #[doc = "The error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<AzureCoreFoundationsError>,
}
impl DevBoxActionDelayResult {
    pub fn new(uri: String, name: String, result: DevBoxActionDelayResultStatus) -> Self {
        Self {
            uri,
            name,
            result,
            action: None,
            error: None,
        }
    }
}
#[doc = "The result of the delay operation on this action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DevBoxActionDelayResultStatus")]
pub enum DevBoxActionDelayResultStatus {
    Succeeded,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DevBoxActionDelayResultStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DevBoxActionDelayResultStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DevBoxActionDelayResultStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("DevBoxActionDelayResultStatus", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("DevBoxActionDelayResultStatus", 1u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type of the resource which triggered the action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DevBoxActionSourceType")]
pub enum DevBoxActionSourceType {
    Pool,
    Schedule,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DevBoxActionSourceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DevBoxActionSourceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DevBoxActionSourceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pool => serializer.serialize_unit_variant("DevBoxActionSourceType", 0u32, "Pool"),
            Self::Schedule => serializer.serialize_unit_variant("DevBoxActionSourceType", 1u32, "Schedule"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type of action which will take place on a Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DevBoxActionType")]
pub enum DevBoxActionType {
    Stop,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DevBoxActionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DevBoxActionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DevBoxActionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Stop => serializer.serialize_unit_variant("DevBoxActionType", 0u32, "Stop"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Details about the next run of an action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBoxNextAction {
    #[doc = "The time the action will be triggered (UTC), in RFC3339 format."]
    #[serde(rename = "scheduledTime", with = "azure_core::date::rfc3339")]
    pub scheduled_time: ::time::OffsetDateTime,
}
impl DevBoxNextAction {
    pub fn new(scheduled_time: ::time::OffsetDateTime) -> Self {
        Self { scheduled_time }
    }
}
#[doc = "Information about an operation on a Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBoxOperation {
    #[doc = "The unique URI for the Dev Box operation."]
    pub uri: String,
    #[doc = "Universally Unique Identifier"]
    #[serde(rename = "operationId")]
    pub operation_id: AzureCoreUuid,
    #[doc = "Enum describing allowed operation states."]
    pub status: AzureCoreFoundationsOperationState,
    #[doc = "Universally Unique Identifier"]
    #[serde(rename = "createdByObjectId", default, skip_serializing_if = "Option::is_none")]
    pub created_by_object_id: Option<AzureCoreUuid>,
    #[doc = "he time the operation started, in RFC3339 format."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "The time the operation finished, in RFC3339 format."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
    #[doc = "The error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<AzureCoreFoundationsError>,
}
impl DevBoxOperation {
    pub fn new(uri: String, operation_id: AzureCoreUuid, status: AzureCoreFoundationsOperationState) -> Self {
        Self {
            uri,
            operation_id,
            status,
            created_by_object_id: None,
            start_time: None,
            end_time: None,
            error: None,
        }
    }
}
#[doc = "The type of Dev Box operation."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum DevBoxOperationUnion {
    Repair(DevBoxRepairOperation),
    Restart(DevBoxRestartOperation),
    RestoreSnapshot(DevBoxRestoreSnapshotOperation),
    Start(DevBoxStartOperation),
    Stop(DevBoxStopOperation),
}
#[doc = "The type of Dev Box operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DevBoxOperationKind")]
pub enum DevBoxOperationKind {
    Start,
    Stop,
    Restart,
    Repair,
    RestoreSnapshot,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DevBoxOperationKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DevBoxOperationKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DevBoxOperationKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Start => serializer.serialize_unit_variant("DevBoxOperationKind", 0u32, "Start"),
            Self::Stop => serializer.serialize_unit_variant("DevBoxOperationKind", 1u32, "Stop"),
            Self::Restart => serializer.serialize_unit_variant("DevBoxOperationKind", 2u32, "Restart"),
            Self::Repair => serializer.serialize_unit_variant("DevBoxOperationKind", 3u32, "Repair"),
            Self::RestoreSnapshot => serializer.serialize_unit_variant("DevBoxOperationKind", 4u32, "RestoreSnapshot"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates the provisioning state of the Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DevBoxProvisioningState")]
pub enum DevBoxProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Creating,
    Deleting,
    Updating,
    Starting,
    Stopping,
    Provisioning,
    ProvisionedWithWarning,
    InGracePeriod,
    NotProvisioned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DevBoxProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DevBoxProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DevBoxProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("DevBoxProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("DevBoxProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("DevBoxProvisioningState", 2u32, "Canceled"),
            Self::Creating => serializer.serialize_unit_variant("DevBoxProvisioningState", 3u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("DevBoxProvisioningState", 4u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("DevBoxProvisioningState", 5u32, "Updating"),
            Self::Starting => serializer.serialize_unit_variant("DevBoxProvisioningState", 6u32, "Starting"),
            Self::Stopping => serializer.serialize_unit_variant("DevBoxProvisioningState", 7u32, "Stopping"),
            Self::Provisioning => serializer.serialize_unit_variant("DevBoxProvisioningState", 8u32, "Provisioning"),
            Self::ProvisionedWithWarning => serializer.serialize_unit_variant("DevBoxProvisioningState", 9u32, "ProvisionedWithWarning"),
            Self::InGracePeriod => serializer.serialize_unit_variant("DevBoxProvisioningState", 10u32, "InGracePeriod"),
            Self::NotProvisioned => serializer.serialize_unit_variant("DevBoxProvisioningState", 11u32, "NotProvisioned"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information about a start operation on a Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBoxRepairOperation {
    #[serde(flatten)]
    pub dev_box_operation: DevBoxOperation,
    #[doc = "Information about the result of a repair operation on a Dev Box."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<DevBoxRepairOperationResult>,
}
impl DevBoxRepairOperation {
    pub fn new(dev_box_operation: DevBoxOperation) -> Self {
        Self {
            dev_box_operation,
            result: None,
        }
    }
}
#[doc = "Information about the result of a repair operation on a Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevBoxRepairOperationResult {
    #[doc = "The action taken during a repair operation."]
    #[serde(rename = "repairOutcome", default, skip_serializing_if = "Option::is_none")]
    pub repair_outcome: Option<DevBoxRepairOutcome>,
    #[doc = "The result code associated with the repair operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The result message associated with the repair operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl DevBoxRepairOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The action taken during a repair operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DevBoxRepairOutcome")]
pub enum DevBoxRepairOutcome {
    FixApplied,
    IssuesDetected,
    NoIssuesDetected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DevBoxRepairOutcome {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DevBoxRepairOutcome {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DevBoxRepairOutcome {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::FixApplied => serializer.serialize_unit_variant("DevBoxRepairOutcome", 0u32, "FixApplied"),
            Self::IssuesDetected => serializer.serialize_unit_variant("DevBoxRepairOutcome", 1u32, "IssuesDetected"),
            Self::NoIssuesDetected => serializer.serialize_unit_variant("DevBoxRepairOutcome", 2u32, "NoIssuesDetected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information about a restart operation on a Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBoxRestartOperation {
    #[serde(flatten)]
    pub dev_box_operation: DevBoxOperation,
}
impl DevBoxRestartOperation {
    pub fn new(dev_box_operation: DevBoxOperation) -> Self {
        Self { dev_box_operation }
    }
}
#[doc = "Information about a restore from snapshot operation on a Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBoxRestoreSnapshotOperation {
    #[serde(flatten)]
    pub dev_box_operation: DevBoxOperation,
    #[doc = "Specifies the snapshot id that was used for the restore operation."]
    #[serde(rename = "snapshotId")]
    pub snapshot_id: String,
}
impl DevBoxRestoreSnapshotOperation {
    pub fn new(dev_box_operation: DevBoxOperation, snapshot_id: String) -> Self {
        Self {
            dev_box_operation,
            snapshot_id,
        }
    }
}
#[doc = "A DevBox snapshot"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBoxSnapshot {
    #[doc = "The id of the snapshot. Should be treated as opaque string."]
    #[serde(rename = "snapshotId")]
    pub snapshot_id: String,
    #[doc = "The datetime that the snapshot was created, in RFC3339 format."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339")]
    pub created_time: ::time::OffsetDateTime,
}
impl DevBoxSnapshot {
    pub fn new(snapshot_id: String, created_time: ::time::OffsetDateTime) -> Self {
        Self { snapshot_id, created_time }
    }
}
#[doc = "Information about a start operation on a Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBoxStartOperation {
    #[serde(flatten)]
    pub dev_box_operation: DevBoxOperation,
}
impl DevBoxStartOperation {
    pub fn new(dev_box_operation: DevBoxOperation) -> Self {
        Self { dev_box_operation }
    }
}
#[doc = "Information about a stop operation on a Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBoxStopOperation {
    #[serde(flatten)]
    pub dev_box_operation: DevBoxOperation,
}
impl DevBoxStopOperation {
    pub fn new(dev_box_operation: DevBoxOperation) -> Self {
        Self { dev_box_operation }
    }
}
#[doc = "Properties of an environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Environment {
    #[doc = "The time the expiration date will be triggered (UTC), after which the\nenvironment and associated resources will be deleted."]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<::time::OffsetDateTime>,
    #[doc = "Parameters object for the environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "The unique URI of the environment."]
    pub uri: String,
    #[doc = "Environment name."]
    pub name: String,
    #[doc = "Environment type."]
    #[serde(rename = "environmentType")]
    pub environment_type: String,
    #[doc = "Universally Unique Identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<AzureCoreUuid>,
    #[doc = "The provisioning state of the environment."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<EnvironmentProvisioningState>,
    #[doc = "The identifier of the resource group containing the environment's resources."]
    #[serde(rename = "resourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    #[doc = "Name of the catalog."]
    #[serde(rename = "catalogName")]
    pub catalog_name: String,
    #[doc = "Name of the environment definition."]
    #[serde(rename = "environmentDefinitionName")]
    pub environment_definition_name: String,
    #[doc = "The error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<AzureCoreFoundationsError>,
}
impl Environment {
    pub fn new(uri: String, name: String, environment_type: String, catalog_name: String, environment_definition_name: String) -> Self {
        Self {
            expiration_date: None,
            parameters: None,
            uri,
            name,
            environment_type,
            user: None,
            provisioning_state: None,
            resource_group_id: None,
            catalog_name,
            environment_definition_name,
            error: None,
        }
    }
}
#[doc = "An upcoming Environment Action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentAction {
    #[doc = "Uniquely identifies the action."]
    pub name: String,
    #[doc = "Uri of the action resource."]
    pub uri: String,
    #[doc = "The scheduled action types."]
    #[serde(rename = "actionType")]
    pub action_type: EnvironmentActionType,
    #[doc = "Details about the next run of an action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<EnvironmentNextAction>,
    #[doc = "Universally Unique Identifier"]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<AzureCoreUuid>,
    #[doc = "Time the object was last modified, in RFC3339 format."]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<::time::OffsetDateTime>,
}
impl EnvironmentAction {
    pub fn new(name: String, uri: String, action_type: EnvironmentActionType) -> Self {
        Self {
            name,
            uri,
            action_type,
            next: None,
            last_modified_by: None,
            last_modified_at: None,
        }
    }
}
#[doc = "The scheduled action types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnvironmentActionType")]
pub enum EnvironmentActionType {
    Delete,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnvironmentActionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnvironmentActionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnvironmentActionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Delete => serializer.serialize_unit_variant("EnvironmentActionType", 0u32, "Delete"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An environment definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentDefinition {
    #[doc = "The unique URI of the environment definition."]
    pub uri: String,
    #[doc = "The ID of the environment definition."]
    pub id: String,
    #[doc = "Name of the environment definition."]
    pub name: String,
    #[doc = "Name of the catalog."]
    #[serde(rename = "catalogName")]
    pub catalog_name: String,
    #[doc = "A short description of the environment definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Input parameters passed to an environment."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parameters: Vec<EnvironmentDefinitionParameter>,
    #[doc = "JSON schema defining the parameters object passed to an environment."]
    #[serde(rename = "parametersSchema", default, skip_serializing_if = "Option::is_none")]
    pub parameters_schema: Option<String>,
    #[doc = "Path to the Environment Definition entrypoint file."]
    #[serde(rename = "templatePath", default, skip_serializing_if = "Option::is_none")]
    pub template_path: Option<String>,
}
impl EnvironmentDefinition {
    pub fn new(uri: String, id: String, name: String, catalog_name: String) -> Self {
        Self {
            uri,
            id,
            name,
            catalog_name,
            description: None,
            parameters: Vec::new(),
            parameters_schema: None,
            template_path: None,
        }
    }
}
#[doc = "Properties of an Environment Definition parameter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentDefinitionParameter {
    #[doc = "Unique ID of the parameter."]
    pub id: String,
    #[doc = "Display name of the parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Description of the parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Default value of the parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[doc = "The type of data a parameter accepts."]
    #[serde(rename = "type")]
    pub type_: ParameterType,
    #[doc = "Whether or not this parameter is read-only.  If true, default should have a\nvalue."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "Whether or not this parameter is required."]
    pub required: bool,
    #[doc = "An array of allowed values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed: Vec<String>,
}
impl EnvironmentDefinitionParameter {
    pub fn new(id: String, type_: ParameterType, required: bool) -> Self {
        Self {
            id,
            name: None,
            description: None,
            default: None,
            type_,
            read_only: None,
            required,
            allowed: Vec::new(),
        }
    }
}
#[doc = "Information about a delete operation on an environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentDeleteOperation {
    #[serde(flatten)]
    pub environment_operation: EnvironmentOperation,
}
impl EnvironmentDeleteOperation {
    pub fn new(environment_operation: EnvironmentOperation) -> Self {
        Self { environment_operation }
    }
}
#[doc = "Information about a deploy operation on an environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentDeployOperation {
    #[serde(flatten)]
    pub environment_operation: EnvironmentOperation,
}
impl EnvironmentDeployOperation {
    pub fn new(environment_operation: EnvironmentOperation) -> Self {
        Self { environment_operation }
    }
}
#[doc = "Details about the next run of an action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentNextAction {
    #[doc = "The time the action will be triggered (UTC), in RFC3339 format."]
    #[serde(rename = "scheduledTime", default, with = "azure_core::date::rfc3339::option")]
    pub scheduled_time: Option<::time::OffsetDateTime>,
}
impl EnvironmentNextAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about an operation on an environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentOperation {
    #[doc = "The unique URI for the environment operation."]
    pub uri: String,
    #[doc = "Universally Unique Identifier"]
    #[serde(rename = "operationId")]
    pub operation_id: AzureCoreUuid,
    #[doc = "Enum describing allowed operation states."]
    pub status: AzureCoreFoundationsOperationState,
    #[doc = "Universally Unique Identifier"]
    #[serde(rename = "createdByObjectId", default, skip_serializing_if = "Option::is_none")]
    pub created_by_object_id: Option<AzureCoreUuid>,
    #[doc = "The time the operation started, in RFC3339 format."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "The time the operation finished, in RFC3339 format."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
    #[doc = "Parameters object for the environment at the time of the operation."]
    #[serde(rename = "environmentParameters", default, skip_serializing_if = "Option::is_none")]
    pub environment_parameters: Option<serde_json::Value>,
    #[doc = "The error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<AzureCoreFoundationsError>,
}
impl EnvironmentOperation {
    pub fn new(uri: String, operation_id: AzureCoreUuid, status: AzureCoreFoundationsOperationState) -> Self {
        Self {
            uri,
            operation_id,
            status,
            created_by_object_id: None,
            start_time: None,
            end_time: None,
            environment_parameters: None,
            error: None,
        }
    }
}
#[doc = "The type of environment operation."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum EnvironmentOperationUnion {
    Delete(EnvironmentDeleteOperation),
    Deploy(EnvironmentDeployOperation),
}
#[doc = "The type of environment operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnvironmentOperationKind")]
pub enum EnvironmentOperationKind {
    Deploy,
    Delete,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnvironmentOperationKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnvironmentOperationKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnvironmentOperationKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Deploy => serializer.serialize_unit_variant("EnvironmentOperationKind", 0u32, "Deploy"),
            Self::Delete => serializer.serialize_unit_variant("EnvironmentOperationKind", 1u32, "Delete"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Output from environment deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentOutput {
    #[doc = "Type of the output value."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<EnvironmentOutputType>,
    #[doc = "The output value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[doc = "Indicates if the value is sensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
}
impl EnvironmentOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the output value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnvironmentOutputType")]
pub enum EnvironmentOutputType {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "string")]
    String,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnvironmentOutputType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnvironmentOutputType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnvironmentOutputType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Array => serializer.serialize_unit_variant("EnvironmentOutputType", 0u32, "array"),
            Self::Boolean => serializer.serialize_unit_variant("EnvironmentOutputType", 1u32, "boolean"),
            Self::Number => serializer.serialize_unit_variant("EnvironmentOutputType", 2u32, "number"),
            Self::Object => serializer.serialize_unit_variant("EnvironmentOutputType", 3u32, "object"),
            Self::String => serializer.serialize_unit_variant("EnvironmentOutputType", 4u32, "string"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Outputs from environment deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentOutputs {
    #[doc = "The outputs Names and Values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
}
impl EnvironmentOutputs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The provisioning state of the environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnvironmentProvisioningState")]
pub enum EnvironmentProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Creating,
    Accepted,
    Deleting,
    Updating,
    Preparing,
    Running,
    Syncing,
    MovingResources,
    TransientFailure,
    StorageProvisioningFailed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnvironmentProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnvironmentProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnvironmentProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("EnvironmentProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("EnvironmentProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("EnvironmentProvisioningState", 2u32, "Canceled"),
            Self::Creating => serializer.serialize_unit_variant("EnvironmentProvisioningState", 3u32, "Creating"),
            Self::Accepted => serializer.serialize_unit_variant("EnvironmentProvisioningState", 4u32, "Accepted"),
            Self::Deleting => serializer.serialize_unit_variant("EnvironmentProvisioningState", 5u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("EnvironmentProvisioningState", 6u32, "Updating"),
            Self::Preparing => serializer.serialize_unit_variant("EnvironmentProvisioningState", 7u32, "Preparing"),
            Self::Running => serializer.serialize_unit_variant("EnvironmentProvisioningState", 8u32, "Running"),
            Self::Syncing => serializer.serialize_unit_variant("EnvironmentProvisioningState", 9u32, "Syncing"),
            Self::MovingResources => serializer.serialize_unit_variant("EnvironmentProvisioningState", 10u32, "MovingResources"),
            Self::TransientFailure => serializer.serialize_unit_variant("EnvironmentProvisioningState", 11u32, "TransientFailure"),
            Self::StorageProvisioningFailed => {
                serializer.serialize_unit_variant("EnvironmentProvisioningState", 12u32, "StorageProvisioningFailed")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties of an environment type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentType {
    #[doc = "The unique URI of the environment type."]
    pub uri: String,
    #[doc = "Name of the environment type."]
    pub name: String,
    #[doc = "Id of a subscription or management group that the environment type will be\nmapped to. The environment's resources will be deployed into this subscription\nor management group."]
    #[serde(rename = "deploymentTargetId")]
    pub deployment_target_id: String,
    #[doc = "Indicates whether an environment type is enabled for use in a project."]
    pub status: EnvironmentTypeEnableStatus,
    #[doc = "Display name of the environment type."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl EnvironmentType {
    pub fn new(uri: String, name: String, deployment_target_id: String, status: EnvironmentTypeEnableStatus) -> Self {
        Self {
            uri,
            name,
            deployment_target_id,
            status,
            display_name: None,
        }
    }
}
#[doc = "Abilities a user has on an environment type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentTypeAbilities {
    #[doc = "The abilities the user has to perform actions on the environment type as an admin."]
    #[serde(rename = "abilitiesAsAdmin")]
    pub abilities_as_admin: Vec<EnvironmentTypeAbilityAsAdmin>,
    #[doc = "The abilities the user has to perform actions on the environment type as a developer."]
    #[serde(rename = "abilitiesAsDeveloper")]
    pub abilities_as_developer: Vec<EnvironmentTypeAbilityAsDeveloper>,
}
impl EnvironmentTypeAbilities {
    pub fn new(
        abilities_as_admin: Vec<EnvironmentTypeAbilityAsAdmin>,
        abilities_as_developer: Vec<EnvironmentTypeAbilityAsDeveloper>,
    ) -> Self {
        Self {
            abilities_as_admin,
            abilities_as_developer,
        }
    }
}
#[doc = "An ability the user has to perform an action on the environment type as an admin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnvironmentTypeAbilityAsAdmin")]
pub enum EnvironmentTypeAbilityAsAdmin {
    DeleteEnvironments,
    ManageEnvironmentActions,
    ReadEnvironmentActions,
    ReadEnvironmentOutputs,
    ReadEnvironments,
    WriteEnvironments,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnvironmentTypeAbilityAsAdmin {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnvironmentTypeAbilityAsAdmin {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnvironmentTypeAbilityAsAdmin {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DeleteEnvironments => serializer.serialize_unit_variant("EnvironmentTypeAbilityAsAdmin", 0u32, "DeleteEnvironments"),
            Self::ManageEnvironmentActions => {
                serializer.serialize_unit_variant("EnvironmentTypeAbilityAsAdmin", 1u32, "ManageEnvironmentActions")
            }
            Self::ReadEnvironmentActions => {
                serializer.serialize_unit_variant("EnvironmentTypeAbilityAsAdmin", 2u32, "ReadEnvironmentActions")
            }
            Self::ReadEnvironmentOutputs => {
                serializer.serialize_unit_variant("EnvironmentTypeAbilityAsAdmin", 3u32, "ReadEnvironmentOutputs")
            }
            Self::ReadEnvironments => serializer.serialize_unit_variant("EnvironmentTypeAbilityAsAdmin", 4u32, "ReadEnvironments"),
            Self::WriteEnvironments => serializer.serialize_unit_variant("EnvironmentTypeAbilityAsAdmin", 5u32, "WriteEnvironments"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An ability the user has to perform an action on the environment type as a developer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnvironmentTypeAbilityAsDeveloper")]
pub enum EnvironmentTypeAbilityAsDeveloper {
    DeleteEnvironments,
    ManageEnvironmentActions,
    ReadEnvironmentActions,
    ReadEnvironmentOutputs,
    ReadEnvironments,
    WriteEnvironments,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnvironmentTypeAbilityAsDeveloper {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnvironmentTypeAbilityAsDeveloper {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnvironmentTypeAbilityAsDeveloper {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DeleteEnvironments => serializer.serialize_unit_variant("EnvironmentTypeAbilityAsDeveloper", 0u32, "DeleteEnvironments"),
            Self::ManageEnvironmentActions => {
                serializer.serialize_unit_variant("EnvironmentTypeAbilityAsDeveloper", 1u32, "ManageEnvironmentActions")
            }
            Self::ReadEnvironmentActions => {
                serializer.serialize_unit_variant("EnvironmentTypeAbilityAsDeveloper", 2u32, "ReadEnvironmentActions")
            }
            Self::ReadEnvironmentOutputs => {
                serializer.serialize_unit_variant("EnvironmentTypeAbilityAsDeveloper", 3u32, "ReadEnvironmentOutputs")
            }
            Self::ReadEnvironments => serializer.serialize_unit_variant("EnvironmentTypeAbilityAsDeveloper", 4u32, "ReadEnvironments"),
            Self::WriteEnvironments => serializer.serialize_unit_variant("EnvironmentTypeAbilityAsDeveloper", 5u32, "WriteEnvironments"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates whether an environment type is enabled for use in a project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnvironmentTypeEnableStatus")]
pub enum EnvironmentTypeEnableStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnvironmentTypeEnableStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnvironmentTypeEnableStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnvironmentTypeEnableStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("EnvironmentTypeEnableStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("EnvironmentTypeEnableStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties of an environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentUpdate {
    #[doc = "The time the expiration date will be triggered (UTC), after which the\nenvironment and associated resources will be deleted."]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<::time::OffsetDateTime>,
    #[doc = "Parameters object for the environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl EnvironmentUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Hardware specifications for the Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HardwareProfile {
    #[doc = "Indicates the Dev Box compute."]
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<SkuName>,
    #[doc = "The number of vCPUs available for the Dev Box."]
    #[serde(rename = "vCPUs", default, skip_serializing_if = "Option::is_none")]
    pub v_cp_us: Option<i32>,
    #[doc = "The amount of memory available for the Dev Box."]
    #[serde(rename = "memoryGB", default, skip_serializing_if = "Option::is_none")]
    pub memory_gb: Option<i32>,
}
impl HardwareProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates whether hibernate is supported and enabled, disabled, or unsupported by the operating system. Unknown hibernate support is represented as null."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HibernateSupport")]
pub enum HibernateSupport {
    Enabled,
    Disabled,
    OsUnsupported,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HibernateSupport {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HibernateSupport {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HibernateSupport {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("HibernateSupport", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("HibernateSupport", 1u32, "Disabled"),
            Self::OsUnsupported => serializer.serialize_unit_variant("HibernateSupport", 2u32, "OsUnsupported"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies information about the image used."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageReference {
    #[doc = "The name of the image used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The version of the image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The operating system of the image."]
    #[serde(rename = "operatingSystem", default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[doc = "The operating system build number of the image."]
    #[serde(rename = "osBuildNumber", default, skip_serializing_if = "Option::is_none")]
    pub os_build_number: Option<String>,
    #[doc = "The datetime that the backing image version was published, in RFC3339 format."]
    #[serde(rename = "publishedDate", default, with = "azure_core::date::rfc3339::option")]
    pub published_date: Option<::time::OffsetDateTime>,
}
impl ImageReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates whether owners of Dev Boxes in a pool are local administrators on the Dev Boxes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LocalAdminStatus")]
pub enum LocalAdminStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LocalAdminStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LocalAdminStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LocalAdminStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("LocalAdminStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("LocalAdminStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The current status of an async operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatus {
    #[doc = "Fully qualified ID for the operation status."]
    pub id: String,
    #[doc = "Universally Unique Identifier"]
    pub name: AzureCoreUuid,
    #[doc = "Enum describing allowed operation states."]
    pub status: AzureCoreFoundationsOperationState,
    #[doc = "The id of the resource."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The start time of the operation, in RFC3339 format."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "The end time of the operation, in RFC3339 format."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
    #[doc = "Percent of the operation that is complete."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "Custom operation properties, populated only for a successful operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<AzureCoreFoundationsError>,
}
impl OperationStatus {
    pub fn new(id: String, name: AzureCoreUuid, status: AzureCoreFoundationsOperationState) -> Self {
        Self {
            id,
            name,
            status,
            resource_id: None,
            start_time: None,
            end_time: None,
            percent_complete: None,
            properties: None,
            error: None,
        }
    }
}
#[doc = "Settings for the operating system disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsDisk {
    #[doc = "The size of the OS Disk in gigabytes."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
}
impl OsDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operating system type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OsType")]
pub enum OsType {
    Windows,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OsType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OsType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OsType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Windows => serializer.serialize_unit_variant("OsType", 0u32, "Windows"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Paged collection of Catalog items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedCatalog {
    #[doc = "The Catalog items on this page"]
    pub value: Vec<Catalog>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedCatalog {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedCatalog {
    pub fn new(value: Vec<Catalog>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of CustomizationGroup items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedCustomizationGroup {
    #[doc = "The CustomizationGroup items on this page"]
    pub value: Vec<CustomizationGroup>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedCustomizationGroup {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedCustomizationGroup {
    pub fn new(value: Vec<CustomizationGroup>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The Customization Task list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedCustomizationTaskDefinition {
    #[doc = "The CustomizationTaskDefinition items on this page"]
    pub value: Vec<CustomizationTaskDefinition>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedCustomizationTaskDefinition {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedCustomizationTaskDefinition {
    pub fn new(value: Vec<CustomizationTaskDefinition>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of DevBox items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedDevBox {
    #[doc = "The DevBox items on this page"]
    pub value: Vec<DevBox>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedDevBox {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedDevBox {
    pub fn new(value: Vec<DevBox>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of DevBoxAction items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedDevBoxAction {
    #[doc = "The DevBoxAction items on this page"]
    pub value: Vec<DevBoxAction>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedDevBoxAction {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedDevBoxAction {
    pub fn new(value: Vec<DevBoxAction>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of DevBoxActionDelayResult items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedDevBoxActionDelayResult {
    #[doc = "The DevBoxActionDelayResult items on this page"]
    pub value: Vec<DevBoxActionDelayResult>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedDevBoxActionDelayResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedDevBoxActionDelayResult {
    pub fn new(value: Vec<DevBoxActionDelayResult>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of DevBoxOperation items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedDevBoxOperation {
    #[doc = "The DevBoxOperation items on this page"]
    pub value: Vec<DevBoxOperationUnion>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedDevBoxOperation {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedDevBoxOperation {
    pub fn new(value: Vec<DevBoxOperationUnion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of DevBoxSnapshot items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedDevBoxSnapshot {
    #[doc = "The DevBoxSnapshot items on this page"]
    pub value: Vec<DevBoxSnapshot>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedDevBoxSnapshot {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedDevBoxSnapshot {
    pub fn new(value: Vec<DevBoxSnapshot>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of Environment items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedEnvironment {
    #[doc = "The Environment items on this page"]
    pub value: Vec<Environment>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedEnvironment {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedEnvironment {
    pub fn new(value: Vec<Environment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of EnvironmentAction items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedEnvironmentAction {
    #[doc = "The EnvironmentAction items on this page"]
    pub value: Vec<EnvironmentAction>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedEnvironmentAction {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedEnvironmentAction {
    pub fn new(value: Vec<EnvironmentAction>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of EnvironmentDefinition items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedEnvironmentDefinition {
    #[doc = "The EnvironmentDefinition items on this page"]
    pub value: Vec<EnvironmentDefinition>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedEnvironmentDefinition {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedEnvironmentDefinition {
    pub fn new(value: Vec<EnvironmentDefinition>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of EnvironmentOperation items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedEnvironmentOperation {
    #[doc = "The EnvironmentOperation items on this page"]
    pub value: Vec<EnvironmentOperationUnion>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedEnvironmentOperation {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedEnvironmentOperation {
    pub fn new(value: Vec<EnvironmentOperationUnion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of EnvironmentType items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedEnvironmentType {
    #[doc = "The EnvironmentType items on this page"]
    pub value: Vec<EnvironmentType>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedEnvironmentType {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedEnvironmentType {
    pub fn new(value: Vec<EnvironmentType>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of Pool items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedPool {
    #[doc = "The Pool items on this page"]
    pub value: Vec<Pool>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedPool {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedPool {
    pub fn new(value: Vec<Pool>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of Project items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedProject {
    #[doc = "The Project items on this page"]
    pub value: Vec<Project>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedProject {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedProject {
    pub fn new(value: Vec<Project>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of Schedule items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedSchedule {
    #[doc = "The Schedule items on this page"]
    pub value: Vec<Schedule>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedSchedule {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedSchedule {
    pub fn new(value: Vec<Schedule>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type of data a parameter accepts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ParameterType")]
pub enum ParameterType {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "string")]
    String,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ParameterType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ParameterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ParameterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Array => serializer.serialize_unit_variant("ParameterType", 0u32, "array"),
            Self::Boolean => serializer.serialize_unit_variant("ParameterType", 1u32, "boolean"),
            Self::Integer => serializer.serialize_unit_variant("ParameterType", 2u32, "integer"),
            Self::Number => serializer.serialize_unit_variant("ParameterType", 3u32, "number"),
            Self::Object => serializer.serialize_unit_variant("ParameterType", 4u32, "object"),
            Self::String => serializer.serialize_unit_variant("ParameterType", 5u32, "string"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A pool of Dev Boxes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pool {
    #[doc = "The unique URI of the pool."]
    pub uri: String,
    #[doc = "Pool name."]
    pub name: String,
    #[doc = "Represents an Azure geography region where supported resource providers live."]
    pub location: AzureCoreAzureLocation,
    #[doc = "The operating system type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "Hardware specifications for the Dev Box."]
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[doc = "Indicates whether hibernate is supported and enabled, disabled, or unsupported by the operating system. Unknown hibernate support is represented as null."]
    #[serde(rename = "hibernateSupport", default, skip_serializing_if = "Option::is_none")]
    pub hibernate_support: Option<HibernateSupport>,
    #[doc = "Storage settings for the Dev Box's disks."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "Specifies information about the image used."]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
    #[doc = "Indicates whether owners of Dev Boxes in a pool are local administrators on the Dev Boxes."]
    #[serde(rename = "localAdministrator", default, skip_serializing_if = "Option::is_none")]
    pub local_administrator: Option<LocalAdminStatus>,
    #[doc = "Stop on disconnect configuration settings for Dev Boxes created in this pool."]
    #[serde(rename = "stopOnDisconnect", default, skip_serializing_if = "Option::is_none")]
    pub stop_on_disconnect: Option<StopOnDisconnectConfiguration>,
    #[doc = "Pool status indicating whether a pool is available to create Dev Boxes."]
    #[serde(rename = "healthStatus")]
    pub health_status: PoolHealthStatus,
    #[doc = "Display name of the pool."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl Pool {
    pub fn new(uri: String, name: String, location: AzureCoreAzureLocation, health_status: PoolHealthStatus) -> Self {
        Self {
            uri,
            name,
            location,
            os_type: None,
            hardware_profile: None,
            hibernate_support: None,
            storage_profile: None,
            image_reference: None,
            local_administrator: None,
            stop_on_disconnect: None,
            health_status,
            display_name: None,
        }
    }
}
#[doc = "Pool status indicating whether a pool is available to create Dev Boxes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PoolHealthStatus")]
pub enum PoolHealthStatus {
    Unknown,
    Pending,
    Healthy,
    Warning,
    Unhealthy,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PoolHealthStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PoolHealthStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PoolHealthStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("PoolHealthStatus", 0u32, "Unknown"),
            Self::Pending => serializer.serialize_unit_variant("PoolHealthStatus", 1u32, "Pending"),
            Self::Healthy => serializer.serialize_unit_variant("PoolHealthStatus", 2u32, "Healthy"),
            Self::Warning => serializer.serialize_unit_variant("PoolHealthStatus", 3u32, "Warning"),
            Self::Unhealthy => serializer.serialize_unit_variant("PoolHealthStatus", 4u32, "Unhealthy"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The power states of a Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PowerState")]
pub enum PowerState {
    Unknown,
    Running,
    Deallocated,
    PoweredOff,
    Hibernated,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PowerState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PowerState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PowerState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("PowerState", 0u32, "Unknown"),
            Self::Running => serializer.serialize_unit_variant("PowerState", 1u32, "Running"),
            Self::Deallocated => serializer.serialize_unit_variant("PowerState", 2u32, "Deallocated"),
            Self::PoweredOff => serializer.serialize_unit_variant("PowerState", 3u32, "PoweredOff"),
            Self::Hibernated => serializer.serialize_unit_variant("PowerState", 4u32, "Hibernated"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Project details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    #[doc = "The unique URI of the project."]
    pub uri: String,
    #[doc = "Name of the project."]
    pub name: String,
    #[doc = "Description of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "When specified, indicates the maximum number of Dev Boxes a single user can\ncreate across all pools in the project."]
    #[serde(rename = "maxDevBoxesPerUser", default, skip_serializing_if = "Option::is_none")]
    pub max_dev_boxes_per_user: Option<i32>,
    #[doc = "Display name of the pool."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl Project {
    pub fn new(uri: String, name: String) -> Self {
        Self {
            uri,
            name,
            description: None,
            max_dev_boxes_per_user: None,
            display_name: None,
        }
    }
}
#[doc = "Abilities a user has on a project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectAbilities {
    #[doc = "The abilities the user has to perform actions on the project as an admin."]
    #[serde(rename = "abilitiesAsAdmin")]
    pub abilities_as_admin: Vec<ProjectAbilityAsAdmin>,
    #[doc = "The abilities the user has to perform actions on the project as a developer."]
    #[serde(rename = "abilitiesAsDeveloper")]
    pub abilities_as_developer: Vec<ProjectAbilityAsDeveloper>,
}
impl ProjectAbilities {
    pub fn new(abilities_as_admin: Vec<ProjectAbilityAsAdmin>, abilities_as_developer: Vec<ProjectAbilityAsDeveloper>) -> Self {
        Self {
            abilities_as_admin,
            abilities_as_developer,
        }
    }
}
#[doc = "An ability the user has to perform an action on the project as an admin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProjectAbilityAsAdmin")]
pub enum ProjectAbilityAsAdmin {
    DeleteDevBoxes,
    DeleteEnvironments,
    ManageEnvironmentActions,
    ReadDevBoxes,
    ReadEnvironmentActions,
    ReadEnvironmentOutputs,
    ReadEnvironments,
    StartDevBoxes,
    StopDevBoxes,
    WriteDevBoxes,
    WriteEnvironments,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProjectAbilityAsAdmin {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProjectAbilityAsAdmin {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProjectAbilityAsAdmin {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DeleteDevBoxes => serializer.serialize_unit_variant("ProjectAbilityAsAdmin", 0u32, "DeleteDevBoxes"),
            Self::DeleteEnvironments => serializer.serialize_unit_variant("ProjectAbilityAsAdmin", 1u32, "DeleteEnvironments"),
            Self::ManageEnvironmentActions => serializer.serialize_unit_variant("ProjectAbilityAsAdmin", 2u32, "ManageEnvironmentActions"),
            Self::ReadDevBoxes => serializer.serialize_unit_variant("ProjectAbilityAsAdmin", 3u32, "ReadDevBoxes"),
            Self::ReadEnvironmentActions => serializer.serialize_unit_variant("ProjectAbilityAsAdmin", 4u32, "ReadEnvironmentActions"),
            Self::ReadEnvironmentOutputs => serializer.serialize_unit_variant("ProjectAbilityAsAdmin", 5u32, "ReadEnvironmentOutputs"),
            Self::ReadEnvironments => serializer.serialize_unit_variant("ProjectAbilityAsAdmin", 6u32, "ReadEnvironments"),
            Self::StartDevBoxes => serializer.serialize_unit_variant("ProjectAbilityAsAdmin", 7u32, "StartDevBoxes"),
            Self::StopDevBoxes => serializer.serialize_unit_variant("ProjectAbilityAsAdmin", 8u32, "StopDevBoxes"),
            Self::WriteDevBoxes => serializer.serialize_unit_variant("ProjectAbilityAsAdmin", 9u32, "WriteDevBoxes"),
            Self::WriteEnvironments => serializer.serialize_unit_variant("ProjectAbilityAsAdmin", 10u32, "WriteEnvironments"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An ability the user has to perform an action on the project as a developer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProjectAbilityAsDeveloper")]
pub enum ProjectAbilityAsDeveloper {
    CustomizeDevBoxes,
    DeleteDevBoxes,
    DeleteEnvironments,
    ManageDevBoxActions,
    ManageEnvironmentActions,
    ReadDevBoxActions,
    ReadDevBoxes,
    ReadEnvironmentActions,
    ReadEnvironmentOutputs,
    ReadEnvironments,
    ReadRemoteConnections,
    StartDevBoxes,
    StopDevBoxes,
    WriteDevBoxes,
    WriteEnvironments,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProjectAbilityAsDeveloper {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProjectAbilityAsDeveloper {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProjectAbilityAsDeveloper {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CustomizeDevBoxes => serializer.serialize_unit_variant("ProjectAbilityAsDeveloper", 0u32, "CustomizeDevBoxes"),
            Self::DeleteDevBoxes => serializer.serialize_unit_variant("ProjectAbilityAsDeveloper", 1u32, "DeleteDevBoxes"),
            Self::DeleteEnvironments => serializer.serialize_unit_variant("ProjectAbilityAsDeveloper", 2u32, "DeleteEnvironments"),
            Self::ManageDevBoxActions => serializer.serialize_unit_variant("ProjectAbilityAsDeveloper", 3u32, "ManageDevBoxActions"),
            Self::ManageEnvironmentActions => {
                serializer.serialize_unit_variant("ProjectAbilityAsDeveloper", 4u32, "ManageEnvironmentActions")
            }
            Self::ReadDevBoxActions => serializer.serialize_unit_variant("ProjectAbilityAsDeveloper", 5u32, "ReadDevBoxActions"),
            Self::ReadDevBoxes => serializer.serialize_unit_variant("ProjectAbilityAsDeveloper", 6u32, "ReadDevBoxes"),
            Self::ReadEnvironmentActions => serializer.serialize_unit_variant("ProjectAbilityAsDeveloper", 7u32, "ReadEnvironmentActions"),
            Self::ReadEnvironmentOutputs => serializer.serialize_unit_variant("ProjectAbilityAsDeveloper", 8u32, "ReadEnvironmentOutputs"),
            Self::ReadEnvironments => serializer.serialize_unit_variant("ProjectAbilityAsDeveloper", 9u32, "ReadEnvironments"),
            Self::ReadRemoteConnections => serializer.serialize_unit_variant("ProjectAbilityAsDeveloper", 10u32, "ReadRemoteConnections"),
            Self::StartDevBoxes => serializer.serialize_unit_variant("ProjectAbilityAsDeveloper", 11u32, "StartDevBoxes"),
            Self::StopDevBoxes => serializer.serialize_unit_variant("ProjectAbilityAsDeveloper", 12u32, "StopDevBoxes"),
            Self::WriteDevBoxes => serializer.serialize_unit_variant("ProjectAbilityAsDeveloper", 13u32, "WriteDevBoxes"),
            Self::WriteEnvironments => serializer.serialize_unit_variant("ProjectAbilityAsDeveloper", 14u32, "WriteEnvironments"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Provides remote connection information for a Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemoteConnection {
    #[doc = "URL to open a browser based RDP session."]
    #[serde(rename = "webUrl", default, skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[doc = "Link to open a Remote Desktop session."]
    #[serde(rename = "rdpConnectionUrl", default, skip_serializing_if = "Option::is_none")]
    pub rdp_connection_url: Option<String>,
    #[doc = "Link to open a remote desktop session via a dev box's underlying Cloud PC (This will default to Windows App)."]
    #[serde(rename = "cloudPcConnectionUrl", default, skip_serializing_if = "Option::is_none")]
    pub cloud_pc_connection_url: Option<String>,
}
impl RemoteConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Schedule to execute action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    #[doc = "The unique URI of the schedule."]
    pub uri: String,
    #[doc = "Display name for the Schedule."]
    pub name: String,
    #[doc = "The URI of the resource that this schedule belongs to."]
    #[serde(rename = "sourceUri")]
    pub source_uri: String,
    #[doc = "The type of the resource that this schedule belongs to."]
    #[serde(rename = "sourceType")]
    pub source_type: ScheduleSourceType,
    #[doc = "The supported types for a scheduled task."]
    #[serde(rename = "type")]
    pub type_: ScheduledType,
    #[doc = "The frequency of task execution."]
    pub frequency: ScheduledFrequency,
    #[doc = "The target time to trigger the action. The format is HH:MM."]
    pub time: String,
    #[doc = "The IANA timezone id at which the schedule should execute."]
    #[serde(rename = "timeZone")]
    pub time_zone: String,
}
impl Schedule {
    pub fn new(
        uri: String,
        name: String,
        source_uri: String,
        source_type: ScheduleSourceType,
        type_: ScheduledType,
        frequency: ScheduledFrequency,
        time: String,
        time_zone: String,
    ) -> Self {
        Self {
            uri,
            name,
            source_uri,
            source_type,
            type_,
            frequency,
            time,
            time_zone,
        }
    }
}
#[doc = "The type of the resource that this schedule belongs to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduleSourceType")]
pub enum ScheduleSourceType {
    Pool,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduleSourceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduleSourceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduleSourceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pool => serializer.serialize_unit_variant("ScheduleSourceType", 0u32, "Pool"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The frequency of task execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduledFrequency")]
pub enum ScheduledFrequency {
    Daily,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduledFrequency {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduledFrequency {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduledFrequency {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Daily => serializer.serialize_unit_variant("ScheduledFrequency", 0u32, "Daily"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The supported types for a scheduled task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduledType")]
pub enum ScheduledType {
    StopDevBox,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduledType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduledType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduledType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::StopDevBox => serializer.serialize_unit_variant("ScheduledType", 0u32, "StopDevBox"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates the Dev Box compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SkuName")]
pub enum SkuName {
    #[serde(rename = "general_i_8c32gb256ssd_v2")]
    GeneralI8c32gb256ssdV2,
    #[serde(rename = "general_i_8c32gb512ssd_v2")]
    GeneralI8c32gb512ssdV2,
    #[serde(rename = "general_i_8c32gb1024ssd_v2")]
    GeneralI8c32gb1024ssdV2,
    #[serde(rename = "general_i_8c32gb2048ssd_v2")]
    GeneralI8c32gb2048ssdV2,
    #[serde(rename = "general_i_16c64gb256ssd_v2")]
    GeneralI16c64gb256ssdV2,
    #[serde(rename = "general_i_16c64gb512ssd_v2")]
    GeneralI16c64gb512ssdV2,
    #[serde(rename = "general_i_16c64gb1024ssd_v2")]
    GeneralI16c64gb1024ssdV2,
    #[serde(rename = "general_i_16c64gb2048ssd_v2")]
    GeneralI16c64gb2048ssdV2,
    #[serde(rename = "general_i_32c128gb512ssd_v2")]
    GeneralI32c128gb512ssdV2,
    #[serde(rename = "general_i_32c128gb1024ssd_v2")]
    GeneralI32c128gb1024ssdV2,
    #[serde(rename = "general_i_32c128gb2048ssd_v2")]
    GeneralI32c128gb2048ssdV2,
    #[serde(rename = "general_a_8c32gb256ssd_v2")]
    GeneralA8c32gb256ssdV2,
    #[serde(rename = "general_a_8c32gb512ssd_v2")]
    GeneralA8c32gb512ssdV2,
    #[serde(rename = "general_a_8c32gb1024ssd_v2")]
    GeneralA8c32gb1024ssdV2,
    #[serde(rename = "general_a_8c32gb2048ssd_v2")]
    GeneralA8c32gb2048ssdV2,
    #[serde(rename = "general_a_16c64gb256ssd_v2")]
    GeneralA16c64gb256ssdV2,
    #[serde(rename = "general_a_16c64gb512ssd_v2")]
    GeneralA16c64gb512ssdV2,
    #[serde(rename = "general_a_16c64gb1024ssd_v2")]
    GeneralA16c64gb1024ssdV2,
    #[serde(rename = "general_a_16c64gb2048ssd_v2")]
    GeneralA16c64gb2048ssdV2,
    #[serde(rename = "general_a_32c128gb512ssd_v2")]
    GeneralA32c128gb512ssdV2,
    #[serde(rename = "general_a_32c128gb1024ssd_v2")]
    GeneralA32c128gb1024ssdV2,
    #[serde(rename = "general_a_32c128gb2048ssd_v2")]
    GeneralA32c128gb2048ssdV2,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SkuName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SkuName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SkuName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::GeneralI8c32gb256ssdV2 => serializer.serialize_unit_variant("SkuName", 0u32, "general_i_8c32gb256ssd_v2"),
            Self::GeneralI8c32gb512ssdV2 => serializer.serialize_unit_variant("SkuName", 1u32, "general_i_8c32gb512ssd_v2"),
            Self::GeneralI8c32gb1024ssdV2 => serializer.serialize_unit_variant("SkuName", 2u32, "general_i_8c32gb1024ssd_v2"),
            Self::GeneralI8c32gb2048ssdV2 => serializer.serialize_unit_variant("SkuName", 3u32, "general_i_8c32gb2048ssd_v2"),
            Self::GeneralI16c64gb256ssdV2 => serializer.serialize_unit_variant("SkuName", 4u32, "general_i_16c64gb256ssd_v2"),
            Self::GeneralI16c64gb512ssdV2 => serializer.serialize_unit_variant("SkuName", 5u32, "general_i_16c64gb512ssd_v2"),
            Self::GeneralI16c64gb1024ssdV2 => serializer.serialize_unit_variant("SkuName", 6u32, "general_i_16c64gb1024ssd_v2"),
            Self::GeneralI16c64gb2048ssdV2 => serializer.serialize_unit_variant("SkuName", 7u32, "general_i_16c64gb2048ssd_v2"),
            Self::GeneralI32c128gb512ssdV2 => serializer.serialize_unit_variant("SkuName", 8u32, "general_i_32c128gb512ssd_v2"),
            Self::GeneralI32c128gb1024ssdV2 => serializer.serialize_unit_variant("SkuName", 9u32, "general_i_32c128gb1024ssd_v2"),
            Self::GeneralI32c128gb2048ssdV2 => serializer.serialize_unit_variant("SkuName", 10u32, "general_i_32c128gb2048ssd_v2"),
            Self::GeneralA8c32gb256ssdV2 => serializer.serialize_unit_variant("SkuName", 11u32, "general_a_8c32gb256ssd_v2"),
            Self::GeneralA8c32gb512ssdV2 => serializer.serialize_unit_variant("SkuName", 12u32, "general_a_8c32gb512ssd_v2"),
            Self::GeneralA8c32gb1024ssdV2 => serializer.serialize_unit_variant("SkuName", 13u32, "general_a_8c32gb1024ssd_v2"),
            Self::GeneralA8c32gb2048ssdV2 => serializer.serialize_unit_variant("SkuName", 14u32, "general_a_8c32gb2048ssd_v2"),
            Self::GeneralA16c64gb256ssdV2 => serializer.serialize_unit_variant("SkuName", 15u32, "general_a_16c64gb256ssd_v2"),
            Self::GeneralA16c64gb512ssdV2 => serializer.serialize_unit_variant("SkuName", 16u32, "general_a_16c64gb512ssd_v2"),
            Self::GeneralA16c64gb1024ssdV2 => serializer.serialize_unit_variant("SkuName", 17u32, "general_a_16c64gb1024ssd_v2"),
            Self::GeneralA16c64gb2048ssdV2 => serializer.serialize_unit_variant("SkuName", 18u32, "general_a_16c64gb2048ssd_v2"),
            Self::GeneralA32c128gb512ssdV2 => serializer.serialize_unit_variant("SkuName", 19u32, "general_a_32c128gb512ssd_v2"),
            Self::GeneralA32c128gb1024ssdV2 => serializer.serialize_unit_variant("SkuName", 20u32, "general_a_32c128gb1024ssd_v2"),
            Self::GeneralA32c128gb2048ssdV2 => serializer.serialize_unit_variant("SkuName", 21u32, "general_a_32c128gb2048ssd_v2"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Stop on disconnect configuration settings for Dev Boxes created in this pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StopOnDisconnectConfiguration {
    #[doc = "Indicates whether the feature to stop the devbox on disconnect once the grace period has lapsed is enabled."]
    pub status: StopOnDisconnectEnableStatus,
    #[doc = "The specified time in minutes to wait before stopping a Dev Box once disconnect\nis detected."]
    #[serde(rename = "gracePeriodMinutes", default, skip_serializing_if = "Option::is_none")]
    pub grace_period_minutes: Option<i32>,
}
impl StopOnDisconnectConfiguration {
    pub fn new(status: StopOnDisconnectEnableStatus) -> Self {
        Self {
            status,
            grace_period_minutes: None,
        }
    }
}
#[doc = "Indicates whether the feature to stop the devbox on disconnect once the grace period has lapsed is enabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StopOnDisconnectEnableStatus")]
pub enum StopOnDisconnectEnableStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StopOnDisconnectEnableStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StopOnDisconnectEnableStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StopOnDisconnectEnableStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("StopOnDisconnectEnableStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("StopOnDisconnectEnableStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Storage settings for the Dev Box's disks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[doc = "Settings for the operating system disk."]
    #[serde(rename = "osDisk", default, skip_serializing_if = "Option::is_none")]
    pub os_disk: Option<OsDisk>,
}
impl StorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}

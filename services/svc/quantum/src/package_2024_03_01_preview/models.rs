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
#[doc = "The details (name and container) of the blob to store or download data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobDetails {
    #[doc = "The container name."]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "The blob name."]
    #[serde(rename = "blobName", default, skip_serializing_if = "Option::is_none")]
    pub blob_name: Option<String>,
}
impl BlobDetails {
    pub fn new(container_name: String) -> Self {
        Self {
            container_name,
            blob_name: None,
        }
    }
}
#[doc = "The job cost billed by the provider. The final cost on your bill might be slightly different due to added taxes and currency conversion rates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostEstimate {
    #[doc = "The currency code."]
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    #[doc = "List of usage events."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub events: Vec<UsageEvent>,
    #[doc = "The estimated total."]
    #[serde(rename = "estimatedTotal")]
    pub estimated_total: f32,
}
impl CostEstimate {
    pub fn new(currency_code: String, estimated_total: f32) -> Self {
        Self {
            currency_code,
            events: Vec::new(),
            estimated_total,
        }
    }
}
#[doc = "The scope at which the quota is applied to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DimensionScope")]
pub enum DimensionScope {
    Workspace,
    Subscription,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DimensionScope {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DimensionScope {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DimensionScope {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Workspace => serializer.serialize_unit_variant("DimensionScope", 0u32, "Workspace"),
            Self::Subscription => serializer.serialize_unit_variant("DimensionScope", 1u32, "Subscription"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorsWorkspaceItemError {
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
impl ErrorsWorkspaceItemError {
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
#[doc = "A workspace item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemDetails {
    #[doc = "Id of the workspace item."]
    pub id: WorkspaceItemId,
    #[doc = "The name of the item. It is not required for the name to be unique and it's only used for display purposes."]
    pub name: String,
    #[doc = "The unique identifier for the provider."]
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[doc = "The target identifier to run the job."]
    pub target: String,
    #[doc = "The creation time of the item."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<::time::OffsetDateTime>,
    #[doc = "The time when the item began execution."]
    #[serde(rename = "beginExecutionTime", default, with = "azure_core::date::rfc3339::option")]
    pub begin_execution_time: Option<::time::OffsetDateTime>,
    #[doc = "The time when the item finished execution."]
    #[serde(rename = "endExecutionTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_execution_time: Option<::time::OffsetDateTime>,
    #[doc = "The job cost billed by the provider. The final cost on your bill might be slightly different due to added taxes and currency conversion rates."]
    #[serde(rename = "costEstimate", default, skip_serializing_if = "Option::is_none")]
    pub cost_estimate: Option<CostEstimate>,
    #[doc = "The error object."]
    #[serde(rename = "errorData", default, skip_serializing_if = "Option::is_none")]
    pub error_data: Option<ErrorsWorkspaceItemError>,
}
impl ItemDetails {
    pub fn new(id: WorkspaceItemId, name: String, provider_id: String, target: String) -> Self {
        Self {
            id,
            name,
            provider_id,
            target,
            creation_time: None,
            begin_execution_time: None,
            end_execution_time: None,
            cost_estimate: None,
            error_data: None,
        }
    }
}
#[doc = "The type of the workspace item."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "itemType")]
pub enum ItemDetailsUnion {
    Job(JobDetails),
    Session(SessionDetails),
}
#[doc = "The type of the workspace item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ItemType")]
pub enum ItemType {
    Job,
    Session,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ItemType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ItemType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ItemType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Job => serializer.serialize_unit_variant("ItemType", 0u32, "Job"),
            Self::Session => serializer.serialize_unit_variant("ItemType", 1u32, "Session"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A job to be run in the workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDetails {
    #[serde(flatten)]
    pub item_details: ItemDetails,
    #[doc = "Id of the workspace item."]
    pub id: WorkspaceItemId,
    #[doc = "The type of the job."]
    #[serde(rename = "jobType", default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<JobType>,
    #[doc = "The ID of the session that the job is part of."]
    #[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[doc = "The blob container SAS uri, the container is used to host job data."]
    #[serde(rename = "containerUri")]
    pub container_uri: String,
    #[doc = "The input blob URI, if specified, it will override the default input blob in the container."]
    #[serde(rename = "inputDataUri", default, skip_serializing_if = "Option::is_none")]
    pub input_data_uri: Option<String>,
    #[doc = "The format of the input data."]
    #[serde(rename = "inputDataFormat", default, skip_serializing_if = "Option::is_none")]
    pub input_data_format: Option<String>,
    #[doc = "The status of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,
    #[doc = "The job metadata. Metadata provides client the ability to store client-specific information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The time when a job was successfully cancelled."]
    #[serde(rename = "cancellationTime", default, with = "azure_core::date::rfc3339::option")]
    pub cancellation_time: Option<::time::OffsetDateTime>,
    #[doc = "List of user-supplied tags associated with the job."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
    #[doc = "Quantum computing data."]
    #[serde(rename = "quantumComputingData", default, skip_serializing_if = "Option::is_none")]
    pub quantum_computing_data: Option<QuantumComputingData>,
    #[doc = "The input parameters for the job. JSON object used by the target solver. It is expected that the size of this object is small and only used to specify parameters for the execution target, not the input data."]
    #[serde(rename = "inputParams", default, skip_serializing_if = "Option::is_none")]
    pub input_params: Option<serde_json::Value>,
    #[doc = "The output blob uri. When a job finishes successfully, results will be uploaded to this blob."]
    #[serde(rename = "outputDataUri", default, skip_serializing_if = "Option::is_none")]
    pub output_data_uri: Option<String>,
    #[doc = "The format of the output data."]
    #[serde(rename = "outputDataFormat", default, skip_serializing_if = "Option::is_none")]
    pub output_data_format: Option<String>,
}
impl JobDetails {
    pub fn new(item_details: ItemDetails, id: WorkspaceItemId, container_uri: String) -> Self {
        Self {
            item_details,
            id,
            job_type: None,
            session_id: None,
            container_uri,
            input_data_uri: None,
            input_data_format: None,
            status: None,
            metadata: None,
            cancellation_time: None,
            tags: Vec::new(),
            quantum_computing_data: None,
            input_params: None,
            output_data_uri: None,
            output_data_format: None,
        }
    }
}
#[doc = "The status of the job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "JobStatus")]
pub enum JobStatus {
    Waiting,
    Executing,
    Succeeded,
    Failed,
    Cancelled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for JobStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for JobStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for JobStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Waiting => serializer.serialize_unit_variant("JobStatus", 0u32, "Waiting"),
            Self::Executing => serializer.serialize_unit_variant("JobStatus", 1u32, "Executing"),
            Self::Succeeded => serializer.serialize_unit_variant("JobStatus", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("JobStatus", 3u32, "Failed"),
            Self::Cancelled => serializer.serialize_unit_variant("JobStatus", 4u32, "Cancelled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type of the job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "JobType")]
pub enum JobType {
    Unknown,
    QuantumComputing,
    Optimization,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for JobType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for JobType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for JobType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("JobType", 0u32, "Unknown"),
            Self::QuantumComputing => serializer.serialize_unit_variant("JobType", 1u32, "QuantumComputing"),
            Self::Optimization => serializer.serialize_unit_variant("JobType", 2u32, "Optimization"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type JsonPatchDocument = Vec<JsonPatchObject>;
#[doc = "A JSONPatch object as defined by RFC 6902."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonPatchObject {
    #[doc = "The operation to be performed."]
    pub op: JsonPatchOperation,
    #[doc = "A JSON-Pointer."]
    pub path: String,
    #[doc = "A value to be used in the operation on the path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[doc = "Optional field used in copy and move operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
}
impl JsonPatchObject {
    pub fn new(op: JsonPatchOperation, path: String) -> Self {
        Self {
            op,
            path,
            value: None,
            from: None,
        }
    }
}
#[doc = "The operation to be performed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "JsonPatchOperation")]
pub enum JsonPatchOperation {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "move")]
    Move,
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "test")]
    Test,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for JsonPatchOperation {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for JsonPatchOperation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for JsonPatchOperation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Add => serializer.serialize_unit_variant("JsonPatchOperation", 0u32, "add"),
            Self::Remove => serializer.serialize_unit_variant("JsonPatchOperation", 1u32, "remove"),
            Self::Replace => serializer.serialize_unit_variant("JsonPatchOperation", 2u32, "replace"),
            Self::Move => serializer.serialize_unit_variant("JsonPatchOperation", 3u32, "move"),
            Self::Copy => serializer.serialize_unit_variant("JsonPatchOperation", 4u32, "copy"),
            Self::Test => serializer.serialize_unit_variant("JsonPatchOperation", 5u32, "test"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The time period in which the quota's underlying meter is accumulated. Based on calendar year. 'None' is used for concurrent quotas."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MeterPeriod")]
pub enum MeterPeriod {
    None,
    Monthly,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MeterPeriod {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MeterPeriod {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MeterPeriod {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("MeterPeriod", 0u32, "None"),
            Self::Monthly => serializer.serialize_unit_variant("MeterPeriod", 1u32, "Monthly"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Paged collection of ItemDetails items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedItemDetails {
    #[doc = "The ItemDetails items on this page"]
    pub value: Vec<ItemDetailsUnion>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedItemDetails {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedItemDetails {
    pub fn new(value: Vec<ItemDetailsUnion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of JobDetails items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedJobDetails {
    #[doc = "The JobDetails items on this page"]
    pub value: Vec<JobDetails>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedJobDetails {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedJobDetails {
    pub fn new(value: Vec<JobDetails>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of ProviderStatus items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedProviderStatus {
    #[doc = "The ProviderStatus items on this page"]
    pub value: Vec<ProviderStatus>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedProviderStatus {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedProviderStatus {
    pub fn new(value: Vec<ProviderStatus>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of Quota items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedQuota {
    #[doc = "The Quota items on this page"]
    pub value: Vec<Quota>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedQuota {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedQuota {
    pub fn new(value: Vec<Quota>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of SessionDetails items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedSessionDetails {
    #[doc = "The SessionDetails items on this page"]
    pub value: Vec<SessionDetails>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedSessionDetails {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedSessionDetails {
    pub fn new(value: Vec<SessionDetails>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Provider availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProviderAvailability")]
pub enum ProviderAvailability {
    Available,
    Degraded,
    Unavailable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProviderAvailability {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProviderAvailability {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProviderAvailability {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Available => serializer.serialize_unit_variant("ProviderAvailability", 0u32, "Available"),
            Self::Degraded => serializer.serialize_unit_variant("ProviderAvailability", 1u32, "Degraded"),
            Self::Unavailable => serializer.serialize_unit_variant("ProviderAvailability", 2u32, "Unavailable"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Provider status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderStatus {
    #[doc = "Provider id."]
    pub id: String,
    #[doc = "Provider availability."]
    #[serde(rename = "currentAvailability")]
    pub current_availability: ProviderAvailability,
    #[doc = "Current target statuses."]
    pub targets: Vec<TargetStatus>,
}
impl ProviderStatus {
    pub fn new(id: String, current_availability: ProviderAvailability, targets: Vec<TargetStatus>) -> Self {
        Self {
            id,
            current_availability,
            targets,
        }
    }
}
#[doc = "Quantum computing data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuantumComputingData {
    #[doc = "The number of quantum computing items in the job."]
    pub count: i64,
}
impl QuantumComputingData {
    pub fn new(count: i64) -> Self {
        Self { count }
    }
}
#[doc = "Quota information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quota {
    #[doc = "The name of the dimension associated with the quota."]
    pub dimension: String,
    #[doc = "The scope at which the quota is applied to."]
    pub scope: DimensionScope,
    #[doc = "The unique identifier for the provider."]
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[doc = "The amount of the usage that has been applied for the current period."]
    pub utilization: f32,
    #[doc = "The amount of the usage that has been reserved but not applied for the current period."]
    pub holds: f32,
    #[doc = "The maximum amount of usage allowed for the current period."]
    pub limit: f32,
    #[doc = "The time period in which the quota's underlying meter is accumulated. Based on calendar year. 'None' is used for concurrent quotas."]
    pub period: MeterPeriod,
}
impl Quota {
    pub fn new(
        dimension: String,
        scope: DimensionScope,
        provider_id: String,
        utilization: f32,
        holds: f32,
        limit: f32,
        period: MeterPeriod,
    ) -> Self {
        Self {
            dimension,
            scope,
            provider_id,
            utilization,
            holds,
            limit,
            period,
        }
    }
}
#[doc = "SAS URI operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SasUriResponse {
    #[doc = "A URL with a SAS token to upload a blob for execution in the given workspace."]
    #[serde(rename = "sasUri")]
    pub sas_uri: String,
}
impl SasUriResponse {
    pub fn new(sas_uri: String) -> Self {
        Self { sas_uri }
    }
}
#[doc = "Session, a logical grouping of jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionDetails {
    #[serde(flatten)]
    pub item_details: ItemDetails,
    #[doc = "Id of the workspace item."]
    pub id: WorkspaceItemId,
    #[doc = "Policy controlling the behavior of the Session when a job in the session fails."]
    #[serde(rename = "jobFailurePolicy")]
    pub job_failure_policy: SessionJobFailurePolicy,
    #[doc = "The status of the session."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SessionStatus>,
}
impl SessionDetails {
    pub fn new(item_details: ItemDetails, id: WorkspaceItemId, job_failure_policy: SessionJobFailurePolicy) -> Self {
        Self {
            item_details,
            id,
            job_failure_policy,
            status: None,
        }
    }
}
#[doc = "Policy controlling the behavior of the Session when a job in the session fails."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SessionJobFailurePolicy")]
pub enum SessionJobFailurePolicy {
    Abort,
    Continue,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SessionJobFailurePolicy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SessionJobFailurePolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SessionJobFailurePolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Abort => serializer.serialize_unit_variant("SessionJobFailurePolicy", 0u32, "Abort"),
            Self::Continue => serializer.serialize_unit_variant("SessionJobFailurePolicy", 1u32, "Continue"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The status of the session."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SessionStatus")]
pub enum SessionStatus {
    Waiting,
    Executing,
    Succeeded,
    Failed,
    #[serde(rename = "Failure(s)")]
    FailureS,
    TimedOut,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SessionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SessionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SessionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Waiting => serializer.serialize_unit_variant("SessionStatus", 0u32, "Waiting"),
            Self::Executing => serializer.serialize_unit_variant("SessionStatus", 1u32, "Executing"),
            Self::Succeeded => serializer.serialize_unit_variant("SessionStatus", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("SessionStatus", 3u32, "Failed"),
            Self::FailureS => serializer.serialize_unit_variant("SessionStatus", 4u32, "Failure(s)"),
            Self::TimedOut => serializer.serialize_unit_variant("SessionStatus", 5u32, "TimedOut"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Target availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TargetAvailability")]
pub enum TargetAvailability {
    Available,
    Degraded,
    Unavailable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TargetAvailability {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TargetAvailability {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TargetAvailability {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Available => serializer.serialize_unit_variant("TargetAvailability", 0u32, "Available"),
            Self::Degraded => serializer.serialize_unit_variant("TargetAvailability", 1u32, "Degraded"),
            Self::Unavailable => serializer.serialize_unit_variant("TargetAvailability", 2u32, "Unavailable"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Target status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetStatus {
    #[doc = "Target id."]
    pub id: String,
    #[doc = "Target availability."]
    #[serde(rename = "currentAvailability")]
    pub current_availability: TargetAvailability,
    #[doc = "Average queue time in seconds."]
    #[serde(rename = "averageQueueTime")]
    pub average_queue_time: i64,
    #[doc = "A page with detailed status of the provider."]
    #[serde(rename = "statusPage", default, skip_serializing_if = "Option::is_none")]
    pub status_page: Option<String>,
}
impl TargetStatus {
    pub fn new(id: String, current_availability: TargetAvailability, average_queue_time: i64) -> Self {
        Self {
            id,
            current_availability,
            average_queue_time,
            status_page: None,
        }
    }
}
#[doc = "Usage event details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageEvent {
    #[doc = "The dimension id."]
    #[serde(rename = "dimensionId")]
    pub dimension_id: String,
    #[doc = "The dimension name."]
    #[serde(rename = "dimensionName")]
    pub dimension_name: String,
    #[doc = "The unit of measure."]
    #[serde(rename = "measureUnit")]
    pub measure_unit: String,
    #[doc = "The amount billed."]
    #[serde(rename = "amountBilled")]
    pub amount_billed: f32,
    #[doc = "The amount consumed."]
    #[serde(rename = "amountConsumed")]
    pub amount_consumed: f32,
    #[doc = "The unit price."]
    #[serde(rename = "unitPrice")]
    pub unit_price: f32,
}
impl UsageEvent {
    pub fn new(
        dimension_id: String,
        dimension_name: String,
        measure_unit: String,
        amount_billed: f32,
        amount_consumed: f32,
        unit_price: f32,
    ) -> Self {
        Self {
            dimension_id,
            dimension_name,
            measure_unit,
            amount_billed,
            amount_consumed,
            unit_price,
        }
    }
}
pub type WorkspaceItemId = String;

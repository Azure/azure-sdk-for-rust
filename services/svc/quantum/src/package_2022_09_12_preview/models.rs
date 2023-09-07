#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Blob details."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CostEstimate {
    #[doc = "The currency code."]
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[doc = "List of usage events"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub events: Vec<UsageEvent>,
    #[doc = "The estimated total."]
    #[serde(rename = "estimatedTotal", default, skip_serializing_if = "Option::is_none")]
    pub estimated_total: Option<f64>,
}
impl CostEstimate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from Azure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorData {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    pub code: String,
    #[doc = "A message describing the error, intended to be suitable for displaying in a user interface."]
    pub message: String,
}
impl ErrorData {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
#[doc = "Item details. An item can be a job or a session."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemDetails {
    #[doc = "The id of the item."]
    pub id: String,
    #[doc = "The name of the item. It is not required for the name to be unique and it's only used for display purposes."]
    pub name: String,
    #[doc = "The unique identifier for the provider."]
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[doc = "The target identifier to run the job."]
    pub target: String,
    #[doc = "The type of item."]
    #[serde(rename = "itemType")]
    pub item_type: item_details::ItemType,
    #[doc = "The creation time of the item."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "The time when the item began execution."]
    #[serde(rename = "beginExecutionTime", default, with = "azure_core::date::rfc3339::option")]
    pub begin_execution_time: Option<time::OffsetDateTime>,
    #[doc = "The time when the item finished execution."]
    #[serde(rename = "endExecutionTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_execution_time: Option<time::OffsetDateTime>,
    #[doc = "The job cost billed by the provider. The final cost on your bill might be slightly different due to added taxes and currency conversion rates."]
    #[serde(rename = "costEstimate", default, skip_serializing_if = "Option::is_none")]
    pub cost_estimate: Option<CostEstimate>,
    #[doc = "An error response from Azure."]
    #[serde(rename = "errorData", default, skip_serializing_if = "Option::is_none")]
    pub error_data: Option<ErrorData>,
}
impl ItemDetails {
    pub fn new(id: String, name: String, provider_id: String, target: String, item_type: item_details::ItemType) -> Self {
        Self {
            id,
            name,
            provider_id,
            target,
            item_type,
            creation_time: None,
            begin_execution_time: None,
            end_execution_time: None,
            cost_estimate: None,
            error_data: None,
        }
    }
}
pub mod item_details {
    use super::*;
    #[doc = "The type of item."]
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
}
#[doc = "List of item details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemDetailsList {
    pub value: Vec<ItemDetails>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ItemDetailsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ItemDetailsList {
    pub fn new(value: Vec<ItemDetails>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Job details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDetails {
    #[serde(flatten)]
    pub item_details: ItemDetails,
    #[doc = "The type of job."]
    #[serde(rename = "jobType", default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<job_details::JobType>,
    #[doc = "The ID of the session that the job is part of."]
    #[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[doc = "The blob container SAS uri, the container is used to host job data."]
    #[serde(rename = "containerUri")]
    pub container_uri: String,
    #[doc = "The input blob SAS uri, if specified, it will override the default input blob in the container."]
    #[serde(rename = "inputDataUri", default, skip_serializing_if = "Option::is_none")]
    pub input_data_uri: Option<String>,
    #[doc = "The format of the input data."]
    #[serde(rename = "inputDataFormat")]
    pub input_data_format: String,
    #[doc = "The input parameters for the job. JSON object used by the target solver. It is expected that the size of this object is small and only used to specify parameters for the execution target, not the input data."]
    #[serde(rename = "inputParams", default, skip_serializing_if = "Option::is_none")]
    pub input_params: Option<serde_json::Value>,
    #[doc = "The status of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<job_details::Status>,
    #[doc = "The job metadata. Metadata provides client the ability to store client-specific information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The output blob SAS uri. When a job finishes successfully, results will be uploaded to this blob."]
    #[serde(rename = "outputDataUri", default, skip_serializing_if = "Option::is_none")]
    pub output_data_uri: Option<String>,
    #[doc = "The format of the output data."]
    #[serde(rename = "outputDataFormat", default, skip_serializing_if = "Option::is_none")]
    pub output_data_format: Option<String>,
    #[doc = "The time when a job was successfully cancelled."]
    #[serde(rename = "cancellationTime", default, with = "azure_core::date::rfc3339::option")]
    pub cancellation_time: Option<time::OffsetDateTime>,
    #[doc = "Quantum computing data."]
    #[serde(rename = "quantumComputingData", default, skip_serializing_if = "Option::is_none")]
    pub quantum_computing_data: Option<QuantumComputingData>,
    #[doc = "List of user-supplied tags associated with the job."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
}
impl JobDetails {
    pub fn new(item_details: ItemDetails, container_uri: String, input_data_format: String) -> Self {
        Self {
            item_details,
            job_type: None,
            session_id: None,
            container_uri,
            input_data_uri: None,
            input_data_format,
            input_params: None,
            status: None,
            metadata: None,
            output_data_uri: None,
            output_data_format: None,
            cancellation_time: None,
            quantum_computing_data: None,
            tags: Vec::new(),
        }
    }
}
pub mod job_details {
    use super::*;
    #[doc = "The type of job."]
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
    #[doc = "The status of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Waiting,
        Executing,
        Succeeded,
        Failed,
        Cancelled,
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
                Self::Waiting => serializer.serialize_unit_variant("Status", 0u32, "Waiting"),
                Self::Executing => serializer.serialize_unit_variant("Status", 1u32, "Executing"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 3u32, "Failed"),
                Self::Cancelled => serializer.serialize_unit_variant("Status", 4u32, "Cancelled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of job details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobDetailsList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<JobDetails>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobDetailsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobDetailsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A JSONPatch document as defined by RFC 6902"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonPatchDocument {
    #[doc = "The operation to be performed."]
    pub op: json_patch_document::Op,
    #[doc = "A JSON-Pointer."]
    pub path: String,
    #[doc = "A value to be used in the operation on the path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[doc = "Optional field used in copy and move operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
}
impl JsonPatchDocument {
    pub fn new(op: json_patch_document::Op, path: String) -> Self {
        Self {
            op,
            path,
            value: None,
            from: None,
        }
    }
}
pub mod json_patch_document {
    use super::*;
    #[doc = "The operation to be performed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Op")]
    pub enum Op {
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
    impl FromStr for Op {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Op {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Op {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Add => serializer.serialize_unit_variant("Op", 0u32, "add"),
                Self::Remove => serializer.serialize_unit_variant("Op", 1u32, "remove"),
                Self::Replace => serializer.serialize_unit_variant("Op", 2u32, "replace"),
                Self::Move => serializer.serialize_unit_variant("Op", 3u32, "move"),
                Self::Copy => serializer.serialize_unit_variant("Op", 4u32, "copy"),
                Self::Test => serializer.serialize_unit_variant("Op", 5u32, "test"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type PatchRequest = Vec<JsonPatchDocument>;
#[doc = "Providers status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderStatus {
    #[doc = "Provider id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Provider availability."]
    #[serde(rename = "currentAvailability", default, skip_serializing_if = "Option::is_none")]
    pub current_availability: Option<provider_status::CurrentAvailability>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub targets: Vec<TargetStatus>,
}
impl ProviderStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod provider_status {
    use super::*;
    #[doc = "Provider availability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CurrentAvailability")]
    pub enum CurrentAvailability {
        Available,
        Degraded,
        Unavailable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CurrentAvailability {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CurrentAvailability {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CurrentAvailability {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Available => serializer.serialize_unit_variant("CurrentAvailability", 0u32, "Available"),
                Self::Degraded => serializer.serialize_unit_variant("CurrentAvailability", 1u32, "Degraded"),
                Self::Unavailable => serializer.serialize_unit_variant("CurrentAvailability", 2u32, "Unavailable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Providers status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderStatusList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProviderStatus>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProviderStatusList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProviderStatusList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quantum computing data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuantumComputingData {
    #[doc = "The number of quantum computing items in the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl QuantumComputingData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Quota {
    #[doc = "The name of the dimension associated with the quota."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
    #[doc = "The scope at which the quota is applied."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<quota::Scope>,
    #[doc = "The unique identifier for the provider."]
    #[serde(rename = "providerId", default, skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    #[doc = "The amount of the usage that has been applied for the current period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utilization: Option<f64>,
    #[doc = "The amount of the usage that has been reserved but not applied for the current period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holds: Option<f64>,
    #[doc = "The maximum amount of usage allowed for the current period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[doc = "The time period in which the quota's underlying meter is accumulated. Based on calendar year. 'None' is used for concurrent quotas."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<quota::Period>,
}
impl Quota {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod quota {
    use super::*;
    #[doc = "The scope at which the quota is applied."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Scope")]
    pub enum Scope {
        Workspace,
        Subscription,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Scope {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Scope {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Scope {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Workspace => serializer.serialize_unit_variant("Scope", 0u32, "Workspace"),
                Self::Subscription => serializer.serialize_unit_variant("Scope", 1u32, "Subscription"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The time period in which the quota's underlying meter is accumulated. Based on calendar year. 'None' is used for concurrent quotas."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Period")]
    pub enum Period {
        None,
        Monthly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Period {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Period {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Period {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Period", 0u32, "None"),
                Self::Monthly => serializer.serialize_unit_variant("Period", 1u32, "Monthly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of quotas."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Quota>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for QuotaList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl QuotaList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error information returned by the API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestError {
    #[doc = "An error response from Azure."]
    pub error: ErrorData,
}
impl azure_core::Continuable for RestError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RestError {
    pub fn new(error: ErrorData) -> Self {
        Self { error }
    }
}
#[doc = "Get SAS URL operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SasUriResponse {
    #[doc = "A URL with a SAS token to upload a blob for execution in the given workspace."]
    #[serde(rename = "sasUri", default, skip_serializing_if = "Option::is_none")]
    pub sas_uri: Option<String>,
}
impl SasUriResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Session details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionDetails {
    #[serde(flatten)]
    pub item_details: ItemDetails,
    #[doc = "Policy controlling the behavior of the Session when a job in the session fails."]
    #[serde(rename = "jobFailurePolicy", default, skip_serializing_if = "Option::is_none")]
    pub job_failure_policy: Option<session_details::JobFailurePolicy>,
    #[doc = "The status of the session."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<session_details::Status>,
}
impl SessionDetails {
    pub fn new(item_details: ItemDetails) -> Self {
        Self {
            item_details,
            job_failure_policy: None,
            status: None,
        }
    }
}
pub mod session_details {
    use super::*;
    #[doc = "Policy controlling the behavior of the Session when a job in the session fails."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "JobFailurePolicy")]
    pub enum JobFailurePolicy {
        Abort,
        Continue,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for JobFailurePolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for JobFailurePolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for JobFailurePolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Abort => serializer.serialize_unit_variant("JobFailurePolicy", 0u32, "Abort"),
                Self::Continue => serializer.serialize_unit_variant("JobFailurePolicy", 1u32, "Continue"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for JobFailurePolicy {
        fn default() -> Self {
            Self::Abort
        }
    }
    #[doc = "The status of the session."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
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
                Self::Waiting => serializer.serialize_unit_variant("Status", 0u32, "Waiting"),
                Self::Executing => serializer.serialize_unit_variant("Status", 1u32, "Executing"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 3u32, "Failed"),
                Self::FailureS => serializer.serialize_unit_variant("Status", 4u32, "Failure(s)"),
                Self::TimedOut => serializer.serialize_unit_variant("Status", 5u32, "TimedOut"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of session details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionDetailsList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SessionDetails>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SessionDetailsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SessionDetailsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Target status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetStatus {
    #[doc = "Target id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Target availability."]
    #[serde(rename = "currentAvailability", default, skip_serializing_if = "Option::is_none")]
    pub current_availability: Option<target_status::CurrentAvailability>,
    #[doc = "Average queue time in seconds."]
    #[serde(rename = "averageQueueTime", default, skip_serializing_if = "Option::is_none")]
    pub average_queue_time: Option<i64>,
    #[doc = "A page with detailed status of the provider."]
    #[serde(rename = "statusPage", default, skip_serializing_if = "Option::is_none")]
    pub status_page: Option<String>,
}
impl TargetStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod target_status {
    use super::*;
    #[doc = "Target availability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CurrentAvailability")]
    pub enum CurrentAvailability {
        Available,
        Degraded,
        Unavailable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CurrentAvailability {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CurrentAvailability {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CurrentAvailability {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Available => serializer.serialize_unit_variant("CurrentAvailability", 0u32, "Available"),
                Self::Degraded => serializer.serialize_unit_variant("CurrentAvailability", 1u32, "Degraded"),
                Self::Unavailable => serializer.serialize_unit_variant("CurrentAvailability", 2u32, "Unavailable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Usage event details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageEvent {
    #[doc = "The dimension id."]
    #[serde(rename = "dimensionId", default, skip_serializing_if = "Option::is_none")]
    pub dimension_id: Option<String>,
    #[doc = "The dimension name."]
    #[serde(rename = "dimensionName", default, skip_serializing_if = "Option::is_none")]
    pub dimension_name: Option<String>,
    #[doc = "The unit of measure."]
    #[serde(rename = "measureUnit", default, skip_serializing_if = "Option::is_none")]
    pub measure_unit: Option<String>,
    #[doc = "The amount billed."]
    #[serde(rename = "amountBilled", default, skip_serializing_if = "Option::is_none")]
    pub amount_billed: Option<f64>,
    #[doc = "The amount consumed."]
    #[serde(rename = "amountConsumed", default, skip_serializing_if = "Option::is_none")]
    pub amount_consumed: Option<f64>,
    #[doc = "The unit price."]
    #[serde(rename = "unitPrice", default, skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f64>,
}
impl UsageEvent {
    pub fn new() -> Self {
        Self::default()
    }
}

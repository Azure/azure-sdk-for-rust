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
#[doc = "An error response from Azure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorData {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for displaying in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDetails {
    #[doc = "The job id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The job name. Is not required for the name to be unique and it's only used for display purposes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    #[doc = "The unique identifier for the provider."]
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[doc = "The target identifier to run the job."]
    pub target: String,
    #[doc = "The job metadata. Metadata provides client the ability to store client-specific information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The output blob SAS uri. When a job finishes successfully, results will be uploaded to this blob."]
    #[serde(rename = "outputDataUri", default, skip_serializing_if = "Option::is_none")]
    pub output_data_uri: Option<String>,
    #[doc = "The format of the output data."]
    #[serde(rename = "outputDataFormat", default, skip_serializing_if = "Option::is_none")]
    pub output_data_format: Option<String>,
    #[doc = "The job status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<job_details::Status>,
    #[doc = "The creation time of the job."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "The time when the job began execution."]
    #[serde(rename = "beginExecutionTime", default, with = "azure_core::date::rfc3339::option")]
    pub begin_execution_time: Option<time::OffsetDateTime>,
    #[doc = "The time when the job finished execution."]
    #[serde(rename = "endExecutionTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_execution_time: Option<time::OffsetDateTime>,
    #[doc = "The time when a job was successfully cancelled."]
    #[serde(rename = "cancellationTime", default, with = "azure_core::date::rfc3339::option")]
    pub cancellation_time: Option<time::OffsetDateTime>,
    #[doc = "An error response from Azure."]
    #[serde(rename = "errorData", default, skip_serializing_if = "Option::is_none")]
    pub error_data: Option<ErrorData>,
}
impl JobDetails {
    pub fn new(container_uri: String, input_data_format: String, provider_id: String, target: String) -> Self {
        Self {
            id: None,
            name: None,
            container_uri,
            input_data_uri: None,
            input_data_format,
            input_params: None,
            provider_id,
            target,
            metadata: None,
            output_data_uri: None,
            output_data_format: None,
            status: None,
            creation_time: None,
            begin_execution_time: None,
            end_execution_time: None,
            cancellation_time: None,
            error_data: None,
        }
    }
}
pub mod job_details {
    use super::*;
    #[doc = "The job status."]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobDetails>,
    #[doc = "Total records count number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
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
#[doc = "Providers status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderStatus {
    #[doc = "Provider id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Provider availability."]
    #[serde(rename = "currentAvailability", default, skip_serializing_if = "Option::is_none")]
    pub current_availability: Option<provider_status::CurrentAvailability>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Error information returned by the API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestError {
    #[doc = "An error response from Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorData>,
}
impl azure_core::Continuable for RestError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RestError {
    pub fn new() -> Self {
        Self::default()
    }
}

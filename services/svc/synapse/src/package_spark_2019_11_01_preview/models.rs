#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkBatchJob {
    #[serde(rename = "livyInfo", default, skip_serializing_if = "Option::is_none")]
    pub livy_info: Option<SparkBatchJobState>,
    #[doc = "The batch name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The workspace name."]
    #[serde(rename = "workspaceName", default, skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
    #[doc = "The Spark pool name."]
    #[serde(rename = "sparkPoolName", default, skip_serializing_if = "Option::is_none")]
    pub spark_pool_name: Option<String>,
    #[doc = "The submitter name."]
    #[serde(rename = "submitterName", default, skip_serializing_if = "Option::is_none")]
    pub submitter_name: Option<String>,
    #[doc = "The submitter identifier."]
    #[serde(rename = "submitterId", default, skip_serializing_if = "Option::is_none")]
    pub submitter_id: Option<String>,
    #[doc = "The artifact identifier."]
    #[serde(rename = "artifactId", default, skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    #[doc = "The job type."]
    #[serde(rename = "jobType", default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<spark_batch_job::JobType>,
    #[doc = "The Spark batch job result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<spark_batch_job::Result>,
    #[serde(rename = "schedulerInfo", default, skip_serializing_if = "Option::is_none")]
    pub scheduler_info: Option<SparkScheduler>,
    #[serde(rename = "pluginInfo", default, skip_serializing_if = "Option::is_none")]
    pub plugin_info: Option<SparkServicePlugin>,
    #[doc = "The error information."]
    #[serde(rename = "errorInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub error_info: Vec<SparkServiceError>,
    #[doc = "The tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The session Id."]
    pub id: i32,
    #[doc = "The application id of this session"]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "The detailed application info."]
    #[serde(rename = "appInfo", default, skip_serializing_if = "Option::is_none")]
    pub app_info: Option<serde_json::Value>,
    #[doc = "The batch state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The log lines."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub log: Vec<String>,
}
impl SparkBatchJob {
    pub fn new(id: i32) -> Self {
        Self {
            livy_info: None,
            name: None,
            workspace_name: None,
            spark_pool_name: None,
            submitter_name: None,
            submitter_id: None,
            artifact_id: None,
            job_type: None,
            result: None,
            scheduler_info: None,
            plugin_info: None,
            error_info: Vec::new(),
            tags: None,
            id,
            app_id: None,
            app_info: None,
            state: None,
            log: Vec::new(),
        }
    }
}
pub mod spark_batch_job {
    use super::*;
    #[doc = "The job type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "JobType")]
    pub enum JobType {
        SparkBatch,
        SparkSession,
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
                Self::SparkBatch => serializer.serialize_unit_variant("JobType", 0u32, "SparkBatch"),
                Self::SparkSession => serializer.serialize_unit_variant("JobType", 1u32, "SparkSession"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The Spark batch job result."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Result")]
    pub enum Result {
        Uncertain,
        Succeeded,
        Failed,
        Cancelled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Result {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Result {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Result {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Uncertain => serializer.serialize_unit_variant("Result", 0u32, "Uncertain"),
                Self::Succeeded => serializer.serialize_unit_variant("Result", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Result", 2u32, "Failed"),
                Self::Cancelled => serializer.serialize_unit_variant("Result", 3u32, "Cancelled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Response for batch list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkBatchJobCollection {
    #[doc = "The start index of fetched sessions."]
    pub from: i32,
    #[doc = "Number of sessions fetched."]
    pub total: i32,
    #[doc = "Batch list"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sessions: Vec<SparkBatchJob>,
}
impl SparkBatchJobCollection {
    pub fn new(from: i32, total: i32) -> Self {
        Self {
            from,
            total,
            sessions: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkBatchJobOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "artifactId", default, skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    pub name: String,
    pub file: String,
    #[serde(rename = "className", default, skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jars: Vec<String>,
    #[serde(rename = "pyFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub py_files: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub archives: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conf: Option<serde_json::Value>,
    #[serde(rename = "driverMemory", default, skip_serializing_if = "Option::is_none")]
    pub driver_memory: Option<String>,
    #[serde(rename = "driverCores", default, skip_serializing_if = "Option::is_none")]
    pub driver_cores: Option<i32>,
    #[serde(rename = "executorMemory", default, skip_serializing_if = "Option::is_none")]
    pub executor_memory: Option<String>,
    #[serde(rename = "executorCores", default, skip_serializing_if = "Option::is_none")]
    pub executor_cores: Option<i32>,
    #[serde(rename = "numExecutors", default, skip_serializing_if = "Option::is_none")]
    pub num_executors: Option<i32>,
}
impl SparkBatchJobOptions {
    pub fn new(name: String, file: String) -> Self {
        Self {
            tags: None,
            artifact_id: None,
            name,
            file,
            class_name: None,
            args: Vec::new(),
            jars: Vec::new(),
            py_files: Vec::new(),
            files: Vec::new(),
            archives: Vec::new(),
            conf: None,
            driver_memory: None,
            driver_cores: None,
            executor_memory: None,
            executor_cores: None,
            num_executors: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkBatchJobState {
    #[doc = "the time that at which \"not_started\" livy state was first seen."]
    #[serde(rename = "notStartedAt", default, with = "azure_core::date::rfc3339::option")]
    pub not_started_at: Option<time::OffsetDateTime>,
    #[doc = "the time that at which \"starting\" livy state was first seen."]
    #[serde(rename = "startingAt", default, with = "azure_core::date::rfc3339::option")]
    pub starting_at: Option<time::OffsetDateTime>,
    #[doc = "the time that at which \"running\" livy state was first seen."]
    #[serde(rename = "runningAt", default, with = "azure_core::date::rfc3339::option")]
    pub running_at: Option<time::OffsetDateTime>,
    #[doc = "time that at which \"dead\" livy state was first seen."]
    #[serde(rename = "deadAt", default, with = "azure_core::date::rfc3339::option")]
    pub dead_at: Option<time::OffsetDateTime>,
    #[doc = "the time that at which \"success\" livy state was first seen."]
    #[serde(rename = "successAt", default, with = "azure_core::date::rfc3339::option")]
    pub success_at: Option<time::OffsetDateTime>,
    #[doc = "the time that at which \"killed\" livy state was first seen."]
    #[serde(rename = "killedAt", default, with = "azure_core::date::rfc3339::option")]
    pub killed_at: Option<time::OffsetDateTime>,
    #[doc = "the time that at which \"recovering\" livy state was first seen."]
    #[serde(rename = "recoveringAt", default, with = "azure_core::date::rfc3339::option")]
    pub recovering_at: Option<time::OffsetDateTime>,
    #[doc = "the Spark job state."]
    #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    #[serde(rename = "jobCreationRequest", default, skip_serializing_if = "Option::is_none")]
    pub job_creation_request: Option<SparkRequest>,
}
impl SparkBatchJobState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(rename = "className", default, skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jars: Vec<String>,
    #[serde(rename = "pyFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub py_files: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub archives: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conf: Option<serde_json::Value>,
    #[serde(rename = "driverMemory", default, skip_serializing_if = "Option::is_none")]
    pub driver_memory: Option<String>,
    #[serde(rename = "driverCores", default, skip_serializing_if = "Option::is_none")]
    pub driver_cores: Option<i32>,
    #[serde(rename = "executorMemory", default, skip_serializing_if = "Option::is_none")]
    pub executor_memory: Option<String>,
    #[serde(rename = "executorCores", default, skip_serializing_if = "Option::is_none")]
    pub executor_cores: Option<i32>,
    #[serde(rename = "numExecutors", default, skip_serializing_if = "Option::is_none")]
    pub num_executors: Option<i32>,
}
impl SparkRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkScheduler {
    #[serde(rename = "submittedAt", default, with = "azure_core::date::rfc3339::option")]
    pub submitted_at: Option<time::OffsetDateTime>,
    #[serde(rename = "scheduledAt", default, with = "azure_core::date::rfc3339::option")]
    pub scheduled_at: Option<time::OffsetDateTime>,
    #[serde(rename = "endedAt", default, with = "azure_core::date::rfc3339::option")]
    pub ended_at: Option<time::OffsetDateTime>,
    #[serde(rename = "cancellationRequestedAt", default, with = "azure_core::date::rfc3339::option")]
    pub cancellation_requested_at: Option<time::OffsetDateTime>,
    #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
    pub current_state: Option<spark_scheduler::CurrentState>,
}
impl SparkScheduler {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod spark_scheduler {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CurrentState")]
    pub enum CurrentState {
        Queued,
        Scheduled,
        Ended,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CurrentState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CurrentState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CurrentState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Queued => serializer.serialize_unit_variant("CurrentState", 0u32, "Queued"),
                Self::Scheduled => serializer.serialize_unit_variant("CurrentState", 1u32, "Scheduled"),
                Self::Ended => serializer.serialize_unit_variant("CurrentState", 2u32, "Ended"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkServiceError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<spark_service_error::Source>,
}
impl SparkServiceError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod spark_service_error {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Source")]
    pub enum Source {
        System,
        User,
        Unknown,
        Dependency,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Source {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Source {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Source {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::System => serializer.serialize_unit_variant("Source", 0u32, "System"),
                Self::User => serializer.serialize_unit_variant("Source", 1u32, "User"),
                Self::Unknown => serializer.serialize_unit_variant("Source", 2u32, "Unknown"),
                Self::Dependency => serializer.serialize_unit_variant("Source", 3u32, "Dependency"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkServicePlugin {
    #[serde(rename = "preparationStartedAt", default, with = "azure_core::date::rfc3339::option")]
    pub preparation_started_at: Option<time::OffsetDateTime>,
    #[serde(rename = "resourceAcquisitionStartedAt", default, with = "azure_core::date::rfc3339::option")]
    pub resource_acquisition_started_at: Option<time::OffsetDateTime>,
    #[serde(rename = "submissionStartedAt", default, with = "azure_core::date::rfc3339::option")]
    pub submission_started_at: Option<time::OffsetDateTime>,
    #[serde(rename = "monitoringStartedAt", default, with = "azure_core::date::rfc3339::option")]
    pub monitoring_started_at: Option<time::OffsetDateTime>,
    #[serde(rename = "cleanupStartedAt", default, with = "azure_core::date::rfc3339::option")]
    pub cleanup_started_at: Option<time::OffsetDateTime>,
    #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
    pub current_state: Option<spark_service_plugin::CurrentState>,
}
impl SparkServicePlugin {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod spark_service_plugin {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CurrentState")]
    pub enum CurrentState {
        Preparation,
        ResourceAcquisition,
        Queued,
        Submission,
        Monitoring,
        Cleanup,
        Ended,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CurrentState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CurrentState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CurrentState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Preparation => serializer.serialize_unit_variant("CurrentState", 0u32, "Preparation"),
                Self::ResourceAcquisition => serializer.serialize_unit_variant("CurrentState", 1u32, "ResourceAcquisition"),
                Self::Queued => serializer.serialize_unit_variant("CurrentState", 2u32, "Queued"),
                Self::Submission => serializer.serialize_unit_variant("CurrentState", 3u32, "Submission"),
                Self::Monitoring => serializer.serialize_unit_variant("CurrentState", 4u32, "Monitoring"),
                Self::Cleanup => serializer.serialize_unit_variant("CurrentState", 5u32, "Cleanup"),
                Self::Ended => serializer.serialize_unit_variant("CurrentState", 6u32, "Ended"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkSession {
    #[serde(rename = "livyInfo", default, skip_serializing_if = "Option::is_none")]
    pub livy_info: Option<SparkSessionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "workspaceName", default, skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
    #[serde(rename = "sparkPoolName", default, skip_serializing_if = "Option::is_none")]
    pub spark_pool_name: Option<String>,
    #[serde(rename = "submitterName", default, skip_serializing_if = "Option::is_none")]
    pub submitter_name: Option<String>,
    #[serde(rename = "submitterId", default, skip_serializing_if = "Option::is_none")]
    pub submitter_id: Option<String>,
    #[serde(rename = "artifactId", default, skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    #[serde(rename = "jobType", default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<spark_session::JobType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<spark_session::Result>,
    #[serde(rename = "schedulerInfo", default, skip_serializing_if = "Option::is_none")]
    pub scheduler_info: Option<SparkScheduler>,
    #[serde(rename = "pluginInfo", default, skip_serializing_if = "Option::is_none")]
    pub plugin_info: Option<SparkServicePlugin>,
    #[serde(rename = "errorInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub error_info: Vec<SparkServiceError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub id: i32,
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "appInfo", default, skip_serializing_if = "Option::is_none")]
    pub app_info: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub log: Vec<String>,
}
impl SparkSession {
    pub fn new(id: i32) -> Self {
        Self {
            livy_info: None,
            name: None,
            workspace_name: None,
            spark_pool_name: None,
            submitter_name: None,
            submitter_id: None,
            artifact_id: None,
            job_type: None,
            result: None,
            scheduler_info: None,
            plugin_info: None,
            error_info: Vec::new(),
            tags: None,
            id,
            app_id: None,
            app_info: None,
            state: None,
            log: Vec::new(),
        }
    }
}
pub mod spark_session {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "JobType")]
    pub enum JobType {
        SparkBatch,
        SparkSession,
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
                Self::SparkBatch => serializer.serialize_unit_variant("JobType", 0u32, "SparkBatch"),
                Self::SparkSession => serializer.serialize_unit_variant("JobType", 1u32, "SparkSession"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Result")]
    pub enum Result {
        Uncertain,
        Succeeded,
        Failed,
        Cancelled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Result {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Result {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Result {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Uncertain => serializer.serialize_unit_variant("Result", 0u32, "Uncertain"),
                Self::Succeeded => serializer.serialize_unit_variant("Result", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Result", 2u32, "Failed"),
                Self::Cancelled => serializer.serialize_unit_variant("Result", 3u32, "Cancelled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkSessionCollection {
    pub from: i32,
    pub total: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sessions: Vec<SparkSession>,
}
impl SparkSessionCollection {
    pub fn new(from: i32, total: i32) -> Self {
        Self {
            from,
            total,
            sessions: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkSessionOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "artifactId", default, skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(rename = "className", default, skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jars: Vec<String>,
    #[serde(rename = "pyFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub py_files: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub archives: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conf: Option<serde_json::Value>,
    #[serde(rename = "driverMemory", default, skip_serializing_if = "Option::is_none")]
    pub driver_memory: Option<String>,
    #[serde(rename = "driverCores", default, skip_serializing_if = "Option::is_none")]
    pub driver_cores: Option<i32>,
    #[serde(rename = "executorMemory", default, skip_serializing_if = "Option::is_none")]
    pub executor_memory: Option<String>,
    #[serde(rename = "executorCores", default, skip_serializing_if = "Option::is_none")]
    pub executor_cores: Option<i32>,
    #[serde(rename = "numExecutors", default, skip_serializing_if = "Option::is_none")]
    pub num_executors: Option<i32>,
}
impl SparkSessionOptions {
    pub fn new(name: String) -> Self {
        Self {
            tags: None,
            artifact_id: None,
            name,
            file: None,
            class_name: None,
            args: Vec::new(),
            jars: Vec::new(),
            py_files: Vec::new(),
            files: Vec::new(),
            archives: Vec::new(),
            conf: None,
            driver_memory: None,
            driver_cores: None,
            executor_memory: None,
            executor_cores: None,
            num_executors: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkSessionState {
    #[serde(rename = "notStartedAt", default, with = "azure_core::date::rfc3339::option")]
    pub not_started_at: Option<time::OffsetDateTime>,
    #[serde(rename = "startingAt", default, with = "azure_core::date::rfc3339::option")]
    pub starting_at: Option<time::OffsetDateTime>,
    #[serde(rename = "idleAt", default, with = "azure_core::date::rfc3339::option")]
    pub idle_at: Option<time::OffsetDateTime>,
    #[serde(rename = "deadAt", default, with = "azure_core::date::rfc3339::option")]
    pub dead_at: Option<time::OffsetDateTime>,
    #[serde(rename = "shuttingDownAt", default, with = "azure_core::date::rfc3339::option")]
    pub shutting_down_at: Option<time::OffsetDateTime>,
    #[serde(rename = "killedAt", default, with = "azure_core::date::rfc3339::option")]
    pub killed_at: Option<time::OffsetDateTime>,
    #[serde(rename = "recoveringAt", default, with = "azure_core::date::rfc3339::option")]
    pub recovering_at: Option<time::OffsetDateTime>,
    #[serde(rename = "busyAt", default, with = "azure_core::date::rfc3339::option")]
    pub busy_at: Option<time::OffsetDateTime>,
    #[serde(rename = "errorAt", default, with = "azure_core::date::rfc3339::option")]
    pub error_at: Option<time::OffsetDateTime>,
    #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    #[serde(rename = "jobCreationRequest", default, skip_serializing_if = "Option::is_none")]
    pub job_creation_request: Option<SparkRequest>,
}
impl SparkSessionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkStatement {
    pub id: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<SparkStatementOutput>,
}
impl SparkStatement {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            code: None,
            state: None,
            output: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkStatementCancellationResult {
    #[doc = "The msg property from the Livy API. The value is always \"canceled\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}
impl SparkStatementCancellationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkStatementCollection {
    pub total_statements: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statements: Vec<SparkStatement>,
}
impl SparkStatementCollection {
    pub fn new(total_statements: i32) -> Self {
        Self {
            total_statements,
            statements: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkStatementOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<spark_statement_options::Kind>,
}
impl SparkStatementOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod spark_statement_options {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "spark")]
        Spark,
        #[serde(rename = "pyspark")]
        Pyspark,
        #[serde(rename = "dotnetspark")]
        Dotnetspark,
        #[serde(rename = "sql")]
        Sql,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Spark => serializer.serialize_unit_variant("Kind", 0u32, "spark"),
                Self::Pyspark => serializer.serialize_unit_variant("Kind", 1u32, "pyspark"),
                Self::Dotnetspark => serializer.serialize_unit_variant("Kind", 2u32, "dotnetspark"),
                Self::Sql => serializer.serialize_unit_variant("Kind", 3u32, "sql"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkStatementOutput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    pub execution_count: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evalue: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub traceback: Vec<String>,
}
impl SparkStatementOutput {
    pub fn new(execution_count: i32) -> Self {
        Self {
            status: None,
            execution_count,
            data: None,
            ename: None,
            evalue: None,
            traceback: Vec::new(),
        }
    }
}

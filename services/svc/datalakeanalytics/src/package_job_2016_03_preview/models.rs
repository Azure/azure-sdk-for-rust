#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Diagnostics {
    #[doc = "the column where the error occurred."]
    #[serde(rename = "columnNumber", default, skip_serializing_if = "Option::is_none")]
    pub column_number: Option<i32>,
    #[doc = "the ending index of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    #[doc = "the line number the error occurred on."]
    #[serde(rename = "lineNumber", default, skip_serializing_if = "Option::is_none")]
    pub line_number: Option<i32>,
    #[doc = "the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "the severity of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<diagnostics::Severity>,
    #[doc = "the starting index of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
impl Diagnostics {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod diagnostics {
    use super::*;
    #[doc = "the severity of the error."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Warning,
        Error,
        Info,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HiveJobProperties {
    #[serde(flatten)]
    pub job_properties: JobProperties,
    #[doc = "the Hive logs location"]
    #[serde(rename = "logsLocation", default, skip_serializing_if = "Option::is_none")]
    pub logs_location: Option<String>,
    #[doc = "the location of Hive job output files (both execution output and results)"]
    #[serde(rename = "outputLocation", default, skip_serializing_if = "Option::is_none")]
    pub output_location: Option<String>,
    #[doc = "the number of statements that will be run based on the script"]
    #[serde(rename = "statementCount", default, skip_serializing_if = "Option::is_none")]
    pub statement_count: Option<i32>,
    #[doc = "the number of statements that have been run based on the script"]
    #[serde(rename = "executedStatementCount", default, skip_serializing_if = "Option::is_none")]
    pub executed_statement_count: Option<i32>,
}
impl HiveJobProperties {
    pub fn new(job_properties: JobProperties) -> Self {
        Self {
            job_properties,
            logs_location: None,
            output_location: None,
            statement_count: None,
            executed_statement_count: None,
        }
    }
}
#[doc = "A Data Lake Analytics job data path item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobDataPath {
    #[doc = "the id of the job this data is for."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "the command that this job data relates to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[doc = "the list of paths to all of the job data."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<String>,
}
impl JobDataPath {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Data Lake Analytics job error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobErrorDetails {
    #[doc = "the error message description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "the details of the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[doc = "the end offset in the job where the error was found."]
    #[serde(rename = "endOffset", default, skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i32>,
    #[doc = "the specific identifier for the type of error encountered in the job."]
    #[serde(rename = "errorId", default, skip_serializing_if = "Option::is_none")]
    pub error_id: Option<String>,
    #[doc = "the path to any supplemental error files, if any."]
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[doc = "the link to MSDN or Azure help for this type of error, if any."]
    #[serde(rename = "helpLink", default, skip_serializing_if = "Option::is_none")]
    pub help_link: Option<String>,
    #[doc = "the internal diagnostic stack trace if the user requesting the job error details has sufficient permissions it will be retrieved, otherwise it will be empty."]
    #[serde(rename = "internalDiagnostics", default, skip_serializing_if = "Option::is_none")]
    pub internal_diagnostics: Option<String>,
    #[doc = "the specific line number in the job where the error occurred."]
    #[serde(rename = "lineNumber", default, skip_serializing_if = "Option::is_none")]
    pub line_number: Option<i32>,
    #[doc = "the user friendly error message for the failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "the recommended resolution for the failure, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[doc = "The Data Lake Analytics job error details."]
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<JobInnerError>,
    #[doc = "the severity level of the failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<job_error_details::Severity>,
    #[doc = "the ultimate source of the failure (usually either SYSTEM or USER)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "the start offset in the job where the error was found"]
    #[serde(rename = "startOffset", default, skip_serializing_if = "Option::is_none")]
    pub start_offset: Option<i32>,
}
impl JobErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_error_details {
    use super::*;
    #[doc = "the severity level of the failure."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Warning,
        Error,
        Info,
    }
}
#[doc = "List of jobInfo items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobInfoListResult {
    #[doc = "the list of jobInfo items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobInformation>,
    #[doc = "the link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "the total count of results that are available, but might not be returned in the current page."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for JobInfoListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobInfoListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The common Data Lake Analytics job information properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobInformation {
    #[doc = "the job's unique identifier (a GUID)."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "the friendly name of the job."]
    pub name: String,
    #[doc = "the job type of the current job (Hive or USql)."]
    #[serde(rename = "type")]
    pub type_: job_information::Type,
    #[doc = "the user or account that submitted the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitter: Option<String>,
    #[doc = "the error message details for the job, if the job failed."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Vec::is_empty")]
    pub error_message: Vec<JobErrorDetails>,
    #[doc = "the degree of parallelism used for this job. This must be greater than 0."]
    #[serde(rename = "degreeOfParallelism", default, skip_serializing_if = "Option::is_none")]
    pub degree_of_parallelism: Option<i32>,
    #[doc = "the degree of parallelism in percentage used for this job."]
    #[serde(rename = "degreeOfParallelismPercent", default, skip_serializing_if = "Option::is_none")]
    pub degree_of_parallelism_percent: Option<f64>,
    #[doc = "the priority value for the current job. Lower numbers have a higher priority. By default, a job has a priority of 1000. This must be greater than 0."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "the time the job was submitted to the service."]
    #[serde(rename = "submitTime", with = "azure_core::date::rfc3339::option")]
    pub submit_time: Option<time::OffsetDateTime>,
    #[doc = "the start time of the job."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "the completion time of the job."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "the job state. When the job is in the Ended state, refer to Result and ErrorMessage for details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<job_information::State>,
    #[doc = "the result of job execution or the current result of the running job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<job_information::Result>,
    #[doc = "the log folder path to use in the following format: adl://<accountName>.azuredatalakestore.net/system/jobservice/jobs/Usql/2016/03/13/17/18/5fe51957-93bc-4de0-8ddc-c5a4753b068b/logs/."]
    #[serde(rename = "logFolder", default, skip_serializing_if = "Option::is_none")]
    pub log_folder: Option<String>,
    #[doc = "the list of log file name patterns to find in the logFolder. '*' is the only matching character allowed. Example format: jobExecution*.log or *mylog*.txt"]
    #[serde(rename = "logFilePatterns", default, skip_serializing_if = "Vec::is_empty")]
    pub log_file_patterns: Vec<String>,
    #[doc = "the job state audit records, indicating when various operations have been performed on this job."]
    #[serde(rename = "stateAuditRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub state_audit_records: Vec<JobStateAuditRecord>,
    #[doc = "the name of hierarchy queue node this job is assigned to, null if job has not been assigned yet or the account doesn't have hierarchy queue."]
    #[serde(rename = "hierarchyQueueNode", default, skip_serializing_if = "Option::is_none")]
    pub hierarchy_queue_node: Option<String>,
    #[doc = "The common Data Lake Analytics job properties."]
    pub properties: JobProperties,
}
impl JobInformation {
    pub fn new(name: String, type_: job_information::Type, properties: JobProperties) -> Self {
        Self {
            job_id: None,
            name,
            type_,
            submitter: None,
            error_message: Vec::new(),
            degree_of_parallelism: None,
            degree_of_parallelism_percent: None,
            priority: None,
            submit_time: None,
            start_time: None,
            end_time: None,
            state: None,
            result: None,
            log_folder: None,
            log_file_patterns: Vec::new(),
            state_audit_records: Vec::new(),
            hierarchy_queue_node: None,
            properties,
        }
    }
}
pub mod job_information {
    use super::*;
    #[doc = "the job type of the current job (Hive or USql)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        USql,
        Hive,
    }
    #[doc = "the job state. When the job is in the Ended state, refer to Result and ErrorMessage for details."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Accepted,
        Compiling,
        Ended,
        New,
        Queued,
        Running,
        Scheduling,
        Starting,
        Paused,
        WaitingForCapacity,
    }
    #[doc = "the result of job execution or the current result of the running job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        None,
        Succeeded,
        Cancelled,
        Failed,
    }
}
#[doc = "The Data Lake Analytics job error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobInnerError {
    #[doc = "the diagnostic error code."]
    #[serde(rename = "diagnosticCode", default, skip_serializing_if = "Option::is_none")]
    pub diagnostic_code: Option<i32>,
    #[doc = "the severity level of the failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<job_inner_error::Severity>,
    #[doc = "the details of the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[doc = "the component that failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[doc = "the specific identifier for the type of error encountered in the job."]
    #[serde(rename = "errorId", default, skip_serializing_if = "Option::is_none")]
    pub error_id: Option<String>,
    #[doc = "the link to MSDN or Azure help for this type of error, if any."]
    #[serde(rename = "helpLink", default, skip_serializing_if = "Option::is_none")]
    pub help_link: Option<String>,
    #[doc = "the internal diagnostic stack trace if the user requesting the job error details has sufficient permissions it will be retrieved, otherwise it will be empty."]
    #[serde(rename = "internalDiagnostics", default, skip_serializing_if = "Option::is_none")]
    pub internal_diagnostics: Option<String>,
    #[doc = "the user friendly error message for the failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "the recommended resolution for the failure, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[doc = "the ultimate source of the failure (usually either SYSTEM or USER)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "the error message description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl JobInnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_inner_error {
    use super::*;
    #[doc = "the severity level of the failure."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Warning,
        Error,
        Info,
    }
}
#[doc = "The common Data Lake Analytics job properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobProperties {
    #[doc = "the runtime version of the Data Lake Analytics engine to use for the specific type of job being run."]
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    #[doc = "the script to run"]
    pub script: String,
    #[doc = "the job type of the current job (i.e. Hive or U-SQL)."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl JobProperties {
    pub fn new(script: String, type_: String) -> Self {
        Self {
            runtime_version: None,
            script,
            type_,
        }
    }
}
#[doc = "The Data Lake Analytics job resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobResource {
    #[doc = "the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "the path to the resource."]
    #[serde(rename = "resourcePath", default, skip_serializing_if = "Option::is_none")]
    pub resource_path: Option<String>,
    #[doc = "the job resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<job_resource::Type>,
}
impl JobResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_resource {
    use super::*;
    #[doc = "the job resource type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        VertexResource,
        JobManagerResource,
        StatisticsResource,
        VertexResourceInUserFolder,
        JobManagerResourceInUserFolder,
        StatisticsResourceInUserFolder,
    }
}
#[doc = "The Data Lake Analytics job state audit records for tracking the lifecycle of a job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStateAuditRecord {
    #[doc = "the new state the job is in."]
    #[serde(rename = "newState", default, skip_serializing_if = "Option::is_none")]
    pub new_state: Option<String>,
    #[doc = "the time stamp that the state change took place."]
    #[serde(rename = "timeStamp", with = "azure_core::date::rfc3339::option")]
    pub time_stamp: Option<time::OffsetDateTime>,
    #[doc = "the user who requests the change."]
    #[serde(rename = "requestedByUser", default, skip_serializing_if = "Option::is_none")]
    pub requested_by_user: Option<String>,
    #[doc = " the details of the audit log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl JobStateAuditRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Data Lake Analytics job execution statistics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStatistics {
    #[doc = "the last update time for the statistics."]
    #[serde(rename = "lastUpdateTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub last_update_time_utc: Option<time::OffsetDateTime>,
    #[doc = "the job finalizing start time."]
    #[serde(rename = "finalizingTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub finalizing_time_utc: Option<time::OffsetDateTime>,
    #[doc = "the list of stages for the job."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stages: Vec<JobStatisticsVertexStage>,
}
impl JobStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Data Lake Analytics job statistics vertex stage information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStatisticsVertexStage {
    #[doc = "the amount of data read, in bytes."]
    #[serde(rename = "dataRead", default, skip_serializing_if = "Option::is_none")]
    pub data_read: Option<i64>,
    #[doc = "the amount of data read across multiple pods, in bytes."]
    #[serde(rename = "dataReadCrossPod", default, skip_serializing_if = "Option::is_none")]
    pub data_read_cross_pod: Option<i64>,
    #[doc = "the amount of data read in one pod, in bytes."]
    #[serde(rename = "dataReadIntraPod", default, skip_serializing_if = "Option::is_none")]
    pub data_read_intra_pod: Option<i64>,
    #[doc = "the amount of data remaining to be read, in bytes."]
    #[serde(rename = "dataToRead", default, skip_serializing_if = "Option::is_none")]
    pub data_to_read: Option<i64>,
    #[doc = "the amount of data written, in bytes."]
    #[serde(rename = "dataWritten", default, skip_serializing_if = "Option::is_none")]
    pub data_written: Option<i64>,
    #[doc = "the number of duplicates that were discarded."]
    #[serde(rename = "duplicateDiscardCount", default, skip_serializing_if = "Option::is_none")]
    pub duplicate_discard_count: Option<i32>,
    #[doc = "the number of failures that occurred in this stage."]
    #[serde(rename = "failedCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i32>,
    #[doc = "the maximum amount of data read in a single vertex, in bytes."]
    #[serde(rename = "maxVertexDataRead", default, skip_serializing_if = "Option::is_none")]
    pub max_vertex_data_read: Option<i64>,
    #[doc = "the minimum amount of data read in a single vertex, in bytes."]
    #[serde(rename = "minVertexDataRead", default, skip_serializing_if = "Option::is_none")]
    pub min_vertex_data_read: Option<i64>,
    #[doc = "the number of read failures in this stage."]
    #[serde(rename = "readFailureCount", default, skip_serializing_if = "Option::is_none")]
    pub read_failure_count: Option<i32>,
    #[doc = "the number of vertices that were revoked during this stage."]
    #[serde(rename = "revocationCount", default, skip_serializing_if = "Option::is_none")]
    pub revocation_count: Option<i32>,
    #[doc = "the number of currently running vertices in this stage."]
    #[serde(rename = "runningCount", default, skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,
    #[doc = "the number of currently scheduled vertices in this stage"]
    #[serde(rename = "scheduledCount", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_count: Option<i32>,
    #[doc = "the name of this stage in job execution."]
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[doc = "the number of vertices that succeeded in this stage."]
    #[serde(rename = "succeededCount", default, skip_serializing_if = "Option::is_none")]
    pub succeeded_count: Option<i32>,
    #[doc = "the amount of temporary data written, in bytes."]
    #[serde(rename = "tempDataWritten", default, skip_serializing_if = "Option::is_none")]
    pub temp_data_written: Option<i64>,
    #[doc = "the total vertex count for this stage."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[doc = "the amount of time that failed vertices took up in this stage."]
    #[serde(rename = "totalFailedTime", default, skip_serializing_if = "Option::is_none")]
    pub total_failed_time: Option<String>,
    #[doc = "the current progress of this stage, as a percentage."]
    #[serde(rename = "totalProgress", default, skip_serializing_if = "Option::is_none")]
    pub total_progress: Option<i32>,
    #[doc = "the amount of time all successful vertices took in this stage."]
    #[serde(rename = "totalSucceededTime", default, skip_serializing_if = "Option::is_none")]
    pub total_succeeded_time: Option<String>,
}
impl JobStatisticsVertexStage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct USqlJobProperties {
    #[serde(flatten)]
    pub job_properties: JobProperties,
    #[doc = "the list of resources that are required by the job"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<JobResource>,
    #[doc = "The Data Lake Analytics job execution statistics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<JobStatistics>,
    #[doc = "A Data Lake Analytics job data path item."]
    #[serde(rename = "debugData", default, skip_serializing_if = "Option::is_none")]
    pub debug_data: Option<JobDataPath>,
    #[doc = "the diagnostics for the job."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostics>,
    #[doc = "the algebra file path after the job has completed"]
    #[serde(rename = "algebraFilePath", default, skip_serializing_if = "Option::is_none")]
    pub algebra_file_path: Option<String>,
    #[doc = "the total time this job spent compiling. This value should not be set by the user and will be ignored if it is."]
    #[serde(rename = "totalCompilationTime", default, skip_serializing_if = "Option::is_none")]
    pub total_compilation_time: Option<String>,
    #[doc = "the total time this job spent paused. This value should not be set by the user and will be ignored if it is."]
    #[serde(rename = "totalPauseTime", default, skip_serializing_if = "Option::is_none")]
    pub total_pause_time: Option<String>,
    #[doc = "the total time this job spent queued. This value should not be set by the user and will be ignored if it is."]
    #[serde(rename = "totalQueuedTime", default, skip_serializing_if = "Option::is_none")]
    pub total_queued_time: Option<String>,
    #[doc = "the total time this job spent executing. This value should not be set by the user and will be ignored if it is."]
    #[serde(rename = "totalRunningTime", default, skip_serializing_if = "Option::is_none")]
    pub total_running_time: Option<String>,
    #[doc = "the ID used to identify the job manager coordinating job execution. This value should not be set by the user and will be ignored if it is."]
    #[serde(rename = "rootProcessNodeId", default, skip_serializing_if = "Option::is_none")]
    pub root_process_node_id: Option<String>,
    #[doc = "the ID used to identify the yarn application executing the job. This value should not be set by the user and will be ignored if it is."]
    #[serde(rename = "yarnApplicationId", default, skip_serializing_if = "Option::is_none")]
    pub yarn_application_id: Option<String>,
    #[doc = "the timestamp (in ticks) for the yarn application executing the job. This value should not be set by the user and will be ignored if it is."]
    #[serde(rename = "yarnApplicationTimeStamp", default, skip_serializing_if = "Option::is_none")]
    pub yarn_application_time_stamp: Option<i64>,
    #[doc = "the compile mode for the job."]
    #[serde(rename = "compileMode", default, skip_serializing_if = "Option::is_none")]
    pub compile_mode: Option<u_sql_job_properties::CompileMode>,
}
impl USqlJobProperties {
    pub fn new(job_properties: JobProperties) -> Self {
        Self {
            job_properties,
            resources: Vec::new(),
            statistics: None,
            debug_data: None,
            diagnostics: Vec::new(),
            algebra_file_path: None,
            total_compilation_time: None,
            total_pause_time: None,
            total_queued_time: None,
            total_running_time: None,
            root_process_node_id: None,
            yarn_application_id: None,
            yarn_application_time_stamp: None,
            compile_mode: None,
        }
    }
}
pub mod u_sql_job_properties {
    use super::*;
    #[doc = "the compile mode for the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CompileMode {
        Semantic,
        Full,
        SingleBox,
    }
}

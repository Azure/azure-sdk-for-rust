#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BasicAuthentication {
    #[serde(flatten)]
    pub http_authentication: HttpAuthentication,
    #[doc = "Gets or sets the username."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "Gets or sets the password."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl BasicAuthentication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientCertAuthentication {
    #[serde(flatten)]
    pub http_authentication: HttpAuthentication,
    #[doc = "Gets or sets the password."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Gets or sets the pfx."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pfx: Option<String>,
    #[doc = "Gets or sets the certificate thumbprint."]
    #[serde(rename = "certificateThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub certificate_thumbprint: Option<String>,
    #[doc = "Gets or sets the certificate expiration date."]
    #[serde(rename = "certificateExpirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub certificate_expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the certificate subject name."]
    #[serde(rename = "certificateSubjectName", default, skip_serializing_if = "Option::is_none")]
    pub certificate_subject_name: Option<String>,
}
impl ClientCertAuthentication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpAuthentication {
    #[doc = "Gets or sets the http authentication type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<http_authentication::Type>,
}
impl HttpAuthentication {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod http_authentication {
    use super::*;
    #[doc = "Gets or sets the http authentication type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        NotSpecified,
        ClientCertificate,
        ActiveDirectoryOAuth,
        Basic,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<HttpAuthentication>,
    #[doc = "Gets or sets the Uri."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Gets or sets the method of the request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[doc = "Gets or sets the request body."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[doc = "Gets or sets the headers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
}
impl HttpRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobAction {
    #[doc = "Gets or sets the job action type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<job_action::Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<HttpRequest>,
    #[serde(rename = "queueMessage", default, skip_serializing_if = "Option::is_none")]
    pub queue_message: Option<StorageQueueMessage>,
    #[serde(rename = "serviceBusQueueMessage", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_queue_message: Option<ServiceBusQueueMessage>,
    #[serde(rename = "serviceBusTopicMessage", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_topic_message: Option<ServiceBusTopicMessage>,
    #[serde(rename = "retryPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
    #[serde(rename = "errorAction", default, skip_serializing_if = "Option::is_none")]
    pub error_action: Option<JobErrorAction>,
}
impl JobAction {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_action {
    use super::*;
    #[doc = "Gets or sets the job action type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Http,
        Https,
        StorageQueue,
        ServiceBusQueue,
        ServiceBusTopic,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobCollectionDefinition {
    #[doc = "Gets the job collection resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the job collection resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the job collection resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the storage account location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets the tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobCollectionProperties>,
}
impl JobCollectionDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobCollectionListResult {
    #[doc = "Gets the job collections."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobCollectionDefinition>,
    #[doc = "Gets or sets the URL to get the next set of job collections."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobCollectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobCollectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobCollectionProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Gets or sets the state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<job_collection_properties::State>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quota: Option<JobCollectionQuota>,
}
impl JobCollectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_collection_properties {
    use super::*;
    #[doc = "Gets or sets the state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
        Suspended,
        Deleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobCollectionQuota {
    #[doc = "Gets or set the maximum job count."]
    #[serde(rename = "maxJobCount", default, skip_serializing_if = "Option::is_none")]
    pub max_job_count: Option<i64>,
    #[doc = "Gets or sets the maximum job occurrence."]
    #[serde(rename = "maxJobOccurrence", default, skip_serializing_if = "Option::is_none")]
    pub max_job_occurrence: Option<i64>,
    #[serde(rename = "maxRecurrence", default, skip_serializing_if = "Option::is_none")]
    pub max_recurrence: Option<JobMaxRecurrence>,
}
impl JobCollectionQuota {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobDefinition {
    #[doc = "Gets the job resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the job resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets the job resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
}
impl JobDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobErrorAction {
    #[doc = "Gets or sets the job error action type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<job_error_action::Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<HttpRequest>,
    #[serde(rename = "queueMessage", default, skip_serializing_if = "Option::is_none")]
    pub queue_message: Option<StorageQueueMessage>,
    #[serde(rename = "serviceBusQueueMessage", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_queue_message: Option<ServiceBusQueueMessage>,
    #[serde(rename = "serviceBusTopicMessage", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_topic_message: Option<ServiceBusTopicMessage>,
    #[serde(rename = "retryPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
}
impl JobErrorAction {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_error_action {
    use super::*;
    #[doc = "Gets or sets the job error action type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Http,
        Https,
        StorageQueue,
        ServiceBusQueue,
        ServiceBusTopic,
    }
}
#[doc = "Gets the job execution status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JobExecutionStatus {
    Completed,
    Failed,
    Postponed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobHistoryDefinition {
    #[doc = "Gets the job history identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the job history resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets the job history name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobHistoryDefinitionProperties>,
}
impl JobHistoryDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobHistoryDefinitionProperties {
    #[doc = "Gets the start time for this job."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the end time for this job."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the expected execution time for this job."]
    #[serde(rename = "expectedExecutionTime", default, with = "azure_core::date::rfc3339::option")]
    pub expected_execution_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the job history action name."]
    #[serde(rename = "actionName", default, skip_serializing_if = "Option::is_none")]
    pub action_name: Option<job_history_definition_properties::ActionName>,
    #[doc = "Gets the job execution status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<JobExecutionStatus>,
    #[doc = "Gets the message for the job history."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Gets the retry count for job."]
    #[serde(rename = "retryCount", default, skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<i64>,
    #[doc = "Gets the repeat count for the job."]
    #[serde(rename = "repeatCount", default, skip_serializing_if = "Option::is_none")]
    pub repeat_count: Option<i64>,
}
impl JobHistoryDefinitionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_history_definition_properties {
    use super::*;
    #[doc = "Gets the job history action name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionName {
        MainAction,
        ErrorAction,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobHistoryFilter {
    #[doc = "Gets the job execution status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<JobExecutionStatus>,
}
impl JobHistoryFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobHistoryListResult {
    #[doc = "Gets or sets the job histories under job."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobHistoryDefinition>,
    #[doc = "Gets or sets the URL to get the next set of job histories."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobHistoryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobHistoryListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobListResult {
    #[doc = "Gets or sets all jobs under job collection."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobDefinition>,
    #[doc = "Gets or sets the URL to get the next set of jobs."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobMaxRecurrence {
    #[doc = "Gets or sets the frequency of recurrence (second, minute, hour, day, week, month)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<job_max_recurrence::Frequency>,
    #[doc = "Gets or sets the interval between retries."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
}
impl JobMaxRecurrence {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_max_recurrence {
    use super::*;
    #[doc = "Gets or sets the frequency of recurrence (second, minute, hour, day, week, month)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Frequency {
        Minute,
        Hour,
        Day,
        Week,
        Month,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobProperties {
    #[doc = "Gets or sets the job start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<JobAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<JobRecurrence>,
    #[doc = "Gets or set the job state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<JobState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,
}
impl JobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobRecurrence {
    #[doc = "Gets or sets the frequency of recurrence (second, minute, hour, day, week, month)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<job_recurrence::Frequency>,
    #[doc = "Gets or sets the interval between retries."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[doc = "Gets or sets the maximum number of times that the job should run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Gets or sets the time at which the job will complete."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<JobRecurrenceSchedule>,
}
impl JobRecurrence {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_recurrence {
    use super::*;
    #[doc = "Gets or sets the frequency of recurrence (second, minute, hour, day, week, month)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Frequency {
        Minute,
        Hour,
        Day,
        Week,
        Month,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobRecurrenceSchedule {
    #[doc = "Gets or sets the days of the week that the job should execute on."]
    #[serde(rename = "weekDays", default, skip_serializing_if = "Vec::is_empty")]
    pub week_days: Vec<String>,
    #[doc = "Gets or sets the hours of the day that the job should execute at."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hours: Vec<i64>,
    #[doc = "Gets or sets the minutes of the hour that the job should execute at."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub minutes: Vec<i64>,
    #[doc = "Gets or sets the days of the month that the job should execute on. Must be between 1 and 31."]
    #[serde(rename = "monthDays", default, skip_serializing_if = "Vec::is_empty")]
    pub month_days: Vec<i64>,
    #[doc = "Gets or sets the occurrences of days within a month."]
    #[serde(rename = "monthlyOccurrences", default, skip_serializing_if = "Vec::is_empty")]
    pub monthly_occurrences: Vec<JobRecurrenceScheduleMonthlyOccurrence>,
}
impl JobRecurrenceSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobRecurrenceScheduleMonthlyOccurrence {
    #[doc = "Gets or sets the day. Must be one of monday, tuesday, wednesday, thursday, friday, saturday, sunday."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<job_recurrence_schedule_monthly_occurrence::Day>,
    #[doc = "Gets or sets the occurrence. Must be between -5 and 5."]
    #[serde(rename = "Occurrence", default, skip_serializing_if = "Option::is_none")]
    pub occurrence: Option<i64>,
}
impl JobRecurrenceScheduleMonthlyOccurrence {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_recurrence_schedule_monthly_occurrence {
    use super::*;
    #[doc = "Gets or sets the day. Must be one of monday, tuesday, wednesday, thursday, friday, saturday, sunday."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
}
#[doc = "Gets or set the job state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JobState {
    Enabled,
    Disabled,
    Faulted,
    Completed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStateFilter {
    #[doc = "Gets or set the job state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<JobState>,
}
impl JobStateFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStatus {
    #[doc = "Gets the number of times this job has executed."]
    #[serde(rename = "executionCount", default, skip_serializing_if = "Option::is_none")]
    pub execution_count: Option<i64>,
    #[doc = "Gets the number of times this job has failed."]
    #[serde(rename = "failureCount", default, skip_serializing_if = "Option::is_none")]
    pub failure_count: Option<i64>,
    #[doc = "Gets the number of faulted occurrences (occurrences that were retried and failed as many times as the retry policy states)."]
    #[serde(rename = "faultedCount", default, skip_serializing_if = "Option::is_none")]
    pub faulted_count: Option<i64>,
    #[doc = "Gets the time the last occurrence executed in ISO-8601 format.  Could be empty if job has not run yet."]
    #[serde(rename = "lastExecutionTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_execution_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the time of the next occurrence in ISO-8601 format. Could be empty if the job is completed."]
    #[serde(rename = "nextExecutionTime", default, with = "azure_core::date::rfc3339::option")]
    pub next_execution_time: Option<time::OffsetDateTime>,
}
impl JobStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OAuthAuthentication {
    #[serde(flatten)]
    pub http_authentication: HttpAuthentication,
    #[doc = "Gets or sets the secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[doc = "Gets or sets the tenant."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    #[doc = "Gets or sets the audience."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[doc = "Gets or sets the client identifier."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl OAuthAuthentication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the frequency of recurrence (minute, hour, day, week, month)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RecurrenceFrequency {
    Minute,
    Hour,
    Day,
    Week,
    Month,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetryPolicy {
    #[doc = "Gets or sets the retry strategy to be used."]
    #[serde(rename = "retryType", default, skip_serializing_if = "Option::is_none")]
    pub retry_type: Option<retry_policy::RetryType>,
    #[doc = "Gets or sets the retry interval between retries."]
    #[serde(rename = "retryInterval", default, skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<String>,
    #[doc = "Gets or sets the number of times a retry should be attempted."]
    #[serde(rename = "retryCount", default, skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<i64>,
}
impl RetryPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod retry_policy {
    use super::*;
    #[doc = "Gets or sets the retry strategy to be used."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RetryType {
        None,
        Fixed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceBusAuthentication {
    #[doc = "Gets or sets the SAS key."]
    #[serde(rename = "sasKey", default, skip_serializing_if = "Option::is_none")]
    pub sas_key: Option<String>,
    #[doc = "Gets or sets the SAS key name."]
    #[serde(rename = "sasKeyName", default, skip_serializing_if = "Option::is_none")]
    pub sas_key_name: Option<String>,
    #[doc = "Gets or sets the authentication type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<service_bus_authentication::Type>,
}
impl ServiceBusAuthentication {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod service_bus_authentication {
    use super::*;
    #[doc = "Gets or sets the authentication type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        NotSpecified,
        SharedAccessKey,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceBusBrokeredMessageProperties {
    #[doc = "Gets or sets the content type."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "Gets or sets the correlation id."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "Gets or sets the force persistence."]
    #[serde(rename = "forcePersistence", default, skip_serializing_if = "Option::is_none")]
    pub force_persistence: Option<bool>,
    #[doc = "Gets or sets the label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Gets or sets the message id."]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[doc = "Gets or sets the partition key."]
    #[serde(rename = "partitionKey", default, skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
    #[doc = "Gets or sets the reply to."]
    #[serde(rename = "replyTo", default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[doc = "Gets or sets the reply to session id."]
    #[serde(rename = "replyToSessionId", default, skip_serializing_if = "Option::is_none")]
    pub reply_to_session_id: Option<String>,
    #[doc = "Gets or sets the scheduled enqueue time UTC."]
    #[serde(rename = "scheduledEnqueueTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub scheduled_enqueue_time_utc: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the session id."]
    #[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[doc = "Gets or sets the time to live."]
    #[serde(rename = "timeToLive", default, with = "azure_core::date::rfc3339::option")]
    pub time_to_live: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[doc = "Gets or sets the via partition key."]
    #[serde(rename = "viaPartitionKey", default, skip_serializing_if = "Option::is_none")]
    pub via_partition_key: Option<String>,
}
impl ServiceBusBrokeredMessageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceBusMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<ServiceBusAuthentication>,
    #[serde(rename = "brokeredMessageProperties", default, skip_serializing_if = "Option::is_none")]
    pub brokered_message_properties: Option<ServiceBusBrokeredMessageProperties>,
    #[doc = "Gets or sets the custom message properties."]
    #[serde(rename = "customMessageProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_message_properties: Option<serde_json::Value>,
    #[doc = "Gets or sets the message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Gets or sets the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "Gets or sets the transport type."]
    #[serde(rename = "transportType", default, skip_serializing_if = "Option::is_none")]
    pub transport_type: Option<service_bus_message::TransportType>,
}
impl ServiceBusMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod service_bus_message {
    use super::*;
    #[doc = "Gets or sets the transport type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TransportType {
        NotSpecified,
        NetMessaging,
        #[serde(rename = "AMQP")]
        Amqp,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceBusQueueMessage {
    #[serde(flatten)]
    pub service_bus_message: ServiceBusMessage,
    #[doc = "Gets or sets the queue name."]
    #[serde(rename = "queueName", default, skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
}
impl ServiceBusQueueMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceBusTopicMessage {
    #[serde(flatten)]
    pub service_bus_message: ServiceBusMessage,
    #[doc = "Gets or sets the topic path."]
    #[serde(rename = "topicPath", default, skip_serializing_if = "Option::is_none")]
    pub topic_path: Option<String>,
}
impl ServiceBusTopicMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "Gets or set the SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<sku::Name>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku {
    use super::*;
    #[doc = "Gets or set the SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Standard,
        Free,
        Premium,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageQueueMessage {
    #[doc = "Gets or sets the storage account name."]
    #[serde(rename = "storageAccount", default, skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<String>,
    #[doc = "Gets or sets the queue name."]
    #[serde(rename = "queueName", default, skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
    #[doc = "Gets or sets the SAS key."]
    #[serde(rename = "sasToken", default, skip_serializing_if = "Option::is_none")]
    pub sas_token: Option<String>,
    #[doc = "Gets or sets the message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl StorageQueueMessage {
    pub fn new() -> Self {
        Self::default()
    }
}

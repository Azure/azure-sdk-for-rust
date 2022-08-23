#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use azure_core::util::case_insensitive_deserialize;
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountListSupportedImagesResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ImageInformation>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for AccountListSupportedImagesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl AccountListSupportedImagesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AffinityInformation {
    #[doc = "You can pass the affinityId of a Node to indicate that this Task needs to run on that Compute Node. Note that this is just a soft affinity. If the target Compute Node is busy or unavailable at the time the Task is scheduled, then the Task will be scheduled elsewhere."]
    #[serde(rename = "affinityId")]
    pub affinity_id: String,
}
impl AffinityInformation {
    pub fn new(affinity_id: String) -> Self {
        Self { affinity_id }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationSummary>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl ApplicationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPackageReference {
    #[serde(rename = "applicationId")]
    pub application_id: String,
    #[doc = "If this is omitted on a Pool, and no default version is specified for this application, the request fails with the error code InvalidApplicationPackageReferences and HTTP status code 409. If this is omitted on a Task, and no default version is specified for this application, the Task fails with a pre-processing error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ApplicationPackageReference {
    pub fn new(application_id: String) -> Self {
        Self {
            application_id,
            version: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationSummary {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub versions: Vec<String>,
}
impl ApplicationSummary {
    pub fn new(id: String, display_name: String, versions: Vec<String>) -> Self {
        Self {
            id,
            display_name,
            versions,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthenticationTokenSettings {
    #[doc = "The authentication token grants access to a limited set of Batch service operations. Currently the only supported value for the access property is 'job', which grants access to all operations related to the Job which contains the Task."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub access: Vec<String>,
}
impl AuthenticationTokenSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoPoolSpecification {
    #[doc = "The Batch service assigns each auto Pool a unique identifier on creation. To distinguish between Pools created for different purposes, you can specify this element to add a prefix to the ID that is assigned. The prefix can be up to 20 characters long."]
    #[serde(rename = "autoPoolIdPrefix", default, skip_serializing_if = "Option::is_none")]
    pub auto_pool_id_prefix: Option<String>,
    #[serde(rename = "poolLifetimeOption", deserialize_with = "case_insensitive_deserialize")]
    pub pool_lifetime_option: auto_pool_specification::PoolLifetimeOption,
    #[doc = "If false, the Batch service deletes the Pool once its lifetime (as determined by the poolLifetimeOption setting) expires; that is, when the Job or Job Schedule completes. If true, the Batch service does not delete the Pool automatically. It is up to the user to delete auto Pools created with this option."]
    #[serde(rename = "keepAlive", default, skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<PoolSpecification>,
}
impl AutoPoolSpecification {
    pub fn new(pool_lifetime_option: auto_pool_specification::PoolLifetimeOption) -> Self {
        Self {
            auto_pool_id_prefix: None,
            pool_lifetime_option,
            keep_alive: None,
            pool: None,
        }
    }
}
pub mod auto_pool_specification {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PoolLifetimeOption {
        #[serde(rename = "jobschedule")]
        Jobschedule,
        #[serde(rename = "job")]
        Job,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScaleRun {
    #[serde(with = "azure_core::date::rfc3339")]
    pub timestamp: time::OffsetDateTime,
    #[doc = "Each variable value is returned in the form $variable=value, and variables are separated by semicolons."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<AutoScaleRunError>,
}
impl AutoScaleRun {
    pub fn new(timestamp: time::OffsetDateTime) -> Self {
        Self {
            timestamp,
            results: None,
            error: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoScaleRunError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<NameValuePair>,
}
impl AutoScaleRunError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoUserSpecification {
    #[doc = "The default value is pool. If the pool is running Windows a value of Task should be specified if stricter isolation between tasks is required. For example, if the task mutates the registry in a way which could impact other tasks, or if certificates have been specified on the pool which should not be accessible by normal tasks but should be accessible by StartTasks."]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub scope: Option<auto_user_specification::Scope>,
    #[serde(
        rename = "elevationLevel",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub elevation_level: Option<ElevationLevel>,
}
impl AutoUserSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod auto_user_specification {
    use super::*;
    #[doc = "The default value is pool. If the pool is running Windows a value of Task should be specified if stricter isolation between tasks is required. For example, if the task mutates the registry in a way which could impact other tasks, or if certificates have been specified on the pool which should not be accessible by normal tasks but should be accessible by StartTasks."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Scope {
        #[serde(rename = "task")]
        Task,
        #[serde(rename = "pool")]
        Pool,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBlobFileSystemConfiguration {
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "This property is mutually exclusive with sasKey and one must be specified."]
    #[serde(rename = "accountKey", default, skip_serializing_if = "Option::is_none")]
    pub account_key: Option<String>,
    #[doc = "This property is mutually exclusive with accountKey and one must be specified."]
    #[serde(rename = "sasKey", default, skip_serializing_if = "Option::is_none")]
    pub sas_key: Option<String>,
    #[doc = "These are 'net use' options in Windows and 'mount' options in Linux."]
    #[serde(rename = "blobfuseOptions", default, skip_serializing_if = "Option::is_none")]
    pub blobfuse_options: Option<String>,
    #[doc = "All file systems are mounted relative to the Batch mounts directory, accessible via the AZ_BATCH_NODE_MOUNTS_DIR environment variable."]
    #[serde(rename = "relativeMountPath")]
    pub relative_mount_path: String,
}
impl AzureBlobFileSystemConfiguration {
    pub fn new(account_name: String, container_name: String, relative_mount_path: String) -> Self {
        Self {
            account_name,
            container_name,
            account_key: None,
            sas_key: None,
            blobfuse_options: None,
            relative_mount_path,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileShareConfiguration {
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[doc = "This is of the form 'https://{account}.file.core.windows.net/'."]
    #[serde(rename = "azureFileUrl")]
    pub azure_file_url: String,
    #[serde(rename = "accountKey")]
    pub account_key: String,
    #[doc = "All file systems are mounted relative to the Batch mounts directory, accessible via the AZ_BATCH_NODE_MOUNTS_DIR environment variable."]
    #[serde(rename = "relativeMountPath")]
    pub relative_mount_path: String,
    #[doc = "These are 'net use' options in Windows and 'mount' options in Linux."]
    #[serde(rename = "mountOptions", default, skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<String>,
}
impl AzureFileShareConfiguration {
    pub fn new(account_name: String, azure_file_url: String, account_key: String, relative_mount_path: String) -> Self {
        Self {
            account_name,
            azure_file_url,
            account_key,
            relative_mount_path,
            mount_options: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<BatchErrorDetail>,
}
impl azure_core::Continuable for BatchError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl BatchError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl BatchErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CifsMountConfiguration {
    pub username: String,
    pub source: String,
    #[doc = "All file systems are mounted relative to the Batch mounts directory, accessible via the AZ_BATCH_NODE_MOUNTS_DIR environment variable."]
    #[serde(rename = "relativeMountPath")]
    pub relative_mount_path: String,
    #[doc = "These are 'net use' options in Windows and 'mount' options in Linux."]
    #[serde(rename = "mountOptions", default, skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<String>,
    pub password: String,
}
impl CifsMountConfiguration {
    pub fn new(username: String, source: String, relative_mount_path: String, password: String) -> Self {
        Self {
            username,
            source,
            relative_mount_path,
            mount_options: None,
            password,
        }
    }
}
#[doc = "The default value for caching is none. For information about the caching options see: https://blogs.msdn.microsoft.com/windowsazurestorage/2012/06/27/exploring-windows-azure-drives-disks-and-images/."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CachingType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "readonly")]
    Readonly,
    #[serde(rename = "readwrite")]
    Readwrite,
}
#[doc = "A Certificate that can be installed on Compute Nodes and can be used to authenticate operations on the machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Certificate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[serde(rename = "thumbprintAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub thumbprint_algorithm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub state: Option<CertificateState>,
    #[serde(rename = "stateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub state_transition_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "previousState",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub previous_state: Option<CertificateState>,
    #[doc = "This property is not set if the Certificate is in its initial Active state."]
    #[serde(rename = "previousStateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub previous_state_transition_time: Option<time::OffsetDateTime>,
    #[serde(rename = "publicData", default, skip_serializing_if = "Option::is_none")]
    pub public_data: Option<String>,
    #[serde(rename = "deleteCertificateError", default, skip_serializing_if = "Option::is_none")]
    pub delete_certificate_error: Option<DeleteCertificateError>,
}
impl Certificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateAddParameter {
    pub thumbprint: String,
    #[serde(rename = "thumbprintAlgorithm")]
    pub thumbprint_algorithm: String,
    pub data: String,
    #[serde(
        rename = "certificateFormat",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub certificate_format: Option<certificate_add_parameter::CertificateFormat>,
    #[doc = "This must be omitted if the Certificate format is cer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl CertificateAddParameter {
    pub fn new(thumbprint: String, thumbprint_algorithm: String, data: String) -> Self {
        Self {
            thumbprint,
            thumbprint_algorithm,
            data,
            certificate_format: None,
            password: None,
        }
    }
}
pub mod certificate_add_parameter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CertificateFormat {
        #[serde(rename = "pfx")]
        Pfx,
        #[serde(rename = "cer")]
        Cer,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Certificate>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for CertificateListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl CertificateListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateReference {
    pub thumbprint: String,
    #[serde(rename = "thumbprintAlgorithm")]
    pub thumbprint_algorithm: String,
    #[doc = "The default value is currentuser. This property is applicable only for Pools configured with Windows Compute Nodes (that is, created with cloudServiceConfiguration, or with virtualMachineConfiguration using a Windows Image reference). For Linux Compute Nodes, the Certificates are stored in a directory inside the Task working directory and an environment variable AZ_BATCH_CERTIFICATES_DIR is supplied to the Task to query for this location. For Certificates with visibility of 'remoteUser', a 'certs' directory is created in the user's home directory (e.g., /home/{user-name}/certs) and Certificates are placed in that directory."]
    #[serde(
        rename = "storeLocation",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub store_location: Option<certificate_reference::StoreLocation>,
    #[doc = "This property is applicable only for Pools configured with Windows Compute Nodes (that is, created with cloudServiceConfiguration, or with virtualMachineConfiguration using a Windows Image reference). Common store names include: My, Root, CA, Trust, Disallowed, TrustedPeople, TrustedPublisher, AuthRoot, AddressBook, but any custom store name can also be used. The default value is My."]
    #[serde(rename = "storeName", default, skip_serializing_if = "Option::is_none")]
    pub store_name: Option<String>,
    #[doc = "You can specify more than one visibility in this collection. The default is all Accounts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub visibility: Vec<String>,
}
impl CertificateReference {
    pub fn new(thumbprint: String, thumbprint_algorithm: String) -> Self {
        Self {
            thumbprint,
            thumbprint_algorithm,
            store_location: None,
            store_name: None,
            visibility: Vec::new(),
        }
    }
}
pub mod certificate_reference {
    use super::*;
    #[doc = "The default value is currentuser. This property is applicable only for Pools configured with Windows Compute Nodes (that is, created with cloudServiceConfiguration, or with virtualMachineConfiguration using a Windows Image reference). For Linux Compute Nodes, the Certificates are stored in a directory inside the Task working directory and an environment variable AZ_BATCH_CERTIFICATES_DIR is supplied to the Task to query for this location. For Certificates with visibility of 'remoteUser', a 'certs' directory is created in the user's home directory (e.g., /home/{user-name}/certs) and Certificates are placed in that directory."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StoreLocation {
        #[serde(rename = "currentuser")]
        Currentuser,
        #[serde(rename = "localmachine")]
        Localmachine,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CertificateState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "deletefailed")]
    Deletefailed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudJob {
    #[doc = "The ID is case-preserving and case-insensitive (that is, you may not have two IDs within an Account that differ only by case)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "usesTaskDependencies", default, skip_serializing_if = "Option::is_none")]
    pub uses_task_dependencies: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "This is an opaque string. You can use it to detect whether the Job has changed between requests. In particular, you can be pass the ETag when updating a Job to specify that your changes should take effect only if nobody else has modified the Job in the meantime."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "This is the last time at which the Job level data, such as the Job state or priority, changed. It does not factor in task-level changes such as adding new Tasks or Tasks changing state."]
    #[serde(rename = "lastModified", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub state: Option<JobState>,
    #[serde(rename = "stateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub state_transition_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "previousState",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub previous_state: Option<JobState>,
    #[doc = "This property is not set if the Job is in its initial Active state."]
    #[serde(rename = "previousStateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub previous_state_transition_time: Option<time::OffsetDateTime>,
    #[doc = "Priority values can range from -1000 to 1000, with -1000 being the lowest priority and 1000 being the highest priority. The default value is 0."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<JobConstraints>,
    #[doc = "The Job Manager Task is automatically started when the Job is created. The Batch service tries to schedule the Job Manager Task before any other Tasks in the Job. When shrinking a Pool, the Batch service tries to preserve Nodes where Job Manager Tasks are running for as long as possible (that is, Compute Nodes running 'normal' Tasks are removed before Compute Nodes running Job Manager Tasks). When a Job Manager Task fails and needs to be restarted, the system tries to schedule it at the highest priority. If there are no idle Compute Nodes available, the system may terminate one of the running Tasks in the Pool and return it to the queue in order to make room for the Job Manager Task to restart. Note that a Job Manager Task in one Job does not have priority over Tasks in other Jobs. Across Jobs, only Job level priorities are observed. For example, if a Job Manager in a priority 0 Job needs to be restarted, it will not displace Tasks of a priority 1 Job. Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing."]
    #[serde(rename = "jobManagerTask", default, skip_serializing_if = "Option::is_none")]
    pub job_manager_task: Option<JobManagerTask>,
    #[doc = "You can use Job Preparation to prepare a Node to run Tasks for the Job. Activities commonly performed in Job Preparation include: Downloading common resource files used by all the Tasks in the Job. The Job Preparation Task can download these common resource files to the shared location on the Node. (AZ_BATCH_NODE_ROOT_DIR\\shared), or starting a local service on the Node so that all Tasks of that Job can communicate with it. If the Job Preparation Task fails (that is, exhausts its retry count before exiting with exit code 0), Batch will not run Tasks of this Job on the Node. The Compute Node remains ineligible to run Tasks of this Job until it is reimaged. The Compute Node remains active and can be used for other Jobs. The Job Preparation Task can run multiple times on the same Node. Therefore, you should write the Job Preparation Task to handle re-execution. If the Node is rebooted, the Job Preparation Task is run again on the Compute Node before scheduling any other Task of the Job, if rerunOnNodeRebootAfterSuccess is true or if the Job Preparation Task did not previously complete. If the Node is reimaged, the Job Preparation Task is run again before scheduling any Task of the Job. Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing."]
    #[serde(rename = "jobPreparationTask", default, skip_serializing_if = "Option::is_none")]
    pub job_preparation_task: Option<JobPreparationTask>,
    #[doc = "The Job Release Task runs when the Job ends, because of one of the following: The user calls the Terminate Job API, or the Delete Job API while the Job is still active, the Job's maximum wall clock time constraint is reached, and the Job is still active, or the Job's Job Manager Task completed, and the Job is configured to terminate when the Job Manager completes. The Job Release Task runs on each Node where Tasks of the Job have run and the Job Preparation Task ran and completed. If you reimage a Node after it has run the Job Preparation Task, and the Job ends without any further Tasks of the Job running on that Node (and hence the Job Preparation Task does not re-run), then the Job Release Task does not run on that Compute Node. If a Node reboots while the Job Release Task is still running, the Job Release Task runs again when the Compute Node starts up. The Job is not marked as complete until all Job Release Tasks have completed. The Job Release Task runs in the background. It does not occupy a scheduling slot; that is, it does not count towards the maxTasksPerNode limit specified on the Pool."]
    #[serde(rename = "jobReleaseTask", default, skip_serializing_if = "Option::is_none")]
    pub job_release_task: Option<JobReleaseTask>,
    #[doc = "Individual Tasks can override an environment setting specified here by specifying the same setting name with a different value."]
    #[serde(rename = "commonEnvironmentSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub common_environment_settings: Vec<EnvironmentSetting>,
    #[serde(rename = "poolInfo", default, skip_serializing_if = "Option::is_none")]
    pub pool_info: Option<PoolInformation>,
    #[serde(
        rename = "onAllTasksComplete",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub on_all_tasks_complete: Option<OnAllTasksComplete>,
    #[doc = "A Task is considered to have failed if has a failureInfo. A failureInfo is set if the Task completes with a non-zero exit code after exhausting its retry count, or if there was an error starting the Task, for example due to a resource file download error. The default is noaction."]
    #[serde(
        rename = "onTaskFailure",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub on_task_failure: Option<OnTaskFailure>,
    #[serde(rename = "networkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<JobNetworkConfiguration>,
    #[doc = "The Batch service does not assign any meaning to metadata; it is solely for the use of user code."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<MetadataItem>,
    #[serde(rename = "executionInfo", default, skip_serializing_if = "Option::is_none")]
    pub execution_info: Option<JobExecutionInformation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<JobStatistics>,
}
impl CloudJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudJobListPreparationAndReleaseTaskStatusResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobPreparationAndReleaseTaskExecutionInformation>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for CloudJobListPreparationAndReleaseTaskStatusResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl CloudJobListPreparationAndReleaseTaskStatusResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudJobListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CloudJob>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for CloudJobListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl CloudJobListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudJobSchedule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "This is an opaque string. You can use it to detect whether the Job Schedule has changed between requests. In particular, you can be pass the ETag with an Update Job Schedule request to specify that your changes should take effect only if nobody else has modified the schedule in the meantime."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "This is the last time at which the schedule level data, such as the Job specification or recurrence information, changed. It does not factor in job-level changes such as new Jobs being created or Jobs changing state."]
    #[serde(rename = "lastModified", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub state: Option<JobScheduleState>,
    #[serde(rename = "stateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub state_transition_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "previousState",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub previous_state: Option<JobScheduleState>,
    #[doc = "This property is not present if the Job Schedule is in its initial active state."]
    #[serde(rename = "previousStateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub previous_state_transition_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    #[serde(rename = "jobSpecification", default, skip_serializing_if = "Option::is_none")]
    pub job_specification: Option<JobSpecification>,
    #[serde(rename = "executionInfo", default, skip_serializing_if = "Option::is_none")]
    pub execution_info: Option<JobScheduleExecutionInformation>,
    #[doc = "The Batch service does not assign any meaning to metadata; it is solely for the use of user code."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<MetadataItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<JobScheduleStatistics>,
}
impl CloudJobSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudJobScheduleListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CloudJobSchedule>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for CloudJobScheduleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl CloudJobScheduleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudPool {
    #[doc = "The ID can contain any combination of alphanumeric characters including hyphens and underscores, and cannot contain more than 64 characters. The ID is case-preserving and case-insensitive (that is, you may not have two IDs within an Account that differ only by case)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The display name need not be unique and can contain any Unicode characters up to a maximum length of 1024."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "This is an opaque string. You can use it to detect whether the Pool has changed between requests. In particular, you can be pass the ETag when updating a Pool to specify that your changes should take effect only if nobody else has modified the Pool in the meantime."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "This is the last time at which the Pool level data, such as the targetDedicatedNodes or enableAutoscale settings, changed. It does not factor in node-level changes such as a Compute Node changing state."]
    #[serde(rename = "lastModified", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub state: Option<cloud_pool::State>,
    #[serde(rename = "stateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub state_transition_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "allocationState",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub allocation_state: Option<cloud_pool::AllocationState>,
    #[serde(rename = "allocationStateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub allocation_state_transition_time: Option<time::OffsetDateTime>,
    #[doc = "For information about available sizes of virtual machines in Pools, see Choose a VM size for Compute Nodes in an Azure Batch Pool (https://docs.microsoft.com/azure/batch/batch-pool-vm-sizes)."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[serde(rename = "cloudServiceConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cloud_service_configuration: Option<CloudServiceConfiguration>,
    #[serde(rename = "virtualMachineConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_configuration: Option<VirtualMachineConfiguration>,
    #[doc = "This is the timeout for the most recent resize operation. (The initial sizing when the Pool is created counts as a resize.) The default value is 15 minutes."]
    #[serde(rename = "resizeTimeout", default, skip_serializing_if = "Option::is_none")]
    pub resize_timeout: Option<String>,
    #[doc = "This property is set only if one or more errors occurred during the last Pool resize, and only when the Pool allocationState is Steady."]
    #[serde(rename = "resizeErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub resize_errors: Vec<ResizeError>,
    #[serde(rename = "currentDedicatedNodes", default, skip_serializing_if = "Option::is_none")]
    pub current_dedicated_nodes: Option<i32>,
    #[doc = "Low-priority Compute Nodes which have been preempted are included in this count."]
    #[serde(rename = "currentLowPriorityNodes", default, skip_serializing_if = "Option::is_none")]
    pub current_low_priority_nodes: Option<i32>,
    #[serde(rename = "targetDedicatedNodes", default, skip_serializing_if = "Option::is_none")]
    pub target_dedicated_nodes: Option<i32>,
    #[serde(rename = "targetLowPriorityNodes", default, skip_serializing_if = "Option::is_none")]
    pub target_low_priority_nodes: Option<i32>,
    #[doc = "If false, at least one of targetDedicateNodes and targetLowPriorityNodes must be specified. If true, the autoScaleFormula property is required and the Pool automatically resizes according to the formula. The default value is false."]
    #[serde(rename = "enableAutoScale", default, skip_serializing_if = "Option::is_none")]
    pub enable_auto_scale: Option<bool>,
    #[doc = "This property is set only if the Pool automatically scales, i.e. enableAutoScale is true."]
    #[serde(rename = "autoScaleFormula", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale_formula: Option<String>,
    #[doc = "This property is set only if the Pool automatically scales, i.e. enableAutoScale is true."]
    #[serde(rename = "autoScaleEvaluationInterval", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale_evaluation_interval: Option<String>,
    #[serde(rename = "autoScaleRun", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale_run: Option<AutoScaleRun>,
    #[doc = "This imposes restrictions on which Compute Nodes can be assigned to the Pool. Specifying this value can reduce the chance of the requested number of Compute Nodes to be allocated in the Pool."]
    #[serde(rename = "enableInterNodeCommunication", default, skip_serializing_if = "Option::is_none")]
    pub enable_inter_node_communication: Option<bool>,
    #[doc = "The network configuration for a Pool."]
    #[serde(rename = "networkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[doc = "Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing. In some cases the StartTask may be re-run even though the Compute Node was not rebooted. Special care should be taken to avoid StartTasks which create breakaway process or install/launch services from the StartTask working directory, as this will block Batch from being able to re-run the StartTask."]
    #[serde(rename = "startTask", default, skip_serializing_if = "Option::is_none")]
    pub start_task: Option<StartTask>,
    #[doc = "For Windows Nodes, the Batch service installs the Certificates to the specified Certificate store and location. For Linux Compute Nodes, the Certificates are stored in a directory inside the Task working directory and an environment variable AZ_BATCH_CERTIFICATES_DIR is supplied to the Task to query for this location. For Certificates with visibility of 'remoteUser', a 'certs' directory is created in the user's home directory (e.g., /home/{user-name}/certs) and Certificates are placed in that directory."]
    #[serde(rename = "certificateReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub certificate_references: Vec<CertificateReference>,
    #[doc = "Changes to Package references affect all new Nodes joining the Pool, but do not affect Compute Nodes that are already in the Pool until they are rebooted or reimaged. There is a maximum of 10 Package references on any given Pool."]
    #[serde(rename = "applicationPackageReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub application_package_references: Vec<ApplicationPackageReference>,
    #[doc = "The list of application licenses must be a subset of available Batch service application licenses. If a license is requested which is not supported, Pool creation will fail."]
    #[serde(rename = "applicationLicenses", default, skip_serializing_if = "Vec::is_empty")]
    pub application_licenses: Vec<String>,
    #[doc = "The default value is 1. The maximum value is the smaller of 4 times the number of cores of the vmSize of the Pool or 256."]
    #[serde(rename = "maxTasksPerNode", default, skip_serializing_if = "Option::is_none")]
    pub max_tasks_per_node: Option<i32>,
    #[serde(rename = "taskSchedulingPolicy", default, skip_serializing_if = "Option::is_none")]
    pub task_scheduling_policy: Option<TaskSchedulingPolicy>,
    #[serde(rename = "userAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub user_accounts: Vec<UserAccount>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<MetadataItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<PoolStatistics>,
    #[doc = "This supports Azure Files, NFS, CIFS/SMB, and Blobfuse."]
    #[serde(rename = "mountConfiguration", default, skip_serializing_if = "Vec::is_empty")]
    pub mount_configuration: Vec<MountConfiguration>,
}
impl CloudPool {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cloud_pool {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "deleting")]
        Deleting,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AllocationState {
        #[serde(rename = "steady")]
        Steady,
        #[serde(rename = "resizing")]
        Resizing,
        #[serde(rename = "stopping")]
        Stopping,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudPoolListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CloudPool>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for CloudPoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl CloudPoolListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudServiceConfiguration {
    #[doc = "Possible values are:\n2 - OS Family 2, equivalent to Windows Server 2008 R2 SP1.\n3 - OS Family 3, equivalent to Windows Server 2012.\n4 - OS Family 4, equivalent to Windows Server 2012 R2.\n5 - OS Family 5, equivalent to Windows Server 2016.\n6 - OS Family 6, equivalent to Windows Server 2019. For more information, see Azure Guest OS Releases (https://azure.microsoft.com/documentation/articles/cloud-services-guestos-update-matrix/#releases)."]
    #[serde(rename = "osFamily")]
    pub os_family: String,
    #[doc = "The default value is * which specifies the latest operating system version for the specified OS family."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
}
impl CloudServiceConfiguration {
    pub fn new(os_family: String) -> Self {
        Self {
            os_family,
            os_version: None,
        }
    }
}
#[doc = "Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudTask {
    #[doc = "The ID can contain any combination of alphanumeric characters including hyphens and underscores, and cannot contain more than 64 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The display name need not be unique and can contain any Unicode characters up to a maximum length of 1024."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "This is an opaque string. You can use it to detect whether the Task has changed between requests. In particular, you can be pass the ETag when updating a Task to specify that your changes should take effect only if nobody else has modified the Task in the meantime."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "lastModified", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[serde(rename = "exitConditions", default, skip_serializing_if = "Option::is_none")]
    pub exit_conditions: Option<ExitConditions>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub state: Option<TaskState>,
    #[serde(rename = "stateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub state_transition_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "previousState",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub previous_state: Option<TaskState>,
    #[doc = "This property is not set if the Task is in its initial Active state."]
    #[serde(rename = "previousStateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub previous_state_transition_time: Option<time::OffsetDateTime>,
    #[doc = "For multi-instance Tasks, the command line is executed as the primary Task, after the primary Task and all subtasks have finished executing the coordination command line. The command line does not run under a shell, and therefore cannot take advantage of shell features such as environment variable expansion. If you want to take advantage of such features, you should invoke the shell in the command line, for example using \"cmd /c MyCommand\" in Windows or \"/bin/sh -c MyCommand\" in Linux. If the command line refers to file paths, it should use a relative path (relative to the Task working directory), or use the Batch provided environment variable (https://docs.microsoft.com/en-us/azure/batch/batch-compute-node-environment-variables)."]
    #[serde(rename = "commandLine", default, skip_serializing_if = "Option::is_none")]
    pub command_line: Option<String>,
    #[serde(rename = "containerSettings", default, skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<TaskContainerSettings>,
    #[doc = "For multi-instance Tasks, the resource files will only be downloaded to the Compute Node on which the primary Task is executed. There is a maximum size for the list of resource files.  When the max size is exceeded, the request will fail and the response error code will be RequestEntityTooLarge. If this occurs, the collection of ResourceFiles must be reduced in size. This can be achieved using .zip files, Application Packages, or Docker Containers."]
    #[serde(rename = "resourceFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_files: Vec<ResourceFile>,
    #[doc = "For multi-instance Tasks, the files will only be uploaded from the Compute Node on which the primary Task is executed."]
    #[serde(rename = "outputFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub output_files: Vec<OutputFile>,
    #[serde(rename = "environmentSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_settings: Vec<EnvironmentSetting>,
    #[serde(rename = "affinityInfo", default, skip_serializing_if = "Option::is_none")]
    pub affinity_info: Option<AffinityInformation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<TaskConstraints>,
    #[doc = "Specify either the userName or autoUser property, but not both."]
    #[serde(rename = "userIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<UserIdentity>,
    #[serde(rename = "executionInfo", default, skip_serializing_if = "Option::is_none")]
    pub execution_info: Option<TaskExecutionInformation>,
    #[serde(rename = "nodeInfo", default, skip_serializing_if = "Option::is_none")]
    pub node_info: Option<ComputeNodeInformation>,
    #[doc = "Multi-instance Tasks are commonly used to support MPI Tasks. In the MPI case, if any of the subtasks fail (for example due to exiting with a non-zero exit code) the entire multi-instance Task fails. The multi-instance Task is then terminated and retried, up to its retry limit."]
    #[serde(rename = "multiInstanceSettings", default, skip_serializing_if = "Option::is_none")]
    pub multi_instance_settings: Option<MultiInstanceSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<TaskStatistics>,
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<TaskDependencies>,
    #[doc = "Application packages are downloaded and deployed to a shared directory, not the Task working directory. Therefore, if a referenced package is already on the Node, and is up to date, then it is not re-downloaded; the existing copy on the Compute Node is used. If a referenced Package cannot be installed, for example because the package has been deleted or because download failed, the Task fails."]
    #[serde(rename = "applicationPackageReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub application_package_references: Vec<ApplicationPackageReference>,
    #[serde(rename = "authenticationTokenSettings", default, skip_serializing_if = "Option::is_none")]
    pub authentication_token_settings: Option<AuthenticationTokenSettings>,
}
impl CloudTask {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudTaskListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CloudTask>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for CloudTaskListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl CloudTaskListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudTaskListSubtasksResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubtaskInformation>,
}
impl CloudTaskListSubtasksResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeNode {
    #[doc = "Every Compute Node that is added to a Pool is assigned a unique ID. Whenever a Compute Node is removed from a Pool, all of its local files are deleted, and the ID is reclaimed and could be reused for new Compute Nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The low-priority Compute Node has been preempted. Tasks which were running on the Compute Node when it was preempted will be rescheduled when another Compute Node becomes available."]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub state: Option<compute_node::State>,
    #[serde(
        rename = "schedulingState",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub scheduling_state: Option<compute_node::SchedulingState>,
    #[serde(rename = "stateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub state_transition_time: Option<time::OffsetDateTime>,
    #[doc = "This property may not be present if the Compute Node state is unusable."]
    #[serde(rename = "lastBootTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_boot_time: Option<time::OffsetDateTime>,
    #[doc = "This is the time when the Compute Node was initially allocated and doesn't change once set. It is not updated when the Compute Node is service healed or preempted."]
    #[serde(rename = "allocationTime", default, with = "azure_core::date::rfc3339::option")]
    pub allocation_time: Option<time::OffsetDateTime>,
    #[doc = "Every Compute Node that is added to a Pool is assigned a unique IP address. Whenever a Compute Node is removed from a Pool, all of its local files are deleted, and the IP address is reclaimed and could be reused for new Compute Nodes."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Note that this is just a soft affinity. If the target Compute Node is busy or unavailable at the time the Task is scheduled, then the Task will be scheduled elsewhere."]
    #[serde(rename = "affinityId", default, skip_serializing_if = "Option::is_none")]
    pub affinity_id: Option<String>,
    #[doc = "For information about available sizes of virtual machines in Pools, see Choose a VM size for Compute Nodes in an Azure Batch Pool (https://docs.microsoft.com/azure/batch/batch-pool-vm-sizes)."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[serde(rename = "totalTasksRun", default, skip_serializing_if = "Option::is_none")]
    pub total_tasks_run: Option<i32>,
    #[serde(rename = "runningTasksCount", default, skip_serializing_if = "Option::is_none")]
    pub running_tasks_count: Option<i32>,
    #[serde(rename = "totalTasksSucceeded", default, skip_serializing_if = "Option::is_none")]
    pub total_tasks_succeeded: Option<i32>,
    #[doc = "This property is present only if at least one Task has run on this Compute Node since it was assigned to the Pool."]
    #[serde(rename = "recentTasks", default, skip_serializing_if = "Vec::is_empty")]
    pub recent_tasks: Vec<TaskInformation>,
    #[doc = "Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing. In some cases the StartTask may be re-run even though the Compute Node was not rebooted. Special care should be taken to avoid StartTasks which create breakaway process or install/launch services from the StartTask working directory, as this will block Batch from being able to re-run the StartTask."]
    #[serde(rename = "startTask", default, skip_serializing_if = "Option::is_none")]
    pub start_task: Option<StartTask>,
    #[serde(rename = "startTaskInfo", default, skip_serializing_if = "Option::is_none")]
    pub start_task_info: Option<StartTaskInformation>,
    #[doc = "For Windows Nodes, the Batch service installs the Certificates to the specified Certificate store and location. For Linux Compute Nodes, the Certificates are stored in a directory inside the Task working directory and an environment variable AZ_BATCH_CERTIFICATES_DIR is supplied to the Task to query for this location. For Certificates with visibility of 'remoteUser', a 'certs' directory is created in the user's home directory (e.g., /home/{user-name}/certs) and Certificates are placed in that directory."]
    #[serde(rename = "certificateReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub certificate_references: Vec<CertificateReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ComputeNodeError>,
    #[serde(rename = "isDedicated", default, skip_serializing_if = "Option::is_none")]
    pub is_dedicated: Option<bool>,
    #[serde(rename = "endpointConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<ComputeNodeEndpointConfiguration>,
    #[doc = "The Batch Compute Node agent is a program that runs on each Compute Node in the Pool and provides Batch capability on the Compute Node."]
    #[serde(rename = "nodeAgentInfo", default, skip_serializing_if = "Option::is_none")]
    pub node_agent_info: Option<NodeAgentInformation>,
}
impl ComputeNode {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod compute_node {
    use super::*;
    #[doc = "The low-priority Compute Node has been preempted. Tasks which were running on the Compute Node when it was preempted will be rescheduled when another Compute Node becomes available."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "idle")]
        Idle,
        #[serde(rename = "rebooting")]
        Rebooting,
        #[serde(rename = "reimaging")]
        Reimaging,
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "unusable")]
        Unusable,
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "starting")]
        Starting,
        #[serde(rename = "waitingforstarttask")]
        Waitingforstarttask,
        #[serde(rename = "starttaskfailed")]
        Starttaskfailed,
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "leavingpool")]
        Leavingpool,
        #[serde(rename = "offline")]
        Offline,
        #[serde(rename = "preempted")]
        Preempted,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SchedulingState {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
    }
}
#[doc = "The default value is requeue."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ComputeNodeDeallocationOption {
    #[serde(rename = "requeue")]
    Requeue,
    #[serde(rename = "terminate")]
    Terminate,
    #[serde(rename = "taskcompletion")]
    Taskcompletion,
    #[serde(rename = "retaineddata")]
    Retaineddata,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeNodeEndpointConfiguration {
    #[serde(rename = "inboundEndpoints")]
    pub inbound_endpoints: Vec<InboundEndpoint>,
}
impl ComputeNodeEndpointConfiguration {
    pub fn new(inbound_endpoints: Vec<InboundEndpoint>) -> Self {
        Self { inbound_endpoints }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeNodeError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<NameValuePair>,
}
impl ComputeNodeError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeNodeGetRemoteLoginSettingsResult {
    #[serde(rename = "remoteLoginIPAddress")]
    pub remote_login_ip_address: String,
    #[serde(rename = "remoteLoginPort")]
    pub remote_login_port: i32,
}
impl ComputeNodeGetRemoteLoginSettingsResult {
    pub fn new(remote_login_ip_address: String, remote_login_port: i32) -> Self {
        Self {
            remote_login_ip_address,
            remote_login_port,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeNodeInformation {
    #[serde(rename = "affinityId", default, skip_serializing_if = "Option::is_none")]
    pub affinity_id: Option<String>,
    #[serde(rename = "nodeUrl", default, skip_serializing_if = "Option::is_none")]
    pub node_url: Option<String>,
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    #[serde(rename = "nodeId", default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "taskRootDirectory", default, skip_serializing_if = "Option::is_none")]
    pub task_root_directory: Option<String>,
    #[serde(rename = "taskRootDirectoryUrl", default, skip_serializing_if = "Option::is_none")]
    pub task_root_directory_url: Option<String>,
}
impl ComputeNodeInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeNodeListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ComputeNode>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for ComputeNodeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl ComputeNodeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeNodeUser {
    pub name: String,
    #[doc = "The default value is false."]
    #[serde(rename = "isAdmin", default, skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    #[doc = "If omitted, the default is 1 day from the current time. For Linux Compute Nodes, the expiryTime has a precision up to a day."]
    #[serde(rename = "expiryTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[doc = "The password is required for Windows Compute Nodes (those created with 'cloudServiceConfiguration', or created with 'virtualMachineConfiguration' using a Windows Image reference). For Linux Compute Nodes, the password can optionally be specified along with the sshPublicKey property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The public key should be compatible with OpenSSH encoding and should be base 64 encoded. This property can be specified only for Linux Compute Nodes. If this is specified for a Windows Compute Node, then the Batch service rejects the request; if you are calling the REST API directly, the HTTP status code is 400 (Bad Request)."]
    #[serde(rename = "sshPublicKey", default, skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
}
impl ComputeNodeUser {
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_admin: None,
            expiry_time: None,
            password: None,
            ssh_public_key: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerConfiguration {
    #[serde(rename = "type", deserialize_with = "case_insensitive_deserialize")]
    pub type_: container_configuration::Type,
    #[doc = "This is the full Image reference, as would be specified to \"docker pull\". An Image will be sourced from the default Docker registry unless the Image is fully qualified with an alternative registry."]
    #[serde(rename = "containerImageNames", default, skip_serializing_if = "Vec::is_empty")]
    pub container_image_names: Vec<String>,
    #[doc = "If any Images must be downloaded from a private registry which requires credentials, then those credentials must be provided here."]
    #[serde(rename = "containerRegistries", default, skip_serializing_if = "Vec::is_empty")]
    pub container_registries: Vec<ContainerRegistry>,
}
impl ContainerConfiguration {
    pub fn new(type_: container_configuration::Type) -> Self {
        Self {
            type_,
            container_image_names: Vec::new(),
            container_registries: Vec::new(),
        }
    }
}
pub mod container_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "dockerCompatible", alias = "dockercompatible")]
        DockerCompatible,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerRegistry {
    #[doc = "If omitted, the default is \"docker.io\"."]
    #[serde(rename = "registryServer", default, skip_serializing_if = "Option::is_none")]
    pub registry_server: Option<String>,
    pub username: String,
    pub password: String,
}
impl ContainerRegistry {
    pub fn new(username: String, password: String) -> Self {
        Self {
            registry_server: None,
            username,
            password,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataDisk {
    #[doc = "The lun is used to uniquely identify each data disk. If attaching multiple disks, each should have a distinct lun."]
    pub lun: i32,
    #[doc = "The default value for caching is none. For information about the caching options see: https://blogs.msdn.microsoft.com/windowsazurestorage/2012/06/27/exploring-windows-azure-drives-disks-and-images/."]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub caching: Option<CachingType>,
    #[serde(rename = "diskSizeGB")]
    pub disk_size_gb: i32,
    #[serde(
        rename = "storageAccountType",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub storage_account_type: Option<StorageAccountType>,
}
impl DataDisk {
    pub fn new(lun: i32, disk_size_gb: i32) -> Self {
        Self {
            lun,
            caching: None,
            disk_size_gb,
            storage_account_type: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteCertificateError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "This list includes details such as the active Pools and Compute Nodes referencing this Certificate. However, if a large number of resources reference the Certificate, the list contains only about the first hundred."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<NameValuePair>,
}
impl DeleteCertificateError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The disk encryption configuration applied on compute nodes in the pool. Disk encryption configuration is not supported on Linux pool created with Shared Image Gallery Image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskEncryptionConfiguration {
    #[doc = "If omitted, no disks on the compute nodes in the pool will be encrypted. On Linux pool, only \"TemporaryDisk\" is supported; on Windows pool, \"OsDisk\" and \"TemporaryDisk\" must be specified."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<String>,
}
impl DiskEncryptionConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ElevationLevel {
    #[serde(rename = "nonadmin")]
    Nonadmin,
    #[serde(rename = "admin")]
    Admin,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentSetting {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl EnvironmentSetting {
    pub fn new(name: String) -> Self {
        Self { name, value: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ErrorCategory {
    #[serde(rename = "usererror")]
    Usererror,
    #[serde(rename = "servererror")]
    Servererror,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ErrorMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExitCodeMapping {
    pub code: i32,
    #[serde(rename = "exitOptions")]
    pub exit_options: ExitOptions,
}
impl ExitCodeMapping {
    pub fn new(code: i32, exit_options: ExitOptions) -> Self {
        Self { code, exit_options }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExitCodeRangeMapping {
    pub start: i32,
    pub end: i32,
    #[serde(rename = "exitOptions")]
    pub exit_options: ExitOptions,
}
impl ExitCodeRangeMapping {
    pub fn new(start: i32, end: i32, exit_options: ExitOptions) -> Self {
        Self { start, end, exit_options }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExitConditions {
    #[serde(rename = "exitCodes", default, skip_serializing_if = "Vec::is_empty")]
    pub exit_codes: Vec<ExitCodeMapping>,
    #[serde(rename = "exitCodeRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub exit_code_ranges: Vec<ExitCodeRangeMapping>,
    #[serde(rename = "preProcessingError", default, skip_serializing_if = "Option::is_none")]
    pub pre_processing_error: Option<ExitOptions>,
    #[serde(rename = "fileUploadError", default, skip_serializing_if = "Option::is_none")]
    pub file_upload_error: Option<ExitOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<ExitOptions>,
}
impl ExitConditions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExitOptions {
    #[doc = "The default is none for exit code 0 and terminate for all other exit conditions. If the Job's onTaskFailed property is noaction, then specifying this property returns an error and the add Task request fails with an invalid property value error; if you are calling the REST API directly, the HTTP status code is 400 (Bad Request)."]
    #[serde(
        rename = "jobAction",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub job_action: Option<exit_options::JobAction>,
    #[doc = "Possible values are 'satisfy' (allowing dependent tasks to progress) and 'block' (dependent tasks continue to wait). Batch does not yet support cancellation of dependent tasks."]
    #[serde(
        rename = "dependencyAction",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub dependency_action: Option<exit_options::DependencyAction>,
}
impl ExitOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod exit_options {
    use super::*;
    #[doc = "The default is none for exit code 0 and terminate for all other exit conditions. If the Job's onTaskFailed property is noaction, then specifying this property returns an error and the add Task request fails with an invalid property value error; if you are calling the REST API directly, the HTTP status code is 400 (Bad Request)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum JobAction {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "disable")]
        Disable,
        #[serde(rename = "terminate")]
        Terminate,
    }
    #[doc = "Possible values are 'satisfy' (allowing dependent tasks to progress) and 'block' (dependent tasks continue to wait). Batch does not yet support cancellation of dependent tasks."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DependencyAction {
        #[serde(rename = "satisfy")]
        Satisfy,
        #[serde(rename = "block")]
        Block,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileProperties {
    #[doc = "The creation time is not returned for files on Linux Compute Nodes."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[serde(rename = "lastModified", with = "azure_core::date::rfc3339")]
    pub last_modified: time::OffsetDateTime,
    #[serde(rename = "contentLength")]
    pub content_length: i64,
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The file mode is returned only for files on Linux Compute Nodes."]
    #[serde(rename = "fileMode", default, skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
}
impl FileProperties {
    pub fn new(last_modified: time::OffsetDateTime, content_length: i64) -> Self {
        Self {
            creation_time: None,
            last_modified,
            content_length,
            content_type: None,
            file_mode: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum IpAddressProvisioningType {
    #[serde(rename = "batchmanaged")]
    Batchmanaged,
    #[serde(rename = "usermanaged")]
    Usermanaged,
    #[serde(rename = "nopublicipaddresses")]
    Nopublicipaddresses,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageInformation {
    #[serde(rename = "nodeAgentSKUId")]
    pub node_agent_sku_id: String,
    #[serde(rename = "imageReference")]
    pub image_reference: ImageReference,
    #[serde(rename = "osType", deserialize_with = "case_insensitive_deserialize")]
    pub os_type: image_information::OsType,
    #[doc = "Not every capability of the Image is listed. Capabilities in this list are considered of special interest and are generally related to integration with other features in the Azure Batch service."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<String>,
    #[serde(rename = "batchSupportEndOfLife", default, with = "azure_core::date::rfc3339::option")]
    pub batch_support_end_of_life: Option<time::OffsetDateTime>,
    #[serde(rename = "verificationType", deserialize_with = "case_insensitive_deserialize")]
    pub verification_type: image_information::VerificationType,
}
impl ImageInformation {
    pub fn new(
        node_agent_sku_id: String,
        image_reference: ImageReference,
        os_type: image_information::OsType,
        verification_type: image_information::VerificationType,
    ) -> Self {
        Self {
            node_agent_sku_id,
            image_reference,
            os_type,
            capabilities: Vec::new(),
            batch_support_end_of_life: None,
            verification_type,
        }
    }
}
pub mod image_information {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        #[serde(rename = "linux")]
        Linux,
        #[serde(rename = "windows")]
        Windows,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VerificationType {
        #[serde(rename = "verified")]
        Verified,
        #[serde(rename = "unverified")]
        Unverified,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageReference {
    #[doc = "For example, Canonical or MicrosoftWindowsServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "For example, UbuntuServer or WindowsServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "For example, 18.04-LTS or 2019-Datacenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "A value of 'latest' can be specified to select the latest version of an Image. If omitted, the default is 'latest'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "This property is mutually exclusive with other ImageReference properties. For Virtual Machine Image it must be in the same region and subscription as the Azure Batch account. The Shared Image Gallery Image must have replicas in the same region as the Azure Batch account. For information about the firewall settings for the Batch Compute Node agent to communicate with the Batch service see https://docs.microsoft.com/en-us/azure/batch/batch-api-basics#virtual-network-vnet-and-firewall-configuration."]
    #[serde(rename = "virtualMachineImageId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_image_id: Option<String>,
}
impl ImageReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InboundEndpoint {
    pub name: String,
    #[serde(deserialize_with = "case_insensitive_deserialize")]
    pub protocol: InboundEndpointProtocol,
    #[serde(rename = "publicIPAddress")]
    pub public_ip_address: String,
    #[serde(rename = "publicFQDN")]
    pub public_fqdn: String,
    #[serde(rename = "frontendPort")]
    pub frontend_port: i32,
    #[serde(rename = "backendPort")]
    pub backend_port: i32,
}
impl InboundEndpoint {
    pub fn new(
        name: String,
        protocol: InboundEndpointProtocol,
        public_ip_address: String,
        public_fqdn: String,
        frontend_port: i32,
        backend_port: i32,
    ) -> Self {
        Self {
            name,
            protocol,
            public_ip_address,
            public_fqdn,
            frontend_port,
            backend_port,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InboundEndpointProtocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InboundNatPool {
    #[doc = "The name must be unique within a Batch Pool, can contain letters, numbers, underscores, periods, and hyphens. Names must start with a letter or number, must end with a letter, number, or underscore, and cannot exceed 77 characters.  If any invalid values are provided the request fails with HTTP status code 400."]
    pub name: String,
    #[serde(deserialize_with = "case_insensitive_deserialize")]
    pub protocol: InboundEndpointProtocol,
    #[doc = "This must be unique within a Batch Pool. Acceptable values are between 1 and 65535 except for 22, 3389, 29876 and 29877 as these are reserved. If any reserved values are provided the request fails with HTTP status code 400."]
    #[serde(rename = "backendPort")]
    pub backend_port: i32,
    #[doc = "Acceptable values range between 1 and 65534 except ports from 50000 to 55000 which are reserved. All ranges within a Pool must be distinct and cannot overlap. Each range must contain at least 40 ports. If any reserved or overlapping values are provided the request fails with HTTP status code 400."]
    #[serde(rename = "frontendPortRangeStart")]
    pub frontend_port_range_start: i32,
    #[doc = "Acceptable values range between 1 and 65534 except ports from 50000 to 55000 which are reserved by the Batch service. All ranges within a Pool must be distinct and cannot overlap. Each range must contain at least 40 ports. If any reserved or overlapping values are provided the request fails with HTTP status code 400."]
    #[serde(rename = "frontendPortRangeEnd")]
    pub frontend_port_range_end: i32,
    #[doc = "The maximum number of rules that can be specified across all the endpoints on a Batch Pool is 25. If no network security group rules are specified, a default rule will be created to allow inbound access to the specified backendPort. If the maximum number of network security group rules is exceeded the request fails with HTTP status code 400."]
    #[serde(rename = "networkSecurityGroupRules", default, skip_serializing_if = "Vec::is_empty")]
    pub network_security_group_rules: Vec<NetworkSecurityGroupRule>,
}
impl InboundNatPool {
    pub fn new(
        name: String,
        protocol: InboundEndpointProtocol,
        backend_port: i32,
        frontend_port_range_start: i32,
        frontend_port_range_end: i32,
    ) -> Self {
        Self {
            name,
            protocol,
            backend_port,
            frontend_port_range_start,
            frontend_port_range_end,
            network_security_group_rules: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobAddParameter {
    #[doc = "The ID can contain any combination of alphanumeric characters including hyphens and underscores, and cannot contain more than 64 characters. The ID is case-preserving and case-insensitive (that is, you may not have two IDs within an Account that differ only by case)."]
    pub id: String,
    #[doc = "The display name need not be unique and can contain any Unicode characters up to a maximum length of 1024."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Priority values can range from -1000 to 1000, with -1000 being the lowest priority and 1000 being the highest priority. The default value is 0."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<JobConstraints>,
    #[doc = "The Job Manager Task is automatically started when the Job is created. The Batch service tries to schedule the Job Manager Task before any other Tasks in the Job. When shrinking a Pool, the Batch service tries to preserve Nodes where Job Manager Tasks are running for as long as possible (that is, Compute Nodes running 'normal' Tasks are removed before Compute Nodes running Job Manager Tasks). When a Job Manager Task fails and needs to be restarted, the system tries to schedule it at the highest priority. If there are no idle Compute Nodes available, the system may terminate one of the running Tasks in the Pool and return it to the queue in order to make room for the Job Manager Task to restart. Note that a Job Manager Task in one Job does not have priority over Tasks in other Jobs. Across Jobs, only Job level priorities are observed. For example, if a Job Manager in a priority 0 Job needs to be restarted, it will not displace Tasks of a priority 1 Job. Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing."]
    #[serde(rename = "jobManagerTask", default, skip_serializing_if = "Option::is_none")]
    pub job_manager_task: Option<JobManagerTask>,
    #[doc = "You can use Job Preparation to prepare a Node to run Tasks for the Job. Activities commonly performed in Job Preparation include: Downloading common resource files used by all the Tasks in the Job. The Job Preparation Task can download these common resource files to the shared location on the Node. (AZ_BATCH_NODE_ROOT_DIR\\shared), or starting a local service on the Node so that all Tasks of that Job can communicate with it. If the Job Preparation Task fails (that is, exhausts its retry count before exiting with exit code 0), Batch will not run Tasks of this Job on the Node. The Compute Node remains ineligible to run Tasks of this Job until it is reimaged. The Compute Node remains active and can be used for other Jobs. The Job Preparation Task can run multiple times on the same Node. Therefore, you should write the Job Preparation Task to handle re-execution. If the Node is rebooted, the Job Preparation Task is run again on the Compute Node before scheduling any other Task of the Job, if rerunOnNodeRebootAfterSuccess is true or if the Job Preparation Task did not previously complete. If the Node is reimaged, the Job Preparation Task is run again before scheduling any Task of the Job. Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing."]
    #[serde(rename = "jobPreparationTask", default, skip_serializing_if = "Option::is_none")]
    pub job_preparation_task: Option<JobPreparationTask>,
    #[doc = "The Job Release Task runs when the Job ends, because of one of the following: The user calls the Terminate Job API, or the Delete Job API while the Job is still active, the Job's maximum wall clock time constraint is reached, and the Job is still active, or the Job's Job Manager Task completed, and the Job is configured to terminate when the Job Manager completes. The Job Release Task runs on each Node where Tasks of the Job have run and the Job Preparation Task ran and completed. If you reimage a Node after it has run the Job Preparation Task, and the Job ends without any further Tasks of the Job running on that Node (and hence the Job Preparation Task does not re-run), then the Job Release Task does not run on that Compute Node. If a Node reboots while the Job Release Task is still running, the Job Release Task runs again when the Compute Node starts up. The Job is not marked as complete until all Job Release Tasks have completed. The Job Release Task runs in the background. It does not occupy a scheduling slot; that is, it does not count towards the maxTasksPerNode limit specified on the Pool."]
    #[serde(rename = "jobReleaseTask", default, skip_serializing_if = "Option::is_none")]
    pub job_release_task: Option<JobReleaseTask>,
    #[doc = "Individual Tasks can override an environment setting specified here by specifying the same setting name with a different value."]
    #[serde(rename = "commonEnvironmentSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub common_environment_settings: Vec<EnvironmentSetting>,
    #[serde(rename = "poolInfo")]
    pub pool_info: PoolInformation,
    #[serde(
        rename = "onAllTasksComplete",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub on_all_tasks_complete: Option<OnAllTasksComplete>,
    #[doc = "A Task is considered to have failed if has a failureInfo. A failureInfo is set if the Task completes with a non-zero exit code after exhausting its retry count, or if there was an error starting the Task, for example due to a resource file download error. The default is noaction."]
    #[serde(
        rename = "onTaskFailure",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub on_task_failure: Option<OnTaskFailure>,
    #[doc = "The Batch service does not assign any meaning to metadata; it is solely for the use of user code."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<MetadataItem>,
    #[serde(rename = "usesTaskDependencies", default, skip_serializing_if = "Option::is_none")]
    pub uses_task_dependencies: Option<bool>,
    #[serde(rename = "networkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<JobNetworkConfiguration>,
}
impl JobAddParameter {
    pub fn new(id: String, pool_info: PoolInformation) -> Self {
        Self {
            id,
            display_name: None,
            priority: None,
            constraints: None,
            job_manager_task: None,
            job_preparation_task: None,
            job_release_task: None,
            common_environment_settings: Vec::new(),
            pool_info,
            on_all_tasks_complete: None,
            on_task_failure: None,
            metadata: Vec::new(),
            uses_task_dependencies: None,
            network_configuration: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobConstraints {
    #[doc = "If the Job does not complete within the time limit, the Batch service terminates it and any Tasks that are still running. In this case, the termination reason will be MaxWallClockTimeExpiry. If this property is not specified, there is no time limit on how long the Job may run."]
    #[serde(rename = "maxWallClockTime", default, skip_serializing_if = "Option::is_none")]
    pub max_wall_clock_time: Option<String>,
    #[doc = "Note that this value specifically controls the number of retries. The Batch service will try each Task once, and may then retry up to this limit. For example, if the maximum retry count is 3, Batch tries a Task up to 4 times (one initial try and 3 retries). If the maximum retry count is 0, the Batch service does not retry Tasks. If the maximum retry count is -1, the Batch service retries Tasks without limit. The default value is 0 (no retries)."]
    #[serde(rename = "maxTaskRetryCount", default, skip_serializing_if = "Option::is_none")]
    pub max_task_retry_count: Option<i32>,
}
impl JobConstraints {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDisableParameter {
    #[serde(rename = "disableTasks", deserialize_with = "case_insensitive_deserialize")]
    pub disable_tasks: job_disable_parameter::DisableTasks,
}
impl JobDisableParameter {
    pub fn new(disable_tasks: job_disable_parameter::DisableTasks) -> Self {
        Self { disable_tasks }
    }
}
pub mod job_disable_parameter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DisableTasks {
        #[serde(rename = "requeue")]
        Requeue,
        #[serde(rename = "terminate")]
        Terminate,
        #[serde(rename = "wait")]
        Wait,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobExecutionInformation {
    #[doc = "This is the time at which the Job was created."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "This property is set only if the Job is in the completed state."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "This element contains the actual Pool where the Job is assigned. When you get Job details from the service, they also contain a poolInfo element, which contains the Pool configuration data from when the Job was added or updated. That poolInfo element may also contain a poolId element. If it does, the two IDs are the same. If it does not, it means the Job ran on an auto Pool, and this property contains the ID of that auto Pool."]
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    #[serde(rename = "schedulingError", default, skip_serializing_if = "Option::is_none")]
    pub scheduling_error: Option<JobSchedulingError>,
    #[doc = "This property is set only if the Job is in the completed state. If the Batch service terminates the Job, it sets the reason as follows: JMComplete - the Job Manager Task completed, and killJobOnCompletion was set to true. MaxWallClockTimeExpiry - the Job reached its maxWallClockTime constraint. TerminateJobSchedule - the Job ran as part of a schedule, and the schedule terminated. AllTasksComplete - the Job's onAllTasksComplete attribute is set to terminatejob, and all Tasks in the Job are complete. TaskFailed - the Job's onTaskFailure attribute is set to performExitOptionsJobAction, and a Task in the Job failed with an exit condition that specified a jobAction of terminatejob. Any other string is a user-defined reason specified in a call to the 'Terminate a Job' operation."]
    #[serde(rename = "terminateReason", default, skip_serializing_if = "Option::is_none")]
    pub terminate_reason: Option<String>,
}
impl JobExecutionInformation {
    pub fn new(start_time: time::OffsetDateTime) -> Self {
        Self {
            start_time,
            end_time: None,
            pool_id: None,
            scheduling_error: None,
            terminate_reason: None,
        }
    }
}
#[doc = "The Job Manager Task is automatically started when the Job is created. The Batch service tries to schedule the Job Manager Task before any other Tasks in the Job. When shrinking a Pool, the Batch service tries to preserve Nodes where Job Manager Tasks are running for as long as possible (that is, Compute Nodes running 'normal' Tasks are removed before Compute Nodes running Job Manager Tasks). When a Job Manager Task fails and needs to be restarted, the system tries to schedule it at the highest priority. If there are no idle Compute Nodes available, the system may terminate one of the running Tasks in the Pool and return it to the queue in order to make room for the Job Manager Task to restart. Note that a Job Manager Task in one Job does not have priority over Tasks in other Jobs. Across Jobs, only Job level priorities are observed. For example, if a Job Manager in a priority 0 Job needs to be restarted, it will not displace Tasks of a priority 1 Job. Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobManagerTask {
    #[doc = "The ID can contain any combination of alphanumeric characters including hyphens and underscores and cannot contain more than 64 characters."]
    pub id: String,
    #[doc = "It need not be unique and can contain any Unicode characters up to a maximum length of 1024."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The command line does not run under a shell, and therefore cannot take advantage of shell features such as environment variable expansion. If you want to take advantage of such features, you should invoke the shell in the command line, for example using \"cmd /c MyCommand\" in Windows or \"/bin/sh -c MyCommand\" in Linux. If the command line refers to file paths, it should use a relative path (relative to the Task working directory), or use the Batch provided environment variable (https://docs.microsoft.com/en-us/azure/batch/batch-compute-node-environment-variables)."]
    #[serde(rename = "commandLine")]
    pub command_line: String,
    #[serde(rename = "containerSettings", default, skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<TaskContainerSettings>,
    #[doc = "Files listed under this element are located in the Task's working directory. There is a maximum size for the list of resource files.  When the max size is exceeded, the request will fail and the response error code will be RequestEntityTooLarge. If this occurs, the collection of ResourceFiles must be reduced in size. This can be achieved using .zip files, Application Packages, or Docker Containers."]
    #[serde(rename = "resourceFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_files: Vec<ResourceFile>,
    #[doc = "For multi-instance Tasks, the files will only be uploaded from the Compute Node on which the primary Task is executed."]
    #[serde(rename = "outputFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub output_files: Vec<OutputFile>,
    #[serde(rename = "environmentSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_settings: Vec<EnvironmentSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<TaskConstraints>,
    #[doc = "If true, when the Job Manager Task completes, the Batch service marks the Job as complete. If any Tasks are still running at this time (other than Job Release), those Tasks are terminated. If false, the completion of the Job Manager Task does not affect the Job status. In this case, you should either use the onAllTasksComplete attribute to terminate the Job, or have a client or user terminate the Job explicitly. An example of this is if the Job Manager creates a set of Tasks but then takes no further role in their execution. The default value is true. If you are using the onAllTasksComplete and onTaskFailure attributes to control Job lifetime, and using the Job Manager Task only to create the Tasks for the Job (not to monitor progress), then it is important to set killJobOnCompletion to false."]
    #[serde(rename = "killJobOnCompletion", default, skip_serializing_if = "Option::is_none")]
    pub kill_job_on_completion: Option<bool>,
    #[doc = "Specify either the userName or autoUser property, but not both."]
    #[serde(rename = "userIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<UserIdentity>,
    #[doc = "If true, no other Tasks will run on the same Node for as long as the Job Manager is running. If false, other Tasks can run simultaneously with the Job Manager on a Compute Node. The Job Manager Task counts normally against the Compute Node's concurrent Task limit, so this is only relevant if the Compute Node allows multiple concurrent Tasks. The default value is true."]
    #[serde(rename = "runExclusive", default, skip_serializing_if = "Option::is_none")]
    pub run_exclusive: Option<bool>,
    #[doc = "Application Packages are downloaded and deployed to a shared directory, not the Task working directory. Therefore, if a referenced Application Package is already on the Compute Node, and is up to date, then it is not re-downloaded; the existing copy on the Compute Node is used. If a referenced Application Package cannot be installed, for example because the package has been deleted or because download failed, the Task fails."]
    #[serde(rename = "applicationPackageReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub application_package_references: Vec<ApplicationPackageReference>,
    #[serde(rename = "authenticationTokenSettings", default, skip_serializing_if = "Option::is_none")]
    pub authentication_token_settings: Option<AuthenticationTokenSettings>,
    #[doc = "The default value is true."]
    #[serde(rename = "allowLowPriorityNode", default, skip_serializing_if = "Option::is_none")]
    pub allow_low_priority_node: Option<bool>,
}
impl JobManagerTask {
    pub fn new(id: String, command_line: String) -> Self {
        Self {
            id,
            display_name: None,
            command_line,
            container_settings: None,
            resource_files: Vec::new(),
            output_files: Vec::new(),
            environment_settings: Vec::new(),
            constraints: None,
            kill_job_on_completion: None,
            user_identity: None,
            run_exclusive: None,
            application_package_references: Vec::new(),
            authentication_token_settings: None,
            allow_low_priority_node: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobNetworkConfiguration {
    #[doc = "The virtual network must be in the same region and subscription as the Azure Batch Account. The specified subnet should have enough free IP addresses to accommodate the number of Compute Nodes which will run Tasks from the Job. This can be up to the number of Compute Nodes in the Pool. The 'MicrosoftAzureBatch' service principal must have the 'Classic Virtual Machine Contributor' Role-Based Access Control (RBAC) role for the specified VNet so that Azure Batch service can schedule Tasks on the Nodes. This can be verified by checking if the specified VNet has any associated Network Security Groups (NSG). If communication to the Nodes in the specified subnet is denied by an NSG, then the Batch service will set the state of the Compute Nodes to unusable. This is of the form /subscriptions/{subscription}/resourceGroups/{group}/providers/{provider}/virtualNetworks/{network}/subnets/{subnet}. If the specified VNet has any associated Network Security Groups (NSG), then a few reserved system ports must be enabled for inbound communication from the Azure Batch service. For Pools created with a Virtual Machine configuration, enable ports 29876 and 29877, as well as port 22 for Linux and port 3389 for Windows. Port 443 is also required to be open for outbound connections for communications to Azure Storage. For more details see: https://docs.microsoft.com/en-us/azure/batch/batch-api-basics#virtual-network-vnet-and-firewall-configuration"]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
}
impl JobNetworkConfiguration {
    pub fn new(subnet_id: String) -> Self {
        Self { subnet_id }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobPatchParameter {
    #[doc = "Priority values can range from -1000 to 1000, with -1000 being the lowest priority and 1000 being the highest priority. If omitted, the priority of the Job is left unchanged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(
        rename = "onAllTasksComplete",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub on_all_tasks_complete: Option<OnAllTasksComplete>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<JobConstraints>,
    #[serde(rename = "poolInfo", default, skip_serializing_if = "Option::is_none")]
    pub pool_info: Option<PoolInformation>,
    #[doc = "If omitted, the existing Job metadata is left unchanged."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<MetadataItem>,
}
impl JobPatchParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobPreparationAndReleaseTaskExecutionInformation {
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    #[serde(rename = "nodeId", default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "nodeUrl", default, skip_serializing_if = "Option::is_none")]
    pub node_url: Option<String>,
    #[serde(rename = "jobPreparationTaskExecutionInfo", default, skip_serializing_if = "Option::is_none")]
    pub job_preparation_task_execution_info: Option<JobPreparationTaskExecutionInformation>,
    #[serde(rename = "jobReleaseTaskExecutionInfo", default, skip_serializing_if = "Option::is_none")]
    pub job_release_task_execution_info: Option<JobReleaseTaskExecutionInformation>,
}
impl JobPreparationAndReleaseTaskExecutionInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "You can use Job Preparation to prepare a Node to run Tasks for the Job. Activities commonly performed in Job Preparation include: Downloading common resource files used by all the Tasks in the Job. The Job Preparation Task can download these common resource files to the shared location on the Node. (AZ_BATCH_NODE_ROOT_DIR\\shared), or starting a local service on the Node so that all Tasks of that Job can communicate with it. If the Job Preparation Task fails (that is, exhausts its retry count before exiting with exit code 0), Batch will not run Tasks of this Job on the Node. The Compute Node remains ineligible to run Tasks of this Job until it is reimaged. The Compute Node remains active and can be used for other Jobs. The Job Preparation Task can run multiple times on the same Node. Therefore, you should write the Job Preparation Task to handle re-execution. If the Node is rebooted, the Job Preparation Task is run again on the Compute Node before scheduling any other Task of the Job, if rerunOnNodeRebootAfterSuccess is true or if the Job Preparation Task did not previously complete. If the Node is reimaged, the Job Preparation Task is run again before scheduling any Task of the Job. Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobPreparationTask {
    #[doc = "The ID can contain any combination of alphanumeric characters including hyphens and underscores and cannot contain more than 64 characters. If you do not specify this property, the Batch service assigns a default value of 'jobpreparation'. No other Task in the Job can have the same ID as the Job Preparation Task. If you try to submit a Task with the same id, the Batch service rejects the request with error code TaskIdSameAsJobPreparationTask; if you are calling the REST API directly, the HTTP status code is 409 (Conflict)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The command line does not run under a shell, and therefore cannot take advantage of shell features such as environment variable expansion. If you want to take advantage of such features, you should invoke the shell in the command line, for example using \"cmd /c MyCommand\" in Windows or \"/bin/sh -c MyCommand\" in Linux. If the command line refers to file paths, it should use a relative path (relative to the Task working directory), or use the Batch provided environment variable (https://docs.microsoft.com/en-us/azure/batch/batch-compute-node-environment-variables)."]
    #[serde(rename = "commandLine")]
    pub command_line: String,
    #[serde(rename = "containerSettings", default, skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<TaskContainerSettings>,
    #[doc = "Files listed under this element are located in the Task's working directory.  There is a maximum size for the list of resource files.  When the max size is exceeded, the request will fail and the response error code will be RequestEntityTooLarge. If this occurs, the collection of ResourceFiles must be reduced in size. This can be achieved using .zip files, Application Packages, or Docker Containers."]
    #[serde(rename = "resourceFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_files: Vec<ResourceFile>,
    #[serde(rename = "environmentSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_settings: Vec<EnvironmentSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<TaskConstraints>,
    #[doc = "If true and the Job Preparation Task fails on a Node, the Batch service retries the Job Preparation Task up to its maximum retry count (as specified in the constraints element). If the Task has still not completed successfully after all retries, then the Batch service will not schedule Tasks of the Job to the Node. The Node remains active and eligible to run Tasks of other Jobs. If false, the Batch service will not wait for the Job Preparation Task to complete. In this case, other Tasks of the Job can start executing on the Compute Node while the Job Preparation Task is still running; and even if the Job Preparation Task fails, new Tasks will continue to be scheduled on the Compute Node. The default value is true."]
    #[serde(rename = "waitForSuccess", default, skip_serializing_if = "Option::is_none")]
    pub wait_for_success: Option<bool>,
    #[doc = "Specify either the userName or autoUser property, but not both."]
    #[serde(rename = "userIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<UserIdentity>,
    #[doc = "The Job Preparation Task is always rerun if a Compute Node is reimaged, or if the Job Preparation Task did not complete (e.g. because the reboot occurred while the Task was running). Therefore, you should always write a Job Preparation Task to be idempotent and to behave correctly if run multiple times. The default value is true."]
    #[serde(rename = "rerunOnNodeRebootAfterSuccess", default, skip_serializing_if = "Option::is_none")]
    pub rerun_on_node_reboot_after_success: Option<bool>,
}
impl JobPreparationTask {
    pub fn new(command_line: String) -> Self {
        Self {
            id: None,
            command_line,
            container_settings: None,
            resource_files: Vec::new(),
            environment_settings: Vec::new(),
            constraints: None,
            wait_for_success: None,
            user_identity: None,
            rerun_on_node_reboot_after_success: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobPreparationTaskExecutionInformation {
    #[doc = "If the Task has been restarted or retried, this is the most recent time at which the Task started running."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "This property is set only if the Task is in the Completed state."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[serde(deserialize_with = "case_insensitive_deserialize")]
    pub state: job_preparation_task_execution_information::State,
    #[serde(rename = "taskRootDirectory", default, skip_serializing_if = "Option::is_none")]
    pub task_root_directory: Option<String>,
    #[serde(rename = "taskRootDirectoryUrl", default, skip_serializing_if = "Option::is_none")]
    pub task_root_directory_url: Option<String>,
    #[doc = "This parameter is returned only if the Task is in the completed state. The exit code for a process reflects the specific convention implemented by the application developer for that process. If you use the exit code value to make decisions in your code, be sure that you know the exit code convention used by the application process. Note that the exit code may also be generated by the Compute Node operating system, such as when a process is forcibly terminated."]
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "containerInfo", default, skip_serializing_if = "Option::is_none")]
    pub container_info: Option<TaskContainerExecutionInformation>,
    #[serde(rename = "failureInfo", default, skip_serializing_if = "Option::is_none")]
    pub failure_info: Option<TaskFailureInformation>,
    #[doc = "Task application failures (non-zero exit code) are retried, pre-processing errors (the Task could not be run) and file upload errors are not retried. The Batch service will retry the Task up to the limit specified by the constraints."]
    #[serde(rename = "retryCount")]
    pub retry_count: i32,
    #[doc = "This property is set only if the Task was retried (i.e. retryCount is nonzero). If present, this is typically the same as startTime, but may be different if the Task has been restarted for reasons other than retry; for example, if the Compute Node was rebooted during a retry, then the startTime is updated but the lastRetryTime is not."]
    #[serde(rename = "lastRetryTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_retry_time: Option<time::OffsetDateTime>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub result: Option<TaskExecutionResult>,
}
impl JobPreparationTaskExecutionInformation {
    pub fn new(start_time: time::OffsetDateTime, state: job_preparation_task_execution_information::State, retry_count: i32) -> Self {
        Self {
            start_time,
            end_time: None,
            state,
            task_root_directory: None,
            task_root_directory_url: None,
            exit_code: None,
            container_info: None,
            failure_info: None,
            retry_count,
            last_retry_time: None,
            result: None,
        }
    }
}
pub mod job_preparation_task_execution_information {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "completed")]
        Completed,
    }
}
#[doc = "The Job Release Task runs when the Job ends, because of one of the following: The user calls the Terminate Job API, or the Delete Job API while the Job is still active, the Job's maximum wall clock time constraint is reached, and the Job is still active, or the Job's Job Manager Task completed, and the Job is configured to terminate when the Job Manager completes. The Job Release Task runs on each Node where Tasks of the Job have run and the Job Preparation Task ran and completed. If you reimage a Node after it has run the Job Preparation Task, and the Job ends without any further Tasks of the Job running on that Node (and hence the Job Preparation Task does not re-run), then the Job Release Task does not run on that Compute Node. If a Node reboots while the Job Release Task is still running, the Job Release Task runs again when the Compute Node starts up. The Job is not marked as complete until all Job Release Tasks have completed. The Job Release Task runs in the background. It does not occupy a scheduling slot; that is, it does not count towards the maxTasksPerNode limit specified on the Pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobReleaseTask {
    #[doc = "The ID can contain any combination of alphanumeric characters including hyphens and underscores and cannot contain more than 64 characters. If you do not specify this property, the Batch service assigns a default value of 'jobrelease'. No other Task in the Job can have the same ID as the Job Release Task. If you try to submit a Task with the same id, the Batch service rejects the request with error code TaskIdSameAsJobReleaseTask; if you are calling the REST API directly, the HTTP status code is 409 (Conflict)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The command line does not run under a shell, and therefore cannot take advantage of shell features such as environment variable expansion. If you want to take advantage of such features, you should invoke the shell in the command line, for example using \"cmd /c MyCommand\" in Windows or \"/bin/sh -c MyCommand\" in Linux. If the command line refers to file paths, it should use a relative path (relative to the Task working directory), or use the Batch provided environment variable (https://docs.microsoft.com/en-us/azure/batch/batch-compute-node-environment-variables)."]
    #[serde(rename = "commandLine")]
    pub command_line: String,
    #[serde(rename = "containerSettings", default, skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<TaskContainerSettings>,
    #[doc = "Files listed under this element are located in the Task's working directory."]
    #[serde(rename = "resourceFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_files: Vec<ResourceFile>,
    #[serde(rename = "environmentSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_settings: Vec<EnvironmentSetting>,
    #[serde(rename = "maxWallClockTime", default, skip_serializing_if = "Option::is_none")]
    pub max_wall_clock_time: Option<String>,
    #[doc = "The default is 7 days, i.e. the Task directory will be retained for 7 days unless the Compute Node is removed or the Job is deleted."]
    #[serde(rename = "retentionTime", default, skip_serializing_if = "Option::is_none")]
    pub retention_time: Option<String>,
    #[doc = "Specify either the userName or autoUser property, but not both."]
    #[serde(rename = "userIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<UserIdentity>,
}
impl JobReleaseTask {
    pub fn new(command_line: String) -> Self {
        Self {
            id: None,
            command_line,
            container_settings: None,
            resource_files: Vec::new(),
            environment_settings: Vec::new(),
            max_wall_clock_time: None,
            retention_time: None,
            user_identity: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobReleaseTaskExecutionInformation {
    #[doc = "If the Task has been restarted or retried, this is the most recent time at which the Task started running."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "This property is set only if the Task is in the Completed state."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[serde(deserialize_with = "case_insensitive_deserialize")]
    pub state: job_release_task_execution_information::State,
    #[serde(rename = "taskRootDirectory", default, skip_serializing_if = "Option::is_none")]
    pub task_root_directory: Option<String>,
    #[serde(rename = "taskRootDirectoryUrl", default, skip_serializing_if = "Option::is_none")]
    pub task_root_directory_url: Option<String>,
    #[doc = "This parameter is returned only if the Task is in the completed state. The exit code for a process reflects the specific convention implemented by the application developer for that process. If you use the exit code value to make decisions in your code, be sure that you know the exit code convention used by the application process. Note that the exit code may also be generated by the Compute Node operating system, such as when a process is forcibly terminated."]
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "containerInfo", default, skip_serializing_if = "Option::is_none")]
    pub container_info: Option<TaskContainerExecutionInformation>,
    #[serde(rename = "failureInfo", default, skip_serializing_if = "Option::is_none")]
    pub failure_info: Option<TaskFailureInformation>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub result: Option<TaskExecutionResult>,
}
impl JobReleaseTaskExecutionInformation {
    pub fn new(start_time: time::OffsetDateTime, state: job_release_task_execution_information::State) -> Self {
        Self {
            start_time,
            end_time: None,
            state,
            task_root_directory: None,
            task_root_directory_url: None,
            exit_code: None,
            container_info: None,
            failure_info: None,
            result: None,
        }
    }
}
pub mod job_release_task_execution_information {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "completed")]
        Completed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobScheduleAddParameter {
    #[doc = "The ID can contain any combination of alphanumeric characters including hyphens and underscores, and cannot contain more than 64 characters. The ID is case-preserving and case-insensitive (that is, you may not have two IDs within an Account that differ only by case)."]
    pub id: String,
    #[doc = "The display name need not be unique and can contain any Unicode characters up to a maximum length of 1024."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub schedule: Schedule,
    #[serde(rename = "jobSpecification")]
    pub job_specification: JobSpecification,
    #[doc = "The Batch service does not assign any meaning to metadata; it is solely for the use of user code."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<MetadataItem>,
}
impl JobScheduleAddParameter {
    pub fn new(id: String, schedule: Schedule, job_specification: JobSpecification) -> Self {
        Self {
            id,
            display_name: None,
            schedule,
            job_specification,
            metadata: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobScheduleExecutionInformation {
    #[doc = "This property is meaningful only if the schedule is in the active state when the time comes around. For example, if the schedule is disabled, no Job will be created at nextRunTime unless the Job is enabled before then."]
    #[serde(rename = "nextRunTime", default, with = "azure_core::date::rfc3339::option")]
    pub next_run_time: Option<time::OffsetDateTime>,
    #[serde(rename = "recentJob", default, skip_serializing_if = "Option::is_none")]
    pub recent_job: Option<RecentJob>,
    #[doc = "This property is set only if the Job Schedule is in the completed state."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}
impl JobScheduleExecutionInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobSchedulePatchParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    #[serde(rename = "jobSpecification", default, skip_serializing_if = "Option::is_none")]
    pub job_specification: Option<JobSpecification>,
    #[doc = "If you do not specify this element, existing metadata is left unchanged."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<MetadataItem>,
}
impl JobSchedulePatchParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JobScheduleState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "terminating")]
    Terminating,
    #[serde(rename = "deleting")]
    Deleting,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobScheduleStatistics {
    pub url: String,
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[serde(rename = "lastUpdateTime", with = "azure_core::date::rfc3339")]
    pub last_update_time: time::OffsetDateTime,
    #[serde(rename = "userCPUTime")]
    pub user_cpu_time: String,
    #[serde(rename = "kernelCPUTime")]
    pub kernel_cpu_time: String,
    #[doc = "The wall clock time is the elapsed time from when the Task started running on a Compute Node to when it finished (or to the last time the statistics were updated, if the Task had not finished by then). If a Task was retried, this includes the wall clock time of all the Task retries."]
    #[serde(rename = "wallClockTime")]
    pub wall_clock_time: String,
    #[serde(rename = "readIOps")]
    pub read_i_ops: i64,
    #[serde(rename = "writeIOps")]
    pub write_i_ops: i64,
    #[serde(rename = "readIOGiB")]
    pub read_io_gi_b: f64,
    #[serde(rename = "writeIOGiB")]
    pub write_io_gi_b: f64,
    #[serde(rename = "numSucceededTasks")]
    pub num_succeeded_tasks: i64,
    #[serde(rename = "numFailedTasks")]
    pub num_failed_tasks: i64,
    #[serde(rename = "numTaskRetries")]
    pub num_task_retries: i64,
    #[doc = "This value is only reported in the Account lifetime statistics; it is not included in the Job statistics."]
    #[serde(rename = "waitTime")]
    pub wait_time: String,
}
impl JobScheduleStatistics {
    pub fn new(
        url: String,
        start_time: time::OffsetDateTime,
        last_update_time: time::OffsetDateTime,
        user_cpu_time: String,
        kernel_cpu_time: String,
        wall_clock_time: String,
        read_i_ops: i64,
        write_i_ops: i64,
        read_io_gi_b: f64,
        write_io_gi_b: f64,
        num_succeeded_tasks: i64,
        num_failed_tasks: i64,
        num_task_retries: i64,
        wait_time: String,
    ) -> Self {
        Self {
            url,
            start_time,
            last_update_time,
            user_cpu_time,
            kernel_cpu_time,
            wall_clock_time,
            read_i_ops,
            write_i_ops,
            read_io_gi_b,
            write_io_gi_b,
            num_succeeded_tasks,
            num_failed_tasks,
            num_task_retries,
            wait_time,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobScheduleUpdateParameter {
    pub schedule: Schedule,
    #[serde(rename = "jobSpecification")]
    pub job_specification: JobSpecification,
    #[doc = "If you do not specify this element, it takes the default value of an empty list; in effect, any existing metadata is deleted."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<MetadataItem>,
}
impl JobScheduleUpdateParameter {
    pub fn new(schedule: Schedule, job_specification: JobSpecification) -> Self {
        Self {
            schedule,
            job_specification,
            metadata: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobSchedulingError {
    #[serde(deserialize_with = "case_insensitive_deserialize")]
    pub category: ErrorCategory,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<NameValuePair>,
}
impl JobSchedulingError {
    pub fn new(category: ErrorCategory) -> Self {
        Self {
            category,
            code: None,
            message: None,
            details: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobSpecification {
    #[doc = "Priority values can range from -1000 to 1000, with -1000 being the lowest priority and 1000 being the highest priority. The default value is 0. This priority is used as the default for all Jobs under the Job Schedule. You can update a Job's priority after it has been created using by using the update Job API."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "The name need not be unique and can contain any Unicode characters up to a maximum length of 1024."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "usesTaskDependencies", default, skip_serializing_if = "Option::is_none")]
    pub uses_task_dependencies: Option<bool>,
    #[serde(
        rename = "onAllTasksComplete",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub on_all_tasks_complete: Option<OnAllTasksComplete>,
    #[doc = "A Task is considered to have failed if has a failureInfo. A failureInfo is set if the Task completes with a non-zero exit code after exhausting its retry count, or if there was an error starting the Task, for example due to a resource file download error. The default is noaction."]
    #[serde(
        rename = "onTaskFailure",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub on_task_failure: Option<OnTaskFailure>,
    #[serde(rename = "networkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<JobNetworkConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<JobConstraints>,
    #[doc = "The Job Manager Task is automatically started when the Job is created. The Batch service tries to schedule the Job Manager Task before any other Tasks in the Job. When shrinking a Pool, the Batch service tries to preserve Nodes where Job Manager Tasks are running for as long as possible (that is, Compute Nodes running 'normal' Tasks are removed before Compute Nodes running Job Manager Tasks). When a Job Manager Task fails and needs to be restarted, the system tries to schedule it at the highest priority. If there are no idle Compute Nodes available, the system may terminate one of the running Tasks in the Pool and return it to the queue in order to make room for the Job Manager Task to restart. Note that a Job Manager Task in one Job does not have priority over Tasks in other Jobs. Across Jobs, only Job level priorities are observed. For example, if a Job Manager in a priority 0 Job needs to be restarted, it will not displace Tasks of a priority 1 Job. Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing."]
    #[serde(rename = "jobManagerTask", default, skip_serializing_if = "Option::is_none")]
    pub job_manager_task: Option<JobManagerTask>,
    #[doc = "You can use Job Preparation to prepare a Node to run Tasks for the Job. Activities commonly performed in Job Preparation include: Downloading common resource files used by all the Tasks in the Job. The Job Preparation Task can download these common resource files to the shared location on the Node. (AZ_BATCH_NODE_ROOT_DIR\\shared), or starting a local service on the Node so that all Tasks of that Job can communicate with it. If the Job Preparation Task fails (that is, exhausts its retry count before exiting with exit code 0), Batch will not run Tasks of this Job on the Node. The Compute Node remains ineligible to run Tasks of this Job until it is reimaged. The Compute Node remains active and can be used for other Jobs. The Job Preparation Task can run multiple times on the same Node. Therefore, you should write the Job Preparation Task to handle re-execution. If the Node is rebooted, the Job Preparation Task is run again on the Compute Node before scheduling any other Task of the Job, if rerunOnNodeRebootAfterSuccess is true or if the Job Preparation Task did not previously complete. If the Node is reimaged, the Job Preparation Task is run again before scheduling any Task of the Job. Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing."]
    #[serde(rename = "jobPreparationTask", default, skip_serializing_if = "Option::is_none")]
    pub job_preparation_task: Option<JobPreparationTask>,
    #[doc = "The Job Release Task runs when the Job ends, because of one of the following: The user calls the Terminate Job API, or the Delete Job API while the Job is still active, the Job's maximum wall clock time constraint is reached, and the Job is still active, or the Job's Job Manager Task completed, and the Job is configured to terminate when the Job Manager completes. The Job Release Task runs on each Node where Tasks of the Job have run and the Job Preparation Task ran and completed. If you reimage a Node after it has run the Job Preparation Task, and the Job ends without any further Tasks of the Job running on that Node (and hence the Job Preparation Task does not re-run), then the Job Release Task does not run on that Compute Node. If a Node reboots while the Job Release Task is still running, the Job Release Task runs again when the Compute Node starts up. The Job is not marked as complete until all Job Release Tasks have completed. The Job Release Task runs in the background. It does not occupy a scheduling slot; that is, it does not count towards the maxTasksPerNode limit specified on the Pool."]
    #[serde(rename = "jobReleaseTask", default, skip_serializing_if = "Option::is_none")]
    pub job_release_task: Option<JobReleaseTask>,
    #[doc = "Individual Tasks can override an environment setting specified here by specifying the same setting name with a different value."]
    #[serde(rename = "commonEnvironmentSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub common_environment_settings: Vec<EnvironmentSetting>,
    #[serde(rename = "poolInfo")]
    pub pool_info: PoolInformation,
    #[doc = "The Batch service does not assign any meaning to metadata; it is solely for the use of user code."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<MetadataItem>,
}
impl JobSpecification {
    pub fn new(pool_info: PoolInformation) -> Self {
        Self {
            priority: None,
            display_name: None,
            uses_task_dependencies: None,
            on_all_tasks_complete: None,
            on_task_failure: None,
            network_configuration: None,
            constraints: None,
            job_manager_task: None,
            job_preparation_task: None,
            job_release_task: None,
            common_environment_settings: Vec::new(),
            pool_info,
            metadata: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JobState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabling")]
    Disabling,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabling")]
    Enabling,
    #[serde(rename = "terminating")]
    Terminating,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "deleting")]
    Deleting,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStatistics {
    pub url: String,
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[serde(rename = "lastUpdateTime", with = "azure_core::date::rfc3339")]
    pub last_update_time: time::OffsetDateTime,
    #[serde(rename = "userCPUTime")]
    pub user_cpu_time: String,
    #[serde(rename = "kernelCPUTime")]
    pub kernel_cpu_time: String,
    #[doc = " The wall clock time is the elapsed time from when the Task started running on a Compute Node to when it finished (or to the last time the statistics were updated, if the Task had not finished by then). If a Task was retried, this includes the wall clock time of all the Task retries."]
    #[serde(rename = "wallClockTime")]
    pub wall_clock_time: String,
    #[serde(rename = "readIOps")]
    pub read_i_ops: i64,
    #[serde(rename = "writeIOps")]
    pub write_i_ops: i64,
    #[serde(rename = "readIOGiB")]
    pub read_io_gi_b: f64,
    #[serde(rename = "writeIOGiB")]
    pub write_io_gi_b: f64,
    #[doc = "A Task completes successfully if it returns exit code 0."]
    #[serde(rename = "numSucceededTasks")]
    pub num_succeeded_tasks: i64,
    #[doc = "A Task fails if it exhausts its maximum retry count without returning exit code 0."]
    #[serde(rename = "numFailedTasks")]
    pub num_failed_tasks: i64,
    #[serde(rename = "numTaskRetries")]
    pub num_task_retries: i64,
    #[doc = "The wait time for a Task is defined as the elapsed time between the creation of the Task and the start of Task execution. (If the Task is retried due to failures, the wait time is the time to the most recent Task execution.) This value is only reported in the Account lifetime statistics; it is not included in the Job statistics."]
    #[serde(rename = "waitTime")]
    pub wait_time: String,
}
impl JobStatistics {
    pub fn new(
        url: String,
        start_time: time::OffsetDateTime,
        last_update_time: time::OffsetDateTime,
        user_cpu_time: String,
        kernel_cpu_time: String,
        wall_clock_time: String,
        read_i_ops: i64,
        write_i_ops: i64,
        read_io_gi_b: f64,
        write_io_gi_b: f64,
        num_succeeded_tasks: i64,
        num_failed_tasks: i64,
        num_task_retries: i64,
        wait_time: String,
    ) -> Self {
        Self {
            url,
            start_time,
            last_update_time,
            user_cpu_time,
            kernel_cpu_time,
            wall_clock_time,
            read_i_ops,
            write_i_ops,
            read_io_gi_b,
            write_io_gi_b,
            num_succeeded_tasks,
            num_failed_tasks,
            num_task_retries,
            wait_time,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobTerminateParameter {
    #[serde(rename = "terminateReason", default, skip_serializing_if = "Option::is_none")]
    pub terminate_reason: Option<String>,
}
impl JobTerminateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobUpdateParameter {
    #[doc = "Priority values can range from -1000 to 1000, with -1000 being the lowest priority and 1000 being the highest priority. If omitted, it is set to the default value 0."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<JobConstraints>,
    #[serde(rename = "poolInfo")]
    pub pool_info: PoolInformation,
    #[doc = "If omitted, it takes the default value of an empty list; in effect, any existing metadata is deleted."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<MetadataItem>,
    #[serde(
        rename = "onAllTasksComplete",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub on_all_tasks_complete: Option<OnAllTasksComplete>,
}
impl JobUpdateParameter {
    pub fn new(pool_info: PoolInformation) -> Self {
        Self {
            priority: None,
            constraints: None,
            pool_info,
            metadata: Vec::new(),
            on_all_tasks_complete: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxUserConfiguration {
    #[doc = "The uid and gid properties must be specified together or not at all. If not specified the underlying operating system picks the uid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<i32>,
    #[doc = "The uid and gid properties must be specified together or not at all. If not specified the underlying operating system picks the gid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gid: Option<i32>,
    #[doc = "The private key must not be password protected. The private key is used to automatically configure asymmetric-key based authentication for SSH between Compute Nodes in a Linux Pool when the Pool's enableInterNodeCommunication property is true (it is ignored if enableInterNodeCommunication is false). It does this by placing the key pair into the user's .ssh directory. If not specified, password-less SSH is not configured between Compute Nodes (no modification of the user's .ssh directory is done)."]
    #[serde(rename = "sshPrivateKey", default, skip_serializing_if = "Option::is_none")]
    pub ssh_private_key: Option<String>,
}
impl LinuxUserConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Batch service does not assign any meaning to this metadata; it is solely for the use of user code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataItem {
    pub name: String,
    pub value: String,
}
impl MetadataItem {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MountConfiguration {
    #[serde(rename = "azureBlobFileSystemConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub azure_blob_file_system_configuration: Option<AzureBlobFileSystemConfiguration>,
    #[serde(rename = "nfsMountConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub nfs_mount_configuration: Option<NfsMountConfiguration>,
    #[serde(rename = "cifsMountConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cifs_mount_configuration: Option<CifsMountConfiguration>,
    #[serde(rename = "azureFileShareConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_configuration: Option<AzureFileShareConfiguration>,
}
impl MountConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Multi-instance Tasks are commonly used to support MPI Tasks. In the MPI case, if any of the subtasks fail (for example due to exiting with a non-zero exit code) the entire multi-instance Task fails. The multi-instance Task is then terminated and retried, up to its retry limit."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiInstanceSettings {
    #[doc = "If omitted, the default is 1."]
    #[serde(rename = "numberOfInstances", default, skip_serializing_if = "Option::is_none")]
    pub number_of_instances: Option<i32>,
    #[doc = "A typical coordination command line launches a background service and verifies that the service is ready to process inter-node messages."]
    #[serde(rename = "coordinationCommandLine")]
    pub coordination_command_line: String,
    #[doc = "The difference between common resource files and Task resource files is that common resource files are downloaded for all subtasks including the primary, whereas Task resource files are downloaded only for the primary. Also note that these resource files are not downloaded to the Task working directory, but instead are downloaded to the Task root directory (one directory above the working directory).  There is a maximum size for the list of resource files.  When the max size is exceeded, the request will fail and the response error code will be RequestEntityTooLarge. If this occurs, the collection of ResourceFiles must be reduced in size. This can be achieved using .zip files, Application Packages, or Docker Containers."]
    #[serde(rename = "commonResourceFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub common_resource_files: Vec<ResourceFile>,
}
impl MultiInstanceSettings {
    pub fn new(coordination_command_line: String) -> Self {
        Self {
            number_of_instances: None,
            coordination_command_line,
            common_resource_files: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NfsMountConfiguration {
    pub source: String,
    #[doc = "All file systems are mounted relative to the Batch mounts directory, accessible via the AZ_BATCH_NODE_MOUNTS_DIR environment variable."]
    #[serde(rename = "relativeMountPath")]
    pub relative_mount_path: String,
    #[doc = "These are 'net use' options in Windows and 'mount' options in Linux."]
    #[serde(rename = "mountOptions", default, skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<String>,
}
impl NfsMountConfiguration {
    pub fn new(source: String, relative_mount_path: String) -> Self {
        Self {
            source,
            relative_mount_path,
            mount_options: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameValuePair {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl NameValuePair {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network configuration for a Pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkConfiguration {
    #[doc = "The virtual network must be in the same region and subscription as the Azure Batch Account. The specified subnet should have enough free IP addresses to accommodate the number of Compute Nodes in the Pool. If the subnet doesn't have enough free IP addresses, the Pool will partially allocate Nodes and a resize error will occur. The 'MicrosoftAzureBatch' service principal must have the 'Classic Virtual Machine Contributor' Role-Based Access Control (RBAC) role for the specified VNet. The specified subnet must allow communication from the Azure Batch service to be able to schedule Tasks on the Nodes. This can be verified by checking if the specified VNet has any associated Network Security Groups (NSG). If communication to the Nodes in the specified subnet is denied by an NSG, then the Batch service will set the state of the Compute Nodes to unusable. For Pools created with virtualMachineConfiguration only ARM virtual networks ('Microsoft.Network/virtualNetworks') are supported, but for Pools created with cloudServiceConfiguration both ARM and classic virtual networks are supported. If the specified VNet has any associated Network Security Groups (NSG), then a few reserved system ports must be enabled for inbound communication. For Pools created with a virtual machine configuration, enable ports 29876 and 29877, as well as port 22 for Linux and port 3389 for Windows. For Pools created with a cloud service configuration, enable ports 10100, 20100, and 30100. Also enable outbound connections to Azure Storage on port 443. For more details see: https://docs.microsoft.com/en-us/azure/batch/batch-api-basics#virtual-network-vnet-and-firewall-configuration"]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(
        rename = "dynamicVNetAssignmentScope",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub dynamic_v_net_assignment_scope: Option<network_configuration::DynamicVNetAssignmentScope>,
    #[serde(rename = "endpointConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<PoolEndpointConfiguration>,
    #[doc = "The public IP Address configuration of the networking configuration of a Pool."]
    #[serde(rename = "publicIPAddressConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address_configuration: Option<PublicIpAddressConfiguration>,
}
impl NetworkConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DynamicVNetAssignmentScope {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "job")]
        Job,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkSecurityGroupRule {
    #[doc = "Priorities within a Pool must be unique and are evaluated in order of priority. The lower the number the higher the priority. For example, rules could be specified with order numbers of 150, 250, and 350. The rule with the order number of 150 takes precedence over the rule that has an order of 250. Allowed priorities are 150 to 4096. If any reserved or duplicate values are provided the request fails with HTTP status code 400."]
    pub priority: i32,
    #[serde(deserialize_with = "case_insensitive_deserialize")]
    pub access: network_security_group_rule::Access,
    #[doc = "Valid values are a single IP address (i.e. 10.10.10.10), IP subnet (i.e. 192.168.1.0/24), default tag, or * (for all addresses).  If any other values are provided the request fails with HTTP status code 400."]
    #[serde(rename = "sourceAddressPrefix")]
    pub source_address_prefix: String,
    #[doc = "Valid values are '*' (for all ports 0 - 65535), a specific port (i.e. 22), or a port range (i.e. 100-200). The ports must be in the range of 0 to 65535. Each entry in this collection must not overlap any other entry (either a range or an individual port). If any other values are provided the request fails with HTTP status code 400. The default value is '*'."]
    #[serde(rename = "sourcePortRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub source_port_ranges: Vec<String>,
}
impl NetworkSecurityGroupRule {
    pub fn new(priority: i32, access: network_security_group_rule::Access, source_address_prefix: String) -> Self {
        Self {
            priority,
            access,
            source_address_prefix,
            source_port_ranges: Vec::new(),
        }
    }
}
pub mod network_security_group_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Access {
        #[serde(rename = "allow")]
        Allow,
        #[serde(rename = "deny")]
        Deny,
    }
}
#[doc = "The Batch Compute Node agent is a program that runs on each Compute Node in the Pool and provides Batch capability on the Compute Node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeAgentInformation {
    #[doc = "This version number can be checked against the Compute Node agent release notes located at https://github.com/Azure/Batch/blob/master/changelogs/nodeagent/CHANGELOG.md."]
    pub version: String,
    #[doc = "This is the most recent time that the Compute Node agent was updated to a new version."]
    #[serde(rename = "lastUpdateTime", with = "azure_core::date::rfc3339")]
    pub last_update_time: time::OffsetDateTime,
}
impl NodeAgentInformation {
    pub fn new(version: String, last_update_time: time::OffsetDateTime) -> Self {
        Self { version, last_update_time }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeCounts {
    pub creating: i32,
    pub idle: i32,
    pub offline: i32,
    pub preempted: i32,
    pub rebooting: i32,
    pub reimaging: i32,
    pub running: i32,
    pub starting: i32,
    #[serde(rename = "startTaskFailed")]
    pub start_task_failed: i32,
    #[serde(rename = "leavingPool")]
    pub leaving_pool: i32,
    pub unknown: i32,
    pub unusable: i32,
    #[serde(rename = "waitingForStartTask")]
    pub waiting_for_start_task: i32,
    pub total: i32,
}
impl NodeCounts {
    pub fn new(
        creating: i32,
        idle: i32,
        offline: i32,
        preempted: i32,
        rebooting: i32,
        reimaging: i32,
        running: i32,
        starting: i32,
        start_task_failed: i32,
        leaving_pool: i32,
        unknown: i32,
        unusable: i32,
        waiting_for_start_task: i32,
        total: i32,
    ) -> Self {
        Self {
            creating,
            idle,
            offline,
            preempted,
            rebooting,
            reimaging,
            running,
            starting,
            start_task_failed,
            leaving_pool,
            unknown,
            unusable,
            waiting_for_start_task,
            total,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeDisableSchedulingParameter {
    #[doc = "The default value is requeue."]
    #[serde(
        rename = "nodeDisableSchedulingOption",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub node_disable_scheduling_option: Option<node_disable_scheduling_parameter::NodeDisableSchedulingOption>,
}
impl NodeDisableSchedulingParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod node_disable_scheduling_parameter {
    use super::*;
    #[doc = "The default value is requeue."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NodeDisableSchedulingOption {
        #[serde(rename = "requeue")]
        Requeue,
        #[serde(rename = "terminate")]
        Terminate,
        #[serde(rename = "taskcompletion")]
        Taskcompletion,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeFile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "isDirectory", default, skip_serializing_if = "Option::is_none")]
    pub is_directory: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FileProperties>,
}
impl NodeFile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeFileListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NodeFile>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for NodeFileListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl NodeFileListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeRebootParameter {
    #[doc = "The default value is requeue."]
    #[serde(
        rename = "nodeRebootOption",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub node_reboot_option: Option<node_reboot_parameter::NodeRebootOption>,
}
impl NodeRebootParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod node_reboot_parameter {
    use super::*;
    #[doc = "The default value is requeue."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NodeRebootOption {
        #[serde(rename = "requeue")]
        Requeue,
        #[serde(rename = "terminate")]
        Terminate,
        #[serde(rename = "taskcompletion")]
        Taskcompletion,
        #[serde(rename = "retaineddata")]
        Retaineddata,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeReimageParameter {
    #[doc = "The default value is requeue."]
    #[serde(
        rename = "nodeReimageOption",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub node_reimage_option: Option<node_reimage_parameter::NodeReimageOption>,
}
impl NodeReimageParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod node_reimage_parameter {
    use super::*;
    #[doc = "The default value is requeue."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NodeReimageOption {
        #[serde(rename = "requeue")]
        Requeue,
        #[serde(rename = "terminate")]
        Terminate,
        #[serde(rename = "taskcompletion")]
        Taskcompletion,
        #[serde(rename = "retaineddata")]
        Retaineddata,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeRemoveParameter {
    #[serde(rename = "nodeList")]
    pub node_list: Vec<String>,
    #[doc = "The default value is 15 minutes. The minimum value is 5 minutes. If you specify a value less than 5 minutes, the Batch service returns an error; if you are calling the REST API directly, the HTTP status code is 400 (Bad Request)."]
    #[serde(rename = "resizeTimeout", default, skip_serializing_if = "Option::is_none")]
    pub resize_timeout: Option<String>,
    #[doc = "The default value is requeue."]
    #[serde(
        rename = "nodeDeallocationOption",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub node_deallocation_option: Option<ComputeNodeDeallocationOption>,
}
impl NodeRemoveParameter {
    pub fn new(node_list: Vec<String>) -> Self {
        Self {
            node_list,
            resize_timeout: None,
            node_deallocation_option: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeUpdateUserParameter {
    #[doc = "The password is required for Windows Compute Nodes (those created with 'cloudServiceConfiguration', or created with 'virtualMachineConfiguration' using a Windows Image reference). For Linux Compute Nodes, the password can optionally be specified along with the sshPublicKey property. If omitted, any existing password is removed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "If omitted, the default is 1 day from the current time. For Linux Compute Nodes, the expiryTime has a precision up to a day."]
    #[serde(rename = "expiryTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[doc = "The public key should be compatible with OpenSSH encoding and should be base 64 encoded. This property can be specified only for Linux Compute Nodes. If this is specified for a Windows Compute Node, then the Batch service rejects the request; if you are calling the REST API directly, the HTTP status code is 400 (Bad Request). If omitted, any existing SSH public key is removed."]
    #[serde(rename = "sshPublicKey", default, skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
}
impl NodeUpdateUserParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OnAllTasksComplete {
    #[serde(rename = "noaction")]
    Noaction,
    #[serde(rename = "terminatejob")]
    Terminatejob,
}
#[doc = "A Task is considered to have failed if has a failureInfo. A failureInfo is set if the Task completes with a non-zero exit code after exhausting its retry count, or if there was an error starting the Task, for example due to a resource file download error. The default is noaction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OnTaskFailure {
    #[serde(rename = "noaction")]
    Noaction,
    #[serde(rename = "performexitoptionsjobaction")]
    Performexitoptionsjobaction,
}
#[doc = "On every file uploads, Batch service writes two log files to the compute node, 'fileuploadout.txt' and 'fileuploaderr.txt'. These log files are used to learn more about a specific failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputFile {
    #[doc = "Both relative and absolute paths are supported. Relative paths are relative to the Task working directory. The following wildcards are supported: * matches 0 or more characters (for example pattern abc* would match abc or abcdef), ** matches any directory, ? matches any single character, [abc] matches one character in the brackets, and [a-c] matches one character in the range. Brackets can include a negation to match any character not specified (for example [!abc] matches any character but a, b, or c). If a file name starts with \".\" it is ignored by default but may be matched by specifying it explicitly (for example *.gif will not match .a.gif, but .*.gif will). A simple example: **\\*.txt matches any file that does not start in '.' and ends with .txt in the Task working directory or any subdirectory. If the filename contains a wildcard character it can be escaped using brackets (for example abc[*] would match a file named abc*). Note that both \\ and / are treated as directory separators on Windows, but only / is on Linux. Environment variables (%var% on Windows or $var on Linux) are expanded prior to the pattern being applied."]
    #[serde(rename = "filePattern")]
    pub file_pattern: String,
    pub destination: OutputFileDestination,
    #[serde(rename = "uploadOptions")]
    pub upload_options: OutputFileUploadOptions,
}
impl OutputFile {
    pub fn new(file_pattern: String, destination: OutputFileDestination, upload_options: OutputFileUploadOptions) -> Self {
        Self {
            file_pattern,
            destination,
            upload_options,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputFileBlobContainerDestination {
    #[doc = "If filePattern refers to a specific file (i.e. contains no wildcards), then path is the name of the blob to which to upload that file. If filePattern contains one or more wildcards (and therefore may match multiple files), then path is the name of the blob virtual directory (which is prepended to each blob name) to which to upload the file(s). If omitted, file(s) are uploaded to the root of the container with a blob name matching their file name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The URL must include a Shared Access Signature (SAS) granting write permissions to the container."]
    #[serde(rename = "containerUrl")]
    pub container_url: String,
}
impl OutputFileBlobContainerDestination {
    pub fn new(container_url: String) -> Self {
        Self { path: None, container_url }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutputFileDestination {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<OutputFileBlobContainerDestination>,
}
impl OutputFileDestination {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OutputFileUploadCondition {
    #[serde(rename = "tasksuccess")]
    Tasksuccess,
    #[serde(rename = "taskfailure")]
    Taskfailure,
    #[serde(rename = "taskcompletion")]
    Taskcompletion,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputFileUploadOptions {
    #[serde(rename = "uploadCondition", deserialize_with = "case_insensitive_deserialize")]
    pub upload_condition: OutputFileUploadCondition,
}
impl OutputFileUploadOptions {
    pub fn new(upload_condition: OutputFileUploadCondition) -> Self {
        Self { upload_condition }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolAddParameter {
    #[doc = "The ID can contain any combination of alphanumeric characters including hyphens and underscores, and cannot contain more than 64 characters. The ID is case-preserving and case-insensitive (that is, you may not have two Pool IDs within an Account that differ only by case)."]
    pub id: String,
    #[doc = "The display name need not be unique and can contain any Unicode characters up to a maximum length of 1024."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "For information about available sizes of virtual machines for Cloud Services Pools (pools created with cloudServiceConfiguration), see Sizes for Cloud Services (https://azure.microsoft.com/documentation/articles/cloud-services-sizes-specs/). Batch supports all Cloud Services VM sizes except ExtraSmall, A1V2 and A2V2. For information about available VM sizes for Pools using Images from the Virtual Machines Marketplace (pools created with virtualMachineConfiguration) see Sizes for Virtual Machines (Linux) (https://azure.microsoft.com/documentation/articles/virtual-machines-linux-sizes/) or Sizes for Virtual Machines (Windows) (https://azure.microsoft.com/documentation/articles/virtual-machines-windows-sizes/). Batch supports all Azure VM sizes except STANDARD_A0 and those with premium storage (STANDARD_GS, STANDARD_DS, and STANDARD_DSV2 series)."]
    #[serde(rename = "vmSize")]
    pub vm_size: String,
    #[serde(rename = "cloudServiceConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cloud_service_configuration: Option<CloudServiceConfiguration>,
    #[serde(rename = "virtualMachineConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_configuration: Option<VirtualMachineConfiguration>,
    #[doc = "This timeout applies only to manual scaling; it has no effect when enableAutoScale is set to true. The default value is 15 minutes. The minimum value is 5 minutes. If you specify a value less than 5 minutes, the Batch service returns an error; if you are calling the REST API directly, the HTTP status code is 400 (Bad Request)."]
    #[serde(rename = "resizeTimeout", default, skip_serializing_if = "Option::is_none")]
    pub resize_timeout: Option<String>,
    #[doc = "This property must not be specified if enableAutoScale is set to true. If enableAutoScale is set to false, then you must set either targetDedicatedNodes, targetLowPriorityNodes, or both."]
    #[serde(rename = "targetDedicatedNodes", default, skip_serializing_if = "Option::is_none")]
    pub target_dedicated_nodes: Option<i32>,
    #[doc = "This property must not be specified if enableAutoScale is set to true. If enableAutoScale is set to false, then you must set either targetDedicatedNodes, targetLowPriorityNodes, or both."]
    #[serde(rename = "targetLowPriorityNodes", default, skip_serializing_if = "Option::is_none")]
    pub target_low_priority_nodes: Option<i32>,
    #[doc = "If false, at least one of targetDedicateNodes and targetLowPriorityNodes must be specified. If true, the autoScaleFormula property is required and the Pool automatically resizes according to the formula. The default value is false."]
    #[serde(rename = "enableAutoScale", default, skip_serializing_if = "Option::is_none")]
    pub enable_auto_scale: Option<bool>,
    #[doc = "This property must not be specified if enableAutoScale is set to false. It is required if enableAutoScale is set to true. The formula is checked for validity before the Pool is created. If the formula is not valid, the Batch service rejects the request with detailed error information. For more information about specifying this formula, see 'Automatically scale Compute Nodes in an Azure Batch Pool' (https://azure.microsoft.com/documentation/articles/batch-automatic-scaling/)."]
    #[serde(rename = "autoScaleFormula", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale_formula: Option<String>,
    #[doc = "The default value is 15 minutes. The minimum and maximum value are 5 minutes and 168 hours respectively. If you specify a value less than 5 minutes or greater than 168 hours, the Batch service returns an error; if you are calling the REST API directly, the HTTP status code is 400 (Bad Request)."]
    #[serde(rename = "autoScaleEvaluationInterval", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale_evaluation_interval: Option<String>,
    #[doc = "Enabling inter-node communication limits the maximum size of the Pool due to deployment restrictions on the Compute Nodes of the Pool. This may result in the Pool not reaching its desired size. The default value is false."]
    #[serde(rename = "enableInterNodeCommunication", default, skip_serializing_if = "Option::is_none")]
    pub enable_inter_node_communication: Option<bool>,
    #[doc = "The network configuration for a Pool."]
    #[serde(rename = "networkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[doc = "Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing. In some cases the StartTask may be re-run even though the Compute Node was not rebooted. Special care should be taken to avoid StartTasks which create breakaway process or install/launch services from the StartTask working directory, as this will block Batch from being able to re-run the StartTask."]
    #[serde(rename = "startTask", default, skip_serializing_if = "Option::is_none")]
    pub start_task: Option<StartTask>,
    #[doc = "For Windows Nodes, the Batch service installs the Certificates to the specified Certificate store and location. For Linux Compute Nodes, the Certificates are stored in a directory inside the Task working directory and an environment variable AZ_BATCH_CERTIFICATES_DIR is supplied to the Task to query for this location. For Certificates with visibility of 'remoteUser', a 'certs' directory is created in the user's home directory (e.g., /home/{user-name}/certs) and Certificates are placed in that directory."]
    #[serde(rename = "certificateReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub certificate_references: Vec<CertificateReference>,
    #[doc = "Changes to Package references affect all new Nodes joining the Pool, but do not affect Compute Nodes that are already in the Pool until they are rebooted or reimaged. There is a maximum of 10 Package references on any given Pool."]
    #[serde(rename = "applicationPackageReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub application_package_references: Vec<ApplicationPackageReference>,
    #[doc = "The list of application licenses must be a subset of available Batch service application licenses. If a license is requested which is not supported, Pool creation will fail."]
    #[serde(rename = "applicationLicenses", default, skip_serializing_if = "Vec::is_empty")]
    pub application_licenses: Vec<String>,
    #[doc = "The default value is 1. The maximum value is the smaller of 4 times the number of cores of the vmSize of the Pool or 256."]
    #[serde(rename = "maxTasksPerNode", default, skip_serializing_if = "Option::is_none")]
    pub max_tasks_per_node: Option<i32>,
    #[serde(rename = "taskSchedulingPolicy", default, skip_serializing_if = "Option::is_none")]
    pub task_scheduling_policy: Option<TaskSchedulingPolicy>,
    #[serde(rename = "userAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub user_accounts: Vec<UserAccount>,
    #[doc = "The Batch service does not assign any meaning to metadata; it is solely for the use of user code."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<MetadataItem>,
    #[doc = "Mount the storage using Azure fileshare, NFS, CIFS or Blobfuse based file system."]
    #[serde(rename = "mountConfiguration", default, skip_serializing_if = "Vec::is_empty")]
    pub mount_configuration: Vec<MountConfiguration>,
}
impl PoolAddParameter {
    pub fn new(id: String, vm_size: String) -> Self {
        Self {
            id,
            display_name: None,
            vm_size,
            cloud_service_configuration: None,
            virtual_machine_configuration: None,
            resize_timeout: None,
            target_dedicated_nodes: None,
            target_low_priority_nodes: None,
            enable_auto_scale: None,
            auto_scale_formula: None,
            auto_scale_evaluation_interval: None,
            enable_inter_node_communication: None,
            network_configuration: None,
            start_task: None,
            certificate_references: Vec::new(),
            application_package_references: Vec::new(),
            application_licenses: Vec::new(),
            max_tasks_per_node: None,
            task_scheduling_policy: None,
            user_accounts: Vec::new(),
            metadata: Vec::new(),
            mount_configuration: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolEnableAutoScaleParameter {
    #[doc = "The formula is checked for validity before it is applied to the Pool. If the formula is not valid, the Batch service rejects the request with detailed error information. For more information about specifying this formula, see Automatically scale Compute Nodes in an Azure Batch Pool (https://azure.microsoft.com/en-us/documentation/articles/batch-automatic-scaling)."]
    #[serde(rename = "autoScaleFormula", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale_formula: Option<String>,
    #[doc = "The default value is 15 minutes. The minimum and maximum value are 5 minutes and 168 hours respectively. If you specify a value less than 5 minutes or greater than 168 hours, the Batch service rejects the request with an invalid property value error; if you are calling the REST API directly, the HTTP status code is 400 (Bad Request). If you specify a new interval, then the existing autoscale evaluation schedule will be stopped and a new autoscale evaluation schedule will be started, with its starting time being the time when this request was issued."]
    #[serde(rename = "autoScaleEvaluationInterval", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale_evaluation_interval: Option<String>,
}
impl PoolEnableAutoScaleParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolEndpointConfiguration {
    #[doc = "The maximum number of inbound NAT Pools per Batch Pool is 5. If the maximum number of inbound NAT Pools is exceeded the request fails with HTTP status code 400."]
    #[serde(rename = "inboundNATPools")]
    pub inbound_nat_pools: Vec<InboundNatPool>,
}
impl PoolEndpointConfiguration {
    pub fn new(inbound_nat_pools: Vec<InboundNatPool>) -> Self {
        Self { inbound_nat_pools }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolEvaluateAutoScaleParameter {
    #[doc = "The formula is validated and its results calculated, but it is not applied to the Pool. To apply the formula to the Pool, 'Enable automatic scaling on a Pool'. For more information about specifying this formula, see Automatically scale Compute Nodes in an Azure Batch Pool (https://azure.microsoft.com/en-us/documentation/articles/batch-automatic-scaling)."]
    #[serde(rename = "autoScaleFormula")]
    pub auto_scale_formula: String,
}
impl PoolEvaluateAutoScaleParameter {
    pub fn new(auto_scale_formula: String) -> Self {
        Self { auto_scale_formula }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolInformation {
    #[doc = "You must ensure that the Pool referenced by this property exists. If the Pool does not exist at the time the Batch service tries to schedule a Job, no Tasks for the Job will run until you create a Pool with that id. Note that the Batch service will not reject the Job request; it will simply not run Tasks until the Pool exists. You must specify either the Pool ID or the auto Pool specification, but not both."]
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    #[serde(rename = "autoPoolSpecification", default, skip_serializing_if = "Option::is_none")]
    pub auto_pool_specification: Option<AutoPoolSpecification>,
}
impl PoolInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolListUsageMetricsResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PoolUsageMetrics>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for PoolListUsageMetricsResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl PoolListUsageMetricsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolNodeCounts {
    #[serde(rename = "poolId")]
    pub pool_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dedicated: Option<NodeCounts>,
    #[serde(rename = "lowPriority", default, skip_serializing_if = "Option::is_none")]
    pub low_priority: Option<NodeCounts>,
}
impl PoolNodeCounts {
    pub fn new(pool_id: String) -> Self {
        Self {
            pool_id,
            dedicated: None,
            low_priority: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolNodeCountsListResult {
    #[doc = "A list of Compute Node counts by Pool."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PoolNodeCounts>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for PoolNodeCountsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl PoolNodeCountsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolPatchParameter {
    #[doc = "Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing. In some cases the StartTask may be re-run even though the Compute Node was not rebooted. Special care should be taken to avoid StartTasks which create breakaway process or install/launch services from the StartTask working directory, as this will block Batch from being able to re-run the StartTask."]
    #[serde(rename = "startTask", default, skip_serializing_if = "Option::is_none")]
    pub start_task: Option<StartTask>,
    #[doc = "If this element is present, it replaces any existing Certificate references configured on the Pool. If omitted, any existing Certificate references are left unchanged. For Windows Nodes, the Batch service installs the Certificates to the specified Certificate store and location. For Linux Compute Nodes, the Certificates are stored in a directory inside the Task working directory and an environment variable AZ_BATCH_CERTIFICATES_DIR is supplied to the Task to query for this location. For Certificates with visibility of 'remoteUser', a 'certs' directory is created in the user's home directory (e.g., /home/{user-name}/certs) and Certificates are placed in that directory."]
    #[serde(rename = "certificateReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub certificate_references: Vec<CertificateReference>,
    #[doc = "Changes to Package references affect all new Nodes joining the Pool, but do not affect Compute Nodes that are already in the Pool until they are rebooted or reimaged. If this element is present, it replaces any existing Package references. If you specify an empty collection, then all Package references are removed from the Pool. If omitted, any existing Package references are left unchanged."]
    #[serde(rename = "applicationPackageReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub application_package_references: Vec<ApplicationPackageReference>,
    #[doc = "If this element is present, it replaces any existing metadata configured on the Pool. If you specify an empty collection, any metadata is removed from the Pool. If omitted, any existing metadata is left unchanged."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<MetadataItem>,
}
impl PoolPatchParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolResizeParameter {
    #[serde(rename = "targetDedicatedNodes", default, skip_serializing_if = "Option::is_none")]
    pub target_dedicated_nodes: Option<i32>,
    #[serde(rename = "targetLowPriorityNodes", default, skip_serializing_if = "Option::is_none")]
    pub target_low_priority_nodes: Option<i32>,
    #[doc = "The default value is 15 minutes. The minimum value is 5 minutes. If you specify a value less than 5 minutes, the Batch service returns an error; if you are calling the REST API directly, the HTTP status code is 400 (Bad Request)."]
    #[serde(rename = "resizeTimeout", default, skip_serializing_if = "Option::is_none")]
    pub resize_timeout: Option<String>,
    #[doc = "The default value is requeue."]
    #[serde(
        rename = "nodeDeallocationOption",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub node_deallocation_option: Option<ComputeNodeDeallocationOption>,
}
impl PoolResizeParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolSpecification {
    #[doc = "The display name need not be unique and can contain any Unicode characters up to a maximum length of 1024."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "For information about available sizes of virtual machines in Pools, see Choose a VM size for Compute Nodes in an Azure Batch Pool (https://docs.microsoft.com/azure/batch/batch-pool-vm-sizes)."]
    #[serde(rename = "vmSize")]
    pub vm_size: String,
    #[serde(rename = "cloudServiceConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cloud_service_configuration: Option<CloudServiceConfiguration>,
    #[serde(rename = "virtualMachineConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_configuration: Option<VirtualMachineConfiguration>,
    #[doc = "The default value is 1. The maximum value is the smaller of 4 times the number of cores of the vmSize of the Pool or 256."]
    #[serde(rename = "maxTasksPerNode", default, skip_serializing_if = "Option::is_none")]
    pub max_tasks_per_node: Option<i32>,
    #[serde(rename = "taskSchedulingPolicy", default, skip_serializing_if = "Option::is_none")]
    pub task_scheduling_policy: Option<TaskSchedulingPolicy>,
    #[doc = "This timeout applies only to manual scaling; it has no effect when enableAutoScale is set to true. The default value is 15 minutes. The minimum value is 5 minutes. If you specify a value less than 5 minutes, the Batch service rejects the request with an error; if you are calling the REST API directly, the HTTP status code is 400 (Bad Request)."]
    #[serde(rename = "resizeTimeout", default, skip_serializing_if = "Option::is_none")]
    pub resize_timeout: Option<String>,
    #[doc = "This property must not be specified if enableAutoScale is set to true. If enableAutoScale is set to false, then you must set either targetDedicatedNodes, targetLowPriorityNodes, or both."]
    #[serde(rename = "targetDedicatedNodes", default, skip_serializing_if = "Option::is_none")]
    pub target_dedicated_nodes: Option<i32>,
    #[doc = "This property must not be specified if enableAutoScale is set to true. If enableAutoScale is set to false, then you must set either targetDedicatedNodes, targetLowPriorityNodes, or both."]
    #[serde(rename = "targetLowPriorityNodes", default, skip_serializing_if = "Option::is_none")]
    pub target_low_priority_nodes: Option<i32>,
    #[doc = "If false, at least one of targetDedicateNodes and targetLowPriorityNodes must be specified. If true, the autoScaleFormula element is required. The Pool automatically resizes according to the formula. The default value is false."]
    #[serde(rename = "enableAutoScale", default, skip_serializing_if = "Option::is_none")]
    pub enable_auto_scale: Option<bool>,
    #[doc = "This property must not be specified if enableAutoScale is set to false. It is required if enableAutoScale is set to true. The formula is checked for validity before the Pool is created. If the formula is not valid, the Batch service rejects the request with detailed error information."]
    #[serde(rename = "autoScaleFormula", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale_formula: Option<String>,
    #[doc = "The default value is 15 minutes. The minimum and maximum value are 5 minutes and 168 hours respectively. If you specify a value less than 5 minutes or greater than 168 hours, the Batch service rejects the request with an invalid property value error; if you are calling the REST API directly, the HTTP status code is 400 (Bad Request)."]
    #[serde(rename = "autoScaleEvaluationInterval", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale_evaluation_interval: Option<String>,
    #[doc = "Enabling inter-node communication limits the maximum size of the Pool due to deployment restrictions on the Compute Nodes of the Pool. This may result in the Pool not reaching its desired size. The default value is false."]
    #[serde(rename = "enableInterNodeCommunication", default, skip_serializing_if = "Option::is_none")]
    pub enable_inter_node_communication: Option<bool>,
    #[doc = "The network configuration for a Pool."]
    #[serde(rename = "networkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[doc = "Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing. In some cases the StartTask may be re-run even though the Compute Node was not rebooted. Special care should be taken to avoid StartTasks which create breakaway process or install/launch services from the StartTask working directory, as this will block Batch from being able to re-run the StartTask."]
    #[serde(rename = "startTask", default, skip_serializing_if = "Option::is_none")]
    pub start_task: Option<StartTask>,
    #[doc = "For Windows Nodes, the Batch service installs the Certificates to the specified Certificate store and location. For Linux Compute Nodes, the Certificates are stored in a directory inside the Task working directory and an environment variable AZ_BATCH_CERTIFICATES_DIR is supplied to the Task to query for this location. For Certificates with visibility of 'remoteUser', a 'certs' directory is created in the user's home directory (e.g., /home/{user-name}/certs) and Certificates are placed in that directory."]
    #[serde(rename = "certificateReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub certificate_references: Vec<CertificateReference>,
    #[doc = "Changes to Package references affect all new Nodes joining the Pool, but do not affect Compute Nodes that are already in the Pool until they are rebooted or reimaged. There is a maximum of 10 Package references on any given Pool."]
    #[serde(rename = "applicationPackageReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub application_package_references: Vec<ApplicationPackageReference>,
    #[doc = "The list of application licenses must be a subset of available Batch service application licenses. If a license is requested which is not supported, Pool creation will fail. The permitted licenses available on the Pool are 'maya', 'vray', '3dsmax', 'arnold'. An additional charge applies for each application license added to the Pool."]
    #[serde(rename = "applicationLicenses", default, skip_serializing_if = "Vec::is_empty")]
    pub application_licenses: Vec<String>,
    #[serde(rename = "userAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub user_accounts: Vec<UserAccount>,
    #[doc = "The Batch service does not assign any meaning to metadata; it is solely for the use of user code."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<MetadataItem>,
    #[doc = "This supports Azure Files, NFS, CIFS/SMB, and Blobfuse."]
    #[serde(rename = "mountConfiguration", default, skip_serializing_if = "Vec::is_empty")]
    pub mount_configuration: Vec<MountConfiguration>,
}
impl PoolSpecification {
    pub fn new(vm_size: String) -> Self {
        Self {
            display_name: None,
            vm_size,
            cloud_service_configuration: None,
            virtual_machine_configuration: None,
            max_tasks_per_node: None,
            task_scheduling_policy: None,
            resize_timeout: None,
            target_dedicated_nodes: None,
            target_low_priority_nodes: None,
            enable_auto_scale: None,
            auto_scale_formula: None,
            auto_scale_evaluation_interval: None,
            enable_inter_node_communication: None,
            network_configuration: None,
            start_task: None,
            certificate_references: Vec::new(),
            application_package_references: Vec::new(),
            application_licenses: Vec::new(),
            user_accounts: Vec::new(),
            metadata: Vec::new(),
            mount_configuration: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolStatistics {
    pub url: String,
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[serde(rename = "lastUpdateTime", with = "azure_core::date::rfc3339")]
    pub last_update_time: time::OffsetDateTime,
    #[serde(rename = "usageStats", default, skip_serializing_if = "Option::is_none")]
    pub usage_stats: Option<UsageStatistics>,
    #[serde(rename = "resourceStats", default, skip_serializing_if = "Option::is_none")]
    pub resource_stats: Option<ResourceStatistics>,
}
impl PoolStatistics {
    pub fn new(url: String, start_time: time::OffsetDateTime, last_update_time: time::OffsetDateTime) -> Self {
        Self {
            url,
            start_time,
            last_update_time,
            usage_stats: None,
            resource_stats: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolUpdatePropertiesParameter {
    #[doc = "Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing. In some cases the StartTask may be re-run even though the Compute Node was not rebooted. Special care should be taken to avoid StartTasks which create breakaway process or install/launch services from the StartTask working directory, as this will block Batch from being able to re-run the StartTask."]
    #[serde(rename = "startTask", default, skip_serializing_if = "Option::is_none")]
    pub start_task: Option<StartTask>,
    #[doc = "This list replaces any existing Certificate references configured on the Pool. If you specify an empty collection, any existing Certificate references are removed from the Pool. For Windows Nodes, the Batch service installs the Certificates to the specified Certificate store and location. For Linux Compute Nodes, the Certificates are stored in a directory inside the Task working directory and an environment variable AZ_BATCH_CERTIFICATES_DIR is supplied to the Task to query for this location. For Certificates with visibility of 'remoteUser', a 'certs' directory is created in the user's home directory (e.g., /home/{user-name}/certs) and Certificates are placed in that directory."]
    #[serde(rename = "certificateReferences")]
    pub certificate_references: Vec<CertificateReference>,
    #[doc = "The list replaces any existing Application Package references on the Pool. Changes to Application Package references affect all new Compute Nodes joining the Pool, but do not affect Compute Nodes that are already in the Pool until they are rebooted or reimaged. There is a maximum of 10 Application Package references on any given Pool. If omitted, or if you specify an empty collection, any existing Application Packages references are removed from the Pool. A maximum of 10 references may be specified on a given Pool."]
    #[serde(rename = "applicationPackageReferences")]
    pub application_package_references: Vec<ApplicationPackageReference>,
    #[doc = "This list replaces any existing metadata configured on the Pool. If omitted, or if you specify an empty collection, any existing metadata is removed from the Pool."]
    pub metadata: Vec<MetadataItem>,
}
impl PoolUpdatePropertiesParameter {
    pub fn new(
        certificate_references: Vec<CertificateReference>,
        application_package_references: Vec<ApplicationPackageReference>,
        metadata: Vec<MetadataItem>,
    ) -> Self {
        Self {
            start_task: None,
            certificate_references,
            application_package_references,
            metadata,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolUsageMetrics {
    #[serde(rename = "poolId")]
    pub pool_id: String,
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339")]
    pub end_time: time::OffsetDateTime,
    #[doc = "For information about available sizes of virtual machines in Pools, see Choose a VM size for Compute Nodes in an Azure Batch Pool (https://docs.microsoft.com/azure/batch/batch-pool-vm-sizes)."]
    #[serde(rename = "vmSize")]
    pub vm_size: String,
    #[serde(rename = "totalCoreHours")]
    pub total_core_hours: f64,
}
impl PoolUsageMetrics {
    pub fn new(
        pool_id: String,
        start_time: time::OffsetDateTime,
        end_time: time::OffsetDateTime,
        vm_size: String,
        total_core_hours: f64,
    ) -> Self {
        Self {
            pool_id,
            start_time,
            end_time,
            vm_size,
            total_core_hours,
        }
    }
}
#[doc = "The public IP Address configuration of the networking configuration of a Pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicIpAddressConfiguration {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub provision: Option<IpAddressProvisioningType>,
    #[doc = "The number of IPs specified here limits the maximum size of the Pool - 50 dedicated nodes or 20 low-priority nodes can be allocated for each public IP. For example, a pool needing 150 dedicated VMs would need at least 3 public IPs specified. Each element of this collection is of the form: /subscriptions/{subscription}/resourceGroups/{group}/providers/Microsoft.Network/publicIPAddresses/{ip}."]
    #[serde(rename = "ipAddressIds", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_address_ids: Vec<String>,
}
impl PublicIpAddressConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecentJob {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl RecentJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResizeError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<NameValuePair>,
}
impl ResizeError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceFile {
    #[doc = "The autoStorageContainerName, storageContainerUrl and httpUrl properties are mutually exclusive and one of them must be specified."]
    #[serde(rename = "autoStorageContainerName", default, skip_serializing_if = "Option::is_none")]
    pub auto_storage_container_name: Option<String>,
    #[doc = "The autoStorageContainerName, storageContainerUrl and httpUrl properties are mutually exclusive and one of them must be specified. This URL must be readable and listable using anonymous access; that is, the Batch service does not present any credentials when downloading blobs from the container. There are two ways to get such a URL for a container in Azure storage: include a Shared Access Signature (SAS) granting read and list permissions on the container, or set the ACL for the container to allow public access."]
    #[serde(rename = "storageContainerUrl", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_url: Option<String>,
    #[doc = "The autoStorageContainerName, storageContainerUrl and httpUrl properties are mutually exclusive and one of them must be specified. If the URL points to Azure Blob Storage, it must be readable using anonymous access; that is, the Batch service does not present any credentials when downloading the blob. There are two ways to get such a URL for a blob in Azure storage: include a Shared Access Signature (SAS) granting read permissions on the blob, or set the ACL for the blob or its container to allow public access."]
    #[serde(rename = "httpUrl", default, skip_serializing_if = "Option::is_none")]
    pub http_url: Option<String>,
    #[doc = "The property is valid only when autoStorageContainerName or storageContainerUrl is used. This prefix can be a partial filename or a subdirectory. If a prefix is not specified, all the files in the container will be downloaded."]
    #[serde(rename = "blobPrefix", default, skip_serializing_if = "Option::is_none")]
    pub blob_prefix: Option<String>,
    #[doc = "If the httpUrl property is specified, the filePath is required and describes the path which the file will be downloaded to, including the filename. Otherwise, if the autoStorageContainerName or storageContainerUrl property is specified, filePath is optional and is the directory to download the files to. In the case where filePath is used as a directory, any directory structure already associated with the input data will be retained in full and appended to the specified filePath directory. The specified relative path cannot break out of the Task's working directory (for example by using '..')."]
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[doc = "This property applies only to files being downloaded to Linux Compute Nodes. It will be ignored if it is specified for a resourceFile which will be downloaded to a Windows Compute Node. If this property is not specified for a Linux Compute Node, then a default value of 0770 is applied to the file."]
    #[serde(rename = "fileMode", default, skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
}
impl ResourceFile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceStatistics {
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[serde(rename = "lastUpdateTime", with = "azure_core::date::rfc3339")]
    pub last_update_time: time::OffsetDateTime,
    #[serde(rename = "avgCPUPercentage")]
    pub avg_cpu_percentage: f64,
    #[serde(rename = "avgMemoryGiB")]
    pub avg_memory_gi_b: f64,
    #[serde(rename = "peakMemoryGiB")]
    pub peak_memory_gi_b: f64,
    #[serde(rename = "avgDiskGiB")]
    pub avg_disk_gi_b: f64,
    #[serde(rename = "peakDiskGiB")]
    pub peak_disk_gi_b: f64,
    #[serde(rename = "diskReadIOps")]
    pub disk_read_i_ops: i64,
    #[serde(rename = "diskWriteIOps")]
    pub disk_write_i_ops: i64,
    #[serde(rename = "diskReadGiB")]
    pub disk_read_gi_b: f64,
    #[serde(rename = "diskWriteGiB")]
    pub disk_write_gi_b: f64,
    #[serde(rename = "networkReadGiB")]
    pub network_read_gi_b: f64,
    #[serde(rename = "networkWriteGiB")]
    pub network_write_gi_b: f64,
}
impl ResourceStatistics {
    pub fn new(
        start_time: time::OffsetDateTime,
        last_update_time: time::OffsetDateTime,
        avg_cpu_percentage: f64,
        avg_memory_gi_b: f64,
        peak_memory_gi_b: f64,
        avg_disk_gi_b: f64,
        peak_disk_gi_b: f64,
        disk_read_i_ops: i64,
        disk_write_i_ops: i64,
        disk_read_gi_b: f64,
        disk_write_gi_b: f64,
        network_read_gi_b: f64,
        network_write_gi_b: f64,
    ) -> Self {
        Self {
            start_time,
            last_update_time,
            avg_cpu_percentage,
            avg_memory_gi_b,
            peak_memory_gi_b,
            avg_disk_gi_b,
            peak_disk_gi_b,
            disk_read_i_ops,
            disk_write_i_ops,
            disk_read_gi_b,
            disk_write_gi_b,
            network_read_gi_b,
            network_write_gi_b,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Schedule {
    #[doc = "If you do not specify a doNotRunUntil time, the schedule becomes ready to create Jobs immediately."]
    #[serde(rename = "doNotRunUntil", default, with = "azure_core::date::rfc3339::option")]
    pub do_not_run_until: Option<time::OffsetDateTime>,
    #[doc = "If you do not specify a doNotRunAfter time, and you are creating a recurring Job Schedule, the Job Schedule will remain active until you explicitly terminate it."]
    #[serde(rename = "doNotRunAfter", default, with = "azure_core::date::rfc3339::option")]
    pub do_not_run_after: Option<time::OffsetDateTime>,
    #[doc = "If a Job is not created within the startWindow interval, then the 'opportunity' is lost; no Job will be created until the next recurrence of the schedule. If the schedule is recurring, and the startWindow is longer than the recurrence interval, then this is equivalent to an infinite startWindow, because the Job that is 'due' in one recurrenceInterval is not carried forward into the next recurrence interval. The default is infinite. The minimum value is 1 minute. If you specify a lower value, the Batch service rejects the schedule with an error; if you are calling the REST API directly, the HTTP status code is 400 (Bad Request)."]
    #[serde(rename = "startWindow", default, skip_serializing_if = "Option::is_none")]
    pub start_window: Option<String>,
    #[doc = "Because a Job Schedule can have at most one active Job under it at any given time, if it is time to create a new Job under a Job Schedule, but the previous Job is still running, the Batch service will not create the new Job until the previous Job finishes. If the previous Job does not finish within the startWindow period of the new recurrenceInterval, then no new Job will be scheduled for that interval. For recurring Jobs, you should normally specify a jobManagerTask in the jobSpecification. If you do not use jobManagerTask, you will need an external process to monitor when Jobs are created, add Tasks to the Jobs and terminate the Jobs ready for the next recurrence. The default is that the schedule does not recur: one Job is created, within the startWindow after the doNotRunUntil time, and the schedule is complete as soon as that Job finishes. The minimum value is 1 minute. If you specify a lower value, the Batch service rejects the schedule with an error; if you are calling the REST API directly, the HTTP status code is 400 (Bad Request)."]
    #[serde(rename = "recurrenceInterval", default, skip_serializing_if = "Option::is_none")]
    pub recurrence_interval: Option<String>,
}
impl Schedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing. In some cases the StartTask may be re-run even though the Compute Node was not rebooted. Special care should be taken to avoid StartTasks which create breakaway process or install/launch services from the StartTask working directory, as this will block Batch from being able to re-run the StartTask."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartTask {
    #[doc = "The command line does not run under a shell, and therefore cannot take advantage of shell features such as environment variable expansion. If you want to take advantage of such features, you should invoke the shell in the command line, for example using \"cmd /c MyCommand\" in Windows or \"/bin/sh -c MyCommand\" in Linux. If the command line refers to file paths, it should use a relative path (relative to the Task working directory), or use the Batch provided environment variable (https://docs.microsoft.com/en-us/azure/batch/batch-compute-node-environment-variables)."]
    #[serde(rename = "commandLine")]
    pub command_line: String,
    #[serde(rename = "containerSettings", default, skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<TaskContainerSettings>,
    #[doc = "Files listed under this element are located in the Task's working directory."]
    #[serde(rename = "resourceFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_files: Vec<ResourceFile>,
    #[serde(rename = "environmentSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_settings: Vec<EnvironmentSetting>,
    #[doc = "Specify either the userName or autoUser property, but not both."]
    #[serde(rename = "userIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<UserIdentity>,
    #[doc = "The Batch service retries a Task if its exit code is nonzero. Note that this value specifically controls the number of retries. The Batch service will try the Task once, and may then retry up to this limit. For example, if the maximum retry count is 3, Batch tries the Task up to 4 times (one initial try and 3 retries). If the maximum retry count is 0, the Batch service does not retry the Task. If the maximum retry count is -1, the Batch service retries the Task without limit."]
    #[serde(rename = "maxTaskRetryCount", default, skip_serializing_if = "Option::is_none")]
    pub max_task_retry_count: Option<i32>,
    #[doc = "If true and the StartTask fails on a Node, the Batch service retries the StartTask up to its maximum retry count (maxTaskRetryCount). If the Task has still not completed successfully after all retries, then the Batch service marks the Node unusable, and will not schedule Tasks to it. This condition can be detected via the Compute Node state and failure info details. If false, the Batch service will not wait for the StartTask to complete. In this case, other Tasks can start executing on the Compute Node while the StartTask is still running; and even if the StartTask fails, new Tasks will continue to be scheduled on the Compute Node. The default is true."]
    #[serde(rename = "waitForSuccess", default, skip_serializing_if = "Option::is_none")]
    pub wait_for_success: Option<bool>,
}
impl StartTask {
    pub fn new(command_line: String) -> Self {
        Self {
            command_line,
            container_settings: None,
            resource_files: Vec::new(),
            environment_settings: Vec::new(),
            user_identity: None,
            max_task_retry_count: None,
            wait_for_success: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartTaskInformation {
    #[serde(deserialize_with = "case_insensitive_deserialize")]
    pub state: start_task_information::State,
    #[doc = "This value is reset every time the Task is restarted or retried (that is, this is the most recent time at which the StartTask started running)."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "This is the end time of the most recent run of the StartTask, if that run has completed (even if that run failed and a retry is pending). This element is not present if the StartTask is currently running."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "This property is set only if the StartTask is in the completed state. In general, the exit code for a process reflects the specific convention implemented by the application developer for that process. If you use the exit code value to make decisions in your code, be sure that you know the exit code convention used by the application process. However, if the Batch service terminates the StartTask (due to timeout, or user termination via the API) you may see an operating system-defined exit code."]
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "containerInfo", default, skip_serializing_if = "Option::is_none")]
    pub container_info: Option<TaskContainerExecutionInformation>,
    #[serde(rename = "failureInfo", default, skip_serializing_if = "Option::is_none")]
    pub failure_info: Option<TaskFailureInformation>,
    #[doc = "Task application failures (non-zero exit code) are retried, pre-processing errors (the Task could not be run) and file upload errors are not retried. The Batch service will retry the Task up to the limit specified by the constraints."]
    #[serde(rename = "retryCount")]
    pub retry_count: i32,
    #[doc = "This element is present only if the Task was retried (i.e. retryCount is nonzero). If present, this is typically the same as startTime, but may be different if the Task has been restarted for reasons other than retry; for example, if the Compute Node was rebooted during a retry, then the startTime is updated but the lastRetryTime is not."]
    #[serde(rename = "lastRetryTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_retry_time: Option<time::OffsetDateTime>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub result: Option<TaskExecutionResult>,
}
impl StartTaskInformation {
    pub fn new(state: start_task_information::State, start_time: time::OffsetDateTime, retry_count: i32) -> Self {
        Self {
            state,
            start_time,
            end_time: None,
            exit_code: None,
            container_info: None,
            failure_info: None,
            retry_count,
            last_retry_time: None,
            result: None,
        }
    }
}
pub mod start_task_information {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "completed")]
        Completed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StorageAccountType {
    #[serde(rename = "standard_lrs")]
    StandardLrs,
    #[serde(rename = "premium_lrs")]
    PremiumLrs,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubtaskInformation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "nodeInfo", default, skip_serializing_if = "Option::is_none")]
    pub node_info: Option<ComputeNodeInformation>,
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "This property is set only if the subtask is in the Completed state."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "This property is set only if the subtask is in the completed state. In general, the exit code for a process reflects the specific convention implemented by the application developer for that process. If you use the exit code value to make decisions in your code, be sure that you know the exit code convention used by the application process. However, if the Batch service terminates the subtask (due to timeout, or user termination via the API) you may see an operating system-defined exit code."]
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "containerInfo", default, skip_serializing_if = "Option::is_none")]
    pub container_info: Option<TaskContainerExecutionInformation>,
    #[serde(rename = "failureInfo", default, skip_serializing_if = "Option::is_none")]
    pub failure_info: Option<TaskFailureInformation>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub state: Option<SubtaskState>,
    #[serde(rename = "stateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub state_transition_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "previousState",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub previous_state: Option<SubtaskState>,
    #[doc = "This property is not set if the subtask is in its initial running state."]
    #[serde(rename = "previousStateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub previous_state_transition_time: Option<time::OffsetDateTime>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub result: Option<TaskExecutionResult>,
}
impl SubtaskInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SubtaskState {
    #[serde(rename = "preparing")]
    Preparing,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskAddCollectionParameter {
    #[doc = "The total serialized size of this collection must be less than 1MB. If it is greater than 1MB (for example if each Task has 100's of resource files or environment variables), the request will fail with code 'RequestBodyTooLarge' and should be retried again with fewer Tasks."]
    pub value: Vec<TaskAddParameter>,
}
impl TaskAddCollectionParameter {
    pub fn new(value: Vec<TaskAddParameter>) -> Self {
        Self { value }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAddCollectionResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TaskAddResult>,
}
impl TaskAddCollectionResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Batch will retry Tasks when a recovery operation is triggered on a Node. Examples of recovery operations include (but are not limited to) when an unhealthy Node is rebooted or a Compute Node disappeared due to host failure. Retries due to recovery operations are independent of and are not counted against the maxTaskRetryCount. Even if the maxTaskRetryCount is 0, an internal retry due to a recovery operation may occur. Because of this, all Tasks should be idempotent. This means Tasks need to tolerate being interrupted and restarted without causing any corruption or duplicate data. The best practice for long running Tasks is to use some form of checkpointing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskAddParameter {
    #[doc = "The ID can contain any combination of alphanumeric characters including hyphens and underscores, and cannot contain more than 64 characters. The ID is case-preserving and case-insensitive (that is, you may not have two IDs within a Job that differ only by case)."]
    pub id: String,
    #[doc = "The display name need not be unique and can contain any Unicode characters up to a maximum length of 1024."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "For multi-instance Tasks, the command line is executed as the primary Task, after the primary Task and all subtasks have finished executing the coordination command line. The command line does not run under a shell, and therefore cannot take advantage of shell features such as environment variable expansion. If you want to take advantage of such features, you should invoke the shell in the command line, for example using \"cmd /c MyCommand\" in Windows or \"/bin/sh -c MyCommand\" in Linux. If the command line refers to file paths, it should use a relative path (relative to the Task working directory), or use the Batch provided environment variable (https://docs.microsoft.com/en-us/azure/batch/batch-compute-node-environment-variables)."]
    #[serde(rename = "commandLine")]
    pub command_line: String,
    #[serde(rename = "containerSettings", default, skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<TaskContainerSettings>,
    #[serde(rename = "exitConditions", default, skip_serializing_if = "Option::is_none")]
    pub exit_conditions: Option<ExitConditions>,
    #[doc = "For multi-instance Tasks, the resource files will only be downloaded to the Compute Node on which the primary Task is executed. There is a maximum size for the list of resource files.  When the max size is exceeded, the request will fail and the response error code will be RequestEntityTooLarge. If this occurs, the collection of ResourceFiles must be reduced in size. This can be achieved using .zip files, Application Packages, or Docker Containers."]
    #[serde(rename = "resourceFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_files: Vec<ResourceFile>,
    #[doc = "For multi-instance Tasks, the files will only be uploaded from the Compute Node on which the primary Task is executed."]
    #[serde(rename = "outputFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub output_files: Vec<OutputFile>,
    #[serde(rename = "environmentSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_settings: Vec<EnvironmentSetting>,
    #[serde(rename = "affinityInfo", default, skip_serializing_if = "Option::is_none")]
    pub affinity_info: Option<AffinityInformation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<TaskConstraints>,
    #[doc = "Specify either the userName or autoUser property, but not both."]
    #[serde(rename = "userIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<UserIdentity>,
    #[doc = "Multi-instance Tasks are commonly used to support MPI Tasks. In the MPI case, if any of the subtasks fail (for example due to exiting with a non-zero exit code) the entire multi-instance Task fails. The multi-instance Task is then terminated and retried, up to its retry limit."]
    #[serde(rename = "multiInstanceSettings", default, skip_serializing_if = "Option::is_none")]
    pub multi_instance_settings: Option<MultiInstanceSettings>,
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<TaskDependencies>,
    #[doc = "Application packages are downloaded and deployed to a shared directory, not the Task working directory. Therefore, if a referenced package is already on the Node, and is up to date, then it is not re-downloaded; the existing copy on the Compute Node is used. If a referenced Package cannot be installed, for example because the package has been deleted or because download failed, the Task fails."]
    #[serde(rename = "applicationPackageReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub application_package_references: Vec<ApplicationPackageReference>,
    #[serde(rename = "authenticationTokenSettings", default, skip_serializing_if = "Option::is_none")]
    pub authentication_token_settings: Option<AuthenticationTokenSettings>,
}
impl TaskAddParameter {
    pub fn new(id: String, command_line: String) -> Self {
        Self {
            id,
            display_name: None,
            command_line,
            container_settings: None,
            exit_conditions: None,
            resource_files: Vec::new(),
            output_files: Vec::new(),
            environment_settings: Vec::new(),
            affinity_info: None,
            constraints: None,
            user_identity: None,
            multi_instance_settings: None,
            depends_on: None,
            application_package_references: Vec::new(),
            authentication_token_settings: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskAddResult {
    #[serde(deserialize_with = "case_insensitive_deserialize")]
    pub status: task_add_result::Status,
    #[serde(rename = "taskId")]
    pub task_id: String,
    #[doc = "You can use this to detect whether the Task has changed between requests. In particular, you can be pass the ETag with an Update Task request to specify that your changes should take effect only if nobody else has modified the Job in the meantime."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "lastModified", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<BatchError>,
}
impl TaskAddResult {
    pub fn new(status: task_add_result::Status, task_id: String) -> Self {
        Self {
            status,
            task_id,
            e_tag: None,
            last_modified: None,
            location: None,
            error: None,
        }
    }
}
pub mod task_add_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "clienterror")]
        Clienterror,
        #[serde(rename = "servererror")]
        Servererror,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskConstraints {
    #[doc = "If this is not specified, there is no time limit on how long the Task may run."]
    #[serde(rename = "maxWallClockTime", default, skip_serializing_if = "Option::is_none")]
    pub max_wall_clock_time: Option<String>,
    #[doc = "The default is 7 days, i.e. the Task directory will be retained for 7 days unless the Compute Node is removed or the Job is deleted."]
    #[serde(rename = "retentionTime", default, skip_serializing_if = "Option::is_none")]
    pub retention_time: Option<String>,
    #[doc = "Note that this value specifically controls the number of retries for the Task executable due to a nonzero exit code. The Batch service will try the Task once, and may then retry up to this limit. For example, if the maximum retry count is 3, Batch tries the Task up to 4 times (one initial try and 3 retries). If the maximum retry count is 0, the Batch service does not retry the Task after the first attempt. If the maximum retry count is -1, the Batch service retries the Task without limit."]
    #[serde(rename = "maxTaskRetryCount", default, skip_serializing_if = "Option::is_none")]
    pub max_task_retry_count: Option<i32>,
}
impl TaskConstraints {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskContainerExecutionInformation {
    #[serde(rename = "containerId", default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[doc = "This is the state of the container according to the Docker service. It is equivalent to the status field returned by \"docker inspect\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "This is the detailed error string from the Docker service, if available. It is equivalent to the error field returned by \"docker inspect\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
impl TaskContainerExecutionInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskContainerSettings {
    #[doc = "These additional options are supplied as arguments to the \"docker create\" command, in addition to those controlled by the Batch Service."]
    #[serde(rename = "containerRunOptions", default, skip_serializing_if = "Option::is_none")]
    pub container_run_options: Option<String>,
    #[doc = "This is the full Image reference, as would be specified to \"docker pull\". If no tag is provided as part of the Image name, the tag \":latest\" is used as a default."]
    #[serde(rename = "imageName")]
    pub image_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<ContainerRegistry>,
    #[doc = "The default is 'taskWorkingDirectory'."]
    #[serde(
        rename = "workingDirectory",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub working_directory: Option<task_container_settings::WorkingDirectory>,
}
impl TaskContainerSettings {
    pub fn new(image_name: String) -> Self {
        Self {
            container_run_options: None,
            image_name,
            registry: None,
            working_directory: None,
        }
    }
}
pub mod task_container_settings {
    use super::*;
    #[doc = "The default is 'taskWorkingDirectory'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum WorkingDirectory {
        #[serde(rename = "taskWorkingDirectory", alias = "taskworkingdirectory")]
        TaskWorkingDirectory,
        #[serde(rename = "containerImageDefault", alias = "containerimagedefault")]
        ContainerImageDefault,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskCounts {
    pub active: i32,
    pub running: i32,
    pub completed: i32,
    pub succeeded: i32,
    pub failed: i32,
}
impl TaskCounts {
    pub fn new(active: i32, running: i32, completed: i32, succeeded: i32, failed: i32) -> Self {
        Self {
            active,
            running,
            completed,
            succeeded,
            failed,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskDependencies {
    #[doc = "The taskIds collection is limited to 64000 characters total (i.e. the combined length of all Task IDs). If the taskIds collection exceeds the maximum length, the Add Task request fails with error code TaskDependencyListTooLong. In this case consider using Task ID ranges instead."]
    #[serde(rename = "taskIds", default, skip_serializing_if = "Vec::is_empty")]
    pub task_ids: Vec<String>,
    #[serde(rename = "taskIdRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub task_id_ranges: Vec<TaskIdRange>,
}
impl TaskDependencies {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskExecutionInformation {
    #[doc = "'Running' corresponds to the running state, so if the Task specifies resource files or Packages, then the start time reflects the time at which the Task started downloading or deploying these. If the Task has been restarted or retried, this is the most recent time at which the Task started running. This property is present only for Tasks that are in the running or completed state."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "This property is set only if the Task is in the Completed state."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "This property is set only if the Task is in the completed state. In general, the exit code for a process reflects the specific convention implemented by the application developer for that process. If you use the exit code value to make decisions in your code, be sure that you know the exit code convention used by the application process. However, if the Batch service terminates the Task (due to timeout, or user termination via the API) you may see an operating system-defined exit code."]
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "containerInfo", default, skip_serializing_if = "Option::is_none")]
    pub container_info: Option<TaskContainerExecutionInformation>,
    #[serde(rename = "failureInfo", default, skip_serializing_if = "Option::is_none")]
    pub failure_info: Option<TaskFailureInformation>,
    #[doc = "Task application failures (non-zero exit code) are retried, pre-processing errors (the Task could not be run) and file upload errors are not retried. The Batch service will retry the Task up to the limit specified by the constraints."]
    #[serde(rename = "retryCount")]
    pub retry_count: i32,
    #[doc = "This element is present only if the Task was retried (i.e. retryCount is nonzero). If present, this is typically the same as startTime, but may be different if the Task has been restarted for reasons other than retry; for example, if the Compute Node was rebooted during a retry, then the startTime is updated but the lastRetryTime is not."]
    #[serde(rename = "lastRetryTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_retry_time: Option<time::OffsetDateTime>,
    #[doc = "When the user removes Compute Nodes from a Pool (by resizing/shrinking the pool) or when the Job is being disabled, the user can specify that running Tasks on the Compute Nodes be requeued for execution. This count tracks how many times the Task has been requeued for these reasons."]
    #[serde(rename = "requeueCount")]
    pub requeue_count: i32,
    #[doc = "This property is set only if the requeueCount is nonzero."]
    #[serde(rename = "lastRequeueTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_requeue_time: Option<time::OffsetDateTime>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub result: Option<TaskExecutionResult>,
}
impl TaskExecutionInformation {
    pub fn new(retry_count: i32, requeue_count: i32) -> Self {
        Self {
            start_time: None,
            end_time: None,
            exit_code: None,
            container_info: None,
            failure_info: None,
            retry_count,
            last_retry_time: None,
            requeue_count,
            last_requeue_time: None,
            result: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TaskExecutionResult {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskFailureInformation {
    #[serde(deserialize_with = "case_insensitive_deserialize")]
    pub category: ErrorCategory,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<NameValuePair>,
}
impl TaskFailureInformation {
    pub fn new(category: ErrorCategory) -> Self {
        Self {
            category,
            code: None,
            message: None,
            details: Vec::new(),
        }
    }
}
#[doc = "The start and end of the range are inclusive. For example, if a range has start 9 and end 12, then it represents Tasks '9', '10', '11' and '12'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskIdRange {
    pub start: i32,
    pub end: i32,
}
impl TaskIdRange {
    pub fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskInformation {
    #[serde(rename = "taskUrl", default, skip_serializing_if = "Option::is_none")]
    pub task_url: Option<String>,
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "taskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "subtaskId", default, skip_serializing_if = "Option::is_none")]
    pub subtask_id: Option<i32>,
    #[serde(rename = "taskState", deserialize_with = "case_insensitive_deserialize")]
    pub task_state: TaskState,
    #[serde(rename = "executionInfo", default, skip_serializing_if = "Option::is_none")]
    pub execution_info: Option<TaskExecutionInformation>,
}
impl TaskInformation {
    pub fn new(task_state: TaskState) -> Self {
        Self {
            task_url: None,
            job_id: None,
            task_id: None,
            subtask_id: None,
            task_state,
            execution_info: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSchedulingPolicy {
    #[doc = "If not specified, the default is spread."]
    #[serde(rename = "nodeFillType", deserialize_with = "case_insensitive_deserialize")]
    pub node_fill_type: task_scheduling_policy::NodeFillType,
}
impl TaskSchedulingPolicy {
    pub fn new(node_fill_type: task_scheduling_policy::NodeFillType) -> Self {
        Self { node_fill_type }
    }
}
pub mod task_scheduling_policy {
    use super::*;
    #[doc = "If not specified, the default is spread."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NodeFillType {
        #[serde(rename = "spread")]
        Spread,
        #[serde(rename = "pack")]
        Pack,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TaskState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "preparing")]
    Preparing,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskStatistics {
    pub url: String,
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[serde(rename = "lastUpdateTime", with = "azure_core::date::rfc3339")]
    pub last_update_time: time::OffsetDateTime,
    #[serde(rename = "userCPUTime")]
    pub user_cpu_time: String,
    #[serde(rename = "kernelCPUTime")]
    pub kernel_cpu_time: String,
    #[doc = "The wall clock time is the elapsed time from when the Task started running on a Compute Node to when it finished (or to the last time the statistics were updated, if the Task had not finished by then). If the Task was retried, this includes the wall clock time of all the Task retries."]
    #[serde(rename = "wallClockTime")]
    pub wall_clock_time: String,
    #[serde(rename = "readIOps")]
    pub read_i_ops: i64,
    #[serde(rename = "writeIOps")]
    pub write_i_ops: i64,
    #[serde(rename = "readIOGiB")]
    pub read_io_gi_b: f64,
    #[serde(rename = "writeIOGiB")]
    pub write_io_gi_b: f64,
    #[serde(rename = "waitTime")]
    pub wait_time: String,
}
impl TaskStatistics {
    pub fn new(
        url: String,
        start_time: time::OffsetDateTime,
        last_update_time: time::OffsetDateTime,
        user_cpu_time: String,
        kernel_cpu_time: String,
        wall_clock_time: String,
        read_i_ops: i64,
        write_i_ops: i64,
        read_io_gi_b: f64,
        write_io_gi_b: f64,
        wait_time: String,
    ) -> Self {
        Self {
            url,
            start_time,
            last_update_time,
            user_cpu_time,
            kernel_cpu_time,
            wall_clock_time,
            read_i_ops,
            write_i_ops,
            read_io_gi_b,
            write_io_gi_b,
            wait_time,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskUpdateParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<TaskConstraints>,
}
impl TaskUpdateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UploadBatchServiceLogsConfiguration {
    #[doc = "The URL must include a Shared Access Signature (SAS) granting write permissions to the container. The SAS duration must allow enough time for the upload to finish. The start time for SAS is optional and recommended to not be specified."]
    #[serde(rename = "containerUrl")]
    pub container_url: String,
    #[doc = "Any log file containing a log message in the time range will be uploaded. This means that the operation might retrieve more logs than have been requested since the entire log file is always uploaded, but the operation should not retrieve fewer logs than have been requested."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "Any log file containing a log message in the time range will be uploaded. This means that the operation might retrieve more logs than have been requested since the entire log file is always uploaded, but the operation should not retrieve fewer logs than have been requested. If omitted, the default is to upload all logs available after the startTime."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}
impl UploadBatchServiceLogsConfiguration {
    pub fn new(container_url: String, start_time: time::OffsetDateTime) -> Self {
        Self {
            container_url,
            start_time,
            end_time: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UploadBatchServiceLogsResult {
    #[doc = "The virtual directory name is part of the blob name for each log file uploaded, and it is built based poolId, nodeId and a unique identifier."]
    #[serde(rename = "virtualDirectoryName")]
    pub virtual_directory_name: String,
    #[serde(rename = "numberOfFilesUploaded")]
    pub number_of_files_uploaded: i32,
}
impl UploadBatchServiceLogsResult {
    pub fn new(virtual_directory_name: String, number_of_files_uploaded: i32) -> Self {
        Self {
            virtual_directory_name,
            number_of_files_uploaded,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageStatistics {
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[serde(rename = "lastUpdateTime", with = "azure_core::date::rfc3339")]
    pub last_update_time: time::OffsetDateTime,
    #[serde(rename = "dedicatedCoreTime")]
    pub dedicated_core_time: String,
}
impl UsageStatistics {
    pub fn new(start_time: time::OffsetDateTime, last_update_time: time::OffsetDateTime, dedicated_core_time: String) -> Self {
        Self {
            start_time,
            last_update_time,
            dedicated_core_time,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAccount {
    pub name: String,
    pub password: String,
    #[serde(
        rename = "elevationLevel",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub elevation_level: Option<ElevationLevel>,
    #[serde(rename = "linuxUserConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub linux_user_configuration: Option<LinuxUserConfiguration>,
    #[serde(rename = "windowsUserConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub windows_user_configuration: Option<WindowsUserConfiguration>,
}
impl UserAccount {
    pub fn new(name: String, password: String) -> Self {
        Self {
            name,
            password,
            elevation_level: None,
            linux_user_configuration: None,
            windows_user_configuration: None,
        }
    }
}
#[doc = "Specify either the userName or autoUser property, but not both."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserIdentity {
    #[doc = "The userName and autoUser properties are mutually exclusive; you must specify one but not both."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "autoUser", default, skip_serializing_if = "Option::is_none")]
    pub auto_user: Option<AutoUserSpecification>,
}
impl UserIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineConfiguration {
    #[serde(rename = "imageReference")]
    pub image_reference: ImageReference,
    #[doc = "The Batch Compute Node agent is a program that runs on each Compute Node in the Pool, and provides the command-and-control interface between the Compute Node and the Batch service. There are different implementations of the Compute Node agent, known as SKUs, for different operating systems. You must specify a Compute Node agent SKU which matches the selected Image reference. To get the list of supported Compute Node agent SKUs along with their list of verified Image references, see the 'List supported Compute Node agent SKUs' operation."]
    #[serde(rename = "nodeAgentSKUId")]
    pub node_agent_sku_id: String,
    #[serde(rename = "windowsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<WindowsConfiguration>,
    #[doc = "This property must be specified if the Compute Nodes in the Pool need to have empty data disks attached to them. This cannot be updated. Each Compute Node gets its own disk (the disk is not a file share). Existing disks cannot be attached, each attached disk is empty. When the Compute Node is removed from the Pool, the disk and all data associated with it is also deleted. The disk is not formatted after being attached, it must be formatted before use - for more information see https://docs.microsoft.com/en-us/azure/virtual-machines/linux/classic/attach-disk#initialize-a-new-data-disk-in-linux and https://docs.microsoft.com/en-us/azure/virtual-machines/windows/attach-disk-ps#add-an-empty-data-disk-to-a-virtual-machine."]
    #[serde(rename = "dataDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disks: Vec<DataDisk>,
    #[doc = "This only applies to Images that contain the Windows operating system, and should only be used when you hold valid on-premises licenses for the Compute Nodes which will be deployed. If omitted, no on-premises licensing discount is applied. Values are:\n\n Windows_Server - The on-premises license is for Windows Server.\n Windows_Client - The on-premises license is for Windows Client.\n"]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[serde(rename = "containerConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub container_configuration: Option<ContainerConfiguration>,
    #[doc = "The disk encryption configuration applied on compute nodes in the pool. Disk encryption configuration is not supported on Linux pool created with Shared Image Gallery Image."]
    #[serde(rename = "diskEncryptionConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_configuration: Option<DiskEncryptionConfiguration>,
}
impl VirtualMachineConfiguration {
    pub fn new(image_reference: ImageReference, node_agent_sku_id: String) -> Self {
        Self {
            image_reference,
            node_agent_sku_id,
            windows_configuration: None,
            data_disks: Vec::new(),
            license_type: None,
            container_configuration: None,
            disk_encryption_configuration: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsConfiguration {
    #[doc = "If omitted, the default value is true."]
    #[serde(rename = "enableAutomaticUpdates", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_updates: Option<bool>,
}
impl WindowsConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsUserConfiguration {
    #[doc = "The default value for VirtualMachineConfiguration Pools is 'batch' and for CloudServiceConfiguration Pools is 'interactive'."]
    #[serde(
        rename = "loginMode",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "case_insensitive_deserialize"
    )]
    pub login_mode: Option<windows_user_configuration::LoginMode>,
}
impl WindowsUserConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod windows_user_configuration {
    use super::*;
    #[doc = "The default value for VirtualMachineConfiguration Pools is 'batch' and for CloudServiceConfiguration Pools is 'interactive'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LoginMode {
        #[serde(rename = "batch")]
        Batch,
        #[serde(rename = "interactive")]
        Interactive,
    }
}

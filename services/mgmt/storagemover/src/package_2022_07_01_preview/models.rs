#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The Agent resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Agent {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: AgentProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Agent {
    pub fn new(properties: AgentProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "List of Agents."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Agent>,
    #[doc = "Request URL that can be used to query next page of containers. Returned when total number of requested containers exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AgentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AgentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentProperties {
    #[doc = "A description for the Agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Agent version."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "The fully qualified resource ID of the Hybrid Compute resource for the Agent."]
    #[serde(rename = "arcResourceId")]
    pub arc_resource_id: String,
    #[doc = "The VM UUID of the Hybrid Compute resource for the Agent."]
    #[serde(rename = "arcVmUuid")]
    pub arc_vm_uuid: String,
    #[doc = "The Agent status."]
    #[serde(rename = "agentStatus", default, skip_serializing_if = "Option::is_none")]
    pub agent_status: Option<agent_properties::AgentStatus>,
    #[doc = "The last updated time of the Agent status."]
    #[serde(rename = "lastStatusUpdate", default, with = "azure_core::date::rfc3339::option")]
    pub last_status_update: Option<time::OffsetDateTime>,
    #[doc = "Local IP address reported by the Agent."]
    #[serde(rename = "localIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub local_ip_address: Option<String>,
    #[doc = "Available memory reported by the Agent, in MB."]
    #[serde(rename = "memoryInMB", default, skip_serializing_if = "Option::is_none")]
    pub memory_in_mb: Option<i64>,
    #[doc = "Available compute cores reported by the Agent."]
    #[serde(rename = "numberOfCores", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores: Option<i64>,
    #[doc = "Uptime of the Agent in seconds."]
    #[serde(rename = "uptimeInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub uptime_in_seconds: Option<i64>,
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<agent_properties::ErrorDetails>,
    #[doc = "The provisioning state of this resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<agent_properties::ProvisioningState>,
}
impl AgentProperties {
    pub fn new(arc_resource_id: String, arc_vm_uuid: String) -> Self {
        Self {
            description: None,
            agent_version: None,
            arc_resource_id,
            arc_vm_uuid,
            agent_status: None,
            last_status_update: None,
            local_ip_address: None,
            memory_in_mb: None,
            number_of_cores: None,
            uptime_in_seconds: None,
            error_details: None,
            provisioning_state: None,
        }
    }
}
pub mod agent_properties {
    use super::*;
    #[doc = "The Agent status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AgentStatus")]
    pub enum AgentStatus {
        Registering,
        Offline,
        Online,
        Executing,
        RequiresAttention,
        Unregistering,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AgentStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AgentStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AgentStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Registering => serializer.serialize_unit_variant("AgentStatus", 0u32, "Registering"),
                Self::Offline => serializer.serialize_unit_variant("AgentStatus", 1u32, "Offline"),
                Self::Online => serializer.serialize_unit_variant("AgentStatus", 2u32, "Online"),
                Self::Executing => serializer.serialize_unit_variant("AgentStatus", 3u32, "Executing"),
                Self::RequiresAttention => serializer.serialize_unit_variant("AgentStatus", 4u32, "RequiresAttention"),
                Self::Unregistering => serializer.serialize_unit_variant("AgentStatus", 5u32, "Unregistering"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ErrorDetails {
        #[doc = "Error code reported by Agent"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Expanded description of reported error code"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl ErrorDetails {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The provisioning state of this resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Agent resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AgentUpdateProperties>,
}
impl AgentUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentUpdateProperties {
    #[doc = "A description for the Agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AgentUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageBlobContainerEndpointProperties {
    #[serde(flatten)]
    pub endpoint_base_properties: EndpointBaseProperties,
    #[doc = "The Azure Resource ID of the storage account that is the target destination."]
    #[serde(rename = "storageAccountResourceId")]
    pub storage_account_resource_id: String,
    #[doc = "The name of the Storage blob container that is the target destination."]
    #[serde(rename = "blobContainerName")]
    pub blob_container_name: String,
}
impl AzureStorageBlobContainerEndpointProperties {
    pub fn new(endpoint_base_properties: EndpointBaseProperties, storage_account_resource_id: String, blob_container_name: String) -> Self {
        Self {
            endpoint_base_properties,
            storage_account_resource_id,
            blob_container_name,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureStorageBlobContainerEndpointUpdateProperties {
    #[serde(flatten)]
    pub endpoint_base_update_properties: EndpointBaseUpdateProperties,
}
impl AzureStorageBlobContainerEndpointUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Endpoint resource, which contains information about file sources and targets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Endpoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The resource specific properties for the Storage Mover resource."]
    pub properties: EndpointBaseProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Endpoint {
    pub fn new(properties: EndpointBaseProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "The resource specific properties for the Storage Mover resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointBaseProperties {
    #[doc = "The Endpoint resource type."]
    #[serde(rename = "endpointType")]
    pub endpoint_type: endpoint_base_properties::EndpointType,
    #[doc = "A description for the Endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The provisioning state of this resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<endpoint_base_properties::ProvisioningState>,
}
impl EndpointBaseProperties {
    pub fn new(endpoint_type: endpoint_base_properties::EndpointType) -> Self {
        Self {
            endpoint_type,
            description: None,
            provisioning_state: None,
        }
    }
}
pub mod endpoint_base_properties {
    use super::*;
    #[doc = "The Endpoint resource type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EndpointType")]
    pub enum EndpointType {
        AzureStorageBlobContainer,
        NfsMount,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EndpointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EndpointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EndpointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureStorageBlobContainer => serializer.serialize_unit_variant("EndpointType", 0u32, "AzureStorageBlobContainer"),
                Self::NfsMount => serializer.serialize_unit_variant("EndpointType", 1u32, "NfsMount"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of this resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointBaseUpdateParameters {
    #[doc = "The Endpoint resource, which contains information about file sources and targets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EndpointBaseUpdateProperties>,
}
impl EndpointBaseUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Endpoint resource, which contains information about file sources and targets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointBaseUpdateProperties {
    #[doc = "A description for the Endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl EndpointBaseUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Endpoints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Endpoint>,
    #[doc = "Request URL that can be used to query next page of containers. Returned when total number of requested containers exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EndpointList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EndpointList {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "The Job Definition resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDefinition {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Job definition properties."]
    pub properties: JobDefinitionProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl JobDefinition {
    pub fn new(properties: JobDefinitionProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "List of Job Definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobDefinitionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobDefinition>,
    #[doc = "Request URL that can be used to query next page of containers. Returned when total number of requested containers exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobDefinitionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobDefinitionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job definition properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDefinitionProperties {
    #[doc = "A description for the Job Definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Strategy to use for copy."]
    #[serde(rename = "copyMode")]
    pub copy_mode: job_definition_properties::CopyMode,
    #[doc = "The name of the source Endpoint."]
    #[serde(rename = "sourceName")]
    pub source_name: String,
    #[doc = "Fully qualified resource ID of the source Endpoint."]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
    #[doc = "The subpath to use when reading from the source Endpoint."]
    #[serde(rename = "sourceSubpath", default, skip_serializing_if = "Option::is_none")]
    pub source_subpath: Option<String>,
    #[doc = "The name of the target Endpoint."]
    #[serde(rename = "targetName")]
    pub target_name: String,
    #[doc = "Fully qualified resource ID of the target Endpoint."]
    #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
    #[doc = "The subpath to use when writing to the target Endpoint."]
    #[serde(rename = "targetSubpath", default, skip_serializing_if = "Option::is_none")]
    pub target_subpath: Option<String>,
    #[doc = "The name of the Job Run in a non-terminal state, if exists."]
    #[serde(rename = "latestJobRunName", default, skip_serializing_if = "Option::is_none")]
    pub latest_job_run_name: Option<String>,
    #[doc = "The fully qualified resource ID of the Job Run in a non-terminal state, if exists."]
    #[serde(rename = "latestJobRunResourceId", default, skip_serializing_if = "Option::is_none")]
    pub latest_job_run_resource_id: Option<String>,
    #[doc = "The current status of the Job Run in a non-terminal state, if exists."]
    #[serde(rename = "latestJobRunStatus", default, skip_serializing_if = "Option::is_none")]
    pub latest_job_run_status: Option<job_definition_properties::LatestJobRunStatus>,
    #[doc = "Name of the Agent to assign for new Job Runs of this Job Definition."]
    #[serde(rename = "agentName", default, skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[doc = "Fully qualified resource id of the Agent to assign for new Job Runs of this Job Definition."]
    #[serde(rename = "agentResourceId", default, skip_serializing_if = "Option::is_none")]
    pub agent_resource_id: Option<String>,
    #[doc = "The provisioning state of this resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<job_definition_properties::ProvisioningState>,
}
impl JobDefinitionProperties {
    pub fn new(copy_mode: job_definition_properties::CopyMode, source_name: String, target_name: String) -> Self {
        Self {
            description: None,
            copy_mode,
            source_name,
            source_resource_id: None,
            source_subpath: None,
            target_name,
            target_resource_id: None,
            target_subpath: None,
            latest_job_run_name: None,
            latest_job_run_resource_id: None,
            latest_job_run_status: None,
            agent_name: None,
            agent_resource_id: None,
            provisioning_state: None,
        }
    }
}
pub mod job_definition_properties {
    use super::*;
    #[doc = "Strategy to use for copy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CopyMode")]
    pub enum CopyMode {
        Additive,
        Mirror,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CopyMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CopyMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CopyMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Additive => serializer.serialize_unit_variant("CopyMode", 0u32, "Additive"),
                Self::Mirror => serializer.serialize_unit_variant("CopyMode", 1u32, "Mirror"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The current status of the Job Run in a non-terminal state, if exists."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LatestJobRunStatus")]
    pub enum LatestJobRunStatus {
        Queued,
        Started,
        Running,
        CancelRequested,
        Canceling,
        Canceled,
        Failed,
        Succeeded,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LatestJobRunStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LatestJobRunStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LatestJobRunStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Queued => serializer.serialize_unit_variant("LatestJobRunStatus", 0u32, "Queued"),
                Self::Started => serializer.serialize_unit_variant("LatestJobRunStatus", 1u32, "Started"),
                Self::Running => serializer.serialize_unit_variant("LatestJobRunStatus", 2u32, "Running"),
                Self::CancelRequested => serializer.serialize_unit_variant("LatestJobRunStatus", 3u32, "CancelRequested"),
                Self::Canceling => serializer.serialize_unit_variant("LatestJobRunStatus", 4u32, "Canceling"),
                Self::Canceled => serializer.serialize_unit_variant("LatestJobRunStatus", 5u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("LatestJobRunStatus", 6u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("LatestJobRunStatus", 7u32, "Succeeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of this resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Job Definition resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobDefinitionUpdateParameters {
    #[doc = "Job definition properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobDefinitionUpdateProperties>,
}
impl JobDefinitionUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job definition properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobDefinitionUpdateProperties {
    #[doc = "A description for the Job Definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Strategy to use for copy."]
    #[serde(rename = "copyMode", default, skip_serializing_if = "Option::is_none")]
    pub copy_mode: Option<job_definition_update_properties::CopyMode>,
    #[doc = "Name of the Agent to assign for new Job Runs of this Job Definition."]
    #[serde(rename = "agentName", default, skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
}
impl JobDefinitionUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_definition_update_properties {
    use super::*;
    #[doc = "Strategy to use for copy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CopyMode")]
    pub enum CopyMode {
        Additive,
        Mirror,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CopyMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CopyMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CopyMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Additive => serializer.serialize_unit_variant("CopyMode", 0u32, "Additive"),
                Self::Mirror => serializer.serialize_unit_variant("CopyMode", 1u32, "Mirror"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Job Run resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobRun {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Job run properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobRunProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl JobRun {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobRunError {
    #[doc = "Error code of the given entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message of the given entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Target of the given error entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl JobRunError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Job Runs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobRunList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobRun>,
    #[doc = "Request URL that can be used to query next page of containers. Returned when total number of requested containers exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobRunList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobRunList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job run properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobRunProperties {
    #[doc = "The state of the job execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<job_run_properties::Status>,
    #[doc = "The status of Agent's scanning of source."]
    #[serde(rename = "scanStatus", default, skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<job_run_properties::ScanStatus>,
    #[doc = "Name of the Agent assigned to this run."]
    #[serde(rename = "agentName", default, skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[doc = "Fully qualified resource id of the Agent assigned to this run."]
    #[serde(rename = "agentResourceId", default, skip_serializing_if = "Option::is_none")]
    pub agent_resource_id: Option<String>,
    #[doc = "Start time of the run. Null if no Agent reported that the job has started."]
    #[serde(rename = "executionStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub execution_start_time: Option<time::OffsetDateTime>,
    #[doc = "End time of the run. Null if Agent has not reported that the job has ended."]
    #[serde(rename = "executionEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub execution_end_time: Option<time::OffsetDateTime>,
    #[doc = "The last updated time of the Job Run."]
    #[serde(rename = "lastStatusUpdate", default, with = "azure_core::date::rfc3339::option")]
    pub last_status_update: Option<time::OffsetDateTime>,
    #[doc = "Number of items scanned so far in source."]
    #[serde(rename = "itemsScanned", default, skip_serializing_if = "Option::is_none")]
    pub items_scanned: Option<i64>,
    #[doc = "Number of items that will not be transferred, as they are excluded by user configuration."]
    #[serde(rename = "itemsExcluded", default, skip_serializing_if = "Option::is_none")]
    pub items_excluded: Option<i64>,
    #[doc = "Number of items that will not be transferred, as they are unsupported on target."]
    #[serde(rename = "itemsUnsupported", default, skip_serializing_if = "Option::is_none")]
    pub items_unsupported: Option<i64>,
    #[doc = "Number of items that will not be transferred, as they are already found on target (e.g. mirror mode)."]
    #[serde(rename = "itemsNoTransferNeeded", default, skip_serializing_if = "Option::is_none")]
    pub items_no_transfer_needed: Option<i64>,
    #[doc = "Number of items that were attempted to transfer and failed."]
    #[serde(rename = "itemsFailed", default, skip_serializing_if = "Option::is_none")]
    pub items_failed: Option<i64>,
    #[doc = "Number of items successfully transferred to target."]
    #[serde(rename = "itemsTransferred", default, skip_serializing_if = "Option::is_none")]
    pub items_transferred: Option<i64>,
    #[doc = "Bytes of data scanned so far in source."]
    #[serde(rename = "bytesScanned", default, skip_serializing_if = "Option::is_none")]
    pub bytes_scanned: Option<i64>,
    #[doc = "Bytes of data that will not be transferred, as they are excluded by user configuration."]
    #[serde(rename = "bytesExcluded", default, skip_serializing_if = "Option::is_none")]
    pub bytes_excluded: Option<i64>,
    #[doc = "Bytes of data that will not be transferred, as they are unsupported on target."]
    #[serde(rename = "bytesUnsupported", default, skip_serializing_if = "Option::is_none")]
    pub bytes_unsupported: Option<i64>,
    #[doc = "Bytes of data that will not be transferred, as they are already found on target (e.g. mirror mode)."]
    #[serde(rename = "bytesNoTransferNeeded", default, skip_serializing_if = "Option::is_none")]
    pub bytes_no_transfer_needed: Option<i64>,
    #[doc = "Bytes of data that were attempted to transfer and failed."]
    #[serde(rename = "bytesFailed", default, skip_serializing_if = "Option::is_none")]
    pub bytes_failed: Option<i64>,
    #[doc = "Bytes of data successfully transferred to target."]
    #[serde(rename = "bytesTransferred", default, skip_serializing_if = "Option::is_none")]
    pub bytes_transferred: Option<i64>,
    #[doc = "Name of source Endpoint resource. This resource may no longer exist."]
    #[serde(rename = "sourceName", default, skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[doc = "Fully qualified resource id of source Endpoint. This id may no longer exist."]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
    #[doc = "Copy of source Endpoint resource's properties at time of Job Run creation."]
    #[serde(rename = "sourceProperties", default, skip_serializing_if = "Option::is_none")]
    pub source_properties: Option<serde_json::Value>,
    #[doc = "Name of target Endpoint resource. This resource may no longer exist."]
    #[serde(rename = "targetName", default, skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,
    #[doc = "Fully qualified resource id of of Endpoint. This id may no longer exist."]
    #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
    #[doc = "Copy of Endpoint resource's properties at time of Job Run creation."]
    #[serde(rename = "targetProperties", default, skip_serializing_if = "Option::is_none")]
    pub target_properties: Option<serde_json::Value>,
    #[doc = "Copy of parent Job Definition's properties at time of Job Run creation."]
    #[serde(rename = "jobDefinitionProperties", default, skip_serializing_if = "Option::is_none")]
    pub job_definition_properties: Option<serde_json::Value>,
    #[doc = "Error type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<JobRunError>,
    #[doc = "The provisioning state of this resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<job_run_properties::ProvisioningState>,
}
impl JobRunProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_run_properties {
    use super::*;
    #[doc = "The state of the job execution."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Queued,
        Started,
        Running,
        CancelRequested,
        Canceling,
        Canceled,
        Failed,
        Succeeded,
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
                Self::Queued => serializer.serialize_unit_variant("Status", 0u32, "Queued"),
                Self::Started => serializer.serialize_unit_variant("Status", 1u32, "Started"),
                Self::Running => serializer.serialize_unit_variant("Status", 2u32, "Running"),
                Self::CancelRequested => serializer.serialize_unit_variant("Status", 3u32, "CancelRequested"),
                Self::Canceling => serializer.serialize_unit_variant("Status", 4u32, "Canceling"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 5u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("Status", 6u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 7u32, "Succeeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of Agent's scanning of source."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScanStatus")]
    pub enum ScanStatus {
        NotStarted,
        Scanning,
        Completed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScanStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScanStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScanStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotStarted => serializer.serialize_unit_variant("ScanStatus", 0u32, "NotStarted"),
                Self::Scanning => serializer.serialize_unit_variant("ScanStatus", 1u32, "Scanning"),
                Self::Completed => serializer.serialize_unit_variant("ScanStatus", 2u32, "Completed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of this resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Response that identifies a Job Run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobRunResourceId {
    #[doc = "Fully qualified resource id of the Job Run."]
    #[serde(rename = "jobRunResourceId", default, skip_serializing_if = "Option::is_none")]
    pub job_run_resource_id: Option<String>,
}
impl JobRunResourceId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NfsMountEndpointProperties {
    #[serde(flatten)]
    pub endpoint_base_properties: EndpointBaseProperties,
    #[doc = "The host name or IP address of the server exporting the file system."]
    pub host: String,
    #[doc = "The NFS protocol version."]
    #[serde(rename = "nfsVersion", default, skip_serializing_if = "Option::is_none")]
    pub nfs_version: Option<nfs_mount_endpoint_properties::NfsVersion>,
    #[doc = "The directory being exported from the server."]
    pub export: String,
}
impl NfsMountEndpointProperties {
    pub fn new(endpoint_base_properties: EndpointBaseProperties, host: String, export: String) -> Self {
        Self {
            endpoint_base_properties,
            host,
            nfs_version: None,
            export,
        }
    }
}
pub mod nfs_mount_endpoint_properties {
    use super::*;
    #[doc = "The NFS protocol version."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NfsVersion")]
    pub enum NfsVersion {
        #[serde(rename = "NFSauto")]
        NfSauto,
        #[serde(rename = "NFSv3")]
        NfSv3,
        #[serde(rename = "NFSv4")]
        NfSv4,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NfsVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NfsVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NfsVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NfSauto => serializer.serialize_unit_variant("NfsVersion", 0u32, "NFSauto"),
                Self::NfSv3 => serializer.serialize_unit_variant("NfsVersion", 1u32, "NFSv3"),
                Self::NfSv4 => serializer.serialize_unit_variant("NfsVersion", 2u32, "NFSv4"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NfsMountEndpointUpdateProperties {
    #[serde(flatten)]
    pub endpoint_base_update_properties: EndpointBaseUpdateProperties,
}
impl NfsMountEndpointUpdateProperties {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Project resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Project {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Project properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProjectProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Project {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Project resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Project>,
    #[doc = "Request URL that can be used to query next page of containers. Returned when total number of requested containers exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProjectList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProjectList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Project properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectProperties {
    #[doc = "A description for the Project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The provisioning state of this resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<project_properties::ProvisioningState>,
}
impl ProjectProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod project_properties {
    use super::*;
    #[doc = "The provisioning state of this resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Project resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectUpdateParameters {
    #[doc = "Project properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProjectUpdateProperties>,
}
impl ProjectUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Project properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectUpdateProperties {
    #[doc = "A description for the Project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ProjectUpdateProperties {
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
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Storage Mover resource, which is a container for a group of Agents, Projects, and Endpoints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageMover {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The resource specific properties for the Storage Mover resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageMoverProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl StorageMover {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "List of Storage Movers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageMoverList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageMover>,
    #[doc = "Request URL that can be used to query next page of containers. Returned when total number of requested containers exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StorageMoverList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StorageMoverList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource specific properties for the Storage Mover resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageMoverProperties {
    #[doc = "A description for the Storage Mover."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The provisioning state of this resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<storage_mover_properties::ProvisioningState>,
}
impl StorageMoverProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_mover_properties {
    use super::*;
    #[doc = "The provisioning state of this resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Storage Mover resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageMoverUpdateParameters {
    #[doc = "The resource specific properties for the Storage Mover resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageMoverUpdateProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl StorageMoverUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource specific properties for the Storage Mover resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageMoverUpdateProperties {
    #[doc = "A description for the Storage Mover."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl StorageMoverUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
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

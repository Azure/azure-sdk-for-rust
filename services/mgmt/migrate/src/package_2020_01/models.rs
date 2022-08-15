#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentConfiguration {
    #[serde(rename = "agentId", default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "dependencyAgentId", default, skip_serializing_if = "Option::is_none")]
    pub dependency_agent_id: Option<String>,
    #[serde(rename = "dependencyAgentVersion", default, skip_serializing_if = "Option::is_none")]
    pub dependency_agent_version: Option<String>,
    #[serde(rename = "dependencyAgentRevision", default, skip_serializing_if = "Option::is_none")]
    pub dependency_agent_revision: Option<String>,
    #[serde(rename = "rebootStatus", default, skip_serializing_if = "Option::is_none")]
    pub reboot_status: Option<agent_configuration::RebootStatus>,
    #[serde(rename = "clockGranularity", default, skip_serializing_if = "Option::is_none")]
    pub clock_granularity: Option<i32>,
}
impl AgentConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod agent_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RebootStatus {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "rebooted")]
        Rebooted,
        #[serde(rename = "notRebooted")]
        NotRebooted,
    }
}
#[doc = "Application in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Application {
    #[doc = "Name of the Application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Version of the Application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Provider of the Application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
impl Application {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AppsAndRoles in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppsAndRoles {
    #[doc = "Applications of the AppsAndRoles."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applications: Vec<Application>,
    #[doc = "WebApplications of the AppsAndRoles."]
    #[serde(rename = "webApplications", default, skip_serializing_if = "Vec::is_empty")]
    pub web_applications: Vec<WebApplication>,
    #[doc = "Features of the AppsAndRoles."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<Feature>,
    #[doc = "SQLServers of the AppsAndRoles."]
    #[serde(rename = "sqlServers", default, skip_serializing_if = "Vec::is_empty")]
    pub sql_servers: Vec<SqlServer>,
    #[doc = "SharePointServers of the AppsAndRoles."]
    #[serde(rename = "sharePointServers", default, skip_serializing_if = "Vec::is_empty")]
    pub share_point_servers: Vec<SharePointServer>,
    #[doc = "SystemCenters of the AppsAndRoles."]
    #[serde(rename = "systemCenters", default, skip_serializing_if = "Vec::is_empty")]
    pub system_centers: Vec<SystemCenter>,
    #[doc = "BizTalkServers of the AppsAndRoles."]
    #[serde(rename = "bizTalkServers", default, skip_serializing_if = "Vec::is_empty")]
    pub biz_talk_servers: Vec<BizTalkServer>,
    #[doc = "ExchangeServers of the AppsAndRoles."]
    #[serde(rename = "exchangeServers", default, skip_serializing_if = "Vec::is_empty")]
    pub exchange_servers: Vec<ExchangeServer>,
    #[doc = "OtherDatabaseServers of the AppsAndRoles."]
    #[serde(rename = "otherDatabases", default, skip_serializing_if = "Vec::is_empty")]
    pub other_databases: Vec<OtherDatabase>,
}
impl AppsAndRoles {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BizTalkServer in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BizTalkServer {
    #[doc = "ProductName of the BizTalkServer."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "Status of the BizTalkServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl BizTalkServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error contract returned when some exception occurs in Rest API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Possible causes of error."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "Recommended action to resolve error."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "Error severity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "Value indicating whether the error originated from a agent or not."]
    #[serde(rename = "isAgentReportedError", default, skip_serializing_if = "Option::is_none")]
    pub is_agent_reported_error: Option<bool>,
    #[doc = "Agent error code."]
    #[serde(rename = "agentErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub agent_error_code: Option<String>,
    #[doc = "Error message from the agent."]
    #[serde(rename = "agentErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub agent_error_message: Option<String>,
    #[doc = "Possible causes for the agent error."]
    #[serde(rename = "agentErrorPossibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub agent_error_possible_causes: Option<String>,
    #[doc = "Recommended action for the agent error."]
    #[serde(rename = "agentErrorRecommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub agent_error_recommended_action: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ExchangeServer in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExchangeServer {
    #[doc = "ProductName of the ExchangeServer."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "Edition of the ExchangeServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[doc = "Roles of the ExchangeServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<String>,
    #[doc = "ServicePack of the ExchangeServer."]
    #[serde(rename = "servicePack", default, skip_serializing_if = "Option::is_none")]
    pub service_pack: Option<String>,
    #[doc = "Version of the ExchangeServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ExchangeServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Feature in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Feature {
    #[doc = "Name of the Feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "FeatureType of the Feature."]
    #[serde(rename = "featureType", default, skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<String>,
    #[doc = "Parent of the Feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[doc = "Status of the Feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl Feature {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data related to a machine's operating system. Serialized and stored as part of Machine Rest object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestOsDetails {
    #[doc = "Type of the operating system."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "Name of the operating system."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "Version of the operating system."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
}
impl GuestOsDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error contract returned when some exception occurs in Rest API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthErrorDetails {
    #[doc = "Error ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Error name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Possible causes of error."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "Recommended action to resolve error."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "Error severity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "Error summary message."]
    #[serde(rename = "summaryMessage", default, skip_serializing_if = "Option::is_none")]
    pub summary_message: Option<String>,
    #[doc = "Error source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Message parameters."]
    #[serde(rename = "messageParameters", default, skip_serializing_if = "Option::is_none")]
    pub message_parameters: Option<serde_json::Value>,
}
impl HealthErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostingConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
impl HostingConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cluster REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVCluster {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. Type = Microsoft.OffAzure/hyperVSites/clusters."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Class for cluster properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HyperVClusterProperties>,
}
impl HyperVCluster {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of Hyper-V clusters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVClusterCollection {
    #[doc = "List of clusters."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HyperVCluster>,
    #[doc = "Value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HyperVClusterCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HyperVClusterCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for cluster properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVClusterProperties {
    #[doc = "Timestamp marking Hyper-V cluster creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Timestamp marking last updated on the Hyper-V cluster."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "FQDN/IPAddress of the Hyper-V cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Functional level of the Hyper-V cluster."]
    #[serde(rename = "functionalLevel", default, skip_serializing_if = "Option::is_none")]
    pub functional_level: Option<i32>,
    #[doc = "Status of the Hyper-V cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Run as account ID of the Hyper-V cluster."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "List of hosts (FQDN) currently being tracked by the cluster."]
    #[serde(rename = "hostFqdnList", default, skip_serializing_if = "Vec::is_empty")]
    pub host_fqdn_list: Vec<String>,
    #[doc = "Errors for Hyper-V clusters."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<HealthErrorDetails>,
}
impl HyperVClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Second level object returned as part of Machine REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVDisk {
    #[doc = "Id of the disk."]
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[doc = "VHD Id of the disk."]
    #[serde(rename = "vhdId", default, skip_serializing_if = "Option::is_none")]
    pub vhd_id: Option<String>,
    #[doc = "Bytes allocated for the disk."]
    #[serde(rename = "maxSizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_in_bytes: Option<i64>,
    #[doc = "Name of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the disk."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
    #[doc = "LUN of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    #[doc = "Path of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl HyperVDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Host REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVHost {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. Type = Microsoft.OffAzure/hyperVSites/hosts."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Class for host properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HyperVHostProperties>,
}
impl HyperVHost {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of Hyper-V hosts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVHostCollection {
    #[doc = "List of hosts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HyperVHost>,
    #[doc = "Value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HyperVHostCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HyperVHostCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for host properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVHostProperties {
    #[doc = "Timestamp marking Hyper-V host creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Timestamp marking last updated on the Hyper-V host."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "FQDN/IPAddress of the Hyper-V host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Run as account ID of the Hyper-V host."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Version of the Hyper-V host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Errors for Hyper-V hosts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<HealthErrorDetails>,
}
impl HyperVHostProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVJob {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the Job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. Type = Microsoft.OffAzure/HyperVSites/Jobs."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Class for machine properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
}
impl HyperVJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of Hyper-V jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVJobCollection {
    #[doc = "List of jobs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HyperVJob>,
    #[doc = "Value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HyperVJobCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HyperVJobCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Machine REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVMachine {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the Sites."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. Type = Microsoft.OffAzure/HyperVSites/Machines."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Class for machine properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HyperVMachineProperties>,
}
impl HyperVMachine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of Hyper-V machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVMachineCollection {
    #[doc = "List of machines."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HyperVMachine>,
    #[doc = "Value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HyperVMachineCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HyperVMachineCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for machine properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVMachineProperties {
    #[doc = "Host FQDN/IPAddress."]
    #[serde(rename = "hostFqdn", default, skip_serializing_if = "Option::is_none")]
    pub host_fqdn: Option<String>,
    #[doc = "Host ARM ID."]
    #[serde(rename = "hostId", default, skip_serializing_if = "Option::is_none")]
    pub host_id: Option<String>,
    #[doc = "Cluster FQDN/IPAddress."]
    #[serde(rename = "clusterFqdn", default, skip_serializing_if = "Option::is_none")]
    pub cluster_fqdn: Option<String>,
    #[doc = "Cluster ARM ID."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "Management server type of the machine. It is either Host or Cluster."]
    #[serde(rename = "managementServerType", default, skip_serializing_if = "Option::is_none")]
    pub management_server_type: Option<String>,
    #[doc = "Generation of the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<i32>,
    #[doc = "VM version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Value indicating whether the VM is highly available."]
    #[serde(rename = "highAvailability", default, skip_serializing_if = "Option::is_none")]
    pub high_availability: Option<hyper_v_machine_properties::HighAvailability>,
    #[doc = "Max memory of the virtual machine in MB."]
    #[serde(rename = "maxMemoryMB", default, skip_serializing_if = "Option::is_none")]
    pub max_memory_mb: Option<i32>,
    #[doc = "Firmware of the machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    #[doc = "Value indicating whether dynamic memory is enabled for the VM."]
    #[serde(rename = "isDynamicMemoryEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_dynamic_memory_enabled: Option<bool>,
    #[doc = "Disks attached to the machine."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<HyperVDisk>,
    #[doc = "AppsAndRoles in the guest virtual machine."]
    #[serde(rename = "appsAndRoles", default, skip_serializing_if = "Option::is_none")]
    pub apps_and_roles: Option<AppsAndRoles>,
    #[doc = "Network adapters attached to the machine."]
    #[serde(rename = "networkAdapters", default, skip_serializing_if = "Vec::is_empty")]
    pub network_adapters: Vec<HyperVNetworkAdapter>,
    #[doc = "Display name of the machine."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Number of Processor Cores allocated for the machine."]
    #[serde(rename = "numberOfProcessorCore", default, skip_serializing_if = "Option::is_none")]
    pub number_of_processor_core: Option<i32>,
    #[doc = "Allocated Memory in MB."]
    #[serde(rename = "allocatedMemoryInMB", default, skip_serializing_if = "Option::is_none")]
    pub allocated_memory_in_mb: Option<f64>,
    #[doc = "Root location of the VM configuration file."]
    #[serde(rename = "vmConfigurationFileLocation", default, skip_serializing_if = "Option::is_none")]
    pub vm_configuration_file_location: Option<String>,
    #[doc = "Second level object returned as part of Machine REST resource."]
    #[serde(rename = "operatingSystemDetails", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_details: Option<OperatingSystem>,
    #[doc = "Data related to a machine's operating system. Serialized and stored as part of Machine Rest object."]
    #[serde(rename = "guestOSDetails", default, skip_serializing_if = "Option::is_none")]
    pub guest_os_details: Option<GuestOsDetails>,
    #[doc = "Number of applications installed in the guest VM."]
    #[serde(rename = "numberOfApplications", default, skip_serializing_if = "Option::is_none")]
    pub number_of_applications: Option<i32>,
    #[doc = "The last time at which the Guest Details of machine was discovered."]
    #[serde(rename = "guestDetailsDiscoveryTimestamp", with = "azure_core::date::rfc3339::option")]
    pub guest_details_discovery_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Whether Refresh Fabric Layout Guest Details has been completed once. Portal will show discovery in progress, if this value is true."]
    #[serde(rename = "isGuestDetailsDiscoveryInProgress", default, skip_serializing_if = "Option::is_none")]
    pub is_guest_details_discovery_in_progress: Option<bool>,
    #[doc = "Timestamp marking machine creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Timestamp marking last updated on the machine."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "On-premise Instance UUID of the machine."]
    #[serde(rename = "instanceUuid", default, skip_serializing_if = "Option::is_none")]
    pub instance_uuid: Option<String>,
    #[doc = "Machine power status."]
    #[serde(rename = "powerStatus", default, skip_serializing_if = "Option::is_none")]
    pub power_status: Option<String>,
    #[doc = "Machine BIOS serial number."]
    #[serde(rename = "biosSerialNumber", default, skip_serializing_if = "Option::is_none")]
    pub bios_serial_number: Option<String>,
    #[doc = "Machine BIOS GUID."]
    #[serde(rename = "biosGuid", default, skip_serializing_if = "Option::is_none")]
    pub bios_guid: Option<String>,
    #[doc = "Machine FQDN."]
    #[serde(rename = "vmFqdn", default, skip_serializing_if = "Option::is_none")]
    pub vm_fqdn: Option<String>,
    #[doc = "Value indicating whether VM is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Errors for machine."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<HealthErrorDetails>,
}
impl HyperVMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hyper_v_machine_properties {
    use super::*;
    #[doc = "Value indicating whether the VM is highly available."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HighAvailability {
        Unknown,
        No,
        Yes,
    }
}
#[doc = "Second level object represented in responses as part of Machine REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVNetworkAdapter {
    #[doc = "Network Id."]
    #[serde(rename = "networkId", default, skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[doc = "Name of the VM subnet within the virtual network the NIC is attached to."]
    #[serde(rename = "subnetName", default, skip_serializing_if = "Option::is_none")]
    pub subnet_name: Option<String>,
    #[doc = "Static IP address."]
    #[serde(rename = "staticIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub static_ip_address: Option<String>,
    #[doc = "Mac address of the NIC."]
    #[serde(rename = "nicType", default, skip_serializing_if = "Option::is_none")]
    pub nic_type: Option<String>,
    #[doc = "NIC Id."]
    #[serde(rename = "nicId", default, skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    #[doc = "Mac address of the NIC."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "IP addresses for the machine."]
    #[serde(rename = "ipAddressList", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_address_list: Vec<String>,
    #[doc = "Network Name."]
    #[serde(rename = "networkName", default, skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[doc = "Type of the IP address."]
    #[serde(rename = "ipAddressType", default, skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
}
impl HyperVNetworkAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Run as account REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVRunAsAccount {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the Sites."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. Type = Microsoft.OffAzure/HyperVSites/RunAsAccounts."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Class for run as account properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RunAsAccountProperties>,
}
impl HyperVRunAsAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of Hyper-V run as accounts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVRunAsAccountCollection {
    #[doc = "List of run as accounts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HyperVRunAsAccount>,
    #[doc = "Value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HyperVRunAsAccountCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HyperVRunAsAccountCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Site REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVSite {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the Hyper-V site."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. Type = Microsoft.OffAzure/HyperVSites."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "eTag for concurrency control."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Azure location in which Sites is created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Class for site properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SiteProperties>,
}
impl HyperVSite {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Hyper-V site usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVSiteUsage {
    #[doc = "Number of machines discovered in the site."]
    #[serde(rename = "machineCount", default, skip_serializing_if = "Option::is_none")]
    pub machine_count: Option<i32>,
    #[doc = "Number of run as accounts in the site."]
    #[serde(rename = "runAsAccountCount", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_count: Option<i32>,
    #[doc = "Number of hosts part of the site."]
    #[serde(rename = "hostCount", default, skip_serializing_if = "Option::is_none")]
    pub host_count: Option<i32>,
    #[doc = "Number of clusters part of the site."]
    #[serde(rename = "clusterCount", default, skip_serializing_if = "Option::is_none")]
    pub cluster_count: Option<i32>,
}
impl HyperVSiteUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervisorConfiguration {
    #[serde(rename = "hypervisorType", default, skip_serializing_if = "Option::is_none")]
    pub hypervisor_type: Option<hypervisor_configuration::HypervisorType>,
    #[serde(rename = "nativeHostMachineId", default, skip_serializing_if = "Option::is_none")]
    pub native_host_machine_id: Option<String>,
}
impl HypervisorConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hypervisor_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HypervisorType {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "hyperv")]
        Hyperv,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Ipv4NetworkInterface {
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "subnetMask", default, skip_serializing_if = "Option::is_none")]
    pub subnet_mask: Option<String>,
}
impl Ipv4NetworkInterface {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Ipv6NetworkInterface {
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
impl Ipv6NetworkInterface {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for machine properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobProperties {
    #[doc = "Operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Operation start time."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Operation end time."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "Display name of the Job."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Client request Id used in the operation execution context."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "Activity Id used in the operation execution context."]
    #[serde(rename = "activityId", default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[doc = "Errors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ErrorDetails>,
}
impl JobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Machine {
    #[serde(rename = "properties.timestamp", with = "azure_core::date::rfc3339::option")]
    pub properties_timestamp: Option<time::OffsetDateTime>,
    #[serde(rename = "properties.monitoringState", default, skip_serializing_if = "Option::is_none")]
    pub properties_monitoring_state: Option<machine::PropertiesMonitoringState>,
    #[serde(rename = "properties.virtualizationState", default, skip_serializing_if = "Option::is_none")]
    pub properties_virtualization_state: Option<machine::PropertiesVirtualizationState>,
    #[serde(rename = "properties.displayName", default, skip_serializing_if = "Option::is_none")]
    pub properties_display_name: Option<String>,
    #[serde(rename = "properties.computerName", default, skip_serializing_if = "Option::is_none")]
    pub properties_computer_name: Option<String>,
    #[serde(rename = "properties.fullyQualifiedDomainName", default, skip_serializing_if = "Option::is_none")]
    pub properties_fully_qualified_domain_name: Option<String>,
    #[serde(rename = "properties.bootTime", with = "azure_core::date::rfc3339::option")]
    pub properties_boot_time: Option<time::OffsetDateTime>,
    #[serde(rename = "properties.timezone", default, skip_serializing_if = "Option::is_none")]
    pub properties_timezone: Option<Timezone>,
    #[serde(rename = "properties.agent", default, skip_serializing_if = "Option::is_none")]
    pub properties_agent: Option<AgentConfiguration>,
    #[serde(rename = "properties.resources", default, skip_serializing_if = "Option::is_none")]
    pub properties_resources: Option<MachineResourcesConfiguration>,
    #[serde(rename = "properties.networking", default, skip_serializing_if = "Option::is_none")]
    pub properties_networking: Option<NetworkConfiguration>,
    #[serde(rename = "properties.operatingSystem", default, skip_serializing_if = "Option::is_none")]
    pub properties_operating_system: Option<OperatingSystemConfiguration>,
    #[serde(rename = "properties.virtualMachine", default, skip_serializing_if = "Option::is_none")]
    pub properties_virtual_machine: Option<VirtualMachineConfiguration>,
    #[serde(rename = "properties.hypervisor", default, skip_serializing_if = "Option::is_none")]
    pub properties_hypervisor: Option<HypervisorConfiguration>,
    #[serde(rename = "properties.hosting", default, skip_serializing_if = "Option::is_none")]
    pub properties_hosting: Option<HostingConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Machine {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PropertiesMonitoringState {
        #[serde(rename = "monitored")]
        Monitored,
        #[serde(rename = "discovered")]
        Discovered,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PropertiesVirtualizationState {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "physical")]
        Physical,
        #[serde(rename = "virtual")]
        Virtual,
        #[serde(rename = "hypervisor")]
        Hypervisor,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineResourcesConfiguration {
    #[serde(rename = "physicalMemory", default, skip_serializing_if = "Option::is_none")]
    pub physical_memory: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpus: Option<i32>,
    #[serde(rename = "cpuSpeed", default, skip_serializing_if = "Option::is_none")]
    pub cpu_speed: Option<i32>,
    #[serde(rename = "cpuSpeedAccuracy", default, skip_serializing_if = "Option::is_none")]
    pub cpu_speed_accuracy: Option<machine_resources_configuration::CpuSpeedAccuracy>,
}
impl MachineResourcesConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_resources_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CpuSpeedAccuracy {
        #[serde(rename = "actual")]
        Actual,
        #[serde(rename = "estimated")]
        Estimated,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkConfiguration {
    #[serde(rename = "ipv4Interfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub ipv4_interfaces: Vec<Ipv4NetworkInterface>,
    #[serde(rename = "ipv6Interfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub ipv6_interfaces: Vec<Ipv6NetworkInterface>,
    #[serde(rename = "defaultIpv4Gateways", default, skip_serializing_if = "Vec::is_empty")]
    pub default_ipv4_gateways: Vec<String>,
    #[serde(rename = "macAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub mac_addresses: Vec<String>,
    #[serde(rename = "dnsNames", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_names: Vec<String>,
    #[serde(rename = "dnsQuestions", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_questions: Vec<String>,
    #[serde(rename = "dnsCanonicalNames", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_canonical_names: Vec<String>,
}
impl NetworkConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Object {}
impl Object {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Second level object returned as part of Machine REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperatingSystem {
    #[doc = "Type of the operating system."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "Name of the operating system."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "Version of the operating system."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
}
impl OperatingSystem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperatingSystemConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<operating_system_configuration::Family>,
    #[serde(rename = "fullName", default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitness: Option<operating_system_configuration::Bitness>,
}
impl OperatingSystemConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operating_system_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Family {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "windows")]
        Windows,
        #[serde(rename = "linux")]
        Linux,
        #[serde(rename = "solaris")]
        Solaris,
        #[serde(rename = "aix")]
        Aix,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Bitness {
        #[serde(rename = "32bit")]
        N32bit,
        #[serde(rename = "64bit")]
        N64bit,
    }
}
#[doc = "A REST API operation supported by the provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Displayable properties of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Origin of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Displayable properties of the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Provider of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource operated on by the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation Type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of API operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResultList {
    #[doc = "List of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "Value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationResultList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationResultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation status REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Status of the operation. ARM expects the terminal status to be one of Succeeded/ Failed/ Canceled. All other values imply that the operation is still running."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Start time."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "End time."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "Class for operation status errors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationStatusError>,
    #[doc = "Class for operation result properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationStatusProperties>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for operation status errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatusError {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl OperationStatusError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for operation result properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatusProperties {
    #[doc = "Result or output of the workflow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}
impl OperationStatusProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OtherDatabase in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OtherDatabase {
    #[doc = "DatabaseType of the OtherDatabase."]
    #[serde(rename = "databaseType", default, skip_serializing_if = "Option::is_none")]
    pub database_type: Option<String>,
    #[doc = "Instance of the OtherDatabase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[doc = "Version of the OtherDatabase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl OtherDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for run as account properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunAsAccountProperties {
    #[doc = "Display name of the run as account."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Credential type of the run as account."]
    #[serde(rename = "credentialType", default, skip_serializing_if = "Option::is_none")]
    pub credential_type: Option<run_as_account_properties::CredentialType>,
    #[doc = "Timestamp marking run as account creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Timestamp marking last updated on the run as account."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
}
impl RunAsAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod run_as_account_properties {
    use super::*;
    #[doc = "Credential type of the run as account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CredentialType")]
    pub enum CredentialType {
        VMwareFabric,
        HyperVFabric,
        LinuxGuest,
        WindowsGuest,
        LinuxServer,
        WindowsServer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CredentialType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CredentialType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CredentialType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::VMwareFabric => serializer.serialize_unit_variant("CredentialType", 0u32, "VMwareFabric"),
                Self::HyperVFabric => serializer.serialize_unit_variant("CredentialType", 1u32, "HyperVFabric"),
                Self::LinuxGuest => serializer.serialize_unit_variant("CredentialType", 2u32, "LinuxGuest"),
                Self::WindowsGuest => serializer.serialize_unit_variant("CredentialType", 3u32, "WindowsGuest"),
                Self::LinuxServer => serializer.serialize_unit_variant("CredentialType", 4u32, "LinuxServer"),
                Self::WindowsServer => serializer.serialize_unit_variant("CredentialType", 5u32, "WindowsServer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "SQLServer in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServer {
    #[doc = "Name of the SQLServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Edition of the SQLServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[doc = "ServicePack of the SQLServer."]
    #[serde(rename = "servicePack", default, skip_serializing_if = "Option::is_none")]
    pub service_pack: Option<String>,
    #[doc = "Version of the SQLServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Clustered of the SQLServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clustered: Option<String>,
    #[doc = "ClusterName of the SQLServer."]
    #[serde(rename = "clusterName", default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
}
impl SqlServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SharePointServer in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharePointServer {
    #[doc = "ProductName of the SharePointServer."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "Value indicating whether the SharePointServer is Enterprise."]
    #[serde(rename = "isEnterprise", default, skip_serializing_if = "Option::is_none")]
    pub is_enterprise: Option<bool>,
    #[doc = "Status of the SharePointServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Version of the SharePointServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl SharePointServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for site agent properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteAgentProperties {
    #[doc = "ID of the agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Version of the agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Last heartbeat time of the agent in UTC."]
    #[serde(rename = "lastHeartBeatUtc", with = "azure_core::date::rfc3339::option")]
    pub last_heart_beat_utc: Option<time::OffsetDateTime>,
    #[doc = "Key vault URI."]
    #[serde(rename = "keyVaultUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_uri: Option<String>,
    #[doc = "Key vault ARM Id."]
    #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_id: Option<String>,
}
impl SiteAgentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Site health summary model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteHealthSummary {
    #[doc = "Appliance name."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
    #[doc = "Error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Summary message."]
    #[serde(rename = "summaryMessage", default, skip_serializing_if = "Option::is_none")]
    pub summary_message: Option<String>,
    #[doc = "Error Id."]
    #[serde(rename = "errorId", default, skip_serializing_if = "Option::is_none")]
    pub error_id: Option<i64>,
    #[doc = "Error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Count of affected objects."]
    #[serde(rename = "affectedObjectsCount", default, skip_serializing_if = "Option::is_none")]
    pub affected_objects_count: Option<i64>,
    #[doc = "Hit count of the error."]
    #[serde(rename = "hitCount", default, skip_serializing_if = "Option::is_none")]
    pub hit_count: Option<i64>,
    #[doc = "Severity of error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "Remediation guidance."]
    #[serde(rename = "remediationGuidance", default, skip_serializing_if = "Option::is_none")]
    pub remediation_guidance: Option<String>,
    #[doc = "Affected resource type."]
    #[serde(rename = "affectedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub affected_resource_type: Option<String>,
    #[doc = "Affected resources."]
    #[serde(rename = "affectedResources", default, skip_serializing_if = "Vec::is_empty")]
    pub affected_resources: Vec<String>,
}
impl SiteHealthSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of SiteHealthSummary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteHealthSummaryCollection {
    #[doc = "List of SiteHealthSummary."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SiteHealthSummary>,
    #[doc = "Value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SiteHealthSummaryCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SiteHealthSummaryCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for site properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteProperties {
    #[doc = "Class for site properties."]
    #[serde(rename = "servicePrincipalIdentityDetails", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_identity_details: Option<SiteSpnProperties>,
    #[doc = "Class for site agent properties."]
    #[serde(rename = "agentDetails", default, skip_serializing_if = "Option::is_none")]
    pub agent_details: Option<SiteAgentProperties>,
    #[doc = "Service endpoint."]
    #[serde(rename = "serviceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,
    #[doc = "ARM ID of migration hub solution for SDS."]
    #[serde(rename = "discoverySolutionId", default, skip_serializing_if = "Option::is_none")]
    pub discovery_solution_id: Option<String>,
    #[doc = "Appliance Name."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
}
impl SiteProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for site properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteSpnProperties {
    #[doc = "Tenant Id for the service principal with which the on-premise management/data plane components would communicate with our Azure services."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Application/client Id for the service principal with which the on-premise management/data plane components would communicate with our Azure services."]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "Object Id of the service principal with which the on-premise management/data plane components would communicate with our Azure services."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Intended audience for the service principal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[doc = "AAD Authority URL which was used to request the token for the service principal."]
    #[serde(rename = "aadAuthority", default, skip_serializing_if = "Option::is_none")]
    pub aad_authority: Option<String>,
    #[doc = "Raw certificate data for building certificate expiry flows."]
    #[serde(rename = "rawCertData", default, skip_serializing_if = "Option::is_none")]
    pub raw_cert_data: Option<String>,
}
impl SiteSpnProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SystemCenter in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemCenter {
    #[doc = "ProductName of the SystemCenter."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "Status of the SystemCenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Version of the SystemCenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl SystemCenter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Timezone {
    #[serde(rename = "fullName", default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
}
impl Timezone {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VCenter REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VCenter {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the vCenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. Type = Microsoft.OffAzure/VMWareSites/VCenters."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Class for vCenter properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VCenterProperties>,
}
impl VCenter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of vCenter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VCenterCollection {
    #[doc = "List of vCenter."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VCenter>,
    #[doc = "Value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VCenterCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VCenterCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for vCenter properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VCenterProperties {
    #[doc = "Timestamp marking vCenter creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Timestamp marking last updated on the vCenter."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "FQDN/IPAddress of the vCenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Port of the vCenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[doc = "Run as account ID of the vCenter."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Version of the vCenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Performance statistics enabled on the vCenter."]
    #[serde(rename = "perfStatisticsLevel", default, skip_serializing_if = "Option::is_none")]
    pub perf_statistics_level: Option<String>,
    #[doc = "Instance UUID of the vCenter."]
    #[serde(rename = "instanceUuid", default, skip_serializing_if = "Option::is_none")]
    pub instance_uuid: Option<String>,
    #[doc = "Error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<HealthErrorDetails>,
}
impl VCenterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Second level object returned as part of Machine REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VMwareDisk {
    #[doc = "Disk UUID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Label of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "The provisioning policy of the disk. It is Thin or Thick or Unknown for the VMWare."]
    #[serde(rename = "diskProvisioningPolicy", default, skip_serializing_if = "Option::is_none")]
    pub disk_provisioning_policy: Option<String>,
    #[doc = "The scrubbing policy of disks which can be eagerly zeroed or lazily zeroed."]
    #[serde(rename = "diskScrubbingPolicy", default, skip_serializing_if = "Option::is_none")]
    pub disk_scrubbing_policy: Option<String>,
    #[doc = "Disk mode property used for identifying independent disks."]
    #[serde(rename = "diskMode", default, skip_serializing_if = "Option::is_none")]
    pub disk_mode: Option<v_mware_disk::DiskMode>,
    #[doc = "Bytes allocated for the disk."]
    #[serde(rename = "maxSizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_in_bytes: Option<i64>,
    #[doc = "Name of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the disk."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
    #[doc = "LUN of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    #[doc = "Path of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl VMwareDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod v_mware_disk {
    use super::*;
    #[doc = "Disk mode property used for identifying independent disks."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiskMode")]
    pub enum DiskMode {
        #[serde(rename = "persistent")]
        Persistent,
        #[serde(rename = "independent_persistent")]
        IndependentPersistent,
        #[serde(rename = "independent_nonpersistent")]
        IndependentNonpersistent,
        #[serde(rename = "nonpersistent")]
        Nonpersistent,
        #[serde(rename = "undoable")]
        Undoable,
        #[serde(rename = "append")]
        Append,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiskMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiskMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiskMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Persistent => serializer.serialize_unit_variant("DiskMode", 0u32, "persistent"),
                Self::IndependentPersistent => serializer.serialize_unit_variant("DiskMode", 1u32, "independent_persistent"),
                Self::IndependentNonpersistent => serializer.serialize_unit_variant("DiskMode", 2u32, "independent_nonpersistent"),
                Self::Nonpersistent => serializer.serialize_unit_variant("DiskMode", 3u32, "nonpersistent"),
                Self::Undoable => serializer.serialize_unit_variant("DiskMode", 4u32, "undoable"),
                Self::Append => serializer.serialize_unit_variant("DiskMode", 5u32, "append"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Job REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VMwareJob {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. Type = Microsoft.OffAzure/VMWareSites/Jobs."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Class for machine properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
}
impl VMwareJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of VMware jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VMwareJobCollection {
    #[doc = "List of jobs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VMwareJob>,
    #[doc = "Value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VMwareJobCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VMwareJobCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Machine REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VMwareMachine {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the Sites."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. Type = Microsoft.OffAzure/VMWareSites/Machines."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Class for machine properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VMwareMachineProperties>,
}
impl VMwareMachine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of VMware machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VMwareMachineCollection {
    #[doc = "List of machines."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VMwareMachine>,
    #[doc = "Value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VMwareMachineCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VMwareMachineCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for machine properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VMwareMachineProperties {
    #[doc = "Scope of the data center."]
    #[serde(rename = "dataCenterScope", default, skip_serializing_if = "Option::is_none")]
    pub data_center_scope: Option<String>,
    #[doc = "Firmware of the machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    #[doc = "User description of the machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "VCenter FQDN/IPAddress."]
    #[serde(rename = "vCenterFQDN", default, skip_serializing_if = "Option::is_none")]
    pub v_center_fqdn: Option<String>,
    #[doc = "VCenter ARM ID."]
    #[serde(rename = "vCenterId", default, skip_serializing_if = "Option::is_none")]
    pub v_center_id: Option<String>,
    #[doc = "VMware tools status."]
    #[serde(rename = "vMwareToolsStatus", default, skip_serializing_if = "Option::is_none")]
    pub v_mware_tools_status: Option<String>,
    #[doc = "Value indicating whether change tracking is supported."]
    #[serde(rename = "changeTrackingSupported", default, skip_serializing_if = "Option::is_none")]
    pub change_tracking_supported: Option<bool>,
    #[doc = "Value indicating whether change tracking is enabled."]
    #[serde(rename = "changeTrackingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub change_tracking_enabled: Option<bool>,
    #[doc = "Maximum number of snapshots for the VM. Default value is -1."]
    #[serde(rename = "maxSnapshots", default, skip_serializing_if = "Option::is_none")]
    pub max_snapshots: Option<i32>,
    #[doc = "Disks attached to the machine."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<VMwareDisk>,
    #[doc = "AppsAndRoles in the guest virtual machine."]
    #[serde(rename = "appsAndRoles", default, skip_serializing_if = "Option::is_none")]
    pub apps_and_roles: Option<AppsAndRoles>,
    #[doc = "Indicates whether the host is in maintenance mode."]
    #[serde(rename = "hostInMaintenanceMode", default, skip_serializing_if = "Option::is_none")]
    pub host_in_maintenance_mode: Option<bool>,
    #[doc = "The host name."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "The host power state."]
    #[serde(rename = "hostPowerState", default, skip_serializing_if = "Option::is_none")]
    pub host_power_state: Option<String>,
    #[doc = "The host version."]
    #[serde(rename = "hostVersion", default, skip_serializing_if = "Option::is_none")]
    pub host_version: Option<String>,
    #[doc = "Network adapters attached to the machine."]
    #[serde(rename = "networkAdapters", default, skip_serializing_if = "Vec::is_empty")]
    pub network_adapters: Vec<VMwareNetworkAdapter>,
    #[doc = "If dependency mapping feature is enabled or not for the VM."]
    #[serde(rename = "dependencyMapping", default, skip_serializing_if = "Option::is_none")]
    pub dependency_mapping: Option<String>,
    #[doc = "When dependency mapping collection is last started."]
    #[serde(rename = "dependencyMappingStartTime", with = "azure_core::date::rfc3339::option")]
    pub dependency_mapping_start_time: Option<time::OffsetDateTime>,
    #[doc = "Display name of the machine."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Number of Processor Cores allocated for the machine."]
    #[serde(rename = "numberOfProcessorCore", default, skip_serializing_if = "Option::is_none")]
    pub number_of_processor_core: Option<i32>,
    #[doc = "Allocated Memory in MB."]
    #[serde(rename = "allocatedMemoryInMB", default, skip_serializing_if = "Option::is_none")]
    pub allocated_memory_in_mb: Option<f64>,
    #[doc = "Root location of the VM configuration file."]
    #[serde(rename = "vmConfigurationFileLocation", default, skip_serializing_if = "Option::is_none")]
    pub vm_configuration_file_location: Option<String>,
    #[doc = "Second level object returned as part of Machine REST resource."]
    #[serde(rename = "operatingSystemDetails", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_details: Option<OperatingSystem>,
    #[doc = "Data related to a machine's operating system. Serialized and stored as part of Machine Rest object."]
    #[serde(rename = "guestOSDetails", default, skip_serializing_if = "Option::is_none")]
    pub guest_os_details: Option<GuestOsDetails>,
    #[doc = "Number of applications installed in the guest VM."]
    #[serde(rename = "numberOfApplications", default, skip_serializing_if = "Option::is_none")]
    pub number_of_applications: Option<i32>,
    #[doc = "The last time at which the Guest Details was discovered or the error while discovering guest details based discovery of the machine."]
    #[serde(rename = "guestDetailsDiscoveryTimestamp", with = "azure_core::date::rfc3339::option")]
    pub guest_details_discovery_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Whether Refresh Fabric Layout Guest Details has been completed once. Portal will show discovery in progress, if this value is true."]
    #[serde(rename = "isGuestDetailsDiscoveryInProgress", default, skip_serializing_if = "Option::is_none")]
    pub is_guest_details_discovery_in_progress: Option<bool>,
    #[doc = "Timestamp marking machine creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Timestamp marking last updated on the machine."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "On-premise Instance UUID of the machine."]
    #[serde(rename = "instanceUuid", default, skip_serializing_if = "Option::is_none")]
    pub instance_uuid: Option<String>,
    #[doc = "Machine power status."]
    #[serde(rename = "powerStatus", default, skip_serializing_if = "Option::is_none")]
    pub power_status: Option<String>,
    #[doc = "Machine BIOS serial number."]
    #[serde(rename = "biosSerialNumber", default, skip_serializing_if = "Option::is_none")]
    pub bios_serial_number: Option<String>,
    #[doc = "BIOS GUID."]
    #[serde(rename = "biosGuid", default, skip_serializing_if = "Option::is_none")]
    pub bios_guid: Option<String>,
    #[doc = "Machine FQDN."]
    #[serde(rename = "vmFqdn", default, skip_serializing_if = "Option::is_none")]
    pub vm_fqdn: Option<String>,
    #[doc = "Value indicating whether VM is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Errors for machine."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<HealthErrorDetails>,
}
impl VMwareMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Second level object represented in responses as part of Machine REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VMwareNetworkAdapter {
    #[doc = "Label of the NIC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "NIC Id."]
    #[serde(rename = "nicId", default, skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    #[doc = "Mac address of the NIC."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "IP addresses for the machine."]
    #[serde(rename = "ipAddressList", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_address_list: Vec<String>,
    #[doc = "Network Name."]
    #[serde(rename = "networkName", default, skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[doc = "Type of the IP address."]
    #[serde(rename = "ipAddressType", default, skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
}
impl VMwareNetworkAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Run as account REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VMwareRunAsAccount {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the Run as account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. Type = Microsoft.OffAzure/VMWareSites/RunAsAccounts."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Class for run as account properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RunAsAccountProperties>,
}
impl VMwareRunAsAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of VMware run as accounts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VMwareRunAsAccountCollection {
    #[doc = "List of run as accounts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VMwareRunAsAccount>,
    #[doc = "Value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VMwareRunAsAccountCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VMwareRunAsAccountCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Site REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VMwareSite {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the VMware site."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. Type = Microsoft.OffAzure/VMWareSites."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "eTag for concurrency control."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Azure location in which Sites is created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Class for site properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SiteProperties>,
}
impl VMwareSite {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VMware site usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VMwareSiteUsage {
    #[doc = "Number of machines discovered in the site."]
    #[serde(rename = "machineCount", default, skip_serializing_if = "Option::is_none")]
    pub machine_count: Option<i32>,
    #[doc = "Number of run as accounts in the site."]
    #[serde(rename = "runAsAccountCount", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_count: Option<i32>,
    #[doc = "Number of vCenters part of the site."]
    #[serde(rename = "vCenterCount", default, skip_serializing_if = "Option::is_none")]
    pub v_center_count: Option<i32>,
}
impl VMwareSiteUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineConfiguration {
    #[serde(rename = "virtualMachineType", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_type: Option<virtual_machine_configuration::VirtualMachineType>,
    #[serde(rename = "nativeMachineId", default, skip_serializing_if = "Option::is_none")]
    pub native_machine_id: Option<String>,
    #[serde(rename = "virtualMachineName", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_name: Option<String>,
    #[serde(rename = "nativeHostMachineId", default, skip_serializing_if = "Option::is_none")]
    pub native_host_machine_id: Option<String>,
}
impl VirtualMachineConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VirtualMachineType {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "hyperv")]
        Hyperv,
        #[serde(rename = "ldom")]
        Ldom,
        #[serde(rename = "lpar")]
        Lpar,
        #[serde(rename = "vmware")]
        Vmware,
        #[serde(rename = "virtualPc")]
        VirtualPc,
        #[serde(rename = "xen")]
        Xen,
    }
}
#[doc = "WebApplication in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplication {
    #[doc = "Name of the WebApplication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Status of the WebApplication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Platform of the WebApplication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[doc = "GroupName of the WebApplication."]
    #[serde(rename = "groupName", default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[doc = "WebServer of the WebApplication."]
    #[serde(rename = "webServer", default, skip_serializing_if = "Option::is_none")]
    pub web_server: Option<String>,
    #[doc = "ApplicationPool of the WebApplication."]
    #[serde(rename = "applicationPool", default, skip_serializing_if = "Option::is_none")]
    pub application_pool: Option<String>,
}
impl WebApplication {
    pub fn new() -> Self {
        Self::default()
    }
}

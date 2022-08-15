#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A process accepting on a port."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Acceptor {
    #[serde(flatten)]
    pub relationship: Relationship,
    #[doc = "Properties for an acceptor relationship."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AcceptorProperties>,
}
impl Acceptor {
    pub fn new(relationship: Relationship) -> Self {
        Self {
            relationship,
            properties: None,
        }
    }
}
#[doc = "Properties for an acceptor relationship."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcceptorProperties {
    #[doc = "Reference to a port."]
    pub source: PortReference,
    #[doc = "Reference to a process."]
    pub destination: ProcessReference,
    #[doc = "Relationship start time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Relationship end time."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}
impl AcceptorProperties {
    pub fn new(source: PortReference, destination: ProcessReference) -> Self {
        Self {
            source,
            destination,
            start_time: None,
            end_time: None,
        }
    }
}
#[doc = "Specifies the accuracy of a computation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Accuracy {
    #[serde(rename = "actual")]
    Actual,
    #[serde(rename = "estimated")]
    Estimated,
}
#[doc = "Describes the configuration of the Dependency Agent installed on a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentConfiguration {
    #[doc = "Health Service Agent unique identifier."]
    #[serde(rename = "agentId")]
    pub agent_id: String,
    #[doc = "Dependency Agent unique identifier."]
    #[serde(rename = "dependencyAgentId", default, skip_serializing_if = "Option::is_none")]
    pub dependency_agent_id: Option<String>,
    #[doc = "Dependency Agent version number."]
    #[serde(rename = "dependencyAgentVersion", default, skip_serializing_if = "Option::is_none")]
    pub dependency_agent_version: Option<String>,
    #[doc = "Dependency Agent revision number."]
    #[serde(rename = "dependencyAgentRevision", default, skip_serializing_if = "Option::is_none")]
    pub dependency_agent_revision: Option<String>,
    #[doc = "Specifies if the machine has been rebooted since the installation of the dependency agent."]
    #[serde(rename = "rebootStatus", default, skip_serializing_if = "Option::is_none")]
    pub reboot_status: Option<MachineRebootStatus>,
    #[doc = "Machine clock granularity in milliseconds."]
    #[serde(rename = "clockGranularity", default, skip_serializing_if = "Option::is_none")]
    pub clock_granularity: Option<i32>,
}
impl AgentConfiguration {
    pub fn new(agent_id: String) -> Self {
        Self {
            agent_id,
            dependency_agent_id: None,
            dependency_agent_version: None,
            dependency_agent_revision: None,
            reboot_status: None,
            clock_granularity: None,
        }
    }
}
#[doc = "Describes an Azure Cloud Service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCloudServiceConfiguration {
    #[doc = "Cloud Service name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Cloud Service instance identifier"]
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[doc = "Cloud Service deployment identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<String>,
    #[doc = "Cloud Service role name"]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[doc = "Used to specify type of an Azure Cloud Service role"]
    #[serde(rename = "roleType", default, skip_serializing_if = "Option::is_none")]
    pub role_type: Option<azure_cloud_service_configuration::RoleType>,
}
impl AzureCloudServiceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod azure_cloud_service_configuration {
    use super::*;
    #[doc = "Used to specify type of an Azure Cloud Service role"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RoleType {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "worker")]
        Worker,
        #[serde(rename = "web")]
        Web,
    }
}
#[doc = "Provides information about how a machine is hosted in Azure"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureHostingConfiguration {
    #[serde(flatten)]
    pub hosting_configuration: HostingConfiguration,
    #[doc = "Virtual Machine ID (unique identifier)."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "Geographical location of the VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Machine name according to the hosting provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Size of the VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "Update domain of the VM."]
    #[serde(rename = "updateDomain", default, skip_serializing_if = "Option::is_none")]
    pub update_domain: Option<String>,
    #[doc = "Fault domain of the VM."]
    #[serde(rename = "faultDomain", default, skip_serializing_if = "Option::is_none")]
    pub fault_domain: Option<String>,
    #[doc = "Subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Resource group name within the specified subscription."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "Unique identifier of the resource."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Describes the VM image of a machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageConfiguration>,
    #[doc = "Describes an Azure Cloud Service"]
    #[serde(rename = "cloudService", default, skip_serializing_if = "Option::is_none")]
    pub cloud_service: Option<AzureCloudServiceConfiguration>,
    #[doc = "Describes an Azure Virtual Machine Scale Set"]
    #[serde(rename = "vmScaleSet", default, skip_serializing_if = "Option::is_none")]
    pub vm_scale_set: Option<AzureVmScaleSetConfiguration>,
    #[doc = "Describes an Azure Service Fabric Cluster"]
    #[serde(rename = "serviceFabricCluster", default, skip_serializing_if = "Option::is_none")]
    pub service_fabric_cluster: Option<AzureServiceFabricClusterConfiguration>,
}
impl AzureHostingConfiguration {
    pub fn new(hosting_configuration: HostingConfiguration) -> Self {
        Self {
            hosting_configuration,
            vm_id: None,
            location: None,
            name: None,
            size: None,
            update_domain: None,
            fault_domain: None,
            subscription_id: None,
            resource_group: None,
            resource_id: None,
            image: None,
            cloud_service: None,
            vm_scale_set: None,
            service_fabric_cluster: None,
        }
    }
}
#[doc = "Describes the hosting configuration of a process when hosted on azure"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureProcessHostingConfiguration {
    #[serde(flatten)]
    pub process_hosting_configuration: ProcessHostingConfiguration,
    #[doc = "Describes an Azure Cloud Service"]
    #[serde(rename = "cloudService", default, skip_serializing_if = "Option::is_none")]
    pub cloud_service: Option<AzureCloudServiceConfiguration>,
}
impl AzureProcessHostingConfiguration {
    pub fn new(process_hosting_configuration: ProcessHostingConfiguration) -> Self {
        Self {
            process_hosting_configuration,
            cloud_service: None,
        }
    }
}
#[doc = "Describes an Azure Service Fabric Cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureServiceFabricClusterConfiguration {
    #[doc = "Service Fabric cluster name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Service Fabric cluster identifier."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
}
impl AzureServiceFabricClusterConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure Virtual Machine Scale Set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureVmScaleSetConfiguration {
    #[doc = "Virtual Machine Scale Set name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Virtual Machine Scale Set instance identifier"]
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[doc = "Virtual Machine Scale Set deployment identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<String>,
    #[doc = "Unique identifier of the resource."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl AzureVmScaleSetConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the bitness of a machine or process."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Bitness {
    #[serde(rename = "32bit")]
    N32bit,
    #[serde(rename = "64bit")]
    N64bit,
}
#[doc = "Represents a collection of clients of a resource. A client group can represent the clients of a port, process, or a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientGroup {
    #[serde(flatten)]
    pub core_resource: CoreResource,
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<client_group::Properties>,
}
impl ClientGroup {
    pub fn new(core_resource: CoreResource) -> Self {
        Self {
            core_resource,
            properties: None,
        }
    }
}
pub mod client_group {
    use super::*;
    #[doc = "Resource properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "Represents a reference to another resource."]
        #[serde(rename = "clientsOf")]
        pub clients_of: ResourceReference,
    }
    impl Properties {
        pub fn new(clients_of: ResourceReference) -> Self {
            Self { clients_of }
        }
    }
}
#[doc = "Represents a member of a client group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientGroupMember {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<client_group_member::Properties>,
}
impl ClientGroupMember {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod client_group_member {
    use super::*;
    #[doc = "Resource properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "IP address."]
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
        pub ip_address: Option<String>,
        #[doc = "Reference to a port."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port: Option<PortReference>,
        #[doc = "Processes accepting on the above port that received connections from this client."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub processes: Vec<ProcessReference>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of ClientGroupMember resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientGroupMembersCollection {
    #[doc = "Collection of ClientGroupMember resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ClientGroupMember>,
    #[doc = "The URL to the next set of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ClientGroupMembersCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ClientGroupMembersCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the number of members in a client group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientGroupMembersCount {
    #[doc = "Membership interval start time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "Membership interval start time."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339")]
    pub end_time: time::OffsetDateTime,
    #[doc = "Client Group URI."]
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[doc = "Number of members in the client group. Use this value together with the value of ```accuracy```. If accuracy is `exact` then the value represents the actual number of members in the cloud. When accuracy is `estimated`, the actual number of members is larger than the value of ```count```."]
    pub count: i32,
    #[doc = "Specifies the accuracy of a computation."]
    pub accuracy: Accuracy,
}
impl ClientGroupMembersCount {
    pub fn new(start_time: time::OffsetDateTime, end_time: time::OffsetDateTime, group_id: String, count: i32, accuracy: Accuracy) -> Self {
        Self {
            start_time,
            end_time,
            group_id,
            count,
            accuracy,
        }
    }
}
#[doc = "Reference to a client group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientGroupReference {
    #[serde(flatten)]
    pub resource_reference: ResourceReference,
}
impl ClientGroupReference {
    pub fn new(resource_reference: ResourceReference) -> Self {
        Self { resource_reference }
    }
}
#[doc = "A network connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Connection {
    #[serde(flatten)]
    pub relationship: Relationship,
    #[doc = "Properties for a connection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConnectionProperties>,
}
impl Connection {
    pub fn new(relationship: Relationship) -> Self {
        Self {
            relationship,
            properties: None,
        }
    }
}
#[doc = "Collection of Connection resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionCollection {
    #[doc = "Collection of Connection resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Connection>,
    #[doc = "The URL to the next set of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConnectionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ConnectionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Connection failure state:\n * ```ok``` indicates no failures\n * ```failed``` indicates only failures\n * ```mixed``` indicates both failures and successes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConnectionFailureState {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "mixed")]
    Mixed,
}
#[doc = "Properties for a connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionProperties {
    #[serde(flatten)]
    pub relationship_properties: RelationshipProperties,
    #[doc = "Reference to a port."]
    #[serde(rename = "serverPort", default, skip_serializing_if = "Option::is_none")]
    pub server_port: Option<PortReference>,
    #[doc = "Connection failure state:\n * ```ok``` indicates no failures\n * ```failed``` indicates only failures\n * ```mixed``` indicates both failures and successes"]
    #[serde(rename = "failureState", default, skip_serializing_if = "Option::is_none")]
    pub failure_state: Option<ConnectionFailureState>,
}
impl ConnectionProperties {
    pub fn new(relationship_properties: RelationshipProperties) -> Self {
        Self {
            relationship_properties,
            server_port: None,
            failure_state: None,
        }
    }
}
#[doc = "Marker resource for the core Service Map resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoreResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource ETAG."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Additional resource type qualifier."]
    pub kind: core_resource::Kind,
}
impl CoreResource {
    pub fn new(kind: core_resource::Kind) -> Self {
        Self {
            resource: Resource::default(),
            etag: None,
            kind,
        }
    }
}
pub mod core_resource {
    use super::*;
    #[doc = "Additional resource type qualifier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "machine")]
        Machine,
        #[serde(rename = "process")]
        Process,
        #[serde(rename = "port")]
        Port,
        #[serde(rename = "clientGroup")]
        ClientGroup,
        #[serde(rename = "machineGroup")]
        MachineGroup,
    }
}
#[doc = "Error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[doc = "Error code identifying the specific error."]
    pub code: String,
    #[doc = "Error message in the caller's locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl Error {
    pub fn new(code: String) -> Self {
        Self { code, message: None }
    }
}
#[doc = "An error response from the API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[doc = "Error details."]
    pub error: Error,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new(error: Error) -> Self {
        Self { error }
    }
}
#[doc = "Describes the hosting configuration of a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostingConfiguration {
    #[doc = "The hosting provider of the VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<hosting_configuration::Provider>,
    #[doc = "Additional hosting configuration type qualifier."]
    pub kind: hosting_configuration::Kind,
}
impl HostingConfiguration {
    pub fn new(kind: hosting_configuration::Kind) -> Self {
        Self { provider: None, kind }
    }
}
pub mod hosting_configuration {
    use super::*;
    #[doc = "The hosting provider of the VM."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Provider {
        #[serde(rename = "azure")]
        Azure,
    }
    #[doc = "Additional hosting configuration type qualifier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "provider:azure")]
        ProviderAzure,
    }
}
#[doc = "Describes the hypervisor configuration of a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervisorConfiguration {
    #[doc = "Specifies the hypervisor type of a machine."]
    #[serde(rename = "hypervisorType", default, skip_serializing_if = "Option::is_none")]
    pub hypervisor_type: Option<HypervisorType>,
    #[doc = "The unique identifier of the hypervisor machine as reported by the underlying virtualization system."]
    #[serde(rename = "nativeHostMachineId", default, skip_serializing_if = "Option::is_none")]
    pub native_host_machine_id: Option<String>,
}
impl HypervisorConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the hypervisor type of a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HypervisorType {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "hyperv")]
    Hyperv,
}
#[doc = "Describes the VM image of a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageConfiguration {
    #[doc = "Publisher of the VM image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Offering of the VM image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offering: Option<String>,
    #[doc = "SKU of the VM image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "Version of the VM image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ImageConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an IPv4 network interface."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ipv4NetworkInterface {
    #[doc = "IPv4 address."]
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    #[doc = "IPv4 subnet mask."]
    #[serde(rename = "subnetMask", default, skip_serializing_if = "Option::is_none")]
    pub subnet_mask: Option<String>,
}
impl Ipv4NetworkInterface {
    pub fn new(ip_address: String) -> Self {
        Self {
            ip_address,
            subnet_mask: None,
        }
    }
}
#[doc = "Describes an IPv6 network interface."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ipv6NetworkInterface {
    #[doc = "IPv6 address."]
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
}
impl Ipv6NetworkInterface {
    pub fn new(ip_address: String) -> Self {
        Self { ip_address }
    }
}
#[doc = "Specifies the contents of a check liveness response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Liveness {
    #[doc = "Liveness interval start time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "Liveness interval end time."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339")]
    pub end_time: time::OffsetDateTime,
    #[doc = "`true` if the resource is live during [startTime, endTime], `false` otherwise"]
    pub live: bool,
}
impl Liveness {
    pub fn new(start_time: time::OffsetDateTime, end_time: time::OffsetDateTime, live: bool) -> Self {
        Self {
            start_time,
            end_time,
            live,
        }
    }
}
#[doc = "A machine resource represents a discovered computer system. It can be *monitored*, i.e., a Dependency Agent is running on it, or *discovered*, i.e., its existence was inferred by observing the data stream from monitored machines. As machines change, prior versions of the machine resource are preserved and available for access. A machine is live during an interval of time, if either its Dependency Agent has reported data during (parts) of that interval, or a Dependency agent running on other machines has reported activity associated with the machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Machine {
    #[serde(flatten)]
    pub core_resource: CoreResource,
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<machine::Properties>,
}
impl Machine {
    pub fn new(core_resource: CoreResource) -> Self {
        Self {
            core_resource,
            properties: None,
        }
    }
}
pub mod machine {
    use super::*;
    #[doc = "Resource properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "UTC date and time when this resource was updated in the system."]
        #[serde(with = "azure_core::date::rfc3339::option")]
        pub timestamp: Option<time::OffsetDateTime>,
        #[doc = "Used to specify if a resources is monitored or discovered."]
        #[serde(rename = "monitoringState", default, skip_serializing_if = "Option::is_none")]
        pub monitoring_state: Option<MonitoringState>,
        #[doc = "Specifies if the machine is physical, virtual, hypervisor, or unknown."]
        #[serde(rename = "virtualizationState", default, skip_serializing_if = "Option::is_none")]
        pub virtualization_state: Option<VirtualizationState>,
        #[doc = "Name to use for display purposes"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Name of the machine, e.g., server"]
        #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
        pub computer_name: Option<String>,
        #[doc = "Fully-qualified name of the machine, e.g., server.company.com"]
        #[serde(rename = "fullyQualifiedDomainName", default, skip_serializing_if = "Option::is_none")]
        pub fully_qualified_domain_name: Option<String>,
        #[doc = "UTC date and time when the machine last booted"]
        #[serde(rename = "bootTime", with = "azure_core::date::rfc3339::option")]
        pub boot_time: Option<time::OffsetDateTime>,
        #[doc = "Describes a timezone."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub timezone: Option<Timezone>,
        #[doc = "Describes the configuration of the Dependency Agent installed on a machine."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub agent: Option<AgentConfiguration>,
        #[doc = "Describes the resources of a machine."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resources: Option<MachineResourcesConfiguration>,
        #[doc = "Describes the network configuration of a machine."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub networking: Option<NetworkConfiguration>,
        #[doc = "Describes the configuration of the operating system of a machine."]
        #[serde(rename = "operatingSystem", default, skip_serializing_if = "Option::is_none")]
        pub operating_system: Option<OperatingSystemConfiguration>,
        #[doc = "Describes the virtualization-related configuration of a machine."]
        #[serde(rename = "virtualMachine", default, skip_serializing_if = "Option::is_none")]
        pub virtual_machine: Option<VirtualMachineConfiguration>,
        #[doc = "Describes the hypervisor configuration of a machine."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hypervisor: Option<HypervisorConfiguration>,
        #[doc = "Describes the hosting configuration of a machine."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hosting: Option<HostingConfiguration>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of Machine resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineCollection {
    #[doc = "Collection of Machine resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Machine>,
    #[doc = "The URL to the next set of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MachineCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MachineCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Machines by operating system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineCountsByOperatingSystem {
    #[doc = "Number of live Windows machines."]
    pub windows: i32,
    #[doc = "Number of live Linux machines."]
    pub linux: i32,
}
impl MachineCountsByOperatingSystem {
    pub fn new(windows: i32, linux: i32) -> Self {
        Self { windows, linux }
    }
}
#[doc = "A user-defined logical grouping of machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineGroup {
    #[serde(flatten)]
    pub core_resource: CoreResource,
    #[doc = "Resource ETAG."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<machine_group::Properties>,
}
impl MachineGroup {
    pub fn new(core_resource: CoreResource) -> Self {
        Self {
            core_resource,
            etag: None,
            properties: None,
        }
    }
}
pub mod machine_group {
    use super::*;
    #[doc = "Resource properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "Type of the machine group"]
        #[serde(rename = "groupType", default, skip_serializing_if = "Option::is_none")]
        pub group_type: Option<properties::GroupType>,
        #[doc = "User defined name for the group"]
        #[serde(rename = "displayName")]
        pub display_name: String,
        #[doc = "Count of machines in this group. The value of count may be bigger than the number of machines in case of the group has been truncated due to exceeding the max number of machines a group can handle."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub count: Option<i32>,
        #[doc = "References of the machines in this group. The hints within each reference do not represent the current value of the corresponding fields. They are a snapshot created during the last time the machine group was updated."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub machines: Vec<MachineReferenceWithHints>,
    }
    impl Properties {
        pub fn new(display_name: String) -> Self {
            Self {
                group_type: None,
                display_name,
                count: None,
                machines: Vec::new(),
            }
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Type of the machine group"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "GroupType")]
        pub enum GroupType {
            #[serde(rename = "unknown")]
            Unknown,
            #[serde(rename = "azure-cs")]
            AzureCs,
            #[serde(rename = "azure-sf")]
            AzureSf,
            #[serde(rename = "azure-vmss")]
            AzureVmss,
            #[serde(rename = "user-static")]
            UserStatic,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for GroupType {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for GroupType {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for GroupType {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Unknown => serializer.serialize_unit_variant("GroupType", 0u32, "unknown"),
                    Self::AzureCs => serializer.serialize_unit_variant("GroupType", 1u32, "azure-cs"),
                    Self::AzureSf => serializer.serialize_unit_variant("GroupType", 2u32, "azure-sf"),
                    Self::AzureVmss => serializer.serialize_unit_variant("GroupType", 3u32, "azure-vmss"),
                    Self::UserStatic => serializer.serialize_unit_variant("GroupType", 4u32, "user-static"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Collection of Machine Group resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineGroupCollection {
    #[doc = "Collection of Machine Group resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MachineGroup>,
    #[doc = "The URL to the next set of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MachineGroupCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MachineGroupCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the computation of a machine group dependency map. A machine group dependency map includes all direct dependencies the machines in the group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineGroupMapRequest {
    #[serde(flatten)]
    pub multiple_machines_map_request: MultipleMachinesMapRequest,
    #[doc = "URI of machine group resource for which to generate the map."]
    #[serde(rename = "machineGroupId")]
    pub machine_group_id: String,
}
impl MachineGroupMapRequest {
    pub fn new(multiple_machines_map_request: MultipleMachinesMapRequest, machine_group_id: String) -> Self {
        Self {
            multiple_machines_map_request,
            machine_group_id,
        }
    }
}
#[doc = "Specifies the computation of a one hope dependency map for a list of machines. The resulting map includes all direct dependencies for the specified machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineListMapRequest {
    #[serde(flatten)]
    pub multiple_machines_map_request: MultipleMachinesMapRequest,
    #[doc = "a list of URIs of machine resources for which to generate the map."]
    #[serde(rename = "machineIds")]
    pub machine_ids: Vec<String>,
}
impl MachineListMapRequest {
    pub fn new(multiple_machines_map_request: MultipleMachinesMapRequest, machine_ids: Vec<String>) -> Self {
        Self {
            multiple_machines_map_request,
            machine_ids,
        }
    }
}
#[doc = "Specifies if the machine has been rebooted since the installation of the dependency agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MachineRebootStatus {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "rebooted")]
    Rebooted,
    #[serde(rename = "notRebooted")]
    NotRebooted,
}
#[doc = "Reference to a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineReference {
    #[serde(flatten)]
    pub resource_reference: ResourceReference,
}
impl MachineReference {
    pub fn new(resource_reference: ResourceReference) -> Self {
        Self { resource_reference }
    }
}
#[doc = "A machine reference with a hint of the machine's name and operating system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineReferenceWithHints {
    #[serde(flatten)]
    pub resource_reference: ResourceReference,
    #[doc = "Machine reference with name and os hints."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<machine_reference_with_hints::Properties>,
}
impl MachineReferenceWithHints {
    pub fn new(resource_reference: ResourceReference) -> Self {
        Self {
            resource_reference,
            properties: None,
        }
    }
}
pub mod machine_reference_with_hints {
    use super::*;
    #[doc = "Machine reference with name and os hints."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Last known display name."]
        #[serde(rename = "displayNameHint", default, skip_serializing_if = "Option::is_none")]
        pub display_name_hint: Option<String>,
        #[doc = "Specifies the operating system family, e.g., Linux, Windows, etc."]
        #[serde(rename = "osFamilyHint", default, skip_serializing_if = "Option::is_none")]
        pub os_family_hint: Option<OperatingSystemFamily>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Describes the resources of a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineResourcesConfiguration {
    #[doc = "Physical memory in megabytes (MB)."]
    #[serde(rename = "physicalMemory", default, skip_serializing_if = "Option::is_none")]
    pub physical_memory: Option<i32>,
    #[doc = "Number of CPUs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpus: Option<i32>,
    #[doc = "CPU speed in megahertz (Mhz)."]
    #[serde(rename = "cpuSpeed", default, skip_serializing_if = "Option::is_none")]
    pub cpu_speed: Option<i32>,
    #[doc = "Specifies the accuracy of a computation."]
    #[serde(rename = "cpuSpeedAccuracy", default, skip_serializing_if = "Option::is_none")]
    pub cpu_speed_accuracy: Option<Accuracy>,
}
impl MachineResourcesConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A summary of the machines in the workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachinesSummary {
    #[serde(flatten)]
    pub summary: Summary,
    #[doc = "Summarizes machines in the workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachinesSummaryProperties>,
}
impl MachinesSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summarizes machines in the workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachinesSummaryProperties {
    #[serde(flatten)]
    pub summary_properties: SummaryProperties,
    #[doc = "Total number of machines."]
    pub total: i32,
    #[doc = "Number of live machines."]
    pub live: i32,
    #[doc = "Machines by operating system."]
    pub os: MachineCountsByOperatingSystem,
}
impl MachinesSummaryProperties {
    pub fn new(summary_properties: SummaryProperties, total: i32, live: i32, os: MachineCountsByOperatingSystem) -> Self {
        Self {
            summary_properties,
            total,
            live,
            os,
        }
    }
}
#[doc = "A map of resources and relationships between them."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Map {
    #[doc = "The nodes (entities) of a map."]
    pub nodes: MapNodes,
    #[doc = "The edges (relationships) of a map."]
    pub edges: MapEdges,
}
impl Map {
    pub fn new(nodes: MapNodes, edges: MapEdges) -> Self {
        Self { nodes, edges }
    }
}
#[doc = "The edges (relationships) of a map."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapEdges {
    #[doc = "Network connections."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub connections: Vec<Connection>,
    #[doc = "Processes accepting on a port."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub acceptors: Vec<Acceptor>,
}
impl MapEdges {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The nodes (entities) of a map."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapNodes {
    #[doc = "Machine resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub machines: Vec<Machine>,
    #[doc = "Process resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processes: Vec<Process>,
    #[doc = "Port resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<Port>,
    #[doc = "Client Group resources."]
    #[serde(rename = "clientGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub client_groups: Vec<ClientGroup>,
}
impl MapNodes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the contents of request to generate a map."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapRequest {
    #[doc = "Map interval start time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Map interval end time."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The type of map to create."]
    pub kind: map_request::Kind,
}
impl MapRequest {
    pub fn new(kind: map_request::Kind) -> Self {
        Self {
            start_time: None,
            end_time: None,
            kind,
        }
    }
}
pub mod map_request {
    use super::*;
    #[doc = "The type of map to create."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "map:single-machine-dependency")]
        MapSingleMachineDependency,
        #[serde(rename = "map:machine-group-dependency")]
        MapMachineGroupDependency,
        #[serde(rename = "map:machine-list-dependency")]
        MapMachineListDependency,
    }
}
#[doc = "Specified the contents of a map response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapResponse {
    #[doc = "Map interval start time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "Map interval end time."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339")]
    pub end_time: time::OffsetDateTime,
    #[doc = "A map of resources and relationships between them."]
    pub map: Map,
}
impl MapResponse {
    pub fn new(start_time: time::OffsetDateTime, end_time: time::OffsetDateTime, map: Map) -> Self {
        Self { start_time, end_time, map }
    }
}
#[doc = "Used to specify if a resources is monitored or discovered."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MonitoringState {
    #[serde(rename = "monitored")]
    Monitored,
    #[serde(rename = "discovered")]
    Discovered,
}
#[doc = "Provides a base class for describing map requests for a collection of machines"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultipleMachinesMapRequest {
    #[serde(flatten)]
    pub map_request: MapRequest,
    #[doc = "If true, only processes between specified machines will be included. Any connections in or out of those processes will be included."]
    #[serde(rename = "filterProcesses", default, skip_serializing_if = "Option::is_none")]
    pub filter_processes: Option<bool>,
}
impl MultipleMachinesMapRequest {
    pub fn new(map_request: MapRequest) -> Self {
        Self {
            map_request,
            filter_processes: None,
        }
    }
}
#[doc = "Describes the network configuration of a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkConfiguration {
    #[doc = "IPv4 interfaces."]
    #[serde(rename = "ipv4Interfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub ipv4_interfaces: Vec<Ipv4NetworkInterface>,
    #[doc = "IPv6 interfaces."]
    #[serde(rename = "ipv6Interfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub ipv6_interfaces: Vec<Ipv6NetworkInterface>,
    #[doc = "Default IPv4 gateways."]
    #[serde(rename = "defaultIpv4Gateways", default, skip_serializing_if = "Vec::is_empty")]
    pub default_ipv4_gateways: Vec<String>,
    #[doc = "MAC addresses of all active network interfaces."]
    #[serde(rename = "macAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub mac_addresses: Vec<String>,
    #[doc = "DNS names associated with the machine."]
    #[serde(rename = "dnsNames", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_names: Vec<String>,
}
impl NetworkConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the configuration of the operating system of a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperatingSystemConfiguration {
    #[doc = "Specifies the operating system family, e.g., Linux, Windows, etc."]
    pub family: OperatingSystemFamily,
    #[doc = "Operating system full name."]
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[doc = "Specifies the bitness of a machine or process."]
    pub bitness: Bitness,
}
impl OperatingSystemConfiguration {
    pub fn new(family: OperatingSystemFamily, full_name: String, bitness: Bitness) -> Self {
        Self {
            family,
            full_name,
            bitness,
        }
    }
}
#[doc = "Specifies the operating system family, e.g., Linux, Windows, etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperatingSystemFamily {
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
#[doc = "A port resource represents a server port on a machine. The port may be actively *monitored*, i.e., a Dependency Agent is running on its machine, or *discovered*, i.e., its existence was inferred by observing the data stream from monitored machines. A port is live during an interval of time, if that port had associated activity during (parts) of that interval."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Port {
    #[serde(flatten)]
    pub core_resource: CoreResource,
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<port::Properties>,
}
impl Port {
    pub fn new(core_resource: CoreResource) -> Self {
        Self {
            core_resource,
            properties: None,
        }
    }
}
pub mod port {
    use super::*;
    #[doc = "Resource properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Used to specify if a resources is monitored or discovered."]
        #[serde(rename = "monitoringState", default, skip_serializing_if = "Option::is_none")]
        pub monitoring_state: Option<MonitoringState>,
        #[doc = "Represents a reference to another resource."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub machine: Option<ResourceReference>,
        #[doc = "Name to use for display purposes."]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "IP address associated with the port. At present only IPv4 addresses are supported."]
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
        pub ip_address: Option<String>,
        #[doc = "Port number."]
        #[serde(rename = "portNumber", default, skip_serializing_if = "Option::is_none")]
        pub port_number: Option<i32>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of Port resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortCollection {
    #[doc = "Collection of Port resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Port>,
    #[doc = "The URL to the next set of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PortCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PortCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to a port."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PortReference {
    #[serde(flatten)]
    pub resource_reference: ResourceReference,
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<port_reference::Properties>,
}
impl PortReference {
    pub fn new(resource_reference: ResourceReference) -> Self {
        Self {
            resource_reference,
            properties: None,
        }
    }
}
pub mod port_reference {
    use super::*;
    #[doc = "Resource properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Reference to a machine."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub machine: Option<MachineReference>,
        #[doc = "IP address of the port."]
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
        pub ip_address: Option<String>,
        #[doc = "Port number."]
        #[serde(rename = "portNumber", default, skip_serializing_if = "Option::is_none")]
        pub port_number: Option<i32>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A process resource represents a process running on a machine. The process may be actively *monitored*, i.e., a Dependency Agent is running on its machine, or *discovered*, i.e., its existence was inferred by observing the data stream from monitored machines. A process resource represents a pool of actual operating system resources that share command lines and metadata. As the process pool evolves over time, prior versions of the process resource are preserved and available for access. A process is live during an interval of time, if that process is executing during (parts) of that interval"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Process {
    #[serde(flatten)]
    pub core_resource: CoreResource,
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<process::Properties>,
}
impl Process {
    pub fn new(core_resource: CoreResource) -> Self {
        Self {
            core_resource,
            properties: None,
        }
    }
}
pub mod process {
    use super::*;
    #[doc = "Resource properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "UTC date and time when this process resource was updated in the system"]
        #[serde(with = "azure_core::date::rfc3339::option")]
        pub timestamp: Option<time::OffsetDateTime>,
        #[doc = "Used to specify if a resources is monitored or discovered."]
        #[serde(rename = "monitoringState", default, skip_serializing_if = "Option::is_none")]
        pub monitoring_state: Option<MonitoringState>,
        #[doc = "Represents a reference to another resource."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub machine: Option<ResourceReference>,
        #[doc = "The name of the process executable"]
        #[serde(rename = "executableName", default, skip_serializing_if = "Option::is_none")]
        pub executable_name: Option<String>,
        #[doc = "Name to use for display purposes"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "UTC date and time when the process started"]
        #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
        pub start_time: Option<time::OffsetDateTime>,
        #[doc = "The inferred role of this process based on its name, command line, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub role: Option<properties::Role>,
        #[doc = "The name of the product or suite of the process. The group is determined by its executable name, command line, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub group: Option<String>,
        #[doc = "Describes process metadata."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub details: Option<ProcessDetails>,
        #[doc = "Describes the user under which a process is running."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub user: Option<ProcessUser>,
        #[doc = "Represents a reference to another resource."]
        #[serde(rename = "clientOf", default, skip_serializing_if = "Option::is_none")]
        pub client_of: Option<ResourceReference>,
        #[doc = "Represents a reference to another resource."]
        #[serde(rename = "acceptorOf", default, skip_serializing_if = "Option::is_none")]
        pub acceptor_of: Option<ResourceReference>,
        #[doc = "Describes the hosting configuration of a process."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hosting: Option<ProcessHostingConfiguration>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "The inferred role of this process based on its name, command line, etc."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Role")]
        pub enum Role {
            #[serde(rename = "webServer")]
            WebServer,
            #[serde(rename = "appServer")]
            AppServer,
            #[serde(rename = "databaseServer")]
            DatabaseServer,
            #[serde(rename = "ldapServer")]
            LdapServer,
            #[serde(rename = "smbServer")]
            SmbServer,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for Role {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for Role {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for Role {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::WebServer => serializer.serialize_unit_variant("Role", 0u32, "webServer"),
                    Self::AppServer => serializer.serialize_unit_variant("Role", 1u32, "appServer"),
                    Self::DatabaseServer => serializer.serialize_unit_variant("Role", 2u32, "databaseServer"),
                    Self::LdapServer => serializer.serialize_unit_variant("Role", 3u32, "ldapServer"),
                    Self::SmbServer => serializer.serialize_unit_variant("Role", 4u32, "smbServer"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Collection of Process resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessCollection {
    #[doc = "Collection of Process resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Process>,
    #[doc = "The URL to the next set of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProcessCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProcessCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes process metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessDetails {
    #[doc = "A unique identifier for a process, generally resilient to process restart, computed by Service Map."]
    #[serde(rename = "persistentKey", default, skip_serializing_if = "Option::is_none")]
    pub persistent_key: Option<String>,
    #[doc = "Represents the identity of the process pool assigned to the process by Dependency Agent."]
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<i32>,
    #[doc = "The Operating System Process Identifier (PID) of the first process in this process pool."]
    #[serde(rename = "firstPid", default, skip_serializing_if = "Option::is_none")]
    pub first_pid: Option<i32>,
    #[doc = "Process description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name of company that created the process executable."]
    #[serde(rename = "companyName", default, skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[doc = "Internal process name."]
    #[serde(rename = "internalName", default, skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[doc = "Product name."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "Product version."]
    #[serde(rename = "productVersion", default, skip_serializing_if = "Option::is_none")]
    pub product_version: Option<String>,
    #[doc = "File version."]
    #[serde(rename = "fileVersion", default, skip_serializing_if = "Option::is_none")]
    pub file_version: Option<String>,
    #[doc = "Process command line."]
    #[serde(rename = "commandLine", default, skip_serializing_if = "Option::is_none")]
    pub command_line: Option<String>,
    #[doc = "Process executable path."]
    #[serde(rename = "executablePath", default, skip_serializing_if = "Option::is_none")]
    pub executable_path: Option<String>,
    #[doc = "Process workingDirectory."]
    #[serde(rename = "workingDirectory", default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    #[doc = "Collection of services hosted by this Process (Windows only)."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<ProcessHostedService>,
    #[doc = "Process zone name (Linux only)."]
    #[serde(rename = "zoneName", default, skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
}
impl ProcessDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A service hosted by a process."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessHostedService {
    #[doc = "The name of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The service's display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl ProcessHostedService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the hosting configuration of a process."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessHostingConfiguration {
    #[doc = "The hosting provider of the VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<process_hosting_configuration::Provider>,
    #[doc = "Additional hosting configuration type qualifier."]
    pub kind: process_hosting_configuration::Kind,
}
impl ProcessHostingConfiguration {
    pub fn new(kind: process_hosting_configuration::Kind) -> Self {
        Self { provider: None, kind }
    }
}
pub mod process_hosting_configuration {
    use super::*;
    #[doc = "The hosting provider of the VM."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Provider {
        #[serde(rename = "azure")]
        Azure,
    }
    #[doc = "Additional hosting configuration type qualifier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "provider:azure")]
        ProviderAzure,
    }
}
#[doc = "Reference to a process."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessReference {
    #[serde(flatten)]
    pub resource_reference: ResourceReference,
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<process_reference::Properties>,
}
impl ProcessReference {
    pub fn new(resource_reference: ResourceReference) -> Self {
        Self {
            resource_reference,
            properties: None,
        }
    }
}
pub mod process_reference {
    use super::*;
    #[doc = "Resource properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Reference to a machine."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub machine: Option<MachineReference>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Describes the user under which a process is running."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessUser {
    #[doc = "User name under which the process is running."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "Domain name for the user."]
    #[serde(rename = "userDomain", default, skip_serializing_if = "Option::is_none")]
    pub user_domain: Option<String>,
}
impl ProcessUser {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A typed relationship between two entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Relationship {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Additional resource type qualifier."]
    pub kind: relationship::Kind,
}
impl Relationship {
    pub fn new(kind: relationship::Kind) -> Self {
        Self {
            resource: Resource::default(),
            kind,
        }
    }
}
pub mod relationship {
    use super::*;
    #[doc = "Additional resource type qualifier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "rel:connection")]
        RelConnection,
        #[serde(rename = "rel:acceptor")]
        RelAcceptor,
    }
}
#[doc = "Relationship properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipProperties {
    #[doc = "Represents a reference to another resource."]
    pub source: ResourceReference,
    #[doc = "Represents a reference to another resource."]
    pub destination: ResourceReference,
    #[doc = "Relationship start time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Relationship end time."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}
impl RelationshipProperties {
    pub fn new(source: ResourceReference, destination: ResourceReference) -> Self {
        Self {
            source,
            destination,
            start_time: None,
            end_time: None,
        }
    }
}
#[doc = "Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a reference to another resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceReference {
    #[doc = "Resource URI."]
    pub id: String,
    #[doc = "Resource type qualifier."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the sub-class of the reference."]
    pub kind: resource_reference::Kind,
}
impl ResourceReference {
    pub fn new(id: String, kind: resource_reference::Kind) -> Self {
        Self {
            id,
            type_: None,
            name: None,
            kind,
        }
    }
}
pub mod resource_reference {
    use super::*;
    #[doc = "Specifies the sub-class of the reference."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "ref:machine")]
        RefMachine,
        #[serde(rename = "ref:machinewithhints")]
        RefMachinewithhints,
        #[serde(rename = "ref:process")]
        RefProcess,
        #[serde(rename = "ref:port")]
        RefPort,
        #[serde(rename = "ref:onmachine")]
        RefOnmachine,
        #[serde(rename = "ref:clientgroup")]
        RefClientgroup,
    }
}
#[doc = "Specifies the computation of a single server dependency map. A single server dependency map includes all direct dependencies of a given machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleMachineDependencyMapRequest {
    #[serde(flatten)]
    pub map_request: MapRequest,
    #[doc = "URI of machine resource for which to generate the map."]
    #[serde(rename = "machineId")]
    pub machine_id: String,
}
impl SingleMachineDependencyMapRequest {
    pub fn new(map_request: MapRequest, machine_id: String) -> Self {
        Self { map_request, machine_id }
    }
}
#[doc = "Base for all resource summaries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Summary {
    #[serde(flatten)]
    pub resource: Resource,
}
impl Summary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base for all summaries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SummaryProperties {
    #[doc = "Summary interval start time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "Summary interval end time."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339")]
    pub end_time: time::OffsetDateTime,
}
impl SummaryProperties {
    pub fn new(start_time: time::OffsetDateTime, end_time: time::OffsetDateTime) -> Self {
        Self { start_time, end_time }
    }
}
#[doc = "Describes a timezone."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Timezone {
    #[doc = "Timezone full name."]
    #[serde(rename = "fullName", default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
}
impl Timezone {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the virtualization-related configuration of a machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineConfiguration {
    #[doc = "Specifies the virtualization type of a machine."]
    #[serde(rename = "virtualMachineType", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_type: Option<VirtualMachineType>,
    #[doc = "The unique identifier of the virtual machine as reported by the underlying virtualization system."]
    #[serde(rename = "nativeMachineId", default, skip_serializing_if = "Option::is_none")]
    pub native_machine_id: Option<String>,
    #[doc = "The Name of the virtual machine."]
    #[serde(rename = "virtualMachineName", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_name: Option<String>,
    #[doc = "The unique identifier of the host of this virtual machine as reported by the underlying virtualization system."]
    #[serde(rename = "nativeHostMachineId", default, skip_serializing_if = "Option::is_none")]
    pub native_host_machine_id: Option<String>,
}
impl VirtualMachineConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the virtualization type of a machine."]
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
#[doc = "Specifies if the machine is physical, virtual, hypervisor, or unknown."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VirtualizationState {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "physical")]
    Physical,
    #[serde(rename = "virtual")]
    Virtual,
    #[serde(rename = "hypervisor")]
    Hypervisor,
}

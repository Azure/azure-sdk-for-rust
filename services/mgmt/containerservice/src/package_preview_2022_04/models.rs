#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Profile for enabling a user to access a managed cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessProfile {
    #[doc = "Base64-encoded Kubernetes configuration file."]
    #[serde(rename = "kubeConfig", default, skip_serializing_if = "Option::is_none")]
    pub kube_config: Option<String>,
}
impl AccessProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Agent Pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPool {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties for the container service agent pool profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedClusterAgentPoolProfileProperties>,
}
impl AgentPool {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of available versions for an agent pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentPoolAvailableVersions {
    #[doc = "The ID of the agent pool version list."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the agent pool version list."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the agent pool version list."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The list of available agent pool versions."]
    pub properties: AgentPoolAvailableVersionsProperties,
}
impl AgentPoolAvailableVersions {
    pub fn new(properties: AgentPoolAvailableVersionsProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "The list of available agent pool versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolAvailableVersionsProperties {
    #[doc = "List of versions available for agent pool."]
    #[serde(rename = "agentPoolVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub agent_pool_versions: Vec<serde_json::Value>,
}
impl AgentPoolAvailableVersionsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from the List Agent Pools operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolListResult {
    #[doc = "The list of agent pools."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AgentPool>,
    #[doc = "The URL to get the next set of agent pool results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AgentPoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AgentPoolListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A cluster must have at least one 'System' Agent Pool at all times. For additional information on agent pool restrictions and best practices, see: https://docs.microsoft.com/azure/aks/use-system-pools"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AgentPoolMode")]
pub enum AgentPoolMode {
    System,
    User,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AgentPoolMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AgentPoolMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AgentPoolMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::System => serializer.serialize_unit_variant("AgentPoolMode", 0u32, "System"),
            Self::User => serializer.serialize_unit_variant("AgentPoolMode", 1u32, "User"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type of Agent Pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AgentPoolType")]
pub enum AgentPoolType {
    VirtualMachineScaleSets,
    AvailabilitySet,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AgentPoolType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AgentPoolType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AgentPoolType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::VirtualMachineScaleSets => serializer.serialize_unit_variant("AgentPoolType", 0u32, "VirtualMachineScaleSets"),
            Self::AvailabilitySet => serializer.serialize_unit_variant("AgentPoolType", 1u32, "AvailabilitySet"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The list of available upgrades for an agent pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentPoolUpgradeProfile {
    #[doc = "The ID of the agent pool upgrade profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the agent pool upgrade profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the agent pool upgrade profile."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The list of available upgrade versions."]
    pub properties: AgentPoolUpgradeProfileProperties,
}
impl AgentPoolUpgradeProfile {
    pub fn new(properties: AgentPoolUpgradeProfileProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "The list of available upgrade versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentPoolUpgradeProfileProperties {
    #[doc = "The Kubernetes version (major.minor.patch)."]
    #[serde(rename = "kubernetesVersion")]
    pub kubernetes_version: String,
    #[doc = "The operating system type. The default is Linux."]
    #[serde(rename = "osType")]
    pub os_type: OsType,
    #[doc = "List of orchestrator types and versions available for upgrade."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub upgrades: Vec<serde_json::Value>,
    #[doc = "The latest AKS supported node image version."]
    #[serde(rename = "latestNodeImageVersion", default, skip_serializing_if = "Option::is_none")]
    pub latest_node_image_version: Option<String>,
}
impl AgentPoolUpgradeProfileProperties {
    pub fn new(kubernetes_version: String, os_type: OsType) -> Self {
        Self {
            kubernetes_version,
            os_type,
            upgrades: Vec::new(),
            latest_node_image_version: None,
        }
    }
}
#[doc = "Settings for upgrading an agentpool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolUpgradeSettings {
    #[doc = "This can either be set to an integer (e.g. '5') or a percentage (e.g. '50%'). If a percentage is specified, it is the percentage of the total agent pool size at the time of the upgrade. For percentages, fractional nodes are rounded up. If not specified, the default is 1. For more information, including best practices, see: https://docs.microsoft.com/azure/aks/upgrade-cluster#customize-node-surge-upgrade"]
    #[serde(rename = "maxSurge", default, skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<String>,
}
impl AgentPoolUpgradeSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Key Vault key management service settings for the security profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureKeyVaultKms {
    #[doc = "Whether to enable Azure Key Vault key management service. The default is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Identifier of Azure Key Vault key. See [key identifier format](https://docs.microsoft.com/en-us/azure/key-vault/general/about-keys-secrets-certificates#vault-name-and-object-name) for more details. When Azure Key Vault key management service is enabled, this field is required and must be a valid key identifier. When Azure Key Vault key management service is disabled, leave the field empty."]
    #[serde(rename = "keyId", default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}
impl AzureKeyVaultKms {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type CapacityReservationGroupId = String;
#[doc = "An error response from the Container service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the Container service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl azure_core::Continuable for CloudError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the Container service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The results of a run command"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommandResultProperties {
    #[doc = "provisioning State"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The exit code of the command"]
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[doc = "The time when the command started."]
    #[serde(rename = "startedAt", default, with = "azure_core::date::rfc3339::option")]
    pub started_at: Option<time::OffsetDateTime>,
    #[doc = "The time when the command finished."]
    #[serde(rename = "finishedAt", default, with = "azure_core::date::rfc3339::option")]
    pub finished_at: Option<time::OffsetDateTime>,
    #[doc = "The command output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logs: Option<String>,
    #[doc = "An explanation of why provisioningState is set to failed (if so)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl CommandResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Profile for diagnostics on the container service cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceDiagnosticsProfile {
    #[doc = "Profile for diagnostics on the container service VMs."]
    #[serde(rename = "vmDiagnostics")]
    pub vm_diagnostics: ContainerServiceVmDiagnostics,
}
impl ContainerServiceDiagnosticsProfile {
    pub fn new(vm_diagnostics: ContainerServiceVmDiagnostics) -> Self {
        Self { vm_diagnostics }
    }
}
#[doc = "Profile for Linux VMs in the container service cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceLinuxProfile {
    #[doc = "The administrator username to use for Linux VMs."]
    #[serde(rename = "adminUsername")]
    pub admin_username: String,
    #[doc = "SSH configuration for Linux-based VMs running on Azure."]
    pub ssh: ContainerServiceSshConfiguration,
}
impl ContainerServiceLinuxProfile {
    pub fn new(admin_username: String, ssh: ContainerServiceSshConfiguration) -> Self {
        Self { admin_username, ssh }
    }
}
#[doc = "Profile for the container service master."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceMasterProfile {
    #[doc = "Number of masters (VMs) in the container service cluster. Allowed values are 1, 3, and 5. The default value is 1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<container_service_master_profile::Count>,
    #[doc = "DNS prefix to be used to create the FQDN for the master pool."]
    #[serde(rename = "dnsPrefix")]
    pub dns_prefix: String,
    #[doc = "Size of agent VMs. Note: This is no longer maintained."]
    #[serde(rename = "vmSize")]
    pub vm_size: ContainerServiceVmSize,
    #[doc = "OS Disk Size in GB to be used to specify the disk size for every machine in the master/agent pool. If you specify 0, it will apply the default osDisk size according to the vmSize specified."]
    #[serde(rename = "osDiskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_size_gb: Option<ContainerServiceOsDisk>,
    #[doc = "specifies a subnet's resource id with subscription, resource group, vnet and subnet name"]
    #[serde(rename = "vnetSubnetID", default, skip_serializing_if = "Option::is_none")]
    pub vnet_subnet_id: Option<ContainerServiceVnetSubnetId>,
    #[doc = "FirstConsecutiveStaticIP used to specify the first static ip of masters."]
    #[serde(rename = "firstConsecutiveStaticIP", default, skip_serializing_if = "Option::is_none")]
    pub first_consecutive_static_ip: Option<String>,
    #[doc = "Specifies what kind of storage to use. If omitted, the default will be chosen on your behalf based on the choice of orchestrator."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<ContainerServiceStorageProfile>,
    #[doc = "FQDN for the master pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
}
impl ContainerServiceMasterProfile {
    pub fn new(dns_prefix: String, vm_size: ContainerServiceVmSize) -> Self {
        Self {
            count: None,
            dns_prefix,
            vm_size,
            os_disk_size_gb: None,
            vnet_subnet_id: None,
            first_consecutive_static_ip: None,
            storage_profile: None,
            fqdn: None,
        }
    }
}
pub mod container_service_master_profile {
    use super::*;
    #[doc = "Number of masters (VMs) in the container service cluster. Allowed values are 1, 3, and 5. The default value is 1."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Count {}
}
#[doc = "Profile of network configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerServiceNetworkProfile {
    #[doc = "Network plugin used for building the Kubernetes network."]
    #[serde(rename = "networkPlugin", default, skip_serializing_if = "Option::is_none")]
    pub network_plugin: Option<NetworkPlugin>,
    #[doc = "The mode the network plugin should use."]
    #[serde(rename = "networkPluginMode", default, skip_serializing_if = "Option::is_none")]
    pub network_plugin_mode: Option<NetworkPluginMode>,
    #[doc = "Network policy used for building the Kubernetes network."]
    #[serde(rename = "networkPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_policy: Option<NetworkPolicy>,
    #[doc = "This cannot be specified if networkPlugin is anything other than 'azure'."]
    #[serde(rename = "networkMode", default, skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<NetworkMode>,
    #[doc = "A CIDR notation IP range from which to assign pod IPs when kubenet is used."]
    #[serde(rename = "podCidr", default, skip_serializing_if = "Option::is_none")]
    pub pod_cidr: Option<String>,
    #[doc = "A CIDR notation IP range from which to assign service cluster IPs. It must not overlap with any Subnet IP ranges."]
    #[serde(rename = "serviceCidr", default, skip_serializing_if = "Option::is_none")]
    pub service_cidr: Option<String>,
    #[doc = "An IP address assigned to the Kubernetes DNS service. It must be within the Kubernetes service address range specified in serviceCidr."]
    #[serde(rename = "dnsServiceIP", default, skip_serializing_if = "Option::is_none")]
    pub dns_service_ip: Option<String>,
    #[doc = "A CIDR notation IP range assigned to the Docker bridge network. It must not overlap with any Subnet IP ranges or the Kubernetes service address range."]
    #[serde(rename = "dockerBridgeCidr", default, skip_serializing_if = "Option::is_none")]
    pub docker_bridge_cidr: Option<String>,
    #[doc = "This can only be set at cluster creation time and cannot be changed later. For more information see [egress outbound type](https://docs.microsoft.com/azure/aks/egress-outboundtype)."]
    #[serde(rename = "outboundType", default, skip_serializing_if = "Option::is_none")]
    pub outbound_type: Option<container_service_network_profile::OutboundType>,
    #[doc = "The default is 'standard'. See [Azure Load Balancer SKUs](https://docs.microsoft.com/azure/load-balancer/skus) for more information about the differences between load balancer SKUs."]
    #[serde(rename = "loadBalancerSku", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_sku: Option<LoadBalancerSku>,
    #[doc = "Profile of the managed cluster load balancer."]
    #[serde(rename = "loadBalancerProfile", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_profile: Option<ManagedClusterLoadBalancerProfile>,
    #[doc = "Profile of the managed cluster NAT gateway."]
    #[serde(rename = "natGatewayProfile", default, skip_serializing_if = "Option::is_none")]
    pub nat_gateway_profile: Option<ManagedClusterNatGatewayProfile>,
    #[doc = "One IPv4 CIDR is expected for single-stack networking. Two CIDRs, one for each IP family (IPv4/IPv6), is expected for dual-stack networking."]
    #[serde(rename = "podCidrs", default, skip_serializing_if = "Vec::is_empty")]
    pub pod_cidrs: Vec<String>,
    #[doc = "One IPv4 CIDR is expected for single-stack networking. Two CIDRs, one for each IP family (IPv4/IPv6), is expected for dual-stack networking. They must not overlap with any Subnet IP ranges."]
    #[serde(rename = "serviceCidrs", default, skip_serializing_if = "Vec::is_empty")]
    pub service_cidrs: Vec<String>,
    #[doc = "IP families are used to determine single-stack or dual-stack clusters. For single-stack, the expected value is IPv4. For dual-stack, the expected values are IPv4 and IPv6."]
    #[serde(rename = "ipFamilies", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_families: Vec<String>,
}
impl ContainerServiceNetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod container_service_network_profile {
    use super::*;
    #[doc = "This can only be set at cluster creation time and cannot be changed later. For more information see [egress outbound type](https://docs.microsoft.com/azure/aks/egress-outboundtype)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OutboundType")]
    pub enum OutboundType {
        #[serde(rename = "loadBalancer")]
        LoadBalancer,
        #[serde(rename = "userDefinedRouting")]
        UserDefinedRouting,
        #[serde(rename = "managedNATGateway")]
        ManagedNatGateway,
        #[serde(rename = "userAssignedNATGateway")]
        UserAssignedNatGateway,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OutboundType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OutboundType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OutboundType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::LoadBalancer => serializer.serialize_unit_variant("OutboundType", 0u32, "loadBalancer"),
                Self::UserDefinedRouting => serializer.serialize_unit_variant("OutboundType", 1u32, "userDefinedRouting"),
                Self::ManagedNatGateway => serializer.serialize_unit_variant("OutboundType", 2u32, "managedNATGateway"),
                Self::UserAssignedNatGateway => serializer.serialize_unit_variant("OutboundType", 3u32, "userAssignedNATGateway"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for OutboundType {
        fn default() -> Self {
            Self::LoadBalancer
        }
    }
}
pub type ContainerServiceOsDisk = i32;
#[doc = "SSH configuration for Linux-based VMs running on Azure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceSshConfiguration {
    #[doc = "The list of SSH public keys used to authenticate with Linux-based VMs. A maximum of 1 key may be specified."]
    #[serde(rename = "publicKeys")]
    pub public_keys: Vec<ContainerServiceSshPublicKey>,
}
impl ContainerServiceSshConfiguration {
    pub fn new(public_keys: Vec<ContainerServiceSshPublicKey>) -> Self {
        Self { public_keys }
    }
}
#[doc = "Contains information about SSH certificate public key data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceSshPublicKey {
    #[doc = "Certificate public key used to authenticate with VMs through SSH. The certificate must be in PEM format with or without headers."]
    #[serde(rename = "keyData")]
    pub key_data: String,
}
impl ContainerServiceSshPublicKey {
    pub fn new(key_data: String) -> Self {
        Self { key_data }
    }
}
#[doc = "Specifies what kind of storage to use. If omitted, the default will be chosen on your behalf based on the choice of orchestrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ContainerServiceStorageProfile")]
pub enum ContainerServiceStorageProfile {
    StorageAccount,
    ManagedDisks,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ContainerServiceStorageProfile {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ContainerServiceStorageProfile {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ContainerServiceStorageProfile {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::StorageAccount => serializer.serialize_unit_variant("ContainerServiceStorageProfile", 0u32, "StorageAccount"),
            Self::ManagedDisks => serializer.serialize_unit_variant("ContainerServiceStorageProfile", 1u32, "ManagedDisks"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Profile for diagnostics on the container service VMs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceVmDiagnostics {
    #[doc = "Whether the VM diagnostic agent is provisioned on the VM."]
    pub enabled: bool,
    #[doc = "The URI of the storage account where diagnostics are stored."]
    #[serde(rename = "storageUri", default, skip_serializing_if = "Option::is_none")]
    pub storage_uri: Option<String>,
}
impl ContainerServiceVmDiagnostics {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            storage_uri: None,
        }
    }
}
#[doc = "Size of agent VMs. Note: This is no longer maintained."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ContainerServiceVmSize")]
pub enum ContainerServiceVmSize {
    #[serde(rename = "Standard_A1")]
    StandardA1,
    #[serde(rename = "Standard_A10")]
    StandardA10,
    #[serde(rename = "Standard_A11")]
    StandardA11,
    #[serde(rename = "Standard_A1_v2")]
    StandardA1V2,
    #[serde(rename = "Standard_A2")]
    StandardA2,
    #[serde(rename = "Standard_A2_v2")]
    StandardA2V2,
    #[serde(rename = "Standard_A2m_v2")]
    StandardA2mV2,
    #[serde(rename = "Standard_A3")]
    StandardA3,
    #[serde(rename = "Standard_A4")]
    StandardA4,
    #[serde(rename = "Standard_A4_v2")]
    StandardA4V2,
    #[serde(rename = "Standard_A4m_v2")]
    StandardA4mV2,
    #[serde(rename = "Standard_A5")]
    StandardA5,
    #[serde(rename = "Standard_A6")]
    StandardA6,
    #[serde(rename = "Standard_A7")]
    StandardA7,
    #[serde(rename = "Standard_A8")]
    StandardA8,
    #[serde(rename = "Standard_A8_v2")]
    StandardA8V2,
    #[serde(rename = "Standard_A8m_v2")]
    StandardA8mV2,
    #[serde(rename = "Standard_A9")]
    StandardA9,
    #[serde(rename = "Standard_B2ms")]
    StandardB2ms,
    #[serde(rename = "Standard_B2s")]
    StandardB2s,
    #[serde(rename = "Standard_B4ms")]
    StandardB4ms,
    #[serde(rename = "Standard_B8ms")]
    StandardB8ms,
    #[serde(rename = "Standard_D1")]
    StandardD1,
    #[serde(rename = "Standard_D11")]
    StandardD11,
    #[serde(rename = "Standard_D11_v2")]
    StandardD11V2,
    #[serde(rename = "Standard_D11_v2_Promo")]
    StandardD11V2Promo,
    #[serde(rename = "Standard_D12")]
    StandardD12,
    #[serde(rename = "Standard_D12_v2")]
    StandardD12V2,
    #[serde(rename = "Standard_D12_v2_Promo")]
    StandardD12V2Promo,
    #[serde(rename = "Standard_D13")]
    StandardD13,
    #[serde(rename = "Standard_D13_v2")]
    StandardD13V2,
    #[serde(rename = "Standard_D13_v2_Promo")]
    StandardD13V2Promo,
    #[serde(rename = "Standard_D14")]
    StandardD14,
    #[serde(rename = "Standard_D14_v2")]
    StandardD14V2,
    #[serde(rename = "Standard_D14_v2_Promo")]
    StandardD14V2Promo,
    #[serde(rename = "Standard_D15_v2")]
    StandardD15V2,
    #[serde(rename = "Standard_D16_v3")]
    StandardD16V3,
    #[serde(rename = "Standard_D16s_v3")]
    StandardD16sV3,
    #[serde(rename = "Standard_D1_v2")]
    StandardD1V2,
    #[serde(rename = "Standard_D2")]
    StandardD2,
    #[serde(rename = "Standard_D2_v2")]
    StandardD2V2,
    #[serde(rename = "Standard_D2_v2_Promo")]
    StandardD2V2Promo,
    #[serde(rename = "Standard_D2_v3")]
    StandardD2V3,
    #[serde(rename = "Standard_D2s_v3")]
    StandardD2sV3,
    #[serde(rename = "Standard_D3")]
    StandardD3,
    #[serde(rename = "Standard_D32_v3")]
    StandardD32V3,
    #[serde(rename = "Standard_D32s_v3")]
    StandardD32sV3,
    #[serde(rename = "Standard_D3_v2")]
    StandardD3V2,
    #[serde(rename = "Standard_D3_v2_Promo")]
    StandardD3V2Promo,
    #[serde(rename = "Standard_D4")]
    StandardD4,
    #[serde(rename = "Standard_D4_v2")]
    StandardD4V2,
    #[serde(rename = "Standard_D4_v2_Promo")]
    StandardD4V2Promo,
    #[serde(rename = "Standard_D4_v3")]
    StandardD4V3,
    #[serde(rename = "Standard_D4s_v3")]
    StandardD4sV3,
    #[serde(rename = "Standard_D5_v2")]
    StandardD5V2,
    #[serde(rename = "Standard_D5_v2_Promo")]
    StandardD5V2Promo,
    #[serde(rename = "Standard_D64_v3")]
    StandardD64V3,
    #[serde(rename = "Standard_D64s_v3")]
    StandardD64sV3,
    #[serde(rename = "Standard_D8_v3")]
    StandardD8V3,
    #[serde(rename = "Standard_D8s_v3")]
    StandardD8sV3,
    #[serde(rename = "Standard_DS1")]
    StandardDs1,
    #[serde(rename = "Standard_DS11")]
    StandardDs11,
    #[serde(rename = "Standard_DS11_v2")]
    StandardDs11V2,
    #[serde(rename = "Standard_DS11_v2_Promo")]
    StandardDs11V2Promo,
    #[serde(rename = "Standard_DS12")]
    StandardDs12,
    #[serde(rename = "Standard_DS12_v2")]
    StandardDs12V2,
    #[serde(rename = "Standard_DS12_v2_Promo")]
    StandardDs12V2Promo,
    #[serde(rename = "Standard_DS13")]
    StandardDs13,
    #[serde(rename = "Standard_DS13-2_v2")]
    StandardDs132V2,
    #[serde(rename = "Standard_DS13-4_v2")]
    StandardDs134V2,
    #[serde(rename = "Standard_DS13_v2")]
    StandardDs13V2,
    #[serde(rename = "Standard_DS13_v2_Promo")]
    StandardDs13V2Promo,
    #[serde(rename = "Standard_DS14")]
    StandardDs14,
    #[serde(rename = "Standard_DS14-4_v2")]
    StandardDs144V2,
    #[serde(rename = "Standard_DS14-8_v2")]
    StandardDs148V2,
    #[serde(rename = "Standard_DS14_v2")]
    StandardDs14V2,
    #[serde(rename = "Standard_DS14_v2_Promo")]
    StandardDs14V2Promo,
    #[serde(rename = "Standard_DS15_v2")]
    StandardDs15V2,
    #[serde(rename = "Standard_DS1_v2")]
    StandardDs1V2,
    #[serde(rename = "Standard_DS2")]
    StandardDs2,
    #[serde(rename = "Standard_DS2_v2")]
    StandardDs2V2,
    #[serde(rename = "Standard_DS2_v2_Promo")]
    StandardDs2V2Promo,
    #[serde(rename = "Standard_DS3")]
    StandardDs3,
    #[serde(rename = "Standard_DS3_v2")]
    StandardDs3V2,
    #[serde(rename = "Standard_DS3_v2_Promo")]
    StandardDs3V2Promo,
    #[serde(rename = "Standard_DS4")]
    StandardDs4,
    #[serde(rename = "Standard_DS4_v2")]
    StandardDs4V2,
    #[serde(rename = "Standard_DS4_v2_Promo")]
    StandardDs4V2Promo,
    #[serde(rename = "Standard_DS5_v2")]
    StandardDs5V2,
    #[serde(rename = "Standard_DS5_v2_Promo")]
    StandardDs5V2Promo,
    #[serde(rename = "Standard_E16_v3")]
    StandardE16V3,
    #[serde(rename = "Standard_E16s_v3")]
    StandardE16sV3,
    #[serde(rename = "Standard_E2_v3")]
    StandardE2V3,
    #[serde(rename = "Standard_E2s_v3")]
    StandardE2sV3,
    #[serde(rename = "Standard_E32-16s_v3")]
    StandardE3216sV3,
    #[serde(rename = "Standard_E32-8s_v3")]
    StandardE328sV3,
    #[serde(rename = "Standard_E32_v3")]
    StandardE32V3,
    #[serde(rename = "Standard_E32s_v3")]
    StandardE32sV3,
    #[serde(rename = "Standard_E4_v3")]
    StandardE4V3,
    #[serde(rename = "Standard_E4s_v3")]
    StandardE4sV3,
    #[serde(rename = "Standard_E64-16s_v3")]
    StandardE6416sV3,
    #[serde(rename = "Standard_E64-32s_v3")]
    StandardE6432sV3,
    #[serde(rename = "Standard_E64_v3")]
    StandardE64V3,
    #[serde(rename = "Standard_E64s_v3")]
    StandardE64sV3,
    #[serde(rename = "Standard_E8_v3")]
    StandardE8V3,
    #[serde(rename = "Standard_E8s_v3")]
    StandardE8sV3,
    #[serde(rename = "Standard_F1")]
    StandardF1,
    #[serde(rename = "Standard_F16")]
    StandardF16,
    #[serde(rename = "Standard_F16s")]
    StandardF16s,
    #[serde(rename = "Standard_F16s_v2")]
    StandardF16sV2,
    #[serde(rename = "Standard_F1s")]
    StandardF1s,
    #[serde(rename = "Standard_F2")]
    StandardF2,
    #[serde(rename = "Standard_F2s")]
    StandardF2s,
    #[serde(rename = "Standard_F2s_v2")]
    StandardF2sV2,
    #[serde(rename = "Standard_F32s_v2")]
    StandardF32sV2,
    #[serde(rename = "Standard_F4")]
    StandardF4,
    #[serde(rename = "Standard_F4s")]
    StandardF4s,
    #[serde(rename = "Standard_F4s_v2")]
    StandardF4sV2,
    #[serde(rename = "Standard_F64s_v2")]
    StandardF64sV2,
    #[serde(rename = "Standard_F72s_v2")]
    StandardF72sV2,
    #[serde(rename = "Standard_F8")]
    StandardF8,
    #[serde(rename = "Standard_F8s")]
    StandardF8s,
    #[serde(rename = "Standard_F8s_v2")]
    StandardF8sV2,
    #[serde(rename = "Standard_G1")]
    StandardG1,
    #[serde(rename = "Standard_G2")]
    StandardG2,
    #[serde(rename = "Standard_G3")]
    StandardG3,
    #[serde(rename = "Standard_G4")]
    StandardG4,
    #[serde(rename = "Standard_G5")]
    StandardG5,
    #[serde(rename = "Standard_GS1")]
    StandardGs1,
    #[serde(rename = "Standard_GS2")]
    StandardGs2,
    #[serde(rename = "Standard_GS3")]
    StandardGs3,
    #[serde(rename = "Standard_GS4")]
    StandardGs4,
    #[serde(rename = "Standard_GS4-4")]
    StandardGs44,
    #[serde(rename = "Standard_GS4-8")]
    StandardGs48,
    #[serde(rename = "Standard_GS5")]
    StandardGs5,
    #[serde(rename = "Standard_GS5-16")]
    StandardGs516,
    #[serde(rename = "Standard_GS5-8")]
    StandardGs58,
    #[serde(rename = "Standard_H16")]
    StandardH16,
    #[serde(rename = "Standard_H16m")]
    StandardH16m,
    #[serde(rename = "Standard_H16mr")]
    StandardH16mr,
    #[serde(rename = "Standard_H16r")]
    StandardH16r,
    #[serde(rename = "Standard_H8")]
    StandardH8,
    #[serde(rename = "Standard_H8m")]
    StandardH8m,
    #[serde(rename = "Standard_L16s")]
    StandardL16s,
    #[serde(rename = "Standard_L32s")]
    StandardL32s,
    #[serde(rename = "Standard_L4s")]
    StandardL4s,
    #[serde(rename = "Standard_L8s")]
    StandardL8s,
    #[serde(rename = "Standard_M128-32ms")]
    StandardM12832ms,
    #[serde(rename = "Standard_M128-64ms")]
    StandardM12864ms,
    #[serde(rename = "Standard_M128ms")]
    StandardM128ms,
    #[serde(rename = "Standard_M128s")]
    StandardM128s,
    #[serde(rename = "Standard_M64-16ms")]
    StandardM6416ms,
    #[serde(rename = "Standard_M64-32ms")]
    StandardM6432ms,
    #[serde(rename = "Standard_M64ms")]
    StandardM64ms,
    #[serde(rename = "Standard_M64s")]
    StandardM64s,
    #[serde(rename = "Standard_NC12")]
    StandardNc12,
    #[serde(rename = "Standard_NC12s_v2")]
    StandardNc12sV2,
    #[serde(rename = "Standard_NC12s_v3")]
    StandardNc12sV3,
    #[serde(rename = "Standard_NC24")]
    StandardNc24,
    #[serde(rename = "Standard_NC24r")]
    StandardNc24r,
    #[serde(rename = "Standard_NC24rs_v2")]
    StandardNc24rsV2,
    #[serde(rename = "Standard_NC24rs_v3")]
    StandardNc24rsV3,
    #[serde(rename = "Standard_NC24s_v2")]
    StandardNc24sV2,
    #[serde(rename = "Standard_NC24s_v3")]
    StandardNc24sV3,
    #[serde(rename = "Standard_NC6")]
    StandardNc6,
    #[serde(rename = "Standard_NC6s_v2")]
    StandardNc6sV2,
    #[serde(rename = "Standard_NC6s_v3")]
    StandardNc6sV3,
    #[serde(rename = "Standard_ND12s")]
    StandardNd12s,
    #[serde(rename = "Standard_ND24rs")]
    StandardNd24rs,
    #[serde(rename = "Standard_ND24s")]
    StandardNd24s,
    #[serde(rename = "Standard_ND6s")]
    StandardNd6s,
    #[serde(rename = "Standard_NV12")]
    StandardNv12,
    #[serde(rename = "Standard_NV24")]
    StandardNv24,
    #[serde(rename = "Standard_NV6")]
    StandardNv6,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ContainerServiceVmSize {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ContainerServiceVmSize {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ContainerServiceVmSize {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::StandardA1 => serializer.serialize_unit_variant("ContainerServiceVmSize", 0u32, "Standard_A1"),
            Self::StandardA10 => serializer.serialize_unit_variant("ContainerServiceVmSize", 1u32, "Standard_A10"),
            Self::StandardA11 => serializer.serialize_unit_variant("ContainerServiceVmSize", 2u32, "Standard_A11"),
            Self::StandardA1V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 3u32, "Standard_A1_v2"),
            Self::StandardA2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 4u32, "Standard_A2"),
            Self::StandardA2V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 5u32, "Standard_A2_v2"),
            Self::StandardA2mV2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 6u32, "Standard_A2m_v2"),
            Self::StandardA3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 7u32, "Standard_A3"),
            Self::StandardA4 => serializer.serialize_unit_variant("ContainerServiceVmSize", 8u32, "Standard_A4"),
            Self::StandardA4V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 9u32, "Standard_A4_v2"),
            Self::StandardA4mV2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 10u32, "Standard_A4m_v2"),
            Self::StandardA5 => serializer.serialize_unit_variant("ContainerServiceVmSize", 11u32, "Standard_A5"),
            Self::StandardA6 => serializer.serialize_unit_variant("ContainerServiceVmSize", 12u32, "Standard_A6"),
            Self::StandardA7 => serializer.serialize_unit_variant("ContainerServiceVmSize", 13u32, "Standard_A7"),
            Self::StandardA8 => serializer.serialize_unit_variant("ContainerServiceVmSize", 14u32, "Standard_A8"),
            Self::StandardA8V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 15u32, "Standard_A8_v2"),
            Self::StandardA8mV2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 16u32, "Standard_A8m_v2"),
            Self::StandardA9 => serializer.serialize_unit_variant("ContainerServiceVmSize", 17u32, "Standard_A9"),
            Self::StandardB2ms => serializer.serialize_unit_variant("ContainerServiceVmSize", 18u32, "Standard_B2ms"),
            Self::StandardB2s => serializer.serialize_unit_variant("ContainerServiceVmSize", 19u32, "Standard_B2s"),
            Self::StandardB4ms => serializer.serialize_unit_variant("ContainerServiceVmSize", 20u32, "Standard_B4ms"),
            Self::StandardB8ms => serializer.serialize_unit_variant("ContainerServiceVmSize", 21u32, "Standard_B8ms"),
            Self::StandardD1 => serializer.serialize_unit_variant("ContainerServiceVmSize", 22u32, "Standard_D1"),
            Self::StandardD11 => serializer.serialize_unit_variant("ContainerServiceVmSize", 23u32, "Standard_D11"),
            Self::StandardD11V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 24u32, "Standard_D11_v2"),
            Self::StandardD11V2Promo => serializer.serialize_unit_variant("ContainerServiceVmSize", 25u32, "Standard_D11_v2_Promo"),
            Self::StandardD12 => serializer.serialize_unit_variant("ContainerServiceVmSize", 26u32, "Standard_D12"),
            Self::StandardD12V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 27u32, "Standard_D12_v2"),
            Self::StandardD12V2Promo => serializer.serialize_unit_variant("ContainerServiceVmSize", 28u32, "Standard_D12_v2_Promo"),
            Self::StandardD13 => serializer.serialize_unit_variant("ContainerServiceVmSize", 29u32, "Standard_D13"),
            Self::StandardD13V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 30u32, "Standard_D13_v2"),
            Self::StandardD13V2Promo => serializer.serialize_unit_variant("ContainerServiceVmSize", 31u32, "Standard_D13_v2_Promo"),
            Self::StandardD14 => serializer.serialize_unit_variant("ContainerServiceVmSize", 32u32, "Standard_D14"),
            Self::StandardD14V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 33u32, "Standard_D14_v2"),
            Self::StandardD14V2Promo => serializer.serialize_unit_variant("ContainerServiceVmSize", 34u32, "Standard_D14_v2_Promo"),
            Self::StandardD15V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 35u32, "Standard_D15_v2"),
            Self::StandardD16V3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 36u32, "Standard_D16_v3"),
            Self::StandardD16sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 37u32, "Standard_D16s_v3"),
            Self::StandardD1V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 38u32, "Standard_D1_v2"),
            Self::StandardD2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 39u32, "Standard_D2"),
            Self::StandardD2V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 40u32, "Standard_D2_v2"),
            Self::StandardD2V2Promo => serializer.serialize_unit_variant("ContainerServiceVmSize", 41u32, "Standard_D2_v2_Promo"),
            Self::StandardD2V3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 42u32, "Standard_D2_v3"),
            Self::StandardD2sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 43u32, "Standard_D2s_v3"),
            Self::StandardD3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 44u32, "Standard_D3"),
            Self::StandardD32V3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 45u32, "Standard_D32_v3"),
            Self::StandardD32sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 46u32, "Standard_D32s_v3"),
            Self::StandardD3V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 47u32, "Standard_D3_v2"),
            Self::StandardD3V2Promo => serializer.serialize_unit_variant("ContainerServiceVmSize", 48u32, "Standard_D3_v2_Promo"),
            Self::StandardD4 => serializer.serialize_unit_variant("ContainerServiceVmSize", 49u32, "Standard_D4"),
            Self::StandardD4V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 50u32, "Standard_D4_v2"),
            Self::StandardD4V2Promo => serializer.serialize_unit_variant("ContainerServiceVmSize", 51u32, "Standard_D4_v2_Promo"),
            Self::StandardD4V3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 52u32, "Standard_D4_v3"),
            Self::StandardD4sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 53u32, "Standard_D4s_v3"),
            Self::StandardD5V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 54u32, "Standard_D5_v2"),
            Self::StandardD5V2Promo => serializer.serialize_unit_variant("ContainerServiceVmSize", 55u32, "Standard_D5_v2_Promo"),
            Self::StandardD64V3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 56u32, "Standard_D64_v3"),
            Self::StandardD64sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 57u32, "Standard_D64s_v3"),
            Self::StandardD8V3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 58u32, "Standard_D8_v3"),
            Self::StandardD8sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 59u32, "Standard_D8s_v3"),
            Self::StandardDs1 => serializer.serialize_unit_variant("ContainerServiceVmSize", 60u32, "Standard_DS1"),
            Self::StandardDs11 => serializer.serialize_unit_variant("ContainerServiceVmSize", 61u32, "Standard_DS11"),
            Self::StandardDs11V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 62u32, "Standard_DS11_v2"),
            Self::StandardDs11V2Promo => serializer.serialize_unit_variant("ContainerServiceVmSize", 63u32, "Standard_DS11_v2_Promo"),
            Self::StandardDs12 => serializer.serialize_unit_variant("ContainerServiceVmSize", 64u32, "Standard_DS12"),
            Self::StandardDs12V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 65u32, "Standard_DS12_v2"),
            Self::StandardDs12V2Promo => serializer.serialize_unit_variant("ContainerServiceVmSize", 66u32, "Standard_DS12_v2_Promo"),
            Self::StandardDs13 => serializer.serialize_unit_variant("ContainerServiceVmSize", 67u32, "Standard_DS13"),
            Self::StandardDs132V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 68u32, "Standard_DS13-2_v2"),
            Self::StandardDs134V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 69u32, "Standard_DS13-4_v2"),
            Self::StandardDs13V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 70u32, "Standard_DS13_v2"),
            Self::StandardDs13V2Promo => serializer.serialize_unit_variant("ContainerServiceVmSize", 71u32, "Standard_DS13_v2_Promo"),
            Self::StandardDs14 => serializer.serialize_unit_variant("ContainerServiceVmSize", 72u32, "Standard_DS14"),
            Self::StandardDs144V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 73u32, "Standard_DS14-4_v2"),
            Self::StandardDs148V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 74u32, "Standard_DS14-8_v2"),
            Self::StandardDs14V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 75u32, "Standard_DS14_v2"),
            Self::StandardDs14V2Promo => serializer.serialize_unit_variant("ContainerServiceVmSize", 76u32, "Standard_DS14_v2_Promo"),
            Self::StandardDs15V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 77u32, "Standard_DS15_v2"),
            Self::StandardDs1V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 78u32, "Standard_DS1_v2"),
            Self::StandardDs2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 79u32, "Standard_DS2"),
            Self::StandardDs2V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 80u32, "Standard_DS2_v2"),
            Self::StandardDs2V2Promo => serializer.serialize_unit_variant("ContainerServiceVmSize", 81u32, "Standard_DS2_v2_Promo"),
            Self::StandardDs3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 82u32, "Standard_DS3"),
            Self::StandardDs3V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 83u32, "Standard_DS3_v2"),
            Self::StandardDs3V2Promo => serializer.serialize_unit_variant("ContainerServiceVmSize", 84u32, "Standard_DS3_v2_Promo"),
            Self::StandardDs4 => serializer.serialize_unit_variant("ContainerServiceVmSize", 85u32, "Standard_DS4"),
            Self::StandardDs4V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 86u32, "Standard_DS4_v2"),
            Self::StandardDs4V2Promo => serializer.serialize_unit_variant("ContainerServiceVmSize", 87u32, "Standard_DS4_v2_Promo"),
            Self::StandardDs5V2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 88u32, "Standard_DS5_v2"),
            Self::StandardDs5V2Promo => serializer.serialize_unit_variant("ContainerServiceVmSize", 89u32, "Standard_DS5_v2_Promo"),
            Self::StandardE16V3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 90u32, "Standard_E16_v3"),
            Self::StandardE16sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 91u32, "Standard_E16s_v3"),
            Self::StandardE2V3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 92u32, "Standard_E2_v3"),
            Self::StandardE2sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 93u32, "Standard_E2s_v3"),
            Self::StandardE3216sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 94u32, "Standard_E32-16s_v3"),
            Self::StandardE328sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 95u32, "Standard_E32-8s_v3"),
            Self::StandardE32V3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 96u32, "Standard_E32_v3"),
            Self::StandardE32sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 97u32, "Standard_E32s_v3"),
            Self::StandardE4V3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 98u32, "Standard_E4_v3"),
            Self::StandardE4sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 99u32, "Standard_E4s_v3"),
            Self::StandardE6416sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 100u32, "Standard_E64-16s_v3"),
            Self::StandardE6432sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 101u32, "Standard_E64-32s_v3"),
            Self::StandardE64V3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 102u32, "Standard_E64_v3"),
            Self::StandardE64sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 103u32, "Standard_E64s_v3"),
            Self::StandardE8V3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 104u32, "Standard_E8_v3"),
            Self::StandardE8sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 105u32, "Standard_E8s_v3"),
            Self::StandardF1 => serializer.serialize_unit_variant("ContainerServiceVmSize", 106u32, "Standard_F1"),
            Self::StandardF16 => serializer.serialize_unit_variant("ContainerServiceVmSize", 107u32, "Standard_F16"),
            Self::StandardF16s => serializer.serialize_unit_variant("ContainerServiceVmSize", 108u32, "Standard_F16s"),
            Self::StandardF16sV2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 109u32, "Standard_F16s_v2"),
            Self::StandardF1s => serializer.serialize_unit_variant("ContainerServiceVmSize", 110u32, "Standard_F1s"),
            Self::StandardF2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 111u32, "Standard_F2"),
            Self::StandardF2s => serializer.serialize_unit_variant("ContainerServiceVmSize", 112u32, "Standard_F2s"),
            Self::StandardF2sV2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 113u32, "Standard_F2s_v2"),
            Self::StandardF32sV2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 114u32, "Standard_F32s_v2"),
            Self::StandardF4 => serializer.serialize_unit_variant("ContainerServiceVmSize", 115u32, "Standard_F4"),
            Self::StandardF4s => serializer.serialize_unit_variant("ContainerServiceVmSize", 116u32, "Standard_F4s"),
            Self::StandardF4sV2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 117u32, "Standard_F4s_v2"),
            Self::StandardF64sV2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 118u32, "Standard_F64s_v2"),
            Self::StandardF72sV2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 119u32, "Standard_F72s_v2"),
            Self::StandardF8 => serializer.serialize_unit_variant("ContainerServiceVmSize", 120u32, "Standard_F8"),
            Self::StandardF8s => serializer.serialize_unit_variant("ContainerServiceVmSize", 121u32, "Standard_F8s"),
            Self::StandardF8sV2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 122u32, "Standard_F8s_v2"),
            Self::StandardG1 => serializer.serialize_unit_variant("ContainerServiceVmSize", 123u32, "Standard_G1"),
            Self::StandardG2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 124u32, "Standard_G2"),
            Self::StandardG3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 125u32, "Standard_G3"),
            Self::StandardG4 => serializer.serialize_unit_variant("ContainerServiceVmSize", 126u32, "Standard_G4"),
            Self::StandardG5 => serializer.serialize_unit_variant("ContainerServiceVmSize", 127u32, "Standard_G5"),
            Self::StandardGs1 => serializer.serialize_unit_variant("ContainerServiceVmSize", 128u32, "Standard_GS1"),
            Self::StandardGs2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 129u32, "Standard_GS2"),
            Self::StandardGs3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 130u32, "Standard_GS3"),
            Self::StandardGs4 => serializer.serialize_unit_variant("ContainerServiceVmSize", 131u32, "Standard_GS4"),
            Self::StandardGs44 => serializer.serialize_unit_variant("ContainerServiceVmSize", 132u32, "Standard_GS4-4"),
            Self::StandardGs48 => serializer.serialize_unit_variant("ContainerServiceVmSize", 133u32, "Standard_GS4-8"),
            Self::StandardGs5 => serializer.serialize_unit_variant("ContainerServiceVmSize", 134u32, "Standard_GS5"),
            Self::StandardGs516 => serializer.serialize_unit_variant("ContainerServiceVmSize", 135u32, "Standard_GS5-16"),
            Self::StandardGs58 => serializer.serialize_unit_variant("ContainerServiceVmSize", 136u32, "Standard_GS5-8"),
            Self::StandardH16 => serializer.serialize_unit_variant("ContainerServiceVmSize", 137u32, "Standard_H16"),
            Self::StandardH16m => serializer.serialize_unit_variant("ContainerServiceVmSize", 138u32, "Standard_H16m"),
            Self::StandardH16mr => serializer.serialize_unit_variant("ContainerServiceVmSize", 139u32, "Standard_H16mr"),
            Self::StandardH16r => serializer.serialize_unit_variant("ContainerServiceVmSize", 140u32, "Standard_H16r"),
            Self::StandardH8 => serializer.serialize_unit_variant("ContainerServiceVmSize", 141u32, "Standard_H8"),
            Self::StandardH8m => serializer.serialize_unit_variant("ContainerServiceVmSize", 142u32, "Standard_H8m"),
            Self::StandardL16s => serializer.serialize_unit_variant("ContainerServiceVmSize", 143u32, "Standard_L16s"),
            Self::StandardL32s => serializer.serialize_unit_variant("ContainerServiceVmSize", 144u32, "Standard_L32s"),
            Self::StandardL4s => serializer.serialize_unit_variant("ContainerServiceVmSize", 145u32, "Standard_L4s"),
            Self::StandardL8s => serializer.serialize_unit_variant("ContainerServiceVmSize", 146u32, "Standard_L8s"),
            Self::StandardM12832ms => serializer.serialize_unit_variant("ContainerServiceVmSize", 147u32, "Standard_M128-32ms"),
            Self::StandardM12864ms => serializer.serialize_unit_variant("ContainerServiceVmSize", 148u32, "Standard_M128-64ms"),
            Self::StandardM128ms => serializer.serialize_unit_variant("ContainerServiceVmSize", 149u32, "Standard_M128ms"),
            Self::StandardM128s => serializer.serialize_unit_variant("ContainerServiceVmSize", 150u32, "Standard_M128s"),
            Self::StandardM6416ms => serializer.serialize_unit_variant("ContainerServiceVmSize", 151u32, "Standard_M64-16ms"),
            Self::StandardM6432ms => serializer.serialize_unit_variant("ContainerServiceVmSize", 152u32, "Standard_M64-32ms"),
            Self::StandardM64ms => serializer.serialize_unit_variant("ContainerServiceVmSize", 153u32, "Standard_M64ms"),
            Self::StandardM64s => serializer.serialize_unit_variant("ContainerServiceVmSize", 154u32, "Standard_M64s"),
            Self::StandardNc12 => serializer.serialize_unit_variant("ContainerServiceVmSize", 155u32, "Standard_NC12"),
            Self::StandardNc12sV2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 156u32, "Standard_NC12s_v2"),
            Self::StandardNc12sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 157u32, "Standard_NC12s_v3"),
            Self::StandardNc24 => serializer.serialize_unit_variant("ContainerServiceVmSize", 158u32, "Standard_NC24"),
            Self::StandardNc24r => serializer.serialize_unit_variant("ContainerServiceVmSize", 159u32, "Standard_NC24r"),
            Self::StandardNc24rsV2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 160u32, "Standard_NC24rs_v2"),
            Self::StandardNc24rsV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 161u32, "Standard_NC24rs_v3"),
            Self::StandardNc24sV2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 162u32, "Standard_NC24s_v2"),
            Self::StandardNc24sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 163u32, "Standard_NC24s_v3"),
            Self::StandardNc6 => serializer.serialize_unit_variant("ContainerServiceVmSize", 164u32, "Standard_NC6"),
            Self::StandardNc6sV2 => serializer.serialize_unit_variant("ContainerServiceVmSize", 165u32, "Standard_NC6s_v2"),
            Self::StandardNc6sV3 => serializer.serialize_unit_variant("ContainerServiceVmSize", 166u32, "Standard_NC6s_v3"),
            Self::StandardNd12s => serializer.serialize_unit_variant("ContainerServiceVmSize", 167u32, "Standard_ND12s"),
            Self::StandardNd24rs => serializer.serialize_unit_variant("ContainerServiceVmSize", 168u32, "Standard_ND24rs"),
            Self::StandardNd24s => serializer.serialize_unit_variant("ContainerServiceVmSize", 169u32, "Standard_ND24s"),
            Self::StandardNd6s => serializer.serialize_unit_variant("ContainerServiceVmSize", 170u32, "Standard_ND6s"),
            Self::StandardNv12 => serializer.serialize_unit_variant("ContainerServiceVmSize", 171u32, "Standard_NV12"),
            Self::StandardNv24 => serializer.serialize_unit_variant("ContainerServiceVmSize", 172u32, "Standard_NV24"),
            Self::StandardNv6 => serializer.serialize_unit_variant("ContainerServiceVmSize", 173u32, "Standard_NV6"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type ContainerServiceVnetSubnetId = String;
#[doc = "Data used when creating a target resource from a source resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreationData {
    #[doc = "This is the ARM ID of the source object to be used to create the target object."]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
}
impl CreationData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The credential result response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CredentialResult {
    #[doc = "The name of the credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Base64-encoded Kubernetes configuration file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl CredentialResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list credential result response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CredentialResults {
    #[doc = "Base64-encoded Kubernetes configuration file."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub kubeconfigs: Vec<CredentialResult>,
}
impl CredentialResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A domain name that AKS agent nodes are reaching at."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDependency {
    #[doc = "The domain name of the dependency."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "The Ports and Protocols used when connecting to domainName."]
    #[serde(rename = "endpointDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint_details: Vec<EndpointDetail>,
}
impl EndpointDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "connect information from the AKS agent nodes to a single endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDetail {
    #[doc = "An IP Address that Domain Name currently resolves to."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The port an endpoint is connected to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "The protocol used for connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[doc = "Description of the detail"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl EndpointDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The complex type of the extended location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedLocation {
    #[doc = "The name of the extended location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of extendedLocation."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ExtendedLocationType>,
}
impl ExtendedLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of extendedLocation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ExtendedLocationType")]
pub enum ExtendedLocationType {
    EdgeZone,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ExtendedLocationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ExtendedLocationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ExtendedLocationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EdgeZone => serializer.serialize_unit_variant("ExtendedLocationType", 0u32, "EdgeZone"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "GPUInstanceProfile to be used to specify GPU MIG instance profile for supported GPU VM SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "GpuInstanceProfile")]
pub enum GpuInstanceProfile {
    #[serde(rename = "MIG1g")]
    Mig1g,
    #[serde(rename = "MIG2g")]
    Mig2g,
    #[serde(rename = "MIG3g")]
    Mig3g,
    #[serde(rename = "MIG4g")]
    Mig4g,
    #[serde(rename = "MIG7g")]
    Mig7g,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for GpuInstanceProfile {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for GpuInstanceProfile {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for GpuInstanceProfile {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Mig1g => serializer.serialize_unit_variant("GpuInstanceProfile", 0u32, "MIG1g"),
            Self::Mig2g => serializer.serialize_unit_variant("GpuInstanceProfile", 1u32, "MIG2g"),
            Self::Mig3g => serializer.serialize_unit_variant("GpuInstanceProfile", 2u32, "MIG3g"),
            Self::Mig4g => serializer.serialize_unit_variant("GpuInstanceProfile", 3u32, "MIG4g"),
            Self::Mig7g => serializer.serialize_unit_variant("GpuInstanceProfile", 4u32, "MIG7g"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type HourInDay = i32;
#[doc = "See [AKS custom node configuration](https://docs.microsoft.com/azure/aks/custom-node-configuration) for more details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubeletConfig {
    #[doc = "The default is 'none'. See [Kubernetes CPU management policies](https://kubernetes.io/docs/tasks/administer-cluster/cpu-management-policies/#cpu-management-policies) for more information. Allowed values are 'none' and 'static'."]
    #[serde(rename = "cpuManagerPolicy", default, skip_serializing_if = "Option::is_none")]
    pub cpu_manager_policy: Option<String>,
    #[doc = "The default is true."]
    #[serde(rename = "cpuCfsQuota", default, skip_serializing_if = "Option::is_none")]
    pub cpu_cfs_quota: Option<bool>,
    #[doc = "The default is '100ms.' Valid values are a sequence of decimal numbers with an optional fraction and a unit suffix. For example: '300ms', '2h45m'. Supported units are 'ns', 'us', 'ms', 's', 'm', and 'h'."]
    #[serde(rename = "cpuCfsQuotaPeriod", default, skip_serializing_if = "Option::is_none")]
    pub cpu_cfs_quota_period: Option<String>,
    #[doc = "To disable image garbage collection, set to 100. The default is 85%"]
    #[serde(rename = "imageGcHighThreshold", default, skip_serializing_if = "Option::is_none")]
    pub image_gc_high_threshold: Option<i32>,
    #[doc = "This cannot be set higher than imageGcHighThreshold. The default is 80%"]
    #[serde(rename = "imageGcLowThreshold", default, skip_serializing_if = "Option::is_none")]
    pub image_gc_low_threshold: Option<i32>,
    #[doc = "For more information see [Kubernetes Topology Manager](https://kubernetes.io/docs/tasks/administer-cluster/topology-manager). The default is 'none'. Allowed values are 'none', 'best-effort', 'restricted', and 'single-numa-node'."]
    #[serde(rename = "topologyManagerPolicy", default, skip_serializing_if = "Option::is_none")]
    pub topology_manager_policy: Option<String>,
    #[doc = "Allowed list of unsafe sysctls or unsafe sysctl patterns (ending in `*`)."]
    #[serde(rename = "allowedUnsafeSysctls", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_unsafe_sysctls: Vec<String>,
    #[doc = "If set to true it will make the Kubelet fail to start if swap is enabled on the node."]
    #[serde(rename = "failSwapOn", default, skip_serializing_if = "Option::is_none")]
    pub fail_swap_on: Option<bool>,
    #[doc = "The maximum size (e.g. 10Mi) of container log file before it is rotated."]
    #[serde(rename = "containerLogMaxSizeMB", default, skip_serializing_if = "Option::is_none")]
    pub container_log_max_size_mb: Option<i32>,
    #[doc = "The maximum number of container log files that can be present for a container. The number must be ≥ 2."]
    #[serde(rename = "containerLogMaxFiles", default, skip_serializing_if = "Option::is_none")]
    pub container_log_max_files: Option<i32>,
    #[doc = "The maximum number of processes per pod."]
    #[serde(rename = "podMaxPids", default, skip_serializing_if = "Option::is_none")]
    pub pod_max_pids: Option<i32>,
}
impl KubeletConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Determines the placement of emptyDir volumes, container runtime data root, and Kubelet ephemeral storage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "KubeletDiskType")]
pub enum KubeletDiskType {
    #[serde(rename = "OS")]
    Os,
    Temporary,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for KubeletDiskType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for KubeletDiskType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for KubeletDiskType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Os => serializer.serialize_unit_variant("KubeletDiskType", 0u32, "OS"),
            Self::Temporary => serializer.serialize_unit_variant("KubeletDiskType", 1u32, "Temporary"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "See [AKS custom node configuration](https://docs.microsoft.com/azure/aks/custom-node-configuration) for more details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxOsConfig {
    #[doc = "Sysctl settings for Linux agent nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<SysctlConfig>,
    #[doc = "Valid values are 'always', 'madvise', and 'never'. The default is 'always'. For more information see [Transparent Hugepages](https://www.kernel.org/doc/html/latest/admin-guide/mm/transhuge.html#admin-guide-transhuge)."]
    #[serde(rename = "transparentHugePageEnabled", default, skip_serializing_if = "Option::is_none")]
    pub transparent_huge_page_enabled: Option<String>,
    #[doc = "Valid values are 'always', 'defer', 'defer+madvise', 'madvise' and 'never'. The default is 'madvise'. For more information see [Transparent Hugepages](https://www.kernel.org/doc/html/latest/admin-guide/mm/transhuge.html#admin-guide-transhuge)."]
    #[serde(rename = "transparentHugePageDefrag", default, skip_serializing_if = "Option::is_none")]
    pub transparent_huge_page_defrag: Option<String>,
    #[doc = "The size in MB of a swap file that will be created on each node."]
    #[serde(rename = "swapFileSizeMB", default, skip_serializing_if = "Option::is_none")]
    pub swap_file_size_mb: Option<i32>,
}
impl LinuxOsConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The default is 'standard'. See [Azure Load Balancer SKUs](https://docs.microsoft.com/azure/load-balancer/skus) for more information about the differences between load balancer SKUs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LoadBalancerSku")]
pub enum LoadBalancerSku {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "basic")]
    Basic,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LoadBalancerSku {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LoadBalancerSku {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LoadBalancerSku {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Standard => serializer.serialize_unit_variant("LoadBalancerSku", 0u32, "standard"),
            Self::Basic => serializer.serialize_unit_variant("LoadBalancerSku", 1u32, "basic"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "See [planned maintenance](https://docs.microsoft.com/azure/aks/planned-maintenance) for more information about planned maintenance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceConfiguration {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties used to configure planned maintenance for a Managed Cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MaintenanceConfigurationProperties>,
}
impl MaintenanceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from the List maintenance configurations operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceConfigurationListResult {
    #[doc = "The list of maintenance configurations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MaintenanceConfiguration>,
    #[doc = "The URL to get the next set of maintenance configuration results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MaintenanceConfigurationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MaintenanceConfigurationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties used to configure planned maintenance for a Managed Cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceConfigurationProperties {
    #[doc = "If two array entries specify the same day of the week, the applied configuration is the union of times in both entries."]
    #[serde(rename = "timeInWeek", default, skip_serializing_if = "Vec::is_empty")]
    pub time_in_week: Vec<TimeInWeek>,
    #[doc = "Time slots on which upgrade is not allowed."]
    #[serde(rename = "notAllowedTime", default, skip_serializing_if = "Vec::is_empty")]
    pub not_allowed_time: Vec<TimeSpan>,
}
impl MaintenanceConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedCluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The SKU of a Managed Cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ManagedClusterSku>,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "Identity for the managed cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedClusterIdentity>,
    #[doc = "Properties of the managed cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedClusterProperties>,
}
impl ManagedCluster {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            sku: None,
            extended_location: None,
            identity: None,
            properties: None,
        }
    }
}
#[doc = "For more details see [managed AAD on AKS](https://docs.microsoft.com/azure/aks/managed-aad)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterAadProfile {
    #[doc = "Whether to enable managed AAD."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    #[doc = "Whether to enable Azure RBAC for Kubernetes authorization."]
    #[serde(rename = "enableAzureRBAC", default, skip_serializing_if = "Option::is_none")]
    pub enable_azure_rbac: Option<bool>,
    #[doc = "The list of AAD group object IDs that will have admin role of the cluster."]
    #[serde(rename = "adminGroupObjectIDs", default, skip_serializing_if = "Vec::is_empty")]
    pub admin_group_object_i_ds: Vec<String>,
    #[doc = "The client AAD application ID."]
    #[serde(rename = "clientAppID", default, skip_serializing_if = "Option::is_none")]
    pub client_app_id: Option<String>,
    #[doc = "The server AAD application ID."]
    #[serde(rename = "serverAppID", default, skip_serializing_if = "Option::is_none")]
    pub server_app_id: Option<String>,
    #[doc = "The server AAD application secret."]
    #[serde(rename = "serverAppSecret", default, skip_serializing_if = "Option::is_none")]
    pub server_app_secret: Option<String>,
    #[doc = "The AAD tenant ID to use for authentication. If not specified, will use the tenant of the deployment subscription."]
    #[serde(rename = "tenantID", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ManagedClusterAadProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Access profile for managed cluster API server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterApiServerAccessProfile {
    #[doc = "IP ranges are specified in CIDR format, e.g. 137.117.106.88/29. This feature is not compatible with clusters that use Public IP Per Node, or clusters that are using a Basic Load Balancer. For more information see [API server authorized IP ranges](https://docs.microsoft.com/azure/aks/api-server-authorized-ip-ranges)."]
    #[serde(rename = "authorizedIPRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub authorized_ip_ranges: Vec<String>,
    #[doc = "For more details, see [Creating a private AKS cluster](https://docs.microsoft.com/azure/aks/private-clusters)."]
    #[serde(rename = "enablePrivateCluster", default, skip_serializing_if = "Option::is_none")]
    pub enable_private_cluster: Option<bool>,
    #[doc = "The default is System. For more details see [configure private DNS zone](https://docs.microsoft.com/azure/aks/private-clusters#configure-private-dns-zone). Allowed values are 'system' and 'none'."]
    #[serde(rename = "privateDNSZone", default, skip_serializing_if = "Option::is_none")]
    pub private_dns_zone: Option<String>,
    #[doc = "Whether to create additional public FQDN for private cluster or not."]
    #[serde(rename = "enablePrivateClusterPublicFQDN", default, skip_serializing_if = "Option::is_none")]
    pub enable_private_cluster_public_fqdn: Option<bool>,
    #[doc = "Whether to disable run command for the cluster or not."]
    #[serde(rename = "disableRunCommand", default, skip_serializing_if = "Option::is_none")]
    pub disable_run_command: Option<bool>,
    #[doc = "Whether to enable apiserver vnet integration for the cluster or not."]
    #[serde(rename = "enableVnetIntegration", default, skip_serializing_if = "Option::is_none")]
    pub enable_vnet_integration: Option<bool>,
    #[doc = "It is required when: 1. creating a new cluster with BYO Vnet; 2. updating an existing cluster to enable apiserver vnet integration."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}
impl ManagedClusterApiServerAccessProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed cluster Access Profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterAccessProfile {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Profile for enabling a user to access a managed cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessProfile>,
}
impl ManagedClusterAccessProfile {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "A Kubernetes add-on profile for a managed cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterAddonProfile {
    #[doc = "Whether the add-on is enabled or not."]
    pub enabled: bool,
    #[doc = "Key-value pairs for configuring an add-on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
    #[doc = "Information of user assigned identity used by this add-on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<serde_json::Value>,
}
impl ManagedClusterAddonProfile {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            config: None,
            identity: None,
        }
    }
}
#[doc = "Profile for the container service agent pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterAgentPoolProfile {
    #[serde(flatten)]
    pub managed_cluster_agent_pool_profile_properties: ManagedClusterAgentPoolProfileProperties,
    #[doc = "Windows agent pool names must be 6 characters or less."]
    pub name: String,
}
impl ManagedClusterAgentPoolProfile {
    pub fn new(name: String) -> Self {
        Self {
            managed_cluster_agent_pool_profile_properties: ManagedClusterAgentPoolProfileProperties::default(),
            name,
        }
    }
}
#[doc = "Properties for the container service agent pool profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterAgentPoolProfileProperties {
    #[doc = "Number of agents (VMs) to host docker containers. Allowed values must be in the range of 0 to 1000 (inclusive) for user pools and in the range of 1 to 1000 (inclusive) for system pools. The default value is 1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "VM size availability varies by region. If a node contains insufficient compute resources (memory, cpu, etc) pods might fail to run correctly. For more details on restricted VM sizes, see: https://docs.microsoft.com/azure/aks/quotas-skus-regions"]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[doc = "OS Disk Size in GB to be used to specify the disk size for every machine in the master/agent pool. If you specify 0, it will apply the default osDisk size according to the vmSize specified."]
    #[serde(rename = "osDiskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_size_gb: Option<ContainerServiceOsDisk>,
    #[doc = "The default is 'Ephemeral' if the VM supports it and has a cache disk larger than the requested OSDiskSizeGB. Otherwise, defaults to 'Managed'. May not be changed after creation. For more information see [Ephemeral OS](https://docs.microsoft.com/azure/aks/cluster-configuration#ephemeral-os)."]
    #[serde(rename = "osDiskType", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_type: Option<OsDiskType>,
    #[doc = "Determines the placement of emptyDir volumes, container runtime data root, and Kubelet ephemeral storage."]
    #[serde(rename = "kubeletDiskType", default, skip_serializing_if = "Option::is_none")]
    pub kubelet_disk_type: Option<KubeletDiskType>,
    #[doc = "Determines the type of workload a node can run."]
    #[serde(rename = "workloadRuntime", default, skip_serializing_if = "Option::is_none")]
    pub workload_runtime: Option<WorkloadRuntime>,
    #[doc = "A base64-encoded string which will be written to /etc/motd after decoding. This allows customization of the message of the day for Linux nodes. It must not be specified for Windows nodes. It must be a static string (i.e., will be printed raw and not be executed as a script)."]
    #[serde(rename = "messageOfTheDay", default, skip_serializing_if = "Option::is_none")]
    pub message_of_the_day: Option<String>,
    #[doc = "If this is not specified, a VNET and subnet will be generated and used. If no podSubnetID is specified, this applies to nodes and pods, otherwise it applies to just nodes. This is of the form: /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{virtualNetworkName}/subnets/{subnetName}"]
    #[serde(rename = "vnetSubnetID", default, skip_serializing_if = "Option::is_none")]
    pub vnet_subnet_id: Option<String>,
    #[doc = "If omitted, pod IPs are statically assigned on the node subnet (see vnetSubnetID for more details). This is of the form: /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{virtualNetworkName}/subnets/{subnetName}"]
    #[serde(rename = "podSubnetID", default, skip_serializing_if = "Option::is_none")]
    pub pod_subnet_id: Option<String>,
    #[doc = "The maximum number of pods that can run on a node."]
    #[serde(rename = "maxPods", default, skip_serializing_if = "Option::is_none")]
    pub max_pods: Option<i32>,
    #[doc = "The operating system type. The default is Linux."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "Specifies the OS SKU used by the agent pool. If not specified, the default is Ubuntu if OSType=Linux or Windows2019 if OSType=Windows. And the default Windows OSSKU will be changed to Windows2022 after Windows2019 is deprecated."]
    #[serde(rename = "osSKU", default, skip_serializing_if = "Option::is_none")]
    pub os_sku: Option<Ossku>,
    #[doc = "The maximum number of nodes for auto-scaling"]
    #[serde(rename = "maxCount", default, skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i32>,
    #[doc = "The minimum number of nodes for auto-scaling"]
    #[serde(rename = "minCount", default, skip_serializing_if = "Option::is_none")]
    pub min_count: Option<i32>,
    #[doc = "Whether to enable auto-scaler"]
    #[serde(rename = "enableAutoScaling", default, skip_serializing_if = "Option::is_none")]
    pub enable_auto_scaling: Option<bool>,
    #[doc = "Describes how VMs are added to or removed from Agent Pools. See [billing states](https://docs.microsoft.com/azure/virtual-machines/states-billing)."]
    #[serde(rename = "scaleDownMode", default, skip_serializing_if = "Option::is_none")]
    pub scale_down_mode: Option<ScaleDownMode>,
    #[doc = "The type of Agent Pool."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<AgentPoolType>,
    #[doc = "A cluster must have at least one 'System' Agent Pool at all times. For additional information on agent pool restrictions and best practices, see: https://docs.microsoft.com/azure/aks/use-system-pools"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<AgentPoolMode>,
    #[doc = "Both patch version <major.minor.patch> and <major.minor> are supported. When <major.minor> is specified, the latest supported patch version is chosen automatically. Updating the agent pool with the same <major.minor> once it has been created will not trigger an upgrade, even if a newer patch version is available. As a best practice, you should upgrade all node pools in an AKS cluster to the same Kubernetes version. The node pool version must have the same major version as the control plane. The node pool minor version must be within two minor versions of the control plane version. The node pool version cannot be greater than the control plane version. For more information see [upgrading a node pool](https://docs.microsoft.com/azure/aks/use-multiple-node-pools#upgrade-a-node-pool)."]
    #[serde(rename = "orchestratorVersion", default, skip_serializing_if = "Option::is_none")]
    pub orchestrator_version: Option<String>,
    #[doc = "If orchestratorVersion was a fully specified version <major.minor.patch>, this field will be exactly equal to it. If orchestratorVersion was <major.minor>, this field will contain the full <major.minor.patch> version being used."]
    #[serde(rename = "currentOrchestratorVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_orchestrator_version: Option<String>,
    #[doc = "The version of node image"]
    #[serde(rename = "nodeImageVersion", default, skip_serializing_if = "Option::is_none")]
    pub node_image_version: Option<String>,
    #[doc = "Settings for upgrading an agentpool"]
    #[serde(rename = "upgradeSettings", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_settings: Option<AgentPoolUpgradeSettings>,
    #[doc = "The current deployment or provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Describes the Power State of the cluster"]
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<PowerState>,
    #[doc = "The list of Availability zones to use for nodes. This can only be specified if the AgentPoolType property is 'VirtualMachineScaleSets'."]
    #[serde(rename = "availabilityZones", default, skip_serializing_if = "Vec::is_empty")]
    pub availability_zones: Vec<String>,
    #[doc = "Some scenarios may require nodes in a node pool to receive their own dedicated public IP addresses. A common scenario is for gaming workloads, where a console needs to make a direct connection to a cloud virtual machine to minimize hops. For more information see [assigning a public IP per node](https://docs.microsoft.com/azure/aks/use-multiple-node-pools#assign-a-public-ip-per-node-for-your-node-pools). The default is false."]
    #[serde(rename = "enableNodePublicIP", default, skip_serializing_if = "Option::is_none")]
    pub enable_node_public_ip: Option<bool>,
    #[doc = "When set to true, AKS deploys a daemonset and host services to sync custom certificate authorities from a user-provided config map into node trust stores. Defaults to false."]
    #[serde(rename = "enableCustomCATrust", default, skip_serializing_if = "Option::is_none")]
    pub enable_custom_ca_trust: Option<bool>,
    #[doc = "This is of the form: /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/publicIPPrefixes/{publicIPPrefixName}"]
    #[serde(rename = "nodePublicIPPrefixID", default, skip_serializing_if = "Option::is_none")]
    pub node_public_ip_prefix_id: Option<String>,
    #[doc = "The Virtual Machine Scale Set priority."]
    #[serde(rename = "scaleSetPriority", default, skip_serializing_if = "Option::is_none")]
    pub scale_set_priority: Option<ScaleSetPriority>,
    #[doc = "The eviction policy specifies what to do with the VM when it is evicted. The default is Delete. For more information about eviction see [spot VMs](https://docs.microsoft.com/azure/virtual-machines/spot-vms)"]
    #[serde(rename = "scaleSetEvictionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub scale_set_eviction_policy: Option<ScaleSetEvictionPolicy>,
    #[doc = "Possible values are any decimal value greater than zero or -1 which indicates the willingness to pay any on-demand price. For more details on spot pricing, see [spot VMs pricing](https://docs.microsoft.com/azure/virtual-machines/spot-vms#pricing)"]
    #[serde(rename = "spotMaxPrice", default, skip_serializing_if = "Option::is_none")]
    pub spot_max_price: Option<SpotMaxPrice>,
    #[doc = "The tags to be persisted on the agent pool virtual machine scale set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The node labels to be persisted across all nodes in agent pool."]
    #[serde(rename = "nodeLabels", default, skip_serializing_if = "Option::is_none")]
    pub node_labels: Option<serde_json::Value>,
    #[doc = "The taints added to new nodes during node pool create and scale. For example, key=value:NoSchedule."]
    #[serde(rename = "nodeTaints", default, skip_serializing_if = "Vec::is_empty")]
    pub node_taints: Vec<String>,
    #[doc = "The ID for Proximity Placement Group."]
    #[serde(rename = "proximityPlacementGroupID", default, skip_serializing_if = "Option::is_none")]
    pub proximity_placement_group_id: Option<ProximityPlacementGroupId>,
    #[doc = "See [AKS custom node configuration](https://docs.microsoft.com/azure/aks/custom-node-configuration) for more details."]
    #[serde(rename = "kubeletConfig", default, skip_serializing_if = "Option::is_none")]
    pub kubelet_config: Option<KubeletConfig>,
    #[doc = "See [AKS custom node configuration](https://docs.microsoft.com/azure/aks/custom-node-configuration) for more details."]
    #[serde(rename = "linuxOSConfig", default, skip_serializing_if = "Option::is_none")]
    pub linux_os_config: Option<LinuxOsConfig>,
    #[doc = "This is only supported on certain VM sizes and in certain Azure regions. For more information, see: https://docs.microsoft.com/azure/aks/enable-host-encryption"]
    #[serde(rename = "enableEncryptionAtHost", default, skip_serializing_if = "Option::is_none")]
    pub enable_encryption_at_host: Option<bool>,
    #[doc = "Whether to enable UltraSSD"]
    #[serde(rename = "enableUltraSSD", default, skip_serializing_if = "Option::is_none")]
    pub enable_ultra_ssd: Option<bool>,
    #[doc = "See [Add a FIPS-enabled node pool](https://docs.microsoft.com/azure/aks/use-multiple-node-pools#add-a-fips-enabled-node-pool-preview) for more details."]
    #[serde(rename = "enableFIPS", default, skip_serializing_if = "Option::is_none")]
    pub enable_fips: Option<bool>,
    #[doc = "GPUInstanceProfile to be used to specify GPU MIG instance profile for supported GPU VM SKU."]
    #[serde(rename = "gpuInstanceProfile", default, skip_serializing_if = "Option::is_none")]
    pub gpu_instance_profile: Option<GpuInstanceProfile>,
    #[doc = "Data used when creating a target resource from a source resource."]
    #[serde(rename = "creationData", default, skip_serializing_if = "Option::is_none")]
    pub creation_data: Option<CreationData>,
    #[doc = "Capacity Reservation Group ID for AgentPool to associate"]
    #[serde(rename = "capacityReservationGroupID", default, skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_group_id: Option<CapacityReservationGroupId>,
    #[doc = "This is of the form: /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/hostGroups/{hostGroupName}. For more information see [Azure dedicated hosts](https://docs.microsoft.com/azure/virtual-machines/dedicated-hosts)."]
    #[serde(rename = "hostGroupID", default, skip_serializing_if = "Option::is_none")]
    pub host_group_id: Option<String>,
}
impl ManagedClusterAgentPoolProfileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Auto upgrade profile for a managed cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterAutoUpgradeProfile {
    #[doc = "For more information see [setting the AKS cluster auto-upgrade channel](https://docs.microsoft.com/azure/aks/upgrade-cluster#set-auto-upgrade-channel)."]
    #[serde(rename = "upgradeChannel", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_channel: Option<managed_cluster_auto_upgrade_profile::UpgradeChannel>,
}
impl ManagedClusterAutoUpgradeProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_cluster_auto_upgrade_profile {
    use super::*;
    #[doc = "For more information see [setting the AKS cluster auto-upgrade channel](https://docs.microsoft.com/azure/aks/upgrade-cluster#set-auto-upgrade-channel)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UpgradeChannel")]
    pub enum UpgradeChannel {
        #[serde(rename = "rapid")]
        Rapid,
        #[serde(rename = "stable")]
        Stable,
        #[serde(rename = "patch")]
        Patch,
        #[serde(rename = "node-image")]
        NodeImage,
        #[serde(rename = "none")]
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UpgradeChannel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UpgradeChannel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UpgradeChannel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Rapid => serializer.serialize_unit_variant("UpgradeChannel", 0u32, "rapid"),
                Self::Stable => serializer.serialize_unit_variant("UpgradeChannel", 1u32, "stable"),
                Self::Patch => serializer.serialize_unit_variant("UpgradeChannel", 2u32, "patch"),
                Self::NodeImage => serializer.serialize_unit_variant("UpgradeChannel", 3u32, "node-image"),
                Self::None => serializer.serialize_unit_variant("UpgradeChannel", 4u32, "none"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Cluster HTTP proxy configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterHttpProxyConfig {
    #[doc = "The HTTP proxy server endpoint to use."]
    #[serde(rename = "httpProxy", default, skip_serializing_if = "Option::is_none")]
    pub http_proxy: Option<String>,
    #[doc = "The HTTPS proxy server endpoint to use."]
    #[serde(rename = "httpsProxy", default, skip_serializing_if = "Option::is_none")]
    pub https_proxy: Option<String>,
    #[doc = "The endpoints that should not go through proxy."]
    #[serde(rename = "noProxy", default, skip_serializing_if = "Vec::is_empty")]
    pub no_proxy: Vec<String>,
    #[doc = "A read-only list of all endpoints for which traffic should not be sent to the proxy. This list is a superset of noProxy and values injected by AKS."]
    #[serde(rename = "effectiveNoProxy", default, skip_serializing_if = "Vec::is_empty")]
    pub effective_no_proxy: Vec<String>,
    #[doc = "Alternative CA cert to use for connecting to proxy servers."]
    #[serde(rename = "trustedCa", default, skip_serializing_if = "Option::is_none")]
    pub trusted_ca: Option<String>,
}
impl ManagedClusterHttpProxyConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the managed cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterIdentity {
    #[doc = "The principal id of the system assigned identity which is used by master components."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id of the system assigned identity which is used by master components."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "For more information see [use managed identities in AKS](https://docs.microsoft.com/azure/aks/use-managed-identity)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<managed_cluster_identity::Type>,
    #[doc = "The keys must be ARM resource IDs in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl ManagedClusterIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_cluster_identity {
    use super::*;
    #[doc = "For more information see [use managed identities in AKS](https://docs.microsoft.com/azure/aks/use-managed-identity)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        None,
    }
}
#[doc = "Ingress profile for the container service cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterIngressProfile {
    #[doc = "Web App Routing settings for the ingress profile."]
    #[serde(rename = "webAppRouting", default, skip_serializing_if = "Option::is_none")]
    pub web_app_routing: Option<ManagedClusterIngressProfileWebAppRouting>,
}
impl ManagedClusterIngressProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web App Routing settings for the ingress profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterIngressProfileWebAppRouting {
    #[doc = "Whether to enable Web App Routing."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Resource ID of the DNS Zone to be associated with the web app. Used only when Web App Routing is enabled."]
    #[serde(rename = "dnsZoneResourceId", default, skip_serializing_if = "Option::is_none")]
    pub dns_zone_resource_id: Option<String>,
}
impl ManagedClusterIngressProfileWebAppRouting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from the List Managed Clusters operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterListResult {
    #[doc = "The list of managed clusters."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedCluster>,
    #[doc = "The URL to get the next set of managed cluster results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedClusterListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagedClusterListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Profile of the managed cluster load balancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterLoadBalancerProfile {
    #[doc = "Desired managed outbound IPs for the cluster load balancer."]
    #[serde(rename = "managedOutboundIPs", default, skip_serializing_if = "Option::is_none")]
    pub managed_outbound_i_ps: Option<managed_cluster_load_balancer_profile::ManagedOutboundIPs>,
    #[doc = "Desired outbound IP Prefix resources for the cluster load balancer."]
    #[serde(rename = "outboundIPPrefixes", default, skip_serializing_if = "Option::is_none")]
    pub outbound_ip_prefixes: Option<managed_cluster_load_balancer_profile::OutboundIpPrefixes>,
    #[doc = "Desired outbound IP resources for the cluster load balancer."]
    #[serde(rename = "outboundIPs", default, skip_serializing_if = "Option::is_none")]
    pub outbound_i_ps: Option<managed_cluster_load_balancer_profile::OutboundIPs>,
    #[doc = "The effective outbound IP resources of the cluster load balancer."]
    #[serde(rename = "effectiveOutboundIPs", default, skip_serializing_if = "Vec::is_empty")]
    pub effective_outbound_i_ps: Vec<ResourceReference>,
    #[doc = "The desired number of allocated SNAT ports per VM. Allowed values are in the range of 0 to 64000 (inclusive). The default value is 0 which results in Azure dynamically allocating ports."]
    #[serde(rename = "allocatedOutboundPorts", default, skip_serializing_if = "Option::is_none")]
    pub allocated_outbound_ports: Option<i32>,
    #[doc = "Desired outbound flow idle timeout in minutes. Allowed values are in the range of 4 to 120 (inclusive). The default value is 30 minutes."]
    #[serde(rename = "idleTimeoutInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_in_minutes: Option<i32>,
    #[doc = "Enable multiple standard load balancers per AKS cluster or not."]
    #[serde(rename = "enableMultipleStandardLoadBalancers", default, skip_serializing_if = "Option::is_none")]
    pub enable_multiple_standard_load_balancers: Option<bool>,
}
impl ManagedClusterLoadBalancerProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_cluster_load_balancer_profile {
    use super::*;
    #[doc = "Desired managed outbound IPs for the cluster load balancer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ManagedOutboundIPs {
        #[doc = "The desired number of IPv4 outbound IPs created/managed by Azure for the cluster load balancer. Allowed values must be in the range of 1 to 100 (inclusive). The default value is 1. "]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub count: Option<i32>,
        #[doc = "The desired number of IPv6 outbound IPs created/managed by Azure for the cluster load balancer. Allowed values must be in the range of 1 to 100 (inclusive). The default value is 0 for single-stack and 1 for dual-stack. "]
        #[serde(rename = "countIPv6", default, skip_serializing_if = "Option::is_none")]
        pub count_i_pv6: Option<i32>,
    }
    impl ManagedOutboundIPs {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Desired outbound IP Prefix resources for the cluster load balancer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct OutboundIpPrefixes {
        #[doc = "A list of public IP prefix resources."]
        #[serde(rename = "publicIPPrefixes", default, skip_serializing_if = "Vec::is_empty")]
        pub public_ip_prefixes: Vec<ResourceReference>,
    }
    impl OutboundIpPrefixes {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Desired outbound IP resources for the cluster load balancer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct OutboundIPs {
        #[doc = "A list of public IP resources."]
        #[serde(rename = "publicIPs", default, skip_serializing_if = "Vec::is_empty")]
        pub public_i_ps: Vec<ResourceReference>,
    }
    impl OutboundIPs {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Profile of the managed outbound IP resources of the managed cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterManagedOutboundIpProfile {
    #[doc = "The desired number of outbound IPs created/managed by Azure. Allowed values must be in the range of 1 to 16 (inclusive). The default value is 1. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
impl ManagedClusterManagedOutboundIpProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Profile of the managed cluster NAT gateway."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterNatGatewayProfile {
    #[doc = "Profile of the managed outbound IP resources of the managed cluster."]
    #[serde(rename = "managedOutboundIPProfile", default, skip_serializing_if = "Option::is_none")]
    pub managed_outbound_ip_profile: Option<ManagedClusterManagedOutboundIpProfile>,
    #[doc = "The effective outbound IP resources of the cluster NAT gateway."]
    #[serde(rename = "effectiveOutboundIPs", default, skip_serializing_if = "Vec::is_empty")]
    pub effective_outbound_i_ps: Vec<ResourceReference>,
    #[doc = "Desired outbound flow idle timeout in minutes. Allowed values are in the range of 4 to 120 (inclusive). The default value is 4 minutes."]
    #[serde(rename = "idleTimeoutInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_in_minutes: Option<i32>,
}
impl ManagedClusterNatGatewayProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The OIDC issuer profile of the Managed Cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterOidcIssuerProfile {
    #[doc = "The OIDC issuer url of the Managed Cluster."]
    #[serde(rename = "issuerURL", default, skip_serializing_if = "Option::is_none")]
    pub issuer_url: Option<String>,
    #[doc = "Whether the OIDC issuer is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl ManagedClusterOidcIssuerProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about the pod identity assigned to the Managed Cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterPodIdentity {
    #[doc = "The name of the pod identity."]
    pub name: String,
    #[doc = "The namespace of the pod identity."]
    pub namespace: String,
    #[doc = "The binding selector to use for the AzureIdentityBinding resource."]
    #[serde(rename = "bindingSelector", default, skip_serializing_if = "Option::is_none")]
    pub binding_selector: Option<String>,
    #[doc = "Details about a user assigned identity."]
    pub identity: UserAssignedIdentity,
    #[doc = "The current provisioning state of the pod identity."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<managed_cluster_pod_identity::ProvisioningState>,
    #[serde(rename = "provisioningInfo", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_info: Option<managed_cluster_pod_identity::ProvisioningInfo>,
}
impl ManagedClusterPodIdentity {
    pub fn new(name: String, namespace: String, identity: UserAssignedIdentity) -> Self {
        Self {
            name,
            namespace,
            binding_selector: None,
            identity,
            provisioning_state: None,
            provisioning_info: None,
        }
    }
}
pub mod managed_cluster_pod_identity {
    use super::*;
    #[doc = "The current provisioning state of the pod identity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Assigned,
        Updating,
        Deleting,
        Failed,
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
                Self::Assigned => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Assigned"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ProvisioningInfo {
        #[doc = "An error response from the pod identity provisioning."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error: Option<ManagedClusterPodIdentityProvisioningError>,
    }
    impl ProvisioningInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "See [disable AAD Pod Identity for a specific Pod/Application](https://azure.github.io/aad-pod-identity/docs/configure/application_exception/) for more details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterPodIdentityException {
    #[doc = "The name of the pod identity exception."]
    pub name: String,
    #[doc = "The namespace of the pod identity exception."]
    pub namespace: String,
    #[doc = "The pod labels to match."]
    #[serde(rename = "podLabels")]
    pub pod_labels: serde_json::Value,
}
impl ManagedClusterPodIdentityException {
    pub fn new(name: String, namespace: String, pod_labels: serde_json::Value) -> Self {
        Self {
            name,
            namespace,
            pod_labels,
        }
    }
}
#[doc = "See [use AAD pod identity](https://docs.microsoft.com/azure/aks/use-azure-ad-pod-identity) for more details on pod identity integration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterPodIdentityProfile {
    #[doc = "Whether the pod identity addon is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Running in Kubenet is disabled by default due to the security related nature of AAD Pod Identity and the risks of IP spoofing. See [using Kubenet network plugin with AAD Pod Identity](https://docs.microsoft.com/azure/aks/use-azure-ad-pod-identity#using-kubenet-network-plugin-with-azure-active-directory-pod-managed-identities) for more information."]
    #[serde(rename = "allowNetworkPluginKubenet", default, skip_serializing_if = "Option::is_none")]
    pub allow_network_plugin_kubenet: Option<bool>,
    #[doc = "The pod identities to use in the cluster."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Vec::is_empty")]
    pub user_assigned_identities: Vec<ManagedClusterPodIdentity>,
    #[doc = "The pod identity exceptions to allow."]
    #[serde(rename = "userAssignedIdentityExceptions", default, skip_serializing_if = "Vec::is_empty")]
    pub user_assigned_identity_exceptions: Vec<ManagedClusterPodIdentityException>,
}
impl ManagedClusterPodIdentityProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the pod identity provisioning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterPodIdentityProvisioningError {
    #[doc = "An error response from the pod identity provisioning."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ManagedClusterPodIdentityProvisioningErrorBody>,
}
impl ManagedClusterPodIdentityProvisioningError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the pod identity provisioning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterPodIdentityProvisioningErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ManagedClusterPodIdentityProvisioningErrorBody>,
}
impl ManagedClusterPodIdentityProvisioningErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of available upgrade versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterPoolUpgradeProfile {
    #[doc = "The Kubernetes version (major.minor.patch)."]
    #[serde(rename = "kubernetesVersion")]
    pub kubernetes_version: String,
    #[doc = "The Agent Pool name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The operating system type. The default is Linux."]
    #[serde(rename = "osType")]
    pub os_type: OsType,
    #[doc = "List of orchestrator types and versions available for upgrade."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub upgrades: Vec<serde_json::Value>,
}
impl ManagedClusterPoolUpgradeProfile {
    pub fn new(kubernetes_version: String, os_type: OsType) -> Self {
        Self {
            kubernetes_version,
            name: None,
            os_type,
            upgrades: Vec::new(),
        }
    }
}
#[doc = "Properties of the managed cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterProperties {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Describes the Power State of the cluster"]
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<PowerState>,
    #[doc = "Data used when creating a target resource from a source resource."]
    #[serde(rename = "creationData", default, skip_serializing_if = "Option::is_none")]
    pub creation_data: Option<CreationData>,
    #[doc = "The max number of agent pools for the managed cluster."]
    #[serde(rename = "maxAgentPools", default, skip_serializing_if = "Option::is_none")]
    pub max_agent_pools: Option<i32>,
    #[doc = "When you upgrade a supported AKS cluster, Kubernetes minor versions cannot be skipped. All upgrades must be performed sequentially by major version number. For example, upgrades between 1.14.x -> 1.15.x or 1.15.x -> 1.16.x are allowed, however 1.14.x -> 1.16.x is not allowed. See [upgrading an AKS cluster](https://docs.microsoft.com/azure/aks/upgrade-cluster) for more details."]
    #[serde(rename = "kubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    #[doc = "The version of Kubernetes the Managed Cluster is running."]
    #[serde(rename = "currentKubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_kubernetes_version: Option<String>,
    #[doc = "This cannot be updated once the Managed Cluster has been created."]
    #[serde(rename = "dnsPrefix", default, skip_serializing_if = "Option::is_none")]
    pub dns_prefix: Option<String>,
    #[doc = "This cannot be updated once the Managed Cluster has been created."]
    #[serde(rename = "fqdnSubdomain", default, skip_serializing_if = "Option::is_none")]
    pub fqdn_subdomain: Option<String>,
    #[doc = "The FQDN of the master pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "The FQDN of private cluster."]
    #[serde(rename = "privateFQDN", default, skip_serializing_if = "Option::is_none")]
    pub private_fqdn: Option<String>,
    #[doc = "The Azure Portal requires certain Cross-Origin Resource Sharing (CORS) headers to be sent in some responses, which Kubernetes APIServer doesn't handle by default. This special FQDN supports CORS, allowing the Azure Portal to function properly."]
    #[serde(rename = "azurePortalFQDN", default, skip_serializing_if = "Option::is_none")]
    pub azure_portal_fqdn: Option<String>,
    #[doc = "The agent pool properties."]
    #[serde(rename = "agentPoolProfiles", default, skip_serializing_if = "Vec::is_empty")]
    pub agent_pool_profiles: Vec<ManagedClusterAgentPoolProfile>,
    #[doc = "Profile for Linux VMs in the container service cluster."]
    #[serde(rename = "linuxProfile", default, skip_serializing_if = "Option::is_none")]
    pub linux_profile: Option<ContainerServiceLinuxProfile>,
    #[doc = "Profile for Windows VMs in the managed cluster."]
    #[serde(rename = "windowsProfile", default, skip_serializing_if = "Option::is_none")]
    pub windows_profile: Option<ManagedClusterWindowsProfile>,
    #[doc = "Information about a service principal identity for the cluster to use for manipulating Azure APIs."]
    #[serde(rename = "servicePrincipalProfile", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_profile: Option<ManagedClusterServicePrincipalProfile>,
    #[doc = "The profile of managed cluster add-on."]
    #[serde(rename = "addonProfiles", default, skip_serializing_if = "Option::is_none")]
    pub addon_profiles: Option<serde_json::Value>,
    #[doc = "See [use AAD pod identity](https://docs.microsoft.com/azure/aks/use-azure-ad-pod-identity) for more details on pod identity integration."]
    #[serde(rename = "podIdentityProfile", default, skip_serializing_if = "Option::is_none")]
    pub pod_identity_profile: Option<ManagedClusterPodIdentityProfile>,
    #[doc = "The OIDC issuer profile of the Managed Cluster."]
    #[serde(rename = "oidcIssuerProfile", default, skip_serializing_if = "Option::is_none")]
    pub oidc_issuer_profile: Option<ManagedClusterOidcIssuerProfile>,
    #[doc = "The name of the resource group containing agent pool nodes."]
    #[serde(rename = "nodeResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub node_resource_group: Option<String>,
    #[doc = "Whether to enable Kubernetes Role-Based Access Control."]
    #[serde(rename = "enableRBAC", default, skip_serializing_if = "Option::is_none")]
    pub enable_rbac: Option<bool>,
    #[doc = "(DEPRECATING) Whether to enable Kubernetes pod security policy (preview). This feature is set for removal on October 15th, 2020. Learn more at aka.ms/aks/azpodpolicy."]
    #[serde(rename = "enablePodSecurityPolicy", default, skip_serializing_if = "Option::is_none")]
    pub enable_pod_security_policy: Option<bool>,
    #[doc = "The default value is false. It can be enabled/disabled on creation and updation of the managed cluster. See [https://aka.ms/NamespaceARMResource](https://aka.ms/NamespaceARMResource) for more details on Namespace as a ARM Resource."]
    #[serde(rename = "enableNamespaceResources", default, skip_serializing_if = "Option::is_none")]
    pub enable_namespace_resources: Option<bool>,
    #[doc = "Profile of network configuration."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<ContainerServiceNetworkProfile>,
    #[doc = "For more details see [managed AAD on AKS](https://docs.microsoft.com/azure/aks/managed-aad)."]
    #[serde(rename = "aadProfile", default, skip_serializing_if = "Option::is_none")]
    pub aad_profile: Option<ManagedClusterAadProfile>,
    #[doc = "Auto upgrade profile for a managed cluster."]
    #[serde(rename = "autoUpgradeProfile", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_profile: Option<ManagedClusterAutoUpgradeProfile>,
    #[doc = "Parameters to be applied to the cluster-autoscaler when enabled"]
    #[serde(rename = "autoScalerProfile", default, skip_serializing_if = "Option::is_none")]
    pub auto_scaler_profile: Option<managed_cluster_properties::AutoScalerProfile>,
    #[doc = "Access profile for managed cluster API server."]
    #[serde(rename = "apiServerAccessProfile", default, skip_serializing_if = "Option::is_none")]
    pub api_server_access_profile: Option<ManagedClusterApiServerAccessProfile>,
    #[doc = "This is of the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/diskEncryptionSets/{encryptionSetName}'"]
    #[serde(rename = "diskEncryptionSetID", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
    #[doc = "Identities associated with the cluster."]
    #[serde(rename = "identityProfile", default, skip_serializing_if = "Option::is_none")]
    pub identity_profile: Option<serde_json::Value>,
    #[doc = "Private link resources associated with the cluster."]
    #[serde(rename = "privateLinkResources", default, skip_serializing_if = "Vec::is_empty")]
    pub private_link_resources: Vec<PrivateLinkResource>,
    #[doc = "If set to true, getting static credentials will be disabled for this cluster. This must only be used on Managed Clusters that are AAD enabled. For more details see [disable local accounts](https://docs.microsoft.com/azure/aks/managed-aad#disable-local-accounts-preview)."]
    #[serde(rename = "disableLocalAccounts", default, skip_serializing_if = "Option::is_none")]
    pub disable_local_accounts: Option<bool>,
    #[doc = "Cluster HTTP proxy configuration."]
    #[serde(rename = "httpProxyConfig", default, skip_serializing_if = "Option::is_none")]
    pub http_proxy_config: Option<ManagedClusterHttpProxyConfig>,
    #[doc = "Security profile for the container service cluster."]
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<ManagedClusterSecurityProfile>,
    #[doc = "Storage profile for the container service cluster."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<ManagedClusterStorageProfile>,
    #[doc = "Ingress profile for the container service cluster."]
    #[serde(rename = "ingressProfile", default, skip_serializing_if = "Option::is_none")]
    pub ingress_profile: Option<ManagedClusterIngressProfile>,
    #[doc = "Allow or deny public network access for AKS"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<managed_cluster_properties::PublicNetworkAccess>,
}
impl ManagedClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_cluster_properties {
    use super::*;
    #[doc = "Parameters to be applied to the cluster-autoscaler when enabled"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct AutoScalerProfile {
        #[doc = "Valid values are 'true' and 'false'"]
        #[serde(rename = "balance-similar-node-groups", default, skip_serializing_if = "Option::is_none")]
        pub balance_similar_node_groups: Option<String>,
        #[doc = "If not specified, the default is 'random'. See [expanders](https://github.com/kubernetes/autoscaler/blob/master/cluster-autoscaler/FAQ.md#what-are-expanders) for more information."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expander: Option<auto_scaler_profile::Expander>,
        #[doc = "The default is 10."]
        #[serde(rename = "max-empty-bulk-delete", default, skip_serializing_if = "Option::is_none")]
        pub max_empty_bulk_delete: Option<String>,
        #[doc = "The default is 600."]
        #[serde(rename = "max-graceful-termination-sec", default, skip_serializing_if = "Option::is_none")]
        pub max_graceful_termination_sec: Option<String>,
        #[doc = "The default is '15m'. Values must be an integer followed by an 'm'. No unit of time other than minutes (m) is supported."]
        #[serde(rename = "max-node-provision-time", default, skip_serializing_if = "Option::is_none")]
        pub max_node_provision_time: Option<String>,
        #[doc = "The default is 45. The maximum is 100 and the minimum is 0."]
        #[serde(rename = "max-total-unready-percentage", default, skip_serializing_if = "Option::is_none")]
        pub max_total_unready_percentage: Option<String>,
        #[doc = "For scenarios like burst/batch scale where you don't want CA to act before the kubernetes scheduler could schedule all the pods, you can tell CA to ignore unscheduled pods before they're a certain age. The default is '0s'. Values must be an integer followed by a unit ('s' for seconds, 'm' for minutes, 'h' for hours, etc)."]
        #[serde(rename = "new-pod-scale-up-delay", default, skip_serializing_if = "Option::is_none")]
        pub new_pod_scale_up_delay: Option<String>,
        #[doc = "This must be an integer. The default is 3."]
        #[serde(rename = "ok-total-unready-count", default, skip_serializing_if = "Option::is_none")]
        pub ok_total_unready_count: Option<String>,
        #[doc = "The default is '10'. Values must be an integer number of seconds."]
        #[serde(rename = "scan-interval", default, skip_serializing_if = "Option::is_none")]
        pub scan_interval: Option<String>,
        #[doc = "The default is '10m'. Values must be an integer followed by an 'm'. No unit of time other than minutes (m) is supported."]
        #[serde(rename = "scale-down-delay-after-add", default, skip_serializing_if = "Option::is_none")]
        pub scale_down_delay_after_add: Option<String>,
        #[doc = "The default is the scan-interval. Values must be an integer followed by an 'm'. No unit of time other than minutes (m) is supported."]
        #[serde(rename = "scale-down-delay-after-delete", default, skip_serializing_if = "Option::is_none")]
        pub scale_down_delay_after_delete: Option<String>,
        #[doc = "The default is '3m'. Values must be an integer followed by an 'm'. No unit of time other than minutes (m) is supported."]
        #[serde(rename = "scale-down-delay-after-failure", default, skip_serializing_if = "Option::is_none")]
        pub scale_down_delay_after_failure: Option<String>,
        #[doc = "The default is '10m'. Values must be an integer followed by an 'm'. No unit of time other than minutes (m) is supported."]
        #[serde(rename = "scale-down-unneeded-time", default, skip_serializing_if = "Option::is_none")]
        pub scale_down_unneeded_time: Option<String>,
        #[doc = "The default is '20m'. Values must be an integer followed by an 'm'. No unit of time other than minutes (m) is supported."]
        #[serde(rename = "scale-down-unready-time", default, skip_serializing_if = "Option::is_none")]
        pub scale_down_unready_time: Option<String>,
        #[doc = "The default is '0.5'."]
        #[serde(rename = "scale-down-utilization-threshold", default, skip_serializing_if = "Option::is_none")]
        pub scale_down_utilization_threshold: Option<String>,
        #[doc = "The default is true."]
        #[serde(rename = "skip-nodes-with-local-storage", default, skip_serializing_if = "Option::is_none")]
        pub skip_nodes_with_local_storage: Option<String>,
        #[doc = "The default is true."]
        #[serde(rename = "skip-nodes-with-system-pods", default, skip_serializing_if = "Option::is_none")]
        pub skip_nodes_with_system_pods: Option<String>,
    }
    impl AutoScalerProfile {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod auto_scaler_profile {
        use super::*;
        #[doc = "If not specified, the default is 'random'. See [expanders](https://github.com/kubernetes/autoscaler/blob/master/cluster-autoscaler/FAQ.md#what-are-expanders) for more information."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Expander")]
        pub enum Expander {
            #[serde(rename = "least-waste")]
            LeastWaste,
            #[serde(rename = "most-pods")]
            MostPods,
            #[serde(rename = "priority")]
            Priority,
            #[serde(rename = "random")]
            Random,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for Expander {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for Expander {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for Expander {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::LeastWaste => serializer.serialize_unit_variant("Expander", 0u32, "least-waste"),
                    Self::MostPods => serializer.serialize_unit_variant("Expander", 1u32, "most-pods"),
                    Self::Priority => serializer.serialize_unit_variant("Expander", 2u32, "priority"),
                    Self::Random => serializer.serialize_unit_variant("Expander", 3u32, "random"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
    #[doc = "Allow or deny public network access for AKS"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "managed cluster properties for snapshot, these properties are read only."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterPropertiesForSnapshot {
    #[doc = "The current kubernetes version."]
    #[serde(rename = "kubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    #[doc = "The SKU of a Managed Cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ManagedClusterSku>,
    #[doc = "Whether the cluster has enabled Kubernetes Role-Based Access Control or not."]
    #[serde(rename = "enableRbac", default, skip_serializing_if = "Option::is_none")]
    pub enable_rbac: Option<bool>,
    #[doc = "network profile for managed cluster snapshot, these properties are read only."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfileForSnapshot>,
}
impl ManagedClusterPropertiesForSnapshot {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SKU of a Managed Cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterSku {
    #[doc = "The name of a managed cluster SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<managed_cluster_sku::Name>,
    #[doc = "If not specified, the default is 'Free'. See [uptime SLA](https://docs.microsoft.com/azure/aks/uptime-sla) for more details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<managed_cluster_sku::Tier>,
}
impl ManagedClusterSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_cluster_sku {
    use super::*;
    #[doc = "The name of a managed cluster SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Basic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("Name", 0u32, "Basic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "If not specified, the default is 'Free'. See [uptime SLA](https://docs.microsoft.com/azure/aks/uptime-sla) for more details."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Paid,
        Free,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Paid => serializer.serialize_unit_variant("Tier", 0u32, "Paid"),
                Self::Free => serializer.serialize_unit_variant("Tier", 1u32, "Free"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Security profile for the container service cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterSecurityProfile {
    #[doc = "Azure Defender settings for the security profile."]
    #[serde(rename = "azureDefender", default, skip_serializing_if = "Option::is_none")]
    pub azure_defender: Option<ManagedClusterSecurityProfileAzureDefender>,
    #[doc = "Azure Key Vault key management service settings for the security profile."]
    #[serde(rename = "azureKeyVaultKms", default, skip_serializing_if = "Option::is_none")]
    pub azure_key_vault_kms: Option<AzureKeyVaultKms>,
    #[doc = "Workload Identity settings for the security profile."]
    #[serde(rename = "workloadIdentity", default, skip_serializing_if = "Option::is_none")]
    pub workload_identity: Option<ManagedClusterSecurityProfileWorkloadIdentity>,
}
impl ManagedClusterSecurityProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Defender settings for the security profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterSecurityProfileAzureDefender {
    #[doc = "Whether to enable Azure Defender"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Resource ID of the Log Analytics workspace to be associated with Azure Defender.  When Azure Defender is enabled, this field is required and must be a valid workspace resource ID. When Azure Defender is disabled, leave the field empty."]
    #[serde(rename = "logAnalyticsWorkspaceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_workspace_resource_id: Option<String>,
}
impl ManagedClusterSecurityProfileAzureDefender {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload Identity settings for the security profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterSecurityProfileWorkloadIdentity {
    #[doc = "Whether to enable Workload Identity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl ManagedClusterSecurityProfileWorkloadIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a service principal identity for the cluster to use for manipulating Azure APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterServicePrincipalProfile {
    #[doc = "The ID for the service principal."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "The secret password associated with the service principal in plain text."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}
impl ManagedClusterServicePrincipalProfile {
    pub fn new(client_id: String) -> Self {
        Self { client_id, secret: None }
    }
}
#[doc = "A managed cluster snapshot resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterSnapshot {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties for a managed cluster snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedClusterSnapshotProperties>,
}
impl ManagedClusterSnapshot {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response from the List Managed Cluster Snapshots operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterSnapshotListResult {
    #[doc = "The list of managed cluster snapshots."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedClusterSnapshot>,
    #[doc = "The URL to get the next set of managed cluster snapshot results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedClusterSnapshotListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagedClusterSnapshotListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for a managed cluster snapshot."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterSnapshotProperties {
    #[doc = "Data used when creating a target resource from a source resource."]
    #[serde(rename = "creationData", default, skip_serializing_if = "Option::is_none")]
    pub creation_data: Option<CreationData>,
    #[doc = "The type of a snapshot. The default is NodePool."]
    #[serde(rename = "snapshotType", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<SnapshotType>,
    #[doc = "managed cluster properties for snapshot, these properties are read only."]
    #[serde(rename = "managedClusterPropertiesReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub managed_cluster_properties_read_only: Option<ManagedClusterPropertiesForSnapshot>,
}
impl ManagedClusterSnapshotProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage profile for the container service cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterStorageProfile {
    #[doc = "AzureDisk CSI Driver settings for the storage profile."]
    #[serde(rename = "diskCSIDriver", default, skip_serializing_if = "Option::is_none")]
    pub disk_csi_driver: Option<ManagedClusterStorageProfileDiskCsiDriver>,
    #[doc = "AzureFile CSI Driver settings for the storage profile."]
    #[serde(rename = "fileCSIDriver", default, skip_serializing_if = "Option::is_none")]
    pub file_csi_driver: Option<ManagedClusterStorageProfileFileCsiDriver>,
    #[doc = "Snapshot Controller settings for the storage profile."]
    #[serde(rename = "snapshotController", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_controller: Option<ManagedClusterStorageProfileSnapshotController>,
}
impl ManagedClusterStorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AzureDisk CSI Driver settings for the storage profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterStorageProfileDiskCsiDriver {
    #[doc = "Whether to enable AzureDisk CSI Driver. The default value is true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The version of AzureDisk CSI Driver. The default value is v1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ManagedClusterStorageProfileDiskCsiDriver {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AzureFile CSI Driver settings for the storage profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterStorageProfileFileCsiDriver {
    #[doc = "Whether to enable AzureFile CSI Driver. The default value is true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl ManagedClusterStorageProfileFileCsiDriver {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Snapshot Controller settings for the storage profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterStorageProfileSnapshotController {
    #[doc = "Whether to enable Snapshot Controller. The default value is true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl ManagedClusterStorageProfileSnapshotController {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of available upgrades for compute pools."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterUpgradeProfile {
    #[doc = "The ID of the upgrade profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the upgrade profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the upgrade profile."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Control plane and agent pool upgrade profiles."]
    pub properties: ManagedClusterUpgradeProfileProperties,
}
impl ManagedClusterUpgradeProfile {
    pub fn new(properties: ManagedClusterUpgradeProfileProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "Control plane and agent pool upgrade profiles."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterUpgradeProfileProperties {
    #[doc = "The list of available upgrade versions."]
    #[serde(rename = "controlPlaneProfile")]
    pub control_plane_profile: ManagedClusterPoolUpgradeProfile,
    #[doc = "The list of available upgrade versions for agent pools."]
    #[serde(rename = "agentPoolProfiles")]
    pub agent_pool_profiles: Vec<ManagedClusterPoolUpgradeProfile>,
}
impl ManagedClusterUpgradeProfileProperties {
    pub fn new(
        control_plane_profile: ManagedClusterPoolUpgradeProfile,
        agent_pool_profiles: Vec<ManagedClusterPoolUpgradeProfile>,
    ) -> Self {
        Self {
            control_plane_profile,
            agent_pool_profiles,
        }
    }
}
#[doc = "Profile for Windows VMs in the managed cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterWindowsProfile {
    #[doc = "Specifies the name of the administrator account. <br><br> **Restriction:** Cannot end in \".\" <br><br> **Disallowed values:** \"administrator\", \"admin\", \"user\", \"user1\", \"test\", \"user2\", \"test1\", \"user3\", \"admin1\", \"1\", \"123\", \"a\", \"actuser\", \"adm\", \"admin2\", \"aspnet\", \"backup\", \"console\", \"david\", \"guest\", \"john\", \"owner\", \"root\", \"server\", \"sql\", \"support\", \"support_388945a0\", \"sys\", \"test2\", \"test3\", \"user4\", \"user5\". <br><br> **Minimum-length:** 1 character <br><br> **Max-length:** 20 characters"]
    #[serde(rename = "adminUsername")]
    pub admin_username: String,
    #[doc = "Specifies the password of the administrator account. <br><br> **Minimum-length:** 8 characters <br><br> **Max-length:** 123 characters <br><br> **Complexity requirements:** 3 out of 4 conditions below need to be fulfilled <br> Has lower characters <br>Has upper characters <br> Has a digit <br> Has a special character (Regex match [\\W_]) <br><br> **Disallowed values:** \"abc@123\", \"P@$$w0rd\", \"P@ssw0rd\", \"P@ssword123\", \"Pa$$word\", \"pass@word1\", \"Password!\", \"Password1\", \"Password22\", \"iloveyou!\""]
    #[serde(rename = "adminPassword", default, skip_serializing_if = "Option::is_none")]
    pub admin_password: Option<String>,
    #[doc = "The license type to use for Windows VMs. See [Azure Hybrid User Benefits](https://azure.microsoft.com/pricing/hybrid-benefit/faq/) for more details."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<managed_cluster_windows_profile::LicenseType>,
    #[doc = "For more details on CSI proxy, see the [CSI proxy GitHub repo](https://github.com/kubernetes-csi/csi-proxy)."]
    #[serde(rename = "enableCSIProxy", default, skip_serializing_if = "Option::is_none")]
    pub enable_csi_proxy: Option<bool>,
    #[doc = "Windows gMSA Profile in the managed cluster."]
    #[serde(rename = "gmsaProfile", default, skip_serializing_if = "Option::is_none")]
    pub gmsa_profile: Option<WindowsGmsaProfile>,
}
impl ManagedClusterWindowsProfile {
    pub fn new(admin_username: String) -> Self {
        Self {
            admin_username,
            admin_password: None,
            license_type: None,
            enable_csi_proxy: None,
            gmsa_profile: None,
        }
    }
}
pub mod managed_cluster_windows_profile {
    use super::*;
    #[doc = "The license type to use for Windows VMs. See [Azure Hybrid User Benefits](https://azure.microsoft.com/pricing/hybrid-benefit/faq/) for more details."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        None,
        #[serde(rename = "Windows_Server")]
        WindowsServer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("LicenseType", 0u32, "None"),
                Self::WindowsServer => serializer.serialize_unit_variant("LicenseType", 1u32, "Windows_Server"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "This cannot be specified if networkPlugin is anything other than 'azure'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NetworkMode")]
pub enum NetworkMode {
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "bridge")]
    Bridge,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NetworkMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NetworkMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NetworkMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Transparent => serializer.serialize_unit_variant("NetworkMode", 0u32, "transparent"),
            Self::Bridge => serializer.serialize_unit_variant("NetworkMode", 1u32, "bridge"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Network plugin used for building the Kubernetes network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NetworkPlugin")]
pub enum NetworkPlugin {
    #[serde(rename = "azure")]
    Azure,
    #[serde(rename = "kubenet")]
    Kubenet,
    #[serde(rename = "none")]
    None,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NetworkPlugin {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NetworkPlugin {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NetworkPlugin {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Azure => serializer.serialize_unit_variant("NetworkPlugin", 0u32, "azure"),
            Self::Kubenet => serializer.serialize_unit_variant("NetworkPlugin", 1u32, "kubenet"),
            Self::None => serializer.serialize_unit_variant("NetworkPlugin", 2u32, "none"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for NetworkPlugin {
    fn default() -> Self {
        Self::Kubenet
    }
}
#[doc = "The mode the network plugin should use."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NetworkPluginMode")]
pub enum NetworkPluginMode {
    Overlay,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NetworkPluginMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NetworkPluginMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NetworkPluginMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Overlay => serializer.serialize_unit_variant("NetworkPluginMode", 0u32, "Overlay"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Network policy used for building the Kubernetes network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NetworkPolicy")]
pub enum NetworkPolicy {
    #[serde(rename = "calico")]
    Calico,
    #[serde(rename = "azure")]
    Azure,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NetworkPolicy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NetworkPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NetworkPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Calico => serializer.serialize_unit_variant("NetworkPolicy", 0u32, "calico"),
            Self::Azure => serializer.serialize_unit_variant("NetworkPolicy", 1u32, "azure"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "network profile for managed cluster snapshot, these properties are read only."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfileForSnapshot {
    #[doc = "Network plugin used for building the Kubernetes network."]
    #[serde(rename = "networkPlugin", default, skip_serializing_if = "Option::is_none")]
    pub network_plugin: Option<NetworkPlugin>,
    #[doc = "The mode the network plugin should use."]
    #[serde(rename = "networkPluginMode", default, skip_serializing_if = "Option::is_none")]
    pub network_plugin_mode: Option<NetworkPluginMode>,
    #[doc = "Network policy used for building the Kubernetes network."]
    #[serde(rename = "networkPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_policy: Option<NetworkPolicy>,
    #[doc = "This cannot be specified if networkPlugin is anything other than 'azure'."]
    #[serde(rename = "networkMode", default, skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<NetworkMode>,
    #[doc = "The default is 'standard'. See [Azure Load Balancer SKUs](https://docs.microsoft.com/azure/load-balancer/skus) for more information about the differences between load balancer SKUs."]
    #[serde(rename = "loadBalancerSku", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_sku: Option<LoadBalancerSku>,
}
impl NetworkProfileForSnapshot {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The default is 'Ephemeral' if the VM supports it and has a cache disk larger than the requested OSDiskSizeGB. Otherwise, defaults to 'Managed'. May not be changed after creation. For more information see [Ephemeral OS](https://docs.microsoft.com/azure/aks/cluster-configuration#ephemeral-os)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OsDiskType")]
pub enum OsDiskType {
    Managed,
    Ephemeral,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OsDiskType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OsDiskType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OsDiskType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Managed => serializer.serialize_unit_variant("OsDiskType", 0u32, "Managed"),
            Self::Ephemeral => serializer.serialize_unit_variant("OsDiskType", 1u32, "Ephemeral"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The OS option profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsOptionProfile {
    #[doc = "The ID of the OS option resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the OS option resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the OS option resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The list of OS option properties."]
    pub properties: OsOptionPropertyList,
}
impl OsOptionProfile {
    pub fn new(properties: OsOptionPropertyList) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "OS option property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsOptionProperty {
    #[doc = "The OS type."]
    #[serde(rename = "os-type")]
    pub os_type: String,
    #[doc = "Whether the image is FIPS-enabled."]
    #[serde(rename = "enable-fips-image")]
    pub enable_fips_image: bool,
}
impl OsOptionProperty {
    pub fn new(os_type: String, enable_fips_image: bool) -> Self {
        Self {
            os_type,
            enable_fips_image,
        }
    }
}
#[doc = "The list of OS option properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsOptionPropertyList {
    #[doc = "The list of OS options."]
    #[serde(rename = "osOptionPropertyList")]
    pub os_option_property_list: Vec<OsOptionProperty>,
}
impl OsOptionPropertyList {
    pub fn new(os_option_property_list: Vec<OsOptionProperty>) -> Self {
        Self { os_option_property_list }
    }
}
#[doc = "Specifies the OS SKU used by the agent pool. If not specified, the default is Ubuntu if OSType=Linux or Windows2019 if OSType=Windows. And the default Windows OSSKU will be changed to Windows2022 after Windows2019 is deprecated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Ossku")]
pub enum Ossku {
    Ubuntu,
    #[serde(rename = "CBLMariner")]
    CblMariner,
    Windows2019,
    Windows2022,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Ossku {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Ossku {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Ossku {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ubuntu => serializer.serialize_unit_variant("Ossku", 0u32, "Ubuntu"),
            Self::CblMariner => serializer.serialize_unit_variant("Ossku", 1u32, "CBLMariner"),
            Self::Windows2019 => serializer.serialize_unit_variant("Ossku", 2u32, "Windows2019"),
            Self::Windows2022 => serializer.serialize_unit_variant("Ossku", 3u32, "Windows2022"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The operating system type. The default is Linux."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OsType")]
pub enum OsType {
    Linux,
    Windows,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OsType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OsType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OsType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Linux => serializer.serialize_unit_variant("OsType", 0u32, "Linux"),
            Self::Windows => serializer.serialize_unit_variant("OsType", 1u32, "Windows"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for OsType {
    fn default() -> Self {
        Self::Linux
    }
}
#[doc = "The List Operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The list of operations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationValue>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Operation value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationValue {
    #[doc = "The origin of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes the properties of a Operation Value Display."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationValueDisplay>,
}
impl OperationValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Operation Value Display."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationValueDisplay {
    #[doc = "The display name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The display name of the resource the operation applies to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The resource provider for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
impl OperationValueDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Egress endpoints which AKS agent nodes connect to for common purpose."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundEnvironmentEndpoint {
    #[doc = "The category of endpoints accessed by the AKS agent node, e.g. azure-resource-management, apiserver, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The endpoints that AKS agent nodes connect to"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<EndpointDependency>,
}
impl OutboundEnvironmentEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of OutboundEnvironmentEndpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutboundEnvironmentEndpointCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<OutboundEnvironmentEndpoint>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OutboundEnvironmentEndpointCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OutboundEnvironmentEndpointCollection {
    pub fn new(value: Vec<OutboundEnvironmentEndpoint>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the Power State of the cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PowerState {
    #[doc = "Tells whether the cluster is Running or Stopped"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<power_state::Code>,
}
impl PowerState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod power_state {
    use super::*;
    #[doc = "Tells whether the cluster is Running or Stopped"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Code")]
    pub enum Code {
        Running,
        Stopped,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Code {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Code {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Code {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Running => serializer.serialize_unit_variant("Code", 0u32, "Running"),
                Self::Stopped => serializer.serialize_unit_variant("Code", 1u32, "Stopped"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Private endpoint which a connection belongs to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The resource ID of the private endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[doc = "The ID of the private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of a private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private endpoint connections"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "The collection value."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_endpoint_connection_properties::ProvisioningState>,
    #[doc = "Private endpoint which a connection belongs to."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "The state of a private link service connection."]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            provisioning_state: None,
            private_endpoint: None,
            private_link_service_connection_state,
        }
    }
}
pub mod private_endpoint_connection_properties {
    use super::*;
    #[doc = "The current provisioning state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Creating,
        Deleting,
        Failed,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[doc = "The ID of the private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The group ID of the resource."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The RequiredMembers of the resource"]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The private link service ID of the resource, this field is exposed only to NRP internally."]
    #[serde(rename = "privateLinkServiceID", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_id: Option<String>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourcesListResult {
    #[doc = "The collection value."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
impl PrivateLinkResourcesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The state of a private link service connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private link service connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<private_link_service_connection_state::Status>,
    #[doc = "The private link service connection description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_link_service_connection_state {
    use super::*;
    #[doc = "The private link service connection status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Pending,
        Approved,
        Rejected,
        Disconnected,
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
                Self::Pending => serializer.serialize_unit_variant("Status", 0u32, "Pending"),
                Self::Approved => serializer.serialize_unit_variant("Status", 1u32, "Approved"),
                Self::Rejected => serializer.serialize_unit_variant("Status", 2u32, "Rejected"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 3u32, "Disconnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type ProximityPlacementGroupId = String;
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A reference to an Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceReference {
    #[doc = "The fully qualified Azure resource id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A run command request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunCommandRequest {
    #[doc = "The command to run."]
    pub command: String,
    #[doc = "A base64 encoded zip file containing the files required by the command."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "AuthToken issued for AKS AAD Server App."]
    #[serde(rename = "clusterToken", default, skip_serializing_if = "Option::is_none")]
    pub cluster_token: Option<String>,
}
impl RunCommandRequest {
    pub fn new(command: String) -> Self {
        Self {
            command,
            context: None,
            cluster_token: None,
        }
    }
}
#[doc = "run command result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunCommandResult {
    #[doc = "The command id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The results of a run command"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CommandResultProperties>,
}
impl RunCommandResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes how VMs are added to or removed from Agent Pools. See [billing states](https://docs.microsoft.com/azure/virtual-machines/states-billing)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScaleDownMode")]
pub enum ScaleDownMode {
    Delete,
    Deallocate,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScaleDownMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScaleDownMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScaleDownMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Delete => serializer.serialize_unit_variant("ScaleDownMode", 0u32, "Delete"),
            Self::Deallocate => serializer.serialize_unit_variant("ScaleDownMode", 1u32, "Deallocate"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The eviction policy specifies what to do with the VM when it is evicted. The default is Delete. For more information about eviction see [spot VMs](https://docs.microsoft.com/azure/virtual-machines/spot-vms)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScaleSetEvictionPolicy")]
pub enum ScaleSetEvictionPolicy {
    Delete,
    Deallocate,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScaleSetEvictionPolicy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScaleSetEvictionPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScaleSetEvictionPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Delete => serializer.serialize_unit_variant("ScaleSetEvictionPolicy", 0u32, "Delete"),
            Self::Deallocate => serializer.serialize_unit_variant("ScaleSetEvictionPolicy", 1u32, "Deallocate"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for ScaleSetEvictionPolicy {
    fn default() -> Self {
        Self::Delete
    }
}
#[doc = "The Virtual Machine Scale Set priority."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScaleSetPriority")]
pub enum ScaleSetPriority {
    Spot,
    Regular,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScaleSetPriority {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScaleSetPriority {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScaleSetPriority {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Spot => serializer.serialize_unit_variant("ScaleSetPriority", 0u32, "Spot"),
            Self::Regular => serializer.serialize_unit_variant("ScaleSetPriority", 1u32, "Regular"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for ScaleSetPriority {
    fn default() -> Self {
        Self::Regular
    }
}
#[doc = "A node pool snapshot resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties used to configure a node pool snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotProperties>,
}
impl Snapshot {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response from the List Snapshots operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotListResult {
    #[doc = "The list of snapshots."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Snapshot>,
    #[doc = "The URL to get the next set of snapshot results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SnapshotListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SnapshotListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties used to configure a node pool snapshot."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotProperties {
    #[doc = "Data used when creating a target resource from a source resource."]
    #[serde(rename = "creationData", default, skip_serializing_if = "Option::is_none")]
    pub creation_data: Option<CreationData>,
    #[doc = "The type of a snapshot. The default is NodePool."]
    #[serde(rename = "snapshotType", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<SnapshotType>,
    #[doc = "The version of Kubernetes."]
    #[serde(rename = "kubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    #[doc = "The version of node image."]
    #[serde(rename = "nodeImageVersion", default, skip_serializing_if = "Option::is_none")]
    pub node_image_version: Option<String>,
    #[doc = "The operating system type. The default is Linux."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "Specifies the OS SKU used by the agent pool. If not specified, the default is Ubuntu if OSType=Linux or Windows2019 if OSType=Windows. And the default Windows OSSKU will be changed to Windows2022 after Windows2019 is deprecated."]
    #[serde(rename = "osSku", default, skip_serializing_if = "Option::is_none")]
    pub os_sku: Option<Ossku>,
    #[doc = "The size of the VM."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[doc = "Whether to use a FIPS-enabled OS."]
    #[serde(rename = "enableFIPS", default, skip_serializing_if = "Option::is_none")]
    pub enable_fips: Option<bool>,
}
impl SnapshotProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of a snapshot. The default is NodePool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SnapshotType")]
pub enum SnapshotType {
    NodePool,
    ManagedCluster,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SnapshotType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SnapshotType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SnapshotType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NodePool => serializer.serialize_unit_variant("SnapshotType", 0u32, "NodePool"),
            Self::ManagedCluster => serializer.serialize_unit_variant("SnapshotType", 1u32, "ManagedCluster"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for SnapshotType {
    fn default() -> Self {
        Self::NodePool
    }
}
pub type SpotMaxPrice = f64;
#[doc = "Reference to another subresource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sysctl settings for Linux agent nodes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SysctlConfig {
    #[doc = "Sysctl setting net.core.somaxconn."]
    #[serde(rename = "netCoreSomaxconn", default, skip_serializing_if = "Option::is_none")]
    pub net_core_somaxconn: Option<i32>,
    #[doc = "Sysctl setting net.core.netdev_max_backlog."]
    #[serde(rename = "netCoreNetdevMaxBacklog", default, skip_serializing_if = "Option::is_none")]
    pub net_core_netdev_max_backlog: Option<i32>,
    #[doc = "Sysctl setting net.core.rmem_default."]
    #[serde(rename = "netCoreRmemDefault", default, skip_serializing_if = "Option::is_none")]
    pub net_core_rmem_default: Option<i32>,
    #[doc = "Sysctl setting net.core.rmem_max."]
    #[serde(rename = "netCoreRmemMax", default, skip_serializing_if = "Option::is_none")]
    pub net_core_rmem_max: Option<i32>,
    #[doc = "Sysctl setting net.core.wmem_default."]
    #[serde(rename = "netCoreWmemDefault", default, skip_serializing_if = "Option::is_none")]
    pub net_core_wmem_default: Option<i32>,
    #[doc = "Sysctl setting net.core.wmem_max."]
    #[serde(rename = "netCoreWmemMax", default, skip_serializing_if = "Option::is_none")]
    pub net_core_wmem_max: Option<i32>,
    #[doc = "Sysctl setting net.core.optmem_max."]
    #[serde(rename = "netCoreOptmemMax", default, skip_serializing_if = "Option::is_none")]
    pub net_core_optmem_max: Option<i32>,
    #[doc = "Sysctl setting net.ipv4.tcp_max_syn_backlog."]
    #[serde(rename = "netIpv4TcpMaxSynBacklog", default, skip_serializing_if = "Option::is_none")]
    pub net_ipv4_tcp_max_syn_backlog: Option<i32>,
    #[doc = "Sysctl setting net.ipv4.tcp_max_tw_buckets."]
    #[serde(rename = "netIpv4TcpMaxTwBuckets", default, skip_serializing_if = "Option::is_none")]
    pub net_ipv4_tcp_max_tw_buckets: Option<i32>,
    #[doc = "Sysctl setting net.ipv4.tcp_fin_timeout."]
    #[serde(rename = "netIpv4TcpFinTimeout", default, skip_serializing_if = "Option::is_none")]
    pub net_ipv4_tcp_fin_timeout: Option<i32>,
    #[doc = "Sysctl setting net.ipv4.tcp_keepalive_time."]
    #[serde(rename = "netIpv4TcpKeepaliveTime", default, skip_serializing_if = "Option::is_none")]
    pub net_ipv4_tcp_keepalive_time: Option<i32>,
    #[doc = "Sysctl setting net.ipv4.tcp_keepalive_probes."]
    #[serde(rename = "netIpv4TcpKeepaliveProbes", default, skip_serializing_if = "Option::is_none")]
    pub net_ipv4_tcp_keepalive_probes: Option<i32>,
    #[doc = "Sysctl setting net.ipv4.tcp_keepalive_intvl."]
    #[serde(rename = "netIpv4TcpkeepaliveIntvl", default, skip_serializing_if = "Option::is_none")]
    pub net_ipv4_tcpkeepalive_intvl: Option<i32>,
    #[doc = "Sysctl setting net.ipv4.tcp_tw_reuse."]
    #[serde(rename = "netIpv4TcpTwReuse", default, skip_serializing_if = "Option::is_none")]
    pub net_ipv4_tcp_tw_reuse: Option<bool>,
    #[doc = "Sysctl setting net.ipv4.ip_local_port_range."]
    #[serde(rename = "netIpv4IpLocalPortRange", default, skip_serializing_if = "Option::is_none")]
    pub net_ipv4_ip_local_port_range: Option<String>,
    #[doc = "Sysctl setting net.ipv4.neigh.default.gc_thresh1."]
    #[serde(rename = "netIpv4NeighDefaultGcThresh1", default, skip_serializing_if = "Option::is_none")]
    pub net_ipv4_neigh_default_gc_thresh1: Option<i32>,
    #[doc = "Sysctl setting net.ipv4.neigh.default.gc_thresh2."]
    #[serde(rename = "netIpv4NeighDefaultGcThresh2", default, skip_serializing_if = "Option::is_none")]
    pub net_ipv4_neigh_default_gc_thresh2: Option<i32>,
    #[doc = "Sysctl setting net.ipv4.neigh.default.gc_thresh3."]
    #[serde(rename = "netIpv4NeighDefaultGcThresh3", default, skip_serializing_if = "Option::is_none")]
    pub net_ipv4_neigh_default_gc_thresh3: Option<i32>,
    #[doc = "Sysctl setting net.netfilter.nf_conntrack_max."]
    #[serde(rename = "netNetfilterNfConntrackMax", default, skip_serializing_if = "Option::is_none")]
    pub net_netfilter_nf_conntrack_max: Option<i32>,
    #[doc = "Sysctl setting net.netfilter.nf_conntrack_buckets."]
    #[serde(rename = "netNetfilterNfConntrackBuckets", default, skip_serializing_if = "Option::is_none")]
    pub net_netfilter_nf_conntrack_buckets: Option<i32>,
    #[doc = "Sysctl setting fs.inotify.max_user_watches."]
    #[serde(rename = "fsInotifyMaxUserWatches", default, skip_serializing_if = "Option::is_none")]
    pub fs_inotify_max_user_watches: Option<i32>,
    #[doc = "Sysctl setting fs.file-max."]
    #[serde(rename = "fsFileMax", default, skip_serializing_if = "Option::is_none")]
    pub fs_file_max: Option<i32>,
    #[doc = "Sysctl setting fs.aio-max-nr."]
    #[serde(rename = "fsAioMaxNr", default, skip_serializing_if = "Option::is_none")]
    pub fs_aio_max_nr: Option<i32>,
    #[doc = "Sysctl setting fs.nr_open."]
    #[serde(rename = "fsNrOpen", default, skip_serializing_if = "Option::is_none")]
    pub fs_nr_open: Option<i32>,
    #[doc = "Sysctl setting kernel.threads-max."]
    #[serde(rename = "kernelThreadsMax", default, skip_serializing_if = "Option::is_none")]
    pub kernel_threads_max: Option<i32>,
    #[doc = "Sysctl setting vm.max_map_count."]
    #[serde(rename = "vmMaxMapCount", default, skip_serializing_if = "Option::is_none")]
    pub vm_max_map_count: Option<i32>,
    #[doc = "Sysctl setting vm.swappiness."]
    #[serde(rename = "vmSwappiness", default, skip_serializing_if = "Option::is_none")]
    pub vm_swappiness: Option<i32>,
    #[doc = "Sysctl setting vm.vfs_cache_pressure."]
    #[serde(rename = "vmVfsCachePressure", default, skip_serializing_if = "Option::is_none")]
    pub vm_vfs_cache_pressure: Option<i32>,
}
impl SysctlConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tags object for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsObject {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Time in a week."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeInWeek {
    #[doc = "The weekday enum."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<WeekDay>,
    #[doc = "Each integer hour represents a time range beginning at 0m after the hour ending at the next hour (non-inclusive). 0 corresponds to 00:00 UTC, 23 corresponds to 23:00 UTC. Specifying [0, 1] means the 00:00 - 02:00 UTC time range."]
    #[serde(rename = "hourSlots", default, skip_serializing_if = "Vec::is_empty")]
    pub hour_slots: Vec<HourInDay>,
}
impl TimeInWeek {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "For example, between 2021-05-25T13:00:00Z and 2021-05-25T14:00:00Z."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeSpan {
    #[doc = "The start of a time span"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub start: Option<time::OffsetDateTime>,
    #[doc = "The end of a time span"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub end: Option<time::OffsetDateTime>,
}
impl TimeSpan {
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
#[doc = "Trusted access role definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrustedAccessRole {
    #[doc = "Resource type of Azure resource"]
    #[serde(rename = "sourceResourceType", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_type: Option<String>,
    #[doc = "Name of role, name is unique under a source resource type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "List of rules for the role. This maps to 'rules' property of [Kubernetes Cluster Role](https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/cluster-role-v1/#ClusterRole)."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<TrustedAccessRoleRule>,
}
impl TrustedAccessRole {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines binding between a resource and role"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrustedAccessRoleBinding {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties for trusted access role binding"]
    pub properties: TrustedAccessRoleBindingProperties,
}
impl TrustedAccessRoleBinding {
    pub fn new(properties: TrustedAccessRoleBindingProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "List of trusted access role bindings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrustedAccessRoleBindingListResult {
    #[doc = "Role binding list"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TrustedAccessRoleBinding>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TrustedAccessRoleBindingListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TrustedAccessRoleBindingListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for trusted access role binding"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrustedAccessRoleBindingProperties {
    #[doc = "The current provisioning state of trusted access role binding."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<trusted_access_role_binding_properties::ProvisioningState>,
    #[doc = "The ARM resource ID of source resource that trusted access is configured for."]
    #[serde(rename = "sourceResourceId")]
    pub source_resource_id: String,
    #[doc = "A list of roles to bind, each item is a resource type qualified role name. For example: 'Microsoft.MachineLearningServices/workspaces/reader'."]
    pub roles: Vec<String>,
}
impl TrustedAccessRoleBindingProperties {
    pub fn new(source_resource_id: String, roles: Vec<String>) -> Self {
        Self {
            provisioning_state: None,
            source_resource_id,
            roles,
        }
    }
}
pub mod trusted_access_role_binding_properties {
    use super::*;
    #[doc = "The current provisioning state of trusted access role binding."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Updating,
        Deleting,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of trusted access roles"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrustedAccessRoleListResult {
    #[doc = "Role list"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TrustedAccessRole>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TrustedAccessRoleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TrustedAccessRoleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rule for trusted access role"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrustedAccessRoleRule {
    #[doc = "List of allowed verbs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verbs: Vec<String>,
    #[doc = "List of allowed apiGroups"]
    #[serde(rename = "apiGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub api_groups: Vec<String>,
    #[doc = "List of allowed resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[doc = "List of allowed names"]
    #[serde(rename = "resourceNames", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_names: Vec<String>,
    #[doc = "List of allowed nonResourceURLs"]
    #[serde(rename = "nonResourceURLs", default, skip_serializing_if = "Vec::is_empty")]
    pub non_resource_ur_ls: Vec<String>,
}
impl TrustedAccessRoleRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about a user assigned identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The resource ID of the user assigned identity."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The client ID of the user assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The object ID of the user assigned identity."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The weekday enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WeekDay")]
pub enum WeekDay {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WeekDay {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WeekDay {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WeekDay {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Sunday => serializer.serialize_unit_variant("WeekDay", 0u32, "Sunday"),
            Self::Monday => serializer.serialize_unit_variant("WeekDay", 1u32, "Monday"),
            Self::Tuesday => serializer.serialize_unit_variant("WeekDay", 2u32, "Tuesday"),
            Self::Wednesday => serializer.serialize_unit_variant("WeekDay", 3u32, "Wednesday"),
            Self::Thursday => serializer.serialize_unit_variant("WeekDay", 4u32, "Thursday"),
            Self::Friday => serializer.serialize_unit_variant("WeekDay", 5u32, "Friday"),
            Self::Saturday => serializer.serialize_unit_variant("WeekDay", 6u32, "Saturday"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Windows gMSA Profile in the managed cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsGmsaProfile {
    #[doc = "Specifies whether to enable Windows gMSA in the managed cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Specifies the DNS server for Windows gMSA. <br><br> Set it to empty if you have configured the DNS server in the vnet which is used to create the managed cluster."]
    #[serde(rename = "dnsServer", default, skip_serializing_if = "Option::is_none")]
    pub dns_server: Option<String>,
    #[doc = "Specifies the root domain name for Windows gMSA. <br><br> Set it to empty if you have configured the DNS server in the vnet which is used to create the managed cluster."]
    #[serde(rename = "rootDomainName", default, skip_serializing_if = "Option::is_none")]
    pub root_domain_name: Option<String>,
}
impl WindowsGmsaProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Determines the type of workload a node can run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WorkloadRuntime")]
pub enum WorkloadRuntime {
    #[serde(rename = "OCIContainer")]
    OciContainer,
    WasmWasi,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WorkloadRuntime {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WorkloadRuntime {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WorkloadRuntime {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::OciContainer => serializer.serialize_unit_variant("WorkloadRuntime", 0u32, "OCIContainer"),
            Self::WasmWasi => serializer.serialize_unit_variant("WorkloadRuntime", 1u32, "WasmWasi"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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

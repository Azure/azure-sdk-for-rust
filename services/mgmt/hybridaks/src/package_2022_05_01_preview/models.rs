#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "AAD Profile specifies attributes for Azure Active Directory integration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AadProfile {
    #[serde(flatten)]
    pub aad_profile_secret: AadProfileSecret,
    #[serde(flatten)]
    pub aad_profile_response: AadProfileResponse,
}
impl AadProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AAD Profile specifies attributes for Azure Active Directory integration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AadProfileResponse {
    #[doc = "The list of AAD group object IDs that will have admin role of the cluster."]
    #[serde(
        rename = "adminGroupObjectIDs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub admin_group_object_i_ds: Vec<String>,
    #[doc = "The client AAD application ID."]
    #[serde(rename = "clientAppID", default, skip_serializing_if = "Option::is_none")]
    pub client_app_id: Option<String>,
    #[doc = "Whether to enable Azure RBAC for Kubernetes authorization."]
    #[serde(rename = "enableAzureRbac", default, skip_serializing_if = "Option::is_none")]
    pub enable_azure_rbac: Option<bool>,
    #[doc = "Whether to enable managed AAD."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    #[doc = "The server AAD application ID."]
    #[serde(rename = "serverAppID", default, skip_serializing_if = "Option::is_none")]
    pub server_app_id: Option<String>,
    #[doc = "The AAD tenant ID to use for authentication. If not specified, will use the tenant of the deployment subscription."]
    #[serde(rename = "tenantID", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl AadProfileResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The server AAD application secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AadProfileSecret {
    #[doc = "The server AAD application secret."]
    #[serde(rename = "serverAppSecret", default, skip_serializing_if = "Option::is_none")]
    pub server_app_secret: Option<String>,
}
impl AadProfileSecret {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Addon configurations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddonProfiles {
    #[doc = "Config - Key-value pairs for configuring an add-on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
    #[doc = "Enabled - Whether the add-on is enabled or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl AddonProfiles {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status of the addon"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddonStatus {
    #[doc = "ErrorMessage will be set in the event that there is a terminal problem reconciling the AddOn and will contain a more verbose string suitable for logging and human consumption."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Phase represents the current phase of cluster actuation. E.g. Pending, Running, Terminating, Failed etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
}
impl AddonStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AgentPool Name"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolName {
    #[doc = "Unique name of the agent pool profile in the context of the subscription and resource group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl AgentPoolName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NodePool configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolProfile {
    #[doc = "Count - Number of agents to host docker containers. Allowed values must be in the range of 1 to 100 (inclusive). The default value is 1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "AvailabilityZones - The list of Availability zones to use for nodes. Datacenter racks modelled as zones"]
    #[serde(
        rename = "availabilityZones",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub availability_zones: Vec<String>,
    #[doc = "The maximum number of nodes for auto-scaling"]
    #[serde(rename = "maxCount", default, skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i32>,
    #[doc = "The maximum number of pods that can run on a node."]
    #[serde(rename = "maxPods", default, skip_serializing_if = "Option::is_none")]
    pub max_pods: Option<i32>,
    #[doc = "The minimum number of nodes for auto-scaling"]
    #[serde(rename = "minCount", default, skip_serializing_if = "Option::is_none")]
    pub min_count: Option<i32>,
    #[doc = "Mode - AgentPoolMode represents mode of an agent pool. Possible values include: 'System', 'LB', 'User'. Default is 'User'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<agent_pool_profile::Mode>,
    #[doc = "NodeLabels - Agent pool node labels to be persisted across all nodes in agent pool."]
    #[serde(rename = "nodeLabels", default, skip_serializing_if = "Option::is_none")]
    pub node_labels: Option<serde_json::Value>,
    #[doc = "NodeTaints - Taints added to new nodes during node pool create and scale. For example, key=value:NoSchedule."]
    #[serde(
        rename = "nodeTaints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub node_taints: Vec<String>,
    #[doc = "OsType - OsType to be used to specify os type. Choose from Linux and Windows. Default to Linux. Possible values include: 'Linux', 'Windows'"]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<agent_pool_profile::OsType>,
    #[doc = "The version of node image"]
    #[serde(rename = "nodeImageVersion", default, skip_serializing_if = "Option::is_none")]
    pub node_image_version: Option<String>,
    #[doc = "VmSize - The size of the agent pool VMs."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[doc = "CloudProviderProfile - The underlying cloud infra provider properties."]
    #[serde(rename = "cloudProviderProfile", default, skip_serializing_if = "Option::is_none")]
    pub cloud_provider_profile: Option<CloudProviderProfile>,
}
impl AgentPoolProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod agent_pool_profile {
    use super::*;
    #[doc = "Mode - AgentPoolMode represents mode of an agent pool. Possible values include: 'System', 'LB', 'User'. Default is 'User'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        System,
        #[serde(rename = "LB")]
        Lb,
        User,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Mode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Mode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Mode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::System => serializer.serialize_unit_variant("Mode", 0u32, "System"),
                Self::Lb => serializer.serialize_unit_variant("Mode", 1u32, "LB"),
                Self::User => serializer.serialize_unit_variant("Mode", 2u32, "User"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Mode {
        fn default() -> Self {
            Self::User
        }
    }
    #[doc = "OsType - OsType to be used to specify os type. Choose from Linux and Windows. Default to Linux. Possible values include: 'Linux', 'Windows'"]
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
}
#[doc = "Defines the Arc Agent properties for the Provisioned clusters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArcAgentProfile {
    #[doc = "Version of the Arc agents to be installed on the provisioned Provisioned cluster resource"]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "Indicates whether the Arc agents on the provisioned clusters be upgraded automatically to the latest version. Defaults to Enabled."]
    #[serde(rename = "agentAutoUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub agent_auto_upgrade: Option<arc_agent_profile::AgentAutoUpgrade>,
}
impl ArcAgentProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod arc_agent_profile {
    use super::*;
    #[doc = "Indicates whether the Arc agents on the provisioned clusters be upgraded automatically to the latest version. Defaults to Enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AgentAutoUpgrade")]
    pub enum AgentAutoUpgrade {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AgentAutoUpgrade {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AgentAutoUpgrade {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AgentAutoUpgrade {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("AgentAutoUpgrade", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("AgentAutoUpgrade", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for AgentAutoUpgrade {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "Defines the observed Arc Agent status that is resourceSynced back to the ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArcAgentStatus {
    #[doc = "Observed deployment state of the Arc Agents on the target cluster. Possible values include: 'pending', 'provisioning', 'provisioned', 'deleting', 'failed', 'upgrading'"]
    #[serde(rename = "deploymentState", default, skip_serializing_if = "Option::is_none")]
    pub deployment_state: Option<arc_agent_status::DeploymentState>,
    #[doc = "Error messages while onboarding/upgrading/uninstalling the Arc agents"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Onboarding public key for provisioning the Managed identity for the HybridAKS cluster. Will be used to create the hybridIdentityMetadata proxy resource and will not be persisted."]
    #[serde(rename = "onboardingPublicKey", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_public_key: Option<String>,
    #[doc = "Version of the Arc agents currently running on the Provisioned cluster resource."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "Number of CPU cores present in the Provisioned cluster resource"]
    #[serde(rename = "coreCount", default, skip_serializing_if = "Option::is_none")]
    pub core_count: Option<i64>,
    #[doc = "ManagedIdentity certificate expiration time (ValidUntil)."]
    #[serde(
        rename = "managedIdentityCertificateExpirationTime",
        default,
        with = "azure_core::date::rfc3339::option"
    )]
    pub managed_identity_certificate_expiration_time: Option<time::OffsetDateTime>,
    #[doc = "Last connected timestamp of the Provisioned cluster resource."]
    #[serde(rename = "lastConnectivityTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_connectivity_time: Option<time::OffsetDateTime>,
}
impl ArcAgentStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod arc_agent_status {
    use super::*;
    #[doc = "Observed deployment state of the Arc Agents on the target cluster. Possible values include: 'pending', 'provisioning', 'provisioned', 'deleting', 'failed', 'upgrading'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeploymentState")]
    pub enum DeploymentState {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "provisioning")]
        Provisioning,
        #[serde(rename = "provisioned")]
        Provisioned,
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "upgrading")]
        Upgrading,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeploymentState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeploymentState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeploymentState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pending => serializer.serialize_unit_variant("DeploymentState", 0u32, "pending"),
                Self::Provisioning => serializer.serialize_unit_variant("DeploymentState", 1u32, "provisioning"),
                Self::Provisioned => serializer.serialize_unit_variant("DeploymentState", 2u32, "provisioned"),
                Self::Deleting => serializer.serialize_unit_variant("DeploymentState", 3u32, "deleting"),
                Self::Failed => serializer.serialize_unit_variant("DeploymentState", 4u32, "failed"),
                Self::Upgrading => serializer.serialize_unit_variant("DeploymentState", 5u32, "upgrading"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "CloudProviderProfile - The underlying cloud infra provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudProviderProfile {
    #[doc = "InfraNetworkProfile - List of infra network profiles for the provisioned cluster"]
    #[serde(rename = "infraNetworkProfile", default, skip_serializing_if = "Option::is_none")]
    pub infra_network_profile: Option<cloud_provider_profile::InfraNetworkProfile>,
    #[doc = "InfraStorageProfile - List of infra storage profiles for the provisioned cluster"]
    #[serde(rename = "infraStorageProfile", default, skip_serializing_if = "Option::is_none")]
    pub infra_storage_profile: Option<cloud_provider_profile::InfraStorageProfile>,
}
impl CloudProviderProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cloud_provider_profile {
    use super::*;
    #[doc = "InfraNetworkProfile - List of infra network profiles for the provisioned cluster"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct InfraNetworkProfile {
        #[doc = "Array of references to azure resource corresponding to the new HybridAKSNetwork object e.g. /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.HybridContainerService/virtualNetworks/{virtualNetworkName}"]
        #[serde(
            rename = "vnetSubnetIds",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub vnet_subnet_ids: Vec<String>,
    }
    impl InfraNetworkProfile {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "InfraStorageProfile - List of infra storage profiles for the provisioned cluster"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct InfraStorageProfile {
        #[doc = "Reference to azure resource corresponding to the new HybridAKSStorage object e.g. /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.HybridContainerService/storageSpaces/{storageSpaceName}"]
        #[serde(
            rename = "storageSpaceIds",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub storage_space_ids: Vec<String>,
    }
    impl InfraStorageProfile {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "controlPlaneEndpoint - API server endpoint for the control plane"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ControlPlaneEndpointProfile {
    #[doc = "API server endpoint for the control plane"]
    #[serde(rename = "controlPlaneEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub control_plane_endpoint: Option<control_plane_endpoint_profile::ControlPlaneEndpoint>,
}
impl ControlPlaneEndpointProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod control_plane_endpoint_profile {
    use super::*;
    #[doc = "API server endpoint for the control plane"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ControlPlaneEndpoint {
        #[doc = "Host IP address for API server"]
        #[serde(rename = "hostIP", default, skip_serializing_if = "Option::is_none")]
        pub host_ip: Option<String>,
        #[doc = "Port for the API server"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port: Option<String>,
    }
    impl ControlPlaneEndpoint {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "ControlPlaneProfile - The control plane properties for the provisioned cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ControlPlaneProfile {
    #[serde(flatten)]
    pub named_agent_pool_profile: NamedAgentPoolProfile,
    #[serde(flatten)]
    pub control_plane_endpoint_profile: ControlPlaneEndpointProfile,
    #[serde(flatten)]
    pub linux_profile: LinuxProfile,
}
impl ControlPlaneProfile {
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "LinuxProfile - Profile for Linux VMs in the container service cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxProfile {
    #[doc = "LinuxProfile - Profile for Linux VMs in the container service cluster."]
    #[serde(rename = "linuxProfile", default, skip_serializing_if = "Option::is_none")]
    pub linux_profile: Option<LinuxProfileProperties>,
}
impl LinuxProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "LinuxProfile - Profile for Linux VMs in the container service cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxProfileProperties {
    #[doc = "AdminUsername - The administrator username to use for Linux VMs."]
    #[serde(rename = "adminUsername", default, skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<String>,
    #[doc = "SSH - SSH configuration for Linux-based VMs running on Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh: Option<linux_profile_properties::Ssh>,
}
impl LinuxProfileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod linux_profile_properties {
    use super::*;
    #[doc = "SSH - SSH configuration for Linux-based VMs running on Azure."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Ssh {
        #[doc = "PublicKeys - The list of SSH public keys used to authenticate with Linux-based VMs. Only expect one key specified."]
        #[serde(
            rename = "publicKeys",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub public_keys: Vec<serde_json::Value>,
    }
    impl Ssh {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "LoadBalancerProfile - Profile of the cluster load balancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancerProfile {
    #[serde(flatten)]
    pub named_agent_pool_profile: NamedAgentPoolProfile,
    #[serde(flatten)]
    pub linux_profile: LinuxProfile,
}
impl LoadBalancerProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Agent pool profile along with a name parameter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamedAgentPoolProfile {
    #[serde(flatten)]
    pub agent_pool_profile: AgentPoolProfile,
    #[serde(flatten)]
    pub agent_pool_name: AgentPoolName,
}
impl NamedAgentPoolProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NetworkProfile - Profile of network configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfile {
    #[doc = "LoadBalancerProfile - Profile of the cluster load balancer."]
    #[serde(rename = "loadBalancerProfile", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_profile: Option<LoadBalancerProfile>,
    #[doc = "LoadBalancerSku - The load balancer sku for the provisioned cluster. Possible values: 'unstacked-haproxy', 'stacked-kube-vip', 'stacked-metallb', 'unmanaged'. The default is 'unmanaged'."]
    #[serde(rename = "loadBalancerSku", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_sku: Option<network_profile::LoadBalancerSku>,
    #[doc = "DNSServiceIP - An IP address assigned to the Kubernetes DNS service. It must be within the Kubernetes service address range specified in serviceCidr."]
    #[serde(rename = "dnsServiceIP", default, skip_serializing_if = "Option::is_none")]
    pub dns_service_ip: Option<String>,
    #[doc = "NetworkPolicy - Network policy used for building Kubernetes network. Possible values include: 'calico', 'flannel'. Default is 'calico'"]
    #[serde(rename = "networkPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_policy: Option<network_profile::NetworkPolicy>,
    #[doc = "PodCidr - A CIDR notation IP range from which to assign pod IPs when kubenet is used."]
    #[serde(rename = "podCidr", default, skip_serializing_if = "Option::is_none")]
    pub pod_cidr: Option<String>,
    #[doc = "The CIDR notation IP ranges from which to assign pod IPs. One IPv4 CIDR is expected for single-stack networking. Two CIDRs, one for each IP family (IPv4/IPv6), is expected for dual-stack networking."]
    #[serde(
        rename = "podCidrs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pod_cidrs: Vec<String>,
    #[doc = "ServiceCidr - A CIDR notation IP range from which to assign service cluster IPs. It must not overlap with any Subnet IP ranges."]
    #[serde(rename = "serviceCidr", default, skip_serializing_if = "Option::is_none")]
    pub service_cidr: Option<String>,
    #[doc = "The CIDR notation IP ranges from which to assign service cluster IPs. One IPv4 CIDR is expected for single-stack networking. Two CIDRs, one for each IP family (IPv4/IPv6), is expected for dual-stack networking. They must not overlap with any Subnet IP ranges."]
    #[serde(
        rename = "serviceCidrs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub service_cidrs: Vec<String>,
}
impl NetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_profile {
    use super::*;
    #[doc = "LoadBalancerSku - The load balancer sku for the provisioned cluster. Possible values: 'unstacked-haproxy', 'stacked-kube-vip', 'stacked-metallb', 'unmanaged'. The default is 'unmanaged'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LoadBalancerSku")]
    pub enum LoadBalancerSku {
        #[serde(rename = "unstacked-haproxy")]
        UnstackedHaproxy,
        #[serde(rename = "stacked-kube-vip")]
        StackedKubeVip,
        #[serde(rename = "stacked-metallb")]
        StackedMetallb,
        #[serde(rename = "unmanaged")]
        Unmanaged,
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
                Self::UnstackedHaproxy => serializer.serialize_unit_variant("LoadBalancerSku", 0u32, "unstacked-haproxy"),
                Self::StackedKubeVip => serializer.serialize_unit_variant("LoadBalancerSku", 1u32, "stacked-kube-vip"),
                Self::StackedMetallb => serializer.serialize_unit_variant("LoadBalancerSku", 2u32, "stacked-metallb"),
                Self::Unmanaged => serializer.serialize_unit_variant("LoadBalancerSku", 3u32, "unmanaged"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for LoadBalancerSku {
        fn default() -> Self {
            Self::Unmanaged
        }
    }
    #[doc = "NetworkPolicy - Network policy used for building Kubernetes network. Possible values include: 'calico', 'flannel'. Default is 'calico'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NetworkPolicy")]
    pub enum NetworkPolicy {
        #[serde(rename = "calico")]
        Calico,
        #[serde(rename = "flannel")]
        Flannel,
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
                Self::Flannel => serializer.serialize_unit_variant("NetworkPolicy", 1u32, "flannel"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for NetworkPolicy {
        fn default() -> Self {
            Self::Calico
        }
    }
}
#[doc = "Contains information about orchestrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrchestratorProfile {
    #[doc = "Whether Kubernetes version is currently in preview."]
    #[serde(rename = "isPreview", default, skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<bool>,
    #[doc = "Orchestrator type."]
    #[serde(rename = "orchestratorType", default, skip_serializing_if = "Option::is_none")]
    pub orchestrator_type: Option<String>,
    #[doc = "Orchestrator version (major, minor, patch)."]
    #[serde(rename = "orchestratorVersion", default, skip_serializing_if = "Option::is_none")]
    pub orchestrator_version: Option<String>,
}
impl OrchestratorProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The profile of an orchestrator and its available versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrchestratorVersionProfile {
    #[doc = "Whether Kubernetes version is currently in preview."]
    #[serde(rename = "isPreview", default, skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<bool>,
    #[doc = "Installed by default if version is not specified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[doc = "Orchestrator type."]
    #[serde(rename = "orchestratorType", default, skip_serializing_if = "Option::is_none")]
    pub orchestrator_type: Option<String>,
    #[doc = "Orchestrator version major.minor.patch, for example 1.21.9"]
    #[serde(rename = "orchestratorVersion", default, skip_serializing_if = "Option::is_none")]
    pub orchestrator_version: Option<String>,
    #[doc = "The list of available upgrade versions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub upgrades: Vec<OrchestratorProfile>,
}
impl OrchestratorVersionProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of versions for supported orchestrators."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrchestratorVersionProfileListResult {
    #[doc = "Profile of the orchestrator versions"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub orchestrators: Vec<OrchestratorVersionProfile>,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl OrchestratorVersionProfileListResult {
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
#[doc = "Results of the request to list operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperation {
    #[doc = "Indicates whether the operation applies to data-plane."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<String>,
    #[doc = "Operation name, in format of {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<resource_provider_operation::Display>,
}
impl ResourceProviderOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_provider_operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The resource provider."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of this operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Results of the request to list operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ResourceProviderOperation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceProviderOperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ResourceProviderOperationList {
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
#[doc = "The list of supported VM SKUs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmSkuListResult {
    #[doc = "Supported VM SKUs."]
    #[serde(
        rename = "vmSKUs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vm_sk_us: Vec<String>,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl VmSkuListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "WindowsProfile - Profile for Windows VMs in the container service cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsProfile {
    #[serde(flatten)]
    pub windows_profile_response: WindowsProfileResponse,
    #[serde(flatten)]
    pub windows_profile_password: WindowsProfilePassword,
}
impl WindowsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Password for the Windows Profile to be used by the Windows VMs in the cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsProfilePassword {
    #[doc = "AdminPassword - Specifies the password of the administrator account. <br><br> **Minimum-length:** 8 characters <br><br> **Max-length:** 123 characters <br><br> **Complexity requirements:** 3 out of 4 conditions below need to be fulfilled <br> Has lower characters <br>Has upper characters <br> Has a digit <br> Has a special character (Regex match [\\W_]) <br><br> **Disallowed values:** \"abc@123\", \"P@$$w0rd\", \"P@ssw0rd\", \"P@ssword123\", \"Pa$$word\", \"pass@word1\", \"Password!\", \"Password1\", \"Password22\", \"iloveyou!\""]
    #[serde(rename = "adminPassword", default, skip_serializing_if = "Option::is_none")]
    pub admin_password: Option<String>,
}
impl WindowsProfilePassword {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Profile for Windows VMs in the container service cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsProfileResponse {
    #[doc = "AdminUsername - Specifies the name of the administrator account. <br><br> **restriction:** Cannot end in \".\" <br><br> **Disallowed values:** \"administrator\", \"admin\", \"user\", \"user1\", \"test\", \"user2\", \"test1\", \"user3\", \"admin1\", \"1\", \"123\", \"a\", \"actuser\", \"adm\", \"admin2\", \"aspnet\", \"backup\", \"console\", \"david\", \"guest\", \"john\", \"owner\", \"root\", \"server\", \"sql\", \"support\", \"support_388945a0\", \"sys\", \"test2\", \"test3\", \"user4\", \"user5\". <br><br> **Minimum-length:** 1 character <br><br> **Max-length:** 20 characters"]
    #[serde(rename = "adminUsername", default, skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<String>,
    #[doc = "EnableCSIProxy - Whether to enable CSI proxy."]
    #[serde(rename = "enableCsiProxy", default, skip_serializing_if = "Option::is_none")]
    pub enable_csi_proxy: Option<bool>,
    #[doc = "LicenseType - The licenseType to use for Windows VMs. Windows_Server is used to enable Azure Hybrid User Benefits for Windows VMs. Possible values include: 'None', 'Windows_Server'"]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<windows_profile_response::LicenseType>,
}
impl WindowsProfileResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod windows_profile_response {
    use super::*;
    #[doc = "LicenseType - The licenseType to use for Windows VMs. Windows_Server is used to enable Azure Hybrid User Benefits for Windows VMs. Possible values include: 'None', 'Windows_Server'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        #[serde(rename = "Windows_Server")]
        WindowsServer,
        None,
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
                Self::WindowsServer => serializer.serialize_unit_variant("LicenseType", 0u32, "Windows_Server"),
                Self::None => serializer.serialize_unit_variant("LicenseType", 1u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The agentPool resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPool {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AgentPoolProperties>,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<agent_pool::ExtendedLocation>,
}
impl AgentPool {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod agent_pool {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ExtendedLocation {
        #[doc = "The extended location type."]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[doc = "The extended location name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    impl ExtendedLocation {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolListResult {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AgentPool>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AgentPoolListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolProperties {
    #[serde(flatten)]
    pub agent_pool_profile: AgentPoolProfile,
    #[serde(flatten)]
    pub agent_pool_provisioning_status: AgentPoolProvisioningStatus,
}
impl AgentPoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The agentPool resource provisioning status definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolProvisioningStatus {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<agent_pool_provisioning_status::ProvisioningState>,
    #[doc = "HybridAKSNodePoolStatus defines the observed state of HybridAKSNodePool"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<agent_pool_provisioning_status::Status>,
}
impl AgentPoolProvisioningStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod agent_pool_provisioning_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Deleting,
        InProgress,
        Canceled,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 3u32, "InProgress"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "HybridAKSNodePoolStatus defines the observed state of HybridAKSNodePool"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Status {
        #[doc = "ErrorMessage - Error messages during creation of cluster"]
        #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
        pub error_message: Option<String>,
        #[doc = "Contains Provisioning errors"]
        #[serde(rename = "provisioningStatus", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_status: Option<status::ProvisioningStatus>,
        #[doc = "Total number of ready machines targeted by this deployment."]
        #[serde(rename = "readyReplicas", default, skip_serializing_if = "Option::is_none")]
        pub ready_replicas: Option<i32>,
        #[doc = "Total number of non-terminated machines targeted by this deployment"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub replicas: Option<i32>,
    }
    impl Status {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod status {
        use super::*;
        #[doc = "Contains Provisioning errors"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct ProvisioningStatus {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub error: Option<provisioning_status::Error>,
            #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
            pub operation_id: Option<String>,
            #[doc = "Phase represents the current phase of cluster actuation. E.g. Pending, Running, Terminating, Failed etc."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub phase: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub status: Option<String>,
        }
        impl ProvisioningStatus {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod provisioning_status {
            use super::*;
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
            pub struct Error {
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub code: Option<String>,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub message: Option<String>,
            }
            impl Error {
                pub fn new() -> Self {
                    Self::default()
                }
            }
        }
    }
}
#[doc = "Configurations for provisioning the cluster with HTTP proxy servers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpProxyConfig {
    #[serde(flatten)]
    pub http_proxy_config_response: HttpProxyConfigResponse,
    #[serde(flatten)]
    pub http_proxy_config_password: HttpProxyConfigPassword,
}
impl HttpProxyConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpProxyConfigPassword {
    #[doc = "Password to use for connecting to proxy server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl HttpProxyConfigPassword {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configurations for provisioning the cluster with HTTP proxy servers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpProxyConfigResponse {
    #[doc = "The HTTP proxy server endpoint to use."]
    #[serde(rename = "httpProxy", default, skip_serializing_if = "Option::is_none")]
    pub http_proxy: Option<String>,
    #[doc = "The HTTPS proxy server endpoint to use."]
    #[serde(rename = "httpsProxy", default, skip_serializing_if = "Option::is_none")]
    pub https_proxy: Option<String>,
    #[doc = "The endpoints that should not go through proxy."]
    #[serde(
        rename = "noProxy",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub no_proxy: Vec<String>,
    #[doc = "Alternative CA cert to use for connecting to proxy servers."]
    #[serde(rename = "trustedCa", default, skip_serializing_if = "Option::is_none")]
    pub trusted_ca: Option<String>,
    #[doc = "Username to use for connecting to proxy server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
impl HttpProxyConfigResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the hybridIdentityMetadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridIdentityMetadata {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Defines the resource properties."]
    pub properties: HybridIdentityMetadataProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl HybridIdentityMetadata {
    pub fn new(properties: HybridIdentityMetadataProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "List of hybridIdentityMetadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridIdentityMetadataList {
    #[doc = "Url to follow for getting next page of hybridIdentityMetadata."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of hybridIdentityMetadata"]
    pub value: Vec<HybridIdentityMetadata>,
}
impl azure_core::Continuable for HybridIdentityMetadataList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HybridIdentityMetadataList {
    pub fn new(value: Vec<HybridIdentityMetadata>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridIdentityMetadataProperties {
    #[doc = "Unique id of the parent provisioned cluster resource."]
    #[serde(rename = "resourceUid", default, skip_serializing_if = "Option::is_none")]
    pub resource_uid: Option<String>,
    #[doc = "Onboarding public key for provisioning the Managed identity for the HybridAKS cluster."]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[doc = "Identity for the Provisioned cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ProvisionedClusterIdentity>,
    #[doc = "provisioning state of the hybridIdentityMetadata resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl HybridIdentityMetadataProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the Provisioned cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvisionedClusterIdentity {
    #[doc = "The principal id of provisioned cluster identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id associated with the provisioned cluster. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of identity used for the provisioned cluster. The type SystemAssigned, includes a system created identity. The type None means no identity is assigned to the provisioned cluster."]
    #[serde(rename = "type")]
    pub type_: provisioned_cluster_identity::Type,
}
impl ProvisionedClusterIdentity {
    pub fn new(type_: provisioned_cluster_identity::Type) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
        }
    }
}
pub mod provisioned_cluster_identity {
    use super::*;
    #[doc = "The type of identity used for the provisioned cluster. The type SystemAssigned, includes a system created identity. The type None means no identity is assigned to the provisioned cluster."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        None,
        SystemAssigned,
    }
}
#[doc = "The provisionedClusters resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvisionedClusters {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Identity for the Provisioned cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ProvisionedClusterIdentity>,
    #[doc = "All properties of the provisioned cluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProvisionedClustersAllProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<provisioned_clusters::ExtendedLocation>,
}
impl ProvisionedClusters {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            properties: None,
            system_data: None,
            extended_location: None,
        }
    }
}
pub mod provisioned_clusters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ExtendedLocation {
        #[doc = "The extended location type."]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[doc = "The extended location name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    impl ExtendedLocation {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "All properties of the provisioned cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedClustersAllProperties {
    #[serde(flatten)]
    pub provisioned_clusters_properties_with_secrets: ProvisionedClustersPropertiesWithSecrets,
    #[serde(flatten)]
    pub provisioned_clusters_common_properties: ProvisionedClustersCommonProperties,
}
impl ProvisionedClustersAllProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "HybridAKSClusterSpec defines the desired state of HybridAKSCluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedClustersCommonProperties {
    #[doc = "EnableRBAC - Whether to enable Kubernetes Role-Based Access Control."]
    #[serde(rename = "enableRbac", default, skip_serializing_if = "Option::is_none")]
    pub enable_rbac: Option<bool>,
    #[doc = "LinuxProfile - Profile for Linux VMs in the container service cluster."]
    #[serde(rename = "linuxProfile", default, skip_serializing_if = "Option::is_none")]
    pub linux_profile: Option<LinuxProfileProperties>,
    #[doc = "Additional features specs like Arc Agent Onboarding."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<provisioned_clusters_common_properties::Features>,
    #[doc = "AddonProfiles - Profile of managed cluster add-on."]
    #[serde(rename = "addonProfiles", default, skip_serializing_if = "Option::is_none")]
    pub addon_profiles: Option<serde_json::Value>,
    #[doc = "ControlPlaneProfile - The control plane properties for the provisioned cluster."]
    #[serde(rename = "controlPlane", default, skip_serializing_if = "Option::is_none")]
    pub control_plane: Option<ControlPlaneProfile>,
    #[doc = "KubernetesVersion - Version of Kubernetes specified when creating the managed cluster."]
    #[serde(rename = "kubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    #[doc = "NetworkProfile - Profile of network configuration."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    #[doc = "NodeResourceGroup - Name of the resource group containing agent pool nodes."]
    #[serde(rename = "nodeResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub node_resource_group: Option<String>,
    #[doc = "The agent pools of the cluster."]
    #[serde(
        rename = "agentPoolProfiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub agent_pool_profiles: Vec<NamedAgentPoolProfile>,
    #[doc = "CloudProviderProfile - The underlying cloud infra provider properties."]
    #[serde(rename = "cloudProviderProfile", default, skip_serializing_if = "Option::is_none")]
    pub cloud_provider_profile: Option<CloudProviderProfile>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<provisioned_clusters_common_properties::ProvisioningState>,
    #[doc = "HybridAKSClusterStatus defines the observed state of HybridAKSCluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<provisioned_clusters_common_properties::Status>,
}
impl ProvisionedClustersCommonProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod provisioned_clusters_common_properties {
    use super::*;
    #[doc = "Additional features specs like Arc Agent Onboarding."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Features {
        #[doc = "Defines the Arc Agent properties for the Provisioned clusters."]
        #[serde(rename = "arcAgentProfile", default, skip_serializing_if = "Option::is_none")]
        pub arc_agent_profile: Option<ArcAgentProfile>,
    }
    impl Features {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        InProgress,
        Deleting,
        Updating,
        Accepted,
        Created,
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
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 3u32, "InProgress"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Updating"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Accepted"),
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Created"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "HybridAKSClusterStatus defines the observed state of HybridAKSCluster"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Status {
        #[doc = "Additional features status like Arc Agent Onboarding."]
        #[serde(rename = "featuresStatus", default, skip_serializing_if = "Option::is_none")]
        pub features_status: Option<status::FeaturesStatus>,
        #[doc = "AddonStatus - Status of Addons"]
        #[serde(rename = "addonStatus", default, skip_serializing_if = "Option::is_none")]
        pub addon_status: Option<serde_json::Value>,
        #[doc = "ErrorMessage - Error messages during creation of cluster"]
        #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
        pub error_message: Option<String>,
        #[doc = "Contains Provisioning errors"]
        #[serde(rename = "provisioningStatus", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_status: Option<status::ProvisioningStatus>,
    }
    impl Status {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod status {
        use super::*;
        #[doc = "Additional features status like Arc Agent Onboarding."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct FeaturesStatus {
            #[doc = "Defines the observed Arc Agent status that is resourceSynced back to the ARM resource."]
            #[serde(rename = "arcAgentStatus", default, skip_serializing_if = "Option::is_none")]
            pub arc_agent_status: Option<ArcAgentStatus>,
        }
        impl FeaturesStatus {
            pub fn new() -> Self {
                Self::default()
            }
        }
        #[doc = "Contains Provisioning errors"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct ProvisioningStatus {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub error: Option<provisioning_status::Error>,
            #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
            pub operation_id: Option<String>,
            #[doc = "Phase represents the current phase of cluster actuation. E.g. Pending, Running, Terminating, Failed etc."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub phase: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub status: Option<String>,
        }
        impl ProvisioningStatus {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod provisioning_status {
            use super::*;
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
            pub struct Error {
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub code: Option<String>,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub message: Option<String>,
            }
            impl Error {
                pub fn new() -> Self {
                    Self::default()
                }
            }
        }
    }
}
#[doc = "The provisionedClusters resource patch definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedClustersPatch {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ProvisionedClustersPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of provisioned clusters that contain secrets"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedClustersPropertiesWithSecrets {
    #[doc = "AAD Profile specifies attributes for Azure Active Directory integration."]
    #[serde(rename = "aadProfile", default, skip_serializing_if = "Option::is_none")]
    pub aad_profile: Option<AadProfile>,
    #[doc = "WindowsProfile - Profile for Windows VMs in the container service cluster."]
    #[serde(rename = "windowsProfile", default, skip_serializing_if = "Option::is_none")]
    pub windows_profile: Option<WindowsProfile>,
    #[doc = "Configurations for provisioning the cluster with HTTP proxy servers."]
    #[serde(rename = "httpProxyConfig", default, skip_serializing_if = "Option::is_none")]
    pub http_proxy_config: Option<HttpProxyConfig>,
}
impl ProvisionedClustersPropertiesWithSecrets {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of provisioned clusters without the corresponding secrets"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedClustersPropertiesWithoutSecrets {
    #[doc = "AAD Profile specifies attributes for Azure Active Directory integration."]
    #[serde(rename = "aadProfile", default, skip_serializing_if = "Option::is_none")]
    pub aad_profile: Option<AadProfileResponse>,
    #[doc = "Profile for Windows VMs in the container service cluster."]
    #[serde(rename = "windowsProfile", default, skip_serializing_if = "Option::is_none")]
    pub windows_profile: Option<WindowsProfileResponse>,
    #[doc = "Configurations for provisioning the cluster with HTTP proxy servers."]
    #[serde(rename = "httpProxyConfig", default, skip_serializing_if = "Option::is_none")]
    pub http_proxy_config: Option<HttpProxyConfigResponse>,
}
impl ProvisionedClustersPropertiesWithoutSecrets {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The provisionedClusters resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvisionedClustersResponse {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Identity for the Provisioned cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ProvisionedClusterIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProvisionedClustersResponseProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<provisioned_clusters_response::ExtendedLocation>,
}
impl ProvisionedClustersResponse {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            properties: None,
            system_data: None,
            extended_location: None,
        }
    }
}
pub mod provisioned_clusters_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ExtendedLocation {
        #[doc = "The extended location type."]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[doc = "The extended location name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    impl ExtendedLocation {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedClustersResponseListResult {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProvisionedClustersResponse>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProvisionedClustersResponseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ProvisionedClustersResponseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedClustersResponseProperties {
    #[serde(flatten)]
    pub provisioned_clusters_properties_without_secrets: ProvisionedClustersPropertiesWithoutSecrets,
    #[serde(flatten)]
    pub provisioned_clusters_common_properties: ProvisionedClustersCommonProperties,
}
impl ProvisionedClustersResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The storageSpaces resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSpaces {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "HybridAKSStorageSpec defines the desired state of HybridAKSStorage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageSpacesProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<storage_spaces::ExtendedLocation>,
}
impl StorageSpaces {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            system_data: None,
            extended_location: None,
        }
    }
}
pub mod storage_spaces {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ExtendedLocation {
        #[doc = "The extended location type."]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[doc = "The extended location name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    impl ExtendedLocation {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSpacesListResult {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<StorageSpaces>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StorageSpacesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl StorageSpacesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The storageSpaces resource patch definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSpacesPatch {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl StorageSpacesPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "HybridAKSStorageSpec defines the desired state of HybridAKSStorage"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSpacesProperties {
    #[serde(rename = "hciStorageProfile", default, skip_serializing_if = "Option::is_none")]
    pub hci_storage_profile: Option<storage_spaces_properties::HciStorageProfile>,
    #[serde(rename = "vmwareStorageProfile", default, skip_serializing_if = "Option::is_none")]
    pub vmware_storage_profile: Option<storage_spaces_properties::VmwareStorageProfile>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<storage_spaces_properties::ProvisioningState>,
    #[doc = "HybridAKSStorageStatus defines the observed state of HybridAKSStorage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<storage_spaces_properties::Status>,
}
impl StorageSpacesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_spaces_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct HciStorageProfile {
        #[doc = "Resource group in MOC(Microsoft On-premises Cloud)"]
        #[serde(rename = "mocGroup", default, skip_serializing_if = "Option::is_none")]
        pub moc_group: Option<String>,
        #[doc = "Location in MOC(Microsoft On-premises Cloud)"]
        #[serde(rename = "mocLocation", default, skip_serializing_if = "Option::is_none")]
        pub moc_location: Option<String>,
        #[doc = "Name of the storage container in MOC(Microsoft On-premises Cloud)"]
        #[serde(rename = "mocStorageContainer", default, skip_serializing_if = "Option::is_none")]
        pub moc_storage_container: Option<String>,
    }
    impl HciStorageProfile {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct VmwareStorageProfile {
        #[doc = "Name of the datacenter in VSphere"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub datacenter: Option<String>,
        #[doc = "Name of the datastore in VSphere"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub datastore: Option<String>,
        #[doc = "Name of the folder in VSphere"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub folder: Option<String>,
        #[doc = "Name of the resource pool in VSphere"]
        #[serde(rename = "resourcePool", default, skip_serializing_if = "Option::is_none")]
        pub resource_pool: Option<String>,
    }
    impl VmwareStorageProfile {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        InProgress,
        Deleting,
        Updating,
        Accepted,
        Created,
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
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 3u32, "InProgress"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Updating"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Accepted"),
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Created"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "HybridAKSStorageStatus defines the observed state of HybridAKSStorage"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Status {
        #[doc = "Contains Provisioning errors"]
        #[serde(rename = "provisioningStatus", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_status: Option<status::ProvisioningStatus>,
    }
    impl Status {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod status {
        use super::*;
        #[doc = "Contains Provisioning errors"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct ProvisioningStatus {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub error: Option<provisioning_status::Error>,
            #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
            pub operation_id: Option<String>,
            #[doc = "Phase represents the current phase of cluster actuation. E.g. Pending, Running, Terminating, Failed etc."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub phase: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub status: Option<String>,
        }
        impl ProvisioningStatus {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod provisioning_status {
            use super::*;
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
            pub struct Error {
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub code: Option<String>,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub message: Option<String>,
            }
            impl Error {
                pub fn new() -> Self {
                    Self::default()
                }
            }
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
#[doc = "The virtualNetworks resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworks {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "HybridAKSNetworkSpec defines the desired state of HybridAKSNetwork"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworksProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<virtual_networks::ExtendedLocation>,
}
impl VirtualNetworks {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            system_data: None,
            extended_location: None,
        }
    }
}
pub mod virtual_networks {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ExtendedLocation {
        #[doc = "The extended location type."]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[doc = "The extended location name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    impl ExtendedLocation {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworksListResult {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VirtualNetworks>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworksListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VirtualNetworksListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The virtualNetworks resource patch definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworksPatch {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl VirtualNetworksPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "HybridAKSNetworkSpec defines the desired state of HybridAKSNetwork"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworksProperties {
    #[serde(rename = "infraVnetProfile", default, skip_serializing_if = "Option::is_none")]
    pub infra_vnet_profile: Option<virtual_networks_properties::InfraVnetProfile>,
    #[doc = "Virtual IP Pool for Kubernetes"]
    #[serde(
        rename = "vipPool",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vip_pool: Vec<serde_json::Value>,
    #[doc = "IP Pool for Virtual Machines"]
    #[serde(
        rename = "vmipPool",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vmip_pool: Vec<serde_json::Value>,
    #[doc = "Address of the DHCP servers associated with the network"]
    #[serde(
        rename = "dhcpServers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dhcp_servers: Vec<String>,
    #[doc = "Address of the DNS servers associated with the network"]
    #[serde(
        rename = "dnsServers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dns_servers: Vec<String>,
    #[doc = "Address of the Gateway associated with the network"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[doc = "IP Address Prefix of the network"]
    #[serde(rename = "ipAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub ip_address_prefix: Option<String>,
    #[doc = "VLAN Id used by the network"]
    #[serde(rename = "vlanID", default, skip_serializing_if = "Option::is_none")]
    pub vlan_id: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<virtual_networks_properties::ProvisioningState>,
    #[doc = "HybridAKSNetworkStatus defines the observed state of HybridAKSNetwork"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<virtual_networks_properties::Status>,
}
impl VirtualNetworksProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_networks_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct InfraVnetProfile {
        #[doc = "Infra network profile for HCI platform"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hci: Option<infra_vnet_profile::Hci>,
        #[doc = "Infra network profile for KubeVirt platform"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kubevirt: Option<infra_vnet_profile::Kubevirt>,
        #[doc = "Infra network profile for VMware platform"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub vmware: Option<infra_vnet_profile::Vmware>,
    }
    impl InfraVnetProfile {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod infra_vnet_profile {
        use super::*;
        #[doc = "Infra network profile for HCI platform"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Hci {
            #[doc = "Resource group in MOC(Microsoft On-premises Cloud)"]
            #[serde(rename = "mocGroup", default, skip_serializing_if = "Option::is_none")]
            pub moc_group: Option<String>,
            #[doc = "Location in MOC(Microsoft On-premises Cloud)"]
            #[serde(rename = "mocLocation", default, skip_serializing_if = "Option::is_none")]
            pub moc_location: Option<String>,
            #[doc = "Virtual Network name in MOC(Microsoft On-premises Cloud)"]
            #[serde(rename = "mocVnetName", default, skip_serializing_if = "Option::is_none")]
            pub moc_vnet_name: Option<String>,
        }
        impl Hci {
            pub fn new() -> Self {
                Self::default()
            }
        }
        #[doc = "Infra network profile for KubeVirt platform"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Kubevirt {
            #[doc = "Name of the network in KubeVirt"]
            #[serde(rename = "vnetName", default, skip_serializing_if = "Option::is_none")]
            pub vnet_name: Option<String>,
        }
        impl Kubevirt {
            pub fn new() -> Self {
                Self::default()
            }
        }
        #[doc = "Infra network profile for VMware platform"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Vmware {
            #[doc = "Name of the network segment in VSphere"]
            #[serde(rename = "segmentName", default, skip_serializing_if = "Option::is_none")]
            pub segment_name: Option<String>,
        }
        impl Vmware {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        InProgress,
        Deleting,
        Updating,
        Accepted,
        Created,
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
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 3u32, "InProgress"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Updating"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Accepted"),
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Created"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "HybridAKSNetworkStatus defines the observed state of HybridAKSNetwork"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Status {
        #[doc = "Contains Provisioning errors"]
        #[serde(rename = "provisioningStatus", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_status: Option<status::ProvisioningStatus>,
    }
    impl Status {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod status {
        use super::*;
        #[doc = "Contains Provisioning errors"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct ProvisioningStatus {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub error: Option<provisioning_status::Error>,
            #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
            pub operation_id: Option<String>,
            #[doc = "Phase represents the current phase of cluster actuation. E.g. Pending, Running, Terminating, Failed etc."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub phase: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub status: Option<String>,
        }
        impl ProvisioningStatus {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod provisioning_status {
            use super::*;
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
            pub struct Error {
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub code: Option<String>,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub message: Option<String>,
            }
            impl Error {
                pub fn new() -> Self {
                    Self::default()
                }
            }
        }
    }
}

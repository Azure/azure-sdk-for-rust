#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The status profile of the addons and other kubernetes components"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddonStatusProfile {
    #[doc = "Name of the addon or component"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Observed phase of the addon or component on the provisioned cluster. Possible values include: 'pending', 'provisioning', 'provisioning {HelmChartInstalled}', 'provisioning {MSICertificateDownloaded}', 'provisioned', 'deleting', 'failed', 'upgrading'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<addon_status_profile::Phase>,
    #[doc = "Indicates whether the addon or component is ready"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    #[doc = "Observed error message from the addon or component"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl AddonStatusProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod addon_status_profile {
    use super::*;
    #[doc = "Observed phase of the addon or component on the provisioned cluster. Possible values include: 'pending', 'provisioning', 'provisioning {HelmChartInstalled}', 'provisioning {MSICertificateDownloaded}', 'provisioned', 'deleting', 'failed', 'upgrading'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Phase")]
    pub enum Phase {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "provisioning")]
        Provisioning,
        #[serde(rename = "provisioning {HelmChartInstalled}")]
        ProvisioningHelmChartInstalled,
        #[serde(rename = "provisioning {MSICertificateDownloaded}")]
        ProvisioningMsiCertificateDownloaded,
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
    impl FromStr for Phase {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Phase {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Phase {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pending => serializer.serialize_unit_variant("Phase", 0u32, "pending"),
                Self::Provisioning => serializer.serialize_unit_variant("Phase", 1u32, "provisioning"),
                Self::ProvisioningHelmChartInstalled => {
                    serializer.serialize_unit_variant("Phase", 2u32, "provisioning {HelmChartInstalled}")
                }
                Self::ProvisioningMsiCertificateDownloaded => {
                    serializer.serialize_unit_variant("Phase", 3u32, "provisioning {MSICertificateDownloaded}")
                }
                Self::Provisioned => serializer.serialize_unit_variant("Phase", 4u32, "provisioned"),
                Self::Deleting => serializer.serialize_unit_variant("Phase", 5u32, "deleting"),
                Self::Failed => serializer.serialize_unit_variant("Phase", 6u32, "failed"),
                Self::Upgrading => serializer.serialize_unit_variant("Phase", 7u32, "upgrading"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Name of the default Agent Pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolName {
    #[doc = "Unique name of the default agent pool in the context of the provisioned cluster. Default value is <clusterName>-nodepool1"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl AgentPoolName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Profile for agent pool properties specified during creation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolProfile {
    #[doc = "OSType to be used to specify OS type for the VMs. Choose from Linux and Windows. Default to Linux. Possible values include: 'Linux', 'Windows'"]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "Specifies the OS SKU used by the agent pool. The default is CBLMariner if OSType is Linux. The default is Windows2019 when OSType is Windows."]
    #[serde(rename = "osSKU", default, skip_serializing_if = "Option::is_none")]
    pub os_sku: Option<Ossku>,
    #[doc = "The node labels to be persisted across all nodes in agent pool."]
    #[serde(rename = "nodeLabels", default, skip_serializing_if = "Option::is_none")]
    pub node_labels: Option<serde_json::Value>,
    #[doc = "Taints added to new nodes during node pool create and scale. For example, key=value:NoSchedule."]
    #[serde(
        rename = "nodeTaints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub node_taints: Vec<String>,
    #[doc = "The maximum number of nodes for auto-scaling"]
    #[serde(rename = "maxCount", default, skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i32>,
    #[doc = "The minimum number of nodes for auto-scaling"]
    #[serde(rename = "minCount", default, skip_serializing_if = "Option::is_none")]
    pub min_count: Option<i32>,
    #[doc = "Whether to enable auto-scaler. Default value is false"]
    #[serde(rename = "enableAutoScaling", default, skip_serializing_if = "Option::is_none")]
    pub enable_auto_scaling: Option<bool>,
    #[doc = "The maximum number of pods that can run on a node."]
    #[serde(rename = "maxPods", default, skip_serializing_if = "Option::is_none")]
    pub max_pods: Option<i32>,
}
impl AgentPoolProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Profile for agent pool properties that can be updated"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolUpdateProfile {
    #[doc = "Number of nodes in the agent pool. The default value is 1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "The VM sku size of the agent pool node VMs."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[doc = "Version of Kubernetes in use by the agent pool. This is inherited from the kubernetesVersion of the provisioned cluster."]
    #[serde(rename = "kubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
}
impl AgentPoolUpdateProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The profile for the underlying cloud infrastructure provider for the provisioned cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudProviderProfile {
    #[doc = "The profile for the infrastructure networks used by the provisioned cluster"]
    #[serde(rename = "infraNetworkProfile", default, skip_serializing_if = "Option::is_none")]
    pub infra_network_profile: Option<cloud_provider_profile::InfraNetworkProfile>,
}
impl CloudProviderProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cloud_provider_profile {
    use super::*;
    #[doc = "The profile for the infrastructure networks used by the provisioned cluster"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct InfraNetworkProfile {
        #[doc = "List of ARM resource Ids (maximum 1) for the infrastructure network object e.g. /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.AzureStackHCI/logicalNetworks/{logicalNetworkName}"]
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
}
#[doc = "The SSH restricted access profile for the VMs in the provisioned cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterVmAccessProfile {
    #[doc = "IP Address or CIDR for SSH access to VMs in the provisioned cluster"]
    #[serde(rename = "authorizedIPRanges", default, skip_serializing_if = "Option::is_none")]
    pub authorized_ip_ranges: Option<String>,
}
impl ClusterVmAccessProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the control plane nodes of the provisioned cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ControlPlaneProfile {
    #[doc = "Number of control plane nodes. The default value is 1, and the count should be an odd number"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "VM sku size of the control plane nodes"]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[doc = "IP Address of the Kubernetes API server"]
    #[serde(rename = "controlPlaneEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub control_plane_endpoint: Option<control_plane_profile::ControlPlaneEndpoint>,
}
impl ControlPlaneProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod control_plane_profile {
    use super::*;
    #[doc = "IP Address of the Kubernetes API server"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ControlPlaneEndpoint {
        #[doc = "IP address of the Kubernetes API server"]
        #[serde(rename = "hostIP", default, skip_serializing_if = "Option::is_none")]
        pub host_ip: Option<String>,
    }
    impl ControlPlaneEndpoint {
        pub fn new() -> Self {
            Self::default()
        }
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
#[doc = "Extended location pointing to the underlying infrastructure"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedLocation {
    #[doc = "The extended location type. Allowed value: 'CustomLocation'"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<extended_location::Type>,
    #[doc = "ARM Id of the extended location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ExtendedLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod extended_location {
    use super::*;
    #[doc = "The extended location type. Allowed value: 'CustomLocation'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        CustomLocation,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CustomLocation => serializer.serialize_unit_variant("Type", 0u32, "CustomLocation"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Kubernetes Patch Version profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesPatchVersions {
    #[doc = "Indicates whether the kubernetes version image is ready or not"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub readiness: Vec<KubernetesVersionReadiness>,
    #[doc = "Possible upgrade paths for given patch version"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub upgrades: Vec<String>,
}
impl KubernetesPatchVersions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The supported kubernetes versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesVersionProfile {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Extended location pointing to the underlying infrastructure"]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<kubernetes_version_profile::Properties>,
}
impl KubernetesVersionProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod kubernetes_version_profile {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Provisioning state of the resource"]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<ProvisioningState>,
        #[doc = "List of supported Kubernetes versions"]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub values: Vec<KubernetesVersionProperties>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "List of supported kubernetes versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesVersionProfileList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<KubernetesVersionProfile>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for KubernetesVersionProfileList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl KubernetesVersionProfileList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kubernetes version profile for given major.minor release"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesVersionProperties {
    #[doc = "major.minor version of Kubernetes release"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Whether this version is in preview mode."]
    #[serde(rename = "isPreview", default, skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<bool>,
    #[doc = "Patch versions of a Kubernetes release"]
    #[serde(rename = "patchVersions", default, skip_serializing_if = "Option::is_none")]
    pub patch_versions: Option<serde_json::Value>,
}
impl KubernetesVersionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates whether the kubernetes version image is ready or not"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesVersionReadiness {
    #[doc = "The particular KubernetesVersion Image OS Type (Linux, Windows)"]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<kubernetes_version_readiness::OsType>,
    #[doc = "Specifies the OS SKU used by the agent pool. The default is CBLMariner if OSType is Linux. The default is Windows2019 when OSType is Windows."]
    #[serde(rename = "osSku", default, skip_serializing_if = "Option::is_none")]
    pub os_sku: Option<Ossku>,
    #[doc = "Whether the kubernetes version image is ready or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    #[doc = "The error message for version not being ready"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl KubernetesVersionReadiness {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod kubernetes_version_readiness {
    use super::*;
    #[doc = "The particular KubernetesVersion Image OS Type (Linux, Windows)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OsType")]
    pub enum OsType {
        Windows,
        Linux,
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
                Self::Windows => serializer.serialize_unit_variant("OsType", 0u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("OsType", 1u32, "Linux"),
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
#[doc = "The profile for Linux VMs in the provisioned cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxProfile {
    #[doc = "SSH profile for control plane and nodepool VMs of the provisioned cluster."]
    #[serde(rename = "linuxProfile", default, skip_serializing_if = "Option::is_none")]
    pub linux_profile: Option<LinuxProfileProperties>,
}
impl LinuxProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SSH profile for control plane and nodepool VMs of the provisioned cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxProfileProperties {
    #[doc = "SSH configuration for VMs of the provisioned cluster."]
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
    #[doc = "SSH configuration for VMs of the provisioned cluster."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Ssh {
        #[doc = "The list of SSH public keys used to authenticate with VMs. A maximum of 1 key may be specified."]
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
#[doc = "The list kubeconfig result response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListCredentialResponse {
    #[doc = "Operation Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Operation Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "ARM Resource Id of the provisioned cluster instance"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Provisioning state of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<list_credential_response::Error>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<list_credential_response::Properties>,
}
impl ListCredentialResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod list_credential_response {
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Base64-encoded Kubernetes configuration file."]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub kubeconfigs: Vec<CredentialResult>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Profile of the default agent pool along with a name parameter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamedAgentPoolProfile {
    #[serde(flatten)]
    pub agent_pool_profile: AgentPoolProfile,
    #[serde(flatten)]
    pub agent_pool_update_profile: AgentPoolUpdateProfile,
    #[serde(flatten)]
    pub agent_pool_name: AgentPoolName,
}
impl NamedAgentPoolProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network configuration profile for the provisioned cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfile {
    #[doc = "Profile of the HA Proxy load balancer."]
    #[serde(rename = "loadBalancerProfile", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_profile: Option<network_profile::LoadBalancerProfile>,
    #[doc = "Network policy used for building Kubernetes network. Possible values include: 'calico'."]
    #[serde(rename = "networkPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_policy: Option<network_profile::NetworkPolicy>,
    #[doc = "A CIDR notation IP Address range from which to assign pod IPs."]
    #[serde(rename = "podCidr", default, skip_serializing_if = "Option::is_none")]
    pub pod_cidr: Option<String>,
}
impl NetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_profile {
    use super::*;
    #[doc = "Profile of the HA Proxy load balancer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct LoadBalancerProfile {
        #[doc = "Number of HA Proxy load balancer VMs. The default value is 0."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub count: Option<i32>,
    }
    impl LoadBalancerProfile {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Network policy used for building Kubernetes network. Possible values include: 'calico'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NetworkPolicy")]
    pub enum NetworkPolicy {
        #[serde(rename = "calico")]
        Calico,
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
#[doc = "Specifies the OS SKU used by the agent pool. The default is CBLMariner if OSType is Linux. The default is Windows2019 when OSType is Windows."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Ossku")]
pub enum Ossku {
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
            Self::CblMariner => serializer.serialize_unit_variant("Ossku", 0u32, "CBLMariner"),
            Self::Windows2019 => serializer.serialize_unit_variant("Ossku", 1u32, "Windows2019"),
            Self::Windows2022 => serializer.serialize_unit_variant("Ossku", 2u32, "Windows2022"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "OSType to be used to specify OS type for the VMs. Choose from Linux and Windows. Default to Linux. Possible values include: 'Linux', 'Windows'"]
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The license profile of the provisioned cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedClusterLicenseProfile {
    #[doc = "Indicates whether Azure Hybrid Benefit is opted in. Default value is false"]
    #[serde(rename = "azureHybridBenefit", default, skip_serializing_if = "Option::is_none")]
    pub azure_hybrid_benefit: Option<provisioned_cluster_license_profile::AzureHybridBenefit>,
}
impl ProvisionedClusterLicenseProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod provisioned_cluster_license_profile {
    use super::*;
    #[doc = "Indicates whether Azure Hybrid Benefit is opted in. Default value is false"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AzureHybridBenefit")]
    pub enum AzureHybridBenefit {
        True,
        False,
        NotApplicable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AzureHybridBenefit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AzureHybridBenefit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AzureHybridBenefit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("AzureHybridBenefit", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("AzureHybridBenefit", 1u32, "False"),
                Self::NotApplicable => serializer.serialize_unit_variant("AzureHybridBenefit", 2u32, "NotApplicable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for AzureHybridBenefit {
        fn default() -> Self {
            Self::NotApplicable
        }
    }
}
#[doc = "The list of available kubernetes versions for upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedClusterPoolUpgradeProfile {
    #[doc = "The Kubernetes version (major.minor.patch)."]
    #[serde(rename = "kubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    #[doc = "OSType to be used to specify OS type for the VMs. Choose from Linux and Windows. Default to Linux. Possible values include: 'Linux', 'Windows'"]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "List of available kubernetes versions for upgrade."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub upgrades: Vec<ProvisionedClusterPoolUpgradeProfileProperties>,
}
impl ProvisionedClusterPoolUpgradeProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The upgrade properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedClusterPoolUpgradeProfileProperties {
    #[doc = "The Kubernetes version (major.minor.patch)."]
    #[serde(rename = "kubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    #[doc = "Whether the Kubernetes version is currently in preview."]
    #[serde(rename = "isPreview", default, skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<bool>,
}
impl ProvisionedClusterPoolUpgradeProfileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of available kubernetes version upgrades for the provisioned cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvisionedClusterUpgradeProfile {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Control plane and agent pool upgrade profiles."]
    pub properties: ProvisionedClusterUpgradeProfileProperties,
}
impl ProvisionedClusterUpgradeProfile {
    pub fn new(properties: ProvisionedClusterUpgradeProfileProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "Control plane and agent pool upgrade profiles."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvisionedClusterUpgradeProfileProperties {
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The list of available kubernetes versions for upgrade."]
    #[serde(rename = "controlPlaneProfile")]
    pub control_plane_profile: ProvisionedClusterPoolUpgradeProfile,
}
impl ProvisionedClusterUpgradeProfileProperties {
    pub fn new(control_plane_profile: ProvisionedClusterPoolUpgradeProfile) -> Self {
        Self {
            provisioning_state: None,
            control_plane_profile,
        }
    }
}
#[doc = "Provisioning state of the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Pending,
    Creating,
    Deleting,
    Updating,
    Upgrading,
    Accepted,
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
            Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Pending"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
            Self::Upgrading => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Upgrading"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Accepted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
    #[doc = "Fully qualified resource ID for the resource. E.g. \"/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}\""]
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
#[doc = "The storage configuration profile for the provisioned cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[doc = "SMB CSI Driver settings for the storage profile."]
    #[serde(rename = "smbCsiDriver", default, skip_serializing_if = "Option::is_none")]
    pub smb_csi_driver: Option<StorageProfileSmbCsiDriver>,
    #[doc = "NFS CSI Driver settings for the storage profile."]
    #[serde(rename = "nfsCsiDriver", default, skip_serializing_if = "Option::is_none")]
    pub nfs_csi_driver: Option<StorageProfileNfsCsiDriver>,
}
impl StorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NFS CSI Driver settings for the storage profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfileNfsCsiDriver {
    #[doc = "Indicates whether to enable NFS CSI Driver. The default value is true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl StorageProfileNfsCsiDriver {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SMB CSI Driver settings for the storage profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfileSmbCsiDriver {
    #[doc = "Indicates whether to enable SMB CSI Driver. The default value is true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl StorageProfileSmbCsiDriver {
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
#[doc = "Describes the VM SKU capabilities like MemoryGB, vCPUs, etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmSkuCapabilities {
    #[doc = "Name of the VM SKU capability"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Value of the VM SKU capability"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl VmSkuCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of supported VM SKUs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmSkuProfile {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Extended location pointing to the underlying infrastructure"]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<vm_sku_profile::Properties>,
}
impl VmSkuProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vm_sku_profile {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Provisioning state of the resource"]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<ProvisioningState>,
        #[doc = "List of supported VM SKUs."]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub values: Vec<VmSkuProperties>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The list of supported VM SKUs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmSkuProfileList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VmSkuProfile>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VmSkuProfileList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VmSkuProfileList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The profile for supported VM SKUs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmSkuProperties {
    #[doc = "The type of resource the SKU applies to."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The list of name-value pairs to describe VM SKU capabilities like MemoryGB, vCPUs, etc."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub capabilities: Vec<VmSkuCapabilities>,
    #[doc = "The name of the VM SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The tier of the VM SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The size of the VM SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}
impl VmSkuProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The agentPool resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPool {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the agent pool resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AgentPoolProperties>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Extended location pointing to the underlying infrastructure"]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
}
impl AgentPool {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of all agent pool resources associated with the provisioned cluster."]
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
impl azure_core::Continuable for AgentPoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AgentPoolListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the agent pool resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolProperties {
    #[serde(flatten)]
    pub agent_pool_profile: AgentPoolProfile,
    #[serde(flatten)]
    pub agent_pool_update_profile: AgentPoolUpdateProfile,
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
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The observed status of the agent pool."]
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
    #[doc = "The observed status of the agent pool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Status {
        #[doc = "Provisioning state of the resource"]
        #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
        pub current_state: Option<ProvisioningState>,
        #[doc = "Error messages during an agent pool operation or steady state."]
        #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
        pub error_message: Option<String>,
        #[serde(
            rename = "readyReplicas",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub ready_replicas: Vec<AgentPoolUpdateProfile>,
    }
    impl Status {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Defines the hybridIdentityMetadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridIdentityMetadata {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Defines the resource properties for the hybrid identity metadata."]
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
#[doc = "Defines the resource properties for the hybrid identity metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridIdentityMetadataProperties {
    #[doc = "Unique id of the parent provisioned cluster resource."]
    #[serde(rename = "resourceUid", default, skip_serializing_if = "Option::is_none")]
    pub resource_uid: Option<String>,
    #[doc = "Onboarding public key for provisioning the Managed identity for the connected cluster."]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl HybridIdentityMetadataProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The provisioned cluster resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedCluster {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the provisioned cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProvisionedClusterProperties>,
    #[doc = "Extended location pointing to the underlying infrastructure"]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
}
impl ProvisionedCluster {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lists the ProvisionedClusterInstance resource associated with the ConnectedCluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedClusterListResult {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProvisionedCluster>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProvisionedClusterListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ProvisionedClusterListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the provisioned cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedClusterProperties {
    #[doc = "SSH profile for control plane and nodepool VMs of the provisioned cluster."]
    #[serde(rename = "linuxProfile", default, skip_serializing_if = "Option::is_none")]
    pub linux_profile: Option<LinuxProfileProperties>,
    #[doc = "The properties of the control plane nodes of the provisioned cluster"]
    #[serde(rename = "controlPlane", default, skip_serializing_if = "Option::is_none")]
    pub control_plane: Option<ControlPlaneProfile>,
    #[doc = "The version of Kubernetes in use by the provisioned cluster."]
    #[serde(rename = "kubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    #[doc = "The network configuration profile for the provisioned cluster."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    #[doc = "The storage configuration profile for the provisioned cluster."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "The SSH restricted access profile for the VMs in the provisioned cluster."]
    #[serde(rename = "clusterVMAccessProfile", default, skip_serializing_if = "Option::is_none")]
    pub cluster_vm_access_profile: Option<ClusterVmAccessProfile>,
    #[doc = "The agent pool properties for the provisioned cluster."]
    #[serde(
        rename = "agentPoolProfiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub agent_pool_profiles: Vec<NamedAgentPoolProfile>,
    #[doc = "The profile for the underlying cloud infrastructure provider for the provisioned cluster."]
    #[serde(rename = "cloudProviderProfile", default, skip_serializing_if = "Option::is_none")]
    pub cloud_provider_profile: Option<CloudProviderProfile>,
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The observed status of the provisioned cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<provisioned_cluster_properties::Status>,
    #[doc = "The license profile of the provisioned cluster."]
    #[serde(rename = "licenseProfile", default, skip_serializing_if = "Option::is_none")]
    pub license_profile: Option<ProvisionedClusterLicenseProfile>,
    #[doc = "Parameters to be applied to the cluster-autoscaler when auto scaling is enabled for the provisioned cluster."]
    #[serde(rename = "autoScalerProfile", default, skip_serializing_if = "Option::is_none")]
    pub auto_scaler_profile: Option<provisioned_cluster_properties::AutoScalerProfile>,
}
impl ProvisionedClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod provisioned_cluster_properties {
    use super::*;
    #[doc = "The observed status of the provisioned cluster."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Status {
        #[doc = "The detailed status of the provisioned cluster components including addons."]
        #[serde(
            rename = "controlPlaneStatus",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub control_plane_status: Vec<AddonStatusProfile>,
        #[doc = "Provisioning state of the resource"]
        #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
        pub current_state: Option<ProvisioningState>,
        #[doc = "Error messages during a provisioned cluster operation or steady state."]
        #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
        pub error_message: Option<String>,
    }
    impl Status {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Parameters to be applied to the cluster-autoscaler when auto scaling is enabled for the provisioned cluster."]
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
    pub created_at: Option<::time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<::time::OffsetDateTime>,
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
#[doc = "The Virtual Network resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetwork {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of the virtual network resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkProperties>,
    #[doc = "Extended location pointing to the underlying infrastructure"]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<virtual_network::ExtendedLocation>,
}
impl VirtualNetwork {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location: None,
        }
    }
}
pub mod virtual_network {
    use super::*;
    #[doc = "Extended location pointing to the underlying infrastructure"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ExtendedLocation {
        #[doc = "The extended location type. Allowed value: 'CustomLocation'"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<extended_location::Type>,
        #[doc = "ARM Id of the extended location."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    impl ExtendedLocation {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod extended_location {
        use super::*;
        #[doc = "The extended location type. Allowed value: 'CustomLocation'"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Type")]
        pub enum Type {
            CustomLocation,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for Type {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for Type {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for Type {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::CustomLocation => serializer.serialize_unit_variant("Type", 0u32, "CustomLocation"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Properties of the virtual network resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkProperties {
    #[serde(rename = "infraVnetProfile", default, skip_serializing_if = "Option::is_none")]
    pub infra_vnet_profile: Option<virtual_network_properties::InfraVnetProfile>,
    #[doc = "Range of IP Addresses for Kubernetes API Server and services if using HA Proxy load balancer"]
    #[serde(
        rename = "vipPool",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vip_pool: Vec<serde_json::Value>,
    #[doc = "Range of IP Addresses for Kubernetes node VMs"]
    #[serde(
        rename = "vmipPool",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vmip_pool: Vec<serde_json::Value>,
    #[doc = "List of DNS server IP Addresses associated with the network"]
    #[serde(
        rename = "dnsServers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dns_servers: Vec<String>,
    #[doc = "IP Address of the Gateway associated with the network"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[doc = "IP Address Prefix of the network"]
    #[serde(rename = "ipAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub ip_address_prefix: Option<String>,
    #[doc = "VLAN Id used by the network"]
    #[serde(rename = "vlanID", default, skip_serializing_if = "Option::is_none")]
    pub vlan_id: Option<i32>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<virtual_network_properties::ProvisioningState>,
    #[doc = "Status of the virtual network resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<virtual_network_properties::Status>,
}
impl VirtualNetworkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_network_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct InfraVnetProfile {
        #[doc = "Infrastructure network profile for HCI platform"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hci: Option<infra_vnet_profile::Hci>,
    }
    impl InfraVnetProfile {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod infra_vnet_profile {
        use super::*;
        #[doc = "Infrastructure network profile for HCI platform"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Hci {
            #[doc = "Group in MOC(Microsoft On-premises Cloud)"]
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
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Pending,
        Creating,
        Deleting,
        Updating,
        Accepted,
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
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Pending"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Accepted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Status of the virtual network resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Status {
        #[doc = "The detailed status of the long running operation."]
        #[serde(rename = "operationStatus", default, skip_serializing_if = "Option::is_none")]
        pub operation_status: Option<status::OperationStatus>,
    }
    impl Status {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod status {
        use super::*;
        #[doc = "The detailed status of the long running operation."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct OperationStatus {
            #[doc = "The error if any from the operation."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub error: Option<operation_status::Error>,
            #[doc = "The identifier of the operation."]
            #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
            pub operation_id: Option<String>,
            #[doc = "The status of the operation."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub status: Option<String>,
        }
        impl OperationStatus {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod operation_status {
            use super::*;
            #[doc = "The error if any from the operation."]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
            pub struct Error {
                #[doc = "The error code from the operation."]
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub code: Option<String>,
                #[doc = "The error message from the operation."]
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
#[doc = "A list of virtual network resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworksListResult {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VirtualNetwork>,
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
#[doc = "The Virtual Network resource patch definition."]
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

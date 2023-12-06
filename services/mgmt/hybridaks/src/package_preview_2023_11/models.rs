#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Defines the addon status profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddonStatusProfile {
    #[doc = "Name of the addon"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Observed phase of the addon on the target cluster. Possible values include: 'pending', 'provisioning', 'provisioning {HelmChartInstalled}', 'provisioning {MSICertificateDownloaded}', 'provisioned', 'deleting', 'failed', 'upgrading'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<addon_status_profile::Phase>,
    #[doc = "Indicates whether the addon is ready"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    #[doc = "Error message while deploying the addon"]
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
    #[doc = "Observed phase of the addon on the target cluster. Possible values include: 'pending', 'provisioning', 'provisioning {HelmChartInstalled}', 'provisioning {MSICertificateDownloaded}', 'provisioned', 'deleting', 'failed', 'upgrading'"]
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
#[doc = "AgentPool configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolProfile {
    #[doc = "AvailabilityZones - The list of Availability zones to use for nodes. Datacenter racks modelled as zones"]
    #[serde(
        rename = "availabilityZones",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub availability_zones: Vec<String>,
    #[doc = "OsType - OsType to be used to specify os type. Choose from Linux and Windows. Default to Linux. Possible values include: 'Linux', 'Windows'"]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "Specifies the OS SKU used by the agent pool. The default is CBLMariner if OSType is Linux. The default is Windows2019 when OSType is Windows."]
    #[serde(rename = "osSKU", default, skip_serializing_if = "Option::is_none")]
    pub os_sku: Option<Ossku>,
    #[doc = "The version of node image"]
    #[serde(rename = "nodeImageVersion", default, skip_serializing_if = "Option::is_none")]
    pub node_image_version: Option<String>,
}
impl AgentPoolProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AgentPool update configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolUpdateProfile {
    #[doc = "Count - Number of agents to host docker containers. Allowed values must be in the range of 1 to 100 (inclusive). The default value is 1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "VmSize - The size of the agent pool VMs."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
}
impl AgentPoolUpdateProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CloudProviderProfile - The underlying cloud infra provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudProviderProfile {
    #[doc = "InfraNetworkProfile - List of infra network profiles for the provisioned cluster"]
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
    #[doc = "InfraNetworkProfile - List of infra network profiles for the provisioned cluster"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct InfraNetworkProfile {
        #[doc = "Array of references to azure resource corresponding to the Network object e.g. /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.AzureStackHCI/logicalNetworks/{logicalNetworkName}"]
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
        pub port: Option<i32>,
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
#[doc = "Extended Location definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedLocation {
    #[doc = "The extended location type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<extended_location::Type>,
    #[doc = "The extended location name."]
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
    #[doc = "The extended location type."]
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
    #[doc = "Whether the kubernetes version variant (Linux, Windows, Windows2022) is ready or not"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub readiness: Vec<KubernetesVersionReadiness>,
    #[doc = "Possible upgrade path for given patch version"]
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
#[doc = "Different support tiers for AKS managed clusters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum KubernetesSupportPlan {
    KubernetesOfficial,
}
#[doc = "Capabilities on this kubernetes version"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesVersionCapabilities {
    #[serde(
        rename = "supportPlan",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub support_plan: Vec<KubernetesSupportPlan>,
}
impl KubernetesVersionCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The supported kubernetes versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesVersionProfile {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Extended Location definition"]
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
#[doc = "A list of kubernetes version resources."]
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
    #[doc = "Capabilities on this kubernetes version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<KubernetesVersionCapabilities>,
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
#[doc = "Whether a particular kubernetes version's variant (CBLMariner, Windows, Windows2022) is ready or not "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesVersionReadiness {
    #[doc = "The particular KubernetesVersion's Image's OS Type (Linux, Windows)"]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<kubernetes_version_readiness::OsType>,
    #[doc = "Specifies the OS SKU used by the agent pool. The default is CBLMariner if OSType is Linux. The default is Windows2019 when OSType is Windows."]
    #[serde(rename = "osSku", default, skip_serializing_if = "Option::is_none")]
    pub os_sku: Option<Ossku>,
    #[doc = "Whether or not the given image is ready"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    #[doc = "If image is not ready, the error message for version not being ready"]
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
    #[doc = "The particular KubernetesVersion's Image's OS Type (Linux, Windows)"]
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
#[doc = "Agent pool profile along with a name parameter"]
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
#[doc = "NetworkProfile - Profile of network configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfile {
    #[doc = "LoadBalancerProfile - Profile of the cluster load balancer."]
    #[serde(rename = "loadBalancerProfile", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_profile: Option<network_profile::LoadBalancerProfile>,
    #[doc = "NetworkPolicy - Network policy used for building Kubernetes network. Possible values include: 'calico', 'flannel'. Default is 'calico'"]
    #[serde(rename = "networkPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_policy: Option<network_profile::NetworkPolicy>,
    #[doc = "PodCidr - A CIDR notation IP range from which to assign pod IPs when kubenet is used."]
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
    #[doc = "LoadBalancerProfile - Profile of the cluster load balancer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct LoadBalancerProfile {
        #[doc = "Count - Number of load balancer VMs. The default value is 0."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub count: Option<i32>,
    }
    impl LoadBalancerProfile {
        pub fn new() -> Self {
            Self::default()
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
    #[doc = "Indicates whether Azure Hybrid Benefit is opted in"]
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
    #[doc = "Indicates whether Azure Hybrid Benefit is opted in"]
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
#[doc = "The list of available upgrade versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedClusterPoolUpgradeProfile {
    #[doc = "The Kubernetes version (major.minor.patch)."]
    #[serde(rename = "kubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    #[doc = "The Agent Pool name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "OsType - OsType to be used to specify os type. Choose from Linux and Windows. Default to Linux. Possible values include: 'Linux', 'Windows'"]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "List of orchestrator types and versions available for upgrade."]
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
#[doc = "The list of available upgrades for compute pools."]
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
    #[doc = "The list of available upgrade versions."]
    #[serde(rename = "controlPlaneProfile")]
    pub control_plane_profile: ProvisionedClusterPoolUpgradeProfile,
    #[doc = "The list of available upgrade versions for agent pools."]
    #[serde(rename = "agentPoolProfiles")]
    pub agent_pool_profiles: Vec<ProvisionedClusterPoolUpgradeProfile>,
}
impl ProvisionedClusterUpgradeProfileProperties {
    pub fn new(
        control_plane_profile: ProvisionedClusterPoolUpgradeProfile,
        agent_pool_profiles: Vec<ProvisionedClusterPoolUpgradeProfile>,
    ) -> Self {
        Self {
            provisioning_state: None,
            control_plane_profile,
            agent_pool_profiles,
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
    Creating,
    Deleting,
    Updating,
    Upgrading,
    InProgress,
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
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Updating"),
            Self::Upgrading => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Upgrading"),
            Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 7u32, "InProgress"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Accepted"),
            Self::Created => serializer.serialize_unit_variant("ProvisioningState", 9u32, "Created"),
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
#[doc = "describes the vm sku capabilities object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmSkuCapabilities {
    #[doc = "An invariant to describe the feature"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "An invariant if the feature is measured by quantity"]
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
    #[doc = "Extended Location definition"]
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
        #[doc = "Array of HybridAKS Support VM Skus"]
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
#[doc = "A list of VM SKU resources."]
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
#[doc = "The profile for supported VM skus"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmSkuProperties {
    #[doc = "The resource type of the vm"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "A name value pair to describe the specific vm's capability"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub capabilities: Vec<VmSkuCapabilities>,
    #[doc = "The name of the VM Family"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The tier of the VM Family"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The size of the VM Family"]
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
    #[doc = "Extended Location definition"]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
}
impl AgentPool {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of agent pool resources."]
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
#[doc = "The agentPool resource patch definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolPatch {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AgentPoolPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
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
    #[doc = "Defines the observed state of the agent pool"]
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
    #[doc = "Defines the observed state of the agent pool"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Status {
        #[doc = "ErrorMessage - Error messages during creation of agent pool"]
        #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
        pub error_message: Option<String>,
        #[doc = "Contains Provisioning errors"]
        #[serde(rename = "operationStatus", default, skip_serializing_if = "Option::is_none")]
        pub operation_status: Option<status::OperationStatus>,
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
    pub mod status {
        use super::*;
        #[doc = "Contains Provisioning errors"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct OperationStatus {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub error: Option<operation_status::Error>,
            #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
            pub operation_id: Option<String>,
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
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl HybridIdentityMetadataProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "All properties of the provisioned cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedClusterProperties {
    #[doc = "LinuxProfile - Profile for Linux VMs in the container service cluster."]
    #[serde(rename = "linuxProfile", default, skip_serializing_if = "Option::is_none")]
    pub linux_profile: Option<LinuxProfileProperties>,
    #[doc = "ControlPlaneProfile - The control plane properties for the provisioned cluster."]
    #[serde(rename = "controlPlane", default, skip_serializing_if = "Option::is_none")]
    pub control_plane: Option<ControlPlaneProfile>,
    #[doc = "KubernetesVersion - Version of Kubernetes specified when creating the managed cluster."]
    #[serde(rename = "kubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    #[doc = "NetworkProfile - Profile of network configuration."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
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
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "HybridAKSClusterStatus defines the observed state of HybridAKSCluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<provisioned_cluster_properties::Status>,
    #[doc = "The license profile of the provisioned cluster."]
    #[serde(rename = "licenseProfile", default, skip_serializing_if = "Option::is_none")]
    pub license_profile: Option<ProvisionedClusterLicenseProfile>,
}
impl ProvisionedClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod provisioned_cluster_properties {
    use super::*;
    #[doc = "HybridAKSClusterStatus defines the observed state of HybridAKSCluster"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Status {
        #[doc = "Status of the control plane components"]
        #[serde(
            rename = "controlPlaneStatus",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub control_plane_status: Vec<AddonStatusProfile>,
        #[doc = "ErrorMessage - Error messages during creation of cluster"]
        #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
        pub error_message: Option<String>,
        #[doc = "Contains Provisioning errors"]
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
        #[doc = "Contains Provisioning errors"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct OperationStatus {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub error: Option<operation_status::Error>,
            #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
            pub operation_id: Option<String>,
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
#[doc = "The provisionedClusterInstances resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedClusters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "All properties of the provisioned cluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProvisionedClusterProperties>,
    #[doc = "Extended Location definition"]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
}
impl ProvisionedClusters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of provisioned clusters resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedClustersListResult {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProvisionedClusters>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProvisionedClustersListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ProvisionedClustersListResult {
    pub fn new() -> Self {
        Self::default()
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
pub struct VirtualNetwork {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "HybridAKSNetworkSpec defines the desired state of HybridAKSNetwork"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<virtual_network::ExtendedLocation>,
}
impl VirtualNetwork {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            system_data: None,
            extended_location: None,
        }
    }
}
pub mod virtual_network {
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
#[doc = "HybridAKSNetworkSpec defines the desired state of HybridAKSNetwork"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkProperties {
    #[serde(rename = "infraVnetProfile", default, skip_serializing_if = "Option::is_none")]
    pub infra_vnet_profile: Option<virtual_network_properties::InfraVnetProfile>,
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
    pub vlan_id: Option<i32>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<virtual_network_properties::ProvisioningState>,
    #[doc = "HybridAKSNetworkStatus defines the observed state of HybridAKSNetwork"]
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
        #[doc = "Infra network profile for HCI platform"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hci: Option<infra_vnet_profile::Hci>,
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
        #[doc = "Contains Provisioning errors"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct OperationStatus {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub error: Option<operation_status::Error>,
            #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
            pub operation_id: Option<String>,
            #[doc = "Phase represents the current phase of the virtual network provisioning. E.g. Pending, Running, Terminating, Failed etc."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub phase: Option<String>,
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

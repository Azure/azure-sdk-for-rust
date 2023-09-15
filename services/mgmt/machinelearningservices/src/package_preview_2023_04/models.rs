#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A Machine Learning compute based on AKS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Aks {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(flatten)]
    pub aks_schema: AksSchema,
}
impl Aks {
    pub fn new(compute: Compute) -> Self {
        Self {
            compute,
            aks_schema: AksSchema::default(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AksSchema {
    #[doc = "AKS properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<aks_schema::Properties>,
}
impl AksSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod aks_schema {
    use super::*;
    #[doc = "AKS properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Cluster full qualified domain name"]
        #[serde(rename = "clusterFqdn", default, skip_serializing_if = "Option::is_none")]
        pub cluster_fqdn: Option<String>,
        #[doc = "System services"]
        #[serde(
            rename = "systemServices",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub system_services: Vec<SystemService>,
        #[doc = "Number of agents"]
        #[serde(rename = "agentCount", default, skip_serializing_if = "Option::is_none")]
        pub agent_count: Option<i32>,
        #[doc = "Agent virtual machine size"]
        #[serde(rename = "agentVmSize", default, skip_serializing_if = "Option::is_none")]
        pub agent_vm_size: Option<String>,
        #[doc = "Intended usage of the cluster"]
        #[serde(rename = "clusterPurpose", default, skip_serializing_if = "Option::is_none")]
        pub cluster_purpose: Option<properties::ClusterPurpose>,
        #[doc = "The ssl configuration for scoring"]
        #[serde(rename = "sslConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub ssl_configuration: Option<SslConfiguration>,
        #[doc = "Advance configuration for AKS networking"]
        #[serde(rename = "aksNetworkingConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub aks_networking_configuration: Option<AksNetworkingConfiguration>,
        #[doc = "Load Balancer Type"]
        #[serde(rename = "loadBalancerType", default, skip_serializing_if = "Option::is_none")]
        pub load_balancer_type: Option<properties::LoadBalancerType>,
        #[doc = "Load Balancer Subnet"]
        #[serde(rename = "loadBalancerSubnet", default, skip_serializing_if = "Option::is_none")]
        pub load_balancer_subnet: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Intended usage of the cluster"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ClusterPurpose")]
        pub enum ClusterPurpose {
            FastProd,
            DenseProd,
            DevTest,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for ClusterPurpose {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for ClusterPurpose {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for ClusterPurpose {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::FastProd => serializer.serialize_unit_variant("ClusterPurpose", 0u32, "FastProd"),
                    Self::DenseProd => serializer.serialize_unit_variant("ClusterPurpose", 1u32, "DenseProd"),
                    Self::DevTest => serializer.serialize_unit_variant("ClusterPurpose", 2u32, "DevTest"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        impl Default for ClusterPurpose {
            fn default() -> Self {
                Self::FastProd
            }
        }
        #[doc = "Load Balancer Type"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "LoadBalancerType")]
        pub enum LoadBalancerType {
            PublicIp,
            InternalLoadBalancer,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for LoadBalancerType {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for LoadBalancerType {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for LoadBalancerType {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::PublicIp => serializer.serialize_unit_variant("LoadBalancerType", 0u32, "PublicIp"),
                    Self::InternalLoadBalancer => serializer.serialize_unit_variant("LoadBalancerType", 1u32, "InternalLoadBalancer"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        impl Default for LoadBalancerType {
            fn default() -> Self {
                Self::PublicIp
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessKeyAuthTypeWorkspaceConnectionProperties {
    #[serde(flatten)]
    pub workspace_connection_properties_v2: WorkspaceConnectionPropertiesV2,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<WorkspaceConnectionAccessKey>,
}
impl AccessKeyAuthTypeWorkspaceConnectionProperties {
    pub fn new(workspace_connection_properties_v2: WorkspaceConnectionPropertiesV2) -> Self {
        Self {
            workspace_connection_properties_v2,
            credentials: None,
        }
    }
}
#[doc = "Account key datastore credentials configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountKeyDatastoreCredentials {
    #[serde(flatten)]
    pub datastore_credentials: DatastoreCredentials,
    #[doc = "Datastore account key secrets."]
    pub secrets: AccountKeyDatastoreSecrets,
}
impl AccountKeyDatastoreCredentials {
    pub fn new(datastore_credentials: DatastoreCredentials, secrets: AccountKeyDatastoreSecrets) -> Self {
        Self {
            datastore_credentials,
            secrets,
        }
    }
}
#[doc = "Datastore account key secrets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountKeyDatastoreSecrets {
    #[serde(flatten)]
    pub datastore_secrets: DatastoreSecrets,
    #[doc = "Storage account key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl AccountKeyDatastoreSecrets {
    pub fn new(datastore_secrets: DatastoreSecrets) -> Self {
        Self {
            datastore_secrets,
            key: None,
        }
    }
}
#[doc = "Details of ACR account to be used for the Registry"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrDetails {
    #[serde(rename = "systemCreatedAcrAccount", default, skip_serializing_if = "Option::is_none")]
    pub system_created_acr_account: Option<SystemCreatedAcrAccount>,
    #[serde(rename = "userCreatedAcrAccount", default, skip_serializing_if = "Option::is_none")]
    pub user_created_acr_account: Option<UserCreatedAcrAccount>,
}
impl AcrDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Secrets related to a Machine Learning compute based on AKS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AksComputeSecrets {
    #[serde(flatten)]
    pub compute_secrets: ComputeSecrets,
    #[serde(flatten)]
    pub aks_compute_secrets_properties: AksComputeSecretsProperties,
}
impl AksComputeSecrets {
    pub fn new(compute_secrets: ComputeSecrets) -> Self {
        Self {
            compute_secrets,
            aks_compute_secrets_properties: AksComputeSecretsProperties::default(),
        }
    }
}
#[doc = "Properties of AksComputeSecrets"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AksComputeSecretsProperties {
    #[doc = "Content of kubeconfig file that can be used to connect to the Kubernetes cluster."]
    #[serde(rename = "userKubeConfig", default, skip_serializing_if = "Option::is_none")]
    pub user_kube_config: Option<String>,
    #[doc = "Content of kubeconfig file that can be used to connect to the Kubernetes cluster."]
    #[serde(rename = "adminKubeConfig", default, skip_serializing_if = "Option::is_none")]
    pub admin_kube_config: Option<String>,
    #[doc = "Image registry pull secret."]
    #[serde(rename = "imagePullSecretName", default, skip_serializing_if = "Option::is_none")]
    pub image_pull_secret_name: Option<String>,
}
impl AksComputeSecretsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Advance configuration for AKS networking"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AksNetworkingConfiguration {
    #[doc = "Virtual network subnet resource ID the compute nodes belong to"]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "A CIDR notation IP range from which to assign service cluster IPs. It must not overlap with any Subnet IP ranges."]
    #[serde(rename = "serviceCidr", default, skip_serializing_if = "Option::is_none")]
    pub service_cidr: Option<String>,
    #[doc = "An IP address assigned to the Kubernetes DNS service. It must be within the Kubernetes service address range specified in serviceCidr."]
    #[serde(rename = "dnsServiceIP", default, skip_serializing_if = "Option::is_none")]
    pub dns_service_ip: Option<String>,
    #[doc = "A CIDR notation IP range assigned to the Docker bridge network. It must not overlap with any Subnet IP ranges or the Kubernetes service address range."]
    #[serde(rename = "dockerBridgeCidr", default, skip_serializing_if = "Option::is_none")]
    pub docker_bridge_cidr: Option<String>,
}
impl AksNetworkingConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllFeatures {
    #[serde(flatten)]
    pub monitoring_feature_filter_base: MonitoringFeatureFilterBase,
}
impl AllFeatures {
    pub fn new(monitoring_feature_filter_base: MonitoringFeatureFilterBase) -> Self {
        Self {
            monitoring_feature_filter_base,
        }
    }
}
#[doc = "All nodes means the service will be running on all of the nodes of the job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllNodes {
    #[serde(flatten)]
    pub nodes: Nodes,
}
impl AllNodes {
    pub fn new(nodes: Nodes) -> Self {
        Self { nodes }
    }
}
#[doc = "An Azure Machine Learning compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmlCompute {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(flatten)]
    pub aml_compute_schema: AmlComputeSchema,
}
impl AmlCompute {
    pub fn new(compute: Compute) -> Self {
        Self {
            compute,
            aml_compute_schema: AmlComputeSchema::default(),
        }
    }
}
#[doc = "Compute node information related to a AmlCompute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmlComputeNodeInformation {
    #[doc = "ID of the compute node."]
    #[serde(rename = "nodeId", default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[doc = "Private IP address of the compute node."]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "Public IP address of the compute node."]
    #[serde(rename = "publicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    #[doc = "SSH port number of the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<f64>,
    #[doc = "State of the compute node. Values are idle, running, preparing, unusable, leaving and preempted."]
    #[serde(rename = "nodeState", default, skip_serializing_if = "Option::is_none")]
    pub node_state: Option<aml_compute_node_information::NodeState>,
    #[doc = "ID of the Experiment running on the node, if any else null."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}
impl AmlComputeNodeInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod aml_compute_node_information {
    use super::*;
    #[doc = "State of the compute node. Values are idle, running, preparing, unusable, leaving and preempted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NodeState")]
    pub enum NodeState {
        #[serde(rename = "idle")]
        Idle,
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "preparing")]
        Preparing,
        #[serde(rename = "unusable")]
        Unusable,
        #[serde(rename = "leaving")]
        Leaving,
        #[serde(rename = "preempted")]
        Preempted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NodeState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NodeState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NodeState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Idle => serializer.serialize_unit_variant("NodeState", 0u32, "idle"),
                Self::Running => serializer.serialize_unit_variant("NodeState", 1u32, "running"),
                Self::Preparing => serializer.serialize_unit_variant("NodeState", 2u32, "preparing"),
                Self::Unusable => serializer.serialize_unit_variant("NodeState", 3u32, "unusable"),
                Self::Leaving => serializer.serialize_unit_variant("NodeState", 4u32, "leaving"),
                Self::Preempted => serializer.serialize_unit_variant("NodeState", 5u32, "preempted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of AmlCompute Nodes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmlComputeNodesInformation {
    #[doc = "The collection of returned AmlCompute nodes details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub nodes: Vec<AmlComputeNodeInformation>,
    #[doc = "The continuation token."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AmlComputeNodesInformation {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AmlComputeNodesInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AML Compute properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmlComputeProperties {
    #[doc = "Compute OS Type"]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<aml_compute_properties::OsType>,
    #[doc = "Virtual Machine Size"]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[doc = "Virtual Machine priority"]
    #[serde(rename = "vmPriority", default, skip_serializing_if = "Option::is_none")]
    pub vm_priority: Option<aml_compute_properties::VmPriority>,
    #[doc = "Virtual Machine image for Windows AML Compute"]
    #[serde(rename = "virtualMachineImage", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_image: Option<VirtualMachineImage>,
    #[doc = "Network is isolated or not"]
    #[serde(rename = "isolatedNetwork", default, skip_serializing_if = "Option::is_none")]
    pub isolated_network: Option<bool>,
    #[doc = "scale settings for AML Compute"]
    #[serde(rename = "scaleSettings", default, skip_serializing_if = "Option::is_none")]
    pub scale_settings: Option<ScaleSettings>,
    #[doc = "Settings for user account that gets created on each on the nodes of a compute."]
    #[serde(rename = "userAccountCredentials", default, skip_serializing_if = "Option::is_none")]
    pub user_account_credentials: Option<UserAccountCredentials>,
    #[doc = "Represents a resource ID. For example, for a subnet, it is the resource URL for the subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<ResourceId>,
    #[doc = "State of the public SSH port. Possible values are: Disabled - Indicates that the public ssh port is closed on all nodes of the cluster. Enabled - Indicates that the public ssh port is open on all nodes of the cluster. NotSpecified - Indicates that the public ssh port is closed on all nodes of the cluster if VNet is defined, else is open all public nodes. It can be default only during cluster creation time, after creation it will be either enabled or disabled."]
    #[serde(rename = "remoteLoginPortPublicAccess", default, skip_serializing_if = "Option::is_none")]
    pub remote_login_port_public_access: Option<aml_compute_properties::RemoteLoginPortPublicAccess>,
    #[doc = "Allocation state of the compute. Possible values are: steady - Indicates that the compute is not resizing. There are no changes to the number of compute nodes in the compute in progress. A compute enters this state when it is created and when no operations are being performed on the compute to change the number of compute nodes. resizing - Indicates that the compute is resizing; that is, compute nodes are being added to or removed from the compute."]
    #[serde(rename = "allocationState", default, skip_serializing_if = "Option::is_none")]
    pub allocation_state: Option<aml_compute_properties::AllocationState>,
    #[doc = "The time at which the compute entered its current allocation state."]
    #[serde(rename = "allocationStateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub allocation_state_transition_time: Option<time::OffsetDateTime>,
    #[doc = "Collection of errors encountered by various compute nodes during node setup."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<ErrorResponse>,
    #[doc = "The number of compute nodes currently assigned to the compute."]
    #[serde(rename = "currentNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub current_node_count: Option<i32>,
    #[doc = "The target number of compute nodes for the compute. If the allocationState is resizing, this property denotes the target node count for the ongoing resize operation. If the allocationState is steady, this property denotes the target node count for the previous resize operation."]
    #[serde(rename = "targetNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub target_node_count: Option<i32>,
    #[doc = "Counts of various compute node states on the amlCompute."]
    #[serde(rename = "nodeStateCounts", default, skip_serializing_if = "Option::is_none")]
    pub node_state_counts: Option<NodeStateCounts>,
    #[doc = "Enable or disable node public IP address provisioning. Possible values are: Possible values are: true - Indicates that the compute nodes will have public IPs provisioned. false - Indicates that the compute nodes will have a private endpoint and no public IPs."]
    #[serde(rename = "enableNodePublicIp", default, skip_serializing_if = "Option::is_none")]
    pub enable_node_public_ip: Option<bool>,
    #[doc = "A property bag containing additional properties."]
    #[serde(rename = "propertyBag", default, skip_serializing_if = "Option::is_none")]
    pub property_bag: Option<serde_json::Value>,
}
impl AmlComputeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod aml_compute_properties {
    use super::*;
    #[doc = "Compute OS Type"]
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
    #[doc = "Virtual Machine priority"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VmPriority")]
    pub enum VmPriority {
        Dedicated,
        LowPriority,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VmPriority {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VmPriority {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VmPriority {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dedicated => serializer.serialize_unit_variant("VmPriority", 0u32, "Dedicated"),
                Self::LowPriority => serializer.serialize_unit_variant("VmPriority", 1u32, "LowPriority"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "State of the public SSH port. Possible values are: Disabled - Indicates that the public ssh port is closed on all nodes of the cluster. Enabled - Indicates that the public ssh port is open on all nodes of the cluster. NotSpecified - Indicates that the public ssh port is closed on all nodes of the cluster if VNet is defined, else is open all public nodes. It can be default only during cluster creation time, after creation it will be either enabled or disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RemoteLoginPortPublicAccess")]
    pub enum RemoteLoginPortPublicAccess {
        Enabled,
        Disabled,
        NotSpecified,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RemoteLoginPortPublicAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RemoteLoginPortPublicAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RemoteLoginPortPublicAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("RemoteLoginPortPublicAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("RemoteLoginPortPublicAccess", 1u32, "Disabled"),
                Self::NotSpecified => serializer.serialize_unit_variant("RemoteLoginPortPublicAccess", 2u32, "NotSpecified"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for RemoteLoginPortPublicAccess {
        fn default() -> Self {
            Self::NotSpecified
        }
    }
    #[doc = "Allocation state of the compute. Possible values are: steady - Indicates that the compute is not resizing. There are no changes to the number of compute nodes in the compute in progress. A compute enters this state when it is created and when no operations are being performed on the compute to change the number of compute nodes. resizing - Indicates that the compute is resizing; that is, compute nodes are being added to or removed from the compute."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AllocationState")]
    pub enum AllocationState {
        Steady,
        Resizing,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AllocationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AllocationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AllocationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Steady => serializer.serialize_unit_variant("AllocationState", 0u32, "Steady"),
                Self::Resizing => serializer.serialize_unit_variant("AllocationState", 1u32, "Resizing"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties(top level) of AmlCompute"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmlComputeSchema {
    #[doc = "AML Compute properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmlComputeProperties>,
}
impl AmlComputeSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Machine Learning REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmlOperation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<aml_operation::Display>,
    #[doc = "Indicates whether the operation applies to data-plane"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl AmlOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod aml_operation {
    use super::*;
    #[doc = "Display name of operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The resource provider name: Microsoft.MachineLearningExperimentation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The resource on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The operation that users can perform."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The description for the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "An array of operations supported by the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmlOperationListResult {
    #[doc = "List of AML operations supported by the AML resource provider."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AmlOperation>,
}
impl azure_core::Continuable for AmlOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AmlOperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AML Token identity configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmlToken {
    #[serde(flatten)]
    pub identity_configuration: IdentityConfiguration,
}
impl AmlToken {
    pub fn new(identity_configuration: IdentityConfiguration) -> Self {
        Self { identity_configuration }
    }
}
#[doc = "Features enabled for a workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmlUserFeature {
    #[doc = "Specifies the feature ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Specifies the feature name "]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Describes the feature for user experience"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AmlUserFeature {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ARM ResourceId of a resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmResourceId {
    #[doc = "Arm ResourceId is in the format \"/subscriptions/{SubscriptionId}/resourceGroups/{ResourceGroupName}/providers/Microsoft.Storage/storageAccounts/{StorageAccountName}\"\r\nor \"/subscriptions/{SubscriptionId}/resourceGroups/{ResourceGroupName}/providers/Microsoft.ContainerRegistry/registries/{AcrName}\""]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl ArmResourceId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetBase {
    #[serde(flatten)]
    pub resource_base: ResourceBase,
    #[serde(rename = "autoDeleteSetting", default, skip_serializing_if = "Option::is_none")]
    pub auto_delete_setting: Option<AutoDeleteSetting>,
    #[doc = "If the name version are system generated (anonymous registration). For types where Stage is defined, when Stage is provided it will be used to populate IsAnonymous"]
    #[serde(rename = "isAnonymous", default, skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[doc = "Is the asset archived? For types where Stage is defined, when Stage is provided it will be used to populate IsArchived"]
    #[serde(rename = "isArchived", default, skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
}
impl AssetBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetContainer {
    #[serde(flatten)]
    pub resource_base: ResourceBase,
    #[doc = "Is the asset archived?"]
    #[serde(rename = "isArchived", default, skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[doc = "The latest version inside this container."]
    #[serde(rename = "latestVersion", default, skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[doc = "The next auto incremental version"]
    #[serde(rename = "nextVersion", default, skip_serializing_if = "Option::is_none")]
    pub next_version: Option<String>,
}
impl AssetContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Asset input type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetJobInput {
    #[doc = "Enum to determine the input data delivery mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<InputDeliveryMode>,
    #[doc = "[Required] Input Asset URI."]
    pub uri: String,
}
impl AssetJobInput {
    pub fn new(uri: String) -> Self {
        Self { mode: None, uri }
    }
}
#[doc = "Asset output type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetJobOutput {
    #[doc = "Output Asset Name."]
    #[serde(rename = "assetName", default, skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    #[doc = "Output Asset Version."]
    #[serde(rename = "assetVersion", default, skip_serializing_if = "Option::is_none")]
    pub asset_version: Option<String>,
    #[serde(rename = "autoDeleteSetting", default, skip_serializing_if = "Option::is_none")]
    pub auto_delete_setting: Option<AutoDeleteSetting>,
    #[doc = "Output data delivery mode enums."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<OutputDeliveryMode>,
    #[doc = "Output Asset URI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl AssetJobOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provisioning state of registry asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AssetProvisioningState")]
pub enum AssetProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Creating,
    Updating,
    Deleting,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AssetProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AssetProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AssetProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("AssetProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("AssetProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("AssetProvisioningState", 2u32, "Canceled"),
            Self::Creating => serializer.serialize_unit_variant("AssetProvisioningState", 3u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("AssetProvisioningState", 4u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("AssetProvisioningState", 5u32, "Deleting"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Base definition for asset references."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetReferenceBase {
    #[doc = "Enum to determine which reference method to use for an asset."]
    #[serde(rename = "referenceType")]
    pub reference_type: ReferenceType,
}
impl AssetReferenceBase {
    pub fn new(reference_type: ReferenceType) -> Self {
        Self { reference_type }
    }
}
#[doc = "A user that can be assigned to a compute instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignedUser {
    #[doc = "User’s AAD Object Id."]
    #[serde(rename = "objectId")]
    pub object_id: String,
    #[doc = "User’s AAD Tenant Id."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}
impl AssignedUser {
    pub fn new(object_id: String, tenant_id: String) -> Self {
        Self { object_id, tenant_id }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutoDeleteCondition")]
pub enum AutoDeleteCondition {
    CreatedGreaterThan,
    LastAccessedGreaterThan,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutoDeleteCondition {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutoDeleteCondition {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutoDeleteCondition {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CreatedGreaterThan => serializer.serialize_unit_variant("AutoDeleteCondition", 0u32, "CreatedGreaterThan"),
            Self::LastAccessedGreaterThan => serializer.serialize_unit_variant("AutoDeleteCondition", 1u32, "LastAccessedGreaterThan"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoDeleteSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<AutoDeleteCondition>,
    #[doc = "Expiration condition value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl AutoDeleteSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Forecast horizon determined automatically by system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoForecastHorizon {
    #[serde(flatten)]
    pub forecast_horizon: ForecastHorizon,
}
impl AutoForecastHorizon {
    pub fn new(forecast_horizon: ForecastHorizon) -> Self {
        Self { forecast_horizon }
    }
}
#[doc = "AutoMLJob class.\r\nUse this class for executing AutoML tasks like Classification/Regression etc.\r\nSee TaskType enum for all the tasks supported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoMlJob {
    #[serde(flatten)]
    pub job_base: JobBase,
    #[doc = "The ARM resource ID of the Environment specification for the job.\r\nThis is optional value to provide, if not provided, AutoML will default this to Production AutoML curated environment version when running the job."]
    #[serde(rename = "environmentId", default, skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[doc = "Environment variables included in the job."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[doc = "Mapping of output data bindings used in the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[serde(rename = "queueSettings", default, skip_serializing_if = "Option::is_none")]
    pub queue_settings: Option<QueueSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<JobResourceConfiguration>,
    #[doc = "AutoML vertical class.\r\nBase class for AutoML verticals - TableVertical/ImageVertical/NLPVertical"]
    #[serde(rename = "taskDetails")]
    pub task_details: AutoMlVertical,
}
impl AutoMlJob {
    pub fn new(job_base: JobBase, task_details: AutoMlVertical) -> Self {
        Self {
            job_base,
            environment_id: None,
            environment_variables: None,
            outputs: None,
            queue_settings: None,
            resources: None,
            task_details,
        }
    }
}
#[doc = "AutoML vertical class.\r\nBase class for AutoML verticals - TableVertical/ImageVertical/NLPVertical"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoMlVertical {
    #[doc = "Enum for setting log verbosity."]
    #[serde(rename = "logVerbosity", default, skip_serializing_if = "Option::is_none")]
    pub log_verbosity: Option<LogVerbosity>,
    #[doc = "Target column name: This is prediction values column.\r\nAlso known as label column name in context of classification tasks."]
    #[serde(rename = "targetColumnName", default, skip_serializing_if = "Option::is_none")]
    pub target_column_name: Option<String>,
    #[doc = "AutoMLJob Task type."]
    #[serde(rename = "taskType")]
    pub task_type: TaskType,
    #[serde(rename = "trainingData")]
    pub training_data: MlTableJobInput,
}
impl AutoMlVertical {
    pub fn new(task_type: TaskType, training_data: MlTableJobInput) -> Self {
        Self {
            log_verbosity: None,
            target_column_name: None,
            task_type,
            training_data,
        }
    }
}
#[doc = "N-Cross validations determined automatically."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoNCrossValidations {
    #[serde(flatten)]
    pub n_cross_validations: NCrossValidations,
}
impl AutoNCrossValidations {
    pub fn new(n_cross_validations: NCrossValidations) -> Self {
        Self { n_cross_validations }
    }
}
#[doc = "Auto pause properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoPauseProperties {
    #[serde(rename = "delayInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub delay_in_minutes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl AutoPauseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AutoRebuild setting for the derived image"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutoRebuildSetting")]
pub enum AutoRebuildSetting {
    Disabled,
    OnBaseImageUpdate,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutoRebuildSetting {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutoRebuildSetting {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutoRebuildSetting {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("AutoRebuildSetting", 0u32, "Disabled"),
            Self::OnBaseImageUpdate => serializer.serialize_unit_variant("AutoRebuildSetting", 1u32, "OnBaseImageUpdate"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Auto scale properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoScaleProperties {
    #[serde(rename = "minNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub min_node_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "maxNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub max_node_count: Option<i32>,
}
impl AutoScaleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoSeasonality {
    #[serde(flatten)]
    pub seasonality: Seasonality,
}
impl AutoSeasonality {
    pub fn new(seasonality: Seasonality) -> Self {
        Self { seasonality }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoTargetLags {
    #[serde(flatten)]
    pub target_lags: TargetLags,
}
impl AutoTargetLags {
    pub fn new(target_lags: TargetLags) -> Self {
        Self { target_lags }
    }
}
#[doc = "Target lags rolling window determined automatically."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoTargetRollingWindowSize {
    #[serde(flatten)]
    pub target_rolling_window_size: TargetRollingWindowSize,
}
impl AutoTargetRollingWindowSize {
    pub fn new(target_rolling_window_size: TargetRollingWindowSize) -> Self {
        Self {
            target_rolling_window_size,
        }
    }
}
#[doc = "Settings for Autologger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutologgerSettings {
    #[doc = "Enum to determine the state of mlflow autologger."]
    #[serde(rename = "mlflowAutologger")]
    pub mlflow_autologger: MlFlowAutologgerState,
}
impl AutologgerSettings {
    pub fn new(mlflow_autologger: MlFlowAutologgerState) -> Self {
        Self { mlflow_autologger }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzMonMonitoringAlertNotificationSettings {
    #[serde(flatten)]
    pub monitoring_alert_notification_settings_base: MonitoringAlertNotificationSettingsBase,
}
impl AzMonMonitoringAlertNotificationSettings {
    pub fn new(monitoring_alert_notification_settings_base: MonitoringAlertNotificationSettingsBase) -> Self {
        Self {
            monitoring_alert_notification_settings_base,
        }
    }
}
#[doc = "Azure Blob datastore configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBlobDatastore {
    #[serde(flatten)]
    pub azure_datastore: AzureDatastore,
    #[serde(flatten)]
    pub datastore: Datastore,
    #[doc = "Storage account name."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "Storage account container name."]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "Azure cloud endpoint for the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "Protocol used to communicate with the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "serviceDataAccessAuthIdentity", default, skip_serializing_if = "Option::is_none")]
    pub service_data_access_auth_identity: Option<ServiceDataAccessAuthIdentity>,
}
impl AzureBlobDatastore {
    pub fn new(datastore: Datastore) -> Self {
        Self {
            azure_datastore: AzureDatastore::default(),
            datastore,
            account_name: None,
            container_name: None,
            endpoint: None,
            protocol: None,
            service_data_access_auth_identity: None,
        }
    }
}
#[doc = "Azure Data Lake Gen1 datastore configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDataLakeGen1Datastore {
    #[serde(flatten)]
    pub azure_datastore: AzureDatastore,
    #[serde(flatten)]
    pub datastore: Datastore,
    #[serde(rename = "serviceDataAccessAuthIdentity", default, skip_serializing_if = "Option::is_none")]
    pub service_data_access_auth_identity: Option<ServiceDataAccessAuthIdentity>,
    #[doc = "[Required] Azure Data Lake store name."]
    #[serde(rename = "storeName")]
    pub store_name: String,
}
impl AzureDataLakeGen1Datastore {
    pub fn new(datastore: Datastore, store_name: String) -> Self {
        Self {
            azure_datastore: AzureDatastore::default(),
            datastore,
            service_data_access_auth_identity: None,
            store_name,
        }
    }
}
#[doc = "Azure Data Lake Gen2 datastore configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDataLakeGen2Datastore {
    #[serde(flatten)]
    pub azure_datastore: AzureDatastore,
    #[serde(flatten)]
    pub datastore: Datastore,
    #[doc = "[Required] Storage account name."]
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[doc = "Azure cloud endpoint for the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "[Required] The name of the Data Lake Gen2 filesystem."]
    pub filesystem: String,
    #[doc = "Protocol used to communicate with the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "serviceDataAccessAuthIdentity", default, skip_serializing_if = "Option::is_none")]
    pub service_data_access_auth_identity: Option<ServiceDataAccessAuthIdentity>,
}
impl AzureDataLakeGen2Datastore {
    pub fn new(datastore: Datastore, account_name: String, filesystem: String) -> Self {
        Self {
            azure_datastore: AzureDatastore::default(),
            datastore,
            account_name,
            endpoint: None,
            filesystem,
            protocol: None,
            service_data_access_auth_identity: None,
        }
    }
}
#[doc = "Base definition for Azure datastore contents configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDatastore {
    #[doc = "Azure Resource Group name"]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "Azure Subscription Id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
impl AzureDatastore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Webhook details specific for Azure DevOps"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDevOpsWebhook {
    #[serde(flatten)]
    pub webhook: Webhook,
}
impl AzureDevOpsWebhook {
    pub fn new(webhook: Webhook) -> Self {
        Self { webhook }
    }
}
#[doc = "Azure File datastore configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileDatastore {
    #[serde(flatten)]
    pub azure_datastore: AzureDatastore,
    #[serde(flatten)]
    pub datastore: Datastore,
    #[doc = "[Required] Storage account name."]
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[doc = "Azure cloud endpoint for the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "[Required] The name of the Azure file share that the datastore points to."]
    #[serde(rename = "fileShareName")]
    pub file_share_name: String,
    #[doc = "Protocol used to communicate with the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "serviceDataAccessAuthIdentity", default, skip_serializing_if = "Option::is_none")]
    pub service_data_access_auth_identity: Option<ServiceDataAccessAuthIdentity>,
}
impl AzureFileDatastore {
    pub fn new(datastore: Datastore, account_name: String, file_share_name: String) -> Self {
        Self {
            azure_datastore: AzureDatastore::default(),
            datastore,
            account_name,
            endpoint: None,
            file_share_name,
            protocol: None,
            service_data_access_auth_identity: None,
        }
    }
}
#[doc = "Azure ML batch inferencing server configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMlBatchInferencingServer {
    #[serde(flatten)]
    pub inferencing_server: InferencingServer,
    #[doc = "Configuration for a scoring code asset."]
    #[serde(rename = "codeConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub code_configuration: Option<CodeConfiguration>,
}
impl AzureMlBatchInferencingServer {
    pub fn new(inferencing_server: InferencingServer) -> Self {
        Self {
            inferencing_server,
            code_configuration: None,
        }
    }
}
#[doc = "Azure ML online inferencing configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMlOnlineInferencingServer {
    #[serde(flatten)]
    pub inferencing_server: InferencingServer,
    #[doc = "Configuration for a scoring code asset."]
    #[serde(rename = "codeConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub code_configuration: Option<CodeConfiguration>,
}
impl AzureMlOnlineInferencingServer {
    pub fn new(inferencing_server: InferencingServer) -> Self {
        Self {
            inferencing_server,
            code_configuration: None,
        }
    }
}
#[doc = "Defines an early termination policy based on slack criteria, and a frequency and delay interval for evaluation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BanditPolicy {
    #[serde(flatten)]
    pub early_termination_policy: EarlyTerminationPolicy,
    #[doc = "Absolute distance allowed from the best performing run."]
    #[serde(rename = "slackAmount", default, skip_serializing_if = "Option::is_none")]
    pub slack_amount: Option<f32>,
    #[doc = "Ratio of the allowed distance from the best performing run."]
    #[serde(rename = "slackFactor", default, skip_serializing_if = "Option::is_none")]
    pub slack_factor: Option<f32>,
}
impl BanditPolicy {
    pub fn new(early_termination_policy: EarlyTerminationPolicy) -> Self {
        Self {
            early_termination_policy,
            slack_amount: None,
            slack_factor: None,
        }
    }
}
#[doc = "Base environment type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseEnvironmentId {
    #[serde(flatten)]
    pub base_environment_source: BaseEnvironmentSource,
    #[doc = "[Required] Resource id accepting ArmId or AzureMlId."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
}
impl BaseEnvironmentId {
    pub fn new(base_environment_source: BaseEnvironmentSource, resource_id: String) -> Self {
        Self {
            base_environment_source,
            resource_id,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseEnvironmentSource {
    #[doc = "Base environment type."]
    #[serde(rename = "baseEnvironmentSourceType")]
    pub base_environment_source_type: BaseEnvironmentSourceType,
}
impl BaseEnvironmentSource {
    pub fn new(base_environment_source_type: BaseEnvironmentSourceType) -> Self {
        Self {
            base_environment_source_type,
        }
    }
}
#[doc = "Base environment type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BaseEnvironmentSourceType")]
pub enum BaseEnvironmentSourceType {
    EnvironmentAsset,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BaseEnvironmentSourceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BaseEnvironmentSourceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BaseEnvironmentSourceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EnvironmentAsset => serializer.serialize_unit_variant("BaseEnvironmentSourceType", 0u32, "EnvironmentAsset"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Batch inference settings per deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchDeployment {
    #[serde(flatten)]
    pub endpoint_deployment_properties_base: EndpointDeploymentPropertiesBase,
    #[doc = "Compute target for batch inference operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compute: Option<String>,
    #[doc = "Properties relevant to different deployment types."]
    #[serde(rename = "deploymentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<BatchDeploymentConfiguration>,
    #[doc = "Error threshold, if the error count for the entire input goes above this value,\r\nthe batch inference will be aborted. Range is [-1, int.MaxValue].\r\nFor FileDataset, this value is the count of file failures.\r\nFor TabularDataset, this value is the count of record failures.\r\nIf set to -1 (the lower bound), all failures during batch inference will be ignored."]
    #[serde(rename = "errorThreshold", default, skip_serializing_if = "Option::is_none")]
    pub error_threshold: Option<i32>,
    #[doc = "Log verbosity for batch inferencing.\r\nIncreasing verbosity order for logging is : Warning, Info and Debug.\r\nThe default value is Info."]
    #[serde(rename = "loggingLevel", default, skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<BatchLoggingLevel>,
    #[doc = "Indicates maximum number of parallelism per instance."]
    #[serde(rename = "maxConcurrencyPerInstance", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrency_per_instance: Option<i32>,
    #[doc = "Size of the mini-batch passed to each batch invocation.\r\nFor FileDataset, this is the number of files per mini-batch.\r\nFor TabularDataset, this is the size of the records in bytes, per mini-batch."]
    #[serde(rename = "miniBatchSize", default, skip_serializing_if = "Option::is_none")]
    pub mini_batch_size: Option<i64>,
    #[doc = "Base definition for asset references."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<AssetReferenceBase>,
    #[doc = "Enum to determine how batch inferencing will handle output"]
    #[serde(rename = "outputAction", default, skip_serializing_if = "Option::is_none")]
    pub output_action: Option<BatchOutputAction>,
    #[doc = "Customized output file name for append_row output action."]
    #[serde(rename = "outputFileName", default, skip_serializing_if = "Option::is_none")]
    pub output_file_name: Option<String>,
    #[doc = "Possible values for DeploymentProvisioningState."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DeploymentProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<DeploymentResourceConfiguration>,
    #[doc = "Retry settings for a batch inference operation."]
    #[serde(rename = "retrySettings", default, skip_serializing_if = "Option::is_none")]
    pub retry_settings: Option<BatchRetrySettings>,
}
impl BatchDeployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties relevant to different deployment types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchDeploymentConfiguration {
    #[doc = "The enumerated property types for batch deployments."]
    #[serde(rename = "deploymentConfigurationType")]
    pub deployment_configuration_type: BatchDeploymentConfigurationType,
}
impl BatchDeploymentConfiguration {
    pub fn new(deployment_configuration_type: BatchDeploymentConfigurationType) -> Self {
        Self {
            deployment_configuration_type,
        }
    }
}
#[doc = "The enumerated property types for batch deployments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BatchDeploymentConfigurationType")]
pub enum BatchDeploymentConfigurationType {
    Model,
    PipelineComponent,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BatchDeploymentConfigurationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BatchDeploymentConfigurationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BatchDeploymentConfigurationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Model => serializer.serialize_unit_variant("BatchDeploymentConfigurationType", 0u32, "Model"),
            Self::PipelineComponent => serializer.serialize_unit_variant("BatchDeploymentConfigurationType", 1u32, "PipelineComponent"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchDeploymentTrackedResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Metadata used by portal/tooling/etc to render different UX experiences for resources of the same type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Batch inference settings per deployment."]
    pub properties: BatchDeployment,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl BatchDeploymentTrackedResource {
    pub fn new(tracked_resource: TrackedResource, properties: BatchDeployment) -> Self {
        Self {
            tracked_resource,
            identity: None,
            kind: None,
            properties,
            sku: None,
        }
    }
}
#[doc = "A paginated list of BatchDeployment entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchDeploymentTrackedResourceArmPaginatedResult {
    #[doc = "The link to the next page of BatchDeployment objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type BatchDeployment."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BatchDeploymentTrackedResource>,
}
impl azure_core::Continuable for BatchDeploymentTrackedResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BatchDeploymentTrackedResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Batch endpoint configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchEndpoint {
    #[serde(flatten)]
    pub endpoint_properties_base: EndpointPropertiesBase,
    #[doc = "Batch endpoint default values"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<BatchEndpointDefaults>,
    #[doc = "State of endpoint provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<EndpointProvisioningState>,
}
impl BatchEndpoint {
    pub fn new(endpoint_properties_base: EndpointPropertiesBase) -> Self {
        Self {
            endpoint_properties_base,
            defaults: None,
            provisioning_state: None,
        }
    }
}
#[doc = "Batch endpoint default values"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchEndpointDefaults {
    #[doc = "Name of the deployment that will be default for the endpoint.\r\nThis deployment will end up getting 100% traffic when the endpoint scoring URL is invoked."]
    #[serde(rename = "deploymentName", default, skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<String>,
}
impl BatchEndpointDefaults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchEndpointTrackedResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Metadata used by portal/tooling/etc to render different UX experiences for resources of the same type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Batch endpoint configuration."]
    pub properties: BatchEndpoint,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl BatchEndpointTrackedResource {
    pub fn new(tracked_resource: TrackedResource, properties: BatchEndpoint) -> Self {
        Self {
            tracked_resource,
            identity: None,
            kind: None,
            properties,
            sku: None,
        }
    }
}
#[doc = "A paginated list of BatchEndpoint entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchEndpointTrackedResourceArmPaginatedResult {
    #[doc = "The link to the next page of BatchEndpoint objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type BatchEndpoint."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BatchEndpointTrackedResource>,
}
impl azure_core::Continuable for BatchEndpointTrackedResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BatchEndpointTrackedResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Log verbosity for batch inferencing.\r\nIncreasing verbosity order for logging is : Warning, Info and Debug.\r\nThe default value is Info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BatchLoggingLevel")]
pub enum BatchLoggingLevel {
    Info,
    Warning,
    Debug,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BatchLoggingLevel {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BatchLoggingLevel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BatchLoggingLevel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Info => serializer.serialize_unit_variant("BatchLoggingLevel", 0u32, "Info"),
            Self::Warning => serializer.serialize_unit_variant("BatchLoggingLevel", 1u32, "Warning"),
            Self::Debug => serializer.serialize_unit_variant("BatchLoggingLevel", 2u32, "Debug"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enum to determine how batch inferencing will handle output"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BatchOutputAction")]
pub enum BatchOutputAction {
    SummaryOnly,
    AppendRow,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BatchOutputAction {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BatchOutputAction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BatchOutputAction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SummaryOnly => serializer.serialize_unit_variant("BatchOutputAction", 0u32, "SummaryOnly"),
            Self::AppendRow => serializer.serialize_unit_variant("BatchOutputAction", 1u32, "AppendRow"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties for a Batch Pipeline Component Deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchPipelineComponentDeploymentConfiguration {
    #[serde(flatten)]
    pub batch_deployment_configuration: BatchDeploymentConfiguration,
    #[doc = "Reference to an asset via its ARM resource ID."]
    #[serde(rename = "componentId", default, skip_serializing_if = "Option::is_none")]
    pub component_id: Option<IdAssetReference>,
    #[doc = "The description which will be applied to the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Run-time settings for the pipeline job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[doc = "The tags which will be applied to the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl BatchPipelineComponentDeploymentConfiguration {
    pub fn new(batch_deployment_configuration: BatchDeploymentConfiguration) -> Self {
        Self {
            batch_deployment_configuration,
            component_id: None,
            description: None,
            settings: None,
            tags: None,
        }
    }
}
#[doc = "Retry settings for a batch inference operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchRetrySettings {
    #[doc = "Maximum retry count for a mini-batch"]
    #[serde(rename = "maxRetries", default, skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[doc = "Invocation timeout for a mini-batch, in ISO 8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}
impl BatchRetrySettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a Sampling Algorithm that generates values based on previous values"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BayesianSamplingAlgorithm {
    #[serde(flatten)]
    pub sampling_algorithm: SamplingAlgorithm,
}
impl BayesianSamplingAlgorithm {
    pub fn new(sampling_algorithm: SamplingAlgorithm) -> Self {
        Self { sampling_algorithm }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BindOptions {
    #[doc = "Type of Bind Option"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub propagation: Option<String>,
    #[doc = "Indicate whether to create host path."]
    #[serde(rename = "createHostPath", default, skip_serializing_if = "Option::is_none")]
    pub create_host_path: Option<bool>,
    #[doc = "Mention the selinux options."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selinux: Option<String>,
}
impl BindOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobReferenceForConsumptionDto {
    #[doc = "Blob URI path for client to upload data.\r\nExample: https://blob.windows.core.net/Container/Path"]
    #[serde(rename = "blobUri", default, skip_serializing_if = "Option::is_none")]
    pub blob_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<PendingUploadCredentialDto>,
    #[doc = "Arm ID of the storage account to use"]
    #[serde(rename = "storageAccountArmId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_arm_id: Option<String>,
}
impl BlobReferenceForConsumptionDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enum for all classification models supported by AutoML."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BlockedTransformers")]
pub enum BlockedTransformers {
    TextTargetEncoder,
    OneHotEncoder,
    CatTargetEncoder,
    TfIdf,
    WoETargetEncoder,
    LabelEncoder,
    WordEmbedding,
    NaiveBayes,
    CountVectorizer,
    HashOneHotEncoder,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BlockedTransformers {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BlockedTransformers {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BlockedTransformers {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::TextTargetEncoder => serializer.serialize_unit_variant("BlockedTransformers", 0u32, "TextTargetEncoder"),
            Self::OneHotEncoder => serializer.serialize_unit_variant("BlockedTransformers", 1u32, "OneHotEncoder"),
            Self::CatTargetEncoder => serializer.serialize_unit_variant("BlockedTransformers", 2u32, "CatTargetEncoder"),
            Self::TfIdf => serializer.serialize_unit_variant("BlockedTransformers", 3u32, "TfIdf"),
            Self::WoETargetEncoder => serializer.serialize_unit_variant("BlockedTransformers", 4u32, "WoETargetEncoder"),
            Self::LabelEncoder => serializer.serialize_unit_variant("BlockedTransformers", 5u32, "LabelEncoder"),
            Self::WordEmbedding => serializer.serialize_unit_variant("BlockedTransformers", 6u32, "WordEmbedding"),
            Self::NaiveBayes => serializer.serialize_unit_variant("BlockedTransformers", 7u32, "NaiveBayes"),
            Self::CountVectorizer => serializer.serialize_unit_variant("BlockedTransformers", 8u32, "CountVectorizer"),
            Self::HashOneHotEncoder => serializer.serialize_unit_variant("BlockedTransformers", 9u32, "HashOneHotEncoder"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Configuration settings for Docker build context"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildContext {
    #[doc = "[Required] URI of the Docker build context used to build the image. Supports blob URIs on environment creation and may return blob or Git URIs.\r\n<seealso href=\"https://docs.docker.com/engine/reference/commandline/build/#extended-description\" />"]
    #[serde(rename = "contextUri")]
    pub context_uri: String,
    #[doc = "Path to the Dockerfile in the build context.\r\n<seealso href=\"https://docs.docker.com/engine/reference/builder/\" />"]
    #[serde(rename = "dockerfilePath", default, skip_serializing_if = "Option::is_none")]
    pub dockerfile_path: Option<String>,
}
impl BuildContext {
    pub fn new(context_uri: String) -> Self {
        Self {
            context_uri,
            dockerfile_path: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CategoricalDataDriftMetric")]
pub enum CategoricalDataDriftMetric {
    JensenShannonDistance,
    PopulationStabilityIndex,
    PearsonsChiSquaredTest,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CategoricalDataDriftMetric {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CategoricalDataDriftMetric {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CategoricalDataDriftMetric {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::JensenShannonDistance => serializer.serialize_unit_variant("CategoricalDataDriftMetric", 0u32, "JensenShannonDistance"),
            Self::PopulationStabilityIndex => {
                serializer.serialize_unit_variant("CategoricalDataDriftMetric", 1u32, "PopulationStabilityIndex")
            }
            Self::PearsonsChiSquaredTest => serializer.serialize_unit_variant("CategoricalDataDriftMetric", 2u32, "PearsonsChiSquaredTest"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CategoricalDataDriftMetricThreshold {
    #[serde(flatten)]
    pub data_drift_metric_threshold_base: DataDriftMetricThresholdBase,
    pub metric: CategoricalDataDriftMetric,
}
impl CategoricalDataDriftMetricThreshold {
    pub fn new(data_drift_metric_threshold_base: DataDriftMetricThresholdBase, metric: CategoricalDataDriftMetric) -> Self {
        Self {
            data_drift_metric_threshold_base,
            metric,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CategoricalDataQualityMetric")]
pub enum CategoricalDataQualityMetric {
    NullValueRate,
    DataTypeErrorRate,
    OutOfBoundsRate,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CategoricalDataQualityMetric {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CategoricalDataQualityMetric {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CategoricalDataQualityMetric {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NullValueRate => serializer.serialize_unit_variant("CategoricalDataQualityMetric", 0u32, "NullValueRate"),
            Self::DataTypeErrorRate => serializer.serialize_unit_variant("CategoricalDataQualityMetric", 1u32, "DataTypeErrorRate"),
            Self::OutOfBoundsRate => serializer.serialize_unit_variant("CategoricalDataQualityMetric", 2u32, "OutOfBoundsRate"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CategoricalDataQualityMetricThreshold {
    #[serde(flatten)]
    pub data_quality_metric_threshold_base: DataQualityMetricThresholdBase,
    pub metric: CategoricalDataQualityMetric,
}
impl CategoricalDataQualityMetricThreshold {
    pub fn new(data_quality_metric_threshold_base: DataQualityMetricThresholdBase, metric: CategoricalDataQualityMetric) -> Self {
        Self {
            data_quality_metric_threshold_base,
            metric,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CategoricalPredictionDriftMetric")]
pub enum CategoricalPredictionDriftMetric {
    JensenShannonDistance,
    PopulationStabilityIndex,
    PearsonsChiSquaredTest,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CategoricalPredictionDriftMetric {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CategoricalPredictionDriftMetric {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CategoricalPredictionDriftMetric {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::JensenShannonDistance => {
                serializer.serialize_unit_variant("CategoricalPredictionDriftMetric", 0u32, "JensenShannonDistance")
            }
            Self::PopulationStabilityIndex => {
                serializer.serialize_unit_variant("CategoricalPredictionDriftMetric", 1u32, "PopulationStabilityIndex")
            }
            Self::PearsonsChiSquaredTest => {
                serializer.serialize_unit_variant("CategoricalPredictionDriftMetric", 2u32, "PearsonsChiSquaredTest")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CategoricalPredictionDriftMetricThreshold {
    #[serde(flatten)]
    pub prediction_drift_metric_threshold_base: PredictionDriftMetricThresholdBase,
    pub metric: CategoricalPredictionDriftMetric,
}
impl CategoricalPredictionDriftMetricThreshold {
    pub fn new(
        prediction_drift_metric_threshold_base: PredictionDriftMetricThresholdBase,
        metric: CategoricalPredictionDriftMetric,
    ) -> Self {
        Self {
            prediction_drift_metric_threshold_base,
            metric,
        }
    }
}
#[doc = "Certificate datastore credentials configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateDatastoreCredentials {
    #[serde(flatten)]
    pub datastore_credentials: DatastoreCredentials,
    #[doc = "Authority URL used for authentication."]
    #[serde(rename = "authorityUrl", default, skip_serializing_if = "Option::is_none")]
    pub authority_url: Option<String>,
    #[doc = "[Required] Service principal client ID."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "Resource the service principal has access to."]
    #[serde(rename = "resourceUrl", default, skip_serializing_if = "Option::is_none")]
    pub resource_url: Option<String>,
    #[doc = "Datastore certificate secrets."]
    pub secrets: CertificateDatastoreSecrets,
    #[doc = "[Required] ID of the tenant to which the service principal belongs."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[doc = "[Required] Thumbprint of the certificate used for authentication."]
    pub thumbprint: String,
}
impl CertificateDatastoreCredentials {
    pub fn new(
        datastore_credentials: DatastoreCredentials,
        client_id: String,
        secrets: CertificateDatastoreSecrets,
        tenant_id: String,
        thumbprint: String,
    ) -> Self {
        Self {
            datastore_credentials,
            authority_url: None,
            client_id,
            resource_url: None,
            secrets,
            tenant_id,
            thumbprint,
        }
    }
}
#[doc = "Datastore certificate secrets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateDatastoreSecrets {
    #[serde(flatten)]
    pub datastore_secrets: DatastoreSecrets,
    #[doc = "Service principal certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}
impl CertificateDatastoreSecrets {
    pub fn new(datastore_secrets: DatastoreSecrets) -> Self {
        Self {
            datastore_secrets,
            certificate: None,
        }
    }
}
#[doc = "Classification task in AutoML Table vertical."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Classification {
    #[serde(flatten)]
    pub table_vertical: TableVertical,
    #[serde(flatten)]
    pub auto_ml_vertical: AutoMlVertical,
    #[doc = "Positive label for binary metrics calculation."]
    #[serde(rename = "positiveLabel", default, skip_serializing_if = "Option::is_none")]
    pub positive_label: Option<String>,
    #[doc = "Primary metrics for classification tasks."]
    #[serde(rename = "primaryMetric", default, skip_serializing_if = "Option::is_none")]
    pub primary_metric: Option<ClassificationPrimaryMetrics>,
    #[doc = "Classification Training related configuration."]
    #[serde(rename = "trainingSettings", default, skip_serializing_if = "Option::is_none")]
    pub training_settings: Option<ClassificationTrainingSettings>,
}
impl Classification {
    pub fn new(auto_ml_vertical: AutoMlVertical) -> Self {
        Self {
            table_vertical: TableVertical::default(),
            auto_ml_vertical,
            positive_label: None,
            primary_metric: None,
            training_settings: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ClassificationModelPerformanceMetric")]
pub enum ClassificationModelPerformanceMetric {
    Accuracy,
    Precision,
    Recall,
    F1Score,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ClassificationModelPerformanceMetric {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ClassificationModelPerformanceMetric {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ClassificationModelPerformanceMetric {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Accuracy => serializer.serialize_unit_variant("ClassificationModelPerformanceMetric", 0u32, "Accuracy"),
            Self::Precision => serializer.serialize_unit_variant("ClassificationModelPerformanceMetric", 1u32, "Precision"),
            Self::Recall => serializer.serialize_unit_variant("ClassificationModelPerformanceMetric", 2u32, "Recall"),
            Self::F1Score => serializer.serialize_unit_variant("ClassificationModelPerformanceMetric", 3u32, "F1Score"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassificationModelPerformanceMetricThreshold {
    #[serde(flatten)]
    pub model_performance_metric_threshold_base: ModelPerformanceMetricThresholdBase,
    pub metric: ClassificationModelPerformanceMetric,
}
impl ClassificationModelPerformanceMetricThreshold {
    pub fn new(
        model_performance_metric_threshold_base: ModelPerformanceMetricThresholdBase,
        metric: ClassificationModelPerformanceMetric,
    ) -> Self {
        Self {
            model_performance_metric_threshold_base,
            metric,
        }
    }
}
#[doc = "Enum for all classification models supported by AutoML."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ClassificationModels")]
pub enum ClassificationModels {
    LogisticRegression,
    #[serde(rename = "SGD")]
    Sgd,
    MultinomialNaiveBayes,
    BernoulliNaiveBayes,
    #[serde(rename = "SVM")]
    Svm,
    #[serde(rename = "LinearSVM")]
    LinearSvm,
    #[serde(rename = "KNN")]
    Knn,
    DecisionTree,
    RandomForest,
    ExtremeRandomTrees,
    #[serde(rename = "LightGBM")]
    LightGbm,
    GradientBoosting,
    #[serde(rename = "XGBoostClassifier")]
    XgBoostClassifier,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ClassificationModels {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ClassificationModels {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ClassificationModels {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::LogisticRegression => serializer.serialize_unit_variant("ClassificationModels", 0u32, "LogisticRegression"),
            Self::Sgd => serializer.serialize_unit_variant("ClassificationModels", 1u32, "SGD"),
            Self::MultinomialNaiveBayes => serializer.serialize_unit_variant("ClassificationModels", 2u32, "MultinomialNaiveBayes"),
            Self::BernoulliNaiveBayes => serializer.serialize_unit_variant("ClassificationModels", 3u32, "BernoulliNaiveBayes"),
            Self::Svm => serializer.serialize_unit_variant("ClassificationModels", 4u32, "SVM"),
            Self::LinearSvm => serializer.serialize_unit_variant("ClassificationModels", 5u32, "LinearSVM"),
            Self::Knn => serializer.serialize_unit_variant("ClassificationModels", 6u32, "KNN"),
            Self::DecisionTree => serializer.serialize_unit_variant("ClassificationModels", 7u32, "DecisionTree"),
            Self::RandomForest => serializer.serialize_unit_variant("ClassificationModels", 8u32, "RandomForest"),
            Self::ExtremeRandomTrees => serializer.serialize_unit_variant("ClassificationModels", 9u32, "ExtremeRandomTrees"),
            Self::LightGbm => serializer.serialize_unit_variant("ClassificationModels", 10u32, "LightGBM"),
            Self::GradientBoosting => serializer.serialize_unit_variant("ClassificationModels", 11u32, "GradientBoosting"),
            Self::XgBoostClassifier => serializer.serialize_unit_variant("ClassificationModels", 12u32, "XGBoostClassifier"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Primary metrics for classification multilabel tasks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ClassificationMultilabelPrimaryMetrics")]
pub enum ClassificationMultilabelPrimaryMetrics {
    #[serde(rename = "AUCWeighted")]
    AucWeighted,
    Accuracy,
    NormMacroRecall,
    AveragePrecisionScoreWeighted,
    PrecisionScoreWeighted,
    #[serde(rename = "IOU")]
    Iou,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ClassificationMultilabelPrimaryMetrics {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ClassificationMultilabelPrimaryMetrics {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ClassificationMultilabelPrimaryMetrics {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AucWeighted => serializer.serialize_unit_variant("ClassificationMultilabelPrimaryMetrics", 0u32, "AUCWeighted"),
            Self::Accuracy => serializer.serialize_unit_variant("ClassificationMultilabelPrimaryMetrics", 1u32, "Accuracy"),
            Self::NormMacroRecall => serializer.serialize_unit_variant("ClassificationMultilabelPrimaryMetrics", 2u32, "NormMacroRecall"),
            Self::AveragePrecisionScoreWeighted => {
                serializer.serialize_unit_variant("ClassificationMultilabelPrimaryMetrics", 3u32, "AveragePrecisionScoreWeighted")
            }
            Self::PrecisionScoreWeighted => {
                serializer.serialize_unit_variant("ClassificationMultilabelPrimaryMetrics", 4u32, "PrecisionScoreWeighted")
            }
            Self::Iou => serializer.serialize_unit_variant("ClassificationMultilabelPrimaryMetrics", 5u32, "IOU"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Primary metrics for classification tasks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ClassificationPrimaryMetrics")]
pub enum ClassificationPrimaryMetrics {
    #[serde(rename = "AUCWeighted")]
    AucWeighted,
    Accuracy,
    NormMacroRecall,
    AveragePrecisionScoreWeighted,
    PrecisionScoreWeighted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ClassificationPrimaryMetrics {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ClassificationPrimaryMetrics {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ClassificationPrimaryMetrics {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AucWeighted => serializer.serialize_unit_variant("ClassificationPrimaryMetrics", 0u32, "AUCWeighted"),
            Self::Accuracy => serializer.serialize_unit_variant("ClassificationPrimaryMetrics", 1u32, "Accuracy"),
            Self::NormMacroRecall => serializer.serialize_unit_variant("ClassificationPrimaryMetrics", 2u32, "NormMacroRecall"),
            Self::AveragePrecisionScoreWeighted => {
                serializer.serialize_unit_variant("ClassificationPrimaryMetrics", 3u32, "AveragePrecisionScoreWeighted")
            }
            Self::PrecisionScoreWeighted => {
                serializer.serialize_unit_variant("ClassificationPrimaryMetrics", 4u32, "PrecisionScoreWeighted")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Classification Training related configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassificationTrainingSettings {
    #[serde(flatten)]
    pub training_settings: TrainingSettings,
    #[doc = "Allowed models for classification task."]
    #[serde(
        rename = "allowedTrainingAlgorithms",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_training_algorithms: Vec<ClassificationModels>,
    #[doc = "Blocked models for classification task."]
    #[serde(
        rename = "blockedTrainingAlgorithms",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_training_algorithms: Vec<ClassificationModels>,
}
impl ClassificationTrainingSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AmlCompute update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpdateParameters {
    #[doc = "The properties of a amlCompute that need to be updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterUpdateProperties>,
}
impl ClusterUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a amlCompute that need to be updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpdateProperties {
    #[doc = "Desired scale settings for the amlCompute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScaleSettingsInformation>,
}
impl ClusterUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CocoExportSummary {
    #[serde(flatten)]
    pub export_summary: ExportSummary,
    #[doc = "The container name to which the labels will be exported."]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "The output path where the labels will be exported."]
    #[serde(rename = "snapshotPath", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_path: Option<String>,
}
impl CocoExportSummary {
    pub fn new(export_summary: ExportSummary) -> Self {
        Self {
            export_summary,
            container_name: None,
            snapshot_path: None,
        }
    }
}
#[doc = "Configuration for a scoring code asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeConfiguration {
    #[doc = "ARM resource ID of the code asset."]
    #[serde(rename = "codeId", default, skip_serializing_if = "Option::is_none")]
    pub code_id: Option<String>,
    #[doc = "[Required] The script to execute on startup. eg. \"score.py\""]
    #[serde(rename = "scoringScript")]
    pub scoring_script: String,
}
impl CodeConfiguration {
    pub fn new(scoring_script: String) -> Self {
        Self {
            code_id: None,
            scoring_script,
        }
    }
}
#[doc = "Container for code asset versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodeContainer {
    #[serde(flatten)]
    pub asset_container: AssetContainer,
    #[doc = "Provisioning state of registry asset."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AssetProvisioningState>,
}
impl CodeContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeContainerResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Container for code asset versions."]
    pub properties: CodeContainer,
}
impl CodeContainerResource {
    pub fn new(properties: CodeContainer) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of CodeContainer entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodeContainerResourceArmPaginatedResult {
    #[doc = "The link to the next page of CodeContainer objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type CodeContainer."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CodeContainerResource>,
}
impl azure_core::Continuable for CodeContainerResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CodeContainerResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Code asset version details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodeVersion {
    #[serde(flatten)]
    pub asset_base: AssetBase,
    #[doc = "Uri where code is located"]
    #[serde(rename = "codeUri", default, skip_serializing_if = "Option::is_none")]
    pub code_uri: Option<String>,
    #[doc = "Provisioning state of registry asset."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AssetProvisioningState>,
}
impl CodeVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeVersionResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Code asset version details."]
    pub properties: CodeVersion,
}
impl CodeVersionResource {
    pub fn new(properties: CodeVersion) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of CodeVersion entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodeVersionResourceArmPaginatedResult {
    #[doc = "The link to the next page of CodeVersion objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type CodeVersion."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CodeVersionResource>,
}
impl azure_core::Continuable for CodeVersionResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CodeVersionResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Collection {
    #[doc = "The msi client id used to collect logging to blob storage. If it's null,backend will pick a registered endpoint identity to auth."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "dataCollectionMode", default, skip_serializing_if = "Option::is_none")]
    pub data_collection_mode: Option<DataCollectionMode>,
    #[doc = "The data asset arm resource id. Client side will ensure data asset is pointing to the blob storage, and backend will collect data to the blob storage."]
    #[serde(rename = "dataId", default, skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    #[doc = "The sampling rate for collection. Sampling rate 1.0 means we collect 100% of data by default."]
    #[serde(rename = "samplingRate", default, skip_serializing_if = "Option::is_none")]
    pub sampling_rate: Option<f64>,
}
impl Collection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Column transformer parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ColumnTransformer {
    #[doc = "Fields to apply transformer logic on."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fields: Vec<String>,
    #[doc = "Different properties to be passed to transformer.\r\nInput expected is dictionary of key,value pairs in JSON format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl ColumnTransformer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Command job definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandJob {
    #[serde(flatten)]
    pub job_base: JobBase,
    #[doc = "Settings for Autologger."]
    #[serde(rename = "autologgerSettings", default, skip_serializing_if = "Option::is_none")]
    pub autologger_settings: Option<AutologgerSettings>,
    #[doc = "ARM resource ID of the code asset."]
    #[serde(rename = "codeId", default, skip_serializing_if = "Option::is_none")]
    pub code_id: Option<String>,
    #[doc = "[Required] The command to execute on startup of the job. eg. \"python train.py\""]
    pub command: String,
    #[doc = "Base definition for job distribution configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribution: Option<DistributionConfiguration>,
    #[doc = "[Required] The ARM resource ID of the Environment specification for the job."]
    #[serde(rename = "environmentId")]
    pub environment_id: String,
    #[doc = "Environment variables included in the job."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[doc = "Mapping of input data bindings used in the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[doc = "Command Job limit class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<CommandJobLimits>,
    #[doc = "Mapping of output data bindings used in the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[doc = "Input parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(rename = "queueSettings", default, skip_serializing_if = "Option::is_none")]
    pub queue_settings: Option<QueueSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<JobResourceConfiguration>,
}
impl CommandJob {
    pub fn new(job_base: JobBase, command: String, environment_id: String) -> Self {
        Self {
            job_base,
            autologger_settings: None,
            code_id: None,
            command,
            distribution: None,
            environment_id,
            environment_variables: None,
            inputs: None,
            limits: None,
            outputs: None,
            parameters: None,
            queue_settings: None,
            resources: None,
        }
    }
}
#[doc = "Command Job limit class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandJobLimits {
    #[serde(flatten)]
    pub job_limits: JobLimits,
}
impl CommandJobLimits {
    pub fn new(job_limits: JobLimits) -> Self {
        Self { job_limits }
    }
}
#[doc = "Component container definition.\r\n<see href=\"https://docs.microsoft.com/en-us/azure/machine-learning/reference-yaml-component-command\" />"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentContainer {
    #[serde(flatten)]
    pub asset_container: AssetContainer,
    #[doc = "Provisioning state of registry asset."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AssetProvisioningState>,
}
impl ComponentContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentContainerResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Component container definition.\r\n<see href=\"https://docs.microsoft.com/en-us/azure/machine-learning/reference-yaml-component-command\" />"]
    pub properties: ComponentContainer,
}
impl ComponentContainerResource {
    pub fn new(properties: ComponentContainer) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of ComponentContainer entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentContainerResourceArmPaginatedResult {
    #[doc = "The link to the next page of ComponentContainer objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type ComponentContainer."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ComponentContainerResource>,
}
impl azure_core::Continuable for ComponentContainerResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ComponentContainerResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of a component version: defines resources that span component types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentVersion {
    #[serde(flatten)]
    pub asset_base: AssetBase,
    #[doc = "Defines Component definition details.\r\n<see href=\"https://docs.microsoft.com/en-us/azure/machine-learning/reference-yaml-component-command\" />"]
    #[serde(rename = "componentSpec", default, skip_serializing_if = "Option::is_none")]
    pub component_spec: Option<serde_json::Value>,
    #[doc = "Provisioning state of registry asset."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AssetProvisioningState>,
    #[doc = "Stage in the component lifecycle"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}
impl ComponentVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentVersionResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Definition of a component version: defines resources that span component types."]
    pub properties: ComponentVersion,
}
impl ComponentVersionResource {
    pub fn new(properties: ComponentVersion) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of ComponentVersion entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentVersionResourceArmPaginatedResult {
    #[doc = "The link to the next page of ComponentVersion objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type ComponentVersion."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ComponentVersionResource>,
}
impl azure_core::Continuable for ComponentVersionResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ComponentVersionResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Machine Learning compute object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Compute {
    #[doc = "The type of compute"]
    #[serde(rename = "computeType")]
    pub compute_type: ComputeType,
    #[doc = "Location for the underlying compute"]
    #[serde(rename = "computeLocation", default, skip_serializing_if = "Option::is_none")]
    pub compute_location: Option<String>,
    #[doc = "The provision state of the cluster. Valid values are Unknown, Updating, Provisioning, Succeeded, and Failed."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<compute::ProvisioningState>,
    #[doc = "The description of the Machine Learning compute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The time at which the compute was created."]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "The time at which the compute was last modified."]
    #[serde(rename = "modifiedOn", default, with = "azure_core::date::rfc3339::option")]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "ARM resource id of the underlying compute"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Errors during provisioning"]
    #[serde(
        rename = "provisioningErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub provisioning_errors: Vec<ErrorResponse>,
    #[doc = "Indicating whether the compute was provisioned by user and brought from outside if true, or machine learning service provisioned it if false."]
    #[serde(rename = "isAttachedCompute", default, skip_serializing_if = "Option::is_none")]
    pub is_attached_compute: Option<bool>,
    #[doc = "Opt-out of local authentication and ensure customers can use only MSI and AAD exclusively for authentication."]
    #[serde(rename = "disableLocalAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_local_auth: Option<bool>,
}
impl Compute {
    pub fn new(compute_type: ComputeType) -> Self {
        Self {
            compute_type,
            compute_location: None,
            provisioning_state: None,
            description: None,
            created_on: None,
            modified_on: None,
            resource_id: None,
            provisioning_errors: Vec::new(),
            is_attached_compute: None,
            disable_local_auth: None,
        }
    }
}
pub mod compute {
    use super::*;
    #[doc = "The provision state of the cluster. Valid values are Unknown, Updating, Provisioning, Succeeded, and Failed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Unknown,
        Updating,
        Creating,
        Deleting,
        Succeeded,
        Failed,
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
                Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Unknown"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An Azure Machine Learning compute instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeInstance {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(flatten)]
    pub compute_instance_schema: ComputeInstanceSchema,
}
impl ComputeInstance {
    pub fn new(compute: Compute) -> Self {
        Self {
            compute,
            compute_instance_schema: ComputeInstanceSchema::default(),
        }
    }
}
#[doc = "Defines an Aml Instance application and its connectivity endpoint URI."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeInstanceApplication {
    #[doc = "Name of the ComputeInstance application."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Application' endpoint URI."]
    #[serde(rename = "endpointUri", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<String>,
}
impl ComputeInstanceApplication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies settings for autologger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeInstanceAutologgerSettings {
    #[doc = "Indicates whether mlflow autologger is enabled for notebooks."]
    #[serde(rename = "mlflowAutologger", default, skip_serializing_if = "Option::is_none")]
    pub mlflow_autologger: Option<compute_instance_autologger_settings::MlflowAutologger>,
}
impl ComputeInstanceAutologgerSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod compute_instance_autologger_settings {
    use super::*;
    #[doc = "Indicates whether mlflow autologger is enabled for notebooks."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MlflowAutologger")]
    pub enum MlflowAutologger {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MlflowAutologger {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MlflowAutologger {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MlflowAutologger {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("MlflowAutologger", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("MlflowAutologger", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines all connectivity endpoints and properties for an ComputeInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeInstanceConnectivityEndpoints {
    #[doc = "Public IP Address of this ComputeInstance."]
    #[serde(rename = "publicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    #[doc = "Private IP Address of this ComputeInstance (local to the VNET in which the compute instance is deployed)."]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
}
impl ComputeInstanceConnectivityEndpoints {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines an Aml Instance container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeInstanceContainer {
    #[doc = "Name of the ComputeInstance container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Auto save settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autosave: Option<compute_instance_container::Autosave>,
    #[doc = "Information of GPU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gpu: Option<String>,
    #[doc = "network of this container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<compute_instance_container::Network>,
    #[doc = "Environment information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<ComputeInstanceEnvironmentInfo>,
    #[doc = "services of this containers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub services: Vec<serde_json::Value>,
}
impl ComputeInstanceContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod compute_instance_container {
    use super::*;
    #[doc = "Auto save settings."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Autosave")]
    pub enum Autosave {
        None,
        Local,
        Remote,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Autosave {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Autosave {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Autosave {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Autosave", 0u32, "None"),
                Self::Local => serializer.serialize_unit_variant("Autosave", 1u32, "Local"),
                Self::Remote => serializer.serialize_unit_variant("Autosave", 2u32, "Remote"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "network of this container."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Network")]
    pub enum Network {
        Bridge,
        Host,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Network {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Network {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Network {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Bridge => serializer.serialize_unit_variant("Network", 0u32, "Bridge"),
                Self::Host => serializer.serialize_unit_variant("Network", 1u32, "Host"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes information on user who created this ComputeInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeInstanceCreatedBy {
    #[doc = "Name of the user."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "Uniquely identifies user' Azure Active Directory organization."]
    #[serde(rename = "userOrgId", default, skip_serializing_if = "Option::is_none")]
    pub user_org_id: Option<String>,
    #[doc = "Uniquely identifies the user within his/her organization."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}
impl ComputeInstanceCreatedBy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines an Aml Instance DataDisk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeInstanceDataDisk {
    #[doc = "Caching type of Data Disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<compute_instance_data_disk::Caching>,
    #[doc = "The initial disk size in gigabytes."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "The lun is used to uniquely identify each data disk. If attaching multiple disks, each should have a distinct lun."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    #[doc = "type of this storage account."]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<compute_instance_data_disk::StorageAccountType>,
}
impl ComputeInstanceDataDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod compute_instance_data_disk {
    use super::*;
    #[doc = "Caching type of Data Disk."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Caching")]
    pub enum Caching {
        None,
        ReadOnly,
        ReadWrite,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Caching {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Caching {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Caching {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Caching", 0u32, "None"),
                Self::ReadOnly => serializer.serialize_unit_variant("Caching", 1u32, "ReadOnly"),
                Self::ReadWrite => serializer.serialize_unit_variant("Caching", 2u32, "ReadWrite"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "type of this storage account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageAccountType")]
    pub enum StorageAccountType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageAccountType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageAccountType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageAccountType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardLrs => serializer.serialize_unit_variant("StorageAccountType", 0u32, "Standard_LRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("StorageAccountType", 1u32, "Premium_LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for StorageAccountType {
        fn default() -> Self {
            Self::StandardLrs
        }
    }
}
#[doc = "Defines an Aml Instance DataMount."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeInstanceDataMount {
    #[doc = "Source of the ComputeInstance data mount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Data source type."]
    #[serde(rename = "sourceType", default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<compute_instance_data_mount::SourceType>,
    #[doc = "name of the ComputeInstance data mount."]
    #[serde(rename = "mountName", default, skip_serializing_if = "Option::is_none")]
    pub mount_name: Option<String>,
    #[doc = "Mount Action."]
    #[serde(rename = "mountAction", default, skip_serializing_if = "Option::is_none")]
    pub mount_action: Option<compute_instance_data_mount::MountAction>,
    #[doc = "who this data mount created by."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Path of this data mount."]
    #[serde(rename = "mountPath", default, skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
    #[doc = "Mount state."]
    #[serde(rename = "mountState", default, skip_serializing_if = "Option::is_none")]
    pub mount_state: Option<compute_instance_data_mount::MountState>,
    #[doc = "The time when the disk mounted."]
    #[serde(rename = "mountedOn", default, with = "azure_core::date::rfc3339::option")]
    pub mounted_on: Option<time::OffsetDateTime>,
    #[doc = "Error of this data mount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
impl ComputeInstanceDataMount {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod compute_instance_data_mount {
    use super::*;
    #[doc = "Data source type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SourceType")]
    pub enum SourceType {
        Dataset,
        Datastore,
        #[serde(rename = "URI")]
        Uri,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SourceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SourceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SourceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dataset => serializer.serialize_unit_variant("SourceType", 0u32, "Dataset"),
                Self::Datastore => serializer.serialize_unit_variant("SourceType", 1u32, "Datastore"),
                Self::Uri => serializer.serialize_unit_variant("SourceType", 2u32, "URI"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Mount Action."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MountAction")]
    pub enum MountAction {
        Mount,
        Unmount,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MountAction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MountAction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MountAction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Mount => serializer.serialize_unit_variant("MountAction", 0u32, "Mount"),
                Self::Unmount => serializer.serialize_unit_variant("MountAction", 1u32, "Unmount"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Mount state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MountState")]
    pub enum MountState {
        MountRequested,
        Mounted,
        MountFailed,
        UnmountRequested,
        UnmountFailed,
        Unmounted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MountState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MountState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MountState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MountRequested => serializer.serialize_unit_variant("MountState", 0u32, "MountRequested"),
                Self::Mounted => serializer.serialize_unit_variant("MountState", 1u32, "Mounted"),
                Self::MountFailed => serializer.serialize_unit_variant("MountState", 2u32, "MountFailed"),
                Self::UnmountRequested => serializer.serialize_unit_variant("MountState", 3u32, "UnmountRequested"),
                Self::UnmountFailed => serializer.serialize_unit_variant("MountState", 4u32, "UnmountFailed"),
                Self::Unmounted => serializer.serialize_unit_variant("MountState", 5u32, "Unmounted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Environment information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeInstanceEnvironmentInfo {
    #[doc = "name of environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "version of environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ComputeInstanceEnvironmentInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The last operation on ComputeInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeInstanceLastOperation {
    #[doc = "Name of the last operation."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<compute_instance_last_operation::OperationName>,
    #[doc = "Time of the last operation."]
    #[serde(rename = "operationTime", default, with = "azure_core::date::rfc3339::option")]
    pub operation_time: Option<time::OffsetDateTime>,
    #[doc = "Operation status."]
    #[serde(rename = "operationStatus", default, skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<compute_instance_last_operation::OperationStatus>,
    #[doc = "Trigger of operation."]
    #[serde(rename = "operationTrigger", default, skip_serializing_if = "Option::is_none")]
    pub operation_trigger: Option<compute_instance_last_operation::OperationTrigger>,
}
impl ComputeInstanceLastOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod compute_instance_last_operation {
    use super::*;
    #[doc = "Name of the last operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationName")]
    pub enum OperationName {
        Create,
        Start,
        Stop,
        Restart,
        Reimage,
        Delete,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OperationName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OperationName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OperationName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Create => serializer.serialize_unit_variant("OperationName", 0u32, "Create"),
                Self::Start => serializer.serialize_unit_variant("OperationName", 1u32, "Start"),
                Self::Stop => serializer.serialize_unit_variant("OperationName", 2u32, "Stop"),
                Self::Restart => serializer.serialize_unit_variant("OperationName", 3u32, "Restart"),
                Self::Reimage => serializer.serialize_unit_variant("OperationName", 4u32, "Reimage"),
                Self::Delete => serializer.serialize_unit_variant("OperationName", 5u32, "Delete"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Operation status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationStatus")]
    pub enum OperationStatus {
        InProgress,
        Succeeded,
        CreateFailed,
        StartFailed,
        StopFailed,
        RestartFailed,
        ReimageFailed,
        DeleteFailed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OperationStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OperationStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OperationStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::InProgress => serializer.serialize_unit_variant("OperationStatus", 0u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("OperationStatus", 1u32, "Succeeded"),
                Self::CreateFailed => serializer.serialize_unit_variant("OperationStatus", 2u32, "CreateFailed"),
                Self::StartFailed => serializer.serialize_unit_variant("OperationStatus", 3u32, "StartFailed"),
                Self::StopFailed => serializer.serialize_unit_variant("OperationStatus", 4u32, "StopFailed"),
                Self::RestartFailed => serializer.serialize_unit_variant("OperationStatus", 5u32, "RestartFailed"),
                Self::ReimageFailed => serializer.serialize_unit_variant("OperationStatus", 6u32, "ReimageFailed"),
                Self::DeleteFailed => serializer.serialize_unit_variant("OperationStatus", 7u32, "DeleteFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Trigger of operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationTrigger")]
    pub enum OperationTrigger {
        User,
        Schedule,
        IdleShutdown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OperationTrigger {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OperationTrigger {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OperationTrigger {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("OperationTrigger", 0u32, "User"),
                Self::Schedule => serializer.serialize_unit_variant("OperationTrigger", 1u32, "Schedule"),
                Self::IdleShutdown => serializer.serialize_unit_variant("OperationTrigger", 2u32, "IdleShutdown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Compute Instance properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeInstanceProperties {
    #[doc = "Virtual Machine Size"]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[doc = "Represents a resource ID. For example, for a subnet, it is the resource URL for the subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<ResourceId>,
    #[doc = "Policy for sharing applications on this compute instance among users of parent workspace. If Personal, only the creator can access applications on this compute instance. When Shared, any workspace user can access applications on this instance depending on his/her assigned role."]
    #[serde(rename = "applicationSharingPolicy", default, skip_serializing_if = "Option::is_none")]
    pub application_sharing_policy: Option<compute_instance_properties::ApplicationSharingPolicy>,
    #[doc = "Specifies settings for autologger."]
    #[serde(rename = "autologgerSettings", default, skip_serializing_if = "Option::is_none")]
    pub autologger_settings: Option<ComputeInstanceAutologgerSettings>,
    #[doc = "Specifies policy and settings for SSH access."]
    #[serde(rename = "sshSettings", default, skip_serializing_if = "Option::is_none")]
    pub ssh_settings: Option<ComputeInstanceSshSettings>,
    #[doc = "List of Custom Services added to the compute."]
    #[serde(
        rename = "customServices",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_services: Vec<CustomService>,
    #[doc = "Returns metadata about the operating system image for this compute instance."]
    #[serde(rename = "osImageMetadata", default, skip_serializing_if = "Option::is_none")]
    pub os_image_metadata: Option<ImageMetadata>,
    #[doc = "Defines all connectivity endpoints and properties for an ComputeInstance."]
    #[serde(rename = "connectivityEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub connectivity_endpoints: Option<ComputeInstanceConnectivityEndpoints>,
    #[doc = "Describes available applications and their endpoints on this ComputeInstance."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub applications: Vec<ComputeInstanceApplication>,
    #[doc = "Describes information on user who created this ComputeInstance."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<ComputeInstanceCreatedBy>,
    #[doc = "Collection of errors encountered on this ComputeInstance."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<ErrorResponse>,
    #[doc = "Current state of an ComputeInstance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ComputeInstanceState>,
    #[doc = "The Compute Instance Authorization type. Available values are personal (default)."]
    #[serde(rename = "computeInstanceAuthorizationType", default, skip_serializing_if = "Option::is_none")]
    pub compute_instance_authorization_type: Option<compute_instance_properties::ComputeInstanceAuthorizationType>,
    #[doc = "Settings for a personal compute instance."]
    #[serde(rename = "personalComputeInstanceSettings", default, skip_serializing_if = "Option::is_none")]
    pub personal_compute_instance_settings: Option<PersonalComputeInstanceSettings>,
    #[doc = "Details of customized scripts to execute for setting up the cluster."]
    #[serde(rename = "setupScripts", default, skip_serializing_if = "Option::is_none")]
    pub setup_scripts: Option<SetupScripts>,
    #[doc = "The last operation on ComputeInstance."]
    #[serde(rename = "lastOperation", default, skip_serializing_if = "Option::is_none")]
    pub last_operation: Option<ComputeInstanceLastOperation>,
    #[doc = "The list of schedules to be applied on the computes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedules: Option<ComputeSchedules>,
    #[doc = "Stops compute instance after user defined period of inactivity. Time is defined in ISO8601 format. Minimum is 15 min, maximum is 3 days."]
    #[serde(rename = "idleTimeBeforeShutdown", default, skip_serializing_if = "Option::is_none")]
    pub idle_time_before_shutdown: Option<String>,
    #[doc = "Enable or disable node public IP address provisioning. Possible values are: Possible values are: true - Indicates that the compute nodes will have public IPs provisioned. false - Indicates that the compute nodes will have a private endpoint and no public IPs."]
    #[serde(rename = "enableNodePublicIp", default, skip_serializing_if = "Option::is_none")]
    pub enable_node_public_ip: Option<bool>,
    #[doc = "Describes informations of containers on this ComputeInstance."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub containers: Vec<ComputeInstanceContainer>,
    #[doc = "Describes informations of dataDisks on this ComputeInstance."]
    #[serde(
        rename = "dataDisks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_disks: Vec<ComputeInstanceDataDisk>,
    #[doc = "Describes informations of dataMounts on this ComputeInstance."]
    #[serde(
        rename = "dataMounts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_mounts: Vec<ComputeInstanceDataMount>,
    #[doc = "Version of computeInstance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<ComputeInstanceVersion>,
}
impl ComputeInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod compute_instance_properties {
    use super::*;
    #[doc = "Policy for sharing applications on this compute instance among users of parent workspace. If Personal, only the creator can access applications on this compute instance. When Shared, any workspace user can access applications on this instance depending on his/her assigned role."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ApplicationSharingPolicy")]
    pub enum ApplicationSharingPolicy {
        Personal,
        Shared,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ApplicationSharingPolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ApplicationSharingPolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ApplicationSharingPolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Personal => serializer.serialize_unit_variant("ApplicationSharingPolicy", 0u32, "Personal"),
                Self::Shared => serializer.serialize_unit_variant("ApplicationSharingPolicy", 1u32, "Shared"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ApplicationSharingPolicy {
        fn default() -> Self {
            Self::Shared
        }
    }
    #[doc = "The Compute Instance Authorization type. Available values are personal (default)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ComputeInstanceAuthorizationType")]
    pub enum ComputeInstanceAuthorizationType {
        #[serde(rename = "personal")]
        Personal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ComputeInstanceAuthorizationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ComputeInstanceAuthorizationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ComputeInstanceAuthorizationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Personal => serializer.serialize_unit_variant("ComputeInstanceAuthorizationType", 0u32, "personal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ComputeInstanceAuthorizationType {
        fn default() -> Self {
            Self::Personal
        }
    }
}
#[doc = "Properties(top level) of ComputeInstance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeInstanceSchema {
    #[doc = "Compute Instance properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ComputeInstanceProperties>,
}
impl ComputeInstanceSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies policy and settings for SSH access."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeInstanceSshSettings {
    #[doc = "State of the public SSH port. Possible values are: Disabled - Indicates that the public ssh port is closed on this instance. Enabled - Indicates that the public ssh port is open and accessible according to the VNet/subnet policy if applicable."]
    #[serde(rename = "sshPublicAccess", default, skip_serializing_if = "Option::is_none")]
    pub ssh_public_access: Option<compute_instance_ssh_settings::SshPublicAccess>,
    #[doc = "Describes the admin user name."]
    #[serde(rename = "adminUserName", default, skip_serializing_if = "Option::is_none")]
    pub admin_user_name: Option<String>,
    #[doc = "Describes the port for connecting through SSH."]
    #[serde(rename = "sshPort", default, skip_serializing_if = "Option::is_none")]
    pub ssh_port: Option<i32>,
    #[doc = "Specifies the SSH rsa public key file as a string. Use \"ssh-keygen -t rsa -b 2048\" to generate your SSH key pairs."]
    #[serde(rename = "adminPublicKey", default, skip_serializing_if = "Option::is_none")]
    pub admin_public_key: Option<String>,
}
impl ComputeInstanceSshSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod compute_instance_ssh_settings {
    use super::*;
    #[doc = "State of the public SSH port. Possible values are: Disabled - Indicates that the public ssh port is closed on this instance. Enabled - Indicates that the public ssh port is open and accessible according to the VNet/subnet policy if applicable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SshPublicAccess")]
    pub enum SshPublicAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SshPublicAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SshPublicAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SshPublicAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("SshPublicAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("SshPublicAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for SshPublicAccess {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[doc = "Current state of an ComputeInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ComputeInstanceState")]
pub enum ComputeInstanceState {
    Creating,
    CreateFailed,
    Deleting,
    Running,
    Restarting,
    JobRunning,
    SettingUp,
    SetupFailed,
    Starting,
    Stopped,
    Stopping,
    UserSettingUp,
    UserSetupFailed,
    Unknown,
    Unusable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ComputeInstanceState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ComputeInstanceState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ComputeInstanceState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Creating => serializer.serialize_unit_variant("ComputeInstanceState", 0u32, "Creating"),
            Self::CreateFailed => serializer.serialize_unit_variant("ComputeInstanceState", 1u32, "CreateFailed"),
            Self::Deleting => serializer.serialize_unit_variant("ComputeInstanceState", 2u32, "Deleting"),
            Self::Running => serializer.serialize_unit_variant("ComputeInstanceState", 3u32, "Running"),
            Self::Restarting => serializer.serialize_unit_variant("ComputeInstanceState", 4u32, "Restarting"),
            Self::JobRunning => serializer.serialize_unit_variant("ComputeInstanceState", 5u32, "JobRunning"),
            Self::SettingUp => serializer.serialize_unit_variant("ComputeInstanceState", 6u32, "SettingUp"),
            Self::SetupFailed => serializer.serialize_unit_variant("ComputeInstanceState", 7u32, "SetupFailed"),
            Self::Starting => serializer.serialize_unit_variant("ComputeInstanceState", 8u32, "Starting"),
            Self::Stopped => serializer.serialize_unit_variant("ComputeInstanceState", 9u32, "Stopped"),
            Self::Stopping => serializer.serialize_unit_variant("ComputeInstanceState", 10u32, "Stopping"),
            Self::UserSettingUp => serializer.serialize_unit_variant("ComputeInstanceState", 11u32, "UserSettingUp"),
            Self::UserSetupFailed => serializer.serialize_unit_variant("ComputeInstanceState", 12u32, "UserSetupFailed"),
            Self::Unknown => serializer.serialize_unit_variant("ComputeInstanceState", 13u32, "Unknown"),
            Self::Unusable => serializer.serialize_unit_variant("ComputeInstanceState", 14u32, "Unusable"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Version of computeInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeInstanceVersion {
    #[doc = "Runtime of compute instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
}
impl ComputeInstanceVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "[Required] The compute power action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ComputePowerAction")]
pub enum ComputePowerAction {
    Start,
    Stop,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ComputePowerAction {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ComputePowerAction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ComputePowerAction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Start => serializer.serialize_unit_variant("ComputePowerAction", 0u32, "Start"),
            Self::Stop => serializer.serialize_unit_variant("ComputePowerAction", 1u32, "Stop"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Machine Learning compute object wrapped into ARM resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub compute_resource_schema: ComputeResourceSchema,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Specifies the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Contains resource tags defined as key/value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl ComputeResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeResourceSchema {
    #[doc = "Machine Learning compute object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Compute>,
}
impl ComputeResourceSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeRuntimeDto {
    #[serde(rename = "sparkRuntimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub spark_runtime_version: Option<String>,
}
impl ComputeRuntimeDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of schedules to be applied on the computes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeSchedules {
    #[doc = "The list of compute start stop schedules to be applied."]
    #[serde(
        rename = "computeStartStop",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub compute_start_stop: Vec<ComputeStartStopSchedule>,
}
impl ComputeSchedules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Secrets related to a Machine Learning compute. Might differ for every type of compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeSecrets {
    #[doc = "The type of compute"]
    #[serde(rename = "computeType")]
    pub compute_type: ComputeType,
}
impl ComputeSecrets {
    pub fn new(compute_type: ComputeType) -> Self {
        Self { compute_type }
    }
}
#[doc = "Compute start stop schedule properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeStartStopSchedule {
    #[doc = "A system assigned id for the schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The current deployment state of schedule."]
    #[serde(rename = "provisioningStatus", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status: Option<compute_start_stop_schedule::ProvisioningStatus>,
    #[doc = "Is the schedule enabled or disabled?"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ScheduleStatus>,
    #[doc = "[Required] The compute power action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ComputePowerAction>,
    #[serde(rename = "triggerType", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<TriggerType>,
    #[doc = "The workflow trigger recurrence for ComputeStartStop schedule type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Recurrence>,
    #[doc = "The workflow trigger cron for ComputeStartStop schedule type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cron: Option<Cron>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ScheduleBase>,
}
impl ComputeStartStopSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod compute_start_stop_schedule {
    use super::*;
    #[doc = "The current deployment state of schedule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningStatus")]
    pub enum ProvisioningStatus {
        Completed,
        Provisioning,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Completed => serializer.serialize_unit_variant("ProvisioningStatus", 0u32, "Completed"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningStatus", 1u32, "Provisioning"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningStatus", 2u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The type of compute"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ComputeType")]
pub enum ComputeType {
    #[serde(rename = "AKS")]
    Aks,
    Kubernetes,
    AmlCompute,
    ComputeInstance,
    DataFactory,
    VirtualMachine,
    #[serde(rename = "HDInsight")]
    HdInsight,
    Databricks,
    DataLakeAnalytics,
    SynapseSpark,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ComputeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ComputeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ComputeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Aks => serializer.serialize_unit_variant("ComputeType", 0u32, "AKS"),
            Self::Kubernetes => serializer.serialize_unit_variant("ComputeType", 1u32, "Kubernetes"),
            Self::AmlCompute => serializer.serialize_unit_variant("ComputeType", 2u32, "AmlCompute"),
            Self::ComputeInstance => serializer.serialize_unit_variant("ComputeType", 3u32, "ComputeInstance"),
            Self::DataFactory => serializer.serialize_unit_variant("ComputeType", 4u32, "DataFactory"),
            Self::VirtualMachine => serializer.serialize_unit_variant("ComputeType", 5u32, "VirtualMachine"),
            Self::HdInsight => serializer.serialize_unit_variant("ComputeType", 6u32, "HDInsight"),
            Self::Databricks => serializer.serialize_unit_variant("ComputeType", 7u32, "Databricks"),
            Self::DataLakeAnalytics => serializer.serialize_unit_variant("ComputeType", 8u32, "DataLakeAnalytics"),
            Self::SynapseSpark => serializer.serialize_unit_variant("ComputeType", 9u32, "SynapseSpark"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Authentication type of the connection target"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ConnectionAuthType")]
pub enum ConnectionAuthType {
    #[serde(rename = "PAT")]
    Pat,
    ManagedIdentity,
    UsernamePassword,
    None,
    #[serde(rename = "SAS")]
    Sas,
    ServicePrincipal,
    AccessKey,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ConnectionAuthType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ConnectionAuthType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ConnectionAuthType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pat => serializer.serialize_unit_variant("ConnectionAuthType", 0u32, "PAT"),
            Self::ManagedIdentity => serializer.serialize_unit_variant("ConnectionAuthType", 1u32, "ManagedIdentity"),
            Self::UsernamePassword => serializer.serialize_unit_variant("ConnectionAuthType", 2u32, "UsernamePassword"),
            Self::None => serializer.serialize_unit_variant("ConnectionAuthType", 3u32, "None"),
            Self::Sas => serializer.serialize_unit_variant("ConnectionAuthType", 4u32, "SAS"),
            Self::ServicePrincipal => serializer.serialize_unit_variant("ConnectionAuthType", 5u32, "ServicePrincipal"),
            Self::AccessKey => serializer.serialize_unit_variant("ConnectionAuthType", 6u32, "AccessKey"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Category of the connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ConnectionCategory")]
pub enum ConnectionCategory {
    PythonFeed,
    ContainerRegistry,
    Git,
    FeatureStore,
    S3,
    Snowflake,
    AzureSqlDb,
    AzureSynapseAnalytics,
    AzureMySqlDb,
    AzurePostgresDb,
    AzureDataLakeGen2,
    Redis,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ConnectionCategory {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ConnectionCategory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ConnectionCategory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::PythonFeed => serializer.serialize_unit_variant("ConnectionCategory", 0u32, "PythonFeed"),
            Self::ContainerRegistry => serializer.serialize_unit_variant("ConnectionCategory", 1u32, "ContainerRegistry"),
            Self::Git => serializer.serialize_unit_variant("ConnectionCategory", 2u32, "Git"),
            Self::FeatureStore => serializer.serialize_unit_variant("ConnectionCategory", 3u32, "FeatureStore"),
            Self::S3 => serializer.serialize_unit_variant("ConnectionCategory", 4u32, "S3"),
            Self::Snowflake => serializer.serialize_unit_variant("ConnectionCategory", 5u32, "Snowflake"),
            Self::AzureSqlDb => serializer.serialize_unit_variant("ConnectionCategory", 6u32, "AzureSqlDb"),
            Self::AzureSynapseAnalytics => serializer.serialize_unit_variant("ConnectionCategory", 7u32, "AzureSynapseAnalytics"),
            Self::AzureMySqlDb => serializer.serialize_unit_variant("ConnectionCategory", 8u32, "AzureMySqlDb"),
            Self::AzurePostgresDb => serializer.serialize_unit_variant("ConnectionCategory", 9u32, "AzurePostgresDb"),
            Self::AzureDataLakeGen2 => serializer.serialize_unit_variant("ConnectionCategory", 10u32, "AzureDataLakeGen2"),
            Self::Redis => serializer.serialize_unit_variant("ConnectionCategory", 11u32, "Redis"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Resource requirements for each container instance within an online deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerResourceRequirements {
    #[serde(rename = "containerResourceLimits", default, skip_serializing_if = "Option::is_none")]
    pub container_resource_limits: Option<ContainerResourceSettings>,
    #[serde(rename = "containerResourceRequests", default, skip_serializing_if = "Option::is_none")]
    pub container_resource_requests: Option<ContainerResourceSettings>,
}
impl ContainerResourceRequirements {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerResourceSettings {
    #[doc = "Number of vCPUs request/limit for container. More info:\r\nhttps://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[doc = "Number of Nvidia GPU cards request/limit for container. More info:\r\nhttps://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gpu: Option<String>,
    #[doc = "Memory size request/limit for container. More info:\r\nhttps://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}
impl ContainerResourceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of container to retrieve logs from."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ContainerType")]
pub enum ContainerType {
    StorageInitializer,
    InferenceServer,
    ModelDataCollector,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ContainerType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ContainerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ContainerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::StorageInitializer => serializer.serialize_unit_variant("ContainerType", 0u32, "StorageInitializer"),
            Self::InferenceServer => serializer.serialize_unit_variant("ContainerType", 1u32, "InferenceServer"),
            Self::ModelDataCollector => serializer.serialize_unit_variant("ContainerType", 2u32, "ModelDataCollector"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CosmosDbSettings {
    #[doc = "The throughput of the collections in cosmosdb database"]
    #[serde(rename = "collectionsThroughput", default, skip_serializing_if = "Option::is_none")]
    pub collections_throughput: Option<i32>,
}
impl CosmosDbSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateMonitorAction {
    #[serde(flatten)]
    pub schedule_action_base: ScheduleActionBase,
    #[serde(rename = "monitorDefinition")]
    pub monitor_definition: MonitorDefinition,
}
impl CreateMonitorAction {
    pub fn new(schedule_action_base: ScheduleActionBase, monitor_definition: MonitorDefinition) -> Self {
        Self {
            schedule_action_base,
            monitor_definition,
        }
    }
}
#[doc = "Enum to determine the datastore credentials type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CredentialsType")]
pub enum CredentialsType {
    AccountKey,
    Certificate,
    None,
    Sas,
    ServicePrincipal,
    KerberosKeytab,
    KerberosPassword,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CredentialsType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CredentialsType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CredentialsType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AccountKey => serializer.serialize_unit_variant("CredentialsType", 0u32, "AccountKey"),
            Self::Certificate => serializer.serialize_unit_variant("CredentialsType", 1u32, "Certificate"),
            Self::None => serializer.serialize_unit_variant("CredentialsType", 2u32, "None"),
            Self::Sas => serializer.serialize_unit_variant("CredentialsType", 3u32, "Sas"),
            Self::ServicePrincipal => serializer.serialize_unit_variant("CredentialsType", 4u32, "ServicePrincipal"),
            Self::KerberosKeytab => serializer.serialize_unit_variant("CredentialsType", 5u32, "KerberosKeytab"),
            Self::KerberosPassword => serializer.serialize_unit_variant("CredentialsType", 6u32, "KerberosPassword"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The workflow trigger cron for ComputeStartStop schedule type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Cron {
    #[doc = "The start time in yyyy-MM-ddTHH:mm:ss format."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Specifies time zone in which the schedule runs.\r\nTimeZone should follow Windows time zone format. Refer: https://docs.microsoft.com/en-us/windows-hardware/manufacture/desktop/default-time-zones?view=windows-11"]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "[Required] Specifies cron expression of schedule.\r\nThe expression should follow NCronTab format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
}
impl Cron {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CronTrigger {
    #[serde(flatten)]
    pub trigger_base: TriggerBase,
    #[doc = "[Required] Specifies cron expression of schedule.\r\nThe expression should follow NCronTab format."]
    pub expression: String,
}
impl CronTrigger {
    pub fn new(trigger_base: TriggerBase, expression: String) -> Self {
        Self { trigger_base, expression }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsvExportSummary {
    #[serde(flatten)]
    pub export_summary: ExportSummary,
    #[doc = "The container name to which the labels will be exported."]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "The output path where the labels will be exported."]
    #[serde(rename = "snapshotPath", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_path: Option<String>,
}
impl CsvExportSummary {
    pub fn new(export_summary: ExportSummary) -> Self {
        Self {
            export_summary,
            container_name: None,
            snapshot_path: None,
        }
    }
}
#[doc = "The desired maximum forecast horizon in units of time-series frequency."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomForecastHorizon {
    #[serde(flatten)]
    pub forecast_horizon: ForecastHorizon,
    #[doc = "[Required] Forecast horizon value."]
    pub value: i32,
}
impl CustomForecastHorizon {
    pub fn new(forecast_horizon: ForecastHorizon, value: i32) -> Self {
        Self { forecast_horizon, value }
    }
}
#[doc = "Custom inference server configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomInferencingServer {
    #[serde(flatten)]
    pub inferencing_server: InferencingServer,
    #[doc = "Online inference configuration options."]
    #[serde(rename = "inferenceConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub inference_configuration: Option<OnlineInferenceConfiguration>,
}
impl CustomInferencingServer {
    pub fn new(inferencing_server: InferencingServer) -> Self {
        Self {
            inferencing_server,
            inference_configuration: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomMetricThreshold {
    #[doc = "[Required] The user-defined metric to calculate."]
    pub metric: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<MonitoringThreshold>,
}
impl CustomMetricThreshold {
    pub fn new(metric: String) -> Self {
        Self { metric, threshold: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomModelJobInput {
    #[serde(flatten)]
    pub asset_job_input: AssetJobInput,
    #[serde(flatten)]
    pub job_input: JobInput,
}
impl CustomModelJobInput {
    pub fn new(asset_job_input: AssetJobInput, job_input: JobInput) -> Self {
        Self {
            asset_job_input,
            job_input,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomModelJobOutput {
    #[serde(flatten)]
    pub asset_job_output: AssetJobOutput,
    #[serde(flatten)]
    pub job_output: JobOutput,
}
impl CustomModelJobOutput {
    pub fn new(job_output: JobOutput) -> Self {
        Self {
            asset_job_output: AssetJobOutput::default(),
            job_output,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomMonitoringSignal {
    #[serde(flatten)]
    pub monitoring_signal_base: MonitoringSignalBase,
    #[doc = "[Required] ARM resource ID of the component resource used to calculate the custom metrics."]
    #[serde(rename = "componentId")]
    pub component_id: String,
    #[doc = "Monitoring assets to take as input. Key is the component input port name, value is the data asset."]
    #[serde(rename = "inputAssets", default, skip_serializing_if = "Option::is_none")]
    pub input_assets: Option<serde_json::Value>,
    #[doc = "[Required] A list of metrics to calculate and their associated thresholds."]
    #[serde(rename = "metricThresholds")]
    pub metric_thresholds: Vec<CustomMetricThreshold>,
}
impl CustomMonitoringSignal {
    pub fn new(monitoring_signal_base: MonitoringSignalBase, component_id: String, metric_thresholds: Vec<CustomMetricThreshold>) -> Self {
        Self {
            monitoring_signal_base,
            component_id,
            input_assets: None,
            metric_thresholds,
        }
    }
}
#[doc = "N-Cross validations are specified by user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomNCrossValidations {
    #[serde(flatten)]
    pub n_cross_validations: NCrossValidations,
    #[doc = "[Required] N-Cross validations value."]
    pub value: i32,
}
impl CustomNCrossValidations {
    pub fn new(n_cross_validations: NCrossValidations, value: i32) -> Self {
        Self {
            n_cross_validations,
            value,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomSeasonality {
    #[serde(flatten)]
    pub seasonality: Seasonality,
    #[doc = "[Required] Seasonality value."]
    pub value: i32,
}
impl CustomSeasonality {
    pub fn new(seasonality: Seasonality, value: i32) -> Self {
        Self { seasonality, value }
    }
}
#[doc = "Specifies the custom service configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomService {
    #[doc = "Name of the Custom Service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    #[doc = "Environment Variable for the container"]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docker: Option<Docker>,
    #[doc = "Configuring the endpoints for the container"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endpoints: Vec<Endpoint>,
    #[doc = "Configuring the volumes for the container"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volumes: Vec<VolumeDefinition>,
}
impl CustomService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomTargetLags {
    #[serde(flatten)]
    pub target_lags: TargetLags,
    #[doc = "[Required] Set target lags values."]
    pub values: Vec<i32>,
}
impl CustomTargetLags {
    pub fn new(target_lags: TargetLags, values: Vec<i32>) -> Self {
        Self { target_lags, values }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomTargetRollingWindowSize {
    #[serde(flatten)]
    pub target_rolling_window_size: TargetRollingWindowSize,
    #[doc = "[Required] TargetRollingWindowSize value."]
    pub value: i32,
}
impl CustomTargetRollingWindowSize {
    pub fn new(target_rolling_window_size: TargetRollingWindowSize, value: i32) -> Self {
        Self {
            target_rolling_window_size,
            value,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataCollectionMode")]
pub enum DataCollectionMode {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataCollectionMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataCollectionMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataCollectionMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("DataCollectionMode", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("DataCollectionMode", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataCollector {
    #[doc = "[Required] The collection configuration. Each collection has it own configuration to collect model data and the name of collection can be arbitrary string.\r\nModel data collector can be used for either payload logging or custom logging or both of them. Collection request and response are reserved for payload logging, others are for custom logging."]
    pub collections: serde_json::Value,
    #[serde(rename = "requestLogging", default, skip_serializing_if = "Option::is_none")]
    pub request_logging: Option<RequestLogging>,
    #[serde(rename = "rollingRate", default, skip_serializing_if = "Option::is_none")]
    pub rolling_rate: Option<RollingRateType>,
}
impl DataCollector {
    pub fn new(collections: serde_json::Value) -> Self {
        Self {
            collections,
            request_logging: None,
            rolling_rate: None,
        }
    }
}
#[doc = "Container for data asset versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataContainer {
    #[serde(flatten)]
    pub asset_container: AssetContainer,
    #[doc = "Enum to determine the type of data."]
    #[serde(rename = "dataType")]
    pub data_type: DataType,
}
impl DataContainer {
    pub fn new(data_type: DataType) -> Self {
        Self {
            asset_container: AssetContainer::default(),
            data_type,
        }
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataContainerResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Container for data asset versions."]
    pub properties: DataContainer,
}
impl DataContainerResource {
    pub fn new(properties: DataContainer) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of DataContainer entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataContainerResourceArmPaginatedResult {
    #[doc = "The link to the next page of DataContainer objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type DataContainer."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DataContainerResource>,
}
impl azure_core::Continuable for DataContainerResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DataContainerResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataDriftMetricThresholdBase {
    #[serde(rename = "dataType")]
    pub data_type: MonitoringFeatureDataType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<MonitoringThreshold>,
}
impl DataDriftMetricThresholdBase {
    pub fn new(data_type: MonitoringFeatureDataType) -> Self {
        Self {
            data_type,
            threshold: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataDriftMonitoringSignal {
    #[serde(flatten)]
    pub monitoring_signal_base: MonitoringSignalBase,
    #[serde(rename = "baselineData")]
    pub baseline_data: MonitoringInputData,
    #[serde(rename = "dataSegment", default, skip_serializing_if = "Option::is_none")]
    pub data_segment: Option<MonitoringDataSegment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<MonitoringFeatureFilterBase>,
    #[doc = "[Required] A list of metrics to calculate and their associated thresholds."]
    #[serde(rename = "metricThresholds")]
    pub metric_thresholds: Vec<DataDriftMetricThresholdBase>,
    #[serde(rename = "targetData")]
    pub target_data: MonitoringInputData,
}
impl DataDriftMonitoringSignal {
    pub fn new(
        monitoring_signal_base: MonitoringSignalBase,
        baseline_data: MonitoringInputData,
        metric_thresholds: Vec<DataDriftMetricThresholdBase>,
        target_data: MonitoringInputData,
    ) -> Self {
        Self {
            monitoring_signal_base,
            baseline_data,
            data_segment: None,
            features: None,
            metric_thresholds,
            target_data,
        }
    }
}
#[doc = "A DataFactory compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataFactory {
    #[serde(flatten)]
    pub compute: Compute,
}
impl DataFactory {
    pub fn new(compute: Compute) -> Self {
        Self { compute }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataImport {
    #[serde(flatten)]
    pub data_version_base: DataVersionBase,
    #[doc = "Name of the asset for data import job to create"]
    #[serde(rename = "assetName", default, skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<DataImportSource>,
}
impl DataImport {
    pub fn new(data_version_base: DataVersionBase) -> Self {
        Self {
            data_version_base,
            asset_name: None,
            source: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataImportSource {
    #[doc = "Workspace connection for data import source storage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
    #[doc = "Enum to determine the type of data."]
    #[serde(rename = "sourceType")]
    pub source_type: DataImportSourceType,
}
impl DataImportSource {
    pub fn new(source_type: DataImportSourceType) -> Self {
        Self {
            connection: None,
            source_type,
        }
    }
}
#[doc = "Enum to determine the type of data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataImportSourceType")]
pub enum DataImportSourceType {
    #[serde(rename = "database")]
    Database,
    #[serde(rename = "file_system")]
    FileSystem,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataImportSourceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataImportSourceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataImportSourceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Database => serializer.serialize_unit_variant("DataImportSourceType", 0u32, "database"),
            Self::FileSystem => serializer.serialize_unit_variant("DataImportSourceType", 1u32, "file_system"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A DataLakeAnalytics compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeAnalytics {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(flatten)]
    pub data_lake_analytics_schema: DataLakeAnalyticsSchema,
}
impl DataLakeAnalytics {
    pub fn new(compute: Compute) -> Self {
        Self {
            compute,
            data_lake_analytics_schema: DataLakeAnalyticsSchema::default(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeAnalyticsSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<data_lake_analytics_schema::Properties>,
}
impl DataLakeAnalyticsSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_lake_analytics_schema {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "DataLake Store Account Name"]
        #[serde(rename = "dataLakeStoreAccountName", default, skip_serializing_if = "Option::is_none")]
        pub data_lake_store_account_name: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Reference to an asset via its path in a datastore."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataPathAssetReference {
    #[serde(flatten)]
    pub asset_reference_base: AssetReferenceBase,
    #[doc = "ARM resource ID of the datastore where the asset is located."]
    #[serde(rename = "datastoreId", default, skip_serializing_if = "Option::is_none")]
    pub datastore_id: Option<String>,
    #[doc = "The path of the file/directory in the datastore."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl DataPathAssetReference {
    pub fn new(asset_reference_base: AssetReferenceBase) -> Self {
        Self {
            asset_reference_base,
            datastore_id: None,
            path: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataQualityMetricThresholdBase {
    #[serde(rename = "dataType")]
    pub data_type: MonitoringFeatureDataType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<MonitoringThreshold>,
}
impl DataQualityMetricThresholdBase {
    pub fn new(data_type: MonitoringFeatureDataType) -> Self {
        Self {
            data_type,
            threshold: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataQualityMonitoringSignal {
    #[serde(flatten)]
    pub monitoring_signal_base: MonitoringSignalBase,
    #[serde(rename = "baselineData")]
    pub baseline_data: MonitoringInputData,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<MonitoringFeatureFilterBase>,
    #[doc = "[Required] A list of metrics to calculate and their associated thresholds."]
    #[serde(rename = "metricThresholds")]
    pub metric_thresholds: Vec<DataQualityMetricThresholdBase>,
    #[serde(rename = "targetData")]
    pub target_data: MonitoringInputData,
}
impl DataQualityMonitoringSignal {
    pub fn new(
        monitoring_signal_base: MonitoringSignalBase,
        baseline_data: MonitoringInputData,
        metric_thresholds: Vec<DataQualityMetricThresholdBase>,
        target_data: MonitoringInputData,
    ) -> Self {
        Self {
            monitoring_signal_base,
            baseline_data,
            features: None,
            metric_thresholds,
            target_data,
        }
    }
}
#[doc = "Enum to determine the type of data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataType")]
pub enum DataType {
    #[serde(rename = "uri_file")]
    UriFile,
    #[serde(rename = "uri_folder")]
    UriFolder,
    #[serde(rename = "mltable")]
    Mltable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UriFile => serializer.serialize_unit_variant("DataType", 0u32, "uri_file"),
            Self::UriFolder => serializer.serialize_unit_variant("DataType", 1u32, "uri_folder"),
            Self::Mltable => serializer.serialize_unit_variant("DataType", 2u32, "mltable"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Data version base definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataVersionBase {
    #[serde(flatten)]
    pub asset_base: AssetBase,
    #[doc = "Enum to determine the type of data."]
    #[serde(rename = "dataType")]
    pub data_type: DataType,
    #[doc = "[Required] Uri of the data. Example: https://go.microsoft.com/fwlink/?linkid=2202330"]
    #[serde(rename = "dataUri")]
    pub data_uri: String,
    #[doc = "Intellectual Property details for a resource."]
    #[serde(rename = "intellectualProperty", default, skip_serializing_if = "Option::is_none")]
    pub intellectual_property: Option<IntellectualProperty>,
    #[doc = "Stage in the data lifecycle assigned to this data asset"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}
impl DataVersionBase {
    pub fn new(data_type: DataType, data_uri: String) -> Self {
        Self {
            asset_base: AssetBase::default(),
            data_type,
            data_uri,
            intellectual_property: None,
            stage: None,
        }
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataVersionBaseResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Data version base definition"]
    pub properties: DataVersionBase,
}
impl DataVersionBaseResource {
    pub fn new(properties: DataVersionBase) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of DataVersionBase entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataVersionBaseResourceArmPaginatedResult {
    #[doc = "The link to the next page of DataVersionBase objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type DataVersionBase."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DataVersionBaseResource>,
}
impl azure_core::Continuable for DataVersionBaseResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DataVersionBaseResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseSource {
    #[serde(flatten)]
    pub data_import_source: DataImportSource,
    #[doc = "SQL Query statement for data import Database source"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[doc = "SQL StoredProcedure on data import Database source"]
    #[serde(rename = "storedProcedure", default, skip_serializing_if = "Option::is_none")]
    pub stored_procedure: Option<String>,
    #[doc = "SQL StoredProcedure parameters"]
    #[serde(
        rename = "storedProcedureParams",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub stored_procedure_params: Vec<serde_json::Value>,
    #[doc = "Name of the table on data import Database source"]
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}
impl DatabaseSource {
    pub fn new(data_import_source: DataImportSource) -> Self {
        Self {
            data_import_source,
            query: None,
            stored_procedure: None,
            stored_procedure_params: Vec::new(),
            table_name: None,
        }
    }
}
#[doc = "A DataFactory compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Databricks {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(flatten)]
    pub databricks_schema: DatabricksSchema,
}
impl Databricks {
    pub fn new(compute: Compute) -> Self {
        Self {
            compute,
            databricks_schema: DatabricksSchema::default(),
        }
    }
}
#[doc = "Secrets related to a Machine Learning compute based on Databricks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabricksComputeSecrets {
    #[serde(flatten)]
    pub compute_secrets: ComputeSecrets,
    #[serde(flatten)]
    pub databricks_compute_secrets_properties: DatabricksComputeSecretsProperties,
}
impl DatabricksComputeSecrets {
    pub fn new(compute_secrets: ComputeSecrets) -> Self {
        Self {
            compute_secrets,
            databricks_compute_secrets_properties: DatabricksComputeSecretsProperties::default(),
        }
    }
}
#[doc = "Properties of Databricks Compute Secrets"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabricksComputeSecretsProperties {
    #[doc = "access token for databricks account."]
    #[serde(rename = "databricksAccessToken", default, skip_serializing_if = "Option::is_none")]
    pub databricks_access_token: Option<String>,
}
impl DatabricksComputeSecretsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Databricks"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabricksProperties {
    #[doc = "Databricks access token"]
    #[serde(rename = "databricksAccessToken", default, skip_serializing_if = "Option::is_none")]
    pub databricks_access_token: Option<String>,
    #[doc = "Workspace Url"]
    #[serde(rename = "workspaceUrl", default, skip_serializing_if = "Option::is_none")]
    pub workspace_url: Option<String>,
}
impl DatabricksProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabricksSchema {
    #[doc = "Properties of Databricks"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabricksProperties>,
}
impl DatabricksSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatasetExportSummary {
    #[serde(flatten)]
    pub export_summary: ExportSummary,
    #[doc = "The unique name of the labeled data asset."]
    #[serde(rename = "labeledAssetName", default, skip_serializing_if = "Option::is_none")]
    pub labeled_asset_name: Option<String>,
}
impl DatasetExportSummary {
    pub fn new(export_summary: ExportSummary) -> Self {
        Self {
            export_summary,
            labeled_asset_name: None,
        }
    }
}
#[doc = "Base definition for datastore contents configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Datastore {
    #[serde(flatten)]
    pub resource_base: ResourceBase,
    #[doc = "Base definition for datastore credentials."]
    pub credentials: DatastoreCredentials,
    #[doc = "Enum to determine the datastore contents type."]
    #[serde(rename = "datastoreType")]
    pub datastore_type: DatastoreType,
    #[doc = "Intellectual Property details for a resource."]
    #[serde(rename = "intellectualProperty", default, skip_serializing_if = "Option::is_none")]
    pub intellectual_property: Option<IntellectualProperty>,
    #[doc = "Readonly property to indicate if datastore is the workspace default datastore"]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}
impl Datastore {
    pub fn new(credentials: DatastoreCredentials, datastore_type: DatastoreType) -> Self {
        Self {
            resource_base: ResourceBase::default(),
            credentials,
            datastore_type,
            intellectual_property: None,
            is_default: None,
        }
    }
}
#[doc = "Base definition for datastore credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatastoreCredentials {
    #[doc = "Enum to determine the datastore credentials type."]
    #[serde(rename = "credentialsType")]
    pub credentials_type: CredentialsType,
}
impl DatastoreCredentials {
    pub fn new(credentials_type: CredentialsType) -> Self {
        Self { credentials_type }
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatastoreResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Base definition for datastore contents configuration."]
    pub properties: Datastore,
}
impl DatastoreResource {
    pub fn new(properties: Datastore) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of Datastore entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatastoreResourceArmPaginatedResult {
    #[doc = "The link to the next page of Datastore objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type Datastore."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DatastoreResource>,
}
impl azure_core::Continuable for DatastoreResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DatastoreResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base definition for datastore secrets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatastoreSecrets {
    #[doc = "Enum to determine the datastore secrets type."]
    #[serde(rename = "secretsType")]
    pub secrets_type: SecretsType,
}
impl DatastoreSecrets {
    pub fn new(secrets_type: SecretsType) -> Self {
        Self { secrets_type }
    }
}
#[doc = "Enum to determine the datastore contents type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DatastoreType")]
pub enum DatastoreType {
    AzureBlob,
    AzureDataLakeGen1,
    AzureDataLakeGen2,
    AzureFile,
    Hdfs,
    OneLake,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DatastoreType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DatastoreType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DatastoreType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureBlob => serializer.serialize_unit_variant("DatastoreType", 0u32, "AzureBlob"),
            Self::AzureDataLakeGen1 => serializer.serialize_unit_variant("DatastoreType", 1u32, "AzureDataLakeGen1"),
            Self::AzureDataLakeGen2 => serializer.serialize_unit_variant("DatastoreType", 2u32, "AzureDataLakeGen2"),
            Self::AzureFile => serializer.serialize_unit_variant("DatastoreType", 3u32, "AzureFile"),
            Self::Hdfs => serializer.serialize_unit_variant("DatastoreType", 4u32, "Hdfs"),
            Self::OneLake => serializer.serialize_unit_variant("DatastoreType", 5u32, "OneLake"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultScaleSettings {
    #[serde(flatten)]
    pub online_scale_settings: OnlineScaleSettings,
}
impl DefaultScaleSettings {
    pub fn new(online_scale_settings: OnlineScaleSettings) -> Self {
        Self { online_scale_settings }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentLogs {
    #[doc = "The retrieved online deployment logs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
impl DeploymentLogs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentLogsRequest {
    #[doc = "The type of container to retrieve logs from."]
    #[serde(rename = "containerType", default, skip_serializing_if = "Option::is_none")]
    pub container_type: Option<ContainerType>,
    #[doc = "The maximum number of lines to tail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tail: Option<i32>,
}
impl DeploymentLogsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Possible values for DeploymentProvisioningState."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeploymentProvisioningState")]
pub enum DeploymentProvisioningState {
    Creating,
    Deleting,
    Scaling,
    Updating,
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeploymentProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeploymentProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeploymentProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Creating => serializer.serialize_unit_variant("DeploymentProvisioningState", 0u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("DeploymentProvisioningState", 1u32, "Deleting"),
            Self::Scaling => serializer.serialize_unit_variant("DeploymentProvisioningState", 2u32, "Scaling"),
            Self::Updating => serializer.serialize_unit_variant("DeploymentProvisioningState", 3u32, "Updating"),
            Self::Succeeded => serializer.serialize_unit_variant("DeploymentProvisioningState", 4u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("DeploymentProvisioningState", 5u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("DeploymentProvisioningState", 6u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentResourceConfiguration {
    #[serde(flatten)]
    pub resource_configuration: ResourceConfiguration,
}
impl DeploymentResourceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnoseRequestProperties {
    #[doc = "Setting for diagnosing user defined routing"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub udr: Option<serde_json::Value>,
    #[doc = "Setting for diagnosing network security group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsg: Option<serde_json::Value>,
    #[doc = "Setting for diagnosing resource lock"]
    #[serde(rename = "resourceLock", default, skip_serializing_if = "Option::is_none")]
    pub resource_lock: Option<serde_json::Value>,
    #[doc = "Setting for diagnosing dns resolution"]
    #[serde(rename = "dnsResolution", default, skip_serializing_if = "Option::is_none")]
    pub dns_resolution: Option<serde_json::Value>,
    #[doc = "Setting for diagnosing dependent storage account"]
    #[serde(rename = "storageAccount", default, skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<serde_json::Value>,
    #[doc = "Setting for diagnosing dependent key vault"]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<serde_json::Value>,
    #[doc = "Setting for diagnosing dependent container registry"]
    #[serde(rename = "containerRegistry", default, skip_serializing_if = "Option::is_none")]
    pub container_registry: Option<serde_json::Value>,
    #[doc = "Setting for diagnosing dependent application insights"]
    #[serde(rename = "applicationInsights", default, skip_serializing_if = "Option::is_none")]
    pub application_insights: Option<serde_json::Value>,
    #[doc = "Setting for diagnosing unclassified category of problems"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub others: Option<serde_json::Value>,
}
impl DiagnoseRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnoseResponseResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<diagnose_response_result::Value>,
}
impl DiagnoseResponseResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod diagnose_response_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Value {
        #[serde(
            rename = "userDefinedRouteResults",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub user_defined_route_results: Vec<DiagnoseResult>,
        #[serde(
            rename = "networkSecurityRuleResults",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub network_security_rule_results: Vec<DiagnoseResult>,
        #[serde(
            rename = "resourceLockResults",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub resource_lock_results: Vec<DiagnoseResult>,
        #[serde(
            rename = "dnsResolutionResults",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub dns_resolution_results: Vec<DiagnoseResult>,
        #[serde(
            rename = "storageAccountResults",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub storage_account_results: Vec<DiagnoseResult>,
        #[serde(
            rename = "keyVaultResults",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub key_vault_results: Vec<DiagnoseResult>,
        #[serde(
            rename = "containerRegistryResults",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub container_registry_results: Vec<DiagnoseResult>,
        #[serde(
            rename = "applicationInsightsResults",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub application_insights_results: Vec<DiagnoseResult>,
        #[serde(
            rename = "otherResults",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub other_results: Vec<DiagnoseResult>,
    }
    impl Value {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of Diagnose"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnoseResult {
    #[doc = "Code for workspace setup error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Level of workspace setup error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<diagnose_result::Level>,
    #[doc = "Message of workspace setup error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl DiagnoseResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod diagnose_result {
    use super::*;
    #[doc = "Level of workspace setup error"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Level")]
    pub enum Level {
        Warning,
        Error,
        Information,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Level {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Level {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Level {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Warning => serializer.serialize_unit_variant("Level", 0u32, "Warning"),
                Self::Error => serializer.serialize_unit_variant("Level", 1u32, "Error"),
                Self::Information => serializer.serialize_unit_variant("Level", 2u32, "Information"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Parameters to diagnose a workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnoseWorkspaceParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<DiagnoseRequestProperties>,
}
impl DiagnoseWorkspaceParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base definition for job distribution configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributionConfiguration {
    #[doc = "Enum to determine the job distribution type."]
    #[serde(rename = "distributionType")]
    pub distribution_type: DistributionType,
}
impl DistributionConfiguration {
    pub fn new(distribution_type: DistributionType) -> Self {
        Self { distribution_type }
    }
}
#[doc = "Enum to determine the job distribution type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DistributionType")]
pub enum DistributionType {
    PyTorch,
    TensorFlow,
    Mpi,
    Ray,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DistributionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DistributionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DistributionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::PyTorch => serializer.serialize_unit_variant("DistributionType", 0u32, "PyTorch"),
            Self::TensorFlow => serializer.serialize_unit_variant("DistributionType", 1u32, "TensorFlow"),
            Self::Mpi => serializer.serialize_unit_variant("DistributionType", 2u32, "Mpi"),
            Self::Ray => serializer.serialize_unit_variant("DistributionType", 3u32, "Ray"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Docker {
    #[doc = "Indicate whether container shall run in privileged or non-privileged mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
}
impl Docker {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Early termination policies enable canceling poor-performing runs before they complete"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EarlyTerminationPolicy {
    #[doc = "Number of intervals by which to delay the first evaluation."]
    #[serde(rename = "delayEvaluation", default, skip_serializing_if = "Option::is_none")]
    pub delay_evaluation: Option<i32>,
    #[doc = "Interval (number of runs) between policy evaluations."]
    #[serde(rename = "evaluationInterval", default, skip_serializing_if = "Option::is_none")]
    pub evaluation_interval: Option<i32>,
    #[serde(rename = "policyType")]
    pub policy_type: EarlyTerminationPolicyType,
}
impl EarlyTerminationPolicy {
    pub fn new(policy_type: EarlyTerminationPolicyType) -> Self {
        Self {
            delay_evaluation: None,
            evaluation_interval: None,
            policy_type,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EarlyTerminationPolicyType")]
pub enum EarlyTerminationPolicyType {
    Bandit,
    MedianStopping,
    TruncationSelection,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EarlyTerminationPolicyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EarlyTerminationPolicyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EarlyTerminationPolicyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bandit => serializer.serialize_unit_variant("EarlyTerminationPolicyType", 0u32, "Bandit"),
            Self::MedianStopping => serializer.serialize_unit_variant("EarlyTerminationPolicyType", 1u32, "MedianStopping"),
            Self::TruncationSelection => serializer.serialize_unit_variant("EarlyTerminationPolicyType", 2u32, "TruncationSelection"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enum to determine whether PublicNetworkAccess is Enabled or Disabled for egress of a deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EgressPublicNetworkAccessType")]
pub enum EgressPublicNetworkAccessType {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EgressPublicNetworkAccessType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EgressPublicNetworkAccessType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EgressPublicNetworkAccessType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("EgressPublicNetworkAccessType", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("EgressPublicNetworkAccessType", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailMonitoringAlertNotificationSettings {
    #[serde(flatten)]
    pub monitoring_alert_notification_settings_base: MonitoringAlertNotificationSettingsBase,
    #[doc = "Configuration for notification."]
    #[serde(rename = "emailNotificationSetting", default, skip_serializing_if = "Option::is_none")]
    pub email_notification_setting: Option<NotificationSetting>,
}
impl EmailMonitoringAlertNotificationSettings {
    pub fn new(monitoring_alert_notification_settings_base: MonitoringAlertNotificationSettingsBase) -> Self {
        Self {
            monitoring_alert_notification_settings_base,
            email_notification_setting: None,
        }
    }
}
#[doc = "Enum to determine the email notification type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EmailNotificationEnableType")]
pub enum EmailNotificationEnableType {
    JobCompleted,
    JobFailed,
    JobCancelled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EmailNotificationEnableType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EmailNotificationEnableType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EmailNotificationEnableType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::JobCompleted => serializer.serialize_unit_variant("EmailNotificationEnableType", 0u32, "JobCompleted"),
            Self::JobFailed => serializer.serialize_unit_variant("EmailNotificationEnableType", 1u32, "JobFailed"),
            Self::JobCancelled => serializer.serialize_unit_variant("EmailNotificationEnableType", 2u32, "JobCancelled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionKeyVaultProperties {
    #[doc = "The ArmId of the keyVault where the customer owned encryption key is present."]
    #[serde(rename = "keyVaultArmId")]
    pub key_vault_arm_id: String,
    #[doc = "Key vault uri to access the encryption key."]
    #[serde(rename = "keyIdentifier")]
    pub key_identifier: String,
    #[doc = "For future use - The client id of the identity which will be used to access key vault."]
    #[serde(rename = "identityClientId", default, skip_serializing_if = "Option::is_none")]
    pub identity_client_id: Option<String>,
}
impl EncryptionKeyVaultProperties {
    pub fn new(key_vault_arm_id: String, key_identifier: String) -> Self {
        Self {
            key_vault_arm_id,
            key_identifier,
            identity_client_id: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionKeyVaultUpdateProperties {
    #[doc = "Key Vault uri to access the encryption key."]
    #[serde(rename = "keyIdentifier")]
    pub key_identifier: String,
}
impl EncryptionKeyVaultUpdateProperties {
    pub fn new(key_identifier: String) -> Self {
        Self { key_identifier }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionProperty {
    #[doc = "Indicates whether or not the encryption is enabled for the workspace."]
    pub status: encryption_property::Status,
    #[doc = "Identity that will be used to access key vault for encryption at rest"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityForCmk>,
    #[serde(rename = "keyVaultProperties")]
    pub key_vault_properties: EncryptionKeyVaultProperties,
}
impl EncryptionProperty {
    pub fn new(status: encryption_property::Status, key_vault_properties: EncryptionKeyVaultProperties) -> Self {
        Self {
            status,
            identity: None,
            key_vault_properties,
        }
    }
}
pub mod encryption_property {
    use super::*;
    #[doc = "Indicates whether or not the encryption is enabled for the workspace."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionUpdateProperties {
    #[serde(rename = "keyVaultProperties")]
    pub key_vault_properties: EncryptionKeyVaultUpdateProperties,
}
impl EncryptionUpdateProperties {
    pub fn new(key_vault_properties: EncryptionKeyVaultUpdateProperties) -> Self {
        Self { key_vault_properties }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Endpoint {
    #[doc = "Protocol over which communication will happen over this endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<endpoint::Protocol>,
    #[doc = "Name of the Endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Application port inside the container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<i32>,
    #[doc = "Port over which the application is exposed from container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published: Option<i32>,
    #[doc = "Host IP over which the application is exposed from the container"]
    #[serde(rename = "hostIp", default, skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,
}
impl Endpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod endpoint {
    use super::*;
    #[doc = "Protocol over which communication will happen over this endpoint"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        #[serde(rename = "tcp")]
        Tcp,
        #[serde(rename = "udp")]
        Udp,
        #[serde(rename = "http")]
        Http,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Protocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Protocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Protocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tcp => serializer.serialize_unit_variant("Protocol", 0u32, "tcp"),
                Self::Udp => serializer.serialize_unit_variant("Protocol", 1u32, "udp"),
                Self::Http => serializer.serialize_unit_variant("Protocol", 2u32, "http"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Protocol {
        fn default() -> Self {
            Self::Tcp
        }
    }
}
#[doc = "Keys for endpoint authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointAuthKeys {
    #[doc = "The primary key."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "The secondary key."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
}
impl EndpointAuthKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enum to determine endpoint authentication mode."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EndpointAuthMode")]
pub enum EndpointAuthMode {
    #[serde(rename = "AMLToken")]
    AmlToken,
    Key,
    #[serde(rename = "AADToken")]
    AadToken,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EndpointAuthMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EndpointAuthMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EndpointAuthMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AmlToken => serializer.serialize_unit_variant("EndpointAuthMode", 0u32, "AMLToken"),
            Self::Key => serializer.serialize_unit_variant("EndpointAuthMode", 1u32, "Key"),
            Self::AadToken => serializer.serialize_unit_variant("EndpointAuthMode", 2u32, "AADToken"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Service Token"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointAuthToken {
    #[doc = "Access token for endpoint authentication."]
    #[serde(rename = "accessToken", default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[doc = "Access token expiry time (UTC)."]
    #[serde(rename = "expiryTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub expiry_time_utc: Option<i64>,
    #[doc = "Refresh access token after time (UTC)."]
    #[serde(rename = "refreshAfterTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub refresh_after_time_utc: Option<i64>,
    #[doc = "Access token type."]
    #[serde(rename = "tokenType", default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
}
impl EndpointAuthToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enum to determine endpoint compute type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EndpointComputeType")]
pub enum EndpointComputeType {
    Managed,
    Kubernetes,
    #[serde(rename = "AzureMLCompute")]
    AzureMlCompute,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EndpointComputeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EndpointComputeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EndpointComputeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Managed => serializer.serialize_unit_variant("EndpointComputeType", 0u32, "Managed"),
            Self::Kubernetes => serializer.serialize_unit_variant("EndpointComputeType", 1u32, "Kubernetes"),
            Self::AzureMlCompute => serializer.serialize_unit_variant("EndpointComputeType", 2u32, "AzureMLCompute"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Base definition for endpoint deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDeploymentPropertiesBase {
    #[doc = "Configuration for a scoring code asset."]
    #[serde(rename = "codeConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub code_configuration: Option<CodeConfiguration>,
    #[doc = "Description of the endpoint deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "ARM resource ID of the environment specification for the endpoint deployment."]
    #[serde(rename = "environmentId", default, skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[doc = "Environment variables configuration for the deployment."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[doc = "Property dictionary. Properties can be added, but not removed or altered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl EndpointDeploymentPropertiesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Inference Endpoint base definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointPropertiesBase {
    #[doc = "Enum to determine endpoint authentication mode."]
    #[serde(rename = "authMode")]
    pub auth_mode: EndpointAuthMode,
    #[doc = "Description of the inference endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Keys for endpoint authentication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<EndpointAuthKeys>,
    #[doc = "Property dictionary. Properties can be added, but not removed or altered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Endpoint URI."]
    #[serde(rename = "scoringUri", default, skip_serializing_if = "Option::is_none")]
    pub scoring_uri: Option<String>,
    #[doc = "Endpoint Swagger URI."]
    #[serde(rename = "swaggerUri", default, skip_serializing_if = "Option::is_none")]
    pub swagger_uri: Option<String>,
}
impl EndpointPropertiesBase {
    pub fn new(auth_mode: EndpointAuthMode) -> Self {
        Self {
            auth_mode,
            description: None,
            keys: None,
            properties: None,
            scoring_uri: None,
            swagger_uri: None,
        }
    }
}
#[doc = "State of endpoint provisioning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EndpointProvisioningState")]
pub enum EndpointProvisioningState {
    Creating,
    Deleting,
    Succeeded,
    Failed,
    Updating,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EndpointProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EndpointProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EndpointProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Creating => serializer.serialize_unit_variant("EndpointProvisioningState", 0u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("EndpointProvisioningState", 1u32, "Deleting"),
            Self::Succeeded => serializer.serialize_unit_variant("EndpointProvisioningState", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("EndpointProvisioningState", 3u32, "Failed"),
            Self::Updating => serializer.serialize_unit_variant("EndpointProvisioningState", 4u32, "Updating"),
            Self::Canceled => serializer.serialize_unit_variant("EndpointProvisioningState", 5u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointScheduleAction {
    #[serde(flatten)]
    pub schedule_action_base: ScheduleActionBase,
    #[doc = "[Required] Defines Schedule action definition details.\r\n<see href=\"TBD\" />"]
    #[serde(rename = "endpointInvocationDefinition")]
    pub endpoint_invocation_definition: serde_json::Value,
}
impl EndpointScheduleAction {
    pub fn new(schedule_action_base: ScheduleActionBase, endpoint_invocation_definition: serde_json::Value) -> Self {
        Self {
            schedule_action_base,
            endpoint_invocation_definition,
        }
    }
}
#[doc = "Connection status of the service consumer with the service provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EndpointServiceConnectionStatus")]
pub enum EndpointServiceConnectionStatus {
    Approved,
    Pending,
    Rejected,
    Disconnected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EndpointServiceConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EndpointServiceConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EndpointServiceConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Approved => serializer.serialize_unit_variant("EndpointServiceConnectionStatus", 0u32, "Approved"),
            Self::Pending => serializer.serialize_unit_variant("EndpointServiceConnectionStatus", 1u32, "Pending"),
            Self::Rejected => serializer.serialize_unit_variant("EndpointServiceConnectionStatus", 2u32, "Rejected"),
            Self::Disconnected => serializer.serialize_unit_variant("EndpointServiceConnectionStatus", 3u32, "Disconnected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Container for environment specification versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentContainer {
    #[serde(flatten)]
    pub asset_container: AssetContainer,
    #[doc = "Provisioning state of registry asset."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AssetProvisioningState>,
}
impl EnvironmentContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentContainerResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Container for environment specification versions."]
    pub properties: EnvironmentContainer,
}
impl EnvironmentContainerResource {
    pub fn new(properties: EnvironmentContainer) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of EnvironmentContainer entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentContainerResourceArmPaginatedResult {
    #[doc = "The link to the next page of EnvironmentContainer objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type EnvironmentContainer."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EnvironmentContainerResource>,
}
impl azure_core::Continuable for EnvironmentContainerResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EnvironmentContainerResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Environment type is either user created or curated by Azure ML service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnvironmentType")]
pub enum EnvironmentType {
    Curated,
    UserCreated,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnvironmentType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnvironmentType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnvironmentType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Curated => serializer.serialize_unit_variant("EnvironmentType", 0u32, "Curated"),
            Self::UserCreated => serializer.serialize_unit_variant("EnvironmentType", 1u32, "UserCreated"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentVariable {
    #[doc = "Type of the Environment Variable. Possible values are: local - For local variable"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<environment_variable::Type>,
    #[doc = "Value of the Environment variable"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl EnvironmentVariable {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod environment_variable {
    use super::*;
    #[doc = "Type of the Environment Variable. Possible values are: local - For local variable"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "local")]
        Local,
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
                Self::Local => serializer.serialize_unit_variant("Type", 0u32, "local"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Type {
        fn default() -> Self {
            Self::Local
        }
    }
}
#[doc = "Environment version details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentVersion {
    #[serde(flatten)]
    pub asset_base: AssetBase,
    #[doc = "AutoRebuild setting for the derived image"]
    #[serde(rename = "autoRebuild", default, skip_serializing_if = "Option::is_none")]
    pub auto_rebuild: Option<AutoRebuildSetting>,
    #[doc = "Configuration settings for Docker build context"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<BuildContext>,
    #[doc = "Standard configuration file used by Conda that lets you install any kind of package, including Python, R, and C/C++ packages.\r\n<see href=\"https://repo2docker.readthedocs.io/en/latest/config_files.html#environment-yml-install-a-conda-environment\" />"]
    #[serde(rename = "condaFile", default, skip_serializing_if = "Option::is_none")]
    pub conda_file: Option<String>,
    #[doc = "Environment type is either user created or curated by Azure ML service"]
    #[serde(rename = "environmentType", default, skip_serializing_if = "Option::is_none")]
    pub environment_type: Option<EnvironmentType>,
    #[doc = "Name of the image that will be used for the environment.\r\n<seealso href=\"https://docs.microsoft.com/en-us/azure/machine-learning/how-to-deploy-custom-docker-image#use-a-custom-base-image\" />"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "inferenceConfig", default, skip_serializing_if = "Option::is_none")]
    pub inference_config: Option<InferenceContainerProperties>,
    #[doc = "Intellectual Property details for a resource."]
    #[serde(rename = "intellectualProperty", default, skip_serializing_if = "Option::is_none")]
    pub intellectual_property: Option<IntellectualProperty>,
    #[doc = "The type of operating system."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OperatingSystemType>,
    #[doc = "Provisioning state of registry asset."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AssetProvisioningState>,
    #[doc = "Stage in the environment lifecycle assigned to this environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}
impl EnvironmentVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentVersionResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Environment version details."]
    pub properties: EnvironmentVersion,
}
impl EnvironmentVersionResource {
    pub fn new(properties: EnvironmentVersion) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of EnvironmentVersion entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentVersionResourceArmPaginatedResult {
    #[doc = "The link to the next page of EnvironmentVersion objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type EnvironmentVersion."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EnvironmentVersionResource>,
}
impl azure_core::Continuable for EnvironmentVersionResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EnvironmentVersionResourceArmPaginatedResult {
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
#[doc = "The estimated price info for using a VM of a particular OS type, tier, etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EstimatedVmPrice {
    #[doc = "The price charged for using the VM."]
    #[serde(rename = "retailPrice")]
    pub retail_price: f64,
    #[doc = "Operating system type used by the VM."]
    #[serde(rename = "osType")]
    pub os_type: estimated_vm_price::OsType,
    #[doc = "The type of the VM."]
    #[serde(rename = "vmTier")]
    pub vm_tier: estimated_vm_price::VmTier,
}
impl EstimatedVmPrice {
    pub fn new(retail_price: f64, os_type: estimated_vm_price::OsType, vm_tier: estimated_vm_price::VmTier) -> Self {
        Self {
            retail_price,
            os_type,
            vm_tier,
        }
    }
}
pub mod estimated_vm_price {
    use super::*;
    #[doc = "Operating system type used by the VM."]
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
    #[doc = "The type of the VM."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VmTier")]
    pub enum VmTier {
        Standard,
        LowPriority,
        Spot,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VmTier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VmTier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VmTier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("VmTier", 0u32, "Standard"),
                Self::LowPriority => serializer.serialize_unit_variant("VmTier", 1u32, "LowPriority"),
                Self::Spot => serializer.serialize_unit_variant("VmTier", 2u32, "Spot"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The estimated price info for using a VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EstimatedVmPrices {
    #[doc = "Three lettered code specifying the currency of the VM price. Example: USD"]
    #[serde(rename = "billingCurrency")]
    pub billing_currency: estimated_vm_prices::BillingCurrency,
    #[doc = "The unit of time measurement for the specified VM price. Example: OneHour"]
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: estimated_vm_prices::UnitOfMeasure,
    #[doc = "The list of estimated prices for using a VM of a particular OS type, tier, etc."]
    pub values: Vec<EstimatedVmPrice>,
}
impl EstimatedVmPrices {
    pub fn new(
        billing_currency: estimated_vm_prices::BillingCurrency,
        unit_of_measure: estimated_vm_prices::UnitOfMeasure,
        values: Vec<EstimatedVmPrice>,
    ) -> Self {
        Self {
            billing_currency,
            unit_of_measure,
            values,
        }
    }
}
pub mod estimated_vm_prices {
    use super::*;
    #[doc = "Three lettered code specifying the currency of the VM price. Example: USD"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingCurrency")]
    pub enum BillingCurrency {
        #[serde(rename = "USD")]
        Usd,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingCurrency {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingCurrency {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingCurrency {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Usd => serializer.serialize_unit_variant("BillingCurrency", 0u32, "USD"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The unit of time measurement for the specified VM price. Example: OneHour"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UnitOfMeasure")]
    pub enum UnitOfMeasure {
        OneHour,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UnitOfMeasure {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UnitOfMeasure {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UnitOfMeasure {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::OneHour => serializer.serialize_unit_variant("UnitOfMeasure", 0u32, "OneHour"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The format of exported labels."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ExportFormatType")]
pub enum ExportFormatType {
    Dataset,
    Coco,
    #[serde(rename = "CSV")]
    Csv,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ExportFormatType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ExportFormatType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ExportFormatType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Dataset => serializer.serialize_unit_variant("ExportFormatType", 0u32, "Dataset"),
            Self::Coco => serializer.serialize_unit_variant("ExportFormatType", 1u32, "Coco"),
            Self::Csv => serializer.serialize_unit_variant("ExportFormatType", 2u32, "CSV"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportSummary {
    #[doc = "The time when the export was completed."]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "The total number of labeled datapoints exported."]
    #[serde(rename = "exportedRowCount", default, skip_serializing_if = "Option::is_none")]
    pub exported_row_count: Option<i64>,
    #[doc = "The format of exported labels."]
    pub format: ExportFormatType,
    #[doc = "Name and identifier of the job containing exported labels."]
    #[serde(rename = "labelingJobId", default, skip_serializing_if = "Option::is_none")]
    pub labeling_job_id: Option<String>,
    #[doc = "The time when the export was requested."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
}
impl ExportSummary {
    pub fn new(format: ExportFormatType) -> Self {
        Self {
            end_date_time: None,
            exported_row_count: None,
            format,
            labeling_job_id: None,
            start_date_time: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalFqdnResponse {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FqdnEndpoints>,
}
impl ExternalFqdnResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FqdnEndpoint {
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(
        rename = "endpointDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endpoint_details: Vec<FqdnEndpointDetail>,
}
impl FqdnEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FqdnEndpointDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
impl FqdnEndpointDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FqdnEndpoints {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FqdnEndpointsProperties>,
}
impl FqdnEndpoints {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FqdnEndpointsProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endpoints: Vec<FqdnEndpoint>,
}
impl FqdnEndpointsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dto object representing feature"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Feature {
    #[serde(flatten)]
    pub resource_base: ResourceBase,
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<FeatureDataType>,
    #[doc = "Specifies name"]
    #[serde(rename = "featureName", default, skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
}
impl Feature {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeatureAttributionDriftMonitoringSignal {
    #[serde(flatten)]
    pub monitoring_signal_base: MonitoringSignalBase,
    #[serde(rename = "baselineData")]
    pub baseline_data: MonitoringInputData,
    #[serde(rename = "metricThreshold")]
    pub metric_threshold: FeatureAttributionMetricThreshold,
    #[serde(rename = "modelType")]
    pub model_type: MonitoringModelType,
    #[serde(rename = "targetData")]
    pub target_data: MonitoringInputData,
}
impl FeatureAttributionDriftMonitoringSignal {
    pub fn new(
        monitoring_signal_base: MonitoringSignalBase,
        baseline_data: MonitoringInputData,
        metric_threshold: FeatureAttributionMetricThreshold,
        model_type: MonitoringModelType,
        target_data: MonitoringInputData,
    ) -> Self {
        Self {
            monitoring_signal_base,
            baseline_data,
            metric_threshold,
            model_type,
            target_data,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FeatureAttributionMetric")]
pub enum FeatureAttributionMetric {
    NormalizedDiscountedCumulativeGain,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FeatureAttributionMetric {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FeatureAttributionMetric {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FeatureAttributionMetric {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NormalizedDiscountedCumulativeGain => {
                serializer.serialize_unit_variant("FeatureAttributionMetric", 0u32, "NormalizedDiscountedCumulativeGain")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeatureAttributionMetricThreshold {
    pub metric: FeatureAttributionMetric,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<MonitoringThreshold>,
}
impl FeatureAttributionMetricThreshold {
    pub fn new(metric: FeatureAttributionMetric) -> Self {
        Self { metric, threshold: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FeatureDataType")]
pub enum FeatureDataType {
    String,
    Integer,
    Long,
    Float,
    Double,
    Binary,
    Datetime,
    Boolean,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FeatureDataType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FeatureDataType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FeatureDataType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::String => serializer.serialize_unit_variant("FeatureDataType", 0u32, "String"),
            Self::Integer => serializer.serialize_unit_variant("FeatureDataType", 1u32, "Integer"),
            Self::Long => serializer.serialize_unit_variant("FeatureDataType", 2u32, "Long"),
            Self::Float => serializer.serialize_unit_variant("FeatureDataType", 3u32, "Float"),
            Self::Double => serializer.serialize_unit_variant("FeatureDataType", 4u32, "Double"),
            Self::Binary => serializer.serialize_unit_variant("FeatureDataType", 5u32, "Binary"),
            Self::Datetime => serializer.serialize_unit_variant("FeatureDataType", 6u32, "Datetime"),
            Self::Boolean => serializer.serialize_unit_variant("FeatureDataType", 7u32, "Boolean"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Flag for generating lags for the numeric features."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FeatureLags")]
pub enum FeatureLags {
    None,
    Auto,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FeatureLags {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FeatureLags {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FeatureLags {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("FeatureLags", 0u32, "None"),
            Self::Auto => serializer.serialize_unit_variant("FeatureLags", 1u32, "Auto"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeatureResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Dto object representing feature"]
    pub properties: Feature,
}
impl FeatureResource {
    pub fn new(properties: Feature) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of Feature entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeatureResourceArmPaginatedResult {
    #[doc = "The link to the next page of Feature objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type Feature."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FeatureResource>,
}
impl azure_core::Continuable for FeatureResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FeatureResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeatureStoreSettings {
    #[serde(rename = "computeRuntime", default, skip_serializing_if = "Option::is_none")]
    pub compute_runtime: Option<ComputeRuntimeDto>,
    #[serde(rename = "offlineStoreConnectionName", default, skip_serializing_if = "Option::is_none")]
    pub offline_store_connection_name: Option<String>,
    #[serde(rename = "onlineStoreConnectionName", default, skip_serializing_if = "Option::is_none")]
    pub online_store_connection_name: Option<String>,
}
impl FeatureStoreSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeatureSubset {
    #[serde(flatten)]
    pub monitoring_feature_filter_base: MonitoringFeatureFilterBase,
    #[doc = "[Required] The list of features to include."]
    pub features: Vec<String>,
}
impl FeatureSubset {
    pub fn new(monitoring_feature_filter_base: MonitoringFeatureFilterBase, features: Vec<String>) -> Self {
        Self {
            monitoring_feature_filter_base,
            features,
        }
    }
}
#[doc = "Specifies the feature window"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeatureWindow {
    #[doc = "Specifies the feature window end time"]
    #[serde(rename = "featureWindowEnd", default, with = "azure_core::date::rfc3339::option")]
    pub feature_window_end: Option<time::OffsetDateTime>,
    #[doc = "Specifies the feature window start time"]
    #[serde(rename = "featureWindowStart", default, with = "azure_core::date::rfc3339::option")]
    pub feature_window_start: Option<time::OffsetDateTime>,
}
impl FeatureWindow {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dto object representing feature set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeaturesetContainer {
    #[serde(flatten)]
    pub asset_container: AssetContainer,
    #[doc = "Provisioning state of registry asset."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AssetProvisioningState>,
}
impl FeaturesetContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeaturesetContainerResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Dto object representing feature set"]
    pub properties: FeaturesetContainer,
}
impl FeaturesetContainerResource {
    pub fn new(properties: FeaturesetContainer) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of FeaturesetContainer entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeaturesetContainerResourceArmPaginatedResult {
    #[doc = "The link to the next page of FeaturesetContainer objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type FeaturesetContainer."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FeaturesetContainerResource>,
}
impl azure_core::Continuable for FeaturesetContainerResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FeaturesetContainerResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dto object representing the feature set job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeaturesetJob {
    #[doc = "Specifies the created date"]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Specifies the display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Specifies the duration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "Specifies the experiment id"]
    #[serde(rename = "experimentId", default, skip_serializing_if = "Option::is_none")]
    pub experiment_id: Option<String>,
    #[doc = "Specifies the feature window"]
    #[serde(rename = "featureWindow", default, skip_serializing_if = "Option::is_none")]
    pub feature_window: Option<FeatureWindow>,
    #[doc = "Specifies the job id"]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "The status of a job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,
    #[doc = "Specifies the tags if any"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<FeaturestoreJobType>,
}
impl FeaturesetJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paginated list of FeaturesetJob entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeaturesetJobArmPaginatedResult {
    #[doc = "The link to the next page of FeaturesetJob objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type FeaturesetJob."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FeaturesetJob>,
}
impl azure_core::Continuable for FeaturesetJobArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FeaturesetJobArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dto object representing specification"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeaturesetSpecification {
    #[doc = "Specifies the spec path"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl FeaturesetSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dto object representing feature set version"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeaturesetVersion {
    #[serde(flatten)]
    pub asset_base: AssetBase,
    #[doc = "Specifies list of entities"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entities: Vec<String>,
    #[serde(rename = "materializationSettings", default, skip_serializing_if = "Option::is_none")]
    pub materialization_settings: Option<MaterializationSettings>,
    #[doc = "Provisioning state of registry asset."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AssetProvisioningState>,
    #[doc = "Dto object representing specification"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specification: Option<FeaturesetSpecification>,
    #[doc = "Specifies the asset stage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}
impl FeaturesetVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request payload for creating a backfill request for a given feature set version"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeaturesetVersionBackfillRequest {
    #[doc = "Specifies description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Specifies description"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Specifies the feature window"]
    #[serde(rename = "featureWindow", default, skip_serializing_if = "Option::is_none")]
    pub feature_window: Option<FeatureWindow>,
    #[doc = "Dto object representing compute resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<MaterializationComputeResource>,
    #[doc = "Specifies the spark compute settings"]
    #[serde(rename = "sparkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub spark_configuration: Option<serde_json::Value>,
    #[doc = "Specifies the tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl FeaturesetVersionBackfillRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeaturesetVersionResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Dto object representing feature set version"]
    pub properties: FeaturesetVersion,
}
impl FeaturesetVersionResource {
    pub fn new(properties: FeaturesetVersion) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of FeaturesetVersion entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeaturesetVersionResourceArmPaginatedResult {
    #[doc = "The link to the next page of FeaturesetVersion objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type FeaturesetVersion."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FeaturesetVersionResource>,
}
impl azure_core::Continuable for FeaturesetVersionResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FeaturesetVersionResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dto object representing feature entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeaturestoreEntityContainer {
    #[serde(flatten)]
    pub asset_container: AssetContainer,
    #[doc = "Provisioning state of registry asset."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AssetProvisioningState>,
}
impl FeaturestoreEntityContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeaturestoreEntityContainerResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Dto object representing feature entity"]
    pub properties: FeaturestoreEntityContainer,
}
impl FeaturestoreEntityContainerResource {
    pub fn new(properties: FeaturestoreEntityContainer) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of FeaturestoreEntityContainer entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeaturestoreEntityContainerResourceArmPaginatedResult {
    #[doc = "The link to the next page of FeaturestoreEntityContainer objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type FeaturestoreEntityContainer."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FeaturestoreEntityContainerResource>,
}
impl azure_core::Continuable for FeaturestoreEntityContainerResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FeaturestoreEntityContainerResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dto object representing feature entity version"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeaturestoreEntityVersion {
    #[serde(flatten)]
    pub asset_base: AssetBase,
    #[doc = "Specifies index columns"]
    #[serde(
        rename = "indexColumns",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub index_columns: Vec<IndexColumn>,
    #[doc = "Provisioning state of registry asset."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AssetProvisioningState>,
    #[doc = "Specifies the asset stage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}
impl FeaturestoreEntityVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeaturestoreEntityVersionResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Dto object representing feature entity version"]
    pub properties: FeaturestoreEntityVersion,
}
impl FeaturestoreEntityVersionResource {
    pub fn new(properties: FeaturestoreEntityVersion) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of FeaturestoreEntityVersion entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeaturestoreEntityVersionResourceArmPaginatedResult {
    #[doc = "The link to the next page of FeaturestoreEntityVersion objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type FeaturestoreEntityVersion."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FeaturestoreEntityVersionResource>,
}
impl azure_core::Continuable for FeaturestoreEntityVersionResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FeaturestoreEntityVersionResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FeaturestoreJobType")]
pub enum FeaturestoreJobType {
    RecurrentMaterialization,
    BackfillMaterialization,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FeaturestoreJobType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FeaturestoreJobType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FeaturestoreJobType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::RecurrentMaterialization => serializer.serialize_unit_variant("FeaturestoreJobType", 0u32, "RecurrentMaterialization"),
            Self::BackfillMaterialization => serializer.serialize_unit_variant("FeaturestoreJobType", 1u32, "BackfillMaterialization"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Featurization mode - determines data featurization mode."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FeaturizationMode")]
pub enum FeaturizationMode {
    Auto,
    Custom,
    Off,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FeaturizationMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FeaturizationMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FeaturizationMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Auto => serializer.serialize_unit_variant("FeaturizationMode", 0u32, "Auto"),
            Self::Custom => serializer.serialize_unit_variant("FeaturizationMode", 1u32, "Custom"),
            Self::Off => serializer.serialize_unit_variant("FeaturizationMode", 2u32, "Off"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Featurization Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeaturizationSettings {
    #[doc = "Dataset language, useful for the text data."]
    #[serde(rename = "datasetLanguage", default, skip_serializing_if = "Option::is_none")]
    pub dataset_language: Option<String>,
}
impl FeaturizationSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileSystemSource {
    #[serde(flatten)]
    pub data_import_source: DataImportSource,
    #[doc = "Path on data import FileSystem source"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl FileSystemSource {
    pub fn new(data_import_source: DataImportSource) -> Self {
        Self {
            data_import_source,
            path: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlavorData {
    #[doc = "Model flavor-specific data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
impl FlavorData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The desired maximum forecast horizon in units of time-series frequency."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForecastHorizon {
    #[doc = "Enum to determine forecast horizon selection mode."]
    pub mode: ForecastHorizonMode,
}
impl ForecastHorizon {
    pub fn new(mode: ForecastHorizonMode) -> Self {
        Self { mode }
    }
}
#[doc = "Enum to determine forecast horizon selection mode."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ForecastHorizonMode")]
pub enum ForecastHorizonMode {
    Auto,
    Custom,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ForecastHorizonMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ForecastHorizonMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ForecastHorizonMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Auto => serializer.serialize_unit_variant("ForecastHorizonMode", 0u32, "Auto"),
            Self::Custom => serializer.serialize_unit_variant("ForecastHorizonMode", 1u32, "Custom"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Forecasting task in AutoML Table vertical."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Forecasting {
    #[serde(flatten)]
    pub table_vertical: TableVertical,
    #[serde(flatten)]
    pub auto_ml_vertical: AutoMlVertical,
    #[doc = "Forecasting specific parameters."]
    #[serde(rename = "forecastingSettings", default, skip_serializing_if = "Option::is_none")]
    pub forecasting_settings: Option<ForecastingSettings>,
    #[doc = "Primary metrics for Forecasting task."]
    #[serde(rename = "primaryMetric", default, skip_serializing_if = "Option::is_none")]
    pub primary_metric: Option<ForecastingPrimaryMetrics>,
    #[doc = "Forecasting Training related configuration."]
    #[serde(rename = "trainingSettings", default, skip_serializing_if = "Option::is_none")]
    pub training_settings: Option<ForecastingTrainingSettings>,
}
impl Forecasting {
    pub fn new(auto_ml_vertical: AutoMlVertical) -> Self {
        Self {
            table_vertical: TableVertical::default(),
            auto_ml_vertical,
            forecasting_settings: None,
            primary_metric: None,
            training_settings: None,
        }
    }
}
#[doc = "Enum for all forecasting models supported by AutoML."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ForecastingModels")]
pub enum ForecastingModels {
    AutoArima,
    Prophet,
    Naive,
    SeasonalNaive,
    Average,
    SeasonalAverage,
    ExponentialSmoothing,
    Arimax,
    #[serde(rename = "TCNForecaster")]
    TcnForecaster,
    ElasticNet,
    GradientBoosting,
    DecisionTree,
    #[serde(rename = "KNN")]
    Knn,
    LassoLars,
    #[serde(rename = "SGD")]
    Sgd,
    RandomForest,
    ExtremeRandomTrees,
    #[serde(rename = "LightGBM")]
    LightGbm,
    #[serde(rename = "XGBoostRegressor")]
    XgBoostRegressor,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ForecastingModels {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ForecastingModels {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ForecastingModels {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AutoArima => serializer.serialize_unit_variant("ForecastingModels", 0u32, "AutoArima"),
            Self::Prophet => serializer.serialize_unit_variant("ForecastingModels", 1u32, "Prophet"),
            Self::Naive => serializer.serialize_unit_variant("ForecastingModels", 2u32, "Naive"),
            Self::SeasonalNaive => serializer.serialize_unit_variant("ForecastingModels", 3u32, "SeasonalNaive"),
            Self::Average => serializer.serialize_unit_variant("ForecastingModels", 4u32, "Average"),
            Self::SeasonalAverage => serializer.serialize_unit_variant("ForecastingModels", 5u32, "SeasonalAverage"),
            Self::ExponentialSmoothing => serializer.serialize_unit_variant("ForecastingModels", 6u32, "ExponentialSmoothing"),
            Self::Arimax => serializer.serialize_unit_variant("ForecastingModels", 7u32, "Arimax"),
            Self::TcnForecaster => serializer.serialize_unit_variant("ForecastingModels", 8u32, "TCNForecaster"),
            Self::ElasticNet => serializer.serialize_unit_variant("ForecastingModels", 9u32, "ElasticNet"),
            Self::GradientBoosting => serializer.serialize_unit_variant("ForecastingModels", 10u32, "GradientBoosting"),
            Self::DecisionTree => serializer.serialize_unit_variant("ForecastingModels", 11u32, "DecisionTree"),
            Self::Knn => serializer.serialize_unit_variant("ForecastingModels", 12u32, "KNN"),
            Self::LassoLars => serializer.serialize_unit_variant("ForecastingModels", 13u32, "LassoLars"),
            Self::Sgd => serializer.serialize_unit_variant("ForecastingModels", 14u32, "SGD"),
            Self::RandomForest => serializer.serialize_unit_variant("ForecastingModels", 15u32, "RandomForest"),
            Self::ExtremeRandomTrees => serializer.serialize_unit_variant("ForecastingModels", 16u32, "ExtremeRandomTrees"),
            Self::LightGbm => serializer.serialize_unit_variant("ForecastingModels", 17u32, "LightGBM"),
            Self::XgBoostRegressor => serializer.serialize_unit_variant("ForecastingModels", 18u32, "XGBoostRegressor"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Primary metrics for Forecasting task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ForecastingPrimaryMetrics")]
pub enum ForecastingPrimaryMetrics {
    SpearmanCorrelation,
    NormalizedRootMeanSquaredError,
    R2Score,
    NormalizedMeanAbsoluteError,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ForecastingPrimaryMetrics {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ForecastingPrimaryMetrics {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ForecastingPrimaryMetrics {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SpearmanCorrelation => serializer.serialize_unit_variant("ForecastingPrimaryMetrics", 0u32, "SpearmanCorrelation"),
            Self::NormalizedRootMeanSquaredError => {
                serializer.serialize_unit_variant("ForecastingPrimaryMetrics", 1u32, "NormalizedRootMeanSquaredError")
            }
            Self::R2Score => serializer.serialize_unit_variant("ForecastingPrimaryMetrics", 2u32, "R2Score"),
            Self::NormalizedMeanAbsoluteError => {
                serializer.serialize_unit_variant("ForecastingPrimaryMetrics", 3u32, "NormalizedMeanAbsoluteError")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Forecasting specific parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForecastingSettings {
    #[doc = "Country or region for holidays for forecasting tasks.\r\nThese should be ISO 3166 two-letter country/region codes, for example 'US' or 'GB'."]
    #[serde(rename = "countryOrRegionForHolidays", default, skip_serializing_if = "Option::is_none")]
    pub country_or_region_for_holidays: Option<String>,
    #[doc = "Number of periods between the origin time of one CV fold and the next fold. For\r\nexample, if `CVStepSize` = 3 for daily data, the origin time for each fold will be\r\nthree days apart."]
    #[serde(rename = "cvStepSize", default, skip_serializing_if = "Option::is_none")]
    pub cv_step_size: Option<i32>,
    #[doc = "Flag for generating lags for the numeric features."]
    #[serde(rename = "featureLags", default, skip_serializing_if = "Option::is_none")]
    pub feature_lags: Option<FeatureLags>,
    #[doc = "The feature columns that are available for training but unknown at the time of forecast/inference.\r\nIf features_unknown_at_forecast_time is not set, it is assumed that all the feature columns in the dataset are known at inference time."]
    #[serde(
        rename = "featuresUnknownAtForecastTime",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub features_unknown_at_forecast_time: Vec<String>,
    #[doc = "The desired maximum forecast horizon in units of time-series frequency."]
    #[serde(rename = "forecastHorizon", default, skip_serializing_if = "Option::is_none")]
    pub forecast_horizon: Option<ForecastHorizon>,
    #[doc = "When forecasting, this parameter represents the period with which the forecast is desired, for example daily, weekly, yearly, etc. The forecast frequency is dataset frequency by default."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[doc = "Forecasting seasonality."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seasonality: Option<Seasonality>,
    #[doc = "The parameter defining how if AutoML should handle short time series."]
    #[serde(rename = "shortSeriesHandlingConfig", default, skip_serializing_if = "Option::is_none")]
    pub short_series_handling_config: Option<ShortSeriesHandlingConfiguration>,
    #[doc = "Target aggregate function."]
    #[serde(rename = "targetAggregateFunction", default, skip_serializing_if = "Option::is_none")]
    pub target_aggregate_function: Option<TargetAggregationFunction>,
    #[doc = "The number of past periods to lag from the target column."]
    #[serde(rename = "targetLags", default, skip_serializing_if = "Option::is_none")]
    pub target_lags: Option<TargetLags>,
    #[doc = "Forecasting target rolling window size."]
    #[serde(rename = "targetRollingWindowSize", default, skip_serializing_if = "Option::is_none")]
    pub target_rolling_window_size: Option<TargetRollingWindowSize>,
    #[doc = "The name of the time column. This parameter is required when forecasting to specify the datetime column in the input data used for building the time series and inferring its frequency."]
    #[serde(rename = "timeColumnName", default, skip_serializing_if = "Option::is_none")]
    pub time_column_name: Option<String>,
    #[doc = "The names of columns used to group a timeseries. It can be used to create multiple series.\r\nIf grain is not defined, the data set is assumed to be one time-series. This parameter is used with task type forecasting."]
    #[serde(
        rename = "timeSeriesIdColumnNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub time_series_id_column_names: Vec<String>,
    #[doc = "Configure STL Decomposition of the time-series target column."]
    #[serde(rename = "useStl", default, skip_serializing_if = "Option::is_none")]
    pub use_stl: Option<UseStl>,
}
impl ForecastingSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Forecasting Training related configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForecastingTrainingSettings {
    #[serde(flatten)]
    pub training_settings: TrainingSettings,
    #[doc = "Allowed models for forecasting task."]
    #[serde(
        rename = "allowedTrainingAlgorithms",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_training_algorithms: Vec<ForecastingModels>,
    #[doc = "Blocked models for forecasting task."]
    #[serde(
        rename = "blockedTrainingAlgorithms",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_training_algorithms: Vec<ForecastingModels>,
}
impl ForecastingTrainingSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "FQDN Outbound Rule for the managed network of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FqdnOutboundRule {
    #[serde(flatten)]
    pub outbound_rule: OutboundRule,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}
impl FqdnOutboundRule {
    pub fn new(outbound_rule: OutboundRule) -> Self {
        Self {
            outbound_rule,
            destination: None,
        }
    }
}
#[doc = "Defines supported metric goals for hyperparameter tuning"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Goal")]
pub enum Goal {
    Minimize,
    Maximize,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Goal {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Goal {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Goal {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Minimize => serializer.serialize_unit_variant("Goal", 0u32, "Minimize"),
            Self::Maximize => serializer.serialize_unit_variant("Goal", 1u32, "Maximize"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines a Sampling Algorithm that exhaustively generates every value combination in the space"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridSamplingAlgorithm {
    #[serde(flatten)]
    pub sampling_algorithm: SamplingAlgorithm,
}
impl GridSamplingAlgorithm {
    pub fn new(sampling_algorithm: SamplingAlgorithm) -> Self {
        Self { sampling_algorithm }
    }
}
#[doc = "A HDInsight compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HdInsight {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(flatten)]
    pub hd_insight_schema: HdInsightSchema,
}
impl HdInsight {
    pub fn new(compute: Compute) -> Self {
        Self {
            compute,
            hd_insight_schema: HdInsightSchema::default(),
        }
    }
}
#[doc = "HDInsight compute properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HdInsightProperties {
    #[doc = "Port open for ssh connections on the master node of the cluster."]
    #[serde(rename = "sshPort", default, skip_serializing_if = "Option::is_none")]
    pub ssh_port: Option<i32>,
    #[doc = "Public IP address of the master node of the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "Admin credentials for virtual machine"]
    #[serde(rename = "administratorAccount", default, skip_serializing_if = "Option::is_none")]
    pub administrator_account: Option<VirtualMachineSshCredentials>,
}
impl HdInsightProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HdInsightSchema {
    #[doc = "HDInsight compute properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HdInsightProperties>,
}
impl HdInsightSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HdfsDatastore {
    #[serde(flatten)]
    pub datastore: Datastore,
    #[doc = "The TLS cert of the HDFS server. Needs to be a base64 encoded string. Required if \"Https\" protocol is selected."]
    #[serde(rename = "hdfsServerCertificate", default, skip_serializing_if = "Option::is_none")]
    pub hdfs_server_certificate: Option<String>,
    #[doc = "[Required] IP Address or DNS HostName."]
    #[serde(rename = "nameNodeAddress")]
    pub name_node_address: String,
    #[doc = "Protocol used to communicate with the storage account (Https/Http)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}
impl HdfsDatastore {
    pub fn new(datastore: Datastore, name_node_address: String) -> Self {
        Self {
            datastore,
            hdfs_server_certificate: None,
            name_node_address,
            protocol: None,
        }
    }
}
#[doc = "Reference to an asset via its ARM resource ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdAssetReference {
    #[serde(flatten)]
    pub asset_reference_base: AssetReferenceBase,
    #[doc = "[Required] ARM resource ID of the asset."]
    #[serde(rename = "assetId")]
    pub asset_id: String,
}
impl IdAssetReference {
    pub fn new(asset_reference_base: AssetReferenceBase, asset_id: String) -> Self {
        Self {
            asset_reference_base,
            asset_id,
        }
    }
}
#[doc = "Base definition for identity configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityConfiguration {
    #[doc = "Enum to determine identity framework."]
    #[serde(rename = "identityType")]
    pub identity_type: IdentityConfigurationType,
}
impl IdentityConfiguration {
    pub fn new(identity_type: IdentityConfigurationType) -> Self {
        Self { identity_type }
    }
}
#[doc = "Enum to determine identity framework."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IdentityConfigurationType")]
pub enum IdentityConfigurationType {
    Managed,
    #[serde(rename = "AMLToken")]
    AmlToken,
    UserIdentity,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IdentityConfigurationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IdentityConfigurationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IdentityConfigurationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Managed => serializer.serialize_unit_variant("IdentityConfigurationType", 0u32, "Managed"),
            Self::AmlToken => serializer.serialize_unit_variant("IdentityConfigurationType", 1u32, "AMLToken"),
            Self::UserIdentity => serializer.serialize_unit_variant("IdentityConfigurationType", 2u32, "UserIdentity"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Identity that will be used to access key vault for encryption at rest"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityForCmk {
    #[doc = "The ArmId of the user assigned identity that will be used to access the customer managed key vault"]
    #[serde(rename = "userAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identity: Option<String>,
}
impl IdentityForCmk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Stops compute instance after user defined period of inactivity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdleShutdownSetting {
    #[doc = "Time is defined in ISO8601 format. Minimum is 15 min, maximum is 3 days."]
    #[serde(rename = "idleTimeBeforeShutdown", default, skip_serializing_if = "Option::is_none")]
    pub idle_time_before_shutdown: Option<String>,
}
impl IdleShutdownSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Image {
    #[doc = "Type of the image. Possible values are: docker - For docker images. azureml - For AzureML images"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<image::Type>,
    #[doc = "Image reference URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
impl Image {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod image {
    use super::*;
    #[doc = "Type of the image. Possible values are: docker - For docker images. azureml - For AzureML images"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "docker")]
        Docker,
        #[serde(rename = "azureml")]
        Azureml,
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
                Self::Docker => serializer.serialize_unit_variant("Type", 0u32, "docker"),
                Self::Azureml => serializer.serialize_unit_variant("Type", 1u32, "azureml"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Type {
        fn default() -> Self {
            Self::Docker
        }
    }
}
#[doc = "Annotation type of image data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ImageAnnotationType")]
pub enum ImageAnnotationType {
    Classification,
    BoundingBox,
    InstanceSegmentation,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ImageAnnotationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ImageAnnotationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ImageAnnotationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Classification => serializer.serialize_unit_variant("ImageAnnotationType", 0u32, "Classification"),
            Self::BoundingBox => serializer.serialize_unit_variant("ImageAnnotationType", 1u32, "BoundingBox"),
            Self::InstanceSegmentation => serializer.serialize_unit_variant("ImageAnnotationType", 2u32, "InstanceSegmentation"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Image Classification. Multi-class image classification is used when an image is classified with only a single label\r\nfrom a set of classes - e.g. each image is classified as either an image of a 'cat' or a 'dog' or a 'duck'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageClassification {
    #[serde(flatten)]
    pub image_classification_base: ImageClassificationBase,
    #[serde(flatten)]
    pub auto_ml_vertical: AutoMlVertical,
    #[doc = "Primary metrics for classification tasks."]
    #[serde(rename = "primaryMetric", default, skip_serializing_if = "Option::is_none")]
    pub primary_metric: Option<ClassificationPrimaryMetrics>,
}
impl ImageClassification {
    pub fn new(image_classification_base: ImageClassificationBase, auto_ml_vertical: AutoMlVertical) -> Self {
        Self {
            image_classification_base,
            auto_ml_vertical,
            primary_metric: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageClassificationBase {
    #[serde(flatten)]
    pub image_vertical: ImageVertical,
    #[doc = "Settings used for training the model.\r\nFor more information on the available settings please visit the official documentation:\r\nhttps://docs.microsoft.com/en-us/azure/machine-learning/how-to-auto-train-image-models."]
    #[serde(rename = "modelSettings", default, skip_serializing_if = "Option::is_none")]
    pub model_settings: Option<ImageModelSettingsClassification>,
    #[doc = "Search space for sampling different combinations of models and their hyperparameters."]
    #[serde(
        rename = "searchSpace",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub search_space: Vec<ImageModelDistributionSettingsClassification>,
}
impl ImageClassificationBase {
    pub fn new(image_vertical: ImageVertical) -> Self {
        Self {
            image_vertical,
            model_settings: None,
            search_space: Vec::new(),
        }
    }
}
#[doc = "Image Classification Multilabel. Multi-label image classification is used when an image could have one or more labels\r\nfrom a set of labels - e.g. an image could be labeled with both 'cat' and 'dog'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageClassificationMultilabel {
    #[serde(flatten)]
    pub image_classification_base: ImageClassificationBase,
    #[serde(flatten)]
    pub auto_ml_vertical: AutoMlVertical,
    #[doc = "Primary metrics for classification multilabel tasks."]
    #[serde(rename = "primaryMetric", default, skip_serializing_if = "Option::is_none")]
    pub primary_metric: Option<ClassificationMultilabelPrimaryMetrics>,
}
impl ImageClassificationMultilabel {
    pub fn new(image_classification_base: ImageClassificationBase, auto_ml_vertical: AutoMlVertical) -> Self {
        Self {
            image_classification_base,
            auto_ml_vertical,
            primary_metric: None,
        }
    }
}
#[doc = "Image Instance Segmentation. Instance segmentation is used to identify objects in an image at the pixel level,\r\ndrawing a polygon around each object in the image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageInstanceSegmentation {
    #[serde(flatten)]
    pub image_object_detection_base: ImageObjectDetectionBase,
    #[serde(flatten)]
    pub auto_ml_vertical: AutoMlVertical,
    #[doc = "Primary metrics for InstanceSegmentation tasks."]
    #[serde(rename = "primaryMetric", default, skip_serializing_if = "Option::is_none")]
    pub primary_metric: Option<InstanceSegmentationPrimaryMetrics>,
}
impl ImageInstanceSegmentation {
    pub fn new(image_object_detection_base: ImageObjectDetectionBase, auto_ml_vertical: AutoMlVertical) -> Self {
        Self {
            image_object_detection_base,
            auto_ml_vertical,
            primary_metric: None,
        }
    }
}
#[doc = "Limit settings for the AutoML job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageLimitSettings {
    #[doc = "Maximum number of concurrent AutoML iterations."]
    #[serde(rename = "maxConcurrentTrials", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_trials: Option<i32>,
    #[doc = "Maximum number of AutoML iterations."]
    #[serde(rename = "maxTrials", default, skip_serializing_if = "Option::is_none")]
    pub max_trials: Option<i32>,
    #[doc = "AutoML job timeout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}
impl ImageLimitSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Returns metadata about the operating system image for this compute instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageMetadata {
    #[doc = "Specifies the current operating system image version this compute instance is running on."]
    #[serde(rename = "currentImageVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_image_version: Option<String>,
    #[doc = "Specifies the latest available operating system image version."]
    #[serde(rename = "latestImageVersion", default, skip_serializing_if = "Option::is_none")]
    pub latest_image_version: Option<String>,
    #[doc = "Specifies whether this compute instance is running on the latest operating system image."]
    #[serde(rename = "isLatestOsImageVersion", default, skip_serializing_if = "Option::is_none")]
    pub is_latest_os_image_version: Option<bool>,
}
impl ImageMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Distribution expressions to sweep over values of model settings.\r\n<example>\r\nSome examples are:\r\n```\r\nModelName = \"choice('seresnext', 'resnest50')\";\r\nLearningRate = \"uniform(0.001, 0.01)\";\r\nLayersToFreeze = \"choice(0, 2)\";\r\n```</example>\r\nAll distributions can be specified as distribution_name(min, max) or choice(val1, val2, ..., valn)\r\nwhere distribution name can be: uniform, quniform, loguniform, etc\r\nFor more details on how to compose distribution expressions please check the documentation:\r\nhttps://docs.microsoft.com/en-us/azure/machine-learning/how-to-tune-hyperparameters\r\nFor more information on the available settings please visit the official documentation:\r\nhttps://docs.microsoft.com/en-us/azure/machine-learning/how-to-auto-train-image-models."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageModelDistributionSettings {
    #[doc = "Enable AMSGrad when optimizer is 'adam' or 'adamw'."]
    #[serde(rename = "amsGradient", default, skip_serializing_if = "Option::is_none")]
    pub ams_gradient: Option<String>,
    #[doc = "Settings for using Augmentations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub augmentations: Option<String>,
    #[doc = "Value of 'beta1' when optimizer is 'adam' or 'adamw'. Must be a float in the range [0, 1]."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beta1: Option<String>,
    #[doc = "Value of 'beta2' when optimizer is 'adam' or 'adamw'. Must be a float in the range [0, 1]."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beta2: Option<String>,
    #[doc = "Whether to use distributer training."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distributed: Option<String>,
    #[doc = "Enable early stopping logic during training."]
    #[serde(rename = "earlyStopping", default, skip_serializing_if = "Option::is_none")]
    pub early_stopping: Option<String>,
    #[doc = "Minimum number of epochs or validation evaluations to wait before primary metric improvement\r\nis tracked for early stopping. Must be a positive integer."]
    #[serde(rename = "earlyStoppingDelay", default, skip_serializing_if = "Option::is_none")]
    pub early_stopping_delay: Option<String>,
    #[doc = "Minimum number of epochs or validation evaluations with no primary metric improvement before\r\nthe run is stopped. Must be a positive integer."]
    #[serde(rename = "earlyStoppingPatience", default, skip_serializing_if = "Option::is_none")]
    pub early_stopping_patience: Option<String>,
    #[doc = "Enable normalization when exporting ONNX model."]
    #[serde(rename = "enableOnnxNormalization", default, skip_serializing_if = "Option::is_none")]
    pub enable_onnx_normalization: Option<String>,
    #[doc = "Frequency to evaluate validation dataset to get metric scores. Must be a positive integer."]
    #[serde(rename = "evaluationFrequency", default, skip_serializing_if = "Option::is_none")]
    pub evaluation_frequency: Option<String>,
    #[doc = "Gradient accumulation means running a configured number of \"GradAccumulationStep\" steps without\r\nupdating the model weights while accumulating the gradients of those steps, and then using\r\nthe accumulated gradients to compute the weight updates. Must be a positive integer."]
    #[serde(rename = "gradientAccumulationStep", default, skip_serializing_if = "Option::is_none")]
    pub gradient_accumulation_step: Option<String>,
    #[doc = "Number of layers to freeze for the model. Must be a positive integer.\r\nFor instance, passing 2 as value for 'seresnext' means\r\nfreezing layer0 and layer1. For a full list of models supported and details on layer freeze, please\r\nsee: https://docs.microsoft.com/en-us/azure/machine-learning/how-to-auto-train-image-models."]
    #[serde(rename = "layersToFreeze", default, skip_serializing_if = "Option::is_none")]
    pub layers_to_freeze: Option<String>,
    #[doc = "Initial learning rate. Must be a float in the range [0, 1]."]
    #[serde(rename = "learningRate", default, skip_serializing_if = "Option::is_none")]
    pub learning_rate: Option<String>,
    #[doc = "Type of learning rate scheduler. Must be 'warmup_cosine' or 'step'."]
    #[serde(rename = "learningRateScheduler", default, skip_serializing_if = "Option::is_none")]
    pub learning_rate_scheduler: Option<String>,
    #[doc = "Name of the model to use for training.\r\nFor more information on the available models please visit the official documentation:\r\nhttps://docs.microsoft.com/en-us/azure/machine-learning/how-to-auto-train-image-models."]
    #[serde(rename = "modelName", default, skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[doc = "Value of momentum when optimizer is 'sgd'. Must be a float in the range [0, 1]."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub momentum: Option<String>,
    #[doc = "Enable nesterov when optimizer is 'sgd'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nesterov: Option<String>,
    #[doc = "Number of training epochs. Must be a positive integer."]
    #[serde(rename = "numberOfEpochs", default, skip_serializing_if = "Option::is_none")]
    pub number_of_epochs: Option<String>,
    #[doc = "Number of data loader workers. Must be a non-negative integer."]
    #[serde(rename = "numberOfWorkers", default, skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<String>,
    #[doc = "Type of optimizer. Must be either 'sgd', 'adam', or 'adamw'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optimizer: Option<String>,
    #[doc = "Random seed to be used when using deterministic training."]
    #[serde(rename = "randomSeed", default, skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<String>,
    #[doc = "Value of gamma when learning rate scheduler is 'step'. Must be a float in the range [0, 1]."]
    #[serde(rename = "stepLRGamma", default, skip_serializing_if = "Option::is_none")]
    pub step_lr_gamma: Option<String>,
    #[doc = "Value of step size when learning rate scheduler is 'step'. Must be a positive integer."]
    #[serde(rename = "stepLRStepSize", default, skip_serializing_if = "Option::is_none")]
    pub step_lr_step_size: Option<String>,
    #[doc = "Training batch size. Must be a positive integer."]
    #[serde(rename = "trainingBatchSize", default, skip_serializing_if = "Option::is_none")]
    pub training_batch_size: Option<String>,
    #[doc = "Validation batch size. Must be a positive integer."]
    #[serde(rename = "validationBatchSize", default, skip_serializing_if = "Option::is_none")]
    pub validation_batch_size: Option<String>,
    #[doc = "Value of cosine cycle when learning rate scheduler is 'warmup_cosine'. Must be a float in the range [0, 1]."]
    #[serde(rename = "warmupCosineLRCycles", default, skip_serializing_if = "Option::is_none")]
    pub warmup_cosine_lr_cycles: Option<String>,
    #[doc = "Value of warmup epochs when learning rate scheduler is 'warmup_cosine'. Must be a positive integer."]
    #[serde(rename = "warmupCosineLRWarmupEpochs", default, skip_serializing_if = "Option::is_none")]
    pub warmup_cosine_lr_warmup_epochs: Option<String>,
    #[doc = "Value of weight decay when optimizer is 'sgd', 'adam', or 'adamw'. Must be a float in the range[0, 1]."]
    #[serde(rename = "weightDecay", default, skip_serializing_if = "Option::is_none")]
    pub weight_decay: Option<String>,
}
impl ImageModelDistributionSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Distribution expressions to sweep over values of model settings.\r\n<example>\r\nSome examples are:\r\n```\r\nModelName = \"choice('seresnext', 'resnest50')\";\r\nLearningRate = \"uniform(0.001, 0.01)\";\r\nLayersToFreeze = \"choice(0, 2)\";\r\n```</example>\r\nFor more details on how to compose distribution expressions please check the documentation:\r\nhttps://docs.microsoft.com/en-us/azure/machine-learning/how-to-tune-hyperparameters\r\nFor more information on the available settings please visit the official documentation:\r\nhttps://docs.microsoft.com/en-us/azure/machine-learning/how-to-auto-train-image-models."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageModelDistributionSettingsClassification {
    #[serde(flatten)]
    pub image_model_distribution_settings: ImageModelDistributionSettings,
    #[doc = "Image crop size that is input to the neural network for the training dataset. Must be a positive integer."]
    #[serde(rename = "trainingCropSize", default, skip_serializing_if = "Option::is_none")]
    pub training_crop_size: Option<String>,
    #[doc = "Image crop size that is input to the neural network for the validation dataset. Must be a positive integer."]
    #[serde(rename = "validationCropSize", default, skip_serializing_if = "Option::is_none")]
    pub validation_crop_size: Option<String>,
    #[doc = "Image size to which to resize before cropping for validation dataset. Must be a positive integer."]
    #[serde(rename = "validationResizeSize", default, skip_serializing_if = "Option::is_none")]
    pub validation_resize_size: Option<String>,
    #[doc = "Weighted loss. The accepted values are 0 for no weighted loss.\r\n1 for weighted loss with sqrt.(class_weights). 2 for weighted loss with class_weights. Must be 0 or 1 or 2."]
    #[serde(rename = "weightedLoss", default, skip_serializing_if = "Option::is_none")]
    pub weighted_loss: Option<String>,
}
impl ImageModelDistributionSettingsClassification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Distribution expressions to sweep over values of model settings.\r\n<example>\r\nSome examples are:\r\n```\r\nModelName = \"choice('seresnext', 'resnest50')\";\r\nLearningRate = \"uniform(0.001, 0.01)\";\r\nLayersToFreeze = \"choice(0, 2)\";\r\n```</example>\r\nFor more details on how to compose distribution expressions please check the documentation:\r\nhttps://docs.microsoft.com/en-us/azure/machine-learning/how-to-tune-hyperparameters\r\nFor more information on the available settings please visit the official documentation:\r\nhttps://docs.microsoft.com/en-us/azure/machine-learning/how-to-auto-train-image-models."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageModelDistributionSettingsObjectDetection {
    #[serde(flatten)]
    pub image_model_distribution_settings: ImageModelDistributionSettings,
    #[doc = "Maximum number of detections per image, for all classes. Must be a positive integer.\r\nNote: This settings is not supported for the 'yolov5' algorithm."]
    #[serde(rename = "boxDetectionsPerImage", default, skip_serializing_if = "Option::is_none")]
    pub box_detections_per_image: Option<String>,
    #[doc = "During inference, only return proposals with a classification score greater than\r\nBoxScoreThreshold. Must be a float in the range[0, 1]."]
    #[serde(rename = "boxScoreThreshold", default, skip_serializing_if = "Option::is_none")]
    pub box_score_threshold: Option<String>,
    #[doc = "Image size for train and validation. Must be a positive integer.\r\nNote: The training run may get into CUDA OOM if the size is too big.\r\nNote: This settings is only supported for the 'yolov5' algorithm."]
    #[serde(rename = "imageSize", default, skip_serializing_if = "Option::is_none")]
    pub image_size: Option<String>,
    #[doc = "Maximum size of the image to be rescaled before feeding it to the backbone.\r\nMust be a positive integer. Note: training run may get into CUDA OOM if the size is too big.\r\nNote: This settings is not supported for the 'yolov5' algorithm."]
    #[serde(rename = "maxSize", default, skip_serializing_if = "Option::is_none")]
    pub max_size: Option<String>,
    #[doc = "Minimum size of the image to be rescaled before feeding it to the backbone.\r\nMust be a positive integer. Note: training run may get into CUDA OOM if the size is too big.\r\nNote: This settings is not supported for the 'yolov5' algorithm."]
    #[serde(rename = "minSize", default, skip_serializing_if = "Option::is_none")]
    pub min_size: Option<String>,
    #[doc = "Model size. Must be 'small', 'medium', 'large', or 'xlarge'.\r\nNote: training run may get into CUDA OOM if the model size is too big.\r\nNote: This settings is only supported for the 'yolov5' algorithm."]
    #[serde(rename = "modelSize", default, skip_serializing_if = "Option::is_none")]
    pub model_size: Option<String>,
    #[doc = "Enable multi-scale image by varying image size by +/- 50%.\r\nNote: training run may get into CUDA OOM if no sufficient GPU memory.\r\nNote: This settings is only supported for the 'yolov5' algorithm."]
    #[serde(rename = "multiScale", default, skip_serializing_if = "Option::is_none")]
    pub multi_scale: Option<String>,
    #[doc = "IOU threshold used during inference in NMS post processing. Must be float in the range [0, 1]."]
    #[serde(rename = "nmsIouThreshold", default, skip_serializing_if = "Option::is_none")]
    pub nms_iou_threshold: Option<String>,
    #[doc = "The grid size to use for tiling each image. Note: TileGridSize must not be\r\nNone to enable small object detection logic. A string containing two integers in mxn format.\r\nNote: This settings is not supported for the 'yolov5' algorithm."]
    #[serde(rename = "tileGridSize", default, skip_serializing_if = "Option::is_none")]
    pub tile_grid_size: Option<String>,
    #[doc = "Overlap ratio between adjacent tiles in each dimension. Must be float in the range [0, 1).\r\nNote: This settings is not supported for the 'yolov5' algorithm."]
    #[serde(rename = "tileOverlapRatio", default, skip_serializing_if = "Option::is_none")]
    pub tile_overlap_ratio: Option<String>,
    #[doc = "The IOU threshold to use to perform NMS while merging predictions from tiles and image.\r\nUsed in validation/ inference. Must be float in the range [0, 1].\r\nNote: This settings is not supported for the 'yolov5' algorithm.\r\nNMS: Non-maximum suppression"]
    #[serde(rename = "tilePredictionsNmsThreshold", default, skip_serializing_if = "Option::is_none")]
    pub tile_predictions_nms_threshold: Option<String>,
    #[doc = "IOU threshold to use when computing validation metric. Must be float in the range [0, 1]."]
    #[serde(rename = "validationIouThreshold", default, skip_serializing_if = "Option::is_none")]
    pub validation_iou_threshold: Option<String>,
    #[doc = "Metric computation method to use for validation metrics. Must be 'none', 'coco', 'voc', or 'coco_voc'."]
    #[serde(rename = "validationMetricType", default, skip_serializing_if = "Option::is_none")]
    pub validation_metric_type: Option<String>,
}
impl ImageModelDistributionSettingsObjectDetection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Settings used for training the model.\r\nFor more information on the available settings please visit the official documentation:\r\nhttps://docs.microsoft.com/en-us/azure/machine-learning/how-to-auto-train-image-models."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageModelSettings {
    #[doc = "Settings for advanced scenarios."]
    #[serde(rename = "advancedSettings", default, skip_serializing_if = "Option::is_none")]
    pub advanced_settings: Option<String>,
    #[doc = "Enable AMSGrad when optimizer is 'adam' or 'adamw'."]
    #[serde(rename = "amsGradient", default, skip_serializing_if = "Option::is_none")]
    pub ams_gradient: Option<bool>,
    #[doc = "Settings for using Augmentations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub augmentations: Option<String>,
    #[doc = "Value of 'beta1' when optimizer is 'adam' or 'adamw'. Must be a float in the range [0, 1]."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beta1: Option<f32>,
    #[doc = "Value of 'beta2' when optimizer is 'adam' or 'adamw'. Must be a float in the range [0, 1]."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beta2: Option<f32>,
    #[doc = "Frequency to store model checkpoints. Must be a positive integer."]
    #[serde(rename = "checkpointFrequency", default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_frequency: Option<i32>,
    #[serde(rename = "checkpointModel", default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_model: Option<MlFlowModelJobInput>,
    #[doc = "The id of a previous run that has a pretrained checkpoint for incremental training."]
    #[serde(rename = "checkpointRunId", default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_run_id: Option<String>,
    #[doc = "Whether to use distributed training."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distributed: Option<bool>,
    #[doc = "Enable early stopping logic during training."]
    #[serde(rename = "earlyStopping", default, skip_serializing_if = "Option::is_none")]
    pub early_stopping: Option<bool>,
    #[doc = "Minimum number of epochs or validation evaluations to wait before primary metric improvement\r\nis tracked for early stopping. Must be a positive integer."]
    #[serde(rename = "earlyStoppingDelay", default, skip_serializing_if = "Option::is_none")]
    pub early_stopping_delay: Option<i32>,
    #[doc = "Minimum number of epochs or validation evaluations with no primary metric improvement before\r\nthe run is stopped. Must be a positive integer."]
    #[serde(rename = "earlyStoppingPatience", default, skip_serializing_if = "Option::is_none")]
    pub early_stopping_patience: Option<i32>,
    #[doc = "Enable normalization when exporting ONNX model."]
    #[serde(rename = "enableOnnxNormalization", default, skip_serializing_if = "Option::is_none")]
    pub enable_onnx_normalization: Option<bool>,
    #[doc = "Frequency to evaluate validation dataset to get metric scores. Must be a positive integer."]
    #[serde(rename = "evaluationFrequency", default, skip_serializing_if = "Option::is_none")]
    pub evaluation_frequency: Option<i32>,
    #[doc = "Gradient accumulation means running a configured number of \"GradAccumulationStep\" steps without\r\nupdating the model weights while accumulating the gradients of those steps, and then using\r\nthe accumulated gradients to compute the weight updates. Must be a positive integer."]
    #[serde(rename = "gradientAccumulationStep", default, skip_serializing_if = "Option::is_none")]
    pub gradient_accumulation_step: Option<i32>,
    #[doc = "Number of layers to freeze for the model. Must be a positive integer.\r\nFor instance, passing 2 as value for 'seresnext' means\r\nfreezing layer0 and layer1. For a full list of models supported and details on layer freeze, please\r\nsee: https://docs.microsoft.com/en-us/azure/machine-learning/how-to-auto-train-image-models."]
    #[serde(rename = "layersToFreeze", default, skip_serializing_if = "Option::is_none")]
    pub layers_to_freeze: Option<i32>,
    #[doc = "Initial learning rate. Must be a float in the range [0, 1]."]
    #[serde(rename = "learningRate", default, skip_serializing_if = "Option::is_none")]
    pub learning_rate: Option<f32>,
    #[doc = "Learning rate scheduler enum."]
    #[serde(rename = "learningRateScheduler", default, skip_serializing_if = "Option::is_none")]
    pub learning_rate_scheduler: Option<LearningRateScheduler>,
    #[doc = "Name of the model to use for training.\r\nFor more information on the available models please visit the official documentation:\r\nhttps://docs.microsoft.com/en-us/azure/machine-learning/how-to-auto-train-image-models."]
    #[serde(rename = "modelName", default, skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[doc = "Value of momentum when optimizer is 'sgd'. Must be a float in the range [0, 1]."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub momentum: Option<f32>,
    #[doc = "Enable nesterov when optimizer is 'sgd'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nesterov: Option<bool>,
    #[doc = "Number of training epochs. Must be a positive integer."]
    #[serde(rename = "numberOfEpochs", default, skip_serializing_if = "Option::is_none")]
    pub number_of_epochs: Option<i32>,
    #[doc = "Number of data loader workers. Must be a non-negative integer."]
    #[serde(rename = "numberOfWorkers", default, skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[doc = "Stochastic optimizer for image models."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optimizer: Option<StochasticOptimizer>,
    #[doc = "Random seed to be used when using deterministic training."]
    #[serde(rename = "randomSeed", default, skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<i32>,
    #[doc = "Value of gamma when learning rate scheduler is 'step'. Must be a float in the range [0, 1]."]
    #[serde(rename = "stepLRGamma", default, skip_serializing_if = "Option::is_none")]
    pub step_lr_gamma: Option<f32>,
    #[doc = "Value of step size when learning rate scheduler is 'step'. Must be a positive integer."]
    #[serde(rename = "stepLRStepSize", default, skip_serializing_if = "Option::is_none")]
    pub step_lr_step_size: Option<i32>,
    #[doc = "Training batch size. Must be a positive integer."]
    #[serde(rename = "trainingBatchSize", default, skip_serializing_if = "Option::is_none")]
    pub training_batch_size: Option<i32>,
    #[doc = "Validation batch size. Must be a positive integer."]
    #[serde(rename = "validationBatchSize", default, skip_serializing_if = "Option::is_none")]
    pub validation_batch_size: Option<i32>,
    #[doc = "Value of cosine cycle when learning rate scheduler is 'warmup_cosine'. Must be a float in the range [0, 1]."]
    #[serde(rename = "warmupCosineLRCycles", default, skip_serializing_if = "Option::is_none")]
    pub warmup_cosine_lr_cycles: Option<f32>,
    #[doc = "Value of warmup epochs when learning rate scheduler is 'warmup_cosine'. Must be a positive integer."]
    #[serde(rename = "warmupCosineLRWarmupEpochs", default, skip_serializing_if = "Option::is_none")]
    pub warmup_cosine_lr_warmup_epochs: Option<i32>,
    #[doc = "Value of weight decay when optimizer is 'sgd', 'adam', or 'adamw'. Must be a float in the range[0, 1]."]
    #[serde(rename = "weightDecay", default, skip_serializing_if = "Option::is_none")]
    pub weight_decay: Option<f32>,
}
impl ImageModelSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Settings used for training the model.\r\nFor more information on the available settings please visit the official documentation:\r\nhttps://docs.microsoft.com/en-us/azure/machine-learning/how-to-auto-train-image-models."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageModelSettingsClassification {
    #[serde(flatten)]
    pub image_model_settings: ImageModelSettings,
    #[doc = "Image crop size that is input to the neural network for the training dataset. Must be a positive integer."]
    #[serde(rename = "trainingCropSize", default, skip_serializing_if = "Option::is_none")]
    pub training_crop_size: Option<i32>,
    #[doc = "Image crop size that is input to the neural network for the validation dataset. Must be a positive integer."]
    #[serde(rename = "validationCropSize", default, skip_serializing_if = "Option::is_none")]
    pub validation_crop_size: Option<i32>,
    #[doc = "Image size to which to resize before cropping for validation dataset. Must be a positive integer."]
    #[serde(rename = "validationResizeSize", default, skip_serializing_if = "Option::is_none")]
    pub validation_resize_size: Option<i32>,
    #[doc = "Weighted loss. The accepted values are 0 for no weighted loss.\r\n1 for weighted loss with sqrt.(class_weights). 2 for weighted loss with class_weights. Must be 0 or 1 or 2."]
    #[serde(rename = "weightedLoss", default, skip_serializing_if = "Option::is_none")]
    pub weighted_loss: Option<i32>,
}
impl ImageModelSettingsClassification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Settings used for training the model.\r\nFor more information on the available settings please visit the official documentation:\r\nhttps://docs.microsoft.com/en-us/azure/machine-learning/how-to-auto-train-image-models."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageModelSettingsObjectDetection {
    #[serde(flatten)]
    pub image_model_settings: ImageModelSettings,
    #[doc = "Maximum number of detections per image, for all classes. Must be a positive integer.\r\nNote: This settings is not supported for the 'yolov5' algorithm."]
    #[serde(rename = "boxDetectionsPerImage", default, skip_serializing_if = "Option::is_none")]
    pub box_detections_per_image: Option<i32>,
    #[doc = "During inference, only return proposals with a classification score greater than\r\nBoxScoreThreshold. Must be a float in the range[0, 1]."]
    #[serde(rename = "boxScoreThreshold", default, skip_serializing_if = "Option::is_none")]
    pub box_score_threshold: Option<f32>,
    #[doc = "Image size for train and validation. Must be a positive integer.\r\nNote: The training run may get into CUDA OOM if the size is too big.\r\nNote: This settings is only supported for the 'yolov5' algorithm."]
    #[serde(rename = "imageSize", default, skip_serializing_if = "Option::is_none")]
    pub image_size: Option<i32>,
    #[serde(rename = "logTrainingMetrics", default, skip_serializing_if = "Option::is_none")]
    pub log_training_metrics: Option<LogTrainingMetrics>,
    #[serde(rename = "logValidationLoss", default, skip_serializing_if = "Option::is_none")]
    pub log_validation_loss: Option<LogValidationLoss>,
    #[doc = "Maximum size of the image to be rescaled before feeding it to the backbone.\r\nMust be a positive integer. Note: training run may get into CUDA OOM if the size is too big.\r\nNote: This settings is not supported for the 'yolov5' algorithm."]
    #[serde(rename = "maxSize", default, skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i32>,
    #[doc = "Minimum size of the image to be rescaled before feeding it to the backbone.\r\nMust be a positive integer. Note: training run may get into CUDA OOM if the size is too big.\r\nNote: This settings is not supported for the 'yolov5' algorithm."]
    #[serde(rename = "minSize", default, skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i32>,
    #[doc = "Image model size."]
    #[serde(rename = "modelSize", default, skip_serializing_if = "Option::is_none")]
    pub model_size: Option<ModelSize>,
    #[doc = "Enable multi-scale image by varying image size by +/- 50%.\r\nNote: training run may get into CUDA OOM if no sufficient GPU memory.\r\nNote: This settings is only supported for the 'yolov5' algorithm."]
    #[serde(rename = "multiScale", default, skip_serializing_if = "Option::is_none")]
    pub multi_scale: Option<bool>,
    #[doc = "IOU threshold used during inference in NMS post processing. Must be a float in the range [0, 1]."]
    #[serde(rename = "nmsIouThreshold", default, skip_serializing_if = "Option::is_none")]
    pub nms_iou_threshold: Option<f32>,
    #[doc = "The grid size to use for tiling each image. Note: TileGridSize must not be\r\nNone to enable small object detection logic. A string containing two integers in mxn format.\r\nNote: This settings is not supported for the 'yolov5' algorithm."]
    #[serde(rename = "tileGridSize", default, skip_serializing_if = "Option::is_none")]
    pub tile_grid_size: Option<String>,
    #[doc = "Overlap ratio between adjacent tiles in each dimension. Must be float in the range [0, 1).\r\nNote: This settings is not supported for the 'yolov5' algorithm."]
    #[serde(rename = "tileOverlapRatio", default, skip_serializing_if = "Option::is_none")]
    pub tile_overlap_ratio: Option<f32>,
    #[doc = "The IOU threshold to use to perform NMS while merging predictions from tiles and image.\r\nUsed in validation/ inference. Must be float in the range [0, 1].\r\nNote: This settings is not supported for the 'yolov5' algorithm."]
    #[serde(rename = "tilePredictionsNmsThreshold", default, skip_serializing_if = "Option::is_none")]
    pub tile_predictions_nms_threshold: Option<f32>,
    #[doc = "IOU threshold to use when computing validation metric. Must be float in the range [0, 1]."]
    #[serde(rename = "validationIouThreshold", default, skip_serializing_if = "Option::is_none")]
    pub validation_iou_threshold: Option<f32>,
    #[doc = "Metric computation method to use for validation metrics in image tasks."]
    #[serde(rename = "validationMetricType", default, skip_serializing_if = "Option::is_none")]
    pub validation_metric_type: Option<ValidationMetricType>,
}
impl ImageModelSettingsObjectDetection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image Object Detection. Object detection is used to identify objects in an image and locate each object with a\r\nbounding box e.g. locate all dogs and cats in an image and draw a bounding box around each."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageObjectDetection {
    #[serde(flatten)]
    pub image_object_detection_base: ImageObjectDetectionBase,
    #[serde(flatten)]
    pub auto_ml_vertical: AutoMlVertical,
    #[doc = "Primary metrics for Image ObjectDetection task."]
    #[serde(rename = "primaryMetric", default, skip_serializing_if = "Option::is_none")]
    pub primary_metric: Option<ObjectDetectionPrimaryMetrics>,
}
impl ImageObjectDetection {
    pub fn new(image_object_detection_base: ImageObjectDetectionBase, auto_ml_vertical: AutoMlVertical) -> Self {
        Self {
            image_object_detection_base,
            auto_ml_vertical,
            primary_metric: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageObjectDetectionBase {
    #[serde(flatten)]
    pub image_vertical: ImageVertical,
    #[doc = "Settings used for training the model.\r\nFor more information on the available settings please visit the official documentation:\r\nhttps://docs.microsoft.com/en-us/azure/machine-learning/how-to-auto-train-image-models."]
    #[serde(rename = "modelSettings", default, skip_serializing_if = "Option::is_none")]
    pub model_settings: Option<ImageModelSettingsObjectDetection>,
    #[doc = "Search space for sampling different combinations of models and their hyperparameters."]
    #[serde(
        rename = "searchSpace",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub search_space: Vec<ImageModelDistributionSettingsObjectDetection>,
}
impl ImageObjectDetectionBase {
    pub fn new(image_vertical: ImageVertical) -> Self {
        Self {
            image_vertical,
            model_settings: None,
            search_space: Vec::new(),
        }
    }
}
#[doc = "Model sweeping and hyperparameter sweeping related settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageSweepSettings {
    #[doc = "Early termination policies enable canceling poor-performing runs before they complete"]
    #[serde(rename = "earlyTermination", default, skip_serializing_if = "Option::is_none")]
    pub early_termination: Option<EarlyTerminationPolicy>,
    #[serde(rename = "samplingAlgorithm")]
    pub sampling_algorithm: SamplingAlgorithmType,
}
impl ImageSweepSettings {
    pub fn new(sampling_algorithm: SamplingAlgorithmType) -> Self {
        Self {
            early_termination: None,
            sampling_algorithm,
        }
    }
}
#[doc = "Abstract class for AutoML tasks that train image (computer vision) models -\r\nsuch as Image Classification / Image Classification Multilabel / Image Object Detection / Image Instance Segmentation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageVertical {
    #[doc = "Limit settings for the AutoML job."]
    #[serde(rename = "limitSettings")]
    pub limit_settings: ImageLimitSettings,
    #[doc = "Model sweeping and hyperparameter sweeping related settings."]
    #[serde(rename = "sweepSettings", default, skip_serializing_if = "Option::is_none")]
    pub sweep_settings: Option<ImageSweepSettings>,
    #[serde(rename = "validationData", default, skip_serializing_if = "Option::is_none")]
    pub validation_data: Option<MlTableJobInput>,
    #[doc = "The fraction of training dataset that needs to be set aside for validation purpose.\r\nValues between (0.0 , 1.0)\r\nApplied when validation dataset is not provided."]
    #[serde(rename = "validationDataSize", default, skip_serializing_if = "Option::is_none")]
    pub validation_data_size: Option<f64>,
}
impl ImageVertical {
    pub fn new(limit_settings: ImageLimitSettings) -> Self {
        Self {
            limit_settings,
            sweep_settings: None,
            validation_data: None,
            validation_data_size: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportDataAction {
    #[serde(flatten)]
    pub schedule_action_base: ScheduleActionBase,
    #[serde(rename = "dataImportDefinition")]
    pub data_import_definition: DataImport,
}
impl ImportDataAction {
    pub fn new(schedule_action_base: ScheduleActionBase, data_import_definition: DataImport) -> Self {
        Self {
            schedule_action_base,
            data_import_definition,
        }
    }
}
#[doc = "Whether IncrementalDataRefresh is enabled"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IncrementalDataRefresh")]
pub enum IncrementalDataRefresh {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IncrementalDataRefresh {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IncrementalDataRefresh {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IncrementalDataRefresh {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("IncrementalDataRefresh", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("IncrementalDataRefresh", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Dto object representing index column"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IndexColumn {
    #[doc = "Specifies the column name"]
    #[serde(rename = "columnName", default, skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<FeatureDataType>,
}
impl IndexColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InferenceContainerProperties {
    #[serde(rename = "livenessRoute", default, skip_serializing_if = "Option::is_none")]
    pub liveness_route: Option<Route>,
    #[serde(rename = "readinessRoute", default, skip_serializing_if = "Option::is_none")]
    pub readiness_route: Option<Route>,
    #[serde(rename = "scoringRoute", default, skip_serializing_if = "Option::is_none")]
    pub scoring_route: Option<Route>,
}
impl InferenceContainerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InferencingServer {
    #[doc = "Inferencing server type for various targets."]
    #[serde(rename = "serverType")]
    pub server_type: InferencingServerType,
}
impl InferencingServer {
    pub fn new(server_type: InferencingServerType) -> Self {
        Self { server_type }
    }
}
#[doc = "Inferencing server type for various targets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InferencingServerType")]
pub enum InferencingServerType {
    #[serde(rename = "AzureMLOnline")]
    AzureMlOnline,
    #[serde(rename = "AzureMLBatch")]
    AzureMlBatch,
    Triton,
    Custom,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InferencingServerType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InferencingServerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InferencingServerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureMlOnline => serializer.serialize_unit_variant("InferencingServerType", 0u32, "AzureMLOnline"),
            Self::AzureMlBatch => serializer.serialize_unit_variant("InferencingServerType", 1u32, "AzureMLBatch"),
            Self::Triton => serializer.serialize_unit_variant("InferencingServerType", 2u32, "Triton"),
            Self::Custom => serializer.serialize_unit_variant("InferencingServerType", 3u32, "Custom"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enum to determine the input data delivery mode."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InputDeliveryMode")]
pub enum InputDeliveryMode {
    ReadOnlyMount,
    ReadWriteMount,
    Download,
    Direct,
    EvalMount,
    EvalDownload,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InputDeliveryMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InputDeliveryMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InputDeliveryMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ReadOnlyMount => serializer.serialize_unit_variant("InputDeliveryMode", 0u32, "ReadOnlyMount"),
            Self::ReadWriteMount => serializer.serialize_unit_variant("InputDeliveryMode", 1u32, "ReadWriteMount"),
            Self::Download => serializer.serialize_unit_variant("InputDeliveryMode", 2u32, "Download"),
            Self::Direct => serializer.serialize_unit_variant("InputDeliveryMode", 3u32, "Direct"),
            Self::EvalMount => serializer.serialize_unit_variant("InputDeliveryMode", 4u32, "EvalMount"),
            Self::EvalDownload => serializer.serialize_unit_variant("InputDeliveryMode", 5u32, "EvalDownload"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Input path type for package inputs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InputPathType")]
pub enum InputPathType {
    Url,
    PathId,
    PathVersion,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InputPathType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InputPathType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InputPathType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Url => serializer.serialize_unit_variant("InputPathType", 0u32, "Url"),
            Self::PathId => serializer.serialize_unit_variant("InputPathType", 1u32, "PathId"),
            Self::PathVersion => serializer.serialize_unit_variant("InputPathType", 2u32, "PathVersion"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Resource requests/limits for this instance type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstanceResourceSchema {}
impl InstanceResourceSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Primary metrics for InstanceSegmentation tasks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InstanceSegmentationPrimaryMetrics")]
pub enum InstanceSegmentationPrimaryMetrics {
    MeanAveragePrecision,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InstanceSegmentationPrimaryMetrics {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InstanceSegmentationPrimaryMetrics {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InstanceSegmentationPrimaryMetrics {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::MeanAveragePrecision => {
                serializer.serialize_unit_variant("InstanceSegmentationPrimaryMetrics", 0u32, "MeanAveragePrecision")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Instance type schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstanceTypeSchema {
    #[doc = "Node Selector"]
    #[serde(rename = "nodeSelector", default, skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<serde_json::Value>,
    #[doc = "Resource requests/limits for this instance type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<instance_type_schema::Resources>,
}
impl InstanceTypeSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod instance_type_schema {
    use super::*;
    #[doc = "Resource requests/limits for this instance type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Resources {
        #[doc = "Resource requests/limits for this instance type"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub requests: Option<InstanceResourceSchema>,
        #[doc = "Resource requests/limits for this instance type"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limits: Option<InstanceResourceSchema>,
    }
    impl Resources {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Intellectual Property details for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntellectualProperty {
    #[doc = "Protection level associated with the Intellectual Property."]
    #[serde(rename = "protectionLevel", default, skip_serializing_if = "Option::is_none")]
    pub protection_level: Option<ProtectionLevel>,
    #[doc = "[Required] Publisher of the Intellectual Property. Must be the same as Registry publisher name."]
    pub publisher: String,
}
impl IntellectualProperty {
    pub fn new(publisher: String) -> Self {
        Self {
            protection_level: None,
            publisher,
        }
    }
}
#[doc = "Isolation mode for the managed network of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IsolationMode")]
pub enum IsolationMode {
    Disabled,
    AllowInternetOutbound,
    AllowOnlyApprovedOutbound,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IsolationMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IsolationMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IsolationMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("IsolationMode", 0u32, "Disabled"),
            Self::AllowInternetOutbound => serializer.serialize_unit_variant("IsolationMode", 1u32, "AllowInternetOutbound"),
            Self::AllowOnlyApprovedOutbound => serializer.serialize_unit_variant("IsolationMode", 2u32, "AllowOnlyApprovedOutbound"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Base definition for a job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobBase {
    #[serde(flatten)]
    pub resource_base: ResourceBase,
    #[doc = "ARM resource ID of the component resource."]
    #[serde(rename = "componentId", default, skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    #[doc = "ARM resource ID of the compute resource."]
    #[serde(rename = "computeId", default, skip_serializing_if = "Option::is_none")]
    pub compute_id: Option<String>,
    #[doc = "Display name of job."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The name of the experiment the job belongs to. If not set, the job is placed in the \"Default\" experiment."]
    #[serde(rename = "experimentName", default, skip_serializing_if = "Option::is_none")]
    pub experiment_name: Option<String>,
    #[doc = "Base definition for identity configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityConfiguration>,
    #[doc = "Is the asset archived?"]
    #[serde(rename = "isArchived", default, skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[doc = "Enum to determine the type of job."]
    #[serde(rename = "jobType")]
    pub job_type: JobType,
    #[doc = "Configuration for notification."]
    #[serde(rename = "notificationSetting", default, skip_serializing_if = "Option::is_none")]
    pub notification_setting: Option<NotificationSetting>,
    #[doc = "Configuration for secrets to be made available during runtime."]
    #[serde(rename = "secretsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub secrets_configuration: Option<serde_json::Value>,
    #[doc = "List of JobEndpoints.\r\nFor local jobs, a job endpoint will have an endpoint value of FileStreamObject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<serde_json::Value>,
    #[doc = "The status of a job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,
}
impl JobBase {
    pub fn new(job_type: JobType) -> Self {
        Self {
            resource_base: ResourceBase::default(),
            component_id: None,
            compute_id: None,
            display_name: None,
            experiment_name: None,
            identity: None,
            is_archived: None,
            job_type,
            notification_setting: None,
            secrets_configuration: None,
            services: None,
            status: None,
        }
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobBaseResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Base definition for a job."]
    pub properties: JobBase,
}
impl JobBaseResource {
    pub fn new(properties: JobBase) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of JobBase entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobBaseResourceArmPaginatedResult {
    #[doc = "The link to the next page of JobBase objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type JobBase."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<JobBaseResource>,
}
impl azure_core::Continuable for JobBaseResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl JobBaseResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Command job definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobInput {
    #[doc = "Description for the input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Enum to determine the Job Input Type."]
    #[serde(rename = "jobInputType")]
    pub job_input_type: JobInputType,
}
impl JobInput {
    pub fn new(job_input_type: JobInputType) -> Self {
        Self {
            description: None,
            job_input_type,
        }
    }
}
#[doc = "Enum to determine the Job Input Type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "JobInputType")]
pub enum JobInputType {
    #[serde(rename = "literal")]
    Literal,
    #[serde(rename = "uri_file")]
    UriFile,
    #[serde(rename = "uri_folder")]
    UriFolder,
    #[serde(rename = "mltable")]
    Mltable,
    #[serde(rename = "custom_model")]
    CustomModel,
    #[serde(rename = "mlflow_model")]
    MlflowModel,
    #[serde(rename = "triton_model")]
    TritonModel,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for JobInputType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for JobInputType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for JobInputType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Literal => serializer.serialize_unit_variant("JobInputType", 0u32, "literal"),
            Self::UriFile => serializer.serialize_unit_variant("JobInputType", 1u32, "uri_file"),
            Self::UriFolder => serializer.serialize_unit_variant("JobInputType", 2u32, "uri_folder"),
            Self::Mltable => serializer.serialize_unit_variant("JobInputType", 3u32, "mltable"),
            Self::CustomModel => serializer.serialize_unit_variant("JobInputType", 4u32, "custom_model"),
            Self::MlflowModel => serializer.serialize_unit_variant("JobInputType", 5u32, "mlflow_model"),
            Self::TritonModel => serializer.serialize_unit_variant("JobInputType", 6u32, "triton_model"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobLimits {
    #[serde(rename = "jobLimitsType")]
    pub job_limits_type: JobLimitsType,
    #[doc = "The max run duration in ISO 8601 format, after which the job will be cancelled. Only supports duration with precision as low as Seconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}
impl JobLimits {
    pub fn new(job_limits_type: JobLimitsType) -> Self {
        Self {
            job_limits_type,
            timeout: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "JobLimitsType")]
pub enum JobLimitsType {
    Command,
    Sweep,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for JobLimitsType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for JobLimitsType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for JobLimitsType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Command => serializer.serialize_unit_variant("JobLimitsType", 0u32, "Command"),
            Self::Sweep => serializer.serialize_unit_variant("JobLimitsType", 1u32, "Sweep"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Job output definition container information on where to find job output/logs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobOutput {
    #[doc = "Description for the output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Enum to determine the Job Output Type."]
    #[serde(rename = "jobOutputType")]
    pub job_output_type: JobOutputType,
}
impl JobOutput {
    pub fn new(job_output_type: JobOutputType) -> Self {
        Self {
            description: None,
            job_output_type,
        }
    }
}
#[doc = "Enum to determine the Job Output Type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "JobOutputType")]
pub enum JobOutputType {
    #[serde(rename = "uri_file")]
    UriFile,
    #[serde(rename = "uri_folder")]
    UriFolder,
    #[serde(rename = "mltable")]
    Mltable,
    #[serde(rename = "custom_model")]
    CustomModel,
    #[serde(rename = "mlflow_model")]
    MlflowModel,
    #[serde(rename = "triton_model")]
    TritonModel,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for JobOutputType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for JobOutputType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for JobOutputType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UriFile => serializer.serialize_unit_variant("JobOutputType", 0u32, "uri_file"),
            Self::UriFolder => serializer.serialize_unit_variant("JobOutputType", 1u32, "uri_folder"),
            Self::Mltable => serializer.serialize_unit_variant("JobOutputType", 2u32, "mltable"),
            Self::CustomModel => serializer.serialize_unit_variant("JobOutputType", 3u32, "custom_model"),
            Self::MlflowModel => serializer.serialize_unit_variant("JobOutputType", 4u32, "mlflow_model"),
            Self::TritonModel => serializer.serialize_unit_variant("JobOutputType", 5u32, "triton_model"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enum to determine the job provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "JobProvisioningState")]
pub enum JobProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    InProgress,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for JobProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for JobProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for JobProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("JobProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("JobProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("JobProvisioningState", 2u32, "Canceled"),
            Self::InProgress => serializer.serialize_unit_variant("JobProvisioningState", 3u32, "InProgress"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobResourceConfiguration {
    #[serde(flatten)]
    pub resource_configuration: ResourceConfiguration,
    #[doc = "Extra arguments to pass to the Docker run command. This would override any parameters that have already been set by the system, or in this section. This parameter is only supported for Azure ML compute types."]
    #[serde(rename = "dockerArgs", default, skip_serializing_if = "Option::is_none")]
    pub docker_args: Option<String>,
    #[doc = "Size of the docker container's shared memory block. This should be in the format of (number)(unit) where number as to be greater than 0 and the unit can be one of b(bytes), k(kilobytes), m(megabytes), or g(gigabytes)."]
    #[serde(rename = "shmSize", default, skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<String>,
}
impl JobResourceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobScheduleAction {
    #[serde(flatten)]
    pub schedule_action_base: ScheduleActionBase,
    #[doc = "Base definition for a job."]
    #[serde(rename = "jobDefinition")]
    pub job_definition: JobBase,
}
impl JobScheduleAction {
    pub fn new(schedule_action_base: ScheduleActionBase, job_definition: JobBase) -> Self {
        Self {
            schedule_action_base,
            job_definition,
        }
    }
}
#[doc = "Job endpoint definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobService {
    #[doc = "Url for endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "Any error in the service."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Endpoint type."]
    #[serde(rename = "jobServiceType", default, skip_serializing_if = "Option::is_none")]
    pub job_service_type: Option<String>,
    #[doc = "Abstract Nodes definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Nodes>,
    #[doc = "Port for endpoint set by user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "Additional properties to set on the endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Status of endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl JobService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of a job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "JobStatus")]
pub enum JobStatus {
    NotStarted,
    Starting,
    Provisioning,
    Preparing,
    Queued,
    Running,
    Finalizing,
    CancelRequested,
    Completed,
    Failed,
    Canceled,
    NotResponding,
    Paused,
    Unknown,
    Scheduled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for JobStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for JobStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for JobStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotStarted => serializer.serialize_unit_variant("JobStatus", 0u32, "NotStarted"),
            Self::Starting => serializer.serialize_unit_variant("JobStatus", 1u32, "Starting"),
            Self::Provisioning => serializer.serialize_unit_variant("JobStatus", 2u32, "Provisioning"),
            Self::Preparing => serializer.serialize_unit_variant("JobStatus", 3u32, "Preparing"),
            Self::Queued => serializer.serialize_unit_variant("JobStatus", 4u32, "Queued"),
            Self::Running => serializer.serialize_unit_variant("JobStatus", 5u32, "Running"),
            Self::Finalizing => serializer.serialize_unit_variant("JobStatus", 6u32, "Finalizing"),
            Self::CancelRequested => serializer.serialize_unit_variant("JobStatus", 7u32, "CancelRequested"),
            Self::Completed => serializer.serialize_unit_variant("JobStatus", 8u32, "Completed"),
            Self::Failed => serializer.serialize_unit_variant("JobStatus", 9u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("JobStatus", 10u32, "Canceled"),
            Self::NotResponding => serializer.serialize_unit_variant("JobStatus", 11u32, "NotResponding"),
            Self::Paused => serializer.serialize_unit_variant("JobStatus", 12u32, "Paused"),
            Self::Unknown => serializer.serialize_unit_variant("JobStatus", 13u32, "Unknown"),
            Self::Scheduled => serializer.serialize_unit_variant("JobStatus", 14u32, "Scheduled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enum to determine the job tier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "JobTier")]
pub enum JobTier {
    Spot,
    Basic,
    Standard,
    Premium,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for JobTier {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for JobTier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for JobTier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Spot => serializer.serialize_unit_variant("JobTier", 0u32, "Spot"),
            Self::Basic => serializer.serialize_unit_variant("JobTier", 1u32, "Basic"),
            Self::Standard => serializer.serialize_unit_variant("JobTier", 2u32, "Standard"),
            Self::Premium => serializer.serialize_unit_variant("JobTier", 3u32, "Premium"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enum to determine the type of job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "JobType")]
pub enum JobType {
    #[serde(rename = "AutoML")]
    AutoMl,
    Command,
    Labeling,
    Sweep,
    Pipeline,
    Spark,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for JobType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for JobType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for JobType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AutoMl => serializer.serialize_unit_variant("JobType", 0u32, "AutoML"),
            Self::Command => serializer.serialize_unit_variant("JobType", 1u32, "Command"),
            Self::Labeling => serializer.serialize_unit_variant("JobType", 2u32, "Labeling"),
            Self::Sweep => serializer.serialize_unit_variant("JobType", 3u32, "Sweep"),
            Self::Pipeline => serializer.serialize_unit_variant("JobType", 4u32, "Pipeline"),
            Self::Spark => serializer.serialize_unit_variant("JobType", 5u32, "Spark"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KerberosCredentials {
    #[doc = "[Required] IP Address or DNS HostName."]
    #[serde(rename = "kerberosKdcAddress")]
    pub kerberos_kdc_address: String,
    #[doc = "[Required] Kerberos Username"]
    #[serde(rename = "kerberosPrincipal")]
    pub kerberos_principal: String,
    #[doc = "[Required] Domain over which a Kerberos authentication server has the authority to authenticate a user, host or service."]
    #[serde(rename = "kerberosRealm")]
    pub kerberos_realm: String,
}
impl KerberosCredentials {
    pub fn new(kerberos_kdc_address: String, kerberos_principal: String, kerberos_realm: String) -> Self {
        Self {
            kerberos_kdc_address,
            kerberos_principal,
            kerberos_realm,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KerberosKeytabCredentials {
    #[serde(flatten)]
    pub kerberos_credentials: KerberosCredentials,
    #[serde(flatten)]
    pub datastore_credentials: DatastoreCredentials,
    pub secrets: KerberosKeytabSecrets,
}
impl KerberosKeytabCredentials {
    pub fn new(
        kerberos_credentials: KerberosCredentials,
        datastore_credentials: DatastoreCredentials,
        secrets: KerberosKeytabSecrets,
    ) -> Self {
        Self {
            kerberos_credentials,
            datastore_credentials,
            secrets,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KerberosKeytabSecrets {
    #[serde(flatten)]
    pub datastore_secrets: DatastoreSecrets,
    #[doc = "Kerberos keytab secret."]
    #[serde(rename = "kerberosKeytab", default, skip_serializing_if = "Option::is_none")]
    pub kerberos_keytab: Option<String>,
}
impl KerberosKeytabSecrets {
    pub fn new(datastore_secrets: DatastoreSecrets) -> Self {
        Self {
            datastore_secrets,
            kerberos_keytab: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KerberosPasswordCredentials {
    #[serde(flatten)]
    pub kerberos_credentials: KerberosCredentials,
    #[serde(flatten)]
    pub datastore_credentials: DatastoreCredentials,
    pub secrets: KerberosPasswordSecrets,
}
impl KerberosPasswordCredentials {
    pub fn new(
        kerberos_credentials: KerberosCredentials,
        datastore_credentials: DatastoreCredentials,
        secrets: KerberosPasswordSecrets,
    ) -> Self {
        Self {
            kerberos_credentials,
            datastore_credentials,
            secrets,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KerberosPasswordSecrets {
    #[serde(flatten)]
    pub datastore_secrets: DatastoreSecrets,
    #[doc = "Kerberos password secret."]
    #[serde(rename = "kerberosPassword", default, skip_serializing_if = "Option::is_none")]
    pub kerberos_password: Option<String>,
}
impl KerberosPasswordSecrets {
    pub fn new(datastore_secrets: DatastoreSecrets) -> Self {
        Self {
            datastore_secrets,
            kerberos_password: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "KeyType")]
pub enum KeyType {
    Primary,
    Secondary,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for KeyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for KeyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for KeyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Primary => serializer.serialize_unit_variant("KeyType", 0u32, "Primary"),
            Self::Secondary => serializer.serialize_unit_variant("KeyType", 1u32, "Secondary"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A Machine Learning compute based on Kubernetes Compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Kubernetes {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(flatten)]
    pub kubernetes_schema: KubernetesSchema,
}
impl Kubernetes {
    pub fn new(compute: Compute) -> Self {
        Self {
            compute,
            kubernetes_schema: KubernetesSchema::default(),
        }
    }
}
#[doc = "Properties specific to a KubernetesOnlineDeployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubernetesOnlineDeployment {
    #[serde(flatten)]
    pub online_deployment: OnlineDeployment,
    #[doc = "Resource requirements for each container instance within an online deployment."]
    #[serde(rename = "containerResourceRequirements", default, skip_serializing_if = "Option::is_none")]
    pub container_resource_requirements: Option<ContainerResourceRequirements>,
}
impl KubernetesOnlineDeployment {
    pub fn new(online_deployment: OnlineDeployment) -> Self {
        Self {
            online_deployment,
            container_resource_requirements: None,
        }
    }
}
#[doc = "Kubernetes properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesProperties {
    #[doc = "Relay connection string."]
    #[serde(rename = "relayConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub relay_connection_string: Option<String>,
    #[doc = "ServiceBus connection string."]
    #[serde(rename = "serviceBusConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_connection_string: Option<String>,
    #[doc = "Extension principal-id."]
    #[serde(rename = "extensionPrincipalId", default, skip_serializing_if = "Option::is_none")]
    pub extension_principal_id: Option<String>,
    #[doc = "Extension instance release train."]
    #[serde(rename = "extensionInstanceReleaseTrain", default, skip_serializing_if = "Option::is_none")]
    pub extension_instance_release_train: Option<String>,
    #[doc = "VC name."]
    #[serde(rename = "vcName", default, skip_serializing_if = "Option::is_none")]
    pub vc_name: Option<String>,
    #[doc = "Compute namespace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "Default instance type"]
    #[serde(rename = "defaultInstanceType", default, skip_serializing_if = "Option::is_none")]
    pub default_instance_type: Option<String>,
    #[doc = "Instance Type Schema"]
    #[serde(rename = "instanceTypes", default, skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<serde_json::Value>,
}
impl KubernetesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kubernetes Compute Schema"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesSchema {
    #[doc = "Kubernetes properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<KubernetesProperties>,
}
impl KubernetesSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Label category definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabelCategory {
    #[doc = "Dictionary of label classes in this category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classes: Option<serde_json::Value>,
    #[doc = "Display name of the label category."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Whether multiSelect is enabled"]
    #[serde(rename = "multiSelect", default, skip_serializing_if = "Option::is_none")]
    pub multi_select: Option<MultiSelect>,
}
impl LabelCategory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Label class definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabelClass {
    #[doc = "Display name of the label class."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Dictionary of subclasses of the label class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subclasses: Option<serde_json::Value>,
}
impl LabelClass {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Labeling data configuration definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabelingDataConfiguration {
    #[doc = "Resource Id of the data asset to perform labeling."]
    #[serde(rename = "dataId", default, skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    #[doc = "Whether IncrementalDataRefresh is enabled"]
    #[serde(rename = "incrementalDataRefresh", default, skip_serializing_if = "Option::is_none")]
    pub incremental_data_refresh: Option<IncrementalDataRefresh>,
}
impl LabelingDataConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Labeling job definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabelingJob {
    #[serde(flatten)]
    pub job_base: JobBase,
    #[doc = "Created time of the job in UTC timezone."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Labeling data configuration definition"]
    #[serde(rename = "dataConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub data_configuration: Option<LabelingDataConfiguration>,
    #[doc = "Instructions for labeling job"]
    #[serde(rename = "jobInstructions", default, skip_serializing_if = "Option::is_none")]
    pub job_instructions: Option<LabelingJobInstructions>,
    #[doc = "Label categories of the job."]
    #[serde(rename = "labelCategories", default, skip_serializing_if = "Option::is_none")]
    pub label_categories: Option<serde_json::Value>,
    #[doc = "Properties of a labeling job"]
    #[serde(rename = "labelingJobMediaProperties", default, skip_serializing_if = "Option::is_none")]
    pub labeling_job_media_properties: Option<LabelingJobMediaProperties>,
    #[doc = "Labeling MLAssist configuration definition"]
    #[serde(rename = "mlAssistConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub ml_assist_configuration: Option<MlAssistConfiguration>,
    #[doc = "Progress metrics definition"]
    #[serde(rename = "progressMetrics", default, skip_serializing_if = "Option::is_none")]
    pub progress_metrics: Option<ProgressMetrics>,
    #[doc = "Internal id of the job(Previously called project)."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "Enum to determine the job provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<JobProvisioningState>,
    #[doc = "Status messages of the job."]
    #[serde(
        rename = "statusMessages",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub status_messages: Vec<StatusMessage>,
}
impl LabelingJob {
    pub fn new(job_base: JobBase) -> Self {
        Self {
            job_base,
            created_date_time: None,
            data_configuration: None,
            job_instructions: None,
            label_categories: None,
            labeling_job_media_properties: None,
            ml_assist_configuration: None,
            progress_metrics: None,
            project_id: None,
            provisioning_state: None,
            status_messages: Vec::new(),
        }
    }
}
#[doc = "Properties of a labeling job for image data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabelingJobImageProperties {
    #[serde(flatten)]
    pub labeling_job_media_properties: LabelingJobMediaProperties,
    #[doc = "Annotation type of image data."]
    #[serde(rename = "annotationType", default, skip_serializing_if = "Option::is_none")]
    pub annotation_type: Option<ImageAnnotationType>,
}
impl LabelingJobImageProperties {
    pub fn new(labeling_job_media_properties: LabelingJobMediaProperties) -> Self {
        Self {
            labeling_job_media_properties,
            annotation_type: None,
        }
    }
}
#[doc = "Instructions for labeling job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabelingJobInstructions {
    #[doc = "The link to a page with detailed labeling instructions for labelers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl LabelingJobInstructions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a labeling job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabelingJobMediaProperties {
    #[doc = "Media type of data asset."]
    #[serde(rename = "mediaType")]
    pub media_type: MediaType,
}
impl LabelingJobMediaProperties {
    pub fn new(media_type: MediaType) -> Self {
        Self { media_type }
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabelingJobResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Labeling job definition"]
    pub properties: LabelingJob,
}
impl LabelingJobResource {
    pub fn new(properties: LabelingJob) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of LabelingJob entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabelingJobResourceArmPaginatedResult {
    #[doc = "The link to the next page of LabelingJob objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type LabelingJob."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<LabelingJobResource>,
}
impl azure_core::Continuable for LabelingJobResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LabelingJobResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a labeling job for text data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabelingJobTextProperties {
    #[serde(flatten)]
    pub labeling_job_media_properties: LabelingJobMediaProperties,
    #[doc = "Annotation type of text data."]
    #[serde(rename = "annotationType", default, skip_serializing_if = "Option::is_none")]
    pub annotation_type: Option<TextAnnotationType>,
}
impl LabelingJobTextProperties {
    pub fn new(labeling_job_media_properties: LabelingJobMediaProperties) -> Self {
        Self {
            labeling_job_media_properties,
            annotation_type: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LakeHouseArtifact {
    #[serde(flatten)]
    pub one_lake_artifact: OneLakeArtifact,
}
impl LakeHouseArtifact {
    pub fn new(one_lake_artifact: OneLakeArtifact) -> Self {
        Self { one_lake_artifact }
    }
}
#[doc = "Learning rate scheduler enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LearningRateScheduler")]
pub enum LearningRateScheduler {
    None,
    WarmupCosine,
    Step,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LearningRateScheduler {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LearningRateScheduler {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LearningRateScheduler {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("LearningRateScheduler", 0u32, "None"),
            Self::WarmupCosine => serializer.serialize_unit_variant("LearningRateScheduler", 1u32, "WarmupCosine"),
            Self::Step => serializer.serialize_unit_variant("LearningRateScheduler", 2u32, "Step"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The List Aml user feature operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListAmlUserFeatureResult {
    #[doc = "The list of AML user facing features."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AmlUserFeature>,
    #[doc = "The URI to fetch the next page of AML user features information. Call ListNext() with this to fetch the next page of AML user features information."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListAmlUserFeatureResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ListAmlUserFeatureResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListNotebookKeysResult {
    #[serde(rename = "primaryAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_access_key: Option<String>,
    #[serde(rename = "secondaryAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_access_key: Option<String>,
}
impl ListNotebookKeysResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListStorageAccountKeysResult {
    #[serde(rename = "userStorageKey", default, skip_serializing_if = "Option::is_none")]
    pub user_storage_key: Option<String>,
}
impl ListStorageAccountKeysResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Usages operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListUsagesResult {
    #[doc = "The list of AML resource usages."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Usage>,
    #[doc = "The URI to fetch the next page of AML resource usage information. Call ListNext() with this to fetch the next page of AML resource usage information."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListUsagesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ListUsagesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ListViewType")]
pub enum ListViewType {
    ActiveOnly,
    ArchivedOnly,
    All,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ListViewType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ListViewType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ListViewType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ActiveOnly => serializer.serialize_unit_variant("ListViewType", 0u32, "ActiveOnly"),
            Self::ArchivedOnly => serializer.serialize_unit_variant("ListViewType", 1u32, "ArchivedOnly"),
            Self::All => serializer.serialize_unit_variant("ListViewType", 2u32, "All"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListWorkspaceKeysResult {
    #[serde(rename = "userStorageKey", default, skip_serializing_if = "Option::is_none")]
    pub user_storage_key: Option<String>,
    #[serde(rename = "userStorageResourceId", default, skip_serializing_if = "Option::is_none")]
    pub user_storage_resource_id: Option<String>,
    #[serde(rename = "appInsightsInstrumentationKey", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_instrumentation_key: Option<String>,
    #[serde(rename = "containerRegistryCredentials", default, skip_serializing_if = "Option::is_none")]
    pub container_registry_credentials: Option<RegistryListCredentialsResult>,
    #[serde(rename = "notebookAccessKeys", default, skip_serializing_if = "Option::is_none")]
    pub notebook_access_keys: Option<ListNotebookKeysResult>,
}
impl ListWorkspaceKeysResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List WorkspaceQuotasByVMFamily operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListWorkspaceQuotas {
    #[doc = "The list of Workspace Quotas by VM Family"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ResourceQuota>,
    #[doc = "The URI to fetch the next page of workspace quota information by VM Family. Call ListNext() with this to fetch the next page of Workspace Quota information."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListWorkspaceQuotas {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ListWorkspaceQuotas {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Literal input type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiteralJobInput {
    #[serde(flatten)]
    pub job_input: JobInput,
    #[doc = "[Required] Literal value for the input."]
    pub value: String,
}
impl LiteralJobInput {
    pub fn new(job_input: JobInput, value: String) -> Self {
        Self { job_input, value }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LogTrainingMetrics")]
pub enum LogTrainingMetrics {
    Enable,
    Disable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LogTrainingMetrics {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LogTrainingMetrics {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LogTrainingMetrics {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enable => serializer.serialize_unit_variant("LogTrainingMetrics", 0u32, "Enable"),
            Self::Disable => serializer.serialize_unit_variant("LogTrainingMetrics", 1u32, "Disable"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LogValidationLoss")]
pub enum LogValidationLoss {
    Enable,
    Disable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LogValidationLoss {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LogValidationLoss {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LogValidationLoss {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enable => serializer.serialize_unit_variant("LogValidationLoss", 0u32, "Enable"),
            Self::Disable => serializer.serialize_unit_variant("LogValidationLoss", 1u32, "Disable"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enum for setting log verbosity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LogVerbosity")]
pub enum LogVerbosity {
    NotSet,
    Debug,
    Info,
    Warning,
    Error,
    Critical,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LogVerbosity {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LogVerbosity {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LogVerbosity {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSet => serializer.serialize_unit_variant("LogVerbosity", 0u32, "NotSet"),
            Self::Debug => serializer.serialize_unit_variant("LogVerbosity", 1u32, "Debug"),
            Self::Info => serializer.serialize_unit_variant("LogVerbosity", 2u32, "Info"),
            Self::Warning => serializer.serialize_unit_variant("LogVerbosity", 3u32, "Warning"),
            Self::Error => serializer.serialize_unit_variant("LogVerbosity", 4u32, "Error"),
            Self::Critical => serializer.serialize_unit_variant("LogVerbosity", 5u32, "Critical"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Labeling MLAssist configuration definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MlAssistConfiguration {
    #[serde(rename = "mlAssist")]
    pub ml_assist: MlAssistConfigurationType,
}
impl MlAssistConfiguration {
    pub fn new(ml_assist: MlAssistConfigurationType) -> Self {
        Self { ml_assist }
    }
}
#[doc = "Labeling MLAssist configuration definition when MLAssist is disabled"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MlAssistConfigurationDisabled {
    #[serde(flatten)]
    pub ml_assist_configuration: MlAssistConfiguration,
}
impl MlAssistConfigurationDisabled {
    pub fn new(ml_assist_configuration: MlAssistConfiguration) -> Self {
        Self { ml_assist_configuration }
    }
}
#[doc = "Labeling MLAssist configuration definition when MLAssist is enabled"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MlAssistConfigurationEnabled {
    #[serde(flatten)]
    pub ml_assist_configuration: MlAssistConfiguration,
    #[doc = "[Required] AML compute binding used in inferencing."]
    #[serde(rename = "inferencingComputeBinding")]
    pub inferencing_compute_binding: String,
    #[doc = "[Required] AML compute binding used in training."]
    #[serde(rename = "trainingComputeBinding")]
    pub training_compute_binding: String,
}
impl MlAssistConfigurationEnabled {
    pub fn new(
        ml_assist_configuration: MlAssistConfiguration,
        inferencing_compute_binding: String,
        training_compute_binding: String,
    ) -> Self {
        Self {
            ml_assist_configuration,
            inferencing_compute_binding,
            training_compute_binding,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MlAssistConfigurationType")]
pub enum MlAssistConfigurationType {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MlAssistConfigurationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MlAssistConfigurationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MlAssistConfigurationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("MlAssistConfigurationType", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("MlAssistConfigurationType", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enum to determine the state of mlflow autologger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MlFlowAutologgerState")]
pub enum MlFlowAutologgerState {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MlFlowAutologgerState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MlFlowAutologgerState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MlFlowAutologgerState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("MlFlowAutologgerState", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("MlFlowAutologgerState", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MlFlowModelJobInput {
    #[serde(flatten)]
    pub asset_job_input: AssetJobInput,
    #[serde(flatten)]
    pub job_input: JobInput,
}
impl MlFlowModelJobInput {
    pub fn new(asset_job_input: AssetJobInput, job_input: JobInput) -> Self {
        Self {
            asset_job_input,
            job_input,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MlFlowModelJobOutput {
    #[serde(flatten)]
    pub asset_job_output: AssetJobOutput,
    #[serde(flatten)]
    pub job_output: JobOutput,
}
impl MlFlowModelJobOutput {
    pub fn new(job_output: JobOutput) -> Self {
        Self {
            asset_job_output: AssetJobOutput::default(),
            job_output,
        }
    }
}
#[doc = "MLTable data definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MlTableData {
    #[serde(flatten)]
    pub data_version_base: DataVersionBase,
    #[doc = "Uris referenced in the MLTable definition (required for lineage)"]
    #[serde(
        rename = "referencedUris",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub referenced_uris: Vec<String>,
}
impl MlTableData {
    pub fn new(data_version_base: DataVersionBase) -> Self {
        Self {
            data_version_base,
            referenced_uris: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MlTableJobInput {
    #[serde(flatten)]
    pub asset_job_input: AssetJobInput,
    #[serde(flatten)]
    pub job_input: JobInput,
}
impl MlTableJobInput {
    pub fn new(asset_job_input: AssetJobInput, job_input: JobInput) -> Self {
        Self {
            asset_job_input,
            job_input,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MlTableJobOutput {
    #[serde(flatten)]
    pub asset_job_output: AssetJobOutput,
    #[serde(flatten)]
    pub job_output: JobOutput,
}
impl MlTableJobOutput {
    pub fn new(job_output: JobOutput) -> Self {
        Self {
            asset_job_output: AssetJobOutput::default(),
            job_output,
        }
    }
}
#[doc = "Managed identity configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedIdentity {
    #[serde(flatten)]
    pub identity_configuration: IdentityConfiguration,
    #[doc = "Specifies a user-assigned identity by client ID. For system-assigned, do not set this field."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Specifies a user-assigned identity by object ID. For system-assigned, do not set this field."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Specifies a user-assigned identity by ARM resource ID. For system-assigned, do not set this field."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl ManagedIdentity {
    pub fn new(identity_configuration: IdentityConfiguration) -> Self {
        Self {
            identity_configuration,
            client_id: None,
            object_id: None,
            resource_id: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedIdentityAuthTypeWorkspaceConnectionProperties {
    #[serde(flatten)]
    pub workspace_connection_properties_v2: WorkspaceConnectionPropertiesV2,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<WorkspaceConnectionManagedIdentity>,
}
impl ManagedIdentityAuthTypeWorkspaceConnectionProperties {
    pub fn new(workspace_connection_properties_v2: WorkspaceConnectionPropertiesV2) -> Self {
        Self {
            workspace_connection_properties_v2,
            credentials: None,
        }
    }
}
#[doc = "Managed Network Provisioning options for managed network of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkProvisionOptions {
    #[serde(rename = "includeSpark", default, skip_serializing_if = "Option::is_none")]
    pub include_spark: Option<bool>,
}
impl ManagedNetworkProvisionOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status of the Provisioning for the managed network of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkProvisionStatus {
    #[doc = "Status for the managed network of a machine learning workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ManagedNetworkStatus>,
    #[serde(rename = "sparkReady", default, skip_serializing_if = "Option::is_none")]
    pub spark_ready: Option<bool>,
}
impl ManagedNetworkProvisionStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed Network settings for a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedNetworkSettings {
    #[doc = "Isolation mode for the managed network of a machine learning workspace."]
    #[serde(rename = "isolationMode", default, skip_serializing_if = "Option::is_none")]
    pub isolation_mode: Option<IsolationMode>,
    #[serde(rename = "networkId", default, skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[serde(rename = "outboundRules", default, skip_serializing_if = "Option::is_none")]
    pub outbound_rules: Option<serde_json::Value>,
    #[doc = "Status of the Provisioning for the managed network of a machine learning workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ManagedNetworkProvisionStatus>,
}
impl ManagedNetworkSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status for the managed network of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedNetworkStatus")]
pub enum ManagedNetworkStatus {
    Inactive,
    Active,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedNetworkStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedNetworkStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedNetworkStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Inactive => serializer.serialize_unit_variant("ManagedNetworkStatus", 0u32, "Inactive"),
            Self::Active => serializer.serialize_unit_variant("ManagedNetworkStatus", 1u32, "Active"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties specific to a ManagedOnlineDeployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedOnlineDeployment {
    #[serde(flatten)]
    pub online_deployment: OnlineDeployment,
}
impl ManagedOnlineDeployment {
    pub fn new(online_deployment: OnlineDeployment) -> Self {
        Self { online_deployment }
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: ManagedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedServiceIdentityType")]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned,UserAssigned")]
    SystemAssignedUserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 2u32, "UserAssigned"),
            Self::SystemAssignedUserAssigned => {
                serializer.serialize_unit_variant("ManagedServiceIdentityType", 3u32, "SystemAssigned,UserAssigned")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Dto object representing compute resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaterializationComputeResource {
    #[doc = "Specifies the instance type"]
    #[serde(rename = "instanceType", default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
}
impl MaterializationComputeResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaterializationSettings {
    #[doc = "Configuration for notification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification: Option<NotificationSetting>,
    #[doc = "Dto object representing compute resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<MaterializationComputeResource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<RecurrenceTrigger>,
    #[doc = "Specifies the spark compute settings"]
    #[serde(rename = "sparkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub spark_configuration: Option<serde_json::Value>,
    #[serde(rename = "storeType", default, skip_serializing_if = "Option::is_none")]
    pub store_type: Option<MaterializationStoreType>,
}
impl MaterializationSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MaterializationStoreType")]
pub enum MaterializationStoreType {
    None,
    Online,
    Offline,
    OnlineAndOffline,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MaterializationStoreType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MaterializationStoreType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MaterializationStoreType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("MaterializationStoreType", 0u32, "None"),
            Self::Online => serializer.serialize_unit_variant("MaterializationStoreType", 1u32, "Online"),
            Self::Offline => serializer.serialize_unit_variant("MaterializationStoreType", 2u32, "Offline"),
            Self::OnlineAndOffline => serializer.serialize_unit_variant("MaterializationStoreType", 3u32, "OnlineAndOffline"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Media type of data asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MediaType")]
pub enum MediaType {
    Image,
    Text,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MediaType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MediaType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MediaType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Image => serializer.serialize_unit_variant("MediaType", 0u32, "Image"),
            Self::Text => serializer.serialize_unit_variant("MediaType", 1u32, "Text"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines an early termination policy based on running averages of the primary metric of all runs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MedianStoppingPolicy {
    #[serde(flatten)]
    pub early_termination_policy: EarlyTerminationPolicy,
}
impl MedianStoppingPolicy {
    pub fn new(early_termination_policy: EarlyTerminationPolicy) -> Self {
        Self { early_termination_policy }
    }
}
#[doc = "Model configuration options."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelConfiguration {
    #[doc = "Mounting type of the model or the inputs"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<PackageInputDeliveryMode>,
    #[doc = "Relative mounting path of the model in the target image."]
    #[serde(rename = "mountPath", default, skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
}
impl ModelConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelContainer {
    #[serde(flatten)]
    pub asset_container: AssetContainer,
    #[doc = "Provisioning state of registry asset."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AssetProvisioningState>,
}
impl ModelContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelContainerResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: ModelContainer,
}
impl ModelContainerResource {
    pub fn new(properties: ModelContainer) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of ModelContainer entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelContainerResourceArmPaginatedResult {
    #[doc = "The link to the next page of ModelContainer objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type ModelContainer."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ModelContainerResource>,
}
impl azure_core::Continuable for ModelContainerResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ModelContainerResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model package input options."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelPackageInput {
    #[doc = "Type of the inputs."]
    #[serde(rename = "inputType")]
    pub input_type: PackageInputType,
    #[doc = "Mounting type of the model or the inputs"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<PackageInputDeliveryMode>,
    #[doc = "Relative mount path of the input in the target image."]
    #[serde(rename = "mountPath", default, skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
    pub path: PackageInputPathBase,
}
impl ModelPackageInput {
    pub fn new(input_type: PackageInputType, path: PackageInputPathBase) -> Self {
        Self {
            input_type,
            mode: None,
            mount_path: None,
            path,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelPerformanceMetricThresholdBase {
    #[serde(rename = "modelType")]
    pub model_type: MonitoringModelType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<MonitoringThreshold>,
}
impl ModelPerformanceMetricThresholdBase {
    pub fn new(model_type: MonitoringModelType) -> Self {
        Self {
            model_type,
            threshold: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelPerformanceSignalBase {
    #[serde(flatten)]
    pub monitoring_signal_base: MonitoringSignalBase,
    #[serde(rename = "baselineData")]
    pub baseline_data: MonitoringInputData,
    #[serde(rename = "dataSegment", default, skip_serializing_if = "Option::is_none")]
    pub data_segment: Option<MonitoringDataSegment>,
    #[serde(rename = "metricThreshold")]
    pub metric_threshold: ModelPerformanceMetricThresholdBase,
    #[serde(rename = "targetData")]
    pub target_data: MonitoringInputData,
}
impl ModelPerformanceSignalBase {
    pub fn new(
        monitoring_signal_base: MonitoringSignalBase,
        baseline_data: MonitoringInputData,
        metric_threshold: ModelPerformanceMetricThresholdBase,
        target_data: MonitoringInputData,
    ) -> Self {
        Self {
            monitoring_signal_base,
            baseline_data,
            data_segment: None,
            metric_threshold,
            target_data,
        }
    }
}
#[doc = "Image model size."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ModelSize")]
pub enum ModelSize {
    None,
    Small,
    Medium,
    Large,
    ExtraLarge,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ModelSize {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ModelSize {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ModelSize {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ModelSize", 0u32, "None"),
            Self::Small => serializer.serialize_unit_variant("ModelSize", 1u32, "Small"),
            Self::Medium => serializer.serialize_unit_variant("ModelSize", 2u32, "Medium"),
            Self::Large => serializer.serialize_unit_variant("ModelSize", 3u32, "Large"),
            Self::ExtraLarge => serializer.serialize_unit_variant("ModelSize", 4u32, "ExtraLarge"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Model asset version details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelVersion {
    #[serde(flatten)]
    pub asset_base: AssetBase,
    #[doc = "Mapping of model flavors to their properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flavors: Option<serde_json::Value>,
    #[doc = "Intellectual Property details for a resource."]
    #[serde(rename = "intellectualProperty", default, skip_serializing_if = "Option::is_none")]
    pub intellectual_property: Option<IntellectualProperty>,
    #[doc = "Name of the training job which produced this model"]
    #[serde(rename = "jobName", default, skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[doc = "The storage format for this entity. Used for NCD."]
    #[serde(rename = "modelType", default, skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    #[doc = "The URI path to the model contents."]
    #[serde(rename = "modelUri", default, skip_serializing_if = "Option::is_none")]
    pub model_uri: Option<String>,
    #[doc = "Provisioning state of registry asset."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AssetProvisioningState>,
    #[doc = "Stage in the model lifecycle assigned to this model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}
impl ModelVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelVersionResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Model asset version details."]
    pub properties: ModelVersion,
}
impl ModelVersionResource {
    pub fn new(properties: ModelVersion) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of ModelVersion entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelVersionResourceArmPaginatedResult {
    #[doc = "The link to the next page of ModelVersion objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type ModelVersion."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ModelVersionResource>,
}
impl azure_core::Continuable for ModelVersionResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ModelVersionResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorDefinition {
    #[serde(rename = "alertNotificationSetting", default, skip_serializing_if = "Option::is_none")]
    pub alert_notification_setting: Option<MonitoringAlertNotificationSettingsBase>,
    #[doc = "[Required] The ARM resource ID of the compute resource to run the monitoring job on."]
    #[serde(rename = "computeId")]
    pub compute_id: String,
    #[doc = "The ARM resource ID of either the model or deployment targeted by this monitor."]
    #[serde(rename = "monitoringTarget", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_target: Option<String>,
    #[doc = "[Required] The signals to monitor."]
    pub signals: serde_json::Value,
}
impl MonitorDefinition {
    pub fn new(compute_id: String, signals: serde_json::Value) -> Self {
        Self {
            alert_notification_setting: None,
            compute_id,
            monitoring_target: None,
            signals,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoringAlertNotificationSettingsBase {
    #[serde(rename = "alertNotificationType")]
    pub alert_notification_type: MonitoringAlertNotificationType,
}
impl MonitoringAlertNotificationSettingsBase {
    pub fn new(alert_notification_type: MonitoringAlertNotificationType) -> Self {
        Self { alert_notification_type }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MonitoringAlertNotificationType")]
pub enum MonitoringAlertNotificationType {
    AzureMonitor,
    Email,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MonitoringAlertNotificationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MonitoringAlertNotificationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MonitoringAlertNotificationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureMonitor => serializer.serialize_unit_variant("MonitoringAlertNotificationType", 0u32, "AzureMonitor"),
            Self::Email => serializer.serialize_unit_variant("MonitoringAlertNotificationType", 1u32, "Email"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringDataSegment {
    #[doc = "The feature to segment the data on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    #[doc = "Filters for only the specified values of the given segmented feature."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl MonitoringDataSegment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MonitoringFeatureDataType")]
pub enum MonitoringFeatureDataType {
    Numerical,
    Categorical,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MonitoringFeatureDataType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MonitoringFeatureDataType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MonitoringFeatureDataType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Numerical => serializer.serialize_unit_variant("MonitoringFeatureDataType", 0u32, "Numerical"),
            Self::Categorical => serializer.serialize_unit_variant("MonitoringFeatureDataType", 1u32, "Categorical"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoringFeatureFilterBase {
    #[serde(rename = "filterType")]
    pub filter_type: MonitoringFeatureFilterType,
}
impl MonitoringFeatureFilterBase {
    pub fn new(filter_type: MonitoringFeatureFilterType) -> Self {
        Self { filter_type }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MonitoringFeatureFilterType")]
pub enum MonitoringFeatureFilterType {
    AllFeatures,
    TopNByAttribution,
    FeatureSubset,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MonitoringFeatureFilterType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MonitoringFeatureFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MonitoringFeatureFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AllFeatures => serializer.serialize_unit_variant("MonitoringFeatureFilterType", 0u32, "AllFeatures"),
            Self::TopNByAttribution => serializer.serialize_unit_variant("MonitoringFeatureFilterType", 1u32, "TopNByAttribution"),
            Self::FeatureSubset => serializer.serialize_unit_variant("MonitoringFeatureFilterType", 2u32, "FeatureSubset"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoringInputData {
    #[doc = "The data asset input to be leveraged by the monitoring job.."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<serde_json::Value>,
    #[serde(rename = "dataContext")]
    pub data_context: MonitoringInputDataContext,
    #[doc = "The ARM resource ID of the component resource used to preprocess the data."]
    #[serde(rename = "preprocessingComponentId", default, skip_serializing_if = "Option::is_none")]
    pub preprocessing_component_id: Option<String>,
    #[doc = "The target column in the given data asset to leverage."]
    #[serde(rename = "targetColumnName", default, skip_serializing_if = "Option::is_none")]
    pub target_column_name: Option<String>,
}
impl MonitoringInputData {
    pub fn new(data_context: MonitoringInputDataContext) -> Self {
        Self {
            asset: None,
            data_context,
            preprocessing_component_id: None,
            target_column_name: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MonitoringInputDataContext")]
pub enum MonitoringInputDataContext {
    ModelInputs,
    ModelOutputs,
    Training,
    Test,
    Validation,
    GroundTruth,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MonitoringInputDataContext {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MonitoringInputDataContext {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MonitoringInputDataContext {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ModelInputs => serializer.serialize_unit_variant("MonitoringInputDataContext", 0u32, "ModelInputs"),
            Self::ModelOutputs => serializer.serialize_unit_variant("MonitoringInputDataContext", 1u32, "ModelOutputs"),
            Self::Training => serializer.serialize_unit_variant("MonitoringInputDataContext", 2u32, "Training"),
            Self::Test => serializer.serialize_unit_variant("MonitoringInputDataContext", 3u32, "Test"),
            Self::Validation => serializer.serialize_unit_variant("MonitoringInputDataContext", 4u32, "Validation"),
            Self::GroundTruth => serializer.serialize_unit_variant("MonitoringInputDataContext", 5u32, "GroundTruth"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MonitoringModelType")]
pub enum MonitoringModelType {
    Classification,
    Regression,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MonitoringModelType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MonitoringModelType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MonitoringModelType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Classification => serializer.serialize_unit_variant("MonitoringModelType", 0u32, "Classification"),
            Self::Regression => serializer.serialize_unit_variant("MonitoringModelType", 1u32, "Regression"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MonitoringNotificationMode")]
pub enum MonitoringNotificationMode {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MonitoringNotificationMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MonitoringNotificationMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MonitoringNotificationMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("MonitoringNotificationMode", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("MonitoringNotificationMode", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoringSignalBase {
    #[doc = "The amount of time a single monitor should look back over the target data on a given run."]
    #[serde(rename = "lookbackPeriod", default, skip_serializing_if = "Option::is_none")]
    pub lookback_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<MonitoringNotificationMode>,
    #[serde(rename = "signalType")]
    pub signal_type: MonitoringSignalType,
}
impl MonitoringSignalBase {
    pub fn new(signal_type: MonitoringSignalType) -> Self {
        Self {
            lookback_period: None,
            mode: None,
            signal_type,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MonitoringSignalType")]
pub enum MonitoringSignalType {
    DataDrift,
    PredictionDrift,
    DataQuality,
    FeatureAttributionDrift,
    Custom,
    ModelPerformance,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MonitoringSignalType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MonitoringSignalType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MonitoringSignalType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DataDrift => serializer.serialize_unit_variant("MonitoringSignalType", 0u32, "DataDrift"),
            Self::PredictionDrift => serializer.serialize_unit_variant("MonitoringSignalType", 1u32, "PredictionDrift"),
            Self::DataQuality => serializer.serialize_unit_variant("MonitoringSignalType", 2u32, "DataQuality"),
            Self::FeatureAttributionDrift => serializer.serialize_unit_variant("MonitoringSignalType", 3u32, "FeatureAttributionDrift"),
            Self::Custom => serializer.serialize_unit_variant("MonitoringSignalType", 4u32, "Custom"),
            Self::ModelPerformance => serializer.serialize_unit_variant("MonitoringSignalType", 5u32, "ModelPerformance"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringThreshold {
    #[doc = "The threshold value. If null, the set default is dependent on the metric type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl MonitoringThreshold {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MPI distribution configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mpi {
    #[serde(flatten)]
    pub distribution_configuration: DistributionConfiguration,
    #[doc = "Number of processes per MPI node."]
    #[serde(rename = "processCountPerInstance", default, skip_serializing_if = "Option::is_none")]
    pub process_count_per_instance: Option<i32>,
}
impl Mpi {
    pub fn new(distribution_configuration: DistributionConfiguration) -> Self {
        Self {
            distribution_configuration,
            process_count_per_instance: None,
        }
    }
}
#[doc = "Whether multiSelect is enabled"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MultiSelect")]
pub enum MultiSelect {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MultiSelect {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MultiSelect {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MultiSelect {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("MultiSelect", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("MultiSelect", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "N-Cross validations value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NCrossValidations {
    #[doc = "Determines how N-Cross validations value is determined."]
    pub mode: NCrossValidationsMode,
}
impl NCrossValidations {
    pub fn new(mode: NCrossValidationsMode) -> Self {
        Self { mode }
    }
}
#[doc = "Determines how N-Cross validations value is determined."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NCrossValidationsMode")]
pub enum NCrossValidationsMode {
    Auto,
    Custom,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NCrossValidationsMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NCrossValidationsMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NCrossValidationsMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Auto => serializer.serialize_unit_variant("NCrossValidationsMode", 0u32, "Auto"),
            Self::Custom => serializer.serialize_unit_variant("NCrossValidationsMode", 1u32, "Custom"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Fixed training parameters that won't be swept over during AutoML NLP training."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NlpFixedParameters {
    #[doc = "Number of steps to accumulate gradients over before running a backward pass."]
    #[serde(rename = "gradientAccumulationSteps", default, skip_serializing_if = "Option::is_none")]
    pub gradient_accumulation_steps: Option<i32>,
    #[doc = "The learning rate for the training procedure."]
    #[serde(rename = "learningRate", default, skip_serializing_if = "Option::is_none")]
    pub learning_rate: Option<f32>,
    #[doc = "Enum of learning rate schedulers that aligns with those supported by HF"]
    #[serde(rename = "learningRateScheduler", default, skip_serializing_if = "Option::is_none")]
    pub learning_rate_scheduler: Option<NlpLearningRateScheduler>,
    #[doc = "The name of the model to train."]
    #[serde(rename = "modelName", default, skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[doc = "Number of training epochs."]
    #[serde(rename = "numberOfEpochs", default, skip_serializing_if = "Option::is_none")]
    pub number_of_epochs: Option<i32>,
    #[doc = "The batch size for the training procedure."]
    #[serde(rename = "trainingBatchSize", default, skip_serializing_if = "Option::is_none")]
    pub training_batch_size: Option<i32>,
    #[doc = "The batch size to be used during evaluation."]
    #[serde(rename = "validationBatchSize", default, skip_serializing_if = "Option::is_none")]
    pub validation_batch_size: Option<i32>,
    #[doc = "The warmup ratio, used alongside LrSchedulerType."]
    #[serde(rename = "warmupRatio", default, skip_serializing_if = "Option::is_none")]
    pub warmup_ratio: Option<f32>,
    #[doc = "The weight decay for the training procedure."]
    #[serde(rename = "weightDecay", default, skip_serializing_if = "Option::is_none")]
    pub weight_decay: Option<f32>,
}
impl NlpFixedParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enum of learning rate schedulers that aligns with those supported by HF"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NlpLearningRateScheduler")]
pub enum NlpLearningRateScheduler {
    None,
    Linear,
    Cosine,
    CosineWithRestarts,
    Polynomial,
    Constant,
    ConstantWithWarmup,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NlpLearningRateScheduler {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NlpLearningRateScheduler {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NlpLearningRateScheduler {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("NlpLearningRateScheduler", 0u32, "None"),
            Self::Linear => serializer.serialize_unit_variant("NlpLearningRateScheduler", 1u32, "Linear"),
            Self::Cosine => serializer.serialize_unit_variant("NlpLearningRateScheduler", 2u32, "Cosine"),
            Self::CosineWithRestarts => serializer.serialize_unit_variant("NlpLearningRateScheduler", 3u32, "CosineWithRestarts"),
            Self::Polynomial => serializer.serialize_unit_variant("NlpLearningRateScheduler", 4u32, "Polynomial"),
            Self::Constant => serializer.serialize_unit_variant("NlpLearningRateScheduler", 5u32, "Constant"),
            Self::ConstantWithWarmup => serializer.serialize_unit_variant("NlpLearningRateScheduler", 6u32, "ConstantWithWarmup"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Stringified search spaces for each parameter. See below examples."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NlpParameterSubspace {
    #[doc = "Number of steps to accumulate gradients over before running a backward pass."]
    #[serde(rename = "gradientAccumulationSteps", default, skip_serializing_if = "Option::is_none")]
    pub gradient_accumulation_steps: Option<String>,
    #[doc = "The learning rate for the training procedure."]
    #[serde(rename = "learningRate", default, skip_serializing_if = "Option::is_none")]
    pub learning_rate: Option<String>,
    #[doc = "The type of learning rate schedule to use during the training procedure."]
    #[serde(rename = "learningRateScheduler", default, skip_serializing_if = "Option::is_none")]
    pub learning_rate_scheduler: Option<String>,
    #[doc = "The name of the model to train."]
    #[serde(rename = "modelName", default, skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[doc = "Number of training epochs."]
    #[serde(rename = "numberOfEpochs", default, skip_serializing_if = "Option::is_none")]
    pub number_of_epochs: Option<String>,
    #[doc = "The batch size for the training procedure."]
    #[serde(rename = "trainingBatchSize", default, skip_serializing_if = "Option::is_none")]
    pub training_batch_size: Option<String>,
    #[doc = "The batch size to be used during evaluation."]
    #[serde(rename = "validationBatchSize", default, skip_serializing_if = "Option::is_none")]
    pub validation_batch_size: Option<String>,
    #[doc = "The warmup ratio, used alongside LrSchedulerType."]
    #[serde(rename = "warmupRatio", default, skip_serializing_if = "Option::is_none")]
    pub warmup_ratio: Option<String>,
    #[doc = "The weight decay for the training procedure."]
    #[serde(rename = "weightDecay", default, skip_serializing_if = "Option::is_none")]
    pub weight_decay: Option<String>,
}
impl NlpParameterSubspace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model sweeping and hyperparameter tuning related settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NlpSweepSettings {
    #[doc = "Early termination policies enable canceling poor-performing runs before they complete"]
    #[serde(rename = "earlyTermination", default, skip_serializing_if = "Option::is_none")]
    pub early_termination: Option<EarlyTerminationPolicy>,
    #[serde(rename = "samplingAlgorithm")]
    pub sampling_algorithm: SamplingAlgorithmType,
}
impl NlpSweepSettings {
    pub fn new(sampling_algorithm: SamplingAlgorithmType) -> Self {
        Self {
            early_termination: None,
            sampling_algorithm,
        }
    }
}
#[doc = "Abstract class for NLP related AutoML tasks.\r\nNLP - Natural Language Processing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NlpVertical {
    #[serde(rename = "featurizationSettings", default, skip_serializing_if = "Option::is_none")]
    pub featurization_settings: Option<NlpVerticalFeaturizationSettings>,
    #[doc = "Fixed training parameters that won't be swept over during AutoML NLP training."]
    #[serde(rename = "fixedParameters", default, skip_serializing_if = "Option::is_none")]
    pub fixed_parameters: Option<NlpFixedParameters>,
    #[doc = "Job execution constraints."]
    #[serde(rename = "limitSettings", default, skip_serializing_if = "Option::is_none")]
    pub limit_settings: Option<NlpVerticalLimitSettings>,
    #[doc = "Search space for sampling different combinations of models and their hyperparameters."]
    #[serde(
        rename = "searchSpace",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub search_space: Vec<NlpParameterSubspace>,
    #[doc = "Model sweeping and hyperparameter tuning related settings."]
    #[serde(rename = "sweepSettings", default, skip_serializing_if = "Option::is_none")]
    pub sweep_settings: Option<NlpSweepSettings>,
    #[serde(rename = "validationData", default, skip_serializing_if = "Option::is_none")]
    pub validation_data: Option<MlTableJobInput>,
}
impl NlpVertical {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NlpVerticalFeaturizationSettings {
    #[serde(flatten)]
    pub featurization_settings: FeaturizationSettings,
}
impl NlpVerticalFeaturizationSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job execution constraints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NlpVerticalLimitSettings {
    #[doc = "Maximum Concurrent AutoML iterations."]
    #[serde(rename = "maxConcurrentTrials", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_trials: Option<i32>,
    #[doc = "Maximum nodes to use for the experiment."]
    #[serde(rename = "maxNodes", default, skip_serializing_if = "Option::is_none")]
    pub max_nodes: Option<i32>,
    #[doc = "Number of AutoML iterations."]
    #[serde(rename = "maxTrials", default, skip_serializing_if = "Option::is_none")]
    pub max_trials: Option<i32>,
    #[doc = "AutoML job timeout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[doc = "Timeout for individual HD trials."]
    #[serde(rename = "trialTimeout", default, skip_serializing_if = "Option::is_none")]
    pub trial_timeout: Option<String>,
}
impl NlpVerticalLimitSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Counts of various compute node states on the amlCompute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeStateCounts {
    #[doc = "Number of compute nodes in idle state."]
    #[serde(rename = "idleNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub idle_node_count: Option<i32>,
    #[doc = "Number of compute nodes which are running jobs."]
    #[serde(rename = "runningNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub running_node_count: Option<i32>,
    #[doc = "Number of compute nodes which are being prepared."]
    #[serde(rename = "preparingNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub preparing_node_count: Option<i32>,
    #[doc = "Number of compute nodes which are in unusable state."]
    #[serde(rename = "unusableNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub unusable_node_count: Option<i32>,
    #[doc = "Number of compute nodes which are leaving the amlCompute."]
    #[serde(rename = "leavingNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub leaving_node_count: Option<i32>,
    #[doc = "Number of compute nodes which are in preempted state."]
    #[serde(rename = "preemptedNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub preempted_node_count: Option<i32>,
}
impl NodeStateCounts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Abstract Nodes definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Nodes {
    #[doc = "The enumerated types for the nodes value"]
    #[serde(rename = "nodesValueType")]
    pub nodes_value_type: NodesValueType,
}
impl Nodes {
    pub fn new(nodes_value_type: NodesValueType) -> Self {
        Self { nodes_value_type }
    }
}
#[doc = "The enumerated types for the nodes value"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NodesValueType")]
pub enum NodesValueType {
    All,
    Custom,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NodesValueType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NodesValueType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NodesValueType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::All => serializer.serialize_unit_variant("NodesValueType", 0u32, "All"),
            Self::Custom => serializer.serialize_unit_variant("NodesValueType", 1u32, "Custom"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NoneAuthTypeWorkspaceConnectionProperties {
    #[serde(flatten)]
    pub workspace_connection_properties_v2: WorkspaceConnectionPropertiesV2,
}
impl NoneAuthTypeWorkspaceConnectionProperties {
    pub fn new(workspace_connection_properties_v2: WorkspaceConnectionPropertiesV2) -> Self {
        Self {
            workspace_connection_properties_v2,
        }
    }
}
#[doc = "Empty/none datastore credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NoneDatastoreCredentials {
    #[serde(flatten)]
    pub datastore_credentials: DatastoreCredentials,
}
impl NoneDatastoreCredentials {
    pub fn new(datastore_credentials: DatastoreCredentials) -> Self {
        Self { datastore_credentials }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotebookAccessTokenResult {
    #[serde(rename = "notebookResourceId", default, skip_serializing_if = "Option::is_none")]
    pub notebook_resource_id: Option<String>,
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[serde(rename = "publicDns", default, skip_serializing_if = "Option::is_none")]
    pub public_dns: Option<String>,
    #[serde(rename = "accessToken", default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "tokenType", default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[serde(rename = "expiresIn", default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i32>,
    #[serde(rename = "refreshToken", default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
impl NotebookAccessTokenResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotebookPreparationError {
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
}
impl NotebookPreparationError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotebookResourceInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "the data plane resourceId that used to initialize notebook component"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "notebookPreparationError", default, skip_serializing_if = "Option::is_none")]
    pub notebook_preparation_error: Option<NotebookPreparationError>,
}
impl NotebookResourceInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration for notification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationSetting {
    #[doc = "Send email notification to user on specified notification type"]
    #[serde(
        rename = "emailOn",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub email_on: Vec<EmailNotificationEnableType>,
    #[doc = "This is the email recipient list which has a limitation of 499 characters in total concat with comma separator"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub emails: Vec<String>,
    #[doc = "Send webhook callback to a service. Key is a user-provided name for the webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<serde_json::Value>,
}
impl NotificationSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NumericalDataDriftMetric")]
pub enum NumericalDataDriftMetric {
    JensenShannonDistance,
    PopulationStabilityIndex,
    NormalizedWassersteinDistance,
    TwoSampleKolmogorovSmirnovTest,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NumericalDataDriftMetric {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NumericalDataDriftMetric {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NumericalDataDriftMetric {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::JensenShannonDistance => serializer.serialize_unit_variant("NumericalDataDriftMetric", 0u32, "JensenShannonDistance"),
            Self::PopulationStabilityIndex => {
                serializer.serialize_unit_variant("NumericalDataDriftMetric", 1u32, "PopulationStabilityIndex")
            }
            Self::NormalizedWassersteinDistance => {
                serializer.serialize_unit_variant("NumericalDataDriftMetric", 2u32, "NormalizedWassersteinDistance")
            }
            Self::TwoSampleKolmogorovSmirnovTest => {
                serializer.serialize_unit_variant("NumericalDataDriftMetric", 3u32, "TwoSampleKolmogorovSmirnovTest")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumericalDataDriftMetricThreshold {
    #[serde(flatten)]
    pub data_drift_metric_threshold_base: DataDriftMetricThresholdBase,
    pub metric: NumericalDataDriftMetric,
}
impl NumericalDataDriftMetricThreshold {
    pub fn new(data_drift_metric_threshold_base: DataDriftMetricThresholdBase, metric: NumericalDataDriftMetric) -> Self {
        Self {
            data_drift_metric_threshold_base,
            metric,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NumericalDataQualityMetric")]
pub enum NumericalDataQualityMetric {
    NullValueRate,
    DataTypeErrorRate,
    OutOfBoundsRate,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NumericalDataQualityMetric {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NumericalDataQualityMetric {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NumericalDataQualityMetric {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NullValueRate => serializer.serialize_unit_variant("NumericalDataQualityMetric", 0u32, "NullValueRate"),
            Self::DataTypeErrorRate => serializer.serialize_unit_variant("NumericalDataQualityMetric", 1u32, "DataTypeErrorRate"),
            Self::OutOfBoundsRate => serializer.serialize_unit_variant("NumericalDataQualityMetric", 2u32, "OutOfBoundsRate"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumericalDataQualityMetricThreshold {
    #[serde(flatten)]
    pub data_quality_metric_threshold_base: DataQualityMetricThresholdBase,
    pub metric: NumericalDataQualityMetric,
}
impl NumericalDataQualityMetricThreshold {
    pub fn new(data_quality_metric_threshold_base: DataQualityMetricThresholdBase, metric: NumericalDataQualityMetric) -> Self {
        Self {
            data_quality_metric_threshold_base,
            metric,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NumericalPredictionDriftMetric")]
pub enum NumericalPredictionDriftMetric {
    JensenShannonDistance,
    PopulationStabilityIndex,
    NormalizedWassersteinDistance,
    TwoSampleKolmogorovSmirnovTest,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NumericalPredictionDriftMetric {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NumericalPredictionDriftMetric {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NumericalPredictionDriftMetric {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::JensenShannonDistance => {
                serializer.serialize_unit_variant("NumericalPredictionDriftMetric", 0u32, "JensenShannonDistance")
            }
            Self::PopulationStabilityIndex => {
                serializer.serialize_unit_variant("NumericalPredictionDriftMetric", 1u32, "PopulationStabilityIndex")
            }
            Self::NormalizedWassersteinDistance => {
                serializer.serialize_unit_variant("NumericalPredictionDriftMetric", 2u32, "NormalizedWassersteinDistance")
            }
            Self::TwoSampleKolmogorovSmirnovTest => {
                serializer.serialize_unit_variant("NumericalPredictionDriftMetric", 3u32, "TwoSampleKolmogorovSmirnovTest")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumericalPredictionDriftMetricThreshold {
    #[serde(flatten)]
    pub prediction_drift_metric_threshold_base: PredictionDriftMetricThresholdBase,
    pub metric: NumericalPredictionDriftMetric,
}
impl NumericalPredictionDriftMetricThreshold {
    pub fn new(prediction_drift_metric_threshold_base: PredictionDriftMetricThresholdBase, metric: NumericalPredictionDriftMetric) -> Self {
        Self {
            prediction_drift_metric_threshold_base,
            metric,
        }
    }
}
#[doc = "Primary metrics for Image ObjectDetection task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ObjectDetectionPrimaryMetrics")]
pub enum ObjectDetectionPrimaryMetrics {
    MeanAveragePrecision,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ObjectDetectionPrimaryMetrics {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ObjectDetectionPrimaryMetrics {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ObjectDetectionPrimaryMetrics {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::MeanAveragePrecision => serializer.serialize_unit_variant("ObjectDetectionPrimaryMetrics", 0u32, "MeanAveragePrecision"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Optimization objective."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Objective {
    #[doc = "Defines supported metric goals for hyperparameter tuning"]
    pub goal: Goal,
    #[doc = "[Required] Name of the metric to optimize."]
    #[serde(rename = "primaryMetric")]
    pub primary_metric: String,
}
impl Objective {
    pub fn new(goal: Goal, primary_metric: String) -> Self {
        Self { goal, primary_metric }
    }
}
#[doc = "OneLake artifact (data source) configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OneLakeArtifact {
    #[doc = "[Required] OneLake artifact name"]
    #[serde(rename = "artifactName")]
    pub artifact_name: String,
    #[doc = "Enum to determine OneLake artifact type."]
    #[serde(rename = "artifactType")]
    pub artifact_type: OneLakeArtifactType,
}
impl OneLakeArtifact {
    pub fn new(artifact_name: String, artifact_type: OneLakeArtifactType) -> Self {
        Self {
            artifact_name,
            artifact_type,
        }
    }
}
#[doc = "Enum to determine OneLake artifact type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OneLakeArtifactType")]
pub enum OneLakeArtifactType {
    LakeHouse,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OneLakeArtifactType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OneLakeArtifactType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OneLakeArtifactType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::LakeHouse => serializer.serialize_unit_variant("OneLakeArtifactType", 0u32, "LakeHouse"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "OneLake (Trident) datastore configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OneLakeDatastore {
    #[serde(flatten)]
    pub datastore: Datastore,
    #[doc = "OneLake artifact (data source) configuration."]
    pub artifact: OneLakeArtifact,
    #[doc = "OneLake endpoint to use for the datastore."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "[Required] OneLake workspace name."]
    #[serde(rename = "oneLakeWorkspaceName")]
    pub one_lake_workspace_name: String,
    #[serde(rename = "serviceDataAccessAuthIdentity", default, skip_serializing_if = "Option::is_none")]
    pub service_data_access_auth_identity: Option<ServiceDataAccessAuthIdentity>,
}
impl OneLakeDatastore {
    pub fn new(datastore: Datastore, artifact: OneLakeArtifact, one_lake_workspace_name: String) -> Self {
        Self {
            datastore,
            artifact,
            endpoint: None,
            one_lake_workspace_name,
            service_data_access_auth_identity: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnlineDeployment {
    #[serde(flatten)]
    pub endpoint_deployment_properties_base: EndpointDeploymentPropertiesBase,
    #[doc = "If true, enables Application Insights logging."]
    #[serde(rename = "appInsightsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_enabled: Option<bool>,
    #[serde(rename = "dataCollector", default, skip_serializing_if = "Option::is_none")]
    pub data_collector: Option<DataCollector>,
    #[doc = "Enum to determine whether PublicNetworkAccess is Enabled or Disabled for egress of a deployment."]
    #[serde(rename = "egressPublicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub egress_public_network_access: Option<EgressPublicNetworkAccessType>,
    #[doc = "Enum to determine endpoint compute type."]
    #[serde(rename = "endpointComputeType")]
    pub endpoint_compute_type: EndpointComputeType,
    #[doc = "Compute instance type."]
    #[serde(rename = "instanceType", default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[doc = "Deployment container liveness/readiness probe configuration."]
    #[serde(rename = "livenessProbe", default, skip_serializing_if = "Option::is_none")]
    pub liveness_probe: Option<ProbeSettings>,
    #[doc = "The URI path to the model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[doc = "The path to mount the model in custom container."]
    #[serde(rename = "modelMountPath", default, skip_serializing_if = "Option::is_none")]
    pub model_mount_path: Option<String>,
    #[doc = "Possible values for DeploymentProvisioningState."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DeploymentProvisioningState>,
    #[doc = "Deployment container liveness/readiness probe configuration."]
    #[serde(rename = "readinessProbe", default, skip_serializing_if = "Option::is_none")]
    pub readiness_probe: Option<ProbeSettings>,
    #[doc = "Online deployment scoring requests configuration."]
    #[serde(rename = "requestSettings", default, skip_serializing_if = "Option::is_none")]
    pub request_settings: Option<OnlineRequestSettings>,
    #[doc = "Online deployment scaling configuration."]
    #[serde(rename = "scaleSettings", default, skip_serializing_if = "Option::is_none")]
    pub scale_settings: Option<OnlineScaleSettings>,
}
impl OnlineDeployment {
    pub fn new(endpoint_compute_type: EndpointComputeType) -> Self {
        Self {
            endpoint_deployment_properties_base: EndpointDeploymentPropertiesBase::default(),
            app_insights_enabled: None,
            data_collector: None,
            egress_public_network_access: None,
            endpoint_compute_type,
            instance_type: None,
            liveness_probe: None,
            model: None,
            model_mount_path: None,
            provisioning_state: None,
            readiness_probe: None,
            request_settings: None,
            scale_settings: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnlineDeploymentTrackedResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Metadata used by portal/tooling/etc to render different UX experiences for resources of the same type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub properties: OnlineDeployment,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl OnlineDeploymentTrackedResource {
    pub fn new(tracked_resource: TrackedResource, properties: OnlineDeployment) -> Self {
        Self {
            tracked_resource,
            identity: None,
            kind: None,
            properties,
            sku: None,
        }
    }
}
#[doc = "A paginated list of OnlineDeployment entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OnlineDeploymentTrackedResourceArmPaginatedResult {
    #[doc = "The link to the next page of OnlineDeployment objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type OnlineDeployment."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OnlineDeploymentTrackedResource>,
}
impl azure_core::Continuable for OnlineDeploymentTrackedResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OnlineDeploymentTrackedResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Online endpoint configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnlineEndpoint {
    #[serde(flatten)]
    pub endpoint_properties_base: EndpointPropertiesBase,
    #[doc = "ARM resource ID of the compute if it exists.\r\noptional"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compute: Option<String>,
    #[doc = "Percentage of traffic to be mirrored to each deployment without using returned scoring. Traffic values need to sum to utmost 50."]
    #[serde(rename = "mirrorTraffic", default, skip_serializing_if = "Option::is_none")]
    pub mirror_traffic: Option<serde_json::Value>,
    #[doc = "State of endpoint provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<EndpointProvisioningState>,
    #[doc = "Enum to determine whether PublicNetworkAccess is Enabled or Disabled."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccessType>,
    #[doc = "Percentage of traffic from endpoint to divert to each deployment. Traffic values need to sum to 100."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traffic: Option<serde_json::Value>,
}
impl OnlineEndpoint {
    pub fn new(endpoint_properties_base: EndpointPropertiesBase) -> Self {
        Self {
            endpoint_properties_base,
            compute: None,
            mirror_traffic: None,
            provisioning_state: None,
            public_network_access: None,
            traffic: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnlineEndpointTrackedResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Metadata used by portal/tooling/etc to render different UX experiences for resources of the same type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Online endpoint configuration"]
    pub properties: OnlineEndpoint,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl OnlineEndpointTrackedResource {
    pub fn new(tracked_resource: TrackedResource, properties: OnlineEndpoint) -> Self {
        Self {
            tracked_resource,
            identity: None,
            kind: None,
            properties,
            sku: None,
        }
    }
}
#[doc = "A paginated list of OnlineEndpoint entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OnlineEndpointTrackedResourceArmPaginatedResult {
    #[doc = "The link to the next page of OnlineEndpoint objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type OnlineEndpoint."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OnlineEndpointTrackedResource>,
}
impl azure_core::Continuable for OnlineEndpointTrackedResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OnlineEndpointTrackedResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Online inference configuration options."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OnlineInferenceConfiguration {
    #[doc = "Additional configurations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configurations: Option<serde_json::Value>,
    #[doc = "Entry script or command to invoke."]
    #[serde(rename = "entryScript", default, skip_serializing_if = "Option::is_none")]
    pub entry_script: Option<String>,
    #[serde(rename = "livenessRoute", default, skip_serializing_if = "Option::is_none")]
    pub liveness_route: Option<Route>,
    #[serde(rename = "readinessRoute", default, skip_serializing_if = "Option::is_none")]
    pub readiness_route: Option<Route>,
    #[serde(rename = "scoringRoute", default, skip_serializing_if = "Option::is_none")]
    pub scoring_route: Option<Route>,
}
impl OnlineInferenceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Online deployment scoring requests configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OnlineRequestSettings {
    #[doc = "The number of maximum concurrent requests per node allowed per deployment. Defaults to 1."]
    #[serde(rename = "maxConcurrentRequestsPerInstance", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_requests_per_instance: Option<i32>,
    #[doc = "The maximum amount of time a request will stay in the queue in ISO 8601 format.\r\nDefaults to 500ms."]
    #[serde(rename = "maxQueueWait", default, skip_serializing_if = "Option::is_none")]
    pub max_queue_wait: Option<String>,
    #[doc = "The scoring timeout in ISO 8601 format.\r\nDefaults to 5000ms."]
    #[serde(rename = "requestTimeout", default, skip_serializing_if = "Option::is_none")]
    pub request_timeout: Option<String>,
}
impl OnlineRequestSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Online deployment scaling configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnlineScaleSettings {
    #[serde(rename = "scaleType")]
    pub scale_type: ScaleType,
}
impl OnlineScaleSettings {
    pub fn new(scale_type: ScaleType) -> Self {
        Self { scale_type }
    }
}
#[doc = "The type of operating system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperatingSystemType")]
pub enum OperatingSystemType {
    Linux,
    Windows,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperatingSystemType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperatingSystemType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperatingSystemType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Linux => serializer.serialize_unit_variant("OperatingSystemType", 0u32, "Linux"),
            Self::Windows => serializer.serialize_unit_variant("OperatingSystemType", 1u32, "Windows"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OrderString")]
pub enum OrderString {
    CreatedAtDesc,
    CreatedAtAsc,
    UpdatedAtDesc,
    UpdatedAtAsc,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OrderString {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OrderString {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OrderString {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CreatedAtDesc => serializer.serialize_unit_variant("OrderString", 0u32, "CreatedAtDesc"),
            Self::CreatedAtAsc => serializer.serialize_unit_variant("OrderString", 1u32, "CreatedAtAsc"),
            Self::UpdatedAtDesc => serializer.serialize_unit_variant("OrderString", 2u32, "UpdatedAtDesc"),
            Self::UpdatedAtAsc => serializer.serialize_unit_variant("OrderString", 3u32, "UpdatedAtAsc"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Outbound Rule for the managed network of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutboundRule {
    #[doc = "Type of a managed network Outbound Rule of a machine learning workspace."]
    #[serde(rename = "type")]
    pub type_: RuleType,
    #[doc = "Status of a managed network Outbound Rule of a machine learning workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RuleStatus>,
    #[doc = "Category of a managed network Outbound Rule of a machine learning workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<RuleCategory>,
}
impl OutboundRule {
    pub fn new(type_: RuleType) -> Self {
        Self {
            type_,
            status: None,
            category: None,
        }
    }
}
#[doc = "Outbound Rule Basic Resource for the managed network of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutboundRuleBasicResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Outbound Rule for the managed network of a machine learning workspace."]
    pub properties: OutboundRule,
}
impl OutboundRuleBasicResource {
    pub fn new(properties: OutboundRule) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "List of outbound rules for the managed network of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundRuleListResult {
    #[doc = "The list of machine learning workspaces. Since this list may be incomplete, the nextLink field should be used to request the next list of machine learning workspaces."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OutboundRuleBasicResource>,
    #[doc = "The link to the next page constructed using the continuationToken.  If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OutboundRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OutboundRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Output data delivery mode enums."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OutputDeliveryMode")]
pub enum OutputDeliveryMode {
    ReadWriteMount,
    Upload,
    Direct,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OutputDeliveryMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OutputDeliveryMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OutputDeliveryMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ReadWriteMount => serializer.serialize_unit_variant("OutputDeliveryMode", 0u32, "ReadWriteMount"),
            Self::Upload => serializer.serialize_unit_variant("OutputDeliveryMode", 1u32, "Upload"),
            Self::Direct => serializer.serialize_unit_variant("OutputDeliveryMode", 2u32, "Direct"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Reference to an asset via its path in a job output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputPathAssetReference {
    #[serde(flatten)]
    pub asset_reference_base: AssetReferenceBase,
    #[doc = "ARM resource ID of the job."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "The path of the file/directory in the job output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl OutputPathAssetReference {
    pub fn new(asset_reference_base: AssetReferenceBase) -> Self {
        Self {
            asset_reference_base,
            job_id: None,
            path: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatAuthTypeWorkspaceConnectionProperties {
    #[serde(flatten)]
    pub workspace_connection_properties_v2: WorkspaceConnectionPropertiesV2,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<WorkspaceConnectionPersonalAccessToken>,
}
impl PatAuthTypeWorkspaceConnectionProperties {
    pub fn new(workspace_connection_properties_v2: WorkspaceConnectionPropertiesV2) -> Self {
        Self {
            workspace_connection_properties_v2,
            credentials: None,
        }
    }
}
#[doc = "Package build state returned in package response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PackageBuildState")]
pub enum PackageBuildState {
    NotStarted,
    Running,
    Succeeded,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PackageBuildState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PackageBuildState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PackageBuildState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotStarted => serializer.serialize_unit_variant("PackageBuildState", 0u32, "NotStarted"),
            Self::Running => serializer.serialize_unit_variant("PackageBuildState", 1u32, "Running"),
            Self::Succeeded => serializer.serialize_unit_variant("PackageBuildState", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("PackageBuildState", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Mounting type of the model or the inputs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PackageInputDeliveryMode")]
pub enum PackageInputDeliveryMode {
    ReadOnlyMount,
    Download,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PackageInputDeliveryMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PackageInputDeliveryMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PackageInputDeliveryMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ReadOnlyMount => serializer.serialize_unit_variant("PackageInputDeliveryMode", 0u32, "ReadOnlyMount"),
            Self::Download => serializer.serialize_unit_variant("PackageInputDeliveryMode", 1u32, "Download"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageInputPathBase {
    #[doc = "Input path type for package inputs."]
    #[serde(rename = "inputPathType")]
    pub input_path_type: InputPathType,
}
impl PackageInputPathBase {
    pub fn new(input_path_type: InputPathType) -> Self {
        Self { input_path_type }
    }
}
#[doc = "Package input path specified with a resource id."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageInputPathId {
    #[serde(flatten)]
    pub package_input_path_base: PackageInputPathBase,
    #[doc = "Input resource id."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl PackageInputPathId {
    pub fn new(package_input_path_base: PackageInputPathBase) -> Self {
        Self {
            package_input_path_base,
            resource_id: None,
        }
    }
}
#[doc = "Package input path specified as an url."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageInputPathUrl {
    #[serde(flatten)]
    pub package_input_path_base: PackageInputPathBase,
    #[doc = "Input path url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl PackageInputPathUrl {
    pub fn new(package_input_path_base: PackageInputPathBase) -> Self {
        Self {
            package_input_path_base,
            url: None,
        }
    }
}
#[doc = "Package input path specified with name and version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageInputPathVersion {
    #[serde(flatten)]
    pub package_input_path_base: PackageInputPathBase,
    #[doc = "Input resource name."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "Input resource version."]
    #[serde(rename = "resourceVersion", default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,
}
impl PackageInputPathVersion {
    pub fn new(package_input_path_base: PackageInputPathBase) -> Self {
        Self {
            package_input_path_base,
            resource_name: None,
            resource_version: None,
        }
    }
}
#[doc = "Type of the inputs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PackageInputType")]
pub enum PackageInputType {
    UriFile,
    UriFolder,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PackageInputType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PackageInputType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PackageInputType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UriFile => serializer.serialize_unit_variant("PackageInputType", 0u32, "UriFile"),
            Self::UriFolder => serializer.serialize_unit_variant("PackageInputType", 1u32, "UriFolder"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Model package operation request properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageRequest {
    #[serde(rename = "baseEnvironmentSource", default, skip_serializing_if = "Option::is_none")]
    pub base_environment_source: Option<BaseEnvironmentSource>,
    #[doc = "Collection of environment variables."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[serde(rename = "inferencingServer")]
    pub inferencing_server: InferencingServer,
    #[doc = "Collection of inputs."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inputs: Vec<ModelPackageInput>,
    #[doc = "Model configuration options."]
    #[serde(rename = "modelConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub model_configuration: Option<ModelConfiguration>,
    #[doc = "Tag dictionary. Tags can be added, removed, and updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "[Required] Target environment name to be generated by package."]
    #[serde(rename = "targetEnvironmentName")]
    pub target_environment_name: String,
    #[doc = "Target environment version to be generated by package."]
    #[serde(rename = "targetEnvironmentVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_environment_version: Option<String>,
}
impl PackageRequest {
    pub fn new(inferencing_server: InferencingServer, target_environment_name: String) -> Self {
        Self {
            base_environment_source: None,
            environment_variables: None,
            inferencing_server,
            inputs: Vec::new(),
            model_configuration: None,
            tags: None,
            target_environment_name,
            target_environment_version: None,
        }
    }
}
#[doc = "Package response returned after async package operation completes successfully."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageResponse {
    #[serde(rename = "baseEnvironmentSource", default, skip_serializing_if = "Option::is_none")]
    pub base_environment_source: Option<BaseEnvironmentSource>,
    #[doc = "Build id of the image build operation."]
    #[serde(rename = "buildId", default, skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
    #[doc = "Package build state returned in package response."]
    #[serde(rename = "buildState", default, skip_serializing_if = "Option::is_none")]
    pub build_state: Option<PackageBuildState>,
    #[doc = "Collection of environment variables."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[serde(rename = "inferencingServer", default, skip_serializing_if = "Option::is_none")]
    pub inferencing_server: Option<InferencingServer>,
    #[doc = "Collection of inputs."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inputs: Vec<ModelPackageInput>,
    #[doc = "Log url of the image build operation."]
    #[serde(rename = "logUrl", default, skip_serializing_if = "Option::is_none")]
    pub log_url: Option<String>,
    #[doc = "Model configuration options."]
    #[serde(rename = "modelConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub model_configuration: Option<ModelConfiguration>,
    #[doc = "Tag dictionary. Tags can be added, removed, and updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Asset ID of the target environment created by package operation."]
    #[serde(rename = "targetEnvironmentId", default, skip_serializing_if = "Option::is_none")]
    pub target_environment_id: Option<String>,
    #[doc = "Target environment name to be generated by package."]
    #[serde(rename = "targetEnvironmentName", default, skip_serializing_if = "Option::is_none")]
    pub target_environment_name: Option<String>,
    #[doc = "Target environment version to be generated by package."]
    #[serde(rename = "targetEnvironmentVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_environment_version: Option<String>,
}
impl PackageResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paginated list of Machine Learning compute objects wrapped in ARM resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedComputeResourcesList {
    #[doc = "An array of Machine Learning compute objects wrapped in ARM resource envelope."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ComputeResource>,
    #[doc = "A continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaginatedComputeResourcesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PaginatedComputeResourcesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Mutable batch inference settings per deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartialBatchDeployment {
    #[doc = "Description of the endpoint deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl PartialBatchDeployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Strictly used in update requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartialBatchDeploymentPartialMinimalTrackedResourceWithProperties {
    #[doc = "Mutable batch inference settings per deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PartialBatchDeployment>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PartialBatchDeploymentPartialMinimalTrackedResourceWithProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Mutable base definition for a job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartialJobBase {
    #[doc = "Mutable configuration for notification."]
    #[serde(rename = "notificationSetting", default, skip_serializing_if = "Option::is_none")]
    pub notification_setting: Option<PartialNotificationSetting>,
}
impl PartialJobBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Resource Manager resource envelope strictly used in update requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartialJobBasePartialResource {
    #[doc = "Mutable base definition for a job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PartialJobBase>,
}
impl PartialJobBasePartialResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartialManagedServiceIdentity {
    #[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ManagedServiceIdentityType>,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl PartialManagedServiceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Strictly used in update requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartialMinimalTrackedResource {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PartialMinimalTrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Strictly used in update requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartialMinimalTrackedResourceWithIdentity {
    #[serde(flatten)]
    pub partial_minimal_tracked_resource: PartialMinimalTrackedResource,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<PartialManagedServiceIdentity>,
}
impl PartialMinimalTrackedResourceWithIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Strictly used in update requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartialMinimalTrackedResourceWithSku {
    #[serde(flatten)]
    pub partial_minimal_tracked_resource: PartialMinimalTrackedResource,
    #[doc = "Common SKU definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PartialSku>,
}
impl PartialMinimalTrackedResourceWithSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Mutable configuration for notification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartialNotificationSetting {
    #[doc = "Send webhook callback to a service. Key is a user-provided name for the webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<serde_json::Value>,
}
impl PartialNotificationSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Partial Registry class for PATCH"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartialRegistry {}
impl PartialRegistry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Strictly used in update requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartialRegistryPartialTrackedResource {
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<RegistryPartialManagedServiceIdentity>,
    #[doc = "Common SKU definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PartialSku>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PartialRegistryPartialTrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common SKU definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartialSku {
    #[doc = "If the SKU supports scale out/in then the capacity integer should be included. If scale out/in is not possible for the resource this may be omitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[doc = "If the service has different generations of hardware, for the same SKU, then that can be captured here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The name of the SKU. Ex - P3. It is typically a letter+number code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The SKU size. When the name field is the combination of tier and some other value, this would be the standalone code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<SkuTier>,
}
impl PartialSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartialUserAssignedIdentity {}
impl PartialUserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Password {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Password {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PendingUploadCredentialDto {
    #[doc = "Enum to determine the PendingUpload credentials type."]
    #[serde(rename = "credentialType")]
    pub credential_type: PendingUploadCredentialType,
}
impl PendingUploadCredentialDto {
    pub fn new(credential_type: PendingUploadCredentialType) -> Self {
        Self { credential_type }
    }
}
#[doc = "Enum to determine the PendingUpload credentials type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PendingUploadCredentialType")]
pub enum PendingUploadCredentialType {
    #[serde(rename = "SAS")]
    Sas,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PendingUploadCredentialType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PendingUploadCredentialType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PendingUploadCredentialType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Sas => serializer.serialize_unit_variant("PendingUploadCredentialType", 0u32, "SAS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PendingUploadRequestDto {
    #[doc = "If PendingUploadId = null then random guid will be used."]
    #[serde(rename = "pendingUploadId", default, skip_serializing_if = "Option::is_none")]
    pub pending_upload_id: Option<String>,
    #[doc = "Type of storage to use for the pending upload location"]
    #[serde(rename = "pendingUploadType", default, skip_serializing_if = "Option::is_none")]
    pub pending_upload_type: Option<PendingUploadType>,
}
impl PendingUploadRequestDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PendingUploadResponseDto {
    #[serde(rename = "blobReferenceForConsumption", default, skip_serializing_if = "Option::is_none")]
    pub blob_reference_for_consumption: Option<BlobReferenceForConsumptionDto>,
    #[doc = "ID for this upload request"]
    #[serde(rename = "pendingUploadId", default, skip_serializing_if = "Option::is_none")]
    pub pending_upload_id: Option<String>,
    #[doc = "Type of storage to use for the pending upload location"]
    #[serde(rename = "pendingUploadType", default, skip_serializing_if = "Option::is_none")]
    pub pending_upload_type: Option<PendingUploadType>,
}
impl PendingUploadResponseDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of storage to use for the pending upload location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PendingUploadType")]
pub enum PendingUploadType {
    None,
    TemporaryBlobReference,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PendingUploadType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PendingUploadType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PendingUploadType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("PendingUploadType", 0u32, "None"),
            Self::TemporaryBlobReference => serializer.serialize_unit_variant("PendingUploadType", 1u32, "TemporaryBlobReference"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Settings for a personal compute instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PersonalComputeInstanceSettings {
    #[doc = "A user that can be assigned to a compute instance."]
    #[serde(rename = "assignedUser", default, skip_serializing_if = "Option::is_none")]
    pub assigned_user: Option<AssignedUser>,
}
impl PersonalComputeInstanceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pipeline Job definition: defines generic to MFE attributes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineJob {
    #[serde(flatten)]
    pub job_base: JobBase,
    #[doc = "Inputs for the pipeline job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[doc = "Jobs construct the Pipeline Job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jobs: Option<serde_json::Value>,
    #[doc = "Outputs for the pipeline job"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[doc = "Pipeline settings, for things like ContinueRunOnStepFailure etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[doc = "ARM resource ID of source job."]
    #[serde(rename = "sourceJobId", default, skip_serializing_if = "Option::is_none")]
    pub source_job_id: Option<String>,
}
impl PipelineJob {
    pub fn new(job_base: JobBase) -> Self {
        Self {
            job_base,
            inputs: None,
            jobs: None,
            outputs: None,
            settings: None,
            source_job_id: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PredictionDriftMetricThresholdBase {
    #[serde(rename = "dataType")]
    pub data_type: MonitoringFeatureDataType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<MonitoringThreshold>,
}
impl PredictionDriftMetricThresholdBase {
    pub fn new(data_type: MonitoringFeatureDataType) -> Self {
        Self {
            data_type,
            threshold: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PredictionDriftMonitoringSignal {
    #[serde(flatten)]
    pub monitoring_signal_base: MonitoringSignalBase,
    #[serde(rename = "baselineData")]
    pub baseline_data: MonitoringInputData,
    #[doc = "[Required] A list of metrics to calculate and their associated thresholds."]
    #[serde(rename = "metricThresholds")]
    pub metric_thresholds: Vec<PredictionDriftMetricThresholdBase>,
    #[serde(rename = "modelType")]
    pub model_type: MonitoringModelType,
    #[serde(rename = "targetData")]
    pub target_data: MonitoringInputData,
}
impl PredictionDriftMonitoringSignal {
    pub fn new(
        monitoring_signal_base: MonitoringSignalBase,
        baseline_data: MonitoringInputData,
        metric_thresholds: Vec<PredictionDriftMetricThresholdBase>,
        model_type: MonitoringModelType,
        target_data: MonitoringInputData,
    ) -> Self {
        Self {
            monitoring_signal_base,
            baseline_data,
            metric_thresholds,
            model_type,
            target_data,
        }
    }
}
#[doc = "The Private Endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for Private Endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint Connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the PrivateEndpointConnectProperties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Specifies the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Contains resource tags defined as key/value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of private endpoint connection associated with the specified workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "Array of private endpoint connections"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateEndpointConnection>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the PrivateEndpointConnectProperties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The Private Endpoint resource."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "A collection of information about the state of the connection between service consumer and provider."]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            private_endpoint: None,
            private_link_service_connection_state,
            provisioning_state: None,
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointConnectionProvisioningState")]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointConnectionProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointConnectionProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointConnectionProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 0u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 1u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 2u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Private Endpoint destination for a Private Endpoint Outbound Rule for the managed network of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointDestination {
    #[serde(rename = "serviceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub service_resource_id: Option<String>,
    #[serde(rename = "subresourceTarget", default, skip_serializing_if = "Option::is_none")]
    pub subresource_target: Option<String>,
    #[serde(rename = "sparkEnabled", default, skip_serializing_if = "Option::is_none")]
    pub spark_enabled: Option<bool>,
    #[doc = "Status of a managed network Outbound Rule of a machine learning workspace."]
    #[serde(rename = "sparkStatus", default, skip_serializing_if = "Option::is_none")]
    pub spark_status: Option<RuleStatus>,
}
impl PrivateEndpointDestination {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private Endpoint Outbound Rule for the managed network of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointOutboundRule {
    #[serde(flatten)]
    pub outbound_rule: OutboundRule,
    #[doc = "Private Endpoint destination for a Private Endpoint Outbound Rule for the managed network of a machine learning workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<PrivateEndpointDestination>,
}
impl PrivateEndpointOutboundRule {
    pub fn new(outbound_rule: OutboundRule) -> Self {
        Self {
            outbound_rule,
            destination: None,
        }
    }
}
#[doc = "The PE network resource that is linked to this PE connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointResource {
    #[serde(flatten)]
    pub private_endpoint: PrivateEndpoint,
    #[doc = "The subnetId that the private endpoint is connected to."]
    #[serde(rename = "subnetArmId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_arm_id: Option<String>,
}
impl PrivateEndpointResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The private endpoint connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointServiceConnectionStatus")]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    Disconnected,
    Timeout,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointServiceConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointServiceConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointServiceConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 2u32, "Rejected"),
            Self::Disconnected => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 3u32, "Disconnected"),
            Self::Timeout => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 4u32, "Timeout"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Specifies the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Contains resource tags defined as key/value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "Array of private link resources"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateLinkResource>,
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(
        rename = "requiredMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_members: Vec<String>,
    #[doc = "The private link resource Private link DNS zone name."]
    #[serde(
        rename = "requiredZoneNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of information about the state of the connection between service consumer and provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[doc = "The reason for approval/rejection of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment container liveness/readiness probe configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProbeSettings {
    #[doc = "The number of failures to allow before returning an unhealthy status."]
    #[serde(rename = "failureThreshold", default, skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    #[doc = "The delay before the first probe in ISO 8601 format."]
    #[serde(rename = "initialDelay", default, skip_serializing_if = "Option::is_none")]
    pub initial_delay: Option<String>,
    #[doc = "The length of time between probes in ISO 8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[doc = "The number of successful probes before returning a healthy status."]
    #[serde(rename = "successThreshold", default, skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,
    #[doc = "The probe timeout in ISO 8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}
impl ProbeSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Progress metrics definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProgressMetrics {
    #[doc = "The completed datapoint count."]
    #[serde(rename = "completedDatapointCount", default, skip_serializing_if = "Option::is_none")]
    pub completed_datapoint_count: Option<i64>,
    #[doc = "The time of last successful incremental data refresh in UTC."]
    #[serde(rename = "incrementalDataLastRefreshDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub incremental_data_last_refresh_date_time: Option<time::OffsetDateTime>,
    #[doc = "The skipped datapoint count."]
    #[serde(rename = "skippedDatapointCount", default, skip_serializing_if = "Option::is_none")]
    pub skipped_datapoint_count: Option<i64>,
    #[doc = "The total datapoint count."]
    #[serde(rename = "totalDatapointCount", default, skip_serializing_if = "Option::is_none")]
    pub total_datapoint_count: Option<i64>,
}
impl ProgressMetrics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Protection level associated with the Intellectual Property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProtectionLevel")]
pub enum ProtectionLevel {
    All,
    None,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProtectionLevel {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProtectionLevel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProtectionLevel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::All => serializer.serialize_unit_variant("ProtectionLevel", 0u32, "All"),
            Self::None => serializer.serialize_unit_variant("ProtectionLevel", 1u32, "None"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enum to determine whether PublicNetworkAccess is Enabled or Disabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PublicNetworkAccessType")]
pub enum PublicNetworkAccessType {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PublicNetworkAccessType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PublicNetworkAccessType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PublicNetworkAccessType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccessType", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccessType", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "PyTorch distribution configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PyTorch {
    #[serde(flatten)]
    pub distribution_configuration: DistributionConfiguration,
    #[doc = "Number of processes per node."]
    #[serde(rename = "processCountPerInstance", default, skip_serializing_if = "Option::is_none")]
    pub process_count_per_instance: Option<i32>,
}
impl PyTorch {
    pub fn new(distribution_configuration: DistributionConfiguration) -> Self {
        Self {
            distribution_configuration,
            process_count_per_instance: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueueSettings {
    #[doc = "Enum to determine the job tier."]
    #[serde(rename = "jobTier", default, skip_serializing_if = "Option::is_none")]
    pub job_tier: Option<JobTier>,
    #[doc = "Controls the priority of the job on a compute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}
impl QueueSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties for Quota update or retrieval."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaBaseProperties {
    #[doc = "Specifies the resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Specifies the resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The maximum permitted quota of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "An enum describing the unit of quota measurement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<quota_base_properties::Unit>,
}
impl QuotaBaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod quota_base_properties {
    use super::*;
    #[doc = "An enum describing the unit of quota measurement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Count,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Count => serializer.serialize_unit_variant("Unit", 0u32, "Count"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Quota update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaUpdateParameters {
    #[doc = "The list for update quota."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<QuotaBaseProperties>,
    #[doc = "Region of workspace quota to be updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl QuotaUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a Sampling Algorithm that generates values randomly"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RandomSamplingAlgorithm {
    #[serde(flatten)]
    pub sampling_algorithm: SamplingAlgorithm,
    #[doc = "An optional positive number or e in string format to be used as base for log based random sampling"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logbase: Option<String>,
    #[doc = "The specific type of random algorithm"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<RandomSamplingAlgorithmRule>,
    #[doc = "An optional integer to use as the seed for random number generation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,
}
impl RandomSamplingAlgorithm {
    pub fn new(sampling_algorithm: SamplingAlgorithm) -> Self {
        Self {
            sampling_algorithm,
            logbase: None,
            rule: None,
            seed: None,
        }
    }
}
#[doc = "The specific type of random algorithm"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RandomSamplingAlgorithmRule")]
pub enum RandomSamplingAlgorithmRule {
    Random,
    Sobol,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RandomSamplingAlgorithmRule {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RandomSamplingAlgorithmRule {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RandomSamplingAlgorithmRule {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Random => serializer.serialize_unit_variant("RandomSamplingAlgorithmRule", 0u32, "Random"),
            Self::Sobol => serializer.serialize_unit_variant("RandomSamplingAlgorithmRule", 1u32, "Sobol"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Ray distribution configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ray {
    #[serde(flatten)]
    pub distribution_configuration: DistributionConfiguration,
    #[doc = "The address of Ray head node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "The port to bind the dashboard server to."]
    #[serde(rename = "dashboardPort", default, skip_serializing_if = "Option::is_none")]
    pub dashboard_port: Option<i32>,
    #[doc = "Additional arguments passed to ray start in head node."]
    #[serde(rename = "headNodeAdditionalArgs", default, skip_serializing_if = "Option::is_none")]
    pub head_node_additional_args: Option<String>,
    #[doc = "Provide this argument to start the Ray dashboard GUI."]
    #[serde(rename = "includeDashboard", default, skip_serializing_if = "Option::is_none")]
    pub include_dashboard: Option<bool>,
    #[doc = "The port of the head ray process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "Additional arguments passed to ray start in worker node."]
    #[serde(rename = "workerNodeAdditionalArgs", default, skip_serializing_if = "Option::is_none")]
    pub worker_node_additional_args: Option<String>,
}
impl Ray {
    pub fn new(distribution_configuration: DistributionConfiguration) -> Self {
        Self {
            distribution_configuration,
            address: None,
            dashboard_port: None,
            head_node_additional_args: None,
            include_dashboard: None,
            port: None,
            worker_node_additional_args: None,
        }
    }
}
#[doc = "The workflow trigger recurrence for ComputeStartStop schedule type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Recurrence {
    #[doc = "Enum to describe the frequency of a recurrence schedule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<RecurrenceFrequency>,
    #[doc = "[Required] Specifies schedule interval in conjunction with frequency"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[doc = "The start time in yyyy-MM-ddTHH:mm:ss format."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Specifies time zone in which the schedule runs.\r\nTimeZone should follow Windows time zone format. Refer: https://docs.microsoft.com/en-us/windows-hardware/manufacture/desktop/default-time-zones?view=windows-11"]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<RecurrenceSchedule>,
}
impl Recurrence {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enum to describe the frequency of a recurrence schedule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RecurrenceFrequency")]
pub enum RecurrenceFrequency {
    Minute,
    Hour,
    Day,
    Week,
    Month,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RecurrenceFrequency {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RecurrenceFrequency {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RecurrenceFrequency {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Minute => serializer.serialize_unit_variant("RecurrenceFrequency", 0u32, "Minute"),
            Self::Hour => serializer.serialize_unit_variant("RecurrenceFrequency", 1u32, "Hour"),
            Self::Day => serializer.serialize_unit_variant("RecurrenceFrequency", 2u32, "Day"),
            Self::Week => serializer.serialize_unit_variant("RecurrenceFrequency", 3u32, "Week"),
            Self::Month => serializer.serialize_unit_variant("RecurrenceFrequency", 4u32, "Month"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurrenceSchedule {
    #[doc = "[Required] List of hours for the schedule."]
    pub hours: Vec<i32>,
    #[doc = "[Required] List of minutes for the schedule."]
    pub minutes: Vec<i32>,
    #[doc = "List of month days for the schedule"]
    #[serde(
        rename = "monthDays",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub month_days: Vec<i32>,
    #[doc = "List of days for the schedule."]
    #[serde(
        rename = "weekDays",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub week_days: Vec<WeekDay>,
}
impl RecurrenceSchedule {
    pub fn new(hours: Vec<i32>, minutes: Vec<i32>) -> Self {
        Self {
            hours,
            minutes,
            month_days: Vec::new(),
            week_days: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurrenceTrigger {
    #[serde(flatten)]
    pub trigger_base: TriggerBase,
    #[doc = "Enum to describe the frequency of a recurrence schedule"]
    pub frequency: RecurrenceFrequency,
    #[doc = "[Required] Specifies schedule interval in conjunction with frequency"]
    pub interval: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<RecurrenceSchedule>,
}
impl RecurrenceTrigger {
    pub fn new(trigger_base: TriggerBase, frequency: RecurrenceFrequency, interval: i32) -> Self {
        Self {
            trigger_base,
            frequency,
            interval,
            schedule: None,
        }
    }
}
#[doc = "Enum to determine which reference method to use for an asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReferenceType")]
pub enum ReferenceType {
    Id,
    DataPath,
    OutputPath,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReferenceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReferenceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReferenceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Id => serializer.serialize_unit_variant("ReferenceType", 0u32, "Id"),
            Self::DataPath => serializer.serialize_unit_variant("ReferenceType", 1u32, "DataPath"),
            Self::OutputPath => serializer.serialize_unit_variant("ReferenceType", 2u32, "OutputPath"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateEndpointKeysRequest {
    #[serde(rename = "keyType")]
    pub key_type: KeyType,
    #[doc = "The value the key is set to."]
    #[serde(rename = "keyValue", default, skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
}
impl RegenerateEndpointKeysRequest {
    pub fn new(key_type: KeyType) -> Self {
        Self { key_type, key_value: None }
    }
}
#[doc = "Details of the Registry"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Registry {
    #[doc = "Discovery URL for the Registry"]
    #[serde(rename = "discoveryUrl", default, skip_serializing_if = "Option::is_none")]
    pub discovery_url: Option<String>,
    #[doc = "IntellectualPropertyPublisher for the registry"]
    #[serde(rename = "intellectualPropertyPublisher", default, skip_serializing_if = "Option::is_none")]
    pub intellectual_property_publisher: Option<String>,
    #[doc = "ARM ResourceId of a resource"]
    #[serde(rename = "managedResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group: Option<ArmResourceId>,
    #[doc = "MLFlow Registry URI for the Registry"]
    #[serde(rename = "mlFlowRegistryUri", default, skip_serializing_if = "Option::is_none")]
    pub ml_flow_registry_uri: Option<String>,
    #[doc = "Private endpoint connections info used for pending connections in private link portal"]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<RegistryPrivateEndpointConnection>,
    #[doc = "Is the Registry accessible from the internet?\r\nPossible values: \"Enabled\" or \"Disabled\""]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<String>,
    #[doc = "Details of each region the registry is in"]
    #[serde(
        rename = "regionDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub region_details: Vec<RegistryRegionArmDetails>,
}
impl Registry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryListCredentialsResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub passwords: Vec<Password>,
}
impl RegistryListCredentialsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryPartialManagedServiceIdentity {
    #[serde(flatten)]
    pub managed_service_identity: ManagedServiceIdentity,
}
impl RegistryPartialManagedServiceIdentity {
    pub fn new(managed_service_identity: ManagedServiceIdentity) -> Self {
        Self { managed_service_identity }
    }
}
#[doc = "Private endpoint connection definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryPrivateEndpointConnection {
    #[doc = "This is the private endpoint connection name created on SRP\r\nFull resource id: /subscriptions/{subId}/resourceGroups/{rgName}/providers/Microsoft.MachineLearningServices/{resourceType}/{resourceName}/privateEndpointConnections/{peConnectionName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Same as workspace location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties of the Private Endpoint Connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistryPrivateEndpointConnectionProperties>,
}
impl RegistryPrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the Private Endpoint Connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryPrivateEndpointConnectionProperties {
    #[doc = "The group ids"]
    #[serde(
        rename = "groupIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_ids: Vec<String>,
    #[doc = "The PE network resource that is linked to this PE connection."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpointResource>,
    #[doc = "The connection state."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<RegistryPrivateLinkServiceConnectionState>,
    #[doc = "One of null, \"Succeeded\", \"Provisioning\", \"Failed\". While not approved, it's null."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl RegistryPrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The connection state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryPrivateLinkServiceConnectionState {
    #[doc = "Some RP chose \"None\". Other RPs use this for region expansion."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
    #[doc = "User-defined message that, per NRP doc, may be used for approval-related message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Connection status of the service consumer with the service provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EndpointServiceConnectionStatus>,
}
impl RegistryPrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details for each region the registry is in"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryRegionArmDetails {
    #[doc = "List of ACR accounts"]
    #[serde(
        rename = "acrDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub acr_details: Vec<AcrDetails>,
    #[doc = "The location where the registry exists"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "List of storage accounts"]
    #[serde(
        rename = "storageAccountDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub storage_account_details: Vec<StorageAccountDetails>,
}
impl RegistryRegionArmDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryTrackedResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Metadata used by portal/tooling/etc to render different UX experiences for resources of the same type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Details of the Registry"]
    pub properties: Registry,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl RegistryTrackedResource {
    pub fn new(tracked_resource: TrackedResource, properties: Registry) -> Self {
        Self {
            tracked_resource,
            identity: None,
            kind: None,
            properties,
            sku: None,
        }
    }
}
#[doc = "A paginated list of Registry entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryTrackedResourceArmPaginatedResult {
    #[doc = "The link to the next page of Registry objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type Registry."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RegistryTrackedResource>,
}
impl azure_core::Continuable for RegistryTrackedResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RegistryTrackedResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Regression task in AutoML Table vertical."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Regression {
    #[serde(flatten)]
    pub table_vertical: TableVertical,
    #[serde(flatten)]
    pub auto_ml_vertical: AutoMlVertical,
    #[doc = "Primary metrics for Regression task."]
    #[serde(rename = "primaryMetric", default, skip_serializing_if = "Option::is_none")]
    pub primary_metric: Option<RegressionPrimaryMetrics>,
    #[doc = "Regression Training related configuration."]
    #[serde(rename = "trainingSettings", default, skip_serializing_if = "Option::is_none")]
    pub training_settings: Option<RegressionTrainingSettings>,
}
impl Regression {
    pub fn new(auto_ml_vertical: AutoMlVertical) -> Self {
        Self {
            table_vertical: TableVertical::default(),
            auto_ml_vertical,
            primary_metric: None,
            training_settings: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RegressionModelPerformanceMetric")]
pub enum RegressionModelPerformanceMetric {
    MeanAbsoluteError,
    RootMeanSquaredError,
    MeanSquaredError,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RegressionModelPerformanceMetric {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RegressionModelPerformanceMetric {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RegressionModelPerformanceMetric {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::MeanAbsoluteError => serializer.serialize_unit_variant("RegressionModelPerformanceMetric", 0u32, "MeanAbsoluteError"),
            Self::RootMeanSquaredError => {
                serializer.serialize_unit_variant("RegressionModelPerformanceMetric", 1u32, "RootMeanSquaredError")
            }
            Self::MeanSquaredError => serializer.serialize_unit_variant("RegressionModelPerformanceMetric", 2u32, "MeanSquaredError"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegressionModelPerformanceMetricThreshold {
    #[serde(flatten)]
    pub model_performance_metric_threshold_base: ModelPerformanceMetricThresholdBase,
    pub metric: RegressionModelPerformanceMetric,
}
impl RegressionModelPerformanceMetricThreshold {
    pub fn new(
        model_performance_metric_threshold_base: ModelPerformanceMetricThresholdBase,
        metric: RegressionModelPerformanceMetric,
    ) -> Self {
        Self {
            model_performance_metric_threshold_base,
            metric,
        }
    }
}
#[doc = "Enum for all Regression models supported by AutoML."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RegressionModels")]
pub enum RegressionModels {
    ElasticNet,
    GradientBoosting,
    DecisionTree,
    #[serde(rename = "KNN")]
    Knn,
    LassoLars,
    #[serde(rename = "SGD")]
    Sgd,
    RandomForest,
    ExtremeRandomTrees,
    #[serde(rename = "LightGBM")]
    LightGbm,
    #[serde(rename = "XGBoostRegressor")]
    XgBoostRegressor,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RegressionModels {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RegressionModels {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RegressionModels {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ElasticNet => serializer.serialize_unit_variant("RegressionModels", 0u32, "ElasticNet"),
            Self::GradientBoosting => serializer.serialize_unit_variant("RegressionModels", 1u32, "GradientBoosting"),
            Self::DecisionTree => serializer.serialize_unit_variant("RegressionModels", 2u32, "DecisionTree"),
            Self::Knn => serializer.serialize_unit_variant("RegressionModels", 3u32, "KNN"),
            Self::LassoLars => serializer.serialize_unit_variant("RegressionModels", 4u32, "LassoLars"),
            Self::Sgd => serializer.serialize_unit_variant("RegressionModels", 5u32, "SGD"),
            Self::RandomForest => serializer.serialize_unit_variant("RegressionModels", 6u32, "RandomForest"),
            Self::ExtremeRandomTrees => serializer.serialize_unit_variant("RegressionModels", 7u32, "ExtremeRandomTrees"),
            Self::LightGbm => serializer.serialize_unit_variant("RegressionModels", 8u32, "LightGBM"),
            Self::XgBoostRegressor => serializer.serialize_unit_variant("RegressionModels", 9u32, "XGBoostRegressor"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Primary metrics for Regression task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RegressionPrimaryMetrics")]
pub enum RegressionPrimaryMetrics {
    SpearmanCorrelation,
    NormalizedRootMeanSquaredError,
    R2Score,
    NormalizedMeanAbsoluteError,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RegressionPrimaryMetrics {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RegressionPrimaryMetrics {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RegressionPrimaryMetrics {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SpearmanCorrelation => serializer.serialize_unit_variant("RegressionPrimaryMetrics", 0u32, "SpearmanCorrelation"),
            Self::NormalizedRootMeanSquaredError => {
                serializer.serialize_unit_variant("RegressionPrimaryMetrics", 1u32, "NormalizedRootMeanSquaredError")
            }
            Self::R2Score => serializer.serialize_unit_variant("RegressionPrimaryMetrics", 2u32, "R2Score"),
            Self::NormalizedMeanAbsoluteError => {
                serializer.serialize_unit_variant("RegressionPrimaryMetrics", 3u32, "NormalizedMeanAbsoluteError")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Regression Training related configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegressionTrainingSettings {
    #[serde(flatten)]
    pub training_settings: TrainingSettings,
    #[doc = "Allowed models for regression task."]
    #[serde(
        rename = "allowedTrainingAlgorithms",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_training_algorithms: Vec<RegressionModels>,
    #[doc = "Blocked models for regression task."]
    #[serde(
        rename = "blockedTrainingAlgorithms",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_training_algorithms: Vec<RegressionModels>,
}
impl RegressionTrainingSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestLogging {
    #[doc = "For payload logging, we only collect payload by default. If customers also want to collect the specified headers, they can set them in captureHeaders so that backend will collect those headers along with payload."]
    #[serde(
        rename = "captureHeaders",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub capture_headers: Vec<String>,
}
impl RequestLogging {
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceBase {
    #[doc = "The asset description text."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The asset property dictionary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Tag dictionary. Tags can be added, removed, and updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceConfiguration {
    #[doc = "Optional number of instances or nodes used by the compute target."]
    #[serde(rename = "instanceCount", default, skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
    #[doc = "Optional type of VM used as supported by the compute target."]
    #[serde(rename = "instanceType", default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[doc = "Locations where the job can run."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub locations: Vec<String>,
    #[doc = "Optional max allowed number of instances or nodes to be used by the compute target.\r\nFor use with elastic training, currently supported by PyTorch distribution type only."]
    #[serde(rename = "maxInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub max_instance_count: Option<i32>,
    #[doc = "Additional properties bag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl ResourceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a resource ID. For example, for a subnet, it is the resource URL for the subnet."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceId {
    #[doc = "The ID of the resource"]
    pub id: String,
}
impl ResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "The Resource Name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceName {
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The localized name of the resource."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl ResourceName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The quota assigned to a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceQuota {
    #[doc = "Specifies the resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Region of the AML workspace in the id."]
    #[serde(rename = "amlWorkspaceLocation", default, skip_serializing_if = "Option::is_none")]
    pub aml_workspace_location: Option<String>,
    #[doc = "Specifies the resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The Resource Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc = "The maximum permitted quota of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "An enum describing the unit of quota measurement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<resource_quota::Unit>,
}
impl ResourceQuota {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_quota {
    use super::*;
    #[doc = "An enum describing the unit of quota measurement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Count,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Count => serializer.serialize_unit_variant("Unit", 0u32, "Count"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RollingRateType")]
pub enum RollingRateType {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RollingRateType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RollingRateType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RollingRateType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Year => serializer.serialize_unit_variant("RollingRateType", 0u32, "Year"),
            Self::Month => serializer.serialize_unit_variant("RollingRateType", 1u32, "Month"),
            Self::Day => serializer.serialize_unit_variant("RollingRateType", 2u32, "Day"),
            Self::Hour => serializer.serialize_unit_variant("RollingRateType", 3u32, "Hour"),
            Self::Minute => serializer.serialize_unit_variant("RollingRateType", 4u32, "Minute"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Route {
    #[doc = "[Required] The path for the route."]
    pub path: String,
    #[doc = "[Required] The port for the route."]
    pub port: i32,
}
impl Route {
    pub fn new(path: String, port: i32) -> Self {
        Self { path, port }
    }
}
#[doc = "Category of a managed network Outbound Rule of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RuleCategory")]
pub enum RuleCategory {
    Required,
    Recommended,
    UserDefined,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RuleCategory {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RuleCategory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RuleCategory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Required => serializer.serialize_unit_variant("RuleCategory", 0u32, "Required"),
            Self::Recommended => serializer.serialize_unit_variant("RuleCategory", 1u32, "Recommended"),
            Self::UserDefined => serializer.serialize_unit_variant("RuleCategory", 2u32, "UserDefined"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Status of a managed network Outbound Rule of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RuleStatus")]
pub enum RuleStatus {
    Inactive,
    Active,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RuleStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RuleStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RuleStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Inactive => serializer.serialize_unit_variant("RuleStatus", 0u32, "Inactive"),
            Self::Active => serializer.serialize_unit_variant("RuleStatus", 1u32, "Active"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Type of a managed network Outbound Rule of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RuleType")]
pub enum RuleType {
    #[serde(rename = "FQDN")]
    Fqdn,
    PrivateEndpoint,
    ServiceTag,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RuleType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RuleType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RuleType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Fqdn => serializer.serialize_unit_variant("RuleType", 0u32, "FQDN"),
            Self::PrivateEndpoint => serializer.serialize_unit_variant("RuleType", 1u32, "PrivateEndpoint"),
            Self::ServiceTag => serializer.serialize_unit_variant("RuleType", 2u32, "ServiceTag"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SasAuthTypeWorkspaceConnectionProperties {
    #[serde(flatten)]
    pub workspace_connection_properties_v2: WorkspaceConnectionPropertiesV2,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<WorkspaceConnectionSharedAccessSignature>,
}
impl SasAuthTypeWorkspaceConnectionProperties {
    pub fn new(workspace_connection_properties_v2: WorkspaceConnectionPropertiesV2) -> Self {
        Self {
            workspace_connection_properties_v2,
            credentials: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SasCredentialDto {
    #[serde(flatten)]
    pub pending_upload_credential_dto: PendingUploadCredentialDto,
    #[doc = "Full SAS Uri, including the storage, container/blob path and SAS token"]
    #[serde(rename = "sasUri", default, skip_serializing_if = "Option::is_none")]
    pub sas_uri: Option<String>,
}
impl SasCredentialDto {
    pub fn new(pending_upload_credential_dto: PendingUploadCredentialDto) -> Self {
        Self {
            pending_upload_credential_dto,
            sas_uri: None,
        }
    }
}
#[doc = "The Sampling Algorithm used to generate hyperparameter values, along with properties to\r\nconfigure the algorithm"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SamplingAlgorithm {
    #[serde(rename = "samplingAlgorithmType")]
    pub sampling_algorithm_type: SamplingAlgorithmType,
}
impl SamplingAlgorithm {
    pub fn new(sampling_algorithm_type: SamplingAlgorithmType) -> Self {
        Self { sampling_algorithm_type }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SamplingAlgorithmType")]
pub enum SamplingAlgorithmType {
    Grid,
    Random,
    Bayesian,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SamplingAlgorithmType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SamplingAlgorithmType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SamplingAlgorithmType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Grid => serializer.serialize_unit_variant("SamplingAlgorithmType", 0u32, "Grid"),
            Self::Random => serializer.serialize_unit_variant("SamplingAlgorithmType", 1u32, "Random"),
            Self::Bayesian => serializer.serialize_unit_variant("SamplingAlgorithmType", 2u32, "Bayesian"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SAS datastore credentials configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SasDatastoreCredentials {
    #[serde(flatten)]
    pub datastore_credentials: DatastoreCredentials,
    #[doc = "Datastore SAS secrets."]
    pub secrets: SasDatastoreSecrets,
}
impl SasDatastoreCredentials {
    pub fn new(datastore_credentials: DatastoreCredentials, secrets: SasDatastoreSecrets) -> Self {
        Self {
            datastore_credentials,
            secrets,
        }
    }
}
#[doc = "Datastore SAS secrets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SasDatastoreSecrets {
    #[serde(flatten)]
    pub datastore_secrets: DatastoreSecrets,
    #[doc = "Storage container SAS token."]
    #[serde(rename = "sasToken", default, skip_serializing_if = "Option::is_none")]
    pub sas_token: Option<String>,
}
impl SasDatastoreSecrets {
    pub fn new(datastore_secrets: DatastoreSecrets) -> Self {
        Self {
            datastore_secrets,
            sas_token: None,
        }
    }
}
#[doc = "scale settings for AML Compute"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleSettings {
    #[doc = "Max number of nodes to use"]
    #[serde(rename = "maxNodeCount")]
    pub max_node_count: i32,
    #[doc = "Min number of nodes to use"]
    #[serde(rename = "minNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub min_node_count: Option<i32>,
    #[doc = "Node Idle Time before scaling down amlCompute. This string needs to be in the RFC Format."]
    #[serde(rename = "nodeIdleTimeBeforeScaleDown", default, skip_serializing_if = "Option::is_none")]
    pub node_idle_time_before_scale_down: Option<String>,
}
impl ScaleSettings {
    pub fn new(max_node_count: i32) -> Self {
        Self {
            max_node_count,
            min_node_count: None,
            node_idle_time_before_scale_down: None,
        }
    }
}
#[doc = "Desired scale settings for the amlCompute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScaleSettingsInformation {
    #[doc = "scale settings for AML Compute"]
    #[serde(rename = "scaleSettings", default, skip_serializing_if = "Option::is_none")]
    pub scale_settings: Option<ScaleSettings>,
}
impl ScaleSettingsInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScaleType")]
pub enum ScaleType {
    Default,
    TargetUtilization,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScaleType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScaleType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScaleType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Default => serializer.serialize_unit_variant("ScaleType", 0u32, "Default"),
            Self::TargetUtilization => serializer.serialize_unit_variant("ScaleType", 1u32, "TargetUtilization"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Base definition of a schedule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(flatten)]
    pub resource_base: ResourceBase,
    pub action: ScheduleActionBase,
    #[doc = "Display name of schedule."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Is the schedule enabled?"]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ScheduleProvisioningStatus>,
    pub trigger: TriggerBase,
}
impl Schedule {
    pub fn new(action: ScheduleActionBase, trigger: TriggerBase) -> Self {
        Self {
            resource_base: ResourceBase::default(),
            action,
            display_name: None,
            is_enabled: None,
            provisioning_state: None,
            trigger,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleActionBase {
    #[serde(rename = "actionType")]
    pub action_type: ScheduleActionType,
}
impl ScheduleActionBase {
    pub fn new(action_type: ScheduleActionType) -> Self {
        Self { action_type }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduleActionType")]
pub enum ScheduleActionType {
    CreateJob,
    InvokeBatchEndpoint,
    ImportData,
    CreateMonitor,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduleActionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduleActionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduleActionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CreateJob => serializer.serialize_unit_variant("ScheduleActionType", 0u32, "CreateJob"),
            Self::InvokeBatchEndpoint => serializer.serialize_unit_variant("ScheduleActionType", 1u32, "InvokeBatchEndpoint"),
            Self::ImportData => serializer.serialize_unit_variant("ScheduleActionType", 2u32, "ImportData"),
            Self::CreateMonitor => serializer.serialize_unit_variant("ScheduleActionType", 3u32, "CreateMonitor"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleBase {
    #[doc = "A system assigned id for the schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The current deployment state of schedule."]
    #[serde(rename = "provisioningStatus", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status: Option<ScheduleProvisioningState>,
    #[doc = "Is the schedule enabled or disabled?"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ScheduleStatus>,
}
impl ScheduleBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduleListViewType")]
pub enum ScheduleListViewType {
    EnabledOnly,
    DisabledOnly,
    All,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduleListViewType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduleListViewType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduleListViewType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EnabledOnly => serializer.serialize_unit_variant("ScheduleListViewType", 0u32, "EnabledOnly"),
            Self::DisabledOnly => serializer.serialize_unit_variant("ScheduleListViewType", 1u32, "DisabledOnly"),
            Self::All => serializer.serialize_unit_variant("ScheduleListViewType", 2u32, "All"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The current deployment state of schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduleProvisioningState")]
pub enum ScheduleProvisioningState {
    Completed,
    Provisioning,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduleProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduleProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduleProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Completed => serializer.serialize_unit_variant("ScheduleProvisioningState", 0u32, "Completed"),
            Self::Provisioning => serializer.serialize_unit_variant("ScheduleProvisioningState", 1u32, "Provisioning"),
            Self::Failed => serializer.serialize_unit_variant("ScheduleProvisioningState", 2u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduleProvisioningStatus")]
pub enum ScheduleProvisioningStatus {
    Creating,
    Updating,
    Deleting,
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduleProvisioningStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduleProvisioningStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduleProvisioningStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Creating => serializer.serialize_unit_variant("ScheduleProvisioningStatus", 0u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ScheduleProvisioningStatus", 1u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ScheduleProvisioningStatus", 2u32, "Deleting"),
            Self::Succeeded => serializer.serialize_unit_variant("ScheduleProvisioningStatus", 3u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ScheduleProvisioningStatus", 4u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ScheduleProvisioningStatus", 5u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Base definition of a schedule"]
    pub properties: Schedule,
}
impl ScheduleResource {
    pub fn new(properties: Schedule) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A paginated list of Schedule entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleResourceArmPaginatedResult {
    #[doc = "The link to the next page of Schedule objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type Schedule."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ScheduleResource>,
}
impl azure_core::Continuable for ScheduleResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ScheduleResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Is the schedule enabled or disabled?"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduleStatus")]
pub enum ScheduleStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduleStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduleStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduleStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("ScheduleStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("ScheduleStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Script reference"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptReference {
    #[doc = "The storage source of the script: inline, workspace."]
    #[serde(rename = "scriptSource", default, skip_serializing_if = "Option::is_none")]
    pub script_source: Option<String>,
    #[doc = "The location of scripts in the mounted volume."]
    #[serde(rename = "scriptData", default, skip_serializing_if = "Option::is_none")]
    pub script_data: Option<String>,
    #[doc = "Optional command line arguments passed to the script to run."]
    #[serde(rename = "scriptArguments", default, skip_serializing_if = "Option::is_none")]
    pub script_arguments: Option<String>,
    #[doc = "Optional time period passed to timeout command."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}
impl ScriptReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Customized setup scripts"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptsToExecute {
    #[doc = "Script reference"]
    #[serde(rename = "startupScript", default, skip_serializing_if = "Option::is_none")]
    pub startup_script: Option<ScriptReference>,
    #[doc = "Script reference"]
    #[serde(rename = "creationScript", default, skip_serializing_if = "Option::is_none")]
    pub creation_script: Option<ScriptReference>,
}
impl ScriptsToExecute {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Forecasting seasonality."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Seasonality {
    #[doc = "Forecasting seasonality mode."]
    pub mode: SeasonalityMode,
}
impl Seasonality {
    pub fn new(mode: SeasonalityMode) -> Self {
        Self { mode }
    }
}
#[doc = "Forecasting seasonality mode."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SeasonalityMode")]
pub enum SeasonalityMode {
    Auto,
    Custom,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SeasonalityMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SeasonalityMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SeasonalityMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Auto => serializer.serialize_unit_variant("SeasonalityMode", 0u32, "Auto"),
            Self::Custom => serializer.serialize_unit_variant("SeasonalityMode", 1u32, "Custom"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Secret Configuration definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretConfiguration {
    #[doc = "Secret Uri.\r\nSample Uri : https://myvault.vault.azure.net/secrets/mysecretname/secretversion"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Name of secret in workspace key vault."]
    #[serde(rename = "workspaceSecretName", default, skip_serializing_if = "Option::is_none")]
    pub workspace_secret_name: Option<String>,
}
impl SecretConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enum to determine the datastore secrets type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SecretsType")]
pub enum SecretsType {
    AccountKey,
    Certificate,
    Sas,
    ServicePrincipal,
    KerberosPassword,
    KerberosKeytab,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SecretsType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SecretsType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SecretsType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AccountKey => serializer.serialize_unit_variant("SecretsType", 0u32, "AccountKey"),
            Self::Certificate => serializer.serialize_unit_variant("SecretsType", 1u32, "Certificate"),
            Self::Sas => serializer.serialize_unit_variant("SecretsType", 2u32, "Sas"),
            Self::ServicePrincipal => serializer.serialize_unit_variant("SecretsType", 3u32, "ServicePrincipal"),
            Self::KerberosPassword => serializer.serialize_unit_variant("SecretsType", 4u32, "KerberosPassword"),
            Self::KerberosKeytab => serializer.serialize_unit_variant("SecretsType", 5u32, "KerberosKeytab"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceDataAccessAuthIdentity")]
pub enum ServiceDataAccessAuthIdentity {
    None,
    WorkspaceSystemAssignedIdentity,
    WorkspaceUserAssignedIdentity,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceDataAccessAuthIdentity {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceDataAccessAuthIdentity {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceDataAccessAuthIdentity {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ServiceDataAccessAuthIdentity", 0u32, "None"),
            Self::WorkspaceSystemAssignedIdentity => {
                serializer.serialize_unit_variant("ServiceDataAccessAuthIdentity", 1u32, "WorkspaceSystemAssignedIdentity")
            }
            Self::WorkspaceUserAssignedIdentity => {
                serializer.serialize_unit_variant("ServiceDataAccessAuthIdentity", 2u32, "WorkspaceUserAssignedIdentity")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceManagedResourcesSettings {
    #[serde(rename = "cosmosDb", default, skip_serializing_if = "Option::is_none")]
    pub cosmos_db: Option<CosmosDbSettings>,
}
impl ServiceManagedResourcesSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalAuthTypeWorkspaceConnectionProperties {
    #[serde(flatten)]
    pub workspace_connection_properties_v2: WorkspaceConnectionPropertiesV2,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<WorkspaceConnectionServicePrincipal>,
}
impl ServicePrincipalAuthTypeWorkspaceConnectionProperties {
    pub fn new(workspace_connection_properties_v2: WorkspaceConnectionPropertiesV2) -> Self {
        Self {
            workspace_connection_properties_v2,
            credentials: None,
        }
    }
}
#[doc = "Service Principal datastore credentials configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalDatastoreCredentials {
    #[serde(flatten)]
    pub datastore_credentials: DatastoreCredentials,
    #[doc = "Authority URL used for authentication."]
    #[serde(rename = "authorityUrl", default, skip_serializing_if = "Option::is_none")]
    pub authority_url: Option<String>,
    #[doc = "[Required] Service principal client ID."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "Resource the service principal has access to."]
    #[serde(rename = "resourceUrl", default, skip_serializing_if = "Option::is_none")]
    pub resource_url: Option<String>,
    #[doc = "Datastore Service Principal secrets."]
    pub secrets: ServicePrincipalDatastoreSecrets,
    #[doc = "[Required] ID of the tenant to which the service principal belongs."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}
impl ServicePrincipalDatastoreCredentials {
    pub fn new(
        datastore_credentials: DatastoreCredentials,
        client_id: String,
        secrets: ServicePrincipalDatastoreSecrets,
        tenant_id: String,
    ) -> Self {
        Self {
            datastore_credentials,
            authority_url: None,
            client_id,
            resource_url: None,
            secrets,
            tenant_id,
        }
    }
}
#[doc = "Datastore Service Principal secrets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalDatastoreSecrets {
    #[serde(flatten)]
    pub datastore_secrets: DatastoreSecrets,
    #[doc = "Service principal secret."]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}
impl ServicePrincipalDatastoreSecrets {
    pub fn new(datastore_secrets: DatastoreSecrets) -> Self {
        Self {
            datastore_secrets,
            client_secret: None,
        }
    }
}
#[doc = "Service Tag destination for a Service Tag Outbound Rule for the managed network of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceTagDestination {
    #[serde(rename = "serviceTag", default, skip_serializing_if = "Option::is_none")]
    pub service_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "portRanges", default, skip_serializing_if = "Option::is_none")]
    pub port_ranges: Option<String>,
}
impl ServiceTagDestination {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service Tag Outbound Rule for the managed network of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceTagOutboundRule {
    #[serde(flatten)]
    pub outbound_rule: OutboundRule,
    #[doc = "Service Tag destination for a Service Tag Outbound Rule for the managed network of a machine learning workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<ServiceTagDestination>,
}
impl ServiceTagOutboundRule {
    pub fn new(outbound_rule: OutboundRule) -> Self {
        Self {
            outbound_rule,
            destination: None,
        }
    }
}
#[doc = "Details of customized scripts to execute for setting up the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SetupScripts {
    #[doc = "Customized setup scripts"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scripts: Option<ScriptsToExecute>,
}
impl SetupScripts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPrivateLinkResource {
    #[doc = "Unique name of the private link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties of a shared private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedPrivateLinkResourceProperty>,
}
impl SharedPrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a shared private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPrivateLinkResourceProperty {
    #[doc = "The resource id that private link links to."]
    #[serde(rename = "privateLinkResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_resource_id: Option<String>,
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Request message."]
    #[serde(rename = "requestMessage", default, skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
}
impl SharedPrivateLinkResourceProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameter defining how if AutoML should handle short time series."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ShortSeriesHandlingConfiguration")]
pub enum ShortSeriesHandlingConfiguration {
    None,
    Auto,
    Pad,
    Drop,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ShortSeriesHandlingConfiguration {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ShortSeriesHandlingConfiguration {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ShortSeriesHandlingConfiguration {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ShortSeriesHandlingConfiguration", 0u32, "None"),
            Self::Auto => serializer.serialize_unit_variant("ShortSeriesHandlingConfiguration", 1u32, "Auto"),
            Self::Pad => serializer.serialize_unit_variant("ShortSeriesHandlingConfiguration", 2u32, "Pad"),
            Self::Drop => serializer.serialize_unit_variant("ShortSeriesHandlingConfiguration", 3u32, "Drop"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition representing SKU"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the SKU. Ex - P3. It is typically a letter+number code"]
    pub name: String,
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<SkuTier>,
    #[doc = "The SKU size. When the name field is the combination of tier and some other value, this would be the standalone code. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "If the service has different generations of hardware, for the same SKU, then that can be captured here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "If the SKU supports scale out/in then the capacity integer should be included. If scale out/in is not possible for the resource this may be omitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
#[doc = "SKU capacity information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapacity {
    #[doc = "Gets or sets the default capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i32>,
    #[doc = "Gets or sets the maximum."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
    #[doc = "Gets or sets the minimum."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,
    #[doc = "Node scaling setting for the compute sku."]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<SkuScaleType>,
}
impl SkuCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Fulfills ARM Contract requirement to list all available SKUS for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuResource {
    #[doc = "SKU capacity information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<SkuCapacity>,
    #[doc = "The resource type name."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "SkuSetting fulfills the need for stripped down SKU info in ARM contract."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuSetting>,
}
impl SkuResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paginated list of SkuResource entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuResourceArmPaginatedResult {
    #[doc = "The link to the next page of SkuResource objects. If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "An array of objects of type SkuResource."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SkuResource>,
}
impl azure_core::Continuable for SkuResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SkuResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Node scaling setting for the compute sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SkuScaleType")]
pub enum SkuScaleType {
    Automatic,
    Manual,
    None,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SkuScaleType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SkuScaleType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SkuScaleType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Automatic => serializer.serialize_unit_variant("SkuScaleType", 0u32, "Automatic"),
            Self::Manual => serializer.serialize_unit_variant("SkuScaleType", 1u32, "Manual"),
            Self::None => serializer.serialize_unit_variant("SkuScaleType", 2u32, "None"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SkuSetting fulfills the need for stripped down SKU info in ARM contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuSetting {
    #[doc = "[Required] The name of the SKU. Ex - P3. It is typically a letter+number code."]
    pub name: String,
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<SkuTier>,
}
impl SkuSetting {
    pub fn new(name: String) -> Self {
        Self { name, tier: None }
    }
}
#[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SkuTier {
    Free,
    Basic,
    Standard,
    Premium,
}
#[doc = "Spark job definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkJob {
    #[serde(flatten)]
    pub job_base: JobBase,
    #[doc = "Archive files used in the job."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub archives: Vec<String>,
    #[doc = "Arguments for the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<String>,
    #[doc = "[Required] ARM resource ID of the code asset."]
    #[serde(rename = "codeId")]
    pub code_id: String,
    #[doc = "Spark configured properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conf: Option<serde_json::Value>,
    #[doc = "Spark job entry point definition."]
    pub entry: SparkJobEntry,
    #[doc = "The ARM resource ID of the Environment specification for the job."]
    #[serde(rename = "environmentId", default, skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[doc = "Files used in the job."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub files: Vec<String>,
    #[doc = "Mapping of input data bindings used in the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[doc = "Jar files used in the job."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub jars: Vec<String>,
    #[doc = "Mapping of output data bindings used in the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[doc = "Python files used in the job."]
    #[serde(
        rename = "pyFiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub py_files: Vec<String>,
    #[serde(rename = "queueSettings", default, skip_serializing_if = "Option::is_none")]
    pub queue_settings: Option<QueueSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<SparkResourceConfiguration>,
}
impl SparkJob {
    pub fn new(job_base: JobBase, code_id: String, entry: SparkJobEntry) -> Self {
        Self {
            job_base,
            archives: Vec::new(),
            args: None,
            code_id,
            conf: None,
            entry,
            environment_id: None,
            files: Vec::new(),
            inputs: None,
            jars: Vec::new(),
            outputs: None,
            py_files: Vec::new(),
            queue_settings: None,
            resources: None,
        }
    }
}
#[doc = "Spark job entry point definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkJobEntry {
    #[serde(rename = "sparkJobEntryType")]
    pub spark_job_entry_type: SparkJobEntryType,
}
impl SparkJobEntry {
    pub fn new(spark_job_entry_type: SparkJobEntryType) -> Self {
        Self { spark_job_entry_type }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SparkJobEntryType")]
pub enum SparkJobEntryType {
    SparkJobPythonEntry,
    SparkJobScalaEntry,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SparkJobEntryType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SparkJobEntryType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SparkJobEntryType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SparkJobPythonEntry => serializer.serialize_unit_variant("SparkJobEntryType", 0u32, "SparkJobPythonEntry"),
            Self::SparkJobScalaEntry => serializer.serialize_unit_variant("SparkJobEntryType", 1u32, "SparkJobScalaEntry"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkJobPythonEntry {
    #[serde(flatten)]
    pub spark_job_entry: SparkJobEntry,
    #[doc = "[Required] Relative python file path for job entry point."]
    pub file: String,
}
impl SparkJobPythonEntry {
    pub fn new(spark_job_entry: SparkJobEntry, file: String) -> Self {
        Self { spark_job_entry, file }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkJobScalaEntry {
    #[serde(flatten)]
    pub spark_job_entry: SparkJobEntry,
    #[doc = "[Required] Scala class name used as entry point."]
    #[serde(rename = "className")]
    pub class_name: String,
}
impl SparkJobScalaEntry {
    pub fn new(spark_job_entry: SparkJobEntry, class_name: String) -> Self {
        Self {
            spark_job_entry,
            class_name,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkResourceConfiguration {
    #[doc = "Optional type of VM used as supported by the compute target."]
    #[serde(rename = "instanceType", default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[doc = "Version of spark runtime used for the job."]
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
}
impl SparkResourceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ssl configuration for scoring"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SslConfiguration {
    #[doc = "Enable or disable ssl for scoring"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ssl_configuration::Status>,
    #[doc = "Cert data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    #[doc = "Key data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "CNAME of the cert"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    #[doc = "Leaf domain label of public endpoint"]
    #[serde(rename = "leafDomainLabel", default, skip_serializing_if = "Option::is_none")]
    pub leaf_domain_label: Option<String>,
    #[doc = "Indicates whether to overwrite existing domain label."]
    #[serde(rename = "overwriteExistingDomain", default, skip_serializing_if = "Option::is_none")]
    pub overwrite_existing_domain: Option<bool>,
}
impl SslConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ssl_configuration {
    use super::*;
    #[doc = "Enable or disable ssl for scoring"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Disabled,
        Enabled,
        Auto,
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
                Self::Disabled => serializer.serialize_unit_variant("Status", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("Status", 1u32, "Enabled"),
                Self::Auto => serializer.serialize_unit_variant("Status", 2u32, "Auto"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Advances setting to customize StackEnsemble run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StackEnsembleSettings {
    #[doc = "Optional parameters to pass to the initializer of the meta-learner."]
    #[serde(rename = "stackMetaLearnerKWargs", default, skip_serializing_if = "Option::is_none")]
    pub stack_meta_learner_k_wargs: Option<serde_json::Value>,
    #[doc = "Specifies the proportion of the training set (when choosing train and validation type of training) to be reserved for training the meta-learner. Default value is 0.2."]
    #[serde(rename = "stackMetaLearnerTrainPercentage", default, skip_serializing_if = "Option::is_none")]
    pub stack_meta_learner_train_percentage: Option<f64>,
    #[doc = "The meta-learner is a model trained on the output of the individual heterogeneous models.\r\nDefault meta-learners are LogisticRegression for classification tasks (or LogisticRegressionCV if cross-validation is enabled) and ElasticNet for regression/forecasting tasks (or ElasticNetCV if cross-validation is enabled).\r\nThis parameter can be one of the following strings: LogisticRegression, LogisticRegressionCV, LightGBMClassifier, ElasticNet, ElasticNetCV, LightGBMRegressor, or LinearRegression"]
    #[serde(rename = "stackMetaLearnerType", default, skip_serializing_if = "Option::is_none")]
    pub stack_meta_learner_type: Option<StackMetaLearnerType>,
}
impl StackEnsembleSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The meta-learner is a model trained on the output of the individual heterogeneous models.\r\nDefault meta-learners are LogisticRegression for classification tasks (or LogisticRegressionCV if cross-validation is enabled) and ElasticNet for regression/forecasting tasks (or ElasticNetCV if cross-validation is enabled).\r\nThis parameter can be one of the following strings: LogisticRegression, LogisticRegressionCV, LightGBMClassifier, ElasticNet, ElasticNetCV, LightGBMRegressor, or LinearRegression"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StackMetaLearnerType")]
pub enum StackMetaLearnerType {
    None,
    LogisticRegression,
    #[serde(rename = "LogisticRegressionCV")]
    LogisticRegressionCv,
    #[serde(rename = "LightGBMClassifier")]
    LightGbmClassifier,
    ElasticNet,
    #[serde(rename = "ElasticNetCV")]
    ElasticNetCv,
    #[serde(rename = "LightGBMRegressor")]
    LightGbmRegressor,
    LinearRegression,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StackMetaLearnerType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StackMetaLearnerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StackMetaLearnerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("StackMetaLearnerType", 0u32, "None"),
            Self::LogisticRegression => serializer.serialize_unit_variant("StackMetaLearnerType", 1u32, "LogisticRegression"),
            Self::LogisticRegressionCv => serializer.serialize_unit_variant("StackMetaLearnerType", 2u32, "LogisticRegressionCV"),
            Self::LightGbmClassifier => serializer.serialize_unit_variant("StackMetaLearnerType", 3u32, "LightGBMClassifier"),
            Self::ElasticNet => serializer.serialize_unit_variant("StackMetaLearnerType", 4u32, "ElasticNet"),
            Self::ElasticNetCv => serializer.serialize_unit_variant("StackMetaLearnerType", 5u32, "ElasticNetCV"),
            Self::LightGbmRegressor => serializer.serialize_unit_variant("StackMetaLearnerType", 6u32, "LightGBMRegressor"),
            Self::LinearRegression => serializer.serialize_unit_variant("StackMetaLearnerType", 7u32, "LinearRegression"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Active message associated with project"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusMessage {
    #[doc = "Service-defined message code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Time in UTC at which the message was created."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<StatusMessageLevel>,
    #[doc = "A human-readable representation of the message code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl StatusMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StatusMessageLevel")]
pub enum StatusMessageLevel {
    Error,
    Information,
    Warning,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StatusMessageLevel {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StatusMessageLevel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StatusMessageLevel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Error => serializer.serialize_unit_variant("StatusMessageLevel", 0u32, "Error"),
            Self::Information => serializer.serialize_unit_variant("StatusMessageLevel", 1u32, "Information"),
            Self::Warning => serializer.serialize_unit_variant("StatusMessageLevel", 2u32, "Warning"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Stochastic optimizer for image models."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StochasticOptimizer")]
pub enum StochasticOptimizer {
    None,
    Sgd,
    Adam,
    Adamw,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StochasticOptimizer {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StochasticOptimizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StochasticOptimizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("StochasticOptimizer", 0u32, "None"),
            Self::Sgd => serializer.serialize_unit_variant("StochasticOptimizer", 1u32, "Sgd"),
            Self::Adam => serializer.serialize_unit_variant("StochasticOptimizer", 2u32, "Adam"),
            Self::Adamw => serializer.serialize_unit_variant("StochasticOptimizer", 3u32, "Adamw"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Details of storage account to be used for the Registry"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountDetails {
    #[serde(rename = "systemCreatedStorageAccount", default, skip_serializing_if = "Option::is_none")]
    pub system_created_storage_account: Option<SystemCreatedStorageAccount>,
    #[serde(rename = "userCreatedStorageAccount", default, skip_serializing_if = "Option::is_none")]
    pub user_created_storage_account: Option<UserCreatedStorageAccount>,
}
impl StorageAccountDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sweep job definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SweepJob {
    #[serde(flatten)]
    pub job_base: JobBase,
    #[doc = "Early termination policies enable canceling poor-performing runs before they complete"]
    #[serde(rename = "earlyTermination", default, skip_serializing_if = "Option::is_none")]
    pub early_termination: Option<EarlyTerminationPolicy>,
    #[doc = "Mapping of input data bindings used in the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[doc = "Sweep Job limit class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<SweepJobLimits>,
    #[doc = "Optimization objective."]
    pub objective: Objective,
    #[doc = "Mapping of output data bindings used in the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[serde(rename = "queueSettings", default, skip_serializing_if = "Option::is_none")]
    pub queue_settings: Option<QueueSettings>,
    #[doc = "The Sampling Algorithm used to generate hyperparameter values, along with properties to\r\nconfigure the algorithm"]
    #[serde(rename = "samplingAlgorithm")]
    pub sampling_algorithm: SamplingAlgorithm,
    #[doc = "[Required] A dictionary containing each parameter and its distribution. The dictionary key is the name of the parameter"]
    #[serde(rename = "searchSpace")]
    pub search_space: serde_json::Value,
    #[doc = "Trial component definition."]
    pub trial: TrialComponent,
}
impl SweepJob {
    pub fn new(
        job_base: JobBase,
        objective: Objective,
        sampling_algorithm: SamplingAlgorithm,
        search_space: serde_json::Value,
        trial: TrialComponent,
    ) -> Self {
        Self {
            job_base,
            early_termination: None,
            inputs: None,
            limits: None,
            objective,
            outputs: None,
            queue_settings: None,
            sampling_algorithm,
            search_space,
            trial,
        }
    }
}
#[doc = "Sweep Job limit class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SweepJobLimits {
    #[serde(flatten)]
    pub job_limits: JobLimits,
    #[doc = "Sweep Job max concurrent trials."]
    #[serde(rename = "maxConcurrentTrials", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_trials: Option<i32>,
    #[doc = "Sweep Job max total trials."]
    #[serde(rename = "maxTotalTrials", default, skip_serializing_if = "Option::is_none")]
    pub max_total_trials: Option<i32>,
    #[doc = "Sweep Job Trial timeout value."]
    #[serde(rename = "trialTimeout", default, skip_serializing_if = "Option::is_none")]
    pub trial_timeout: Option<String>,
}
impl SweepJobLimits {
    pub fn new(job_limits: JobLimits) -> Self {
        Self {
            job_limits,
            max_concurrent_trials: None,
            max_total_trials: None,
            trial_timeout: None,
        }
    }
}
#[doc = "A SynapseSpark compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SynapseSpark {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<synapse_spark::Properties>,
}
impl SynapseSpark {
    pub fn new(compute: Compute) -> Self {
        Self { compute, properties: None }
    }
}
pub mod synapse_spark {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Auto scale properties"]
        #[serde(rename = "autoScaleProperties", default, skip_serializing_if = "Option::is_none")]
        pub auto_scale_properties: Option<AutoScaleProperties>,
        #[doc = "Auto pause properties"]
        #[serde(rename = "autoPauseProperties", default, skip_serializing_if = "Option::is_none")]
        pub auto_pause_properties: Option<AutoPauseProperties>,
        #[doc = "Spark version."]
        #[serde(rename = "sparkVersion", default, skip_serializing_if = "Option::is_none")]
        pub spark_version: Option<String>,
        #[doc = "The number of compute nodes currently assigned to the compute."]
        #[serde(rename = "nodeCount", default, skip_serializing_if = "Option::is_none")]
        pub node_count: Option<i32>,
        #[doc = "Node size."]
        #[serde(rename = "nodeSize", default, skip_serializing_if = "Option::is_none")]
        pub node_size: Option<String>,
        #[doc = "Node size family."]
        #[serde(rename = "nodeSizeFamily", default, skip_serializing_if = "Option::is_none")]
        pub node_size_family: Option<String>,
        #[doc = "Azure subscription identifier."]
        #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
        pub subscription_id: Option<String>,
        #[doc = "Name of the resource group in which workspace is located."]
        #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
        pub resource_group: Option<String>,
        #[doc = "Name of Azure Machine Learning workspace."]
        #[serde(rename = "workspaceName", default, skip_serializing_if = "Option::is_none")]
        pub workspace_name: Option<String>,
        #[doc = "Pool name."]
        #[serde(rename = "poolName", default, skip_serializing_if = "Option::is_none")]
        pub pool_name: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemCreatedAcrAccount {
    #[doc = "Name of the ACR account"]
    #[serde(rename = "acrAccountName", default, skip_serializing_if = "Option::is_none")]
    pub acr_account_name: Option<String>,
    #[doc = "SKU of the ACR account"]
    #[serde(rename = "acrAccountSku", default, skip_serializing_if = "Option::is_none")]
    pub acr_account_sku: Option<String>,
    #[doc = "ARM ResourceId of a resource"]
    #[serde(rename = "armResourceId", default, skip_serializing_if = "Option::is_none")]
    pub arm_resource_id: Option<ArmResourceId>,
}
impl SystemCreatedAcrAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemCreatedStorageAccount {
    #[doc = "Public blob access allowed"]
    #[serde(rename = "allowBlobPublicAccess", default, skip_serializing_if = "Option::is_none")]
    pub allow_blob_public_access: Option<bool>,
    #[doc = "ARM ResourceId of a resource"]
    #[serde(rename = "armResourceId", default, skip_serializing_if = "Option::is_none")]
    pub arm_resource_id: Option<ArmResourceId>,
    #[doc = "HNS enabled for storage account"]
    #[serde(rename = "storageAccountHnsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_hns_enabled: Option<bool>,
    #[doc = "Name of the storage account"]
    #[serde(rename = "storageAccountName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_name: Option<String>,
    #[doc = "Allowed values:\r\n\"Standard_LRS\",\r\n\"Standard_GRS\",\r\n\"Standard_RAGRS\",\r\n\"Standard_ZRS\",\r\n\"Standard_GZRS\",\r\n\"Standard_RAGZRS\",\r\n\"Premium_LRS\",\r\n\"Premium_ZRS\""]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<String>,
}
impl SystemCreatedStorageAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A system service running on a compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemService {
    #[doc = "The type of this system service."]
    #[serde(rename = "systemServiceType", default, skip_serializing_if = "Option::is_none")]
    pub system_service_type: Option<String>,
    #[doc = "Public IP address"]
    #[serde(rename = "publicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    #[doc = "The version for this type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl SystemService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Fixed training parameters that won't be swept over during AutoML Table training."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableFixedParameters {
    #[doc = "Specify the boosting type, e.g gbdt for XGBoost."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub booster: Option<String>,
    #[doc = "Specify the boosting type, e.g gbdt for LightGBM."]
    #[serde(rename = "boostingType", default, skip_serializing_if = "Option::is_none")]
    pub boosting_type: Option<String>,
    #[doc = "Specify the grow policy, which controls the way new nodes are added to the tree."]
    #[serde(rename = "growPolicy", default, skip_serializing_if = "Option::is_none")]
    pub grow_policy: Option<String>,
    #[doc = "The learning rate for the training procedure."]
    #[serde(rename = "learningRate", default, skip_serializing_if = "Option::is_none")]
    pub learning_rate: Option<f64>,
    #[doc = "Specify the Maximum number of discrete bins to bucket continuous features ."]
    #[serde(rename = "maxBin", default, skip_serializing_if = "Option::is_none")]
    pub max_bin: Option<i32>,
    #[doc = "Specify the max depth to limit the tree depth explicitly."]
    #[serde(rename = "maxDepth", default, skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<i32>,
    #[doc = "Specify the max leaves to limit the tree leaves explicitly."]
    #[serde(rename = "maxLeaves", default, skip_serializing_if = "Option::is_none")]
    pub max_leaves: Option<i32>,
    #[doc = "The minimum number of data per leaf."]
    #[serde(rename = "minDataInLeaf", default, skip_serializing_if = "Option::is_none")]
    pub min_data_in_leaf: Option<i32>,
    #[doc = "Minimum loss reduction required to make a further partition on a leaf node of the tree."]
    #[serde(rename = "minSplitGain", default, skip_serializing_if = "Option::is_none")]
    pub min_split_gain: Option<f64>,
    #[doc = "The name of the model to train."]
    #[serde(rename = "modelName", default, skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[doc = "Specify the number of trees (or rounds) in an model."]
    #[serde(rename = "nEstimators", default, skip_serializing_if = "Option::is_none")]
    pub n_estimators: Option<i32>,
    #[doc = "Specify the number of leaves."]
    #[serde(rename = "numLeaves", default, skip_serializing_if = "Option::is_none")]
    pub num_leaves: Option<i32>,
    #[doc = "The name of the preprocessor to use."]
    #[serde(rename = "preprocessorName", default, skip_serializing_if = "Option::is_none")]
    pub preprocessor_name: Option<String>,
    #[doc = "L1 regularization term on weights."]
    #[serde(rename = "regAlpha", default, skip_serializing_if = "Option::is_none")]
    pub reg_alpha: Option<f64>,
    #[doc = "L2 regularization term on weights."]
    #[serde(rename = "regLambda", default, skip_serializing_if = "Option::is_none")]
    pub reg_lambda: Option<f64>,
    #[doc = "Subsample ratio of the training instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subsample: Option<f64>,
    #[doc = "Frequency of subsample."]
    #[serde(rename = "subsampleFreq", default, skip_serializing_if = "Option::is_none")]
    pub subsample_freq: Option<f64>,
    #[doc = "Specify the tree method."]
    #[serde(rename = "treeMethod", default, skip_serializing_if = "Option::is_none")]
    pub tree_method: Option<String>,
    #[doc = "If true, center before scaling the data with StandardScalar."]
    #[serde(rename = "withMean", default, skip_serializing_if = "Option::is_none")]
    pub with_mean: Option<bool>,
    #[doc = "If true, scaling the data with Unit Variance with StandardScalar."]
    #[serde(rename = "withStd", default, skip_serializing_if = "Option::is_none")]
    pub with_std: Option<bool>,
}
impl TableFixedParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableParameterSubspace {
    #[doc = "Specify the boosting type, e.g gbdt for XGBoost."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub booster: Option<String>,
    #[doc = "Specify the boosting type, e.g gbdt for LightGBM."]
    #[serde(rename = "boostingType", default, skip_serializing_if = "Option::is_none")]
    pub boosting_type: Option<String>,
    #[doc = "Specify the grow policy, which controls the way new nodes are added to the tree."]
    #[serde(rename = "growPolicy", default, skip_serializing_if = "Option::is_none")]
    pub grow_policy: Option<String>,
    #[doc = "The learning rate for the training procedure."]
    #[serde(rename = "learningRate", default, skip_serializing_if = "Option::is_none")]
    pub learning_rate: Option<String>,
    #[doc = "Specify the Maximum number of discrete bins to bucket continuous features ."]
    #[serde(rename = "maxBin", default, skip_serializing_if = "Option::is_none")]
    pub max_bin: Option<String>,
    #[doc = "Specify the max depth to limit the tree depth explicitly."]
    #[serde(rename = "maxDepth", default, skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<String>,
    #[doc = "Specify the max leaves to limit the tree leaves explicitly."]
    #[serde(rename = "maxLeaves", default, skip_serializing_if = "Option::is_none")]
    pub max_leaves: Option<String>,
    #[doc = "The minimum number of data per leaf."]
    #[serde(rename = "minDataInLeaf", default, skip_serializing_if = "Option::is_none")]
    pub min_data_in_leaf: Option<String>,
    #[doc = "Minimum loss reduction required to make a further partition on a leaf node of the tree."]
    #[serde(rename = "minSplitGain", default, skip_serializing_if = "Option::is_none")]
    pub min_split_gain: Option<String>,
    #[doc = "The name of the model to train."]
    #[serde(rename = "modelName", default, skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[doc = "Specify the number of trees (or rounds) in an model."]
    #[serde(rename = "nEstimators", default, skip_serializing_if = "Option::is_none")]
    pub n_estimators: Option<String>,
    #[doc = "Specify the number of leaves."]
    #[serde(rename = "numLeaves", default, skip_serializing_if = "Option::is_none")]
    pub num_leaves: Option<String>,
    #[doc = "The name of the preprocessor to use."]
    #[serde(rename = "preprocessorName", default, skip_serializing_if = "Option::is_none")]
    pub preprocessor_name: Option<String>,
    #[doc = "L1 regularization term on weights."]
    #[serde(rename = "regAlpha", default, skip_serializing_if = "Option::is_none")]
    pub reg_alpha: Option<String>,
    #[doc = "L2 regularization term on weights."]
    #[serde(rename = "regLambda", default, skip_serializing_if = "Option::is_none")]
    pub reg_lambda: Option<String>,
    #[doc = "Subsample ratio of the training instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subsample: Option<String>,
    #[doc = "Frequency of subsample"]
    #[serde(rename = "subsampleFreq", default, skip_serializing_if = "Option::is_none")]
    pub subsample_freq: Option<String>,
    #[doc = "Specify the tree method."]
    #[serde(rename = "treeMethod", default, skip_serializing_if = "Option::is_none")]
    pub tree_method: Option<String>,
    #[doc = "If true, center before scaling the data with StandardScalar."]
    #[serde(rename = "withMean", default, skip_serializing_if = "Option::is_none")]
    pub with_mean: Option<String>,
    #[doc = "If true, scaling the data with Unit Variance with StandardScalar."]
    #[serde(rename = "withStd", default, skip_serializing_if = "Option::is_none")]
    pub with_std: Option<String>,
}
impl TableParameterSubspace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableSweepSettings {
    #[doc = "Early termination policies enable canceling poor-performing runs before they complete"]
    #[serde(rename = "earlyTermination", default, skip_serializing_if = "Option::is_none")]
    pub early_termination: Option<EarlyTerminationPolicy>,
    #[serde(rename = "samplingAlgorithm")]
    pub sampling_algorithm: SamplingAlgorithmType,
}
impl TableSweepSettings {
    pub fn new(sampling_algorithm: SamplingAlgorithmType) -> Self {
        Self {
            early_termination: None,
            sampling_algorithm,
        }
    }
}
#[doc = "Abstract class for AutoML tasks that use table dataset as input - such as Classification/Regression/Forecasting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableVertical {
    #[doc = "Columns to use for CVSplit data."]
    #[serde(
        rename = "cvSplitColumnNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cv_split_column_names: Vec<String>,
    #[doc = "Featurization Configuration."]
    #[serde(rename = "featurizationSettings", default, skip_serializing_if = "Option::is_none")]
    pub featurization_settings: Option<TableVerticalFeaturizationSettings>,
    #[doc = "Fixed training parameters that won't be swept over during AutoML Table training."]
    #[serde(rename = "fixedParameters", default, skip_serializing_if = "Option::is_none")]
    pub fixed_parameters: Option<TableFixedParameters>,
    #[doc = "Job execution constraints."]
    #[serde(rename = "limitSettings", default, skip_serializing_if = "Option::is_none")]
    pub limit_settings: Option<TableVerticalLimitSettings>,
    #[doc = "N-Cross validations value."]
    #[serde(rename = "nCrossValidations", default, skip_serializing_if = "Option::is_none")]
    pub n_cross_validations: Option<NCrossValidations>,
    #[doc = "Search space for sampling different combinations of models and their hyperparameters."]
    #[serde(
        rename = "searchSpace",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub search_space: Vec<TableParameterSubspace>,
    #[serde(rename = "sweepSettings", default, skip_serializing_if = "Option::is_none")]
    pub sweep_settings: Option<TableSweepSettings>,
    #[serde(rename = "testData", default, skip_serializing_if = "Option::is_none")]
    pub test_data: Option<MlTableJobInput>,
    #[doc = "The fraction of test dataset that needs to be set aside for validation purpose.\r\nValues between (0.0 , 1.0)\r\nApplied when validation dataset is not provided."]
    #[serde(rename = "testDataSize", default, skip_serializing_if = "Option::is_none")]
    pub test_data_size: Option<f64>,
    #[serde(rename = "validationData", default, skip_serializing_if = "Option::is_none")]
    pub validation_data: Option<MlTableJobInput>,
    #[doc = "The fraction of training dataset that needs to be set aside for validation purpose.\r\nValues between (0.0 , 1.0)\r\nApplied when validation dataset is not provided."]
    #[serde(rename = "validationDataSize", default, skip_serializing_if = "Option::is_none")]
    pub validation_data_size: Option<f64>,
    #[doc = "The name of the sample weight column. Automated ML supports a weighted column as an input, causing rows in the data to be weighted up or down."]
    #[serde(rename = "weightColumnName", default, skip_serializing_if = "Option::is_none")]
    pub weight_column_name: Option<String>,
}
impl TableVertical {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Featurization Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableVerticalFeaturizationSettings {
    #[serde(flatten)]
    pub featurization_settings: FeaturizationSettings,
    #[doc = "These transformers shall not be used in featurization."]
    #[serde(
        rename = "blockedTransformers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_transformers: Vec<BlockedTransformers>,
    #[doc = "Dictionary of column name and its type (int, float, string, datetime etc)."]
    #[serde(rename = "columnNameAndTypes", default, skip_serializing_if = "Option::is_none")]
    pub column_name_and_types: Option<serde_json::Value>,
    #[doc = "Determines whether to use Dnn based featurizers for data featurization."]
    #[serde(rename = "enableDnnFeaturization", default, skip_serializing_if = "Option::is_none")]
    pub enable_dnn_featurization: Option<bool>,
    #[doc = "Featurization mode - determines data featurization mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<FeaturizationMode>,
    #[doc = "User can specify additional transformers to be used along with the columns to which it would be applied and parameters for the transformer constructor."]
    #[serde(rename = "transformerParams", default, skip_serializing_if = "Option::is_none")]
    pub transformer_params: Option<serde_json::Value>,
}
impl TableVerticalFeaturizationSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job execution constraints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableVerticalLimitSettings {
    #[doc = "Enable early termination, determines whether or not if AutoMLJob will terminate early if there is no score improvement in last 20 iterations."]
    #[serde(rename = "enableEarlyTermination", default, skip_serializing_if = "Option::is_none")]
    pub enable_early_termination: Option<bool>,
    #[doc = "Exit score for the AutoML job."]
    #[serde(rename = "exitScore", default, skip_serializing_if = "Option::is_none")]
    pub exit_score: Option<f64>,
    #[doc = "Maximum Concurrent iterations."]
    #[serde(rename = "maxConcurrentTrials", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_trials: Option<i32>,
    #[doc = "Max cores per iteration."]
    #[serde(rename = "maxCoresPerTrial", default, skip_serializing_if = "Option::is_none")]
    pub max_cores_per_trial: Option<i32>,
    #[doc = "Maximum nodes to use for the experiment."]
    #[serde(rename = "maxNodes", default, skip_serializing_if = "Option::is_none")]
    pub max_nodes: Option<i32>,
    #[doc = "Number of iterations."]
    #[serde(rename = "maxTrials", default, skip_serializing_if = "Option::is_none")]
    pub max_trials: Option<i32>,
    #[doc = "Number of concurrent sweeping runs that user wants to trigger."]
    #[serde(rename = "sweepConcurrentTrials", default, skip_serializing_if = "Option::is_none")]
    pub sweep_concurrent_trials: Option<i32>,
    #[doc = "Number of sweeping runs that user wants to trigger."]
    #[serde(rename = "sweepTrials", default, skip_serializing_if = "Option::is_none")]
    pub sweep_trials: Option<i32>,
    #[doc = "AutoML job timeout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[doc = "Iteration timeout."]
    #[serde(rename = "trialTimeout", default, skip_serializing_if = "Option::is_none")]
    pub trial_timeout: Option<String>,
}
impl TableVerticalLimitSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Target aggregate function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TargetAggregationFunction")]
pub enum TargetAggregationFunction {
    None,
    Sum,
    Max,
    Min,
    Mean,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TargetAggregationFunction {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TargetAggregationFunction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TargetAggregationFunction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("TargetAggregationFunction", 0u32, "None"),
            Self::Sum => serializer.serialize_unit_variant("TargetAggregationFunction", 1u32, "Sum"),
            Self::Max => serializer.serialize_unit_variant("TargetAggregationFunction", 2u32, "Max"),
            Self::Min => serializer.serialize_unit_variant("TargetAggregationFunction", 3u32, "Min"),
            Self::Mean => serializer.serialize_unit_variant("TargetAggregationFunction", 4u32, "Mean"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The number of past periods to lag from the target column."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetLags {
    #[doc = "Target lags selection modes."]
    pub mode: TargetLagsMode,
}
impl TargetLags {
    pub fn new(mode: TargetLagsMode) -> Self {
        Self { mode }
    }
}
#[doc = "Target lags selection modes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TargetLagsMode")]
pub enum TargetLagsMode {
    Auto,
    Custom,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TargetLagsMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TargetLagsMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TargetLagsMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Auto => serializer.serialize_unit_variant("TargetLagsMode", 0u32, "Auto"),
            Self::Custom => serializer.serialize_unit_variant("TargetLagsMode", 1u32, "Custom"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Forecasting target rolling window size."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetRollingWindowSize {
    #[doc = "Target rolling windows size mode."]
    pub mode: TargetRollingWindowSizeMode,
}
impl TargetRollingWindowSize {
    pub fn new(mode: TargetRollingWindowSizeMode) -> Self {
        Self { mode }
    }
}
#[doc = "Target rolling windows size mode."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TargetRollingWindowSizeMode")]
pub enum TargetRollingWindowSizeMode {
    Auto,
    Custom,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TargetRollingWindowSizeMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TargetRollingWindowSizeMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TargetRollingWindowSizeMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Auto => serializer.serialize_unit_variant("TargetRollingWindowSizeMode", 0u32, "Auto"),
            Self::Custom => serializer.serialize_unit_variant("TargetRollingWindowSizeMode", 1u32, "Custom"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetUtilizationScaleSettings {
    #[serde(flatten)]
    pub online_scale_settings: OnlineScaleSettings,
    #[doc = "The maximum number of instances that the deployment can scale to. The quota will be reserved for max_instances."]
    #[serde(rename = "maxInstances", default, skip_serializing_if = "Option::is_none")]
    pub max_instances: Option<i32>,
    #[doc = "The minimum number of instances to always be present."]
    #[serde(rename = "minInstances", default, skip_serializing_if = "Option::is_none")]
    pub min_instances: Option<i32>,
    #[doc = "The polling interval in ISO 8691 format. Only supports duration with precision as low as Seconds."]
    #[serde(rename = "pollingInterval", default, skip_serializing_if = "Option::is_none")]
    pub polling_interval: Option<String>,
    #[doc = "Target CPU usage for the autoscaler."]
    #[serde(rename = "targetUtilizationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub target_utilization_percentage: Option<i32>,
}
impl TargetUtilizationScaleSettings {
    pub fn new(online_scale_settings: OnlineScaleSettings) -> Self {
        Self {
            online_scale_settings,
            max_instances: None,
            min_instances: None,
            polling_interval: None,
            target_utilization_percentage: None,
        }
    }
}
#[doc = "AutoMLJob Task type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TaskType")]
pub enum TaskType {
    Classification,
    Regression,
    Forecasting,
    ImageClassification,
    ImageClassificationMultilabel,
    ImageObjectDetection,
    ImageInstanceSegmentation,
    TextClassification,
    TextClassificationMultilabel,
    #[serde(rename = "TextNER")]
    TextNer,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TaskType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TaskType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TaskType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Classification => serializer.serialize_unit_variant("TaskType", 0u32, "Classification"),
            Self::Regression => serializer.serialize_unit_variant("TaskType", 1u32, "Regression"),
            Self::Forecasting => serializer.serialize_unit_variant("TaskType", 2u32, "Forecasting"),
            Self::ImageClassification => serializer.serialize_unit_variant("TaskType", 3u32, "ImageClassification"),
            Self::ImageClassificationMultilabel => serializer.serialize_unit_variant("TaskType", 4u32, "ImageClassificationMultilabel"),
            Self::ImageObjectDetection => serializer.serialize_unit_variant("TaskType", 5u32, "ImageObjectDetection"),
            Self::ImageInstanceSegmentation => serializer.serialize_unit_variant("TaskType", 6u32, "ImageInstanceSegmentation"),
            Self::TextClassification => serializer.serialize_unit_variant("TaskType", 7u32, "TextClassification"),
            Self::TextClassificationMultilabel => serializer.serialize_unit_variant("TaskType", 8u32, "TextClassificationMultilabel"),
            Self::TextNer => serializer.serialize_unit_variant("TaskType", 9u32, "TextNER"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "TensorFlow distribution configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TensorFlow {
    #[serde(flatten)]
    pub distribution_configuration: DistributionConfiguration,
    #[doc = "Number of parameter server tasks."]
    #[serde(rename = "parameterServerCount", default, skip_serializing_if = "Option::is_none")]
    pub parameter_server_count: Option<i32>,
    #[doc = "Number of workers. If not specified, will default to the instance count."]
    #[serde(rename = "workerCount", default, skip_serializing_if = "Option::is_none")]
    pub worker_count: Option<i32>,
}
impl TensorFlow {
    pub fn new(distribution_configuration: DistributionConfiguration) -> Self {
        Self {
            distribution_configuration,
            parameter_server_count: None,
            worker_count: None,
        }
    }
}
#[doc = "Annotation type of text data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TextAnnotationType")]
pub enum TextAnnotationType {
    Classification,
    NamedEntityRecognition,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TextAnnotationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TextAnnotationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TextAnnotationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Classification => serializer.serialize_unit_variant("TextAnnotationType", 0u32, "Classification"),
            Self::NamedEntityRecognition => serializer.serialize_unit_variant("TextAnnotationType", 1u32, "NamedEntityRecognition"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Text Classification task in AutoML NLP vertical.\r\nNLP - Natural Language Processing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextClassification {
    #[serde(flatten)]
    pub nlp_vertical: NlpVertical,
    #[serde(flatten)]
    pub auto_ml_vertical: AutoMlVertical,
    #[doc = "Primary metrics for classification tasks."]
    #[serde(rename = "primaryMetric", default, skip_serializing_if = "Option::is_none")]
    pub primary_metric: Option<ClassificationPrimaryMetrics>,
}
impl TextClassification {
    pub fn new(auto_ml_vertical: AutoMlVertical) -> Self {
        Self {
            nlp_vertical: NlpVertical::default(),
            auto_ml_vertical,
            primary_metric: None,
        }
    }
}
#[doc = "Text Classification Multilabel task in AutoML NLP vertical.\r\nNLP - Natural Language Processing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextClassificationMultilabel {
    #[serde(flatten)]
    pub nlp_vertical: NlpVertical,
    #[serde(flatten)]
    pub auto_ml_vertical: AutoMlVertical,
    #[doc = "Primary metrics for classification multilabel tasks."]
    #[serde(rename = "primaryMetric", default, skip_serializing_if = "Option::is_none")]
    pub primary_metric: Option<ClassificationMultilabelPrimaryMetrics>,
}
impl TextClassificationMultilabel {
    pub fn new(auto_ml_vertical: AutoMlVertical) -> Self {
        Self {
            nlp_vertical: NlpVertical::default(),
            auto_ml_vertical,
            primary_metric: None,
        }
    }
}
#[doc = "Text-NER task in AutoML NLP vertical.\r\nNER - Named Entity Recognition.\r\nNLP - Natural Language Processing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextNer {
    #[serde(flatten)]
    pub nlp_vertical: NlpVertical,
    #[serde(flatten)]
    pub auto_ml_vertical: AutoMlVertical,
    #[doc = "Primary metrics for classification tasks."]
    #[serde(rename = "primaryMetric", default, skip_serializing_if = "Option::is_none")]
    pub primary_metric: Option<ClassificationPrimaryMetrics>,
}
impl TextNer {
    pub fn new(auto_ml_vertical: AutoMlVertical) -> Self {
        Self {
            nlp_vertical: NlpVertical::default(),
            auto_ml_vertical,
            primary_metric: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TmpfsOptions {
    #[doc = "Mention the Tmpfs size"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}
impl TmpfsOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopNFeaturesByAttribution {
    #[serde(flatten)]
    pub monitoring_feature_filter_base: MonitoringFeatureFilterBase,
    #[doc = "The number of top features to include."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
}
impl TopNFeaturesByAttribution {
    pub fn new(monitoring_feature_filter_base: MonitoringFeatureFilterBase) -> Self {
        Self {
            monitoring_feature_filter_base,
            top: None,
        }
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
#[doc = "Training mode dictates whether to use distributed training or not"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TrainingMode")]
pub enum TrainingMode {
    Auto,
    Distributed,
    NonDistributed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TrainingMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TrainingMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TrainingMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Auto => serializer.serialize_unit_variant("TrainingMode", 0u32, "Auto"),
            Self::Distributed => serializer.serialize_unit_variant("TrainingMode", 1u32, "Distributed"),
            Self::NonDistributed => serializer.serialize_unit_variant("TrainingMode", 2u32, "NonDistributed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Training related configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrainingSettings {
    #[doc = "Enable recommendation of DNN models."]
    #[serde(rename = "enableDnnTraining", default, skip_serializing_if = "Option::is_none")]
    pub enable_dnn_training: Option<bool>,
    #[doc = "Flag to turn on explainability on best model."]
    #[serde(rename = "enableModelExplainability", default, skip_serializing_if = "Option::is_none")]
    pub enable_model_explainability: Option<bool>,
    #[doc = "Flag for enabling onnx compatible models."]
    #[serde(rename = "enableOnnxCompatibleModels", default, skip_serializing_if = "Option::is_none")]
    pub enable_onnx_compatible_models: Option<bool>,
    #[doc = "Enable stack ensemble run."]
    #[serde(rename = "enableStackEnsemble", default, skip_serializing_if = "Option::is_none")]
    pub enable_stack_ensemble: Option<bool>,
    #[doc = "Enable voting ensemble run."]
    #[serde(rename = "enableVoteEnsemble", default, skip_serializing_if = "Option::is_none")]
    pub enable_vote_ensemble: Option<bool>,
    #[doc = "During VotingEnsemble and StackEnsemble model generation, multiple fitted models from the previous child runs are downloaded.\r\nConfigure this parameter with a higher value than 300 secs, if more time is needed."]
    #[serde(rename = "ensembleModelDownloadTimeout", default, skip_serializing_if = "Option::is_none")]
    pub ensemble_model_download_timeout: Option<String>,
    #[doc = "Advances setting to customize StackEnsemble run."]
    #[serde(rename = "stackEnsembleSettings", default, skip_serializing_if = "Option::is_none")]
    pub stack_ensemble_settings: Option<StackEnsembleSettings>,
    #[doc = "Training mode dictates whether to use distributed training or not"]
    #[serde(rename = "trainingMode", default, skip_serializing_if = "Option::is_none")]
    pub training_mode: Option<TrainingMode>,
}
impl TrainingSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Trial component definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrialComponent {
    #[doc = "ARM resource ID of the code asset."]
    #[serde(rename = "codeId", default, skip_serializing_if = "Option::is_none")]
    pub code_id: Option<String>,
    #[doc = "[Required] The command to execute on startup of the job. eg. \"python train.py\""]
    pub command: String,
    #[doc = "Base definition for job distribution configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribution: Option<DistributionConfiguration>,
    #[doc = "[Required] The ARM resource ID of the Environment specification for the job."]
    #[serde(rename = "environmentId")]
    pub environment_id: String,
    #[doc = "Environment variables included in the job."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<JobResourceConfiguration>,
}
impl TrialComponent {
    pub fn new(command: String, environment_id: String) -> Self {
        Self {
            code_id: None,
            command,
            distribution: None,
            environment_id,
            environment_variables: None,
            resources: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerBase {
    #[doc = "Specifies end time of schedule in ISO 8601, but without a UTC offset. Refer https://en.wikipedia.org/wiki/ISO_8601.\r\nRecommented format would be \"2022-06-01T00:00:01\"\r\nIf not present, the schedule will run indefinitely"]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "Specifies start time of schedule in ISO 8601 format, but without a UTC offset."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Specifies time zone in which the schedule runs.\r\nTimeZone should follow Windows time zone format. Refer: https://docs.microsoft.com/en-us/windows-hardware/manufacture/desktop/default-time-zones?view=windows-11"]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(rename = "triggerType")]
    pub trigger_type: TriggerType,
}
impl TriggerBase {
    pub fn new(trigger_type: TriggerType) -> Self {
        Self {
            end_time: None,
            start_time: None,
            time_zone: None,
            trigger_type,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TriggerType")]
pub enum TriggerType {
    Recurrence,
    Cron,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TriggerType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TriggerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TriggerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Recurrence => serializer.serialize_unit_variant("TriggerType", 0u32, "Recurrence"),
            Self::Cron => serializer.serialize_unit_variant("TriggerType", 1u32, "Cron"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Triton inferencing server configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TritonInferencingServer {
    #[serde(flatten)]
    pub inferencing_server: InferencingServer,
    #[doc = "Online inference configuration options."]
    #[serde(rename = "inferenceConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub inference_configuration: Option<OnlineInferenceConfiguration>,
}
impl TritonInferencingServer {
    pub fn new(inferencing_server: InferencingServer) -> Self {
        Self {
            inferencing_server,
            inference_configuration: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TritonModelJobInput {
    #[serde(flatten)]
    pub asset_job_input: AssetJobInput,
    #[serde(flatten)]
    pub job_input: JobInput,
}
impl TritonModelJobInput {
    pub fn new(asset_job_input: AssetJobInput, job_input: JobInput) -> Self {
        Self {
            asset_job_input,
            job_input,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TritonModelJobOutput {
    #[serde(flatten)]
    pub asset_job_output: AssetJobOutput,
    #[serde(flatten)]
    pub job_output: JobOutput,
}
impl TritonModelJobOutput {
    pub fn new(job_output: JobOutput) -> Self {
        Self {
            asset_job_output: AssetJobOutput::default(),
            job_output,
        }
    }
}
#[doc = "Defines an early termination policy that cancels a given percentage of runs at each evaluation interval."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TruncationSelectionPolicy {
    #[serde(flatten)]
    pub early_termination_policy: EarlyTerminationPolicy,
    #[doc = "The percentage of runs to cancel at each evaluation interval."]
    #[serde(rename = "truncationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub truncation_percentage: Option<i32>,
}
impl TruncationSelectionPolicy {
    pub fn new(early_termination_policy: EarlyTerminationPolicy) -> Self {
        Self {
            early_termination_policy,
            truncation_percentage: None,
        }
    }
}
#[doc = "The properties for update Quota response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateWorkspaceQuotas {
    #[doc = "Specifies the resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Specifies the resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The maximum permitted quota of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "An enum describing the unit of quota measurement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<update_workspace_quotas::Unit>,
    #[doc = "Status of update workspace quota."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<update_workspace_quotas::Status>,
}
impl UpdateWorkspaceQuotas {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_workspace_quotas {
    use super::*;
    #[doc = "An enum describing the unit of quota measurement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Count,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Count => serializer.serialize_unit_variant("Unit", 0u32, "Count"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Status of update workspace quota."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Undefined,
        Success,
        Failure,
        InvalidQuotaBelowClusterMinimum,
        InvalidQuotaExceedsSubscriptionLimit,
        #[serde(rename = "InvalidVMFamilyName")]
        InvalidVmFamilyName,
        OperationNotSupportedForSku,
        OperationNotEnabledForRegion,
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
                Self::Undefined => serializer.serialize_unit_variant("Status", 0u32, "Undefined"),
                Self::Success => serializer.serialize_unit_variant("Status", 1u32, "Success"),
                Self::Failure => serializer.serialize_unit_variant("Status", 2u32, "Failure"),
                Self::InvalidQuotaBelowClusterMinimum => {
                    serializer.serialize_unit_variant("Status", 3u32, "InvalidQuotaBelowClusterMinimum")
                }
                Self::InvalidQuotaExceedsSubscriptionLimit => {
                    serializer.serialize_unit_variant("Status", 4u32, "InvalidQuotaExceedsSubscriptionLimit")
                }
                Self::InvalidVmFamilyName => serializer.serialize_unit_variant("Status", 5u32, "InvalidVMFamilyName"),
                Self::OperationNotSupportedForSku => serializer.serialize_unit_variant("Status", 6u32, "OperationNotSupportedForSku"),
                Self::OperationNotEnabledForRegion => serializer.serialize_unit_variant("Status", 7u32, "OperationNotEnabledForRegion"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The result of update workspace quota."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateWorkspaceQuotasResult {
    #[doc = "The list of workspace quota update result."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<UpdateWorkspaceQuotas>,
    #[doc = "The URI to fetch the next page of workspace quota update result. Call ListNext() with this to fetch the next page of Workspace Quota update result."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl UpdateWorkspaceQuotasResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "uri-file data version entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UriFileDataVersion {
    #[serde(flatten)]
    pub data_version_base: DataVersionBase,
}
impl UriFileDataVersion {
    pub fn new(data_version_base: DataVersionBase) -> Self {
        Self { data_version_base }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UriFileJobInput {
    #[serde(flatten)]
    pub asset_job_input: AssetJobInput,
    #[serde(flatten)]
    pub job_input: JobInput,
}
impl UriFileJobInput {
    pub fn new(asset_job_input: AssetJobInput, job_input: JobInput) -> Self {
        Self {
            asset_job_input,
            job_input,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UriFileJobOutput {
    #[serde(flatten)]
    pub asset_job_output: AssetJobOutput,
    #[serde(flatten)]
    pub job_output: JobOutput,
}
impl UriFileJobOutput {
    pub fn new(job_output: JobOutput) -> Self {
        Self {
            asset_job_output: AssetJobOutput::default(),
            job_output,
        }
    }
}
#[doc = "uri-folder data version entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UriFolderDataVersion {
    #[serde(flatten)]
    pub data_version_base: DataVersionBase,
}
impl UriFolderDataVersion {
    pub fn new(data_version_base: DataVersionBase) -> Self {
        Self { data_version_base }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UriFolderJobInput {
    #[serde(flatten)]
    pub asset_job_input: AssetJobInput,
    #[serde(flatten)]
    pub job_input: JobInput,
}
impl UriFolderJobInput {
    pub fn new(asset_job_input: AssetJobInput, job_input: JobInput) -> Self {
        Self {
            asset_job_input,
            job_input,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UriFolderJobOutput {
    #[serde(flatten)]
    pub asset_job_output: AssetJobOutput,
    #[serde(flatten)]
    pub job_output: JobOutput,
}
impl UriFolderJobOutput {
    pub fn new(job_output: JobOutput) -> Self {
        Self {
            asset_job_output: AssetJobOutput::default(),
            job_output,
        }
    }
}
#[doc = "Describes AML Resource Usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[doc = "Specifies the resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Region of the AML workspace in the id."]
    #[serde(rename = "amlWorkspaceLocation", default, skip_serializing_if = "Option::is_none")]
    pub aml_workspace_location: Option<String>,
    #[doc = "Specifies the resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "An enum describing the unit of usage measurement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<usage::Unit>,
    #[doc = "The current usage of the resource."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "The maximum permitted usage of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "The Usage Names."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<UsageName>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod usage {
    use super::*;
    #[doc = "An enum describing the unit of usage measurement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Count,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Count => serializer.serialize_unit_variant("Unit", 0u32, "Count"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Usage Names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageName {
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The localized name of the resource."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl UsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configure STL Decomposition of the time-series target column."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UseStl")]
pub enum UseStl {
    None,
    Season,
    SeasonTrend,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UseStl {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UseStl {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UseStl {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("UseStl", 0u32, "None"),
            Self::Season => serializer.serialize_unit_variant("UseStl", 1u32, "Season"),
            Self::SeasonTrend => serializer.serialize_unit_variant("UseStl", 2u32, "SeasonTrend"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Settings for user account that gets created on each on the nodes of a compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAccountCredentials {
    #[doc = "Name of the administrator user account which can be used to SSH to nodes."]
    #[serde(rename = "adminUserName")]
    pub admin_user_name: String,
    #[doc = "SSH public key of the administrator user account."]
    #[serde(rename = "adminUserSshPublicKey", default, skip_serializing_if = "Option::is_none")]
    pub admin_user_ssh_public_key: Option<String>,
    #[doc = "Password of the administrator user account."]
    #[serde(rename = "adminUserPassword", default, skip_serializing_if = "Option::is_none")]
    pub admin_user_password: Option<String>,
}
impl UserAccountCredentials {
    pub fn new(admin_user_name: String) -> Self {
        Self {
            admin_user_name,
            admin_user_ssh_public_key: None,
            admin_user_password: None,
        }
    }
}
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserCreatedAcrAccount {
    #[doc = "ARM ResourceId of a resource"]
    #[serde(rename = "armResourceId", default, skip_serializing_if = "Option::is_none")]
    pub arm_resource_id: Option<ArmResourceId>,
}
impl UserCreatedAcrAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserCreatedStorageAccount {
    #[doc = "ARM ResourceId of a resource"]
    #[serde(rename = "armResourceId", default, skip_serializing_if = "Option::is_none")]
    pub arm_resource_id: Option<ArmResourceId>,
}
impl UserCreatedStorageAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User identity configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserIdentity {
    #[serde(flatten)]
    pub identity_configuration: IdentityConfiguration,
}
impl UserIdentity {
    pub fn new(identity_configuration: IdentityConfiguration) -> Self {
        Self { identity_configuration }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsernamePasswordAuthTypeWorkspaceConnectionProperties {
    #[serde(flatten)]
    pub workspace_connection_properties_v2: WorkspaceConnectionPropertiesV2,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<WorkspaceConnectionUsernamePassword>,
}
impl UsernamePasswordAuthTypeWorkspaceConnectionProperties {
    pub fn new(workspace_connection_properties_v2: WorkspaceConnectionPropertiesV2) -> Self {
        Self {
            workspace_connection_properties_v2,
            credentials: None,
        }
    }
}
#[doc = "Metric computation method to use for validation metrics in image tasks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ValidationMetricType")]
pub enum ValidationMetricType {
    None,
    Coco,
    Voc,
    CocoVoc,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ValidationMetricType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ValidationMetricType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ValidationMetricType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ValidationMetricType", 0u32, "None"),
            Self::Coco => serializer.serialize_unit_variant("ValidationMetricType", 1u32, "Coco"),
            Self::Voc => serializer.serialize_unit_variant("ValidationMetricType", 2u32, "Voc"),
            Self::CocoVoc => serializer.serialize_unit_variant("ValidationMetricType", 3u32, "CocoVoc"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A Machine Learning compute based on Azure Virtual Machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachine {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(flatten)]
    pub virtual_machine_schema: VirtualMachineSchema,
}
impl VirtualMachine {
    pub fn new(compute: Compute) -> Self {
        Self {
            compute,
            virtual_machine_schema: VirtualMachineSchema::default(),
        }
    }
}
#[doc = "Virtual Machine image for Windows AML Compute"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineImage {
    #[doc = "Virtual Machine image path"]
    pub id: String,
}
impl VirtualMachineImage {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<virtual_machine_schema::Properties>,
}
impl VirtualMachineSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_schema {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Virtual Machine size"]
        #[serde(rename = "virtualMachineSize", default, skip_serializing_if = "Option::is_none")]
        pub virtual_machine_size: Option<String>,
        #[doc = "Port open for ssh connections."]
        #[serde(rename = "sshPort", default, skip_serializing_if = "Option::is_none")]
        pub ssh_port: Option<i32>,
        #[doc = "Notebook server port open for ssh connections."]
        #[serde(rename = "notebookServerPort", default, skip_serializing_if = "Option::is_none")]
        pub notebook_server_port: Option<i32>,
        #[doc = "Public IP address of the virtual machine."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[doc = "Admin credentials for virtual machine"]
        #[serde(rename = "administratorAccount", default, skip_serializing_if = "Option::is_none")]
        pub administrator_account: Option<VirtualMachineSshCredentials>,
        #[doc = "Indicates whether this compute will be used for running notebooks."]
        #[serde(rename = "isNotebookInstanceCompute", default, skip_serializing_if = "Option::is_none")]
        pub is_notebook_instance_compute: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Secrets related to a Machine Learning compute based on AKS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineSecrets {
    #[serde(flatten)]
    pub compute_secrets: ComputeSecrets,
    #[serde(flatten)]
    pub virtual_machine_secrets_schema: VirtualMachineSecretsSchema,
}
impl VirtualMachineSecrets {
    pub fn new(compute_secrets: ComputeSecrets) -> Self {
        Self {
            compute_secrets,
            virtual_machine_secrets_schema: VirtualMachineSecretsSchema::default(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineSecretsSchema {
    #[doc = "Admin credentials for virtual machine"]
    #[serde(rename = "administratorAccount", default, skip_serializing_if = "Option::is_none")]
    pub administrator_account: Option<VirtualMachineSshCredentials>,
}
impl VirtualMachineSecretsSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a VM size."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineSize {
    #[doc = "The name of the virtual machine size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The family name of the virtual machine size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The number of vCPUs supported by the virtual machine size."]
    #[serde(rename = "vCPUs", default, skip_serializing_if = "Option::is_none")]
    pub v_cp_us: Option<i32>,
    #[doc = "The number of gPUs supported by the virtual machine size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gpus: Option<i32>,
    #[doc = "The OS VHD disk size, in MB, allowed by the virtual machine size."]
    #[serde(rename = "osVhdSizeMB", default, skip_serializing_if = "Option::is_none")]
    pub os_vhd_size_mb: Option<i32>,
    #[doc = "The resource volume size, in MB, allowed by the virtual machine size."]
    #[serde(rename = "maxResourceVolumeMB", default, skip_serializing_if = "Option::is_none")]
    pub max_resource_volume_mb: Option<i32>,
    #[doc = "The amount of memory, in GB, supported by the virtual machine size."]
    #[serde(rename = "memoryGB", default, skip_serializing_if = "Option::is_none")]
    pub memory_gb: Option<f64>,
    #[doc = "Specifies if the virtual machine size supports low priority VMs."]
    #[serde(rename = "lowPriorityCapable", default, skip_serializing_if = "Option::is_none")]
    pub low_priority_capable: Option<bool>,
    #[doc = "Specifies if the virtual machine size supports premium IO."]
    #[serde(rename = "premiumIO", default, skip_serializing_if = "Option::is_none")]
    pub premium_io: Option<bool>,
    #[doc = "The estimated price info for using a VM."]
    #[serde(rename = "estimatedVMPrices", default, skip_serializing_if = "Option::is_none")]
    pub estimated_vm_prices: Option<EstimatedVmPrices>,
    #[doc = "Specifies the compute types supported by the virtual machine size."]
    #[serde(
        rename = "supportedComputeTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_compute_types: Vec<String>,
}
impl VirtualMachineSize {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Virtual Machine size operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineSizeListResult {
    #[doc = "The list of virtual machine sizes supported by AmlCompute."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VirtualMachineSize>,
}
impl VirtualMachineSizeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Admin credentials for virtual machine"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineSshCredentials {
    #[doc = "Username of admin account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "Password of admin account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Public key data"]
    #[serde(rename = "publicKeyData", default, skip_serializing_if = "Option::is_none")]
    pub public_key_data: Option<String>,
    #[doc = "Private key data"]
    #[serde(rename = "privateKeyData", default, skip_serializing_if = "Option::is_none")]
    pub private_key_data: Option<String>,
}
impl VirtualMachineSshCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeDefinition {
    #[doc = "Type of Volume Definition. Possible Values: bind,volume,tmpfs,npipe"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<volume_definition::Type>,
    #[doc = "Indicate whether to mount volume as readOnly. Default value for this is false."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "Source of the mount. For bind mounts this is the host path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Target of the mount. For bind mounts this is the path in the container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Consistency of the volume"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consistency: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<BindOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<TmpfsOptions>,
}
impl VolumeDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod volume_definition {
    use super::*;
    #[doc = "Type of Volume Definition. Possible Values: bind,volume,tmpfs,npipe"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "bind")]
        Bind,
        #[serde(rename = "volume")]
        Volume,
        #[serde(rename = "tmpfs")]
        Tmpfs,
        #[serde(rename = "npipe")]
        Npipe,
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
                Self::Bind => serializer.serialize_unit_variant("Type", 0u32, "bind"),
                Self::Volume => serializer.serialize_unit_variant("Type", 1u32, "volume"),
                Self::Tmpfs => serializer.serialize_unit_variant("Type", 2u32, "tmpfs"),
                Self::Npipe => serializer.serialize_unit_variant("Type", 3u32, "npipe"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Type {
        fn default() -> Self {
            Self::Bind
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeOptions {
    #[doc = "Indicate whether volume is nocopy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nocopy: Option<bool>,
}
impl VolumeOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Webhook base"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Webhook {
    #[doc = "Send callback on a specified notification event"]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[doc = "Enum to determine the webhook callback service type."]
    #[serde(rename = "webhookType")]
    pub webhook_type: WebhookType,
}
impl Webhook {
    pub fn new(webhook_type: WebhookType) -> Self {
        Self {
            event_type: None,
            webhook_type,
        }
    }
}
#[doc = "Enum to determine the webhook callback service type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WebhookType")]
pub enum WebhookType {
    AzureDevOps,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WebhookType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WebhookType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WebhookType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureDevOps => serializer.serialize_unit_variant("WebhookType", 0u32, "AzureDevOps"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enum of weekday"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WeekDay")]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
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
            Self::Monday => serializer.serialize_unit_variant("WeekDay", 0u32, "Monday"),
            Self::Tuesday => serializer.serialize_unit_variant("WeekDay", 1u32, "Tuesday"),
            Self::Wednesday => serializer.serialize_unit_variant("WeekDay", 2u32, "Wednesday"),
            Self::Thursday => serializer.serialize_unit_variant("WeekDay", 3u32, "Thursday"),
            Self::Friday => serializer.serialize_unit_variant("WeekDay", 4u32, "Friday"),
            Self::Saturday => serializer.serialize_unit_variant("WeekDay", 5u32, "Saturday"),
            Self::Sunday => serializer.serialize_unit_variant("WeekDay", 6u32, "Sunday"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An object that represents a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Workspace {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a machine learning workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Specifies the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Contains resource tags defined as key/value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl Workspace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceConnectionAccessKey {
    #[serde(rename = "accessKeyId", default, skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "secretAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
}
impl WorkspaceConnectionAccessKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceConnectionManagedIdentity {
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl WorkspaceConnectionManagedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceConnectionPersonalAccessToken {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pat: Option<String>,
}
impl WorkspaceConnectionPersonalAccessToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceConnectionPropertiesV2 {
    #[doc = "Authentication type of the connection target"]
    #[serde(rename = "authType")]
    pub auth_type: ConnectionAuthType,
    #[serde(rename = "expiryTime", default, skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
    #[doc = "Category of the connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<ConnectionCategory>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Value details of the workspace connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "format for the workspace connection value"]
    #[serde(rename = "valueFormat", default, skip_serializing_if = "Option::is_none")]
    pub value_format: Option<workspace_connection_properties_v2::ValueFormat>,
}
impl WorkspaceConnectionPropertiesV2 {
    pub fn new(auth_type: ConnectionAuthType) -> Self {
        Self {
            auth_type,
            expiry_time: None,
            category: None,
            target: None,
            value: None,
            value_format: None,
        }
    }
}
pub mod workspace_connection_properties_v2 {
    use super::*;
    #[doc = "format for the workspace connection value"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ValueFormat")]
    pub enum ValueFormat {
        #[serde(rename = "JSON")]
        Json,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ValueFormat {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ValueFormat {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ValueFormat {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Json => serializer.serialize_unit_variant("ValueFormat", 0u32, "JSON"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceConnectionPropertiesV2BasicResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: WorkspaceConnectionPropertiesV2,
}
impl WorkspaceConnectionPropertiesV2BasicResource {
    pub fn new(properties: WorkspaceConnectionPropertiesV2) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceConnectionPropertiesV2BasicResourceArmPaginatedResult {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkspaceConnectionPropertiesV2BasicResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkspaceConnectionPropertiesV2BasicResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkspaceConnectionPropertiesV2BasicResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceConnectionServicePrincipal {
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl WorkspaceConnectionServicePrincipal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceConnectionSharedAccessSignature {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sas: Option<String>,
}
impl WorkspaceConnectionSharedAccessSignature {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceConnectionUsernamePassword {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl WorkspaceConnectionUsernamePassword {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a request to list machine learning workspaces."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceListResult {
    #[doc = "The list of machine learning workspaces. Since this list may be incomplete, the nextLink field should be used to request the next list of machine learning workspaces."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Workspace>,
    #[doc = "The URI that can be used to request the next list of machine learning workspaces."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkspaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkspaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceProperties {
    #[doc = "The immutable id associated with this workspace."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The description of this workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The friendly name for this workspace. This name in mutable"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "ARM id of the key vault associated with this workspace. This cannot be changed once the workspace has been created"]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<String>,
    #[doc = "ARM id of the application insights associated with this workspace."]
    #[serde(rename = "applicationInsights", default, skip_serializing_if = "Option::is_none")]
    pub application_insights: Option<String>,
    #[doc = "ARM id of the container registry associated with this workspace."]
    #[serde(rename = "containerRegistry", default, skip_serializing_if = "Option::is_none")]
    pub container_registry: Option<String>,
    #[doc = "ARM id of the storage account associated with this workspace. This cannot be changed once the workspace has been created"]
    #[serde(rename = "storageAccount", default, skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<String>,
    #[doc = "Url for the discovery service to identify regional endpoints for machine learning experimentation services"]
    #[serde(rename = "discoveryUrl", default, skip_serializing_if = "Option::is_none")]
    pub discovery_url: Option<String>,
    #[doc = "The current deployment state of workspace resource. The provisioningState is to indicate states for resource provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workspace_properties::ProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionProperty>,
    #[doc = "The flag to signal HBI data in the workspace and reduce diagnostic data collected by the service"]
    #[serde(rename = "hbiWorkspace", default, skip_serializing_if = "Option::is_none")]
    pub hbi_workspace: Option<bool>,
    #[doc = "The name of the managed resource group created by workspace RP in customer subscription if the workspace is CMK workspace"]
    #[serde(rename = "serviceProvisionedResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub service_provisioned_resource_group: Option<String>,
    #[doc = "Count of private connections in the workspace"]
    #[serde(rename = "privateLinkCount", default, skip_serializing_if = "Option::is_none")]
    pub private_link_count: Option<i32>,
    #[doc = "The compute name for image build"]
    #[serde(rename = "imageBuildCompute", default, skip_serializing_if = "Option::is_none")]
    pub image_build_compute: Option<String>,
    #[doc = "The flag to indicate whether to allow public access when behind VNet."]
    #[serde(rename = "allowPublicAccessWhenBehindVnet", default, skip_serializing_if = "Option::is_none")]
    pub allow_public_access_when_behind_vnet: Option<bool>,
    #[doc = "Whether requests from Public Network are allowed."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<workspace_properties::PublicNetworkAccess>,
    #[doc = "The list of private endpoint connections in the workspace."]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "The list of shared private link resources in this workspace."]
    #[serde(
        rename = "sharedPrivateLinkResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub shared_private_link_resources: Vec<SharedPrivateLinkResource>,
    #[serde(rename = "notebookInfo", default, skip_serializing_if = "Option::is_none")]
    pub notebook_info: Option<NotebookResourceInfo>,
    #[serde(rename = "serviceManagedResourcesSettings", default, skip_serializing_if = "Option::is_none")]
    pub service_managed_resources_settings: Option<ServiceManagedResourcesSettings>,
    #[doc = "The user assigned identity resource id that represents the workspace identity."]
    #[serde(rename = "primaryUserAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub primary_user_assigned_identity: Option<String>,
    #[doc = "The tenant id associated with this workspace."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "If the storage associated with the workspace has hierarchical namespace(HNS) enabled."]
    #[serde(rename = "storageHnsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub storage_hns_enabled: Option<bool>,
    #[doc = "The URI associated with this workspace that machine learning flow must point at to set up tracking."]
    #[serde(rename = "mlFlowTrackingUri", default, skip_serializing_if = "Option::is_none")]
    pub ml_flow_tracking_uri: Option<String>,
    #[doc = "Enabling v1_legacy_mode may prevent you from using features provided by the v2 API."]
    #[serde(rename = "v1LegacyMode", default, skip_serializing_if = "Option::is_none")]
    pub v1_legacy_mode: Option<bool>,
    #[doc = "The timestamp when the workspace was soft deleted"]
    #[serde(rename = "softDeletedAt", default, skip_serializing_if = "Option::is_none")]
    pub soft_deleted_at: Option<String>,
    #[doc = "The timestamp when the soft deleted workspace is going to be purged"]
    #[serde(rename = "scheduledPurgeDate", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_purge_date: Option<String>,
    #[doc = "The auth mode used for accessing the system datastores of the workspace"]
    #[serde(rename = "systemDatastoresAuthMode", default, skip_serializing_if = "Option::is_none")]
    pub system_datastores_auth_mode: Option<String>,
    #[serde(rename = "featureStoreSettings", default, skip_serializing_if = "Option::is_none")]
    pub feature_store_settings: Option<FeatureStoreSettings>,
    #[doc = "Retention time in days after workspace get soft deleted."]
    #[serde(rename = "softDeleteRetentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_retention_in_days: Option<i32>,
    #[doc = "A flag to determine if workspace has data isolation enabled. The flag can only be set at the creation phase, it can't be updated."]
    #[serde(rename = "enableDataIsolation", default, skip_serializing_if = "Option::is_none")]
    pub enable_data_isolation: Option<bool>,
    #[doc = ": A list of storage accounts used by Hub."]
    #[serde(
        rename = "storageAccounts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub storage_accounts: Vec<String>,
    #[doc = "A list of key vaults used by Hub."]
    #[serde(
        rename = "keyVaults",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub key_vaults: Vec<String>,
    #[doc = "A list of container registries used by Hub."]
    #[serde(
        rename = "containerRegistries",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub container_registries: Vec<String>,
    #[doc = "A list of existing workspaces used by Hub to perform convert."]
    #[serde(
        rename = "existingWorkspaces",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub existing_workspaces: Vec<String>,
    #[doc = "Resource Id of Hub used for lean workspace."]
    #[serde(rename = "hubResourceId", default, skip_serializing_if = "Option::is_none")]
    pub hub_resource_id: Option<String>,
    #[doc = "A list of lean workspaces associated with Hub."]
    #[serde(
        rename = "associatedWorkspaces",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub associated_workspaces: Vec<String>,
    #[doc = "Managed Network settings for a machine learning workspace."]
    #[serde(rename = "managedNetwork", default, skip_serializing_if = "Option::is_none")]
    pub managed_network: Option<ManagedNetworkSettings>,
}
impl WorkspaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workspace_properties {
    use super::*;
    #[doc = "The current deployment state of workspace resource. The provisioningState is to indicate states for resource provisioning."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Unknown,
        Updating,
        Creating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
        SoftDeleted,
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
                Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Unknown"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Canceled"),
                Self::SoftDeleted => serializer.serialize_unit_variant("ProvisioningState", 7u32, "SoftDeleted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether requests from Public Network are allowed."]
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
#[doc = "The parameters for updating the properties of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspacePropertiesUpdateParameters {
    #[doc = "The description of this workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The friendly name for this workspace."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The compute name for image build"]
    #[serde(rename = "imageBuildCompute", default, skip_serializing_if = "Option::is_none")]
    pub image_build_compute: Option<String>,
    #[serde(rename = "serviceManagedResourcesSettings", default, skip_serializing_if = "Option::is_none")]
    pub service_managed_resources_settings: Option<ServiceManagedResourcesSettings>,
    #[doc = "The user assigned identity resource id that represents the workspace identity."]
    #[serde(rename = "primaryUserAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub primary_user_assigned_identity: Option<String>,
    #[doc = "Whether requests from Public Network are allowed."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<workspace_properties_update_parameters::PublicNetworkAccess>,
    #[doc = "ARM id of the application insights associated with this workspace."]
    #[serde(rename = "applicationInsights", default, skip_serializing_if = "Option::is_none")]
    pub application_insights: Option<String>,
    #[doc = "ARM id of the container registry associated with this workspace."]
    #[serde(rename = "containerRegistry", default, skip_serializing_if = "Option::is_none")]
    pub container_registry: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionUpdateProperties>,
    #[serde(rename = "featureStoreSettings", default, skip_serializing_if = "Option::is_none")]
    pub feature_store_settings: Option<FeatureStoreSettings>,
    #[doc = "Managed Network settings for a machine learning workspace."]
    #[serde(rename = "managedNetwork", default, skip_serializing_if = "Option::is_none")]
    pub managed_network: Option<ManagedNetworkSettings>,
}
impl WorkspacePropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workspace_properties_update_parameters {
    use super::*;
    #[doc = "Whether requests from Public Network are allowed."]
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
#[doc = "The parameters for updating a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceUpdateParameters {
    #[doc = "The resource tags for the machine learning workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "The parameters for updating the properties of a machine learning workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspacePropertiesUpdateParameters>,
}
impl WorkspaceUpdateParameters {
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

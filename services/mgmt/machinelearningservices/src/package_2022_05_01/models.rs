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
        #[serde(rename = "systemServices", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<AmlComputeNodeInformation>,
    #[doc = "The continuation token."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AmlComputeNodesInformation {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "allocationStateTransitionTime", with = "azure_core::date::rfc3339::option")]
    pub allocation_state_transition_time: Option<time::OffsetDateTime>,
    #[doc = "Collection of errors encountered by various compute nodes during node setup."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Azure Machine Learning workspace REST API operation"]
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
    #[doc = "List of AML workspace operations supported by the AML workspace resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetBase {
    #[serde(flatten)]
    pub resource_base: ResourceBase,
    #[doc = "If the name version are system generated (anonymous registration)."]
    #[serde(rename = "isAnonymous", default, skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[doc = "Is the asset archived?"]
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
#[doc = "Azure Blob datastore configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBlobDatastore {
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
            datastore,
            account_name,
            endpoint: None,
            filesystem,
            protocol: None,
            service_data_access_auth_identity: None,
        }
    }
}
#[doc = "Azure File datastore configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileDatastore {
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
            datastore,
            account_name,
            endpoint: None,
            file_share_name,
            protocol: None,
            service_data_access_auth_identity: None,
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
#[doc = "Batch inference settings per deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchDeployment {
    #[serde(flatten)]
    pub endpoint_deployment_properties_base: EndpointDeploymentPropertiesBase,
    #[doc = "Compute target for batch inference operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compute: Option<String>,
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
    pub resources: Option<ResourceConfiguration>,
    #[doc = "Retry settings for a batch inference operation."]
    #[serde(rename = "retrySettings", default, skip_serializing_if = "Option::is_none")]
    pub retry_settings: Option<BatchRetrySettings>,
}
impl BatchDeployment {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BatchDeploymentTrackedResource>,
}
impl azure_core::Continuable for BatchDeploymentTrackedResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BatchEndpointTrackedResource>,
}
impl azure_core::Continuable for BatchEndpointTrackedResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CodeContainerResource>,
}
impl azure_core::Continuable for CodeContainerResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CodeVersionResource>,
}
impl azure_core::Continuable for CodeVersionResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CodeVersionResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Command job definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandJob {
    #[serde(flatten)]
    pub job_base: JobBase,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceConfiguration>,
}
impl CommandJob {
    pub fn new(job_base: JobBase, command: String, environment_id: String) -> Self {
        Self {
            job_base,
            code_id: None,
            command,
            distribution: None,
            environment_id,
            environment_variables: None,
            inputs: None,
            limits: None,
            outputs: None,
            parameters: None,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ComponentContainerResource>,
}
impl azure_core::Continuable for ComponentContainerResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ComponentVersionResource>,
}
impl azure_core::Continuable for ComponentVersionResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "createdOn", with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "The time at which the compute was last modified."]
    #[serde(rename = "modifiedOn", with = "azure_core::date::rfc3339::option")]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "ARM resource id of the underlying compute"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Errors during provisioning"]
    #[serde(rename = "provisioningErrors", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "mountedOn", with = "azure_core::date::rfc3339::option")]
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
    #[serde(rename = "operationTime", with = "azure_core::date::rfc3339::option")]
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
    #[doc = "Specifies policy and settings for SSH access."]
    #[serde(rename = "sshSettings", default, skip_serializing_if = "Option::is_none")]
    pub ssh_settings: Option<ComputeInstanceSshSettings>,
    #[doc = "Defines all connectivity endpoints and properties for an ComputeInstance."]
    #[serde(rename = "connectivityEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub connectivity_endpoints: Option<ComputeInstanceConnectivityEndpoints>,
    #[doc = "Describes available applications and their endpoints on this ComputeInstance."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applications: Vec<ComputeInstanceApplication>,
    #[doc = "Describes information on user who created this ComputeInstance."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<ComputeInstanceCreatedBy>,
    #[doc = "Collection of errors encountered on this ComputeInstance."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[doc = "Enable or disable node public IP address provisioning. Possible values are: Possible values are: true - Indicates that the compute nodes will have public IPs provisioned. false - Indicates that the compute nodes will have a private endpoint and no public IPs."]
    #[serde(rename = "enableNodePublicIp", default, skip_serializing_if = "Option::is_none")]
    pub enable_node_public_ip: Option<bool>,
    #[doc = "Describes informations of containers on this ComputeInstance."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub containers: Vec<ComputeInstanceContainer>,
    #[doc = "Describes informations of dataDisks on this ComputeInstance."]
    #[serde(rename = "dataDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disks: Vec<ComputeInstanceDataDisk>,
    #[doc = "Describes informations of dataMounts on this ComputeInstance."]
    #[serde(rename = "dataMounts", default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "The compute power action."]
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
#[doc = "The list of schedules to be applied on the computes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeSchedules {
    #[doc = "The list of compute start stop schedules to be applied."]
    #[serde(rename = "computeStartStop", default, skip_serializing_if = "Vec::is_empty")]
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
    #[doc = "Schedule id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The current deployment state of schedule."]
    #[serde(rename = "provisioningStatus", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status: Option<compute_start_stop_schedule::ProvisioningStatus>,
    #[doc = "The compute power action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ComputePowerAction>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ContainerType")]
pub enum ContainerType {
    StorageInitializer,
    InferenceServer,
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
#[doc = "Enum to determine the datastore credentials type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CredentialsType")]
pub enum CredentialsType {
    AccountKey,
    Certificate,
    None,
    Sas,
    ServicePrincipal,
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
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataContainerResource>,
}
impl azure_core::Continuable for DataContainerResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataContainerResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "[Required] Uri of the data. Usage/meaning depends on Microsoft.MachineLearning.ManagementFrontEnd.Contracts.V20220501.Assets.DataVersionBase.DataType"]
    #[serde(rename = "dataUri")]
    pub data_uri: String,
}
impl DataVersionBase {
    pub fn new(data_type: DataType, data_uri: String) -> Self {
        Self {
            asset_base: AssetBase::default(),
            data_type,
            data_uri,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataVersionBaseResource>,
}
impl azure_core::Continuable for DataVersionBaseResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataVersionBaseResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatastoreResource>,
}
impl azure_core::Continuable for DatastoreResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
        #[serde(rename = "userDefinedRouteResults", default, skip_serializing_if = "Vec::is_empty")]
        pub user_defined_route_results: Vec<DiagnoseResult>,
        #[serde(rename = "networkSecurityRuleResults", default, skip_serializing_if = "Vec::is_empty")]
        pub network_security_rule_results: Vec<DiagnoseResult>,
        #[serde(rename = "resourceLockResults", default, skip_serializing_if = "Vec::is_empty")]
        pub resource_lock_results: Vec<DiagnoseResult>,
        #[serde(rename = "dnsResolutionResults", default, skip_serializing_if = "Vec::is_empty")]
        pub dns_resolution_results: Vec<DiagnoseResult>,
        #[serde(rename = "storageAccountResults", default, skip_serializing_if = "Vec::is_empty")]
        pub storage_account_results: Vec<DiagnoseResult>,
        #[serde(rename = "keyVaultResults", default, skip_serializing_if = "Vec::is_empty")]
        pub key_vault_results: Vec<DiagnoseResult>,
        #[serde(rename = "containerRegistryResults", default, skip_serializing_if = "Vec::is_empty")]
        pub container_registry_results: Vec<DiagnoseResult>,
        #[serde(rename = "applicationInsightsResults", default, skip_serializing_if = "Vec::is_empty")]
        pub application_insights_results: Vec<DiagnoseResult>,
        #[serde(rename = "otherResults", default, skip_serializing_if = "Vec::is_empty")]
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
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
    #[doc = "ARM resource ID or AssetId of the environment specification for the endpoint deployment."]
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
#[doc = "Container for environment specification versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentContainer {
    #[serde(flatten)]
    pub asset_container: AssetContainer,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EnvironmentContainerResource>,
}
impl azure_core::Continuable for EnvironmentContainerResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[doc = "Environment version details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentVersion {
    #[serde(flatten)]
    pub asset_base: AssetBase,
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
    #[doc = "The type of operating system."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OperatingSystemType>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EnvironmentVersionResource>,
}
impl azure_core::Continuable for EnvironmentVersionResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalFqdnResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "endpointDetails", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<FqdnEndpoint>,
}
impl FqdnEndpointsProperties {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Resource requests/limits for this instance type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstanceResourceSchema {}
impl InstanceResourceSchema {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Base definition for a job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobBase {
    #[serde(flatten)]
    pub resource_base: ResourceBase,
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
            compute_id: None,
            display_name: None,
            experiment_name: None,
            identity: None,
            is_archived: None,
            job_type,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobBaseResource>,
}
impl azure_core::Continuable for JobBaseResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[doc = "Port for endpoint."]
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
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enum to determine the type of job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "JobType")]
pub enum JobType {
    Command,
    Sweep,
    Pipeline,
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
            Self::Command => serializer.serialize_unit_variant("JobType", 0u32, "Command"),
            Self::Sweep => serializer.serialize_unit_variant("JobType", 1u32, "Sweep"),
            Self::Pipeline => serializer.serialize_unit_variant("JobType", 2u32, "Pipeline"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "The List Aml user feature operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListAmlUserFeatureResult {
    #[doc = "The list of AML user facing features."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AmlUserFeature>,
    #[doc = "The URI to fetch the next page of AML user features information. Call ListNext() with this to fetch the next page of AML user features information."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListAmlUserFeatureResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
    #[doc = "The URI to fetch the next page of AML resource usage information. Call ListNext() with this to fetch the next page of AML resource usage information."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListUsagesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceQuota>,
    #[doc = "The URI to fetch the next page of workspace quota information by VM Family. Call ListNext() with this to fetch the next page of Workspace Quota information."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListWorkspaceQuotas {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "referencedUris", default, skip_serializing_if = "Vec::is_empty")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelContainer {
    #[serde(flatten)]
    pub asset_container: AssetContainer,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ModelContainerResource>,
}
impl azure_core::Continuable for ModelContainerResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ModelContainerResourceArmPaginatedResult {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Name of the training job which produced this model"]
    #[serde(rename = "jobName", default, skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[doc = "The storage format for this entity. Used for NCD."]
    #[serde(rename = "modelType", default, skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    #[doc = "The URI path to the model contents."]
    #[serde(rename = "modelUri", default, skip_serializing_if = "Option::is_none")]
    pub model_uri: Option<String>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ModelVersionResource>,
}
impl azure_core::Continuable for ModelVersionResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ModelVersionResourceArmPaginatedResult {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnlineDeployment {
    #[serde(flatten)]
    pub endpoint_deployment_properties_base: EndpointDeploymentPropertiesBase,
    #[doc = "If true, enables Application Insights logging."]
    #[serde(rename = "appInsightsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_enabled: Option<bool>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OnlineDeploymentTrackedResource>,
}
impl azure_core::Continuable for OnlineDeploymentTrackedResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[doc = "State of endpoint provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<EndpointProvisioningState>,
    #[doc = "Percentage of traffic from endpoint to divert to each deployment. Traffic values need to sum to 100."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traffic: Option<serde_json::Value>,
}
impl OnlineEndpoint {
    pub fn new(endpoint_properties_base: EndpointPropertiesBase) -> Self {
        Self {
            endpoint_properties_base,
            compute: None,
            provisioning_state: None,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OnlineEndpointTrackedResource>,
}
impl azure_core::Continuable for OnlineEndpointTrackedResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OnlineEndpointTrackedResourceArmPaginatedResult {
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
#[doc = "Output data delivery mode enums."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OutputDeliveryMode")]
pub enum OutputDeliveryMode {
    ReadWriteMount,
    Upload,
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
#[doc = "Paginated list of Machine Learning compute objects wrapped in ARM resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedComputeResourcesList {
    #[doc = "An array of Machine Learning compute objects wrapped in ARM resource envelope."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ComputeResource>,
    #[doc = "A continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaginatedComputeResourcesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
}
impl PipelineJob {
    pub fn new(job_base: JobBase) -> Self {
        Self {
            job_base,
            inputs: None,
            jobs: None,
            outputs: None,
            settings: None,
        }
    }
}
#[doc = "The Private Endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for Private Endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ARM identifier for Subnet resource that private endpoint links to"]
    #[serde(rename = "subnetArmId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_arm_id: Option<String>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The private link resource Private link DNS zone name."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryListCredentialsResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub passwords: Vec<Password>,
}
impl RegistryListCredentialsResult {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "provisioningStatus", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status: Option<ScheduleProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ScheduleStatus>,
}
impl ScheduleBase {
    pub fn new() -> Self {
        Self::default()
    }
}
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
#[doc = "Enum to determine the datastore secrets type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SecretsType")]
pub enum SecretsType {
    AccountKey,
    Certificate,
    Sas,
    ServicePrincipal,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuResource>,
}
impl azure_core::Continuable for SkuResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resources: Option<ResourceConfiguration>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "supportedComputeTypes", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
}
impl Workspace {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkspaceConnectionPropertiesV2BasicResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkspaceConnectionPropertiesV2BasicResourceArmPaginatedResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkspaceConnectionPropertiesV2BasicResourceArmPaginatedResult {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
    #[doc = "The URI that can be used to request the next list of machine learning workspaces."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkspaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "The list of shared private link resources in this workspace."]
    #[serde(rename = "sharedPrivateLinkResources", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
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

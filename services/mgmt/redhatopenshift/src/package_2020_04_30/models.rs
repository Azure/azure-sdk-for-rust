#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "APIServerProfile represents an API server profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiServerProfile {
    #[doc = "Visibility represents visibility."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    #[doc = "The URL to access the cluster API server (immutable)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The IP of the cluster API server (immutable)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}
impl ApiServerProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CloudError represents a cloud error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "CloudErrorBody represents the body of a cloud error."]
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
#[doc = "CloudErrorBody represents the body of a cloud error."]
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
#[doc = "ClusterProfile represents a cluster profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterProfile {
    #[doc = "The pull secret for the cluster (immutable)."]
    #[serde(rename = "pullSecret", default, skip_serializing_if = "Option::is_none")]
    pub pull_secret: Option<String>,
    #[doc = "The domain for the cluster (immutable)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "The version of the cluster (immutable)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The ID of the cluster resource group (immutable)."]
    #[serde(rename = "resourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}
impl ClusterProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ConsoleProfile represents a console profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsoleProfile {
    #[doc = "The URL to access the cluster console (immutable)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ConsoleProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Display represents the display details of an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Display {
    #[doc = "Friendly name of the resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource type on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation type: read, write, delete, listKeys/action, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Friendly name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl Display {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IngressProfile represents an ingress profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IngressProfile {
    #[doc = "The ingress profile name.  Must be \"default\" (immutable)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Visibility represents visibility."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    #[doc = "The IP of the ingress (immutable)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}
impl IngressProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MasterProfile represents a master profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MasterProfile {
    #[doc = "VMSize represents a VM size."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<VmSize>,
    #[doc = "The Azure resource ID of the master subnet (immutable)."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}
impl MasterProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NetworkProfile represents a network profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfile {
    #[doc = "The CIDR used for OpenShift/Kubernetes Pods (immutable)."]
    #[serde(rename = "podCidr", default, skip_serializing_if = "Option::is_none")]
    pub pod_cidr: Option<String>,
    #[doc = "The CIDR used for OpenShift/Kubernetes Services (immutable)."]
    #[serde(rename = "serviceCidr", default, skip_serializing_if = "Option::is_none")]
    pub service_cidr: Option<String>,
}
impl NetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OpenShiftCluster represents an Azure Red Hat OpenShift cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenShiftCluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "OpenShiftClusterProperties represents an OpenShift cluster's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OpenShiftClusterProperties>,
}
impl OpenShiftCluster {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "OpenShiftClusterCredentials represents an OpenShift cluster's credentials"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenShiftClusterCredentials {
    #[doc = "The username for the kubeadmin user"]
    #[serde(rename = "kubeadminUsername", default, skip_serializing_if = "Option::is_none")]
    pub kubeadmin_username: Option<String>,
    #[doc = "The password for the kubeadmin user"]
    #[serde(rename = "kubeadminPassword", default, skip_serializing_if = "Option::is_none")]
    pub kubeadmin_password: Option<String>,
}
impl OpenShiftClusterCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OpenShiftClusterList represents a list of OpenShift clusters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenShiftClusterList {
    #[doc = "The list of OpenShift clusters."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OpenShiftCluster>,
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OpenShiftClusterList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OpenShiftClusterList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OpenShiftClusterProperties represents an OpenShift cluster's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenShiftClusterProperties {
    #[doc = "ProvisioningState represents a provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "ClusterProfile represents a cluster profile."]
    #[serde(rename = "clusterProfile", default, skip_serializing_if = "Option::is_none")]
    pub cluster_profile: Option<ClusterProfile>,
    #[doc = "ConsoleProfile represents a console profile."]
    #[serde(rename = "consoleProfile", default, skip_serializing_if = "Option::is_none")]
    pub console_profile: Option<ConsoleProfile>,
    #[doc = "ServicePrincipalProfile represents a service principal profile."]
    #[serde(rename = "servicePrincipalProfile", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_profile: Option<ServicePrincipalProfile>,
    #[doc = "NetworkProfile represents a network profile."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    #[doc = "MasterProfile represents a master profile."]
    #[serde(rename = "masterProfile", default, skip_serializing_if = "Option::is_none")]
    pub master_profile: Option<MasterProfile>,
    #[doc = "The cluster worker profiles."]
    #[serde(rename = "workerProfiles", default, skip_serializing_if = "Vec::is_empty")]
    pub worker_profiles: Vec<WorkerProfile>,
    #[doc = "APIServerProfile represents an API server profile."]
    #[serde(rename = "apiserverProfile", default, skip_serializing_if = "Option::is_none")]
    pub apiserver_profile: Option<ApiServerProfile>,
    #[doc = "The cluster ingress profiles."]
    #[serde(rename = "ingressProfiles", default, skip_serializing_if = "Vec::is_empty")]
    pub ingress_profiles: Vec<IngressProfile>,
}
impl OpenShiftClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OpenShiftCluster represents an Azure Red Hat OpenShift cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenShiftClusterUpdate {
    #[doc = "Tags represents an OpenShift cluster's tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "OpenShiftClusterProperties represents an OpenShift cluster's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OpenShiftClusterProperties>,
}
impl OpenShiftClusterUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation represents an RP operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display represents the display details of an operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Display>,
    #[doc = "Sources of requests to this operation.  Comma separated list with valid values user or system, e.g. \"user,system\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OperationList represents an RP operation list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of operations supported by the resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ProvisioningState represents a provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    AdminUpdating,
    Creating,
    Deleting,
    Failed,
    Succeeded,
    Updating,
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
#[doc = "ServicePrincipalProfile represents a service principal profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipalProfile {
    #[doc = "The client ID used for the cluster (immutable)."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The client secret used for the cluster (immutable)."]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}
impl ServicePrincipalProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tags represents an OpenShift cluster's tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {}
impl Tags {
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
#[doc = "VMSize represents a VM size."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VmSize {
    #[serde(rename = "Standard_D2s_v3")]
    StandardD2sV3,
    #[serde(rename = "Standard_D4s_v3")]
    StandardD4sV3,
    #[serde(rename = "Standard_D8s_v3")]
    StandardD8sV3,
}
#[doc = "Visibility represents visibility."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Visibility {
    Private,
    Public,
}
#[doc = "WorkerProfile represents a worker profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkerProfile {
    #[doc = "The worker profile name.  Must be \"worker\" (immutable)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "VMSize represents a VM size."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<VmSize>,
    #[doc = "The disk size of the worker VMs.  Must be 128 or greater (immutable)."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i64>,
    #[doc = "The Azure resource ID of the worker subnet (immutable)."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "The number of worker VMs.  Must be between 3 and 20 (immutable)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl WorkerProfile {
    pub fn new() -> Self {
        Self::default()
    }
}

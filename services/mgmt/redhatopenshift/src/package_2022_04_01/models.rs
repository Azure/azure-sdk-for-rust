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
    #[doc = "The URL to access the cluster API server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The IP of the cluster API server."]
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
    #[doc = "The pull secret for the cluster."]
    #[serde(rename = "pullSecret", default, skip_serializing_if = "Option::is_none")]
    pub pull_secret: Option<String>,
    #[doc = "The domain for the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "The version of the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The ID of the cluster resource group."]
    #[serde(rename = "resourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    #[doc = "FipsValidatedModules determines if FIPS is used."]
    #[serde(rename = "fipsValidatedModules", default, skip_serializing_if = "Option::is_none")]
    pub fips_validated_modules: Option<FipsValidatedModules>,
}
impl ClusterProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ConsoleProfile represents a console profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsoleProfile {
    #[doc = "The URL to access the cluster console."]
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
#[doc = "EncryptionAtHost represents encryption at host state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EncryptionAtHost")]
pub enum EncryptionAtHost {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EncryptionAtHost {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EncryptionAtHost {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EncryptionAtHost {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("EncryptionAtHost", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("EncryptionAtHost", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "FipsValidatedModules determines if FIPS is used."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FipsValidatedModules")]
pub enum FipsValidatedModules {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FipsValidatedModules {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FipsValidatedModules {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FipsValidatedModules {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("FipsValidatedModules", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("FipsValidatedModules", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "IngressProfile represents an ingress profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IngressProfile {
    #[doc = "The ingress profile name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Visibility represents visibility."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    #[doc = "The IP of the ingress."]
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
    #[doc = "VM size availability varies by region.\nIf a node contains insufficient compute resources (memory, cpu, etc.), pods might fail to run correctly.\nFor more details on restricted VM sizes, see: https://docs.microsoft.com/en-us/azure/openshift/support-policies-v4#supported-virtual-machine-sizes"]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<VmSize>,
    #[doc = "The Azure resource ID of the master subnet."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "EncryptionAtHost represents encryption at host state"]
    #[serde(rename = "encryptionAtHost", default, skip_serializing_if = "Option::is_none")]
    pub encryption_at_host: Option<EncryptionAtHost>,
    #[doc = "The resource ID of an associated DiskEncryptionSet, if applicable."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
}
impl MasterProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NetworkProfile represents a network profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfile {
    #[doc = "The CIDR used for OpenShift/Kubernetes Pods."]
    #[serde(rename = "podCidr", default, skip_serializing_if = "Option::is_none")]
    pub pod_cidr: Option<String>,
    #[doc = "The CIDR used for OpenShift/Kubernetes Services."]
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl OpenShiftCluster {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "OpenShiftClusterAdminKubeconfig represents an OpenShift cluster's admin kubeconfig."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenShiftClusterAdminKubeconfig {
    #[doc = "The base64-encoded kubeconfig file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubeconfig: Option<String>,
}
impl OpenShiftClusterAdminKubeconfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OpenShiftClusterCredentials represents an OpenShift cluster's credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenShiftClusterCredentials {
    #[doc = "The username for the kubeadmin user."]
    #[serde(rename = "kubeadminUsername", default, skip_serializing_if = "Option::is_none")]
    pub kubeadmin_username: Option<String>,
    #[doc = "The password for the kubeadmin user."]
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
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
    #[doc = "The client ID used for the cluster."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The client secret used for the cluster."]
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
pub type VmSize = String;
#[doc = "Visibility represents visibility."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Visibility")]
pub enum Visibility {
    Private,
    Public,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Visibility {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Visibility {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Visibility {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Private => serializer.serialize_unit_variant("Visibility", 0u32, "Private"),
            Self::Public => serializer.serialize_unit_variant("Visibility", 1u32, "Public"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "WorkerProfile represents a worker profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkerProfile {
    #[doc = "The worker profile name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "VM size availability varies by region.\nIf a node contains insufficient compute resources (memory, cpu, etc.), pods might fail to run correctly.\nFor more details on restricted VM sizes, see: https://docs.microsoft.com/en-us/azure/openshift/support-policies-v4#supported-virtual-machine-sizes"]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<VmSize>,
    #[doc = "The disk size of the worker VMs."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i64>,
    #[doc = "The Azure resource ID of the worker subnet."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "The number of worker VMs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "EncryptionAtHost represents encryption at host state"]
    #[serde(rename = "encryptionAtHost", default, skip_serializing_if = "Option::is_none")]
    pub encryption_at_host: Option<EncryptionAtHost>,
    #[doc = "The resource ID of an associated DiskEncryptionSet, if applicable."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
}
impl WorkerProfile {
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

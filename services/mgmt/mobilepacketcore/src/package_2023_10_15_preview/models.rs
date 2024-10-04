#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Azure for Operators 5G Core Access and Mobility Function (AMF) Deployment Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfDeploymentResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "AMF Deployment Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AmfDeploymentResourceProperties>,
}
impl AmfDeploymentResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a AmfDeploymentResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfDeploymentResourceListResult {
    #[doc = "The AmfDeploymentResource items on this page"]
    pub value: Vec<AmfDeploymentResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AmfDeploymentResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AmfDeploymentResourceListResult {
    pub fn new(value: Vec<AmfDeploymentResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "AMF Deployment Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfDeploymentResourceProperties {
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Containerized Network Function (CNF) Component Deployment Parameters"]
    #[serde(rename = "componentParameters", default, skip_serializing_if = "Option::is_none")]
    pub component_parameters: Option<ComponentDeploymentParameters>,
    #[doc = "Containerized Network Function (CNF) Component Secrets"]
    #[serde(rename = "secretsParameters", default, skip_serializing_if = "Option::is_none")]
    pub secrets_parameters: Option<ComponentDeploymentSecrets>,
    #[doc = "ARM resource ID of a cluster."]
    #[serde(rename = "clusterService")]
    pub cluster_service: ClusterServiceIdProperty,
    #[doc = "Azure for Operators 5G Core Release Version."]
    #[serde(rename = "releaseVersion", default, skip_serializing_if = "Option::is_none")]
    pub release_version: Option<ReleaseVersion>,
    #[doc = "Operational Status of the resource"]
    #[serde(rename = "operationalStatus", default, skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<OperationalStatus>,
}
impl AmfDeploymentResourceProperties {
    pub fn new(cluster_service: ClusterServiceIdProperty) -> Self {
        Self {
            provisioning_state: None,
            component_parameters: None,
            secrets_parameters: None,
            cluster_service,
            release_version: None,
            operational_status: None,
        }
    }
}
#[doc = "The type used for updating tags in AmfDeploymentResource resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmfDeploymentResourceTagsUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AmfDeploymentResourceTagsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AKS Cluster specific data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterServiceAksClusterData {
    #[serde(flatten)]
    pub cluster_service_cluster_type_specific_data: ClusterServiceClusterTypeSpecificData,
}
impl ClusterServiceAksClusterData {
    pub fn new(cluster_service_cluster_type_specific_data: ClusterServiceClusterTypeSpecificData) -> Self {
        Self {
            cluster_service_cluster_type_specific_data,
        }
    }
}
#[doc = "Cluster Service cluster type specific data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterServiceClusterTypeSpecificData {
    #[doc = "ARM resource ID of a custom location."]
    #[serde(rename = "customLocationId")]
    pub custom_location_id: CustomLocationIdProperty,
}
impl ClusterServiceClusterTypeSpecificData {
    pub fn new(custom_location_id: CustomLocationIdProperty) -> Self {
        Self { custom_location_id }
    }
}
#[doc = "Cluster Type Definitions"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClusterServiceClusterTypeSpecificDataUnion {
    Aks(ClusterServiceAksClusterData),
    NexusAks(ClusterServiceNexusAksClusterData),
}
pub type ClusterServiceIdProperty = String;
#[doc = "Nexus AKS Cluster specific data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterServiceNexusAksClusterData {
    #[serde(flatten)]
    pub cluster_service_cluster_type_specific_data: ClusterServiceClusterTypeSpecificData,
}
impl ClusterServiceNexusAksClusterData {
    pub fn new(cluster_service_cluster_type_specific_data: ClusterServiceClusterTypeSpecificData) -> Self {
        Self {
            cluster_service_cluster_type_specific_data,
        }
    }
}
#[doc = "Azure for Operators 5G Core Cluster Service Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterServiceResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Cluster Service Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterServiceResourceProperties>,
}
impl ClusterServiceResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a ClusterServiceResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterServiceResourceListResult {
    #[doc = "The ClusterServiceResource items on this page"]
    pub value: Vec<ClusterServiceResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ClusterServiceResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ClusterServiceResourceListResult {
    pub fn new(value: Vec<ClusterServiceResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Cluster Service Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterServiceResourceProperties {
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Deployment Type Definitions"]
    #[serde(rename = "deploymentType")]
    pub deployment_type: SkuDeploymentType,
    #[doc = "Azure for Operators 5G Core Release Version."]
    #[serde(rename = "releaseVersion")]
    pub release_version: ReleaseVersion,
    #[doc = "Cluster Service cluster type specific data."]
    #[serde(rename = "clusterTypeSpecificData")]
    pub cluster_type_specific_data: ClusterServiceClusterTypeSpecificDataUnion,
    #[doc = "Azure for Operators 5G Core Local PaaS component parameters.  One set per component type"]
    #[serde(rename = "componentParameters")]
    pub component_parameters: Vec<QualifiedComponentDeploymentParameters>,
    #[doc = "Operational Status of the resource"]
    #[serde(rename = "operationalStatus", default, skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<OperationalStatus>,
}
impl ClusterServiceResourceProperties {
    pub fn new(
        deployment_type: SkuDeploymentType,
        release_version: ReleaseVersion,
        cluster_type_specific_data: ClusterServiceClusterTypeSpecificDataUnion,
        component_parameters: Vec<QualifiedComponentDeploymentParameters>,
    ) -> Self {
        Self {
            provisioning_state: None,
            deployment_type,
            release_version,
            cluster_type_specific_data,
            component_parameters,
            operational_status: None,
        }
    }
}
#[doc = "The type used for updating tags in ClusterServiceResource resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterServiceResourceTagsUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ClusterServiceResourceTagsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cluster Type Definitions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ClusterType")]
pub enum ClusterType {
    Aks,
    NexusAks,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ClusterType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ClusterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ClusterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Aks => serializer.serialize_unit_variant("ClusterType", 0u32, "Aks"),
            Self::NexusAks => serializer.serialize_unit_variant("ClusterType", 1u32, "NexusAks"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type ComponentDeploymentParameters = String;
pub type ComponentDeploymentSecrets = String;
pub type ComponentDeploymentType = String;
pub type CustomLocationIdProperty = String;
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
#[doc = "Infrastructure Type Definitions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InfrastructureType")]
pub enum InfrastructureType {
    AzureCore,
    AzureOperatorNexus,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InfrastructureType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InfrastructureType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InfrastructureType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureCore => serializer.serialize_unit_variant("InfrastructureType", 0u32, "AzureCore"),
            Self::AzureOperatorNexus => serializer.serialize_unit_variant("InfrastructureType", 1u32, "AzureOperatorNexus"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "ManagedResourceGroupConfiguration represents the configuration of the resource group managed by Azure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedResourceGroupConfiguration {
    #[doc = "The name for the managed resource group. If not specified, the unique name is automatically generated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The location of the managed resource group. If not specified, the location of the parent resource is chosen."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl ManagedResourceGroupConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type MobileCoreScalingUnitSku = String;
#[doc = "Azure for Operators 5G Core Network Repository Function (NRF) Deployment Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NrfDeploymentResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "NRF Deployment Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NrfDeploymentResourceProperties>,
}
impl NrfDeploymentResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a NrfDeploymentResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NrfDeploymentResourceListResult {
    #[doc = "The NrfDeploymentResource items on this page"]
    pub value: Vec<NrfDeploymentResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NrfDeploymentResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NrfDeploymentResourceListResult {
    pub fn new(value: Vec<NrfDeploymentResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "NRF Deployment Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NrfDeploymentResourceProperties {
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Containerized Network Function (CNF) Component Deployment Parameters"]
    #[serde(rename = "componentParameters", default, skip_serializing_if = "Option::is_none")]
    pub component_parameters: Option<ComponentDeploymentParameters>,
    #[doc = "Containerized Network Function (CNF) Component Secrets"]
    #[serde(rename = "secretsParameters", default, skip_serializing_if = "Option::is_none")]
    pub secrets_parameters: Option<ComponentDeploymentSecrets>,
    #[doc = "ARM resource ID of a cluster."]
    #[serde(rename = "clusterService")]
    pub cluster_service: ClusterServiceIdProperty,
    #[doc = "Azure for Operators 5G Core Release Version."]
    #[serde(rename = "releaseVersion", default, skip_serializing_if = "Option::is_none")]
    pub release_version: Option<ReleaseVersion>,
    #[doc = "Operational Status of the resource"]
    #[serde(rename = "operationalStatus", default, skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<OperationalStatus>,
}
impl NrfDeploymentResourceProperties {
    pub fn new(cluster_service: ClusterServiceIdProperty) -> Self {
        Self {
            provisioning_state: None,
            component_parameters: None,
            secrets_parameters: None,
            cluster_service,
            release_version: None,
            operational_status: None,
        }
    }
}
#[doc = "The type used for updating tags in NrfDeploymentResource resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NrfDeploymentResourceTagsUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl NrfDeploymentResourceTagsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure for Operators 5G Core Network Slice Selection Function (NSSF) Deployment Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NssfDeploymentResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "NSSF Deployment Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NssfDeploymentResourceProperties>,
}
impl NssfDeploymentResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a NssfDeploymentResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NssfDeploymentResourceListResult {
    #[doc = "The NssfDeploymentResource items on this page"]
    pub value: Vec<NssfDeploymentResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NssfDeploymentResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NssfDeploymentResourceListResult {
    pub fn new(value: Vec<NssfDeploymentResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "NSSF Deployment Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NssfDeploymentResourceProperties {
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Containerized Network Function (CNF) Component Deployment Parameters"]
    #[serde(rename = "componentParameters", default, skip_serializing_if = "Option::is_none")]
    pub component_parameters: Option<ComponentDeploymentParameters>,
    #[doc = "Containerized Network Function (CNF) Component Secrets"]
    #[serde(rename = "secretsParameters", default, skip_serializing_if = "Option::is_none")]
    pub secrets_parameters: Option<ComponentDeploymentSecrets>,
    #[doc = "ARM resource ID of a cluster."]
    #[serde(rename = "clusterService")]
    pub cluster_service: ClusterServiceIdProperty,
    #[doc = "Azure for Operators 5G Core Release Version."]
    #[serde(rename = "releaseVersion", default, skip_serializing_if = "Option::is_none")]
    pub release_version: Option<ReleaseVersion>,
    #[doc = "Operational Status of the resource"]
    #[serde(rename = "operationalStatus", default, skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<OperationalStatus>,
}
impl NssfDeploymentResourceProperties {
    pub fn new(cluster_service: ClusterServiceIdProperty) -> Self {
        Self {
            provisioning_state: None,
            component_parameters: None,
            secrets_parameters: None,
            cluster_service,
            release_version: None,
            operational_status: None,
        }
    }
}
#[doc = "The type used for updating tags in NssfDeploymentResource resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NssfDeploymentResourceTagsUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl NssfDeploymentResourceTagsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure for Operators 5G Core Observability Service Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservabilityServiceResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Observability Service Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ObservabilityServiceResourceProperties>,
}
impl ObservabilityServiceResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a ObservabilityServiceResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservabilityServiceResourceListResult {
    #[doc = "The ObservabilityServiceResource items on this page"]
    pub value: Vec<ObservabilityServiceResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ObservabilityServiceResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ObservabilityServiceResourceListResult {
    pub fn new(value: Vec<ObservabilityServiceResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Observability Service Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservabilityServiceResourceProperties {
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Azure for Operators 5G Core Observability component parameters.  One set per component type"]
    #[serde(rename = "componentParameters")]
    pub component_parameters: Vec<QualifiedComponentDeploymentParameters>,
    #[doc = "ARM resource ID of a cluster."]
    #[serde(rename = "clusterService")]
    pub cluster_service: ClusterServiceIdProperty,
    #[doc = "Azure for Operators 5G Core Release Version."]
    #[serde(rename = "releaseVersion", default, skip_serializing_if = "Option::is_none")]
    pub release_version: Option<ReleaseVersion>,
    #[doc = "Operational Status of the resource"]
    #[serde(rename = "operationalStatus", default, skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<OperationalStatus>,
}
impl ObservabilityServiceResourceProperties {
    pub fn new(component_parameters: Vec<QualifiedComponentDeploymentParameters>, cluster_service: ClusterServiceIdProperty) -> Self {
        Self {
            provisioning_state: None,
            component_parameters,
            cluster_service,
            release_version: None,
            operational_status: None,
        }
    }
}
#[doc = "The type used for updating tags in ObservabilityServiceResource resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ObservabilityServiceResourceTagsUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ObservabilityServiceResourceTagsUpdate {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Operational Status of the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationalStatus {
    #[doc = "Status of the deployed workload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workload: Option<String>,
    #[doc = "Health check results"]
    #[serde(rename = "healthCheck", default, skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
}
impl OperationalStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provisioning state of the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Provisioning,
    Updating,
    Deleting,
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
            Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Accepted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Containerized Network Function (CNF) Qualified Deployment Parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QualifiedComponentDeploymentParameters {
    #[doc = "Containerized Network Function (CNF) Component Deployment Type"]
    #[serde(rename = "type")]
    pub type_: ComponentDeploymentType,
    #[doc = "Containerized Network Function (CNF) Component Deployment Parameters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ComponentDeploymentParameters>,
    #[doc = "Containerized Network Function (CNF) Component Secrets"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<ComponentDeploymentSecrets>,
}
impl QualifiedComponentDeploymentParameters {
    pub fn new(type_: ComponentDeploymentType) -> Self {
        Self {
            type_,
            parameters: None,
            secrets: None,
        }
    }
}
pub type ReleaseVersion = String;
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
#[doc = "Deployment Type Definitions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SkuDeploymentType")]
pub enum SkuDeploymentType {
    Production,
    Lab,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SkuDeploymentType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SkuDeploymentType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SkuDeploymentType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Production => serializer.serialize_unit_variant("SkuDeploymentType", 0u32, "Production"),
            Self::Lab => serializer.serialize_unit_variant("SkuDeploymentType", 1u32, "Lab"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Azure for Operators 5G Core Session Management Function (SMF) Deployment Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmfDeploymentResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "SMF Deployment Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SmfDeploymentResourceProperties>,
}
impl SmfDeploymentResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a SmfDeploymentResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmfDeploymentResourceListResult {
    #[doc = "The SmfDeploymentResource items on this page"]
    pub value: Vec<SmfDeploymentResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SmfDeploymentResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SmfDeploymentResourceListResult {
    pub fn new(value: Vec<SmfDeploymentResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "SMF Deployment Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmfDeploymentResourceProperties {
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Containerized Network Function (CNF) Component Deployment Parameters"]
    #[serde(rename = "componentParameters", default, skip_serializing_if = "Option::is_none")]
    pub component_parameters: Option<ComponentDeploymentParameters>,
    #[doc = "Containerized Network Function (CNF) Component Secrets"]
    #[serde(rename = "secretsParameters", default, skip_serializing_if = "Option::is_none")]
    pub secrets_parameters: Option<ComponentDeploymentSecrets>,
    #[doc = "ARM resource ID of a cluster."]
    #[serde(rename = "clusterService")]
    pub cluster_service: ClusterServiceIdProperty,
    #[doc = "Azure for Operators 5G Core Release Version."]
    #[serde(rename = "releaseVersion", default, skip_serializing_if = "Option::is_none")]
    pub release_version: Option<ReleaseVersion>,
    #[doc = "Operational Status of the resource"]
    #[serde(rename = "operationalStatus", default, skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<OperationalStatus>,
}
impl SmfDeploymentResourceProperties {
    pub fn new(cluster_service: ClusterServiceIdProperty) -> Self {
        Self {
            provisioning_state: None,
            component_parameters: None,
            secrets_parameters: None,
            cluster_service,
            release_version: None,
            operational_status: None,
        }
    }
}
#[doc = "The type used for updating tags in SmfDeploymentResource resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SmfDeploymentResourceTagsUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SmfDeploymentResourceTagsUpdate {
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
#[doc = "Azure for Operators 5G Core User Plane Function (UPF) Deployment Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpfDeploymentResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "UPF Deployment Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpfDeploymentResourceProperties>,
}
impl UpfDeploymentResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a UpfDeploymentResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpfDeploymentResourceListResult {
    #[doc = "The UpfDeploymentResource items on this page"]
    pub value: Vec<UpfDeploymentResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UpfDeploymentResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl UpfDeploymentResourceListResult {
    pub fn new(value: Vec<UpfDeploymentResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "UPF Deployment Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpfDeploymentResourceProperties {
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Containerized Network Function (CNF) Component Deployment Parameters"]
    #[serde(rename = "componentParameters")]
    pub component_parameters: ComponentDeploymentParameters,
    #[doc = "Containerized Network Function (CNF) Component Secrets"]
    #[serde(rename = "secretsParameters", default, skip_serializing_if = "Option::is_none")]
    pub secrets_parameters: Option<ComponentDeploymentSecrets>,
    #[doc = "ARM resource ID of a cluster."]
    #[serde(rename = "clusterService")]
    pub cluster_service: ClusterServiceIdProperty,
    #[doc = "Azure for Operators 5G Core Release Version."]
    #[serde(rename = "releaseVersion", default, skip_serializing_if = "Option::is_none")]
    pub release_version: Option<ReleaseVersion>,
    #[doc = "Operational Status of the resource"]
    #[serde(rename = "operationalStatus", default, skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<OperationalStatus>,
}
impl UpfDeploymentResourceProperties {
    pub fn new(component_parameters: ComponentDeploymentParameters, cluster_service: ClusterServiceIdProperty) -> Self {
        Self {
            provisioning_state: None,
            component_parameters,
            secrets_parameters: None,
            cluster_service,
            release_version: None,
            operational_status: None,
        }
    }
}
#[doc = "The type used for updating tags in UpfDeploymentResource resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpfDeploymentResourceTagsUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl UpfDeploymentResourceTagsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type UsageSku = String;
#[doc = "API Versions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Versions")]
pub enum Versions {
    #[serde(rename = "2023-10-15-preview")]
    N2023_10_15_preview,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Versions {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Versions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Versions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::N2023_10_15_preview => serializer.serialize_unit_variant("Versions", 0u32, "2023-10-15-preview"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type VnfAgentDeploymentIdProperty = String;
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

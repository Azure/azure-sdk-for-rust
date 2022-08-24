#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The RP custom operation error info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AffectedMoveResource {
    #[doc = "The affected move resource id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The affected move resource source id."]
    #[serde(rename = "sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[doc = "The affected move resources."]
    #[serde(rename = "moveResources", default, skip_serializing_if = "Vec::is_empty")]
    pub move_resources: Vec<AffectedMoveResource>,
}
impl AffectedMoveResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the properties for automatic resolution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomaticResolutionProperties {
    #[doc = "Gets the MoveResource ARM ID of\r\nthe dependent resource if the resolution type is Automatic."]
    #[serde(rename = "moveResourceId", default, skip_serializing_if = "Option::is_none")]
    pub move_resource_id: Option<String>,
}
impl AutomaticResolutionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the availability set resource settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailabilitySetResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets the target fault domain."]
    #[serde(rename = "faultDomain", default, skip_serializing_if = "Option::is_none")]
    pub fault_domain: Option<i32>,
    #[doc = "Gets or sets the target update domain."]
    #[serde(rename = "updateDomain", default, skip_serializing_if = "Option::is_none")]
    pub update_domain: Option<i32>,
}
impl AvailabilitySetResourceSettings {
    pub fn new(resource_settings: ResourceSettings) -> Self {
        Self {
            resource_settings,
            tags: None,
            fault_domain: None,
            update_domain: None,
        }
    }
}
#[doc = "Defines reference to an Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceReference {
    #[doc = "Gets the ARM resource ID of the tracked resource being referenced."]
    #[serde(rename = "sourceArmResourceId")]
    pub source_arm_resource_id: String,
}
impl AzureResourceReference {
    pub fn new(source_arm_resource_id: String) -> Self {
        Self { source_arm_resource_id }
    }
}
#[doc = "Defines the request body for bulk remove of move resources operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BulkRemoveRequest {
    #[doc = "Gets or sets a value indicating whether the operation needs to only run pre-requisite."]
    #[serde(rename = "validateOnly", default, skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
    #[doc = "Gets or sets the list of resource Id's, by default it accepts move resource id's unless the input type is switched via moveResourceInputType property."]
    #[serde(rename = "moveResources", default, skip_serializing_if = "Vec::is_empty")]
    pub move_resources: Vec<String>,
    #[doc = "Defines the move resource input type."]
    #[serde(rename = "moveResourceInputType", default, skip_serializing_if = "Option::is_none")]
    pub move_resource_input_type: Option<MoveResourceInputType>,
}
impl BulkRemoveRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the service."]
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
#[doc = "An error response from the service."]
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
#[doc = "Defines the request body for commit operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitRequest {
    #[doc = "Gets or sets a value indicating whether the operation needs to only run pre-requisite."]
    #[serde(rename = "validateOnly", default, skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
    #[doc = "Gets or sets the list of resource Id's, by default it accepts move resource id's unless the input type is switched via moveResourceInputType property."]
    #[serde(rename = "moveResources")]
    pub move_resources: Vec<String>,
    #[doc = "Defines the move resource input type."]
    #[serde(rename = "moveResourceInputType", default, skip_serializing_if = "Option::is_none")]
    pub move_resource_input_type: Option<MoveResourceInputType>,
}
impl CommitRequest {
    pub fn new(move_resources: Vec<String>) -> Self {
        Self {
            validate_only: None,
            move_resources,
            move_resource_input_type: None,
        }
    }
}
#[doc = "Defines the dependency type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DependencyType")]
pub enum DependencyType {
    RequiredForPrepare,
    RequiredForMove,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DependencyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DependencyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DependencyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::RequiredForPrepare => serializer.serialize_unit_variant("DependencyType", 0u32, "RequiredForPrepare"),
            Self::RequiredForMove => serializer.serialize_unit_variant("DependencyType", 1u32, "RequiredForMove"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the request body for discard operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscardRequest {
    #[doc = "Gets or sets a value indicating whether the operation needs to only run pre-requisite."]
    #[serde(rename = "validateOnly", default, skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
    #[doc = "Gets or sets the list of resource Id's, by default it accepts move resource id's unless the input type is switched via moveResourceInputType property."]
    #[serde(rename = "moveResources")]
    pub move_resources: Vec<String>,
    #[doc = "Defines the move resource input type."]
    #[serde(rename = "moveResourceInputType", default, skip_serializing_if = "Option::is_none")]
    pub move_resource_input_type: Option<MoveResourceInputType>,
}
impl DiscardRequest {
    pub fn new(move_resources: Vec<String>) -> Self {
        Self {
            validate_only: None,
            move_resources,
            move_resource_input_type: None,
        }
    }
}
#[doc = "Defines the disk encryption set resource settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskEncryptionSetResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
}
impl DiskEncryptionSetResourceSettings {
    pub fn new(resource_settings: ResourceSettings) -> Self {
        Self { resource_settings }
    }
}
#[doc = "Contains the localized display information for this particular operation / action. These\r\nvalue will be used by several clients for\r\n(1) custom role definitions for RBAC;\r\n(2) complex query filters for the event service; and\r\n(3) audit history / records for management operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Display {
    #[doc = "Gets or sets the provider.\r\nThe localized friendly form of the resource provider name – it is expected to also\r\ninclude the publisher/company responsible.\r\nIt should use Title Casing and begin with \"Microsoft\" for 1st party services.\r\ne.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute.\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Gets or sets the resource.\r\nThe localized friendly form of the resource related to this action/operation – it\r\nshould match the public documentation for the resource provider.\r\nIt should use Title Casing.\r\nThis value should be unique for a particular URL type (e.g. nested types should *not*\r\nreuse their parent’s display.resource field)\r\ne.g. \"Virtual Machines\" or \"Scheduler Job Collections\", or \"Virtual Machine VM Sizes\"\r\nor \"Scheduler Jobs\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Gets or sets the operation.\r\nThe localized friendly name for the operation, as it should be shown to the user.\r\nIt should be concise (to fit in drop downs) but clear (i.e. self-documenting).\r\nIt should use Title Casing.\r\nPrescriptive guidance: Read Create or Update Delete 'ActionName'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Gets or sets the description.\r\nThe localized friendly description for the operation, as it should be shown to the\r\nuser.\r\nIt should be thorough, yet concise – it will be used in tool tips and detailed views.\r\nPrescriptive guidance for namespace:\r\nRead any 'display.provider'  resource\r\nCreate or Update any  'display.provider'  resource\r\nDelete any  'display.provider'  resource\r\nPerform any other action on any  'display.provider'  resource\r\nPrescriptive guidance for namespace:\r\nRead any 'display.resource' Create or Update any  'display.resource' Delete any\r\n 'display.resource' 'ActionName' any 'display.resources'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl Display {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the MSI properties of the Move Collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "The type of identity used for the resource mover service."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ResourceIdentityType>,
    #[doc = "Gets or sets the principal id."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Gets or sets the tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the job name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "JobName")]
pub enum JobName {
    InitialSync,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for JobName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for JobName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for JobName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::InitialSync => serializer.serialize_unit_variant("JobName", 0u32, "InitialSync"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the job status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStatus {
    #[doc = "Defines the job name."]
    #[serde(rename = "jobName", default, skip_serializing_if = "Option::is_none")]
    pub job_name: Option<JobName>,
    #[doc = "Gets or sets the monitoring job percentage."]
    #[serde(rename = "jobProgress", default, skip_serializing_if = "Option::is_none")]
    pub job_progress: Option<String>,
}
impl JobStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the key vault resource settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
}
impl KeyVaultResourceSettings {
    pub fn new(resource_settings: ResourceSettings) -> Self {
        Self { resource_settings }
    }
}
#[doc = "Defines load balancer backend address pool properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LbBackendAddressPoolResourceSettings {
    #[doc = "Gets or sets the backend address pool name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl LbBackendAddressPoolResourceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines load balancer frontend IP configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LbFrontendIpConfigurationResourceSettings {
    #[doc = "Gets or sets the frontend IP configuration name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the IP address of the Load Balancer.This is only specified if a specific\r\nprivate IP address shall be allocated from the subnet specified in subnetRef."]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "Gets or sets PrivateIP allocation method (Static/Dynamic)."]
    #[serde(rename = "privateIpAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_allocation_method: Option<String>,
    #[doc = "Defines reference to subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<SubnetReference>,
    #[doc = "Gets or sets the csv list of zones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<String>,
}
impl LbFrontendIpConfigurationResourceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines reference to load balancer backend address pools."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerBackendAddressPoolReference {
    #[serde(flatten)]
    pub proxy_resource_reference: ProxyResourceReference,
}
impl LoadBalancerBackendAddressPoolReference {
    pub fn new(proxy_resource_reference: ProxyResourceReference) -> Self {
        Self { proxy_resource_reference }
    }
}
#[doc = "Defines reference to load balancer NAT rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerNatRuleReference {
    #[serde(flatten)]
    pub proxy_resource_reference: ProxyResourceReference,
}
impl LoadBalancerNatRuleReference {
    pub fn new(proxy_resource_reference: ProxyResourceReference) -> Self {
        Self { proxy_resource_reference }
    }
}
#[doc = "Defines the load balancer resource settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets load balancer sku (Basic/Standard)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "Gets or sets the frontend IP configurations of the load balancer."]
    #[serde(rename = "frontendIPConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub frontend_ip_configurations: Vec<LbFrontendIpConfigurationResourceSettings>,
    #[doc = "Gets or sets the backend address pools of the load balancer."]
    #[serde(rename = "backendAddressPools", default, skip_serializing_if = "Vec::is_empty")]
    pub backend_address_pools: Vec<LbBackendAddressPoolResourceSettings>,
    #[doc = "Gets or sets the csv list of zones common for all frontend IP configurations. Note this is given\r\n precedence only if frontend IP configurations settings are not present."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<String>,
}
impl LoadBalancerResourceSettings {
    pub fn new(resource_settings: ResourceSettings) -> Self {
        Self {
            resource_settings,
            tags: None,
            sku: None,
            frontend_ip_configurations: Vec::new(),
            backend_address_pools: Vec::new(),
            zones: None,
        }
    }
}
#[doc = "Defines the properties for manual resolution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManualResolutionProperties {
    #[doc = "Gets or sets the target resource ARM ID of the dependent resource if the resource type is Manual."]
    #[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}
impl ManualResolutionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Define the move collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveCollection {
    #[doc = "Fully qualified resource Id for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The etag of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Defines the MSI properties of the Move Collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Defines the move collection properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MoveCollectionProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl MoveCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the move collection properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveCollectionProperties {
    #[doc = "Gets or sets the source region."]
    #[serde(rename = "sourceRegion")]
    pub source_region: String,
    #[doc = "Gets or sets the target region."]
    #[serde(rename = "targetRegion")]
    pub target_region: String,
    #[doc = "Defines the provisioning states."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Defines the move collection errors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
}
impl MoveCollectionProperties {
    pub fn new(source_region: String, target_region: String) -> Self {
        Self {
            source_region,
            target_region,
            provisioning_state: None,
            errors: None,
        }
    }
}
#[doc = "Defines the collection of move collections."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveCollectionResultList {
    #[doc = "Gets the list of move collections."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MoveCollection>,
    #[doc = "Gets the value of  next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MoveCollectionResultList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MoveCollectionResultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The move custom error info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveErrorInfo {
    #[doc = "The affected move resources."]
    #[serde(rename = "moveResources", default, skip_serializing_if = "Vec::is_empty")]
    pub move_resources: Vec<AffectedMoveResource>,
}
impl MoveErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the move resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveResource {
    #[doc = "Fully qualified resource Id for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Defines the move resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MoveResourceProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl MoveResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the collection of move resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveResourceCollection {
    #[doc = "Gets the list of move resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MoveResource>,
    #[doc = "Gets the value of  next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Summary Collection."]
    #[serde(rename = "summaryCollection", default, skip_serializing_if = "Option::is_none")]
    pub summary_collection: Option<SummaryCollection>,
    #[doc = "Gets the total count."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}
impl azure_core::Continuable for MoveResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MoveResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the dependency of the move resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveResourceDependency {
    #[doc = "Gets the source ARM ID of the dependent resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the dependency resolution status."]
    #[serde(rename = "resolutionStatus", default, skip_serializing_if = "Option::is_none")]
    pub resolution_status: Option<String>,
    #[doc = "Defines the resolution type."]
    #[serde(rename = "resolutionType", default, skip_serializing_if = "Option::is_none")]
    pub resolution_type: Option<ResolutionType>,
    #[doc = "Defines the dependency type."]
    #[serde(rename = "dependencyType", default, skip_serializing_if = "Option::is_none")]
    pub dependency_type: Option<DependencyType>,
    #[doc = "Defines the properties for manual resolution."]
    #[serde(rename = "manualResolution", default, skip_serializing_if = "Option::is_none")]
    pub manual_resolution: Option<ManualResolutionProperties>,
    #[doc = "Defines the properties for automatic resolution."]
    #[serde(rename = "automaticResolution", default, skip_serializing_if = "Option::is_none")]
    pub automatic_resolution: Option<AutomaticResolutionProperties>,
    #[doc = "Gets or sets a value indicating whether the dependency is optional."]
    #[serde(rename = "isOptional", default, skip_serializing_if = "Option::is_none")]
    pub is_optional: Option<String>,
}
impl MoveResourceDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the dependency override of the move resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveResourceDependencyOverride {
    #[doc = "Gets or sets the ARM ID of the dependent resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the resource ARM id of either the MoveResource or the resource ARM ID of\r\nthe dependent resource."]
    #[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}
impl MoveResourceDependencyOverride {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the azure resource mover service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveResourceError {
    #[doc = "An error response from the Azure Migrate service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MoveResourceErrorBody>,
}
impl MoveResourceError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the Azure Migrate service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveResourceErrorBody {
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
    pub details: Vec<MoveResourceErrorBody>,
}
impl MoveResourceErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Move resource filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveResourceFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MoveResourceFilterProperties>,
}
impl MoveResourceFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveResourceFilterProperties {
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl MoveResourceFilterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the move resource input type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MoveResourceInputType")]
pub enum MoveResourceInputType {
    MoveResourceId,
    MoveResourceSourceId,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MoveResourceInputType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MoveResourceInputType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MoveResourceInputType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::MoveResourceId => serializer.serialize_unit_variant("MoveResourceInputType", 0u32, "MoveResourceId"),
            Self::MoveResourceSourceId => serializer.serialize_unit_variant("MoveResourceInputType", 1u32, "MoveResourceSourceId"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the move resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveResourceProperties {
    #[doc = "Defines the provisioning states."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Gets or sets the Source ARM Id of the resource."]
    #[serde(rename = "sourceId")]
    pub source_id: String,
    #[doc = "Gets or sets the Target ARM Id of the resource."]
    #[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[doc = "Gets or sets the existing target ARM Id of the resource."]
    #[serde(rename = "existingTargetId", default, skip_serializing_if = "Option::is_none")]
    pub existing_target_id: Option<String>,
    #[doc = "Gets or sets the resource settings."]
    #[serde(rename = "resourceSettings", default, skip_serializing_if = "Option::is_none")]
    pub resource_settings: Option<ResourceSettings>,
    #[doc = "Gets or sets the resource settings."]
    #[serde(rename = "sourceResourceSettings", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_settings: Option<ResourceSettings>,
    #[doc = "Defines the move resource status."]
    #[serde(rename = "moveStatus", default, skip_serializing_if = "Option::is_none")]
    pub move_status: Option<serde_json::Value>,
    #[doc = "Gets or sets the move resource dependencies."]
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<MoveResourceDependency>,
    #[doc = "Gets or sets the move resource dependencies overrides."]
    #[serde(rename = "dependsOnOverrides", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on_overrides: Vec<MoveResourceDependencyOverride>,
    #[doc = "Gets a value indicating whether the resolve action is required over the move collection."]
    #[serde(rename = "isResolveRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_resolve_required: Option<bool>,
    #[doc = "Defines the move resource errors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
}
impl MoveResourceProperties {
    pub fn new(source_id: String) -> Self {
        Self {
            provisioning_state: None,
            source_id,
            target_id: None,
            existing_target_id: None,
            resource_settings: None,
            source_resource_settings: None,
            move_status: None,
            depends_on: Vec::new(),
            depends_on_overrides: Vec::new(),
            is_resolve_required: None,
            errors: None,
        }
    }
}
#[doc = "Defines the move resource status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveResourceStatus {
    #[doc = "Defines the MoveResource states."]
    #[serde(rename = "moveState", default, skip_serializing_if = "Option::is_none")]
    pub move_state: Option<MoveState>,
    #[doc = "Defines the job status."]
    #[serde(rename = "jobStatus", default, skip_serializing_if = "Option::is_none")]
    pub job_status: Option<JobStatus>,
    #[doc = "An error response from the azure resource mover service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<MoveResourceError>,
}
impl MoveResourceStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the MoveResource states."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MoveState")]
pub enum MoveState {
    AssignmentPending,
    PreparePending,
    PrepareInProgress,
    PrepareFailed,
    MovePending,
    MoveInProgress,
    MoveFailed,
    DiscardInProgress,
    DiscardFailed,
    CommitPending,
    CommitInProgress,
    CommitFailed,
    Committed,
    DeleteSourcePending,
    ResourceMoveCompleted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MoveState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MoveState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MoveState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AssignmentPending => serializer.serialize_unit_variant("MoveState", 0u32, "AssignmentPending"),
            Self::PreparePending => serializer.serialize_unit_variant("MoveState", 1u32, "PreparePending"),
            Self::PrepareInProgress => serializer.serialize_unit_variant("MoveState", 2u32, "PrepareInProgress"),
            Self::PrepareFailed => serializer.serialize_unit_variant("MoveState", 3u32, "PrepareFailed"),
            Self::MovePending => serializer.serialize_unit_variant("MoveState", 4u32, "MovePending"),
            Self::MoveInProgress => serializer.serialize_unit_variant("MoveState", 5u32, "MoveInProgress"),
            Self::MoveFailed => serializer.serialize_unit_variant("MoveState", 6u32, "MoveFailed"),
            Self::DiscardInProgress => serializer.serialize_unit_variant("MoveState", 7u32, "DiscardInProgress"),
            Self::DiscardFailed => serializer.serialize_unit_variant("MoveState", 8u32, "DiscardFailed"),
            Self::CommitPending => serializer.serialize_unit_variant("MoveState", 9u32, "CommitPending"),
            Self::CommitInProgress => serializer.serialize_unit_variant("MoveState", 10u32, "CommitInProgress"),
            Self::CommitFailed => serializer.serialize_unit_variant("MoveState", 11u32, "CommitFailed"),
            Self::Committed => serializer.serialize_unit_variant("MoveState", 12u32, "Committed"),
            Self::DeleteSourcePending => serializer.serialize_unit_variant("MoveState", 13u32, "DeleteSourcePending"),
            Self::ResourceMoveCompleted => serializer.serialize_unit_variant("MoveState", 14u32, "ResourceMoveCompleted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the network interface resource settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkInterfaceResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets the IP configurations of the NIC."]
    #[serde(rename = "ipConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_configurations: Vec<NicIpConfigurationResourceSettings>,
    #[doc = "Gets or sets a value indicating whether accelerated networking is enabled."]
    #[serde(rename = "enableAcceleratedNetworking", default, skip_serializing_if = "Option::is_none")]
    pub enable_accelerated_networking: Option<bool>,
}
impl NetworkInterfaceResourceSettings {
    pub fn new(resource_settings: ResourceSettings) -> Self {
        Self {
            resource_settings,
            tags: None,
            ip_configurations: Vec::new(),
            enable_accelerated_networking: None,
        }
    }
}
#[doc = "Defines the NSG resource settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkSecurityGroupResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets Security rules of network security group."]
    #[serde(rename = "securityRules", default, skip_serializing_if = "Vec::is_empty")]
    pub security_rules: Vec<NsgSecurityRule>,
}
impl NetworkSecurityGroupResourceSettings {
    pub fn new(resource_settings: ResourceSettings) -> Self {
        Self {
            resource_settings,
            tags: None,
            security_rules: Vec::new(),
        }
    }
}
#[doc = "Defines NIC IP configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NicIpConfigurationResourceSettings {
    #[doc = "Gets or sets the IP configuration name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the private IP address of the network interface IP Configuration."]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "Gets or sets the private IP address allocation method."]
    #[serde(rename = "privateIpAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_allocation_method: Option<String>,
    #[doc = "Defines reference to subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<SubnetReference>,
    #[doc = "Gets or sets a value indicating whether this IP configuration is the primary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[doc = "Gets or sets the references of the load balancer backend address pools."]
    #[serde(rename = "loadBalancerBackendAddressPools", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_backend_address_pools: Vec<LoadBalancerBackendAddressPoolReference>,
    #[doc = "Gets or sets the references of the load balancer NAT rules."]
    #[serde(rename = "loadBalancerNatRules", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_nat_rules: Vec<LoadBalancerNatRuleReference>,
    #[doc = "Defines reference to a public IP."]
    #[serde(rename = "publicIp", default, skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<PublicIpReference>,
}
impl NicIpConfigurationResourceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines reference to NSG."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NsgReference {
    #[serde(flatten)]
    pub azure_resource_reference: AzureResourceReference,
}
impl NsgReference {
    pub fn new(azure_resource_reference: AzureResourceReference) -> Self {
        Self { azure_resource_reference }
    }
}
#[doc = "Security Rule data model for Network Security Groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NsgSecurityRule {
    #[doc = "Gets or sets the Security rule name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets whether network traffic is allowed or denied.\r\nPossible values are “Allow” and “Deny”."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[doc = "Gets or sets a description for this rule. Restricted to 140 chars."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets destination address prefix. CIDR or source IP range.\r\n A “*” can also be used to match all source IPs. Default tags such\r\nas ‘VirtualNetwork’, ‘AzureLoadBalancer’ and ‘Internet’ can also be used."]
    #[serde(rename = "destinationAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub destination_address_prefix: Option<String>,
    #[doc = "Gets or sets Destination Port or Range. Integer or range between\r\n0 and 65535. A “*” can also be used to match all ports."]
    #[serde(rename = "destinationPortRange", default, skip_serializing_if = "Option::is_none")]
    pub destination_port_range: Option<String>,
    #[doc = "Gets or sets the direction of the rule.InBound or Outbound. The\r\ndirection specifies if rule will be evaluated on incoming or outgoing traffic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[doc = "Gets or sets the priority of the rule. The value can be between\r\n100 and 4096. The priority number must be unique for each rule in the collection.\r\nThe lower the priority number, the higher the priority of the rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "Gets or sets Network protocol this rule applies to. Can be Tcp, Udp or All(*)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[doc = "Gets or sets source address prefix. CIDR or source IP range. A\r\n“*” can also be used to match all source IPs.  Default tags such as ‘VirtualNetwork’,\r\n‘AzureLoadBalancer’ and ‘Internet’ can also be used. If this is an ingress\r\nrule, specifies where network traffic originates from."]
    #[serde(rename = "sourceAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub source_address_prefix: Option<String>,
    #[doc = "Gets or sets Source Port or Range. Integer or range between 0 and\r\n65535. A “*” can also be used to match all ports."]
    #[serde(rename = "sourcePortRange", default, skip_serializing_if = "Option::is_none")]
    pub source_port_range: Option<String>,
}
impl NsgSecurityRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operation error info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationErrorAdditionalInfo {
    #[doc = "The error type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The move custom error info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<MoveErrorInfo>,
}
impl OperationErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation status REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Status of the operation. ARM expects the terminal status to be one of Succeeded/ Failed/ Canceled. All other values imply that the operation is still running."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Start time."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "End time."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "Class for operation status errors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationStatusError>,
    #[doc = "Class for operation result properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationStatusProperties>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for operation status errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatusError {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<OperationStatusError>,
    #[doc = "The additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<OperationErrorAdditionalInfo>,
}
impl OperationStatusError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for operation result properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatusProperties {}
impl OperationStatusProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operations discovery class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsDiscovery {
    #[doc = "Gets or sets Name of the API.\r\nThe name of the operation being performed on this particular object. It should\r\nmatch the action name that appears in RBAC / the event service.\r\nExamples of operations include:\r\n* Microsoft.Compute/virtualMachine/capture/action\r\n* Microsoft.Compute/virtualMachine/restart/action\r\n* Microsoft.Compute/virtualMachine/write\r\n* Microsoft.Compute/virtualMachine/read\r\n* Microsoft.Compute/virtualMachine/delete\r\nEach action should include, in order:\r\n(1) Resource Provider Namespace\r\n(2) Type hierarchy for which the action applies (e.g. server/databases for a SQL\r\nAzure database)\r\n(3) Read, Write, Action or Delete indicating which type applies. If it is a PUT/PATCH\r\non a collection or named value, Write should be used.\r\nIf it is a GET, Read should be used. If it is a DELETE, Delete should be used. If it\r\nis a POST, Action should be used.\r\nAs a note: all resource providers would need to include the \"{Resource Provider\r\nNamespace}/register/action\" operation in their response.\r\nThis API is used to register for their service, and should include details about the\r\noperation (e.g. a localized name for the resource provider + any special\r\nconsiderations like PII release)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Contains the localized display information for this particular operation / action. These\r\nvalue will be used by several clients for\r\n(1) custom role definitions for RBAC;\r\n(2) complex query filters for the event service; and\r\n(3) audit history / records for management operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Display>,
    #[doc = "Gets or sets Origin.\r\nThe intended executor of the operation; governs the display of the operation in the\r\nRBAC UX and the audit logs UX.\r\nDefault value is \"user,system\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "ClientDiscovery properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationsDiscoveryProperties>,
}
impl OperationsDiscovery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of ClientDiscovery details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsDiscoveryCollection {
    #[doc = "Gets or sets the ClientDiscovery details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationsDiscovery>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationsDiscoveryCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ClientDiscovery properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsDiscoveryProperties {}
impl OperationsDiscoveryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the request body for initiate prepare operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrepareRequest {
    #[doc = "Gets or sets a value indicating whether the operation needs to only run pre-requisite."]
    #[serde(rename = "validateOnly", default, skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
    #[doc = "Gets or sets the list of resource Id's, by default it accepts move resource id's unless the input type is switched via moveResourceInputType property."]
    #[serde(rename = "moveResources")]
    pub move_resources: Vec<String>,
    #[doc = "Defines the move resource input type."]
    #[serde(rename = "moveResourceInputType", default, skip_serializing_if = "Option::is_none")]
    pub move_resource_input_type: Option<MoveResourceInputType>,
}
impl PrepareRequest {
    pub fn new(move_resources: Vec<String>) -> Self {
        Self {
            validate_only: None,
            move_resources,
            move_resource_input_type: None,
        }
    }
}
#[doc = "Defines the provisioning states."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Updating,
    Creating,
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
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Creating"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines reference to a proxy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResourceReference {
    #[serde(flatten)]
    pub azure_resource_reference: AzureResourceReference,
    #[doc = "Gets the name of the proxy resource on the target side."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ProxyResourceReference {
    pub fn new(azure_resource_reference: AzureResourceReference) -> Self {
        Self {
            azure_resource_reference,
            name: None,
        }
    }
}
#[doc = "Defines the public IP address resource settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicIpAddressResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets the domain name label."]
    #[serde(rename = "domainNameLabel", default, skip_serializing_if = "Option::is_none")]
    pub domain_name_label: Option<String>,
    #[doc = "Gets or sets the fully qualified domain name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Gets or sets public IP allocation method."]
    #[serde(rename = "publicIpAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_allocation_method: Option<String>,
    #[doc = "Gets or sets public IP sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "Gets or sets public IP zones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<String>,
}
impl PublicIpAddressResourceSettings {
    pub fn new(resource_settings: ResourceSettings) -> Self {
        Self {
            resource_settings,
            tags: None,
            domain_name_label: None,
            fqdn: None,
            public_ip_allocation_method: None,
            sku: None,
            zones: None,
        }
    }
}
#[doc = "Defines reference to a public IP."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicIpReference {
    #[serde(flatten)]
    pub azure_resource_reference: AzureResourceReference,
}
impl PublicIpReference {
    pub fn new(azure_resource_reference: AzureResourceReference) -> Self {
        Self { azure_resource_reference }
    }
}
#[doc = "Required for resources collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequiredForResourcesCollection {
    #[doc = "Gets or sets the list of source Ids for which the input resource is required."]
    #[serde(rename = "sourceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub source_ids: Vec<String>,
}
impl RequiredForResourcesCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resolution type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResolutionType")]
pub enum ResolutionType {
    Manual,
    Automatic,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResolutionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResolutionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResolutionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Manual => serializer.serialize_unit_variant("ResolutionType", 0u32, "Manual"),
            Self::Automatic => serializer.serialize_unit_variant("ResolutionType", 1u32, "Automatic"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the resource group resource settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroupResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
}
impl ResourceGroupResourceSettings {
    pub fn new(resource_settings: ResourceSettings) -> Self {
        Self { resource_settings }
    }
}
#[doc = "The type of identity used for the resource mover service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceIdentityType")]
pub enum ResourceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ResourceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ResourceIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ResourceIdentityType", 2u32, "UserAssigned"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the request body for resource move operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceMoveRequest {
    #[doc = "Gets or sets a value indicating whether the operation needs to only run pre-requisite."]
    #[serde(rename = "validateOnly", default, skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
    #[doc = "Gets or sets the list of resource Id's, by default it accepts move resource id's unless the input type is switched via moveResourceInputType property."]
    #[serde(rename = "moveResources")]
    pub move_resources: Vec<String>,
    #[doc = "Defines the move resource input type."]
    #[serde(rename = "moveResourceInputType", default, skip_serializing_if = "Option::is_none")]
    pub move_resource_input_type: Option<MoveResourceInputType>,
}
impl ResourceMoveRequest {
    pub fn new(move_resources: Vec<String>) -> Self {
        Self {
            validate_only: None,
            move_resources,
            move_resource_input_type: None,
        }
    }
}
#[doc = "Gets or sets the resource settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSettings {
    #[doc = "The resource type. For example, the value can be Microsoft.Compute/virtualMachines."]
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[doc = "Gets or sets the target Resource name."]
    #[serde(rename = "targetResourceName")]
    pub target_resource_name: String,
}
impl ResourceSettings {
    pub fn new(resource_type: String, target_resource_name: String) -> Self {
        Self {
            resource_type,
            target_resource_name,
        }
    }
}
#[doc = "Defines the Sql Database resource settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDatabaseResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Defines the zone redundant resource setting."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<ZoneRedundant>,
}
impl SqlDatabaseResourceSettings {
    pub fn new(resource_settings: ResourceSettings) -> Self {
        Self {
            resource_settings,
            tags: None,
            zone_redundant: None,
        }
    }
}
#[doc = "Defines the Sql ElasticPool resource settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlElasticPoolResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Defines the zone redundant resource setting."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<ZoneRedundant>,
}
impl SqlElasticPoolResourceSettings {
    pub fn new(resource_settings: ResourceSettings) -> Self {
        Self {
            resource_settings,
            tags: None,
            zone_redundant: None,
        }
    }
}
#[doc = "Defines the SQL Server resource settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
}
impl SqlServerResourceSettings {
    pub fn new(resource_settings: ResourceSettings) -> Self {
        Self { resource_settings }
    }
}
#[doc = "Defines reference to subnet."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubnetReference {
    #[serde(flatten)]
    pub proxy_resource_reference: ProxyResourceReference,
}
impl SubnetReference {
    pub fn new(proxy_resource_reference: ProxyResourceReference) -> Self {
        Self { proxy_resource_reference }
    }
}
#[doc = "Defines the virtual network subnets resource settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubnetResourceSettings {
    #[doc = "Gets or sets the Subnet name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets address prefix for the subnet."]
    #[serde(rename = "addressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub address_prefix: Option<String>,
    #[doc = "Defines reference to NSG."]
    #[serde(rename = "networkSecurityGroup", default, skip_serializing_if = "Option::is_none")]
    pub network_security_group: Option<NsgReference>,
}
impl SubnetResourceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summary item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Summary {
    #[doc = "Gets the count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Gets the item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
}
impl Summary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summary Collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SummaryCollection {
    #[doc = "Gets or sets the field name on which summary is done."]
    #[serde(rename = "fieldName", default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[doc = "Gets or sets the list of summary items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub summary: Vec<Summary>,
}
impl SummaryCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Unresolved dependencies contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnresolvedDependenciesFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UnresolvedDependenciesFilterProperties>,
}
impl UnresolvedDependenciesFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnresolvedDependenciesFilterProperties {
    #[doc = "The count of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
impl UnresolvedDependenciesFilterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Unresolved dependency."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnresolvedDependency {
    #[doc = "Gets or sets the count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Gets or sets the arm id of the dependency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl UnresolvedDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Unresolved dependency collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnresolvedDependencyCollection {
    #[doc = "Gets or sets the list of unresolved dependencies."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UnresolvedDependency>,
    #[doc = "Gets or sets the value of  next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Summary Collection."]
    #[serde(rename = "summaryCollection", default, skip_serializing_if = "Option::is_none")]
    pub summary_collection: Option<SummaryCollection>,
    #[doc = "Gets the total count."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}
impl azure_core::Continuable for UnresolvedDependencyCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UnresolvedDependencyCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the request body for updating move collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateMoveCollectionRequest {
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Defines the MSI properties of the Move Collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl UpdateMoveCollectionRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the virtual machine resource settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets user-managed identities"]
    #[serde(rename = "userManagedIdentities", default, skip_serializing_if = "Vec::is_empty")]
    pub user_managed_identities: Vec<String>,
    #[doc = "Gets or sets the target availability zone."]
    #[serde(rename = "targetAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_zone: Option<virtual_machine_resource_settings::TargetAvailabilityZone>,
    #[doc = "Gets or sets the target virtual machine size."]
    #[serde(rename = "targetVmSize", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_size: Option<String>,
    #[doc = "Gets or sets the target availability set id for virtual machines not in an availability set at source."]
    #[serde(rename = "targetAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_set_id: Option<String>,
}
impl VirtualMachineResourceSettings {
    pub fn new(resource_settings: ResourceSettings) -> Self {
        Self {
            resource_settings,
            tags: None,
            user_managed_identities: Vec::new(),
            target_availability_zone: None,
            target_vm_size: None,
            target_availability_set_id: None,
        }
    }
}
pub mod virtual_machine_resource_settings {
    use super::*;
    #[doc = "Gets or sets the target availability zone."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TargetAvailabilityZone")]
    pub enum TargetAvailabilityZone {
        #[serde(rename = "1")]
        N1,
        #[serde(rename = "2")]
        N2,
        #[serde(rename = "3")]
        N3,
        #[serde(rename = "NA")]
        Na,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TargetAvailabilityZone {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TargetAvailabilityZone {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TargetAvailabilityZone {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N1 => serializer.serialize_unit_variant("TargetAvailabilityZone", 0u32, "1"),
                Self::N2 => serializer.serialize_unit_variant("TargetAvailabilityZone", 1u32, "2"),
                Self::N3 => serializer.serialize_unit_variant("TargetAvailabilityZone", 2u32, "3"),
                Self::Na => serializer.serialize_unit_variant("TargetAvailabilityZone", 3u32, "NA"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the virtual network resource settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets a value indicating whether gets or sets whether the\r\nDDOS protection should be switched on."]
    #[serde(rename = "enableDdosProtection", default, skip_serializing_if = "Option::is_none")]
    pub enable_ddos_protection: Option<bool>,
    #[doc = "Gets or sets the address prefixes for the virtual network."]
    #[serde(rename = "addressSpace", default, skip_serializing_if = "Vec::is_empty")]
    pub address_space: Vec<String>,
    #[doc = "Gets or sets DHCPOptions that contains an array of DNS servers available to VMs\r\ndeployed in the virtual network."]
    #[serde(rename = "dnsServers", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_servers: Vec<String>,
    #[doc = "Gets or sets List of subnets in a VirtualNetwork."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<SubnetResourceSettings>,
}
impl VirtualNetworkResourceSettings {
    pub fn new(resource_settings: ResourceSettings) -> Self {
        Self {
            resource_settings,
            tags: None,
            enable_ddos_protection: None,
            address_space: Vec::new(),
            dns_servers: Vec::new(),
            subnets: Vec::new(),
        }
    }
}
#[doc = "Defines the zone redundant resource setting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ZoneRedundant")]
pub enum ZoneRedundant {
    Enable,
    Disable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ZoneRedundant {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ZoneRedundant {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ZoneRedundant {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enable => serializer.serialize_unit_variant("ZoneRedundant", 0u32, "Enable"),
            Self::Disable => serializer.serialize_unit_variant("ZoneRedundant", 1u32, "Disable"),
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

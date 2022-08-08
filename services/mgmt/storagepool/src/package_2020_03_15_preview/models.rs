#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Access Control List (ACL) for an iSCSI target portal group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Acl {
    #[doc = "iSCSI initiator IQN (iSCSI Qualified Name); example: \"iqn.2005-03.org.iscsi:client\"."]
    #[serde(rename = "initiatorIqn")]
    pub initiator_iqn: String,
    #[doc = "List of LUN names mapped to the ACL."]
    #[serde(rename = "mappedLuns")]
    pub mapped_luns: Vec<String>,
    #[doc = "Challenge Handshake Authentication Protocol (CHAP) credentials for an iSCSI target ACL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<IscsiTargetCredentials>,
}
impl Acl {
    pub fn new(initiator_iqn: String, mapped_luns: Vec<String>) -> Self {
        Self {
            initiator_iqn,
            mapped_luns,
            credentials: None,
        }
    }
}
pub type AdditionalCapability = String;
#[doc = "Attributes of a iSCSI target portal group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    #[doc = "Indicates whether or not authentication is enabled on the ACL."]
    pub authentication: bool,
    #[doc = "Indicates whether or not write protect is enabled on the LUNs."]
    #[serde(rename = "prodModeWriteProtect")]
    pub prod_mode_write_protect: bool,
}
impl Attributes {
    pub fn new(authentication: bool, prod_mode_write_protect: bool) -> Self {
        Self {
            authentication,
            prod_mode_write_protect,
        }
    }
}
pub type AvailabilityZone = String;
#[doc = "Azure Managed Disk to attach to the Disk pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Disk {
    #[doc = "Unique Azure Resource ID of the Managed Disk."]
    pub id: String,
}
impl Disk {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Response for Disk pool request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Disk pool response properties."]
    pub properties: DiskPoolProperties,
    #[doc = "Resource metadata required by ARM RPC."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemMetadata>,
}
impl DiskPool {
    pub fn new(tracked_resource: TrackedResource, properties: DiskPoolProperties) -> Self {
        Self {
            tracked_resource,
            properties,
            system_data: None,
        }
    }
}
#[doc = "Request payload for create or update Disk pool request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolCreate {
    #[doc = "Properties for Disk pool create or update request."]
    pub properties: DiskPoolCreateProperties,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives."]
    pub location: String,
    #[doc = "Fully qualified resource Id for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. Ex- Microsoft.Compute/virtualMachines or Microsoft.Storage/storageAccounts."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl DiskPoolCreate {
    pub fn new(properties: DiskPoolCreateProperties, location: String) -> Self {
        Self {
            properties,
            tags: None,
            location,
            id: None,
            name: None,
            type_: None,
        }
    }
}
#[doc = "Properties for Disk pool create or update request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolCreateProperties {
    #[doc = "Logical zone for Disk pool resource; example: [\"1\"]."]
    #[serde(rename = "availabilityZones")]
    pub availability_zones: Vec<AvailabilityZone>,
    #[doc = "List of Azure Managed Disks to attach to a Disk pool. Can attach 8 disks at most."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<Disk>,
    #[doc = "Azure Resource ID of a Subnet for the Disk pool."]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[doc = "List of additional capabilities for a Disk pool."]
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_capabilities: Vec<AdditionalCapability>,
    #[doc = "SKU of the VM host part of the Disk pool deployment"]
    pub tier: DiskPoolTier,
}
impl DiskPoolCreateProperties {
    pub fn new(availability_zones: Vec<AvailabilityZone>, subnet_id: String, tier: DiskPoolTier) -> Self {
        Self {
            availability_zones,
            disks: Vec::new(),
            subnet_id,
            additional_capabilities: Vec::new(),
            tier,
        }
    }
}
#[doc = "List of Disk Pools"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolListResult {
    #[doc = "An array of Disk pool objects."]
    pub value: Vec<DiskPool>,
    #[doc = "URI to fetch the next section of the paginated response."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiskPoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DiskPoolListResult {
    pub fn new(value: Vec<DiskPool>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Disk pool response properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolProperties {
    #[doc = "Provisioning state of the iSCSI target."]
    #[serde(rename = "provisioningState")]
    pub provisioning_state: ProvisioningState,
    #[doc = "Logical zone for Disk pool resource; example: [\"1\"]."]
    #[serde(rename = "availabilityZones")]
    pub availability_zones: Vec<AvailabilityZone>,
    #[doc = "Operational status of the resource."]
    pub status: OperationalStatus,
    #[doc = "List of Azure Managed Disks to attach to a Disk pool. Can attach 8 disks at most."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<Disk>,
    #[doc = "Azure Resource ID of a Subnet for the Disk pool."]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[doc = "List of additional capabilities for Disk pool."]
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_capabilities: Vec<AdditionalCapability>,
    #[doc = "SKU of the VM host part of the Disk pool deployment"]
    pub tier: DiskPoolTier,
}
impl DiskPoolProperties {
    pub fn new(
        provisioning_state: ProvisioningState,
        availability_zones: Vec<AvailabilityZone>,
        status: OperationalStatus,
        subnet_id: String,
        tier: DiskPoolTier,
    ) -> Self {
        Self {
            provisioning_state,
            availability_zones,
            status,
            disks: Vec::new(),
            subnet_id,
            additional_capabilities: Vec::new(),
            tier,
        }
    }
}
#[doc = "SKU of the VM host part of the Disk pool deployment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DiskPoolTier")]
pub enum DiskPoolTier {
    Basic,
    Standard,
    Premium,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DiskPoolTier {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DiskPoolTier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DiskPoolTier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Basic => serializer.serialize_unit_variant("DiskPoolTier", 0u32, "Basic"),
            Self::Standard => serializer.serialize_unit_variant("DiskPoolTier", 1u32, "Standard"),
            Self::Premium => serializer.serialize_unit_variant("DiskPoolTier", 2u32, "Premium"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Request payload for Update Disk pool request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolUpdate {
    #[doc = "Properties for Disk pool update request."]
    pub properties: DiskPoolUpdateProperties,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DiskPoolUpdate {
    pub fn new(properties: DiskPoolUpdateProperties) -> Self {
        Self { properties, tags: None }
    }
}
#[doc = "Properties for Disk pool update request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskPoolUpdateProperties {
    #[doc = "List of Azure Managed Disks to attach to a Disk pool. Can attach 8 disks at most."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<Disk>,
}
impl DiskPoolUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "The resource management error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl azure_core::Continuable for Error {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl Error {
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
#[doc = "The resource management error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
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
    pub details: Vec<ErrorResponse>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "LUN to expose the Azure Managed Disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiLun {
    #[doc = "User defined name for iSCSI LUN; example: \"lun0\""]
    pub name: String,
    #[doc = "Azure Resource ID of the Managed Disk."]
    #[serde(rename = "managedDiskAzureResourceId")]
    pub managed_disk_azure_resource_id: String,
}
impl IscsiLun {
    pub fn new(name: String, managed_disk_azure_resource_id: String) -> Self {
        Self {
            name,
            managed_disk_azure_resource_id,
        }
    }
}
#[doc = "Response for iSCSI target requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTarget {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Response properties for iSCSI target operations."]
    pub properties: IscsiTargetProperties,
}
impl IscsiTarget {
    pub fn new(properties: IscsiTargetProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "Payload for iSCSI target create or update requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetCreate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties for iSCSI target create or update request."]
    pub properties: IscsiTargetCreateProperties,
}
impl IscsiTargetCreate {
    pub fn new(properties: IscsiTargetCreateProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "Properties for iSCSI target create or update request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetCreateProperties {
    #[doc = "List of iSCSI target portal groups. Can have 1 portal group at most."]
    pub tpgs: Vec<TargetPortalGroupCreate>,
    #[doc = "iSCSI target IQN (iSCSI Qualified Name); example: \"iqn.2005-03.org.iscsi:server\"."]
    #[serde(rename = "targetIqn", default, skip_serializing_if = "Option::is_none")]
    pub target_iqn: Option<String>,
}
impl IscsiTargetCreateProperties {
    pub fn new(tpgs: Vec<TargetPortalGroupCreate>) -> Self {
        Self { tpgs, target_iqn: None }
    }
}
#[doc = "Challenge Handshake Authentication Protocol (CHAP) credentials for an iSCSI target ACL."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetCredentials {
    #[doc = "Username for Challenge Handshake Authentication Protocol (CHAP) authentication."]
    pub username: String,
    #[doc = "Password for Challenge Handshake Authentication Protocol (CHAP) authentication."]
    pub password: String,
}
impl IscsiTargetCredentials {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}
#[doc = "List of iSCSI Targets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetList {
    #[doc = "An array of iSCSI targets in a Disk pool."]
    pub value: Vec<IscsiTarget>,
    #[doc = "URI to fetch the next section of the paginated response."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IscsiTargetList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IscsiTargetList {
    pub fn new(value: Vec<IscsiTarget>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Response properties for iSCSI target operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetProperties {
    #[doc = "Provisioning state of the iSCSI target."]
    #[serde(rename = "provisioningState")]
    pub provisioning_state: ProvisioningState,
    #[doc = "Operational status of the resource."]
    pub status: OperationalStatus,
    #[doc = "List of iSCSI target portal groups. Can have 1 portal group at most."]
    pub tpgs: Vec<TargetPortalGroup>,
    #[doc = "iSCSI target IQN (iSCSI Qualified Name); example: \"iqn.2005-03.org.iscsi:server\"."]
    #[serde(rename = "targetIqn")]
    pub target_iqn: String,
}
impl IscsiTargetProperties {
    pub fn new(provisioning_state: ProvisioningState, status: OperationalStatus, tpgs: Vec<TargetPortalGroup>, target_iqn: String) -> Self {
        Self {
            provisioning_state,
            status,
            tpgs,
            target_iqn,
        }
    }
}
#[doc = "Payload for iSCSI target update request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetUpdate {
    #[doc = "Properties for iSCSI target update request."]
    pub properties: IscsiTargetUpdateProperties,
}
impl IscsiTargetUpdate {
    pub fn new(properties: IscsiTargetUpdateProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Properties for iSCSI target update request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetUpdateProperties {
    #[doc = "List of iSCSI target portal groups. Can have 1 portal group at most."]
    pub tpgs: Vec<TargetPortalGroupUpdate>,
}
impl IscsiTargetUpdateProperties {
    pub fn new(tpgs: Vec<TargetPortalGroupUpdate>) -> Self {
        Self { tpgs }
    }
}
#[doc = "Operational status of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationalStatus")]
pub enum OperationalStatus {
    Invalid,
    Unknown,
    Healthy,
    Unhealthy,
    Updating,
    Running,
    Stopped,
    #[serde(rename = "Stopped (deallocated)")]
    StoppedDeallocated,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationalStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationalStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationalStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("OperationalStatus", 0u32, "Invalid"),
            Self::Unknown => serializer.serialize_unit_variant("OperationalStatus", 1u32, "Unknown"),
            Self::Healthy => serializer.serialize_unit_variant("OperationalStatus", 2u32, "Healthy"),
            Self::Unhealthy => serializer.serialize_unit_variant("OperationalStatus", 3u32, "Unhealthy"),
            Self::Updating => serializer.serialize_unit_variant("OperationalStatus", 4u32, "Updating"),
            Self::Running => serializer.serialize_unit_variant("OperationalStatus", 5u32, "Running"),
            Self::Stopped => serializer.serialize_unit_variant("OperationalStatus", 6u32, "Stopped"),
            Self::StoppedDeallocated => serializer.serialize_unit_variant("OperationalStatus", 7u32, "Stopped (deallocated)"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Provisioning state of the iSCSI target."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Invalid,
    Succeeded,
    Failed,
    Canceled,
    Pending,
    Creating,
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
            Self::Invalid => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Invalid"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
            Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Pending"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleting"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition for a ARM proxy resource. It will have everything other than required location and tags"]
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
#[doc = "ARM resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource Id for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. Ex- Microsoft.Compute/virtualMachines or Microsoft.Storage/storageAccounts."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata about an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolOperationDisplay {
    #[doc = "Localized friendly form of the resource provider name."]
    pub provider: String,
    #[doc = "Localized friendly form of the resource type related to this action/operation."]
    pub resource: String,
    #[doc = "Localized friendly name for the operation, as it should be shown to the user."]
    pub operation: String,
    #[doc = "Localized friendly description for the operation, as it should be shown to the user."]
    pub description: String,
}
impl StoragePoolOperationDisplay {
    pub fn new(provider: String, resource: String, operation: String, description: String) -> Self {
        Self {
            provider,
            resource,
            operation,
            description,
        }
    }
}
#[doc = "List of operations supported by the RP."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolOperationListResult {
    #[doc = "An array of operations supported by the StoragePool RP."]
    pub value: Vec<StoragePoolRpOperation>,
    #[doc = "URI to fetch the next section of the paginated response."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StoragePoolOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl StoragePoolOperationListResult {
    pub fn new(value: Vec<StoragePoolRpOperation>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Description of a StoragePool RP Operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolRpOperation {
    #[doc = "The name of the operation being performed on this particular object"]
    pub name: String,
    #[doc = "Indicates whether the operation applies to data-plane."]
    #[serde(rename = "isDataAction")]
    pub is_data_action: bool,
    #[doc = "Indicates the action type."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[doc = "Metadata about an operation."]
    pub display: StoragePoolOperationDisplay,
    #[doc = "The intended executor of the operation; governs the display of the operation in the RBAC UX and the audit logs UX."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl StoragePoolRpOperation {
    pub fn new(name: String, is_data_action: bool, display: StoragePoolOperationDisplay) -> Self {
        Self {
            name,
            is_data_action,
            action_type: None,
            display,
            origin: None,
        }
    }
}
#[doc = "Resource metadata required by ARM RPC."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemMetadata {
    #[doc = "A string identifier for the identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource: user, application, managedIdentity."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<String>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[doc = "A string identifier for the identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource: user, application, managedIdentity."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<String>,
    #[doc = "The timestamp of resource last modification (UTC)."]
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
impl SystemMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response properties for iSCSI target portal group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetPortalGroup {
    #[doc = "List of LUNs to be exposed through iSCSI target portal group."]
    pub luns: Vec<IscsiLun>,
    #[doc = "Access Control List (ACL) for an iSCSI target portal group."]
    pub acls: Vec<Acl>,
    #[doc = "Attributes of a iSCSI target portal group."]
    pub attributes: Attributes,
    #[doc = "List of private IPv4 addresses to connect to the iSCSI target."]
    pub endpoints: Vec<String>,
    #[doc = "The tag associated with the iSCSI target portal group."]
    pub tag: i32,
    #[doc = "The port used by iSCSI target portal group."]
    pub port: i32,
}
impl TargetPortalGroup {
    pub fn new(luns: Vec<IscsiLun>, acls: Vec<Acl>, attributes: Attributes, endpoints: Vec<String>, tag: i32, port: i32) -> Self {
        Self {
            luns,
            acls,
            attributes,
            endpoints,
            tag,
            port,
        }
    }
}
#[doc = "Target portal group properties for create or update iSCSI target request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetPortalGroupCreate {
    #[doc = "List of LUNs to be exposed through the iSCSI target portal group."]
    pub luns: Vec<IscsiLun>,
    #[doc = "Access Control List (ACL) for an iSCSI target portal group."]
    pub acls: Vec<Acl>,
    #[doc = "Attributes of a iSCSI target portal group."]
    pub attributes: Attributes,
}
impl TargetPortalGroupCreate {
    pub fn new(luns: Vec<IscsiLun>, acls: Vec<Acl>, attributes: Attributes) -> Self {
        Self { luns, acls, attributes }
    }
}
#[doc = "Target portal group properties for update iSCSI target request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetPortalGroupUpdate {
    #[doc = "List of LUNs to be exposed through the iSCSI target portal group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub luns: Vec<IscsiLun>,
    #[doc = "Access Control List (ACL) for an iSCSI target portal group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub acls: Vec<Acl>,
}
impl TargetPortalGroupUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a ARM tracked top level resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives."]
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

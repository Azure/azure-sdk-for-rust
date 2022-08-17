#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Access Control List (ACL) for an iSCSI Target; defines LUN masking policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Acl {
    #[doc = "iSCSI initiator IQN (iSCSI Qualified Name); example: \"iqn.2005-03.org.iscsi:client\"."]
    #[serde(rename = "initiatorIqn")]
    pub initiator_iqn: String,
    #[doc = "List of LUN names mapped to the ACL."]
    #[serde(rename = "mappedLuns")]
    pub mapped_luns: Vec<String>,
}
impl Acl {
    pub fn new(initiator_iqn: String, mapped_luns: Vec<String>) -> Self {
        Self {
            initiator_iqn,
            mapped_luns,
        }
    }
}
#[doc = "ACL mode for iSCSI Target."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AclMode")]
pub enum AclMode {
    Dynamic,
    Static,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AclMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AclMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AclMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Dynamic => serializer.serialize_unit_variant("AclMode", 0u32, "Dynamic"),
            Self::Static => serializer.serialize_unit_variant("AclMode", 1u32, "Static"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type AdditionalCapability = String;
pub type AvailabilityZone = String;
#[doc = "Azure Managed Disk to attach to the Disk Pool."]
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
#[doc = "Response for Disk Pool request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Sku for ARM resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Disk Pool response properties."]
    pub properties: DiskPoolProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemMetadata>,
}
impl DiskPool {
    pub fn new(tracked_resource: TrackedResource, properties: DiskPoolProperties) -> Self {
        Self {
            tracked_resource,
            sku: None,
            properties,
            system_data: None,
        }
    }
}
#[doc = "Request payload for create or update Disk Pool request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolCreate {
    #[doc = "Sku for ARM resource"]
    pub sku: Sku,
    #[doc = "Properties for Disk Pool create or update request."]
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
    pub fn new(sku: Sku, properties: DiskPoolCreateProperties, location: String) -> Self {
        Self {
            sku,
            properties,
            tags: None,
            location,
            id: None,
            name: None,
            type_: None,
        }
    }
}
#[doc = "Properties for Disk Pool create or update request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolCreateProperties {
    #[doc = "Logical zone for Disk Pool resource; example: [\"1\"]."]
    #[serde(rename = "availabilityZones", default, skip_serializing_if = "Vec::is_empty")]
    pub availability_zones: Vec<AvailabilityZone>,
    #[doc = "List of Azure Managed Disks to attach to a Disk Pool."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<Disk>,
    #[doc = "Azure Resource ID of a Subnet for the Disk Pool."]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[doc = "List of additional capabilities for a Disk Pool."]
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_capabilities: Vec<AdditionalCapability>,
}
impl DiskPoolCreateProperties {
    pub fn new(subnet_id: String) -> Self {
        Self {
            availability_zones: Vec::new(),
            disks: Vec::new(),
            subnet_id,
            additional_capabilities: Vec::new(),
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
#[doc = "Disk Pool response properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolProperties {
    #[doc = "Provisioning state of the iSCSI Target."]
    #[serde(rename = "provisioningState")]
    pub provisioning_state: ProvisioningState,
    #[doc = "Logical zone for Disk Pool resource; example: [\"1\"]."]
    #[serde(rename = "availabilityZones")]
    pub availability_zones: Vec<AvailabilityZone>,
    #[doc = "Operational status of the resource."]
    pub status: OperationalStatus,
    #[doc = "List of Azure Managed Disks to attach to a Disk Pool."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<Disk>,
    #[doc = "Azure Resource ID of a Subnet for the Disk Pool."]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[doc = "List of additional capabilities for Disk Pool."]
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_capabilities: Vec<AdditionalCapability>,
}
impl DiskPoolProperties {
    pub fn new(
        provisioning_state: ProvisioningState,
        availability_zones: Vec<AvailabilityZone>,
        status: OperationalStatus,
        subnet_id: String,
    ) -> Self {
        Self {
            provisioning_state,
            availability_zones,
            status,
            disks: Vec::new(),
            subnet_id,
            additional_capabilities: Vec::new(),
        }
    }
}
#[doc = "SKU of the VM host part of the Disk Pool deployment"]
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
#[doc = "Request payload for Update Disk Pool request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolUpdate {
    #[doc = "Properties for Disk Pool update request."]
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
#[doc = "Properties for Disk Pool update request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskPoolUpdateProperties {
    #[doc = "List of Azure Managed Disks to attach to a Disk Pool."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<Disk>,
}
impl DiskPoolUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Disk Pool Sku Details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskPoolZoneInfo {
    #[doc = "Logical zone for Disk Pool resource; example: [\"1\"]."]
    #[serde(rename = "availabilityZones", default, skip_serializing_if = "Vec::is_empty")]
    pub availability_zones: Vec<AvailabilityZone>,
    #[doc = "List of additional capabilities for Disk Pool."]
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_capabilities: Vec<AdditionalCapability>,
    #[doc = "Sku for ARM resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl DiskPoolZoneInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List Disk Pool skus operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskPoolZoneListResult {
    #[doc = "The list of Disk Pool Skus."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DiskPoolZoneInfo>,
    #[doc = "URI to fetch the next section of the paginated response."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiskPoolZoneListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DiskPoolZoneListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A domain name that a service is reached at, including details of the current connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDependency {
    #[doc = "The domain name of the dependency."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "The IP Addresses and Ports used when connecting to DomainName."]
    #[serde(rename = "endpointDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint_details: Vec<EndpointDetail>,
}
impl EndpointDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Current TCP connectivity information from the App Service Environment to a single endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDetail {
    #[doc = "An IP Address that Domain Name currently resolves to."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The port an endpoint is connected to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "The time in milliseconds it takes for a TCP connection to be created from the App Service Environment to this IpAddress at this Port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency: Option<f64>,
    #[doc = "Whether it is possible to create a TCP connection from the App Service Environment to this IpAddress at this Port."]
    #[serde(rename = "isAccessible", default, skip_serializing_if = "Option::is_none")]
    pub is_accessible: Option<bool>,
}
impl EndpointDetail {
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
    #[doc = "Specifies the Logical Unit Number of the iSCSI LUN."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
}
impl IscsiLun {
    pub fn new(name: String, managed_disk_azure_resource_id: String) -> Self {
        Self {
            name,
            managed_disk_azure_resource_id,
            lun: None,
        }
    }
}
#[doc = "Response for iSCSI Target requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTarget {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Response properties for iSCSI Target operations."]
    pub properties: IscsiTargetProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemMetadata>,
}
impl IscsiTarget {
    pub fn new(properties: IscsiTargetProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "Payload for iSCSI Target create or update requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetCreate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties for iSCSI Target create or update request."]
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
#[doc = "Properties for iSCSI Target create or update request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetCreateProperties {
    #[doc = "ACL mode for iSCSI Target."]
    #[serde(rename = "aclMode")]
    pub acl_mode: AclMode,
    #[doc = "iSCSI Target IQN (iSCSI Qualified Name); example: \"iqn.2005-03.org.iscsi:server\"."]
    #[serde(rename = "targetIqn", default, skip_serializing_if = "Option::is_none")]
    pub target_iqn: Option<String>,
    #[doc = "Access Control List (ACL) for an iSCSI Target; defines LUN masking policy"]
    #[serde(rename = "staticAcls", default, skip_serializing_if = "Vec::is_empty")]
    pub static_acls: Vec<Acl>,
    #[doc = "List of LUNs to be exposed through iSCSI Target."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub luns: Vec<IscsiLun>,
}
impl IscsiTargetCreateProperties {
    pub fn new(acl_mode: AclMode) -> Self {
        Self {
            acl_mode,
            target_iqn: None,
            static_acls: Vec::new(),
            luns: Vec::new(),
        }
    }
}
#[doc = "List of iSCSI Targets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetList {
    #[doc = "An array of iSCSI Targets in a Disk Pool."]
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
#[doc = "Response properties for iSCSI Target operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetProperties {
    #[doc = "ACL mode for iSCSI Target."]
    #[serde(rename = "aclMode")]
    pub acl_mode: AclMode,
    #[doc = "Access Control List (ACL) for an iSCSI Target; defines LUN masking policy"]
    #[serde(rename = "staticAcls", default, skip_serializing_if = "Vec::is_empty")]
    pub static_acls: Vec<Acl>,
    #[doc = "List of LUNs to be exposed through iSCSI Target."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub luns: Vec<IscsiLun>,
    #[doc = "iSCSI Target IQN (iSCSI Qualified Name); example: \"iqn.2005-03.org.iscsi:server\"."]
    #[serde(rename = "targetIqn")]
    pub target_iqn: String,
    #[doc = "Provisioning state of the iSCSI Target."]
    #[serde(rename = "provisioningState")]
    pub provisioning_state: ProvisioningState,
    #[doc = "Operational status of the resource."]
    pub status: OperationalStatus,
    #[doc = "List of private IPv4 addresses to connect to the iSCSI Target."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<String>,
    #[doc = "The port used by iSCSI Target portal group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
impl IscsiTargetProperties {
    pub fn new(acl_mode: AclMode, target_iqn: String, provisioning_state: ProvisioningState, status: OperationalStatus) -> Self {
        Self {
            acl_mode,
            static_acls: Vec::new(),
            luns: Vec::new(),
            target_iqn,
            provisioning_state,
            status,
            endpoints: Vec::new(),
            port: None,
        }
    }
}
#[doc = "Payload for iSCSI Target update requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetUpdate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties for iSCSI Target update request."]
    pub properties: IscsiTargetUpdateProperties,
}
impl IscsiTargetUpdate {
    pub fn new(properties: IscsiTargetUpdateProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "Properties for iSCSI Target update request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IscsiTargetUpdateProperties {
    #[doc = "Access Control List (ACL) for an iSCSI Target; defines LUN masking policy"]
    #[serde(rename = "staticAcls", default, skip_serializing_if = "Vec::is_empty")]
    pub static_acls: Vec<Acl>,
    #[doc = "List of LUNs to be exposed through iSCSI Target."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub luns: Vec<IscsiLun>,
}
impl IscsiTargetUpdateProperties {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Endpoints accessed for a common purpose that the App Service Environment requires outbound network access to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundEnvironmentEndpoint {
    #[doc = "The type of service accessed by the App Service Environment, e.g., Azure Storage, Azure SQL Database, and Azure Active Directory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The endpoints that the App Service Environment reaches the service at."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<EndpointDependency>,
}
impl OutboundEnvironmentEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of Outbound Environment Endpoints"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutboundEnvironmentEndpointList {
    #[doc = "Collection of resources."]
    pub value: Vec<OutboundEnvironmentEndpoint>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OutboundEnvironmentEndpointList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OutboundEnvironmentEndpointList {
    pub fn new(value: Vec<OutboundEnvironmentEndpoint>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Provisioning state of the iSCSI Target."]
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
#[doc = "Sku for ARM resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "Sku name"]
    pub name: String,
    #[doc = "Sku tier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self { name, tier: None }
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
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemMetadata {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_metadata::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_metadata::LastModifiedByType>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_metadata {
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

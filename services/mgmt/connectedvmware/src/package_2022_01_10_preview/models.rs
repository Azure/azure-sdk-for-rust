#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Summarization of patches available for installation on the machine by classification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailablePatchCountByClassification {
    #[doc = "Number of security patches available for installation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security: Option<i32>,
    #[doc = "Number of critical patches available for installation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<i32>,
    #[doc = "Number of definition patches available for installation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<i32>,
    #[doc = "Number of update Rollup patches available for installation."]
    #[serde(rename = "updateRollup", default, skip_serializing_if = "Option::is_none")]
    pub update_rollup: Option<i32>,
    #[doc = "Number of feature pack patches available for installation."]
    #[serde(rename = "featurePack", default, skip_serializing_if = "Option::is_none")]
    pub feature_pack: Option<i32>,
    #[doc = "Number of service pack patches available for installation."]
    #[serde(rename = "servicePack", default, skip_serializing_if = "Option::is_none")]
    pub service_pack: Option<i32>,
    #[doc = "Number of tools patches available for installation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<i32>,
    #[doc = "Number of updates category patches available for installation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updates: Option<i32>,
    #[doc = "Number of other patches available for installation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other: Option<i32>,
}
impl AvailablePatchCountByClassification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Define the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[doc = "Defines the resource properties."]
    pub properties: ClusterProperties,
    #[doc = "Gets or sets the location."]
    pub location: String,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets the name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata used by portal/tooling/etc to render different UX experiences for resources of the same type; e.g. ApiApps are a kind of Microsoft.Web/sites type.  If supported, the resource provider must validate and persist this value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl Cluster {
    pub fn new(properties: ClusterProperties, location: String) -> Self {
        Self {
            properties,
            location,
            extended_location: None,
            system_data: None,
            tags: None,
            name: None,
            id: None,
            type_: None,
            kind: None,
        }
    }
}
#[doc = "The cluster inventory item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterInventoryItem {
    #[serde(flatten)]
    pub inventory_item_properties: InventoryItemProperties,
}
impl ClusterInventoryItem {
    pub fn new(inventory_item_properties: InventoryItemProperties) -> Self {
        Self { inventory_item_properties }
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterProperties {
    #[doc = "Gets or sets a unique identifier for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Gets or sets the ARM Id of the vCenter resource in which this cluster resides."]
    #[serde(rename = "vCenterId", default, skip_serializing_if = "Option::is_none")]
    pub v_center_id: Option<String>,
    #[doc = "Gets or sets the vCenter MoRef (Managed Object Reference) ID for the cluster."]
    #[serde(rename = "moRefId", default, skip_serializing_if = "Option::is_none")]
    pub mo_ref_id: Option<String>,
    #[doc = "Gets or sets the inventory Item ID for the cluster."]
    #[serde(rename = "inventoryItemId", default, skip_serializing_if = "Option::is_none")]
    pub inventory_item_id: Option<String>,
    #[doc = "Gets or sets the vCenter Managed Object name for the cluster."]
    #[serde(rename = "moName", default, skip_serializing_if = "Option::is_none")]
    pub mo_name: Option<String>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<ResourceStatus>,
    #[doc = "Gets the name of the corresponding resource in Kubernetes."]
    #[serde(rename = "customResourceName", default, skip_serializing_if = "Option::is_none")]
    pub custom_resource_name: Option<String>,
    #[doc = "Gets or sets the datastore ARM ids."]
    #[serde(rename = "datastoreIds", default, skip_serializing_if = "Vec::is_empty")]
    pub datastore_ids: Vec<String>,
    #[doc = "Gets or sets the network ARM ids."]
    #[serde(rename = "networkIds", default, skip_serializing_if = "Vec::is_empty")]
    pub network_ids: Vec<String>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Clusters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClustersList {
    #[doc = "Url to follow for getting next page of Clusters."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of Clusters"]
    pub value: Vec<Cluster>,
}
impl azure_core::Continuable for ClustersList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ClustersList {
    pub fn new(value: Vec<Cluster>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Condition defines an extension to status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Condition {
    #[doc = "Status of the condition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The reason for the condition's status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "A human readable message indicating details about the status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Severity with which to treat failures of this type of condition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}
impl Condition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Define the datastore."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Datastore {
    #[doc = "Defines the resource properties."]
    pub properties: DatastoreProperties,
    #[doc = "Gets or sets the location."]
    pub location: String,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets the name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata used by portal/tooling/etc to render different UX experiences for resources of the same type; e.g. ApiApps are a kind of Microsoft.Web/sites type.  If supported, the resource provider must validate and persist this value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl Datastore {
    pub fn new(properties: DatastoreProperties, location: String) -> Self {
        Self {
            properties,
            location,
            extended_location: None,
            system_data: None,
            tags: None,
            name: None,
            id: None,
            type_: None,
            kind: None,
        }
    }
}
#[doc = "The datastore inventory item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatastoreInventoryItem {
    #[serde(flatten)]
    pub inventory_item_properties: InventoryItemProperties,
    #[doc = "Gets or sets Maximum capacity of this datastore, in GBs."]
    #[serde(rename = "capacityGB", default, skip_serializing_if = "Option::is_none")]
    pub capacity_gb: Option<i64>,
    #[doc = "Gets or sets Available space of this datastore, in GBs."]
    #[serde(rename = "freeSpaceGB", default, skip_serializing_if = "Option::is_none")]
    pub free_space_gb: Option<i64>,
}
impl DatastoreInventoryItem {
    pub fn new(inventory_item_properties: InventoryItemProperties) -> Self {
        Self {
            inventory_item_properties,
            capacity_gb: None,
            free_space_gb: None,
        }
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatastoreProperties {
    #[doc = "Gets or sets a unique identifier for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Gets or sets the ARM Id of the vCenter resource in which this datastore resides."]
    #[serde(rename = "vCenterId", default, skip_serializing_if = "Option::is_none")]
    pub v_center_id: Option<String>,
    #[doc = "Gets or sets the vCenter MoRef (Managed Object Reference) ID for the datastore."]
    #[serde(rename = "moRefId", default, skip_serializing_if = "Option::is_none")]
    pub mo_ref_id: Option<String>,
    #[doc = "Gets or sets the inventory Item ID for the datastore."]
    #[serde(rename = "inventoryItemId", default, skip_serializing_if = "Option::is_none")]
    pub inventory_item_id: Option<String>,
    #[doc = "Gets or sets the vCenter Managed Object name for the datastore."]
    #[serde(rename = "moName", default, skip_serializing_if = "Option::is_none")]
    pub mo_name: Option<String>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<ResourceStatus>,
    #[doc = "Gets the name of the corresponding resource in Kubernetes."]
    #[serde(rename = "customResourceName", default, skip_serializing_if = "Option::is_none")]
    pub custom_resource_name: Option<String>,
    #[doc = "The current deployment state of resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ResourceProvisioningState>,
}
impl DatastoreProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Datastores."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatastoresList {
    #[doc = "Url to follow for getting next page of Datastores."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of Datastores"]
    pub value: Vec<Datastore>,
}
impl azure_core::Continuable for DatastoresList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DatastoresList {
    pub fn new(value: Vec<Datastore>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Defines the different types of disk modes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DiskMode")]
pub enum DiskMode {
    #[serde(rename = "persistent")]
    Persistent,
    #[serde(rename = "independent_persistent")]
    IndependentPersistent,
    #[serde(rename = "independent_nonpersistent")]
    IndependentNonpersistent,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DiskMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DiskMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DiskMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Persistent => serializer.serialize_unit_variant("DiskMode", 0u32, "persistent"),
            Self::IndependentPersistent => serializer.serialize_unit_variant("DiskMode", 1u32, "independent_persistent"),
            Self::IndependentNonpersistent => serializer.serialize_unit_variant("DiskMode", 2u32, "independent_nonpersistent"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the different types of disks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DiskType")]
pub enum DiskType {
    #[serde(rename = "flat")]
    Flat,
    #[serde(rename = "pmem")]
    Pmem,
    #[serde(rename = "rawphysical")]
    Rawphysical,
    #[serde(rename = "rawvirtual")]
    Rawvirtual,
    #[serde(rename = "sparse")]
    Sparse,
    #[serde(rename = "sesparse")]
    Sesparse,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DiskType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DiskType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DiskType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Flat => serializer.serialize_unit_variant("DiskType", 0u32, "flat"),
            Self::Pmem => serializer.serialize_unit_variant("DiskType", 1u32, "pmem"),
            Self::Rawphysical => serializer.serialize_unit_variant("DiskType", 2u32, "rawphysical"),
            Self::Rawvirtual => serializer.serialize_unit_variant("DiskType", 3u32, "rawvirtual"),
            Self::Sparse => serializer.serialize_unit_variant("DiskType", 4u32, "sparse"),
            Self::Sesparse => serializer.serialize_unit_variant("DiskType", 5u32, "sesparse"),
            Self::Unknown => serializer.serialize_unit_variant("DiskType", 6u32, "unknown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Internal error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDefinition>,
}
impl ErrorDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[doc = "The error's code."]
    pub code: String,
    #[doc = "A human readable error message."]
    pub message: String,
    #[doc = "Indicates which property in the request is responsible for the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Additional error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
}
impl ErrorDetail {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
        }
    }
}
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
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
#[doc = "The extended location."]
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
#[doc = "Firmware type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FirmwareType")]
pub enum FirmwareType {
    #[serde(rename = "bios")]
    Bios,
    #[serde(rename = "efi")]
    Efi,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FirmwareType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FirmwareType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FirmwareType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bios => serializer.serialize_unit_variant("FirmwareType", 0u32, "bios"),
            Self::Efi => serializer.serialize_unit_variant("FirmwareType", 1u32, "efi"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the GuestAgent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestAgent {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Defines the resource properties."]
    pub properties: GuestAgentProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl GuestAgent {
    pub fn new(properties: GuestAgentProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "List of GuestAgent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestAgentList {
    #[doc = "Url to follow for getting next page of GuestAgent."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of GuestAgent"]
    pub value: Vec<GuestAgent>,
}
impl azure_core::Continuable for GuestAgentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GuestAgentList {
    pub fn new(value: Vec<GuestAgent>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestAgentProfile {
    #[doc = "Specifies the VM's unique SMBIOS ID."]
    #[serde(rename = "vmUuid", default, skip_serializing_if = "Option::is_none")]
    pub vm_uuid: Option<String>,
    #[doc = "The status of the hybrid machine agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<guest_agent_profile::Status>,
    #[doc = "The time of the last status change."]
    #[serde(rename = "lastStatusChange", default, with = "azure_core::date::rfc3339::option")]
    pub last_status_change: Option<time::OffsetDateTime>,
    #[doc = "The hybrid machine agent full version."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "Details about the error state."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<ErrorDetail>,
}
impl GuestAgentProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod guest_agent_profile {
    use super::*;
    #[doc = "The status of the hybrid machine agent."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Connected,
        Disconnected,
        Error,
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
                Self::Connected => serializer.serialize_unit_variant("Status", 0u32, "Connected"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 1u32, "Disconnected"),
                Self::Error => serializer.serialize_unit_variant("Status", 2u32, "Error"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestAgentProperties {
    #[doc = "Gets or sets a unique identifier for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Username / Password Credentials to connect to guest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<GuestCredential>,
    #[doc = "HTTP Proxy configuration for the VM."]
    #[serde(rename = "httpProxyConfig", default, skip_serializing_if = "Option::is_none")]
    pub http_proxy_config: Option<HttpProxyConfiguration>,
    #[doc = "Defines the different types of operations for guest agent."]
    #[serde(rename = "provisioningAction", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_action: Option<ProvisioningAction>,
    #[doc = "Gets or sets the guest agent status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets the name of the corresponding resource in Kubernetes."]
    #[serde(rename = "customResourceName", default, skip_serializing_if = "Option::is_none")]
    pub custom_resource_name: Option<String>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<ResourceStatus>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl GuestAgentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Username / Password Credentials to connect to guest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestCredential {
    #[doc = "Gets or sets username to connect with the guest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "Gets or sets the password to connect with the guest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl GuestCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HardwareProfile {
    #[doc = "Gets or sets memory size in MBs for the vm."]
    #[serde(rename = "memorySizeMB", default, skip_serializing_if = "Option::is_none")]
    pub memory_size_mb: Option<i32>,
    #[doc = "Gets or sets the number of vCPUs for the vm."]
    #[serde(rename = "numCPUs", default, skip_serializing_if = "Option::is_none")]
    pub num_cp_us: Option<i32>,
    #[doc = "Gets or sets the number of cores per socket for the vm. Defaults to 1 if unspecified."]
    #[serde(rename = "numCoresPerSocket", default, skip_serializing_if = "Option::is_none")]
    pub num_cores_per_socket: Option<i32>,
    #[doc = "Gets or sets a value indicating whether virtual processors can be added while this virtual machine is running."]
    #[serde(rename = "cpuHotAddEnabled", default, skip_serializing_if = "Option::is_none")]
    pub cpu_hot_add_enabled: Option<bool>,
    #[doc = "Gets or sets a value indicating whether virtual processors can be removed while this virtual machine is running."]
    #[serde(rename = "cpuHotRemoveEnabled", default, skip_serializing_if = "Option::is_none")]
    pub cpu_hot_remove_enabled: Option<bool>,
    #[doc = "Gets or sets a value indicating whether memory can be added while this virtual machine is running."]
    #[serde(rename = "memoryHotAddEnabled", default, skip_serializing_if = "Option::is_none")]
    pub memory_hot_add_enabled: Option<bool>,
}
impl HardwareProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Define the host."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Host {
    #[doc = "Defines the resource properties."]
    pub properties: HostProperties,
    #[doc = "Gets or sets the location."]
    pub location: String,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets the name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata used by portal/tooling/etc to render different UX experiences for resources of the same type; e.g. ApiApps are a kind of Microsoft.Web/sites type.  If supported, the resource provider must validate and persist this value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl Host {
    pub fn new(properties: HostProperties, location: String) -> Self {
        Self {
            properties,
            location,
            extended_location: None,
            system_data: None,
            tags: None,
            name: None,
            id: None,
            type_: None,
            kind: None,
        }
    }
}
#[doc = "The host inventory item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostInventoryItem {
    #[serde(flatten)]
    pub inventory_item_properties: InventoryItemProperties,
    #[doc = "Defines the resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<InventoryItemDetails>,
}
impl HostInventoryItem {
    pub fn new(inventory_item_properties: InventoryItemProperties) -> Self {
        Self {
            inventory_item_properties,
            parent: None,
        }
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostProperties {
    #[doc = "Gets or sets a unique identifier for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Gets or sets the ARM Id of the vCenter resource in which this host resides."]
    #[serde(rename = "vCenterId", default, skip_serializing_if = "Option::is_none")]
    pub v_center_id: Option<String>,
    #[doc = "Gets or sets the vCenter MoRef (Managed Object Reference) ID for the host."]
    #[serde(rename = "moRefId", default, skip_serializing_if = "Option::is_none")]
    pub mo_ref_id: Option<String>,
    #[doc = "Gets or sets the inventory Item ID for the host."]
    #[serde(rename = "inventoryItemId", default, skip_serializing_if = "Option::is_none")]
    pub inventory_item_id: Option<String>,
    #[doc = "Gets or sets the vCenter Managed Object name for the host."]
    #[serde(rename = "moName", default, skip_serializing_if = "Option::is_none")]
    pub mo_name: Option<String>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<ResourceStatus>,
    #[doc = "Gets the name of the corresponding resource in Kubernetes."]
    #[serde(rename = "customResourceName", default, skip_serializing_if = "Option::is_none")]
    pub custom_resource_name: Option<String>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl HostProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Hosts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostsList {
    #[doc = "Url to follow for getting next page of Hosts."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of Hosts"]
    pub value: Vec<Host>,
}
impl azure_core::Continuable for HostsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HostsList {
    pub fn new(value: Vec<Host>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "HTTP Proxy configuration for the VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpProxyConfiguration {
    #[doc = "Gets or sets httpsProxy url."]
    #[serde(rename = "httpsProxy", default, skip_serializing_if = "Option::is_none")]
    pub https_proxy: Option<String>,
}
impl HttpProxyConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the HybridIdentityMetadata."]
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
#[doc = "List of HybridIdentityMetadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridIdentityMetadataList {
    #[doc = "Url to follow for getting next page of HybridIdentityMetadata."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of HybridIdentityMetadata"]
    pub value: Vec<HybridIdentityMetadata>,
}
impl azure_core::Continuable for HybridIdentityMetadataList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[doc = "Gets or sets the Vm Id."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "Gets or sets the Public Key."]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[doc = "Managed service identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl HybridIdentityMetadataProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IP address allocation method."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IpAddressAllocationMethod")]
pub enum IpAddressAllocationMethod {
    #[serde(rename = "unset")]
    Unset,
    #[serde(rename = "dynamic")]
    Dynamic,
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "linklayer")]
    Linklayer,
    #[serde(rename = "random")]
    Random,
    #[serde(rename = "other")]
    Other,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IpAddressAllocationMethod {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IpAddressAllocationMethod {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IpAddressAllocationMethod {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unset => serializer.serialize_unit_variant("IpAddressAllocationMethod", 0u32, "unset"),
            Self::Dynamic => serializer.serialize_unit_variant("IpAddressAllocationMethod", 1u32, "dynamic"),
            Self::Static => serializer.serialize_unit_variant("IpAddressAllocationMethod", 2u32, "static"),
            Self::Linklayer => serializer.serialize_unit_variant("IpAddressAllocationMethod", 3u32, "linklayer"),
            Self::Random => serializer.serialize_unit_variant("IpAddressAllocationMethod", 4u32, "random"),
            Self::Other => serializer.serialize_unit_variant("IpAddressAllocationMethod", 5u32, "other"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Managed service identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[doc = "The principal id of managed service identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant of managed service identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of managed service identity."]
    #[serde(rename = "type")]
    pub type_: identity::Type,
}
impl Identity {
    pub fn new(type_: identity::Type) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
        }
    }
}
pub mod identity {
    use super::*;
    #[doc = "The type of managed service identity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        SystemAssigned,
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
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 1u32, "SystemAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the inventory item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryItem {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Defines the resource properties."]
    pub properties: InventoryItemProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Metadata used by portal/tooling/etc to render different UX experiences for resources of the same type; e.g. ApiApps are a kind of Microsoft.Web/sites type.  If supported, the resource provider must validate and persist this value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl InventoryItem {
    pub fn new(properties: InventoryItemProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
            kind: None,
        }
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InventoryItemDetails {
    #[doc = "Gets or sets the inventory Item ID for the resource."]
    #[serde(rename = "inventoryItemId", default, skip_serializing_if = "Option::is_none")]
    pub inventory_item_id: Option<String>,
    #[doc = "Gets or sets the vCenter Managed Object name for the resource."]
    #[serde(rename = "moName", default, skip_serializing_if = "Option::is_none")]
    pub mo_name: Option<String>,
}
impl InventoryItemDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryItemProperties {
    #[doc = "The inventory type."]
    #[serde(rename = "inventoryType")]
    pub inventory_type: InventoryType,
    #[doc = "Gets or sets the tracked resource id corresponding to the inventory resource."]
    #[serde(rename = "managedResourceId", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_id: Option<String>,
    #[doc = "Gets or sets the MoRef (Managed Object Reference) ID for the inventory item."]
    #[serde(rename = "moRefId", default, skip_serializing_if = "Option::is_none")]
    pub mo_ref_id: Option<String>,
    #[doc = "Gets or sets the vCenter Managed Object name for the inventory item."]
    #[serde(rename = "moName", default, skip_serializing_if = "Option::is_none")]
    pub mo_name: Option<String>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl InventoryItemProperties {
    pub fn new(inventory_type: InventoryType) -> Self {
        Self {
            inventory_type,
            managed_resource_id: None,
            mo_ref_id: None,
            mo_name: None,
            provisioning_state: None,
        }
    }
}
#[doc = "List of InventoryItems."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryItemsList {
    #[doc = "Url to follow for getting next page of InventoryItems."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of InventoryItems"]
    pub value: Vec<InventoryItem>,
}
impl azure_core::Continuable for InventoryItemsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl InventoryItemsList {
    pub fn new(value: Vec<InventoryItem>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "The inventory type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InventoryType")]
pub enum InventoryType {
    ResourcePool,
    VirtualMachine,
    VirtualMachineTemplate,
    VirtualNetwork,
    Cluster,
    Datastore,
    Host,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InventoryType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InventoryType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InventoryType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ResourcePool => serializer.serialize_unit_variant("InventoryType", 0u32, "ResourcePool"),
            Self::VirtualMachine => serializer.serialize_unit_variant("InventoryType", 1u32, "VirtualMachine"),
            Self::VirtualMachineTemplate => serializer.serialize_unit_variant("InventoryType", 2u32, "VirtualMachineTemplate"),
            Self::VirtualNetwork => serializer.serialize_unit_variant("InventoryType", 3u32, "VirtualNetwork"),
            Self::Cluster => serializer.serialize_unit_variant("InventoryType", 4u32, "Cluster"),
            Self::Datastore => serializer.serialize_unit_variant("InventoryType", 5u32, "Datastore"),
            Self::Host => serializer.serialize_unit_variant("InventoryType", 6u32, "Host"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Input for InstallPatches on a Linux VM, as directly received by the API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxParameters {
    #[doc = "The update classifications to select when installing patches for Linux."]
    #[serde(rename = "classificationsToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub classifications_to_include: Vec<String>,
    #[doc = "packages to include in the patch operation. Format: packageName_packageVersion"]
    #[serde(rename = "packageNameMasksToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub package_name_masks_to_include: Vec<String>,
    #[doc = "packages to exclude in the patch operation. Format: packageName_packageVersion"]
    #[serde(rename = "packageNameMasksToExclude", default, skip_serializing_if = "Vec::is_empty")]
    pub package_name_masks_to_exclude: Vec<String>,
}
impl LinuxParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Machine Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtension {
    #[doc = "Describes the properties of a Machine Extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineExtensionProperties>,
    #[doc = "Gets or sets the location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets the name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl MachineExtension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the Machine Extension Instance View."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionInstanceView {
    #[doc = "The machine extension name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Instance view status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<machine_extension_instance_view::Status>,
}
impl MachineExtensionInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_extension_instance_view {
    use super::*;
    #[doc = "Instance view status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Status {
        #[doc = "The status code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "The level code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub level: Option<status::Level>,
        #[doc = "The short localizable label for the status."]
        #[serde(rename = "displayStatus", default, skip_serializing_if = "Option::is_none")]
        pub display_status: Option<String>,
        #[doc = "The detailed status message, including for alerts and error messages."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "The time of the status."]
        #[serde(default, with = "azure_core::date::rfc3339::option")]
        pub time: Option<time::OffsetDateTime>,
    }
    impl Status {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod status {
        use super::*;
        #[doc = "The level code."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Level")]
        pub enum Level {
            Info,
            Warning,
            Error,
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
                    Self::Info => serializer.serialize_unit_variant("Level", 0u32, "Info"),
                    Self::Warning => serializer.serialize_unit_variant("Level", 1u32, "Warning"),
                    Self::Error => serializer.serialize_unit_variant("Level", 2u32, "Error"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Describes the properties of a Machine Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionProperties {
    #[doc = "How the extension handler should be forced to update even if the extension configuration has not changed."]
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[doc = "The name of the extension handler publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Indicates whether the extension should be automatically upgraded by the platform if there is a newer version available."]
    #[serde(rename = "enableAutomaticUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_upgrade: Option<bool>,
    #[doc = "Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true."]
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[doc = "Json formatted public settings for the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[doc = "The extension can contain either protectedSettings or protectedSettingsFromKeyVault or no protected settings at all."]
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<serde_json::Value>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The machine extension instance view."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<serde_json::Value>,
}
impl MachineExtensionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Machine Extension Update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionUpdate {
    #[serde(flatten)]
    pub resource_patch: ResourcePatch,
    #[doc = "Describes the properties of a Machine Extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineExtensionUpdateProperties>,
}
impl MachineExtensionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Machine Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionUpdateProperties {
    #[doc = "How the extension handler should be forced to update even if the extension configuration has not changed."]
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[doc = "The name of the extension handler publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Indicates whether the extension should be automatically upgraded by the platform if there is a newer version available."]
    #[serde(rename = "enableAutomaticUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_upgrade: Option<bool>,
    #[doc = "Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true."]
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[doc = "Json formatted public settings for the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[doc = "The extension can contain either protectedSettings or protectedSettingsFromKeyVault or no protected settings at all."]
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<serde_json::Value>,
}
impl MachineExtensionUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the Machine Extensions List Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionsListResult {
    #[doc = "The list of extensions"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MachineExtension>,
    #[doc = "The uri to fetch the next page of machine extensions. Call ListNext() with this to fetch the next page of extensions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MachineExtensionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MachineExtensionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NIC type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NicType")]
pub enum NicType {
    #[serde(rename = "vmxnet3")]
    Vmxnet3,
    #[serde(rename = "vmxnet2")]
    Vmxnet2,
    #[serde(rename = "vmxnet")]
    Vmxnet,
    #[serde(rename = "e1000")]
    E1000,
    #[serde(rename = "e1000e")]
    E1000e,
    #[serde(rename = "pcnet32")]
    Pcnet32,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NicType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NicType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NicType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Vmxnet3 => serializer.serialize_unit_variant("NicType", 0u32, "vmxnet3"),
            Self::Vmxnet2 => serializer.serialize_unit_variant("NicType", 1u32, "vmxnet2"),
            Self::Vmxnet => serializer.serialize_unit_variant("NicType", 2u32, "vmxnet"),
            Self::E1000 => serializer.serialize_unit_variant("NicType", 3u32, "e1000"),
            Self::E1000e => serializer.serialize_unit_variant("NicType", 4u32, "e1000e"),
            Self::Pcnet32 => serializer.serialize_unit_variant("NicType", 5u32, "pcnet32"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Network Interface model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterface {
    #[doc = "Gets or sets the name of the network interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the label of the virtual network in vCenter that the nic is connected to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Gets or sets the nic ip addresses."]
    #[serde(rename = "ipAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_addresses: Vec<String>,
    #[doc = "Gets or sets the NIC MAC address."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "Gets or sets the ARM Id of the network resource to connect the virtual machine."]
    #[serde(rename = "networkId", default, skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[doc = "NIC type"]
    #[serde(rename = "nicType", default, skip_serializing_if = "Option::is_none")]
    pub nic_type: Option<NicType>,
    #[doc = "Defines the options for power on boot."]
    #[serde(rename = "powerOnBoot", default, skip_serializing_if = "Option::is_none")]
    pub power_on_boot: Option<PowerOnBootOption>,
    #[doc = "Gets or sets the vCenter MoRef (Managed Object Reference) ID of the virtual network\r\nthat the nic is connected to."]
    #[serde(rename = "networkMoRefId", default, skip_serializing_if = "Option::is_none")]
    pub network_mo_ref_id: Option<String>,
    #[doc = "Gets or sets the name of the virtual network in vCenter that the nic is connected to."]
    #[serde(rename = "networkMoName", default, skip_serializing_if = "Option::is_none")]
    pub network_mo_name: Option<String>,
    #[doc = "Gets or sets the device key value."]
    #[serde(rename = "deviceKey", default, skip_serializing_if = "Option::is_none")]
    pub device_key: Option<i32>,
    #[doc = "Defines the network interface ip settings."]
    #[serde(rename = "ipSettings", default, skip_serializing_if = "Option::is_none")]
    pub ip_settings: Option<NicIpSettings>,
}
impl NetworkInterface {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the network interface update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceUpdate {
    #[doc = "Gets or sets the name of the network interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the ARM Id of the network resource to connect the virtual machine."]
    #[serde(rename = "networkId", default, skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[doc = "NIC type"]
    #[serde(rename = "nicType", default, skip_serializing_if = "Option::is_none")]
    pub nic_type: Option<NicType>,
    #[doc = "Defines the options for power on boot."]
    #[serde(rename = "powerOnBoot", default, skip_serializing_if = "Option::is_none")]
    pub power_on_boot: Option<PowerOnBootOption>,
    #[doc = "Gets or sets the device key value."]
    #[serde(rename = "deviceKey", default, skip_serializing_if = "Option::is_none")]
    pub device_key: Option<i32>,
}
impl NetworkInterfaceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfile {
    #[doc = "Gets or sets the list of network interfaces associated with the virtual machine."]
    #[serde(rename = "networkInterfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<NetworkInterface>,
}
impl NetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the update resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfileUpdate {
    #[doc = "Gets or sets the list of network interfaces associated with the virtual machine."]
    #[serde(rename = "networkInterfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<NetworkInterfaceUpdate>,
}
impl NetworkProfileUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IP address information for a virtual network adapter reported by the fabric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NicIpAddressSettings {
    #[doc = "Gets the ip address allocation method."]
    #[serde(rename = "allocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub allocation_method: Option<String>,
    #[doc = "Gets the ip address for the nic."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Gets the mask."]
    #[serde(rename = "subnetMask", default, skip_serializing_if = "Option::is_none")]
    pub subnet_mask: Option<String>,
}
impl NicIpAddressSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the network interface ip settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NicIpSettings {
    #[doc = "IP address allocation method."]
    #[serde(rename = "allocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub allocation_method: Option<IpAddressAllocationMethod>,
    #[doc = "Gets or sets the dns servers."]
    #[serde(rename = "dnsServers", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_servers: Vec<String>,
    #[doc = "Gets or sets the gateway."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gateway: Vec<String>,
    #[doc = "Gets or sets the ip address for the nic."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Gets or sets the mask."]
    #[serde(rename = "subnetMask", default, skip_serializing_if = "Option::is_none")]
    pub subnet_mask: Option<String>,
    #[doc = "Gets or sets the primary server."]
    #[serde(rename = "primaryWinsServer", default, skip_serializing_if = "Option::is_none")]
    pub primary_wins_server: Option<String>,
    #[doc = "Gets or sets the secondary server."]
    #[serde(rename = "secondaryWinsServer", default, skip_serializing_if = "Option::is_none")]
    pub secondary_wins_server: Option<String>,
    #[doc = "Gets or sets the IP address information being reported for this NIC. This contains the same IPv4 information above plus IPV6 information."]
    #[serde(rename = "ipAddressInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_address_info: Vec<NicIpAddressSettings>,
}
impl NicIpSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfile {
    #[doc = "Gets or sets computer name."]
    #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[doc = "Gets or sets administrator username."]
    #[serde(rename = "adminUsername", default, skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<String>,
    #[doc = "Gets or sets administrator password."]
    #[serde(rename = "adminPassword", default, skip_serializing_if = "Option::is_none")]
    pub admin_password: Option<String>,
    #[doc = "Gets or sets the guestId."]
    #[serde(rename = "guestId", default, skip_serializing_if = "Option::is_none")]
    pub guest_id: Option<String>,
    #[doc = "Gets or sets a value indicating whether the VM is ready for extension operations."]
    #[serde(rename = "allowExtensionOperations", default, skip_serializing_if = "Option::is_none")]
    pub allow_extension_operations: Option<bool>,
    #[doc = "Defines the different types of VM guest operating systems."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "Gets or sets os name."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "Gets or sets the current running status of VMware Tools running in the guest operating system."]
    #[serde(rename = "toolsRunningStatus", default, skip_serializing_if = "Option::is_none")]
    pub tools_running_status: Option<String>,
    #[doc = "Gets or sets the current version status of VMware Tools installed in the guest operating system."]
    #[serde(rename = "toolsVersionStatus", default, skip_serializing_if = "Option::is_none")]
    pub tools_version_status: Option<String>,
    #[doc = "Gets or sets the current version of VMware Tools."]
    #[serde(rename = "toolsVersion", default, skip_serializing_if = "Option::is_none")]
    pub tools_version: Option<String>,
    #[doc = "Specifies the windows configuration for update management."]
    #[serde(rename = "windowsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<os_profile::WindowsConfiguration>,
    #[doc = "Specifies the linux configuration for update management."]
    #[serde(rename = "linuxConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub linux_configuration: Option<os_profile::LinuxConfiguration>,
}
impl OsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod os_profile {
    use super::*;
    #[doc = "Specifies the windows configuration for update management."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct WindowsConfiguration {
        #[doc = "Specifies the patch settings."]
        #[serde(rename = "patchSettings", default, skip_serializing_if = "Option::is_none")]
        pub patch_settings: Option<PatchSettings>,
    }
    impl WindowsConfiguration {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Specifies the linux configuration for update management."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct LinuxConfiguration {
        #[doc = "Specifies the patch settings."]
        #[serde(rename = "patchSettings", default, skip_serializing_if = "Option::is_none")]
        pub patch_settings: Option<PatchSettings>,
    }
    impl LinuxConfiguration {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Defines the os update properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfileUpdate {
    #[doc = "Specifies the windows configuration for update management."]
    #[serde(rename = "windowsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<os_profile_update::WindowsConfiguration>,
    #[doc = "Specifies the linux configuration for update management."]
    #[serde(rename = "linuxConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub linux_configuration: Option<os_profile_update::LinuxConfiguration>,
}
impl OsProfileUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod os_profile_update {
    use super::*;
    #[doc = "Specifies the windows configuration for update management."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct WindowsConfiguration {
        #[doc = "Specifies the patch settings."]
        #[serde(rename = "patchSettings", default, skip_serializing_if = "Option::is_none")]
        pub patch_settings: Option<PatchSettings>,
    }
    impl WindowsConfiguration {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Specifies the linux configuration for update management."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct LinuxConfiguration {
        #[doc = "Specifies the patch settings."]
        #[serde(rename = "patchSettings", default, skip_serializing_if = "Option::is_none")]
        pub patch_settings: Option<PatchSettings>,
    }
    impl LinuxConfiguration {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Defines the different types of VM guest operating systems."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OsType")]
pub enum OsType {
    Windows,
    Linux,
    Other,
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
            Self::Other => serializer.serialize_unit_variant("OsType", 2u32, "Other"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies the patch settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchSettings {
    #[doc = "Specifies the assessment mode."]
    #[serde(rename = "assessmentMode", default, skip_serializing_if = "Option::is_none")]
    pub assessment_mode: Option<String>,
    #[doc = "Specifies the patch mode."]
    #[serde(rename = "patchMode", default, skip_serializing_if = "Option::is_none")]
    pub patch_mode: Option<String>,
}
impl PatchSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlacementProfile {
    #[doc = "Gets or sets the ARM Id of the resourcePool resource on which this virtual machine will deploy."]
    #[serde(rename = "resourcePoolId", default, skip_serializing_if = "Option::is_none")]
    pub resource_pool_id: Option<String>,
    #[doc = "Gets or sets the ARM Id of the cluster resource on which this virtual machine will deploy."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "Gets or sets the ARM Id of the host resource on which this virtual machine will deploy."]
    #[serde(rename = "hostId", default, skip_serializing_if = "Option::is_none")]
    pub host_id: Option<String>,
    #[doc = "Gets or sets the ARM Id of the datastore resource on which the data for the virtual machine will be kept."]
    #[serde(rename = "datastoreId", default, skip_serializing_if = "Option::is_none")]
    pub datastore_id: Option<String>,
}
impl PlacementProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the options for power on boot."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PowerOnBootOption")]
pub enum PowerOnBootOption {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PowerOnBootOption {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PowerOnBootOption {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PowerOnBootOption {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("PowerOnBootOption", 0u32, "enabled"),
            Self::Disabled => serializer.serialize_unit_variant("PowerOnBootOption", 1u32, "disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the different types of operations for guest agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningAction")]
pub enum ProvisioningAction {
    #[serde(rename = "install")]
    Install,
    #[serde(rename = "uninstall")]
    Uninstall,
    #[serde(rename = "repair")]
    Repair,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProvisioningAction {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProvisioningAction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProvisioningAction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Install => serializer.serialize_unit_variant("ProvisioningAction", 0u32, "install"),
            Self::Uninstall => serializer.serialize_unit_variant("ProvisioningAction", 1u32, "uninstall"),
            Self::Repair => serializer.serialize_unit_variant("ProvisioningAction", 2u32, "repair"),
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
#[doc = "Object containing updates for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcePatch {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourcePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Define the resourcePool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcePool {
    #[doc = "Defines the resource properties."]
    pub properties: ResourcePoolProperties,
    #[doc = "Gets or sets the location."]
    pub location: String,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets the name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata used by portal/tooling/etc to render different UX experiences for resources of the same type; e.g. ApiApps are a kind of Microsoft.Web/sites type.  If supported, the resource provider must validate and persist this value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl ResourcePool {
    pub fn new(properties: ResourcePoolProperties, location: String) -> Self {
        Self {
            properties,
            location,
            extended_location: None,
            system_data: None,
            tags: None,
            name: None,
            id: None,
            type_: None,
            kind: None,
        }
    }
}
#[doc = "The resource pool inventory item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcePoolInventoryItem {
    #[serde(flatten)]
    pub inventory_item_properties: InventoryItemProperties,
    #[doc = "Defines the resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<InventoryItemDetails>,
}
impl ResourcePoolInventoryItem {
    pub fn new(inventory_item_properties: InventoryItemProperties) -> Self {
        Self {
            inventory_item_properties,
            parent: None,
        }
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcePoolProperties {
    #[doc = "Gets or sets a unique identifier for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Gets or sets the ARM Id of the vCenter resource in which this resource pool resides."]
    #[serde(rename = "vCenterId", default, skip_serializing_if = "Option::is_none")]
    pub v_center_id: Option<String>,
    #[doc = "Gets or sets the vCenter MoRef (Managed Object Reference) ID for the resource pool."]
    #[serde(rename = "moRefId", default, skip_serializing_if = "Option::is_none")]
    pub mo_ref_id: Option<String>,
    #[doc = "Gets or sets the inventory Item ID for the resource pool."]
    #[serde(rename = "inventoryItemId", default, skip_serializing_if = "Option::is_none")]
    pub inventory_item_id: Option<String>,
    #[doc = "Gets or sets the vCenter Managed Object name for the resource pool."]
    #[serde(rename = "moName", default, skip_serializing_if = "Option::is_none")]
    pub mo_name: Option<String>,
    #[doc = "Gets or sets CPUSharesLevel which specifies the CPU allocation level for this pool.\r\nThis property is used in relative allocation between resource consumers."]
    #[serde(rename = "cpuSharesLevel", default, skip_serializing_if = "Option::is_none")]
    pub cpu_shares_level: Option<String>,
    #[doc = "Gets or sets CPUReservationMHz which specifies the CPU size in MHz that is guaranteed\r\nto be available."]
    #[serde(rename = "cpuReservationMHz", default, skip_serializing_if = "Option::is_none")]
    pub cpu_reservation_m_hz: Option<i64>,
    #[doc = "Gets or sets CPULimitMHz which specifies a CPU usage limit in MHz.\r\nUtilization will not exceed this limit even if there are available resources."]
    #[serde(rename = "cpuLimitMHz", default, skip_serializing_if = "Option::is_none")]
    pub cpu_limit_m_hz: Option<i64>,
    #[doc = "Gets or sets CPUSharesLevel which specifies the memory allocation level for this pool.\r\nThis property is used in relative allocation between resource consumers."]
    #[serde(rename = "memSharesLevel", default, skip_serializing_if = "Option::is_none")]
    pub mem_shares_level: Option<String>,
    #[doc = "Gets or sets MemReservationMB which specifies the guaranteed available memory in\r\nmegabytes."]
    #[serde(rename = "memReservationMB", default, skip_serializing_if = "Option::is_none")]
    pub mem_reservation_mb: Option<i64>,
    #[doc = "Gets or sets MemLimitMB specifies a memory usage limit in megabytes.\r\nUtilization will not exceed the specified limit even if there are available resources."]
    #[serde(rename = "memLimitMB", default, skip_serializing_if = "Option::is_none")]
    pub mem_limit_mb: Option<i64>,
    #[doc = "Gets the name of the corresponding resource in Kubernetes."]
    #[serde(rename = "customResourceName", default, skip_serializing_if = "Option::is_none")]
    pub custom_resource_name: Option<String>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<ResourceStatus>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ResourcePoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of ResourcePools."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcePoolsList {
    #[doc = "Url to follow for getting next page of ResourcePools."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of ResourcePools"]
    pub value: Vec<ResourcePool>,
}
impl azure_core::Continuable for ResourcePoolsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourcePoolsList {
    pub fn new(value: Vec<ResourcePool>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "The current deployment state of resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceProvisioningState")]
pub enum ResourceProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Provisioning,
    Updating,
    Deleting,
    Accepted,
    Created,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("ResourceProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ResourceProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ResourceProvisioningState", 2u32, "Canceled"),
            Self::Provisioning => serializer.serialize_unit_variant("ResourceProvisioningState", 3u32, "Provisioning"),
            Self::Updating => serializer.serialize_unit_variant("ResourceProvisioningState", 4u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ResourceProvisioningState", 5u32, "Deleting"),
            Self::Accepted => serializer.serialize_unit_variant("ResourceProvisioningState", 6u32, "Accepted"),
            Self::Created => serializer.serialize_unit_variant("ResourceProvisioningState", 7u32, "Created"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource status information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceStatus {
    #[doc = "The type of the condition."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Status of the condition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The reason for the condition's status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "A human readable message indicating details about the status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Severity with which to treat failures of this type of condition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "The last update time for this condition."]
    #[serde(rename = "lastUpdatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_at: Option<time::OffsetDateTime>,
}
impl ResourceStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the different types of SCSI controllers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScsiControllerType")]
pub enum ScsiControllerType {
    #[serde(rename = "lsilogic")]
    Lsilogic,
    #[serde(rename = "buslogic")]
    Buslogic,
    #[serde(rename = "pvscsi")]
    Pvscsi,
    #[serde(rename = "lsilogicsas")]
    Lsilogicsas,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScsiControllerType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScsiControllerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScsiControllerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Lsilogic => serializer.serialize_unit_variant("ScsiControllerType", 0u32, "lsilogic"),
            Self::Buslogic => serializer.serialize_unit_variant("ScsiControllerType", 1u32, "buslogic"),
            Self::Pvscsi => serializer.serialize_unit_variant("ScsiControllerType", 2u32, "pvscsi"),
            Self::Lsilogicsas => serializer.serialize_unit_variant("ScsiControllerType", 3u32, "lsilogicsas"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies the Security profile settings for the virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityProfile {
    #[doc = "Specifies the security settings like secure boot used while creating the virtual machine."]
    #[serde(rename = "uefiSettings", default, skip_serializing_if = "Option::is_none")]
    pub uefi_settings: Option<UefiSettings>,
}
impl SecurityProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the stop action properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StopVirtualMachineOptions {
    #[doc = "Gets or sets a value indicating whether to request non-graceful VM shutdown. True value for this flag indicates non-graceful shutdown whereas false indicates otherwise. Defaults to false."]
    #[serde(rename = "skipShutdown", default, skip_serializing_if = "Option::is_none")]
    pub skip_shutdown: Option<bool>,
}
impl StopVirtualMachineOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[doc = "Gets or sets the list of virtual disks associated with the virtual machine."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<VirtualDisk>,
    #[doc = "Gets or sets the list of virtual SCSI controllers associated with the virtual machine."]
    #[serde(rename = "scsiControllers", default, skip_serializing_if = "Vec::is_empty")]
    pub scsi_controllers: Vec<VirtualScsiController>,
}
impl StorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource update properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfileUpdate {
    #[doc = "Gets or sets the list of virtual disks associated with the virtual machine."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<VirtualDiskUpdate>,
}
impl StorageProfileUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the security settings like secure boot used while creating the virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UefiSettings {
    #[doc = "Specifies whether secure boot should be enabled on the virtual machine."]
    #[serde(rename = "secureBootEnabled", default, skip_serializing_if = "Option::is_none")]
    pub secure_boot_enabled: Option<bool>,
}
impl UefiSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the vCenter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VCenter {
    #[doc = "Defines the resource properties."]
    pub properties: VCenterProperties,
    #[doc = "Gets or sets the location."]
    pub location: String,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets the name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata used by portal/tooling/etc to render different UX experiences for resources of the same type; e.g. ApiApps are a kind of Microsoft.Web/sites type.  If supported, the resource provider must validate and persist this value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl VCenter {
    pub fn new(properties: VCenterProperties, location: String) -> Self {
        Self {
            properties,
            location,
            extended_location: None,
            system_data: None,
            tags: None,
            name: None,
            id: None,
            type_: None,
            kind: None,
        }
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VCenterProperties {
    #[doc = "Gets or sets a unique identifier for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Gets or sets the FQDN/IPAddress of the vCenter."]
    pub fqdn: String,
    #[doc = "Gets or sets the port of the vCenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "Gets or sets the version of the vCenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets or sets the instance UUID of the vCenter."]
    #[serde(rename = "instanceUuid", default, skip_serializing_if = "Option::is_none")]
    pub instance_uuid: Option<String>,
    #[doc = "Gets or sets the connection status to the vCenter."]
    #[serde(rename = "connectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<String>,
    #[doc = "Gets the name of the corresponding resource in Kubernetes."]
    #[serde(rename = "customResourceName", default, skip_serializing_if = "Option::is_none")]
    pub custom_resource_name: Option<String>,
    #[doc = "Username / Password Credentials to connect to vcenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<ViCredential>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<ResourceStatus>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl VCenterProperties {
    pub fn new(fqdn: String) -> Self {
        Self {
            uuid: None,
            fqdn,
            port: None,
            version: None,
            instance_uuid: None,
            connection_status: None,
            custom_resource_name: None,
            credentials: None,
            statuses: Vec::new(),
            provisioning_state: None,
        }
    }
}
#[doc = "List of VCenters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VCentersList {
    #[doc = "Url to follow for getting next page of VCenters."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of VCenters"]
    pub value: Vec<VCenter>,
}
impl azure_core::Continuable for VCentersList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VCentersList {
    pub fn new(value: Vec<VCenter>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Username / Password Credentials to connect to vcenter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ViCredential {
    #[doc = "Gets or sets username to connect with the vCenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "Gets or sets the password to connect with the vCenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl ViCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual disk model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualDisk {
    #[doc = "Gets or sets the name of the virtual disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the label of the virtual disk in vCenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Gets or sets the disk object id."]
    #[serde(rename = "diskObjectId", default, skip_serializing_if = "Option::is_none")]
    pub disk_object_id: Option<String>,
    #[doc = "Gets or sets the disk total size."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "Gets or sets the device key value."]
    #[serde(rename = "deviceKey", default, skip_serializing_if = "Option::is_none")]
    pub device_key: Option<i32>,
    #[doc = "Defines the different types of disk modes."]
    #[serde(rename = "diskMode", default, skip_serializing_if = "Option::is_none")]
    pub disk_mode: Option<DiskMode>,
    #[doc = "Gets or sets the controller id."]
    #[serde(rename = "controllerKey", default, skip_serializing_if = "Option::is_none")]
    pub controller_key: Option<i32>,
    #[doc = "Gets or sets the unit number of the disk on the controller."]
    #[serde(rename = "unitNumber", default, skip_serializing_if = "Option::is_none")]
    pub unit_number: Option<i32>,
    #[doc = "Gets or sets the device name."]
    #[serde(rename = "deviceName", default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[doc = "Defines the different types of disks."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<DiskType>,
}
impl VirtualDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the virtual disk update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualDiskUpdate {
    #[doc = "Gets or sets the name of the virtual disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the disk total size."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "Gets or sets the device key value."]
    #[serde(rename = "deviceKey", default, skip_serializing_if = "Option::is_none")]
    pub device_key: Option<i32>,
    #[doc = "Defines the different types of disk modes."]
    #[serde(rename = "diskMode", default, skip_serializing_if = "Option::is_none")]
    pub disk_mode: Option<DiskMode>,
    #[doc = "Gets or sets the controller id."]
    #[serde(rename = "controllerKey", default, skip_serializing_if = "Option::is_none")]
    pub controller_key: Option<i32>,
    #[doc = "Gets or sets the unit number of the disk on the controller."]
    #[serde(rename = "unitNumber", default, skip_serializing_if = "Option::is_none")]
    pub unit_number: Option<i32>,
    #[doc = "Gets or sets the device name."]
    #[serde(rename = "deviceName", default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[doc = "Defines the different types of disks."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<DiskType>,
}
impl VirtualDiskUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Define the virtualMachine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachine {
    #[doc = "Defines the resource properties."]
    pub properties: VirtualMachineProperties,
    #[doc = "Gets or sets the location."]
    pub location: String,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets the name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata used by portal/tooling/etc to render different UX experiences for resources of the same type; e.g. ApiApps are a kind of Microsoft.Web/sites type.  If supported, the resource provider must validate and persist this value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Managed service identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl VirtualMachine {
    pub fn new(properties: VirtualMachineProperties, location: String) -> Self {
        Self {
            properties,
            location,
            extended_location: None,
            system_data: None,
            tags: None,
            name: None,
            id: None,
            type_: None,
            kind: None,
            identity: None,
        }
    }
}
#[doc = "Describes the properties of an AssessPatches result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineAssessPatchesResult {
    #[doc = "The overall success or failure status of the operation. It remains \"InProgress\" until the operation completes. At that point it will become \"Unknown\", \"Failed\", \"Succeeded\", or \"CompletedWithWarnings.\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<virtual_machine_assess_patches_result::Status>,
    #[doc = "The activity ID of the operation that produced this result."]
    #[serde(rename = "assessmentActivityId", default, skip_serializing_if = "Option::is_none")]
    pub assessment_activity_id: Option<String>,
    #[doc = "The overall reboot status of the VM. It will be true when partially installed patches require a reboot to complete installation but the reboot has not yet occurred."]
    #[serde(rename = "rebootPending", default, skip_serializing_if = "Option::is_none")]
    pub reboot_pending: Option<bool>,
    #[doc = "Summarization of patches available for installation on the machine by classification."]
    #[serde(rename = "availablePatchCountByClassification", default, skip_serializing_if = "Option::is_none")]
    pub available_patch_count_by_classification: Option<AvailablePatchCountByClassification>,
    #[doc = "The UTC timestamp when the operation began."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "The UTC timestamp when the operation finished."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Indicates if operation was triggered by user or by platform."]
    #[serde(rename = "startedBy", default, skip_serializing_if = "Option::is_none")]
    pub started_by: Option<virtual_machine_assess_patches_result::StartedBy>,
    #[doc = "Specifies the patch service used for the operation."]
    #[serde(rename = "patchServiceUsed", default, skip_serializing_if = "Option::is_none")]
    pub patch_service_used: Option<virtual_machine_assess_patches_result::PatchServiceUsed>,
    #[doc = "The operating system type of the machine."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<virtual_machine_assess_patches_result::OsType>,
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetail>,
}
impl VirtualMachineAssessPatchesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_assess_patches_result {
    use super::*;
    #[doc = "The overall success or failure status of the operation. It remains \"InProgress\" until the operation completes. At that point it will become \"Unknown\", \"Failed\", \"Succeeded\", or \"CompletedWithWarnings.\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Unknown,
        InProgress,
        Failed,
        Succeeded,
        CompletedWithWarnings,
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
                Self::Unknown => serializer.serialize_unit_variant("Status", 0u32, "Unknown"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 3u32, "Succeeded"),
                Self::CompletedWithWarnings => serializer.serialize_unit_variant("Status", 4u32, "CompletedWithWarnings"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates if operation was triggered by user or by platform."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StartedBy")]
    pub enum StartedBy {
        User,
        Platform,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StartedBy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StartedBy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StartedBy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("StartedBy", 0u32, "User"),
                Self::Platform => serializer.serialize_unit_variant("StartedBy", 1u32, "Platform"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the patch service used for the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PatchServiceUsed")]
    pub enum PatchServiceUsed {
        Unknown,
        #[serde(rename = "WU")]
        Wu,
        #[serde(rename = "WU_WSUS")]
        WuWsus,
        #[serde(rename = "YUM")]
        Yum,
        #[serde(rename = "APT")]
        Apt,
        Zypper,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PatchServiceUsed {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PatchServiceUsed {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PatchServiceUsed {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("PatchServiceUsed", 0u32, "Unknown"),
                Self::Wu => serializer.serialize_unit_variant("PatchServiceUsed", 1u32, "WU"),
                Self::WuWsus => serializer.serialize_unit_variant("PatchServiceUsed", 2u32, "WU_WSUS"),
                Self::Yum => serializer.serialize_unit_variant("PatchServiceUsed", 3u32, "YUM"),
                Self::Apt => serializer.serialize_unit_variant("PatchServiceUsed", 4u32, "APT"),
                Self::Zypper => serializer.serialize_unit_variant("PatchServiceUsed", 5u32, "Zypper"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The operating system type of the machine."]
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
}
#[doc = "Input for InstallPatches as directly received by the API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineInstallPatchesParameters {
    #[doc = "Specifies the maximum amount of time that the operation will run. It must be an ISO 8601-compliant duration string such as PT4H (4 hours)"]
    #[serde(rename = "maximumDuration")]
    pub maximum_duration: String,
    #[doc = "Defines when it is acceptable to reboot a VM during a software update operation."]
    #[serde(rename = "rebootSetting")]
    pub reboot_setting: virtual_machine_install_patches_parameters::RebootSetting,
    #[doc = "Input for InstallPatches on a Windows VM, as directly received by the API"]
    #[serde(rename = "windowsParameters", default, skip_serializing_if = "Option::is_none")]
    pub windows_parameters: Option<WindowsParameters>,
    #[doc = "Input for InstallPatches on a Linux VM, as directly received by the API"]
    #[serde(rename = "linuxParameters", default, skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,
}
impl VirtualMachineInstallPatchesParameters {
    pub fn new(maximum_duration: String, reboot_setting: virtual_machine_install_patches_parameters::RebootSetting) -> Self {
        Self {
            maximum_duration,
            reboot_setting,
            windows_parameters: None,
            linux_parameters: None,
        }
    }
}
pub mod virtual_machine_install_patches_parameters {
    use super::*;
    #[doc = "Defines when it is acceptable to reboot a VM during a software update operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RebootSetting")]
    pub enum RebootSetting {
        IfRequired,
        Never,
        Always,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RebootSetting {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RebootSetting {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RebootSetting {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IfRequired => serializer.serialize_unit_variant("RebootSetting", 0u32, "IfRequired"),
                Self::Never => serializer.serialize_unit_variant("RebootSetting", 1u32, "Never"),
                Self::Always => serializer.serialize_unit_variant("RebootSetting", 2u32, "Always"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The result summary of an installation operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineInstallPatchesResult {
    #[doc = "The overall success or failure status of the operation. It remains \"InProgress\" until the operation completes. At that point it will become \"Failed\", \"Succeeded\", \"Unknown\" or \"CompletedWithWarnings.\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<virtual_machine_install_patches_result::Status>,
    #[doc = "The activity ID of the operation that produced this result."]
    #[serde(rename = "installationActivityId", default, skip_serializing_if = "Option::is_none")]
    pub installation_activity_id: Option<String>,
    #[doc = "The reboot state of the VM following completion of the operation."]
    #[serde(rename = "rebootStatus", default, skip_serializing_if = "Option::is_none")]
    pub reboot_status: Option<virtual_machine_install_patches_result::RebootStatus>,
    #[doc = "Whether the operation ran out of time before it completed all its intended actions."]
    #[serde(rename = "maintenanceWindowExceeded", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window_exceeded: Option<bool>,
    #[doc = "The number of patches that were not installed due to the user blocking their installation."]
    #[serde(rename = "excludedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub excluded_patch_count: Option<i32>,
    #[doc = "The number of patches that were detected as available for install, but did not meet the operation's criteria."]
    #[serde(rename = "notSelectedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub not_selected_patch_count: Option<i32>,
    #[doc = "The number of patches that were identified as meeting the installation criteria, but were not able to be installed. Typically this happens when maintenanceWindowExceeded == true."]
    #[serde(rename = "pendingPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub pending_patch_count: Option<i32>,
    #[doc = "The number of patches successfully installed."]
    #[serde(rename = "installedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub installed_patch_count: Option<i32>,
    #[doc = "The number of patches that could not be installed due to some issue. See errors for details."]
    #[serde(rename = "failedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_patch_count: Option<i32>,
    #[doc = "The UTC timestamp when the operation began."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "The UTC timestamp when the operation finished."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Indicates if operation was triggered by user or by platform."]
    #[serde(rename = "startedBy", default, skip_serializing_if = "Option::is_none")]
    pub started_by: Option<virtual_machine_install_patches_result::StartedBy>,
    #[doc = "Specifies the patch service used for the operation."]
    #[serde(rename = "patchServiceUsed", default, skip_serializing_if = "Option::is_none")]
    pub patch_service_used: Option<virtual_machine_install_patches_result::PatchServiceUsed>,
    #[doc = "The operating system type of the machine."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<virtual_machine_install_patches_result::OsType>,
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetail>,
}
impl VirtualMachineInstallPatchesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_install_patches_result {
    use super::*;
    #[doc = "The overall success or failure status of the operation. It remains \"InProgress\" until the operation completes. At that point it will become \"Failed\", \"Succeeded\", \"Unknown\" or \"CompletedWithWarnings.\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Unknown,
        InProgress,
        Failed,
        Succeeded,
        CompletedWithWarnings,
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
                Self::Unknown => serializer.serialize_unit_variant("Status", 0u32, "Unknown"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 3u32, "Succeeded"),
                Self::CompletedWithWarnings => serializer.serialize_unit_variant("Status", 4u32, "CompletedWithWarnings"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The reboot state of the VM following completion of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RebootStatus")]
    pub enum RebootStatus {
        Unknown,
        NotNeeded,
        Required,
        Started,
        Failed,
        Completed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RebootStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RebootStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RebootStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("RebootStatus", 0u32, "Unknown"),
                Self::NotNeeded => serializer.serialize_unit_variant("RebootStatus", 1u32, "NotNeeded"),
                Self::Required => serializer.serialize_unit_variant("RebootStatus", 2u32, "Required"),
                Self::Started => serializer.serialize_unit_variant("RebootStatus", 3u32, "Started"),
                Self::Failed => serializer.serialize_unit_variant("RebootStatus", 4u32, "Failed"),
                Self::Completed => serializer.serialize_unit_variant("RebootStatus", 5u32, "Completed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates if operation was triggered by user or by platform."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StartedBy")]
    pub enum StartedBy {
        User,
        Platform,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StartedBy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StartedBy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StartedBy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("StartedBy", 0u32, "User"),
                Self::Platform => serializer.serialize_unit_variant("StartedBy", 1u32, "Platform"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the patch service used for the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PatchServiceUsed")]
    pub enum PatchServiceUsed {
        Unknown,
        #[serde(rename = "WU")]
        Wu,
        #[serde(rename = "WU_WSUS")]
        WuWsus,
        #[serde(rename = "YUM")]
        Yum,
        #[serde(rename = "APT")]
        Apt,
        Zypper,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PatchServiceUsed {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PatchServiceUsed {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PatchServiceUsed {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("PatchServiceUsed", 0u32, "Unknown"),
                Self::Wu => serializer.serialize_unit_variant("PatchServiceUsed", 1u32, "WU"),
                Self::WuWsus => serializer.serialize_unit_variant("PatchServiceUsed", 2u32, "WU_WSUS"),
                Self::Yum => serializer.serialize_unit_variant("PatchServiceUsed", 3u32, "YUM"),
                Self::Apt => serializer.serialize_unit_variant("PatchServiceUsed", 4u32, "APT"),
                Self::Zypper => serializer.serialize_unit_variant("PatchServiceUsed", 5u32, "Zypper"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The operating system type of the machine."]
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
}
#[doc = "The VM inventory item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineInventoryItem {
    #[serde(flatten)]
    pub inventory_item_properties: InventoryItemProperties,
    #[doc = "Defines the different types of VM guest operating systems."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "Gets or sets os name."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "Gets or sets the nic ip addresses."]
    #[serde(rename = "ipAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_addresses: Vec<String>,
    #[doc = "Gets or sets the folder path of the vm."]
    #[serde(rename = "folderPath", default, skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<String>,
    #[doc = "Defines the resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<InventoryItemDetails>,
    #[doc = "Defines the resource properties."]
    #[serde(rename = "resourcePool", default, skip_serializing_if = "Option::is_none")]
    pub resource_pool: Option<InventoryItemDetails>,
    #[doc = "Gets or sets the instance uuid of the vm."]
    #[serde(rename = "instanceUuid", default, skip_serializing_if = "Option::is_none")]
    pub instance_uuid: Option<String>,
    #[doc = "Gets or sets the SMBIOS UUID of the vm."]
    #[serde(rename = "smbiosUuid", default, skip_serializing_if = "Option::is_none")]
    pub smbios_uuid: Option<String>,
    #[doc = "Gets the power state of the virtual machine."]
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<String>,
    #[doc = "Gets or sets the current running status of VMware Tools running in the guest operating system."]
    #[serde(rename = "toolsRunningStatus", default, skip_serializing_if = "Option::is_none")]
    pub tools_running_status: Option<String>,
    #[doc = "Gets or sets the current version status of VMware Tools installed in the guest operating system."]
    #[serde(rename = "toolsVersionStatus", default, skip_serializing_if = "Option::is_none")]
    pub tools_version_status: Option<String>,
    #[doc = "Gets or sets the current version of VMware Tools."]
    #[serde(rename = "toolsVersion", default, skip_serializing_if = "Option::is_none")]
    pub tools_version: Option<String>,
}
impl VirtualMachineInventoryItem {
    pub fn new(inventory_item_properties: InventoryItemProperties) -> Self {
        Self {
            inventory_item_properties,
            os_type: None,
            os_name: None,
            ip_addresses: Vec::new(),
            folder_path: None,
            host: None,
            resource_pool: None,
            instance_uuid: None,
            smbios_uuid: None,
            power_state: None,
            tools_running_status: None,
            tools_version_status: None,
            tools_version: None,
        }
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineProperties {
    #[doc = "Gets or sets the ARM Id of the resourcePool resource on which this virtual machine will\r\ndeploy."]
    #[serde(rename = "resourcePoolId", default, skip_serializing_if = "Option::is_none")]
    pub resource_pool_id: Option<String>,
    #[doc = "Gets or sets the ARM Id of the template resource to deploy the virtual machine."]
    #[serde(rename = "templateId", default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[doc = "Gets or sets the ARM Id of the vCenter resource in which this resource pool resides."]
    #[serde(rename = "vCenterId", default, skip_serializing_if = "Option::is_none")]
    pub v_center_id: Option<String>,
    #[doc = "Defines the resource properties."]
    #[serde(rename = "placementProfile", default, skip_serializing_if = "Option::is_none")]
    pub placement_profile: Option<PlacementProfile>,
    #[doc = "Defines the resource properties."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "Defines the resource properties."]
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[doc = "Defines the resource properties."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    #[doc = "Defines the resource properties."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "Defines the resource properties."]
    #[serde(rename = "guestAgentProfile", default, skip_serializing_if = "Option::is_none")]
    pub guest_agent_profile: Option<GuestAgentProfile>,
    #[doc = "Specifies the Security profile settings for the virtual machine."]
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<SecurityProfile>,
    #[doc = "Gets or sets the vCenter MoRef (Managed Object Reference) ID for the virtual machine."]
    #[serde(rename = "moRefId", default, skip_serializing_if = "Option::is_none")]
    pub mo_ref_id: Option<String>,
    #[doc = "Gets or sets the inventory Item ID for the virtual machine."]
    #[serde(rename = "inventoryItemId", default, skip_serializing_if = "Option::is_none")]
    pub inventory_item_id: Option<String>,
    #[doc = "Gets or sets the vCenter Managed Object name for the virtual machine."]
    #[serde(rename = "moName", default, skip_serializing_if = "Option::is_none")]
    pub mo_name: Option<String>,
    #[doc = "Gets or sets the folder path of the vm."]
    #[serde(rename = "folderPath", default, skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<String>,
    #[doc = "Gets or sets the instance uuid of the vm."]
    #[serde(rename = "instanceUuid", default, skip_serializing_if = "Option::is_none")]
    pub instance_uuid: Option<String>,
    #[doc = "Gets or sets the SMBIOS UUID of the vm."]
    #[serde(rename = "smbiosUuid", default, skip_serializing_if = "Option::is_none")]
    pub smbios_uuid: Option<String>,
    #[doc = "Firmware type"]
    #[serde(rename = "firmwareType", default, skip_serializing_if = "Option::is_none")]
    pub firmware_type: Option<FirmwareType>,
    #[doc = "Gets the power state of the virtual machine."]
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<String>,
    #[doc = "Gets the name of the corresponding resource in Kubernetes."]
    #[serde(rename = "customResourceName", default, skip_serializing_if = "Option::is_none")]
    pub custom_resource_name: Option<String>,
    #[doc = "Gets or sets a unique identifier for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<ResourceStatus>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Gets or sets a unique identifier for the vm resource."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
}
impl VirtualMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Define the virtualMachineTemplate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineTemplate {
    #[doc = "Defines the resource properties."]
    pub properties: VirtualMachineTemplateProperties,
    #[doc = "Gets or sets the location."]
    pub location: String,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets the name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata used by portal/tooling/etc to render different UX experiences for resources of the same type; e.g. ApiApps are a kind of Microsoft.Web/sites type.  If supported, the resource provider must validate and persist this value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl VirtualMachineTemplate {
    pub fn new(properties: VirtualMachineTemplateProperties, location: String) -> Self {
        Self {
            properties,
            location,
            extended_location: None,
            system_data: None,
            tags: None,
            name: None,
            id: None,
            type_: None,
            kind: None,
        }
    }
}
#[doc = "The VM Template inventory item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineTemplateInventoryItem {
    #[serde(flatten)]
    pub inventory_item_properties: InventoryItemProperties,
    #[doc = "Gets or sets memory size in MBs for the template."]
    #[serde(rename = "memorySizeMB", default, skip_serializing_if = "Option::is_none")]
    pub memory_size_mb: Option<i32>,
    #[doc = "Gets or sets the number of vCPUs for the template."]
    #[serde(rename = "numCPUs", default, skip_serializing_if = "Option::is_none")]
    pub num_cp_us: Option<i32>,
    #[doc = "Gets or sets the number of cores per socket for the template.\r\nDefaults to 1 if unspecified."]
    #[serde(rename = "numCoresPerSocket", default, skip_serializing_if = "Option::is_none")]
    pub num_cores_per_socket: Option<i32>,
    #[doc = "Defines the different types of VM guest operating systems."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "Gets or sets os name."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "Gets or sets the folder path of the template."]
    #[serde(rename = "folderPath", default, skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<String>,
}
impl VirtualMachineTemplateInventoryItem {
    pub fn new(inventory_item_properties: InventoryItemProperties) -> Self {
        Self {
            inventory_item_properties,
            memory_size_mb: None,
            num_cp_us: None,
            num_cores_per_socket: None,
            os_type: None,
            os_name: None,
            folder_path: None,
        }
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineTemplateProperties {
    #[doc = "Gets or sets a unique identifier for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Gets or sets the ARM Id of the vCenter resource in which this template resides."]
    #[serde(rename = "vCenterId", default, skip_serializing_if = "Option::is_none")]
    pub v_center_id: Option<String>,
    #[doc = "Gets or sets the vCenter MoRef (Managed Object Reference) ID for the virtual machine\r\ntemplate."]
    #[serde(rename = "moRefId", default, skip_serializing_if = "Option::is_none")]
    pub mo_ref_id: Option<String>,
    #[doc = "Gets or sets the inventory Item ID for the virtual machine template."]
    #[serde(rename = "inventoryItemId", default, skip_serializing_if = "Option::is_none")]
    pub inventory_item_id: Option<String>,
    #[doc = "Gets or sets the vCenter Managed Object name for the virtual machine template."]
    #[serde(rename = "moName", default, skip_serializing_if = "Option::is_none")]
    pub mo_name: Option<String>,
    #[doc = "Gets or sets memory size in MBs for the template."]
    #[serde(rename = "memorySizeMB", default, skip_serializing_if = "Option::is_none")]
    pub memory_size_mb: Option<i32>,
    #[doc = "Gets or sets the number of vCPUs for the template."]
    #[serde(rename = "numCPUs", default, skip_serializing_if = "Option::is_none")]
    pub num_cp_us: Option<i32>,
    #[doc = "Gets or sets the number of cores per socket for the template.\r\nDefaults to 1 if unspecified."]
    #[serde(rename = "numCoresPerSocket", default, skip_serializing_if = "Option::is_none")]
    pub num_cores_per_socket: Option<i32>,
    #[doc = "Defines the different types of VM guest operating systems."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "Gets or sets os name."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "Gets or sets the folder path of the template."]
    #[serde(rename = "folderPath", default, skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<String>,
    #[doc = "Gets or sets the network interfaces of the template."]
    #[serde(rename = "networkInterfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<NetworkInterface>,
    #[doc = "Gets or sets the disks the template."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<VirtualDisk>,
    #[doc = "Gets the name of the corresponding resource in Kubernetes."]
    #[serde(rename = "customResourceName", default, skip_serializing_if = "Option::is_none")]
    pub custom_resource_name: Option<String>,
    #[doc = "Gets or sets the current version status of VMware Tools installed in the guest operating system."]
    #[serde(rename = "toolsVersionStatus", default, skip_serializing_if = "Option::is_none")]
    pub tools_version_status: Option<String>,
    #[doc = "Gets or sets the current version of VMware Tools."]
    #[serde(rename = "toolsVersion", default, skip_serializing_if = "Option::is_none")]
    pub tools_version: Option<String>,
    #[doc = "Firmware type"]
    #[serde(rename = "firmwareType", default, skip_serializing_if = "Option::is_none")]
    pub firmware_type: Option<FirmwareType>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<ResourceStatus>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl VirtualMachineTemplateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of VirtualMachineTemplates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineTemplatesList {
    #[doc = "Url to follow for getting next page of VirtualMachineTemplates."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of VirtualMachineTemplates"]
    pub value: Vec<VirtualMachineTemplate>,
}
impl azure_core::Continuable for VirtualMachineTemplatesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualMachineTemplatesList {
    pub fn new(value: Vec<VirtualMachineTemplate>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Defines the virtualMachineUpdate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineUpdate {
    #[doc = "Defines the resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineUpdateProperties>,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Managed service identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl VirtualMachineUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineUpdateProperties {
    #[doc = "Defines the resource properties."]
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[doc = "Defines the os update properties."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfileUpdate>,
    #[doc = "Defines the resource update properties."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfileUpdate>,
    #[doc = "Defines the update resource properties."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfileUpdate>,
}
impl VirtualMachineUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of VirtualMachines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachinesList {
    #[doc = "Url to follow for getting next page of VirtualMachines."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of VirtualMachines"]
    pub value: Vec<VirtualMachine>,
}
impl azure_core::Continuable for VirtualMachinesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualMachinesList {
    pub fn new(value: Vec<VirtualMachine>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Define the virtualNetwork."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetwork {
    #[doc = "Defines the resource properties."]
    pub properties: VirtualNetworkProperties,
    #[doc = "Gets or sets the location."]
    pub location: String,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets the name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata used by portal/tooling/etc to render different UX experiences for resources of the same type; e.g. ApiApps are a kind of Microsoft.Web/sites type.  If supported, the resource provider must validate and persist this value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl VirtualNetwork {
    pub fn new(properties: VirtualNetworkProperties, location: String) -> Self {
        Self {
            properties,
            location,
            extended_location: None,
            system_data: None,
            tags: None,
            name: None,
            id: None,
            type_: None,
            kind: None,
        }
    }
}
#[doc = "The Virtual network inventory item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkInventoryItem {
    #[serde(flatten)]
    pub inventory_item_properties: InventoryItemProperties,
}
impl VirtualNetworkInventoryItem {
    pub fn new(inventory_item_properties: InventoryItemProperties) -> Self {
        Self { inventory_item_properties }
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkProperties {
    #[doc = "Gets or sets a unique identifier for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Gets or sets the ARM Id of the vCenter resource in which this template resides."]
    #[serde(rename = "vCenterId", default, skip_serializing_if = "Option::is_none")]
    pub v_center_id: Option<String>,
    #[doc = "Gets or sets the vCenter MoRef (Managed Object Reference) ID for the virtual network."]
    #[serde(rename = "moRefId", default, skip_serializing_if = "Option::is_none")]
    pub mo_ref_id: Option<String>,
    #[doc = "Gets or sets the inventory Item ID for the virtual network."]
    #[serde(rename = "inventoryItemId", default, skip_serializing_if = "Option::is_none")]
    pub inventory_item_id: Option<String>,
    #[doc = "Gets or sets the vCenter Managed Object name for the virtual network."]
    #[serde(rename = "moName", default, skip_serializing_if = "Option::is_none")]
    pub mo_name: Option<String>,
    #[doc = "Gets the name of the corresponding resource in Kubernetes."]
    #[serde(rename = "customResourceName", default, skip_serializing_if = "Option::is_none")]
    pub custom_resource_name: Option<String>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<ResourceStatus>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl VirtualNetworkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of VirtualNetworks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworksList {
    #[doc = "Url to follow for getting next page of VirtualNetworks."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of VirtualNetworks"]
    pub value: Vec<VirtualNetwork>,
}
impl azure_core::Continuable for VirtualNetworksList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualNetworksList {
    pub fn new(value: Vec<VirtualNetwork>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "This data object type contains the properties of a SCSI controller device attached to a virtual machine that is reported by the controller."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualScsiController {
    #[doc = "Defines the different types of SCSI controllers."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ScsiControllerType>,
    #[doc = "Gets or sets the key of the controller."]
    #[serde(rename = "controllerKey", default, skip_serializing_if = "Option::is_none")]
    pub controller_key: Option<i32>,
    #[doc = "Gets or sets the bus number of the controller."]
    #[serde(rename = "busNumber", default, skip_serializing_if = "Option::is_none")]
    pub bus_number: Option<i32>,
    #[doc = "Gets or sets the SCSI controller unit number."]
    #[serde(rename = "scsiCtlrUnitNumber", default, skip_serializing_if = "Option::is_none")]
    pub scsi_ctlr_unit_number: Option<i32>,
    #[doc = "Defines the sharing mode for sharing the SCSI bus."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sharing: Option<VirtualScsiSharing>,
}
impl VirtualScsiController {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the sharing mode for sharing the SCSI bus."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VirtualScsiSharing")]
pub enum VirtualScsiSharing {
    #[serde(rename = "noSharing")]
    NoSharing,
    #[serde(rename = "physicalSharing")]
    PhysicalSharing,
    #[serde(rename = "virtualSharing")]
    VirtualSharing,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VirtualScsiSharing {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VirtualScsiSharing {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VirtualScsiSharing {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NoSharing => serializer.serialize_unit_variant("VirtualScsiSharing", 0u32, "noSharing"),
            Self::PhysicalSharing => serializer.serialize_unit_variant("VirtualScsiSharing", 1u32, "physicalSharing"),
            Self::VirtualSharing => serializer.serialize_unit_variant("VirtualScsiSharing", 2u32, "virtualSharing"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Input for InstallPatches on a Windows VM, as directly received by the API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsParameters {
    #[doc = "The update classifications to select when installing patches for Windows."]
    #[serde(rename = "classificationsToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub classifications_to_include: Vec<String>,
    #[doc = "Kbs to include in the patch operation"]
    #[serde(rename = "kbNumbersToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub kb_numbers_to_include: Vec<String>,
    #[doc = "Kbs to exclude in the patch operation"]
    #[serde(rename = "kbNumbersToExclude", default, skip_serializing_if = "Vec::is_empty")]
    pub kb_numbers_to_exclude: Vec<String>,
    #[doc = "Filters out Kbs that don't have an InstallationRebootBehavior of 'NeverReboots' when this is set to true."]
    #[serde(rename = "excludeKbsRequiringReboot", default, skip_serializing_if = "Option::is_none")]
    pub exclude_kbs_requiring_reboot: Option<bool>,
    #[doc = "This is used to install patches that were published on or before this given max published date."]
    #[serde(rename = "maxPatchPublishDate", default, with = "azure_core::date::rfc3339::option")]
    pub max_patch_publish_date: Option<time::OffsetDateTime>,
}
impl WindowsParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation provided by provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is data action or not."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Properties of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Properties of the operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Provider name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Lists the operations available."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsList {
    #[doc = "Url to follow for getting next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of operations"]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationsList {
    pub fn new(value: Vec<Operation>) -> Self {
        Self { next_link: None, value }
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

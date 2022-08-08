#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
pub type N5Qi = i32;
pub type N5QiPriorityLevel = i32;
pub type N5QiPriorityLevelRm = i32;
pub type N5QiRm = i32;
#[doc = "Aggregate Maximum Bit Rate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ambr {
    #[doc = "Bit rate."]
    pub uplink: BitRate,
    #[doc = "Bit rate."]
    pub downlink: BitRate,
}
impl Ambr {
    pub fn new(uplink: BitRate, downlink: BitRate) -> Self {
        Self { uplink, downlink }
    }
}
#[doc = "Aggregate Maximum Bit Rate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmbrRm {
    #[doc = "Bit rate."]
    pub uplink: BitRate,
    #[doc = "Bit rate."]
    pub downlink: BitRate,
}
impl AmbrRm {
    pub fn new(uplink: BitRate, downlink: BitRate) -> Self {
        Self { uplink, downlink }
    }
}
#[doc = "Allocation and Retention Priority (ARP) parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Arp {
    #[doc = "ARP priority level."]
    #[serde(rename = "priorityLevel")]
    pub priority_level: ArpPriorityLevel,
    #[doc = "Preemption capability."]
    #[serde(rename = "preemptCap")]
    pub preempt_cap: PreemptionCapability,
    #[doc = "Preemption vulnerability."]
    #[serde(rename = "preemptVuln")]
    pub preempt_vuln: PreemptionVulnerability,
}
impl Arp {
    pub fn new(priority_level: ArpPriorityLevel, preempt_cap: PreemptionCapability, preempt_vuln: PreemptionVulnerability) -> Self {
        Self {
            priority_level,
            preempt_cap,
            preempt_vuln,
        }
    }
}
pub type ArpPriorityLevel = i32;
pub type ArpPriorityLevelRm = i32;
#[doc = "Attached data network resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachedDataNetwork {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Data network properties."]
    pub properties: AttachedDataNetworkPropertiesFormat,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl AttachedDataNetwork {
    pub fn new(tracked_resource: TrackedResource, properties: AttachedDataNetworkPropertiesFormat) -> Self {
        Self {
            tracked_resource,
            properties,
            system_data: None,
        }
    }
}
#[doc = "Response for attached data network API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachedDataNetworkListResult {
    #[doc = "A list of data networks in a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AttachedDataNetwork>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AttachedDataNetworkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AttachedDataNetworkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data network properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachedDataNetworkPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Interface properties"]
    #[serde(rename = "userPlaneDataInterface")]
    pub user_plane_data_interface: InterfaceProperties,
    #[doc = "The Network Address and Port Translation settings to use for the attached data network."]
    #[serde(rename = "naptConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub napt_configuration: Option<NaptConfiguration>,
    #[doc = "The user equipment address pool prefixes for the attached data network that are dynamically assigned by the core to UEs when they set up a PDU session.\nAt least one of userEquipmentAddressPoolPrefix and userEquipmentStaticAddressPoolPrefix must be defined. If both are defined then they must be the same size."]
    #[serde(rename = "userEquipmentAddressPoolPrefix", default, skip_serializing_if = "Vec::is_empty")]
    pub user_equipment_address_pool_prefix: Vec<Ipv4AddrMask>,
    #[doc = "The user equipment address pool prefixes for the attached data network that are statically assigned by the core to UEs when they set up a PDU session.\nThe mapping of static IP to sim is configured in staticIpConfiguration on the sim resource.\nAt least one of userEquipmentAddressPoolPrefix and userEquipmentStaticAddressPoolPrefix must be defined. If both are defined then they must be the same size."]
    #[serde(rename = "userEquipmentStaticAddressPoolPrefix", default, skip_serializing_if = "Vec::is_empty")]
    pub user_equipment_static_address_pool_prefix: Vec<Ipv4AddrMask>,
}
impl AttachedDataNetworkPropertiesFormat {
    pub fn new(user_plane_data_interface: InterfaceProperties) -> Self {
        Self {
            provisioning_state: None,
            user_plane_data_interface,
            napt_configuration: None,
            user_equipment_address_pool_prefix: Vec::new(),
            user_equipment_static_address_pool_prefix: Vec::new(),
        }
    }
}
#[doc = "Reference to an Attached Data Network resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachedDataNetworkResourceId {
    #[doc = "Attached Data Network resource ID."]
    pub id: String,
}
impl AttachedDataNetworkResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
pub type BitRate = String;
pub type BitRateRm = String;
#[doc = "Core network type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CoreNetworkType")]
pub enum CoreNetworkType {
    #[serde(rename = "5GC")]
    N5GC,
    #[serde(rename = "EPC")]
    Epc,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CoreNetworkType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CoreNetworkType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CoreNetworkType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::N5GC => serializer.serialize_unit_variant("CoreNetworkType", 0u32, "5GC"),
            Self::Epc => serializer.serialize_unit_variant("CoreNetworkType", 1u32, "EPC"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Core network type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CoreNetworkTypeRm")]
pub enum CoreNetworkTypeRm {
    #[serde(rename = "5GC")]
    N5GC,
    #[serde(rename = "EPC")]
    Epc,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CoreNetworkTypeRm {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CoreNetworkTypeRm {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CoreNetworkTypeRm {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::N5GC => serializer.serialize_unit_variant("CoreNetworkTypeRm", 0u32, "5GC"),
            Self::Epc => serializer.serialize_unit_variant("CoreNetworkTypeRm", 1u32, "EPC"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Reference to an Azure ARC custom location resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomLocationResourceId {
    #[doc = "Azure ARC custom location resource ID."]
    pub id: String,
}
impl CustomLocationResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Data network resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataNetwork {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Data network properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataNetworkPropertiesFormat>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DataNetwork {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "Settings controlling Data Network use"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataNetworkConfiguration {
    #[doc = "Reference to a Data Network resource."]
    #[serde(rename = "dataNetwork")]
    pub data_network: DataNetworkResourceId,
    #[doc = "Aggregate Maximum Bit Rate."]
    #[serde(rename = "sessionAmbr")]
    pub session_ambr: Ambr,
    #[doc = "5G QoS Identifier priority level."]
    #[serde(rename = "5qi", default, skip_serializing_if = "Option::is_none")]
    pub n5qi: Option<N5QiPriorityLevel>,
    #[doc = "ARP priority level."]
    #[serde(rename = "allocationAndRetentionPriorityLevel", default, skip_serializing_if = "Option::is_none")]
    pub allocation_and_retention_priority_level: Option<ArpPriorityLevel>,
    #[doc = "Preemption capability."]
    #[serde(rename = "preemptionCapability", default, skip_serializing_if = "Option::is_none")]
    pub preemption_capability: Option<PreemptionCapability>,
    #[doc = "Preemption vulnerability."]
    #[serde(rename = "preemptionVulnerability", default, skip_serializing_if = "Option::is_none")]
    pub preemption_vulnerability: Option<PreemptionVulnerability>,
    #[doc = "PDU session type (IPv4/IPv6)."]
    #[serde(rename = "defaultSessionType", default, skip_serializing_if = "Option::is_none")]
    pub default_session_type: Option<PduSessionType>,
    #[doc = "Allowed session types in addition to the default session type.  Must not duplicate the default session type."]
    #[serde(rename = "additionalAllowedSessionTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_allowed_session_types: Vec<PduSessionType>,
    #[doc = "List of Services that can be used as part of this Sim Policy. The list must not contain duplicate items and must contain at least one item."]
    #[serde(rename = "allowedServices")]
    pub allowed_services: Vec<ServiceResourceId>,
}
impl DataNetworkConfiguration {
    pub fn new(data_network: DataNetworkResourceId, session_ambr: Ambr, allowed_services: Vec<ServiceResourceId>) -> Self {
        Self {
            data_network,
            session_ambr,
            n5qi: None,
            allocation_and_retention_priority_level: None,
            preemption_capability: None,
            preemption_vulnerability: None,
            default_session_type: None,
            additional_allowed_session_types: Vec::new(),
            allowed_services,
        }
    }
}
#[doc = "Response for data network API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataNetworkListResult {
    #[doc = "A list of data networks in a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataNetwork>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataNetworkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataNetworkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data network properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataNetworkPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "An optional description for this data network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl DataNetworkPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to a Data Network resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataNetworkResourceId {
    #[doc = "Data Network resource ID."]
    pub id: String,
}
impl DataNetworkResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
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
#[doc = "Interface properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterfaceProperties {
    #[doc = "The logical name for this interface. This should match one of the interfaces configured on your Azure Stack Edge machine."]
    pub name: String,
    #[doc = "IPv4 address."]
    #[serde(rename = "ipv4Address", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<Ipv4Addr>,
    #[doc = "IPv4 address mask."]
    #[serde(rename = "ipv4Subnet", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_subnet: Option<Ipv4AddrMask>,
    #[doc = "IPv4 address."]
    #[serde(rename = "ipv4Gateway", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_gateway: Option<Ipv4Addr>,
}
impl InterfaceProperties {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ipv4_address: None,
            ipv4_subnet: None,
            ipv4_gateway: None,
        }
    }
}
pub type Ipv4Addr = String;
pub type Ipv4AddrMask = String;
pub type Ipv4AddrMaskRm = String;
pub type Ipv4AddrRm = String;
pub type Mcc = String;
pub type MccRm = String;
pub type Mnc = String;
pub type MncRm = String;
#[doc = "Mobile network resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MobileNetwork {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Mobile network properties."]
    pub properties: MobileNetworkPropertiesFormat,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl MobileNetwork {
    pub fn new(tracked_resource: TrackedResource, properties: MobileNetworkPropertiesFormat) -> Self {
        Self {
            tracked_resource,
            properties,
            system_data: None,
        }
    }
}
#[doc = "Response for mobile networks API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MobileNetworkListResult {
    #[doc = "A list of mobile networks in a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MobileNetwork>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MobileNetworkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MobileNetworkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Mobile network properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MobileNetworkPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Public Land Mobile Network (PLMN) ID."]
    #[serde(rename = "publicLandMobileNetworkIdentifier")]
    pub public_land_mobile_network_identifier: PlmnId,
    #[doc = "The mobile network resource identifier"]
    #[serde(rename = "serviceKey", default, skip_serializing_if = "Option::is_none")]
    pub service_key: Option<String>,
}
impl MobileNetworkPropertiesFormat {
    pub fn new(public_land_mobile_network_identifier: PlmnId) -> Self {
        Self {
            provisioning_state: None,
            public_land_mobile_network_identifier,
            service_key: None,
        }
    }
}
#[doc = "Reference to a Mobile Network resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MobileNetworkResourceId {
    #[doc = "Mobile Network resource ID."]
    pub id: String,
}
impl MobileNetworkResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "The Network Address and Port Translation settings to use for the attached data network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NaptConfiguration {
    #[doc = "Whether Network Address and Port Translation is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<NaptEnabled>,
    #[doc = "Range of port numbers to use as translated ports on each translated address.\nIf not specified and NAPT is enabled, this range defaults to 1,024 - 65,535. (Ports under 1,024 should not be used because these are special purpose ports reserved by IANA.)"]
    #[serde(rename = "portRange", default, skip_serializing_if = "Option::is_none")]
    pub port_range: Option<PortRange>,
    #[doc = "The minimum time (in seconds) that will pass before a port that was used by a closed pinhole can be recycled for use by another pinhole. All hold times must be minimum 1 second."]
    #[serde(rename = "portReuseHoldTime", default, skip_serializing_if = "Option::is_none")]
    pub port_reuse_hold_time: Option<PortReuseHoldTimes>,
    #[doc = "Maximum number of UDP and TCP pinholes that can be open simultaneously on the core interface."]
    #[serde(rename = "pinholeLimits", default, skip_serializing_if = "Option::is_none")]
    pub pinhole_limits: Option<i32>,
    #[doc = "Expiry times of inactive NAPT pinholes, in seconds. All timers must be at least 1 second."]
    #[serde(rename = "pinholeTimeouts", default, skip_serializing_if = "Option::is_none")]
    pub pinhole_timeouts: Option<PinholeTimeouts>,
}
impl NaptConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Whether Network Address and Port Translation is enabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NaptEnabled")]
pub enum NaptEnabled {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NaptEnabled {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NaptEnabled {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NaptEnabled {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("NaptEnabled", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("NaptEnabled", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for NaptEnabled {
    fn default() -> Self {
        Self::Enabled
    }
}
#[doc = "Object that describes a single Microsoft.MobileNetwork operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Indicates whether the operation applies to data-plane."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
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
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.MobileNetwork"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Registration definition, registration assignment etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "List of the operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of Microsoft.MobileNetwork operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The URL to get the next set of results."]
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
#[doc = "Packet core control plane resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PacketCoreControlPlane {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "PacketCoreControlPlane properties."]
    pub properties: PacketCoreControlPlanePropertiesFormat,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PacketCoreControlPlane {
    pub fn new(tracked_resource: TrackedResource, properties: PacketCoreControlPlanePropertiesFormat) -> Self {
        Self {
            tracked_resource,
            properties,
            system_data: None,
        }
    }
}
#[doc = "Response for packet core control planes API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PacketCoreControlPlaneListResult {
    #[doc = "A list of packet core control planes in a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PacketCoreControlPlane>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PacketCoreControlPlaneListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PacketCoreControlPlaneListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "PacketCoreControlPlane properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PacketCoreControlPlanePropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Reference to a Mobile Network resource."]
    #[serde(rename = "mobileNetwork")]
    pub mobile_network: MobileNetworkResourceId,
    #[doc = "Reference to an Azure ARC custom location resource."]
    #[serde(rename = "customLocation", default, skip_serializing_if = "Option::is_none")]
    pub custom_location: Option<CustomLocationResourceId>,
    #[doc = "Core network type."]
    #[serde(rename = "coreNetworkTechnology", default, skip_serializing_if = "Option::is_none")]
    pub core_network_technology: Option<CoreNetworkType>,
    #[doc = "The version of the packet core software that is deployed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Interface properties"]
    #[serde(rename = "controlPlaneAccessInterface")]
    pub control_plane_access_interface: InterfaceProperties,
}
impl PacketCoreControlPlanePropertiesFormat {
    pub fn new(mobile_network: MobileNetworkResourceId, control_plane_access_interface: InterfaceProperties) -> Self {
        Self {
            provisioning_state: None,
            mobile_network,
            custom_location: None,
            core_network_technology: None,
            version: None,
            control_plane_access_interface,
        }
    }
}
#[doc = "Packet core data plane resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PacketCoreDataPlane {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "PacketCoreDataPlane properties."]
    pub properties: PacketCoreDataPlanePropertiesFormat,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PacketCoreDataPlane {
    pub fn new(tracked_resource: TrackedResource, properties: PacketCoreDataPlanePropertiesFormat) -> Self {
        Self {
            tracked_resource,
            properties,
            system_data: None,
        }
    }
}
#[doc = "Response for packet core data planes API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PacketCoreDataPlaneListResult {
    #[doc = "A list of packet core data planes in a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PacketCoreDataPlane>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PacketCoreDataPlaneListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PacketCoreDataPlaneListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "PacketCoreDataPlane properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PacketCoreDataPlanePropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Interface properties"]
    #[serde(rename = "userPlaneAccessInterface")]
    pub user_plane_access_interface: InterfaceProperties,
}
impl PacketCoreDataPlanePropertiesFormat {
    pub fn new(user_plane_access_interface: InterfaceProperties) -> Self {
        Self {
            provisioning_state: None,
            user_plane_access_interface,
        }
    }
}
#[doc = "PCC rule configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PccRuleConfiguration {
    #[doc = "The name of the rule. This must be unique within the parent Service. You must not use any of the following reserved strings - `default`, `requested` or `service`."]
    #[serde(rename = "ruleName")]
    pub rule_name: String,
    #[doc = "A precedence value that is used to decide between PCC Rules when identifying the QoS values to use for a particular Sim. A lower value means a higher priority. This value should be unique among all PCC Rules configured in the Mobile Network."]
    #[serde(rename = "rulePrecedence")]
    pub rule_precedence: i32,
    #[doc = "PCC rule QoS policy"]
    #[serde(rename = "ruleQosPolicy", default, skip_serializing_if = "Option::is_none")]
    pub rule_qos_policy: Option<PccRuleQosPolicy>,
    #[doc = "Traffic control permission."]
    #[serde(rename = "trafficControl", default, skip_serializing_if = "Option::is_none")]
    pub traffic_control: Option<TrafficControlPermission>,
    #[doc = "The set of service data flow templates to use for this PCC Rule."]
    #[serde(rename = "serviceDataFlowTemplates")]
    pub service_data_flow_templates: Vec<ServiceDataFlowTemplate>,
}
impl PccRuleConfiguration {
    pub fn new(rule_name: String, rule_precedence: i32, service_data_flow_templates: Vec<ServiceDataFlowTemplate>) -> Self {
        Self {
            rule_name,
            rule_precedence,
            rule_qos_policy: None,
            traffic_control: None,
            service_data_flow_templates,
        }
    }
}
#[doc = "PCC rule QoS policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PccRuleQosPolicy {
    #[serde(flatten)]
    pub qos_policy: QosPolicy,
    #[doc = "Aggregate Maximum Bit Rate."]
    #[serde(rename = "guaranteedBitRate", default, skip_serializing_if = "Option::is_none")]
    pub guaranteed_bit_rate: Option<Ambr>,
}
impl PccRuleQosPolicy {
    pub fn new(qos_policy: QosPolicy) -> Self {
        Self {
            qos_policy,
            guaranteed_bit_rate: None,
        }
    }
}
#[doc = "PDU session type (IPv4/IPv6)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PduSessionType")]
pub enum PduSessionType {
    IPv4,
    IPv6,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PduSessionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PduSessionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PduSessionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::IPv4 => serializer.serialize_unit_variant("PduSessionType", 0u32, "IPv4"),
            Self::IPv6 => serializer.serialize_unit_variant("PduSessionType", 1u32, "IPv6"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "PDU session type (IPv4/IPv6)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PduSessionTypeRm")]
pub enum PduSessionTypeRm {
    IPv4,
    IPv6,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PduSessionTypeRm {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PduSessionTypeRm {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PduSessionTypeRm {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::IPv4 => serializer.serialize_unit_variant("PduSessionTypeRm", 0u32, "IPv4"),
            Self::IPv6 => serializer.serialize_unit_variant("PduSessionTypeRm", 1u32, "IPv6"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Expiry times of inactive NAPT pinholes, in seconds. All timers must be at least 1 second."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PinholeTimeouts {
    #[doc = "Pinhole timeout for TCP pinholes in seconds. Default for TCP is 2 hours 4 minutes per RFC 5382 section 5."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<i32>,
    #[doc = "Pinhole timeout for UDP pinholes in seconds. Default for UDP is 5 minutes per RFC 4787 section 4.3."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub udp: Option<i32>,
    #[doc = "Pinhole timeout for ICMP pinholes in seconds. Default for ICMP Echo is 60 seconds per RFC 5508 section 3.2."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icmp: Option<i32>,
}
impl PinholeTimeouts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Public Land Mobile Network (PLMN) ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlmnId {
    #[doc = "Mobile Country Code."]
    pub mcc: Mcc,
    #[doc = "Mobile Network Code."]
    pub mnc: Mnc,
}
impl PlmnId {
    pub fn new(mcc: Mcc, mnc: Mnc) -> Self {
        Self { mcc, mnc }
    }
}
#[doc = "Public Land Mobile Network (PLMN) ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlmnIdRm {
    #[doc = "Mobile Country Code."]
    pub mcc: Mcc,
    #[doc = "Mobile Network Code."]
    pub mnc: Mnc,
}
impl PlmnIdRm {
    pub fn new(mcc: Mcc, mnc: Mnc) -> Self {
        Self { mcc, mnc }
    }
}
#[doc = "Range of port numbers to use as translated ports on each translated address.\nIf not specified and NAPT is enabled, this range defaults to 1,024 - 65,535. (Ports under 1,024 should not be used because these are special purpose ports reserved by IANA.)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortRange {
    #[doc = "The minimum port number"]
    #[serde(rename = "minPort", default, skip_serializing_if = "Option::is_none")]
    pub min_port: Option<i32>,
    #[doc = "The maximum port number"]
    #[serde(rename = "maxPort", default, skip_serializing_if = "Option::is_none")]
    pub max_port: Option<i32>,
}
impl PortRange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The minimum time (in seconds) that will pass before a port that was used by a closed pinhole can be recycled for use by another pinhole. All hold times must be minimum 1 second."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortReuseHoldTimes {
    #[doc = "Minimum time in seconds that will pass before a TCP port that was used by a closed pinhole can be reused. Default for TCP is 2 minutes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<i32>,
    #[doc = "Minimum time in seconds that will pass before a UDP port that was used by a closed pinhole can be reused. Default for UDP is 1 minute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub udp: Option<i32>,
}
impl PortReuseHoldTimes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Preemption capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PreemptionCapability")]
pub enum PreemptionCapability {
    NotPreempt,
    MayPreempt,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PreemptionCapability {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PreemptionCapability {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PreemptionCapability {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotPreempt => serializer.serialize_unit_variant("PreemptionCapability", 0u32, "NotPreempt"),
            Self::MayPreempt => serializer.serialize_unit_variant("PreemptionCapability", 1u32, "MayPreempt"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Preemption vulnerability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PreemptionVulnerability")]
pub enum PreemptionVulnerability {
    NotPreemptable,
    Preemptable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PreemptionVulnerability {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PreemptionVulnerability {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PreemptionVulnerability {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotPreemptable => serializer.serialize_unit_variant("PreemptionVulnerability", 0u32, "NotPreemptable"),
            Self::Preemptable => serializer.serialize_unit_variant("PreemptionVulnerability", 1u32, "Preemptable"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Unknown,
    Succeeded,
    Accepted,
    Deleting,
    Failed,
    Canceled,
    Deleted,
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
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Accepted"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
            Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "QoS policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QosPolicy {
    #[doc = "5G QoS Identifier priority level."]
    #[serde(rename = "5qi", default, skip_serializing_if = "Option::is_none")]
    pub n5qi: Option<N5QiPriorityLevel>,
    #[doc = "ARP priority level."]
    #[serde(rename = "allocationAndRetentionPriorityLevel", default, skip_serializing_if = "Option::is_none")]
    pub allocation_and_retention_priority_level: Option<ArpPriorityLevel>,
    #[doc = "Preemption capability."]
    #[serde(rename = "preemptionCapability", default, skip_serializing_if = "Option::is_none")]
    pub preemption_capability: Option<PreemptionCapability>,
    #[doc = "Preemption vulnerability."]
    #[serde(rename = "preemptionVulnerability", default, skip_serializing_if = "Option::is_none")]
    pub preemption_vulnerability: Option<PreemptionVulnerability>,
    #[doc = "Aggregate Maximum Bit Rate."]
    #[serde(rename = "maximumBitRate")]
    pub maximum_bit_rate: Ambr,
}
impl QosPolicy {
    pub fn new(maximum_bit_rate: Ambr) -> Self {
        Self {
            n5qi: None,
            allocation_and_retention_priority_level: None,
            preemption_capability: None,
            preemption_vulnerability: None,
            maximum_bit_rate,
        }
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
pub type RfspIndex = i32;
pub type RfspIndexRm = i32;
#[doc = "Service data flow direction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SdfDirection")]
pub enum SdfDirection {
    Uplink,
    Downlink,
    Bidirectional,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SdfDirection {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SdfDirection {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SdfDirection {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Uplink => serializer.serialize_unit_variant("SdfDirection", 0u32, "Uplink"),
            Self::Downlink => serializer.serialize_unit_variant("SdfDirection", 1u32, "Downlink"),
            Self::Bidirectional => serializer.serialize_unit_variant("SdfDirection", 2u32, "Bidirectional"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Service properties."]
    pub properties: ServicePropertiesFormat,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Service {
    pub fn new(tracked_resource: TrackedResource, properties: ServicePropertiesFormat) -> Self {
        Self {
            tracked_resource,
            properties,
            system_data: None,
        }
    }
}
#[doc = "Service data flow (SDF) template"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDataFlowTemplate {
    #[doc = "The name of the SDF template. This must be unique within the parent PccRuleConfiguration. You must not use any of the following reserved strings - `default`, `requested` or `service`."]
    #[serde(rename = "templateName")]
    pub template_name: String,
    #[doc = "Service data flow direction."]
    pub direction: SdfDirection,
    #[doc = "A list of the allowed protocol(s) for this flow. If you want this flow to be able to use any protocol within the internet protocol suite, use the value `ip`. If you only want to allow a selection of protocols, you must use the corresponding IANA Assigned Internet Protocol Number for each protocol, as described in https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml. For example, for UDP, you must use 17. If you use the value `ip` then you must leave the field `port` unspecified."]
    pub protocol: Vec<String>,
    #[doc = "The remote IP address(es) to which UEs will connect for this flow. If you want to allow connections on any IP address, use the value `any`. Otherwise, you must provide each of the remote IP addresses to which Fusion Core will connect for this flow. You must provide each IP address in CIDR notation, including the netmask (for example, 192.0.2.54/24)."]
    #[serde(rename = "remoteIpList")]
    pub remote_ip_list: Vec<String>,
    #[doc = "The port(s) to which UEs will connect for this flow. You can specify zero or more ports or port ranges. If you specify one or more ports or port ranges then you must specify a value other than `ip` in the `protocol` field. This is an optional setting. If you do not specify it then connections will be allowed on all ports. Port ranges must be specified as <FirstPort>-<LastPort>. For example: [`8080`, `8082-8085`]."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<String>,
}
impl ServiceDataFlowTemplate {
    pub fn new(template_name: String, direction: SdfDirection, protocol: Vec<String>, remote_ip_list: Vec<String>) -> Self {
        Self {
            template_name,
            direction,
            protocol,
            remote_ip_list,
            ports: Vec::new(),
        }
    }
}
#[doc = "Response for Services API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceListResult {
    #[doc = "A list of Services."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Service>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServiceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServiceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "A precedence value that is used to decide between services when identifying the QoS values to use for a particular Sim. A lower value means a higher priority. This value should be unique among all services configured in the Mobile Network."]
    #[serde(rename = "servicePrecedence")]
    pub service_precedence: i32,
    #[doc = "QoS policy"]
    #[serde(rename = "serviceQosPolicy", default, skip_serializing_if = "Option::is_none")]
    pub service_qos_policy: Option<QosPolicy>,
    #[doc = "The set of PCC Rules that make up this service."]
    #[serde(rename = "pccRules")]
    pub pcc_rules: Vec<PccRuleConfiguration>,
}
impl ServicePropertiesFormat {
    pub fn new(service_precedence: i32, pcc_rules: Vec<PccRuleConfiguration>) -> Self {
        Self {
            provisioning_state: None,
            service_precedence,
            service_qos_policy: None,
            pcc_rules,
        }
    }
}
#[doc = "Reference to a Service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceResourceId {
    #[doc = "Service resource ID."]
    pub id: String,
}
impl ServiceResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Sim resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sim {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Sim properties."]
    pub properties: SimPropertiesFormat,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Sim {
    pub fn new(tracked_resource: TrackedResource, properties: SimPropertiesFormat) -> Self {
        Self {
            tracked_resource,
            properties,
            system_data: None,
        }
    }
}
#[doc = "Response for list sim ids API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SimIdListResult {
    #[doc = "A list of sim profile ids in a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubResource>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SimIdListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for list Sims API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SimListResult {
    #[doc = "A list of Sims in a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Sim>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SimListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SimListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sim policy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimPolicy {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "SimPolicy properties."]
    pub properties: SimPolicyPropertiesFormat,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl SimPolicy {
    pub fn new(tracked_resource: TrackedResource, properties: SimPolicyPropertiesFormat) -> Self {
        Self {
            tracked_resource,
            properties,
            system_data: None,
        }
    }
}
#[doc = "Response for SimPolicies API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SimPolicyListResult {
    #[doc = "A list of SimPolicies."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SimPolicy>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SimPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SimPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SimPolicy properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimPolicyPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Aggregate Maximum Bit Rate."]
    #[serde(rename = "ueAmbr")]
    pub ue_ambr: Ambr,
    #[doc = "Reference to a Slice resource."]
    #[serde(rename = "defaultSlice")]
    pub default_slice: SliceResourceId,
    #[doc = "RAT/Frequency Selection Priority Index"]
    #[serde(rename = "rfspIndex", default, skip_serializing_if = "Option::is_none")]
    pub rfsp_index: Option<RfspIndex>,
    #[doc = "Interval for the UE periodic registration update procedure, in seconds."]
    #[serde(rename = "registrationTimer", default, skip_serializing_if = "Option::is_none")]
    pub registration_timer: Option<i32>,
    #[doc = "The allowed slices and the settings to use for them. The list must not contain duplicate items and must contain at least one item."]
    #[serde(rename = "sliceConfigurations")]
    pub slice_configurations: Vec<SliceConfiguration>,
}
impl SimPolicyPropertiesFormat {
    pub fn new(ue_ambr: Ambr, default_slice: SliceResourceId, slice_configurations: Vec<SliceConfiguration>) -> Self {
        Self {
            provisioning_state: None,
            ue_ambr,
            default_slice,
            rfsp_index: None,
            registration_timer: None,
            slice_configurations,
        }
    }
}
#[doc = "Reference to a SIM Policy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimPolicyResourceId {
    #[doc = "SIM Policy resource ID."]
    pub id: String,
}
impl SimPolicyResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Sim properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The state of the sim resource."]
    #[serde(rename = "simState", default, skip_serializing_if = "Option::is_none")]
    pub sim_state: Option<SimState>,
    #[doc = "The International Mobile Subscriber Identity (IMSI) for the sim."]
    #[serde(rename = "internationalMobileSubscriberIdentity")]
    pub international_mobile_subscriber_identity: String,
    #[doc = "The Integrated Circuit Card ID (ICC Id) for the sim."]
    #[serde(rename = "integratedCircuitCardIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub integrated_circuit_card_identifier: Option<String>,
    #[doc = "The ki value for the sim."]
    #[serde(rename = "authenticationKey", default, skip_serializing_if = "Option::is_none")]
    pub authentication_key: Option<String>,
    #[doc = "The Opc value for the sim."]
    #[serde(rename = "operatorKeyCode", default, skip_serializing_if = "Option::is_none")]
    pub operator_key_code: Option<String>,
    #[doc = "Reference to a Mobile Network resource."]
    #[serde(rename = "mobileNetwork", default, skip_serializing_if = "Option::is_none")]
    pub mobile_network: Option<MobileNetworkResourceId>,
    #[doc = "An optional free-form text field that can be used to record the device type this sim is associated with, for example 'Video camera'. The Azure portal allows Sims to be grouped and filtered based on this value."]
    #[serde(rename = "deviceType", default, skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[doc = "Reference to a SIM Policy resource."]
    #[serde(rename = "simPolicy", default, skip_serializing_if = "Option::is_none")]
    pub sim_policy: Option<SimPolicyResourceId>,
    #[doc = "A list of static IP addresses assigned to this sim. Each address is assigned at a defined network scope, made up of {attached data network, slice}."]
    #[serde(rename = "staticIpConfiguration", default, skip_serializing_if = "Vec::is_empty")]
    pub static_ip_configuration: Vec<SimStaticIpProperties>,
}
impl SimPropertiesFormat {
    pub fn new(international_mobile_subscriber_identity: String) -> Self {
        Self {
            provisioning_state: None,
            sim_state: None,
            international_mobile_subscriber_identity,
            integrated_circuit_card_identifier: None,
            authentication_key: None,
            operator_key_code: None,
            mobile_network: None,
            device_type: None,
            sim_policy: None,
            static_ip_configuration: Vec::new(),
        }
    }
}
#[doc = "The state of the sim resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SimState")]
pub enum SimState {
    Disabled,
    Enabled,
    Invalid,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SimState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SimState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SimState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("SimState", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("SimState", 1u32, "Enabled"),
            Self::Invalid => serializer.serialize_unit_variant("SimState", 2u32, "Invalid"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Static IP configuration for a sim, scoped to a particular attached data network and slice."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SimStaticIpProperties {
    #[doc = "Reference to an Attached Data Network resource."]
    #[serde(rename = "attachedDataNetwork", default, skip_serializing_if = "Option::is_none")]
    pub attached_data_network: Option<AttachedDataNetworkResourceId>,
    #[doc = "Reference to a Slice resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slice: Option<SliceResourceId>,
    #[doc = "The static IP configuration for the sim to use at the defined network scope."]
    #[serde(rename = "staticIp", default, skip_serializing_if = "Option::is_none")]
    pub static_ip: Option<sim_static_ip_properties::StaticIp>,
}
impl SimStaticIpProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sim_static_ip_properties {
    use super::*;
    #[doc = "The static IP configuration for the sim to use at the defined network scope."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct StaticIp {
        #[doc = "IPv4 address."]
        #[serde(rename = "ipv4Address", default, skip_serializing_if = "Option::is_none")]
        pub ipv4_address: Option<Ipv4Addr>,
    }
    impl StaticIp {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Site {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Site properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SitePropertiesFormat>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Site {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "Response for sites API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteListResult {
    #[doc = "A list of sites in a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Site>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SiteListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SiteListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Site properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SitePropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "An array of ids of the network functions deployed on the site, maintained by the user."]
    #[serde(rename = "networkFunctions", default, skip_serializing_if = "Vec::is_empty")]
    pub network_functions: Vec<SubResource>,
}
impl SitePropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network slice resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Slice {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Network slice properties."]
    pub properties: SlicePropertiesFormat,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Slice {
    pub fn new(tracked_resource: TrackedResource, properties: SlicePropertiesFormat) -> Self {
        Self {
            tracked_resource,
            properties,
            system_data: None,
        }
    }
}
#[doc = "Per-slice settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SliceConfiguration {
    #[doc = "Reference to a Slice resource."]
    pub slice: SliceResourceId,
    #[doc = "Reference to a Data Network resource."]
    #[serde(rename = "defaultDataNetwork")]
    pub default_data_network: DataNetworkResourceId,
    #[doc = "The allowed data networks and the settings to use for them. The list must not contain duplicate items and must contain at least one item."]
    #[serde(rename = "dataNetworkConfigurations")]
    pub data_network_configurations: Vec<DataNetworkConfiguration>,
}
impl SliceConfiguration {
    pub fn new(
        slice: SliceResourceId,
        default_data_network: DataNetworkResourceId,
        data_network_configurations: Vec<DataNetworkConfiguration>,
    ) -> Self {
        Self {
            slice,
            default_data_network,
            data_network_configurations,
        }
    }
}
#[doc = "Response for attached data network API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SliceListResult {
    #[doc = "A list of data networks in a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Slice>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SliceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SliceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network slice properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlicePropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Single-Network Slice Selection Assistance Information (S-NSSAI)."]
    pub snssai: Snssai,
    #[doc = "An optional description for this network slice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl SlicePropertiesFormat {
    pub fn new(snssai: Snssai) -> Self {
        Self {
            provisioning_state: None,
            snssai,
            description: None,
        }
    }
}
#[doc = "Reference to a Slice resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SliceResourceId {
    #[doc = "Slice resource ID."]
    pub id: String,
}
impl SliceResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Single-Network Slice Selection Assistance Information (S-NSSAI)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snssai {
    #[doc = "Slice/Service Type (SST)."]
    pub sst: i32,
    #[doc = "Slice Differentiator (SD)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd: Option<String>,
}
impl Snssai {
    pub fn new(sst: i32) -> Self {
        Self { sst, sd: None }
    }
}
#[doc = "Reference to another sub resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubResource {
    #[doc = "Resource ID."]
    pub id: String,
}
impl SubResource {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
pub type Tac = String;
pub type TacRm = String;
#[doc = "Tags object for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsObject {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsObject {
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
#[doc = "Traffic control permission."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TrafficControlPermission")]
pub enum TrafficControlPermission {
    Enabled,
    Blocked,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TrafficControlPermission {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TrafficControlPermission {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TrafficControlPermission {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("TrafficControlPermission", 0u32, "Enabled"),
            Self::Blocked => serializer.serialize_unit_variant("TrafficControlPermission", 1u32, "Blocked"),
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

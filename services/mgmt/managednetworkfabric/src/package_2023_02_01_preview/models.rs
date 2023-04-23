#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Show ARP table entry properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArpProperties {
    #[doc = "Ipv4 or Ipv6 address"]
    pub address: String,
    #[doc = "Duration in seconds."]
    pub age: String,
    #[doc = "Hardware address."]
    #[serde(rename = "macAddress")]
    pub mac_address: String,
    #[doc = "Layer 2 interface name."]
    pub interface: String,
    #[doc = "ARP status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl ArpProperties {
    pub fn new(address: String, age: String, mac_address: String, interface: String) -> Self {
        Self {
            address,
            age,
            mac_address,
            interface,
            state: None,
        }
    }
}
#[doc = "The AccessControlList resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessControlList {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "AccessControlListProperties define the resource properties."]
    pub properties: AccessControlListProperties,
}
impl AccessControlList {
    pub fn new(tracked_resource: TrackedResource, properties: AccessControlListProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "The AccessControlList patch resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessControlListPatch {
    #[doc = "AccessControlListPatchProperties define the patchable resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessControlListPatchProperties>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AccessControlListPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AccessControlListPatchProperties define the patchable resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessControlListPatchProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[doc = "IP address family. Example: ipv4 | ipv6."]
    #[serde(rename = "addressFamily", default, skip_serializing_if = "Option::is_none")]
    pub address_family: Option<access_control_list_patch_properties::AddressFamily>,
    #[doc = "Access Control List conditions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions: Vec<serde_json::Value>,
}
impl AccessControlListPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_control_list_patch_properties {
    use super::*;
    #[doc = "IP address family. Example: ipv4 | ipv6."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AddressFamily")]
    pub enum AddressFamily {
        #[serde(rename = "ipv4")]
        Ipv4,
        #[serde(rename = "ipv6")]
        Ipv6,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AddressFamily {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AddressFamily {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AddressFamily {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ipv4 => serializer.serialize_unit_variant("AddressFamily", 0u32, "ipv4"),
                Self::Ipv6 => serializer.serialize_unit_variant("AddressFamily", 1u32, "ipv6"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "AccessControlListProperties define the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessControlListProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[doc = "IP address family. Example: ipv4 | ipv6."]
    #[serde(rename = "addressFamily")]
    pub address_family: access_control_list_properties::AddressFamily,
    #[doc = "Access Control List conditions."]
    pub conditions: Vec<serde_json::Value>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl AccessControlListProperties {
    pub fn new(address_family: access_control_list_properties::AddressFamily, conditions: Vec<serde_json::Value>) -> Self {
        Self {
            annotation_resource: AnnotationResource::default(),
            address_family,
            conditions,
            provisioning_state: None,
        }
    }
}
pub mod access_control_list_properties {
    use super::*;
    #[doc = "IP address family. Example: ipv4 | ipv6."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AddressFamily")]
    pub enum AddressFamily {
        #[serde(rename = "ipv4")]
        Ipv4,
        #[serde(rename = "ipv6")]
        Ipv6,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AddressFamily {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AddressFamily {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AddressFamily {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ipv4 => serializer.serialize_unit_variant("AddressFamily", 0u32, "ipv4"),
                Self::Ipv6 => serializer.serialize_unit_variant("AddressFamily", 1u32, "ipv6"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of AccessControlLists."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessControlListsListResult {
    #[doc = "List of AccessControlList resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AccessControlList>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessControlListsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AccessControlListsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Switch configuration entries require a description to discern between configuration groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnnotationResource {
    #[doc = "Switch configuration description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
}
impl AnnotationResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BFD configuration properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BfdConfiguration {
    #[doc = "EnabledDisabledState state for the resource."]
    #[serde(rename = "administrativeState", default, skip_serializing_if = "Option::is_none")]
    pub administrative_state: Option<EnabledDisabledState>,
    #[doc = "interval in milliseconds. Example: 300."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[doc = "Multiplier for the Bfd Configuration. Example: 3."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<i32>,
}
impl BfdConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Boolean Enum. Example- True/False"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BooleanEnumProperty")]
pub enum BooleanEnumProperty {
    True,
    False,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BooleanEnumProperty {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BooleanEnumProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BooleanEnumProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::True => serializer.serialize_unit_variant("BooleanEnumProperty", 0u32, "True"),
            Self::False => serializer.serialize_unit_variant("BooleanEnumProperty", 1u32, "False"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Update administrative state on list of resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnableDisableOnResources {
    #[doc = "Network Fabrics or Network Rack resource Id."]
    #[serde(
        rename = "resourceIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resource_ids: Vec<String>,
}
impl EnableDisableOnResources {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "EnabledDisabledState state for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnabledDisabledState")]
pub enum EnabledDisabledState {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnabledDisabledState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnabledDisabledState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnabledDisabledState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("EnabledDisabledState", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("EnabledDisabledState", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "The ExpressRoute circuit ID and the Auth Key are required for you to successfully deploy NFC service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpressRouteConnectionInformation {
    #[doc = "The express route circuit Azure resource ID, must be of type Microsoft.Network/expressRouteCircuits/circuitName. The ExpressRoute Circuit is a mandatory attribute."]
    #[serde(rename = "expressRouteCircuitId")]
    pub express_route_circuit_id: String,
    #[doc = "Authorization key for the circuit, must be of type Microsoft.Network/expressRouteCircuits/authorizations. The Auth Key is a mandatory attribute."]
    #[serde(rename = "expressRouteAuthorizationKey")]
    pub express_route_authorization_key: String,
}
impl ExpressRouteConnectionInformation {
    pub fn new(express_route_circuit_id: String, express_route_authorization_key: String) -> Self {
        Self {
            express_route_circuit_id,
            express_route_authorization_key,
        }
    }
}
#[doc = "ExpressRouteStatus defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpressRouteStatusDef {
    #[doc = "The express route circuit Azure resource ID, must be of type Microsoft.Network/expressRouteCircuits/circuitName."]
    #[serde(rename = "expressRouteCircuitId", default, skip_serializing_if = "Option::is_none")]
    pub express_route_circuit_id: Option<String>,
    #[doc = "Express route connection state for the resource."]
    #[serde(rename = "expressRouteStatus", default, skip_serializing_if = "Option::is_none")]
    pub express_route_status: Option<express_route_status_def::ExpressRouteStatus>,
}
impl ExpressRouteStatusDef {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod express_route_status_def {
    use super::*;
    #[doc = "Express route connection state for the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ExpressRouteStatus")]
    pub enum ExpressRouteStatus {
        Connecting,
        Connected,
        Disconnected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ExpressRouteStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ExpressRouteStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ExpressRouteStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Connecting => serializer.serialize_unit_variant("ExpressRouteStatus", 0u32, "Connecting"),
                Self::Connected => serializer.serialize_unit_variant("ExpressRouteStatus", 1u32, "Connected"),
                Self::Disconnected => serializer.serialize_unit_variant("ExpressRouteStatus", 2u32, "Disconnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "Defines the ExternalNetwork item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalNetwork {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "External Network Properties."]
    pub properties: ExternalNetworkProperties,
}
impl ExternalNetwork {
    pub fn new(properties: ExternalNetworkProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "The ExternalNetwork patch resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalNetworkPatch {
    #[doc = "External Network Patch properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl ExternalNetworkPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ExternalNetwork patchable properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalNetworkPatchableProperties {
    #[doc = "Peering option list."]
    #[serde(rename = "peeringOption", default, skip_serializing_if = "Option::is_none")]
    pub peering_option: Option<PeeringOption>,
    #[doc = "Option B configuration."]
    #[serde(rename = "optionBProperties", default, skip_serializing_if = "Option::is_none")]
    pub option_b_properties: Option<OptionBProperties>,
    #[doc = "Peering optionA properties"]
    #[serde(rename = "optionAProperties", default, skip_serializing_if = "Option::is_none")]
    pub option_a_properties: Option<OptionAProperties>,
    #[doc = "ARM resource ID of importRoutePolicy."]
    #[serde(rename = "importRoutePolicyId", default, skip_serializing_if = "Option::is_none")]
    pub import_route_policy_id: Option<String>,
    #[doc = "ARM resource ID of exportRoutePolicy."]
    #[serde(rename = "exportRoutePolicyId", default, skip_serializing_if = "Option::is_none")]
    pub export_route_policy_id: Option<String>,
}
impl ExternalNetworkPatchableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "External Network Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalNetworkProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[doc = "Gets the networkToNetworkInterconnectId of the resource."]
    #[serde(rename = "networkToNetworkInterconnectId", default, skip_serializing_if = "Option::is_none")]
    pub network_to_network_interconnect_id: Option<String>,
    #[doc = "List of resources the externalNetwork is disabled on. Can be either entire NetworkFabric or NetworkRack."]
    #[serde(
        rename = "disabledOnResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disabled_on_resources: Vec<String>,
    #[doc = "EnabledDisabledState state for the resource."]
    #[serde(rename = "administrativeState", default, skip_serializing_if = "Option::is_none")]
    pub administrative_state: Option<EnabledDisabledState>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Peering option list."]
    #[serde(rename = "peeringOption")]
    pub peering_option: PeeringOption,
    #[doc = "Option B configuration."]
    #[serde(rename = "optionBProperties", default, skip_serializing_if = "Option::is_none")]
    pub option_b_properties: Option<OptionBProperties>,
    #[doc = "option A properties object"]
    #[serde(rename = "optionAProperties", default, skip_serializing_if = "Option::is_none")]
    pub option_a_properties: Option<serde_json::Value>,
    #[doc = "ARM resource ID of importRoutePolicy."]
    #[serde(rename = "importRoutePolicyId", default, skip_serializing_if = "Option::is_none")]
    pub import_route_policy_id: Option<String>,
    #[doc = "ARM resource ID of exportRoutePolicy."]
    #[serde(rename = "exportRoutePolicyId", default, skip_serializing_if = "Option::is_none")]
    pub export_route_policy_id: Option<String>,
}
impl ExternalNetworkProperties {
    pub fn new(peering_option: PeeringOption) -> Self {
        Self {
            annotation_resource: AnnotationResource::default(),
            network_to_network_interconnect_id: None,
            disabled_on_resources: Vec::new(),
            administrative_state: None,
            provisioning_state: None,
            peering_option,
            option_b_properties: None,
            option_a_properties: None,
            import_route_policy_id: None,
            export_route_policy_id: None,
        }
    }
}
#[doc = "List of ExternalNetworks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalNetworksList {
    #[doc = "List of ExternalNetworks resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ExternalNetwork>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExternalNetworksList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ExternalNetworksList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "FailedSucceeded state for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FailedSucceededState")]
pub enum FailedSucceededState {
    Succeeded,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FailedSucceededState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FailedSucceededState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FailedSucceededState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("FailedSucceededState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("FailedSucceededState", 1u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Show ARP entries response per network device"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetArpResponse {}
impl GetArpResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Get Device status response properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetDeviceStatusProperties {
    #[doc = "Primary or Secondary power end."]
    #[serde(rename = "operationalStatus")]
    pub operational_status: get_device_status_properties::OperationalStatus,
    #[doc = "On or Off power cycle state."]
    #[serde(rename = "powerCycleState")]
    pub power_cycle_state: get_device_status_properties::PowerCycleState,
    #[doc = "The serial number of the device"]
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
}
impl GetDeviceStatusProperties {
    pub fn new(
        operational_status: get_device_status_properties::OperationalStatus,
        power_cycle_state: get_device_status_properties::PowerCycleState,
        serial_number: String,
    ) -> Self {
        Self {
            operational_status,
            power_cycle_state,
            serial_number,
        }
    }
}
pub mod get_device_status_properties {
    use super::*;
    #[doc = "Primary or Secondary power end."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationalStatus")]
    pub enum OperationalStatus {
        Booted,
        BootPrompt,
        Ztp,
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
                Self::Booted => serializer.serialize_unit_variant("OperationalStatus", 0u32, "Booted"),
                Self::BootPrompt => serializer.serialize_unit_variant("OperationalStatus", 1u32, "BootPrompt"),
                Self::Ztp => serializer.serialize_unit_variant("OperationalStatus", 2u32, "Ztp"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "On or Off power cycle state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PowerCycleState")]
    pub enum PowerCycleState {
        On,
        Off,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PowerCycleState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PowerCycleState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PowerCycleState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::On => serializer.serialize_unit_variant("PowerCycleState", 0u32, "On"),
                Self::Off => serializer.serialize_unit_variant("PowerCycleState", 1u32, "Off"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type GetDynamicInterfaceMapsProperties = Vec<serde_json::Value>;
pub type GetStaticInterfaceMapsProperties = Vec<serde_json::Value>;
#[doc = "Interface running status properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InterfaceStatus {
    #[doc = "EnabledDisabledState state for the resource."]
    #[serde(rename = "administrativeState", default, skip_serializing_if = "Option::is_none")]
    pub administrative_state: Option<EnabledDisabledState>,
    #[doc = "The interface operational status."]
    #[serde(rename = "operationalStatus", default, skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<String>,
    #[doc = "The physical status."]
    #[serde(rename = "phyStatus", default, skip_serializing_if = "Option::is_none")]
    pub phy_status: Option<String>,
    #[doc = "The interface transceiver type. Example: up or down"]
    #[serde(rename = "transceiverStatus", default, skip_serializing_if = "Option::is_none")]
    pub transceiver_status: Option<String>,
    #[doc = "Connected to ARM resource or external interface"]
    #[serde(rename = "connectedTo", default, skip_serializing_if = "Option::is_none")]
    pub connected_to: Option<String>,
}
impl InterfaceStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the InternalNetwork item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalNetwork {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Internal Network Properties"]
    pub properties: InternalNetworkProperties,
}
impl InternalNetwork {
    pub fn new(properties: InternalNetworkProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "The InternalNetwork patch resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InternalNetworkPatch {
    #[doc = "InternalNetwork Patch properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl InternalNetworkPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ExternalNetwork patchable properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InternalNetworkPatchableProperties {
    #[doc = "Maximum transmission unit. Default value is 1500."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[doc = "List with object connectedIPv4Subnets."]
    #[serde(
        rename = "connectedIPv4Subnets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub connected_i_pv4_subnets: Vec<serde_json::Value>,
    #[doc = "List with object connectedIPv6Subnets."]
    #[serde(
        rename = "connectedIPv6Subnets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub connected_i_pv6_subnets: Vec<serde_json::Value>,
    #[doc = "staticRouteConfiguration model."]
    #[serde(rename = "staticRouteConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub static_route_configuration: Option<internal_network_patchable_properties::StaticRouteConfiguration>,
    #[doc = "BGP configuration properties"]
    #[serde(rename = "bgpConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub bgp_configuration: Option<internal_network_patchable_properties::BgpConfiguration>,
    #[doc = "ARM resource ID of importRoutePolicy."]
    #[serde(rename = "importRoutePolicyId", default, skip_serializing_if = "Option::is_none")]
    pub import_route_policy_id: Option<String>,
    #[doc = "ARM resource ID of importRoutePolicy."]
    #[serde(rename = "exportRoutePolicyId", default, skip_serializing_if = "Option::is_none")]
    pub export_route_policy_id: Option<String>,
}
impl InternalNetworkPatchableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod internal_network_patchable_properties {
    use super::*;
    #[doc = "staticRouteConfiguration model."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct StaticRouteConfiguration {
        #[doc = "BFD configuration properties"]
        #[serde(rename = "bfdConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub bfd_configuration: Option<BfdConfiguration>,
        #[doc = "List with object IPv4Routes."]
        #[serde(
            rename = "ipv4Routes",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub ipv4_routes: Vec<serde_json::Value>,
        #[doc = "List with object IPv6Routes."]
        #[serde(
            rename = "ipv6Routes",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub ipv6_routes: Vec<serde_json::Value>,
    }
    impl StaticRouteConfiguration {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "BGP configuration properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct BgpConfiguration {
        #[serde(flatten)]
        pub annotation_resource: AnnotationResource,
        #[doc = "BFD configuration properties"]
        #[serde(rename = "bfdConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub bfd_configuration: Option<BfdConfiguration>,
        #[doc = "Boolean Enum. Example- True/False"]
        #[serde(rename = "defaultRouteOriginate", default, skip_serializing_if = "Option::is_none")]
        pub default_route_originate: Option<BooleanEnumProperty>,
        #[doc = "Allows for routes to be received and processed even if the router detects its own ASN in the AS-Path. 0 is disable, Possible values are 1-10, default is 2."]
        #[serde(rename = "allowAS", default, skip_serializing_if = "Option::is_none")]
        pub allow_as: Option<i32>,
        #[doc = "Enable Or Disable state."]
        #[serde(rename = "allowASOverride", default, skip_serializing_if = "Option::is_none")]
        pub allow_as_override: Option<bgp_configuration::AllowAsOverride>,
        #[doc = "ASN of Network Fabric. Example: 65048."]
        #[serde(rename = "fabricASN", default, skip_serializing_if = "Option::is_none")]
        pub fabric_asn: Option<i32>,
        #[doc = "Peer ASN. Example: 65047."]
        #[serde(rename = "peerASN")]
        pub peer_asn: i32,
        #[doc = "BGP Ipv4 ListenRange."]
        #[serde(
            rename = "ipv4ListenRangePrefixes",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub ipv4_listen_range_prefixes: Vec<String>,
        #[doc = "BGP Ipv6 ListenRange."]
        #[serde(
            rename = "ipv6ListenRangePrefixes",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub ipv6_listen_range_prefixes: Vec<String>,
        #[doc = "List with stringified ipv4NeighborAddresses."]
        #[serde(
            rename = "ipv4NeighborAddress",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub ipv4_neighbor_address: Vec<serde_json::Value>,
        #[doc = "List with stringified ipv6NeighborAddress."]
        #[serde(
            rename = "ipv6NeighborAddress",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub ipv6_neighbor_address: Vec<serde_json::Value>,
    }
    impl BgpConfiguration {
        pub fn new(peer_asn: i32) -> Self {
            Self {
                annotation_resource: AnnotationResource::default(),
                bfd_configuration: None,
                default_route_originate: None,
                allow_as: None,
                allow_as_override: None,
                fabric_asn: None,
                peer_asn,
                ipv4_listen_range_prefixes: Vec::new(),
                ipv6_listen_range_prefixes: Vec::new(),
                ipv4_neighbor_address: Vec::new(),
                ipv6_neighbor_address: Vec::new(),
            }
        }
    }
    pub mod bgp_configuration {
        use super::*;
        #[doc = "Enable Or Disable state."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "AllowAsOverride")]
        pub enum AllowAsOverride {
            Enable,
            Disable,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for AllowAsOverride {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for AllowAsOverride {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for AllowAsOverride {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Enable => serializer.serialize_unit_variant("AllowAsOverride", 0u32, "Enable"),
                    Self::Disable => serializer.serialize_unit_variant("AllowAsOverride", 1u32, "Disable"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Internal Network Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalNetworkProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[serde(flatten)]
    pub internal_network_patchable_properties: InternalNetworkPatchableProperties,
    #[doc = "List of resources the InternalNetwork is disabled on. Can be either entire NetworkFabric or NetworkRack."]
    #[serde(
        rename = "disabledOnResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disabled_on_resources: Vec<String>,
    #[doc = "EnabledDisabledState state for the resource."]
    #[serde(rename = "administrativeState", default, skip_serializing_if = "Option::is_none")]
    pub administrative_state: Option<EnabledDisabledState>,
    #[doc = "List of resources the BGP is disabled on. Can be either entire NetworkFabric or NetworkRack."]
    #[serde(
        rename = "bgpDisabledOnResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bgp_disabled_on_resources: Vec<String>,
    #[doc = "List of resources the BFD for BGP is disabled on. Can be either entire NetworkFabric or NetworkRack."]
    #[serde(
        rename = "bfdDisabledOnResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bfd_disabled_on_resources: Vec<String>,
    #[doc = "List of resources the BFD of StaticRoutes is disabled on. Can be either entire NetworkFabric or NetworkRack."]
    #[serde(
        rename = "bfdForStaticRoutesDisabledOnResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bfd_for_static_routes_disabled_on_resources: Vec<String>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Vlan identifier. Example: 1001."]
    #[serde(rename = "vlanId")]
    pub vlan_id: i32,
}
impl InternalNetworkProperties {
    pub fn new(vlan_id: i32) -> Self {
        Self {
            annotation_resource: AnnotationResource::default(),
            internal_network_patchable_properties: InternalNetworkPatchableProperties::default(),
            disabled_on_resources: Vec::new(),
            administrative_state: None,
            bgp_disabled_on_resources: Vec::new(),
            bfd_disabled_on_resources: Vec::new(),
            bfd_for_static_routes_disabled_on_resources: Vec::new(),
            provisioning_state: None,
            vlan_id,
        }
    }
}
#[doc = "List of InternalNetworks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InternalNetworksList {
    #[doc = "List of InternalNetworks resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<InternalNetwork>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InternalNetworksList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl InternalNetworksList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The IpCommunityList resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpCommunityList {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "IpCommunityListProperties define the resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IpCommunityListProperties>,
}
impl IpCommunityList {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The IpCommunityList patch resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpCommunityListPatch {
    #[doc = "IpCommunityListPatch define the patchable resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IpCommunityListPatchProperties>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl IpCommunityListPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IpCommunityListPatch define the patchable resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpCommunityListPatchProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[doc = "action. Example: allow | deny."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ip_community_list_patch_properties::Action>,
    #[doc = "Local Autonomous System. Example: true | false."]
    #[serde(rename = "localAS", default, skip_serializing_if = "Option::is_none")]
    pub local_as: Option<ip_community_list_patch_properties::LocalAs>,
    #[doc = "noAdvertise. Example: true | false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub advertise: Option<ip_community_list_patch_properties::Advertise>,
    #[doc = "noExport. Example: true | false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export: Option<ip_community_list_patch_properties::Export>,
    #[doc = "Ip Community List communityMembers."]
    #[serde(
        rename = "communityMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub community_members: Vec<serde_json::Value>,
    #[doc = "Ip Community List evpnEsImportRouteTargets."]
    #[serde(
        rename = "evpnEsImportRouteTargets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub evpn_es_import_route_targets: Vec<serde_json::Value>,
}
impl IpCommunityListPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ip_community_list_patch_properties {
    use super::*;
    #[doc = "action. Example: allow | deny."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        #[serde(rename = "allow")]
        Allow,
        #[serde(rename = "deny")]
        Deny,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Action {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Action {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Action {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("Action", 0u32, "allow"),
                Self::Deny => serializer.serialize_unit_variant("Action", 1u32, "deny"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Local Autonomous System. Example: true | false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LocalAs")]
    pub enum LocalAs {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LocalAs {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LocalAs {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LocalAs {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("LocalAs", 0u32, "true"),
                Self::False => serializer.serialize_unit_variant("LocalAs", 1u32, "false"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "noAdvertise. Example: true | false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Advertise")]
    pub enum Advertise {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Advertise {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Advertise {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Advertise {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Advertise", 0u32, "true"),
                Self::False => serializer.serialize_unit_variant("Advertise", 1u32, "false"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "noExport. Example: true | false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Export")]
    pub enum Export {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Export {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Export {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Export {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Export", 0u32, "true"),
                Self::False => serializer.serialize_unit_variant("Export", 1u32, "false"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "IpCommunityListProperties define the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpCommunityListProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[doc = "action. Example: allow | deny."]
    pub action: ip_community_list_properties::Action,
    #[doc = "Local Autonomous System. Example: true | false."]
    #[serde(rename = "localAS")]
    pub local_as: ip_community_list_properties::LocalAs,
    #[doc = " Graceful Shutdown (GSHUT). Example: true | false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gshut: Option<ip_community_list_properties::Gshut>,
    #[doc = "Internet access. Example: true | false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internet: Option<ip_community_list_properties::Internet>,
    #[doc = "noAdvertise. Example: true | false."]
    pub advertise: ip_community_list_properties::Advertise,
    #[doc = "noExport. Example: true | false."]
    pub export: ip_community_list_properties::Export,
    #[doc = "Ip Community List communityMembers."]
    #[serde(
        rename = "communityMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub community_members: Vec<serde_json::Value>,
    #[doc = "Ip Community List evpnEsImportRouteTargets."]
    #[serde(
        rename = "evpnEsImportRouteTargets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub evpn_es_import_route_targets: Vec<serde_json::Value>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl IpCommunityListProperties {
    pub fn new(
        action: ip_community_list_properties::Action,
        local_as: ip_community_list_properties::LocalAs,
        advertise: ip_community_list_properties::Advertise,
        export: ip_community_list_properties::Export,
    ) -> Self {
        Self {
            annotation_resource: AnnotationResource::default(),
            action,
            local_as,
            gshut: None,
            internet: None,
            advertise,
            export,
            community_members: Vec::new(),
            evpn_es_import_route_targets: Vec::new(),
            provisioning_state: None,
        }
    }
}
pub mod ip_community_list_properties {
    use super::*;
    #[doc = "action. Example: allow | deny."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        #[serde(rename = "allow")]
        Allow,
        #[serde(rename = "deny")]
        Deny,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Action {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Action {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Action {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("Action", 0u32, "allow"),
                Self::Deny => serializer.serialize_unit_variant("Action", 1u32, "deny"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Local Autonomous System. Example: true | false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LocalAs")]
    pub enum LocalAs {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LocalAs {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LocalAs {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LocalAs {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("LocalAs", 0u32, "true"),
                Self::False => serializer.serialize_unit_variant("LocalAs", 1u32, "false"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = " Graceful Shutdown (GSHUT). Example: true | false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Gshut")]
    pub enum Gshut {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Gshut {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Gshut {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Gshut {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Gshut", 0u32, "true"),
                Self::False => serializer.serialize_unit_variant("Gshut", 1u32, "false"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Internet access. Example: true | false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Internet")]
    pub enum Internet {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Internet {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Internet {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Internet {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Internet", 0u32, "true"),
                Self::False => serializer.serialize_unit_variant("Internet", 1u32, "false"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "noAdvertise. Example: true | false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Advertise")]
    pub enum Advertise {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Advertise {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Advertise {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Advertise {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Advertise", 0u32, "true"),
                Self::False => serializer.serialize_unit_variant("Advertise", 1u32, "false"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "noExport. Example: true | false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Export")]
    pub enum Export {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Export {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Export {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Export {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Export", 0u32, "true"),
                Self::False => serializer.serialize_unit_variant("Export", 1u32, "false"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of IpCommunityLists."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpCommunityListsListResult {
    #[doc = "List of IpCommunityList resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<IpCommunityList>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IpCommunityListsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IpCommunityListsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The IpPrefixList resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpPrefixList {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "IpPrefixListProperties define the resource properties."]
    pub properties: IpPrefixListProperties,
}
impl IpPrefixList {
    pub fn new(tracked_resource: TrackedResource, properties: IpPrefixListProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "The IpPrefixList patch resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpPrefixListPatch {
    #[doc = "IpPrefixListPatchProperties define the patchable resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IpPrefixListPatchProperties>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl IpPrefixListPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IpPrefixListPatchProperties define the patchable resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpPrefixListPatchProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[doc = "action. Example: allow | deny."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ip_prefix_list_patch_properties::Action>,
    #[doc = "sequenceNumber of the Ip Prefix List."]
    #[serde(rename = "sequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i32>,
    #[doc = "networkAddress. Example:1.1.1.0/24 | 1.1.10.10."]
    #[serde(rename = "networkAddress", default, skip_serializing_if = "Option::is_none")]
    pub network_address: Option<String>,
}
impl IpPrefixListPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ip_prefix_list_patch_properties {
    use super::*;
    #[doc = "action. Example: allow | deny."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        #[serde(rename = "allow")]
        Allow,
        #[serde(rename = "deny")]
        Deny,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Action {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Action {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Action {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("Action", 0u32, "allow"),
                Self::Deny => serializer.serialize_unit_variant("Action", 1u32, "deny"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "IpPrefixListProperties define the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpPrefixListProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[doc = "action. Example: allow | deny."]
    pub action: ip_prefix_list_properties::Action,
    #[doc = "sequenceNumber of the Ip Prefix List."]
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: i32,
    #[doc = "networkAddress. Example:1.1.1.0/24 | 1.1.10.10."]
    #[serde(rename = "networkAddress")]
    pub network_address: String,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl IpPrefixListProperties {
    pub fn new(action: ip_prefix_list_properties::Action, sequence_number: i32, network_address: String) -> Self {
        Self {
            annotation_resource: AnnotationResource::default(),
            action,
            sequence_number,
            network_address,
            provisioning_state: None,
        }
    }
}
pub mod ip_prefix_list_properties {
    use super::*;
    #[doc = "action. Example: allow | deny."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        #[serde(rename = "allow")]
        Allow,
        #[serde(rename = "deny")]
        Deny,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Action {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Action {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Action {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("Action", 0u32, "allow"),
                Self::Deny => serializer.serialize_unit_variant("Action", 1u32, "deny"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of IpPrefixLists."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpPrefixListsListResult {
    #[doc = "List of IpPrefixList resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<IpPrefixList>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IpPrefixListsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IpPrefixListsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The L2IsolationDomain resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct L2IsolationDomain {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "L2IsolationDomainProperties define the resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<L2IsolationDomainProperties>,
}
impl L2IsolationDomain {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The L2IsolationDomain patch resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct L2IsolationDomainPatch {
    #[doc = "L2IsolationDomainPatchProperties define the patchable resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<L2IsolationDomainPatchProperties>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl L2IsolationDomainPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "L2IsolationDomainPatchProperties define the patchable resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct L2IsolationDomainPatchProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[doc = "maximum transmission unit. Default value is 1500."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
}
impl L2IsolationDomainPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "L2IsolationDomainProperties define the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct L2IsolationDomainProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[doc = "Network Fabric ARM resource id."]
    #[serde(rename = "networkFabricId")]
    pub network_fabric_id: String,
    #[doc = "vlanId. Example: 501."]
    #[serde(rename = "vlanId")]
    pub vlan_id: i32,
    #[doc = "maximum transmission unit. Default value is 1500."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[doc = "List of resources the L2 Isolation Domain is disabled on. Can be either entire NetworkFabric or NetworkRack."]
    #[serde(
        rename = "disabledOnResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disabled_on_resources: Vec<String>,
    #[doc = "EnabledDisabledState state for the resource."]
    #[serde(rename = "administrativeState", default, skip_serializing_if = "Option::is_none")]
    pub administrative_state: Option<EnabledDisabledState>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl L2IsolationDomainProperties {
    pub fn new(network_fabric_id: String, vlan_id: i32) -> Self {
        Self {
            annotation_resource: AnnotationResource::default(),
            network_fabric_id,
            vlan_id,
            mtu: None,
            disabled_on_resources: Vec::new(),
            administrative_state: None,
            provisioning_state: None,
        }
    }
}
#[doc = "List of L2IsolationDomains."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct L2IsolationDomainsListResult {
    #[doc = "Displays list of L2IsolationDomain resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<L2IsolationDomain>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for L2IsolationDomainsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl L2IsolationDomainsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The L3IsolationDomain resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct L3IsolationDomain {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "L3IsolationDomainProperties define the resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<L3IsolationDomainProperties>,
}
impl L3IsolationDomain {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The L3IsolationDomain patch resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct L3IsolationDomainPatch {
    #[doc = "L3IsolationDomainPatchProperties define the patch resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<L3IsolationDomainPatchProperties>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl L3IsolationDomainPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "L3IsolationDomainPatchProperties define the patch resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct L3IsolationDomainPatchProperties {
    #[doc = "Advertise Connected Subnets. Ex: \"True\" | \"False\"."]
    #[serde(rename = "redistributeConnectedSubnets", default, skip_serializing_if = "Option::is_none")]
    pub redistribute_connected_subnets: Option<l3_isolation_domain_patch_properties::RedistributeConnectedSubnets>,
    #[doc = "Advertise Static Routes. Ex: \"True\" | \"False\"."]
    #[serde(rename = "redistributeStaticRoutes", default, skip_serializing_if = "Option::is_none")]
    pub redistribute_static_routes: Option<l3_isolation_domain_patch_properties::RedistributeStaticRoutes>,
    #[doc = "List of Ipv4 and Ipv6 route configurations."]
    #[serde(rename = "aggregateRouteConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub aggregate_route_configuration: Option<l3_isolation_domain_patch_properties::AggregateRouteConfiguration>,
    #[doc = "L3 Isolation Domain description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Connected Subnet RoutePolicy"]
    #[serde(rename = "connectedSubnetRoutePolicy", default, skip_serializing_if = "Option::is_none")]
    pub connected_subnet_route_policy: Option<l3_isolation_domain_patch_properties::ConnectedSubnetRoutePolicy>,
}
impl L3IsolationDomainPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod l3_isolation_domain_patch_properties {
    use super::*;
    #[doc = "Advertise Connected Subnets. Ex: \"True\" | \"False\"."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RedistributeConnectedSubnets")]
    pub enum RedistributeConnectedSubnets {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RedistributeConnectedSubnets {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RedistributeConnectedSubnets {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RedistributeConnectedSubnets {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("RedistributeConnectedSubnets", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("RedistributeConnectedSubnets", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for RedistributeConnectedSubnets {
        fn default() -> Self {
            Self::True
        }
    }
    #[doc = "Advertise Static Routes. Ex: \"True\" | \"False\"."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RedistributeStaticRoutes")]
    pub enum RedistributeStaticRoutes {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RedistributeStaticRoutes {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RedistributeStaticRoutes {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RedistributeStaticRoutes {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("RedistributeStaticRoutes", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("RedistributeStaticRoutes", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for RedistributeStaticRoutes {
        fn default() -> Self {
            Self::False
        }
    }
    #[doc = "List of Ipv4 and Ipv6 route configurations."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct AggregateRouteConfiguration {
        #[doc = "List of Ipv4Route prefixes."]
        #[serde(
            rename = "ipv4Routes",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub ipv4_routes: Vec<serde_json::Value>,
        #[doc = "List of Ipv6Routes prefixes."]
        #[serde(
            rename = "ipv6Routes",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub ipv6_routes: Vec<serde_json::Value>,
    }
    impl AggregateRouteConfiguration {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Connected Subnet RoutePolicy"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ConnectedSubnetRoutePolicy {
        #[doc = "exportRoutePolicyId value."]
        #[serde(rename = "exportRoutePolicyId", default, skip_serializing_if = "Option::is_none")]
        pub export_route_policy_id: Option<String>,
        #[doc = "EnabledDisabledState state for the resource."]
        #[serde(rename = "administrativeState", default, skip_serializing_if = "Option::is_none")]
        pub administrative_state: Option<EnabledDisabledState>,
    }
    impl ConnectedSubnetRoutePolicy {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "L3IsolationDomainProperties define the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct L3IsolationDomainProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[serde(flatten)]
    pub l3_isolation_domain_patch_properties: L3IsolationDomainPatchProperties,
    #[doc = "Network Fabric ARM resource id."]
    #[serde(rename = "networkFabricId")]
    pub network_fabric_id: String,
    #[doc = "List of resources the L3 Isolation Domain is disabled on. Can be either entire NetworkFabric or NetworkRack."]
    #[serde(
        rename = "disabledOnResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disabled_on_resources: Vec<String>,
    #[doc = "EnabledDisabledState state for the resource."]
    #[serde(rename = "administrativeState", default, skip_serializing_if = "Option::is_none")]
    pub administrative_state: Option<EnabledDisabledState>,
    #[doc = "List of resources the OptionB is disabled on. Can be either entire NetworkFabric or NetworkRack."]
    #[serde(
        rename = "optionBDisabledOnResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub option_b_disabled_on_resources: Vec<String>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl L3IsolationDomainProperties {
    pub fn new(network_fabric_id: String) -> Self {
        Self {
            annotation_resource: AnnotationResource::default(),
            l3_isolation_domain_patch_properties: L3IsolationDomainPatchProperties::default(),
            network_fabric_id,
            disabled_on_resources: Vec::new(),
            administrative_state: None,
            option_b_disabled_on_resources: Vec::new(),
            provisioning_state: None,
        }
    }
}
#[doc = "List of L3IsolationDomains."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct L3IsolationDomainsListResult {
    #[doc = "List of L3IsolationDomain resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<L3IsolationDomain>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for L3IsolationDomainsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl L3IsolationDomainsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "layer2Configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Layer2Configuration {
    #[doc = "Number of ports connected between PE/CE. Maximum value depends on FabricSKU."]
    #[serde(rename = "portCount", default, skip_serializing_if = "Option::is_none")]
    pub port_count: Option<i32>,
    #[doc = "MTU of the packets between PE & CE."]
    pub mtu: i32,
    #[doc = "List of network device interfaces resource IDs."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub interfaces: Vec<String>,
}
impl Layer2Configuration {
    pub fn new(mtu: i32) -> Self {
        Self {
            port_count: None,
            mtu,
            interfaces: Vec::new(),
        }
    }
}
#[doc = "layer3Configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Layer3Configuration {
    #[serde(flatten)]
    pub layer3_ip_prefix_properties: Layer3IpPrefixProperties,
    #[doc = "importRoutePolicyId"]
    #[serde(rename = "importRoutePolicyId", default, skip_serializing_if = "Option::is_none")]
    pub import_route_policy_id: Option<String>,
    #[doc = "exportRoutePolicyId"]
    #[serde(rename = "exportRoutePolicyId", default, skip_serializing_if = "Option::is_none")]
    pub export_route_policy_id: Option<String>,
    #[doc = "ASN of PE devices for CE/PE connectivity.Example : 28"]
    #[serde(rename = "peerASN", default, skip_serializing_if = "Option::is_none")]
    pub peer_asn: Option<i32>,
    #[doc = "VLAN for CE/PE Layer 3 connectivity.Example : 501"]
    #[serde(rename = "vlanId", default, skip_serializing_if = "Option::is_none")]
    pub vlan_id: Option<i32>,
    #[doc = "ASN of CE devices for CE/PE connectivity."]
    #[serde(rename = "fabricASN", default, skip_serializing_if = "Option::is_none")]
    pub fabric_asn: Option<i32>,
}
impl Layer3Configuration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Layer 3 primary and secondary ip address prefixes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Layer3IpPrefixProperties {
    #[doc = "IPv4 Address Prefix of CE-PE interconnect links. Default value is 172.23.1.0/31. The values can be specified at the time of creation or can be updated afterwards. Any update to the values post-provisioning may disrupt traffic. The 1st and 3rd IPs are to be configured on CE1 and CE2 for Option B interfaces. The 2nd and 4th IPs are to be configured on PE1 and PE2 for Option B interfaces."]
    #[serde(rename = "primaryIpv4Prefix", default, skip_serializing_if = "Option::is_none")]
    pub primary_ipv4_prefix: Option<String>,
    #[doc = "IPv6 Address Prefix of CE-PE interconnect links. Default value is 3FFE:FFFF:0:CD30::a1/126. The values can be specified at the time of creation or can be updated afterwards. Any update to the values post-provisioning may disrupt traffic. The 1st and 3rd IPs are to be configured on CE1 and CE2 for Option B interfaces. The 2nd and 4th IPs are to be configured on PE1 and PE2 for Option B interfaces."]
    #[serde(rename = "primaryIpv6Prefix", default, skip_serializing_if = "Option::is_none")]
    pub primary_ipv6_prefix: Option<String>,
    #[doc = "Secondary IPv4 Address Prefix of CE-PE interconnect links. Default value is 172.23.1.2/31. The values can be specified at the time of creation or can be updated afterwards. Any update to the values post-provisioning may disrupt traffic. The 1st and 3rd IPs are to be configured on CE1 and CE2 for Option B interfaces. The 2nd and 4th IPs are to be configured on PE1 and PE2 for Option B interfaces."]
    #[serde(rename = "secondaryIpv4Prefix", default, skip_serializing_if = "Option::is_none")]
    pub secondary_ipv4_prefix: Option<String>,
    #[doc = "Secondary IPv6 Address Prefix of CE-PE interconnect links. Default value is 3FFE:FFFF:0:CD30::a1/126. The values can be specified at the time of creation or can be updated afterwards. Any update to the values post-provisioning may disrupt traffic. The 1st and 3rd IPs are to be configured on CE1 and CE2 for Option B interfaces. The 2nd and 4th IPs are to be configured on PE1 and PE2 for Option B interfaces."]
    #[serde(rename = "secondaryIpv6Prefix", default, skip_serializing_if = "Option::is_none")]
    pub secondary_ipv6_prefix: Option<String>,
}
impl Layer3IpPrefixProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The NetworkDevice resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkDevice {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "NetworkDeviceProperties define the resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkDeviceProperties>,
}
impl NetworkDevice {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The NetworkDevicePatchParameters resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkDevicePatchParameters {
    #[doc = "Network Device Patch properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl NetworkDevicePatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkDevicePatchableProperties {
    #[doc = "The host Name of the device."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "serialNumber of the format Make;Model;HardwareRevisionId;SerialNumber. Example: Arista;DCS-7280DR3-24;12.05;JPE21116969"]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
}
impl NetworkDevicePatchableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NetworkDeviceProperties define the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkDeviceProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[serde(flatten)]
    pub network_device_patchable_properties: NetworkDevicePatchableProperties,
    #[doc = "Current version of the device as defined in SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Network Device SKU name."]
    #[serde(rename = "networkDeviceSku")]
    pub network_device_sku: String,
    #[doc = "Available roles for the network device."]
    #[serde(rename = "networkDeviceRole")]
    pub network_device_role: NetworkDeviceRoleTypes,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Reference to network rack resource id."]
    #[serde(rename = "networkRackId", default, skip_serializing_if = "Option::is_none")]
    pub network_rack_id: Option<String>,
}
impl NetworkDeviceProperties {
    pub fn new(network_device_sku: String, network_device_role: NetworkDeviceRoleTypes) -> Self {
        Self {
            annotation_resource: AnnotationResource::default(),
            network_device_patchable_properties: NetworkDevicePatchableProperties::default(),
            version: None,
            network_device_sku,
            network_device_role,
            provisioning_state: None,
            network_rack_id: None,
        }
    }
}
#[doc = "Available roles for the network device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NetworkDeviceRoleTypes")]
pub enum NetworkDeviceRoleTypes {
    #[serde(rename = "CE")]
    Ce,
    ToR,
    #[serde(rename = "NPB")]
    Npb,
    #[serde(rename = "TS")]
    Ts,
    Management,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NetworkDeviceRoleTypes {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NetworkDeviceRoleTypes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NetworkDeviceRoleTypes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ce => serializer.serialize_unit_variant("NetworkDeviceRoleTypes", 0u32, "CE"),
            Self::ToR => serializer.serialize_unit_variant("NetworkDeviceRoleTypes", 1u32, "ToR"),
            Self::Npb => serializer.serialize_unit_variant("NetworkDeviceRoleTypes", 2u32, "NPB"),
            Self::Ts => serializer.serialize_unit_variant("NetworkDeviceRoleTypes", 3u32, "TS"),
            Self::Management => serializer.serialize_unit_variant("NetworkDeviceRoleTypes", 4u32, "Management"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The NetworkDeviceSku resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkDeviceSku {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "NetworkDeviceSkuProperties define the resource properties."]
    pub properties: NetworkDeviceSkuProperties,
}
impl NetworkDeviceSku {
    pub fn new(properties: NetworkDeviceSkuProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "NetworkDeviceSkuProperties define the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkDeviceSkuProperties {
    #[doc = "Model of the network device."]
    pub model: String,
    #[doc = "Manufacturer of the network device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[doc = "List of network device interfaces."]
    #[serde(
        rename = "supportedVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_versions: Vec<serde_json::Value>,
    #[doc = "Network device limits."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<network_device_sku_properties::Limits>,
    #[doc = "Available roles for the network device."]
    #[serde(
        rename = "supportedRoleTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_role_types: Vec<String>,
    #[doc = "List of network device interfaces."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub interfaces: Vec<serde_json::Value>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl NetworkDeviceSkuProperties {
    pub fn new(model: String) -> Self {
        Self {
            model,
            manufacturer: None,
            supported_versions: Vec::new(),
            limits: None,
            supported_role_types: Vec::new(),
            interfaces: Vec::new(),
            provisioning_state: None,
        }
    }
}
pub mod network_device_sku_properties {
    use super::*;
    #[doc = "Network device limits."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Limits {
        #[doc = "Maximum number of physical interfaces."]
        #[serde(rename = "physicalInterfaceCount", default, skip_serializing_if = "Option::is_none")]
        pub physical_interface_count: Option<i32>,
        #[doc = "Maximum number of sub-interfaces."]
        #[serde(rename = "maxSubInterfaces", default, skip_serializing_if = "Option::is_none")]
        pub max_sub_interfaces: Option<i32>,
        #[doc = "Maximum number of tunnel interfaces."]
        #[serde(rename = "maxTunnelInterfaces", default, skip_serializing_if = "Option::is_none")]
        pub max_tunnel_interfaces: Option<i32>,
        #[doc = "Maximum number of virtual router functions."]
        #[serde(rename = "maxVirtualRouterFunctions", default, skip_serializing_if = "Option::is_none")]
        pub max_virtual_router_functions: Option<i32>,
        #[doc = "Maximum number of Border Gateway Protocol (BGP) peers."]
        #[serde(rename = "maxBorderGatewayProtocolPeers", default, skip_serializing_if = "Option::is_none")]
        pub max_border_gateway_protocol_peers: Option<i32>,
        #[doc = "Maximum number of Bidirectional Forwarding Detection (BFD) peers."]
        #[serde(
            rename = "maxBidirectionalForwardingDetectionPeers",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_bidirectional_forwarding_detection_peers: Option<i32>,
    }
    impl Limits {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "List of NetworkDeviceSkus."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkDeviceSkusListResult {
    #[doc = "List of NetworkDeviceSku resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkDeviceSku>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkDeviceSkusListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkDeviceSkusListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of NetworkDevices."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkDevicesListResult {
    #[doc = "List of NetworkDevice resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkDevice>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkDevicesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkDevicesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The NetworkFabric resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkFabric {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "NetworkFabricProperties - define the resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkFabricProperties>,
}
impl NetworkFabric {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The NetworkFabricController resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkFabricController {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "NetworkFabricControllerProperties define the resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkFabricControllerProperties>,
}
impl NetworkFabricController {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Operational state for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NetworkFabricControllerOperationalState")]
pub enum NetworkFabricControllerOperationalState {
    Configuring,
    Succeeded,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NetworkFabricControllerOperationalState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NetworkFabricControllerOperationalState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NetworkFabricControllerOperationalState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Configuring => serializer.serialize_unit_variant("NetworkFabricControllerOperationalState", 0u32, "Configuring"),
            Self::Succeeded => serializer.serialize_unit_variant("NetworkFabricControllerOperationalState", 1u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("NetworkFabricControllerOperationalState", 2u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The NetworkFabricControllerPatch payload definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFabricControllerPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkFabricControllerPatchableProperties>,
    #[doc = "Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl NetworkFabricControllerPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFabricControllerPatchableProperties {
    #[doc = "As part of an update, the Infrastructure ExpressRoute CircuitID should be provided to create and Provision a NFC. This Express route is dedicated for Infrastructure services. (This is a Mandatory attribute)"]
    #[serde(
        rename = "infrastructureExpressRouteConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub infrastructure_express_route_connections: Vec<ExpressRouteConnectionInformation>,
    #[doc = "As part of an update, the workload ExpressRoute CircuitID should be provided to create and Provision a NFC. This Express route is dedicated for Workload services. (This is a Mandatory attribute)."]
    #[serde(
        rename = "workloadExpressRouteConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub workload_express_route_connections: Vec<ExpressRouteConnectionInformation>,
}
impl NetworkFabricControllerPatchableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NetworkFabricControllerProperties define the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFabricControllerProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[serde(flatten)]
    pub network_fabric_controller_patchable_properties: NetworkFabricControllerPatchableProperties,
    #[doc = "InfrastructureServices IP ranges."]
    #[serde(rename = "infrastructureServices", default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_services: Option<network_fabric_controller_properties::InfrastructureServices>,
    #[doc = "WorkloadServices IP ranges."]
    #[serde(rename = "workloadServices", default, skip_serializing_if = "Option::is_none")]
    pub workload_services: Option<network_fabric_controller_properties::WorkloadServices>,
    #[doc = "Managed Resource Group configuration properties."]
    #[serde(rename = "managedResourceGroupConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_configuration: Option<network_fabric_controller_properties::ManagedResourceGroupConfiguration>,
    #[doc = "The NF-ID will be an input parameter used by the NF to link and get associated with the parent NFC Service."]
    #[serde(
        rename = "networkFabricIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_fabric_ids: Vec<String>,
    #[doc = "A workload management network is required for all the tenant (workload) traffic. This traffic is only dedicated for Tenant workloads which are required to access internet or any other MSFT/Public endpoints."]
    #[serde(rename = "workloadManagementNetwork", default, skip_serializing_if = "Option::is_none")]
    pub workload_management_network: Option<bool>,
    #[doc = "IPv4 Network Fabric Controller Address Space."]
    #[serde(rename = "ipv4AddressSpace", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_address_space: Option<String>,
    #[doc = "IPv6 Network Fabric Controller Address Space."]
    #[serde(rename = "ipv6AddressSpace", default, skip_serializing_if = "Option::is_none")]
    pub ipv6_address_space: Option<String>,
    #[doc = "Operational state for the resource."]
    #[serde(rename = "operationalState", default, skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<NetworkFabricControllerOperationalState>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl NetworkFabricControllerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_fabric_controller_properties {
    use super::*;
    #[doc = "InfrastructureServices IP ranges."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct InfrastructureServices {
        #[doc = "The IPv4 Address space is optional, if the value is not defined at the time of NFC creation, then the default value 10.0.0.0/19 is considered. The IPV4 address subnet is an optional attribute."]
        #[serde(
            rename = "ipv4AddressSpaces",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub ipv4_address_spaces: Vec<String>,
        #[doc = "The IPv6 is not supported right now."]
        #[serde(
            rename = "ipv6AddressSpaces",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub ipv6_address_spaces: Vec<String>,
    }
    impl InfrastructureServices {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "WorkloadServices IP ranges."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct WorkloadServices {
        #[doc = "The IPv4 Address space is optional, if the value is defined at the time of NFC creation, then the default value 10.0.0.0/19 is considered. The IPV4 address subnet is an optional attribute."]
        #[serde(
            rename = "ipv4AddressSpaces",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub ipv4_address_spaces: Vec<String>,
        #[doc = "The IPv6 is not supported right now."]
        #[serde(
            rename = "ipv6AddressSpaces",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub ipv6_address_spaces: Vec<String>,
    }
    impl WorkloadServices {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Managed Resource Group configuration properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ManagedResourceGroupConfiguration {
        #[doc = "The NFC service will be hosted in a Managed resource group."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[doc = "Managed resource group location."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
    }
    impl ManagedResourceGroupConfiguration {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "List of NetworkFabricControllers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFabricControllersListResult {
    #[doc = "List of NetworkFabricController resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkFabricController>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkFabricControllersListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkFabricControllersListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operational state for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NetworkFabricOperationalState")]
pub enum NetworkFabricOperationalState {
    Provisioning,
    Provisioned,
    ErrorProvisioning,
    Deprovisioning,
    Deprovisioned,
    ErrorDeprovisioning,
    DeferredControl,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NetworkFabricOperationalState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NetworkFabricOperationalState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NetworkFabricOperationalState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Provisioning => serializer.serialize_unit_variant("NetworkFabricOperationalState", 0u32, "Provisioning"),
            Self::Provisioned => serializer.serialize_unit_variant("NetworkFabricOperationalState", 1u32, "Provisioned"),
            Self::ErrorProvisioning => serializer.serialize_unit_variant("NetworkFabricOperationalState", 2u32, "ErrorProvisioning"),
            Self::Deprovisioning => serializer.serialize_unit_variant("NetworkFabricOperationalState", 3u32, "Deprovisioning"),
            Self::Deprovisioned => serializer.serialize_unit_variant("NetworkFabricOperationalState", 4u32, "Deprovisioned"),
            Self::ErrorDeprovisioning => serializer.serialize_unit_variant("NetworkFabricOperationalState", 5u32, "ErrorDeprovisioning"),
            Self::DeferredControl => serializer.serialize_unit_variant("NetworkFabricOperationalState", 6u32, "DeferredControl"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The NetworkFabric resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFabricPatchParameters {
    #[doc = "Network Fabric Patch properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl NetworkFabricPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFabricPatchableProperties {
    #[doc = "List of NetworkRack resource IDs under the Network Fabric. The number of racks allowed depends on the Network Fabric SKU."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub racks: Vec<String>,
    #[doc = "List of L2IsolationDomain resource IDs under the Network Fabric."]
    #[serde(
        rename = "l2IsolationDomains",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub l2_isolation_domains: Vec<String>,
    #[doc = "List of L3IsolationDomain resource IDs under the Network Fabric."]
    #[serde(
        rename = "l3IsolationDomains",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub l3_isolation_domains: Vec<String>,
}
impl NetworkFabricPatchableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NetworkFabricProperties - define the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkFabricProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[serde(flatten)]
    pub network_fabric_patchable_properties: NetworkFabricPatchableProperties,
    #[doc = "Supported Network Fabric SKU.Example: Compute / Aggregate racks. Once the user chooses a particular SKU, only supported racks can be added to the Network Fabric. The SKU determines whether it is a single / multi rack Network Fabric."]
    #[serde(rename = "networkFabricSku")]
    pub network_fabric_sku: String,
    #[doc = "Number of racks associated to Network Fabric.Possible values are from 2-8."]
    #[serde(rename = "rackCount")]
    pub rack_count: i32,
    #[doc = "Number of servers.Possible values are from 1-16."]
    #[serde(rename = "serverCountPerRack")]
    pub server_count_per_rack: i32,
    #[doc = "IPv4Prefix for Management Network. Default value : 10.1.0.0/19."]
    #[serde(rename = "ipv4Prefix", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_prefix: Option<String>,
    #[doc = "IPv6Prefix for Management Network. Default value 3FFE:FFFF:0:CD40::/59."]
    #[serde(rename = "ipv6Prefix", default, skip_serializing_if = "Option::is_none")]
    pub ipv6_prefix: Option<String>,
    #[doc = "Router Id of CE to be used for MP-BGP between PE and CE"]
    #[serde(rename = "routerId", default, skip_serializing_if = "Option::is_none")]
    pub router_id: Option<String>,
    #[doc = "ASN of CE devices for CE/PE connectivity."]
    #[serde(rename = "fabricASN")]
    pub fabric_asn: i32,
    #[doc = "Azure resource ID for the NetworkFabricController the NetworkFabric belongs."]
    #[serde(rename = "networkFabricControllerId")]
    pub network_fabric_controller_id: String,
    #[doc = "Network and credentials configuration currently applied to terminal server."]
    #[serde(rename = "terminalServerConfiguration")]
    pub terminal_server_configuration: network_fabric_properties::TerminalServerConfiguration,
    #[doc = "Configuration to be used to setup the management network."]
    #[serde(rename = "managementNetworkConfiguration")]
    pub management_network_configuration: network_fabric_properties::ManagementNetworkConfiguration,
    #[doc = "Operational state for the resource."]
    #[serde(rename = "operationalState", default, skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<NetworkFabricOperationalState>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl NetworkFabricProperties {
    pub fn new(
        network_fabric_sku: String,
        rack_count: i32,
        server_count_per_rack: i32,
        fabric_asn: i32,
        network_fabric_controller_id: String,
        terminal_server_configuration: network_fabric_properties::TerminalServerConfiguration,
        management_network_configuration: network_fabric_properties::ManagementNetworkConfiguration,
    ) -> Self {
        Self {
            annotation_resource: AnnotationResource::default(),
            network_fabric_patchable_properties: NetworkFabricPatchableProperties::default(),
            network_fabric_sku,
            rack_count,
            server_count_per_rack,
            ipv4_prefix: None,
            ipv6_prefix: None,
            router_id: None,
            fabric_asn,
            network_fabric_controller_id,
            terminal_server_configuration,
            management_network_configuration,
            operational_state: None,
            provisioning_state: None,
        }
    }
}
pub mod network_fabric_properties {
    use super::*;
    #[doc = "Network and credentials configuration currently applied to terminal server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct TerminalServerConfiguration {
        #[serde(flatten)]
        pub layer3_ip_prefix_properties: Layer3IpPrefixProperties,
        #[serde(flatten)]
        pub terminal_server_patchable_properties: TerminalServerPatchableProperties,
        #[doc = "ARM Resource ID used for the NetworkDevice."]
        #[serde(rename = "networkDeviceId", default, skip_serializing_if = "Option::is_none")]
        pub network_device_id: Option<String>,
    }
    impl TerminalServerConfiguration {
        pub fn new() -> Self {
            Self {
                layer3_ip_prefix_properties: Layer3IpPrefixProperties::default(),
                terminal_server_patchable_properties: TerminalServerPatchableProperties::default(),
                network_device_id: None,
            }
        }
    }
    #[doc = "Configuration to be used to setup the management network."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ManagementNetworkConfiguration {
        #[doc = "Configuration for infrastructure vpn."]
        #[serde(rename = "infrastructureVpnConfiguration")]
        pub infrastructure_vpn_configuration: management_network_configuration::InfrastructureVpnConfiguration,
        #[doc = "Configuration for workload vpn."]
        #[serde(rename = "workloadVpnConfiguration")]
        pub workload_vpn_configuration: management_network_configuration::WorkloadVpnConfiguration,
    }
    impl ManagementNetworkConfiguration {
        pub fn new(
            infrastructure_vpn_configuration: management_network_configuration::InfrastructureVpnConfiguration,
            workload_vpn_configuration: management_network_configuration::WorkloadVpnConfiguration,
        ) -> Self {
            Self {
                infrastructure_vpn_configuration,
                workload_vpn_configuration,
            }
        }
    }
    pub mod management_network_configuration {
        use super::*;
        #[doc = "Configuration for infrastructure vpn."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct InfrastructureVpnConfiguration {
            #[doc = "EnabledDisabledState state for the resource."]
            #[serde(rename = "administrativeState", default, skip_serializing_if = "Option::is_none")]
            pub administrative_state: Option<EnabledDisabledState>,
            #[doc = "Gets the networkToNetworkInterconnectId of the resource."]
            #[serde(rename = "networkToNetworkInterconnectId", default, skip_serializing_if = "Option::is_none")]
            pub network_to_network_interconnect_id: Option<String>,
            #[doc = "Peering option list."]
            #[serde(rename = "peeringOption", default, skip_serializing_if = "Option::is_none")]
            pub peering_option: Option<PeeringOption>,
            #[doc = "Option B configuration to be used for management vpn."]
            #[serde(rename = "optionBProperties")]
            pub option_b_properties: OptionBProperties,
            #[doc = "Peering optionA properties"]
            #[serde(rename = "optionAProperties", default, skip_serializing_if = "Option::is_none")]
            pub option_a_properties: Option<OptionAProperties>,
        }
        impl InfrastructureVpnConfiguration {
            pub fn new(option_b_properties: OptionBProperties) -> Self {
                Self {
                    administrative_state: None,
                    network_to_network_interconnect_id: None,
                    peering_option: None,
                    option_b_properties,
                    option_a_properties: None,
                }
            }
        }
        #[doc = "Configuration for workload vpn."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct WorkloadVpnConfiguration {
            #[doc = "EnabledDisabledState state for the resource."]
            #[serde(rename = "administrativeState", default, skip_serializing_if = "Option::is_none")]
            pub administrative_state: Option<EnabledDisabledState>,
            #[doc = "Peering option list."]
            #[serde(rename = "peeringOption", default, skip_serializing_if = "Option::is_none")]
            pub peering_option: Option<PeeringOption>,
            #[doc = "Gets the networkToNetworkInterconnectId of the resource."]
            #[serde(rename = "networkToNetworkInterconnectId", default, skip_serializing_if = "Option::is_none")]
            pub network_to_network_interconnect_id: Option<String>,
            #[doc = "Peering optionA properties"]
            #[serde(rename = "optionAProperties", default, skip_serializing_if = "Option::is_none")]
            pub option_a_properties: Option<OptionAProperties>,
            #[doc = "Option B configuration to be used for management vpn."]
            #[serde(rename = "optionBProperties")]
            pub option_b_properties: OptionBProperties,
        }
        impl WorkloadVpnConfiguration {
            pub fn new(option_b_properties: OptionBProperties) -> Self {
                Self {
                    administrative_state: None,
                    peering_option: None,
                    network_to_network_interconnect_id: None,
                    option_a_properties: None,
                    option_b_properties,
                }
            }
        }
    }
}
#[doc = "The NetworkFabricSku resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkFabricSku {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "NetworkFabricSkuProperties define the resource properties."]
    pub properties: NetworkFabricSkuProperties,
}
impl NetworkFabricSku {
    pub fn new(properties: NetworkFabricSkuProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "NetworkFabricSkuProperties define the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFabricSkuProperties {
    #[doc = "Type of Network Fabric Sku."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Maximum number of compute racks available for this Network Fabric SKU."]
    #[serde(rename = "maxComputeRacks", default, skip_serializing_if = "Option::is_none")]
    pub max_compute_racks: Option<i32>,
    #[doc = "Minimum supported version."]
    #[serde(rename = "minSupportedVer", default, skip_serializing_if = "Option::is_none")]
    pub min_supported_ver: Option<String>,
    #[doc = "Maximum supported version."]
    #[serde(rename = "maxSupportedVer", default, skip_serializing_if = "Option::is_none")]
    pub max_supported_ver: Option<String>,
    #[doc = "The URI gives full details of sku."]
    #[serde(rename = "detailsUri", default, skip_serializing_if = "Option::is_none")]
    pub details_uri: Option<String>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl NetworkFabricSkuProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of NetworkFabricSkus."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFabricSkusListResult {
    #[doc = "List of NetworkFabricSku resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkFabricSku>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkFabricSkusListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkFabricSkusListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of NetworkFabrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFabricsListResult {
    #[doc = "List of NetworkFabric resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkFabric>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkFabricsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkFabricsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the NetworkInterface resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkInterface {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "NetworkInterfaceProperties define the resource properties."]
    pub properties: NetworkInterfaceProperties,
}
impl NetworkInterface {
    pub fn new(properties: NetworkInterfaceProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "The NetworkInterfacePatch resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfacePatch {
    #[doc = "Network Interface Patch properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl NetworkInterfacePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NetworkInterfaceProperties define the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[doc = "physicalIdentifier of the network interface."]
    #[serde(rename = "physicalIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub physical_identifier: Option<String>,
    #[doc = "EnabledDisabledState state for the resource."]
    #[serde(rename = "administrativeState", default, skip_serializing_if = "Option::is_none")]
    pub administrative_state: Option<EnabledDisabledState>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The arm resource id of the interface or compute server its connected to."]
    #[serde(rename = "connectedTo", default, skip_serializing_if = "Option::is_none")]
    pub connected_to: Option<String>,
    #[doc = "The Interface Type. Example: Management/Data"]
    #[serde(rename = "interfaceType", default, skip_serializing_if = "Option::is_none")]
    pub interface_type: Option<network_interface_properties::InterfaceType>,
    #[doc = "ipv4Address."]
    #[serde(rename = "ipv4Address", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    #[doc = "ipv6Address."]
    #[serde(rename = "ipv6Address", default, skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
}
impl NetworkInterfaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_interface_properties {
    use super::*;
    #[doc = "The Interface Type. Example: Management/Data"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InterfaceType")]
    pub enum InterfaceType {
        Management,
        Data,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InterfaceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InterfaceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InterfaceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Management => serializer.serialize_unit_variant("InterfaceType", 0u32, "Management"),
                Self::Data => serializer.serialize_unit_variant("InterfaceType", 1u32, "Data"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of NetworkInterfaces."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfacesList {
    #[doc = "List of NetworkInterfaces resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkInterface>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkInterfacesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkInterfacesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The NetworkRack resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRack {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "NetworkRackProperties define the resource properties."]
    pub properties: NetworkRackProperties,
}
impl NetworkRack {
    pub fn new(tracked_resource: TrackedResource, properties: NetworkRackProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "The NetworkRack patch resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRackPatch {
    #[doc = "NetworkRackPatchProperties define the patch resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkRackPatchProperties>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl NetworkRackPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NetworkRackPatchProperties define the patch resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRackPatchProperties {}
impl NetworkRackPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NetworkRackProperties define the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRackProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[doc = "Network Rack SKU name."]
    #[serde(rename = "networkRackSku")]
    pub network_rack_sku: String,
    #[doc = "Network Fabric ARM resource id."]
    #[serde(rename = "networkFabricId")]
    pub network_fabric_id: String,
    #[doc = "List of network device ARM resource ids."]
    #[serde(
        rename = "networkDevices",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_devices: Vec<String>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl NetworkRackProperties {
    pub fn new(network_rack_sku: String, network_fabric_id: String) -> Self {
        Self {
            annotation_resource: AnnotationResource::default(),
            network_rack_sku,
            network_fabric_id,
            network_devices: Vec::new(),
            provisioning_state: None,
        }
    }
}
#[doc = "The NetworkRackSku resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRackSku {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "NetworkRackSkuProperties define the resource properties."]
    pub properties: NetworkRackSkuProperties,
}
impl NetworkRackSku {
    pub fn new(properties: NetworkRackSkuProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "NetworkRackSkuProperties define the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRackSkuProperties {
    #[doc = "The role of the Network Rack: Aggregate or Compute."]
    #[serde(rename = "roleName")]
    pub role_name: network_rack_sku_properties::RoleName,
    #[doc = "Maximum number of servers available for this SKU."]
    #[serde(rename = "maximumServerCount", default, skip_serializing_if = "Option::is_none")]
    pub maximum_server_count: Option<i32>,
    #[doc = "Maximum number of storage devices available for this SKU."]
    #[serde(rename = "maximumStorageCount", default, skip_serializing_if = "Option::is_none")]
    pub maximum_storage_count: Option<i32>,
    #[doc = "Maximum number of network uplinks available for this SKU."]
    #[serde(rename = "maximumUplinks", default, skip_serializing_if = "Option::is_none")]
    pub maximum_uplinks: Option<i32>,
    #[doc = "List of network device properties / role for the Network Rack."]
    #[serde(
        rename = "networkDevices",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_devices: Vec<serde_json::Value>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl NetworkRackSkuProperties {
    pub fn new(role_name: network_rack_sku_properties::RoleName) -> Self {
        Self {
            role_name,
            maximum_server_count: None,
            maximum_storage_count: None,
            maximum_uplinks: None,
            network_devices: Vec::new(),
            provisioning_state: None,
        }
    }
}
pub mod network_rack_sku_properties {
    use super::*;
    #[doc = "The role of the Network Rack: Aggregate or Compute."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RoleName")]
    pub enum RoleName {
        ComputeRack,
        AggregateRack,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RoleName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RoleName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RoleName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ComputeRack => serializer.serialize_unit_variant("RoleName", 0u32, "ComputeRack"),
                Self::AggregateRack => serializer.serialize_unit_variant("RoleName", 1u32, "AggregateRack"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of NetworkRackSkus."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRackSkusListResult {
    #[doc = "List of NetworkRackSku resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkRackSku>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkRackSkusListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkRackSkusListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of NetworkRacks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRacksListResult {
    #[doc = "List of NetworkRack resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkRack>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkRacksListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkRacksListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The NetworkToNetworkInterconnect resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkToNetworkInterconnect {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Configuration used to setup CE-PE connectivity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkToNetworkInterconnectProperties>,
}
impl NetworkToNetworkInterconnect {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration used to setup CE-PE connectivity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkToNetworkInterconnectProperties {
    #[doc = "EnabledDisabledState state for the resource."]
    #[serde(rename = "administrativeState", default, skip_serializing_if = "Option::is_none")]
    pub administrative_state: Option<EnabledDisabledState>,
    #[doc = "Boolean Enum. Example- True/False"]
    #[serde(rename = "isManagementType")]
    pub is_management_type: BooleanEnumProperty,
    #[doc = "Boolean Enum. Example- True/False"]
    #[serde(rename = "useOptionB")]
    pub use_option_b: BooleanEnumProperty,
    #[doc = "Common properties for Layer2Configuration."]
    #[serde(rename = "layer2Configuration", default, skip_serializing_if = "Option::is_none")]
    pub layer2_configuration: Option<serde_json::Value>,
    #[doc = "layer3Configuration"]
    #[serde(rename = "layer3Configuration", default, skip_serializing_if = "Option::is_none")]
    pub layer3_configuration: Option<Layer3Configuration>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl NetworkToNetworkInterconnectProperties {
    pub fn new(is_management_type: BooleanEnumProperty, use_option_b: BooleanEnumProperty) -> Self {
        Self {
            administrative_state: None,
            is_management_type,
            use_option_b,
            layer2_configuration: None,
            layer3_configuration: None,
            provisioning_state: None,
        }
    }
}
#[doc = "List of NetworkToNetworkInterconnects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkToNetworkInterconnectsList {
    #[doc = "List of NetworkToNetworkInterconnects resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkToNetworkInterconnect>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkToNetworkInterconnectsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkToNetworkInterconnectsList {
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
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operational state for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationalState")]
pub enum OperationalState {
    Configuring,
    Succeeded,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationalState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationalState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationalState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Configuring => serializer.serialize_unit_variant("OperationalState", 0u32, "Configuring"),
            Self::Succeeded => serializer.serialize_unit_variant("OperationalState", 1u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("OperationalState", 2u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Peering optionA properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OptionAProperties {
    #[serde(flatten)]
    pub layer3_ip_prefix_properties: Layer3IpPrefixProperties,
    #[doc = "MTU to use for option A peering."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[doc = "Vlan identifier. Example : 501"]
    #[serde(rename = "vlanId", default, skip_serializing_if = "Option::is_none")]
    pub vlan_id: Option<i32>,
    #[doc = "Fabric ASN number. Example 65001 "]
    #[serde(rename = "fabricASN", default, skip_serializing_if = "Option::is_none")]
    pub fabric_asn: Option<i32>,
    #[doc = "Peer ASN number.Example : 28"]
    #[serde(rename = "peerASN", default, skip_serializing_if = "Option::is_none")]
    pub peer_asn: Option<i32>,
    #[doc = "BFD configuration properties"]
    #[serde(rename = "bfdConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub bfd_configuration: Option<BfdConfiguration>,
}
impl OptionAProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Option B configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OptionBProperties {
    #[doc = "Route Targets to be applied for incoming routes into CE."]
    #[serde(
        rename = "importRouteTargets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub import_route_targets: Vec<String>,
    #[doc = "Route Targets to be applied for outgoing routes from CE."]
    #[serde(
        rename = "exportRouteTargets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub export_route_targets: Vec<String>,
}
impl OptionBProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Peering option list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PeeringOption")]
pub enum PeeringOption {
    OptionA,
    OptionB,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PeeringOption {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PeeringOption {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PeeringOption {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::OptionA => serializer.serialize_unit_variant("PeeringOption", 0u32, "OptionA"),
            Self::OptionB => serializer.serialize_unit_variant("PeeringOption", 1u32, "OptionB"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Updating,
    Canceled,
    Deleting,
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
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
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
#[doc = "Generic network reachability state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReachabilityState")]
pub enum ReachabilityState {
    Reachable,
    Unreachable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReachabilityState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReachabilityState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReachabilityState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Reachable => serializer.serialize_unit_variant("ReachabilityState", 0u32, "Reachable"),
            Self::Unreachable => serializer.serialize_unit_variant("ReachabilityState", 1u32, "Unreachable"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "List of RoutePolicies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutePoliciesListResult {
    #[doc = "List of RoutePolicy resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RoutePolicy>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoutePoliciesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoutePoliciesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The RoutePolicy resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutePolicy {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "RoutePolicyProperties define the resource properties."]
    pub properties: RoutePolicyProperties,
}
impl RoutePolicy {
    pub fn new(tracked_resource: TrackedResource, properties: RoutePolicyProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "The RoutePolicy patch resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutePolicyPatch {
    #[doc = "RoutePolicyPatchProperties define the patch resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoutePolicyPatchProperties>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl RoutePolicyPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "RoutePolicyPatchProperties define the patch resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutePolicyPatchProperties {}
impl RoutePolicyPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "RoutePolicyProperties define the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutePolicyProperties {
    #[serde(flatten)]
    pub annotation_resource: AnnotationResource,
    #[doc = "Route Policy description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Route Policy conditions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions: Vec<serde_json::Value>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl RoutePolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Generate support package post action properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportPackageProperties {
    #[doc = "The URL to fetch the generated support package from."]
    #[serde(rename = "supportPackageURL")]
    pub support_package_url: String,
}
impl SupportPackageProperties {
    pub fn new(support_package_url: String) -> Self {
        Self { support_package_url }
    }
}
#[doc = "TerminalServerConnectivity state for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TerminalServerConnectivityState")]
pub enum TerminalServerConnectivityState {
    Ipv4Reachable,
    Ipv4Unreachable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TerminalServerConnectivityState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TerminalServerConnectivityState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TerminalServerConnectivityState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ipv4Reachable => serializer.serialize_unit_variant("TerminalServerConnectivityState", 0u32, "Ipv4Reachable"),
            Self::Ipv4Unreachable => serializer.serialize_unit_variant("TerminalServerConnectivityState", 1u32, "Ipv4Unreachable"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TerminalServerPatchParameters {
    #[doc = "Network and credentials configuration already applied to terminal server."]
    #[serde(rename = "terminalServerConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub terminal_server_configuration: Option<serde_json::Value>,
}
impl TerminalServerPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network and credential configuration currently applied on terminal server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TerminalServerPatchableProperties {
    #[doc = "Username for the terminal server connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "Password for the terminal server connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Serial Number of Terminal server."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
}
impl TerminalServerPatchableProperties {
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
#[doc = "Update administrative state on list of resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateAdministrativeState {
    #[serde(flatten)]
    pub enable_disable_on_resources: EnableDisableOnResources,
    #[doc = "Administrative state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<update_administrative_state::State>,
}
impl UpdateAdministrativeState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_administrative_state {
    use super::*;
    #[doc = "Administrative state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Enable,
        Disable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enable => serializer.serialize_unit_variant("State", 0u32, "Enable"),
                Self::Disable => serializer.serialize_unit_variant("State", 1u32, "Disable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Update power cycle input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePowerCycleProperties {
    #[doc = "Primary or Secondary power end."]
    #[serde(rename = "powerEnd")]
    pub power_end: update_power_cycle_properties::PowerEnd,
    #[doc = "On or Off toggle state."]
    pub state: update_power_cycle_properties::State,
}
impl UpdatePowerCycleProperties {
    pub fn new(power_end: update_power_cycle_properties::PowerEnd, state: update_power_cycle_properties::State) -> Self {
        Self { power_end, state }
    }
}
pub mod update_power_cycle_properties {
    use super::*;
    #[doc = "Primary or Secondary power end."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PowerEnd")]
    pub enum PowerEnd {
        Primary,
        Secondary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PowerEnd {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PowerEnd {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PowerEnd {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("PowerEnd", 0u32, "Primary"),
                Self::Secondary => serializer.serialize_unit_variant("PowerEnd", 1u32, "Secondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "On or Off toggle state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        On,
        Off,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::On => serializer.serialize_unit_variant("State", 0u32, "On"),
                Self::Off => serializer.serialize_unit_variant("State", 1u32, "Off"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Generate support package post action properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateVersionProperties {
    #[doc = "The supported version defined in network device SKU."]
    #[serde(rename = "skuVersion")]
    pub sku_version: String,
}
impl UpdateVersionProperties {
    pub fn new(sku_version: String) -> Self {
        Self { sku_version }
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

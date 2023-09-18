#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "AddressSpace contains an array of IP address ranges that can be used by subnets of the virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddressSpace {
    #[doc = "A list of address blocks reserved for this virtual network in CIDR notation."]
    #[serde(
        rename = "addressPrefixes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub address_prefixes: Vec<String>,
}
impl AddressSpace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Backend address of an application gateway."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationGatewayBackendAddress {
    #[doc = "Fully qualified domain name (FQDN)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "IP address"]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
impl ApplicationGatewayBackendAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Backend Address Pool of an application gateway."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationGatewayBackendAddressPool {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of Backend Address Pool of an application gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationGatewayBackendAddressPoolPropertiesFormat>,
    #[doc = "Name of the backend address pool that is unique within an Application Gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ApplicationGatewayBackendAddressPool {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Backend Address Pool of an application gateway."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationGatewayBackendAddressPoolPropertiesFormat {
    #[doc = "Collection of references to IPs defined in network interfaces."]
    #[serde(
        rename = "backendIPConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub backend_ip_configurations: Vec<NetworkInterfaceIpConfiguration>,
    #[doc = "Backend addresses"]
    #[serde(
        rename = "backendAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub backend_addresses: Vec<ApplicationGatewayBackendAddress>,
    #[doc = "Provisioning state of the backend address pool resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ApplicationGatewayBackendAddressPoolPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An application security group in a resource group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationSecurityGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Application security group properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationSecurityGroupPropertiesFormat>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ApplicationSecurityGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application security group properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationSecurityGroupPropertiesFormat {
    #[doc = "The resource GUID property of the application security group resource. It uniquely identifies a resource, even if the user changes its name or migrate the resource across subscriptions or resource groups."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[doc = "The provisioning state of the application security group resource. Possible values are: 'Succeeded', 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ApplicationSecurityGroupPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Availability of the metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Availability {
    #[doc = "The time grain of the availability."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "The retention of the availability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<String>,
    #[doc = "Duration of the availability blob."]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl Availability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response body contains the status of the specified asynchronous operation, indicating whether it has succeeded, is in progress, or has failed. Note that this status is distinct from the HTTP status code returned for the Get Operation Status operation itself. If the asynchronous operation succeeded, the response body includes the HTTP status code for the successful request. If the asynchronous operation failed, the response body includes the HTTP status code for the failed request and error information regarding the failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureAsyncOperationResult {
    #[doc = "Status of the Azure async operation. Possible values are: 'InProgress', 'Succeeded', and 'Failed'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<azure_async_operation_result::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
impl AzureAsyncOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod azure_async_operation_result {
    use super::*;
    #[doc = "Status of the Azure async operation. Possible values are: 'InProgress', 'Succeeded', and 'Failed'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        InProgress,
        Succeeded,
        Failed,
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
                Self::InProgress => serializer.serialize_unit_variant("Status", 0u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Pool of backend IP addresses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendAddressPool {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of the backend address pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackendAddressPoolPropertiesFormat>,
    #[doc = "Gets name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl BackendAddressPool {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the backend address pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendAddressPoolPropertiesFormat {
    #[doc = "Gets collection of references to IP addresses defined in network interfaces."]
    #[serde(
        rename = "backendIPConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub backend_ip_configurations: Vec<NetworkInterfaceIpConfiguration>,
    #[doc = "Gets load balancing rules that use this backend address pool."]
    #[serde(
        rename = "loadBalancingRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub load_balancing_rules: Vec<SubResource>,
    #[doc = "Reference to another subresource."]
    #[serde(rename = "outboundRule", default, skip_serializing_if = "Option::is_none")]
    pub outbound_rule: Option<SubResource>,
    #[doc = "Gets outbound rules that use this backend address pool."]
    #[serde(
        rename = "outboundRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub outbound_rules: Vec<SubResource>,
    #[doc = "Get provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl BackendAddressPoolPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BGP peer status details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BgpPeerStatus {
    #[doc = "The virtual network gateway's local address"]
    #[serde(rename = "localAddress", default, skip_serializing_if = "Option::is_none")]
    pub local_address: Option<String>,
    #[doc = "The remote BGP peer"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neighbor: Option<String>,
    #[doc = "The autonomous system number of the remote BGP peer"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[doc = "The BGP peer state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<bgp_peer_status::State>,
    #[doc = "For how long the peering has been up"]
    #[serde(rename = "connectedDuration", default, skip_serializing_if = "Option::is_none")]
    pub connected_duration: Option<String>,
    #[doc = "The number of routes learned from this peer"]
    #[serde(rename = "routesReceived", default, skip_serializing_if = "Option::is_none")]
    pub routes_received: Option<i64>,
    #[doc = "The number of BGP messages sent"]
    #[serde(rename = "messagesSent", default, skip_serializing_if = "Option::is_none")]
    pub messages_sent: Option<i64>,
    #[doc = "The number of BGP messages received"]
    #[serde(rename = "messagesReceived", default, skip_serializing_if = "Option::is_none")]
    pub messages_received: Option<i64>,
}
impl BgpPeerStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod bgp_peer_status {
    use super::*;
    #[doc = "The BGP peer state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Unknown,
        Stopped,
        Idle,
        Connecting,
        Connected,
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
                Self::Unknown => serializer.serialize_unit_variant("State", 0u32, "Unknown"),
                Self::Stopped => serializer.serialize_unit_variant("State", 1u32, "Stopped"),
                Self::Idle => serializer.serialize_unit_variant("State", 2u32, "Idle"),
                Self::Connecting => serializer.serialize_unit_variant("State", 3u32, "Connecting"),
                Self::Connected => serializer.serialize_unit_variant("State", 4u32, "Connected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Response for list BGP peer status API service call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BgpPeerStatusListResult {
    #[doc = "List of BGP peers"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BgpPeerStatus>,
}
impl BgpPeerStatusListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BGP settings details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BgpSettings {
    #[doc = "The BGP speaker's ASN."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    #[doc = "The BGP peering address and BGP identifier of this BGP speaker."]
    #[serde(rename = "bgpPeeringAddress", default, skip_serializing_if = "Option::is_none")]
    pub bgp_peering_address: Option<String>,
    #[doc = "The weight added to routes learned from this BGP speaker."]
    #[serde(rename = "peerWeight", default, skip_serializing_if = "Option::is_none")]
    pub peer_weight: Option<i32>,
}
impl BgpSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gateway connection protocol. Possible values are: 'IKEv2', 'IKEv1'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ConnectionProtocol")]
pub enum ConnectionProtocol {
    #[serde(rename = "IKEv2")]
    IkEv2,
    #[serde(rename = "IKEv1")]
    IkEv1,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ConnectionProtocol {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ConnectionProtocol {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ConnectionProtocol {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::IkEv2 => serializer.serialize_unit_variant("ConnectionProtocol", 0u32, "IKEv2"),
            Self::IkEv1 => serializer.serialize_unit_variant("ConnectionProtocol", 1u32, "IKEv1"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The virtual network connection reset shared key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionResetSharedKey {
    #[doc = "The virtual network connection reset shared key length, should between 1 and 128."]
    #[serde(rename = "keyLength")]
    pub key_length: i32,
}
impl ConnectionResetSharedKey {
    pub fn new(key_length: i32) -> Self {
        Self { key_length }
    }
}
#[doc = "Response for GetConnectionSharedKey API service call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionSharedKey {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The virtual network connection shared key value."]
    pub value: String,
}
impl ConnectionSharedKey {
    pub fn new(value: String) -> Self {
        Self {
            sub_resource: SubResource::default(),
            value,
        }
    }
}
#[doc = "Contains the DDoS protection settings of the public IP."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DdosSettings {
    #[doc = "Reference to another subresource."]
    #[serde(rename = "ddosCustomPolicy", default, skip_serializing_if = "Option::is_none")]
    pub ddos_custom_policy: Option<SubResource>,
    #[doc = "The DDoS protection policy customizability of the public IP. Only standard coverage will have the ability to be customized."]
    #[serde(rename = "protectionCoverage", default, skip_serializing_if = "Option::is_none")]
    pub protection_coverage: Option<ddos_settings::ProtectionCoverage>,
}
impl DdosSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ddos_settings {
    use super::*;
    #[doc = "The DDoS protection policy customizability of the public IP. Only standard coverage will have the ability to be customized."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProtectionCoverage {
        Basic,
        Standard,
    }
}
#[doc = "Details the service to which the subnet is delegated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Delegation {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of a service delegation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceDelegationPropertiesFormat>,
    #[doc = "The name of the resource that is unique within a subnet. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Delegation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DhcpOptions contains an array of DNS servers available to VMs deployed in the virtual network. Standard DHCP option for a subnet overrides VNET DHCP options."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DhcpOptions {
    #[doc = "The list of DNS servers IP addresses."]
    #[serde(
        rename = "dnsServers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dns_servers: Vec<String>,
}
impl DhcpOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dimension of the metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[doc = "The name of the dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The display name of the dimension."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The internal name of the dimension."]
    #[serde(rename = "internalName", default, skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
}
impl Dimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Effective network security group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EffectiveNetworkSecurityGroup {
    #[doc = "Reference to another subresource."]
    #[serde(rename = "networkSecurityGroup", default, skip_serializing_if = "Option::is_none")]
    pub network_security_group: Option<SubResource>,
    #[doc = "The effective network security group association."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub association: Option<EffectiveNetworkSecurityGroupAssociation>,
    #[doc = "A collection of effective security rules."]
    #[serde(
        rename = "effectiveSecurityRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub effective_security_rules: Vec<EffectiveNetworkSecurityRule>,
    #[doc = "Mapping of tags to list of IP Addresses included within the tag."]
    #[serde(rename = "tagMap", default, skip_serializing_if = "Option::is_none")]
    pub tag_map: Option<String>,
}
impl EffectiveNetworkSecurityGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The effective network security group association."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EffectiveNetworkSecurityGroupAssociation {
    #[doc = "Reference to another subresource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<SubResource>,
    #[doc = "Reference to another subresource."]
    #[serde(rename = "networkInterface", default, skip_serializing_if = "Option::is_none")]
    pub network_interface: Option<SubResource>,
}
impl EffectiveNetworkSecurityGroupAssociation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for list effective network security groups API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EffectiveNetworkSecurityGroupListResult {
    #[doc = "A list of effective network security groups."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EffectiveNetworkSecurityGroup>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl EffectiveNetworkSecurityGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Effective network security rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EffectiveNetworkSecurityRule {
    #[doc = "The name of the security rule specified by the user (if created by the user)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The network protocol this rule applies to. Possible values are: 'Tcp', 'Udp', and 'All'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<effective_network_security_rule::Protocol>,
    #[doc = "The source port or range."]
    #[serde(rename = "sourcePortRange", default, skip_serializing_if = "Option::is_none")]
    pub source_port_range: Option<String>,
    #[doc = "The destination port or range."]
    #[serde(rename = "destinationPortRange", default, skip_serializing_if = "Option::is_none")]
    pub destination_port_range: Option<String>,
    #[doc = "The source port ranges. Expected values include a single integer between 0 and 65535, a range using '-' as separator (e.g. 100-400), or an asterisk (*)"]
    #[serde(
        rename = "sourcePortRanges",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_port_ranges: Vec<String>,
    #[doc = "The destination port ranges. Expected values include a single integer between 0 and 65535, a range using '-' as separator (e.g. 100-400), or an asterisk (*)"]
    #[serde(
        rename = "destinationPortRanges",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub destination_port_ranges: Vec<String>,
    #[doc = "The source address prefix."]
    #[serde(rename = "sourceAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub source_address_prefix: Option<String>,
    #[doc = "The destination address prefix."]
    #[serde(rename = "destinationAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub destination_address_prefix: Option<String>,
    #[doc = "The source address prefixes. Expected values include CIDR IP ranges, Default Tags (VirtualNetwork, AzureLoadBalancer, Internet), System Tags, and the asterisk (*)."]
    #[serde(
        rename = "sourceAddressPrefixes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_address_prefixes: Vec<String>,
    #[doc = "The destination address prefixes. Expected values include CIDR IP ranges, Default Tags (VirtualNetwork, AzureLoadBalancer, Internet), System Tags, and the asterisk (*)."]
    #[serde(
        rename = "destinationAddressPrefixes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub destination_address_prefixes: Vec<String>,
    #[doc = "The expanded source address prefix."]
    #[serde(
        rename = "expandedSourceAddressPrefix",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub expanded_source_address_prefix: Vec<String>,
    #[doc = "Expanded destination address prefix."]
    #[serde(
        rename = "expandedDestinationAddressPrefix",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub expanded_destination_address_prefix: Vec<String>,
    #[doc = "Whether network traffic is allowed or denied. Possible values are: 'Allow' and 'Deny'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<effective_network_security_rule::Access>,
    #[doc = "The priority of the rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "The direction of the rule. Possible values are: 'Inbound and Outbound'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<effective_network_security_rule::Direction>,
}
impl EffectiveNetworkSecurityRule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod effective_network_security_rule {
    use super::*;
    #[doc = "The network protocol this rule applies to. Possible values are: 'Tcp', 'Udp', and 'All'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        Tcp,
        Udp,
        All,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Protocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Protocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Protocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tcp => serializer.serialize_unit_variant("Protocol", 0u32, "Tcp"),
                Self::Udp => serializer.serialize_unit_variant("Protocol", 1u32, "Udp"),
                Self::All => serializer.serialize_unit_variant("Protocol", 2u32, "All"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether network traffic is allowed or denied. Possible values are: 'Allow' and 'Deny'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Access")]
    pub enum Access {
        Allow,
        Deny,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Access {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Access {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Access {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("Access", 0u32, "Allow"),
                Self::Deny => serializer.serialize_unit_variant("Access", 1u32, "Deny"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The direction of the rule. Possible values are: 'Inbound and Outbound'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Direction")]
    pub enum Direction {
        Inbound,
        Outbound,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Direction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Direction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Direction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inbound => serializer.serialize_unit_variant("Direction", 0u32, "Inbound"),
                Self::Outbound => serializer.serialize_unit_variant("Direction", 1u32, "Outbound"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Effective Route"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EffectiveRoute {
    #[doc = "The name of the user defined route. This is optional."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Who created the route. Possible values are: 'Unknown', 'User', 'VirtualNetworkGateway', and 'Default'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<effective_route::Source>,
    #[doc = "The value of effective route. Possible values are: 'Active' and 'Invalid'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<effective_route::State>,
    #[doc = "The address prefixes of the effective routes in CIDR notation."]
    #[serde(
        rename = "addressPrefix",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub address_prefix: Vec<String>,
    #[doc = "The IP address of the next hop of the effective route."]
    #[serde(
        rename = "nextHopIpAddress",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub next_hop_ip_address: Vec<String>,
    #[doc = "The type of Azure hop the packet should be sent to. Possible values are: 'VirtualNetworkGateway', 'VnetLocal', 'Internet', 'VirtualAppliance', and 'None'."]
    #[serde(rename = "nextHopType", default, skip_serializing_if = "Option::is_none")]
    pub next_hop_type: Option<effective_route::NextHopType>,
}
impl EffectiveRoute {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod effective_route {
    use super::*;
    #[doc = "Who created the route. Possible values are: 'Unknown', 'User', 'VirtualNetworkGateway', and 'Default'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Source")]
    pub enum Source {
        Unknown,
        User,
        VirtualNetworkGateway,
        Default,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Source {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Source {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Source {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("Source", 0u32, "Unknown"),
                Self::User => serializer.serialize_unit_variant("Source", 1u32, "User"),
                Self::VirtualNetworkGateway => serializer.serialize_unit_variant("Source", 2u32, "VirtualNetworkGateway"),
                Self::Default => serializer.serialize_unit_variant("Source", 3u32, "Default"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The value of effective route. Possible values are: 'Active' and 'Invalid'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Active,
        Invalid,
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
                Self::Active => serializer.serialize_unit_variant("State", 0u32, "Active"),
                Self::Invalid => serializer.serialize_unit_variant("State", 1u32, "Invalid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of Azure hop the packet should be sent to. Possible values are: 'VirtualNetworkGateway', 'VnetLocal', 'Internet', 'VirtualAppliance', and 'None'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NextHopType")]
    pub enum NextHopType {
        VirtualNetworkGateway,
        VnetLocal,
        Internet,
        VirtualAppliance,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NextHopType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NextHopType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NextHopType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::VirtualNetworkGateway => serializer.serialize_unit_variant("NextHopType", 0u32, "VirtualNetworkGateway"),
                Self::VnetLocal => serializer.serialize_unit_variant("NextHopType", 1u32, "VnetLocal"),
                Self::Internet => serializer.serialize_unit_variant("NextHopType", 2u32, "Internet"),
                Self::VirtualAppliance => serializer.serialize_unit_variant("NextHopType", 3u32, "VirtualAppliance"),
                Self::None => serializer.serialize_unit_variant("NextHopType", 4u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Response for list effective route API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EffectiveRouteListResult {
    #[doc = "A list of effective routes."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EffectiveRoute>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl EffectiveRouteListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identifies the service being brought into the virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointService {
    #[doc = "A unique identifier of the service being referenced by the interface endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl EndpointService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetails>,
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<String>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Frontend IP address of the load balancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontendIpConfiguration {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of Frontend IP Configuration of the load balancer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FrontendIpConfigurationPropertiesFormat>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "A list of availability zones denoting the IP allocated for the resource needs to come from."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub zones: Vec<String>,
}
impl FrontendIpConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Frontend IP Configuration of the load balancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontendIpConfigurationPropertiesFormat {
    #[doc = "Read only. Inbound rules URIs that use this frontend IP."]
    #[serde(
        rename = "inboundNatRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inbound_nat_rules: Vec<SubResource>,
    #[doc = "Read only. Inbound pools URIs that use this frontend IP."]
    #[serde(
        rename = "inboundNatPools",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inbound_nat_pools: Vec<SubResource>,
    #[doc = "Read only. Outbound rules URIs that use this frontend IP."]
    #[serde(
        rename = "outboundRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub outbound_rules: Vec<SubResource>,
    #[doc = "Gets load balancing rules URIs that use this frontend IP."]
    #[serde(
        rename = "loadBalancingRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub load_balancing_rules: Vec<SubResource>,
    #[doc = "The private IP address of the IP configuration."]
    #[serde(rename = "privateIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "The Private IP allocation method. Possible values are: 'Static' and 'Dynamic'."]
    #[serde(rename = "privateIPAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_allocation_method: Option<frontend_ip_configuration_properties_format::PrivateIpAllocationMethod>,
    #[doc = "Subnet in a virtual network resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<Subnet>,
    #[doc = "Public IP address resource."]
    #[serde(rename = "publicIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<PublicIpAddress>,
    #[doc = "Reference to another subresource."]
    #[serde(rename = "publicIPPrefix", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_prefix: Option<SubResource>,
    #[doc = "Gets the provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl FrontendIpConfigurationPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod frontend_ip_configuration_properties_format {
    use super::*;
    #[doc = "The Private IP allocation method. Possible values are: 'Static' and 'Dynamic'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrivateIpAllocationMethod")]
    pub enum PrivateIpAllocationMethod {
        Static,
        Dynamic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrivateIpAllocationMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrivateIpAllocationMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrivateIpAllocationMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Static => serializer.serialize_unit_variant("PrivateIpAllocationMethod", 0u32, "Static"),
                Self::Dynamic => serializer.serialize_unit_variant("PrivateIpAllocationMethod", 1u32, "Dynamic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Gateway routing details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayRoute {
    #[doc = "The gateway's local address"]
    #[serde(rename = "localAddress", default, skip_serializing_if = "Option::is_none")]
    pub local_address: Option<String>,
    #[doc = "The route's network prefix"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[doc = "The route's next hop"]
    #[serde(rename = "nextHop", default, skip_serializing_if = "Option::is_none")]
    pub next_hop: Option<String>,
    #[doc = "The peer this route was learned from"]
    #[serde(rename = "sourcePeer", default, skip_serializing_if = "Option::is_none")]
    pub source_peer: Option<String>,
    #[doc = "The source this route was learned from"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The route's AS path sequence"]
    #[serde(rename = "asPath", default, skip_serializing_if = "Option::is_none")]
    pub as_path: Option<String>,
    #[doc = "The route's weight"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}
impl GatewayRoute {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of virtual network gateway routes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayRouteListResult {
    #[doc = "List of gateway routes"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GatewayRoute>,
}
impl GatewayRouteListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for CheckIPAddressAvailability API service call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpAddressAvailabilityResult {
    #[doc = "Private IP address availability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    #[doc = "Contains other available private IP addresses if the asked for address is taken."]
    #[serde(
        rename = "availableIPAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub available_ip_addresses: Vec<String>,
}
impl IpAddressAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IP configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpConfiguration {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of IP configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IpConfigurationPropertiesFormat>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl IpConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IP configuration profile child resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpConfigurationProfile {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "IP configuration profile properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IpConfigurationProfilePropertiesFormat>,
    #[doc = "The name of the resource. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Sub Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl IpConfigurationProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IP configuration profile properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpConfigurationProfilePropertiesFormat {
    #[doc = "Subnet in a virtual network resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<Subnet>,
    #[doc = "The provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl IpConfigurationProfilePropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of IP configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpConfigurationPropertiesFormat {
    #[doc = "The private IP address of the IP configuration."]
    #[serde(rename = "privateIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "The private IP allocation method. Possible values are 'Static' and 'Dynamic'."]
    #[serde(rename = "privateIPAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_allocation_method: Option<ip_configuration_properties_format::PrivateIpAllocationMethod>,
    #[doc = "Subnet in a virtual network resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<Subnet>,
    #[doc = "Public IP address resource."]
    #[serde(rename = "publicIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<PublicIpAddress>,
    #[doc = "Gets the provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl IpConfigurationPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ip_configuration_properties_format {
    use super::*;
    #[doc = "The private IP allocation method. Possible values are 'Static' and 'Dynamic'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrivateIpAllocationMethod")]
    pub enum PrivateIpAllocationMethod {
        Static,
        Dynamic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrivateIpAllocationMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrivateIpAllocationMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrivateIpAllocationMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Static => serializer.serialize_unit_variant("PrivateIpAllocationMethod", 0u32, "Static"),
                Self::Dynamic => serializer.serialize_unit_variant("PrivateIpAllocationMethod", 1u32, "Dynamic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Inbound NAT pool of the load balancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InboundNatPool {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of Inbound NAT pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InboundNatPoolPropertiesFormat>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl InboundNatPool {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Inbound NAT pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InboundNatPoolPropertiesFormat {
    #[doc = "Reference to another subresource."]
    #[serde(rename = "frontendIPConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub frontend_ip_configuration: Option<SubResource>,
    #[doc = "The transport protocol for the endpoint. Possible values are 'Udp' or 'Tcp' or 'All'."]
    pub protocol: TransportProtocol,
    #[doc = "The first port number in the range of external ports that will be used to provide Inbound Nat to NICs associated with a load balancer. Acceptable values range between 1 and 65534."]
    #[serde(rename = "frontendPortRangeStart")]
    pub frontend_port_range_start: i32,
    #[doc = "The last port number in the range of external ports that will be used to provide Inbound Nat to NICs associated with a load balancer. Acceptable values range between 1 and 65535."]
    #[serde(rename = "frontendPortRangeEnd")]
    pub frontend_port_range_end: i32,
    #[doc = "The port used for internal connections on the endpoint. Acceptable values are between 1 and 65535."]
    #[serde(rename = "backendPort")]
    pub backend_port: i32,
    #[doc = "The timeout for the TCP idle connection. The value can be set between 4 and 30 minutes. The default value is 4 minutes. This element is only used when the protocol is set to TCP."]
    #[serde(rename = "idleTimeoutInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_in_minutes: Option<i32>,
    #[doc = "Configures a virtual machine's endpoint for the floating IP capability required to configure a SQL AlwaysOn Availability Group. This setting is required when using the SQL AlwaysOn Availability Groups in SQL server. This setting can't be changed after you create the endpoint."]
    #[serde(rename = "enableFloatingIP", default, skip_serializing_if = "Option::is_none")]
    pub enable_floating_ip: Option<bool>,
    #[doc = "Receive bidirectional TCP Reset on TCP flow idle timeout or unexpected connection termination. This element is only used when the protocol is set to TCP."]
    #[serde(rename = "enableTcpReset", default, skip_serializing_if = "Option::is_none")]
    pub enable_tcp_reset: Option<bool>,
    #[doc = "Gets the provisioning state of the PublicIP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl InboundNatPoolPropertiesFormat {
    pub fn new(protocol: TransportProtocol, frontend_port_range_start: i32, frontend_port_range_end: i32, backend_port: i32) -> Self {
        Self {
            frontend_ip_configuration: None,
            protocol,
            frontend_port_range_start,
            frontend_port_range_end,
            backend_port,
            idle_timeout_in_minutes: None,
            enable_floating_ip: None,
            enable_tcp_reset: None,
            provisioning_state: None,
        }
    }
}
#[doc = "Inbound NAT rule of the load balancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InboundNatRule {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of the inbound NAT rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InboundNatRulePropertiesFormat>,
    #[doc = "Gets name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl InboundNatRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for ListInboundNatRule API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InboundNatRuleListResult {
    #[doc = "A list of inbound nat rules in a load balancer."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<InboundNatRule>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InboundNatRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl InboundNatRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the inbound NAT rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InboundNatRulePropertiesFormat {
    #[doc = "Reference to another subresource."]
    #[serde(rename = "frontendIPConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub frontend_ip_configuration: Option<SubResource>,
    #[doc = "IPConfiguration in a network interface."]
    #[serde(rename = "backendIPConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub backend_ip_configuration: Option<NetworkInterfaceIpConfiguration>,
    #[doc = "The transport protocol for the endpoint. Possible values are 'Udp' or 'Tcp' or 'All'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<TransportProtocol>,
    #[doc = "The port for the external endpoint. Port numbers for each rule must be unique within the Load Balancer. Acceptable values range from 1 to 65534."]
    #[serde(rename = "frontendPort", default, skip_serializing_if = "Option::is_none")]
    pub frontend_port: Option<i32>,
    #[doc = "The port used for the internal endpoint. Acceptable values range from 1 to 65535."]
    #[serde(rename = "backendPort", default, skip_serializing_if = "Option::is_none")]
    pub backend_port: Option<i32>,
    #[doc = "The timeout for the TCP idle connection. The value can be set between 4 and 30 minutes. The default value is 4 minutes. This element is only used when the protocol is set to TCP."]
    #[serde(rename = "idleTimeoutInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_in_minutes: Option<i32>,
    #[doc = "Configures a virtual machine's endpoint for the floating IP capability required to configure a SQL AlwaysOn Availability Group. This setting is required when using the SQL AlwaysOn Availability Groups in SQL server. This setting can't be changed after you create the endpoint."]
    #[serde(rename = "enableFloatingIP", default, skip_serializing_if = "Option::is_none")]
    pub enable_floating_ip: Option<bool>,
    #[doc = "Receive bidirectional TCP Reset on TCP flow idle timeout or unexpected connection termination. This element is only used when the protocol is set to TCP."]
    #[serde(rename = "enableTcpReset", default, skip_serializing_if = "Option::is_none")]
    pub enable_tcp_reset: Option<bool>,
    #[doc = "Gets the provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl InboundNatRulePropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Interface endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InterfaceEndpoint {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the interface endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InterfaceEndpointProperties>,
    #[doc = "Gets a unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl InterfaceEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the interface endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InterfaceEndpointProperties {
    #[doc = "A first-party service's FQDN that is mapped to the private IP allocated via this interface endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Identifies the service being brought into the virtual network."]
    #[serde(rename = "endpointService", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_service: Option<EndpointService>,
    #[doc = "Subnet in a virtual network resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<Subnet>,
    #[doc = "Gets an array of references to the network interfaces created for this interface endpoint."]
    #[serde(
        rename = "networkInterfaces",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_interfaces: Vec<NetworkInterface>,
    #[doc = "A read-only property that identifies who created this interface endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[doc = "The provisioning state of the interface endpoint. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl InterfaceEndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the IpTag associated with the object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpTag {
    #[doc = "Gets or sets the ipTag type: Example FirstPartyUsage."]
    #[serde(rename = "ipTagType", default, skip_serializing_if = "Option::is_none")]
    pub ip_tag_type: Option<String>,
    #[doc = "Gets or sets value of the IpTag associated with the public IP. Example SQL, Storage etc"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}
impl IpTag {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An IPSec Policy configuration for a virtual network gateway connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpsecPolicy {
    #[doc = "The IPSec Security Association (also called Quick Mode or Phase 2 SA) lifetime in seconds for a site to site VPN tunnel."]
    #[serde(rename = "saLifeTimeSeconds")]
    pub sa_life_time_seconds: i32,
    #[doc = "The IPSec Security Association (also called Quick Mode or Phase 2 SA) payload size in KB for a site to site VPN tunnel."]
    #[serde(rename = "saDataSizeKilobytes")]
    pub sa_data_size_kilobytes: i32,
    #[doc = "The IPSec encryption algorithm (IKE phase 1)."]
    #[serde(rename = "ipsecEncryption")]
    pub ipsec_encryption: ipsec_policy::IpsecEncryption,
    #[doc = "The IPSec integrity algorithm (IKE phase 1)."]
    #[serde(rename = "ipsecIntegrity")]
    pub ipsec_integrity: ipsec_policy::IpsecIntegrity,
    #[doc = "The IKE encryption algorithm (IKE phase 2)."]
    #[serde(rename = "ikeEncryption")]
    pub ike_encryption: ipsec_policy::IkeEncryption,
    #[doc = "The IKE integrity algorithm (IKE phase 2)."]
    #[serde(rename = "ikeIntegrity")]
    pub ike_integrity: ipsec_policy::IkeIntegrity,
    #[doc = "The DH Groups used in IKE Phase 1 for initial SA."]
    #[serde(rename = "dhGroup")]
    pub dh_group: ipsec_policy::DhGroup,
    #[doc = "The Pfs Groups used in IKE Phase 2 for new child SA."]
    #[serde(rename = "pfsGroup")]
    pub pfs_group: ipsec_policy::PfsGroup,
}
impl IpsecPolicy {
    pub fn new(
        sa_life_time_seconds: i32,
        sa_data_size_kilobytes: i32,
        ipsec_encryption: ipsec_policy::IpsecEncryption,
        ipsec_integrity: ipsec_policy::IpsecIntegrity,
        ike_encryption: ipsec_policy::IkeEncryption,
        ike_integrity: ipsec_policy::IkeIntegrity,
        dh_group: ipsec_policy::DhGroup,
        pfs_group: ipsec_policy::PfsGroup,
    ) -> Self {
        Self {
            sa_life_time_seconds,
            sa_data_size_kilobytes,
            ipsec_encryption,
            ipsec_integrity,
            ike_encryption,
            ike_integrity,
            dh_group,
            pfs_group,
        }
    }
}
pub mod ipsec_policy {
    use super::*;
    #[doc = "The IPSec encryption algorithm (IKE phase 1)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IpsecEncryption")]
    pub enum IpsecEncryption {
        None,
        #[serde(rename = "DES")]
        Des,
        #[serde(rename = "DES3")]
        Des3,
        #[serde(rename = "AES128")]
        Aes128,
        #[serde(rename = "AES192")]
        Aes192,
        #[serde(rename = "AES256")]
        Aes256,
        #[serde(rename = "GCMAES128")]
        Gcmaes128,
        #[serde(rename = "GCMAES192")]
        Gcmaes192,
        #[serde(rename = "GCMAES256")]
        Gcmaes256,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IpsecEncryption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IpsecEncryption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IpsecEncryption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("IpsecEncryption", 0u32, "None"),
                Self::Des => serializer.serialize_unit_variant("IpsecEncryption", 1u32, "DES"),
                Self::Des3 => serializer.serialize_unit_variant("IpsecEncryption", 2u32, "DES3"),
                Self::Aes128 => serializer.serialize_unit_variant("IpsecEncryption", 3u32, "AES128"),
                Self::Aes192 => serializer.serialize_unit_variant("IpsecEncryption", 4u32, "AES192"),
                Self::Aes256 => serializer.serialize_unit_variant("IpsecEncryption", 5u32, "AES256"),
                Self::Gcmaes128 => serializer.serialize_unit_variant("IpsecEncryption", 6u32, "GCMAES128"),
                Self::Gcmaes192 => serializer.serialize_unit_variant("IpsecEncryption", 7u32, "GCMAES192"),
                Self::Gcmaes256 => serializer.serialize_unit_variant("IpsecEncryption", 8u32, "GCMAES256"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The IPSec integrity algorithm (IKE phase 1)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IpsecIntegrity")]
    pub enum IpsecIntegrity {
        #[serde(rename = "MD5")]
        Md5,
        #[serde(rename = "SHA1")]
        Sha1,
        #[serde(rename = "SHA256")]
        Sha256,
        #[serde(rename = "GCMAES128")]
        Gcmaes128,
        #[serde(rename = "GCMAES192")]
        Gcmaes192,
        #[serde(rename = "GCMAES256")]
        Gcmaes256,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IpsecIntegrity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IpsecIntegrity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IpsecIntegrity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Md5 => serializer.serialize_unit_variant("IpsecIntegrity", 0u32, "MD5"),
                Self::Sha1 => serializer.serialize_unit_variant("IpsecIntegrity", 1u32, "SHA1"),
                Self::Sha256 => serializer.serialize_unit_variant("IpsecIntegrity", 2u32, "SHA256"),
                Self::Gcmaes128 => serializer.serialize_unit_variant("IpsecIntegrity", 3u32, "GCMAES128"),
                Self::Gcmaes192 => serializer.serialize_unit_variant("IpsecIntegrity", 4u32, "GCMAES192"),
                Self::Gcmaes256 => serializer.serialize_unit_variant("IpsecIntegrity", 5u32, "GCMAES256"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The IKE encryption algorithm (IKE phase 2)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IkeEncryption")]
    pub enum IkeEncryption {
        #[serde(rename = "DES")]
        Des,
        #[serde(rename = "DES3")]
        Des3,
        #[serde(rename = "AES128")]
        Aes128,
        #[serde(rename = "AES192")]
        Aes192,
        #[serde(rename = "AES256")]
        Aes256,
        #[serde(rename = "GCMAES256")]
        Gcmaes256,
        #[serde(rename = "GCMAES128")]
        Gcmaes128,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IkeEncryption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IkeEncryption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IkeEncryption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Des => serializer.serialize_unit_variant("IkeEncryption", 0u32, "DES"),
                Self::Des3 => serializer.serialize_unit_variant("IkeEncryption", 1u32, "DES3"),
                Self::Aes128 => serializer.serialize_unit_variant("IkeEncryption", 2u32, "AES128"),
                Self::Aes192 => serializer.serialize_unit_variant("IkeEncryption", 3u32, "AES192"),
                Self::Aes256 => serializer.serialize_unit_variant("IkeEncryption", 4u32, "AES256"),
                Self::Gcmaes256 => serializer.serialize_unit_variant("IkeEncryption", 5u32, "GCMAES256"),
                Self::Gcmaes128 => serializer.serialize_unit_variant("IkeEncryption", 6u32, "GCMAES128"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The IKE integrity algorithm (IKE phase 2)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IkeIntegrity")]
    pub enum IkeIntegrity {
        #[serde(rename = "MD5")]
        Md5,
        #[serde(rename = "SHA1")]
        Sha1,
        #[serde(rename = "SHA256")]
        Sha256,
        #[serde(rename = "SHA384")]
        Sha384,
        #[serde(rename = "GCMAES256")]
        Gcmaes256,
        #[serde(rename = "GCMAES128")]
        Gcmaes128,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IkeIntegrity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IkeIntegrity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IkeIntegrity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Md5 => serializer.serialize_unit_variant("IkeIntegrity", 0u32, "MD5"),
                Self::Sha1 => serializer.serialize_unit_variant("IkeIntegrity", 1u32, "SHA1"),
                Self::Sha256 => serializer.serialize_unit_variant("IkeIntegrity", 2u32, "SHA256"),
                Self::Sha384 => serializer.serialize_unit_variant("IkeIntegrity", 3u32, "SHA384"),
                Self::Gcmaes256 => serializer.serialize_unit_variant("IkeIntegrity", 4u32, "GCMAES256"),
                Self::Gcmaes128 => serializer.serialize_unit_variant("IkeIntegrity", 5u32, "GCMAES128"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The DH Groups used in IKE Phase 1 for initial SA."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DhGroup")]
    pub enum DhGroup {
        None,
        #[serde(rename = "DHGroup1")]
        DhGroup1,
        #[serde(rename = "DHGroup2")]
        DhGroup2,
        #[serde(rename = "DHGroup14")]
        DhGroup14,
        #[serde(rename = "DHGroup2048")]
        DhGroup2048,
        #[serde(rename = "ECP256")]
        Ecp256,
        #[serde(rename = "ECP384")]
        Ecp384,
        #[serde(rename = "DHGroup24")]
        DhGroup24,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DhGroup {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DhGroup {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DhGroup {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("DhGroup", 0u32, "None"),
                Self::DhGroup1 => serializer.serialize_unit_variant("DhGroup", 1u32, "DHGroup1"),
                Self::DhGroup2 => serializer.serialize_unit_variant("DhGroup", 2u32, "DHGroup2"),
                Self::DhGroup14 => serializer.serialize_unit_variant("DhGroup", 3u32, "DHGroup14"),
                Self::DhGroup2048 => serializer.serialize_unit_variant("DhGroup", 4u32, "DHGroup2048"),
                Self::Ecp256 => serializer.serialize_unit_variant("DhGroup", 5u32, "ECP256"),
                Self::Ecp384 => serializer.serialize_unit_variant("DhGroup", 6u32, "ECP384"),
                Self::DhGroup24 => serializer.serialize_unit_variant("DhGroup", 7u32, "DHGroup24"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The Pfs Groups used in IKE Phase 2 for new child SA."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PfsGroup")]
    pub enum PfsGroup {
        None,
        #[serde(rename = "PFS1")]
        Pfs1,
        #[serde(rename = "PFS2")]
        Pfs2,
        #[serde(rename = "PFS2048")]
        Pfs2048,
        #[serde(rename = "ECP256")]
        Ecp256,
        #[serde(rename = "ECP384")]
        Ecp384,
        #[serde(rename = "PFS24")]
        Pfs24,
        #[serde(rename = "PFS14")]
        Pfs14,
        #[serde(rename = "PFSMM")]
        Pfsmm,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PfsGroup {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PfsGroup {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PfsGroup {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("PfsGroup", 0u32, "None"),
                Self::Pfs1 => serializer.serialize_unit_variant("PfsGroup", 1u32, "PFS1"),
                Self::Pfs2 => serializer.serialize_unit_variant("PfsGroup", 2u32, "PFS2"),
                Self::Pfs2048 => serializer.serialize_unit_variant("PfsGroup", 3u32, "PFS2048"),
                Self::Ecp256 => serializer.serialize_unit_variant("PfsGroup", 4u32, "ECP256"),
                Self::Ecp384 => serializer.serialize_unit_variant("PfsGroup", 5u32, "ECP384"),
                Self::Pfs24 => serializer.serialize_unit_variant("PfsGroup", 6u32, "PFS24"),
                Self::Pfs14 => serializer.serialize_unit_variant("PfsGroup", 7u32, "PFS14"),
                Self::Pfsmm => serializer.serialize_unit_variant("PfsGroup", 8u32, "PFSMM"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "LoadBalancer resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancer {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "SKU of a load balancer"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<LoadBalancerSku>,
    #[doc = "Properties of the load balancer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LoadBalancerPropertiesFormat>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl LoadBalancer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for ListBackendAddressPool API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancerBackendAddressPoolListResult {
    #[doc = "A list of backend address pools in a load balancer."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BackendAddressPool>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LoadBalancerBackendAddressPoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LoadBalancerBackendAddressPoolListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for ListFrontendIPConfiguration API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancerFrontendIpConfigurationListResult {
    #[doc = "A list of frontend IP configurations in a load balancer."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FrontendIpConfiguration>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LoadBalancerFrontendIpConfigurationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LoadBalancerFrontendIpConfigurationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for ListLoadBalancers API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancerListResult {
    #[doc = "A list of load balancers in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<LoadBalancer>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LoadBalancerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LoadBalancerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for ListLoadBalancingRule API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancerLoadBalancingRuleListResult {
    #[doc = "A list of load balancing rules in a load balancer."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<LoadBalancingRule>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LoadBalancerLoadBalancingRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LoadBalancerLoadBalancingRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for ListOutboundRule API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancerOutboundRuleListResult {
    #[doc = "A list of outbound rules in a load balancer."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OutboundRule>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LoadBalancerOutboundRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LoadBalancerOutboundRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for ListProbe API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancerProbeListResult {
    #[doc = "A list of probes in a load balancer."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Probe>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LoadBalancerProbeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LoadBalancerProbeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the load balancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancerPropertiesFormat {
    #[doc = "Object representing the frontend IPs to be used for the load balancer"]
    #[serde(
        rename = "frontendIPConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub frontend_ip_configurations: Vec<FrontendIpConfiguration>,
    #[doc = "Collection of backend address pools used by a load balancer"]
    #[serde(
        rename = "backendAddressPools",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub backend_address_pools: Vec<BackendAddressPool>,
    #[doc = "Object collection representing the load balancing rules Gets the provisioning "]
    #[serde(
        rename = "loadBalancingRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub load_balancing_rules: Vec<LoadBalancingRule>,
    #[doc = "Collection of probe objects used in the load balancer"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub probes: Vec<Probe>,
    #[doc = "Collection of inbound NAT Rules used by a load balancer. Defining inbound NAT rules on your load balancer is mutually exclusive with defining an inbound NAT pool. Inbound NAT pools are referenced from virtual machine scale sets. NICs that are associated with individual virtual machines cannot reference an Inbound NAT pool. They have to reference individual inbound NAT rules."]
    #[serde(
        rename = "inboundNatRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inbound_nat_rules: Vec<InboundNatRule>,
    #[doc = "Defines an external port range for inbound NAT to a single backend port on NICs associated with a load balancer. Inbound NAT rules are created automatically for each NIC associated with the Load Balancer using an external port from this range. Defining an Inbound NAT pool on your Load Balancer is mutually exclusive with defining inbound Nat rules. Inbound NAT pools are referenced from virtual machine scale sets. NICs that are associated with individual virtual machines cannot reference an inbound NAT pool. They have to reference individual inbound NAT rules."]
    #[serde(
        rename = "inboundNatPools",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inbound_nat_pools: Vec<InboundNatPool>,
    #[doc = "The outbound rules."]
    #[serde(
        rename = "outboundRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub outbound_rules: Vec<OutboundRule>,
    #[doc = "The resource GUID property of the load balancer resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[doc = "Gets the provisioning state of the PublicIP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl LoadBalancerPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU of a load balancer"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancerSku {
    #[doc = "Name of a load balancer SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<load_balancer_sku::Name>,
}
impl LoadBalancerSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod load_balancer_sku {
    use super::*;
    #[doc = "Name of a load balancer SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Basic,
        Standard,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("Name", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Name", 1u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A load balancing rule for a load balancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancingRule {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of the load balancer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LoadBalancingRulePropertiesFormat>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl LoadBalancingRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the load balancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancingRulePropertiesFormat {
    #[doc = "Reference to another subresource."]
    #[serde(rename = "frontendIPConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub frontend_ip_configuration: Option<SubResource>,
    #[doc = "Reference to another subresource."]
    #[serde(rename = "backendAddressPool", default, skip_serializing_if = "Option::is_none")]
    pub backend_address_pool: Option<SubResource>,
    #[doc = "Reference to another subresource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probe: Option<SubResource>,
    #[doc = "The transport protocol for the endpoint. Possible values are 'Udp' or 'Tcp' or 'All'."]
    pub protocol: TransportProtocol,
    #[doc = "The load distribution policy for this rule. Possible values are 'Default', 'SourceIP', and 'SourceIPProtocol'."]
    #[serde(rename = "loadDistribution", default, skip_serializing_if = "Option::is_none")]
    pub load_distribution: Option<load_balancing_rule_properties_format::LoadDistribution>,
    #[doc = "The port for the external endpoint. Port numbers for each rule must be unique within the Load Balancer. Acceptable values are between 0 and 65534. Note that value 0 enables \"Any Port\""]
    #[serde(rename = "frontendPort")]
    pub frontend_port: i32,
    #[doc = "The port used for internal connections on the endpoint. Acceptable values are between 0 and 65535. Note that value 0 enables \"Any Port\""]
    #[serde(rename = "backendPort", default, skip_serializing_if = "Option::is_none")]
    pub backend_port: Option<i32>,
    #[doc = "The timeout for the TCP idle connection. The value can be set between 4 and 30 minutes. The default value is 4 minutes. This element is only used when the protocol is set to TCP."]
    #[serde(rename = "idleTimeoutInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_in_minutes: Option<i32>,
    #[doc = "Configures a virtual machine's endpoint for the floating IP capability required to configure a SQL AlwaysOn Availability Group. This setting is required when using the SQL AlwaysOn Availability Groups in SQL server. This setting can't be changed after you create the endpoint."]
    #[serde(rename = "enableFloatingIP", default, skip_serializing_if = "Option::is_none")]
    pub enable_floating_ip: Option<bool>,
    #[doc = "Receive bidirectional TCP Reset on TCP flow idle timeout or unexpected connection termination. This element is only used when the protocol is set to TCP."]
    #[serde(rename = "enableTcpReset", default, skip_serializing_if = "Option::is_none")]
    pub enable_tcp_reset: Option<bool>,
    #[doc = "Configures SNAT for the VMs in the backend pool to use the publicIP address specified in the frontend of the load balancing rule."]
    #[serde(rename = "disableOutboundSnat", default, skip_serializing_if = "Option::is_none")]
    pub disable_outbound_snat: Option<bool>,
    #[doc = "Gets the provisioning state of the PublicIP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl LoadBalancingRulePropertiesFormat {
    pub fn new(protocol: TransportProtocol, frontend_port: i32) -> Self {
        Self {
            frontend_ip_configuration: None,
            backend_address_pool: None,
            probe: None,
            protocol,
            load_distribution: None,
            frontend_port,
            backend_port: None,
            idle_timeout_in_minutes: None,
            enable_floating_ip: None,
            enable_tcp_reset: None,
            disable_outbound_snat: None,
            provisioning_state: None,
        }
    }
}
pub mod load_balancing_rule_properties_format {
    use super::*;
    #[doc = "The load distribution policy for this rule. Possible values are 'Default', 'SourceIP', and 'SourceIPProtocol'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LoadDistribution")]
    pub enum LoadDistribution {
        Default,
        #[serde(rename = "SourceIP")]
        SourceIp,
        #[serde(rename = "SourceIPProtocol")]
        SourceIpProtocol,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LoadDistribution {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LoadDistribution {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LoadDistribution {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("LoadDistribution", 0u32, "Default"),
                Self::SourceIp => serializer.serialize_unit_variant("LoadDistribution", 1u32, "SourceIP"),
                Self::SourceIpProtocol => serializer.serialize_unit_variant("LoadDistribution", 2u32, "SourceIPProtocol"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A common class for general resource information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalNetworkGateway {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "LocalNetworkGateway properties"]
    pub properties: LocalNetworkGatewayPropertiesFormat,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl LocalNetworkGateway {
    pub fn new(properties: LocalNetworkGatewayPropertiesFormat) -> Self {
        Self {
            resource: Resource::default(),
            properties,
            etag: None,
        }
    }
}
#[doc = "Response for ListLocalNetworkGateways API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalNetworkGatewayListResult {
    #[doc = "A list of local network gateways that exists in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<LocalNetworkGateway>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LocalNetworkGatewayListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LocalNetworkGatewayListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "LocalNetworkGateway properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalNetworkGatewayPropertiesFormat {
    #[doc = "AddressSpace contains an array of IP address ranges that can be used by subnets of the virtual network."]
    #[serde(rename = "localNetworkAddressSpace", default, skip_serializing_if = "Option::is_none")]
    pub local_network_address_space: Option<AddressSpace>,
    #[doc = "IP address of local network gateway."]
    #[serde(rename = "gatewayIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub gateway_ip_address: Option<String>,
    #[doc = "BGP settings details"]
    #[serde(rename = "bgpSettings", default, skip_serializing_if = "Option::is_none")]
    pub bgp_settings: Option<BgpSettings>,
    #[doc = "The resource GUID property of the LocalNetworkGateway resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[doc = "The provisioning state of the LocalNetworkGateway resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl LocalNetworkGatewayPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of logging specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[doc = "The name of the specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The display name of the specification."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Duration of the blob."]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedServiceIdentity {
    #[doc = "The principal id of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of identity used for the resource. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the virtual machine."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<managed_service_identity::Type>,
    #[doc = "The list of user identities associated with resource. The user identity dictionary key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl ManagedServiceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_service_identity {
    use super::*;
    #[doc = "The type of identity used for the resource. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the virtual machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[doc = "Description of metrics specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[doc = "The name of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The display name of the metric."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the metric."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "Units the metric to be displayed in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The aggregation type."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "List of availability."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub availabilities: Vec<Availability>,
    #[doc = "Whether regional MDM account enabled."]
    #[serde(rename = "enableRegionalMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub enable_regional_mdm_account: Option<bool>,
    #[doc = "Whether gaps would be filled with zeros."]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[doc = "Pattern for the filter of the metric."]
    #[serde(rename = "metricFilterPattern", default, skip_serializing_if = "Option::is_none")]
    pub metric_filter_pattern: Option<String>,
    #[doc = "List of dimensions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<Dimension>,
    #[doc = "Whether the metric is internal."]
    #[serde(rename = "isInternal", default, skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
    #[doc = "The source MDM account."]
    #[serde(rename = "sourceMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_account: Option<String>,
    #[doc = "The source MDM namespace."]
    #[serde(rename = "sourceMdmNamespace", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_namespace: Option<String>,
    #[doc = "The resource Id dimension name override."]
    #[serde(rename = "resourceIdDimensionNameOverride", default, skip_serializing_if = "Option::is_none")]
    pub resource_id_dimension_name_override: Option<String>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A network interface in a resource group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterface {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "NetworkInterface properties. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkInterfacePropertiesFormat>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl NetworkInterface {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DNS settings of a network interface."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceDnsSettings {
    #[doc = "List of DNS servers IP addresses. Use 'AzureProvidedDNS' to switch to azure provided DNS resolution. 'AzureProvidedDNS' value cannot be combined with other IPs, it must be the only value in dnsServers collection."]
    #[serde(
        rename = "dnsServers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dns_servers: Vec<String>,
    #[doc = "If the VM that uses this NIC is part of an Availability Set, then this list will have the union of all DNS servers from all NICs that are part of the Availability Set. This property is what is configured on each of those VMs."]
    #[serde(
        rename = "appliedDnsServers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub applied_dns_servers: Vec<String>,
    #[doc = "Relative DNS name for this NIC used for internal communications between VMs in the same virtual network."]
    #[serde(rename = "internalDnsNameLabel", default, skip_serializing_if = "Option::is_none")]
    pub internal_dns_name_label: Option<String>,
    #[doc = "Fully qualified DNS name supporting internal communications between VMs in the same virtual network."]
    #[serde(rename = "internalFqdn", default, skip_serializing_if = "Option::is_none")]
    pub internal_fqdn: Option<String>,
    #[doc = "Even if internalDnsNameLabel is not specified, a DNS entry is created for the primary NIC of the VM. This DNS name can be constructed by concatenating the VM name with the value of internalDomainNameSuffix."]
    #[serde(rename = "internalDomainNameSuffix", default, skip_serializing_if = "Option::is_none")]
    pub internal_domain_name_suffix: Option<String>,
}
impl NetworkInterfaceDnsSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IPConfiguration in a network interface."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceIpConfiguration {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of IP configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkInterfaceIpConfigurationPropertiesFormat>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl NetworkInterfaceIpConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for list ip configurations API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceIpConfigurationListResult {
    #[doc = "A list of ip configurations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkInterfaceIpConfiguration>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkInterfaceIpConfigurationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NetworkInterfaceIpConfigurationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of IP configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceIpConfigurationPropertiesFormat {
    #[doc = "The reference to Virtual Network Taps."]
    #[serde(
        rename = "virtualNetworkTaps",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub virtual_network_taps: Vec<VirtualNetworkTap>,
    #[doc = "The reference of ApplicationGatewayBackendAddressPool resource."]
    #[serde(
        rename = "applicationGatewayBackendAddressPools",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub application_gateway_backend_address_pools: Vec<ApplicationGatewayBackendAddressPool>,
    #[doc = "The reference of LoadBalancerBackendAddressPool resource."]
    #[serde(
        rename = "loadBalancerBackendAddressPools",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub load_balancer_backend_address_pools: Vec<BackendAddressPool>,
    #[doc = "A list of references of LoadBalancerInboundNatRules."]
    #[serde(
        rename = "loadBalancerInboundNatRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub load_balancer_inbound_nat_rules: Vec<InboundNatRule>,
    #[doc = "Private IP address of the IP configuration."]
    #[serde(rename = "privateIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "Defines how a private IP address is assigned. Possible values are: 'Static' and 'Dynamic'."]
    #[serde(rename = "privateIPAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_allocation_method: Option<network_interface_ip_configuration_properties_format::PrivateIpAllocationMethod>,
    #[doc = "Available from Api-Version 2016-03-30 onwards, it represents whether the specific ipconfiguration is IPv4 or IPv6. Default is taken as IPv4.  Possible values are: 'IPv4' and 'IPv6'."]
    #[serde(rename = "privateIPAddressVersion", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address_version: Option<network_interface_ip_configuration_properties_format::PrivateIpAddressVersion>,
    #[doc = "Subnet in a virtual network resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<Subnet>,
    #[doc = "Gets whether this is a primary customer address on the network interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[doc = "Public IP address resource."]
    #[serde(rename = "publicIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<PublicIpAddress>,
    #[doc = "Application security groups in which the IP configuration is included."]
    #[serde(
        rename = "applicationSecurityGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub application_security_groups: Vec<ApplicationSecurityGroup>,
    #[doc = "The provisioning state of the network interface IP configuration. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl NetworkInterfaceIpConfigurationPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_interface_ip_configuration_properties_format {
    use super::*;
    #[doc = "Defines how a private IP address is assigned. Possible values are: 'Static' and 'Dynamic'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrivateIpAllocationMethod")]
    pub enum PrivateIpAllocationMethod {
        Static,
        Dynamic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrivateIpAllocationMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrivateIpAllocationMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrivateIpAllocationMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Static => serializer.serialize_unit_variant("PrivateIpAllocationMethod", 0u32, "Static"),
                Self::Dynamic => serializer.serialize_unit_variant("PrivateIpAllocationMethod", 1u32, "Dynamic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Available from Api-Version 2016-03-30 onwards, it represents whether the specific ipconfiguration is IPv4 or IPv6. Default is taken as IPv4.  Possible values are: 'IPv4' and 'IPv6'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrivateIpAddressVersion")]
    pub enum PrivateIpAddressVersion {
        IPv4,
        IPv6,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrivateIpAddressVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrivateIpAddressVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrivateIpAddressVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IPv4 => serializer.serialize_unit_variant("PrivateIpAddressVersion", 0u32, "IPv4"),
                Self::IPv6 => serializer.serialize_unit_variant("PrivateIpAddressVersion", 1u32, "IPv6"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Response for the ListNetworkInterface API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceListResult {
    #[doc = "A list of network interfaces in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkInterface>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkInterfaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NetworkInterfaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for list ip configurations API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceLoadBalancerListResult {
    #[doc = "A list of load balancers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<LoadBalancer>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkInterfaceLoadBalancerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NetworkInterfaceLoadBalancerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NetworkInterface properties. "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfacePropertiesFormat {
    #[doc = "Reference to another subresource."]
    #[serde(rename = "virtualMachine", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine: Option<SubResource>,
    #[doc = "NetworkSecurityGroup resource."]
    #[serde(rename = "networkSecurityGroup", default, skip_serializing_if = "Option::is_none")]
    pub network_security_group: Option<NetworkSecurityGroup>,
    #[doc = "Interface endpoint resource."]
    #[serde(rename = "interfaceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub interface_endpoint: Option<InterfaceEndpoint>,
    #[doc = "A list of IPConfigurations of the network interface."]
    #[serde(
        rename = "ipConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_configurations: Vec<NetworkInterfaceIpConfiguration>,
    #[doc = "A list of TapConfigurations of the network interface."]
    #[serde(
        rename = "tapConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tap_configurations: Vec<NetworkInterfaceTapConfiguration>,
    #[doc = "DNS settings of a network interface."]
    #[serde(rename = "dnsSettings", default, skip_serializing_if = "Option::is_none")]
    pub dns_settings: Option<NetworkInterfaceDnsSettings>,
    #[doc = "The MAC address of the network interface."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "Gets whether this is a primary network interface on a virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[doc = "If the network interface is accelerated networking enabled."]
    #[serde(rename = "enableAcceleratedNetworking", default, skip_serializing_if = "Option::is_none")]
    pub enable_accelerated_networking: Option<bool>,
    #[doc = "Indicates whether IP forwarding is enabled on this network interface."]
    #[serde(rename = "enableIPForwarding", default, skip_serializing_if = "Option::is_none")]
    pub enable_ip_forwarding: Option<bool>,
    #[doc = "A list of references to linked BareMetal resources"]
    #[serde(
        rename = "hostedWorkloads",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hosted_workloads: Vec<String>,
    #[doc = "The resource GUID property of the network interface resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[doc = "The provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl NetworkInterfacePropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tap configuration in a Network Interface"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceTapConfiguration {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of Virtual Network Tap configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkInterfaceTapConfigurationPropertiesFormat>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Sub Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl NetworkInterfaceTapConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for list tap configurations API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceTapConfigurationListResult {
    #[doc = "A list of tap configurations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkInterfaceTapConfiguration>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkInterfaceTapConfigurationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NetworkInterfaceTapConfigurationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Virtual Network Tap configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceTapConfigurationPropertiesFormat {
    #[doc = "Virtual Network Tap resource"]
    #[serde(rename = "virtualNetworkTap", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_tap: Option<VirtualNetworkTap>,
    #[doc = "The provisioning state of the network interface tap configuration. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl NetworkInterfaceTapConfigurationPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NetworkSecurityGroup resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkSecurityGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Network Security Group resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkSecurityGroupPropertiesFormat>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl NetworkSecurityGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for ListNetworkSecurityGroups API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkSecurityGroupListResult {
    #[doc = "A list of NetworkSecurityGroup resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkSecurityGroup>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkSecurityGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NetworkSecurityGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network Security Group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkSecurityGroupPropertiesFormat {
    #[doc = "A collection of security rules of the network security group."]
    #[serde(
        rename = "securityRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub security_rules: Vec<SecurityRule>,
    #[doc = "The default security rules of network security group."]
    #[serde(
        rename = "defaultSecurityRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub default_security_rules: Vec<SecurityRule>,
    #[doc = "A collection of references to network interfaces."]
    #[serde(
        rename = "networkInterfaces",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_interfaces: Vec<NetworkInterface>,
    #[doc = "A collection of references to subnets."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subnets: Vec<Subnet>,
    #[doc = "The resource GUID property of the network security group resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[doc = "The provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl NetworkSecurityGroupPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network REST API operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "Origin of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Description of operation properties format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationPropertiesFormat>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft Network."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of the operation: get, read, delete, etc."]
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
#[doc = "Result of the request to list Network operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Network operations supported by the Network resource provider."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
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
#[doc = "Description of operation properties format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationPropertiesFormat {
    #[doc = "Specification of the service."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<operation_properties_format::ServiceSpecification>,
}
impl OperationPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_properties_format {
    use super::*;
    #[doc = "Specification of the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ServiceSpecification {
        #[doc = "Operation service specification."]
        #[serde(
            rename = "metricSpecifications",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub metric_specifications: Vec<MetricSpecification>,
        #[doc = "Operation log specification."]
        #[serde(
            rename = "logSpecifications",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub log_specifications: Vec<LogSpecification>,
    }
    impl ServiceSpecification {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Outbound rule of the load balancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundRule {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Outbound rule of the load balancer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OutboundRulePropertiesFormat>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl OutboundRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Outbound rule of the load balancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutboundRulePropertiesFormat {
    #[doc = "The number of outbound ports to be used for NAT."]
    #[serde(rename = "allocatedOutboundPorts", default, skip_serializing_if = "Option::is_none")]
    pub allocated_outbound_ports: Option<i32>,
    #[doc = "The Frontend IP addresses of the load balancer."]
    #[serde(rename = "frontendIPConfigurations")]
    pub frontend_ip_configurations: Vec<SubResource>,
    #[doc = "Reference to another subresource."]
    #[serde(rename = "backendAddressPool")]
    pub backend_address_pool: SubResource,
    #[doc = "Gets the provisioning state of the PublicIP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Protocol - TCP, UDP or All"]
    pub protocol: outbound_rule_properties_format::Protocol,
    #[doc = "Receive bidirectional TCP Reset on TCP flow idle timeout or unexpected connection termination. This element is only used when the protocol is set to TCP."]
    #[serde(rename = "enableTcpReset", default, skip_serializing_if = "Option::is_none")]
    pub enable_tcp_reset: Option<bool>,
    #[doc = "The timeout for the TCP idle connection"]
    #[serde(rename = "idleTimeoutInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_in_minutes: Option<i64>,
}
impl OutboundRulePropertiesFormat {
    pub fn new(
        frontend_ip_configurations: Vec<SubResource>,
        backend_address_pool: SubResource,
        protocol: outbound_rule_properties_format::Protocol,
    ) -> Self {
        Self {
            allocated_outbound_ports: None,
            frontend_ip_configurations,
            backend_address_pool,
            provisioning_state: None,
            protocol,
            enable_tcp_reset: None,
            idle_timeout_in_minutes: None,
        }
    }
}
pub mod outbound_rule_properties_format {
    use super::*;
    #[doc = "Protocol - TCP, UDP or All"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Protocol {
        Tcp,
        Udp,
        All,
    }
}
#[doc = "A load balancer probe."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Probe {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Load balancer probe resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProbePropertiesFormat>,
    #[doc = "Gets name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Probe {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Load balancer probe resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProbePropertiesFormat {
    #[doc = "The load balancer rules that use this probe."]
    #[serde(
        rename = "loadBalancingRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub load_balancing_rules: Vec<SubResource>,
    #[doc = "The protocol of the end point. Possible values are: 'Http', 'Tcp', or 'Https'. If 'Tcp' is specified, a received ACK is required for the probe to be successful. If 'Http' or 'Https' is specified, a 200 OK response from the specifies URI is required for the probe to be successful."]
    pub protocol: probe_properties_format::Protocol,
    #[doc = "The port for communicating the probe. Possible values range from 1 to 65535, inclusive."]
    pub port: i32,
    #[doc = "The interval, in seconds, for how frequently to probe the endpoint for health status. Typically, the interval is slightly less than half the allocated timeout period (in seconds) which allows two full probes before taking the instance out of rotation. The default value is 15, the minimum value is 5."]
    #[serde(rename = "intervalInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i32>,
    #[doc = "The number of probes where if no response, will result in stopping further traffic from being delivered to the endpoint. This values allows endpoints to be taken out of rotation faster or slower than the typical times used in Azure."]
    #[serde(rename = "numberOfProbes", default, skip_serializing_if = "Option::is_none")]
    pub number_of_probes: Option<i32>,
    #[doc = "The URI used for requesting health status from the VM. Path is required if a protocol is set to http. Otherwise, it is not allowed. There is no default value."]
    #[serde(rename = "requestPath", default, skip_serializing_if = "Option::is_none")]
    pub request_path: Option<String>,
    #[doc = "Gets the provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ProbePropertiesFormat {
    pub fn new(protocol: probe_properties_format::Protocol, port: i32) -> Self {
        Self {
            load_balancing_rules: Vec::new(),
            protocol,
            port,
            interval_in_seconds: None,
            number_of_probes: None,
            request_path: None,
            provisioning_state: None,
        }
    }
}
pub mod probe_properties_format {
    use super::*;
    #[doc = "The protocol of the end point. Possible values are: 'Http', 'Tcp', or 'Https'. If 'Tcp' is specified, a received ACK is required for the probe to be successful. If 'Http' or 'Https' is specified, a 200 OK response from the specifies URI is required for the probe to be successful."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        Http,
        Tcp,
        Https,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Protocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Protocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Protocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Http => serializer.serialize_unit_variant("Protocol", 0u32, "Http"),
                Self::Tcp => serializer.serialize_unit_variant("Protocol", 1u32, "Tcp"),
                Self::Https => serializer.serialize_unit_variant("Protocol", 2u32, "Https"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Public IP address resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicIpAddress {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "SKU of a public IP address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PublicIpAddressSku>,
    #[doc = "Public IP address properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PublicIpAddressPropertiesFormat>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "A list of availability zones denoting the IP allocated for the resource needs to come from."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub zones: Vec<String>,
}
impl PublicIpAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains FQDN of the DNS record associated with the public IP address"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicIpAddressDnsSettings {
    #[doc = "Gets or sets the Domain name label.The concatenation of the domain name label and the regionalized DNS zone make up the fully qualified domain name associated with the public IP address. If a domain name label is specified, an A DNS record is created for the public IP in the Microsoft Azure DNS system."]
    #[serde(rename = "domainNameLabel", default, skip_serializing_if = "Option::is_none")]
    pub domain_name_label: Option<String>,
    #[doc = "Gets the FQDN, Fully qualified domain name of the A DNS record associated with the public IP. This is the concatenation of the domainNameLabel and the regionalized DNS zone."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Gets or Sets the Reverse FQDN. A user-visible, fully qualified domain name that resolves to this public IP address. If the reverseFqdn is specified, then a PTR DNS record is created pointing from the IP address in the in-addr.arpa domain to the reverse FQDN. "]
    #[serde(rename = "reverseFqdn", default, skip_serializing_if = "Option::is_none")]
    pub reverse_fqdn: Option<String>,
}
impl PublicIpAddressDnsSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for ListPublicIpAddresses API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicIpAddressListResult {
    #[doc = "A list of public IP addresses that exists in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PublicIpAddress>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PublicIpAddressListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PublicIpAddressListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Public IP address properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicIpAddressPropertiesFormat {
    #[doc = "The public IP allocation method. Possible values are: 'Static' and 'Dynamic'."]
    #[serde(rename = "publicIPAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_allocation_method: Option<public_ip_address_properties_format::PublicIpAllocationMethod>,
    #[doc = "The public IP address version. Possible values are: 'IPv4' and 'IPv6'."]
    #[serde(rename = "publicIPAddressVersion", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address_version: Option<public_ip_address_properties_format::PublicIpAddressVersion>,
    #[doc = "IP configuration"]
    #[serde(rename = "ipConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub ip_configuration: Option<IpConfiguration>,
    #[doc = "Contains FQDN of the DNS record associated with the public IP address"]
    #[serde(rename = "dnsSettings", default, skip_serializing_if = "Option::is_none")]
    pub dns_settings: Option<PublicIpAddressDnsSettings>,
    #[doc = "Contains the DDoS protection settings of the public IP."]
    #[serde(rename = "ddosSettings", default, skip_serializing_if = "Option::is_none")]
    pub ddos_settings: Option<DdosSettings>,
    #[doc = "The list of tags associated with the public IP address."]
    #[serde(
        rename = "ipTags",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_tags: Vec<IpTag>,
    #[doc = "The IP address associated with the public IP address resource."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Reference to another subresource."]
    #[serde(rename = "publicIPPrefix", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_prefix: Option<SubResource>,
    #[doc = "The idle timeout of the public IP address."]
    #[serde(rename = "idleTimeoutInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_in_minutes: Option<i32>,
    #[doc = "The resource GUID property of the public IP resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[doc = "The provisioning state of the PublicIP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl PublicIpAddressPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod public_ip_address_properties_format {
    use super::*;
    #[doc = "The public IP allocation method. Possible values are: 'Static' and 'Dynamic'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicIpAllocationMethod")]
    pub enum PublicIpAllocationMethod {
        Static,
        Dynamic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicIpAllocationMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicIpAllocationMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicIpAllocationMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Static => serializer.serialize_unit_variant("PublicIpAllocationMethod", 0u32, "Static"),
                Self::Dynamic => serializer.serialize_unit_variant("PublicIpAllocationMethod", 1u32, "Dynamic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The public IP address version. Possible values are: 'IPv4' and 'IPv6'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicIpAddressVersion")]
    pub enum PublicIpAddressVersion {
        IPv4,
        IPv6,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicIpAddressVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicIpAddressVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicIpAddressVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IPv4 => serializer.serialize_unit_variant("PublicIpAddressVersion", 0u32, "IPv4"),
                Self::IPv6 => serializer.serialize_unit_variant("PublicIpAddressVersion", 1u32, "IPv6"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "SKU of a public IP address"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicIpAddressSku {
    #[doc = "Name of a public IP address SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<public_ip_address_sku::Name>,
}
impl PublicIpAddressSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod public_ip_address_sku {
    use super::*;
    #[doc = "Name of a public IP address SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Basic,
        Standard,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("Name", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Name", 1u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Common resource representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ResourceNavigationLink resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceNavigationLink {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of ResourceNavigationLink."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceNavigationLinkFormat>,
    #[doc = "Name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ResourceNavigationLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of ResourceNavigationLink."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceNavigationLinkFormat {
    #[doc = "Resource type of the linked resource."]
    #[serde(rename = "linkedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub linked_resource_type: Option<String>,
    #[doc = "Link to the external resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[doc = "Provisioning state of the ResourceNavigationLink resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ResourceNavigationLinkFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Route resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Route {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Route resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoutePropertiesFormat>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Route {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for the ListRoute API service call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RouteListResult {
    #[doc = "Gets a list of routes in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Route>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RouteListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RouteListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Route resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutePropertiesFormat {
    #[doc = "The destination CIDR to which the route applies."]
    #[serde(rename = "addressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub address_prefix: Option<String>,
    #[doc = "The type of Azure hop the packet should be sent to. Possible values are: 'VirtualNetworkGateway', 'VnetLocal', 'Internet', 'VirtualAppliance', and 'None'"]
    #[serde(rename = "nextHopType")]
    pub next_hop_type: route_properties_format::NextHopType,
    #[doc = "The IP address packets should be forwarded to. Next hop values are only allowed in routes where the next hop type is VirtualAppliance."]
    #[serde(rename = "nextHopIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub next_hop_ip_address: Option<String>,
    #[doc = "The provisioning state of the resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl RoutePropertiesFormat {
    pub fn new(next_hop_type: route_properties_format::NextHopType) -> Self {
        Self {
            address_prefix: None,
            next_hop_type,
            next_hop_ip_address: None,
            provisioning_state: None,
        }
    }
}
pub mod route_properties_format {
    use super::*;
    #[doc = "The type of Azure hop the packet should be sent to. Possible values are: 'VirtualNetworkGateway', 'VnetLocal', 'Internet', 'VirtualAppliance', and 'None'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NextHopType")]
    pub enum NextHopType {
        VirtualNetworkGateway,
        VnetLocal,
        Internet,
        VirtualAppliance,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NextHopType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NextHopType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NextHopType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::VirtualNetworkGateway => serializer.serialize_unit_variant("NextHopType", 0u32, "VirtualNetworkGateway"),
                Self::VnetLocal => serializer.serialize_unit_variant("NextHopType", 1u32, "VnetLocal"),
                Self::Internet => serializer.serialize_unit_variant("NextHopType", 2u32, "Internet"),
                Self::VirtualAppliance => serializer.serialize_unit_variant("NextHopType", 3u32, "VirtualAppliance"),
                Self::None => serializer.serialize_unit_variant("NextHopType", 4u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Route table resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RouteTable {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Route Table resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RouteTablePropertiesFormat>,
    #[doc = "Gets a unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl RouteTable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for the ListRouteTable API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RouteTableListResult {
    #[doc = "Gets a list of route tables in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RouteTable>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RouteTableListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RouteTableListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Route Table resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RouteTablePropertiesFormat {
    #[doc = "Collection of routes contained within a route table."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub routes: Vec<Route>,
    #[doc = "A collection of references to subnets."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subnets: Vec<Subnet>,
    #[doc = "Gets or sets whether to disable the routes learned by BGP on that route table. True means disable."]
    #[serde(rename = "disableBgpRoutePropagation", default, skip_serializing_if = "Option::is_none")]
    pub disable_bgp_route_propagation: Option<bool>,
    #[doc = "The provisioning state of the resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl RouteTablePropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network security rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityRule {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Security rule resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityRulePropertiesFormat>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl SecurityRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for ListSecurityRule API service call. Retrieves all security rules that belongs to a network security group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityRuleListResult {
    #[doc = "The security rules in a network security group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SecurityRule>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecurityRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SecurityRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Security rule resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityRulePropertiesFormat {
    #[doc = "A description for this rule. Restricted to 140 chars."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Network protocol this rule applies to. Possible values are 'Tcp', 'Udp', and '*'."]
    pub protocol: security_rule_properties_format::Protocol,
    #[doc = "The source port or range. Integer or range between 0 and 65535. Asterisks '*' can also be used to match all ports."]
    #[serde(rename = "sourcePortRange", default, skip_serializing_if = "Option::is_none")]
    pub source_port_range: Option<String>,
    #[doc = "The destination port or range. Integer or range between 0 and 65535. Asterisks '*' can also be used to match all ports."]
    #[serde(rename = "destinationPortRange", default, skip_serializing_if = "Option::is_none")]
    pub destination_port_range: Option<String>,
    #[doc = "The CIDR or source IP range. Asterisks '*' can also be used to match all source IPs. Default tags such as 'VirtualNetwork', 'AzureLoadBalancer' and 'Internet' can also be used. If this is an ingress rule, specifies where network traffic originates from. "]
    #[serde(rename = "sourceAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub source_address_prefix: Option<String>,
    #[doc = "The CIDR or source IP ranges."]
    #[serde(
        rename = "sourceAddressPrefixes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_address_prefixes: Vec<String>,
    #[doc = "The application security group specified as source."]
    #[serde(
        rename = "sourceApplicationSecurityGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_application_security_groups: Vec<ApplicationSecurityGroup>,
    #[doc = "The destination address prefix. CIDR or destination IP range. Asterisks '*' can also be used to match all source IPs. Default tags such as 'VirtualNetwork', 'AzureLoadBalancer' and 'Internet' can also be used."]
    #[serde(rename = "destinationAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub destination_address_prefix: Option<String>,
    #[doc = "The destination address prefixes. CIDR or destination IP ranges."]
    #[serde(
        rename = "destinationAddressPrefixes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub destination_address_prefixes: Vec<String>,
    #[doc = "The application security group specified as destination."]
    #[serde(
        rename = "destinationApplicationSecurityGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub destination_application_security_groups: Vec<ApplicationSecurityGroup>,
    #[doc = "The source port ranges."]
    #[serde(
        rename = "sourcePortRanges",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_port_ranges: Vec<String>,
    #[doc = "The destination port ranges."]
    #[serde(
        rename = "destinationPortRanges",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub destination_port_ranges: Vec<String>,
    #[doc = "The network traffic is allowed or denied. Possible values are: 'Allow' and 'Deny'."]
    pub access: security_rule_properties_format::Access,
    #[doc = "The priority of the rule. The value can be between 100 and 4096. The priority number must be unique for each rule in the collection. The lower the priority number, the higher the priority of the rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "The direction of the rule. The direction specifies if rule will be evaluated on incoming or outgoing traffic. Possible values are: 'Inbound' and 'Outbound'."]
    pub direction: security_rule_properties_format::Direction,
    #[doc = "The provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl SecurityRulePropertiesFormat {
    pub fn new(
        protocol: security_rule_properties_format::Protocol,
        access: security_rule_properties_format::Access,
        direction: security_rule_properties_format::Direction,
    ) -> Self {
        Self {
            description: None,
            protocol,
            source_port_range: None,
            destination_port_range: None,
            source_address_prefix: None,
            source_address_prefixes: Vec::new(),
            source_application_security_groups: Vec::new(),
            destination_address_prefix: None,
            destination_address_prefixes: Vec::new(),
            destination_application_security_groups: Vec::new(),
            source_port_ranges: Vec::new(),
            destination_port_ranges: Vec::new(),
            access,
            priority: None,
            direction,
            provisioning_state: None,
        }
    }
}
pub mod security_rule_properties_format {
    use super::*;
    #[doc = "Network protocol this rule applies to. Possible values are 'Tcp', 'Udp', and '*'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        Tcp,
        Udp,
        #[serde(rename = "*")]
        U2a,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Protocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Protocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Protocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tcp => serializer.serialize_unit_variant("Protocol", 0u32, "Tcp"),
                Self::Udp => serializer.serialize_unit_variant("Protocol", 1u32, "Udp"),
                Self::U2a => serializer.serialize_unit_variant("Protocol", 2u32, "*"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The network traffic is allowed or denied. Possible values are: 'Allow' and 'Deny'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Access")]
    pub enum Access {
        Allow,
        Deny,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Access {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Access {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Access {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("Access", 0u32, "Allow"),
                Self::Deny => serializer.serialize_unit_variant("Access", 1u32, "Deny"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The direction of the rule. The direction specifies if rule will be evaluated on incoming or outgoing traffic. Possible values are: 'Inbound' and 'Outbound'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Direction")]
    pub enum Direction {
        Inbound,
        Outbound,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Direction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Direction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Direction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inbound => serializer.serialize_unit_variant("Direction", 0u32, "Inbound"),
                Self::Outbound => serializer.serialize_unit_variant("Direction", 1u32, "Outbound"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "ServiceAssociationLink resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceAssociationLink {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of ServiceAssociationLink."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceAssociationLinkPropertiesFormat>,
    #[doc = "Name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ServiceAssociationLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of ServiceAssociationLink."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceAssociationLinkPropertiesFormat {
    #[doc = "Resource type of the linked resource."]
    #[serde(rename = "linkedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub linked_resource_type: Option<String>,
    #[doc = "Link to the external resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[doc = "Provisioning state of the ServiceAssociationLink resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ServiceAssociationLinkPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a service delegation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceDelegationPropertiesFormat {
    #[doc = "The name of the service to whom the subnet should be delegated (e.g. Microsoft.Sql/servers)"]
    #[serde(rename = "serviceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[doc = "Describes the actions permitted to the service upon delegation"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<String>,
    #[doc = "The provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ServiceDelegationPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service End point policy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointPolicy {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Service Endpoint Policy resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceEndpointPolicyPropertiesFormat>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ServiceEndpointPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service Endpoint policy definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointPolicyDefinition {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Service Endpoint policy definition resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceEndpointPolicyDefinitionPropertiesFormat>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ServiceEndpointPolicyDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service Endpoint policy definition resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointPolicyDefinitionPropertiesFormat {
    #[doc = "A description for this rule. Restricted to 140 chars."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "service endpoint name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[doc = "A list of service resources."]
    #[serde(
        rename = "serviceResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub service_resources: Vec<String>,
    #[doc = "The provisioning state of the service end point policy definition. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ServiceEndpointPolicyDefinitionPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service Endpoint Policy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointPolicyPropertiesFormat {
    #[doc = "A collection of service endpoint policy definitions of the service endpoint policy."]
    #[serde(
        rename = "serviceEndpointPolicyDefinitions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub service_endpoint_policy_definitions: Vec<ServiceEndpointPolicyDefinition>,
    #[doc = "A collection of references to subnets."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subnets: Vec<Subnet>,
    #[doc = "The resource GUID property of the service endpoint policy resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[doc = "The provisioning state of the service endpoint policy. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ServiceEndpointPolicyPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The service endpoint properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointPropertiesFormat {
    #[doc = "The type of the endpoint service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[doc = "A list of locations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub locations: Vec<String>,
    #[doc = "The provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ServiceEndpointPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to another subresource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subnet in a virtual network resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Subnet {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of the subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubnetPropertiesFormat>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Subnet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for ListSubnets API service callRetrieves all subnet that belongs to a virtual network"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubnetListResult {
    #[doc = "The subnets in a virtual network."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Subnet>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SubnetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SubnetListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the subnet."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubnetPropertiesFormat {
    #[doc = "The address prefix for the subnet."]
    #[serde(rename = "addressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub address_prefix: Option<String>,
    #[doc = "List of  address prefixes for the subnet."]
    #[serde(
        rename = "addressPrefixes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub address_prefixes: Vec<String>,
    #[doc = "NetworkSecurityGroup resource."]
    #[serde(rename = "networkSecurityGroup", default, skip_serializing_if = "Option::is_none")]
    pub network_security_group: Option<NetworkSecurityGroup>,
    #[doc = "Route table resource."]
    #[serde(rename = "routeTable", default, skip_serializing_if = "Option::is_none")]
    pub route_table: Option<RouteTable>,
    #[doc = "An array of service endpoints."]
    #[serde(
        rename = "serviceEndpoints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub service_endpoints: Vec<ServiceEndpointPropertiesFormat>,
    #[doc = "An array of service endpoint policies."]
    #[serde(
        rename = "serviceEndpointPolicies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub service_endpoint_policies: Vec<ServiceEndpointPolicy>,
    #[doc = "An array of references to interface endpoints "]
    #[serde(
        rename = "interfaceEndpoints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub interface_endpoints: Vec<InterfaceEndpoint>,
    #[doc = "Gets an array of references to the network interface IP configurations using subnet."]
    #[serde(
        rename = "ipConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_configurations: Vec<IpConfiguration>,
    #[doc = "Array of IP configuration profiles which reference this subnet."]
    #[serde(
        rename = "ipConfigurationProfiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_configuration_profiles: Vec<IpConfigurationProfile>,
    #[doc = "Gets an array of references to the external resources using subnet."]
    #[serde(
        rename = "resourceNavigationLinks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resource_navigation_links: Vec<ResourceNavigationLink>,
    #[doc = "Gets an array of references to services injecting into this subnet."]
    #[serde(
        rename = "serviceAssociationLinks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub service_association_links: Vec<ServiceAssociationLink>,
    #[doc = "Gets an array of references to the delegations on the subnet."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub delegations: Vec<Delegation>,
    #[doc = "A read-only string identifying the intention of use for this subnet based on delegations and other user-defined properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[doc = "The provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl SubnetPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
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
#[doc = "The transport protocol for the endpoint. Possible values are 'Udp' or 'Tcp' or 'All'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TransportProtocol")]
pub enum TransportProtocol {
    Udp,
    Tcp,
    All,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TransportProtocol {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TransportProtocol {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TransportProtocol {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Udp => serializer.serialize_unit_variant("TransportProtocol", 0u32, "Udp"),
            Self::Tcp => serializer.serialize_unit_variant("TransportProtocol", 1u32, "Tcp"),
            Self::All => serializer.serialize_unit_variant("TransportProtocol", 2u32, "All"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "VirtualNetworkGatewayConnection properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TunnelConnectionHealth {
    #[doc = "Tunnel name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tunnel: Option<String>,
    #[doc = "Virtual network Gateway connection status"]
    #[serde(rename = "connectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<tunnel_connection_health::ConnectionStatus>,
    #[doc = "The Ingress Bytes Transferred in this connection"]
    #[serde(rename = "ingressBytesTransferred", default, skip_serializing_if = "Option::is_none")]
    pub ingress_bytes_transferred: Option<i64>,
    #[doc = "The Egress Bytes Transferred in this connection"]
    #[serde(rename = "egressBytesTransferred", default, skip_serializing_if = "Option::is_none")]
    pub egress_bytes_transferred: Option<i64>,
    #[doc = "The time at which connection was established in Utc format."]
    #[serde(rename = "lastConnectionEstablishedUtcTime", default, skip_serializing_if = "Option::is_none")]
    pub last_connection_established_utc_time: Option<String>,
}
impl TunnelConnectionHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod tunnel_connection_health {
    use super::*;
    #[doc = "Virtual network Gateway connection status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConnectionStatus")]
    pub enum ConnectionStatus {
        Unknown,
        Connecting,
        Connected,
        NotConnected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConnectionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConnectionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConnectionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("ConnectionStatus", 0u32, "Unknown"),
                Self::Connecting => serializer.serialize_unit_variant("ConnectionStatus", 1u32, "Connecting"),
                Self::Connected => serializer.serialize_unit_variant("ConnectionStatus", 2u32, "Connected"),
                Self::NotConnected => serializer.serialize_unit_variant("ConnectionStatus", 3u32, "NotConnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Virtual Network resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetwork {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the virtual network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkPropertiesFormat>,
    #[doc = "Gets a unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl VirtualNetwork {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A common class for general resource information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkGateway {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "VirtualNetworkGateway properties"]
    pub properties: VirtualNetworkGatewayPropertiesFormat,
    #[doc = "Gets a unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl VirtualNetworkGateway {
    pub fn new(properties: VirtualNetworkGatewayPropertiesFormat) -> Self {
        Self {
            resource: Resource::default(),
            properties,
            etag: None,
        }
    }
}
#[doc = "A common class for general resource information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkGatewayConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "VirtualNetworkGatewayConnection properties"]
    pub properties: VirtualNetworkGatewayConnectionPropertiesFormat,
    #[doc = "Gets a unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl VirtualNetworkGatewayConnection {
    pub fn new(properties: VirtualNetworkGatewayConnectionPropertiesFormat) -> Self {
        Self {
            resource: Resource::default(),
            properties,
            etag: None,
        }
    }
}
#[doc = "A common class for general resource information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkGatewayConnectionListEntity {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "VirtualNetworkGatewayConnection properties"]
    pub properties: VirtualNetworkGatewayConnectionListEntityPropertiesFormat,
    #[doc = "Gets a unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl VirtualNetworkGatewayConnectionListEntity {
    pub fn new(properties: VirtualNetworkGatewayConnectionListEntityPropertiesFormat) -> Self {
        Self {
            resource: Resource::default(),
            properties,
            etag: None,
        }
    }
}
#[doc = "VirtualNetworkGatewayConnection properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkGatewayConnectionListEntityPropertiesFormat {
    #[doc = "The authorizationKey."]
    #[serde(rename = "authorizationKey", default, skip_serializing_if = "Option::is_none")]
    pub authorization_key: Option<String>,
    #[doc = "A reference to VirtualNetworkGateway or LocalNetworkGateway resource."]
    #[serde(rename = "virtualNetworkGateway1")]
    pub virtual_network_gateway1: VirtualNetworkConnectionGatewayReference,
    #[doc = "A reference to VirtualNetworkGateway or LocalNetworkGateway resource."]
    #[serde(rename = "virtualNetworkGateway2", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_gateway2: Option<VirtualNetworkConnectionGatewayReference>,
    #[doc = "A reference to VirtualNetworkGateway or LocalNetworkGateway resource."]
    #[serde(rename = "localNetworkGateway2", default, skip_serializing_if = "Option::is_none")]
    pub local_network_gateway2: Option<VirtualNetworkConnectionGatewayReference>,
    #[doc = "Gateway connection type. Possible values are: 'Ipsec','Vnet2Vnet','ExpressRoute', and 'VPNClient."]
    #[serde(rename = "connectionType")]
    pub connection_type: virtual_network_gateway_connection_list_entity_properties_format::ConnectionType,
    #[doc = "Gateway connection protocol. Possible values are: 'IKEv2', 'IKEv1'."]
    #[serde(rename = "connectionProtocol", default, skip_serializing_if = "Option::is_none")]
    pub connection_protocol: Option<ConnectionProtocol>,
    #[doc = "The routing weight."]
    #[serde(rename = "routingWeight", default, skip_serializing_if = "Option::is_none")]
    pub routing_weight: Option<i32>,
    #[doc = "The IPSec shared key."]
    #[serde(rename = "sharedKey", default, skip_serializing_if = "Option::is_none")]
    pub shared_key: Option<String>,
    #[doc = "Virtual network Gateway connection status. Possible values are 'Unknown', 'Connecting', 'Connected' and 'NotConnected'."]
    #[serde(rename = "connectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<virtual_network_gateway_connection_list_entity_properties_format::ConnectionStatus>,
    #[doc = "Collection of all tunnels' connection health status."]
    #[serde(
        rename = "tunnelConnectionStatus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tunnel_connection_status: Vec<TunnelConnectionHealth>,
    #[doc = "The egress bytes transferred in this connection."]
    #[serde(rename = "egressBytesTransferred", default, skip_serializing_if = "Option::is_none")]
    pub egress_bytes_transferred: Option<i64>,
    #[doc = "The ingress bytes transferred in this connection."]
    #[serde(rename = "ingressBytesTransferred", default, skip_serializing_if = "Option::is_none")]
    pub ingress_bytes_transferred: Option<i64>,
    #[doc = "Reference to another subresource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer: Option<SubResource>,
    #[doc = "EnableBgp flag"]
    #[serde(rename = "enableBgp", default, skip_serializing_if = "Option::is_none")]
    pub enable_bgp: Option<bool>,
    #[doc = "Enable policy-based traffic selectors."]
    #[serde(rename = "usePolicyBasedTrafficSelectors", default, skip_serializing_if = "Option::is_none")]
    pub use_policy_based_traffic_selectors: Option<bool>,
    #[doc = "The IPSec Policies to be considered by this connection."]
    #[serde(
        rename = "ipsecPolicies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ipsec_policies: Vec<IpsecPolicy>,
    #[doc = "The resource GUID property of the VirtualNetworkGatewayConnection resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[doc = "The provisioning state of the VirtualNetworkGatewayConnection resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Bypass ExpressRoute Gateway for data forwarding"]
    #[serde(rename = "expressRouteGatewayBypass", default, skip_serializing_if = "Option::is_none")]
    pub express_route_gateway_bypass: Option<bool>,
}
impl VirtualNetworkGatewayConnectionListEntityPropertiesFormat {
    pub fn new(
        virtual_network_gateway1: VirtualNetworkConnectionGatewayReference,
        connection_type: virtual_network_gateway_connection_list_entity_properties_format::ConnectionType,
    ) -> Self {
        Self {
            authorization_key: None,
            virtual_network_gateway1,
            virtual_network_gateway2: None,
            local_network_gateway2: None,
            connection_type,
            connection_protocol: None,
            routing_weight: None,
            shared_key: None,
            connection_status: None,
            tunnel_connection_status: Vec::new(),
            egress_bytes_transferred: None,
            ingress_bytes_transferred: None,
            peer: None,
            enable_bgp: None,
            use_policy_based_traffic_selectors: None,
            ipsec_policies: Vec::new(),
            resource_guid: None,
            provisioning_state: None,
            express_route_gateway_bypass: None,
        }
    }
}
pub mod virtual_network_gateway_connection_list_entity_properties_format {
    use super::*;
    #[doc = "Gateway connection type. Possible values are: 'Ipsec','Vnet2Vnet','ExpressRoute', and 'VPNClient."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConnectionType")]
    pub enum ConnectionType {
        IPsec,
        Vnet2Vnet,
        ExpressRoute,
        #[serde(rename = "VPNClient")]
        VpnClient,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConnectionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConnectionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConnectionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IPsec => serializer.serialize_unit_variant("ConnectionType", 0u32, "IPsec"),
                Self::Vnet2Vnet => serializer.serialize_unit_variant("ConnectionType", 1u32, "Vnet2Vnet"),
                Self::ExpressRoute => serializer.serialize_unit_variant("ConnectionType", 2u32, "ExpressRoute"),
                Self::VpnClient => serializer.serialize_unit_variant("ConnectionType", 3u32, "VPNClient"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Virtual network Gateway connection status. Possible values are 'Unknown', 'Connecting', 'Connected' and 'NotConnected'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConnectionStatus")]
    pub enum ConnectionStatus {
        Unknown,
        Connecting,
        Connected,
        NotConnected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConnectionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConnectionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConnectionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("ConnectionStatus", 0u32, "Unknown"),
                Self::Connecting => serializer.serialize_unit_variant("ConnectionStatus", 1u32, "Connecting"),
                Self::Connected => serializer.serialize_unit_variant("ConnectionStatus", 2u32, "Connected"),
                Self::NotConnected => serializer.serialize_unit_variant("ConnectionStatus", 3u32, "NotConnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Response for the ListVirtualNetworkGatewayConnections API service call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkGatewayConnectionListResult {
    #[doc = "Gets a list of VirtualNetworkGatewayConnection resources that exists in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VirtualNetworkGatewayConnection>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkGatewayConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VirtualNetworkGatewayConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VirtualNetworkGatewayConnection properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkGatewayConnectionPropertiesFormat {
    #[doc = "The authorizationKey."]
    #[serde(rename = "authorizationKey", default, skip_serializing_if = "Option::is_none")]
    pub authorization_key: Option<String>,
    #[doc = "A common class for general resource information"]
    #[serde(rename = "virtualNetworkGateway1")]
    pub virtual_network_gateway1: VirtualNetworkGateway,
    #[doc = "A common class for general resource information"]
    #[serde(rename = "virtualNetworkGateway2", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_gateway2: Option<VirtualNetworkGateway>,
    #[doc = "A common class for general resource information"]
    #[serde(rename = "localNetworkGateway2", default, skip_serializing_if = "Option::is_none")]
    pub local_network_gateway2: Option<LocalNetworkGateway>,
    #[doc = "Gateway connection type. Possible values are: 'Ipsec','Vnet2Vnet','ExpressRoute', and 'VPNClient."]
    #[serde(rename = "connectionType")]
    pub connection_type: virtual_network_gateway_connection_properties_format::ConnectionType,
    #[doc = "Gateway connection protocol. Possible values are: 'IKEv2', 'IKEv1'."]
    #[serde(rename = "connectionProtocol", default, skip_serializing_if = "Option::is_none")]
    pub connection_protocol: Option<ConnectionProtocol>,
    #[doc = "The routing weight."]
    #[serde(rename = "routingWeight", default, skip_serializing_if = "Option::is_none")]
    pub routing_weight: Option<i32>,
    #[doc = "The IPSec shared key."]
    #[serde(rename = "sharedKey", default, skip_serializing_if = "Option::is_none")]
    pub shared_key: Option<String>,
    #[doc = "Virtual network Gateway connection status. Possible values are 'Unknown', 'Connecting', 'Connected' and 'NotConnected'."]
    #[serde(rename = "connectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<virtual_network_gateway_connection_properties_format::ConnectionStatus>,
    #[doc = "Collection of all tunnels' connection health status."]
    #[serde(
        rename = "tunnelConnectionStatus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tunnel_connection_status: Vec<TunnelConnectionHealth>,
    #[doc = "The egress bytes transferred in this connection."]
    #[serde(rename = "egressBytesTransferred", default, skip_serializing_if = "Option::is_none")]
    pub egress_bytes_transferred: Option<i64>,
    #[doc = "The ingress bytes transferred in this connection."]
    #[serde(rename = "ingressBytesTransferred", default, skip_serializing_if = "Option::is_none")]
    pub ingress_bytes_transferred: Option<i64>,
    #[doc = "Reference to another subresource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer: Option<SubResource>,
    #[doc = "EnableBgp flag"]
    #[serde(rename = "enableBgp", default, skip_serializing_if = "Option::is_none")]
    pub enable_bgp: Option<bool>,
    #[doc = "Enable policy-based traffic selectors."]
    #[serde(rename = "usePolicyBasedTrafficSelectors", default, skip_serializing_if = "Option::is_none")]
    pub use_policy_based_traffic_selectors: Option<bool>,
    #[doc = "The IPSec Policies to be considered by this connection."]
    #[serde(
        rename = "ipsecPolicies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ipsec_policies: Vec<IpsecPolicy>,
    #[doc = "The resource GUID property of the VirtualNetworkGatewayConnection resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[doc = "The provisioning state of the VirtualNetworkGatewayConnection resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Bypass ExpressRoute Gateway for data forwarding"]
    #[serde(rename = "expressRouteGatewayBypass", default, skip_serializing_if = "Option::is_none")]
    pub express_route_gateway_bypass: Option<bool>,
}
impl VirtualNetworkGatewayConnectionPropertiesFormat {
    pub fn new(
        virtual_network_gateway1: VirtualNetworkGateway,
        connection_type: virtual_network_gateway_connection_properties_format::ConnectionType,
    ) -> Self {
        Self {
            authorization_key: None,
            virtual_network_gateway1,
            virtual_network_gateway2: None,
            local_network_gateway2: None,
            connection_type,
            connection_protocol: None,
            routing_weight: None,
            shared_key: None,
            connection_status: None,
            tunnel_connection_status: Vec::new(),
            egress_bytes_transferred: None,
            ingress_bytes_transferred: None,
            peer: None,
            enable_bgp: None,
            use_policy_based_traffic_selectors: None,
            ipsec_policies: Vec::new(),
            resource_guid: None,
            provisioning_state: None,
            express_route_gateway_bypass: None,
        }
    }
}
pub mod virtual_network_gateway_connection_properties_format {
    use super::*;
    #[doc = "Gateway connection type. Possible values are: 'Ipsec','Vnet2Vnet','ExpressRoute', and 'VPNClient."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConnectionType")]
    pub enum ConnectionType {
        IPsec,
        Vnet2Vnet,
        ExpressRoute,
        #[serde(rename = "VPNClient")]
        VpnClient,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConnectionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConnectionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConnectionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IPsec => serializer.serialize_unit_variant("ConnectionType", 0u32, "IPsec"),
                Self::Vnet2Vnet => serializer.serialize_unit_variant("ConnectionType", 1u32, "Vnet2Vnet"),
                Self::ExpressRoute => serializer.serialize_unit_variant("ConnectionType", 2u32, "ExpressRoute"),
                Self::VpnClient => serializer.serialize_unit_variant("ConnectionType", 3u32, "VPNClient"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Virtual network Gateway connection status. Possible values are 'Unknown', 'Connecting', 'Connected' and 'NotConnected'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConnectionStatus")]
    pub enum ConnectionStatus {
        Unknown,
        Connecting,
        Connected,
        NotConnected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConnectionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConnectionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConnectionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("ConnectionStatus", 0u32, "Unknown"),
                Self::Connecting => serializer.serialize_unit_variant("ConnectionStatus", 1u32, "Connecting"),
                Self::Connected => serializer.serialize_unit_variant("ConnectionStatus", 2u32, "Connected"),
                Self::NotConnected => serializer.serialize_unit_variant("ConnectionStatus", 3u32, "NotConnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "IP configuration for virtual network gateway"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkGatewayIpConfiguration {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of VirtualNetworkGatewayIPConfiguration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkGatewayIpConfigurationPropertiesFormat>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl VirtualNetworkGatewayIpConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of VirtualNetworkGatewayIPConfiguration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkGatewayIpConfigurationPropertiesFormat {
    #[doc = "The private IP allocation method. Possible values are: 'Static' and 'Dynamic'."]
    #[serde(rename = "privateIPAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_allocation_method: Option<virtual_network_gateway_ip_configuration_properties_format::PrivateIpAllocationMethod>,
    #[doc = "Reference to another subresource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<SubResource>,
    #[doc = "Reference to another subresource."]
    #[serde(rename = "publicIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<SubResource>,
    #[doc = "The provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl VirtualNetworkGatewayIpConfigurationPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_network_gateway_ip_configuration_properties_format {
    use super::*;
    #[doc = "The private IP allocation method. Possible values are: 'Static' and 'Dynamic'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrivateIpAllocationMethod")]
    pub enum PrivateIpAllocationMethod {
        Static,
        Dynamic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrivateIpAllocationMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrivateIpAllocationMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrivateIpAllocationMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Static => serializer.serialize_unit_variant("PrivateIpAllocationMethod", 0u32, "Static"),
                Self::Dynamic => serializer.serialize_unit_variant("PrivateIpAllocationMethod", 1u32, "Dynamic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Response for the VirtualNetworkGatewayListConnections API service call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkGatewayListConnectionsResult {
    #[doc = "Gets a list of VirtualNetworkGatewayConnection resources that exists in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VirtualNetworkGatewayConnectionListEntity>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkGatewayListConnectionsResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VirtualNetworkGatewayListConnectionsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for the ListVirtualNetworkGateways API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkGatewayListResult {
    #[doc = "Gets a list of VirtualNetworkGateway resources that exists in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VirtualNetworkGateway>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkGatewayListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VirtualNetworkGatewayListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VirtualNetworkGateway properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkGatewayPropertiesFormat {
    #[doc = "IP configurations for virtual network gateway."]
    #[serde(
        rename = "ipConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_configurations: Vec<VirtualNetworkGatewayIpConfiguration>,
    #[doc = "The type of this virtual network gateway. Possible values are: 'Vpn' and 'ExpressRoute'."]
    #[serde(rename = "gatewayType", default, skip_serializing_if = "Option::is_none")]
    pub gateway_type: Option<virtual_network_gateway_properties_format::GatewayType>,
    #[doc = "The type of this virtual network gateway. Possible values are: 'PolicyBased' and 'RouteBased'."]
    #[serde(rename = "vpnType", default, skip_serializing_if = "Option::is_none")]
    pub vpn_type: Option<virtual_network_gateway_properties_format::VpnType>,
    #[doc = "Whether BGP is enabled for this virtual network gateway or not."]
    #[serde(rename = "enableBgp", default, skip_serializing_if = "Option::is_none")]
    pub enable_bgp: Option<bool>,
    #[doc = "ActiveActive flag"]
    #[serde(rename = "activeActive", default, skip_serializing_if = "Option::is_none")]
    pub active_active: Option<bool>,
    #[doc = "Reference to another subresource."]
    #[serde(rename = "gatewayDefaultSite", default, skip_serializing_if = "Option::is_none")]
    pub gateway_default_site: Option<SubResource>,
    #[doc = "VirtualNetworkGatewaySku details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<VirtualNetworkGatewaySku>,
    #[doc = "VpnClientConfiguration for P2S client."]
    #[serde(rename = "vpnClientConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub vpn_client_configuration: Option<VpnClientConfiguration>,
    #[doc = "BGP settings details"]
    #[serde(rename = "bgpSettings", default, skip_serializing_if = "Option::is_none")]
    pub bgp_settings: Option<BgpSettings>,
    #[doc = "The resource GUID property of the VirtualNetworkGateway resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[doc = "The provisioning state of the VirtualNetworkGateway resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl VirtualNetworkGatewayPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_network_gateway_properties_format {
    use super::*;
    #[doc = "The type of this virtual network gateway. Possible values are: 'Vpn' and 'ExpressRoute'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "GatewayType")]
    pub enum GatewayType {
        Vpn,
        ExpressRoute,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for GatewayType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for GatewayType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for GatewayType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Vpn => serializer.serialize_unit_variant("GatewayType", 0u32, "Vpn"),
                Self::ExpressRoute => serializer.serialize_unit_variant("GatewayType", 1u32, "ExpressRoute"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of this virtual network gateway. Possible values are: 'PolicyBased' and 'RouteBased'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VpnType")]
    pub enum VpnType {
        PolicyBased,
        RouteBased,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VpnType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VpnType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VpnType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PolicyBased => serializer.serialize_unit_variant("VpnType", 0u32, "PolicyBased"),
                Self::RouteBased => serializer.serialize_unit_variant("VpnType", 1u32, "RouteBased"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "VirtualNetworkGatewaySku details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkGatewaySku {
    #[doc = "Gateway SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<virtual_network_gateway_sku::Name>,
    #[doc = "Gateway SKU tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<virtual_network_gateway_sku::Tier>,
    #[doc = "The capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl VirtualNetworkGatewaySku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_network_gateway_sku {
    use super::*;
    #[doc = "Gateway SKU name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Basic,
        HighPerformance,
        Standard,
        UltraPerformance,
        VpnGw1,
        VpnGw2,
        VpnGw3,
        #[serde(rename = "VpnGw1AZ")]
        VpnGw1Az,
        #[serde(rename = "VpnGw2AZ")]
        VpnGw2Az,
        #[serde(rename = "VpnGw3AZ")]
        VpnGw3Az,
        #[serde(rename = "ErGw1AZ")]
        ErGw1Az,
        #[serde(rename = "ErGw2AZ")]
        ErGw2Az,
        #[serde(rename = "ErGw3AZ")]
        ErGw3Az,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("Name", 0u32, "Basic"),
                Self::HighPerformance => serializer.serialize_unit_variant("Name", 1u32, "HighPerformance"),
                Self::Standard => serializer.serialize_unit_variant("Name", 2u32, "Standard"),
                Self::UltraPerformance => serializer.serialize_unit_variant("Name", 3u32, "UltraPerformance"),
                Self::VpnGw1 => serializer.serialize_unit_variant("Name", 4u32, "VpnGw1"),
                Self::VpnGw2 => serializer.serialize_unit_variant("Name", 5u32, "VpnGw2"),
                Self::VpnGw3 => serializer.serialize_unit_variant("Name", 6u32, "VpnGw3"),
                Self::VpnGw1Az => serializer.serialize_unit_variant("Name", 7u32, "VpnGw1AZ"),
                Self::VpnGw2Az => serializer.serialize_unit_variant("Name", 8u32, "VpnGw2AZ"),
                Self::VpnGw3Az => serializer.serialize_unit_variant("Name", 9u32, "VpnGw3AZ"),
                Self::ErGw1Az => serializer.serialize_unit_variant("Name", 10u32, "ErGw1AZ"),
                Self::ErGw2Az => serializer.serialize_unit_variant("Name", 11u32, "ErGw2AZ"),
                Self::ErGw3Az => serializer.serialize_unit_variant("Name", 12u32, "ErGw3AZ"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gateway SKU tier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Basic,
        HighPerformance,
        Standard,
        UltraPerformance,
        VpnGw1,
        VpnGw2,
        VpnGw3,
        #[serde(rename = "VpnGw1AZ")]
        VpnGw1Az,
        #[serde(rename = "VpnGw2AZ")]
        VpnGw2Az,
        #[serde(rename = "VpnGw3AZ")]
        VpnGw3Az,
        #[serde(rename = "ErGw1AZ")]
        ErGw1Az,
        #[serde(rename = "ErGw2AZ")]
        ErGw2Az,
        #[serde(rename = "ErGw3AZ")]
        ErGw3Az,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("Tier", 0u32, "Basic"),
                Self::HighPerformance => serializer.serialize_unit_variant("Tier", 1u32, "HighPerformance"),
                Self::Standard => serializer.serialize_unit_variant("Tier", 2u32, "Standard"),
                Self::UltraPerformance => serializer.serialize_unit_variant("Tier", 3u32, "UltraPerformance"),
                Self::VpnGw1 => serializer.serialize_unit_variant("Tier", 4u32, "VpnGw1"),
                Self::VpnGw2 => serializer.serialize_unit_variant("Tier", 5u32, "VpnGw2"),
                Self::VpnGw3 => serializer.serialize_unit_variant("Tier", 6u32, "VpnGw3"),
                Self::VpnGw1Az => serializer.serialize_unit_variant("Tier", 7u32, "VpnGw1AZ"),
                Self::VpnGw2Az => serializer.serialize_unit_variant("Tier", 8u32, "VpnGw2AZ"),
                Self::VpnGw3Az => serializer.serialize_unit_variant("Tier", 9u32, "VpnGw3AZ"),
                Self::ErGw1Az => serializer.serialize_unit_variant("Tier", 10u32, "ErGw1AZ"),
                Self::ErGw2Az => serializer.serialize_unit_variant("Tier", 11u32, "ErGw2AZ"),
                Self::ErGw3Az => serializer.serialize_unit_variant("Tier", 12u32, "ErGw3AZ"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Response for the ListVirtualNetworks API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkListResult {
    #[doc = "Gets a list of VirtualNetwork resources in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VirtualNetwork>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VirtualNetworkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for the virtual networks GetUsage API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkListUsageResult {
    #[doc = "VirtualNetwork usage stats."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VirtualNetworkUsage>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkListUsageResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VirtualNetworkListUsageResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Peerings in a virtual network resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkPeering {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of the virtual network peering."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkPeeringPropertiesFormat>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl VirtualNetworkPeering {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for ListSubnets API service call. Retrieves all subnets that belong to a virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkPeeringListResult {
    #[doc = "The peerings in a virtual network."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VirtualNetworkPeering>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkPeeringListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VirtualNetworkPeeringListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the virtual network peering."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkPeeringPropertiesFormat {
    #[doc = "Whether the VMs in the linked virtual network space would be able to access all the VMs in local Virtual network space."]
    #[serde(rename = "allowVirtualNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub allow_virtual_network_access: Option<bool>,
    #[doc = "Whether the forwarded traffic from the VMs in the remote virtual network will be allowed/disallowed."]
    #[serde(rename = "allowForwardedTraffic", default, skip_serializing_if = "Option::is_none")]
    pub allow_forwarded_traffic: Option<bool>,
    #[doc = "If gateway links can be used in remote virtual networking to link to this virtual network."]
    #[serde(rename = "allowGatewayTransit", default, skip_serializing_if = "Option::is_none")]
    pub allow_gateway_transit: Option<bool>,
    #[doc = "If remote gateways can be used on this virtual network. If the flag is set to true, and allowGatewayTransit on remote peering is also true, virtual network will use gateways of remote virtual network for transit. Only one peering can have this flag set to true. This flag cannot be set if virtual network already has a gateway."]
    #[serde(rename = "useRemoteGateways", default, skip_serializing_if = "Option::is_none")]
    pub use_remote_gateways: Option<bool>,
    #[doc = "Reference to another subresource."]
    #[serde(rename = "remoteVirtualNetwork", default, skip_serializing_if = "Option::is_none")]
    pub remote_virtual_network: Option<SubResource>,
    #[doc = "AddressSpace contains an array of IP address ranges that can be used by subnets of the virtual network."]
    #[serde(rename = "remoteAddressSpace", default, skip_serializing_if = "Option::is_none")]
    pub remote_address_space: Option<AddressSpace>,
    #[doc = "The status of the virtual network peering. Possible values are 'Initiated', 'Connected', and 'Disconnected'."]
    #[serde(rename = "peeringState", default, skip_serializing_if = "Option::is_none")]
    pub peering_state: Option<virtual_network_peering_properties_format::PeeringState>,
    #[doc = "The provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl VirtualNetworkPeeringPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_network_peering_properties_format {
    use super::*;
    #[doc = "The status of the virtual network peering. Possible values are 'Initiated', 'Connected', and 'Disconnected'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PeeringState")]
    pub enum PeeringState {
        Initiated,
        Connected,
        Disconnected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PeeringState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PeeringState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PeeringState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Initiated => serializer.serialize_unit_variant("PeeringState", 0u32, "Initiated"),
                Self::Connected => serializer.serialize_unit_variant("PeeringState", 1u32, "Connected"),
                Self::Disconnected => serializer.serialize_unit_variant("PeeringState", 2u32, "Disconnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkPropertiesFormat {
    #[doc = "AddressSpace contains an array of IP address ranges that can be used by subnets of the virtual network."]
    #[serde(rename = "addressSpace", default, skip_serializing_if = "Option::is_none")]
    pub address_space: Option<AddressSpace>,
    #[doc = "DhcpOptions contains an array of DNS servers available to VMs deployed in the virtual network. Standard DHCP option for a subnet overrides VNET DHCP options."]
    #[serde(rename = "dhcpOptions", default, skip_serializing_if = "Option::is_none")]
    pub dhcp_options: Option<DhcpOptions>,
    #[doc = "A list of subnets in a Virtual Network."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subnets: Vec<Subnet>,
    #[doc = "A list of peerings in a Virtual Network."]
    #[serde(
        rename = "virtualNetworkPeerings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub virtual_network_peerings: Vec<VirtualNetworkPeering>,
    #[doc = "The resourceGuid property of the Virtual Network resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[doc = "The provisioning state of the PublicIP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Indicates if DDoS protection is enabled for all the protected resources in the virtual network. It requires a DDoS protection plan associated with the resource."]
    #[serde(rename = "enableDdosProtection", default, skip_serializing_if = "Option::is_none")]
    pub enable_ddos_protection: Option<bool>,
    #[doc = "Indicates if VM protection is enabled for all the subnets in the virtual network."]
    #[serde(rename = "enableVmProtection", default, skip_serializing_if = "Option::is_none")]
    pub enable_vm_protection: Option<bool>,
    #[doc = "Reference to another subresource."]
    #[serde(rename = "ddosProtectionPlan", default, skip_serializing_if = "Option::is_none")]
    pub ddos_protection_plan: Option<SubResource>,
}
impl VirtualNetworkPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual Network Tap resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkTap {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Virtual Network Tap properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkTapPropertiesFormat>,
    #[doc = "Gets a unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl VirtualNetworkTap {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual Network Tap properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkTapPropertiesFormat {
    #[doc = "Specifies the list of resource IDs for the network interface IP configuration that needs to be tapped."]
    #[serde(
        rename = "networkInterfaceTapConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_interface_tap_configurations: Vec<NetworkInterfaceTapConfiguration>,
    #[doc = "The resourceGuid property of the virtual network tap."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[doc = "The provisioning state of the virtual network tap. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "IPConfiguration in a network interface."]
    #[serde(
        rename = "destinationNetworkInterfaceIPConfiguration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_network_interface_ip_configuration: Option<NetworkInterfaceIpConfiguration>,
    #[doc = "Frontend IP address of the load balancer."]
    #[serde(
        rename = "destinationLoadBalancerFrontEndIPConfiguration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_load_balancer_front_end_ip_configuration: Option<FrontendIpConfiguration>,
    #[doc = "The VXLAN destination port that will receive the tapped traffic."]
    #[serde(rename = "destinationPort", default, skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<i64>,
}
impl VirtualNetworkTapPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Usage details for subnet."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkUsage {
    #[doc = "Indicates number of IPs used from the Subnet."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[doc = "Subnet identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Indicates the size of the subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[doc = "Usage strings container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<VirtualNetworkUsageName>,
    #[doc = "Usage units. Returns 'Count'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl VirtualNetworkUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Usage strings container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkUsageName {
    #[doc = "Localized subnet size and usage string."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
    #[doc = "Subnet size and usage string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl VirtualNetworkUsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VpnClientConfiguration for P2S client."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VpnClientConfiguration {
    #[doc = "AddressSpace contains an array of IP address ranges that can be used by subnets of the virtual network."]
    #[serde(rename = "vpnClientAddressPool", default, skip_serializing_if = "Option::is_none")]
    pub vpn_client_address_pool: Option<AddressSpace>,
    #[doc = "VpnClientRootCertificate for virtual network gateway."]
    #[serde(
        rename = "vpnClientRootCertificates",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vpn_client_root_certificates: Vec<VpnClientRootCertificate>,
    #[doc = "VpnClientRevokedCertificate for Virtual network gateway."]
    #[serde(
        rename = "vpnClientRevokedCertificates",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vpn_client_revoked_certificates: Vec<VpnClientRevokedCertificate>,
    #[doc = "VpnClientProtocols for Virtual network gateway."]
    #[serde(
        rename = "vpnClientProtocols",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vpn_client_protocols: Vec<String>,
    #[doc = "VpnClientIpsecPolicies for virtual network gateway P2S client."]
    #[serde(
        rename = "vpnClientIpsecPolicies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vpn_client_ipsec_policies: Vec<IpsecPolicy>,
    #[doc = "The radius server address property of the VirtualNetworkGateway resource for vpn client connection."]
    #[serde(rename = "radiusServerAddress", default, skip_serializing_if = "Option::is_none")]
    pub radius_server_address: Option<String>,
    #[doc = "The radius secret property of the VirtualNetworkGateway resource for vpn client connection."]
    #[serde(rename = "radiusServerSecret", default, skip_serializing_if = "Option::is_none")]
    pub radius_server_secret: Option<String>,
}
impl VpnClientConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An IPSec parameters for a virtual network gateway P2S connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VpnClientIPsecParameters {
    #[doc = "The IPSec Security Association (also called Quick Mode or Phase 2 SA) lifetime in seconds for P2S client."]
    #[serde(rename = "saLifeTimeSeconds")]
    pub sa_life_time_seconds: i32,
    #[doc = "The IPSec Security Association (also called Quick Mode or Phase 2 SA) payload size in KB for P2S client.."]
    #[serde(rename = "saDataSizeKilobytes")]
    pub sa_data_size_kilobytes: i32,
    #[doc = "The IPSec encryption algorithm (IKE phase 1)."]
    #[serde(rename = "ipsecEncryption")]
    pub ipsec_encryption: vpn_client_i_psec_parameters::IpsecEncryption,
    #[doc = "The IPSec integrity algorithm (IKE phase 1)."]
    #[serde(rename = "ipsecIntegrity")]
    pub ipsec_integrity: vpn_client_i_psec_parameters::IpsecIntegrity,
    #[doc = "The IKE encryption algorithm (IKE phase 2)."]
    #[serde(rename = "ikeEncryption")]
    pub ike_encryption: vpn_client_i_psec_parameters::IkeEncryption,
    #[doc = "The IKE integrity algorithm (IKE phase 2)."]
    #[serde(rename = "ikeIntegrity")]
    pub ike_integrity: vpn_client_i_psec_parameters::IkeIntegrity,
    #[doc = "The DH Groups used in IKE Phase 1 for initial SA."]
    #[serde(rename = "dhGroup")]
    pub dh_group: vpn_client_i_psec_parameters::DhGroup,
    #[doc = "The Pfs Groups used in IKE Phase 2 for new child SA."]
    #[serde(rename = "pfsGroup")]
    pub pfs_group: vpn_client_i_psec_parameters::PfsGroup,
}
impl VpnClientIPsecParameters {
    pub fn new(
        sa_life_time_seconds: i32,
        sa_data_size_kilobytes: i32,
        ipsec_encryption: vpn_client_i_psec_parameters::IpsecEncryption,
        ipsec_integrity: vpn_client_i_psec_parameters::IpsecIntegrity,
        ike_encryption: vpn_client_i_psec_parameters::IkeEncryption,
        ike_integrity: vpn_client_i_psec_parameters::IkeIntegrity,
        dh_group: vpn_client_i_psec_parameters::DhGroup,
        pfs_group: vpn_client_i_psec_parameters::PfsGroup,
    ) -> Self {
        Self {
            sa_life_time_seconds,
            sa_data_size_kilobytes,
            ipsec_encryption,
            ipsec_integrity,
            ike_encryption,
            ike_integrity,
            dh_group,
            pfs_group,
        }
    }
}
pub mod vpn_client_i_psec_parameters {
    use super::*;
    #[doc = "The IPSec encryption algorithm (IKE phase 1)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IpsecEncryption")]
    pub enum IpsecEncryption {
        None,
        #[serde(rename = "DES")]
        Des,
        #[serde(rename = "DES3")]
        Des3,
        #[serde(rename = "AES128")]
        Aes128,
        #[serde(rename = "AES192")]
        Aes192,
        #[serde(rename = "AES256")]
        Aes256,
        #[serde(rename = "GCMAES128")]
        Gcmaes128,
        #[serde(rename = "GCMAES192")]
        Gcmaes192,
        #[serde(rename = "GCMAES256")]
        Gcmaes256,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IpsecEncryption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IpsecEncryption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IpsecEncryption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("IpsecEncryption", 0u32, "None"),
                Self::Des => serializer.serialize_unit_variant("IpsecEncryption", 1u32, "DES"),
                Self::Des3 => serializer.serialize_unit_variant("IpsecEncryption", 2u32, "DES3"),
                Self::Aes128 => serializer.serialize_unit_variant("IpsecEncryption", 3u32, "AES128"),
                Self::Aes192 => serializer.serialize_unit_variant("IpsecEncryption", 4u32, "AES192"),
                Self::Aes256 => serializer.serialize_unit_variant("IpsecEncryption", 5u32, "AES256"),
                Self::Gcmaes128 => serializer.serialize_unit_variant("IpsecEncryption", 6u32, "GCMAES128"),
                Self::Gcmaes192 => serializer.serialize_unit_variant("IpsecEncryption", 7u32, "GCMAES192"),
                Self::Gcmaes256 => serializer.serialize_unit_variant("IpsecEncryption", 8u32, "GCMAES256"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The IPSec integrity algorithm (IKE phase 1)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IpsecIntegrity")]
    pub enum IpsecIntegrity {
        #[serde(rename = "MD5")]
        Md5,
        #[serde(rename = "SHA1")]
        Sha1,
        #[serde(rename = "SHA256")]
        Sha256,
        #[serde(rename = "GCMAES128")]
        Gcmaes128,
        #[serde(rename = "GCMAES192")]
        Gcmaes192,
        #[serde(rename = "GCMAES256")]
        Gcmaes256,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IpsecIntegrity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IpsecIntegrity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IpsecIntegrity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Md5 => serializer.serialize_unit_variant("IpsecIntegrity", 0u32, "MD5"),
                Self::Sha1 => serializer.serialize_unit_variant("IpsecIntegrity", 1u32, "SHA1"),
                Self::Sha256 => serializer.serialize_unit_variant("IpsecIntegrity", 2u32, "SHA256"),
                Self::Gcmaes128 => serializer.serialize_unit_variant("IpsecIntegrity", 3u32, "GCMAES128"),
                Self::Gcmaes192 => serializer.serialize_unit_variant("IpsecIntegrity", 4u32, "GCMAES192"),
                Self::Gcmaes256 => serializer.serialize_unit_variant("IpsecIntegrity", 5u32, "GCMAES256"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The IKE encryption algorithm (IKE phase 2)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IkeEncryption")]
    pub enum IkeEncryption {
        #[serde(rename = "DES")]
        Des,
        #[serde(rename = "DES3")]
        Des3,
        #[serde(rename = "AES128")]
        Aes128,
        #[serde(rename = "AES192")]
        Aes192,
        #[serde(rename = "AES256")]
        Aes256,
        #[serde(rename = "GCMAES256")]
        Gcmaes256,
        #[serde(rename = "GCMAES128")]
        Gcmaes128,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IkeEncryption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IkeEncryption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IkeEncryption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Des => serializer.serialize_unit_variant("IkeEncryption", 0u32, "DES"),
                Self::Des3 => serializer.serialize_unit_variant("IkeEncryption", 1u32, "DES3"),
                Self::Aes128 => serializer.serialize_unit_variant("IkeEncryption", 2u32, "AES128"),
                Self::Aes192 => serializer.serialize_unit_variant("IkeEncryption", 3u32, "AES192"),
                Self::Aes256 => serializer.serialize_unit_variant("IkeEncryption", 4u32, "AES256"),
                Self::Gcmaes256 => serializer.serialize_unit_variant("IkeEncryption", 5u32, "GCMAES256"),
                Self::Gcmaes128 => serializer.serialize_unit_variant("IkeEncryption", 6u32, "GCMAES128"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The IKE integrity algorithm (IKE phase 2)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IkeIntegrity")]
    pub enum IkeIntegrity {
        #[serde(rename = "MD5")]
        Md5,
        #[serde(rename = "SHA1")]
        Sha1,
        #[serde(rename = "SHA256")]
        Sha256,
        #[serde(rename = "SHA384")]
        Sha384,
        #[serde(rename = "GCMAES256")]
        Gcmaes256,
        #[serde(rename = "GCMAES128")]
        Gcmaes128,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IkeIntegrity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IkeIntegrity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IkeIntegrity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Md5 => serializer.serialize_unit_variant("IkeIntegrity", 0u32, "MD5"),
                Self::Sha1 => serializer.serialize_unit_variant("IkeIntegrity", 1u32, "SHA1"),
                Self::Sha256 => serializer.serialize_unit_variant("IkeIntegrity", 2u32, "SHA256"),
                Self::Sha384 => serializer.serialize_unit_variant("IkeIntegrity", 3u32, "SHA384"),
                Self::Gcmaes256 => serializer.serialize_unit_variant("IkeIntegrity", 4u32, "GCMAES256"),
                Self::Gcmaes128 => serializer.serialize_unit_variant("IkeIntegrity", 5u32, "GCMAES128"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The DH Groups used in IKE Phase 1 for initial SA."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DhGroup")]
    pub enum DhGroup {
        None,
        #[serde(rename = "DHGroup1")]
        DhGroup1,
        #[serde(rename = "DHGroup2")]
        DhGroup2,
        #[serde(rename = "DHGroup14")]
        DhGroup14,
        #[serde(rename = "DHGroup2048")]
        DhGroup2048,
        #[serde(rename = "ECP256")]
        Ecp256,
        #[serde(rename = "ECP384")]
        Ecp384,
        #[serde(rename = "DHGroup24")]
        DhGroup24,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DhGroup {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DhGroup {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DhGroup {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("DhGroup", 0u32, "None"),
                Self::DhGroup1 => serializer.serialize_unit_variant("DhGroup", 1u32, "DHGroup1"),
                Self::DhGroup2 => serializer.serialize_unit_variant("DhGroup", 2u32, "DHGroup2"),
                Self::DhGroup14 => serializer.serialize_unit_variant("DhGroup", 3u32, "DHGroup14"),
                Self::DhGroup2048 => serializer.serialize_unit_variant("DhGroup", 4u32, "DHGroup2048"),
                Self::Ecp256 => serializer.serialize_unit_variant("DhGroup", 5u32, "ECP256"),
                Self::Ecp384 => serializer.serialize_unit_variant("DhGroup", 6u32, "ECP384"),
                Self::DhGroup24 => serializer.serialize_unit_variant("DhGroup", 7u32, "DHGroup24"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The Pfs Groups used in IKE Phase 2 for new child SA."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PfsGroup")]
    pub enum PfsGroup {
        None,
        #[serde(rename = "PFS1")]
        Pfs1,
        #[serde(rename = "PFS2")]
        Pfs2,
        #[serde(rename = "PFS2048")]
        Pfs2048,
        #[serde(rename = "ECP256")]
        Ecp256,
        #[serde(rename = "ECP384")]
        Ecp384,
        #[serde(rename = "PFS24")]
        Pfs24,
        #[serde(rename = "PFS14")]
        Pfs14,
        #[serde(rename = "PFSMM")]
        Pfsmm,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PfsGroup {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PfsGroup {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PfsGroup {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("PfsGroup", 0u32, "None"),
                Self::Pfs1 => serializer.serialize_unit_variant("PfsGroup", 1u32, "PFS1"),
                Self::Pfs2 => serializer.serialize_unit_variant("PfsGroup", 2u32, "PFS2"),
                Self::Pfs2048 => serializer.serialize_unit_variant("PfsGroup", 3u32, "PFS2048"),
                Self::Ecp256 => serializer.serialize_unit_variant("PfsGroup", 4u32, "ECP256"),
                Self::Ecp384 => serializer.serialize_unit_variant("PfsGroup", 5u32, "ECP384"),
                Self::Pfs24 => serializer.serialize_unit_variant("PfsGroup", 6u32, "PFS24"),
                Self::Pfs14 => serializer.serialize_unit_variant("PfsGroup", 7u32, "PFS14"),
                Self::Pfsmm => serializer.serialize_unit_variant("PfsGroup", 8u32, "PFSMM"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Vpn Client Parameters for package generation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VpnClientParameters {
    #[doc = "VPN client Processor Architecture. Possible values are: 'AMD64' and 'X86'."]
    #[serde(rename = "processorArchitecture", default, skip_serializing_if = "Option::is_none")]
    pub processor_architecture: Option<vpn_client_parameters::ProcessorArchitecture>,
    #[doc = "VPN client Authentication Method. Possible values are: 'EAPTLS' and 'EAPMSCHAPv2'."]
    #[serde(rename = "authenticationMethod", default, skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<vpn_client_parameters::AuthenticationMethod>,
    #[doc = "The public certificate data for the radius server authentication certificate as a Base-64 encoded string. Required only if external radius authentication has been configured with EAPTLS authentication."]
    #[serde(rename = "radiusServerAuthCertificate", default, skip_serializing_if = "Option::is_none")]
    pub radius_server_auth_certificate: Option<String>,
    #[doc = "A list of client root certificates public certificate data encoded as Base-64 strings. Optional parameter for external radius based authentication with EAPTLS."]
    #[serde(
        rename = "clientRootCertificates",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub client_root_certificates: Vec<String>,
}
impl VpnClientParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vpn_client_parameters {
    use super::*;
    #[doc = "VPN client Processor Architecture. Possible values are: 'AMD64' and 'X86'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProcessorArchitecture")]
    pub enum ProcessorArchitecture {
        Amd64,
        X86,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProcessorArchitecture {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProcessorArchitecture {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProcessorArchitecture {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Amd64 => serializer.serialize_unit_variant("ProcessorArchitecture", 0u32, "Amd64"),
                Self::X86 => serializer.serialize_unit_variant("ProcessorArchitecture", 1u32, "X86"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "VPN client Authentication Method. Possible values are: 'EAPTLS' and 'EAPMSCHAPv2'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthenticationMethod")]
    pub enum AuthenticationMethod {
        #[serde(rename = "EAPTLS")]
        Eaptls,
        #[serde(rename = "EAPMSCHAPv2")]
        EapmschaPv2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthenticationMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthenticationMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthenticationMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Eaptls => serializer.serialize_unit_variant("AuthenticationMethod", 0u32, "EAPTLS"),
                Self::EapmschaPv2 => serializer.serialize_unit_variant("AuthenticationMethod", 1u32, "EAPMSCHAPv2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "VPN client revoked certificate of virtual network gateway."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VpnClientRevokedCertificate {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of the revoked VPN client certificate of virtual network gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VpnClientRevokedCertificatePropertiesFormat>,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl VpnClientRevokedCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the revoked VPN client certificate of virtual network gateway."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VpnClientRevokedCertificatePropertiesFormat {
    #[doc = "The revoked VPN client certificate thumbprint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "The provisioning state of the VPN client revoked certificate resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl VpnClientRevokedCertificatePropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VPN client root certificate of virtual network gateway"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VpnClientRootCertificate {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Properties of SSL certificates of application gateway"]
    pub properties: VpnClientRootCertificatePropertiesFormat,
    #[doc = "The name of the resource that is unique within a resource group. This name can be used to access the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl VpnClientRootCertificate {
    pub fn new(properties: VpnClientRootCertificatePropertiesFormat) -> Self {
        Self {
            sub_resource: SubResource::default(),
            properties,
            name: None,
            etag: None,
        }
    }
}
#[doc = "Properties of SSL certificates of application gateway"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VpnClientRootCertificatePropertiesFormat {
    #[doc = "The certificate public data."]
    #[serde(rename = "publicCertData")]
    pub public_cert_data: String,
    #[doc = "The provisioning state of the VPN client root certificate resource. Possible values are: 'Updating', 'Deleting', and 'Failed'."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl VpnClientRootCertificatePropertiesFormat {
    pub fn new(public_cert_data: String) -> Self {
        Self {
            public_cert_data,
            provisioning_state: None,
        }
    }
}
#[doc = "Vpn device configuration script generation parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VpnDeviceScriptParameters {
    #[doc = "The vendor for the vpn device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[doc = "The device family for the vpn device."]
    #[serde(rename = "deviceFamily", default, skip_serializing_if = "Option::is_none")]
    pub device_family: Option<String>,
    #[doc = "The firmware version for the vpn device."]
    #[serde(rename = "firmwareVersion", default, skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
}
impl VpnDeviceScriptParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A reference to VirtualNetworkGateway or LocalNetworkGateway resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkConnectionGatewayReference {
    #[doc = "The ID of VirtualNetworkGateway or LocalNetworkGateway resource."]
    pub id: String,
}
impl VirtualNetworkConnectionGatewayReference {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

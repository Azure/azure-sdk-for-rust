#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An A record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ARecord {
    #[doc = "The IPv4 address of this A record."]
    #[serde(rename = "ipv4Address", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
}
impl ARecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An AAAA record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AaaaRecord {
    #[doc = "The IPv6 address of this AAAA record."]
    #[serde(rename = "ipv6Address", default, skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
}
impl AaaaRecord {
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
#[doc = "A CNAME record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CnameRecord {
    #[doc = "The canonical name for this CNAME record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
}
impl CnameRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An MX record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MxRecord {
    #[doc = "The preference value for this MX record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preference: Option<i32>,
    #[doc = "The domain name of the mail host for this MX record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
}
impl MxRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Private DNS zone."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateZone {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The ETag of the zone."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of the Private DNS zone."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateZoneProperties>,
}
impl PrivateZone {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a Private DNS zone list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateZoneListResult {
    #[doc = "Information about the Private DNS zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateZone>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateZoneListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateZoneListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of the Private DNS zone."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateZoneProperties {
    #[doc = "The maximum number of record sets that can be created in this Private DNS zone. This is a read-only property and any attempt to set this value will be ignored."]
    #[serde(rename = "maxNumberOfRecordSets", default, skip_serializing_if = "Option::is_none")]
    pub max_number_of_record_sets: Option<i64>,
    #[doc = "The current number of record sets in this Private DNS zone. This is a read-only property and any attempt to set this value will be ignored."]
    #[serde(rename = "numberOfRecordSets", default, skip_serializing_if = "Option::is_none")]
    pub number_of_record_sets: Option<i64>,
    #[doc = "The maximum number of virtual networks that can be linked to this Private DNS zone. This is a read-only property and any attempt to set this value will be ignored."]
    #[serde(rename = "maxNumberOfVirtualNetworkLinks", default, skip_serializing_if = "Option::is_none")]
    pub max_number_of_virtual_network_links: Option<i64>,
    #[doc = "The current number of virtual networks that are linked to this Private DNS zone. This is a read-only property and any attempt to set this value will be ignored."]
    #[serde(rename = "numberOfVirtualNetworkLinks", default, skip_serializing_if = "Option::is_none")]
    pub number_of_virtual_network_links: Option<i64>,
    #[doc = "The maximum number of virtual networks that can be linked to this Private DNS zone with registration enabled. This is a read-only property and any attempt to set this value will be ignored."]
    #[serde(
        rename = "maxNumberOfVirtualNetworkLinksWithRegistration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_number_of_virtual_network_links_with_registration: Option<i64>,
    #[doc = "The current number of virtual networks that are linked to this Private DNS zone with registration enabled. This is a read-only property and any attempt to set this value will be ignored."]
    #[serde(
        rename = "numberOfVirtualNetworkLinksWithRegistration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_virtual_network_links_with_registration: Option<i64>,
    #[doc = "The provisioning state of the resource. This is a read-only property and any attempt to set this value will be ignored."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_zone_properties::ProvisioningState>,
    #[doc = "Private zone internal Id"]
    #[serde(rename = "internalId", default, skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<String>,
}
impl PrivateZoneProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_zone_properties {
    use super::*;
    #[doc = "The provisioning state of the resource. This is a read-only property and any attempt to set this value will be ignored."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The resource model definition for an ARM proxy resource."]
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
#[doc = "A PTR record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PtrRecord {
    #[doc = "The PTR target domain name for this PTR record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ptrdname: Option<String>,
}
impl PtrRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a DNS record set (a collection of DNS records with the same name and type) in a Private DNS zone."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecordSet {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The ID of the record set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the record set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the record set."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The ETag of the record set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of the records in the record set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecordSetProperties>,
}
impl RecordSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a record set list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecordSetListResult {
    #[doc = "Information about the record sets in the response."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RecordSet>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RecordSetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RecordSetListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of the records in the record set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecordSetProperties {
    #[doc = "The metadata attached to the record set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The TTL (time-to-live) of the records in the record set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    #[doc = "Fully qualified domain name of the record set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Is the record set auto-registered in the Private DNS zone through a virtual network link?"]
    #[serde(rename = "isAutoRegistered", default, skip_serializing_if = "Option::is_none")]
    pub is_auto_registered: Option<bool>,
    #[doc = "The list of A records in the record set."]
    #[serde(rename = "aRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub a_records: Vec<ARecord>,
    #[doc = "The list of AAAA records in the record set."]
    #[serde(rename = "aaaaRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub aaaa_records: Vec<AaaaRecord>,
    #[doc = "A CNAME record."]
    #[serde(rename = "cnameRecord", default, skip_serializing_if = "Option::is_none")]
    pub cname_record: Option<CnameRecord>,
    #[doc = "The list of MX records in the record set."]
    #[serde(rename = "mxRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub mx_records: Vec<MxRecord>,
    #[doc = "The list of PTR records in the record set."]
    #[serde(rename = "ptrRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub ptr_records: Vec<PtrRecord>,
    #[doc = "An SOA record."]
    #[serde(rename = "soaRecord", default, skip_serializing_if = "Option::is_none")]
    pub soa_record: Option<SoaRecord>,
    #[doc = "The list of SRV records in the record set."]
    #[serde(rename = "srvRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub srv_records: Vec<SrvRecord>,
    #[doc = "The list of TXT records in the record set."]
    #[serde(rename = "txtRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub txt_records: Vec<TxtRecord>,
}
impl RecordSetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The core properties of ARM resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource Id for the resource. Example - '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/privateDnsZones/{privateDnsZoneName}'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. Example - 'Microsoft.Network/privateDnsZones'."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An SOA record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SoaRecord {
    #[doc = "The domain name of the authoritative name server for this SOA record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "The email contact for this SOA record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The serial number for this SOA record."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<i64>,
    #[doc = "The refresh value for this SOA record."]
    #[serde(rename = "refreshTime", default, skip_serializing_if = "Option::is_none")]
    pub refresh_time: Option<i64>,
    #[doc = "The retry time for this SOA record."]
    #[serde(rename = "retryTime", default, skip_serializing_if = "Option::is_none")]
    pub retry_time: Option<i64>,
    #[doc = "The expire time for this SOA record."]
    #[serde(rename = "expireTime", default, skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    #[doc = "The minimum value for this SOA record. By convention this is used to determine the negative caching duration."]
    #[serde(rename = "minimumTtl", default, skip_serializing_if = "Option::is_none")]
    pub minimum_ttl: Option<i64>,
}
impl SoaRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An SRV record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SrvRecord {
    #[doc = "The priority value for this SRV record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "The weight value for this SRV record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[doc = "The port value for this SRV record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "The target domain name for this SRV record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl SrvRecord {
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
#[doc = "The resource model definition for a ARM tracked top level resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The Azure Region where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A TXT record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TxtRecord {
    #[doc = "The text value of this TXT record."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}
impl TxtRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a link to virtual network for a Private DNS zone."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkLink {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The ETag of the virtual network link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of the Private DNS zone."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkLinkProperties>,
}
impl VirtualNetworkLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list virtual network link to Private DNS zone operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkLinkListResult {
    #[doc = "Information about the virtual network links to the Private DNS zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetworkLink>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkLinkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualNetworkLinkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of the Private DNS zone."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkLinkProperties {
    #[doc = "Reference to another subresource."]
    #[serde(rename = "virtualNetwork", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network: Option<SubResource>,
    #[doc = "Is auto-registration of virtual machine records in the virtual network in the Private DNS zone enabled?"]
    #[serde(rename = "registrationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub registration_enabled: Option<bool>,
    #[doc = "The status of the virtual network link to the Private DNS zone. Possible values are 'InProgress' and 'Done'. This is a read-only property and any attempt to set this value will be ignored."]
    #[serde(rename = "virtualNetworkLinkState", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_link_state: Option<virtual_network_link_properties::VirtualNetworkLinkState>,
    #[doc = "The provisioning state of the resource. This is a read-only property and any attempt to set this value will be ignored."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<virtual_network_link_properties::ProvisioningState>,
}
impl VirtualNetworkLinkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_network_link_properties {
    use super::*;
    #[doc = "The status of the virtual network link to the Private DNS zone. Possible values are 'InProgress' and 'Done'. This is a read-only property and any attempt to set this value will be ignored."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VirtualNetworkLinkState")]
    pub enum VirtualNetworkLinkState {
        InProgress,
        Completed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VirtualNetworkLinkState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VirtualNetworkLinkState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VirtualNetworkLinkState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::InProgress => serializer.serialize_unit_variant("VirtualNetworkLinkState", 0u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("VirtualNetworkLinkState", 1u32, "Completed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the resource. This is a read-only property and any attempt to set this value will be ignored."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}

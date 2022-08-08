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
#[doc = "A CAA record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CaaRecord {
    #[doc = "The flags for this CAA record as an integer between 0 and 255."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    #[doc = "The tag for this CAA record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "The value for this CAA record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl CaaRecord {
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
#[doc = "An NS record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NsRecord {
    #[doc = "The name server name for this NS record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsdname: Option<String>,
}
impl NsRecord {
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
#[doc = "Describes a DNS record set (a collection of DNS records with the same name and type)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecordSet {
    #[doc = "The ID of the record set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the record set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the record set."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The etag of the record set."]
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
#[doc = "The response to a record set List operation."]
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
    #[serde(rename = "TTL", default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    #[doc = "Fully qualified domain name of the record set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "The list of A records in the record set."]
    #[serde(rename = "ARecords", default, skip_serializing_if = "Vec::is_empty")]
    pub a_records: Vec<ARecord>,
    #[doc = "The list of AAAA records in the record set."]
    #[serde(rename = "AAAARecords", default, skip_serializing_if = "Vec::is_empty")]
    pub aaaa_records: Vec<AaaaRecord>,
    #[doc = "The list of MX records in the record set."]
    #[serde(rename = "MXRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub mx_records: Vec<MxRecord>,
    #[doc = "The list of NS records in the record set."]
    #[serde(rename = "NSRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub ns_records: Vec<NsRecord>,
    #[doc = "The list of PTR records in the record set."]
    #[serde(rename = "PTRRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub ptr_records: Vec<PtrRecord>,
    #[doc = "The list of SRV records in the record set."]
    #[serde(rename = "SRVRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub srv_records: Vec<SrvRecord>,
    #[doc = "The list of TXT records in the record set."]
    #[serde(rename = "TXTRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub txt_records: Vec<TxtRecord>,
    #[doc = "A CNAME record."]
    #[serde(rename = "CNAMERecord", default, skip_serializing_if = "Option::is_none")]
    pub cname_record: Option<CnameRecord>,
    #[doc = "An SOA record."]
    #[serde(rename = "SOARecord", default, skip_serializing_if = "Option::is_none")]
    pub soa_record: Option<SoaRecord>,
    #[doc = "The list of CAA records in the record set."]
    #[serde(rename = "caaRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub caa_records: Vec<CaaRecord>,
}
impl RecordSetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to update a record set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecordSetUpdateParameters {
    #[doc = "Describes a DNS record set (a collection of DNS records with the same name and type)."]
    #[serde(rename = "RecordSet", default, skip_serializing_if = "Option::is_none")]
    pub record_set: Option<RecordSet>,
}
impl RecordSetUpdateParameters {
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
    #[serde(rename = "minimumTTL", default, skip_serializing_if = "Option::is_none")]
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
#[doc = "A reference to a another resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
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
#[doc = "Describes a DNS zone."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Zone {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The etag of the zone."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of the zone."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ZoneProperties>,
}
impl Zone {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties: None,
        }
    }
}
#[doc = "The response to a Zone List or ListAll operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ZoneListResult {
    #[doc = "Information about the DNS zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Zone>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ZoneListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ZoneListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of the zone."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ZoneProperties {
    #[doc = "The maximum number of record sets that can be created in this DNS zone.  This is a read-only property and any attempt to set this value will be ignored."]
    #[serde(rename = "maxNumberOfRecordSets", default, skip_serializing_if = "Option::is_none")]
    pub max_number_of_record_sets: Option<i64>,
    #[doc = "The maximum number of records per record set that can be created in this DNS zone.  This is a read-only property and any attempt to set this value will be ignored."]
    #[serde(rename = "maxNumberOfRecordsPerRecordSet", default, skip_serializing_if = "Option::is_none")]
    pub max_number_of_records_per_record_set: Option<i64>,
    #[doc = "The current number of record sets in this DNS zone.  This is a read-only property and any attempt to set this value will be ignored."]
    #[serde(rename = "numberOfRecordSets", default, skip_serializing_if = "Option::is_none")]
    pub number_of_record_sets: Option<i64>,
    #[doc = "The name servers for this DNS zone. This is a read-only property and any attempt to set this value will be ignored."]
    #[serde(rename = "nameServers", default, skip_serializing_if = "Vec::is_empty")]
    pub name_servers: Vec<String>,
    #[doc = "The type of this DNS zone (Public or Private)."]
    #[serde(rename = "zoneType", default, skip_serializing_if = "Option::is_none")]
    pub zone_type: Option<zone_properties::ZoneType>,
    #[doc = "A list of references to virtual networks that register hostnames in this DNS zone. This is a only when ZoneType is Private."]
    #[serde(rename = "registrationVirtualNetworks", default, skip_serializing_if = "Vec::is_empty")]
    pub registration_virtual_networks: Vec<SubResource>,
    #[doc = "A list of references to virtual networks that resolve records in this DNS zone. This is a only when ZoneType is Private."]
    #[serde(rename = "resolutionVirtualNetworks", default, skip_serializing_if = "Vec::is_empty")]
    pub resolution_virtual_networks: Vec<SubResource>,
}
impl ZoneProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod zone_properties {
    use super::*;
    #[doc = "The type of this DNS zone (Public or Private)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ZoneType {
        Public,
        Private,
    }
    impl Default for ZoneType {
        fn default() -> Self {
            Self::Public
        }
    }
}
#[doc = "Describes a request to update a DNS zone."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ZoneUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ZoneUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}

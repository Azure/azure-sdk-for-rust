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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "The delegation signer information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DelegationSignerInfo {
    #[doc = "The digest algorithm type represents the standard digest algorithm number used to construct the digest. See: https://www.iana.org/assignments/ds-rr-types/ds-rr-types.xhtml"]
    #[serde(rename = "digestAlgorithmType", default, skip_serializing_if = "Option::is_none")]
    pub digest_algorithm_type: Option<DigestAlgorithmType>,
    #[doc = "The digest value is a cryptographic hash value of the referenced DNSKEY Resource Record."]
    #[serde(rename = "digestValue", default, skip_serializing_if = "Option::is_none")]
    pub digest_value: Option<String>,
    #[doc = "The record represents a delegation signer (DS) record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
}
impl DelegationSignerInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A digest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Digest {
    #[doc = "The digest algorithm type represents the standard digest algorithm number used to construct the digest. See: https://www.iana.org/assignments/ds-rr-types/ds-rr-types.xhtml"]
    #[serde(rename = "algorithmType", default, skip_serializing_if = "Option::is_none")]
    pub algorithm_type: Option<DigestAlgorithmType>,
    #[doc = "The digest value is a cryptographic hash value of the referenced DNSKEY Resource Record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Digest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type DigestAlgorithmType = i32;
#[doc = "Represents a single Azure resource and its referencing DNS records."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsResourceReference {
    #[doc = "A list of dns Records "]
    #[serde(
        rename = "dnsResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dns_resources: Vec<SubResource>,
    #[doc = "A reference to a another resource"]
    #[serde(rename = "targetResource", default, skip_serializing_if = "Option::is_none")]
    pub target_resource: Option<SubResource>,
}
impl DnsResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of the Dns Resource Reference Request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsResourceReferenceRequest {
    #[doc = "Represents the properties of the Dns Resource Reference Request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DnsResourceReferenceRequestProperties>,
}
impl DnsResourceReferenceRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of the Dns Resource Reference Request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsResourceReferenceRequestProperties {
    #[doc = "A list of references to azure resources for which referencing dns records need to be queried."]
    #[serde(
        rename = "targetResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub target_resources: Vec<SubResource>,
}
impl DnsResourceReferenceRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of the Dns Resource Reference Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsResourceReferenceResult {
    #[doc = "The result of dns resource reference request. Returns a list of dns resource references for each of the azure resource in the request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DnsResourceReferenceResultProperties>,
}
impl DnsResourceReferenceResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of dns resource reference request. Returns a list of dns resource references for each of the azure resource in the request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsResourceReferenceResultProperties {
    #[doc = "The result of dns resource reference request. A list of dns resource references for each of the azure resource in the request"]
    #[serde(
        rename = "dnsResourceReferences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dns_resource_references: Vec<DnsResourceReference>,
}
impl DnsResourceReferenceResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the DNSSEC configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnssecConfig {
    #[doc = "Represents the DNSSEC properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DnssecProperties>,
    #[doc = "The ID of the DNSSEC configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the DNSSEC configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the DNSSEC configuration."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The etag of the DNSSEC configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DnssecConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a List DNSSEC configurations operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnssecConfigListResult {
    #[doc = "Information about the DNSSEC configurations in the response."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DnssecConfig>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DnssecConfigListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DnssecConfigListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the DNSSEC properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnssecProperties {
    #[doc = "Provisioning State of the DNSSEC configuration."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The list of signing keys."]
    #[serde(
        rename = "signingKeys",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub signing_keys: Vec<SigningKey>,
}
impl DnssecProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A DS record. For more information about the DS record format, see RFC 4034: https://www.rfc-editor.org/rfc/rfc4034"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DsRecord {
    #[doc = "The key tag value is used to determine which DNSKEY Resource Record is used for signature verification."]
    #[serde(rename = "keyTag", default, skip_serializing_if = "Option::is_none")]
    pub key_tag: Option<i32>,
    #[doc = "The security algorithm type represents the standard security algorithm number of the DNSKEY Resource Record. See: https://www.iana.org/assignments/dns-sec-alg-numbers/dns-sec-alg-numbers.xhtml"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<SecurityAlgorithmType>,
    #[doc = "A digest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<Digest>,
}
impl DsRecord {
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
#[doc = "A NAPTR record. For more information about the NAPTR record format, see RFC 3403: https://www.rfc-editor.org/rfc/rfc3403"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NaptrRecord {
    #[doc = "The order in which the NAPTR records MUST be processed in order to accurately represent the ordered list of rules. The ordering is from lowest to highest. Valid values: 0-65535."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "The preference specifies the order in which NAPTR records with equal 'order' values should be processed, low numbers being processed before high numbers. Valid values: 0-65535."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preference: Option<i32>,
    #[doc = "The flags specific to DDDS applications. Values currently defined in RFC 3404 are uppercase and lowercase letters \"A\", \"P\", \"S\", and \"U\", and the empty string, \"\". Enclose Flags in quotation marks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    #[doc = "The services specific to DDDS applications. Enclose Services in quotation marks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<String>,
    #[doc = "The regular expression that the DDDS application uses to convert an input value into an output value. For example: an IP phone system might use a regular expression to convert a phone number that is entered by a user into a SIP URI. Enclose the regular expression in quotation marks. Specify either a value for 'regexp' or a value for 'replacement'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<String>,
    #[doc = "The replacement is a fully qualified domain name (FQDN) of the next domain name that you want the DDDS application to submit a DNS query for. The DDDS application replaces the input value with the value specified for replacement. Specify either a value for 'regexp' or a value for 'replacement'. If you specify a value for 'regexp', specify a dot (.) for 'replacement'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
}
impl NaptrRecord {
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RecordSet>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RecordSetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
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
    #[doc = "provisioning State of the record set."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "A reference to a another resource"]
    #[serde(rename = "targetResource", default, skip_serializing_if = "Option::is_none")]
    pub target_resource: Option<SubResource>,
    #[doc = "The list of A records in the record set."]
    #[serde(
        rename = "ARecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub a_records: Vec<ARecord>,
    #[doc = "The list of AAAA records in the record set."]
    #[serde(
        rename = "AAAARecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub aaaa_records: Vec<AaaaRecord>,
    #[doc = "The list of MX records in the record set."]
    #[serde(
        rename = "MXRecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mx_records: Vec<MxRecord>,
    #[doc = "The list of NS records in the record set."]
    #[serde(
        rename = "NSRecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ns_records: Vec<NsRecord>,
    #[doc = "The list of PTR records in the record set."]
    #[serde(
        rename = "PTRRecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ptr_records: Vec<PtrRecord>,
    #[doc = "The list of SRV records in the record set."]
    #[serde(
        rename = "SRVRecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub srv_records: Vec<SrvRecord>,
    #[doc = "The list of TXT records in the record set."]
    #[serde(
        rename = "TXTRecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub txt_records: Vec<TxtRecord>,
    #[doc = "A CNAME record."]
    #[serde(rename = "CNAMERecord", default, skip_serializing_if = "Option::is_none")]
    pub cname_record: Option<CnameRecord>,
    #[doc = "An SOA record."]
    #[serde(rename = "SOARecord", default, skip_serializing_if = "Option::is_none")]
    pub soa_record: Option<SoaRecord>,
    #[doc = "The list of CAA records in the record set."]
    #[serde(
        rename = "caaRecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub caa_records: Vec<CaaRecord>,
    #[doc = "The list of DS records in the record set."]
    #[serde(
        rename = "DSRecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ds_records: Vec<DsRecord>,
    #[doc = "The list of TLSA records in the record set."]
    #[serde(
        rename = "TLSARecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tlsa_records: Vec<TlsaRecord>,
    #[doc = "The list of NAPTR records in the record set."]
    #[serde(
        rename = "NAPTRRecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub naptr_records: Vec<NaptrRecord>,
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
#[doc = "Common properties of an Azure Resource Manager resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub location: String,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
pub type SecurityAlgorithmType = i32;
#[doc = "Represents the signing key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SigningKey {
    #[doc = "The delegation signer information."]
    #[serde(
        rename = "delegationSignerInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub delegation_signer_info: Vec<DelegationSignerInfo>,
    #[doc = "The flags specifies how the key is used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    #[doc = "The key tag value of the DNSKEY Resource Record."]
    #[serde(rename = "keyTag", default, skip_serializing_if = "Option::is_none")]
    pub key_tag: Option<i32>,
    #[doc = "The protocol value. The value is always 3."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<i32>,
    #[doc = "The public key, represented as a Base64 encoding."]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[doc = "The security algorithm type represents the standard security algorithm number of the DNSKEY Resource Record. See: https://www.iana.org/assignments/dns-sec-alg-numbers/dns-sec-alg-numbers.xhtml"]
    #[serde(rename = "securityAlgorithmType", default, skip_serializing_if = "Option::is_none")]
    pub security_algorithm_type: Option<SecurityAlgorithmType>,
}
impl SigningKey {
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
#[doc = "A TLSA record. For more information about the TLSA record format, see RFC 6698: https://www.rfc-editor.org/rfc/rfc6698"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TlsaRecord {
    #[doc = "The usage specifies the provided association that will be used to match the certificate presented in the TLS handshake."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<i32>,
    #[doc = "The selector specifies which part of the TLS certificate presented by the server will be matched against the association data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<i32>,
    #[doc = "The matching type specifies how the certificate association is presented."]
    #[serde(rename = "matchingType", default, skip_serializing_if = "Option::is_none")]
    pub matching_type: Option<i32>,
    #[doc = "This specifies the certificate association data to be matched."]
    #[serde(rename = "certAssociationData", default, skip_serializing_if = "Option::is_none")]
    pub cert_association_data: Option<String>,
}
impl TlsaRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A TXT record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TxtRecord {
    #[doc = "The text value of this TXT record."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    pub resource: Resource,
    #[doc = "The etag of the zone."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of the zone."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ZoneProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Zone {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            etag: None,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "The response to a Zone List or ListAll operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ZoneListResult {
    #[doc = "Information about the DNS zones."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Zone>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ZoneListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
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
    #[serde(
        rename = "nameServers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub name_servers: Vec<String>,
    #[doc = "The type of this DNS zone (Public or Private)."]
    #[serde(rename = "zoneType", default, skip_serializing_if = "Option::is_none")]
    pub zone_type: Option<zone_properties::ZoneType>,
    #[doc = "A list of references to virtual networks that register hostnames in this DNS zone. This is a only when ZoneType is Private."]
    #[serde(
        rename = "registrationVirtualNetworks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub registration_virtual_networks: Vec<SubResource>,
    #[doc = "A list of references to virtual networks that resolve records in this DNS zone. This is a only when ZoneType is Private."]
    #[serde(
        rename = "resolutionVirtualNetworks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resolution_virtual_networks: Vec<SubResource>,
    #[doc = "The list of signing keys."]
    #[serde(
        rename = "signingKeys",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub signing_keys: Vec<SigningKey>,
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

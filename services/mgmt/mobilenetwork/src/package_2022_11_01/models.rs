#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
pub type N5Qi = i32;
pub type N5QiPriorityLevel = i32;
pub type N5QiPriorityLevelRm = i32;
pub type N5QiRm = i32;
#[doc = "Aggregate maximum bit rate."]
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
#[doc = "Aggregate maximum bit rate."]
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
#[doc = "Reference to an Azure Async Operation ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AsyncOperationId {
    #[doc = "Azure Async Operation ID."]
    pub id: String,
}
impl AsyncOperationId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "The current status of an async operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AsyncOperationStatus {
    #[doc = "Fully qualified ID for the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The operation status."]
    pub status: String,
    #[doc = "Fully qualified ID for the resource that this async operation status relates to."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Percentage of the operation that is complete."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "Properties returned by the resource provider on a successful operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl AsyncOperationStatus {
    pub fn new(status: String) -> Self {
        Self {
            id: None,
            name: None,
            status,
            resource_id: None,
            start_time: None,
            end_time: None,
            percent_complete: None,
            properties: None,
            error: None,
        }
    }
}
#[doc = "Attached data network resource. Must be created in the same location as its parent packet core data plane."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachedDataNetwork {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Data network properties."]
    pub properties: AttachedDataNetworkPropertiesFormat,
}
impl AttachedDataNetwork {
    pub fn new(tracked_resource: TrackedResource, properties: AttachedDataNetworkPropertiesFormat) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "Response for attached data network API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachedDataNetworkListResult {
    #[doc = "A list of data networks in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[doc = "The DNS servers to signal to UEs to use for this attached data network. This configuration is mandatory - if you don't want DNS servers, you must provide an empty array."]
    #[serde(rename = "dnsAddresses")]
    pub dns_addresses: Vec<Ipv4Addr>,
    #[doc = "The network address and port translation settings to use for the attached data network."]
    #[serde(rename = "naptConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub napt_configuration: Option<NaptConfiguration>,
    #[doc = "The user equipment (UE) address pool prefixes for the attached data network from which the packet core instance will dynamically assign IP addresses to UEs.\nThe packet core instance assigns an IP address to a UE when the UE sets up a PDU session.\n You must define at least one of userEquipmentAddressPoolPrefix and userEquipmentStaticAddressPoolPrefix. If you define both, they must be of the same size."]
    #[serde(
        rename = "userEquipmentAddressPoolPrefix",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_equipment_address_pool_prefix: Vec<Ipv4AddrMask>,
    #[doc = "The user equipment (UE) address pool prefixes for the attached data network from which the packet core instance will assign static IP addresses to UEs.\nThe packet core instance assigns an IP address to a UE when the UE sets up a PDU session. The static IP address for a specific UE is set in StaticIPConfiguration on the corresponding SIM resource.\nAt least one of userEquipmentAddressPoolPrefix and userEquipmentStaticAddressPoolPrefix must be defined. If both are defined, they must be of the same size."]
    #[serde(
        rename = "userEquipmentStaticAddressPoolPrefix",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_equipment_static_address_pool_prefix: Vec<Ipv4AddrMask>,
}
impl AttachedDataNetworkPropertiesFormat {
    pub fn new(user_plane_data_interface: InterfaceProperties, dns_addresses: Vec<Ipv4Addr>) -> Self {
        Self {
            provisioning_state: None,
            user_plane_data_interface,
            dns_addresses,
            napt_configuration: None,
            user_equipment_address_pool_prefix: Vec::new(),
            user_equipment_static_address_pool_prefix: Vec::new(),
        }
    }
}
#[doc = "Reference to an attached data network resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachedDataNetworkResourceId {
    #[doc = "Attached data network resource ID."]
    pub id: String,
}
impl AttachedDataNetworkResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Reference to an Azure Stack Edge device resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStackEdgeDeviceResourceId {
    #[doc = "Azure Stack Edge device resource ID."]
    pub id: String,
}
impl AzureStackEdgeDeviceResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Reference to an Azure Stack HCI cluster resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStackHciClusterResourceId {
    #[doc = "Azure Stack HCI cluster resource ID."]
    pub id: String,
}
impl AzureStackHciClusterResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "The SKU of the packet core control plane resource. The SKU list may change over time when a new SKU gets added or an exiting SKU gets removed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BillingSku")]
pub enum BillingSku {
    G0,
    G1,
    G2,
    G5,
    G10,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BillingSku {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BillingSku {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BillingSku {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::G0 => serializer.serialize_unit_variant("BillingSku", 0u32, "G0"),
            Self::G1 => serializer.serialize_unit_variant("BillingSku", 1u32, "G1"),
            Self::G2 => serializer.serialize_unit_variant("BillingSku", 2u32, "G2"),
            Self::G5 => serializer.serialize_unit_variant("BillingSku", 3u32, "G5"),
            Self::G10 => serializer.serialize_unit_variant("BillingSku", 4u32, "G10"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type BitRate = String;
pub type BitRateRm = String;
#[doc = "Certificate provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateProvisioning {
    #[doc = "The certificate's provisioning state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<certificate_provisioning::State>,
    #[doc = "Reason for certificate provisioning failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl CertificateProvisioning {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod certificate_provisioning {
    use super::*;
    #[doc = "The certificate's provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        NotProvisioned,
        Provisioned,
        Failed,
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
                Self::NotProvisioned => serializer.serialize_unit_variant("State", 0u32, "NotProvisioned"),
                Self::Provisioned => serializer.serialize_unit_variant("State", 1u32, "Provisioned"),
                Self::Failed => serializer.serialize_unit_variant("State", 2u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Common SIM properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommonSimPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The state of the SIM resource."]
    #[serde(rename = "simState", default, skip_serializing_if = "Option::is_none")]
    pub sim_state: Option<SimState>,
    #[doc = "The provisioning state of a resource e.g. SIM/SIM policy on a site. The dictionary keys will ARM resource IDs in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.MobileNetwork/mobileNetworks/{mobileNetworkName}/sites/{siteName}. The dictionary values will be from the \"SiteProvisioningState\" enum."]
    #[serde(rename = "siteProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub site_provisioning_state: Option<SiteProvisioning>,
    #[doc = "The international mobile subscriber identity (IMSI) for the SIM."]
    #[serde(rename = "internationalMobileSubscriberIdentity")]
    pub international_mobile_subscriber_identity: String,
    #[doc = "The integrated circuit card ID (ICCID) for the SIM."]
    #[serde(rename = "integratedCircuitCardIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub integrated_circuit_card_identifier: Option<String>,
    #[doc = "An optional free-form text field that can be used to record the device type this SIM is associated with, for example 'Video camera'. The Azure portal allows SIMs to be grouped and filtered based on this value."]
    #[serde(rename = "deviceType", default, skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[doc = "Reference to a SIM policy resource."]
    #[serde(rename = "simPolicy", default, skip_serializing_if = "Option::is_none")]
    pub sim_policy: Option<SimPolicyResourceId>,
    #[doc = "A list of static IP addresses assigned to this SIM. Each address is assigned at a defined network scope, made up of {attached data network, slice}."]
    #[serde(
        rename = "staticIpConfiguration",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub static_ip_configuration: Vec<SimStaticIpProperties>,
    #[doc = "The name of the SIM vendor who provided this SIM, if any."]
    #[serde(rename = "vendorName", default, skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    #[doc = "The public key fingerprint of the SIM vendor who provided this SIM, if any."]
    #[serde(rename = "vendorKeyFingerprint", default, skip_serializing_if = "Option::is_none")]
    pub vendor_key_fingerprint: Option<String>,
}
impl CommonSimPropertiesFormat {
    pub fn new(international_mobile_subscriber_identity: String) -> Self {
        Self {
            provisioning_state: None,
            sim_state: None,
            site_provisioning_state: None,
            international_mobile_subscriber_identity,
            integrated_circuit_card_identifier: None,
            device_type: None,
            sim_policy: None,
            static_ip_configuration: Vec::new(),
            vendor_name: None,
            vendor_key_fingerprint: None,
        }
    }
}
#[doc = "Reference to an Azure Arc custom location resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectedClusterResourceId {
    #[doc = "Azure Arc connected cluster resource ID."]
    pub id: String,
}
impl ConnectedClusterResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "The core network technology generation (5G core or EPC / 4G core)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CoreNetworkType {
    #[serde(rename = "5GC")]
    N5GC,
    #[serde(rename = "EPC")]
    Epc,
}
impl Default for CoreNetworkType {
    fn default() -> Self {
        Self::N5GC
    }
}
#[doc = "The core network technology generation (5G core or EPC / 4G core)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CoreNetworkTypeRm {
    #[serde(rename = "5GC")]
    N5GC,
    #[serde(rename = "EPC")]
    Epc,
}
impl Default for CoreNetworkTypeRm {
    fn default() -> Self {
        Self::N5GC
    }
}
#[doc = "Reference to an Azure Arc custom location resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomLocationResourceId {
    #[doc = "Azure Arc custom location resource ID."]
    pub id: String,
}
impl CustomLocationResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Data network resource. Must be created in the same location as its parent mobile network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataNetwork {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Data network properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataNetworkPropertiesFormat>,
}
impl DataNetwork {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Settings controlling data network use"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataNetworkConfiguration {
    #[doc = "Reference to a data network resource."]
    #[serde(rename = "dataNetwork")]
    pub data_network: DataNetworkResourceId,
    #[doc = "Aggregate maximum bit rate."]
    #[serde(rename = "sessionAmbr")]
    pub session_ambr: Ambr,
    #[doc = "5G QoS Identifier."]
    #[serde(rename = "5qi", default, skip_serializing_if = "Option::is_none")]
    pub n5qi: Option<N5Qi>,
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
    #[doc = "Allowed session types in addition to the default session type. Must not duplicate the default session type."]
    #[serde(
        rename = "additionalAllowedSessionTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_allowed_session_types: Vec<PduSessionType>,
    #[doc = "List of services that can be used as part of this SIM policy. The list must not contain duplicate items and must contain at least one item. The services must be in the same location as the SIM policy."]
    #[serde(rename = "allowedServices")]
    pub allowed_services: Vec<ServiceResourceId>,
    #[doc = "The maximum number of downlink packets to buffer at the user plane for High Latency Communication - Extended Buffering. See 3GPP TS29.272 v15.10.0 section 7.3.188 for a full description. This maximum is not guaranteed because there is a internal limit on buffered packets across all PDU sessions."]
    #[serde(rename = "maximumNumberOfBufferedPackets", default, skip_serializing_if = "Option::is_none")]
    pub maximum_number_of_buffered_packets: Option<i32>,
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
            maximum_number_of_buffered_packets: None,
        }
    }
}
#[doc = "Response for data network API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataNetworkListResult {
    #[doc = "A list of data networks."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "Reference to a data network resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataNetworkResourceId {
    #[doc = "Data network resource ID."]
    pub id: String,
}
impl DataNetworkResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Encrypted SIM properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptedSimPropertiesFormat {
    #[serde(flatten)]
    pub common_sim_properties_format: CommonSimPropertiesFormat,
    #[doc = "The encrypted SIM credentials."]
    #[serde(rename = "encryptedCredentials", default, skip_serializing_if = "Option::is_none")]
    pub encrypted_credentials: Option<String>,
}
impl EncryptedSimPropertiesFormat {
    pub fn new(common_sim_properties_format: CommonSimPropertiesFormat) -> Self {
        Self {
            common_sim_properties_format,
            encrypted_credentials: None,
        }
    }
}
#[doc = "The SIMs to upload. The SIM credentials must be encrypted."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptedSimUploadList {
    #[doc = "The upload file format version."]
    pub version: i32,
    #[doc = "An identifier for the Azure SIM onboarding public key used for encrypted upload."]
    #[serde(rename = "azureKeyIdentifier")]
    pub azure_key_identifier: i32,
    #[doc = "The fingerprint of the SIM vendor public key. The private counterpart is used for signing the encrypted transport key."]
    #[serde(rename = "vendorKeyFingerprint")]
    pub vendor_key_fingerprint: String,
    #[doc = "The transport key used for encrypting SIM credentials, encrypted using the SIM onboarding public key."]
    #[serde(rename = "encryptedTransportKey")]
    pub encrypted_transport_key: String,
    #[doc = "The encrypted transport key, signed using the SIM vendor private key."]
    #[serde(rename = "signedTransportKey")]
    pub signed_transport_key: String,
    #[doc = "A list of SIMs to upload, with encrypted properties."]
    pub sims: Vec<SimNameAndEncryptedProperties>,
}
impl EncryptedSimUploadList {
    pub fn new(
        version: i32,
        azure_key_identifier: i32,
        vendor_key_fingerprint: String,
        encrypted_transport_key: String,
        signed_transport_key: String,
        sims: Vec<SimNameAndEncryptedProperties>,
    ) -> Self {
        Self {
            version,
            azure_key_identifier,
            vendor_key_fingerprint,
            encrypted_transport_key,
            signed_transport_key,
            sims,
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
#[doc = "HTTPS server certificate configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpsServerCertificate {
    #[doc = "The certificate URL, unversioned. For example: https://contosovault.vault.azure.net/certificates/ingress."]
    #[serde(rename = "certificateUrl")]
    pub certificate_url: String,
    #[doc = "Certificate provisioning state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioning: Option<CertificateProvisioning>,
}
impl HttpsServerCertificate {
    pub fn new(certificate_url: String) -> Self {
        Self {
            certificate_url,
            provisioning: None,
        }
    }
}
#[doc = "The installation state of the packet core."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Installation {
    #[doc = "The installation state of the packet core."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<InstallationState>,
    #[doc = "Reference to an Azure Async Operation ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<AsyncOperationId>,
}
impl Installation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The installation state of the packet core."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InstallationState")]
pub enum InstallationState {
    Uninstalled,
    Installing,
    Installed,
    Updating,
    Upgrading,
    Uninstalling,
    Reinstalling,
    RollingBack,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InstallationState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InstallationState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InstallationState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Uninstalled => serializer.serialize_unit_variant("InstallationState", 0u32, "Uninstalled"),
            Self::Installing => serializer.serialize_unit_variant("InstallationState", 1u32, "Installing"),
            Self::Installed => serializer.serialize_unit_variant("InstallationState", 2u32, "Installed"),
            Self::Updating => serializer.serialize_unit_variant("InstallationState", 3u32, "Updating"),
            Self::Upgrading => serializer.serialize_unit_variant("InstallationState", 4u32, "Upgrading"),
            Self::Uninstalling => serializer.serialize_unit_variant("InstallationState", 5u32, "Uninstalling"),
            Self::Reinstalling => serializer.serialize_unit_variant("InstallationState", 6u32, "Reinstalling"),
            Self::RollingBack => serializer.serialize_unit_variant("InstallationState", 7u32, "RollingBack"),
            Self::Failed => serializer.serialize_unit_variant("InstallationState", 8u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Interface properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InterfaceProperties {
    #[doc = "The logical name for this interface. This should match one of the interfaces configured on your Azure Stack Edge device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Ipv4Addr = String;
pub type Ipv4AddrMask = String;
pub type Ipv4AddrMaskRm = String;
pub type Ipv4AddrRm = String;
#[doc = "An Azure key vault key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultKey {
    #[doc = "The key URL, unversioned. For example: https://contosovault.vault.azure.net/keys/azureKey."]
    #[serde(rename = "keyUrl", default, skip_serializing_if = "Option::is_none")]
    pub key_url: Option<String>,
}
impl KeyVaultKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kubernetes ingress configuration to control access to packet core diagnostics over local APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalDiagnosticsAccessConfiguration {
    #[doc = "How to authenticate users who access local diagnostics APIs."]
    #[serde(rename = "authenticationType")]
    pub authentication_type: local_diagnostics_access_configuration::AuthenticationType,
    #[doc = "HTTPS server certificate configuration."]
    #[serde(rename = "httpsServerCertificate", default, skip_serializing_if = "Option::is_none")]
    pub https_server_certificate: Option<HttpsServerCertificate>,
}
impl LocalDiagnosticsAccessConfiguration {
    pub fn new(authentication_type: local_diagnostics_access_configuration::AuthenticationType) -> Self {
        Self {
            authentication_type,
            https_server_certificate: None,
        }
    }
}
pub mod local_diagnostics_access_configuration {
    use super::*;
    #[doc = "How to authenticate users who access local diagnostics APIs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthenticationType")]
    pub enum AuthenticationType {
        #[serde(rename = "AAD")]
        Aad,
        Password,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthenticationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthenticationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthenticationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Aad => serializer.serialize_unit_variant("AuthenticationType", 0u32, "AAD"),
                Self::Password => serializer.serialize_unit_variant("AuthenticationType", 1u32, "Password"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: ManagedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedServiceIdentityType")]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned,UserAssigned")]
    SystemAssignedUserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 2u32, "UserAssigned"),
            Self::SystemAssignedUserAssigned => {
                serializer.serialize_unit_variant("ManagedServiceIdentityType", 3u32, "SystemAssigned,UserAssigned")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
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
}
impl MobileNetwork {
    pub fn new(tracked_resource: TrackedResource, properties: MobileNetworkPropertiesFormat) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "Response for mobile networks API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MobileNetworkListResult {
    #[doc = "A list of mobile networks in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[doc = "Public land mobile network (PLMN) ID."]
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
#[doc = "Reference to a mobile network resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MobileNetworkResourceId {
    #[doc = "Mobile network resource ID."]
    pub id: String,
}
impl MobileNetworkResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "The network address and port translation settings to use for the attached data network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NaptConfiguration {
    #[doc = "Whether network address and port translation is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<NaptEnabled>,
    #[doc = "Range of port numbers to use as translated ports on each translated address.\nIf not specified and NAPT is enabled, this range defaults to 1,024 - 49,999.\n(Ports under 1,024 should not be used because these are special purpose ports reserved by IANA. Ports 50,000 and above are reserved for non-NAPT use.)"]
    #[serde(rename = "portRange", default, skip_serializing_if = "Option::is_none")]
    pub port_range: Option<PortRange>,
    #[doc = "The minimum time (in seconds) that will pass before a port that was used by a closed pinhole can be recycled for use by another pinhole. All hold times must be minimum 1 second."]
    #[serde(rename = "portReuseHoldTime", default, skip_serializing_if = "Option::is_none")]
    pub port_reuse_hold_time: Option<PortReuseHoldTimes>,
    #[doc = "Maximum number of UDP and TCP pinholes that can be open simultaneously on the core interface. For 5G networks, this is the N6 interface. For 4G networks, this is the SGi interface."]
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
#[doc = "Whether network address and port translation is enabled."]
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
#[doc = "Indicates whether this version is obsolete."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ObsoleteVersion")]
pub enum ObsoleteVersion {
    Obsolete,
    NotObsolete,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ObsoleteVersion {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ObsoleteVersion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ObsoleteVersion {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Obsolete => serializer.serialize_unit_variant("ObsoleteVersion", 0u32, "Obsolete"),
            Self::NotObsolete => serializer.serialize_unit_variant("ObsoleteVersion", 1u32, "NotObsolete"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[doc = "Packet core control plane properties."]
    pub properties: PacketCoreControlPlanePropertiesFormat,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl PacketCoreControlPlane {
    pub fn new(tracked_resource: TrackedResource, properties: PacketCoreControlPlanePropertiesFormat) -> Self {
        Self {
            tracked_resource,
            properties,
            identity: None,
        }
    }
}
#[doc = "Packet core control plane collect diagnostics package options"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PacketCoreControlPlaneCollectDiagnosticsPackage {
    #[doc = "The Storage Account Blob URL to upload the diagnostics package to."]
    #[serde(rename = "storageAccountBlobUrl")]
    pub storage_account_blob_url: String,
}
impl PacketCoreControlPlaneCollectDiagnosticsPackage {
    pub fn new(storage_account_blob_url: String) -> Self {
        Self { storage_account_blob_url }
    }
}
#[doc = "Response for packet core control planes API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PacketCoreControlPlaneListResult {
    #[doc = "A list of packet core control planes in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "Packet core control plane properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PacketCoreControlPlanePropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The installation state of the packet core."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<Installation>,
    #[doc = "Site(s) under which this packet core control plane should be deployed. The sites must be in the same location as the packet core control plane."]
    pub sites: Vec<SiteResourceId>,
    #[doc = "The platform where the packet core is deployed."]
    pub platform: PlatformConfiguration,
    #[doc = "The core network technology generation (5G core or EPC / 4G core)."]
    #[serde(rename = "coreNetworkTechnology", default, skip_serializing_if = "Option::is_none")]
    pub core_network_technology: Option<CoreNetworkType>,
    #[doc = "The version of the packet core software that is deployed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The previous version of the packet core software that was deployed. Used when performing the rollback action."]
    #[serde(rename = "rollbackVersion", default, skip_serializing_if = "Option::is_none")]
    pub rollback_version: Option<String>,
    #[doc = "Interface properties"]
    #[serde(rename = "controlPlaneAccessInterface")]
    pub control_plane_access_interface: InterfaceProperties,
    #[doc = "The SKU of the packet core control plane resource. The SKU list may change over time when a new SKU gets added or an exiting SKU gets removed."]
    pub sku: BillingSku,
    #[doc = "The MTU (in bytes) signaled to the UE. The same MTU is set on the user plane data links for all data networks. The MTU set on the user plane access link is calculated to be 60 bytes greater than this value to allow for GTP encapsulation."]
    #[serde(rename = "ueMtu", default, skip_serializing_if = "Option::is_none")]
    pub ue_mtu: Option<i32>,
    #[doc = "The kubernetes ingress configuration to control access to packet core diagnostics over local APIs."]
    #[serde(rename = "localDiagnosticsAccess")]
    pub local_diagnostics_access: LocalDiagnosticsAccessConfiguration,
    #[doc = "Settings to allow interoperability with third party components e.g. RANs and UEs."]
    #[serde(rename = "interopSettings", default, skip_serializing_if = "Option::is_none")]
    pub interop_settings: Option<serde_json::Value>,
}
impl PacketCoreControlPlanePropertiesFormat {
    pub fn new(
        sites: Vec<SiteResourceId>,
        platform: PlatformConfiguration,
        control_plane_access_interface: InterfaceProperties,
        sku: BillingSku,
        local_diagnostics_access: LocalDiagnosticsAccessConfiguration,
    ) -> Self {
        Self {
            provisioning_state: None,
            installation: None,
            sites,
            platform,
            core_network_technology: None,
            version: None,
            rollback_version: None,
            control_plane_access_interface,
            sku,
            ue_mtu: None,
            local_diagnostics_access,
            interop_settings: None,
        }
    }
}
#[doc = "Packet core control plane version resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PacketCoreControlPlaneVersion {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Packet core control plane version properties."]
    pub properties: PacketCoreControlPlaneVersionPropertiesFormat,
}
impl PacketCoreControlPlaneVersion {
    pub fn new(properties: PacketCoreControlPlaneVersionPropertiesFormat) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "Response for packet core control plane version API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PacketCoreControlPlaneVersionListResult {
    #[doc = "A list of supported packet core control plane versions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PacketCoreControlPlaneVersion>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PacketCoreControlPlaneVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PacketCoreControlPlaneVersionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Packet core control plane version properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PacketCoreControlPlaneVersionPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Platform specific packet core control plane version properties."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub platforms: Vec<Platform>,
}
impl PacketCoreControlPlaneVersionPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Packet core data plane resource. Must be created in the same location as its parent packet core control plane."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PacketCoreDataPlane {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Packet core data plane properties."]
    pub properties: PacketCoreDataPlanePropertiesFormat,
}
impl PacketCoreDataPlane {
    pub fn new(tracked_resource: TrackedResource, properties: PacketCoreDataPlanePropertiesFormat) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "Response for packet core data planes API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PacketCoreDataPlaneListResult {
    #[doc = "A list of packet core data planes in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "Packet core data plane properties."]
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
#[doc = "Data flow policy rule configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PccRuleConfiguration {
    #[doc = "The name of the rule. This must be unique within the parent service. You must not use any of the following reserved strings - `default`, `requested` or `service`."]
    #[serde(rename = "ruleName")]
    pub rule_name: String,
    #[doc = "A precedence value that is used to decide between data flow policy rules when identifying the QoS values to use for a particular SIM. A lower value means a higher priority. This value should be unique among all data flow policy rules configured in the mobile network."]
    #[serde(rename = "rulePrecedence")]
    pub rule_precedence: i32,
    #[doc = "Data flow policy rule QoS policy"]
    #[serde(rename = "ruleQosPolicy", default, skip_serializing_if = "Option::is_none")]
    pub rule_qos_policy: Option<PccRuleQosPolicy>,
    #[doc = "Traffic control permission."]
    #[serde(rename = "trafficControl", default, skip_serializing_if = "Option::is_none")]
    pub traffic_control: Option<TrafficControlPermission>,
    #[doc = "The set of data flow templates to use for this data flow policy rule."]
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
#[doc = "Data flow policy rule QoS policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PccRuleQosPolicy {
    #[serde(flatten)]
    pub qos_policy: QosPolicy,
    #[doc = "Aggregate maximum bit rate."]
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
    #[doc = "Pinhole timeout for TCP pinholes in seconds. Default for TCP is 3 minutes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<i32>,
    #[doc = "Pinhole timeout for UDP pinholes in seconds. Default for UDP is 30 seconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub udp: Option<i32>,
    #[doc = "Pinhole timeout for ICMP pinholes in seconds. Default for ICMP Echo is 30 seconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icmp: Option<i32>,
}
impl PinholeTimeouts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Platform specific packet core control plane version properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Platform {
    #[doc = "The platform type where packet core is deployed. The contents of this enum can change."]
    #[serde(rename = "platformType", default, skip_serializing_if = "Option::is_none")]
    pub platform_type: Option<PlatformType>,
    #[doc = "The state of this packet core control plane version."]
    #[serde(rename = "versionState", default, skip_serializing_if = "Option::is_none")]
    pub version_state: Option<VersionState>,
    #[doc = "The minimum software version of the platform where this packet core version can be deployed."]
    #[serde(rename = "minimumPlatformSoftwareVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimum_platform_software_version: Option<String>,
    #[doc = "The maximum software version of the platform where this packet core version can be deployed."]
    #[serde(rename = "maximumPlatformSoftwareVersion", default, skip_serializing_if = "Option::is_none")]
    pub maximum_platform_software_version: Option<String>,
    #[doc = "Indicates whether this is the recommended version to use for new packet core control plane deployments."]
    #[serde(rename = "recommendedVersion", default, skip_serializing_if = "Option::is_none")]
    pub recommended_version: Option<RecommendedVersion>,
    #[doc = "Indicates whether this version is obsolete."]
    #[serde(rename = "obsoleteVersion", default, skip_serializing_if = "Option::is_none")]
    pub obsolete_version: Option<ObsoleteVersion>,
}
impl Platform {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The platform where the packet core is deployed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlatformConfiguration {
    #[doc = "The platform type where packet core is deployed. The contents of this enum can change."]
    #[serde(rename = "type")]
    pub type_: PlatformType,
    #[doc = "Reference to an Azure Stack Edge device resource."]
    #[serde(rename = "azureStackEdgeDevice", default, skip_serializing_if = "Option::is_none")]
    pub azure_stack_edge_device: Option<AzureStackEdgeDeviceResourceId>,
    #[doc = "The Azure Stack Edge devices where the packet core is deployed. If the packet core is deployed across multiple devices, all devices will appear in this list."]
    #[serde(
        rename = "azureStackEdgeDevices",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub azure_stack_edge_devices: Vec<AzureStackEdgeDeviceResourceId>,
    #[doc = "Reference to an Azure Stack HCI cluster resource."]
    #[serde(rename = "azureStackHciCluster", default, skip_serializing_if = "Option::is_none")]
    pub azure_stack_hci_cluster: Option<AzureStackHciClusterResourceId>,
    #[doc = "Reference to an Azure Arc custom location resource."]
    #[serde(rename = "connectedCluster", default, skip_serializing_if = "Option::is_none")]
    pub connected_cluster: Option<ConnectedClusterResourceId>,
    #[doc = "Reference to an Azure Arc custom location resource."]
    #[serde(rename = "customLocation", default, skip_serializing_if = "Option::is_none")]
    pub custom_location: Option<CustomLocationResourceId>,
}
impl PlatformConfiguration {
    pub fn new(type_: PlatformType) -> Self {
        Self {
            type_,
            azure_stack_edge_device: None,
            azure_stack_edge_devices: Vec::new(),
            azure_stack_hci_cluster: None,
            connected_cluster: None,
            custom_location: None,
        }
    }
}
#[doc = "The platform type where packet core is deployed. The contents of this enum can change."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PlatformType")]
pub enum PlatformType {
    #[serde(rename = "AKS-HCI")]
    AksHci,
    #[serde(rename = "3P-AZURE-STACK-HCI")]
    N3P_AZURE_STACK_HCI,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PlatformType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PlatformType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PlatformType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AksHci => serializer.serialize_unit_variant("PlatformType", 0u32, "AKS-HCI"),
            Self::N3P_AZURE_STACK_HCI => serializer.serialize_unit_variant("PlatformType", 1u32, "3P-AZURE-STACK-HCI"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Public land mobile network (PLMN) ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlmnId {
    #[doc = "Mobile country code."]
    pub mcc: Mcc,
    #[doc = "Mobile network code."]
    pub mnc: Mnc,
}
impl PlmnId {
    pub fn new(mcc: Mcc, mnc: Mnc) -> Self {
        Self { mcc, mnc }
    }
}
#[doc = "Public land mobile network (PLMN) ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlmnIdRm {
    #[doc = "Mobile country code."]
    pub mcc: Mcc,
    #[doc = "Mobile network code."]
    pub mnc: Mnc,
}
impl PlmnIdRm {
    pub fn new(mcc: Mcc, mnc: Mnc) -> Self {
        Self { mcc, mnc }
    }
}
#[doc = "Range of port numbers to use as translated ports on each translated address.\nIf not specified and NAPT is enabled, this range defaults to 1,024 - 49,999.\n(Ports under 1,024 should not be used because these are special purpose ports reserved by IANA. Ports 50,000 and above are reserved for non-NAPT use.)"]
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
    #[doc = "Aggregate maximum bit rate."]
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
#[doc = "Indicates whether this is the recommended version to use for new packet core control plane deployments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RecommendedVersion")]
pub enum RecommendedVersion {
    Recommended,
    NotRecommended,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RecommendedVersion {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RecommendedVersion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RecommendedVersion {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Recommended => serializer.serialize_unit_variant("RecommendedVersion", 0u32, "Recommended"),
            Self::NotRecommended => serializer.serialize_unit_variant("RecommendedVersion", 1u32, "NotRecommended"),
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
#[doc = "Service resource. Must be created in the same location as its parent mobile network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Service properties."]
    pub properties: ServicePropertiesFormat,
}
impl Service {
    pub fn new(tracked_resource: TrackedResource, properties: ServicePropertiesFormat) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "Data flow template"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDataFlowTemplate {
    #[doc = "The name of the data flow template. This must be unique within the parent data flow policy rule. You must not use any of the following reserved strings - `default`, `requested` or `service`."]
    #[serde(rename = "templateName")]
    pub template_name: String,
    #[doc = "Service data flow direction."]
    pub direction: SdfDirection,
    #[doc = "A list of the allowed protocol(s) for this flow. If you want this flow to be able to use any protocol within the internet protocol suite, use the value `ip`. If you only want to allow a selection of protocols, you must use the corresponding IANA Assigned Internet Protocol Number for each protocol, as described in https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml. For example, for UDP, you must use 17. If you use the value `ip` then you must leave the field `port` unspecified."]
    pub protocol: Vec<String>,
    #[doc = "The remote IP address(es) to which UEs will connect for this flow. If you want to allow connections on any IP address, use the value `any`. Otherwise, you must provide each of the remote IP addresses to which the packet core instance will connect for this flow. You must provide each IP address in CIDR notation, including the netmask (for example, 192.0.2.54/24)."]
    #[serde(rename = "remoteIpList")]
    pub remote_ip_list: Vec<String>,
    #[doc = "The port(s) to which UEs will connect for this flow. You can specify zero or more ports or port ranges. If you specify one or more ports or port ranges then you must specify a value other than `ip` in the `protocol` field. This is an optional setting. If you do not specify it then connections will be allowed on all ports. Port ranges must be specified as <FirstPort>-<LastPort>. For example: [`8080`, `8082-8085`]."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "Response for services API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceListResult {
    #[doc = "A list of services."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[doc = "A precedence value that is used to decide between services when identifying the QoS values to use for a particular SIM. A lower value means a higher priority. This value should be unique among all services configured in the mobile network."]
    #[serde(rename = "servicePrecedence")]
    pub service_precedence: i32,
    #[doc = "QoS policy"]
    #[serde(rename = "serviceQosPolicy", default, skip_serializing_if = "Option::is_none")]
    pub service_qos_policy: Option<QosPolicy>,
    #[doc = "The set of data flow policy rules that make up this service."]
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
#[doc = "Reference to a service resource."]
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
#[doc = "SIM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sim {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "SIM properties."]
    pub properties: SimPropertiesFormat,
}
impl Sim {
    pub fn new(properties: SimPropertiesFormat) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "The SIMs to delete."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimDeleteList {
    #[doc = "A list of SIM resource names to delete."]
    pub sims: Vec<String>,
}
impl SimDeleteList {
    pub fn new(sims: Vec<String>) -> Self {
        Self { sims }
    }
}
#[doc = "SIM group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimGroup {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "SIM group properties."]
    pub properties: SimGroupPropertiesFormat,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl SimGroup {
    pub fn new(tracked_resource: TrackedResource, properties: SimGroupPropertiesFormat) -> Self {
        Self {
            tracked_resource,
            properties,
            identity: None,
        }
    }
}
#[doc = "Response for list SIM groups API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SimGroupListResult {
    #[doc = "A list of SIM groups in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SimGroup>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SimGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SimGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SIM group properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SimGroupPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "An Azure key vault key."]
    #[serde(rename = "encryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<KeyVaultKey>,
    #[doc = "Reference to a mobile network resource."]
    #[serde(rename = "mobileNetwork", default, skip_serializing_if = "Option::is_none")]
    pub mobile_network: Option<MobileNetworkResourceId>,
}
impl SimGroupPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to a SIM group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimGroupResourceId {
    #[doc = "SIM group resource ID."]
    pub id: String,
}
impl SimGroupResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Response for list SIMs API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SimListResult {
    #[doc = "A list of SIMs in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "SIM name and encrypted properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimNameAndEncryptedProperties {
    #[doc = "The name of the SIM."]
    pub name: String,
    #[doc = "Encrypted SIM properties."]
    pub properties: EncryptedSimPropertiesFormat,
}
impl SimNameAndEncryptedProperties {
    pub fn new(name: String, properties: EncryptedSimPropertiesFormat) -> Self {
        Self { name, properties }
    }
}
#[doc = "SIM name and properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimNameAndProperties {
    #[doc = "The name of the SIM."]
    pub name: String,
    #[doc = "SIM properties."]
    pub properties: SimPropertiesFormat,
}
impl SimNameAndProperties {
    pub fn new(name: String, properties: SimPropertiesFormat) -> Self {
        Self { name, properties }
    }
}
#[doc = "SIM policy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimPolicy {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "SIM policy properties. Must be created in the same location as its parent mobile network."]
    pub properties: SimPolicyPropertiesFormat,
}
impl SimPolicy {
    pub fn new(tracked_resource: TrackedResource, properties: SimPolicyPropertiesFormat) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "Response for SIM policies API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SimPolicyListResult {
    #[doc = "A list of SIM policies."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "SIM policy properties. Must be created in the same location as its parent mobile network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimPolicyPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The provisioning state of a resource e.g. SIM/SIM policy on a site. The dictionary keys will ARM resource IDs in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.MobileNetwork/mobileNetworks/{mobileNetworkName}/sites/{siteName}. The dictionary values will be from the \"SiteProvisioningState\" enum."]
    #[serde(rename = "siteProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub site_provisioning_state: Option<SiteProvisioning>,
    #[doc = "Aggregate maximum bit rate."]
    #[serde(rename = "ueAmbr")]
    pub ue_ambr: Ambr,
    #[doc = "Reference to a slice resource."]
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
            site_provisioning_state: None,
            ue_ambr,
            default_slice,
            rfsp_index: None,
            registration_timer: None,
            slice_configurations,
        }
    }
}
#[doc = "Reference to a SIM policy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimPolicyResourceId {
    #[doc = "SIM policy resource ID."]
    pub id: String,
}
impl SimPolicyResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "SIM properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimPropertiesFormat {
    #[serde(flatten)]
    pub common_sim_properties_format: CommonSimPropertiesFormat,
    #[doc = "The Ki value for the SIM."]
    #[serde(rename = "authenticationKey", default, skip_serializing_if = "Option::is_none")]
    pub authentication_key: Option<String>,
    #[doc = "The Opc value for the SIM."]
    #[serde(rename = "operatorKeyCode", default, skip_serializing_if = "Option::is_none")]
    pub operator_key_code: Option<String>,
}
impl SimPropertiesFormat {
    pub fn new(common_sim_properties_format: CommonSimPropertiesFormat) -> Self {
        Self {
            common_sim_properties_format,
            authentication_key: None,
            operator_key_code: None,
        }
    }
}
#[doc = "The state of the SIM resource."]
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
#[doc = "Static IP configuration for a SIM, scoped to a particular attached data network and slice."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SimStaticIpProperties {
    #[doc = "Reference to an attached data network resource."]
    #[serde(rename = "attachedDataNetwork", default, skip_serializing_if = "Option::is_none")]
    pub attached_data_network: Option<AttachedDataNetworkResourceId>,
    #[doc = "Reference to a slice resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slice: Option<SliceResourceId>,
    #[doc = "The static IP configuration for the SIM to use at the defined network scope."]
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
    #[doc = "The static IP configuration for the SIM to use at the defined network scope."]
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
#[doc = "The SIMs to upload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimUploadList {
    #[doc = "A list of SIMs to upload."]
    pub sims: Vec<SimNameAndProperties>,
}
impl SimUploadList {
    pub fn new(sims: Vec<SimNameAndProperties>) -> Self {
        Self { sims }
    }
}
#[doc = "Site resource. Must be created in the same location as its parent mobile network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Site {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Site properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SitePropertiesFormat>,
}
impl Site {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Response for sites API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteListResult {
    #[doc = "A list of sites in a mobile network."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[doc = "An array of IDs of the network functions deployed in the site. Deleting the site will delete any network functions that are deployed in the site."]
    #[serde(
        rename = "networkFunctions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_functions: Vec<SubResource>,
}
impl SitePropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The provisioning state of a resource e.g. SIM/SIM policy on a site. The dictionary keys will ARM resource IDs in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.MobileNetwork/mobileNetworks/{mobileNetworkName}/sites/{siteName}. The dictionary values will be from the \"SiteProvisioningState\" enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteProvisioning {}
impl SiteProvisioning {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The provisioning state of a resource e.g. SIM/SIM policy on a site."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SiteProvisioningState")]
pub enum SiteProvisioningState {
    NotApplicable,
    Adding,
    Updating,
    Deleting,
    Provisioned,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SiteProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SiteProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SiteProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotApplicable => serializer.serialize_unit_variant("SiteProvisioningState", 0u32, "NotApplicable"),
            Self::Adding => serializer.serialize_unit_variant("SiteProvisioningState", 1u32, "Adding"),
            Self::Updating => serializer.serialize_unit_variant("SiteProvisioningState", 2u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("SiteProvisioningState", 3u32, "Deleting"),
            Self::Provisioned => serializer.serialize_unit_variant("SiteProvisioningState", 4u32, "Provisioned"),
            Self::Failed => serializer.serialize_unit_variant("SiteProvisioningState", 5u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Reference to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteResourceId {
    #[doc = "Site resource ID."]
    pub id: String,
}
impl SiteResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Network slice resource. Must be created in the same location as its parent mobile network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Slice {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Network slice properties."]
    pub properties: SlicePropertiesFormat,
}
impl Slice {
    pub fn new(tracked_resource: TrackedResource, properties: SlicePropertiesFormat) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "Per-slice settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SliceConfiguration {
    #[doc = "Reference to a slice resource."]
    pub slice: SliceResourceId,
    #[doc = "Reference to a data network resource."]
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
#[doc = "Response for network slice API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SliceListResult {
    #[doc = "A list of network slices in a mobile network."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[doc = "Single-network slice selection assistance information (S-NSSAI)."]
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
#[doc = "Reference to a slice resource."]
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
#[doc = "Single-network slice selection assistance information (S-NSSAI)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snssai {
    #[doc = "Slice/service type (SST)."]
    pub sst: i32,
    #[doc = "Slice differentiator (SD)."]
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
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The state of this packet core control plane version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VersionState")]
pub enum VersionState {
    Unknown,
    Preview,
    Validating,
    ValidationFailed,
    Active,
    Deprecated,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VersionState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VersionState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VersionState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("VersionState", 0u32, "Unknown"),
            Self::Preview => serializer.serialize_unit_variant("VersionState", 1u32, "Preview"),
            Self::Validating => serializer.serialize_unit_variant("VersionState", 2u32, "Validating"),
            Self::ValidationFailed => serializer.serialize_unit_variant("VersionState", 3u32, "ValidationFailed"),
            Self::Active => serializer.serialize_unit_variant("VersionState", 4u32, "Active"),
            Self::Deprecated => serializer.serialize_unit_variant("VersionState", 5u32, "Deprecated"),
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

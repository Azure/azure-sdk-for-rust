#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The properties that define a BGP session."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BgpSession {
    #[doc = "The IPv4 prefix that contains both ends' IPv4 addresses."]
    #[serde(rename = "sessionPrefixV4", default, skip_serializing_if = "Option::is_none")]
    pub session_prefix_v4: Option<String>,
    #[doc = "The IPv6 prefix that contains both ends' IPv6 addresses."]
    #[serde(rename = "sessionPrefixV6", default, skip_serializing_if = "Option::is_none")]
    pub session_prefix_v6: Option<String>,
    #[doc = "The IPv4 session address on Microsoft's end."]
    #[serde(rename = "microsoftSessionIPv4Address", default, skip_serializing_if = "Option::is_none")]
    pub microsoft_session_i_pv4_address: Option<String>,
    #[doc = "The IPv6 session address on Microsoft's end."]
    #[serde(rename = "microsoftSessionIPv6Address", default, skip_serializing_if = "Option::is_none")]
    pub microsoft_session_i_pv6_address: Option<String>,
    #[doc = "The IPv4 session address on peer's end."]
    #[serde(rename = "peerSessionIPv4Address", default, skip_serializing_if = "Option::is_none")]
    pub peer_session_i_pv4_address: Option<String>,
    #[doc = "The IPv6 session address on peer's end."]
    #[serde(rename = "peerSessionIPv6Address", default, skip_serializing_if = "Option::is_none")]
    pub peer_session_i_pv6_address: Option<String>,
    #[doc = "The state of the IPv4 session."]
    #[serde(rename = "sessionStateV4", default, skip_serializing_if = "Option::is_none")]
    pub session_state_v4: Option<bgp_session::SessionStateV4>,
    #[doc = "The state of the IPv6 session."]
    #[serde(rename = "sessionStateV6", default, skip_serializing_if = "Option::is_none")]
    pub session_state_v6: Option<bgp_session::SessionStateV6>,
    #[doc = "The maximum number of prefixes advertised over the IPv4 session."]
    #[serde(rename = "maxPrefixesAdvertisedV4", default, skip_serializing_if = "Option::is_none")]
    pub max_prefixes_advertised_v4: Option<i32>,
    #[doc = "The maximum number of prefixes advertised over the IPv6 session."]
    #[serde(rename = "maxPrefixesAdvertisedV6", default, skip_serializing_if = "Option::is_none")]
    pub max_prefixes_advertised_v6: Option<i32>,
    #[doc = "The MD5 authentication key of the session."]
    #[serde(rename = "md5AuthenticationKey", default, skip_serializing_if = "Option::is_none")]
    pub md5_authentication_key: Option<String>,
}
impl BgpSession {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod bgp_session {
    use super::*;
    #[doc = "The state of the IPv4 session."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SessionStateV4")]
    pub enum SessionStateV4 {
        None,
        Idle,
        Connect,
        Active,
        OpenSent,
        OpenConfirm,
        OpenReceived,
        Established,
        PendingAdd,
        PendingUpdate,
        PendingRemove,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SessionStateV4 {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SessionStateV4 {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SessionStateV4 {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("SessionStateV4", 0u32, "None"),
                Self::Idle => serializer.serialize_unit_variant("SessionStateV4", 1u32, "Idle"),
                Self::Connect => serializer.serialize_unit_variant("SessionStateV4", 2u32, "Connect"),
                Self::Active => serializer.serialize_unit_variant("SessionStateV4", 3u32, "Active"),
                Self::OpenSent => serializer.serialize_unit_variant("SessionStateV4", 4u32, "OpenSent"),
                Self::OpenConfirm => serializer.serialize_unit_variant("SessionStateV4", 5u32, "OpenConfirm"),
                Self::OpenReceived => serializer.serialize_unit_variant("SessionStateV4", 6u32, "OpenReceived"),
                Self::Established => serializer.serialize_unit_variant("SessionStateV4", 7u32, "Established"),
                Self::PendingAdd => serializer.serialize_unit_variant("SessionStateV4", 8u32, "PendingAdd"),
                Self::PendingUpdate => serializer.serialize_unit_variant("SessionStateV4", 9u32, "PendingUpdate"),
                Self::PendingRemove => serializer.serialize_unit_variant("SessionStateV4", 10u32, "PendingRemove"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The state of the IPv6 session."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SessionStateV6")]
    pub enum SessionStateV6 {
        None,
        Idle,
        Connect,
        Active,
        OpenSent,
        OpenConfirm,
        OpenReceived,
        Established,
        PendingAdd,
        PendingUpdate,
        PendingRemove,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SessionStateV6 {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SessionStateV6 {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SessionStateV6 {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("SessionStateV6", 0u32, "None"),
                Self::Idle => serializer.serialize_unit_variant("SessionStateV6", 1u32, "Idle"),
                Self::Connect => serializer.serialize_unit_variant("SessionStateV6", 2u32, "Connect"),
                Self::Active => serializer.serialize_unit_variant("SessionStateV6", 3u32, "Active"),
                Self::OpenSent => serializer.serialize_unit_variant("SessionStateV6", 4u32, "OpenSent"),
                Self::OpenConfirm => serializer.serialize_unit_variant("SessionStateV6", 5u32, "OpenConfirm"),
                Self::OpenReceived => serializer.serialize_unit_variant("SessionStateV6", 6u32, "OpenReceived"),
                Self::Established => serializer.serialize_unit_variant("SessionStateV6", 7u32, "Established"),
                Self::PendingAdd => serializer.serialize_unit_variant("SessionStateV6", 8u32, "PendingAdd"),
                Self::PendingUpdate => serializer.serialize_unit_variant("SessionStateV6", 9u32, "PendingUpdate"),
                Self::PendingRemove => serializer.serialize_unit_variant("SessionStateV6", 10u32, "PendingRemove"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The CDN peering prefix"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CdnPeeringPrefix {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties that define a CDN peering prefix"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CdnPeeringPrefixProperties>,
}
impl CdnPeeringPrefix {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paginated list of CDN peering prefixes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CdnPeeringPrefixListResult {
    #[doc = "The list of CDN peering prefixes."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CdnPeeringPrefix>,
    #[doc = "The link to fetch the next page of CDN peering prefixes."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CdnPeeringPrefixListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CdnPeeringPrefixListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that define a CDN peering prefix"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CdnPeeringPrefixProperties {
    #[doc = "The prefix."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[doc = "The Azure region."]
    #[serde(rename = "azureRegion", default, skip_serializing_if = "Option::is_none")]
    pub azure_region: Option<String>,
    #[doc = "The Azure service."]
    #[serde(rename = "azureService", default, skip_serializing_if = "Option::is_none")]
    pub azure_service: Option<String>,
    #[doc = "The flag that indicates whether or not this is the primary region."]
    #[serde(rename = "isPrimaryRegion", default, skip_serializing_if = "Option::is_none")]
    pub is_primary_region: Option<bool>,
    #[doc = "The BGP Community"]
    #[serde(rename = "bgpCommunity", default, skip_serializing_if = "Option::is_none")]
    pub bgp_community: Option<String>,
}
impl CdnPeeringPrefixProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for CheckServiceProviderAvailabilityInput"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckServiceProviderAvailabilityInput {
    #[doc = "Gets or sets the peering service location."]
    #[serde(rename = "peeringServiceLocation", default, skip_serializing_if = "Option::is_none")]
    pub peering_service_location: Option<String>,
    #[doc = "Gets or sets the peering service provider."]
    #[serde(rename = "peeringServiceProvider", default, skip_serializing_if = "Option::is_none")]
    pub peering_service_provider: Option<String>,
}
impl CheckServiceProviderAvailabilityInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The contact detail class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContactDetail {
    #[doc = "The role of the contact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<contact_detail::Role>,
    #[doc = "The e-mail address of the contact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The phone number of the contact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl ContactDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod contact_detail {
    use super::*;
    #[doc = "The role of the contact."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Role")]
    pub enum Role {
        Noc,
        Policy,
        Technical,
        Service,
        Escalation,
        Other,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Role {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Role {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Role {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Noc => serializer.serialize_unit_variant("Role", 0u32, "Noc"),
                Self::Policy => serializer.serialize_unit_variant("Role", 1u32, "Policy"),
                Self::Technical => serializer.serialize_unit_variant("Role", 2u32, "Technical"),
                Self::Service => serializer.serialize_unit_variant("Role", 3u32, "Service"),
                Self::Escalation => serializer.serialize_unit_variant("Role", 4u32, "Escalation"),
                Self::Other => serializer.serialize_unit_variant("Role", 5u32, "Other"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties that define a direct connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DirectConnection {
    #[doc = "The bandwidth of the connection."]
    #[serde(rename = "bandwidthInMbps", default, skip_serializing_if = "Option::is_none")]
    pub bandwidth_in_mbps: Option<i32>,
    #[doc = "The bandwidth that is actually provisioned."]
    #[serde(rename = "provisionedBandwidthInMbps", default, skip_serializing_if = "Option::is_none")]
    pub provisioned_bandwidth_in_mbps: Option<i32>,
    #[doc = "The field indicating if Microsoft provides session ip addresses."]
    #[serde(rename = "sessionAddressProvider", default, skip_serializing_if = "Option::is_none")]
    pub session_address_provider: Option<direct_connection::SessionAddressProvider>,
    #[doc = "The flag that indicates whether or not the connection is used for peering service."]
    #[serde(rename = "useForPeeringService", default, skip_serializing_if = "Option::is_none")]
    pub use_for_peering_service: Option<bool>,
    #[doc = "The ID used within Microsoft's peering provisioning system to track the connection"]
    #[serde(rename = "microsoftTrackingId", default, skip_serializing_if = "Option::is_none")]
    pub microsoft_tracking_id: Option<String>,
    #[doc = "The PeeringDB.com ID of the facility at which the connection has to be set up."]
    #[serde(rename = "peeringDBFacilityId", default, skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_id: Option<i32>,
    #[doc = "The state of the connection."]
    #[serde(rename = "connectionState", default, skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<direct_connection::ConnectionState>,
    #[doc = "The properties that define a BGP session."]
    #[serde(rename = "bgpSession", default, skip_serializing_if = "Option::is_none")]
    pub bgp_session: Option<BgpSession>,
    #[doc = "The unique identifier (GUID) for the connection."]
    #[serde(rename = "connectionIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub connection_identifier: Option<String>,
    #[doc = "The error message related to the connection state, if any."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl DirectConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod direct_connection {
    use super::*;
    #[doc = "The field indicating if Microsoft provides session ip addresses."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SessionAddressProvider")]
    pub enum SessionAddressProvider {
        Microsoft,
        Peer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SessionAddressProvider {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SessionAddressProvider {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SessionAddressProvider {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Microsoft => serializer.serialize_unit_variant("SessionAddressProvider", 0u32, "Microsoft"),
                Self::Peer => serializer.serialize_unit_variant("SessionAddressProvider", 1u32, "Peer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The state of the connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConnectionState")]
    pub enum ConnectionState {
        None,
        PendingApproval,
        Approved,
        ProvisioningStarted,
        ProvisioningFailed,
        ProvisioningCompleted,
        Validating,
        Active,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConnectionState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConnectionState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConnectionState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ConnectionState", 0u32, "None"),
                Self::PendingApproval => serializer.serialize_unit_variant("ConnectionState", 1u32, "PendingApproval"),
                Self::Approved => serializer.serialize_unit_variant("ConnectionState", 2u32, "Approved"),
                Self::ProvisioningStarted => serializer.serialize_unit_variant("ConnectionState", 3u32, "ProvisioningStarted"),
                Self::ProvisioningFailed => serializer.serialize_unit_variant("ConnectionState", 4u32, "ProvisioningFailed"),
                Self::ProvisioningCompleted => serializer.serialize_unit_variant("ConnectionState", 5u32, "ProvisioningCompleted"),
                Self::Validating => serializer.serialize_unit_variant("ConnectionState", 6u32, "Validating"),
                Self::Active => serializer.serialize_unit_variant("ConnectionState", 7u32, "Active"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties that define a direct peering facility."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DirectPeeringFacility {
    #[doc = "The address of the direct peering facility."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "The type of the direct peering."]
    #[serde(rename = "directPeeringType", default, skip_serializing_if = "Option::is_none")]
    pub direct_peering_type: Option<direct_peering_facility::DirectPeeringType>,
    #[doc = "The PeeringDB.com ID of the facility."]
    #[serde(rename = "peeringDBFacilityId", default, skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_id: Option<i32>,
    #[doc = "The PeeringDB.com URL of the facility."]
    #[serde(rename = "peeringDBFacilityLink", default, skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_link: Option<String>,
}
impl DirectPeeringFacility {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod direct_peering_facility {
    use super::*;
    #[doc = "The type of the direct peering."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DirectPeeringType")]
    pub enum DirectPeeringType {
        Edge,
        Transit,
        Cdn,
        Internal,
        Ix,
        IxRs,
        Voice,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DirectPeeringType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DirectPeeringType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DirectPeeringType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Edge => serializer.serialize_unit_variant("DirectPeeringType", 0u32, "Edge"),
                Self::Transit => serializer.serialize_unit_variant("DirectPeeringType", 1u32, "Transit"),
                Self::Cdn => serializer.serialize_unit_variant("DirectPeeringType", 2u32, "Cdn"),
                Self::Internal => serializer.serialize_unit_variant("DirectPeeringType", 3u32, "Internal"),
                Self::Ix => serializer.serialize_unit_variant("DirectPeeringType", 4u32, "Ix"),
                Self::IxRs => serializer.serialize_unit_variant("DirectPeeringType", 5u32, "IxRs"),
                Self::Voice => serializer.serialize_unit_variant("DirectPeeringType", 6u32, "Voice"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The error detail that describes why an operation has failed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error response that indicates why an operation has failed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error detail that describes why an operation has failed."]
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
#[doc = "The properties that define an exchange connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExchangeConnection {
    #[doc = "The PeeringDB.com ID of the facility at which the connection has to be set up."]
    #[serde(rename = "peeringDBFacilityId", default, skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_id: Option<i32>,
    #[doc = "The state of the connection."]
    #[serde(rename = "connectionState", default, skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<exchange_connection::ConnectionState>,
    #[doc = "The properties that define a BGP session."]
    #[serde(rename = "bgpSession", default, skip_serializing_if = "Option::is_none")]
    pub bgp_session: Option<BgpSession>,
    #[doc = "The unique identifier (GUID) for the connection."]
    #[serde(rename = "connectionIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub connection_identifier: Option<String>,
    #[doc = "The error message related to the connection state, if any."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl ExchangeConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod exchange_connection {
    use super::*;
    #[doc = "The state of the connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConnectionState")]
    pub enum ConnectionState {
        None,
        PendingApproval,
        Approved,
        ProvisioningStarted,
        ProvisioningFailed,
        ProvisioningCompleted,
        Validating,
        Active,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConnectionState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConnectionState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConnectionState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ConnectionState", 0u32, "None"),
                Self::PendingApproval => serializer.serialize_unit_variant("ConnectionState", 1u32, "PendingApproval"),
                Self::Approved => serializer.serialize_unit_variant("ConnectionState", 2u32, "Approved"),
                Self::ProvisioningStarted => serializer.serialize_unit_variant("ConnectionState", 3u32, "ProvisioningStarted"),
                Self::ProvisioningFailed => serializer.serialize_unit_variant("ConnectionState", 4u32, "ProvisioningFailed"),
                Self::ProvisioningCompleted => serializer.serialize_unit_variant("ConnectionState", 5u32, "ProvisioningCompleted"),
                Self::Validating => serializer.serialize_unit_variant("ConnectionState", 6u32, "Validating"),
                Self::Active => serializer.serialize_unit_variant("ConnectionState", 7u32, "Active"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties that define an exchange peering facility."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExchangePeeringFacility {
    #[doc = "The name of the exchange peering facility."]
    #[serde(rename = "exchangeName", default, skip_serializing_if = "Option::is_none")]
    pub exchange_name: Option<String>,
    #[doc = "The bandwidth of the connection between Microsoft and the exchange peering facility."]
    #[serde(rename = "bandwidthInMbps", default, skip_serializing_if = "Option::is_none")]
    pub bandwidth_in_mbps: Option<i32>,
    #[doc = "The IPv4 address of Microsoft at the exchange peering facility."]
    #[serde(rename = "microsoftIPv4Address", default, skip_serializing_if = "Option::is_none")]
    pub microsoft_i_pv4_address: Option<String>,
    #[doc = "The IPv6 address of Microsoft at the exchange peering facility."]
    #[serde(rename = "microsoftIPv6Address", default, skip_serializing_if = "Option::is_none")]
    pub microsoft_i_pv6_address: Option<String>,
    #[doc = "The IPv4 prefixes associated with the exchange peering facility."]
    #[serde(rename = "facilityIPv4Prefix", default, skip_serializing_if = "Option::is_none")]
    pub facility_i_pv4_prefix: Option<String>,
    #[doc = "The IPv6 prefixes associated with the exchange peering facility."]
    #[serde(rename = "facilityIPv6Prefix", default, skip_serializing_if = "Option::is_none")]
    pub facility_i_pv6_prefix: Option<String>,
    #[doc = "The PeeringDB.com ID of the facility."]
    #[serde(rename = "peeringDBFacilityId", default, skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_id: Option<i32>,
    #[doc = "The PeeringDB.com URL of the facility."]
    #[serde(rename = "peeringDBFacilityLink", default, skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_link: Option<String>,
}
impl ExchangePeeringFacility {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The peering API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The information related to the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
    #[doc = "The flag that indicates whether the operation applies to data plane."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The information related to the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayInfo {
    #[doc = "The name of the resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplayInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paginated list of peering API operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The list of peering API operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The link to fetch the next page of peering API operations."]
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
#[doc = "The essential information related to the peer's ASN."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeerAsn {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties that define a peer's ASN."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeerAsnProperties>,
}
impl PeerAsn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paginated list of peer ASNs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeerAsnListResult {
    #[doc = "The list of peer ASNs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeerAsn>,
    #[doc = "The link to fetch the next page of peer ASNs."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PeerAsnListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PeerAsnListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that define a peer's ASN."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeerAsnProperties {
    #[doc = "The Autonomous System Number (ASN) of the peer."]
    #[serde(rename = "peerAsn", default, skip_serializing_if = "Option::is_none")]
    pub peer_asn: Option<i32>,
    #[doc = "The contact details of the peer."]
    #[serde(rename = "peerContactDetail", default, skip_serializing_if = "Vec::is_empty")]
    pub peer_contact_detail: Vec<ContactDetail>,
    #[doc = "The name of the peer."]
    #[serde(rename = "peerName", default, skip_serializing_if = "Option::is_none")]
    pub peer_name: Option<String>,
    #[doc = "The validation state of the ASN associated with the peer."]
    #[serde(rename = "validationState", default, skip_serializing_if = "Option::is_none")]
    pub validation_state: Option<peer_asn_properties::ValidationState>,
    #[doc = "The error message for the validation state"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl PeerAsnProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peer_asn_properties {
    use super::*;
    #[doc = "The validation state of the ASN associated with the peer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ValidationState")]
    pub enum ValidationState {
        None,
        Pending,
        Approved,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ValidationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ValidationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ValidationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ValidationState", 0u32, "None"),
                Self::Pending => serializer.serialize_unit_variant("ValidationState", 1u32, "Pending"),
                Self::Approved => serializer.serialize_unit_variant("ValidationState", 2u32, "Approved"),
                Self::Failed => serializer.serialize_unit_variant("ValidationState", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Peering is a logical representation of a set of connections to the Microsoft Cloud Edge at a location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Peering {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The SKU that defines the tier and kind of the peering."]
    pub sku: PeeringSku,
    #[doc = "The kind of the peering."]
    pub kind: peering::Kind,
    #[doc = "The properties that define connectivity to the Microsoft Cloud Edge."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringProperties>,
    #[doc = "The location of the resource."]
    pub location: String,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Peering {
    pub fn new(sku: PeeringSku, kind: peering::Kind, location: String) -> Self {
        Self {
            resource: Resource::default(),
            sku,
            kind,
            properties: None,
            location,
            tags: None,
        }
    }
}
pub mod peering {
    use super::*;
    #[doc = "The kind of the peering."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Direct,
        Exchange,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Direct => serializer.serialize_unit_variant("Kind", 0u32, "Direct"),
                Self::Exchange => serializer.serialize_unit_variant("Kind", 1u32, "Exchange"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties that define a peering bandwidth offer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringBandwidthOffer {
    #[doc = "The name of the bandwidth offer."]
    #[serde(rename = "offerName", default, skip_serializing_if = "Option::is_none")]
    pub offer_name: Option<String>,
    #[doc = "The value of the bandwidth offer in Mbps."]
    #[serde(rename = "valueInMbps", default, skip_serializing_if = "Option::is_none")]
    pub value_in_mbps: Option<i32>,
}
impl PeeringBandwidthOffer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paginated list of peerings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringListResult {
    #[doc = "The list of peerings."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Peering>,
    #[doc = "The link to fetch the next page of peerings."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PeeringListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PeeringListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Peering location is where connectivity could be established to the Microsoft Cloud Edge."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringLocation {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The kind of peering that the peering location supports."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<peering_location::Kind>,
    #[doc = "The properties that define a peering location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringLocationProperties>,
}
impl PeeringLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peering_location {
    use super::*;
    #[doc = "The kind of peering that the peering location supports."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Direct,
        Exchange,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Direct => serializer.serialize_unit_variant("Kind", 0u32, "Direct"),
                Self::Exchange => serializer.serialize_unit_variant("Kind", 1u32, "Exchange"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The paginated list of peering locations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringLocationListResult {
    #[doc = "The list of peering locations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringLocation>,
    #[doc = "The link to fetch the next page of peering locations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PeeringLocationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PeeringLocationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that define a peering location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringLocationProperties {
    #[doc = "The properties that define a direct peering location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct: Option<PeeringLocationPropertiesDirect>,
    #[doc = "The properties that define an exchange peering location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange: Option<PeeringLocationPropertiesExchange>,
    #[doc = "The name of the peering location."]
    #[serde(rename = "peeringLocation", default, skip_serializing_if = "Option::is_none")]
    pub peering_location: Option<String>,
    #[doc = "The country in which the peering location exists."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "The Azure region associated with the peering location."]
    #[serde(rename = "azureRegion", default, skip_serializing_if = "Option::is_none")]
    pub azure_region: Option<String>,
}
impl PeeringLocationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that define a direct peering location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringLocationPropertiesDirect {
    #[doc = "The list of direct peering facilities at the peering location."]
    #[serde(rename = "peeringFacilities", default, skip_serializing_if = "Vec::is_empty")]
    pub peering_facilities: Vec<DirectPeeringFacility>,
    #[doc = "The list of bandwidth offers available at the peering location."]
    #[serde(rename = "bandwidthOffers", default, skip_serializing_if = "Vec::is_empty")]
    pub bandwidth_offers: Vec<PeeringBandwidthOffer>,
}
impl PeeringLocationPropertiesDirect {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that define an exchange peering location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringLocationPropertiesExchange {
    #[doc = "The list of exchange peering facilities at the peering location."]
    #[serde(rename = "peeringFacilities", default, skip_serializing_if = "Vec::is_empty")]
    pub peering_facilities: Vec<ExchangePeeringFacility>,
}
impl PeeringLocationPropertiesExchange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that define connectivity to the Microsoft Cloud Edge."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringProperties {
    #[doc = "The properties that define a direct peering."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct: Option<PeeringPropertiesDirect>,
    #[doc = "The properties that define an exchange peering."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange: Option<PeeringPropertiesExchange>,
    #[doc = "The location of the peering."]
    #[serde(rename = "peeringLocation", default, skip_serializing_if = "Option::is_none")]
    pub peering_location: Option<String>,
    #[doc = "The provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<peering_properties::ProvisioningState>,
}
impl PeeringProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peering_properties {
    use super::*;
    #[doc = "The provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Updating,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties that define a direct peering."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringPropertiesDirect {
    #[doc = "The set of connections that constitute a direct peering."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub connections: Vec<DirectConnection>,
    #[doc = "The flag that indicates whether or not the peering is used for peering service."]
    #[serde(rename = "useForPeeringService", default, skip_serializing_if = "Option::is_none")]
    pub use_for_peering_service: Option<bool>,
    #[doc = "The sub resource."]
    #[serde(rename = "peerAsn", default, skip_serializing_if = "Option::is_none")]
    pub peer_asn: Option<SubResource>,
    #[doc = "The type of direct peering."]
    #[serde(rename = "directPeeringType", default, skip_serializing_if = "Option::is_none")]
    pub direct_peering_type: Option<peering_properties_direct::DirectPeeringType>,
}
impl PeeringPropertiesDirect {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peering_properties_direct {
    use super::*;
    #[doc = "The type of direct peering."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DirectPeeringType")]
    pub enum DirectPeeringType {
        Edge,
        Transit,
        Cdn,
        Internal,
        Ix,
        IxRs,
        Voice,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DirectPeeringType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DirectPeeringType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DirectPeeringType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Edge => serializer.serialize_unit_variant("DirectPeeringType", 0u32, "Edge"),
                Self::Transit => serializer.serialize_unit_variant("DirectPeeringType", 1u32, "Transit"),
                Self::Cdn => serializer.serialize_unit_variant("DirectPeeringType", 2u32, "Cdn"),
                Self::Internal => serializer.serialize_unit_variant("DirectPeeringType", 3u32, "Internal"),
                Self::Ix => serializer.serialize_unit_variant("DirectPeeringType", 4u32, "Ix"),
                Self::IxRs => serializer.serialize_unit_variant("DirectPeeringType", 5u32, "IxRs"),
                Self::Voice => serializer.serialize_unit_variant("DirectPeeringType", 6u32, "Voice"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties that define an exchange peering."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringPropertiesExchange {
    #[doc = "The set of connections that constitute an exchange peering."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub connections: Vec<ExchangeConnection>,
    #[doc = "The sub resource."]
    #[serde(rename = "peerAsn", default, skip_serializing_if = "Option::is_none")]
    pub peer_asn: Option<SubResource>,
}
impl PeeringPropertiesExchange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that define a received route."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringReceivedRoute {
    #[doc = "The prefix."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[doc = "The next hop for the prefix."]
    #[serde(rename = "nextHop", default, skip_serializing_if = "Option::is_none")]
    pub next_hop: Option<String>,
    #[doc = "The AS path for the prefix."]
    #[serde(rename = "asPath", default, skip_serializing_if = "Option::is_none")]
    pub as_path: Option<String>,
    #[doc = "The origin AS change information for the prefix."]
    #[serde(rename = "originAsValidationState", default, skip_serializing_if = "Option::is_none")]
    pub origin_as_validation_state: Option<String>,
    #[doc = "The RPKI validation state for the prefix and origin AS that's listed in the AS path."]
    #[serde(rename = "rpkiValidationState", default, skip_serializing_if = "Option::is_none")]
    pub rpki_validation_state: Option<String>,
    #[doc = "The authority which holds the Route Origin Authorization record for the prefix, if any."]
    #[serde(rename = "trustAnchor", default, skip_serializing_if = "Option::is_none")]
    pub trust_anchor: Option<String>,
    #[doc = "The received timestamp associated with the prefix."]
    #[serde(rename = "receivedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub received_timestamp: Option<String>,
}
impl PeeringReceivedRoute {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paginated list of received routes for the peering."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringReceivedRouteListResult {
    #[doc = "The list of received routes for the peering."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringReceivedRoute>,
    #[doc = "The link to fetch the next page of received routes for the peering."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PeeringReceivedRouteListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PeeringReceivedRouteListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The customer's ASN that is registered by the peering service provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringRegisteredAsn {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties that define a registered ASN."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringRegisteredAsnProperties>,
}
impl PeeringRegisteredAsn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paginated list of peering registered ASNs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringRegisteredAsnListResult {
    #[doc = "The list of peering registered ASNs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringRegisteredAsn>,
    #[doc = "The link to fetch the next page of peering registered ASNs."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PeeringRegisteredAsnListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PeeringRegisteredAsnListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that define a registered ASN."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringRegisteredAsnProperties {
    #[doc = "The customer's ASN from which traffic originates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[doc = "The peering service prefix key that is to be shared with the customer."]
    #[serde(rename = "peeringServicePrefixKey", default, skip_serializing_if = "Option::is_none")]
    pub peering_service_prefix_key: Option<String>,
    #[doc = "The provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<peering_registered_asn_properties::ProvisioningState>,
}
impl PeeringRegisteredAsnProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peering_registered_asn_properties {
    use super::*;
    #[doc = "The provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Updating,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The customer's prefix that is registered by the peering service provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringRegisteredPrefix {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties that define a registered prefix."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringRegisteredPrefixProperties>,
}
impl PeeringRegisteredPrefix {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paginated list of peering registered prefixes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringRegisteredPrefixListResult {
    #[doc = "The list of peering registered prefixes."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringRegisteredPrefix>,
    #[doc = "The link to fetch the next page of peering registered prefixes."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PeeringRegisteredPrefixListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PeeringRegisteredPrefixListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that define a registered prefix."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringRegisteredPrefixProperties {
    #[doc = "The customer's prefix from which traffic originates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[doc = "The prefix validation state."]
    #[serde(rename = "prefixValidationState", default, skip_serializing_if = "Option::is_none")]
    pub prefix_validation_state: Option<peering_registered_prefix_properties::PrefixValidationState>,
    #[doc = "The peering service prefix key that is to be shared with the customer."]
    #[serde(rename = "peeringServicePrefixKey", default, skip_serializing_if = "Option::is_none")]
    pub peering_service_prefix_key: Option<String>,
    #[doc = "The error message associated with the validation state, if any."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<peering_registered_prefix_properties::ProvisioningState>,
}
impl PeeringRegisteredPrefixProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peering_registered_prefix_properties {
    use super::*;
    #[doc = "The prefix validation state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrefixValidationState")]
    pub enum PrefixValidationState {
        None,
        Invalid,
        Verified,
        Failed,
        Pending,
        Warning,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrefixValidationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrefixValidationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrefixValidationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("PrefixValidationState", 0u32, "None"),
                Self::Invalid => serializer.serialize_unit_variant("PrefixValidationState", 1u32, "Invalid"),
                Self::Verified => serializer.serialize_unit_variant("PrefixValidationState", 2u32, "Verified"),
                Self::Failed => serializer.serialize_unit_variant("PrefixValidationState", 3u32, "Failed"),
                Self::Pending => serializer.serialize_unit_variant("PrefixValidationState", 4u32, "Pending"),
                Self::Warning => serializer.serialize_unit_variant("PrefixValidationState", 5u32, "Warning"),
                Self::Unknown => serializer.serialize_unit_variant("PrefixValidationState", 6u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Updating,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Peering Service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringService {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The SKU that defines the type of the peering service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PeeringServiceSku>,
    #[doc = "The properties that define connectivity to the Peering Service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringServiceProperties>,
    #[doc = "The location of the resource."]
    pub location: String,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PeeringService {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            sku: None,
            properties: None,
            location,
            tags: None,
        }
    }
}
#[doc = "The peering service country."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceCountry {
    #[serde(flatten)]
    pub resource: Resource,
}
impl PeeringServiceCountry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paginated list of peering service countries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceCountryListResult {
    #[doc = "The list of peering service countries."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringServiceCountry>,
    #[doc = "The link to fetch the next page of peering service countries."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PeeringServiceCountryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PeeringServiceCountryListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paginated list of peering services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceListResult {
    #[doc = "The list of peering services."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringService>,
    #[doc = "The link to fetch the next page of peering services."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PeeringServiceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PeeringServiceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The peering service location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceLocation {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties that define connectivity to the Peering Service Location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringServiceLocationProperties>,
}
impl PeeringServiceLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paginated list of peering service locations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceLocationListResult {
    #[doc = "The list of peering service locations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringServiceLocation>,
    #[doc = "The link to fetch the next page of peering service locations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PeeringServiceLocationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PeeringServiceLocationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that define connectivity to the Peering Service Location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceLocationProperties {
    #[doc = "Country of the customer"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "State of the customer"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Azure region for the location"]
    #[serde(rename = "azureRegion", default, skip_serializing_if = "Option::is_none")]
    pub azure_region: Option<String>,
}
impl PeeringServiceLocationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The peering service prefix class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServicePrefix {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The peering service prefix properties class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringServicePrefixProperties>,
}
impl PeeringServicePrefix {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the event associated with a prefix."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServicePrefixEvent {
    #[doc = "The timestamp of the event associated with a prefix."]
    #[serde(rename = "eventTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub event_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The type of the event associated with a prefix."]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[doc = "The summary of the event associated with a prefix."]
    #[serde(rename = "eventSummary", default, skip_serializing_if = "Option::is_none")]
    pub event_summary: Option<String>,
    #[doc = "The level of the event associated with a prefix."]
    #[serde(rename = "eventLevel", default, skip_serializing_if = "Option::is_none")]
    pub event_level: Option<String>,
    #[doc = "The description of the event associated with a prefix."]
    #[serde(rename = "eventDescription", default, skip_serializing_if = "Option::is_none")]
    pub event_description: Option<String>,
}
impl PeeringServicePrefixEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paginated list of peering service prefixes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServicePrefixListResult {
    #[doc = "The list of peering service prefixes."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringServicePrefix>,
    #[doc = "The link to fetch the next page of peering service prefixes."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PeeringServicePrefixListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PeeringServicePrefixListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The peering service prefix properties class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServicePrefixProperties {
    #[doc = "The prefix from which your traffic originates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[doc = "The prefix validation state"]
    #[serde(rename = "prefixValidationState", default, skip_serializing_if = "Option::is_none")]
    pub prefix_validation_state: Option<peering_service_prefix_properties::PrefixValidationState>,
    #[doc = "The prefix learned type"]
    #[serde(rename = "learnedType", default, skip_serializing_if = "Option::is_none")]
    pub learned_type: Option<peering_service_prefix_properties::LearnedType>,
    #[doc = "The error message for validation state"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The list of events for peering service prefix"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<PeeringServicePrefixEvent>,
    #[doc = "The peering service prefix key"]
    #[serde(rename = "peeringServicePrefixKey", default, skip_serializing_if = "Option::is_none")]
    pub peering_service_prefix_key: Option<String>,
    #[doc = "The provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<peering_service_prefix_properties::ProvisioningState>,
}
impl PeeringServicePrefixProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peering_service_prefix_properties {
    use super::*;
    #[doc = "The prefix validation state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrefixValidationState")]
    pub enum PrefixValidationState {
        None,
        Invalid,
        Verified,
        Failed,
        Pending,
        Warning,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrefixValidationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrefixValidationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrefixValidationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("PrefixValidationState", 0u32, "None"),
                Self::Invalid => serializer.serialize_unit_variant("PrefixValidationState", 1u32, "Invalid"),
                Self::Verified => serializer.serialize_unit_variant("PrefixValidationState", 2u32, "Verified"),
                Self::Failed => serializer.serialize_unit_variant("PrefixValidationState", 3u32, "Failed"),
                Self::Pending => serializer.serialize_unit_variant("PrefixValidationState", 4u32, "Pending"),
                Self::Warning => serializer.serialize_unit_variant("PrefixValidationState", 5u32, "Warning"),
                Self::Unknown => serializer.serialize_unit_variant("PrefixValidationState", 6u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The prefix learned type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LearnedType")]
    pub enum LearnedType {
        None,
        ViaServiceProvider,
        ViaSession,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LearnedType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LearnedType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LearnedType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("LearnedType", 0u32, "None"),
                Self::ViaServiceProvider => serializer.serialize_unit_variant("LearnedType", 1u32, "ViaServiceProvider"),
                Self::ViaSession => serializer.serialize_unit_variant("LearnedType", 2u32, "ViaSession"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Updating,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties that define connectivity to the Peering Service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceProperties {
    #[doc = "The location (state/province) of the customer."]
    #[serde(rename = "peeringServiceLocation", default, skip_serializing_if = "Option::is_none")]
    pub peering_service_location: Option<String>,
    #[doc = "The name of the service provider."]
    #[serde(rename = "peeringServiceProvider", default, skip_serializing_if = "Option::is_none")]
    pub peering_service_provider: Option<String>,
    #[doc = "The provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<peering_service_properties::ProvisioningState>,
    #[doc = "The primary peering (Microsoft/service provider) location to be used for customer traffic."]
    #[serde(rename = "providerPrimaryPeeringLocation", default, skip_serializing_if = "Option::is_none")]
    pub provider_primary_peering_location: Option<String>,
    #[doc = "The backup peering (Microsoft/service provider) location to be used for customer traffic."]
    #[serde(rename = "providerBackupPeeringLocation", default, skip_serializing_if = "Option::is_none")]
    pub provider_backup_peering_location: Option<String>,
}
impl PeeringServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peering_service_properties {
    use super::*;
    #[doc = "The provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Updating,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "PeeringService provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceProvider {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties that define connectivity to the Peering Service Provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringServiceProviderProperties>,
}
impl PeeringServiceProvider {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paginated list of peering service providers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceProviderListResult {
    #[doc = "The list of peering service providers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringServiceProvider>,
    #[doc = "The link to fetch the next page of peering service providers."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PeeringServiceProviderListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PeeringServiceProviderListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that define connectivity to the Peering Service Provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceProviderProperties {
    #[doc = "The name of the service provider."]
    #[serde(rename = "serviceProviderName", default, skip_serializing_if = "Option::is_none")]
    pub service_provider_name: Option<String>,
    #[doc = "The list of locations at which the service provider peers with Microsoft."]
    #[serde(rename = "peeringLocations", default, skip_serializing_if = "Vec::is_empty")]
    pub peering_locations: Vec<String>,
}
impl PeeringServiceProviderProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SKU that defines the type of the peering service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringServiceSku {
    #[doc = "The name of the peering service SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl PeeringServiceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SKU that defines the tier and kind of the peering."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeeringSku {
    #[doc = "The name of the peering SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The tier of the peering SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<peering_sku::Tier>,
    #[doc = "The family of the peering SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<peering_sku::Family>,
    #[doc = "The size of the peering SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<peering_sku::Size>,
}
impl PeeringSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod peering_sku {
    use super::*;
    #[doc = "The tier of the peering SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Basic,
        Premium,
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
                Self::Premium => serializer.serialize_unit_variant("Tier", 1u32, "Premium"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The family of the peering SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Family")]
    pub enum Family {
        Direct,
        Exchange,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Family {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Family {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Family {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Direct => serializer.serialize_unit_variant("Family", 0u32, "Direct"),
                Self::Exchange => serializer.serialize_unit_variant("Family", 1u32, "Exchange"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The size of the peering SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Size")]
    pub enum Size {
        Free,
        Metered,
        Unlimited,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Size {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Size {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Size {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Free => serializer.serialize_unit_variant("Size", 0u32, "Free"),
                Self::Metered => serializer.serialize_unit_variant("Size", 1u32, "Metered"),
                Self::Unlimited => serializer.serialize_unit_variant("Size", 2u32, "Unlimited"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The ARM resource class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The ID of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTags {
    #[doc = "Gets or sets the tags, a dictionary of descriptors arm object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceTags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The sub resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "The identifier of the referenced resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}

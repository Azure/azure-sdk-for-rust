#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Authorized groundstation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizedGroundstation {
    #[doc = "Groundstation name."]
    #[serde(rename = "groundStation", default, skip_serializing_if = "Option::is_none")]
    pub ground_station: Option<String>,
    #[doc = "Date of authorization expiration."]
    #[serde(rename = "expirationDate", default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
}
impl AuthorizedGroundstation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Customer retrieves list of Available Contacts for a spacecraft resource. Later, one of the available contact can be selected to create a contact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableContacts {
    #[doc = "The reference to the spacecraft resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spacecraft: Option<serde_json::Value>,
    #[doc = "Name of Azure Ground Station."]
    #[serde(rename = "groundStationName", default, skip_serializing_if = "Option::is_none")]
    pub ground_station_name: Option<String>,
    #[doc = "Properties of Contact resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AvailableContacts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for the ListAvailableContacts API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableContactsListResult {
    #[doc = "A list of available contacts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AvailableContacts>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvailableContactsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AvailableContactsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Ground Stations available to schedule Contacts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableGroundStation {
    #[doc = "ID of groundStation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the ground station."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure region."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The properties bag for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AvailableGroundStation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for the AvailableGroundStations API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableGroundStationListResult {
    #[doc = "A list of ground station resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AvailableGroundStation>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvailableGroundStationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AvailableGroundStationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties object for Available groundstation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableGroundStationProperties {
    #[doc = "City of ground station."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Ground station provider name."]
    #[serde(rename = "providerName", default, skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[doc = "Longitude of the ground station in decimal degrees."]
    #[serde(rename = "longitudeDegrees", default, skip_serializing_if = "Option::is_none")]
    pub longitude_degrees: Option<f64>,
    #[doc = "Latitude of the ground station in decimal degrees."]
    #[serde(rename = "latitudeDegrees", default, skip_serializing_if = "Option::is_none")]
    pub latitude_degrees: Option<f64>,
    #[doc = "Altitude of the ground station."]
    #[serde(rename = "altitudeMeters", default, skip_serializing_if = "Option::is_none")]
    pub altitude_meters: Option<f64>,
    #[doc = "Release Status of a ground station."]
    #[serde(rename = "releaseMode", default, skip_serializing_if = "Option::is_none")]
    pub release_mode: Option<available_ground_station_properties::ReleaseMode>,
}
impl AvailableGroundStationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod available_ground_station_properties {
    use super::*;
    #[doc = "Release Status of a ground station."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReleaseMode")]
    pub enum ReleaseMode {
        Preview,
        #[serde(rename = "GA")]
        Ga,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReleaseMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReleaseMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReleaseMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Preview => serializer.serialize_unit_variant("ReleaseMode", 0u32, "Preview"),
                Self::Ga => serializer.serialize_unit_variant("ReleaseMode", 1u32, "GA"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Capability of the Ground Station."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Capability")]
pub enum Capability {
    EarthObservation,
    Communication,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Capability {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Capability {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Capability {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EarthObservation => serializer.serialize_unit_variant("Capability", 0u32, "EarthObservation"),
            Self::Communication => serializer.serialize_unit_variant("Capability", 1u32, "Communication"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "Customer creates a contact resource for a spacecraft resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Contact {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the Contact Resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ContactsProperties>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Contact {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contact Instance Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContactInstanceProperties {
    #[doc = "Maximum elevation of the antenna during the contact in decimal degrees."]
    #[serde(rename = "maximumElevationDegrees", default, skip_serializing_if = "Option::is_none")]
    pub maximum_elevation_degrees: Option<f64>,
    #[doc = "Time at which antenna transmit will be enabled (ISO 8601 UTC standard)."]
    #[serde(rename = "txStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub tx_start_time: Option<time::OffsetDateTime>,
    #[doc = "Time at which antenna transmit will be disabled (ISO 8601 UTC standard)."]
    #[serde(rename = "txEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub tx_end_time: Option<time::OffsetDateTime>,
    #[doc = "Earliest time to receive a signal (ISO 8601 UTC standard)."]
    #[serde(rename = "rxStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub rx_start_time: Option<time::OffsetDateTime>,
    #[doc = "Time to lost receiving a signal (ISO 8601 UTC standard)."]
    #[serde(rename = "rxEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub rx_end_time: Option<time::OffsetDateTime>,
    #[doc = "Azimuth of the antenna at the start of the contact in decimal degrees."]
    #[serde(rename = "startAzimuthDegrees", default, skip_serializing_if = "Option::is_none")]
    pub start_azimuth_degrees: Option<f64>,
    #[doc = "Azimuth of the antenna at the end of the contact in decimal degrees."]
    #[serde(rename = "endAzimuthDegrees", default, skip_serializing_if = "Option::is_none")]
    pub end_azimuth_degrees: Option<f64>,
    #[doc = "Spacecraft elevation above the horizon at contact start."]
    #[serde(rename = "startElevationDegrees", default, skip_serializing_if = "Option::is_none")]
    pub start_elevation_degrees: Option<f64>,
    #[doc = "Spacecraft elevation above the horizon at contact end."]
    #[serde(rename = "endElevationDegrees", default, skip_serializing_if = "Option::is_none")]
    pub end_elevation_degrees: Option<f64>,
}
impl ContactInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for the ListContacts API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContactListResult {
    #[doc = "A list of contact resources in a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Contact>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ContactListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ContactListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters that define the contact resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactParameters {
    #[doc = "The reference to the contact profile resource."]
    #[serde(rename = "contactProfile")]
    pub contact_profile: serde_json::Value,
    #[doc = "Name of Azure Ground Station."]
    #[serde(rename = "groundStationName")]
    pub ground_station_name: String,
    #[doc = "Start time of a contact (ISO 8601 UTC standard)."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "End time of a contact (ISO 8601 UTC standard)."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339")]
    pub end_time: time::OffsetDateTime,
}
impl ContactParameters {
    pub fn new(
        contact_profile: serde_json::Value,
        ground_station_name: String,
        start_time: time::OffsetDateTime,
        end_time: time::OffsetDateTime,
    ) -> Self {
        Self {
            contact_profile,
            ground_station_name,
            start_time,
            end_time,
        }
    }
}
#[doc = "Customer creates a Contact Profile Resource, which will contain all of the configurations required for scheduling a contact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactProfile {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of the contact profile resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ContactProfile {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            etag: None,
        }
    }
}
#[doc = "Contact Profile Link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactProfileLink {
    #[doc = "Link name."]
    pub name: String,
    #[doc = "Polarization. e.g. (RHCP, LHCP)."]
    pub polarization: contact_profile_link::Polarization,
    #[doc = "Direction (uplink or downlink)."]
    pub direction: contact_profile_link::Direction,
    #[doc = "Gain To Noise Temperature in db/K. It is the required G/T by the customer. Not used yet."]
    #[serde(rename = "gainOverTemperature", default, skip_serializing_if = "Option::is_none")]
    pub gain_over_temperature: Option<f64>,
    #[doc = "Effective Isotropic Radiated Power (EIRP) in dBW. It is the required EIRP by the customer. Not used yet."]
    #[serde(rename = "eirpdBW", default, skip_serializing_if = "Option::is_none")]
    pub eirpd_bw: Option<f64>,
    #[doc = "Contact Profile Link Channel."]
    pub channels: Vec<ContactProfileLinkChannel>,
}
impl ContactProfileLink {
    pub fn new(
        name: String,
        polarization: contact_profile_link::Polarization,
        direction: contact_profile_link::Direction,
        channels: Vec<ContactProfileLinkChannel>,
    ) -> Self {
        Self {
            name,
            polarization,
            direction,
            gain_over_temperature: None,
            eirpd_bw: None,
            channels,
        }
    }
}
pub mod contact_profile_link {
    use super::*;
    #[doc = "Polarization. e.g. (RHCP, LHCP)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Polarization")]
    pub enum Polarization {
        #[serde(rename = "RHCP")]
        Rhcp,
        #[serde(rename = "LHCP")]
        Lhcp,
        #[serde(rename = "linearVertical")]
        LinearVertical,
        #[serde(rename = "linearHorizontal")]
        LinearHorizontal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Polarization {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Polarization {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Polarization {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Rhcp => serializer.serialize_unit_variant("Polarization", 0u32, "RHCP"),
                Self::Lhcp => serializer.serialize_unit_variant("Polarization", 1u32, "LHCP"),
                Self::LinearVertical => serializer.serialize_unit_variant("Polarization", 2u32, "linearVertical"),
                Self::LinearHorizontal => serializer.serialize_unit_variant("Polarization", 3u32, "linearHorizontal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Direction (uplink or downlink)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Direction")]
    pub enum Direction {
        #[serde(rename = "uplink")]
        Uplink,
        #[serde(rename = "downlink")]
        Downlink,
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
                Self::Uplink => serializer.serialize_unit_variant("Direction", 0u32, "uplink"),
                Self::Downlink => serializer.serialize_unit_variant("Direction", 1u32, "downlink"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Contact Profile Link Channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactProfileLinkChannel {
    #[doc = "Channel name."]
    pub name: String,
    #[doc = "Center Frequency in MHz."]
    #[serde(rename = "centerFrequencyMHz")]
    pub center_frequency_m_hz: f64,
    #[doc = "Bandwidth in MHz."]
    #[serde(rename = "bandwidthMHz")]
    pub bandwidth_m_hz: f64,
    #[doc = "Customer End point to store/retrieve data during a contact."]
    #[serde(rename = "endPoint")]
    pub end_point: EndPoint,
    #[doc = "Copy of the modem configuration file such as Kratos QRadio. Only valid for uplink directions. If provided, the modem connects to the customer endpoint and accepts commands from the customer instead of a VITA.49 stream."]
    #[serde(rename = "modulationConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub modulation_configuration: Option<String>,
    #[doc = "Copy of the modem configuration file such as Kratos QRadio or Kratos QuantumRx. Only valid for downlink directions. If provided, the modem connects to the customer endpoint and sends demodulated data instead of a VITA.49 stream."]
    #[serde(rename = "demodulationConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub demodulation_configuration: Option<String>,
    #[doc = "Currently unused."]
    #[serde(rename = "encodingConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub encoding_configuration: Option<String>,
    #[doc = "Currently unused."]
    #[serde(rename = "decodingConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub decoding_configuration: Option<String>,
}
impl ContactProfileLinkChannel {
    pub fn new(name: String, center_frequency_m_hz: f64, bandwidth_m_hz: f64, end_point: EndPoint) -> Self {
        Self {
            name,
            center_frequency_m_hz,
            bandwidth_m_hz,
            end_point,
            modulation_configuration: None,
            demodulation_configuration: None,
            encoding_configuration: None,
            decoding_configuration: None,
        }
    }
}
#[doc = "Response for the ListContactProfiles API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContactProfileListResult {
    #[doc = "A list of contact profile resources in a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ContactProfile>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ContactProfileListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ContactProfileListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Contact Profile Resource Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactProfilesProperties {
    #[doc = "The current state of the resource's creation, deletion, or modification."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Minimum viable contact duration in ISO 8601 format. Used for listing the available contacts with a spacecraft at a given ground station."]
    #[serde(rename = "minimumViableContactDuration", default, skip_serializing_if = "Option::is_none")]
    pub minimum_viable_contact_duration: Option<String>,
    #[doc = "Minimum viable elevation for the contact in decimal degrees. Used for listing the available contacts with a spacecraft at a given ground station."]
    #[serde(rename = "minimumElevationDegrees", default, skip_serializing_if = "Option::is_none")]
    pub minimum_elevation_degrees: Option<f64>,
    #[doc = "Auto-tracking configuration."]
    #[serde(rename = "autoTrackingConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub auto_tracking_configuration: Option<contact_profiles_properties::AutoTrackingConfiguration>,
    #[doc = "ARM resource identifier of the Event Hub used for telemetry. Requires granting Orbital Resource Provider the rights to send telemetry into the hub."]
    #[serde(rename = "eventHubUri", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_uri: Option<String>,
    #[doc = "Network configuration of customer virtual network."]
    #[serde(rename = "networkConfiguration")]
    pub network_configuration: contact_profiles_properties::NetworkConfiguration,
    #[doc = "Links of the Contact Profile. Describes RF links, modem processing, and IP endpoints."]
    pub links: Vec<ContactProfileLink>,
}
impl ContactProfilesProperties {
    pub fn new(network_configuration: contact_profiles_properties::NetworkConfiguration, links: Vec<ContactProfileLink>) -> Self {
        Self {
            provisioning_state: None,
            minimum_viable_contact_duration: None,
            minimum_elevation_degrees: None,
            auto_tracking_configuration: None,
            event_hub_uri: None,
            network_configuration,
            links,
        }
    }
}
pub mod contact_profiles_properties {
    use super::*;
    #[doc = "Auto-tracking configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AutoTrackingConfiguration {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "xBand")]
        XBand,
        #[serde(rename = "sBand")]
        SBand,
    }
    #[doc = "Network configuration of customer virtual network."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct NetworkConfiguration {
        #[doc = "ARM resource identifier of the subnet delegated to the Microsoft.Orbital/orbitalGateways. Needs to be at least a class C subnet, and should not have any IP created in it."]
        #[serde(rename = "subnetId")]
        pub subnet_id: String,
    }
    impl NetworkConfiguration {
        pub fn new(subnet_id: String) -> Self {
            Self { subnet_id }
        }
    }
}
#[doc = "Properties of the Contact Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactsProperties {
    #[doc = "The current state of the resource's creation, deletion, or modification."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Status of a contact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<contacts_properties::Status>,
    #[doc = "Reservation start time of a contact (ISO 8601 UTC standard)."]
    #[serde(rename = "reservationStartTime", with = "azure_core::date::rfc3339")]
    pub reservation_start_time: time::OffsetDateTime,
    #[doc = "Reservation end time of a contact (ISO 8601 UTC standard)."]
    #[serde(rename = "reservationEndTime", with = "azure_core::date::rfc3339")]
    pub reservation_end_time: time::OffsetDateTime,
    #[doc = "Receive start time of a contact (ISO 8601 UTC standard)."]
    #[serde(rename = "rxStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub rx_start_time: Option<time::OffsetDateTime>,
    #[doc = "Receive end time of a contact (ISO 8601 UTC standard)."]
    #[serde(rename = "rxEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub rx_end_time: Option<time::OffsetDateTime>,
    #[doc = "Transmit start time of a contact (ISO 8601 UTC standard)."]
    #[serde(rename = "txStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub tx_start_time: Option<time::OffsetDateTime>,
    #[doc = "Transmit end time of a contact (ISO 8601 UTC standard)."]
    #[serde(rename = "txEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub tx_end_time: Option<time::OffsetDateTime>,
    #[doc = "Any error message while scheduling a contact."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Maximum elevation of the antenna during the contact in decimal degrees."]
    #[serde(rename = "maximumElevationDegrees", default, skip_serializing_if = "Option::is_none")]
    pub maximum_elevation_degrees: Option<f64>,
    #[doc = "Azimuth of the antenna at the start of the contact in decimal degrees."]
    #[serde(rename = "startAzimuthDegrees", default, skip_serializing_if = "Option::is_none")]
    pub start_azimuth_degrees: Option<f64>,
    #[doc = "Azimuth of the antenna at the end of the contact in decimal degrees."]
    #[serde(rename = "endAzimuthDegrees", default, skip_serializing_if = "Option::is_none")]
    pub end_azimuth_degrees: Option<f64>,
    #[doc = "Azure Ground Station name."]
    #[serde(rename = "groundStationName")]
    pub ground_station_name: String,
    #[doc = "Spacecraft elevation above the horizon at contact start."]
    #[serde(rename = "startElevationDegrees", default, skip_serializing_if = "Option::is_none")]
    pub start_elevation_degrees: Option<f64>,
    #[doc = "Spacecraft elevation above the horizon at contact end."]
    #[serde(rename = "endElevationDegrees", default, skip_serializing_if = "Option::is_none")]
    pub end_elevation_degrees: Option<f64>,
    #[doc = "The configuration associated with the allocated antenna."]
    #[serde(rename = "antennaConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub antenna_configuration: Option<contacts_properties::AntennaConfiguration>,
    #[doc = "The reference to the contact profile resource."]
    #[serde(rename = "contactProfile")]
    pub contact_profile: serde_json::Value,
}
impl ContactsProperties {
    pub fn new(
        reservation_start_time: time::OffsetDateTime,
        reservation_end_time: time::OffsetDateTime,
        ground_station_name: String,
        contact_profile: serde_json::Value,
    ) -> Self {
        Self {
            provisioning_state: None,
            status: None,
            reservation_start_time,
            reservation_end_time,
            rx_start_time: None,
            rx_end_time: None,
            tx_start_time: None,
            tx_end_time: None,
            error_message: None,
            maximum_elevation_degrees: None,
            start_azimuth_degrees: None,
            end_azimuth_degrees: None,
            ground_station_name,
            start_elevation_degrees: None,
            end_elevation_degrees: None,
            antenna_configuration: None,
            contact_profile,
        }
    }
}
pub mod contacts_properties {
    use super::*;
    #[doc = "Status of a contact."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "scheduled")]
        Scheduled,
        #[serde(rename = "cancelled")]
        Cancelled,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "providerCancelled")]
        ProviderCancelled,
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
                Self::Scheduled => serializer.serialize_unit_variant("Status", 0u32, "scheduled"),
                Self::Cancelled => serializer.serialize_unit_variant("Status", 1u32, "cancelled"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 2u32, "succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 3u32, "failed"),
                Self::ProviderCancelled => serializer.serialize_unit_variant("Status", 4u32, "providerCancelled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The configuration associated with the allocated antenna."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct AntennaConfiguration {
        #[doc = "The destination IP a packet can be sent to. This would for example be the TCP endpoint you would send data to."]
        #[serde(rename = "destinationIp", default, skip_serializing_if = "Option::is_none")]
        pub destination_ip: Option<String>,
        #[doc = "List of Source IP"]
        #[serde(rename = "sourceIps", default, skip_serializing_if = "Vec::is_empty")]
        pub source_ips: Vec<String>,
    }
    impl AntennaConfiguration {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Customer End point to store/retrieve data during a contact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndPoint {
    #[doc = "IP Address."]
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    #[doc = "Name of an end point."]
    #[serde(rename = "endPointName")]
    pub end_point_name: String,
    #[doc = "TCP port to listen on to receive data."]
    pub port: String,
    #[doc = "Protocol either UDP or TCP."]
    pub protocol: end_point::Protocol,
}
impl EndPoint {
    pub fn new(ip_address: String, end_point_name: String, port: String, protocol: end_point::Protocol) -> Self {
        Self {
            ip_address,
            end_point_name,
            port,
            protocol,
        }
    }
}
pub mod end_point {
    use super::*;
    #[doc = "Protocol either UDP or TCP."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
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
                Self::Tcp => serializer.serialize_unit_variant("Protocol", 0u32, "TCP"),
                Self::Udp => serializer.serialize_unit_variant("Protocol", 1u32, "UDP"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type Etag = String;
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation Result Entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[doc = "ID of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[doc = "The operation start time (ISO 8601 UTC standard)."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The operation end time (ISO 8601 UTC standard)."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Percentage completed."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "Operation result properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationResultProperties>,
    #[doc = "Operation result error properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationResultErrorProperties>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation result error properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResultErrorProperties {
    #[doc = "The code of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl OperationResultErrorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation result properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResultProperties {}
impl OperationResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current state of the resource's creation, deletion, or modification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Creating,
    Succeeded,
    Failed,
    Canceled,
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
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for an API service call that lists the resource IDs of resources associated with another resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceIdListResult {
    #[doc = "A list of Azure Resource IDs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<serde_json::Value>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ResourceIdListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource Reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceReference {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Customer creates a spacecraft resource to schedule a contact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Spacecraft {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "List of Spacecraft Resource Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SpacecraftsProperties>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Spacecraft {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            etag: None,
        }
    }
}
#[doc = "List of authorized spacecraft links per ground station and the expiration date of the authorization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpacecraftLink {
    #[doc = "Link name."]
    pub name: String,
    #[doc = "Center Frequency in MHz."]
    #[serde(rename = "centerFrequencyMHz")]
    pub center_frequency_m_hz: f64,
    #[doc = "Bandwidth in MHz."]
    #[serde(rename = "bandwidthMHz")]
    pub bandwidth_m_hz: f64,
    #[doc = "Direction (uplink or downlink)."]
    pub direction: spacecraft_link::Direction,
    #[doc = "Polarization. e.g. (RHCP, LHCP)."]
    pub polarization: spacecraft_link::Polarization,
    #[doc = "Authorized Ground Stations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorizations: Vec<AuthorizedGroundstation>,
}
impl SpacecraftLink {
    pub fn new(
        name: String,
        center_frequency_m_hz: f64,
        bandwidth_m_hz: f64,
        direction: spacecraft_link::Direction,
        polarization: spacecraft_link::Polarization,
    ) -> Self {
        Self {
            name,
            center_frequency_m_hz,
            bandwidth_m_hz,
            direction,
            polarization,
            authorizations: Vec::new(),
        }
    }
}
pub mod spacecraft_link {
    use super::*;
    #[doc = "Direction (uplink or downlink)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Direction")]
    pub enum Direction {
        #[serde(rename = "uplink")]
        Uplink,
        #[serde(rename = "downlink")]
        Downlink,
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
                Self::Uplink => serializer.serialize_unit_variant("Direction", 0u32, "uplink"),
                Self::Downlink => serializer.serialize_unit_variant("Direction", 1u32, "downlink"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Polarization. e.g. (RHCP, LHCP)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Polarization")]
    pub enum Polarization {
        #[serde(rename = "RHCP")]
        Rhcp,
        #[serde(rename = "LHCP")]
        Lhcp,
        #[serde(rename = "linearVertical")]
        LinearVertical,
        #[serde(rename = "linearHorizontal")]
        LinearHorizontal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Polarization {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Polarization {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Polarization {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Rhcp => serializer.serialize_unit_variant("Polarization", 0u32, "RHCP"),
                Self::Lhcp => serializer.serialize_unit_variant("Polarization", 1u32, "LHCP"),
                Self::LinearVertical => serializer.serialize_unit_variant("Polarization", 2u32, "linearVertical"),
                Self::LinearHorizontal => serializer.serialize_unit_variant("Polarization", 3u32, "linearHorizontal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Response for the ListSpacecrafts API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpacecraftListResult {
    #[doc = "A list of spacecraft resources in a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Spacecraft>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SpacecraftListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SpacecraftListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Spacecraft Resource Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpacecraftsProperties {
    #[doc = "The current state of the resource's creation, deletion, or modification."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "NORAD ID of the spacecraft."]
    #[serde(rename = "noradId")]
    pub norad_id: String,
    #[doc = "Title line of the two-line element set (TLE)."]
    #[serde(rename = "titleLine", default, skip_serializing_if = "Option::is_none")]
    pub title_line: Option<String>,
    #[doc = "Line 1 of the two-line element set (TLE)."]
    #[serde(rename = "tleLine1", default, skip_serializing_if = "Option::is_none")]
    pub tle_line1: Option<String>,
    #[doc = "Line 2 of the two-line element set (TLE)."]
    #[serde(rename = "tleLine2", default, skip_serializing_if = "Option::is_none")]
    pub tle_line2: Option<String>,
    #[doc = "Immutable list of Spacecraft links."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<SpacecraftLink>,
}
impl SpacecraftsProperties {
    pub fn new(norad_id: String) -> Self {
        Self {
            provisioning_state: None,
            norad_id,
            title_line: None,
            tle_line1: None,
            tle_line2: None,
            links: Vec::new(),
        }
    }
}
#[doc = "The status of operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Status")]
pub enum Status {
    Succeeded,
    Canceled,
    Failed,
    Running,
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
            Self::Succeeded => serializer.serialize_unit_variant("Status", 0u32, "Succeeded"),
            Self::Canceled => serializer.serialize_unit_variant("Status", 1u32, "Canceled"),
            Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
            Self::Running => serializer.serialize_unit_variant("Status", 3u32, "Running"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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

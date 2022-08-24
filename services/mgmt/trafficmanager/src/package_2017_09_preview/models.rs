#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Parameters supplied to check Traffic Manager name operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckTrafficManagerRelativeDnsNameAvailabilityParameters {
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckTrafficManagerRelativeDnsNameAvailabilityParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error returned by the Azure Resource Manager"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "The content of an error returned by the Azure Resource Manager"]
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
#[doc = "The content of an error returned by the Azure Resource Manager"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Error target"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Error details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of the request or operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteOperationResult {
    #[doc = "The result of the operation or request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boolean: Option<bool>,
}
impl DeleteOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class containing DNS settings in a Traffic Manager profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsConfig {
    #[doc = "The relative DNS name provided by this Traffic Manager profile. This value is combined with the DNS domain name used by Azure Traffic Manager to form the fully-qualified domain name (FQDN) of the profile."]
    #[serde(rename = "relativeName", default, skip_serializing_if = "Option::is_none")]
    pub relative_name: Option<String>,
    #[doc = "The fully-qualified domain name (FQDN) of the Traffic Manager profile. This is formed from the concatenation of the RelativeName with the DNS domain used by Azure Traffic Manager."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "The DNS Time-To-Live (TTL), in seconds. This informs the local DNS resolvers and DNS clients how long to cache DNS responses provided by this Traffic Manager profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
}
impl DnsConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing a Traffic Manager endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Endpoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class representing a Traffic Manager endpoint properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EndpointProperties>,
}
impl Endpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing a Traffic Manager endpoint properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointProperties {
    #[doc = "The Azure Resource URI of the of the endpoint. Not applicable to endpoints of type 'ExternalEndpoints'."]
    #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
    #[doc = "The fully-qualified DNS name of the endpoint. Traffic Manager returns this value in DNS responses to direct traffic to this endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The status of the endpoint. If the endpoint is Enabled, it is probed for endpoint health and is included in the traffic routing method."]
    #[serde(rename = "endpointStatus", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<endpoint_properties::EndpointStatus>,
    #[doc = "The weight of this endpoint when using the 'Weighted' traffic routing method. Possible values are from 1 to 1000."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
    #[doc = "The priority of this endpoint when using the ‘Priority’ traffic routing method. Possible values are from 1 to 1000, lower values represent higher priority. This is an optional parameter.  If specified, it must be specified on all endpoints, and no two endpoints can share the same priority value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[doc = "Specifies the location of the external or nested endpoints when using the ‘Performance’ traffic routing method."]
    #[serde(rename = "endpointLocation", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_location: Option<String>,
    #[doc = "The monitoring status of the endpoint."]
    #[serde(rename = "endpointMonitorStatus", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_monitor_status: Option<endpoint_properties::EndpointMonitorStatus>,
    #[doc = "The minimum number of endpoints that must be available in the child profile in order for the parent profile to be considered available. Only applicable to endpoint of type 'NestedEndpoints'."]
    #[serde(rename = "minChildEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub min_child_endpoints: Option<i64>,
    #[doc = "The list of countries/regions mapped to this endpoint when using the ‘Geographic’ traffic routing method. Please consult Traffic Manager Geographic documentation for a full list of accepted values."]
    #[serde(rename = "geoMapping", default, skip_serializing_if = "Vec::is_empty")]
    pub geo_mapping: Vec<String>,
}
impl EndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod endpoint_properties {
    use super::*;
    #[doc = "The status of the endpoint. If the endpoint is Enabled, it is probed for endpoint health and is included in the traffic routing method."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EndpointStatus")]
    pub enum EndpointStatus {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EndpointStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EndpointStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EndpointStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("EndpointStatus", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("EndpointStatus", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The monitoring status of the endpoint."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EndpointMonitorStatus")]
    pub enum EndpointMonitorStatus {
        CheckingEndpoint,
        Online,
        Degraded,
        Disabled,
        Inactive,
        Stopped,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EndpointMonitorStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EndpointMonitorStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EndpointMonitorStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CheckingEndpoint => serializer.serialize_unit_variant("EndpointMonitorStatus", 0u32, "CheckingEndpoint"),
                Self::Online => serializer.serialize_unit_variant("EndpointMonitorStatus", 1u32, "Online"),
                Self::Degraded => serializer.serialize_unit_variant("EndpointMonitorStatus", 2u32, "Degraded"),
                Self::Disabled => serializer.serialize_unit_variant("EndpointMonitorStatus", 3u32, "Disabled"),
                Self::Inactive => serializer.serialize_unit_variant("EndpointMonitorStatus", 4u32, "Inactive"),
                Self::Stopped => serializer.serialize_unit_variant("EndpointMonitorStatus", 5u32, "Stopped"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Class representing the properties of the Geographic hierarchy used with the Geographic traffic routing method."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GeographicHierarchyProperties {
    #[doc = "Class representing a region in the Geographic hierarchy used with the Geographic traffic routing method."]
    #[serde(rename = "geographicHierarchy", default, skip_serializing_if = "Option::is_none")]
    pub geographic_hierarchy: Option<Region>,
}
impl GeographicHierarchyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class which is a sparse representation of a Traffic Manager endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HeatMapEndpoint {
    #[doc = "The ARM Resource ID of this Traffic Manager endpoint."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "A number uniquely identifying this endpoint in query experiences."]
    #[serde(rename = "endpointId", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<i64>,
}
impl HeatMapEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing a Traffic Manager HeatMap."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HeatMapModel {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class representing a Traffic Manager HeatMap properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HeatMapProperties>,
}
impl HeatMapModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing a Traffic Manager HeatMap properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HeatMapProperties {
    #[doc = "The beginning of the time window for this HeatMap, inclusive."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The ending of the time window for this HeatMap, exclusive."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The endpoints used in this HeatMap calculation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<HeatMapEndpoint>,
    #[doc = "The traffic flows produced in this HeatMap calculation."]
    #[serde(rename = "trafficFlows", default, skip_serializing_if = "Vec::is_empty")]
    pub traffic_flows: Vec<TrafficFlow>,
}
impl HeatMapProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class containing endpoint monitoring settings in a Traffic Manager profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorConfig {
    #[doc = "The profile-level monitoring status of the Traffic Manager profile."]
    #[serde(rename = "profileMonitorStatus", default, skip_serializing_if = "Option::is_none")]
    pub profile_monitor_status: Option<monitor_config::ProfileMonitorStatus>,
    #[doc = "The protocol (HTTP, HTTPS or TCP) used to probe for endpoint health."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<monitor_config::Protocol>,
    #[doc = "The TCP port used to probe for endpoint health."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[doc = "The path relative to the endpoint domain name used to probe for endpoint health."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The monitor interval for endpoints in this profile. This is the interval at which Traffic Manager will check the health of each endpoint in this profile."]
    #[serde(rename = "intervalInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i64>,
    #[doc = "The monitor timeout for endpoints in this profile. This is the time that Traffic Manager allows endpoints in this profile to response to the health check."]
    #[serde(rename = "timeoutInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
    #[doc = "The number of consecutive failed health check that Traffic Manager tolerates before declaring an endpoint in this profile Degraded after the next failed health check."]
    #[serde(rename = "toleratedNumberOfFailures", default, skip_serializing_if = "Option::is_none")]
    pub tolerated_number_of_failures: Option<i64>,
}
impl MonitorConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod monitor_config {
    use super::*;
    #[doc = "The profile-level monitoring status of the Traffic Manager profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProfileMonitorStatus")]
    pub enum ProfileMonitorStatus {
        CheckingEndpoints,
        Online,
        Degraded,
        Disabled,
        Inactive,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProfileMonitorStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProfileMonitorStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProfileMonitorStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CheckingEndpoints => serializer.serialize_unit_variant("ProfileMonitorStatus", 0u32, "CheckingEndpoints"),
                Self::Online => serializer.serialize_unit_variant("ProfileMonitorStatus", 1u32, "Online"),
                Self::Degraded => serializer.serialize_unit_variant("ProfileMonitorStatus", 2u32, "Degraded"),
                Self::Disabled => serializer.serialize_unit_variant("ProfileMonitorStatus", 3u32, "Disabled"),
                Self::Inactive => serializer.serialize_unit_variant("ProfileMonitorStatus", 4u32, "Inactive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The protocol (HTTP, HTTPS or TCP) used to probe for endpoint health."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        #[serde(rename = "HTTP")]
        Http,
        #[serde(rename = "HTTPS")]
        Https,
        #[serde(rename = "TCP")]
        Tcp,
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
                Self::Http => serializer.serialize_unit_variant("Protocol", 0u32, "HTTP"),
                Self::Https => serializer.serialize_unit_variant("Protocol", 1u32, "HTTPS"),
                Self::Tcp => serializer.serialize_unit_variant("Protocol", 2u32, "TCP"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Class representing a Traffic Manager profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Profile {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Class representing the Traffic Manager profile properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProfileProperties>,
}
impl Profile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list Traffic Manager profiles operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileListResult {
    #[doc = "Gets the list of Traffic manager profiles."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Profile>,
}
impl azure_core::Continuable for ProfileListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ProfileListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the Traffic Manager profile properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileProperties {
    #[doc = "The status of the Traffic Manager profile."]
    #[serde(rename = "profileStatus", default, skip_serializing_if = "Option::is_none")]
    pub profile_status: Option<profile_properties::ProfileStatus>,
    #[doc = "The traffic routing method of the Traffic Manager profile."]
    #[serde(rename = "trafficRoutingMethod", default, skip_serializing_if = "Option::is_none")]
    pub traffic_routing_method: Option<profile_properties::TrafficRoutingMethod>,
    #[doc = "Class containing DNS settings in a Traffic Manager profile."]
    #[serde(rename = "dnsConfig", default, skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<DnsConfig>,
    #[doc = "Class containing endpoint monitoring settings in a Traffic Manager profile."]
    #[serde(rename = "monitorConfig", default, skip_serializing_if = "Option::is_none")]
    pub monitor_config: Option<MonitorConfig>,
    #[doc = "The list of endpoints in the Traffic Manager profile."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<Endpoint>,
}
impl ProfileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod profile_properties {
    use super::*;
    #[doc = "The status of the Traffic Manager profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProfileStatus")]
    pub enum ProfileStatus {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProfileStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProfileStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProfileStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("ProfileStatus", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("ProfileStatus", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The traffic routing method of the Traffic Manager profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TrafficRoutingMethod")]
    pub enum TrafficRoutingMethod {
        Performance,
        Priority,
        Weighted,
        Geographic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TrafficRoutingMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TrafficRoutingMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TrafficRoutingMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Performance => serializer.serialize_unit_variant("TrafficRoutingMethod", 0u32, "Performance"),
                Self::Priority => serializer.serialize_unit_variant("TrafficRoutingMethod", 1u32, "Priority"),
                Self::Weighted => serializer.serialize_unit_variant("TrafficRoutingMethod", 2u32, "Weighted"),
                Self::Geographic => serializer.serialize_unit_variant("TrafficRoutingMethod", 3u32, "Geographic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The resource model definition for a ARM proxy resource. It will have everything other than required location and tags"]
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
#[doc = "Class representing a Traffic Manager HeatMap query experience properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryExperience {
    #[doc = "The id of the endpoint from the 'endpoints' array which these queries were routed to."]
    #[serde(rename = "endpointId")]
    pub endpoint_id: i64,
    #[doc = "The number of queries originating from this location."]
    #[serde(rename = "queryCount")]
    pub query_count: i64,
    #[doc = "The latency experienced by queries originating from this location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency: Option<f64>,
}
impl QueryExperience {
    pub fn new(endpoint_id: i64, query_count: i64) -> Self {
        Self {
            endpoint_id,
            query_count,
            latency: None,
        }
    }
}
#[doc = "Class representing a region in the Geographic hierarchy used with the Geographic traffic routing method."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Region {
    #[doc = "The code of the region"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The name of the region"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of Regions grouped under this Region in the Geographic Hierarchy."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<Region>,
}
impl Region {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The core properties of ARM resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource Id for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/trafficManagerProfiles/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. Ex- Microsoft.Network/trafficManagerProfiles."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
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
#[doc = "Class representing a Traffic Manager HeatMap traffic flow properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrafficFlow {
    #[doc = "The IP address that this query experience originated from."]
    #[serde(rename = "sourceIp", default, skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
    #[doc = "The approximate latitude that these queries originated from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[doc = "The approximate longitude that these queries originated from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[doc = "The query experiences produced in this HeatMap calculation."]
    #[serde(rename = "queryExperiences", default, skip_serializing_if = "Vec::is_empty")]
    pub query_experiences: Vec<QueryExperience>,
}
impl TrafficFlow {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the Geographic hierarchy used with the Geographic traffic routing method."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrafficManagerGeographicHierarchy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class representing the properties of the Geographic hierarchy used with the Geographic traffic routing method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GeographicHierarchyProperties>,
}
impl TrafficManagerGeographicHierarchy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing a Traffic Manager Name Availability response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrafficManagerNameAvailability {
    #[doc = "The relative name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Traffic Manager profile resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Describes whether the relative name is available or not."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason why the name is not available, when applicable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Descriptive message that explains why the name is not available, when applicable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl TrafficManagerNameAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing a Traffic Manager Real User Metrics key response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrafficManagerUserMetricsKeyModel {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Fully qualified resource Id for the resource. Ex - /providers/Microsoft.Network/trafficManagerUserMetricsKeys"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The word default."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. Ex- Microsoft.Network/trafficManagerUserMetricsKeys."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The key returned by the Real User Metrics operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl TrafficManagerUserMetricsKeyModel {
    pub fn new() -> Self {
        Self::default()
    }
}

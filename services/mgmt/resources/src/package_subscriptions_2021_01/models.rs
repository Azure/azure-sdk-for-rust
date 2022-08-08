#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "List of availability zones shared by the subscriptions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityZonePeers {
    #[doc = "The availabilityZone."]
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc = "Details of shared availability zone."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub peers: Vec<Peers>,
}
impl AvailabilityZonePeers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource Name valid if not a reserved word, does not contain a reserved word and does not start with a reserved word"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckResourceNameResult {
    #[doc = "Name of Resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of Resource"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Is the resource name Allowed or Reserved"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<check_resource_name_result::Status>,
}
impl CheckResourceNameResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_resource_name_result {
    use super::*;
    #[doc = "Is the resource name Allowed or Reserved"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Allowed,
        Reserved,
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
                Self::Allowed => serializer.serialize_unit_variant("Status", 0u32, "Allowed"),
                Self::Reserved => serializer.serialize_unit_variant("Status", 1u32, "Reserved"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Check zone peers request parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckZonePeersRequest {
    #[doc = "The Microsoft location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The peer Microsoft Azure subscription ID."]
    #[serde(rename = "subscriptionIds", default, skip_serializing_if = "Vec::is_empty")]
    pub subscription_ids: Vec<String>,
}
impl CheckZonePeersRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the Check zone peers operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckZonePeersResult {
    #[doc = "The subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "the location of the subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The Availability Zones shared by the subscriptions."]
    #[serde(rename = "availabilityZonePeers", default, skip_serializing_if = "Vec::is_empty")]
    pub availability_zone_peers: Vec<AvailabilityZonePeers>,
}
impl CheckZonePeersResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response for a resource management request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
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
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
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
    pub details: Vec<ErrorResponse>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Location information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Location {
    #[doc = "The fully qualified ID of the location. For example, /subscriptions/00000000-0000-0000-0000-000000000000/locations/westus."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The location name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The location type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<location::Type>,
    #[doc = "The display name of the location."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The display name of the location and its region."]
    #[serde(rename = "regionalDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub regional_display_name: Option<String>,
    #[doc = "Location metadata information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LocationMetadata>,
}
impl Location {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod location {
    use super::*;
    #[doc = "The location type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Region,
        EdgeZone,
    }
}
#[doc = "Location list operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocationListResult {
    #[doc = "An array of locations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Location>,
}
impl azure_core::Continuable for LocationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl LocationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Location metadata information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocationMetadata {
    #[doc = "The type of the region."]
    #[serde(rename = "regionType", default, skip_serializing_if = "Option::is_none")]
    pub region_type: Option<location_metadata::RegionType>,
    #[doc = "The category of the region."]
    #[serde(rename = "regionCategory", default, skip_serializing_if = "Option::is_none")]
    pub region_category: Option<location_metadata::RegionCategory>,
    #[doc = "The geography group of the location."]
    #[serde(rename = "geographyGroup", default, skip_serializing_if = "Option::is_none")]
    pub geography_group: Option<String>,
    #[doc = "The longitude of the location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
    #[doc = "The latitude of the location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    #[doc = "The physical location of the Azure location."]
    #[serde(rename = "physicalLocation", default, skip_serializing_if = "Option::is_none")]
    pub physical_location: Option<String>,
    #[doc = "The regions paired to this region."]
    #[serde(rename = "pairedRegion", default, skip_serializing_if = "Vec::is_empty")]
    pub paired_region: Vec<PairedRegion>,
    #[doc = "The home location of an edge zone."]
    #[serde(rename = "homeLocation", default, skip_serializing_if = "Option::is_none")]
    pub home_location: Option<String>,
}
impl LocationMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod location_metadata {
    use super::*;
    #[doc = "The type of the region."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RegionType")]
    pub enum RegionType {
        Physical,
        Logical,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RegionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RegionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RegionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Physical => serializer.serialize_unit_variant("RegionType", 0u32, "Physical"),
                Self::Logical => serializer.serialize_unit_variant("RegionType", 1u32, "Logical"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The category of the region."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RegionCategory")]
    pub enum RegionCategory {
        Recommended,
        Extended,
        Other,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RegionCategory {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RegionCategory {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RegionCategory {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Recommended => serializer.serialize_unit_variant("RegionCategory", 0u32, "Recommended"),
                Self::Extended => serializer.serialize_unit_variant("RegionCategory", 1u32, "Extended"),
                Self::Other => serializer.serialize_unit_variant("RegionCategory", 2u32, "Other"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information about a tenant managing the subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedByTenant {
    #[doc = "The tenant ID of the managing tenant. This is a GUID."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ManagedByTenant {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Microsoft.Resources operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
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
        #[doc = "Service provider: Microsoft.Resources"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Profile, endpoint, etc."]
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
#[doc = "Result of the request to list Microsoft.Resources operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Microsoft.Resources operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information regarding paired region."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PairedRegion {
    #[doc = "The name of the paired region."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The fully qualified ID of the location. For example, /subscriptions/00000000-0000-0000-0000-000000000000/locations/westus."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
impl PairedRegion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about shared availability zone."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Peers {
    #[doc = "The subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The availabilityZone."]
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
}
impl Peers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Name and Type of the Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceName {
    #[doc = "Name of the resource"]
    pub name: String,
    #[doc = "The type of the resource"]
    #[serde(rename = "type")]
    pub type_: String,
}
impl ResourceName {
    pub fn new(name: String, type_: String) -> Self {
        Self { name, type_ }
    }
}
#[doc = "Subscription information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Subscription {
    #[doc = "The fully qualified ID for the subscription. For example, /subscriptions/00000000-0000-0000-0000-000000000000."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The subscription display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The subscription tenant ID."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The subscription state. Possible values are Enabled, Warned, PastDue, Disabled, and Deleted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<subscription::State>,
    #[doc = "Subscription policies."]
    #[serde(rename = "subscriptionPolicies", default, skip_serializing_if = "Option::is_none")]
    pub subscription_policies: Option<SubscriptionPolicies>,
    #[doc = "The authorization source of the request. Valid values are one or more combinations of Legacy, RoleBased, Bypassed, Direct and Management. For example, 'Legacy, RoleBased'."]
    #[serde(rename = "authorizationSource", default, skip_serializing_if = "Option::is_none")]
    pub authorization_source: Option<String>,
    #[doc = "An array containing the tenants managing the subscription."]
    #[serde(rename = "managedByTenants", default, skip_serializing_if = "Vec::is_empty")]
    pub managed_by_tenants: Vec<ManagedByTenant>,
    #[doc = "The tags attached to the subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Subscription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription {
    use super::*;
    #[doc = "The subscription state. Possible values are Enabled, Warned, PastDue, Disabled, and Deleted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Warned,
        PastDue,
        Disabled,
        Deleted,
    }
}
#[doc = "Subscription list operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionListResult {
    #[doc = "An array of subscriptions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Subscription>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink")]
    pub next_link: String,
}
impl azure_core::Continuable for SubscriptionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        if self.next_link.is_empty() {
            None
        } else {
            Some(self.next_link.clone())
        }
    }
}
impl SubscriptionListResult {
    pub fn new(next_link: String) -> Self {
        Self {
            value: Vec::new(),
            next_link,
        }
    }
}
#[doc = "Subscription policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionPolicies {
    #[doc = "The subscription location placement ID. The ID indicates which regions are visible for a subscription. For example, a subscription with a location placement Id of Public_2014-09-01 has access to Azure public regions."]
    #[serde(rename = "locationPlacementId", default, skip_serializing_if = "Option::is_none")]
    pub location_placement_id: Option<String>,
    #[doc = "The subscription quota ID."]
    #[serde(rename = "quotaId", default, skip_serializing_if = "Option::is_none")]
    pub quota_id: Option<String>,
    #[doc = "The subscription spending limit."]
    #[serde(rename = "spendingLimit", default, skip_serializing_if = "Option::is_none")]
    pub spending_limit: Option<subscription_policies::SpendingLimit>,
}
impl SubscriptionPolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription_policies {
    use super::*;
    #[doc = "The subscription spending limit."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SpendingLimit {
        On,
        Off,
        CurrentPeriodOff,
    }
}
#[doc = "Tenant Id information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantIdDescription {
    #[doc = "The fully qualified ID of the tenant. For example, /tenants/00000000-0000-0000-0000-000000000000."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The tenant ID. For example, 00000000-0000-0000-0000-000000000000."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Category of the tenant."]
    #[serde(rename = "tenantCategory", default, skip_serializing_if = "Option::is_none")]
    pub tenant_category: Option<tenant_id_description::TenantCategory>,
    #[doc = "Country/region name of the address for the tenant."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Country/region abbreviation for the tenant."]
    #[serde(rename = "countryCode", default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[doc = "The display name of the tenant."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The list of domains for the tenant."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub domains: Vec<String>,
    #[doc = "The default domain for the tenant."]
    #[serde(rename = "defaultDomain", default, skip_serializing_if = "Option::is_none")]
    pub default_domain: Option<String>,
    #[doc = "The tenant type. Only available for 'Home' tenant category."]
    #[serde(rename = "tenantType", default, skip_serializing_if = "Option::is_none")]
    pub tenant_type: Option<String>,
    #[doc = "The tenant's branding logo URL. Only available for 'Home' tenant category."]
    #[serde(rename = "tenantBrandingLogoUrl", default, skip_serializing_if = "Option::is_none")]
    pub tenant_branding_logo_url: Option<String>,
}
impl TenantIdDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod tenant_id_description {
    use super::*;
    #[doc = "Category of the tenant."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TenantCategory {
        Home,
        ProjectedBy,
        ManagedBy,
    }
}
#[doc = "Tenant Ids information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantListResult {
    #[doc = "An array of tenants."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TenantIdDescription>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink")]
    pub next_link: String,
}
impl azure_core::Continuable for TenantListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        if self.next_link.is_empty() {
            None
        } else {
            Some(self.next_link.clone())
        }
    }
}
impl TenantListResult {
    pub fn new(next_link: String) -> Self {
        Self {
            value: Vec::new(),
            next_link,
        }
    }
}

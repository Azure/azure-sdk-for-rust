#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An addon resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Addon {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of an addon"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AddonPropertiesUnion>,
}
impl Addon {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an Arc addon"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddonArcProperties {
    #[serde(flatten)]
    pub addon_properties: AddonProperties,
    #[doc = "The VMware vCenter resource ID"]
    #[serde(rename = "vCenter", default, skip_serializing_if = "Option::is_none")]
    pub v_center: Option<String>,
}
impl AddonArcProperties {
    pub fn new(addon_properties: AddonProperties) -> Self {
        Self {
            addon_properties,
            v_center: None,
        }
    }
}
#[doc = "The properties of an HCX addon"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddonHcxProperties {
    #[serde(flatten)]
    pub addon_properties: AddonProperties,
    #[doc = "The HCX offer, example VMware MaaS Cloud Provider (Enterprise)"]
    pub offer: String,
}
impl AddonHcxProperties {
    pub fn new(addon_properties: AddonProperties, offer: String) -> Self {
        Self { addon_properties, offer }
    }
}
#[doc = "The response of a Addon list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddonList {
    #[doc = "The Addon items on this page"]
    pub value: Vec<Addon>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AddonList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AddonList {
    pub fn new(value: Vec<Addon>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of an addon"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddonProperties {
    #[doc = "Addon provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AddonProvisioningState>,
}
impl AddonProperties {
    pub fn new() -> Self {
        Self { provisioning_state: None }
    }
}
#[doc = "Addon type"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "addonType")]
pub enum AddonPropertiesUnion {
    Arc(AddonArcProperties),
    #[serde(rename = "HCX")]
    Hcx(AddonHcxProperties),
    #[serde(rename = "SRM")]
    Srm(AddonSrmProperties),
    #[serde(rename = "VR")]
    Vr(AddonVrProperties),
}
#[doc = "Addon provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AddonProvisioningState")]
pub enum AddonProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Cancelled,
    Building,
    Deleting,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AddonProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AddonProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AddonProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("AddonProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("AddonProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("AddonProvisioningState", 2u32, "Canceled"),
            Self::Cancelled => serializer.serialize_unit_variant("AddonProvisioningState", 3u32, "Cancelled"),
            Self::Building => serializer.serialize_unit_variant("AddonProvisioningState", 4u32, "Building"),
            Self::Deleting => serializer.serialize_unit_variant("AddonProvisioningState", 5u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("AddonProvisioningState", 6u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties of a Site Recovery Manager (SRM) addon"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddonSrmProperties {
    #[serde(flatten)]
    pub addon_properties: AddonProperties,
    #[doc = "The Site Recovery Manager (SRM) license"]
    #[serde(rename = "licenseKey", default, skip_serializing_if = "Option::is_none")]
    pub license_key: Option<String>,
}
impl AddonSrmProperties {
    pub fn new(addon_properties: AddonProperties) -> Self {
        Self {
            addon_properties,
            license_key: None,
        }
    }
}
#[doc = "Addon type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AddonType")]
pub enum AddonType {
    #[serde(rename = "SRM")]
    Srm,
    #[serde(rename = "VR")]
    Vr,
    #[serde(rename = "HCX")]
    Hcx,
    Arc,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AddonType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AddonType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AddonType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Srm => serializer.serialize_unit_variant("AddonType", 0u32, "SRM"),
            Self::Vr => serializer.serialize_unit_variant("AddonType", 1u32, "VR"),
            Self::Hcx => serializer.serialize_unit_variant("AddonType", 2u32, "HCX"),
            Self::Arc => serializer.serialize_unit_variant("AddonType", 3u32, "Arc"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties of a vSphere Replication (VR) addon"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddonVrProperties {
    #[serde(flatten)]
    pub addon_properties: AddonProperties,
    #[doc = "The vSphere Replication Server (VRS) count"]
    #[serde(rename = "vrsCount")]
    pub vrs_count: i32,
}
impl AddonVrProperties {
    pub fn new(addon_properties: AddonProperties, vrs_count: i32) -> Self {
        Self {
            addon_properties,
            vrs_count,
        }
    }
}
#[doc = "Administrative credentials for accessing vCenter and NSX-T"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdminCredentials {
    #[doc = "NSX-T Manager username"]
    #[serde(rename = "nsxtUsername", default, skip_serializing_if = "Option::is_none")]
    pub nsxt_username: Option<String>,
    #[doc = "NSX-T Manager password"]
    #[serde(rename = "nsxtPassword", default, skip_serializing_if = "Option::is_none")]
    pub nsxt_password: Option<String>,
    #[doc = "vCenter admin username"]
    #[serde(rename = "vcenterUsername", default, skip_serializing_if = "Option::is_none")]
    pub vcenter_username: Option<String>,
    #[doc = "vCenter admin password"]
    #[serde(rename = "vcenterPassword", default, skip_serializing_if = "Option::is_none")]
    pub vcenter_password: Option<String>,
}
impl AdminCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Affinity Strength"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AffinityStrength")]
pub enum AffinityStrength {
    Should,
    Must,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AffinityStrength {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AffinityStrength {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AffinityStrength {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Should => serializer.serialize_unit_variant("AffinityStrength", 0u32, "Should"),
            Self::Must => serializer.serialize_unit_variant("AffinityStrength", 1u32, "Must"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Affinity type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AffinityType")]
pub enum AffinityType {
    Affinity,
    AntiAffinity,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AffinityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AffinityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AffinityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Affinity => serializer.serialize_unit_variant("AffinityType", 0u32, "Affinity"),
            Self::AntiAffinity => serializer.serialize_unit_variant("AffinityType", 1u32, "AntiAffinity"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties describing private cloud availability zone distribution"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityProperties {
    #[doc = "Whether the private clouds is available in a single zone or two zones"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<AvailabilityStrategy>,
    #[doc = "The primary availability zone for the private cloud"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<i32>,
    #[doc = "The secondary availability zone for the private cloud"]
    #[serde(rename = "secondaryZone", default, skip_serializing_if = "Option::is_none")]
    pub secondary_zone: Option<i32>,
}
impl AvailabilityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Whether the private clouds is available in a single zone or two zones"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AvailabilityStrategy")]
pub enum AvailabilityStrategy {
    SingleZone,
    DualZone,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AvailabilityStrategy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AvailabilityStrategy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AvailabilityStrategy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SingleZone => serializer.serialize_unit_variant("AvailabilityStrategy", 0u32, "SingleZone"),
            Self::DualZone => serializer.serialize_unit_variant("AvailabilityStrategy", 1u32, "DualZone"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Azure Hybrid Benefit type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureHybridBenefitType")]
pub enum AzureHybridBenefitType {
    SqlHost,
    None,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureHybridBenefitType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureHybridBenefitType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureHybridBenefitType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SqlHost => serializer.serialize_unit_variant("AzureHybridBenefitType", 0u32, "SqlHost"),
            Self::None => serializer.serialize_unit_variant("AzureHybridBenefitType", 1u32, "None"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An ExpressRoute Circuit"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Circuit {
    #[doc = "CIDR of primary subnet"]
    #[serde(rename = "primarySubnet", default, skip_serializing_if = "Option::is_none")]
    pub primary_subnet: Option<String>,
    #[doc = "CIDR of secondary subnet"]
    #[serde(rename = "secondarySubnet", default, skip_serializing_if = "Option::is_none")]
    pub secondary_subnet: Option<String>,
    #[doc = "Identifier of the ExpressRoute Circuit (Microsoft Colo only)"]
    #[serde(rename = "expressRouteID", default, skip_serializing_if = "Option::is_none")]
    pub express_route_id: Option<String>,
    #[doc = "ExpressRoute Circuit private peering identifier"]
    #[serde(rename = "expressRoutePrivatePeeringID", default, skip_serializing_if = "Option::is_none")]
    pub express_route_private_peering_id: Option<String>,
}
impl Circuit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A cloud link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudLink {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a cloud link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudLinkProperties>,
}
impl CloudLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a CloudLink list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudLinkList {
    #[doc = "The CloudLink items on this page"]
    pub value: Vec<CloudLink>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CloudLinkList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CloudLinkList {
    pub fn new(value: Vec<CloudLink>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of a cloud link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudLinkProperties {
    #[doc = "cloud link provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<CloudLinkProvisioningState>,
    #[doc = "Cloud Link status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CloudLinkStatus>,
    #[doc = "Identifier of the other private cloud participating in the link."]
    #[serde(rename = "linkedCloud", default, skip_serializing_if = "Option::is_none")]
    pub linked_cloud: Option<String>,
}
impl CloudLinkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "cloud link provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CloudLinkProvisioningState")]
pub enum CloudLinkProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CloudLinkProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CloudLinkProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CloudLinkProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("CloudLinkProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("CloudLinkProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("CloudLinkProvisioningState", 2u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Cloud Link status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CloudLinkStatus")]
pub enum CloudLinkStatus {
    Active,
    Building,
    Deleting,
    Failed,
    Disconnected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CloudLinkStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CloudLinkStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CloudLinkStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("CloudLinkStatus", 0u32, "Active"),
            Self::Building => serializer.serialize_unit_variant("CloudLinkStatus", 1u32, "Building"),
            Self::Deleting => serializer.serialize_unit_variant("CloudLinkStatus", 2u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("CloudLinkStatus", 3u32, "Failed"),
            Self::Disconnected => serializer.serialize_unit_variant("CloudLinkStatus", 4u32, "Disconnected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A cluster resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a cluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
    #[doc = "The resource model definition representing SKU"]
    pub sku: Sku,
}
impl Cluster {
    pub fn new(sku: Sku) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties: None,
            sku,
        }
    }
}
#[doc = "The response of a Cluster list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterList {
    #[doc = "The Cluster items on this page"]
    pub value: Vec<Cluster>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ClusterList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ClusterList {
    pub fn new(value: Vec<Cluster>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of a cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterProperties {
    #[doc = "The cluster size"]
    #[serde(rename = "clusterSize", default, skip_serializing_if = "Option::is_none")]
    pub cluster_size: Option<i32>,
    #[doc = "Cluster provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ClusterProvisioningState>,
    #[doc = "The identity"]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<i32>,
    #[doc = "The hosts"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hosts: Vec<String>,
    #[doc = "Name of the vsan datastore associated with the cluster"]
    #[serde(rename = "vsanDatastoreName", default, skip_serializing_if = "Option::is_none")]
    pub vsan_datastore_name: Option<String>,
}
impl ClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cluster provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ClusterProvisioningState")]
pub enum ClusterProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Cancelled,
    Deleting,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ClusterProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ClusterProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ClusterProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("ClusterProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ClusterProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ClusterProvisioningState", 2u32, "Canceled"),
            Self::Cancelled => serializer.serialize_unit_variant("ClusterProvisioningState", 3u32, "Cancelled"),
            Self::Deleting => serializer.serialize_unit_variant("ClusterProvisioningState", 4u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("ClusterProvisioningState", 5u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An update of a cluster resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpdate {
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The properties of a cluster that may be updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterUpdateProperties>,
}
impl ClusterUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a cluster that may be updated"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpdateProperties {
    #[doc = "The cluster size"]
    #[serde(rename = "clusterSize", default, skip_serializing_if = "Option::is_none")]
    pub cluster_size: Option<i32>,
    #[doc = "The hosts"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hosts: Vec<String>,
}
impl ClusterUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Zone and associated hosts info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterZone {
    #[doc = "List of hosts belonging to the availability zone in a cluster"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hosts: Vec<String>,
    #[doc = "Availability zone identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}
impl ClusterZone {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of all zones and associated hosts for a cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterZoneList {
    #[doc = "Zone and associated hosts info"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub zones: Vec<ClusterZone>,
}
impl ClusterZoneList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A datastore resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Datastore {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a datastore"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatastoreProperties>,
}
impl Datastore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a Datastore list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatastoreList {
    #[doc = "The Datastore items on this page"]
    pub value: Vec<Datastore>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatastoreList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DatastoreList {
    pub fn new(value: Vec<Datastore>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of a datastore"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatastoreProperties {
    #[doc = "datastore provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DatastoreProvisioningState>,
    #[doc = "An Azure NetApp Files volume from Microsoft.NetApp provider"]
    #[serde(rename = "netAppVolume", default, skip_serializing_if = "Option::is_none")]
    pub net_app_volume: Option<NetAppVolume>,
    #[doc = "An iSCSI volume from Microsoft.StoragePool provider"]
    #[serde(rename = "diskPoolVolume", default, skip_serializing_if = "Option::is_none")]
    pub disk_pool_volume: Option<DiskPoolVolume>,
    #[doc = "An Elastic SAN volume from Microsoft.ElasticSan provider"]
    #[serde(rename = "elasticSanVolume", default, skip_serializing_if = "Option::is_none")]
    pub elastic_san_volume: Option<ElasticSanVolume>,
    #[doc = "datastore status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DatastoreStatus>,
}
impl DatastoreProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "datastore provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DatastoreProvisioningState")]
pub enum DatastoreProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Cancelled,
    Pending,
    Creating,
    Updating,
    Deleting,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DatastoreProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DatastoreProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DatastoreProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("DatastoreProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("DatastoreProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("DatastoreProvisioningState", 2u32, "Canceled"),
            Self::Cancelled => serializer.serialize_unit_variant("DatastoreProvisioningState", 3u32, "Cancelled"),
            Self::Pending => serializer.serialize_unit_variant("DatastoreProvisioningState", 4u32, "Pending"),
            Self::Creating => serializer.serialize_unit_variant("DatastoreProvisioningState", 5u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("DatastoreProvisioningState", 6u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("DatastoreProvisioningState", 7u32, "Deleting"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "datastore status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DatastoreStatus")]
pub enum DatastoreStatus {
    Unknown,
    Accessible,
    Inaccessible,
    Attached,
    Detached,
    LostCommunication,
    DeadOrError,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DatastoreStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DatastoreStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DatastoreStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("DatastoreStatus", 0u32, "Unknown"),
            Self::Accessible => serializer.serialize_unit_variant("DatastoreStatus", 1u32, "Accessible"),
            Self::Inaccessible => serializer.serialize_unit_variant("DatastoreStatus", 2u32, "Inaccessible"),
            Self::Attached => serializer.serialize_unit_variant("DatastoreStatus", 3u32, "Attached"),
            Self::Detached => serializer.serialize_unit_variant("DatastoreStatus", 4u32, "Detached"),
            Self::LostCommunication => serializer.serialize_unit_variant("DatastoreStatus", 5u32, "LostCommunication"),
            Self::DeadOrError => serializer.serialize_unit_variant("DatastoreStatus", 6u32, "DeadOrError"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Type of DHCP: SERVER or RELAY."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DhcpTypeEnum")]
pub enum DhcpTypeEnum {
    #[serde(rename = "SERVER")]
    Server,
    #[serde(rename = "RELAY")]
    Relay,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DhcpTypeEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DhcpTypeEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DhcpTypeEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Server => serializer.serialize_unit_variant("DhcpTypeEnum", 0u32, "SERVER"),
            Self::Relay => serializer.serialize_unit_variant("DhcpTypeEnum", 1u32, "RELAY"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An iSCSI volume from Microsoft.StoragePool provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolVolume {
    #[doc = "Azure resource ID of the iSCSI target"]
    #[serde(rename = "targetId")]
    pub target_id: String,
    #[doc = "Name of the LUN to be used for datastore"]
    #[serde(rename = "lunName")]
    pub lun_name: String,
    #[doc = "Mode that describes whether the LUN has to be mounted as a datastore or\nattached as a LUN"]
    #[serde(rename = "mountOption", default, skip_serializing_if = "Option::is_none")]
    pub mount_option: Option<disk_pool_volume::MountOption>,
    #[doc = "Device path"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl DiskPoolVolume {
    pub fn new(target_id: String, lun_name: String) -> Self {
        Self {
            target_id,
            lun_name,
            mount_option: None,
            path: None,
        }
    }
}
pub mod disk_pool_volume {
    use super::*;
    #[doc = "Mode that describes whether the LUN has to be mounted as a datastore or\nattached as a LUN"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MountOption")]
    pub enum MountOption {
        #[serde(rename = "MOUNT")]
        Mount,
        #[serde(rename = "ATTACH")]
        Attach,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MountOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MountOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MountOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Mount => serializer.serialize_unit_variant("MountOption", 0u32, "MOUNT"),
                Self::Attach => serializer.serialize_unit_variant("MountOption", 1u32, "ATTACH"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for MountOption {
        fn default() -> Self {
            Self::Mount
        }
    }
}
#[doc = "DNS service log level"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DnsServiceLogLevelEnum")]
pub enum DnsServiceLogLevelEnum {
    #[serde(rename = "DEBUG")]
    Debug,
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "WARNING")]
    Warning,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "FATAL")]
    Fatal,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DnsServiceLogLevelEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DnsServiceLogLevelEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DnsServiceLogLevelEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Debug => serializer.serialize_unit_variant("DnsServiceLogLevelEnum", 0u32, "DEBUG"),
            Self::Info => serializer.serialize_unit_variant("DnsServiceLogLevelEnum", 1u32, "INFO"),
            Self::Warning => serializer.serialize_unit_variant("DnsServiceLogLevelEnum", 2u32, "WARNING"),
            Self::Error => serializer.serialize_unit_variant("DnsServiceLogLevelEnum", 3u32, "ERROR"),
            Self::Fatal => serializer.serialize_unit_variant("DnsServiceLogLevelEnum", 4u32, "FATAL"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "DNS service status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DnsServiceStatusEnum")]
pub enum DnsServiceStatusEnum {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DnsServiceStatusEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DnsServiceStatusEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DnsServiceStatusEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Success => serializer.serialize_unit_variant("DnsServiceStatusEnum", 0u32, "SUCCESS"),
            Self::Failure => serializer.serialize_unit_variant("DnsServiceStatusEnum", 1u32, "FAILURE"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type of DNS zone."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DnsZoneType")]
pub enum DnsZoneType {
    Public,
    Private,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DnsZoneType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DnsZoneType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DnsZoneType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Public => serializer.serialize_unit_variant("DnsZoneType", 0u32, "Public"),
            Self::Private => serializer.serialize_unit_variant("DnsZoneType", 1u32, "Private"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An Elastic SAN volume from Microsoft.ElasticSan provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticSanVolume {
    #[doc = "Azure resource ID of the Elastic SAN Volume"]
    #[serde(rename = "targetId")]
    pub target_id: String,
}
impl ElasticSanVolume {
    pub fn new(target_id: String) -> Self {
        Self { target_id }
    }
}
#[doc = "The properties of customer managed encryption key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Encryption {
    #[doc = "Whether encryption is enabled or disabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EncryptionState>,
    #[doc = "An Encryption Key"]
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<EncryptionKeyVaultProperties>,
}
impl Encryption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Whether the the encryption key is connected or access denied"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EncryptionKeyStatus")]
pub enum EncryptionKeyStatus {
    Connected,
    AccessDenied,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EncryptionKeyStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EncryptionKeyStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EncryptionKeyStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Connected => serializer.serialize_unit_variant("EncryptionKeyStatus", 0u32, "Connected"),
            Self::AccessDenied => serializer.serialize_unit_variant("EncryptionKeyStatus", 1u32, "AccessDenied"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An Encryption Key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionKeyVaultProperties {
    #[doc = "The name of the key."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "The version of the key."]
    #[serde(rename = "keyVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
    #[doc = "The auto-detected version of the key if versionType is auto-detected."]
    #[serde(rename = "autoDetectedKeyVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_detected_key_version: Option<String>,
    #[doc = "The URL of the vault."]
    #[serde(rename = "keyVaultUrl", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_url: Option<String>,
    #[doc = "Whether the the encryption key is connected or access denied"]
    #[serde(rename = "keyState", default, skip_serializing_if = "Option::is_none")]
    pub key_state: Option<EncryptionKeyStatus>,
    #[doc = "Whether the encryption version is fixed or auto-detected"]
    #[serde(rename = "versionType", default, skip_serializing_if = "Option::is_none")]
    pub version_type: Option<EncryptionVersionType>,
}
impl EncryptionKeyVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Whether encryption is enabled or disabled"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EncryptionState")]
pub enum EncryptionState {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EncryptionState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EncryptionState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EncryptionState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("EncryptionState", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("EncryptionState", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Whether the encryption version is fixed or auto-detected"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EncryptionVersionType")]
pub enum EncryptionVersionType {
    Fixed,
    AutoDetected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EncryptionVersionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EncryptionVersionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EncryptionVersionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Fixed => serializer.serialize_unit_variant("EncryptionVersionType", 0u32, "Fixed"),
            Self::AutoDetected => serializer.serialize_unit_variant("EncryptionVersionType", 1u32, "AutoDetected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Endpoint addresses"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Endpoints {
    #[doc = "Endpoint FQDN for the NSX-T Data Center manager"]
    #[serde(rename = "nsxtManager", default, skip_serializing_if = "Option::is_none")]
    pub nsxt_manager: Option<String>,
    #[doc = "Endpoint FQDN for Virtual Center Server Appliance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vcsa: Option<String>,
    #[doc = "Endpoint FQDN for the HCX Cloud Manager"]
    #[serde(rename = "hcxCloudManager", default, skip_serializing_if = "Option::is_none")]
    pub hcx_cloud_manager: Option<String>,
    #[doc = "Endpoint IP for the NSX-T Data Center manager"]
    #[serde(rename = "nsxtManagerIp", default, skip_serializing_if = "Option::is_none")]
    pub nsxt_manager_ip: Option<String>,
    #[doc = "Endpoint IP for Virtual Center Server Appliance"]
    #[serde(rename = "vcenterIp", default, skip_serializing_if = "Option::is_none")]
    pub vcenter_ip: Option<String>,
    #[doc = "Endpoint IP for the HCX Cloud Manager"]
    #[serde(rename = "hcxCloudManagerIp", default, skip_serializing_if = "Option::is_none")]
    pub hcx_cloud_manager_ip: Option<String>,
}
impl Endpoints {
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
#[doc = "ExpressRoute Circuit Authorization"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpressRouteAuthorization {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of an ExpressRoute Circuit Authorization resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExpressRouteAuthorizationProperties>,
}
impl ExpressRouteAuthorization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a ExpressRouteAuthorization list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpressRouteAuthorizationList {
    #[doc = "The ExpressRouteAuthorization items on this page"]
    pub value: Vec<ExpressRouteAuthorization>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExpressRouteAuthorizationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ExpressRouteAuthorizationList {
    pub fn new(value: Vec<ExpressRouteAuthorization>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of an ExpressRoute Circuit Authorization resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpressRouteAuthorizationProperties {
    #[doc = "Express Route Circuit Authorization provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ExpressRouteAuthorizationProvisioningState>,
    #[doc = "The ID of the ExpressRoute Circuit Authorization"]
    #[serde(rename = "expressRouteAuthorizationId", default, skip_serializing_if = "Option::is_none")]
    pub express_route_authorization_id: Option<String>,
    #[doc = "The key of the ExpressRoute Circuit Authorization"]
    #[serde(rename = "expressRouteAuthorizationKey", default, skip_serializing_if = "Option::is_none")]
    pub express_route_authorization_key: Option<String>,
    #[doc = "The ID of the ExpressRoute Circuit"]
    #[serde(rename = "expressRouteId", default, skip_serializing_if = "Option::is_none")]
    pub express_route_id: Option<String>,
}
impl ExpressRouteAuthorizationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Express Route Circuit Authorization provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ExpressRouteAuthorizationProvisioningState")]
pub enum ExpressRouteAuthorizationProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ExpressRouteAuthorizationProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ExpressRouteAuthorizationProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ExpressRouteAuthorizationProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("ExpressRouteAuthorizationProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ExpressRouteAuthorizationProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ExpressRouteAuthorizationProvisioningState", 2u32, "Canceled"),
            Self::Updating => serializer.serialize_unit_variant("ExpressRouteAuthorizationProvisioningState", 3u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A global reach connection resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalReachConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a global reach connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GlobalReachConnectionProperties>,
}
impl GlobalReachConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a GlobalReachConnection list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalReachConnectionList {
    #[doc = "The GlobalReachConnection items on this page"]
    pub value: Vec<GlobalReachConnection>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GlobalReachConnectionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GlobalReachConnectionList {
    pub fn new(value: Vec<GlobalReachConnection>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of a global reach connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalReachConnectionProperties {
    #[doc = "Global Reach Connection provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<GlobalReachConnectionProvisioningState>,
    #[doc = "The network used for global reach carved out from the original network block\nprovided for the private cloud"]
    #[serde(rename = "addressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub address_prefix: Option<String>,
    #[doc = "Authorization key from the peer express route used for the global reach\nconnection"]
    #[serde(rename = "authorizationKey", default, skip_serializing_if = "Option::is_none")]
    pub authorization_key: Option<String>,
    #[doc = "Global Reach Connection status"]
    #[serde(rename = "circuitConnectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub circuit_connection_status: Option<GlobalReachConnectionStatus>,
    #[doc = "Identifier of the ExpressRoute Circuit to peer with in the global reach\nconnection"]
    #[serde(rename = "peerExpressRouteCircuit", default, skip_serializing_if = "Option::is_none")]
    pub peer_express_route_circuit: Option<String>,
    #[doc = "The ID of the Private Cloud's ExpressRoute Circuit that is participating in the\nglobal reach connection"]
    #[serde(rename = "expressRouteId", default, skip_serializing_if = "Option::is_none")]
    pub express_route_id: Option<String>,
}
impl GlobalReachConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Global Reach Connection provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "GlobalReachConnectionProvisioningState")]
pub enum GlobalReachConnectionProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for GlobalReachConnectionProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for GlobalReachConnectionProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for GlobalReachConnectionProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("GlobalReachConnectionProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("GlobalReachConnectionProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("GlobalReachConnectionProvisioningState", 2u32, "Canceled"),
            Self::Updating => serializer.serialize_unit_variant("GlobalReachConnectionProvisioningState", 3u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Global Reach Connection status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "GlobalReachConnectionStatus")]
pub enum GlobalReachConnectionStatus {
    Connected,
    Connecting,
    Disconnected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for GlobalReachConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for GlobalReachConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for GlobalReachConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Connected => serializer.serialize_unit_variant("GlobalReachConnectionStatus", 0u32, "Connected"),
            Self::Connecting => serializer.serialize_unit_variant("GlobalReachConnectionStatus", 1u32, "Connecting"),
            Self::Disconnected => serializer.serialize_unit_variant("GlobalReachConnectionStatus", 2u32, "Disconnected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An HCX Enterprise Site resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HcxEnterpriseSite {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of an HCX Enterprise Site"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HcxEnterpriseSiteProperties>,
}
impl HcxEnterpriseSite {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a HcxEnterpriseSite list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HcxEnterpriseSiteList {
    #[doc = "The HcxEnterpriseSite items on this page"]
    pub value: Vec<HcxEnterpriseSite>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HcxEnterpriseSiteList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HcxEnterpriseSiteList {
    pub fn new(value: Vec<HcxEnterpriseSite>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of an HCX Enterprise Site"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HcxEnterpriseSiteProperties {
    #[doc = "HCX Enterprise Site provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<HcxEnterpriseSiteProvisioningState>,
    #[doc = "The activation key"]
    #[serde(rename = "activationKey", default, skip_serializing_if = "Option::is_none")]
    pub activation_key: Option<String>,
    #[doc = "HCX Enterprise Site status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<HcxEnterpriseSiteStatus>,
}
impl HcxEnterpriseSiteProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "HCX Enterprise Site provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HcxEnterpriseSiteProvisioningState")]
pub enum HcxEnterpriseSiteProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HcxEnterpriseSiteProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HcxEnterpriseSiteProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HcxEnterpriseSiteProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("HcxEnterpriseSiteProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("HcxEnterpriseSiteProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("HcxEnterpriseSiteProvisioningState", 2u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "HCX Enterprise Site status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HcxEnterpriseSiteStatus")]
pub enum HcxEnterpriseSiteStatus {
    Available,
    Consumed,
    Deactivated,
    Deleted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HcxEnterpriseSiteStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HcxEnterpriseSiteStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HcxEnterpriseSiteStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Available => serializer.serialize_unit_variant("HcxEnterpriseSiteStatus", 0u32, "Available"),
            Self::Consumed => serializer.serialize_unit_variant("HcxEnterpriseSiteStatus", 1u32, "Consumed"),
            Self::Deactivated => serializer.serialize_unit_variant("HcxEnterpriseSiteStatus", 2u32, "Deactivated"),
            Self::Deleted => serializer.serialize_unit_variant("HcxEnterpriseSiteStatus", 3u32, "Deleted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "vCenter Single Sign On Identity Source"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentitySource {
    #[doc = "The name of the identity source"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The domain's NetBIOS name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[doc = "The domain's dns name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "The base distinguished name for users"]
    #[serde(rename = "baseUserDN", default, skip_serializing_if = "Option::is_none")]
    pub base_user_dn: Option<String>,
    #[doc = "The base distinguished name for groups"]
    #[serde(rename = "baseGroupDN", default, skip_serializing_if = "Option::is_none")]
    pub base_group_dn: Option<String>,
    #[doc = "Primary server URL"]
    #[serde(rename = "primaryServer", default, skip_serializing_if = "Option::is_none")]
    pub primary_server: Option<String>,
    #[doc = "Secondary server URL"]
    #[serde(rename = "secondaryServer", default, skip_serializing_if = "Option::is_none")]
    pub secondary_server: Option<String>,
    #[doc = "Whether SSL is enabled or disabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl: Option<SslEnum>,
    #[doc = "The ID of an Active Directory user with a minimum of read-only access to Base\nDN for users and group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The password of the Active Directory user with a minimum of read-only access to\nBase DN for users and groups."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl IdentitySource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Whether internet is enabled or disabled"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InternetEnum")]
pub enum InternetEnum {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InternetEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InternetEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InternetEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("InternetEnum", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("InternetEnum", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An iSCSI path resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IscsiPath {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of an iSCSI path resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IscsiPathProperties>,
}
impl IscsiPath {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a IscsiPath list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiPathListResult {
    #[doc = "The IscsiPath items on this page"]
    pub value: Vec<IscsiPath>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IscsiPathListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl IscsiPathListResult {
    pub fn new(value: Vec<IscsiPath>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of an iSCSI path resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiPathProperties {
    #[doc = "private cloud provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<IscsiPathProvisioningState>,
    #[doc = "CIDR Block for iSCSI path."]
    #[serde(rename = "networkBlock")]
    pub network_block: String,
}
impl IscsiPathProperties {
    pub fn new(network_block: String) -> Self {
        Self {
            provisioning_state: None,
            network_block,
        }
    }
}
#[doc = "private cloud provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IscsiPathProvisioningState")]
pub enum IscsiPathProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Pending,
    Building,
    Deleting,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IscsiPathProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IscsiPathProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IscsiPathProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("IscsiPathProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("IscsiPathProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("IscsiPathProvisioningState", 2u32, "Canceled"),
            Self::Pending => serializer.serialize_unit_variant("IscsiPathProvisioningState", 3u32, "Pending"),
            Self::Building => serializer.serialize_unit_variant("IscsiPathProvisioningState", 4u32, "Building"),
            Self::Deleting => serializer.serialize_unit_variant("IscsiPathProvisioningState", 5u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("IscsiPathProvisioningState", 6u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties of a management cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementCluster {
    #[doc = "The cluster size"]
    #[serde(rename = "clusterSize", default, skip_serializing_if = "Option::is_none")]
    pub cluster_size: Option<i32>,
    #[doc = "Cluster provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ClusterProvisioningState>,
    #[doc = "The identity"]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<i32>,
    #[doc = "The hosts"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hosts: Vec<String>,
    #[doc = "Name of the vsan datastore associated with the cluster"]
    #[serde(rename = "vsanDatastoreName", default, skip_serializing_if = "Option::is_none")]
    pub vsan_datastore_name: Option<String>,
}
impl ManagementCluster {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure NetApp Files volume from Microsoft.NetApp provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetAppVolume {
    #[doc = "Azure resource ID of the NetApp volume"]
    pub id: String,
}
impl NetAppVolume {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "NSX public IP quota raised"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NsxPublicIpQuotaRaisedEnum")]
pub enum NsxPublicIpQuotaRaisedEnum {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NsxPublicIpQuotaRaisedEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NsxPublicIpQuotaRaisedEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NsxPublicIpQuotaRaisedEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("NsxPublicIpQuotaRaisedEnum", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("NsxPublicIpQuotaRaisedEnum", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Optional Param"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OptionalParamEnum")]
pub enum OptionalParamEnum {
    Optional,
    Required,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OptionalParamEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OptionalParamEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OptionalParamEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Optional => serializer.serialize_unit_variant("OptionalParamEnum", 0u32, "Optional"),
            Self::Required => serializer.serialize_unit_variant("OptionalParamEnum", 1u32, "Required"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "a powershell credential object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PsCredentialExecutionParameter {
    #[serde(flatten)]
    pub script_execution_parameter: ScriptExecutionParameter,
    #[doc = "username for login"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "password for login"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl PsCredentialExecutionParameter {
    pub fn new(script_execution_parameter: ScriptExecutionParameter) -> Self {
        Self {
            script_execution_parameter,
            username: None,
            password: None,
        }
    }
}
#[doc = "The response of a PlacementPolicy list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlacementPoliciesList {
    #[doc = "The PlacementPolicy items on this page"]
    pub value: Vec<PlacementPolicy>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PlacementPoliciesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PlacementPoliciesList {
    pub fn new(value: Vec<PlacementPolicy>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A vSphere Distributed Resource Scheduler (DRS) placement policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlacementPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Abstract placement policy properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PlacementPolicyPropertiesUnion>,
}
impl PlacementPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Abstract placement policy properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlacementPolicyProperties {
    #[doc = "Placement Policy state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<PlacementPolicyState>,
    #[doc = "Display name of the placement policy"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Placement Policy provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PlacementPolicyProvisioningState>,
}
impl PlacementPolicyProperties {
    pub fn new() -> Self {
        Self {
            state: None,
            display_name: None,
            provisioning_state: None,
        }
    }
}
#[doc = "Placement Policy type"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PlacementPolicyPropertiesUnion {
    VmHost(VmHostPlacementPolicyProperties),
    VmVm(VmVmPlacementPolicyProperties),
}
#[doc = "Placement Policy provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PlacementPolicyProvisioningState")]
pub enum PlacementPolicyProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Building,
    Deleting,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PlacementPolicyProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PlacementPolicyProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PlacementPolicyProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("PlacementPolicyProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("PlacementPolicyProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("PlacementPolicyProvisioningState", 2u32, "Canceled"),
            Self::Building => serializer.serialize_unit_variant("PlacementPolicyProvisioningState", 3u32, "Building"),
            Self::Deleting => serializer.serialize_unit_variant("PlacementPolicyProvisioningState", 4u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("PlacementPolicyProvisioningState", 5u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Placement Policy state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PlacementPolicyState")]
pub enum PlacementPolicyState {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PlacementPolicyState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PlacementPolicyState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PlacementPolicyState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("PlacementPolicyState", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("PlacementPolicyState", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Placement Policy type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PlacementPolicyType")]
pub enum PlacementPolicyType {
    VmVm,
    VmHost,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PlacementPolicyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PlacementPolicyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PlacementPolicyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::VmVm => serializer.serialize_unit_variant("PlacementPolicyType", 0u32, "VmVm"),
            Self::VmHost => serializer.serialize_unit_variant("PlacementPolicyType", 1u32, "VmHost"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An update of a DRS placement policy resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlacementPolicyUpdate {
    #[doc = "The properties of a placement policy resource that may be updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PlacementPolicyUpdateProperties>,
}
impl PlacementPolicyUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a placement policy resource that may be updated"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlacementPolicyUpdateProperties {
    #[doc = "Placement Policy state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<PlacementPolicyState>,
    #[doc = "Virtual machine members list"]
    #[serde(
        rename = "vmMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vm_members: Vec<String>,
    #[doc = "Host members list"]
    #[serde(
        rename = "hostMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub host_members: Vec<String>,
    #[doc = "Affinity Strength"]
    #[serde(rename = "affinityStrength", default, skip_serializing_if = "Option::is_none")]
    pub affinity_strength: Option<AffinityStrength>,
    #[doc = "Azure Hybrid Benefit type"]
    #[serde(rename = "azureHybridBenefitType", default, skip_serializing_if = "Option::is_none")]
    pub azure_hybrid_benefit_type: Option<AzureHybridBenefitType>,
}
impl PlacementPolicyUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Port Mirroring Direction"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PortMirroringDirectionEnum")]
pub enum PortMirroringDirectionEnum {
    #[serde(rename = "INGRESS")]
    Ingress,
    #[serde(rename = "EGRESS")]
    Egress,
    #[serde(rename = "BIDIRECTIONAL")]
    Bidirectional,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PortMirroringDirectionEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PortMirroringDirectionEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PortMirroringDirectionEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ingress => serializer.serialize_unit_variant("PortMirroringDirectionEnum", 0u32, "INGRESS"),
            Self::Egress => serializer.serialize_unit_variant("PortMirroringDirectionEnum", 1u32, "EGRESS"),
            Self::Bidirectional => serializer.serialize_unit_variant("PortMirroringDirectionEnum", 2u32, "BIDIRECTIONAL"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Port Mirroring status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PortMirroringStatusEnum")]
pub enum PortMirroringStatusEnum {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PortMirroringStatusEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PortMirroringStatusEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PortMirroringStatusEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Success => serializer.serialize_unit_variant("PortMirroringStatusEnum", 0u32, "SUCCESS"),
            Self::Failure => serializer.serialize_unit_variant("PortMirroringStatusEnum", 1u32, "FAILURE"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A private cloud resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloud {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of a private cloud resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateCloudProperties>,
    #[doc = "The resource model definition representing SKU"]
    pub sku: Sku,
    #[doc = "Managed service identity (either system assigned, or none)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<SystemAssignedServiceIdentity>,
}
impl PrivateCloud {
    pub fn new(tracked_resource: TrackedResource, sku: Sku) -> Self {
        Self {
            tracked_resource,
            properties: None,
            sku,
            identity: None,
        }
    }
}
#[doc = "The response of a PrivateCloud list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloudList {
    #[doc = "The PrivateCloud items on this page"]
    pub value: Vec<PrivateCloud>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateCloudList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PrivateCloudList {
    pub fn new(value: Vec<PrivateCloud>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of a private cloud resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloudProperties {
    #[doc = "The properties of a management cluster"]
    #[serde(rename = "managementCluster")]
    pub management_cluster: ManagementCluster,
    #[doc = "Connectivity to internet is enabled or disabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internet: Option<private_cloud_properties::Internet>,
    #[doc = "vCenter Single Sign On Identity Sources"]
    #[serde(
        rename = "identitySources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub identity_sources: Vec<IdentitySource>,
    #[doc = "The properties describing private cloud availability zone distribution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability: Option<AvailabilityProperties>,
    #[doc = "The properties of customer managed encryption key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "Array of additional networks noncontiguous with networkBlock. Networks must be\nunique and non-overlapping across VNet in your subscription, on-premise, and\nthis privateCloud networkBlock attribute. Make sure the CIDR format conforms to\n(A.B.C.D/X)."]
    #[serde(
        rename = "extendedNetworkBlocks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extended_network_blocks: Vec<String>,
    #[doc = "private cloud provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateCloudProvisioningState>,
    #[doc = "An ExpressRoute Circuit"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circuit: Option<Circuit>,
    #[doc = "Endpoint addresses"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Endpoints>,
    #[doc = "The block of addresses should be unique across VNet in your subscription as\nwell as on-premise. Make sure the CIDR format is conformed to (A.B.C.D/X) where\nA,B,C,D are between 0 and 255, and X is between 0 and 22"]
    #[serde(rename = "networkBlock")]
    pub network_block: String,
    #[doc = "Network used to access vCenter Server and NSX-T Manager"]
    #[serde(rename = "managementNetwork", default, skip_serializing_if = "Option::is_none")]
    pub management_network: Option<String>,
    #[doc = "Used for virtual machine cold migration, cloning, and snapshot migration"]
    #[serde(rename = "provisioningNetwork", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_network: Option<String>,
    #[doc = "Used for live migration of virtual machines"]
    #[serde(rename = "vmotionNetwork", default, skip_serializing_if = "Option::is_none")]
    pub vmotion_network: Option<String>,
    #[doc = "Optionally, set the vCenter admin password when the private cloud is created"]
    #[serde(rename = "vcenterPassword", default, skip_serializing_if = "Option::is_none")]
    pub vcenter_password: Option<String>,
    #[doc = "Optionally, set the NSX-T Manager password when the private cloud is created"]
    #[serde(rename = "nsxtPassword", default, skip_serializing_if = "Option::is_none")]
    pub nsxt_password: Option<String>,
    #[doc = "Thumbprint of the vCenter Server SSL certificate"]
    #[serde(rename = "vcenterCertificateThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub vcenter_certificate_thumbprint: Option<String>,
    #[doc = "Thumbprint of the NSX-T Manager SSL certificate"]
    #[serde(rename = "nsxtCertificateThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub nsxt_certificate_thumbprint: Option<String>,
    #[doc = "Array of cloud link IDs from other clouds that connect to this one"]
    #[serde(
        rename = "externalCloudLinks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub external_cloud_links: Vec<String>,
    #[doc = "An ExpressRoute Circuit"]
    #[serde(rename = "secondaryCircuit", default, skip_serializing_if = "Option::is_none")]
    pub secondary_circuit: Option<Circuit>,
    #[doc = "NSX public IP quota raised"]
    #[serde(rename = "nsxPublicIpQuotaRaised", default, skip_serializing_if = "Option::is_none")]
    pub nsx_public_ip_quota_raised: Option<NsxPublicIpQuotaRaisedEnum>,
    #[doc = "Azure resource ID of the virtual network"]
    #[serde(rename = "virtualNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_id: Option<String>,
    #[doc = "The type of DNS zone."]
    #[serde(rename = "dnsZoneType", default, skip_serializing_if = "Option::is_none")]
    pub dns_zone_type: Option<DnsZoneType>,
}
impl PrivateCloudProperties {
    pub fn new(management_cluster: ManagementCluster, network_block: String) -> Self {
        Self {
            management_cluster,
            internet: None,
            identity_sources: Vec::new(),
            availability: None,
            encryption: None,
            extended_network_blocks: Vec::new(),
            provisioning_state: None,
            circuit: None,
            endpoints: None,
            network_block,
            management_network: None,
            provisioning_network: None,
            vmotion_network: None,
            vcenter_password: None,
            nsxt_password: None,
            vcenter_certificate_thumbprint: None,
            nsxt_certificate_thumbprint: None,
            external_cloud_links: Vec::new(),
            secondary_circuit: None,
            nsx_public_ip_quota_raised: None,
            virtual_network_id: None,
            dns_zone_type: None,
        }
    }
}
pub mod private_cloud_properties {
    use super::*;
    #[doc = "Connectivity to internet is enabled or disabled"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Internet")]
    pub enum Internet {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Internet", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Internet", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Internet {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[doc = "private cloud provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateCloudProvisioningState")]
pub enum PrivateCloudProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Cancelled,
    Pending,
    Building,
    Deleting,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateCloudProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateCloudProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateCloudProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("PrivateCloudProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("PrivateCloudProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("PrivateCloudProvisioningState", 2u32, "Canceled"),
            Self::Cancelled => serializer.serialize_unit_variant("PrivateCloudProvisioningState", 3u32, "Cancelled"),
            Self::Pending => serializer.serialize_unit_variant("PrivateCloudProvisioningState", 4u32, "Pending"),
            Self::Building => serializer.serialize_unit_variant("PrivateCloudProvisioningState", 5u32, "Building"),
            Self::Deleting => serializer.serialize_unit_variant("PrivateCloudProvisioningState", 6u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("PrivateCloudProvisioningState", 7u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An update to a private cloud resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateCloudUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Managed service identity (either system assigned, or none)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<SystemAssignedServiceIdentity>,
    #[doc = "The properties of a private cloud resource that may be updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateCloudUpdateProperties>,
}
impl PrivateCloudUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a private cloud resource that may be updated"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateCloudUpdateProperties {
    #[doc = "The properties of a management cluster"]
    #[serde(rename = "managementCluster", default, skip_serializing_if = "Option::is_none")]
    pub management_cluster: Option<ManagementCluster>,
    #[doc = "Whether internet is enabled or disabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internet: Option<InternetEnum>,
    #[doc = "vCenter Single Sign On Identity Sources"]
    #[serde(
        rename = "identitySources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub identity_sources: Vec<IdentitySource>,
    #[doc = "The properties describing private cloud availability zone distribution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability: Option<AvailabilityProperties>,
    #[doc = "The properties of customer managed encryption key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "Array of additional networks noncontiguous with networkBlock. Networks must be\nunique and non-overlapping across VNet in your subscription, on-premise, and\nthis privateCloud networkBlock attribute. Make sure the CIDR format conforms to\n(A.B.C.D/X)."]
    #[serde(
        rename = "extendedNetworkBlocks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extended_network_blocks: Vec<String>,
    #[doc = "The type of DNS zone."]
    #[serde(rename = "dnsZoneType", default, skip_serializing_if = "Option::is_none")]
    pub dns_zone_type: Option<DnsZoneType>,
}
impl PrivateCloudUpdateProperties {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Subscription quotas"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Quota {
    #[doc = "Remaining hosts quota by sku type"]
    #[serde(rename = "hostsRemaining", default, skip_serializing_if = "Option::is_none")]
    pub hosts_remaining: Option<serde_json::Value>,
    #[doc = "quota enabled"]
    #[serde(rename = "quotaEnabled", default, skip_serializing_if = "Option::is_none")]
    pub quota_enabled: Option<QuotaEnabled>,
}
impl Quota {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "quota enabled"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "QuotaEnabled")]
pub enum QuotaEnabled {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for QuotaEnabled {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for QuotaEnabled {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for QuotaEnabled {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("QuotaEnabled", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("QuotaEnabled", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. E.g. \"/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}\""]
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
#[doc = "A cmdlet available for script execution"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptCmdlet {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a pre-canned script"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScriptCmdletProperties>,
}
impl ScriptCmdlet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies whether a script cmdlet is intended to be invoked only through automation or visible to customers"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScriptCmdletAudience")]
pub enum ScriptCmdletAudience {
    Automation,
    Any,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScriptCmdletAudience {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScriptCmdletAudience {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScriptCmdletAudience {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Automation => serializer.serialize_unit_variant("ScriptCmdletAudience", 0u32, "Automation"),
            Self::Any => serializer.serialize_unit_variant("ScriptCmdletAudience", 1u32, "Any"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties of a pre-canned script"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptCmdletProperties {
    #[doc = "A script cmdlet provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ScriptCmdletProvisioningState>,
    #[doc = "Description of the scripts functionality"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Recommended time limit for execution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[doc = "Specifies whether a script cmdlet is intended to be invoked only through automation or visible to customers"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<ScriptCmdletAudience>,
    #[doc = "Parameters the script will accept"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parameters: Vec<ScriptParameter>,
}
impl ScriptCmdletProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A script cmdlet provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScriptCmdletProvisioningState")]
pub enum ScriptCmdletProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScriptCmdletProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScriptCmdletProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScriptCmdletProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("ScriptCmdletProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ScriptCmdletProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ScriptCmdletProvisioningState", 2u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a ScriptCmdlet list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptCmdletsList {
    #[doc = "The ScriptCmdlet items on this page"]
    pub value: Vec<ScriptCmdlet>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScriptCmdletsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ScriptCmdletsList {
    pub fn new(value: Vec<ScriptCmdlet>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "An instance of a script executed by a user - custom or AVS"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptExecution {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a user-invoked script"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScriptExecutionProperties>,
}
impl ScriptExecution {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The arguments passed in to the execution"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptExecutionParameter {
    #[doc = "The parameter name"]
    pub name: String,
}
impl ScriptExecutionParameter {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "script execution parameter type"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ScriptExecutionParameterUnion {
    Credential(PsCredentialExecutionParameter),
    SecureValue(ScriptSecureStringExecutionParameter),
    Value(ScriptStringExecutionParameter),
}
#[doc = "script execution parameter type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScriptExecutionParameterType")]
pub enum ScriptExecutionParameterType {
    Value,
    SecureValue,
    Credential,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScriptExecutionParameterType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScriptExecutionParameterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScriptExecutionParameterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Value => serializer.serialize_unit_variant("ScriptExecutionParameterType", 0u32, "Value"),
            Self::SecureValue => serializer.serialize_unit_variant("ScriptExecutionParameterType", 1u32, "SecureValue"),
            Self::Credential => serializer.serialize_unit_variant("ScriptExecutionParameterType", 2u32, "Credential"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties of a user-invoked script"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptExecutionProperties {
    #[doc = "A reference to the script cmdlet resource if user is running a AVS script"]
    #[serde(rename = "scriptCmdletId", default, skip_serializing_if = "Option::is_none")]
    pub script_cmdlet_id: Option<String>,
    #[doc = "Parameters the script will accept"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parameters: Vec<ScriptExecutionParameterUnion>,
    #[doc = "Parameters that will be hidden/not visible to ARM, such as passwords and\ncredentials"]
    #[serde(
        rename = "hiddenParameters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hidden_parameters: Vec<ScriptExecutionParameterUnion>,
    #[doc = "Error message if the script was able to run, but if the script itself had\nerrors or powershell threw an exception"]
    #[serde(rename = "failureReason", default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[doc = "Time limit for execution"]
    pub timeout: String,
    #[doc = "Time to live for the resource. If not provided, will be available for 60 days"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<String>,
    #[doc = "Time the script execution was submitted"]
    #[serde(rename = "submittedAt", default, with = "azure_core::date::rfc3339::option")]
    pub submitted_at: Option<::time::OffsetDateTime>,
    #[doc = "Time the script execution was started"]
    #[serde(rename = "startedAt", default, with = "azure_core::date::rfc3339::option")]
    pub started_at: Option<::time::OffsetDateTime>,
    #[doc = "Time the script execution was finished"]
    #[serde(rename = "finishedAt", default, with = "azure_core::date::rfc3339::option")]
    pub finished_at: Option<::time::OffsetDateTime>,
    #[doc = "Script Execution provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ScriptExecutionProvisioningState>,
    #[doc = "Standard output stream from the powershell execution"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<String>,
    #[doc = "User-defined dictionary."]
    #[serde(rename = "namedOutputs", default, skip_serializing_if = "Option::is_none")]
    pub named_outputs: Option<serde_json::Value>,
    #[doc = "Standard information out stream from the powershell execution"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub information: Vec<String>,
    #[doc = "Standard warning out stream from the powershell execution"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub warnings: Vec<String>,
    #[doc = "Standard error output stream from the powershell execution"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<String>,
}
impl ScriptExecutionProperties {
    pub fn new(timeout: String) -> Self {
        Self {
            script_cmdlet_id: None,
            parameters: Vec::new(),
            hidden_parameters: Vec::new(),
            failure_reason: None,
            timeout,
            retention: None,
            submitted_at: None,
            started_at: None,
            finished_at: None,
            provisioning_state: None,
            output: Vec::new(),
            named_outputs: None,
            information: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }
}
#[doc = "Script Execution provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScriptExecutionProvisioningState")]
pub enum ScriptExecutionProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Pending,
    Running,
    Cancelling,
    Cancelled,
    Deleting,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScriptExecutionProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScriptExecutionProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScriptExecutionProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("ScriptExecutionProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ScriptExecutionProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ScriptExecutionProvisioningState", 2u32, "Canceled"),
            Self::Pending => serializer.serialize_unit_variant("ScriptExecutionProvisioningState", 3u32, "Pending"),
            Self::Running => serializer.serialize_unit_variant("ScriptExecutionProvisioningState", 4u32, "Running"),
            Self::Cancelling => serializer.serialize_unit_variant("ScriptExecutionProvisioningState", 5u32, "Cancelling"),
            Self::Cancelled => serializer.serialize_unit_variant("ScriptExecutionProvisioningState", 6u32, "Cancelled"),
            Self::Deleting => serializer.serialize_unit_variant("ScriptExecutionProvisioningState", 7u32, "Deleting"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a ScriptExecution list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptExecutionsList {
    #[doc = "The ScriptExecution items on this page"]
    pub value: Vec<ScriptExecution>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScriptExecutionsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ScriptExecutionsList {
    pub fn new(value: Vec<ScriptExecution>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Script Output Stream type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScriptOutputStreamType")]
pub enum ScriptOutputStreamType {
    Information,
    Warning,
    Output,
    Error,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScriptOutputStreamType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScriptOutputStreamType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScriptOutputStreamType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Information => serializer.serialize_unit_variant("ScriptOutputStreamType", 0u32, "Information"),
            Self::Warning => serializer.serialize_unit_variant("ScriptOutputStreamType", 1u32, "Warning"),
            Self::Output => serializer.serialize_unit_variant("ScriptOutputStreamType", 2u32, "Output"),
            Self::Error => serializer.serialize_unit_variant("ScriptOutputStreamType", 3u32, "Error"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Script Package resources available for execution"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptPackage {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a Script Package subresource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScriptPackageProperties>,
}
impl ScriptPackage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Script Package subresource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptPackageProperties {
    #[doc = "Script Package provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ScriptPackageProvisioningState>,
    #[doc = "User friendly description of the package"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Module version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Company that created and supports the package"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[doc = "Link to support by the package vendor"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl ScriptPackageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Script Package provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScriptPackageProvisioningState")]
pub enum ScriptPackageProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScriptPackageProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScriptPackageProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScriptPackageProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("ScriptPackageProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ScriptPackageProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ScriptPackageProvisioningState", 2u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a ScriptPackage list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptPackagesList {
    #[doc = "The ScriptPackage items on this page"]
    pub value: Vec<ScriptPackage>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScriptPackagesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ScriptPackagesList {
    pub fn new(value: Vec<ScriptPackage>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "An parameter that the script will accept"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptParameter {
    #[doc = "Script Parameter types"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ScriptParameterTypes>,
    #[doc = "The parameter name that the script will expect a parameter value for"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "User friendly description of the parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Visibility Parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<VisibilityParameterEnum>,
    #[doc = "Optional Param"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<OptionalParamEnum>,
}
impl ScriptParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Script Parameter types"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScriptParameterTypes")]
pub enum ScriptParameterTypes {
    String,
    SecureString,
    Credential,
    Int,
    Bool,
    Float,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScriptParameterTypes {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScriptParameterTypes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScriptParameterTypes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::String => serializer.serialize_unit_variant("ScriptParameterTypes", 0u32, "String"),
            Self::SecureString => serializer.serialize_unit_variant("ScriptParameterTypes", 1u32, "SecureString"),
            Self::Credential => serializer.serialize_unit_variant("ScriptParameterTypes", 2u32, "Credential"),
            Self::Int => serializer.serialize_unit_variant("ScriptParameterTypes", 3u32, "Int"),
            Self::Bool => serializer.serialize_unit_variant("ScriptParameterTypes", 4u32, "Bool"),
            Self::Float => serializer.serialize_unit_variant("ScriptParameterTypes", 5u32, "Float"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "a plain text value execution parameter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptSecureStringExecutionParameter {
    #[serde(flatten)]
    pub script_execution_parameter: ScriptExecutionParameter,
    #[doc = "A secure value for the passed parameter, not to be stored in logs"]
    #[serde(rename = "secureValue", default, skip_serializing_if = "Option::is_none")]
    pub secure_value: Option<String>,
}
impl ScriptSecureStringExecutionParameter {
    pub fn new(script_execution_parameter: ScriptExecutionParameter) -> Self {
        Self {
            script_execution_parameter,
            secure_value: None,
        }
    }
}
#[doc = "a plain text value execution parameter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptStringExecutionParameter {
    #[serde(flatten)]
    pub script_execution_parameter: ScriptExecutionParameter,
    #[doc = "The value for the passed parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ScriptStringExecutionParameter {
    pub fn new(script_execution_parameter: ScriptExecutionParameter) -> Self {
        Self {
            script_execution_parameter,
            value: None,
        }
    }
}
#[doc = "Segment status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SegmentStatusEnum")]
pub enum SegmentStatusEnum {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SegmentStatusEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SegmentStatusEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SegmentStatusEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Success => serializer.serialize_unit_variant("SegmentStatusEnum", 0u32, "SUCCESS"),
            Self::Failure => serializer.serialize_unit_variant("SegmentStatusEnum", 1u32, "FAILURE"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition representing SKU"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the SKU. E.g. P3. It is typically a letter+number code"]
    pub name: String,
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<SkuTier>,
    #[doc = "The SKU size. When the name field is the combination of tier and some other value, this would be the standalone code. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "If the service has different generations of hardware, for the same SKU, then that can be captured here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "If the SKU supports scale out/in then the capacity integer should be included. If scale out/in is not possible for the resource this may be omitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
#[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SkuTier {
    Free,
    Basic,
    Standard,
    Premium,
}
#[doc = "Whether SSL is enabled or disabled"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SslEnum")]
pub enum SslEnum {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SslEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SslEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SslEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("SslEnum", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("SslEnum", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Managed service identity (either system assigned, or none)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemAssignedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (either system assigned, or none)."]
    #[serde(rename = "type")]
    pub type_: SystemAssignedServiceIdentityType,
}
impl SystemAssignedServiceIdentity {
    pub fn new(type_: SystemAssignedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
        }
    }
}
#[doc = "Type of managed service identity (either system assigned, or none)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SystemAssignedServiceIdentityType")]
pub enum SystemAssignedServiceIdentityType {
    None,
    SystemAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SystemAssignedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SystemAssignedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SystemAssignedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("SystemAssignedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("SystemAssignedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "Subscription trial availability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Trial {
    #[doc = "trial status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TrialStatus>,
    #[doc = "Number of trial hosts available"]
    #[serde(rename = "availableHosts", default, skip_serializing_if = "Option::is_none")]
    pub available_hosts: Option<i32>,
}
impl Trial {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "trial status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TrialStatus")]
pub enum TrialStatus {
    TrialAvailable,
    TrialUsed,
    TrialDisabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TrialStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TrialStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TrialStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::TrialAvailable => serializer.serialize_unit_variant("TrialStatus", 0u32, "TrialAvailable"),
            Self::TrialUsed => serializer.serialize_unit_variant("TrialStatus", 1u32, "TrialUsed"),
            Self::TrialDisabled => serializer.serialize_unit_variant("TrialStatus", 2u32, "TrialDisabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "VM group status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VmGroupStatusEnum")]
pub enum VmGroupStatusEnum {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VmGroupStatusEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VmGroupStatusEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VmGroupStatusEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Success => serializer.serialize_unit_variant("VmGroupStatusEnum", 0u32, "SUCCESS"),
            Self::Failure => serializer.serialize_unit_variant("VmGroupStatusEnum", 1u32, "FAILURE"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "VM type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VmTypeEnum")]
pub enum VmTypeEnum {
    #[serde(rename = "REGULAR")]
    Regular,
    #[serde(rename = "EDGE")]
    Edge,
    #[serde(rename = "SERVICE")]
    Service,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VmTypeEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VmTypeEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VmTypeEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Regular => serializer.serialize_unit_variant("VmTypeEnum", 0u32, "REGULAR"),
            Self::Edge => serializer.serialize_unit_variant("VmTypeEnum", 1u32, "EDGE"),
            Self::Service => serializer.serialize_unit_variant("VmTypeEnum", 2u32, "SERVICE"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Virtual Machine"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachine {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Virtual Machine Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineProperties>,
}
impl VirtualMachine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual Machine Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineProperties {
    #[doc = "Virtual Machine provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<VirtualMachineProvisioningState>,
    #[doc = "Display name of the VM."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Virtual machine managed object reference id"]
    #[serde(rename = "moRefId", default, skip_serializing_if = "Option::is_none")]
    pub mo_ref_id: Option<String>,
    #[doc = "Path to virtual machine's folder starting from datacenter virtual machine folder"]
    #[serde(rename = "folderPath", default, skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<String>,
    #[doc = "Virtual Machine Restrict Movement state"]
    #[serde(rename = "restrictMovement", default, skip_serializing_if = "Option::is_none")]
    pub restrict_movement: Option<VirtualMachineRestrictMovementState>,
}
impl VirtualMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual Machine provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VirtualMachineProvisioningState")]
pub enum VirtualMachineProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VirtualMachineProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VirtualMachineProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VirtualMachineProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("VirtualMachineProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("VirtualMachineProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("VirtualMachineProvisioningState", 2u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Set VM DRS-driven movement to restricted (enabled) or not (disabled)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineRestrictMovement {
    #[doc = "Virtual Machine Restrict Movement state"]
    #[serde(rename = "restrictMovement", default, skip_serializing_if = "Option::is_none")]
    pub restrict_movement: Option<VirtualMachineRestrictMovementState>,
}
impl VirtualMachineRestrictMovement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual Machine Restrict Movement state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VirtualMachineRestrictMovementState")]
pub enum VirtualMachineRestrictMovementState {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VirtualMachineRestrictMovementState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VirtualMachineRestrictMovementState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VirtualMachineRestrictMovementState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("VirtualMachineRestrictMovementState", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("VirtualMachineRestrictMovementState", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a VirtualMachine list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachinesList {
    #[doc = "The VirtualMachine items on this page"]
    pub value: Vec<VirtualMachine>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualMachinesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VirtualMachinesList {
    pub fn new(value: Vec<VirtualMachine>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Visibility Parameter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VisibilityParameterEnum")]
pub enum VisibilityParameterEnum {
    Visible,
    Hidden,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VisibilityParameterEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VisibilityParameterEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VisibilityParameterEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Visible => serializer.serialize_unit_variant("VisibilityParameterEnum", 0u32, "Visible"),
            Self::Hidden => serializer.serialize_unit_variant("VisibilityParameterEnum", 1u32, "Hidden"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "VM-Host placement policy properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmHostPlacementPolicyProperties {
    #[serde(flatten)]
    pub placement_policy_properties: PlacementPolicyProperties,
    #[doc = "Virtual machine members list"]
    #[serde(rename = "vmMembers")]
    pub vm_members: Vec<String>,
    #[doc = "Host members list"]
    #[serde(rename = "hostMembers")]
    pub host_members: Vec<String>,
    #[doc = "Affinity type"]
    #[serde(rename = "affinityType")]
    pub affinity_type: AffinityType,
    #[doc = "Affinity Strength"]
    #[serde(rename = "affinityStrength", default, skip_serializing_if = "Option::is_none")]
    pub affinity_strength: Option<AffinityStrength>,
    #[doc = "Azure Hybrid Benefit type"]
    #[serde(rename = "azureHybridBenefitType", default, skip_serializing_if = "Option::is_none")]
    pub azure_hybrid_benefit_type: Option<AzureHybridBenefitType>,
}
impl VmHostPlacementPolicyProperties {
    pub fn new(
        placement_policy_properties: PlacementPolicyProperties,
        vm_members: Vec<String>,
        host_members: Vec<String>,
        affinity_type: AffinityType,
    ) -> Self {
        Self {
            placement_policy_properties,
            vm_members,
            host_members,
            affinity_type,
            affinity_strength: None,
            azure_hybrid_benefit_type: None,
        }
    }
}
#[doc = "VM-VM placement policy properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmVmPlacementPolicyProperties {
    #[serde(flatten)]
    pub placement_policy_properties: PlacementPolicyProperties,
    #[doc = "Virtual machine members list"]
    #[serde(rename = "vmMembers")]
    pub vm_members: Vec<String>,
    #[doc = "Affinity type"]
    #[serde(rename = "affinityType")]
    pub affinity_type: AffinityType,
}
impl VmVmPlacementPolicyProperties {
    pub fn new(placement_policy_properties: PlacementPolicyProperties, vm_members: Vec<String>, affinity_type: AffinityType) -> Self {
        Self {
            placement_policy_properties,
            vm_members,
            affinity_type,
        }
    }
}
#[doc = "Workload Network"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetwork {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a workload network"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkProperties>,
}
impl WorkloadNetwork {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NSX DHCP"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkDhcp {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Base class for WorkloadNetworkDhcpServer and WorkloadNetworkDhcpRelay to\ninherit from"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkDhcpEntityUnion>,
}
impl WorkloadNetworkDhcp {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for WorkloadNetworkDhcpServer and WorkloadNetworkDhcpRelay to\ninherit from"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDhcpEntity {
    #[doc = "Display name of the DHCP entity."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "NSX Segments consuming DHCP."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub segments: Vec<String>,
    #[doc = "Workload Network DHCP provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<WorkloadNetworkDhcpProvisioningState>,
    #[doc = "NSX revision number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
impl WorkloadNetworkDhcpEntity {
    pub fn new() -> Self {
        Self {
            display_name: None,
            segments: Vec::new(),
            provisioning_state: None,
            revision: None,
        }
    }
}
#[doc = "Type of DHCP: SERVER or RELAY."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "dhcpType")]
pub enum WorkloadNetworkDhcpEntityUnion {
    #[serde(rename = "RELAY")]
    Relay(WorkloadNetworkDhcpRelay),
    #[serde(rename = "SERVER")]
    Server(WorkloadNetworkDhcpServer),
}
#[doc = "The response of a WorkloadNetworkDhcp list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDhcpList {
    #[doc = "The WorkloadNetworkDhcp items on this page"]
    pub value: Vec<WorkloadNetworkDhcp>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkDhcpList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkloadNetworkDhcpList {
    pub fn new(value: Vec<WorkloadNetworkDhcp>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Workload Network DHCP provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WorkloadNetworkDhcpProvisioningState")]
pub enum WorkloadNetworkDhcpProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Building,
    Deleting,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WorkloadNetworkDhcpProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WorkloadNetworkDhcpProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WorkloadNetworkDhcpProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("WorkloadNetworkDhcpProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("WorkloadNetworkDhcpProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("WorkloadNetworkDhcpProvisioningState", 2u32, "Canceled"),
            Self::Building => serializer.serialize_unit_variant("WorkloadNetworkDhcpProvisioningState", 3u32, "Building"),
            Self::Deleting => serializer.serialize_unit_variant("WorkloadNetworkDhcpProvisioningState", 4u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("WorkloadNetworkDhcpProvisioningState", 5u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "NSX DHCP Relay"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDhcpRelay {
    #[serde(flatten)]
    pub workload_network_dhcp_entity: WorkloadNetworkDhcpEntity,
    #[doc = "DHCP Relay Addresses. Max 3."]
    #[serde(
        rename = "serverAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub server_addresses: Vec<String>,
}
impl WorkloadNetworkDhcpRelay {
    pub fn new(workload_network_dhcp_entity: WorkloadNetworkDhcpEntity) -> Self {
        Self {
            workload_network_dhcp_entity,
            server_addresses: Vec::new(),
        }
    }
}
#[doc = "NSX DHCP Server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDhcpServer {
    #[serde(flatten)]
    pub workload_network_dhcp_entity: WorkloadNetworkDhcpEntity,
    #[doc = "DHCP Server Address."]
    #[serde(rename = "serverAddress", default, skip_serializing_if = "Option::is_none")]
    pub server_address: Option<String>,
    #[doc = "DHCP Server Lease Time."]
    #[serde(rename = "leaseTime", default, skip_serializing_if = "Option::is_none")]
    pub lease_time: Option<i64>,
}
impl WorkloadNetworkDhcpServer {
    pub fn new(workload_network_dhcp_entity: WorkloadNetworkDhcpEntity) -> Self {
        Self {
            workload_network_dhcp_entity,
            server_address: None,
            lease_time: None,
        }
    }
}
#[doc = "NSX DNS Service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkDnsService {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "NSX DNS Service Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkDnsServiceProperties>,
}
impl WorkloadNetworkDnsService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NSX DNS Service Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkDnsServiceProperties {
    #[doc = "Display name of the DNS Service."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "DNS service IP of the DNS Service."]
    #[serde(rename = "dnsServiceIp", default, skip_serializing_if = "Option::is_none")]
    pub dns_service_ip: Option<String>,
    #[doc = "Default DNS zone of the DNS Service."]
    #[serde(rename = "defaultDnsZone", default, skip_serializing_if = "Option::is_none")]
    pub default_dns_zone: Option<String>,
    #[doc = "FQDN zones of the DNS Service."]
    #[serde(
        rename = "fqdnZones",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fqdn_zones: Vec<String>,
    #[doc = "DNS service log level"]
    #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<DnsServiceLogLevelEnum>,
    #[doc = "DNS service status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DnsServiceStatusEnum>,
    #[doc = "Workload Network DNS Service provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<WorkloadNetworkDnsServiceProvisioningState>,
    #[doc = "NSX revision number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
impl WorkloadNetworkDnsServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload Network DNS Service provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WorkloadNetworkDnsServiceProvisioningState")]
pub enum WorkloadNetworkDnsServiceProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Building,
    Deleting,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WorkloadNetworkDnsServiceProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WorkloadNetworkDnsServiceProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WorkloadNetworkDnsServiceProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("WorkloadNetworkDnsServiceProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("WorkloadNetworkDnsServiceProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("WorkloadNetworkDnsServiceProvisioningState", 2u32, "Canceled"),
            Self::Building => serializer.serialize_unit_variant("WorkloadNetworkDnsServiceProvisioningState", 3u32, "Building"),
            Self::Deleting => serializer.serialize_unit_variant("WorkloadNetworkDnsServiceProvisioningState", 4u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("WorkloadNetworkDnsServiceProvisioningState", 5u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a WorkloadNetworkDnsService list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDnsServicesList {
    #[doc = "The WorkloadNetworkDnsService items on this page"]
    pub value: Vec<WorkloadNetworkDnsService>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkDnsServicesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkloadNetworkDnsServicesList {
    pub fn new(value: Vec<WorkloadNetworkDnsService>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "NSX DNS Zone"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkDnsZone {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "NSX DNS Zone Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkDnsZoneProperties>,
}
impl WorkloadNetworkDnsZone {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NSX DNS Zone Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkDnsZoneProperties {
    #[doc = "Display name of the DNS Zone."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Domain names of the DNS Zone."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub domain: Vec<String>,
    #[doc = "DNS Server IP array of the DNS Zone."]
    #[serde(
        rename = "dnsServerIps",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dns_server_ips: Vec<String>,
    #[doc = "Source IP of the DNS Zone."]
    #[serde(rename = "sourceIp", default, skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
    #[doc = "Number of DNS Services using the DNS zone."]
    #[serde(rename = "dnsServices", default, skip_serializing_if = "Option::is_none")]
    pub dns_services: Option<i64>,
    #[doc = "Workload Network DNS Zone provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<WorkloadNetworkDnsZoneProvisioningState>,
    #[doc = "NSX revision number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
impl WorkloadNetworkDnsZoneProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload Network DNS Zone provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WorkloadNetworkDnsZoneProvisioningState")]
pub enum WorkloadNetworkDnsZoneProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Building,
    Deleting,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WorkloadNetworkDnsZoneProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WorkloadNetworkDnsZoneProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WorkloadNetworkDnsZoneProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("WorkloadNetworkDnsZoneProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("WorkloadNetworkDnsZoneProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("WorkloadNetworkDnsZoneProvisioningState", 2u32, "Canceled"),
            Self::Building => serializer.serialize_unit_variant("WorkloadNetworkDnsZoneProvisioningState", 3u32, "Building"),
            Self::Deleting => serializer.serialize_unit_variant("WorkloadNetworkDnsZoneProvisioningState", 4u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("WorkloadNetworkDnsZoneProvisioningState", 5u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a WorkloadNetworkDnsZone list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDnsZonesList {
    #[doc = "The WorkloadNetworkDnsZone items on this page"]
    pub value: Vec<WorkloadNetworkDnsZone>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkDnsZonesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkloadNetworkDnsZonesList {
    pub fn new(value: Vec<WorkloadNetworkDnsZone>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "NSX Gateway."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkGateway {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a NSX Gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkGatewayProperties>,
}
impl WorkloadNetworkGateway {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a WorkloadNetworkGateway list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkGatewayList {
    #[doc = "The WorkloadNetworkGateway items on this page"]
    pub value: Vec<WorkloadNetworkGateway>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkGatewayList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkloadNetworkGatewayList {
    pub fn new(value: Vec<WorkloadNetworkGateway>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties of a NSX Gateway."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkGatewayProperties {
    #[doc = "base Workload Network provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<WorkloadNetworkProvisioningState>,
    #[doc = "Display name of the DHCP entity."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "NSX Gateway Path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl WorkloadNetworkGatewayProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a WorkloadNetwork list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkList {
    #[doc = "The WorkloadNetwork items on this page"]
    pub value: Vec<WorkloadNetwork>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkloadNetworkList {
    pub fn new(value: Vec<WorkloadNetwork>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "NSX Port Mirroring"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkPortMirroring {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "NSX Port Mirroring Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkPortMirroringProperties>,
}
impl WorkloadNetworkPortMirroring {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a WorkloadNetworkPortMirroring list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkPortMirroringList {
    #[doc = "The WorkloadNetworkPortMirroring items on this page"]
    pub value: Vec<WorkloadNetworkPortMirroring>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkPortMirroringList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkloadNetworkPortMirroringList {
    pub fn new(value: Vec<WorkloadNetworkPortMirroring>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "NSX Port Mirroring Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkPortMirroringProperties {
    #[doc = "Display name of the port mirroring profile."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Port Mirroring Direction"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<PortMirroringDirectionEnum>,
    #[doc = "Source VM Group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Destination VM Group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[doc = "Port Mirroring status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PortMirroringStatusEnum>,
    #[doc = "Workload Network Port Mirroring provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<WorkloadNetworkPortMirroringProvisioningState>,
    #[doc = "NSX revision number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
impl WorkloadNetworkPortMirroringProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload Network Port Mirroring provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WorkloadNetworkPortMirroringProvisioningState")]
pub enum WorkloadNetworkPortMirroringProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Building,
    Deleting,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WorkloadNetworkPortMirroringProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WorkloadNetworkPortMirroringProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WorkloadNetworkPortMirroringProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("WorkloadNetworkPortMirroringProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("WorkloadNetworkPortMirroringProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("WorkloadNetworkPortMirroringProvisioningState", 2u32, "Canceled"),
            Self::Building => serializer.serialize_unit_variant("WorkloadNetworkPortMirroringProvisioningState", 3u32, "Building"),
            Self::Deleting => serializer.serialize_unit_variant("WorkloadNetworkPortMirroringProvisioningState", 4u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("WorkloadNetworkPortMirroringProvisioningState", 5u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties of a workload network"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkProperties {
    #[doc = "base Workload Network provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<WorkloadNetworkProvisioningState>,
}
impl WorkloadNetworkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "base Workload Network provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WorkloadNetworkProvisioningState")]
pub enum WorkloadNetworkProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Building,
    Deleting,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WorkloadNetworkProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WorkloadNetworkProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WorkloadNetworkProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("WorkloadNetworkProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("WorkloadNetworkProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("WorkloadNetworkProvisioningState", 2u32, "Canceled"),
            Self::Building => serializer.serialize_unit_variant("WorkloadNetworkProvisioningState", 3u32, "Building"),
            Self::Deleting => serializer.serialize_unit_variant("WorkloadNetworkProvisioningState", 4u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("WorkloadNetworkProvisioningState", 5u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "NSX Public IP Block"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkPublicIp {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "NSX Public IP Block Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkPublicIpProperties>,
}
impl WorkloadNetworkPublicIp {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NSX Public IP Block Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkPublicIpProperties {
    #[doc = "Display name of the Public IP Block."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Number of Public IPs requested."]
    #[serde(rename = "numberOfPublicIPs", default, skip_serializing_if = "Option::is_none")]
    pub number_of_public_i_ps: Option<i64>,
    #[doc = "CIDR Block of the Public IP Block."]
    #[serde(rename = "publicIPBlock", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_block: Option<String>,
    #[doc = "Workload Network Public IP provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<WorkloadNetworkPublicIpProvisioningState>,
}
impl WorkloadNetworkPublicIpProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload Network Public IP provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WorkloadNetworkPublicIpProvisioningState")]
pub enum WorkloadNetworkPublicIpProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Building,
    Deleting,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WorkloadNetworkPublicIpProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WorkloadNetworkPublicIpProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WorkloadNetworkPublicIpProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("WorkloadNetworkPublicIpProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("WorkloadNetworkPublicIpProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("WorkloadNetworkPublicIpProvisioningState", 2u32, "Canceled"),
            Self::Building => serializer.serialize_unit_variant("WorkloadNetworkPublicIpProvisioningState", 3u32, "Building"),
            Self::Deleting => serializer.serialize_unit_variant("WorkloadNetworkPublicIpProvisioningState", 4u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("WorkloadNetworkPublicIpProvisioningState", 5u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a WorkloadNetworkPublicIP list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkPublicIPsList {
    #[doc = "The WorkloadNetworkPublicIP items on this page"]
    pub value: Vec<WorkloadNetworkPublicIp>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkPublicIPsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkloadNetworkPublicIPsList {
    pub fn new(value: Vec<WorkloadNetworkPublicIp>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "NSX Segment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkSegment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "NSX Segment Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkSegmentProperties>,
}
impl WorkloadNetworkSegment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Ports and any VIF attached to segment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkSegmentPortVif {
    #[doc = "Name of port or VIF attached to segment."]
    #[serde(rename = "portName", default, skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
}
impl WorkloadNetworkSegmentPortVif {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NSX Segment Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkSegmentProperties {
    #[doc = "Display name of the segment."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gateway which to connect segment to."]
    #[serde(rename = "connectedGateway", default, skip_serializing_if = "Option::is_none")]
    pub connected_gateway: Option<String>,
    #[doc = "Subnet configuration for segment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<WorkloadNetworkSegmentSubnet>,
    #[doc = "Port Vif which segment is associated with."]
    #[serde(
        rename = "portVif",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub port_vif: Vec<WorkloadNetworkSegmentPortVif>,
    #[doc = "Segment status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SegmentStatusEnum>,
    #[doc = "Workload Network Segment provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<WorkloadNetworkSegmentProvisioningState>,
    #[doc = "NSX revision number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
impl WorkloadNetworkSegmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload Network Segment provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WorkloadNetworkSegmentProvisioningState")]
pub enum WorkloadNetworkSegmentProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Building,
    Deleting,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WorkloadNetworkSegmentProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WorkloadNetworkSegmentProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WorkloadNetworkSegmentProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("WorkloadNetworkSegmentProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("WorkloadNetworkSegmentProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("WorkloadNetworkSegmentProvisioningState", 2u32, "Canceled"),
            Self::Building => serializer.serialize_unit_variant("WorkloadNetworkSegmentProvisioningState", 3u32, "Building"),
            Self::Deleting => serializer.serialize_unit_variant("WorkloadNetworkSegmentProvisioningState", 4u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("WorkloadNetworkSegmentProvisioningState", 5u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Subnet configuration for segment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkSegmentSubnet {
    #[doc = "DHCP Range assigned for subnet."]
    #[serde(
        rename = "dhcpRanges",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dhcp_ranges: Vec<String>,
    #[doc = "Gateway address."]
    #[serde(rename = "gatewayAddress", default, skip_serializing_if = "Option::is_none")]
    pub gateway_address: Option<String>,
}
impl WorkloadNetworkSegmentSubnet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a WorkloadNetworkSegment list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkSegmentsList {
    #[doc = "The WorkloadNetworkSegment items on this page"]
    pub value: Vec<WorkloadNetworkSegment>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkSegmentsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkloadNetworkSegmentsList {
    pub fn new(value: Vec<WorkloadNetworkSegment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "NSX VM Group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkVmGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "NSX VM Group Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkVmGroupProperties>,
}
impl WorkloadNetworkVmGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NSX VM Group Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkVmGroupProperties {
    #[doc = "Display name of the VM group."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Virtual machine members of this group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub members: Vec<String>,
    #[doc = "VM group status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<VmGroupStatusEnum>,
    #[doc = "Workload Network VM Group provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<WorkloadNetworkVmGroupProvisioningState>,
    #[doc = "NSX revision number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
impl WorkloadNetworkVmGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload Network VM Group provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WorkloadNetworkVmGroupProvisioningState")]
pub enum WorkloadNetworkVmGroupProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Building,
    Deleting,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WorkloadNetworkVmGroupProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WorkloadNetworkVmGroupProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WorkloadNetworkVmGroupProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("WorkloadNetworkVmGroupProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("WorkloadNetworkVmGroupProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("WorkloadNetworkVmGroupProvisioningState", 2u32, "Canceled"),
            Self::Building => serializer.serialize_unit_variant("WorkloadNetworkVmGroupProvisioningState", 3u32, "Building"),
            Self::Deleting => serializer.serialize_unit_variant("WorkloadNetworkVmGroupProvisioningState", 4u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("WorkloadNetworkVmGroupProvisioningState", 5u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a WorkloadNetworkVMGroup list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkVmGroupsList {
    #[doc = "The WorkloadNetworkVMGroup items on this page"]
    pub value: Vec<WorkloadNetworkVmGroup>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkVmGroupsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkloadNetworkVmGroupsList {
    pub fn new(value: Vec<WorkloadNetworkVmGroup>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "NSX Virtual Machine"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkVirtualMachine {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "NSX Virtual Machine Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkVirtualMachineProperties>,
}
impl WorkloadNetworkVirtualMachine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NSX Virtual Machine Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkVirtualMachineProperties {
    #[doc = "base Workload Network provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<WorkloadNetworkProvisioningState>,
    #[doc = "Display name of the VM."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "VM type"]
    #[serde(rename = "vmType", default, skip_serializing_if = "Option::is_none")]
    pub vm_type: Option<VmTypeEnum>,
}
impl WorkloadNetworkVirtualMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a WorkloadNetworkVirtualMachine list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkVirtualMachinesList {
    #[doc = "The WorkloadNetworkVirtualMachine items on this page"]
    pub value: Vec<WorkloadNetworkVirtualMachine>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkVirtualMachinesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkloadNetworkVirtualMachinesList {
    pub fn new(value: Vec<WorkloadNetworkVirtualMachine>) -> Self {
        Self { value, next_link: None }
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
    pub created_at: Option<::time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<::time::OffsetDateTime>,
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

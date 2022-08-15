#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An addon resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Addon {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of an addon"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AddonProperties>,
}
impl Addon {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "A paged list of addons"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddonList {
    #[doc = "The items on a page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Addon>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AddonList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AddonList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an addon"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddonProperties {
    #[doc = "The type of private cloud addon"]
    #[serde(rename = "addonType")]
    pub addon_type: addon_properties::AddonType,
    #[doc = "The state of the addon provisioning"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<addon_properties::ProvisioningState>,
}
impl AddonProperties {
    pub fn new(addon_type: addon_properties::AddonType) -> Self {
        Self {
            addon_type,
            provisioning_state: None,
        }
    }
}
pub mod addon_properties {
    use super::*;
    #[doc = "The type of private cloud addon"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AddonType")]
    pub enum AddonType {
        #[serde(rename = "SRM")]
        Srm,
        #[serde(rename = "VR")]
        Vr,
        #[serde(rename = "HCX")]
        Hcx,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The state of the addon provisioning"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Cancelled,
        Building,
        Deleting,
        Updating,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Cancelled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Cancelled"),
                Self::Building => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Building"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "Placement policy affinity type"]
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
    #[doc = "The availability strategy for the private cloud"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<availability_properties::Strategy>,
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
pub mod availability_properties {
    use super::*;
    #[doc = "The availability strategy for the private cloud"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Strategy")]
    pub enum Strategy {
        SingleZone,
        DualZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Strategy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Strategy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Strategy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SingleZone => serializer.serialize_unit_variant("Strategy", 0u32, "SingleZone"),
                Self::DualZone => serializer.serialize_unit_variant("Strategy", 1u32, "DualZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "API error response"]
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
#[doc = "A cloud link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudLink {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a cloud link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudLinkProperties>,
}
impl CloudLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paged list of cloud links"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudLinkList {
    #[doc = "The items on a page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CloudLink>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CloudLinkList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CloudLinkList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a cloud link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudLinkProperties {
    #[doc = "The state of the cloud link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<cloud_link_properties::Status>,
    #[doc = "Identifier of the other private cloud participating in the link."]
    #[serde(rename = "linkedCloud", default, skip_serializing_if = "Option::is_none")]
    pub linked_cloud: Option<String>,
}
impl CloudLinkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cloud_link_properties {
    use super::*;
    #[doc = "The state of the cloud link."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Active,
        Building,
        Deleting,
        Failed,
        Disconnected,
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
                Self::Active => serializer.serialize_unit_variant("Status", 0u32, "Active"),
                Self::Building => serializer.serialize_unit_variant("Status", 1u32, "Building"),
                Self::Deleting => serializer.serialize_unit_variant("Status", 2u32, "Deleting"),
                Self::Failed => serializer.serialize_unit_variant("Status", 3u32, "Failed"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 4u32, "Disconnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A cluster resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The resource model definition representing SKU"]
    pub sku: Sku,
    #[doc = "The properties of a cluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
impl Cluster {
    pub fn new(sku: Sku) -> Self {
        Self {
            resource: Resource::default(),
            sku,
            properties: None,
        }
    }
}
#[doc = "A paged list of clusters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterList {
    #[doc = "The items on a page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cluster>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ClusterList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ClusterList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterProperties {
    #[serde(flatten)]
    pub common_cluster_properties: CommonClusterProperties,
}
impl ClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The state of the cluster provisioning"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ClusterProvisioningState")]
pub enum ClusterProvisioningState {
    Succeeded,
    Failed,
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
            Self::Cancelled => serializer.serialize_unit_variant("ClusterProvisioningState", 2u32, "Cancelled"),
            Self::Deleting => serializer.serialize_unit_variant("ClusterProvisioningState", 3u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("ClusterProvisioningState", 4u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An update of a cluster resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpdate {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hosts: Vec<String>,
}
impl ClusterUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The common properties of a cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommonClusterProperties {
    #[doc = "The cluster size"]
    #[serde(rename = "clusterSize", default, skip_serializing_if = "Option::is_none")]
    pub cluster_size: Option<i32>,
    #[doc = "The state of the cluster provisioning"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ClusterProvisioningState>,
    #[doc = "The identity"]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<i32>,
    #[doc = "The hosts"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hosts: Vec<String>,
}
impl CommonClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A datastore resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Datastore {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a datastore"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatastoreProperties>,
}
impl Datastore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paged list of datastores"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatastoreList {
    #[doc = "The items on a page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Datastore>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatastoreList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DatastoreList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a datastore"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatastoreProperties {
    #[doc = "The state of the datastore provisioning"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<datastore_properties::ProvisioningState>,
    #[doc = "An Azure NetApp Files volume from Microsoft.NetApp provider"]
    #[serde(rename = "netAppVolume", default, skip_serializing_if = "Option::is_none")]
    pub net_app_volume: Option<NetAppVolume>,
    #[doc = "An iSCSI volume from Microsoft.StoragePool provider"]
    #[serde(rename = "diskPoolVolume", default, skip_serializing_if = "Option::is_none")]
    pub disk_pool_volume: Option<DiskPoolVolume>,
    #[doc = "The operational status of the datastore"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<datastore_properties::Status>,
}
impl DatastoreProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod datastore_properties {
    use super::*;
    #[doc = "The state of the datastore provisioning"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Cancelled,
        Pending,
        Creating,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Cancelled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Cancelled"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Pending"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The operational status of the datastore"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
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
                Self::Unknown => serializer.serialize_unit_variant("Status", 0u32, "Unknown"),
                Self::Accessible => serializer.serialize_unit_variant("Status", 1u32, "Accessible"),
                Self::Inaccessible => serializer.serialize_unit_variant("Status", 2u32, "Inaccessible"),
                Self::Attached => serializer.serialize_unit_variant("Status", 3u32, "Attached"),
                Self::Detached => serializer.serialize_unit_variant("Status", 4u32, "Detached"),
                Self::LostCommunication => serializer.serialize_unit_variant("Status", 5u32, "LostCommunication"),
                Self::DeadOrError => serializer.serialize_unit_variant("Status", 6u32, "DeadOrError"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
    #[doc = "Mode that describes whether the LUN has to be mounted as a datastore or attached as a LUN"]
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
    #[doc = "Mode that describes whether the LUN has to be mounted as a datastore or attached as a LUN"]
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
#[doc = "The properties of customer managed encryption key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Encryption {
    #[doc = "Status of customer managed encryption key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<encryption::Status>,
    #[doc = "An Encryption Key"]
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<EncryptionKeyVaultProperties>,
}
impl Encryption {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption {
    use super::*;
    #[doc = "Status of customer managed encryption key"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
    #[doc = "The URL of the vault."]
    #[serde(rename = "keyVaultUrl", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_url: Option<String>,
    #[doc = "The state of key provided"]
    #[serde(rename = "keyState", default, skip_serializing_if = "Option::is_none")]
    pub key_state: Option<encryption_key_vault_properties::KeyState>,
    #[doc = "Property of the key if user provided or auto detected"]
    #[serde(rename = "versionType", default, skip_serializing_if = "Option::is_none")]
    pub version_type: Option<encryption_key_vault_properties::VersionType>,
}
impl EncryptionKeyVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption_key_vault_properties {
    use super::*;
    #[doc = "The state of key provided"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KeyState")]
    pub enum KeyState {
        Connected,
        AccessDenied,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KeyState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KeyState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KeyState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Connected => serializer.serialize_unit_variant("KeyState", 0u32, "Connected"),
                Self::AccessDenied => serializer.serialize_unit_variant("KeyState", 1u32, "AccessDenied"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Property of the key if user provided or auto detected"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VersionType")]
    pub enum VersionType {
        Fixed,
        AutoDetected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VersionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VersionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VersionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Fixed => serializer.serialize_unit_variant("VersionType", 0u32, "Fixed"),
                Self::AutoDetected => serializer.serialize_unit_variant("VersionType", 1u32, "AutoDetected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Endpoint addresses"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Endpoints {
    #[doc = "Endpoint for the NSX-T Data Center manager"]
    #[serde(rename = "nsxtManager", default, skip_serializing_if = "Option::is_none")]
    pub nsxt_manager: Option<String>,
    #[doc = "Endpoint for Virtual Center Server Appliance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vcsa: Option<String>,
    #[doc = "Endpoint for the HCX Cloud Manager"]
    #[serde(rename = "hcxCloudManager", default, skip_serializing_if = "Option::is_none")]
    pub hcx_cloud_manager: Option<String>,
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
#[doc = "ExpressRoute Circuit Authorization"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpressRouteAuthorization {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of an ExpressRoute Circuit Authorization resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExpressRouteAuthorizationProperties>,
}
impl ExpressRouteAuthorization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paged list of ExpressRoute Circuit Authorizations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpressRouteAuthorizationList {
    #[doc = "The items on a page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ExpressRouteAuthorization>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExpressRouteAuthorizationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ExpressRouteAuthorizationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an ExpressRoute Circuit Authorization resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpressRouteAuthorizationProperties {
    #[doc = "The state of the  ExpressRoute Circuit Authorization provisioning"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<express_route_authorization_properties::ProvisioningState>,
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
pub mod express_route_authorization_properties {
    use super::*;
    #[doc = "The state of the  ExpressRoute Circuit Authorization provisioning"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Updating,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A global reach connection resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalReachConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a global reach connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GlobalReachConnectionProperties>,
}
impl GlobalReachConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paged list of global reach connections"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalReachConnectionList {
    #[doc = "The items on a page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GlobalReachConnection>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GlobalReachConnectionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GlobalReachConnectionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a global reach connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalReachConnectionProperties {
    #[doc = "The state of the  ExpressRoute Circuit Authorization provisioning"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<global_reach_connection_properties::ProvisioningState>,
    #[doc = "The network used for global reach carved out from the original network block provided for the private cloud"]
    #[serde(rename = "addressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub address_prefix: Option<String>,
    #[doc = "Authorization key from the peer express route used for the global reach connection"]
    #[serde(rename = "authorizationKey", default, skip_serializing_if = "Option::is_none")]
    pub authorization_key: Option<String>,
    #[doc = "The connection status of the global reach connection"]
    #[serde(rename = "circuitConnectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub circuit_connection_status: Option<global_reach_connection_properties::CircuitConnectionStatus>,
    #[doc = "Identifier of the ExpressRoute Circuit to peer with in the global reach connection"]
    #[serde(rename = "peerExpressRouteCircuit", default, skip_serializing_if = "Option::is_none")]
    pub peer_express_route_circuit: Option<String>,
    #[doc = "The ID of the Private Cloud's ExpressRoute Circuit that is participating in the global reach connection"]
    #[serde(rename = "expressRouteId", default, skip_serializing_if = "Option::is_none")]
    pub express_route_id: Option<String>,
}
impl GlobalReachConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod global_reach_connection_properties {
    use super::*;
    #[doc = "The state of the  ExpressRoute Circuit Authorization provisioning"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Updating,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The connection status of the global reach connection"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CircuitConnectionStatus")]
    pub enum CircuitConnectionStatus {
        Connected,
        Connecting,
        Disconnected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CircuitConnectionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CircuitConnectionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CircuitConnectionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Connected => serializer.serialize_unit_variant("CircuitConnectionStatus", 0u32, "Connected"),
                Self::Connecting => serializer.serialize_unit_variant("CircuitConnectionStatus", 1u32, "Connecting"),
                Self::Disconnected => serializer.serialize_unit_variant("CircuitConnectionStatus", 2u32, "Disconnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An HCX Enterprise Site resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HcxEnterpriseSite {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of an HCX Enterprise Site"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HcxEnterpriseSiteProperties>,
}
impl HcxEnterpriseSite {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paged list of HCX Enterprise Sites"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HcxEnterpriseSiteList {
    #[doc = "The items on a page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HcxEnterpriseSite>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HcxEnterpriseSiteList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HcxEnterpriseSiteList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an HCX Enterprise Site"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HcxEnterpriseSiteProperties {
    #[doc = "The activation key"]
    #[serde(rename = "activationKey", default, skip_serializing_if = "Option::is_none")]
    pub activation_key: Option<String>,
    #[doc = "The status of the HCX Enterprise Site"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<hcx_enterprise_site_properties::Status>,
}
impl HcxEnterpriseSiteProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hcx_enterprise_site_properties {
    use super::*;
    #[doc = "The status of the HCX Enterprise Site"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Available,
        Consumed,
        Deactivated,
        Deleted,
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
                Self::Available => serializer.serialize_unit_variant("Status", 0u32, "Available"),
                Self::Consumed => serializer.serialize_unit_variant("Status", 1u32, "Consumed"),
                Self::Deactivated => serializer.serialize_unit_variant("Status", 2u32, "Deactivated"),
                Self::Deleted => serializer.serialize_unit_variant("Status", 3u32, "Deleted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
    #[doc = "Protect LDAP communication using SSL certificate (LDAPS)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl: Option<identity_source::Ssl>,
    #[doc = "The ID of an Active Directory user with a minimum of read-only access to Base DN for users and group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The password of the Active Directory user with a minimum of read-only access to Base DN for users and groups."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl IdentitySource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity_source {
    use super::*;
    #[doc = "Protect LDAP communication using SSL certificate (LDAPS)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Ssl")]
    pub enum Ssl {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Ssl {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Ssl {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Ssl {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("Ssl", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Ssl", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifications of the Log for Azure Monitoring"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[doc = "Name of the log"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the log"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Blob duration of the log"]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a management cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementCluster {
    #[serde(flatten)]
    pub common_cluster_properties: CommonClusterProperties,
}
impl ManagementCluster {
    pub fn new() -> Self {
        Self {
            common_cluster_properties: CommonClusterProperties::default(),
        }
    }
}
#[doc = "Specifications of the Dimension of metrics"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDimension {
    #[doc = "Name of the dimension"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the dimension"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Name of the dimension as it appears in MDM"]
    #[serde(rename = "internalName", default, skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[doc = "A boolean flag indicating whether this dimension should be included for the shoebox export scenario"]
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl MetricDimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifications of the Metrics for Azure Monitoring"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[doc = "Name of the metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the metric"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Localized friendly description of the metric"]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "Unit that makes sense for the metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Name of the metric category that the metric belongs to. A metric can only belong to a single category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Only provide one value for this field. Valid values: Average, Minimum, Maximum, Total, Count."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "Supported aggregation types"]
    #[serde(rename = "supportedAggregationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
    #[doc = "Supported time grain types"]
    #[serde(rename = "supportedTimeGrainTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_time_grain_types: Vec<String>,
    #[doc = "Optional. If set to true, then zero will be returned for time duration where no metric is emitted/published."]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[doc = "Dimensions of the metric"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<MetricDimension>,
    #[doc = "Whether or not the service is using regional MDM accounts."]
    #[serde(rename = "enableRegionalMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub enable_regional_mdm_account: Option<String>,
    #[doc = "The name of the MDM account."]
    #[serde(rename = "sourceMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_account: Option<String>,
    #[doc = "The name of the MDM namespace."]
    #[serde(rename = "sourceMdmNamespace", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_namespace: Option<String>,
}
impl MetricSpecification {
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
#[doc = "A REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation being performed on this object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Contains the localized display information for this operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "Gets or sets a value indicating whether the operation is a data action or not"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Extra Operation properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Contains the localized display information for this operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Localized friendly form of the resource provider name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Localized friendly form of the resource type related to this operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Localized friendly name for the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Localized friendly description for the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Pageable list of operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of operations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next page if any"]
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
#[doc = "Extra Operation properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "Service specification payload"]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Represents list of placement policies"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlacementPoliciesList {
    #[doc = "The items on the page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PlacementPolicy>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PlacementPoliciesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PlacementPoliciesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A vSphere Distributed Resource Scheduler (DRS) placement policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlacementPolicy {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Abstract placement policy properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PlacementPolicyProperties>,
}
impl PlacementPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Abstract placement policy properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlacementPolicyProperties {
    #[doc = "placement policy type"]
    #[serde(rename = "type")]
    pub type_: placement_policy_properties::Type,
    #[doc = "Whether the placement policy is enabled or disabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<placement_policy_properties::State>,
    #[doc = "Display name of the placement policy"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<placement_policy_properties::ProvisioningState>,
}
impl PlacementPolicyProperties {
    pub fn new(type_: placement_policy_properties::Type) -> Self {
        Self {
            type_,
            state: None,
            display_name: None,
            provisioning_state: None,
        }
    }
}
pub mod placement_policy_properties {
    use super::*;
    #[doc = "placement policy type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        VmVm,
        VmHost,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::VmVm => serializer.serialize_unit_variant("Type", 0u32, "VmVm"),
                Self::VmHost => serializer.serialize_unit_variant("Type", 1u32, "VmHost"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether the placement policy is enabled or disabled"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("State", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("State", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Building,
        Deleting,
        Updating,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Building => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Building"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
    #[doc = "Whether the placement policy is enabled or disabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<placement_policy_update_properties::State>,
    #[doc = "Virtual machine members list"]
    #[serde(rename = "vmMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_members: Vec<String>,
    #[doc = "Host members list"]
    #[serde(rename = "hostMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub host_members: Vec<String>,
}
impl PlacementPolicyUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod placement_policy_update_properties {
    use super::*;
    #[doc = "Whether the placement policy is enabled or disabled"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("State", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("State", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A private cloud resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloud {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The resource model definition representing SKU"]
    pub sku: Sku,
    #[doc = "The properties of a private cloud resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateCloudProperties>,
    #[doc = "Identity for the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<PrivateCloudIdentity>,
}
impl PrivateCloud {
    pub fn new(sku: Sku) -> Self {
        Self {
            tracked_resource: TrackedResource::default(),
            sku,
            properties: None,
            identity: None,
        }
    }
}
#[doc = "Identity for the virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateCloudIdentity {
    #[doc = "The principal ID of private cloud identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID associated with the private cloud. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of identity used for the private cloud. The type 'SystemAssigned' refers to an implicitly created identity. The type 'None' will remove any identities from the Private Cloud."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<private_cloud_identity::Type>,
}
impl PrivateCloudIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_cloud_identity {
    use super::*;
    #[doc = "The type of identity used for the private cloud. The type 'SystemAssigned' refers to an implicitly created identity. The type 'None' will remove any identities from the Private Cloud."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        SystemAssigned,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 0u32, "SystemAssigned"),
                Self::None => serializer.serialize_unit_variant("Type", 1u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A paged list of private clouds"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateCloudList {
    #[doc = "The items on the page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateCloud>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateCloudList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateCloudList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a private cloud resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloudProperties {
    #[serde(flatten)]
    pub private_cloud_update_properties: PrivateCloudUpdateProperties,
    #[doc = "The provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_cloud_properties::ProvisioningState>,
    #[doc = "An ExpressRoute Circuit"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circuit: Option<Circuit>,
    #[doc = "Endpoint addresses"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Endpoints>,
    #[doc = "The block of addresses should be unique across VNet in your subscription as well as on-premise. Make sure the CIDR format is conformed to (A.B.C.D/X) where A,B,C,D are between 0 and 255, and X is between 0 and 22"]
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
    #[serde(rename = "externalCloudLinks", default, skip_serializing_if = "Vec::is_empty")]
    pub external_cloud_links: Vec<String>,
    #[doc = "An ExpressRoute Circuit"]
    #[serde(rename = "secondaryCircuit", default, skip_serializing_if = "Option::is_none")]
    pub secondary_circuit: Option<Circuit>,
}
impl PrivateCloudProperties {
    pub fn new(network_block: String) -> Self {
        Self {
            private_cloud_update_properties: PrivateCloudUpdateProperties::default(),
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
        }
    }
}
pub mod private_cloud_properties {
    use super::*;
    #[doc = "The provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Cancelled,
        Pending,
        Building,
        Deleting,
        Updating,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Cancelled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Cancelled"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Pending"),
                Self::Building => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Building"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An update to a private cloud resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateCloudUpdate {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[doc = "The properties of a private cloud resource that may be updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateCloudUpdateProperties>,
    #[doc = "Identity for the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<PrivateCloudIdentity>,
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
    #[doc = "Connectivity to internet is enabled or disabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internet: Option<private_cloud_update_properties::Internet>,
    #[doc = "vCenter Single Sign On Identity Sources"]
    #[serde(rename = "identitySources", default, skip_serializing_if = "Vec::is_empty")]
    pub identity_sources: Vec<IdentitySource>,
    #[doc = "The properties describing private cloud availability zone distribution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability: Option<AvailabilityProperties>,
    #[doc = "The properties of customer managed encryption key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
}
impl PrivateCloudUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_cloud_update_properties {
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
#[doc = "The resource model definition for a ARM proxy resource"]
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
    #[doc = "Host quota is active for current subscription"]
    #[serde(rename = "quotaEnabled", default, skip_serializing_if = "Option::is_none")]
    pub quota_enabled: Option<quota::QuotaEnabled>,
}
impl Quota {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod quota {
    use super::*;
    #[doc = "Host quota is active for current subscription"]
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
}
#[doc = "The core properties of ARM resources"]
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
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTags {}
impl ResourceTags {
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
#[doc = "Properties of a pre-canned script"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptCmdletProperties {
    #[doc = "Description of the scripts functionality"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Recommended time limit for execution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[doc = "Parameters the script will accept"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ScriptParameter>,
}
impl ScriptCmdletProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pageable list of scripts/cmdlets"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptCmdletsList {
    #[doc = "List of scripts"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ScriptCmdlet>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScriptCmdletsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ScriptCmdletsList {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "The type of execution parameter"]
    #[serde(rename = "type")]
    pub type_: script_execution_parameter::Type,
}
impl ScriptExecutionParameter {
    pub fn new(name: String, type_: script_execution_parameter::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod script_execution_parameter {
    use super::*;
    #[doc = "The type of execution parameter"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Value,
        SecureValue,
        Credential,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Value => serializer.serialize_unit_variant("Type", 0u32, "Value"),
                Self::SecureValue => serializer.serialize_unit_variant("Type", 1u32, "SecureValue"),
                Self::Credential => serializer.serialize_unit_variant("Type", 2u32, "Credential"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ScriptExecutionParameter>,
    #[doc = "Parameters that will be hidden/not visible to ARM, such as passwords and credentials"]
    #[serde(rename = "hiddenParameters", default, skip_serializing_if = "Vec::is_empty")]
    pub hidden_parameters: Vec<ScriptExecutionParameter>,
    #[doc = "Error message if the script was able to run, but if the script itself had errors or powershell threw an exception"]
    #[serde(rename = "failureReason", default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[doc = "Time limit for execution"]
    pub timeout: String,
    #[doc = "Time to live for the resource. If not provided, will be available for 60 days"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<String>,
    #[doc = "Time the script execution was submitted"]
    #[serde(rename = "submittedAt", with = "azure_core::date::rfc3339::option")]
    pub submitted_at: Option<time::OffsetDateTime>,
    #[doc = "Time the script execution was started"]
    #[serde(rename = "startedAt", with = "azure_core::date::rfc3339::option")]
    pub started_at: Option<time::OffsetDateTime>,
    #[doc = "Time the script execution was finished"]
    #[serde(rename = "finishedAt", with = "azure_core::date::rfc3339::option")]
    pub finished_at: Option<time::OffsetDateTime>,
    #[doc = "The state of the script execution resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<script_execution_properties::ProvisioningState>,
    #[doc = "Standard output stream from the powershell execution"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output: Vec<String>,
    #[doc = "User-defined dictionary."]
    #[serde(rename = "namedOutputs", default, skip_serializing_if = "Option::is_none")]
    pub named_outputs: Option<serde_json::Value>,
    #[doc = "Standard information out stream from the powershell execution"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information: Vec<String>,
    #[doc = "Standard warning out stream from the powershell execution"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
    #[doc = "Standard error output stream from the powershell execution"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
pub mod script_execution_properties {
    use super::*;
    #[doc = "The state of the script execution resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Pending,
        Running,
        Succeeded,
        Failed,
        Cancelling,
        Cancelled,
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
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Pending"),
                Self::Running => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Running"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Cancelling => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Cancelling"),
                Self::Cancelled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Cancelled"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Pageable list of script executions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptExecutionsList {
    #[doc = "List of scripts"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ScriptExecution>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScriptExecutionsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ScriptExecutionsList {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "User friendly description of the package"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Module version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ScriptPackageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the available script packages"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptPackagesList {
    #[doc = "List of script package resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ScriptPackage>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScriptPackagesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ScriptPackagesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An parameter that the script will accept"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptParameter {
    #[doc = "The type of parameter the script is expecting. psCredential is a PSCredentialObject"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<script_parameter::Type>,
    #[doc = "The parameter name that the script will expect a parameter value for"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "User friendly description of the parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Should this parameter be visible to arm and passed in the parameters argument when executing"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<script_parameter::Visibility>,
    #[doc = "Is this parameter required or optional"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<script_parameter::Optional>,
}
impl ScriptParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod script_parameter {
    use super::*;
    #[doc = "The type of parameter the script is expecting. psCredential is a PSCredentialObject"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        String,
        SecureString,
        Credential,
        Int,
        Bool,
        Float,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::String => serializer.serialize_unit_variant("Type", 0u32, "String"),
                Self::SecureString => serializer.serialize_unit_variant("Type", 1u32, "SecureString"),
                Self::Credential => serializer.serialize_unit_variant("Type", 2u32, "Credential"),
                Self::Int => serializer.serialize_unit_variant("Type", 3u32, "Int"),
                Self::Bool => serializer.serialize_unit_variant("Type", 4u32, "Bool"),
                Self::Float => serializer.serialize_unit_variant("Type", 5u32, "Float"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Should this parameter be visible to arm and passed in the parameters argument when executing"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Visibility")]
    pub enum Visibility {
        Visible,
        Hidden,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Visibility {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Visibility {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Visibility {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Visible => serializer.serialize_unit_variant("Visibility", 0u32, "Visible"),
                Self::Hidden => serializer.serialize_unit_variant("Visibility", 1u32, "Hidden"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Is this parameter required or optional"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Optional")]
    pub enum Optional {
        Optional,
        Required,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Optional {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Optional {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Optional {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Optional => serializer.serialize_unit_variant("Optional", 0u32, "Optional"),
                Self::Required => serializer.serialize_unit_variant("Optional", 1u32, "Required"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "Service specification payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Specifications of the Log for Azure Monitoring"]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
    #[doc = "Specifications of the Metrics for Azure Monitoring"]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition representing SKU"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the SKU."]
    pub name: String,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "The resource model definition for a ARM tracked top level resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscription trial availability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Trial {
    #[doc = "Trial status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<trial::Status>,
    #[doc = "Number of trial hosts available"]
    #[serde(rename = "availableHosts", default, skip_serializing_if = "Option::is_none")]
    pub available_hosts: Option<i32>,
}
impl Trial {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod trial {
    use super::*;
    #[doc = "Trial status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        TrialAvailable,
        TrialUsed,
        TrialDisabled,
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
                Self::TrialAvailable => serializer.serialize_unit_variant("Status", 0u32, "TrialAvailable"),
                Self::TrialUsed => serializer.serialize_unit_variant("Status", 1u32, "TrialUsed"),
                Self::TrialDisabled => serializer.serialize_unit_variant("Status", 2u32, "TrialDisabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
    #[doc = "Display name of the VM."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Virtual machine managed object reference id"]
    #[serde(rename = "moRefId", default, skip_serializing_if = "Option::is_none")]
    pub mo_ref_id: Option<String>,
    #[doc = "Path to virtual machine's folder starting from datacenter virtual machine folder"]
    #[serde(rename = "folderPath", default, skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<String>,
    #[doc = "Whether VM DRS-driven movement is restricted (enabled) or not (disabled)"]
    #[serde(rename = "restrictMovement", default, skip_serializing_if = "Option::is_none")]
    pub restrict_movement: Option<VirtualMachineRestrictMovementState>,
}
impl VirtualMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Set VM DRS-driven movement to restricted (enabled) or not (disabled)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineRestrictMovement {
    #[doc = "Whether VM DRS-driven movement is restricted (enabled) or not (disabled)"]
    #[serde(rename = "restrictMovement", default, skip_serializing_if = "Option::is_none")]
    pub restrict_movement: Option<VirtualMachineRestrictMovementState>,
}
impl VirtualMachineRestrictMovement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Whether VM DRS-driven movement is restricted (enabled) or not (disabled)"]
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
#[doc = "A list of Virtual Machines"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachinesList {
    #[doc = "The items to be displayed on the page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualMachine>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualMachinesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualMachinesList {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Placement policy affinity type"]
    #[serde(rename = "affinityType")]
    pub affinity_type: AffinityType,
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
    #[doc = "Placement policy affinity type"]
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
#[doc = "NSX DHCP"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkDhcp {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Base class for WorkloadNetworkDhcpServer and WorkloadNetworkDhcpRelay to inherit from"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadNetworkDhcpEntity>,
}
impl WorkloadNetworkDhcp {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for WorkloadNetworkDhcpServer and WorkloadNetworkDhcpRelay to inherit from"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDhcpEntity {
    #[doc = "Type of DHCP: SERVER or RELAY."]
    #[serde(rename = "dhcpType")]
    pub dhcp_type: workload_network_dhcp_entity::DhcpType,
    #[doc = "Display name of the DHCP entity."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "NSX Segments consuming DHCP."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub segments: Vec<String>,
    #[doc = "The provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workload_network_dhcp_entity::ProvisioningState>,
    #[doc = "NSX revision number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
impl WorkloadNetworkDhcpEntity {
    pub fn new(dhcp_type: workload_network_dhcp_entity::DhcpType) -> Self {
        Self {
            dhcp_type,
            display_name: None,
            segments: Vec::new(),
            provisioning_state: None,
            revision: None,
        }
    }
}
pub mod workload_network_dhcp_entity {
    use super::*;
    #[doc = "Type of DHCP: SERVER or RELAY."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DhcpType")]
    pub enum DhcpType {
        #[serde(rename = "SERVER")]
        Server,
        #[serde(rename = "RELAY")]
        Relay,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DhcpType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DhcpType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DhcpType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Server => serializer.serialize_unit_variant("DhcpType", 0u32, "SERVER"),
                Self::Relay => serializer.serialize_unit_variant("DhcpType", 1u32, "RELAY"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Building,
        Deleting,
        Updating,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Building => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Building"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of NSX dhcp entities"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkDhcpList {
    #[doc = "The items on the page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkloadNetworkDhcp>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkDhcpList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkloadNetworkDhcpList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NSX DHCP Relay"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadNetworkDhcpRelay {
    #[serde(flatten)]
    pub workload_network_dhcp_entity: WorkloadNetworkDhcpEntity,
    #[doc = "DHCP Relay Addresses. Max 3."]
    #[serde(rename = "serverAddresses", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "fqdnZones", default, skip_serializing_if = "Vec::is_empty")]
    pub fqdn_zones: Vec<String>,
    #[doc = "DNS Service log level."]
    #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<workload_network_dns_service_properties::LogLevel>,
    #[doc = "DNS Service status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<workload_network_dns_service_properties::Status>,
    #[doc = "The provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workload_network_dns_service_properties::ProvisioningState>,
    #[doc = "NSX revision number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
impl WorkloadNetworkDnsServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workload_network_dns_service_properties {
    use super::*;
    #[doc = "DNS Service log level."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LogLevel")]
    pub enum LogLevel {
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
    impl FromStr for LogLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LogLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LogLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Debug => serializer.serialize_unit_variant("LogLevel", 0u32, "DEBUG"),
                Self::Info => serializer.serialize_unit_variant("LogLevel", 1u32, "INFO"),
                Self::Warning => serializer.serialize_unit_variant("LogLevel", 2u32, "WARNING"),
                Self::Error => serializer.serialize_unit_variant("LogLevel", 3u32, "ERROR"),
                Self::Fatal => serializer.serialize_unit_variant("LogLevel", 4u32, "FATAL"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "DNS Service status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "SUCCESS")]
        Success,
        #[serde(rename = "FAILURE")]
        Failure,
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
                Self::Success => serializer.serialize_unit_variant("Status", 0u32, "SUCCESS"),
                Self::Failure => serializer.serialize_unit_variant("Status", 1u32, "FAILURE"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Building,
        Deleting,
        Updating,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Building => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Building"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of NSX DNS Services"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkDnsServicesList {
    #[doc = "The items on the page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkloadNetworkDnsService>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkDnsServicesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkloadNetworkDnsServicesList {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub domain: Vec<String>,
    #[doc = "DNS Server IP array of the DNS Zone."]
    #[serde(rename = "dnsServerIps", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_server_ips: Vec<String>,
    #[doc = "Source IP of the DNS Zone."]
    #[serde(rename = "sourceIp", default, skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
    #[doc = "Number of DNS Services using the DNS zone."]
    #[serde(rename = "dnsServices", default, skip_serializing_if = "Option::is_none")]
    pub dns_services: Option<i64>,
    #[doc = "The provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workload_network_dns_zone_properties::ProvisioningState>,
    #[doc = "NSX revision number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
impl WorkloadNetworkDnsZoneProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workload_network_dns_zone_properties {
    use super::*;
    #[doc = "The provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Building,
        Deleting,
        Updating,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Building => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Building"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of NSX DNS Zones"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkDnsZonesList {
    #[doc = "The items on the page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkloadNetworkDnsZone>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkDnsZonesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkloadNetworkDnsZonesList {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "A list of NSX Gateways"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkGatewayList {
    #[doc = "The items on the page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkloadNetworkGateway>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkGatewayList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkloadNetworkGatewayList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a NSX Gateway."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkGatewayProperties {
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
#[doc = "A list of NSX Port Mirroring"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkPortMirroringList {
    #[doc = "The items on the page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkloadNetworkPortMirroring>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkPortMirroringList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkloadNetworkPortMirroringList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NSX Port Mirroring Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkPortMirroringProperties {
    #[doc = "Display name of the port mirroring profile."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Direction of port mirroring profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<workload_network_port_mirroring_properties::Direction>,
    #[doc = "Source VM Group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Destination VM Group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[doc = "Port Mirroring Status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<workload_network_port_mirroring_properties::Status>,
    #[doc = "The provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workload_network_port_mirroring_properties::ProvisioningState>,
    #[doc = "NSX revision number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
impl WorkloadNetworkPortMirroringProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workload_network_port_mirroring_properties {
    use super::*;
    #[doc = "Direction of port mirroring profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Direction")]
    pub enum Direction {
        #[serde(rename = "INGRESS")]
        Ingress,
        #[serde(rename = "EGRESS")]
        Egress,
        #[serde(rename = "BIDIRECTIONAL")]
        Bidirectional,
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
                Self::Ingress => serializer.serialize_unit_variant("Direction", 0u32, "INGRESS"),
                Self::Egress => serializer.serialize_unit_variant("Direction", 1u32, "EGRESS"),
                Self::Bidirectional => serializer.serialize_unit_variant("Direction", 2u32, "BIDIRECTIONAL"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Port Mirroring Status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "SUCCESS")]
        Success,
        #[serde(rename = "FAILURE")]
        Failure,
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
                Self::Success => serializer.serialize_unit_variant("Status", 0u32, "SUCCESS"),
                Self::Failure => serializer.serialize_unit_variant("Status", 1u32, "FAILURE"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Building,
        Deleting,
        Updating,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Building => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Building"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
    #[doc = "The provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workload_network_public_ip_properties::ProvisioningState>,
}
impl WorkloadNetworkPublicIpProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workload_network_public_ip_properties {
    use super::*;
    #[doc = "The provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Building,
        Deleting,
        Updating,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Building => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Building"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of NSX Public IP Blocks"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkPublicIPsList {
    #[doc = "The items on the page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkloadNetworkPublicIp>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkPublicIPsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkloadNetworkPublicIPsList {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(rename = "portVif", default, skip_serializing_if = "Vec::is_empty")]
    pub port_vif: Vec<WorkloadNetworkSegmentPortVif>,
    #[doc = "Segment status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<workload_network_segment_properties::Status>,
    #[doc = "The provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workload_network_segment_properties::ProvisioningState>,
    #[doc = "NSX revision number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
impl WorkloadNetworkSegmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workload_network_segment_properties {
    use super::*;
    #[doc = "Segment status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "SUCCESS")]
        Success,
        #[serde(rename = "FAILURE")]
        Failure,
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
                Self::Success => serializer.serialize_unit_variant("Status", 0u32, "SUCCESS"),
                Self::Failure => serializer.serialize_unit_variant("Status", 1u32, "FAILURE"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Building,
        Deleting,
        Updating,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Building => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Building"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Subnet configuration for segment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkSegmentSubnet {
    #[doc = "DHCP Range assigned for subnet."]
    #[serde(rename = "dhcpRanges", default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "A list of NSX Segments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkSegmentsList {
    #[doc = "The items on the page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkloadNetworkSegment>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkSegmentsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkloadNetworkSegmentsList {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<String>,
    #[doc = "VM Group status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<workload_network_vm_group_properties::Status>,
    #[doc = "The provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workload_network_vm_group_properties::ProvisioningState>,
    #[doc = "NSX revision number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
impl WorkloadNetworkVmGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workload_network_vm_group_properties {
    use super::*;
    #[doc = "VM Group status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "SUCCESS")]
        Success,
        #[serde(rename = "FAILURE")]
        Failure,
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
                Self::Success => serializer.serialize_unit_variant("Status", 0u32, "SUCCESS"),
                Self::Failure => serializer.serialize_unit_variant("Status", 1u32, "FAILURE"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Building,
        Deleting,
        Updating,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Building => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Building"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of NSX VM Groups"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkVmGroupsList {
    #[doc = "The items on the page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkloadNetworkVmGroup>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkVmGroupsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkloadNetworkVmGroupsList {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Display name of the VM."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Virtual machine type."]
    #[serde(rename = "vmType", default, skip_serializing_if = "Option::is_none")]
    pub vm_type: Option<workload_network_virtual_machine_properties::VmType>,
}
impl WorkloadNetworkVirtualMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workload_network_virtual_machine_properties {
    use super::*;
    #[doc = "Virtual machine type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VmType")]
    pub enum VmType {
        #[serde(rename = "REGULAR")]
        Regular,
        #[serde(rename = "EDGE")]
        Edge,
        #[serde(rename = "SERVICE")]
        Service,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VmType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VmType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VmType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Regular => serializer.serialize_unit_variant("VmType", 0u32, "REGULAR"),
                Self::Edge => serializer.serialize_unit_variant("VmType", 1u32, "EDGE"),
                Self::Service => serializer.serialize_unit_variant("VmType", 2u32, "SERVICE"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of NSX Virtual Machines"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadNetworkVirtualMachinesList {
    #[doc = "The items on the page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkloadNetworkVirtualMachine>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadNetworkVirtualMachinesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkloadNetworkVirtualMachinesList {
    pub fn new() -> Self {
        Self::default()
    }
}

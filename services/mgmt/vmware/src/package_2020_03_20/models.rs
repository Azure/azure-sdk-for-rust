#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
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
#[doc = "A cluster resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The resource model definition representing SKU"]
    pub sku: Sku,
    #[doc = "The properties of a cluster"]
    pub properties: ClusterProperties,
}
impl Cluster {
    pub fn new(sku: Sku, properties: ClusterProperties) -> Self {
        Self {
            resource: Resource::default(),
            sku,
            properties,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterProperties {
    #[serde(flatten)]
    pub management_cluster: ManagementCluster,
    #[doc = "The state of the cluster provisioning"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ClusterProvisioningState>,
}
impl ClusterProperties {
    pub fn new(management_cluster: ManagementCluster) -> Self {
        Self {
            management_cluster,
            provisioning_state: None,
        }
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
}
impl ClusterUpdateProperties {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The properties of a default cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementCluster {
    #[serde(flatten)]
    pub cluster_update_properties: ClusterUpdateProperties,
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
impl ManagementCluster {
    pub fn new() -> Self {
        Self {
            cluster_update_properties: ClusterUpdateProperties::default(),
            provisioning_state: None,
            cluster_id: None,
            hosts: Vec::new(),
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
#[doc = "A private cloud resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloud {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The resource model definition representing SKU"]
    pub sku: Sku,
    #[doc = "The properties of a private cloud resource"]
    pub properties: PrivateCloudProperties,
}
impl PrivateCloud {
    pub fn new(sku: Sku, properties: PrivateCloudProperties) -> Self {
        Self {
            tracked_resource: TrackedResource::default(),
            sku,
            properties,
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
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
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
    #[doc = "The properties of a default cluster"]
    #[serde(rename = "managementCluster", default, skip_serializing_if = "Option::is_none")]
    pub management_cluster: Option<ManagementCluster>,
    #[doc = "Connectivity to internet is enabled or disabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internet: Option<private_cloud_update_properties::Internet>,
    #[doc = "vCenter Single Sign On Identity Sources"]
    #[serde(rename = "identitySources", default, skip_serializing_if = "Vec::is_empty")]
    pub identity_sources: Vec<IdentitySource>,
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
    pub tags: Option<serde_json::Value>,
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

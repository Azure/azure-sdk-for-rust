#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCapacity {
    #[doc = "Scale type."]
    #[serde(rename = "scaleType")]
    pub scale_type: azure_capacity::ScaleType,
    #[doc = "Minimum allowed capacity."]
    pub minimum: i64,
    #[doc = "Maximum allowed capacity."]
    pub maximum: i64,
    #[doc = "The default capacity that would be used."]
    pub default: i64,
}
impl AzureCapacity {
    pub fn new(scale_type: azure_capacity::ScaleType, minimum: i64, maximum: i64, default: i64) -> Self {
        Self {
            scale_type,
            minimum,
            maximum,
            default,
        }
    }
}
pub mod azure_capacity {
    use super::*;
    #[doc = "Scale type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScaleType")]
    pub enum ScaleType {
        #[serde(rename = "automatic")]
        Automatic,
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "none")]
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScaleType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScaleType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScaleType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Automatic => serializer.serialize_unit_variant("ScaleType", 0u32, "automatic"),
                Self::Manual => serializer.serialize_unit_variant("ScaleType", 1u32, "manual"),
                Self::None => serializer.serialize_unit_variant("ScaleType", 2u32, "none"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceSku {
    #[doc = "Resource Namespace and Type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<AzureSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<AzureCapacity>,
}
impl AzureResourceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSku {
    #[doc = "SKU name."]
    pub name: azure_sku::Name,
    #[doc = "SKU capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[doc = "SKU tier."]
    pub tier: azure_sku::Tier,
}
impl AzureSku {
    pub fn new(name: azure_sku::Name, tier: azure_sku::Tier) -> Self {
        Self {
            name,
            capacity: None,
            tier,
        }
    }
}
pub mod azure_sku {
    use super::*;
    #[doc = "SKU name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        #[serde(rename = "KC8")]
        Kc8,
        #[serde(rename = "KC16")]
        Kc16,
        #[serde(rename = "KS8")]
        Ks8,
        #[serde(rename = "KS16")]
        Ks16,
        #[serde(rename = "D13_v2")]
        D13V2,
        #[serde(rename = "D14_v2")]
        D14V2,
        L8,
        L16,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Kc8 => serializer.serialize_unit_variant("Name", 0u32, "KC8"),
                Self::Kc16 => serializer.serialize_unit_variant("Name", 1u32, "KC16"),
                Self::Ks8 => serializer.serialize_unit_variant("Name", 2u32, "KS8"),
                Self::Ks16 => serializer.serialize_unit_variant("Name", 3u32, "KS16"),
                Self::D13V2 => serializer.serialize_unit_variant("Name", 4u32, "D13_v2"),
                Self::D14V2 => serializer.serialize_unit_variant("Name", 5u32, "D14_v2"),
                Self::L8 => serializer.serialize_unit_variant("Name", 6u32, "L8"),
                Self::L16 => serializer.serialize_unit_variant("Name", 7u32, "L16"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "SKU tier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Standard,
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
                Self::Standard => serializer.serialize_unit_variant("Tier", 0u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameResult {
    #[doc = "Specifies a Boolean value that indicates if the name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The name that was checked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Message indicating an unavailable name due to a conflict, or a description of the naming rules that are violated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from Kusto."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from Kusto."]
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
#[doc = "An error response from Kusto."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for displaying in a user interface."]
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
#[doc = "Class representing a Kusto cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "An ETag of the resource created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    pub sku: AzureSku,
    #[doc = "Class representing the Kusto cluster properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
impl Cluster {
    pub fn new(tracked_resource: TrackedResource, sku: AzureSku) -> Self {
        Self {
            tracked_resource,
            etag: None,
            sku,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterCheckNameRequest {
    #[doc = "Cluster name."]
    pub name: String,
    #[doc = "The type of resource, Microsoft.Kusto/clusters."]
    #[serde(rename = "type")]
    pub type_: cluster_check_name_request::Type,
}
impl ClusterCheckNameRequest {
    pub fn new(name: String, type_: cluster_check_name_request::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod cluster_check_name_request {
    use super::*;
    #[doc = "The type of resource, Microsoft.Kusto/clusters."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Kusto/clusters")]
        MicrosoftKustoClusters,
    }
}
#[doc = "The list Kusto clusters operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterListResult {
    #[doc = "The list of Kusto clusters."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cluster>,
}
impl azure_core::Continuable for ClusterListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ClusterListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the Kusto cluster properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterProperties {
    #[doc = "The state of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<cluster_properties::State>,
    #[doc = "The provisioned state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<cluster_properties::ProvisioningState>,
    #[doc = "The cluster URI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "The cluster data ingestion URI."]
    #[serde(rename = "dataIngestionUri", default, skip_serializing_if = "Option::is_none")]
    pub data_ingestion_uri: Option<String>,
    #[doc = "The cluster's external tenants."]
    #[serde(rename = "trustedExternalTenants", default, skip_serializing_if = "Vec::is_empty")]
    pub trusted_external_tenants: Vec<TrustedExternalTenant>,
}
impl ClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cluster_properties {
    use super::*;
    #[doc = "The state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Creating,
        Unavailable,
        Running,
        Deleting,
        Deleted,
        Stopping,
        Stopped,
        Starting,
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
                Self::Creating => serializer.serialize_unit_variant("State", 0u32, "Creating"),
                Self::Unavailable => serializer.serialize_unit_variant("State", 1u32, "Unavailable"),
                Self::Running => serializer.serialize_unit_variant("State", 2u32, "Running"),
                Self::Deleting => serializer.serialize_unit_variant("State", 3u32, "Deleting"),
                Self::Deleted => serializer.serialize_unit_variant("State", 4u32, "Deleted"),
                Self::Stopping => serializer.serialize_unit_variant("State", 5u32, "Stopping"),
                Self::Stopped => serializer.serialize_unit_variant("State", 6u32, "Stopped"),
                Self::Starting => serializer.serialize_unit_variant("State", 7u32, "Starting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioned state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Running,
        Creating,
        Deleting,
        Succeeded,
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
                Self::Running => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Running"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Class representing an update to a Kusto cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpdate {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "An ETag of the resource updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<AzureSku>,
    #[doc = "Class representing the Kusto cluster properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
impl ClusterUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing a Kusto database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Database {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "An ETag of the resource created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Class representing the Kusto database properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseProperties>,
}
impl Database {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseCheckNameRequest {
    #[doc = "Database name."]
    pub name: String,
    #[doc = "The type of resource, Microsoft.Kusto/clusters/databases."]
    #[serde(rename = "type")]
    pub type_: database_check_name_request::Type,
}
impl DatabaseCheckNameRequest {
    pub fn new(name: String, type_: database_check_name_request::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod database_check_name_request {
    use super::*;
    #[doc = "The type of resource, Microsoft.Kusto/clusters/databases."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Kusto/clusters/databases")]
        MicrosoftKustoClustersDatabases,
    }
}
#[doc = "The list Kusto databases operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseListResult {
    #[doc = "The list of Kusto databases."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Database>,
}
impl azure_core::Continuable for DatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DatabaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabasePrincipal {
    #[doc = "Database principal role."]
    pub role: database_principal::Role,
    #[doc = "Database principal name."]
    pub name: String,
    #[doc = "Database principal type."]
    #[serde(rename = "type")]
    pub type_: database_principal::Type,
    #[doc = "Database principal fully qualified name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqn: Option<String>,
    #[doc = "Database principal email if exists."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Application id - relevant only for application principal type."]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}
impl DatabasePrincipal {
    pub fn new(role: database_principal::Role, name: String, type_: database_principal::Type) -> Self {
        Self {
            role,
            name,
            type_,
            fqn: None,
            email: None,
            app_id: None,
        }
    }
}
pub mod database_principal {
    use super::*;
    #[doc = "Database principal role."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Role")]
    pub enum Role {
        Admin,
        Ingestor,
        Monitor,
        User,
        UnrestrictedViewers,
        Viewer,
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
                Self::Admin => serializer.serialize_unit_variant("Role", 0u32, "Admin"),
                Self::Ingestor => serializer.serialize_unit_variant("Role", 1u32, "Ingestor"),
                Self::Monitor => serializer.serialize_unit_variant("Role", 2u32, "Monitor"),
                Self::User => serializer.serialize_unit_variant("Role", 3u32, "User"),
                Self::UnrestrictedViewers => serializer.serialize_unit_variant("Role", 4u32, "UnrestrictedViewers"),
                Self::Viewer => serializer.serialize_unit_variant("Role", 5u32, "Viewer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Database principal type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        App,
        Group,
        User,
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
                Self::App => serializer.serialize_unit_variant("Type", 0u32, "App"),
                Self::Group => serializer.serialize_unit_variant("Type", 1u32, "Group"),
                Self::User => serializer.serialize_unit_variant("Type", 2u32, "User"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The list Kusto database principals operation request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabasePrincipalListRequest {
    #[doc = "The list of Kusto database principals."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabasePrincipal>,
}
impl DatabasePrincipalListRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list Kusto database principals operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabasePrincipalListResult {
    #[doc = "The list of Kusto database principals."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabasePrincipal>,
}
impl azure_core::Continuable for DatabasePrincipalListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DatabasePrincipalListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the Kusto database properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseProperties {
    #[doc = "The provisioned state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<database_properties::ProvisioningState>,
    #[doc = "The number of days data should be kept before it stops being accessible to queries."]
    #[serde(rename = "softDeletePeriodInDays")]
    pub soft_delete_period_in_days: i64,
    #[doc = "The number of days of data that should be kept in cache for fast queries."]
    #[serde(rename = "hotCachePeriodInDays", default, skip_serializing_if = "Option::is_none")]
    pub hot_cache_period_in_days: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<DatabaseStatistics>,
}
impl DatabaseProperties {
    pub fn new(soft_delete_period_in_days: i64) -> Self {
        Self {
            provisioning_state: None,
            soft_delete_period_in_days,
            hot_cache_period_in_days: None,
            statistics: None,
        }
    }
}
pub mod database_properties {
    use super::*;
    #[doc = "The provisioned state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Running,
        Creating,
        Deleting,
        Succeeded,
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
                Self::Running => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Running"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseStatistics {
    #[doc = "The database size - the total size of compressed data and index in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
}
impl DatabaseStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing an update to a Kusto database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseUpdate {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "An ETag of the resource updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Class representing the Kusto database properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseProperties>,
}
impl DatabaseUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing an event hub connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Class representing the Kusto event hub connection properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubConnectionProperties>,
}
impl EventHubConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list Kusto event hub connections operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubConnectionListResult {
    #[doc = "The list of Kusto event hub connections."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EventHubConnection>,
}
impl azure_core::Continuable for EventHubConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl EventHubConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the Kusto event hub connection properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubConnectionProperties {
    #[doc = "The resource ID of the event hub to be used to create a data connection."]
    #[serde(rename = "eventHubResourceId")]
    pub event_hub_resource_id: String,
    #[doc = "The event hub consumer group."]
    #[serde(rename = "consumerGroup")]
    pub consumer_group: String,
    #[doc = "The table where the data should be ingested. Optionally the table information can be added to each message."]
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[doc = "The mapping rule to be used to ingest the data. Optionally the mapping information can be added to each message."]
    #[serde(rename = "mappingRuleName", default, skip_serializing_if = "Option::is_none")]
    pub mapping_rule_name: Option<String>,
    #[doc = "The data format of the message. Optionally the data format can be added to each message."]
    #[serde(rename = "dataFormat", default, skip_serializing_if = "Option::is_none")]
    pub data_format: Option<event_hub_connection_properties::DataFormat>,
}
impl EventHubConnectionProperties {
    pub fn new(event_hub_resource_id: String, consumer_group: String) -> Self {
        Self {
            event_hub_resource_id,
            consumer_group,
            table_name: None,
            mapping_rule_name: None,
            data_format: None,
        }
    }
}
pub mod event_hub_connection_properties {
    use super::*;
    #[doc = "The data format of the message. Optionally the data format can be added to each message."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataFormat")]
    pub enum DataFormat {
        #[serde(rename = "MULTIJSON")]
        Multijson,
        #[serde(rename = "JSON")]
        Json,
        #[serde(rename = "CSV")]
        Csv,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataFormat {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataFormat {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataFormat {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Multijson => serializer.serialize_unit_variant("DataFormat", 0u32, "MULTIJSON"),
                Self::Json => serializer.serialize_unit_variant("DataFormat", 1u32, "JSON"),
                Self::Csv => serializer.serialize_unit_variant("DataFormat", 2u32, "CSV"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Class representing an update to event hub connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubConnectionUpdate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Class representing the Kusto event hub connection properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubConnectionProperties>,
}
impl EventHubConnectionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing an event hub connection validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubConnectionValidation {
    #[doc = "The name of the event hub connection."]
    #[serde(rename = "eventhubConnectionName", default, skip_serializing_if = "Option::is_none")]
    pub eventhub_connection_name: Option<String>,
    #[doc = "Class representing the Kusto event hub connection properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubConnectionProperties>,
}
impl EventHubConnectionValidation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list Kusto event hub connection validation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubConnectionValidationListResult {
    #[doc = "The list of Kusto event hub connection validation errors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EventHubConnectionValidationResult>,
}
impl EventHubConnectionValidationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubConnectionValidationResult {
    #[doc = "A message which indicates a problem in event hub connection validation."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl EventHubConnectionValidationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of available SKUs for an existing Kusto Cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListResourceSkusResult {
    #[doc = "The collection of available SKUs for an existing resource."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AzureResourceSku>,
}
impl azure_core::Continuable for ListResourceSkusResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ListResourceSkusResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of available SKUs for a new Kusto Cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListSkusResult {
    #[doc = "The collection of available SKUs for new resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AzureSku>,
}
impl azure_core::Continuable for ListSkusResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ListSkusResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "This is of the format {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "For example: read, write, delete."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
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
}
impl Resource {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrustedExternalTenant {
    #[doc = "GUID representing an external tenant."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl TrustedExternalTenant {
    pub fn new() -> Self {
        Self::default()
    }
}

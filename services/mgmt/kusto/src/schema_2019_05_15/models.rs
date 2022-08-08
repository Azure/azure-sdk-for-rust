#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Azure capacity definition."]
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
#[doc = "Azure resource SKU definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceSku {
    #[doc = "Resource Namespace and Type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Azure SKU definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<AzureSku>,
    #[doc = "Azure capacity definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<AzureCapacity>,
}
impl AzureResourceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure SKU definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSku {
    #[doc = "SKU name."]
    pub name: azure_sku::Name,
    #[doc = "The number of instances of the cluster."]
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
        #[serde(rename = "Standard_DS13_v2+1TB_PS")]
        StandardDs13V21tbPs,
        #[serde(rename = "Standard_DS13_v2+2TB_PS")]
        StandardDs13V22tbPs,
        #[serde(rename = "Standard_DS14_v2+3TB_PS")]
        StandardDs14V23tbPs,
        #[serde(rename = "Standard_DS14_v2+4TB_PS")]
        StandardDs14V24tbPs,
        #[serde(rename = "Standard_D13_v2")]
        StandardD13V2,
        #[serde(rename = "Standard_D14_v2")]
        StandardD14V2,
        #[serde(rename = "Standard_L8s")]
        StandardL8s,
        #[serde(rename = "Standard_L16s")]
        StandardL16s,
        #[serde(rename = "Standard_D11_v2")]
        StandardD11V2,
        #[serde(rename = "Standard_D12_v2")]
        StandardD12V2,
        #[serde(rename = "Standard_L4s")]
        StandardL4s,
        #[serde(rename = "Dev(No SLA)_Standard_D11_v2")]
        DevNoSlaStandardD11V2,
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
                Self::StandardDs13V21tbPs => serializer.serialize_unit_variant("Name", 0u32, "Standard_DS13_v2+1TB_PS"),
                Self::StandardDs13V22tbPs => serializer.serialize_unit_variant("Name", 1u32, "Standard_DS13_v2+2TB_PS"),
                Self::StandardDs14V23tbPs => serializer.serialize_unit_variant("Name", 2u32, "Standard_DS14_v2+3TB_PS"),
                Self::StandardDs14V24tbPs => serializer.serialize_unit_variant("Name", 3u32, "Standard_DS14_v2+4TB_PS"),
                Self::StandardD13V2 => serializer.serialize_unit_variant("Name", 4u32, "Standard_D13_v2"),
                Self::StandardD14V2 => serializer.serialize_unit_variant("Name", 5u32, "Standard_D14_v2"),
                Self::StandardL8s => serializer.serialize_unit_variant("Name", 6u32, "Standard_L8s"),
                Self::StandardL16s => serializer.serialize_unit_variant("Name", 7u32, "Standard_L16s"),
                Self::StandardD11V2 => serializer.serialize_unit_variant("Name", 8u32, "Standard_D11_v2"),
                Self::StandardD12V2 => serializer.serialize_unit_variant("Name", 9u32, "Standard_D12_v2"),
                Self::StandardL4s => serializer.serialize_unit_variant("Name", 10u32, "Standard_L4s"),
                Self::DevNoSlaStandardD11V2 => serializer.serialize_unit_variant("Name", 11u32, "Dev(No SLA)_Standard_D11_v2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "SKU tier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Basic,
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
                Self::Basic => serializer.serialize_unit_variant("Tier", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Tier", 1u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The result returned from a check name availability request."]
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
    #[doc = "Message providing the reason why the given name is invalid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_result::Reason>,
}
impl CheckNameResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_result {
    use super::*;
    #[doc = "Message providing the reason why the given name is invalid."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        Invalid,
        AlreadyExists,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Reason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Reason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Reason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("Reason", 0u32, "Invalid"),
                Self::AlreadyExists => serializer.serialize_unit_variant("Reason", 1u32, "AlreadyExists"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
    #[doc = "Azure SKU definition."]
    pub sku: AzureSku,
    #[doc = "An array represents the availability zones of the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<Zones>,
    #[doc = "Class representing the Kusto cluster properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
impl Cluster {
    pub fn new(tracked_resource: TrackedResource, sku: AzureSku) -> Self {
        Self {
            tracked_resource,
            sku,
            zones: None,
            properties: None,
        }
    }
}
#[doc = "The result returned from a cluster check name availability request."]
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
    #[doc = "A class that contains the optimized auto scale definition."]
    #[serde(rename = "optimizedAutoscale", default, skip_serializing_if = "Option::is_none")]
    pub optimized_autoscale: Option<OptimizedAutoscale>,
    #[doc = "A boolean value that indicates if the cluster's disks are encrypted."]
    #[serde(rename = "enableDiskEncryption", default, skip_serializing_if = "Option::is_none")]
    pub enable_disk_encryption: Option<bool>,
    #[doc = "A boolean value that indicates if the streaming ingest is enabled."]
    #[serde(rename = "enableStreamingIngest", default, skip_serializing_if = "Option::is_none")]
    pub enable_streaming_ingest: Option<bool>,
    #[doc = "A class that contains virtual network definition."]
    #[serde(rename = "virtualNetworkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_configuration: Option<VirtualNetworkConfiguration>,
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
        Updating,
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
                Self::Updating => serializer.serialize_unit_variant("State", 8u32, "Updating"),
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
        Moving,
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
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Moving"),
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
    #[doc = "Azure SKU definition."]
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
#[doc = "Class representing an data connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Kind of the endpoint for the data connection"]
    pub kind: data_connection::Kind,
}
impl DataConnection {
    pub fn new(kind: data_connection::Kind) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            location: None,
            kind,
        }
    }
}
pub mod data_connection {
    use super::*;
    #[doc = "Kind of the endpoint for the data connection"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        EventHub,
        EventGrid,
        IotHub,
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
                Self::EventHub => serializer.serialize_unit_variant("Kind", 0u32, "EventHub"),
                Self::EventGrid => serializer.serialize_unit_variant("Kind", 1u32, "EventGrid"),
                Self::IotHub => serializer.serialize_unit_variant("Kind", 2u32, "IotHub"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The result returned from a data connections check name availability request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectionCheckNameRequest {
    #[doc = "Data Connection name."]
    pub name: String,
    #[doc = "The type of resource, Microsoft.Kusto/clusters/databases/dataConnections."]
    #[serde(rename = "type")]
    pub type_: data_connection_check_name_request::Type,
}
impl DataConnectionCheckNameRequest {
    pub fn new(name: String, type_: data_connection_check_name_request::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod data_connection_check_name_request {
    use super::*;
    #[doc = "The type of resource, Microsoft.Kusto/clusters/databases/dataConnections."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Kusto/clusters/databases/dataConnections")]
        MicrosoftKustoClustersDatabasesDataConnections,
    }
}
#[doc = "The list Kusto data connections operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataConnectionListResult {
    #[doc = "The list of Kusto data connections."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataConnection>,
}
impl azure_core::Continuable for DataConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DataConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing an data connection validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataConnectionValidation {
    #[doc = "The name of the data connection."]
    #[serde(rename = "dataConnectionName", default, skip_serializing_if = "Option::is_none")]
    pub data_connection_name: Option<String>,
    #[doc = "Class representing an data connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataConnection>,
}
impl DataConnectionValidation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list Kusto data connection validation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataConnectionValidationListResult {
    #[doc = "The list of Kusto data connection validation errors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataConnectionValidationResult>,
}
impl DataConnectionValidationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result returned from a data connection validation request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataConnectionValidationResult {
    #[doc = "A message which indicates a problem in data connection validation."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl DataConnectionValidationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
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
    #[serde(rename = "TSV")]
    Tsv,
    #[serde(rename = "SCSV")]
    Scsv,
    #[serde(rename = "SOHSV")]
    Sohsv,
    #[serde(rename = "PSV")]
    Psv,
    #[serde(rename = "TXT")]
    Txt,
    #[serde(rename = "RAW")]
    Raw,
    #[serde(rename = "SINGLEJSON")]
    Singlejson,
    #[serde(rename = "AVRO")]
    Avro,
    #[serde(rename = "TSVE")]
    Tsve,
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
            Self::Tsv => serializer.serialize_unit_variant("DataFormat", 3u32, "TSV"),
            Self::Scsv => serializer.serialize_unit_variant("DataFormat", 4u32, "SCSV"),
            Self::Sohsv => serializer.serialize_unit_variant("DataFormat", 5u32, "SOHSV"),
            Self::Psv => serializer.serialize_unit_variant("DataFormat", 6u32, "PSV"),
            Self::Txt => serializer.serialize_unit_variant("DataFormat", 7u32, "TXT"),
            Self::Raw => serializer.serialize_unit_variant("DataFormat", 8u32, "RAW"),
            Self::Singlejson => serializer.serialize_unit_variant("DataFormat", 9u32, "SINGLEJSON"),
            Self::Avro => serializer.serialize_unit_variant("DataFormat", 10u32, "AVRO"),
            Self::Tsve => serializer.serialize_unit_variant("DataFormat", 11u32, "TSVE"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Class representing a Kusto database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Database {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Class representing the Kusto database properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseProperties>,
}
impl Database {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result returned from a database check name availability request."]
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
#[doc = "A class representing database principal entity."]
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
    #[doc = "The tenant name of the principal"]
    #[serde(rename = "tenantName", default, skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
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
            tenant_name: None,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseProperties {
    #[doc = "The provisioned state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<database_properties::ProvisioningState>,
    #[doc = "The time the data should be kept before it stops being accessible to queries in TimeSpan."]
    #[serde(rename = "softDeletePeriod", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_period: Option<String>,
    #[doc = "The time the data should be kept in cache for fast queries in TimeSpan."]
    #[serde(rename = "hotCachePeriod", default, skip_serializing_if = "Option::is_none")]
    pub hot_cache_period: Option<String>,
    #[doc = "A class that contains database statistics information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<DatabaseStatistics>,
}
impl DatabaseProperties {
    pub fn new() -> Self {
        Self::default()
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
        Moving,
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
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Moving"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A class that contains database statistics information."]
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
    #[doc = "Class representing the Kusto database properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseProperties>,
}
impl DatabaseUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the Kusto event grid connection properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventGridConnectionProperties {
    #[doc = "The resource ID of the storage account where the data resides."]
    #[serde(rename = "storageAccountResourceId")]
    pub storage_account_resource_id: String,
    #[doc = "The resource ID where the event grid is configured to send events."]
    #[serde(rename = "eventHubResourceId")]
    pub event_hub_resource_id: String,
    #[doc = "The event hub consumer group."]
    #[serde(rename = "consumerGroup")]
    pub consumer_group: String,
    #[doc = "The table where the data should be ingested. Optionally the table information can be added to each message."]
    #[serde(rename = "tableName")]
    pub table_name: String,
    #[doc = "The mapping rule to be used to ingest the data. Optionally the mapping information can be added to each message."]
    #[serde(rename = "mappingRuleName", default, skip_serializing_if = "Option::is_none")]
    pub mapping_rule_name: Option<String>,
    #[doc = "The data format of the message. Optionally the data format can be added to each message."]
    #[serde(rename = "dataFormat")]
    pub data_format: DataFormat,
}
impl EventGridConnectionProperties {
    pub fn new(
        storage_account_resource_id: String,
        event_hub_resource_id: String,
        consumer_group: String,
        table_name: String,
        data_format: DataFormat,
    ) -> Self {
        Self {
            storage_account_resource_id,
            event_hub_resource_id,
            consumer_group,
            table_name,
            mapping_rule_name: None,
            data_format,
        }
    }
}
#[doc = "Class representing an Event Grid data connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventGridDataConnection {
    #[serde(flatten)]
    pub data_connection: DataConnection,
    #[doc = "Class representing the Kusto event grid connection properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventGridConnectionProperties>,
}
impl EventGridDataConnection {
    pub fn new(data_connection: DataConnection) -> Self {
        Self {
            data_connection,
            properties: None,
        }
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
    pub data_format: Option<DataFormat>,
    #[doc = "System properties of the event hub"]
    #[serde(rename = "eventSystemProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub event_system_properties: Vec<String>,
}
impl EventHubConnectionProperties {
    pub fn new(event_hub_resource_id: String, consumer_group: String) -> Self {
        Self {
            event_hub_resource_id,
            consumer_group,
            table_name: None,
            mapping_rule_name: None,
            data_format: None,
            event_system_properties: Vec::new(),
        }
    }
}
#[doc = "Class representing an event hub data connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubDataConnection {
    #[serde(flatten)]
    pub data_connection: DataConnection,
    #[doc = "Class representing the Kusto event hub connection properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubConnectionProperties>,
}
impl EventHubDataConnection {
    pub fn new(data_connection: DataConnection) -> Self {
        Self {
            data_connection,
            properties: None,
        }
    }
}
#[doc = "Class representing the Kusto iot hub connection properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubConnectionProperties {
    #[doc = "The resource ID of the Iot hub to be used to create a data connection."]
    #[serde(rename = "iotHubResourceId")]
    pub iot_hub_resource_id: String,
    #[doc = "The iot hub consumer group."]
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
    pub data_format: Option<DataFormat>,
    #[doc = "System properties of the iot hub"]
    #[serde(rename = "eventSystemProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub event_system_properties: Vec<String>,
    #[doc = "The name of the share access policy name"]
    #[serde(rename = "sharedAccessPolicyName")]
    pub shared_access_policy_name: String,
}
impl IotHubConnectionProperties {
    pub fn new(iot_hub_resource_id: String, consumer_group: String, shared_access_policy_name: String) -> Self {
        Self {
            iot_hub_resource_id,
            consumer_group,
            table_name: None,
            mapping_rule_name: None,
            data_format: None,
            event_system_properties: Vec::new(),
            shared_access_policy_name,
        }
    }
}
#[doc = "Class representing an iot hub data connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubDataConnection {
    #[serde(flatten)]
    pub data_connection: DataConnection,
    #[doc = "Class representing the Kusto iot hub connection properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IotHubConnectionProperties>,
}
impl IotHubDataConnection {
    pub fn new(data_connection: DataConnection) -> Self {
        Self {
            data_connection,
            properties: None,
        }
    }
}
#[doc = "List of available SKUs for a Kusto Cluster."]
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
#[doc = "A class that contains the optimized auto scale definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OptimizedAutoscale {
    #[doc = "The version of the template defined, for instance 1."]
    pub version: i64,
    #[doc = "A boolean value that indicate if the optimized autoscale feature is enabled or not."]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[doc = "Minimum allowed instances count."]
    pub minimum: i64,
    #[doc = "Maximum allowed instances count."]
    pub maximum: i64,
}
impl OptimizedAutoscale {
    pub fn new(version: i64, is_enabled: bool, minimum: i64, maximum: i64) -> Self {
        Self {
            version,
            is_enabled,
            minimum,
            maximum,
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
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Kusto SKU description of given resource type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuDescription {
    #[doc = "The resource type"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The name of the SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The tier of the SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The set of locations that the SKU is available"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "Locations and zones"]
    #[serde(rename = "locationInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub location_info: Vec<SkuLocationInfoItem>,
    #[doc = "The restrictions because of which SKU cannot be used"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<serde_json::Value>,
}
impl SkuDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of the EngagementFabric SKU descriptions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuDescriptionList {
    #[doc = "SKU descriptions"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuDescription>,
}
impl azure_core::Continuable for SkuDescriptionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SkuDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The locations and zones info for SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuLocationInfoItem {
    #[doc = "The available location of the SKU."]
    pub location: String,
    #[doc = "The available zone of the SKU."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl SkuLocationInfoItem {
    pub fn new(location: String) -> Self {
        Self {
            location,
            zones: Vec::new(),
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
#[doc = "Represents a tenant ID that is trusted by the cluster."]
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
#[doc = "A class that contains virtual network definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkConfiguration {
    #[doc = "The subnet resource id."]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[doc = "Engine service's public IP address resource id."]
    #[serde(rename = "enginePublicIpId")]
    pub engine_public_ip_id: String,
    #[doc = "Data management's service public IP address resource id."]
    #[serde(rename = "dataManagementPublicIpId")]
    pub data_management_public_ip_id: String,
}
impl VirtualNetworkConfiguration {
    pub fn new(subnet_id: String, engine_public_ip_id: String, data_management_public_ip_id: String) -> Self {
        Self {
            subnet_id,
            engine_public_ip_id,
            data_management_public_ip_id,
        }
    }
}
pub type Zones = Vec<String>;

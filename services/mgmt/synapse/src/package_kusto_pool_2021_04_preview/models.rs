#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Azure SKU definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSku {
    #[doc = "SKU name."]
    pub name: azure_sku::Name,
    #[doc = "The number of instances of the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
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
        #[serde(rename = "Standard_L8s_v2")]
        StandardL8sV2,
        #[serde(rename = "Standard_L16s_v2")]
        StandardL16sV2,
        #[serde(rename = "Standard_D11_v2")]
        StandardD11V2,
        #[serde(rename = "Standard_D12_v2")]
        StandardD12V2,
        #[serde(rename = "Standard_L4s")]
        StandardL4s,
        #[serde(rename = "Dev(No SLA)_Standard_D11_v2")]
        DevNoSlaStandardD11V2,
        #[serde(rename = "Standard_E64i_v3")]
        StandardE64iV3,
        #[serde(rename = "Standard_E80ids_v4")]
        StandardE80idsV4,
        #[serde(rename = "Standard_E2a_v4")]
        StandardE2aV4,
        #[serde(rename = "Standard_E4a_v4")]
        StandardE4aV4,
        #[serde(rename = "Standard_E8a_v4")]
        StandardE8aV4,
        #[serde(rename = "Standard_E16a_v4")]
        StandardE16aV4,
        #[serde(rename = "Standard_E8as_v4+1TB_PS")]
        StandardE8asV41tbPs,
        #[serde(rename = "Standard_E8as_v4+2TB_PS")]
        StandardE8asV42tbPs,
        #[serde(rename = "Standard_E16as_v4+3TB_PS")]
        StandardE16asV43tbPs,
        #[serde(rename = "Standard_E16as_v4+4TB_PS")]
        StandardE16asV44tbPs,
        #[serde(rename = "Dev(No SLA)_Standard_E2a_v4")]
        DevNoSlaStandardE2aV4,
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
                Self::StandardL8sV2 => serializer.serialize_unit_variant("Name", 8u32, "Standard_L8s_v2"),
                Self::StandardL16sV2 => serializer.serialize_unit_variant("Name", 9u32, "Standard_L16s_v2"),
                Self::StandardD11V2 => serializer.serialize_unit_variant("Name", 10u32, "Standard_D11_v2"),
                Self::StandardD12V2 => serializer.serialize_unit_variant("Name", 11u32, "Standard_D12_v2"),
                Self::StandardL4s => serializer.serialize_unit_variant("Name", 12u32, "Standard_L4s"),
                Self::DevNoSlaStandardD11V2 => serializer.serialize_unit_variant("Name", 13u32, "Dev(No SLA)_Standard_D11_v2"),
                Self::StandardE64iV3 => serializer.serialize_unit_variant("Name", 14u32, "Standard_E64i_v3"),
                Self::StandardE80idsV4 => serializer.serialize_unit_variant("Name", 15u32, "Standard_E80ids_v4"),
                Self::StandardE2aV4 => serializer.serialize_unit_variant("Name", 16u32, "Standard_E2a_v4"),
                Self::StandardE4aV4 => serializer.serialize_unit_variant("Name", 17u32, "Standard_E4a_v4"),
                Self::StandardE8aV4 => serializer.serialize_unit_variant("Name", 18u32, "Standard_E8a_v4"),
                Self::StandardE16aV4 => serializer.serialize_unit_variant("Name", 19u32, "Standard_E16a_v4"),
                Self::StandardE8asV41tbPs => serializer.serialize_unit_variant("Name", 20u32, "Standard_E8as_v4+1TB_PS"),
                Self::StandardE8asV42tbPs => serializer.serialize_unit_variant("Name", 21u32, "Standard_E8as_v4+2TB_PS"),
                Self::StandardE16asV43tbPs => serializer.serialize_unit_variant("Name", 22u32, "Standard_E16as_v4+3TB_PS"),
                Self::StandardE16asV44tbPs => serializer.serialize_unit_variant("Name", 23u32, "Standard_E16as_v4+4TB_PS"),
                Self::DevNoSlaStandardE2aV4 => serializer.serialize_unit_variant("Name", 24u32, "Dev(No SLA)_Standard_E2a_v4"),
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
#[doc = "The name of blob storage event type to process."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BlobStorageEventType")]
pub enum BlobStorageEventType {
    #[serde(rename = "Microsoft.Storage.BlobCreated")]
    MicrosoftStorageBlobCreated,
    #[serde(rename = "Microsoft.Storage.BlobRenamed")]
    MicrosoftStorageBlobRenamed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BlobStorageEventType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BlobStorageEventType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BlobStorageEventType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::MicrosoftStorageBlobCreated => {
                serializer.serialize_unit_variant("BlobStorageEventType", 0u32, "Microsoft.Storage.BlobCreated")
            }
            Self::MicrosoftStorageBlobRenamed => {
                serializer.serialize_unit_variant("BlobStorageEventType", 1u32, "Microsoft.Storage.BlobRenamed")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "Class representing a cluster principal assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterPrincipalAssignment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "A class representing cluster principal property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterPrincipalProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ClusterPrincipalAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list Kusto cluster principal assignments operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterPrincipalAssignmentListResult {
    #[doc = "The list of Kusto cluster principal assignments."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ClusterPrincipalAssignment>,
}
impl azure_core::Continuable for ClusterPrincipalAssignmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ClusterPrincipalAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class representing cluster principal property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterPrincipalProperties {
    #[doc = "The principal ID assigned to the cluster principal. It can be a user email, application ID, or security group name."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "Cluster principal role."]
    pub role: cluster_principal_properties::Role,
    #[doc = "The tenant id of the principal"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Principal type."]
    #[serde(rename = "principalType")]
    pub principal_type: cluster_principal_properties::PrincipalType,
    #[doc = "The tenant name of the principal"]
    #[serde(rename = "tenantName", default, skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
    #[doc = "The principal name"]
    #[serde(rename = "principalName", default, skip_serializing_if = "Option::is_none")]
    pub principal_name: Option<String>,
    #[doc = "The provisioned state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ResourceProvisioningState>,
}
impl ClusterPrincipalProperties {
    pub fn new(
        principal_id: String,
        role: cluster_principal_properties::Role,
        principal_type: cluster_principal_properties::PrincipalType,
    ) -> Self {
        Self {
            principal_id,
            role,
            tenant_id: None,
            principal_type,
            tenant_name: None,
            principal_name: None,
            provisioning_state: None,
        }
    }
}
pub mod cluster_principal_properties {
    use super::*;
    #[doc = "Cluster principal role."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Role")]
    pub enum Role {
        AllDatabasesAdmin,
        AllDatabasesViewer,
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
                Self::AllDatabasesAdmin => serializer.serialize_unit_variant("Role", 0u32, "AllDatabasesAdmin"),
                Self::AllDatabasesViewer => serializer.serialize_unit_variant("Role", 1u32, "AllDatabasesViewer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Principal type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        App,
        Group,
        User,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::App => serializer.serialize_unit_variant("PrincipalType", 0u32, "App"),
                Self::Group => serializer.serialize_unit_variant("PrincipalType", 1u32, "Group"),
                Self::User => serializer.serialize_unit_variant("PrincipalType", 2u32, "User"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The compression type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Compression")]
pub enum Compression {
    None,
    GZip,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Compression {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Compression {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Compression {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("Compression", 0u32, "None"),
            Self::GZip => serializer.serialize_unit_variant("Compression", 1u32, "GZip"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Class representing a data connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Kind of the endpoint for the data connection"]
    pub kind: data_connection::Kind,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DataConnection {
    pub fn new(kind: data_connection::Kind) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            location: None,
            kind,
            system_data: None,
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
#[doc = "Class representing a Kusto database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Database {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Kind of the database"]
    pub kind: database::Kind,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Database {
    pub fn new(kind: database::Kind) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            location: None,
            kind,
            system_data: None,
        }
    }
}
pub mod database {
    use super::*;
    #[doc = "Kind of the database"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        ReadWrite,
        ReadOnlyFollowing,
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
                Self::ReadWrite => serializer.serialize_unit_variant("Kind", 0u32, "ReadWrite"),
                Self::ReadOnlyFollowing => serializer.serialize_unit_variant("Kind", 1u32, "ReadOnlyFollowing"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "Class representing a database principal assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabasePrincipalAssignment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "A class representing database principal property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabasePrincipalProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DatabasePrincipalAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list Kusto database principal assignments operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabasePrincipalAssignmentListResult {
    #[doc = "The list of Kusto database principal assignments."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabasePrincipalAssignment>,
}
impl azure_core::Continuable for DatabasePrincipalAssignmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DatabasePrincipalAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class representing database principal property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabasePrincipalProperties {
    #[doc = "The principal ID assigned to the database principal. It can be a user email, application ID, or security group name."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "Database principal role."]
    pub role: database_principal_properties::Role,
    #[doc = "The tenant id of the principal"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Principal type."]
    #[serde(rename = "principalType")]
    pub principal_type: database_principal_properties::PrincipalType,
    #[doc = "The tenant name of the principal"]
    #[serde(rename = "tenantName", default, skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
    #[doc = "The principal name"]
    #[serde(rename = "principalName", default, skip_serializing_if = "Option::is_none")]
    pub principal_name: Option<String>,
    #[doc = "The provisioned state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ResourceProvisioningState>,
}
impl DatabasePrincipalProperties {
    pub fn new(
        principal_id: String,
        role: database_principal_properties::Role,
        principal_type: database_principal_properties::PrincipalType,
    ) -> Self {
        Self {
            principal_id,
            role,
            tenant_id: None,
            principal_type,
            tenant_name: None,
            principal_name: None,
            provisioning_state: None,
        }
    }
}
pub mod database_principal_properties {
    use super::*;
    #[doc = "Database principal role."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Role")]
    pub enum Role {
        Admin,
        Ingestor,
        Monitor,
        User,
        UnrestrictedViewer,
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
                Self::UnrestrictedViewer => serializer.serialize_unit_variant("Role", 4u32, "UnrestrictedViewer"),
                Self::Viewer => serializer.serialize_unit_variant("Role", 5u32, "Viewer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Principal type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        App,
        Group,
        User,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::App => serializer.serialize_unit_variant("PrincipalType", 0u32, "App"),
                Self::Group => serializer.serialize_unit_variant("PrincipalType", 1u32, "Group"),
                Self::User => serializer.serialize_unit_variant("PrincipalType", 2u32, "User"),
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
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[doc = "The mapping rule to be used to ingest the data. Optionally the mapping information can be added to each message."]
    #[serde(rename = "mappingRuleName", default, skip_serializing_if = "Option::is_none")]
    pub mapping_rule_name: Option<String>,
    #[doc = "The data format of the message. Optionally the data format can be added to each message."]
    #[serde(rename = "dataFormat", default, skip_serializing_if = "Option::is_none")]
    pub data_format: Option<EventGridDataFormat>,
    #[doc = "A Boolean value that, if set to true, indicates that ingestion should ignore the first record of every file"]
    #[serde(rename = "ignoreFirstRecord", default, skip_serializing_if = "Option::is_none")]
    pub ignore_first_record: Option<bool>,
    #[doc = "The name of blob storage event type to process."]
    #[serde(rename = "blobStorageEventType", default, skip_serializing_if = "Option::is_none")]
    pub blob_storage_event_type: Option<BlobStorageEventType>,
    #[doc = "The provisioned state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ResourceProvisioningState>,
}
impl EventGridConnectionProperties {
    pub fn new(storage_account_resource_id: String, event_hub_resource_id: String, consumer_group: String) -> Self {
        Self {
            storage_account_resource_id,
            event_hub_resource_id,
            consumer_group,
            table_name: None,
            mapping_rule_name: None,
            data_format: None,
            ignore_first_record: None,
            blob_storage_event_type: None,
            provisioning_state: None,
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
#[doc = "The data format of the message. Optionally the data format can be added to each message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EventGridDataFormat")]
pub enum EventGridDataFormat {
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
    #[serde(rename = "PARQUET")]
    Parquet,
    #[serde(rename = "ORC")]
    Orc,
    #[serde(rename = "APACHEAVRO")]
    Apacheavro,
    #[serde(rename = "W3CLOGFILE")]
    W3clogfile,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EventGridDataFormat {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EventGridDataFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EventGridDataFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Multijson => serializer.serialize_unit_variant("EventGridDataFormat", 0u32, "MULTIJSON"),
            Self::Json => serializer.serialize_unit_variant("EventGridDataFormat", 1u32, "JSON"),
            Self::Csv => serializer.serialize_unit_variant("EventGridDataFormat", 2u32, "CSV"),
            Self::Tsv => serializer.serialize_unit_variant("EventGridDataFormat", 3u32, "TSV"),
            Self::Scsv => serializer.serialize_unit_variant("EventGridDataFormat", 4u32, "SCSV"),
            Self::Sohsv => serializer.serialize_unit_variant("EventGridDataFormat", 5u32, "SOHSV"),
            Self::Psv => serializer.serialize_unit_variant("EventGridDataFormat", 6u32, "PSV"),
            Self::Txt => serializer.serialize_unit_variant("EventGridDataFormat", 7u32, "TXT"),
            Self::Raw => serializer.serialize_unit_variant("EventGridDataFormat", 8u32, "RAW"),
            Self::Singlejson => serializer.serialize_unit_variant("EventGridDataFormat", 9u32, "SINGLEJSON"),
            Self::Avro => serializer.serialize_unit_variant("EventGridDataFormat", 10u32, "AVRO"),
            Self::Tsve => serializer.serialize_unit_variant("EventGridDataFormat", 11u32, "TSVE"),
            Self::Parquet => serializer.serialize_unit_variant("EventGridDataFormat", 12u32, "PARQUET"),
            Self::Orc => serializer.serialize_unit_variant("EventGridDataFormat", 13u32, "ORC"),
            Self::Apacheavro => serializer.serialize_unit_variant("EventGridDataFormat", 14u32, "APACHEAVRO"),
            Self::W3clogfile => serializer.serialize_unit_variant("EventGridDataFormat", 15u32, "W3CLOGFILE"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
    pub data_format: Option<EventHubDataFormat>,
    #[doc = "System properties of the event hub"]
    #[serde(rename = "eventSystemProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub event_system_properties: Vec<String>,
    #[doc = "The compression type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<Compression>,
    #[doc = "The provisioned state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ResourceProvisioningState>,
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
            compression: None,
            provisioning_state: None,
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
#[doc = "The data format of the message. Optionally the data format can be added to each message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EventHubDataFormat")]
pub enum EventHubDataFormat {
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
    #[serde(rename = "PARQUET")]
    Parquet,
    #[serde(rename = "ORC")]
    Orc,
    #[serde(rename = "APACHEAVRO")]
    Apacheavro,
    #[serde(rename = "W3CLOGFILE")]
    W3clogfile,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EventHubDataFormat {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EventHubDataFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EventHubDataFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Multijson => serializer.serialize_unit_variant("EventHubDataFormat", 0u32, "MULTIJSON"),
            Self::Json => serializer.serialize_unit_variant("EventHubDataFormat", 1u32, "JSON"),
            Self::Csv => serializer.serialize_unit_variant("EventHubDataFormat", 2u32, "CSV"),
            Self::Tsv => serializer.serialize_unit_variant("EventHubDataFormat", 3u32, "TSV"),
            Self::Scsv => serializer.serialize_unit_variant("EventHubDataFormat", 4u32, "SCSV"),
            Self::Sohsv => serializer.serialize_unit_variant("EventHubDataFormat", 5u32, "SOHSV"),
            Self::Psv => serializer.serialize_unit_variant("EventHubDataFormat", 6u32, "PSV"),
            Self::Txt => serializer.serialize_unit_variant("EventHubDataFormat", 7u32, "TXT"),
            Self::Raw => serializer.serialize_unit_variant("EventHubDataFormat", 8u32, "RAW"),
            Self::Singlejson => serializer.serialize_unit_variant("EventHubDataFormat", 9u32, "SINGLEJSON"),
            Self::Avro => serializer.serialize_unit_variant("EventHubDataFormat", 10u32, "AVRO"),
            Self::Tsve => serializer.serialize_unit_variant("EventHubDataFormat", 11u32, "TSVE"),
            Self::Parquet => serializer.serialize_unit_variant("EventHubDataFormat", 12u32, "PARQUET"),
            Self::Orc => serializer.serialize_unit_variant("EventHubDataFormat", 13u32, "ORC"),
            Self::Apacheavro => serializer.serialize_unit_variant("EventHubDataFormat", 14u32, "APACHEAVRO"),
            Self::W3clogfile => serializer.serialize_unit_variant("EventHubDataFormat", 15u32, "W3CLOGFILE"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Class representing the Kusto Iot hub connection properties."]
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
    pub data_format: Option<IotHubDataFormat>,
    #[doc = "System properties of the iot hub"]
    #[serde(rename = "eventSystemProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub event_system_properties: Vec<String>,
    #[doc = "The name of the share access policy"]
    #[serde(rename = "sharedAccessPolicyName")]
    pub shared_access_policy_name: String,
    #[doc = "The provisioned state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ResourceProvisioningState>,
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
            provisioning_state: None,
        }
    }
}
#[doc = "Class representing an iot hub data connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubDataConnection {
    #[serde(flatten)]
    pub data_connection: DataConnection,
    #[doc = "Class representing the Kusto Iot hub connection properties."]
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
#[doc = "The data format of the message. Optionally the data format can be added to each message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IotHubDataFormat")]
pub enum IotHubDataFormat {
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
    #[serde(rename = "PARQUET")]
    Parquet,
    #[serde(rename = "ORC")]
    Orc,
    #[serde(rename = "APACHEAVRO")]
    Apacheavro,
    #[serde(rename = "W3CLOGFILE")]
    W3clogfile,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IotHubDataFormat {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IotHubDataFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IotHubDataFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Multijson => serializer.serialize_unit_variant("IotHubDataFormat", 0u32, "MULTIJSON"),
            Self::Json => serializer.serialize_unit_variant("IotHubDataFormat", 1u32, "JSON"),
            Self::Csv => serializer.serialize_unit_variant("IotHubDataFormat", 2u32, "CSV"),
            Self::Tsv => serializer.serialize_unit_variant("IotHubDataFormat", 3u32, "TSV"),
            Self::Scsv => serializer.serialize_unit_variant("IotHubDataFormat", 4u32, "SCSV"),
            Self::Sohsv => serializer.serialize_unit_variant("IotHubDataFormat", 5u32, "SOHSV"),
            Self::Psv => serializer.serialize_unit_variant("IotHubDataFormat", 6u32, "PSV"),
            Self::Txt => serializer.serialize_unit_variant("IotHubDataFormat", 7u32, "TXT"),
            Self::Raw => serializer.serialize_unit_variant("IotHubDataFormat", 8u32, "RAW"),
            Self::Singlejson => serializer.serialize_unit_variant("IotHubDataFormat", 9u32, "SINGLEJSON"),
            Self::Avro => serializer.serialize_unit_variant("IotHubDataFormat", 10u32, "AVRO"),
            Self::Tsve => serializer.serialize_unit_variant("IotHubDataFormat", 11u32, "TSVE"),
            Self::Parquet => serializer.serialize_unit_variant("IotHubDataFormat", 12u32, "PARQUET"),
            Self::Orc => serializer.serialize_unit_variant("IotHubDataFormat", 13u32, "ORC"),
            Self::Apacheavro => serializer.serialize_unit_variant("IotHubDataFormat", 14u32, "APACHEAVRO"),
            Self::W3clogfile => serializer.serialize_unit_variant("IotHubDataFormat", 15u32, "W3CLOGFILE"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Class representing a Kusto kusto pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KustoPool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Azure SKU definition."]
    pub sku: AzureSku,
    #[doc = "Class representing the Kusto pool properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<KustoPoolProperties>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl KustoPool {
    pub fn new(tracked_resource: TrackedResource, sku: AzureSku) -> Self {
        Self {
            tracked_resource,
            sku,
            properties: None,
            etag: None,
            system_data: None,
        }
    }
}
#[doc = "The object sent for a kusto pool check name availability request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KustoPoolCheckNameRequest {
    #[doc = "Kusto Pool name."]
    pub name: String,
    #[doc = "The type of resource, Microsoft.Synapse/workspaces/kustoPools."]
    #[serde(rename = "type")]
    pub type_: kusto_pool_check_name_request::Type,
}
impl KustoPoolCheckNameRequest {
    pub fn new(name: String, type_: kusto_pool_check_name_request::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod kusto_pool_check_name_request {
    use super::*;
    #[doc = "The type of resource, Microsoft.Synapse/workspaces/kustoPools."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Synapse/workspaces/kustoPools")]
        MicrosoftSynapseWorkspacesKustoPools,
    }
}
#[doc = "The list Kusto pools operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KustoPoolListResult {
    #[doc = "The list of Kusto pools."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<KustoPool>,
}
impl KustoPoolListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the Kusto pool properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KustoPoolProperties {
    #[doc = "The state of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<kusto_pool_properties::State>,
    #[doc = "The provisioned state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ResourceProvisioningState>,
    #[doc = "The Kusto Pool URI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "The Kusto Pool data ingestion URI."]
    #[serde(rename = "dataIngestionUri", default, skip_serializing_if = "Option::is_none")]
    pub data_ingestion_uri: Option<String>,
    #[doc = "The reason for the Kusto Pool's current state."]
    #[serde(rename = "stateReason", default, skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[doc = "The engine type"]
    #[serde(rename = "engineType", default, skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<kusto_pool_properties::EngineType>,
    #[doc = "The workspace unique identifier."]
    #[serde(rename = "workspaceUid", default, skip_serializing_if = "Option::is_none")]
    pub workspace_uid: Option<String>,
}
impl KustoPoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod kusto_pool_properties {
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
    #[doc = "The engine type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EngineType")]
    pub enum EngineType {
        V2,
        V3,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EngineType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EngineType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EngineType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::V2 => serializer.serialize_unit_variant("EngineType", 0u32, "V2"),
                Self::V3 => serializer.serialize_unit_variant("EngineType", 1u32, "V3"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Class representing an update to a Kusto kusto pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KustoPoolUpdate {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Azure SKU definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<AzureSku>,
    #[doc = "Class representing the Kusto pool properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<KustoPoolProperties>,
}
impl KustoPoolUpdate {
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
#[doc = "Class representing a read write database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadWriteDatabase {
    #[serde(flatten)]
    pub database: Database,
    #[doc = "Class representing the Kusto database properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReadWriteDatabaseProperties>,
}
impl ReadWriteDatabase {
    pub fn new(database: Database) -> Self {
        Self {
            database,
            properties: None,
        }
    }
}
#[doc = "Class representing the Kusto database properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReadWriteDatabaseProperties {
    #[doc = "The provisioned state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ResourceProvisioningState>,
    #[doc = "The time the data should be kept before it stops being accessible to queries in TimeSpan."]
    #[serde(rename = "softDeletePeriod", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_period: Option<String>,
    #[doc = "The time the data should be kept in cache for fast queries in TimeSpan."]
    #[serde(rename = "hotCachePeriod", default, skip_serializing_if = "Option::is_none")]
    pub hot_cache_period: Option<String>,
    #[doc = "A class that contains database statistics information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<DatabaseStatistics>,
    #[doc = "Indicates whether the database is followed."]
    #[serde(rename = "isFollowed", default, skip_serializing_if = "Option::is_none")]
    pub is_followed: Option<bool>,
}
impl ReadWriteDatabaseProperties {
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
#[doc = "The provisioned state of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceProvisioningState")]
pub enum ResourceProvisioningState {
    Running,
    Creating,
    Deleting,
    Succeeded,
    Failed,
    Moving,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Running => serializer.serialize_unit_variant("ResourceProvisioningState", 0u32, "Running"),
            Self::Creating => serializer.serialize_unit_variant("ResourceProvisioningState", 1u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("ResourceProvisioningState", 2u32, "Deleting"),
            Self::Succeeded => serializer.serialize_unit_variant("ResourceProvisioningState", 3u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ResourceProvisioningState", 4u32, "Failed"),
            Self::Moving => serializer.serialize_unit_variant("ResourceProvisioningState", 5u32, "Moving"),
            Self::Canceled => serializer.serialize_unit_variant("ResourceProvisioningState", 6u32, "Canceled"),
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
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
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

#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Storage Class Access Mode"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AccessMode")]
pub enum AccessMode {
    ReadWriteOnce,
    ReadWriteMany,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AccessMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AccessMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AccessMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ReadWriteOnce => serializer.serialize_unit_variant("AccessMode", 0u32, "ReadWriteOnce"),
            Self::ReadWriteMany => serializer.serialize_unit_variant("AccessMode", 1u32, "ReadWriteMany"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enum of advertise mode"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AdvertiseMode")]
pub enum AdvertiseMode {
    #[serde(rename = "ARP")]
    Arp,
    #[serde(rename = "BGP")]
    Bgp,
    Both,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AdvertiseMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AdvertiseMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AdvertiseMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Arp => serializer.serialize_unit_variant("AdvertiseMode", 0u32, "ARP"),
            Self::Bgp => serializer.serialize_unit_variant("AdvertiseMode", 1u32, "BGP"),
            Self::Both => serializer.serialize_unit_variant("AdvertiseMode", 2u32, "Both"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A BgpPeer resource for an Arc connected cluster (Microsoft.Kubernetes/connectedClusters)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BgpPeer {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Details of the BgpPeer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BgpPeerProperties>,
}
impl BgpPeer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a BgpPeer list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BgpPeerListResult {
    #[doc = "The BgpPeer items on this page"]
    pub value: Vec<BgpPeer>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BgpPeerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BgpPeerListResult {
    pub fn new(value: Vec<BgpPeer>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Details of the BgpPeer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BgpPeerProperties {
    #[doc = "My ASN"]
    #[serde(rename = "myAsn")]
    pub my_asn: i32,
    #[doc = "Peer ASN"]
    #[serde(rename = "peerAsn")]
    pub peer_asn: i32,
    #[doc = "Peer Address"]
    #[serde(rename = "peerAddress")]
    pub peer_address: String,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl BgpPeerProperties {
    pub fn new(my_asn: i32, peer_asn: i32, peer_address: String) -> Self {
        Self {
            my_asn,
            peer_asn,
            peer_address,
            provisioning_state: None,
        }
    }
}
#[doc = "The properties of Blob StorageClass"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobStorageClassTypeProperties {
    #[doc = "Azure Storage Account Name"]
    #[serde(rename = "azureStorageAccountName")]
    pub azure_storage_account_name: String,
    #[doc = "Azure Storage Account Key"]
    #[serde(rename = "azureStorageAccountKey")]
    pub azure_storage_account_key: String,
}
impl BlobStorageClassTypeProperties {
    pub fn new(azure_storage_account_name: String, azure_storage_account_key: String) -> Self {
        Self {
            azure_storage_account_name,
            azure_storage_account_key,
        }
    }
}
#[doc = "Data resilience tier of a storage class"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataResilienceTier")]
pub enum DataResilienceTier {
    NotDataResilient,
    DataResilient,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataResilienceTier {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataResilienceTier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataResilienceTier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotDataResilient => serializer.serialize_unit_variant("DataResilienceTier", 0u32, "NotDataResilient"),
            Self::DataResilient => serializer.serialize_unit_variant("DataResilienceTier", 1u32, "DataResilient"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "Failover tier of a storage class"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FailoverTier")]
pub enum FailoverTier {
    NotAvailable,
    Slow,
    Fast,
    Super,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FailoverTier {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FailoverTier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FailoverTier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotAvailable => serializer.serialize_unit_variant("FailoverTier", 0u32, "NotAvailable"),
            Self::Slow => serializer.serialize_unit_variant("FailoverTier", 1u32, "Slow"),
            Self::Fast => serializer.serialize_unit_variant("FailoverTier", 2u32, "Fast"),
            Self::Super => serializer.serialize_unit_variant("FailoverTier", 3u32, "Super"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A LoadBalancer resource for an Arc connected cluster (Microsoft.Kubernetes/connectedClusters)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancer {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Details of the LoadBalancer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LoadBalancerProperties>,
}
impl LoadBalancer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a LoadBalancer list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerListResult {
    #[doc = "The LoadBalancer items on this page"]
    pub value: Vec<LoadBalancer>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LoadBalancerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LoadBalancerListResult {
    pub fn new(value: Vec<LoadBalancer>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Details of the LoadBalancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerProperties {
    #[doc = "IP Range"]
    pub addresses: Vec<String>,
    #[doc = "A dynamic label mapping to select related services. For instance, if you want to create a load balancer only for services with label \"a=b\", then please specify {\"a\": \"b\"} in the field."]
    #[serde(rename = "serviceSelector", default, skip_serializing_if = "Option::is_none")]
    pub service_selector: Option<serde_json::Value>,
    #[doc = "Enum of advertise mode"]
    #[serde(rename = "advertiseMode")]
    pub advertise_mode: AdvertiseMode,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl LoadBalancerProperties {
    pub fn new(addresses: Vec<String>, advertise_mode: AdvertiseMode) -> Self {
        Self {
            addresses,
            service_selector: None,
            advertise_mode,
            provisioning_state: None,
        }
    }
}
#[doc = "The properties of Native StorageClass"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NativeStorageClassTypeProperties {}
impl NativeStorageClassTypeProperties {
    pub fn new() -> Self {
        Self {}
    }
}
#[doc = "The action to take when a NFS volume is deleted"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NfsDirectoryActionOnVolumeDeletion")]
pub enum NfsDirectoryActionOnVolumeDeletion {
    Delete,
    Retain,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NfsDirectoryActionOnVolumeDeletion {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NfsDirectoryActionOnVolumeDeletion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NfsDirectoryActionOnVolumeDeletion {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Delete => serializer.serialize_unit_variant("NfsDirectoryActionOnVolumeDeletion", 0u32, "Delete"),
            Self::Retain => serializer.serialize_unit_variant("NfsDirectoryActionOnVolumeDeletion", 1u32, "Retain"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties of NFS StorageClass"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NfsStorageClassTypeProperties {
    #[doc = "NFS Server"]
    pub server: String,
    #[doc = "NFS share"]
    pub share: String,
    #[doc = "Sub directory under share. If the sub directory doesn't exist, driver will create it"]
    #[serde(rename = "subDir", default, skip_serializing_if = "Option::is_none")]
    pub sub_dir: Option<String>,
    #[doc = "Mounted folder permissions. Default is 0. If set as non-zero, driver will perform `chmod` after mount"]
    #[serde(rename = "mountPermissions", default, skip_serializing_if = "Option::is_none")]
    pub mount_permissions: Option<String>,
    #[doc = "The action to take when a NFS volume is deleted"]
    #[serde(rename = "onDelete", default, skip_serializing_if = "Option::is_none")]
    pub on_delete: Option<NfsDirectoryActionOnVolumeDeletion>,
}
impl NfsStorageClassTypeProperties {
    pub fn new(server: String, share: String) -> Self {
        Self {
            server,
            share,
            sub_dir: None,
            mount_permissions: None,
            on_delete: None,
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
#[doc = "Performance tier of a storage class"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PerformanceTier")]
pub enum PerformanceTier {
    Undefined,
    Basic,
    Standard,
    Premium,
    Ultra,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PerformanceTier {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PerformanceTier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PerformanceTier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Undefined => serializer.serialize_unit_variant("PerformanceTier", 0u32, "Undefined"),
            Self::Basic => serializer.serialize_unit_variant("PerformanceTier", 1u32, "Basic"),
            Self::Standard => serializer.serialize_unit_variant("PerformanceTier", 2u32, "Standard"),
            Self::Premium => serializer.serialize_unit_variant("PerformanceTier", 3u32, "Premium"),
            Self::Ultra => serializer.serialize_unit_variant("PerformanceTier", 4u32, "Ultra"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The status of the current operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Provisioning,
    Updating,
    Deleting,
    Accepted,
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
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
            Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Accepted"),
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
#[doc = "The properties of RWX StorageClass"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RwxStorageClassTypeProperties {
    #[doc = "The backing storageclass used to create new storageclass"]
    #[serde(rename = "backingStorageClassName")]
    pub backing_storage_class_name: String,
}
impl RwxStorageClassTypeProperties {
    pub fn new(backing_storage_class_name: String) -> Self {
        Self {
            backing_storage_class_name,
        }
    }
}
#[doc = "Type of a storage class"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScType")]
pub enum ScType {
    Native,
    #[serde(rename = "RWX")]
    Rwx,
    Blob,
    #[serde(rename = "NFS")]
    Nfs,
    #[serde(rename = "SMB")]
    Smb,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Native => serializer.serialize_unit_variant("ScType", 0u32, "Native"),
            Self::Rwx => serializer.serialize_unit_variant("ScType", 1u32, "RWX"),
            Self::Blob => serializer.serialize_unit_variant("ScType", 2u32, "Blob"),
            Self::Nfs => serializer.serialize_unit_variant("ScType", 3u32, "NFS"),
            Self::Smb => serializer.serialize_unit_variant("ScType", 4u32, "SMB"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties for the service resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceProperties {
    #[doc = "The object id of the service principal of the RP provisioned in the tenant"]
    #[serde(rename = "rpObjectId", default, skip_serializing_if = "Option::is_none")]
    pub rp_object_id: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Service resource for an Arc connected cluster (Microsoft.Kubernetes/connectedClusters)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties for the service resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceProperties>,
}
impl ServiceResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a ServiceResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceResourceListResult {
    #[doc = "The ServiceResource items on this page"]
    pub value: Vec<ServiceResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServiceResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServiceResourceListResult {
    pub fn new(value: Vec<ServiceResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of SMB StorageClass"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmbStorageClassTypeProperties {
    #[doc = "SMB Source"]
    pub source: String,
    #[doc = "Sub directory under share. If the sub directory doesn't exist, driver will create it"]
    #[serde(rename = "subDir", default, skip_serializing_if = "Option::is_none")]
    pub sub_dir: Option<String>,
    #[doc = "Server username"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "Server password"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Server domain"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}
impl SmbStorageClassTypeProperties {
    pub fn new(source: String) -> Self {
        Self {
            source,
            sub_dir: None,
            username: None,
            password: None,
            domain: None,
        }
    }
}
#[doc = "Details of the StorageClass StorageClass."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageClassProperties {
    #[doc = "Ability to expand volumes of a storage class"]
    #[serde(rename = "allowVolumeExpansion", default, skip_serializing_if = "Option::is_none")]
    pub allow_volume_expansion: Option<VolumeExpansion>,
    #[doc = "Additional mount options"]
    #[serde(
        rename = "mountOptions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mount_options: Vec<String>,
    #[doc = "Provisioner name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioner: Option<String>,
    #[doc = "Storage class volume binding mode"]
    #[serde(rename = "volumeBindingMode", default, skip_serializing_if = "Option::is_none")]
    pub volume_binding_mode: Option<VolumeBindingMode>,
    #[doc = "The access mode: [ReadWriteOnce, ReadWriteMany] or [ReadWriteOnce]"]
    #[serde(
        rename = "accessModes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub access_modes: Vec<AccessMode>,
    #[doc = "Data resilience tier of a storage class"]
    #[serde(rename = "dataResilience", default, skip_serializing_if = "Option::is_none")]
    pub data_resilience: Option<DataResilienceTier>,
    #[doc = "Failover tier of a storage class"]
    #[serde(rename = "failoverSpeed", default, skip_serializing_if = "Option::is_none")]
    pub failover_speed: Option<FailoverTier>,
    #[doc = "Limitations of the storage class"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub limitations: Vec<String>,
    #[doc = "Performance tier of a storage class"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performance: Option<PerformanceTier>,
    #[doc = "Selection priority when multiple storage classes meet the criteria. 0: Highest, -1: Never use"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[doc = "The properties of storage class of the StorageClass"]
    #[serde(rename = "typeProperties")]
    pub type_properties: StorageClassTypePropertiesUnion,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl StorageClassProperties {
    pub fn new(type_properties: StorageClassTypePropertiesUnion) -> Self {
        Self {
            allow_volume_expansion: None,
            mount_options: Vec::new(),
            provisioner: None,
            volume_binding_mode: None,
            access_modes: Vec::new(),
            data_resilience: None,
            failover_speed: None,
            limitations: Vec::new(),
            performance: None,
            priority: None,
            type_properties,
            provisioning_state: None,
        }
    }
}
#[doc = "The model for updating storageClass properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageClassPropertiesUpdate {
    #[doc = "Ability to expand volumes of a storage class"]
    #[serde(rename = "allowVolumeExpansion", default, skip_serializing_if = "Option::is_none")]
    pub allow_volume_expansion: Option<VolumeExpansion>,
    #[doc = "Additional mount options"]
    #[serde(
        rename = "mountOptions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mount_options: Vec<String>,
    #[doc = "The access mode: [ReadWriteOnce, ReadWriteMany] or [ReadWriteOnce]"]
    #[serde(
        rename = "accessModes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub access_modes: Vec<AccessMode>,
    #[doc = "Data resilience tier of a storage class"]
    #[serde(rename = "dataResilience", default, skip_serializing_if = "Option::is_none")]
    pub data_resilience: Option<DataResilienceTier>,
    #[doc = "Failover tier of a storage class"]
    #[serde(rename = "failoverSpeed", default, skip_serializing_if = "Option::is_none")]
    pub failover_speed: Option<FailoverTier>,
    #[doc = "Limitations of the storage class"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub limitations: Vec<String>,
    #[doc = "Performance tier of a storage class"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performance: Option<PerformanceTier>,
    #[doc = "Selection priority when multiple storage classes meet the criteria. 0: Highest, -1: Never use"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[doc = "The model for update a storageClass"]
    #[serde(rename = "typeProperties", default, skip_serializing_if = "Option::is_none")]
    pub type_properties: Option<StorageClassTypePropertiesUpdate>,
}
impl StorageClassPropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A StorageClass resource for an Arc connected cluster (Microsoft.Kubernetes/connectedClusters)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageClassResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Details of the StorageClass StorageClass."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageClassProperties>,
}
impl StorageClassResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a StorageClassResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageClassResourceListResult {
    #[doc = "The StorageClassResource items on this page"]
    pub value: Vec<StorageClassResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StorageClassResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl StorageClassResourceListResult {
    pub fn new(value: Vec<StorageClassResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The model for updating a storageClass"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageClassResourceUpdate {
    #[doc = "The model for updating storageClass properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageClassPropertiesUpdate>,
}
impl StorageClassResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of a storage class"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StorageClassTypePropertiesUnion {
    Blob(BlobStorageClassTypeProperties),
    Native(NativeStorageClassTypeProperties),
    #[serde(rename = "NFS")]
    Nfs(NfsStorageClassTypeProperties),
    #[serde(rename = "RWX")]
    Rwx(RwxStorageClassTypeProperties),
    #[serde(rename = "SMB")]
    Smb(SmbStorageClassTypeProperties),
}
#[doc = "The model for update a storageClass"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageClassTypePropertiesUpdate {
    #[doc = "The backing storageclass used to create new storageclass"]
    #[serde(rename = "backingStorageClassName", default, skip_serializing_if = "Option::is_none")]
    pub backing_storage_class_name: Option<String>,
    #[doc = "Azure Storage Account Name"]
    #[serde(rename = "azureStorageAccountName", default, skip_serializing_if = "Option::is_none")]
    pub azure_storage_account_name: Option<String>,
    #[doc = "Azure Storage Account Key"]
    #[serde(rename = "azureStorageAccountKey", default, skip_serializing_if = "Option::is_none")]
    pub azure_storage_account_key: Option<String>,
    #[doc = "NFS Server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[doc = "NFS share"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share: Option<String>,
    #[doc = "Sub directory under share. If the sub directory doesn't exist, driver will create it"]
    #[serde(rename = "subDir", default, skip_serializing_if = "Option::is_none")]
    pub sub_dir: Option<String>,
    #[doc = "Mounted folder permissions. Default is 0. If set as non-zero, driver will perform `chmod` after mount"]
    #[serde(rename = "mountPermissions", default, skip_serializing_if = "Option::is_none")]
    pub mount_permissions: Option<String>,
    #[doc = "The action to take when a NFS volume is deleted"]
    #[serde(rename = "onDelete", default, skip_serializing_if = "Option::is_none")]
    pub on_delete: Option<NfsDirectoryActionOnVolumeDeletion>,
    #[doc = "SMB Source"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Server username"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "Server password"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Server domain"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}
impl StorageClassTypePropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage class volume binding mode"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VolumeBindingMode")]
pub enum VolumeBindingMode {
    Immediate,
    WaitForFirstConsumer,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VolumeBindingMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VolumeBindingMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VolumeBindingMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Immediate => serializer.serialize_unit_variant("VolumeBindingMode", 0u32, "Immediate"),
            Self::WaitForFirstConsumer => serializer.serialize_unit_variant("VolumeBindingMode", 1u32, "WaitForFirstConsumer"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Ability to expand volumes of a storage class"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VolumeExpansion")]
pub enum VolumeExpansion {
    Allow,
    Disallow,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VolumeExpansion {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VolumeExpansion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VolumeExpansion {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Allow => serializer.serialize_unit_variant("VolumeExpansion", 0u32, "Allow"),
            Self::Disallow => serializer.serialize_unit_variant("VolumeExpansion", 1u32, "Disallow"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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

#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The provisioning state of a resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureResourceManagerResourceProvisioningState")]
pub enum AzureResourceManagerResourceProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureResourceManagerResourceProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureResourceManagerResourceProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureResourceManagerResourceProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("AzureResourceManagerResourceProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("AzureResourceManagerResourceProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("AzureResourceManagerResourceProvisioningState", 2u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The status of the last provisioning operation performed on the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DatabaseWatcherProvisioningState")]
pub enum DatabaseWatcherProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DatabaseWatcherProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DatabaseWatcherProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DatabaseWatcherProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("DatabaseWatcherProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("DatabaseWatcherProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("DatabaseWatcherProvisioningState", 2u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties of a data store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Datastore {
    #[doc = "The Azure ResourceId of an Azure Data Explorer cluster."]
    #[serde(rename = "adxClusterResourceId", default, skip_serializing_if = "Option::is_none")]
    pub adx_cluster_resource_id: Option<String>,
    #[doc = "The Kusto cluster display name."]
    #[serde(rename = "kustoClusterDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub kusto_cluster_display_name: Option<String>,
    #[doc = "The Kusto cluster URI."]
    #[serde(rename = "kustoClusterUri")]
    pub kusto_cluster_uri: String,
    #[doc = "The Kusto data ingestion URI."]
    #[serde(rename = "kustoDataIngestionUri")]
    pub kusto_data_ingestion_uri: String,
    #[doc = "The name of a Kusto database."]
    #[serde(rename = "kustoDatabaseName")]
    pub kusto_database_name: String,
    #[doc = "The Kusto management URL."]
    #[serde(rename = "kustoManagementUrl")]
    pub kusto_management_url: String,
    #[doc = "The type of Kusto offering."]
    #[serde(rename = "kustoOfferingType")]
    pub kusto_offering_type: KustoOfferingType,
}
impl Datastore {
    pub fn new(
        kusto_cluster_uri: String,
        kusto_data_ingestion_uri: String,
        kusto_database_name: String,
        kusto_management_url: String,
        kusto_offering_type: KustoOfferingType,
    ) -> Self {
        Self {
            adx_cluster_resource_id: None,
            kusto_cluster_display_name: None,
            kusto_cluster_uri,
            kusto_data_ingestion_uri,
            kusto_database_name,
            kusto_management_url,
            kusto_offering_type,
        }
    }
}
#[doc = "The properties of a data store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatastoreUpdate {
    #[doc = "The Azure ResourceId of an Azure Data Explorer cluster."]
    #[serde(rename = "adxClusterResourceId", default, skip_serializing_if = "Option::is_none")]
    pub adx_cluster_resource_id: Option<String>,
    #[doc = "The Kusto cluster display name."]
    #[serde(rename = "kustoClusterDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub kusto_cluster_display_name: Option<String>,
    #[doc = "The Kusto cluster URI."]
    #[serde(rename = "kustoClusterUri", default, skip_serializing_if = "Option::is_none")]
    pub kusto_cluster_uri: Option<String>,
    #[doc = "The Kusto data ingestion URI."]
    #[serde(rename = "kustoDataIngestionUri", default, skip_serializing_if = "Option::is_none")]
    pub kusto_data_ingestion_uri: Option<String>,
    #[doc = "The name of a Kusto database."]
    #[serde(rename = "kustoDatabaseName", default, skip_serializing_if = "Option::is_none")]
    pub kusto_database_name: Option<String>,
    #[doc = "The Kusto management URL."]
    #[serde(rename = "kustoManagementUrl", default, skip_serializing_if = "Option::is_none")]
    pub kusto_management_url: Option<String>,
    #[doc = "The type of Kusto offering."]
    #[serde(rename = "kustoOfferingType", default, skip_serializing_if = "Option::is_none")]
    pub kusto_offering_type: Option<KustoOfferingType>,
}
impl DatastoreUpdate {
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
#[doc = "The type of Kusto offering."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "KustoOfferingType")]
pub enum KustoOfferingType {
    #[serde(rename = "adx")]
    Adx,
    #[serde(rename = "free")]
    Free,
    #[serde(rename = "fabric")]
    Fabric,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for KustoOfferingType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for KustoOfferingType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for KustoOfferingType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Adx => serializer.serialize_unit_variant("KustoOfferingType", 0u32, "adx"),
            Self::Free => serializer.serialize_unit_variant("KustoOfferingType", 1u32, "free"),
            Self::Fabric => serializer.serialize_unit_variant("KustoOfferingType", 2u32, "fabric"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: ManagedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedServiceIdentityType")]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned, UserAssigned")]
    SystemAssignedUserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 2u32, "UserAssigned"),
            Self::SystemAssignedUserAssigned => {
                serializer.serialize_unit_variant("ManagedServiceIdentityType", 3u32, "SystemAssigned, UserAssigned")
            }
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
#[doc = "Concrete proxy resource types can be created by aliasing this type using a specific property type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The generic properties of a Shared Private Link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedPrivateLinkResourceProperties>,
}
impl SharedPrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SharedPrivateLinkResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedPrivateLinkResourceListResult {
    #[doc = "The SharedPrivateLinkResource items on this page"]
    pub value: Vec<SharedPrivateLinkResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SharedPrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SharedPrivateLinkResourceListResult {
    pub fn new(value: Vec<SharedPrivateLinkResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The generic properties of a Shared Private Link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedPrivateLinkResourceProperties {
    #[doc = "The resource id of the resource the shared private link resource is for."]
    #[serde(rename = "privateLinkResourceId")]
    pub private_link_resource_id: String,
    #[doc = "The group id from the provider of resource the shared private link resource is for."]
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[doc = "The request message for requesting approval of the shared private link resource."]
    #[serde(rename = "requestMessage")]
    pub request_message: String,
    #[doc = "The DNS zone to be included in the DNS name of the shared private link. Value is service-specific."]
    #[serde(rename = "dnsZone", default, skip_serializing_if = "Option::is_none")]
    pub dns_zone: Option<String>,
    #[doc = "Status of the shared private link resource. Can be Pending, Approved, Rejected or Disconnected."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SharedPrivateLinkResourceStatus>,
    #[doc = "The provisioning state of a resource type."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AzureResourceManagerResourceProvisioningState>,
}
impl SharedPrivateLinkResourceProperties {
    pub fn new(private_link_resource_id: String, group_id: String, request_message: String) -> Self {
        Self {
            private_link_resource_id,
            group_id,
            request_message,
            dns_zone: None,
            status: None,
            provisioning_state: None,
        }
    }
}
#[doc = "Status of the shared private link resource. Can be Pending, Approved, Rejected or Disconnected."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SharedPrivateLinkResourceStatus")]
pub enum SharedPrivateLinkResourceStatus {
    Pending,
    Approved,
    Rejected,
    Disconnected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SharedPrivateLinkResourceStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SharedPrivateLinkResourceStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SharedPrivateLinkResourceStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("SharedPrivateLinkResourceStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("SharedPrivateLinkResourceStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("SharedPrivateLinkResourceStatus", 2u32, "Rejected"),
            Self::Disconnected => serializer.serialize_unit_variant("SharedPrivateLinkResourceStatus", 3u32, "Disconnected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties specific to elastic pool in Azure SQL Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDbElasticPoolTargetProperties {
    #[serde(flatten)]
    pub target_properties: TargetProperties,
    #[doc = "The Azure ResourceId of an Azure SQL DB elastic pool target."]
    #[serde(rename = "sqlEpResourceId")]
    pub sql_ep_resource_id: String,
    #[doc = "The Azure ResourceId of the anchor database used to connect to an elastic pool."]
    #[serde(rename = "anchorDatabaseResourceId")]
    pub anchor_database_resource_id: String,
    #[doc = "Set to true to monitor a high availability replica of specified target, if any."]
    #[serde(rename = "readIntent", default, skip_serializing_if = "Option::is_none")]
    pub read_intent: Option<bool>,
}
impl SqlDbElasticPoolTargetProperties {
    pub fn new(target_properties: TargetProperties, sql_ep_resource_id: String, anchor_database_resource_id: String) -> Self {
        Self {
            target_properties,
            sql_ep_resource_id,
            anchor_database_resource_id,
            read_intent: None,
        }
    }
}
#[doc = "The properties specific to single database in Azure SQL Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDbSingleDatabaseTargetProperties {
    #[serde(flatten)]
    pub target_properties: TargetProperties,
    #[doc = "The Azure ResourceId of an Azure SQL DB single database target."]
    #[serde(rename = "sqlDbResourceId")]
    pub sql_db_resource_id: String,
    #[doc = "Set to true to monitor a high availability replica of specified target, if any."]
    #[serde(rename = "readIntent", default, skip_serializing_if = "Option::is_none")]
    pub read_intent: Option<bool>,
}
impl SqlDbSingleDatabaseTargetProperties {
    pub fn new(target_properties: TargetProperties, sql_db_resource_id: String) -> Self {
        Self {
            target_properties,
            sql_db_resource_id,
            read_intent: None,
        }
    }
}
#[doc = "The properties specific to Azure SQL Managed Instance targets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlMiTargetProperties {
    #[serde(flatten)]
    pub target_properties: TargetProperties,
    #[doc = "The Azure ResourceId of an Azure SQL Managed Instance target."]
    #[serde(rename = "sqlMiResourceId")]
    pub sql_mi_resource_id: String,
    #[doc = "The TCP port number to optionally use in the connection string when connecting to an Azure SQL Managed Instance target."]
    #[serde(rename = "connectionTcpPort", default, skip_serializing_if = "Option::is_none")]
    pub connection_tcp_port: Option<i32>,
    #[doc = "Set to true to monitor a high availability replica of specified target, if any."]
    #[serde(rename = "readIntent", default, skip_serializing_if = "Option::is_none")]
    pub read_intent: Option<bool>,
}
impl SqlMiTargetProperties {
    pub fn new(target_properties: TargetProperties, sql_mi_resource_id: String) -> Self {
        Self {
            target_properties,
            sql_mi_resource_id,
            connection_tcp_port: None,
            read_intent: None,
        }
    }
}
#[doc = "The properties specific to Azure SQL VM targets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlVmTargetProperties {
    #[serde(flatten)]
    pub target_properties: TargetProperties,
    #[doc = "The Azure ResourceId of an Azure SQL VM target."]
    #[serde(rename = "sqlVmResourceId")]
    pub sql_vm_resource_id: String,
    #[doc = "The TCP port number to optionally use in the connection string when connecting to an Azure SQL VM target."]
    #[serde(rename = "connectionTcpPort", default, skip_serializing_if = "Option::is_none")]
    pub connection_tcp_port: Option<i32>,
    #[doc = "The SQL instance name to optionally use in the connection string when connecting to an Azure SQL VM target."]
    #[serde(rename = "sqlNamedInstanceName", default, skip_serializing_if = "Option::is_none")]
    pub sql_named_instance_name: Option<String>,
}
impl SqlVmTargetProperties {
    pub fn new(target_properties: TargetProperties, sql_vm_resource_id: String) -> Self {
        Self {
            target_properties,
            sql_vm_resource_id,
            connection_tcp_port: None,
            sql_named_instance_name: None,
        }
    }
}
#[doc = "Concrete proxy resource types can be created by aliasing this type using a specific property type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Target {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The generic properties of a target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TargetPropertiesUnion>,
}
impl Target {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of authentication to use when connecting to a target."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TargetAuthenticationType")]
pub enum TargetAuthenticationType {
    Aad,
    Sql,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TargetAuthenticationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TargetAuthenticationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TargetAuthenticationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Aad => serializer.serialize_unit_variant("TargetAuthenticationType", 0u32, "Aad"),
            Self::Sql => serializer.serialize_unit_variant("TargetAuthenticationType", 1u32, "Sql"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a Target list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetListResult {
    #[doc = "The Target items on this page"]
    pub value: Vec<Target>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TargetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TargetListResult {
    pub fn new(value: Vec<Target>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The generic properties of a target."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetProperties {
    #[doc = "The type of authentication to use when connecting to a target."]
    #[serde(rename = "targetAuthenticationType")]
    pub target_authentication_type: TargetAuthenticationType,
    #[doc = "The vault specific details required if using SQL authentication to connect to a target."]
    #[serde(rename = "targetVault", default, skip_serializing_if = "Option::is_none")]
    pub target_vault: Option<VaultSecret>,
    #[doc = "The server name to use in the connection string when connecting to a target. Port number and instance name must be specified separately."]
    #[serde(rename = "connectionServerName")]
    pub connection_server_name: String,
    #[doc = "The provisioning state of a resource type."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AzureResourceManagerResourceProvisioningState>,
}
impl TargetProperties {
    pub fn new(target_authentication_type: TargetAuthenticationType, connection_server_name: String) -> Self {
        Self {
            target_authentication_type,
            target_vault: None,
            connection_server_name,
            provisioning_state: None,
        }
    }
}
#[doc = "Discriminator property for TargetProperties."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "targetType")]
pub enum TargetPropertiesUnion {
    SqlEp(SqlDbElasticPoolTargetProperties),
    SqlDb(SqlDbSingleDatabaseTargetProperties),
    SqlMi(SqlMiTargetProperties),
    SqlVm(SqlVmTargetProperties),
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
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The vault specific details required if using SQL authentication to connect to a target."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultSecret {
    #[doc = "The Azure ResourceId of the Key Vault instance storing database authentication secrets."]
    #[serde(rename = "akvResourceId", default, skip_serializing_if = "Option::is_none")]
    pub akv_resource_id: Option<String>,
    #[doc = "The path to the Key Vault secret storing the login name (aka user name, aka account name) for authentication to a target."]
    #[serde(rename = "akvTargetUser", default, skip_serializing_if = "Option::is_none")]
    pub akv_target_user: Option<String>,
    #[doc = "The path to the Key Vault secret storing the password for authentication to a target."]
    #[serde(rename = "akvTargetPassword", default, skip_serializing_if = "Option::is_none")]
    pub akv_target_password: Option<String>,
}
impl VaultSecret {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The DatabaseWatcherProviderHub resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Watcher {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The RP specific properties of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WatcherProperties>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl Watcher {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
        }
    }
}
#[doc = "The response of a Watcher list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatcherListResult {
    #[doc = "The Watcher items on this page"]
    pub value: Vec<Watcher>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WatcherListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WatcherListResult {
    pub fn new(value: Vec<Watcher>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The RP specific properties of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WatcherProperties {
    #[doc = "The properties of a data store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datastore: Option<Datastore>,
    #[doc = "The monitoring collection status of a watcher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WatcherStatus>,
    #[doc = "The status of the last provisioning operation performed on the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DatabaseWatcherProvisioningState>,
}
impl WatcherProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The monitoring collection status of a watcher."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WatcherStatus")]
pub enum WatcherStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Deleting,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WatcherStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WatcherStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WatcherStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Starting => serializer.serialize_unit_variant("WatcherStatus", 0u32, "Starting"),
            Self::Running => serializer.serialize_unit_variant("WatcherStatus", 1u32, "Running"),
            Self::Stopping => serializer.serialize_unit_variant("WatcherStatus", 2u32, "Stopping"),
            Self::Stopped => serializer.serialize_unit_variant("WatcherStatus", 3u32, "Stopped"),
            Self::Deleting => serializer.serialize_unit_variant("WatcherStatus", 4u32, "Deleting"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type used for update operations of the Watcher."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WatcherUpdate {
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the Watcher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WatcherUpdateProperties>,
}
impl WatcherUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the Watcher."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WatcherUpdateProperties {
    #[doc = "The properties of a data store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datastore: Option<DatastoreUpdate>,
}
impl WatcherUpdateProperties {
    pub fn new() -> Self {
        Self::default()
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

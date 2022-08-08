#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Workspace active directory administrator properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AadAdminProperties {
    #[doc = "Tenant ID of the workspace active directory administrator"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Login of the workspace active directory administrator"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[doc = "Workspace active directory administrator type"]
    #[serde(rename = "administratorType", default, skip_serializing_if = "Option::is_none")]
    pub administrator_type: Option<String>,
    #[doc = "Object ID of the workspace active directory administrator"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
}
impl AadAdminProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Auto-pausing properties of a Big Data pool powered by Apache Spark"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoPauseProperties {
    #[doc = "Number of minutes of idle time before the Big Data pool is automatically paused."]
    #[serde(rename = "delayInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub delay_in_minutes: Option<i32>,
    #[doc = "Whether auto-pausing is enabled for the Big Data pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl AutoPauseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Auto-scaling properties of a Big Data pool powered by Apache Spark"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoScaleProperties {
    #[doc = "The minimum number of nodes the Big Data pool can support."]
    #[serde(rename = "minNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub min_node_count: Option<i32>,
    #[doc = "Whether automatic scaling is enabled for the Big Data pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The maximum number of nodes the Big Data pool can support."]
    #[serde(rename = "maxNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub max_node_count: Option<i32>,
}
impl AutoScaleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An operation that is available in this resource provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableRpOperation {
    #[doc = "Description of an available operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<AvailableRpOperationDisplayInfo>,
    #[doc = "Whether this operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<String>,
    #[doc = "Operation name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "What is this?"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationMetaPropertyInfo>,
    #[doc = "Operation origin"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl AvailableRpOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of an available operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableRpOperationDisplayInfo {
    #[doc = "Operation description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Resource type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Resource provider name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Operation name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
}
impl AvailableRpOperationDisplayInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Active Directory Only Authentication Info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureAdOnlyAuthentication {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a active directory only authentication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureAdOnlyAuthenticationProperties>,
}
impl AzureAdOnlyAuthentication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of active directory only authentications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureAdOnlyAuthenticationListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AzureAdOnlyAuthentication>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AzureAdOnlyAuthenticationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AzureAdOnlyAuthenticationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a active directory only authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureAdOnlyAuthenticationProperties {
    #[doc = "Azure Active Directory only Authentication enabled."]
    #[serde(rename = "azureADOnlyAuthentication")]
    pub azure_ad_only_authentication: bool,
    #[doc = "property configuration state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<azure_ad_only_authentication_properties::State>,
    #[doc = "property configuration date"]
    #[serde(rename = "creationDate", with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
}
impl AzureAdOnlyAuthenticationProperties {
    pub fn new(azure_ad_only_authentication: bool) -> Self {
        Self {
            azure_ad_only_authentication,
            state: None,
            creation_date: None,
        }
    }
}
pub mod azure_ad_only_authentication_properties {
    use super::*;
    #[doc = "property configuration state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Consistent,
        InConsistent,
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
                Self::Consistent => serializer.serialize_unit_variant("State", 0u32, "Consistent"),
                Self::InConsistent => serializer.serialize_unit_variant("State", 1u32, "InConsistent"),
                Self::Updating => serializer.serialize_unit_variant("State", 2u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The resource model definition for an Azure Resource Manager resource with an etag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureEntityResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource Etag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl AzureEntityResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties patch for a Big Data pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BigDataPoolPatchInfo {
    #[doc = "Updated tags for the Big Data pool"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl BigDataPoolPatchInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Big Data pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BigDataPoolResourceInfo {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of a Big Data pool powered by Apache Spark"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BigDataPoolResourceProperties>,
}
impl BigDataPoolResourceInfo {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Collection of Big Data pool information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BigDataPoolResourceInfoListResult {
    #[doc = "Link to the next page of results"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of Big Data pools"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BigDataPoolResourceInfo>,
}
impl azure_core::Continuable for BigDataPoolResourceInfoListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BigDataPoolResourceInfoListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Big Data pool powered by Apache Spark"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BigDataPoolResourceProperties {
    #[doc = "The state of the Big Data pool."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Auto-scaling properties of a Big Data pool powered by Apache Spark"]
    #[serde(rename = "autoScale", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale: Option<AutoScaleProperties>,
    #[doc = "The time when the Big Data pool was created."]
    #[serde(rename = "creationDate", with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "Auto-pausing properties of a Big Data pool powered by Apache Spark"]
    #[serde(rename = "autoPause", default, skip_serializing_if = "Option::is_none")]
    pub auto_pause: Option<AutoPauseProperties>,
    #[doc = "Whether compute isolation is required or not."]
    #[serde(rename = "isComputeIsolationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_compute_isolation_enabled: Option<bool>,
    #[doc = "Whether session level packages enabled."]
    #[serde(rename = "sessionLevelPackagesEnabled", default, skip_serializing_if = "Option::is_none")]
    pub session_level_packages_enabled: Option<bool>,
    #[doc = "The cache size"]
    #[serde(rename = "cacheSize", default, skip_serializing_if = "Option::is_none")]
    pub cache_size: Option<i32>,
    #[doc = "Dynamic Executor Allocation Properties"]
    #[serde(rename = "dynamicExecutorAllocation", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_executor_allocation: Option<DynamicExecutorAllocation>,
    #[doc = "The Spark events folder"]
    #[serde(rename = "sparkEventsFolder", default, skip_serializing_if = "Option::is_none")]
    pub spark_events_folder: Option<String>,
    #[doc = "The number of nodes in the Big Data pool."]
    #[serde(rename = "nodeCount", default, skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i32>,
    #[doc = "Library requirements for a Big Data pool powered by Apache Spark"]
    #[serde(rename = "libraryRequirements", default, skip_serializing_if = "Option::is_none")]
    pub library_requirements: Option<LibraryRequirements>,
    #[doc = "List of custom libraries/packages associated with the spark pool."]
    #[serde(rename = "customLibraries", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_libraries: Vec<LibraryInfo>,
    #[doc = "SparkConfig Properties for a Big Data pool powered by Apache Spark"]
    #[serde(rename = "sparkConfigProperties", default, skip_serializing_if = "Option::is_none")]
    pub spark_config_properties: Option<SparkConfigProperties>,
    #[doc = "The Apache Spark version."]
    #[serde(rename = "sparkVersion", default, skip_serializing_if = "Option::is_none")]
    pub spark_version: Option<String>,
    #[doc = "The default folder where Spark logs will be written."]
    #[serde(rename = "defaultSparkLogFolder", default, skip_serializing_if = "Option::is_none")]
    pub default_spark_log_folder: Option<String>,
    #[doc = "The level of compute power that each node in the Big Data pool has."]
    #[serde(rename = "nodeSize", default, skip_serializing_if = "Option::is_none")]
    pub node_size: Option<big_data_pool_resource_properties::NodeSize>,
    #[doc = "The kind of nodes that the Big Data pool provides."]
    #[serde(rename = "nodeSizeFamily", default, skip_serializing_if = "Option::is_none")]
    pub node_size_family: Option<big_data_pool_resource_properties::NodeSizeFamily>,
    #[doc = "The time when the Big Data pool was updated successfully."]
    #[serde(rename = "lastSucceededTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_succeeded_timestamp: Option<time::OffsetDateTime>,
}
impl BigDataPoolResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod big_data_pool_resource_properties {
    use super::*;
    #[doc = "The level of compute power that each node in the Big Data pool has."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NodeSize")]
    pub enum NodeSize {
        None,
        Small,
        Medium,
        Large,
        XLarge,
        #[serde(rename = "XXLarge")]
        XxLarge,
        #[serde(rename = "XXXLarge")]
        XxxLarge,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NodeSize {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NodeSize {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NodeSize {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("NodeSize", 0u32, "None"),
                Self::Small => serializer.serialize_unit_variant("NodeSize", 1u32, "Small"),
                Self::Medium => serializer.serialize_unit_variant("NodeSize", 2u32, "Medium"),
                Self::Large => serializer.serialize_unit_variant("NodeSize", 3u32, "Large"),
                Self::XLarge => serializer.serialize_unit_variant("NodeSize", 4u32, "XLarge"),
                Self::XxLarge => serializer.serialize_unit_variant("NodeSize", 5u32, "XXLarge"),
                Self::XxxLarge => serializer.serialize_unit_variant("NodeSize", 6u32, "XXXLarge"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The kind of nodes that the Big Data pool provides."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NodeSizeFamily")]
    pub enum NodeSizeFamily {
        None,
        MemoryOptimized,
        #[serde(rename = "HardwareAcceleratedFPGA")]
        HardwareAcceleratedFpga,
        #[serde(rename = "HardwareAcceleratedGPU")]
        HardwareAcceleratedGpu,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NodeSizeFamily {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NodeSizeFamily {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NodeSizeFamily {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("NodeSizeFamily", 0u32, "None"),
                Self::MemoryOptimized => serializer.serialize_unit_variant("NodeSizeFamily", 1u32, "MemoryOptimized"),
                Self::HardwareAcceleratedFpga => serializer.serialize_unit_variant("NodeSizeFamily", 2u32, "HardwareAcceleratedFPGA"),
                Self::HardwareAcceleratedGpu => serializer.serialize_unit_variant("NodeSizeFamily", 3u32, "HardwareAcceleratedGPU"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A request about whether a workspace name is available"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityRequest {
    #[doc = "Workspace name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type: workspace"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckNameAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A response saying whether the workspace name is available"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResponse {
    #[doc = "Validation message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Whether the workspace name is available"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    #[doc = "Reason the workspace name is or is not available"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Workspace name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CheckNameAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The custom setup of running cmdkey commands."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CmdkeySetup {
    #[serde(flatten)]
    pub custom_setup_base: CustomSetupBase,
    #[doc = "Cmdkey command custom setup type properties."]
    #[serde(rename = "typeProperties")]
    pub type_properties: CmdkeySetupTypeProperties,
}
impl CmdkeySetup {
    pub fn new(custom_setup_base: CustomSetupBase, type_properties: CmdkeySetupTypeProperties) -> Self {
        Self {
            custom_setup_base,
            type_properties,
        }
    }
}
#[doc = "Cmdkey command custom setup type properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CmdkeySetupTypeProperties {
    #[doc = "The server name of data source access."]
    #[serde(rename = "targetName")]
    pub target_name: serde_json::Value,
    #[doc = "The user name of data source access."]
    #[serde(rename = "userName")]
    pub user_name: serde_json::Value,
    #[doc = "The base definition of a secret type."]
    pub password: SecretBase,
}
impl CmdkeySetupTypeProperties {
    pub fn new(target_name: serde_json::Value, user_name: serde_json::Value, password: SecretBase) -> Self {
        Self {
            target_name,
            user_name,
            password,
        }
    }
}
#[doc = "The custom setup of installing 3rd party components."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentSetup {
    #[serde(flatten)]
    pub custom_setup_base: CustomSetupBase,
    #[doc = "Installation of licensed component setup type properties."]
    #[serde(rename = "typeProperties")]
    pub type_properties: LicensedComponentSetupTypeProperties,
}
impl ComponentSetup {
    pub fn new(custom_setup_base: CustomSetupBase, type_properties: LicensedComponentSetupTypeProperties) -> Self {
        Self {
            custom_setup_base,
            type_properties,
        }
    }
}
#[doc = "Contains the information necessary to perform a create Sql pool restore point operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSqlPoolRestorePointDefinition {
    #[doc = "The restore point label to apply"]
    #[serde(rename = "restorePointLabel")]
    pub restore_point_label: String,
}
impl CreateSqlPoolRestorePointDefinition {
    pub fn new(restore_point_label: String) -> Self {
        Self { restore_point_label }
    }
}
#[doc = "Initial workspace AAD admin properties for a CSP subscription"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CspWorkspaceAdminProperties {
    #[doc = "AAD object ID of initial workspace admin"]
    #[serde(rename = "initialWorkspaceAdminObjectId", default, skip_serializing_if = "Option::is_none")]
    pub initial_workspace_admin_object_id: Option<String>,
}
impl CspWorkspaceAdminProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The base definition of the custom setup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomSetupBase {
    #[doc = "The type of custom setup."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl CustomSetupBase {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "Details of the customer managed key associated with the workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomerManagedKeyDetails {
    #[doc = "The customer managed key status on the workspace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Details of the customer managed key associated with the workspace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<WorkspaceKeyDetails>,
    #[doc = "Key encryption key properties"]
    #[serde(rename = "kekIdentity", default, skip_serializing_if = "Option::is_none")]
    pub kek_identity: Option<KekIdentityProperties>,
}
impl CustomerManagedKeyDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the data lake storage account associated with the workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeStorageAccountDetails {
    #[doc = "Account URL"]
    #[serde(rename = "accountUrl", default, skip_serializing_if = "Option::is_none")]
    pub account_url: Option<String>,
    #[doc = "Filesystem name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filesystem: Option<String>,
    #[doc = "ARM resource Id of this storage account"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Create managed private endpoint to this storage account or not"]
    #[serde(rename = "createManagedPrivateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub create_managed_private_endpoint: Option<bool>,
}
impl DataLakeStorageAccountDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataMaskingPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a database data masking policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataMaskingPolicyProperties>,
    #[doc = "The location of the data masking policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The kind of data masking policy. Metadata, used for Azure portal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Fully qualified resource ID of the sql pool"]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
}
impl DataMaskingPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a database data masking policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataMaskingPolicyProperties {
    #[doc = "The state of the data masking policy."]
    #[serde(rename = "dataMaskingState")]
    pub data_masking_state: data_masking_policy_properties::DataMaskingState,
    #[doc = "The list of the exempt principals. Specifies the semicolon-separated list of database users for which the data masking policy does not apply. The specified users receive data results without masking for all of the database queries."]
    #[serde(rename = "exemptPrincipals", default, skip_serializing_if = "Option::is_none")]
    pub exempt_principals: Option<String>,
    #[doc = "The list of the application principals. This is a legacy parameter and is no longer used."]
    #[serde(rename = "applicationPrincipals", default, skip_serializing_if = "Option::is_none")]
    pub application_principals: Option<String>,
    #[doc = "The masking level. This is a legacy parameter and is no longer used."]
    #[serde(rename = "maskingLevel", default, skip_serializing_if = "Option::is_none")]
    pub masking_level: Option<String>,
}
impl DataMaskingPolicyProperties {
    pub fn new(data_masking_state: data_masking_policy_properties::DataMaskingState) -> Self {
        Self {
            data_masking_state,
            exempt_principals: None,
            application_principals: None,
            masking_level: None,
        }
    }
}
pub mod data_masking_policy_properties {
    use super::*;
    #[doc = "The state of the data masking policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataMaskingState {
        Disabled,
        Enabled,
    }
}
#[doc = "Represents a Sql pool data masking rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataMaskingRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a Sql pool data masking rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataMaskingRuleProperties>,
    #[doc = "The location of the data masking rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The kind of Data Masking Rule. Metadata, used for Azure portal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl DataMaskingRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list data masking rules request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataMaskingRuleListResult {
    #[doc = "The list of Sql pool data masking rules."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataMaskingRule>,
}
impl azure_core::Continuable for DataMaskingRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DataMaskingRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a Sql pool data masking rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataMaskingRuleProperties {
    #[doc = "The rule Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The alias name. This is a legacy parameter and is no longer used."]
    #[serde(rename = "aliasName", default, skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[doc = "The rule state. Used to delete a rule. To delete an existing rule, specify the schemaName, tableName, columnName, maskingFunction, and specify ruleState as disabled. However, if the rule doesn't already exist, the rule will be created with ruleState set to enabled, regardless of the provided value of ruleState."]
    #[serde(rename = "ruleState", default, skip_serializing_if = "Option::is_none")]
    pub rule_state: Option<data_masking_rule_properties::RuleState>,
    #[doc = "The schema name on which the data masking rule is applied."]
    #[serde(rename = "schemaName")]
    pub schema_name: String,
    #[doc = "The table name on which the data masking rule is applied."]
    #[serde(rename = "tableName")]
    pub table_name: String,
    #[doc = "The column name on which the data masking rule is applied."]
    #[serde(rename = "columnName")]
    pub column_name: String,
    #[doc = "The masking function that is used for the data masking rule."]
    #[serde(rename = "maskingFunction")]
    pub masking_function: data_masking_rule_properties::MaskingFunction,
    #[doc = "The numberFrom property of the masking rule. Required if maskingFunction is set to Number, otherwise this parameter will be ignored."]
    #[serde(rename = "numberFrom", default, skip_serializing_if = "Option::is_none")]
    pub number_from: Option<String>,
    #[doc = "The numberTo property of the data masking rule. Required if maskingFunction is set to Number, otherwise this parameter will be ignored."]
    #[serde(rename = "numberTo", default, skip_serializing_if = "Option::is_none")]
    pub number_to: Option<String>,
    #[doc = "If maskingFunction is set to Text, the number of characters to show unmasked in the beginning of the string. Otherwise, this parameter will be ignored."]
    #[serde(rename = "prefixSize", default, skip_serializing_if = "Option::is_none")]
    pub prefix_size: Option<String>,
    #[doc = "If maskingFunction is set to Text, the number of characters to show unmasked at the end of the string. Otherwise, this parameter will be ignored."]
    #[serde(rename = "suffixSize", default, skip_serializing_if = "Option::is_none")]
    pub suffix_size: Option<String>,
    #[doc = "If maskingFunction is set to Text, the character to use for masking the unexposed part of the string. Otherwise, this parameter will be ignored."]
    #[serde(rename = "replacementString", default, skip_serializing_if = "Option::is_none")]
    pub replacement_string: Option<String>,
}
impl DataMaskingRuleProperties {
    pub fn new(
        schema_name: String,
        table_name: String,
        column_name: String,
        masking_function: data_masking_rule_properties::MaskingFunction,
    ) -> Self {
        Self {
            id: None,
            alias_name: None,
            rule_state: None,
            schema_name,
            table_name,
            column_name,
            masking_function,
            number_from: None,
            number_to: None,
            prefix_size: None,
            suffix_size: None,
            replacement_string: None,
        }
    }
}
pub mod data_masking_rule_properties {
    use super::*;
    #[doc = "The rule state. Used to delete a rule. To delete an existing rule, specify the schemaName, tableName, columnName, maskingFunction, and specify ruleState as disabled. However, if the rule doesn't already exist, the rule will be created with ruleState set to enabled, regardless of the provided value of ruleState."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RuleState {
        Disabled,
        Enabled,
    }
    #[doc = "The masking function that is used for the data masking rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MaskingFunction {
        Default,
        #[serde(rename = "CCN")]
        Ccn,
        Email,
        Number,
        #[serde(rename = "SSN")]
        Ssn,
        Text,
    }
}
#[doc = "User activities of a data warehouse"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataWarehouseUserActivities {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "User activities of a data warehouse. This currently includes the count of running or suspended queries. For more information, please view the sys.dm_pdw_exec_requests dynamic management view (DMV)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataWarehouseUserActivitiesProperties>,
}
impl DataWarehouseUserActivities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User activities of a data warehouse. This currently includes the count of running or suspended queries. For more information, please view the sys.dm_pdw_exec_requests dynamic management view (DMV)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataWarehouseUserActivitiesProperties {
    #[doc = "Count of running and suspended queries."]
    #[serde(rename = "activeQueriesCount", default, skip_serializing_if = "Option::is_none")]
    pub active_queries_count: Option<i32>,
}
impl DataWarehouseUserActivitiesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dedicated Sql Minimal Tls Settings Info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedSqLminimalTlsSettings {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties of a dedicated sql minimal tls settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DedicatedSqLminimalTlsSettingsProperties>,
}
impl DedicatedSqLminimalTlsSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the server's dedicated sql minimal tls settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedSqLminimalTlsSettingsListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DedicatedSqLminimalTlsSettings>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DedicatedSqLminimalTlsSettingsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DedicatedSqLminimalTlsSettingsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dedicated SQL minimal tls settings patch info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedSqLminimalTlsSettingsPatchInfo {
    #[doc = "minimal tls version"]
    #[serde(rename = "minimalTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimal_tls_version: Option<String>,
}
impl DedicatedSqLminimalTlsSettingsPatchInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a dedicated sql minimal tls settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedSqLminimalTlsSettingsProperties {
    #[doc = "The minimal tls version of the sql server."]
    #[serde(rename = "minimalTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimal_tls_version: Option<String>,
}
impl DedicatedSqLminimalTlsSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dynamic Executor Allocation Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DynamicExecutorAllocation {
    #[doc = "Indicates whether Dynamic Executor Allocation is enabled or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The minimum number of executors alloted"]
    #[serde(rename = "minExecutors", default, skip_serializing_if = "Option::is_none")]
    pub min_executors: Option<i32>,
    #[doc = "The maximum number of executors alloted"]
    #[serde(rename = "maxExecutors", default, skip_serializing_if = "Option::is_none")]
    pub max_executors: Option<i32>,
}
impl DynamicExecutorAllocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the encryption associated with the workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionDetails {
    #[doc = "Double Encryption enabled"]
    #[serde(rename = "doubleEncryptionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub double_encryption_enabled: Option<bool>,
    #[doc = "Details of the customer managed key associated with the workspace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmk: Option<CustomerManagedKeyDetails>,
}
impl EncryptionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The server encryption protector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionProtector {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Kind of encryption protector. This is metadata used for the Azure portal experience."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties for an encryption protector execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EncryptionProtectorProperties>,
}
impl EncryptionProtector {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server encryption protectors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionProtectorListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EncryptionProtector>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EncryptionProtectorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EncryptionProtectorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for an encryption protector execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionProtectorProperties {
    #[doc = "Subregion of the encryption protector."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subregion: Option<String>,
    #[doc = "The name of the server key."]
    #[serde(rename = "serverKeyName", default, skip_serializing_if = "Option::is_none")]
    pub server_key_name: Option<String>,
    #[doc = "The encryption protector type like 'ServiceManaged', 'AzureKeyVault'."]
    #[serde(rename = "serverKeyType")]
    pub server_key_type: encryption_protector_properties::ServerKeyType,
    #[doc = "The URI of the server key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Thumbprint of the server key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}
impl EncryptionProtectorProperties {
    pub fn new(server_key_type: encryption_protector_properties::ServerKeyType) -> Self {
        Self {
            subregion: None,
            server_key_name: None,
            server_key_type,
            uri: None,
            thumbprint: None,
        }
    }
}
pub mod encryption_protector_properties {
    use super::*;
    #[doc = "The encryption protector type like 'ServiceManaged', 'AzureKeyVault'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServerKeyType")]
    pub enum ServerKeyType {
        ServiceManaged,
        AzureKeyVault,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServerKeyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServerKeyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServerKeyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ServiceManaged => serializer.serialize_unit_variant("ServerKeyType", 0u32, "ServiceManaged"),
                Self::AzureKeyVault => serializer.serialize_unit_variant("ServerKeyType", 1u32, "AzureKeyVault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The entity reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityReference {
    #[doc = "The type of this referenced entity."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<entity_reference::Type>,
    #[doc = "The name of this referenced entity."]
    #[serde(rename = "referenceName", default, skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<String>,
}
impl EntityReference {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod entity_reference {
    use super::*;
    #[doc = "The type of this referenced entity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        IntegrationRuntimeReference,
        LinkedServiceReference,
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
                Self::IntegrationRuntimeReference => serializer.serialize_unit_variant("Type", 0u32, "IntegrationRuntimeReference"),
                Self::LinkedServiceReference => serializer.serialize_unit_variant("Type", 1u32, "LinkedServiceReference"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The custom setup of setting environment variable."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentVariableSetup {
    #[serde(flatten)]
    pub custom_setup_base: CustomSetupBase,
    #[doc = "Environment variable custom setup type properties."]
    #[serde(rename = "typeProperties")]
    pub type_properties: EnvironmentVariableSetupTypeProperties,
}
impl EnvironmentVariableSetup {
    pub fn new(custom_setup_base: CustomSetupBase, type_properties: EnvironmentVariableSetupTypeProperties) -> Self {
        Self {
            custom_setup_base,
            type_properties,
        }
    }
}
#[doc = "Environment variable custom setup type properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentVariableSetupTypeProperties {
    #[doc = "The name of the environment variable."]
    #[serde(rename = "variableName")]
    pub variable_name: String,
    #[doc = "The value of the environment variable."]
    #[serde(rename = "variableValue")]
    pub variable_value: String,
}
impl EnvironmentVariableSetupTypeProperties {
    pub fn new(variable_name: String, variable_value: String) -> Self {
        Self {
            variable_name,
            variable_value,
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
#[doc = "An extended server blob auditing policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedServerBlobAuditingPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an extended server blob auditing policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExtendedServerBlobAuditingPolicyProperties>,
}
impl ExtendedServerBlobAuditingPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server extended auditing settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedServerBlobAuditingPolicyListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ExtendedServerBlobAuditingPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExtendedServerBlobAuditingPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ExtendedServerBlobAuditingPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an extended server blob auditing policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedServerBlobAuditingPolicyProperties {
    #[doc = "Specifies condition of where clause when creating an audit."]
    #[serde(rename = "predicateExpression", default, skip_serializing_if = "Option::is_none")]
    pub predicate_expression: Option<String>,
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    pub state: extended_server_blob_auditing_policy_properties::State,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled is required."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the auditing storage account. \r\nIf state is Enabled and storageEndpoint is specified, not specifying the storageAccountAccessKey will use SQL server system-assigned managed identity to access the storage.\r\nPrerequisites for using managed identity authentication:\r\n1. Assign SQL Server a system-assigned managed identity in Azure Active Directory (AAD).\r\n2. Grant SQL Server identity access to the storage account by adding 'Storage Blob Data Contributor' RBAC role to the server identity.\r\nFor more information, see [Auditing to storage using Managed Identity authentication](https://go.microsoft.com/fwlink/?linkid=2114355)"]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the number of days to keep in the audit logs in the storage account."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies the Actions-Groups and Actions to audit.\r\n\r\nThe recommended set of action groups to use is the following combination - this will audit all the queries and stored procedures executed against the database, as well as successful and failed logins:\r\n\r\nBATCH_COMPLETED_GROUP,\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP,\r\nFAILED_DATABASE_AUTHENTICATION_GROUP.\r\n\r\nThis above combination is also the set that is configured by default when enabling auditing from the Azure portal.\r\n\r\nThe supported action groups to audit are (note: choose only specific groups that cover your auditing needs. Using unnecessary groups could lead to very large quantities of audit records):\r\n\r\nAPPLICATION_ROLE_CHANGE_PASSWORD_GROUP\r\nBACKUP_RESTORE_GROUP\r\nDATABASE_LOGOUT_GROUP\r\nDATABASE_OBJECT_CHANGE_GROUP\r\nDATABASE_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_OBJECT_PERMISSION_CHANGE_GROUP\r\nDATABASE_OPERATION_GROUP\r\nDATABASE_PERMISSION_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_IMPERSONATION_GROUP\r\nDATABASE_ROLE_MEMBER_CHANGE_GROUP\r\nFAILED_DATABASE_AUTHENTICATION_GROUP\r\nSCHEMA_OBJECT_ACCESS_GROUP\r\nSCHEMA_OBJECT_CHANGE_GROUP\r\nSCHEMA_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nSCHEMA_OBJECT_PERMISSION_CHANGE_GROUP\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP\r\nUSER_CHANGE_PASSWORD_GROUP\r\nBATCH_STARTED_GROUP\r\nBATCH_COMPLETED_GROUP\r\n\r\nThese are groups that cover all sql statements and stored procedures executed against the database, and should not be used in combination with other groups as this will result in duplicate audit logs.\r\n\r\nFor more information, see [Database-Level Audit Action Groups](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-action-groups).\r\n\r\nFor Database auditing policy, specific Actions can also be specified (note that Actions cannot be specified for Server auditing policy). The supported actions to audit are:\r\nSELECT\r\nUPDATE\r\nINSERT\r\nDELETE\r\nEXECUTE\r\nRECEIVE\r\nREFERENCES\r\n\r\nThe general form for defining an action to be audited is:\r\n{action} ON {object} BY {principal}\r\n\r\nNote that <object> in the above format can refer to an object like a table, view, or stored procedure, or an entire database or schema. For the latter cases, the forms DATABASE::{db_name} and SCHEMA::{schema_name} are used, respectively.\r\n\r\nFor example:\r\nSELECT on dbo.myTable by public\r\nSELECT on DATABASE::myDatabase by public\r\nSELECT on SCHEMA::mySchema by public\r\n\r\nFor more information, see [Database-Level Audit Actions](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-actions)"]
    #[serde(rename = "auditActionsAndGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub audit_actions_and_groups: Vec<String>,
    #[doc = "Specifies the blob storage subscription Id."]
    #[serde(rename = "storageAccountSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_subscription_id: Option<String>,
    #[doc = "Specifies whether storageAccountAccessKey value is the storage's secondary key."]
    #[serde(rename = "isStorageSecondaryKeyInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_storage_secondary_key_in_use: Option<bool>,
    #[doc = "Specifies whether audit events are sent to Azure Monitor. \r\nIn order to send the events to Azure Monitor, specify 'state' as 'Enabled' and 'isAzureMonitorTargetEnabled' as true.\r\n\r\nWhen using REST API to configure auditing, Diagnostic Settings with 'SQLSecurityAuditEvents' diagnostic logs category on the database should be also created.\r\nNote that for server level audit you should use the 'master' database as {databaseName}.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/{databaseName}/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isAzureMonitorTargetEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_monitor_target_enabled: Option<bool>,
    #[doc = "Specifies the amount of time in milliseconds that can elapse before audit actions are forced to be processed.\r\nThe default minimum value is 1000 (1 second). The maximum is 2,147,483,647."]
    #[serde(rename = "queueDelayMs", default, skip_serializing_if = "Option::is_none")]
    pub queue_delay_ms: Option<i32>,
    #[doc = "Specifies the state of devops audit. If state is Enabled, devops logs will be sent to Azure Monitor.\r\nIn order to send the events to Azure Monitor, specify 'State' as 'Enabled', 'IsAzureMonitorTargetEnabled' as true and 'IsDevopsAuditEnabled' as true\r\n\r\nWhen using REST API to configure auditing, Diagnostic Settings with 'DevOpsOperationsAudit' diagnostic logs category on the master database should also be created.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/master/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isDevopsAuditEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_devops_audit_enabled: Option<bool>,
}
impl ExtendedServerBlobAuditingPolicyProperties {
    pub fn new(state: extended_server_blob_auditing_policy_properties::State) -> Self {
        Self {
            predicate_expression: None,
            state,
            storage_endpoint: None,
            storage_account_access_key: None,
            retention_days: None,
            audit_actions_and_groups: Vec::new(),
            storage_account_subscription_id: None,
            is_storage_secondary_key_in_use: None,
            is_azure_monitor_target_enabled: None,
            queue_delay_ms: None,
            is_devops_audit_enabled: None,
        }
    }
}
pub mod extended_server_blob_auditing_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "An extended Sql pool blob auditing policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedSqlPoolBlobAuditingPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an extended Sql pool blob auditing policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExtendedSqlPoolBlobAuditingPolicyProperties>,
}
impl ExtendedSqlPoolBlobAuditingPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of sql pool extended auditing settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedSqlPoolBlobAuditingPolicyListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ExtendedSqlPoolBlobAuditingPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExtendedSqlPoolBlobAuditingPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ExtendedSqlPoolBlobAuditingPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an extended Sql pool blob auditing policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedSqlPoolBlobAuditingPolicyProperties {
    #[doc = "Specifies condition of where clause when creating an audit."]
    #[serde(rename = "predicateExpression", default, skip_serializing_if = "Option::is_none")]
    pub predicate_expression: Option<String>,
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    pub state: extended_sql_pool_blob_auditing_policy_properties::State,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled is required."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the auditing storage account. \r\nIf state is Enabled and storageEndpoint is specified, not specifying the storageAccountAccessKey will use SQL server system-assigned managed identity to access the storage.\r\nPrerequisites for using managed identity authentication:\r\n1. Assign SQL Server a system-assigned managed identity in Azure Active Directory (AAD).\r\n2. Grant SQL Server identity access to the storage account by adding 'Storage Blob Data Contributor' RBAC role to the server identity.\r\nFor more information, see [Auditing to storage using Managed Identity authentication](https://go.microsoft.com/fwlink/?linkid=2114355)"]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the number of days to keep in the audit logs in the storage account."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies the Actions-Groups and Actions to audit.\r\n\r\nThe recommended set of action groups to use is the following combination - this will audit all the queries and stored procedures executed against the database, as well as successful and failed logins:\r\n\r\nBATCH_COMPLETED_GROUP,\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP,\r\nFAILED_DATABASE_AUTHENTICATION_GROUP.\r\n\r\nThis above combination is also the set that is configured by default when enabling auditing from the Azure portal.\r\n\r\nThe supported action groups to audit are (note: choose only specific groups that cover your auditing needs. Using unnecessary groups could lead to very large quantities of audit records):\r\n\r\nAPPLICATION_ROLE_CHANGE_PASSWORD_GROUP\r\nBACKUP_RESTORE_GROUP\r\nDATABASE_LOGOUT_GROUP\r\nDATABASE_OBJECT_CHANGE_GROUP\r\nDATABASE_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_OBJECT_PERMISSION_CHANGE_GROUP\r\nDATABASE_OPERATION_GROUP\r\nDATABASE_PERMISSION_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_IMPERSONATION_GROUP\r\nDATABASE_ROLE_MEMBER_CHANGE_GROUP\r\nFAILED_DATABASE_AUTHENTICATION_GROUP\r\nSCHEMA_OBJECT_ACCESS_GROUP\r\nSCHEMA_OBJECT_CHANGE_GROUP\r\nSCHEMA_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nSCHEMA_OBJECT_PERMISSION_CHANGE_GROUP\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP\r\nUSER_CHANGE_PASSWORD_GROUP\r\nBATCH_STARTED_GROUP\r\nBATCH_COMPLETED_GROUP\r\n\r\nThese are groups that cover all sql statements and stored procedures executed against the database, and should not be used in combination with other groups as this will result in duplicate audit logs.\r\n\r\nFor more information, see [Database-Level Audit Action Groups](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-action-groups).\r\n\r\nFor Database auditing policy, specific Actions can also be specified (note that Actions cannot be specified for Server auditing policy). The supported actions to audit are:\r\nSELECT\r\nUPDATE\r\nINSERT\r\nDELETE\r\nEXECUTE\r\nRECEIVE\r\nREFERENCES\r\n\r\nThe general form for defining an action to be audited is:\r\n{action} ON {object} BY {principal}\r\n\r\nNote that <object> in the above format can refer to an object like a table, view, or stored procedure, or an entire database or schema. For the latter cases, the forms DATABASE::{db_name} and SCHEMA::{schema_name} are used, respectively.\r\n\r\nFor example:\r\nSELECT on dbo.myTable by public\r\nSELECT on DATABASE::myDatabase by public\r\nSELECT on SCHEMA::mySchema by public\r\n\r\nFor more information, see [Database-Level Audit Actions](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-actions)"]
    #[serde(rename = "auditActionsAndGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub audit_actions_and_groups: Vec<String>,
    #[doc = "Specifies the blob storage subscription Id."]
    #[serde(rename = "storageAccountSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_subscription_id: Option<String>,
    #[doc = "Specifies whether storageAccountAccessKey value is the storage's secondary key."]
    #[serde(rename = "isStorageSecondaryKeyInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_storage_secondary_key_in_use: Option<bool>,
    #[doc = "Specifies whether audit events are sent to Azure Monitor. \r\nIn order to send the events to Azure Monitor, specify 'state' as 'Enabled' and 'isAzureMonitorTargetEnabled' as true.\r\n\r\nWhen using REST API to configure auditing, Diagnostic Settings with 'SQLSecurityAuditEvents' diagnostic logs category on the database should be also created.\r\nNote that for server level audit you should use the 'master' database as {databaseName}.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/{databaseName}/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isAzureMonitorTargetEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_monitor_target_enabled: Option<bool>,
    #[doc = "Specifies the amount of time in milliseconds that can elapse before audit actions are forced to be processed.\r\nThe default minimum value is 1000 (1 second). The maximum is 2,147,483,647."]
    #[serde(rename = "queueDelayMs", default, skip_serializing_if = "Option::is_none")]
    pub queue_delay_ms: Option<i32>,
}
impl ExtendedSqlPoolBlobAuditingPolicyProperties {
    pub fn new(state: extended_sql_pool_blob_auditing_policy_properties::State) -> Self {
        Self {
            predicate_expression: None,
            state,
            storage_endpoint: None,
            storage_account_access_key: None,
            retention_days: None,
            audit_actions_and_groups: Vec::new(),
            storage_account_subscription_id: None,
            is_storage_secondary_key_in_use: None,
            is_azure_monitor_target_enabled: None,
            queue_delay_ms: None,
        }
    }
}
pub mod extended_sql_pool_blob_auditing_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "A database geo backup policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoBackupPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of the geo backup policy."]
    pub properties: GeoBackupPolicyProperties,
    #[doc = "Kind of geo backup policy.  This is metadata used for the Azure portal experience."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Backup policy location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl GeoBackupPolicy {
    pub fn new(properties: GeoBackupPolicyProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            kind: None,
            location: None,
        }
    }
}
#[doc = "The response to a list geo backup policies request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GeoBackupPolicyListResult {
    #[doc = "The list of geo backup policies."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GeoBackupPolicy>,
}
impl azure_core::Continuable for GeoBackupPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl GeoBackupPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the geo backup policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoBackupPolicyProperties {
    #[doc = "The state of the geo backup policy."]
    pub state: geo_backup_policy_properties::State,
    #[doc = "The storage type of the geo backup policy."]
    #[serde(rename = "storageType", default, skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}
impl GeoBackupPolicyProperties {
    pub fn new(state: geo_backup_policy_properties::State) -> Self {
        Self { state, storage_type: None }
    }
}
pub mod geo_backup_policy_properties {
    use super::*;
    #[doc = "The state of the geo backup policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Disabled,
        Enabled,
    }
}
#[doc = "The request payload of get SSIS object metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetSsisObjectMetadataRequest {
    #[doc = "Metadata path."]
    #[serde(rename = "metadataPath", default, skip_serializing_if = "Option::is_none")]
    pub metadata_path: Option<String>,
}
impl GetSsisObjectMetadataRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Synapse nested object which serves as a compute resource for activities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntime {
    #[doc = "The type of integration runtime."]
    #[serde(rename = "type")]
    pub type_: IntegrationRuntimeType,
    #[doc = "Integration runtime description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl IntegrationRuntime {
    pub fn new(type_: IntegrationRuntimeType) -> Self {
        Self { type_, description: None }
    }
}
#[doc = "The integration runtime authentication keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeAuthKeys {
    #[doc = "The primary integration runtime authentication key."]
    #[serde(rename = "authKey1", default, skip_serializing_if = "Option::is_none")]
    pub auth_key1: Option<String>,
    #[doc = "The secondary integration runtime authentication key."]
    #[serde(rename = "authKey2", default, skip_serializing_if = "Option::is_none")]
    pub auth_key2: Option<String>,
}
impl IntegrationRuntimeAuthKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The state of integration runtime auto update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IntegrationRuntimeAutoUpdate")]
pub enum IntegrationRuntimeAutoUpdate {
    On,
    Off,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IntegrationRuntimeAutoUpdate {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IntegrationRuntimeAutoUpdate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IntegrationRuntimeAutoUpdate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::On => serializer.serialize_unit_variant("IntegrationRuntimeAutoUpdate", 0u32, "On"),
            Self::Off => serializer.serialize_unit_variant("IntegrationRuntimeAutoUpdate", 1u32, "Off"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The compute resource properties for managed integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeComputeProperties {
    #[doc = "The location for managed integration runtime. The supported regions could be found on https://docs.microsoft.com/en-us/azure/data-factory/data-factory-data-movement-activities"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The node size requirement to managed integration runtime."]
    #[serde(rename = "nodeSize", default, skip_serializing_if = "Option::is_none")]
    pub node_size: Option<String>,
    #[doc = "The required number of nodes for managed integration runtime."]
    #[serde(rename = "numberOfNodes", default, skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,
    #[doc = "Maximum parallel executions count per node for managed integration runtime."]
    #[serde(rename = "maxParallelExecutionsPerNode", default, skip_serializing_if = "Option::is_none")]
    pub max_parallel_executions_per_node: Option<i32>,
    #[doc = "Data flow properties for managed integration runtime."]
    #[serde(rename = "dataFlowProperties", default, skip_serializing_if = "Option::is_none")]
    pub data_flow_properties: Option<IntegrationRuntimeDataFlowProperties>,
    #[doc = "VNet properties for managed integration runtime."]
    #[serde(rename = "vNetProperties", default, skip_serializing_if = "Option::is_none")]
    pub v_net_properties: Option<IntegrationRuntimeVNetProperties>,
}
impl IntegrationRuntimeComputeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Connection information for encrypting the on-premises data source credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeConnectionInfo {
    #[doc = "The token generated in service. Callers use this token to authenticate to integration runtime."]
    #[serde(rename = "serviceToken", default, skip_serializing_if = "Option::is_none")]
    pub service_token: Option<String>,
    #[doc = "The integration runtime SSL certificate thumbprint. Click-Once application uses it to do server validation."]
    #[serde(rename = "identityCertThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub identity_cert_thumbprint: Option<String>,
    #[doc = "The on-premises integration runtime host URL."]
    #[serde(rename = "hostServiceUri", default, skip_serializing_if = "Option::is_none")]
    pub host_service_uri: Option<String>,
    #[doc = "The integration runtime version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The public key for encrypting a credential when transferring the credential to the integration runtime."]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[doc = "Whether the identity certificate is expired."]
    #[serde(rename = "isIdentityCertExprired", default, skip_serializing_if = "Option::is_none")]
    pub is_identity_cert_exprired: Option<bool>,
}
impl IntegrationRuntimeConnectionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom setup script properties for a managed dedicated integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeCustomSetupScriptProperties {
    #[doc = "The URI of the Azure blob container that contains the custom setup script."]
    #[serde(rename = "blobContainerUri", default, skip_serializing_if = "Option::is_none")]
    pub blob_container_uri: Option<String>,
    #[doc = "Azure Synapse secure string definition. The string value will be masked with asterisks '*' during Get or List API calls."]
    #[serde(rename = "sasToken", default, skip_serializing_if = "Option::is_none")]
    pub sas_token: Option<SecureString>,
}
impl IntegrationRuntimeCustomSetupScriptProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data flow properties for managed integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeDataFlowProperties {
    #[doc = "Compute type of the cluster which will execute data flow job."]
    #[serde(rename = "computeType", default, skip_serializing_if = "Option::is_none")]
    pub compute_type: Option<integration_runtime_data_flow_properties::ComputeType>,
    #[doc = "Core count of the cluster which will execute data flow job. Supported values are: 8, 16, 32, 48, 80, 144 and 272."]
    #[serde(rename = "coreCount", default, skip_serializing_if = "Option::is_none")]
    pub core_count: Option<i32>,
    #[doc = "Time to live (in minutes) setting of the cluster which will execute data flow job."]
    #[serde(rename = "timeToLive", default, skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i32>,
    #[doc = "Cluster will not be recycled and it will be used in next data flow activity run until TTL (time to live) is reached if this is set as false. Default is true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cleanup: Option<bool>,
}
impl IntegrationRuntimeDataFlowProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod integration_runtime_data_flow_properties {
    use super::*;
    #[doc = "Compute type of the cluster which will execute data flow job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ComputeType")]
    pub enum ComputeType {
        General,
        MemoryOptimized,
        ComputeOptimized,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ComputeType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ComputeType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ComputeType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::General => serializer.serialize_unit_variant("ComputeType", 0u32, "General"),
                Self::MemoryOptimized => serializer.serialize_unit_variant("ComputeType", 1u32, "MemoryOptimized"),
                Self::ComputeOptimized => serializer.serialize_unit_variant("ComputeType", 2u32, "ComputeOptimized"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Data proxy properties for a managed dedicated integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeDataProxyProperties {
    #[doc = "The entity reference."]
    #[serde(rename = "connectVia", default, skip_serializing_if = "Option::is_none")]
    pub connect_via: Option<EntityReference>,
    #[doc = "The entity reference."]
    #[serde(rename = "stagingLinkedService", default, skip_serializing_if = "Option::is_none")]
    pub staging_linked_service: Option<EntityReference>,
    #[doc = "The path to contain the staged data in the Blob storage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl IntegrationRuntimeDataProxyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of integration runtime resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeListResponse {
    #[doc = "List of integration runtimes."]
    pub value: Vec<IntegrationRuntimeResource>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IntegrationRuntimeListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IntegrationRuntimeListResponse {
    pub fn new(value: Vec<IntegrationRuntimeResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Get monitoring data response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeMonitoringData {
    #[doc = "Integration runtime name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Integration runtime node monitoring data."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<IntegrationRuntimeNodeMonitoringData>,
}
impl IntegrationRuntimeMonitoringData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The IP address of self-hosted integration runtime node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeNodeIpAddress {
    #[doc = "The IP address of self-hosted integration runtime node."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
impl IntegrationRuntimeNodeIpAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Monitoring data for integration runtime node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeNodeMonitoringData {
    #[doc = "Name of the integration runtime node."]
    #[serde(rename = "nodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[doc = "Available memory (MB) on the integration runtime node."]
    #[serde(rename = "availableMemoryInMB", default, skip_serializing_if = "Option::is_none")]
    pub available_memory_in_mb: Option<i32>,
    #[doc = "CPU percentage on the integration runtime node."]
    #[serde(rename = "cpuUtilization", default, skip_serializing_if = "Option::is_none")]
    pub cpu_utilization: Option<i32>,
    #[doc = "Maximum concurrent jobs on the integration runtime node."]
    #[serde(rename = "concurrentJobsLimit", default, skip_serializing_if = "Option::is_none")]
    pub concurrent_jobs_limit: Option<i32>,
    #[doc = "The number of jobs currently running on the integration runtime node."]
    #[serde(rename = "concurrentJobsRunning", default, skip_serializing_if = "Option::is_none")]
    pub concurrent_jobs_running: Option<i32>,
    #[doc = "The maximum concurrent jobs in this integration runtime."]
    #[serde(rename = "maxConcurrentJobs", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_jobs: Option<i32>,
    #[doc = "Sent bytes on the integration runtime node."]
    #[serde(rename = "sentBytes", default, skip_serializing_if = "Option::is_none")]
    pub sent_bytes: Option<f64>,
    #[doc = "Received bytes on the integration runtime node."]
    #[serde(rename = "receivedBytes", default, skip_serializing_if = "Option::is_none")]
    pub received_bytes: Option<f64>,
}
impl IntegrationRuntimeNodeMonitoringData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure-SSIS integration runtime outbound network dependency endpoints for one category."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeOutboundNetworkDependenciesCategoryEndpoint {
    #[doc = "The category of outbound network dependency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The endpoints for outbound network dependency."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<IntegrationRuntimeOutboundNetworkDependenciesEndpoint>,
}
impl IntegrationRuntimeOutboundNetworkDependenciesCategoryEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The endpoint for Azure-SSIS integration runtime outbound network dependency."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeOutboundNetworkDependenciesEndpoint {
    #[doc = "The domain name of endpoint."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "The details of endpoint."]
    #[serde(rename = "endpointDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint_details: Vec<IntegrationRuntimeOutboundNetworkDependenciesEndpointDetails>,
}
impl IntegrationRuntimeOutboundNetworkDependenciesEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of Azure-SSIS integration runtime outbound network dependency endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeOutboundNetworkDependenciesEndpointDetails {
    #[doc = "The port of endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
impl IntegrationRuntimeOutboundNetworkDependenciesEndpointDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure-SSIS integration runtime outbound network dependency endpoints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeOutboundNetworkDependenciesEndpointsResponse {
    #[doc = "The list of outbound network dependency endpoints."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IntegrationRuntimeOutboundNetworkDependenciesCategoryEndpoint>,
}
impl IntegrationRuntimeOutboundNetworkDependenciesEndpointsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters to regenerate the authentication key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeRegenerateKeyParameters {
    #[doc = "The name of the authentication key to regenerate."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<integration_runtime_regenerate_key_parameters::KeyName>,
}
impl IntegrationRuntimeRegenerateKeyParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod integration_runtime_regenerate_key_parameters {
    use super::*;
    #[doc = "The name of the authentication key to regenerate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KeyName")]
    pub enum KeyName {
        #[serde(rename = "authKey1")]
        AuthKey1,
        #[serde(rename = "authKey2")]
        AuthKey2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KeyName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KeyName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KeyName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AuthKey1 => serializer.serialize_unit_variant("KeyName", 0u32, "authKey1"),
                Self::AuthKey2 => serializer.serialize_unit_variant("KeyName", 1u32, "authKey2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Integration runtime resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeResource {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Azure Synapse nested object which serves as a compute resource for activities."]
    pub properties: IntegrationRuntime,
}
impl IntegrationRuntimeResource {
    pub fn new(properties: IntegrationRuntime) -> Self {
        Self {
            sub_resource: SubResource::default(),
            properties,
        }
    }
}
#[doc = "Catalog information for managed dedicated integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeSsisCatalogInfo {
    #[doc = "The catalog database server URL."]
    #[serde(rename = "catalogServerEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub catalog_server_endpoint: Option<String>,
    #[doc = "The administrator user name of catalog database."]
    #[serde(rename = "catalogAdminUserName", default, skip_serializing_if = "Option::is_none")]
    pub catalog_admin_user_name: Option<String>,
    #[doc = "Azure Synapse secure string definition. The string value will be masked with asterisks '*' during Get or List API calls."]
    #[serde(rename = "catalogAdminPassword", default, skip_serializing_if = "Option::is_none")]
    pub catalog_admin_password: Option<SecureString>,
    #[doc = "The pricing tier for the catalog database. The valid values could be found in https://azure.microsoft.com/en-us/pricing/details/sql-database/"]
    #[serde(rename = "catalogPricingTier", default, skip_serializing_if = "Option::is_none")]
    pub catalog_pricing_tier: Option<integration_runtime_ssis_catalog_info::CatalogPricingTier>,
}
impl IntegrationRuntimeSsisCatalogInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod integration_runtime_ssis_catalog_info {
    use super::*;
    #[doc = "The pricing tier for the catalog database. The valid values could be found in https://azure.microsoft.com/en-us/pricing/details/sql-database/"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CatalogPricingTier")]
    pub enum CatalogPricingTier {
        Basic,
        Standard,
        Premium,
        #[serde(rename = "PremiumRS")]
        PremiumRs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CatalogPricingTier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CatalogPricingTier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CatalogPricingTier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("CatalogPricingTier", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("CatalogPricingTier", 1u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("CatalogPricingTier", 2u32, "Premium"),
                Self::PremiumRs => serializer.serialize_unit_variant("CatalogPricingTier", 3u32, "PremiumRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "SSIS properties for managed integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeSsisProperties {
    #[doc = "Catalog information for managed dedicated integration runtime."]
    #[serde(rename = "catalogInfo", default, skip_serializing_if = "Option::is_none")]
    pub catalog_info: Option<IntegrationRuntimeSsisCatalogInfo>,
    #[doc = "License type for bringing your own license scenario."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<integration_runtime_ssis_properties::LicenseType>,
    #[doc = "Custom setup script properties for a managed dedicated integration runtime."]
    #[serde(rename = "customSetupScriptProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_setup_script_properties: Option<IntegrationRuntimeCustomSetupScriptProperties>,
    #[doc = "Data proxy properties for a managed dedicated integration runtime."]
    #[serde(rename = "dataProxyProperties", default, skip_serializing_if = "Option::is_none")]
    pub data_proxy_properties: Option<IntegrationRuntimeDataProxyProperties>,
    #[doc = "The edition for the SSIS Integration Runtime"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<integration_runtime_ssis_properties::Edition>,
    #[doc = "Custom setup without script properties for a SSIS integration runtime."]
    #[serde(rename = "expressCustomSetupProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub express_custom_setup_properties: Vec<CustomSetupBase>,
}
impl IntegrationRuntimeSsisProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod integration_runtime_ssis_properties {
    use super::*;
    #[doc = "License type for bringing your own license scenario."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        BasePrice,
        LicenseIncluded,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BasePrice => serializer.serialize_unit_variant("LicenseType", 0u32, "BasePrice"),
                Self::LicenseIncluded => serializer.serialize_unit_variant("LicenseType", 1u32, "LicenseIncluded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The edition for the SSIS Integration Runtime"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Edition")]
    pub enum Edition {
        Standard,
        Enterprise,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Edition {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Edition {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Edition {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("Edition", 0u32, "Standard"),
                Self::Enterprise => serializer.serialize_unit_variant("Edition", 1u32, "Enterprise"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The state of integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IntegrationRuntimeState")]
pub enum IntegrationRuntimeState {
    Initial,
    Stopped,
    Started,
    Starting,
    Stopping,
    NeedRegistration,
    Online,
    Limited,
    Offline,
    AccessDenied,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IntegrationRuntimeState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IntegrationRuntimeState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IntegrationRuntimeState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Initial => serializer.serialize_unit_variant("IntegrationRuntimeState", 0u32, "Initial"),
            Self::Stopped => serializer.serialize_unit_variant("IntegrationRuntimeState", 1u32, "Stopped"),
            Self::Started => serializer.serialize_unit_variant("IntegrationRuntimeState", 2u32, "Started"),
            Self::Starting => serializer.serialize_unit_variant("IntegrationRuntimeState", 3u32, "Starting"),
            Self::Stopping => serializer.serialize_unit_variant("IntegrationRuntimeState", 4u32, "Stopping"),
            Self::NeedRegistration => serializer.serialize_unit_variant("IntegrationRuntimeState", 5u32, "NeedRegistration"),
            Self::Online => serializer.serialize_unit_variant("IntegrationRuntimeState", 6u32, "Online"),
            Self::Limited => serializer.serialize_unit_variant("IntegrationRuntimeState", 7u32, "Limited"),
            Self::Offline => serializer.serialize_unit_variant("IntegrationRuntimeState", 8u32, "Offline"),
            Self::AccessDenied => serializer.serialize_unit_variant("IntegrationRuntimeState", 9u32, "AccessDenied"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Integration runtime status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeStatus {
    #[doc = "The type of integration runtime."]
    #[serde(rename = "type")]
    pub type_: IntegrationRuntimeType,
    #[doc = "The workspace name which the integration runtime belong to."]
    #[serde(rename = "dataFactoryName", default, skip_serializing_if = "Option::is_none")]
    pub data_factory_name: Option<String>,
    #[doc = "The state of integration runtime."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<IntegrationRuntimeState>,
}
impl IntegrationRuntimeStatus {
    pub fn new(type_: IntegrationRuntimeType) -> Self {
        Self {
            type_,
            data_factory_name: None,
            state: None,
        }
    }
}
#[doc = "Integration runtime status response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeStatusResponse {
    #[doc = "The integration runtime name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Integration runtime status."]
    pub properties: IntegrationRuntimeStatus,
}
impl IntegrationRuntimeStatusResponse {
    pub fn new(properties: IntegrationRuntimeStatus) -> Self {
        Self { name: None, properties }
    }
}
#[doc = "The type of integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IntegrationRuntimeType")]
pub enum IntegrationRuntimeType {
    Managed,
    SelfHosted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IntegrationRuntimeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IntegrationRuntimeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IntegrationRuntimeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Managed => serializer.serialize_unit_variant("IntegrationRuntimeType", 0u32, "Managed"),
            Self::SelfHosted => serializer.serialize_unit_variant("IntegrationRuntimeType", 1u32, "SelfHosted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "VNet properties for managed integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeVNetProperties {
    #[doc = "The ID of the VNet that this integration runtime will join."]
    #[serde(rename = "vNetId", default, skip_serializing_if = "Option::is_none")]
    pub v_net_id: Option<String>,
    #[doc = "The name of the subnet this integration runtime will join."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[doc = "Resource IDs of the public IP addresses that this integration runtime will use."]
    #[serde(rename = "publicIPs", default, skip_serializing_if = "Vec::is_empty")]
    pub public_i_ps: Vec<String>,
    #[doc = "The ID of subnet, to which this Azure-SSIS integration runtime will be joined."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}
impl IntegrationRuntimeVNetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IP firewall rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpFirewallRuleInfo {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "IP firewall rule properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IpFirewallRuleProperties>,
}
impl IpFirewallRuleInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of IP firewall rules"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpFirewallRuleInfoListResult {
    #[doc = "Link to next page of results"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of IP firewall rules"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IpFirewallRuleInfo>,
}
impl azure_core::Continuable for IpFirewallRuleInfoListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IpFirewallRuleInfoListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IP firewall rule properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpFirewallRuleProperties {
    #[doc = "The end IP address of the firewall rule. Must be IPv4 format. Must be greater than or equal to startIpAddress"]
    #[serde(rename = "endIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub end_ip_address: Option<String>,
    #[doc = "Resource provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ip_firewall_rule_properties::ProvisioningState>,
    #[doc = "The start IP address of the firewall rule. Must be IPv4 format"]
    #[serde(rename = "startIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub start_ip_address: Option<String>,
}
impl IpFirewallRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ip_firewall_rule_properties {
    use super::*;
    #[doc = "Resource provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Provisioning,
        Succeeded,
        Deleting,
        Failed,
        DeleteError,
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
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Provisioning"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::DeleteError => serializer.serialize_unit_variant("ProvisioningState", 4u32, "DeleteError"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Key encryption key properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KekIdentityProperties {
    #[doc = "User assigned identity resource Id"]
    #[serde(rename = "userAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identity: Option<String>,
    #[doc = "Boolean specifying whether to use system assigned identity or not"]
    #[serde(rename = "useSystemAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub use_system_assigned_identity: Option<serde_json::Value>,
}
impl KekIdentityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A workspace key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Key {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Key properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<KeyProperties>,
}
impl Key {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of keys"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyInfoListResult {
    #[doc = "Link to the next page of results"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of keys"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Key>,
}
impl azure_core::Continuable for KeyInfoListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl KeyInfoListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Key properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyProperties {
    #[doc = "Used to activate the workspace after a customer managed key is provided."]
    #[serde(rename = "isActiveCMK", default, skip_serializing_if = "Option::is_none")]
    pub is_active_cmk: Option<bool>,
    #[doc = "The Key Vault Url of the workspace key."]
    #[serde(rename = "keyVaultUrl", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_url: Option<String>,
}
impl KeyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Library/package information of a Big Data pool powered by Apache Spark"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LibraryInfo {
    #[doc = "Name of the library."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Storage blob path of library."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Storage blob container name."]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "The last update time of the library."]
    #[serde(rename = "uploadedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub uploaded_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Type of the library."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Provisioning status of the library/package."]
    #[serde(rename = "provisioningStatus", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status: Option<String>,
    #[doc = "Creator Id of the library/package."]
    #[serde(rename = "creatorId", default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
}
impl LibraryInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of Library resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LibraryListResponse {
    #[doc = "List of Library."]
    pub value: Vec<LibraryResource>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LibraryListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl LibraryListResponse {
    pub fn new(value: Vec<LibraryResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Library requirements for a Big Data pool powered by Apache Spark"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LibraryRequirements {
    #[doc = "The last update time of the library requirements file."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
    #[doc = "The library requirements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "The filename of the library requirements file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}
impl LibraryRequirements {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Library response details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LibraryResource {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Library/package information of a Big Data pool powered by Apache Spark"]
    pub properties: LibraryInfo,
}
impl LibraryResource {
    pub fn new(properties: LibraryInfo) -> Self {
        Self {
            sub_resource: SubResource::default(),
            properties,
        }
    }
}
#[doc = "Installation of licensed component setup type properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicensedComponentSetupTypeProperties {
    #[doc = "The name of the 3rd party component."]
    #[serde(rename = "componentName")]
    pub component_name: String,
    #[doc = "The base definition of a secret type."]
    #[serde(rename = "licenseKey", default, skip_serializing_if = "Option::is_none")]
    pub license_key: Option<SecretBase>,
}
impl LicensedComponentSetupTypeProperties {
    pub fn new(component_name: String) -> Self {
        Self {
            component_name,
            license_key: None,
        }
    }
}
#[doc = "The linked integration runtime information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedIntegrationRuntime {
    #[doc = "The name of the linked integration runtime."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The subscription ID for which the linked integration runtime belong to."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The name of the workspace for which the linked integration runtime belong to."]
    #[serde(rename = "dataFactoryName", default, skip_serializing_if = "Option::is_none")]
    pub data_factory_name: Option<String>,
    #[doc = "The location of the workspace for which the linked integration runtime belong to."]
    #[serde(rename = "dataFactoryLocation", default, skip_serializing_if = "Option::is_none")]
    pub data_factory_location: Option<String>,
    #[doc = "The creating time of the linked integration runtime."]
    #[serde(rename = "createTime", with = "azure_core::date::rfc3339::option")]
    pub create_time: Option<time::OffsetDateTime>,
}
impl LinkedIntegrationRuntime {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The key authorization type integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedIntegrationRuntimeKeyAuthorization {
    #[serde(flatten)]
    pub linked_integration_runtime_type: LinkedIntegrationRuntimeType,
    #[doc = "Azure Synapse secure string definition. The string value will be masked with asterisks '*' during Get or List API calls."]
    pub key: SecureString,
}
impl LinkedIntegrationRuntimeKeyAuthorization {
    pub fn new(linked_integration_runtime_type: LinkedIntegrationRuntimeType, key: SecureString) -> Self {
        Self {
            linked_integration_runtime_type,
            key,
        }
    }
}
#[doc = "The role based access control (RBAC) authorization type integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedIntegrationRuntimeRbacAuthorization {
    #[serde(flatten)]
    pub linked_integration_runtime_type: LinkedIntegrationRuntimeType,
    #[doc = "The resource identifier of the integration runtime to be shared."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
}
impl LinkedIntegrationRuntimeRbacAuthorization {
    pub fn new(linked_integration_runtime_type: LinkedIntegrationRuntimeType, resource_id: String) -> Self {
        Self {
            linked_integration_runtime_type,
            resource_id,
        }
    }
}
#[doc = "The base definition of a linked integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedIntegrationRuntimeType {
    #[doc = "The authorization type for integration runtime sharing."]
    #[serde(rename = "authorizationType")]
    pub authorization_type: String,
}
impl LinkedIntegrationRuntimeType {
    pub fn new(authorization_type: String) -> Self {
        Self { authorization_type }
    }
}
#[doc = "A list of SQL pool security alert policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListSqlPoolSecurityAlertPolicies {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlPoolSecurityAlertPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListSqlPoolSecurityAlertPolicies {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListSqlPoolSecurityAlertPolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Maintenance window options."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceWindowOptions {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Maintenance window options properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MaintenanceWindowOptionsProperties>,
}
impl MaintenanceWindowOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Maintenance window options properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceWindowOptionsProperties {
    #[doc = "Whether maintenance windows are enabled for the database."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Available maintenance cycles e.g. {Saturday, 0, 48*60}, {Wednesday, 0, 24*60}."]
    #[serde(rename = "maintenanceWindowCycles", default, skip_serializing_if = "Vec::is_empty")]
    pub maintenance_window_cycles: Vec<MaintenanceWindowTimeRange>,
    #[doc = "Minimum duration of maintenance window."]
    #[serde(rename = "minDurationInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub min_duration_in_minutes: Option<i32>,
    #[doc = "Default duration for maintenance window."]
    #[serde(rename = "defaultDurationInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub default_duration_in_minutes: Option<i32>,
    #[doc = "Minimum number of maintenance windows cycles to be set on the database."]
    #[serde(rename = "minCycles", default, skip_serializing_if = "Option::is_none")]
    pub min_cycles: Option<i32>,
    #[doc = "Time granularity in minutes for maintenance windows."]
    #[serde(rename = "timeGranularityInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub time_granularity_in_minutes: Option<i32>,
    #[doc = "Whether we allow multiple maintenance windows per cycle."]
    #[serde(
        rename = "allowMultipleMaintenanceWindowsPerCycle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_multiple_maintenance_windows_per_cycle: Option<bool>,
}
impl MaintenanceWindowOptionsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Maintenance window time range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceWindowTimeRange {
    #[doc = "Day of maintenance window."]
    #[serde(rename = "dayOfWeek", default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<maintenance_window_time_range::DayOfWeek>,
    #[doc = "Start time minutes offset from 12am."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Duration of maintenance window in minutes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}
impl MaintenanceWindowTimeRange {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod maintenance_window_time_range {
    use super::*;
    #[doc = "Day of maintenance window."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DayOfWeek")]
    pub enum DayOfWeek {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DayOfWeek {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DayOfWeek {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DayOfWeek {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Sunday => serializer.serialize_unit_variant("DayOfWeek", 0u32, "Sunday"),
                Self::Monday => serializer.serialize_unit_variant("DayOfWeek", 1u32, "Monday"),
                Self::Tuesday => serializer.serialize_unit_variant("DayOfWeek", 2u32, "Tuesday"),
                Self::Wednesday => serializer.serialize_unit_variant("DayOfWeek", 3u32, "Wednesday"),
                Self::Thursday => serializer.serialize_unit_variant("DayOfWeek", 4u32, "Thursday"),
                Self::Friday => serializer.serialize_unit_variant("DayOfWeek", 5u32, "Friday"),
                Self::Saturday => serializer.serialize_unit_variant("DayOfWeek", 6u32, "Saturday"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Maintenance windows."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceWindows {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Maintenance windows resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MaintenanceWindowsProperties>,
}
impl MaintenanceWindows {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Maintenance windows resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceWindowsProperties {
    #[serde(rename = "timeRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub time_ranges: Vec<MaintenanceWindowTimeRange>,
}
impl MaintenanceWindowsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workspace managed identity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentity {
    #[doc = "The principal ID of the workspace managed identity"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the workspace managed identity"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of managed identity for the workspace"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<managed_identity::Type>,
    #[doc = "The User Assigned Managed Identities."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedManagedIdentities>,
}
impl ManagedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_identity {
    use super::*;
    #[doc = "The type of managed identity for the workspace"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        None,
        SystemAssigned,
        #[serde(rename = "SystemAssigned,UserAssigned")]
        SystemAssignedUserAssigned,
    }
}
#[doc = "Sql Control Settings for workspace managed identity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentitySqlControlSettingsModel {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Sql Control Settings for workspace managed identity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<managed_identity_sql_control_settings_model::Properties>,
}
impl ManagedIdentitySqlControlSettingsModel {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_identity_sql_control_settings_model {
    use super::*;
    #[doc = "Sql Control Settings for workspace managed identity"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Grant sql control to managed identity"]
        #[serde(rename = "grantSqlControlToManagedIdentity", default, skip_serializing_if = "Option::is_none")]
        pub grant_sql_control_to_managed_identity: Option<properties::GrantSqlControlToManagedIdentity>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Grant sql control to managed identity"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct GrantSqlControlToManagedIdentity {
            #[doc = "Desired state"]
            #[serde(rename = "desiredState", default, skip_serializing_if = "Option::is_none")]
            pub desired_state: Option<grant_sql_control_to_managed_identity::DesiredState>,
            #[doc = "Actual state"]
            #[serde(rename = "actualState", default, skip_serializing_if = "Option::is_none")]
            pub actual_state: Option<grant_sql_control_to_managed_identity::ActualState>,
        }
        impl GrantSqlControlToManagedIdentity {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod grant_sql_control_to_managed_identity {
            use super::*;
            #[doc = "Desired state"]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            pub enum DesiredState {
                Enabled,
                Disabled,
            }
            #[doc = "Actual state"]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            pub enum ActualState {
                Enabling,
                Enabled,
                Disabling,
                Disabled,
                Unknown,
            }
        }
    }
}
#[doc = "Managed integration runtime, including managed elastic and managed dedicated integration runtimes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedIntegrationRuntime {
    #[serde(flatten)]
    pub integration_runtime: IntegrationRuntime,
    #[doc = "The state of integration runtime."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<IntegrationRuntimeState>,
    #[doc = "Managed integration runtime type properties."]
    #[serde(rename = "typeProperties")]
    pub type_properties: ManagedIntegrationRuntimeTypeProperties,
    #[doc = "Managed integration runtime managed virtual network reference."]
    #[serde(rename = "managedVirtualNetwork", default, skip_serializing_if = "Option::is_none")]
    pub managed_virtual_network: Option<ManagedIntegrationRuntimeManagedVirtualNetworkReference>,
}
impl ManagedIntegrationRuntime {
    pub fn new(integration_runtime: IntegrationRuntime, type_properties: ManagedIntegrationRuntimeTypeProperties) -> Self {
        Self {
            integration_runtime,
            state: None,
            type_properties,
            managed_virtual_network: None,
        }
    }
}
#[doc = "Error definition for managed integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIntegrationRuntimeError {
    #[doc = "The time when the error occurred."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Managed integration runtime error parameters."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<String>,
    #[doc = "Error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ManagedIntegrationRuntimeError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed integration runtime managed virtual network reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIntegrationRuntimeManagedVirtualNetworkReference {
    #[doc = "The reference name of the managed virtual network."]
    #[serde(rename = "referenceName", default, skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<String>,
    #[doc = "The type of the managed virtual network."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The id of the managed virtual network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ManagedIntegrationRuntimeManagedVirtualNetworkReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of integration runtime node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIntegrationRuntimeNode {
    #[doc = "The managed integration runtime node id."]
    #[serde(rename = "nodeId", default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[doc = "The managed integration runtime node status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<managed_integration_runtime_node::Status>,
    #[doc = "The errors that occurred on this integration runtime node."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ManagedIntegrationRuntimeError>,
}
impl ManagedIntegrationRuntimeNode {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_integration_runtime_node {
    use super::*;
    #[doc = "The managed integration runtime node status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Starting,
        Available,
        Recycling,
        Unavailable,
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
                Self::Starting => serializer.serialize_unit_variant("Status", 0u32, "Starting"),
                Self::Available => serializer.serialize_unit_variant("Status", 1u32, "Available"),
                Self::Recycling => serializer.serialize_unit_variant("Status", 2u32, "Recycling"),
                Self::Unavailable => serializer.serialize_unit_variant("Status", 3u32, "Unavailable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of managed integration runtime operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIntegrationRuntimeOperationResult {
    #[doc = "The operation type. Could be start or stop."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The operation result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[doc = "The error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Managed integration runtime error parameters."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<String>,
    #[doc = "The activity id for the operation request."]
    #[serde(rename = "activityId", default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
}
impl ManagedIntegrationRuntimeOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed integration runtime status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedIntegrationRuntimeStatus {
    #[serde(flatten)]
    pub integration_runtime_status: IntegrationRuntimeStatus,
    #[doc = "Managed integration runtime status type properties."]
    #[serde(rename = "typeProperties")]
    pub type_properties: ManagedIntegrationRuntimeStatusTypeProperties,
}
impl ManagedIntegrationRuntimeStatus {
    pub fn new(
        integration_runtime_status: IntegrationRuntimeStatus,
        type_properties: ManagedIntegrationRuntimeStatusTypeProperties,
    ) -> Self {
        Self {
            integration_runtime_status,
            type_properties,
        }
    }
}
#[doc = "Managed integration runtime status type properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIntegrationRuntimeStatusTypeProperties {
    #[doc = "The time at which the integration runtime was created, in ISO8601 format."]
    #[serde(rename = "createTime", with = "azure_core::date::rfc3339::option")]
    pub create_time: Option<time::OffsetDateTime>,
    #[doc = "The list of nodes for managed integration runtime."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<ManagedIntegrationRuntimeNode>,
    #[doc = "The errors that occurred on this integration runtime."]
    #[serde(rename = "otherErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub other_errors: Vec<ManagedIntegrationRuntimeError>,
    #[doc = "Properties of managed integration runtime operation result."]
    #[serde(rename = "lastOperation", default, skip_serializing_if = "Option::is_none")]
    pub last_operation: Option<ManagedIntegrationRuntimeOperationResult>,
}
impl ManagedIntegrationRuntimeStatusTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed integration runtime type properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIntegrationRuntimeTypeProperties {
    #[doc = "The compute resource properties for managed integration runtime."]
    #[serde(rename = "computeProperties", default, skip_serializing_if = "Option::is_none")]
    pub compute_properties: Option<IntegrationRuntimeComputeProperties>,
    #[doc = "SSIS properties for managed integration runtime."]
    #[serde(rename = "ssisProperties", default, skip_serializing_if = "Option::is_none")]
    pub ssis_properties: Option<IntegrationRuntimeSsisProperties>,
}
impl ManagedIntegrationRuntimeTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed Virtual Network Settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedVirtualNetworkSettings {
    #[doc = "Prevent Data Exfiltration"]
    #[serde(rename = "preventDataExfiltration", default, skip_serializing_if = "Option::is_none")]
    pub prevent_data_exfiltration: Option<bool>,
    #[doc = "Linked Access Check On Target Resource"]
    #[serde(rename = "linkedAccessCheckOnTargetResource", default, skip_serializing_if = "Option::is_none")]
    pub linked_access_check_on_target_resource: Option<bool>,
    #[doc = "Allowed Aad Tenant Ids For Linking"]
    #[serde(rename = "allowedAadTenantIdsForLinking", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_aad_tenant_ids_for_linking: Vec<String>,
}
impl ManagedVirtualNetworkSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration for metadata sync"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataSyncConfig {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata Sync Config properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<metadata_sync_config::Properties>,
}
impl MetadataSyncConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod metadata_sync_config {
    use super::*;
    #[doc = "Metadata Sync Config properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Indicates whether the metadata sync is enabled or disabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The Sync Interval in minutes."]
        #[serde(rename = "syncIntervalInMinutes", default, skip_serializing_if = "Option::is_none")]
        pub sync_interval_in_minutes: Option<i32>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "What is this?"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaLogSpecification {
    #[doc = "Log display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Time range the log covers"]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
    #[doc = "Log unique name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl OperationMetaLogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "What is this?"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaMetricDimensionSpecification {
    #[doc = "Dimension display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Dimension unique name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether this metric should be exported for Shoebox"]
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl OperationMetaMetricDimensionSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "What is this?"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaMetricSpecification {
    #[doc = "The source MDM namespace"]
    #[serde(rename = "sourceMdmNamespace", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_namespace: Option<String>,
    #[doc = "Metric display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Metric unique name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Metric aggregation type"]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "Metric description"]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "The source MDM account"]
    #[serde(rename = "sourceMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_account: Option<String>,
    #[doc = "Whether the regional MDM account is enabled"]
    #[serde(rename = "enableRegionalMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub enable_regional_mdm_account: Option<bool>,
    #[doc = "Metric units"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Metric dimensions"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<OperationMetaMetricDimensionSpecification>,
    #[doc = "Whether the metric supports instance-level aggregation"]
    #[serde(rename = "supportsInstanceLevelAggregation", default, skip_serializing_if = "Option::is_none")]
    pub supports_instance_level_aggregation: Option<bool>,
    #[doc = "Metric filter"]
    #[serde(rename = "metricFilterPattern", default, skip_serializing_if = "Option::is_none")]
    pub metric_filter_pattern: Option<String>,
}
impl OperationMetaMetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "What is this?"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaPropertyInfo {
    #[doc = "What is this?"]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<OperationMetaServiceSpecification>,
}
impl OperationMetaPropertyInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "What is this?"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaServiceSpecification {
    #[doc = "Service metric specifications"]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<OperationMetaMetricSpecification>,
    #[doc = "Service log specifications"]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<OperationMetaLogSpecification>,
}
impl OperationMetaServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResource {
    #[doc = "Operation ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Operation name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Operation status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_resource::Status>,
    #[doc = "Operation properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
    #[doc = "Operation start time"]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Operation start time"]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Completion percentage of the operation"]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
}
impl OperationResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_resource {
    use super::*;
    #[doc = "Operation status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        InProgress,
        Succeeded,
        Failed,
        Canceled,
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
                Self::InProgress => serializer.serialize_unit_variant("Status", 0u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 3u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Private endpoint details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "Resource id of the private endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionForPrivateLinkHub {
    #[serde(flatten)]
    pub private_endpoint_connection_for_private_link_hub_basic: PrivateEndpointConnectionForPrivateLinkHubBasic,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PrivateEndpointConnectionForPrivateLinkHub {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private Endpoint Connection For Private Link Hub - Basic"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionForPrivateLinkHubBasic {
    #[doc = "identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Properties of a private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnectionForPrivateLinkHubBasic {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionForPrivateLinkHubResourceCollectionResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnectionForPrivateLinkHub>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionForPrivateLinkHubResourceCollectionResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateEndpointConnectionForPrivateLinkHubResourceCollectionResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private endpoint connections"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionList {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateEndpointConnectionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "Private endpoint details"]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "Connection state details of the private endpoint"]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
    #[doc = "Provisioning state of the private endpoint connection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A privateLinkHub"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkHub {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "PrivateLinkHub properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkHubProperties>,
}
impl PrivateLinkHub {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "List of privateLinkHubs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkHubInfoListResult {
    #[doc = "Link to the next page of results"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of privateLinkHubs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkHub>,
}
impl azure_core::Continuable for PrivateLinkHubInfoListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateLinkHubInfoListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "PrivateLinkHub patch details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkHubPatchInfo {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PrivateLinkHubPatchInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "PrivateLinkHub properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkHubProperties {
    #[doc = "PrivateLinkHub provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "List of private endpoint connections"]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnectionForPrivateLinkHubBasic>,
}
impl PrivateLinkHubProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "Required DNS zone names of the the private link resource."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Connection state details of the private endpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private link service connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The private link service connection description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The actions required for private link service connection."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
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
#[doc = "Purview Configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PurviewConfiguration {
    #[doc = "Purview Resource ID"]
    #[serde(rename = "purviewResourceId", default, skip_serializing_if = "Option::is_none")]
    pub purview_resource_id: Option<String>,
}
impl PurviewConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A database query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryInterval {
    #[doc = "The start time of the measurement interval (ISO8601 format)."]
    #[serde(rename = "intervalStartTime", with = "azure_core::date::rfc3339::option")]
    pub interval_start_time: Option<time::OffsetDateTime>,
    #[doc = "The number of times the query was executed during this interval."]
    #[serde(rename = "executionCount", default, skip_serializing_if = "Option::is_none")]
    pub execution_count: Option<f64>,
    #[doc = "The list of query metrics during this interval."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<QueryMetric>,
}
impl QueryInterval {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A database query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryMetric {
    #[doc = "The name of the metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The name of the metric for display in user interface"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The unit of measurement"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<query_metric::Unit>,
    #[doc = "The measured value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl QueryMetric {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod query_metric {
    use super::*;
    #[doc = "The unit of measurement"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        #[serde(rename = "percentage")]
        Percentage,
        #[serde(rename = "KB")]
        Kb,
        #[serde(rename = "microseconds")]
        Microseconds,
    }
}
#[doc = "A database query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryStatistic {
    #[doc = "The id of the query"]
    #[serde(rename = "queryId", default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[doc = "The list of query intervals."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub intervals: Vec<QueryInterval>,
}
impl QueryStatistic {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A recommended sensitivity label update operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedSensitivityLabelUpdate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an operation executed on a recommended sensitivity label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecommendedSensitivityLabelUpdateProperties>,
}
impl RecommendedSensitivityLabelUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of recommended sensitivity label update operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedSensitivityLabelUpdateList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<RecommendedSensitivityLabelUpdate>,
}
impl RecommendedSensitivityLabelUpdateList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an operation executed on a recommended sensitivity label."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendedSensitivityLabelUpdateProperties {
    pub op: recommended_sensitivity_label_update_properties::Op,
    #[doc = "Schema name of the column to update."]
    pub schema: String,
    #[doc = "Table name of the column to update."]
    pub table: String,
    #[doc = "Column name to update."]
    pub column: String,
}
impl RecommendedSensitivityLabelUpdateProperties {
    pub fn new(op: recommended_sensitivity_label_update_properties::Op, schema: String, table: String, column: String) -> Self {
        Self { op, schema, table, column }
    }
}
pub mod recommended_sensitivity_label_update_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Op {
        #[serde(rename = "enable")]
        Enable,
        #[serde(rename = "disable")]
        Disable,
    }
}
#[doc = "A recoverable sql pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoverableSqlPool {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a recoverable sql pool"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecoverableSqlPoolProperties>,
}
impl RecoverableSqlPool {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list recoverable sql pools request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoverableSqlPoolListResult {
    #[doc = "A list of recoverable sql pool"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RecoverableSqlPool>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RecoverableSqlPoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RecoverableSqlPoolListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a recoverable sql pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoverableSqlPoolProperties {
    #[doc = "The edition of the database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[doc = "The service level objective name of the database"]
    #[serde(rename = "serviceLevelObjective", default, skip_serializing_if = "Option::is_none")]
    pub service_level_objective: Option<String>,
    #[doc = "The elastic pool name of the database"]
    #[serde(rename = "elasticPoolName", default, skip_serializing_if = "Option::is_none")]
    pub elastic_pool_name: Option<String>,
    #[doc = "The last available backup date of the database (ISO8601 format)"]
    #[serde(rename = "lastAvailableBackupDate", with = "azure_core::date::rfc3339::option")]
    pub last_available_backup_date: Option<time::OffsetDateTime>,
}
impl RecoverableSqlPoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An existing operation for replacing the firewall rules"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplaceAllFirewallRulesOperationResponse {
    #[doc = "The operation ID"]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}
impl ReplaceAllFirewallRulesOperationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replace all IP firewall rules request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplaceAllIpFirewallRulesRequest {
    #[doc = "IP firewall rule properties"]
    #[serde(rename = "ipFirewallRules", default, skip_serializing_if = "Option::is_none")]
    pub ip_firewall_rules: Option<serde_json::Value>,
}
impl ReplaceAllIpFirewallRulesRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Sql pool replication link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationLink {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Location of the workspace that contains this firewall rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Type of resource this is."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Represents the properties of a Sql pool replication link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReplicationLinkProperties>,
}
impl ReplicationLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the response to a List Sql pool replication link request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationLinkListResult {
    #[doc = "The list of Sql pool replication links housed in the Sql pool."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReplicationLink>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReplicationLinkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ReplicationLinkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of a Sql pool replication link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationLinkProperties {
    #[doc = "Legacy value indicating whether termination is allowed.  Currently always returns true."]
    #[serde(rename = "isTerminationAllowed", default, skip_serializing_if = "Option::is_none")]
    pub is_termination_allowed: Option<bool>,
    #[doc = "Replication mode of this replication link."]
    #[serde(rename = "replicationMode", default, skip_serializing_if = "Option::is_none")]
    pub replication_mode: Option<String>,
    #[doc = "The name of the workspace hosting the partner Sql pool."]
    #[serde(rename = "partnerServer", default, skip_serializing_if = "Option::is_none")]
    pub partner_server: Option<String>,
    #[doc = "The name of the partner Sql pool."]
    #[serde(rename = "partnerDatabase", default, skip_serializing_if = "Option::is_none")]
    pub partner_database: Option<String>,
    #[doc = "The Azure Region of the partner Sql pool."]
    #[serde(rename = "partnerLocation", default, skip_serializing_if = "Option::is_none")]
    pub partner_location: Option<String>,
    #[doc = "The role of the Sql pool in the replication link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<replication_link_properties::Role>,
    #[doc = "The role of the partner Sql pool in the replication link."]
    #[serde(rename = "partnerRole", default, skip_serializing_if = "Option::is_none")]
    pub partner_role: Option<replication_link_properties::PartnerRole>,
    #[doc = "The start time for the replication link."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The percentage of seeding complete for the replication link."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "The replication state for the replication link."]
    #[serde(rename = "replicationState", default, skip_serializing_if = "Option::is_none")]
    pub replication_state: Option<replication_link_properties::ReplicationState>,
}
impl ReplicationLinkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod replication_link_properties {
    use super::*;
    #[doc = "The role of the Sql pool in the replication link."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Role {
        Primary,
        Secondary,
        NonReadableSecondary,
        Source,
        Copy,
    }
    #[doc = "The role of the partner Sql pool in the replication link."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PartnerRole {
        Primary,
        Secondary,
        NonReadableSecondary,
        Source,
        Copy,
    }
    #[doc = "The replication state for the replication link."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReplicationState")]
    pub enum ReplicationState {
        #[serde(rename = "PENDING")]
        Pending,
        #[serde(rename = "SEEDING")]
        Seeding,
        #[serde(rename = "CATCH_UP")]
        CatchUp,
        #[serde(rename = "SUSPENDED")]
        Suspended,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReplicationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReplicationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReplicationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pending => serializer.serialize_unit_variant("ReplicationState", 0u32, "PENDING"),
                Self::Seeding => serializer.serialize_unit_variant("ReplicationState", 1u32, "SEEDING"),
                Self::CatchUp => serializer.serialize_unit_variant("ReplicationState", 2u32, "CATCH_UP"),
                Self::Suspended => serializer.serialize_unit_variant("ReplicationState", 3u32, "SUSPENDED"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "Contains the information necessary to perform a resource move (rename)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceMoveDefinition {
    #[doc = "The target ID for the resource"]
    pub id: String,
}
impl ResourceMoveDefinition {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "A restorable dropped Sql pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableDroppedSqlPool {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The properties of a restorable dropped Sql pool"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RestorableDroppedSqlPoolProperties>,
}
impl RestorableDroppedSqlPool {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list restorable dropped Sql pools request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestorableDroppedSqlPoolListResult {
    #[doc = "A list of restorable dropped Sql pools"]
    pub value: Vec<RestorableDroppedSqlPool>,
}
impl azure_core::Continuable for RestorableDroppedSqlPoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RestorableDroppedSqlPoolListResult {
    pub fn new(value: Vec<RestorableDroppedSqlPool>) -> Self {
        Self { value }
    }
}
#[doc = "The properties of a restorable dropped Sql pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableDroppedSqlPoolProperties {
    #[doc = "The name of the database"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The edition of the database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[doc = "The max size in bytes of the database"]
    #[serde(rename = "maxSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_bytes: Option<String>,
    #[doc = "The service level objective name of the database"]
    #[serde(rename = "serviceLevelObjective", default, skip_serializing_if = "Option::is_none")]
    pub service_level_objective: Option<String>,
    #[doc = "The elastic pool name of the database"]
    #[serde(rename = "elasticPoolName", default, skip_serializing_if = "Option::is_none")]
    pub elastic_pool_name: Option<String>,
    #[doc = "The creation date of the database (ISO8601 format)"]
    #[serde(rename = "creationDate", with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The deletion date of the database (ISO8601 format)"]
    #[serde(rename = "deletionDate", with = "azure_core::date::rfc3339::option")]
    pub deletion_date: Option<time::OffsetDateTime>,
    #[doc = "The earliest restore date of the database (ISO8601 format)"]
    #[serde(rename = "earliestRestoreDate", with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_date: Option<time::OffsetDateTime>,
}
impl RestorableDroppedSqlPoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Database restore points."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties of a database restore point"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RestorePointProperties>,
}
impl RestorePoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of long term retention backups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePointListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RestorePoint>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RestorePointListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RestorePointListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a database restore point"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePointProperties {
    #[doc = "The type of restore point"]
    #[serde(rename = "restorePointType", default, skip_serializing_if = "Option::is_none")]
    pub restore_point_type: Option<restore_point_properties::RestorePointType>,
    #[doc = "The earliest time to which this database can be restored"]
    #[serde(rename = "earliestRestoreDate", with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_date: Option<time::OffsetDateTime>,
    #[doc = "The time the backup was taken"]
    #[serde(rename = "restorePointCreationDate", with = "azure_core::date::rfc3339::option")]
    pub restore_point_creation_date: Option<time::OffsetDateTime>,
    #[doc = "The label of restore point for backup request by user"]
    #[serde(rename = "restorePointLabel", default, skip_serializing_if = "Option::is_none")]
    pub restore_point_label: Option<String>,
}
impl RestorePointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod restore_point_properties {
    use super::*;
    #[doc = "The type of restore point"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RestorePointType {
        #[serde(rename = "CONTINUOUS")]
        Continuous,
        #[serde(rename = "DISCRETE")]
        Discrete,
    }
}
#[doc = "The base definition of a secret type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretBase {
    #[doc = "Type of the secret."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl SecretBase {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "Azure Synapse secure string definition. The string value will be masked with asterisks '*' during Get or List API calls."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecureString {
    #[serde(flatten)]
    pub secret_base: SecretBase,
    #[doc = "Value of secure string."]
    pub value: String,
}
impl SecureString {
    pub fn new(secret_base: SecretBase, value: String) -> Self {
        Self { secret_base, value }
    }
}
#[doc = "Properties of a security alert policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAlertPolicyProperties {
    #[doc = "Specifies the state of the policy, whether it is enabled or disabled or a policy has not been applied yet on the specific Sql pool."]
    pub state: security_alert_policy_properties::State,
    #[doc = "Specifies an array of alerts that are disabled. Allowed values are: Sql_Injection, Sql_Injection_Vulnerability, Access_Anomaly, Data_Exfiltration, Unsafe_Action"]
    #[serde(rename = "disabledAlerts", default, skip_serializing_if = "Vec::is_empty")]
    pub disabled_alerts: Vec<String>,
    #[doc = "Specifies an array of e-mail addresses to which the alert is sent."]
    #[serde(rename = "emailAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub email_addresses: Vec<String>,
    #[doc = "Specifies that the alert is sent to the account administrators."]
    #[serde(rename = "emailAccountAdmins", default, skip_serializing_if = "Option::is_none")]
    pub email_account_admins: Option<bool>,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). This blob storage will hold all Threat Detection audit logs."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the Threat Detection audit storage account."]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the number of days to keep in the Threat Detection audit logs."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies the UTC creation time of the policy."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
}
impl SecurityAlertPolicyProperties {
    pub fn new(state: security_alert_policy_properties::State) -> Self {
        Self {
            state,
            disabled_alerts: Vec::new(),
            email_addresses: Vec::new(),
            email_account_admins: None,
            storage_endpoint: None,
            storage_account_access_key: None,
            retention_days: None,
            creation_time: None,
        }
    }
}
pub mod security_alert_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the policy, whether it is enabled or disabled or a policy has not been applied yet on the specific Sql pool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        New,
        Enabled,
        Disabled,
    }
}
#[doc = "Self-hosted integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelfHostedIntegrationRuntime {
    #[serde(flatten)]
    pub integration_runtime: IntegrationRuntime,
    #[doc = "The self-hosted integration runtime properties."]
    #[serde(rename = "typeProperties", default, skip_serializing_if = "Option::is_none")]
    pub type_properties: Option<SelfHostedIntegrationRuntimeTypeProperties>,
}
impl SelfHostedIntegrationRuntime {
    pub fn new(integration_runtime: IntegrationRuntime) -> Self {
        Self {
            integration_runtime,
            type_properties: None,
        }
    }
}
#[doc = "Properties of Self-hosted integration runtime node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SelfHostedIntegrationRuntimeNode {
    #[doc = "Name of the integration runtime node."]
    #[serde(rename = "nodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[doc = "Machine name of the integration runtime node."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "URI for the host machine of the integration runtime."]
    #[serde(rename = "hostServiceUri", default, skip_serializing_if = "Option::is_none")]
    pub host_service_uri: Option<String>,
    #[doc = "Status of the integration runtime node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<self_hosted_integration_runtime_node::Status>,
    #[doc = "The integration runtime capabilities dictionary"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    #[doc = "Status of the integration runtime node version."]
    #[serde(rename = "versionStatus", default, skip_serializing_if = "Option::is_none")]
    pub version_status: Option<String>,
    #[doc = "Version of the integration runtime node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The time at which the integration runtime node was registered in ISO8601 format."]
    #[serde(rename = "registerTime", with = "azure_core::date::rfc3339::option")]
    pub register_time: Option<time::OffsetDateTime>,
    #[doc = "The most recent time at which the integration runtime was connected in ISO8601 format."]
    #[serde(rename = "lastConnectTime", with = "azure_core::date::rfc3339::option")]
    pub last_connect_time: Option<time::OffsetDateTime>,
    #[doc = "The time at which the integration runtime will expire in ISO8601 format."]
    #[serde(rename = "expiryTime", with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[doc = "The time the node last started up."]
    #[serde(rename = "lastStartTime", with = "azure_core::date::rfc3339::option")]
    pub last_start_time: Option<time::OffsetDateTime>,
    #[doc = "The integration runtime node last stop time."]
    #[serde(rename = "lastStopTime", with = "azure_core::date::rfc3339::option")]
    pub last_stop_time: Option<time::OffsetDateTime>,
    #[doc = "The result of the last integration runtime node update."]
    #[serde(rename = "lastUpdateResult", default, skip_serializing_if = "Option::is_none")]
    pub last_update_result: Option<self_hosted_integration_runtime_node::LastUpdateResult>,
    #[doc = "The last time for the integration runtime node update start."]
    #[serde(rename = "lastStartUpdateTime", with = "azure_core::date::rfc3339::option")]
    pub last_start_update_time: Option<time::OffsetDateTime>,
    #[doc = "The last time for the integration runtime node update end."]
    #[serde(rename = "lastEndUpdateTime", with = "azure_core::date::rfc3339::option")]
    pub last_end_update_time: Option<time::OffsetDateTime>,
    #[doc = "Indicates whether this node is the active dispatcher for integration runtime requests."]
    #[serde(rename = "isActiveDispatcher", default, skip_serializing_if = "Option::is_none")]
    pub is_active_dispatcher: Option<bool>,
    #[doc = "Maximum concurrent jobs on the integration runtime node."]
    #[serde(rename = "concurrentJobsLimit", default, skip_serializing_if = "Option::is_none")]
    pub concurrent_jobs_limit: Option<i32>,
    #[doc = "The maximum concurrent jobs in this integration runtime."]
    #[serde(rename = "maxConcurrentJobs", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_jobs: Option<i32>,
}
impl SelfHostedIntegrationRuntimeNode {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod self_hosted_integration_runtime_node {
    use super::*;
    #[doc = "Status of the integration runtime node."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        NeedRegistration,
        Online,
        Limited,
        Offline,
        Upgrading,
        Initializing,
        InitializeFailed,
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
                Self::NeedRegistration => serializer.serialize_unit_variant("Status", 0u32, "NeedRegistration"),
                Self::Online => serializer.serialize_unit_variant("Status", 1u32, "Online"),
                Self::Limited => serializer.serialize_unit_variant("Status", 2u32, "Limited"),
                Self::Offline => serializer.serialize_unit_variant("Status", 3u32, "Offline"),
                Self::Upgrading => serializer.serialize_unit_variant("Status", 4u32, "Upgrading"),
                Self::Initializing => serializer.serialize_unit_variant("Status", 5u32, "Initializing"),
                Self::InitializeFailed => serializer.serialize_unit_variant("Status", 6u32, "InitializeFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The result of the last integration runtime node update."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastUpdateResult")]
    pub enum LastUpdateResult {
        None,
        Succeed,
        Fail,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastUpdateResult {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastUpdateResult {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastUpdateResult {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("LastUpdateResult", 0u32, "None"),
                Self::Succeed => serializer.serialize_unit_variant("LastUpdateResult", 1u32, "Succeed"),
                Self::Fail => serializer.serialize_unit_variant("LastUpdateResult", 2u32, "Fail"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Self-hosted integration runtime status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelfHostedIntegrationRuntimeStatus {
    #[serde(flatten)]
    pub integration_runtime_status: IntegrationRuntimeStatus,
    #[doc = "Self-hosted integration runtime status type properties."]
    #[serde(rename = "typeProperties")]
    pub type_properties: SelfHostedIntegrationRuntimeStatusTypeProperties,
}
impl SelfHostedIntegrationRuntimeStatus {
    pub fn new(
        integration_runtime_status: IntegrationRuntimeStatus,
        type_properties: SelfHostedIntegrationRuntimeStatusTypeProperties,
    ) -> Self {
        Self {
            integration_runtime_status,
            type_properties,
        }
    }
}
#[doc = "Self-hosted integration runtime status type properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SelfHostedIntegrationRuntimeStatusTypeProperties {
    #[doc = "The time at which the integration runtime was created, in ISO8601 format."]
    #[serde(rename = "createTime", with = "azure_core::date::rfc3339::option")]
    pub create_time: Option<time::OffsetDateTime>,
    #[doc = "The task queue id of the integration runtime."]
    #[serde(rename = "taskQueueId", default, skip_serializing_if = "Option::is_none")]
    pub task_queue_id: Option<String>,
    #[doc = "The node communication Channel encryption mode"]
    #[serde(
        rename = "nodeCommunicationChannelEncryptionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub node_communication_channel_encryption_mode: Option<String>,
    #[doc = "It is used to set the encryption mode for node-node communication channel (when more than 2 self-hosted integration runtime nodes exist)."]
    #[serde(rename = "internalChannelEncryption", default, skip_serializing_if = "Option::is_none")]
    pub internal_channel_encryption: Option<self_hosted_integration_runtime_status_type_properties::InternalChannelEncryption>,
    #[doc = "Version of the integration runtime."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The list of nodes for this integration runtime."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<SelfHostedIntegrationRuntimeNode>,
    #[doc = "The date at which the integration runtime will be scheduled to update, in ISO8601 format."]
    #[serde(rename = "scheduledUpdateDate", with = "azure_core::date::rfc3339::option")]
    pub scheduled_update_date: Option<time::OffsetDateTime>,
    #[doc = "The time in the date scheduled by service to update the integration runtime, e.g., PT03H is 3 hours"]
    #[serde(rename = "updateDelayOffset", default, skip_serializing_if = "Option::is_none")]
    pub update_delay_offset: Option<String>,
    #[doc = "The local time zone offset in hours."]
    #[serde(rename = "localTimeZoneOffset", default, skip_serializing_if = "Option::is_none")]
    pub local_time_zone_offset: Option<String>,
    #[doc = "Object with additional information about integration runtime capabilities."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    #[doc = "The URLs for the services used in integration runtime backend service."]
    #[serde(rename = "serviceUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub service_urls: Vec<String>,
    #[doc = "The state of integration runtime auto update."]
    #[serde(rename = "autoUpdate", default, skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<IntegrationRuntimeAutoUpdate>,
    #[doc = "Status of the integration runtime version."]
    #[serde(rename = "versionStatus", default, skip_serializing_if = "Option::is_none")]
    pub version_status: Option<String>,
    #[doc = "The list of linked integration runtimes that are created to share with this integration runtime."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<LinkedIntegrationRuntime>,
    #[doc = "The version that the integration runtime is going to update to."]
    #[serde(rename = "pushedVersion", default, skip_serializing_if = "Option::is_none")]
    pub pushed_version: Option<String>,
    #[doc = "The latest version on download center."]
    #[serde(rename = "latestVersion", default, skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[doc = "The estimated time when the self-hosted integration runtime will be updated."]
    #[serde(rename = "autoUpdateETA", with = "azure_core::date::rfc3339::option")]
    pub auto_update_eta: Option<time::OffsetDateTime>,
    #[doc = "The service region of the integration runtime"]
    #[serde(rename = "serviceRegion", default, skip_serializing_if = "Option::is_none")]
    pub service_region: Option<String>,
    #[doc = "The newer versions on download center."]
    #[serde(rename = "newerVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub newer_versions: Vec<String>,
}
impl SelfHostedIntegrationRuntimeStatusTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod self_hosted_integration_runtime_status_type_properties {
    use super::*;
    #[doc = "It is used to set the encryption mode for node-node communication channel (when more than 2 self-hosted integration runtime nodes exist)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InternalChannelEncryption")]
    pub enum InternalChannelEncryption {
        NotSet,
        SslEncrypted,
        NotEncrypted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InternalChannelEncryption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InternalChannelEncryption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InternalChannelEncryption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSet => serializer.serialize_unit_variant("InternalChannelEncryption", 0u32, "NotSet"),
                Self::SslEncrypted => serializer.serialize_unit_variant("InternalChannelEncryption", 1u32, "SslEncrypted"),
                Self::NotEncrypted => serializer.serialize_unit_variant("InternalChannelEncryption", 2u32, "NotEncrypted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The self-hosted integration runtime properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SelfHostedIntegrationRuntimeTypeProperties {
    #[doc = "The base definition of a linked integration runtime."]
    #[serde(rename = "linkedInfo", default, skip_serializing_if = "Option::is_none")]
    pub linked_info: Option<LinkedIntegrationRuntimeType>,
}
impl SelfHostedIntegrationRuntimeTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A sensitivity label."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensitivityLabel {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a sensitivity label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SensitivityLabelProperties>,
    #[doc = "managed by"]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
}
impl SensitivityLabel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of sensitivity labels."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensitivityLabelListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SensitivityLabel>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SensitivityLabelListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SensitivityLabelListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a sensitivity label."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensitivityLabelProperties {
    #[doc = "The schema name."]
    #[serde(rename = "schemaName", default, skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[doc = "The table name."]
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[doc = "The column name."]
    #[serde(rename = "columnName", default, skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[doc = "The label name."]
    #[serde(rename = "labelName", default, skip_serializing_if = "Option::is_none")]
    pub label_name: Option<String>,
    #[doc = "The label ID."]
    #[serde(rename = "labelId", default, skip_serializing_if = "Option::is_none")]
    pub label_id: Option<String>,
    #[doc = "The information type."]
    #[serde(rename = "informationType", default, skip_serializing_if = "Option::is_none")]
    pub information_type: Option<String>,
    #[doc = "The information type ID."]
    #[serde(rename = "informationTypeId", default, skip_serializing_if = "Option::is_none")]
    pub information_type_id: Option<String>,
    #[doc = "Is sensitivity recommendation disabled. Applicable for recommended sensitivity label only. Specifies whether the sensitivity recommendation on this column is disabled (dismissed) or not."]
    #[serde(rename = "isDisabled", default, skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<sensitivity_label_properties::Rank>,
}
impl SensitivityLabelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sensitivity_label_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Rank {
        None,
        Low,
        Medium,
        High,
        Critical,
    }
}
#[doc = "A sensitivity label update operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensitivityLabelUpdate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an operation executed on a sensitivity label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SensitivityLabelUpdateProperties>,
}
impl SensitivityLabelUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of sensitivity label update operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensitivityLabelUpdateList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<SensitivityLabelUpdate>,
}
impl SensitivityLabelUpdateList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an operation executed on a sensitivity label."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitivityLabelUpdateProperties {
    pub op: sensitivity_label_update_properties::Op,
    #[doc = "Schema name of the column to update."]
    pub schema: String,
    #[doc = "Table name of the column to update."]
    pub table: String,
    #[doc = "Column name to update."]
    pub column: String,
    #[doc = "A sensitivity label."]
    #[serde(rename = "sensitivityLabel", default, skip_serializing_if = "Option::is_none")]
    pub sensitivity_label: Option<SensitivityLabel>,
}
impl SensitivityLabelUpdateProperties {
    pub fn new(op: sensitivity_label_update_properties::Op, schema: String, table: String, column: String) -> Self {
        Self {
            op,
            schema,
            table,
            column,
            sensitivity_label: None,
        }
    }
}
pub mod sensitivity_label_update_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Op {
        #[serde(rename = "set")]
        Set,
        #[serde(rename = "remove")]
        Remove,
    }
}
#[doc = "A server blob auditing policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerBlobAuditingPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a server blob auditing policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerBlobAuditingPolicyProperties>,
}
impl ServerBlobAuditingPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server auditing settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerBlobAuditingPolicyListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerBlobAuditingPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerBlobAuditingPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServerBlobAuditingPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a server blob auditing policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerBlobAuditingPolicyProperties {
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    pub state: server_blob_auditing_policy_properties::State,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled is required."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the auditing storage account. \r\nIf state is Enabled and storageEndpoint is specified, not specifying the storageAccountAccessKey will use SQL server system-assigned managed identity to access the storage.\r\nPrerequisites for using managed identity authentication:\r\n1. Assign SQL Server a system-assigned managed identity in Azure Active Directory (AAD).\r\n2. Grant SQL Server identity access to the storage account by adding 'Storage Blob Data Contributor' RBAC role to the server identity.\r\nFor more information, see [Auditing to storage using Managed Identity authentication](https://go.microsoft.com/fwlink/?linkid=2114355)"]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the number of days to keep in the audit logs in the storage account."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies the Actions-Groups and Actions to audit.\r\n\r\nThe recommended set of action groups to use is the following combination - this will audit all the queries and stored procedures executed against the database, as well as successful and failed logins:\r\n\r\nBATCH_COMPLETED_GROUP,\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP,\r\nFAILED_DATABASE_AUTHENTICATION_GROUP.\r\n\r\nThis above combination is also the set that is configured by default when enabling auditing from the Azure portal.\r\n\r\nThe supported action groups to audit are (note: choose only specific groups that cover your auditing needs. Using unnecessary groups could lead to very large quantities of audit records):\r\n\r\nAPPLICATION_ROLE_CHANGE_PASSWORD_GROUP\r\nBACKUP_RESTORE_GROUP\r\nDATABASE_LOGOUT_GROUP\r\nDATABASE_OBJECT_CHANGE_GROUP\r\nDATABASE_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_OBJECT_PERMISSION_CHANGE_GROUP\r\nDATABASE_OPERATION_GROUP\r\nDATABASE_PERMISSION_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_IMPERSONATION_GROUP\r\nDATABASE_ROLE_MEMBER_CHANGE_GROUP\r\nFAILED_DATABASE_AUTHENTICATION_GROUP\r\nSCHEMA_OBJECT_ACCESS_GROUP\r\nSCHEMA_OBJECT_CHANGE_GROUP\r\nSCHEMA_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nSCHEMA_OBJECT_PERMISSION_CHANGE_GROUP\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP\r\nUSER_CHANGE_PASSWORD_GROUP\r\nBATCH_STARTED_GROUP\r\nBATCH_COMPLETED_GROUP\r\n\r\nThese are groups that cover all sql statements and stored procedures executed against the database, and should not be used in combination with other groups as this will result in duplicate audit logs.\r\n\r\nFor more information, see [Database-Level Audit Action Groups](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-action-groups).\r\n\r\nFor Database auditing policy, specific Actions can also be specified (note that Actions cannot be specified for Server auditing policy). The supported actions to audit are:\r\nSELECT\r\nUPDATE\r\nINSERT\r\nDELETE\r\nEXECUTE\r\nRECEIVE\r\nREFERENCES\r\n\r\nThe general form for defining an action to be audited is:\r\n{action} ON {object} BY {principal}\r\n\r\nNote that <object> in the above format can refer to an object like a table, view, or stored procedure, or an entire database or schema. For the latter cases, the forms DATABASE::{db_name} and SCHEMA::{schema_name} are used, respectively.\r\n\r\nFor example:\r\nSELECT on dbo.myTable by public\r\nSELECT on DATABASE::myDatabase by public\r\nSELECT on SCHEMA::mySchema by public\r\n\r\nFor more information, see [Database-Level Audit Actions](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-actions)"]
    #[serde(rename = "auditActionsAndGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub audit_actions_and_groups: Vec<String>,
    #[doc = "Specifies the blob storage subscription Id."]
    #[serde(rename = "storageAccountSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_subscription_id: Option<String>,
    #[doc = "Specifies whether storageAccountAccessKey value is the storage's secondary key."]
    #[serde(rename = "isStorageSecondaryKeyInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_storage_secondary_key_in_use: Option<bool>,
    #[doc = "Specifies whether audit events are sent to Azure Monitor. \r\nIn order to send the events to Azure Monitor, specify 'state' as 'Enabled' and 'isAzureMonitorTargetEnabled' as true.\r\n\r\nWhen using REST API to configure auditing, Diagnostic Settings with 'SQLSecurityAuditEvents' diagnostic logs category on the database should be also created.\r\nNote that for server level audit you should use the 'master' database as {databaseName}.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/{databaseName}/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isAzureMonitorTargetEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_monitor_target_enabled: Option<bool>,
    #[doc = "Specifies the amount of time in milliseconds that can elapse before audit actions are forced to be processed.\r\nThe default minimum value is 1000 (1 second). The maximum is 2,147,483,647."]
    #[serde(rename = "queueDelayMs", default, skip_serializing_if = "Option::is_none")]
    pub queue_delay_ms: Option<i32>,
    #[doc = "Specifies the state of devops audit. If state is Enabled, devops logs will be sent to Azure Monitor.\r\nIn order to send the events to Azure Monitor, specify 'State' as 'Enabled', 'IsAzureMonitorTargetEnabled' as true and 'IsDevopsAuditEnabled' as true\r\n\r\nWhen using REST API to configure auditing, Diagnostic Settings with 'DevOpsOperationsAudit' diagnostic logs category on the master database should also be created.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/master/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isDevopsAuditEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_devops_audit_enabled: Option<bool>,
}
impl ServerBlobAuditingPolicyProperties {
    pub fn new(state: server_blob_auditing_policy_properties::State) -> Self {
        Self {
            state,
            storage_endpoint: None,
            storage_account_access_key: None,
            retention_days: None,
            audit_actions_and_groups: Vec::new(),
            storage_account_subscription_id: None,
            is_storage_secondary_key_in_use: None,
            is_azure_monitor_target_enabled: None,
            queue_delay_ms: None,
            is_devops_audit_enabled: None,
        }
    }
}
pub mod server_blob_auditing_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "Workspace managed Sql server security alert policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerSecurityAlertPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a security alert policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerSecurityAlertPolicyProperties>,
}
impl ServerSecurityAlertPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the workspace managed sql server's security alert policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerSecurityAlertPolicyListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerSecurityAlertPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerSecurityAlertPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServerSecurityAlertPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a security alert policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerSecurityAlertPolicyProperties {
    #[doc = "Specifies the state of the policy, whether it is enabled or disabled or a policy has not been applied yet on the specific server"]
    pub state: server_security_alert_policy_properties::State,
    #[doc = "Specifies an array of alerts that are disabled. Allowed values are: Sql_Injection, Sql_Injection_Vulnerability, Access_Anomaly, Data_Exfiltration, Unsafe_Action"]
    #[serde(rename = "disabledAlerts", default, skip_serializing_if = "Vec::is_empty")]
    pub disabled_alerts: Vec<String>,
    #[doc = "Specifies an array of e-mail addresses to which the alert is sent."]
    #[serde(rename = "emailAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub email_addresses: Vec<String>,
    #[doc = "Specifies that the alert is sent to the account administrators."]
    #[serde(rename = "emailAccountAdmins", default, skip_serializing_if = "Option::is_none")]
    pub email_account_admins: Option<bool>,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). This blob storage will hold all Threat Detection audit logs."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the Threat Detection audit storage account."]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the number of days to keep in the Threat Detection audit logs."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies the UTC creation time of the policy."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
}
impl ServerSecurityAlertPolicyProperties {
    pub fn new(state: server_security_alert_policy_properties::State) -> Self {
        Self {
            state,
            disabled_alerts: Vec::new(),
            email_addresses: Vec::new(),
            email_account_admins: None,
            storage_endpoint: None,
            storage_account_access_key: None,
            retention_days: None,
            creation_time: None,
        }
    }
}
pub mod server_security_alert_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the policy, whether it is enabled or disabled or a policy has not been applied yet on the specific server"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        New,
        Enabled,
        Disabled,
    }
}
#[doc = "Represents server metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerUsage {
    #[doc = "Name of the server usage metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "The metric display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The current value of the metric."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[doc = "The current limit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[doc = "The units of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The next reset time for the metric (ISO8601 format)."]
    #[serde(rename = "nextResetTime", with = "azure_core::date::rfc3339::option")]
    pub next_reset_time: Option<time::OffsetDateTime>,
}
impl ServerUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the response to a list server metrics request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerUsageListResult {
    #[doc = "The list of server metrics for the server."]
    pub value: Vec<ServerUsage>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerUsageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServerUsageListResult {
    pub fn new(value: Vec<ServerUsage>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A server vulnerability assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerVulnerabilityAssessment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a server Vulnerability Assessment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerVulnerabilityAssessmentProperties>,
}
impl ServerVulnerabilityAssessment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the server's vulnerability assessments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerVulnerabilityAssessmentListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerVulnerabilityAssessment>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerVulnerabilityAssessmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServerVulnerabilityAssessmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a server Vulnerability Assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerVulnerabilityAssessmentProperties {
    #[doc = "A blob storage container path to hold the scan results (e.g. https://myStorage.blob.core.windows.net/VaScans/)."]
    #[serde(rename = "storageContainerPath")]
    pub storage_container_path: String,
    #[doc = "A shared access signature (SAS Key) that has read and write access to the blob container specified in 'storageContainerPath' parameter. If 'storageAccountAccessKey' isn't specified, StorageContainerSasKey is required."]
    #[serde(rename = "storageContainerSasKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_sas_key: Option<String>,
    #[doc = "Specifies the identifier key of the storage account for vulnerability assessment scan results. If 'StorageContainerSasKey' isn't specified, storageAccountAccessKey is required."]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Properties of a Vulnerability Assessment recurring scans."]
    #[serde(rename = "recurringScans", default, skip_serializing_if = "Option::is_none")]
    pub recurring_scans: Option<VulnerabilityAssessmentRecurringScansProperties>,
}
impl ServerVulnerabilityAssessmentProperties {
    pub fn new(storage_container_path: String) -> Self {
        Self {
            storage_container_path,
            storage_container_sas_key: None,
            storage_account_access_key: None,
            recurring_scans: None,
        }
    }
}
#[doc = "SQL pool SKU"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "The service tier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The SKU name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "If the SKU supports scale out/in then the capacity integer should be included. If scale out/in is not possible for the resource this may be omitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SparkConfig Properties for a Big Data pool powered by Apache Spark"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkConfigProperties {
    #[doc = "The last update time of the spark config properties file."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
    #[doc = "The spark config properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "The filename of the spark config properties file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[doc = "The type of the spark config properties file."]
    #[serde(rename = "configurationType", default, skip_serializing_if = "Option::is_none")]
    pub configuration_type: Option<spark_config_properties::ConfigurationType>,
}
impl SparkConfigProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod spark_config_properties {
    use super::*;
    #[doc = "The type of the spark config properties file."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConfigurationType")]
    pub enum ConfigurationType {
        File,
        Artifact,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConfigurationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConfigurationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConfigurationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::File => serializer.serialize_unit_variant("ConfigurationType", 0u32, "File"),
                Self::Artifact => serializer.serialize_unit_variant("ConfigurationType", 1u32, "Artifact"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "SparkConfiguration Artifact information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkConfigurationInfo {
    #[doc = "Description about the SparkConfiguration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "SparkConfiguration configs."]
    pub configs: serde_json::Value,
    #[doc = "Annotations for SparkConfiguration."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<String>,
    #[doc = "additional Notes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The timestamp of resource creation."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
}
impl SparkConfigurationInfo {
    pub fn new(configs: serde_json::Value) -> Self {
        Self {
            description: None,
            configs,
            annotations: Vec::new(),
            notes: None,
            created_by: None,
            created: None,
        }
    }
}
#[doc = "A list of SparkConfiguration resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkConfigurationListResponse {
    #[doc = "List of SparkConfiguration."]
    pub value: Vec<SparkConfigurationResource>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SparkConfigurationListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SparkConfigurationListResponse {
    pub fn new(value: Vec<SparkConfigurationResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "SparkConfiguration response details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkConfigurationResource {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "SparkConfiguration Artifact information"]
    pub properties: SparkConfigurationInfo,
}
impl SparkConfigurationResource {
    pub fn new(properties: SparkConfigurationInfo) -> Self {
        Self {
            sub_resource: SubResource::default(),
            properties,
        }
    }
}
#[doc = "A SQL Analytics pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlPool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "SQL pool SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Properties of a SQL Analytics pool"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlPoolResourceProperties>,
}
impl SqlPool {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            sku: None,
            properties: None,
        }
    }
}
#[doc = "A Sql pool blob auditing policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolBlobAuditingPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource kind."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Properties of a Sql pool blob auditing policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlPoolBlobAuditingPolicyProperties>,
}
impl SqlPoolBlobAuditingPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of Sql pool auditing settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolBlobAuditingPolicyListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlPoolBlobAuditingPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlPoolBlobAuditingPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SqlPoolBlobAuditingPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Sql pool blob auditing policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlPoolBlobAuditingPolicyProperties {
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    pub state: sql_pool_blob_auditing_policy_properties::State,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). If state is Enabled, storageEndpoint is required."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the auditing storage account. If state is Enabled and storageEndpoint is specified, storageAccountAccessKey is required."]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the number of days to keep in the audit logs in the storage account."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies the Actions-Groups and Actions to audit.\r\n\r\nThe recommended set of action groups to use is the following combination - this will audit all the queries and stored procedures executed against the database, as well as successful and failed logins:\r\n\r\nBATCH_COMPLETED_GROUP,\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP,\r\nFAILED_DATABASE_AUTHENTICATION_GROUP.\r\n\r\nThis above combination is also the set that is configured by default when enabling auditing from the Azure portal.\r\n\r\nThe supported action groups to audit are (note: choose only specific groups that cover your auditing needs. Using unnecessary groups could lead to very large quantities of audit records):\r\n\r\nAPPLICATION_ROLE_CHANGE_PASSWORD_GROUP\r\nBACKUP_RESTORE_GROUP\r\nDATABASE_LOGOUT_GROUP\r\nDATABASE_OBJECT_CHANGE_GROUP\r\nDATABASE_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_OBJECT_PERMISSION_CHANGE_GROUP\r\nDATABASE_OPERATION_GROUP\r\nDATABASE_PERMISSION_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_IMPERSONATION_GROUP\r\nDATABASE_ROLE_MEMBER_CHANGE_GROUP\r\nFAILED_DATABASE_AUTHENTICATION_GROUP\r\nSCHEMA_OBJECT_ACCESS_GROUP\r\nSCHEMA_OBJECT_CHANGE_GROUP\r\nSCHEMA_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nSCHEMA_OBJECT_PERMISSION_CHANGE_GROUP\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP\r\nUSER_CHANGE_PASSWORD_GROUP\r\nBATCH_STARTED_GROUP\r\nBATCH_COMPLETED_GROUP\r\n\r\nThese are groups that cover all sql statements and stored procedures executed against the database, and should not be used in combination with other groups as this will result in duplicate audit logs.\r\n\r\nFor more information, see [Database-Level Audit Action Groups](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-action-groups).\r\n\r\nFor Database auditing policy, specific Actions can also be specified (note that Actions cannot be specified for Server auditing policy). The supported actions to audit are:\r\nSELECT\r\nUPDATE\r\nINSERT\r\nDELETE\r\nEXECUTE\r\nRECEIVE\r\nREFERENCES\r\n\r\nThe general form for defining an action to be audited is:\r\n{action} ON {object} BY {principal}\r\n\r\nNote that <object> in the above format can refer to an object like a table, view, or stored procedure, or an entire database or schema. For the latter cases, the forms DATABASE::{db_name} and SCHEMA::{schema_name} are used, respectively.\r\n\r\nFor example:\r\nSELECT on dbo.myTable by public\r\nSELECT on DATABASE::myDatabase by public\r\nSELECT on SCHEMA::mySchema by public\r\n\r\nFor more information, see [Database-Level Audit Actions](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-actions)"]
    #[serde(rename = "auditActionsAndGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub audit_actions_and_groups: Vec<String>,
    #[doc = "Specifies the blob storage subscription Id."]
    #[serde(rename = "storageAccountSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_subscription_id: Option<String>,
    #[doc = "Specifies whether storageAccountAccessKey value is the storage's secondary key."]
    #[serde(rename = "isStorageSecondaryKeyInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_storage_secondary_key_in_use: Option<bool>,
    #[doc = "Specifies whether audit events are sent to Azure Monitor. \r\nIn order to send the events to Azure Monitor, specify 'state' as 'Enabled' and 'isAzureMonitorTargetEnabled' as true.\r\n\r\nWhen using REST API to configure auditing, Diagnostic Settings with 'SQLSecurityAuditEvents' diagnostic logs category on the database should be also created.\r\nNote that for server level audit you should use the 'master' database as {databaseName}.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/{databaseName}/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isAzureMonitorTargetEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_monitor_target_enabled: Option<bool>,
}
impl SqlPoolBlobAuditingPolicyProperties {
    pub fn new(state: sql_pool_blob_auditing_policy_properties::State) -> Self {
        Self {
            state,
            storage_endpoint: None,
            storage_account_access_key: None,
            retention_days: None,
            audit_actions_and_groups: Vec::new(),
            storage_account_subscription_id: None,
            is_storage_secondary_key_in_use: None,
            is_azure_monitor_target_enabled: None,
        }
    }
}
pub mod sql_pool_blob_auditing_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "The response to a list Sql pool operations request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolBlobAuditingPolicySqlPoolOperationListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlPoolOperation>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlPoolBlobAuditingPolicySqlPoolOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SqlPoolBlobAuditingPolicySqlPoolOperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Sql pool column resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolColumn {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Sql pool column properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlPoolColumnProperties>,
}
impl SqlPoolColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of Sql pool columns."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolColumnListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlPoolColumn>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlPoolColumnListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SqlPoolColumnListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sql pool column properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolColumnProperties {
    #[doc = "The column data type."]
    #[serde(rename = "columnType", default, skip_serializing_if = "Option::is_none")]
    pub column_type: Option<sql_pool_column_properties::ColumnType>,
    #[doc = "Indicates whether column value is computed or not"]
    #[serde(rename = "isComputed", default, skip_serializing_if = "Option::is_none")]
    pub is_computed: Option<bool>,
}
impl SqlPoolColumnProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_pool_column_properties {
    use super::*;
    #[doc = "The column data type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ColumnType")]
    pub enum ColumnType {
        #[serde(rename = "image")]
        Image,
        #[serde(rename = "text")]
        Text,
        #[serde(rename = "uniqueidentifier")]
        Uniqueidentifier,
        #[serde(rename = "date")]
        Date,
        #[serde(rename = "time")]
        Time,
        #[serde(rename = "datetime2")]
        Datetime2,
        #[serde(rename = "datetimeoffset")]
        Datetimeoffset,
        #[serde(rename = "tinyint")]
        Tinyint,
        #[serde(rename = "smallint")]
        Smallint,
        #[serde(rename = "int")]
        Int,
        #[serde(rename = "smalldatetime")]
        Smalldatetime,
        #[serde(rename = "real")]
        Real,
        #[serde(rename = "money")]
        Money,
        #[serde(rename = "datetime")]
        Datetime,
        #[serde(rename = "float")]
        Float,
        #[serde(rename = "sql_variant")]
        SqlVariant,
        #[serde(rename = "ntext")]
        Ntext,
        #[serde(rename = "bit")]
        Bit,
        #[serde(rename = "decimal")]
        Decimal,
        #[serde(rename = "numeric")]
        Numeric,
        #[serde(rename = "smallmoney")]
        Smallmoney,
        #[serde(rename = "bigint")]
        Bigint,
        #[serde(rename = "hierarchyid")]
        Hierarchyid,
        #[serde(rename = "geometry")]
        Geometry,
        #[serde(rename = "geography")]
        Geography,
        #[serde(rename = "varbinary")]
        Varbinary,
        #[serde(rename = "varchar")]
        Varchar,
        #[serde(rename = "binary")]
        Binary,
        #[serde(rename = "char")]
        Char,
        #[serde(rename = "timestamp")]
        Timestamp,
        #[serde(rename = "nvarchar")]
        Nvarchar,
        #[serde(rename = "nchar")]
        Nchar,
        #[serde(rename = "xml")]
        Xml,
        #[serde(rename = "sysname")]
        Sysname,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ColumnType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ColumnType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ColumnType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Image => serializer.serialize_unit_variant("ColumnType", 0u32, "image"),
                Self::Text => serializer.serialize_unit_variant("ColumnType", 1u32, "text"),
                Self::Uniqueidentifier => serializer.serialize_unit_variant("ColumnType", 2u32, "uniqueidentifier"),
                Self::Date => serializer.serialize_unit_variant("ColumnType", 3u32, "date"),
                Self::Time => serializer.serialize_unit_variant("ColumnType", 4u32, "time"),
                Self::Datetime2 => serializer.serialize_unit_variant("ColumnType", 5u32, "datetime2"),
                Self::Datetimeoffset => serializer.serialize_unit_variant("ColumnType", 6u32, "datetimeoffset"),
                Self::Tinyint => serializer.serialize_unit_variant("ColumnType", 7u32, "tinyint"),
                Self::Smallint => serializer.serialize_unit_variant("ColumnType", 8u32, "smallint"),
                Self::Int => serializer.serialize_unit_variant("ColumnType", 9u32, "int"),
                Self::Smalldatetime => serializer.serialize_unit_variant("ColumnType", 10u32, "smalldatetime"),
                Self::Real => serializer.serialize_unit_variant("ColumnType", 11u32, "real"),
                Self::Money => serializer.serialize_unit_variant("ColumnType", 12u32, "money"),
                Self::Datetime => serializer.serialize_unit_variant("ColumnType", 13u32, "datetime"),
                Self::Float => serializer.serialize_unit_variant("ColumnType", 14u32, "float"),
                Self::SqlVariant => serializer.serialize_unit_variant("ColumnType", 15u32, "sql_variant"),
                Self::Ntext => serializer.serialize_unit_variant("ColumnType", 16u32, "ntext"),
                Self::Bit => serializer.serialize_unit_variant("ColumnType", 17u32, "bit"),
                Self::Decimal => serializer.serialize_unit_variant("ColumnType", 18u32, "decimal"),
                Self::Numeric => serializer.serialize_unit_variant("ColumnType", 19u32, "numeric"),
                Self::Smallmoney => serializer.serialize_unit_variant("ColumnType", 20u32, "smallmoney"),
                Self::Bigint => serializer.serialize_unit_variant("ColumnType", 21u32, "bigint"),
                Self::Hierarchyid => serializer.serialize_unit_variant("ColumnType", 22u32, "hierarchyid"),
                Self::Geometry => serializer.serialize_unit_variant("ColumnType", 23u32, "geometry"),
                Self::Geography => serializer.serialize_unit_variant("ColumnType", 24u32, "geography"),
                Self::Varbinary => serializer.serialize_unit_variant("ColumnType", 25u32, "varbinary"),
                Self::Varchar => serializer.serialize_unit_variant("ColumnType", 26u32, "varchar"),
                Self::Binary => serializer.serialize_unit_variant("ColumnType", 27u32, "binary"),
                Self::Char => serializer.serialize_unit_variant("ColumnType", 28u32, "char"),
                Self::Timestamp => serializer.serialize_unit_variant("ColumnType", 29u32, "timestamp"),
                Self::Nvarchar => serializer.serialize_unit_variant("ColumnType", 30u32, "nvarchar"),
                Self::Nchar => serializer.serialize_unit_variant("ColumnType", 31u32, "nchar"),
                Self::Xml => serializer.serialize_unit_variant("ColumnType", 32u32, "xml"),
                Self::Sysname => serializer.serialize_unit_variant("ColumnType", 33u32, "sysname"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A Sql pool connection policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolConnectionPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource kind."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties of a Sql pool connection policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlPoolConnectionPolicyProperties>,
}
impl SqlPoolConnectionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Sql pool connection policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolConnectionPolicyProperties {
    #[doc = "The state of security access."]
    #[serde(rename = "securityEnabledAccess", default, skip_serializing_if = "Option::is_none")]
    pub security_enabled_access: Option<String>,
    #[doc = "The fully qualified host name of the auditing proxy."]
    #[serde(rename = "proxyDnsName", default, skip_serializing_if = "Option::is_none")]
    pub proxy_dns_name: Option<String>,
    #[doc = "The port number of the auditing proxy."]
    #[serde(rename = "proxyPort", default, skip_serializing_if = "Option::is_none")]
    pub proxy_port: Option<String>,
    #[doc = "The visibility of the auditing proxy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[doc = "Whether server default is enabled or disabled."]
    #[serde(rename = "useServerDefault", default, skip_serializing_if = "Option::is_none")]
    pub use_server_default: Option<String>,
    #[doc = "The state of proxy redirection."]
    #[serde(rename = "redirectionState", default, skip_serializing_if = "Option::is_none")]
    pub redirection_state: Option<String>,
    #[doc = "The connection policy state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl SqlPoolConnectionPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of SQL pools"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolInfoListResult {
    #[doc = "Link to the next page of results"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of SQL pools"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlPool>,
}
impl azure_core::Continuable for SqlPoolInfoListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SqlPoolInfoListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Sql pool operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolOperation {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a Sql pool operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlPoolOperationProperties>,
}
impl SqlPoolOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a Sql pool operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolOperationProperties {
    #[doc = "The name of the Sql pool the operation is being performed on."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The name of operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The friendly name of operation."]
    #[serde(rename = "operationFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub operation_friendly_name: Option<String>,
    #[doc = "The percentage of the operation completed."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "The name of the server."]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "The operation start time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The operation state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<sql_pool_operation_properties::State>,
    #[doc = "The operation error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "The operation error description."]
    #[serde(rename = "errorDescription", default, skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[doc = "The operation error severity."]
    #[serde(rename = "errorSeverity", default, skip_serializing_if = "Option::is_none")]
    pub error_severity: Option<i32>,
    #[doc = "Whether or not the error is a user error."]
    #[serde(rename = "isUserError", default, skip_serializing_if = "Option::is_none")]
    pub is_user_error: Option<bool>,
    #[doc = "The estimated completion time of the operation."]
    #[serde(rename = "estimatedCompletionTime", with = "azure_core::date::rfc3339::option")]
    pub estimated_completion_time: Option<time::OffsetDateTime>,
    #[doc = "The operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether the operation can be cancelled."]
    #[serde(rename = "isCancellable", default, skip_serializing_if = "Option::is_none")]
    pub is_cancellable: Option<bool>,
}
impl SqlPoolOperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_pool_operation_properties {
    use super::*;
    #[doc = "The operation state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Pending,
        InProgress,
        Succeeded,
        Failed,
        CancelInProgress,
        Cancelled,
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
                Self::Pending => serializer.serialize_unit_variant("State", 0u32, "Pending"),
                Self::InProgress => serializer.serialize_unit_variant("State", 1u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("State", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("State", 3u32, "Failed"),
                Self::CancelInProgress => serializer.serialize_unit_variant("State", 4u32, "CancelInProgress"),
                Self::Cancelled => serializer.serialize_unit_variant("State", 5u32, "Cancelled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A SQL Analytics pool patch info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolPatchInfo {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "SQL pool SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Properties of a SQL Analytics pool"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlPoolResourceProperties>,
}
impl SqlPoolPatchInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a SQL Analytics pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolResourceProperties {
    #[doc = "Maximum size in bytes"]
    #[serde(rename = "maxSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_bytes: Option<i64>,
    #[doc = "Collation mode"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    #[doc = "Source database to create from"]
    #[serde(rename = "sourceDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub source_database_id: Option<String>,
    #[doc = "Backup database to restore from"]
    #[serde(rename = "recoverableDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub recoverable_database_id: Option<String>,
    #[doc = "Resource state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Resource status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Snapshot time to restore"]
    #[serde(rename = "restorePointInTime", with = "azure_core::date::rfc3339::option")]
    pub restore_point_in_time: Option<time::OffsetDateTime>,
    #[doc = "Specifies the mode of sql pool creation.\n\nDefault: regular sql pool creation.\n\nPointInTimeRestore: Creates a sql pool by restoring a point in time backup of an existing sql pool. sourceDatabaseId must be specified as the resource ID of the existing sql pool, and restorePointInTime must be specified.\n\nRecovery: Creates a sql pool by a geo-replicated backup. sourceDatabaseId  must be specified as the recoverableDatabaseId to restore.\n\nRestore: Creates a sql pool by restoring a backup of a deleted sql  pool. SourceDatabaseId should be the sql pool's original resource ID. SourceDatabaseId and sourceDatabaseDeletionDate must be specified."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<sql_pool_resource_properties::CreateMode>,
    #[doc = "Date the SQL pool was created"]
    #[serde(rename = "creationDate", with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The storage account type used to store backups for this sql pool."]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<sql_pool_resource_properties::StorageAccountType>,
    #[doc = "Specifies the time that the sql pool was deleted"]
    #[serde(rename = "sourceDatabaseDeletionDate", with = "azure_core::date::rfc3339::option")]
    pub source_database_deletion_date: Option<time::OffsetDateTime>,
}
impl SqlPoolResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_pool_resource_properties {
    use super::*;
    #[doc = "Specifies the mode of sql pool creation.\n\nDefault: regular sql pool creation.\n\nPointInTimeRestore: Creates a sql pool by restoring a point in time backup of an existing sql pool. sourceDatabaseId must be specified as the resource ID of the existing sql pool, and restorePointInTime must be specified.\n\nRecovery: Creates a sql pool by a geo-replicated backup. sourceDatabaseId  must be specified as the recoverableDatabaseId to restore.\n\nRestore: Creates a sql pool by restoring a backup of a deleted sql  pool. SourceDatabaseId should be the sql pool's original resource ID. SourceDatabaseId and sourceDatabaseDeletionDate must be specified."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreateMode")]
    pub enum CreateMode {
        Default,
        PointInTimeRestore,
        Recovery,
        Restore,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreateMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreateMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreateMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("CreateMode", 0u32, "Default"),
                Self::PointInTimeRestore => serializer.serialize_unit_variant("CreateMode", 1u32, "PointInTimeRestore"),
                Self::Recovery => serializer.serialize_unit_variant("CreateMode", 2u32, "Recovery"),
                Self::Restore => serializer.serialize_unit_variant("CreateMode", 3u32, "Restore"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The storage account type used to store backups for this sql pool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageAccountType")]
    pub enum StorageAccountType {
        #[serde(rename = "GRS")]
        Grs,
        #[serde(rename = "LRS")]
        Lrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageAccountType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageAccountType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageAccountType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Grs => serializer.serialize_unit_variant("StorageAccountType", 0u32, "GRS"),
                Self::Lrs => serializer.serialize_unit_variant("StorageAccountType", 1u32, "LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for StorageAccountType {
        fn default() -> Self {
            Self::Grs
        }
    }
}
#[doc = "A Sql pool schema resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolSchema {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
}
impl SqlPoolSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of Sql pool schemas."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolSchemaListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlPoolSchema>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlPoolSchemaListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SqlPoolSchemaListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Sql pool security alert policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolSecurityAlertPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a security alert policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAlertPolicyProperties>,
}
impl SqlPoolSecurityAlertPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Sql pool table resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolTable {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
}
impl SqlPoolTable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of Sql pool tables."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolTableListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlPoolTable>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlPoolTableListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SqlPoolTableListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Sql pool usages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolUsage {
    #[doc = "The name of the usage metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "The usage metric display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The current value of the usage metric."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[doc = "The current limit of the usage metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[doc = "The units of the usage metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The next reset time for the usage metric (ISO8601 format)."]
    #[serde(rename = "nextResetTime", with = "azure_core::date::rfc3339::option")]
    pub next_reset_time: Option<time::OffsetDateTime>,
}
impl SqlPoolUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list Sql pool usages request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlPoolUsageListResult {
    #[doc = "The list of usages for the Sql pool."]
    pub value: Vec<SqlPoolUsage>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlPoolUsageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SqlPoolUsageListResult {
    pub fn new(value: Vec<SqlPoolUsage>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A Sql pool vulnerability assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolVulnerabilityAssessment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a Sql pool Vulnerability Assessment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlPoolVulnerabilityAssessmentProperties>,
}
impl SqlPoolVulnerabilityAssessment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the Sql pool's vulnerability assessments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolVulnerabilityAssessmentListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlPoolVulnerabilityAssessment>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlPoolVulnerabilityAssessmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SqlPoolVulnerabilityAssessmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Sql pool Vulnerability Assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolVulnerabilityAssessmentProperties {
    #[doc = "A blob storage container path to hold the scan results (e.g. https://myStorage.blob.core.windows.net/VaScans/).  It is required if server level vulnerability assessment policy doesn't set"]
    #[serde(rename = "storageContainerPath", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_path: Option<String>,
    #[doc = "A shared access signature (SAS Key) that has write access to the blob container specified in 'storageContainerPath' parameter. If 'storageAccountAccessKey' isn't specified, StorageContainerSasKey is required."]
    #[serde(rename = "storageContainerSasKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_sas_key: Option<String>,
    #[doc = "Specifies the identifier key of the storage account for vulnerability assessment scan results. If 'StorageContainerSasKey' isn't specified, storageAccountAccessKey is required."]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Properties of a Vulnerability Assessment recurring scans."]
    #[serde(rename = "recurringScans", default, skip_serializing_if = "Option::is_none")]
    pub recurring_scans: Option<VulnerabilityAssessmentRecurringScansProperties>,
}
impl SqlPoolVulnerabilityAssessmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Sql pool vulnerability assessment rule baseline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolVulnerabilityAssessmentRuleBaseline {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a Sql pool vulnerability assessment rule baseline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlPoolVulnerabilityAssessmentRuleBaselineProperties>,
}
impl SqlPoolVulnerabilityAssessmentRuleBaseline {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for an Sql pool vulnerability assessment rule baseline's result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlPoolVulnerabilityAssessmentRuleBaselineItem {
    #[doc = "The rule baseline result"]
    pub result: Vec<String>,
}
impl SqlPoolVulnerabilityAssessmentRuleBaselineItem {
    pub fn new(result: Vec<String>) -> Self {
        Self { result }
    }
}
#[doc = "Properties of a Sql pool vulnerability assessment rule baseline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlPoolVulnerabilityAssessmentRuleBaselineProperties {
    #[doc = "The rule baseline result"]
    #[serde(rename = "baselineResults")]
    pub baseline_results: Vec<SqlPoolVulnerabilityAssessmentRuleBaselineItem>,
}
impl SqlPoolVulnerabilityAssessmentRuleBaselineProperties {
    pub fn new(baseline_results: Vec<SqlPoolVulnerabilityAssessmentRuleBaselineItem>) -> Self {
        Self { baseline_results }
    }
}
#[doc = "Properties of the export operation's result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolVulnerabilityAssessmentScanExportProperties {
    #[doc = "Location of the exported report (e.g. https://myStorage.blob.core.windows.net/VaScans/scans/serverName/databaseName/scan_scanId.xlsx)."]
    #[serde(rename = "exportedReportLocation", default, skip_serializing_if = "Option::is_none")]
    pub exported_report_location: Option<String>,
}
impl SqlPoolVulnerabilityAssessmentScanExportProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Sql pool Vulnerability Assessment scan export resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlPoolVulnerabilityAssessmentScansExport {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the export operation's result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlPoolVulnerabilityAssessmentScanExportProperties>,
}
impl SqlPoolVulnerabilityAssessmentScansExport {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Ssis environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SsisEnvironment {
    #[serde(flatten)]
    pub ssis_object_metadata: SsisObjectMetadata,
    #[doc = "Folder id which contains environment."]
    #[serde(rename = "folderId", default, skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<i64>,
    #[doc = "Variable in environment"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<SsisVariable>,
}
impl SsisEnvironment {
    pub fn new(ssis_object_metadata: SsisObjectMetadata) -> Self {
        Self {
            ssis_object_metadata,
            folder_id: None,
            variables: Vec::new(),
        }
    }
}
#[doc = "Ssis environment reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SsisEnvironmentReference {
    #[doc = "Environment reference id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Environment folder name."]
    #[serde(rename = "environmentFolderName", default, skip_serializing_if = "Option::is_none")]
    pub environment_folder_name: Option<String>,
    #[doc = "Environment name."]
    #[serde(rename = "environmentName", default, skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[doc = "Reference type"]
    #[serde(rename = "referenceType", default, skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<String>,
}
impl SsisEnvironmentReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Ssis folder."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SsisFolder {
    #[serde(flatten)]
    pub ssis_object_metadata: SsisObjectMetadata,
}
impl SsisFolder {
    pub fn new(ssis_object_metadata: SsisObjectMetadata) -> Self {
        Self { ssis_object_metadata }
    }
}
#[doc = "SSIS object metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SsisObjectMetadata {
    #[doc = "The type of SSIS object metadata."]
    #[serde(rename = "type")]
    pub type_: SsisObjectMetadataType,
    #[doc = "Metadata id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Metadata name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Metadata description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl SsisObjectMetadata {
    pub fn new(type_: SsisObjectMetadataType) -> Self {
        Self {
            type_,
            id: None,
            name: None,
            description: None,
        }
    }
}
#[doc = "A list of SSIS object metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SsisObjectMetadataListResponse {
    #[doc = "List of SSIS object metadata."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SsisObjectMetadata>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SsisObjectMetadataListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SsisObjectMetadataStatusResponse {
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The operation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    #[doc = "The operation error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
impl SsisObjectMetadataStatusResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of SSIS object metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SsisObjectMetadataType")]
pub enum SsisObjectMetadataType {
    Folder,
    Project,
    Package,
    Environment,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SsisObjectMetadataType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SsisObjectMetadataType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SsisObjectMetadataType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Folder => serializer.serialize_unit_variant("SsisObjectMetadataType", 0u32, "Folder"),
            Self::Project => serializer.serialize_unit_variant("SsisObjectMetadataType", 1u32, "Project"),
            Self::Package => serializer.serialize_unit_variant("SsisObjectMetadataType", 2u32, "Package"),
            Self::Environment => serializer.serialize_unit_variant("SsisObjectMetadataType", 3u32, "Environment"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Ssis Package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SsisPackage {
    #[serde(flatten)]
    pub ssis_object_metadata: SsisObjectMetadata,
    #[doc = "Folder id which contains package."]
    #[serde(rename = "folderId", default, skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<i64>,
    #[doc = "Project version which contains package."]
    #[serde(rename = "projectVersion", default, skip_serializing_if = "Option::is_none")]
    pub project_version: Option<i64>,
    #[doc = "Project id which contains package."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    #[doc = "Parameters in package"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<SsisParameter>,
}
impl SsisPackage {
    pub fn new(ssis_object_metadata: SsisObjectMetadata) -> Self {
        Self {
            ssis_object_metadata,
            folder_id: None,
            project_version: None,
            project_id: None,
            parameters: Vec::new(),
        }
    }
}
#[doc = "Ssis parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SsisParameter {
    #[doc = "Parameter id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Parameter name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Parameter description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Parameter type."]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[doc = "Whether parameter is required."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[doc = "Whether parameter is sensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    #[doc = "Design default value of parameter."]
    #[serde(rename = "designDefaultValue", default, skip_serializing_if = "Option::is_none")]
    pub design_default_value: Option<String>,
    #[doc = "Default value of parameter."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "Default sensitive value of parameter."]
    #[serde(rename = "sensitiveDefaultValue", default, skip_serializing_if = "Option::is_none")]
    pub sensitive_default_value: Option<String>,
    #[doc = "Parameter value type."]
    #[serde(rename = "valueType", default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
    #[doc = "Parameter value set."]
    #[serde(rename = "valueSet", default, skip_serializing_if = "Option::is_none")]
    pub value_set: Option<bool>,
    #[doc = "Parameter reference variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable: Option<String>,
}
impl SsisParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Ssis project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SsisProject {
    #[serde(flatten)]
    pub ssis_object_metadata: SsisObjectMetadata,
    #[doc = "Folder id which contains project."]
    #[serde(rename = "folderId", default, skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<i64>,
    #[doc = "Project version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "Environment reference in project"]
    #[serde(rename = "environmentRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_refs: Vec<SsisEnvironmentReference>,
    #[doc = "Parameters in project"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<SsisParameter>,
}
impl SsisProject {
    pub fn new(ssis_object_metadata: SsisObjectMetadata) -> Self {
        Self {
            ssis_object_metadata,
            folder_id: None,
            version: None,
            environment_refs: Vec::new(),
            parameters: Vec::new(),
        }
    }
}
#[doc = "Ssis variable."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SsisVariable {
    #[doc = "Variable id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Variable name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Variable description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Variable type."]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[doc = "Whether variable is sensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    #[doc = "Variable value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Variable sensitive value."]
    #[serde(rename = "sensitiveValue", default, skip_serializing_if = "Option::is_none")]
    pub sensitive_value: Option<String>,
}
impl SsisVariable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Synapse nested resource, which belongs to a workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A database query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopQueries {
    #[doc = "The function that is used to aggregate each query's metrics."]
    #[serde(rename = "aggregationFunction", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_function: Option<top_queries::AggregationFunction>,
    #[doc = "The execution type that is used to filter the query instances that are returned."]
    #[serde(rename = "executionType", default, skip_serializing_if = "Option::is_none")]
    pub execution_type: Option<top_queries::ExecutionType>,
    #[doc = "The duration of the interval (ISO8601 duration format)."]
    #[serde(rename = "intervalType", default, skip_serializing_if = "Option::is_none")]
    pub interval_type: Option<String>,
    #[doc = "The number of requested queries."]
    #[serde(rename = "numberOfTopQueries", default, skip_serializing_if = "Option::is_none")]
    pub number_of_top_queries: Option<f64>,
    #[doc = "The start time for queries that are returned (ISO8601 format)"]
    #[serde(rename = "observationStartTime", with = "azure_core::date::rfc3339::option")]
    pub observation_start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time for queries that are returned (ISO8601 format)"]
    #[serde(rename = "observationEndTime", with = "azure_core::date::rfc3339::option")]
    pub observation_end_time: Option<time::OffsetDateTime>,
    #[doc = "The type of metric to use for ordering the top metrics."]
    #[serde(rename = "observedMetric", default, skip_serializing_if = "Option::is_none")]
    pub observed_metric: Option<top_queries::ObservedMetric>,
    #[doc = "The list of queries."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub queries: Vec<QueryStatistic>,
}
impl TopQueries {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod top_queries {
    use super::*;
    #[doc = "The function that is used to aggregate each query's metrics."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AggregationFunction {
        #[serde(rename = "min")]
        Min,
        #[serde(rename = "max")]
        Max,
        #[serde(rename = "avg")]
        Avg,
        #[serde(rename = "sum")]
        Sum,
    }
    #[doc = "The execution type that is used to filter the query instances that are returned."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ExecutionType {
        #[serde(rename = "any")]
        Any,
        #[serde(rename = "regular")]
        Regular,
        #[serde(rename = "irregular")]
        Irregular,
        #[serde(rename = "aborted")]
        Aborted,
        #[serde(rename = "exception")]
        Exception,
    }
    #[doc = "The type of metric to use for ordering the top metrics."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ObservedMetric {
        #[serde(rename = "cpu")]
        Cpu,
        #[serde(rename = "io")]
        Io,
        #[serde(rename = "logio")]
        Logio,
        #[serde(rename = "duration")]
        Duration,
        #[serde(rename = "executionCount")]
        ExecutionCount,
    }
}
#[doc = "Represents the response to a get top queries request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopQueriesListResult {
    #[doc = "The list of top queries."]
    pub value: Vec<TopQueries>,
}
impl TopQueriesListResult {
    pub fn new(value: Vec<TopQueries>) -> Self {
        Self { value }
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
#[doc = "Represents a Sql pool transparent data encryption configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransparentDataEncryption {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Represents the properties of a database transparent data encryption."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TransparentDataEncryptionProperties>,
}
impl TransparentDataEncryption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of transparent data encryption configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransparentDataEncryptionListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TransparentDataEncryption>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TransparentDataEncryptionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TransparentDataEncryptionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of a database transparent data encryption."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransparentDataEncryptionProperties {
    #[doc = "The status of the database transparent data encryption."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<transparent_data_encryption_properties::Status>,
}
impl TransparentDataEncryptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod transparent_data_encryption_properties {
    use super::*;
    #[doc = "The status of the database transparent data encryption."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Enabled,
        Disabled,
    }
}
#[doc = "Update integration runtime node request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateIntegrationRuntimeNodeRequest {
    #[doc = "The number of concurrent jobs permitted to run on the integration runtime node. Values between 1 and maxConcurrentJobs(inclusive) are allowed."]
    #[serde(rename = "concurrentJobsLimit", default, skip_serializing_if = "Option::is_none")]
    pub concurrent_jobs_limit: Option<i32>,
}
impl UpdateIntegrationRuntimeNodeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update integration runtime request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateIntegrationRuntimeRequest {
    #[doc = "The state of integration runtime auto update."]
    #[serde(rename = "autoUpdate", default, skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<IntegrationRuntimeAutoUpdate>,
    #[doc = "The time offset (in hours) in the day, e.g., PT03H is 3 hours. The integration runtime auto update will happen on that time."]
    #[serde(rename = "updateDelayOffset", default, skip_serializing_if = "Option::is_none")]
    pub update_delay_offset: Option<String>,
}
impl UpdateIntegrationRuntimeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The User Assigned Managed Identities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedManagedIdentities {}
impl UserAssignedManagedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User Assigned Managed Identity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedManagedIdentity {
    #[doc = "The client ID."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The principal ID."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
}
impl UserAssignedManagedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual Network Profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkProfile {
    #[doc = "Subnet ID used for computes in workspace"]
    #[serde(rename = "computeSubnetId", default, skip_serializing_if = "Option::is_none")]
    pub compute_subnet_id: Option<String>,
}
impl VirtualNetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Vulnerability Assessment recurring scans."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VulnerabilityAssessmentRecurringScansProperties {
    #[doc = "Recurring scans state."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Specifies that the schedule scan notification will be is sent to the subscription administrators."]
    #[serde(rename = "emailSubscriptionAdmins", default, skip_serializing_if = "Option::is_none")]
    pub email_subscription_admins: Option<bool>,
    #[doc = "Specifies an array of e-mail addresses to which the scan notification is sent."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<String>,
}
impl VulnerabilityAssessmentRecurringScansProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a vulnerability assessment scan error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VulnerabilityAssessmentScanError {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl VulnerabilityAssessmentScanError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A vulnerability assessment scan record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VulnerabilityAssessmentScanRecord {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a vulnerability assessment scan record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VulnerabilityAssessmentScanRecordProperties>,
}
impl VulnerabilityAssessmentScanRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of vulnerability assessment scan records."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VulnerabilityAssessmentScanRecordListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VulnerabilityAssessmentScanRecord>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VulnerabilityAssessmentScanRecordListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VulnerabilityAssessmentScanRecordListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a vulnerability assessment scan record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VulnerabilityAssessmentScanRecordProperties {
    #[doc = "The scan ID."]
    #[serde(rename = "scanId", default, skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[doc = "The scan trigger type."]
    #[serde(rename = "triggerType", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<vulnerability_assessment_scan_record_properties::TriggerType>,
    #[doc = "The scan status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<vulnerability_assessment_scan_record_properties::State>,
    #[doc = "The scan start time (UTC)."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The scan end time (UTC)."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The scan errors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<VulnerabilityAssessmentScanError>,
    #[doc = "The scan results storage container path."]
    #[serde(rename = "storageContainerPath", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_path: Option<String>,
    #[doc = "The number of failed security checks."]
    #[serde(rename = "numberOfFailedSecurityChecks", default, skip_serializing_if = "Option::is_none")]
    pub number_of_failed_security_checks: Option<i32>,
}
impl VulnerabilityAssessmentScanRecordProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vulnerability_assessment_scan_record_properties {
    use super::*;
    #[doc = "The scan trigger type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TriggerType")]
    pub enum TriggerType {
        OnDemand,
        Recurring,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TriggerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TriggerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TriggerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::OnDemand => serializer.serialize_unit_variant("TriggerType", 0u32, "OnDemand"),
                Self::Recurring => serializer.serialize_unit_variant("TriggerType", 1u32, "Recurring"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The scan status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Passed,
        Failed,
        FailedToRun,
        InProgress,
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
                Self::Passed => serializer.serialize_unit_variant("State", 0u32, "Passed"),
                Self::Failed => serializer.serialize_unit_variant("State", 1u32, "Failed"),
                Self::FailedToRun => serializer.serialize_unit_variant("State", 2u32, "FailedToRun"),
                Self::InProgress => serializer.serialize_unit_variant("State", 3u32, "InProgress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Workload classifier operations for a data warehouse"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadClassifier {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Workload classifier definition. For more information look at sys.workload_management_workload_classifiers (DMV)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadClassifierProperties>,
}
impl WorkloadClassifier {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of workload classifiers for a workload group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadClassifierListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkloadClassifier>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadClassifierListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkloadClassifierListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload classifier definition. For more information look at sys.workload_management_workload_classifiers (DMV)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadClassifierProperties {
    #[doc = "The workload classifier member name."]
    #[serde(rename = "memberName")]
    pub member_name: String,
    #[doc = "The workload classifier label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "The workload classifier context."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "The workload classifier start time for classification."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The workload classifier end time for classification."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The workload classifier importance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub importance: Option<String>,
}
impl WorkloadClassifierProperties {
    pub fn new(member_name: String) -> Self {
        Self {
            member_name,
            label: None,
            context: None,
            start_time: None,
            end_time: None,
            importance: None,
        }
    }
}
#[doc = "Workload group operations for a sql pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Workload group definition. For more information look at sys.workload_management_workload_groups (DMV)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadGroupProperties>,
}
impl WorkloadGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of workload groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadGroupListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkloadGroup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkloadGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload group definition. For more information look at sys.workload_management_workload_groups (DMV)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadGroupProperties {
    #[doc = "The workload group minimum percentage resource."]
    #[serde(rename = "minResourcePercent")]
    pub min_resource_percent: i32,
    #[doc = "The workload group cap percentage resource."]
    #[serde(rename = "maxResourcePercent")]
    pub max_resource_percent: i32,
    #[doc = "The workload group request minimum grant percentage."]
    #[serde(rename = "minResourcePercentPerRequest")]
    pub min_resource_percent_per_request: f64,
    #[doc = "The workload group request maximum grant percentage."]
    #[serde(rename = "maxResourcePercentPerRequest", default, skip_serializing_if = "Option::is_none")]
    pub max_resource_percent_per_request: Option<f64>,
    #[doc = "The workload group importance level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub importance: Option<String>,
    #[doc = "The workload group query execution timeout."]
    #[serde(rename = "queryExecutionTimeout", default, skip_serializing_if = "Option::is_none")]
    pub query_execution_timeout: Option<i32>,
}
impl WorkloadGroupProperties {
    pub fn new(min_resource_percent: i32, max_resource_percent: i32, min_resource_percent_per_request: f64) -> Self {
        Self {
            min_resource_percent,
            max_resource_percent,
            min_resource_percent_per_request,
            max_resource_percent_per_request: None,
            importance: None,
            query_execution_timeout: None,
        }
    }
}
#[doc = "A workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Workspace properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
    #[doc = "The workspace managed identity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedIdentity>,
}
impl Workspace {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
        }
    }
}
#[doc = "Workspace active directory administrator"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceAadAdminInfo {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Workspace active directory administrator properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AadAdminProperties>,
}
impl WorkspaceAadAdminInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of workspaces"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceInfoListResult {
    #[doc = "Link to the next page of results"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of workspaces"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
}
impl azure_core::Continuable for WorkspaceInfoListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkspaceInfoListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the customer managed key associated with the workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceKeyDetails {
    #[doc = "Workspace Key sub-resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Workspace Key sub-resource key vault url"]
    #[serde(rename = "keyVaultUrl", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_url: Option<String>,
}
impl WorkspaceKeyDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workspace patch details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspacePatchInfo {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The workspace managed identity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedIdentity>,
    #[doc = "Workspace patch properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspacePatchProperties>,
}
impl WorkspacePatchInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workspace patch properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspacePatchProperties {
    #[doc = "SQL administrator login password"]
    #[serde(rename = "sqlAdministratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
    pub sql_administrator_login_password: Option<String>,
    #[doc = "Managed Virtual Network Settings"]
    #[serde(rename = "managedVirtualNetworkSettings", default, skip_serializing_if = "Option::is_none")]
    pub managed_virtual_network_settings: Option<ManagedVirtualNetworkSettings>,
    #[doc = "Git integration settings"]
    #[serde(rename = "workspaceRepositoryConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub workspace_repository_configuration: Option<WorkspaceRepositoryConfiguration>,
    #[doc = "Purview Configuration"]
    #[serde(rename = "purviewConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub purview_configuration: Option<PurviewConfiguration>,
    #[doc = "Resource provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Details of the encryption associated with the workspace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionDetails>,
    #[doc = "Enable or Disable public network access to workspace"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<workspace_patch_properties::PublicNetworkAccess>,
}
impl WorkspacePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workspace_patch_properties {
    use super::*;
    #[doc = "Enable or Disable public network access to workspace"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Workspace properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceProperties {
    #[doc = "Details of the data lake storage account associated with the workspace"]
    #[serde(rename = "defaultDataLakeStorage", default, skip_serializing_if = "Option::is_none")]
    pub default_data_lake_storage: Option<DataLakeStorageAccountDetails>,
    #[doc = "SQL administrator login password"]
    #[serde(rename = "sqlAdministratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
    pub sql_administrator_login_password: Option<String>,
    #[doc = "Workspace managed resource group. The resource group name uniquely identifies the resource group within the user subscriptionId. The resource group name must be no longer than 90 characters long, and must be alphanumeric characters (Char.IsLetterOrDigit()) and '-', '_', '(', ')' and'.'. Note that the name cannot end with '.'"]
    #[serde(rename = "managedResourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_name: Option<String>,
    #[doc = "Resource provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Login for workspace SQL active directory administrator"]
    #[serde(rename = "sqlAdministratorLogin", default, skip_serializing_if = "Option::is_none")]
    pub sql_administrator_login: Option<String>,
    #[doc = "Virtual Network Profile"]
    #[serde(rename = "virtualNetworkProfile", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_profile: Option<VirtualNetworkProfile>,
    #[doc = "Connectivity endpoints"]
    #[serde(rename = "connectivityEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub connectivity_endpoints: Option<serde_json::Value>,
    #[doc = "Setting this to 'default' will ensure that all compute for this workspace is in a virtual network managed on behalf of the user."]
    #[serde(rename = "managedVirtualNetwork", default, skip_serializing_if = "Option::is_none")]
    pub managed_virtual_network: Option<String>,
    #[doc = "Private endpoint connections to the workspace"]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "Details of the encryption associated with the workspace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionDetails>,
    #[doc = "The workspace unique identifier"]
    #[serde(rename = "workspaceUID", default, skip_serializing_if = "Option::is_none")]
    pub workspace_uid: Option<String>,
    #[doc = "Workspace level configs and feature flags"]
    #[serde(rename = "extraProperties", default, skip_serializing_if = "Option::is_none")]
    pub extra_properties: Option<serde_json::Value>,
    #[doc = "Managed Virtual Network Settings"]
    #[serde(rename = "managedVirtualNetworkSettings", default, skip_serializing_if = "Option::is_none")]
    pub managed_virtual_network_settings: Option<ManagedVirtualNetworkSettings>,
    #[doc = "Git integration settings"]
    #[serde(rename = "workspaceRepositoryConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub workspace_repository_configuration: Option<WorkspaceRepositoryConfiguration>,
    #[doc = "Purview Configuration"]
    #[serde(rename = "purviewConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub purview_configuration: Option<PurviewConfiguration>,
    #[doc = "The ADLA resource ID."]
    #[serde(rename = "adlaResourceId", default, skip_serializing_if = "Option::is_none")]
    pub adla_resource_id: Option<String>,
    #[doc = "Enable or Disable public network access to workspace"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<workspace_properties::PublicNetworkAccess>,
    #[doc = "Initial workspace AAD admin properties for a CSP subscription"]
    #[serde(rename = "cspWorkspaceAdminProperties", default, skip_serializing_if = "Option::is_none")]
    pub csp_workspace_admin_properties: Option<CspWorkspaceAdminProperties>,
    #[doc = "Workspace settings"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[doc = "Enable or Disable AzureADOnlyAuthentication on All Workspace subresource"]
    #[serde(rename = "azureADOnlyAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub azure_ad_only_authentication: Option<bool>,
    #[doc = "Is trustedServiceBypassEnabled for the workspace"]
    #[serde(rename = "trustedServiceBypassEnabled", default, skip_serializing_if = "Option::is_none")]
    pub trusted_service_bypass_enabled: Option<bool>,
}
impl WorkspaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workspace_properties {
    use super::*;
    #[doc = "Enable or Disable public network access to workspace"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for PublicNetworkAccess {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "Git integration settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceRepositoryConfiguration {
    #[doc = "Type of workspace repositoryID configuration. Example WorkspaceVSTSConfiguration, WorkspaceGitHubConfiguration"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "GitHub Enterprise host name. For example: https://github.mydomain.com"]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "Account name"]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "VSTS project name"]
    #[serde(rename = "projectName", default, skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[doc = "Repository name"]
    #[serde(rename = "repositoryName", default, skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[doc = "Collaboration branch"]
    #[serde(rename = "collaborationBranch", default, skip_serializing_if = "Option::is_none")]
    pub collaboration_branch: Option<String>,
    #[doc = "Root folder to use in the repository"]
    #[serde(rename = "rootFolder", default, skip_serializing_if = "Option::is_none")]
    pub root_folder: Option<String>,
    #[doc = "The last commit ID"]
    #[serde(rename = "lastCommitId", default, skip_serializing_if = "Option::is_none")]
    pub last_commit_id: Option<String>,
    #[doc = "The VSTS tenant ID"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl WorkspaceRepositoryConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}

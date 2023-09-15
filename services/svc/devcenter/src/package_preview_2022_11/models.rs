#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Action request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionRequest {
    #[doc = "The Catalog Item action id to execute"]
    #[serde(rename = "actionId")]
    pub action_id: String,
    #[doc = "Parameters object for the Action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl ActionRequest {
    pub fn new(action_id: String) -> Self {
        Self {
            action_id,
            parameters: None,
        }
    }
}
#[doc = "The type of action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ActionType")]
pub enum ActionType {
    Custom,
    Deploy,
    Delete,
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
            Self::Custom => serializer.serialize_unit_variant("ActionType", 0u32, "Custom"),
            Self::Deploy => serializer.serialize_unit_variant("ActionType", 1u32, "Deploy"),
            Self::Delete => serializer.serialize_unit_variant("ActionType", 2u32, "Delete"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A catalog item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogItem {
    #[doc = "Unique identifier of the catalog item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the catalog item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Name of the catalog."]
    #[serde(rename = "catalogName", default, skip_serializing_if = "Option::is_none")]
    pub catalog_name: Option<String>,
}
impl CatalogItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An action that can be taken on a catalog item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogItemAction {
    #[doc = "Unique identifier of the action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Description of the action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "JSON schema defining the parameters specific to the custom action"]
    #[serde(rename = "parametersSchema", default, skip_serializing_if = "Option::is_none")]
    pub parameters_schema: Option<String>,
    #[doc = "Input parameters passed to the action"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parameters: Vec<CatalogItemParameter>,
    #[doc = "The type of action."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ActionType>,
    #[doc = "Name of the custom action type"]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[doc = "The container image to use to execute the action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runner: Option<String>,
}
impl CatalogItemAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results of the catalog item list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CatalogItemListResult {
    #[doc = "Current page of results."]
    pub value: Vec<CatalogItem>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CatalogItemListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CatalogItemListResult {
    pub fn new(value: Vec<CatalogItem>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties of an Catalog Item parameter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogItemParameter {
    #[doc = "Unique ID of the parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Description of the parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Default value of the parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[doc = "The type of data a parameter accepts."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ParameterType>,
    #[doc = "Whether or not this parameter is read-only.  If true, default should have a value."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "Whether or not this parameter is required"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[doc = "An array of allowed values"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed: Vec<serde_json::Value>,
}
impl CatalogItemParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A catalog item version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogItemVersion {
    #[doc = "Unique identifier of the catalog item."]
    #[serde(rename = "catalogItemId", default, skip_serializing_if = "Option::is_none")]
    pub catalog_item_id: Option<String>,
    #[doc = "Name of the catalog item."]
    #[serde(rename = "catalogItemName", default, skip_serializing_if = "Option::is_none")]
    pub catalog_item_name: Option<String>,
    #[doc = "Name of the catalog."]
    #[serde(rename = "catalogName", default, skip_serializing_if = "Option::is_none")]
    pub catalog_name: Option<String>,
    #[doc = "The version of the catalog item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "A short summary of the catalog item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "A long description of the catalog item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Path to the catalog item entrypoint file."]
    #[serde(rename = "templatePath", default, skip_serializing_if = "Option::is_none")]
    pub template_path: Option<String>,
    #[doc = "JSON schema defining the parameters object passed to actions"]
    #[serde(rename = "parametersSchema", default, skip_serializing_if = "Option::is_none")]
    pub parameters_schema: Option<String>,
    #[doc = "Input parameters passed to actions"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parameters: Vec<CatalogItemParameter>,
    #[doc = "Custom actions for the catalog item."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<CatalogItemAction>,
    #[doc = "The default container image to use to execute actions"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runner: Option<String>,
    #[doc = "Enable or disable status. Indicates whether the property applied to is either enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EnableStatus>,
    #[doc = "Whether the version is eligible to be the latest version."]
    #[serde(rename = "eligibleForLatestVersion", default, skip_serializing_if = "Option::is_none")]
    pub eligible_for_latest_version: Option<bool>,
}
impl CatalogItemVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results of the catalog item list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CatalogItemVersionListResult {
    #[doc = "Current page of results."]
    pub value: Vec<CatalogItemVersion>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CatalogItemVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CatalogItemVersionListResult {
    pub fn new(value: Vec<CatalogItemVersion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[doc = "An error response from the service."]
    pub error: CloudErrorBody,
}
impl azure_core::Continuable for CloudError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CloudError {
    pub fn new(error: CloudErrorBody) -> Self {
        Self { error }
    }
}
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    pub code: String,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    pub message: String,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
        }
    }
}
#[doc = "A DevBox Dev Box"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBox {
    #[doc = "Display name for the Dev Box"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Name of the project this Dev Box belongs to"]
    #[serde(rename = "projectName", default, skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[doc = "The name of the Dev Box pool this machine belongs to."]
    #[serde(rename = "poolName")]
    pub pool_name: String,
    #[doc = "Indicates whether hibernate is supported and enabled or disabled. Unknown hibernate support is represented as null."]
    #[serde(rename = "hibernateSupport", default, skip_serializing_if = "Option::is_none")]
    pub hibernate_support: Option<HibernateSupport>,
    #[doc = "The current provisioning state of the Dev Box."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The current action state of the Dev Box. This is state is based on previous action performed by user."]
    #[serde(rename = "actionState", default, skip_serializing_if = "Option::is_none")]
    pub action_state: Option<String>,
    #[doc = "The power states of a Dev Box."]
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<PowerState>,
    #[doc = "A unique identifier for the Dev Box. This is a GUID-formatted string (e.g. 00000000-0000-0000-0000-000000000000)."]
    #[serde(rename = "uniqueId", default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[doc = "Error details"]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ProvisioningError>,
    #[doc = "Azure region where this Dev Box is located. This will be the same region as the Virtual Network it is attached to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The operating system type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "The AAD object id of the user this Dev Box is assigned to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[doc = "Hardware specifications for the Dev Box."]
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[doc = "Storage settings for the Dev Box's disks"]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "Specifies information about the image used"]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
    #[doc = "Creation time of this Dev Box"]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[serde(rename = "localAdministrator", default, skip_serializing_if = "Option::is_none")]
    pub local_administrator: Option<LocalAdminStatus>,
}
impl DevBox {
    pub fn new(pool_name: String) -> Self {
        Self {
            name: None,
            project_name: None,
            pool_name,
            hibernate_support: None,
            provisioning_state: None,
            action_state: None,
            power_state: None,
            unique_id: None,
            error_details: None,
            location: None,
            os_type: None,
            user: None,
            hardware_profile: None,
            storage_profile: None,
            image_reference: None,
            created_time: None,
            local_administrator: None,
        }
    }
}
#[doc = "The Dev Box list result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBoxListResult {
    #[doc = "The list of DevBox Dev Boxes"]
    pub value: Vec<DevBox>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DevBoxListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DevBoxListResult {
    pub fn new(value: Vec<DevBox>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Enable or disable status. Indicates whether the property applied to is either enabled or disabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnableStatus")]
pub enum EnableStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnableStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnableStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnableStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("EnableStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("EnableStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties of an environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Environment {
    #[serde(flatten)]
    pub environment_update_properties: EnvironmentUpdateProperties,
    #[doc = "Environment name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Environment type."]
    #[serde(rename = "environmentType")]
    pub environment_type: String,
    #[doc = "The AAD object id of the owner of this Environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[doc = "The provisioning state of the environment."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The identifier of the resource group containing the environment's resources."]
    #[serde(rename = "resourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}
impl Environment {
    pub fn new(environment_type: String) -> Self {
        Self {
            environment_update_properties: EnvironmentUpdateProperties::default(),
            name: None,
            environment_type,
            user: None,
            provisioning_state: None,
            resource_group_id: None,
        }
    }
}
#[doc = "Results of the environment list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentListResult {
    #[doc = "Current page of results."]
    pub value: Vec<Environment>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EnvironmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EnvironmentListResult {
    pub fn new(value: Vec<Environment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties of an environment type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentType {
    #[doc = "Name of the environment type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Id of a subscription or management group that the environment type will be mapped to. The environment's resources will be deployed into this subscription or management group."]
    #[serde(rename = "deploymentTargetId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_target_id: Option<String>,
    #[doc = "Enable or disable status. Indicates whether the property applied to is either enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EnableStatus>,
}
impl EnvironmentType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the environment type list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentTypeListResult {
    #[doc = "Current page of results."]
    pub value: Vec<EnvironmentType>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EnvironmentTypeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EnvironmentTypeListResult {
    pub fn new(value: Vec<EnvironmentType>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties of an environment. These properties can be updated after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentUpdateProperties {
    #[doc = "Description of the Environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name of the catalog."]
    #[serde(rename = "catalogName", default, skip_serializing_if = "Option::is_none")]
    pub catalog_name: Option<String>,
    #[doc = "Name of the catalog item."]
    #[serde(rename = "catalogItemName", default, skip_serializing_if = "Option::is_none")]
    pub catalog_item_name: Option<String>,
    #[doc = "Parameters object for the deploy action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Set of supported scheduled tasks to help manage cost."]
    #[serde(rename = "scheduledTasks", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_tasks: Option<serde_json::Value>,
    #[doc = "Key value pairs that will be applied to resources deployed in this environment as tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl EnvironmentUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Hardware specifications for the Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HardwareProfile {
    #[doc = "The name of the SKU"]
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<String>,
    #[doc = "The number of vCPUs available for the Dev Box."]
    #[serde(rename = "vCPUs", default, skip_serializing_if = "Option::is_none")]
    pub v_cp_us: Option<i32>,
    #[doc = "The amount of memory available for the Dev Box."]
    #[serde(rename = "memoryGB", default, skip_serializing_if = "Option::is_none")]
    pub memory_gb: Option<i32>,
}
impl HardwareProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates whether hibernate is supported and enabled or disabled. Unknown hibernate support is represented as null."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HibernateSupport")]
pub enum HibernateSupport {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HibernateSupport {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HibernateSupport {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HibernateSupport {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("HibernateSupport", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("HibernateSupport", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies information about the image used"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageReference {
    #[doc = "The name of the image used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The version of the image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The operating system of the image."]
    #[serde(rename = "operatingSystem", default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[doc = "The operating system build number of the image."]
    #[serde(rename = "osBuildNumber", default, skip_serializing_if = "Option::is_none")]
    pub os_build_number: Option<String>,
    #[doc = "The datetime that the backing image version was published."]
    #[serde(rename = "publishedDate", default, with = "azure_core::date::rfc3339::option")]
    pub published_date: Option<time::OffsetDateTime>,
}
impl ImageReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LocalAdminStatus")]
pub enum LocalAdminStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LocalAdminStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LocalAdminStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LocalAdminStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("LocalAdminStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("LocalAdminStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Settings for the operating system disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsDisk {
    #[doc = "The size of the OS Disk in gigabytes."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
}
impl OsDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operating system type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OsType")]
pub enum OsType {
    Windows,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OsType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OsType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OsType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Windows => serializer.serialize_unit_variant("OsType", 0u32, "Windows"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type of data a parameter accepts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ParameterType")]
pub enum ParameterType {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "string")]
    String,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ParameterType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ParameterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ParameterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Array => serializer.serialize_unit_variant("ParameterType", 0u32, "array"),
            Self::Boolean => serializer.serialize_unit_variant("ParameterType", 1u32, "boolean"),
            Self::Integer => serializer.serialize_unit_variant("ParameterType", 2u32, "integer"),
            Self::Null => serializer.serialize_unit_variant("ParameterType", 3u32, "null"),
            Self::Number => serializer.serialize_unit_variant("ParameterType", 4u32, "number"),
            Self::Object => serializer.serialize_unit_variant("ParameterType", 5u32, "object"),
            Self::String => serializer.serialize_unit_variant("ParameterType", 6u32, "string"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A pool of Dev Boxes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Pool {
    #[doc = "Pool name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure region where Dev Boxes in the pool are located"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The operating system type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "Hardware specifications for the Dev Box."]
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[doc = "Indicates whether hibernate is supported and enabled or disabled. Unknown hibernate support is represented as null."]
    #[serde(rename = "hibernateSupport", default, skip_serializing_if = "Option::is_none")]
    pub hibernate_support: Option<HibernateSupport>,
    #[doc = "Storage settings for the Dev Box's disks"]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "Specifies information about the image used"]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
    #[serde(rename = "localAdministrator", default, skip_serializing_if = "Option::is_none")]
    pub local_administrator: Option<LocalAdminStatus>,
}
impl Pool {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Pool list result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolListResult {
    #[doc = "Current page of results"]
    pub value: Vec<Pool>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PoolListResult {
    pub fn new(value: Vec<Pool>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The power states of a Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PowerState")]
pub enum PowerState {
    Unknown,
    Deallocated,
    PoweredOff,
    Running,
    Hibernated,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PowerState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PowerState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PowerState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("PowerState", 0u32, "Unknown"),
            Self::Deallocated => serializer.serialize_unit_variant("PowerState", 1u32, "Deallocated"),
            Self::PoweredOff => serializer.serialize_unit_variant("PowerState", 2u32, "PoweredOff"),
            Self::Running => serializer.serialize_unit_variant("PowerState", 3u32, "Running"),
            Self::Hibernated => serializer.serialize_unit_variant("PowerState", 4u32, "Hibernated"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Project details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Project {
    #[doc = "Name of the project"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Description of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl Project {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results of the project list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectListResult {
    #[doc = "Current page of results."]
    pub value: Vec<Project>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProjectListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ProjectListResult {
    pub fn new(value: Vec<Project>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Error details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisioningError {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ProvisioningError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provides RDP connection information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemoteConnection {
    #[doc = "URL to open a browser based RDP session"]
    #[serde(rename = "webUrl", default, skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[doc = "Link to open a Remote Desktop session"]
    #[serde(rename = "rdpConnectionUrl", default, skip_serializing_if = "Option::is_none")]
    pub rdp_connection_url: Option<String>,
}
impl RemoteConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Schedule to execute action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Schedule {
    #[doc = "Display name for the Schedule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The supported types for a scheduled task."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ScheduledType>,
    #[doc = "The frequency of task execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<ScheduledFrequency>,
    #[doc = "The target time to trigger the action. The format is HH:MM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[doc = "The IANA timezone id at which the schedule should execute."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}
impl Schedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Schedule list result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleListResult {
    #[doc = "Current page of results"]
    pub value: Vec<Schedule>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScheduleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ScheduleListResult {
    pub fn new(value: Vec<Schedule>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The frequency of task execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduledFrequency")]
pub enum ScheduledFrequency {
    Daily,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduledFrequency {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduledFrequency {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduledFrequency {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Daily => serializer.serialize_unit_variant("ScheduledFrequency", 0u32, "Daily"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Scheduled task to auto-expire an environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledTask {
    #[doc = "The supported types for a scheduled task."]
    #[serde(rename = "type")]
    pub type_: ScheduledTaskType,
    #[doc = "Enable or disable status. Indicates whether the property applied to is either enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<EnableStatus>,
    #[doc = "Date/time by which the environment should expire"]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
}
impl ScheduledTask {
    pub fn new(type_: ScheduledTaskType, start_time: time::OffsetDateTime) -> Self {
        Self {
            type_,
            enabled: None,
            start_time,
        }
    }
}
#[doc = "The supported types for a scheduled task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduledTaskType")]
pub enum ScheduledTaskType {
    AutoExpire,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduledTaskType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduledTaskType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduledTaskType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AutoExpire => serializer.serialize_unit_variant("ScheduledTaskType", 0u32, "AutoExpire"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The supported types for a scheduled task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduledType")]
pub enum ScheduledType {
    StopDevBox,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduledType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduledType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduledType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::StopDevBox => serializer.serialize_unit_variant("ScheduledType", 0u32, "StopDevBox"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Storage settings for the Dev Box's disks"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[doc = "Settings for the operating system disk."]
    #[serde(rename = "osDisk", default, skip_serializing_if = "Option::is_none")]
    pub os_disk: Option<OsDisk>,
}
impl StorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An upcoming Action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpcomingAction {
    #[doc = "Uniquely identifies the action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The upcoming action types."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<UpcomingActionType>,
    #[doc = "The reason for the upcoming action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<UpcomingActionReason>,
    #[doc = "The target time the action will be triggered (UTC)."]
    #[serde(rename = "scheduledTime", default, with = "azure_core::date::rfc3339::option")]
    pub scheduled_time: Option<time::OffsetDateTime>,
    #[doc = "The original scheduled time for the action (UTC)."]
    #[serde(rename = "originalScheduledTime", default, with = "azure_core::date::rfc3339::option")]
    pub original_scheduled_time: Option<time::OffsetDateTime>,
    #[doc = "The id of the resource which triggered this action"]
    #[serde(rename = "sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
}
impl UpcomingAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The reason for the upcoming action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UpcomingActionReason")]
pub enum UpcomingActionReason {
    Schedule,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UpcomingActionReason {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UpcomingActionReason {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UpcomingActionReason {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Schedule => serializer.serialize_unit_variant("UpcomingActionReason", 0u32, "Schedule"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The upcoming action types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UpcomingActionType")]
pub enum UpcomingActionType {
    Stop,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UpcomingActionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UpcomingActionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UpcomingActionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Stop => serializer.serialize_unit_variant("UpcomingActionType", 0u32, "Stop"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The Upcoming Action list result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpcomingActionsListResult {
    #[doc = "Current page of results"]
    pub value: Vec<UpcomingAction>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UpcomingActionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl UpcomingActionsListResult {
    pub fn new(value: Vec<UpcomingAction>) -> Self {
        Self { value, next_link: None }
    }
}

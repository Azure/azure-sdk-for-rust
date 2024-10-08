#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The base export parameter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseExportModel {
    #[doc = "The target Azure Terraform Provider"]
    #[serde(rename = "targetProvider", default, skip_serializing_if = "Option::is_none")]
    pub target_provider: Option<base_export_model::TargetProvider>,
    #[doc = "Whether to output all non-computed properties in the generated Terraform configuration? This probably needs manual modifications to make it valid"]
    #[serde(rename = "fullProperties", default, skip_serializing_if = "Option::is_none")]
    pub full_properties: Option<bool>,
    #[doc = "Mask sensitive attributes in the Terraform configuration"]
    #[serde(rename = "maskSensitive", default, skip_serializing_if = "Option::is_none")]
    pub mask_sensitive: Option<bool>,
}
impl BaseExportModel {
    pub fn new() -> Self {
        Self {
            target_provider: None,
            full_properties: None,
            mask_sensitive: None,
        }
    }
}
pub mod base_export_model {
    use super::*;
    #[doc = "The target Azure Terraform Provider"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TargetProvider")]
    pub enum TargetProvider {
        #[serde(rename = "azurerm")]
        Azurerm,
        #[serde(rename = "azapi")]
        Azapi,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TargetProvider {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TargetProvider {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TargetProvider {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Azurerm => serializer.serialize_unit_variant("TargetProvider", 0u32, "azurerm"),
                Self::Azapi => serializer.serialize_unit_variant("TargetProvider", 1u32, "azapi"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for TargetProvider {
        fn default() -> Self {
            Self::Azurerm
        }
    }
}
#[doc = "The parameter type"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BaseExportModelUnion {
    ExportQuery(ExportQuery),
    ExportResource(ExportResource),
    ExportResourceGroup(ExportResourceGroup),
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
#[doc = "Export parameter for resources queried by ARG (Azure Resource Graph)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportQuery {
    #[serde(flatten)]
    pub base_export_model: BaseExportModel,
    #[doc = "The ARG where predicate. Note that you can combine multiple conditions in one `where` predicate, e.g. `resourceGroup =~ \"my-rg\" and type =~ \"microsoft.network/virtualnetworks\"`"]
    pub query: String,
    #[doc = "The name pattern of the Terraform resources"]
    #[serde(rename = "namePattern", default, skip_serializing_if = "Option::is_none")]
    pub name_pattern: Option<String>,
    #[doc = "Whether to recursively list child resources of the query result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
}
impl ExportQuery {
    pub fn new(base_export_model: BaseExportModel, query: String) -> Self {
        Self {
            base_export_model,
            query,
            name_pattern: None,
            recursive: None,
        }
    }
}
#[doc = "Export parameter for individual resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportResource {
    #[serde(flatten)]
    pub base_export_model: BaseExportModel,
    #[doc = "The id of the resource to be exported"]
    #[serde(rename = "resourceIds")]
    pub resource_ids: Vec<String>,
    #[doc = "The Terraform resource name. Only works when `resourceIds` contains only one item."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "The Terraform resource type. Only works when `resourceIds` contains only one item."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The name pattern of the Terraform resources"]
    #[serde(rename = "namePattern", default, skip_serializing_if = "Option::is_none")]
    pub name_pattern: Option<String>,
}
impl ExportResource {
    pub fn new(base_export_model: BaseExportModel, resource_ids: Vec<String>) -> Self {
        Self {
            base_export_model,
            resource_ids,
            resource_name: None,
            resource_type: None,
            name_pattern: None,
        }
    }
}
#[doc = "Export parameter for a resource group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportResourceGroup {
    #[serde(flatten)]
    pub base_export_model: BaseExportModel,
    #[doc = "The name of the resource group to be exported"]
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
    #[doc = "The name pattern of the Terraform resources"]
    #[serde(rename = "namePattern", default, skip_serializing_if = "Option::is_none")]
    pub name_pattern: Option<String>,
}
impl ExportResourceGroup {
    pub fn new(base_export_model: BaseExportModel, resource_group_name: String) -> Self {
        Self {
            base_export_model,
            resource_group_name,
            name_pattern: None,
        }
    }
}
#[doc = "The Terraform export result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportResult {
    #[doc = "The Terraform configuration content"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[doc = "A list of Azure resources which are not exported to Terraform due to there is no corresponding resources in Terraform"]
    #[serde(
        rename = "skippedResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub skipped_resources: Vec<String>,
    #[doc = "A list of errors derived during exporting each resource"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<ErrorDetail>,
}
impl ExportResult {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The status of the LRO operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "The operation status resource id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The fully qualified resource id of the resource for which the operation was performed."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The progress percentage of the operation, ranges from 0 to 100"]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "The Terraform export result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExportResult>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameter type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Type")]
pub enum Type {
    ExportResource,
    ExportResourceGroup,
    ExportQuery,
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
            Self::ExportResource => serializer.serialize_unit_variant("Type", 0u32, "ExportResource"),
            Self::ExportResourceGroup => serializer.serialize_unit_variant("Type", 1u32, "ExportResourceGroup"),
            Self::ExportQuery => serializer.serialize_unit_variant("Type", 2u32, "ExportQuery"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}

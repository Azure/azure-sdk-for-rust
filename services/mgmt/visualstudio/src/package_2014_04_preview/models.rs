#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The response to an account resource GET request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AccountResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to an account resource list GET request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountResourceListResult {
    #[doc = "Array of resource details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AccountResource>,
}
impl AccountResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The body of a PUT request to modify a Visual Studio account resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountResourceRequest {
    #[doc = "The account name."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The Azure instance location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The type of the operation."]
    #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<account_resource_request::OperationType>,
    #[doc = "The custom properties of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The custom tags of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AccountResourceRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod account_resource_request {
    use super::*;
    #[doc = "The type of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OperationType {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "create")]
        Create,
        #[serde(rename = "update")]
        Update,
        #[serde(rename = "link")]
        Link,
    }
}
#[doc = "The body of a Patch request to add tags to a Visual Studio account resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountTagRequest {
    #[doc = "The custom tags of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AccountTagRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The body of a POST request to check name availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityParameter {
    #[doc = "The name of the resource to check availability for."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "The type of resource to check availability for."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}
impl CheckNameAvailabilityParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a name availability request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "The message describing the detailed reason."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The value which indicates whether the provided name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to an extension resource GET request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Plan data for an extension resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<ExtensionResourcePlan>,
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl ExtensionResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to an extension resource list GET request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionResourceListResult {
    #[doc = "Array of extension resource details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ExtensionResource>,
}
impl ExtensionResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Plan data for an extension resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionResourcePlan {
    #[doc = "Name of the plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Product name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "Optional: the promotion code associated with the plan."]
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[doc = "Name of the extension publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "A string that uniquely identifies the plan version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ExtensionResourcePlan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The body of an extension resource PUT request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionResourceRequest {
    #[doc = "The Azure region of the Visual Studio account associated with this request (i.e 'southcentralus'.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Plan data for an extension resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<ExtensionResourcePlan>,
    #[doc = "A dictionary of extended properties. This property is currently unused."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "A dictionary of user-defined tags to be stored with the extension resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ExtensionResourceRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an operation supported by the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Properties of an operation supported by the resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationProperties>,
    #[doc = "The name of the resource operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container for a list of operations supported by a resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "A list of operations supported by a resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an operation supported by the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "The description of the resource operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The provider name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Visual Studio Team Services project resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Key/value pair of resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl ProjectResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a request to list Team Services project resources in a resource group/account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectResourceListResult {
    #[doc = "List of project resource details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProjectResource>,
}
impl ProjectResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A generic Azure Resource Manager resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Unique identifier of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}

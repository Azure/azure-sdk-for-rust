#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Tenant configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Configuration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Tenant configuration properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationProperties>,
}
impl Configuration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of tenant configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationList {
    #[doc = "The array of tenant configurations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Configuration>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConfigurationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ConfigurationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProperties {
    #[doc = "When flag is set to true Markdown tile will require external storage configuration (URI). The inline content configuration will be prohibited."]
    #[serde(rename = "enforcePrivateMarkdownStorage", default, skip_serializing_if = "Option::is_none")]
    pub enforce_private_markdown_storage: Option<bool>,
}
impl ConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The shared dashboard resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dashboard {
    #[doc = "The shared dashboard properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DashboardProperties>,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Dashboard {
    pub fn new(location: String) -> Self {
        Self {
            properties: None,
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[doc = "A dashboard lens."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardLens {
    #[doc = "The lens order."]
    pub order: i32,
    #[doc = "The dashboard parts."]
    pub parts: Vec<DashboardParts>,
    #[doc = "The dashboard len's metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl DashboardLens {
    pub fn new(order: i32, parts: Vec<DashboardParts>) -> Self {
        Self {
            order,
            parts,
            metadata: None,
        }
    }
}
#[doc = "List of dashboards."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DashboardListResult {
    #[doc = "The array of custom resource provider manifests."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Dashboard>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DashboardListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DashboardListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A dashboard part metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardPartMetadata {
    #[doc = "The type of dashboard part."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl DashboardPartMetadata {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "A dashboard part."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardParts {
    #[doc = "The dashboard's part position."]
    pub position: dashboard_parts::Position,
    #[doc = "A dashboard part metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DashboardPartMetadata>,
}
impl DashboardParts {
    pub fn new(position: dashboard_parts::Position) -> Self {
        Self { position, metadata: None }
    }
}
pub mod dashboard_parts {
    use super::*;
    #[doc = "The dashboard's part position."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Position {
        #[doc = "The dashboard's part x coordinate."]
        pub x: i32,
        #[doc = "The dashboard's part y coordinate."]
        pub y: i32,
        #[doc = "The dashboard's part row span."]
        #[serde(rename = "rowSpan")]
        pub row_span: i32,
        #[doc = "The dashboard's part column span."]
        #[serde(rename = "colSpan")]
        pub col_span: i32,
        #[doc = "The dashboard part's metadata."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metadata: Option<serde_json::Value>,
    }
    impl Position {
        pub fn new(x: i32, y: i32, row_span: i32, col_span: i32) -> Self {
            Self {
                x,
                y,
                row_span,
                col_span,
                metadata: None,
            }
        }
    }
}
#[doc = "The shared dashboard properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DashboardProperties {
    #[doc = "The dashboard lenses."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lenses: Vec<DashboardLens>,
    #[doc = "The dashboard metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl DashboardProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[doc = "Description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Internal error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDefinition>,
}
impl ErrorDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
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
#[doc = "Markdown part metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkdownPartMetadata {
    #[serde(flatten)]
    pub dashboard_part_metadata: DashboardPartMetadata,
    #[doc = "Input to dashboard part."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<serde_json::Value>,
    #[doc = "Markdown part settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<markdown_part_metadata::Settings>,
}
impl MarkdownPartMetadata {
    pub fn new(dashboard_part_metadata: DashboardPartMetadata) -> Self {
        Self {
            dashboard_part_metadata,
            inputs: Vec::new(),
            settings: None,
        }
    }
}
pub mod markdown_part_metadata {
    use super::*;
    #[doc = "Markdown part settings."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Settings {
        #[doc = "The content of markdown part."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub content: Option<settings::Content>,
    }
    impl Settings {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod settings {
        use super::*;
        #[doc = "The content of markdown part."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Content {
            #[doc = "The setting of the content of markdown part."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub settings: Option<content::Settings>,
        }
        impl Content {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod content {
            use super::*;
            #[doc = "The setting of the content of markdown part."]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
            pub struct Settings {
                #[doc = "The content of the markdown part."]
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub content: Option<String>,
                #[doc = "The title of the markdown part."]
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub title: Option<String>,
                #[doc = "The subtitle of the markdown part."]
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub subtitle: Option<String>,
                #[doc = "The source of the content of the markdown part."]
                #[serde(rename = "markdownSource", default, skip_serializing_if = "Option::is_none")]
                pub markdown_source: Option<i32>,
                #[doc = "The uri of markdown content."]
                #[serde(rename = "markdownUri", default, skip_serializing_if = "Option::is_none")]
                pub markdown_uri: Option<String>,
            }
            impl Settings {
                pub fn new() -> Self {
                    Self::default()
                }
            }
        }
    }
}
#[doc = "The shared dashboard resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchableDashboard {
    #[doc = "The shared dashboard properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DashboardProperties>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PatchableDashboard {
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
#[doc = "Supported operations of this resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperation {
    #[doc = "Operation name, in format of {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation applies to data-plane."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<resource_provider_operation::Display>,
}
impl ResourceProviderOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_provider_operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Resource provider: Microsoft Custom Providers."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of this operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Results of the request to list operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationList {
    #[doc = "List of operations supported by this resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceProviderOperation>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceProviderOperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceProviderOperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Violation information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Violation {
    #[doc = "Id of the item that violates tenant configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Id of the user who owns violated item."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "Error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl Violation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of list of items that violate tenant's configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ViolationsList {
    #[doc = "The array of violations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Violation>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ViolationsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ViolationsList {
    pub fn new() -> Self {
        Self::default()
    }
}

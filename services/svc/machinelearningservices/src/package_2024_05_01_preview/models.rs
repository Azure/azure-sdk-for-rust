#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "AssetVersion Definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetVersion {
    #[doc = "Fully qualified resource Id: azureml://workspace/{workspaceName}/indexes/{name}/versions/{version} of the index."]
    pub id: String,
    #[doc = "Update stage to 'Archive' to archive the asset. Default is Development, which means the asset is under development."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[doc = "Description information of the asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Asset's tags. Unlike properties, tags are fully mutable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Asset's properties. Unlike tags, properties are add-only. Once added, a property cannot be removed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AssetVersion {
    pub fn new(id: String) -> Self {
        Self {
            id,
            stage: None,
            description: None,
            system_data: None,
            tags: None,
            properties: None,
        }
    }
}
#[doc = "The error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreFoundationsError {
    #[doc = "One of a server-defined set of error codes."]
    pub code: String,
    #[doc = "A human-readable representation of the error."]
    pub message: String,
    #[doc = "The target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "An array of details about specific errors that led to this reported error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<AzureCoreFoundationsError>,
    #[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<AzureCoreFoundationsInnerError>,
}
impl AzureCoreFoundationsError {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
            innererror: None,
        }
    }
}
#[doc = "A response containing error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreFoundationsErrorResponse {
    #[doc = "The error object."]
    pub error: AzureCoreFoundationsError,
}
impl azure_core::Continuable for AzureCoreFoundationsErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AzureCoreFoundationsErrorResponse {
    pub fn new(error: AzureCoreFoundationsError) -> Self {
        Self { error }
    }
}
#[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCoreFoundationsInnerError {
    #[doc = "One of a server-defined set of error codes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<Box<AzureCoreFoundationsInnerError>>,
}
impl AzureCoreFoundationsInnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Index resource Definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Index {
    #[doc = "Fully qualified resource Id: azureml://workspace/{workspaceName}/indexes/{name}/versions/{version} of the index."]
    pub id: String,
    #[doc = "Update stage to 'Archive' to archive the asset. Default is Development, which means the asset is under development."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[doc = "Description information of the asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Asset's tags. Unlike properties, tags are fully mutable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Asset's properties. Unlike tags, properties are add-only. Once added, a property cannot be removed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Default workspace blob storage Uri. Should work across storage types and auth scenarios."]
    #[serde(rename = "storageUri")]
    pub storage_uri: String,
}
impl Index {
    pub fn new(id: String, storage_uri: String) -> Self {
        Self {
            id,
            stage: None,
            description: None,
            system_data: None,
            tags: None,
            properties: None,
            storage_uri,
        }
    }
}
#[doc = "Paged collection of IndexVersion items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedIndex {
    #[doc = "The list of Indexes."]
    pub value: Vec<Index>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedIndex {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedIndex {
    pub fn new(value: Vec<Index>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of PromptVersion items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedPrompt {
    #[doc = "The list of Prompts."]
    pub value: Vec<Prompt>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedPrompt {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedPrompt {
    pub fn new(value: Vec<Prompt>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Prompt resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
    #[doc = "Fully qualified resource Id: azureml://workspace/{workspaceName}/indexes/{name}/versions/{version} of the index."]
    pub id: String,
    #[doc = "Update stage to 'Archive' to archive the asset. Default is Development, which means the asset is under development."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[doc = "Description information of the asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Asset's tags. Unlike properties, tags are fully mutable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Asset's properties. Unlike tags, properties are add-only. Once added, a property cannot be removed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Default workspace blob storage Ui. Should work across storage types and auth scenarios."]
    #[serde(rename = "dataUri")]
    pub data_uri: String,
    #[doc = "Relative path of the prompt data file at the dataUri location"]
    #[serde(rename = "templatePath")]
    pub template_path: String,
}
impl Prompt {
    pub fn new(id: String, data_uri: String, template_path: String) -> Self {
        Self {
            id,
            stage: None,
            description: None,
            system_data: None,
            tags: None,
            properties: None,
            data_uri,
            template_path,
        }
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The timestamp the resource was created at."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<::time::OffsetDateTime>,
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The identity type that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<String>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<::time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Next version definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionInfo {
    #[doc = "Next version as defined by the server. The server keeps track of all versions that are string-representations of integers. If one exists, the nextVersion will be a string representation of the highest integer value + 1. Otherwise, the nextVersion will default to '1'."]
    #[serde(rename = "nextVersion", default, skip_serializing_if = "Option::is_none")]
    pub next_version: Option<i64>,
    #[doc = "Current latest version of the resource."]
    #[serde(rename = "latestVersion")]
    pub latest_version: String,
}
impl VersionInfo {
    pub fn new(latest_version: String) -> Self {
        Self {
            next_version: None,
            latest_version,
        }
    }
}

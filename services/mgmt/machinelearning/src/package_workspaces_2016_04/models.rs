#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The error response send when an operation fails."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[doc = "error code"]
    pub code: String,
    #[doc = "error message"]
    pub message: String,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
#[doc = "Azure Machine Learning Studio REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Display name of operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The resource provider name: Microsoft.MachineLearning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The resource on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The operation that users can perform."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The description for the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "An array of operations supported by the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of AML Studio operations supported by the AML Studio resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "The resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The location of the resource. This cannot be changed after the resource is created."]
    pub location: String,
    #[doc = "The tags of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[doc = "An object that represents a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a machine learning workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
}
impl Workspace {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "Workspace authorization keys for a workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceKeysResponse {
    #[doc = "Primary authorization key for this workspace."]
    #[serde(rename = "primaryToken", default, skip_serializing_if = "Option::is_none")]
    pub primary_token: Option<String>,
    #[doc = "Secondary authorization key for this workspace."]
    #[serde(rename = "secondaryToken", default, skip_serializing_if = "Option::is_none")]
    pub secondary_token: Option<String>,
}
impl WorkspaceKeysResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a request to list machine learning workspace keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceListResult {
    #[doc = "The list of machine learning workspaces. Since this list may be incomplete, the nextLink field should be used to request the next list of machine learning workspaces."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
    #[doc = "The URI that can be used to request the next list of machine learning workspaces."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkspaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkspaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceProperties {
    #[doc = "The fully qualified arm id of the storage account associated with this workspace."]
    #[serde(rename = "userStorageAccountId")]
    pub user_storage_account_id: String,
    #[doc = "The email id of the owner for this workspace."]
    #[serde(rename = "ownerEmail")]
    pub owner_email: String,
    #[doc = "The type of this workspace."]
    #[serde(rename = "workspaceType", default, skip_serializing_if = "Option::is_none")]
    pub workspace_type: Option<workspace_properties::WorkspaceType>,
    #[doc = "The current state of workspace resource."]
    #[serde(rename = "workspaceState", default, skip_serializing_if = "Option::is_none")]
    pub workspace_state: Option<workspace_properties::WorkspaceState>,
    #[doc = "The immutable id associated with this workspace."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The creation time for this workspace resource."]
    #[serde(rename = "creationTime", default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[doc = "The regional endpoint for the machine learning studio service which hosts this workspace."]
    #[serde(rename = "studioEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub studio_endpoint: Option<String>,
    #[doc = "The key vault identifier used for encrypted workspaces."]
    #[serde(rename = "keyVaultIdentifierId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_identifier_id: Option<String>,
}
impl WorkspaceProperties {
    pub fn new(user_storage_account_id: String, owner_email: String) -> Self {
        Self {
            user_storage_account_id,
            owner_email,
            workspace_type: None,
            workspace_state: None,
            workspace_id: None,
            creation_time: None,
            studio_endpoint: None,
            key_vault_identifier_id: None,
        }
    }
}
pub mod workspace_properties {
    use super::*;
    #[doc = "The type of this workspace."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum WorkspaceType {
        Production,
        Free,
        Anonymous,
        PaidStandard,
        PaidPremium,
    }
    #[doc = "The current state of workspace resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum WorkspaceState {
        Deleted,
        Enabled,
        Disabled,
        Migrated,
        Updated,
        Registered,
        Unregistered,
    }
}
#[doc = "The parameters for updating the properties of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspacePropertiesUpdateParameters {
    #[doc = "The current state of workspace resource."]
    #[serde(rename = "workspaceState", default, skip_serializing_if = "Option::is_none")]
    pub workspace_state: Option<workspace_properties_update_parameters::WorkspaceState>,
    #[doc = "The key vault identifier used for encrypted workspaces."]
    #[serde(rename = "keyVaultIdentifierId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_identifier_id: Option<String>,
}
impl WorkspacePropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workspace_properties_update_parameters {
    use super::*;
    #[doc = "The current state of workspace resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum WorkspaceState {
        Deleted,
        Enabled,
        Disabled,
        Migrated,
        Updated,
        Registered,
        Unregistered,
    }
}
#[doc = "The parameters for updating a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceUpdateParameters {
    #[doc = "The resource tags for the machine learning workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The parameters for updating the properties of a machine learning workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspacePropertiesUpdateParameters>,
}
impl WorkspaceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}

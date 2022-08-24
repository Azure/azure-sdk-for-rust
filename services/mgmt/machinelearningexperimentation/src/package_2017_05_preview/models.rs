#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An object that represents a machine learning team account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a machine learning team account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountProperties>,
}
impl Account {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "The result of a request to list machine learning team accounts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountListResult {
    #[doc = "The list of machine learning team accounts. Since this list may be incomplete, the nextLink field should be used to request the next list of machine learning team accounts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Account>,
    #[doc = "The URI that can be used to request the next list of machine learning team accounts."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a machine learning team account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountProperties {
    #[doc = "The fully qualified arm id of the vso account to be used for this team account."]
    #[serde(rename = "vsoAccountId")]
    pub vso_account_id: String,
    #[doc = "The immutable id associated with this team account."]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "The description of this workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The friendly name for this workspace. This will be the workspace name in the arm id when the workspace object gets created"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The fully qualified arm id of the user key vault."]
    #[serde(rename = "keyVaultId")]
    pub key_vault_id: String,
    #[doc = "The no of users/seats who can access this team account. This property defines the charge on the team account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seats: Option<String>,
    #[doc = "The uri for this machine learning team account."]
    #[serde(rename = "discoveryUri", default, skip_serializing_if = "Option::is_none")]
    pub discovery_uri: Option<String>,
    #[doc = "The creation date of the machine learning team account in ISO8601 format."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The properties of a storage account for a machine learning team account."]
    #[serde(rename = "storageAccount")]
    pub storage_account: StorageAccountProperties,
    #[doc = "The current deployment state of team account resource. The provisioningState is to indicate states for resource provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<account_properties::ProvisioningState>,
}
impl AccountProperties {
    pub fn new(vso_account_id: String, key_vault_id: String, storage_account: StorageAccountProperties) -> Self {
        Self {
            vso_account_id,
            account_id: None,
            description: None,
            friendly_name: None,
            key_vault_id,
            seats: None,
            discovery_uri: None,
            creation_date: None,
            storage_account,
            provisioning_state: None,
        }
    }
}
pub mod account_properties {
    use super::*;
    #[doc = "The current deployment state of team account resource. The provisioningState is to indicate states for resource provisioning."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Succeeded,
        Updating,
        Deleting,
        Failed,
    }
}
#[doc = "The parameters for updating the properties of a machine learning team account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountPropertiesUpdateParameters {
    #[doc = "The description of this workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The friendly name for this workspace. This will be the workspace name in the arm id when the workspace object gets created"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The no of users/seats who can access this team account. This property defines the charge on the team account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seats: Option<String>,
    #[doc = "The key for storage account associated with this team account"]
    #[serde(rename = "storageAccountKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_key: Option<String>,
}
impl AccountPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for updating a machine learning team account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountUpdateParameters {
    #[doc = "The resource tags for the machine learning team account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The parameters for updating the properties of a machine learning team account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountPropertiesUpdateParameters>,
}
impl AccountUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
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
#[doc = "Azure Machine Learning team account REST API operation"]
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
        #[doc = "The resource provider name: Microsoft.MachineLearningExperimentation"]
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
    #[doc = "List of AML team account operations supported by the AML team account resource provider."]
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
#[doc = "An object that represents a machine learning project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a machine learning project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProjectProperties>,
}
impl Project {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "The result of a request to list projects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectListResult {
    #[doc = "The list of projects. Since this list may be incomplete, the nextLink field should be used to request the next list of projects."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Project>,
    #[doc = "The URI that can be used to request the next list of projects."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProjectListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProjectListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a machine learning project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectProperties {
    #[doc = "The description of this project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The immutable id of the team account which contains this project."]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "The immutable id of the workspace which contains this project."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The immutable id of this project."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "The reference to git repo for this project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gitrepo: Option<String>,
    #[doc = "The friendly name for this project."]
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
    #[doc = "The creation date of the project in ISO8601 format."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The current deployment state of project resource. The provisioningState is to indicate states for resource provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<project_properties::ProvisioningState>,
}
impl ProjectProperties {
    pub fn new(friendly_name: String) -> Self {
        Self {
            description: None,
            account_id: None,
            workspace_id: None,
            project_id: None,
            gitrepo: None,
            friendly_name,
            creation_date: None,
            provisioning_state: None,
        }
    }
}
pub mod project_properties {
    use super::*;
    #[doc = "The current deployment state of project resource. The provisioningState is to indicate states for resource provisioning."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Succeeded,
        Updating,
        Deleting,
        Failed,
    }
}
#[doc = "The parameters for updating the properties of a project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectPropertiesUpdateParameters {
    #[doc = "The friendly name for this project."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The description of this project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The reference to git repo for this project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gitrepo: Option<String>,
}
impl ProjectPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for updating a machine learning project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectUpdateParameters {
    #[doc = "The resource tags for the machine learning project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The parameters for updating the properties of a project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProjectPropertiesUpdateParameters>,
}
impl ProjectUpdateParameters {
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
#[doc = "The properties of a storage account for a machine learning team account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountProperties {
    #[doc = "The fully qualified arm Id of the storage account."]
    #[serde(rename = "storageAccountId")]
    pub storage_account_id: String,
    #[doc = "The access key to the storage account."]
    #[serde(rename = "accessKey")]
    pub access_key: String,
}
impl StorageAccountProperties {
    pub fn new(storage_account_id: String, access_key: String) -> Self {
        Self {
            storage_account_id,
            access_key,
        }
    }
}
#[doc = "An object that represents a machine learning team account workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a machine learning team account workspace."]
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
#[doc = "The result of a request to list machine learning team account workspaces."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceListResult {
    #[doc = "The list of machine learning team account workspaces. Since this list may be incomplete, the nextLink field should be used to request the next list of machine learning team accounts."]
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
#[doc = "The properties of a machine learning team account workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceProperties {
    #[doc = "The description of this workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The immutable id of the team account which contains this workspace."]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "The immutable id of this workspace."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The friendly name for this workspace. This will be the workspace name in the arm id when the workspace object gets created"]
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
    #[doc = "The creation date of the machine learning workspace in ISO8601 format."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The current deployment state of team account workspace resource. The provisioningState is to indicate states for resource provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workspace_properties::ProvisioningState>,
}
impl WorkspaceProperties {
    pub fn new(friendly_name: String) -> Self {
        Self {
            description: None,
            account_id: None,
            workspace_id: None,
            friendly_name,
            creation_date: None,
            provisioning_state: None,
        }
    }
}
pub mod workspace_properties {
    use super::*;
    #[doc = "The current deployment state of team account workspace resource. The provisioningState is to indicate states for resource provisioning."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Succeeded,
        Updating,
        Deleting,
        Failed,
    }
}
#[doc = "The parameters for updating the properties of a machine learning team account workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspacePropertiesUpdateParameters {
    #[doc = "Friendly name of this workspace."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Description for this workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl WorkspacePropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for updating a machine learning team account workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceUpdateParameters {
    #[doc = "The resource tags for the machine learning team account workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The parameters for updating the properties of a machine learning team account workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspacePropertiesUpdateParameters>,
}
impl WorkspaceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}

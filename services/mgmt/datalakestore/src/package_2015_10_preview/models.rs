#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The response body contains the status of the specified asynchronous operation, indicating whether it has succeeded, is in progress, or has failed. Note that this status is distinct from the HTTP status code returned for the Get Operation Status operation itself. If the asynchronous operation succeeded, the response body includes the HTTP status code for the successful request. If the asynchronous operation failed, the response body includes the HTTP status code for the failed request and error information regarding the failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureAsyncOperationResult {
    #[doc = "the status of the AzureAsyncOperation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<azure_async_operation_result::Status>,
    #[doc = "Data Lake Store error information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
impl AzureAsyncOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod azure_async_operation_result {
    use super::*;
    #[doc = "the status of the AzureAsyncOperation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        InProgress,
        Succeeded,
        Failed,
    }
}
#[doc = "Data Lake Store account information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeStoreAccount {
    #[doc = "the account regional location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "the account name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "the namespace and type of the account."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "the account subscription ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<EncryptionIdentity>,
    #[doc = "the value of custom properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Data Lake Store account properties information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeStoreAccountProperties>,
}
impl DataLakeStoreAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Store account list information response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeStoreAccountListResult {
    #[doc = "the results of the list operation"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataLakeStoreAccount>,
    #[doc = "the link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "the total count of results that are available, but might not be returned in the current page."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for DataLakeStoreAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataLakeStoreAccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Store account properties information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeStoreAccountProperties {
    #[doc = "the status of the Data Lake Store account while being provisioned."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<data_lake_store_account_properties::ProvisioningState>,
    #[doc = "the status of the Data Lake Store account after provisioning has completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<data_lake_store_account_properties::State>,
    #[doc = "the account creation time."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "The current state of encryption for this Data Lake store account."]
    #[serde(rename = "encryptionState", default, skip_serializing_if = "Option::is_none")]
    pub encryption_state: Option<data_lake_store_account_properties::EncryptionState>,
    #[doc = "The current state of encryption provisioning for this Data Lake store account."]
    #[serde(rename = "encryptionProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub encryption_provisioning_state: Option<data_lake_store_account_properties::EncryptionProvisioningState>,
    #[serde(rename = "encryptionConfig", default, skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    #[doc = "the account last modified time."]
    #[serde(rename = "lastModifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "the gateway host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "the default owner group for all new folders and files created in the Data Lake Store account."]
    #[serde(rename = "defaultGroup", default, skip_serializing_if = "Option::is_none")]
    pub default_group: Option<String>,
}
impl DataLakeStoreAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_lake_store_account_properties {
    use super::*;
    #[doc = "the status of the Data Lake Store account while being provisioned."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Failed,
        Creating,
        Running,
        Succeeded,
        Patching,
        Suspending,
        Resuming,
        Deleting,
        Deleted,
    }
    #[doc = "the status of the Data Lake Store account after provisioning has completed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "suspended")]
        Suspended,
    }
    #[doc = "The current state of encryption for this Data Lake store account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionState {
        Enabled,
        Disabled,
    }
    #[doc = "The current state of encryption provisioning for this Data Lake store account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionProvisioningState {
        Creating,
        Succeeded,
    }
}
#[doc = "Data Lake Store firewall rule list information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeStoreFirewallRuleListResult {
    #[doc = "the results of the list operation"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FirewallRule>,
    #[doc = "the link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "the total count of results that are available, but might not be returned in the current page."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for DataLakeStoreFirewallRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataLakeStoreFirewallRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionConfig {
    #[doc = "The type of encryption configuration being used. Currently the only supported types are 'UserManaged' and 'ServiceManaged'."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<encryption_config::Type>,
    #[serde(rename = "keyVaultMetaInfo", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_meta_info: Option<KeyVaultMetaInfo>,
}
impl EncryptionConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption_config {
    use super::*;
    #[doc = "The type of encryption configuration being used. Currently the only supported types are 'UserManaged' and 'ServiceManaged'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        UserManaged,
        ServiceManaged,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionIdentity {
    #[doc = "The type of encryption being used. Currently the only supported type is 'SystemAssigned'."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<encryption_identity::Type>,
    #[doc = "The principal identifier associated with the encryption."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant identifier associated with the encryption."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl EncryptionIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption_identity {
    use super::*;
    #[doc = "The type of encryption being used. Currently the only supported type is 'SystemAssigned'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[doc = "Data Lake Store error information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "the HTTP status code or error code associated with this error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "the error message to display."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "the target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "the list of error details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetails>,
    #[doc = "Data Lake Store inner error information"]
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<InnerError>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Store error details information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "the HTTP status code or error code associated with this error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "the error message localized based on Accept-Language"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "the target of the particular error (for example, the name of the property in error)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Store firewall rule information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRule {
    #[doc = "the firewall rule's name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "the namespace and type of the firewall Rule."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "the firewall rule's subscription ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "the firewall rule's regional location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Data Lake Store firewall rule properties information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FirewallRuleProperties>,
}
impl FirewallRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Store firewall rule properties information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRuleProperties {
    #[doc = "the start IP address for the firewall rule."]
    #[serde(rename = "startIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub start_ip_address: Option<String>,
    #[doc = "the end IP address for the firewall rule."]
    #[serde(rename = "endIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub end_ip_address: Option<String>,
}
impl FirewallRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Store inner error information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[doc = "the stack trace for the error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace: Option<String>,
    #[doc = "the context for the error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}
impl InnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultMetaInfo {
    #[doc = "The resource identifier for the user managed Key Vault being used to encrypt."]
    #[serde(rename = "keyVaultResourceId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_resource_id: Option<String>,
    #[doc = "The name of the user managed encryption key."]
    #[serde(rename = "encryptionKeyName", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key_name: Option<String>,
    #[doc = "The version of the user managed encryption key."]
    #[serde(rename = "encryptionKeyVersion", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key_version: Option<String>,
}
impl KeyVaultMetaInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

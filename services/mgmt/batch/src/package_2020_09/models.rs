#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Parameters for an activating an application package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivateApplicationPackageParameters {
    #[doc = "The format of the application package binary file."]
    pub format: String,
}
impl ActivateApplicationPackageParameters {
    pub fn new(format: String) -> Self {
        Self { format }
    }
}
#[doc = "Contains information about an application in a Batch account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Application {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties associated with the Application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationProperties>,
}
impl Application {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An application package which represents a particular version of an application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationPackage {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an application package"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationPackageProperties>,
}
impl ApplicationPackage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an application package"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationPackageProperties {
    #[doc = "The current state of the application package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<application_package_properties::State>,
    #[doc = "The format of the application package, if the package is active."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[doc = "The URL for the application package in Azure Storage."]
    #[serde(rename = "storageUrl", default, skip_serializing_if = "Option::is_none")]
    pub storage_url: Option<String>,
    #[doc = "The UTC time at which the Azure Storage URL will expire."]
    #[serde(rename = "storageUrlExpiry", default, with = "azure_core::date::rfc3339::option")]
    pub storage_url_expiry: Option<time::OffsetDateTime>,
    #[doc = "The time at which the package was last activated, if the package is active."]
    #[serde(rename = "lastActivationTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_activation_time: Option<time::OffsetDateTime>,
}
impl ApplicationPackageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod application_package_properties {
    use super::*;
    #[doc = "The current state of the application package."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Pending,
        Active,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPackageReference {
    pub id: String,
    #[doc = "If this is omitted, and no default version is specified for this application, the request fails with the error code InvalidApplicationPackageReferences. If you are calling the REST API directly, the HTTP status code is 409."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ApplicationPackageReference {
    pub fn new(id: String) -> Self {
        Self { id, version: None }
    }
}
#[doc = "The properties associated with the Application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationProperties {
    #[doc = "The display name for the application."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "A value indicating whether packages within the application may be overwritten using the same version string."]
    #[serde(rename = "allowUpdates", default, skip_serializing_if = "Option::is_none")]
    pub allow_updates: Option<bool>,
    #[doc = "The package to use if a client requests the application but does not specify a version. This property can only be set to the name of an existing package."]
    #[serde(rename = "defaultVersion", default, skip_serializing_if = "Option::is_none")]
    pub default_version: Option<String>,
}
impl ApplicationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScaleRun {
    #[serde(rename = "evaluationTime", with = "azure_core::date::rfc3339")]
    pub evaluation_time: time::OffsetDateTime,
    #[doc = "Each variable value is returned in the form $variable=value, and variables are separated by semicolons."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<AutoScaleRunError>,
}
impl AutoScaleRun {
    pub fn new(evaluation_time: time::OffsetDateTime) -> Self {
        Self {
            evaluation_time,
            results: None,
            error: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScaleRunError {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    pub code: String,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<AutoScaleRunError>,
}
impl AutoScaleRunError {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScaleSettings {
    pub formula: String,
    #[doc = "If omitted, the default value is 15 minutes (PT15M)."]
    #[serde(rename = "evaluationInterval", default, skip_serializing_if = "Option::is_none")]
    pub evaluation_interval: Option<String>,
}
impl AutoScaleSettings {
    pub fn new(formula: String) -> Self {
        Self {
            formula,
            evaluation_interval: None,
        }
    }
}
#[doc = "The properties related to the auto-storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoStorageBaseProperties {
    #[doc = "The resource ID of the storage account to be used for auto-storage account."]
    #[serde(rename = "storageAccountId")]
    pub storage_account_id: String,
}
impl AutoStorageBaseProperties {
    pub fn new(storage_account_id: String) -> Self {
        Self { storage_account_id }
    }
}
#[doc = "Contains information about the auto-storage account associated with a Batch account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoStorageProperties {
    #[serde(flatten)]
    pub auto_storage_base_properties: AutoStorageBaseProperties,
    #[doc = "The UTC time at which storage keys were last synchronized with the Batch account."]
    #[serde(rename = "lastKeySync", with = "azure_core::date::rfc3339")]
    pub last_key_sync: time::OffsetDateTime,
}
impl AutoStorageProperties {
    pub fn new(auto_storage_base_properties: AutoStorageBaseProperties, last_key_sync: time::OffsetDateTime) -> Self {
        Self {
            auto_storage_base_properties,
            last_key_sync,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoUserSpecification {
    #[doc = "The default value is Pool. If the pool is running Windows a value of Task should be specified if stricter isolation between tasks is required. For example, if the task mutates the registry in a way which could impact other tasks, or if certificates have been specified on the pool which should not be accessible by normal tasks but should be accessible by start tasks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<auto_user_specification::Scope>,
    #[serde(rename = "elevationLevel", default, skip_serializing_if = "Option::is_none")]
    pub elevation_level: Option<ElevationLevel>,
}
impl AutoUserSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod auto_user_specification {
    use super::*;
    #[doc = "The default value is Pool. If the pool is running Windows a value of Task should be specified if stricter isolation between tasks is required. For example, if the task mutates the registry in a way which could impact other tasks, or if certificates have been specified on the pool which should not be accessible by normal tasks but should be accessible by start tasks."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Scope {
        Task,
        Pool,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBlobFileSystemConfiguration {
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "This property is mutually exclusive with sasKey and one must be specified."]
    #[serde(rename = "accountKey", default, skip_serializing_if = "Option::is_none")]
    pub account_key: Option<String>,
    #[doc = "This property is mutually exclusive with accountKey and one must be specified."]
    #[serde(rename = "sasKey", default, skip_serializing_if = "Option::is_none")]
    pub sas_key: Option<String>,
    #[doc = "These are 'net use' options in Windows and 'mount' options in Linux."]
    #[serde(rename = "blobfuseOptions", default, skip_serializing_if = "Option::is_none")]
    pub blobfuse_options: Option<String>,
    #[doc = "All file systems are mounted relative to the Batch mounts directory, accessible via the AZ_BATCH_NODE_MOUNTS_DIR environment variable."]
    #[serde(rename = "relativeMountPath")]
    pub relative_mount_path: String,
}
impl AzureBlobFileSystemConfiguration {
    pub fn new(account_name: String, container_name: String, relative_mount_path: String) -> Self {
        Self {
            account_name,
            container_name,
            account_key: None,
            sas_key: None,
            blobfuse_options: None,
            relative_mount_path,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileShareConfiguration {
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[doc = "This is of the form 'https://{account}.file.core.windows.net/'."]
    #[serde(rename = "azureFileUrl")]
    pub azure_file_url: String,
    #[serde(rename = "accountKey")]
    pub account_key: String,
    #[doc = "All file systems are mounted relative to the Batch mounts directory, accessible via the AZ_BATCH_NODE_MOUNTS_DIR environment variable."]
    #[serde(rename = "relativeMountPath")]
    pub relative_mount_path: String,
    #[doc = "These are 'net use' options in Windows and 'mount' options in Linux."]
    #[serde(rename = "mountOptions", default, skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<String>,
}
impl AzureFileShareConfiguration {
    pub fn new(account_name: String, azure_file_url: String, account_key: String, relative_mount_path: String) -> Self {
        Self {
            account_name,
            azure_file_url,
            account_key,
            relative_mount_path,
            mount_options: None,
        }
    }
}
#[doc = "Contains information about an Azure Batch account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Account specific properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BatchAccountProperties>,
    #[doc = "The identity of the Batch account, if configured. This is only used when the user specifies 'Microsoft.KeyVault' as their Batch account encryption configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<BatchAccountIdentity>,
}
impl BatchAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Create operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchAccountCreateParameters {
    #[doc = "The region in which to create the account."]
    pub location: String,
    #[doc = "The user-specified tags associated with the account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The properties of a Batch account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BatchAccountCreateProperties>,
    #[doc = "The identity of the Batch account, if configured. This is only used when the user specifies 'Microsoft.KeyVault' as their Batch account encryption configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<BatchAccountIdentity>,
}
impl BatchAccountCreateParameters {
    pub fn new(location: String) -> Self {
        Self {
            location,
            tags: None,
            properties: None,
            identity: None,
        }
    }
}
#[doc = "The properties of a Batch account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchAccountCreateProperties {
    #[doc = "The properties related to the auto-storage account."]
    #[serde(rename = "autoStorage", default, skip_serializing_if = "Option::is_none")]
    pub auto_storage: Option<AutoStorageBaseProperties>,
    #[doc = "The allocation mode for creating pools in the Batch account."]
    #[serde(rename = "poolAllocationMode", default, skip_serializing_if = "Option::is_none")]
    pub pool_allocation_mode: Option<PoolAllocationMode>,
    #[doc = "Identifies the Azure key vault associated with a Batch account."]
    #[serde(rename = "keyVaultReference", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_reference: Option<KeyVaultReference>,
    #[doc = "The network access type for operating on the resources in the Batch account."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccessType>,
    #[doc = "Configures how customer data is encrypted inside the Batch account. By default, accounts are encrypted using a Microsoft managed key. For additional control, a customer-managed key can be used instead."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionProperties>,
}
impl BatchAccountCreateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The identity of the Batch account, if configured. This is only used when the user specifies 'Microsoft.KeyVault' as their Batch account encryption configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchAccountIdentity {
    #[doc = "The principal id of the Batch account. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id associated with the Batch account. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of identity used for the Batch account."]
    #[serde(rename = "type")]
    pub type_: batch_account_identity::Type,
}
impl BatchAccountIdentity {
    pub fn new(type_: batch_account_identity::Type) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
        }
    }
}
pub mod batch_account_identity {
    use super::*;
    #[doc = "The type of identity used for the Batch account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        None,
    }
}
#[doc = "A set of Azure Batch account keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchAccountKeys {
    #[doc = "The Batch account name."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The primary key associated with the account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
    #[doc = "The secondary key associated with the account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,
}
impl BatchAccountKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Values returned by the List operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchAccountListResult {
    #[doc = "The collection of Batch accounts returned by the listing operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BatchAccount>,
    #[doc = "The continuation token."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BatchAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BatchAccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Account specific properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchAccountProperties {
    #[doc = "The account endpoint used to interact with the Batch service."]
    #[serde(rename = "accountEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub account_endpoint: Option<String>,
    #[doc = "The provisioned state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<batch_account_properties::ProvisioningState>,
    #[doc = "The allocation mode for creating pools in the Batch account."]
    #[serde(rename = "poolAllocationMode", default, skip_serializing_if = "Option::is_none")]
    pub pool_allocation_mode: Option<PoolAllocationMode>,
    #[doc = "Identifies the Azure key vault associated with a Batch account."]
    #[serde(rename = "keyVaultReference", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_reference: Option<KeyVaultReference>,
    #[doc = "The network access type for operating on the resources in the Batch account."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccessType>,
    #[doc = "List of private endpoint connections associated with the Batch account"]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "Contains information about the auto-storage account associated with a Batch account."]
    #[serde(rename = "autoStorage", default, skip_serializing_if = "Option::is_none")]
    pub auto_storage: Option<AutoStorageProperties>,
    #[doc = "Configures how customer data is encrypted inside the Batch account. By default, accounts are encrypted using a Microsoft managed key. For additional control, a customer-managed key can be used instead."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionProperties>,
    #[doc = "For accounts with PoolAllocationMode set to UserSubscription, quota is managed on the subscription so this value is not returned."]
    #[serde(rename = "dedicatedCoreQuota", default, skip_serializing_if = "Option::is_none")]
    pub dedicated_core_quota: Option<i32>,
    #[doc = "For accounts with PoolAllocationMode set to UserSubscription, quota is managed on the subscription so this value is not returned."]
    #[serde(rename = "lowPriorityCoreQuota", default, skip_serializing_if = "Option::is_none")]
    pub low_priority_core_quota: Option<i32>,
    #[doc = "A list of the dedicated core quota per Virtual Machine family for the Batch account. For accounts with PoolAllocationMode set to UserSubscription, quota is managed on the subscription so this value is not returned."]
    #[serde(rename = "dedicatedCoreQuotaPerVMFamily", default, skip_serializing_if = "Vec::is_empty")]
    pub dedicated_core_quota_per_vm_family: Vec<VirtualMachineFamilyCoreQuota>,
    #[doc = "Batch is transitioning its core quota system for dedicated cores to be enforced per Virtual Machine family. During this transitional phase, the dedicated core quota per Virtual Machine family may not yet be enforced. If this flag is false, dedicated core quota is enforced via the old dedicatedCoreQuota property on the account and does not consider Virtual Machine family. If this flag is true, dedicated core quota is enforced via the dedicatedCoreQuotaPerVMFamily property on the account, and the old dedicatedCoreQuota does not apply."]
    #[serde(rename = "dedicatedCoreQuotaPerVMFamilyEnforced", default, skip_serializing_if = "Option::is_none")]
    pub dedicated_core_quota_per_vm_family_enforced: Option<bool>,
    #[serde(rename = "poolQuota", default, skip_serializing_if = "Option::is_none")]
    pub pool_quota: Option<i32>,
    #[serde(rename = "activeJobAndJobScheduleQuota", default, skip_serializing_if = "Option::is_none")]
    pub active_job_and_job_schedule_quota: Option<i32>,
}
impl BatchAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod batch_account_properties {
    use super::*;
    #[doc = "The provisioned state of the resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Invalid,
        Creating,
        Deleting,
        Succeeded,
        Failed,
        Cancelled,
    }
}
#[doc = "Parameters supplied to the RegenerateKey operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchAccountRegenerateKeyParameters {
    #[doc = "The type of account key to regenerate."]
    #[serde(rename = "keyName")]
    pub key_name: batch_account_regenerate_key_parameters::KeyName,
}
impl BatchAccountRegenerateKeyParameters {
    pub fn new(key_name: batch_account_regenerate_key_parameters::KeyName) -> Self {
        Self { key_name }
    }
}
pub mod batch_account_regenerate_key_parameters {
    use super::*;
    #[doc = "The type of account key to regenerate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyName {
        Primary,
        Secondary,
    }
}
#[doc = "Parameters for updating an Azure Batch account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchAccountUpdateParameters {
    #[doc = "The user-specified tags associated with the account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The properties of a Batch account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BatchAccountUpdateProperties>,
    #[doc = "The identity of the Batch account, if configured. This is only used when the user specifies 'Microsoft.KeyVault' as their Batch account encryption configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<BatchAccountIdentity>,
}
impl BatchAccountUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a Batch account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchAccountUpdateProperties {
    #[doc = "The properties related to the auto-storage account."]
    #[serde(rename = "autoStorage", default, skip_serializing_if = "Option::is_none")]
    pub auto_storage: Option<AutoStorageBaseProperties>,
    #[doc = "Configures how customer data is encrypted inside the Batch account. By default, accounts are encrypted using a Microsoft managed key. For additional control, a customer-managed key can be used instead."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionProperties>,
}
impl BatchAccountUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quotas associated with a Batch region for a particular subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchLocationQuota {
    #[doc = "The number of Batch accounts that may be created under the subscription in the specified region."]
    #[serde(rename = "accountQuota", default, skip_serializing_if = "Option::is_none")]
    pub account_quota: Option<i32>,
}
impl BatchLocationQuota {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CifsMountConfiguration {
    pub username: String,
    pub source: String,
    #[doc = "All file systems are mounted relative to the Batch mounts directory, accessible via the AZ_BATCH_NODE_MOUNTS_DIR environment variable."]
    #[serde(rename = "relativeMountPath")]
    pub relative_mount_path: String,
    #[doc = "These are 'net use' options in Windows and 'mount' options in Linux."]
    #[serde(rename = "mountOptions", default, skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<String>,
    pub password: String,
}
impl CifsMountConfiguration {
    pub fn new(username: String, source: String, relative_mount_path: String, password: String) -> Self {
        Self {
            username,
            source,
            relative_mount_path,
            mount_options: None,
            password,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CachingType {
    None,
    ReadOnly,
    ReadWrite,
}
#[doc = "Contains information about a certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Certificate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Certificate properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificateProperties>,
}
impl Certificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateBaseProperties {
    #[doc = "This must match the first portion of the certificate name. Currently required to be 'SHA1'."]
    #[serde(rename = "thumbprintAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub thumbprint_algorithm: Option<String>,
    #[doc = "This must match the thumbprint from the name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "The format of the certificate - either Pfx or Cer. If omitted, the default is Pfx."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<certificate_base_properties::Format>,
}
impl CertificateBaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod certificate_base_properties {
    use super::*;
    #[doc = "The format of the certificate - either Pfx or Cer. If omitted, the default is Pfx."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Format {
        Pfx,
        Cer,
    }
}
#[doc = "Contains information about a certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateCreateOrUpdateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Certificate properties for create operations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificateCreateOrUpdateProperties>,
}
impl CertificateCreateOrUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Certificate properties for create operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateCreateOrUpdateProperties {
    #[serde(flatten)]
    pub certificate_base_properties: CertificateBaseProperties,
    #[doc = "The maximum size is 10KB."]
    pub data: String,
    #[doc = "This must not be specified if the certificate format is Cer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl CertificateCreateOrUpdateProperties {
    pub fn new(data: String) -> Self {
        Self {
            certificate_base_properties: CertificateBaseProperties::default(),
            data,
            password: None,
        }
    }
}
#[doc = "Certificate properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateProperties {
    #[serde(flatten)]
    pub certificate_base_properties: CertificateBaseProperties,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<certificate_properties::ProvisioningState>,
    #[serde(rename = "provisioningStateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_state_transition_time: Option<time::OffsetDateTime>,
    #[doc = "The previous provisioned state of the resource"]
    #[serde(rename = "previousProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub previous_provisioning_state: Option<certificate_properties::PreviousProvisioningState>,
    #[serde(
        rename = "previousProvisioningStateTransitionTime",
        default,
        with = "azure_core::date::rfc3339::option"
    )]
    pub previous_provisioning_state_transition_time: Option<time::OffsetDateTime>,
    #[doc = "The public key of the certificate."]
    #[serde(rename = "publicData", default, skip_serializing_if = "Option::is_none")]
    pub public_data: Option<String>,
    #[doc = "An error response from the Batch service."]
    #[serde(rename = "deleteCertificateError", default, skip_serializing_if = "Option::is_none")]
    pub delete_certificate_error: Option<DeleteCertificateError>,
}
impl CertificateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod certificate_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Deleting,
        Failed,
    }
    #[doc = "The previous provisioned state of the resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PreviousProvisioningState {
        Succeeded,
        Deleting,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateReference {
    pub id: String,
    #[doc = "The default value is currentUser. This property is applicable only for pools configured with Windows nodes (that is, created with cloudServiceConfiguration, or with virtualMachineConfiguration using a Windows image reference). For Linux compute nodes, the certificates are stored in a directory inside the task working directory and an environment variable AZ_BATCH_CERTIFICATES_DIR is supplied to the task to query for this location. For certificates with visibility of 'remoteUser', a 'certs' directory is created in the user's home directory (e.g., /home/{user-name}/certs) and certificates are placed in that directory."]
    #[serde(rename = "storeLocation", default, skip_serializing_if = "Option::is_none")]
    pub store_location: Option<certificate_reference::StoreLocation>,
    #[doc = "This property is applicable only for pools configured with Windows nodes (that is, created with cloudServiceConfiguration, or with virtualMachineConfiguration using a Windows image reference). Common store names include: My, Root, CA, Trust, Disallowed, TrustedPeople, TrustedPublisher, AuthRoot, AddressBook, but any custom store name can also be used. The default value is My."]
    #[serde(rename = "storeName", default, skip_serializing_if = "Option::is_none")]
    pub store_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub visibility: Vec<String>,
}
impl CertificateReference {
    pub fn new(id: String) -> Self {
        Self {
            id,
            store_location: None,
            store_name: None,
            visibility: Vec::new(),
        }
    }
}
pub mod certificate_reference {
    use super::*;
    #[doc = "The default value is currentUser. This property is applicable only for pools configured with Windows nodes (that is, created with cloudServiceConfiguration, or with virtualMachineConfiguration using a Windows image reference). For Linux compute nodes, the certificates are stored in a directory inside the task working directory and an environment variable AZ_BATCH_CERTIFICATES_DIR is supplied to the task to query for this location. For certificates with visibility of 'remoteUser', a 'certs' directory is created in the user's home directory (e.g., /home/{user-name}/certs) and certificates are placed in that directory."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StoreLocation {
        CurrentUser,
        LocalMachine,
    }
}
#[doc = "Parameters for a check name availability request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameters {
    #[doc = "The name to check for availability"]
    pub name: String,
    #[doc = "The resource type."]
    #[serde(rename = "type")]
    pub type_: check_name_availability_parameters::Type,
}
impl CheckNameAvailabilityParameters {
    pub fn new(name: String, type_: check_name_availability_parameters::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod check_name_availability_parameters {
    use super::*;
    #[doc = "The resource type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Batch/batchAccounts")]
        MicrosoftBatchBatchAccounts,
    }
}
#[doc = "The CheckNameAvailability operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "Gets a boolean value that indicates whether the name is available for you to use. If true, the name is available. If false, the name has already been taken or invalid and cannot be used."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Gets the reason that a Batch account name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[doc = "Gets an error message explaining the Reason value in more detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_result {
    use super::*;
    #[doc = "Gets the reason that a Batch account name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[doc = "An error response from the Batch service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the Batch service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl azure_core::Continuable for CloudError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the Batch service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudServiceConfiguration {
    #[doc = "Possible values are: 2 - OS Family 2, equivalent to Windows Server 2008 R2 SP1. 3 - OS Family 3, equivalent to Windows Server 2012. 4 - OS Family 4, equivalent to Windows Server 2012 R2. 5 - OS Family 5, equivalent to Windows Server 2016. 6 - OS Family 6, equivalent to Windows Server 2019. For more information, see Azure Guest OS Releases (https://azure.microsoft.com/documentation/articles/cloud-services-guestos-update-matrix/#releases)."]
    #[serde(rename = "osFamily")]
    pub os_family: String,
    #[doc = "The default value is * which specifies the latest operating system version for the specified OS family."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
}
impl CloudServiceConfiguration {
    pub fn new(os_family: String) -> Self {
        Self {
            os_family,
            os_version: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ComputeNodeDeallocationOption {
    Requeue,
    Terminate,
    TaskCompletion,
    RetainedData,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerConfiguration {
    #[serde(rename = "type")]
    pub type_: container_configuration::Type,
    #[doc = "This is the full image reference, as would be specified to \"docker pull\". An image will be sourced from the default Docker registry unless the image is fully qualified with an alternative registry."]
    #[serde(rename = "containerImageNames", default, skip_serializing_if = "Vec::is_empty")]
    pub container_image_names: Vec<String>,
    #[doc = "If any images must be downloaded from a private registry which requires credentials, then those credentials must be provided here."]
    #[serde(rename = "containerRegistries", default, skip_serializing_if = "Vec::is_empty")]
    pub container_registries: Vec<ContainerRegistry>,
}
impl ContainerConfiguration {
    pub fn new(type_: container_configuration::Type) -> Self {
        Self {
            type_,
            container_image_names: Vec::new(),
            container_registries: Vec::new(),
        }
    }
}
pub mod container_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        DockerCompatible,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerRegistry {
    #[doc = "If omitted, the default is \"docker.io\"."]
    #[serde(rename = "registryServer", default, skip_serializing_if = "Option::is_none")]
    pub registry_server: Option<String>,
    pub username: String,
    pub password: String,
}
impl ContainerRegistry {
    pub fn new(username: String, password: String) -> Self {
        Self {
            registry_server: None,
            username,
            password,
        }
    }
}
#[doc = "Settings which will be used by the data disks associated to Compute Nodes in the Pool. When using attached data disks, you need to mount and format the disks from within a VM to use them."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataDisk {
    #[doc = "The lun is used to uniquely identify each data disk. If attaching multiple disks, each should have a distinct lun. The value must be between 0 and 63, inclusive."]
    pub lun: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<CachingType>,
    #[serde(rename = "diskSizeGB")]
    pub disk_size_gb: i32,
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<StorageAccountType>,
}
impl DataDisk {
    pub fn new(lun: i32, disk_size_gb: i32) -> Self {
        Self {
            lun,
            caching: None,
            disk_size_gb,
            storage_account_type: None,
        }
    }
}
#[doc = "An error response from the Batch service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteCertificateError {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    pub code: String,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    pub message: String,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<DeleteCertificateError>,
}
impl DeleteCertificateError {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentConfiguration {
    #[serde(rename = "cloudServiceConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cloud_service_configuration: Option<CloudServiceConfiguration>,
    #[serde(rename = "virtualMachineConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_configuration: Option<VirtualMachineConfiguration>,
}
impl DeploymentConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The disk encryption configuration applied on compute nodes in the pool. Disk encryption configuration is not supported on Linux pool created with Virtual Machine Image or Shared Image Gallery Image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskEncryptionConfiguration {
    #[doc = "On Linux pool, only \"TemporaryDisk\" is supported; on Windows pool, \"OsDisk\" and \"TemporaryDisk\" must be specified."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<String>,
}
impl DiskEncryptionConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ElevationLevel {
    NonAdmin,
    Admin,
}
#[doc = "Configures how customer data is encrypted inside the Batch account. By default, accounts are encrypted using a Microsoft managed key. For additional control, a customer-managed key can be used instead."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionProperties {
    #[doc = "Type of the key source."]
    #[serde(rename = "keySource", default, skip_serializing_if = "Option::is_none")]
    pub key_source: Option<encryption_properties::KeySource>,
    #[doc = "KeyVault configuration when using an encryption KeySource of Microsoft.KeyVault."]
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
}
impl EncryptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption_properties {
    use super::*;
    #[doc = "Type of the key source."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeySource {
        #[serde(rename = "Microsoft.Batch")]
        MicrosoftBatch,
        #[serde(rename = "Microsoft.KeyVault")]
        MicrosoftKeyVault,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentSetting {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl EnvironmentSetting {
    pub fn new(name: String) -> Self {
        Self { name, value: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FixedScaleSettings {
    #[doc = "The default value is 15 minutes. Timeout values use ISO 8601 format. For example, use PT10M for 10 minutes. The minimum value is 5 minutes. If you specify a value less than 5 minutes, the Batch service rejects the request with an error; if you are calling the REST API directly, the HTTP status code is 400 (Bad Request)."]
    #[serde(rename = "resizeTimeout", default, skip_serializing_if = "Option::is_none")]
    pub resize_timeout: Option<String>,
    #[doc = "At least one of targetDedicatedNodes, targetLowPriorityNodes must be set."]
    #[serde(rename = "targetDedicatedNodes", default, skip_serializing_if = "Option::is_none")]
    pub target_dedicated_nodes: Option<i32>,
    #[doc = "At least one of targetDedicatedNodes, targetLowPriorityNodes must be set."]
    #[serde(rename = "targetLowPriorityNodes", default, skip_serializing_if = "Option::is_none")]
    pub target_low_priority_nodes: Option<i32>,
    #[serde(rename = "nodeDeallocationOption", default, skip_serializing_if = "Option::is_none")]
    pub node_deallocation_option: Option<ComputeNodeDeallocationOption>,
}
impl FixedScaleSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum IpAddressProvisioningType {
    BatchManaged,
    UserManaged,
    #[serde(rename = "NoPublicIPAddresses")]
    NoPublicIpAddresses,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageReference {
    #[doc = "For example, Canonical or MicrosoftWindowsServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "For example, UbuntuServer or WindowsServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "For example, 18.04-LTS or 2019-Datacenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "A value of 'latest' can be specified to select the latest version of an image. If omitted, the default is 'latest'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "This property is mutually exclusive with other properties. The Shared Image Gallery image must have replicas in the same region as the Azure Batch account. For information about the firewall settings for the Batch node agent to communicate with the Batch service see https://docs.microsoft.com/en-us/azure/batch/batch-api-basics#virtual-network-vnet-and-firewall-configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ImageReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InboundNatPool {
    #[doc = "The name must be unique within a Batch pool, can contain letters, numbers, underscores, periods, and hyphens. Names must start with a letter or number, must end with a letter, number, or underscore, and cannot exceed 77 characters.  If any invalid values are provided the request fails with HTTP status code 400."]
    pub name: String,
    pub protocol: inbound_nat_pool::Protocol,
    #[doc = "This must be unique within a Batch pool. Acceptable values are between 1 and 65535 except for 22, 3389, 29876 and 29877 as these are reserved. If any reserved values are provided the request fails with HTTP status code 400."]
    #[serde(rename = "backendPort")]
    pub backend_port: i32,
    #[doc = "Acceptable values range between 1 and 65534 except ports from 50000 to 55000 which are reserved. All ranges within a pool must be distinct and cannot overlap. If any reserved or overlapping values are provided the request fails with HTTP status code 400."]
    #[serde(rename = "frontendPortRangeStart")]
    pub frontend_port_range_start: i32,
    #[doc = "Acceptable values range between 1 and 65534 except ports from 50000 to 55000 which are reserved by the Batch service. All ranges within a pool must be distinct and cannot overlap. If any reserved or overlapping values are provided the request fails with HTTP status code 400."]
    #[serde(rename = "frontendPortRangeEnd")]
    pub frontend_port_range_end: i32,
    #[doc = "The maximum number of rules that can be specified across all the endpoints on a Batch pool is 25. If no network security group rules are specified, a default rule will be created to allow inbound access to the specified backendPort. If the maximum number of network security group rules is exceeded the request fails with HTTP status code 400."]
    #[serde(rename = "networkSecurityGroupRules", default, skip_serializing_if = "Vec::is_empty")]
    pub network_security_group_rules: Vec<NetworkSecurityGroupRule>,
}
impl InboundNatPool {
    pub fn new(
        name: String,
        protocol: inbound_nat_pool::Protocol,
        backend_port: i32,
        frontend_port_range_start: i32,
        frontend_port_range_end: i32,
    ) -> Self {
        Self {
            name,
            protocol,
            backend_port,
            frontend_port_range_start,
            frontend_port_range_end,
            network_security_group_rules: Vec::new(),
        }
    }
}
pub mod inbound_nat_pool {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Protocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
    }
}
#[doc = "KeyVault configuration when using an encryption KeySource of Microsoft.KeyVault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultProperties {
    #[doc = "Full path to the versioned secret. Example https://mykeyvault.vault.azure.net/keys/testkey/6e34a81fef704045975661e297a4c053. To be usable the following prerequisites must be met:\n\n The Batch Account has a System Assigned identity\n The account identity has been granted Key/Get, Key/Unwrap and Key/Wrap permissions\n The KeyVault has soft-delete and purge protection enabled"]
    #[serde(rename = "keyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub key_identifier: Option<String>,
}
impl KeyVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identifies the Azure key vault associated with a Batch account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultReference {
    #[doc = "The resource ID of the Azure key vault associated with the Batch account."]
    pub id: String,
    #[doc = "The URL of the Azure key vault associated with the Batch account."]
    pub url: String,
}
impl KeyVaultReference {
    pub fn new(id: String, url: String) -> Self {
        Self { id, url }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxUserConfiguration {
    #[doc = "The uid and gid properties must be specified together or not at all. If not specified the underlying operating system picks the uid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<i32>,
    #[doc = "The uid and gid properties must be specified together or not at all. If not specified the underlying operating system picks the gid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gid: Option<i32>,
    #[doc = "The private key must not be password protected. The private key is used to automatically configure asymmetric-key based authentication for SSH between nodes in a Linux pool when the pool's enableInterNodeCommunication property is true (it is ignored if enableInterNodeCommunication is false). It does this by placing the key pair into the user's .ssh directory. If not specified, password-less SSH is not configured between nodes (no modification of the user's .ssh directory is done)."]
    #[serde(rename = "sshPrivateKey", default, skip_serializing_if = "Option::is_none")]
    pub ssh_private_key: Option<String>,
}
impl LinuxUserConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of performing list application packages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListApplicationPackagesResult {
    #[doc = "The list of application packages."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationPackage>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListApplicationPackagesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListApplicationPackagesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of performing list applications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListApplicationsResult {
    #[doc = "The list of applications."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Application>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListApplicationsResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListApplicationsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Values returned by the List operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListCertificatesResult {
    #[doc = "The collection of returned certificates."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Certificate>,
    #[doc = "The continuation token."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListCertificatesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListCertificatesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Values returned by the List operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListPoolsResult {
    #[doc = "The collection of returned pools."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Pool>,
    #[doc = "The continuation token."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListPoolsResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListPoolsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Values returned by the List operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListPrivateEndpointConnectionsResult {
    #[doc = "The collection of returned private endpoint connection."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "The continuation token."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListPrivateEndpointConnectionsResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListPrivateEndpointConnectionsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Values returned by the List operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListPrivateLinkResourcesResult {
    #[doc = "The collection of returned private link resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "The continuation token."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListPrivateLinkResourcesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListPrivateLinkResourcesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Batch service does not assign any meaning to this metadata; it is solely for the use of user code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataItem {
    pub name: String,
    pub value: String,
}
impl MetadataItem {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MountConfiguration {
    #[serde(rename = "azureBlobFileSystemConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub azure_blob_file_system_configuration: Option<AzureBlobFileSystemConfiguration>,
    #[serde(rename = "nfsMountConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub nfs_mount_configuration: Option<NfsMountConfiguration>,
    #[serde(rename = "cifsMountConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cifs_mount_configuration: Option<CifsMountConfiguration>,
    #[serde(rename = "azureFileShareConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_configuration: Option<AzureFileShareConfiguration>,
}
impl MountConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NfsMountConfiguration {
    pub source: String,
    #[doc = "All file systems are mounted relative to the Batch mounts directory, accessible via the AZ_BATCH_NODE_MOUNTS_DIR environment variable."]
    #[serde(rename = "relativeMountPath")]
    pub relative_mount_path: String,
    #[doc = "These are 'net use' options in Windows and 'mount' options in Linux."]
    #[serde(rename = "mountOptions", default, skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<String>,
}
impl NfsMountConfiguration {
    pub fn new(source: String, relative_mount_path: String) -> Self {
        Self {
            source,
            relative_mount_path,
            mount_options: None,
        }
    }
}
#[doc = "The network configuration for a pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkConfiguration {
    #[doc = "The virtual network must be in the same region and subscription as the Azure Batch account. The specified subnet should have enough free IP addresses to accommodate the number of nodes in the pool. If the subnet doesn't have enough free IP addresses, the pool will partially allocate compute nodes and a resize error will occur. The 'MicrosoftAzureBatch' service principal must have the 'Classic Virtual Machine Contributor' Role-Based Access Control (RBAC) role for the specified VNet. The specified subnet must allow communication from the Azure Batch service to be able to schedule tasks on the compute nodes. This can be verified by checking if the specified VNet has any associated Network Security Groups (NSG). If communication to the compute nodes in the specified subnet is denied by an NSG, then the Batch service will set the state of the compute nodes to unusable. If the specified VNet has any associated Network Security Groups (NSG), then a few reserved system ports must be enabled for inbound communication. For pools created with a virtual machine configuration, enable ports 29876 and 29877, as well as port 22 for Linux and port 3389 for Windows. For pools created with a cloud service configuration, enable ports 10100, 20100, and 30100. Also enable outbound connections to Azure Storage on port 443. For cloudServiceConfiguration pools, only 'classic' VNETs are supported. For more details see: https://docs.microsoft.com/en-us/azure/batch/batch-api-basics#virtual-network-vnet-and-firewall-configuration"]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "endpointConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<PoolEndpointConfiguration>,
    #[doc = "The public IP Address configuration of the networking configuration of a Pool."]
    #[serde(rename = "publicIPAddressConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address_configuration: Option<PublicIpAddressConfiguration>,
}
impl NetworkConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkSecurityGroupRule {
    #[doc = "Priorities within a pool must be unique and are evaluated in order of priority. The lower the number the higher the priority. For example, rules could be specified with order numbers of 150, 250, and 350. The rule with the order number of 150 takes precedence over the rule that has an order of 250. Allowed priorities are 150 to 4096. If any reserved or duplicate values are provided the request fails with HTTP status code 400."]
    pub priority: i32,
    pub access: network_security_group_rule::Access,
    #[doc = "Valid values are a single IP address (i.e. 10.10.10.10), IP subnet (i.e. 192.168.1.0/24), default tag, or * (for all addresses).  If any other values are provided the request fails with HTTP status code 400."]
    #[serde(rename = "sourceAddressPrefix")]
    pub source_address_prefix: String,
    #[doc = "Valid values are '*' (for all ports 0 - 65535) or arrays of ports or port ranges (i.e. 100-200). The ports should in the range of 0 to 65535 and the port ranges or ports can't overlap. If any other values are provided the request fails with HTTP status code 400. Default value will be *."]
    #[serde(rename = "sourcePortRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub source_port_ranges: Vec<String>,
}
impl NetworkSecurityGroupRule {
    pub fn new(priority: i32, access: network_security_group_rule::Access, source_address_prefix: String) -> Self {
        Self {
            priority,
            access,
            source_address_prefix,
            source_port_ranges: Vec::new(),
        }
    }
}
pub mod network_security_group_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Access {
        Allow,
        Deny,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "This is of the format {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "For example: read, write, delete, or listKeys/action"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains information about a pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Pool {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Pool properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PoolProperties>,
}
impl Pool {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The allocation mode for creating pools in the Batch account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PoolAllocationMode {
    BatchService,
    UserSubscription,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolEndpointConfiguration {
    #[doc = "The maximum number of inbound NAT pools per Batch pool is 5. If the maximum number of inbound NAT pools is exceeded the request fails with HTTP status code 400. This cannot be specified if the IPAddressProvisioningType is NoPublicIPAddresses."]
    #[serde(rename = "inboundNatPools")]
    pub inbound_nat_pools: Vec<InboundNatPool>,
}
impl PoolEndpointConfiguration {
    pub fn new(inbound_nat_pools: Vec<InboundNatPool>) -> Self {
        Self { inbound_nat_pools }
    }
}
#[doc = "Pool properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolProperties {
    #[doc = "The display name need not be unique and can contain any Unicode characters up to a maximum length of 1024."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "This is the last time at which the pool level data, such as the targetDedicatedNodes or autoScaleSettings, changed. It does not factor in node-level changes such as a compute node changing state."]
    #[serde(rename = "lastModified", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<pool_properties::ProvisioningState>,
    #[serde(rename = "provisioningStateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_state_transition_time: Option<time::OffsetDateTime>,
    #[serde(rename = "allocationState", default, skip_serializing_if = "Option::is_none")]
    pub allocation_state: Option<pool_properties::AllocationState>,
    #[serde(rename = "allocationStateTransitionTime", default, with = "azure_core::date::rfc3339::option")]
    pub allocation_state_transition_time: Option<time::OffsetDateTime>,
    #[doc = "For information about available sizes of virtual machines for Cloud Services pools (pools created with cloudServiceConfiguration), see Sizes for Cloud Services (https://azure.microsoft.com/documentation/articles/cloud-services-sizes-specs/). Batch supports all Cloud Services VM sizes except ExtraSmall. For information about available VM sizes for pools using images from the Virtual Machines Marketplace (pools created with virtualMachineConfiguration) see Sizes for Virtual Machines (Linux) (https://azure.microsoft.com/documentation/articles/virtual-machines-linux-sizes/) or Sizes for Virtual Machines (Windows) (https://azure.microsoft.com/documentation/articles/virtual-machines-windows-sizes/). Batch supports all Azure VM sizes except STANDARD_A0 and those with premium storage (STANDARD_GS, STANDARD_DS, and STANDARD_DSV2 series)."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[serde(rename = "deploymentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<DeploymentConfiguration>,
    #[serde(rename = "currentDedicatedNodes", default, skip_serializing_if = "Option::is_none")]
    pub current_dedicated_nodes: Option<i32>,
    #[serde(rename = "currentLowPriorityNodes", default, skip_serializing_if = "Option::is_none")]
    pub current_low_priority_nodes: Option<i32>,
    #[doc = "Defines the desired size of the pool. This can either be 'fixedScale' where the requested targetDedicatedNodes is specified, or 'autoScale' which defines a formula which is periodically reevaluated. If this property is not specified, the pool will have a fixed scale with 0 targetDedicatedNodes."]
    #[serde(rename = "scaleSettings", default, skip_serializing_if = "Option::is_none")]
    pub scale_settings: Option<ScaleSettings>,
    #[serde(rename = "autoScaleRun", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale_run: Option<AutoScaleRun>,
    #[doc = "This imposes restrictions on which nodes can be assigned to the pool. Enabling this value can reduce the chance of the requested number of nodes to be allocated in the pool. If not specified, this value defaults to 'Disabled'."]
    #[serde(rename = "interNodeCommunication", default, skip_serializing_if = "Option::is_none")]
    pub inter_node_communication: Option<pool_properties::InterNodeCommunication>,
    #[doc = "The network configuration for a pool."]
    #[serde(rename = "networkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[doc = "The default value is 1. The maximum value is the smaller of 4 times the number of cores of the vmSize of the pool or 256."]
    #[serde(rename = "taskSlotsPerNode", default, skip_serializing_if = "Option::is_none")]
    pub task_slots_per_node: Option<i32>,
    #[serde(rename = "taskSchedulingPolicy", default, skip_serializing_if = "Option::is_none")]
    pub task_scheduling_policy: Option<TaskSchedulingPolicy>,
    #[serde(rename = "userAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub user_accounts: Vec<UserAccount>,
    #[doc = "The Batch service does not assign any meaning to metadata; it is solely for the use of user code."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<MetadataItem>,
    #[doc = "In some cases the start task may be re-run even though the node was not rebooted. Due to this, start tasks should be idempotent and exit gracefully if the setup they're performing has already been done. Special care should be taken to avoid start tasks which create breakaway process or install/launch services from the start task working directory, as this will block Batch from being able to re-run the start task."]
    #[serde(rename = "startTask", default, skip_serializing_if = "Option::is_none")]
    pub start_task: Option<StartTask>,
    #[doc = "For Windows compute nodes, the Batch service installs the certificates to the specified certificate store and location. For Linux compute nodes, the certificates are stored in a directory inside the task working directory and an environment variable AZ_BATCH_CERTIFICATES_DIR is supplied to the task to query for this location. For certificates with visibility of 'remoteUser', a 'certs' directory is created in the user's home directory (e.g., /home/{user-name}/certs) and certificates are placed in that directory."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certificates: Vec<CertificateReference>,
    #[doc = "Changes to application package references affect all new compute nodes joining the pool, but do not affect compute nodes that are already in the pool until they are rebooted or reimaged. There is a maximum of 10 application package references on any given pool."]
    #[serde(rename = "applicationPackages", default, skip_serializing_if = "Vec::is_empty")]
    pub application_packages: Vec<ApplicationPackageReference>,
    #[doc = "The list of application licenses must be a subset of available Batch service application licenses. If a license is requested which is not supported, pool creation will fail."]
    #[serde(rename = "applicationLicenses", default, skip_serializing_if = "Vec::is_empty")]
    pub application_licenses: Vec<String>,
    #[doc = "Describes either the current operation (if the pool AllocationState is Resizing) or the previously completed operation (if the AllocationState is Steady)."]
    #[serde(rename = "resizeOperationStatus", default, skip_serializing_if = "Option::is_none")]
    pub resize_operation_status: Option<ResizeOperationStatus>,
    #[doc = "This supports Azure Files, NFS, CIFS/SMB, and Blobfuse."]
    #[serde(rename = "mountConfiguration", default, skip_serializing_if = "Vec::is_empty")]
    pub mount_configuration: Vec<MountConfiguration>,
}
impl PoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod pool_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Deleting,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AllocationState {
        Steady,
        Resizing,
        Stopping,
    }
    #[doc = "This imposes restrictions on which nodes can be assigned to the pool. Enabling this value can reduce the chance of the requested number of nodes to be allocated in the pool. If not specified, this value defaults to 'Disabled'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum InterNodeCommunication {
        Enabled,
        Disabled,
    }
}
#[doc = "The private endpoint of the private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains information about a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Private endpoint connection properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint connection properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_endpoint_connection_properties::ProvisioningState>,
    #[doc = "The private endpoint of the private endpoint connection."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "The private link service connection state of the private endpoint connection"]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_endpoint_connection_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Updating,
        Failed,
    }
}
#[doc = "Contains information about a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Private link resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private link resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The group id is used to establish the private link connection."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The private link service connection state of the private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionState {
    pub status: PrivateLinkServiceConnectionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "actionRequired", default, skip_serializing_if = "Option::is_none")]
    pub action_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new(status: PrivateLinkServiceConnectionStatus) -> Self {
        Self {
            status,
            description: None,
            action_required: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateLinkServiceConnectionStatus {
    Approved,
    Pending,
    Rejected,
    Disconnected,
}
#[doc = "A definition of an Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "The ID of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The ETag of the resource, used for concurrency statements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The public IP Address configuration of the networking configuration of a Pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicIpAddressConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provision: Option<IpAddressProvisioningType>,
    #[doc = "The number of IPs specified here limits the maximum size of the Pool - 100 dedicated nodes or 100 low-priority nodes can be allocated for each public IP. For example, a pool needing 250 dedicated VMs would need at least 3 public IPs specified. Each element of this collection is of the form: /subscriptions/{subscription}/resourceGroups/{group}/providers/Microsoft.Network/publicIPAddresses/{ip}."]
    #[serde(rename = "ipAddressIds", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_address_ids: Vec<String>,
}
impl PublicIpAddressConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network access type for operating on the resources in the Batch account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PublicNetworkAccessType {
    Enabled,
    Disabled,
}
impl Default for PublicNetworkAccessType {
    fn default() -> Self {
        Self::Enabled
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResizeError {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    pub code: String,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ResizeError>,
}
impl ResizeError {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: Vec::new(),
        }
    }
}
#[doc = "Describes either the current operation (if the pool AllocationState is Resizing) or the previously completed operation (if the AllocationState is Steady)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResizeOperationStatus {
    #[serde(rename = "targetDedicatedNodes", default, skip_serializing_if = "Option::is_none")]
    pub target_dedicated_nodes: Option<i32>,
    #[serde(rename = "targetLowPriorityNodes", default, skip_serializing_if = "Option::is_none")]
    pub target_low_priority_nodes: Option<i32>,
    #[doc = "The default value is 15 minutes. The minimum value is 5 minutes. If you specify a value less than 5 minutes, the Batch service returns an error; if you are calling the REST API directly, the HTTP status code is 400 (Bad Request)."]
    #[serde(rename = "resizeTimeout", default, skip_serializing_if = "Option::is_none")]
    pub resize_timeout: Option<String>,
    #[serde(rename = "nodeDeallocationOption", default, skip_serializing_if = "Option::is_none")]
    pub node_deallocation_option: Option<ComputeNodeDeallocationOption>,
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "This property is set only if an error occurred during the last pool resize, and only when the pool allocationState is Steady."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ResizeError>,
}
impl ResizeOperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A definition of an Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The ID of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The tags of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceFile {
    #[doc = "The autoStorageContainerName, storageContainerUrl and httpUrl properties are mutually exclusive and one of them must be specified."]
    #[serde(rename = "autoStorageContainerName", default, skip_serializing_if = "Option::is_none")]
    pub auto_storage_container_name: Option<String>,
    #[doc = "The autoStorageContainerName, storageContainerUrl and httpUrl properties are mutually exclusive and one of them must be specified. This URL must be readable and listable using anonymous access; that is, the Batch service does not present any credentials when downloading the blob. There are two ways to get such a URL for a blob in Azure storage: include a Shared Access Signature (SAS) granting read and list permissions on the blob, or set the ACL for the blob or its container to allow public access."]
    #[serde(rename = "storageContainerUrl", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_url: Option<String>,
    #[doc = "The autoStorageContainerName, storageContainerUrl and httpUrl properties are mutually exclusive and one of them must be specified. If the URL is Azure Blob Storage, it must be readable using anonymous access; that is, the Batch service does not present any credentials when downloading the blob. There are two ways to get such a URL for a blob in Azure storage: include a Shared Access Signature (SAS) granting read permissions on the blob, or set the ACL for the blob or its container to allow public access."]
    #[serde(rename = "httpUrl", default, skip_serializing_if = "Option::is_none")]
    pub http_url: Option<String>,
    #[doc = "The property is valid only when autoStorageContainerName or storageContainerUrl is used. This prefix can be a partial filename or a subdirectory. If a prefix is not specified, all the files in the container will be downloaded."]
    #[serde(rename = "blobPrefix", default, skip_serializing_if = "Option::is_none")]
    pub blob_prefix: Option<String>,
    #[doc = "If the httpUrl property is specified, the filePath is required and describes the path which the file will be downloaded to, including the filename. Otherwise, if the autoStorageContainerName or storageContainerUrl property is specified, filePath is optional and is the directory to download the files to. In the case where filePath is used as a directory, any directory structure already associated with the input data will be retained in full and appended to the specified filePath directory. The specified relative path cannot break out of the task's working directory (for example by using '..')."]
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[doc = "This property applies only to files being downloaded to Linux compute nodes. It will be ignored if it is specified for a resourceFile which will be downloaded to a Windows node. If this property is not specified for a Linux node, then a default value of 0770 is applied to the file."]
    #[serde(rename = "fileMode", default, skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
}
impl ResourceFile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the desired size of the pool. This can either be 'fixedScale' where the requested targetDedicatedNodes is specified, or 'autoScale' which defines a formula which is periodically reevaluated. If this property is not specified, the pool will have a fixed scale with 0 targetDedicatedNodes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScaleSettings {
    #[serde(rename = "fixedScale", default, skip_serializing_if = "Option::is_none")]
    pub fixed_scale: Option<FixedScaleSettings>,
    #[serde(rename = "autoScale", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale: Option<AutoScaleSettings>,
}
impl ScaleSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "In some cases the start task may be re-run even though the node was not rebooted. Due to this, start tasks should be idempotent and exit gracefully if the setup they're performing has already been done. Special care should be taken to avoid start tasks which create breakaway process or install/launch services from the start task working directory, as this will block Batch from being able to re-run the start task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StartTask {
    #[doc = "The command line does not run under a shell, and therefore cannot take advantage of shell features such as environment variable expansion. If you want to take advantage of such features, you should invoke the shell in the command line, for example using \"cmd /c MyCommand\" in Windows or \"/bin/sh -c MyCommand\" in Linux. Required if any other properties of the startTask are specified."]
    #[serde(rename = "commandLine", default, skip_serializing_if = "Option::is_none")]
    pub command_line: Option<String>,
    #[serde(rename = "resourceFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_files: Vec<ResourceFile>,
    #[serde(rename = "environmentSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_settings: Vec<EnvironmentSetting>,
    #[doc = "Specify either the userName or autoUser property, but not both."]
    #[serde(rename = "userIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<UserIdentity>,
    #[doc = "The Batch service retries a task if its exit code is nonzero. Note that this value specifically controls the number of retries. The Batch service will try the task once, and may then retry up to this limit. For example, if the maximum retry count is 3, Batch tries the task up to 4 times (one initial try and 3 retries). If the maximum retry count is 0, the Batch service does not retry the task. If the maximum retry count is -1, the Batch service retries the task without limit."]
    #[serde(rename = "maxTaskRetryCount", default, skip_serializing_if = "Option::is_none")]
    pub max_task_retry_count: Option<i32>,
    #[doc = "If true and the start task fails on a compute node, the Batch service retries the start task up to its maximum retry count (maxTaskRetryCount). If the task has still not completed successfully after all retries, then the Batch service marks the compute node unusable, and will not schedule tasks to it. This condition can be detected via the node state and scheduling error detail. If false, the Batch service will not wait for the start task to complete. In this case, other tasks can start executing on the compute node while the start task is still running; and even if the start task fails, new tasks will continue to be scheduled on the node. The default is true."]
    #[serde(rename = "waitForSuccess", default, skip_serializing_if = "Option::is_none")]
    pub wait_for_success: Option<bool>,
    #[serde(rename = "containerSettings", default, skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<TaskContainerSettings>,
}
impl StartTask {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StorageAccountType {
    #[serde(rename = "Standard_LRS")]
    StandardLrs,
    #[serde(rename = "Premium_LRS")]
    PremiumLrs,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskContainerSettings {
    #[doc = "These additional options are supplied as arguments to the \"docker create\" command, in addition to those controlled by the Batch Service."]
    #[serde(rename = "containerRunOptions", default, skip_serializing_if = "Option::is_none")]
    pub container_run_options: Option<String>,
    #[doc = "This is the full image reference, as would be specified to \"docker pull\". If no tag is provided as part of the image name, the tag \":latest\" is used as a default."]
    #[serde(rename = "imageName")]
    pub image_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<ContainerRegistry>,
    #[serde(rename = "workingDirectory", default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<task_container_settings::WorkingDirectory>,
}
impl TaskContainerSettings {
    pub fn new(image_name: String) -> Self {
        Self {
            container_run_options: None,
            image_name,
            registry: None,
            working_directory: None,
        }
    }
}
pub mod task_container_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum WorkingDirectory {
        TaskWorkingDirectory,
        ContainerImageDefault,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSchedulingPolicy {
    #[serde(rename = "nodeFillType")]
    pub node_fill_type: task_scheduling_policy::NodeFillType,
}
impl TaskSchedulingPolicy {
    pub fn new(node_fill_type: task_scheduling_policy::NodeFillType) -> Self {
        Self { node_fill_type }
    }
}
pub mod task_scheduling_policy {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NodeFillType {
        Spread,
        Pack,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAccount {
    pub name: String,
    pub password: String,
    #[serde(rename = "elevationLevel", default, skip_serializing_if = "Option::is_none")]
    pub elevation_level: Option<ElevationLevel>,
    #[serde(rename = "linuxUserConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub linux_user_configuration: Option<LinuxUserConfiguration>,
    #[serde(rename = "windowsUserConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub windows_user_configuration: Option<WindowsUserConfiguration>,
}
impl UserAccount {
    pub fn new(name: String, password: String) -> Self {
        Self {
            name,
            password,
            elevation_level: None,
            linux_user_configuration: None,
            windows_user_configuration: None,
        }
    }
}
#[doc = "Specify either the userName or autoUser property, but not both."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserIdentity {
    #[doc = "The userName and autoUser properties are mutually exclusive; you must specify one but not both."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "autoUser", default, skip_serializing_if = "Option::is_none")]
    pub auto_user: Option<AutoUserSpecification>,
}
impl UserIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineConfiguration {
    #[serde(rename = "imageReference")]
    pub image_reference: ImageReference,
    #[doc = "The Batch node agent is a program that runs on each node in the pool, and provides the command-and-control interface between the node and the Batch service. There are different implementations of the node agent, known as SKUs, for different operating systems. You must specify a node agent SKU which matches the selected image reference. To get the list of supported node agent SKUs along with their list of verified image references, see the 'List supported node agent SKUs' operation."]
    #[serde(rename = "nodeAgentSkuId")]
    pub node_agent_sku_id: String,
    #[serde(rename = "windowsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<WindowsConfiguration>,
    #[doc = "This property must be specified if the compute nodes in the pool need to have empty data disks attached to them."]
    #[serde(rename = "dataDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disks: Vec<DataDisk>,
    #[doc = "This only applies to images that contain the Windows operating system, and should only be used when you hold valid on-premises licenses for the nodes which will be deployed. If omitted, no on-premises licensing discount is applied. Values are:\n\n Windows_Server - The on-premises license is for Windows Server.\n Windows_Client - The on-premises license is for Windows Client.\n"]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[serde(rename = "containerConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub container_configuration: Option<ContainerConfiguration>,
    #[doc = "The disk encryption configuration applied on compute nodes in the pool. Disk encryption configuration is not supported on Linux pool created with Virtual Machine Image or Shared Image Gallery Image."]
    #[serde(rename = "diskEncryptionConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_configuration: Option<DiskEncryptionConfiguration>,
}
impl VirtualMachineConfiguration {
    pub fn new(image_reference: ImageReference, node_agent_sku_id: String) -> Self {
        Self {
            image_reference,
            node_agent_sku_id,
            windows_configuration: None,
            data_disks: Vec::new(),
            license_type: None,
            container_configuration: None,
            disk_encryption_configuration: None,
        }
    }
}
#[doc = "A VM Family and its associated core quota for the Batch account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineFamilyCoreQuota {
    #[doc = "The Virtual Machine family name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The core quota for the VM family for the Batch account."]
    #[serde(rename = "coreQuota", default, skip_serializing_if = "Option::is_none")]
    pub core_quota: Option<i32>,
}
impl VirtualMachineFamilyCoreQuota {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsConfiguration {
    #[doc = "If omitted, the default value is true."]
    #[serde(rename = "enableAutomaticUpdates", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_updates: Option<bool>,
}
impl WindowsConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsUserConfiguration {
    #[doc = "Specifies login mode for the user. The default value for VirtualMachineConfiguration pools is interactive mode and for CloudServiceConfiguration pools is batch mode."]
    #[serde(rename = "loginMode", default, skip_serializing_if = "Option::is_none")]
    pub login_mode: Option<windows_user_configuration::LoginMode>,
}
impl WindowsUserConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod windows_user_configuration {
    use super::*;
    #[doc = "Specifies login mode for the user. The default value for VirtualMachineConfiguration pools is interactive mode and for CloudServiceConfiguration pools is batch mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LoginMode {
        Batch,
        Interactive,
    }
}

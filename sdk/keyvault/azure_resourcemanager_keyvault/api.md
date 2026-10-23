# azure_resourcemanager_keyvault

- **Description**: Azure Resource Manager client library for Azure Key Vault
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
  - `azure_core/default`

```rust
#![crate_name = "azure_resourcemanager_keyvault"]
#![crate_type = "lib"]
#![cfg_attr(docsrs, feature(doc_cfg))]
pub struct KeyVaultClient {
}
impl KeyVaultClient {
    pub fn endpoint(&self) -> &Url;
    pub fn get_key_vault_keys_client(&self) -> KeyVaultKeysClient;
    pub fn get_key_vault_managed_hsm_keys_client(&self) -> KeyVaultManagedHsmKeysClient;
    pub fn get_key_vault_managed_hsms_client(&self) -> KeyVaultManagedHsmsClient;
    pub fn get_key_vault_mhsm_private_endpoint_connections_client(&self) -> KeyVaultMHSMPrivateEndpointConnectionsClient;
    pub fn get_key_vault_mhsm_private_link_resources_client(&self) -> KeyVaultMHSMPrivateLinkResourcesClient;
    pub fn get_key_vault_mhsm_regions_client(&self) -> KeyVaultMHSMRegionsClient;
    pub fn get_key_vault_operations_client(&self) -> KeyVaultOperationsClient;
    pub fn get_key_vault_private_endpoint_connections_client(&self) -> KeyVaultPrivateEndpointConnectionsClient;
    pub fn get_key_vault_private_link_resources_client(&self) -> KeyVaultPrivateLinkResourcesClient;
    pub fn get_key_vault_secrets_client(&self) -> KeyVaultSecretsClient;
    pub fn get_key_vault_vaults_client(&self) -> KeyVaultVaultsClient;
    pub fn new(subscription_id: String, credential: Arc<dyn TokenCredential>, options: Option<KeyVaultClientOptions>) -> Result<Self>;
}
#[derive(Clone, Debug)]
pub struct KeyVaultClientOptions {
    pub api_version: String,
    pub client_options: azure_core::http::ClientOptions,
}
impl Default for KeyVaultClientOptions {
    fn default() -> Self;
}
pub mod clients {
    pub struct KeyVaultClient {
    }
    impl KeyVaultClient {
        pub fn endpoint(&self) -> &Url;
        pub fn get_key_vault_keys_client(&self) -> KeyVaultKeysClient;
        pub fn get_key_vault_managed_hsm_keys_client(&self) -> KeyVaultManagedHsmKeysClient;
        pub fn get_key_vault_managed_hsms_client(&self) -> KeyVaultManagedHsmsClient;
        pub fn get_key_vault_mhsm_private_endpoint_connections_client(&self) -> KeyVaultMHSMPrivateEndpointConnectionsClient;
        pub fn get_key_vault_mhsm_private_link_resources_client(&self) -> KeyVaultMHSMPrivateLinkResourcesClient;
        pub fn get_key_vault_mhsm_regions_client(&self) -> KeyVaultMHSMRegionsClient;
        pub fn get_key_vault_operations_client(&self) -> KeyVaultOperationsClient;
        pub fn get_key_vault_private_endpoint_connections_client(&self) -> KeyVaultPrivateEndpointConnectionsClient;
        pub fn get_key_vault_private_link_resources_client(&self) -> KeyVaultPrivateLinkResourcesClient;
        pub fn get_key_vault_secrets_client(&self) -> KeyVaultSecretsClient;
        pub fn get_key_vault_vaults_client(&self) -> KeyVaultVaultsClient;
        pub fn new(subscription_id: String, credential: Arc<dyn TokenCredential>, options: Option<KeyVaultClientOptions>) -> Result<Self>;
    }
    #[derive(Clone, Debug)]
    pub struct KeyVaultClientOptions {
        pub api_version: String,
        pub client_options: azure_core::http::ClientOptions,
    }
    impl Default for KeyVaultClientOptions {
        fn default() -> Self;
    }
    pub struct KeyVaultKeysClient {
    }
    impl KeyVaultKeysClient {
        pub async fn create_if_not_exist(&self, resource_group_name: &str, vault_name: &str, key_name: &str, parameters: RequestContent<KeyCreateParameters>, options: Option<KeyVaultKeysClientCreateIfNotExistOptions<'_>>) -> Result<Response<Key>>;
        pub fn endpoint(&self) -> &Url;
        pub async fn get(&self, resource_group_name: &str, vault_name: &str, key_name: &str, options: Option<KeyVaultKeysClientGetOptions<'_>>) -> Result<Response<Key>>;
        pub async fn get_version(&self, resource_group_name: &str, vault_name: &str, key_name: &str, key_version: &str, options: Option<KeyVaultKeysClientGetVersionOptions<'_>>) -> Result<Response<Key>>;
        pub fn list(&self, resource_group_name: &str, vault_name: &str, options: Option<KeyVaultKeysClientListOptions<'_>>) -> Result<Pager<KeyListResult>>;
        pub fn list_versions(&self, resource_group_name: &str, vault_name: &str, key_name: &str, options: Option<KeyVaultKeysClientListVersionsOptions<'_>>) -> Result<Pager<KeyListResult>>;
    }
    pub struct KeyVaultMHSMPrivateEndpointConnectionsClient {
    }
    impl KeyVaultMHSMPrivateEndpointConnectionsClient {
        pub fn begin_delete(&self, resource_group_name: &str, name: &str, private_endpoint_connection_name: &str, options: Option<KeyVaultMHSMPrivateEndpointConnectionsClientBeginDeleteOptions<'_>>) -> Result<Poller<KeyVaultMHSMPrivateEndpointConnectionsClientBeginDeleteOperationStatus>>;
        pub fn endpoint(&self) -> &Url;
        pub async fn get(&self, resource_group_name: &str, name: &str, private_endpoint_connection_name: &str, options: Option<KeyVaultMHSMPrivateEndpointConnectionsClientGetOptions<'_>>) -> Result<Response<MHSMPrivateEndpointConnection>>;
        pub fn list_by_resource(&self, resource_group_name: &str, name: &str, options: Option<KeyVaultMHSMPrivateEndpointConnectionsClientListByResourceOptions<'_>>) -> Result<Pager<MHSMPrivateEndpointConnectionsListResult>>;
        pub async fn put(&self, resource_group_name: &str, name: &str, private_endpoint_connection_name: &str, properties: RequestContent<MHSMPrivateEndpointConnection>, options: Option<KeyVaultMHSMPrivateEndpointConnectionsClientPutOptions<'_>>) -> Result<Response<MHSMPrivateEndpointConnection>>;
    }
    pub struct KeyVaultMHSMPrivateLinkResourcesClient {
    }
    impl KeyVaultMHSMPrivateLinkResourcesClient {
        pub fn endpoint(&self) -> &Url;
        pub async fn list_by_mhsm_resource(&self, resource_group_name: &str, name: &str, options: Option<KeyVaultMHSMPrivateLinkResourcesClientListByMhsmResourceOptions<'_>>) -> Result<Response<MHSMPrivateLinkResourceListResult>>;
    }
    pub struct KeyVaultMHSMRegionsClient {
    }
    impl KeyVaultMHSMRegionsClient {
        pub fn endpoint(&self) -> &Url;
        pub fn list_by_resource(&self, resource_group_name: &str, name: &str, options: Option<KeyVaultMHSMRegionsClientListByResourceOptions<'_>>) -> Result<Pager<MHSMRegionsListResult>>;
    }
    pub struct KeyVaultManagedHsmKeysClient {
    }
    impl KeyVaultManagedHsmKeysClient {
        pub async fn create_if_not_exist(&self, resource_group_name: &str, name: &str, key_name: &str, parameters: RequestContent<ManagedHsmKeyCreateParameters>, options: Option<KeyVaultManagedHsmKeysClientCreateIfNotExistOptions<'_>>) -> Result<Response<ManagedHsmKey>>;
        pub fn endpoint(&self) -> &Url;
        pub async fn get(&self, resource_group_name: &str, name: &str, key_name: &str, options: Option<KeyVaultManagedHsmKeysClientGetOptions<'_>>) -> Result<Response<ManagedHsmKey>>;
        pub async fn get_version(&self, resource_group_name: &str, name: &str, key_name: &str, key_version: &str, options: Option<KeyVaultManagedHsmKeysClientGetVersionOptions<'_>>) -> Result<Response<ManagedHsmKey>>;
        pub fn list(&self, resource_group_name: &str, name: &str, options: Option<KeyVaultManagedHsmKeysClientListOptions<'_>>) -> Result<Pager<ManagedHsmKeyListResult>>;
        pub fn list_versions(&self, resource_group_name: &str, name: &str, key_name: &str, options: Option<KeyVaultManagedHsmKeysClientListVersionsOptions<'_>>) -> Result<Pager<ManagedHsmKeyListResult>>;
    }
    pub struct KeyVaultManagedHsmsClient {
    }
    impl KeyVaultManagedHsmsClient {
        pub fn begin_create_or_update(&self, resource_group_name: &str, name: &str, parameters: RequestContent<ManagedHsm>, options: Option<KeyVaultManagedHsmsClientBeginCreateOrUpdateOptions<'_>>) -> Result<Poller<KeyVaultManagedHsmsClientBeginCreateOrUpdateOperationStatus>>;
        pub fn begin_delete(&self, resource_group_name: &str, name: &str, options: Option<KeyVaultManagedHsmsClientBeginDeleteOptions<'_>>) -> Result<Poller<KeyVaultManagedHsmsClientBeginDeleteOperationStatus>>;
        pub fn begin_purge_deleted(&self, location: &str, name: &str, options: Option<KeyVaultManagedHsmsClientBeginPurgeDeletedOptions<'_>>) -> Result<Poller<KeyVaultManagedHsmsClientBeginPurgeDeletedOperationStatus>>;
        pub fn begin_update(&self, resource_group_name: &str, name: &str, parameters: RequestContent<ManagedHsm>, options: Option<KeyVaultManagedHsmsClientBeginUpdateOptions<'_>>) -> Result<Poller<KeyVaultManagedHsmsClientBeginUpdateOperationStatus>>;
        pub async fn check_mhsm_name_availability(&self, mhsm_name: RequestContent<CheckMhsmNameAvailabilityParameters>, options: Option<KeyVaultManagedHsmsClientCheckMhsmNameAvailabilityOptions<'_>>) -> Result<Response<CheckMhsmNameAvailabilityResult>>;
        pub fn endpoint(&self) -> &Url;
        pub async fn get(&self, resource_group_name: &str, name: &str, options: Option<KeyVaultManagedHsmsClientGetOptions<'_>>) -> Result<Response<ManagedHsm>>;
        pub async fn get_deleted(&self, location: &str, name: &str, options: Option<KeyVaultManagedHsmsClientGetDeletedOptions<'_>>) -> Result<Response<DeletedManagedHsm>>;
        pub fn list_by_resource_group(&self, resource_group_name: &str, options: Option<KeyVaultManagedHsmsClientListByResourceGroupOptions<'_>>) -> Result<Pager<ManagedHsmListResult>>;
        pub fn list_by_subscription(&self, options: Option<KeyVaultManagedHsmsClientListBySubscriptionOptions<'_>>) -> Result<Pager<ManagedHsmListResult>>;
        pub fn list_deleted(&self, options: Option<KeyVaultManagedHsmsClientListDeletedOptions<'_>>) -> Result<Pager<DeletedManagedHsmListResult>>;
    }
    pub struct KeyVaultOperationsClient {
    }
    impl KeyVaultOperationsClient {
        pub fn endpoint(&self) -> &Url;
        pub fn list(&self, options: Option<KeyVaultOperationsClientListOptions<'_>>) -> Result<Pager<OperationListResult>>;
    }
    pub struct KeyVaultPrivateEndpointConnectionsClient {
    }
    impl KeyVaultPrivateEndpointConnectionsClient {
        pub fn begin_delete(&self, resource_group_name: &str, vault_name: &str, private_endpoint_connection_name: &str, options: Option<KeyVaultPrivateEndpointConnectionsClientBeginDeleteOptions<'_>>) -> Result<Poller<KeyVaultPrivateEndpointConnectionsClientBeginDeleteOperationStatus>>;
        pub fn endpoint(&self) -> &Url;
        pub async fn get(&self, resource_group_name: &str, vault_name: &str, private_endpoint_connection_name: &str, options: Option<KeyVaultPrivateEndpointConnectionsClientGetOptions<'_>>) -> Result<Response<PrivateEndpointConnection>>;
        pub fn list_by_resource(&self, resource_group_name: &str, vault_name: &str, options: Option<KeyVaultPrivateEndpointConnectionsClientListByResourceOptions<'_>>) -> Result<Pager<PrivateEndpointConnectionListResult>>;
        pub async fn put(&self, resource_group_name: &str, vault_name: &str, private_endpoint_connection_name: &str, properties: RequestContent<PrivateEndpointConnection>, options: Option<KeyVaultPrivateEndpointConnectionsClientPutOptions<'_>>) -> Result<Response<PrivateEndpointConnection>>;
    }
    pub struct KeyVaultPrivateLinkResourcesClient {
    }
    impl KeyVaultPrivateLinkResourcesClient {
        pub fn endpoint(&self) -> &Url;
        pub async fn list_by_vault(&self, resource_group_name: &str, vault_name: &str, options: Option<KeyVaultPrivateLinkResourcesClientListByVaultOptions<'_>>) -> Result<Response<PrivateLinkResourceListResult>>;
    }
    pub struct KeyVaultSecretsClient {
    }
    impl KeyVaultSecretsClient {
        pub async fn create_or_update(&self, resource_group_name: &str, vault_name: &str, secret_name: &str, parameters: RequestContent<SecretCreateOrUpdateParameters>, options: Option<KeyVaultSecretsClientCreateOrUpdateOptions<'_>>) -> Result<Response<Secret>>;
        pub fn endpoint(&self) -> &Url;
        pub async fn get(&self, resource_group_name: &str, vault_name: &str, secret_name: &str, options: Option<KeyVaultSecretsClientGetOptions<'_>>) -> Result<Response<Secret>>;
        pub fn list(&self, resource_group_name: &str, vault_name: &str, options: Option<KeyVaultSecretsClientListOptions<'_>>) -> Result<Pager<SecretListResult>>;
        pub async fn update(&self, resource_group_name: &str, vault_name: &str, secret_name: &str, parameters: RequestContent<SecretPatchParameters>, options: Option<KeyVaultSecretsClientUpdateOptions<'_>>) -> Result<Response<Secret>>;
    }
    pub struct KeyVaultVaultsClient {
    }
    impl KeyVaultVaultsClient {
        pub fn begin_create_or_update(&self, resource_group_name: &str, vault_name: &str, parameters: RequestContent<VaultCreateOrUpdateParameters>, options: Option<KeyVaultVaultsClientBeginCreateOrUpdateOptions<'_>>) -> Result<Poller<KeyVaultVaultsClientBeginCreateOrUpdateOperationStatus>>;
        pub fn begin_purge_deleted(&self, location: &str, vault_name: &str, options: Option<KeyVaultVaultsClientBeginPurgeDeletedOptions<'_>>) -> Result<Poller<KeyVaultVaultsClientBeginPurgeDeletedOperationStatus>>;
        pub async fn check_name_availability(&self, vault_name: RequestContent<VaultCheckNameAvailabilityParameters>, options: Option<KeyVaultVaultsClientCheckNameAvailabilityOptions<'_>>) -> Result<Response<CheckNameAvailabilityResult>>;
        pub async fn delete(&self, resource_group_name: &str, vault_name: &str, options: Option<KeyVaultVaultsClientDeleteOptions<'_>>) -> Result<Response<(), NoFormat>>;
        pub fn endpoint(&self) -> &Url;
        pub async fn get(&self, resource_group_name: &str, vault_name: &str, options: Option<KeyVaultVaultsClientGetOptions<'_>>) -> Result<Response<Vault>>;
        pub async fn get_deleted(&self, location: &str, vault_name: &str, options: Option<KeyVaultVaultsClientGetDeletedOptions<'_>>) -> Result<Response<DeletedVault>>;
        pub fn list(&self, options: Option<KeyVaultVaultsClientListOptions<'_>>) -> Result<Pager<ResourceListResult>>;
        pub fn list_by_resource_group(&self, resource_group_name: &str, options: Option<KeyVaultVaultsClientListByResourceGroupOptions<'_>>) -> Result<Pager<VaultListResult>>;
        pub fn list_by_subscription(&self, options: Option<KeyVaultVaultsClientListBySubscriptionOptions<'_>>) -> Result<Pager<VaultListResult>>;
        pub fn list_deleted(&self, options: Option<KeyVaultVaultsClientListDeletedOptions<'_>>) -> Result<Pager<DeletedVaultListResult>>;
        pub async fn update(&self, resource_group_name: &str, vault_name: &str, parameters: RequestContent<VaultPatchParameters>, options: Option<KeyVaultVaultsClientUpdateOptions<'_>>) -> Result<Response<Vault>>;
        pub async fn update_access_policy(&self, resource_group_name: &str, vault_name: &str, operation_kind: AccessPolicyUpdateKind, parameters: RequestContent<VaultAccessPolicyParameters>, options: Option<KeyVaultVaultsClientUpdateAccessPolicyOptions<'_>>) -> Result<Response<VaultAccessPolicyParameters>>;
    }
}
pub mod models {
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct AccessPolicyEntry {
        #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
        pub application_id: Option<String>,
        #[serde(rename = "objectId", skip_serializing_if = "Option::is_none")]
        pub object_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub permissions: Option<Permissions>,
        #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
        pub tenant_id: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct Action {
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        pub type_prop: Option<super::KeyRotationPolicyActionType>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct CheckMhsmNameAvailabilityParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    impl TryFrom<CheckMhsmNameAvailabilityParameters> for azure_core::http::RequestContent<super::CheckMhsmNameAvailabilityParameters> {
        type Error = Error;
        fn try_from(value: CheckMhsmNameAvailabilityParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct CheckMhsmNameAvailabilityResult {
        #[serde(skip_serializing)]
        pub message: Option<String>,
        #[serde(rename = "nameAvailable", skip_serializing)]
        pub name_available: Option<bool>,
        #[serde(skip_serializing)]
        pub reason: Option<super::Reason>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct CheckNameAvailabilityResult {
        #[serde(skip_serializing)]
        pub message: Option<String>,
        #[serde(rename = "nameAvailable", skip_serializing)]
        pub name_available: Option<bool>,
        #[serde(skip_serializing)]
        pub reason: Option<super::Reason>,
    }
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct CloudError {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<CloudErrorBody>,
    }
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct CloudErrorBody {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct DeletedManagedHsm {
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<DeletedManagedHsmProperties>,
        #[serde(rename = "systemData", skip_serializing)]
        pub system_data: Option<SystemData>,
        #[serde(rename = "type", skip_serializing)]
        pub type_prop: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct DeletedManagedHsmListResult {
        #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
        #[serde(default)]
        pub value: Vec<DeletedManagedHsm>,
    }
    #[async_trait]
    impl Page for super::DeletedManagedHsmListResult {
        type IntoIter = <Vec<DeletedManagedHsm> as IntoIterator>::IntoIter;
        type Item = DeletedManagedHsm;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct DeletedManagedHsmProperties {
        #[serde(default, rename = "deletionDate", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub deletion_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing)]
        pub location: Option<String>,
        #[serde(rename = "mhsmId", skip_serializing)]
        pub mhsm_id: Option<String>,
        #[serde(rename = "purgeProtectionEnabled", skip_serializing)]
        pub purge_protection_enabled: Option<bool>,
        #[serde(default, rename = "scheduledPurgeDate", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub scheduled_purge_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing)]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct DeletedVault {
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<DeletedVaultProperties>,
        #[serde(rename = "systemData", skip_serializing)]
        pub system_data: Option<SystemData>,
        #[serde(rename = "type", skip_serializing)]
        pub type_prop: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct DeletedVaultListResult {
        #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
        #[serde(default)]
        pub value: Vec<DeletedVault>,
    }
    #[async_trait]
    impl Page for super::DeletedVaultListResult {
        type IntoIter = <Vec<DeletedVault> as IntoIterator>::IntoIter;
        type Item = DeletedVault;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct DeletedVaultProperties {
        #[serde(default, rename = "deletionDate", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub deletion_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing)]
        pub location: Option<String>,
        #[serde(rename = "purgeProtectionEnabled", skip_serializing)]
        pub purge_protection_enabled: Option<bool>,
        #[serde(default, rename = "scheduledPurgeDate", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub scheduled_purge_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing)]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(rename = "vaultId", skip_serializing)]
        pub vault_id: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct DimensionProperties {
        #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "toBeExportedForShoebox", skip_serializing_if = "Option::is_none")]
        pub to_be_exported_for_shoebox: Option<bool>,
    }
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct Error {
        #[serde(skip_serializing)]
        pub code: Option<String>,
        #[serde(rename = "innererror", skip_serializing)]
        pub inner_error: Option<Box<Error>>,
        #[serde(skip_serializing)]
        pub message: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ErrorAdditionalInfo {
        #[serde(skip_serializing)]
        pub info: Option<azure_core::Value>,
        #[serde(rename = "type", skip_serializing)]
        pub type_prop: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ErrorDetail {
        #[serde(rename = "additionalInfo", skip_serializing)]
        pub additional_info: Option<Vec<ErrorAdditionalInfo>>,
        #[serde(skip_serializing)]
        pub code: Option<String>,
        #[serde(skip_serializing)]
        pub details: Option<Vec<ErrorDetail>>,
        #[serde(skip_serializing)]
        pub message: Option<String>,
        #[serde(skip_serializing)]
        pub target: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct IPRule {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct Key {
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub location: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<KeyProperties>,
        #[serde(rename = "systemData", skip_serializing)]
        pub system_data: Option<SystemData>,
        #[serde(skip_serializing)]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(rename = "type", skip_serializing)]
        pub type_prop: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct KeyAttributes {
        #[serde(skip_serializing)]
        pub created: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(rename = "exp", skip_serializing_if = "Option::is_none")]
        pub expires: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub exportable: Option<bool>,
        #[serde(rename = "nbf", skip_serializing_if = "Option::is_none")]
        pub not_before: Option<i64>,
        #[serde(rename = "recoveryLevel", skip_serializing)]
        pub recovery_level: Option<super::DeletionRecoveryLevel>,
        #[serde(skip_serializing)]
        pub updated: Option<i64>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct KeyCreateParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<KeyProperties>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    impl TryFrom<KeyCreateParameters> for azure_core::http::RequestContent<super::KeyCreateParameters> {
        type Error = Error;
        fn try_from(value: KeyCreateParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct KeyListResult {
        #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
        #[serde(default)]
        pub value: Vec<Key>,
    }
    #[async_trait]
    impl Page for super::KeyListResult {
        type IntoIter = <Vec<Key> as IntoIterator>::IntoIter;
        type Item = Key;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct KeyProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<KeyAttributes>,
        #[serde(rename = "curveName", skip_serializing_if = "Option::is_none")]
        pub curve_name: Option<super::JsonWebKeyCurveName>,
        #[serde(rename = "keyOps", skip_serializing_if = "Option::is_none")]
        pub key_ops: Option<Vec<super::JsonWebKeyOperation>>,
        #[serde(rename = "keySize", skip_serializing_if = "Option::is_none")]
        pub key_size: Option<i32>,
        #[serde(rename = "keyUri", skip_serializing)]
        pub key_uri: Option<String>,
        #[serde(rename = "keyUriWithVersion", skip_serializing)]
        pub key_uri_with_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub kty: Option<super::JsonWebKeyType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub release_policy: Option<KeyReleasePolicy>,
        #[serde(rename = "rotationPolicy", skip_serializing_if = "Option::is_none")]
        pub rotation_policy: Option<RotationPolicy>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct KeyReleasePolicy {
        #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub data: Option<Vec<u8>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct KeyRotationPolicyAttributes {
        #[serde(skip_serializing)]
        pub created: Option<i64>,
        #[serde(rename = "expiryTime", skip_serializing_if = "Option::is_none")]
        pub expiry_time: Option<String>,
        #[serde(skip_serializing)]
        pub updated: Option<i64>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultKeysClientCreateIfNotExistOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultKeysClientGetOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultKeysClientGetVersionOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultKeysClientListOptions<'a> {
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl KeyVaultKeysClientListOptions<'_> {
        pub fn into_owned(self) -> KeyVaultKeysClientListOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultKeysClientListVersionsOptions<'a> {
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl KeyVaultKeysClientListVersionsOptions<'_> {
        pub fn into_owned(self) -> KeyVaultKeysClientListVersionsOptions<'static>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct KeyVaultMHSMPrivateEndpointConnectionsClientBeginDeleteOperationStatus {
        #[serde(default, rename = "endTime", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub end_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing)]
        pub error: Option<ErrorDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(rename = "percentComplete", skip_serializing)]
        pub percent_complete: Option<f64>,
        #[serde(default, rename = "startTime", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub start_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<super::ResourceProvisioningState>,
    }
    impl StatusMonitor for super::KeyVaultMHSMPrivateEndpointConnectionsClientBeginDeleteOperationStatus {
        type Format = JsonFormat;
        type Output = MHSMPrivateEndpointConnection;
        fn status(&self) -> PollerStatus;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultMHSMPrivateEndpointConnectionsClientBeginDeleteOptions<'a> {
        pub method_options: azure_core::http::poller::resourcemanager::PollerOptions<'a>,
    }
    impl KeyVaultMHSMPrivateEndpointConnectionsClientBeginDeleteOptions<'_> {
        pub fn into_owned(self) -> KeyVaultMHSMPrivateEndpointConnectionsClientBeginDeleteOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultMHSMPrivateEndpointConnectionsClientGetOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultMHSMPrivateEndpointConnectionsClientListByResourceOptions<'a> {
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl KeyVaultMHSMPrivateEndpointConnectionsClientListByResourceOptions<'_> {
        pub fn into_owned(self) -> KeyVaultMHSMPrivateEndpointConnectionsClientListByResourceOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultMHSMPrivateEndpointConnectionsClientPutOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultMHSMPrivateLinkResourcesClientListByMhsmResourceOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultMHSMRegionsClientListByResourceOptions<'a> {
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl KeyVaultMHSMRegionsClientListByResourceOptions<'_> {
        pub fn into_owned(self) -> KeyVaultMHSMRegionsClientListByResourceOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultManagedHsmKeysClientCreateIfNotExistOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultManagedHsmKeysClientGetOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultManagedHsmKeysClientGetVersionOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultManagedHsmKeysClientListOptions<'a> {
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl KeyVaultManagedHsmKeysClientListOptions<'_> {
        pub fn into_owned(self) -> KeyVaultManagedHsmKeysClientListOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultManagedHsmKeysClientListVersionsOptions<'a> {
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl KeyVaultManagedHsmKeysClientListVersionsOptions<'_> {
        pub fn into_owned(self) -> KeyVaultManagedHsmKeysClientListVersionsOptions<'static>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct KeyVaultManagedHsmsClientBeginCreateOrUpdateOperationStatus {
        #[serde(default, rename = "endTime", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub end_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing)]
        pub error: Option<ErrorDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(rename = "percentComplete", skip_serializing)]
        pub percent_complete: Option<f64>,
        #[serde(default, rename = "startTime", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub start_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<super::ResourceProvisioningState>,
    }
    impl StatusMonitor for super::KeyVaultManagedHsmsClientBeginCreateOrUpdateOperationStatus {
        type Format = JsonFormat;
        type Output = ManagedHsm;
        fn status(&self) -> PollerStatus;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultManagedHsmsClientBeginCreateOrUpdateOptions<'a> {
        pub method_options: azure_core::http::poller::resourcemanager::PollerOptions<'a>,
    }
    impl KeyVaultManagedHsmsClientBeginCreateOrUpdateOptions<'_> {
        pub fn into_owned(self) -> KeyVaultManagedHsmsClientBeginCreateOrUpdateOptions<'static>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct KeyVaultManagedHsmsClientBeginDeleteOperationStatus {
        #[serde(default, rename = "endTime", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub end_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing)]
        pub error: Option<ErrorDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(rename = "percentComplete", skip_serializing)]
        pub percent_complete: Option<f64>,
        #[serde(default, rename = "startTime", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub start_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<super::ResourceProvisioningState>,
    }
    impl StatusMonitor for super::KeyVaultManagedHsmsClientBeginDeleteOperationStatus {
        type Format = NoFormat;
        type Output = ();
        fn status(&self) -> PollerStatus;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultManagedHsmsClientBeginDeleteOptions<'a> {
        pub method_options: azure_core::http::poller::resourcemanager::PollerOptions<'a>,
    }
    impl KeyVaultManagedHsmsClientBeginDeleteOptions<'_> {
        pub fn into_owned(self) -> KeyVaultManagedHsmsClientBeginDeleteOptions<'static>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct KeyVaultManagedHsmsClientBeginPurgeDeletedOperationStatus {
        #[serde(default, rename = "endTime", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub end_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing)]
        pub error: Option<ErrorDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(rename = "percentComplete", skip_serializing)]
        pub percent_complete: Option<f64>,
        #[serde(default, rename = "startTime", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub start_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<super::ResourceProvisioningState>,
    }
    impl StatusMonitor for super::KeyVaultManagedHsmsClientBeginPurgeDeletedOperationStatus {
        type Format = NoFormat;
        type Output = ();
        fn status(&self) -> PollerStatus;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultManagedHsmsClientBeginPurgeDeletedOptions<'a> {
        pub method_options: azure_core::http::poller::resourcemanager::PollerOptions<'a>,
    }
    impl KeyVaultManagedHsmsClientBeginPurgeDeletedOptions<'_> {
        pub fn into_owned(self) -> KeyVaultManagedHsmsClientBeginPurgeDeletedOptions<'static>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct KeyVaultManagedHsmsClientBeginUpdateOperationStatus {
        #[serde(default, rename = "endTime", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub end_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing)]
        pub error: Option<ErrorDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(rename = "percentComplete", skip_serializing)]
        pub percent_complete: Option<f64>,
        #[serde(default, rename = "startTime", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub start_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<super::ResourceProvisioningState>,
    }
    impl StatusMonitor for super::KeyVaultManagedHsmsClientBeginUpdateOperationStatus {
        type Format = JsonFormat;
        type Output = ManagedHsm;
        fn status(&self) -> PollerStatus;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultManagedHsmsClientBeginUpdateOptions<'a> {
        pub method_options: azure_core::http::poller::resourcemanager::PollerOptions<'a>,
    }
    impl KeyVaultManagedHsmsClientBeginUpdateOptions<'_> {
        pub fn into_owned(self) -> KeyVaultManagedHsmsClientBeginUpdateOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultManagedHsmsClientCheckMhsmNameAvailabilityOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultManagedHsmsClientGetDeletedOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultManagedHsmsClientGetOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultManagedHsmsClientListByResourceGroupOptions<'a> {
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub top: Option<i32>,
    }
    impl KeyVaultManagedHsmsClientListByResourceGroupOptions<'_> {
        pub fn into_owned(self) -> KeyVaultManagedHsmsClientListByResourceGroupOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultManagedHsmsClientListBySubscriptionOptions<'a> {
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub top: Option<i32>,
    }
    impl KeyVaultManagedHsmsClientListBySubscriptionOptions<'_> {
        pub fn into_owned(self) -> KeyVaultManagedHsmsClientListBySubscriptionOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultManagedHsmsClientListDeletedOptions<'a> {
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl KeyVaultManagedHsmsClientListDeletedOptions<'_> {
        pub fn into_owned(self) -> KeyVaultManagedHsmsClientListDeletedOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultOperationsClientListOptions<'a> {
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl KeyVaultOperationsClientListOptions<'_> {
        pub fn into_owned(self) -> KeyVaultOperationsClientListOptions<'static>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct KeyVaultPrivateEndpointConnectionsClientBeginDeleteOperationStatus {
        #[serde(default, rename = "endTime", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub end_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing)]
        pub error: Option<ErrorDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(rename = "percentComplete", skip_serializing)]
        pub percent_complete: Option<f64>,
        #[serde(default, rename = "startTime", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub start_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<super::ResourceProvisioningState>,
    }
    impl StatusMonitor for super::KeyVaultPrivateEndpointConnectionsClientBeginDeleteOperationStatus {
        type Format = JsonFormat;
        type Output = PrivateEndpointConnection;
        fn status(&self) -> PollerStatus;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultPrivateEndpointConnectionsClientBeginDeleteOptions<'a> {
        pub method_options: azure_core::http::poller::resourcemanager::PollerOptions<'a>,
    }
    impl KeyVaultPrivateEndpointConnectionsClientBeginDeleteOptions<'_> {
        pub fn into_owned(self) -> KeyVaultPrivateEndpointConnectionsClientBeginDeleteOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultPrivateEndpointConnectionsClientGetOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultPrivateEndpointConnectionsClientListByResourceOptions<'a> {
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl KeyVaultPrivateEndpointConnectionsClientListByResourceOptions<'_> {
        pub fn into_owned(self) -> KeyVaultPrivateEndpointConnectionsClientListByResourceOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultPrivateEndpointConnectionsClientPutOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultPrivateLinkResourcesClientListByVaultOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultSecretsClientCreateOrUpdateOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultSecretsClientGetOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultSecretsClientListOptions<'a> {
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub top: Option<i32>,
    }
    impl KeyVaultSecretsClientListOptions<'_> {
        pub fn into_owned(self) -> KeyVaultSecretsClientListOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultSecretsClientUpdateOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct KeyVaultVaultsClientBeginCreateOrUpdateOperationStatus {
        #[serde(default, rename = "endTime", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub end_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing)]
        pub error: Option<ErrorDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(rename = "percentComplete", skip_serializing)]
        pub percent_complete: Option<f64>,
        #[serde(default, rename = "startTime", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub start_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<super::ResourceProvisioningState>,
    }
    impl StatusMonitor for super::KeyVaultVaultsClientBeginCreateOrUpdateOperationStatus {
        type Format = JsonFormat;
        type Output = Vault;
        fn status(&self) -> PollerStatus;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultVaultsClientBeginCreateOrUpdateOptions<'a> {
        pub method_options: azure_core::http::poller::resourcemanager::PollerOptions<'a>,
    }
    impl KeyVaultVaultsClientBeginCreateOrUpdateOptions<'_> {
        pub fn into_owned(self) -> KeyVaultVaultsClientBeginCreateOrUpdateOptions<'static>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct KeyVaultVaultsClientBeginPurgeDeletedOperationStatus {
        #[serde(default, rename = "endTime", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub end_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing)]
        pub error: Option<ErrorDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(rename = "percentComplete", skip_serializing)]
        pub percent_complete: Option<f64>,
        #[serde(default, rename = "startTime", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub start_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<super::ResourceProvisioningState>,
    }
    impl StatusMonitor for super::KeyVaultVaultsClientBeginPurgeDeletedOperationStatus {
        type Format = NoFormat;
        type Output = ();
        fn status(&self) -> PollerStatus;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultVaultsClientBeginPurgeDeletedOptions<'a> {
        pub method_options: azure_core::http::poller::resourcemanager::PollerOptions<'a>,
    }
    impl KeyVaultVaultsClientBeginPurgeDeletedOptions<'_> {
        pub fn into_owned(self) -> KeyVaultVaultsClientBeginPurgeDeletedOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultVaultsClientCheckNameAvailabilityOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultVaultsClientDeleteOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultVaultsClientGetDeletedOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultVaultsClientGetOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultVaultsClientListByResourceGroupOptions<'a> {
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub top: Option<i32>,
    }
    impl KeyVaultVaultsClientListByResourceGroupOptions<'_> {
        pub fn into_owned(self) -> KeyVaultVaultsClientListByResourceGroupOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultVaultsClientListBySubscriptionOptions<'a> {
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub top: Option<i32>,
    }
    impl KeyVaultVaultsClientListBySubscriptionOptions<'_> {
        pub fn into_owned(self) -> KeyVaultVaultsClientListBySubscriptionOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultVaultsClientListDeletedOptions<'a> {
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
    }
    impl KeyVaultVaultsClientListDeletedOptions<'_> {
        pub fn into_owned(self) -> KeyVaultVaultsClientListDeletedOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultVaultsClientListOptions<'a> {
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub top: Option<i32>,
    }
    impl KeyVaultVaultsClientListOptions<'_> {
        pub fn into_owned(self) -> KeyVaultVaultsClientListOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultVaultsClientUpdateAccessPolicyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct KeyVaultVaultsClientUpdateOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct LifetimeAction {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub action: Option<Action>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trigger: Option<Trigger>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct LogSpecification {
        #[serde(rename = "blobDuration", skip_serializing_if = "Option::is_none")]
        pub blob_duration: Option<String>,
        #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct MHSMGeoReplicatedRegion {
        #[serde(rename = "isPrimary", skip_serializing_if = "Option::is_none")]
        pub is_primary: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "provisioningState", skip_serializing)]
        pub provisioning_state: Option<super::GeoReplicationRegionProvisioningState>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct MHSMIPRule {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct MHSMNetworkRuleSet {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bypass: Option<super::NetworkRuleBypassOptions>,
        #[serde(rename = "defaultAction", skip_serializing_if = "Option::is_none")]
        pub default_action: Option<super::NetworkRuleAction>,
        #[serde(rename = "ipRules", skip_serializing_if = "Option::is_none")]
        pub ip_rules: Option<Vec<MHSMIPRule>>,
        #[serde(rename = "serviceTags", skip_serializing_if = "Option::is_none")]
        pub service_tags: Option<Vec<MHSMServiceTagRule>>,
        #[serde(rename = "virtualNetworkRules", skip_serializing_if = "Option::is_none")]
        pub virtual_network_rules: Option<Vec<MHSMVirtualNetworkRule>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct MHSMPrivateEndpoint {
        #[serde(skip_serializing)]
        pub id: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct MHSMPrivateEndpointConnection {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub etag: Option<azure_core::http::Etag>,
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub identity: Option<ManagedServiceIdentity>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<MHSMPrivateEndpointConnectionProperties>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sku: Option<ManagedHsmSku>,
        #[serde(rename = "systemData", skip_serializing)]
        pub system_data: Option<SystemData>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(rename = "type", skip_serializing)]
        pub type_prop: Option<String>,
    }
    impl TryFrom<MHSMPrivateEndpointConnection> for azure_core::http::RequestContent<super::MHSMPrivateEndpointConnection> {
        type Error = Error;
        fn try_from(value: MHSMPrivateEndpointConnection) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct MHSMPrivateEndpointConnectionItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub etag: Option<azure_core::http::Etag>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<MHSMPrivateEndpointConnectionProperties>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct MHSMPrivateEndpointConnectionProperties {
        #[serde(rename = "privateEndpoint", skip_serializing_if = "Option::is_none")]
        pub private_endpoint: Option<MHSMPrivateEndpoint>,
        #[serde(rename = "privateLinkServiceConnectionState", skip_serializing_if = "Option::is_none")]
        pub private_link_service_connection_state: Option<MHSMPrivateLinkServiceConnectionState>,
        #[serde(rename = "provisioningState", skip_serializing)]
        pub provisioning_state: Option<super::PrivateEndpointConnectionProvisioningState>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct MHSMPrivateEndpointConnectionsListResult {
        #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
        #[serde(default)]
        pub value: Vec<MHSMPrivateEndpointConnection>,
    }
    #[async_trait]
    impl Page for super::MHSMPrivateEndpointConnectionsListResult {
        type IntoIter = <Vec<MHSMPrivateEndpointConnection> as IntoIterator>::IntoIter;
        type Item = MHSMPrivateEndpointConnection;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct MHSMPrivateLinkResource {
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub identity: Option<ManagedServiceIdentity>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<MHSMPrivateLinkResourceProperties>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sku: Option<ManagedHsmSku>,
        #[serde(rename = "systemData", skip_serializing)]
        pub system_data: Option<SystemData>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(rename = "type", skip_serializing)]
        pub type_prop: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct MHSMPrivateLinkResourceListResult {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<Vec<MHSMPrivateLinkResource>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct MHSMPrivateLinkResourceProperties {
        #[serde(rename = "groupId", skip_serializing)]
        pub group_id: Option<String>,
        #[serde(rename = "requiredMembers", skip_serializing)]
        pub required_members: Option<Vec<String>>,
        #[serde(rename = "requiredZoneNames", skip_serializing_if = "Option::is_none")]
        pub required_zone_names: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct MHSMPrivateLinkServiceConnectionState {
        #[serde(rename = "actionsRequired", skip_serializing_if = "Option::is_none")]
        pub actions_required: Option<super::ActionsRequired>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<super::PrivateEndpointServiceConnectionStatus>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct MHSMRegionsListResult {
        #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
        #[serde(default)]
        pub value: Vec<MHSMGeoReplicatedRegion>,
    }
    #[async_trait]
    impl Page for super::MHSMRegionsListResult {
        type IntoIter = <Vec<MHSMGeoReplicatedRegion> as IntoIterator>::IntoIter;
        type Item = MHSMGeoReplicatedRegion;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct MHSMServiceTagRule {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tag: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct MHSMVirtualNetworkRule {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ManagedHSMSecurityDomainProperties {
        #[serde(rename = "activationStatus", skip_serializing)]
        pub activation_status: Option<super::ActivationStatus>,
        #[serde(rename = "activationStatusMessage", skip_serializing)]
        pub activation_status_message: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ManagedHsm {
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub identity: Option<ManagedServiceIdentity>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<ManagedHsmProperties>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sku: Option<ManagedHsmSku>,
        #[serde(rename = "systemData", skip_serializing)]
        pub system_data: Option<SystemData>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(rename = "type", skip_serializing)]
        pub type_prop: Option<String>,
    }
    impl TryFrom<ManagedHsm> for azure_core::http::RequestContent<super::ManagedHsm> {
        type Error = Error;
        fn try_from(value: ManagedHsm) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ManagedHsmAction {
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        pub type_prop: Option<super::KeyRotationPolicyActionType>,
    }
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct ManagedHsmError {
        #[serde(skip_serializing)]
        pub error: Option<Error>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ManagedHsmKey {
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<ManagedHsmKeyProperties>,
        #[serde(rename = "systemData", skip_serializing)]
        pub system_data: Option<SystemData>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(rename = "type", skip_serializing)]
        pub type_prop: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ManagedHsmKeyAttributes {
        #[serde(skip_serializing)]
        pub created: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(rename = "exp", skip_serializing_if = "Option::is_none")]
        pub expires: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub exportable: Option<bool>,
        #[serde(rename = "nbf", skip_serializing_if = "Option::is_none")]
        pub not_before: Option<i64>,
        #[serde(rename = "recoveryLevel", skip_serializing)]
        pub recovery_level: Option<super::DeletionRecoveryLevel>,
        #[serde(skip_serializing)]
        pub updated: Option<i64>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ManagedHsmKeyCreateParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<ManagedHsmKeyProperties>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    impl TryFrom<ManagedHsmKeyCreateParameters> for azure_core::http::RequestContent<super::ManagedHsmKeyCreateParameters> {
        type Error = Error;
        fn try_from(value: ManagedHsmKeyCreateParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ManagedHsmKeyListResult {
        #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
        #[serde(default)]
        pub value: Vec<ManagedHsmKey>,
    }
    #[async_trait]
    impl Page for super::ManagedHsmKeyListResult {
        type IntoIter = <Vec<ManagedHsmKey> as IntoIterator>::IntoIter;
        type Item = ManagedHsmKey;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ManagedHsmKeyProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<ManagedHsmKeyAttributes>,
        #[serde(rename = "curveName", skip_serializing_if = "Option::is_none")]
        pub curve_name: Option<super::JsonWebKeyCurveName>,
        #[serde(rename = "keyOps", skip_serializing_if = "Option::is_none")]
        pub key_ops: Option<Vec<super::JsonWebKeyOperation>>,
        #[serde(rename = "keySize", skip_serializing_if = "Option::is_none")]
        pub key_size: Option<i32>,
        #[serde(rename = "keyUri", skip_serializing)]
        pub key_uri: Option<String>,
        #[serde(rename = "keyUriWithVersion", skip_serializing)]
        pub key_uri_with_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub kty: Option<super::JsonWebKeyType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub release_policy: Option<ManagedHsmKeyReleasePolicy>,
        #[serde(rename = "rotationPolicy", skip_serializing_if = "Option::is_none")]
        pub rotation_policy: Option<ManagedHsmRotationPolicy>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ManagedHsmKeyReleasePolicy {
        #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(default, deserialize_with = "base64::option::deserialize_url_safe", serialize_with = "base64::option::serialize_url_safe", skip_serializing_if = "Option::is_none")]
        pub data: Option<Vec<u8>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ManagedHsmKeyRotationPolicyAttributes {
        #[serde(skip_serializing)]
        pub created: Option<i64>,
        #[serde(rename = "expiryTime", skip_serializing_if = "Option::is_none")]
        pub expiry_time: Option<String>,
        #[serde(skip_serializing)]
        pub updated: Option<i64>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ManagedHsmLifetimeAction {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub action: Option<ManagedHsmAction>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trigger: Option<ManagedHsmTrigger>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ManagedHsmListResult {
        #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
        #[serde(default)]
        pub value: Vec<ManagedHsm>,
    }
    #[async_trait]
    impl Page for super::ManagedHsmListResult {
        type IntoIter = <Vec<ManagedHsm> as IntoIterator>::IntoIter;
        type Item = ManagedHsm;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ManagedHsmProperties {
        #[serde(rename = "createMode", skip_serializing_if = "Option::is_none")]
        pub create_mode: Option<super::CreateMode>,
        #[serde(rename = "enablePurgeProtection", skip_serializing_if = "Option::is_none")]
        pub enable_purge_protection: Option<bool>,
        #[serde(rename = "enableSoftDelete", skip_serializing_if = "Option::is_none")]
        pub enable_soft_delete: Option<bool>,
        #[serde(rename = "hsmUri", skip_serializing)]
        pub hsm_uri: Option<String>,
        #[serde(rename = "initialAdminObjectIds", skip_serializing_if = "Option::is_none")]
        pub initial_admin_object_ids: Option<Vec<String>>,
        #[serde(rename = "networkAcls", skip_serializing_if = "Option::is_none")]
        pub network_acls: Option<MHSMNetworkRuleSet>,
        #[serde(rename = "privateEndpointConnections", skip_serializing)]
        pub private_endpoint_connections: Option<Vec<MHSMPrivateEndpointConnectionItem>>,
        #[serde(rename = "provisioningState", skip_serializing)]
        pub provisioning_state: Option<super::ProvisioningState>,
        #[serde(rename = "publicNetworkAccess", skip_serializing_if = "Option::is_none")]
        pub public_network_access: Option<super::PublicNetworkAccess>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub regions: Option<Vec<MHSMGeoReplicatedRegion>>,
        #[serde(default, rename = "scheduledPurgeDate", skip_serializing, with = "azure_core::time::rfc3339::option")]
        pub scheduled_purge_date: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "securityDomainProperties", skip_serializing)]
        pub security_domain_properties: Option<ManagedHSMSecurityDomainProperties>,
        #[serde(rename = "softDeleteRetentionInDays", skip_serializing_if = "Option::is_none")]
        pub soft_delete_retention_in_days: Option<i32>,
        #[serde(rename = "statusMessage", skip_serializing)]
        pub status_message: Option<String>,
        #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
        pub tenant_id: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ManagedHsmRotationPolicy {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<ManagedHsmKeyRotationPolicyAttributes>,
        #[serde(rename = "lifetimeActions", skip_serializing_if = "Option::is_none")]
        pub lifetime_actions: Option<Vec<ManagedHsmLifetimeAction>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ManagedHsmSku {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub family: Option<super::ManagedHsmSkuFamily>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<super::ManagedHsmSkuNameV2>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ManagedHsmTrigger {
        #[serde(rename = "timeAfterCreate", skip_serializing_if = "Option::is_none")]
        pub time_after_create: Option<String>,
        #[serde(rename = "timeBeforeExpiry", skip_serializing_if = "Option::is_none")]
        pub time_before_expiry: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct ManagedServiceIdentity {
        #[serde(rename = "principalId", skip_serializing)]
        pub principal_id: Option<String>,
        #[serde(rename = "tenantId", skip_serializing)]
        pub tenant_id: Option<String>,
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        pub type_prop: Option<super::ManagedServiceIdentityType>,
        #[serde(rename = "userAssignedIdentities", skip_serializing_if = "Option::is_none")]
        pub user_assigned_identities: Option<std::collections::HashMap<String, UserAssignedIdentity>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct MetricSpecification {
        #[serde(rename = "aggregationType", skip_serializing_if = "Option::is_none")]
        pub aggregation_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dimensions: Option<Vec<DimensionProperties>>,
        #[serde(rename = "displayDescription", skip_serializing_if = "Option::is_none")]
        pub display_description: Option<String>,
        #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[serde(rename = "fillGapWithZero", skip_serializing_if = "Option::is_none")]
        pub fill_gap_with_zero: Option<bool>,
        #[serde(rename = "internalMetricName", skip_serializing_if = "Option::is_none")]
        pub internal_metric_name: Option<String>,
        #[serde(rename = "lockAggregationType", skip_serializing_if = "Option::is_none")]
        pub lock_aggregation_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "supportedAggregationTypes", skip_serializing_if = "Option::is_none")]
        pub supported_aggregation_types: Option<Vec<String>>,
        #[serde(rename = "supportedTimeGrainTypes", skip_serializing_if = "Option::is_none")]
        pub supported_time_grain_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct NetworkRuleSet {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bypass: Option<super::NetworkRuleBypassOptions>,
        #[serde(rename = "defaultAction", skip_serializing_if = "Option::is_none")]
        pub default_action: Option<super::NetworkRuleAction>,
        #[serde(rename = "ipRules", skip_serializing_if = "Option::is_none")]
        pub ip_rules: Option<Vec<IPRule>>,
        #[serde(rename = "virtualNetworkRules", skip_serializing_if = "Option::is_none")]
        pub virtual_network_rules: Option<Vec<VirtualNetworkRule>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct Operation {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub display: Option<OperationDisplay>,
        #[serde(rename = "isDataAction", skip_serializing_if = "Option::is_none")]
        pub is_data_action: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
        pub operation_properties: Option<OperationProperties>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub origin: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct OperationDisplay {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct OperationListResult {
        #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
        #[serde(default)]
        pub value: Vec<Operation>,
    }
    #[async_trait]
    impl Page for super::OperationListResult {
        type IntoIter = <Vec<Operation> as IntoIterator>::IntoIter;
        type Item = Operation;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct OperationProperties {
        #[serde(rename = "serviceSpecification", skip_serializing_if = "Option::is_none")]
        pub service_specification: Option<ServiceSpecification>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct Permissions {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub certificates: Option<Vec<super::CertificatePermissions>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub keys: Option<Vec<super::KeyPermissions>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub secrets: Option<Vec<super::SecretPermissions>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub storage: Option<Vec<super::StoragePermissions>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct PrivateEndpoint {
        #[serde(skip_serializing)]
        pub id: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct PrivateEndpointConnection {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub etag: Option<azure_core::http::Etag>,
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub location: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<PrivateEndpointConnectionProperties>,
        #[serde(rename = "systemData", skip_serializing)]
        pub system_data: Option<SystemData>,
        #[serde(skip_serializing)]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(rename = "type", skip_serializing)]
        pub type_prop: Option<String>,
    }
    impl TryFrom<PrivateEndpointConnection> for azure_core::http::RequestContent<super::PrivateEndpointConnection> {
        type Error = Error;
        fn try_from(value: PrivateEndpointConnection) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct PrivateEndpointConnectionItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub etag: Option<azure_core::http::Etag>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<PrivateEndpointConnectionProperties>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct PrivateEndpointConnectionListResult {
        #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
        #[serde(default)]
        pub value: Vec<PrivateEndpointConnection>,
    }
    #[async_trait]
    impl Page for super::PrivateEndpointConnectionListResult {
        type IntoIter = <Vec<PrivateEndpointConnection> as IntoIterator>::IntoIter;
        type Item = PrivateEndpointConnection;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct PrivateEndpointConnectionProperties {
        #[serde(rename = "privateEndpoint", skip_serializing_if = "Option::is_none")]
        pub private_endpoint: Option<PrivateEndpoint>,
        #[serde(rename = "privateLinkServiceConnectionState", skip_serializing_if = "Option::is_none")]
        pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
        #[serde(rename = "provisioningState", skip_serializing)]
        pub provisioning_state: Option<super::PrivateEndpointConnectionProvisioningState>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct PrivateLinkResource {
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub location: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<PrivateLinkResourceProperties>,
        #[serde(rename = "systemData", skip_serializing)]
        pub system_data: Option<SystemData>,
        #[serde(skip_serializing)]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(rename = "type", skip_serializing)]
        pub type_prop: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct PrivateLinkResourceListResult {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<Vec<PrivateLinkResource>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct PrivateLinkResourceProperties {
        #[serde(rename = "groupId", skip_serializing)]
        pub group_id: Option<String>,
        #[serde(rename = "requiredMembers", skip_serializing)]
        pub required_members: Option<Vec<String>>,
        #[serde(rename = "requiredZoneNames", skip_serializing_if = "Option::is_none")]
        pub required_zone_names: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct PrivateLinkServiceConnectionState {
        #[serde(rename = "actionsRequired", skip_serializing_if = "Option::is_none")]
        pub actions_required: Option<super::ActionsRequired>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<super::PrivateEndpointServiceConnectionStatus>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ResourceListResult {
        #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
        #[serde(default)]
        pub value: Vec<TrackedResource>,
    }
    #[async_trait]
    impl Page for super::ResourceListResult {
        type IntoIter = <Vec<TrackedResource> as IntoIterator>::IntoIter;
        type Item = TrackedResource;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct RotationPolicy {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<KeyRotationPolicyAttributes>,
        #[serde(rename = "lifetimeActions", skip_serializing_if = "Option::is_none")]
        pub lifetime_actions: Option<Vec<LifetimeAction>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct Secret {
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub location: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<SecretProperties>,
        #[serde(rename = "systemData", skip_serializing)]
        pub system_data: Option<SystemData>,
        #[serde(skip_serializing)]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(rename = "type", skip_serializing)]
        pub type_prop: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct SecretAttributes {
        #[serde(default, skip_serializing, with = "azure_core::time::unix_time::option")]
        pub created: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(default, rename = "exp", skip_serializing_if = "Option::is_none", with = "azure_core::time::unix_time::option")]
        pub expires: Option<azure_core::time::OffsetDateTime>,
        #[serde(default, rename = "nbf", skip_serializing_if = "Option::is_none", with = "azure_core::time::unix_time::option")]
        pub not_before: Option<azure_core::time::OffsetDateTime>,
        #[serde(default, skip_serializing, with = "azure_core::time::unix_time::option")]
        pub updated: Option<azure_core::time::OffsetDateTime>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct SecretCreateOrUpdateParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<SecretProperties>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    impl TryFrom<SecretCreateOrUpdateParameters> for azure_core::http::RequestContent<super::SecretCreateOrUpdateParameters> {
        type Error = Error;
        fn try_from(value: SecretCreateOrUpdateParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct SecretListResult {
        #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
        #[serde(default)]
        pub value: Vec<Secret>,
    }
    #[async_trait]
    impl Page for super::SecretListResult {
        type IntoIter = <Vec<Secret> as IntoIterator>::IntoIter;
        type Item = Secret;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct SecretPatchParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<SecretPatchProperties>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    impl TryFrom<SecretPatchParameters> for azure_core::http::RequestContent<super::SecretPatchParameters> {
        type Error = Error;
        fn try_from(value: SecretPatchParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct SecretPatchProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<SecretAttributes>,
        #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct SecretProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<SecretAttributes>,
        #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(rename = "secretUri", skip_serializing)]
        pub secret_uri: Option<String>,
        #[serde(rename = "secretUriWithVersion", skip_serializing)]
        pub secret_uri_with_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ServiceSpecification {
        #[serde(rename = "logSpecifications", skip_serializing_if = "Option::is_none")]
        pub log_specifications: Option<Vec<LogSpecification>>,
        #[serde(rename = "metricSpecifications", skip_serializing_if = "Option::is_none")]
        pub metric_specifications: Option<Vec<MetricSpecification>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct Sku {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub family: Option<super::SkuFamily>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<super::SkuName>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct SystemData {
        #[serde(default, rename = "createdAt", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc3339::option")]
        pub created_at: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
        pub created_by: Option<String>,
        #[serde(rename = "createdByType", skip_serializing_if = "Option::is_none")]
        pub created_by_type: Option<super::CreatedByType>,
        #[serde(default, rename = "lastModifiedAt", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc3339::option")]
        pub last_modified_at: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "lastModifiedBy", skip_serializing_if = "Option::is_none")]
        pub last_modified_by: Option<String>,
        #[serde(rename = "lastModifiedByType", skip_serializing_if = "Option::is_none")]
        pub last_modified_by_type: Option<super::CreatedByType>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct TrackedResource {
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(rename = "systemData", skip_serializing)]
        pub system_data: Option<SystemData>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(rename = "type", skip_serializing)]
        pub type_prop: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct Trigger {
        #[serde(rename = "timeAfterCreate", skip_serializing_if = "Option::is_none")]
        pub time_after_create: Option<String>,
        #[serde(rename = "timeBeforeExpiry", skip_serializing_if = "Option::is_none")]
        pub time_before_expiry: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct UserAssignedIdentity {
        #[serde(rename = "clientId", skip_serializing)]
        pub client_id: Option<String>,
        #[serde(rename = "principalId", skip_serializing)]
        pub principal_id: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct Vault {
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<VaultProperties>,
        #[serde(rename = "systemData", skip_serializing)]
        pub system_data: Option<SystemData>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        #[serde(rename = "type", skip_serializing)]
        pub type_prop: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct VaultAccessPolicyParameters {
        #[serde(skip_serializing)]
        pub id: Option<String>,
        #[serde(skip_serializing)]
        pub location: Option<String>,
        #[serde(skip_serializing)]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<VaultAccessPolicyProperties>,
        #[serde(rename = "type", skip_serializing)]
        pub type_prop: Option<String>,
    }
    impl TryFrom<VaultAccessPolicyParameters> for azure_core::http::RequestContent<super::VaultAccessPolicyParameters> {
        type Error = Error;
        fn try_from(value: VaultAccessPolicyParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct VaultAccessPolicyProperties {
        #[serde(rename = "accessPolicies", skip_serializing_if = "Option::is_none")]
        pub access_policies: Option<Vec<AccessPolicyEntry>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct VaultCheckNameAvailabilityParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "type", serialize_with = "models_serde::serialize_string_literal_Microsoft_KeyVault_vaults")]
        pub type_prop: Option<String>,
    }
    impl TryFrom<VaultCheckNameAvailabilityParameters> for azure_core::http::RequestContent<super::VaultCheckNameAvailabilityParameters> {
        type Error = Error;
        fn try_from(value: VaultCheckNameAvailabilityParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct VaultCreateOrUpdateParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<VaultProperties>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    impl TryFrom<VaultCreateOrUpdateParameters> for azure_core::http::RequestContent<super::VaultCreateOrUpdateParameters> {
        type Error = Error;
        fn try_from(value: VaultCreateOrUpdateParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct VaultListResult {
        #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
        #[serde(default)]
        pub value: Vec<Vault>,
    }
    #[async_trait]
    impl Page for super::VaultListResult {
        type IntoIter = <Vec<Vault> as IntoIterator>::IntoIter;
        type Item = Vault;
        async fn into_items(self) -> Result<<Self as >::IntoIter>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct VaultPatchParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub properties: Option<VaultPatchProperties>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
    }
    impl TryFrom<VaultPatchParameters> for azure_core::http::RequestContent<super::VaultPatchParameters> {
        type Error = Error;
        fn try_from(value: VaultPatchParameters) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct VaultPatchProperties {
        #[serde(rename = "accessPolicies", skip_serializing_if = "Option::is_none")]
        pub access_policies: Option<Vec<AccessPolicyEntry>>,
        #[serde(rename = "createMode", skip_serializing_if = "Option::is_none")]
        pub create_mode: Option<super::CreateMode>,
        #[serde(rename = "enablePurgeProtection", skip_serializing_if = "Option::is_none")]
        pub enable_purge_protection: Option<bool>,
        #[serde(rename = "enableRbacAuthorization", skip_serializing_if = "Option::is_none")]
        pub enable_rbac_authorization: Option<bool>,
        #[serde(rename = "enableSoftDelete", skip_serializing_if = "Option::is_none")]
        pub enable_soft_delete: Option<bool>,
        #[serde(rename = "enabledForDeployment", skip_serializing_if = "Option::is_none")]
        pub enabled_for_deployment: Option<bool>,
        #[serde(rename = "enabledForDiskEncryption", skip_serializing_if = "Option::is_none")]
        pub enabled_for_disk_encryption: Option<bool>,
        #[serde(rename = "enabledForTemplateDeployment", skip_serializing_if = "Option::is_none")]
        pub enabled_for_template_deployment: Option<bool>,
        #[serde(rename = "networkAcls", skip_serializing_if = "Option::is_none")]
        pub network_acls: Option<NetworkRuleSet>,
        #[serde(rename = "publicNetworkAccess", skip_serializing_if = "Option::is_none")]
        pub public_network_access: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sku: Option<Sku>,
        #[serde(rename = "softDeleteRetentionInDays", skip_serializing_if = "Option::is_none")]
        pub soft_delete_retention_in_days: Option<i32>,
        #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
        pub tenant_id: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct VaultProperties {
        #[serde(rename = "accessPolicies", skip_serializing_if = "Option::is_none")]
        pub access_policies: Option<Vec<AccessPolicyEntry>>,
        #[serde(rename = "createMode", skip_serializing_if = "Option::is_none")]
        pub create_mode: Option<super::CreateMode>,
        #[serde(rename = "enablePurgeProtection", skip_serializing_if = "Option::is_none")]
        pub enable_purge_protection: Option<bool>,
        #[serde(rename = "enableRbacAuthorization", skip_serializing_if = "Option::is_none")]
        pub enable_rbac_authorization: Option<bool>,
        #[serde(rename = "enableSoftDelete", skip_serializing_if = "Option::is_none")]
        pub enable_soft_delete: Option<bool>,
        #[serde(rename = "enabledForDeployment", skip_serializing_if = "Option::is_none")]
        pub enabled_for_deployment: Option<bool>,
        #[serde(rename = "enabledForDiskEncryption", skip_serializing_if = "Option::is_none")]
        pub enabled_for_disk_encryption: Option<bool>,
        #[serde(rename = "enabledForTemplateDeployment", skip_serializing_if = "Option::is_none")]
        pub enabled_for_template_deployment: Option<bool>,
        #[serde(rename = "hsmPoolResourceId", skip_serializing)]
        pub hsm_pool_resource_id: Option<String>,
        #[serde(rename = "networkAcls", skip_serializing_if = "Option::is_none")]
        pub network_acls: Option<NetworkRuleSet>,
        #[serde(rename = "privateEndpointConnections", skip_serializing)]
        pub private_endpoint_connections: Option<Vec<PrivateEndpointConnectionItem>>,
        #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<super::VaultProvisioningState>,
        #[serde(rename = "publicNetworkAccess", skip_serializing_if = "Option::is_none")]
        pub public_network_access: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sku: Option<Sku>,
        #[serde(rename = "softDeleteRetentionInDays", skip_serializing_if = "Option::is_none")]
        pub soft_delete_retention_in_days: Option<i32>,
        #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
        pub tenant_id: Option<String>,
        #[serde(rename = "vaultUri", skip_serializing_if = "Option::is_none")]
        pub vault_uri: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct VirtualNetworkRule {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "ignoreMissingVnetServiceEndpoint", skip_serializing_if = "Option::is_none")]
        pub ignore_missing_vnet_service_endpoint: Option<bool>,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum AccessPolicyUpdateKind {
        Add,
        Remove,
        Replace,
    }
    impl AsRef<str> for super::AccessPolicyUpdateKind {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::AccessPolicyUpdateKind {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::AccessPolicyUpdateKind {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::AccessPolicyUpdateKind {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::AccessPolicyUpdateKind {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ActionsRequired {
        None,
        UnknownValue(String),
    }
    impl AsRef<str> for super::ActionsRequired {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::ActionsRequired {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::ActionsRequired {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::ActionsRequired {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a ActionsRequired> for &'a str {
        fn from(e: &'a ActionsRequired) -> Self;
    }
    impl<'de> Deserialize<'de> for super::ActionsRequired {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ActivationStatus {
        Active,
        Failed,
        NotActivated,
        Unknown,
        UnknownValue(String),
    }
    impl AsRef<str> for super::ActivationStatus {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::ActivationStatus {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::ActivationStatus {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::ActivationStatus {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a ActivationStatus> for &'a str {
        fn from(e: &'a ActivationStatus) -> Self;
    }
    impl<'de> Deserialize<'de> for super::ActivationStatus {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum CertificatePermissions {
        All,
        Backup,
        Create,
        Delete,
        Deleteissuers,
        Get,
        Getissuers,
        Import,
        List,
        Listissuers,
        Managecontacts,
        Manageissuers,
        Purge,
        Recover,
        Restore,
        Setissuers,
        Update,
        UnknownValue(String),
    }
    impl AsRef<str> for super::CertificatePermissions {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::CertificatePermissions {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::CertificatePermissions {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::CertificatePermissions {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a CertificatePermissions> for &'a str {
        fn from(e: &'a CertificatePermissions) -> Self;
    }
    impl<'de> Deserialize<'de> for super::CertificatePermissions {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CreateMode {
        Default,
        Recover,
    }
    impl AsRef<str> for super::CreateMode {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::CreateMode {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::CreateMode {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::CreateMode {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::CreateMode {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum CreatedByType {
        Application,
        Key,
        ManagedIdentity,
        User,
        UnknownValue(String),
    }
    impl AsRef<str> for super::CreatedByType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::CreatedByType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::CreatedByType {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::CreatedByType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a CreatedByType> for &'a str {
        fn from(e: &'a CreatedByType) -> Self;
    }
    impl<'de> Deserialize<'de> for super::CreatedByType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum DeletionRecoveryLevel {
        Purgeable,
        Recoverable,
        RecoverableProtectedSubscription,
        RecoverablePurgeable,
        UnknownValue(String),
    }
    impl AsRef<str> for super::DeletionRecoveryLevel {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::DeletionRecoveryLevel {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::DeletionRecoveryLevel {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::DeletionRecoveryLevel {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a DeletionRecoveryLevel> for &'a str {
        fn from(e: &'a DeletionRecoveryLevel) -> Self;
    }
    impl<'de> Deserialize<'de> for super::DeletionRecoveryLevel {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum GeoReplicationRegionProvisioningState {
        Cleanup,
        Deleting,
        Failed,
        Preprovisioning,
        Provisioning,
        Succeeded,
        UnknownValue(String),
    }
    impl AsRef<str> for super::GeoReplicationRegionProvisioningState {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::GeoReplicationRegionProvisioningState {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::GeoReplicationRegionProvisioningState {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::GeoReplicationRegionProvisioningState {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a GeoReplicationRegionProvisioningState> for &'a str {
        fn from(e: &'a GeoReplicationRegionProvisioningState) -> Self;
    }
    impl<'de> Deserialize<'de> for super::GeoReplicationRegionProvisioningState {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum JsonWebKeyCurveName {
        P256,
        P256K,
        P384,
        P521,
        UnknownValue(String),
    }
    impl AsRef<str> for super::JsonWebKeyCurveName {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::JsonWebKeyCurveName {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::JsonWebKeyCurveName {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::JsonWebKeyCurveName {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a JsonWebKeyCurveName> for &'a str {
        fn from(e: &'a JsonWebKeyCurveName) -> Self;
    }
    impl<'de> Deserialize<'de> for super::JsonWebKeyCurveName {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum JsonWebKeyOperation {
        Decrypt,
        Encrypt,
        Import,
        Release,
        Sign,
        UnwrapKey,
        Verify,
        WrapKey,
        UnknownValue(String),
    }
    impl AsRef<str> for super::JsonWebKeyOperation {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::JsonWebKeyOperation {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::JsonWebKeyOperation {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::JsonWebKeyOperation {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a JsonWebKeyOperation> for &'a str {
        fn from(e: &'a JsonWebKeyOperation) -> Self;
    }
    impl<'de> Deserialize<'de> for super::JsonWebKeyOperation {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum JsonWebKeyType {
        Ec,
        EcHsm,
        OctHsm,
        Rsa,
        RsaHsm,
        UnknownValue(String),
    }
    impl AsRef<str> for super::JsonWebKeyType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::JsonWebKeyType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::JsonWebKeyType {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::JsonWebKeyType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a JsonWebKeyType> for &'a str {
        fn from(e: &'a JsonWebKeyType) -> Self;
    }
    impl<'de> Deserialize<'de> for super::JsonWebKeyType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum KeyPermissions {
        All,
        Backup,
        Create,
        Decrypt,
        Delete,
        Encrypt,
        Get,
        Getrotationpolicy,
        Import,
        List,
        Purge,
        Recover,
        Release,
        Restore,
        Rotate,
        Setrotationpolicy,
        Sign,
        UnwrapKey,
        Update,
        Verify,
        WrapKey,
        UnknownValue(String),
    }
    impl AsRef<str> for super::KeyPermissions {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::KeyPermissions {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::KeyPermissions {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::KeyPermissions {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a KeyPermissions> for &'a str {
        fn from(e: &'a KeyPermissions) -> Self;
    }
    impl<'de> Deserialize<'de> for super::KeyPermissions {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum KeyRotationPolicyActionType {
        Notify,
        Rotate,
    }
    impl AsRef<str> for super::KeyRotationPolicyActionType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::KeyRotationPolicyActionType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::KeyRotationPolicyActionType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::KeyRotationPolicyActionType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::KeyRotationPolicyActionType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ManagedHsmSkuFamily {
        B,
        C,
        UnknownValue(String),
    }
    impl AsRef<str> for super::ManagedHsmSkuFamily {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::ManagedHsmSkuFamily {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::ManagedHsmSkuFamily {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::ManagedHsmSkuFamily {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a ManagedHsmSkuFamily> for &'a str {
        fn from(e: &'a ManagedHsmSkuFamily) -> Self;
    }
    impl<'de> Deserialize<'de> for super::ManagedHsmSkuFamily {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ManagedHsmSkuNameV2 {
        CustomB32,
        CustomB6,
        CustomC10,
        CustomC42,
        StandardB1,
        StandardB10V2,
        StandardB15V2,
        StandardB1V2,
        StandardB20V2,
        StandardB5V2,
        UnknownValue(String),
    }
    impl AsRef<str> for super::ManagedHsmSkuNameV2 {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::ManagedHsmSkuNameV2 {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::ManagedHsmSkuNameV2 {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::ManagedHsmSkuNameV2 {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a ManagedHsmSkuNameV2> for &'a str {
        fn from(e: &'a ManagedHsmSkuNameV2) -> Self;
    }
    impl<'de> Deserialize<'de> for super::ManagedHsmSkuNameV2 {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ManagedServiceIdentityType {
        None,
        SystemAssigned,
        SystemAssignedUserAssigned,
        UserAssigned,
        UnknownValue(String),
    }
    impl AsRef<str> for super::ManagedServiceIdentityType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::ManagedServiceIdentityType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::ManagedServiceIdentityType {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::ManagedServiceIdentityType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a ManagedServiceIdentityType> for &'a str {
        fn from(e: &'a ManagedServiceIdentityType) -> Self;
    }
    impl<'de> Deserialize<'de> for super::ManagedServiceIdentityType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum NetworkRuleAction {
        Allow,
        Deny,
        UnknownValue(String),
    }
    impl AsRef<str> for super::NetworkRuleAction {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::NetworkRuleAction {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::NetworkRuleAction {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::NetworkRuleAction {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a NetworkRuleAction> for &'a str {
        fn from(e: &'a NetworkRuleAction) -> Self;
    }
    impl<'de> Deserialize<'de> for super::NetworkRuleAction {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum NetworkRuleBypassOptions {
        AzureServices,
        None,
        UnknownValue(String),
    }
    impl AsRef<str> for super::NetworkRuleBypassOptions {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::NetworkRuleBypassOptions {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::NetworkRuleBypassOptions {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::NetworkRuleBypassOptions {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a NetworkRuleBypassOptions> for &'a str {
        fn from(e: &'a NetworkRuleBypassOptions) -> Self;
    }
    impl<'de> Deserialize<'de> for super::NetworkRuleBypassOptions {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum PrivateEndpointConnectionProvisioningState {
        Creating,
        Deleting,
        Disconnected,
        Failed,
        Succeeded,
        Updating,
        UnknownValue(String),
    }
    impl AsRef<str> for super::PrivateEndpointConnectionProvisioningState {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::PrivateEndpointConnectionProvisioningState {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::PrivateEndpointConnectionProvisioningState {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::PrivateEndpointConnectionProvisioningState {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a PrivateEndpointConnectionProvisioningState> for &'a str {
        fn from(e: &'a PrivateEndpointConnectionProvisioningState) -> Self;
    }
    impl<'de> Deserialize<'de> for super::PrivateEndpointConnectionProvisioningState {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum PrivateEndpointServiceConnectionStatus {
        Approved,
        Disconnected,
        Pending,
        Rejected,
        UnknownValue(String),
    }
    impl AsRef<str> for super::PrivateEndpointServiceConnectionStatus {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::PrivateEndpointServiceConnectionStatus {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::PrivateEndpointServiceConnectionStatus {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::PrivateEndpointServiceConnectionStatus {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a PrivateEndpointServiceConnectionStatus> for &'a str {
        fn from(e: &'a PrivateEndpointServiceConnectionStatus) -> Self;
    }
    impl<'de> Deserialize<'de> for super::PrivateEndpointServiceConnectionStatus {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ProvisioningState {
        Activated,
        Deleting,
        Failed,
        Provisioning,
        Restoring,
        SecurityDomainRestore,
        Succeeded,
        Updating,
        UnknownValue(String),
    }
    impl AsRef<str> for super::ProvisioningState {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::ProvisioningState {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::ProvisioningState {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::ProvisioningState {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a ProvisioningState> for &'a str {
        fn from(e: &'a ProvisioningState) -> Self;
    }
    impl<'de> Deserialize<'de> for super::ProvisioningState {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum PublicNetworkAccess {
        Disabled,
        Enabled,
        UnknownValue(String),
    }
    impl AsRef<str> for super::PublicNetworkAccess {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::PublicNetworkAccess {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::PublicNetworkAccess {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::PublicNetworkAccess {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a PublicNetworkAccess> for &'a str {
        fn from(e: &'a PublicNetworkAccess) -> Self;
    }
    impl<'de> Deserialize<'de> for super::PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Reason {
        AccountNameInvalid,
        AlreadyExists,
        UnknownValue(String),
    }
    impl AsRef<str> for super::Reason {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::Reason {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::Reason {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::Reason {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a Reason> for &'a str {
        fn from(e: &'a Reason) -> Self;
    }
    impl<'de> Deserialize<'de> for super::Reason {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ResourceProvisioningState {
        Canceled,
        Failed,
        Succeeded,
        UnknownValue(String),
    }
    impl AsRef<str> for super::ResourceProvisioningState {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::ResourceProvisioningState {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::ResourceProvisioningState {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::ResourceProvisioningState {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a ResourceProvisioningState> for &'a str {
        fn from(e: &'a ResourceProvisioningState) -> Self;
    }
    impl<'de> Deserialize<'de> for super::ResourceProvisioningState {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum SecretPermissions {
        All,
        Backup,
        Delete,
        Get,
        List,
        Purge,
        Recover,
        Restore,
        Set,
        UnknownValue(String),
    }
    impl AsRef<str> for super::SecretPermissions {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::SecretPermissions {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::SecretPermissions {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::SecretPermissions {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a SecretPermissions> for &'a str {
        fn from(e: &'a SecretPermissions) -> Self;
    }
    impl<'de> Deserialize<'de> for super::SecretPermissions {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum SkuFamily {
        A,
        UnknownValue(String),
    }
    impl AsRef<str> for super::SkuFamily {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::SkuFamily {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::SkuFamily {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::SkuFamily {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a SkuFamily> for &'a str {
        fn from(e: &'a SkuFamily) -> Self;
    }
    impl<'de> Deserialize<'de> for super::SkuFamily {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum SkuName {
        Premium,
        Standard,
    }
    impl AsRef<str> for super::SkuName {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::SkuName {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::SkuName {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::SkuName {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::SkuName {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum StoragePermissions {
        All,
        Backup,
        Delete,
        Deletesas,
        Get,
        Getsas,
        List,
        Listsas,
        Purge,
        Recover,
        Regeneratekey,
        Restore,
        Set,
        Setsas,
        Update,
        UnknownValue(String),
    }
    impl AsRef<str> for super::StoragePermissions {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::StoragePermissions {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::StoragePermissions {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::StoragePermissions {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a StoragePermissions> for &'a str {
        fn from(e: &'a StoragePermissions) -> Self;
    }
    impl<'de> Deserialize<'de> for super::StoragePermissions {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum VaultProvisioningState {
        RegisteringDns,
        Succeeded,
        UnknownValue(String),
    }
    impl AsRef<str> for super::VaultProvisioningState {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::VaultProvisioningState {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::VaultProvisioningState {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::VaultProvisioningState {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a VaultProvisioningState> for &'a str {
        fn from(e: &'a VaultProvisioningState) -> Self;
    }
    impl<'de> Deserialize<'de> for super::VaultProvisioningState {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    pub trait KeyVaultMHSMPrivateEndpointConnectionsClientBeginDeleteOperationStatusHeaders: private::Sealed {
        fn location(&self) -> Result<Option<String>>;
        fn retry_after(&self) -> Result<Option<i32>>;
    }
    pub trait KeyVaultManagedHsmsClientBeginCreateOrUpdateOperationStatusHeaders: private::Sealed {
        fn location(&self) -> Result<Option<String>>;
        fn retry_after(&self) -> Result<Option<i32>>;
    }
    pub trait KeyVaultManagedHsmsClientBeginDeleteOperationStatusHeaders: private::Sealed {
        fn location(&self) -> Result<Option<String>>;
        fn retry_after(&self) -> Result<Option<i32>>;
    }
    pub trait KeyVaultManagedHsmsClientBeginPurgeDeletedOperationStatusHeaders: private::Sealed {
        fn location(&self) -> Result<Option<String>>;
        fn retry_after(&self) -> Result<Option<i32>>;
    }
    pub trait KeyVaultManagedHsmsClientBeginUpdateOperationStatusHeaders: private::Sealed {
        fn location(&self) -> Result<Option<String>>;
        fn retry_after(&self) -> Result<Option<i32>>;
    }
    pub trait KeyVaultPrivateEndpointConnectionsClientBeginDeleteOperationStatusHeaders: private::Sealed {
        fn location(&self) -> Result<Option<String>>;
        fn retry_after(&self) -> Result<Option<i32>>;
    }
    pub trait KeyVaultVaultsClientBeginCreateOrUpdateOperationStatusHeaders: private::Sealed {
        fn location(&self) -> Result<Option<String>>;
        fn retry_after(&self) -> Result<Option<i32>>;
    }
    pub trait KeyVaultVaultsClientBeginPurgeDeletedOperationStatusHeaders: private::Sealed {
        fn location(&self) -> Result<Option<String>>;
        fn retry_after(&self) -> Result<Option<i32>>;
    }
    pub trait MHSMPrivateEndpointConnectionHeaders: private::Sealed {
        fn azure_async_operation(&self) -> Result<Option<String>>;
        fn retry_after(&self) -> Result<Option<i32>>;
    }
    pub trait PrivateEndpointConnectionHeaders: private::Sealed {
        fn azure_async_operation(&self) -> Result<Option<String>>;
        fn retry_after(&self) -> Result<Option<i32>>;
    }
}
```

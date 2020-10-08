#![allow(unused_mut)] // TODO
#![allow(unused_variables)] // TODO
use crate::*;
use anyhow::*;

pub async fn operations_list(configuration: &Configuration, api_version: &str) -> Result<OperationListResult> {
    let client = &configuration.client;
    let uri_str = &format!("{}/providers/Microsoft.Storage/operations", &configuration.base_path,);
    let mut req_builder = client.get(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn skus_list(configuration: &Configuration, api_version: &str, subscription_id: &str) -> Result<StorageSkuListResult> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/providers/Microsoft.Storage/skus",
        &configuration.base_path, subscription_id
    );
    let mut req_builder = client.get(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn storage_accounts_check_name_availability(
    configuration: &Configuration,
    account_name: StorageAccountCheckNameAvailabilityParameters,
    api_version: &str,
    subscription_id: &str,
) -> Result<CheckNameAvailabilityResult> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/providers/Microsoft.Storage/checkNameAvailability",
        &configuration.base_path, subscription_id
    );
    let mut req_builder = client.post(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn storage_accounts_get_properties(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
    expand: &str,
) -> Result<StorageAccount> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}",
        &configuration.base_path, subscription_id, resource_group_name, account_name
    );
    let mut req_builder = client.get(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn storage_accounts_create(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    parameters: StorageAccountCreateParameters,
    api_version: &str,
    subscription_id: &str,
) -> Result<StorageAccount> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}",
        &configuration.base_path, subscription_id, resource_group_name, account_name
    );
    let mut req_builder = client.put(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn storage_accounts_update(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    parameters: StorageAccountUpdateParameters,
    api_version: &str,
    subscription_id: &str,
) -> Result<StorageAccount> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}",
        &configuration.base_path, subscription_id, resource_group_name, account_name
    );
    let mut req_builder = client.patch(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn storage_accounts_delete(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
) -> Result<()> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}",
        &configuration.base_path, subscription_id, resource_group_name, account_name
    );
    let mut req_builder = client.delete(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn storage_accounts_list(
    configuration: &Configuration,
    api_version: &str,
    subscription_id: &str,
) -> Result<StorageAccountListResult> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/providers/Microsoft.Storage/storageAccounts",
        &configuration.base_path, subscription_id
    );
    let mut req_builder = client.get(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn storage_accounts_list_by_resource_group(
    configuration: &Configuration,
    resource_group_name: &str,
    api_version: &str,
    subscription_id: &str,
) -> Result<StorageAccountListResult> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts",
        &configuration.base_path, subscription_id, resource_group_name
    );
    let mut req_builder = client.get(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn storage_accounts_list_keys(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
    expand: &str,
) -> Result<StorageAccountListKeysResult> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/listKeys",
        &configuration.base_path, subscription_id, resource_group_name, account_name
    );
    let mut req_builder = client.post(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn storage_accounts_regenerate_key(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    regenerate_key: StorageAccountRegenerateKeyParameters,
    api_version: &str,
    subscription_id: &str,
) -> Result<StorageAccountListKeysResult> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/regenerateKey",
        &configuration.base_path, subscription_id, resource_group_name, account_name
    );
    let mut req_builder = client.post(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn usages_list_by_location(
    configuration: &Configuration,
    api_version: &str,
    subscription_id: &str,
    location: &str,
) -> Result<UsageListResult> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/providers/Microsoft.Storage/locations/{}/usages",
        &configuration.base_path, subscription_id, location
    );
    let mut req_builder = client.get(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn storage_accounts_list_account_sas(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    parameters: AccountSasParameters,
    api_version: &str,
    subscription_id: &str,
) -> Result<ListAccountSasResponse> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/ListAccountSas",
        &configuration.base_path, subscription_id, resource_group_name, account_name
    );
    let mut req_builder = client.post(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn storage_accounts_list_service_sas(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    parameters: ServiceSasParameters,
    api_version: &str,
    subscription_id: &str,
) -> Result<ListServiceSasResponse> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/ListServiceSas",
        &configuration.base_path, subscription_id, resource_group_name, account_name
    );
    let mut req_builder = client.post(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn storage_accounts_failover(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
) -> Result<()> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/failover",
        &configuration.base_path, subscription_id, resource_group_name, account_name
    );
    let mut req_builder = client.post(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn storage_accounts_restore_blob_ranges(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
    parameters: BlobRestoreParameters,
) -> Result<BlobRestoreStatus> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/restoreBlobRanges",
        &configuration.base_path, subscription_id, resource_group_name, account_name
    );
    let mut req_builder = client.post(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn management_policies_get(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
    management_policy_name: &str,
) -> Result<ManagementPolicy> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/managementPolicies/{}",
        &configuration.base_path, subscription_id, resource_group_name, account_name, management_policy_name
    );
    let mut req_builder = client.get(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn management_policies_create_or_update(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
    management_policy_name: &str,
    properties: ManagementPolicy,
) -> Result<ManagementPolicy> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/managementPolicies/{}",
        &configuration.base_path, subscription_id, resource_group_name, account_name, management_policy_name
    );
    let mut req_builder = client.put(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn management_policies_delete(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
    management_policy_name: &str,
) -> Result<()> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/managementPolicies/{}",
        &configuration.base_path, subscription_id, resource_group_name, account_name, management_policy_name
    );
    let mut req_builder = client.delete(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub type PrivateEndpointConnectionListResult = serde_json::Value; // TODO
pub async fn private_endpoint_connections_list(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
) -> Result<PrivateEndpointConnectionListResult> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/privateEndpointConnections",
        &configuration.base_path, subscription_id, resource_group_name, account_name
    );
    let mut req_builder = client.get(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn private_endpoint_connections_get(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
    private_endpoint_connection_name: &str,
) -> Result<PrivateEndpointConnection> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/privateEndpointConnections/{}",
        &configuration.base_path, subscription_id, resource_group_name, account_name, private_endpoint_connection_name
    );
    let mut req_builder = client.get(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn private_endpoint_connections_put(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
    private_endpoint_connection_name: &str,
    properties: PrivateEndpointConnection,
) -> Result<PrivateEndpointConnection> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/privateEndpointConnections/{}",
        &configuration.base_path, subscription_id, resource_group_name, account_name, private_endpoint_connection_name
    );
    let mut req_builder = client.put(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn private_endpoint_connections_delete(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
    private_endpoint_connection_name: &str,
) -> Result<ErrorResponse> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/privateEndpointConnections/{}",
        &configuration.base_path, subscription_id, resource_group_name, account_name, private_endpoint_connection_name
    );
    let mut req_builder = client.delete(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub type PrivateLinkResourceListResult = serde_json::Value; // TODO
pub async fn private_link_resources_list_by_storage_account(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
) -> Result<PrivateLinkResourceListResult> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/privateLinkResources",
        &configuration.base_path, subscription_id, resource_group_name, account_name
    );
    let mut req_builder = client.get(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn object_replication_policies_list(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
) -> Result<ObjectReplicationPolicies> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/objectReplicationPolicies",
        &configuration.base_path, subscription_id, resource_group_name, account_name
    );
    let mut req_builder = client.get(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn object_replication_policies_get(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
    object_replication_policy_id: &str,
) -> Result<ObjectReplicationPolicy> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/objectReplicationPolicies/{}",
        &configuration.base_path, subscription_id, resource_group_name, account_name, object_replication_policy_id
    );
    let mut req_builder = client.get(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn object_replication_policies_create_or_update(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
    object_replication_policy_id: &str,
    properties: ObjectReplicationPolicy,
) -> Result<ObjectReplicationPolicy> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/objectReplicationPolicies/{}",
        &configuration.base_path, subscription_id, resource_group_name, account_name, object_replication_policy_id
    );
    let mut req_builder = client.put(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn object_replication_policies_delete(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
    object_replication_policy_id: &str,
) -> Result<ErrorResponse> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/objectReplicationPolicies/{}",
        &configuration.base_path, subscription_id, resource_group_name, account_name, object_replication_policy_id
    );
    let mut req_builder = client.delete(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn storage_accounts_revoke_user_delegation_keys(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
) -> Result<()> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/revokeUserDelegationKeys",
        &configuration.base_path, subscription_id, resource_group_name, account_name
    );
    let mut req_builder = client.post(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn encryption_scopes_get(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
    encryption_scope_name: &str,
) -> Result<EncryptionScope> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/encryptionScopes/{}",
        &configuration.base_path, subscription_id, resource_group_name, account_name, encryption_scope_name
    );
    let mut req_builder = client.get(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn encryption_scopes_put(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
    encryption_scope_name: &str,
    encryption_scope: EncryptionScope,
) -> Result<EncryptionScope> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/encryptionScopes/{}",
        &configuration.base_path, subscription_id, resource_group_name, account_name, encryption_scope_name
    );
    let mut req_builder = client.put(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn encryption_scopes_patch(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
    encryption_scope_name: &str,
    encryption_scope: EncryptionScope,
) -> Result<EncryptionScope> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/encryptionScopes/{}",
        &configuration.base_path, subscription_id, resource_group_name, account_name, encryption_scope_name
    );
    let mut req_builder = client.patch(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}
pub async fn encryption_scopes_list(
    configuration: &Configuration,
    resource_group_name: &str,
    account_name: &str,
    api_version: &str,
    subscription_id: &str,
) -> Result<EncryptionScopeListResult> {
    let client = &configuration.client;
    let uri_str = &format!(
        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/encryptionScopes",
        &configuration.base_path, subscription_id, resource_group_name, account_name
    );
    let mut req_builder = client.get(uri_str);
    let req = req_builder.build()?;
    let res = client.execute(req).await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(res.json().await?),
        Err(err) => {
            let e = Error::new(err);
            let e = e.context(res.text().await?);
            Err(e)
        }
    }
}

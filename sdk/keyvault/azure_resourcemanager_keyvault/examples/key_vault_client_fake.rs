// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{
    http::{fake::ResponseExt as _, headers::Headers, AsyncRawResponse, StatusCode},
    Result,
};
use azure_resourcemanager_keyvault::{
    clients::fake::{
        KeyVaultKeysServer, KeyVaultMHSMPrivateEndpointConnectionsServer,
        KeyVaultMHSMPrivateLinkResourcesServer, KeyVaultMHSMRegionsServer,
        KeyVaultManagedHsmKeysServer, KeyVaultManagedHsmsServer,
        KeyVaultNetworkSecurityPerimeterConfigurationsServer, KeyVaultOperationsServer,
        KeyVaultPrivateEndpointConnectionsServer, KeyVaultPrivateLinkResourcesServer,
        KeyVaultSecretsServer, KeyVaultServer, KeyVaultVaultsServer,
    },
    models::KeyVaultKeysClientGetOptions,
    KeyVaultClient,
};
use std::sync::Arc;

struct FakeKeyVault;

#[async_trait]
impl KeyVaultServer for FakeKeyVault {}
impl KeyVaultManagedHsmKeysServer for FakeKeyVault {}
impl KeyVaultManagedHsmsServer for FakeKeyVault {}
impl KeyVaultMHSMPrivateEndpointConnectionsServer for FakeKeyVault {}
impl KeyVaultMHSMPrivateLinkResourcesServer for FakeKeyVault {}
impl KeyVaultMHSMRegionsServer for FakeKeyVault {}
impl KeyVaultNetworkSecurityPerimeterConfigurationsServer for FakeKeyVault {}
impl KeyVaultOperationsServer for FakeKeyVault {}
impl KeyVaultPrivateEndpointConnectionsServer for FakeKeyVault {}
impl KeyVaultPrivateLinkResourcesServer for FakeKeyVault {}
impl KeyVaultSecretsServer for FakeKeyVault {}
impl KeyVaultVaultsServer for FakeKeyVault {}

#[async_trait]
impl KeyVaultKeysServer for FakeKeyVault {
    async fn get(
        &self,
        resource_group_name: &str,
        vault_name: &str,
        key_name: &str,
        _options: Option<KeyVaultKeysClientGetOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        assert_eq!(resource_group_name, "my-resource-group");
        assert_eq!(vault_name, "my-vault");
        match key_name {
            "my-key" => Ok(AsyncRawResponse::fake_with_bytes(
                StatusCode::Ok,
                Headers::new(),
                r#"{"name":"my-key"}"#,
            )),
            // Return an HTTP response so the client applies its normal service-error handling.
            _ => AsyncRawResponse::fake_with_error(
                StatusCode::NotFound,
                Headers::new(),
                "KeyNotFound",
                "The requested key was not found.",
            ),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = KeyVaultClient::with_fake(
        "https://management.azure.com",
        Arc::new(FakeKeyVault),
        "00000000-0000-0000-0000-000000000000".into(),
        None,
    )?;
    let keys = client.get_key_vault_keys_client();
    let key = keys
        .get("my-resource-group", "my-vault", "my-key", None)
        .await?
        .into_model()?;
    assert_eq!(key.name.as_deref(), Some("my-key"));
    println!("Key name: {:?}", key.name);

    let error = keys
        .get("my-resource-group", "my-vault", "missing", None)
        .await
        .expect_err("the fake returns a 404 for an unknown key");
    assert_eq!(error.http_status(), Some(StatusCode::NotFound));
    println!("Expected error: {error}");
    Ok(())
}

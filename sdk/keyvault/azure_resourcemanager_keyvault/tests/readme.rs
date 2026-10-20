// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{http::Url, Result};
use azure_core_test::{recorded, TestContext, SANITIZE_BODY_NAME};
use azure_resourcemanager_keyvault::{KeyVaultClient, KeyVaultClientOptions};
use include_file::include_markdown;

#[recorded::test]
async fn readme(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    recording.remove_sanitizers(&[SANITIZE_BODY_NAME]).await?;
    let mut options = KeyVaultClientOptions::default();
    recording.instrument(&mut options.client_options);
    let client = KeyVaultClient::new(
        recording.var("KEYVAULT_SUBSCRIPTION_ID", None),
        recording.credential(),
        Some(options),
    )?;
    let endpoint: Url = recording
        .var("KEYVAULT_RESOURCE_MANAGER_URL", None)
        .parse()?;
    assert_eq!(client.endpoint(), &endpoint);
    let vaults = client.get_key_vault_vaults_client();
    let resource_group = recording.var("KEYVAULT_RESOURCE_GROUP", None);
    let location = recording.var("KEYVAULT_LOCATION", None);
    let tenant_id = recording.var("KEYVAULT_TENANT_ID", None);
    let vault_name = recording.random_string::<16>(Some("v"));
    let missing_vault_name = recording.random_string::<16>(Some("missing"));

    include_markdown!("README.md", "create_vault", scope);
    include_markdown!("README.md", "list_vaults", scope);
    include_markdown!("README.md", "update_vault", scope);
    include_markdown!("README.md", "errors", scope);
    include_markdown!("README.md", "delete_vault", scope);
    Ok(())
}

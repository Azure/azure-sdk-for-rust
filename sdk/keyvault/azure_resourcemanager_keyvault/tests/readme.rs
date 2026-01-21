// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::error::Result;
use azure_core_test::{recorded, TestContext};
use azure_resourcemanager_keyvault::KeyVaultClient;
use include_file::include_markdown;

#[recorded::test]
async fn readme(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = azure_resourcemanager_keyvault::KeyVaultClientOptions::default();
    recording.instrument(&mut options.client_options);

    let subscription_id = recording.var("AZURE_SUBSCRIPTION_ID", None);
    let client = KeyVaultClient::new(
        "https://management.azure.com",
        recording.credential(),
        subscription_id,
        Some(options),
    )?;

    // Define variables used in README examples
    let resource_group = recording.var("AZURE_RESOURCE_GROUP", None);
    let vault_name = recording.var("AZURE_KEYVAULT_NAME", None);
    let tenant_id = recording.var("AZURE_TENANT_ID", None);

    // Each macro invocation is in its own block to prevent errors with duplicate imports.
    println!("Create a vault");
    include_markdown!("README.md", "create_vault", scope);

    println!("List vaults");
    include_markdown!("README.md", "list_vaults", scope);

    println!("Update a vault");
    include_markdown!("README.md", "update_vault", scope);

    println!("Handle errors");
    include_markdown!("README.md", "errors", scope);

    println!("Delete a vault");
    include_markdown!("README.md", "delete_vault", scope);

    Ok(())
}

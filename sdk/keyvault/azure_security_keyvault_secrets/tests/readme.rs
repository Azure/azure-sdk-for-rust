// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{error::Result, http::StatusCode};
use azure_core_test::{recorded, TestContext, TestMode};
use azure_security_keyvault_secrets::{SecretClient, SecretClientOptions};
use include_file::include_markdown;

#[recorded::test]
async fn readme(ctx: TestContext) -> Result<()> {
    use azure_security_keyvault_test::Retry;

    let recording = ctx.recording();

    let mut options = SecretClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = SecretClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Each macro invocation is in its own block to prevent errors with duplicate imports.
    println!("Create a secret");
    include_markdown!("README.md", "create_secret", scope);

    println!("Get a secret");
    include_markdown!("README.md", "get_secret", scope);

    println!("Update a secret");
    include_markdown!("README.md", "update_secret", scope);

    println!("List secrets");
    include_markdown!("README.md", "list_secrets", scope);

    println!("Handle errors");
    include_markdown!("README.md", "errors", scope);

    println!("Delete a secret");
    include_markdown!("README.md", "delete_secret", scope);

    println!("Purge a secret");
    // Because deletes may not happen right away, try purging in a loop.
    let mut retry = match recording.test_mode() {
        TestMode::Playback => Retry::immediate(),
        _ => Retry::progressive(None),
    };

    loop {
        match client.purge_deleted_secret("secret-name", None).await {
            Ok(_) => break,
            Err(err) if matches!(err.http_status(), Some(StatusCode::Conflict)) => {
                if retry.next().await.is_none() {
                    return Err(err);
                }
            }
            Err(err) => return Err(err),
        }
    }

    Ok(())
}

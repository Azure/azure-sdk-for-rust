// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    error::{ErrorKind, Result},
    http::StatusCode,
    time::Duration,
};
use azure_core_test::{recorded, TestContext};
use azure_security_keyvault_secrets::SecretClient;
use include_file::include_markdown;

#[recorded::test]
async fn readme(ctx: TestContext) -> Result<()> {
    use azure_security_keyvault_secrets::SecretClientOptions;

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

    // Make sure the secret gets purged (may not take immediate effect).
    println!("Purge a secret");
    for _ in 0..5 {
        match client.purge_deleted_secret("secret-name", None).await {
            Ok(_) => break,
            Err(err) if matches!(err.kind(), ErrorKind::HttpResponse { status, .. } if *status == StatusCode::Conflict) =>
            {
                azure_core::sleep(Duration::seconds(1)).await;
            }
            Err(err) => return Err(err),
        }
    }

    Ok(())
}

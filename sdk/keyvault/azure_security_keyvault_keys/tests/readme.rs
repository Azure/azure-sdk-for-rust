// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    error::{ErrorKind, Result},
    http::StatusCode,
    time::Duration,
};
use azure_core_test::{recorded, TestContext, TestMode};
use azure_security_keyvault_keys::KeyClient;
use include_file::include_markdown;

#[recorded::test]
async fn readme(ctx: TestContext) -> Result<()> {
    use azure_security_keyvault_keys::KeyClientOptions;

    let recording = ctx.recording();

    let mut options = KeyClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = KeyClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Each macro invocation is in its own block to prevent errors with duplicate imports.
    println!("Create a key");
    include_markdown!("README.md", "create_key", scope);

    println!("Get a key");
    include_markdown!("README.md", "get_key", scope);

    println!("Update a key");
    include_markdown!("README.md", "update_key", scope);

    println!("List keys");
    include_markdown!("README.md", "list_keys", scope);

    println!("Encrypt and decrypt");
    include_markdown!("README.md", "encrypt_decrypt", scope);

    println!("Handle errors");
    include_markdown!("README.md", "errors", scope);

    println!("Delete a key");
    include_markdown!("README.md", "delete_key", scope);

    // Make sure the key gets purged (may not take immediate effect).
    println!("Purge a key");
    for _ in 0..5 {
        match client.purge_deleted_key("key-name", None).await {
            Ok(_) => break,
            Err(err) if matches!(err.kind(), ErrorKind::HttpResponse { status, .. } if *status == StatusCode::Conflict) => {
                if recording.test_mode() != TestMode::Playback {
                    azure_core::sleep(Duration::seconds(1)).await;
                }
            }
            Err(err) => return Err(err),
        }
    }

    Ok(())
}

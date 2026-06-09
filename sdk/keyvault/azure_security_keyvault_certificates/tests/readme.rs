// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{error::Result, http::StatusCode};
use azure_core_test::{recorded, TestContext, TestMode};
use azure_security_keyvault_certificates::{CertificateClient, CertificateClientOptions};
use azure_security_keyvault_keys::{KeyClient, KeyClientOptions};
use azure_security_keyvault_test::Retry;
use include_file::include_markdown;

#[recorded::test]
async fn readme(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = CertificateClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = CertificateClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    let mut key_options = KeyClientOptions::default();
    recording.instrument(&mut key_options.client_options);

    let key_client = KeyClient::new(
        client.endpoint().as_str(),
        recording.credential(),
        Some(key_options),
    )?;

    // Each macro invocation is in its own block to prevent errors with duplicate imports.
    println!("Create a certificate");
    include_markdown!("README.md", "create_certificate", scope);

    println!("Get a certificate");
    include_markdown!("README.md", "get_certificate", scope);

    println!("Update a certificate");
    include_markdown!("README.md", "update_certificate", scope);

    println!("List certificates");
    include_markdown!("README.md", "list_certificates", scope);

    println!("Key operations using certificates");
    include_markdown!("README.md", "key_operations", scope);

    println!("Handle errors");
    include_markdown!("README.md", "errors", scope);

    println!("Delete a certificate");
    include_markdown!("README.md", "delete_certificate", scope);

    println!("Purge a certificate");
    // Because deletes may not happen right away, try purging in a loop.
    let mut retry = match recording.test_mode() {
        TestMode::Playback => Retry::immediate(),
        _ => Retry::progressive(None),
    };

    loop {
        match client
            .purge_deleted_certificate("certificate-name", None)
            .await
        {
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

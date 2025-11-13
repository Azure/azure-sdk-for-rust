// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    error::{ErrorKind, Result},
    http::StatusCode,
    time::Duration,
};
use azure_core_test::{recorded, TestContext, TestMode};
use azure_security_keyvault_certificates::CertificateClient;
use include_file::include_markdown;

#[recorded::test]
async fn readme(ctx: TestContext) -> Result<()> {
    use azure_security_keyvault_certificates::CertificateClientOptions;

    let recording = ctx.recording();

    let mut options = CertificateClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = CertificateClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
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

    // Make sure the certificate gets purged (may not take immediate effect).
    println!("Purge a certificate");
    for _ in 0..5 {
        match client
            .purge_deleted_certificate("certificate-name", None)
            .await
        {
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

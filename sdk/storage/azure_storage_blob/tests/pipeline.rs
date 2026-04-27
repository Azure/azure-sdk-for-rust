// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{
    headers::{CLIENT_REQUEST_ID, VERSION},
    RequestContent,
};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    models::BlobClientGetPropertiesResultHeaders, BlobContainerClientOptions,
};
use azure_storage_blob_test::{
    create_test_blob, get_blob_name, get_container_client, ClientOptionsExt, StorageAccount,
    TestPolicy,
};
use std::error::Error;
use std::sync::Arc;

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_storage_headers_present(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Arrange: capture outgoing request headers via a per-call policy
    let check_policy = Arc::new(TestPolicy::new(
        Some(Arc::new(|request: &azure_core::http::Request| {
            let headers = request.headers();
            assert!(
                headers.get_optional_str(&VERSION).is_some(),
                "x-ms-version header must be present on outgoing requests"
            );
            assert!(
                headers.get_optional_str(&CLIENT_REQUEST_ID).is_some(),
                "x-ms-client-request-id header must be present on outgoing requests"
            );
            Ok(())
        })),
        None,
    ));

    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(
        recording,
        true,
        StorageAccount::Standard,
        Some(BlobContainerClientOptions::default().with_per_call_policy(check_policy.clone())),
    )
    .await?;

    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Act - the check_policy fires and asserts on every request
    let _scope = check_policy.check_request_scope();
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(b"pipeline header test".to_vec())),
        None,
    )
    .await?;

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_version_header_matches_options(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let api_version = "2024-11-04";

    // Capture what x-ms-version is sent
    let check_policy = Arc::new(TestPolicy::new(
        Some(Arc::new(move |request: &azure_core::http::Request| {
            let sent_version = request
                .headers()
                .get_optional_str(&VERSION)
                .unwrap_or_default()
                .to_string();
            assert_eq!(
                api_version, sent_version,
                "x-ms-version header should match configured api version"
            );
            Ok(())
        })),
        None,
    ));

    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(
        recording,
        true,
        StorageAccount::Standard,
        Some(
            BlobContainerClientOptions {
                version: api_version.to_string(),
                ..Default::default()
            }
            .with_per_call_policy(check_policy.clone()),
        ),
    )
    .await?;

    let blob_client = container_client.blob_client(&get_blob_name(recording));

    let _scope = check_policy.check_request_scope();
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(b"version header test".to_vec())),
        None,
    )
    .await?;

    // Assert
    let props = blob_client.get_properties(None).await?;
    assert_eq!(Some(19), props.content_length()?);

    container_client.delete(None).await?;
    Ok(())
}

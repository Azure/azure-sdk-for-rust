// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{
        ClientOptions, ExponentialRetryOptions, FixedRetryOptions, RequestContent, RetryOptions,
    },
    time::Duration,
};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    models::{BlobClientDownloadOptions, BlobClientGetPropertiesResultHeaders},
    BlobContainerClient, BlobContainerClientOptions,
};
use azure_storage_blob_test::{
    create_test_blob, get_blob_name, get_container_client, ClientOptionsExt, FailFirstPolicy,
    StorageAccount, TestPolicy,
};
use std::{
    error::Error,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

#[recorded::test]
async fn test_ranged_download(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let data = b"hello rusty world";
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(data.to_vec())),
        None,
    )
    .await?;

    // Bounded Range Download (first 5 bytes: "hello")
    let response = blob_client
        .download(Some(BlobClientDownloadOptions {
            range: Some((0..5usize).into()),
            ..Default::default()
        }))
        .await?;
    assert_eq!(5, response.properties.content_length.unwrap());
    let body = response.body.collect().await?;
    assert_eq!(b"hello".to_vec(), body.to_vec());

    // Bounded Range Download (middle 6 bytes: " rusty")
    let response = blob_client
        .download(Some(BlobClientDownloadOptions {
            range: Some((5..11usize).into()),
            ..Default::default()
        }))
        .await?;
    assert_eq!(6, response.properties.content_length.unwrap());
    let body = response.body.collect().await?;
    assert_eq!(b" rusty".to_vec(), body.to_vec());

    // Bounded Range Download (last 6 bytes: " world")
    let response = blob_client
        .download(Some(BlobClientDownloadOptions {
            range: Some((11..17usize).into()),
            ..Default::default()
        }))
        .await?;
    assert_eq!(6, response.properties.content_length.unwrap());
    let body = response.body.collect().await?;
    assert_eq!(b" world".to_vec(), body.to_vec());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_per_call_policy(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let request_count = Arc::new(AtomicUsize::new(0));
    let count_policy = Arc::new(TestPolicy::count_requests(request_count.clone(), None));
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(
        recording,
        true,
        StorageAccount::Standard,
        Some(BlobContainerClientOptions::default().with_per_call_policy(count_policy.clone())),
    )
    .await?;

    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Per-Call Policy Scenario
    let _scope = count_policy.check_request_scope();
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(b"options test".to_vec())),
        None,
    )
    .await?;

    // Assert
    assert!(
        request_count.load(Ordering::Relaxed) >= 1,
        "per-call policy should have been invoked"
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_per_try_policy(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let request_count = Arc::new(AtomicUsize::new(0));
    let count_policy = Arc::new(TestPolicy::count_requests(request_count.clone(), None));

    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(
        recording,
        true,
        StorageAccount::Standard,
        Some(BlobContainerClientOptions::default().with_per_try_policy(count_policy.clone())),
    )
    .await?;

    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Per-Try Policy Scenario
    let _scope = count_policy.check_request_scope();
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(b"per-try policy test".to_vec())),
        None,
    )
    .await?;

    // Assert
    assert!(
        request_count.load(Ordering::Relaxed) >= 1,
        "per-try policy should have been invoked"
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_retry_options_none(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let per_try_count = Arc::new(AtomicUsize::new(0));
    let count_policy = Arc::new(TestPolicy::count_requests(per_try_count.clone(), None));

    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(
        recording,
        true,
        StorageAccount::Standard,
        Some(
            BlobContainerClientOptions {
                client_options: ClientOptions {
                    retry: RetryOptions::none(),
                    ..Default::default()
                },
                ..Default::default()
            }
            .with_per_try_policy(count_policy.clone()),
        ),
    )
    .await?;

    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Retry Options None Scenario
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(b"retry none test".to_vec())),
        None,
    )
    .await?;

    // Assert: with RetryOptions::none(), each request is attempted exactly once.
    // Verify by counting per-try invocations for a single known request.
    let count_before = per_try_count.load(Ordering::Relaxed);
    // `TestPolicy::send` only invokes its callback when `request_scope_counter > 0`.
    // Without activating the scope here, the counter never increments and the assertion fails.
    let _count_scope = count_policy.check_request_scope();
    let props = blob_client.get_properties(None).await?;
    assert_eq!(
        per_try_count.load(Ordering::Relaxed) - count_before,
        1,
        "expected exactly 1 per-try invocation (no retries)"
    );
    drop(_count_scope);
    assert_eq!(Some(15), props.content_length()?);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_retry_fires_on_transient_error(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let call_count = Arc::new(AtomicUsize::new(0));
    // Fail one time, then succeed - requires at least 1 retry
    let fail_policy = Arc::new(FailFirstPolicy::new(1, call_count.clone()));

    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(
        recording,
        true,
        StorageAccount::Standard,
        Some(
            BlobContainerClientOptions {
                client_options: ClientOptions {
                    retry: RetryOptions::exponential(ExponentialRetryOptions {
                        max_retries: 3,
                        initial_delay: Duration::milliseconds(0),
                        max_delay: Duration::milliseconds(0),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }
            .with_per_try_policy(fail_policy.clone()),
        ),
    )
    .await?;

    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Act
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(b"retry transient test".to_vec())),
        None,
    )
    .await?;

    // Assert
    assert!(
        call_count.load(Ordering::SeqCst) >= 2,
        "expected at least 2 invocations (1 failure + retry), got {}",
        call_count.load(Ordering::SeqCst)
    );

    container_client.delete(None).await?;
    Ok(())
}

// `#[tokio::test]` is intentional here: FailFirstPolicy exhausts all retry attempts
// before ever forwarding to the real transport, so there is nothing to record or replay.
// The test is pure in-process logic and does not need the recording harness.
#[tokio::test]
async fn test_retry_exhaustion() -> Result<(), Box<dyn std::error::Error>> {
    // max_retries=2 means 1 original attempt + 2 retry attempts = 3 total invocations
    let max_retries = 2u32;
    let call_count = Arc::new(AtomicUsize::new(0));
    // fail_count > max_retries+1 so every attempt fails
    let fail_policy = Arc::new(FailFirstPolicy::new(10, call_count.clone()));

    let container_client = BlobContainerClient::new(
        "https://fake.blob.core.windows.net/",
        "fakecontainer",
        None,
        Some(
            BlobContainerClientOptions {
                client_options: ClientOptions {
                    retry: RetryOptions::fixed(FixedRetryOptions {
                        max_retries,
                        delay: Duration::milliseconds(0),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }
            .with_per_try_policy(fail_policy.clone()),
        ),
    )?;

    // Act - should fail after all retry attempts are exhausted
    let result = container_client.get_properties(None).await;
    assert!(result.is_err(), "expected exhausted retries to return Err");

    // The policy should have been invoked max_retries+1 times
    let invocations = call_count.load(Ordering::SeqCst);
    assert_eq!(
        invocations,
        (max_retries + 1) as usize,
        "expected {} total invocations (1 original + {} retries), got {}",
        max_retries + 1,
        max_retries,
        invocations
    );

    Ok(())
}

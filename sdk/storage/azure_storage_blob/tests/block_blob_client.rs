// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{RequestContent, StatusCode},
    Bytes,
};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    models::{
        method_options::BlockBlobClientManagedUploadOptions, BlobClientDownloadResultHeaders,
        BlockBlobClientUploadBlobFromUrlOptions, BlockListType, BlockLookupList,
    },
    BlobContainerClientOptions,
};
use azure_storage_blob_test::{
    create_test_blob, get_blob_name, get_container_client, predicates, ClientOptionsExt,
    StorageAccount, TestPolicy, KB, MB,
};
use bytes::{BufMut, BytesMut};
use std::{
    error::Error,
    io::Write,
    num::NonZero,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

#[recorded::test]
async fn test_block_list(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let block_blob_client = blob_client.block_blob_client();

    let block_1 = b"AAA";
    let block_2 = b"BBB";
    let block_3 = b"CCC";

    let block_1_id: Vec<u8> = b"1".to_vec();
    let block_2_id: Vec<u8> = b"2".to_vec();
    let block_3_id: Vec<u8> = b"3".to_vec();

    block_blob_client
        .stage_block(
            &block_1_id,
            u64::try_from(block_1.len())?,
            RequestContent::from(block_1.to_vec()),
            None,
        )
        .await?;

    block_blob_client
        .stage_block(
            &block_2_id,
            u64::try_from(block_2.len())?,
            RequestContent::from(block_2.to_vec()),
            None,
        )
        .await?;
    block_blob_client
        .stage_block(
            &block_3_id,
            u64::try_from(block_3.len())?,
            RequestContent::from(block_3.to_vec()),
            None,
        )
        .await?;

    // Three Staged Blocks Scenario
    let block_list = block_blob_client
        .get_block_list(BlockListType::All, None)
        .await?
        .into_model()?;

    // Assert
    assert!(block_list.committed_blocks.is_none());
    assert_eq!(
        3,
        block_list
            .uncommitted_blocks
            .expect("expected uncommitted_blocks")
            .len()
    );

    let latest_blocks: Vec<Vec<u8>> = vec![block_1_id, block_2_id, block_3_id];

    let block_lookup_list = BlockLookupList {
        committed: Some(Vec::new()),
        latest: Some(latest_blocks),
        uncommitted: Some(Vec::new()),
    };

    block_blob_client
        .commit_block_list(block_lookup_list.try_into()?, None)
        .await?;

    // Three Committed Blocks Scenario
    let block_list = block_blob_client
        .get_block_list(BlockListType::All, None)
        .await?
        .into_model()?;
    let response = blob_client.download(None).await?;

    // Assert
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(9, content_length.unwrap());
    assert_eq!(
        Bytes::from_static(b"AAABBBCCC"),
        response_body.collect().await?.as_ref(),
    );
    assert_eq!(
        3,
        block_list
            .committed_blocks
            .expect("expected committed_blocks")
            .len()
    );
    assert!(block_list.uncommitted_blocks.is_none());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_upload_blob_from_url(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let source_blob_client = container_client.blob_client(&get_blob_name(recording));
    create_test_blob(
        &source_blob_client,
        Some(RequestContent::from(b"initialD ata".to_vec())),
        None,
    )
    .await?;

    let blob_client = container_client.blob_client(&get_blob_name(recording));

    let overwrite_blob_client = container_client.blob_client(&get_blob_name(recording));
    create_test_blob(
        &overwrite_blob_client,
        Some(RequestContent::from(b"overruled!".to_vec())),
        None,
    )
    .await?;

    // Regular Scenario
    blob_client
        .block_blob_client()
        .upload_blob_from_url(source_blob_client.url().as_str().into(), None)
        .await?;

    let create_options = BlockBlobClientUploadBlobFromUrlOptions::default().with_if_not_exists();

    // No Overwrite Existing Blob Scenario
    let response = blob_client
        .block_blob_client()
        .upload_blob_from_url(
            overwrite_blob_client.url().as_str().into(),
            Some(create_options),
        )
        .await;
    // Assert
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::Conflict, error.unwrap());

    // Overwrite Existing Blob Scenario
    blob_client
        .block_blob_client()
        .upload_blob_from_url(overwrite_blob_client.url().as_str().into(), None)
        .await?;

    // Public Resource Scenario
    blob_client
        .block_blob_client()
        .upload_blob_from_url(
            "https://www.gutenberg.org/cache/epub/1533/pg1533.txt".into(),
            None,
        )
        .await?;

    // Source Authorization Scenario
    let access_token = format!(
        "Bearer {}",
        recording
            .credential()
            .get_token(&["https://storage.azure.com/.default"], None)
            .await?
            .token
            .secret()
    );

    let source_auth_options = BlockBlobClientUploadBlobFromUrlOptions {
        copy_source_authorization: Some(access_token),
        ..Default::default()
    };

    blob_client
        .block_blob_client()
        .upload_blob_from_url(
            overwrite_blob_client.url().as_str().into(),
            Some(source_auth_options),
        )
        .await?;

    container_client.delete_container(None).await?;
    Ok(())
}

// TODO successfully record block ID generation to remove live-only marker
#[recorded::test(live)]
async fn managed_upload(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let stage_block_count = Arc::new(AtomicUsize::new(0));
    let count_policy = Arc::new(TestPolicy::count_requests(
        stage_block_count.clone(),
        Some(Arc::new(predicates::is_stage_block_request)),
    ));

    let recording = ctx.recording();
    let container_client = get_container_client(
        recording,
        true,
        StorageAccount::Standard,
        Some(BlobContainerClientOptions::default().with_per_call_policy(count_policy.clone())),
    )
    .await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let block_blob_client = blob_client.block_blob_client();

    let data: [u8; 1024] = recording.random();
    let bytes: Bytes = data.to_vec().into();

    for (parallel, partition_size, expected_stage_block_calls) in [
        (1, 2048, 0), // put blob expected
        (2, 1024, 0), // put blob expected
        (2, 512, 2),
        (1, 256, 4),
        (8, 31, 34),
    ] {
        stage_block_count.store(0, Ordering::Relaxed);
        let options = BlockBlobClientManagedUploadOptions {
            parallel: Some(NonZero::new(parallel).unwrap()),
            partition_size: Some(NonZero::new(partition_size).unwrap()),
            ..Default::default()
        };
        {
            let _scope = count_policy.check_request_scope();
            block_blob_client
                .managed_upload(bytes.clone().into(), Some(options))
                .await?;
        }
        assert_eq!(
            blob_client
                .download(None)
                .await?
                .into_body()
                .collect()
                .await?[..],
            data,
            "Failed parallel={},partition_size={}",
            parallel,
            partition_size
        );
        assert_eq!(
            stage_block_count.load(Ordering::Relaxed),
            expected_stage_block_calls,
            "Failed parallel={},partition_size={}",
            parallel,
            partition_size
        );
    }

    Ok(())
}

#[recorded::test]
async fn managed_upload_empty(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let request_count = Arc::new(AtomicUsize::new(0));
    let count_policy = Arc::new(TestPolicy::count_requests(request_count.clone(), None));

    let recording = ctx.recording();
    let container_client = get_container_client(
        recording,
        true,
        StorageAccount::Standard,
        Some(BlobContainerClientOptions::default().with_per_call_policy(count_policy.clone())),
    )
    .await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let block_blob_client = blob_client.block_blob_client();

    let data = [];
    let bytes: Bytes = data.to_vec().into();

    request_count.store(0, Ordering::Relaxed);
    let options = BlockBlobClientManagedUploadOptions {
        ..Default::default()
    };
    {
        let _scope = count_policy.check_request_scope();
        block_blob_client
            .managed_upload(bytes.clone().into(), Some(options))
            .await?;
    }
    assert_eq!(
        blob_client
            .download(None)
            .await?
            .into_body()
            .collect()
            .await?[..],
        data
    );
    assert_eq!(request_count.load(Ordering::Relaxed), 1);

    Ok(())
}

#[recorded::test(live)]
async fn managed_upload_large(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let stage_block_count = Arc::new(AtomicUsize::new(0));
    let count_policy = Arc::new(TestPolicy::count_requests(
        stage_block_count.clone(),
        Some(Arc::new(predicates::is_stage_block_request)),
    ));

    let recording = ctx.recording();
    let container_client = get_container_client(
        recording,
        true,
        StorageAccount::Standard,
        Some(BlobContainerClientOptions::default().with_per_call_policy(count_policy.clone())),
    )
    .await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let block_blob_client = blob_client.block_blob_client();

    let data_len = 50 * MB;
    let expected_stage_block_count = data_len.div_ceil(4 * MB);
    let mut bytes = BytesMut::with_capacity(data_len).writer();
    {
        let mut buf = [0u8; 4 * KB];
        for _ in (0..data_len).step_by(buf.len()) {
            buf = recording.random();
            bytes.write_all(&buf)?;
        }
    }
    let bytes = bytes.into_inner().freeze();

    stage_block_count.store(0, Ordering::Relaxed);
    {
        let _scope = count_policy.check_request_scope();
        block_blob_client
            .managed_upload(bytes.clone().into(), None)
            .await?;
    }
    assert_eq!(
        blob_client
            .download(None)
            .await?
            .into_body()
            .collect()
            .await?[..],
        bytes[..]
    );
    assert_eq!(
        stage_block_count.load(Ordering::Relaxed),
        expected_stage_block_count
    );

    Ok(())
}

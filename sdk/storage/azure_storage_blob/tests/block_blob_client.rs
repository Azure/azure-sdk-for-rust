// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{headers::CONTENT_TYPE, RequestContent, StatusCode},
    Bytes,
};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    models::{
        BlobClientGetPropertiesResultHeaders, BlockBlobClientCommitBlockListOptions,
        BlockBlobClientStageBlockFromUrlOptions, BlockBlobClientStageBlockOptions,
        BlockBlobClientUploadBlobFromUrlOptions, BlockBlobClientUploadOptions, BlockListType,
        BlockLookupList,
    },
    BlobContainerClientOptions,
};
use azure_storage_blob_test::{
    block_lookup, create_test_blob, get_blob_name, get_container_client, predicates,
    ClientOptionsExt, StorageAccount, TestPolicy, KB, MB,
};
use bytes::{BufMut, BytesMut};
use std::{
    collections::HashMap,
    error::Error,
    io::Write,
    num::NonZero,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
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
    assert_eq!(9, response.properties.content_length.unwrap());
    let body_data = response.body.collect().await?;
    assert_eq!(Bytes::from_static(b"AAABBBCCC"), &body_data[..],);
    assert_eq!(
        3,
        block_list
            .committed_blocks
            .expect("expected committed_blocks")
            .len()
    );
    assert!(block_list.uncommitted_blocks.is_none());

    container_client.delete(None).await?;
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

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_stage_block_from_url(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let source_blob_client = container_client.blob_client(&get_blob_name(recording));
    let source_content = b"Hello from source blob!";
    create_test_blob(
        &source_blob_client,
        Some(RequestContent::from(source_content.to_vec())),
        None,
    )
    .await?;

    let dest_blob_client = container_client.blob_client(&get_blob_name(recording));
    let block_blob_client = dest_blob_client.block_blob_client();
    let block_id: Vec<u8> = b"block1".to_vec();

    // Regular Scenario
    block_blob_client
        .stage_block_from_url(
            &block_id,
            u64::try_from(source_content.len())?,
            source_blob_client.url().as_str().into(),
            None,
        )
        .await?;

    // Staged Block Scenario
    let block_list = block_blob_client
        .get_block_list(BlockListType::All, None)
        .await?
        .into_model()?;

    // Assert
    assert!(block_list.committed_blocks.is_none());
    assert_eq!(
        1,
        block_list
            .uncommitted_blocks
            .expect("expected uncommitted_blocks")
            .len()
    );

    let block_lookup_list = BlockLookupList {
        committed: Some(Vec::new()),
        latest: Some(vec![block_id]),
        uncommitted: Some(Vec::new()),
    };

    block_blob_client
        .commit_block_list(block_lookup_list.try_into()?, None)
        .await?;

    // Committed Block Scenario
    let response = dest_blob_client.download(None).await?;
    // Assert
    assert_eq!(
        source_content.len(),
        response.properties.content_length.unwrap() as usize
    );
    let body_data = response.body.collect().await?;
    assert_eq!(Bytes::from_static(source_content), &body_data[..],);

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

    let source_auth_options = BlockBlobClientStageBlockFromUrlOptions {
        copy_source_authorization: Some(access_token),
        ..Default::default()
    };

    let block_id_2: Vec<u8> = b"block2".to_vec();
    let source_blob_client_2 = container_client.blob_client(&get_blob_name(recording));
    let source_content_2 = b"Authorized content!";
    create_test_blob(
        &source_blob_client_2,
        Some(RequestContent::from(source_content_2.to_vec())),
        None,
    )
    .await?;

    block_blob_client
        .stage_block_from_url(
            &block_id_2,
            u64::try_from(source_content_2.len())?,
            source_blob_client_2.url().as_str().into(),
            Some(source_auth_options),
        )
        .await?;

    let block_lookup_list_2 = BlockLookupList {
        committed: Some(Vec::new()),
        latest: Some(vec![block_id_2]),
        uncommitted: Some(Vec::new()),
    };

    block_blob_client
        .commit_block_list(block_lookup_list_2.try_into()?, None)
        .await?;

    let response = dest_blob_client.download(None).await?;
    // Assert
    let body_data = response.body.collect().await?;
    assert_eq!(Bytes::from_static(source_content_2), &body_data[..],);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test(live)]
async fn upload(ctx: TestContext) -> Result<(), Box<dyn Error>> {
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
        let options = BlockBlobClientUploadOptions {
            parallel: Some(NonZero::new(parallel).unwrap()),
            partition_size: Some(NonZero::new(partition_size).unwrap()),
            ..Default::default()
        };
        {
            let _scope = count_policy.check_request_scope();
            block_blob_client
                .upload(bytes.clone().into(), Some(options))
                .await?;
        }
        let body_data = blob_client.download(None).await?.body.collect().await?;
        assert_eq!(
            body_data[..],
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
async fn upload_empty(ctx: TestContext) -> Result<(), Box<dyn Error>> {
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
    let options = BlockBlobClientUploadOptions {
        ..Default::default()
    };
    {
        let _scope = count_policy.check_request_scope();
        block_blob_client
            .upload(bytes.clone().into(), Some(options))
            .await?;
    }
    let body_data = blob_client.download(None).await?.body.collect().await?;
    assert_eq!(body_data[..], data);
    assert_eq!(request_count.load(Ordering::Relaxed), 1);

    Ok(())
}

#[recorded::test]
#[ignore = "Temporarily ignoring until we can figure out how to get this to not take down the whole test pipeline."]
async fn upload_large(ctx: TestContext) -> Result<(), Box<dyn Error>> {
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
        block_blob_client.upload(bytes.clone().into(), None).await?;
    }
    let body_data = blob_client.download(None).await?.body.collect().await?;
    assert_eq!(body_data[..], bytes[..]);
    assert_eq!(
        stage_block_count.load(Ordering::Relaxed),
        expected_stage_block_count
    );

    Ok(())
}

#[recorded::test]
async fn test_commit_block_list_content_headers(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let block_blob_client = blob_client.block_blob_client();
    let block_id = b"block-1".to_vec();
    let content = b"commit-block-list-content-headers";
    let md5: Vec<u8> = (0u8..16).collect();

    // Stage Block
    block_blob_client
        .stage_block(
            &block_id,
            u64::try_from(content.len())?,
            RequestContent::from(content.to_vec()),
            None,
        )
        .await?;

    // Commit Block List with Content Headers
    // Note: blob_content_md5 on commit_block_list is stored metadata (not validated against
    // actual content), so an arbitrary value can be used to verify roundtrip behavior.
    block_blob_client
        .commit_block_list(
            block_lookup(block_id).try_into()?,
            Some(BlockBlobClientCommitBlockListOptions {
                blob_cache_control: Some("max-age=600".to_string()),
                blob_content_disposition: Some("inline".to_string()),
                blob_content_encoding: Some("identity".to_string()),
                blob_content_language: Some("de-DE".to_string()),
                blob_content_md5: Some(md5.clone()),
                blob_content_type: Some("application/json".to_string()),
                ..Default::default()
            }),
        )
        .await?;

    // Assert Content Headers Roundtrip
    let props = blob_client.get_properties(None).await?;
    assert_eq!(Some("max-age=600".to_string()), props.cache_control()?);
    assert_eq!(Some("inline".to_string()), props.content_disposition()?);
    assert_eq!(Some("identity".to_string()), props.content_encoding()?);
    assert_eq!(Some("de-DE".to_string()), props.content_language()?);
    assert_eq!(Some(md5), props.content_md5()?);
    let content_type: Option<String> = props.headers().get_optional_as(&CONTENT_TYPE)?;
    assert_eq!(Some("application/json".to_string()), content_type);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_block_list_types(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let block_blob_client = blob_client.block_blob_client();

    let block_id: Vec<u8> = b"a".to_vec();
    block_blob_client
        .stage_block(&block_id, 1, RequestContent::from(b"X".to_vec()), None)
        .await?;

    // Uncommitted Blocks Only
    let block_list = block_blob_client
        .get_block_list(BlockListType::Uncommitted, None)
        .await?
        .into_model()?;
    assert!(block_list.committed_blocks.is_none());
    assert!(block_list
        .uncommitted_blocks
        .as_ref()
        .map(|v| v.len() == 1)
        .unwrap_or(false));

    // Commit the block
    block_blob_client
        .commit_block_list(block_lookup(block_id).try_into()?, None)
        .await?;

    // Committed Blocks Only
    let block_list = block_blob_client
        .get_block_list(BlockListType::Committed, None)
        .await?
        .into_model()?;
    assert!(block_list
        .committed_blocks
        .as_ref()
        .map(|v| v.len() == 1)
        .unwrap_or(false));
    assert!(block_list.uncommitted_blocks.is_none());

    // All Blocks
    let block_list = block_blob_client
        .get_block_list(BlockListType::All, None)
        .await?
        .into_model()?;
    assert!(block_list
        .committed_blocks
        .as_ref()
        .map(|v| v.len() == 1)
        .unwrap_or(false));

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_stage_block_transactional_checksums(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let block_blob_client = blob_client.block_blob_client();

    let content = b"hello".to_vec();
    // MD5("hello") - well-known test vector
    let correct_md5: Vec<u8> = vec![
        0x5d, 0x41, 0x40, 0x2a, 0xbc, 0x4b, 0x2a, 0x76, 0xb9, 0x71, 0x9d, 0x91, 0x10, 0x17, 0xc5,
        0x92,
    ];
    // CRC64-ECMA-182 of b"hello", server-confirmed (base64: V0JSBnCFdzM=)
    let correct_crc64: Vec<u8> = vec![87, 66, 82, 6, 112, 133, 119, 51];
    let block_id: Vec<u8> = b"1".to_vec();

    // MD5 Mismatch Scenario
    let response = block_blob_client
        .stage_block(
            &block_id,
            u64::try_from(content.len())?,
            RequestContent::from(content.clone()),
            Some(BlockBlobClientStageBlockOptions {
                transactional_content_md5: Some(vec![0u8; 16]),
                ..Default::default()
            }),
        )
        .await;
    assert_eq!(
        StatusCode::BadRequest,
        response.unwrap_err().http_status().unwrap()
    );

    // MD5 Match Scenario
    block_blob_client
        .stage_block(
            &block_id,
            u64::try_from(content.len())?,
            RequestContent::from(content.clone()),
            Some(BlockBlobClientStageBlockOptions {
                transactional_content_md5: Some(correct_md5),
                ..Default::default()
            }),
        )
        .await?;

    // CRC64 Mismatch Scenario
    let response = block_blob_client
        .stage_block(
            &block_id,
            u64::try_from(content.len())?,
            RequestContent::from(content.clone()),
            Some(BlockBlobClientStageBlockOptions {
                transactional_content_crc64: Some(vec![0u8; 8]),
                ..Default::default()
            }),
        )
        .await;
    assert_eq!(
        StatusCode::BadRequest,
        response.unwrap_err().http_status().unwrap()
    );

    // CRC64 Match Scenario
    block_blob_client
        .stage_block(
            &block_id,
            u64::try_from(content.len())?,
            RequestContent::from(content),
            Some(BlockBlobClientStageBlockOptions {
                transactional_content_crc64: Some(correct_crc64),
                ..Default::default()
            }),
        )
        .await?;

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_commit_block_list_with_tags(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let block_blob_client = blob_client.block_blob_client();

    let block_id: Vec<u8> = b"1".to_vec();
    block_blob_client
        .stage_block(&block_id, 5, RequestContent::from(b"hello".to_vec()), None)
        .await?;
    block_blob_client
        .commit_block_list(
            block_lookup(block_id).try_into()?,
            Some(BlockBlobClientCommitBlockListOptions {
                blob_tags_string: Some("sdk=rust".to_string()),
                ..Default::default()
            }),
        )
        .await?;

    // Assert
    let expected = HashMap::from([("sdk".to_string(), "rust".to_string())]);
    let map: HashMap<String, String> = blob_client.get_tags(None).await?.into_model()?.into();
    assert_eq!(expected, map);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_stage_block_from_url_source_if_match(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let source_blob_client = container_client.blob_client(&get_blob_name(recording));
    let source_data = b"source";
    create_test_blob(
        &source_blob_client,
        Some(RequestContent::from(source_data.to_vec())),
        None,
    )
    .await?;
    let etag = source_blob_client
        .get_properties(None)
        .await?
        .etag()?
        .unwrap()
        .to_string();

    let dest_blob_client = container_client.blob_client(&get_blob_name(recording));
    let block_blob_client = dest_blob_client.block_blob_client();
    let block_id: Vec<u8> = b"b1".to_vec();

    // Source If-Match Scenario
    block_blob_client
        .stage_block_from_url(
            &block_id,
            u64::try_from(source_data.len())?,
            source_blob_client.url().as_str().into(),
            Some(BlockBlobClientStageBlockFromUrlOptions {
                source_if_match: Some(etag.clone().into()),
                ..Default::default()
            }),
        )
        .await?;

    // Source If-None-Match Scenario (ETag matches, so condition is not satisfied)
    let response = block_blob_client
        .stage_block_from_url(
            &block_id,
            u64::try_from(source_data.len())?,
            source_blob_client.url().as_str().into(),
            Some(BlockBlobClientStageBlockFromUrlOptions {
                source_if_none_match: Some(etag.into()),
                ..Default::default()
            }),
        )
        .await;

    // Assert
    assert_eq!(
        StatusCode::NotModified,
        response.unwrap_err().http_status().unwrap()
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_upload_blob_from_url_source_if_match(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let source_blob_client = container_client.blob_client(&get_blob_name(recording));
    create_test_blob(
        &source_blob_client,
        Some(RequestContent::from(b"source data".to_vec())),
        None,
    )
    .await?;
    let etag = source_blob_client
        .get_properties(None)
        .await?
        .etag()?
        .unwrap()
        .to_string();

    let dest_blob_client = container_client.blob_client(&get_blob_name(recording));
    let block_blob_client = dest_blob_client.block_blob_client();

    // Source If-Match Scenario
    block_blob_client
        .upload_blob_from_url(
            source_blob_client.url().as_str().into(),
            Some(BlockBlobClientUploadBlobFromUrlOptions {
                source_if_match: Some(etag.clone().into()),
                ..Default::default()
            }),
        )
        .await?;

    // Source If-None-Match Scenario (ETag matches, so condition is not satisfied)
    let response = block_blob_client
        .upload_blob_from_url(
            source_blob_client.url().as_str().into(),
            Some(BlockBlobClientUploadBlobFromUrlOptions {
                source_if_none_match: Some(etag.into()),
                ..Default::default()
            }),
        )
        .await;

    // Assert
    assert_eq!(
        StatusCode::NotModified,
        response.unwrap_err().http_status().unwrap()
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_upload_block_blob_with_tags(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    let expected = HashMap::from([("version".to_string(), "1".to_string())]);
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(b"tagged blob content".to_vec())),
        Some(BlockBlobClientUploadOptions {
            blob_tags_string: Some("version=1".to_string()),
            ..Default::default()
        }),
    )
    .await?;

    // Assert
    let map: HashMap<String, String> = blob_client.get_tags(None).await?.into_model()?.into();
    assert_eq!(expected, map);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_upload_blob_from_url_source_timestamp_conditions(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let source_blob_client = container_client.blob_client(&get_blob_name(recording));
    create_test_blob(&source_blob_client, None, None).await?;
    let last_modified = source_blob_client
        .get_properties(None)
        .await?
        .last_modified()?
        .unwrap();
    let before = last_modified - Duration::from_secs(60);
    let after = last_modified + Duration::from_secs(60);

    let dest_blob_client = container_client.blob_client(&get_blob_name(recording));
    let block_blob_client = dest_blob_client.block_blob_client();

    // source_if_modified_since=before - Succeeds (source was modified after 'before')
    block_blob_client
        .upload_blob_from_url(
            source_blob_client.url().as_str().into(),
            Some(BlockBlobClientUploadBlobFromUrlOptions {
                source_if_modified_since: Some(before),
                ..Default::default()
            }),
        )
        .await?;

    // source_if_modified_since=after - Not Modified (source was not modified after 'after')
    let err = block_blob_client
        .upload_blob_from_url(
            source_blob_client.url().as_str().into(),
            Some(BlockBlobClientUploadBlobFromUrlOptions {
                source_if_modified_since: Some(after),
                ..Default::default()
            }),
        )
        .await;
    assert_eq!(
        StatusCode::NotModified,
        err.unwrap_err().http_status().unwrap()
    );

    // source_if_unmodified_since=after - Succeeds (source was not modified after 'after')
    block_blob_client
        .upload_blob_from_url(
            source_blob_client.url().as_str().into(),
            Some(BlockBlobClientUploadBlobFromUrlOptions {
                source_if_unmodified_since: Some(after),
                ..Default::default()
            }),
        )
        .await?;

    // source_if_unmodified_since=before - Precondition Failed (source was modified after 'before')
    let err = block_blob_client
        .upload_blob_from_url(
            source_blob_client.url().as_str().into(),
            Some(BlockBlobClientUploadBlobFromUrlOptions {
                source_if_unmodified_since: Some(before),
                ..Default::default()
            }),
        )
        .await;
    assert_eq!(
        StatusCode::PreconditionFailed,
        err.unwrap_err().http_status().unwrap()
    );

    container_client.delete(None).await?;
    Ok(())
}

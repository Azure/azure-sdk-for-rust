// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod common;

use std::sync::{mpsc::TryRecvError, Arc};

use azure_core::{
    base64,
    http::{policies::Policy, Body},
    stream::BytesStream,
    Result,
};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    models::{BlockBlobClientStageBlockOptions, UploadContentValidation},
    BlobContainerClientOptions,
};
use bytes::Bytes;
use crc_fast::{CrcAlgorithm, Digest};

use crate::common::{get_blob_name, get_container_client, StorageAccount, TestPolicy};

#[recorded::test]
async fn test_stage_block_no_options_validates(ctx: TestContext) -> Result<()> {
    let (tx, requests_rx) = std::sync::mpsc::channel();
    let test_policy = Arc::new(TestPolicy::capture(Some(tx)));

    let recording = ctx.recording();
    let client = get_container_client(
        recording,
        true,
        StorageAccount::Standard,
        Some(options_with_policies(vec![test_policy.clone()], vec![])),
    )
    .await?
    .blob_client(&get_blob_name(recording))
    .block_blob_client();

    let content = Bytes::from("Hello, world!");
    let content_crc = crc_inline(&content).to_le_bytes();
    {
        let _test_policy_scope = test_policy.check_request_scope();
        let _result = client
            .stage_block(
                b"block_id",
                content.len() as u64,
                content.clone().into(),
                None,
            )
            .await?;
    }

    let request = requests_rx.recv().unwrap();
    assert!(matches!(requests_rx.try_recv(), Err(TryRecvError::Empty)));
    assert_eq!(
        request.headers().get_str(&"x-ms-content-crc64".into())?,
        base64::encode(&content_crc)
    );

    Ok(())
}

#[recorded::test]
async fn test_stage_block_no_validation_options_validates(ctx: TestContext) -> Result<()> {
    let (tx, requests_rx) = std::sync::mpsc::channel();
    let test_policy = Arc::new(TestPolicy::capture(Some(tx)));

    let recording = ctx.recording();
    let client = get_container_client(
        recording,
        true,
        StorageAccount::Standard,
        Some(options_with_policies(vec![test_policy.clone()], vec![])),
    )
    .await?
    .blob_client(&get_blob_name(recording))
    .block_blob_client();

    let content = Bytes::from("Hello, world!");
    let content_crc = crc_inline(&content).to_le_bytes();
    {
        let _test_policy_scope = test_policy.check_request_scope();
        let _result = client
            .stage_block(
                b"block_id",
                content.len() as u64,
                content.clone().into(),
                Some(BlockBlobClientStageBlockOptions {
                    content_validation: None,
                    ..Default::default()
                }),
            )
            .await?;
    }

    let request = requests_rx.recv().unwrap();
    assert!(matches!(requests_rx.try_recv(), Err(TryRecvError::Empty)));
    assert_eq!(
        request.headers().get_str(&"x-ms-content-crc64".into())?,
        base64::encode(&content_crc)
    );

    Ok(())
}

#[recorded::test]
async fn test_stage_block_disable_validation(ctx: TestContext) -> Result<()> {
    let (tx, requests_rx) = std::sync::mpsc::channel();
    let test_policy = Arc::new(TestPolicy::capture(Some(tx)));

    let recording = ctx.recording();
    let client = get_container_client(
        recording,
        true,
        StorageAccount::Standard,
        Some(options_with_policies(vec![test_policy.clone()], vec![])),
    )
    .await?
    .blob_client(&get_blob_name(recording))
    .block_blob_client();

    let content = Bytes::from("Hello, world!");
    {
        let _test_policy_scope = test_policy.check_request_scope();
        let _result = client
            .stage_block(
                b"block_id",
                content.len() as u64,
                content.clone().into(),
                Some(BlockBlobClientStageBlockOptions {
                    content_validation: Some(UploadContentValidation::None),
                    ..Default::default()
                }),
            )
            .await?;
    }

    let request = requests_rx.recv().unwrap();
    assert!(matches!(requests_rx.try_recv(), Err(TryRecvError::Empty)));
    assert!(request
        .headers()
        .get_str(&"x-ms-content-crc64".into())
        .is_err());
    assert!(request.headers().get_str(&"content-md5".into()).is_err());

    Ok(())
}

#[recorded::test]
async fn test_stage_block_precalculated_crc(ctx: TestContext) -> Result<()> {
    let (tx, requests_rx) = std::sync::mpsc::channel();
    let test_policy = Arc::new(TestPolicy::capture(Some(tx)));

    let recording = ctx.recording();
    let client = get_container_client(
        recording,
        true,
        StorageAccount::Standard,
        Some(options_with_policies(vec![test_policy.clone()], vec![])),
    )
    .await?
    .blob_client(&get_blob_name(recording))
    .block_blob_client();

    let content = Bytes::from("Hello, world!");
    // use a wrong crc to ensure that's what is passed and not an sdk-calculated one
    let precalculated_crc = [0, 1, 2, 3, 4, 5, 6, 7];
    let result;
    {
        let _test_policy_scope = test_policy.check_request_scope();
        result = client
            .stage_block(
                b"block_id",
                content.len() as u64,
                content.clone().into(),
                Some(BlockBlobClientStageBlockOptions {
                    content_validation: Some(UploadContentValidation::Crc64Nvme(Some(
                        precalculated_crc,
                    ))),
                    ..Default::default()
                }),
            )
            .await;
    }

    let request = requests_rx.recv().unwrap();
    assert!(matches!(requests_rx.try_recv(), Err(TryRecvError::Empty)));
    assert_eq!(
        request.headers().get_str(&"x-ms-content-crc64".into())?,
        base64::encode(&precalculated_crc)
    );
    // we used a wrong crc
    assert!(result.is_err());

    Ok(())
}

#[recorded::test]
async fn test_stage_block_streaming(ctx: TestContext) -> Result<()> {
    let (tx, requests_rx) = std::sync::mpsc::channel();
    let test_policy = Arc::new(TestPolicy::capture(Some(tx)));

    let recording = ctx.recording();
    let client = get_container_client(
        recording,
        true,
        StorageAccount::Standard,
        Some(options_with_policies(vec![test_policy.clone()], vec![])),
    )
    .await?
    .blob_client(&get_blob_name(recording))
    .block_blob_client();

    let data = "Hello, world!";
    let content = Body::SeekableStream(Box::new(BytesStream::from(Bytes::from(data))));
    {
        let _test_policy_scope = test_policy.check_request_scope();
        let _result = client
            .stage_block(b"block_id", data.len() as u64, content.into(), None)
            .await?;
    }

    let request = requests_rx.recv().unwrap();
    assert!(matches!(requests_rx.try_recv(), Err(TryRecvError::Empty)));
    assert_eq!(
        request.headers().get_str(&"x-ms-structured-body".into())?,
        "XSM/1.0; properties=crc64"
    );
    assert!(request
        .headers()
        .get_str(&"x-ms-structured-content-length".into())
        .is_ok());

    Ok(())
}

fn crc_inline(content: &[u8]) -> u64 {
    let mut digest = Digest::new(CrcAlgorithm::Crc64Nvme);
    digest.update(content);
    digest.finalize()
}

fn options_with_policies(
    per_call: Vec<Arc<dyn Policy>>,
    per_try: Vec<Arc<dyn Policy>>,
) -> BlobContainerClientOptions {
    BlobContainerClientOptions {
        client_options: azure_core::http::ClientOptions {
            per_call_policies: per_call,
            per_try_policies: per_try,
            ..Default::default()
        },
        ..Default::default()
    }
}

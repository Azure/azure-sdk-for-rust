// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{http::RequestContent, Bytes};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{BlobClientDownloadResultHeaders, BlockListType, BlockLookupList};
use azure_storage_blob_test::{get_blob_name, get_container_client};
use std::error::Error;

#[recorded::test]
async fn test_block_list(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));
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
        .into_body()
        .await?;

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
        .into_body()
        .await?;
    let response = blob_client.download(None).await?;

    // Assert
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(9, content_length.unwrap());
    assert_eq!(
        Bytes::from_static(b"AAABBBCCC"),
        response_body.collect().await?
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

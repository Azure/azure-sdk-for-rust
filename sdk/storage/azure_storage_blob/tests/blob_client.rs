// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{headers::HeaderName, Bytes, RequestContent, StatusCode};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    clients::{BlobClient, BlobContainerClient},
    models::{
        BlobBlock, BlobClientGetPropertiesResultHeaders, BlobType, BlockLookupList, LeaseState,
    },
    BlobClientOptions, BlobContainerClientOptions,
};
use azure_storage_blob_test::recorded_test_setup;
use std::error::Error;

#[recorded::test]
async fn test_get_blob_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let (options, endpoint) = recorded_test_setup(recording, BlobClientOptions::default()).await;
    let container_name = recording
        .random_string::<17>(Some("container"))
        .to_ascii_lowercase();
    let blob_name = recording
        .random_string::<12>(Some("blob"))
        .to_ascii_lowercase();

    // This is to shove the modified pipeline policy into the container client options
    let container_client_options = BlobContainerClientOptions {
        client_options: options.client_options.clone(),
        ..Default::default()
    };
    // Act
    let container_client = BlobContainerClient::new(
        &endpoint,
        container_name.clone(),
        recording.credential(),
        Some(container_client_options),
    )?;
    container_client.create_container(None).await?;

    let blob_client = BlobClient::new(
        &endpoint,
        container_name,
        blob_name,
        recording.credential(),
        Some(options),
    )?;
    let data = b"hello rusty world";

    // This is completely absent from the recording due to the fact that this spins up a BlockBlobClient w/ default options (thus no recording policy)
    // I didn't inherit from the base BlobClient and haven't introduced a knob for these options, only the API-specific options bag
    blob_client
        .upload_blob(
            RequestContent::from(data.to_vec()),
            true,
            u64::try_from(data.len())?,
            None,
        )
        .await?;

    // Where's my Intellisense!!!
    let response = blob_client.get_blob_properties(None).await?;
    let field = response.lease_state()?;
    assert_eq!(field, Some(LeaseState::Available));
    println!("{:?}", response);
    // Assert

    // container_client.delete_container(None).await?;
    Ok(())
}

// #[recorded::test]
// async fn test_get_blob_properties_invalid_container(
//     ctx: TestContext,
// ) -> Result<(), Box<dyn Error>> {
//     // Recording Setup
//     let recording = ctx.recording();
//     let (options, endpoint) = recorded_test_setup(recording, BlobClientOptions::default()).await;
//     let container_name = recording
//         .random_string::<17>(Some("container"))
//         .to_ascii_lowercase();
//     let blob_name = recording
//         .random_string::<12>(Some("blob"))
//         .to_ascii_lowercase();

//     // Act
//     let blob_client = BlobClient::new(
//         &endpoint,
//         container_name,
//         blob_name,
//         recording.credential(),
//         Some(options),
//     )?;
//     let response = blob_client.get_blob_properties(None).await;

//     // Assert
//     assert_eq!(
//         String::from("HttpResponse(NotFound, \"ContainerNotFound\")"),
//         response.unwrap_err().kind().to_string()
//     );

//     Ok(())
// }

// #[recorded::test]
// async fn test_download_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
//     // Recording Setup
//     let recording = ctx.recording();
//     let (options, endpoint) = recorded_test_setup(recording, BlobClientOptions::default()).await;
//     let container_name = recording
//         .random_string::<17>(Some("container"))
//         .to_ascii_lowercase();
//     let blob_name = recording
//         .random_string::<12>(Some("blob"))
//         .to_ascii_lowercase();

//     // Act
//     let container_client = ContainerClient::new(
//         &endpoint,
//         container_name.clone(),
//         recording.credential(),
//         Some(options.clone()),
//     )?;
//     container_client.create_container(None).await?;

//     let blob_client = BlobClient::new(
//         &endpoint,
//         container_name,
//         blob_name,
//         recording.credential(),
//         Some(options),
//     )?;
//     let data = b"test download content";
//     blob_client
//         .upload_blob(
//             RequestContent::from(data.to_vec()),
//             true,
//             i64::try_from(data.len())?,
//             None,
//         )
//         .await?;
//     let response = blob_client.download_blob(None).await?;

//     // Assert
//     let (status_code, headers, response_body) = response.deconstruct();
//     assert!(status_code.is_success());
//     assert_eq!(
//         "21",
//         headers.get_str(&HeaderName::from_static("content-length"))?
//     );
//     assert_eq!(Bytes::from_static(data), response_body.collect().await?);

//     container_client.delete_container(None).await?;
//     Ok(())
// }

// #[recorded::test]
// async fn test_upload_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
//     // Recording Setup
//     let recording = ctx.recording();
//     let (options, endpoint) = recorded_test_setup(recording, BlobClientOptions::default()).await;
//     let container_name = recording
//         .random_string::<17>(Some("container"))
//         .to_ascii_lowercase();
//     let blob_name = recording
//         .random_string::<12>(Some("blob"))
//         .to_ascii_lowercase();

//     // Act
//     let container_client = ContainerClient::new(
//         &endpoint,
//         container_name.clone(),
//         recording.credential(),
//         Some(options.clone()),
//     )?;
//     container_client.create_container(None).await?;

//     let blob_client = BlobClient::new(
//         &endpoint,
//         container_name,
//         blob_name,
//         recording.credential(),
//         Some(options),
//     )?;

//     let data = b"hello rusty world";
//     let response = blob_client
//         .upload_blob(
//             RequestContent::from(data.to_vec()),
//             false,
//             i64::try_from(data.len())?,
//             None,
//         )
//         .await?;

//     // Assert
//     assert_eq!(response.status(), StatusCode::Created);

//     container_client.delete_container(None).await?;
//     Ok(())
// }

// #[recorded::test]
// async fn test_upload_blob_overwrite(ctx: TestContext) -> Result<(), Box<dyn Error>> {
//     // Recording Setup
//     let recording = ctx.recording();
//     let (options, endpoint) = recorded_test_setup(recording, BlobClientOptions::default()).await;
//     let container_name = recording
//         .random_string::<17>(Some("container"))
//         .to_ascii_lowercase();
//     let blob_name = recording
//         .random_string::<12>(Some("blob"))
//         .to_ascii_lowercase();

//     // Act
//     let container_client = ContainerClient::new(
//         &endpoint,
//         container_name.clone(),
//         recording.credential(),
//         Some(options.clone()),
//     )?;
//     container_client.create_container(None).await?;

//     let blob_client = BlobClient::new(
//         &endpoint,
//         container_name,
//         blob_name,
//         recording.credential(),
//         Some(options),
//     )?;

//     let old_data = b"hello rusty world";
//     blob_client
//         .upload_blob(
//             RequestContent::from(old_data.to_vec()),
//             false,
//             i64::try_from(old_data.len())?,
//             None,
//         )
//         .await?;

//     let new_data = b"hello overwritten rusty world";

//     // Error Case (overwrite=false/none)
//     let error_response = blob_client
//         .upload_blob(
//             RequestContent::from(new_data.to_vec()),
//             false,
//             i64::try_from(new_data.len())?,
//             None,
//         )
//         .await;
//     assert!(error_response.is_err());

//     // Working Case (overwrite=true)
//     let upload_response = blob_client
//         .upload_blob(
//             RequestContent::from(new_data.to_vec()),
//             true,
//             i64::try_from(new_data.len())?,
//             None,
//         )
//         .await?;
//     let response = blob_client.download_blob(None).await?;

//     // Assert
//     assert_eq!(upload_response.status(), StatusCode::Created);
//     let (status_code, headers, response_body) = response.deconstruct();
//     assert!(status_code.is_success());
//     assert_eq!(
//         "29",
//         headers.get_str(&HeaderName::from_static("content-length"))?
//     );
//     assert_eq!(Bytes::from_static(new_data), response_body.collect().await?);

//     container_client.delete_container(None).await?;
//     Ok(())
// }

// #[recorded::test]
// async fn test_put_block_list(ctx: TestContext) -> Result<(), Box<dyn Error>> {
//     // Recording Setup
//     let recording = ctx.recording();
//     let (options, endpoint) = recorded_test_setup(recording, BlobClientOptions::default()).await;
//     let container_name = recording
//         .random_string::<17>(Some("container"))
//         .to_ascii_lowercase();
//     let blob_name = recording
//         .random_string::<12>(Some("blob"))
//         .to_ascii_lowercase();

//     // Act
//     let container_client = ContainerClient::new(
//         &endpoint,
//         container_name.clone(),
//         recording.credential(),
//         Some(options.clone()),
//     )?;
//     container_client.create_container(None).await?;

//     let blob_client = BlobClient::new(
//         &endpoint,
//         container_name,
//         blob_name,
//         recording.credential(),
//         Some(options),
//     )?;

//     let block_1 = b"AAA";
//     let block_2 = b"BBB";
//     let block_3 = b"CCC";

//     blob_client
//         .stage_block(
//             "1",
//             i64::try_from(block_1.len())?,
//             RequestContent::from(block_1.to_vec()),
//             None,
//         )
//         .await?;

//     blob_client
//         .stage_block(
//             "2",
//             i64::try_from(block_2.len())?,
//             RequestContent::from(block_2.to_vec()),
//             None,
//         )
//         .await?;
//     blob_client
//         .stage_block(
//             "3",
//             i64::try_from(block_3.len())?,
//             RequestContent::from(block_3.to_vec()),
//             None,
//         )
//         .await?;

//     let latest_blocks = BlobBlock::new(vec![
//         String::from("1"),
//         String::from("2"),
//         String::from("3"),
//     ]);

//     let block_lookup_list = BlockLookupList {
//         committed: None,
//         latest: Some(latest_blocks.into()),
//         uncommitted: None,
//     };

//     let request_content = RequestContent::try_from(block_lookup_list)?;

//     blob_client.commit_block_list(request_content, None).await?;

//     let response = blob_client.download_blob(None).await?;

//     // Assert
//     let (status_code, headers, response_body) = response.deconstruct();
//     assert!(status_code.is_success());
//     assert_eq!(
//         "9",
//         headers.get_str(&HeaderName::from_static("content-length"))?
//     );
//     assert_eq!(
//         Bytes::from_static(b"AAABBBCCC"),
//         response_body.collect().await?
//     );

//     container_client.delete_container(None).await?;
//     Ok(())
// }

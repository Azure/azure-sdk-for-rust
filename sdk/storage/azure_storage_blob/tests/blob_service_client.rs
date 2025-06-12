// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::BlobServiceClientGetPropertiesOptions;
use azure_storage_blob_test::get_blob_service_client;
use futures::StreamExt;
use std::error::Error;

#[recorded::test]
async fn test_get_service_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let service_client = get_blob_service_client(recording)?;

    let response = service_client
        .get_properties(Some(BlobServiceClientGetPropertiesOptions::default()))
        .await?;

    // Assert
    let storage_service_properties = response.into_body().await?;
    let hour_metrics = storage_service_properties.hour_metrics;
    assert!(hour_metrics.is_some());
    Ok(())
}

// #[recorded::test]
// async fn test_list_containers(ctx: TestContext) -> Result<(), Box<dyn Error>> {
//     // Recording Setup
//     let recording = ctx.recording();
//     let service_client = get_blob_service_client(recording)?;
//     let container_names = [
//         "testcontainer1".to_string(),
//         "testcontainer2".to_string(),
//         "testcontainer3".to_string(),
//         "testcontainer4".to_string(),
//     ];
//     let mut container_clients = Vec::new();
//     for container_name in container_names.clone() {
//         let container_client = service_client.blob_container_client(container_name);
//         container_client.create_container(None).await?;
//         container_clients.push(container_client);
//     }

//     // Assert
//     let mut pager_response = service_client.list_containers(None)?;
//     while let Some(page) = pager_response.next().await {
//         let current_page = page.unwrap().into_body().await?;
//         let container_list = current_page.container_items;
//         for container in container_list {
//             let container_name = container.name.unwrap();
//             assert!(container_names.contains(&container_name));
//         }
//     }

//     for container_client in container_clients {
//         container_client.delete_container(None).await?;
//     }

//     Ok(())
// }

#[recorded::test]
async fn test_list_containers_singleton(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let service_client = get_blob_service_client(recording)?;
    let container_client = service_client.blob_container_client("testcontainer".to_string());
    container_client.create_container(None).await?;

    // Assert
    let mut pager_response = service_client.list_containers(None)?;
    while let Some(page) = pager_response.next().await {
        let current_page = page.unwrap().into_body().await?;
        let container_list = current_page.container_items;
        for container in container_list {
            println!("Is Deleted Option Some: {}", container.delete.is_some());
            println!("Is Metadata Option Some: {}", container.metadata.is_some());
            println!("Is Name Option Some: {}", container.name.is_some());
            println!(
                "Is Properties Option Some: {}",
                container.properties.is_some()
            );
            println!("Is Version Option Some: {}", container.version.is_some());
        }
    }

    container_client.delete_container(None).await?;

    Ok(())
}

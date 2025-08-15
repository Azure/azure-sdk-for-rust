// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{
    BlobServiceClientGetPropertiesOptions, BlobServiceClientListContainersSegmentOptions,
};
use azure_storage_blob_test::{get_blob_service_client, get_container_name};
use futures::StreamExt;
use std::collections::HashMap;
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
    let blob_service_properties = response.into_body().await?;
    let hour_metrics = blob_service_properties.hour_metrics;
    assert!(hour_metrics.is_some());
    Ok(())
}

#[recorded::test]
async fn test_list_containers(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let service_client = get_blob_service_client(recording)?;
    let mut container_names = HashMap::from([
        (get_container_name(recording), 0),
        (get_container_name(recording), 0),
        (get_container_name(recording), 0),
        (get_container_name(recording), 0),
    ]);
    let mut container_clients = Vec::new();
    for container_name in container_names.keys() {
        let container_client = service_client.blob_container_client(container_name.to_string());
        container_client.create_container(None).await?;
        container_clients.push(container_client);
    }

    // Assert
    let mut pager_response = service_client.list_containers(None)?;
    while let Some(page) = pager_response.next().await {
        let current_page = page.unwrap().into_body().await?;
        let container_list = current_page.container_items;
        for container in container_list {
            let container_name = container.name.unwrap();
            if container_names.contains_key(&container_name) {
                container_names
                    .entry(container_name)
                    .and_modify(|val| *val = 1);
            }
        }
    }

    for containers in container_names {
        assert_eq!(containers.1, 1)
    }

    for container_client in container_clients {
        container_client.delete_container(None).await?;
    }

    Ok(())
}

#[recorded::test]
async fn test_list_containers_with_continuation(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let service_client = get_blob_service_client(recording)?;
    let mut container_names = HashMap::from([
        (get_container_name(recording), 0),
        (get_container_name(recording), 0),
        (get_container_name(recording), 0),
        (get_container_name(recording), 0),
    ]);
    let mut container_clients = Vec::new();
    for container_name in container_names.keys() {
        let container_client = service_client.blob_container_client(container_name.to_string());
        container_client.create_container(None).await?;
        container_clients.push(container_client);
    }

    let list_containers_options = BlobServiceClientListContainersSegmentOptions {
        maxresults: Some(2),
        ..Default::default()
    };

    // Assert
    let mut pager_response = service_client.list_containers(Some(list_containers_options))?;
    let mut page_count = 0;
    while let Some(page) = pager_response.next().await {
        page_count += 1;
        let current_page = page.unwrap().into_body().await?;
        let container_list = current_page.container_items;
        for container in container_list {
            let container_name = container.name.unwrap();
            if container_names.contains_key(&container_name) {
                container_names
                    .entry(container_name)
                    .and_modify(|val| *val = 1);
            }
        }
    }

    for containers in container_names {
        assert_eq!(containers.1, 1)
    }
    assert!(page_count >= 2);

    for container_client in container_clients {
        container_client.delete_container(None).await?;
    }

    Ok(())
}

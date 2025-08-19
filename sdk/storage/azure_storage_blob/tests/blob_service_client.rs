// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{RequestContent, XmlFormat};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{
    AccountKind, BlobServiceClientGetAccountInfoResultHeaders,
    BlobServiceClientGetPropertiesOptions, BlobServiceClientListContainersSegmentOptions,
    StorageServiceProperties,
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
    let storage_service_properties = response.into_body().await?;
    let hour_metrics = storage_service_properties.hour_metrics;
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

#[recorded::test]
async fn test_set_service_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let service_client = get_blob_service_client(recording)?;

    // Storage Service Properties
    let storage_service_properties = StorageServiceProperties {
        default_service_version: Some("2022-11-02".to_string()),
        ..Default::default()
    };
    let request_content: RequestContent<StorageServiceProperties, XmlFormat> =
        storage_service_properties.try_into()?;

    service_client.set_properties(request_content, None).await?;

    // Assert
    let response = service_client.get_properties(None).await?;
    let storage_service_properties = response.into_body().await?;
    let default_service_version = storage_service_properties.default_service_version;
    assert_eq!("2022-11-02".to_string(), default_service_version.unwrap());
    Ok(())
}

#[recorded::test]
async fn test_get_account_info(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let service_client = get_blob_service_client(recording)?;

    // Act
    let response = service_client.get_account_info(None).await?;

    // Assert
    let sku_name = response.sku_name()?;
    let account_kind = response.account_kind()?;

    assert!(sku_name.is_some());
    assert_eq!(AccountKind::StorageV2, account_kind.unwrap());

    Ok(())
}

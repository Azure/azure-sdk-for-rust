// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod common;

use azure_core::{http::StatusCode, time::OffsetDateTime, Result};
use azure_core_test::{recorded, Recording, TestContext};
use azure_storage_queue::{
    models::{GeoReplicationStatus, ListQueuesIncludeType, QueueServiceClientListQueuesOptions},
    QueueServiceClient, QueueServiceClientOptions,
};
use common::{assert_successful_response, get_queue_name, recorded_test_setup};
use futures::StreamExt;

use std::collections::HashMap;
use std::option::Option;

/// Creates a new queue under the given account.
#[recorded::test]
async fn test_create_queue(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client(recording).await?;
    let queue_name = get_queue_name(recording);

    // Act
    let response = queue_service_client
        .queue_client(&queue_name)?
        .create(None)
        .await?;
    let test_result = async {
        // Assert
        assert_successful_response(&response);
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_service_client
        .queue_client(&queue_name)?
        .delete(None)
        .await
        .unwrap();

    test_result?;

    Ok(())
}

/// Deletes an existing queue.
#[recorded::test]
async fn test_delete_queue(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client(recording).await?;
    let queue_name = get_queue_name(recording);

    // Arrange
    queue_service_client
        .queue_client(&queue_name)?
        .create(None)
        .await?;

    // Act
    let response = queue_service_client
        .queue_client(&queue_name)?
        .delete(None)
        .await?;

    // Assert
    assert!(
        response.status() == StatusCode::NoContent,
        "Expected status code 204, got {}",
        response.status(),
    );
    Ok(())
}

/// Gets the properties of the Queue service.
#[recorded::test]
async fn test_get_queue_properties(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client(recording).await?;

    // Act
    let response = queue_service_client.get_properties(None).await?;

    // Assert
    assert!(
        response.status() == StatusCode::Ok,
        "Expected status code 200, got {}",
        response.status(),
    );

    Ok(())
}

/// Sets Queue service properties.
#[recorded::test]
async fn test_set_queue_properties(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client(recording).await?;

    // Arrange
    let properties = queue_service_client
        .get_properties(None)
        .await?
        .into_model()?;

    // Act
    let response = queue_service_client
        .set_properties(properties.try_into()?, None)
        .await?;

    // Assert
    assert!(
        response.status() == StatusCode::Accepted,
        "Expected status code 202, got {}",
        response.status(),
    );

    Ok(())
}

/// Lists all queues in the storage account, ensuring that at least one queue is present.
#[recorded::test]
pub async fn test_list_queues(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client(recording).await?;
    let queue_name = get_queue_name(recording);

    // Arrange - create a queue so the listing is guaranteed to contain at least one entry
    queue_service_client
        .queue_client(&queue_name)?
        .create(None)
        .await?;

    // Act
    let options = QueueServiceClientListQueuesOptions {
        maxresults: Some(1),
        ..Default::default()
    };

    let mut page_iterator = queue_service_client
        .list_queues(Some(options))?
        .into_pages();
    let mut all_queue_names = Vec::new();

    // Act - iterate through all pages
    while let Some(page) = page_iterator.next().await {
        let response = page?;
        let queue_list = response.into_model()?;

        // Collect queue names from this page.
        for queue_item in &queue_list.queue_items {
            if let Some(queue_name_found) = &queue_item.name {
                all_queue_names.push(queue_name_found.clone());
            }
        }
    }

    // Assert - the test queue appears in the list
    assert!(
        all_queue_names.contains(&queue_name),
        "Expected queue '{}' to be found in the list of queues: {:?}",
        queue_name,
        all_queue_names
    );

    // Cleanup
    queue_service_client
        .queue_client(&queue_name)?
        .delete(None)
        .await?;

    Ok(())
}

/// Lists queues filtered by a prefix and checks that all returned queues share the prefix.
#[recorded::test]
pub async fn test_list_queues_with_prefix(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client(recording).await?;
    let queue_name = get_queue_name(recording);
    let queue_name_a = format!("{queue_name}-a");
    let queue_name_b = format!("{queue_name}-b");

    // Arrange - create two queues that share the same base name as prefix
    queue_service_client
        .queue_client(&queue_name_a)?
        .create(None)
        .await?;
    queue_service_client
        .queue_client(&queue_name_b)?
        .create(None)
        .await?;

    let test_result = async {
        // Act
        let options = QueueServiceClientListQueuesOptions {
            prefix: Some(queue_name.clone()),
            ..Default::default()
        };

        let mut page_iterator = queue_service_client
            .list_queues(Some(options))?
            .into_pages();
        let mut returned_names = Vec::new();

        while let Some(page) = page_iterator.next().await {
            let queue_list = page?.into_model()?;
            for queue_item in &queue_list.queue_items {
                if let Some(name) = &queue_item.name {
                    returned_names.push(name.clone());
                }
            }
        }

        // Assert - both test queues are returned
        assert!(
            returned_names.contains(&queue_name_a),
            "Expected '{}' in results: {:?}",
            queue_name_a,
            returned_names
        );
        assert!(
            returned_names.contains(&queue_name_b),
            "Expected '{}' in results: {:?}",
            queue_name_b,
            returned_names
        );

        // Assert - all returned queues start with the prefix
        for name in &returned_names {
            assert!(
                name.starts_with(&queue_name),
                "Queue '{}' does not start with prefix '{}'",
                name,
                queue_name
            );
        }

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_service_client
        .queue_client(&queue_name_a)?
        .delete(None)
        .await
        .unwrap();
    queue_service_client
        .queue_client(&queue_name_b)?
        .delete(None)
        .await
        .unwrap();

    test_result?;

    Ok(())
}

/// Lists queues with metadata included and checks the metadata on the matching queue.
#[recorded::test]
pub async fn test_list_queues_include_metadata(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client(recording).await?;
    let queue_name = get_queue_name(recording);

    // Arrange - create a queue and set metadata on it
    queue_service_client
        .queue_client(&queue_name)?
        .create(None)
        .await?;

    let test_result = async {
        // Arrange - set metadata on the queue
        let metadata = HashMap::from([("env".to_string(), "test".to_string())]);
        queue_service_client
            .queue_client(&queue_name)?
            .set_metadata(&metadata, None)
            .await?;

        // Act - list queues with metadata included and filter to our prefix
        let options = QueueServiceClientListQueuesOptions {
            prefix: Some(queue_name.clone()),
            include: Some(vec![ListQueuesIncludeType::Metadata]),
            ..Default::default()
        };

        let mut page_iterator = queue_service_client
            .list_queues(Some(options))?
            .into_pages();
        let mut found_queue = None;

        while let Some(page) = page_iterator.next().await {
            let queue_list = page?.into_model()?;
            for queue_item in queue_list.queue_items {
                if queue_item.name.as_deref() == Some(&queue_name) {
                    found_queue = Some(queue_item);
                    break;
                }
            }
            if found_queue.is_some() {
                break;
            }
        }

        // Assert - queue was found in the listing
        let found = found_queue.expect("Expected to find test queue in list");

        // Assert - metadata was returned and contains the expected key-value pair
        let returned_metadata = found
            .metadata
            .expect("Expected metadata to be present when include=metadata");
        assert_eq!(
            returned_metadata.get("env").map(String::as_str),
            Some("test"),
            "Expected metadata key 'env' with value 'test'"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_service_client
        .queue_client(&queue_name)?
        .delete(None)
        .await
        .unwrap();

    test_result?;

    Ok(())
}

/// Gets Queue service statistics.
#[recorded::test]
pub async fn test_get_queue_statistics(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client_secondary(recording).await?;

    // Act
    let response = queue_service_client.get_statistics(None).await?;

    // Assert
    assert!(
        response.status() == StatusCode::Ok,
        "Expected status code 200, got {}",
        response.status(),
    );
    let stats = response.into_model()?;
    let geo_replication = stats.geo_replication.as_ref().unwrap();
    assert!(
        geo_replication.status.as_ref().unwrap() == &GeoReplicationStatus::Live,
        "Geo-replication status should be Live"
    );
    // Assert - `last_sync_time` is greater than Fri, 1 Jun 2025 00:00:00 GMT.
    assert!(
        geo_replication.last_sync_time.unwrap()
            > OffsetDateTime::from_unix_timestamp(1748728800).unwrap(),
        "Last sync time should be after 2025-06-01T00:00:00Z"
    );

    Ok(())
}

/// Returns an instance of a QueueServiceClient.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
pub async fn get_queue_service_client(recording: &Recording) -> Result<QueueServiceClient> {
    let (options, endpoint, _) = recorded_test_setup(recording);
    let queue_client_options = QueueServiceClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    let queue_client = QueueServiceClient::new(
        &endpoint,
        Some(recording.credential()),
        Option::Some(queue_client_options),
    )?;

    Ok(queue_client)
}

/// Returns an instance of a QueueServiceClient on the secondary endpoint.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
pub async fn get_queue_service_client_secondary(
    recording: &Recording,
) -> Result<QueueServiceClient> {
    let (options, _, endpoint) = recorded_test_setup(recording);
    let queue_client_options = QueueServiceClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    let queue_client = QueueServiceClient::new(
        &endpoint,
        Some(recording.credential()),
        Option::Some(queue_client_options),
    )?;

    Ok(queue_client)
}

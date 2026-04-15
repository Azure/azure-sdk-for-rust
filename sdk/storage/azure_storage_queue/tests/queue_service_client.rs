// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod common;

use azure_core::{http::StatusCode, time::OffsetDateTime, Result};
use azure_core_test::{recorded, Recording, TestContext, TestMode};
use azure_storage_queue::{
    models::{
        CorsRule, GeoReplicationStatus, ListQueuesIncludeType, Logging, Metrics,
        QueueServiceClientListQueuesOptions, QueueServiceProperties, RetentionPolicy,
    },
    QueueServiceClient, QueueServiceClientOptions,
};
use common::{get_queue_name, recorded_test_setup};
use futures::StreamExt;

use std::{collections::HashMap, time::Duration};
use tokio::time;

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

    // Cleanup
    queue_service_client
        .queue_client(&queue_name)?
        .delete(None)
        .await
        .unwrap();

    // Assert
    assert!(
        response.status().is_success(),
        "Expected successful status code, got {}",
        response.status()
    );

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
    let queue_name_b = format!("{queue_name}-b");
    let queue_name_c = format!("{queue_name}-c");

    // Arrange - create three queues sharing the same prefix
    for name in [&queue_name, &queue_name_b, &queue_name_c] {
        queue_service_client
            .queue_client(name)?
            .create(None)
            .await?;
    }

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

    // Act - first page with prefix filter: maxresults=1
    let first_page_options = QueueServiceClientListQueuesOptions {
        prefix: Some(queue_name.clone()),
        maxresults: Some(1),
        ..Default::default()
    };
    let mut pager = queue_service_client
        .list_queues(Some(first_page_options))?
        .into_pages();
    let first_page = pager
        .next()
        .await
        .expect("Expected at least one page")?
        .into_model()?;
    assert_eq!(
        first_page.queue_items.len(),
        1,
        "Expected first page to contain exactly 1 queue"
    );
    let marker = first_page
        .next_marker
        .clone()
        .expect("Expected a next_marker to be present after first page");

    // Act - second page: resume via explicit marker
    let second_page_options = QueueServiceClientListQueuesOptions {
        prefix: Some(queue_name.clone()),
        maxresults: Some(1),
        marker: Some(marker.clone()),
        ..Default::default()
    };
    let mut pager2 = queue_service_client
        .list_queues(Some(second_page_options))?
        .into_pages();
    let second_page = pager2
        .next()
        .await
        .expect("Expected second page")?
        .into_model()?;
    assert_eq!(
        second_page.queue_items.len(),
        1,
        "Expected second page to contain exactly 1 queue"
    );

    // Assert - the two pages returned different queues
    let first_name = first_page.queue_items[0]
        .name
        .as_deref()
        .expect("Expected queue name");
    let second_name = second_page.queue_items[0]
        .name
        .as_deref()
        .expect("Expected queue name");
    assert_ne!(
        first_name, second_name,
        "Expected the two pages to return different queues"
    );

    // Cleanup
    for name in [&queue_name, &queue_name_b, &queue_name_c] {
        queue_service_client
            .queue_client(name)?
            .delete(None)
            .await
            .unwrap();
    }

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
    // Assert - `last_sync_time` is greater than 1 Jun 2025 00:00:00 GMT.
    assert!(
        geo_replication.last_sync_time.unwrap()
            > OffsetDateTime::from_unix_timestamp(1748728800).unwrap(),
        "Last sync time should be after 2025-06-01T00:00:00Z"
    );

    Ok(())
}

/// Sets account-level service properties and verifies they all round-trip correctly.
#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_set_service_properties(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client(recording).await?;

    // Arrange
    let original = queue_service_client
        .get_properties(None)
        .await?
        .into_model()?;

    let test_result = async {
        // Act - set all properties in one call
        let props = QueueServiceProperties {
            logging: Some(Logging {
                version: Some("1.0".to_string()),
                delete: Some(true),
                read: Some(true),
                write: Some(true),
                retention_policy: Some(RetentionPolicy {
                    enabled: Some(true),
                    days: Some(5),
                }),
            }),
            hour_metrics: Some(Metrics {
                version: Some("1.0".to_string()),
                enabled: Some(true),
                include_apis: Some(true),
                retention_policy: Some(RetentionPolicy {
                    enabled: Some(true),
                    days: Some(5),
                }),
            }),
            minute_metrics: Some(Metrics {
                version: Some("1.0".to_string()),
                enabled: Some(true),
                include_apis: Some(true),
                retention_policy: Some(RetentionPolicy {
                    enabled: Some(true),
                    days: Some(5),
                }),
            }),
            cors: Some(vec![
                CorsRule {
                    allowed_origins: Some("http://www.contoso.com".to_string()),
                    allowed_methods: Some("GET,PUT".to_string()),
                    allowed_headers: Some("x-ms-meta-*".to_string()),
                    exposed_headers: Some("x-ms-meta-data*".to_string()),
                    max_age_in_seconds: Some(3600),
                },
                CorsRule {
                    allowed_origins: Some("http://www.fabrikam.com".to_string()),
                    allowed_methods: Some("POST".to_string()),
                    allowed_headers: Some("Content-Type".to_string()),
                    exposed_headers: Some("Content-Length".to_string()),
                    max_age_in_seconds: Some(1800),
                },
            ]),
        };
        queue_service_client
            .set_properties(props.try_into()?, None)
            .await?;

        // Allow settings to propagate in live/record mode
        if recording.test_mode() == TestMode::Live || recording.test_mode() == TestMode::Record {
            time::sleep(Duration::from_secs(15)).await;
        }

        // Act - read back
        let updated = queue_service_client
            .get_properties(None)
            .await?
            .into_model()?;

        // Assert - logging
        let logging = updated.logging.as_ref().expect("Expected logging settings");
        assert_eq!(logging.delete, Some(true), "Expected delete logging = true");
        assert_eq!(logging.read, Some(true), "Expected read logging = true");
        assert_eq!(logging.write, Some(true), "Expected write logging = true");
        let rp = logging
            .retention_policy
            .as_ref()
            .expect("Expected retention policy on logging");
        assert_eq!(rp.enabled, Some(true), "Expected logging retention enabled");
        assert_eq!(rp.days, Some(5), "Expected logging retention days = 5");

        // Assert - hour metrics
        let hm = updated
            .hour_metrics
            .as_ref()
            .expect("Expected hour_metrics settings");
        assert_eq!(
            hm.enabled,
            Some(true),
            "Expected hour_metrics enabled = true"
        );
        assert_eq!(
            hm.include_apis,
            Some(true),
            "Expected hour_metrics include_apis = true"
        );
        let rp = hm
            .retention_policy
            .as_ref()
            .expect("Expected retention policy on hour_metrics");
        assert_eq!(
            rp.enabled,
            Some(true),
            "Expected hour_metrics retention enabled"
        );
        assert_eq!(rp.days, Some(5), "Expected hour_metrics retention days = 5");

        // Assert - minute metrics
        let mm = updated
            .minute_metrics
            .as_ref()
            .expect("Expected minute_metrics settings");
        assert_eq!(
            mm.enabled,
            Some(true),
            "Expected minute_metrics enabled = true"
        );
        assert_eq!(
            mm.include_apis,
            Some(true),
            "Expected minute_metrics include_apis = true"
        );
        let rp = mm
            .retention_policy
            .as_ref()
            .expect("Expected retention policy on minute_metrics");
        assert_eq!(
            rp.enabled,
            Some(true),
            "Expected minute_metrics retention enabled"
        );
        assert_eq!(
            rp.days,
            Some(5),
            "Expected minute_metrics retention days = 5"
        );

        // Assert - CORS rules
        let returned = updated.cors.as_ref().expect("Expected CORS rules");
        assert_eq!(returned.len(), 2, "Expected exactly 2 CORS rules");
        assert_eq!(
            returned[0].allowed_origins.as_deref(),
            Some("http://www.contoso.com")
        );
        assert_eq!(returned[0].allowed_methods.as_deref(), Some("GET,PUT"));
        assert_eq!(returned[0].max_age_in_seconds, Some(3600));
        assert_eq!(
            returned[1].allowed_origins.as_deref(),
            Some("http://www.fabrikam.com")
        );
        assert_eq!(returned[1].allowed_methods.as_deref(), Some("POST"));
        assert_eq!(returned[1].max_age_in_seconds, Some(1800));

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Restore original properties regardless of test outcome
    queue_service_client
        .set_properties(original.try_into()?, None)
        .await
        .unwrap();

    test_result?;
    Ok(())
}

/// Setting more than 5 CORS rules is rejected by the service with 400 Bad Request.
#[recorded::test]
async fn test_set_cors_too_many_rules(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client(recording).await?;

    let original = queue_service_client
        .get_properties(None)
        .await?
        .into_model()?;

    let test_result = async {
        let rule = CorsRule {
            allowed_origins: Some("http://example.com".to_string()),
            allowed_methods: Some("GET".to_string()),
            allowed_headers: Some("*".to_string()),
            exposed_headers: Some("*".to_string()),
            max_age_in_seconds: Some(60),
        };
        let props = QueueServiceProperties {
            cors: Some(vec![
                rule.clone(),
                rule.clone(),
                rule.clone(),
                rule.clone(),
                rule.clone(),
                rule.clone(),
            ]),
            ..original.clone()
        };

        // Act
        let err = queue_service_client
            .set_properties(props.try_into()?, None)
            .await
            .err()
            .unwrap();

        // Assert
        assert_eq!(
            err.http_status(),
            Some(StatusCode::BadRequest),
            "Expected 400 Bad Request for too many CORS rules, got {:?}",
            err.http_status()
        );
        Ok::<(), azure_core::Error>(())
    }
    .await;

    test_result?;
    Ok(())
}

/// Setting a retention policy with days > 365 is rejected by the service with 400 Bad Request.
#[recorded::test]
async fn test_set_retention_too_long(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client(recording).await?;

    let original = queue_service_client
        .get_properties(None)
        .await?
        .into_model()?;

    let test_result = async {
        let props = QueueServiceProperties {
            logging: Some(Logging {
                version: Some("1.0".to_string()),
                delete: Some(false),
                read: Some(false),
                write: Some(false),
                retention_policy: Some(RetentionPolicy {
                    enabled: Some(true),
                    days: Some(366),
                }),
            }),
            ..original.clone()
        };

        // Act
        let err = queue_service_client
            .set_properties(props.try_into()?, None)
            .await
            .err()
            .unwrap();

        // Assert
        assert_eq!(
            err.http_status(),
            Some(StatusCode::BadRequest),
            "Expected 400 Bad Request for retention days > 365, got {:?}",
            err.http_status()
        );
        Ok::<(), azure_core::Error>(())
    }
    .await;

    test_result?;
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
        Some(queue_client_options),
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
        Some(queue_client_options),
    )?;

    Ok(queue_client)
}

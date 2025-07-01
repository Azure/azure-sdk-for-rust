use azure_core::http::{
    RequestContent, {ClientOptions, Response},
};
use azure_core::Result;
use azure_core_test::{recorded, Recording, TestContext};
use azure_storage_queue::{
    clients::{QueueServiceClient, QueueServiceClientOptions},
    models::{GeoReplicationStatusType, QueueServiceClientListQueuesOptions},
};
use futures::StreamExt;

use std::option::Option;

/// Creates a new queue under the given account.
#[recorded::test]
async fn test_create_queue(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client(recording).await?;

    let response = queue_service_client
        .create_queue("test-service-create-queue", None)
        .await?;
    let test_result = async {
        assert_successful_response(&response);
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_service_client
        .delete_queue("test-service-create-queue", None)
        .await
        .unwrap();

    test_result?;

    Ok(())
}

/// Tests the deletion of a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_delete_queue(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client(recording).await?;

    let queue_name = "test-service-delete-queue";

    queue_service_client.create_queue(queue_name, None).await?;

    let response = queue_service_client.delete_queue(queue_name, None).await?;

    assert!(
        response.status() == 204,
        "Expected status code 204, got {}",
        response.status(),
    );
    Ok(())
}

/// Retrieves the properties of a storage account's Queue service.
#[recorded::test]
async fn test_get_queue_properties(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client(recording).await?;

    let response = queue_service_client.get_properties(None).await.unwrap();

    assert!(
        response.status() == 200,
        "Expected status code 200, got {}",
        response.status(),
    );

    Ok(())
}

/// Retrieves the properties of a storage account's Queue service.
#[recorded::test]
async fn test_set_queue_properties(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client(recording).await?;

    let properties = queue_service_client
        .get_properties(None)
        .await?
        .into_body()
        .await?;
    let properties_xml = quick_xml::se::to_string(&properties).unwrap();
    let properties_bytes = properties_xml.into_bytes();

    let response = queue_service_client
        .set_properties(RequestContent::from(properties_bytes), None)
        .await
        .unwrap();

    assert!(
        response.status() == 202,
        "Expected status code 202, got {}",
        response.status(),
    );

    Ok(())
}

/// Lists all queues in the storage account, ensuring that at least one queue is present.
#[recorded::test]
pub async fn test_list_queues(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client(recording).await?;

    // Create a queue to ensure we have at least one queue to list
    let queue_name = "test-service-list-queues";
    queue_service_client.create_queue(queue_name, None).await?;

    let options = QueueServiceClientListQueuesOptions {
        maxresults: Some(1),
        ..Default::default()
    };

    let mut page_iterator = queue_service_client.list_queues(Some(options))?;
    let mut all_queue_names = Vec::new();

    // Iterate through all pages
    while let Some(page) = page_iterator.next().await {
        let response = page?;
        let queue_list = response.into_body().await?;

        //Collect queue names from this page
        for queue_item in &queue_list.queue_items {
            if let Some(queue_name_found) = &queue_item.name {
                all_queue_names.push(queue_name_found.clone());
            }
        }
    }

    // Assert that our test queue is in the list
    assert!(
        all_queue_names.contains(&queue_name.to_string()),
        "Expected queue '{}' to be found in the list of queues: {:?}",
        queue_name,
        all_queue_names
    );

    // Clean up by deleting the created queue
    queue_service_client.delete_queue(queue_name, None).await?;

    Ok(())
}

/// Gets statistics for the Queue service, ensuring that the service is available and returns a successful response.
#[recorded::test]
pub async fn test_get_queue_statistics(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_service_client = get_queue_service_client_secondary(recording).await?;

    let response = queue_service_client.get_statistics(None).await?;
    assert!(
        response.status() == 200,
        "Expected status code 200, got {}",
        response.status(),
    );
    let stats = response.into_body().await?;
    let geo_replication = stats.geo_replication.as_ref().unwrap();
    assert!(
        geo_replication.status.as_ref().unwrap() == &GeoReplicationStatusType::Live,
        "Geo-replication status should be Live"
    );
    // assert that last_sync_time is greater than Fri, 1 Jun 2025 00:00:00 GMT
    assert!(
        geo_replication.last_sync_time.unwrap()
            > time::OffsetDateTime::from_unix_timestamp(1748728800).unwrap(),
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
        recording.credential(),
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
        recording.credential(),
        Option::Some(queue_client_options),
    )?;

    Ok(queue_client)
}

/// Takes in a Recording instance and returns an instrumented options bag and endpoint.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
fn recorded_test_setup(recording: &Recording) -> (ClientOptions, String, String) {
    let mut client_options = ClientOptions::default();
    recording.instrument(&mut client_options);
    let endpoint = format!(
        "https://{}.queue.core.windows.net/",
        recording
            .var("AZURE_QUEUE_STORAGE_ACCOUNT_NAME", None)
            .as_str()
    );
    let secondary_endpoint = format!(
        "https://{}-secondary.queue.core.windows.net/",
        recording
            .var("AZURE_QUEUE_STORAGE_ACCOUNT_NAME", None)
            .as_str()
    );

    (client_options, endpoint, secondary_endpoint)
}

/// Helper function to verify a successful response
fn assert_successful_response<T, F>(response: &Response<T, F>) {
    assert!(
        response.status().is_success(),
        "Expected successful status code, got {}",
        response.status()
    );
}

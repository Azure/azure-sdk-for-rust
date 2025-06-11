use azure_core::{http::ClientOptions, Result};
use azure_core_test::{recorded, Recording, TestContext};
use azure_storage_queue::clients::AzureQueueStorageClientOptions;
use azure_storage_queue::clients::QueueClient;
use std::option::Option;

/// Creates a new queue under the given account.
#[recorded::test]
async fn test_create_queue(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await;

    let response = queue_client?.create("test-queue", None).await?;

    assert!(
        response.status() == 201,
        "Expected status code 201, got {}",
        response.status(),
    );

    Ok(())
}

#[recorded::test]
/// Tests the creation of a queue in Azure Storage Queue service, ensuring it does not fail if the queue already exists.
async fn test_create_queue_if_not_exists(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;

    // First, create the queue
    let response = queue_client.create("test-queue-if-exists", None).await?;
    assert!(
        response.status() == 201,
        "Expected status code 201, got {}",
        response.status(),
    );

    // Now, try to create the same queue again
    let response = queue_client
        .create_if_not_exists("test-queue-if-exists", None)
        .await?;
    assert!(
        response.status() == 204,
        "Expected status code 204, got {}",
        response.status(),
    );

    Ok(())
}

/// Tests the deletion of a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_delete_queue(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await;

    let response = queue_client?.delete("test-queue", None).await?;

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
    let queue_client = get_queue_client(recording).await;

    let response = queue_client?.get_properties().await?;

    assert!(
        response.status() == 200,
        "Expected status code 200, got {}",
        response.status(),
    );

    Ok(())
}

#[recorded::test]
async fn test_exists(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;

    // Check if a queue exists
    let exists_response = queue_client.exists("test-queue").await?;
    assert!(exists_response, "Queue should exist");

    // Check a non-existent queue
    let non_existent_exists_response = queue_client.exists("non-existent-queue").await?;
    assert!(!non_existent_exists_response, "Queue should not exist");

    Ok(())
}

/// Returns an instance of a QueueClient.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
/// * `create` - An optional flag to determine whether the container should also be created.
pub async fn get_queue_client(recording: &Recording) -> Result<QueueClient> {
    let (options, endpoint) = recorded_test_setup(recording);
    let queue_client_options = AzureQueueStorageClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    let queue_client = QueueClient::new(
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
fn recorded_test_setup(recording: &Recording) -> (ClientOptions, String) {
    let mut client_options = ClientOptions::default();
    recording.instrument(&mut client_options);
    let endpoint = format!(
        "https://{}.queue.core.windows.net/",
        recording
            .var("AZURE_QUEUE_STORAGE_ACCOUNT_NAME", None)
            .as_str()
    );

    (client_options, endpoint)
}

use azure_core::{http::ClientOptions, Result};
use azure_core_test::{recorded, Recording, TestContext};
use azure_storage_queue::clients::AzureQueueStorageClientOptions;
use azure_storage_queue::clients::QueueClient;
use std::option::Option;
use uuid::Uuid;

use once_cell::sync::Lazy;

static QUEUE_SUFFIX: Lazy<String> = Lazy::new(|| get_random_queue_suffix());

/// Creates a new queue under the given account.
#[recorded::test]
async fn test_create_queue(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await;

    let queue_name = format!("test-queue-{}", QUEUE_SUFFIX.as_str());
    let response = queue_client?.create(&queue_name, None).await?;

    assert!(
        response.status().is_success(),
        "Expected success status code, got {}",
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
    let queue_name = String::from(format!("test-queue-if-exists-{}", QUEUE_SUFFIX.as_str()));
    let response = queue_client.create(&queue_name, None).await?;
    assert!(
        response.status().is_success(),
        "Expected success status code, got {}",
        response.status(),
    );

    // Now, try to create the same queue again
    let response = queue_client.create_if_not_exists(&queue_name, None).await?;
    assert!(
        response.status().is_success(),
        "Expected success status code, got {}",
        response.status(),
    );

    Ok(())
}

/// Tests the deletion of a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_delete_queue(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = String::from(format!("test-queue-{}", QUEUE_SUFFIX.as_str()));

    queue_client.create_if_not_exists(&queue_name, None).await?;

    let response = queue_client.delete(&queue_name, None).await?;

    assert!(
        response.status() == 204,
        "Expected status code 204, got {}",
        response.status(),
    );
    Ok(())
}

#[recorded::test]
async fn test_delete_queue_if_exists(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = String::from(format!("test-queue-if-exists-{}", QUEUE_SUFFIX.as_str()));

    // First, create the queue
    let response = queue_client.create(&queue_name, None).await?;
    assert!(
        response.status().is_success(),
        "Expected success status code, got {}",
        response.status(),
    );

    // Now, try to delete the same queue
    let response = queue_client.delete_if_exists(&queue_name, None).await?;
    assert!(
        response.status().is_success(),
        "Expected success status code, got {}",
        response.status(),
    );

    // Try to delete a non-existent queue
    let non_existent_response = queue_client
        .delete_if_exists("non-existent-queue", None)
        .await?;
    assert!(
        non_existent_response.status().is_success(),
        "Expected success status code for non-existent queue, got {}",
        non_existent_response.status(),
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
    let queue_name = String::from(format!("test-queue-{}", QUEUE_SUFFIX.as_str()));

    queue_client.create_if_not_exists(&queue_name, None).await?;

    let exists_response = queue_client.exists(&queue_name).await?;
    assert!(exists_response, "Queue should exist");

    queue_client.delete(&queue_name, None).await?;

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
fn get_random_queue_suffix() -> String {
    format!("{}", Uuid::new_v4())
}

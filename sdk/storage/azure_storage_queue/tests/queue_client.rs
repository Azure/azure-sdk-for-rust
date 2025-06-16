use azure_core::{http::ClientOptions, Result};
use azure_core_test::{recorded, Recording, TestContext};
use azure_storage_queue::clients::{
    AzureQueueStorageClientOptions, AzureQueueStorageMessagesOperationsClientDequeueOptions,
    QueueClient,
};
use once_cell::sync::Lazy;
use std::option::Option;
use uuid::Uuid;

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

/// Enqueues a message to the specified queue.
#[recorded::test]
async fn test_send_message(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = format!("test-queue-send-message-{}", QUEUE_SUFFIX.as_str());
    queue_client.create(&queue_name, None).await?;

    let test_result = async {
        let response = queue_client
            .send_message(&queue_name, "queue-message", None)
            .await?;

        assert!(
            response.status() == 201,
            "Expected status code 201, got {}",
            response.status(),
        );
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(&queue_name, None).await.unwrap();

    test_result?;

    Ok(())
}

#[recorded::test]
/// Tests the creation of a queue in Azure Storage Queue service, ensuring it does not fail if the queue already exists.
async fn test_create_queue_if_not_exists(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = format!("test-queue-if-exists-{}", QUEUE_SUFFIX.as_str());

    let test_result = async {
        // First, create the queue
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
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(&queue_name, None).await.unwrap();

    test_result?;

    Ok(())
}

/// Tests the deletion of a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_delete_queue(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = format!("test-queue-{}", QUEUE_SUFFIX.as_str());

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
    let queue_name = format!("test-queue-if-exists-{}", QUEUE_SUFFIX.as_str());

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

/// Checks if a queue exists in the Azure Storage Queue service.
#[recorded::test]
async fn test_exists(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = format!("test-queue-exists-{}", QUEUE_SUFFIX.as_str());

    let test_result = async {
        // Create a queue if it does not exist
        queue_client.create_if_not_exists(&queue_name, None).await?;

        // Check if the queue exists
        let exists_response = queue_client.exists(&queue_name).await?;
        assert!(exists_response, "Queue should exist");

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // let queue_name = format!("test-queue-{}", QUEUE_SUFFIX.as_str());
    // queue_client.create_if_not_exists(&queue_name, None).await?;

    // // Check if a queue exists
    // let exists_response = queue_client.exists(&queue_name).await?;
    // assert!(exists_response, "Queue should exist");

    queue_client.delete(&queue_name, None).await?;

    // Check a non-existent queue
    let non_existent_exists_response = queue_client.exists("non-existent-queue").await?;
    assert!(!non_existent_exists_response, "Queue should not exist");

    // Return the test result
    test_result?;

    Ok(())
}

/// Sets metadata for a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_set_metadata(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = format!("test-queue-metadata-{}", QUEUE_SUFFIX.as_str());
    queue_client.create_if_not_exists(&queue_name, None).await?;

    let test_result = async {
        // Set metadata for the queue
        let metadata = Some(
            vec![("key1", "value1"), ("key2", "value2")]
                .into_iter()
                .collect(),
        );
        let response = queue_client.set_metadata(&queue_name, metadata).await?;

        assert!(
            response.status().is_success(),
            "Expected successful status code, got {}",
            response.status()
        );
        Ok::<(), azure_core::Error>(())
    }
    .await;

    queue_client.delete(&queue_name, None).await?;

    // Return the test result
    test_result?;
    Ok(())
}

/// Deletes all messages from a queue in Azure Storage Queue service.
#[recorded::test]
async fn delete_messages(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = format!("test-delete-messages-{}", QUEUE_SUFFIX.as_str());

    // Create a queue if it does not exist
    queue_client.create_if_not_exists(&queue_name, None).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        // Delete messages from the queue
        let response = queue_client.delete_messages(&queue_name).await?;
        assert!(
            response.status().is_success(),
            "Expected successful status code, got {}",
            response.status(),
        );
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(&queue_name, None).await.unwrap();

    // Return the test result
    test_result?;
    Ok(())
}

/// Receives the first message from a queue in Azure Storage Queue service.
#[recorded::test]
async fn receive_message(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = format!("test-receive-messages-{}", QUEUE_SUFFIX.as_str());

    // Create a queue if it does not exist
    queue_client.create_if_not_exists(&queue_name, None).await?;
    queue_client
        .send_message(&queue_name, "Message 1", None)
        .await?;
    queue_client
        .send_message(&queue_name, "Message 2", None)
        .await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        // Delete messages from the queue
        let response = queue_client.receive_message(&queue_name, None).await?;
        assert!(
            response.status().is_success(),
            "Expected successful status code, got {}",
            response.status(),
        );
        let messages = response.into_body().await?;
        assert!(
            messages.clone().value.iter().len() == 1,
            "Expected to receive at least 1 message, got {}",
            messages.clone().value.iter().len()
        );
        let messages = messages.value.unwrap();
        let message = messages.first().unwrap();
        assert!(
            message.clone().message_text.unwrap() == "Message 1",
            "Expected to receive 'Message 1', got {}",
            message.clone().message_text.unwrap()
        );
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(&queue_name, None).await.unwrap();

    // Return the test result
    test_result?;
    Ok(())
}

/// Receives all messages from a queue in Azure Storage Queue service.
#[recorded::test]
async fn receive_messages(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = format!("test-receive-messages-{}", QUEUE_SUFFIX.as_str());

    // Create a queue if it does not exist
    queue_client.create_if_not_exists(&queue_name, None).await?;
    queue_client
        .send_message(&queue_name, "Message 1", None)
        .await?;
    queue_client
        .send_message(&queue_name, "Message 2", None)
        .await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        let options = Some(AzureQueueStorageMessagesOperationsClientDequeueOptions {
            number_of_messages: Some(10),
            ..Default::default()
        });

        // Delete messages from the queue
        let response = queue_client.receive_messages(&queue_name, options).await?;
        assert!(
            response.status().is_success(),
            "Expected successful status code, got {}",
            response.status(),
        );
        let messages = response.into_body().await?;
        let messages = messages.value.unwrap();
        assert!(
            messages.clone().iter().len() == 2,
            "Expected to receive 2 messages, got {}",
            messages.clone().iter().len()
        );
        let message1 = messages.first().unwrap();
        assert!(
            message1.clone().message_text.unwrap() == "Message 1",
            "Expected to receive 'Message 1', got {}",
            message1.clone().message_text.unwrap()
        );
        let message2 = messages.last().unwrap();
        assert!(
            message2.clone().message_text.unwrap() == "Message 2",
            "Expected to receive 'Message 2', got {}",
            message2.clone().message_text.unwrap()
        );
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(&queue_name, None).await.unwrap();

    // Return the test result
    test_result?;
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

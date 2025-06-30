use azure_core::http::{
    RequestContent, {ClientOptions, Response},
};
use azure_core::Result;
use azure_core_test::{recorded, Recording, TestContext};
use azure_storage_queue::{
    clients::{QueueClient, QueueClientOptions},
    models::{
        QueueClientDequeueOptions, QueueClientPeekOptions, QueueClientSetMetadataOptions,
        QueueClientUpdateOptions, QueueMessage,
    },
};

use std::collections::HashMap;

/// Creates a new queue under the given account.
#[recorded::test]
async fn test_create_queue(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-create-queue").await?;

    let response = queue_client.create(None).await?;
    let test_result = async {
        assert_successful_response(&response);
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(None).await.unwrap();

    test_result?;

    Ok(())
}

/// Enqueues a message to the specified queue.
#[recorded::test]
async fn test_enqueue_message(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-enqueue-message").await?;
    queue_client.create(None).await?;

    let test_result = async {
        let response = queue_client
            .enqueue_message("enqueue_message", None)
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
    queue_client.delete(None).await.unwrap();

    test_result?;

    Ok(())
}

#[recorded::test]
/// Tests the creation of a queue in Azure Storage Queue service, ensuring it does not fail if the queue already exists.
async fn test_create_queue_if_not_exists(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-create-queue-if-not-exists").await?;

    let test_result = async {
        // First, create the queue
        let response = queue_client.create(None).await?;
        assert_successful_response(&response);

        // Now, try to create the same queue again
        let response = queue_client.create_if_not_exists(None).await?;
        assert_successful_response(&response);

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(None).await.unwrap();

    test_result?;

    Ok(())
}

/// Tests the deletion of a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_delete_queue(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-delete-queue").await?;

    queue_client.create_if_not_exists(None).await?;

    let response = queue_client.delete(None).await?;

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
    let queue_client = get_queue_client(recording, "test-delete-queue-if-exists").await?;

    // First, create the queue
    let response = queue_client.create(None).await?;
    assert_successful_response(&response);

    // Now, try to delete the same queue
    let response = queue_client.delete_if_exists(None).await?;
    assert_successful_response(&response);

    // Try to delete a non-existent queue, as it has been already deleted
    let non_existent_response = queue_client.delete_if_exists(None).await?;
    assert_successful_response(&non_existent_response);

    Ok(())
}

/// Checks if a queue exists in the Azure Storage Queue service.
#[recorded::test]
async fn test_queue_exists(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-queue-exists").await?;

    let test_result = async {
        // Create a queue if it does not exist
        queue_client.create_if_not_exists(None).await?;

        // Check if the queue exists
        let exists_response = queue_client.exists().await?;
        assert!(exists_response, "Queue should exist");

        Ok::<(), azure_core::Error>(())
    }
    .await;

    queue_client.delete(None).await?;

    // Check a non-existent queue
    let non_existent_exists_response = queue_client.exists().await?;
    assert!(!non_existent_exists_response, "Queue should not exist");

    // Return the test result
    test_result?;

    Ok(())
}

/// Sets metadata for a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_set_metadata(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-queue-metadata").await?;
    queue_client.create_if_not_exists(None).await?;

    let test_result = async {
        // Set metadata for the queue

        let metadata_options = Some(QueueClientSetMetadataOptions {
            metadata: Some(HashMap::from([
                ("key1".to_string(), "value1".to_string()),
                ("key2".to_string(), "value2".to_string()),
            ])),
            ..Default::default()
        });

        let response = queue_client.set_metadata(metadata_options).await?;

        assert_successful_response(&response);

        Ok::<(), azure_core::Error>(())
    }
    .await;

    queue_client.delete(None).await?;

    // Return the test result
    test_result?;
    Ok(())
}

/// Clears all messages from a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_clear_messages(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-clear-messages").await?;

    // Create a queue if it does not exist
    queue_client.create_if_not_exists(None).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        // Clear messages from the queue
        let response = queue_client.clear(None).await?;
        assert_successful_response(&response);

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(None).await.unwrap();

    // Return the test result
    test_result?;
    Ok(())
}

#[recorded::test]
async fn test_delete_message(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-delete-message").await?;

    // Create a queue if it does not exist
    queue_client.create_if_not_exists(None).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        // Send a message to the queue
        // Note: The message ID and pop receipt are required for deletion, so we need to capture them.
        let enqueue_message_response = queue_client
            .enqueue_message(
                "Example message created from Rust, ready for deletion",
                None,
            )
            .await?;

        let enqueued_message = enqueue_message_response.into_body().await?.unwrap();

        let delete_response = queue_client
            .delete_message(
                &enqueued_message.message_id.unwrap(),
                &enqueued_message.pop_receipt.unwrap(),
                None,
            )
            .await?;
        assert_successful_response(&delete_response);
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(None).await.unwrap();

    // Return the test result
    test_result?;
    Ok(())
}

#[recorded::test]
async fn test_update_message(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-update-message").await?;

    // Create a queue if it does not exist
    queue_client.create_if_not_exists(None).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        // Enqueue a message to the queue
        let enqueue_message_response = queue_client
            .enqueue_message("Example message created from Rust, ready for update", None)
            .await?;

        let enqueued_message = enqueue_message_response.into_body().await?.unwrap();

        // Update the message in the queue
        let option = Some(QueueClientUpdateOptions {
            queue_message: Some(RequestContent::from(
                quick_xml::se::to_string(&QueueMessage {
                    message_text: Some("Updated message text from Rust".to_string()),
                })
                .unwrap()
                .into_bytes(),
            )),
            ..Default::default()
        });

        // Update the message in the queue
        let update_response = queue_client
            .update_message(
                &enqueued_message.message_id.unwrap(),
                &enqueued_message.pop_receipt.unwrap(),
                10,
                option,
            )
            .await?;
        assert!(
            update_response.status().is_success(),
            "Expected successful status code, got {}",
            update_response.status(),
        );
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(None).await.unwrap();

    // Return the test result
    test_result?;
    Ok(())
}

/// Attempts to peek the first message from an empty queue in Azure Storage Queue service.
#[recorded::test]
async fn test_peek_message_empty(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-peek-message-empty").await?;

    // Setup test queue with messages
    queue_client.create_if_not_exists(None).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        let response = queue_client.peek_message(None).await?;
        assert_successful_response(&response);

        let message = response.into_body().await?;

        assert!(
            message.is_none(),
            "Expected to receive no message, but got Some"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(None).await.unwrap();

    test_result
}

/// Peeks the first message from a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_peek_message(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-peek-message").await?;
    let test_messages = ["Message 1", "Message 2"];

    // Setup test queue with messages
    setup_test_queue_with_messages(&queue_client, &test_messages).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        peek_and_assert(&queue_client, &test_messages, 1, None).await?;

        // The messages should not have been dequeued, so we can peek again
        // and expect to receive the same message again.
        peek_and_assert(&queue_client, &test_messages, 1, None).await?;

        Ok(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(None).await.unwrap();

    test_result
}

/// Receives all messages from a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_peek_messages(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-peek-messages").await?;
    let test_messages = ["Message 1", "Message 2"];

    // Setup test queue with messages
    setup_test_queue_with_messages(&queue_client, &test_messages).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        let options = Some(QueueClientPeekOptions {
            number_of_messages: Some(10),
            ..Default::default()
        });

        peek_and_assert(
            &queue_client,
            &test_messages,
            test_messages.len(),
            options.clone(),
        )
        .await?;

        // The messages should not have been dequeued, so we can peek again
        // and expect to receive both messages this time.
        peek_and_assert(
            &queue_client,
            &test_messages,
            test_messages.len(),
            options.clone(),
        )
        .await?;

        Ok(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(None).await.unwrap();

    test_result
}

/// Attempts to receive the first message from an empty queue in Azure Storage Queue service.
#[recorded::test]
async fn test_dequeue_message_empty(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-dequeue-message-empty").await?;

    // Setup test queue with messages
    queue_client.create_if_not_exists(None).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        let response = queue_client.dequeue_message(None).await?;
        assert_successful_response(&response);

        let message = response.into_body().await?;

        assert!(
            message.is_none(),
            "Expected to dequeue no message, but got Some"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(None).await.unwrap();

    test_result
}

/// Dequeues the first message from a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_dequeue_message(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-dequeue-message").await?;
    let test_messages = ["Message 1", "Message 2"];

    // Setup test queue with messages
    setup_test_queue_with_messages(&queue_client, &test_messages).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        let response = queue_client.dequeue_message(None).await?;
        assert_successful_response(&response);

        let message = response.into_body().await?;

        assert!(
            message.is_some(),
            "Expected to dequeue a message, but got None"
        );

        let message = message.unwrap();
        assert_message_text(message.message_text.clone(), test_messages[0], 0);

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(None).await.unwrap();

    test_result
}

/// Dequeues all messages from a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_dequeue_messages(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-dequeue-messages").await?;
    let test_messages = ["Message 1", "Message 2"];

    // Setup test queue with messages
    setup_test_queue_with_messages(&queue_client, &test_messages).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        let options = Some(QueueClientDequeueOptions {
            number_of_messages: Some(10),
            ..Default::default()
        });

        let response = queue_client.dequeue_messages(options).await?;
        assert_successful_response(&response);

        let messages = response.into_body().await?;
        let messages = messages.items.unwrap();

        assert_eq!(
            messages.len(),
            test_messages.len(),
            "Expected to dequeue {} messages, got {}",
            test_messages.len(),
            messages.len()
        );

        // Verify messages are received in order
        for (i, message) in messages.iter().enumerate() {
            assert_message_text(message.message_text.clone(), test_messages[i], i);
        }

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(None).await.unwrap();

    test_result
}

/// Gets the access policies for a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_get_access_policies(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-get-access-policies").await?;

    // Setup test queue with messages
    queue_client.create(None).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        let response = queue_client.get_access_policy(None).await?;
        assert_successful_response(&response);
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(None).await.unwrap();

    test_result
}

/// Sets the access policies for a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_set_access_policies(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-set-access-policies").await?;

    // Setup test queue with messages
    queue_client.create(None).await?;

    // Get the current access policies to set them back after the test
    let result = queue_client.get_access_policy(None).await?;

    let signed_identifiers = result.into_body().await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        let response = queue_client
            .set_access_policy(signed_identifiers.try_into()?, None)
            .await?;
        assert_successful_response(&response);
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(None).await.unwrap();

    test_result
}

/// Returns an instance of a QueueClient.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
pub async fn get_queue_client(recording: &Recording, queue_name: &str) -> Result<QueueClient> {
    let (options, endpoint) = recorded_test_setup(recording);
    let queue_client_options = QueueClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    let queue_client = QueueClient::new(
        &endpoint,
        queue_name,
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

/// Helper function to set up a test queue with messages
async fn setup_test_queue_with_messages(
    queue_client: &QueueClient,
    messages: &[&str],
) -> Result<()> {
    queue_client.create_if_not_exists(None).await?;
    for message in messages {
        queue_client.enqueue_message(message, None).await?;
    }
    Ok(())
}

/// Helper function to verify a successful response
fn assert_successful_response<T, F>(response: &Response<T, F>) {
    assert!(
        response.status().is_success(),
        "Expected successful status code, got {}",
        response.status()
    );
}

/// Helper function to verify message contents
fn assert_message_text(actual: Option<String>, expected: &str, message_index: usize) {
    let actual = actual.unwrap();
    assert!(
        actual == expected,
        "Message at index {} has wrong text. Expected '{}', got '{}'",
        message_index,
        expected,
        actual
    );
}

async fn peek_and_assert<'a>(
    queue_client: &QueueClient,
    expected_messages: &[&str],
    count: usize,
    options: Option<QueueClientPeekOptions<'a>>,
) -> Result<()> {
    // Peek the messages in the queue
    let response = queue_client.peek_messages(options).await?;
    assert_successful_response(&response);

    let messages = response.into_body().await?;
    let messages = messages.items.unwrap();

    assert_eq!(
        messages.len(),
        count,
        "Expected to receive exactly {} messages, got {}",
        count,
        messages.len()
    );

    // Assert each message matches the expected text
    for (i, message) in messages.iter().enumerate() {
        assert_message_text(message.message_text.clone(), expected_messages[i], i);
    }

    Ok(())
}

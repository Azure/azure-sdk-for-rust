use azure_core::http::{
    RequestContent, {ClientOptions, Response},
};
use azure_core::Result;
use azure_core_test::{recorded, Recording, TestContext};
use azure_storage_queue::AzureQueueStorageMessageIdOperationsClientUpdateOptions;
use azure_storage_queue::{
    clients::{
        AzureQueueStorageClientOptions, AzureQueueStorageMessagesOperationsClientDequeueOptions,
        QueueClient, QueueMessage,
    },
    ListOfEnqueuedMessage,
};
use quick_xml::de::from_str;
use std::option::Option;

/// Creates a new queue under the given account.
#[recorded::test]
async fn test_create_queue(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await;

    let queue_name = "test-create-queue";
    let response = queue_client?.create(queue_name, None).await?;

    assert_successful_response(&response);

    Ok(())
}

/// Enqueues a message to the specified queue.
#[recorded::test]
async fn test_send_message(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = "test-send-message";
    queue_client.create(queue_name, None).await?;

    let test_result = async {
        let response = queue_client
            .send_message(queue_name, "queue-message", None)
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
    queue_client.delete(queue_name, None).await.unwrap();

    test_result?;

    Ok(())
}

#[recorded::test]
/// Tests the creation of a queue in Azure Storage Queue service, ensuring it does not fail if the queue already exists.
async fn test_create_queue_if_not_exists(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = "test-create-queue-if-not-exists";

    let test_result = async {
        // First, create the queue
        let response = queue_client.create(queue_name, None).await?;
        assert_successful_response(&response);

        // Now, try to create the same queue again
        let response = queue_client.create_if_not_exists(queue_name, None).await?;
        assert_successful_response(&response);

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(queue_name, None).await.unwrap();

    test_result?;

    Ok(())
}

/// Tests the deletion of a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_delete_queue(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = "test-delete-queue";

    queue_client.create_if_not_exists(queue_name, None).await?;

    let response = queue_client.delete(queue_name, None).await?;

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
    let queue_name = "test-delete-queue-if-exists";

    // First, create the queue
    let response = queue_client.create(queue_name, None).await?;
    assert_successful_response(&response);

    // Now, try to delete the same queue
    let response = queue_client.delete_if_exists(queue_name, None).await?;
    assert_successful_response(&response);

    // Try to delete a non-existent queue
    let non_existent_response = queue_client
        .delete_if_exists("non-existent-queue", None)
        .await?;
    assert_successful_response(&non_existent_response);

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
async fn test_queue_exists(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = "test-queue-exists";

    let test_result = async {
        // Create a queue if it does not exist
        queue_client.create_if_not_exists(queue_name, None).await?;

        // Check if the queue exists
        let exists_response = queue_client.exists(queue_name).await?;
        assert!(exists_response, "Queue should exist");

        Ok::<(), azure_core::Error>(())
    }
    .await;

    queue_client.delete(queue_name, None).await?;

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
    let queue_name = "test-queue-metadata";
    queue_client.create_if_not_exists(queue_name, None).await?;

    let test_result = async {
        // Set metadata for the queue
        let metadata = Some(
            vec![("key1", "value1"), ("key2", "value2")]
                .into_iter()
                .collect(),
        );
        let response = queue_client.set_metadata(queue_name, metadata).await?;

        assert_successful_response(&response);

        Ok::<(), azure_core::Error>(())
    }
    .await;

    queue_client.delete(queue_name, None).await?;

    // Return the test result
    test_result?;
    Ok(())
}

/// Deletes all messages from a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_delete_messages(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = "test-delete-messages";

    // Create a queue if it does not exist
    queue_client.create_if_not_exists(queue_name, None).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        // Delete messages from the queue
        let response = queue_client.delete_messages(queue_name).await?;
        assert_successful_response(&response);

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(queue_name, None).await.unwrap();

    // Return the test result
    test_result?;
    Ok(())
}

#[recorded::test]
async fn test_delete_message(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = "test-delete-message";

    // Create a queue if it does not exist
    queue_client.create_if_not_exists(queue_name, None).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        // Send a message to the queue
        // Note: The message ID and pop receipt are required for deletion, so we need to capture them.
        let send_message_response = queue_client
            .send_message(
                queue_name,
                "Example message created from Rust, ready for deletion",
                None,
            )
            .await?;

        let (_status_code, _headers, properties) = send_message_response.deconstruct();
        let xml = properties.collect_string().await?;
        let queue_messages_list: ListOfEnqueuedMessage = from_str(&xml).unwrap();

        // Get the first message from the vector
        let enqueued_message = queue_messages_list
            .value
            .as_ref()
            .and_then(|msgs| msgs.first())
            .ok_or("No messages found in response")
            .unwrap();

        let pop_receipt = enqueued_message
            .pop_receipt
            .as_ref()
            .ok_or("PopReceipt not found")
            .unwrap();
        let message_id = enqueued_message
            .message_id
            .as_ref()
            .ok_or("MessageId not found")
            .unwrap();

        let delete_response = queue_client
            .delete_message(queue_name, message_id, pop_receipt, None)
            .await?;
        assert_successful_response(&delete_response);
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(queue_name, None).await.unwrap();

    // Return the test result
    test_result?;
    Ok(())
}

#[recorded::test]
async fn test_update_message(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = "test-update-message";

    // Create a queue if it does not exist
    queue_client.create_if_not_exists(queue_name, None).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        // Send a message to the queue
        let send_message_response = queue_client
            .send_message(
                queue_name,
                "Example message created from Rust, ready for update",
                None,
            )
            .await?;

        let (_status_code, _headers, properties) = send_message_response.deconstruct();
        let xml = properties.collect_string().await?;
        let queue_messages_list: ListOfEnqueuedMessage = from_str(&xml).unwrap();

        // Get the first message from the vector
        let enqueued_message = queue_messages_list
            .value
            .as_ref()
            .and_then(|msgs| msgs.first())
            .ok_or("No messages found in response")
            .unwrap();

        let pop_receipt = enqueued_message
            .pop_receipt
            .as_ref()
            .ok_or("PopReceipt not found")
            .unwrap();
        let message_id = enqueued_message
            .message_id
            .as_ref()
            .ok_or("MessageId not found")
            .unwrap();

        // Update the message in the queue
        let option = Some(AzureQueueStorageMessageIdOperationsClientUpdateOptions {
            queue_message: Some(RequestContent::from(
                quick_xml::se::to_string(&QueueMessage {
                    message_text: Some("Updated message text from Rust".to_string()),
                })
                .unwrap()
                .into_bytes(),
            )),
            request_id: Some(message_id.clone()),
            ..Default::default()
        });

        // Update the message in the queue
        let update_response = queue_client
            .update_message(queue_name, message_id, pop_receipt, 10, option)
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
    queue_client.delete(queue_name, None).await.unwrap();

    // Return the test result
    test_result?;
    Ok(())
}

/// Receives the first message from a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_receive_message(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = "test-receive-message";
    let test_messages = ["Message 1", "Message 2"];

    // Setup test queue with messages
    setup_test_queue_for_receive_message(&queue_client, queue_name, &test_messages).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        let response = queue_client.receive_message(queue_name, None).await?;
        assert_successful_response(&response);

        let messages = response.into_body().await?;
        let messages = messages.value.unwrap();

        assert_eq!(
            messages.len(),
            1,
            "Expected to receive exactly 1 message, got {}",
            messages.len()
        );

        let message = messages.first().unwrap();
        assert_message_text(message.message_text.clone(), test_messages[0], 0);

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(queue_name, None).await.unwrap();

    test_result
}

/// Receives all messages from a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_receive_messages(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording).await?;
    let queue_name = "test-receive-messages";
    let test_messages = ["Message 1", "Message 2"];

    // Setup test queue with messages
    setup_test_queue_for_receive_message(&queue_client, queue_name, &test_messages).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        let options = Some(AzureQueueStorageMessagesOperationsClientDequeueOptions {
            number_of_messages: Some(10),
            ..Default::default()
        });

        let response = queue_client.receive_messages(queue_name, options).await?;
        assert_successful_response(&response);

        let messages = response.into_body().await?;
        let messages = messages.value.unwrap();

        assert_eq!(
            messages.len(),
            test_messages.len(),
            "Expected to receive {} messages, got {}",
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
    queue_client.delete(queue_name, None).await.unwrap();

    test_result
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

/// Helper function to set up a test queue with messages
async fn setup_test_queue_for_receive_message(
    queue_client: &QueueClient,
    queue_name: &str,
    messages: &[&str],
) -> Result<()> {
    queue_client.create_if_not_exists(queue_name, None).await?;
    for message in messages {
        queue_client.send_message(queue_name, message, None).await?;
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

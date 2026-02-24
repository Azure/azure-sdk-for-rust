// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{ClientOptions, Response};
use azure_core::Result;
use azure_core_test::{recorded, Recording, TestContext, TestMode};
use azure_storage_queue::{
    models::{
        AccessPolicy, QueueClientPeekMessagesOptions, QueueClientReceiveMessagesOptions,
        QueueClientUpdateOptions, QueueMessage, SignedIdentifier, SignedIdentifiers,
    },
    QueueClient, QueueClientOptions,
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

/// Sends a message to the specified queue.
#[recorded::test]
async fn test_send_message(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-send-message").await?;
    queue_client.create(None).await?;
    let queue_message = QueueMessage {
        message_text: Some("send_message".to_string()),
    };

    let test_result = async {
        let response = queue_client
            .send_message(queue_message.try_into()?, None)
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

/// Tests the deletion of a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_delete_queue(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-delete-queue").await?;

    queue_client.create(None).await?;

    let response = queue_client.delete(None).await?;

    assert!(
        response.status() == 204,
        "Expected status code 204, got {}",
        response.status(),
    );
    Ok(())
}

/// Checks if a queue exists in the Azure Storage Queue service.
#[recorded::test]
async fn test_queue_exists(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-queue-exists").await?;
    queue_client.create(None).await?;

    let test_result = async {
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
    queue_client.create(None).await?;

    let test_result = async {
        // Set metadata for the queue

        let response = queue_client
            .set_metadata(
                &HashMap::from([
                    ("key1".to_string(), "value1".to_string()),
                    ("key2".to_string(), "value2".to_string()),
                ]),
                None,
            )
            .await?;

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
    queue_client.create(None).await?;

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
    queue_client.create(None).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        // Send a message to the queue
        // Note: The message ID and pop receipt are required for deletion, so we need to capture them.
        let sent_message_response = queue_client
            .send_message(
                QueueMessage {
                    message_text: Some(
                        "Example message created from Rust, ready for deletion".to_string(),
                    ),
                }
                .try_into()?,
                None,
            )
            .await?;

        let send_message = sent_message_response.into_model()?;

        let delete_response = queue_client
            .delete_message(
                &send_message.message_id.unwrap(),
                &send_message.pop_receipt.unwrap(),
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
    queue_client.create(None).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        // Send a message to the queue
        let send_message_response = queue_client
            .send_message(
                QueueMessage {
                    message_text: Some(
                        "Example message created from Rust, ready for update".to_string(),
                    ),
                }
                .try_into()?,
                None,
            )
            .await?;

        let sent_message = send_message_response.into_model()?;

        // Update the message in the queue
        let option = Some(QueueClientUpdateOptions {
            queue_message: Some(
                QueueMessage {
                    message_text: Some("Updated message text from Rust".to_string()),
                }
                .try_into()?,
            ),
            ..Default::default()
        });

        // Update the message in the queue
        let update_response = queue_client
            .update_message(
                &sent_message.message_id.unwrap(),
                &sent_message.pop_receipt.unwrap(),
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

/// Attempts to peek messages from an empty queue in Azure Storage Queue service.
#[recorded::test]
async fn test_peek_messages_empty(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-peek-messages-empty").await?;
    queue_client.create(None).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        let response = queue_client.peek_messages(None).await?;
        assert_successful_response(&response);

        let messages = response.into_model()?;

        assert!(
            messages.items.is_none(),
            "Expected to receive no messages, but got Some"
        );

        Ok::<(), azure_core::Error>(())
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
        let options = Some(QueueClientPeekMessagesOptions {
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

/// Attempts to receive messages from an empty queue in Azure Storage Queue service.
#[recorded::test]
async fn test_receive_messages_empty(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-receive-messages-empty").await?;
    queue_client.create(None).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        let response = queue_client.receive_messages(None).await?;
        assert_successful_response(&response);

        let messages = response.into_model()?;

        assert!(
            messages.items.is_none(),
            "Expected to dequeue no messages, but got Some"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Clean up by deleting the queue - this always executes
    queue_client.delete(None).await.unwrap();

    test_result
}

/// Dequeues all messages from a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_receive_messages(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-receive-messages").await?;
    let test_messages = ["Message 1", "Message 2"];

    // Setup test queue with messages
    setup_test_queue_with_messages(&queue_client, &test_messages).await?;

    // Run the test logic and ensure cleanup always happens
    let test_result = async {
        let options = Some(QueueClientReceiveMessagesOptions {
            number_of_messages: Some(10),
            ..Default::default()
        });

        let response = queue_client.receive_messages(options).await?;
        assert_successful_response(&response);

        let messages = response.into_model()?;
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

/// Sets an access policy on a queue and then gets it to verify.
#[recorded::test]
async fn test_queue_access_policy(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, "test-set-get-acl").await?;
    queue_client.create(None).await?;

    let test_result = async {
        // Verify the queue starts with no access policy.
        let response = queue_client.get_access_policy(None).await?;
        assert_successful_response(&response);

        let acl = response.into_model()?;
        assert!(
            acl.items.is_none(),
            "Expected no signed identifiers, got {:?}",
            acl.items
        );

        let policy = SignedIdentifiers {
            items: Some(vec![SignedIdentifier {
                id: Some("policy1".to_string()),
                access_policy: Some(AccessPolicy {
                    permission: Some("raup".to_string()),
                    ..Default::default()
                }),
            }]),
        };

        let set_response = queue_client
            .set_access_policy(policy.try_into()?, None)
            .await?;

        assert_successful_response(&set_response);

        // Sleep in live mode to allow access policies to take effect.
        if recording.test_mode() == TestMode::Live || recording.test_mode() == TestMode::Record {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }

        let get_response = queue_client.get_access_policy(None).await?;
        assert_successful_response(&get_response);

        let acl = get_response.into_model()?;
        let items = acl.items.expect("Expected signed identifiers");
        assert_eq!(items.len(), 1, "Expected exactly one signed identifier");
        assert_eq!(items[0].id.as_deref(), Some("policy1"));
        let ap = items[0]
            .access_policy
            .as_ref()
            .expect("Expected access policy");
        assert_eq!(ap.permission.as_deref(), Some("raup"));

        Ok::<(), azure_core::Error>(())
    }
    .await;

    queue_client.delete(None).await.unwrap();

    test_result?;

    Ok(())
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

    QueueClient::new(
        &endpoint,
        queue_name,
        Some(recording.credential()),
        Option::Some(queue_client_options),
    )
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
        recording.var("AZURE_STORAGE_ACCOUNT_NAME", None).as_str()
    );

    (client_options, endpoint)
}

/// Helper function to set up a test queue with messages
async fn setup_test_queue_with_messages(
    queue_client: &QueueClient,
    messages: &[&str],
) -> Result<()> {
    queue_client.create(None).await?;
    for message in messages {
        let queue_message = QueueMessage {
            message_text: Some(message.to_string()),
        };
        queue_client
            .send_message(queue_message.try_into()?, None)
            .await?;
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

async fn peek_and_assert(
    queue_client: &QueueClient,
    expected_messages: &[&str],
    count: usize,
    options: Option<QueueClientPeekMessagesOptions<'_>>,
) -> Result<()> {
    // Peek the messages in the queue
    let response = queue_client.peek_messages(options).await?;
    assert_successful_response(&response);

    let messages = response.into_model()?;
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

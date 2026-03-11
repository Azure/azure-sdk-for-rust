// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod common;

use azure_core::{http::StatusCode, Result};
use azure_core_test::{recorded, Recording, TestContext, TestMode};
use azure_storage_queue::{
    models::{
        AccessPolicy, QueueClientCreateOptions, QueueClientGetPropertiesResultHeaders,
        QueueClientPeekMessagesOptions, QueueClientReceiveMessagesOptions,
        QueueClientSendMessageOptions, QueueClientUpdateMessageOptions, QueueMessage,
        SignedIdentifier, SignedIdentifiers,
    },
    QueueClient, QueueClientOptions,
};
use common::{assert_successful_response, get_queue_name, recorded_test_setup};

use std::collections::HashMap;

/// Creates a new queue under the given account.
#[recorded::test]
async fn test_create_queue(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Act
    let response = queue_client.create(None).await?;
    let test_result = async {
        // Assert
        assert_successful_response(&response);
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;

    Ok(())
}

/// Sends a message to the specified queue.
#[recorded::test]
async fn test_send_message(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;
    let queue_message = QueueMessage {
        message_text: Some("send_message".to_string()),
    };

    let test_result = async {
        // Act
        let response = queue_client
            .send_message(queue_message.try_into()?, None)
            .await?;

        // Assert
        assert!(
            response.status() == StatusCode::Created,
            "Expected status code 201, got {}",
            response.status(),
        );
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;

    Ok(())
}

/// Verifies that `send_message` with `visibility_timeout` enqueues a message that is
/// initially hidden and therefore not yet visible in peek results.
#[recorded::test]
async fn test_send_message_with_visibility_timeout(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        let options = Some(QueueClientSendMessageOptions {
            visibility_timeout: Some(30),    // hidden for 30 seconds after enqueue
            message_time_to_live: Some(300), // expires after 5 minutes
            ..Default::default()
        });

        // Act
        let response = queue_client
            .send_message(
                QueueMessage {
                    message_text: Some("Message with options".to_string()),
                }
                .try_into()?,
                options,
            )
            .await?;

        // Assert — message was enqueued successfully
        assert!(
            response.status() == StatusCode::Created,
            "Expected status code 201, got {}",
            response.status(),
        );

        // Assert — message is not yet visible due to visibility_timeout=30
        let peek_response = queue_client.peek_messages(None).await?;
        let peek_result = peek_response.into_model()?;
        assert!(
            peek_result.items.is_none(),
            "Expected no visible messages because message should be hidden by visibility_timeout"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;

    Ok(())
}

/// Verifies that `receive_messages` with `visibility_timeout` and `number_of_messages`
/// options correctly receives messages using both options.
#[recorded::test]
async fn test_receive_messages_with_options(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Arrange — send a message
        queue_client
            .send_message(
                QueueMessage {
                    message_text: Some("Receive with options".to_string()),
                }
                .try_into()?,
                None,
            )
            .await?;

        let options = Some(QueueClientReceiveMessagesOptions {
            number_of_messages: Some(1),
            visibility_timeout: Some(60), // hide for 60 seconds after receive
            ..Default::default()
        });

        // Act
        let response = queue_client.receive_messages(options).await?;
        assert_successful_response(&response);

        let messages = response.into_model()?.items.expect("Expected messages");

        // Assert — received the expected message with correct text
        assert_eq!(messages.len(), 1, "Expected exactly one message");
        assert_eq!(
            messages[0].message_text.as_deref(),
            Some("Receive with options"),
            "Message text did not match"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;

    Ok(())
}

/// Tests the deletion of a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_delete_queue(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    // Act
    let response = queue_client.delete(None).await?;

    // Assert
    assert!(
        response.status() == StatusCode::NoContent,
        "Expected status code 204, got {}",
        response.status(),
    );
    Ok(())
}

/// Checks if a queue exists in the Azure Storage Queue service.
#[recorded::test]
async fn test_queue_exists(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
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
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Act
        let metadata = HashMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ]);
        let response = queue_client.set_metadata(&metadata, None).await?;

        // Assert — set succeeded
        assert_successful_response(&response);

        // Assert — read back and verify metadata was stored
        let props = queue_client.get_properties(None).await?;
        let stored = props.metadata()?;
        assert_eq!(
            stored.get("key1").map(String::as_str),
            Some("value1"),
            "Expected key1=value1 in metadata"
        );
        assert_eq!(
            stored.get("key2").map(String::as_str),
            Some("value2"),
            "Expected key2=value2 in metadata"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    queue_client.delete(None).await?;

    // Return the test result
    test_result?;
    Ok(())
}

/// Verifies that `get_properties` returns queue properties including an accurate message count.
#[recorded::test]
async fn test_get_queue_properties(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Act
        let props = queue_client.get_properties(None).await?;

        // Assert — a freshly created queue has zero messages
        assert_eq!(
            props.approximate_messages_count()?,
            Some(0),
            "Expected approximate_messages_count to be 0 for empty queue"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;
    Ok(())
}

/// Verifies that `create` with metadata stores and returns the metadata in `get_properties`.
#[recorded::test]
async fn test_create_queue_with_metadata(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    let options = Some(QueueClientCreateOptions {
        metadata: Some(HashMap::from([
            ("env".to_string(), "test".to_string()),
            ("owner".to_string(), "rust-sdk".to_string()),
        ])),
        ..Default::default()
    });

    // Act
    queue_client.create(options).await?;

    let test_result = async {
        // Assert — metadata is readable back via get_properties
        let props = queue_client.get_properties(None).await?;
        let stored = props.metadata()?;
        assert_eq!(
            stored.get("env").map(String::as_str),
            Some("test"),
            "Expected metadata key 'env' = 'test'"
        );
        assert_eq!(
            stored.get("owner").map(String::as_str),
            Some("rust-sdk"),
            "Expected metadata key 'owner' = 'rust-sdk'"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;
    Ok(())
}

/// Verifies that `get_properties` returns an `approximate_messages_count` of at least 1
/// after a message has been sent.
#[recorded::test]
async fn test_message_count_after_send(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Arrange — send a message
        queue_client
            .send_message(
                QueueMessage {
                    message_text: Some("hello".to_string()),
                }
                .try_into()?,
                None,
            )
            .await?;

        // Act
        let props = queue_client.get_properties(None).await?;

        // Assert — at least one message is known to be in the queue
        let count = props.approximate_messages_count()?.unwrap_or(0);
        assert!(
            count >= 1,
            "Expected approximate_messages_count >= 1 after sending a message, got {count}"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;
    Ok(())
}

/// Verifies that Unicode text in a message body round-trips correctly through send and receive.
#[recorded::test]
async fn test_unicode_message_content(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        let unicode_text = "こんにちは Azure 🦀 — Ünïcödé";

        // Act — send
        queue_client
            .send_message(
                QueueMessage {
                    message_text: Some(unicode_text.to_string()),
                }
                .try_into()?,
                None,
            )
            .await?;

        // Act — receive
        let received = queue_client.receive_messages(None).await?;
        let messages = received.into_model()?.items.expect("Expected a message");

        // Assert — text is preserved exactly
        assert_eq!(messages.len(), 1, "Expected exactly one message");
        assert_eq!(
            messages[0].message_text.as_deref(),
            Some(unicode_text),
            "Unicode message text did not round-trip correctly"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;
    Ok(())
}

/// Clears all messages from a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_clear_messages(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange — create a queue and send messages so we have something to clear
    queue_client.create(None).await?;

    let test_result = async {
        // Arrange — populate the queue
        for msg in ["message1", "message2", "message3"] {
            queue_client
                .send_message(
                    QueueMessage {
                        message_text: Some(msg.to_string()),
                    }
                    .try_into()?,
                    None,
                )
                .await?;
        }

        // Act
        let response = queue_client.clear(None).await?;

        // Assert — clear succeeded
        assert_successful_response(&response);

        // Assert — queue is now empty
        let peek_response = queue_client.peek_messages(None).await?;
        let peeked = peek_response.into_model()?;
        assert!(
            peeked.items.is_none(),
            "Expected queue to be empty after clear, but found messages"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    // Return the test result
    test_result?;
    Ok(())
}

#[recorded::test]
async fn test_delete_message(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Arrange — send a message to capture pop receipt
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

        // Act
        let delete_response = queue_client
            .delete_message(
                &send_message.message_id.clone().unwrap(),
                &send_message.pop_receipt.clone().unwrap(),
                None,
            )
            .await?;

        // Assert — delete succeeded
        assert_successful_response(&delete_response);

        // Assert — queue is now empty (no messages remain)
        let received = queue_client.receive_messages(None).await?;
        let remaining = received.into_model()?;
        assert!(
            remaining.items.is_none(),
            "Expected queue to be empty after deleting the only message"
        );
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;
    Ok(())
}

#[recorded::test]
async fn test_update_message(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Arrange — send a message to capture pop receipt
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
        let option = Some(QueueClientUpdateMessageOptions {
            queue_message: Some(
                QueueMessage {
                    message_text: Some("Updated message text from Rust".to_string()),
                }
                .try_into()?,
            ),
            ..Default::default()
        });

        // Act — update with visibility_timeout=0 so message is immediately receivable
        let update_response = queue_client
            .update_message(
                &sent_message.message_id.clone().unwrap(),
                &sent_message.pop_receipt.clone().unwrap(),
                0,
                option,
            )
            .await?;

        // Assert — update succeeded
        assert!(
            update_response.status().is_success(),
            "Expected successful status code, got {}",
            update_response.status(),
        );

        // Assert — receive and verify the updated text
        let received = queue_client.receive_messages(None).await?;
        let messages = received.into_model()?.items.expect("Expected one message");
        assert_eq!(messages.len(), 1, "Expected exactly one message");
        assert_eq!(
            messages[0].message_text.as_deref(),
            Some("Updated message text from Rust"),
            "Message text was not updated"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    // Return the test result
    test_result?;
    Ok(())
}

/// Attempts to peek messages from an empty queue in Azure Storage Queue service.
#[recorded::test]
async fn test_peek_messages_empty(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

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

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result
}

/// Receives all messages from a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_peek_messages(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;
    let test_messages = ["Message 1", "Message 2"];

    // Arrange
    setup_test_queue_with_messages(&queue_client, &test_messages).await?;

    // Act & Assert
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

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result
}

/// Attempts to receive messages from an empty queue in Azure Storage Queue service.
#[recorded::test]
async fn test_receive_messages_empty(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

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

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result
}

/// Dequeues all messages from a queue in Azure Storage Queue service.
#[recorded::test]
async fn test_receive_messages(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;
    let test_messages = ["Message 1", "Message 2"];

    // Arrange
    setup_test_queue_with_messages(&queue_client, &test_messages).await?;

    // Act & Assert
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

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result
}

/// Sets an access policy on a queue and then gets it to verify.
#[recorded::test]
async fn test_queue_access_policy(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Assert — queue starts with no access policy
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

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;

    Ok(())
}

/// Verifies that `get_properties` returns 404 Not Found for a queue that does not exist.
#[recorded::test]
async fn test_get_properties_queue_not_found(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Act
    let err = queue_client.get_properties(None).await.err().unwrap();

    // Assert
    assert_eq!(
        err.http_status(),
        Some(StatusCode::NotFound),
        "Expected 404 Not Found for get_properties on non-existent queue, got {:?}",
        err.http_status()
    );
    Ok(())
}

/// Verifies that `set_metadata` returns 404 Not Found for a queue that does not exist.
#[recorded::test]
async fn test_set_metadata_queue_not_found(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Act
    let err = queue_client
        .set_metadata(
            &HashMap::from([("key".to_string(), "value".to_string())]),
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_eq!(
        err.http_status(),
        Some(StatusCode::NotFound),
        "Expected 404 Not Found for set_metadata on non-existent queue, got {:?}",
        err.http_status()
    );
    Ok(())
}

/// Verifies that `receive_messages` returns 404 Not Found for a queue that does not exist.
#[recorded::test]
async fn test_receive_messages_queue_not_found(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Act
    let err = queue_client.receive_messages(None).await.err().unwrap();

    // Assert
    assert_eq!(
        err.http_status(),
        Some(StatusCode::NotFound),
        "Expected 404 Not Found for receive_messages on non-existent queue, got {:?}",
        err.http_status()
    );
    Ok(())
}

/// Verifies that `peek_messages` returns 404 Not Found for a queue that does not exist.
#[recorded::test]
async fn test_peek_messages_queue_not_found(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Act
    let err = queue_client.peek_messages(None).await.err().unwrap();

    // Assert
    assert_eq!(
        err.http_status(),
        Some(StatusCode::NotFound),
        "Expected 404 Not Found for peek_messages on non-existent queue, got {:?}",
        err.http_status()
    );
    Ok(())
}

/// Verifies that `delete` returns 404 Not Found for a queue that does not exist.
#[recorded::test]
async fn test_delete_queue_not_found(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Act
    let err = queue_client.delete(None).await.err().unwrap();

    // Assert
    assert_eq!(
        err.http_status(),
        Some(StatusCode::NotFound),
        "Expected 404 Not Found for delete on non-existent queue, got {:?}",
        err.http_status()
    );
    Ok(())
}

/// Verifies that `get_access_policy` returns 404 Not Found for a queue that does not exist.
#[recorded::test]
async fn test_get_access_policy_queue_not_found(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Act
    let err = queue_client.get_access_policy(None).await.err().unwrap();

    // Assert
    assert_eq!(
        err.http_status(),
        Some(StatusCode::NotFound),
        "Expected 404 Not Found for get_access_policy on non-existent queue, got {:?}",
        err.http_status()
    );
    Ok(())
}

/// Verifies that `set_access_policy` returns 404 Not Found for a queue that does not exist.
#[recorded::test]
async fn test_set_access_policy_queue_not_found(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Act
    let err = queue_client
        .set_access_policy(SignedIdentifiers { items: None }.try_into()?, None)
        .await
        .err()
        .unwrap();

    // Assert
    assert_eq!(
        err.http_status(),
        Some(StatusCode::NotFound),
        "Expected 404 Not Found for set_access_policy on non-existent queue, got {:?}",
        err.http_status()
    );
    Ok(())
}

/// Returns an instance of a QueueClient.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
pub async fn get_queue_client(recording: &Recording, queue_name: &str) -> Result<QueueClient> {
    let (options, endpoint, _) = recorded_test_setup(recording);
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

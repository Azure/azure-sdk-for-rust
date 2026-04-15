// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod common;

use azure_core::{
    error::ErrorKind,
    http::{
        policies::{Policy, PolicyResult},
        Context, FixedRetryOptions, Response, RetryOptions, StatusCode,
    },
    time::{parse_rfc3339, to_rfc3339, Duration, OffsetDateTime},
    Result,
};
use azure_core_test::{recorded, Recording, TestContext, TestMode, VarOptions};
use azure_storage_queue::{
    models::{
        AccessPolicy, QueueClientCreateOptions, QueueClientGetPropertiesResultHeaders,
        QueueClientPeekMessagesOptions, QueueClientReceiveMessagesOptions,
        QueueClientSendMessageOptions, QueueClientUpdateMessageOptions, QueueMessage, SentMessage,
        SignedIdentifier, SignedIdentifiers,
    },
    QueueClient, QueueClientOptions,
};
use common::{get_queue_name, recorded_test_setup};

use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

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

/// Creates a queue with metadata.
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
        // Assert
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

/// Re-creating a queue with identical metadata returns 204 No Content (success).
#[recorded::test]
async fn test_create_queue_fail_on_exist(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Act
        let response = queue_client.create(None).await?;

        // Assert
        assert_eq!(
            response.status(),
            StatusCode::NoContent,
            "Expected 204 No Content when re-creating an existing queue with same metadata, got {}",
            response.status()
        );
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;
    Ok(())
}

/// Creating an already-existing queue with different metadata returns 409 Conflict.
#[recorded::test]
async fn test_create_queue_fail_on_exist_different_metadata(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Act
        let options = Some(QueueClientCreateOptions {
            metadata: Some(HashMap::from([("env".to_string(), "conflict".to_string())])),
            ..Default::default()
        });
        let err = queue_client.create(options).await.err().unwrap();

        // Assert
        assert_eq!(
            err.http_status(),
            Some(StatusCode::Conflict),
            "Expected 409 Conflict when re-creating a queue with different metadata, got {:?}",
            err.http_status()
        );
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;
    Ok(())
}

/// Deletes a queue.
#[recorded::test]
async fn test_delete_queue(ctx: TestContext) -> Result<()> {
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

/// Checks whether a queue exists.
#[recorded::test]
async fn test_queue_exists(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Act
        let exists_response = queue_client.exists().await?;

        // Assert
        assert!(exists_response, "Queue should exist");

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await?;

    // Act
    let non_existent_exists_response = queue_client.exists().await?;

    // Assert
    assert!(!non_existent_exists_response, "Queue should not exist");

    test_result?;
    Ok(())
}

/// Sets queue metadata.
#[recorded::test]
async fn test_set_metadata(ctx: TestContext) -> Result<()> {
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

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Arrange
        let metadata = HashMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ]);

        // Act
        let response = queue_client.set_metadata(&metadata, None).await?;

        // Assert
        assert_successful_response(&response);

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

    // Cleanup
    queue_client.delete(None).await?;

    test_result?;
    Ok(())
}

/// Gets queue properties.
#[recorded::test]
async fn test_get_queue_properties(ctx: TestContext) -> Result<()> {
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

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Act
        let props = queue_client.get_properties(None).await?;

        // Assert
        assert_eq!(
            props.approximate_messages_count()?,
            Some(0),
            "Expected approximate_messages_count to be 0 for empty queue"
        );

        queue_client
            .send_message(
                QueueMessage {
                    message_text: Some("hello".to_string()),
                }
                .try_into()?,
                None,
            )
            .await?;

        let props = queue_client.get_properties(None).await?;
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

/// Sends a message to the queue.
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

        let sent: SentMessage = response.into_model()?;
        assert!(
            sent.message_id.is_some(),
            "Expected message_id to be set on the sent message"
        );
        assert!(
            sent.pop_receipt.is_some(),
            "Expected pop_receipt to be set on the sent message"
        );
        assert!(
            sent.insertion_time.is_some(),
            "Expected insertion_time to be set on the sent message"
        );
        assert!(
            sent.expiration_time.is_some(),
            "Expected expiration_time to be set on the sent message"
        );
        assert!(
            sent.time_next_visible.is_some(),
            "Expected time_next_visible to be set on the sent message"
        );

        // Invalid Control Character Scenario
        let err = queue_client
            .send_message(
                QueueMessage {
                    message_text: Some("\u{0001}invalid".to_string()),
                }
                .try_into()?,
                None,
            )
            .await
            .err()
            .unwrap();
        assert_eq!(
            err.http_status(),
            Some(StatusCode::BadRequest),
            "Expected 400 Bad Request for message containing XML-invalid control char, got {:?}",
            err.http_status()
        );

        // Whitespace Content Scenario
        queue_client.clear(None).await?;
        let whitespace_text = " mess\t age1\n";
        queue_client
            .send_message(
                QueueMessage {
                    message_text: Some(whitespace_text.to_string()),
                }
                .try_into()?,
                None,
            )
            .await?;
        let recv_response = queue_client.receive_messages(None).await?;
        assert_successful_response(&recv_response);
        let whitespace_messages = recv_response
            .into_model()?
            .items
            .expect("Expected at least one message");
        assert_eq!(whitespace_messages.len(), 1, "Expected exactly one message");
        assert_eq!(
            whitespace_messages[0].message_text.as_deref(),
            Some(whitespace_text),
            "Whitespace message did not round-trip correctly"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;
    Ok(())
}

/// Sends a message with `visibility_timeout` and confirms it is initially hidden.
#[recorded::test]
async fn test_send_message_with_visibility_timeout(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Arrange
        let options = Some(QueueClientSendMessageOptions {
            visibility_timeout: Some(30),
            message_time_to_live: Some(300),
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

        // Assert
        assert!(
            response.status() == StatusCode::Created,
            "Expected status code 201, got {}",
            response.status(),
        );

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

/// Sends a message with `message_time_to_live` and checks the returned expiration metadata.
#[recorded::test]
async fn test_send_message_with_ttl(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Arrange
        let options = Some(QueueClientSendMessageOptions {
            message_time_to_live: Some(300),
            ..Default::default()
        });

        // Act
        let response = queue_client
            .send_message(
                QueueMessage {
                    message_text: Some("TTL test message".to_string()),
                }
                .try_into()?,
                options,
            )
            .await?;

        // Assert
        assert_eq!(
            response.status(),
            StatusCode::Created,
            "Expected 201 Created, got {}",
            response.status()
        );

        let sent: SentMessage = response.into_model()?;
        assert!(
            sent.expiration_time.is_some(),
            "Expected expiration_time to be set for a message sent with message_time_to_live"
        );

        if let (Some(exp), Some(ins)) = (sent.expiration_time, sent.insertion_time) {
            assert!(
                exp > ins,
                "Expected expiration_time ({exp}) to be after insertion_time ({ins})"
            );
        }

        // Infinite TTL Scenario
        let infinite_ttl_options = Some(QueueClientSendMessageOptions {
            message_time_to_live: Some(-1),
            ..Default::default()
        });
        let infinite_response = queue_client
            .send_message(
                QueueMessage {
                    message_text: Some("infinite TTL".to_string()),
                }
                .try_into()?,
                infinite_ttl_options,
            )
            .await?;
        assert_eq!(
            infinite_response.status(),
            StatusCode::Created,
            "Expected 201 Created for infinite TTL message, got {}",
            infinite_response.status()
        );
        let infinite_sent: SentMessage = infinite_response.into_model()?;
        let expiry_year = infinite_sent
            .expiration_time
            .expect("Expected expiration_time to be set for infinite TTL message")
            .year();
        assert_eq!(
            expiry_year, 9999,
            "Expected expiration year to be 9999 for a message with infinite TTL, got {expiry_year}"
        );

        // Large Finite TTL Scenario
        let large_ttl_response = queue_client
            .send_message(
                QueueMessage {
                    message_text: Some("large TTL".to_string()),
                }
                .try_into()?,
                Some(QueueClientSendMessageOptions {
                    message_time_to_live: Some(1024 * 1024 * 1024),
                    ..Default::default()
                }),
            )
            .await?;
        assert_eq!(
            large_ttl_response.status(),
            StatusCode::Created,
            "Expected 201 Created for large TTL message, got {}",
            large_ttl_response.status()
        );
        let large_ttl_sent: SentMessage = large_ttl_response.into_model()?;
        assert!(
            large_ttl_sent.expiration_time.is_some(),
            "Expected expiration_time to be set for a message with a large time-to-live"
        );
        if let (Some(exp), Some(ins)) = (
            large_ttl_sent.expiration_time,
            large_ttl_sent.insertion_time,
        ) {
            assert!(
                exp > ins,
                "Expected expiration_time ({exp}) to be after insertion_time ({ins}) for large TTL"
            );
        }

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;
    Ok(())
}

/// Sends and receives a Unicode message to confirm the content round-trips correctly.
#[recorded::test]
async fn test_unicode_message_content(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Arrange
        let unicode_text = "こんにちは Azure 🦀 - Ünïcödé";
        queue_client
            .send_message(
                QueueMessage {
                    message_text: Some(unicode_text.to_string()),
                }
                .try_into()?,
                None,
            )
            .await?;

        // Act
        let received = queue_client.receive_messages(None).await?;
        let messages = received.into_model()?.items.expect("Expected a message");

        // Assert
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

/// Sends and peeks a message body near the 64 KiB limit.
#[recorded::test]
async fn test_send_near_64kb_message(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Arrange
        const PAYLOAD_LEN: usize = 60 * 1024;
        let large_text = "a".repeat(PAYLOAD_LEN);

        // Act
        let response = queue_client
            .send_message(
                QueueMessage {
                    message_text: Some(large_text.clone()),
                }
                .try_into()?,
                None,
            )
            .await?;

        // Assert
        assert_eq!(
            response.status(),
            StatusCode::Created,
            "Expected 201 Created for large message send, got {}",
            response.status()
        );

        let peek_response = queue_client.peek_messages(None).await?;
        assert_successful_response(&peek_response);

        let peeked = peek_response
            .into_model()?
            .items
            .expect("Expected peeked messages");
        assert_eq!(peeked.len(), 1, "Expected exactly one peeked message");

        let recovered_len = peeked[0]
            .message_text
            .as_ref()
            .map(String::len)
            .unwrap_or(0);
        assert_eq!(
            recovered_len, PAYLOAD_LEN,
            "Expected recovered message length {PAYLOAD_LEN}, got {recovered_len}"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;
    Ok(())
}

/// Receives messages using `visibility_timeout` and `number_of_messages` options.
#[recorded::test]
async fn test_receive_messages_with_options(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Arrange
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
            visibility_timeout: Some(60),
            ..Default::default()
        });

        // Act
        let response = queue_client.receive_messages(options).await?;
        assert_successful_response(&response);

        let messages = response.into_model()?.items.expect("Expected messages");

        // Assert
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

/// Receives messages with the service maximum `number_of_messages` value.
#[recorded::test]
async fn test_receive_messages_max_count(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Arrange
        for i in 0..5 {
            queue_client
                .send_message(
                    QueueMessage {
                        message_text: Some(format!("msg-{i}")),
                    }
                    .try_into()?,
                    None,
                )
                .await?;
        }

        let options = Some(QueueClientReceiveMessagesOptions {
            number_of_messages: Some(32),
            ..Default::default()
        });

        // Act
        let response = queue_client.receive_messages(options).await?;
        assert_successful_response(&response);

        let messages = response.into_model()?.items.expect("Expected messages");

        // Assert
        assert_eq!(
            messages.len(),
            5,
            "Expected 5 messages when requesting up to 32, got {}",
            messages.len()
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;
    Ok(())
}

/// Peeks messages across missing, empty, and populated queue states.
#[recorded::test]
async fn test_peek_messages(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;
    let test_messages = ["Message 1", "Message 2"];

    // Act
    let err = queue_client.peek_messages(None).await.err().unwrap();

    // Assert
    assert_eq!(
        err.http_status(),
        Some(StatusCode::NotFound),
        "Expected 404 Not Found for peek_messages on non-existent queue, got {:?}",
        err.http_status()
    );

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Act
        let response = queue_client.peek_messages(None).await?;
        assert_successful_response(&response);

        let messages = response.into_model()?;

        // Assert
        assert!(
            messages.items.is_none(),
            "Expected to receive no messages from an empty queue, but got Some"
        );

        setup_test_queue_with_messages(&queue_client, &test_messages).await?;

        // Default No Options Scenario
        let default_peek = queue_client.peek_messages(None).await?;
        assert_successful_response(&default_peek);
        let default_peeked = default_peek
            .into_model()?
            .items
            .expect("Expected at least one message with default peek");
        assert_eq!(
            default_peeked.len(),
            1,
            "Expected default peek (no options) to return exactly 1 message even when multiple are queued"
        );

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

/// Receives messages across missing, empty, and populated queue states.
#[recorded::test]
async fn test_receive_messages(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;
    let test_messages = ["Message 1", "Message 2"];

    // Act
    let err = queue_client.receive_messages(None).await.err().unwrap();

    // Assert
    assert_eq!(
        err.http_status(),
        Some(StatusCode::NotFound),
        "Expected 404 Not Found for receive_messages on non-existent queue, got {:?}",
        err.http_status()
    );

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Act
        let response = queue_client.receive_messages(None).await?;
        assert_successful_response(&response);

        let messages = response.into_model()?;

        // Assert
        assert!(
            messages.items.is_none(),
            "Expected to dequeue no messages from an empty queue, but got Some"
        );

        setup_test_queue_with_messages(&queue_client, &test_messages).await?;

        let options = Some(QueueClientReceiveMessagesOptions {
            number_of_messages: Some(10),
            ..Default::default()
        });

        // Act
        let response = queue_client.receive_messages(options).await?;
        assert_successful_response(&response);

        let messages = response.into_model()?.items.unwrap();

        // Assert
        assert_eq!(
            messages.len(),
            test_messages.len(),
            "Expected to dequeue {} messages, got {}",
            test_messages.len(),
            messages.len()
        );

        for (i, message) in messages.iter().enumerate() {
            assert_message_text(message.message_text.clone(), test_messages[i], i);
            assert_eq!(
                message.dequeue_count,
                Some(1),
                "Expected dequeue_count == 1 for message {i}"
            );
            assert!(
                message.insertion_time.is_some(),
                "Expected insertion_time to be set on received message {i}"
            );
            assert!(
                message.expiration_time.is_some(),
                "Expected expiration_time to be set on received message {i}"
            );
            assert!(
                message.pop_receipt.is_some(),
                "Expected pop_receipt to be set on received message {i}"
            );
            assert!(
                message.time_next_visible.is_some(),
                "Expected time_next_visible to be set on received message {i}"
            );
        }

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result
}

/// Updates a message and then receives the updated content.
#[recorded::test]
async fn test_update_message(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Arrange
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

        let option = Some(QueueClientUpdateMessageOptions {
            queue_message: Some(
                QueueMessage {
                    message_text: Some("Updated message text from Rust".to_string()),
                }
                .try_into()?,
            ),
            ..Default::default()
        });

        // Act
        let update_response = queue_client
            .update_message(
                &sent_message.message_id.clone().unwrap(),
                &sent_message.pop_receipt.clone().unwrap(),
                0,
                option,
            )
            .await?;

        // Assert
        assert!(
            update_response.status().is_success(),
            "Expected successful status code, got {}",
            update_response.status(),
        );

        let received = queue_client.receive_messages(None).await?;
        let messages = received.into_model()?.items.expect("Expected one message");
        assert_eq!(messages.len(), 1, "Expected exactly one message");
        assert_eq!(
            messages[0].message_text.as_deref(),
            Some("Updated message text from Rust"),
            "Message text was not updated"
        );
        assert_eq!(
            messages[0].dequeue_count,
            Some(1),
            "Expected dequeue_count == 1 on first receive after update"
        );

        // Dequeue Count Increment Scenario
        let recv_id = messages[0].message_id.clone().expect("Expected message_id");
        let recv_receipt = messages[0]
            .pop_receipt
            .clone()
            .expect("Expected pop_receipt");
        queue_client
            .update_message(&recv_id, &recv_receipt, 0, None)
            .await?;
        let second_recv = queue_client.receive_messages(None).await?;
        let second_msgs = second_recv
            .into_model()?
            .items
            .expect("Expected message on second receive");
        assert_eq!(
            second_msgs[0].dequeue_count,
            Some(2),
            "Expected dequeue_count == 2 after receiving an updated message a second time"
        );

        queue_client.clear(None).await?;

        // Unicode Content Update Scenario
        let unicode_text = "啊齄丂狛狜";
        let send_response = queue_client
            .send_message(
                QueueMessage {
                    message_text: Some("initial ascii".to_string()),
                }
                .try_into()?,
                None,
            )
            .await?;
        let sent: SentMessage = send_response.into_model()?;
        let message_id = sent.message_id.clone().expect("Expected message_id");
        let pop_receipt = sent.pop_receipt.clone().expect("Expected pop_receipt");
        let update_options = Some(QueueClientUpdateMessageOptions {
            queue_message: Some(
                QueueMessage {
                    message_text: Some(unicode_text.to_string()),
                }
                .try_into()?,
            ),
            ..Default::default()
        });
        queue_client
            .update_message(&message_id, &pop_receipt, 0, update_options)
            .await?;
        let recv_response = queue_client.receive_messages(None).await?;
        assert_successful_response(&recv_response);
        let unicode_messages = recv_response
            .into_model()?
            .items
            .expect("Expected at least one message");
        assert_eq!(
            unicode_messages.len(),
            1,
            "Expected exactly one message after unicode update"
        );
        assert_eq!(
            unicode_messages[0].message_text.as_deref(),
            Some(unicode_text),
            "Expected updated Unicode content, got {:?}",
            unicode_messages[0].message_text
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;
    Ok(())
}

/// Deletes a previously enqueued message.
#[recorded::test]
async fn test_delete_message(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Arrange
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

        // Assert
        assert_successful_response(&delete_response);

        let received = queue_client.receive_messages(None).await?;
        let remaining = received.into_model()?;
        assert!(
            remaining.items.is_none(),
            "Expected queue to be empty after deleting the only message"
        );

        // Partial Queue Delete Scenario
        for message in ["message-a", "message-b", "message-c"] {
            queue_client
                .send_message(
                    QueueMessage {
                        message_text: Some(message.to_string()),
                    }
                    .try_into()?,
                    None,
                )
                .await?;
        }
        let recv = queue_client.receive_messages(None).await?;
        let first_message = recv
            .into_model()?
            .items
            .expect("Expected at least one message")
            .into_iter()
            .next()
            .expect("Expected a message");
        queue_client
            .delete_message(
                &first_message.message_id.clone().unwrap(),
                &first_message.pop_receipt.clone().unwrap(),
                None,
            )
            .await?;
        let peek_resp = queue_client
            .peek_messages(Some(QueueClientPeekMessagesOptions {
                number_of_messages: Some(10),
                ..Default::default()
            }))
            .await?;
        let remaining_messages = peek_resp
            .into_model()?
            .items
            .expect("Expected 2 remaining messages");
        assert_eq!(
            remaining_messages.len(),
            2,
            "Expected 2 messages to remain after deleting 1 of 3"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;
    Ok(())
}

/// Clears all messages from the queue.
#[recorded::test]
async fn test_clear_messages(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Arrange
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

        // Assert
        assert_successful_response(&response);

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

    test_result?;
    Ok(())
}

/// Sets an access policy on a queue.
#[recorded::test]
async fn test_queue_access_policy(ctx: TestContext) -> Result<()> {
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

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Act
        let response = queue_client.get_access_policy(None).await?;
        assert_successful_response(&response);

        let acl = response.into_model()?;

        // Assert
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

/// Sets an access policy with explicit `start` and `expiry` dates.
#[recorded::test]
async fn test_queue_access_policy_with_dates(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        let start_str = recording.var(
            "start",
            Some(VarOptions {
                default_value: Some(to_rfc3339(&OffsetDateTime::now_utc()).into()),
                ..Default::default()
            }),
        );
        let expiry_str = recording.var(
            "expiry",
            Some(VarOptions {
                default_value: Some(
                    to_rfc3339(&(OffsetDateTime::now_utc() + Duration::days(365))).into(),
                ),
                ..Default::default()
            }),
        );

        let policy = SignedIdentifiers {
            items: Some(vec![SignedIdentifier {
                id: Some("timed-policy".to_string()),
                access_policy: Some(AccessPolicy {
                    permission: Some("r".to_string()),
                    start: Some(parse_rfc3339(&start_str)?),
                    expiry: Some(parse_rfc3339(&expiry_str)?),
                }),
            }]),
        };

        // Act
        let set_response = queue_client
            .set_access_policy(policy.try_into()?, None)
            .await?;
        assert_successful_response(&set_response);

        if recording.test_mode() == TestMode::Live || recording.test_mode() == TestMode::Record {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }

        let get_response = queue_client.get_access_policy(None).await?;
        assert_successful_response(&get_response);

        let acl = get_response.into_model()?;
        let items = acl.items.expect("Expected signed identifiers");
        assert_eq!(items.len(), 1, "Expected exactly one signed identifier");

        let ap = items[0]
            .access_policy
            .as_ref()
            .expect("Expected access policy");

        // Assert - both dates survive the round-trip
        assert_eq!(ap.permission.as_deref(), Some("r"));
        assert!(ap.start.is_some(), "Expected start to round-trip");
        assert!(ap.expiry.is_some(), "Expected expiry to round-trip");
        assert_eq!(
            ap.start.map(|t| t.unix_timestamp()),
            Some(parse_rfc3339(&start_str)?.unix_timestamp()),
            "start date did not round-trip"
        );
        assert_eq!(
            ap.expiry.map(|t| t.unix_timestamp()),
            Some(parse_rfc3339(&expiry_str)?.unix_timestamp()),
            "expiry date did not round-trip"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;
    Ok(())
}

/// Sets a named access policy identifier without a policy body and verifies the ID is persisted.
#[recorded::test]
async fn test_set_access_policy_empty_named_identifier(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        // Act
        let policy = SignedIdentifiers {
            items: Some(vec![SignedIdentifier {
                id: Some("empty".to_string()),
                access_policy: None,
            }]),
        };
        let set_response = queue_client
            .set_access_policy(policy.try_into()?, None)
            .await?;
        assert_successful_response(&set_response);

        if recording.test_mode() == azure_core_test::TestMode::Live
            || recording.test_mode() == azure_core_test::TestMode::Record
        {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }

        // Assert
        let acl = queue_client.get_access_policy(None).await?.into_model()?;
        let items = acl.items.expect("Expected at least one signed identifier");
        assert_eq!(items.len(), 1, "Expected exactly one signed identifier");
        assert_eq!(
            items[0].id.as_deref(),
            Some("empty"),
            "Expected ID 'empty' to be persisted"
        );
        assert!(
            items[0].access_policy.is_none()
                || items[0]
                    .access_policy
                    .as_ref()
                    .map(|ap| ap.permission.is_none() && ap.start.is_none() && ap.expiry.is_none())
                    .unwrap_or(false),
            "Expected access_policy fields to be absent for an empty named identifier"
        );

        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;
    Ok(())
}

/// Setting more than 5 signed identifiers is rejected by the service with 400 Bad Request.
#[recorded::test]
async fn test_set_access_policy_too_many_identifiers(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let queue_client = get_queue_client(recording, &get_queue_name(recording)).await?;

    // Arrange
    queue_client.create(None).await?;

    let test_result = async {
        let identifiers: Vec<SignedIdentifier> = (0..16)
            .map(|i| SignedIdentifier {
                id: Some(format!("policy{i}")),
                access_policy: Some(AccessPolicy {
                    permission: Some("r".to_string()),
                    ..Default::default()
                }),
            })
            .collect();
        let policy = SignedIdentifiers {
            items: Some(identifiers),
        };

        // Act
        let err = queue_client
            .set_access_policy(policy.try_into()?, None)
            .await
            .err()
            .unwrap();

        // Assert
        assert_eq!(
            err.http_status(),
            Some(StatusCode::BadRequest),
            "Expected 400 Bad Request for too many signed identifiers, got {:?}",
            err.http_status()
        );
        Ok::<(), azure_core::Error>(())
    }
    .await;

    // Cleanup
    queue_client.delete(None).await.unwrap();

    test_result?;
    Ok(())
}

/// Creating a queue with an invalid name (non-ASCII characters) is rejected with 400 Bad Request.
#[recorded::test]
async fn test_invalid_queue_name(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let (options, endpoint, _) = recorded_test_setup(recording);
    let queue_client_options = QueueClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };

    // Act - queue names must be lowercase alphanumeric; Unicode characters are not valid.
    let queue_client = QueueClient::new(
        &endpoint,
        "啊齄丂狛狜",
        Some(recording.credential()),
        Some(queue_client_options),
    )?;
    let err = queue_client.create(None).await.err().unwrap();

    // Assert
    assert_eq!(
        err.http_status(),
        Some(StatusCode::BadRequest),
        "Expected 400 Bad Request for invalid queue name, got {:?}",
        err.http_status()
    );
    Ok(())
}

/// Retries an initial IO error before surfacing the service error.
#[recorded::test]
async fn test_retry_on_io_error(ctx: TestContext) -> Result<()> {
    // Recording Setup
    let recording = ctx.recording();
    let (base_options, endpoint, _) = recorded_test_setup(recording);

    // Arrange
    let invocations = Arc::new(AtomicUsize::new(0));
    let fail_first: Arc<dyn Policy> = Arc::new(FailFirstPolicy {
        invocations: invocations.clone(),
    });

    let mut queue_client_options = QueueClientOptions {
        client_options: base_options,
        ..Default::default()
    };
    queue_client_options.client_options.retry = RetryOptions::fixed(FixedRetryOptions {
        delay: azure_core::time::Duration::ZERO,
        max_retries: 1,
        max_total_elapsed: azure_core::time::Duration::seconds(5),
    });
    queue_client_options
        .client_options
        .per_try_policies
        .push(fail_first);

    let queue_name = get_queue_name(recording);
    let queue_client = QueueClient::new(
        &endpoint,
        &queue_name,
        Some(recording.credential()),
        Some(queue_client_options),
    )?;

    // Act
    let err = queue_client.get_properties(None).await.err().unwrap();

    // Assert
    assert_eq!(
        err.http_status(),
        Some(StatusCode::NotFound),
        "Expected 404 from the retried attempt, got {:?}",
        err.http_status()
    );
    assert_eq!(
        invocations.load(Ordering::Relaxed),
        2,
        "Expected FailFirstPolicy to be invoked exactly 2 times (1 initial + 1 retry)"
    );

    Ok(())
}

#[derive(Debug)]
struct FailFirstPolicy {
    invocations: Arc<AtomicUsize>,
}

#[async_trait::async_trait]
impl Policy for FailFirstPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut azure_core::http::Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let n = self.invocations.fetch_add(1, Ordering::Relaxed);
        if n == 0 {
            Err(azure_core::Error::new(
                ErrorKind::Io,
                "Simulated IO error for retry testing",
            ))
        } else {
            if next.is_empty() {
                return Err(azure_core::Error::new(
                    ErrorKind::Other,
                    "FailFirstPolicy: no next policy available in pipeline",
                ));
            }
            next[0].send(ctx, request, &next[1..]).await
        }
    }
}

async fn get_queue_client(recording: &Recording, queue_name: &str) -> Result<QueueClient> {
    let (options, endpoint, _) = recorded_test_setup(recording);
    let queue_client_options = QueueClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };

    QueueClient::new(
        &endpoint,
        queue_name,
        Some(recording.credential()),
        Some(queue_client_options),
    )
}

async fn setup_test_queue_with_messages(
    queue_client: &QueueClient,
    messages: &[&str],
) -> Result<()> {
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
    // Act
    let response = queue_client.peek_messages(options).await?;
    assert_successful_response(&response);

    let messages = response.into_model()?;
    let messages = messages.items.unwrap();

    // Assert
    assert_eq!(
        messages.len(),
        count,
        "Expected to receive exactly {} messages, got {}",
        count,
        messages.len()
    );

    for (i, message) in messages.iter().enumerate() {
        assert_message_text(message.message_text.clone(), expected_messages[i], i);
        assert_eq!(
            message.dequeue_count,
            Some(0),
            "Expected dequeue_count == 0 for peeked message {i}"
        );
        assert!(
            message.insertion_time.is_some(),
            "Expected insertion_time to be set on peeked message {i}"
        );
        assert!(
            message.expiration_time.is_some(),
            "Expected expiration_time to be set on peeked message {i}"
        );
    }

    Ok(())
}

fn assert_successful_response<T, F>(response: &Response<T, F>) {
    assert!(
        response.status().is_success(),
        "Expected successful status code, got {}",
        response.status()
    );
}

use std::collections::HashMap;

mod helpers;
use helpers::endpoint::get_endpoint;
use helpers::logs::log_operation_result;
use helpers::random_queue_name::get_random_queue_name;

use azure_core::{
    http::{Response, XmlFormat},
    Error,
};
use azure_identity::DefaultAzureCredential;
use azure_storage_queue::{
    clients::QueueClient,
    models::{
        QueueClientGetMetadataResultHeaders, QueueClientPeekMessagesOptions,
        QueueClientReceiveMessagesOptions, QueueClientSetMetadataOptions, QueueClientUpdateOptions,
        QueueMessage, SentMessage,
    },
};

async fn send_message(
    queue_client: &QueueClient,
    message: &str,
) -> Result<Response<Option<SentMessage>, XmlFormat>, Error> {
    let queue_message = QueueMessage {
        message_text: Some(message.to_owned()),
    };

    queue_client
        .send_message(queue_message.try_into()?, None)
        .await
}

async fn send_and_delete_message(
    queue_client: &QueueClient,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = send_message(queue_client, message).await;

    if let Ok(response) = result {
        let message = response.into_body().await?;

        if let Some(message) = message {
            if let (Some(message_id), Some(pop_receipt)) = (message.message_id, message.pop_receipt)
            {
                println!(
                    "Deleting message with ID: {} and pop receipt: {}",
                    message_id, pop_receipt
                );
                let delete_result = queue_client
                    .delete_message(&message_id, &pop_receipt, None)
                    .await;
                log_operation_result(&delete_result, "delete_message");
            }
        }
    }

    Ok(())
}

async fn send_and_update_message(
    queue_client: &QueueClient,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = send_message(queue_client, message).await;

    if let Ok(response) = result {
        let message = response.into_body().await?;

        if let Some(message) = message {
            if let (Some(message_id), Some(pop_receipt)) = (message.message_id, message.pop_receipt)
            {
                println!(
                    "Updating message with ID: {} and pop receipt: {}",
                    message_id, pop_receipt
                );
                let queue_message = QueueMessage {
                    message_text: Some("Updated message text from Rust".to_string()),
                };
                let update_option = QueueClientUpdateOptions {
                    // Serialize the message text as bytes for the update
                    queue_message: Some(queue_message.try_into()?),
                    ..Default::default()
                };
                let update_result = queue_client
                    .update_message(&message_id.clone(), &pop_receipt, 50, Some(update_option))
                    .await;
                log_operation_result(&update_result, "update_message");
            }
        }
    }

    Ok(())
}

async fn set_and_get_metadata(
    queue_client: &QueueClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let metadata_options = Some(QueueClientSetMetadataOptions {
        metadata: Some(HashMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ])),
        ..Default::default()
    });
    let result = queue_client.set_metadata(metadata_options).await;
    log_operation_result(&result, "set_metadata");

    let result = queue_client.get_metadata(None).await;
    log_operation_result(&result, "get_metadata");

    let metadata = result.unwrap().metadata().unwrap_or_default();
    for (key, value) in metadata {
        println!("Metadata - {}: {}", key, value);
    }

    Ok(())
}

async fn peek_and_receive_messages(
    queue_client: &QueueClient,
) -> Result<(), Box<dyn std::error::Error>> {
    _ = send_message(queue_client, "Message 1 from Rust Queue SDK").await;
    _ = send_message(queue_client, "Message 2 from Rust Queue SDK").await;

    let options = QueueClientPeekMessagesOptions {
        number_of_messages: Some(5),
        ..Default::default()
    };

    let result = queue_client.peek_messages(Some(options)).await;
    log_operation_result(&result, "peek_messages");

    if let Ok(response) = result {
        let messages = response.into_body().await?;
        if let Some(messages) = messages.items {
            for msg in messages {
                println!(
                    "Successfully peeked message ({}): {}",
                    msg.message_id.unwrap(),
                    msg.message_text.unwrap_or_default()
                );
            }
        }
    }

    let options = QueueClientReceiveMessagesOptions {
        number_of_messages: Some(5),
        ..Default::default()
    };

    let result = queue_client.receive_messages(Some(options)).await;
    log_operation_result(&result, "receive_messages");

    if let Ok(response) = result {
        let messages = response.into_body().await?;
        if let Some(messages) = messages.items {
            for msg in messages {
                println!(
                    "Successfully received message ({}): {}",
                    msg.message_id.unwrap(),
                    msg.message_text.unwrap_or_default()
                );
            }
        }
    }

    Ok(())
}

async fn peek_and_receive_message(
    queue_client: &QueueClient,
) -> Result<(), Box<dyn std::error::Error>> {
    _ = send_message(queue_client, "Message 1 from Rust Queue SDK").await;
    _ = send_message(queue_client, "Message 2 from Rust Queue SDK").await;

    let options = QueueClientPeekMessagesOptions {
        number_of_messages: Some(5),
        ..Default::default()
    };

    let result = queue_client.peek_message(Some(options)).await;
    log_operation_result(&result, "peek_message");

    if let Ok(response) = result {
        let message = response.into_body().await?;
        if let Some(message) = message {
            println!(
                "Successfully peeked message ({}): {}",
                message.message_id.unwrap(),
                message.message_text.unwrap_or_default()
            );
        }
    }

    loop {
        let result = queue_client.receive_message(None).await;
        log_operation_result(&result, "receive_message");

        if let Ok(response) = result {
            let message = response.into_body().await?;
            if let Some(msg) = message {
                println!(
                    "Successfully received message ({}): {}",
                    msg.message_id.unwrap(),
                    msg.message_text.unwrap_or_default()
                );
            } else {
                // No more messages available
                break;
            }
        } else {
            // Error occurred, break the loop
            break;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;

    // Retrieve the storage account endpoint from environment variable.
    let endpoint = get_endpoint();

    let queue_name = get_random_queue_name();
    let queue_client = QueueClient::new(&endpoint, &queue_name, credential.clone(), None)?;

    // Create and manage queue
    let result = queue_client.create(None).await;
    log_operation_result(&result, "create");

    let result = queue_client.exists().await;
    log_operation_result(&result, "check_exists");

    let result = queue_client.create_if_not_exists(None).await;
    log_operation_result(&result, "create_if_not_exists");

    // Set and get queue metadata
    set_and_get_metadata(&queue_client).await?;

    let result = send_message(&queue_client, "Example Message").await;
    log_operation_result(&result, "send_message");

    send_and_update_message(
        &queue_client,
        "Example message created from Rust, ready for update",
    )
    .await?;

    // Clear messages
    let result = queue_client.clear(None).await;
    log_operation_result(&result, "clear");

    // Send and process messages
    send_and_delete_message(
        &queue_client,
        "Example message created from Rust, ready for deletion",
    )
    .await?;

    // Peek and Receive messages
    peek_and_receive_messages(&queue_client).await?;

    // Peek and Receive message
    peek_and_receive_message(&queue_client).await?;

    // Cleanup
    let result = queue_client.delete(None).await;
    log_operation_result(&result, "delete");

    let non_existing_queue_client =
        QueueClient::new(&endpoint, "non-existent-queue", credential.clone(), None)?;
    let result = non_existing_queue_client.exists().await;
    log_operation_result(&result, "check_non_existent");

    let result = non_existing_queue_client.delete_if_exists(None).await;
    log_operation_result(&result, "delete_if_exists");

    Ok(())
}

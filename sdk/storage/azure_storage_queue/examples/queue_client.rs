// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::collections::HashMap;

use azure_core::{
    http::{Response, StatusCode, XmlFormat},
    Error,
};
use azure_identity::DeveloperToolsCredential;
use azure_storage_queue::{
    models::{
        QueueClientGetMetadataResultHeaders, QueueClientPeekMessagesOptions,
        QueueClientReceiveMessagesOptions, QueueClientUpdateOptions, QueueMessage, SentMessage,
    },
    QueueClient,
};

async fn send_message(
    queue_client: &QueueClient,
    message: &str,
) -> Result<Response<SentMessage, XmlFormat>, Error> {
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
        let message = response.into_model()?;

        if let (Some(message_id), Some(pop_receipt)) = (message.message_id, message.pop_receipt) {
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

    Ok(())
}

async fn send_and_update_message(
    queue_client: &QueueClient,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = send_message(queue_client, message).await;

    if let Ok(response) = result {
        let message = response.into_model()?;

        if let (Some(message_id), Some(pop_receipt)) = (message.message_id, message.pop_receipt) {
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

    Ok(())
}

async fn set_and_get_metadata(
    queue_client: &QueueClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = queue_client
        .set_metadata(
            &HashMap::from([
                ("key1".to_string(), "value1".to_string()),
                ("key2".to_string(), "value2".to_string()),
            ]),
            None,
        )
        .await;
    log_operation_result(&result, "set_metadata");

    let result = queue_client.get_metadata(None).await;
    log_operation_result(&result, "get_metadata");

    let metadata = result.unwrap().metadata().unwrap_or_default();
    for (key, value) in metadata {
        println!("Metadata - {}: {}", key, value);
    }

    let result = queue_client.set_metadata(&HashMap::new(), None).await;
    log_operation_result(&result, "set_metadata_empty");

    let result = queue_client.get_metadata(None).await;
    log_operation_result(&result, "get_metadata_empty");

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
        let messages = response.into_model()?;
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
        let messages = response.into_model()?;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;

    // Retrieve the storage account endpoint from environment variable.
    let endpoint = get_endpoint();

    let queue_name = get_random_queue_name();
    let queue_client = QueueClient::new(&endpoint, &queue_name, Some(credential.clone()), None)?;

    // Create and manage queue
    let result = queue_client.create(None).await;
    log_operation_result(&result, "create");

    let result = queue_client.exists().await;
    log_operation_result(&result, "check_exists");

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

    // Cleanup
    let result = queue_client.delete(None).await;
    log_operation_result(&result, "delete");

    let non_existing_queue_client = QueueClient::new(
        &endpoint,
        "non-existent-queue",
        Some(credential.clone()),
        None,
    )?;
    let result = non_existing_queue_client.exists().await;
    log_operation_result(&result, "check_non_existent");

    Ok(())
}

fn get_endpoint() -> String {
    // Retrieve the storage account endpoint from environment variable.
    let storage_account_name = std::env::var("AZURE_QUEUE_STORAGE_ACCOUNT_NAME");
    let storage_account_name = match storage_account_name {
        Ok(url) => url,
        Err(_) => {
            eprintln!("Environment variable AZURE_QUEUE_STORAGE_ACCOUNT_NAME is not set");
            std::process::exit(1);
        }
    };

    format!("https://{}.queue.core.windows.net/", storage_account_name)
}

fn get_random_queue_name() -> String {
    use rand::RngExt;
    let mut rng = rand::rng();
    let random_suffix: u32 = rng.random_range(1000..9999);
    format!("sdk-test-queue-{}", random_suffix)
}

fn log_operation_result<T>(result: &Result<T, Error>, operation: &str)
where
    T: std::fmt::Debug,
{
    match result {
        Ok(response) => println!("Successfully {}: {:?}", operation, response),
        Err(e) => match e.http_status() {
            Some(StatusCode::NotFound) => println!("Unable to {}, resource not found", operation),
            Some(StatusCode::Forbidden) => println!(
                "Unable to {}, access forbidden - check credentials",
                operation
            ),
            _ => {
                eprintln!("Error during {}: {}", operation, e);
                if let Some(status) = e.http_status() {
                    eprintln!("HTTP Status: {}", status);
                }
                eprintln!("Full Error: {:#?}", e);
            }
        },
    }
}

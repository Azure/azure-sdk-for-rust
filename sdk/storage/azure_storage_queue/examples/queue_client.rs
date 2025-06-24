use std::collections::HashMap;

mod helpers;
use helpers::endpoint::get_endpoint;
use helpers::logs::log_operation_result;
use helpers::random_queue_name::get_random_queue_name;

use azure_core::http::RequestContent;
use azure_identity::DefaultAzureCredential;
use azure_storage_queue::{
    clients::QueueClient,
    models::{
        QueueMessage, QueueMessageIdOperationGroupClientUpdateOptions,
        QueueMessagesOperationGroupClientDequeueOptions,
        QueueMessagesOperationGroupClientPeekOptions,
    },
};

async fn send_and_delete_message(
    queue_client: &QueueClient,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = queue_client.enqueue_message(message, None).await;

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
    let result = queue_client.enqueue_message(message, None).await;

    if let Ok(response) = result {
        let message = response.into_body().await?;

        if let Some(message) = message {
            if let (Some(message_id), Some(pop_receipt)) = (message.message_id, message.pop_receipt)
            {
                println!(
                    "Updating message with ID: {} and pop receipt: {}",
                    message_id, pop_receipt
                );
                let message_xml_string = quick_xml::se::to_string(&QueueMessage {
                    message_text: Some("Updated message text from Rust".to_string()),
                });
                let update_option = QueueMessageIdOperationGroupClientUpdateOptions {
                    // Serialize the message text as bytes for the update
                    queue_message: Some(RequestContent::from(message_xml_string?.into_bytes())),
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

async fn get_and_set_access_policies(
    queue_client: &QueueClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = queue_client.get_access_policy(None).await;
    log_operation_result(&result, "get_access_policy");

    let properties = result.unwrap().into_body().await?;
    let properties_xml = quick_xml::se::to_string(&properties)?;
    let properties_bytes = properties_xml.into_bytes();

    let result = queue_client
        .set_access_policy(RequestContent::from(properties_bytes), None)
        .await;
    log_operation_result(&result, "set_access_policy");

    Ok(())
}

async fn peek_and_receive_messages(
    queue_client: &QueueClient,
) -> Result<(), Box<dyn std::error::Error>> {
    _ = queue_client
        .enqueue_message("Message 1 from Rust Queue SDK", None)
        .await;
    _ = queue_client
        .enqueue_message("Message 2 from Rust Queue SDK", None)
        .await;

    let options = QueueMessagesOperationGroupClientPeekOptions {
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

    let options = QueueMessagesOperationGroupClientDequeueOptions {
        number_of_messages: Some(5),
        ..Default::default()
    };

    let result = queue_client.dequeue_messages(Some(options)).await;
    log_operation_result(&result, "dequeue_messages");

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;

    // Retrieve the storage account endpoint from environment variable.
    let endpoint = get_endpoint();

    let queue_name = get_random_queue_name();
    let queue_client = QueueClient::new(&endpoint, &queue_name, credential.clone(), None)?;

    // Get queue service properties
    let result = queue_client.get_properties(None).await;
    log_operation_result(&result, "get_properties");

    // Create and manage queue
    let result = queue_client.create(None).await;
    log_operation_result(&result, "create");

    let result = queue_client.exists().await;
    log_operation_result(&result, "check_exists");

    let result = queue_client.create_if_not_exists(None).await;
    log_operation_result(&result, "create_if_not_exists");

    // Set queue metadata
    let metadata = HashMap::from([("key1", "value1"), ("key2", "value2")]);
    let result = queue_client.set_metadata(Some(metadata)).await;
    log_operation_result(&result, "set_metadata");

    let result = queue_client.enqueue_message("Example Message", None).await;
    log_operation_result(&result, "enqueue_message");

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

    // Receive messages
    peek_and_receive_messages(&queue_client).await?;

    get_and_set_access_policies(&queue_client).await?;

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

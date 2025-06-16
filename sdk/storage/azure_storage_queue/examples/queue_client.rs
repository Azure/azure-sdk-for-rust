use std::collections::HashMap;

use azure_core::{
    error::Error,
    http::{RequestContent, Response, StatusCode},
};
use azure_identity::DefaultAzureCredential;
use azure_storage_queue::{AzureQueueStorageMessagesOperationsClientDequeueOptions, QueueClient};

/// Custom error type for queue operations
#[derive(Debug)]
enum QueueError {
    NotFound(&'static str),
    Forbidden(&'static str),
    Other(Error),
}

impl std::fmt::Display for QueueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueError::NotFound(msg) => write!(f, "Not found: {}", msg),
            QueueError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            QueueError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl std::error::Error for QueueError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            QueueError::Other(e) => Some(e),
            _ => None,
        }
    }
}

impl From<Error> for QueueError {
    fn from(err: Error) -> Self {
        match err.http_status() {
            Some(StatusCode::NotFound) => QueueError::NotFound("Resource not found"),
            Some(StatusCode::Forbidden) => {
                QueueError::Forbidden("Access forbidden - check credentials")
            }
            _ => QueueError::Other(err),
        }
    }
}

/// Helper function to log operation results
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
            _ => eprintln!("Error during {}: {}", operation, e),
        },
    }
}

async fn send_and_delete_message(
    queue_client: &QueueClient,
    queue_name: &str,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = queue_client.send_message(queue_name, message, None).await;

    if let Ok(response) = result {
        let messages = response.into_body().await?;

        if let Some(message) = messages.value.and_then(|msgs| msgs.first().cloned()) {
            if let (Some(message_id), Some(pop_receipt)) = (message.message_id, message.pop_receipt)
            {
                println!(
                    "Message ready for deletion - ID: {}, Receipt: {}",
                    message_id, pop_receipt
                );
                let delete_result = queue_client
                    .delete_message(queue_name, &message_id, &pop_receipt, None)
                    .await;
                log_operation_result(&delete_result, "delete_message");
            }
        }
    }

    Ok(())
}

async fn receive_and_process_messages(
    queue_client: &QueueClient,
    queue_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    _ = queue_client
        .send_message(queue_name, "Message 1 from Rust Queue SDK", None)
        .await;
    _ = queue_client
        .send_message(queue_name, "Message 2 from Rust Queue SDK", None)
        .await;

    let options = AzureQueueStorageMessagesOperationsClientDequeueOptions {
        number_of_messages: Some(5),
        ..Default::default()
    };

    let result = queue_client
        .receive_messages(queue_name, Some(options))
        .await;
    log_operation_result(&result, "receive_messages");

    if let Ok(response) = result {
        let messages = response.into_body().await?;
        if let Some(messages) = messages.value {
            for msg in messages {
                if let Some(text) = msg.message_text {
                    println!("Received message: {}", text);
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;

    // Retrieve the storage account endpoint from environment variable.
    let endpoint = std::env::var("AZURE_QUEUE_STORAGE_ACCOUNT")?;

    // Validate endpoint format
    if !endpoint.ends_with("/") || !endpoint.starts_with("https://") {
        eprintln!("Endpoint must start with 'https://' and end with '/'");
        std::process::exit(1);
    }

    let queue_client = QueueClient::new(&endpoint, credential, None)?;
    let queue_name = get_random_queue_name();

    // Get queue service properties
    let result = queue_client.get_properties().await;
    log_operation_result(&result, "get_properties");

    // Create and manage queue
    let result = queue_client.create(&queue_name, None).await;
    log_operation_result(&result, "create");

    let result = queue_client.exists(&queue_name).await;
    log_operation_result(&result, "check_exists");

    let result = queue_client.exists("non-existent-queue").await;
    log_operation_result(&result, "check_non_existent");

    let result = queue_client.create_if_not_exists(&queue_name, None).await;
    log_operation_result(&result, "create_if_not_exists");

    // Set queue metadata
    let metadata = HashMap::from([("key1", "value1"), ("key2", "value2")]);
    let result = queue_client.set_metadata(&queue_name, Some(metadata)).await;
    log_operation_result(&result, "set_metadata");

    let result = queue_client
        .send_message(&queue_name, "Example Message", None)
        .await;
    log_operation_result(&result, "send_message");

    // Delete messages
    let result = queue_client.delete_messages(&queue_name).await;
    log_operation_result(&result, "delete_messages");

    // Send and process messages
    send_and_delete_message(
        &queue_client,
        &queue_name,
        "Example message created from Rust, ready for deletion",
    )
    .await?;

    // Receive messages
    receive_and_process_messages(&queue_client, &queue_name).await?;

    // Cleanup
    let result = queue_client.delete(&queue_name, None).await;
    log_operation_result(&result, "delete");

    let result = queue_client
        .delete_if_exists("non-existent-queue", None)
        .await;
    log_operation_result(&result, "delete_if_exists");

    Ok(())
}

/// Generates a random queue name with a suffix to ensure uniqueness.
fn get_random_queue_name() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random_suffix: u32 = rng.gen_range(1000..9999);
    format!("sdk-test-queue-{}", random_suffix)
}

async fn get_enqueued_message_properties(
    response: Response<ListOfEnqueuedMessage, azure_core::http::XmlFormat>,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let (_status_code, _headers, properties) = response.deconstruct();
    let xml = properties.collect_string().await?;
    let queue_messages_list: ListOfEnqueuedMessage = from_str(&xml)?;

    // Get the first message from the vector
    let enqueued_message = queue_messages_list
        .value
        .as_ref()
        .and_then(|msgs| msgs.first())
        .ok_or("No messages found in response")?;

    let pop_receipt = enqueued_message
        .pop_receipt
        .as_ref()
        .ok_or("PopReceipt not found")?;
    let message_id = enqueued_message
        .message_id
        .as_ref()
        .ok_or("MessageId not found")?;

    Ok((message_id.clone(), pop_receipt.clone()))
}

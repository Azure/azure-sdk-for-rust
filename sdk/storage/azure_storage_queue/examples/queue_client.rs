use std::collections::HashMap;

use azure_core::{
    error::Error,
    http::{RequestContent, StatusCode},
};
use azure_identity::DefaultAzureCredential;
use azure_storage_queue::{
    clients::QueueClient,
    models::{
        AzureQueueStorageMessageIdOperationGroupClientUpdateOptions,
        AzureQueueStorageMessagesOperationGroupClientDequeueOptions,
        AzureQueueStorageMessagesOperationGroupClientPeekOptions, QueueMessage,
    },
};

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
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = queue_client.send_message(message, None).await;

    if let Ok(response) = result {
        let messages = response.into_body().await?;

        if let Some(message) = messages.value.and_then(|msgs| msgs.first().cloned()) {
            if let (Some(message_id), Some(pop_receipt)) = (message.message_id, message.pop_receipt)
            {
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
    let result = queue_client.send_message(message, None).await;

    if let Ok(response) = result {
        let messages = response.into_body().await?;

        if let Some(message) = messages.value.and_then(|msgs| msgs.first().cloned()) {
            if let (Some(message_id), Some(pop_receipt)) = (message.message_id, message.pop_receipt)
            {
                let update_option = AzureQueueStorageMessageIdOperationGroupClientUpdateOptions {
                    // Serialize the message text as bytes for the update
                    queue_message: Some(RequestContent::from(
                        quick_xml::se::to_string(&QueueMessage {
                            message_text: Some("Updated message text from Rust".to_string()),
                        })?
                        .into_bytes(),
                    )),
                    request_id: Some(message_id.clone()),
                    ..Default::default()
                };
                let update_result = queue_client
                    .update_message(&message_id.clone(), &pop_receipt, 1, Some(update_option))
                    .await;
                log_operation_result(&update_result, "update_message");
            }
        }
    }

    Ok(())
}

async fn peek_and_receive_messages(
    queue_client: &QueueClient,
) -> Result<(), Box<dyn std::error::Error>> {
    _ = queue_client
        .send_message("Message 1 from Rust Queue SDK", None)
        .await;
    _ = queue_client
        .send_message("Message 2 from Rust Queue SDK", None)
        .await;

    let options = AzureQueueStorageMessagesOperationGroupClientPeekOptions {
        number_of_messages: Some(5),
        ..Default::default()
    };

    let result = queue_client.peek_messages(Some(options)).await;
    log_operation_result(&result, "peek_messages");

    if let Ok(response) = result {
        let messages = response.into_body().await?;
        if let Some(messages) = messages.value {
            for msg in messages {
                println!(
                    "Successfully peeked message ({}): {}",
                    msg.message_id.unwrap(),
                    msg.message_text.unwrap_or_default()
                );
            }
        }
    }

    let options = AzureQueueStorageMessagesOperationGroupClientDequeueOptions {
        number_of_messages: Some(5),
        ..Default::default()
    };

    let result = queue_client.receive_messages(Some(options)).await;
    log_operation_result(&result, "receive_messages");

    if let Ok(response) = result {
        let messages = response.into_body().await?;
        if let Some(messages) = messages.value {
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
    let endpoint = std::env::var("AZURE_QUEUE_STORAGE_ACCOUNT");
    let endpoint = match endpoint {
        Ok(url) => url,
        Err(_) => {
            eprintln!("Environment variable AZURE_QUEUE_STORAGE_ACCOUNT is not set");
            std::process::exit(1);
        }
    };

    // Validate endpoint format
    if !endpoint.ends_with("/") || !endpoint.starts_with("https://") {
        eprintln!("Endpoint must start with 'https://' and end with '/'");
        std::process::exit(1);
    }

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

    let result = queue_client.send_message("Example Message", None).await;
    log_operation_result(&result, "send_message");

    send_and_update_message(
        &queue_client,
        "Example message created from Rust, ready for update",
    )
    .await?;

    // Delete messages
    let result = queue_client.delete_messages(None).await;
    log_operation_result(&result, "delete_messages");

    // Send and process messages
    send_and_delete_message(
        &queue_client,
        "Example message created from Rust, ready for deletion",
    )
    .await?;

    // Receive messages
    peek_and_receive_messages(&queue_client).await?;

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

/// Generates a random queue name with a suffix to ensure uniqueness.
fn get_random_queue_name() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random_suffix: u32 = rng.gen_range(1000..9999);
    format!("sdk-test-queue-{}", random_suffix)
}

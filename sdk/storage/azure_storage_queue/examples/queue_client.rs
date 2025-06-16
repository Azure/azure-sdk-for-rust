use std::collections::HashMap;

use azure_core::http::StatusCode;
use azure_identity::DefaultAzureCredential;
use azure_storage_queue::QueueClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;

    // Retrieve the storage account endpoint from environment variable.
    let endpoint = std::env::var("AZURE_QUEUE_STORAGE_ACCOUNT")?;

    // Create a QueueClient using the endpoint and credential.
    // Note: Ensure that the endpoint is in the format "https://<account_name>.queue.core.windows.net/"
    if !endpoint.ends_with("/") {
        eprintln!("Endpoint must end with a '/' character.");
        std::process::exit(1);
    }
    if !endpoint.starts_with("https://") {
        eprintln!("Endpoint must start with 'https://'.");
        std::process::exit(1);
    }

    let queue_client = QueueClient::new(&endpoint, credential, None)?;

    // Get the properties of the queue service
    let properties_response = queue_client.get_properties().await;
    match properties_response {
        Ok(response) => {
            let (_status_code, _headers, properties) = response.deconstruct();
            println!(
                "Successfully retrieved properties: {:?}",
                properties.collect_string().await?
            );
        }
        Err(e) => eprintln!("Error retrieving queue properties: {}", e),
    }

    let queue_name = get_random_queue_name();

    // Create a new queue
    let create_response = queue_client.create(queue_name.as_str(), None).await;
    match create_response {
        Ok(response) => println!("Successfully created queue: {:?}", response),
        Err(e) => eprintln!("Error creating queue: {}", e),
    }

    // Check if the queue exists
    let exists_response = queue_client.exists(queue_name.as_str()).await;
    match exists_response {
        Ok(response) => println!("Queue exists: {:?}", response),
        Err(e) => eprintln!("Error checking if queue exists: {}", e),
    }

    // Check a non-existent queue exists
    let non_existent_queue = "non-existent-queue";
    let non_existent_exists_response = queue_client.exists(non_existent_queue).await;
    match non_existent_exists_response {
        Ok(response) => println!("Non-existent queue exists: {:?}", response),
        Err(e) => eprintln!("Error checking non-existent queue: {}", e),
    }

    // Create the queue again with the not exists option
    let create_if_not_exists_response = queue_client
        .create_if_not_exists(queue_name.as_str(), None)
        .await;
    match create_if_not_exists_response {
        Ok(response) => println!(
            "Did not error when creating a queue that already existed: {:?}",
            response
        ),
        Err(e) => eprintln!("Error when creating a queue that already existed: {}", e),
    }

    // Set metadata for the queue
    let metadata = HashMap::from([("key1", "value1"), ("key2", "value2")]);
    let set_metadata_response = queue_client
        .set_metadata(queue_name.as_str(), Some(metadata))
        .await;
    match set_metadata_response {
        Ok(response) => println!("Successfully set metadata: {:?}", response),
        Err(e) => eprintln!("Error setting metadata: {}", e),
    }

    // Delete messages from the queue
    let delete_messages_response = queue_client.delete_messages(queue_name.as_str()).await;
    match delete_messages_response {
        Ok(response) => println!("Successfully deleted messages: {:?}", response),
        Err(e) => {
            if e.http_status() == Some(StatusCode::NotFound) {
                // Handle the case where the queue does not exist
                // This is a common case when trying to delete messages from a queue that has already been deleted.
                println!("Unable to delete messages, queue not found");
            } else if e.http_status() == Some(StatusCode::Forbidden) {
                // Handle the case where the user does not have permission to delete messages
                // This can happen if the credentials used do not have the necessary permissions.
                println!("Unable to delete messages, you do not have permission to delete messages from this queue. Please check your credentials.");
            } else {
                eprintln!("Error deleting messages: {}", e);
            }
        }
    }

    // Send a message to the queue
    let send_message_response = queue_client
        .send_message(
            queue_name.as_str(),
            "Example message created from Rust.",
            None,
        )
        .await;
    match send_message_response {
        Ok(response) => println!("Successfully sent messages: {:?}", response),
        Err(e) => {
            if e.http_status() == Some(StatusCode::NotFound) {
                // Handle the case where the queue does not exist
                // This is a common case when trying to send messages to a queue that has already been deleted.
                println!("Unable to send messages, queue not found");
            } else if e.http_status() == Some(StatusCode::Forbidden) {
                // Handle the case where the user does not have permission to send messages
                // This can happen if the credentials used do not have the necessary permissions.
                println!("Unable to send messages, you do not have permission to send messages to this queue. Please check your credentials.");
            } else {
                eprintln!("Error sending messages: {}", e);
            }
        }
    }

    // Receive messages from the queue
    let receive_message_response = queue_client
        .receive_messages(queue_name.as_str(), None)
        .await;
    match receive_message_response {
        Ok(response) => {
            println!("Successfully received messages: {:?}", response);
            let messages = response.into_body().await?;
            for msg in messages.value.unwrap() {
                println!("Received message: {:?}", msg.message_text.unwrap());
            }
        }
        Err(e) => {
            if e.http_status() == Some(StatusCode::NotFound) {
                // Handle the case where the queue does not exist
                // This is a common case when trying to receive messages from a queue that has already been deleted.
                println!("Unable to receive messages, queue not found");
            } else if e.http_status() == Some(StatusCode::Forbidden) {
                // Handle the case where the user does not have permission to receive messages
                // This can happen if the credentials used do not have the necessary permissions.
                println!("Unable to receive messages, you do not have permission to receive messages from this queue. Please check your credentials.");
            } else {
                eprintln!("Error receiving messages: {}", e);
            }
        }
    }

    // Delete the queue after use
    let delete_response = queue_client.delete(queue_name.as_str(), None).await;
    match delete_response {
        Ok(response) => println!("Successfully deleted queue: {:?}", response),
        Err(e) => eprintln!("Error deleting queue: {}", e),
    }

    // Delete a non-existent queue
    let delete_non_existent_response = queue_client
        .delete_if_exists("non-existent-queue", None)
        .await;
    match delete_non_existent_response {
        Ok(response) => println!(
            "Did not error when deleting non-existent queue: {:?}",
            response
        ),
        Err(e) => eprintln!("Error deleting non-existent queue: {}", e),
    }

    Ok(())
}

/// Generates a random queue name with a suffix to ensure uniqueness.
fn get_random_queue_name() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random_suffix: u32 = rng.gen_range(1000..9999);
    format!("sdk-test-queue-{}", random_suffix)
}

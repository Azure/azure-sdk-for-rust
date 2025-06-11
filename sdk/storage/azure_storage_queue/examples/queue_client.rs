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

    let queue_name = "sdk-test-queue";

    // Create a new queue
    let create_response = queue_client.create(queue_name, None).await;
    println!("Queue created: {:?}", create_response);

    //Delete the queue after use
    let delete_response = queue_client.delete(queue_name, None).await;
    println!("Queue deleted: {:?}", delete_response);
    Ok(())
}

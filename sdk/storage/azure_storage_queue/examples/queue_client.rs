use azure_identity::DefaultAzureCredential;
use azure_storage_queue::QueueClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new().unwrap();

    // let endpoint = std::env::var("AZURE_QUEUE_STORAGE_ACCOUNT").ok().unwrap();
    let endpoint = "https://enzymetestinputstorage.queue.core.windows.net/".to_string();

    let queue_client = QueueClient::new(&endpoint, credential, None).ok().unwrap();

    let queue_name = "sdk-test-queue";

    // Create a new queue
    let create_response = queue_client.create(queue_name, None).await;
    println!("Queue created: {:?}", create_response);
    Ok(())
}

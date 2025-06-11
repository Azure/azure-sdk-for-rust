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
            println!("Queue properties: {:?}", properties.collect_string().await?);
        }
        Err(e) => eprintln!("Error retrieving queue properties: {}", e),
    }

    let queue_name = get_random_queue_name();

    // Create a new queue
    let create_response = queue_client.create(queue_name.as_str(), None).await;
    println!("Queue created: {:?}", create_response);

    //Delete the queue after use
    let delete_response = queue_client.delete(queue_name.as_str(), None).await;
    println!("Queue deleted: {:?}", delete_response);
    Ok(())
}

/// Generates a random queue name with a suffix to ensure uniqueness.
fn get_random_queue_name() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random_suffix: u32 = rng.gen_range(1000..9999);
    format!("sdk-test-queue-{}", random_suffix)
}

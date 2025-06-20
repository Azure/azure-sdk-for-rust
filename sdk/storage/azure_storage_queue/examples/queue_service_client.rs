mod helpers;

use helpers::endpoint::get_endpoint;
use helpers::logs::log_operation_result;
use helpers::random_queue_name::get_random_queue_name;

use azure_core::http::RequestContent;
use azure_identity::DefaultAzureCredential;
use azure_storage_queue::clients::QueueServiceClient;

async fn get_and_set_properties(
    queue_client: &QueueServiceClient,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get queue service properties
    let result = queue_client.get_properties(None).await;
    log_operation_result(&result, "get_properties");

    // Set queue service properties
    let properties = queue_client.get_properties(None).await?.into_body().await?;
    let properties_xml = quick_xml::se::to_string(&properties)?;
    let properties_bytes = properties_xml.into_bytes();

    let result = queue_client
        .set_properties(
            RequestContent::from(properties_bytes),
            "application/xml".to_string(),
            None,
        )
        .await;
    log_operation_result(&result, "set_properties");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;

    // Retrieve the storage account endpoint from environment variable.
    let endpoint = get_endpoint();

    let queue_name = get_random_queue_name();
    let queue_client = QueueServiceClient::new(&endpoint, credential.clone(), None)?;

    // Create and manage queue
    let result = queue_client.create_queue(&queue_name, None).await;
    log_operation_result(&result, "create_queue");

    get_and_set_properties(&queue_client).await?;

    // List queues
    let result = queue_client.list_queues_segment(None);
    log_operation_result(&result, "list_queues_segment");

    // Cleanup
    let result = queue_client.delete_queue(&queue_name, None).await;
    log_operation_result(&result, "delete_queue");

    Ok(())
}

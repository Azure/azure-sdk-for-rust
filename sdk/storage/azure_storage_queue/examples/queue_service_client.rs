mod helpers;

use azure_storage_queue::models::ListQueuesSegmentResponse;
use helpers::endpoint::get_endpoint;
use helpers::logs::log_operation_result;
use helpers::random_queue_name::get_random_queue_name;

use azure_core::http::RequestContent;
use azure_identity::DefaultAzureCredential;
use azure_storage_queue::clients::QueueServiceClient;

use futures::StreamExt;

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
        .set_properties(RequestContent::from(properties_bytes), None)
        .await;
    log_operation_result(&result, "set_properties");

    Ok(())
}

async fn list_queues_segment(
    queue_client: &QueueServiceClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let options =
        azure_storage_queue::models::QueueServiceOperationGroupClientListQueuesSegmentOptions {
            maxresults: Some(1),
            ..Default::default()
        };
    let result = queue_client.list_queues_segment(Some(options));
    log_operation_result(&result, "list_queues_segment");

    if let Ok(mut pager_response) = result {
        while let Some(response_result) = pager_response.next().await {
            println!("Processing next page of queues...");
            match response_result {
                Ok(response) => {
                    let queue_list: ListQueuesSegmentResponse = response.into_body().await?;
                    for queue in queue_list.queue_items {
                        println!("Queue: {}", queue.name.unwrap_or_default());
                    }
                }
                Err(e) => {
                    eprintln!("Error getting queue page: {}", e);
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
    let endpoint = get_endpoint();

    let queue_name = get_random_queue_name();
    let queue_client = QueueServiceClient::new(&endpoint, credential.clone(), None)?;

    // Create and manage queue
    let result = queue_client.create_queue(&queue_name, None).await;
    log_operation_result(&result, "create_queue");

    get_and_set_properties(&queue_client).await?;

    // List queues
    list_queues_segment(&queue_client).await?;

    // Cleanup
    let result = queue_client.delete_queue(&queue_name, None).await;
    log_operation_result(&result, "delete_queue");

    Ok(())
}

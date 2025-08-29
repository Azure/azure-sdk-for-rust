use std::sync::Arc;

use azure_core::{http::StatusCode, Error};
use azure_storage_queue::{
    models::{
        CorsRule, ListQueuesIncludeType, ListQueuesResponse, QueueServiceClientListQueuesOptions,
        QueueServiceProperties,
    },
    QueueServiceClient,
};

use azure_identity::DeveloperToolsCredential;

use futures::StreamExt;

async fn set_and_get_properties(
    queue_client: &QueueServiceClient,
) -> Result<(), Box<dyn std::error::Error>> {
    // Set queue service properties
    let properties = QueueServiceProperties {
        cors: Some(vec![CorsRule {
            allowed_origins: Some("https://example.com".to_string()),
            allowed_methods: Some("GET,POST".to_string()),
            max_age_in_seconds: Some(3600),
            exposed_headers: Some("x-ms-meta-data".to_string()),
            allowed_headers: Some("x-ms-meta-target".to_string()),
        }]),
        ..Default::default()
    };
    let result = queue_client
        .set_properties(properties.try_into()?, None)
        .await;
    log_operation_result(&result, "set_properties");

    // Get queue service properties
    let result = queue_client.get_properties(None).await;
    log_operation_result(&result, "get_properties");

    if let Ok(response) = result {
        let properties = response.into_body().await?;
        println!("Queue Service Properties:");
        println!("Logging: {:#?}", properties.logging);
        println!("Hour Metrics: {:#?}", properties.hour_metrics);
        println!("Minute Metrics: {:#?}", properties.minute_metrics);

        if let Some(cors_rules) = &properties.cors {
            println!("CORS Rules ({} rules):", cors_rules.len());
            for (index, rule) in cors_rules.iter().enumerate() {
                println!("  Rule {}:", index + 1);
                println!("    Allowed Origins: {:?}", rule.allowed_origins);
                println!("    Allowed Methods: {:?}", rule.allowed_methods);
                println!("    Allowed Headers: {:?}", rule.allowed_headers);
                println!("    Exposed Headers: {:?}", rule.exposed_headers);
                println!("    Max Age in Seconds: {:?}", rule.max_age_in_seconds);
            }
        } else {
            println!("CORS Rules: None");
        }
    } else {
        eprintln!("Failed to get queue service properties.");
    }

    Ok(())
}

async fn list_queues(queue_client: &QueueServiceClient) -> Result<(), Box<dyn std::error::Error>> {
    let options = QueueServiceClientListQueuesOptions {
        maxresults: Some(1),
        include: Some(vec![ListQueuesIncludeType::Metadata]), // Include metadata in the response
        ..Default::default()
    };
    let result = queue_client.list_queues(Some(options));
    log_operation_result(&result, "list_queues_segment");

    if let Ok(mut pager_response) = result {
        while let Some(response_result) = pager_response.next().await {
            println!("Processing next page of queues...");
            match response_result {
                Ok(response) => {
                    let queue_list: ListQueuesResponse = response.into_body().await?;
                    for queue in queue_list.queue_items {
                        println!("Queue: {}", queue.name.unwrap_or_default());
                        for (key, value) in queue.metadata.unwrap_or_default() {
                            println!("  Metadata - {}: {}", key, value);
                        }
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

async fn get_statistics(
    credential: Arc<DeveloperToolsCredential>,
) -> Result<(), Box<dyn std::error::Error>> {
    let secondary_endpoint = get_secondary_endpoint();
    let secondary_queue_client =
        QueueServiceClient::new(&secondary_endpoint, credential.clone(), None)?;
    let result = secondary_queue_client.get_statistics(None).await;
    log_operation_result(&result, "get_statistics");

    if let Ok(response) = result {
        let stats = response.into_body().await?;
        let geo_replication = stats.geo_replication.as_ref().unwrap();
        println!(
            "Geo-replication status: {}, Last sync time: {}",
            geo_replication.status.as_ref().unwrap(),
            geo_replication.last_sync_time.unwrap()
        );
    } else {
        eprintln!("Failed to get queue service statistics. Ensure the queue service is geo-replicated and the secondary endpoint is accessible.");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;

    // Retrieve the storage account endpoint from environment variable.
    let endpoint = get_endpoint();

    let queue_name = get_random_queue_name();
    let queue_client = QueueServiceClient::new(&endpoint, credential.clone(), None)?;

    // Create and manage queue
    let result = queue_client.create_queue(&queue_name, None).await;
    log_operation_result(&result, "create_queue");

    set_and_get_properties(&queue_client).await?;

    // List queues
    list_queues(&queue_client).await?;

    // Get statistics
    get_statistics(credential.clone()).await?;

    // Cleanup
    let result = queue_client.delete_queue(&queue_name, None).await;
    log_operation_result(&result, "delete_queue");

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

fn get_secondary_endpoint() -> String {
    // Retrieve the storage account endpoint from environment variable.
    let storage_account_name = std::env::var("AZURE_QUEUE_STORAGE_ACCOUNT_NAME");
    let storage_account_name = match storage_account_name {
        Ok(url) => url,
        Err(_) => {
            eprintln!("Environment variable AZURE_QUEUE_STORAGE_ACCOUNT_NAME is not set");
            std::process::exit(1);
        }
    };

    format!(
        "https://{}-secondary.queue.core.windows.net/",
        storage_account_name
    )
}

fn get_random_queue_name() -> String {
    use rand::Rng;
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

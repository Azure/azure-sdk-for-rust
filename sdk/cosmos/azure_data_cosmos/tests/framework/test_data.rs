use azure_core::http::StatusCode;
use azure_data_cosmos::options::CreateContainerOptions;
use azure_data_cosmos::{
    clients::{ContainerClient, DatabaseClient},
    models::{ContainerProperties, ThroughputProperties},
};
use std::time::Duration;

use super::test_client::{probe_data_plane_ready, AuthMode};
use super::MockItem;

const ITEMS_PER_PARTITION: usize = 10;
const PARTITION_COUNT: usize = 10;

pub fn generate_mock_item(partition_index: usize, item_index: usize) -> MockItem {
    let partition_key = format!("partition{}", partition_index);
    MockItem {
        id: format!("{}", partition_index * ITEMS_PER_PARTITION + item_index),
        partition_key: partition_key.clone(),
        merge_order: partition_index + item_index * PARTITION_COUNT,
    }
}

pub fn generate_mock_items(partition_count: usize, items_per_partition: usize) -> Vec<MockItem> {
    let mut items = Vec::new();
    for i in 0..partition_count {
        for j in 0..items_per_partition {
            items.push(generate_mock_item(i, j));
        }
    }
    items
}

/// Creates a batch of simple mock items
pub async fn create_container_with_items(
    db: &DatabaseClient,
    items: Vec<MockItem>,
    throughput: Option<ThroughputProperties>,
) -> azure_data_cosmos::Result<ContainerClient> {
    let properties = ContainerProperties::new("TestContainer", "/partitionKey".into());

    // Retry on 429 errors
    loop {
        match db
            .create_container(
                properties.clone(),
                throughput.clone().map(|throughput| {
                    CreateContainerOptions::default().with_throughput(throughput)
                }),
            )
            .await
        {
            Ok(_) => break,
            Err(e) if e.status().status_code() == StatusCode::TooManyRequests => {
                println!("Create container got 429 (Too Many Requests). Retrying...");
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Err(e) if e.status().status_code() == StatusCode::Conflict => {
                // Container already exists, continue
                break;
            }
            Err(e) => return Err(e),
        }
    }

    let container_client = db.container_client("TestContainer").await?;

    // Under AAD, the first data-plane request against a freshly created
    // container can race with 403/5302 RbacUnauthorizedNameBasedDataRequest
    // before the name→RID mapping is cached on this client's connection.
    // Probe once so the item-seeding loop below doesn't burn its budget
    // being retried by the driver.
    if AuthMode::from_env() == AuthMode::Aad {
        probe_data_plane_ready("test_data::create_container_with_items", &container_client).await?;
    }

    for item in items {
        let item_id = item.id.clone();
        container_client
            .create_item(item.partition_key.clone(), &item_id, item, None)
            .await?;
    }

    Ok(container_client)
}

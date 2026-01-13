use azure_data_cosmos::{
    clients::{ContainerClient, DatabaseClient},
    models::{ContainerProperties, ThroughputProperties},
    CreateContainerOptions,
};

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
) -> azure_core::Result<ContainerClient> {
    let properties = ContainerProperties {
        id: "TestContainer".into(),
        partition_key: "/partitionKey".into(),
        ..Default::default()
    };
    db.create_container(
        properties,
        Some(CreateContainerOptions {
            throughput,
            ..Default::default()
        }),
    )
    .await?;

    let container_client = db.container_client("TestContainer");

    for item in items {
        container_client
            .create_item(item.partition_key.clone(), item, None)
            .await?;
    }

    Ok(container_client)
}

use azure_data_cosmos::{
    clients::{ContainerClient, DatabaseClient},
    models::{ContainerProperties, ThroughputProperties},
    CosmosClient, CreateContainerOptions,
};

use super::{MockItem, TestAccount};

const ITEMS_PER_PARTITION: usize = 10;
const PARTITION_COUNT: usize = 10;

pub fn generate_mock_items(
    partition_indexes: impl Iterator<Item = usize>,
    item_indexes: impl Iterator<Item = usize>,
) -> Vec<MockItem> {
    let mut items = Vec::new();
    for i in partition_indexes {
        let partition_key = format!("partition{}", partition_index);
        for j in item_indexes {
            items.push(MockItem {
                id: format!("{}", partition_index * ITEMS_PER_PARTITION + item_index),
                partition_key: partition_key.clone(),
                merge_order: partition_index + item_index * PARTITION_COUNT,
            })
        }
    }
}

/// Creates a batch of simple mock items
pub async fn create_container_with_items(
    db: DatabaseClient,
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

    for i in 0..PARTITION_COUNT {
        for j in 0..ITEMS_PER_PARTITION {
            let item = generate_mock_item(i, j);
            container_client
                .create_item(item.partition_key.clone(), item, None)
                .await?;
        }
    }

    Ok(container_client)
}

pub async fn create_database(
    account: &TestAccount,
    cosmos_client: &CosmosClient,
) -> azure_core::Result<DatabaseClient> {
    // The TestAccount has a unique context_id that includes the test name.
    let db_name = account.unique_db("TestData");
    let props = cosmos_client
        .create_database(&db_name, None)
        .await?
        .into_body()
        .await?;

    let db_client = cosmos_client.database_client(&props.id);
    Ok(db_client)
}

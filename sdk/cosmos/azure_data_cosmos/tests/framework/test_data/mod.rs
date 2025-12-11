use azure_core::http::StatusCode;
use azure_data_cosmos::{
    clients::{ContainerClient, DatabaseClient},
    models::{ContainerProperties, ThroughputProperties},
    CosmosClient, CreateContainerOptions,
};

use super::{MockItem, TestAccount};

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
    db: DatabaseClient,
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

pub async fn create_database(
    account: &TestAccount,
    cosmos_client: &CosmosClient,
) -> azure_core::Result<DatabaseClient> {
    // The TestAccount has a unique context_id that includes the test name.
    let db_name = account.db_name();
    let response = match cosmos_client.create_database(&db_name, None).await {
        // The database creation was successful.
        Ok(props) => props,
        Err(e) if e.http_status() == Some(StatusCode::Conflict) => {
            // The database already exists, from a previous test run.
            // Delete it and re-create it.
            let db_client = cosmos_client.database_client(&db_name);
            db_client.delete(None).await?;

            // Re-create the database.

            cosmos_client.create_database(&db_name, None).await?
        }
        Err(e) => {
            // Some other error occurred.
            return Err(e);
        }
    };

    let props = response.into_model()?;

    let db_client = cosmos_client.database_client(&props.id);
    Ok(db_client)
}

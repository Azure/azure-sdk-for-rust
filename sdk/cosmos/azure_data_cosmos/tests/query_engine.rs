#![cfg(feature = "query_engine")]

mod framework;

use azure_core_test::{recorded, TestContext};
use azure_data_cosmos::{
    clients::{ContainerClient, DatabaseClient},
    models::{ContainerProperties, ThroughputProperties},
    CreateContainerOptions, QueryOptions,
};
use framework::{query_engine::MockItem, TestAccount};

const RU_COUNT: usize = 40000;
const ITEMS_PER_PARTITION: usize = 10;

#[recorded::test]
pub async fn query_via_query_engine(
    context: TestContext,
) -> Result<(), Box<dyn std::error::Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;

    let test_db_id = account.unique_db("QueryViaQueryEngine");

    // Create a database
    cosmos_client
        .create_database(&test_db_id, None)
        .await?
        .into_body()
        .await?;
    let db_client = cosmos_client.database_client(&test_db_id);

    // let container = create_test_items(&db_client).await?;
    // let pager = container.query_items(
    //     "SELECT * FROM c ORDER BY c.merge_order",
    //     None,
    //     Some(QueryOptions {
    //         query_engine: todo!(),
    //         ..Default::default()
    //     }),
    // )?;

    todo!()
}

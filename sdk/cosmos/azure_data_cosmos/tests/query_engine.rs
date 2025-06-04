#![cfg(feature = "preview_query_engine")]

mod framework;

use std::sync::Arc;

use azure_core::error::{Error, ErrorKind};
use azure_core_test::{recorded, TestContext};
use azure_data_cosmos::{models::ThroughputProperties, QueryOptions};
use framework::{query_engine::MockQueryEngine, test_data, MockItem, TestAccount};
use futures::TryStreamExt;

#[recorded::test]
pub async fn create_errors_in_query_engine_appear_in_first_result(
    context: TestContext,
) -> Result<(), Box<dyn std::error::Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let db_client = test_data::create_database(&account, &cosmos_client).await?;
    let items = test_data::generate_mock_items(1, 1);
    let container_client = test_data::create_container_with_items(
        db_client,
        items.clone(),
        Some(ThroughputProperties::manual(40000)), // Force multiple physical partitions
    )
    .await?;

    let query_engine = Arc::new(MockQueryEngine::with_error(Error::message(
        ErrorKind::Other,
        "Mock error",
    )));

    let mut results = container_client.query_items::<MockItem>(
        "select * from c order by c.mergeOrder",
        (),
        Some(QueryOptions {
            query_engine: Some(query_engine),
            ..Default::default()
        }),
    )?;
    let err = results.try_next().await.unwrap_err();
    assert_eq!("Mock error", format!("{}", err));

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn query_via_query_engine(
    context: TestContext,
) -> Result<(), Box<dyn std::error::Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let db_client = test_data::create_database(&account, &cosmos_client).await?;
    let mut items = test_data::generate_mock_items(10, 10);
    let container_client = test_data::create_container_with_items(
        db_client,
        items.clone(),
        Some(ThroughputProperties::manual(40000)), // Force multiple physical partitions
    )
    .await?;

    let query_engine = Arc::new(MockQueryEngine::new());

    let result_items: Vec<MockItem> = container_client
        .query_items(
            "select * from c order by c.mergeOrder",
            (),
            Some(QueryOptions {
                query_engine: Some(query_engine),
                ..Default::default()
            }),
        )?
        .try_collect()
        .await?;
    items.sort_by_key(|p| p.merge_order); // Sort the expected items by merge order, to match what the results should be
    assert_eq!(items, result_items);

    account.cleanup().await?;
    Ok(())
}

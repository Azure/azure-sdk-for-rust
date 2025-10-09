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

    let query_engine = Arc::new(MockQueryEngine::with_error(Error::with_message(
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

#[recorded::test]
pub async fn query_override_without_parameters(
    context: TestContext,
) -> Result<(), Box<dyn std::error::Error>> {
    use framework::query_engine::{MockQueryEngine, QueryRequestConfig};

    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let db_client = test_data::create_database(&account, &cosmos_client).await?;
    let items = test_data::generate_mock_items(5, 5);
    let container_client = test_data::create_container_with_items(
        db_client,
        items.clone(),
        Some(ThroughputProperties::manual(40000)), // Force multiple physical partitions
    )
    .await?;

    let query_config = QueryRequestConfig {
        query: Some("SELECT * FROM c WHERE c.id = 'override'".to_string()),
        include_parameters: false,
    };
    let query_engine = Arc::new(MockQueryEngine::with_query_request_config(query_config));

    let original_query = azure_data_cosmos::Query::from("SELECT * FROM c WHERE c.id = @param1")
        .with_parameter("@param1", "should_not_be_used")?;

    let result_items: Vec<MockItem> = container_client
        .query_items(
            original_query,
            (),
            Some(QueryOptions {
                query_engine: Some(query_engine),
                ..Default::default()
            }),
        )?
        .try_collect()
        .await?;

    // Since the override query looks for id = 'override' and our test data doesn't have items
    // with that id, we should get no results. This proves the override query was used without parameters.
    assert_eq!(0, result_items.len());

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn query_override_with_parameters(
    context: TestContext,
) -> Result<(), Box<dyn std::error::Error>> {
    use framework::query_engine::{MockQueryEngine, QueryRequestConfig};

    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let db_client = test_data::create_database(&account, &cosmos_client).await?;
    let items = test_data::generate_mock_items(5, 5);
    let container_client = test_data::create_container_with_items(
        db_client,
        items.clone(),
        Some(ThroughputProperties::manual(40000)), // Force multiple physical partitions
    )
    .await?;

    let query_config = QueryRequestConfig {
        query: Some("SELECT * FROM c WHERE c.mergeOrder = @targetOrder".to_string()),
        include_parameters: true,
    };
    let query_engine = Arc::new(MockQueryEngine::with_query_request_config(query_config));

    let target_merge_order = items[0].merge_order;

    let original_query =
        azure_data_cosmos::Query::from("SELECT * FROM c WHERE c.id = @targetOrder")
            .with_parameter("@targetOrder", target_merge_order)?;

    let result_items: Vec<MockItem> = container_client
        .query_items(
            original_query,
            (),
            Some(QueryOptions {
                query_engine: Some(query_engine),
                ..Default::default()
            }),
        )?
        .try_collect()
        .await?;

    // Since the override query uses "c.mergeOrder = @targetOrder" and we passed a valid merge order,
    // we should get exactly the items that match that merge order. This proves the override query
    // was used with the original parameters.
    let expected_items: Vec<MockItem> = items
        .into_iter()
        .filter(|item| item.merge_order == target_merge_order)
        .collect();

    assert_eq!(expected_items.len(), result_items.len());
    assert!(
        !expected_items.is_empty(),
        "Should have found at least one matching item"
    );

    for expected_item in expected_items {
        assert!(result_items.contains(&expected_item));
    }

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn no_query_override_uses_original(
    context: TestContext,
) -> Result<(), Box<dyn std::error::Error>> {
    use framework::query_engine::{MockQueryEngine, QueryRequestConfig};

    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let db_client = test_data::create_database(&account, &cosmos_client).await?;
    let items = test_data::generate_mock_items(5, 5);
    let container_client = test_data::create_container_with_items(
        db_client,
        items.clone(),
        Some(ThroughputProperties::manual(40000)), // Force multiple physical partitions
    )
    .await?;

    let query_config = QueryRequestConfig {
        query: None,
        include_parameters: false,
    };
    let query_engine = Arc::new(MockQueryEngine::with_query_request_config(query_config));

    let target_merge_order = items[0].merge_order;

    let original_query =
        azure_data_cosmos::Query::from("SELECT * FROM c WHERE c.mergeOrder = @targetOrder")
            .with_parameter("@targetOrder", target_merge_order)?;

    let result_items: Vec<MockItem> = container_client
        .query_items(
            original_query,
            (),
            Some(QueryOptions {
                query_engine: Some(query_engine),
                ..Default::default()
            }),
        )?
        .try_collect()
        .await?;

    // Since there's no query override, the original query "c.mergeOrder = @targetOrder" should be used
    // with the original parameters, so we should get items matching the target merge order.
    // This proves the original query and parameters were both preserved.
    let expected_items: Vec<MockItem> = items
        .into_iter()
        .filter(|item| item.merge_order == target_merge_order)
        .collect();

    assert_eq!(expected_items.len(), result_items.len());
    assert!(
        !expected_items.is_empty(),
        "Should have found at least one matching item"
    );

    for expected_item in expected_items {
        assert!(result_items.contains(&expected_item));
    }

    account.cleanup().await?;
    Ok(())
}

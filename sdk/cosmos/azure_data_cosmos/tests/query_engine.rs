#![cfg(feature = "preview_query_engine")]

mod framework;

use std::sync::Arc;

use azure_core::error::{Error, ErrorKind};
use azure_data_cosmos::{models::ThroughputProperties, Query, QueryOptions};
use framework::{query_engine::MockQueryEngine, test_data, MockItem, TestClient};
use futures::TryStreamExt;

use crate::framework::query_engine::QueryRequestConfig;

#[tokio::test]
pub async fn create_errors_in_query_engine_appear_in_first_result(
) -> Result<(), Box<dyn std::error::Error>> {
    TestClient::run_with_db(async |_, db_client| {
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

        Ok(())
    })
    .await
}

#[tokio::test]
pub async fn no_query_override_uses_original() -> Result<(), Box<dyn std::error::Error>> {
    TestClient::run_with_db(async |_, db_client| {
        let items = test_data::generate_mock_items(5, 2);
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
        let original_query = Query::from("SELECT * FROM c WHERE c.mergeOrder = @targetOrder")
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
        let expected_items: Vec<MockItem> = items
            .into_iter()
            .filter(|item| item.merge_order == target_merge_order)
            .collect();
        assert_eq!(expected_items, result_items);

        Ok(())
    })
    .await
}

#[tokio::test]
pub async fn query_override_without_parameters() -> Result<(), Box<dyn std::error::Error>> {
    TestClient::run_with_db(async |_, db_client| {
        let items = test_data::generate_mock_items(5, 2);
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

        Ok(())
    })
    .await
}

#[tokio::test]
pub async fn query_override_with_parameters() -> Result<(), Box<dyn std::error::Error>> {
    TestClient::run_with_db(async |_, db_client| {
        let items = test_data::generate_mock_items(5, 2);
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

        Ok(())
    })
    .await
}

#[tokio::test]
pub async fn no_global_query_with_override() -> Result<(), Box<dyn std::error::Error>> {
    TestClient::run_with_db(async |_, db_client| {
        let items = test_data::generate_mock_items(5, 2);
        let container_client = test_data::create_container_with_items(
            db_client,
            items.clone(),
            Some(ThroughputProperties::manual(40000)), // Force multiple physical partitions
        )
        .await?;

        // Configure a query override that will be used in all QueryRequests
        let query_config = QueryRequestConfig {
            query: Some("SELECT * FROM c WHERE c.mergeOrder = @targetOrder".to_string()),
            include_parameters: true,
        };

        // Create a MockQueryEngine with NO top-level global query (rewritten_query = None)
        let mut query_engine = MockQueryEngine::with_rewritten_query(None);
        query_engine.query_request_config = std::sync::Mutex::new(Some(query_config));
        let query_engine = Arc::new(query_engine);

        let target_merge_order = items[0].merge_order;

        // The original query doesn't matter since there's no global query in the pipeline
        // and we're using an override
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

        // The override query uses "c.mergeOrder = @targetOrder" (not "c.id = @targetOrder"),
        // so we should get items matching the target merge order.
        // This proves that:
        // 1. The pipeline has no global query (query() returns None)
        // 2. The QueryRequest always contains the override query
        // 3. The override query is used for execution
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

        Ok(())
    })
    .await
}

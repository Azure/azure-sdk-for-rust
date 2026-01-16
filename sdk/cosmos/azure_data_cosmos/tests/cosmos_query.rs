#![cfg(feature = "key_auth")]

use std::error::Error;

use azure_data_cosmos::Query;
use framework::{test_data, MockItem, TestClient};
use futures::TryStreamExt;

mod framework;

fn collect_matching_items(
    items: &[MockItem],
    predicate: impl Fn(&MockItem) -> bool,
) -> Vec<MockItem> {
    items.iter().filter(|p| predicate(p)).cloned().collect()
}

#[tokio::test]
pub async fn single_partition_query_simple() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_db(async |_, db_client| {
        let items = test_data::generate_mock_items(2, 2);
        let container_client =
            test_data::create_container_with_items(db_client, items.clone(), None).await?;

        let result_items: Vec<MockItem> = container_client
            .query_items("select * from docs c", "partition0", None)?
            .try_collect()
            .await?;
        assert_eq!(
            collect_matching_items(&items, |p| p.partition_key == "partition0"),
            result_items
        );

        Ok(())
    })
    .await
}

#[tokio::test]
pub async fn single_partition_query_with_parameters() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_db(async |_, db_client| {
        let items = test_data::generate_mock_items(2, 2);
        let container_client =
            test_data::create_container_with_items(db_client, items.clone(), None).await?;

        // Find a merge order value in partition1's items
        let merge_order = items
            .iter()
            .find(|p| p.partition_key == "partition1")
            .expect("No items in partition1")
            .merge_order;

        // Query for items with that merge order
        let query = Query::from("select * from c where c.mergeOrder = @some_value")
            .with_parameter("@some_value", merge_order)?;
        let result_items: Vec<MockItem> = container_client
            .query_items(query, "partition1", None)?
            .try_collect()
            .await?;
        assert_eq!(
            collect_matching_items(&items, |p| p.merge_order == merge_order),
            result_items
        );

        Ok(())
    })
    .await
}

#[tokio::test]
pub async fn single_partition_query_with_projection() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_db(async |_, db_client| {
        let items = test_data::generate_mock_items(2, 2);
        let container_client =
            test_data::create_container_with_items(db_client, items.clone(), None).await?;

        let result_items: Vec<String> = container_client
            .query_items("select value c.id from c", "partition1", None)?
            .try_collect()
            .await?;
        assert_eq!(
            items
                .iter()
                .filter(|p| p.partition_key == "partition1")
                .map(|p| p.id.to_string())
                .collect::<Vec<_>>(),
            result_items
        );

        Ok(())
    })
    .await
}

#[tokio::test]
pub async fn cross_partition_query_with_projection_and_filter() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_db(async |_, db_client| {
        let items = test_data::generate_mock_items(10, 2);
        let container_client =
            test_data::create_container_with_items(db_client, items.clone(), None).await?;

        let result_items: Vec<String> = container_client
            .query_items(
                "select value c.id from c where c.mergeOrder between 40 and 60",
                (),
                None,
            )?
            .try_collect()
            .await?;
        assert_eq!(
            items
                .iter()
                .filter(|p| p.merge_order >= 40 && p.merge_order <= 60)
                .map(|p| p.id.to_string())
                .collect::<Vec<_>>(),
            result_items
        );

        Ok(())
    })
    .await
}

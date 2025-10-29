#![cfg(feature = "key_auth")]

use std::error::Error;

use azure_core::http::StatusCode;
use azure_core_test::{recorded, TestContext};
use azure_data_cosmos::{constants, Query};
use framework::{test_data, MockItem, TestAccount};
use futures::TryStreamExt;

mod framework;

fn collect_matching_items(
    items: &[MockItem],
    predicate: impl Fn(&MockItem) -> bool,
) -> Vec<MockItem> {
    items.iter().filter(|p| predicate(p)).cloned().collect()
}

#[recorded::test]
pub async fn single_partition_query(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let db_client = test_data::create_database(&account, &cosmos_client).await?;
    let items = test_data::generate_mock_items(10, 10);
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

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn single_partition_query_with_parameters(
    context: TestContext,
) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let db_client = test_data::create_database(&account, &cosmos_client).await?;
    let items = test_data::generate_mock_items(10, 10);
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

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn single_partition_query_with_projection(
    context: TestContext,
) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let db_client = test_data::create_database(&account, &cosmos_client).await?;
    let items = test_data::generate_mock_items(10, 10);
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

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn cross_partition_query_with_projection_and_filter(
    context: TestContext,
) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let db_client = test_data::create_database(&account, &cosmos_client).await?;
    let items = test_data::generate_mock_items(10, 10);
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

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn cross_partition_query_with_order_by_fails_without_query_engine(
    context: TestContext,
) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let db_client = test_data::create_database(&account, &cosmos_client).await?;
    let items = test_data::generate_mock_items(10, 10);
    let container_client =
        test_data::create_container_with_items(db_client, items.clone(), None).await?;

    let mut pager = container_client.query_items::<String>(
        "select value c.id from c order by c.mergeOrder",
        (),
        None,
    )?;
    let result = pager.try_next().await;

    let Err(err) = result else {
        panic!("expected an error but got a successful result");
    };
    assert_eq!(Some(StatusCode::BadRequest), err.http_status());

    let response =
        if let azure_core::error::ErrorKind::HttpResponse { raw_response, .. } = err.kind() {
            raw_response.as_ref().unwrap().clone()
        } else {
            panic!("expected an HTTP response error");
        };
    let sub_status = response.headers().get_optional_str(&constants::SUB_STATUS);

    // 1004 = CrossPartitionQueryNotServable
    assert_eq!(Some("1004"), sub_status);

    account.cleanup().await?;
    Ok(())
}

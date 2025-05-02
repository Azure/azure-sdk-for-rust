#![cfg(feature = "key_auth")]

use std::{borrow::Cow, error::Error};

use azure_core_test::{recorded, TestContext};
use azure_data_cosmos::Query;
use framework::{test_data, MockItem, TestAccount};
use futures::TryStreamExt;

mod framework;

#[recorded::test]
pub async fn single_partition_query(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let db_client = test_data::create_database(&account, &cosmos_client).await?;
    let container_client = test_data::create_container_with_items(db_client, None).await?;

    let mut results =
        container_client.query_items("select * from docs c", Some("partition0"), None)?;
    let mut items: Vec<MockItem> = Vec::new();
    while let Some(page) = results.try_next().await? {
        items.extend(page.into_items());
    }
    assert_eq!(TEST_DATA[0..2].to_vec(), items);

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn single_partition_query_with_parameters(
    context: TestContext,
) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let container_client = create_container(&account, &cosmos_client).await?;

    let query = Query::from("select * from c where c.some_value = @some_value")
        .with_parameter("@some_value", 2)?;
    let mut results = container_client.query_items(query, Some("Partition1"), None)?;
    let mut items: Vec<TestItem> = Vec::new();
    while let Some(page) = results.try_next().await? {
        items.extend(page.into_items());
    }
    assert_eq!(TEST_DATA[1..2].to_vec(), items);

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn single_partition_query_with_projection(
    context: TestContext,
) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let container_client = create_container(&account, &cosmos_client).await?;

    let mut results =
        container_client.query_items("select value c.id from c", Some("Partition1"), None)?;
    let mut items: Vec<Cow<'static, str>> = Vec::new();
    while let Some(page) = results.try_next().await? {
        items.extend(page.into_items());
    }
    assert_eq!(
        TEST_DATA[0..2]
            .iter()
            .map(|t| t.id.clone())
            .collect::<Vec<_>>(),
        items
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
    let container_client = create_container(&account, &cosmos_client).await?;

    let mut results =
        container_client.query_items("select value c.id from c", Some("Partition1"), None)?;
    let mut items: Vec<Cow<'static, str>> = Vec::new();
    while let Some(page) = results.try_next().await? {
        items.extend(page.into_items());
    }
    assert_eq!(
        TEST_DATA[0..2]
            .iter()
            .map(|t| t.id.clone())
            .collect::<Vec<_>>(),
        items
    );

    account.cleanup().await?;
    Ok(())
}

#![cfg(feature = "key_auth")]

use std::{borrow::Cow, error::Error};

use azure_core_test::{recorded, TestContext};
use azure_data_cosmos::{
    clients::ContainerClient, models::ContainerProperties, CosmosClient, Query,
};
use framework::TestAccount;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};

mod framework;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct TestItem {
    id: Cow<'static, str>,
    partition_key: Option<Cow<'static, str>>,
    some_value: usize,
}

const TEST_DATA: [TestItem; 4] = [
    TestItem {
        id: Cow::Borrowed("Item1"),
        partition_key: Some(Cow::Borrowed("Partition1")),
        some_value: 1,
    },
    TestItem {
        id: Cow::Borrowed("Item2"),
        partition_key: Some(Cow::Borrowed("Partition1")),
        some_value: 2,
    },
    TestItem {
        id: Cow::Borrowed("Item3"),
        partition_key: Some(Cow::Borrowed("Partition2")),
        some_value: 3,
    },
    TestItem {
        id: Cow::Borrowed("Item4"),
        partition_key: Some(Cow::Borrowed("Partition2")),
        some_value: 4,
    },
];

async fn create_container(
    account: &TestAccount,
    cosmos_client: &CosmosClient,
) -> azure_core::Result<ContainerClient> {
    let test_db_id = account.unique_db("ItemCRUD");

    // Create a database and a container
    cosmos_client.create_database(&test_db_id, None).await?;
    let db_client = cosmos_client.database_client(&test_db_id);
    db_client
        .create_container(
            ContainerProperties {
                id: "Container".into(),
                partition_key: "/partition_key".into(),
                ..Default::default()
            },
            None,
        )
        .await?;
    let container_client = db_client.container_client("Container");
    // Create some items
    for item in TEST_DATA.iter() {
        let pk = item.partition_key.clone().unwrap();
        container_client.create_item(pk, &item, None).await?;
    }

    Ok(container_client)
}

#[recorded::test(live)]
pub async fn single_partition_query(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None)?;
    let cosmos_client = account.connect_with_key(None)?;
    let container_client = create_container(&account, &cosmos_client).await?;

    let mut results = container_client.query_items("select * from docs c", "Partition1", None)?;
    let mut items: Vec<TestItem> = Vec::new();
    while let Some(response) = results.try_next().await? {
        items.extend(response.into_body().await?.items);
    }
    assert_eq!(TEST_DATA[0..2].to_vec(), items);

    account.cleanup().await?;
    Ok(())
}

#[recorded::test(live)]
pub async fn single_partition_query_with_parameters(
    context: TestContext,
) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None)?;
    let cosmos_client = account.connect_with_key(None)?;
    let container_client = create_container(&account, &cosmos_client).await?;

    let query = Query::from("select * from c where c.some_value = @some_value")
        .with_parameter("@some_value", 2)?;
    let mut results = container_client.query_items(query, "Partition1", None)?;
    let mut items: Vec<TestItem> = Vec::new();
    while let Some(response) = results.try_next().await? {
        items.extend(response.into_body().await?.items);
    }
    assert_eq!(TEST_DATA[1..2].to_vec(), items);

    account.cleanup().await?;
    Ok(())
}

#[recorded::test(live)]
pub async fn single_partition_query_with_projection(
    context: TestContext,
) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None)?;
    let cosmos_client = account.connect_with_key(None)?;
    let container_client = create_container(&account, &cosmos_client).await?;

    let mut results =
        container_client.query_items("select value c.id from c", "Partition1", None)?;
    let mut items: Vec<Cow<'static, str>> = Vec::new();
    while let Some(response) = results.try_next().await? {
        items.extend(response.into_body().await?.items);
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

#![cfg(feature = "key_auth")]

mod framework;

use std::error::Error;

use azure_core_test::{recorded, TestContext};
use azure_data_cosmos::{models::ThroughputProperties, CreateDatabaseOptions, Query};
use framework::TestAccount;
use futures::TryStreamExt;

#[recorded::test]
pub async fn database_crud(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;

    let test_db_id = account.unique_db("DatabaseCRUD");

    // Create a database
    let properties = cosmos_client
        .create_database(&test_db_id, None)
        .await?
        .into_body()
        .await?;

    assert_eq!(&test_db_id, &properties.id);

    let db_client = cosmos_client.database_client(&test_db_id);
    let read_properties = db_client.read(None).await?.into_body().await?;

    assert_eq!(&test_db_id, &read_properties.id);

    let query =
        Query::from("SELECT * FROM root r WHERE r.id = @id").with_parameter("@id", &test_db_id)?;
    let mut pager = cosmos_client.query_databases(query.clone(), None)?;
    let mut ids = Vec::new();
    while let Some(db) = pager.try_next().await? {
        ids.push(db.id);
    }
    assert_eq!(vec![test_db_id.clone()], ids);

    let current_throughput = db_client.read_throughput(None).await?;
    assert!(current_throughput.is_none());

    // We're testing delete, so we want to manually delete the DB rather than letting the clean-up process do it.
    db_client.delete(None).await?;

    let mut pager = cosmos_client.query_databases(query, None)?;
    let mut ids = Vec::new();
    while let Some(db) = pager.try_next().await? {
        ids.push(db.id);
    }
    assert!(ids.is_empty());

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
#[cfg(feature = "key_auth")]
pub async fn database_with_offer_crud(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;

    let test_db_id = account.unique_db("DatabaseWithOfferCRUD");
    let throughput = ThroughputProperties::manual(400);

    // Create a database
    let properties = cosmos_client
        .create_database(
            &test_db_id,
            Some(CreateDatabaseOptions {
                throughput: Some(throughput),
                ..Default::default()
            }),
        )
        .await?
        .into_body()
        .await?;

    assert_eq!(&test_db_id, &properties.id);

    let db_client = cosmos_client.database_client(&test_db_id);
    let read_properties = db_client.read(None).await?.into_body().await?;
    assert_eq!(&test_db_id, &read_properties.id);

    // Read and then replace throughput
    let current_throughput = db_client
        .read_throughput(None)
        .await?
        .ok_or("expected a throughput offer")?
        .into_body()
        .await?;
    assert_eq!(Some(400), current_throughput.throughput());
    assert!(current_throughput.autoscale_increment().is_none());
    assert!(current_throughput.autoscale_maximum().is_none());

    let new_throughput = db_client
        .replace_throughput(ThroughputProperties::manual(500), None)
        .await?
        .into_body()
        .await?;
    assert_eq!(Some(500), new_throughput.throughput());
    assert!(new_throughput.autoscale_increment().is_none());
    assert!(new_throughput.autoscale_maximum().is_none());

    account.cleanup().await?;
    Ok(())
}

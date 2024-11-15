mod framework;

use std::error::Error;

use azure_data_cosmos::Query;
use framework::TestAccount;
use futures::StreamExt;

#[tokio::test]
#[cfg_attr(not(livetest), ignore)]
pub async fn database_crud() -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env()?;
    let cosmos_client = account.connect_with_key(None)?;

    let test_db_id = account.unique_db("DatabaseCRUD");

    // Create a database
    let properties = cosmos_client
        .create_database(&test_db_id, None)
        .await?
        .deserialize_body()
        .await?
        .unwrap();

    assert_eq!(&test_db_id, &properties.id);

    let db_client = cosmos_client.database_client(&test_db_id);
    let read_properties = db_client.read(None).await?.deserialize_body().await?;

    assert_eq!(&test_db_id, &read_properties.id);

    let query =
        Query::from("SELECT * FROM root r WHERE r.id = @id").with_parameter("@id", &test_db_id)?;
    let mut pager = cosmos_client.query_databases(query.clone(), None)?;
    let mut ids = Vec::new();
    while let Some(page) = pager.next().await {
        let results = page?.deserialize_body().await?;
        for db in results.databases {
            ids.push(db.id);
        }
    }
    assert_eq!(vec![test_db_id.clone()], ids);

    // TODO: Read Throughput, once those APIs exist.

    // We're testing delete, so we want to manually delete the DB rather than letting the clean-up process do it.
    db_client.delete(None).await?;

    let mut pager = cosmos_client.query_databases(query, None)?;
    let mut ids = Vec::new();
    while let Some(page) = pager.next().await {
        let results = page?.deserialize_body().await?;
        for db in results.databases {
            ids.push(db.id);
        }
    }
    assert!(ids.is_empty());

    account.cleanup().await?;
    Ok(())
}

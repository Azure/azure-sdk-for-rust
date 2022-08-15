mod setup_mock;

use futures::StreamExt;

#[tokio::test]
async fn database_operations() -> azure_core::Result<()> {
    const DATABASE_NAME: &str = "cosmos-test-db-create-and-delete-database";

    let client = setup_mock::initialize("database_operations")?;

    // list existing databases and remember their number
    let databases = client
        .list_databases()
        .into_stream()
        .next()
        .await
        .unwrap()?;
    let database_count_before = databases.databases.len();

    // create a new database and check if the number of DBs increased
    let database = client.create_database(DATABASE_NAME).into_future().await?;

    let databases = client
        .list_databases()
        .into_stream()
        .next()
        .await
        .unwrap()?;

    assert!(databases.databases.len() == database_count_before + 1);

    // get the previously created database
    let database_after_get = client
        .database_client(DATABASE_NAME)
        .get_database()
        .into_future()
        .await?;

    assert!(database.database.rid == database_after_get.database.rid);

    // delete the database
    client
        .database_client(DATABASE_NAME)
        .delete_database()
        .into_future()
        .await?;

    let databases = client
        .list_databases()
        .into_stream()
        .next()
        .await
        .unwrap()?;

    assert!(databases.databases.len() == database_count_before);

    Ok(())
}

#![cfg(all(test, feature = "test_e2e"))]

mod setup;

use azure_core::prelude::*;
use azure_data_cosmos::prelude::*;
use futures::stream::StreamExt;

#[tokio::test]
async fn create_and_delete_database() {
    const DATABASE_NAME: &str = "cosmos-test-db-create-and-delete-database";

    let client = setup::initialize().unwrap();

    // list existing databases and remember their number
    let databases = Box::pin(client.list_databases().into_stream())
        .next()
        .await
        .unwrap()
        .unwrap();
    let database_count_before = databases.databases.len();

    // create a new database and check if the number of DBs increased
    let database = client
        .create_database(DATABASE_NAME)
        .into_future()
        .await
        .unwrap();

    let databases = Box::pin(client.list_databases().into_stream())
        .next()
        .await
        .unwrap()
        .unwrap();

    assert!(databases.databases.len() == database_count_before + 1);

    // get the previously created database
    let database_after_get = client
        .clone()
        .into_database_client(DATABASE_NAME)
        .get_database(Context::new(), GetDatabaseOptions::new())
        .await
        .unwrap();
    assert!(database.database.rid == database_after_get.database.rid);

    // delete the database
    client
        .clone()
        .into_database_client(DATABASE_NAME)
        .delete_database()
        .into_future()
        .await
        .unwrap();

    let databases = Box::pin(client.list_databases().into_stream())
        .next()
        .await
        .unwrap()
        .unwrap();
    assert!(databases.databases.len() == database_count_before);
}

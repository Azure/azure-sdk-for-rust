#![cfg(feature = "mock_transport_framework")]

use azure_core::Context;
use azure_cosmos::prelude::*;
use std::error::Error;

mod setup;

type BoxedError = Box<dyn Error + Send + Sync>;

#[tokio::test]
async fn create_database_and_collection() -> Result<(), BoxedError> {
    env_logger::init();

    let client = setup::initialize("create_database_and_collection")?;
    let database_name = "sample";
    let context = Context::new();

    log::info!("Creating a database with name '{}'...", database_name);
    let db = client
        .create_database(
            context.clone(),
            &database_name,
            CreateDatabaseOptions::new(),
        )
        .await?;
    log::info!("Successfully created a database");
    log::debug!("The create_database response: {:#?}", db);

    assert_eq!(db.database.id, database_name);

    // create collection!
    let db_client = client.clone().into_database_client(database_name.clone());

    let collection_name = "panzadoro";
    log::info!("Creating a collection with name '{}'...", collection_name);
    let collection = db_client
        .create_collection(
            context,
            collection_name,
            CreateCollectionOptions::new("/id"),
        )
        .await?;

    assert_eq!(collection.collection.id, collection_name);

    log::info!("Successfully created a collection");
    log::debug!("The create_collection response: {:#?}", collection);

    Ok(())
}

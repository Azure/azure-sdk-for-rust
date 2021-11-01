#![cfg(feature = "mock_transport_framework")]

use azure_core::Context;
use azure_cosmos::prelude::*;
use futures::stream::StreamExt;
use std::error::Error;

mod setup;

type BoxedError = Box<dyn Error + Send + Sync>;

#[tokio::test]
async fn create_database_and_collection() -> Result<(), BoxedError> {
    env_logger::init();

    let client = setup::initialize("create_database_and_collection")?;
    let database_name = "test-create-database-and-collection";
    let context = Context::new();

    // create database!
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
            context.clone(),
            collection_name,
            CreateCollectionOptions::new("/id"),
        )
        .await?;
    assert_eq!(collection.collection.id, collection_name);
    log::info!("Successfully created a collection");
    log::debug!("The create_collection response: {:#?}", collection);

    // list collections!
    log::info!("Listing all collections...");
    let collections =
        Box::pin(db_client.list_collections(context.clone(), ListCollectionsOptions::new()))
            .next()
            .await
            .expect("No collection page")?;
    assert_eq!(collections.count, 1);
    log::info!("Successfully listed collections");
    log::debug!("The list_collection response: {:#?}", collections);

    // delete database
    log::info!("Deleting the database...");
    let deleted_database = db_client
        .delete_database(context.clone(), DeleteDatabaseOptions::new())
        .await?;
    log::info!("Successfully deleted database");
    log::debug!("The delete_database response: {:#?}", deleted_database);

    Ok(())
}

#![cfg(feature = "mock_transport_framework")]

use azure_core::prelude::*;
use azure_storage::table::prelude::*;
use std::error::Error;

mod setup;

type BoxedError = Box<dyn Error + Send + Sync>;

#[tokio::test]
async fn create_query_and_delete_table_storage() -> Result<(), BoxedError> {
    env_logger::init();

    let client = setup::initialize("create_query_and_delete_table_storage");
    let table_name = "table";
    let context = Context::new();

    // create table
    log::info!("Creating a table with name '{}'...", table_name);
    let create_table = client
        .create_table(context.clone(), &table_name, CreateTableOptions::default())
        .await?;
    log::info!(
        "Successfully created a table. response: {:#?}",
        create_table
    );

    // list tables
    log::info!("Listing all tables...");
    let query_tables = client
        .query_tables(context.clone(), QueryTablesOptions::default())
        .await?;
    log::info!("Successfully listed tables. response: {:#?}", query_tables);

    // delete table
    log::info!("Listing all tables...");
    let delete_table = client
        .delete_table(context.clone(), table_name, DeleteTableOptions::default())
        .await?;
    log::info!(
        "Successfully deleted a table. response: {:#?}",
        delete_table
    );

    Ok(())
}

use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_table::table::TableService;
use futures_util::stream::StreamExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account names and master keys from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let to_account =
        std::env::var("TO_STORAGE_ACCOUNT").expect("Set env variable TO_STORAGE_ACCOUNT first!");
    let to_master_key = std::env::var("TO_STORAGE_MASTER_KEY")
        .expect("Set env variable TO_STORAGE_MASTER_KEY first!");

    let table_name = std::env::args()
        .nth(1)
        .expect("please specify table name as command line parameter");

    let table_service = TableService::new(Client::new(&account, &master_key)?);
    let to_table_service = TableService::new(Client::new(&to_account, &to_master_key)?);

    println!("creating table {}", &table_name);
    to_table_service.create_table(&table_name).await?;

    let mut count: u32 = 0;

    while let Some(entities) = Box::pin(
        table_service.stream_query_entities_fullmetadata::<serde_json::Value>(&table_name, None),
    )
    .next()
    .await
    {
        count += 1;
        for entity in entities {
            to_table_service.insert_entity(&table_name, &entity).await?;
        }
    }
    println!("copied {} entities to table {}", count, &table_name);

    Ok(())
}

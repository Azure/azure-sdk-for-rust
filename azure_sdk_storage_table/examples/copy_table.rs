use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_table::table::TableService;
use futures::stream::StreamExt;
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

    let from_table_name = std::env::args()
        .nth(1)
        .expect("please specify source table name as first command line parameter");
    let to_table_name = std::env::args()
        .nth(2)
        .expect("please specify destination table name as second command line parameter");

    let from_table_service = TableService::new(Client::new(&account, &master_key)?);
    let to_table_service = TableService::new(Client::new(&to_account, &to_master_key)?);

    println!("creating table {}", &to_table_name);
    to_table_service.create_table(&to_table_name).await?;

    let mut count: u32 = 0;

    let mut stream = Box::pin(
        from_table_service
            .stream_query_entries_fullmetadata::<serde_json::Value>(&from_table_name, None),
    );

    while let Some(entries) = stream.next().await {
        let entries = entries?;
        for entry in entries {
            count += 1;
            println!("before {:?}", entry);
            let entry = to_table_service
                .insert_entry(&to_table_name, entry)
                .await?;
            println!("after {:?}", entry);
        }
    }
    println!(
        "copied {} entities to table {} in {}",
        count, &to_table_name, to_account,
    );

    Ok(())
}

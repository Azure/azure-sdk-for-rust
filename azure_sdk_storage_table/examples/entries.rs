#[macro_use]
extern crate serde_derive;
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_table::table::{TableService, TableStorage};
use azure_sdk_storage_table::TableEntry;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct MyEntry {
    pub my_value: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let client = Client::new(&account, &master_key)?;
    let table_service = TableService::new(client);

    let table_name = std::env::args()
        .nth(1)
        .expect("pass the table name as first command line parameter.");

    let row_key = std::env::args()
        .nth(2)
        .expect("pass the row key as second command line parameter.");

    let table_storage = TableStorage::new(table_service.clone(), table_name.clone());

    let mut my_entry = TableEntry {
        row_key: row_key,
        partition_key: "100".to_owned(),
        etag: None,
        payload: {
            MyEntry {
                my_value: "Itsy bitsy spider".to_owned(),
            }
        },
    };

    // insert the entry
    table_service.insert_entry(&table_name, &my_entry).await?;
    println!("entry inserted");

    // get the entry (notice the etag)
    let ret: TableEntry<MyEntry> = table_service
        .get_entry(&table_name, &my_entry.partition_key, &my_entry.row_key)
        .await?;
    println!("get_entry2 result == {:?}", ret);

    // now we update the entry passing the etag.
    my_entry.payload.my_value = "Wheel on the bus".to_owned();

    table_service.update_entry("example", &my_entry).await?;
    println!("update_entry2 completed without errors");

    // get the entry again (new payload and etag)
    let ret: TableEntry<MyEntry> = table_service
        .get_entry(&table_name, &my_entry.partition_key, &my_entry.row_key)
        .await?;
    println!("get_entry2 result == {:?}", ret);

    Ok(())
}

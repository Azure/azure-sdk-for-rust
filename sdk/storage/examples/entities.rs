#[macro_use]
extern crate serde_derive;

use azure_core::errors::AzureError;
use azure_storage::table::{CloudTable, TableClient, TableEntity};
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct MyEntity {
    pub my_value: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let table_name = std::env::args()
        .nth(1)
        .expect("pass the table name as first command line parameter.");

    let row_key = std::env::args()
        .nth(2)
        .expect("pass the row key as second command line parameter.");

    let client = TableClient::new(&account, &master_key);
    let table = CloudTable::new(client, table_name);
    table.create_if_not_exists().await?;

    // insert the entity
    let mut my_entity = table
        .insert(
            &row_key,
            "100",
            MyEntity {
                my_value: "Itsy bitsy spider".to_owned(),
            },
        )
        .await?;
    println!("entity inserted: {:?}", my_entity);

    // get the entity (notice the etag)
    let ret: TableEntity<MyEntity> = table
        .get(&my_entity.partition_key, &my_entity.row_key, None)
        .await?
        .ok_or(AzureError::GenericErrorWithText(
            "item not found after insertion".to_string(),
        ))?;
    println!("get_entity result == {:?}", ret);

    // now we update the entity passing the etag.
    my_entity.payload.my_value = "Wheel on the bus".to_owned();
    let mut my_entity = table.update_entity(my_entity).await?;
    println!("update_entity completed without errors: {:?}", my_entity);

    my_entity.payload.my_value = "Going round and round".to_owned();
    let my_entity = table.insert_or_update_entity(my_entity).await?;
    println!(
        "insert_or_update_entity completed without errors: {:?}",
        my_entity
    );

    // get the entity again (new payload and etag)
    let ret: TableEntity<MyEntity> = table
        .get(&my_entity.partition_key, &my_entity.row_key, None)
        .await?
        .ok_or(AzureError::GenericErrorWithText(
            "item not found after update".to_string(),
        ))?;
    println!("get_entity result == {:?}", ret);

    Ok(())
}

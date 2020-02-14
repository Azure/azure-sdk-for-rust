#[macro_use]
extern crate serde_derive;

use azure_sdk_storage_table::{CloudTable, TableClient};
use futures::stream::StreamExt;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct MyEntity {
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account names and master keys from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let to_account = std::env::var("TO_STORAGE_ACCOUNT").unwrap_or(account.clone());
    let to_master_key = std::env::var("TO_STORAGE_MASTER_KEY").unwrap_or(master_key.clone());

    let from_table_name = std::env::args()
        .nth(1)
        .expect("please specify source table name as first command line parameter");
    let to_table_name = std::env::args()
        .nth(2)
        .expect("please specify destination table name as second command line parameter");

    let from_table = CloudTable::new(TableClient::new(&account, &master_key)?, from_table_name);
    let to_table = CloudTable::new(
        TableClient::new(&to_account, &to_master_key)?,
        to_table_name.clone(),
    );

    println!("creating table {}", &to_table_name);
    to_table.create_if_not_exists().await?;

    let mut count: u32 = 0;

    let mut stream = Box::pin(from_table.stream_query::<MyEntity>(None));

    while let Some(Ok(entities)) = stream.next().await {
        println!("segemnt len: {}", entities.len());
        for entity in entities {
            count += 1;
            println!("before {:?}", entity);
            let entity = to_table.insert_entity(entity).await?;
            println!("after {:?}", entity);
        }
    }
    println!(
        "copied {} entities to table {} in {}",
        count, &to_table_name, to_account,
    );

    Ok(())
}

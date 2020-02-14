use azure_sdk_storage_table::TableClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let client = TableClient::new(&account, &master_key)?;
    let tables = client.list_tables().await?;

    println!("Account {} has {} tables(s)", account, tables.len());
    for ref table in tables {
        println!("{}", table);
    }
    Ok(())
}

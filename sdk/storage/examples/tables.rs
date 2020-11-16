use azure_storage::table::*;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Clone, Deserialize)]
struct MyEntity {
    my_value: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let client = TableClient::new(&account, &master_key);
    let tables = client.list_tables().await?;

    println!("Account {} has {} tables(s)", account, tables.len());
    for ref table in tables {
        println!("{}", table);
    }

    let table_client = CloudTable::new(client, "example");

    let response = table_client.begin_get_all::<MyEntity>().await?;
    println!("{:?}", response.entities);
    println!("{:?}", response.continuation_token);

    let mut response = table_client.begin_query::<MyEntity>("$top=2").await?;
    println!("{:?}", response.entities);
    println!("{:?}", response.continuation_token);

    while let Some(continuation_token) = response.continuation_token {
        println!("we have more data!");

        response = table_client.continue_execution(continuation_token).await?;
        println!("{:?}", response.entities);
        println!("{:?}", response.continuation_token);
    }

    Ok(())
}

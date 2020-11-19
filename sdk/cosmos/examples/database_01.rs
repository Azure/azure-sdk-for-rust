use azure_cosmos::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    // We expect master keys (ie, not resource constrained)
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;
    let cosmos_client = CosmosClient::new(account, authorization_token);
    //let databases = cosmos_client.list_databases().execute().await?;
    //println!("databases == {:#?}", databases);

    let database_client = cosmos_client.into_database_client("pollo");
    println!("database_name == {}", database_client.database_name());

    let collections = database_client.list_collections().execute().await?;
    println!("collections == {:#?}", collections);

    let collection_client = database_client.into_collection_client("cnt");
    let collection = collection_client.get_collection().execute().await?;
    println!("collection == {:#?}", collection);

    //let collection_client = database_client.with_collection(&"cnt");

    Ok(())
}

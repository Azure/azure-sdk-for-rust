use azure_core::HttpClient;
use azure_cosmos::prelude::*;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    // We expect master keys (ie, not resource constrained)
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
    let client = CosmosStruct::new(http_client, account, authorization_token);

    let database_client = client.with_database_client("pollo");
    println!("database_name == {}", database_client.database_name());

    let collections = database_client.list_collections().execute().await?;
    println!("collections == {:#?}", collections);

    let collection_client = database_client.with_collection_client("cnt");
    let collection = collection_client.get_collection().execute().await?;
    println!("collection == {:#?}", collection);

    Ok(())
}

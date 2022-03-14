use azure_data_cosmos::prelude::*;
use futures::stream::StreamExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    // We expect master keys (ie, not resource constrained)
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    let client = CosmosClient::new(account, authorization_token, CosmosOptions::default());

    let database = client.database_client("pollo");
    println!("database_name == {}", database.database_name());

    let collections = database
        .list_collections()
        .into_stream()
        .next()
        .await
        .unwrap()?;
    println!("collections == {:#?}", collections);

    let collection = database
        .collection_client("cnt")
        .get_collection()

        .await?;
    println!("collection == {:#?}", collection);

    Ok(())
}

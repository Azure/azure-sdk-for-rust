use azure_core::Context;
use azure_cosmos::prelude::*;
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

    let database_client = client.into_database_client("pollo");
    println!("database_name == {}", database_client.database_name());

    let collections =
        Box::pin(database_client.list_collections(Context::new(), ListCollectionsOptions::new()))
            .next()
            .await
            .unwrap()?;
    println!("collections == {:#?}", collections);

    let collection_client = database_client.into_collection_client("cnt");
    let collection = collection_client
        .get_collection(Context::new(), GetCollectionOptions::new())
        .await?;
    println!("collection == {:#?}", collection);

    Ok(())
}

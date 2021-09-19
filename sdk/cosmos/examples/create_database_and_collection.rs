use azure_core::Context;
use azure_cosmos::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();

    // First we retrieve the account name and master key from environment variables.
    // We expect master keys (ie, not resource constrained)
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let database_name = "sample";

    let mut context = Context::new();
    context.start_mock_transaction("create_database_and_collection");

    let authorization_token = permission::AuthorizationToken::primary_from_base64(&master_key)?;

    let client = CosmosClient::new(account, authorization_token, CosmosOptions::default());

    println!("before create database");
    let db = client
        .create_database(&mut context, &database_name, CreateDatabaseOptions::new())
        .await?;
    println!("create_database_response = {:#?}", db);

    // create collection!
    let db_client = client.clone().into_database_client(database_name.clone());

    let create_collection_response = db_client
        .create_collection(
            &mut context,
            "panzadoro",
            CreateCollectionOptions::new("/id"),
        )
        .await?;

    println!(
        "create_collection_response == {:#?}",
        create_collection_response
    );

    Ok(())
}

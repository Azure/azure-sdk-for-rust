use azure_core::Context;
use azure_cosmos::prelude::*;
use std::error::Error;

type BoxedError = Box<dyn Error + Send + Sync>;
#[tokio::main]
async fn main() -> Result<(), BoxedError> {
    env_logger::init();

    let client = create_client()?;
    let database_name = "sample";
    let context = Context::new();

    log::info!("Creating a database with name '{}'...", database_name);
    let db = client
        .create_database(
            context.clone(),
            &database_name,
            CreateDatabaseOptions::new(),
        )
        .await?;
    log::info!("Successfully created a database");
    log::debug!("The create_database response: {:#?}", db);

    // create collection!
    let db_client = client.clone().into_database_client(database_name.clone());

    let collection_name = "panzadoro";
    log::info!("Creating a collection with name '{}'...", collection_name);
    let create_collection_response = db_client
        .create_collection(
            context,
            collection_name,
            CreateCollectionOptions::new("/id"),
        )
        .await?;

    log::info!("Successfully created a collection");
    log::debug!(
        "The create_collection response: {:#?}",
        create_collection_response
    );

    Ok(())
}

#[cfg(not(feature = "mock_transport_framework"))]
fn create_client() -> Result<CosmosClient, BoxedError> {
    // First we retrieve the account name and master key from environment variables.
    // We expect keys that are not resource constrained.
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let authorization_token = permission::AuthorizationToken::primary_from_base64(&master_key)?;
    Ok(CosmosClient::new(
        account,
        authorization_token,
        CosmosOptions::default(),
    ))
}

#[cfg(feature = "mock_transport_framework")]
fn create_client() -> Result<CosmosClient, BoxedError> {
    let authorization_token = permission::AuthorizationToken::primary_from_base64("").unwrap();
    let options = CosmosOptions::new("create_database_and_collection");
    Ok(CosmosClient::new(
        String::new(),
        authorization_token,
        options,
    ))
}

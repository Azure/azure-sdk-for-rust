use azure_core::HttpClient;
use azure_cosmos::prelude::*;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let database = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");

    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
    let client = CosmosClient::new(http_client, account.clone(), authorization_token);

    let client = client.into_database_client(database);
    let client = client.into_collection_client(collection);

    let resp = client.get_partition_key_ranges().execute().await?;
    println!("resp == {:#?}", resp);

    Ok(())
}

use azure_core::HttpClient;
use azure_cosmos::prelude::*;
use azure_cosmos::AuthorizationToken;
use std::sync::Arc;

pub fn initialize() -> Result<CosmosClient, CosmosError> {
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::primary_from_base64(&key)?;
    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
    let client = CosmosClient::new(http_client, account, authorization_token);

    Ok(client)
}

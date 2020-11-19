use azure_core::errors::AzureError;
use azure_cosmos::prelude::*;
use azure_cosmos::AuthorizationToken;

pub fn initialize() -> Result<CosmosClient, AzureError> {
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::new_master(&key)?;
    let client = CosmosClient::new(account, authorization_token);

    Ok(client)
}

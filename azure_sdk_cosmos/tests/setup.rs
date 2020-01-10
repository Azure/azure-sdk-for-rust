use azure_sdk_core::errors::AzureError;
use azure_sdk_cosmos::clients::{Client, ClientBuilder, CosmosUriBuilder};
use azure_sdk_cosmos::{AuthorizationToken, TokenType};

pub fn initialize() -> Result<Client<impl CosmosUriBuilder>, AzureError> {
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let key = std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_KEY first!");

    let authorization_token = AuthorizationToken::new(TokenType::Master, &key)?;
    let client = ClientBuilder::new(account, authorization_token)?;

    Ok(client)
}

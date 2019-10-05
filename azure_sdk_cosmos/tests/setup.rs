use azure_sdk_core::errors::AzureError;
use azure_sdk_cosmos::{AuthorizationToken, Client, ClientBuilder, CosmosUriBuilder, TokenType};
use tokio_core::reactor::Core;

pub fn initialize() -> Result<(Client<impl CosmosUriBuilder>, Core), AzureError> {
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let key = std::env::var("COSMOS_KEY").expect("Set env variable COSMOS_KEY first!");

    let authorization_token = AuthorizationToken::new(account, TokenType::Master, &key)?;
    let client = ClientBuilder::new(authorization_token)?;
    let core = Core::new()?;

    Ok((client, core))
}

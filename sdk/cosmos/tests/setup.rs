use azure_cosmos::prelude::*;

pub fn initialize() -> Result<CosmosClient, azure_cosmos::Error> {
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::primary_from_base64(&key)?;
    let client = CosmosClient::new(account, authorization_token, CosmosOptions::default());

    Ok(client)
}

use azure_cosmos::prelude::*;

pub fn initialize() -> Result<CosmosClient, CosmosError> {
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::primary_from_base64(&key)?;
    let http_client = azure_core::new_http_client();
    let client = CosmosClient::new(http_client, account, authorization_token);

    Ok(client)
}

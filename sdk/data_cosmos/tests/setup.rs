use azure_data_cosmos::prelude::*;

pub fn initialize() -> azure_core::Result<CosmosClient> {
    let account = get_account();
    let authorization_token = get_authorization_token()?;

    let client = CosmosClient::new(account, authorization_token);

    Ok(client)
}

fn get_account() -> String {
    std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!")
}

fn get_authorization_token() -> azure_core::Result<AuthorizationToken> {
    let key =
        std::env::var("COSMOS_PRIMARY_KEY").expect("Set env variable COSMOS_PRIMARY_KEY first!");

    AuthorizationToken::primary_from_base64(&key)
}

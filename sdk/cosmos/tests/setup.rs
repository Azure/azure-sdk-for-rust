use crate::permission::AuthorizationTokenParsingError;
use azure_cosmos::prelude::*;

#[cfg(not(feature = "mock_transport_framework"))]
pub fn initialize() -> Result<CosmosClient, azure_cosmos::Error> {
    let account = get_account();
    let authorization_token = get_authorization_token()?;

    let client = CosmosClient::new(account, authorization_token, CosmosOptions::default());

    Ok(client)
}

fn get_account() -> String {
    std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!")
}

fn get_authorization_token() -> Result<AuthorizationToken, AuthorizationTokenParsingError> {
    let key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    AuthorizationToken::primary_from_base64(&key)
}

#[cfg(feature = "mock_transport_framework")]
pub fn initialize(
    transaction_name: impl Into<String>,
) -> Result<CosmosClient, azure_cosmos::Error> {
    let account_name = (std::env::var(azure_core::TESTING_MODE_KEY).as_deref()
        == Ok(azure_core::TESTING_MODE_RECORD))
    .then(get_account)
    .unwrap_or_else(String::new);
    let authorization_token = get_authorization_token()?;

    Ok(CosmosClient::new_with_transaction(
        account_name,
        authorization_token,
        transaction_name,
    ))
}

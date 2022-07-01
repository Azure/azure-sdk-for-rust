use azure_data_cosmos::prelude::*;

#[cfg(not(feature = "mock_transport_framework"))]
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

#[cfg(feature = "mock_transport_framework")]
pub fn initialize(transaction_name: impl Into<String>) -> azure_core::Result<CosmosClient> {
    let account_name = (std::env::var(azure_core::mock::TESTING_MODE_KEY).as_deref()
        == Ok(azure_core::mock::TESTING_MODE_RECORD))
    .then(get_account)
    .unwrap_or_else(String::new);
    let authorization_token = (std::env::var(azure_core::mock::TESTING_MODE_KEY).as_deref()
        == Ok(azure_core::mock::TESTING_MODE_RECORD))
    .then(|| get_authorization_token().ok())
    .flatten()
    .unwrap_or_else(|| AuthorizationToken::new_resource(String::new()));

    Ok(CosmosClient::new_with_transaction(
        account_name,
        authorization_token,
        transaction_name,
    ))
}

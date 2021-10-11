use azure_cosmos::prelude::*;

#[cfg(not(feature = "mock_transport_framework"))]
pub fn initialize() -> Result<CosmosClient, azure_cosmos::Error> {
    let account = get_account();
    let key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::primary_from_base64(&key)?;
    let client = CosmosClient::new(account, authorization_token, CosmosOptions::default());

    Ok(client)
}

fn get_account() -> String {
    std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!")
}

#[cfg(feature = "mock_transport_framework")]
pub fn initialize(
    transaction_name: impl Into<String>,
) -> Result<CosmosClient, azure_cosmos::Error> {
    let account_name = (std::env::var("TESTING_MODE").as_deref() == Ok("RECORD"))
        .then(get_account)
        .unwrap_or_else(String::new);
    Ok(CosmosClient::new_with_transaction(
        account_name,
        transaction_name,
    ))
}

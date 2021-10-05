use azure_cosmos::prelude::*;

#[cfg(not(feature = "mock_transport_framework"))]
pub fn initialize() -> Result<CosmosClient, azure_cosmos::Error> {
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::primary_from_base64(&key)?;
    let client = CosmosClient::new(account, authorization_token, CosmosOptions::default());

    Ok(client)
}

#[cfg(feature = "mock_transport_framework")]
pub fn initialize(
    transaction_name: impl Into<String>,
) -> Result<CosmosClient, azure_cosmos::Error> {
    Ok(CosmosClient::new_with_transaction(transaction_name))
}

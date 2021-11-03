use azure_core::Error;
use azure_storage::table::prelude::*;

#[cfg(not(feature = "mock_transport_framework"))]
pub fn initialize() -> Result<TableClient, Error> {
    Ok(TableClient::new(
        get_account(),
        get_authorization_token(),
        TableOptions::default(),
    ))
}

#[cfg(feature = "mock_transport_framework")]
pub fn initialize(transaction_name: impl Into<String>) -> TableClient {
    let account_name = (std::env::var(azure_core::TESTING_MODE_KEY).as_deref()
        == Ok(azure_core::TESTING_MODE_RECORD))
    .then(get_account)
    .unwrap_or_else(String::new);

    let auth_token = (std::env::var(azure_core::TESTING_MODE_KEY).as_deref()
        == Ok(azure_core::TESTING_MODE_RECORD))
    .then(|| get_authorization_token().ok())
    .flatten()
    .unwrap_or_else(|| AuthorizationToken::new_resource(String::new()));

    TableClient::new_with_transaction(account_name, auth_token, transaction_name)
}

fn get_account() -> String {
    std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!")
}

fn get_authorization_token() -> AuthorizationToken {
    AuthorizationToken::SharedKeyToken {
        key: std::env::var("STORAGE_MASTER_KEY")
            .expect("Set env variable COSMOS_MASTER_KEY first!"),
        account: get_account(),
    }
}

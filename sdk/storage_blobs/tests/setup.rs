use azure_identity::DefaultAzureCredential;
use azure_storage::clients::StorageClient;
use std::sync::Arc;

#[cfg(feature = "mock_transport_framework")]
use azure_storage::clients::StorageCredentials;

#[cfg(not(feature = "mock_transport_framework"))]
pub fn initialize() -> azure_core::Result<StorageClient> {
    let credentials = Arc::new(DefaultAzureCredential::default());

    let client = StorageClient::new_token_credential(get_account(), credentials);

    Ok(client)
}

#[cfg(feature = "mock_transport_framework")]
pub fn initialize(transaction_name: impl Into<String>) -> azure_core::Result<StorageClient> {
    let account_name = (std::env::var(azure_core::mock::TESTING_MODE_KEY).as_deref()
        == Ok(azure_core::mock::TESTING_MODE_RECORD))
    .then(get_account)
    .unwrap_or_default();
    let storage_credentials = (std::env::var(azure_core::mock::TESTING_MODE_KEY).as_deref()
        == Ok(azure_core::mock::TESTING_MODE_RECORD))
    .then(|| StorageCredentials::TokenCredential(Arc::new(DefaultAzureCredential::default())))
    .unwrap_or_else(|| StorageCredentials::BearerToken(String::default()));

    Ok(StorageClient::new_mock(
        account_name,
        storage_credentials,
        transaction_name,
    ))
}

fn get_account() -> String {
    std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!")
}

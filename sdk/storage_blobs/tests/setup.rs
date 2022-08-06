use azure_identity::DefaultAzureCredential;
use azure_storage::clients::{StorageClient, StorageCredentials};
use std::sync::Arc;

pub fn initialize(transaction_name: impl Into<String>) -> azure_core::Result<StorageClient> {
    let account_name = (std::env::var(mock_transport::TESTING_MODE_KEY).as_deref()
        == Ok(mock_transport::TESTING_MODE_RECORD))
    .then(get_account)
    .unwrap_or_default();
    let storage_credentials = (std::env::var(mock_transport::TESTING_MODE_KEY).as_deref()
        == Ok(mock_transport::TESTING_MODE_RECORD))
    .then(|| StorageCredentials::TokenCredential(Arc::new(DefaultAzureCredential::default())))
    .unwrap_or_else(|| StorageCredentials::BearerToken(String::default()));

    let transport_policy = mock_transport::new_mock_transport(transaction_name.into());
    Ok(StorageClient::new_mock(
        account_name,
        storage_credentials,
        transport_policy,
    ))
}

fn get_account() -> String {
    std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!")
}

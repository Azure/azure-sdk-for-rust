use azure_core::TransportOptions;
use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::BlobServiceClient;
use std::env::var;

pub fn initialize(transaction_name: impl Into<String>) -> azure_core::Result<BlobServiceClient> {
    let (account_name, storage_credentials) = if var(mock_transport::TESTING_MODE_KEY).as_deref()
        == Ok(mock_transport::TESTING_MODE_RECORD)
    {
        let account_name = var("STORAGE_ACCOUNT").expect("missing env STORAGE_ACCOUNT");
        let account_key = var("STORAGE_ACCESS_KEY").expect("missing env STORAGE_ACCESS_KEY");
        let storage_credentials = StorageCredentials::Key(account_name.clone(), account_key);
        (account_name, storage_credentials)
    } else {
        (
            String::new(),
            StorageCredentials::BearerToken(String::new()),
        )
    };

    let transport_options = TransportOptions::new_custom_policy(
        mock_transport::new_mock_transport(transaction_name.into()),
    );
    let client = BlobServiceClient::builder(account_name, storage_credentials)
        .transport(transport_options)
        .blob_service_client();
    Ok(client)
}

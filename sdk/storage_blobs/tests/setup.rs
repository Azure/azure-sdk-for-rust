use azure_core::TransportOptions;
use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::BlobServiceClient;

pub fn initialize(transaction_name: impl Into<String>) -> azure_core::Result<BlobServiceClient> {
    let account_name = (std::env::var(mock_transport::TESTING_MODE_KEY).as_deref()
        == Ok(mock_transport::TESTING_MODE_RECORD))
    .then(get_account)
    .unwrap_or_default();

    let storage_credentials = if std::env::var(mock_transport::TESTING_MODE_KEY).as_deref()
        == Ok(mock_transport::TESTING_MODE_RECORD)
    {
        let account =
            std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
        let access_key = std::env::var("STORAGE_ACCESS_KEY")
            .expect("Set env variable STORAGE_ACCESS_KEY first!");

        StorageCredentials::Key(account.clone(), access_key)
    } else {
        StorageCredentials::BearerToken(String::default())
    };

    let transport_options = TransportOptions::new_custom_policy(
        mock_transport::new_mock_transport(transaction_name.into()),
    );
    let client = BlobServiceClient::builder(account_name, storage_credentials)
        .transport(transport_options)
        .blob_service_client();
    Ok(client)
}

fn get_account() -> String {
    std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!")
}

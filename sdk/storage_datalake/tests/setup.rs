use azure_storage::prelude::StorageCredentials;
use azure_storage_datalake::prelude::*;

pub async fn create_data_lake_client(transaction_name: &str) -> azure_core::Result<DataLakeClient> {
    let account_name = (std::env::var(mock_transport::TESTING_MODE_KEY).as_deref()
        == Ok(mock_transport::TESTING_MODE_RECORD))
    .then(get_account)
    .unwrap_or_default();

    let account_key = (std::env::var(mock_transport::TESTING_MODE_KEY).as_deref()
        == Ok(mock_transport::TESTING_MODE_RECORD))
    .then(get_key)
    .unwrap_or_default();

    let transport_options = azure_core::TransportOptions::new_custom_policy(
        mock_transport::new_mock_transport(transaction_name.into()),
    );

    let storage_credentials = StorageCredentials::access_key(account_name.clone(), account_key);
    Ok(DataLakeClient::builder(account_name, storage_credentials)
        .transport(transport_options)
        .build())
}

fn get_account() -> String {
    std::env::var("ADLSGEN2_STORAGE_ACCOUNT")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCOUNT first!")
}

fn get_key() -> String {
    std::env::var("ADLSGEN2_STORAGE_ACCESS_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCESS_KEY first!")
}

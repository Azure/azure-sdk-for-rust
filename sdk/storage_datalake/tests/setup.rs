use azure_core::ClientOptions;
use azure_storage::storage_shared_key_credential::StorageSharedKeyCredential;
use azure_storage_datalake::prelude::*;

pub async fn create_data_lake_client(transaction_name: &str) -> azure_core::Result<DataLakeClient> {
    let account_name = (std::env::var(mock_transport::TESTING_MODE_KEY).as_deref()
        == Ok(mock_transport::TESTING_MODE_RECORD))
    .then(get_account)
    .unwrap_or_else(String::new);

    let account_key = (std::env::var(mock_transport::TESTING_MODE_KEY).as_deref()
        == Ok(mock_transport::TESTING_MODE_RECORD))
    .then(get_key)
    .unwrap_or_else(String::new);

    let transport_options = azure_core::TransportOptions::new_custom_policy(
        mock_transport::new_mock_transport(transaction_name.into()),
    );
    let options = ClientOptions::new(transport_options);

    Ok(DataLakeClient::new_with_shared_key(
        StorageSharedKeyCredential::new(account_name, account_key),
        None,
        options,
    ))
}

fn get_account() -> String {
    std::env::var("ADLSGEN2_STORAGE_ACCOUNT")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCOUNT first!")
}

fn get_key() -> String {
    std::env::var("ADLSGEN2_STORAGE_ACCESS_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCESS_KEY first!")
}

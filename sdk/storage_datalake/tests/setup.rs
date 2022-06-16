#![cfg(feature = "mock_transport_framework")]
use azure_core::error::Result;
use azure_core::ClientOptions;
use azure_storage::storage_shared_key_credential::StorageSharedKeyCredential;
use azure_storage_datalake::prelude::*;

pub async fn create_data_lake_client(transaction_name: &str) -> Result<DataLakeClient> {
    let account_name = (std::env::var(azure_core::mock::TESTING_MODE_KEY).as_deref()
        == Ok(azure_core::mock::TESTING_MODE_RECORD))
    .then(get_account)
    .unwrap_or_else(String::new);

    let account_key = (std::env::var(azure_core::mock::TESTING_MODE_KEY).as_deref()
        == Ok(azure_core::mock::TESTING_MODE_RECORD))
    .then(get_key)
    .unwrap_or_else(String::new);

    let options = ClientOptions::new_with_transaction_name(transaction_name.into());

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
    std::env::var("ADLSGEN2_STORAGE_MASTER_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_MASTER_KEY first!")
}

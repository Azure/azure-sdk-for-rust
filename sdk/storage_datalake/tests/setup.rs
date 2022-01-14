#![cfg(feature = "mock_transport_framework")]
use azure_core::ClientOptions;
use azure_storage::storage_shared_key_credential::StorageSharedKeyCredential;
use azure_storage_datalake::prelude::*;
use std::error::Error;

pub async fn create_data_lake_client(
    transaction_name: &str,
) -> Result<DataLakeClient, Box<dyn Error + Send + Sync>> {
    let account_name = std::env::var("ADLSGEN2_STORAGE_ACCOUNT")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCOUNT first!");
    let account_key = std::env::var("ADLSGEN2_STORAGE_MASTER_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_MASTER_KEY first!");

    let options = ClientOptions::new_with_transaction_name(transaction_name.into());

    Ok(DataLakeClient::new_with_options(
        StorageSharedKeyCredential::new(account_name, account_key),
        None,
        options,
    ))
}

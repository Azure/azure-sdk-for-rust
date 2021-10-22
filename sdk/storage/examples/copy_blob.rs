use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use chrono::{Duration, Utc};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let source_account = std::env::var("SOURCE_STORAGE_ACCOUNT")
        .expect("Set env variable SOURCE_STORAGE_ACCOUNT first!");
    let source_master_key = std::env::var("SOURCE_STORAGE_MASTER_KEY")
        .expect("Set env variable SOURCE_STORAGE_MASTER_KEY first!");
    let destination_account = std::env::var("DESTINATION_STORAGE_ACCOUNT")
        .expect("Set env variable DESTINATION_STORAGE_ACCOUNT first!");
    let destination_master_key = std::env::var("DESTINATION_STORAGE_MASTER_KEY")
        .expect("Set env variable DESTINATION_STORAGE_MASTER_KEY first!");

    let source_container_name = std::env::args()
        .nth(1)
        .expect("please specify source container name as first command line parameter");
    let source_blob_name = std::env::args()
        .nth(2)
        .expect("please specify source blob name as second command line parameter");

    let destination_container_name = std::env::args()
        .nth(3)
        .expect("please specify destination container name as third command line parameter");
    let destination_blob_name = std::env::args()
        .nth(4)
        .expect("please specify destination blob name as fourth command line parameter");

    let http_client = new_http_client();

    let source_storage_account_client = StorageAccountClient::new_access_key(
        http_client.clone(),
        &source_account,
        &source_master_key,
    );
    let source_blob = source_storage_account_client
        .as_storage_client()
        .as_container_client(&source_container_name)
        .as_blob_client(&source_blob_name);

    let destination_blob = StorageAccountClient::new_access_key(
        http_client.clone(),
        &destination_account,
        &destination_master_key,
    )
    .as_storage_client()
    .as_container_client(&destination_container_name)
    .as_blob_client(&destination_blob_name);

    // let's get a SAS key for the source
    let sas_url = {
        let now = Utc::now();
        let later = now + Duration::hours(1);
        let sas = source_storage_account_client
            .shared_access_signature()?
            .with_resource(SasResource::Blob)
            .with_resource_type(SasResourceType::Object)
            .with_start(now)
            .with_expiry(later)
            .with_permissions(SasPermissions {
                read: true,
                ..Default::default()
            })
            .with_protocol(SasProtocol::HttpHttps)
            .finalize();
        println!("token: '{}'", sas.token());

        source_blob.generate_signed_blob_url(&sas)?
    };
    println!("read only SAS url: '{}'", sas_url);

    let response = destination_blob.copy(&sas_url).execute().await?;
    println!("copy response == {:#?}", response);

    Ok(())
}

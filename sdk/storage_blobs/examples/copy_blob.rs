use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let source_account = std::env::var("SOURCE_STORAGE_ACCOUNT")
        .expect("Set env variable SOURCE_STORAGE_ACCOUNT first!");
    let source_access_key = std::env::var("SOURCE_STORAGE_ACCESS_KEY")
        .expect("Set env variable SOURCE_STORAGE_ACCESS_KEY first!");
    let destination_account = std::env::var("DESTINATION_STORAGE_ACCOUNT")
        .expect("Set env variable DESTINATION_STORAGE_ACCOUNT first!");
    let destination_access_key = std::env::var("DESTINATION_STORAGE_ACCESS_KEY")
        .expect("Set env variable DESTINATION_STORAGE_ACCESS_KEY first!");

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

    let http_client = azure_core::new_http_client();

    let source_storage_account_client = StorageAccountClient::new_access_key(
        http_client.clone(),
        &source_account,
        &source_access_key,
    );
    let source_blob = source_storage_account_client
        .container_client(&source_container_name)
        .blob_client(&source_blob_name);

    let destination_blob = StorageAccountClient::new_access_key(
        http_client.clone(),
        &destination_account,
        &destination_access_key,
    )
    .container_client(&destination_container_name)
    .blob_client(&destination_blob_name);

    // let's get a SAS key for the source
    let sas_url = {
        let now = Utc::now();
        let later = now + Duration::hours(1);
        let sas = source_storage_account_client
            .shared_access_signature()?
            .with_resource(AccountSasResource::Blob)
            .with_resource_type(AccountSasResourceType::Object)
            .with_start(now)
            .with_expiry(later)
            .with_permissions(AccountSasPermissions {
                read: true,
                ..Default::default()
            })
            .with_protocol(SasProtocol::HttpHttps)
            .finalize();
        println!("token: '{}'", sas.token());

        source_blob.generate_signed_blob_url(&sas)?
    };
    println!("read only SAS url: '{}'", sas_url);

    let response = destination_blob.copy(sas_url).into_future().await?;
    println!("copy response == {:#?}", response);

    Ok(())
}

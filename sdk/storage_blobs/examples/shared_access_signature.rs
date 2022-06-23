use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use chrono::{Duration, Utc};

fn main() {
    env_logger::init();
    code().unwrap();
}

fn code() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob_name = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    // allow for some time skew
    let now = Utc::now() - Duration::minutes(15);
    let later = now + Duration::hours(1);

    let http_client = azure_core::new_http_client();

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &access_key);
    let container_client = storage_account_client.container_client(&container_name);
    let blob_client = container_client.blob_client(&blob_name);

    let sas = storage_account_client
        .shared_access_signature()?
        .with_resource(AccountSasResource::Blob)
        .with_resource_type(AccountSasResourceType::Object)
        .with_start(now)
        .with_expiry(later)
        .with_permissions(AccountSasPermissions {
            read: true,
            ..Default::default()
        })
        .with_protocol(SasProtocol::Https)
        .finalize();

    println!("blob account level token: '{}'", sas.token());
    let url = blob_client.generate_signed_blob_url(&sas)?;
    println!("blob account level url: '{}'", url);

    let sas = blob_client
        .shared_access_signature()?
        .with_expiry(later)
        .with_start(now)
        .with_permissions(BlobSasPermissions {
            write: true,
            ..Default::default()
        })
        .finalize();
    println!("blob service token: {}", sas.token());
    let url = blob_client.generate_signed_blob_url(&sas)?;
    println!("blob service level url: '{}'", url);

    let sas = container_client
        .shared_access_signature()?
        .with_expiry(later)
        .with_start(now)
        .with_permissions(BlobSasPermissions {
            read: true,
            list: true,
            write: true,
            ..Default::default()
        })
        .with_protocol(SasProtocol::HttpHttps)
        .finalize();

    println!("container sas token: {}", sas.token());
    let url = container_client.generate_signed_container_url(&sas)?;
    println!("container level url: '{}'", url);

    Ok(())
}

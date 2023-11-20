use azure_core::date;
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();

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
    let now = OffsetDateTime::now_utc() - date::duration_from_minutes(15);
    let later = now + date::duration_from_hours(1);

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let service_client = BlobServiceClient::new(account, storage_credentials);
    let container_client = service_client.container_client(container_name);
    let blob_client = container_client.blob_client(blob_name);

    let sas = service_client
        .shared_access_signature(
            AccountSasResourceType::Object,
            later,
            AccountSasPermissions {
                read: true,
                ..Default::default()
            },
        )
        .await?
        .start(now)
        .protocol(SasProtocol::Https);

    println!("blob account level token: '{}'", sas.token());
    let url = blob_client.generate_signed_blob_url(&sas)?;
    println!("blob account level url: '{url}'");

    let sas = blob_client
        .shared_access_signature(
            BlobSasPermissions {
                write: true,
                ..Default::default()
            },
            later,
        )
        .await?
        .start(now);
    println!("blob service token: {}", sas.token());
    let url = blob_client.generate_signed_blob_url(&sas)?;
    println!("blob service level url: '{url}'");

    let sas = container_client
        .shared_access_signature(
            BlobSasPermissions {
                read: true,
                list: true,
                write: true,
                ..Default::default()
            },
            later,
        )
        .await?
        .start(now)
        .protocol(SasProtocol::HttpHttps);

    println!("container sas token: {}", sas.token());
    let url = container_client.generate_signed_container_url(&sas)?;
    println!("container level url: '{url}'");

    Ok(())
}

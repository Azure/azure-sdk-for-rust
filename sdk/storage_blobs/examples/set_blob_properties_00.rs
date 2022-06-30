#[macro_use]
extern crate log;
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    let storage_client = StorageClient::new_access_key(&account, &master_key);

    // this is how you would use the SAS token:
    // let storage_client = StorageAccountClient::new_sas_token(http_client.clone(), &account,
    //      "sv=2018-11-09&ss=b&srt=o&se=2021-01-15T12%3A09%3A01Z&sp=r&st=2021-01-15T11%3A09%3A01Z&spr=http,https&sig=some_signature")?;

    let blob_client = storage_client
        .container_client(&container)
        .blob_client(&blob);

    trace!("Requesting blob properties");

    let properties = blob_client
        .get_properties()
        .into_future()
        .await?
        .blob
        .properties;

    blob_client
        .set_properties()
        .set_from_blob_properties(properties)
        .content_md5(md5::compute("howdy"))
        .into_future()
        .await?;

    Ok(())
}

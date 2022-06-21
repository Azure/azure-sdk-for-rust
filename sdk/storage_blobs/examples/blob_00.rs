#[macro_use]
extern crate log;
use azure_core::error::{ErrorKind, ResultExt};
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

    let http_client = azure_core::new_http_client();
    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    // this is how you would use the SAS token:
    // let storage_account_client = StorageAccountClient::new_sas_token(http_client.clone(), &account,
    //      "sv=2018-11-09&ss=b&srt=o&se=2021-01-15T12%3A09%3A01Z&sp=r&st=2021-01-15T11%3A09%3A01Z&spr=http,https&sig=some_signature")?;

    let storage_client = storage_account_client.as_storage_client();
    let blob_client = storage_client
        .as_container_client(&container)
        .as_blob_client(&blob);

    trace!("Requesting blob");

    let blob = blob_client.get_content().await?;

    let s_content = String::from_utf8(blob).map_kind(ErrorKind::DataConversion)?;
    println!("s_content == {}", s_content);

    Ok(())
}

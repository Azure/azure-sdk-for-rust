#[macro_use]
extern crate log;

use azure_core::{error::Result, prelude::*};
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob_name = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    let http_client = azure_core::new_http_client();

    let blob_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
            .as_container_client(&container)
            .as_blob_client(&blob_name);

    //let data = b"something";

    let mut metadata = Metadata::new();

    metadata.insert("pollo".to_owned(), "arrosto".to_owned());
    metadata.insert("milk".to_owned(), "shake".to_owned());

    // this is not mandatory but it helps preventing
    // spurious data to be uploaded.
    //let _hash = md5::compute(data).into();

    // The required parameters are container_name, blob_name and body.
    // The builder supports many more optional
    // parameters (such as LeaseID, or ContentDisposition, etc...)
    // so make sure to check with the documentation.
    trace!("before put_append_blob");
    let res = blob_client
        .put_append_blob()
        .content_type("text/plain")
        .content_language("en/us")
        .metadata(&metadata)
        .execute()
        .await?;

    println!("{:?}", res);

    // let get back the metadata
    let res = blob_client.get_metadata().execute().await?;
    println!("{:?}", res);

    Ok(())
}

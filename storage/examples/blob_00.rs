#[macro_use]
extern crate log;

use azure_sdk_core::prelude::*;
use azure_sdk_storage::blob::prelude::*;
use azure_sdk_storage::core::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

    let client = client::with_access_key(&account, &master_key);

    trace!("Requesting blob");

    let response = client
        .get_blob()
        .with_container_name(&container)
        .with_blob_name(&blob)
        .finalize()
        .await?;

    let s_content = String::from_utf8(response.data)?;
    println!("blob == {:?}", blob);
    println!("s_content == {}", s_content);

    Ok(())
}

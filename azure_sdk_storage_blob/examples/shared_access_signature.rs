use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;
use chrono::{Duration, Utc};
use std::error::Error;

fn main() {
    env_logger::init();
    code().unwrap();
}

fn code() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob_name = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    let client = client::with_access_key(&account, &master_key);

    let now = Utc::now();
    let later = now + Duration::hours(1);
    let sas = client
        .shared_access_signature()
        .with_resource(SasResource::Blob)
        .with_resource_type(SasResourceType::Object)
        .with_start(now)
        .with_expiry(later)
        .with_permissions(SasPermissions::Read)
        .with_protocol(SasProtocol::HttpHttps)
        .finalize();
    println!("token: '{}'", sas.token());

    let url = client
        .generate_signed_blob_url()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_shared_access_signature(&sas)
        .finalize();
    println!("url: '{}'", url);

    Ok(())
}

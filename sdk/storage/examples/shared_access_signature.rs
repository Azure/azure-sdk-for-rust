use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use chrono::{Duration, Utc};
use std::error::Error;
use std::sync::Arc;

fn main() {
    env_logger::init();
    code().unwrap();
}

fn code() -> Result<(), Box<dyn Error + Sync + Send>> {
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

    let http_client: Arc<dyn HttpClient> = Arc::new(reqwest::Client::new());

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);
    let blob = storage_account_client
        .as_storage_client()
        .as_container_client(&container_name)
        .as_blob_client(&blob_name);

    let now = Utc::now();
    let later = now + Duration::hours(1);
    let sas = storage_account_client
        .shared_access_signature()?
        .with_resource(SasResource::Blob)
        .with_resource_type(SasResourceType::Object)
        .with_start(now)
        .with_expiry(later)
        .with_permissions(SasPermissions::Read)
        .with_protocol(SasProtocol::HttpHttps)
        .finalize();
    println!("token: '{}'", sas.token());

    let url = blob.generate_signed_blob_url(&sas)?;
    println!("url: '{}'", url);

    Ok(())
}

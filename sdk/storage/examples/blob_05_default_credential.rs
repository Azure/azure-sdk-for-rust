#[macro_use]
extern crate log;

use azure_core::prelude::*;
use azure_core::TokenCredential;
use azure_identity::token_credentials::DefaultCredential;
use azure_storage::core::prelude::*;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();
    // First we retrieve the account name, container and blob name from command line args

    let account = std::env::args()
        .nth(1)
        .expect("please specify the account name as first command line parameter");
    let container = std::env::args()
        .nth(2)
        .expect("please specify the container name as second command line parameter");
    let blob = std::env::args()
        .nth(3)
        .expect("please specify the blob name as third command line parameter");

    let bearer_token = DefaultCredential::default()
        .get_token("https://storage.azure.com/")
        .await?;

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

    let storage_account_client = StorageAccountClient::new_bearer_token(
        http_client.clone(),
        &account,
        bearer_token.token.secret(),
    );
    let storage_client = storage_account_client.as_storage_client();
    let blob = storage_client
        .as_container_client(&container)
        .as_blob_client(&blob);

    trace!("Requesting blob");

    let response = blob.get().execute().await?;

    let s_content = String::from_utf8(response.data.to_vec())?;
    println!("blob == {:?}", blob);
    println!("s_content == {}", s_content);

    Ok(())
}

#[macro_use]
extern crate log;
use azure_core::prelude::*;
use azure_storage::clients::*;
use futures::stream::StreamExt;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
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

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

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

    let response = blob_client
        .get()
        .range(Range::new(0, 128000))
        .execute()
        .await?;

    println!("{:#?}", response);

    let mut stream = Box::pin(blob_client.get().stream(128));
    while let Some(value) = stream.next().await {
        println!("received {:?} bytes", value?.data.len());
    }

    let s_content = String::from_utf8(response.data.to_vec())?;
    println!("blob == {:?}", blob);
    println!("s_content == {}", s_content);

    Ok(())
}

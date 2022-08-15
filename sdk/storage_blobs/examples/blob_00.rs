#[macro_use]
extern crate log;
use azure_core::error::{ErrorKind, ResultExt};
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    let storage_client = StorageClient::new_access_key(&account, &access_key);

    // this is how you would use the SAS token:
    // let storage_client = StorageAccountClient::new_sas_token(http_client.clone(), &account,
    //      "sv=2018-11-09&ss=b&srt=o&se=2021-01-15T12%3A09%3A01Z&sp=r&st=2021-01-15T11%3A09%3A01Z&spr=http,https&sig=some_signature")?;

    let blob_client = storage_client
        .container_client(&container)
        .blob_client(&blob);

    trace!("Requesting blob");

    // this is a single call that retrieves the first 1KB of the blob (or less if the blob is
    // smaller). The range(...) call is optional.
    let response = blob_client
        .get()
        .range(0u64..1024)
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;

    println!("{:#?}", response);

    let mut complete_response = vec![];
    // this is how you stream a blob. You can specify the range(...) value as above if necessary.
    // In this case we are retrieving the whole blob in 8KB chunks.
    let mut stream = blob_client.get().chunk_size(0x2000u64).into_stream();
    while let Some(value) = stream.next().await {
        let data = value?.data.collect().await?;
        println!("received {:?} bytes", data.len());
        complete_response.extend(&data);
    }

    let s_content = String::from_utf8(complete_response).map_kind(ErrorKind::DataConversion)?;
    println!("s_content == {}", s_content);

    Ok(())
}

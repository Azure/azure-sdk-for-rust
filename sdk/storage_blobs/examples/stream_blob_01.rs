use azure_core::error::Result;
use azure_storage::core::prelude::*;
use azure_storage_blobs::{blob::responses::GetBlobResponse, prelude::*};
use futures::stream::StreamExt;

// This example shows how to stream data from a blob. We will create a simple blob first, the we
// ask it back using streaming features of the future crate. In this simple example we just
// concatenate the data received in order to make sure the retrieved blob is equals to the one
// created in the first place.
// We do not use leases here but you definitely want to do so otherwise the returned stream
// is not guaranteed to be consistent.
#[tokio::main]
async fn main() -> Result<()> {
    let file_name = "azure_sdk_for_rust_stream_test.txt";

    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as first command line parameter");

    let http_client = azure_core::new_http_client();

    let blob_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
            .as_container_client(&container_name)
            .as_blob_client(file_name);

    let mut stream = Box::pin(get_blob_stream(&blob_client));

    while let Some(res) = stream.next().await {
        println!("{:?}", res.unwrap());
    }

    Ok(())
}

fn get_blob_stream(
    blob_client: &'_ BlobClient,
) -> impl futures::Stream<Item = Result<GetBlobResponse>> + '_ {
    let stream = blob_client.get().stream(1024);
    stream
}

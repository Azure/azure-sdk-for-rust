use azure_core::prelude::*;
use azure_storage::blob::blob::responses::GetBlobResponse;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use futures::stream::TryStreamExt;
use futures::AsyncBufReadExt;
use std::io;
// This example shows how to async read data from a blob by wrapping the stream.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file_name = "azure_sdk_for_rust_async_read_test.txt";

    // First we retrieve the account name and master key from environment variables.
    let connection_string =
        std::env::var("CONNECTION_STRING").expect("Set env variable CONNECTION_STRING first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as first command line parameter");

    let http_client = new_http_client();

    let storage_client =
        StorageAccountClient::new_connection_string(http_client.clone(), &connection_string)?
            .as_storage_client();
    let blob = storage_client
        .as_container_client(&container_name)
        .as_blob_client(file_name);

    let mut reader = Box::pin(
        get_blob_stream(&blob).map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", &e))),
    )
    .into_async_read();

    let mut buf = String::new();
    let mut i = 0;
    while reader.read_line(&mut buf).await? > 0 {
        println!("line {}: {}", i, buf);
        i += 1;
        buf.clear();
    }

    Ok(())
}

fn get_blob_stream<'a>(
    blob: &'a BlobClient,
) -> impl futures::Stream<Item = Result<GetBlobResponse, Box<dyn std::error::Error + Send + Sync>>> + 'a
{
    let stream = blob.get().stream(1024);
    stream
}

use azure_core::error::{ErrorKind, ResultExt};
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;

// This example shows how to stream data from a blob. We will create a simple blob first, the we
// ask it back using streaming features of the future crate. In this simple example we just
// concatenate the data received in order to make sure the retrieved blob is equals to the one
// created in the first place.
// We do not use leases here but you definitely want to do so otherwise the returned stream
// is not guaranteed to be consistent.
#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let file_name = "azure_sdk_for_rust_stream_test.txt";

    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as first command line parameter");

    let blob_client = StorageClient::new_access_key(&account, &access_key)
        .container_client(&container_name)
        .blob_client(file_name);

    let string = "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF";

    let _response = blob_client
        .put_block_blob(string)
        .content_type("text/plain")
        .into_future()
        .await?;

    println!("{}/{} blob created!", container_name, file_name);

    // this is how you stream data from azure blob. Notice that you have
    // to specify the range requested. Also make sure to specify how big
    // a chunk is going to be. Bigger chunks are of course more efficient as the
    // http overhead will be less but it also means you will have to wait for more
    // time before receiving anything. In this example we use a very small chunk size
    // just to make sure to loop at least twice.
    let mut stream = blob_client.get().chunk_size(128u64).into_stream();

    let mut result = vec![];

    // The stream is composed of individual calls to the get blob endpoint
    while let Some(value) = stream.next().await {
        let mut body = value?.data;
        // For each response, we stream the body instead of collecting it all into one large allocation.
        while let Some(value) = body.next().await {
            let value = value?;
            println!("received {:?} bytes", value.len());
            result.extend(&value);
        }
    }

    let returned_string = { String::from_utf8(result).map_kind(ErrorKind::DataConversion)? };

    // You can of course conctenate all the
    // pieces as shown below.
    // It generally does not make sense as you
    // will lose the ability to process the data as it
    // comes in.
    //
    //let fut = stream.concat2().map(|res| {
    //    println!("all blocks received");
    //    res
    //});
    //
    //let result = reactor.run(fut)?;
    //let returned_string = String::from_utf8(result)?;

    println!("{}", returned_string);

    assert!(
        string == returned_string,
        "string = {}, returned_string = {}",
        string,
        returned_string
    );

    blob_client
        .delete()
        .delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .into_future()
        .await?;

    Ok(())
}

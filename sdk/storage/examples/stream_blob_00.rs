use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use futures::stream::StreamExt;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

// This example shows how to stream data from a blob. We will create a simple blob first, the we
// ask it back using streaming features of the future crate. In this simple example we just
// concatenate the data received in order to make sure the retrieved blob is equals to the one
// created in the first place.
// We do not use leases here but you definitely want to do so otherwise the returned stream
// is not guaranteed to be consistent.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file_name = "azure_sdk_for_rust_stream_test.txt";

    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as first command line parameter");

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);
    let storage_client = storage_account_client.as_storage_client();
    let blob = storage_client
        .as_container_client(&container_name)
        .as_blob_client(file_name);

    let string = "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF";

    let _response = blob
        .put_block_blob(string)
        .content_type("text/plain")
        .execute()
        .await?;

    println!("{}/{} blob created!", container_name, file_name);

    // this is how you stream data from azure blob. Notice that you have
    // to specify the range requested. Also make sure to specify how big
    // a chunk is going to be. Bigger chunks are of course more efficient as the
    // http overhead will be less but it also means you will have to wait for more
    // time before receiving anything. In this example we use a very small chunk size
    // just to make sure to loop at least twice.
    let mut stream = Box::pin(blob.get().stream_client_chunk(128));

    let result = Rc::new(RefCell::new(Vec::new()));

    {
        let mut res_closure = result.borrow_mut();
        while let Some(value) = stream.next().await {
            let mut value = value?.data.to_vec();
            println!("received {:?} bytes", value.len());
            res_closure.append(&mut value);
        }
    }

    let returned_string = {
        let rlock = result.borrow();
        String::from_utf8(rlock.to_vec())?
    };

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

    blob.delete()
        .delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .execute()
        .await?;

    Ok(())
}

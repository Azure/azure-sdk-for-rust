#![cfg(all(test, feature = "test_e2e"))]
use azure_sdk_core::prelude::*;
use azure_sdk_core::{range::Range, DeleteSnapshotsMethod};
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;
use futures::stream::StreamExt;

#[tokio::test]
async fn create_blob_and_stream_back() {
    code().await.unwrap();
}

async fn code() -> Result<(), Box<dyn std::error::Error>> {
    let container_name = "azuresdkforrust";
    let file_name = "azure_sdk_for_rust_stream_test.txt";

    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let client = client::with_access_key(&account, &master_key);

    if client
        .list_containers()
        .finalize()
        .await?
        .incomplete_vector
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        client
            .create_container()
            .with_container_name(container_name)
            .with_public_access(PublicAccess::Blob)
            .finalize()
            .await?;
    }

    let string = "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF";

    client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(file_name)
        .with_content_type("text/plain")
        .with_body(string.as_ref())
        .finalize()
        .await?;

    println!("{}/{} blob created!", container_name, file_name);

    for dropped_suffix_len in &[3usize, 2, 1, 0] {
        // this is how you stream data from azure blob. Notice that you have
        // to specify the range requested. Also make sure to specify how big
        // a chunk is going to be. Bigger chunks are of course more efficient as the
        // http overhead will be less but it also means you will have to wait for more
        // time before receiving anything. In this example we use an awkward value
        // just to make the test worthwile.
        let slice_range = 0..(string.len() - dropped_suffix_len);
        let expected_string = &string[slice_range.clone()];
        let range: Range = slice_range.into();

        let chunk_size: usize = 4;

        let mut stream = Box::pin(
            client
                .stream_blob()
                .with_container_name(&container_name)
                .with_blob_name(file_name)
                .with_range(&range)
                .with_chunk_size(chunk_size as u64)
                .finalize(),
        );

        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));

        {
            let mut res_closure = result.borrow_mut();
            while let Some(value) = stream.next().await {
                let mut value = value?;
                assert!(value.len() <= chunk_size);
                println!("received {:?} bytes", value.len());
                res_closure.append(&mut value);
            }
        }

        let returned_string = {
            let rlock = result.borrow();
            String::from_utf8(rlock.to_vec())?
        };

        println!(
            "dropped_suffix_len == {} returned_string == {}",
            dropped_suffix_len, returned_string
        );

        assert_eq!(expected_string, returned_string);
    }

    client
        .delete_blob()
        .with_container_name(&container_name)
        .with_blob_name(file_name)
        .with_delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .finalize()
        .await?;

    println!("{}/{} blob deleted!", container_name, file_name);

    Ok(())
}

#![cfg(all(test, feature = "test_e2e"))]
use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use futures::stream::StreamExt;

#[tokio::test]
async fn create_blob_and_stream_back() {
    code().await.unwrap();
}

async fn code() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let container_name = "azuresdkforrust";
    let file_name = "azure_sdk_for_rust_stream_test.txt";

    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let http_client = new_http_client();

    let storage = StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
        .as_storage_client();
    let container = storage.as_container_client(container_name);
    let blob = container.as_blob_client(file_name);

    if storage
        .list_containers()
        .execute()
        .await?
        .incomplete_vector
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        container
            .create()
            .public_access(PublicAccess::None)
            .execute()
            .await?;
    }

    let string = "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF";

    blob.put_block_blob(string)
        .content_type("text/plain")
        .execute()
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

        let chunk_size = 8;

        let mut stream = Box::pin(blob.get().range(range).stream(chunk_size));

        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));

        {
            let mut res_closure = result.borrow_mut();
            while let Some(value) = stream.next().await {
                let mut value = value?.data.to_vec();
                assert!(value.len() as u64 <= chunk_size);
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

    // test streaming a blob smaller than the chunk size issue 239.
    let mut stream = Box::pin(blob.get().stream(1024 * 8));
    while let Some(_value) = stream.next().await {}

    blob.delete()
        .delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .execute()
        .await?;

    println!("{}/{} blob deleted!", container_name, file_name);

    Ok(())
}

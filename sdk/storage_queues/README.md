# azure_storage_queues

> Microsoft is developing the official Azure SDK for Rust crates and has no plans to update this unofficial crate.
> In the future we may release an official version that may have a different package name.
> If releasing an official version of this crate is important to you [let us know](https://github.com/Azure/azure-sdk-for-rust/issues/new/choose).
>
> Source for this crate can now be found in <https://github.com/Azure/azure-sdk-for-rust/tree/legacy>.
> To monitor for an official, supported version of this crate, see <https://aka.ms/azsdk/releases>.

## The Azure Storage Queue crate

This crate is from the [Azure SDK for Rust](https://github.com/azure/azure-sdk-for-rust).
It supports [Azure Queue Storage](https://docs.microsoft.com/azure/storage/queues/storage-queues-introduction).

## Example

```rust no_run
use azure_storage::prelude::*;
use azure_storage_queues::prelude::*;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
    let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCESS_KEY");
    let queue_name = std::env::var("STORAGE_QUEUE_NAME").expect("missing STORAGE_QUEUE_NAME");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let queue_service = QueueServiceClient::new(account, storage_credentials);
    let queue = queue_service.queue_client(queue_name);

    // process messages until there are no more
    loop {
        let response = queue.get_messages().await?;
        if response.messages.is_empty() {
            break;
        }
        for message in response.messages {
            println!("processing message {:?}", message);
            queue.pop_receipt_client(message).delete().await?;
        }
    }

    Ok(())
}

```


License: MIT

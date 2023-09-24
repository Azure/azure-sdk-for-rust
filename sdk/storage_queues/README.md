# azure_storage_queues

## The Azure Storage Queue crate

This crate is from the [Azure SDK for Rust](https://github.com/azure/azure-sdk-for-rust).
It supports [Azure Queue Storage](https://docs.microsoft.com/azure/storage/queues/storage-queues-introduction).

## Example
```rust
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

/*!
# The Azure Storage Queue crate

This crate is from the [Azure SDK for Rust](https://github.com/azure/azure-sdk-for-rust).
It supports [Azure Queue Storage](https://docs.microsoft.com/azure/storage/queues/storage-queues-introduction).

# Example
```no_run
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

*/

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_core;

mod clients;
mod message_ttl;
mod number_of_messages;
pub mod operations;
mod pop_receipt;
pub mod prelude;
mod queue_message;
mod queue_service_properties;
mod queue_stored_access_policy;
mod visibility_timeout;

pub use clients::*;
pub use message_ttl::MessageTTL;
pub use number_of_messages::NumberOfMessages;
pub use pop_receipt::PopReceipt;
pub use queue_service_properties::QueueServiceProperties;
pub use queue_stored_access_policy::QueueStoredAccessPolicy;
pub use visibility_timeout::VisibilityTimeout;

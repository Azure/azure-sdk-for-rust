# azure_messaging_eventhubs

Azure Eventhubs crate for the Microsoft Azure SDK for Rust.
This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).

## Example

```no_run,rust
use azure_messaging::*;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let eventhubs_namespace = std::env::var("AZURE_EVENTHUB_NAMESPACE").expect("missing AZURE_EVENTHUB_NAMESPACE");

    let  client = ProducerClient::new(
        eventhubs_namespace, credential,
    )?;

    client.send_event("hello world").await?;

    Ok(())
}
```

License: MIT

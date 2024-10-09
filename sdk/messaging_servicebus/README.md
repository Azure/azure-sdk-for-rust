# azure_messaging_servicebus

> Microsoft is developing the official Azure SDK for Rust crates and has no plans to update this unofficial crate.
> In the future we may release an official version that may have a different package name.
> If releasing an official version of this crate is important to you [let us know](https://github.com/Azure/azure-sdk-for-rust/issues/new/choose).
>
> Source for this crate can now be found in <https://github.com/Azure/azure-sdk-for-rust/tree/legacy>.
> To monitor for an official, supported version of this crate, see <https://aka.ms/azsdk/releases>.

Azure Service Bus crate for the unofficial Microsoft Azure SDK for Rust.
This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).

## Example
```no_run,rust
use azure_messaging_servicebus::prelude::*;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let service_bus_namespace = std::env::var("AZURE_SERVICE_BUS_NAMESPACE").expect("missing AZURE_SERVICE_BUS_NAMESPACE");
    let queue_name = std::env::var("AZURE_QUEUE_NAME").expect("missing AZURE_QUEUE_NAME");
    let policy_name = std::env::var("AZURE_POLICY_NAME").expect("missing AZURE_POLICY_NAME");
    let policy_key = std::env::var("AZURE_POLICY_KEY").expect("missing AZURE_POLICY_KEY");

    let http_client = azure_core::new_http_client();
    let mut client = QueueClient::new(
        http_client,
        service_bus_namespace,
        queue_name,
        policy_name,
        policy_key,
    )?;

    client.send_message("hello world", None).await?;

    let received_message = client.receive_and_delete_message().await?;
    println!("Received Message: {}", received_message);

    Ok(())
}
```

License: MIT

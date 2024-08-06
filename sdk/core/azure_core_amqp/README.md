<!-- cspell: words  -->

# azure_core_amqp

Azure AMQP crate for consumption of AMQP based packages in the Azure SDK for C++.

NOTE: THIS IS NOT A GENERAL PURPOSE AMQP LIBRARY AND SHOULD NOT BE USED AS SUCH.

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

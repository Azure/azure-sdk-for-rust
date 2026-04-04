# Azure VMware Solution client library for Rust

Azure VMware Solution (AVS) is a Microsoft service that runs VMware workloads
natively on Azure infrastructure. The `azure_resourcemanager_vmware` crate
provides Rust bindings for managing AVS private clouds, clusters, datastores,
and related resources through the Microsoft.AVS resource provider.

- [Package (crates.io)](https://crates.io/crates/azure_resourcemanager_vmware)
- [API reference documentation](https://docs.rs/azure_resourcemanager_vmware)
- [Product documentation](https://learn.microsoft.com/azure/azure-vmware/)
- [Source code](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/avs/azure_resourcemanager_vmware)

## Getting started

### Install the package

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
azure_resourcemanager_vmware = "0.1.0"
azure_identity = "0.30.0"
```

### Prerequisites

- An [Azure subscription](https://azure.microsoft.com/free/)
- An Azure VMware Solution private cloud, or permissions to create one

### Authenticate the client

The client uses Entra ID (Azure Active Directory) authentication. During
development you can use `DeveloperToolsCredential`, which chains the Azure
CLI, Azure PowerShell, and other developer credentials:

```rust no_run
use azure_identity::DeveloperToolsCredential;
use azure_resourcemanager_vmware::{AVSClient, AVSClientOptions};
use std::sync::Arc;

let credential = Arc::new(DeveloperToolsCredential::new(None)?);
let client = AVSClient::new(
    "https://management.azure.com",
    credential,
    "<subscription-id>".to_string(),
    None,
)?;
```

## Key concepts

### AVSClient

The [`AVSClient`] is the primary entry point. It provides access to
sub-clients for each resource type (private clouds, clusters, hosts,
datastores, and more).

## Examples

### List private clouds

```rust no_run
use azure_identity::DeveloperToolsCredential;
use azure_resourcemanager_vmware::{AVSClient, AVSClientOptions};
use std::sync::Arc;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let credential = Arc::new(DeveloperToolsCredential::new(None)?);
    let client = AVSClient::new(
        "https://management.azure.com",
        credential,
        "<subscription-id>".to_string(),
        None,
    )?;

    let private_clouds_client = client.private_clouds_client();
    let mut pager = private_clouds_client
        .list_in_subscription()
        .send()
        .await?
        .into_body()
        .await?;

    // Process the results
    for cloud in &pager.value {
        println!("{:?}", cloud);
    }

    Ok(())
}
```

## Troubleshooting

### General

Azure VMware Solution clients raise errors from `azure_core::Error`. For
example, attempting to access a resource that does not exist returns a 404:

```rust no_run
use azure_core::error::ErrorKind;

match result {
    Err(e) if matches!(e.kind(), ErrorKind::HttpResponse { status, .. } if *status == 404) => {
        println!("Resource not found");
    }
    Err(e) => return Err(e.into()),
    Ok(response) => { /* use response */ }
}
```

## Contributing

This project welcomes contributions and suggestions. Most contributions require
you to agree to a Contributor License Agreement (CLA) declaring that you have
the right to, and actually do, grant us the rights to use your contribution.
For details, visit <https://cla.microsoft.com>.

When you submit a pull request, a CLA-bot will automatically determine whether
you need to provide a CLA and decorate the PR appropriately (e.g., label,
comment). Simply follow the instructions provided by the bot. You will only need
to do this once across all repos using our CLA.

This project has adopted the
[Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information, see the
[Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or
contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any
additional questions or comments.

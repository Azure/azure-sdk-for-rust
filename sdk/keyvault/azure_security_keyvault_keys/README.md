# Azure KeyVault keys client library for Rust

Azure Key Vault is a cloud service that provides secure storage and management of sensitive information such as API keys, passwords, certificates, and cryptographic keys. The `azure_security_keyvault_keys` crate provides a client library for interacting with the Azure Key Vault Keys service.

## Getting started

### Install the crate

Install the Azure Key Vault keys library for Rust with cargo: 

```bash
cargo add azure_security_keyvault_keys

### Prerequisites

- An [Azure subscription](https://azure.microsoft.com/free/)
- A [Key Vault resource](https://docs.microsoft.com/azure/key-vault/quick-create-portal)

### Authentication

This crate uses the [azure_identity](https://crates.io/crates/azure_identity) crate for authentication. You can authenticate using a variety of methods, including environment variables, managed identity, and more.

## Key concepts

### KeyClient

The `KeyClient` struct provides methods to manage keys in the Azure Key Vault. You can create, retrieve, update, and delete keys, as well as perform cryptographic operations.

## Examples

### Creating a key

```rust

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let credential = DefaultAzureCredential::new().unwrap();

    let mut options = KeyClientOptions::default();

    let client = KeyClient::new(
        "https://<your-key-vault-name>.vault.azure.net",
        credential.clone(),
        Some(options));

    let key = client.create_key("my-key", None).await?.into_body().await?;
    println!("Created key: {:?}", key);

    Ok(())
}
```

## Contributing

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit [https://cla.microsoft.com](https://cla.microsoft.com).

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/). For more information, see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

# azure-sdk-keyvault

[![Crates.io](https://img.shields.io/crates/v/azure-sdk-keyvault)](https://crates.io/crates/azure-sdk-keyvault)
[![Crates.io](https://img.shields.io/crates/l/azure-sdk-keyvault)](https://crates.io/crates/azure-sdk-keyvault)
[![Build Status](https://travis-ci.org/guywaldman/azure-sdk-keyvault.svg?branch=master)](https://travis-ci.org/guywaldman/azure-sdk-keyvault)

> 🚧 Work in progress, API is prone to changes. 🚧

## About this Crate

[Azure Key Vault](https://azure.microsoft.com/en-us/services/key-vault/) is a service in Microsoft Azure for securely storing and accessing secrets, credentials and certificates in the cloud.
This crate exposes Rust bindings for the Azure Key Vault [REST API](https://docs.microsoft.com/en-us/rest/api/keyvault/).

This was started as a standalone contribution to [MindFlavor/AzureSDKForRust](https://github.com/MindFlavor/AzureSDKForRust),
which has many other useful Azure REST API bindings for Rust.

## Important Disclaimer

I am a Microsoft employee, but this is not an official Microsoft product nor an endorsed product.
Purely a project for fun and for learning Rust.

## Example Usage

```rust
use azure_sdk_keyvault::KeyVaultClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = KeyVaultClient::new(&"c1a6d79b-082b-4798-b362-a77e96de50db", &"SUPER_SECRET_KEY", &"bc598e67-03d8-44d5-aa46-8289b9a39a14", &"test-keyvault");

    // Set a secret.
    client.set_secret("test-secret", "42").await?;

    // Get a secret.
    let secret = client.get_secret("test-secret").await?;
    assert_eq!("42", secret.value());

    Ok(())
}
```

## Features

### Secrets

- Get secret
- Get secret versions
- List secrets
- Set secret
- Update secret
- Delete secret
- Restore secret
- Backup secret

## Contributions

...are welcome! Currently the repo exposes a very small number of operations.

## Related Work

This project was started from the fantastic [MindFlavor/AzureSDKForRust](https://github.com/MindFlavor/AzureSDKForRust) repo.

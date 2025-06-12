# Azure Core shared client library for Rust

`azure_core` provides shared primitives, abstractions, and helpers for modern Rust Azure SDK client libraries.
These libraries follow the [Azure SDK Design Guidelines for Rust][guidelines]
and can typically be identified by package and namespaces names starting with `azure_`, e.g. `azure_identity`.

`azure_core` allows client libraries to expose common functionality in a consistent fashion
so that once you learn how to use these APIs in one client library, you will know how to use them in other client libraries.

[Source code] | [Package (crates.io)] | [API Reference Documentation]

## Getting started

Typically, you will not need to install `azure_core`;
it will be installed for you when you install one of the client libraries using it.
In case you want to install it explicitly - to implement your own client library, for example -
you can find the [package on crates.io][Package (crates.io)].

## Key concepts

The main shared concepts of `azure_core` - and Azure SDK libraries using `azure_core` - include:

- Configuring service clients, e.g. configuring retries, logging (`ClientOptions`).
- Accessing HTTP response details (`Response<T>`).
- Paging and asynchronous streams (`Pager<T>`).
- Errors from service requests in a consistent fashion. (`azure_core::Error`).
- Customizing requests (`ClientOptions`).
- Abstractions for representing Azure SDK credentials. (`TokenCredentials`).

### Thread safety

We guarantee that all client instance methods are thread-safe and independent of each other ([guidelines]). This ensures that the recommendation of reusing client instances is always safe, even across threads.

### Additional concepts

<!-- CLIENT COMMON BAR -->
[Client options](#configuring-service-clients-using-clientoptions) |
[Accessing the response](#accessing-http-response-details-using-responset) |
[Handling Errors Results](#handling-errors-results) |
[Consuming Service Methods Returning `Pager<T>`](#consuming-service-methods-returning-pagert)
<!-- CLIENT COMMON BAR -->

## Features

- `debug`: enables extra information for developers e.g., emitting all fields in `std::fmt::Debug` implementation.
- `hmac_openssl`: configures HMAC using `openssl`.
- `hmac_rust`: configures HMAC using pure Rust.
- `reqwest` (default): enables and sets `reqwest` as the default `HttpClient`. Enables `reqwest`'s `native-tls` feature.
- `reqwest_deflate` (default): enables deflate compression for `reqwest`.
- `reqwest_gzip` (default): enables gzip compression for `reqwest`.
- `reqwest_rustls`: enables `reqwest`'s `rustls-tls-native-roots-no-provider` feature,
- `tokio`: enables and sets `tokio` as the default async runtime.
- `xml`: enables XML support.

## Examples

**NOTE:** Samples in this file apply only to packages that follow [Azure SDK Design Guidelines][guidelines]. Names of such packages typically start with `azure_`.

### Configuring service clients using `ClientOptions`

Azure SDK client libraries typically expose one or more _service client_ types that
are the main starting points for calling corresponding Azure services.
You can easily find these client types as their names end with the word _Client_.
For example, `SecretClient` can be used to call the Key Vault service and interact with secrets,
and `KeyClient` can be used to access Key Vault service cryptographic keys.

These client types can be instantiated by calling a simple `new` function that takes various configuration options.These options are passed as a parameter that extends `ClientOptions` class exposed by `azure_core`.
Various service specific options are usually added to its subclasses, but a set of SDK-wide options are
available directly on `ClientOptions`.

```rust no_run
use azure_core::http::ClientOptions;
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{SecretClient, SecretClientOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;

    let options = SecretClientOptions {
        api_version: "7.5".to_string(),
        ..Default::default()
    };

    let client = SecretClient::new(
        "https://<your-key-vault-name>.vault.azure.net/",
        credential.clone(),
        Some(options),
    )?;

    Ok(())
}
```

### Accessing HTTP response details using `Response<T>`

_Service clients_ have methods that can be used to call Azure services. We refer to these client methods as _service methods_.
_Service methods_ return a shared `azure_core` type `Response<T>` where `T` is either a `Model` type or a `ResponseBody` representing a raw stream of bytes.
This type provides access to both the deserialized result of the service call, and to the details of the HTTP response returned from the server.

```rust no_run
use azure_core::http::Response;
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{models::Secret, SecretClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a client
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://<your-key-vault-name>.vault.azure.net/",
        credential.clone(),
        None,
    )?;

    // call a service method, which returns Response<T>
    let response = client.get_secret("secret-name", "", None).await?;

    // Response<T> has two main accessors:
    // 1. The `into_body()` function consumes self to deserialize into a model type
    let secret = response.into_body().await?;

    // get response again because it was moved in above statement
    let response: Response<Secret> = client.get_secret("secret-name", "", None).await?;

    // 2. The deconstruct() method for accessing all the details of the HTTP response
    let (status, headers, body) = response.deconstruct();

    // for example, you can access HTTP status
    println!("Status: {}", status);

    // or the headers
    for (header_name, header_value) in headers.iter() {
        println!("{}: {}", header_name.as_str(), header_value.as_str());
    }

    Ok(())
}
```

### Handling errors results

When a service call fails, the returned `Result` will contain an `Error`. The `Error` type provides a status property with an HTTP status code and an error_code property with a service-specific error code.

```rust no_run
use azure_core::{error::{ErrorKind, HttpError}, http::{Response, StatusCode}};
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::SecretClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a client
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://<your-key-vault-name>.vault.azure.net/",
        credential.clone(),
        None,
    )?;

    match client.get_secret("secret-name", "", None).await {
        Ok(secret) => println!("Secret: {:?}", secret.into_body().await?.value),
        Err(e) => match e.kind() {
            ErrorKind::HttpResponse { status, error_code, .. } if *status == StatusCode::NotFound => {
                // handle not found error
                if let Some(code) = error_code {
                    println!("ErrorCode: {}", code);
                } else {
                    println!("Secret not found, but no error code provided.");
                }
            },
            _ => println!("An error occurred: {e:?}"),
        },
    }

    Ok(())
}
```

### Consuming service methods returning `Pager<T>`

If a service call returns multiple values in pages, it would return `Result<Pager<T>>` as a result. You can iterate all items from all pages.

```rust no_run
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{ResourceExt, SecretClient};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a client
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://<your-key-vault-name>.vault.azure.net/",
        credential.clone(),
        None,
    )?;

    // get a stream of items
    let mut pager = client.list_secret_properties(None)?;

    // poll the pager until there are no more SecretListResults
    while let Some(secret) = pager.try_next().await? {
        // get the secret name from the ID
        let name = secret.resource_id()?.name;
        println!("Found secret with name: {}", name);
    }

    Ok(())
}
```

To instead iterate over all pages, call `into_pages()` on the returned `Pager`.

```rust no_run
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{ResourceExt, SecretClient};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a client
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://<your-key-vault-name>.vault.azure.net/",
        credential.clone(),
        None,
    )?;

    // get a stream of pages
    let mut pager = client.list_secret_properties(None)?.into_pages();

    // poll the pager until there are no more SecretListResults
    while let Some(secrets) = pager.try_next().await? {
        let secrets = secrets.into_body().await?.value;
        // loop through secrets in SecretsListResults
        for secret in secrets {
            // get the secret name from the ID
            let name = secret.resource_id()?.name;
            println!("Found secret with name: {}", name);
        }
    }

    Ok(())
}
```

## Troubleshooting

### Logging

To help protected end users from accidental Personally-Identifiable Information (PII) from leaking into logs or traces, models' default implementation of `core::fmt::Debug` formats as non-exhaustive structure tuple e.g.,

```rust no_run
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{ResourceExt, SecretClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a client
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://<your-key-vault-name>.vault.azure.net/",
        credential.clone(),
        None,
    )?;

    // get a secret
    let secret = client.get_secret("secret-name", "", None)
        .await?
        .into_body()
        .await?;

    println!("{secret:#?}");

    Ok(())
}
```

By default this will print:

```text
Secret { .. }
```

Though not recommended for production, you can enable normal `core::fmt::Debug` formatting complete with field names and values by enabling the `debug` feature of `azure_core` e.g.,

```sh
cargo add azure_core -F debug
```

## Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to these libraries.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit <https://opensource.microsoft.com/cla/>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/core/azure_core/src
[Package (crates.io)]: https://crates.io/crates/azure_core
[API Reference Documentation]: https://docs.rs/azure_core
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/
[guidelines]: https://azure.github.io/azure-sdk/rust_introduction.html

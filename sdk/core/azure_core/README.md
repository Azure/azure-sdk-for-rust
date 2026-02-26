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
[Sending an HTTP request body](#sending-an-http-request-body) |
[Accessing the response](#accessing-http-response-details-using-responset) |
[Handling errors](#handling-errors) |
[Iterating through pages of resources](#consuming-service-methods-returning-pagert) |
[Waiting on long-running operations](#consuming-service-methods-returning-pollert) |
[Replacing the HTTP client](#replacing-the-http-client) |
[Replacing the async runtime](#replacing-the-async-runtime)

<!-- CLIENT COMMON BAR -->

## Features

- `debug`: enables extra information for developers e.g., emitting all fields in `std::fmt::Debug` implementation and no PII sanitization.
- `decimal`: enables support for `rust_decimal::Decimal` type.
- `derive`: enable derive macros e.g., `SafeDebug`.
- `hmac_openssl`: enables HMAC signing using `openssl`. If both `hmac_openssl` and `hmac_rust` are enabled, `hmac_openssl` is used.
- `hmac_rust`: enables HMAC signing using rust-native libraries `sha2` and `hmac`. If both `hmac_openssl` and `hmac_rust` are enabled, `hmac_openssl` is used.
- `reqwest` (default): enables and sets `reqwest` as the default `HttpClient`.
- `reqwest_deflate` (default): enables deflate compression for `reqwest`.
- `reqwest_gzip` (default): enables gzip compression for `reqwest`.
- `reqwest_native_tls` (default): enables `reqwest`'s `native-tls` feature, which uses schannel on Windows and openssl elsewhere.
- `tokio`: enables and sets `tokio` as the default async runtime.
- `xml`: enables XML support.

### Enabling dependencies' features

We define features to avoid dependencies which may be unnecessary for some applications or even some client libraries e.g.,
some Azure services do not support the `accept-encoding` request header nor send `content-encoding` back so there's no need to add `reqwest_gzip` by default
for all client libraries. We also want to support developers that want to use additional features of some dependencies like `reqwest`
or even replace some dependencies completely like `reqwest` or `tokio` to use different HTTP clients or async runtimes.

We do not define features to provide parity with all dependencies' features since the [resolver](https://doc.rust-lang.org/cargo/reference/resolver.html)
will unify features e.g., you can add `reqwest`'s `system-proxy` feature without making any changes to Azure SDK dependencies:

```toml
[dependencies]
azure_identity = "1"
azure_security_keyvault_secrets = "1"
reqwest = { version = "0.13.1", features = ["system-proxy"] }
```

Similarly, you can choose to support `reqwest::Client` but use a different TLS provider by disabling our default features and adding
only what you need. For example, adding our `reqwest` feature to enable the `HttpClient` trait implementation on `reqwest::Client` and
a dependency on `reqwest` with the feature you want:

```toml
[dependencies]
azure_core = { version = "1", default-features = false, features = ["reqwest"] }
azure_identity = { version = "1", default-features = false }
azure_security_keyvault_secrets = { version = "1", default-features = false }
reqwest = { version = "0.13.1", features = ["rustls"] }
```

You could even completely replace `reqwest` and provide your own `HttpClient` implementation. See [an example](#other-http-client) below.

**NOTE:** The `debug` feature may expose PII and/or secrets to logs or tracing spans which would normally be redacted.

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

```rust ignore new-client
use azure_identity::DeveloperToolsCredential;
use azure_security_keyvault_secrets::{SecretClient, SecretClientOptions};

let credential = DeveloperToolsCredential::new(None)?;

let options = SecretClientOptions {
    api_version: "7.5".to_string(),
    ..Default::default()
};

let client = SecretClient::new(
    "https://<your-key-vault-name>.vault.azure.net/",
    credential.clone(),
    Some(options),
)?;
```

### Sending an HTTP request body

_Service clients_ have methods that can be used to call Azure services. We refer to these client methods as _service methods_.
Some service clients have methods have parameters for required path components, query string parameters, or request bodies.
You can serialize request bodies from a model or from a formatted string such as raw JSON.

Using the `client` we instantiated above:

```rust ignore request
use azure_security_keyvault_secrets::models::UpdateSecretPropertiesParameters;
use std::collections::HashMap;

let tags = HashMap::from([
    ("classification".into(), "example".into()),
]);
#[allow(clippy::needless_update)]
let parameters = UpdateSecretPropertiesParameters {
    content_type: Some("text/plain".into()),
    secret_attributes: None, // No change
    tags: Some(tags),
    ..Default::default()
};

client.update_secret_properties("secret-name", parameters.try_into()?, None).await?;
```

**NOTE:** Service models and other structs that can be created by the caller are not attributed with `#[non_exhaustive]` to allow struct initialization.
To mitigate breaking changes should fields be added, we recommend the struct update syntax with `Default` even if you assign all fields.
You can attribute the struct initialization, module, or crate to ignore `clippy::needless_update` as shown above.

### Accessing HTTP response details using `Response<T>`

_Service clients_ have methods that can be used to call Azure services. We refer to these client methods as _service methods_.
_Service methods_ return a shared `azure_core` type `Response<T>` where `T` is either a `Model` type or a `ResponseBody` representing a raw stream of bytes.
This type provides access to both the deserialized result of the service call, and to the details of the HTTP response returned from the server.

Using the `client` we instantiated above:

```rust ignore response
// Call a service method, which returns `Response<T>`.
let response = client.get_secret("secret-name", None).await?;

// `Response<T>` has two main accessors:
// 1. The `into_model()` function consumes self to deserialize into a model type defined by the client library.
let secret = response.into_model()?;

// Get response again because it was moved in above statement.
let response = client.get_secret("secret-name", None).await?;

// 2. The `deconstruct()` method for accessing all the details of the HTTP response.
let (status, headers, body) = response.deconstruct();

// For example, you can access HTTP status.
println!("Status: {}", status);

// ...or the headers.
for (header_name, header_value) in headers.iter() {
    println!("{}: {}", header_name.as_str(), header_value.as_str());
}
```

### Handling errors

When a service call fails, the returned `Result` will contain an `Error`. The `Error` type provides a status property with an HTTP status code and an error_code property with a service-specific error code.

Using the `client` we instantiated above:

```rust ignore errors
use azure_core::{error::ErrorKind, http::StatusCode};

match client.get_secret("secret-name", None).await {
    Ok(secret) => println!("Secret: {:?}", secret.into_model()?.value),
    Err(e) => match e.kind() {
        ErrorKind::HttpResponse { status, error_code, .. } if *status == StatusCode::NotFound => {
            // Handle not found error.
            if let Some(code) = error_code {
                println!("ErrorCode: {}", code);
            } else {
                println!("Secret not found, but no error code provided.");
            }
        },
        _ => println!("An error occurred: {e:?}"),
    },
}
```

Most Azure services return a standard error response model, which will populate the `status` and `error_code` with more specific information.
The `raw_response` field (elided above) will always contain the `RawResponse` including the error response body in its entirety,
and you can [deserialize](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/azure_core/examples/core_error_response.rs) the standard error response model to get more information.

### Consuming service methods returning `Pager<T>`

When a service returns a pageable collection, it will return `Result<Pager<T>>` as a result. You can iterate all items from all pages.

Using the `client` we instantiated above:

```rust ignore item-pager
use azure_security_keyvault_secrets::ResourceExt;
use futures::TryStreamExt as _;

// Get a stream of items.
let mut pager = client.list_secret_properties(None)?;
while let Some(secret) = pager.try_next().await? {
    // Get the secret name from the ID.
    let name = secret.resource_id()?.name;
    println!("Found secret with name: {}", name);
}
```

To instead iterate over all pages, call `into_pages()` on the returned `Pager`.

```rust ignore page-pager
use azure_security_keyvault_secrets::ResourceExt;
use futures::TryStreamExt as _;

// Get a stream of pages.
let mut pager = client.list_secret_properties(None)?.into_pages();

// Poll the pager until there are no more `SecretListResults`.
while let Some(secrets) = pager.try_next().await? {
    let secrets = secrets.into_model()?.value;
    // Loop through secrets in `SecretsListResults`.
    for secret in secrets {
        // Get the secret name from the ID.
        let name = secret.resource_id()?.name;
        println!("Found secret with name: {}", name);
    }
}
```

### Consuming service methods returning `Poller<T>`

If a service call may take a while to process, it should return `Result<Poller<T>>` as a result, representing a long-running operation (LRO).

The `Poller<T>` implements `std::future::IntoFuture` so you can `await` it to get the final result:

Using a `CertificateClient` created similarly to the `client` we instantiated above:

```rust ignore poller-future
use azure_security_keyvault_certificates::{
    models::{CreateCertificateParameters, CertificatePolicy, X509CertificateProperties, IssuerParameters},
};

// Create a self-signed certificate.
let policy = CertificatePolicy {
    x509_certificate_properties: Some(X509CertificateProperties {
        subject: Some("CN=DefaultPolicy".into()),
        ..Default::default()
    }),
    issuer_parameters: Some(IssuerParameters {
        name: Some("Self".into()),
        ..Default::default()
    }),
    ..Default::default()
};
let body = CreateCertificateParameters {
    certificate_policy: Some(policy),
    ..Default::default()
};

// Wait for the certificate operation to complete and get the certificate.
let certificate = client
    .create_certificate("certificate-name", body.try_into()?, None)?
    .await?
    .into_model()?;
```

The `Poller<T>` also implements `futures::Stream` so you can asynchronously iterate over each status monitor update:

```rust ignore poller-stream
use azure_security_keyvault_certificates::{
    models::{CreateCertificateParameters, CertificatePolicy, X509CertificateProperties, IssuerParameters},
};
use futures::stream::TryStreamExt as _;

// Create a self-signed certificate.
let policy = CertificatePolicy {
    x509_certificate_properties: Some(X509CertificateProperties {
        subject: Some("CN=DefaultPolicy".into()),
        ..Default::default()
    }),
    issuer_parameters: Some(IssuerParameters {
        name: Some("Self".into()),
        ..Default::default()
    }),
    ..Default::default()
};
let body = CreateCertificateParameters {
    certificate_policy: Some(policy),
    ..Default::default()
};

// Wait for the certificate operation to complete.
// The Poller implements futures::Stream and automatically waits between polls.
let mut poller = client.create_certificate("certificate-name", body.try_into()?, None)?;
while let Some(operation) = poller.try_next().await? {
    let operation = operation.into_model()?;
    match operation.status.as_deref().unwrap_or("unknown") {
        "inProgress" => continue,
        "completed" => {
            let target = operation.target.ok_or("expected target")?;
            println!("Created certificate {}", target);
            break;
        },
        status => Err(format!("operation terminated with status {status}"))?,
    }
}
```

### Adding HTTP policies

You can add custom HTTP policies for each client method (per-call) or request attempt (per-try) by implementing `Policy` and adding it to the appropriate field on `ClientOptions`.
For example, to remove the `user-agent` header entirely:

```rust ignore custom-policy
use azure_core::http::{
    policies::{Policy, PolicyResult},
    ClientOptions, Context, Request,
};
use azure_identity::DeveloperToolsCredential;
use azure_security_keyvault_secrets::{SecretClient, SecretClientOptions};
use std::sync::Arc;

#[derive(Debug)]
struct RemoveUserAgent;

#[async_trait::async_trait]
impl Policy for RemoveUserAgent {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let headers = request.headers_mut();
        headers.remove("user-agent");

        next[0].send(ctx, request, &next[1..]).await
    }
}

let remove_user_agent = Arc::new(RemoveUserAgent);
let mut options = SecretClientOptions::default();
options
    .client_options
    .per_call_policies
    .push(remove_user_agent);

let credential = DeveloperToolsCredential::new(None)?;
let client = SecretClient::new(
    "https://your-key-vault-name.vault.azure.net/",
    credential.clone(),
    Some(options),
)?;
```

See the [example](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/azure_core/examples/core_remove_user_agent.rs) for a full sample implementation.

### Replacing the HTTP client

Though `azure_core` uses [`reqwest`] for its default HTTP client, you can replace it with either a customized `reqwest::Client` or an entirely different HTTP client.

#### Reqwest

We define a `reqwest` feature that provides a blanket implementation of our `HttpClient` trait for `reqwest::Client` and depends on the `reqwest` crate.
If you just want to configure a `reqwest::Client` to use different options including a different TLS provider, optionally add a dependency on `reqwest` and enable whichever feature you want:

```sh
cargo add reqwest -F rustls-tls-native-roots
```

You can then disable default features of any of the Azure SDK crates and add a dependency on `azure_core` with the `reqwest` feature for the blanket `HttpClient` implementation:

```sh
cargo add azure_core --no-default-features -F reqwest
```

You should end up with a `Cargo.toml` that looks something like:

```toml
[dependencies]
azure_core = { version = "1", default-features = false, features = ["reqwest"] }
azure_identity = { version = "1", default-features = false }
azure_security_keyvault_secrets = { version = "1", default-features = false }
reqwest = { version = "0.12.23", default-features = false, features = [
    "deflate",
    "gzip",
    "rustls-tls-native-roots",
] }
```

In many cases with `reqwest`, importing features may be enough. See their [documentation][`reqwest`] for more information.
If you do need to write code to customize the `reqwest::Client`, you can pass it in `ClientOptions` to our client libraries:

```rust ignore custom-reqwest
use azure_core::http::{ClientOptions, Transport};
use azure_identity::DeveloperToolsCredential;
use azure_security_keyvault_secrets::{SecretClient, SecretClientOptions};
use std::sync::Arc;

let http_client = Arc::new(reqwest::ClientBuilder::new().gzip(true).build()?);

let options = SecretClientOptions {
    client_options: ClientOptions {
        transport: Some(Transport::new(http_client)),
        ..Default::default()
    },
    ..Default::default()
};

let credential = DeveloperToolsCredential::new(None)?;
let client = SecretClient::new(
    "https://your-key-vault-name.vault.azure.net/",
    credential.clone(),
    Some(options),
)?;
```

#### Other HTTP client

If you do not want to take a dependency on [`reqwest`] at all - perhaps because you [want to use a different async runtime](#replacing-the-async-runtime) other than [`tokio`] -
you can implement the `HttpClient` (recommended) or the `Policy` trait yourself.

Similar to [customizing `reqwest` above](#reqwest), you can disable default features for Azure SDK crates. In this example where we do not want a dependency on `reqwest` at all,
we need to import `azure_core` with no default features only to implement `HttpClient` so that your `Cargo.toml` looks something like:

```toml
[dependencies]
azure_core = { version = "1", default-features = false }
azure_identity = { version = "1", default-features = false }
azure_security_keyvault_secrets = { version = "1", default-features = false }
http = "1"
ureq = { version = "3", default-features = false, features = [
    "gzip",
    "native-tls",
] }
```

Then we need to implement `HttpClient` for another HTTP client like [`ureq`](https://docs.rs/ureq):

```rust no_run
use azure_core::{error::{ErrorKind, ResultExt as _}, http::{HttpClient, AsyncRawResponse, Request}};
use ureq::tls::{TlsConfig, TlsProvider};

#[derive(Debug)]
struct Agent(ureq::Agent);

impl Default for Agent {
    fn default() -> Self {
        Self(
            ureq::Agent::config_builder()
                .https_only(true)
                .tls_config(
                    TlsConfig::builder()
                        .provider(TlsProvider::NativeTls)
                        .build(),
                )
                .build()
                .into(),
        )
    }
}

#[async_trait::async_trait]
impl HttpClient for Agent {
    async fn execute_request(&self, request: &Request) -> azure_core::Result<AsyncRawResponse> {
        let request: ::http::request::Request<Vec<u8>> = todo!("convert our request into their request");
        let response = self
            .0
            .run(request)
            .with_context_fn(ErrorKind::Io, || "failed to send request")?;

        Ok(todo!("convert their response into our response"))
    }
}
```

See the [example](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/azure_core/examples/core_ureq_client.rs) for a full sample implementation.

After you've implemented `HttpClient`, you pass it in `ClientOptions` to our client libraries as [shown for `reqwest` above](#reqwest).

### Replacing the async runtime

Internally, the Azure SDK uses either the [`tokio`] async runtime (with the `tokio` feature), or it implements asynchronous functionality using functions in the `std` namespace.

If your application uses a different asynchronous runtime, you can replace the asynchronous runtime used for internal functions by providing your own implementation of the `azure_core::async_runtime::AsyncRuntime` trait.

You provide the implementation by calling the `set_async_runtime()` API:

```rust no_run
use azure_core::{async_runtime::{
     set_async_runtime, AsyncRuntime, TaskFuture, SpawnedTask},
     time::Duration};
use std::sync::Arc;
use futures::FutureExt;

struct CustomRuntime;

impl AsyncRuntime for CustomRuntime {
    fn spawn(&self, f: TaskFuture) -> SpawnedTask {
        unimplemented!("custom spawn not implemented");
    }
    fn sleep(&self, duration: Duration) -> TaskFuture {
        unimplemented!("custom sleep not implemented");
    }
    fn yield_now(&self) -> TaskFuture {
        unimplemented!("custom yield not implemented");
    }
}

set_async_runtime(Arc::new(CustomRuntime)).expect("failed to set async runtime");
```

There can only be one async runtime set in a given process, so attempts to set the async runtime multiple times will fail.

## Troubleshooting

### Logging

To help protected end users from accidental Personally-Identifiable Information (PII) from leaking into logs or traces, models' default implementation of `core::fmt::Debug` formats as non-exhaustive structure tuple e.g.,

Using the `client` we instantiated above:

```rust ignore safe-debug
let secret = client.get_secret("secret-name", None)
    .await?
    .into_model()?;

println!("{secret:#?}");
```

By default this will print:

```text
Secret { .. }
```

Though not recommended for production, you can enable normal `core::fmt::Debug` formatting complete with field names and values by enabling the `debug` feature of `azure_core` e.g.,

```sh
cargo add azure_core -F debug
```

### Known issues

#### Hang when invoking multiple HTTP operations using the default HTTP transport

Some customers have reported hangs when using the default `reqwest` HTTP transport.
The issue is tracked in [this GitHub issue](https://github.com/hyperium/hyper/issues/2312).
The recommended workaround is to disable connection pooling in a custom `reqwest` transport.

If you are encountering this issue, you can construct an `HttpClient` which disables HTTP connection pooling
and set that as the transport in any `ClientOptions` used to configure your Azure SDK clients:

```rust ignore reqwest-hang
use std::sync::Arc;
use azure_core::http::{ClientOptions, Transport};
use azure_security_keyvault_secrets::SecretClientOptions;

let client = Arc::new(
    ::reqwest::ClientBuilder::new()
        .pool_max_idle_per_host(0)
        .build()?,
);

let options = SecretClientOptions {
    client_options: ClientOptions {
        transport: Some(Transport::new(client.clone())),
        ..Default::default()
    },
    ..Default::default()
};
```

Note that implementing this workaround can result in a significant performance slowdown depending on your scenario.

## Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to these libraries.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit <https://opensource.microsoft.com/cla/>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

[API Reference Documentation]: https://docs.rs/azure_core
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[guidelines]: https://azure.github.io/azure-sdk/rust_introduction.html
[Package (crates.io)]: https://crates.io/crates/azure_core
[`reqwest`]: https://docs.rs/reqwest
[`tokio`]: https://docs.rs/tokio
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/core/azure_core/src

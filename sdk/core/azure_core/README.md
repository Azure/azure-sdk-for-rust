# Azure Core shared client library for Rust

`azure_core` provides shared primitives, abstractions, and helpers for modern Rust Azure SDK client libraries.
These libraries follow the [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html)
and can be easily identified by package and namespaces names starting with 'azure_', e.g. `azure_identity`.

`azure_core` allows client libraries to expose common functionality in a consistent fashion,
so that once you learn how to use these APIs in one client library, you will know how to use them in other client libraries.

[Source code] | [Package (crates.io)] | [API Reference Documentation]

## Getting started

Typically, you will not need to install `azure_core`;
it will be installed for you when you install one of the client libraries using it.
In case you want to install it explicitly (to implement your own client library, for example),
you can find the crates.io package [here][Package (crates.io)].

## Key concepts

The main shared concepts of `azure_core` (and so Azure SDK libraries using `azure_core`) include:

- Configuring service clients, e.g. configuring retries, logging (`ClientOptions`).
- Accessing HTTP response details (`Response`, `Response<T>`).
- Calling long-running operations (`Operation<T>`).
- Paging and asynchronous streams (`AsyncPageable<T>`).
- Exceptions for reporting errors from service requests in a consistent fashion. (`RequestFailedException`).
- Customizing requests (`RequestContext`).
- Abstractions for representing Azure SDK credentials. (`TokenCredentials`).

Below, you will find sections explaining these shared concepts in more detail.

### Thread safety

We guarantee that all client instance methods are thread-safe and independent of each other ([guideline](https://azure.github.io/azure-sdk/rust_introduction.html#rust-service-methods-thread-safety)). This ensures that the recommendation of reusing client instances is always safe, even across threads.

### Additional concepts
<!-- CLIENT COMMON BAR -->
[Client options](#configuring-service-clients-using-clientoptions) |
[Accessing the response](#accessing-http-response-details-using-responset) |
[Long-running operations](#consuming-long-running-operations-using-operationt) |
<!-- [Handling failures](#reporting-errors-requestfailedexception) |
[Diagnostics](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/azure_core/samples/Diagnostics.md) |
[Mocking](https://learn.microsoft.com/rust/azure/sdk/unit-testing-mocking) |
[Client lifetime](https://devblogs.microsoft.com/azure-sdk/lifetime-management-and-thread-safety-guarantees-of-azure-sdk-rust-clients/) -->
<!-- CLIENT COMMON BAR -->

## Examples

**NOTE:** Samples in this file apply only to packages that follow [Azure SDK Design Guidelines](https://azure.github.io/azure-sdk/rust_introduction.html). Names of such packages usually start with `azure_`.

### Configuring Service Clients Using `ClientOptions`

Azure SDK client libraries typically expose one or more _service client_ types that
are the main starting points for calling corresponding Azure services.
You can easily find these client types as their names end with the word _Client_.
For example, `SecretClient` can be used to call the Key Vault service and interact with secrets,
and `KeyClient` can be used to access Key Vault service cryptographic keys.

These client types can be instantiated by calling a simple `new` or `builder` function that takes various configuration options.These options are passed as a parameter that extends `ClientOptions` class exposed by `azure_core`.
Various service specific options are usually added to its subclasses, but a set of SDK-wide options are
available directly on `ClientOptions`.

```rust no_run
use azure_core::ClientOptions;
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{SecretClient, SecretClientOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = SecretClientOptions {
        api_version: "7.0".to_string(),
        client_options: ClientOptions::default(),
    };

    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://your-key-vault-name.vault.azure.net/",
        credential.clone(),
        Some(options),
    )?;

    Ok(())
}
```

### Accessing HTTP Response Details Using `Response<T>`

_Service clients_ have methods that can be used to call Azure services. We refer to these client methods as _service methods_.
_Service methods_ return a shared `azure_core` type `Response<T>` (in rare cases its non-generic sibling, a raw `Response`).
This type provides access to both the deserialized result of the service call, and to the details of the HTTP response returned from the server.

```rust no_run
use azure_core::Response;
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{models::SecretBundle, SecretClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a client
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://your-key-vault-name.vault.azure.net/",
        credential.clone(),
        None,
    )?;

    // call a service method, which returns Response<T>
    let response: Response<SecretBundle> = client.get_secret("SecretName", "", None).await?;

    // Response<T> has two main accessors.
    // The into_body() function for accessing the deserialized result of the call
    let secret = response.into_body().await?;

        // get response again because it was moved in above statement
    let response: Response<SecretBundle> = client.get_secret("SecretName", "", None).await?;

    // .. and the deconstruct() method for accessing all the details of the HTTP response
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

More on response types in [response samples](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/azure_core/samples/Response.md).

### Setting up console logging

To create an Azure SDK log listener that outputs messages to console use `AzureEventSourceListener::create_console_logger` method.

```rust no_run
// Setup a listener to monitor logged events.
let listener = AzureEventSourceListener::create_console_logger();
```

More on logging in [diagnostics samples](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/azure_core/samples/Diagnostics.md).

### Reporting Errors `RequestFailedException`

When a service call fails `Azure.RequestFailedException` would get thrown. The exception type provides a status property with an HTTP status code and an error_code property with a service-specific error code.

```rust no_run
match client.get_secret("NonexistentSecret").await {
    Ok(secret) => println!("Secret: {:?}", secret),
    Err(e) => match e {
        RequestFailedException { status: 404, error_code, .. } => {
            // handle not found error
            println!("ErrorCode: {}", error_code);
        },
        _ => println!("An error occurred: {:?}", e),
    },
}
```

More on handling responses in [response samples](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/azure_core/samples/Response.md).

### Consuming Service Methods Returning `AsyncPageable<T>`

If a service call returns multiple values in pages, it would return `Pageable<T>/AsyncPageable<T>` as a result. You can iterate over `AsyncPageable` directly or in pages.

```rust no_run
// call a service method, which returns AsyncPageable<T>
let all_secret_properties = client.get_properties_of_secrets().await?;

while let Some(secret_properties) = all_secret_properties.next().await {
    println!("{}", secret_properties.name);
}
```

For more information on paged responses, see [Pagination with the Azure SDK for Rust](https://learn.microsoft.com/rust/azure/sdk/pagination).

### Consuming Long-Running Operations Using `Operation<T>`

Some operations take long time to complete and require polling for their status. Methods starting long-running operations return `*Operation<T>` types.

The `wait_for_completion` method is an easy way to wait for operation completion and get the resulting value.

```rust no_run
// create a client
let client = SecretClient::new("http://example.com", DefaultAzureCredential::default(), Default::default());

// Start the operation
let mut operation = client.start_delete_secret("SecretName").await?;

// Wait for the operation to complete
let response = operation.wait_for_completion().await?;
let value = response.value;

println!("{}", value.name);
println!("{:?}", value.scheduled_purge_date);
```

More on long-running operations in [long-running operation samples](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/azure_core/samples/LongRunningOperations.md).

### Customizing Requests Using `RequestContext`

Besides general configuration of _service clients_ through `ClientOptions`, it is possible to customize the requests sent by _service clients_
using protocol methods or convenience APIs that expose `RequestContext` as a parameter.

```rust no_run
let mut context = RequestContext::new();
context.add_classifier(404, false);

let response = client.get_pet("pet1", context).await?;
```

More on request customization in [RequestContext samples](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/azure_core/samples/RequestContext.md).

### Mocking

One of the most important cross-cutting features of our new client libraries using `azure_core` is that they are designed for mocking.
Mocking is enabled by:

- providing a protected parameterless constructor on client types.
- making service methods virtual.
- providing APIs for constructing model types returned from virtual service methods. To find these factory methods look for types with the _ModelFactory_ suffix, e.g. `SecretModelFactory`.

For example, the ConfigurationClient.get method can be mocked (with [Mockall](https://github.com/asomers/mockall)) as follows:

```rust no_run
// Create a mock response
let mock_response = MockResponse::new();

// Create a mock value
let mock_value = SecretModelFactory::key_vault_secret(
    SecretModelFactory::secret_properties("http://example.com".parse().unwrap())
);

// Create a client mock
let mut mock = MockSecretClient::new();

// Setup client method
mock.expect_get_secret()
    .with(eq("Name"), eq(None), eq(Default::default()))
    .returning(move |_, _, _| Ok(Response::from_value(mock_value.clone(), mock_response.clone())));

// Use the client mock
let client = mock;
let secret = client.get_secret("Name").await?;
```

More on mocking in [Unit testing and mocking with the Azure SDK for Rust](https://learn.microsoft.com/rust/azure/sdk/unit-testing-mocking).

## Distributed tracing with OpenTelemetry

Azure SDKs are instrumented for distributed tracing using [OpenTelemetry](https://opentelemetry.io/). Distributed tracing allows to follow request through multiple services, record how long network or logical call take along with structured properties describing such operations.

More on diagnostics in [diagnostics samples](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/azure_core/samples/Diagnostics.md).

To setup distributed tracing for your application follow your observability vendor documentation. If you use Azure Monitor, follow the [Start Monitoring Application](https://learn.microsoft.com/azure/azure-monitor/app/opentelemetry-enable?tabs=aspnetcore) guide.

## Troubleshooting

Three main ways of troubleshooting failures are [inspecting exceptions](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/azure_core/samples/Response.md#handling-exceptions), enabling [logging](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/azure_core/samples/Diagnostics.md#Logging), and [distributed tracing](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/azure_core/samples/Diagnostics.md#Distributed-tracing)

## Next steps

Explore and install [available Azure SDK libraries](https://azure.github.io/azure-sdk/releases/latest/rust.html).

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

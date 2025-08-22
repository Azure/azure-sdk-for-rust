# TypeSpec Client Runtime

This is the runtime for [TypeSpec](https://typespec.io)-generated clients.

## Features

* `debug`: enables extra information for developers e.g., emitting all fields in `std::fmt::Debug` implementation.
* `decimal`: enables support for `rust_decimal::Decimal` type.
* `derive`: enable derive macros e.g., `SafeDebug`.
* `http` (default): enables HTTP support.
* `json` (default): enables JSON support.
* `reqwest` (default): enables and sets `reqwest` as the default `HttpClient`. Enables `reqwest`'s `native-tls` feature.
* `reqwest_deflate` (default): enables deflate compression for `reqwest`.
* `reqwest_gzip` (default): enables gzip compression for `reqwest`.
* `reqwest_rustls`: enables `reqwest`'s `rustls-tls-native-roots-no-provider` feature,
  which requires manually configuring a cryptography provider since `ring` is a banned dependency.
* `tokio`: enables and sets `tokio` as the default async runtime.
* `xml`: enables XML support.

## Troubleshooting

### Logging

To help protected end users from accidental Personally-Identifiable Information (PII) from leaking into logs or traces, models' default implementation of `core::fmt::Debug` formats as non-exhaustive structure tuple e.g.,

```rust ignore
#[macro_use]
use typespec_client_core::fmt::SafeDebug;

#[derive(SafeDebug)]
struct Person {
    name: String,
}

let p = Person {
    name: "Any One".into(),
};
println!("{p:?}");
```

By default this will print:

```text
Person { .. }
```

Though not recommended for production, you can enable normal `core::fmt::Debug` formatting complete with field names and values by enabling the `debug` feature of `typespec_client_core` e.g.,

```sh
cargo add typespec_client_core -F debug
```

### Known issues

#### Hang when invoking multiple HTTP operations using the default HTTP transport

Some customers have reported hangs when using the default `reqwest` HTTP transport.
The issue is tracked in [this GitHub issue](https://github.com/hyperium/hyper/issues/2312).
The recommended workaround is to disable connection pooling in a custom `reqwest` transport.

If you are encountering this issue, you can construct an `HttpClient` which disables HTTP connection pooling
and set that as the transport in any `ClientOptions` used to configure your clients:

```rust no_run
use std::sync::Arc;
use typespec_client_core::http::{HttpClient, ClientOptions, TransportOptions};

let client = Arc::new(
    ::reqwest::ClientBuilder::new()
        // Note that reqwest does not support `pool_max_idle_per_host` on WASM.
        .pool_max_idle_per_host(0)
        .build()
        .expect("failed to build `reqwest` client"),
);

let options = ClientOptions {
    transport: Some(TransportOptions::new(client.clone())),
    ..Default::default()
};
```

Note that implementing this workaround can result in a significant performance slowdown depending on your scenario.

## Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to these libraries.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit <https://opensource.microsoft.com/cla/>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

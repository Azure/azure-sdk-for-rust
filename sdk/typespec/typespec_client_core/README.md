# TypeSpec Client Runtime

This is the runtime for [TypeSpec](https://typespec.io)-generated clients.

## Features

* `debug`: enables extra information for developers e.g., emitting all fields in `std::fmt::Debug` implementation.
* `decimal`: enables support for `rust_decimal::Decimal` type.
* `derive`: enable derive macros e.g., `Model` and `SafeDebug`.
* `http` (default): enables HTTP support.
* `json` (default): enables JSON support.
* `reqwest` (default): enables and sets `reqwest` as the default `HttpClient`. Enables `reqwest`'s `native-tls` feature.
* `reqwest_deflate` (default): enables deflate compression for `reqwest`.
* `reqwest_gzip` (default): enables gzip compression for `reqwest`.
* `reqwest_rustls`: enables `reqwest`'s `rustls-tls-native-roots-no-provider` feature,
  which requires manually configuring a cryptography provider since `ring` is a banned dependency.
* `tokio`: enables and sets `tokio` as the default async runtime.
* `xml`: enables XML support.

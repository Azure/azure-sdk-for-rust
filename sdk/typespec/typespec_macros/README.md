# TypeSpec Macros crate for Rust

The TypeSpec Macros crate provides procedural macros for [TypeSpec](https://typespec.io)-generated client libraries. These macros simplify the implementation of common patterns required when working with TypeSpec-generated code.

[Source code] | [Package (crates.io)] | [API reference documentation] | [TypeSpec documentation]

## Getting started

> **Note:** This crate is primarily intended for internal use by the `typespec_client_core` crate. Direct usage is discouraged unless you are contributing to or debugging the TypeSpec ecosystem.
### Install the package

> **Disclaimer:** The following instructions are intended for advanced users or contributors who need to work directly with the `typespec_macros` crate. Most users should depend on the `typespec_client_core` crate instead.

```bash
cargo add typespec_macros
```

## Key concepts

This crate provides the following derive macros:

- `Model`: A derive macro that implements the `Model` trait, allowing a type to be deserialized from an HTTP response body.
- `SafeDebug`: A derive macro that implements debug formatting in a way that avoids leaking personally identifiable information (PII).

### The Model derive macro

The `Model` derive macro is used to implement the `Model` trait for structs that represent data returned from an API. It handles the deserialization of HTTP response bodies into your model types.

### The SafeDebug derive macro

The `SafeDebug` derive macro creates a `Debug` implementation that respects the `#[sensitive]` attribute on struct fields. Fields marked with this attribute will not have their values printed in debug output, protecting potentially sensitive information.

## Examples

### Using the Model derive macro

```rust
use typespec_macros::Model;
use serde::Deserialize;

#[derive(Model, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}
```

### Using the SafeDebug derive macro

```rust
use typespec_macros::SafeDebug;

#[derive(SafeDebug)]
pub struct Credentials {
    pub username: String,
    #[sensitive]
    pub password: String,
}

// When debug printed, the password will be hidden:
// Credentials { username: "user123", password: "***" }
```

## Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to this library.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit <https://opensource.microsoft.com/cla/>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/typespec/typespec_macros/src
[Package (crates.io)]: https://crates.io/crates/typespec_macros
[API reference documentation]: https://docs.rs/typespec_macros
[TypeSpec documentation]: https://typespec.io/
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/

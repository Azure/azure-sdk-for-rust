# TypeSpec Macros crate for Rust

The TypeSpec Macros crate provides procedural macros for [TypeSpec](https://typespec.io)-generated client libraries. These macros simplify the implementation of common patterns required when working with TypeSpec-generated code.

[Source code] | [Package (crates.io)] | [API reference documentation] | [TypeSpec documentation]

## Getting started

> **Note:** This crate should not be used directly. Users should depend on the `typespec_client_core` crate instead.

## Key concepts

This crate provides the following derive macros:

-   `SafeDebug`: A derive macro that implements debug formatting in a way that avoids leaking personally identifiable information (PII).

### The SafeDebug derive macro

The `SafeDebug` derive macro creates a `Debug` implementation that respects the `#[safe(true)]` and `#[safe(false)]` attribute on struct fields. By default fields are considered sensitive and will not have their values printed in debug output, protecting potentially sensitive information.

## Examples

### Using the SafeDebug derive macro

```rust
use typespec_macros::SafeDebug;

#[derive(SafeDebug)]
struct Credentials {
  #[safe(true)]
  pub username: String,
  pub password: String,
};
let credentials: Credentials = Credentials {
  username: "admin".into(),
  password: "hunter2".into(),
};
println!("{credentials:?}");
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

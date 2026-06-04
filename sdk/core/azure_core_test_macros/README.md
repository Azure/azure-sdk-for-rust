# Azure client library test macros

Macros for testing client libraries built on `azure_core`.
Read about the [`azure_core_test`](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/core/azure_core_test) crate
for more information about the generated code and how to write recorded tests.

🚨 WARNING 🚨: This project is not supported for anything other that testing [Azure client libraries for Rust](https://github.com/Azure/azure-sdk-for-rust).
The public API and behavior may change at any time.

## Client methods

To test client methods using our [Test Proxy] or to run against live resources, you can attribute asynchronous tests
using the `#[recorded::test]` attribute:

```rust
use azure_core_test::{recorded, TestContext};

#[recorded::test]
async fn get_secret(ctx: TestContext) -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}
```

The `TestContext` parameter is required unless your test function is attribute as `#[recorded::test(live)]` (live-only).
You can name the parameter whatever you want.
The `TestContext` parameter is used to initialize an HTTP client to play back or record tests
and provides other information to test functions that may be useful.

These tests must also return a `std::result::Result<T, E>`, which can be redefined e.g., `azure_core::Result<T>`.

[Test Proxy]: https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/README.md

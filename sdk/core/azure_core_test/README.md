# Azure client library test utilities

The types and functions in this crate help test client libraries built on `azure_core`.

## Client methods

To test client methods using our [Test Proxy] or run against live resources, you can attribute asynchronous tests
using the `#[recorded::test]` attribute:

```rust
use azure_core::Result;
use azure_core_test::{recorded, TestContext};

#[recorded::test]
async fn get_secret(ctx: TestContext) -> Result<()> {
    todo!()
}
```

The `TestContext` parameter is required unless your test function is attributed as `#[recorded::test(live)]` (live-only).
You can name the parameter whatever you want.
The `TestContext` parameter is used to initialize an HTTP client to play back or record tests
and provides other information to test functions that may be useful.

These tests must also return a `std::result::Result<T, E>`, which can be redefined e.g., `azure_core::Result<T>`.

[Test Proxy]: https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/README.md

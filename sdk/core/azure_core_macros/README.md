# Azure client library test macros

Macros for testing client libraries built on `azure_core`.

## Client methods

To test client methods using our [Test Proxy], you can attribute both synchronous and asynchronous (recommend) tests
using the `#[recorded]` attribute:

```rust
use azure_core_macros::recorded;
use azure_core_test::TestContext;
use azure_core::Result;

#[recorded]
async fn get_secret(ctx: TestContext) -> Result<()> {
    todo!()
}
```

[Test Proxy]: https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/README.md

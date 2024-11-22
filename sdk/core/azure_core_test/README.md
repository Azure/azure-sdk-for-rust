# Azure client library test utilities

The types and functions in this crate help test client libraries built on `azure_core`.

## Client methods

To test client methods using our [Test Proxy], you can attribute both synchronous and asynchronous (recommend) tests
using the `#[recorded]` attribute. See examples in the `azure_core_macros` crate.

[Test Proxy]: https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/README.md

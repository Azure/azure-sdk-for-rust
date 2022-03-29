# azure_svc_applicationinsights crate

The is an [Azure SDK for Rust](https://github.com/Azure/azure-sdk-for-rust) crate that is generated from the Azure REST API specifications listed in:

https://github.com/Azure/azure-rest-api-specs/blob/main/specification/applicationinsights/data-plane/readme.md

The default `Tag` is `v1`.

The following `Tag`s are available:

- `v1` has 10 operations from 1 API versions: `v1`. Use crate feature `v1` to enable. The operations will be in the `v1` module.
# azure_mgmt_support crate

The is an [Azure SDK for Rust](https://github.com/Azure/azure-sdk-for-rust) crate that is generated from the Azure REST API specifications listed in:

https://github.com/Azure/azure-rest-api-specs/blob/main/specification/support/resource-manager/readme.md

The default `Tag` is `package-2020-04`.

The following `Tag`s are available:

- `package-preview-2021-06` has 2 operations from 1 API versions: `2021-06-01-preview`. Use crate feature `package-preview-2021-06` to enable. The operations will be in the `package_preview_2021_06` module.
- `package-2020-04` has 14 operations from 1 API versions: `2020-04-01`. Use crate feature `package-2020-04` to enable. The operations will be in the `package_2020_04` module.
- `package-2019-05-preview` has 14 operations from 1 API versions: `2019-05-01-preview`. Use crate feature `package-2019-05-preview` to enable. The operations will be in the `package_2019_05_preview` module.
# azure_mgmt_attestation crate

The is an [Azure SDK for Rust](https://github.com/Azure/azure-sdk-for-rust) crate that is generated from the Azure REST API specifications listed in:

https://github.com/Azure/azure-rest-api-specs/blob/main/specification/attestation/resource-manager/readme.md

The default `Tag` is `package-2020-10-01`.

The following `Tag`s are available:

- `package-2018-09-01` has 9 operations from 1 API versions: `2018-09-01-preview`. Use crate feature `package-2018-09-01` to enable. The operations will be in the `package_2018_09_01` module.
- `package-2020-10-01` has 13 operations from 1 API versions: `2020-10-01`. Use crate feature `package-2020-10-01` to enable. The operations will be in the `package_2020_10_01` module.
- `package-2021-06-01` has 14 operations from 1 API versions: `2021-06-01-preview`. Use crate feature `package-2021-06-01` to enable. The operations will be in the `package_2021_06_01` module.
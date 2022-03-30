# azure_mgmt_migrate crate

The is an [Azure SDK for Rust](https://github.com/Azure/azure-sdk-for-rust) crate that is generated from the Azure REST API specifications listed in:

https://github.com/Azure/azure-rest-api-specs/blob/main/specification/migrate/resource-manager/readme.md

The default `Tag` is `package-2019-10`.

The following `Tag`s are available:

- `package-2018-02` has 24 operations from 1 API versions: `2018-02-02`. Use crate feature `package-2018-02` to enable. The operations will be in the `package_2018_02` module.
- `package-2019-10` has 46 operations from 1 API versions: `2019-10-01`. Use crate feature `package-2019-10` to enable. The operations will be in the `package_2019_10` module.
- `package-2020-01` has 41 operations from 1 API versions: `2020-01-01`. Use crate feature `package-2020-01` to enable. The operations will be in the `package_2020_01` module.
- `package-2020-05` has 13 operations from 1 API versions: `2020-05-01`. Use crate feature `package-2020-05` to enable. The operations will be in the `package_2020_05` module.
- `package-2020-07` has 57 operations from 1 API versions: `2020-07-07`. Use crate feature `package-2020-07` to enable. The operations will be in the `package_2020_07` module.
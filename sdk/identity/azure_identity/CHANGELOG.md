# Release History

## 0.22.0 (2025-02-18)

### Features Added

- Initial supported release.

### Breaking Changes

- Removed service credentials from `DefaultAzureCredential` ([#2093](https://github.com/Azure/azure-sdk-for-rust/issues/2093))

## 0.20.0 (2024-02)

### Breaking Changes

- [#1532](https://github.com/Azure/azure-sdk-for-rust/pull/1532) add `azure_identity::create_credential()`, `SpecificAzureCredential`, `AppServiceManagedIdentityCredential`, `VirtualMachineManagedIdentityCredential`
  - BREAKING CHANGE: `DefaultAzureCredentialBuilder::build` now returns a `Result`. If fails when it is unable to create at least one source credential.
  - Most credentials may now fail earlier, when they are created, instead of only during `get_token`.
  - `DefaultAzureCredential::default()` has been removed, because creating the credential may fail. Please use `azure_identity::create_default_credential()?` or `azure_identity::create_credential()?` instead.

## 0.18.0 (2023-12)

### Breaking Changes

- Removed AutoRefreshingTokenCredential, instead all token credentials now implement caching

## 0.3.0 (2022-05)

### Breaking Changes

- [#756](https://github.com/Azure/azure-sdk-for-rust/pull/756) Export credentials from azure_identity
  - BREAKING CHANGE: the credential types have moved. For example:
  - use `azure_identity::DefaultAzureCredential` instead of `azure_identity::token_credentials::DefaultAzureCredential`

### Bugs Fixed

- [#751](https://github.com/Azure/azure-sdk-for-rust/pull/751) datetime from azure cli token is in the local timezone
- [#748](https://github.com/Azure/azure-sdk-for-rust/pull/748) adding option to specify client_id for MSI

## 0.2.0 (2022-05)

### Other Changes

- update to azure_core 0.2.1

## 0.1.1 (2022-01)

### Features Added

- Initial publish to [crates.io](https://crates.io/crates/azure_identity)

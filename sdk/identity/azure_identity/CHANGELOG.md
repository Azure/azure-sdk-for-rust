# Release History

## 0.28.0 (Unreleased)

### Features Added

### Breaking Changes

- Replaced `DefaultAzureCredential` with `DeveloperToolsCredential`. This new type is excluded from WASM32 builds because it can't authenticate in a WASM runtime environment; however, neither could `DefaultAzureCredential`, which wasn't properly excluded.

### Bugs Fixed

### Other Changes

## 0.27.0 (2025-08-05)

### Other Changes

- Updated dependencies.

## 0.26.0 (2025-07-15)

### Breaking Changes

- Minimum supported Rust version (MSRV) is now 1.85.
- Converted all `time::Duration` types to `azure_core::time::Duration`
- Replaced `azure_core::process::Executor` with `azure_identity::process::Executor`.
- Renamed `azure_core::date` module to `azure_core::time`

## 0.25.0 (2025-06-10)

### Bugs Fixed

- `AzureCliCredential` didn't invoke `az` within a shell on all platforms

### Other Changes

- `AzureCliCredential::get_token()` always invokes the Azure CLI

## 0.24.0 (2025-05-06)

### Features Added

- `AzureDeveloperCliCredential` authenticates the identity logged in to the [Azure Developer CLI](https://learn.microsoft.com/azure/developer/azure-developer-cli/overview).
- Added the `AzureDeveloperCliCredential` to the `DefaultAzureCredential`.

### Breaking Changes

- Moved `WorkloadIdentityCredential::new` arguments into `WorkloadIdentityCredentialOptions` except `token`, which has been removed (the credential now reads service account tokens only from a file).
- Removed `ClientAssertionCredential::from_env` and `ClientCertificateCredential::from_env`.
- Removed `WorkloadIdentityCredential::from_env`. `::new` now reads the same environment variables except for `AZURE_FEDERATED_TOKEN` (the Workload Identity webhook doesn't set that variable). `WorkloadIdentityCredentialOptions` overrides environment variable values.

## 0.23.0 (2025-04-09)

### Features Added

- Added `AzurePipelinesCredential`.
- `AzureCliCredentialOptions` (new) accepts a `azure_core::process::Executor` to run the Azure CLI asynchronously.
  The `tokio` feature is disabled by default so `std::process::Command` is used; otherwise, if enabled, `tokio::process::Command` is used.
  Callers can also implement the trait themselves to use a different asynchronous runtime.
- Restored `ClientSecretCredential`

### Breaking Changes

- Added `Option<AzureCliCredentialOptions>` to `AzureCliCredential::new`.
- `AzureCliCredential` authenticates only against the first scope passed as a resource to support both v1 and v2 CLI versions.
- `ClientAssertionCredential` constructors moved some parameters to an `Option<ClientAssertionCredentialOptions>` parameter.
- Removed `get_subscription()` and `get_tenant()` from `AzureCliCredential`.
- `WorkloadIdentityCredential` constructors moved some parameters to an `Option<ClientAssertionCredentialOptions>` parameter.
- Removed `clear_cache()` from all credential types
- Removed `old_azure_cli` feature. `AzureCliCredential` now requires a recent version of the Azure CLI (2.54.0 or later).
- Replaced `AppServiceManagedIdentityCredential`, `VirtualMachineManagedIdentityCredential`, and `ImdsId` with `ManagedIdentityCredential` and `UserAssignedId`

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

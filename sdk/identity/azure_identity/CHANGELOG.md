# Release History

## 0.33.0 (Unreleased)

### Features Added

- Added `SecretBytes` to `azure_core::credentials` for securely passing certificate bytes without printing them in `Debug` or `Display` output.

### Breaking Changes

- Support for `wasm32-unknown-unknown` has been removed ([#3377](https://github.com/Azure/azure-sdk-for-rust/issues/3377))
- `ClientCertificateCredential::new()` now takes `SecretBytes` instead of `Secret` for the `certificate` parameter. Pass the raw PKCS12 bytes wrapped in `SecretBytes` instead of a base64-encoded string wrapped in `Secret`.

### Bugs Fixed

### Other Changes

## 0.32.0 (2026-02-11)

### Breaking Changes

- Changed our minimum supported Rust version (MSRV) from 1.85 to 1.88.

### Bugs Fixed

- Removed redundant content and extraneous JSON from `Azure[Developer]CliCredential` error messages

## 0.31.0 (2026-01-21)

### Breaking Changes

- Removed unused `additionally_allowed_tenants` and `disable_instance_discovery` options for `AzureCliCredential` and `ClientAssertionCredential`.
- Changed the type of the `certificate` parameter of `ClientCertificateCredential::new()` from `impl Into<Secret>` to `Secret`.

## 0.30.0 (2025-11-11)

### Features Added

- A `get_token()` error caused by an HTTP response carries that response. See the [troubleshooting guide](https://aka.ms/azsdk/rust/identity/troubleshoot#find-relevant-information-in-errors) for example code showing how to access the response.

### Breaking Changes

- `ClientCertificateCredential::new()`:
  - `client_certificate` parameter is now `certificate`
  - `client_certificate_password` parameter is now `password: Option<azure_core::credentials::Secret>` in `ClientCertificateCredentialOptions`
  - now returns an error when the given certificate can't be parsed
- Removed `ClientCertificateCredentialOptions.send_certificate_chain`. Set environment variable `AZURE_CLIENT_SEND_CERTIFICATE_CHAIN` to "1" or "true" to enable this feature.

### Bugs Fixed

- `ClientCertificateCredential::get_token()` returned an error when given multiple scopes.
- `ManagedIdentityCredential` didn't follow IMDS retry guidance.

## 0.29.0 (2025-10-08)

### Breaking Changes

- `ClientCertificateCredential::new()` takes `Option<ClientCertificateCredentialOptions>` instead of `impl Into<ClientCertificateCredentialOptions>`.
- Credential constructors return an error when given a non-HTTPS authority host.
- Renamed `ClientCertificateCredential::new()` parameter `client_certificate_pass` to `client_certificate_password`.
- Replaced credential-specific `authority_host` options with `azure_core::cloud::CloudConfiguration` configured via `ClientOptions.cloud`.

## 0.28.0 (2025-09-16)

### Features Added

- Credentials retry HTTP requests by default.

### Breaking Changes

- Removed all `ClientCertificateCredentialOptions` methods
- Removed `TokenCredentialOptions`. HTTP client options are now set on `ClientOptions`. Credentials which formerly got an authority host from this type now get it from an `authority_host` field in their own options type.
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

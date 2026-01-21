# Release History

## 0.11.0 (Unreleased)

### Features Added

### Breaking Changes

### Bugs Fixed

### Other Changes

## 0.10.0 (2026-01-20)

### Features Added

- Added `continuation_token` to `PagerOptions` for methods that return a `Pager`.
- Added support for sovereign and private clouds. Clients now discover authentication parameters at runtime by eliciting a challenge from Key Vault by sending their first request without an access token. You may see this request and Key Vault's 401 response in logs, followed by a successful retry.

### Breaking Changes

- Removed `Pager::with_continuation_token()` for methods that return a `Pager`.

## 0.9.0 (2025-11-10)

### Breaking Changes

- `SecretClientListDeletedSecretPropertiesOptions::method_options` is now `PagerOptions`.
- `SecretClientListSecretPropertiesOptions::method_options` is now `PagerOptions`.
- `SecretClientListSecretPropertiesVersionsOptions::method_options` is now `PagerOptions`.
- Renamed `Response<T, F>::into_body(self) -> Result<Response<T>>` to `into_model(self) -> Result<Response<T>>`. `into_body(self)` now returns a `ResponseBody`.

## 0.8.0 (2025-10-07)

### Breaking Changes

- Client methods that return a `Response<T>>` asynchronously buffer the entire model within the internal pipeline, so `into_body()` and other methods on the response are no longer async.

## 0.7.0 (2025-09-16)

### Features Added

- Updated to latest version of fixes for the "7.6" specification after TypeSpec migration.

### Breaking Changes

- Moved secret version parameters to client method options.

## 0.6.0 (2025-08-05)

### Other Changes

- Updated dependencies.

## 0.5.0 (2025-07-10)

### Breaking Changes

- Minimum supported Rust version (MSRV) is now 1.85.

## 0.4.0 (2025-06-10)

### Features Added

- Built on Azure Key Vault service version 7.6.

### Breaking Changes

- `Pager<T>` now asynchronously iterates items of type `T` instead of pages containing items of type `T`. Call `Pager::into_pages()` to get a `PageIterator` to asynchronously iterate over all pages. This affects:
  - `SecretClient::list_deleted_secret_properties`
  - `SecretClient::list_secret_properties`
  - `SecretClient::list_secret_properties_versions`

### Bugs Fixed

### Other Changes

## 0.3.0 (2025-05-06)

## Other Changes

- Updated dependencies.

## 0.2.0 (2025-04-09)

### Breaking Changes

- Changed model fields defined as an `Option<HashMap<K, V>>` to just a `HashMap<K, V>`.
- Changed model fields defined as an `Option<Vec<T>>` to just a `Vec<T>`.
- Renamed `DeletedSecretBundle` to `DeleteSecret`.
- Renamed `DeletedSecretItem` to `DeletedSecretProperties`.
- Renamed `SecretBundle` to `Secret`.
- Renamed `SecretBundleBackup` to `SecretBackup`.
- Renamed `SecretClient::get_secrets` to `list_secret_properties`.
- Renamed `SecretItem` to `SecretProperties`.
- Renamed all parameter types to match the *{Verb}{Noun}* format of the client methods that use them e.g., `set_secret` uses `SetSecretParameters`.

### Bugs Fixed

- `ResourceExt` canonicalizes URL path segments ([#2177](https://github.com/Azure/azure-sdk-for-rust/issues/2177))

## 0.1.0 (2025-02-18)

### Features Added

- Initial public release.
- Built on Azure Key Vault service version 7.6-preview.2.

# Release History

## 0.4.0 (2025-06-10)

### Features Added

- Built on Azure Key Vault service version 7.6.

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

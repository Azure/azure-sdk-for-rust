# Release History

## 0.4.0 (Unreleased)

### Features Added

### Breaking Changes

### Bugs Fixed

### Other Changes

## 0.3.0 (2025-05-06)

### Other Changes

- Updated dependencies.

## 0.2.0 (2025-04-09)

### Breaking Changes

- Changed model fields defined as an `Option<HashMap<K, V>>` to just a `HashMap<K, V>`.
- Changed model fields defined as an `Option<Vec<T>>` to just a `Vec<T>`.
- Removed the "JsonWebKey" prefix from all types e.g., `JsonWebKeyCurveName` is now `CurveName`.
- Renamed `DeletedKeyBundle` to `DeleteKey`.
- Renamed `DeletedKeyItem` to `DeletedKeyProperties`.
- Renamed `KeyClient::get_keys` to `list_keys`.
- Renamed `KeyBundle` to `Key`.
- Renamed `KeyBundleBackup` to `KeyBackup`.
- Renamed `KeyClient::get_keys` to `list_key_properties`.
- Renamed `KeyItem` to `KeyProperties`.
- Renamed all parameter types to match the *{Verb}{Noun}* format of the client methods that use them e.g., `create_key` uses `CreateKeyParameters`.

### Bugs Fixed

- `ResourceExt` canonicalizes URL path segments ([#2177](https://github.com/Azure/azure-sdk-for-rust/issues/2177))

## 0.1.0 (2025-02-18)

### Features Added

- Initial public release.
- Built on Azure Key Vault service version 7.6-preview.2.

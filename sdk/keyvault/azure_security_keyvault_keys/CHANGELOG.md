# Release History

## 0.2.0 (Unreleased)

### Features Added

### Breaking Changes

- Changed model fields defined as an `Option<HashMap<K, V>>` to just a `HashMap<K, V>`.
- Changed model fields defined as an `Option<Vec<T>>` to just a `Vec<T>`.
- Renamed `KeyClient::get_keys` to `list_keys`.

### Bugs Fixed

### Other Changes

## 0.1.0 (2025-02-18)

### Features Added

- Initial public release.
- Built on Azure Key Vault service version 7.6-preview.2.

# Release History

## 0.9.0 (Unreleased)

### Features Added

### Breaking Changes

### Bugs Fixed

### Other Changes

## 0.8.0 (2025-10-07)

### Breaking Changes

- Client methods that return a `Response<T>>` asynchronously buffer the entire model within the internal pipeline, so `into_body()` and other methods on the response are no longer async.
- Renamed `KeyType::EC` to `KeyType::Ec` to align with guidelines.
- Renamed `KeyType::RSA` to `KeyType::Rsa` to align with guidelines.
- Renamed `EncryptionAlgorithm::RSA1_5` to `EncryptionAlgorithm::Rsa1_5` to align with guidelines.
- Renamed `EncryptionAlgorithm::RsaOAEP256` to `EncryptionAlgorithm::RsaOaep256` to align with guidelines.
- Renamed several variants in `SignatureAlgorithm` to align with guidelines e.g., `ES256K` to `Es256K`.

## 0.7.0 (2025-09-16)

### Features Added

- Updated to latest version of fixes for the "7.6" specification after TypeSpec migration.

### Breaking Changes

- Moved key version parameters to client method options.

## 0.6.0 (2025-08-05)

### Other Changes

- Updated dependencies.

## 0.5.0 (2025-07-10)

### Breaking Changes

- Minimum supported Rust version (MSRV) is now 1.85.

## 0.4.0 (2025-06-10)

### Features Added

- Built on Azure Key Vault service version 7.6.
- Added `EncryptionAlgorithm::CkmAesKeyWrap` and `CkmAesKeyWrapPad`.
- Added `JsonWebKeySignatureAlgorithm::Hs256`, `Hs384`, and `Hs512`.
- Added `KeyAttestation` returned by `KeyAttributes::attestation` field.
- Added `KeyClient::get_key_attestation` client method.

### Breaking Changes

- `Pager<T>` now asynchronously iterates items of type `T` instead of pages containing items of type `T`. Call `Pager::into_pages()` to get a `PageIterator` to asynchronously iterate over all pages. This affects:
  - `KeyClient::list_deleted_key_properties`
  - `KeyClient::list_key_properties`
  - `KeyClient::list_key_properties_versions`

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

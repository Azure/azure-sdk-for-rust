# Release History

## 0.3.0 (Unreleased)

### Features Added

### Breaking Changes

- `Pager<T>` now asynchronously iterates items of type `T` instead of pages containing items of type `T`. Call `Pager::into_pages()` to get a `PageIterator` to asynchronously iterate over all pages. This affects:
  - `CertificateClient::list_deleted_certificate_properties`
  - `CertificateClient::list_certificate_properties`
  - `CertificateClient::list_certificate_properties_versions`
  - `CertificateClient::list_issuer_properties`

### Bugs Fixed

### Other Changes

## 0.2.0 (2025-05-06)

### Other Changes

- Updated dependencies.

## 0.1.0 (2025-04-09)

### Features Added

- Initial public release.
- Built on Azure Key Vault service version 7.6-preview.2.

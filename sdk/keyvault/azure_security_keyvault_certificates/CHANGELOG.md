# Release History

## 0.6.0 (Unreleased)

### Features Added

- Updated to latest version of fixes for the "7.6" specification after TypeSpec migration.

### Breaking Changes

- Moved certificate version parameters to client method options.

### Bugs Fixed

### Other Changes

## 0.5.0 (2025-08-05)

### Breaking Changes

- Removed the `CertificateClientExt` trait for `CertificateClient`. The `begin_create_certificate` and `resume_certificate_operation` methods are implemented for `CertificateClient` with the same method signatures.

### Other Changes

- Updated dependencies.

## 0.4.0 (2025-07-10)

### Features Added

- Added the `CertificateClientExt` trait for `CertificateClient` with `begin_create_certificate` and `resume_certificate_operation` methods that return the new `Poller<CertificateOperation>`.
  These extension methods are likely temporary until `Poller` support is added to the TypeSpec emitter and we update `CertificateClient::create_certificate` and `get_certificate_operation` to return a `Poller<CertificateOperation>`.

### Breaking Changes

- Minimum supported Rust version (MSRV) is now 1.85.

## 0.3.0 (2025-06-10)

### Features Added

- Built on Azure Key Vault service version 7.6.
- Added `preserve_cert_order` to several models to preserve the order of signing certificates when importing or merging certificates.

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

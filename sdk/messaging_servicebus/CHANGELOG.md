# CHANGELOG

## 0.15.1

- Updated `fe2o3-amqp-ws` dependency to 0.4.0, which includes an upstream fix for [CVE-2023-43669](https://github.com/snapview/tungstenite-rs/pull/379).

### Breaking Changes

## 0.15.0 - Sep. 15, 2023

- Updated `azure_core` dependency to 0.15.0
- Fixed clippy warnings

## 0.14.0 - Sep. 7, 2023

- Unified error type for most public functions to `azure_core::Error`
- Changed versioning to follow that of `azure_core`
- Fixed bug with `TokenCredential` support

## 0.1.2 - Feb. 7, 2023

- Added support for `wasm32-unknown-unknown` target

## 0.1.1 - Jan. 10, 2023

- Fixed typo in readme

## 0.1.0 - Jan. 10, 2023

- Initial release

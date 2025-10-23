# Release History

## 0.9.0 (Unreleased)

### Features Added

### Breaking Changes

- All AMQP APIs now return an `azure_core_amqp::AmqpError` instead of an `azure_core::Error`.
- Several `azure_core_amqp::error::AmqpErrorKind` enumerations have been removed because they are no longer needed.
- Modified several errors which previously used `azure_core::Error::with_message` to use `azure_core_amqp::AmqpError::with_message`, changing their underlying type.

### Bugs Fixed

### Other Changes

- Added  `azure_core_amqp::error::AmqpErrorKind::AzureCore`, and `azure_core_amqp::error::AmqpErrorKind::SimpleMessage` to describe Azure Core originated errors, and messages which just have a string value respectively.
- Added `azure_core_amqp::Error::with_message` to enable simple error returns with a text message.

## 0.8.1 (2025-10-06)

### Bugs Fixed

- Fix feature documentation ([#3118](https://github.com/Azure/azure-sdk-for-rust/issues/3118))

## 0.8.0 (2025-10-03)

### Breaking Changes

- Removed non-idiomatic accessor functions from `AmqpDescribed`, `AmqpSessionOptions`, `AmqpDescribedError`, `AmqpMessage`, and `AmqpTarget`
- Renamed the "cplusplus" feature to "ffi" because "ffi" is more idiomatic to Rust.

### Other Changes

`AmqpMessage` now implements the `SafeDebug` trait, redacting the `body` and `application_properties` fields because they are likely to contain PII. To view the full contents of the message, enable the `debug` feature in the `azure_core` package.

## 0.7.0 (2025-09-11)

### Breaking Changes

- Distinguish remote disconnect and remote closed errors by origin

## 0.6.0 (2025-08-01)

### Other Changes

- Updated dependencies.

## 0.5.0 (2025-07-10)

### Breaking Changes

- Minimum supported Rust version (MSRV) is now 1.85.
- Converted all `time::Duration` references to be `azure_core::time::Duration`.

## 0.4.0 (2025-06-06)

### Breaking Changes

- `AmqpClaimsBasedSecurity` now takes ownership of the associated session rather than simply referencing the associated session. This means that all CBS authentication operations should be performed on dedicated AmqpSession objects.

- `AmqpOrderedMap::iter` now iterates over references to key and value, not clones of the key and value, thus eliminating unnecessary clones.

### Other Changes

- Use the `SafeDebug` macro to hide potential PII from trace logs. The SafeDebug macro currently applies to the `AmqpApplicationProperties` and `AmqpMessageBody` types.

## 0.3.1 (2025-05-08)

### Other Changes

- Added `From<&AmqpValue>` converters for fundamental types.

## 0.3.0 (2025-05-02)

### Other Changes

- Updated dependencies.
- Converted AMQP traits to use `async_trait` rather than attempting to implement the `async_trait` functionality manually.
- Restructured and refactored AMQP errors to make them easier to interpret.

## 0.2.0 (2025-04-08)

### Features Added

- Added the ability to compare an `AmqpAnnotationKey` with a string and string slice.

### Breaking Changes

- APIs which used to return `Option<String>`, and `Option<Vec<T>>` now return `Option<&str>`, and `Option<&[T]>`.
- APIs which take ownership of string parameters now take a `String` parameter instead of a `&str` parameter.

## 0.1.0 (2025-02-18)

### Features Added

- Initial supported release.

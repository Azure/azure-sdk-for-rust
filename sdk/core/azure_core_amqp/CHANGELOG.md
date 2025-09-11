# Release History

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

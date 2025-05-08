# Release History

## 0.3.1 (2025-05-08)

### Features Added

### Breaking Changes

### Bugs Fixed

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

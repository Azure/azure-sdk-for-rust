# Release History

## 0.3.0 (2025-05-02)

### Other Changes

- Updated dependencies.

## 0.2.0 (2025-04-08)

### Features Added

- Added the ability to compare an `AmqpAnnotationKey` with a string and string slice.

### Breaking Changes

- APIs which used to return `Option<String>`, and `Option<Vec<T>>` now return `Option<&str>`, and `Option<&[T]>`.
- APIs which take ownership of string parameters now take a `String` parameter instead of a `&str` parameter.

## 0.1.0 (2025-02-18)

### Features Added

- Initial supported release.

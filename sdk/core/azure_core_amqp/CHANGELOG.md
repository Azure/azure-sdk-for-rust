# Release History

## 0.2.0 (Unreleased)

### Features Added

- Added the ability to compare an `AmqpAnnotationKey` with a string and string slice.

### Breaking Changes

- APIs which used to return `Option<String>`, and `Option<Vec<T>>` now return `Option<&str>`, and `Option<&[T]>`.
- APIs which take ownership of string parameters now take a `String` parameter instead of a `&str` parameter.

### Bugs Fixed

### Other Changes

## 0.1.0 (2025-02-18)

### Features Added

- Initial supported release.

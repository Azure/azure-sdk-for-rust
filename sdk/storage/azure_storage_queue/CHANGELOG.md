# Release History

## 0.1.0 (Unreleased)

### Features Added

* Initial supported release.

### Breaking Changes

- Client methods that return a `Response<T>>` asynchronously buffer the entire model within the internal pipeline, so `into_body()` and other methods on the response are no longer async.

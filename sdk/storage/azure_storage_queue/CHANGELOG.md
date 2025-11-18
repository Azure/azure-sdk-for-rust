# Release History

## 0.3.0 (Unreleased)

### Features Added

### Breaking Changes

### Bugs Fixed

### Other Changes

## 0.2.0 (2025-11-11)

### Breaking Changes

- Changed ` QueueServiceClientListQueuesOptions::method_options` from `ClientMethodOptions` to `PagerOptions`
- Renamed `Response<T, F>::into_body(self) -> Result<Response<T>>` to `into_model(self) -> Result<Response<T>>`. `into_body(self)` now returns a `ResponseBody`.

## 0.1.0 (2025-10-15)

### Features Added

- Initial supported release.

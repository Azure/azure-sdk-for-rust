# Change log

## 0.1.2-beta

- Changed visibility of `IntoAzureCoreError` to `pub(crate)` and restricted its impl to only foreign
  error types. All other error type natively implements `Into<azure_core::error::Error>`

## 0.1.2-alpha

- Fixed a bug where `EventStream` is not `Send` because `ClosingBoxedFuture` misses `Send` in its
  trait bounds
- Changed visibility of struct `EventStream` to public
- Changed visibility of trait `IntoAzureCoreError` to public
- Updated dependency `azure_core` to `"0.13"`
- Updated dependency `time`'s version to `"<=0.3.23"`, which is the latest version that supports
  rust version 1.65

## 0.1.1

- Fixed wrong crate name in the example

## 0.1.0

- Initial release

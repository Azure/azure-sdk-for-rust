# Release History

## Unreleased

### Features Added

- Added `cosmos_read_many_request_init` and `cosmos_read_many_open_submit` with versioned request records for item or complete-partition selections and optional parameterized filters, returning paged results through the existing native cursor APIs without durable checkpoint/resume support. ([#5385](https://github.com/Azure/azure-sdk-for-rust/pull/5385))

### Breaking Changes

### Bugs Fixed

### Other Changes

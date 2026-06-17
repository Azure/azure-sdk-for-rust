# Release History

## 0.2.0 (Unreleased)

### Features Added

- Added an `#[options(env_only)]` struct-level mode to `CosmosOptions` that generates only the `from_env()`/`from_env_vars()` constructors (no View, Builder, or `Default`), allowing an existing builder-style type to double as its own environment-variable source. ([#4562](https://github.com/Azure/azure-sdk-for-rust/pull/4562))

### Breaking Changes

### Bugs Fixed

### Other Changes

## 0.1.0 (2026-04-09)

### Features Added

- Initial release of `azure_data_cosmos_macros` (procedural macros for the Cosmos DB SDK hierarchical configuration model). ([#3868](https://github.com/Azure/azure-sdk-for-rust/pull/3868))


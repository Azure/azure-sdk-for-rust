# Release History

## 0.2.0 (2026-06-19)

### Features Added

- Added an `overridable` field-level flag (`#[option(env = "...", overridable)]`) that recognizes a `{ENV}_OVERRIDE` kill-switch environment variable, generates `from_env_override()`/`from_env_override_vars()` parsing, and adds a top-priority `env_override` layer to the generated View (constructed via `new_with_override`). ([#4562](https://github.com/Azure/azure-sdk-for-rust/pull/4562))
- Added an `#[options(env_only)]` struct-level mode to `CosmosOptions` that generates only the `from_env()`/`from_env_vars()` constructors (no View, Builder, or `Default`), allowing an existing builder-style type to double as its own environment-variable source. ([#4562](https://github.com/Azure/azure-sdk-for-rust/pull/4562))
- Added a `parser` field-level attribute (`#[option(env = "...", parser = path::to::fn)]`) to `CosmosOptions` that parses an environment variable with a custom `fn(&str) -> Option<T>` instead of `FromStr`, supporting field types without a suitable `FromStr` (such as a `Duration` read from a millisecond count). A `None` result is logged and ignored, matching the lenient built-in parsers. ([#4562](https://github.com/Azure/azure-sdk-for-rust/pull/4562))

## 0.1.0 (2026-04-09)

### Features Added

- Initial release of `azure_data_cosmos_macros` (procedural macros for the Cosmos DB SDK hierarchical configuration model). ([#3868](https://github.com/Azure/azure-sdk-for-rust/pull/3868))


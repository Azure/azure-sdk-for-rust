# Release History

> **Note:** This crate is an internal performance testing tool. It is **not published** to crates.io
> and is **not supported** for production use. It may change or be removed at any time without notice.

## 0.1.0 (Unreleased)

### Features Added

- Initial implementation of the Cosmos DB performance testing CLI tool.
- Point read, create item, single-partition query, and upsert operations with concurrent execution.
- Key-based and AAD (Entra ID) authentication via `ManagedIdentityCredential`.
- Configurable concurrency, run duration, seed count, and reporting interval.
- Automatic container creation if it does not exist, with configurable throughput.
- Preferred and excluded region configuration.
- Periodic latency summary reporting (count, min, max, mean, p50, p90, p99, errors).
- Process-level CPU and memory metrics in each report.
- Added `--user-agent-suffix` (env `AZURE_COSMOS_USER_AGENT_SUFFIX`, default `rust-perf`) for tagging perf-harness traffic in Cosmos server-side telemetry. Applied to both the primary and results clients; empty string disables. ([#4465](https://github.com/Azure/azure-sdk-for-rust/pull/4465))


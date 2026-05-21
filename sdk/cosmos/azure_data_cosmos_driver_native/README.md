# Azure Cosmos DB Driver — Native C Bindings (skeleton)

This crate exposes the [`azure_data_cosmos_driver`](../azure_data_cosmos_driver) crate via a C ABI so that
non-Rust SDKs (Java, .NET, Python, …) and C/C++ applications can reuse the
driver's transport, routing, retry, partition-key, and diagnostics machinery.

> **Status:** Phase 0–2 skeleton. See
> [`../azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md`](../azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md)
> for the full design and the phased implementation plan.

## What works today

- Crate compiles and links as `cdylib` / `staticlib` / `rlib` named `azurecosmosdriver`.
- `cosmos_version()` returns the crate version as a C string.
- `cosmos_string_free` / `cosmos_bytes_free` for releasing SDK-owned buffers.
- Error types (`cosmos_error_t`, `cosmos_error_code_t`) and the `Error` / `CosmosError` round-trip.
- `cosmos_call_context_t` + `cosmos_runtime_t` lifecycle (Phase 2).
- Tracing init via `cosmos_enable_tracing()` (feature `tracing`).

## What's stubbed (TODO)

- `cbindgen` integration in `build.rs` (Phase 0 finalize).
- `CMakeLists.txt` + `c_tests/` (Phase 0 finalize).
- Account / driver / partition-key / operation / response / diagnostics handles (Phases 3–7).
- Pager (Phase 8), batch + patch helpers (Phase 9), advanced surface (Phase 10).

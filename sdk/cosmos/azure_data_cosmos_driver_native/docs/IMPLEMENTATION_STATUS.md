# `azure_data_cosmos_driver_native` — Implementation Status

> **Snapshot date:** May 21, 2026
> **Branch:** `users/kundadebdatta/4372_implement_driver_native_crate`
> **Spec:** [`../azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md`](../azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md)

## Commits on this branch

| SHA (short) | Title |
|---|---|
| `ccf43caae` (prior baseline) | Cosmos: scaffold `azure_data_cosmos_driver_native` (Phase 0–3 skeleton) |
| `3fece35e1` | Cosmos native: cbindgen, partition keys, refs, operations, execute (Phases 0-finalize, 4, 5-partial, 6-partial) |

## What is done

### ✅ Phase 0 — Scaffolding (finalized)

- Crate `azure_data_cosmos_driver_native` in workspace (`cdylib + staticlib + rlib`, lib name `azurecosmosdriver`).
- `build.rs` runs `cbindgen 0.29.2` against the crate + `parse_deps = true` for `azure_data_cosmos_driver`.
- `include/azurecosmosdriver.h` (~23 KB) generated on every build, checked in.
- 50 `cosmos_*` C functions exported with Doxygen-style docs.
- `cosmos_version()`, `cosmos_enable_tracing()`, `BUILD_IDENTIFIER` static, `cosmos_string_free`, `cosmos_bytes_free` working.

### ✅ Phase 1 — Error + context primitives

- `CosmosErrorCode` enum with the spec's 30xx FFI codes and new 40xx driver-wrapper codes (`RuntimeAlreadyInitialized`, `DriverNotInitialized`, `InvalidAccountReference`, `InvalidPartitionKey`, `OperationConsumed`, `ResponseConsumed`).
- `Error` (rich, internal) ↔ `CosmosError` (`#[repr(C)]`, external) round-trip with optional detail string.
- `CallContext` + `CallContextOptions`, `cosmos_call_context_create/free`, `run_sync`/`run_async`/`*_with_output` helpers, `context!` macro, `IntoRaw` trait.

### ✅ Phase 2 — Runtime

- Tokio-backed `RuntimeContext` owning the executor **and** an `Arc<CosmosDriverRuntime>`.
- `cosmos_runtime_create(options, out_error)` and `cosmos_runtime_free(rt)`.
- `RuntimeOptions { worker_threads }` honored.

### ✅ Phase 3 — Account + driver + references (partial)

- `cosmos_account_ref_with_master_key` → opaque `cosmos_account_ref` (master-key auth only so far).
- `cosmos_driver_get_or_create` → opaque `cosmos_driver` (wraps `Arc<CosmosDriver>`, no driver-options handle yet).
- `cosmos_database_ref_create` (local, no network).
- `cosmos_container_ref_resolve` (async, calls `driver.resolve_container`).
- `cosmos_item_ref_create` (local, takes container + partition key + item id).
- All have matching `_free`.

### ✅ Phase 4 — Partition keys (complete)

- `cosmos_partition_key_builder_new` + `append_{string,number,bool,null,undefined}` + `build` (consumes builder).
- `cosmos_partition_key_from_string` convenience for the common single-string case.
- Returns `InvalidPartitionKey` on empty build. Hierarchical keys supported by chaining appends.

### ✅ Phase 5 — Operation construction (initial subset)

Implemented factories (all `cosmos_operation_*`):
- **Database:** `read_database`, `delete_database`
- **Container:** `read_container`, `delete_container`
- **Item:** `create_item`, `read_item`, `replace_item`, `upsert_item`, `delete_item`

Implemented mutators:
- `cosmos_operation_with_body(op, CosmosBytesView)`
- `cosmos_operation_with_partition_key(op, *const cosmos_partition_key)`

Operations are move-only: `cosmos_driver_execute` consumes them; double-execute returns `OperationConsumed`. `cosmos_operation_free` is always safe (handle holds `Option<CosmosOperation>`).

### ✅ Phase 6 — Execute + response (initial subset)

- `cosmos_driver_execute` runs `OperationOptions::default()` against `driver.execute_operation`. **Non-success HTTP statuses are NOT mapped to error codes** — they surface via `cosmos_response_status_code` per spec §6.
- Response accessors:
  - `cosmos_response_status_code` → `uint16_t`
  - `cosmos_response_request_charge` → `double`
  - `cosmos_response_activity_id` → borrowed `const char*` (out-param) or null
  - `cosmos_response_etag` → borrowed `const char*` or null
  - `cosmos_response_continuation_token` → borrowed `const char*` or null
  - `cosmos_response_body` → zero-copy `CosmosBytesView` (only for single-part `ResponseBody::Bytes`)
  - `cosmos_response_into_body` → owned `CosmosBytes` (frees response)
- `cosmos_response_free`.

### ✅ Tests — rlib smoke suite

8 tests in `tests/smoke.rs`, all green via `cargo test -p azure_data_cosmos_driver_native`:

```
test version_matches_cargo_pkg_version ... ok
test runtime_create_and_free ... ok
test account_ref_rejects_invalid_endpoint ... ok
test account_ref_with_valid_endpoint_succeeds ... ok
test partition_key_builder_single_string_round_trip ... ok
test partition_key_builder_empty_build_is_rejected ... ok
test partition_key_from_string_convenience_works ... ok
test bytes_buffer_round_trip ... ok
```

> The user made manual edits to `tests/smoke.rs` after the commit — re-check the file before editing.

## What is NOT done yet (TODO queue)

### 🟡 Phase 3 finish — driver options

- No `cosmos_driver_options_*` builder yet. `cosmos_driver_get_or_create` takes `_options: *const c_void` and ignores it.
- Need: opaque `cosmos_driver_options_builder_t` mirroring `DriverOptionsBuilder` (preferred regions, excluded regions, read consistency, content-response-on-write, allow-emulator-invalid-certs, …).
- Need: `cosmos_runtime_builder_*` mirroring `CosmosDriverRuntimeBuilder` (workload id, correlation id, user-agent suffix, connection pool, default operation options).
- Need: `cosmos_account_ref_with_resource_token`, and a `cosmos_account_ref_from_connection_string` convenience (also flagged as a spec gap).

### 🟡 Phase 5 finish — remaining operation surface

- Factories still missing: `create_database`, `read_all_databases`, `query_databases`, `create_container`, `read_all_containers`, `query_containers`, `replace_container`, `read_container_by_name`, `read_container_by_rid`, `patch_item`, `read_all_items`, `read_all_items_cross_partition`, `query_items`, `query_items_cross_partition`, `batch`.
- Mutators still missing: `with_request_header`, `with_session_token`, `with_activity_id`, `with_precondition_if_match`, `with_precondition_if_none_match`, `with_patch_max_attempts`.
- Operation-options builder (`cosmos_operation_options_*`): consistency, session token, throughput control group, priority level, end-to-end timeout, max item count, excluded regions, content-response-on-write.
- `cosmos_driver_execute` currently passes `OperationOptions::default()` and ignores the options pointer — wire it once the builder lands.

### 🟡 Phase 6 finish — response polish

- `cosmos_response_iter_headers` (visitor-based) not implemented.
- `cosmos_response_session_token` accessor not implemented (only ETag/continuation/activity-id are).
- `cosmos_response_body` for multi-part `ResponseBody::Items` returns the default (empty) view — needs design decision (concat? error? new pager surface?).

### ⏳ Phase 7 — Diagnostics

- `cosmos_diagnostics_*` accessors (RU charge total, total elapsed, retry count, regions-contacted visitor, JSON snapshot via `cosmos_diagnostics_to_json`).
- Currently only `cosmos_diagnostics_free` exists.

### ⏳ Phase 8 — Pager

- `cosmos_pager_t` for read-feeds and queries (`cosmos_driver_execute_pager`, `cosmos_pager_next`, `cosmos_pager_continuation_token`, `cosmos_pager_free`).
- Depends on Phase 5 query/read-feed factories.

### ⏳ Phase 9 — Patch + transactional batch

- Patch is partially expressible already (`patch_item` factory + `with_body`) but missing `with_patch_max_attempts` mutator.
- TransactionalBatch handle (`cosmos_batch_t`) not started.

### ⏳ Phase 10 — Advanced surface

- `fault_injection` feature integration.
- AAD `cosmos_account_ref_with_token_provider(callback, user_data)`.
- `__internal_in_memory_emulator` hookup for emulator-free C tests.
- Tracing init promotion to Phase 0/1.

### ⏳ C-side build & tests

- No `CMakeLists.txt` yet (spec §5).
- No `c_tests/` directory yet.
- No `azurecosmosdriver.pc.in` for pkg-config on Linux/macOS.
- No `cmake/DiscoverTests.cmake`.

## Files in the crate (current tree)

```
sdk/cosmos/azure_data_cosmos_driver_native/
├── Cargo.toml                  cdylib+staticlib+rlib, cbindgen build-dep
├── README.md                   phase status pointer
├── build.rs                    BUILD_IDENTIFIER + cbindgen
├── include/
│   └── azurecosmosdriver.h     ~23 KB, auto-generated, checked in
├── src/
│   ├── lib.rs                  module wiring, cosmos_version, tracing
│   ├── string.rs               c_str! macro, parse_cstr, cosmos_string_free
│   ├── bytes.rs                CosmosBytesView, CosmosBytes, cosmos_bytes_free
│   ├── error.rs                CosmosErrorCode, Error, CosmosError
│   ├── context.rs              CallContext, run_sync/async, context! macro
│   ├── runtime/
│   │   ├── mod.rs              cosmos_runtime_create/free + RuntimeOptions
│   │   └── tokio.rs            Tokio executor + Arc<CosmosDriverRuntime>
│   ├── handles/
│   │   ├── mod.rs
│   │   ├── account.rs          ✅ master_key only
│   │   ├── driver.rs           ✅ get_or_create + execute
│   │   ├── references.rs       ✅ database, container (async resolve), item
│   │   ├── partition_key.rs    ✅ full builder
│   │   ├── operation.rs        ✅ item CRUD + db/container read/delete + 2 mutators
│   │   ├── response.rs         ✅ status, RU, activity_id, etag, continuation, body
│   │   └── diagnostics.rs      ⏳ stub (only _free)
│   └── options/
│       ├── mod.rs
│       ├── driver_options.rs   ⏳ stub
│       ├── runtime_options.rs  ⏳ stub
│       └── operation_options.rs ⏳ stub
└── tests/
    └── smoke.rs                8 rlib tests, all green
```

## How to verify the current state

```pwsh
# Build the crate + regenerate header
cargo build -p azure_data_cosmos_driver_native

# Run smoke tests
cargo test -p azure_data_cosmos_driver_native --tests

# Inspect the generated C surface
Select-String -Path sdk/cosmos/azure_data_cosmos_driver_native/include/azurecosmosdriver.h `
              -Pattern 'cosmos_[a-z_]+\(' -AllMatches |
  ForEach-Object { $_.Matches.Value.TrimEnd('(') } | Sort-Object -Unique
```

## Open design questions (parking lot)

1. Should `cosmos_runtime_t` cap to exactly one instance per process? (Spec §9 Q1)
2. `cosmos_response_iter_headers` visitor: borrowed pointers (proposed) vs. copies?
3. Continuation-token format: `const char*` (current) vs. `cosmos_bytes_view_t` (future binary tokens)?
4. C++ RAII wrapper header `azurecosmosdriver.hpp`?
5. `cosmos_response_body` behavior for `ResponseBody::Items` — concat to single buffer, return error, or require pager?
6. `azure_data_cosmos::ConnectionString` parsing — duplicate, promote to driver, or punt to host SDK?

## Next session — likely starting points

- **Option A (depth):** Phase 7 diagnostics — gives language SDKs real telemetry hooks.
- **Option B (breadth):** Finish Phase 5 (queries, read-feeds, all mutators) so the API is feature-complete for CRUD + query.
- **Option C (validation):** CMake + `c_tests/` so we have end-to-end C-side coverage against the emulator before broadening the API.

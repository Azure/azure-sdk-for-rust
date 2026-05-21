# `azure_data_cosmos_driver_native` — C Bindings Specification

> **Status:** Draft / Proposal
> **Owners:** Cosmos DB Rust SDK team
> **Target crate:** `sdk/cosmos/azure_data_cosmos_driver_native`
> **Wraps:** [`azure_data_cosmos_driver`](../../azure_data_cosmos_driver) (Layer 1 — driver crate)
> **Supersedes:** `azure_data_cosmos_native` (removed in PR [#4103](https://github.com/Azure/azure-sdk-for-rust/pull/4103), commit `ccf43caae`), which wrapped the high-level `azure_data_cosmos` SDK.

---

## 1. Motivation and Context

### 1.1 Why a new wrapper

The original `azure_data_cosmos_native` crate (commits `de5bf3ba8` → `ccf43caae`) provided C bindings on top of the **typed SDK** (`azure_data_cosmos`). That wrapper:

- Forced `serde` serialization through the FFI layer (it called `read_item::<()>`, `create_item(..., RawValue)`, collected pagers into `Vec` and re-serialized to JSON via `serde_json::to_string`).
- Hid driver-level concepts (diagnostics, RU charge, activity id, region routing, throughput control, session tokens) that other-language SDKs *must* expose to their users.
- Created an upside-down dependency for non-Rust SDKs: a Java/.NET/Python SDK consuming the C ABI would receive responses that had already been parsed by Rust's `serde`, then re-serialized to a string, only to be re-parsed by the host language.

The `azure_data_cosmos_driver` crate was explicitly designed (see [`ARCHITECTURE.md`](../ARCHITECTURE.md) "Schema-Agnostic Data Plane") to be the reuse point for **all** non-Rust language SDKs. Wrapping the driver — not the typed SDK — is the correct boundary.

### 1.2 Goals

| # | Goal |
|---|------|
| G1 | Expose every primitive a language SDK needs to build a fully-featured Cosmos DB client: account, runtime, driver, operations, partition keys, options, responses, diagnostics. |
| G2 | Stay **schema-agnostic**: bodies are `const uint8_t*`/`size_t` in, `const uint8_t*`/`size_t` out. No JSON parsing inside the wrapper. |
| G3 | Map cleanly to the driver's Rust API. A C function should correspond to a single driver method or a small, mechanical builder step. |
| G4 | Be **ABI-stable enough** for `corrosion` / `cmake` consumers; breaking changes only on documented driver minor bumps. |
| G5 | Be **runtime-agnostic at the ABI** (Tokio is an implementation detail behind `cosmos_runtime_create`) so a future runtime swap doesn't break C consumers. |
| G6 | Provide first-class **diagnostics** access (request charge, activity id, status, headers, regions contacted, retry attempts). |

### 1.3 Non-goals

- **Item serialization.** Callers pass raw bytes; they handle their own JSON / Cosmos binary encoding.
- **Query plan execution in C.** The driver owns the query engine; the wrapper exposes only the pager-style result iteration once the driver lands query support.
- **Typed `ContainerClient` / `DatabaseClient` C handles** like the old wrapper had. The new model is `cosmos_driver_t` + `cosmos_operation_t` factories scoped by `*_reference_t` handles.
- **A "convenience" layer.** That belongs in each language's own SDK on top of these bindings.

---

## 2. Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│  Consumer (Java / .NET / Python / C / C++ language SDK)     │
├─────────────────────────────────────────────────────────────┤
│  azurecosmosdriver.h               (cbindgen-generated)     │
│  libazurecosmosdriver.{so,dylib,dll,.a}                     │
├─────────────────────────────────────────────────────────────┤
│  azure_data_cosmos_driver_native  (THIS CRATE)              │
│    • #[no_mangle] extern "C" fns                            │
│    • CallContext + RuntimeContext glue                      │
│    • Boxed driver handles                                   │
│    • Bytes-in / bytes-out shims                             │
├─────────────────────────────────────────────────────────────┤
│  azure_data_cosmos_driver         (Layer 1)                 │
│    CosmosDriverRuntime, CosmosDriver, CosmosOperation,      │
│    CosmosResponse, DiagnosticsContext, PartitionKey, …      │
├─────────────────────────────────────────────────────────────┤
│  azure_core / reqwest / tokio                               │
└─────────────────────────────────────────────────────────────┘
```

### 2.1 Crate layout

```
sdk/cosmos/azure_data_cosmos_driver_native/
├── Cargo.toml                # crate-type = ["cdylib", "staticlib"]
├── CMakeLists.txt            # corrosion_import_crate + C test harness
├── README.md
├── build.rs                  # cbindgen + BUILD_IDENTIFIER
├── azurecosmosdriver.pc.in
├── cmake/
│   └── DiscoverTests.cmake
├── include/
│   └── azurecosmosdriver.h   # cbindgen output, checked in
├── src/
│   ├── lib.rs                # crate root, version, tracing init
│   ├── context.rs            # CallContext, IntoRaw, run_sync/run_async
│   ├── error.rs              # CosmosError, error code mapping
│   ├── string.rs             # c_str! macro, parse_cstr, cosmos_string_free
│   ├── bytes.rs              # cosmos_bytes_free, ByteBuf marshalling
│   ├── runtime/
│   │   ├── mod.rs            # RuntimeContext FFI surface
│   │   └── tokio.rs          # Tokio-backed RuntimeContext
│   ├── handles/
│   │   ├── mod.rs
│   │   ├── account.rs        # AccountReference handle + builder
│   │   ├── driver.rs         # CosmosDriver handle (from runtime.get_or_create_driver)
│   │   ├── partition_key.rs  # PartitionKey builder + handle
│   │   ├── operation.rs      # CosmosOperation factories + mutators
│   │   ├── response.rs       # CosmosResponse + body/headers/status accessors
│   │   └── diagnostics.rs    # DiagnosticsContext accessors
│   └── options/
│       ├── mod.rs
│       ├── driver_options.rs # DriverOptions / DriverOptionsBuilder
│       ├── runtime_options.rs# CosmosDriverRuntimeBuilder mirror
│       └── operation_options.rs  # OperationOptions builder
└── c_tests/
    ├── test_common.h
    ├── version.c
    ├── runtime_lifecycle.c
    ├── driver_init.c
    ├── partition_key.c
    ├── item_crud.c
    ├── diagnostics.c
    └── error_handling.c
```

### 2.2 Naming conventions

| Rust type | C type | C function prefix |
|---|---|---|
| `RuntimeContext` (wrapper) | `cosmos_runtime_t` | `cosmos_runtime_*` |
| `CosmosDriver` | `cosmos_driver_t` | `cosmos_driver_*` |
| `AccountReference` | `cosmos_account_ref_t` | `cosmos_account_ref_*` |
| `DatabaseReference` | `cosmos_database_ref_t` | `cosmos_database_ref_*` |
| `ContainerReference` | `cosmos_container_ref_t` | `cosmos_container_ref_*` |
| `PartitionKey` | `cosmos_partition_key_t` | `cosmos_partition_key_*` |
| `CosmosOperation` | `cosmos_operation_t` | `cosmos_operation_*` |
| `CosmosResponse` | `cosmos_response_t` | `cosmos_response_*` |
| `DiagnosticsContext` | `cosmos_diagnostics_t` | `cosmos_diagnostics_*` |
| `DriverOptions` / `OperationOptions` / `RuntimeOptions` | `cosmos_*_options_t` | `cosmos_*_options_*` |
| `CallContext` | `cosmos_call_context_t` | `cosmos_call_context_*` |
| `CosmosError` | `cosmos_error_t` | `cosmos_error_*` |
| `CosmosErrorCode` | `cosmos_error_code_t` | enum variants `COSMOS_ERROR_CODE_*` |

All exported symbols start with `cosmos_`. cbindgen `rename_variants = QualifiedScreamingSnakeCase` is reused from the previous wrapper.

---

## 3. Core FFI Patterns

These are inherited (largely intact) from the original wrapper, because they worked well at the FFI boundary. The driver wrapper extends them.

### 3.1 `CallContext` + `RuntimeContext`

```c
typedef struct cosmos_runtime cosmos_runtime_t;
typedef struct cosmos_call_context {
    const cosmos_runtime_t *runtime;
    bool include_error_details;
    cosmos_error_t error;
} cosmos_call_context_t;
```

- A `RuntimeContext` owns the async runtime (Tokio by default) **plus a strong reference to a shared `CosmosDriverRuntime`** (see §4.1). It is reference-counted internally; one process typically creates exactly one.
- A `CallContext` is a thin POD struct the caller may stack-allocate. It carries the runtime pointer and receives the most recent error. Reusable across calls but **not** thread-safe; one per caller-thread.

### 3.2 Function signature template

Every fallible API follows:

```c
cosmos_error_code_t cosmos_<noun>_<verb>(
    cosmos_call_context_t *ctx,         // required
    /* required handle(s) */,
    /* required scalars */,
    const cosmos_<noun>_<verb>_options_t *options,  // nullable
    /* out parameters: out_handle, out_bytes, out_len, … */
);
```

- Return value is **always** a status code; outputs go through `out_*` pointers. (Identical to the old wrapper.)
- Allocated outputs (handles, byte buffers, strings) require an explicit `cosmos_*_free` to release.
- `options == NULL` is always valid and means "use driver defaults".

### 3.3 Bytes marshalling (new)

Because the driver is schema-agnostic, request/response bodies are raw bytes, not C strings:

```c
// Caller-owned input: caller keeps memory live for the duration of the call.
typedef struct cosmos_bytes_view {
    const uint8_t *data;
    size_t len;
} cosmos_bytes_view_t;

// SDK-owned output: must be freed via cosmos_bytes_free.
typedef struct cosmos_bytes {
    const uint8_t *data;
    size_t len;
    void *_handle;   // opaque, points to a Box<Vec<u8>> on the Rust side
} cosmos_bytes_t;

void cosmos_bytes_free(cosmos_bytes_t bytes);
```

Rationale:

- Bodies may legitimately contain `0x00` bytes (Cosmos binary encoding), so NUL-terminated `const char*` cannot represent them.
- `cosmos_bytes_t` carries an opaque `_handle` so the Rust side can free the original `Vec<u8>` via `Box::from_raw` without juggling separate alloc/dealloc routines.

### 3.4 Handle ownership rules

| Handle | Created by | Freed by | Cloneable? |
|---|---|---|---|
| `cosmos_runtime_t*` | `cosmos_runtime_create` | `cosmos_runtime_free` | No (use one per process) |
| `cosmos_driver_t*` | `cosmos_driver_get_or_create` | `cosmos_driver_free` | Internally `Arc`; FFI handle is a single owner |
| `cosmos_account_ref_t*` | `cosmos_account_ref_builder_*` | `cosmos_account_ref_free` | Cheap clone via `cosmos_account_ref_clone` |
| `cosmos_database_ref_t*` / `cosmos_container_ref_t*` | `cosmos_*_ref_create` from parent | matching `_free` | Cheap clone |
| `cosmos_partition_key_t*` | `cosmos_partition_key_builder_build` | `cosmos_partition_key_free` | Cheap clone |
| `cosmos_operation_t*` | `cosmos_operation_*` factory | `cosmos_operation_free` (or consumed by `execute`) | No (move semantics) |
| `cosmos_response_t*` | `cosmos_driver_execute` | `cosmos_response_free` | No |
| `cosmos_diagnostics_t*` | `cosmos_response_diagnostics` | `cosmos_diagnostics_free` (drops `Arc`) | Internally `Arc` |

`cosmos_driver_t` is **the** unit of cardinality. Each call to `cosmos_driver_get_or_create` for the same account returns the same underlying driver instance (the runtime caches them). The FFI handle, however, is a distinct `Box<Arc<CosmosDriver>>` — freeing it only drops one `Arc` strong count.

### 3.5 Error model

`cosmos_error_code_t` keeps the existing layout (success = 0, FFI errors in the 30xx range, HTTP-mapped codes, Cosmos-specific codes). New codes added for the driver:

| Code | Variant | Meaning |
|---|---|---|
| 4001 | `RuntimeAlreadyInitialized` | A second `cosmos_runtime_create` was rejected when single-runtime mode is enforced. |
| 4002 | `DriverNotInitialized` | Operation issued before `initialize()` completed (should not happen via `get_or_create`). |
| 4003 | `InvalidAccountReference` | Account endpoint/credential could not be parsed. |
| 4004 | `InvalidPartitionKey` | `PartitionKey` builder produced an empty / inconsistent key. |
| 4005 | `OperationConsumed` | `cosmos_operation_*` mutator called after `cosmos_driver_execute`. |
| 4006 | `ResponseConsumed` | `cosmos_response_into_*` called twice. |

The status / sub-status / activity id from a server error are exposed via the `cosmos_response_t` returned by a *successful* `execute` call (Cosmos non-success status codes are not modeled as `azure_core::Error` here — see §6).

---

## 4. Module-by-Module Surface

### 4.1 Runtime (`src/runtime/`)

```c
typedef struct cosmos_runtime_options {
    // Worker thread count for the internal Tokio runtime. 0 = number of CPUs.
    uint32_t worker_threads;
    // Thread name prefix for runtime worker threads (NUL-terminated, may be NULL).
    const char *thread_name_prefix;
} cosmos_runtime_options_t;

cosmos_runtime_t *cosmos_runtime_create(
    const cosmos_runtime_options_t *options,
    cosmos_error_t *out_error);

void cosmos_runtime_free(cosmos_runtime_t *runtime);
```

Internally:

```rust
pub struct RuntimeContext {
    tokio: tokio::runtime::Runtime,
    driver_runtime: Arc<CosmosDriverRuntime>,
}
```

The shared `CosmosDriverRuntime` is built via `CosmosDriverRuntimeBuilder::new()` and is the cache key for `cosmos_driver_get_or_create`.

### 4.2 Driver options (`src/options/driver_options.rs`)

`DriverOptions` in the driver crate is a builder-heavy type. The wrapper exposes an opaque builder handle, not a `#[repr(C)]` mirror — that avoids re-emitting every driver setting in two places:

```c
typedef struct cosmos_driver_options_builder cosmos_driver_options_builder_t;
typedef struct cosmos_driver_options cosmos_driver_options_t;

cosmos_driver_options_builder_t *cosmos_driver_options_builder_new();
void cosmos_driver_options_builder_free(cosmos_driver_options_builder_t *b);

cosmos_error_code_t cosmos_driver_options_builder_with_preferred_regions(
    cosmos_driver_options_builder_t *b,
    const char *const *regions, size_t regions_len);

cosmos_error_code_t cosmos_driver_options_builder_with_read_consistency(
    cosmos_driver_options_builder_t *b,
    cosmos_read_consistency_t consistency);

/* … one setter per supported DriverOptions field … */

cosmos_driver_options_t *cosmos_driver_options_builder_build(
    cosmos_driver_options_builder_t *b);  // consumes builder
void cosmos_driver_options_free(cosmos_driver_options_t *opts);
```

Setters mirror, 1:1, the Rust `DriverOptionsBuilder::with_*` methods. New driver settings can be added without ABI-breaking changes because each is its own `cosmos_*_with_*` function.

Same pattern for:

- `cosmos_runtime_builder_*` (mirrors `CosmosDriverRuntimeBuilder`)
- `cosmos_operation_options_*` (mirrors `OperationOptions`)
- `cosmos_diagnostics_options_*` (mirrors `DiagnosticsOptions`)

### 4.3 Account / Database / Container references (`src/handles/account.rs` etc.)

```c
// AccountReference — wraps azure_data_cosmos_driver::models::AccountReference
cosmos_error_code_t cosmos_account_ref_with_master_key(
    const char *endpoint,
    const char *key,
    cosmos_account_ref_t **out_account,
    cosmos_error_t *out_error);

cosmos_error_code_t cosmos_account_ref_with_resource_token(
    const char *endpoint,
    const char *token,
    cosmos_account_ref_t **out_account,
    cosmos_error_t *out_error);

// Future: cosmos_account_ref_with_aad_token_callback(...)

void cosmos_account_ref_free(cosmos_account_ref_t *account);

// DatabaseReference / ContainerReference
cosmos_error_code_t cosmos_database_ref_create(
    const cosmos_account_ref_t *account,
    const char *database_id,
    cosmos_database_ref_t **out_database);

cosmos_error_code_t cosmos_container_ref_create(
    const cosmos_database_ref_t *database,
    const char *container_id,
    cosmos_container_ref_t **out_container);
```

These are pure value-types that do not touch the network — they correspond to the driver's reference types in `models/resource_reference.rs`.

### 4.4 Driver instance (`src/handles/driver.rs`)

```c
cosmos_error_code_t cosmos_driver_get_or_create(
    cosmos_call_context_t *ctx,
    const cosmos_account_ref_t *account,
    const cosmos_driver_options_t *options,  // nullable
    cosmos_driver_t **out_driver);

void cosmos_driver_free(cosmos_driver_t *driver);

// Convenience access to the cached account metadata.
cosmos_error_code_t cosmos_driver_initialize(
    cosmos_call_context_t *ctx,
    const cosmos_driver_t *driver);
```

`get_or_create` calls `CosmosDriverRuntime::get_or_create_driver(account, options).await` which already awaits `initialize()` (see `runtime.rs:364`). The explicit `initialize` is exposed for parity but is normally unnecessary.

### 4.5 Partition keys (`src/handles/partition_key.rs`)

`PartitionKey` is a `Vec<PartitionKeyValue>` (`models/partition_key.rs:294`), supporting hierarchical keys and the five JSON types. The wrapper exposes a tiny builder:

```c
typedef struct cosmos_partition_key_builder cosmos_partition_key_builder_t;

cosmos_partition_key_builder_t *cosmos_partition_key_builder_new();
void cosmos_partition_key_builder_free(cosmos_partition_key_builder_t *b);

cosmos_error_code_t cosmos_partition_key_builder_append_string(
    cosmos_partition_key_builder_t *b, const char *value);
cosmos_error_code_t cosmos_partition_key_builder_append_number(
    cosmos_partition_key_builder_t *b, double value);
cosmos_error_code_t cosmos_partition_key_builder_append_bool(
    cosmos_partition_key_builder_t *b, bool value);
cosmos_error_code_t cosmos_partition_key_builder_append_null(
    cosmos_partition_key_builder_t *b);
cosmos_error_code_t cosmos_partition_key_builder_append_none(
    cosmos_partition_key_builder_t *b);

cosmos_error_code_t cosmos_partition_key_builder_build(
    cosmos_partition_key_builder_t *b,     // consumed on success
    cosmos_partition_key_t **out_pk);

void cosmos_partition_key_free(cosmos_partition_key_t *pk);
```

A convenience helper for the common 1-value case:

```c
cosmos_error_code_t cosmos_partition_key_from_string(
    const char *value, cosmos_partition_key_t **out_pk);
```

### 4.6 Operations (`src/handles/operation.rs`)

This is the heart of the new wrapper. Operations mirror the `CosmosOperation` constructors in `models/cosmos_operation.rs`. Every constructor produces a heap-owned operation that the caller mutates with `with_*` shims and finally passes to `cosmos_driver_execute`.

Naming convention: `cosmos_operation_<rust_constructor_name>`.

```c
/* Account-scope */
cosmos_operation_t *cosmos_operation_create_database(
    const cosmos_account_ref_t *account);
cosmos_operation_t *cosmos_operation_read_all_databases(
    const cosmos_account_ref_t *account);
cosmos_operation_t *cosmos_operation_query_databases(
    const cosmos_account_ref_t *account);

/* Database-scope */
cosmos_operation_t *cosmos_operation_read_database(const cosmos_database_ref_t *db);
cosmos_operation_t *cosmos_operation_delete_database(const cosmos_database_ref_t *db);
cosmos_operation_t *cosmos_operation_create_container(const cosmos_database_ref_t *db);
cosmos_operation_t *cosmos_operation_read_all_containers(const cosmos_database_ref_t *db);
cosmos_operation_t *cosmos_operation_query_containers(const cosmos_database_ref_t *db);

/* Container-scope */
cosmos_operation_t *cosmos_operation_read_container(const cosmos_container_ref_t *c);
cosmos_operation_t *cosmos_operation_replace_container(const cosmos_container_ref_t *c);
cosmos_operation_t *cosmos_operation_delete_container(const cosmos_container_ref_t *c);
cosmos_operation_t *cosmos_operation_read_all_items(
    const cosmos_container_ref_t *c, const cosmos_partition_key_t *pk);

/* Item-scope (ItemReference is constructed inline from container + id) */
cosmos_operation_t *cosmos_operation_create_item(
    const cosmos_container_ref_t *c, const char *item_id);
cosmos_operation_t *cosmos_operation_read_item(
    const cosmos_container_ref_t *c, const char *item_id);
cosmos_operation_t *cosmos_operation_upsert_item(
    const cosmos_container_ref_t *c, const char *item_id);
cosmos_operation_t *cosmos_operation_replace_item(
    const cosmos_container_ref_t *c, const char *item_id);
cosmos_operation_t *cosmos_operation_delete_item(
    const cosmos_container_ref_t *c, const char *item_id);
cosmos_operation_t *cosmos_operation_patch_item(
    const cosmos_container_ref_t *c, const char *item_id);

/* Mutators (mirror CosmosOperation::with_*) */
cosmos_error_code_t cosmos_operation_with_partition_key(
    cosmos_operation_t *op, const cosmos_partition_key_t *pk);
cosmos_error_code_t cosmos_operation_with_body(
    cosmos_operation_t *op, cosmos_bytes_view_t body);
cosmos_error_code_t cosmos_operation_with_request_header(
    cosmos_operation_t *op, const char *name, const char *value);
cosmos_error_code_t cosmos_operation_with_session_token(
    cosmos_operation_t *op, const char *token);
cosmos_error_code_t cosmos_operation_with_activity_id(
    cosmos_operation_t *op, const char *activity_id_uuid);
cosmos_error_code_t cosmos_operation_with_precondition_if_match(
    cosmos_operation_t *op, const char *etag);
cosmos_error_code_t cosmos_operation_with_precondition_if_none_match(
    cosmos_operation_t *op, const char *etag);
cosmos_error_code_t cosmos_operation_with_patch_max_attempts(
    cosmos_operation_t *op, uint8_t max_attempts);

void cosmos_operation_free(cosmos_operation_t *op);  // safe even after execute
```

### 4.7 Execution + response (`src/handles/response.rs`)

```c
cosmos_error_code_t cosmos_driver_execute(
    cosmos_call_context_t *ctx,
    const cosmos_driver_t *driver,
    cosmos_operation_t *op,                       // CONSUMED on success
    const cosmos_operation_options_t *options,    // nullable
    cosmos_response_t **out_response);

/* Response accessors — all O(1), do not allocate unless noted */
uint16_t cosmos_response_status_code(const cosmos_response_t *r);
double   cosmos_response_request_charge(const cosmos_response_t *r);
cosmos_error_code_t cosmos_response_activity_id(
    const cosmos_response_t *r, const char **out_str);  // borrowed, valid until free
cosmos_error_code_t cosmos_response_session_token(
    const cosmos_response_t *r, const char **out_str_or_null);
cosmos_error_code_t cosmos_response_etag(
    const cosmos_response_t *r, const char **out_str_or_null);
cosmos_error_code_t cosmos_response_continuation_token(
    const cosmos_response_t *r, const char **out_str_or_null);

/* Headers: iteration callback */
typedef void (*cosmos_header_visitor)(
    void *user_data, const char *name, const char *value);
void cosmos_response_iter_headers(
    const cosmos_response_t *r,
    cosmos_header_visitor visitor, void *user_data);

/* Body access — zero-copy view valid for the response's lifetime */
cosmos_bytes_view_t cosmos_response_body(const cosmos_response_t *r);

/* Or, take ownership and free the response shell */
cosmos_error_code_t cosmos_response_into_body(
    cosmos_response_t *r,            // freed by this call
    cosmos_bytes_t *out_body);

/* Diagnostics handle (Arc-cloned) */
cosmos_diagnostics_t *cosmos_response_diagnostics(const cosmos_response_t *r);

void cosmos_response_free(cosmos_response_t *r);
```

### 4.8 Diagnostics (`src/handles/diagnostics.rs`)

`DiagnosticsContext` exposes timings, regions contacted, retry attempts, and per-request `RequestDiagnostics`. We expose it as an opaque handle with accessors:

```c
/* Aggregate metrics */
double  cosmos_diagnostics_total_request_charge(const cosmos_diagnostics_t *d);
uint64_t cosmos_diagnostics_total_elapsed_micros(const cosmos_diagnostics_t *d);
uint32_t cosmos_diagnostics_retry_count(const cosmos_diagnostics_t *d);

/* Region info — iteration via visitor */
typedef void (*cosmos_region_visitor)(
    void *user_data, const char *region_name, const char *endpoint,
    bool succeeded, uint64_t elapsed_micros);
void cosmos_diagnostics_iter_regions_contacted(
    const cosmos_diagnostics_t *d,
    cosmos_region_visitor visitor, void *user_data);

/* Full JSON snapshot for log/telemetry forwarding (allocates) */
cosmos_error_code_t cosmos_diagnostics_to_json(
    const cosmos_diagnostics_t *d, cosmos_bytes_t *out_json);

void cosmos_diagnostics_free(cosmos_diagnostics_t *d);
```

The JSON snapshot is the **only** place the wrapper serializes anything to JSON, and it's purely a debugging aid — schema-agnosticism is preserved on the data plane.

---

## 5. Build & Distribution

Same toolchain as the original wrapper:

- `Cargo.toml`: `crate-type = ["cdylib", "staticlib"]`, `name = "azurecosmosdriver"`.
- `build.rs`: cbindgen-driven; same `cargo:rustc-env=BUILD_IDENTIFIER=…` line.
- `CMakeLists.txt`: corrosion-based, identical structure to the old crate's. New `azurecosmosdriver.pc.in`.
- Header: `include/azurecosmosdriver.h` (checked in, regenerated on each build).
- C test harness in `c_tests/` using the same `TEST_SUITE_BEGIN` / `REQUIRE` / `ASSERT` macros from the old `test_common.h`.

Features (mirror the driver):

```toml
[features]
default = ["tokio", "reqwest", "reqwest_native_tls", "tracing"]
tokio = ["dep:tokio"]
reqwest = ["azure_data_cosmos_driver/reqwest"]
reqwest_native_tls = ["azure_data_cosmos_driver/reqwest_native_tls"]
tracing = ["dep:tracing-subscriber"]
fault_injection = ["azure_data_cosmos_driver/fault_injection"]
```

---

## 6. Error Semantics

The driver's `execute_operation` returns `azure_core::Result<CosmosResponse>` and **does not** map non-success HTTP statuses to errors — that is intentional. The wrapper preserves that contract:

- A successful `cosmos_driver_execute` returns `COSMOS_ERROR_CODE_SUCCESS` even if the server returned `404`, `409`, `412`, `429`, etc. The caller inspects `cosmos_response_status_code(r)`.
- Only transport/auth/marshalling failures produce a non-success `cosmos_error_code_t` on `execute`.

This is a **deliberate change** from the old wrapper, which mapped HTTP statuses into `cosmos_error_code_t` for the caller. The new behavior matches what every other-language SDK needs: full access to status, headers, and (where applicable) body for error responses (e.g., 429 with retry-after, 412 with current ETag, 409 with conflict details).

---

## 7. Versioning & Compatibility

- Crate version tracks `azure_data_cosmos_driver` minor versions.
- ABI breaks (struct layout changes, function removal/signature change) require a **major** crate bump.
- Adding new `cosmos_*_with_*` setter functions or new operation factories is **additive** and does not break ABI.
- The `_unused: u8` placeholder pattern from the old wrapper is **dropped** — we use opaque builders instead, so empty option structs are never visible across the ABI.
- A `cosmos_version()` and `cosmos_driver_abi_version()` pair are exported; consumers must check that `cosmos_driver_abi_version() == COSMOSDRIVER_H_ABI_VERSION` at startup.

---

## 8. Phased Implementation Plan

Each phase is independently shippable, has explicit acceptance criteria, and ends with a green C-test suite.

### Phase 0 — Scaffolding *(Goal: an empty crate that builds and emits a header)*

- Create `sdk/cosmos/azure_data_cosmos_driver_native/` with `Cargo.toml`, `build.rs`, empty `lib.rs`, `CMakeLists.txt`, `azurecosmosdriver.pc.in`, `include/.gitignore`, `cmake/DiscoverTests.cmake`.
- Port `c_str!` macro, `cosmos_string_free`, `cosmos_bytes_free`, `cosmos_version()`, `BUILD_IDENTIFIER` from the old wrapper.
- One C test (`c_tests/version.c`) that loads the library and checks the version string.
- Wire crate into the workspace `Cargo.toml`.

**Done when:** `cargo build -p azure_data_cosmos_driver_native` produces `libazurecosmosdriver.{so,dylib,dll}` and `azurecosmosdriver.h`; `ctest` runs the version test green.

### Phase 1 — Error + context primitives *(Goal: reusable plumbing for every later API)*

- Port `CosmosError` / `CosmosErrorCode` / `Error` (extended with the new 40xx codes from §3.5).
- Port `CallContext` + `run_sync` / `run_async` helpers.
- `cosmos_call_context_create` / `_free`.
- Error-handling C tests (null-pointer rejection, error-detail string lifecycle).

**Done when:** Calling APIs with `NULL` runtime, `NULL` call context, etc. all return the right `cosmos_error_code_t` and populate `ctx->error`.

### Phase 2 — Runtime *(Goal: a single shared `CosmosDriverRuntime` reachable from C)*

- Implement `runtime/tokio.rs`: owns a `Runtime` **and** an `Arc<CosmosDriverRuntime>` built via `CosmosDriverRuntimeBuilder::new().build()`.
- Expose `cosmos_runtime_create` / `cosmos_runtime_free`.
- Expose `cosmos_runtime_builder_*` mirror of `CosmosDriverRuntimeBuilder` (workload id, correlation id, user-agent suffix, connection pool options, operation-option defaults).
- `c_tests/runtime_lifecycle.c`: create/free in loop, concurrent `CallContext` usage from multiple threads.

**Done when:** Multiple threads can build `CallContext`s on top of one runtime and tear them down cleanly.

### Phase 3 — Account / resource references + driver instance *(Goal: open a connection to a real Cosmos account)*

- `cosmos_account_ref_with_master_key`, `with_resource_token`, `clone`, `free`.
- `cosmos_database_ref_create`, `cosmos_container_ref_create`, and matching frees.
- `cosmos_driver_options_builder_*` (start with: preferred regions, excluded regions, read consistency, content-response-on-write, allow-emulator-invalid-certs).
- `cosmos_driver_get_or_create` → wraps `runtime.get_or_create_driver(account, options).await`.
- `cosmos_driver_initialize`, `cosmos_driver_free`.
- `c_tests/driver_init.c`: emulator-backed test that creates a driver, asserts `initialize()` succeeds.

**Done when:** A C test against the emulator can stand up a `cosmos_driver_t`, free it, recreate it, and see the cached instance.

### Phase 4 — Partition keys *(Goal: build every partition-key shape C needs)*

- Builder + accessors per §4.5.
- `c_tests/partition_key.c`: covers all 5 value types, single + hierarchical keys, edge cases (empty key returns `INVALID_PARTITION_KEY`).

**Done when:** Round-trip: build → debug-print via a Rust-side test helper → assert the wire value matches the gateway baseline `PartitionKeyHashBaselineTest.*.xml` files already in the driver `testdata/`.

### Phase 5 — Operation construction *(Goal: every `CosmosOperation::*` factory has a C entry point, no execution yet)*

- All factories in §4.6.
- All `with_*` mutators.
- `cosmos_operation_free` (idempotent; safe to call after `execute`).
- Operation-options builder (`OperationOptions`) with: `with_consistency_level`, `with_session_token`, `with_throughput_control_group_name`, `with_priority_level`, `with_end_to_end_timeout`, `with_max_item_count`, `with_excluded_regions`, `with_content_response_on_write`.

**Done when:** Unit tests in Rust can build each operation shape via the C entry points and assert the resulting `CosmosOperation` fields equal those produced by the native Rust constructors.

### Phase 6 — Execute + response *(Goal: end-to-end CRUD)*

- `cosmos_driver_execute` (consumes the operation handle).
- `cosmos_response_*` accessors (body view, into_body, status, RU charge, activity id, ETag, session token, continuation token, header iteration).
- `c_tests/item_crud.c`: create / read / replace / upsert / delete / patch against the emulator, asserting status codes, RU charge > 0, body round-trip.
- Cross-partition `read_all_items` is **not** in this phase (no pager yet).

**Done when:** Full single-item CRUD passes against the emulator; non-success statuses (404 on read-after-delete) surface as `COSMOS_ERROR_CODE_SUCCESS` from `execute` with `cosmos_response_status_code() == 404`.

### Phase 7 — Diagnostics surface *(Goal: language SDKs can log + emit OTel from C)*

- `cosmos_diagnostics_*` accessors per §4.8.
- `cosmos_diagnostics_to_json` (the one sanctioned JSON producer in the wrapper, gated behind the diagnostics feature).
- `c_tests/diagnostics.c`: asserts RU charge matches the response's, retry count is sane, region-contacted callback fires at least once.

**Done when:** A `to_json` snapshot can be diff'd against the Rust driver's `DiagnosticsContext::Debug` output and matches structurally.

### Phase 8 — Pagination (read-feeds & query) *(Goal: handle multi-page responses)*

- Block-on iterator handle: `cosmos_pager_t`, with:
  - `cosmos_driver_execute_pager` → returns `cosmos_pager_t*` for `read_all_*` / `query_*` operations.
  - `cosmos_pager_next` → fetches next `cosmos_response_t*`, or returns `COSMOS_ERROR_CODE_PAGER_EXHAUSTED`.
  - `cosmos_pager_continuation_token` (borrowed view).
  - `cosmos_pager_free`.
- Initially supports server-side cross-partition + single-partition queries that don't require local plan execution.
- `c_tests/pagination.c`: emulator-backed read-all + simple query test.

**Done when:** A query that spans multiple pages collects all results across page boundaries; partial-page resumption via continuation token works.

### Phase 9 — Patch & transactional batch *(Goal: parity with the driver's specialty operations)*

- Expose patch via the existing operation factory + `cosmos_operation_with_patch_max_attempts`, but add helpers to build patch documents as raw bytes (caller provides the JSON; wrapper just wraps it in a body).
- TransactionalBatch: opaque builder (`cosmos_batch_t`) + per-op append + `cosmos_driver_execute_batch`. Body marshalling is bytes-only, consistent with §3.3.

**Done when:** Emulator-backed C tests pass for: add/remove/replace/set/incr patch ops, and a 5-operation batch (create + replace + delete) with both success and 412-precondition-failed batch outcomes.

### Phase 10 — Optional advanced surface *(Goal: opt-in features that aren't required for parity)*

Each item below is independent; ship as feature-gated when ready.

- **Fault injection** (`fault_injection` feature): minimal handle to register a single rule, drop rules. Useful for cross-language SDK reliability tests.
- **In-memory emulator** (`__internal_in_memory_emulator` feature, test-only): not exported through the public C header; used by `c_tests/` to run without an external emulator.
- **AAD callback authentication**: `cosmos_account_ref_with_token_provider(callback, user_data)` that converts a C callback into an `azure_core::credentials::TokenCredential` adapter.
- **Tracing initialization**: `cosmos_enable_tracing()` ported as-is from the old wrapper.

---

## 9. Open Questions

1. **Should `cosmos_runtime_t` cap to exactly one instance per process?** The driver's `CosmosDriverRuntime` is process-cardinal in the spirit of `ARCHITECTURE.md`; do we enforce that via `RuntimeAlreadyInitialized`, or allow N for testing convenience?
2. **Bytes ownership for header iteration.** Should `cosmos_response_iter_headers` give the visitor borrowed pointers (current proposal) or a copy each time? Borrowed is cheaper but the visitor must not stash pointers — needs documentation.
3. **Continuation token format.** The driver currently treats continuation tokens as opaque strings. If we move to byte-level tokens (binary encoding), the C API should expose them via `cosmos_bytes_view_t`, not `const char *`. Decide before Phase 8.
4. **C++ header.** cbindgen's `cpp_compat: true` is sufficient for C++ consumers; do we also want a thin C++ wrapper (`azurecosmosdriver.hpp`) with RAII handle wrappers? Probably out-of-scope for v1.
5. **Symbol stripping.** The old crate used `Box::into_raw` and friends. We may want `-Cstrip=symbols` on release builds; verify that doesn't trip up corrosion's debug-symbol discovery on Windows.

---

## 10. Migration Notes from the Old Wrapper

For anyone consulting the deleted `azure_data_cosmos_native` crate as a reference:

| Old (`azure_data_cosmos_native`) | New (`azure_data_cosmos_driver_native`) |
|---|---|
| `cosmos_client_create_with_key` | `cosmos_account_ref_with_master_key` + `cosmos_driver_get_or_create` |
| `cosmos_client_database_client` | `cosmos_database_ref_create` (no network call) |
| `cosmos_database_create_container` | `cosmos_operation_create_container` + body bytes + `cosmos_driver_execute` |
| `cosmos_container_create_item(pk, json_data)` | `cosmos_operation_create_item` + `with_partition_key` + `with_body(bytes_view)` + `execute` |
| Returned `out_json` (NUL-terminated `const char*`) | Returns `cosmos_response_t*` with byte-view body |
| HTTP errors mapped to `cosmos_error_code_t` | HTTP status surfaced via `cosmos_response_status_code` (see §6) |
| One `ContainerClient` per container | Cheap `cosmos_container_ref_t` value handles |
| Tokio runtime hidden inside `CosmosClient` | Tokio runtime explicit on `cosmos_runtime_t` |
| No diagnostics access | Full `cosmos_diagnostics_*` surface |

The new model is a **lower-level, more explicit, more powerful** API. Convenience and ergonomics belong in each host-language SDK that consumes these bindings.

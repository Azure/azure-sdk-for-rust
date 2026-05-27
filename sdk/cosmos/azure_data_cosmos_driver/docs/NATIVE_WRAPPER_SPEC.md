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

All exported symbols start with `cosmos_`. The names in the **C type** column are normative — generated cbindgen output **must** match them exactly.

Achieving that requires the wrapper's `cbindgen.toml` to:

- Rename the underlying Rust types so they emit without the redundant `Cosmos` prefix and with the `_t` suffix this spec uses. Concretely (`export.rename`):

  ```toml
  [export.rename]
  "RuntimeContext"     = "cosmos_runtime_t"
  "CosmosDriver"       = "cosmos_driver_t"
  "AccountReference"   = "cosmos_account_ref_t"
  "DatabaseReference"  = "cosmos_database_ref_t"
  "ContainerReference" = "cosmos_container_ref_t"
  "PartitionKey"       = "cosmos_partition_key_t"
  "CosmosOperation"    = "cosmos_operation_t"
  "CosmosResponse"     = "cosmos_response_t"
  "DiagnosticsContext" = "cosmos_diagnostics_t"
  "CallContext"        = "cosmos_call_context_t"
  "CosmosError"        = "cosmos_error_t"
  "CosmosErrorCode"    = "cosmos_error_code_t"
  ```

- Restrict the items cbindgen exports to those explicitly defined in this spec (`export.item_types = ["functions", "constants", "enums", "structs", "opaque", "typedefs"]` plus an `include_list` / `exclude` policy). Driver-internal types such as `CosmosStatus`, `SubStatusCode`, `ResponseHeaders`, etc. must **not** leak into the generated header — they are surfaced (where needed) through explicit wrapper-defined accessors with `cosmos_*` names.

- Keep `rename_variants = "QualifiedScreamingSnakeCase"` for enums (e.g. `COSMOS_ERROR_KIND_TRANSPORT`, `COSMOS_READ_CONSISTENCY_SESSION`).

A CI check should diff the regenerated header against `include/azurecosmosdriver.h` and fail the build on any unrenamed `cosmos_cosmos_*`-style identifier, any missing `_t` suffix on the types above, or any newly-exported driver-internal type. This is what keeps §4's surface inventory and the actual ABI in sync over time.

---

## 3. Core FFI Patterns

Several of these patterns are inherited from earlier work — but the inheritance is *not* uniform, and this section calls out the lineage explicitly so reviewers can trace what is new vs. what is established prior art:

- **From PR [#2906](https://github.com/Azure/azure-sdk-for-rust/pull/2906) (deleted `azure_data_cosmos_native` bootstrap)** — `c_str!` macro, `BUILD_IDENTIFIER` env var pattern, cbindgen-at-build with `.gitignore`d header (this crate **reverses** that to check the header in — see §5.1), `package.name` / `[lib].name` split, MPL-2.0 entry in `deny.toml`, CMake + Corrosion bootstrap.
- **From PR #3347 and follow-ups** — `CallContext` + `RuntimeContext` shape and ownership, the `cosmos_error_code_t` value-range layout (FFI / Cosmos / plumbing bands), `cosmos_string_free` / `cosmos_bytes_free`, the `cosmos_*` symbol prefix convention, the `_t` suffix on type names, the `_unused: u8` placeholder pattern (which this crate **drops** — see §7).
- **New in this crate** — every `cosmos_*` API documented in §3.5 and §4 below, the cbindgen `export.rename` / `item_types` policy from §2.2, the rich `cosmos_error_t` accessor + predicate surface from §3.5.2, the operation factory / mutator surface from §4.6, the driver cache normative documentation from §4.4.1, the ABI version major-equal / minor-≥ rule from §7.

When this spec says "inherited from the original wrapper" elsewhere, it means PR #2906 specifically. Patterns introduced in later PRs are called out by PR number at the point of use.

### 3.1 `CallContext` + `RuntimeContext`

Both types are **opaque** to the C ABI — consumers receive `cosmos_*_t *` pointers and use accessor functions. Publishing a concrete struct layout would freeze G4's ABI-stability promise the first time either type grows a field, so every interaction goes through documented C entry points instead.

```c
typedef struct cosmos_runtime cosmos_runtime_t;
typedef struct cosmos_call_context cosmos_call_context_t;

/* Lifecycle. */
cosmos_call_context_t *cosmos_call_context_create(const cosmos_runtime_t *runtime);
void                   cosmos_call_context_free(cosmos_call_context_t *ctx);

/* Borrowed accessor for the bound runtime. Lifetime = the call context. */
const cosmos_runtime_t *cosmos_call_context_runtime(const cosmos_call_context_t *ctx);

/* When set, fallible APIs populate the rich cosmos_error_t payload (see §3.5.2)
 * in addition to returning the coarse cosmos_error_code_t. Defaults to true. */
void cosmos_call_context_set_include_error_details(cosmos_call_context_t *ctx,
                                                   bool include);
bool cosmos_call_context_include_error_details(const cosmos_call_context_t *ctx);

/* Error access — see §3.5.2 for details. */
const cosmos_error_t *cosmos_call_context_last_error(const cosmos_call_context_t *ctx);
cosmos_error_t       *cosmos_call_context_take_error(cosmos_call_context_t *ctx);
```

- A `RuntimeContext` owns the async runtime (Tokio by default) **plus a strong reference to a shared `CosmosDriverRuntime`** (see §4.1). It is reference-counted internally; one process typically creates exactly one.
- A `CallContext` is a heap-allocated opaque handle the caller obtains via `cosmos_call_context_create`. It carries the runtime pointer and receives the most recent error. Reusable across calls but **not** thread-safe; one per caller-thread.

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

Because the driver is schema-agnostic, request/response bodies are raw bytes, not C strings. The wrapper exposes two distinct types — a **view-by-value** for caller-owned inputs and an **opaque handle** for SDK-owned outputs:

```c
// Caller-owned input: caller keeps memory live for the duration of the call.
// Layout is published because this is pass-by-value across the ABI.
typedef struct cosmos_bytes_view {
    const uint8_t *data;
    size_t len;
} cosmos_bytes_view_t;

// SDK-owned output: opaque handle. Layout is intentionally NOT published so the
// internal representation (currently a Box<Vec<u8>>) can evolve without an ABI
// break.
typedef struct cosmos_bytes cosmos_bytes_t;

const uint8_t *cosmos_bytes_data(const cosmos_bytes_t *b);  /* borrowed; valid until _free */
size_t         cosmos_bytes_len(const cosmos_bytes_t *b);
void           cosmos_bytes_free(cosmos_bytes_t *b);        /* NULL is a no-op */
```

Rationale:

- Bodies may legitimately contain `0x00` bytes (Cosmos binary encoding), so NUL-terminated `const char*` cannot represent them.
- Keeping `cosmos_bytes_t` opaque lets the Rust side hold a `Box<Vec<u8>>` (or anything else — `bytes::Bytes`, mmap-backed buffer, refcounted slice) and free it via `Box::from_raw` without exposing the storage representation through the ABI.
- `cosmos_bytes_view_t` keeps its published struct layout because views are passed by value as inputs; treating them as opaque would force every caller to round-trip through `_create` / `_free` for an ephemeral input.

### 3.4 Handle ownership rules

| Handle | Created by | Freed by | Cloneable? |
|---|---|---|---|
| `cosmos_runtime_t*` | `cosmos_runtime_create` | `cosmos_runtime_free` | No (use one per process) |
| `cosmos_driver_t*` | `cosmos_driver_get_or_create` | `cosmos_driver_free` | Internally `Arc`; FFI handle is a single owner |
| `cosmos_call_context_t*` | `cosmos_call_context_create(runtime, ctx_options)` | `cosmos_call_context_free` | No (single-thread-affine; create one per logical caller) |
| `cosmos_account_ref_t*` | `cosmos_account_ref_with_*` | `cosmos_account_ref_free` | Yes, via `cosmos_account_ref_clone` (cheap; new strong handle to the same `Arc`) |
| `cosmos_database_ref_t*` / `cosmos_container_ref_t*` | `cosmos_*_ref_create` from parent | matching `_free` | Yes, via `cosmos_*_ref_clone` (cheap) |
| `cosmos_partition_key_t*` | `cosmos_partition_key_builder_build` / `cosmos_partition_key_from_string` | `cosmos_partition_key_free` | Yes, via `cosmos_partition_key_clone` (cheap) |
| `cosmos_feed_range_t*` | `cosmos_feed_range_full` / `cosmos_feed_range_for_partition_key` | `cosmos_feed_range_free` | Yes, via `cosmos_feed_range_clone` (cheap; small `enum FeedRange` copy) |
| `cosmos_operation_t*` | `cosmos_operation_*` factory | `cosmos_operation_free` (always safe — see §4.6 "Execute consumption" subsection) | No (move semantics on `execute`) |
| `cosmos_response_t*` | `cosmos_driver_execute` | `cosmos_response_free` | No |
| `cosmos_bytes_t*` | `cosmos_response_into_body` / `cosmos_diagnostics_to_json` | `cosmos_bytes_free(bytes)` (single-owner heap allocation; see §3.3) | No (the underlying `bytes::Bytes` is internally refcounted but the FFI handle is single-owner) |
| `cosmos_diagnostics_t*` | `cosmos_response_diagnostics` / `cosmos_error_diagnostics` | `cosmos_diagnostics_free` (drops `Arc`) | Internally `Arc`; each accessor returns a new strong handle the caller must free |
| `cosmos_error_t*` | populated into a caller-supplied `cosmos_error_t` slot, or surfaced via `cosmos_call_context_take_error` | `cosmos_error_free` (only for handles returned from `_take_error`; in-`CallContext` errors are owned by the context) | No |

`cosmos_driver_t` is **the** unit of cardinality. Each call to `cosmos_driver_get_or_create` for the same account endpoint returns the same underlying driver instance (the runtime caches them — see the cache-key discussion in §4.4). The FFI handle, however, is a distinct `Box<Arc<CosmosDriver>>` — freeing it only drops one `Arc` strong count.

**Cloning is a refcount bump, not a deep copy.** The `_clone` functions on reference / partition-key / feed-range handles allocate a fresh FFI handle that aliases the same underlying `Arc<…>` (where one exists) or copies a small `Vec<PartitionKeyValue>` / `enum FeedRange` (for partition keys and feed ranges). Cloning never touches the network. Every successful `_clone` must be paired with a matching `_free`.

### 3.5 Error model

The wrapper's error surface is built on two complementary types — a coarse `cosmos_error_code_t` numeric return value for the C function contract, and a rich `cosmos_error_t` payload that mirrors the driver's `azure_data_cosmos::Error` (introduced in [#4442](https://github.com/Azure/azure-sdk-for-rust/pull/4442)). Both **must** be exposed so language SDKs can implement retry policies, throttling backoff, and conditional-write recovery without re-parsing HTTP headers.

> **Landing prerequisites — read this before implementing.** The §3.5.2 rich-error surface and the §6 error-semantics chapter both depend on PR [#4442](https://github.com/Azure/azure-sdk-for-rust/pull/4442) ("Refactoring to use `Error` instead of `azure_core::Error`") landing first. On `main` today `execute_singleton_operation` still returns `azure_core::Result<…>`, so none of the `cosmos_error_*` accessors below can be wired until #4442 merges. Predicate placement, backtrace-knob naming, and the `Service` / `Transport` / `Client` / `Authentication` / `Serialization` / `Configuration` `Kind` taxonomy are all owned by #4442 — verify the final shape against the merged commit before lifting this section out of draft. Sub-status synthetic codes (`20008`, `20912`, `20010..=20015`, `20020..=20021`, `20030`, `20402`) are also defined in #4442.

#### 3.5.1 `cosmos_error_code_t`

A coarse numeric return value for every fallible C function. The layout retains the FFI / Cosmos-specific ranges established by the old wrapper:

- `0` — `SUCCESS`
- `1..=999` — FFI / argument-validation errors carried over from the old wrapper (null-pointer rejection, invalid UTF-8, etc.). Reuse the existing assignments verbatim.
- `1001..=1999` — auth / conversion errors carried over from the old wrapper.
- `2001..=2999` — Cosmos-specific errors carried over from the old wrapper (no HTTP-status mapping — see §6 for why).
- `3001..=3999` — FFI plumbing errors carried over from the old wrapper.
- `4001..=4999` — **driver-wrapper-specific** fatal codes new in this crate:

  | Code | Variant | Meaning |
  |---|---|---|
  | 4002 | `DRIVER_NOT_INITIALIZED` | Operation issued before `initialize()` completed (should not happen via `get_or_create`). |
  | 4003 | `INVALID_ACCOUNT_REFERENCE` | Account endpoint URL or credential could not be parsed. |
  | 4004 | `INVALID_PARTITION_KEY` | `PartitionKey` builder produced an empty / inconsistent key. |
  | 4005 | `OPERATION_CONSUMED` | `cosmos_operation_*` mutator or a second `cosmos_driver_execute` was called after the operation handle was already consumed by an earlier successful `execute`. (See §4.6 "Execute consumption".) |
  | 4006 | `RESPONSE_CONSUMED` | `cosmos_response_into_*` called twice on the same response. |
  | 4007 | `FEED_EXHAUSTED` | `cosmos_driver_execute` returned `Ok(None)` from the driver — meaning the call was a feed page that produced no more data. Use the pager API in §4.7 to iterate feeds; a single-shot `execute` against a feed-style operation will surface this code rather than panic. |
  | 4008 | `PRECONDITION_ALREADY_SET` | A second precondition setter (`with_precondition_if_match` / `with_precondition_if_none_match`) was called on an operation that already has a precondition. The driver's `with_precondition` takes a single `Precondition` enum value, so only one of If-Match / If-None-Match may be set per operation. |
  | 4009 | `UNSUPPORTED_OPERATION_FOR_MUTATOR` | A mutator only meaningful for a specific operation kind (e.g. `with_patch_max_attempts` on a non-patch operation) was rejected at the FFI boundary. |
  | 4010 | `INVALID_HEADER_NAME` / `INVALID_HEADER_VALUE` | A `cosmos_operation_with_request_header` call passed a non-ASCII / control-character header name or value. |

  Code `4001` is **reserved** (formerly used for `OPTIONS_IGNORED_ON_CACHE_HIT`, which was moved to the `5xxx` warning class — see below — once the SUCCESS-plus-populated-error pattern was rejected). `4001..=4999` is otherwise reserved for additive growth; consumers must treat unknown `4xxx` codes as fatal but recoverable (i.e. log + propagate) rather than panic.

- `5001..=5999` — **non-fatal warnings.** A `5xxx` return is **not** `SUCCESS`; `out_*` pointers are **populated** (the call did the work) and the rich `cosmos_error_t` is populated as advisory detail. Host SDKs that follow the convention `if (code != SUCCESS) handle_error();` will safely treat warnings as failures by default. Host SDKs that want to opt into the advisory treat the warning explicitly. There is **no** "success with populated error" return pattern in this ABI.

  | Code | Variant | Meaning |
  |---|---|---|
  | 5001 | `OPTIONS_IGNORED_ON_CACHE_HIT` | `cosmos_driver_get_or_create` was called with non-NULL `options` while a driver for the same account endpoint was already cached. **`out_driver` is populated with the cached instance.** The passed `options` were dropped. Treat as fatal (host SDK rejects mismatched options) or ignore (host SDK accepts cached instance) per local policy. See §4.4.1. |

  `5001..=5999` is reserved for additive growth; consumers must treat unknown `5xxx` codes the same way: `out_*` populated, warning details on the rich error.

The wrapper **must not** invent 4xxx codes for things that already correspond to a `cosmos_error_t::kind()` — those go through the rich error type instead.

#### 3.5.2 `cosmos_error_t` (rich payload, mirrors `azure_data_cosmos::Error`)

The driver's new `Error` type (PR [#4442](https://github.com/Azure/azure-sdk-for-rust/pull/4442)) carries structured information that every host SDK needs in order to implement correct retry policies and conditional-write recovery. The wrapper mirrors that surface 1:1 through accessor functions on `cosmos_error_t`:

```c
/* Kind — mirrors azure_data_cosmos::Kind. #[non_exhaustive] on the Rust side,
 * so consumers MUST treat unknown values as COSMOS_ERROR_KIND_UNKNOWN. */
typedef enum cosmos_error_kind {
    COSMOS_ERROR_KIND_SERVICE        = 0,
    COSMOS_ERROR_KIND_TRANSPORT      = 1,
    COSMOS_ERROR_KIND_CLIENT         = 2,
    COSMOS_ERROR_KIND_AUTHENTICATION = 3,
    COSMOS_ERROR_KIND_SERIALIZATION  = 4,
    COSMOS_ERROR_KIND_CONFIGURATION  = 5,
    /* Sentinel: any non-zero value not in the list above. The wrapper maps
     * unknown driver Kind variants to this so that older C clients keep
     * compiling when the driver grows new variants. */
    COSMOS_ERROR_KIND_UNKNOWN        = 255,
} cosmos_error_kind_t;

/* Categorical & status accessors. Always populated, including for client-side
 * synthetic errors (where status is a synthetic CosmosStatus). */
cosmos_error_kind_t cosmos_error_kind(const cosmos_error_t *e);
uint16_t            cosmos_error_status_code(const cosmos_error_t *e);
/* sub_status: -1 if absent; otherwise a non-negative integer (e.g. 20008 for
 * CLIENT_OPERATION_TIMEOUT, 20010 for TRANSPORT_CONNECTION_FAILED, etc.).
 * The wrapper does NOT enumerate the sub-status space — see the driver's
 * SubStatusCode for the authoritative list. */
int32_t             cosmos_error_sub_status(const cosmos_error_t *e);

/* Message — borrowed UTF-8, valid until the error is freed. */
const char *cosmos_error_message(const cosmos_error_t *e);

/* Raw service-error response body (e.g. the JSON returned by the gateway on a
 * 4xx/5xx). Returns {NULL, 0} for client-side / transport / serialization
 * errors that don't have a service response. */
cosmos_bytes_view_t cosmos_error_response_body(const cosmos_error_t *e);

/* Diagnostics for the request that produced this error. Returns a NEW handle
 * that must be freed via cosmos_diagnostics_free. Returns NULL when not
 * available (e.g. some Configuration errors). */
cosmos_diagnostics_t *cosmos_error_diagnostics(const cosmos_error_t *e);

/* Backtrace — rate-limited rendered string (Option<&str> on the Rust side).
 * Returns NULL when no backtrace was captured (e.g. budget exhausted). */
const char *cosmos_error_backtrace(const cosmos_error_t *e);

/* Typed header accessors for service errors. All return NULL for non-service
 * errors. Borrowed strings, valid until the error is freed. */
const char *cosmos_error_activity_id(const cosmos_error_t *e);
const char *cosmos_error_session_token(const cosmos_error_t *e);
const char *cosmos_error_etag(const cosmos_error_t *e);
/* retry_after: -1 if not present; otherwise milliseconds. */
int64_t     cosmos_error_retry_after_ms(const cosmos_error_t *e);

/* Predicates — flat namespace on cosmos_error_t for caller ergonomics. As of
 * PR #4442, the equivalent helpers on the Rust side live on the inner
 * azure_data_cosmos::CosmosStatus type (invoked as `err.status().is_*()`),
 * not directly on Error. The wrapper hides that detail: every predicate below
 * forwards internally to the corresponding CosmosStatus method, returning
 * false for errors that have no associated status (rare — most Client / Config
 * errors still carry a synthetic status). */
bool cosmos_error_is_transient(const cosmos_error_t *e);
bool cosmos_error_is_throttled(const cosmos_error_t *e);
bool cosmos_error_is_not_found(const cosmos_error_t *e);
bool cosmos_error_is_conflict(const cosmos_error_t *e);
bool cosmos_error_is_precondition_failed(const cosmos_error_t *e);
bool cosmos_error_is_timeout(const cosmos_error_t *e);
bool cosmos_error_is_gone(const cosmos_error_t *e);
/* True iff Kind == Service (i.e. the error originated from a non-2xx HTTP
 * response from the gateway, as opposed to client / transport / config
 * failures). Mirrors CosmosStatus::is_service_error. */
bool cosmos_error_is_service_error(const cosmos_error_t *e);

/* Free a cosmos_error_t obtained from cosmos_call_context_take_error.
 * Errors stored *inside* a CallContext are owned by the CallContext and freed
 * with it; call _take_error to detach an error before _free. */
void cosmos_error_free(cosmos_error_t *e);
```

**Where `cosmos_error_t` is populated.** For every fallible API that takes a `cosmos_call_context_t *ctx`, the wrapper stores the rich error inside the call context on failure (and on `5xxx` warnings). Callers retrieve the most recent error via `cosmos_call_context_last_error(ctx)` (borrowed, lifetime = the next call on this context) or `cosmos_call_context_take_error(ctx)` (transfers ownership; caller must `cosmos_error_free`). Populating the rich error is controlled by `cosmos_call_context_set_include_error_details` (default `true`); host SDKs that only care about the coarse code can disable rich capture for a tiny per-call allocation saving. For the non-`ctx` factories in §4.3/§4.5 the caller passes a `cosmos_error_t *out_error` slot.

**Wrapper does NOT construct `cosmos_error_t`.** Errors are only ever *received* from the driver; no `cosmos_error_create_*` API is exposed.

**Synthetic sub-status codes** for client-side failures (see PR #4442 for the authoritative list, e.g. `CLIENT_OPERATION_TIMEOUT = 20008`, `CLOSED_CLIENT = 20912`, transport `20010..=20015`, serialization `20020..=20021`, configuration `20030`, authentication `20402`) are surfaced verbatim through `cosmos_error_sub_status` — the wrapper does not re-number them.

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

> **Landing prerequisites.** The companion implementation prototype lives in PR [#4452](https://github.com/Azure/azure-sdk-for-rust/pull/4452) ("Implement `azure_data_cosmos_driver_native` crate"). #4452 is the implementation of this spec — its symbol set, `_t` suffix policy, and `export.rename` configuration **must** be reconciled with §2.2 before either PR merges (#4452 currently checks in headers using `cosmos_cosmos_*` symbols without `_t` suffix, which §2.2 mandates CI must reject). Any builder method below whose Rust counterpart only exists on a branch in #4452 is marked "*landed in #4452*" inline; once #4452 merges, those marks are removed.

`DriverOptions` in the driver crate is a small, account-scoped settings bag (3 fields: the bound `AccountReference`, an `Arc<OperationOptions>` carrying the per-driver operation defaults, and a `Vec<Region>` of preferred regions — see `src/options/driver_options.rs:44-127`). The wrapper exposes a builder handle that mirrors `DriverOptionsBuilder` exactly — including the fact that the builder is constructed from an `AccountReference`:

```c
typedef struct cosmos_driver_options_builder cosmos_driver_options_builder_t;
typedef struct cosmos_driver_options cosmos_driver_options_t;

/* Mirrors DriverOptionsBuilder::new(account). The account ref is borrowed
 * during build; the produced cosmos_driver_options_t holds its own clone. */
cosmos_driver_options_builder_t *cosmos_driver_options_builder_new(
    const cosmos_account_ref_t *account);
void cosmos_driver_options_builder_free(cosmos_driver_options_builder_t *b);

/* Mirrors DriverOptionsBuilder::with_preferred_regions. */
cosmos_error_code_t cosmos_driver_options_builder_with_preferred_regions(
    cosmos_driver_options_builder_t *b,
    const char *const *regions, size_t regions_len);

/* Mirrors DriverOptionsBuilder::with_operation_options. Takes ownership of
 * the provided cosmos_operation_options_t handle (consumed). */
cosmos_error_code_t cosmos_driver_options_builder_with_operation_options(
    cosmos_driver_options_builder_t *b,
    cosmos_operation_options_t *operation_options);

cosmos_driver_options_t *cosmos_driver_options_builder_build(
    cosmos_driver_options_builder_t *b);  /* consumes builder */
void cosmos_driver_options_free(cosmos_driver_options_t *opts);
```

That is the **entire** `DriverOptions` surface. The settings frequently associated with "per-driver" defaults in older Cosmos SDKs — `excluded_regions`, `read_consistency_strategy`, `content_response_on_write`, throughput-control group, priority, end-to-end timeout, per-partition circuit-breaker tuning (8 knobs), retry counts (`max_failover_retry_count`, `max_session_retry_count`), `session_capturing_disabled`, `endpoint_unavailability_ttl`, and custom headers — all live on `OperationOptions` in this driver (`src/options/operation_options.rs:41-188`, 17 public fields), not `DriverOptions`. They are exposed under `cosmos_operation_options_*` (per-call) and can be set as driver-wide defaults by stashing them in the `DriverOptions` via `cosmos_driver_options_builder_with_operation_options`. **`max_item_count` is the exception**: it lives directly on `CosmosOperation::with_max_item_count` (not on `OperationOptions`) and is exposed through the §4.6.2 mutators rather than `cosmos_operation_options_*`. See Phase 5 in §8 for the full enumeration of `OperationOptions` setters the wrapper ships.

Likewise, transport-side knobs (connection pool sizing, user-agent suffix, workload id, correlation id, emulator-certificate trust) live on `CosmosDriverRuntimeBuilder` and are exposed under `cosmos_runtime_builder_*`, **not** `cosmos_driver_options_*`. There is no `cosmos_driver_options_builder_with_allow_emulator_invalid_certs` — that knob lives on the runtime.

Same builder pattern applies to:

- `cosmos_runtime_builder_*` (mirrors `CosmosDriverRuntimeBuilder`, including emulator-trust / connection-pool / user-agent suffix / workload id / correlation id / the Tokio thread-name prefix exposed via `CosmosDriverRuntimeBuilder::with_tokio_thread_name_prefix` — *landed in #4452*).
- `cosmos_operation_options_*` (mirrors `OperationOptionsBuilder` — see §4.6 for the full list of mirrored setters).
- `cosmos_diagnostics_options_*` (mirrors `DiagnosticsOptions`).

### 4.3 Account / Database / Container references (`src/handles/account.rs` etc.)

```c
/* AccountReference — wraps azure_data_cosmos_driver::models::AccountReference.
 * Mirrors AccountReference::with_master_key(endpoint, key) at
 * src/models/account_reference.rs:205. */
cosmos_error_code_t cosmos_account_ref_with_master_key(
    const char *endpoint,
    const char *key,
    cosmos_account_ref_t **out_account,
    cosmos_error_t *out_error);

/* Mirrors AccountReference::with_credential(endpoint, Arc<dyn TokenCredential>)
 * at src/models/account_reference.rs:216. The credential is supplied by a C
 * callback that the wrapper adapts into a TokenCredential impl; the callback +
 * its user_data are kept alive by the AccountReference's Arc. */
cosmos_error_code_t cosmos_account_ref_with_credential(
    const char *endpoint,
    cosmos_token_provider_t credential,   /* see §4.10 */
    void *user_data,
    void (*user_data_free)(void *user_data),
    cosmos_account_ref_t **out_account,
    cosmos_error_t *out_error);

/* Resource-token authentication. The driver does NOT have a dedicated
 * AccountReference::with_resource_token constructor — resource tokens are
 * passed through the same Secret-backed master-key code path. The wrapper
 * keeps this as a distinct C function purely for caller clarity; internally
 * it routes to AccountReference::with_master_key with a Secret built from
 * the token string. If the driver later adds a first-class resource-token
 * constructor, this function will switch to it without an ABI change. */
cosmos_error_code_t cosmos_account_ref_with_resource_token(
    const char *endpoint,
    const char *token,
    cosmos_account_ref_t **out_account,
    cosmos_error_t *out_error);

/* Cheap clone — produces an independent FFI handle aliasing the same
 * Arc<AccountReferenceInner>. Never touches the network. */
cosmos_error_code_t cosmos_account_ref_clone(
    const cosmos_account_ref_t *account,
    cosmos_account_ref_t **out_clone);

void cosmos_account_ref_free(cosmos_account_ref_t *account);

/* DatabaseReference / ContainerReference — pure value types, no network. */
cosmos_error_code_t cosmos_database_ref_create(
    const cosmos_account_ref_t *account,
    const char *database_id,
    cosmos_database_ref_t **out_database);

cosmos_error_code_t cosmos_database_ref_clone(
    const cosmos_database_ref_t *database,
    cosmos_database_ref_t **out_clone);

void cosmos_database_ref_free(cosmos_database_ref_t *database);

cosmos_error_code_t cosmos_container_ref_create(
    const cosmos_database_ref_t *database,
    const char *container_id,
    cosmos_container_ref_t **out_container);

cosmos_error_code_t cosmos_container_ref_clone(
    const cosmos_container_ref_t *container,
    cosmos_container_ref_t **out_clone);

void cosmos_container_ref_free(cosmos_container_ref_t *container);
```

These are pure value-types that do not touch the network — they correspond to the driver's reference types in `src/models/resource_reference.rs`. The `_clone` functions are explicitly part of the surface (see §3.4) because every host SDK that holds reference handles inside its own object model needs cheap independent ownership.

### 4.4 Driver instance (`src/handles/driver.rs`)

```c
cosmos_error_code_t cosmos_driver_get_or_create(
    cosmos_call_context_t *ctx,
    const cosmos_account_ref_t *account,
    const cosmos_driver_options_t *options,  /* nullable */
    cosmos_driver_t **out_driver);

void cosmos_driver_free(cosmos_driver_t *driver);

/* Convenience: explicit initialization. Normally unnecessary because
 * get_or_create already awaits initialize() on first creation. */
cosmos_error_code_t cosmos_driver_initialize(
    cosmos_call_context_t *ctx,
    const cosmos_driver_t *driver);
```

`get_or_create` calls `CosmosDriverRuntime::get_or_create_driver(account, options).await` (`src/driver/runtime.rs:364-395`) which already awaits `initialize()`. The explicit `cosmos_driver_initialize` is exposed for parity but is normally unnecessary.

#### 4.4.1 Driver cache semantics — *normative*

The wrapper inherits the driver's cache exactly as implemented in `runtime.rs:364-395`. Host SDKs (and language-binding authors) **must** account for the following:

- **Cache key.** The cache key is the account endpoint URL **only** (`account.endpoint().to_string()`, `runtime.rs:369`). It does **not** include the credential identity, the application name, the `DriverOptions` contents, or anything else.
- **Options are silently ignored on cache hit.** If `cosmos_driver_get_or_create` is called twice for the same endpoint with different `options`, the second call returns the *first* driver and discards the second `options` argument (`runtime.rs:374`). When the wrapper detects this — i.e. `options != NULL` and the cache already contained an entry — it **must**:
  1. Populate `*out_driver` with the cached instance.
  2. Populate `cosmos_call_context_last_error(ctx)` with an advisory `cosmos_error_t` whose code is `COSMOS_ERROR_CODE_OPTIONS_IGNORED_ON_CACHE_HIT` (`5001`).
  3. **Return `5001`** — *not* `SUCCESS`. `5001` lives in the `5xxx` warning class (see §3.5.1) which by convention populates `out_*` like a success but is non-zero so host SDKs that do `if (code != SUCCESS) handle_error();` will not silently miss the advisory. Host SDKs that want to accept the cached instance explicitly switch on the warning code; the rest correctly treat it as a hard error. This is **not** firing-mode-dependent: the warning is always emitted on cache-hit-with-options. (Earlier drafts predicated it on the unresolved single-runtime-mode decision in §9 Q1; that coupling is removed.)
- **Credential collisions.** Two `cosmos_account_ref_t`s built with the same endpoint but **different credentials** collide in the cache: the first credential wins, the second is silently dropped. The wrapper does **not** transparently detect this (the driver does not currently surface credential identity through `AccountReference`). Host SDKs that need to multiplex independent credentials against a single endpoint must build a workaround — typically by using `cosmos_runtime_create` per credential, since cache scoping is per-runtime. See §9 Q6.
- **Driver lifetime is bounded by the runtime.** Freeing the last FFI handle to a cached driver does **not** evict the cache entry — the runtime keeps a strong `Arc` reference. Eviction happens only when the owning `cosmos_runtime_t` is freed. Tests that need to force a fresh driver must create a fresh runtime.
- **Lost-race redundant init.** `runtime.rs:380-390` uses `or_insert_with` rather than a double-checked-lock pattern: if N callers concurrently invoke `get_or_create_driver` for the same brand-new endpoint, each may run a redundant network `initialize()` before all-but-one of the resulting drivers is dropped. The wrapper does not mitigate this — host SDKs that warm a pool of `CallContext`s at startup should serialize the first `get_or_create` for each endpoint to avoid duplicate metadata round-trips.

These behaviors are **not** new in this wrapper — they are the contract of the underlying driver. The wrapper documents them prominently because the old wrapper (`azure_data_cosmos_native`) hid the cache behind a per-`ContainerClient` opaque object, so few host-SDK authors will have encountered them before.

### 4.5 Partition keys (`src/handles/partition_key.rs`)

`PartitionKey` is a `Vec<PartitionKeyValue>` (`models/partition_key.rs:303`). The driver supports six logical value kinds — `String`, `Number`, `Bool`, `Null`, `Undefined`, and `Infinity` — but only the first five are reachable through the driver's **public** API. `Infinity` is documented as "used only internally for EPK boundary calculations" (`models/partition_key.rs:44`) and is `pub(crate)`; its predicate `is_infinity()` is also `pub(crate)`. The wrapper therefore exposes the five public variants and **does not** ship a `cosmos_partition_key_builder_append_infinity` in v1.

If a host SDK needs to construct EPK upper-bound sentinels for cross-partition range queries — the only scenario where `Infinity` matters externally — that should arrive through `cosmos_feed_range_t` (§4.6.4) rather than through a partition-key builder, because EPK boundary construction is a feed-range concern, not a partition-key one. See §9 Q10 for the open question on whether to promote `Infinity` to `pub` in the driver vs. routing EPK boundaries entirely through the feed-range surface.

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
/* Appends an Undefined component (driver variant InnerPartitionKeyValue::Undefined,
 * exposed publicly as PartitionKeyValue::UNDEFINED).
 *
 * This represents an item with **no value** at the partition-key path —
 * the JSON-undefined semantics Cosmos uses for sparse partition keys in
 * hierarchical layouts. It is NOT the JSON literal `null`, which has its
 * own dedicated _append_null above. */
cosmos_error_code_t cosmos_partition_key_builder_append_undefined(
    cosmos_partition_key_builder_t *b);

/* NOTE: No `_append_infinity` is exposed in v1 — the driver's
 * `InnerPartitionKeyValue::Infinity` is `pub(crate)` and has no public
 * constructor. EPK upper-bound sentinels (the only use case) are
 * expressed through `cosmos_feed_range_t` (§4.6.4) instead. */

cosmos_error_code_t cosmos_partition_key_builder_build(
    cosmos_partition_key_builder_t *b,     // consumed on success
    cosmos_partition_key_t **out_pk);

void cosmos_partition_key_free(cosmos_partition_key_t *pk);
```

A convenience helper for the common 1-value case:

```c
cosmos_error_code_t cosmos_partition_key_from_string(
    const char *value, cosmos_partition_key_t **out_pk);

/* Cheap clone — copies the underlying Vec<PartitionKeyValue>. */
cosmos_error_code_t cosmos_partition_key_clone(
    const cosmos_partition_key_t *pk,
    cosmos_partition_key_t **out_clone);
```

### 4.6 Operations (`src/handles/operation.rs`)

This is the heart of the new wrapper. Operations mirror the `CosmosOperation` constructors in `src/models/cosmos_operation.rs`. Every constructor produces a heap-owned operation that the caller mutates with `with_*` shims and finally passes to `cosmos_driver_execute`.

Naming convention: `cosmos_operation_<rust_constructor_name>`.

#### 4.6.1 Partition keys live on the factory, not on a mutator

Unlike the old wrapper, the driver's `CosmosOperation` does **not** carry a settable partition key. Partition keys are baked into the operation's underlying `ItemReference` at construction time (`src/models/resource_reference.rs:268`, `ItemReference::from_name(container, item_name, pk)`). The two operations that take a *container-level* partition key — `read_all_items(container, pk)` and `batch(container, pk)` — do so via their factory arguments. Accordingly, the wrapper **does not** expose a `cosmos_operation_with_partition_key` mutator. The partition key is supplied directly to every item / container-feed factory.

**Factory signature template.** Every factory follows the normative §3.2 shape: returns `cosmos_error_code_t` and writes the allocated operation through `cosmos_operation_t **out_op`. This lets the factories report null-arg rejection (`COSMOS_ERROR_CODE_INVALID_ARGUMENT`), OOM on the internal `Box` allocation, invalid UTF-8 on `item_id`, etc., without resorting to a "poisoned handle" pattern. The earlier draft used bare-pointer returns and was changed here to keep the factory surface consistent with the rest of the ABI.

```c
/* Account-scope */
cosmos_error_code_t cosmos_operation_create_database(
    const cosmos_account_ref_t *account,
    cosmos_operation_t **out_op);
cosmos_error_code_t cosmos_operation_read_all_databases(
    const cosmos_account_ref_t *account,
    cosmos_operation_t **out_op);
cosmos_error_code_t cosmos_operation_query_databases(
    const cosmos_account_ref_t *account,
    cosmos_operation_t **out_op);

/* Database-scope */
cosmos_error_code_t cosmos_operation_read_database(
    const cosmos_database_ref_t *db,
    cosmos_operation_t **out_op);
cosmos_error_code_t cosmos_operation_delete_database(
    const cosmos_database_ref_t *db,
    cosmos_operation_t **out_op);
cosmos_error_code_t cosmos_operation_create_container(
    const cosmos_database_ref_t *db,
    cosmos_operation_t **out_op);
cosmos_error_code_t cosmos_operation_read_all_containers(
    const cosmos_database_ref_t *db,
    cosmos_operation_t **out_op);
cosmos_error_code_t cosmos_operation_query_containers(
    const cosmos_database_ref_t *db,
    cosmos_operation_t **out_op);

/* Container-scope */
cosmos_error_code_t cosmos_operation_read_container(
    const cosmos_container_ref_t *c, cosmos_operation_t **out_op);
cosmos_error_code_t cosmos_operation_replace_container(
    const cosmos_container_ref_t *c, cosmos_operation_t **out_op);
cosmos_error_code_t cosmos_operation_delete_container(
    const cosmos_container_ref_t *c, cosmos_operation_t **out_op);

/* Single-partition feed read — mirrors CosmosOperation::read_all_items(c, pk). */
cosmos_error_code_t cosmos_operation_read_all_items(
    const cosmos_container_ref_t *c, const cosmos_partition_key_t *pk,
    cosmos_operation_t **out_op);

/* Cross-partition feed read — mirrors
 * CosmosOperation::read_all_items_cross_partition(c) at
 * src/models/cosmos_operation.rs:611. */
cosmos_error_code_t cosmos_operation_read_all_items_cross_partition(
    const cosmos_container_ref_t *c, cosmos_operation_t **out_op);

/* Query — mirrors CosmosOperation::query_items(container, Option<FeedRange>)
 * at src/models/cosmos_operation.rs:625.
 *
 * The driver's query_items takes an optional FeedRange that targets a
 * specific physical partition (NULL = entire container, i.e. cross-partition);
 * it does NOT take a SQL string directly. The SQL query text and
 * parameters live in the operation's request body as JSON of shape
 *
 *     { "query": "SELECT * FROM c WHERE c.foo = @p", "parameters": [...] }
 *
 * and are attached via cosmos_operation_with_body() per the
 * schema-agnostic data-plane contract (G2). The wrapper does NOT JSON-encode
 * the query body — host SDKs build the bytes in their native serializer
 * exactly as they do for create / replace / upsert item bodies.
 *
 * Phase 5 ships this factory + body-attach pattern; Phase 8 wires the
 * resulting operation through the pager. */
cosmos_error_code_t cosmos_operation_query_items(
    const cosmos_container_ref_t *c,
    const cosmos_feed_range_t *feed_range,    /* nullable — entire container */
    cosmos_operation_t **out_op);

/* Query plan — mirrors CosmosOperation::query_plan(container,
 * supported_query_features) at src/models/cosmos_operation.rs:639.
 *
 * NOTE: query_plan is NOT a SQL-execution entry point. It is a metadata
 * fetch that asks the gateway "given that the client can execute the
 * following query features, return a compatible query plan for this
 * container." The `supported_features_mask` is the comma-separated string
 * the driver sends as the `x-ms-cosmos-supported-query-features` header
 * (e.g. "OrderBy,TopAndLimit,Aggregate"). Most language SDKs never call
 * this directly — they execute queries via cosmos_operation_query_items
 * above and let the driver's pipeline decide whether a query plan is
 * needed. We expose it for parity with the driver's public surface;
 * cross-language SDKs that implement their own query-plan caching can
 * consume it. */
cosmos_error_code_t cosmos_operation_query_plan_for_features(
    const cosmos_container_ref_t *c,
    const char *supported_features_mask,
    cosmos_operation_t **out_op);

/* Item-scope — partition key is baked into the underlying ItemReference at
 * construction (src/models/resource_reference.rs:268). */
cosmos_error_code_t cosmos_operation_create_item(
    const cosmos_container_ref_t *c, const char *item_id,
    const cosmos_partition_key_t *pk, cosmos_operation_t **out_op);
cosmos_error_code_t cosmos_operation_read_item(
    const cosmos_container_ref_t *c, const char *item_id,
    const cosmos_partition_key_t *pk, cosmos_operation_t **out_op);
cosmos_error_code_t cosmos_operation_upsert_item(
    const cosmos_container_ref_t *c, const char *item_id,
    const cosmos_partition_key_t *pk, cosmos_operation_t **out_op);
cosmos_error_code_t cosmos_operation_replace_item(
    const cosmos_container_ref_t *c, const char *item_id,
    const cosmos_partition_key_t *pk, cosmos_operation_t **out_op);
cosmos_error_code_t cosmos_operation_delete_item(
    const cosmos_container_ref_t *c, const char *item_id,
    const cosmos_partition_key_t *pk, cosmos_operation_t **out_op);
cosmos_error_code_t cosmos_operation_patch_item(
    const cosmos_container_ref_t *c, const char *item_id,
    const cosmos_partition_key_t *pk, cosmos_operation_t **out_op);

/* Transactional batch — mirrors CosmosOperation::batch(c, pk) at
 * src/models/cosmos_operation.rs:549. Sub-operations are appended via the
 * batch builder (out of scope for the initial surface — see Phase 9). */
cosmos_error_code_t cosmos_operation_batch(
    const cosmos_container_ref_t *c, const cosmos_partition_key_t *pk,
    cosmos_operation_t **out_op);

/* Offer / throughput operations — mirror CosmosOperation::query_offers /
 * read_offer / replace_offer at src/models/cosmos_operation.rs:733/743/754.
 * These take an account ref because throughput offers are addressed by
 * resource link, not container ref. */
cosmos_error_code_t cosmos_operation_query_offers(
    const cosmos_account_ref_t *account, cosmos_operation_t **out_op);
cosmos_error_code_t cosmos_operation_read_offer(
    const cosmos_account_ref_t *account, const char *resource_link,
    cosmos_operation_t **out_op);
cosmos_error_code_t cosmos_operation_replace_offer(
    const cosmos_account_ref_t *account, const char *resource_link,
    cosmos_operation_t **out_op);
```

A minimal `cosmos_feed_range_t` builder surface (mirroring the driver's `FeedRange` constructors) is exposed under §4.6.4. NULL `feed_range` on `cosmos_operation_query_items` targets the entire container.

The partition key passed to a factory is **cloned** into the operation — the caller retains ownership of its `cosmos_partition_key_t` and must `cosmos_partition_key_free` it independently.

#### 4.6.2 Mutators

```c
/* Body — UTF-8 JSON bytes. Replaces any previously-set body. */
cosmos_error_code_t cosmos_operation_with_body(
    cosmos_operation_t *op, cosmos_bytes_view_t body);

/* Request header — incremental. Each call appends one header to a builder
 * that is finalized into the driver's CosmosRequestHeaders at execute time.
 * (The driver's CosmosOperation::with_request_headers at
 * src/models/cosmos_operation.rs:140 takes a complete CosmosRequestHeaders
 * value and REPLACES the previous one — the wrapper accumulates incrementally
 * so the C ABI can stay header-at-a-time.) */
cosmos_error_code_t cosmos_operation_with_request_header(
    cosmos_operation_t *op, const char *name, const char *value);

/* Session token, activity id, max item count, populate index/query metrics —
 * mirror CosmosOperation::with_session_token / with_activity_id /
 * with_max_item_count / with_populate_index_metrics /
 * with_populate_query_metrics. */
cosmos_error_code_t cosmos_operation_with_session_token(
    cosmos_operation_t *op, const char *token);
cosmos_error_code_t cosmos_operation_with_activity_id(
    cosmos_operation_t *op, const char *activity_id_uuid);
cosmos_error_code_t cosmos_operation_with_max_item_count(
    cosmos_operation_t *op, int32_t max_item_count);
cosmos_error_code_t cosmos_operation_with_populate_index_metrics(
    cosmos_operation_t *op, bool enable);
cosmos_error_code_t cosmos_operation_with_populate_query_metrics(
    cosmos_operation_t *op, bool enable);

/* Precondition — the driver exposes a single
 * CosmosOperation::with_precondition(Precondition) at
 * src/models/cosmos_operation.rs:184, where Precondition is an enum with
 * IfMatch(etag) and IfNoneMatch(etag) variants (mutually exclusive). The
 * wrapper splits this into two convenience setters for C ergonomics, but the
 * underlying constraint stands: setting one precondition replaces any
 * previously-set precondition. Calling either setter while a precondition is
 * already configured returns COSMOS_ERROR_CODE_PRECONDITION_ALREADY_SET (4008)
 * so host SDKs catch accidental double-set rather than silently overwriting. */
cosmos_error_code_t cosmos_operation_with_precondition_if_match(
    cosmos_operation_t *op, const char *etag);
cosmos_error_code_t cosmos_operation_with_precondition_if_none_match(
    cosmos_operation_t *op, const char *etag);

/* Patch-only mutator — mirrors CosmosOperation::with_patch_max_attempts(NonZeroU8).
 * Wrapper validates max_attempts != 0 and rejects with
 * COSMOS_ERROR_CODE_INVALID_ARGUMENT. Calling this on a non-patch operation
 * returns COSMOS_ERROR_CODE_UNSUPPORTED_OPERATION_FOR_MUTATOR (4009). */
cosmos_error_code_t cosmos_operation_with_patch_max_attempts(
    cosmos_operation_t *op, uint8_t max_attempts);

void cosmos_operation_free(cosmos_operation_t *op);  /* always safe — see §4.6.3 */
```

The wrapper deliberately does **not** expose a generic `cosmos_operation_with_partition_key` mutator (see §4.6.1). Calls that would mirror driver setters not listed above (e.g. `with_consistency_level`, `with_throughput_control_group_name`) live on `cosmos_operation_options_*` and are passed via the `options` argument to `cosmos_driver_execute`.

#### 4.6.3 Execute consumption — *normative*

The driver's `execute_operation` takes the operation by value (move semantics on the Rust side). The C ABI cannot move out of a pointer, so the wrapper uses a sentinel pattern: `cosmos_operation_t*` is backed internally by `Box<Option<CosmosOperation>>`. `execute` takes the inner `CosmosOperation` via `Option::take` and leaves the `Box` in place as a consumed sentinel.

The full contract is:

1. **`cosmos_operation_free` is always safe to call.** It is safe immediately after a successful `execute` (the sentinel is freed), after a failed `execute` (the operation is *not* consumed — see point 4), after multiple `_with_*` calls, and on a fresh handle that was never executed. `cosmos_operation_free(NULL)` is a no-op.
2. **`cosmos_operation_free` is idempotent only in the sense that the handle is then NULL on the caller side.** The wrapper does not track post-free use; double-free is undefined behavior, exactly as in the rest of the C ABI.
3. **A second `cosmos_driver_execute` on a successfully-executed handle returns `COSMOS_ERROR_CODE_OPERATION_CONSUMED` (4005)** and does not touch the network. Mutator calls (`cosmos_operation_with_*`) on a consumed handle return the same error.
4. **A `cosmos_driver_execute` that returns an error does NOT consume the handle.** The caller may inspect the error, mutate the operation (e.g. update the session token from the response, adjust headers), and re-execute. This matches the driver's semantics: `execute_operation` only moves out on the successful path.
5. **`cosmos_operation_t` is non-cloneable.** A host SDK that needs to retry must either keep the inputs and rebuild via a factory, or use the failure-doesn't-consume path in point 4.

#### 4.6.4 FeedRange handle

`cosmos_operation_query_items` (and the Phase 8 pager-side query / read-feed surface that consumes the same operation) accept a `cosmos_feed_range_t *`. The handle mirrors the driver's `FeedRange` type (`src/models/feed_range.rs`) and is exposed through a minimal builder. The v1 surface matches the driver's **current public constructors** exactly; additional variants will be added as the driver's `FeedRange` grows.

```c
typedef struct cosmos_feed_range cosmos_feed_range_t;

/* Entire EPK key space ("" .. FF) — mirrors FeedRange::full() at
 * src/models/feed_range.rs:89. Equivalent to passing NULL to
 * cosmos_operation_query_items. */
cosmos_error_code_t cosmos_feed_range_full(cosmos_feed_range_t **out_fr);

/* FeedRange for a single logical partition key — mirrors
 * FeedRange::for_partition(pk, &PartitionKeyDefinition) at
 * src/models/feed_range.rs:108. The driver requires the container's
 * partition-key definition to compute the effective-partition-key bounds;
 * the wrapper obtains it from `container.partition_key_definition()` so
 * callers do not have to plumb it manually. */
cosmos_error_code_t cosmos_feed_range_for_partition_key(
    const cosmos_container_ref_t *container,
    const cosmos_partition_key_t *pk,
    cosmos_feed_range_t **out_fr);

cosmos_error_code_t cosmos_feed_range_clone(
    const cosmos_feed_range_t *fr, cosmos_feed_range_t **out_clone);
void                cosmos_feed_range_free(cosmos_feed_range_t *fr);
```

**Deferred to a future revision** (no driver-side public constructor today — tracked in §9 Q11):

- A `FeedRange::new(min: EffectivePartitionKey, max: EffectivePartitionKey)` constructor exists on the driver (`feed_range.rs:71`) but takes a strongly-typed `EffectivePartitionKey`, not strings. The wrapper would need either a `cosmos_effective_partition_key_t` opaque type with `_from_hex` / `_min` / `_max` constructors, or a string-parsing helper on the driver side. Neither exists today, so `cosmos_feed_range_for_epk_range(min_hex, max_hex, ...)` is **not** part of v1.
- The driver's internal `FeedRangeRepr` does **not** have a `PartitionKeyRangeId` variant — physical-partition routing happens at a different layer (PPAF/PPCB routing maps), not through `FeedRange`. The earlier draft's `cosmos_feed_range_for_partition_key_range(pkrange_id, ...)` is therefore **dropped** from v1.

If a host SDK needs to resume a query from a continuation token (the typical EPK-range use case), Phase 8's `cosmos_pager_*` surface already covers that via the continuation-token round-trip on the pager handle — no `FeedRange` construction from the C side is required.

### 4.7 Execution + response (`src/handles/response.rs`)

```c
/* Single-shot execute — binds to CosmosDriver::execute_singleton_operation
 * (src/driver/cosmos_driver.rs:1281), which returns Result<CosmosResponse>
 * by collapsing the Option<CosmosResponse> returned by the underlying
 * execute_operation (src/driver/cosmos_driver.rs:1242,1246).
 *
 * Semantics:
 * - On success, *out_response is populated and the operation handle is left
 *   in a consumed sentinel state (see §4.6.3). Return value is
 *   COSMOS_ERROR_CODE_SUCCESS.
 * - On a service / transport / client error, *out_response is NULL, a
 *   cosmos_error_t is written into ctx, the operation is NOT consumed, and
 *   the return value is a non-success cosmos_error_code_t (see §3.5 and §6
 *   for the mapping rules).
 * - If the underlying operation is a feed-style operation that yielded
 *   Ok(None) (no further data), the return value is
 *   COSMOS_ERROR_CODE_FEED_EXHAUSTED (4007) and *out_response is NULL. Use
 *   the pager API below for feed-style operations to avoid this.
 */
cosmos_error_code_t cosmos_driver_execute(
    cosmos_call_context_t *ctx,
    const cosmos_driver_t *driver,
    cosmos_operation_t *op,                       /* consumed on success */
    const cosmos_operation_options_t *options,    /* nullable */
    cosmos_response_t **out_response);

/* Feed / query execution — binds to CosmosDriver::execute_operation
 * directly (src/driver/cosmos_driver.rs:1242,1246). The wrapper holds the
 * operation inside the pager and re-invokes execute_operation per page,
 * surfacing Ok(None) as COSMOS_ERROR_CODE_FEED_EXHAUSTED on cosmos_pager_next.
 * See Phase 8 in §8 for rollout. */
cosmos_error_code_t cosmos_driver_execute_pager(
    cosmos_call_context_t *ctx,
    const cosmos_driver_t *driver,
    cosmos_operation_t *op,                       /* consumed on success */
    const cosmos_operation_options_t *options,    /* nullable */
    cosmos_pager_t **out_pager);

/* Response accessors — all O(1), do not allocate unless noted */
uint16_t cosmos_response_status_code(const cosmos_response_t *r);
double   cosmos_response_request_charge(const cosmos_response_t *r);
cosmos_error_code_t cosmos_response_activity_id(
    const cosmos_response_t *r, const char **out_str);  /* borrowed, valid until free */
cosmos_error_code_t cosmos_response_session_token(
    const cosmos_response_t *r, const char **out_str_or_null);
cosmos_error_code_t cosmos_response_etag(
    const cosmos_response_t *r, const char **out_str_or_null);
cosmos_error_code_t cosmos_response_continuation_token(
    const cosmos_response_t *r, const char **out_str_or_null);

/* Typed-header accessors. Per PR #4401 (merged), the driver's ResponseHeaders
 * is a typed struct of ~27 named Option<...> fields (e.g. x_ms_session_token,
 * x_ms_continuation, etag, last_state_change_utc, content_path) — NOT a
 * generic name/value map. The wrapper therefore exposes one accessor per
 * known header; the original cosmos_response_iter_headers visitor pattern
 * (replaced here) cannot be supported because there is no name/value
 * iteration API on the underlying struct.
 *
 * Each accessor returns COSMOS_ERROR_CODE_SUCCESS and writes a borrowed
 * NUL-terminated UTF-8 pointer (valid until the response is freed) on
 * presence; writes NULL on absence. Numeric / boolean headers return a
 * parsed value via dedicated accessors as needed.
 *
 * The activity-id / session-token / etag / continuation-token accessors
 * above are the high-traffic four; the rest follow the same shape and are
 * added as host SDKs need them:
 *   cosmos_response_x_ms_request_charge_units(...)
 *   cosmos_response_x_ms_resource_quota(...)
 *   cosmos_response_x_ms_resource_usage(...)
 *   cosmos_response_x_ms_retry_after_ms(...)
 *   cosmos_response_x_ms_alt_content_path(...)
 *   cosmos_response_content_path(...)
 *   cosmos_response_last_state_change_utc(...)
 *   ... etc. (see ResponseHeaders for the authoritative list)
 *
 * NOTE: Unknown response headers are dropped at parse time by the driver
 * (ResponseHeaders has no catch-all "other" field). Host SDKs that need
 * forward-compat over future Cosmos headers must wait for a driver-side
 * extension; the wrapper cannot synthesize what the driver discarded. See
 * §9 Q2 for the open decision on whether/how to expose a passthrough.
 */

/* Body access — zero-copy view valid for the response's lifetime.
 * NOTE: For multi-part feed responses (ResponseBody::Items) this returns the
 * first part's bytes only; the multi-part case is exposed via a dedicated
 * cosmos_response_iter_items API to be added with Phase 8. See §9 Q4. */
cosmos_bytes_view_t cosmos_response_body(const cosmos_response_t *r);

/* Or, take ownership of the body and free the response shell. The returned
 * cosmos_bytes_t is an opaque handle (see §3.3); free with cosmos_bytes_free. */
cosmos_error_code_t cosmos_response_into_body(
    cosmos_response_t *r,            /* freed by this call */
    cosmos_bytes_t **out_body);

/* Diagnostics handle (Arc-cloned). Mirrors CosmosResponse::diagnostics()
 * at src/models/cosmos_response.rs:109. Caller must free via
 * cosmos_diagnostics_free. */
cosmos_diagnostics_t *cosmos_response_diagnostics(const cosmos_response_t *r);

void cosmos_response_free(cosmos_response_t *r);
```

The error-payload accessors (`cosmos_error_kind`, `cosmos_error_status_code`, `cosmos_error_sub_status`, `cosmos_error_is_throttled`, etc.) are defined in §3.5.2 and live on `cosmos_error_t` — they are **not** redundantly re-exposed on `cosmos_response_t`. A successful `cosmos_driver_execute` returns a `cosmos_response_t` whose `cosmos_response_status_code` may still be a Cosmos-success status (200, 201, 204, ...); a failed `cosmos_driver_execute` returns a `cosmos_error_t` whose accessors expose the equivalent service-error fields.

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

/* Full JSON snapshot for log/telemetry forwarding (allocates). The returned
 * cosmos_bytes_t is an opaque handle (see §3.3); free with cosmos_bytes_free. */
cosmos_error_code_t cosmos_diagnostics_to_json(
    const cosmos_diagnostics_t *d, cosmos_bytes_t **out_json);

void cosmos_diagnostics_free(cosmos_diagnostics_t *d);
```

The JSON snapshot is the **only** place the wrapper serializes anything to JSON, and it's purely a debugging aid — schema-agnosticism is preserved on the data plane.

---

## 5. Build & Distribution

### 5.1 Toolchain & layout

The crate inherits the C-FFI toolchain established by PR [#2906](https://github.com/Azure/azure-sdk-for-rust/pull/2906) (`c_str!` macro, `BUILD_IDENTIFIER` env, cbindgen-at-build, headers `.gitignore`d, `package.name` / `[lib].name` split, MPL-2.0 in `deny.toml`, CMake + Corrosion bootstrap) and the runtime / call-context / error-payload primitives added in PR #3347 (`CallContext`, `RuntimeContext`, `cosmos_error_code_t` value-range layout, `cosmos_string_free`, `cosmos_bytes_free`, `_unused: u8` placeholder convention).

What is **new** in this crate vs the inherited surface:

- `Cargo.toml`: `crate-type = ["cdylib", "staticlib"]`, `name = "azurecosmosdriver"` (driver-specific, distinct from the deleted `azure_data_cosmos_native` crate's `azurecosmos`).
- `build.rs`: cbindgen-driven, with the explicit `export.rename` / `item_types` policy from §2.2. Keeps the `cargo:rustc-env=BUILD_IDENTIFIER=…` line inherited from #2906.
- `CMakeLists.txt`: corrosion-based, identical structure to the deleted crate's. New `azurecosmosdriver.pc.in`.
- **Header check-in policy.** This crate **reverses** the `.gitignore` convention from #2906: `include/azurecosmosdriver.h` is **checked in** so language-binding consumers can vendor it without a Rust toolchain. The build regenerates the header and a CI check diffs the regenerated output against the checked-in copy (see §2.2 for the rename / item-types invariants the diff enforces).
- C test harness in `c_tests/` using the same `TEST_SUITE_BEGIN` / `REQUIRE` / `ASSERT` macros from the old `test_common.h`.

### 5.2 Cargo features

The feature matrix mirrors the driver. Default features deliberately match `azure_data_cosmos_driver`'s defaults (rustls, Tokio) — host SDKs that need OpenSSL or want to swap the TLS provider opt in explicitly:

```toml
[features]
default = ["tokio", "rustls"]
tokio = ["dep:tokio", "azure_data_cosmos_driver/tokio"]
reqwest = ["azure_data_cosmos_driver/reqwest"]
rustls = ["azure_data_cosmos_driver/rustls"]
native_tls = ["azure_data_cosmos_driver/native_tls"]
fault_injection = ["azure_data_cosmos_driver/fault_injection"]
# __internal_* features are private to the workspace test surface — they
# follow the driver's __internal_in_memory_emulator / __internal_* pattern
# and are NOT documented for external consumption.
```

`tracing` is a workspace dependency (`workspace.dependencies`), not a Cargo feature. Tracing initialization for the wrapper is exposed via `cosmos_enable_tracing()` (Phase 10).

### 5.3 Ancillary tooling re-introduction checklist

**Lessons from #4090 / #4103.** PR [#4090](https://github.com/Azure/azure-sdk-for-rust/pull/4090) added the original `azure_data_cosmos_native` crate and missed several pieces of repo plumbing that subsequent PRs had to patch; PR [#4103](https://github.com/Azure/azure-sdk-for-rust/pull/4103) then ripped the crate out and, in doing so, removed *more* than just the crate sources — workspace `members`, `deny.toml` license overrides, sibling-crate cross-links in `azure_data_cosmos/README.md` + `src/lib.rs` + `ARCHITECTURE.md`, and the `eng/dict/*` + `.cspell.json` entries all went with it. The two PRs together form the authoritative "what does this crate need beyond its own sources" inventory. Phase 0 of this wrapper **must** treat every item below as a hard prerequisite — without the workspace-members entry the crate will not even be `cargo check`-able; without `deny.toml` the MPL-2.0 license check fails in CI; without the cross-links the sibling `azure_data_cosmos` crate continues to advertise a non-existent integration story. The complete checklist is:

- [ ] **`Cargo.toml` workspace `members`** — add `sdk/cosmos/azure_data_cosmos_driver_native` to `[workspace] members = [...]` at the repo root. **P0 — without this, `cargo` cannot see the crate and every subsequent step fails.**
- [ ] **`deny.toml`** — re-add the MPL-2.0 license allowance (removed by #4103) so `cargo deny check licenses` continues to pass for transitively-pulled deps like `webpki-roots`.
- [ ] **`sdk/cosmos/azure_data_cosmos_driver_native/Cargo.toml`** — declare `[lib] crate-type = ["cdylib", "staticlib"]`, the workspace inheritance block, and `[build-dependencies] cbindgen = ...`.
- [ ] **`sdk/cosmos/azure_data_cosmos/README.md`** — restore the "via C API wrapper" cross-link paragraph (deleted by #4103) so the canonical Rust SDK still surfaces the C ABI as a discoverability path.
- [ ] **`sdk/cosmos/azure_data_cosmos/src/lib.rs`** — restore the crate-level doc-comment cross-reference to the native wrapper that #4103 stripped.
- [ ] **`sdk/cosmos/azure_data_cosmos/ARCHITECTURE.md`** — analogrelay flagged a stale crate reference on #4103; refresh the wrapper paragraph to point at `azure_data_cosmos_driver_native` rather than the old `azure_data_cosmos_native`.
- [ ] `eng/dict/rust-custom.txt` — re-add `azurecosmosdriver`, `corrosion`, `cbindgen` entries.
- [ ] `eng/dict/crates.txt` — re-add `azure_data_cosmos_driver_native`.
- [ ] `sdk/cosmos/.cspell.json` — re-add `ignoreWords` for `azurecosmosdriver`, header guard macros, C-test helper names. Run `eng/common/spelling/Invoke-Cspell.ps1` against `sdk/cosmos/azure_data_cosmos_driver_native/**` and diff the result against the equivalent run on `azure_data_cosmos` to catch regressions like the `brazilsouth` token the Copilot bot flagged on #4103.
- [ ] `eng/scripts/verify-dependencies.rs` — re-add an exemption for the new crate's `cdylib`/`staticlib` lib-only target (if still required by the script's current rules).
- [ ] `Cargo.lock` — `cbindgen` MUST be reintroduced strictly as a `[build-dependencies]` entry of `azure_data_cosmos_driver_native`. Per heaths' decision in #2906 review, `cbindgen` is **not** to be promoted to a workspace-level dependency or moved to runtime `[dependencies]`. The build-dep entry is the only place it appears.
- [ ] `AGENTS.md` — re-add the `azure_data_cosmos_driver_native` entry under the Cosmos crate taxonomy.
- [ ] `.github/skills/cosmos-pre-commit-validation/SKILL.md` — re-add scope hint covering the new crate (file globs, expected lint surface).
- [ ] `.github/skills/cosmos-design-struct/SKILL.md` — re-add scope hint covering the new crate.
- [ ] **Deleted-file disposition** — #4090 / #4103 churned `sdk/cosmos/azure_data_cosmos_native/azurecosmos.pc.in`, `cmake/DiscoverTests.cmake`, and `docs/next_generation_sdks_design_principles.md`. None of those are reintroduced here: `pkg-config` files are deferred to a future packaging RFC, the CMake test discovery is replaced by the §8 Phase 11 C test harness sitting under `tests/c_smoke/`, and the design-principles doc has been folded into this spec's §2 + §11. Phase 0 should add a one-line README pointer noting where each old file's content now lives so future grep-driven archaeologists do not chase ghosts.

Each line is intentionally a checklist item rather than prose — Phase 0 acceptance requires every box checked.

---

## 6. Error Semantics

The driver moved to a structured error type in PR [#4442](https://github.com/Azure/azure-sdk-for-rust/pull/4442): `azure_data_cosmos::Result<T>` with a rich `Error` carrying a `Kind` enum (Service / Transport / Client / Authentication / Serialization / Configuration; `#[non_exhaustive]`), typed accessors (status, sub_status, headers, diagnostics, response body, backtrace, source), and predicates (`is_throttled`, `is_not_found`, `is_conflict`, `is_precondition_failed`, `is_timeout`, `is_gone`, `is_transient`). Synthetic sub-status codes (e.g. `CLIENT_OPERATION_TIMEOUT = 20008`, `CLOSED_CLIENT = 20912`, transport `20010..=20015`, serialization `20020..=20021`, configuration `20030`, authentication `20402`) make every client-side failure observable through the same typed surface as service errors.

The wrapper's contract is shaped by that decision:

### 6.1 Return-type mapping

- `cosmos_driver_execute` binds to `CosmosDriver::execute_singleton_operation` (`src/driver/cosmos_driver.rs:1281`), which returns `Result<CosmosResponse>` by collapsing the `Option<CosmosResponse>` returned by `execute_operation` (`src/driver/cosmos_driver.rs:1242,1246`). Three outcomes:
  - **`Ok(CosmosResponse)`** → return `COSMOS_ERROR_CODE_SUCCESS`, populate `*out_response`. The response may itself carry a Cosmos non-success HTTP status (404, 409, 412, 429, ...) only when the driver's policy explicitly does not error on it — see §6.2 below.
  - **`Err(azure_data_cosmos::Error)`** → return the mapped `cosmos_error_code_t` (see §6.3), write the structured `cosmos_error_t` into `ctx`, leave `*out_response = NULL`. The operation handle is **not** consumed (see §4.6.3 point 4).
  - **`execute_singleton_operation` is never expected to surface the `Ok(None)` case**; if the underlying operation is mis-categorized and the driver hands back `None`, the wrapper returns `COSMOS_ERROR_CODE_FEED_EXHAUSTED` (4007) for diagnosability rather than fabricating an empty response.

- `cosmos_driver_execute_pager` binds to `execute_operation` directly and surfaces `Ok(None)` as `COSMOS_ERROR_CODE_FEED_EXHAUSTED` on the *pager*, not the initial call — see Phase 8.

### 6.2 Service errors vs. successful "non-2xx" responses

The driver classifies *every* non-2xx HTTP status that the gateway returns as an `Error` of `Kind::Service` (this is a behavior change from the old wrapper, which returned 404 / 409 / 412 / 429 as a successful `CosmosResponse`). Accordingly:

- The wrapper does **not** return 404 / 409 / 412 / 429 as `COSMOS_ERROR_CODE_SUCCESS` with a non-2xx `cosmos_response_status_code`. Those surface as `Err` with `cosmos_error_kind() == COSMOS_ERROR_KIND_SERVICE` and the appropriate `cosmos_error_status_code()`.
- Host SDKs implement "expected 404" semantics by checking `cosmos_error_is_not_found(err)`; "expected 412 / 409" via `cosmos_error_is_precondition_failed(err)` / `cosmos_error_is_conflict(err)`; 429 retry-after via `cosmos_error_is_throttled(err)` + `cosmos_error_retry_after_ms(err)`. See §3.5.2 for the full accessor surface.
- The auth / metadata initialization path can still surface `azure_core::Error::HttpResponse` errors before the operation is dispatched (`src/driver/cosmos_driver.rs:1104-1114`); those propagate through the same `Kind::Authentication` / `Kind::Service` channels.

### 6.3 `cosmos_error_code_t` ↔ `cosmos_error_t` mapping

When `execute_operation` returns `Err(Error)`, the wrapper picks a coarse `cosmos_error_code_t` based on `Kind` *and* always populates the rich `cosmos_error_t` for full detail:

| `Kind` | Coarse `cosmos_error_code_t` |
|---|---|
| `Kind::Service` | A code from the existing `2001..=2999` Cosmos range, chosen by `status_code` (e.g. 429 → `COSMOS_ERROR_CODE_THROTTLED`, 404 → `COSMOS_ERROR_CODE_NOT_FOUND`, 409 → `COSMOS_ERROR_CODE_CONFLICT`, 412 → `COSMOS_ERROR_CODE_PRECONDITION_FAILED`); unmapped service statuses fall through to `COSMOS_ERROR_CODE_SERVICE_ERROR`. |
| `Kind::Transport` | `COSMOS_ERROR_CODE_TRANSPORT_ERROR` |
| `Kind::Client` | `COSMOS_ERROR_CODE_CLIENT_ERROR` (covers operation-timeout, closed-client, generic client-side failures) |
| `Kind::Authentication` | `COSMOS_ERROR_CODE_AUTHENTICATION_FAILED` |
| `Kind::Serialization` | `COSMOS_ERROR_CODE_SERIALIZATION_FAILED` |
| `Kind::Configuration` | `COSMOS_ERROR_CODE_CONFIGURATION_ERROR` |
| any unknown variant introduced after this spec | `COSMOS_ERROR_CODE_DRIVER_ERROR` (sentinel for `#[non_exhaustive]` future-proofing); `cosmos_error_kind() == COSMOS_ERROR_KIND_UNKNOWN` |

The coarse code is **only** for the common dispatch path (switch in C, "expected vs unexpected" branch). All structured detail — including the synthetic sub-status codes — lives on `cosmos_error_t` and is the source of truth.

### 6.4 Backtrace rate-limit knobs

The driver's `Error` carries an optional backtrace whose capture rate **and** rendering rate are independently bounded — capture (collecting the raw stack) and resolution (symbolicating it) are different operations with different costs. The wrapper mirrors **both** knobs:

| Knob | Per-driver setter | Per-runtime default | Environment override |
|---|---|---|---|
| Backtrace **captures** per second (bounds how often a raw backtrace is grabbed inside an `Error`) | `cosmos_driver_set_max_error_backtrace_captures_per_second(driver, double rate)` | `cosmos_runtime_set_max_error_backtrace_captures_per_second(runtime, double rate)` | `AZURE_COSMOS_BACKTRACE_CAPTURES_PER_SECOND` |
| Backtrace **resolutions** per second (bounds how often a captured backtrace is symbolicated for `cosmos_error_backtrace`) | `cosmos_driver_set_max_error_backtrace_resolutions_per_second(driver, double rate)` | `cosmos_runtime_set_max_error_backtrace_resolutions_per_second(runtime, double rate)` | `AZURE_COSMOS_BACKTRACE_RESOLUTIONS_PER_SECOND` |

Each setter maps 1:1 to the corresponding driver method introduced in #4442 (`CosmosDriverBuilder::with_max_error_backtrace_captures_per_second` / `…_resolutions_per_second` and the `CosmosDriverRuntimeBuilder` equivalents). The environment variables are honored as-is by the driver — the wrapper does not intercept them. Consult the driver's `Error` module for the authoritative default values.

---

## 7. Versioning & Compatibility

- Crate version tracks `azure_data_cosmos_driver` minor versions.
- ABI breaks (struct layout changes, function removal/signature change) require a **major** crate bump.
- Adding new `cosmos_*_with_*` setter functions or new operation factories is **additive** and does not break ABI.
- The `_unused: u8` placeholder pattern from the old wrapper is **dropped** — we use opaque builders instead, so empty option structs are never visible across the ABI.
- A `cosmos_version()` and `cosmos_driver_abi_version()` pair are exported. ABI version is a `uint32_t` whose high 16 bits are the **major** version and low 16 bits are the **minor** version (additive growth bumps minor, breaking changes bump major). Consumers must check at startup that:
  - `(cosmos_driver_abi_version() >> 16) == (COSMOSDRIVER_H_ABI_VERSION >> 16)` — major must match exactly; **and**
  - `(cosmos_driver_abi_version() & 0xFFFF) >= (COSMOSDRIVER_H_ABI_VERSION & 0xFFFF)` — runtime minor must be ≥ the header's minor (additive growth is forward-compatible).

  The reference C helper that consumers can copy:

  ```c
  bool cosmos_abi_compatible_with_header(void) {
      uint32_t lib = cosmos_driver_abi_version();
      uint32_t hdr = COSMOSDRIVER_H_ABI_VERSION;
      return (lib >> 16) == (hdr >> 16) && (lib & 0xFFFF) >= (hdr & 0xFFFF);
  }
  ```

  A strict `lib == hdr` check would defeat the additive-growth promise above (every additive minor bump would require every consumer to rebuild). The major-equal / minor-≥ rule is what makes "adding a new `cosmos_*_with_*` setter" actually additive.

---

## 8. Phased Implementation Plan

Each phase is independently shippable, has explicit acceptance criteria, and ends with a green C-test suite.

### Phase 0 — Scaffolding *(Goal: an empty crate that builds and emits a header)*

- Create `sdk/cosmos/azure_data_cosmos_driver_native/` with `Cargo.toml`, `build.rs`, empty `lib.rs`, `CMakeLists.txt`, `azurecosmosdriver.pc.in`, `cmake/DiscoverTests.cmake`.
- **Check in** `include/azurecosmosdriver.h` (header check-in policy — see §5.1).
- Port `c_str!` macro, `cosmos_string_free`, `cosmos_bytes_free`, `cosmos_version()`, `BUILD_IDENTIFIER` from the old wrapper.
- Configure cbindgen per §2.2 (`export.rename`, `item_types`, `rename_variants`) and wire a CI check that diffs the regenerated header against the checked-in copy.
- Complete the ancillary tooling re-introduction checklist from §5.3 (`eng/dict/*`, `.cspell.json`, `verify-dependencies.rs`, `Cargo.lock` cbindgen entry strictly as `[build-dependencies]`, `AGENTS.md`, `cosmos-*` skill scope hints).
- One C test (`c_tests/version.c`) that loads the library and checks the version string.
- Wire crate into the workspace `Cargo.toml`.

**Done when:** `cargo build -p azure_data_cosmos_driver_native` produces `libazurecosmosdriver.{so,dylib,dll}` and a regenerated `azurecosmosdriver.h` that matches the checked-in copy byte-for-byte; `ctest` runs the version test green; every checklist item in §5.3 is complete.

### Phase 1 — Error + context primitives *(Goal: reusable plumbing for every later API)*

- Port `CosmosError` / `CosmosErrorCode` / `Error` (extended with the new 40xx codes from §3.5.1).
- Implement the rich `cosmos_error_t` accessor + predicate surface from §3.5.2 against `azure_data_cosmos::Error` (from PR #4442).
- Port `CallContext` + `run_sync` / `run_async` helpers.
- `cosmos_call_context_create` / `_free` / `_last_error` / `_take_error`.
- Error-handling C tests (null-pointer rejection, error-detail string lifecycle, `cosmos_error_*` accessor coverage, `Kind::Unknown` future-proofing fallback).

**Done when:** Calling APIs with `NULL` runtime, `NULL` call context, etc. all return the right `cosmos_error_code_t` and populate `ctx->error`; every `cosmos_error_kind_t` variant + every predicate is exercised by at least one synthetic test (no emulator needed).

### Phase 2 — Runtime *(Goal: a single shared `CosmosDriverRuntime` reachable from C)*

- Implement `runtime/tokio.rs`: owns a `Runtime` **and** an `Arc<CosmosDriverRuntime>` built via `CosmosDriverRuntimeBuilder::new().build()`.
- Expose `cosmos_runtime_create` / `cosmos_runtime_free`.
- Expose `cosmos_runtime_builder_*` mirror of `CosmosDriverRuntimeBuilder` (workload id, correlation id, user-agent suffix, connection pool options, operation-option defaults, **Tokio thread-name prefix** introduced in #4452, `allow_emulator_invalid_certs` — see §4.2 on why this lives on the runtime and not on `DriverOptions`).
- Expose `cosmos_runtime_set_max_error_backtrace_captures_per_second` and `cosmos_runtime_set_max_error_backtrace_resolutions_per_second` per §6.4.
- `c_tests/runtime_lifecycle.c`: create/free in loop, concurrent `CallContext` usage from multiple threads.

**Done when:** Multiple threads can build `CallContext`s on top of one runtime and tear them down cleanly.

### Phase 3 — Account / resource references + driver instance *(Goal: open a connection to a real Cosmos account)*

- `cosmos_account_ref_with_master_key`, `with_credential`, `with_resource_token`, `clone`, `free`.
- `cosmos_database_ref_create`, `cosmos_container_ref_create`, matching `_clone` and `_free`.
- `cosmos_driver_options_builder_*` — the actual 3-field surface from §4.2 (`new(account)`, `with_preferred_regions`, `with_operation_options`, `build`). Per-call options (`excluded_regions`, `read_consistency`, `content_response_on_write`, etc.) live on `cosmos_operation_options_*` and are wired in Phase 5.
- `cosmos_driver_get_or_create` → wraps `runtime.get_or_create_driver(account, options).await`, including the warning-class `COSMOS_ERROR_CODE_OPTIONS_IGNORED_ON_CACHE_HIT` (`5001`, non-SUCCESS, `out_driver` still populated) per §4.4.1.
- `cosmos_driver_set_max_error_backtrace_captures_per_second` and `cosmos_driver_set_max_error_backtrace_resolutions_per_second` per §6.4.
- `cosmos_driver_initialize`, `cosmos_driver_free`.
- `c_tests/driver_init.c`: emulator-backed test that creates a driver, asserts `initialize()` succeeds, and asserts the cache-hit advisory fires when a second `get_or_create` passes non-NULL options for the same endpoint.

**Done when:** A C test against the emulator can stand up a `cosmos_driver_t`, free it, recreate it, observe the cached instance, and observe the `OPTIONS_IGNORED_ON_CACHE_HIT` advisory.

### Phase 4 — Partition keys *(Goal: build every partition-key shape C needs)*

- Builder + accessors per §4.5.
- `c_tests/partition_key.c`: covers all 5 value types, single + hierarchical keys, edge cases (empty key returns `INVALID_PARTITION_KEY`).

**Done when:** Round-trip: build → debug-print via a Rust-side test helper → assert the wire value matches the gateway baseline `PartitionKeyHashBaselineTest.*.xml` files already in the driver `testdata/`.

### Phase 5 — Operation construction *(Goal: every `CosmosOperation::*` factory has a C entry point, no execution yet)*

- All factories in §4.6.1 (including `read_all_items_cross_partition`, `query_items`, `query_plan_for_features`, `batch`, `query_offers`, `read_offer`, `replace_offer`).
- All `with_*` mutators from §4.6.2 — including `with_max_item_count`, which lives on `CosmosOperation` itself (NOT on `OperationOptions`; see §4.6.2). Note: **no** `cosmos_operation_with_partition_key` (PK lives on the factory; see §4.6.1).
- `cosmos_operation_free` per the normative semantics in §4.6.3.
- `cosmos_feed_range_*` builder surface per §4.6.4.
- **`cosmos_operation_options_*` builder mirroring the driver's `OperationOptions` (`src/options/operation_options.rs:41-188`, 17 public fields). The wrapper exposes them grouped as below; every field has a `_with_<field>` setter and a `_clear_<field>` resetter so callers can express "inherit from a higher options layer" (see also §4.2 layered resolution):**
  - **Consistency & regions:** `with_consistency_level`, `with_session_token`, `with_excluded_regions`.
  - **Response shape & paging:** `with_content_response_on_write`, `with_priority_level`, `with_end_to_end_timeout`.
  - **Throughput control & QoS:** `with_throughput_control_group_name`.
  - **Per-partition circuit-breaker (8 knobs):** `with_partition_level_circuit_breaker_enabled`, `with_partition_level_circuit_breaker_failure_threshold`, `with_partition_level_circuit_breaker_recovery_window`, `with_partition_level_circuit_breaker_min_request_volume`, `with_partition_level_circuit_breaker_consecutive_failures_threshold`, `with_partition_level_circuit_breaker_open_window`, `with_partition_level_circuit_breaker_half_open_max_requests`, `with_partition_level_circuit_breaker_failure_rate_threshold`. (Confirm the exact names against `operation_options.rs` once #4452 lands; the spec preserves the per-knob granularity so language SDKs can model the same surface as Java / .NET.)
  - **Retry tuning:** `with_max_failover_retry_count`, `with_max_session_retry_count`.
  - **Session capture:** `with_session_capturing_disabled`.
  - **Region availability:** `with_endpoint_unavailability_ttl`.
  - **Custom headers (HashMap<String, String>):** `cosmos_operation_options_set_custom_header(opts, name, value)` for incremental key/value population, plus `cosmos_operation_options_clear_custom_headers(opts)`.

  If a v1 cut is desired before the full surface lands, ship the first three groups (consistency/regions, response shape & paging, throughput) and document the remainder as "Phase 5+ additive growth" with a public tracking issue — but pick that scope explicitly rather than letting fields fall through unrepresented.

**Done when:** Unit tests in Rust can build each operation shape via the C entry points and assert the resulting `CosmosOperation` fields equal those produced by the native Rust constructors.

### Phase 6 — Execute + response *(Goal: end-to-end CRUD)*

- `cosmos_driver_execute` binds to `execute_singleton_operation` per §4.7 / §6.1; populates `cosmos_response_t` on success and the rich `cosmos_error_t` on failure.
- `cosmos_response_*` accessors (body view, into_body, status, RU charge, activity id, ETag, session token, continuation token, header iteration).
- `c_tests/item_crud.c`: create / read / replace / upsert / delete / patch against the emulator, asserting status codes, RU charge > 0, body round-trip.
- Cross-partition `read_all_items` / `query_items` / `batch` are **not** wired into `cosmos_driver_execute` in this phase — they belong to Phase 8 (pager) and Phase 9 (batch).

**Done when:** Full single-item CRUD passes against the emulator; per §6.2, a 404 read-after-delete surfaces as a failed `cosmos_driver_execute` with `cosmos_error_kind() == COSMOS_ERROR_KIND_SERVICE`, `cosmos_error_status_code() == 404`, and `cosmos_error_is_not_found() == true` — **not** as `SUCCESS` with a 404 response.

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

1. **Single-runtime-per-process enforcement.** The driver's `CosmosDriverRuntime` is process-cardinal in the spirit of `ARCHITECTURE.md`; do we enforce that at the wrapper boundary (rejecting a second `cosmos_runtime_create` with a dedicated `cosmos_error_code_t`), allow N for testing convenience, or rely solely on host-SDK discipline? *Not yet decided — §3.5.1 does not pre-allocate a code for the enforcement variant.* (Note: the §4.4.1 `OPTIONS_IGNORED_ON_CACHE_HIT` advisory is **no longer** predicated on this question — it fires unconditionally whenever a cache hit is observed with non-NULL `options`, regardless of runtime cardinality.)
2. **Forward-compat for unknown response headers.** §4.7 freezes the response-header surface as a curated set of typed accessors (`activity_id`, `session_token`, `etag`, `continuation_token`, `request_charge`, plus per-feature additions). Unknown headers are dropped — a host-SDK author asking "what is the equivalent of `IDictionary<string, string> ResponseHeaders` in .NET?" gets nothing. Do we (a) keep the strict typed surface and add a public driver-side `Response::custom_headers()` extension point as features need it, (b) expose a wrapper-only forward-compat passthrough (`cosmos_response_get_custom_header(name)` returning a `cosmos_string_view_t`) populated from a wrapper-maintained passlist of header names the driver does not yet model, or (c) leave it strict and document the limitation? Decide before the response-accessor surface is frozen. (Previously this slot asked about borrow-vs-copy semantics for an `iter_headers` visitor; that visitor was removed in this revision — see §4.7 — so the question is now about the *shape* of the extension story rather than the iteration mechanics.)
3. **Continuation token format.** The driver currently treats continuation tokens as opaque strings. If `azure_data_cosmos` moves to byte-level tokens (binary encoding), the C API should expose them via `cosmos_bytes_view_t` rather than `const char *`. Decide before Phase 8 — the choice is locked in once `cosmos_response_continuation_token` ships.
4. **Multi-part response bodies.** `azure_data_cosmos::CosmosResponse::ResponseBody::Items` carries an iterator over multiple parts. `cosmos_response_body` currently exposes a single `cosmos_bytes_view_t`. Options: (a) collapse parts into one buffer (allocates + copies); (b) expose a dedicated `cosmos_response_iter_items(visitor, user_data)`; (c) defer multi-part to the pager surface only. Pick before Phase 8.
5. **C++ companion header.** cbindgen's `cpp_compat = true` is sufficient for C++ consumers using the C API directly; do we also want a thin `azurecosmosdriver.hpp` with RAII handle wrappers (one struct per `_t`, dtor wired to the matching `_free`, `unique_ptr` semantics)? Probably out-of-scope for v1, but record the decision so host SDKs can plan around it.
6. **Driver cache scoping vs. credential identity.** §4.4.1 documents that two `cosmos_account_ref_t`s sharing an endpoint but using different credentials collide in the cache. Should the wrapper teach the driver to incorporate credential identity (e.g. via `TokenCredential::token_provider_kind()` or a wrapper-supplied opaque tag) into the cache key? If not, document the per-runtime-per-credential workaround prominently in the host-SDK author guide.
7. **ConnectionString parser ownership.** The driver's `ConnectionString` parser currently lives in `azure_data_cosmos`. If the wrapper wants to expose `cosmos_account_ref_from_connection_string(const char *cs, ...)`, do we mirror the parser in the wrapper (extra surface to maintain) or depend on the driver re-exporting the parser publicly?
8. **Symbol stripping on release builds.** The old crate used `Box::into_raw` and friends. We may want `-Cstrip=symbols` on release builds; verify that doesn't trip up corrosion's debug-symbol discovery on Windows, or interact badly with backtrace capture (§6.4).
9. **`cosmos_pager_t` continuation-token re-entry contract.** When a pager is freed mid-iteration, the caller may want to recreate it from a continuation token. Decide whether `cosmos_driver_execute_pager` takes an optional starting continuation token, or whether resumption is always operation-level via `cosmos_operation_with_continuation_token`. Resolve before Phase 8.
10. **`PartitionKeyValue::Infinity` visibility.** The driver's `Infinity` variant is `pub(crate)` (`models/partition_key.rs:44`) and documented as "used only internally for EPK boundary calculations." §4.5 of this spec therefore does **not** ship `cosmos_partition_key_builder_append_infinity` in v1; EPK upper-bound construction is deferred to the feed-range surface (§4.6.4). Decide whether to (a) promote `Infinity` to `pub` in the driver with appropriate use-case documentation, or (b) keep it private and route every EPK-boundary use case through `cosmos_feed_range_*` constructors going forward. The choice affects whether language SDKs ever need a direct partition-key sentinel for boundary semantics; if (b), §4.5 stays as-is permanently.
11. **`FeedRange` v1 constructor gaps.** §4.6.4 ships only the two driver-public constructors today (`FeedRange::full()` and `FeedRange::for_partition(...)`). Two construction shapes that callers may want are deferred because the driver does not expose them in a string-parseable form:
    - **EPK-range construction.** `FeedRange::new(min: EffectivePartitionKey, max: EffectivePartitionKey)` exists on the driver (`feed_range.rs:71`) but takes strongly-typed EPK values, not strings. Decide whether to (a) wait for a driver-side `FeedRange::from_hex_range(min_hex, max_hex)` parser, or (b) add a `cosmos_effective_partition_key_t` opaque type to the wrapper with its own `_from_hex` / `_min` / `_max` constructors. Phase 8 (pager) is the natural forcing function — if continuation-token resumption is operation-level (Q9 option B), this gap may never bite.
    - **Physical-partition-range targeting.** The driver's `FeedRangeRepr` has no `PartitionKeyRangeId` variant; PKRangeId-keyed routing happens elsewhere (PPAF/PPCB). Decide whether host SDKs that want explicit physical-partition pinning should drive that through some other surface, or whether the driver should grow a `FeedRange::for_partition_key_range_id(pkrange_id)` constructor that the wrapper can mirror.

---

## 10. Migration Notes from the Old Wrapper

For anyone consulting the deleted `azure_data_cosmos_native` crate as a reference:

| Old (`azure_data_cosmos_native`) | New (`azure_data_cosmos_driver_native`) |
|---|---|
| `cosmos_client_create_with_key` | `cosmos_account_ref_with_master_key` + `cosmos_driver_get_or_create` |
| `cosmos_client_database_client` | `cosmos_database_ref_create` (no network call) |
| `cosmos_database_create_container` | `cosmos_operation_create_container(database, container_id, &op)` → `cosmos_operation_with_body(op, body_view)` → `cosmos_driver_execute(driver, ctx, op, options, &response)` |
| `cosmos_container_create_item(pk, json_data)` | `cosmos_operation_create_item(container, item_id, pk, &op)` → `cosmos_operation_with_body(op, bytes_view)` → `cosmos_driver_execute(...)` |
| Returned `out_json` (NUL-terminated `const char*`) | Returns `cosmos_response_t*`; body via `cosmos_response_into_body(response, &cosmos_bytes_t_handle)` (caller frees with `cosmos_bytes_free`) |
| HTTP errors mapped to `cosmos_error_code_t` | Surfaced as failed `cosmos_driver_execute` with a rich `cosmos_error_t` retrievable via `cosmos_call_context_last_error(ctx)` / `cosmos_call_context_take_error(ctx)` (see §3.5.2, §6) |
| One `ContainerClient` per container | Cheap `cosmos_container_ref_t` value handles (`_clone` is a refcount bump) |
| Tokio runtime hidden inside `CosmosClient` | Tokio runtime explicit on `cosmos_runtime_t` (one per process; see §4.2) |
| No diagnostics access | Full `cosmos_diagnostics_*` surface (`cosmos_response_diagnostics`, `cosmos_error_diagnostics`, `cosmos_diagnostics_to_json` → `cosmos_bytes_t**`) |

The new model is a **lower-level, more explicit, more powerful** API. Convenience and ergonomics belong in each host-language SDK that consumes these bindings.

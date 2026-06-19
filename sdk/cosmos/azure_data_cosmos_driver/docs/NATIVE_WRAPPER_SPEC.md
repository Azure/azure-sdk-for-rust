# `azure_data_cosmos_driver_native` — C Bindings Specification

> **Status:** Draft / Proposal
> **Owners:** Cosmos DB Rust SDK team
> **Target crate:** `sdk/cosmos/azure_data_cosmos_driver_native`
> **Wraps:** [`azure_data_cosmos_driver`](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/cosmos/azure_data_cosmos_driver) (Layer 1 — driver crate)
> **Supersedes:** `azure_data_cosmos_native` (removed in PR [#4103](https://github.com/Azure/azure-sdk-for-rust/pull/4103), commit `ccf43caae`), which wrapped the high-level `azure_data_cosmos` SDK.

---

## 1. Motivation and Context

### 1.1 Why a new wrapper

The original `azure_data_cosmos_native` crate (commits `de5bf3ba8` → `ccf43caae`) provided C bindings on top of the **typed SDK** (`azure_data_cosmos`). That wrapper:

- Forced `serde` serialization through the FFI layer (it called `read_item::<()>`, `create_item(..., RawValue)`, collected pagers into `Vec` and re-serialized to JSON via `serde_json::to_string`).
- Hid driver-level concepts (diagnostics, RU charge, activity id, region routing, throughput control, session tokens) that other-language SDKs *must* expose to their users.
- Created an upside-down dependency for non-Rust SDKs: a Java/.NET/Python SDK consuming the C ABI would receive responses that had already been parsed by Rust's `serde`, then re-serialized to a string, only to be re-parsed by the host language.

The `azure_data_cosmos_driver` crate was explicitly designed (see [`ARCHITECTURE.md`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/ARCHITECTURE.md) "Schema-Agnostic Data Plane") to be the reuse point for **all** non-Rust language SDKs. Wrapping the driver — not the typed SDK — is the correct boundary.

### 1.2 Goals

| # | Goal |
|---|------|
| G1 | Expose every primitive a language SDK needs to build a fully-featured Cosmos DB client: account, runtime, driver, operations, partition keys, options, responses, diagnostics. |
| G2 | Stay **schema-agnostic**: bodies are `const uint8_t*`/`size_t` in, `const uint8_t*`/`size_t` out. No JSON parsing inside the wrapper. |
| G3 | Map cleanly to the driver's Rust API. A C function should correspond to a single driver method or a small, mechanical builder step. |
| G4 | Be **ABI-stable enough** for `corrosion` / `cmake` consumers; breaking changes only on documented driver minor bumps. |
| G5 | Be **runtime-agnostic at the ABI** (Tokio is an implementation detail behind the `cosmos_runtime_builder_*` family) so a future runtime swap doesn't break C consumers. |
| G6 | Provide first-class **diagnostics** access (request charge, activity id, status, headers, regions contacted, retry attempts). |

### 1.3 Non-goals

- **Item serialization.** Callers pass raw bytes; they handle their own JSON / Cosmos binary encoding.
- **Query plan execution in C.** The driver owns the query engine; the wrapper exposes only the pager-style result iteration once the driver lands query support.
- **Typed `ContainerClient` / `DatabaseClient` C handles** like the old wrapper had. The new model is `cosmos_driver_t` + `cosmos_operation_t` factories scoped by `*_reference_t` handles.
- **A "convenience" layer.** That belongs in each language's own SDK on top of these bindings.

---

## 2. Architecture Overview

The stack, top (host language) to bottom (transport), with each layer calling
the one below it:

- **Consumer** — the Java / .NET / Python / C / C++ language SDK.
- **C ABI boundary** — `azurecosmosdriver.h` (cbindgen-generated) and the
  compiled `libazurecosmosdriver.{so,dylib,dll,.a}`.
- **`azure_data_cosmos_driver_native`** (this crate):
  - `#[no_mangle] extern "C"` functions
  - `CompletionQueue` + `RuntimeContext` glue
  - boxed driver handles
  - bytes-in / bytes-out shims
- **`azure_data_cosmos_driver`** (Layer 1) — `CosmosDriverRuntime`,
  `CosmosDriver`, `CosmosOperation`, `CosmosResponse`, `DiagnosticsContext`,
  `PartitionKey`, etc.
- **`azure_core` / `reqwest` / `tokio`** — HTTP transport and async runtime.

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
│   ├── completion.rs         # CompletionQueue + OperationHandle + Completion records
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
| `CompletionQueue` (wrapper) | `cosmos_cq_t` | `cosmos_cq_*` |
| `OperationHandle` (wrapper) | `cosmos_operation_handle_t` | `cosmos_operation_handle_*` |
| `Completion` (wrapper) | `cosmos_completion_t` | `cosmos_completion_*` |
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
  "CompletionQueue"    = "cosmos_cq_t"
  "OperationHandle"    = "cosmos_operation_handle_t"
  "Completion"         = "cosmos_completion_t"
  "CosmosError"        = "cosmos_error_t"
  "CosmosErrorCode"    = "cosmos_error_code_t"
  ```

- Restrict the items cbindgen exports to those explicitly defined in this spec (`export.item_types = ["functions", "constants", "enums", "structs", "opaque", "typedefs"]` plus an `include_list` / `exclude` policy). Driver-internal types such as `CosmosStatus`, `SubStatusCode`, `ResponseHeaders`, etc. must **not** leak into the generated header — they are surfaced (where needed) through explicit wrapper-defined accessors with `cosmos_*` names.

- Keep `rename_variants = "QualifiedScreamingSnakeCase"` for enums (e.g. `COSMOS_COMPLETION_OUTCOME_OK`, `COSMOS_READ_CONSISTENCY_SESSION`).

A CI check should diff the regenerated header against `include/azurecosmosdriver.h` and fail the build on any unrenamed `cosmos_cosmos_*`-style identifier, any missing `_t` suffix on the types above, or any newly-exported driver-internal type. This is what keeps §4's surface inventory and the actual ABI in sync over time.

---

## 3. Core FFI Patterns

Several of these patterns are inherited from earlier work — but the inheritance is *not* uniform, and this section calls out the lineage explicitly so reviewers can trace what is new vs. what is established prior art:

- **From PR [#2906](https://github.com/Azure/azure-sdk-for-rust/pull/2906) (deleted `azure_data_cosmos_native` bootstrap)** — `c_str!` macro, `BUILD_IDENTIFIER` env var pattern, cbindgen-at-build with `.gitignore`d header (this crate **reverses** that to check the header in — see §5.1), `package.name` / `[lib].name` split, MPL-2.0 entry in `deny.toml`, CMake + Corrosion bootstrap.
- **From PR #3347 and follow-ups** — `RuntimeContext` shape and ownership, the `cosmos_error_code_t` value-range layout (FFI / Cosmos / plumbing bands), `cosmos_string_free` / `cosmos_bytes_free`, the `cosmos_*` symbol prefix convention, the `_t` suffix on type names, the `_unused: u8` placeholder pattern (which this crate **drops** — see §7). The `CallContext` thread-affine error slot from #3347 is **superseded** in this crate by the completion-queue invocation model in §3.1 / §3.6 — errors now flow through completion records, not a per-thread context.
- **New in this crate** — every `cosmos_*` API documented in §3.5 and §4 below, the cbindgen `export.rename` / `item_types` policy from §2.2, the rich `cosmos_error_t` accessor + predicate surface from §3.5.2, the operation factory / mutator surface from §4.6, the driver cache normative documentation from §4.4.1, the ABI version major-equal / minor-≥ rule from §7.

When this spec says "inherited from the original wrapper" elsewhere, it means PR #2906 specifically. Patterns introduced in later PRs are called out by PR number at the point of use.

### 3.1 Invocation model — completion queues

> **Visual overview:** see [`ASYNC_INVOCATION_ARCHITECTURE.md`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/ASYNC_INVOCATION_ARCHITECTURE.md) for the picture-first walkthrough of every flow described below (component layout, submission lifecycle, two-handle ownership, cancellation, queue states, per-language pinning).

Every operation that touches the network in this wrapper is **asynchronous and non-blocking at the FFI boundary**. Submitting a request returns a lightweight in-flight handle (`cosmos_operation_handle_t`); the response (or error) is delivered later on a caller-owned **completion queue** (`cosmos_cq_t`). Host SDKs spin one or more "receive loop" threads that wait on a queue, dequeue completed operations, and dispatch them — typically by carrying the host-language continuation (e.g. a .NET `TaskCompletionSource`) through the `void *user_data` field round-trip.

This is a deliberate change from PR #3347's `CallContext` model (synchronous return + thread-affine last-error slot). The completion-queue model:

- **Avoids blocking host threads** inside Tokio. The host calls `cosmos_*_submit(...)` and returns immediately; Tokio drives the operation; the completion lands on a queue the host reads from.
- **Eliminates the per-thread CallContext slot** as a synchronization point. The `user_data` round-trip is the per-call identity, so each in-flight operation carries its own continuation correlator — no thread affinity required.
- **Maps directly to host async machinery.** .NET → `TaskCompletionSource`; Java → `CompletableFuture`; Python → `asyncio.Future`; native C → manual event loop. The host's receive-loop thread is the only place the wrapper needs to surface results back to host code.

Canonical host-SDK call sites follow the same pattern in every language: allocate a host-language continuation, pin it across the FFI boundary as `user_data`, submit, and translate the eventual completion back into the host's async primitive on a dedicated receive-loop thread.

**.NET (`TaskCompletionSource`):**

```csharp
public Task<CosmosResponse> ReadItemAsync(...) {
    var tcs = new TaskCompletionSource<CosmosResponse>(TaskCreationOptions.RunContinuationsAsynchronously);
    var ud  = (IntPtr)GCHandle.Alloc(tcs);
    var op  = NativeMethods.cosmos_driver_submit(driver, opBuilder, options, queue, ud, out var preErr);
    if (op == IntPtr.Zero) {
        GCHandle.FromIntPtr(ud).Free();
        throw TranslatePreflight(preErr);
    }
    // Free the operation handle immediately — this caller does not retain it
    // for cancel / poll. The receive loop can still borrow the handle via
    // cosmos_completion_op_handle for diagnostics. A host SDK that wants to
    // expose CancellationToken integration would instead stash `op` alongside
    // `tcs` and free it after the completion is observed (see Go example).
    NativeMethods.cosmos_operation_handle_free(op);
    return tcs.Task;
}

// On a dedicated receive thread:
while (!shutdown) {
    var c = NativeMethods.cosmos_cq_wait(queue, uint.MaxValue);
    if (c == IntPtr.Zero) continue;
    var ud  = NativeMethods.cosmos_completion_user_data(c);
    var tcs = (TaskCompletionSource<CosmosResponse>)GCHandle.FromIntPtr(ud).Target;
    switch (NativeMethods.cosmos_completion_outcome(c)) {
        case OK:        tcs.SetResult(WrapResponse(NativeMethods.cosmos_completion_take_response(c))); break;
        case ERROR:     tcs.SetException(WrapError(NativeMethods.cosmos_completion_take_error(c)));     break;
        case CANCELLED: tcs.SetCanceled();                                                              break;
    }
    GCHandle.FromIntPtr(ud).Free();
    NativeMethods.cosmos_completion_free(c);
}
```

**Java (JEP 442 panama `Linker` + `CompletableFuture`):** the per-call continuation is the `CompletableFuture` itself; pinning is done by stashing the future in a concurrent `Long → CompletableFuture` map keyed by a monotonic correlator (Java has no equivalent of `GCHandle.ToIntPtr`, so we round-trip an integer ticket instead of an object pointer).

```java
private final AtomicLong nextTicket = new AtomicLong(1);
private final ConcurrentHashMap<Long, CompletableFuture<CosmosResponse>> inflight = new ConcurrentHashMap<>();

public CompletableFuture<CosmosResponse> readItemAsync(...) {
    var future = new CompletableFuture<CosmosResponse>();
    long ticket = nextTicket.getAndIncrement();
    inflight.put(ticket, future);
    try (var arena = Arena.ofConfined()) {
        MemorySegment preErr = arena.allocate(ValueLayout.JAVA_INT);
        MemorySegment op = (MemorySegment) cosmos_driver_submit.invokeExact(
            driver, opBuilder, options, queue, MemorySegment.ofAddress(ticket), preErr);
        if (op.equals(MemorySegment.NULL)) {
            inflight.remove(ticket);
            throw translatePreflight(preErr.get(ValueLayout.JAVA_INT, 0));
        }
        // Same pattern as the .NET example: this caller does not retain the
        // handle, so free it immediately. To support CompletableFuture.cancel
        // propagation, stash `op` in the inflight map alongside the future
        // and free it from the receive loop after the completion lands.
        cosmos_operation_handle_free.invokeExact(op);
    }
    return future;
}

// Dedicated receive thread (one per cosmos_cq_t):
while (!shutdown) {
    MemorySegment c = (MemorySegment) cosmos_cq_wait.invokeExact(queue, Integer.MAX_VALUE);
    if (c.equals(MemorySegment.NULL)) continue;
    long ticket = ((MemorySegment) cosmos_completion_user_data.invokeExact(c)).address();
    CompletableFuture<CosmosResponse> future = inflight.remove(ticket);
    int outcome = (int) cosmos_completion_outcome.invokeExact(c);
    switch (outcome) {
        case OK        -> future.complete(wrapResponse((MemorySegment) cosmos_completion_take_response.invokeExact(c)));
        case ERROR     -> future.completeExceptionally(wrapError((MemorySegment) cosmos_completion_take_error.invokeExact(c)));
        case CANCELLED -> future.cancel(false);
        default        -> future.completeExceptionally(new IllegalStateException("unknown outcome " + outcome));
    }
    cosmos_completion_free.invokeExact(c);
}
```

The ticket-map indirection is the idiomatic Java pattern because JNI / panama do not pin Java references across native calls and Java has no managed-pointer concept the runtime can hand the wrapper. Performance is equivalent — one extra hashmap lookup per completion.

**Go (`cgo` + channels):** the continuation is a per-call channel that the receive goroutine writes once. Like Java, Go pins via a `uintptr` ticket stored in a sync map rather than passing a Go pointer across cgo (Go's checkptr rules forbid handing the C side a live `*T` whose lifetime crosses cgo boundaries).

```go
type completion struct {
    resp *C.cosmos_response_t
    err  error
}

var (
    nextTicket atomic.Uint64
    inflight   sync.Map // map[uintptr]chan completion
)

func (d *Driver) ReadItemAsync(ctx context.Context, op *Operation, opts *Options) (*Response, error) {
    ch := make(chan completion, 1)
    ticket := uintptr(nextTicket.Add(1))
    inflight.Store(ticket, ch)

    var preErr C.cosmos_error_code_t
    handle := C.cosmos_driver_submit(d.raw, op.raw, opts.raw, d.queue, unsafe.Pointer(ticket), &preErr)
    if handle == nil {
        inflight.Delete(ticket)
        return nil, translatePreflight(preErr)
    }
    defer C.cosmos_operation_handle_free(handle)

    select {
    case c := <-ch:
        return wrapResponse(c.resp), c.err
    case <-ctx.Done():
        C.cosmos_operation_handle_cancel(handle)
        c := <-ch // still wait for the completion record so we can free it
        if c.resp != nil { C.cosmos_response_free(c.resp) }
        return nil, ctx.Err()
    }
}

// Dedicated receive goroutine (one per cosmos_cq_t):
for !shutdown.Load() {
    c := C.cosmos_cq_wait(d.queue, math.MaxUint32)
    if c == nil { continue }
    ticket := uintptr(C.cosmos_completion_user_data(c))
    chAny, _ := inflight.LoadAndDelete(ticket)
    ch := chAny.(chan completion)

    switch C.cosmos_completion_outcome(c) {
    case C.COSMOS_COMPLETION_OUTCOME_OK:
        ch <- completion{resp: C.cosmos_completion_take_response(c)}
    case C.COSMOS_COMPLETION_OUTCOME_ERROR:
        ch <- completion{err: wrapError(C.cosmos_completion_take_error(c))}
    case C.COSMOS_COMPLETION_OUTCOME_CANCELLED:
        ch <- completion{err: context.Canceled}
    }
    C.cosmos_completion_free(c)
}
```

Go's `context.Context` integration is the key benefit of the cancel path — propagating a caller's `ctx.Done()` into `cosmos_operation_handle_cancel` requires no special wrapper support, since the handle is just a `uintptr` the goroutine retains until it has drained the completion. The same pattern fits Python (`asyncio.Future` + ticket map driven by a dedicated thread that bridges into the event loop via `loop.call_soon_threadsafe`) and Node.js (`Promise` resolvers + `napi_async_work` for the receive loop).

#### 3.1.1 Types

Both types below are **opaque** to the C ABI — consumers receive `cosmos_*_t *` pointers and use accessor functions. Publishing a concrete struct layout would freeze G4's ABI-stability promise the first time either type grows a field.

```c
typedef struct cosmos_runtime          cosmos_runtime_t;
typedef struct cosmos_cq               cosmos_cq_t;
typedef struct cosmos_operation_handle cosmos_operation_handle_t;
typedef struct cosmos_completion       cosmos_completion_t;
```

- `cosmos_runtime_t` owns the async runtime (Tokio by default) **plus a strong reference to a shared `CosmosDriverRuntime`** (see §4.1). One per process is typical.
- `cosmos_cq_t` is a multi-producer / single-consumer completion queue. Multiple submissions from multiple threads can target the same queue; **only one thread at a time** should call `cosmos_cq_wait` on a given queue. Host SDKs that want work-stealing across multiple consumer threads should create one queue per consumer, not one queue shared by all consumers — the wrapper does not coordinate cross-thread fairness inside a single queue. (See §9 Q12 for whether a multi-consumer mode will ever be added.)
- `cosmos_operation_handle_t` is the in-flight identity of a submitted operation. The caller can use it to (a) request cancellation, (b) snapshot diagnostics mid-flight, and (c) correlate to the eventually-delivered completion. It is **not** the place the response is delivered — that's the completion record.
- `cosmos_completion_t` is a single dequeued record — it pairs the caller's `user_data` with the response or error. See §3.6 for the full surface.

#### 3.1.2 Lifecycle

```c
typedef struct cosmos_cq_options {
    /* Capacity hint — soft upper bound on pending completions queued before
     * the wrapper emits a one-shot diagnostic warning. 0 = use default
     * (currently 1024). Submissions never fail because the soft hint is
     * exceeded; the queue simply grows past it. */
    uint32_t capacity_hint;

    /* Hard capacity — when non-zero, submits that would push the queue past
     * `max_capacity` pending completions are rejected pre-flight with
     * COSMOS_ERROR_CODE_QUEUE_FULL (4013). Use `cosmos_cq_wait_writable` (see
     * §3.1.3) to block until space frees, or treat 4013 as the host SDK's
     * back-pressure signal. 0 = unbounded (the default) — appropriate for
     * latency-sensitive workloads where the host SDK already throttles
     * upstream. Bulk-import / fan-out workloads SHOULD set a hard cap to
     * avoid unbounded memory growth on a stuck consumer. */
    uint32_t max_capacity;

    /* When true, completion records include the rich cosmos_error_t payload
     * on failure (see §3.5.2). When false, only the coarse
     * cosmos_error_code_t is set and the rich payload is NULL. Defaults to true. */
    bool include_error_details;
} cosmos_cq_options_t;

/* Lifecycle. The runtime must outlive every queue created against it. */
cosmos_cq_t *cosmos_cq_create(const cosmos_runtime_t *runtime,
                              const cosmos_cq_options_t *options /* nullable */);
void         cosmos_cq_free(cosmos_cq_t *queue);   /* NULL is a no-op */

/* Returns the runtime the queue was bound to (borrowed; lifetime = the queue). */
const cosmos_runtime_t *cosmos_cq_runtime(const cosmos_cq_t *queue);
```

**Freeing a queue with operations still in-flight** is a programming error: `cosmos_cq_free` will block until all in-flight submissions targeting that queue have completed (cancelling each one first). Host SDKs that need a non-blocking shutdown must call `cosmos_cq_shutdown` and drain via `cosmos_cq_wait` until `cosmos_cq_state` returns `DRAINED`, then `_free`. See §3.6.4.

#### 3.1.3 Waiting for completions

```c
/* Block until a completion is available or `timeout_ms` elapses.
 *
 *   timeout_ms == 0          → poll (return immediately, possibly NULL)
 *   timeout_ms == UINT32_MAX → wait forever (only returns NULL on shutdown / spurious wake)
 *   otherwise                → wait up to that many milliseconds
 *
 * Returns:
 *   - non-NULL cosmos_completion_t* on a delivered completion; caller MUST
 *     free it via cosmos_completion_free.
 *   - NULL on timeout, queue shutdown, drained queue, or spurious wake.
 *     Distinguish via cosmos_cq_state(queue).
 */
cosmos_completion_t *cosmos_cq_wait(cosmos_cq_t *queue, uint32_t timeout_ms);

/* Non-blocking poll. Equivalent to cosmos_cq_wait(queue, 0). */
cosmos_completion_t *cosmos_cq_try_wait(cosmos_cq_t *queue);

/* Batched wait — drains up to `max_count` completions in a single call.
 *
 * Blocks until at least one completion is available or `timeout_ms` elapses,
 * then opportunistically drains additional completions that are already
 * queued without blocking again. Returns the number of completions written
 * into `out_completions[0..max_count]`; the caller MUST free each one via
 * cosmos_completion_free.
 *
 * Returns 0 on timeout / shutdown / drained (distinguish via cosmos_cq_state).
 * Recommended for single-consumer receive loops at high throughput — one
 * wait/wake cycle amortizes the cost of N completions, eliminating the
 * per-op syscall overhead of cosmos_cq_wait. `max_count` of 32-256 is
 * typical; values above ~1024 see diminishing returns. */
uint32_t cosmos_cq_wait_batch(cosmos_cq_t *queue,
                              cosmos_completion_t **out_completions,
                              uint32_t max_count,
                              uint32_t timeout_ms);

/* Block until the queue has room for at least one more pending completion,
 * or `timeout_ms` elapses. Only meaningful when the queue was created with
 * cosmos_cq_options.max_capacity > 0; for an unbounded queue this returns
 * true immediately.
 *
 * Returns:
 *   true   — space is available (best-effort: another producer may consume
 *            it before the caller submits, in which case a subsequent submit
 *            still returns QUEUE_FULL — treat this as a hint, not a lock).
 *   false  — timeout, shutdown, or drained queue.
 *
 * Host SDKs that want strict back-pressure typically loop:
 *   while (!cosmos_cq_wait_writable(q, 1000)) { check_shutdown(); }
 *   handle = cosmos_driver_submit(...);
 *   if (handle == NULL && pre_err == QUEUE_FULL) continue; // retry
 */
bool cosmos_cq_wait_writable(cosmos_cq_t *queue, uint32_t timeout_ms);

/* Signal shutdown: in-flight ops are cancelled and any thread blocked in
 * cosmos_cq_wait wakes with NULL. Idempotent. After shutdown, no further
 * submissions targeting this queue succeed (they fail pre-flight with
 * COSMOS_ERROR_CODE_QUEUE_SHUTDOWN). Pending completions can still be drained
 * via cosmos_cq_wait until empty. */
void cosmos_cq_shutdown(cosmos_cq_t *queue);

/* Queue state — used to distinguish wait-returned-NULL reasons. */
typedef enum cosmos_cq_state {
    COSMOS_CQ_STATE_RUNNING   = 0,
    COSMOS_CQ_STATE_SHUTDOWN  = 1,   /* shutdown requested; may still have completions to drain */
    COSMOS_CQ_STATE_DRAINED   = 2,   /* shutdown + queue empty + no in-flight ops */
} cosmos_cq_state_t;
cosmos_cq_state_t cosmos_cq_state(const cosmos_cq_t *queue);
```

**Threading rules:**

1. A single queue is **multi-producer**: any thread holding a `cosmos_cq_t*` may submit to it.
2. A single queue is **single-consumer**: only one thread at a time should call `cosmos_cq_wait` / `_try_wait`. The wrapper does not enforce this in v1 (no internal lock around the consumer side); calling from two threads simultaneously is undefined behavior. See §9 Q12 for promoting to MPMC in a future revision.
3. `cosmos_cq_shutdown` is safe to call from any thread.

### 3.2 Function signature templates

The wrapper has **two** call shapes — synchronous for builders and pure accessors, and asynchronous submit-then-complete for everything that touches the network.

**Pattern A — synchronous (builders, accessors, free, clone):**

```c
cosmos_error_code_t cosmos_<noun>_<verb>(
    /* required handle(s) */,
    /* required scalars */,
    const cosmos_<noun>_<verb>_options_t *options,  /* nullable */
    /* out parameters: out_handle, out_bytes, out_len, … */
);
```

Used by: account / database / container / partition-key / feed-range / operation **builders** (no network), response and diagnostics **accessors** (in-memory only), `_free` functions, `_clone` functions. Same contract as the old wrapper: return value is a coarse status code, outputs go through `out_*` pointers, `options == NULL` means "use defaults". There is no `cosmos_call_context_t` argument — pre-flight rejections (NULL handle, bad UTF-8, etc.) are returned directly via the status code, and any rich error is written into an optional `cosmos_error_t *out_error` slot the function lists explicitly.

**Pattern B — asynchronous submit (network-touching APIs):**

```c
cosmos_operation_handle_t *cosmos_<noun>_<verb>_submit(
    const cosmos_driver_t *driver,                  /* or runtime, for driver_get_or_create */
    /* required handle(s) */,
    /* required scalars */,
    const cosmos_<noun>_<verb>_options_t *options,  /* nullable */
    cosmos_cq_t *queue,                             /* where the completion will land */
    void *user_data,                                /* opaque correlator, round-tripped verbatim */
    cosmos_error_code_t *out_pre_error);            /* synchronous pre-flight error slot, nullable */
```

Behavior:

- On successful **submission** (the request was accepted and will eventually post a completion to `queue`): returns a non-NULL `cosmos_operation_handle_t*` and writes `COSMOS_ERROR_CODE_SUCCESS` to `*out_pre_error` (when non-NULL).
- On **pre-flight FFI rejection** (NULL required handle, bad UTF-8 in a string arg, queue shut down, operation already consumed, etc.): returns `NULL` and writes a coarse code to `*out_pre_error`. **No completion is ever posted in this case** — the caller doesn't need to wait on the queue for it.
- On any **runtime / service / transport error** detected after the submission was accepted: the operation handle is returned successfully, and the failure is delivered later as a completion with `outcome = ERROR` and a populated `cosmos_error_t`.

Pre-flight rejections never touch the network and never enqueue anything; runtime failures always go through the completion queue. Host SDKs that want to surface pre-flight failures the same way as completed ones simply translate `*out_pre_error` into the language-native exception path inside the submit wrapper (e.g. `tcs.SetException(...)` before returning from `ReadItemAsync`).

`user_data` is a host-owned opaque pointer. The wrapper performs **no** lifecycle management on it — it stores the value verbatim and round-trips it to the completion record. Typical use: `GCHandle.ToIntPtr(GCHandle.Alloc(tcs))` in .NET; `Box::into_raw(Box::new(...))` from another Rust caller; a `malloc`'d struct in C. Freeing the `user_data` is the caller's job in the receive loop (after dispatching the continuation).

### 3.3 Bytes marshalling (new)

Because the driver is schema-agnostic, request/response bodies are raw bytes, not C strings. The wrapper exposes two distinct types — a **view-by-value** for caller-owned inputs and an **opaque handle** for SDK-owned outputs:

```c
// Caller-owned input: caller keeps memory live for the duration of the
// synchronous FFI call that consumes the view. Any function that needs the
// bytes to outlive the call (notably submit-into-async paths like
// cosmos_operation_with_body — see §4.6.2) MUST document that it copies the
// view's contents into wrapper-owned storage before returning. The default
// rule "caller may release the source memory immediately after the call
// returns SUCCESS" therefore holds uniformly across both Pattern A (§3.2)
// synchronous calls AND Pattern B async submits — even though the network
// I/O for a submit happens later, the input bytes have already been copied
// in by the time submit returns. See §9 Q17 for the open question on
// reintroducing zero-copy borrow-until-completion semantics behind a
// driver-owned buffer pool.
//
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
| `cosmos_runtime_t*` | `cosmos_runtime_builder_build` | `cosmos_runtime_free` | No (use one per process; see §4.1) |
| `cosmos_driver_t*` | `cosmos_driver_get_or_create` | `cosmos_driver_free` | Internally `Arc`; FFI handle is a single owner |
| `cosmos_cq_t*` | `cosmos_cq_create(runtime, options)` | `cosmos_cq_free` (blocks until drained — call `cosmos_cq_shutdown` first for non-blocking) | No |
| `cosmos_operation_handle_t*` | every `cosmos_*_submit` | `cosmos_operation_handle_free` (independent of completion lifetime — see §3.6.2) | No |
| `cosmos_completion_t*` | `cosmos_cq_wait` / `cosmos_cq_try_wait` | `cosmos_completion_free` (response/error obtained via `_take_*` remain owned by caller) | No |
| `cosmos_account_ref_t*` | `cosmos_account_ref_with_*` | `cosmos_account_ref_free` | Yes, via `cosmos_account_ref_clone` (cheap; new strong handle to the same `Arc`) |
| `cosmos_database_ref_t*` / `cosmos_container_ref_t*` | `cosmos_*_ref_create` from parent | matching `_free` | Yes, via `cosmos_*_ref_clone` (cheap) |
| `cosmos_partition_key_t*` | `cosmos_partition_key_builder_build` / `cosmos_partition_key_from_string` | `cosmos_partition_key_free` | Yes, via `cosmos_partition_key_clone` (cheap) |
| `cosmos_feed_range_t*` | `cosmos_feed_range_full` / `cosmos_feed_range_for_partition_key` | `cosmos_feed_range_free` | Yes, via `cosmos_feed_range_clone` (cheap; small `enum FeedRange` copy) |
| `cosmos_operation_t*` | `cosmos_operation_*` factory | `cosmos_operation_free` (always safe — see §4.6 "Execute consumption" subsection) | No (move semantics on `execute`) |
| `cosmos_response_t*` | `cosmos_completion_take_response` | `cosmos_response_free` | No |
| `cosmos_bytes_t*` | `cosmos_response_into_body` / `cosmos_diagnostics_to_json` | `cosmos_bytes_free(bytes)` (single-owner heap allocation; see §3.3) | No (the underlying `bytes::Bytes` is internally refcounted but the FFI handle is single-owner) |
| `cosmos_diagnostics_t*` | `cosmos_response_diagnostics` / `cosmos_error_diagnostics` | `cosmos_diagnostics_free` (drops `Arc`) | Internally `Arc`; each accessor returns a new strong handle the caller must free |
| `cosmos_error_t*` | `cosmos_completion_take_error` (async failures), or a synchronous factory `out_error` slot (§4.3 / §4.5) | `cosmos_error_free` (borrowed via `cosmos_completion_error` is owned by the completion — do NOT free) | No |

`cosmos_driver_t` is **the** unit of cardinality. Each call to `cosmos_driver_get_or_create` for the same account endpoint returns the same underlying driver instance (the runtime caches them — see the cache-key discussion in §4.4). The FFI handle, however, is a distinct `Box<Arc<CosmosDriver>>` — freeing it only drops one `Arc` strong count.

**Cloning is a refcount bump, not a deep copy.** The `_clone` functions on reference / partition-key / feed-range handles allocate a fresh FFI handle that aliases the same underlying `Arc<…>` (where one exists) or copies a small `Vec<PartitionKeyValue>` / `enum FeedRange` (for partition keys and feed ranges). Cloning never touches the network. Every successful `_clone` must be paired with a matching `_free`.

**Immutability and thread-safety of value handles.** `cosmos_account_ref_t`, `cosmos_database_ref_t`, `cosmos_container_ref_t`, `cosmos_partition_key_t`, and `cosmos_feed_range_t` are **immutable post-build**. The wrapper exposes no mutator on these handles after the corresponding builder produced them. As a result, `_clone`, accessor reads, and `_free` of *distinct* FFI handles are race-free across threads without external locking — even when the clones alias the same underlying `Arc`. Two threads must still not concurrently `_free` the **same** FFI handle (that's a double-free, not a race).

### 3.5 Error model

The wrapper's error surface is built on two complementary types — a coarse `cosmos_error_code_t` numeric return value for the C function contract, and a rich `cosmos_error_t` payload that mirrors the driver's `azure_data_cosmos::Error` (introduced in [#4442](https://github.com/Azure/azure-sdk-for-rust/pull/4442)). Both **must** be exposed because the host SDKs sitting on top of this wrapper need full error fidelity for **diagnosability** and for **routing failure classes into language-native exception types** — they do **not** re-implement retry / throttling / conditional-write recovery (that's the driver's responsibility, by design — see [`ARCHITECTURE.md`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/ARCHITECTURE.md) "Schema-Agnostic Data Plane"). Concretely:

- **Diagnosability.** `400 Bad Request` is the canonical example: callers cannot debug it without the gateway response body, headers (`x-ms-activity-id`, `x-ms-substatus`), and the driver's `DiagnosticsContext` for the failed attempt. The rich payload exposes all three.
- **Failure-class routing.** Host SDKs translate `cosmos_error_is_not_found(e)` / `_is_conflict(e)` / `_is_precondition_failed(e)` / `_is_throttled(e)` etc. into their language-native exceptions (`CosmosException` subclasses in Java, dedicated error variants in Go, `CosmosException.StatusCode` in .NET). Routing is **classification**, not retry.
- **What host SDKs do NOT do.** They do **not** drive retry loops, back-off timers, conditional-write recovery, or cross-region failover off the wrapper's error surface — those are owned by the driver's pipeline (`transport_pipeline.rs`, the throttle / failover / circuit-breaker components, and the `OperationOptions` retry knobs). A host SDK that re-implements any of these on top of the wrapper is defeating the whole point of the driver split.

> **Landing prerequisites — read this before implementing.** §3.5.2 below and §6 mirror the **actually merged** shape of [`azure_data_cosmos_driver::error::CosmosError`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/src/error/mod.rs) from PR [#4442](https://github.com/Azure/azure-sdk-for-rust/pull/4442) (now on `main`). Notable departures from earlier drafts of this spec:
>
> - There is **no `Kind` enum** on the merged `CosmosError` — the type is monomorphic. Failure-class taxonomy is encoded entirely through the `CosmosStatus` HTTP status code + optional [`SubStatusCode`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/src/error/cosmos_status.rs) (16-bit, with synthetic values such as `TRANSPORT_GENERATED_503 = 20003`, `CLIENT_OPERATION_TIMEOUT = 20008` for client-side categories). The spec therefore exposes no `cosmos_error_kind_t` enum or `cosmos_error_kind(e)` accessor; host SDKs route on `(status_code, sub_status)` directly. The earlier `COSMOS_ERROR_KIND_*` taxonomy has been removed from this section.
> - **Predicates live on `CosmosStatus`, invoked as `err.status().is_*()`.** The wrapper still exposes them as flat `cosmos_error_is_*(e)` calls for caller ergonomics, but only mirrors the predicates that actually exist on the merged `CosmosStatus`. See §3.5.2 for the exact list.
> - **Header accessors (`activity_id`, `session_token`, `etag`, retry-after) do not live on `CosmosError`.** They are reachable via `err.response().headers()` when a wire response was received. The wrapper exposes them as `cosmos_error_*` convenience accessors that internally walk through `cosmos_error_response(e)`; they return NULL / -1 for non-wire errors (transport, client, configuration).
> - **Backtrace tuning is process-global, not per-runtime / per-driver.** The driver exposes [`error::set_backtrace_options(BacktraceOptions { max_captures_per_second, max_resolutions_per_second })`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/src/error/backtrace.rs) as a free function, not as a `CosmosDriverRuntimeBuilder` / `CosmosDriverBuilder` method. The wrapper therefore exposes a single `cosmos_set_backtrace_options(captures, resolutions)` entry at module scope instead of the per-runtime / per-driver setters earlier drafts described. See §6.4.
>
> Sub-status synthetic codes (`20003`, `20008`, `20010..=20015`, `20020`, `20402`, `20912`, ...) are defined as `pub const` on [`SubStatusCode`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/src/error/cosmos_status.rs); the wrapper re-exports them verbatim through the `COSMOS_SUB_STATUS_*` constants described in §3.5.2.

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
  | 4005 | `OPERATION_CONSUMED` | A mutator (`cosmos_operation_with_*`) or a second submit (`cosmos_driver_submit`) was called after the operation handle was already consumed by an earlier successful submit. (See §4.6.3 "Submission and completion lifecycle".) |
  | 4006 | `RESPONSE_CONSUMED` | `cosmos_response_into_*` called twice on the same response. |
  | 4007 | `FEED_EXHAUSTED` | A single-shot `cosmos_driver_submit` produced `Ok(None)` from the driver — i.e. the call targeted a feed page that had no more data. Use the pager submit in §4.7 to iterate feeds; the code surfaces on the completion's `cosmos_completion_status` (with `outcome = ERROR`) rather than panicking. |
  | 4008 | `PRECONDITION_ALREADY_SET` | A second precondition setter (`with_precondition_if_match` / `with_precondition_if_none_match`) was called on an operation that already has a precondition. The driver's `with_precondition` takes a single `Precondition` enum value, so only one of If-Match / If-None-Match may be set per operation. |
  | 4009 | `UNSUPPORTED_OPERATION_FOR_MUTATOR` | A mutator only meaningful for a specific operation kind (e.g. `with_patch_max_attempts` on a non-patch operation) was rejected at the FFI boundary. |
  | 4010 | `INVALID_HEADER_NAME` / `INVALID_HEADER_VALUE` | A `cosmos_operation_with_request_header` call passed a non-ASCII / control-character header name or value. |
  | 4011 | `QUEUE_SHUTDOWN` | A submit targeted a `cosmos_cq_t` that had already been shut down via `cosmos_cq_shutdown`. Pre-flight rejection — no completion is posted. |
  | 4012 | `OPERATION_CANCELLED` | Surfaced via `cosmos_completion_status` on a completion whose `cosmos_completion_outcome` is `COSMOS_COMPLETION_OUTCOME_CANCELLED`. Triggered by `cosmos_operation_handle_cancel` or by `cosmos_cq_shutdown` cancelling all in-flight ops for the queue. |
  | 4013 | `QUEUE_FULL` | A submit targeted a `cosmos_cq_t` whose hard capacity (`cosmos_cq_options.max_capacity`, when set) is already reached. Pre-flight rejection — no completion is posted. Default queue configuration does **not** set a hard cap; this code only fires when the host SDK opts in. See §3.1.2. |
  | 4014 | `INVALID_OPTION_VALUE` | A builder setter (`cosmos_runtime_builder_with_*`, `cosmos_operation_options_*`, etc.) was passed a value outside the documented range (e.g. negative `worker_threads`, empty thread-name prefix containing NUL, malformed user-agent suffix). The builder is left unchanged. |
  | 4015 | `RUNTIME_BUILD_FAILED` | `cosmos_runtime_builder_build` could not construct the underlying `CosmosDriverRuntime`. Typical causes: HTTP transport TLS init failure, invalid proxy configuration, environment variables out of range. The rich `cosmos_error_t` carries the inner cause. |

  Code `4001` is **reserved** (formerly used for `OPTIONS_IGNORED_ON_CACHE_HIT`, which was moved to the `5xxx` warning class — see below — once the SUCCESS-plus-populated-error pattern was rejected). `4001..=4999` is otherwise reserved for additive growth; consumers must treat unknown `4xxx` codes as fatal but recoverable (i.e. log + propagate) rather than panic.

- `5001..=5999` — **non-fatal warnings.** A `5xxx` return is **not** `SUCCESS`; `out_*` pointers are **populated** (the call did the work) and the rich `cosmos_error_t` is populated as advisory detail. Host SDKs that follow the convention `if (code != SUCCESS) handle_error();` will safely treat warnings as failures by default. Host SDKs that want to opt into the advisory treat the warning explicitly. There is **no** "success with populated error" return pattern in this ABI.

  | Code | Variant | Meaning |
  |---|---|---|
  | 5001 | `OPTIONS_IGNORED_ON_CACHE_HIT` | `cosmos_driver_get_or_create` was called with non-NULL `options` while a driver for the same account endpoint was already cached. **`out_driver` is populated with the cached instance.** The passed `options` were dropped. Treat as fatal (host SDK rejects mismatched options) or ignore (host SDK accepts cached instance) per local policy. See §4.4.1. |

  `5001..=5999` is reserved for additive growth; consumers must treat unknown `5xxx` codes the same way: `out_*` populated, warning details on the rich error.

The wrapper **must not** invent 4xxx codes for things that already correspond to a `cosmos_error_t::kind()` — those go through the rich error type instead.

#### 3.5.2 `cosmos_error_t` (rich payload, mirrors `azure_data_cosmos_driver::error::CosmosError`)

The driver's `CosmosError` ([`azure_data_cosmos_driver::error::CosmosError`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/src/error/mod.rs)) carries structured information host SDKs need for correct retry / throttle / conditional-write handling. The wrapper mirrors that surface 1:1 through accessor functions on `cosmos_error_t`:

```c
/* Status / categorical accessors. `cosmos_error_status_code(e)` is always
 * populated, including for synthetic client-side errors (which carry
 * placeholder HTTP codes such as 503 via CosmosStatus::TRANSPORT_GENERATED_503).
 * `cosmos_error_sub_status(e)` returns -1 when no sub-status is present,
 * otherwise the SubStatusCode value (driver-side u16, surfaced as int32_t so
 * the -1 sentinel is safe). */
uint16_t cosmos_error_status_code(const cosmos_error_t *e);
int32_t  cosmos_error_sub_status(const cosmos_error_t *e);

/* True iff the error originated from a service wire response (status() carries
 * an Authentic HTTP response, response() is Some). Mirrors
 * `CosmosError::is_from_wire`. Use this to distinguish a 404 the gateway
 * returned (true) from a synthetic 404 the client created (false). */
bool cosmos_error_is_from_wire(const cosmos_error_t *e);

/* Message — borrowed UTF-8, valid until the error is freed. */
const char *cosmos_error_message(const cosmos_error_t *e);

/* Borrowed access to the wire response that produced this error, or NULL when
 * `is_from_wire(e) == false`. Lifetime = until cosmos_error_free. The
 * convenience header / body accessors below walk through this same pointer. */
const cosmos_response_t *cosmos_error_response(const cosmos_error_t *e);

/* Convenience accessors that walk through `cosmos_error_response(e)` so host
 * SDKs do not have to dereference the response handle for the four
 * highest-traffic headers. Each returns NULL (or -1 for retry-after) when
 * either:
 *   • the error has no wire response (Transport / Client / Configuration); or
 *   • the wire response did not carry that header.
 * Borrowed UTF-8, valid until the error is freed. */
const char *cosmos_error_activity_id(const cosmos_error_t *e);
const char *cosmos_error_session_token(const cosmos_error_t *e);
const char *cosmos_error_etag(const cosmos_error_t *e);
int64_t     cosmos_error_retry_after_ms(const cosmos_error_t *e);

/* Raw service-error response body, sourced from `cosmos_error_response(e)`.
 * Returns {NULL, 0} when there is no wire response. */
cosmos_bytes_view_t cosmos_error_response_body(const cosmos_error_t *e);

/* Diagnostics for the request that produced this error. Returns a NEW handle
 * that must be freed via cosmos_diagnostics_free. Returns NULL when no
 * diagnostics were attached (e.g. some Configuration / construction-time
 * errors). Mirrors `CosmosError::diagnostics`. */
cosmos_diagnostics_t *cosmos_error_diagnostics(const cosmos_error_t *e);

/* Backtrace — rate-limited rendered string (Option<Arc<str>> on the Rust
 * side). Returns NULL when no backtrace was captured (e.g. capture budget
 * exhausted, or backtraces disabled). Capture / rendering rate is bounded by
 * `cosmos_set_backtrace_options` — see §6.4. */
const char *cosmos_error_backtrace(const cosmos_error_t *e);

/* Predicates — flat namespace on cosmos_error_t for caller ergonomics. Each
 * forwards internally to the corresponding `err.status().is_*()` method on
 * [`CosmosStatus`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/src/error/cosmos_status.rs). The set below mirrors the
 * predicates actually implemented on the merged CosmosStatus; consult that
 * file for the exact decision rules per status / sub-status combination. */
bool cosmos_error_is_success(const cosmos_error_t *e);              /* status in 2xx */
bool cosmos_error_is_throttled(const cosmos_error_t *e);            /* 429 */
bool cosmos_error_is_not_found(const cosmos_error_t *e);            /* 404 */
bool cosmos_error_is_conflict(const cosmos_error_t *e);             /* 409 */
bool cosmos_error_is_precondition_failed(const cosmos_error_t *e);  /* 412 */
bool cosmos_error_is_timeout(const cosmos_error_t *e);              /* 408 + CLIENT_OPERATION_TIMEOUT */
bool cosmos_error_is_gone(const cosmos_error_t *e);                 /* 410 */
bool cosmos_error_is_bad_request(const cosmos_error_t *e);          /* 400 */
bool cosmos_error_is_unauthorized(const cosmos_error_t *e);         /* 401 */
bool cosmos_error_is_forbidden(const cosmos_error_t *e);            /* 403 */
bool cosmos_error_is_service_unavailable(const cosmos_error_t *e);  /* 503 (excluding transport-generated) */
bool cosmos_error_is_transient(const cosmos_error_t *e);            /* 408 / 429 / 503 family */
bool cosmos_error_is_write_forbidden(const cosmos_error_t *e);              /* 403 / WRITE_FORBIDDEN */
bool cosmos_error_is_read_session_not_available(const cosmos_error_t *e);   /* 404 / READ_SESSION_NOT_AVAILABLE */
bool cosmos_error_is_partition_key_range_gone(const cosmos_error_t *e);     /* 410 / PARTITION_KEY_RANGE_GONE */
bool cosmos_error_is_transport_generated_503(const cosmos_error_t *e);      /* synthetic 503 / TRANSPORT_GENERATED_503 */

/* Free a cosmos_error_t obtained from cosmos_completion_take_error or from a
 * synchronous `out_error` slot. Errors borrowed from a completion via
 * cosmos_completion_error are owned by the completion and freed with it — do
 * NOT call this on borrowed pointers. */
void cosmos_error_free(cosmos_error_t *e);
```

**Where `cosmos_error_t` is populated.** Errors flow through two paths depending on the call shape that produced them:

- **Async submit (Pattern B in §3.2).** Runtime / service / transport / authentication failures are delivered as a completion record with `cosmos_completion_outcome == ERROR`. The rich payload is retrieved via `cosmos_completion_take_error(c)` (transfers ownership; caller must `cosmos_error_free`) or borrowed via `cosmos_completion_error(c)` (lifetime = until `cosmos_completion_free`). Population is controlled **per queue** by `cosmos_cq_options.include_error_details` (default `true`); host SDKs that only care about the coarse code can disable rich capture for a small per-completion allocation saving.
- **Pre-flight / synchronous (Pattern A in §3.2).** For non-network APIs (factories in §4.3 / §4.5, accessors, builders) the caller passes an `cosmos_error_t *out_error` slot. For submit Pattern B's `*out_pre_error`, only the coarse `cosmos_error_code_t` is written — rich detail for pre-flight rejections is intentionally minimal and surfaces via `cosmos_error_message` if at all.

**Wrapper does NOT construct `cosmos_error_t`.** Errors are only ever *received* from the driver; no `cosmos_error_create_*` API is exposed.

**Synthetic sub-status codes** for client-side / transport / serialization failures are surfaced verbatim through `cosmos_error_sub_status` — the wrapper does not re-number them. Authoritative names + values live on [`SubStatusCode`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/src/error/cosmos_status.rs) in the driver. The currently-defined `20xxx` set is:
>
> - `TRANSPORT_GENERATED_503 = 20003` (transport-synthesized 503 in the response pipeline)
> - `CLIENT_OPERATION_TIMEOUT = 20008` (end-to-end deadline exceeded on the client)
> - `TRANSPORT_CONNECTION_FAILED = 20010`
> - `TRANSPORT_IO_FAILED = 20011`
> - `TRANSPORT_DNS_FAILED = 20012`
> - `TRANSPORT_BODY_READ_FAILED = 20014`
> - `TRANSPORT_HTTP2_INCOMPATIBLE = 20015`
> - `SERIALIZATION_RESPONSE_BODY_INVALID = 20020`
> - `AUTHENTICATION_TOKEN_ACQUISITION_FAILED = 20402`
>
> Plus the `20912` "closed client" value the driver currently displays without a `pub const` (tracked in §9 Q17 below as a driver-side cleanup before the wrapper re-exports it). Earlier drafts of this spec listed `TRANSPORT_REQUEST_TIMEOUT`, `TRANSPORT_RESPONSE_TIMEOUT`, `TRANSPORT_RESPONSE_BODY_FAILED`, `TRANSPORT_TLS_FAILURE`, `SERIALIZATION_REQUEST_FAILED`, and `CONFIGURATION_INVALID` — those names are **not** on the merged `SubStatusCode` and have been removed; if a wrapper-side consumer needs a code for any of those concepts, it must be added to the driver first and then re-exported.

**Substatus codes are emitted as named C constants in `azurecosmosdriver.h`.** Per Ashley's review feedback on this spec, language-binding authors should not have to hand-transcribe the synthetic substatus integers into their own SDK in order to switch on them — that is fragile and silently rots when the driver adds new variants. The wrapper therefore emits the **full** synthetic substatus set as `#define` constants (or a `cosmos_substatus_t` enum, whichever cbindgen produces cleanly for a `pub const` group of `u16` literals) in the generated header, sourced from a single Rust module that mirrors the driver's `SubStatusCode` constants 1:1:

```rust
// src/error/substatus_constants.rs (wrapper-side, codegen-fed into the header).
// Each constant pulls its value from the driver-side const so there is a single
// source of truth and the CI check in §2.2 catches drift.
use azure_data_cosmos_driver::error::SubStatusCode;
pub const COSMOS_SUB_STATUS_TRANSPORT_GENERATED_503:           u16 = SubStatusCode::TRANSPORT_GENERATED_503.value();
pub const COSMOS_SUB_STATUS_CLIENT_OPERATION_TIMEOUT:          u16 = SubStatusCode::CLIENT_OPERATION_TIMEOUT.value();
pub const COSMOS_SUB_STATUS_TRANSPORT_CONNECTION_FAILED:       u16 = SubStatusCode::TRANSPORT_CONNECTION_FAILED.value();
pub const COSMOS_SUB_STATUS_TRANSPORT_IO_FAILED:               u16 = SubStatusCode::TRANSPORT_IO_FAILED.value();
pub const COSMOS_SUB_STATUS_TRANSPORT_DNS_FAILED:              u16 = SubStatusCode::TRANSPORT_DNS_FAILED.value();
pub const COSMOS_SUB_STATUS_TRANSPORT_BODY_READ_FAILED:        u16 = SubStatusCode::TRANSPORT_BODY_READ_FAILED.value();
pub const COSMOS_SUB_STATUS_TRANSPORT_HTTP2_INCOMPATIBLE:      u16 = SubStatusCode::TRANSPORT_HTTP2_INCOMPATIBLE.value();
pub const COSMOS_SUB_STATUS_SERIALIZATION_RESPONSE_BODY_INVALID: u16 = SubStatusCode::SERIALIZATION_RESPONSE_BODY_INVALID.value();
pub const COSMOS_SUB_STATUS_AUTHENTICATION_TOKEN_ACQUISITION_FAILED: u16 = SubStatusCode::AUTHENTICATION_TOKEN_ACQUISITION_FAILED.value();
// Service-error substatus mirrors (LSN/session/etc.) emitted under the
// same naming rule from the corresponding pub const SubStatusCode definitions.
```

The cbindgen `export.item_types` policy in §2.2 explicitly includes `"constants"` so these emit as `#define` (C) / `static const` (C++) in `azurecosmosdriver.h`. The same Rust module is the single source of truth — a CI check (extending the header-diff check in §2.2) compares the wrapper's `COSMOS_SUB_STATUS_*` constants against the driver's `SubStatusCode` variants by name and fails if any driver-side variant is missing a wrapper-side mirror. This lets language-binding generators (the .NET `T4` template, the Java `cosmos-driver-bindgen` step, etc.) read the C header and emit language-native constants without re-typing integer literals. **Service-error HTTP-status substatus codes** (e.g. `READ_SESSION_NOT_AVAILABLE = 1002`, `PARTITION_KEY_RANGE_GONE = 1002`) follow the same emission rule.

Status-code main values follow the same approach via the existing `cosmos_error_code_t` enum (§3.5.1) plus the standard HTTP status codes the wrapper surfaces through `cosmos_error_status_code(e)` — host SDKs that switch on HTTP status either use their language-standard `HttpStatusCode` enum or define their own constants from `cosmos_error_status_code`'s `uint16_t` return; the wrapper does **not** re-export the standard HTTP code values as `COSMOS_HTTP_*` constants to avoid polluting the header with values that are already standardized across every language.

### 3.6 Completion records & operation handles

#### 3.6.1 `cosmos_completion_t`

Every async submission eventually produces exactly one completion record — success, failure, or cancellation. The record is an opaque handle with the following accessors:

```c
/* Outcome — exactly one of OK / ERROR / CANCELLED. UNKNOWN is reserved
 * for forward compatibility. */
typedef enum cosmos_completion_outcome {
    COSMOS_COMPLETION_OUTCOME_OK        = 0,
    COSMOS_COMPLETION_OUTCOME_ERROR     = 1,
    COSMOS_COMPLETION_OUTCOME_CANCELLED = 2,
    COSMOS_COMPLETION_OUTCOME_UNKNOWN   = 255,
} cosmos_completion_outcome_t;

cosmos_completion_outcome_t cosmos_completion_outcome(const cosmos_completion_t *c);

/* The user_data the caller supplied at submit time. NULL is allowed and
 * preserved verbatim. The wrapper does NOT free user_data — the receive loop
 * owns its lifetime. */
void *cosmos_completion_user_data(const cosmos_completion_t *c);

/* The in-flight handle that produced this completion. Borrowed; valid until
 * cosmos_completion_free regardless of whether the submitter has already
 * called cosmos_operation_handle_free on its own handle to the same
 * operation. The completion holds its own independent strong reference to
 * the internal operation state; the borrowed pointer here is a companion
 * handle owned by the completion. Useful for correlating to host-side state
 * keyed by the operation (e.g. retrieving the final state via
 * cosmos_operation_handle_state, see §3.6.2). */
const cosmos_operation_handle_t *cosmos_completion_op_handle(const cosmos_completion_t *c);

/* Coarse status — always populated, even when the rich cosmos_error_t was
 * suppressed via include_error_details = false. Population rules:
 *   • outcome = OK         → SUCCESS (0).
 *   • outcome = CANCELLED  → COSMOS_ERROR_CODE_OPERATION_CANCELLED (4012).
 *   • outcome = ERROR      → derived from the inner CosmosError's
 *     (status, sub_status) pair per the routing table in §6.3. Examples:
 *       — a wire error with HTTP 429 → COSMOS_ERROR_CODE_THROTTLED;
 *       — a wire error with HTTP 404 → COSMOS_ERROR_CODE_NOT_FOUND;
 *       — a synthetic 503 with sub_status TRANSPORT_GENERATED_503 →
 *         COSMOS_ERROR_CODE_TRANSPORT_FAILURE;
 *       — a synthetic 408 with sub_status CLIENT_OPERATION_TIMEOUT →
 *         COSMOS_ERROR_CODE_TIMEOUT.
 *     The wrapper preserves the same routing when include_error_details =
 *     false, so host SDKs that only care about the coarse code can disable
 *     rich capture without losing routing information. */
cosmos_error_code_t cosmos_completion_status(const cosmos_completion_t *c);

/* True iff the caller invoked cosmos_operation_handle_cancel on this
 * operation's handle before the completion was posted, regardless of the
 * eventual outcome. Allows host SDKs to express "cancel was requested, but
 * the operation completed naturally before the cancellation landed" (cf.
 * Java CompletableFuture.cancel() returning false for already-completed
 * futures) by combining this predicate with the outcome accessor. */
bool cosmos_completion_was_cancel_requested(const cosmos_completion_t *c);

/* Take ownership of the response. Returns NULL when outcome != OK or after a
 * previous _take_response on this completion. After this call,
 * cosmos_completion_response below returns NULL. */
cosmos_response_t *cosmos_completion_take_response(cosmos_completion_t *c);

/* Borrowed access to the response. Lifetime = until _take_response or
 * cosmos_completion_free is called on the completion. NULL when outcome != OK. */
const cosmos_response_t *cosmos_completion_response(const cosmos_completion_t *c);

/* Take ownership of the rich error payload. Returns NULL when outcome != ERROR,
 * when the queue was created with include_error_details = false, or after a
 * previous _take_error on this completion. */
cosmos_error_t *cosmos_completion_take_error(cosmos_completion_t *c);

/* Borrowed access to the rich error. NULL on OK / CANCELLED, when details
 * are suppressed, or after a previous cosmos_completion_take_error call on
 * the same completion (the take call transfers ownership; subsequent
 * borrows return NULL). */
const cosmos_error_t *cosmos_completion_error(const cosmos_completion_t *c);

/* Free the completion record. Any borrowed response/error obtained via the
 * non-take accessors becomes invalid. Owned response/error obtained via
 * _take_response / _take_error remain valid until their own _free. */
void cosmos_completion_free(cosmos_completion_t *c);
```

The typical receive-loop body in C is:

```c
for (;;) {
    cosmos_completion_t *c = cosmos_cq_wait(queue, UINT32_MAX);
    if (c == NULL) {
        if (cosmos_cq_state(queue) != COSMOS_CQ_STATE_RUNNING) break;
        continue;  /* spurious wake; keep waiting */
    }
    void *ud = cosmos_completion_user_data(c);
    switch (cosmos_completion_outcome(c)) {
        case COSMOS_COMPLETION_OUTCOME_OK:        dispatch_ok(ud,  cosmos_completion_take_response(c)); break;
        case COSMOS_COMPLETION_OUTCOME_ERROR:     dispatch_err(ud, cosmos_completion_take_error(c));    break;
        case COSMOS_COMPLETION_OUTCOME_CANCELLED: dispatch_cancel(ud);                                  break;
        default:                                  dispatch_unknown(ud, cosmos_completion_status(c));    break;
    }
    cosmos_completion_free(c);
    /* receive-loop also frees user_data here, after the host continuation has run */
}
```

#### 3.6.2 `cosmos_operation_handle_t`

A submit returns one of these. It is the caller's handle to an in-flight (or just-completed) operation.

```c
/* Request cooperative cancellation. Idempotent; non-blocking. The operation
 * still posts a completion record to its queue — with outcome = CANCELLED
 * if the cancellation arrived before the natural completion, or
 * outcome = OK / ERROR if the cancellation lost the race. See §3.6.3 for
 * exactly how cancellation is implemented. */
void cosmos_operation_handle_cancel(cosmos_operation_handle_t *op);

/* Poll the operation's lifecycle state. Lock-free, non-blocking; safe to
 * call from any thread without coordinating with the receive loop. Lets a
 * producer that did not retain the completion (e.g. a fire-and-forget
 * caller) discover whether the operation has finished and how, without
 * draining the queue.
 *
 *   IN_FLIGHT   — submission succeeded; no completion has been posted yet.
 *   COMPLETED   — completion was posted with outcome = OK.
 *   FAILED      — completion was posted with outcome = ERROR.
 *   CANCELLED   — completion was posted with outcome = CANCELLED.
 *
 * After cosmos_operation_handle_free this function is a use-after-free and
 * must NOT be called. After the completion has been freed via
 * cosmos_completion_free, the state remains observable on this handle
 * because the handle's own Arc keeps the inner state alive. */
typedef enum cosmos_operation_handle_state {
    COSMOS_OPERATION_HANDLE_STATE_IN_FLIGHT = 0,
    COSMOS_OPERATION_HANDLE_STATE_COMPLETED = 1,
    COSMOS_OPERATION_HANDLE_STATE_FAILED    = 2,
    COSMOS_OPERATION_HANDLE_STATE_CANCELLED = 3,
} cosmos_operation_handle_state_t;
cosmos_operation_handle_state_t cosmos_operation_handle_state(
    const cosmos_operation_handle_t *op);

/* Free the FFI handle. Safe to call before or after the completion has been
 * delivered. Does NOT cancel the operation — call _cancel first if that's
 * what you want. cosmos_operation_handle_free(NULL) is a no-op.
 *
 * Freeing only drops THIS handle's Arc reference; if the completion holds
 * its own reference (it does — see cosmos_completion_op_handle in §3.6.1)
 * the inner operation state stays alive until both the producer's handle
 * and the completion record are freed. */
void cosmos_operation_handle_free(cosmos_operation_handle_t *op);
```

> **Mid-flight diagnostics snapshot — deferred to a future revision.** Earlier drafts of this spec exposed `cosmos_operation_handle_diagnostics_snapshot(op)` for grabbing partial diagnostics from a still-running operation. That accessor is **not** in v1 because the driver's `DiagnosticsContext` is constructed inside the executing future's stack frame (`CosmosResponse::diagnostics()` returns `Arc<DiagnosticsContext>` only once the response is built), not in a shared `Arc<Mutex<…>>` the wrapper can clone while the future is still running. Implementing mid-flight diagnostics therefore requires a driver-side refactor to expose a shared mutable diagnostics handle through the pipeline. Tracked in §9 Q16. Today, diagnostics are available only after completion via `cosmos_response_diagnostics` (success path) or `cosmos_error_diagnostics` (error path).

**Lifetime relationship between handle and completion.** The operation handle and its completion record are independent FFI handles with overlapping lifetimes. The handle is alive from submit-return until `cosmos_operation_handle_free`; the completion is alive from `cosmos_cq_wait` return until `cosmos_completion_free`. Both hold their own `Arc<OperationInner>` strong reference, so:

- A caller that does not want cancel / state-poll integration can free `op` immediately after a successful submit. The receive loop still sees a valid completion and can borrow the operation handle via `cosmos_completion_op_handle` if it needs the `_state` accessor.
- A caller that wants cancel propagation (e.g. .NET `CancellationToken`, Go `ctx.Done()`, Java `CompletableFuture.cancel`) stashes `op` alongside the host-language continuation and frees it from the receive loop after the completion has been observed.

Freeing the handle does **not** cancel the operation — the inner driver-side state is independently reachable from the completion's Arc, so the underlying request continues to completion.

#### 3.6.3 Cancellation

`cosmos_operation_handle_cancel` requests cancellation. The driver crate does **not** currently accept a `CancellationToken` on `CosmosDriver::execute_operation` or `execute_singleton_operation` (`src/driver/cosmos_driver.rs:1242,1281`) — the only end-to-end timing primitive the pipeline honors is the `deadline: Option<Instant>` carried through `OperationOptions` (`pipeline/components.rs`, enforced in `transport_pipeline.rs` at every retry boundary). Cancellation is therefore implemented **in the wrapper layer**, not the driver, with the following semantics:

**Implementation.** For every async submit, the wrapper drives the driver future inside a `tokio::select!` against a per-operation `tokio::sync::Notify` (or equivalent) tied to the operation handle:

```rust
let cancel = Arc::new(Notify::new());
let fut = driver.execute_singleton_operation(operation, options);
let outcome = tokio::select! {
    biased;
    _ = cancel.notified()       => Outcome::Cancelled,
    r = fut                      => match r { Ok(resp) => Outcome::Ok(resp), Err(e) => Outcome::Error(e) },
};
post_completion_to_queue(outcome, /* was_cancel_requested = */ cancel_observed, user_data);
```

`cosmos_operation_handle_cancel` signals the `Notify`. The driver future is then **dropped**. Tokio unwinds its parked awaits; `reqwest` aborts any in-flight HTTP request as part of its `Drop` impl (closing the connection if mid-body, leaving the pool intact otherwise); buffered request bodies and per-attempt retry state are released. The wrapper synthesizes a completion record with `outcome = COSMOS_COMPLETION_OUTCOME_CANCELLED` and `cosmos_completion_status = COSMOS_ERROR_CODE_OPERATION_CANCELLED (4012)`.

**Caveats — call these out to host SDK authors.**

- **Granularity is "drop at the next await point", not "check a token between operations".** The future is abandoned wherever it is currently parked. If it is parked inside a non-cancellable syscall (e.g. a DNS lookup that has already entered `getaddrinfo` on a thread-pool blocking task), the cancel takes effect only when control returns to the Tokio reactor — typically within a few milliseconds, but not instantaneous.
- **Cancel-vs-completion race.** If the driver future resolves to `Ok(resp)` / `Err(e)` before the `Notify` is observed, the completion is delivered with the natural outcome (`OK` / `ERROR`). `cosmos_completion_was_cancel_requested` (§3.6.1) returns `true` in this case so host SDKs that model "cancel won the race vs. operation succeeded but caller no longer wants the result" can distinguish them.
- **No driver-side cancellation diagnostics.** Because the driver future is dropped rather than receiving a cancellation signal, the partial `DiagnosticsContext` it was building is dropped with it — the wrapper has no `Arc` view into mid-flight diagnostics (see §3.6.2 and §9 Q16). A cancelled completion therefore carries no `cosmos_response_t` and no `cosmos_diagnostics_t`. Host SDKs that want partial diagnostics for cancelled operations should rely on the `cosmos_operation_handle_state` poller plus their own external instrumentation, not on the completion record.
- **In-flight requests are not actively aborted on the wire.** Dropping the `reqwest` future closes the underlying TCP connection (preventing connection-pool reuse for that request) but does not send a protocol-level cancel; the gateway may still execute the request server-side. Idempotency considerations for writes are unchanged from the regular retry path.
- **Hedging is not in scope.** Hedged-read / hedged-write features that exist in the .NET and Java SDKs are not currently implemented in the Rust driver. References to "hedge race" cancellation behavior in earlier drafts of this spec are removed; if hedging lands in a future driver version, this section must be revisited.

A cancelled operation **always** produces a completion record — there is no "silent drop" path. The `was_cancel_requested` flag plus `outcome` give host SDKs everything they need to express the four cases: (cancel requested, outcome = CANCELLED), (cancel requested, outcome = OK / ERROR — race lost), and (no cancel requested, outcome = OK / ERROR).

**Future driver-side primitive.** A first-class `execute_operation(op, options, cancel: CancellationToken)` overload in the driver — propagating the token through the pipeline and checking it at every `await` boundary — would let the wrapper retire its `select!` shim and surface mid-flight cancellation diagnostics. Tracked in §9 Q13; not blocking on v1.

#### 3.6.4 Queue shutdown semantics

`cosmos_cq_shutdown(queue)`:

1. Marks the queue as shutting down (`cosmos_cq_state` → `SHUTDOWN`).
2. Requests cancellation on every in-flight operation targeting this queue.
3. Wakes any thread currently blocked in `cosmos_cq_wait` — it returns NULL.
4. Subsequent submits targeting this queue fail their pre-flight check with `COSMOS_ERROR_CODE_QUEUE_SHUTDOWN` (4011).

The consumer drains by calling `cosmos_cq_wait` until it returns NULL **and** `cosmos_cq_state` returns `DRAINED`. Only at that point is `cosmos_cq_free` safe to call without blocking on in-flight work.

---

## 4. Module-by-Module Surface

### 4.1 Runtime (`src/runtime/`)

The wrapper exposes a **builder API** that mirrors `CosmosDriverRuntimeBuilder` (`src/driver/runtime.rs:398-756`) exactly, rather than a single-shot `cosmos_runtime_create(options)` constructor. There are two reasons for this:

1. `CosmosDriverRuntimeBuilder::build()` is **`pub async fn`** (`runtime.rs:662`) and performs real network I/O during build — bootstrap metadata transport handshake (`runtime.rs:730`) and IMDS probe (`runtime.rs:747`). A flat synchronous `_create` cannot directly call `build().await`; it would need to wrap the call in its own ad-hoc Tokio executor, which gets confused when the runtime being created is itself the Tokio runtime the wrapper intends to keep. The builder API isolates the bootstrap step explicitly.
2. The runtime carries many more knobs than the two-field `cosmos_runtime_options_t` shape suggests (connection pool sizing, user-agent suffix, workload id, correlation id, CPU refresh interval, emulator certificate trust, optional fault-injection rules, optional HTTP client factory under `#[cfg(test)]`, etc.). A flat options struct would either lock out post-merge driver additions or grow a `_unused` tail that violates §7 ABI rules.

```c
typedef struct cosmos_runtime_builder cosmos_runtime_builder_t;

cosmos_runtime_builder_t *cosmos_runtime_builder_new(void);
void cosmos_runtime_builder_free(cosmos_runtime_builder_t *b);

/* Worker thread count for the internal Tokio runtime. 0 = number of CPUs.
 * Returns COSMOS_ERROR_CODE_INVALID_OPTION_VALUE (4014) on values exceeding
 * an internal sanity cap (currently 4096); the builder is left unchanged. */
cosmos_error_code_t cosmos_runtime_builder_with_worker_threads(
    cosmos_runtime_builder_t *b, uint32_t worker_threads);

/* Mirrors CosmosDriverRuntimeBuilder::with_tokio_thread_name_prefix. NUL-
 * terminated UTF-8, must not contain NUL bytes, must be ≤ 64 chars after
 * trimming. INVALID_OPTION_VALUE on violation. */
cosmos_error_code_t cosmos_runtime_builder_with_thread_name_prefix(
    cosmos_runtime_builder_t *b, const char *prefix);

/* Mirrors the user-agent / workload-id / correlation-id / emulator-trust /
 * connection-pool setters in src/driver/runtime.rs (CosmosDriverRuntimeBuilder
 * methods). Names follow cosmos_runtime_builder_with_<rust_method_name>.
 * Full list maintained in §8 Phase 2; each one returns SUCCESS or
 * INVALID_OPTION_VALUE and never blocks. */
cosmos_error_code_t cosmos_runtime_builder_with_user_agent_suffix(
    cosmos_runtime_builder_t *b, const char *suffix);
cosmos_error_code_t cosmos_runtime_builder_with_workload_id(
    cosmos_runtime_builder_t *b, uint64_t workload_id);
cosmos_error_code_t cosmos_runtime_builder_with_correlation_id(
    cosmos_runtime_builder_t *b, const char *correlation_id);
cosmos_error_code_t cosmos_runtime_builder_with_allow_emulator_invalid_certs(
    cosmos_runtime_builder_t *b, bool allow);
/* ... and the rest of CosmosDriverRuntimeBuilder's surface, mirrored 1:1 ... */

/* Consume the builder and produce a runtime. Internally:
 *   1. Constructs the Tokio runtime synchronously with the configured
 *      worker_threads / thread_name_prefix.
 *   2. Calls `tokio_runtime.block_on(CosmosDriverRuntimeBuilder::build())`
 *      on the just-created runtime to drive the async bootstrap
 *      (transport handshake + IMDS probe).
 *   3. Boxes (tokio_runtime, Arc<CosmosDriverRuntime>) as the
 *      cosmos_runtime_t backing store.
 *
 * Failure modes (rich detail in *out_error when non-NULL):
 *   • RUNTIME_BUILD_FAILED (4015) for TLS init, proxy config, IMDS / env
 *     parsing, or bootstrap transport errors. Inner cosmos_error_t mirrors
 *     the original azure_core::Error / azure_data_cosmos::Error.
 *   • INVALID_OPTION_VALUE (4014) for a builder field that was somehow
 *     accepted by a `with_*` setter but rejected during build (defense in
 *     depth — should not normally occur).
 *
 * Consumes the builder regardless of outcome — on failure the builder is
 * freed and *out_runtime is left NULL. */
cosmos_error_code_t cosmos_runtime_builder_build(
    cosmos_runtime_builder_t *b,
    cosmos_runtime_t **out_runtime,
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

The shared `CosmosDriverRuntime` is built via the wrapper's mirrored `CosmosDriverRuntimeBuilder` and is the cache key for `cosmos_driver_get_or_create`.

**Cardinality and cache-scoping advisory (read carefully — this is the most common porting trap).** Each `cosmos_runtime_t` owns its own Tokio runtime **and** its own `CosmosDriverRuntime`, which means its own driver cache (§4.4.1), its own connection pool, its own bootstrap transport, and its own IMDS-derived VM metadata. Two runtimes pointing at the same Cosmos endpoint therefore open **two independent connection pools** and pay double the metadata-refresh cost. Host SDKs that map naturally onto a "single global client" pattern in their parent language (.NET `static CosmosClient`, Java `CosmosAsyncClient` singleton, Go `*Client` reused across goroutines) **must** create exactly one `cosmos_runtime_t` per process and share it across all consumers. The wrapper does not enforce this in v1 — `cosmos_runtime_builder_build` succeeds N times if you call it N times — but doing so is essentially always a bug. See §9 Q1 for whether v2 will reject the second call with a dedicated error code.

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

`get_or_create` calls `CosmosDriverRuntime::get_or_create_driver(account, options).await` (`src/driver/runtime.rs:364-395`) which already awaits `initialize()` — i.e. it touches the network. It therefore uses Pattern B (async submit). The completion delivers a degenerate `cosmos_response_t` from which the constructed `cosmos_driver_t*` is extracted via `cosmos_response_take_driver`:

```c
/* Submit driver creation / lookup. The completion delivers a cosmos_response_t
 * whose only meaningful payload is the driver handle, retrieved via
 * cosmos_response_take_driver. */
cosmos_operation_handle_t *cosmos_driver_get_or_create_submit(
    const cosmos_runtime_t *runtime,
    const cosmos_account_ref_t *account,
    const cosmos_driver_options_t *options,  /* nullable */
    cosmos_cq_t *queue,
    void *user_data,
    cosmos_error_code_t *out_pre_error);

/* Synchronous convenience wrapper — submits then drains a single completion
 * from a private internal queue. Provided for the common case of "initialize
 * the driver at startup" where async ergonomics are unnecessary. */
cosmos_error_code_t cosmos_driver_get_or_create_blocking(
    const cosmos_runtime_t *runtime,
    const cosmos_account_ref_t *account,
    const cosmos_driver_options_t *options,  /* nullable */
    cosmos_driver_t **out_driver,
    cosmos_error_t *out_error);

void cosmos_driver_free(cosmos_driver_t *driver);

/* Explicit re-initialization. Normally unnecessary because get_or_create
 * already awaits initialize() on first creation. The completion's response
 * carries no payload — outcome alone is meaningful. */
cosmos_operation_handle_t *cosmos_driver_initialize_submit(
    const cosmos_driver_t *driver,
    cosmos_cq_t *queue,
    void *user_data,
    cosmos_error_code_t *out_pre_error);

/* Take the driver handle out of the degenerate response delivered by
 * cosmos_driver_get_or_create_submit. May be called at most once per
 * response; subsequent calls return NULL. */
cosmos_driver_t *cosmos_response_take_driver(cosmos_response_t *r);
```

The `_blocking` convenience exists only for `get_or_create` (and is implemented by submitting to a private internal queue and draining one completion). All other network-touching APIs are async-only — there is no `cosmos_driver_execute_blocking`.

#### 4.4.1 Driver cache semantics — *normative*

The wrapper inherits the driver's cache exactly as implemented in `runtime.rs:364-395`. Host SDKs (and language-binding authors) **must** account for the following:

- **Cache key.** The cache key is the account endpoint URL **only** (`account.endpoint().to_string()`, `runtime.rs:369`). It does **not** include the credential identity, the application name, the `DriverOptions` contents, or anything else.
- **Options are silently ignored on cache hit.** If `cosmos_driver_get_or_create` is called twice for the same endpoint with different `options`, the second call returns the *first* driver and discards the second `options` argument (`runtime.rs:374`). When the wrapper detects this — i.e. `options != NULL` and the cache already contained an entry — it **must**:
  1. Deliver a completion with `outcome = OK`. The cached driver is extractable via `cosmos_response_take_driver`.
  2. Populate the completion's `cosmos_error_t` (retrievable via `cosmos_completion_take_error` / `_error`) with an advisory whose code is `COSMOS_ERROR_CODE_OPTIONS_IGNORED_ON_CACHE_HIT` (`5001`).
  3. Set `cosmos_completion_status` to `5001` — *not* `SUCCESS`. `5001` lives in the `5xxx` warning class (see §3.5.1) so host SDKs that route any non-success status into their error path will not silently miss the advisory. Host SDKs that want to accept the cached instance explicitly switch on the warning code; the rest correctly treat it as a hard error. For the synchronous `cosmos_driver_get_or_create_blocking` convenience, the return code is `5001` and `*out_error` carries the same advisory. This is **not** firing-mode-dependent: the warning is always emitted on cache-hit-with-options. (Earlier drafts predicated it on the unresolved single-runtime-mode decision in §9 Q1; that coupling is removed.)
- **Credential collisions.** Two `cosmos_account_ref_t`s built with the same endpoint but **different credentials** collide in the cache: the first credential wins, the second is silently dropped. The wrapper does **not** transparently detect this (the driver does not currently surface credential identity through `AccountReference`). Host SDKs that need to multiplex independent credentials against a single endpoint must build a workaround — typically by constructing one `cosmos_runtime_t` per credential via `cosmos_runtime_builder_build`, since cache scoping is per-runtime. See §9 Q6 and the per-runtime cache advisory in §4.1.
- **Driver lifetime is bounded by the runtime.** Freeing the last FFI handle to a cached driver does **not** evict the cache entry — the runtime keeps a strong `Arc` reference. Eviction happens only when the owning `cosmos_runtime_t` is freed. Tests that need to force a fresh driver must create a fresh runtime.
- **Lost-race redundant init.** `runtime.rs:380-390` uses `or_insert_with` rather than a double-checked-lock pattern: if N callers concurrently invoke `get_or_create_driver` for the same brand-new endpoint, each may run a redundant network `initialize()` before all-but-one of the resulting drivers is dropped. The wrapper does not mitigate this — host SDKs that warm a pool of receive-loop threads at startup should serialize the first `get_or_create` for each endpoint to avoid duplicate metadata round-trips.

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

### 4.6 Operations (`src/handles/operation.rs`, `src/op_request.rs`)

This is the heart of the new wrapper. All data-plane work flows through a
single flat `cosmos_operation_request_t` that the host fills out in its own
language and hands to one of the two submit entry points in §4.7. There is
**no** per-operation factory or mutator surface: the operation kind, resource
references, ids, partition key, feed range, body, per-call tweaks, and a
pointer to `cosmos_operation_options_t` all ride on the one request struct.
The wrapper validates the request, builds the driver's `CosmosOperation` +
`OperationOptions` internally, and dispatches to the requested driver method.

Reference *handles* (`cosmos_account_ref_t`, `cosmos_container_ref_t`, the
partition-key and feed-range handles) remain opaque handles, not flat data:
they wrap `Arc`-shared Rust state that cannot be safely round-tripped as plain
`#[repr(C)]` bytes.

#### 4.6.1 `cosmos_operation_request_t` and `cosmos_operation_kind_t`

The host fills out a single `#[repr(C)]` request struct. The `kind` field
(a validated `cosmos_operation_kind_t` discriminant, stored as a raw `int32`
and range-checked before dispatch — never transmuted) selects which driver
`CosmosOperation` the wrapper builds; the remaining fields supply that kind's
inputs. Fields irrelevant to the chosen kind must be left NULL / sentinel —
strict validation rejects mismatches with `COSMOS_ERROR_CODE_INVALID_ARGUMENT`.
All pointer fields are borrowed for the duration of the submit call only; the
wrapper copies what it needs before returning.

```c
typedef struct cosmos_operation_request {
    int32_t kind;                       /* cosmos_operation_kind_t discriminant */

    /* Resource references — supply the one(s) the kind requires; else NULL. */
    const cosmos_account_ref_t   *account;
    const cosmos_database_ref_t  *database;
    const cosmos_container_ref_t *container;

    const char *item_id;                /* item-scope kinds; else NULL */
    const char *resource_link;          /* offer kinds; else NULL */

    const cosmos_partition_key_t *partition_key;  /* item / read_all_items / batch */
    const cosmos_feed_range_t    *feed_range;     /* optional for query_items */

    cosmos_bytes_view_t body;           /* {NULL, 0} = no body */

    const char *session_token;          /* NULL = unset */
    const char *activity_id;            /* NULL = auto-generate */
    const char *continuation_token;     /* feed resume; NULL = none */

    int32_t max_item_count;             /* < 0 = unset */
    uint8_t patch_max_attempts;         /* 0 = unset */
    int8_t  populate_index_metrics;     /* tri-state: 0 unset / 1 false / 2 true */
    int8_t  populate_query_metrics;     /* tri-state */

    int32_t     precondition_kind;      /* cosmos_precondition_kind_t discriminant */
    const char *precondition_etag;      /* required iff precondition_kind != None */

    const cosmos_operation_options_t *options;  /* NULL = driver/runtime defaults */
} cosmos_operation_request_t;
```

`cosmos_operation_kind_t` is an append-only enum (new kinds get new trailing
discriminants so the ABI stays stable). `0` (`Invalid`) is always rejected.
The kinds and their required fields:

| Kind (value) | Driver `CosmosOperation` | Required fields |
|---|---|---|
| `CreateDatabase` (1) | `create_database` | `account` + body |
| `ReadAllDatabases` (2) | `read_all_databases` | `account` |
| `QueryDatabases` (3) | `query_databases` | `account` + body |
| `QueryOffers` (4) | `query_offers` | `account` + body |
| `ReadOffer` (5) | `read_offer` | `account` + `resource_link` |
| `ReplaceOffer` (6) | `replace_offer` | `account` + `resource_link` + body |
| `ReadDatabase` (7) | `read_database` | `database` |
| `DeleteDatabase` (8) | `delete_database` | `database` |
| `CreateContainer` (9) | `create_container` | `database` + body |
| `ReadAllContainers` (10) | `read_all_containers` | `database` |
| `QueryContainers` (11) | `query_containers` | `database` + body |
| `ReadContainer` (12) | `read_container` | `container` |
| `ReplaceContainer` (13) | `replace_container` | `container` + body |
| `DeleteContainer` (14) | `delete_container` | `container` |
| `ReadAllItems` (15) | `read_all_items` | `container` + `partition_key` |
| `ReadAllItemsCrossPartition` (16) | `read_all_items_cross_partition` | `container` |
| `QueryItems` (17) | `query_items` | `container` + body; `feed_range` optional |
| `Batch` (18) | `batch` | `container` + `partition_key` + body |
| `CreateItem` (19) | `create_item` | `container` + `partition_key` + body |
| `ReadItem` (20) | `read_item` | `container` + `partition_key` + `item_id` |
| `UpsertItem` (21) | `upsert_item` | `container` + `partition_key` + body |
| `ReplaceItem` (22) | `replace_item` | `container` + `partition_key` + `item_id` + body |
| `DeleteItem` (23) | `delete_item` | `container` + `partition_key` + `item_id` |
| `PatchItem` (24) | `patch_item` | `container` + `partition_key` + `item_id` + body |

The query text / parameters for the `Query*` kinds live in `body` as JSON of
shape `{ "query": "SELECT * FROM c WHERE c.foo = @p", "parameters": [...] }`,
per the schema-agnostic data-plane contract — the wrapper does **not**
JSON-encode it; host SDKs build the bytes in their native serializer exactly
as they do for item bodies. A NULL `feed_range` on `QueryItems` targets the
entire container (cross-partition).

Partition keys are baked into the operation's underlying `ItemReference` at
construction; there is no settable-partition-key mutator. The partition key /
feed range / references in the request are **cloned** into the operation, so
the caller retains ownership of its handles and must free each independently.

#### 4.6.2 `cosmos_operation_options_t`

Per-call options ride on `cosmos_operation_request_t.options` (NULL = use the
driver/runtime defaults for every field). It is a flat `#[repr(C)]` mirror of
the driver's `OperationOptions`, every field tri-state encoded so the host can
distinguish "inherit from a lower layer" from an explicit value:

- **enum fields** (`read_consistency_strategy`, `content_response_on_write`),
  stored as raw `int32` and validated on conversion: `0` = unset (inherit),
  other values map to the driver variant.
- **tri-state bools** (`session_capturing_disabled`,
  `per_partition_circuit_breaker_enabled`): `0` unset / `1` false / `2` true.
- **`int32` numeric fields** (retry / circuit-breaker counters): `< 0` = unset.
- **`int64` duration fields** (`*_ms`): `< 0` = unset, else milliseconds.
- **string / array fields** (`throughput_control_group`, `excluded_regions`,
  `custom_headers`): NULL / length `0` = unset.

```c
typedef struct cosmos_operation_options {
    int32_t read_consistency_strategy;  /* cosmos_read_consistency_strategy_t; 0 = unset */
    int32_t content_response_on_write;  /* cosmos_content_response_on_write_t; 0 = unset */
    int8_t  session_capturing_disabled;                 /* tri-state */
    int8_t  per_partition_circuit_breaker_enabled;      /* tri-state */
    int32_t max_failover_retry_count;                   /* < 0 = unset */
    int32_t max_session_retry_count;                    /* < 0 = unset */
    int32_t circuit_breaker_failure_count_for_reads;    /* < 0 = unset */
    int32_t circuit_breaker_failure_count_for_writes;   /* < 0 = unset */
    int32_t circuit_breaker_timeout_counter_reset_window_in_minutes;         /* < 0 = unset */
    int32_t allowed_partition_unavailability_duration_in_seconds;            /* < 0 = unset */
    int32_t ppcb_stale_partition_unavailability_refresh_interval_in_seconds; /* < 0 = unset */
    int64_t end_to_end_timeout_ms;                      /* < 0 = unset */
    int64_t endpoint_unavailability_ttl_ms;             /* < 0 = unset */
    const char        *throughput_control_group;        /* NULL = unset */
    const char *const *excluded_regions;                /* NULL / len 0 = unset */
    size_t             excluded_regions_len;
    const cosmos_header_kv_t *custom_headers;           /* NULL / len 0 = none */
    size_t                    custom_headers_len;
} cosmos_operation_options_t;

/* Returns an all-unset value by value; the host sets the fields it cares
 * about and leaves the rest at their inherit sentinels. */
cosmos_operation_options_t cosmos_operation_options_default(void);
```

The wrapper validates every field at the boundary when translating the struct
into the driver's `OperationOptions` (out-of-range enum / tri-state
discriminants return `COSMOS_ERROR_CODE_INVALID_OPTION_VALUE`; a non-NULL
`excluded_regions` with length `0` is rejected as ambiguous). Driver setters
without a dedicated field above (e.g. consistency level, throughput-control
group name) are expressed through the corresponding option field rather than a
per-operation mutator.

#### 4.6.3 Submission and completion lifecycle — *normative*

The request struct is **borrowed**, not consumed: both submit entry points
read everything they need from `cosmos_operation_request_t` (copying owned
bytes / strings, cloning the reference handles) before returning, so the host
may free or reuse the request and all its inputs immediately after submit
returns. There is no operation-builder handle to consume or free.

```c
cosmos_operation_handle_t *cosmos_driver_execute_singleton_operation_submit(
    const cosmos_driver_t *driver,
    const cosmos_operation_request_t *request,    /* borrowed for the call only */
    cosmos_cq_t *queue,
    void *user_data,
    cosmos_error_code_t *out_pre_error);

cosmos_operation_handle_t *cosmos_driver_execute_operation_submit(
    const cosmos_driver_t *driver,
    const cosmos_operation_request_t *request,    /* borrowed for the call only */
    cosmos_cq_t *queue,
    void *user_data,
    cosmos_error_code_t *out_pre_error);
```

The contract (shared by both entry points; full response / accessor surface in §4.7):

1. **Each successful submit posts exactly one completion** to `queue` — `OK`
   (response), `ERROR` (rich error payload), or `CANCELLED` — and returns a
   non-NULL `cosmos_operation_handle_t*`. The handle is the in-flight identity:
   it supports `cosmos_operation_handle_cancel` / `_state` and lives until
   `cosmos_operation_handle_free`, independent of the completion record.
2. **A pre-flight rejection posts no completion.** When the driver / request is
   NULL, the request fails validation, or the queue is shut down / at hard
   capacity, the submit returns NULL and writes a coarse `cosmos_error_code_t`
   to `*out_pre_error`. The host may fix the request and re-submit.
3. **Request validation is strict and happens before any work is spawned.** An
   out-of-range `kind` / `precondition_kind`, a missing required field for the
   `kind`, or an out-of-range option discriminant is rejected with
   `COSMOS_ERROR_CODE_INVALID_ARGUMENT` (or
   `COSMOS_ERROR_CODE_INVALID_OPTION_VALUE`) before a Tokio task is spawned.
4. **Use the right entry point.** Singleton submit is for single-result
   operations (point ops, database / container / offer CRUD) and ignores the
   inbound `continuation_token`; the feed submit is for paginated kinds
   (queries, read-all, change feed) and threads the `continuation_token`
   through the planner. Submitting a feed kind through the singleton entry
   point makes the driver assert in debug and yields a
   `CLIENT_SINGLETON_OPERATION_RETURNED_EMPTY_PAGE`-shaped error in release.
5. **Retrying is just re-submitting** the same (or an adjusted) request; there
   is no per-operation builder state to rebuild.

#### 4.6.4 FeedRange handle

A `QueryItems` request's `feed_range` field accepts a `cosmos_feed_range_t *` (NULL targets the entire container). The handle mirrors the driver's `FeedRange` type (`src/models/feed_range.rs`) and is exposed through a minimal builder. The surface matches the driver's **current public constructors** exactly; additional variants will be added as the driver's `FeedRange` grows.

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

If a host SDK needs to resume a query from a continuation token (the typical EPK-range use case), the feed submit (`cosmos_driver_execute_operation_submit`) already covers that via the `continuation_token` round-trip on `cosmos_operation_request_t` — no `FeedRange` construction from the C side is required.

### 4.7 Submission + response (`src/handles/response.rs`)

All data-plane work flows through the two submit entry points from §4.6.3.
Each takes a single flat `cosmos_operation_request_t` (no per-operation
factory/mutator surface) and posts exactly one completion to the queue.

```c
/* Single-result submit — binds to CosmosDriver::execute_singleton_operation.
 * Use for point operations and database/container/offer CRUD (create / read /
 * replace / delete / patch item, etc.). The inbound
 * cosmos_operation_request_t.continuation_token is ignored on this path.
 *
 * If the driver unexpectedly yields Ok(None) for a mis-categorized singleton,
 * the wrapper surfaces a CLIENT_SINGLETON_OPERATION_RETURNED_EMPTY_PAGE-shaped
 * error rather than fabricating an empty response.
 *
 * Completion semantics:
 *   OK        — cosmos_completion_take_response returns the populated
 *               cosmos_response_t. The cosmos_operation_handle_t* remains
 *               valid for state polling (cosmos_operation_handle_state)
 *               until cosmos_operation_handle_free.
 *   ERROR     — cosmos_completion_take_error returns the rich payload
 *               (service / transport / client / authentication, etc.).
 *               cosmos_completion_status is the coarse code derived per
 *               §3.6.1.
 *   CANCELLED — cosmos_operation_handle_cancel or cosmos_cq_shutdown won
 *               the race against the natural completion. status is
 *               COSMOS_ERROR_CODE_OPERATION_CANCELLED (4012). See §3.6.3
 *               for the wrapper-layer drop-based implementation.
 *
 * Pre-flight rejection (NULL driver/request, malformed request, queue shut
 * down, queue at hard capacity, ...) returns NULL and writes a coarse code
 * to *out_pre_error; no completion is posted.
 */
cosmos_operation_handle_t *cosmos_driver_execute_singleton_operation_submit(
    const cosmos_driver_t *driver,
    const cosmos_operation_request_t *request,    /* borrowed for the call only */
    cosmos_cq_t *queue,
    void *user_data,
    cosmos_error_code_t *out_pre_error);

/* Feed-capable / paginated submit — binds to plan_operation + execute_plan.
 * Use for queries, read-all-items, and change feed. It resumes from an
 * inbound cosmos_operation_request_t.continuation_token and surfaces the
 * next-page token via cosmos_response_next_continuation on the completion
 * response.
 *
 * There is no wrapper-side pager type: the driver's native pagination
 * primitive is (OperationPlan, execute_plan), and the wrapper threads the
 * continuation token through it per call. Host SDKs iterate a feed by
 * re-submitting the same request with continuation_token set to the previous
 * response's next-continuation, stopping when no next token is returned.
 *
 * Completion semantics match the singleton submit above.
 */
cosmos_operation_handle_t *cosmos_driver_execute_operation_submit(
    const cosmos_driver_t *driver,
    const cosmos_operation_request_t *request,    /* borrowed for the call only */
    cosmos_cq_t *queue,
    void *user_data,
    cosmos_error_code_t *out_pre_error);

/* Response accessors — all O(1), borrowed pointers valid until
 * cosmos_response_free. String accessors return the pointer directly
 * (NULL when the field is absent), not via an out-parameter. */
uint16_t     cosmos_response_status_code(const cosmos_response_t *r);
double       cosmos_response_request_charge(const cosmos_response_t *r);
const char  *cosmos_response_activity_id(const cosmos_response_t *r);
const char  *cosmos_response_session_token(const cosmos_response_t *r);
const char  *cosmos_response_etag(const cosmos_response_t *r);

/* Raw server-header continuation (valid only for trivial single-partition
 * reads). For paginating a feed submitted via
 * cosmos_driver_execute_operation_submit, prefer
 * cosmos_response_next_continuation below. */
const char  *cosmos_response_continuation_token(const cosmos_response_t *r);

/* Planner-derived next-page token for a feed page produced by
 * cosmos_driver_execute_operation_submit, or NULL on the last page / a
 * non-feed response. Pass it back as
 * cosmos_operation_request_t.continuation_token to fetch the following page,
 * including for cross-partition queries. */
const char  *cosmos_response_next_continuation(const cosmos_response_t *r);

/* Body access — zero-copy borrowed view valid until cosmos_response_free.
 * Writes a NULL pointer + 0 length when the body is empty. For multi-part
 * feed bodies (driver's ResponseBody::Items) this returns the first part
 * only. */
cosmos_error_code_t cosmos_response_body(
    const cosmos_response_t *r,
    const uint8_t **out_data,
    size_t *out_len);

/* Side-payload take accessors for the degenerate responses produced by the
 * bootstrap / control-plane submits (cosmos_driver_get_or_create_submit /
 * cosmos_driver_resolve_container_submit). Each returns NULL on any other
 * response, on NULL input, or after a previous take. */
cosmos_driver_t        *cosmos_response_take_driver(cosmos_response_t *r);
cosmos_container_ref_t *cosmos_response_take_container(cosmos_response_t *r);

void cosmos_response_free(cosmos_response_t *r);
```

The error-payload accessors (`cosmos_error_status_code`, `cosmos_error_sub_status`, `cosmos_error_is_throttled`, etc.) are defined in §3.5.2 and live on `cosmos_error_t` — they are **not** redundantly re-exposed on `cosmos_response_t`. A completion whose `outcome == OK` yields a `cosmos_response_t` whose `cosmos_response_status_code` may still be a Cosmos-success status (200, 201, 204, ...); a completion whose `outcome == ERROR` yields a `cosmos_error_t` whose accessors expose the equivalent service-error fields.

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

The crate inherits the C-FFI toolchain established by PR [#2906](https://github.com/Azure/azure-sdk-for-rust/pull/2906) (`c_str!` macro, `BUILD_IDENTIFIER` env, cbindgen-at-build, headers `.gitignore`d, `package.name` / `[lib].name` split, MPL-2.0 in `deny.toml`, CMake + Corrosion bootstrap) and the runtime / error-payload primitives added in PR #3347 (`RuntimeContext`, `cosmos_error_code_t` value-range layout, `cosmos_string_free`, `cosmos_bytes_free`, `_unused: u8` placeholder convention). The `CallContext` thread-affine error slot from #3347 is **not** carried forward — see §3.1 for the completion-queue model that replaces it.

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
- [ ] **`sdk/cosmos/azure_data_cosmos/ARCHITECTURE.md`** — Ashley flagged a stale crate reference on #4103; refresh the wrapper paragraph to point at `azure_data_cosmos_driver_native` rather than the old `azure_data_cosmos_native`.
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

The driver moved to a structured error type in PR [#4442](https://github.com/Azure/azure-sdk-for-rust/pull/4442): [`azure_data_cosmos_driver::error::CosmosError`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/src/error/mod.rs) plus `Result<T> = std::result::Result<T, CosmosError>`. `CosmosError` is monomorphic — it carries a [`CosmosStatus`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/src/error/cosmos_status.rs) (HTTP status code + optional [`SubStatusCode`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/src/error/cosmos_status.rs)), the originating [`CosmosResponse`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/src/models/cosmos_response.rs) when a wire response was received, an `Arc<DiagnosticsContext>`, a human message, an optional `source` chain, and an optional rate-limited `Arc<str>` backtrace. There is no `Kind` enum on the merged type; failure-class taxonomy is encoded entirely through the `(status_code, sub_status)` pair. Synthetic sub-status codes (e.g. `TRANSPORT_GENERATED_503 = 20003`, `CLIENT_OPERATION_TIMEOUT = 20008`, transport `20010..=20015`, serialization `20020`, authentication `20402`) make every client-side failure observable through the same typed surface as service errors.

The wrapper's contract is shaped by that decision:

### 6.1 Return-type mapping

- `cosmos_driver_execute_singleton_operation_submit` binds to `CosmosDriver::execute_singleton_operation`, which returns `Result<CosmosResponse>` by collapsing the `Option<CosmosResponse>` returned by `execute_operation`. The submission returns a `cosmos_operation_handle_t*`; the eventual completion delivers one of three outcomes:
  - **`Ok(CosmosResponse)`** → completion outcome = OK, `cosmos_completion_take_response` returns the populated `cosmos_response_t`. The response may itself carry a Cosmos non-success HTTP status (404, 409, 412, 429, ...) only when the driver's policy explicitly does not error on it — see §6.2 below.
  - **`Err(CosmosError)`** → completion outcome = ERROR, `cosmos_completion_status` returns the mapped `cosmos_error_code_t` (see §6.3), `cosmos_completion_take_error` returns the structured `cosmos_error_t`.
  - **`execute_singleton_operation` is never expected to surface the `Ok(None)` case**; if the underlying operation is mis-categorized and the driver hands back `None`, the wrapper surfaces a `CLIENT_SINGLETON_OPERATION_RETURNED_EMPTY_PAGE`-shaped error rather than fabricating an empty response.

- `cosmos_driver_execute_operation_submit` (the feed-capable / paginated path) binds to `plan_operation` + `execute_plan`. It resumes from an inbound `cosmos_operation_request_t.continuation_token` and surfaces the next-page token via `cosmos_response_next_continuation`. Host SDKs paginate by re-submitting the same request with `continuation_token` set to the previous response's next-continuation, stopping once no next token is returned.

### 6.2 Service errors vs. successful "non-2xx" responses

The driver classifies *every* non-2xx HTTP status that the gateway returns as a `CosmosError` (this is a behavior change from the old wrapper, which returned 404 / 409 / 412 / 429 as a successful `CosmosResponse`). Accordingly:

- The wrapper does **not** return 404 / 409 / 412 / 429 as `COSMOS_ERROR_CODE_SUCCESS` with a non-2xx `cosmos_response_status_code`. Those surface as completion outcome = ERROR with `cosmos_error_is_from_wire(err) == true` and the appropriate `cosmos_error_status_code(err)`.
- Host SDKs implement "expected 404" semantics by checking `cosmos_error_is_not_found(err)`; "expected 412 / 409" via `cosmos_error_is_precondition_failed(err)` / `cosmos_error_is_conflict(err)`; 429 retry-after via `cosmos_error_is_throttled(err)` + `cosmos_error_retry_after_ms(err)`. See §3.5.2 for the full accessor surface.
- The auth / metadata initialization path can still surface auth-bootstrap failures before the operation is dispatched (`src/driver/cosmos_driver.rs:1104-1114`); those propagate through the same `CosmosError` channel — typically with `cosmos_error_is_from_wire(err) == false` and a synthetic `CosmosStatus::AUTHENTICATION_TOKEN_ACQUISITION_FAILED` carrying sub-status `20402`.

### 6.3 `cosmos_error_code_t` ↔ `cosmos_error_t` mapping

When `execute_operation` returns `Err(CosmosError)`, the wrapper picks a coarse `cosmos_error_code_t` based on the `(status_code, sub_status)` pair *and* always populates the rich `cosmos_error_t` for full detail. The routing rule below is *not* a strict pattern-match — the wrapper checks the more specific conditions (sub-status presence, synthetic vs. wire) first, then falls through to the generic HTTP-status branch:

| Condition (matched top-to-bottom) | Coarse `cosmos_error_code_t` |
|---|---|
| `is_from_wire == false` and `sub_status == AUTHENTICATION_TOKEN_ACQUISITION_FAILED (20402)` | `COSMOS_ERROR_CODE_AUTHENTICATION_FAILED` |
| `is_from_wire == false` and `sub_status ∈ 20010..=20015 ∪ {20003}` (transport synthesized) | `COSMOS_ERROR_CODE_TRANSPORT_FAILURE` |
| `is_from_wire == false` and `sub_status == CLIENT_OPERATION_TIMEOUT (20008)` | `COSMOS_ERROR_CODE_CLIENT_OPERATION_TIMEOUT` |
| `is_from_wire == false` and `sub_status == SERIALIZATION_RESPONSE_BODY_INVALID (20020)` | `COSMOS_ERROR_CODE_SERIALIZATION_FAILED` |
| `is_from_wire == false` (everything else — generic synthetic) | `COSMOS_ERROR_CODE_CLIENT_ERROR` |
| Wire status 429 | `COSMOS_ERROR_CODE_THROTTLED` |
| Wire status 404 | `COSMOS_ERROR_CODE_NOT_FOUND` |
| Wire status 409 | `COSMOS_ERROR_CODE_CONFLICT` |
| Wire status 412 | `COSMOS_ERROR_CODE_PRECONDITION_FAILED` |
| Wire status 408 | `COSMOS_ERROR_CODE_TIMEOUT` |
| Wire status 410 | `COSMOS_ERROR_CODE_GONE` |
| Wire status 401 | `COSMOS_ERROR_CODE_UNAUTHORIZED` |
| Wire status 403 (incl. `WRITE_FORBIDDEN`) | `COSMOS_ERROR_CODE_FORBIDDEN` |
| Wire status 400 | `COSMOS_ERROR_CODE_BAD_REQUEST` |
| Wire status 503 (driver-classified, **not** transport-synthesized) | `COSMOS_ERROR_CODE_SERVICE_UNAVAILABLE` |
| Any other wire status (5xx, unmapped 4xx) | `COSMOS_ERROR_CODE_SERVICE_ERROR` |

The coarse code is **only** for the common dispatch path (switch in C, "expected vs unexpected" branch). All structured detail — including the synthetic sub-status codes — lives on `cosmos_error_t` and is the source of truth. Note in particular that the wrapper does **not** introduce a `COSMOS_ERROR_CODE_CONFIGURATION_ERROR` distinct from `COSMOS_ERROR_CODE_CLIENT_ERROR`: the merged driver does not surface a configuration-vs-client distinction at the error-type level, so the wrapper does not invent one. Configuration-time failures (invalid endpoint URL, missing credential parts, etc.) surface as the pre-flight 4014 / 4015 codes from §3.5.1 plus the rich `cosmos_error_t` with `is_from_wire == false`.

### 6.4 Backtrace rate-limit knobs

The driver's `CosmosError` carries an optional backtrace whose capture rate **and** rendering rate are independently bounded — capture (collecting the raw stack) and resolution (symbolicating it) are different operations with different costs. The merged driver exposes both knobs as a **process-global** function:

```rust
pub fn azure_data_cosmos_driver::error::set_backtrace_options(BacktraceOptions {
    max_captures_per_second: u32,    // 0 disables capture
    max_resolutions_per_second: u32, // 0 disables resolution
});
```

The limits are process-global atomics inside the error-construction path (see [`src/error/backtrace.rs`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/src/error/backtrace.rs)); they are *not* attached to `CosmosDriverRuntimeBuilder` or any driver instance, because per-runtime state on the hot error-construction path is not viable. The wrapper therefore exposes a single module-scope C entry point that mirrors the driver function 1:1:

```c
/* Process-global. Last-writer-wins semantics across concurrent calls. Pass
 * 0 to either parameter to disable that knob. Environment-derived defaults
 * (RUST_LIB_BACKTRACE, RUST_BACKTRACE, AZURE_COSMOS_BACKTRACE_*) are
 * **overridden** for the rest of the process once this is called — see the
 * `set_backtrace_options` Rust docs for the precedence rule. */
void cosmos_set_backtrace_options(uint32_t max_captures_per_second,
                                  uint32_t max_resolutions_per_second);
```

There are no per-runtime / per-driver setters — host SDKs that need to gate backtraces per workload should do so at the host-SDK layer, not via the FFI.

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

### Phase 1 — Error + async invocation primitives *(Goal: reusable plumbing for every later API)*

- Port `CosmosError` / `CosmosErrorCode` / `Error` (extended with the new 40xx codes from §3.5.1 — including `QUEUE_SHUTDOWN` and `OPERATION_CANCELLED`).
- Implement the rich `cosmos_error_t` accessor + predicate surface from §3.5.2 against `azure_data_cosmos::Error` (from PR #4442).
- Implement `CompletionQueue` + `OperationHandle` + `Completion` Rust types and their Tokio-backed delivery channel. Public C entry points (`cosmos_cq_create`, etc.) require a runtime and therefore land green only after Phase 2 wires the runtime in — Phase 1 ships the plumbing + a workspace-internal test scaffold that constructs a stub `RuntimeContext` so the queue / completion / handle types can be exercised in isolation.
- `cosmos_cq_create` / `_free` / `_wait` / `_try_wait` / `_wait_batch` / `_wait_writable` / `_shutdown` / `_state`.
- `cosmos_completion_*` accessors (outcome / status / user_data / op_handle / was_cancel_requested / take_response / take_error / response / error / free).
- `cosmos_operation_handle_cancel` / `_state` / `_free`.
- Error-handling C tests (null-pointer rejection, error-detail string lifecycle, `cosmos_error_*` accessor coverage, forward-compat behavior for sub-status values introduced after this spec).

**Done when:** Calling APIs with `NULL` runtime, `NULL` queue, etc. produce the right pre-flight `cosmos_error_code_t`; submitting and dequeuing a synthetic completion round-trips `user_data` and the rich error payload; every `cosmos_error_is_*` predicate is exercised by at least one synthetic test (no emulator needed).

### Phase 2 — Runtime *(Goal: a single shared `CosmosDriverRuntime` reachable from C)*

- Implement `runtime/tokio.rs`: owns a `Runtime` **and** an `Arc<CosmosDriverRuntime>` built via `CosmosDriverRuntimeBuilder::new().build()`.
- Expose `cosmos_runtime_builder_*` mirror of `CosmosDriverRuntimeBuilder` (workload id, correlation id, user-agent suffix, connection pool options, operation-option defaults, worker thread count, **Tokio thread-name prefix** introduced in #4452, `allow_emulator_invalid_certs` — see §4.1 / §4.2 on why this lives on the runtime and not on `DriverOptions`), plus `cosmos_runtime_builder_new` / `_free` / `_build` and `cosmos_runtime_free`.
- Expose `cosmos_set_backtrace_options(max_captures_per_second, max_resolutions_per_second)` per §6.4 (process-global, no per-runtime variant on the merged driver).
- `c_tests/runtime_lifecycle.c`: create/free runtime in loop; submit-and-drain a synthetic op against a `cosmos_cq_t` from multiple producer threads with a single consumer; verify clean shutdown via `cosmos_cq_shutdown` + drain.

**Done when:** Multiple producer threads can submit against one `cosmos_cq_t` while a single consumer drains, and `cosmos_cq_shutdown` cleanly cancels in-flight ops and drains the queue.

### Phase 3 — Account / resource references + driver instance *(Goal: open a connection to a real Cosmos account)*

- `cosmos_account_ref_with_master_key`, `with_credential`, `with_resource_token`, `clone`, `free`.
- `cosmos_database_ref_create`, `cosmos_container_ref_create`, matching `_clone` and `_free`.
- `cosmos_driver_options_builder_*` — the actual 3-field surface from §4.2 (`new(account)`, `with_preferred_regions`, `with_operation_options`, `build`). Per-call options (`excluded_regions`, `read_consistency`, `content_response_on_write`, etc.) live on `cosmos_operation_options_*` and are wired in Phase 5.
- `cosmos_driver_get_or_create_submit` → wraps `runtime.get_or_create_driver(account, options).await` and delivers the result as a completion (`cosmos_response_take_driver` on the response). The cache-hit advisory `COSMOS_ERROR_CODE_OPTIONS_IGNORED_ON_CACHE_HIT` (`5001`, non-SUCCESS) per §4.4.1 is surfaced through `cosmos_completion_status` + `cosmos_completion_take_error` on the OK completion.
- `cosmos_driver_get_or_create_blocking` — synchronous convenience for startup-init paths (submits to a private internal queue and drains one completion). Cache-hit advisory surfaces via the return code + `out_error` slot.
- `cosmos_driver_initialize_submit` for the explicit re-initialization path (§4.4). Normally unnecessary because `_get_or_create_*` already awaits `initialize()`.
- `cosmos_response_take_driver` (§4.4) extracts the driver handle from the degenerate response delivered by `cosmos_driver_get_or_create_submit`.
- Backtrace tuning ships in Phase 2 (§6.4 is process-global, not per-driver); no per-driver backtrace setter exists.
- `cosmos_driver_free`.
- `c_tests/driver_init.c`: emulator-backed test that creates a driver (via `_blocking` for setup ergonomics), then runs a second `_submit` against the same endpoint with non-NULL `options` and asserts the cache-hit advisory fires on the delivered completion (`status == 5001`, `outcome == OK`, advisory rich error present).

**Done when:** A C test against the emulator can stand up a `cosmos_driver_t`, free it, recreate it, observe the cached instance, and observe the `OPTIONS_IGNORED_ON_CACHE_HIT` advisory through both the `_blocking` and `_submit` paths.

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

- `cosmos_driver_submit` binds to `execute_singleton_operation` per §4.7 / §6.1; delivers `cosmos_response_t` on `OK` completions and the rich `cosmos_error_t` on `ERROR` completions.
- `cosmos_response_*` accessors (body view, into_body, status, RU charge, activity id, ETag, session token, continuation token, header iteration).
- `c_tests/item_crud.c`: create / read / replace / upsert / delete / patch against the emulator, driven by a single shared `cosmos_cq_t` and a dedicated receive-loop helper thread that translates completions into per-call C condition-variable signals.
- Cross-partition `read_all_items` / `query_items` / `batch` are **not** wired into `cosmos_driver_submit` in this phase — they belong to Phase 8 (pager) and Phase 9 (batch).

**Done when:** Full single-item CRUD passes against the emulator; per §6.2, a 404 read-after-delete surfaces as a completion whose `outcome == ERROR` with `cosmos_error_is_from_wire(err) == true`, `cosmos_error_status_code(err) == 404`, and `cosmos_error_is_not_found(err) == true` — **not** as `OK` with a 404 response.

### Phase 7 — Diagnostics surface *(Goal: language SDKs can log + emit OTel from C)*

- `cosmos_diagnostics_*` accessors per §4.8.
- `cosmos_diagnostics_to_json` (the one sanctioned JSON producer in the wrapper, gated behind the diagnostics feature).
- `c_tests/diagnostics.c`: asserts RU charge matches the response's, retry count is sane, region-contacted callback fires at least once.

**Done when:** A `to_json` snapshot can be diff'd against the Rust driver's `DiagnosticsContext::Debug` output and matches structurally.

### Phase 8 — Pagination (read-feeds & query) *(Goal: handle multi-page responses)*

- Async pager handle: `cosmos_pager_t`, with:
  - `cosmos_driver_submit_pager` → returns `cosmos_operation_handle_t*`; first completion delivers a `cosmos_response_t` from which `cosmos_response_take_pager` extracts the `cosmos_pager_t*`.
  - `cosmos_pager_next_submit(pager, queue, user_data, &pre_err)` → returns `cosmos_operation_handle_t*`; each page lands as its own completion (response per page; `COSMOS_ERROR_CODE_FEED_EXHAUSTED` on the completion that drains the pager).
  - `cosmos_pager_continuation_token` (borrowed view; sync accessor).
  - `cosmos_pager_free`.
- Initially supports server-side cross-partition + single-partition queries that don't require local plan execution.
- `c_tests/pagination.c`: emulator-backed read-all + simple query test driving a multi-completion receive loop.

**Done when:** A query that spans multiple pages collects all results across page boundaries; partial-page resumption via continuation token works.

### Phase 9 — Patch & transactional batch *(Goal: parity with the driver's specialty operations)*

- Expose patch via the existing operation factory + `cosmos_operation_with_patch_max_attempts`, but add helpers to build patch documents as raw bytes (caller provides the JSON; wrapper just wraps it in a body).
- TransactionalBatch: opaque builder (`cosmos_batch_t`) + per-op append + `cosmos_driver_submit_batch` (async submit; result delivered as a completion). Body marshalling is bytes-only, consistent with §3.3.

**Done when:** Emulator-backed C tests pass for: add/remove/replace/set/incr patch ops, and a 5-operation batch (create + replace + delete) with both success and 412-precondition-failed batch outcomes.

### Phase 10 — Optional advanced surface *(Goal: opt-in features that aren't required for parity)*

Each item below is independent; ship as feature-gated when ready.

- **Fault injection** (`fault_injection` feature): minimal handle to register a single rule, drop rules. Useful for cross-language SDK reliability tests.
- **In-memory emulator** (`__internal_in_memory_emulator` feature, test-only): not exported through the public C header; used by `c_tests/` to run without an external emulator.
- **Tracing initialization**: `cosmos_enable_tracing()` ported as-is from the old wrapper.

---

## 9. Open Questions

1. **Single-runtime-per-process enforcement.** The driver's `CosmosDriverRuntime` is process-cardinal in the spirit of `ARCHITECTURE.md`; do we enforce that at the wrapper boundary (rejecting a second `cosmos_runtime_builder_build` with a dedicated `cosmos_error_code_t`), allow N for testing convenience, or rely solely on host-SDK discipline (the §4.1 cardinality advisory)? *Not yet decided — §3.5.1 does not pre-allocate a code for the enforcement variant.* (Note: the §4.4.1 `OPTIONS_IGNORED_ON_CACHE_HIT` advisory is **no longer** predicated on this question — it fires unconditionally whenever a cache hit is observed with non-NULL `options`, regardless of runtime cardinality.)
2. **Forward-compat for unknown response headers.** §4.7 freezes the response-header surface as a curated set of typed accessors (`activity_id`, `session_token`, `etag`, `continuation_token`, `request_charge`, plus per-feature additions). Unknown headers are dropped — a host-SDK author asking "what is the equivalent of `IDictionary<string, string> ResponseHeaders` in .NET?" gets nothing. Do we (a) keep the strict typed surface and add a public driver-side `Response::custom_headers()` extension point as features need it, (b) expose a wrapper-only forward-compat passthrough (`cosmos_response_get_custom_header(name)` returning a `cosmos_string_view_t`) populated from a wrapper-maintained passlist of header names the driver does not yet model, or (c) leave it strict and document the limitation? Decide before the response-accessor surface is frozen. (Previously this slot asked about borrow-vs-copy semantics for an `iter_headers` visitor; that visitor was removed in this revision — see §4.7 — so the question is now about the *shape* of the extension story rather than the iteration mechanics.)
3. **Continuation token format.** The driver currently treats continuation tokens as opaque strings. If `azure_data_cosmos` moves to byte-level tokens (binary encoding), the C API should expose them via `cosmos_bytes_view_t` rather than `const char *`. Decide before Phase 8 — the choice is locked in once `cosmos_response_continuation_token` ships.
4. **Multi-part response bodies.** `azure_data_cosmos::CosmosResponse::ResponseBody::Items` carries an iterator over multiple parts. `cosmos_response_body` currently exposes a single `cosmos_bytes_view_t`. Options: (a) collapse parts into one buffer (allocates + copies); (b) expose a dedicated `cosmos_response_iter_items(visitor, user_data)`; (c) defer multi-part to the pager surface only. Pick before Phase 8.
5. **C++ companion header.** cbindgen's `cpp_compat = true` is sufficient for C++ consumers using the C API directly; do we also want a thin `azurecosmosdriver.hpp` with RAII handle wrappers (one struct per `_t`, dtor wired to the matching `_free`, `unique_ptr` semantics)? Probably out-of-scope for v1, but record the decision so host SDKs can plan around it.
6. **Driver cache scoping vs. credential identity.** §4.4.1 documents that two `cosmos_account_ref_t`s sharing an endpoint but using different credentials collide in the cache. Should the wrapper teach the driver to incorporate credential identity (e.g. via `TokenCredential::token_provider_kind()` or a wrapper-supplied opaque tag) into the cache key? If not, document the per-runtime-per-credential workaround prominently in the host-SDK author guide.
7. **ConnectionString parser ownership.** The driver's `ConnectionString` parser currently lives in `azure_data_cosmos`. If the wrapper wants to expose `cosmos_account_ref_from_connection_string(const char *cs, ...)`, do we mirror the parser in the wrapper (extra surface to maintain) or depend on the driver re-exporting the parser publicly?
8. **Symbol stripping on release builds.** The old crate used `Box::into_raw` and friends. We may want `-Cstrip=symbols` on release builds; verify that doesn't trip up corrosion's debug-symbol discovery on Windows, or interact badly with backtrace capture (§6.4).
9. **`cosmos_pager_t` continuation-token re-entry contract.** When a pager is freed mid-iteration, the caller may want to recreate it from a continuation token. Decide whether `cosmos_driver_submit_pager` takes an optional starting continuation token, or whether resumption is always operation-level via `cosmos_operation_with_continuation_token`. Resolve before Phase 8.
10. **`PartitionKeyValue::Infinity` visibility.** The driver's `Infinity` variant is `pub(crate)` (`models/partition_key.rs:44`) and documented as "used only internally for EPK boundary calculations." §4.5 of this spec therefore does **not** ship `cosmos_partition_key_builder_append_infinity` in v1; EPK upper-bound construction is deferred to the feed-range surface (§4.6.4). Decide whether to (a) promote `Infinity` to `pub` in the driver with appropriate use-case documentation, or (b) keep it private and route every EPK-boundary use case through `cosmos_feed_range_*` constructors going forward. The choice affects whether language SDKs ever need a direct partition-key sentinel for boundary semantics; if (b), §4.5 stays as-is permanently.
11. **`FeedRange` v1 constructor gaps.** §4.6.4 ships only the two driver-public constructors today (`FeedRange::full()` and `FeedRange::for_partition(...)`). Two construction shapes that callers may want are deferred because the driver does not expose them in a string-parseable form:
    - **EPK-range construction.** `FeedRange::new(min: EffectivePartitionKey, max: EffectivePartitionKey)` exists on the driver (`feed_range.rs:71`) but takes strongly-typed EPK values, not strings. Decide whether to (a) wait for a driver-side `FeedRange::from_hex_range(min_hex, max_hex)` parser, or (b) add a `cosmos_effective_partition_key_t` opaque type to the wrapper with its own `_from_hex` / `_min` / `_max` constructors. Phase 8 (pager) is the natural forcing function — if continuation-token resumption is operation-level (Q9 option B), this gap may never bite.
    - **Physical-partition-range targeting.** The driver's `FeedRangeRepr` has no `PartitionKeyRangeId` variant; PKRangeId-keyed routing happens elsewhere (PPAF/PPCB). Decide whether host SDKs that want explicit physical-partition pinning should drive that through some other surface, or whether the driver should grow a `FeedRange::for_partition_key_range_id(pkrange_id)` constructor that the wrapper can mirror.
12. **Multi-consumer (MPMC) `cosmos_cq_t`.** §3.1.2 / §3.1.3 ship the queue as multi-producer / single-consumer in v1, with the explicit workaround of "one queue per consumer" for work-stealing. Decide whether a future revision promotes the queue to MPMC (internal lock around the consumer side; lets host SDKs spin N receive-loop threads against one queue) or keeps the v1 contract permanently. The decision affects whether `cosmos_cq_wait` ever needs a fairness / batching mode and whether per-completion ordering across consumers needs to be specified.
13. **Driver-side `CancellationToken` primitive.** §3.6.3 documents that v1 cancellation is implemented in the wrapper layer via `tokio::select!` + future-drop, because `CosmosDriver::execute_operation` / `execute_singleton_operation` (`src/driver/cosmos_driver.rs:1242,1281`) and `execute_plan` (`:1313`) do **not** currently accept a `CancellationToken`. A first-class `execute_*(op, options, cancel: CancellationToken)` overload — checked at every `await` boundary in `pipeline/components.rs` and `transport_pipeline.rs`, alongside the existing `deadline: Option<Instant>` — would let the wrapper retire its drop-based shim, surface mid-flight cancellation diagnostics, and give the host SDK author a "cancelled" outcome carrying the partial `DiagnosticsContext`. Decide whether to land this driver-side change in the same release as the wrapper (so the §3.6.3 caveats become historical) or defer it to a v2 revision. The wrapper API surface does not change either way — only the §3.6.3 caveat list shrinks.
14. **Pager ownership: wrapper-side vs. driver-side.** §4.7 documents that `cosmos_pager_t` is a wrapper-owned opaque type built on top of `(OperationPlan, execute_plan)`, because the driver crate has no `Pager` / `PageStream` type. Two consequences: (a) the wrapper has to keep the originating `CosmosDriver` Arc-cloned for the pager's lifetime so `execute_plan` can re-enter the driver per page, and (b) any host SDK that wants prefetched/pipelined pages has to issue multiple pagers because a single `cosmos_pager_t` is strictly sequential. Decide whether to (i) keep the wrapper-side pager as the long-term contract, (ii) push for a driver-side `Pager` type that owns the `OperationPlan` and exposes `async fn next_page()` (cleaner ownership, opens the door to driver-side prefetch), or (iii) wait for the streaming-feed redesign tracked elsewhere. Resolve before Phase 8 ships.
15. **Runtime API: keep builder, or eventually add a fast-path constructor?** §4.1 ships only the `cosmos_runtime_builder_*` family — there is no `cosmos_runtime_create(options)` shortcut, because `CosmosDriverRuntimeBuilder::build()` is async + does network I/O and the runtime carries too many knobs for a flat options struct. For host SDKs that pass nothing but defaults the builder is a 3-call dance (`_new` / `_build` / `_free`). Decide whether to (a) leave the builder as the only entry point permanently — clean, mirrors the driver crate — or (b) once the runtime stabilizes, add a `cosmos_runtime_default(out_runtime, out_error)` convenience that internally does `_new` + `_build`. Not blocking on v1; revisit when host-SDK author feedback arrives.
16. **Mid-flight diagnostics snapshot.** §3.6.2 removes the v0 `cosmos_operation_handle_diagnostics_snapshot` accessor because the driver's `DiagnosticsContext` is constructed inside the executing future's stack frame, not in a shared `Arc<Mutex<…>>` the wrapper can clone while the future is still running (`CosmosResponse::diagnostics()` at `models/cosmos_response.rs:109` is only reachable post-completion). Two driver-side options would re-enable mid-flight snapshots: (a) refactor the pipeline to thread an `Arc<RwLock<DiagnosticsContext>>` (or equivalent lock-free append-only structure) through every stage, allowing concurrent readers, or (b) emit a snapshot eagerly to a per-operation shared-memory ring buffer the wrapper exports. Decide whether stuck-op observability is a v1.x must-have (and which driver-side mechanism is preferred) or a v2 nice-to-have. Until resolved, host SDKs that need stuck-op visibility must rely on `cosmos_operation_handle_state` polling plus external instrumentation (request timestamps captured at submit time).
17. **Driver-owned buffer pool for request bodies.** §3.3 / §4.6.2 currently specify that `cosmos_operation_with_body` **copies** the caller's `cosmos_bytes_view_t` into wrapper-owned storage, because that contract interoperates cleanly with host-language GCs (pinned .NET buffers, Java `DirectByteBuffer`, Go `[]byte`, etc.) — the caller can release the source memory immediately. Both reviewers (Ashley #12, FabianMeiswinkel #16) flagged that for high-traffic workloads this copy is the obvious hot spot, and that a **driver-owned buffer pool** would let host SDKs write request bodies directly into pool-allocated memory and hand the buffer back via FFI without an intermediate copy. The conceptual API shape:

    ```c
    /* Acquire a writable buffer from the driver's per-driver (or per-runtime?
     * — open sub-question) body buffer pool. Returns a wrapper-owned buffer
     * the caller can write into for up to `min_capacity` bytes. */
    cosmos_error_code_t cosmos_driver_acquire_request_buffer(
        const cosmos_driver_t *driver,
        size_t min_capacity,
        cosmos_request_buffer_t **out_buffer,
        cosmos_error_t *out_error);

    /* Borrow the writable region. Lifetime = until cosmos_operation_with_body_pooled
     * consumes the buffer, OR cosmos_request_buffer_release returns it to the pool
     * without submitting. */
    uint8_t *cosmos_request_buffer_data(cosmos_request_buffer_t *b);
    size_t   cosmos_request_buffer_capacity(const cosmos_request_buffer_t *b);
    cosmos_error_code_t cosmos_request_buffer_set_len(
        cosmos_request_buffer_t *b, size_t actual_len);

    /* Attach the pooled buffer as the operation's body. The buffer is consumed
     * (Box<Option<...>> sentinel pattern, like cosmos_operation_t in §4.6.3) —
     * ownership transfers to the operation, which returns it to the pool when
     * the operation completes (success / error / cancel — all three paths). */
    cosmos_error_code_t cosmos_operation_with_body_pooled(
        cosmos_operation_t *op, cosmos_request_buffer_t *buf);

    /* Return a pool buffer that won't be submitted (caller decided not to send). */
    void cosmos_request_buffer_release(cosmos_request_buffer_t *b);
    ```

    Open sub-questions: (a) pool granularity — per-driver, per-runtime, or process-global? (b) pool eviction policy — bounded LRU per size class, or simple bucketed free-list? (c) interaction with the existing `cosmos_bytes_view_t` path — keep both as parallel APIs, or deprecate the copy path once the pool stabilizes? (d) similar story on the **response** side: today `cosmos_response_body` returns a borrowed view that lives until `cosmos_response_free` (one allocation per response), and `cosmos_response_into_body` takes ownership of a `cosmos_bytes_t` — a response-side pool could let host SDKs return the buffer to the pool explicitly via `cosmos_response_release_body_to_pool(...)` instead of `cosmos_bytes_free`. This is a v1.x feature, not v1; the v1 contract per §3.3 + §4.6.2 (copy on `cosmos_operation_with_body`) is stable and host SDKs can build on it today.

18. **Driver→SDK logging callback.** FabianMeiswinkel (review comment #17) raised that the driver currently logs via the `tracing` crate, but host SDK customers expect driver-emitted logs to appear in **their** logger (.NET `ILogger`, Java SLF4J, Go `slog`, Python `logging`, etc.), with their formatting, sink, and filtering. The wrapper today exposes `cosmos_enable_tracing()` (Phase 10) which just bootstraps the driver's internal `tracing-subscriber` and writes to stderr — that is not what host SDKs want. The conceptual API shape:

    ```c
    typedef enum cosmos_log_level {
        COSMOS_LOG_LEVEL_TRACE = 0,
        COSMOS_LOG_LEVEL_DEBUG = 1,
        COSMOS_LOG_LEVEL_INFO  = 2,
        COSMOS_LOG_LEVEL_WARN  = 3,
        COSMOS_LOG_LEVEL_ERROR = 4,
    } cosmos_log_level_t;

    /* Callback invoked from inside the Tokio runtime on whatever worker
     * thread emitted the log record. Host SDKs must NOT block in this
     * callback — push records onto a host-side MPSC channel and have a
     * separate thread drain them into the host logger. */
    typedef void (*cosmos_log_callback)(
        void *user_data,
        cosmos_log_level_t level,
        const char *target,         /* tracing target, NUL-terminated UTF-8 */
        const char *message,        /* rendered message, NUL-terminated UTF-8 */
        const char *fields_json,    /* tracing fields as JSON object, NUL-terminated; NULL when no fields */
        uint64_t timestamp_nanos);  /* nanoseconds since UNIX epoch */

    /* Install a host-provided log sink for the runtime. Replaces any previously
     * installed sink. NULL callback uninstalls the sink (falls back to the
     * driver's default tracing-subscriber, or silence if cosmos_enable_tracing
     * was not called). */
    cosmos_error_code_t cosmos_runtime_set_log_callback(
        cosmos_runtime_t *runtime,
        cosmos_log_callback callback,
        void *user_data,
        void (*user_data_free)(void *user_data),
        cosmos_log_level_t min_level);
    ```

    Open sub-questions: (a) sink scope — per-runtime (proposed above) or per-driver? Per-runtime is simpler because the `tracing` subscriber is process-global today; per-driver would require a custom subscriber that demultiplexes by span context. (b) field marshalling — JSON (proposed) or a structured visitor callback? JSON is straightforward but adds an alloc per record. (c) overhead — `tracing` already gates by level cheaply when no subscriber is interested; this callback would need to be similarly cheap when the host SDK installs a `WARN` filter and the driver emits `DEBUG`. (d) interaction with `cosmos_enable_tracing()` (Phase 10) — does installing a callback uninstall the default subscriber, or are they composed? Decide before Phase 10 ships, because shipping `cosmos_enable_tracing()` first without thinking about the callback path locks the wrapper into a global-subscriber model that's harder to retrofit later.

19. **`cosmos_operation_options_t` shape: builder vs. flat C struct.** §4.2 and §4.6.2 / Phase 5 currently mirror the driver's `OperationOptionsBuilder` 1:1 (17+ `cosmos_operation_options_with_<field>` setters). Ashley (review comment #11) and FabianMeiswinkel (#15) both flagged that for a **per-call** options bag — invoked once per CRUD operation in tight loops — the builder pattern is the worst-case FFI shape: 17 round-trips per submit, all to write into wrapper-side heap memory the driver then has to copy back into a Rust struct. The conceptual alternative is a **flat, ABI-stable C struct** that the host SDK populates in its own memory and passes by pointer to `cosmos_driver_submit`:

    ```c
    /* Layout is published and stable. Field reordering / removal is an ABI
     * break (requires major version bump per §7). NEW fields are appended at
     * the end of the struct and protected by an explicit version word; older
     * consumers reading a newer struct are bounded by `struct_size_bytes`. */
    typedef struct cosmos_operation_options_v1 {
        uint32_t struct_size_bytes;            /* sizeof(this) at consumer compile time */
        uint32_t flags;                        /* bitmask of which optional fields are set; one bit per Option<T> driver field */

        /* Consistency / regions */
        cosmos_read_consistency_t consistency_level;     /* see §3.5 enum; flag bit 0 = present */
        const char *session_token;                       /* NUL-terminated; flag bit 1 = present */
        const char *const *excluded_regions;             /* array of NUL-terminated strings; flag bit 2 = present */
        size_t excluded_regions_len;

        /* Response shape & paging */
        bool content_response_on_write;                  /* flag bit 3 = present */
        cosmos_priority_level_t priority_level;          /* flag bit 4 = present */
        uint64_t end_to_end_timeout_millis;              /* flag bit 5 = present */

        /* Throughput control & QoS */
        const char *throughput_control_group_name;       /* flag bit 6 = present */

        /* Per-partition circuit-breaker (8 knobs) */
        bool partition_level_circuit_breaker_enabled;    /* flag bit 7 = present */
        uint32_t partition_level_cb_failure_threshold;   /* flag bit 8 */
        uint64_t partition_level_cb_recovery_window_ms;  /* flag bit 9 */
        uint32_t partition_level_cb_min_request_volume;  /* flag bit 10 */
        uint32_t partition_level_cb_consecutive_failures_threshold;  /* flag bit 11 */
        uint64_t partition_level_cb_open_window_ms;      /* flag bit 12 */
        uint32_t partition_level_cb_half_open_max_requests;  /* flag bit 13 */
        double   partition_level_cb_failure_rate_threshold;   /* flag bit 14 */

        /* Retry tuning */
        uint32_t max_failover_retry_count;               /* flag bit 15 */
        uint32_t max_session_retry_count;                /* flag bit 16 */

        /* Session capture */
        bool session_capturing_disabled;                 /* flag bit 17 */

        /* Region availability */
        uint64_t endpoint_unavailability_ttl_ms;         /* flag bit 18 */

        /* Custom headers — flat C array of (name, value) pairs */
        const cosmos_header_pair_t *custom_headers;      /* flag bit 19 = present */
        size_t custom_headers_len;
    } cosmos_operation_options_v1_t;

    typedef struct cosmos_header_pair {
        const char *name;   /* NUL-terminated UTF-8 */
        const char *value;  /* NUL-terminated UTF-8 */
    } cosmos_header_pair_t;
    ```

    Submit signature would become:

    ```c
    cosmos_operation_handle_t *cosmos_driver_submit(
        const cosmos_driver_t *driver,
        cosmos_operation_t *op,
        const cosmos_operation_options_v1_t *options,   /* flat struct, by pointer; nullable */
        cosmos_cq_t *queue,
        void *user_data,
        cosmos_error_code_t *out_pre_error);
    ```

    The driver crate already has `OperationOptions` as a plain Rust struct (`src/options/operation_options.rs:41-188`), so the wrapper's job becomes "copy each present flag/field from the C struct into the Rust struct" on a single FFI boundary crossing — instead of 17 boundary crossings. Fabian's variant (#15) goes further: **the host SDK** merges runtime / driver / per-call defaults into one struct, and the wrapper just hands that struct verbatim to `execute_operation`. The wrapper would then **stop** exposing `cosmos_runtime_set_operation_options_default` / `cosmos_driver_options_builder_with_operation_options` as separate plumbing — those layering concerns move into the host SDK.

    Open sub-questions: (a) flag-bit layout — bitmask (proposed) or `Option<>`-style sentinel (e.g. `UINT32_MAX` means "unset")? Bitmask is more compact and avoids "is this a real value or a sentinel?" ambiguity for fields whose valid range includes the sentinel. (b) ABI evolution — `struct_size_bytes` + appended-field rule (proposed) or v2/v3 versioned structs? The size-prefix rule is forward-compatible without requiring host-SDK recompiles, but requires every wrapper-side reader to bounds-check. (c) string lifetime — caller-owned for the duration of the synchronous `cosmos_driver_submit` call (and the wrapper copies into the operation's `OperationOptions`), matching the §3.3 rule for `cosmos_bytes_view_t`. (d) preserve the builder API as a convenience for non-hot-path startup options (driver init) — yes; `cosmos_driver_options_builder_*` stays as-is because it fires once per process. (e) timing — Phase 5 currently freezes the builder API; if the flat struct is the desired contract, Phase 5 should ship the flat struct **instead of** the builder, not in addition (decide before Phase 5 starts to avoid ABI churn).

20. **Handle table for value-type references.** §3.4 documents that `cosmos_account_ref_t*` / `cosmos_database_ref_t*` / `cosmos_container_ref_t*` / `cosmos_partition_key_t*` / `cosmos_feed_range_t*` are heap-allocated handles, and §3.4 (post-#14 revision) notes they're immutable post-build so concurrent reads / clones / frees of **distinct** FFI handles are race-free. Ashley (review comments #5 and #6) flagged a remaining ergonomic / safety concern: nothing in the C ABI prevents a caller from copying a `cosmos_account_ref_t*` value (a raw pointer) around their codebase, leading to (a) double-free if two copies are both `_free`'d, (b) use-after-free if one copy is `_free`'d while another is in use, and (c) (for any future stack-allocated value handles) dangling pointers from value moves. The proposed mitigation is a **handle table**: the wrapper stores `Arc<…>`s in a process-global slab, and the C ABI hands out **integer indices** (`uint64_t handle_id` plus a generation counter to detect use-after-free) instead of raw pointers:

    ```c
    typedef uint64_t cosmos_handle_id_t;   /* low 48 bits: slab slot; high 16 bits: generation */

    cosmos_handle_id_t cosmos_account_ref_with_master_key_h(...);
    /* All subsequent APIs take cosmos_handle_id_t instead of cosmos_account_ref_t*. */
    cosmos_error_code_t cosmos_database_ref_create_h(
        cosmos_handle_id_t account_id, const char *db_id,
        cosmos_handle_id_t *out_db_id);
    /* _free is idempotent (subsequent _free calls on the same id are no-ops),
     * AND lookups on a freed id return a dedicated COSMOS_ERROR_CODE_INVALID_HANDLE
     * (rather than UB) thanks to the generation counter. */
    void cosmos_account_ref_free_h(cosmos_handle_id_t id);
    ```

    Trade-offs: (+) catches double-free / use-after-free / accidental pointer-copy aliasing without UB. (+) opens the door to stack-allocated value types in the future (the C side never sees the heap address). (−) every wrapper API call pays a slab lookup (one branch + one indirection — small but not zero, and harder to inline across the FFI boundary than a direct pointer deref). (−) doubles the number of APIs the wrapper has to expose during the transition (`_h` variants alongside the existing pointer variants), or breaks ABI by switching the whole surface over. (−) the slab is a process-global mutable shared structure — needs sharded RwLock or lock-free design to avoid becoming the FFI bottleneck.

    Decide whether to (a) ship the v1 pointer-based surface (current spec) and add `_h` variants in a future revision if real-world use shows pointer-aliasing bugs in host SDKs, (b) switch the entire reference / partition-key / feed-range surface to handle-IDs in v1 (one-time ABI churn but cleaner long-term), or (c) hybrid — pointer-based for handles whose lifetime is single-owner-and-short (operations, completions, errors), handle-table for value types that callers might naturally want to copy around (`cosmos_account_ref_t`, `cosmos_partition_key_t`). Resolve before Phase 3 freezes the reference API.

---

## 10. Migration Notes from the Old Wrapper

For anyone consulting the deleted `azure_data_cosmos_native` crate as a reference:

| Old (`azure_data_cosmos_native`) | New (`azure_data_cosmos_driver_native`) |
|---|---|
| `cosmos_client_create_with_key` | `cosmos_account_ref_with_master_key` + `cosmos_driver_get_or_create_submit` (or `_blocking` for startup-init) |
| `cosmos_client_database_client` | `cosmos_database_ref_create` (no network call) |
| `cosmos_database_create_container` | `cosmos_operation_create_container(database, container_id, &op)` → `cosmos_operation_with_body(op, body_view)` → `cosmos_driver_submit(driver, op, options, queue, user_data, &pre_err)` → drain completion on `queue` |
| `cosmos_container_create_item(pk, json_data)` | `cosmos_operation_create_item(container, item_id, pk, &op)` → `cosmos_operation_with_body(op, bytes_view)` → `cosmos_driver_submit(...)` → drain completion on `queue` |
| Returned `out_json` (NUL-terminated `const char*`) | Returns `cosmos_response_t*` via `cosmos_completion_take_response`; body via `cosmos_response_into_body(response, &cosmos_bytes_t_handle)` (caller frees with `cosmos_bytes_free`) |
| HTTP errors mapped to `cosmos_error_code_t` | Surfaced as a completion with `outcome = ERROR` carrying a rich `cosmos_error_t` retrievable via `cosmos_completion_take_error(c)` / `cosmos_completion_error(c)` (see §3.5.2, §3.6, §6) |
| Synchronous return code per call | Asynchronous submit returns `cosmos_operation_handle_t*`; completion delivered on caller-owned `cosmos_cq_t` (see §3.1, §3.6) |
| One `ContainerClient` per container | Cheap `cosmos_container_ref_t` value handles (`_clone` is a refcount bump) |
| Tokio runtime hidden inside `CosmosClient` | Tokio runtime explicit on `cosmos_runtime_t` (one per process; see §4.1) |
| No diagnostics access | Full `cosmos_diagnostics_*` surface (`cosmos_response_diagnostics`, `cosmos_error_diagnostics`, `cosmos_diagnostics_to_json` → `cosmos_bytes_t**`) |

The new model is a **lower-level, more explicit, more powerful** API. Convenience and ergonomics belong in each host-language SDK that consumes these bindings.

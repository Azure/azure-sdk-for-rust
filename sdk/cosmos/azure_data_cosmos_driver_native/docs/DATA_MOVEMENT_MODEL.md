# Phase 0 — Data-movement model for `azure_data_cosmos_driver_native`

> **Status:** **Direction agreed** from @analogrelay's review (commit
> `d714527`, 2026-06-30). He answered the open questions inline, so this is no
> longer "proposed for a meeting" — see **§10** for the resolutions and the
> phased implementation plan. §1–§9 are retained for context on how we got here;
> where they conflict with §10, **§10 wins**.
> **Driver of this note:** @analogrelay review feedback on PR #4515.

## 1. Purpose

Establish **one** agreed rule for how data crosses the C ABI boundary in the
native wrapper, so the surface is consistent and reviewable, and so the open
`CHANGES_REQUESTED` can be resolved. This note is the thing to take into the
sync-up meeting.

## 2. The rule

> **Flat `#[repr(C)]` structs at every operation boundary; opaque handles only
> for state that genuinely cannot round-trip as bytes.**

Concretely:

1. **Inputs** to an operation are translated from a flat `#[repr(C)]` struct
   into the driver's Rust types **once**, at the boundary of that operation —
   not via a series of per-field setter FFI calls.
2. **Outputs** (scalars + borrowed strings) of an operation are read out of a
   flat `#[repr(C)]` **snapshot struct** in a single `*_view` call — not via a
   series of per-field getter FFI calls.
3. **Opaque handles** (`*mut T` to a real Rust struct, emitted opaque by
   cbindgen) are used **only** for live state that owns resources or shares an
   `Arc` and therefore cannot be flattened: runtimes, drivers, refs
   (account / database / container), the partition-key / feed-range handles,
   the completion queue, and the detachable response/error payloads.
4. **Ownership transfer** stays on dedicated `_take_*` accessors (they move an
   `Arc`/`Box` out and must not be folded into a read-only snapshot).
5. **No `as` casts on opaque pointers.** Return raw pointers to real Rust
   structs and reconstruct with `Box::from_raw` / `Arc::from_raw`. (cbindgen
   already emits these as opaque; the host never sees the fields.)

## 3. Why (mapping to the feedback)

- **"Too chatty."** Per-field setter/getter functions multiply the ABI every
  host SDK must bind and keep in sync. One flat struct + one call per direction
  collapses that. The operation surface already did this
  (`op_request.rs`: ~60 fns → 2 structs + a submit call); this note generalizes
  the same move to the remaining types.
- **"Opaque pointers are messy / `as` casts."** Real-Rust-struct handles
  (already the norm here) give type safety on the round-trip and let cbindgen
  do the opaque emission for free.
- **"How we're moving data around."** A single, written rule removes the
  case-by-case inconsistency that prompted the concern.

## 4. Current state vs. the rule

| Type / module | Direction | Today | Complies? |
|---|---|---|---|
| `op_request.rs` (operation + options) | in | flat `#[repr(C)]` structs + submit | ✅ yes |
| `completion.rs` (`CosmosCompletionView`) | out | flat snapshot `*_view` | ✅ yes |
| `response.rs` (`CosmosResponseView`) | out | flat snapshot `*_view` | ✅ yes |
| `error.rs` (`CosmosErrorView`) | out | flat snapshot `*_view` (Phase 1) | ✅ yes |
| `partition_key.rs` | in | flat `CosmosPartitionKeyComponent[]` consumed on the request (incremental builder retained) | ✅ yes (builder redundant) |
| `runtime_builder.rs` (`CosmosRuntimeOptions`) | in | flat struct + `cosmos_runtime_build` (Phase 2; builder retained) | ✅ yes |
| `driver_options.rs` (`CosmosDriverOptionsConfig`) | in | flat struct + `cosmos_driver_options_build` (Phase 2; builder retained) | ✅ yes |
| refs / queue / runtime / driver handles | handle | opaque real-Rust-struct `Arc`/`Box` | ✅ yes |

**Reference handles, the queue, and runtime/driver stay handles** — they own
`Arc`-shared live state and are explicitly **out of scope** for flattening.

> **Migration style (additive).** Each flat path is added **alongside** the
> existing builder/getter surface, exactly as `cosmos_response_view` was added
> beside the response getters. The old per-field surface is retained for
> back-compat; **removing** it is a sign-off decision (Open Question 2).

## 5. Scope of the immediate implementation (Phase 1)

Close the **`error.rs` gap** first, because it is the lowest-risk, highest-
fidelity application of the already-merged template:

- Add a `CosmosErrorView` flat `#[repr(C)]` snapshot mirroring
  `CosmosResponseView` exactly: `status_code`, `sub_status`, `is_from_wire`,
  `message`, `activity_id`, `session_token`, `etag`, `retry_after_ms`,
  `backtrace`. All string pointers borrowed, valid until `cosmos_error_free`.
- Add `cosmos_error_view(e, *mut CosmosErrorView) -> cosmos_error_code_t`,
  the single-call alternative to the 9 field accessors. It reuses the existing
  accessors (which lazily cache their `CString`s) so pointer lifetimes are
  unchanged.
- Keep the 9 individual accessors for now (backwards-compatible; the view is
  additive, exactly as `cosmos_response_view` was added alongside its getters).

### Deliberately **not** in Phase 1 (raise at the sync-up)

- **Status predicates** (`cosmos_error_is_*`, 16 fns). These are boolean
  *classifications*, not field reads, so they don't map cleanly onto the
  snapshot precedent. Folding them into a `flags` bitmask on the view is a
  reasonable follow-up but introduces a new bit-assignment ABI — decide
  together before committing to it.
- **`partition_key.rs` / `runtime_builder.rs` / `driver_options.rs`**
  flattening (the "in" builders). Bigger surface changes; sequence them as
  Phase 2 once the rule is signed off.

## 5a. Phase 2 status (implemented, additive)

The "in" builders now expose a flat single-call constructor alongside the
retained incremental builder:

- **`runtime_builder.rs`** — `CosmosRuntimeOptions` (`runtime_options_t`) +
  `cosmos_runtime_build(options, out_runtime, out_error)`. Sentinel-encoded
  fields (`workload_id` `0` = unset; the three string fields NULL = unset;
  `cpu_refresh_interval_ms` `0` = unset). A NULL `options` pointer means "all
  driver defaults". Same per-field validation as the setters; a shared
  `finish_runtime_build` helper backs both build paths.
- **`driver_options.rs`** — `CosmosDriverOptionsConfig`
  (`driver_options_config_t`) + `cosmos_driver_options_build(account, config,
  out_options)`. The account stays a separate handle parameter (it owns
  `Arc`-shared state and cannot be flattened); preferred regions + a pointer to
  the flat `cosmos_operation_options_t` carry the rest. A shared
  `decode_preferred_regions` helper backs both the setter and the flat path.
- **`partition_key.rs`** — already compliant: the request consumes a flat
  `CosmosPartitionKeyComponent[]` directly; no new work was needed.

The incremental builders remain for back-compat; removing them is Open
Question 2.

## 6. Acceptance for Phase 1

- `error.rs` exposes one `CosmosErrorView` + `cosmos_error_view`, mirroring the
  `CosmosResponseView` shape and NULL-handling (`INVALID_ARGUMENT` when `e` or
  `out_view` is NULL, `*out_view` left untouched).
- Borrowed-pointer lifetimes identical to the individual accessors.
- cbindgen rename registered (`CosmosErrorView` → `error_view_t`); header
  regenerates cleanly.
- Unit tests cover NULL handling and a populated snapshot (synthetic + wire
  errors), mirroring the existing `error.rs` tests.
- `fmt` / `clippy` / `doc -D warnings` / `cspell` clean; existing tests pass.

## 7. Open questions for the sync-up — **now resolved (see §10)**

0. **Invert the completion structure? (§8)** ✅ **Yes** — adopt it (§10 Phase 4).
0b. **Handle ownership: `Arc` → `Box`? (§9)** ✅ **Yes for refs**; the driver
   *may* stay `Arc`. Remove every `*_clone` FFI (§10 Phase 2).
1. ~~**Predicates → bitmask?**~~ ✅ **Neither.** No predicates, no flags bitmask;
   SDKs switch on status/sub-status, and a future **codegen** step generates the
   mapping tables from an authoritative Rust dataset (§10 Phase 3).
2. ~~**Retire the individual getters / builders?**~~ ✅ **Yes, remove them.**
   *"There is no back-compat to be preserved here."* The additive framing in
   §4/§5/§5a is superseded (§10 Phase 5).
3. ~~**"In" builder flattening order.**~~ ✅ Resolved in Phase 2 (already shipped).
4. **Diagnostics.** ✅ Direction: a **handle**, fetched later via an API; not
   inlined into the completion now (§10 Phase 4, deferred sub-item).

## 8. Open design decision — invert the completion structure

> **Status:** Proposed by @analogrelay (PR #4515 review). **Not implemented.**
> This is a redesign of the completion/response boundary, not an additive nit,
> and it **conflicts with** the recently-merged `cosmos_completion_view` /
> `CosmosResponseView` / `CosmosErrorView` snapshot pattern. Resolve at the
> sync-up before building.

### The feedback

When a caller gets a `cosmos_completion_t`, the first thing they do is read its
data — so the current "opaque handle + a `*_view` call" shape costs an extra FFI
round-trip per completion. Invert it:

- `cosmos_cq_wait` returns a `cosmos_completion_t *` that is a **`Box`-backed
  `#[repr(C)]` struct** (via `Box::into_raw`).
- The struct holds the data the client wants — **headers, body, user-data,
  status/outcome** — as **fields directly on it**. Diagnostics may be just a
  handle (fetched later via an API).
- The struct is **owned by the driver**, so the wrapper immediately **copies**
  the relevant data (headers, body, …) into SDK-owned memory, then calls
  `cosmos_cq_free_completion(completion)`.
- `cosmos_cq_free_completion` reconstitutes the `Box` and frees it **and any
  associated memory** (the `Vec` holding the body, the header structures, etc.).

### How it differs from today

| Aspect | Requested | Current |
|---|---|---|
| `cosmos_completion_t` layout | `Box`-backed **`#[repr(C)]`**, data inline | **opaque** Rust struct (no `#[repr(C)]`) |
| `cosmos_cq_wait` returns | the data, inline | opaque `*mut Completion` |
| Headers / body / user-data | **fields on the struct** | `cosmos_completion_view` (scalars only) **+** `cosmos_completion_take_response` → a separate `cosmos_response_t` handle → `cosmos_response_view` / `_body` |
| Diagnostics | a handle, fetched later | not surfaced inline |
| Free | one `cosmos_cq_free_completion` (Box + inner `Vec`/headers) | `cosmos_completion_free` **+** `cosmos_response_free` |

**FFI round-trips for a successful body completion today:** `cq_wait` →
`completion_view` → `take_response` → `response_view`/`_body` → `response_free` →
`completion_free` (six). **Proposed:** `cq_wait` (data inline) → copy →
`cq_free_completion` (effectively one read + one free).

### Implications / open points to settle

1. **Supersedes `*_view`.** The `cosmos_completion_view` snapshot and the
   separate `cosmos_response_t` body handle would be replaced for this path, not
   extended. `CosmosErrorView` / `CosmosResponseView` may stay for the
   synchronous / error paths, or also fold in — to decide.
2. **`#[repr(C)]` + variable-length data.** Headers and body are variable-length;
   a `#[repr(C)]` struct carries them as `(ptr, len)` fields pointing at
   driver-owned `Vec`/array memory that `cosmos_cq_free_completion` releases. The
   host must copy before freeing (matches the "copy then free" contract above).
3. **Lifetime contract.** Inline pointers are valid only until
   `cosmos_cq_free_completion`; document this as a hard rule (no detached
   borrows surviving the free).
4. **Error payload.** Where does the rich error go on an `ERROR` completion —
   inline fields, or still a detachable `cosmos_error_t *`?
5. **Batch wait.** `cosmos_cq_wait_batch` would return an array of these
   `#[repr(C)]` completions; the free contract must cover the batch.

### Recommendation

Bring this to the sync-up as the **primary** data-movement decision. If adopted,
it changes the migration plan in §5/§5a (the completion/response path moves from
"add `*_view` snapshots" to "inline `#[repr(C)]` completion + single free"), so
it should be settled before any further `*_view` work on that path.

## 9. Open design decision — handle ownership: `Arc` vs `Box`

> **Status:** Proposed by @analogrelay (PR #4515 review). **Not implemented.**
> This changes the lifetime/ownership contract of nearly every handle, so resolve
> at the sync-up before building.

### The feedback

> "Having everything we give to the language be in an `Arc` adds overhead and
> complexity to the wrapper. I'd rather the wrapper give *ownership* over the
> Driver, and other components, to the SDK. The SDK gets a `cosmos_driver_t *`
> backed by a `Box`, not an `Arc`, and the SDK is responsible for sharing that
> between several Clients, etc. The SDK will know when it can release an object,
> and can call a `cosmos_driver_free` to release it."

In short: hand **single ownership** to the host via `Box`; let the SDK manage
any sharing on its side; drop the wrapper-side refcounting and the FFI `*_clone`
functions.

### What the code does today (the opposite)

Nearly every public handle is **`Arc`-backed**, and several expose a `*_clone`
FFI that bumps the strong count so the host can mint sibling handles:

| Handle | Backing | FFI `*_clone`? |
|---|---|---|
| `cosmos_driver_t` | **Arc** | no (shared internally: the submit pipeline `Arc::clone`s it onto each task) |
| `cosmos_account_ref_t` | **Arc** | `cosmos_account_ref_clone` |
| `cosmos_container_ref_t` | **Arc** | `cosmos_container_ref_clone` |
| `cosmos_database_ref_t` | **Arc** | `cosmos_database_ref_clone` |
| `cosmos_feed_range_t` | **Arc** | `cosmos_feed_range_clone` |
| `cosmos_partition_key_t` | **Arc** | `cosmos_partition_key_clone` |
| `cosmos_driver_options_t` / `cosmos_error_t` / runtime | **Arc** | no |
| `cosmos_completion_t` / `cosmos_response_t` | Box | n/a (already single-owner) |

`driver.rs` states the rationale verbatim: *"reference-counted via `Arc` so the
submit pipeline and a degenerate response's stashed side payload can share it
with only an atomic bump."* That shared-`Arc` machinery is exactly what the
feedback wants removed.

### Implications / open points to settle

1. **Submit pipeline holds the driver across `.await`.** Today the driver is
   `Arc::clone`d onto each spawned task so the operation outlives the caller's
   handle. With `Box` single-ownership, the in-flight task needs a different
   guarantee — e.g. the host must keep the `cosmos_driver_t` alive until all its
   operations complete (documented contract), or the pipeline borrows rather
   than owns. This is the crux of the change.
2. **Degenerate-response side payload.** `cosmos_response_take_driver` /
   `_take_container` currently mint a handle from a stashed `Arc`. Under `Box`
   ownership these become a straight ownership *transfer* (the response hands the
   `Box` out and no longer holds it).
3. **Remove the `*_clone` FFI.** `account_ref` / `container_ref` / `database_ref`
   / `feed_range` / `partition_key` `*_clone` functions go away; the SDK clones on
   its side (it owns the data) or the wrapper deep-copies on demand.
4. **Refs that wrap driver-`Arc` state.** Some refs (`AccountReference`, etc.) are
   `Arc`-cheap to clone on the *driver* side already; `Box`-owning the FFI handle
   is independent of that and is fine — the handle is a thin owned wrapper.
5. **Thread-safety expectations.** `Arc` made handles trivially shareable across
   threads; with `Box` single-ownership the host is responsible for not using a
   handle from two threads at once (or the wrapper documents `Send`/`!Sync`-style
   rules).

### Relationship to §8

§8 (inline `#[repr(C)]` completion) and §9 (`Box` ownership) are **independent
but aligned** — both reduce wrapper-side machinery and push lifetime control to
the SDK. They can be decided separately, but they tell the same story and should
be discussed together.

### Recommendation

Bring to the sync-up alongside §8. Flipping `Arc` → `Box` is a breaking
ownership-contract change (removes `*_clone`, changes who guarantees liveness
across the submit `.await`), so it must be agreed before implementation — and it
partly revises the "handles stay handles" row of the §4 compliance table (the
handles stay handles, but their *backing* and *clone* contract change).

## 10. Review resolutions & phased implementation plan

@analogrelay reviewed commit `d714527` and answered the open questions inline.
This section is the **authoritative agreed direction**; where §1–§9 differ,
this wins.

### 10.1 Resolutions (verbatim intent)

1. **Owned, not "views".** Values handed to the SDK are **owned `#[repr(C)]`
   structs with all data owned in Rust** (`Box::into_raw`). Drop the `_view`
   concept and suffix — *"a 'View' to me is a non-owning type… so why is it a
   view?"* The SDK becomes the **sole owner**, reads/copies fields, then calls a
   single `*_free` that reclaims the `Box` and drops all inner data
   (`String`s, `Vec<u8>` body, headers, diagnostics).
2. **Invert the completion.** `cosmos_cq_wait` returns a
   `Box<CosmosCompletion>` as a `#[repr(C)]` struct holding **headers, body,
   user-data, status/outcome** inline; diagnostics may be a **handle** fetched
   later. `cosmos_cq_free_completion` reclaims it and frees all associated
   memory. No separate `cosmos_response_t` + `take_response` + `_view`.
3. **One wait op, always an array.** *"We should just have one wait operation:
   `cosmos_cq_wait(...)`. It should ALWAYS return an array of completions."* The
   wrapper may yield one at a time internally; a future option can tune the
   batch size.
4. **Headers as a flat list on the completion.** A `cosmos_header_kv_t`
   (`<name, value>`) array — the same shape already used for request
   `custom_headers` — carried directly on the completion and handed to the SDK.
5. **No status predicates, no flags bitmask.** *"SDKs can use the
   status/substatus code… let's use codegen to solve it."* The Rust crate owns
   an authoritative status/sub-status dataset; mapping code for each SDK is
   **generated**, not exposed as FFI predicate functions.
6. **`cosmos_error_t`, plainly.** The FFI error is just `CosmosError` exposing
   `cosmos_error_t` — owned, fields read directly. No `CosmosErrorView`.
7. **Ownership = `Box`, not `Arc`, for refs.** Account / database / container
   refs, feed ranges, and partition keys are handed over as **owned `Box`**; the
   SDK owns and frees them. The **driver may stay `Arc`** (a fair argument given
   the submit pipeline holds it across `.await`).
8. **Remove the `*_clone` FFI entirely.** *"An SDK shouldn't be cloning its copy
   of the `Arc`… it just passes in `cosmos_container_ref_t *` and then the Rust
   side clones it before returning."* Cloning, when needed, happens on the Rust
   side from a borrowed pointer — never exposed to the SDK.
9. **No back-compat.** *"There is no back-compat to be preserved here."* Remove
   the superseded incremental builders and per-field getters rather than keeping
   them additively.
10. **Export hygiene.** `cosmos_` prefix on **all** exported functions/types
    (dynamic-library symbol collisions break the linker); **consider `cosmos_v1_`
    for functions** to allow API iteration. Remove the agent-session
    explanatory blobs from doc comments (they propagate into the C header).

### 10.2 Phases

Sequenced foundational → high-risk, each independently buildable + validated
(`fmt` / `clippy` / `cargo test` / header regeneration / `cspell`). The
driver-stays-`Arc` concession (10.1 #7) keeps Phase 2 tractable.

| Phase | Scope | Risk | Touches |
|---|---|---|---|
| **P1 — Doc hygiene** | Strip the "real Rust struct, not `#[repr(C)]`… cbindgen emits opaque…" agent blobs from every handle doc comment; keep a one-line purpose. No ABI change. | low | all `src/*.rs` handle docs |
| **P2 — Refs → `Box`, drop `*_clone`** | Convert `account_ref` / `database_ref` / `container_ref` / `feed_range` / `partition_key` from `Arc` to `Box` single-ownership; delete the 5 `*_clone` fns; clone driver-side from a borrowed `*const` where a build/submit path needs to retain. Driver stays `Arc`. | med | the 5 ref modules, `op_request.rs`, `driver_options.rs`, header, C tests |
| **P3 — Error simplification** | Remove the 16 `cosmos_error_is_*` predicates and the `CosmosErrorView` snapshot + `cosmos_error_view`; `cosmos_error_t` keeps status/sub-status/message/etc. (mappings move to codegen, out of this crate's API). | med | `error.rs`, `build.rs`, header, C tests |
| **P4 — Completion inversion** (the big one) | `cosmos_completion_t` → `Box`-backed `#[repr(C)]` with inline `outcome`, `status`, `sub_status`, `user_data`, headers (`cosmos_header_kv_t[]`), body (`ptr,len`), error (inline/owned), diagnostics handle (deferred). `cosmos_cq_wait` returns an **array**; `cosmos_cq_free_completion` frees box + inner allocations. Delete `cosmos_completion_view`, the whole `response.rs` (`cosmos_response_t` + accessors + `_view` + `take_response`). | **high** | `completion.rs`, `response.rs` (removed), `submit.rs`, `op_request.rs`, header, C tests, docs |
| **P5 — Remove redundant builders** | Delete `cosmos_runtime_builder_*`, `cosmos_driver_options_builder_*`, `cosmos_partition_key_builder_*` now that flat constructors / inline arrays exist (no back-compat). | med | `runtime_builder.rs`, `driver_options.rs`, `partition_key.rs`, header, C tests |
| **P6 — Export prefix/versioning** | Audit every export for the `cosmos_` prefix; **decide `cosmos_v1_` for functions** (needs explicit sign-off before the rename churn). Regenerate header. | low–med (but **needs sign-off**) | every `#[no_mangle]`, `build.rs`, header, C tests |

**Notes**

- **P1 first** — mechanical, zero ABI risk, clears the doc noise before the
  structural phases.
- **P4 is the keystone** and the largest; P2/P3 de-risk it by settling
  ownership and the error shape first.
- **P6 (`cosmos_v1_`)** is the one item still needing an explicit decision; it is
  sequenced last and flagged so the rename isn't done speculatively.
- The **driver `Arc`-vs-`Box`** final call (10.1 #7 leaves the driver on `Arc`)
  can be revisited, but the agreed default is: driver `Arc`, everything else
  `Box`.

### 10.3 Phase 4 concrete design — inverted `cosmos_completion_t`

The keystone change. `cosmos_cq_wait` stops returning an opaque completion the
host then unpacks through `cosmos_completion_view` + `cosmos_completion_take_response`
+ a separate `cosmos_response_t`; instead it returns an **array of owned
`#[repr(C)]` completions** whose data is already inline.

#### The struct

```rust
#[repr(C)]
pub struct CosmosCompletion {
    // ── Always valid ─────────────────────────────────────────────
    outcome: CosmosCompletionOutcome,     // i32: Ok / Error / Cancelled / Unknown
    status: CosmosErrorCode,              // i32: coarse code (always populated)
    user_data: isize,                     // opaque pointer-sized cookie, verbatim
    was_cancel_requested: u8,             // 0/1 (u8, not bool, to avoid UB)

    // ── Response scalars (0 / default on non-OK / degenerate) ────
    http_status_code: u16,                // wire HTTP status (0 when none)
    request_charge: f64,                  // RU (0.0 when absent)

    // ── Borrowed strings — valid until cosmos_cq_free_completions ─
    activity_id: *const c_char,           // NULL when absent
    session_token: *const c_char,
    etag: *const c_char,
    continuation: *const c_char,          // server header continuation
    next_continuation: *const c_char,     // planner-derived next-page token

    // ── Borrowed header list (net-new) ───────────────────────────
    headers: *const CosmosHeaderKv,       // NULL/0 when none
    headers_len: usize,

    // ── Borrowed body — NULL/0 when empty ────────────────────────
    body: *const u8,
    body_len: usize,

    // ── Owned payloads the SDK takes / the free reclaims ─────────
    error: *mut CosmosErrorHandle,        // owned; non-NULL only on Error (+ details on)
    diagnostics: *mut c_void,             // deferred — always NULL for now
    driver: *mut DriverHandle,            // owned; degenerate get_or_create completion
    container: *mut ContainerRefHandle,   // owned; degenerate resolve_container completion

    // ── Opaque owner (cbindgen: opaque ptr the C side never touches) ─
    backing: *mut CosmosCompletionBacking,
}
```

`CosmosCompletionBacking` is **not** `#[repr(C)]`; cbindgen emits it as an
opaque forward-declared pointer. It owns everything the borrowed pointers
reference so they stay valid until free:

```rust
struct CosmosCompletionBacking {
    response: Option<CosmosResponse>,      // owns the body bytes
    header_values: Vec<CString>,           // backs each CosmosResponseHeader.value
    headers: Vec<CosmosResponseHeader>,    // the array `headers` points at
    message: Option<CString>,              // backs the error-string ptrs
    activity_id: Option<CString>,
    session_token: Option<CString>,
    etag: Option<CString>,
    continuation: Option<CString>,
    next_continuation: Option<CString>,
    backtrace: Option<CString>,
    op_inner: Arc<OperationInner>,         // for op-handle state after the fact
}
```

#### The free contract

```rust
// Frees an array of `count` completions and every inner allocation
// (backing box, owned driver/container handles NOT detached by the SDK).
pub extern "C" fn cosmos_completion_queue_free_completions(
    completions: *mut CosmosCompletion,
    count: usize,
);
```

- The host reads the fields, **copies** what it needs into SDK memory, then
  calls the free. All borrowed pointers die at free.
- Owned payloads (`driver` / `container`): the SDK either (a) reads them in
  place and lets the free drop them, or (b) detaches ownership by zeroing the
  field first (a `cosmos_completion_take_*` helper sets the field to NULL and
  returns the pointer, so the free skips it). Default: the free reclaims any
  non-NULL owned handle.

#### The wait

```rust
// The ONLY wait. Blocks until ≥1 completion or timeout, then drains up to
// `max` already-queued completions without blocking again. Writes into the
// caller's `out[0..max]`, returns the count. Each element is freed by
// cosmos_completion_queue_free_completions.
pub extern "C" fn cosmos_completion_queue_wait(
    queue: *mut CompletionQueue,
    out: *mut CosmosCompletion,
    max: usize,
    timeout_ms: u32,
) -> usize;
```

Collapses today's `cosmos_cq_wait` (single) + `cosmos_cq_wait_batch` (array)
into one array-returning call.

#### Removed by this phase

- `cosmos_completion_view` + `CosmosCompletionView`.
- `cosmos_completion_take_response` / `cosmos_completion_response`.
- `cosmos_completion_take_error` / `cosmos_completion_error` (error is now
  inline scalar fields).
- The entire `response.rs`: `cosmos_response_t` + all `cosmos_response_*`
  accessors + `CosmosResponseView` + `cosmos_response_view` +
  `cosmos_response_take_driver` / `_take_container` + `cosmos_response_free`.
  The degenerate side-payloads (`driver` / `container`) move onto the
  completion as owned fields.
- `cosmos_cq_wait` (single) + `cosmos_cq_wait_batch` → the one array wait; the
  remaining `cosmos_cq_*` functions are renamed to `cosmos_completion_queue_*`.

#### Header list — header-id model (confirmed)

The driver keeps **typed** response headers (`CosmosResponseHeaders`), not a raw
key/value map, and its header-name constants are `pub(crate)`. Rather than a
string-named list, the completion carries a **header-id** list (RNTBD-token /
codegen style):

```rust
#[repr(i32)]
pub enum CosmosHeaderId {
    CosmosHeaderIdUnknown = 0,
    CosmosHeaderIdActivityId = 1,
    CosmosHeaderIdRequestCharge = 2,
    // … one stable id per known response header
}

#[repr(C)]
pub struct CosmosResponseHeader {
    pub id: CosmosHeaderId,               // stable numeric id
    pub value: *const c_char,             // borrowed value string
}

// Exposes the id → canonical wire-name mapping to the SDK.
pub extern "C" fn cosmos_header_name(id: CosmosHeaderId) -> *const c_char;
```

The wrapper **synthesizes** the list from the typed `CosmosResponseHeaders`,
assigning each populated field its stable id and rendering its value as a
string; the
`cosmos_header_name` accessor (hardcoded canonical `x-ms-*` names) gives the SDK
the id→name mapping. This is the codegen-friendly shape @analogrelay described
(SDKs map ids, not string-compare names). Exhaustive coverage / a driver-side
raw header map is a follow-up; the first cut covers the commonly-needed headers.

**Sub-phasing.** *P4a* builds this header-id model in isolation (new module,
`CosmosHeaderId` + `CosmosResponseHeader` + `cosmos_header_name` + a synthesis
fn from `CosmosResponseHeaders`), unit-tested and additive — no completion
changes, tree stays green. *P4b* does the completion inversion consuming it.

## 11. Before / after — the three data-movement points (implemented)

> **Status:** All three shipped. Point 1 = P4a (`b54f61f05`), point 2 = P4b
> (`64c7f3b0e`), point 3 = P2 (`13f43cb31`). This appendix records the concrete
> before/after so the shape of the change is legible without diffing.

### 11.0 At a glance

| # | Review point | Previous (opaque-handle model) | Current (owned `#[repr(C)]` model) | Phase |
|---|---|---|---|---|
| 1 | Header-id list on the completion | No response-header list existed; only 4 typed header strings via per-field accessor calls on a separate `cosmos_response_t` | `cosmos_response_header_t { id, value }` array carried **inline** on the completion; `id`→name via `cosmos_header_name()` | P4a |
| 2 | Raw response in the completion | `cq_wait` → opaque completion → *view* → *take_response* → separate `cosmos_response_t` → *response_view/body* → 2 frees (**6 FFI calls**) | `cosmos_completion_queue_wait` returns an array of owned completions with body + headers **inline**; **1 read + 1 bulk free** | P4b |
| 3 | Call-in / copy, not `Arc` | Partition key was an `Arc`-backed `cosmos_partition_key_t` handle, cloned/shared; all refs `Arc` | Partition key passed as a raw `cosmos_partition_key_component_t[]` on the request, **copied** in place; refs are `Box` single-owner | P2 |

### 11.1 Point 1 — header struct `<id: value>` (RNTBD / codegen style)

| Aspect | Previous | Current |
|---|---|---|
| Header list | **none** — no generic header list existed | `cosmos_response_header_t[]` inline on the completion (`headers`, `headers_len`) |
| Representation | 4 hardcoded typed strings (activity id, session token, etag, continuation) via separate accessor calls | `struct { cosmos_header_id_t id; const char* value; }` — an `(id, value)` pair |
| Name model | string-named accessors baked into the ABI | numeric **`cosmos_header_id_t`** enum (append-only, `0 = UNKNOWN` sentinel, 19 known ids) |
| Id → name mapping | n/a | `cosmos_header_name(id)` returns the canonical `x-ms-*` wire name (statically allocated) |
| SDK usage | string-compare header names | **switch on numeric ids** (codegen-friendly, RNTBD-token style) |
| Source | — | synthesized from the driver's typed `CosmosResponseHeaders`; `CString` storage owned by the completion's backing box |
| Coverage | 4 headers | 19 headers; append-only, exhaustive/raw-map is a follow-up |

### 11.2 Point 2 — raw response in the completion (avoid query → response → destroy)

| Aspect | Previous | Current |
|---|---|---|
| Wait ops | `cosmos_cq_wait` (single) + `cosmos_cq_wait_batch` (array) | **one** `cosmos_completion_queue_wait(queue, out[], max, timeout)` → array |
| Completion type | **opaque** handle (`cosmos_completion_t*`, no `#[repr(C)]`) | owned **`#[repr(C)]` value** the SDK reads directly |
| Reading scalars | `cosmos_completion_view()` snapshot call | inline fields (`outcome`, `status`, `user_data`, `http_status_code`, `sub_status`, `request_charge`, `retry_after_ms`, …) |
| Getting the response | `cosmos_completion_take_response()` → separate `cosmos_response_t*` handle | **body inline** on the completion (`body`, `body_len`) |
| Reading the body | `cosmos_response_view()` / `cosmos_response_body()` on that handle | direct `body` / `body_len` fields |
| Error detail | detachable `cosmos_error_t*` via `take_error` | **inline scalar fields** (`message`, `backtrace`, `is_from_wire`, …) — no handle |
| Degenerate payloads (driver / container) | stashed on the response, `cosmos_response_take_driver/_container` | owned `driver` / `container` fields on the completion, optional `take_*` |
| Freeing | `cosmos_response_free` **+** `cosmos_completion_free` (two) | **one** `cosmos_completion_queue_free_completions(out[], count)` |
| **FFI round-trips (happy path)** | **6**: `cq_wait` → `completion_view` → `take_response` → `response_view/body` → `response_free` → `completion_free` | **2**: `queue_wait` (data inline) → `free_completions` |
| `response.rs` | entire module (`cosmos_response_t` + accessors + view + takes + free) | **deleted** — folded onto the completion |

### 11.3 Point 3 — `Arc` vs `Box`; call-in (copy) for the partition key

| Aspect | Previous | Current |
|---|---|---|
| Partition key on an operation | `Arc`-backed `cosmos_partition_key_t*` handle, built + cloned + shared into the request | raw **`cosmos_partition_key_component_t[]`** (`partition_key_components` + `partition_key_len`) passed **inline** on `cosmos_operation_request_t` |
| Ownership at the boundary | `Arc::clone` / atomic refcount bump | **caller owns** the array (stack / SDK memory); the wrapper **copies** what it needs (`partition_key_from_components`) — the **call-in** pattern |
| Handle path | primary | retained only as a **fallback**; the inline components path takes precedence |
| Ref handles (account / db / container / feed-range / partition-key) | all **`Arc`**-backed with `*_clone` FFI | all **`Box`** single-owner; **5 `*_clone` FFI removed** |
| Who clones | SDK bumps the `Arc` via `*_clone` | Rust side clones from a borrowed `*const` when a build/submit path must retain; never exposed to the SDK |
| Driver handle | `Arc` | **stays `Arc`** (submit pipeline holds it across `.await`) — the sole intentional exception |

**Net effect:** the SDK now does **one wait → read inline fields / body /
header-ids → one free**, owns everything it is handed (`Box`, not `Arc`), and
passes partition keys by **copy-in** rather than shared refcounted handles.

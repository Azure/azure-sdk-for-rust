# Phase 0 — Data-movement model for `azure_data_cosmos_driver_native`

> **Status:** Proposed — for alignment before further FFI work
> **Driver of this note:** @analogrelay review feedback on PR #4515
> (2026-06-22 "too chatty / opaque pointers + `as` casts" and 2026-06-29
> `CHANGES_REQUESTED` "still have some concerns about how we're moving data
> around … maybe we should have a sync-up meeting").

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

## 7. Open questions for the sync-up

0. **Invert the completion structure? (biggest call — see §8.)** Make
   `cosmos_completion_t` a `Box`-backed **`#[repr(C)]`** struct that carries the
   data (headers, body, user-data, diagnostics handle) inline, copied once into
   SDK memory, then freed by a single `cosmos_cq_free_completion` — instead of
   the current opaque completion + `cosmos_completion_view` + a separate
   `cosmos_response_t` handle. This **supersedes** the `*_view` snapshot pattern
   for the completion/response path, so resolve it before investing further in
   that pattern.
0b. **Handle ownership: `Arc` → `Box`? (see §9.)** Replace the `Arc`-everywhere
   handle model (and the `*_clone` FFI) with `Box` single-ownership handed to the
   SDK, which manages any sharing and calls `cosmos_*_free` when done. Breaking
   ownership-contract change; interacts with the submit pipeline's hold across
   `.await`.
1. **Predicates → bitmask?** Fold the 16 `is_*` predicates into a `flags` field
   on `CosmosErrorView`, or keep them as separate calls?
2. **Retire the individual getters?** Once `*_view` exists for response + error,
   do we deprecate/remove the per-field accessors, or keep both indefinitely for
   host ergonomics?
3. **"In" builder flattening order.** ~~partition-key vs. runtime/driver-options
   first for Phase 2?~~ **Resolved** — both `cosmos_runtime_build` and
   `cosmos_driver_options_build` landed in Phase 2; partition-key was already
   flat.
4. **Diagnostics.** The richer `DiagnosticsContext` surface is still a follow-up
   — flat snapshot, opaque handle, or deferred entirely? (Tied to §8: the
   inverted completion would carry diagnostics as a handle fetched later.)

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

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
| `error.rs` | out | **9 per-field getters, no snapshot** | ❌ **gap** |
| `partition_key.rs` | in | **component-by-component builder (13 fns)** | ❌ gap |
| `runtime_builder.rs` | in | **builder setters (8 fns)** | ❌ gap |
| `driver_options.rs` | in | **builder setters (6 fns)** | ❌ gap |
| refs / queue / runtime / driver handles | handle | opaque real-Rust-struct `Arc`/`Box` | ✅ yes |

**Reference handles, the queue, and runtime/driver stay handles** — they own
`Arc`-shared live state and are explicitly **out of scope** for flattening.

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

1. **Predicates → bitmask?** Fold the 16 `is_*` predicates into a `flags` field
   on `CosmosErrorView`, or keep them as separate calls?
2. **Retire the individual getters?** Once `*_view` exists for response + error,
   do we deprecate/remove the per-field accessors, or keep both indefinitely for
   host ergonomics?
3. **"In" builder flattening order.** partition-key vs. runtime/driver-options
   first for Phase 2?
4. **Diagnostics.** The richer `DiagnosticsContext` surface is still a follow-up
   — flat snapshot, opaque handle, or deferred entirely?

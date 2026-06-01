# INVARIANTS — `azure_data_cosmos_native_async_poc`

The FFI surface in `include/cosmos_async_poc.h` holds the following invariants.
These are the **core contract** the feasibility spike is testing. Every Rust
implementation file and every host-side P/Invoke binding MUST uphold them.

If a spike experiment is found to violate one of these, that finding **is**
the verdict — write it up in `poc-async-ffi-verdict.md` instead of relaxing
the invariant.

---

## I1 — `user_data` is byte-opaque

The `void* user_data` pointer passed to any submission API is **never**
dereferenced, inspected, or modified by Rust. It is moved by value into the
completion record and surfaced through `cosmos_cq_wait`'s `out_user_data`
slot, byte-identical to what the host passed in.

**Rationale.** Hosts encode arbitrary per-op state into this slot
(`GCHandle.ToIntPtr` in .NET, channel sender ptr in Go, etc.). Any
inspection by Rust would couple the ABI to a single host's encoding.

**Enforcement.** `user_data` is stored as `usize` in the Rust completion
record; there is no Rust code path that calls `*` on it.

---

## I2 — Exactly one completion per accepted submission

For every submission API call that returns a **non-null** `cosmos_op_t*`, the
implementation guarantees exactly one completion is eventually delivered on
the associated `cosmos_cq_t`, with status in
`{ COSMOS_OK, COSMOS_ERROR, COSMOS_CANCELLED }`.

Corollaries:

- **No double-completion.** A late completion from the network MUST NOT race
  a cancellation completion. The implementation arbitrates internally and
  posts exactly one of the two on the queue.
- **No silent drops.** Even if the op succeeded but the host had already
  asked to cancel, the host gets exactly one completion (the one that won
  the race in the impl). The host can identify "this is my op" from
  `out_user_data` and act accordingly.
- **No completion if submission was rejected.** If the submission API returns
  NULL, NO completion is queued. The host releases its per-op state on the
  spot.

**Rationale.** The dispatcher in the host (the per-op `TaskCompletionSource`,
`CompletableFuture`, etc.) is freed when the completion is observed. Zero
completions leak. Two completions either double-free or call `TrySetResult`
on an already-completed TCS (silently ignored, then a dangling per-op state
in the dispatcher map).

**Enforcement.** The Rust `op.rs` uses a single `AtomicBool` `done` plus a
`crossbeam_channel::Sender<Completion>`. The first writer wins; subsequent
attempts return early without posting.

---

## I3 — `catch_unwind` at every FFI boundary

Every `#[no_mangle] extern "C" fn` in the crate wraps its body in
`std::panic::catch_unwind` and translates a panic into either:

- `COSMOS_ERROR` written into the `cosmos_error_t*` out-parameter (for
  submission/setup APIs), with `message_utf8` borrowed from a thread-local
  panic message buffer, OR
- A completion with status `COSMOS_ERROR` posted to the CQ (for panics
  inside a tokio task spawned from a submission), OR
- A silent no-op for `*_free` / `*_release` functions (panics during free
  paths swallow rather than corrupting the FFI; this is the same trade-off
  the C++ destructor world makes).

**Rationale.** Rust unwinding across the FFI boundary is undefined behavior.
`-Cpanic=abort` would lose information; per-fn `catch_unwind` is the
idiomatic compromise.

**Enforcement.** Code review of `src/lib.rs`. Every `extern "C" fn` starts
with `let _ = std::panic::catch_unwind(|| { ... });` or uses a helper macro
`ffi_guard!`.

---

## I4 — Single waiter per CQ

At most ONE thread may be inside `cosmos_cq_wait` for a given CQ at any
time. The POC implementation does NOT defend against multi-waiter use; the
internal channel `Receiver` is held behind a `Mutex` but contention is
considered a host bug.

**Rationale.** Spec simplification for the spike. Production designs MAY
choose to allow N waiters; this is left explicitly as a "did not prove"
item for the verdict doc.

**Enforcement.** Documented in the header for `cosmos_cq_wait`. No runtime
check.

---

## I5 — Op handle disposal

For every op handle `cosmos_op_t*` returned by a successful submission,
the host MUST call EXACTLY ONE of `{ cosmos_cancel, cosmos_op_release }`.
Calling neither leaks the handle's ref count; calling both is undefined
behavior in the POC (in practice: a double-decrement that will panic in
`Arc::strong_count` checks if any are present in `op.rs`).

Cancellation **always** counts as a disposal (cosmos_cancel releases the
host ref atomically with requesting cancellation).

**Rationale.** The host needs the handle so it can request cancellation; the
impl needs the handle so the tokio task can keep its arbitration state
alive. Refcounted ownership is the cleanest formulation; the rule above
keeps the contract simple at the API surface.

**Enforcement.** `op.rs` wraps the inner state in `Arc<OpState>`; each FFI
constructor / `cosmos_cancel` / `cosmos_op_release` is paired with exactly
one Arc clone or drop.

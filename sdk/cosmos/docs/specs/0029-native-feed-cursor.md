# Generic native feed cursor

> **Status:** Pending release
> **Owners:** Cosmos DB Rust SDK team
> **Target:** `azure_data_cosmos_driver_native`

## 1. Overview and scope

The cursor retains the driver's `OperationPlan` across query, read-feed, and
change-feed pages. Page advancement is independent of checkpoint serialization,
so buffered queries can return all results even without a supported durable token.
The driver owns planning, routing, retries, and topology recovery; the wrapper
owns ABI validation, cursor lifetime, and result delivery.

Options are fixed for the cursor lifetime. Each Next advances one driver page,
which may require several backend requests. Existing query admission, memory
limits, and feed-range capabilities apply. Read feeds preserve existing behavior:
database/container and single-partition item reads use the trivial pipeline;
full-container item read-all returns `400/20112` (fan-out required).

New query operators, pipeline-node APIs, range enumeration, processor leases,
token formats, and native-managed prefetch/polling are outside scope. Wrapper SDKs
can prefetch within their own buffer limits. Metrics aggregation, binary encoding
optimization, and partial cancellation diagnostics remain separate concerns.

## 2. Relationship to existing designs

Paths below are relative to `sdk/cosmos/docs`.

| Document | Relationship |
| --- | --- |
| `specs/0019-native-wrapper.md`, section 4.7, Phase 8, and open questions 3/4/9/14 | Consolidates the pager proposal and resolves the conflicting no-wrapper-pager description. Explicit End, independent checkpoints, and complete buffer arrays replace the proposed 404-as-EOF and borrowed token accessor. |
| `specs/0020-native-async-invocation.md` | Reuses operation handles, completion queues, and correlation, with isolated cursor results and delivery rules. |
| `adrs/0005-flat-native-abi-data-model.md` | Preserves the distinction between flat data records and opaque live-state handles. |

Driver dataflow and query semantics remain defined by specifications 0012/0013,
encoding by 0014/0015, status/retries by 0006, and diagnostics by 0018. This design
supersedes only the overlapping pager proposal. The generated native header is
authoritative for exported names, layouts, and numeric values.

## 3. ABI and ownership

### 3.1 Records and queue isolation

The additive cursor records preserve existing ABI layouts, signatures,
discriminants, and ownership contracts. Their `struct_size_bytes`/`abi_version`
prefix is validated before accessing the rest of the caller-declared allocation.
The current record version is 1. Unknown versions, insufficient sizes, invalid
integer selectors, and unsupported nonzero extensions fail explicitly; reserved
fields are zero.

The open record embeds the frozen legacy request plus change-feed selectors.
Queries, read feeds, and change feed are accepted; writes, batches, and other
singleton operations are rejected before planning. Common input and status
contracts follow the native ABI; reserved diagnostics fields are not repurposed.

Queue format is immutable. Mismatched submissions or cursor waits fail without
consuming results. Legacy wait has no error output: on a cursor queue it logs a
format error and returns zero. Cursor wait transfers pointers to Rust-allocated
records through caller-provided pointer slots, avoiding a caller-compiled record
stride. Results use the matching cursor completion-free API.

### 3.2 Operations

| API | Behavior |
| --- | --- |
| `cosmos_cursor_queue_create` | Creates a format-isolated queue with a capacity limit. |
| `cosmos_cursor_open_submit` | Copies/validates inputs and asynchronously creates the plan; returns Opened without consuming a data page. |
| `cosmos_cursor_next_submit` | Advances the retained plan; returns Page or End. |
| `cosmos_cursor_checkpoint_submit` | Returns a token for delivered progress or a driver error. |
| `cosmos_cursor_queue_wait` | Transfers owned completion pointers; errors are distinct from timeout/empty. |
| `cosmos_cursor_completion_take_cursor` | Detaches the opened cursor once. |
| `cosmos_cursor_completion_free` | Releases result backing and any undetached cursor. |
| `cosmos_cursor_free` | Releases the caller's handle without waiting for I/O. |
| `cosmos_cursor_status` | Reports terminal cursor failure. |
| `cosmos_operation_handle_status` | Reports packed operation status, including delivery loss. |

Submissions return existing operation handles and preserve `user_data`.
Network work, including Open's planning/metadata requests, is asynchronous.
Each cursor is bound to its opening queue/runtime; moving queues requires a new
cursor and a supported checkpoint. Different cursors can execute concurrently.

### 3.3 Results and payload lifetime

| Field | Values |
| --- | --- |
| Outcome | Success, error, cancelled |
| Successful result | Opened, Page, Checkpoint, End |
| Body shape | NoPayload, RawBytes, Items |

RawBytes is one opaque payload, possibly a feed envelope. Items is an ordered
pointer/length array of **all** item buffers, including zero-length members.
Shape is explicit, never inferred from application JSON fields. HTTP status,
headers, and errors remain distinct from end-of-feed state.

The completion owns its response, payload views, headers, and token/error text.
**Page buffers survive further advancement and cursor release**, until their
completion is freed. Opened also owns its cursor until Take cursor detaches it.

Bytes retain the driver's negotiated encoding and complete change envelopes
(`current`, `previous`, metadata). Binary items are not produced by slicing an
envelope and prepending a marker: references can address the original page.

## 4. Feed and checkpoint semantics

### 4.1 Pages and exhaustion

A driver page produces Page even when empty. Only driver `None` produces End;
repeated Next after End returns End without I/O. Change-feed HTTP 304 is a
successful idle Page, not End, and the caller chooses when to poll again.
Absence of a checkpoint never implies exhaustion.

### 4.2 Change feed

`change_feed_mode` selects LatestVersion (1) or AllVersionsAndDeletes (2), with
the embedded legacy kind set to 0. Fresh reads require an explicit start;
PointInTime is counted UTF-8 RFC 3339 text with driver/service wire precision.

| Mode | Fresh start | Resume |
| --- | --- | --- |
| LatestVersion | Beginning, Now, PointInTime | Token position takes precedence. |
| AllVersionsAndDeletes | Now; well-formed Beginning/PointInTime requests retain service rejection behavior. | Token position takes precedence; mode must match. |

Malformed timestamps, invalid selectors, missing fresh starts, and time fields
on non-time selectors fail preflight. Account opt-in and retention restrictions
remain service errors. Resume allows Unset start; any supplied start is validated
but does not override the token. The driver validates operation/scope/mode
compatibility and owns unpolled-range positions and split/merge recovery.

### 4.3 Checkpoints and prefetch

Checkpoint calls `OperationPlan::to_continuation_token` without advancing the
plan, returning the current opaque composite token or the original driver error.
It is neither a cached older token nor a raw server ETag/continuation header.
Unsupported snapshots are non-terminal: DISTINCT and non-streaming ORDER BY can
continue paging in memory. Poisoned/uncertain progress is terminal. Snapshot
support depends on current plan state, including whether it has drained.

**Tokens describe delivery to the wrapper, not application consumption.**
Wrapper-managed prefetch can request the next page after transfer while retaining
earlier completions. The wrapper bounds buffering, captures supported checkpoints
at page boundaries before advancing, and persists only progress corresponding to
consumed pages. There is no exactly-once or application-acknowledgement guarantee.

## 5. Concurrency and failure

### 5.1 State transitions

Cursor state is reference-counted. The admitted task owns the plan while executing;
short synchronized transitions hold no blocking mutex across await points.

| State/event | Result |
| --- | --- |
| Ready + accepted Next/Checkpoint | Busy, with result capacity reserved before execution. |
| Busy + another Next/Checkpoint | Synchronous Busy rejection; no completion or progress change. |
| Page transferred | Ready; the previous completion can remain alive. |
| End transferred | Exhausted; subsequent Next returns End without I/O. |
| Successful/unsupported Checkpoint transferred | Previous Ready/Exhausted state restored. |
| Execution failure, panic, or cancellation with uncertain progress | Failed/Cancelled; partially advanced state is unusable. |
| Admission rejection | Previous state preserved; no asynchronous completion. |
| Host Free | Caller handle released; admitted work retains ownership. |

**Busy lasts until transfer**, not execution or publication, preventing a
checkpoint from passing an undelivered page. Host threads synchronize cursor use
after receipt. Driver retries remain unchanged; an uncertain failure escaping
Next requires recovery from a saved checkpoint or a restart, not an automatic
whole-plan retry.

### 5.2 Delivery, cancellation, and release

Capacity covers all admitted-but-undrained operations, including Open. Reservations
are released on transfer or preflight rollback, ensuring space for an admitted
result without blocking executor threads on a consumer.

Shutdown rejects new work but permits admitted work to drain. Queue destruction
abandons undelivered results and invalidates affected cursors. **Lost delivery
cannot be followed by successful advancement past the missing page.** Terminal
cursor/operation status remains observable without the queue.

Cancellation is ordered atomically against publication. Cancellation that wins
can terminate uncertain progress; a published result is preserved, with late
cancellation reflected at transfer. Future-drop cancellation has no partial
diagnostics.

Free is non-blocking and does not cancel admitted work, which retains strong
references through completion. Callers synchronize raw-handle release against
new calls using that handle. The panic firewall preserves one result per admitted
operation unless the queue is explicitly abandoned.

## 6. Legacy compatibility

Legacy layouts and genuine singleton behavior are unchanged. One-shot feeds now
report unsupported/invalid checkpoints and unrepresentable Items payloads rather
than silently truncating output; feed kinds submitted through the singleton API
use the same guards. Representable raw feeds retain existing behavior, and
non-feed operations gain no checkpoint requirement. Cursor errors use the
canonical packed HTTP/substatus model.

## 7. Design alternatives

| Decision | Rationale |
| --- | --- |
| Retained state rather than per-page serialization | Some buffered plans have no supported bounded token; live paging remains possible independently. |
| Additive versioned ABI rather than expanded legacy records | Existing bindings keep their layouts while cursor results expose complete payloads and distinct states. |
| Wrapper-managed rather than native-managed prefetch | Buffering, polling, and consumption policy remain with the host SDK; the native cursor provides explicit page advancement and stable result lifetimes. |

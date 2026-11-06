<!-- cspell:ignore persistable checkpointing retryability schedulable -->

# Change Feed Processor Public Rust API

**Status:** Draft; proposed API for review, not implemented or approved
**Date:** 2026-10-12
**Authors:** Not specified

## Table of Contents

- [1. Purpose and Scope](#1-purpose-and-scope)
- [2. Existing SDK Contracts](#2-existing-sdk-contracts)
- [3. Proposed Public Surface](#3-proposed-public-surface)
- [4. Prepared Resources and Usage](#4-prepared-resources-and-usage)
- [5. Callback and Processing Contract](#5-callback-and-processing-contract)
- [6. Configuration and Workload Identity](#6-configuration-and-workload-identity)
- [7. Lifecycle and Resource Ownership](#7-lifecycle-and-resource-ownership)
- [8. Failures and Health](#8-failures-and-health)
- [9. LeaseStore Extension Contract](#9-leasestore-extension-contract)
- [10. Planned Acceptance Scenarios](#10-planned-acceptance-scenarios)
- [11. Compatibility and Rollout](#11-compatibility-and-rollout)
- [12. Open Decisions and Alternatives](#12-open-decisions-and-alternatives)
- [13. Evidence and References](#13-evidence-and-references)

## 1. Purpose and Scope

Applications need a supported way to process change-feed batches without
implementing distributed lease coordination themselves. This specification
proposes the Rust application boundary: construction from prepared resources,
an asynchronous typed callback, safe progress persistence, lifecycle control,
and an extension point for lease stores.

It refines [0030: Package Boundaries](0030-change-feed-processor-package-boundaries.md),
not its package decision. Reuse `azure_data_cosmos_driver` for shared feed,
lease, and checkpoint primitives and `azure_data_cosmos_driver_native` for
their existing native distribution path. No CFP-specific driver, native
artifact, process, or second callback-completion protocol is introduced.

**Proposed first scope:** `LatestVersion`, initialization from `Beginning` or
resumption from durable checkpoints, a Cosmos-backed lease store, and
conditional persistence after each successfully processed batch. The public
Rust API is evaluated inside `azure_data_cosmos` first; a companion crate
remains **Open**.

### Goals

- Reuse existing clients, credential contexts, change-feed items, decoding,
  request options, typed errors, and diagnostics.
- Make application completion, checkpoint confirmation, ownership loss, and
  shutdown observably different events.
- Bound processor-owned work while permitting independent lease renewal.
- Describe one schema-independent `LeaseStore` contract that the
  [alternate-store exploration](0032-change-feed-processor-alternate-lease-stores.md)
  can evaluate without inventing another Rust API.

**Non-goals:** exactly-once application effects; a transaction spanning a
callback and a lease write; manual or less-frequent checkpoints; built-in
dead-letter handling; `Now`, point-in-time starts, or
`AllVersionsAndDeletes` in the first scope; automatic store migration;
compatibility with every existing CFP lease format; or support for every
topology transition and backend. Native ABI layouts, lease scheduling
algorithms, and a full internal state machine require separate design.

All CFP names and signatures below are **Proposed**. They are not APIs
available in today's package, and the examples are not compiling samples
against the current SDK.

## 2. Existing SDK Contracts

These are source observations, not additions made by this proposal. Paths in
this document are repository-relative unless stated otherwise.

| Existing surface | Relevant contract and reuse |
| --- | --- |
| `ContainerClient` | Cloneable, prepared container client with private resource/context fields. `DatabaseClient::container_client(...).await` resolves container metadata and normally topology. CFP retains this client's context; there is no public driver-extraction API assumed here. |
| `AccountReference::with_credential` | Accepts an `AccountEndpoint` and `Arc<dyn azure_core::credentials::TokenCredential>`. `CosmosClientBuilder::build` performs client preparation; `with_runtime` attaches a shared `CosmosRuntime`. These remain outside CFP construction. |
| `ContainerClient::query_change_feed<T>` | Takes `FeedScope`, explicit `ChangeFeedStartFrom`, and `Option<ChangeFeedOptions>`; returns `ChangeFeedPageIterator<ChangeFeedItem<T>>`. Its bound is `DeserializeOwned + Send + 'static`. |
| `ChangeFeedItem<T>` | Owns optional current/previous documents and change metadata. Existing accessors and decoder semantics are reused; LatestVersion does not promise every intermediate version or delete. |
| `FeedPage<T>` | Owns its item vector, response headers, and an `Arc<DiagnosticsContext>`. It exposes `items`, `into_items`, `headers`, and `diagnostics`. |
| `ChangeFeedOptions` and `FeedOptions` | Compose request options, mode, session token, page-size hint, continuation, and initial fan-out guard. `ChangeFeedMode::LatestVersion` is the existing default. A page-size hint is not a hard memory or item-count bound. |
| `CosmosError`, `CosmosStatus`, `Result<T>` | The SDK's result alias uses the typed Cosmos error. Errors preserve source chains, optional wire responses, and available diagnostics. A synthetic error is not evidence of a service response. |
| `DiagnosticsContext` and `DiagnosticsHandler` | Existing operation-level diagnostics and emission hooks. Client handlers are configured during client preparation, not replaced by CFP. |

The pull API supports more modes and starts than this initial CFP proposal.
Reuse of its types does not automatically make all their values supported by
CFP. Existing request-option behavior also does not establish lease authority
or prove checkpoint durability.

## 3. Proposed Public Surface

### 3.1 Inventory

For readability, `cfp` below denotes a **proposed**
`azure_data_cosmos::change_feed_processor` module. This alias does not settle
crate placement, Cargo gating, or the location of the store trait's definition.

| Proposed item | Public responsibility |
| --- | --- |
| `ChangeFeedProcessorBuilder<T>` | Accept a prepared feed `ContainerClient`, `Arc<dyn LeaseStore>`, namespace, and instance label. Validate options and register the callback. |
| `ProcessorOptions`, `ProcessorLimits`, `ProcessorTiming` | Describe fixed run configuration, explicit resource limits, and timing policy; see section 6. |
| `ChangeFeedProcessor` | Validated, inert configuration. `start(self)` consumes it and creates one running processor. Callback type erasure is internal. |
| `RunningChangeFeedProcessor` | Non-cloneable, must-use owner of the running tasks. Exposes `health`, `wait`, and `stop`. |
| `ChangeFeedBatch<T>` | Proposed convenience alias for existing `FeedPage<ChangeFeedItem<T>>`, not a new event model or decoder. |
| `ProcessingContext` | Owned, per-invocation context with namespace/lease correlation and cooperative cancellation. It carries no checkpoint method or writable ownership proof. |
| `HandlerResult` | Application success or a boxed `Send + Sync` error. Success means the actual work represented by the batch has completed. |
| `StopMode` | Explicit `Drain { timeout: Duration }` or `Cancel`. |
| `ProcessorHealth`, `ProcessorIssueKind` | Bounded health snapshot and machine-readable failure stage; not a replacement for request diagnostics. |
| `CosmosLeaseStore` | Prepared Cosmos implementation, constructed from a lease `ContainerClient`. It does not provision the container or build another account client. |
| `LeaseStore` and its schema-independent records | Provider injection and semantic lease operations described in section 9. No application `T` or credential parameter belongs to those records. |

No public lease manager, partition supervisor, client factory, or callback
acknowledgement object is proposed.

### 3.2 Signature sketch

**Illustrative interface declarations only.** Imports and method bodies are
omitted; these are not standalone Rust implementations.

```rust
pub type HandlerResult =
    std::result::Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
pub type ChangeFeedBatch<T> = FeedPage<ChangeFeedItem<T>>;

impl<T: DeserializeOwned + Send + 'static> ChangeFeedProcessorBuilder<T> {
    pub fn new(
        feed: ContainerClient,
        store: Arc<dyn LeaseStore>,
        namespace: impl Into<String>,
        instance_name: impl Into<String>,
    ) -> Self;

    pub fn with_options(self, options: ProcessorOptions) -> Self;

    pub fn build<F, Fut>(self, handler: F)
        -> azure_data_cosmos::Result<ChangeFeedProcessor>
    where
        F: Fn(ProcessingContext, ChangeFeedBatch<T>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = HandlerResult> + Send + 'static;
}

impl ProcessorOptions {
    pub fn new(
        start_from: Option<ChangeFeedStartFrom>,
        limits: ProcessorLimits,
        timing: ProcessorTiming,
    ) -> Self;

    pub fn with_change_feed_options(self, options: ChangeFeedOptions) -> Self;
}

impl CosmosLeaseStore {
    pub fn new(container: ContainerClient) -> azure_data_cosmos::Result<Self>;
}

impl ChangeFeedProcessor {
    pub async fn start(self)
        -> azure_data_cosmos::Result<RunningChangeFeedProcessor>;
}

impl RunningChangeFeedProcessor {
    pub fn health(&self) -> ProcessorHealth;
    pub async fn wait(&mut self) -> azure_data_cosmos::Result<()>;
    pub async fn stop(&mut self, mode: StopMode) -> azure_data_cosmos::Result<()>;
}
```

`ProcessingContext::namespace()` and `lease_id()` return borrowed opaque
correlation strings from the owned context. `is_cancelled()` and asynchronous
`cancelled()` expose a read-only cancellation signal. A lease ID is not an
authorization token, a logical partition key, or a permanent physical partition
number. The context and batch can move into the callback future without
borrowing a processor or a transport buffer.

Cancellation is signaled for cancellation shutdown, loss of authority, or
terminal processor failure. Drain alone stops admission without cancelling
an admitted callback; expiration of its drain timeout requests cancellation.
The signal is local and does not itself revoke remote authority.

The proposal uses one completion result: the callback's `HandlerResult`.
Returning `Ok(())` makes the batch eligible for a conditional checkpoint; it
does not itself confirm that checkpoint.

## 4. Prepared Resources and Usage

### 4.1 Preparation boundary

The application first creates clients/resources using existing SDK APIs and
its chosen credentials. It then prepares the lease provider. Only then does
it construct CFP.

The candidate constructor subscribes to the prepared container as a whole.
Per-lease feed-work assignment is internal; an arbitrary `FeedScope` input
is not part of this initial surface. This does not promise support for every
topology transition.

```text
credential-bound feed ContainerClient --------------------+
                                                          |
credential-bound lease ContainerClient -> CosmosLeaseStore +-> CFP builder
                                                          |
or independently authenticated custom LeaseStore ----------+
```

Feed and Cosmos lease clients may share an account and identity, share an
account with different identities, or use different accounts and identities.
CFP preserves each client's credential provider, refresh behavior, options,
diagnostics, and runtime association. Matching endpoint strings do not
authorize substituting one client's credentials for another's.

`CosmosLeaseStore::new` retains the prepared client and performs available
local validation. `start` validates remotely dependent store/schema
requirements. Neither operation creates the database/container. Lease-record
creation, when allowed by the start policy, is a later coordination operation.
Successful metadata reads do not guarantee permission for future lease writes.
A custom provider authenticates and retains its own resources before injection.

### 4.2 Cosmos-backed example

**Illustrative proposal.** `cfp` is the provisional module alias, and the
caller supplies prepared clients, explicit options, and an asynchronous
application function. The Tokio selection is example host orchestration,
not a new CFP runtime requirement.

For fresh initialization, construct the supplied options with
`ProcessorOptions::new(Some(ChangeFeedStartFrom::Beginning), limits, timing)`;
use `None` instead for resume-only. The start type is reused from
`azure_data_cosmos::options`; no numerical defaults are implied here.

```rust
use azure_data_cosmos::{change_feed_processor as cfp, ContainerClient};
use serde::Deserialize;
use std::{future::Future, sync::Arc};

#[derive(Deserialize)]
struct Order {
    id: String,
}

async fn run_orders<F, Fut>(
    feed: ContainerClient,
    lease_container: ContainerClient,
    options: cfp::ProcessorOptions,
    apply_batch: F,
    shutdown: impl Future<Output = ()>,
    drain_timeout: std::time::Duration,
) -> azure_data_cosmos::Result<()>
where
    F: Fn(cfp::ProcessingContext, cfp::ChangeFeedBatch<Order>) -> Fut
        + Send + Sync + 'static,
    Fut: Future<Output = cfp::HandlerResult> + Send + 'static,
{
    let store = Arc::new(cfp::CosmosLeaseStore::new(lease_container)?);
    let processor = cfp::ChangeFeedProcessorBuilder::<Order>::new(
        feed, store, "orders-projection-v1", "worker-a",
    )
    .with_options(options)
    .build(move |context, batch| {
        let work = apply_batch(context, batch);
        async move { work.await }
    })?;

    let mut running = processor.start().await?;
    tokio::select! {
        result = running.wait() => result,
        () = shutdown => running.stop(cfp::StopMode::Drain {
            timeout: drain_timeout,
        }).await,
    }
}
```

The application's `apply_batch` future must include the work whose success
permits persistence, including awaiting any tasks it starts for that work.
Enqueueing items or spawning detached work and immediately returning success
would checkpoint before that work finishes. CFP cannot infer or repair such
an application-level early acknowledgement.

### 4.3 Injected-provider example

**Illustrative proposal, not a shipping provider implementation.** The
application has already authenticated, configured, and validated a custom
provider implementing the section 9 contract. The builder does not receive
that provider's connection string or credentials. This sketch reuses the
imports from the previous example.

```rust
fn configure_custom<T, F, Fut>(
    feed: ContainerClient,
    prepared_store: Arc<dyn cfp::LeaseStore>,
    options: cfp::ProcessorOptions,
    handler: F,
) -> azure_data_cosmos::Result<cfp::ChangeFeedProcessor>
where
    T: serde::de::DeserializeOwned + Send + 'static,
    F: Fn(cfp::ProcessingContext, cfp::ChangeFeedBatch<T>) -> Fut
        + Send + Sync + 'static,
    Fut: Future<Output = cfp::HandlerResult> + Send + 'static,
{
    cfp::ChangeFeedProcessorBuilder::<T>::new(
        feed, prepared_store, "orders-projection-v1", "worker-b",
    )
    .with_options(options)
    .build(handler)
}
```

This example describes injection, not an online migration from the first
example's store. Moving the namespace to another store requires a separate,
coordinated migration decision.

## 5. Callback and Processing Contract

### 5.1 Ownership and concurrency

**Proposed initial coupling:** `ProcessorLimits::max_concurrent_leases` caps
owned lease runners, including their acquisition, idle, and pending-checkpoint
periods. Reserve a runner slot before acquisition; do not accumulate additional
owned leases outside that cap. Each runner admits at most one batch, including
its callback and pending checkpoint, so the same value also bounds concurrent
callbacks across leases. Owned-lease count and active-callback count are not
identical. Independent ownership and callback-concurrency controls remain
**Open**. No cross-lease ordering is promised.

The handler is `Fn + Send + Sync + 'static`, not `FnMut`. Shared application
state therefore needs application-selected synchronization. Each invocation
receives an owned context and owned batch; `T` need not be `Clone` or `Sync`.
Application code borrowing `batch.items()` across an `await` may itself need
stronger bounds to keep its future `Send`; consuming `batch.into_items()`
does not add a blanket `T: Sync` requirement to CFP.

Processor byte limits budget admission and retention of raw response buffers;
they do not guarantee an end-to-end peak-memory bound. There is no next-batch
prefetch for a lease. Capacity is reserved before admitting a read, but that
reservation does not cap allocations inside the existing driver/transport
while fetching or assembling its response. Rejecting an oversized returned body before invoking the
callback or checkpoint cannot undo an allocation already made there.

A stronger end-to-end bound would require lower-layer bounded-read support
that enforces limits before/during buffering; that is an implementation
prerequisite, not established by the current pull API or a page-size hint.
The proposed budgets also do not bound arbitrary allocations inside
`T::deserialize`, application work, or items the application retains after its
callback. Decoded-allocation measurement remains open.

### 5.2 Processing and persistence

For each delivered batch, the consuming Rust SDK performs:

```text
read under current lease authority
    -> deserialize using existing SDK semantics
    -> invoke and await the application's callback
    -> revalidate authority and invoke conditional checkpoint
    -> await confirmed persistence
    -> admit the next read/batch for that lease
```

Renewal runs independently of the callback and of application backpressure.
Its execution capacity must not be exhausted by callback slots or buffered
batches. This is a scheduling contract, not protection against an application
blocking every executor thread.

Only successful completion of the actual callback future permits checkpoint
submission. A callback error, panic, or cancellation does not. If persistence
is definitely uncommitted across the logical operation's entire attempt
history, retry the checkpoint for the same successful batch while authorized;
do not unnecessarily rerun the callback. If persistence is uncertain, suspend
that lease's next batch. Reconciliation belongs to the provider/shared lease
layer; section 9.4 records the currently undesigned public recovery seam and
the safe-quiescence baseline. An earlier confirmed checkpoint never restores
ownership subsequently lost.

An idle change-feed response is not end-of-processing. An empty poll need not
invoke an application callback. If it advances a persistable cursor, that
empty progress still requires conditional confirmation before another read
advances that lease; it cannot bypass pending nonempty work.

These rules provide **at-least-once processing**, not atomicity with application
effects. A crash, loss of authority, or cancellation after application effects
but before checkpoint confirmation can cause replay. Applications must make
effects idempotent or deduplicate using their own durable identifiers; a lease
ID or callback invocation number is not an exactly-once event identifier.

## 6. Configuration and Workload Identity

### 6.1 Scope, precedence, and defaults

**Proposed:** configuration is immutable for a run. `with_options` replaces
the complete processor-options value, rather than merging hidden settings.
`ProcessorOptions::new` requires the start policy, limits, and timings.
Omitting processor options is a build error in this draft; it does not select
unreviewed numerical defaults.

| Setting | Proposed meaning and validation | Default or precedence |
| --- | --- | --- |
| `namespace` | Required, nonempty logical subscription identity. No silent normalization that changes durable identity. | Explicit builder input; not derived from the instance label. |
| `instance_name` | Required, nonempty diagnostic label. Not sufficient proof of ownership; a run/acquisition has a distinct ownership identity. | Explicit input; unique labels are recommended for operations. |
| `start_from: Option<ChangeFeedStartFrom>` | `None` means require existing checkpoints for the required work. `Some(Beginning)` permits initializing work lacking a checkpoint. Existing checkpoints always win. | Required argument; no implicit rewind or bootstrap fallback. |
| `change_feed: ChangeFeedOptions` | Reuse request settings and decoding semantics. Initially only `LatestVersion` is accepted; caller-supplied continuation tokens are rejected because CFP owns per-lease progress. | Initialize with the existing `ChangeFeedOptions::default()`; an explicit value replaces it. |
| `ProcessorLimits` | Positive `max_concurrent_leases: NonZeroU32` initially couples the owned-runner cap and derived callback upper bound. Positive `max_batch_bytes: NonZeroUsize` and `max_buffered_bytes: NonZeroUsize` bound admitted/retained raw response buffers, including decoding admission; batch-byte limit must fit the total budget. They do not cap earlier driver/transport peak allocations. | Explicit; numerical defaults and independent concurrency controls are **Open**. These are not the feed's `max_fan_out`. |
| `ProcessorTiming` | Positive `poll_interval`, `acquire_interval`, `renew_interval`, `lease_duration`, and `checkpoint_retry_budget`, all `Duration`. Renewal and bounded request/retry work must leave a feasible safety margin before authority expires. | Explicit; numerical defaults and provider-specific safety margins are **Open**. |
| Cosmos request settings | Feed overrides apply only to feed operations. Lease operations use their prepared provider/client configuration. CFP-managed conditions and identity cannot be overridden by headers. | Existing operation → client/account → runtime → environment resolution remains in force for inherited request options. |
| Stop mode and drain timeout | The caller chooses `Cancel` or a positive drain timeout. Draining does not extend an expired lease. | Explicit for `stop`; no invented grace-period default. |

The existing `FeedOptions::max_fan_out` is an initial planning guard, not a
runtime cap on callbacks or lease inventory. Its existing `None`/zero
semantics must not be reinterpreted as CFP concurrency settings. Likewise,
`max_item_count` remains a service hint. Any inherited request setting that
cannot preserve the CFP contract must be rejected, not silently ignored.

There are no new CFP environment variables or hot-reload setters proposed.
Credential refresh continues through the retained providers; immutable CFP
configuration does not freeze access tokens. Existing lower-layer options
retain their documented resolution behavior.

### 6.2 Validation boundary

`build` performs local validation without I/O: missing configuration, invalid
limits/timings, unsupported mode/start, supplied continuation, and attempts to
override CFP-controlled conditions fail here. Nonzero timings alone do not
prove a safe renewal budget.

`start` validates facts requiring prepared resource metadata or provider I/O:
the source/mode binding, store schema and runtime prerequisites, readable
checkpoint versions, necessary checkpoints under resume-only policy, and
compatible timing/consistency guarantees. A later authorization failure is
still possible and remains a runtime failure. Provider semantic conformance
is a prerequisite, not something startup can prove by probing generic CRUD.

### 6.3 Durable identity

A lease namespace is bound to the resolved source account/container identity,
feed mode, and lease/checkpoint format contract. Regional routing changes do
not create a new source. Recreating a container with the same display name
does not make its old checkpoints valid.

Namespace identity survives instance restarts and rolling upgrades. Instance
labels, credential changes, or a new `start_from` value must not erase it.
Changing application handler code or `T` does not automatically rewind
progress; an intentional replay uses a deliberately different subscription
namespace or a separately designed administrative migration/reset.

Source, mode, scope, or format mismatches fail without overwriting records.
Required work missing a checkpoint under resume-only policy fails explicitly;
it does not quietly start at the beginning. Lease identity must describe owned
feed work across supported topology changes, not assume physical partition
numbers are permanent. Exact split/merge handoff coverage remains a release
decision; unsupported transitions stop affected work visibly.

## 7. Lifecycle and Resource Ownership

| Operation or event | Proposed contract |
| --- | --- |
| `build` | Returns inert validated configuration. No acquisitions, callbacks, or background work. |
| `start(self)` | Consumes the configuration; performs readiness checks and starts supervised work. Readiness means valid startup and a running acquisition loop, not ownership of every lease or proof of all future permissions. A new start requires a new configured processor. |
| Cancellation of `start` | No running worker is left without an owning handle. Startup I/O may already have created a lease record; cancellation does not undo it. Bootstrap must tolerate that record on the next attempt. |
| `wait(&mut self)` | Observes termination and joins processor-owned tasks. Cancelling this observation does not cancel the processor. The terminal result is retained so later `wait` or `stop` observes the same outcome. |
| `stop(Drain { timeout })` | Stops acquisitions and new batch admission. Awaits already admitted work and its eligible checkpoints while renewing independently and still authorized. Then conditionally releases ownership and joins tasks. |
| Drain timeout | Requests cancellation of remaining work and returns a shutdown failure after processor tasks are settled. It is not reported as a clean drain. The timeout is not a hard preemption guarantee for non-yielding application code. |
| `stop(Cancel)` | Signals cancellation and stops polling/drops in-flight callback futures at a safe scheduling opportunity; no new checkpoint is authorized by their cancellation. Already submitted writes can still commit. Conditional release is best effort if authority is still known. |
| Cancelling a `stop` future | Once polled and requested, stopping stays requested. A later `wait` or `stop` can await completion; cancellation cannot restart admission. |
| Repeated stop | Idempotent stop request. It may escalate Drain to Cancel, but cannot reverse Cancel to Drain. Terminal failures remain failures rather than becoming `Ok` on a second call. |
| `Drop` | Requests cancellation through the owned task handles. It cannot await callbacks, guarantee remote release, confirm a last checkpoint, or return failures. Explicit `stop`/`wait` is the supported completion path. |

The running handle supervises all processor-owned tasks; failure must not be
left solely in a detached task. Dropping the handle must not leave a continuing
processor. Best-effort cancellation cleanup may outlive `Drop` briefly; it
uses bounded policy and existing diagnostic emission, not continued batch
processing. Runtime destruction or process termination can prevent cleanup
and reporting entirely.

CFP retains references/clones to the supplied clients, store, and their
resource contexts until its work settles. It neither shuts down a shared
`CosmosRuntime` nor disposes a custom provider's caller-owned shared pool.
Other client references remain usable after a processor stops. Retaining a
Cosmos resource runtime does not keep the application's async executor alive;
the application must keep that executor running through shutdown.

No stop mode can roll back user effects or a remote write. Work explicitly
detached by an application is outside CFP's task ownership, even if the
callback future is cancelled.

## 8. Failures and Health

### 8.1 Failure policy

**Proposed initial policy (Open):** terminal callback/decode failures stop the
processor rather than silently skipping a batch or retrying user code forever.
Ordinary ownership handoff is not automatically a processor-wide failure.
This run-wide failure scope remains a review choice, not an agreed policy;
the prohibition on checkpointing failed work does not depend on that choice.

| Condition | Observation and proposed policy |
| --- | --- |
| Transient feed/provider request error | Use bounded applicable retry policy; expose stage and existing diagnostics. Exhaustion/permanent failure becomes observable through health and terminal result. Retry/wrapping layers must preserve earlier mutation uncertainty, not merely the final attempt's status, and share a total budget. |
| Deserialization or callback error | No checkpoint for that batch. Preserve the error/source and lease correlation; stop new work and cancel remaining workers. A later run can replay from durable progress. Independently committed batches remain committed. |
| Handler panic under unwinding | Observe failure of both handler invocation and future polling; do not checkpoint the panicking batch or reuse the suspect handler. Terminate the processor with an observable panic classification. This is containment, not recovery of application invariants. |
| `panic = "abort"` or process termination | No in-process recovery, join, release, or diagnostics guarantee. Expiry and durable checkpoints govern subsequent recovery. |
| Ownership lost or expired | Stop that lease's admission, signal its callback's cancellation, and reject late results. Never retry its checkpoint under obsolete authority. Other leases may continue. |
| Definitely uncommitted checkpoint | Only a whole-logical-operation non-commit guarantee permits this retry. Retry the same checkpoint while the successful callback remains recorded locally and ownership is valid; do not rerun the callback. Bound retries by policy and authority. |
| Uncertain checkpoint outcome | Suspend the lease and expose uncertainty. Provider/shared-layer reconciliation must confirm persistence separately from current authority; the SDK cannot interpret opaque records/receipts as proof. If that evidence or the recovery seam is unavailable, stay quiescent or terminate with uncertainty preserved. |
| Renewal/checkpoint revision race | A changed provider revision is not necessarily a new owner. Refresh/reconcile only if the same valid ownership is established; otherwise treat authority as lost or unknown. |
| Cleanup/release failure | Preserve the failure and possible outstanding remote effect. Never claim successful release merely because local tasks ended. Lease expiry remains a recovery mechanism. |

Fatal failure initiates cancellation of other workers, not a transaction that
undoes their confirmed progress. A successful callback that finishes after
cancellation or lost ownership cannot reopen checkpoint permission.
Unexpected processor/provider task panics are also terminal; a panic after
submitting a write does not establish that the write was uncommitted.

### 8.2 Health and diagnostics

`ProcessorHealth` is a bounded snapshot: state (`Running`, `Stopping`,
`Stopped`, or `Failed`), owned-lease and active-callback counts, the latest
issue, cumulative issue counts, and the latched terminal failure if present.
It must not retain an unbounded per-batch history.

The proposed `ProcessorIssueKind` distinguishes configuration, feed,
deserialization, handler error, handler panic, lease-store operation,
ownership loss, uncertain checkpoint, and shutdown. An issue includes the
operation stage and safe namespace/lease correlation; terminal results use
the SDK's `Result<T>`/`CosmosError` and preserve available source errors.
Exact new Cosmos status/substatus assignments remain **Open**, rather than
inventing numeric codes in this draft.

Existing feed and Cosmos lease diagnostics retain status, request charge,
activity/retry/region information where available. Callback and non-Cosmos
provider errors must not manufacture Cosmos wire responses or RU charges.
Processor-level signals add callback duration, renewal delay, checkpoint
wait/uncertainty, and shutdown outcome; they do not replace the prepared
clients' handlers.

Opaque checkpoints, ownership proofs, authentication material, and document
bodies are not logged by default. Correlation labels need privacy-aware
handling. A health snapshot is not a persistence or ownership proof.
An event stream and lag estimator are extensions, not required initial APIs.

## 9. LeaseStore Extension Contract

### 9.1 Port boundary and type ownership

`LeaseStore` is a semantic provider contract, **not generic CRUD**. The store
persists lease/progress metadata, never deserializes application `T`, and never
obtains feed credentials from the processor's configuration.

The shared implementation must be able to use the port without depending on
`azure_data_cosmos`. The port's descriptors use schema-independent, owned data
and ordinary Rust async types. They must not require `ContainerClient`,
`ChangeFeedItem<T>`, or the SDK's typed decoder.

**Open placement choice:** define a narrowly stable port below the SDK and
deliberately re-export it, or expose a public Rust API with a compatibility
adapter to an independent driver port. The former needs an explicit stable-type
sharing decision; the latter carries adapter/versioning cost. The `cfp::LeaseStore`
spelling denotes the application-facing seam, not an already chosen
definition crate. This does not introduce a new CFP driver/native artifact.

### 9.2 Canonical candidate signatures

All names here are **Proposed** and canonical for this draft's companion
exploration. Boxed, `Send` futures make provider injection object-safe without
requiring a particular async-trait macro. Borrowed arguments remain valid
until the returned future completes or is dropped.

```rust
pub type LeaseStoreFuture<'a, T> = std::pin::Pin<Box<
    dyn Future<Output = std::result::Result<T, LeaseStoreError>> + Send + 'a
>>;

pub trait LeaseStore: Send + Sync + 'static {
    fn inventory<'a>(
        &'a self, scope: &'a LeaseScope, cursor: Option<&'a InventoryCursor>,
    ) -> LeaseStoreFuture<'a, LeaseInventoryPage>;

    fn create<'a>(&'a self, definition: &'a LeaseDefinition)
        -> LeaseStoreFuture<'a, LeaseRecord>;

    fn inspect<'a>(&'a self, key: &'a LeaseKey)
        -> LeaseStoreFuture<'a, Option<LeaseRecord>>;

    fn acquire<'a>(
        &'a self, observed: &'a LeaseRecord, owner: &'a LeaseOwner,
        duration: Duration,
    ) -> LeaseStoreFuture<'a, OwnedLease>;

    fn renew<'a>(&'a self, lease: &'a OwnedLease, duration: Duration)
        -> LeaseStoreFuture<'a, OwnedLease>;

    fn checkpoint<'a>(&'a self, lease: &'a OwnedLease, progress: &'a FeedCheckpoint)
        -> LeaseStoreFuture<'a, OwnedLease>;

    fn release<'a>(&'a self, lease: &'a OwnedLease)
        -> LeaseStoreFuture<'a, ()>;
}
```

| Record | Semantic content; representation is not frozen |
| --- | --- |
| `LeaseScope`, `LeaseKey`, `LeaseDefinition` | Bound namespace/source/mode identity, lease's owned feed-work descriptor, and bootstrap definition. Work descriptors are supplied by the shared feed implementation, not inferred by the backend from a physical partition number. |
| `LeaseRecord` | Observed durable definition/checkpoint and provider ownership/revision metadata. Observation alone does not grant ownership. |
| `LeaseOwner`, `OwnedLease` | Ownership incarnation and current provider proof/revision returned by successful conditional acquisition or mutation. The instance label alone is insufficient. |
| `FeedCheckpoint` | Versioned, opaque feed continuation/progress for the bound work. The store preserves it exactly and does not parse it as a revision or application document. |
| `InventoryCursor`, `LeaseInventoryPage` | Provider-owned paging cursor and a bounded page of records, with explicit continuation/completion. Inventory cursors are not feed checkpoints. |
| `LeaseStoreError`, `LeaseStoreErrorKind` | Typed classification, retryability where meaningful, and preserved provider error source. They do not require an SDK application type or fabricated Cosmos response. |

Provider revisions/ownership proofs and feed checkpoints are distinct opaque
values with different lifetimes. The shared implementation must not interpret
an ETag, row version, expiry timestamp, or checkpoint as interchangeable.
Representations must support safe ownership across await points and redact
sensitive opaque values from default debug output.

### 9.3 Required semantics

| Operation/capability | Required behavior |
| --- | --- |
| Scope binding | Inventory validates an existing namespace's source/mode binding rather than filtering a mismatch into an empty result. Create conditionally establishes that binding if absent and rejects a conflicting binding before initializing work. The binding cannot change through ordinary lease operations. |
| Inventory | A complete enumeration for the requested scope, possibly paged and with explicitly documented concurrent-change behavior. Final-page completion cannot mean “query failed,” “access denied,” or “only a convenient subset.” Partial enumeration is not evidence that leases are absent. |
| Create | Conditional create-if-absent with an existing-record outcome that preserves its progress. Concurrent bootstrap cannot overwrite another worker's checkpoint. Required when the run must initialize missing work; a pre-provisioned-only provider must reject unsupported initialization before processing. |
| Inspect | Distinguish a record, authoritative absence at observation time, and failure to observe it. Supports provider/shared-layer investigation, not generic SDK interpretation of receipts; an absent value cannot rule out a late write. |
| Acquire | Atomically test observed state and ownership eligibility, establish a new ownership incarnation/proof, and preserve checkpoint progress. Never unconditional last-writer-wins takeover. |
| Renew | Conditionally extend current authority without resetting progress. Reject obsolete ownership. Provider time/expiry semantics must establish a safe validity window; a client's wall clock alone is insufficient proof. |
| Checkpoint | Atomically validate current authority and the applicable revision, persist the supplied progress, and return confirmed durable state. Reject obsolete owners and delayed older mutations; do not overwrite a successor's progress. |
| Release | Conditionally relinquish only the caller's current ownership and preserve progress. A delayed release cannot evict the successor. |
| Fencing | Every authority-sensitive mutation rejects obsolete ownership, including delayed requests arriving after expiry or takeover. Cancellation alone neither invalidates remote authority nor rolls back a write. Fencing lease metadata does not fence arbitrary application side effects. |

The provider must tolerate concurrent calls for different leases. Renewal
must remain schedulable while a callback runs. Renew and checkpoint changes
to one lease's proof/revision must be coordinated so an old local snapshot
cannot undo a newer mutation. How the coordinator serializes conditional
writes is internal; this API does not require holding a lease-store lock
through application execution.

Implementing the trait asserts the required semantics, to be checked by a
provider conformance suite. Startup performs the required inventory/bootstrap
operations before admitting callbacks; it must not infer atomicity from a
successful metadata read. Optional efficiencies include server-side filtering,
notifications, bulk operations, and indexing; none may weaken the baseline
semantics. Atomic split/merge handoffs, migration/export, administrative reset,
and a public recovery carrier/helper are not provided by this seven-operation
sketch; the reconciliation gap is explicit in section 9.4.
Their availability cannot be inferred from generic transaction support.

### 9.4 Outcomes, uncertainty, and unsupported behavior

`LeaseStoreError::kind()` returns the proposed `LeaseStoreErrorKind`;
`is_retryable()` indicates whether the failed operation is eligible for retry
under its stated conditions, not permission to bypass reconciliation.
The proposed classifications are:

- `NotCommitted`: every attempt of the logical mutation is conclusively
  uncommitted, with no unresolved attempt that could later commit.
  An explicit retryability indication and source distinguish temporary
  unavailability from permanent permission/configuration failures.
- `PreconditionFailed`: conditional rejection with no earlier attempt of the
  logical mutation committed or unresolved. This alone does not distinguish a
  harmless renewal revision change from ownership loss.
- `OwnershipLost`: current authority is invalid; obsolete proof cannot be
  reused for another checkpoint.
- `OutcomeUnknown`: the operation might have committed. Timeout or
  cancellation after submission commonly belongs here.
- `Unsupported` or `Incompatible`: the provider cannot satisfy a required
  operation, identity, format, or consistency contract.

These outcomes describe the **whole logical provider operation**, including
all internal attempts. A final conditionally rejected retry cannot prove that
an earlier ambiguous attempt did not commit. Provider, shared lease, driver,
and SDK retry/error-wrapping layers must preserve unresolved uncertainty as
`OutcomeUnknown`; a later status or error reason cannot erase it. Successful
mutation results mean durable confirmation under the provider's stated
failure model, not acceptance for later processing.

**Reconciliation ownership:** the provider/shared lease layer owns logical
operation context. Where a backend uses operation IDs/receipts, that context
must keep a stable ID across internal attempts; backend-specific receipt
interpretation belongs inside that layer, not in generic SDK inspection of
opaque `LeaseRecord` fields. An operation ID is distinct from a feed
checkpoint, provider revision, ownership proof, or instance label.

**Open seam:** the candidate `checkpoint(&OwnedLease, &FeedCheckpoint)` boundary
has no explicit operation-context parameter or recovery carrier/helper. It
does not yet define how an ID/receipt survives a separate method invocation,
a dropped future, or a process restart. Internal reconciliation while the
provider retains its context is possible, but cross-call recovery must not be
assumed from the two opaque inputs. Designing any public helper/carrier
remains open; this draft adds no competing signature or acknowledgement
protocol to fill that gap.

A receipt can confirm historical persistence only. It never grants current
or successor authority, refreshes an ownership proof, or undoes ownership
loss; authority must be established separately before further work. A stale
read or absent value cannot prove an in-flight write will never commit.
Without adequate provider/shared-layer evidence or a defined recovery path,
the baseline is safe quiescence or termination with uncertainty retained, not
callback replay, advancement, or unguarded retries. Unknown acquisition or
renewal outcomes likewise cannot grant authority by assumption.

Dropping an operation future is not remote rollback. A write may finish after
the caller stops waiting; the provider's conditional ownership/revision
guarantees must still prevent that late write from corrupting newer state.
These are persistence semantics, not a second application-success protocol.

## 10. Planned Acceptance Scenarios

The following are **planned checks**, not executed test results. Deterministic
fake providers, controlled callback futures, and a controllable clock should
cover contract behavior first; any later authenticated provider validation
requires separate authorization and setup.

| Scenario | Observable acceptance condition |
| --- | --- |
| Prepared resources and identities | Feed and lease operations use their respective supplied contexts, including same-account/different-identity and different-account cases; no hidden client construction or credential substitution occurs. A lease-write denial is surfaced independently of successful feed access. |
| Invalid construction | Unsupported mode/start, a supplied continuation, missing options, impossible limits/timings, or conflicting managed conditions fail before background work or callbacks. |
| Slow callback and concurrency | A suspended callback blocks its lease's next read/checkpoint, while renewal and other runners can proceed. Acquisition/owned-runner slots remain capped even when no callback is active; concurrent callbacks obey the intentionally coupled upper bound. |
| Actual asynchronous completion | No checkpoint is submitted before the application future resolves successfully. Awaited child work delays success; a failed child returned by the callback leaves progress unchanged. |
| Callback error and unwind panic | No checkpoint for the failing batch; health and `wait` expose the stage/cause. Handler panic terminates rather than disappearing in a background task. Abort mode is documented as outside recovery guarantees. |
| Definitely uncommitted checkpoint | Multiple checkpoint attempts can occur with one successful callback invocation; no next batch is admitted before confirmation. Retry-budget exhaustion remains visible. |
| Unknown persistence and late completion | An ambiguous attempt followed by a rejected retry remains `OutcomeUnknown` through every wrapper. Provider/shared-layer evidence may confirm historical persistence, never successor authority. Missing operation context/recovery support leaves work quiescent; a delayed old write cannot overwrite successor state. |
| Ownership and revision races | Concurrent renew/checkpoint can cause a revision conflict without falsely granting authority. After actual takeover, every obsolete renew/checkpoint/release is rejected, even if an earlier checkpoint was confirmed. |
| Inventory and bootstrap | Paged complete enumeration, failed partial enumeration, and racing create calls are distinguishable. Missing checkpoints under resume-only fail; bootstrap preserves existing records. |
| Durable identity and restart | A valid checkpoint wins over `Beginning`. Mismatched source/mode/format or recreated source fails without reset; instance-label changes do not reset progress. |
| Stop, cancellation, and Drop | Drain waits for eligible work; Cancel never authorizes new progress; repeated stop preserves failures. Explicit completion settles owned tasks and leaves shared clients usable. Drop cannot leave ongoing processing or claim a completed drain. |
| Idle and capacity limits | An idle poll is not terminal; empty progress obeys checkpoint authority. Admission/retained-buffer budgets hold and oversized returned batches fail without checkpoint advancement. Separate lower-layer allocation instrumentation is required before claiming a peak bound; post-buffer size rejection alone is insufficient. |
| Diagnostics and rolling versions | Typed source errors and available request diagnostics survive wrapping without exposing payloads/secrets. Mixed versions reject unreadable formats without destructive rewriting; unsupported topology handoff is visible. |

## 11. Compatibility and Rollout

The pull API remains unchanged. Reusing `ChangeFeedItem<T>` means decoder
fixes and metadata semantics remain aligned; the proposed alias does not
create a second serialization contract.

**Proposed rollout:** review the API and store contract, establish deterministic
conformance checks, and evaluate a Cosmos-backed implementation behind an
opt-in preview surface. ADR-0007 specifies `preview_<name>`; the current
`azure_data_cosmos/Cargo.toml` follows it with `preview_opentelemetry`,
`preview_dtx`, and `preview_patch`. An in-SDK candidate name is therefore
`preview_change_feed_processor`, still **Open** and not an existing feature.
Disabled CFP must add no polling, lease writes, or CFP background tasks.
A companion remains an alternative distribution decision, not a dependency
created by this document.

Before release, specify supported source topologies, lease-container
partitioning/schema, token versions, and mixed-version ownership behavior.
Do not infer CFP split/merge correctness solely from existing pull-feed
topology recovery. Unknown schema/token versions must preserve stored bytes
and fail safely. Reading a newer checkpoint is a compatibility requirement
to prove, not something rollback can solve by deleting it.

SDK types and any exposed store trait are public compatibility commitments.
Moving the public Rust API to a companion or re-exporting driver-owned types
requires a deliberate type-identity/versioning policy. Compatible dependency
versions are required; no “types-only Cargo dependency” is assumed.

No compatibility with .NET/Java lease documents is claimed here. Switching
providers, changing a namespace's source/mode, or rewriting lease formats
requires a migration plan that prevents concurrent divergent owners.

## 12. Open Decisions and Alternatives

| Decision | Current proposal and remaining review |
| --- | --- |
| API placement and feature gate | Evaluate `azure_data_cosmos` first. Compare opt-in footprint and release coupling against a companion; do not expose private driver handles as an accidental workaround. |
| Store interface ownership | Choose stable lower-layer definition/re-export versus a public Rust API with a compatibility adapter. Resolve compatibility and whether custom-provider APIs publish in the first preview or after built-in conformance work. |
| Limits, defaults, and timing | Require explicit positive limits/timings initially. Independent ownership/callback controls, usable defaults, provider margins, request budgets, and decoded-memory accounting remain open. End-to-end peak bounds additionally require lower-layer bounded-read support. |
| Callback shape | Owned batches plus `Fn` permit concurrent per-lease processing without callback lifetimes or mandatory `T: Sync`. A borrowed/zero-copy alternative needs a separate lifetime and performance argument. |
| Error policy | **Proposed/Open:** fail the run on terminal callback/decode error or unwind panic. Run-wide versus per-lease failure scope is not agreed; neither may checkpoint failed work. Retry customization, dead-letter policy, and typed status mappings remain open. |
| Provider completion model | Preserve logical-operation uncertainty across every attempt/wrapper. Stable operation context and a public cross-call/restart recovery carrier/helper are undesigned; provider/shared-layer reconciliation or safe quiescence is the baseline. |
| Scope and topology | Set an explicit first-release support matrix and handoff protocol. `Now`, `AllVersionsAndDeletes`, arbitrary scopes, less-frequent checkpoints, and advanced stores remain deferred. |
| Lease compatibility | Define namespace encoding, key/partition schema, ownership incarnation representation, and rolling token/record compatibility. No migration or old-format compatibility is approved by this draft. |

The package reuse and processing order are the shared **proposed baseline**
from 0030. None of the choices in this document records repository approval.

## 13. Evidence and References

- [0030: Change Feed Processor Package Boundaries](0030-change-feed-processor-package-boundaries.md):
  shared layering, prepared-resource boundary, and callback/checkpoint order.
- [0032: Alternate Lease Stores](0032-change-feed-processor-alternate-lease-stores.md):
  companion exploration; evaluates semantic mappings to section 9 rather
  than defining competing Rust signatures.
- Existing client declarations:
  `sdk/cosmos/azure_data_cosmos/src/clients/container_client.rs`
  (`ContainerClient`, `query_change_feed`),
  `src/clients/database_client.rs` (`container_client`),
  `src/clients/cosmos_client_builder.rs` (`build`, `with_runtime`,
  `with_default_operation_options`, `with_diagnostics_handler`),
  `src/account_reference.rs` (`with_credential`), and
  `src/runtime.rs` (`CosmosRuntime`), with the shorter paths relative to
  `sdk/cosmos/azure_data_cosmos`.
- Existing data/options declarations, relative to that same crate:
  `src/models/change_feed_item.rs`, `src/feed/page.rs`,
  `src/options/change_feed.rs`, and `src/options/feed.rs`.
- Existing errors and diagnostics:
  `sdk/cosmos/azure_data_cosmos/src/error.rs`,
  `sdk/cosmos/azure_data_cosmos/src/diagnostics/mod.rs`, and
  `sdk/cosmos/azure_data_cosmos_driver/src/error/mod.rs`
  (`CosmosError`, its source/response/diagnostics accessors).
- Existing credential trait:
  `sdk/core/azure_core/src/credentials.rs` (`TokenCredential`).
- Feature-prefix evidence:
  `sdk/cosmos/azure_data_cosmos/Cargo.toml` (`preview_opentelemetry`,
  `preview_dtx`, `preview_patch`) and the ADR below.
- Repository context:
  `sdk/cosmos/docs/Project.md`, `Architecture.md`,
  `adrs/0007-cargo-feature-categories.md`, and
  `specs/0029-native-feed-cursor.md`, with the shorter paths relative to
  `sdk/cosmos/docs`. The native cursor contract is background evidence;
  this draft does not add or redefine its ABI.

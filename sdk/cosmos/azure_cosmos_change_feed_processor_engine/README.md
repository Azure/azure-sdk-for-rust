# Azure Cosmos DB Change Feed Processor Engine

Schema-agnostic execution engine for the Azure Cosmos DB change feed processor.

This crate is unpublished and experimental.

## Managed execution

`ProcessorEngine::start_managed` connects the existing preparation, bootstrap,
balancing, and lockstep lease coordinator into continuous execution. It accepts
independently prepared lease-driver/container references and a completion-bearing
shared raw callback. Application schemas remain outside the engine.

`ManagedProcessorOptions` separates host lease capacity from callback concurrency.
Both are bounded by the existing 32-record workload protocol. Startup first loads
the saved winning plan; only a missing workload triggers source range discovery
and fresh mode/start-compatible positions. Now positions are validated as concrete
driver checkpoints, not re-evaluated after restart.

`ManagedProcessor` retains its coordinator and lease task set. `stop` borrows the
handle, so caller cancellation does not drop ownership of its join barrier. Later
stop calls resume joining and return the same cached report after completion.
Inspect processing, maintenance, task, and release failures; `is_complete` and
`is_clean` answer different questions. Drop cancels work without claiming release.

Transient reads reopen from confirmed progress. Callback failures are isolated
and surfaced. Ambiguous checkpoint recovery revokes the old session, rereads
storage, and conditionally proves the same owner/generation with a new revision
before retrying only an unpersisted candidate. It cannot adopt a replacement owner
or rerun a successful callback just because checkpoint transport failed.

Snapshots distinguish feed-response activity, callback completion, and confirmed
checkpoint writes. Elapsed-time markers are not a service item-count lag estimate.
There is no globally registered zero-lease worker list or weighted balancing.
Managed execution currently requires Tokio, and callbacks must yield cooperatively.

The durable handoff storage protocol stages a quiesced parent and all pending
children atomically, then conditionally retires the parent and activates the
complete child set. Restart resumes its saved journal; children cannot acquire
before activation. Parent evidence remains retained and counts toward the
32-record bound. Driver checkpoint derivation is required; opaque tokens are
never edited or manufactured by CFP. Periodic reconciliation refreshes topology,
stops and joins only the affected local parent, and uses its latest confirmed
position to stage replacements. Unrelated lease tasks continue. A concurrent
reacquisition changes the parent revision and prevents the stale handoff.
Physical merges keep logical scopes separate. At the 32-record bound or with
an unsupported buffered checkpoint shape, subdivision is visibly deferred and
the logical parent is retained.

## Data-plane reads

`ProcessorEngine` connects through `azure_data_cosmos_driver`,
resolves an existing monitored container, and opens a retained `ChangeFeedReader`.
LatestVersion and AllVersionsAndDeletes use the driver's operation factories.
It returns a complete raw response with status, headers, diagnostics, and an
opaque continuation. Application document decoding belongs in
`azure_cosmos_change_feed_processor`.

An absent continuation starts at the beginning; an idle partition returns a
304 page rather than ending the feed. Non-idle pages can also be empty.
`ChangeFeedReadOptions::new(range, start)` configures the required source range
and initial position; optional overrides select mode, continuation, page-size
hint, session token, and operation/planning options.

The reader retains the driver's `OperationPlan` and captures a continuation
candidate after each page. Failed or cancelled reads invalidate it; reopen
from confirmed progress rather than skipping uncertain work. Fresh plans keep
the driver's default 100-partition limit unless explicitly configured.
The existing `ProcessorEngine::read_page` remains a one-page convenience API.

```rust no_run
use std::num::NonZeroU32;
use azure_cosmos_change_feed_processor_engine::ProcessorEngine;

# async fn example(engine: ProcessorEngine) -> azure_data_cosmos_driver::Result<()> {
let page = engine.read_page(NonZeroU32::new(100).unwrap(), None).await?;
let _next_page = engine
    .read_page(NonZeroU32::new(100).unwrap(), Some(page.continuation()))
    .await?;
# Ok(())
# }
```

`ChangeFeedProcessorEngine` remains available as a compatibility name for
`ProcessorEngine`.

Prepared callers can use `ProcessorEngine::from_resolved`,
`CosmosLeaseStore::from_resolved_single_writer`, and
`BootstrapStore::from_resolved_single_writer` to retain an `Arc<CosmosDriver>`
and `ContainerReference` without name resolution or I/O. These constructors
reject account/credential mismatches; token-provider identity must match
explicitly, not just the endpoint. Subsequent CFP operations reuse the reference,
while driver-managed metadata refresh remains allowed. Source identities include
the container RID; replacement resources do not inherit workload checkpoints.

## Single-lease foundation

`lease_processing::run_lease` is one coordinator function, not a supervisor
hierarchy. `ProcessorEngine::run_owned_lease` opens a reader on the lease's
range and saved checkpoint, then delegates to it. The shared implementation
never requires a customer document type and does not depend on the typed SDK.

An `OwnedLease` carries confirmed owner/epoch/revision/range/checkpoint state.
`RawBatchSource` and `CheckpointStore` are small testable I/O seams; there are no
alternate storage backends in this slice. Checkpoint receipts must confirm the
exact candidate, matching ownership/range, and a new store revision.

Only one batch can be outstanding. A successful handler is not rerun while
definitely-not-persisted checkpoint writes retry. Ambiguous writes and rejected
conditions stop with recovery state. Idle pages bypass handlers but still
checkpoint their position; non-idle empty pages go through application handling.

Idle delay precedes another read, not completion of the requested batch limit.
A final confirmed idle page returns without sleeping.

**Batch completion requires both callback success and confirmed durable
checkpointing.** The next read cannot begin between those steps. Callback
success means the returned async operation has completed the actual work, not
merely scheduled another task. If a callback spawns work, it must await and
propagate that work's outcome before returning success.

Unwinding panics during callback construction or polling are surfaced as
`LeaseRunError` with the previous durable position, unconfirmed candidate, and
page diagnostics. Only the application callback boundary is caught; the lease
exits and that callback is not retried. This does not restore application state
mutated before a panic. Panic hooks still run, and `panic = "abort"` cannot be
caught. Dropping/aborting the whole run cannot return a result to that dropped
future; callers observe the cancellation through their task handle. In every
case unfinished callbacks do not issue a pending-batch checkpoint.

Renewal can continue independently but carries only confirmed progress.
Persistence failure blocks the next batch during safe retries. Uncertainty or
exhausted authority/budget stops with explicit recovery state for caller
reconciliation. A crash after application completion but before
checkpoint confirmation can redeliver that batch. Application side effects
and the lease checkpoint are not one atomic transaction.

Stop drains handling/persistence within policy and admits no new reads.
Ownership loss revokes local completion immediately. An interrupted write may
still commit remotely, so its candidate remains unconfirmed until store
reconciliation. Drain completion does not mean the lease was released.
The run report preserves the phase, durable lease, candidate, and diagnostics.
Handler/store futures must yield cooperatively for cancellation and deadlines.

The generic checkpoint seam remains useful for controlled tests. The
Cosmos-backed ownership path below now supplies real conditional acquisition,
renewal, checkpoint, and release operations. Bounded explicit-plan bootstrap is
described below; equal-count balancing is available for initialized workloads.
Durable lease topology transitions remain out of scope.

## One pre-created Cosmos lease

`CosmosLeaseStore::new_single_writer` addresses an existing item with an explicit
partition key. Calling this constructor acknowledges that the lease account
has a **single write region**. The current public driver does not expose a
production account-capability accessor to verify this automatically. Do not
use this store on a multi-writer account; per-region conditional writes and
conflict resolution do not establish the required global authority.

The version-1 pre-created item has this schema (the range uses the driver's
`FeedRange` serialization; the checkpoint must come from the corresponding
source/mode/range reader, not a fabricated string):

```text
id: lease item ID
<partition-key property>: supplied partition-key value
version: 1
ownership: { owner: null, generation: 0 }
range: serialized driver FeedRange
checkpoint: confirmed recoverable source continuation
lease_duration_ms: whole-millisecond lease interval shared by all workers
```

Application extension fields are retained on replacements. Service ETags come
from response headers; body fields cannot substitute for the revision fence.
Missing leases, malformed records, and policy mismatches fail explicitly.
There is no lease creation or upsert in the store.

`try_acquire(owner)` reads and attempts immediate acquisition of a released
record. For an occupied record, retain `observe()`'s process-local observation
and call `acquire(observation, owner)` only after the full stored interval.
The observation is bound to that store instance and must not be transferred to
another process. Takeover replaces the observed ETag and increments generation;
concurrent acquisition/renewal invalidates the observation. Read again on a
conflict rather than reuse a stale plan.

```rust no_run
use std::{num::NonZeroU32, sync::Arc, time::Duration};
use azure_cosmos_change_feed_processor_engine::{
    ChangeFeedReadOptions, CosmosLeaseStore, LeaseOwnershipOptions,
    LeaseRunOptions, ProcessorEngine,
};
use azure_data_cosmos_driver::{
    models::{ChangeFeedStartFrom, FeedRange, PartitionKey},
    CosmosDriver,
};

# async fn example(driver: Arc<CosmosDriver>, engine: ProcessorEngine) -> Result<(), Box<dyn std::error::Error>> {
let policy = LeaseOwnershipOptions::new(
    Duration::from_secs(30), Duration::from_secs(5),
    Duration::from_secs(5), Duration::from_secs(2),
)?;
let store = CosmosLeaseStore::new_single_writer(
    driver, "leases-db", "leases", PartitionKey::from("workload"),
    "pre-created-lease", policy,
).await?;
if let Some(session) = store.try_acquire("unique-process-incarnation").await? {
    let lease = session.lease().await;
    let result = engine.run_with_lease_session(
        &session,
        ChangeFeedReadOptions::new(lease.range().clone(), ChangeFeedStartFrom::Beginning),
        &LeaseRunOptions::new(NonZeroU32::new(10).unwrap()),
        |raw| async move {
            // Deliver complete payloads and await application completion here.
            let _body = raw.response().body();
            Ok(())
        },
    ).await;
    println!("processing: {:?}, release: {:?}", result.processing(), result.release());
}
# Ok(())
# }
```

### Ownership safety and local revision coordination

Workers do not compare wall-clock expiration timestamps. A contender observes
an occupied record unchanged for its stored interval, then uses `If-Match`.
A stale read cannot overwrite a newer record because its ETag fails. Local
authority expires before that interval by the configured safety margin,
measured conservatively from the start of a confirmed acquisition/renewal write.
This requires timer drift smaller than the margin and a monotonic clock that
accounts for pauses; if the runtime/platform cannot provide that assumption,
do not use this policy without an external fencing/clock contract.

Renewal waits after a confirmed write completes. Timing validation therefore
requires `2 * request_timeout + renew_interval < duration - safety_margin`:
both the preceding write's latency and the next renewal's latency consume
the authority window.

`LeaseSession` owns one authoritative lease record/revision. Its async mutex
serializes renewal, checkpoint, and release I/O only; it never spans the
application handler. This local mutex is **not** cross-process coordination:
the shared Cosmos item and ETag do that. Renewal can update the ETag while a
handler is blocked, and the completed handler checkpoints using that same
session's latest revision without overwriting ownership or extension fields.
One processing run is admitted per session.

The session keeps the persisted record and its ETag as canonical data.
`OwnedLease` values are derived snapshots, not a second independently updated
copy. Internal authority states distinguish active, write-in-flight/uncertain,
and revoked. Cancelling an in-flight renewal, checkpoint, release, or bootstrap
write immediately signals ownership loss to active processing. Its state remains
uncertain and cannot silently restore authority.

Checkpoint validation rejected before I/O remains `CheckpointError::Rejected`.
Sent-write failures are conservatively `CheckpointError::Ambiguous`, including
a final HTTP 412: an earlier driver attempt may have committed and lost its
response. Stop and reconcile the stored progress; do not infer non-persistence
from the final status alone.

Decision functions take explicit record, progress, revision, policy, and time
inputs. Acquisition eligibility, checkpoint/release proposals, authority checks,
and write confirmation perform no I/O or hidden clock reads. The async methods
sample time, apply conditional store operations, commit validated results, and
publish signals. Processing separately validates receipts and decides safe
retries; its progress enum couples each pending candidate to handling or
persistence. This is data-oriented organization, not a ban on state-owning
structs or methods.

`run_with_lease_session` polls renewal independently of handling. A separate
deadline watchdog does not wait on the revision mutex, so blocked lease I/O
cannot extend assumed authority. Writes must finish before the previous safety
deadline. A conflict, ambiguous result, cancellation during lease I/O, or
expired deadline invalidates local authority; no retry silently rebases onto a
replacement owner's revision. The same fences protect checkpoint and release.

Processing, maintenance failure, and release are separate results.
Successful draining/batch-limit completion attempts conditional release.
Lost authority skips release; uncertain release is not reported as successful.
Dropping a run revokes its session and leaves the remote record to expire;
there are no detached renewal tasks. Callers must inspect all result fields.
Ambiguous writes require rereading; they never authorize more processing.

### Validation scope

Ownership tests use two independent driver runtimes against a shared in-memory
service, not the fake checkpoint store. They exercise Cosmos point operations,
ETags, contention, expiry takeover, stale-owner fencing, concurrent checkpoint
and renewal, busy handlers, and delayed renewal fault injection. They do not
establish behavior of a real service deployment or multi-writer account.

The ignored `lease_ownership_live` test writes one dedicated, already released
pre-created test lease and uses two independent runtimes. It requires separate
authorization to run:

```sh
cargo test -p azure_cosmos_change_feed_processor_engine --test lease_ownership_live -- --ignored --nocapture
```

Set `AZURE_COSMOS_LEASE_ENDPOINT`, `AZURE_COSMOS_LEASE_KEY`,
`AZURE_COSMOS_LEASE_DATABASE`, `AZURE_COSMOS_LEASE_CONTAINER`,
`AZURE_COSMOS_LEASE_PK`, `AZURE_COSMOS_LEASE_ID`, and
`AZURE_COSMOS_LEASE_DURATION_MS` securely. The stored interval must match and
be 5–30 seconds for this test. It preserves the checkpoint and releases the
winner, but an interrupted test can leave the item owned until expiry.

## Bounded workload bootstrap

`BootstrapStore::ensure_initialized` now implements a bounded workload
initializer over the same Cosmos-backed authority. It requires an existing
single-writer container with the single partition-key path `/workload`.
The bootstrap item ID **and partition-key value** are derived from stable
source-account/container-RID identity and processor group, identically for every
worker. The bootstrap authority is workload-wide; processing authority remains
exclusive per work lease. After readiness, workers can acquire different leases.

`InitialLease` supplies explicit ranges and already-established recoverable
continuations. `BootstrapPlan` validates complete expected coverage, with at
most 32 assignments. `ProcessorEngine::bootstrap_plan` additionally checks
tokens with the source driver and derives stable source identity. The first
atomic create-if-absent persists the winning plan and its initial positions;
later workers adopt those saved positions rather than reseeding from new
tokens or a fresh `Now`. Configuration changes to source, group, mode, exact
start policy, expected coverage, or protocol fail explicitly.

For `Now`, seeds must already have concrete post-poll positions before
bootstrap. A snapshot taken before a Now poll may reevaluate now on resume and
is not sufficient. Bootstrap does not inspect or transform opaque tokens to
try to repair that condition. This explicit-plan slice does not automatically
discover source ranges or establish fresh start positions.

One initializer acquires the bootstrap lease and renews while working; others
wait with bounded jitter and an overall startup deadline. Expired ownership
uses the same ETag/generation takeover protocol. Each work-lease creation is a
transactional batch that conditionally replaces the initializer's bootstrap
record and creates one missing lease in the **same partition**. A stale
initializer therefore cannot create work even if its code has not stopped.
Retries never upsert or reset an existing lease.

After discovery proves complete expected logical coverage, a transaction reads
all covering leases with their ETags and conditionally publishes `Ready`.
Partial creation, discovery failure, invalid coverage, or a failed batch cannot
publish readiness. Cleanup preserves the original error and adds cleanup
failure context instead of converting it to successful initialization.

`Ready` is reverified on **every** `ensure_initialized` call. Missing completely
uncovered assignments can be recreated from original persisted seed positions;
existing parent/overlapping logical coverage retains its checkpoints. Partial
seed coverage that would require narrowing a checkpoint fails explicitly.
Unknown initialization generations or overlapping assignments are not ignored.
There is no continuous background verification task.

Use `BootstrapStore::work_lease_store(ready, id)` for processing acquisitions.
It verifies current readiness and committed generation in the same transaction
as the work-lease CAS. Old readiness snapshots do not authorize acquisition
during repair. The plain pre-created `CosmosLeaseStore` rejects bootstrapped
items rather than allowing a readiness bypass.

## Decentralized equal-count balancing

`plan_equal_lease_balance` is a decision function over logical lease snapshots,
worker identity, explicit expiry/eligibility, and an explicit tie-breaking input.
It performs no I/O and uses no hidden clocks. Membership is distinct non-expired
owners plus the current incarnation, including when it owns zero leases.
Other zero-lease workers are not globally discovered. Identities are
case-sensitive and must be unique process incarnations.

For eligible logical-lease count `L` and observed membership `W`, the upper
target is `ceil(L / W)`. A worker at or above target does not acquire more.
Below target, it prefers unowned/expired work. Otherwise it selects one lease
from a most-loaded owner only if donor and recipient counts differ by at least
two. Stable `4,3,3` is balanced: the two three-lease workers do not steal merely
to reach four. Counts do not represent CPU, RU consumption, or handler cost.
This is the initial policy, not exact Java parity or weighted balancing.

`LeaseBalancer::cycle` reads every inventory page for the already-initialized
workload and attempts at most one acquisition/transfer. Queries exclude
bootstrap/control records before document decoding. Version-1 work records
without `recordKind` are legacy `WorkLease` records; explicit other roles are
excluded. Only the committed initialization generation is eligible.
`leaseState` may be absent/`Active`, or ineligible `Pending`, `Transitioning`,
or `Retired`; unsupported lifecycle values fail explicitly.

A failed or partial page is never an empty successful inventory. The completed
inventory updates local ETag observation history; unchanged occupied revisions
must be observed for the full lease interval before they count as expired.
Fresh observers conservatively count occupied records as active. The inventory
is not a global atomic membership snapshot; exact acquisition/transfer CAS and
readiness guards resolve races. Each cycle rotates candidate order, and the
bounded runner uses jittered scheduling.

Inventory expiry does not revoke a local session with matching owner/generation:
earlier pages may contain revisions replaced by independent renewal while later
pages are read. The session watchdog enforces its current authority deadline;
ineligible records, owner/generation changes, or absent assignments still revoke it.

Active transfer uses `CosmosLeaseStore::transfer`: it validates the exact
observed revision/owner generation, preserves the checkpoint, and installs a
new generation. A conflict returns a cycle result and the next cycle rereads;
it never rebases a stale plan onto the winner's ETag. An uncertain write is an
error and grants no processing session. Later cycles read fresh inventory; an apparent
same-incarnation orphan remains untrusted until it can be safely reacquired
after an unchanged observation interval.

`run_cycles` invokes its callback only for confirmed acquisitions. The callback
registers/starts work and returns promptly; it is not the application handler.
Processing uses the engine's independent renewal/watchdog, and the caller owns
task handles and checks final processing/release outcomes. Inventory ownership
changes revoke known local sessions; stop/deadline requests their drain.
Stopping the runner is not proof that those tasks have joined or released.
Cancellation during an incomplete handoff can leave a lease to expire.

```rust no_run
use std::{num::NonZeroU32, sync::Arc};
use azure_cosmos_change_feed_processor_engine::{
    BalanceRunOptions, ChangeFeedReadOptions, LeaseBalancer, LeaseControl,
    LeaseRunOptions, ProcessorEngine,
};
use azure_data_cosmos_driver::models::ChangeFeedStartFrom;

# async fn example(mut balancer: LeaseBalancer, engine: Arc<ProcessorEngine>)
# -> Result<(), Box<dyn std::error::Error>> {
let mut tasks = Vec::new();
let report = balancer.run_cycles(
    &BalanceRunOptions::new(NonZeroU32::new(3).unwrap()),
    &LeaseControl::default(),
    |session| {
        let engine = engine.clone();
        tasks.push(tokio::spawn(async move {
            let lease = session.lease().await;
            engine.run_with_lease_session(
                &session,
                ChangeFeedReadOptions::new(lease.range().clone(), ChangeFeedStartFrom::Beginning),
                &LeaseRunOptions::new(NonZeroU32::new(1).unwrap()),
                |batch| async move {
                    println!("received raw page: {:?}", batch.response().status());
                    Ok(())
                },
            ).await
        }));
        async { Ok(()) }
    },
).await?;
println!("confirmed handoffs: {}", report.acquisitions());
for task in tasks {
    let result = task.await?;
    println!("processing: {:?}, release: {:?}", result.processing(), result.release());
}
# Ok(())
# }
```

This slice retains the existing workload inventory bound of 32 records and
single-write-region store/timer assumptions. It does not add capacity weights,
worker heartbeat registration, physical-range subdivision, a permanent
dispatcher, or native bindings. Fencing rejects obsolete checkpoint/release
operations, but cannot undo application side effects already running.

## Bootstrap protocol limits and usage

The bootstrap protocol uses initialization generation 1, retained across initializer
ownership takeovers; the initializer's ownership generation is a separate
counter. Later whole-workload replacement generations, migration of untagged
legacy leases, cross-partition staging/commit schemes, automatic provisioning,
and source-range discovery are not implemented. Existing covering leases are
never retired, deleted, or combined by this bootstrap path.

```rust no_run
use std::{sync::Arc, time::Duration};
use azure_cosmos_change_feed_processor_engine::{
    BootstrapStartPolicy, BootstrapStore, ChangeFeedMode, InitialLease,
    LeaseOwnershipOptions, ProcessorEngine,
};
use azure_data_cosmos_driver::{models::FeedRange, CosmosDriver};

# async fn example(source: ProcessorEngine, lease_driver: Arc<CosmosDriver>, seeds: Vec<InitialLease>)
# -> Result<(), Box<dyn std::error::Error>> {
let plan = source.bootstrap_plan(
    "processor-group", ChangeFeedMode::LatestVersion,
    BootstrapStartPolicy::Beginning, FeedRange::full(), seeds,
).await?;
let policy = LeaseOwnershipOptions::new(
    Duration::from_secs(30), Duration::from_secs(5),
    Duration::from_secs(5), Duration::from_secs(2),
)?;
let bootstrap = BootstrapStore::new_single_writer(
    lease_driver, "leases-db", "workloads", plan, policy,
).await?;
let ready = bootstrap.ensure_initialized(
    "unique-process-incarnation", Duration::from_secs(60),
).await?;
for id in ready.lease_ids() {
    let store = bootstrap.work_lease_store(&ready, id)?;
    // Compete for work through this readiness-gated store.
    if let Some(session) = store.try_acquire("unique-process-incarnation").await? {
        // Hand the session to processing; this example releases it immediately.
        session.release().await?;
    }
}
# Ok(())
# }
```

## Responsibility boundaries

The engine starts with six responsibility areas, not six interfaces, factories,
and implementations. Add state-owning structs only where state needs an owner.

| Module | Responsibility | Shape and current status |
| --- | --- | --- |
| `processor_engine` | Own processor state and coordinate lifecycle, lease tasks, delivery, acknowledgments, and failures. | Connection, retained reader creation, and bounded single-lease entry point implemented. |
| `change_feed_bootstrapper` | Initialize groups and leases safely; reconcile recoverable range coverage after interruptions or topology changes. | Bounded explicit-plan `bootstrap::ensure_initialized` implemented; no permanent task or child-token conversion. |
| `change_feed_poller` | Read feeds and deliver bounded batches without treating a fetch as successful processing. | Retained raw reader implemented; single-lease ordering lives in `lease_processing`. |
| `lease_checkpointer` | Persist only acknowledged progress through ownership-conditional writes. | Receipt validation/retry policy in `lease_processing`; Cosmos session writes preserve authoritative revisions. |
| `lease_load_balancer` | Calculate acquisition and transfer decisions; the store grants authority. | Pure equal-count policy and bounded paginated/jittered coordination implemented; no central dispatcher. |
| `lease_renewer` | Renew ownership independently of handlers and detect ownership loss or an exceeded safety deadline. | Session renewal and independent watchdog implemented in `cosmos_lease_store`. |

Only implemented responsibilities have source files today. The others will
be added with their behavior, not as empty abstractions. The public CFP crate
owns the fluent builder; configuration is data plus validation, not a
configuration object. Routing, transport retries, and feed topology mechanics
remain driver responsibilities.

## Durable lease topology

`lease_topology_handler::plan_lease_topology` compares durable logical assignments
with refreshed physical ranges. `ProcessorEngine::reconcile_lease_topology`
refreshes through the driver and invokes the same planner. Single-lease startup
calls this before opening the reader. A future runtime coordinator/bootstrapper
can reuse it at startup and on a bounded periodic cadence; no periodic task or
driver topology notification is installed in this slice.

Do not rely on a 410 reaching CFP: the driver's `UnorderedMerge` repairs split
leaves internally. Physical topology and processing ownership are separate.
The current planner retains every original lease/range/checkpoint/revision.
For splits, the driver reads the new physical partitions under the old logical
lease. This preserves recovery but does not increase lease-level parallelism.
For merges, two logical ranges keep their distinct checkpoints even if they
route to one physical partition. The planner rejects overlapping assignments,
duplicate IDs, invalid physical maps, and uncovered lease ranges.

Lease subdivision is deferred. The current public driver API has no dedicated
operation to derive guaranteed child-scope checkpoints or combine independently
advanced parent positions. Tokens must not be decoded, edited, ranked, or
blindly copied into a different scope by CFP.

Future durable handoffs require an apply/resume function and a storage contract:

1. Quiesce parent delivery and use only confirmed durable progress.
2. Conditionally mark the parent `Transitioning` using ownership epoch and revision.
3. Create `Pending` replacements idempotently without resetting existing progress.
4. Verify all replacements are durable and cover the parent without gaps.
5. Commit a durable marker that retires parent authority and makes replacements eligible.
6. Retain retired records and the marker until recovery and activation no longer depend on them.

If records cannot be changed atomically, the committed marker determines child
eligibility. Load balancing must honor it, rather than acquiring newly created
records immediately. Do not collapse merged leases without supported composite
progress and coordinated parent retirement.

No such transition storage or child activation is implemented here; the fake
checkpoint store is not a durable handoff store. The safe retention plan avoids
deleting recovery evidence while those capabilities remain unavailable.

The running engine will coordinate a per-lease async function:

```text
run_lease:
    read -> deliver -> await acknowledgment -> checkpoint
    renew ownership independently of handler progress
    on ownership loss -> revoke delivery and stop the lease
```

There is no separate supervisor class. Each lease has one authoritative local
ownership state and store revision. Renewal and checkpoint writes coordinate
that revision and preserve each other's fields. Store-side conditional writes
enforce ownership across processes.

Acknowledgments must identify the current ownership epoch and delivered batch.
Handler failure or a late acknowledgment cannot advance a checkpoint past
unfinished work. Ownership loss or an exceeded safety deadline stops admission
and rejects obsolete completions. Shutdown drains within policy, attempts
release while ownership is valid, and observes task failures. Interrupted
bootstrap and topology reconciliation must preserve recoverable range coverage
and progress.

Weighted capacity, worker registry, automatic discovery, migration, and durable topology handoffs remain requirements for
the later full processor. If alternate stores are required,
introduce one small `LeaseStore` trait rather than a family of providers.

See the [Cosmos SDK project documentation](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/cosmos/docs)
for project and architecture context.

# Rust Change Feed Processor design

## Problem

We are building a Rust-based Cosmos DB Change Feed Processor that also provides
a shared engine for language shims to build their own Change Feed Processor
abstractions.

A **language shim** adapts the shared engine to a language's SDK types and
execution model.

The [public Change Feed Processor documentation](https://learn.microsoft.com/en-us/azure/cosmos-db/change-feed-processor)
describes its purpose:

> "simplify the process of reading the change feed and distribute the event processing across multiple consumers effectively."

## Functional requirements

- Deliver changes from each physical partition with **at-least-once delivery
  semantics**.
- Monitor **all physical partitions within the configured scope**, including
  replacement partitions after topology changes.

## Non-functional requirements

- Distribute processing load equally across cooperating workers.

## How would SDKs process changes using the shared Rust engine?

The **SDK** is the application-facing library; the **driver** is the shared
engine beneath it. A **workload** is the cooperating processors reading one
source container under a shared workload name.

The **application handler** is the callback supplied by the application to
process a batch of changes. An **internal processing pipeline** is the SDK's
sequence of steps that reads changes, invokes that handler, and records progress.

Here, **progress** means how far the application has successfully processed
the change feed. A **checkpoint** saves a resume position durably; the
**checkpoint policy** determines when that save is required.

A **worker** is one running Change Feed Processor instance. Multiple workers
can cooperate on the same workload; a worker is not necessarily a separate
process, machine, or thread.

A **partition range** is a portion of the source container's partition key
space. Workers divide processing by these ranges; the progress and ownership
records are described in the next sections.

- Existing .NET and Java CFP implementations pass the application handler into
  their internal processing pipelines and invoke it within the same language
  runtime.
- A shared Rust driver cannot directly invoke a Java, Go, or other language's
  callback object using that language's types, exception handling, and async
  execution model.
- Supporting foreign callbacks inside the driver would require an explicit
  foreign-function interface (FFI) adapter for callback lifetime, runtime dispatch, and completion. Callbacks
  can cross FFI, but the Java model cannot be reused unchanged.
- Instead, each SDK or shim invokes its application handler and acknowledges
  successful processing to the shared driver.

### Existing callback implementations

The following excerpts show direct handler invocation, not complete methods.

**.NET:** [`ChangeFeedObserverFactoryCore<T>`](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/ab37d0a240f0c2f858d7c2f4bcdb71fa80195361/Microsoft.Azure.Cosmos/src/ChangeFeedProcessor/Observers/ChangeFeedObserverFactoryCore.cs)
invokes the supplied delegate and returns its task:

```csharp
if (this.onChanges != null)
{
    return this.onChanges(context, changes, cancellationToken);
}
```

**Java:** [`DefaultObserver<T>`](https://github.com/Azure/azure-sdk-for-java/blob/0f3b0113dab92249963ee1bb3b1b62433143f80c/sdk/cosmos/azure-cosmos/src/main/java/com/azure/cosmos/implementation/changefeed/common/DefaultObserver.java#L43)
invokes the supplied consumer within the observer:

```java
if (consumer != null) {
    consumer.accept(docs);
} else if (biConsumer != null) {
    biConsumer.accept(docs, new ChangeFeedProcessorContextImpl<>(context));
}
```

These handlers execute inside their SDKs' CFP pipelines, not a separate
cross-language driver.

### Shared Rust engine

The SDK owns application callback execution. The shared driver engine owns
lease coordination, feed reads, and checkpoint persistence.

1. **SDK requests work:** request a batch of changes.
2. **Driver reads changes:** select an eligible partition range owned by the
   worker and read its change feed, or wait until a range is eligible.
3. **Driver returns a batch:** deliver changes to the SDK.
4. **SDK processes changes:** deserialize the items, invoke the application
   handler, and wait for it to complete.
5. **SDK acknowledges success:** acknowledge only successfully processed changes.
6. **Driver records progress:** record safe progress and persist it according
   to the checkpoint policy.
7. **SDK requests another batch:** repeat after handling the current batch.

```mermaid
flowchart LR
    subgraph SDK["SDK / language shim boundary"]
        Request["1. Request a batch"]
        Handle["4. Deserialize changes<br/>and await application handler"]
        Ack["5. Acknowledge successful processing"]
        Next["7. Request another batch"]
        Handle -->|Success| Ack
        Next --> Request
    end

    subgraph Driver["Shared driver boundary"]
        Read["2. Select an eligible owned range<br/>and read its change feed"]
        Return["3. Return changes to the SDK"]
        Progress["6. Record safe progress<br/>and apply persistence policy"]
        Read --> Return
    end

    Request -->|Batch request| Read
    Return -->|Batch of changes| Handle
    Ack -->|Processing acknowledgment| Progress
    Progress -->|Acknowledgment result| Next
```

Requests, batches, and acknowledgments cross the SDK/driver boundary.
The application handler does not: it executes inside the SDK or language shim.
The driver performs source reads and persists checkpoint progress in Cosmos DB.

**Parallelism comes from multiple workers running this flow concurrently**,
with the driver coordinating which partition ranges each worker owns. Within
each SDK worker, batch handling remains sequential.

The Rust SDK implements this boundary in
`azure_data_cosmos/src/change_feed_processor/execution.rs`. These are focused
implementation excerpts, not standalone examples.

In these snippets, `T` is the application's document type and `Fut` is its
handler's returned **future**, an object representing asynchronous work.
`.await` waits for that work without blocking an executor thread while it is
pending.

**Request and process a batch:** the SDK waits for one batch from the driver,
then waits for SDK-side processing before requesting another. `read_next_change_feed_batch()`
is the driver's receive method; `dispatch()` handles one returned batch:

```rust
while let Some(batch) = driver.read_next_change_feed_batch().await {
    dispatch::<T, Fut>(&container, handler, batch).await;
}
```

`consume()` owns the container client and awaits each dispatch, so `dispatch()`
borrows that client as `&ContainerClient` instead of cloning a handle per batch.
Dispatch is not separately spawned; the borrow remains valid until it returns.

**Invoke the handler in the SDK:** deserialization converts returned data to
application types. The SDK passes those items and batch context (lease identity
and read diagnostics) to the handler and awaits its completion:

```rust
let items = container.deserialize_processor_batch::<T>(response)?;
if !items.is_empty() {
    handler(
        ChangeFeedProcessorContext {
            lease_token: lease_token.clone(),
            diagnostics,
        },
        items,
    )
    .await?;
}
```

**Acknowledge successful processing:** the batch includes a **receipt**, a
single-use acknowledgment handle, which
lets the SDK report success back to the driver without supplying a callback
or an arbitrary progress position:

```rust
Some(Ok(Ok(()))) => {
    if let Err(err) = receipt.acknowledge().await {
        tracing::warn!(lease = %lease_token, error = %err, "Callback succeeded but acknowledgment failed");
    }
}
```

This match arm runs only after deserialization and handler execution succeed
without an unwinding panic. The persistence sequence is explained in
[delivery and checkpointing](#how-would-batch-requests-delivery-and-checkpointing-stay-coordinated);
failure outcomes are collected in [failure handling](#how-would-failures-be-handled-and-exposed-to-the-application).

## How is progress tracked in the Change Feed Processor?

**Progress is the position in each partition range's change feed up to which
changes have been successfully processed.**

The processor must:

- Distinguish the **read position** (changes fetched) from the **processed
  position** (changes successfully handled by the application).
- Persist a **saved position** so a restarting or replacement worker knows
  where to resume.
- Never advance the saved position past successfully processed changes.
- Allow the saved position to lag behind processed changes; interruption can
  therefore cause those changes to replay.

A **lease document** stores the range, current worker owner, and saved position
in a separate Cosmos DB **lease container**. This shared record lets workers
coordinate responsibility and resume after restart or ownership change rather
than rely only on a worker's memory.

## How would a workload be initialized before processing?

### Startup initialization requirements

**Bootstrap prepares a workload so workers can begin processing safely.**
It must:

- Identify the source container and workload.
- Establish complete initial partition-range coverage to process.
- Ensure coordination and progress records exist for that coverage.
- Preserve existing ownership and saved progress on restart.
- Allow interrupted initialization to be completed by a later attempt.
- Report initialization failure rather than declare readiness prematurely.

### Current implementation

During startup, the driver initializes the workload:
it discovers the source's partition ranges and ensures a lease document exists
for each range in the initial plan before starting worker coordination.

New leases have no owner or saved position. Workers acquire them, read changes
using the configured starting position, and persist successfully processed
progress. Existing lease progress is preserved when the workload resumes.

### How would workers coordinate bootstrap?

#### Identify the workload

Within the supplied lease container, the workload namespace combines the
source account origin, source container RID, and workload name. A durable
`.definition` record validates the feed mode and schema. Worker names do not
split the workload into separate bootstrap operations.

The **container RID** is its service-assigned resource identifier, rather
than its application-facing name.

`namespace()` encodes the shared workload key; it excludes worker identity:

```rust
let key = serde_json::to_vec(&(
    &identity.account,
    &identity.container_rid,
    &identity.workload_name,
))?;
Ok(format!("cfp1.{}", URL_SAFE_NO_PAD.encode(key)))
```

These Rust snippets are focused excerpts from
`azure_data_cosmos_driver/src/change_feed_processor/bootstrap.rs`, not
standalone programs.

#### Elect a designated initializer and check completion

An **ETag** is a document-version token supplied by Cosmos DB. A conditional
write or delete can require that version still match, preventing it from
overwriting a change made after the read. An initialization **claim** is the
winning attempt's local lock version and time limit.

Each starter first checks the workload's `.info` completion marker. If it is
compatible, bootstrap is complete and no election is necessary.

Otherwise, workers compete to create `.lock`. The winner reads the lock back
and verifies its unique attempt ID before initializing. Other workers wait,
check completion, and can retry election if the lock expires.

`try_claim()` constructs and attempts to create the lock:

```rust
let started = Instant::now();
let mut record = self.record("lock");
record.ttl = LOCK_TTL_SECONDS;
let lock = InitializationLock {
    record,
    attempt: Uuid::new_v4().to_string(),
    owner: self.worker_name.clone(),
};
self.store.create(&lock.record.id, &lock).await?;
```

Creation success alone does not grant a local claim. After reading the lock
back, the initializer checks its identity and attempt ID:

```rust
self.validate(&current.0.record, &lock.record)?;
if current.0.attempt != lock.attempt {
    return Ok(None);
}
Ok(Some(InitializationClaim {
    etag: current.1,
    deadline: (started + ATTEMPT_TIMEOUT).min(self.store.deadline),
}))
```

The intervening read returns `None` if no lock is visible. The ETag retained
in the claim is used for subsequent claim checks and conditional release.

The following is an illustrative lock document. Placeholder strings stand
for the actual namespace, account origin, container RID, and attempt UUID:

```json
{
  "id": "<workload-namespace>.lock",
  "workload": {
    "version": 1,
    "account": "<source-account-origin>",
    "containerRid": "<source-container-rid>",
    "processorName": "orders",
    "mode": "latestVersion"
  },
  "ttl": 30,
  "attempt": "<attempt-uuid>",
  "owner": "worker-01"
}
```

`processorName` is the existing stored field for the workload name. The lock
identifies the initializer; processing leases separately track ownership of
source ranges.

**The intent is one active initializer per workload. TTL election is not
strict mutual exclusion:** delayed operations from an expired initializer
can overlap a replacement.

`run_attempt()` checks completion before competing for that claim:

```rust
self.validate_container().await?;
self.ensure_record(&self.record("definition")).await?;
if self.ready().await? {
    return Ok(true);
}
let Some(claim) = self.try_claim().await? else {
    return Ok(false);
};
```

```mermaid
flowchart TD
    Start["SDK worker starts"] --> Definition["Ensure compatible workload definition"]
    Definition --> Ready{"Compatible completion marker exists?"}
    Ready -->|Yes| Done["Bootstrap complete; start worker coordination"]
    Ready -->|No| Claim["Attempt to create expiring initialization lock"]
    Claim --> Won{"Read back lock: this attempt won?"}
    Won -->|No| Wait["Wait within caller budget"]
    Wait --> Ready
    Won -->|Yes| Recheck{"Completion marker now exists?"}
    Recheck -->|Yes| Release["Conditionally release initialization lock"]
    Recheck -->|No| Plan["Persist or reuse initial partition plan"]
    Plan --> Leases["Ensure planned leases using create-only writes"]
    Leases --> ClaimCheck["Verify initialization claim"]
    ClaimCheck --> Publish["Publish completion marker"]
    Publish --> Release
    Release --> Done
    Plan -.->|Failure or interruption| Partial["Preserve completed writes; attempt lock release"]
    Leases -.->|Failure or interruption| Partial
    Partial --> Retry["Later attempt reuses plan and existing leases"]
    Retry --> Ready
```

The diagram summarizes coordination, not a transaction. Claim expiry,
service failures, or the caller deadline can end an attempt.

#### Publish readiness only after initialization

The current ordering in `bootstrap.rs` is:

```rust
self.create_leases(&plan, claim).await?;
self.check_claim(claim).await?;
self.ensure_record(&self.record("info")).await
```

An unresolved failure prevents new readiness publication. Existing
lease-create conflicts are accepted without reading or validating the
conflicting lease's contents; this assumes a managed namespace and compatible
deterministic IDs. Later starts trust `.info` rather than automatically
auditing every lease.

Inside `create_leases()`, each range gets a deterministic ID based on its
boundaries:

```rust
record: self.record(&format!(
    "lease.{}-{}",
    range.min_inclusive(),
    range.max_exclusive()
)),
```

After preparing the unowned lease document, creation is guarded by the
initialization claim:

```rust
self.check_claim(claim).await?;
if !self.store.create(&expected.record.id, &expected).await? {
    tracing::debug!(lease = %expected.record.id, "Processing lease already exists; preserving it");
}
```

The storage helper returns `false` for a create conflict; other failures
propagate through `?`. No existing lease is overwritten.

#### Allocate time for large workloads

**Requirement: the overall bootstrap time budget should scale with the number
of lease documents that need to be created**, rather than require customers
to predict a fixed total duration.

**This proportional budget is not implemented today:** the current attempt
and lock limits remain fixed, as shown below.

| Current limit | Purpose |
| --- | --- |
| Caller budget: configurable, default 120 seconds | Total wait across attempts and retry delays. |
| Claimed attempt: at most 25 seconds | Bounds one initializer's work, also subject to the caller deadline. |
| Lock TTL: 30 seconds, without renewal | Allows takeover after an initializer disappears. |

The fixed limits are defined explicitly:

```rust
const LOCK_TTL_SECONDS: i32 = 30;
const ATTEMPT_TIMEOUT: Duration = Duration::from_secs(25);
pub(super) const BOOTSTRAP_TIMEOUT: Duration = Duration::from_secs(120);
```

The claim-deadline calculation shown above uses the earlier of the attempt
deadline and caller deadline. There is no lease-count multiplier in it.

Increasing the caller budget allows more attempts, not a longer claim.
Attempts reuse the persisted plan but traverse it again, checking the claim and
attempting creation even for existing leases. A large plan can repeatedly
exhaust the attempt window on the same prefix without completing.

**Open design:** resumable chunks and/or renewable initialization ownership
could permit continued work while measurable progress is made. Individual
operations still need bounds, and progress, stalls, throttling, and takeover
rules need explicit definitions. No proportional-duration formula or scalable
completion guarantee is currently implemented.

#### Relationship to the partition key range cache

The **partition key range cache** is the driver's shared routing inventory of
the source container's physical partition ranges. Bootstrap uses it to build
the initial plan; it does not maintain a separate partition inventory.

When no persisted plan exists, `plan()` requests a refresh:

```rust
self.source_driver
    .resolve_all_partition_key_ranges_with_options(self.source, true, operation_options())
    .await?
```

The cache fetches partition metadata through the `/pkranges` feed and builds
the routing map. Bootstrap converts the physical ranges into feed ranges,
checks coverage, and persists that snapshot as `.plan`.

After obtaining `topology`, the conversion and create-only persistence are:

```rust
let ranges = topology
    .iter()
    .map(FeedRange::try_from)
    .collect::<Result<Vec<_>>>()?;
validate_ranges(&ranges)?;
let plan = RangePlan { record, ranges };
self.check_claim(claim).await?;
if self.store.create(&plan.record.id, &plan).await? {
    return Ok(Some(plan));
}
```

If creation conflicts, the following code reads and validates the persisted
plan rather than replacing it.

- The refresh uses the same cache as other source-container operations, so
  bootstrap metadata traffic and cache updates affect shared routing state.
- A later initialization attempt reuses `.plan`; it does not refresh the
  inventory merely because it is a new attempt.
- A compatible `.info` marker skips bootstrap inventory discovery entirely.
- **A requested refresh is not a strict freshness guarantee:** the cache can
  return an older map after a fetch failure. Full keyspace coverage does not
  prove that every current physical partition is represented individually.

Consequently, bootstrap readiness describes the persisted initial plan, not
an atomic inventory of all physical partitions at the instant readiness is
published. Subsequent source topology changes are handled by processing
recovery.

#### Recover when the initializer fails

The persisted plan and successfully created leases survive the designated
initializer. A later contender can become initializer and reuse that state,
preserving existing lease ownership and saved progress.

This supports recovery from partial initialization, but it is not a saved
cursor: the replacement traverses the plan again. Inventory freshness has the
[cache limitation](#relationship-to-the-partition-key-range-cache) described above.

`initialize_and_release()` attempts lock release after the bounded
initialization result, including initialization failure:

```rust
let result = catch_bootstrap_panic(bounded(initialization, claim.deadline)).await;
let release = catch_bootstrap_panic(Box::pin(
    self.store
        .release(&self.record("lock").id, claim.etag.clone()),
))
.await;
```

The following result handling preserves the original initialization error
and logs a cleanup error if release also fails. If initialization succeeded,
a release error is returned instead.

#### Why the initialization lock needs TTL

A crashed initializer cannot release its lock. Document TTL prevents an
abandoned lock from blocking initialization indefinitely and allows the next
potential initializer to compete.

**TTL supplies eventual takeover, not proof that the old initializer stopped.**
Long-running bootstrap needs renewal and safe claim validation, rather than
simply extending an attempt beyond the lock TTL. Persistent partial recovery
is implemented; progress-aware duration and stronger initialization fencing
remain open.

## How would leases be allocated across workers?

### Worker lifecycle

1. **Construct:** the application supplies the source and lease containers,
   workload name, worker name, handler, and processing options.
2. **Start:** initialize or resume the workload, then launch driver
   coordination and the SDK's batch-consumption loop. Startup does not wait
   for the worker to acquire a lease or deliver a batch.
3. **Acquire:** obtain ownership of ranges to process. A running worker can
   own zero leases and wait for work.
4. **Process:** read changes and keep ownership alive. The SDK handles one
   batch at a time; the driver records successful progress.
5. **Lose or change ownership:** stop processing the affected range and clean
   up without changing another worker's ownership. Continue coordinating
   other ranges; source topology changes can require replacement leases.
6. **Stop gracefully:** drain issued work and release ownership.
7. **Cancel or crash:** end processing without a confirmed graceful drain.

The detailed completion and restart rules appear in
[shutdown and restart](#how-would-a-worker-stop-and-resume-processing).

### Recording ownership

To **acquire** a lease, a worker conditionally writes its own name as owner in
the lease document. Only after that write succeeds does it start processing
the range. Cooperating workers share the workload name and use distinct worker
names; the SDK constructor inputs are listed in the [public API](#how-would-an-application-configure-and-control-the-rust-processor).

The following are focused implementation excerpts, not standalone programs.

### Preventing stale or competing ownership writes

A lease replacement uses the ETag condition introduced during bootstrap.
`If-Match` is the request condition requiring the stored ETag to match the
version just read.

An **ownership turn** is one acquisition of a lease. The driver records a new
UUID, stored as `incarnation`, for each successful acquisition. This distinguishes
old work from new work even if the same named worker loses and reacquires the
lease.

Before renewal, checkpoint, or release, `transition_lease()` compares fresh
stored ownership with the acquisition's expected ownership:

```rust
if current.owner != observed.owner || current.incarnation != observed.incarnation {
    return Err(lost());
}
```

`modify_lease()` then writes with the ETag obtained from that fresh read:

```rust
self.store().replace(id, &next, etag).await
```

Source: `azure_data_cosmos_driver/src/change_feed_processor/processing.rs`.
Owner/incarnation checks reject stale ownership turns; the ETag prevents
changes racing the write. Retried write conflicts recheck ownership. These
checks are called **fencing**: an old ownership turn cannot mutate replacement
ownership or saved progress.

### Selecting leases across workers

Workers periodically read the workload's lease documents, count leases held
by healthy owners, and calculate their target share. They prefer available
leases: unowned, expired, or eligible for processing-inactivity takeover.
If none are available, a worker below its target can select a donor lease
according to the balancing policy.

An **expired lease** has not been renewed within the expiration budget.
**Processing-inactivity takeover** permits another worker to select a lease
whose processing deadline has passed, even if renewal continues.

Driver `balance()` reads current leases and selects acquisition candidates:

```rust
let active = context.list().await?;
let mut acquired = Vec::new();
let selected = balancing::select(&active, &context.options, now_millis());
```

For each selected lease, it attempts acquisition:

```rust
match context.acquire_tracked(&active[index], Some(&state)).await {
    Ok(lease) => acquired.push(lease),
    Err(err) if err.status().is_precondition_failed() => {
        tracing::debug!(error = %err, "Lease acquisition lost a race");
    }
    Err(err) => state.failed(Some(&active[index].record.id), err),
}
```

Source: `processing.rs`; selection policy:
`processing/balancing.rs`, `select()`.
The first balancing pass has no initial delay; later passes use the acquisition
interval. An empty result does not stop the worker.

### Maintaining ownership while processing

**Renewal** is a periodic conditional lease update that keeps the owner's claim
alive. It does not advance the saved change feed position. Successful checkpoint
writes also refresh lease timing.

The **supervisor** coordinates one acquired lease's processing and renewal.
`LeaseState` holds its local document copy, ETag, renewal timing, and pending
checkpoint progress. A **mutex** permits only one mutation of that local state
at a time.
It polls both asynchronous operations concurrently, so renewal can continue
while processing waits for a source read or application acknowledgment.
Renewal and checkpoint writes use the same lease-state mutex and are serialized.

`.fuse()` supports use in
`select!`, which waits for either operation to finish; `pin_mut!` pins those
futures for polling. These are not separately spawned tasks.

Driver `supervise()` prepares its processing and renewal futures for concurrent
polling:

```rust
let renewing = renew(context.clone(), lease.clone(), cancelled).fuse();
futures::pin_mut!(processing, renewing);
```

Source: `processing.rs`, `supervise()` and `renew()`.

### Detecting ownership loss and cleaning up

Ownership loss is not immediately pushed to the old worker. Its next fenced
lease operation discovers a different owner or ownership-turn identifier.

The supervisor then signals cancellation to the processing operation and waits
for it to finish before release cleanup. Release uses the same ownership checks,
so it cannot clear the replacement owner's claim. The coordinator continues balancing. The local delivery session and its queue
cleanup are explained in the next section.

In the renewal-completion branch, `cancel` is the local cancellation sender;
dropping it wakes cancellation waiters. `processing.await` joins the issued
work. `PollError::Other` preserves the fact that renewal errors are not
source-feed topology signals:

```rust
result = renewing => {
    // Renewal ended: cancel processing and join its issued work before releasing the lease.
    drop(cancel.take());
    let processed = processing.await;
    result.map_err(PollError::Other).and(processed)
},
```

Only an explicit source-feed topology failure can initiate source-range recovery.
Cancellation cannot undo application effects already performed.

### How start and stop affect rebalancing

Starting launches a new worker's balancing loop. Stopping prevents that worker
from acquiring more leases and eventually releases its existing ownership.
Other workers periodically inspect lease documents to discover those changes.
There is no fleet-wide join/leave notification or atomic redistribution.

With the default 60-second lease expiration budget, acquisition checks run
every one sixth of that budget: 10 seconds.

Released leases become available on subsequent balancing passes, not by direct
assignment from `stop()`. Forced cancellation attempts cleanup; after a crash,
workers must wait until eligibility rules permit takeover. The shutdown API
and cleanup sequence are shown once in
[shutdown and restart](#how-would-a-worker-stop-and-resume-processing).

## How would batch requests, delivery, and checkpointing stay coordinated?

The SDK requests one batch using the driver's
`read_next_change_feed_batch()` method. Each acquired partition range registers
when its feed poller can accept another read turn. These registrations form a
waiting list; the driver selects one when an SDK request is pending.

**The waiting list contains no fetched changes.** This request-matching
component is named `ChangeFeedRequestDistributor`: it distributes read
requests, not lease ownership. Below, "distributor" refers to that component
and "eligibility queue" to its waiting list.

Each acquisition has a separate **delivery session**, its local state and
cleanup scope. Its delivery state controls whether it may register, read, or
deliver again.

### Register readiness and grant one read turn

The SDK's `consume()` loop shown in the first section waits on `read_next_change_feed_batch()`.
Meanwhile, the acquired lease's **feed poller**, the driver operation that
reads changes for that range, calls `wait_for_demand()`. That
registers its delivery state in the queue. Registration and the SDK request can
arrive in either order; **both are required before a read turn is granted**.

The distributor selects a waiting registration, removes it from the queue, and
sends read permission with a one-shot reply channel. A **one-shot channel**
carries one value to its waiting receiver; it does not invoke the application
handler. The poller uses the reply sender to return the batch.

A one-shot channel fits because each read permission expects **one batch
response**, and each acknowledgment expects **one completion result**. It ties
the reply directly to the waiting request without a request-ID lookup,
transfers ownership of the value, and wakes the receiver without repeated
polling. If the sender is dropped without replying, the receiver observes
cancellation.

Each turn creates a new channel pair. **The channel provides in-memory
coordination, not durable delivery or a timeout.**

### Track the acquisition's delivery state

| State | Meaning | Next action |
| --- | --- | --- |
| `Idle` | No outstanding delivery or demand registration. | Register readiness. |
| `WaitingForDemand` | Registered in the eligibility queue. | Distributor grants a read turn. |
| `Fetching` | A read turn has been granted. | Read and deliver, or yield after an idle/throttled turn. |
| `AwaitingAcknowledgment` | A batch has been handed to the receiver. | Wait for successful processing and acknowledgment completion. |
| `Closed` | This acquisition's delivery session has ended. | Remove or skip its queue entries. |

These are delivery states, not persisted lease lifecycle states. `Fetching`
also remains in use during an idle delay or throttle cooldown, until the
poller registers again. `AwaitingAcknowledgment` covers SDK deserialization,
handler execution, and any required checkpoint write; it does not distinguish
those activities diagnostically.

### How source-read outcomes change delivery state

**Each SDK `read_next_change_feed_batch()` call returns at most one batch.**
For an authorized range turn, the poller executes the read plan once and
handles that response; it does not drain multiple pages from that partition
range into a single delivery. Another data page requires successful
acknowledgment and a new read turn.

This is a page/delivery count, not an item-count limit. A returned batch can
contain multiple changes; `with_max_item_count()` is a service item-count hint.
One pending SDK call can authorize several sequential idle or throttled turns
while seeking a batch, and request-level retries can issue additional service
requests.

```mermaid
flowchart TD
    Waiting["WaitingForDemand: queued registration"] -->|SDK request grants permission| Fetching["Fetching: one authorized range read turn"]
    Fetching --> Outcome{"Source-read outcome"}
    Outcome -->|Data| Delivered["AwaitingAcknowledgment: send one batch and receipt"]
    Delivered --> Success{"Successful processing and required persistence?"}
    Success -->|Yes| Idle["Idle"]
    Idle -->|Register another turn| Waiting
    Success -->|No| Closed["End acquisition and close delivery"]
    Outcome -->|304: no changes| NoChanges["Remain Fetching: record safe idle position and drop demand"]
    NoChanges --> PollDelay["Await poll delay while servicing checkpoint deadlines"]
    PollDelay -->|Register again| Waiting
    Outcome -->|Ordinary 429| Throttled["Remain Fetching: drop demand before cooldown"]
    Throttled --> Cooldown["Await cooldown while servicing checkpoint deadlines"]
    Cooldown --> Retry{"Retry budget remains?"}
    Retry -->|Yes: register again| Waiting
    Retry -->|No| Closed
    Outcome -->|Other remaining source error| Closed
```

The delay/cooldown boxes describe work done while the actual state remains
`Fetching`; they are not additional enum variants.

| Outcome | State and channel handling | Effect on the pending SDK receive |
| --- | --- | --- |
| Data | Move the response to its original receiver and enter `AwaitingAcknowledgment`. Do not register another turn until receipt completion succeeds. | Return this batch, then SDK dispatch handles it. |
| 304 | Drop the response, record safe idle continuation according to policy, and drop demand before waiting. Remain `Fetching` until readiness is registered again. | Reply sender closes without a batch; the request matcher tries another eligible acquisition. |
| Ordinary 429 | Drop demand before the lease-local cooldown. Keep retry state across turns; register again only when cooldown permits retry. | Reply sender closes without a batch; another eligible range can serve the same receive. |
| Remaining source error | Propagate the error to supervision and end the affected delivery session during cleanup. Explicit source-topology failures can initiate range recovery. | No error batch is returned; the receive can seek another eligible acquisition. |

Idle progress needs no application callback because no changes were returned.
Eager can persist that position immediately; Deferred does not force a write
merely because the result was idle. Checkpoint failures while handling idle,
cooldown, or acknowledgment remain maintenance failures, not source-read 429
or topology signals.

Implementation anchors: the `idle` branch, throttle-error branch, and
`session.deliver(...)` in `poll_feed()`; the response-channel retry loop in
`ChangeFeedRequestDistributor::read_next_change_feed_batch()`.

### Visualizing the waiting list and read outcomes

The queue contains waiting registrations, not batches. With A and B ready,
the front is on the left:

```text
SDK requests one batch
        |
        v
Queue [A, B] -- select A --> Queue [B]
                                |
                                v
                         A: Fetching
                                |
              +-----------------+------------------+
              |                 |                  |
             DATA              304                429
              |                 |                  |
              v                 v                  v
 Send ONE batch to SDK    Handle idle progress   Drop demand
 A: AwaitingAcknowledgment Drop demand           Wait cooldown
              |           Wait poll delay       A stays Fetching
              |           A stays Fetching          |
              v                 |                  |
 Callback success               +--------+---------+
 -> acknowledge                          |
 -> persist if required                  v
 -> A: Idle                   Same SDK receive can select B
 -> re-register A             A re-registers after delay/cooldown
              |               if permitted, without reading yet
              v
 Queue [B, A] if B has not already been selected
```

Eager persists before acknowledgment completes; Deferred may re-register with
progress pending in memory. A 304 needs no callback. An ordinary 429 accepts
no new progress, and retry exhaustion ends the acquisition rather than
re-registering. Delay and cooldown are not separate delivery-state variants.

Ownership loss changes the waiting list only after the old worker detects it:

```text
Queue [A/X, B] -- remote takeover --> Queue [A/X, B] (not notified yet)
      |
      +-- detect loss, cancel/join, close X --> Queue [B]

Later reacquire A with fresh session Z
      |
      +-- register Z --> Queue [B, A/Z]

Old X cleanup removes X's entries, never Z's.
```

`X` and `Z` identify different delivery objects for the same lease A.
If the old acquisition already left the queue to read or deliver, cancellation
and receipt handling govern that in-flight work. The next subsection explains
the ownership checks; external callback effects cannot be undone.

### How ownership loss affects queue membership

The [ownership checks](#preventing-stale-or-competing-ownership-writes)
operate at lease-document writes, not queue selection. When a fenced operation
detects loss, supervision cancels
processing and ends that session.

The old delivery session is closed and its queue entries are removed by
**delivery-object identity**, not lease ID. A later acquisition of the same
lease creates a fresh object, so old cleanup cannot remove its registration.
If the old session was already selected or its batch delivered, cancellation
and receipt handling govern that in-flight work.

**Queue selection does not read Cosmos DB to revalidate ownership before
every source read.** Stale reads and callback overlap are possible before
ownership loss is detected. Fencing protects stored ownership and progress;
it cannot undo external application effects.

### Tie successful processing to the next delivery

The SDK completes the batch's receipt only after successful deserialization
and handler execution.
The driver already holds the continuation for that batch; the SDK does not
choose an arbitrary saved position.

Checkpoint policy determines what acknowledgment completion requires:

- **Eager:** persist the successfully processed position before completing
  acknowledgment and permitting another read from that acquisition.
- **Deferred:** accept safe progress in memory and permit another read; attempt
  persistence when the configured interval since the first unsaved progress
  expires, or at an explicit/lifecycle barrier. Saved progress can therefore
  lag behind processed progress.

#### How acknowledgment records safe progress

After the handler succeeds, SDK dispatch calls:

```rust
receipt.acknowledge().await
```

The receipt sends an acknowledgment to the waiting driver poller and waits
for its completion result. It does not itself write to Cosmos DB. The poller
already knows the continuation associated with the delivered batch:

```rust
lease.acknowledge(token, handled_changes);
```

Here, `lease` is the local `LeaseState`. Its pending checkpoint records the
latest safe token, handled-change count, and time of the first unsaved
progress. Later successful acknowledgments update the token and count without
resetting that timestamp. No completed batch payload is retained.

#### How Eager persists progress

Inside `poll_feed()`, the policy branch and delivery transition are:

```rust
let result = if acknowledgment.durable
    || context.options.checkpoint_policy == CheckpointPolicy::Eager
{
    context.flush(&mut lease).await
} else {
    Ok(())
};
let result = result.and_then(|()| session.acknowledged());
```

The receipt's `checkpoint()` method requests durable persistence explicitly
under either policy; this sets the `acknowledgment.durable` flag in the excerpt.
`and_then` reopens delivery only if the required work succeeded:
`AwaitingAcknowledgment` becomes `Idle`. The poller replies to the SDK and
can register for another turn; after dispatch completes, the SDK requests
another batch. Neither side must register first.

`flush()` writes only pending safe progress:

```rust
if let Some(pending) = lease.pending.clone() {
    if let Err(err) = self.update(lease, Update::Checkpoint(&pending.token)).await {
        if let Some(pending) = &mut lease.pending {
            pending.failed = true;
        }
        return Err(err);
    }
}
```

This is a focused excerpt. `update()` reads fresh lease state, validates
ownership, and replaces the document using its ETag. A successful checkpoint
installs updated local state and clears the pending progress. With no pending
progress, `flush()` makes no write.

#### How Deferred schedules persistence

Deferred may acknowledge in memory before that write. `checkpoint_delay()`
computes the time remaining from the first unsaved progress:

```rust
Some(max_interval.saturating_sub(now.saturating_duration_since(pending.since)))
```

An overdue checkpoint produces zero delay. `wait_with_checkpoint()` polls
the current operation alongside that deadline and cancellation signals.
When the deadline fires, it awaits the same `flush()` operation, then resumes
waiting on the original operation rather than recreating it.

An explicit durable receipt or eligible final cleanup can flush sooner.
The current default Deferred interval is five minutes; the SDK allows callers
to choose a policy and interval.

```text
Eager:    callback success → pending progress → durable write → acknowledgment result
Deferred: callback success → pending progress → acknowledgment result
                                           → deadline or barrier → durable write
```

#### Failure and replay boundaries

A required persistence failure returns an error and ends the affected
acquisition rather than permitting its next batch through the success path.
For either policy, a timeout can be ambiguous: the server may have committed
the write before the caller observed failure.

The saved position and initial-start limitation are discussed in
[restart and saved positions](#restart-and-saved-positions); application
failure behavior is detailed in [failure handling](#how-would-failures-be-handled-and-exposed-to-the-application).

```mermaid
sequenceDiagram
    participant SDK as SDK consumer
    participant D as Driver distributor
    participant P as Feed poller for owned lease
    participant L as Lease container
    Note over SDK,P: SDK demand and readiness registration may arrive in either order
    SDK->>D: Request one batch
    P->>D: Register readiness: WaitingForDemand
    D->>P: Grant read turn: Fetching
    Note over P: Read the source change feed
    P->>D: Send batch and receipt: AwaitingAcknowledgment
    D->>SDK: Return batch
    Note over SDK: Deserialize and await application handler
    SDK->>P: Acknowledge successful processing
    Note over P: Record safe pending progress
    alt Eager or explicit durable receipt
        P->>L: Ownership-validated checkpoint write
        L-->>P: Write succeeds
    else Deferred
        Note over P,L: Accept in memory and persist at deadline or barrier
    end
    Note over P: Reopen delivery: Idle
    P-->>SDK: Acknowledgment result
    Note over SDK,P: SDK requests again and poller re-registers for another turn
```

### Preserve the ordering during failures

Idle or throttled source reads release demand without invoking the handler,
allowing the same pending receive to try another eligible acquisition.
Handler and checkpoint failures follow the outcomes described in
[failure handling](#how-would-failures-be-handled-and-exposed-to-the-application).

Queue changes and delivery transitions are protected by mutexes. Network reads
and callbacks run outside those short queue locks; lease renewal and checkpoint
writes serialize through the lease-state mutex. Deferred deadline handling can
persist earlier safe progress while another operation is pending, without
accepting an unfinished batch.

**The invariant is one outstanding unacknowledged batch per acquisition, not
one global batch and not a global memory limit.** Different workers progress
independently. A replacement acquisition gets new delivery state, and stale
ownership turns cannot update its saved progress.

## How would partition splits and merges affect processing?

A **physical partition split** replaces one source partition with smaller
partitions. A **merge** combines physical partitions. The processor's leases
represent partition key ranges, so their saved positions must remain valid
as the physical layout changes.

**Recovery is triggered by a source-feed topology failure, not by a lease
checkpoint error or a background scan.** It affects the failed acquisition,
not every worker or the workload's bootstrap records.

### Stop the affected acquisition and settle safe progress

When source planning or execution reports a topology change, `poll_feed()`
returns an explicitly tagged `PollError::SourceTopologyChange`. Its enclosing
`poll()` attempts to flush eligible acknowledged progress. A failed flush
prevents recovery from proceeding with unpersisted progress.

The supervisor settles processing and renewal before dispatching recovery.
It does not initiate recovery during shutdown. A renewal failure remains a
lease-operation failure, even if its status describes topology change.

Other acquisitions remain coordinated independently. The pending SDK receive
can try another eligible acquisition when the failed read drops its reply
sender.

### Discover replacement ranges

`recover()` refreshes physical ranges overlapping the affected lease and
intersects them with that lease's boundaries. It sorts the intersections and
checks exact coverage, rejecting gaps or overlaps.

If several ranges cover the scope, the split path ensures corresponding child
leases. If just one remains, `recover()` returns `false`: there is no child
replacement. Normal cleanup releases the logical lease, which can be
reacquired and replanned.

**A physical merge does not merge saved lease progress.** Separate logical
range leases can remain inside the merged physical partition, preserving
their independent positions.

### Ensure children before deleting the parent

`split()` validates parent identity and child coverage, requires no pending
checkpoint progress, and conditionally renews the parent:

```rust
self.validate(&parent.value)?;
validate_children(&parent.value.range, children)?;
if parent.pending.is_some() {
    return Err(error(
        StatusCode::Conflict,
        "Split requires durable progress.",
    ));
}
self.update(parent, Update::Renew).await?;
```

For each child, the driver projects the parent's saved continuation onto the
child's range, then creates the missing lease unowned. Existing child ownership
and progress are preserved. Creation is followed by read-back validation;
this is stronger than bootstrap's lease-create conflict handling.

Child validation requires the expected range and a continuation, not equality
with the newly projected token: an existing child may already have advanced.
If further splits have occurred, existing replacement coverage can satisfy
the child range.

Only after every child is ensured does the driver conditionally delete the
parent:

```rust
self.store().delete(&id, parent.etag.clone()).await
```

Normal balancing discovers and acquires the children. Recovery does not bypass
ownership acquisition to start their pollers.

```text
Source-feed topology failure
→ settle acknowledged progress and processing/renewal
→ refresh affected physical ranges and validate coverage
→ validate ownership and renew parent conditionally
→ ensure children with projected saved positions
→ delete parent conditionally
→ acquire children through normal balancing
```

### How ownership checks constrain recovery, checkpoints, and queue readiness

| Activity | Enforcement and effect |
| --- | --- |
| Parent preparation for split | Conditional renewal validates ownership before creating replacement leases. |
| Parent deletion | Uses the parent's ETag. A competing parent mutation causes deletion to fail rather than deleting unconditionally. |
| Detected ownership loss | Cancel and join old processing, close its delivery session, and remove its queue entries by object identity. A replacement session remains separate. |

The general [queue and ownership rules](#how-ownership-loss-affects-queue-membership)
still apply during recovery: readiness is not proof of current ownership,
and checkpoint writes use the previously described fencing.

If ownership changes during recovery, the final conditional parent deletion
must not remove the changed document. Child creation is create-only and
preserves existing state, but the entire recovery is not one transaction or
an uninterrupted ownership lock.

### Interrupted recovery and replay

If a child operation fails, the parent is not deleted by this attempt. Already
created children remain. If parent deletion fails, parent and children can
coexist until a later acquisition retries recovery.

**Children-first ordering supports resumable recovery; it does not make all
steps atomic.** Overlapping documents during partial recovery, repeated
topology changes, and ownership races need explicit test coverage.
Application-effect and replay limitations are collected in
[failure handling](#how-would-failures-be-handled-and-exposed-to-the-application).

Implementation: `azure_data_cosmos_driver/src/change_feed_processor/processing/topology.rs`,
`recover()`, `split()`, `checkpoint()`, and `has_replacements()`, together with
`poll()` and `supervise()` in `processing.rs`.

## How would a worker stop and resume processing?

**Requesting shutdown is different from confirming cleanup has finished.**
The Rust SDK provides an awaitable graceful `stop()`; dropping the processor
requests cancellation without waiting.

### Graceful shutdown

1. The SDK requests driver shutdown.
2. The driver stops granting new read turns and acquiring new leases.
3. Issued work settles. A batch already handed to the SDK can finish its
   handler and acknowledgment; a read not yet delivered can be discarded.
4. Lease renewal continues while processing drains.
5. Each poller attempts to persist eligible acknowledged progress, including
   pending Deferred progress. It does not acknowledge unfinished or failed
   application work.
6. Supervisors join processing and renewal, then conditionally release leases.
7. The coordinator joins outstanding acquisition work and supervisors, and
   retries recorded ownership cleanup without clearing a replacement owner.
8. The SDK waits for its dispatcher to finish and returns the shutdown result.

The public SDK method waits for both driver and SDK completion:

```rust
let result = driver.stop().await;
let dispatched = stopped.await;
result.and(dispatched)
```

There is no implicit callback or overall graceful-stop timeout. A handler
that never completes can prevent graceful drain. Canceling the application's
wait does not mean cleanup has completed or stopped.

### Forced cancellation and process failure

Dropping SDK lifecycle state explicitly requests forced driver shutdown:

```rust
impl Drop for Running {
    fn drop(&mut self) {
        self.driver.request_stop();
    }
}
```

Forced cancellation ends outstanding receipt waits rather than waiting for
application callbacks. SDK dispatch races the callback against lease
cancellation. Already-issued service operations or application side effects
cannot be undone.

The driver still attempts conditional lease cleanup. The SDK's destructor
does not await it, and a process crash cannot reliably run cleanup at all.
Leases left owned become eligible for another worker according to expiration
and processing-inactivity rules.

The driver exposes awaitable `abort()` for forced cleanup; the Rust SDK does
not expose that method. A graceful stop interrupted by forced shutdown reports
the forced outcome rather than claiming a successful drain.

### Restart and saved positions

After its previous processing and cleanup finish, the SDK processor can be
started again. Startup checks workload initialization and preserves existing
lease documents. A replacement acquisition uses saved lease progress before
the configured initial position.

Successfully processed changes whose position was not persisted may replay.
Recovery uses the stored position, including after the
[ambiguous checkpoint outcomes](#failure-and-replay-boundaries) described above.

**Current limitation:** with an initial `Now` start and no durable continuation,
a restart can choose a later starting position. The default Deferred policy
does not yet establish a durable initial anchor before relying on restart.

Stopping one worker does not stop the workload's other workers.
[Rebalancing](#how-start-and-stop-affect-rebalancing) explains when they discover
released leases.

## How would failures be handled and exposed to the application?

Failures are handled at the stage that can distinguish their meaning.
**An error is not an instruction to skip changes or advance a checkpoint.**

### Source reads and lease maintenance

| Failure | Current handling | Application-facing effect |
| --- | --- | --- |
| Ordinary source-read throttling | Release receive demand, apply lease-local cooldown, and retry within the throttle budget. | The pending receive can try another eligible acquisition. |
| Slow source read | Planning/read execution is bounded to five seconds or a shorter configured driver deadline. A remaining timeout ends the acquisition. | The pending receive can continue seeking a batch elsewhere. |
| Other service/transport errors | Applicable driver request/routing policies run first; remaining errors propagate to supervision. | Report failure and end the affected acquisition as appropriate. |
| Source-feed topology change | Preserve explicit source origin and attempt range-local recovery after safe-progress handling. | No topology-error batch is sent to the handler. |
| Lost ownership | Reject stale lease mutations, cancel/join processing, and attempt fenced cleanup. | The callback can be canceled; already-performed effects remain. |
| Checkpoint failure | Return an error to the receipt waiter when applicable and end affected processing. | Callback success may be followed by acknowledgment failure and replay. |
| Lease release failure | Report it and retain cleanup state for the coordinator's final attempt. | Shutdown can fail; unreleased ownership can later expire. |

Metadata and lease writes keep their normal request retries. Ordinary CFP
page-read throttle retries are instead handled outside the request pipeline
so cooldown does not occupy the pending receive.

Deferred deadline failures are maintenance failures, not source-read throttling
or source-topology signals. A failed checkpoint must not cause recovery of an
unrelated source partition.

### Application handler failures

SDK dispatch acknowledges only successful deserialization and handler completion.

- Deserialization or handler errors are logged; the receipt is abandoned.
- An unwinding handler panic is caught and logged without acknowledgment.
- Lease cancellation can end the callback wait without acknowledgment.
- If the callback succeeds but acknowledgment fails, that failure is logged;
  application side effects are not rolled back.

An abandoned receipt ends the acquisition without accepting that batch's
progress. The SDK consumer can request another batch. Previously acknowledged
safe progress may still be eligible for persistence during cleanup.

The handler must report success only after its required effects have completed
and must tolerate replay and possible overlap. Fencing protects stored progress,
not external effects already performed. Repeatedly failing items are not
automatically skipped.

### What the application observes

- SDK `start()` returns initialization errors and rejects duplicate startup on
  the same running processor object.
- SDK `stop()` returns graceful-cleanup errors.
- Later per-lease failures are logged. The SDK has no separate asynchronous
  error callback.
- The driver has an optional synchronous error observer for adapters or advanced
  callers; it must return promptly.
- `read_next_change_feed_batch()` returns a batch or end-of-delivery, not a read-error result:

  ```rust
  pub async fn read_next_change_feed_batch(&self) -> Option<ChangeFeedBatch>
  ```

Channel closure for a failed or idle read causes the distributor to seek
another eligible acquisition. If every acquisition repeatedly fails, receiving
can remain pending while errors are logged; an application must not interpret
that wait as proof of healthy processing.

### Persistent failures and limits

The current worker can release and later reacquire a failing lease. There is
no general acquisition-level suspension/backoff policy for deterministic
handler, authorization, or invalid-continuation failures.

Request-level retry budgets therefore do not guarantee that the workload stops
repeating the same failure. Defining persistent-failure behavior and operational
signals remains open design work.

Handling unwinding panics, cancellation, request deadlines, and TTL recovery
do not cover process aborts or non-yielding application code.

## How would background tasks coordinate polling, renewal, and balancing?

A **spawned task** is asynchronous work scheduled by the runtime. Within one
worker, the SDK spawns a consumer task and the driver spawns a coordinator
task. The coordinator polls a balancing future and the supervisor futures for
its acquired leases. Each supervisor polls feed processing and renewal
together; **there is no separately spawned task or thread per lease**.

```mermaid
flowchart TB
    subgraph SDKTask["SDK consumer task"]
        Consumer["Request batch"]
        Handler["Deserialize and await handler"]
        Ack["Await acknowledgment result"]
        Consumer --> Handler --> Ack --> Consumer
    end
    subgraph DriverTask["Driver coordinator task"]
        Coordinator["run: poll balancing and supervisors"]
        Balance["balance: wait interval, list leases, acquire candidates"]
        subgraph Lease["Supervisor future for each acquired lease"]
            Supervisor["supervise: poll both activities"]
            Feed["poll: demand, source read, receipt wait, safe-progress flush"]
            Renew["renew: wait until due, validate ownership, update lease"]
            Supervisor --> Feed
            Supervisor --> Renew
        end
        Coordinator --> Balance
        Coordinator --> Supervisor
    end
    Delivery["Distributor: demand and response channels"]
    State["Per-lease mutex: serialize renewal and checkpoint writes"]
    Consumer -->|Demand| Delivery
    Delivery -->|Read permission| Feed
    Feed -->|Batch through distributor| Handler
    Ack -->|Successful processing signal| Feed
    Feed -->|Checkpoint mutation| State
    Renew -->|Renewal mutation| State
    Feed -->|Acknowledgment result| Ack
```

The arrows describe coordination, not one synchronous call chain. While an
operation is asynchronously pending, the coordinator can poll other ready
futures. Concurrency does not require each activity to run simultaneously on
a separate OS thread.

### During normal feed processing

| Activity | What makes it run? | Interaction with other work |
| --- | --- | --- |
| Load balancing | First pass starts immediately; later passes wait the acquisition interval. | Can discover and acquire leases while existing supervisors await reads or receipts. Does not wait for an entire feed backlog to drain. |
| Feed processing | A pending SDK receive grants a read turn to an eligible acquisition. | A delivered batch waits for receipt completion before that acquisition can read another batch. |
| Application handling | The SDK consumer receives a batch. | Sequential within this SDK worker; a slow handler prevents that consumer from requesting another batch, but does not itself stop driver balancing or renewal. |
| Lease renewal | Renewal becomes due after the latest successful lease write. | Continues while processing awaits demand, reads, or acknowledgment. Successful checkpoint writes also refresh renewal timing. |
| Deferred persistence | The first unsaved safe progress reaches its deadline, or a barrier requires a flush. | Can flush earlier acknowledged progress while current work remains pending. It cannot checkpoint an unfinished batch. |

The default lease-expiration budget is 60 seconds. Balancing checks use one
sixth of that budget (10 seconds); renewal is scheduled after one third
(20 seconds) without a successful lease write. These are independent of the
source's idle poll delay and application handler duration.

**Renewal and checkpoint mutations for the same lease are serialized by its
mutex.** Source reads and SDK handlers do not hold that mutation lock.
A slow lease-store write can therefore delay another mutation of that lease,
even though its futures are polled concurrently.

### During graceful feed-processing drain

Here, **drain** means finishing already-issued work during shutdown, not
prefetching all remaining changes in the source container.

1. Stop accepting new acquisitions and granting new read turns.
2. Let issued delivery and acknowledgment work settle. The SDK may complete
   a handler for a batch it already received; undelivered read results can be
   discarded.
3. Keep renewing each lease while its processing drains.
4. Flush eligible safe progress when that poller exits.
5. The supervisor cancels and joins renewal, then conditionally releases the
   lease.
6. The coordinator joins in-flight acquisition work rather than forgetting a
   successful ownership write, joins all supervisors, and performs final
   ownership cleanup.

```text
Normal operation:
balancing continues | feed waits/reads/acknowledges | renewal continues

Graceful stop:
close new demand/acquisition
→ finish issued processing while renewing
→ flush safe progress
→ stop and join renewal
→ release ownership
→ coordinator and SDK consumer finish
```

This is coordinated completion, not a periodic global barrier. Each lease's
supervisor settles its own work; the coordinator waits for all of them at
shutdown. If renewal ends first, it cancels and joins processing before
cleanup, using the [ownership-loss path](#detecting-ownership-loss-and-cleaning-up).
Forced cancellation and failures follow the
[shutdown rules](#how-would-a-worker-stop-and-resume-processing), not the
successful-drain sequence.

Implementation anchors: `run()`, `balance()`, `supervise()`, `renew()`, and
`poll()` in `azure_data_cosmos_driver/src/change_feed_processor/processing.rs`;
SDK `consume()` in `azure_data_cosmos/src/change_feed_processor/execution.rs`.

## How would an application configure and control the Rust processor?

The public API is defined in
`azure_data_cosmos/src/change_feed_processor.rs`; lifecycle methods are
implemented in its private `execution.rs` module.

### Supply the workload and application handler

`ChangeFeedProcessorBuilder::new` requires six inputs:

| Input | Meaning |
| --- | --- |
| `mode` | `LatestVersion` or `AllVersionsAndDeletes`. |
| `monitored_container` | Source container whose change feed is read. |
| `lease_container` | Container storing workload coordination and progress. Requires `/id` partitioning and default TTL `-1`. |
| `workload_name` | Shared name for workers cooperating on this source and lease storage. |
| `worker_name` | Distinct name for this running worker; uniqueness is the caller's responsibility. |
| `handler` | Required async function-pointer callback for typed changes. |

The handler accepts batch context and a vector of change-feed items:

```rust
pub type ChangeFeedHandler<T, Fut> = fn(ChangeFeedProcessorContext, Vec<ChangeFeedItem<T>>) -> Fut;
```

`T` is the application's deserializable document type. `Fut` is the future
returned by the callback; it must resolve to `crate::Result<()>`.
The current API accepts function pointers, not closures capturing application
state. This is a callback-shape restriction, separate from keeping callback
execution in the SDK rather than the shared driver.

### Choose processing options

Builder setters return the builder so they can be chained. They configure
behavior without starting processing.

| Setter | Default and behavior |
| --- | --- |
| `with_checkpoint_policy(policy)` | Default `ChangeFeedCheckpointPolicy::deferred()`, with a five-minute interval. Choose `Eager` or `Deferred { max_interval }`; Deferred intervals must be positive and representable. |
| `with_max_item_count(hint)` | Default hint of 100 items per lease read. `MaxItemCountHint::ServerDecides` lets the service choose. This is not a hard payload or memory limit. |
| `with_feed_poll_delay(delay)` | Default five seconds after an idle read, independently per lease. Zero disables the delay; nonempty batches do not incur it. |
| `with_bootstrap_timeout(timeout)` | Default 120-second caller wait budget. Must be positive and representable; does not extend claims or cover optional restart verification. |
| `with_min_scale_count(minimum)` | Default zero. Sets the worker's minimum lease target, not a number of processes or threads. |
| `with_max_scale_count(maximum)` | Default zero means uncapped lease count. A nonzero maximum must not be below the minimum. |
| `with_lease_verification_enabled_on_restart(enabled)` | Default false. Check the persisted initial coverage and recreate missing leases while preserving existing progress and replacement coverage. Invalid coverage fails startup. |

There is no SDK callback-concurrency setter; the
[processing flow](#shared-rust-engine) describes worker-level parallelism.

### Select Eager or Deferred checkpointing

Pass either policy to `with_checkpoint_policy()`. **Deferred is the default**;
the convenience constructor uses a five-minute interval:

```rust
ChangeFeedCheckpointPolicy::deferred()
```

To configure the interval explicitly:

```rust
ChangeFeedCheckpointPolicy::Deferred {
    max_interval: Duration::from_secs(60),
}
```

Replace the Eager setter in the builder example below with:

```rust
.with_checkpoint_policy(ChangeFeedCheckpointPolicy::Deferred {
    max_interval: Duration::from_secs(60),
})
```

`max_interval` must be positive and representable. It measures time from the
first unsaved safe progress, not the most recent batch. Deferred acknowledgment
can succeed before persistence, so acknowledged changes may replay after
interruption. For immediate per-batch persistence, select
`ChangeFeedCheckpointPolicy::Eager`.

### Choose where new leases begin reading

Saved lease progress takes precedence over all initial-start options.
For leases without saved progress, the last start setter wins:

| Setter | Behavior |
| --- | --- |
| `with_start_from_now()` | Default: start from now. Clears an earlier initial continuation. |
| `with_start_from_beginning()` | Start from the beginning; requires `LatestVersion`. Clears an earlier initial continuation. |
| `with_start_time(start_time)` | Start after the supplied time; requires `LatestVersion`. Clears an earlier initial continuation. |
| `with_start_continuation(continuation)` | Set an initial continuation and replace earlier start settings. Blank tokens are rejected; driver validation checks applicability to the source/range/mode. |

`AllVersionsAndDeletes` does not support beginning or time-based starts.
Initial `Now` without a durable saved position has the
[restart limitation](#restart-and-saved-positions) described above.

### Build, then start

The following is a usage excerpt. `source`, `leases`, and `handle_changes`
represent an application's configured container clients and async handler:

```rust
let processor = ChangeFeedProcessorBuilder::new(
    ChangeFeedMode::LatestVersion,
    source,
    leases,
    "orders",
    "worker-01",
    handle_changes,
)
.with_checkpoint_policy(ChangeFeedCheckpointPolicy::Eager)
.with_feed_poll_delay(Duration::from_secs(5))
.build()?;

processor.start().await?;
```

`build()` consumes the builder and validates local configuration without I/O.
`start()` then performs the initialization and lifecycle behavior below.

### Inspect batch context and lifecycle

The handler's `ChangeFeedProcessorContext` exposes:

- `lease_token()`: the batch's lease identity.
- `diagnostics()`: an owning `Arc<DiagnosticsContext>` that can be retained
  after response consumption. Retention is not proof of current ownership
  and does not authorize checkpoints.

The processor exposes these lifecycle methods:

| Method | Behavior |
| --- | --- |
| `start().await` | Initialize/resume and launch processing; does not wait for the first batch or ownership acquisition. |
| `is_started()` | Read local running state; not proof of ownership or service connectivity. |
| `stop().await` | Gracefully drain callbacks, persist eligible progress, and release leases. Returns cleanup errors; no implicit overall timeout. |
| `current_state().await` | Currently unimplemented: returns `NotImplemented` without I/O. |

When the application is ready to shut down:

```rust
processor.stop().await?;
```

For drop/cancellation semantics, see [shutdown and restart](#how-would-a-worker-stop-and-resume-processing).
The Rust SDK does not expose `abort()` or a separate asynchronous error-handler
registration method.

## Open questions and scope limits

- **Partition-key prefix scope:** should a customer be able to restrict a
  workload to a partition-key prefix rather than the whole source container?
- **Process-until timestamp:** should a customer be able to specify a timestamp
  up to which changes are processed, then have the workload finish?
- **Alternate lease stores:** this design considers Cosmos DB lease storage
  only. Alternate lease stores are not currently being considered.
- **Initial restart position:** how should an initial `Now` position remain
  stable across restart before the first durable Deferred checkpoint, so
  changes are not missed?
- **Persistent failures:** should repeatedly failing acquisitions back off or
  suspend, and how should the application observe failure when batch receives
  remain pending?

Prefix scope and process-until behavior are proposed use cases, not implemented
processor options.

## How would we iterate on and review changes?

Build confidence through progressively harder scenarios, rather than reviewing
every component at once. **Start with Eager to isolate delivery and ownership
correctness; add Deferred after those contracts are established.**

| Stage | Scenario | Review and test focus |
| --- | --- | --- |
| 1 | One worker, fixed leases, Eager | Read, handler success, durable checkpoint, and next batch. Verify restart from saved progress and failure without checkpoint advancement. |
| 2 | One worker, partition splits, Eager | Exact replacement coverage, continuation projection, and children-before-parent deletion. Inject interrupted child creation and failed parent deletion. |
| 3 | Multiple workers, partition splits, Eager | Balancing, competing acquisition, ownership loss, stale acknowledgments, and recovery races. |
| 4 | Multiple workers, partition splits, Deferred | Safe pending progress, deadline/barrier flushes, permitted replay, and topology recovery with delayed persistence. |

Before stage 3, test workers joining, leaving gracefully, and crashing with
fixed topology. This separates ownership failures from split recovery failures.
Before stage 4, test one-worker Deferred deadline and graceful-stop flushes
without topology changes.

Include merges in the topology stages and exercise both feed modes where the
scenario is supported. Document unsupported scenarios rather than weakening
their expected results.

### Review each increment against its contract

For each scenario:

1. State expected behavior before deriving assertions from the implementation.
2. Run a deterministic normal path and focused failure-injection cases.
3. Review the code needed to explain delivery, ownership, and persistence.
4. Establish explicit pass criteria before introducing the next interaction.

Pass criteria must cover delivered changes, permitted replay, durable progress,
lease-range coverage, and cleanup. Distinguish missed changes from duplicates
that are allowed by replay; do not assume exactly-once callback side effects.

Use existing in-memory and fault-injection harnesses first. Report compiled,
executed, ignored, emulator, and live-service coverage separately. The stages
above are an iteration strategy, not a claim that all scenarios have already
passed.
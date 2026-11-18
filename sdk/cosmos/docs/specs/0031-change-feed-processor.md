# Change feed processor: batch processing and public API

> **Status:** Draft
> **Scope:** Rust SDK processing and the shared cross-language driver engine
> **Crates:** `azure_data_cosmos`, `azure_data_cosmos_driver`, `azure_data_cosmos_driver_native`

## 1. Problem

The Cosmos DB Change Feed Processor provides automatic change-feed processing
for the Rust SDK and a shared Rust engine for other language SDKs.

The [public Change Feed Processor documentation](https://learn.microsoft.com/azure/cosmos-db/change-feed-processor)
describes its purpose:

> "simplify the process of reading the change feed and distribute the event processing across multiple consumers effectively."

## 2. Moving pieces

A **workload** is a set of processors reading one source container under a
shared workload name. A **worker** is one running processor instance, not
necessarily a separate process, machine, or thread.

| Component | Responsibility |
| --- | --- |
| Application | Supplies the handler that processes a batch of changes and reports success or failure. |
| SDK | Requests batches, deserializes changes, invokes the application handler, and acknowledges successful processing. |
| Driver | Initializes the workload, manages leases, reads the source change feed, balances ownership across workers, and persists checkpoints. |
| Native/FFI adapter | Exposes driver operations to other language SDKs. GoV2 is the first integration to validate. |
| Source container | Supplies change-feed data. |
| Lease container | Stores shared coordination records, range ownership, and durable resume positions. |

A **language shim** adapts the shared engine to a language's SDK types and
execution model through the native foreign-function interface (FFI).
Application handlers execute in their SDK's runtime, not in the shared driver.

**Feed processing** is the complete request, source read, handler, and
acknowledgment cycle. A **source read** is the driver operation that reads
changes from Cosmos DB. A **checkpoint** durably saves a safe resume position.

## 3. Functional requirements

- Deliver changes from each physical partition with **at-least-once delivery
  semantics**.
- Monitor **all physical partitions within the configured scope**, including
  replacement partitions after topology changes.
- Preserve existing ownership and saved progress when initializing or resuming
  a workload.
- Acknowledge only successful processing and never advance saved progress past
  unprocessed changes. Interruption can cause replay.
- Distinguish healthy idle processing from stalled operations, expose failures,
  and confirm processing activity after recovery.
- Support graceful shutdown and safe cancellation without changing replacement
  ownership or acknowledging unfinished work.

Delivery is bounded by the selected feed mode and service retention. Latest
Version does not promise every intermediate version or delete event.
At-least-once delivery permits duplicate callbacks and does not guarantee
exactly-once application effects.

## 4. Non-functional requirements

- Distribute processing load equally across cooperating workers.
- Keep the shared driver independent of language-specific application handlers.
- Allow lease renewal and balancing to continue while the SDK awaits an
  asynchronous application handler.

## 5. How does the SDK layer request for changes?

The SDK requests one batch, invokes the application handler, and acknowledges
successful processing before requesting another batch. The calls are
asynchronous; awaiting a response suspends the SDK task rather than blocking
its executor thread.

```text
Application boundary  SDK boundary                  Driver boundary
--------------------  ----------------------------  -------------------------
Application          SDK                         Driver
    |                 |                             |
    | start()         |                             |
    |---------------->| Start processor             |
    |                 |---------------------------->|
    |                 |                             | Initialize workload
    |                 |                             | Manage leases
    |                 |<----------------------------|
    |<----------------|                             |
    |                 |                             |
    |                 | Request batch               |
    |                 |---------------------------->|
    |                 |                             | Read owned range
    |                 | ChangeFeedBatch             |
    |                 |<----------------------------|
    |                 |                             |
    |                 | Deserialize                 |
    | Run handler     |                             |
    |<----------------|                             |
    | Success         |                             |
    |---------------->| batch.acknowledge()         |
    |                 |---------------------------->|
    |                 |                             | Record safe progress
    |                 |                             | Checkpoint per policy
    |                 | Acknowledgment completes    |
    |                 |<----------------------------|
    |                 |                             |
    |                 | Repeat                      |
```

- The SDK acknowledges the driver batch automatically after handler success.
  The application sees typed changes and context, not the driver batch.
  Handler failure means no acknowledgment.
- Lease renewal and balancing run independently of the SDK's batch-processing
  loop.
- **Eager** acknowledgment waits for checkpoint persistence. **Deferred**
  acknowledgment may complete with safe progress held in memory; interruption
  before persistence can cause replay.

## 6. How does the driver respond to an SDK request?

The SDK starts with an ordinary async method call:

```rust
let batch = processor.read_next_change_feed_batch().await;
```

**A one-shot channel connects background completion to that pending method
call.** The method awaits the receiver internally; background work gets the
sender. Sending a value wakes the receiver so the method can return it. The
SDK awaits the method, not a channel it must manage. The method call itself is
not another channel.

`ChangeFeedRequestDistributor` matches the SDK's pending request to a ready,
owned lease and grants one source-read turn. Its
`read_next_change_feed_batch()` future owns the one-shot response receiver
while the SDK awaits the returned batch. The sender is carried in `ReadDemand`
to `poll_feed()` through `AcquisitionDelivery::wait_for_demand()`.
The SDK does not hold or manipulate the channel directly.

The diagram names the driver components and functions used by the delivery
mechanism. `batch.acknowledge()` is the proposed adapter-facing API; the
acknowledgment channels remain internal.

```text
SDK boundary       Driver boundary
-----------------  ------------------------------------------------------------------
SDK consumer       ChangeFeedRequestDistributor                 poll_feed()
 |                              |                                  |
 |                              |           AcquisitionDelivery::wait_for_demand()
 |                              |<--------- Register ready delivery |
 | read_next_change_feed_batch()|                                  |
 |----------------------------->|                                  |
 |                              | Select queued delivery            |
 |                              | Create response channel           |
 |                              | Send ReadDemand { response }      |
 |                              |--------------------------------->|
 |                              | Await received                    | Plan/read source
 |                              |<----- AcquisitionDelivery::deliver(...)
 | ChangeFeedBatch              |                                  |
 |<-----------------------------|                                  |
 |                              |                                  |
 | Run application handler      |                                  | Await acknowledgment
 | batch.acknowledge()           |                                  |
 |---------------------------------------------------------------->|
 |                              |                                  | LeaseState::acknowledge(...)
 |                              |                                  | Context::flush(...) if required
 |                              |                                  | AcquisitionDelivery::acknowledged()
 |<-------------------------------------- Acknowledgment result -----|
```

These components have distinct roles:

- `ChangeFeedRequestDistributor` selects a queued delivery and awaits its
  response. It does not execute source reads or checkpoints.
- `AcquisitionDelivery` registers readiness, delivers the batch through the
  supplied sender, and controls when that acquisition can register again.
- `poll_feed()` performs source reads and handles acknowledgment for one
  acquisition. The term **lease processor** elsewhere refers to this driver
  operation, not a separate type. It does not invoke application handlers.

- If no lease is ready, the SDK request waits asynchronously. The waiting list
  contains ready lease registrations, not fetched batches.
- An empty or throttled read releases its response sender without a batch.
  The same pending SDK request can select another eligible lease; the affected
  lease waits its idle delay or throttle cooldown before registering again.
- When a read returns changes, the lease processor sends a `ChangeFeedBatch`
  through the response channel. That lease cannot deliver another batch until
  acknowledgment completes successfully.
- On acknowledgment, the lease processor records safe progress and persists it
  as required by the checkpoint policy before returning the completion result.
  Dropping the batch without acknowledgment does not accept its progress.
- Lease renewal and balancing continue independently of these waits.

Batch delivery and acknowledgment are separate asynchronous exchanges:
**request to batch**, then **acknowledgment to completion**.

For one successful batch and acknowledgment, four `oneshot::channel()` calls
create two request/reply pairs:

| SDK call | Internal request channel | Internal response channel |
| --- | --- | --- |
| `read_next_change_feed_batch().await` | Distributor grants the selected acquisition read permission, carrying a batch-response sender. | Acquisition sends the batch to the distributor, which returns it to the SDK. |
| `batch.acknowledge().await` | SDK-side acknowledgment code reports handler success, carrying an acknowledgment-result sender. | Acquisition returns success or failure after any persistence required by policy. |

Each channel pair has one sender and one receiver. The permission channel is
created when the acquisition registers readiness; the other channels are
created as selection, delivery, and acknowledgment proceed. The application
handler runs between the two SDK calls. Empty or throttled turns can create
additional permission/response pairs for the same pending receive; shutdown
and cancellation channels are separate.

### 6.1 Register readiness and match an SDK request

The queue holds **leases ready for feed processing**, not SDK requests or batch data.
The SDK's request is a pending asynchronous receive. Each worker has its own
queue; the lease container coordinates ownership across workers.

1. A lease processor registers at the back of the queue when it can accept a
   source-read turn.
2. The SDK requests a batch. If the queue is empty, the request waits until a
   lease registers or delivery stops.
3. The driver removes the next eligible registration and grants that lease a
   read turn, carrying the response-channel sender for this SDK request.
4. The lease stays out of the queue while reading and awaiting acknowledgment.
   After successful acknowledgment, it can register at the back again.

Registration and the SDK request can arrive in either order. An empty or
throttled read returns no batch and lets the pending SDK request select another
ready lease, as described above.

The states below describe one acquisition's delivery, not the queue as a whole
or persisted lease ownership. Only `WaitingForDemand` has an eligible queue
registration:

| Delivery state | Queue membership and meaning |
| --- | --- |
| `Idle` | Not queued; ready to register for another turn. |
| `WaitingForDemand` | Queued; waiting for an SDK request to select this lease. |
| `Fetching` | Removed from the queue; the source-read turn has been granted. Also remains the state during an idle delay or throttle cooldown, before registering again. |
| `AwaitingAcknowledgment` | Not queued; the SDK is processing the batch or the driver is completing acknowledgment and any required persistence. |
| `Closed` | Acquisition ended; remove or skip its registrations and reject further delivery. |

### 6.2 How queue readiness relates to checkpoints

**The queue does not checkpoint progress.** It controls which lease may read
next. Batch acknowledgment and checkpoint policy determine when that lease becomes
eligible to rejoin the queue.

```text
SDK boundary                        Driver boundary
----------------------------------  -----------------------------------------
SDK consumer                        Ready-lease queue and lease processor
     |                                   |
     | Request batch                     | A: WaitingForDemand; queue [A, B]
     |---------------------------------->| Select A; queue becomes [B]
     |                                   | A: Fetching; read source range
     | ChangeFeedBatch                   |
     |<----------------------------------|
     |                                   |
     | Run application handler           | A: AwaitingAcknowledgment
     |                                   |
     | Acknowledge successful processing |
     |---------------------------------->|
     |                                   | Eager: save continuation durably
     |                                   | OR
     |                                   | Deferred: record safe progress
     |                                   |           in memory
     |                                   |
     | Acknowledgment succeeds           |
     |<----------------------------------|
     |                                   | A: Idle -> WaitingForDemand
     |                                   | Queue: [B, A] if B is still waiting
     | Request next batch                |
     |---------------------------------->|
```

| Outcome | Progress and queue behavior |
| --- | --- |
| Handler fails or batch is abandoned | Do not accept the batch's progress. End the affected acquisition rather than requeue it as successful work. A later acquisition resumes from durable progress. |
| Eager acknowledgment | Save safe progress durably before completing acknowledgment and allowing this lease to rejoin the queue. |
| Deferred acknowledgment | Record safe progress in memory before completing acknowledgment and allowing re-registration. A deadline or explicit/lifecycle barrier later triggers persistence. |
| Required checkpoint fails | Return failure to a waiting acknowledgment and end the affected acquisition; do not reopen delivery through the successful path. |

This keeps at most one batch awaiting acknowledgment per acquisition and
prevents that acquisition from advancing past unfinished processing. It also
rotates ready leases rather than always selecting the same one. Deferred
progress may still replay after interruption: queue readiness is not proof of
durable persistence.

### 6.3 Parallel workers have independent queues

Each running worker has its own SDK consumer and driver ready-lease queue.
In this example, worker W1 owns ranges A and B; worker W2 owns C and D.
The workers can run in the same process or separate processes.

```text
SDK boundary                        Driver boundary
----------------------------------  -----------------------------------------
Worker W1                           Worker W1
  SDK consumer                      Ready-lease queue: [A, B]
       | Request batch ------------------> Select A; queue becomes [B]
       | <---------------- Batch A ------- Lease processor A reads source
       | Run handler A                     A waits for acknowledgment
       | Acknowledge A ------------------> Record progress; persist per policy
       | <---------- Completion ---------- A may rejoin: [B, A]
       |
       |       W1 and W2 progress concurrently
       |
Worker W2                           Worker W2
  SDK consumer                      Ready-lease queue: [C, D]
       | Request batch ------------------> Select C; queue becomes [D]
       | <---------------- Batch C ------- Lease processor C reads source
       | Run handler C                     C waits for acknowledgment
       | Acknowledge C ------------------> Record progress; persist per policy
       | <---------- Completion ---------- C may rejoin: [D, C]

                                    W1 lease coordination --+
                                                             |
                                    W2 lease coordination --+--> Lease container
                                                                 Shared owners
                                                                 and checkpoints
```

The queue contents are illustrative: B or D may already have been selected
before A or C registers again. Each SDK consumer handles one batch at a time,
but a slow W1 handler does not prevent W2 from processing its own batch.
Lease renewal and balancing continue independently in both workers.

There is no shared ready-lease queue across workers. If W2 acquires A from W1, W1 closes
the old delivery session when ownership loss is detected. W2 creates a new
lease processor for A and registers it in W2's queue when ready; no queue entry
or in-flight batch is transferred between workers. Old batches cannot update
the replacement owner's progress. Already-issued application effects can
overlap before loss is detected and must tolerate replay.

## 7. What does the Rust SDK expose?

**Audience: application developers using `azure_data_cosmos`.** Applications
configure a processor, supply a typed handler, and call `start()` and `stop()`.
They do not request driver batches or manually acknowledge them.

| Application-visible type | What the application uses |
| --- | --- |
| `ChangeFeedProcessorBuilder<T, Fut>` | `new(...)`, the configuration setters below, and `build()`. |
| `ChangeFeedProcessor<T, Fut>` | `start().await`, `stop().await`, and `is_started()`. |
| `ChangeFeedHandler<T, Fut>` | Required async function-pointer handler returning the SDK's `Result<()>`. |
| `ChangeFeedProcessorContext` | Handler context with `lease_token()` and read `diagnostics()`. |
| `ChangeFeedCheckpointPolicy` | `Eager`, `Deferred { max_interval }`, and `deferred()` for the default interval. |

The handler uses SDK-owned `ChangeFeedItem<T>` values and the builder uses
`ChangeFeedMode`. Configuration also uses container clients, continuation
tokens, timestamps, durations, and item-count hints. These supporting types
are not additional processor lifecycle objects.

**Not application-visible:** driver `ChangeFeedBatch`, native handles, native
completion queues, and acknowledgment operations. There is no public
`BatchReceipt` type to learn or manage. The SDK performs acknowledgment after
the application handler succeeds.

The application configures five dimensions. The SDK owns its public types;
driver types do not appear in the SDK's public API.

| Dimension | SDK surface | Contract |
| --- | --- | --- |
| Source and progress persistence | Required `monitored_container` and `lease_container` clients. | Read changes from the source; store ownership and durable continuation tokens in the lease container. The containers can use different clients and credentials. The lease container requires `/id` partitioning and default TTL `-1`, with permission to read and mutate coordination records. |
| Workload and worker identity | Required `workload_name` and `worker_name`. | Workers using the same source, lease storage, and workload name cooperate. Each running worker needs a distinct name. |
| Feed mode and initial position | Required `ChangeFeedMode`; start-position setters. | Choose `LatestVersion` or `AllVersionsAndDeletes`. Saved lease progress takes precedence over initial-start settings. |
| Checkpoint policy | `ChangeFeedCheckpointPolicy::Eager` or `Deferred { max_interval }`. | Eager saves progress before acknowledgment completes. Deferred may acknowledge in memory and persist later, allowing more replay after interruption. |
| Application handler | Required `ChangeFeedHandler<T, Fut>`. | Receive typed items and batch context; return asynchronous success only after required application effects finish. The SDK acknowledges on success, not on failure. |

### 7.1 Builder and lifecycle

The proposed builder requires the mode, both containers, both names, and the
handler. The following is an API usage excerpt, not a standalone program:

```rust
let processor = ChangeFeedProcessorBuilder::new(
    ChangeFeedMode::LatestVersion,
    source,
    leases,
    "orders",
    "worker-01",
    handle_changes,
)
.with_start_from_now()
.with_checkpoint_policy(ChangeFeedCheckpointPolicy::Deferred {
    max_interval: Duration::from_secs(300),
})
.build()?;

processor.start().await?;
// The SDK runs the request, handler, and acknowledgment loop.
processor.stop().await?;
```

`build()` validates configuration without I/O. `start()` initializes or resumes
the workload and launches coordination; it does not wait for lease acquisition
or the first batch. `stop()` drains issued processing, persists eligible safe
progress, and conditionally releases leases. It has no implicit handler
timeout. `is_started()` reports local lifecycle state, not service health.

### 7.2 Feed mode and start position

| Setting | Meaning and restrictions |
| --- | --- |
| `ChangeFeedMode::LatestVersion` | Process the latest-version feed; not every intermediate version or delete event. |
| `ChangeFeedMode::AllVersionsAndDeletes` | Process version and delete events within the service's supported retention. |
| `with_start_from_now()` | Default for leases without saved progress. |
| `with_start_from_beginning()` | Start from the beginning; Latest Version only. |
| `with_start_time(time)` | Start after the supplied time; Latest Version only. |
| `with_start_continuation(token)` | Resume an initial continuation applicable to this source, range, and mode. Blank or incompatible tokens are invalid. |

The last start-position setter replaces previous initial-start settings.
It never overrides saved lease progress. A missing or unusable checkpoint
must not silently fall back to a later `Now`.

**Open: initial resume anchoring.** Establishing a durable initial position
before the first normal checkpoint is required to avoid skipping changes on
restart. The mechanism must be resolved for each supported mode and start
position; choosing Deferred does not waive this requirement.

### 7.3 Checkpoint policy and handler

Deferred is the proposed default, with a five-minute interval measured from
the first unsaved safe progress. Later acknowledgments do not reset that
clock. An idle read does not itself force Deferred persistence; graceful
shutdown or topology recovery can flush sooner. The interval must be positive
and representable. Neither policy makes application effects and checkpoints
one transaction.

The handler shape is:

```rust
pub type ChangeFeedHandler<T, Fut> =
    fn(ChangeFeedProcessorContext, Vec<ChangeFeedItem<T>>) -> Fut;
```

`T` is the application's deserializable item type; `Fut` resolves to the SDK's
`Result<()>`. `ChangeFeedItem<T>` represents changes for the selected feed
mode. `ChangeFeedProcessorContext` provides `lease_token()` and read
`diagnostics()`. The SDK handles deserialization and invokes handlers
sequentially within one worker.

This callback shape accepts function pointers, not capturing closures.
Callbacks must tolerate replay. Handler errors, deserialization errors, and
unwinding panics do not acknowledge the batch. The SDK does not expose driver acknowledgment state or require the application
to call the driver directly.

### 7.4 Additional user options

| Option | Default | Behavior |
| --- | --- | --- |
| `with_max_item_count(hint)` | 100 | Per-source-read item-count hint, not a hard batch-size or memory limit. `ServerDecides` lets the service choose. |
| `with_feed_poll_delay(delay)` | Five seconds | Idle read delay per lease; zero disables it. Nonempty reads do not incur the delay. |
| `with_bootstrap_timeout(timeout)` | 120 seconds | Positive, representable caller wait budget for initialization. Scaling initialization to large inventories remains an open design question. |
| `with_min_scale_count(minimum)` | Zero | Minimum target lease count, not a guarantee of available work. |
| `with_max_scale_count(maximum)` | Zero, meaning uncapped | Maximum owned lease count. A nonzero maximum must not be below the minimum. |
| `with_lease_verification_enabled_on_restart(enabled)` | False | Rediscover ranges and verify lease coverage on restart. Restore missing coverage only with a safe resume position; otherwise fail instead of resetting to `Now`. |

Lease renewal and balancing are driver-managed: the proposed defaults are
60-second ownership expiration, renewal due after 20 seconds without a
successful lease write, and balancing checks every 10 seconds after an
immediate first pass. These are not additional SDK tuning setters.

## 8. What does the driver expose to SDK adapters?

**Audience: SDK and native-adapter implementers, not application developers.**
The processor and option types in this section belong to
`azure_data_cosmos_driver`, even where their names match SDK-owned types.

The driver exposes processor operations and an acknowledgeable batch, not
application handlers or a separate public receipt type.
These are Rust APIs consumed by the Rust SDK and native adapter; the Go SDK
uses the native API rather than Rust types.

| Surface | Exposed behavior |
| --- | --- |
| `ChangeFeedProcessorOptions::new(mode, workload_name, worker_name)` | Configure driver-owned mode and worker identity. SDK `ChangeFeedMode` maps to the separate driver `ChangeFeedFactoryMode`; these are not one shared public type. |
| `ChangeFeedProcessorOptions` setters | Configure initial start/continuation, item-count hint, idle delay, checkpoint policy, bootstrap timeout, lease-count bounds, and restart verification. |
| `ChangeFeedProcessor::start(source_driver, source, lease_driver, leases, options).await` | Initialize and start a processor using separate source and lease-store driver/container references. Return startup failures directly. |
| `read_next_change_feed_batch().await` | Return `Some(ChangeFeedBatch)` or `None` when delivery ends. Wait asynchronously if no batch is available. |
| `ChangeFeedBatch::lease_token()` / `response()` | Borrow lease identity and the raw response, including read diagnostics. Deserialization belongs to the SDK. |
| `ChangeFeedBatch::acknowledge(self).await` | Consume the successfully processed batch. With Eager, durably checkpoint before returning success. With Deferred, record safe progress in memory for a later checkpoint; success does not confirm durability. |
| `ChangeFeedBatch::cancelled(&self).await` | Observe acquisition cancellation, including ownership loss; adapters can race their handler against this signal. |
| `stop().await` / `abort().await` | Await graceful drain or forced cleanup respectively; cleanup errors are returned. Forced cleanup cannot undo application effects. |
| `request_stop()` / `is_running()` | Request forced shutdown without awaiting cleanup, or inspect local running state. |
| `ChangeFeedProcessorOptions::with_error_observer(...)` | Optional synchronous observer of background errors, scoped to a lease when known. It must return promptly and is not an application batch handler. |

The SDK adapter's loop reduces to the following illustrative control flow:

```rust
while let Some(batch) = processor.read_next_change_feed_batch().await {
    let items = deserialize(batch.response())?;
    handler(items).await?;
    batch.acknowledge().await?;
}
```

Decoding produces application-owned items before the batch is consumed.
Production adapter code must also handle cancellation and report failures;
the sketch shows only the success path.

Driver `CheckpointPolicy` maps to the SDK's separate checkpoint policy type.
The batch owns its single-use acknowledgment state internally. There is no
public receipt to extract or manage separately. Dropping the batch without
completing it abandons its progress. Completion is scoped to the acquisition
that delivered it; stale batches cannot authorize writes using a replacement
owner's ETag.

Startup, batch completion, and shutdown report errors directly. The receive
method returns `Option`, not `Result`: a failed source read can leave the SDK
waiting while other leases are tried and the failure is reported separately.
**Open: health observation.** Define how the SDK exposes persistent failures
and recovery status; a pending receive or local running flag is not proof of
healthy processing.

## 9. Native and GoV2 integration requirements

**Open: the C-ABI contract and Go public API require a separate design pass.**
The Rust driver supplies the processing boundary; each language SDK owns its
application handler, deserialization, and idiomatic configuration. This
proposal does not define native function signatures or Go public types.

The integration must establish:

| Area | Required contract |
| --- | --- |
| Completion delivery | Reuse the existing native completion-queue type and GoV2 client reactor, allowing compatible workers and point operations to share a queue. Correlate each submission with its completion; distinguish rejection, error, end-of-delivery, batch, and acknowledgment results. |
| Worker lifetime | Define creation, startup, graceful stop, forced cleanup, and release of each processor. Each worker retains its own driver distributor and ready-lease queue. Stopping one worker must not shut down the client's shared completion infrastructure. |
| Batch ownership | Define payload lifetime and the single-use acknowledgment state associated with a delivered batch and acquisition. Release abandoned or late batches without accepting unprocessed progress. Decide whether common completion ownership APIs suffice before introducing new extraction or release functions. |
| Acknowledgment | Expose one policy-controlled acknowledgment operation. Eager success confirms durable persistence; Deferred success confirms in-memory acceptance for later persistence. Return status/error, not another batch. Report later Deferred persistence failures separately. |
| Cancellation and cleanup | Define how canceling a Go wait affects pending native work and handler cancellation. Existing GoV2 point-operation cancellation stops waiting, not native execution. Keep resources valid until completion delivery or disposal finishes, and keep completion delivery active during graceful drain. |
| Capacity and progress | Bound outstanding CFP receives and retained batches without starving point operations, acknowledgment, or shutdown. Run application handlers outside the shared reactor. Define admission and cleanup behavior under saturation. |

Configuration must cover the same dimensions as the Rust SDK: source and lease
storage, workload/worker identity, feed mode and initial position, checkpoint
policy, and tuning options. Keep application handlers in Go; applications
should not manage driver batches, native handles, or manual acknowledgments.

GoV2's existing
[point-operation adapter](https://github.com/Azure/azure-sdk-for-go/blob/64e45360ec225bb2280e4ca4cdf9a087fc3fe644/sdk/data/azcosmos/operation_native.go)
and [completion reactor](https://github.com/Azure/azure-sdk-for-go/blob/64e45360ec225bb2280e4ca4cdf9a087fc3fe644/sdk/data/azcosmos/reactor_native.go)
are integration precedents, not proof that their current completion payloads
already support CFP.

Validate the eventual contract end to end with multiple workers and point
operations sharing completion infrastructure. Include pending idle receives,
handler failure, Eager and Deferred acknowledgment, cancellation with late
results, and shutdown under saturation. Native layouts, ownership operations,
Go signatures, and background-error reporting remain unresolved.

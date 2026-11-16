# Rust change feed processor

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
    |                 | Batch + receipt             |
    |                 |<----------------------------|
    |                 |                             |
    |                 | Deserialize                 |
    | Run handler     |                             |
    |<----------------|                             |
    | Success         |                             |
    |---------------->| Acknowledge receipt         |
    |                 |---------------------------->|
    |                 |                             | Record safe progress
    |                 |                             | Checkpoint per policy
    |                 | Acknowledgment completes    |
    |                 |<----------------------------|
    |                 |                             |
    |                 | Repeat                      |
```

- A **receipt** is a single-use handle associating acknowledgment with the
  delivered batch. Handler failure means no acknowledgment.
- Lease renewal and balancing run independently of the SDK's batch-processing
  loop.
- **Eager** acknowledgment waits for checkpoint persistence. **Deferred**
  acknowledgment may complete with safe progress held in memory; interruption
  before persistence can cause replay.

## 6. How does the driver respond to an SDK request?

The driver matches the SDK's pending request to a ready, owned lease and grants
one source-read turn. A one-shot response channel connects that turn to the
waiting SDK call: the SDK call holds the receiver, and the selected lease
processor receives the sender. No request-ID lookup is needed.

```text
SDK                   Driver request matching       Lease processing
 |                              |                           |
 | Request batch                |                           |
 |----------------------------->|                           |
 |                              | Wait for a ready lease    |
 |                              |                           |
 |                              | Grant one source read     |
 |                              | + response-channel sender |
 |                              |-------------------------->|
 |                              |                           | Read change feed
 |                              |                           |
 | Batch + receipt, through the request's response channel  |
 |<---------------------------------------------------------|
 |                              |                           |
 | Run application handler      |                           | Wait for receipt
 |                              |                           |
 | Acknowledge receipt          |                           |
 |--------------------------------------------------------->|
 |                              |                           | Record safe progress
 |                              |                           | Persist if required
 | Acknowledgment result        |                           |
 |<---------------------------------------------------------|
```

The **lease processor** is the driver operation for an acquired lease. It
performs source reads, delivers batches, and handles acknowledgments; it does
not invoke application handlers.

- If no lease is ready, the SDK request waits asynchronously. The waiting list
  contains ready lease registrations, not fetched batches.
- An empty or throttled read releases its response sender without a batch.
  The same pending SDK request can select another eligible lease; the affected
  lease waits its idle delay or throttle cooldown before registering again.
- When a read returns changes, the lease processor sends the batch and receipt
  through the response channel. That lease cannot deliver another batch until
  receipt completion succeeds.
- On acknowledgment, the lease processor records safe progress and persists it
  as required by the checkpoint policy before returning the completion result.
  An abandoned receipt does not accept that batch's progress.
- Lease renewal and balancing continue independently of these waits.

Batch delivery and acknowledgment are separate asynchronous exchanges:
**request to batch**, then **acknowledgment to completion**.

### 6.1 Register readiness and match an SDK request

The queue holds **ready lease registrations**, not SDK requests or batch data.
The SDK's request is a pending asynchronous receive. Each worker has its own
queue; the lease container coordinates ownership across workers.

1. A lease processor registers at the back of the queue when it can accept a
   source-read turn.
2. The SDK requests a batch. If the queue is empty, the request waits until a
   lease registers or delivery stops.
3. The driver removes the next eligible registration and grants that lease a
   read turn, carrying the response-channel sender for this SDK request.
4. The lease stays out of the queue while reading and awaiting acknowledgment.
   After successful receipt completion, it can register at the back again.

Registration and the SDK request can arrive in either order. An empty or
throttled read returns no batch and lets the pending SDK request select another
ready lease, as described above.

### 6.2 How queue readiness relates to checkpoints

**The queue does not checkpoint progress.** It controls which lease may read
next. The receipt and checkpoint policy determine when that lease becomes
eligible to rejoin the queue.

```text
Ready leases: [A, B]
       |
       +-- SDK request selects A --> Ready leases: [B]
                                      |
                         A reads and delivers a batch
                                      |
                              SDK runs handler
                                      |
                       Successful acknowledgment
                                      |
                +---------------------+---------------------+
                |                                           |
              Eager                                      Deferred
                |                                           |
       Save continuation durably                 Record safe progress in memory
                |                                           |
                +---------------------+---------------------+
                                      |
                         Receipt completion succeeds
                                      |
                        A registers at the back again
                         [B, A] if B is still waiting
```

| Outcome | Progress and queue behavior |
| --- | --- |
| Handler fails or receipt is abandoned | Do not accept the batch's progress. End the affected acquisition rather than requeue it as successful work. A later acquisition resumes from durable progress. |
| Eager acknowledgment | Save safe progress durably before completing the receipt and allowing this lease to rejoin the queue. |
| Deferred acknowledgment | Record safe progress in memory before completing the receipt and allowing re-registration. A deadline or explicit/lifecycle barrier later triggers persistence. |
| Required checkpoint fails | Return failure when a receipt is waiting and end the affected acquisition; do not reopen delivery through the successful path. |

This keeps at most one batch awaiting acknowledgment per acquisition and
prevents that acquisition from advancing past unfinished processing. It also
rotates ready leases rather than always selecting the same one. Deferred
progress may still replay after interruption: queue readiness is not proof of
durable persistence.

## 7. What does the Rust SDK expose?

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
unwinding panics do not acknowledge the batch. The SDK does not expose a
receipt or require the application to call the driver directly.

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

The driver exposes processing and receipt operations, not application handlers.
These are Rust APIs consumed by the Rust SDK and native adapter; the Go SDK
uses the native API rather than Rust types.

| Surface | Exposed behavior |
| --- | --- |
| `ProcessorOptions::new(mode, workload_name, worker_name)` | Configure driver-owned mode and worker identity. SDK `ChangeFeedMode` maps to the separate driver `ChangeFeedFactoryMode`; these are not one shared public type. |
| `ProcessorOptions` setters | Configure initial start/continuation, item-count hint, idle delay, checkpoint policy, bootstrap timeout, lease-count bounds, and restart verification. |
| `Processor::start(source_driver, source, lease_driver, leases, options).await` | Initialize and start a processor using separate source and lease-store driver/container references. Return startup failures directly. |
| `read_next_change_feed_batch().await` | Return `Some(ChangeFeedBatch)` or `None` when delivery ends. Wait asynchronously if no batch is available. |
| `ChangeFeedBatch::into_parts()` | Transfer lease identity, raw `CosmosResponse`, and its single-use `BatchReceipt`. Deserialization belongs to the SDK. |
| `BatchReceipt::acknowledge().await` | Consume the receipt and accept successful processing according to Eager or Deferred policy. |
| `BatchReceipt::checkpoint().await` | Consume the receipt and require durable persistence of that batch's safe progress under either policy. This is not a request to checkpoint an arbitrary position. |
| `BatchReceipt::cancelled().await` | Observe acquisition cancellation, including ownership loss; adapters can race their handler against this signal. |
| `stop().await` / `abort().await` | Await graceful drain or forced cleanup respectively; cleanup errors are returned. Forced cleanup cannot undo application effects. |
| `request_stop()` / `is_running()` | Request forced shutdown without awaiting cleanup, or inspect local running state. |
| `ProcessorOptions::with_error_observer(...)` | Optional synchronous observer of background errors, scoped to a lease when known. It must return promptly and is not an application batch handler. |

Driver `CheckpointPolicy` maps to the SDK's separate checkpoint policy type.
Dropping a receipt without completing it abandons the batch without accepting
its progress. Receipt success is scoped to the acquisition that delivered it;
stale receipts cannot authorize writes using a replacement owner's ETag.

Startup, receipt completion, and shutdown report errors directly. The receive
method returns `Option`, not `Result`: a failed source read can leave the SDK
waiting while other leases are tried and the failure is reported separately.
**Open: health observation.** Define how the SDK exposes persistent failures
and recovery status; a pending receive or local running flag is not proof of
healthy processing.

## 9. How does GoV2 use the driver through FFI?

The native adapter exposes opaque processor/receipt handles and asynchronous
operation submission with completion delivery. GoV2 owns deserialization,
the Go handler, and the mapping to idiomatic Go options and errors. The
following native names are proposed; this document does not define the final
Go public API or the complete C ABI layouts.

### 9.1 Native operations

| Native API | Purpose |
| --- | --- |
| `cosmos_processor_request_init` | Initialize a versioned `CosmosProcessorRequest` carrying workload/worker names, feed mode, initial position, and tuning options. |
| `cosmos_processor_queue_create` | Create the native completion queue. This is not the ready-lease queue described in section 6. |
| `cosmos_processor_start_with_policy_submit` | Submit startup with source/lease driver and container handles, the request, and explicit checkpoint policy/interval. `cosmos_processor_start_submit` uses five-minute Deferred persistence. |
| `cosmos_processor_next_submit` | Submit one batch receive. Accepted submission does not mean a batch is ready. |
| `cosmos_processor_acknowledge_submit` | Submit successful processing under the selected policy. Completion can mean in-memory acceptance under Deferred. |
| `cosmos_processor_checkpoint_submit` | Submit successful processing with an explicit durable checkpoint barrier. |
| `cosmos_processor_stop_submit` / `cosmos_processor_abort_submit` | Submit graceful shutdown or forced cleanup. |
| `cosmos_processor_queue_wait` | Retrieve owned completion records. This is a timed blocking native wait; the Go adapter must integrate it with its completion pump, not treat it as an async Rust call. |
| `cosmos_processor_completion_take_processor` / `cosmos_processor_completion_take_receipt` | Detach an owned handle from a completion before freeing that completion. |
| `cosmos_processor_completion_free`, `cosmos_processor_receipt_free`, `cosmos_processor_free` | Release owned native resources. Free an unacknowledged receipt on handler failure. |
| `cosmos_processor_queue_shutdown` / `cosmos_processor_queue_free` | End completion delivery and request forced processor shutdown; do not use queue shutdown as graceful stop. |

Startup maps the same configuration dimensions across FFI: source and lease
handles, names, feed mode and start position, checkpoint policy, and tuning
options. No Go application-handler pointer is passed to the Rust processor.

A completion identifies its submitted operation through correlation data and
contains success/error status, diagnostics, and any batch payload or handles.
The adapter distinguishes submission rejection from an accepted operation's
later error completion, end-of-delivery, policy acknowledgment, and durable
checkpoint completion.

### 9.2 Go adapter processing loop

This is language-neutral pseudocode for the Go adapter, not proposed Go
method signatures:

```text
submit start; await its completion; take processor handle
repeat:
    submit next batch; await its completion
    on error: report failure
    on end-of-delivery: exit
    take receipt handle
    decode or copy payload before freeing its completion
    run Go handler
    on decode or handler failure:
        free receipt without acknowledgment
        report failure
    on success:
        submit acknowledgment (or explicit durable checkpoint)
        if submission is accepted:
            free receipt handle; await acknowledgment completion
        otherwise:
            retain receipt for retry or free it without acknowledgment
        report any submission or completion failure
submit stop; keep completion delivery running until stop completes
free remaining handles and completions, then shut down the queue
```

Completion payload views are borrowed until that completion is freed; copy
them into Go-owned memory or retain the completion until decoding finishes.
An accepted acknowledgment submission consumes the receipt's operation, but
its native handle still requires freeing. Also release operation handles using
the native runtime's existing operation-handle contract.

Go context cancellation must be mapped to native operation cancellation and,
when required, processor abort. Canceling a Go wait alone does not establish
native cleanup or stop the application's side effects. Keep completion
delivery active during graceful stop so issued batches and acknowledgments
can settle. Acknowledgment and stop must remain admissible when batch delivery
is saturated.

**GoV2 integration gate:** execute the basic request/handler/acknowledgment loop
over FFI, including handler failure, cancellation, durable checkpoint
completion, and shutdown under saturation. Final Go signatures, per-receipt
cancellation visibility, and cancellation mapping require validation; this is
not a claim that the Go adapter is implemented.

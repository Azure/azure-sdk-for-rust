# Azure Cosmos DB Change Feed Processor

Read typed LatestVersion or AllVersionsAndDeletes change-feed events and run
a bounded, already-owned single-lease processing path through the Cosmos driver.

This crate is unpublished and experimental. This is a foundation slice, not a
production distributed processor: weighted balancing and durable topology
handoffs are not implemented. One pre-created Cosmos-backed lease supports
conditional acquisition, renewal during handling, checkpoint, takeover, and release.

A bounded workload bootstrap is also available through the shared-core
`BootstrapStore`: callers provide explicit recoverable seed positions and
complete expected logical coverage in an existing `/workload` lease container.
Readiness is verified on restart; partial initialization failures never become
success. Obtain processing stores through `work_lease_store` so acquisition is
fenced against the workload's current readiness generation.

Shared-core `LeaseBalancer` adds decentralized equal logical-lease counts for
an already-initialized workload. It reads all inventory pages and attempts one
conditional pickup/transfer per cycle. The ceiling target does not force
unnecessary transfers in balanced distributions. Membership/expiry are
approximate observations, not a globally registered worker list. The policy and
effects remain in the shared core; this crate only re-exports their APIs.

## Data-plane hierarchy

### Credential-bound preparation

Use `ContainerBinding` twice to bind the feed and lease containers independently.
Each accepts an application-supplied `Arc<dyn TokenCredential>` or an explicit
`Secret` account key, resource names, routing preferences, and request defaults.
Reuse a credential explicitly when appropriate; no processor credential or
default identity is silently applied to both accounts.

```rust no_run
use std::sync::Arc;
use azure_core::credentials::TokenCredential;
use azure_cosmos_change_feed_processor::{ChangeFeedProcessor, ContainerBinding};

# async fn example(
#     feed_credential: Arc<dyn TokenCredential>,
#     lease_credential: Arc<dyn TokenCredential>,
# ) -> Result<(), Box<dyn std::error::Error>> {
let feed = ContainerBinding::with_token_credential(
    std::env::var("FEED_ENDPOINT")?.parse()?,
    "application-db", "orders", feed_credential,
)?;
let leases = ContainerBinding::with_token_credential(
    std::env::var("LEASE_ENDPOINT")?.parse()?,
    "coordination-db", "leases", lease_credential,
)?;
let processor = ChangeFeedProcessor::builder()
    .build("orders-projection", feed, leases)
    .await?;
// Explicit bounded processing APIs are available on the prepared processor.
# let _ = processor;
# Ok(())
# }
```

`build` prepares both sides under one overall deadline (60 seconds by default,
configurable up to five minutes). It returns a processor only after both
drivers and container references are ready. Failures identify the feed or lease
side and preserve Cosmos status and diagnostics. Build does not create resources,
acquire leases, initialize a workload, invoke handlers, or write authorization probes.
Preparation may require multiple metadata requests, pages, and retries.

Prepared resources are reused for feed reads, bootstrap, acquisition, renewal,
checkpointing, and release. CFP does not resolve container names again in these
paths; driver-managed topology, account, and invalidated-metadata refresh remains
allowed. `lease_store` and `bootstrap_store` address the retained lease reference
locally. `bootstrap_plan` validates source-bound seed positions; `bootstrap_store`
rejects plans for another source RID or group. Replacing a named container does
not authorize migrating the old workload's checkpoints.

Each binding currently creates a separate runtime/cache namespace, including
when the endpoints or runtime templates match. This avoids cached resource
references retaining another credential. These are long-lived contexts, not
new drivers per page or lease. Shared live-runtime injection is not supported.
No CFP token cache, automatic key rotation, or cross-cloud compatibility is
implied. Metadata resolution is not proof of change-feed or checkpoint permission:
the service authorizes ongoing requests, and failures remain explicit.

The prepared API does not yet provide an automatic `start`/`stop` supervisor,
instance registration, or automatic handler dispatch. Use the bounded
processing/session and bootstrap APIs below. The existing source-only
`connect` constructors remain supported; configured lease factories reject
those processors rather than falling back to their feed credentials.

```text
azure_cosmos_change_feed_processor (typed decoding)
  -> azure_cosmos_change_feed_processor_engine (raw page and continuation)
    -> azure_data_cosmos_driver (planning, routing, retries, and transport)
      -> Azure Cosmos DB
```

`ChangeFeedProcessor::connect` accepts an Azure token credential;
`connect_with_key` accepts an account key. Both resolve an existing container
without creating resources. A read-only key or a token credential with Cosmos
data-plane read permissions is sufficient.

Each `read_page` call starts from the supplied continuation, or the beginning
when none is supplied. Save the returned token only after processing the page.
`BatchState::Idle` means a range returned HTTP 304, not that the feed ended.
A non-idle page may also have zero events.
Reads consume request units. Fresh plans retain the driver's 100-partition
fan-out limit, and page size is a service hint.

```rust no_run
use azure_core::credentials::Secret;
use azure_cosmos_change_feed_processor::{ChangeFeedProcessor, ReadChangesOptions};
use serde::Deserialize;

#[derive(Deserialize)]
struct Document { id: String }

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let processor = ChangeFeedProcessor::connect_with_key(
    std::env::var("AZURE_COSMOS_ENDPOINT")?.parse()?,
    Secret::new(std::env::var("AZURE_COSMOS_KEY")?),
    &std::env::var("AZURE_COSMOS_DATABASE")?,
    &std::env::var("AZURE_COSMOS_CONTAINER")?,
).await?;
let page = processor.read_page::<Document>(&ReadChangesOptions::default()).await?;
for change in page.items() {
    if let Some(document) = change.current() {
        println!("{}", document.id);
    }
}
let options = ReadChangesOptions::default().with_continuation(page.continuation());
let _next_page = processor.read_page::<Document>(&options).await?;
# Ok(())
# }
```

Errors use `azure_core::Error`; driver failures retain Cosmos status,
sub-status, and diagnostics in the source error. Failed decoding or cancellation
does not advance hidden cursor state for this one-page API: retry using the same
options. `read_page_with_options` accepts range, mode, initial start, continuation,
session token, and operation/planning options.

## Complete event contract

`ChangeFeedItem<T>` preserves optional `current`, `previous`, and modeled
metadata. Partial LatestVersion metadata, unknown operation types, deleted-item
identity, and missing/null/empty delete images are supported. Previous images
are not guaranteed; minimal delete images require a tolerant application type.
Legacy flat documents become `current`, including documents with ordinary
application fields named `current`, `previous`, or `metadata`. An object with
only reserved envelope names remains ambiguous.

The four CFP event model types are independent of the typed Cosmos SDK, not
interchangeable Rust types. Unknown modeled fields are not retained in the typed
view, but `ChangeFeedPage::raw()` retains the complete original driver payload,
headers, status/sub-status, and diagnostics for opaque delivery or inspection.

## Bounded single-lease processing

`run_owned_lease` requires an `OwnedLease` with a source range, owner, epoch,
store revision, and confirmed recoverable checkpoint. Its checkpoint overrides
any supplied read-option continuation. The handler receives a typed page;
successful completion of its future acknowledges exactly that batch.

Batch completion additionally requires confirmed durable checkpointing before
the next read. Returning success after scheduling work elsewhere does not meet
the callback contract; await that work and propagate its outcome. Unwinding
callback panics are reported as failures without checkpointing the batch.
Aborting the whole run is observed through its task handle; aborting panics
cannot be caught. Neither case advances unfinished progress.

```text
read -> capture candidate -> deliver -> await handler -> conditional checkpoint
     -> verify receipt -> update confirmed durable progress -> next read
```

Only one batch is outstanding. There is no prefetch or checkpoint coalescing.
A handler failure retains the prior durable position. The `CheckpointStore`
seam must enforce owner, epoch, and revision conditions and confirm the exact
candidate with a new revision before returning success.

Only a `Retryable` checkpoint error that guarantees no write occurred is retried;
the successful handler is not rerun. `Ambiguous` and `Rejected` writes, invalid
receipts, or exhausted retries fail with `LeaseRunError`, including durable and
candidate positions, phase, and diagnostics. Reconcile ambiguous results with
the store before restarting. Restarting from the last checkpoint can redeliver
an event; application handlers must tolerate that possibility.

In particular, a crash between callback success and checkpoint persistence can
redeliver the batch. Lockstep prevents premature progress; it cannot atomically
commit arbitrary application side effects with the Cosmos lease checkpoint.

`LeaseControl::stop` prevents new reads/deliveries and allows bounded drain.
`lose_ownership` revokes pending completion and stops checkpoint admission.
Reports distinguish drained stop, unprocessed fetched work, drain timeout,
ownership loss, run timeout, and batch-limit completion. Interrupted persistence
may have taken effect remotely; an unconfirmed candidate is never called durable.
The generic seam accepts an external loss signal; it does not renew ownership.
The generic `run_owned_lease` seam still uses externally maintained authority.
For the Cosmos-backed path, use `CosmosLeaseStore::new_single_writer` to acquire
a `LeaseSession`, then `run_with_lease_session` for typed handling with independent
renewal/watchdog and conditional release. Inspect processing, maintenance, and
release results separately. The engine README defines the required pre-created
record schema, timing assumptions, and single-write-region account restriction.

```rust no_run
use std::num::NonZeroU32;
use azure_cosmos_change_feed_processor::{
    ChangeFeedPage, ChangeFeedProcessor, ChangeFeedReadOptions, CheckpointStore,
    LeaseControl, LeaseRunError, LeaseRunOptions, LeaseRunReport, OwnedLease,
};
use azure_data_cosmos_driver::models::ChangeFeedStartFrom;
use serde::Deserialize;

#[derive(Deserialize)]
struct Document { id: String }

async fn process(
    processor: &ChangeFeedProcessor,
    lease: OwnedLease,
    store: &mut impl CheckpointStore,
    control: &LeaseControl,
) -> Result<LeaseRunReport, LeaseRunError> {
    let read = ChangeFeedReadOptions::new(
        lease.range().clone(), ChangeFeedStartFrom::Beginning,
    );
    let policy = LeaseRunOptions::new(NonZeroU32::new(10).unwrap());
    processor.run_owned_lease(
        lease, read, control, &policy, store,
        |page: ChangeFeedPage<Document>| async move {
            for change in page.items() {
                if let Some(document) = change.current() {
                    println!("{}", document.id);
                }
            }
            Ok(())
        },
    ).await
}
```

The default run budget is 60 seconds with a 5-second drain budget and at most
three safe persistence attempts. Blocking application code cannot be forcibly
interrupted by async deadlines; handlers and stores must yield cooperatively.

## Testing

Deterministic tests exercise typed decoding, page-size hints, continuation
resumption, idle partitions, invalid tokens, and decoding failure through the
engine and real driver pipeline with an in-memory transport. Controllable
handler/checkpoint gates test ordering, retries, ownership loss, and stop/drain.
The checkpoint store in these tests is a fake, not evidence of distributed
ownership correctness against a real account.

```sh
cargo test -p azure_cosmos_change_feed_processor --all-features
```

The read-only live integration test uses an existing account and container.
Set `AZURE_COSMOS_ENDPOINT`, `AZURE_COSMOS_KEY`, `AZURE_COSMOS_DATABASE`, and
`AZURE_COSMOS_CONTAINER` in the process environment using a secure credential
source, then run:

```sh
cargo test -p azure_cosmos_change_feed_processor --all-features --test live -- --ignored --nocapture
```

The test requires existing changes, reads at most 16 discovery pages plus one
resumed page, and has a 90-second timeout. It never creates or writes resources.

## Planned processor functionality

Automatic bootstrap range/start discovery, weighted balancing, lease topology transitions, and a production
multi-worker lifecycle remain future work. Physical splits can be repaired by
the reader without increasing lease-level parallelism; parents are not deleted
and child tokens are not fabricated.

The separation follows the Java SDK's
[ChangeFeedProcessor](https://github.com/Azure/azure-sdk-for-java/blob/main/sdk/cosmos/azure-cosmos/src/main/java/com/azure/cosmos/ChangeFeedProcessor.java)
and its implementation.

See the [Cosmos SDK project documentation](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/cosmos/docs)
for project and architecture context.

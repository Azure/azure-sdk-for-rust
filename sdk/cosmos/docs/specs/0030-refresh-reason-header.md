# Refresh reason header

## Overview

The client sends `x-ms-cosmos-refresh-reason` on metadata refresh requests so the
backend can attribute refresh traffic to its cause and aggregate metrics by reason
prefix. For request-driven refreshes, the same reason is retained in the initiating
operation's diagnostics.

This specification defines shared refresh-reason values across Cosmos DB SDKs for
both Gateway mode and Direct mode, normalizing the vocabulary in the
[.NET refresh-reason proposal](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/users/kirankk/cosmos-refresh-reason/docs/design/refresh-reason-header.md).
The Rust driver uses Gateway and Gateway 2.0; there is no intent to support Direct
mode in the Rust driver. The implementation outline is Rust-specific and does not
change existing refresh or retry policies.

## HTTP header and value

```http
x-ms-cosmos-refresh-reason: server.topology.completing_split
```

These rules govern SDK-generated headers. Caller-supplied Cosmos headers are
unsupported and outside these guarantees; Rust's compatibility handling is
specified in the implementation outline.

- Send exactly one unquoted, case-sensitive ASCII value. Do not send a list,
  whitespace within the value, parameters, or encoded substrings.
- Values use lowercase dotted `snake_case`, with at most three segments. The
  grammar is `segment = [a-z][a-z0-9]*(?:_[a-z0-9]+)*` and
  `value = segment(?:\.segment){1,2}`. All currently defined values have three
  segments.
- The first segment identifies the source (`server` or `client`); the second
  identifies the aggregation family; the third identifies the cause. Aggregate
  on segment boundaries, for example `server.topology`, `client.connect`,
  `client.send`, or `client.replica`. Do not include cache names, status numbers,
  endpoints, resource IDs, exception text, or retry counts in values. The request
  resource identifies the refreshed cache separately.
- Emit only registered values from the tables below. New values require a
  specification and constant update; syntactic validity alone does not permit
  emission.
- Attach the header to the metadata requests made for a refresh, including all
  pages and transport attempts. Do not attach it to the triggering data request
  or its subsequent ordinary retry. Initial discovery, ordinary cache misses,
  cache hits, and connectivity probes have no header. A cache miss directly
  caused by refresh recovery inherits that recovery's reason.
- Presence describes the cause, not a command to refresh. Preserve existing
  force-refresh flags and conditional headers; do not introduce
  `x-ms-force-refresh` on paths that do not already use it.

**Open question: Gateway 2.0 RNTBD encoding.** Is the outer HTTP header sufficient,
or must the reason also reach the backend through an RNTBD token? Account,
container, and partition-range metadata currently use standard Gateway, so the
Rust refresh paths need only HTTP encoding today. Before enabling refresh-bearing
Gateway 2.0 operations, agree on token necessity and, if required, its ID and
string encoding with the service. The Gateway 2.0 wrapper rebuilds HTTP headers
from an allowlist; it must explicitly preserve this header if that path is used.
Do not invent a token ID or assume arbitrary headers survive wrapping.

## Known codes and attachment conditions

Every row is conditional on an actual refresh being selected. An error alone does
not cause header emission. The groups describe connection-mode applicability, not
SDK implementation coverage. Direct-mode clients also fetch account, container,
and partition-range metadata through Gateway.
Failed/timeout pairs intentionally share a code unless listed separately.

The tables below contain all wire values defined by this specification, including
all 22 non-sentinel reasons in the .NET proposal. Its `Unspecified` sentinel has no
wire equivalent: absence is `None`, and a refresh requires a reason. Do not emit
`unspecified`, an empty value, or a generic forced-refresh placeholder.

The Rust topology codes apply to both logical-key recovery and effective-key-range
split/merge recovery for query and change-feed requests. Preserve existing
container-recreation eligibility checks, session-token and continuation
restrictions, retry budgets, and rate limits. For ordinary data requests, other
410s, generic transport failures, 408s, 429s, and 5xx responses currently retry or
fail over without independently requesting a Rust metadata refresh; do not add
one to emit a code.

### Gateway mode only

| Code | Attach when |
| --- | --- |
| None currently defined. | The current Gateway metadata-refresh reasons also apply to Direct-mode clients with the same refresh behavior. |

### Gateway or Direct mode

| Code | Attach when |
| --- | --- |
| `server.topology.completing_split` | Server 410/1007 (`CompletingSplit`) triggers partition-range recovery; also retain it for an address lookup caused by that recovery. |
| `server.topology.completing_partition_migration` | Server 410/1008 (`CompletingPartitionMigration`) triggers partition-range recovery or its dependent address lookup. |
| `server.topology.name_cache_stale` | Server 410/1000 (`NameCacheStale`) triggers eligible container-name metadata recovery or a dependent routing/address lookup. |
| `server.topology.partition_key_range_gone` | Server 410/1002 (`PartitionKeyRangeGone`) triggers partition-range recovery after a split, merge, or obsolete range identity, including a dependent address lookup. |
| `server.topology.write_forbidden` | Server 403/3 selects the account-refresh effect and the refresh passes its rate limit. Exclude the hub-region discovery read branch, which rotates a partition's hub endpoint without refreshing account metadata. |
| `server.topology.account_not_found` | Server 403/1008 (`DatabaseAccountNotFound`) selects the account-refresh effect and the refresh passes its rate limit. This is distinct from 410/1008. |
| `server.topology.collection_rid_mismatch` | Server 400/1024 (`CollectionRidMismatch`) triggers eligible name-addressed container recreation recovery. |
| `server.session.not_available` | Server 404/1002 (`ReadSessionNotAvailable`) selects container recreation recovery: after exhausting session retries on the sequential path, or during eligible failed-hedge or initial-plan recovery. Ordinary session retries do not use this header. |
| `client.cache.periodic` | The background account-metadata refresh timer fires (five minutes by default). A retry of that refresh retains this code. |
| `client.cache.explicit` | A caller passes `force_refresh = true` to either public partition-range resolution method, or directly invokes the container-refresh seam without an internal trigger. Internal recovery must pass its specific reason instead. |
| `client.cache.offer_not_found` | A successful offer query returns no offer for a name-addressed container's cached RID, and `find_offer_for_container` re-reads container metadata to check for recreation. Not used for RID-addressed containers or a failed offer query. |
| `client.routing.lookup_miss` | Without an explicit force-refresh option, SDK `read_feed_ranges` gets no map or an empty range list, or `feed_range_from_partition_key` gets an empty range list, and starts its automatic forced retry. A missing map in the latter method returns an error instead. |
| `client.routing.invalid_snapshot` | A cold partition-range read produced an empty, incomplete, or overlapping map, and `build_complete_map_with_retry` starts an additional full fetch. The first cold read has no reason unless it already belongs to recovery. |
| `client.routing.incremental_merge` | Combining fetched ranges with the cached map fails or yields an incomplete map. Attach to the incremental retry and subsequent full-fetch fallback; a later invalid cold snapshot selects `client.routing.invalid_snapshot`. |
| `client.routing.pinned_chain_failed` | A partition-range fetch resumed from a cached continuation fails or returns no usable result, and recovery discards the continuation and region pin to restart cold. Ordinary retries within the pinned fetch retain its incoming reason. |

### Direct mode only (not applicable to Rust)

For transport
failures, the error causes an address refresh through the synthesized Gone path.

| Code | Attach when |
| --- | --- |
| `server.topology.gone` | A server-originated 410 with no transport cause and none of the more specific topology substatus codes causes address refresh. A client-synthesized 410 must not use this value. |
| `client.transport.unknown` | A refresh-triggering transport cause is `Unknown`, `ChannelOpenFailed`, `ChannelOpenTimeout`, `RequestTimeout`, or an unrecognized transport code. It is not a substitute for missing propagation. |
| `client.connect.dns_resolution` | `DnsResolutionFailed` or `DnsResolutionTimeout` triggers address refresh. |
| `client.connect.failed` | `ConnectFailed` or `ConnectTimeout`: the TCP connection could not be established. |
| `client.connect.tls_negotiation` | `SslNegotiationFailed` or `SslNegotiationTimeout`: TLS negotiation failed after TCP connection establishment. |
| `client.connect.rntbd_negotiation_timeout` | `TransportNegotiationTimeout`: RNTBD parameter negotiation timed out after TLS negotiation. |
| `client.connect.slot_wait_timeout` | `ChannelWaitingToOpenTimeout`: the request timed out waiting for a connection-opening slot, before starting to open its own channel. |
| `client.connection.dispatcher_closed` | `ChannelMultiplexerClosed`: the dispatcher stopped accepting requests. |
| `client.connection.broken` | `ConnectionBroken`: the selected connection was already marked unusable. |
| `client.connection.read_event` | The asynchronous connection-event listener schedules address revalidation after `ReadEof` or `ReadFailure`. This replaces .NET `ConnectionEventServerClosed`; a read failure does not prove a clean server close. |
| `client.send.failed` | `SendFailed` or `SendTimeout`: sending request bytes failed or timed out. |
| `client.send.lock_timeout` | `SendLockTimeout`: the request timed out waiting for the client send lock. |
| `client.send.write_not_sent` | The explicit write Gone-synthesis branch selects refresh because `UserRequestSent == false`, regardless of its inner transport code. Do not classify every `NotSent` request this way. |
| `client.receive.failed` | `ReceiveFailed` or `ReceiveTimeout`: receiving the response failed or timed out after sending the request. |
| `client.receive.stream_closed` | Request-path `ReceiveStreamClosed`: the peer closed the stream while the client awaited a response. An asynchronous listener-triggered refresh instead uses `client.connection.read_event`. |
| `client.replica.insufficient_quorum` | A consistency/barrier operation refreshes because the known replica set cannot satisfy its required quorum, without waiting for a Gone response. |
| `client.replica.suboptimal_set` | A server or master address set has fewer than `MaxReplicaSetSize` addresses and the suboptimal-set refresh interval has elapsed (10 minutes in the referenced proposal). |
| `client.replica.unhealthy` | On-demand address revalidation is scheduled because a cached replica URI has remained unhealthy for at least one minute. Use the connection-event code if that event, rather than this age check, schedules the refresh. |

## Implementation outline

### Rust type definition

Define `RefreshReason` in the driver's header-value models and re-export it for
diagnostics consumers. Its inner normalized string is private:

```rust
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(transparent)]
pub struct RefreshReason(Cow<'static, str>);

impl RefreshReason {
    pub const SERVER_TOPOLOGY_COMPLETING_SPLIT: Self =
        Self(Cow::Borrowed("server.topology.completing_split"));
    pub const CLIENT_SEND_FAILED: Self = Self(Cow::Borrowed("client.send.failed"));
}
```

Define an associated constant for **every** listed code, using the full value
in uppercase with dots replaced by underscores, as illustrated above. Provide
`AsRef<str>` and `Display`; `FromStr` validates the grammar and registry, returning
a typed error for invalid or unregistered values rather than normalizing arbitrary
input into a metric label. Constants borrow static strings. There is no `Default`
or `UNSPECIFIED` constant.

Use an internal refresh descriptor containing `reason: RefreshReason` and
`target: RefreshTarget`, where targets are `Account`, `Container`,
`PartitionKeyRanges`, and reserved `Addresses`. Ordinary fetches carry no
descriptor. Internal force-refresh entry points require one; public Boolean
entry points translate `true` to `CLIENT_CACHE_EXPLICIT` without changing their
signatures. Add reason-bearing SDK-internal entry points for automatic recovery
so SDK callers do not lose their more specific cause at these Boolean boundaries.

### Diagnostics context

For request-driven refreshes, add these fields to the existing canonical
diagnostics model, not a separate log:

| Location | Field | Meaning |
| --- | --- | --- |
| `RequestDiagnostics` | `refresh_reason: Option<RefreshReason>` | Reason assigned before signing/sending this metadata attempt, including attempts that fail before transmission. |
| `RequestDiagnostics` | `refresh_target: Option<RefreshTarget>` | Cache being refreshed; set together with `refresh_reason`. |
| `DiagnosticsContextBuilder` and `DiagnosticsContext` | `refresh_reason_counts: Vec<RefreshReasonCount>` | Counts keyed by `(reason, target)`, accumulated before compaction. Each entry contains `reason`, `target`, and `attempt_count: u64`. Count fetch attempts attributed to the initiating operation, not logical refresh decisions or presumed backend receipts. |

Expose read-only accessors and serialize the reason as its wire string. Preserve
the fields in detailed JSON, request summaries, deduplicated groups, compaction
keys/rollups, cloning, and sub-operation aggregation. Include the bounded
reason-count summary in summary/truncated output so compaction cannot erase all
evidence of a refresh cause. Existing sent-status, status, endpoint, and timing
fields describe the attempt's outcome.

Attribute a fetch only to the operation whose fetch factory starts the network
work. The single-flight result shared with waiters contains metadata, not fetch
diagnostics. Return successful and failed fetch diagnostics through the owning
operation's path, including failed refreshes when recovery returns the original
error. Waiters do not include that fetch's attempts, charges, or reason counts.
Cancellation of the initiating operation does not transfer ownership to a waiter.

Merge the contribution once into the initiating operation's `DiagnosticsContext`.
Preserve one owning path through PATCH helpers, hedges, plan rebuilding, and
dataflow node replacement; do not copy the contribution into sibling contexts.
Nested aggregation and compaction must preserve the owner's totals without
duplicating them. This requires neither a new fetch identifier nor cross-operation
deduplication. Being rate-limited must not fabricate a fetch attempt.

Timer-driven refreshes still send the header, but retention or delivery of their
diagnostics is outside this specification. Do not attach them to a user operation,
including one that joins a timer-initiated fetch. For request-driven refreshes,
keep collection independent of SDK emission settings and expose the additions
through existing SDK diagnostics handlers and native diagnostics JSON without
changing the existing native attempt-callback ABI.

### Driver pipeline changes

Paths below are relative to `azure_data_cosmos_driver/src` unless qualified.

| Area | Planned change |
| --- | --- |
| `driver/pipeline/retry_evaluation.rs`, `driver/routing/location_effects.rs`, `driver/routing/location_state_store.rs` | Carry the classified reason in `RefreshAccountProperties`, including hedge-leg evaluation. Pass the descriptor through effect application, rate limiting, and the account-refresh callback; create a periodic descriptor in the background loop. Keep it local to each invocation, not in shared routing state. |
| `driver/cosmos_driver.rs` account-fetch helpers | Pass the descriptor through the off-pipeline account fetch, local retries, alternate HTTP-version fallback, and regional fallback. Apply the header on every attempt. For request-driven refreshes, return diagnostics to the initiating operation on successful fallback as well as terminal failure. Initial discovery and HTTP/2 recovery probes whose metadata result is discarded remain untagged. |
| `driver/dataflow/{context,request,topology}.rs` and `driver/cache/partition_key_range_cache.rs` | Replace internal reasonless `ForceRefresh`/Boolean flow with a reason-bearing refresh request. Classify the original 410 before consuming the error. Carry it through `CachedTopologyProvider`, cache fetch closures, every change-feed page, and the three local cache-repair branches in the shared-mode table. Preserve metadata diagnostics through logical-key retry and split replacement nodes. `DriverRequestExecutor` currently ignores its refresh argument; keep refresh ownership in topology resolution and do not copy it onto the retried data request or cause a second refresh. |
| `driver/pipeline/operation_pipeline.rs`, `driver/cosmos_driver.rs`, `driver/cache/container_cache.rs`, SDK `clients/offers_client.rs` | Pass the triggering status's reason into container recreation recovery from sequential, hedged, and plan-rebuild paths. Pass the offer-miss reason from the SDK seam. Carry it into the name-based metadata read and any directly dependent routing reload, retaining generation checks and single-flight behavior. |
| SDK `clients/container_client.rs` and driver partition-range resolution entry points | Distinguish the caller's force-refresh option from the automatic retry after a missing/empty lookup. Pass `CLIENT_ROUTING_LOOKUP_MISS` through a reason-bearing internal seam for the latter, and retain refresh diagnostics for SDK dispatch. |
| `driver/pipeline/operation_pipeline.rs`, `driver/pipeline/components.rs`, `driver/transport/transport_pipeline.rs`, `models/cosmos_headers.rs` | Carry the descriptor through internal operation overrides and `TransportRequest`. Add the canonical header-name constant and one shared emission helper, also used by off-pipeline account requests. Apply the driver's reason after custom headers so it takes precedence on refresh requests. Without a descriptor, preserve existing custom-header handling; do not introduce runtime rejection or a header denylist. Populate refresh diagnostics only from the typed descriptor, not caller-supplied text. |
| `driver/transport/gateway_v2_dispatch.rs` and `driver/transport/rntbd/` | Preserve the HTTP field if refresh-bearing operations become eligible. Add binary encoding only after resolving the RNTBD open question. Do not change current Gateway 2.0 eligibility as part of this feature. |
| `diagnostics/{diagnostics_context,compaction}.rs` and metadata result plumbing | Add the fields and initiating-operation ownership described above. Surface successful metadata contexts currently discarded by fetch helpers through the owner-only path, without embedding diagnostics in shared cache results. Retain the owner's contribution across the existing split-path diagnostics aggregation gap. |

**Reason lifetime and precedence.** The site that starts a refresh selects its
reason. An explicit local repair decision (invalid snapshot, failed incremental
merge, or failed pinned chain) starts a new refresh with its own code; ordinary
pagination, throttling, transport retry, protocol fallback, and regional fallback
retain the current code. Dependent metadata reads inherit it unless they encounter
a new refresh trigger. Previous diagnostics remain immutable. A single-flight
fetch uses its initiating caller's reason and diagnostics ownership; joiners
cannot overwrite either. Do not store a mutable "last reason" on the driver,
cache entry, or shared operation state, where concurrent requests or hedges could
exchange reasons.

**Transport classification.** Where a transport failure actually selects a
refresh, classify its typed source before status flattening. A transport-synthesized
Gone is not a server response. Preserve the explicit write-not-sent classification;
otherwise classify the transport cause, using `client.transport.unknown` when its
stage is unknown. Only without a transport cause, select a specific server
status/substatus or genuine server Gone. Rust currently distinguishes DNS, connection, body-read,
and general I/O failures but does not expose all direct-mode stages. Adding those
emission sites requires preserving typed stage information through
`TransportError`/`TransportOutcome`; do not infer TLS, send-lock, or RNTBD failures
from error-message text or `RequestSentStatus` alone.

Implementation coverage must assert every registered constant's exact value and
grammar, each Rust trigger's expected header, and request-driven refresh
diagnostics. Verify absence on ordinary requests without a caller-supplied header.
Cover pagination, local cache-repair precedence, retries and fallback, periodic
header emission, competing/joined refreshes, hedges, and failed recovery. Assert
that only the initiating operation includes fetch attempts, charges, and reason
counts, exactly once across split-node replacement, nested aggregation, and
compaction; waiters include none, even if the initiator is cancelled or is a
background task. Verify custom-header pass-through without a descriptor and
driver precedence with one, without parsing custom values into refresh
diagnostics. No retry or cache policy changes are required.

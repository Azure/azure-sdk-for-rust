<!-- cspell:ignore nonapplication linearizable SQLSTATE schedulable -->

# Change Feed Processor Alternate Lease Stores

**Status:** Draft; feasibility exploration, not a backend selection
**Date:** 2026-10-12
**Authors:** Not specified

**Decision status:** No alternate backend is committed for the first release.
Azure Blob Storage and PostgreSQL are candidates to investigate, not approved
providers or implemented SDK capabilities.

**Table of Contents:**

- [1. Problem and Scope](#1-problem-and-scope)
- [2. Integration Boundary and Existing Evidence](#2-integration-boundary-and-existing-evidence)
- [3. Minimum Lease-Store Semantics](#3-minimum-lease-store-semantics)
- [4. Cosmos-Backed Comparison Baseline](#4-cosmos-backed-comparison-baseline)
- [5. Azure Blob Storage Candidate](#5-azure-blob-storage-candidate)
- [6. PostgreSQL Candidate](#6-postgresql-candidate)
- [7. Comparative Tradeoffs and Diagnostics](#7-comparative-tradeoffs-and-diagnostics)
- [8. Compatibility and Migration](#8-compatibility-and-migration)
- [9. Planned Conformance Validation](#9-planned-conformance-validation)
- [10. Recommendation and Open Requirements](#10-recommendation-and-open-requirements)
- [11. References](#11-references)

## 1. Problem and Scope

A Cosmos-backed lease container is the comparison baseline for coordinating a
Change Feed Processor (CFP). An application might prefer another store because
it already operates that service or wants separate deployment, permission, or
capacity boundaries for coordination. That preference does not establish a
correctness, cost, or availability advantage.

Lease storage is not interchangeable CRUD. Workers must agree on ownership,
reject writes from obsolete owners, preserve progress, and recover safely when
a write might have committed without returning a response. A store that offers
durable objects but cannot enforce the ownership protocol is insufficient.

This draft complements
[0030: Package Boundaries](0030-change-feed-processor-package-boundaries.md)
and [0031: Public API](0031-change-feed-processor-public-api.md).
Spec 0030 controls the package baseline; spec 0031 is canonical for proposed
Rust names and signatures. This document describes semantic operations and
possible backend mappings, not a second Rust trait definition.

**Goals:**

- Identify minimum observable guarantees for the provisional `LeaseStore` port.
- Map those guarantees to documented Blob Storage and PostgreSQL primitives,
  including uncertainty, expiry, inventory, and failover.
- Separate service facts from proposed protocols and deployment assumptions.
- Define a shared, bounded conformance exercise that could disqualify a
  candidate before provider implementation or support commitments.

**Non-goals:**

- Implementing or approving a provider, provisioning resources, or selecting
  storage packages, public API placement, or a release date.
- Supporting arbitrary stores through a CRUD adapter, automatic online backend
  switching, or exactly-once application effects.
- Changing the initial per-batch checkpoint policy, selecting checkpoint or
  renewal intervals, or adding native dead-letter handling.
- Making feed reads, callbacks, bootstrap, topology changes, and all lease
  records one distributed transaction.

Success for this exploration is a defensible primitive mapping with explicit
limits and validation requirements, not a claim that either candidate is ready.

## 2. Integration Boundary and Existing Evidence

The baseline remains:

- Reuse `azure_data_cosmos_driver` and `azure_data_cosmos_driver_native`.
  There is no new CFP driver or native artifact.
- Evaluate the public Rust surface inside `azure_data_cosmos` first.
  A companion package remains an open distribution choice.
- The caller injects a prepared, credential-bound feed resource and a prepared
  lease-store provider. An alternate provider owns its own storage client or
  connection pool, credentials, refresh, and endpoint configuration.
- Do not reuse a Cosmos access token to authorize another service. Optional
  Blob or SQL dependencies must not become mandatory shared-driver dependencies
  without a deliberate packaging decision.
- The port's declaration and re-export placement remain open. Its semantic
  contract must remain independent of typed SDK application models; the
  provider never deserializes application `T`.

For each owned lease, the consuming SDK coordinates:

```text
read batch -> deserialize -> await application callback
           -> if successful and still authorized, await conditional checkpoint
           -> confirm persistence before reading/delivering the next batch

renewal runs independently of callback execution
```

There is initially one outstanding batch per lease. A failed callback cannot
authorize a checkpoint. A successful callback followed by an ambiguous
checkpoint is a persistence problem, not evidence that the callback failed.
At-least-once delivery allows replay after application effects; it does not make
those effects transactional with any of the stores below.

**Existing evidence:** The package baseline was read at revision
`68d943e79b391858df52a7502836f2acaaf0faf9`. In the existing SDK,
`sdk/cosmos/azure_data_cosmos/src/feed/change_feed_iterator.rs` contains
`ChangeFeedPageIterator`, SDK-side typed decoding, and continuation capture.
`sdk/cosmos/azure_data_cosmos/src/options/change_feed.rs` defines
`ChangeFeedOptions` and `ChangeFeedMode`, with continuation-based resumption
distinct from a new start position. These are pull API facts, not proof of a
lease-store implementation. The mappings below are **proposed**; no service
semantics are inferred from an in-memory fake or a method name.

## 3. Minimum Lease-Store Semantics

### 3.1 Required guarantees

| Concern | Minimum semantic obligation |
| --- | --- |
| Identity and uniqueness | Scope each record to a processor/workload namespace, stable source incarnation, feed mode, and work-unit identity. A recreated source must not silently reuse old progress. Enforce one record per logical key within the authoritative store. A routing endpoint or worker name alone is insufficient identity. |
| Immutable namespace binding | As required by spec 0031 section 9.3, identify the namespace independently of the requested source/mode. Inventory must reject an existing namespace bound to another source/mode. Create must conditionally establish or validate that immutable binding before initializing work. Deriving a different empty prefix from the mismatched source/mode is not validation. |
| Complete inventory | Expose fallible pagination and an explicit end of enumeration. Preserve errors on any page, record read, or decode. A partial result, denied listing, or unknown schema is not an empty or complete inventory. State the snapshot/churn semantics rather than implying a global snapshot. |
| Create | After establishing the namespace binding, create work records only if absent, preserving existing progress on conflict. Validate the winning record's identity and schema before treating the operation as idempotent success. Binding a namespace does not prove complete bootstrap. |
| Conditional acquisition | Arbitrate competing ownership changes using a server-enforced atomic condition or exclusive lock. Bind a fresh ownership generation to a worker incarnation and publish usable authority only after the required ownership state is confirmed. Intermediate preparation is not a usable grant. |
| Renewal and expiry | Renew only the expected ownership, without changing progress. Define the authoritative expiry mechanism and conservative client stop rule. Uncertainty or an elapsed safety deadline requires quiescence, not continued work based on an old heartbeat. No timing default is selected here. |
| Checkpoint and release fencing | Bind the mutation to the expected owner incarnation, ownership generation, and applicable provider revision. Reject obsolete ownership and stale progress updates. Release must not delete progress or clear a successor's ownership. |
| Durable opaque progress | Persist the checkpoint unchanged within a declared durability/failure domain. Do not parse it as an application object, order it lexically, or confuse it with an ETag, SQL revision, or ownership proof. Reject unsupported size/encoding rather than truncating it. |
| Honest outcomes | Distinguish confirmed application, definite nonapplication, lost authority, and unknown outcome. Preserve enough operation context for safe reconciliation; a timeout alone does not prove failure. |

A read/check followed by an **unconditional** write does not satisfy this
contract. A checked record must be tied to the atomic mutation through its
revision, or checked under a lock held through the transaction's write and
commit. Refreshing a revision is not permission to adopt a different owner's
generation or overwrite newer progress.

Expiry is an ownership rule; TTL-based cleanup is only deletion policy.
Deleting expired records can also erase checkpoints and permit identity reuse.
Retention and cleanup therefore need separate rules. Fencing protects accepted
coordination writes; it cannot stop an already-running callback or undo an
external application effect after that callback's owner loses authority.

### 3.2 Illustrative record information

This is conceptual information, not a persisted schema or proposed Rust API.
Namespace binding is separate control information, not inferred from the
existence or absence of individual lease records.

| Information | Purpose |
| --- | --- |
| Immutable namespace control record | Bind the namespace to source incarnation and feed mode under a namespace-only key within the prepared store. Its existence proves neither lease inventory completeness nor topology initialization. |
| Protocol/schema version and logical key | Validate supported interpretation and exact scope/source/mode/work identity. |
| Owner incarnation and ownership generation | Distinguish a current grant from previous workers, restarts, and previous grants to the same worker. Prevent reuse of obsolete authority. |
| Provider revision | An opaque concurrency value, such as an ETag or provider-managed row revision. It is distinct from progress and ownership. |
| Opaque checkpoint | The last confirmed feed position for this work unit, with lossless storage encoding. Absence is distinct from an empty or invalid token. |
| Expiry/lease state | Native service lease state or a store-authoritative deadline and protocol state; not an application clock's unguarded assertion. |
| Operation receipt | Provider-defined logical operation identity, kind, originating owner/generation, and binding to the intended update. The provider/shared lease layer interprets it; the generic SDK does not parse opaque metadata. Checkpoint receipts must survive unrelated renewal and release updates. |
| Membership/topology association | Enough information to reject incompatible work-unit mappings and distinguish incomplete bootstrap from an initialized workload. Its detailed transition protocol remains open. |

A native service lease ID may be part of an ownership proof; it is not itself
the CFP record, the checkpoint, or a general fencing token for application
side effects. Provider-generated revisions/proofs remain opaque outside the
provider contract.

All candidates assume protocol-compliant writers and controlled administration.
Credentials that can overwrite records or break native locks are not made
harmless by this protocol.

### 3.3 Results, uncertainty, and transaction boundaries

| Observation | Required interpretation |
| --- | --- |
| Confirmed conditional write/commit | The intended mutation was accepted within the supported durability domain. This alone does not prove that ownership is still current when the response arrives. |
| Definitive conditional rejection or confirmed rollback | That attempt did not apply. Determine whether this is a revision conflict, lost ownership, expiry, or another failure before retrying. A rejected retry does not prove an earlier timed-out attempt failed. |
| Request/commit response lost, cancellation, or transport failure after submission | The outcome may be unknown. Retain the logical operation identity and original authority; reconcile or quiesce. Do not advance to another batch or retry an unconditional write. |
| Authorization, schema, or inventory failure | Surface a failure with context. Do not create a replacement namespace, reset progress, or silently fall back to another provider. |

**Recovery boundary — proposed, with the public carrier open:** Spec 0031 sketches
`checkpoint(OwnedLease, FeedCheckpoint)` and `inspect` with opaque provider
metadata; it does not finalize a public operation-ID or reconciliation carrier.
For one logical checkpoint call, the provider, or its shared lease adapter,
creates and retains stable operation identity before the first storage attempt
and preserves it across internal retries and receipt inspection. That layer
interprets backend-specific receipts and reports semantic outcomes. The generic
SDK coordinates callbacks, awaits outcomes, and stops unsafe work; it must not
parse opaque receipt blobs or derive authority from them.

How pending identity remains addressable after an ambiguous return or
cancellation, and how a later `inspect` call is correlated with that operation,
remain **Open** in spec 0031, including any opaque carrier/helper. A new public
checkpoint call must not be assumed to resume the same logical operation.
Without retained context or a defined recovery helper, safe quiescence is valid;
`inspect` alone is not automatically proof of a particular write. These mappings
do not claim that the current API sketch already exposes receipt-aware recovery.

Outcome classification covers all attempts in that logical operation. The
provider/shared lease layer must not replace an earlier unresolved write outcome
with a definite failure merely because the final retry was rejected.

**Proposed recovery rule:** Atomically persist a checkpoint receipt with its
progress update. Provider interpretation of an authoritative read finding the
matching receipt can prove that historical update, but cannot restore a previous
grant or grant the successor's authority. A missing receipt is not proof of
nonapplication while an earlier request can still complete. Retries must retain
conditional guards and operation identity, or wait until the old attempt can no
longer mutate the record.

A later owner might replace the receipt. A bounded single-record receipt is
therefore not an unlimited operation history. If reconciliation cannot establish
the result, stop using that grant; recover through fresh acquisition and the
current stored progress rather than guessing or rewinding it. Receipt retention
and its public outcome representation remain open.

Caller cancellation is not rollback. Shutdown must account for submitted writes
whose results are unknown, even when the caller has stopped awaiting them.
Renewal cannot be blocked for the duration of an application callback, and
renewal updates must never replay an old whole-record checkpoint snapshot.

Immutable namespace binding, complete inventory, idempotent creation of missing
records, and initialization completion are separate steps. An unbound namespace
is not initialized; a successfully bound namespace can still have an incomplete
set of work records. Likewise, parent/child work-unit transitions need an
explicit recoverable membership protocol; neither namespace binding nor
per-record CAS provides a global transaction with source topology discovery.

## 4. Cosmos-Backed Comparison Baseline

The proposed Cosmos provider uses a prepared Cosmos resource and per-item
conditional mutation. Cosmos documents `_etag`/`If-Match` checks and rejection
of an outdated revision. A provider can validate the owner/generation in a read
record and condition its mutation on that exact record revision. A successful
conditional renewal must retain progress; a checkpoint must retain the intended
ownership binding. These are provider obligations, not automatic properties of
an ordinary item replacement. [C1][cosmos-occ]

Cosmos transactions within a logical partition can support bounded multi-item
work, but do not automatically make a whole lease inventory, source-container
discovery, or records across partitions/accounts one transaction. The exact
expiry, receipt, and bootstrap protocols still need specification.

Account topology matters. The documentation distinguishes single-region writes
from multi-region writes that can accept a locally matching ETag before conflict
resolution and global replication. ETag support alone is not a proof of one
global owner across arbitrary multi-write configurations. Read consistency also
depends on the selected account/request consistency; independent clients do
not automatically share a session's observations. [C1][cosmos-occ]
[C2][cosmos-consistency]

**Assumption to validate:** A supported lease-account write topology and recovery
read policy provide the authoritative conditional updates required in section 3.
Single-write operation is a narrower starting point for that analysis, not a
claim that every Cosmos account is globally linearizable. The feed resource and
lease resource may use different accounts, identities, and consistency choices.

## 5. Azure Blob Storage Candidate

### 5.1 Documented primitives and proposed profile

Blob Storage offers both ETag conditions and native blob leases. A native lease
protects write/delete operations on one blob; ordinary reads need not present
its ID. `Lease Blob` supports acquire, renew, release, change, and break.
Finite durations are **15 to 60 seconds**; infinite leases are also supported.
Renewal resets the native duration and can succeed after expiry if the blob has
not been modified or leased again. [B1][blob-lease]

Crucially, **lease operations do not change the blob's ETag or Last-Modified
property**. The native lease ID gates access to a currently leased blob; an ETag
conditions the version of its content/properties. Neither value substitutes for
the other. A lease acquisition does not atomically write a CFP owner or
checkpoint. [B1][blob-lease] [B2][blob-conditions]

**Proposed feasibility profile:** An immutable namespace control blob plus one
ordinary block blob per logical lease in a prepared container. Each lease blob
uses a finite native lease and the section 3 envelope in its body. Every new
ownership attempt uses a fresh proposed native lease ID and a fresh CFP
generation. Use single-request conditional `Put Blob` for the small records,
not a multipart upload protocol assumed to be equivalent.

The control blob's key depends only on the namespace within that container,
not on the requested source/mode. Before creating any work record, use
`Put Blob` with `If-None-Match: *` to establish its source/mode binding; on
conflict or an uncertain response, read that same primary control blob and
validate the binding. Mismatch is an error. Do not initialize work until the
binding is confirmed. Inventory reads this control blob before enumerating the
namespace's separate lease prefix; a different source/mode cannot select a
different empty namespace. [B2][blob-conditions] [B3][blob-put]

Normal protocol operations never delete or rebind the control blob. Unexpected
lease records without it fail closed; administrative deletion/restore requires
the fenced recovery discussed below. Control-blob creation and individual lease
creation are separate conditional writes, not a multi-blob transaction. A crash
after binding but before all lease creates leaves bootstrap incomplete.

### 5.2 Semantic operation mapping

| Operation | Proposed mapping and atomic boundary |
| --- | --- |
| Create | First conditionally establish/validate the namespace control binding. Then `Put Blob` with `If-None-Match: *` creates an initial unowned lease envelope only if absent. Validate conflicts without overwriting progress. These writes and native acquisition are separate operations. [B2][blob-conditions] [B3][blob-put] |
| List | Read/validate the namespace control binding on the primary, rejecting a source/mode mismatch. Then `List Blobs` over its lease prefix, without a delimiter hiding descendants, following all markers and validating bodies. Neither binding nor listing properties grants ownership or proves bootstrap complete. [B4][blob-list] |
| Acquire | Read/validate the candidate, then `Lease Blob: acquire` with a fresh proposed ID and an `If-Match` condition on the observed ETag. After winning, read using that lease ID and conditionally `Put Blob` with both the lease ID and current ETag to bind the owner/generation, preserving progress and receipts. Do not publish a usable CFP grant until that binding is confirmed and native validity is still safely established. [B1][blob-lease] [B2][blob-conditions] |
| Renew | `Lease Blob: renew` with the grant's native ID, independently of the callback. It leaves the stored checkpoint and ETag unchanged. Coordinate local grant retirement with renewal; a successful native renewal must not resurrect a grant already treated as lost. [B1][blob-lease] |
| Checkpoint | Validate active owner/generation in the record and replace its body using **both** `x-ms-lease-id` and `If-Match`. The single `Put Blob` stores progress and its receipt together; retain other envelope fields and any required metadata. A stale read cannot authorize a later unconditional replacement. [B1][blob-lease] [B3][blob-put] |
| Release | First stop admitting work for the grant and resolve outstanding mutations. Validate its active owner/generation, then conditionally mark it released using its native ID and ETag, retaining progress/receipt. Then `Lease Blob: release` with the same ID and the marker's ETag. These are two operations, not one transaction. If the second is uncertain, remain quiescent rather than claiming confirmed native unlock. [B1][blob-lease] [B2][blob-conditions] |
| Recovery (provider-internal) | The provider reads the primary blob and interprets the original operation's receipt/binding using retained context. Verify native authority separately. Historical confirmation, an unchanged ETag, or a list result is not sufficient authority. The public recovery carrier/helper remains open as described in section 3.3. |

A crash between native acquisition and CFP binding temporarily holds a native
lock without a usable CFP owner. A crash after the release marker but before
native release leaves a released CFP record temporarily locked. Finite expiry
can restore availability in either case; it does not complete the missing
record transition. Other workers must acquire and establish their own binding.
The conditional binding write is the proposed CFP acquisition's logical commit
point; native acquisition alone is only preparation.

### 5.3 Expiry and ambiguous outcomes

The provider must use a conservative elapsed-time budget for native validity,
including request/retry time and scheduling delays. It must not infer the
remaining duration from `Last-Modified`, an unchanged ETag, or a successful
listing. No safety margin or renewal interval is selected here.

Although the service permits some expired-lease renewals, the **proposed CFP
rule** retires a grant once its safe validity can no longer be established.
Recovery then requires a fresh native acquisition/CFP generation and rereading
progress. This prevents a late success from silently reviving abandoned work.
The service also documents a possible acquisition delay after natural expiry;
expiry is not a promise of instantaneous handoff. [B1][blob-lease]

Use `x-ms-client-request-id` and returned `x-ms-request-id` for diagnostics.
Their documented role is correlation, **not** durable deduplication or an
operation-result lookup API. A logical receipt stored with the body is the
proposed checkpoint reconciliation mechanism inside the provider/shared lease
layer, subject to section 3.3's open context/carrier boundary. A generic SDK
does not decode this body to reconcile its public `checkpoint` call.
[B1][blob-lease] [B3][blob-put]

- **Uncertain create/binding/checkpoint:** Read the primary record. A matching
  receipt can establish the particular write; otherwise preserve uncertainty.
  An absent receipt cannot disprove an in-flight write. Any retry retains its
  original conditional base, ownership binding, and logical operation identity.
- **Uncertain acquisition:** Retain the proposed lease ID. Carefully bounded
  native operations using that ID can help reconcile the original attempt,
  but reacquiring with the same ID after expiry could create another native
  lease. Do not interpret that as proof of uninterrupted CFP ownership.
- **Uncertain renewal:** A read showing a leased blob does not prove the renewed
  deadline. Obtain sufficient confirmation before the conservative stop
  boundary, or retire the grant.
- **Uncertain release:** A confirmed released marker prevents further CFP work
  by that grant but does not prove native release succeeded. A successor's
  native ID/record must never be adopted to finish the old release.
- **Historical commit after ownership loss:** The old checkpoint may have
  committed before a successor acquired. Report that history if provable,
  without returning the successor's proof or resuming the old callback loop.

Checkpoint writes and retries must reject an inactive or mismatched generation,
including after reading a newer ETag. The native lease and record protocol work
together; a blob lease alone does not implement CFP semantics.

### 5.4 Inventory, consistency, permissions, and limits

**Inventory.** `List Blobs` returns up to 5,000 entries per page and can return
fewer while still returning a continuation marker. Follow markers until
`NextMarker` is empty; do not use page length as completion. Enumerate current
committed lease blobs, not snapshots, versions, or deleted entries as additional
leases. The body is not part of a normal listing, so envelope validation can
require further reads. Do not substitute tag-index queries for complete
inventory: that secondary index is documented as eventually consistent.
[B4][blob-list]

Blob Storage documents strong read/list consistency after writes. This does
not provide a conditional, point-in-time transaction spanning a paginated scan,
subsequent body reads, and concurrent topology changes. The listing API has no
conditional-header operation in the documented table. Bootstrap still needs
membership reconciliation; mutations during a scan cannot be hidden under an
unsupported global-snapshot claim. [B2][blob-conditions]
[B5][blob-concurrency]

**Read authority.** Use the primary endpoint for coordination and reconciliation.
Read-access geo-redundant secondaries are asynchronously replicated and can lag.
An SDK's secondary-read fallback must not turn stale data into authority or an
apparent missing record. [B6][blob-redundancy]

**Account failover.** Local/zone redundancy and geo-replication protect different
failure domains. Unplanned account failover can lose writes not yet replicated;
planned failover has different documented preconditions and data-loss
expectations. An acknowledged checkpoint or newer owner record must not be
assumed to survive every failover. Last Sync Time helps assess possible loss;
it is not a per-operation receipt or proof that a native lease remains valid.
[B6][blob-redundancy] [B7][blob-failover]

**Open service verification:** Native lease-state/ID behavior across the selected
account failover must be established separately from blob-content durability.
Until the supported behavior is proven, quiesce on an unsupported failover and
require a fenced recovery procedure. A generation stored only in a rolled-back
blob cannot by itself prevent old authority from reappearing. Automatic
continuation through potentially lossy failover is unsuitable if the contract
requires no loss of acknowledged coordination state.

**Authentication and administration.** The prepared provider owns Storage
authentication, token refresh, and client lifetime. It needs data read/list and
write/lease permissions at the correct scope. `Lease Blob` documents blob-write
permission and Storage Blob Data Contributor; listing has separate read/list
requirements, including SAS list permission. Metadata resolution alone is not
an authorization check for all later operations. [B1][blob-lease]
[B4][blob-list]

Any suitably authorized caller can break a native lease without its ID, and a
blob lease does not prevent deleting its container. Restrict normal provider
behavior to the protocol and treat administrative break/delete/restore as
disruptive recovery actions. Also verify that retention, immutability, lifecycle
rules, account namespace features, and payload limits permit the proposed
mutable-record protocol; this draft does not certify every account feature.

## 6. PostgreSQL Candidate

### 6.1 Documented primitives and proposed profile

PostgreSQL offers unique constraints, conditional `UPDATE`, row locks, and
transactions. At Read Committed, an updater waiting behind another updater
rechecks its predicate against the updated row. `SELECT ... FOR UPDATE` blocks
conflicting writers until transaction end; the check and mutation must remain
inside that transaction. `UPDATE ... RETURNING` reports updated rows, but is
not confirmation of a later explicit transaction commit.
[P1][pg-isolation] [P2][pg-locks] [P4][pg-update] [P6][pg-commit]

**Proposed feasibility profile:** A namespace table keyed uniquely by namespace,
independently of source/mode, plus a logged lease table unique by namespace/work
unit. Lease records carry ownership generation/revision, server-clock expiry,
opaque progress, and receipts. Use one authoritative writable primary and short
per-record transactions. PostgreSQL 18 documentation grounds this discussion;
supported server versions, managed offerings, HA configurations, SQL client, and
pool are not selected.

For create, a short Read Committed transaction first uses
`INSERT ... ON CONFLICT DO NOTHING` on the namespace key, then reads/locks the
winning namespace row and validates its immutable source/mode binding. A
mismatch aborts before inserting work. Insert the individual lease record only
after validation, then commit; normal writers never update/delete that binding.
This can make binding validation and one lease create atomic within the
database, not the complete bootstrap or Cosmos source topology.
[P1][pg-isolation] [P2][pg-locks] [P3][pg-insert]

Inventory reads the namespace row by namespace alone and validates source/mode
in its enumeration snapshot before selecting lease rows. Absence is unbound
state, not initialization success; mismatched bindings are errors even when
no lease rows match. A query filtered only by the full source/mode tuple would
incorrectly turn the mismatch into an empty result.

### 6.2 Semantic operation mapping

| Operation | Proposed mapping and atomic boundary |
| --- | --- |
| Create | Conditionally establish/validate the namespace binding first, then `INSERT ... ON CONFLICT DO NOTHING` against namespace/work-unit uniqueness and validate the winning lease row. Never reset ownership/progress with an unconditional upsert. Confirm after transaction commit. [P3][pg-insert] |
| List | In a read-only Repeatable Read transaction, first validate the namespace binding, then enumerate deterministically in that same snapshot. Retain the connection across pages, bound its lifetime, and fail on cursor/transaction errors. Independent Read Committed pages require churn reconciliation. Neither approach atomically includes Cosmos topology or proves bootstrap complete. [P1][pg-isolation] |
| Acquire | In a short transaction, lock the exact row, validate scope/schema, sample server time after obtaining the lock, and check that it is unowned or expired. Update to a fresh owner/generation, expiry, revision, and coordination receipt, preserving progress/checkpoint receipt; commit before exposing the grant. Contenders serialize on that row. [P1][pg-isolation] [P2][pg-locks] |
| Renew | Lock/check the expected active owner/generation/revision and unexpired grant; update expiry, coordination revision, and coordination receipt. Preserve checkpoint bytes and checkpoint receipt. Commit and return the updated coordination state. |
| Checkpoint | In the same short transaction as the authority check, conditionally update progress and its receipt and advance the revision, checking expected owner/generation, current validity, and the applicable revision/progress base. Commit before confirming persistence. Another transaction cannot change ownership between the locked check and commit. |
| Release | Under the same authority/revision guards, mark the grant released or clear its owner and advance the revision with a coordination receipt. Preserve progress/checkpoint receipt. Unlike Blob, this needs no second native-unlock request. Commit releases the temporary SQL row lock, not a long-lived native CFP lease. |
| Recovery (provider-internal) | On the authoritative primary, the provider interprets receipts using the original logical operation context. A locked recheck or guarded retry accounts for an in-progress original transaction. Historical confirmation grants no newer ownership; absent recovery context/carrier, remain unknown and quiesce. |

An equivalent single conditional `UPDATE` with `WHERE` guards and `RETURNING`
may reduce round trips, but its time evaluation and retry behavior must be
verified, not inferred from the SQL's appearance. Zero updated rows is not
successful acquisition. With explicit transactions, inspect commit outcome,
not just affected-row count.

No transaction or checked-out connection may be held for the duration of an
application callback. Independent renewal can briefly contend with checkpoints
on the row, but must not wait on callback completion. A provider that advances
the revision during renewal must reconcile a checkpoint's stale revision only
under unchanged ownership and the expected progress base, never by overwriting
the entire row from an older snapshot.

`SKIP LOCKED` deliberately omits locked rows and is therefore not a complete
inventory strategy. Session/advisory locks alone also do not provide the durable
owner, expiry, progress, and restart semantics required here.
[P11][pg-select] [P2][pg-locks]

### 6.3 Time, cancellation, and unknown commits

Use database time for expiry comparisons, not an arbitrary worker's wall clock.
`CURRENT_TIMESTAMP`/`now()` return transaction-start time;
`statement_timestamp()` returns statement-start time. Both can predate a long
lock wait. `clock_timestamp()` advances within a statement. The proposed locked
protocol samples it **after lock acquisition**, immediately before deciding and
updating expiry. Server wall time is still not a promised monotonic clock:
clock adjustment and primary replacement require deployment-specific analysis.
[P5][pg-time]

Expiry is checked at the serialized mutation decision. It does not abort a
transaction that already passed that check. Locks prevent a new owner from
committing a conflicting transition until the transaction ends. A delayed
successful response must still be rejected as usable authority if its validity
budget has elapsed or ownership has since changed. Bound transactions and lock
waits; no lease-duration or clock-skew allowance is selected here.

PostgreSQL documents that dispatching cancellation does not guarantee any effect:
the server might already have completed the command. Cancellation of an async
client future is an additional client-library behavior requiring verification.
A confirmed abort/rollback establishes nonapplication only for that transaction,
not for earlier uncertain attempts in the logical operation. A generic query
error or loss of the connection around `COMMIT` does not establish rollback.
The pool must not return a connection with an unknown transaction/protocol state
for unrelated reuse. [P7][pg-cancel]

**Proposed receipt strategy:** The provider/shared lease layer stores the
checkpoint and operation receipt in the same transaction. After a lost commit
response, that layer reads the authoritative primary and interprets the receipt
against its retained original operation identity/binding; the generic SDK does
not parse SQL provider metadata. Context lifetime and a public inspection/helper
carrier remain open as described in section 3.3. If a retry waits for an original
transaction, reevaluate the receipt and guards afterward. Neither a pre-commit
row read nor rejection of the final retry disproves an earlier uncertain commit.
If evidence is superseded or the context is unavailable, remain uncertain and
abandon the old grant rather than using the successor's proof.

For acquisition, renewal, and release, the proposed coordination receipt is
written in the same transaction as that transition, independently of the
checkpoint receipt. Match the original operation, owner/generation, and resulting
coordination state when reconciling; a successful renewal receipt still needs
a currently valid deadline, and a historical acquisition receipt cannot revive
released ownership. An uncertain create can be reconciled by a validated row at
the unique key once the pending transaction is resolved. A failed inventory
transaction must restart enumeration rather than treating fetched pages as
complete. Receipt history remains bounded, so superseded evidence can still
require quiescence.

### 6.4 Durability, failover, and provider ownership

SQL transactions alone do not define the failover guarantee. The deployment
must retain acknowledged ownership changes and checkpoints in its supported
failure domain and ensure there is only one writable authority.

- `fsync` and durable WAL behavior matter. `synchronous_commit = off` can lose
  acknowledged transactions after a crash. With no synchronous standbys,
  `synchronous_commit = on` confirms local durability, not replication to every
  possible failover target. [P8][pg-wal]
- With configured synchronous standbys, `on`, `remote_write`, and
  `remote_apply` have different guarantees. In particular, `remote_write` does
  not require standby disk flush. Promotion must choose a replica that satisfies
  the required acknowledgement policy, not an arbitrary lagging reader.
  [P8][pg-wal]
- PostgreSQL explicitly requires fencing the old primary to prevent two servers
  believing they are primary. Row locks cannot coordinate two independent
  writable histories. Reconnect logic and a generation counter cannot repair
  lost history or unfenced promotion. [P9][pg-failover]
- Restore, failback, clock changes, and asynchronous promotion can invalidate
  ownership/progress assumptions. Either exclude those transitions from
  automatic operation or define externally fenced recovery and fresh grants.
  Do not claim identical guarantees for self-managed, managed HA, and read-replica
  deployments merely because all speak PostgreSQL.

The prepared provider owns its pool/client, database credentials or
service-specific authentication, TLS policy, and refresh/reconnect behavior.
It must preserve application-owned shared resources on processor shutdown.
Runtime privileges need the required read/insert/update/locking operations;
schema administration should be separate. Row-level security can silently hide
records, so authorization must demonstrably cover the entire intended workload
inventory without granting unrelated workloads access. [P3][pg-insert]
[P4][pg-update] [P10][pg-row-security]

**Open deployment verification:** SQL library cancellation/retry behavior, pool
ownership, effective transaction settings, clock behavior, complete visibility,
and the chosen HA topology must be established together. A locally correct SQL
test is not a certification of those operational guarantees.

## 7. Comparative Tradeoffs and Diagnostics

These are qualitative consequences of the proposed mappings, not measured
benchmarks, prices, latency targets, or support promises.

| Dimension | Cosmos-backed baseline | Blob Storage candidate | PostgreSQL candidate |
| --- | --- | --- | --- |
| Correctness prerequisites | Conditional item revisions plus an explicit owner/expiry protocol and supported account write/read topology. | Native ID and ETag have different roles; durable CFP binding and native lease transitions are separate. Finite lease timing must fit the host. | Unique key, guarded row transactions, safe server-clock expiry, durable commits, and fenced primary promotion. |
| Atomic unit | Conditional item mutation; bounded logical-partition transactions where applicable. | One conditional blob replacement or one native lease operation, not both together or an inventory-wide transaction. | A transaction can combine record fields and receipts, potentially several rows in that database; not Cosmos feed reads or arbitrary callback effects. |
| Writes | Checkpoints and renewals consume lease-container throughput; contention adds retries. | Body replacement for checkpoints plus native renewals; acquisition/release require extra protocol steps. Separate calls, retries, and optional version retention add work. | Row updates generate WAL and replication work; locks, indexes, pool limits, and vacuum behavior affect sustained coordination load. |
| Reads and inventory | Point reads and complete paginated queries consume throughput; partitioning/indexing affect scans. | Namespace-control read, prefix-list pages, and possible per-blob body reads; no general body-field query. Repeated inventories across workers amplify request volume. | Namespace validation and indexed scoped scans are possible; snapshot pagination holds a connection/transaction and can retain old row versions. [P12][pg-vacuum] |
| Dependencies and deployment | Reuse Cosmos client/authentication machinery, though the lease container/account remains a separately prepared resource. | Additional Storage client/authentication and account policy surface. Avoid making its SDK a mandatory driver dependency. | SQL driver, TLS/authentication, pool/runtime integration, schema management, and a separately operated database/HA policy. |
| Recovery and support burden | Explain account consistency, write topology, conditional failures, throttling, and receipt recovery. | Explain native versus CFP lease state, two-step transitions, short finite lease limits, primary-only coordination, and failover loss. | Explain SQLSTATE/commit phase, contention, connection health, clock/replication configuration, promotion fencing, and schema upgrades. |
| Compatibility | No assumed compatibility with another SDK's Cosmos lease schema or opaque continuation format. | A new physical schema; native IDs/ETags are not portable ownership. | A new schema and privileges; database migrations and mixed client versions become support obligations. |

One checkpoint per successful batch and independent renewal remain the shared
policy. Reducing checkpoint frequency is a separate replay/cost tradeoff, not a
shortcut for making one provider appear cheaper.

**Proposed diagnostics:** Preserve operation kind, provider, sanitized workload
identity, attempt correlation, timing, condition failure, lost ownership,
inventory completeness, and unknown-outcome/reconciliation state. Include
Cosmos status/substatus and charge when available, Blob status/service error
and request IDs, or PostgreSQL SQLSTATE and transaction phase as applicable.
Distinguish callback errors from store errors. Do not log checkpoint contents,
credentials, native lease IDs, or ownership proofs by default, or use per-lease
identifiers as unbounded metric labels.

## 8. Compatibility and Migration

**Same-provider rolling upgrades** require compatible record interpretation,
conditional predicates, expiry rules, receipt preservation, and continuation
decoding. A newer writer must not drop a field needed by an older supported
worker. Unknown required schema/protocol versions must fail closed, not be
skipped as unrelated records. Unsupported version mixtures require a coordinated
stop; permissive JSON decoding alone is not protocol compatibility.

**Changing providers creates a new authority domain.** A Cosmos worker and a
Blob or PostgreSQL worker can each acquire their own record for the same feed.
Matching processor names, copied checkpoints, or dual writes do not make those
independent locks exclude each other. Do not permit uncoordinated online switching
or automatic fallback from one store to another.

**Proposed offline migration discipline, if migration is later authorized:**

1. Stop admission across all old workers and drain callbacks/checkpoints where
   possible. Reconcile submitted writes or explicitly retain their uncertainty.
   Fence all old writers and restarts through deployment/access controls; a
   local process stop or observation of expired records alone is insufficient.
2. Take a complete validated inventory after the old authority is quiescent,
   including work-unit membership and confirmed progress. Establish the source,
   mode, schema, and token compatibility before copying.
3. Conditionally establish/validate the target namespace's immutable source/mode
   binding, then create work records idempotently as unowned records. Copy
   compatible opaque progress losslessly, not Cosmos `_etag` values, old owner
   proofs, native IDs, or backend revisions. Establish fresh target generations
   and inventory completion separately, without claiming a cross-store transaction.
4. Switch every eligible worker to the target only after completeness and old
   writer fencing are verified. Reject mixed-provider configurations. An epoch
   stored solely in the target cannot fence an old worker that never consults it.
5. Treat rollback as another coordinated migration. Do not resume the old store
   at its now-stale checkpoints while target workers remain active.

The exact migration artifact, operational fencing mechanism, and compatibility
matrix are open. This draft supplies no migration tool or compatibility promise
with existing Cosmos CFP implementations. Tokens whose source/mode or decoder
compatibility cannot be established must not be translated by guessing, and
progress loss must not silently reset the feed start.

## 9. Planned Conformance Validation

**Planned only; none of these provider tests or live workloads were run.**

Use the same observable contract for the Cosmos baseline and both candidates.
Controlled boundaries can validate SDK scheduling, error propagation, receipt
logic, and stop rules. They cannot establish a service's conditional-write,
lease, durability, authorization, or failover guarantees.

| Scenario | Observable assertion | Required evidence |
| --- | --- | --- |
| Namespace binding race and mismatch | Two clients use the same namespace with different sources/modes: only one binding wins; mismatched create initializes no work and mismatched inventory fails even with no lease records. A crash after binding does not imply bootstrap completion. | Controlled boundaries plus real conditional control-blob creation or SQL namespace uniqueness. |
| Two independent clients acquire one logical lease | Exactly one current grant is accepted at the arbitration boundary; the loser cannot checkpoint. Repeat with distinct client instances/process incarnations. | Controlled scheduling plus the real backend's concurrency primitive. |
| Stale owner checkpoint/release after takeover | Both mutations are rejected; successor ownership and progress remain unchanged. Include delayed original requests and a reused worker display name. | Controlled delays plus real backend guards. |
| Renewal concurrent with a slow callback/checkpoint | Renewal remains schedulable and never restores stale progress or overwrites a checkpoint receipt. Local revision conflicts do not adopt new ownership. | Controlled SDK boundaries plus real record mutation semantics. |
| Callback failure or cancellation before success | No checkpoint is issued for that batch; application effects are not claimed rolled back. | SDK-level controlled callbacks and a recording store boundary. |
| Commit followed by response loss | Provider-owned receipt inspection confirms the original write or remains unknown; a rejected final retry cannot erase earlier uncertainty. Do not deliver the next batch unconfirmed or return successor authority after historical confirmation. | Controlled response loss after a real backend write/commit. |
| Opaque recovery boundary | The provider/shared lease layer retains stable identity across internal attempts and interprets receipts. The generic SDK does not parse metadata. If context cannot be correlated through the eventual public helper/carrier, report unknown and quiesce rather than inventing a successful reconciliation. | Controlled provider/API boundaries; the carrier/helper is still open, not an implemented interface. |
| Partial inventory/bootstrap failure | Failure on a later page, record decode/read, or create is not empty inventory or initialization success. Do not skip locked SQL rows or stop at a short Blob page with a marker. | Controlled page/create failures plus real listing/isolation behavior. |
| Expiry, process pause, cancellation, and restart | Stop at the conservative validity boundary. Restart uses a fresh incarnation and current persisted progress, not a saved proof. A late response cannot revive a retired grant. | Controlled clocks/process boundaries plus real expiry/commit semantics. |
| Blob two-step transitions and lease metadata | Acquisition without binding authorizes no callbacks; release marker without unlock remains quiescent. Lease calls do not serve as ETag/Last-Modified heartbeat updates. Exercise expired renewal and proposed-ID reuse. | Real Blob service with controlled request/response boundaries; a fake cannot certify this. |
| SQL lock wait, expiry, and commit boundaries | Expiry is evaluated after the lock wait; callback execution holds no lease-row transaction. Cancel/connection loss around commit is not inferred rollback. | Real PostgreSQL concurrency plus the selected client/pool. |
| Permissions and inventory visibility | Read success does not mask denied renewal/checkpoint/list. Restricted SQL row visibility cannot certify complete scope inventory. Authentication refresh failure stops unsafe work. | Real service roles/policies and selected authentication clients. |
| Backend failover or restore | Unsupported or lossy transitions quiesce; supported promotion retains required state and rejects obsolete proofs. Test old-primary fencing and reacquisition, not just reconnection. | The actual deployment's failover/durability semantics, not recorded responses or a single-node fake. |
| Schema, source/mode, version, and provider mismatch | Refuse incompatible records/configurations without resetting or translating opaque progress. Demonstrate why independent stores permit independent owners unless migration fences the old population. | Controlled compatibility cases plus real independent stores for the authority-domain claim. |

A real local PostgreSQL instance can exercise SQL isolation and locking, but not
certify a managed HA topology. Likewise, emulator or recorded Blob/Cosmos
responses are useful for adapter behavior, not sufficient evidence of live
timing, concurrent arbitration, permissions, or failover durability.

## 10. Recommendation and Open Requirements

**Recommendation, not approval:** Resolve the supported durability/failover
domain before choosing an alternate store. If a primary-authoritative,
finite-lease Blob profile fits the requirements, a narrow next feasibility
exercise should challenge its two-step acquire/release protocol, expired-lease
behavior, and response-loss reconciliation with two independent clients.
That exercise should either demonstrate the contract or identify a blocking
requirement; it is not a provider rollout.

PostgreSQL is a useful contrast where an application already operates a suitable
database: record transitions and receipts fit a transaction, but the operational
contract is larger. Any follow-up must name a server/client/HA profile rather
than claiming support for all PostgreSQL deployments.

| Open requirement or verification | Why it gates feasibility |
| --- | --- |
| Failure domain and acknowledged-state loss | Determine which outages/failovers must retain ownership and progress, and when operator-fenced recovery is acceptable. Lossy failover cannot silently satisfy a no-loss contract. |
| Blob service/client verification | Validate combined lease-ID/ETag conditions, two-step crash windows, proposed-ID retries, expired renewal, primary-only reads, and native lease state through the selected account failover. Pin a supported REST/client profile. |
| PostgreSQL deployment/client verification | Validate after-lock server-time checks, clock changes, effective WAL/commit settings, promotion target and old-primary fencing, permissions, cancellation, and pool cleanup. |
| Receipt and outcome contract | Set provider/shared lease context retention and any public opaque carrier/helper linking later inspection to an uncertain operation. Keep backend receipt interpretation out of the generic SDK, aggregate all attempt outcomes, and separate historical confirmation from usable authority. Spec 0031 remains canonical. |
| Inventory/bootstrap/topology protocol | Immutable namespace binding is required, but does not prove completeness. Define membership completeness and recover interrupted parent/child transitions without requiring a global transaction that some providers cannot supply. |
| Identity and persisted-format compatibility | Set source-incarnation rules, token compatibility, schema/version negotiation, record-size limits, and prevention of obsolete generation reuse after restore or migration. |
| Package, port, and resource lifetime | Keep spec 0031 canonical for Rust signatures; settle declaration/re-exports, native integration, optional dependency placement, shared-client lifetime, and support ownership. No new driver/native artifact is needed. |
| Timing and capacity | Establish acceptable validity budgets, retry/stop rules, renewal isolation, inventory memory/connection bounds, and workload evidence before making performance claims. No defaults or latency targets are established here. |

Checkpoint-frequency changes and native dead-letter policies remain separate
questions. Neither authoring this document nor a recommended feasibility
exercise commits an alternate backend to the first release.

## 11. References

Official public sources consulted for this draft on 2026-10-12. They document
backend building blocks, not approval or validation of the proposed CFP
protocols.

### Cosmos DB

- [C1. Transactions and optimistic concurrency][cosmos-occ]: ETag conditions,
  logical-partition transaction scope, and single-/multi-region write behavior.
- [C2. Consistency levels][cosmos-consistency]: read consistency scope,
  replication, and session guarantees.

### Azure Blob Storage

- [B1. Lease Blob REST operation][blob-lease]: native lease actions, durations,
  expiry behavior, conditional use, unchanged ETag/Last-Modified, permissions,
  and diagnostic request IDs.
- [B2. Conditional headers][blob-conditions]: `If-Match`, `If-None-Match: *`,
  supported operations, and precondition failures.
- [B3. Put Blob REST operation][blob-put]: block-blob replacement, metadata,
  lease ID, and conditional writes.
- [B4. List Blobs REST operation][blob-list]: pagination, markers, returned
  properties, permissions, and tag-index consistency.
- [B5. Manage concurrency in Blob Storage][blob-concurrency]: strong read/list
  consistency and optimistic/pessimistic concurrency.
- [B6. Azure Storage redundancy][blob-redundancy]: primary/secondary replication
  and read-access behavior.
- [B7. Disaster recovery and failover][blob-failover]: planned/unplanned account
  failover, possible data loss, and Last Sync Time.

### PostgreSQL

- [P1. Transaction isolation][pg-isolation]: predicate rechecks, Read Committed,
  and Repeatable Read snapshots.
- [P2. Explicit locking][pg-locks]: row-lock scope/lifetime and advisory locks.
- [P3. INSERT][pg-insert]: unique-conflict handling, `RETURNING`, and privileges.
- [P4. UPDATE][pg-update]: conditional updates, retained columns, affected rows,
  and privileges.
- [P5. Date/time functions][pg-time]: transaction, statement, and clock time.
- [P6. COMMIT][pg-commit]: the explicit transaction completion boundary.
- [P7. Canceling queries][pg-cancel]: cancellation dispatch is not guaranteed
  command cancellation.
- [P8. WAL settings][pg-wal]: durability and `synchronous_commit` distinctions.
- [P9. Failover][pg-failover]: promotion and prevention of dual primaries.
- [P10. Row security policies][pg-row-security]: row visibility under policies.
- [P11. SELECT][pg-select]: deterministic pagination and `SKIP LOCKED` omissions.
- [P12. Routine vacuuming][pg-vacuum]: retained row versions and maintenance.

[cosmos-occ]: https://learn.microsoft.com/en-us/azure/cosmos-db/database-transactions-optimistic-concurrency
[cosmos-consistency]: https://learn.microsoft.com/en-us/azure/cosmos-db/consistency-levels
[blob-lease]: https://learn.microsoft.com/en-us/rest/api/storageservices/lease-blob
[blob-conditions]: https://learn.microsoft.com/en-us/rest/api/storageservices/specifying-conditional-headers-for-blob-service-operations
[blob-put]: https://learn.microsoft.com/en-us/rest/api/storageservices/put-blob
[blob-list]: https://learn.microsoft.com/en-us/rest/api/storageservices/list-blobs
[blob-concurrency]: https://learn.microsoft.com/en-us/azure/storage/blobs/concurrency-manage
[blob-redundancy]: https://learn.microsoft.com/en-us/azure/storage/common/storage-redundancy
[blob-failover]: https://learn.microsoft.com/en-us/azure/storage/common/storage-disaster-recovery-guidance
[pg-isolation]: https://www.postgresql.org/docs/18/transaction-iso.html
[pg-locks]: https://www.postgresql.org/docs/18/explicit-locking.html
[pg-insert]: https://www.postgresql.org/docs/18/sql-insert.html
[pg-update]: https://www.postgresql.org/docs/18/sql-update.html
[pg-time]: https://www.postgresql.org/docs/18/functions-datetime.html#FUNCTIONS-DATETIME-CURRENT
[pg-commit]: https://www.postgresql.org/docs/18/sql-commit.html
[pg-cancel]: https://www.postgresql.org/docs/18/libpq-cancel.html
[pg-wal]: https://www.postgresql.org/docs/18/runtime-config-wal.html
[pg-failover]: https://www.postgresql.org/docs/18/warm-standby-failover.html
[pg-row-security]: https://www.postgresql.org/docs/18/ddl-rowsecurity.html
[pg-select]: https://www.postgresql.org/docs/18/sql-select.html
[pg-vacuum]: https://www.postgresql.org/docs/18/routine-vacuuming.html

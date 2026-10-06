<!-- cspell:ignore SHIM SHIMs checkpointing lockstep -->

# Change Feed Processor Package Boundaries

**Status:** Draft; package decision requested, not repository consensus
**Date:** 2026-10-06
**Updated:** 2026-10-06
**Authors:** Not specified

## Table of Contents

- [1. Motivation and Decision Requested](#1-motivation-and-decision-requested)
- [2. Package Dependencies](#2-package-dependencies)
- [3. Ownership, Reuse, and Duplication](#3-ownership-reuse-and-duplication)
- [4. Callback and Serialization Boundaries](#4-callback-and-serialization-boundaries)
- [5. Alternatives and Compatibility Consequences](#5-alternatives-and-compatibility-consequences)
- [6. Integration Boundary](#6-integration-boundary)
- [7. Package Decisions Still Open](#7-package-decisions-still-open)

## 1. Motivation and Decision Requested

The Change Feed Processor (CFP) needs an idiomatic Rust API and a shared
coordination engine reusable by other language SDKs. Putting application typing
inside that engine would couple reusable processing to Rust document models and
serialization.

This proposal requests agreement on two library packages:

- **`azure_data_cosmos_change_feed_processor`**: public Rust API.
- **`azure_data_cosmos_change_feed_processor_driver`**: schema-agnostic CFP
  coordination engine.

The engine builds on `azure_data_cosmos_driver`, not the typed
`azure_data_cosmos` SDK. This extends the existing SDK/driver separation to CFP;
it does not establish support or release approval.

This document requests agreement on packages, dependency direction, ownership,
and distribution consequences, including the processing-completion boundary.
It does not detail polling, lease algorithms, checkpoint state machines,
scheduling, an FFI protocol, or implementation tests.

## 2. Package Dependencies

Concatenating a node's wrapped lines gives its package name. Every package edge
below is a proposed Cargo dependency; only the host-to-FFI edge crosses a C ABI.

```mermaid
flowchart TB
    App["Rust application"]
    API["azure_data_cosmos_<br/>change_feed_<br/>processor"]
    Core["azure_data_cosmos_<br/>change_feed_<br/>processor_driver"]
    DB["azure_data_cosmos_driver"]
    Host["Other language SDK<br/>host-language SHIM"]
    Native["azure_data_cosmos_<br/>change_feed_<br/>processor_native<br/>(future package)"]
    App --> API
    API -->|"processing"| Core
    Core -->|"database execution"| DB
    API -->|"configuration / codecs"| DB
    Host -->|"C ABI"| Native
    Native -->|"processing"| Core
```

**The public Rust API also directly depends on `azure_data_cosmos_driver`** for
configuration, credential binding, and codec access. The author selected this
edge for the proposal. This avoids making the shared CFP engine a forwarding layer for
unrelated database configuration and codecs. It does not move processor
coordination into the public Rust API or introduce a dependency on `azure_data_cosmos`.

`azure_data_cosmos_change_feed_processor_native` is the selected name for a future
Rust FFI wrapper, not an approved third deliverable. It consumes the shared CFP
engine rather than the public Rust API; its placement and distribution remain
open. The existing `azure_data_cosmos_driver_native` wraps database execution,
not CFP coordination. The CFP driver calls `azure_data_cosmos_driver` through
Rust, not through that existing database FFI package.

## 3. Ownership, Reuse, and Duplication

Removing a dependency on `azure_data_cosmos` does not remove the capabilities
implemented in `azure_data_cosmos_driver`. Duplication is primarily at the typed
public Rust API, not in transport or database execution.

| Surface | Package ownership and treatment |
| --- | --- |
| Public Rust callback API | `azure_data_cosmos_change_feed_processor` owns application-type decoding, invocation of the application's `handleChanges` callback, options, error adaptation, and the internal processing adapter. |
| Shared CFP engine | `azure_data_cosmos_change_feed_processor_driver` owns autonomous polling, bootstrap, lease authority, the shared processing-exchange implementation, and checkpoint decisions. Rust and native bindings use the same delivery/result handling. It invokes an internal adapter, not the application's typed callback, and does not know the application's document type. |
| Typed change events | For an equivalent SDK event contract, the public Rust API duplicates/adapts `ChangeFeedItem<T>`, `ChangeFeedMetadata`, `ChangeFeedOperationType`, and `LogicalSequenceNumber`, including their custom deserialization behavior. |
| Change-feed-specific public options | The public Rust API provides equivalents/adapters for the relevant `ChangeFeedMode`, `ChangeFeedOptions`, and `FeedOptions` responsibilities, including start-position configuration. Do not copy unrelated options or the entire typed iterator implementation. |
| SDK-only conveniences | Recreate only those the public Rust API promises: for example, `RoutingStrategy::ProximityTo` expansion, SDK binary-option environment/default resolution, and application-facing diagnostics adapters. These are not inherited merely by depending on the database driver. |
| Request policies and database execution | Reuse `azure_data_cosmos_driver` implementations and driver-owned `OperationOptions`: retries, cross-region routing, hedging, failover, metadata/topology caches, credential binding, and diagnostics data. Public exposure of their types is a separate compatibility decision. |
| Wire encoding | Reuse `azure_data_cosmos_driver` binary JSON decoding, encoding, and transcoding facilities. Do not create another codec implementation in either CFP package. |
| Rust FFI wrapper | The proposed `azure_data_cosmos_change_feed_processor_native` exposes a thin C-compatible binding to the shared processing exchange and forwards host results into its completion path. Other language SDKs own application-type decoding and invocation of their application callbacks. |

Copying event declarations alone is insufficient. The SDK event decoder distinguishes envelopes
from flat documents, preserves optional metadata and previous images, handles
empty/null delete images, and tolerates unknown operation types. Equivalent
typed behavior requires adapting those semantics as well as the four types.
These copied types have **different Rust type identities**, even when their
definitions match the SDK's; matching wire semantics is not source-level
interchangeability.

## 4. Callback and Serialization Boundaries

### 4.1 Bidirectional processing completion and lockstep checkpointing

```text
Shared CFP engine -> raw batch -> public Rust API / Rust FFI wrapper -> SDK callback
Shared CFP engine <- correlated processing outcome <------------------ completion
```

The proposed requirement is **one shared processing-exchange implementation in
`azure_data_cosmos_change_feed_processor_driver`, accessed through two thin
bindings, not two independently implemented protocols with equivalent behavior.**
It implements a bidirectional asynchronous request-reply boundary: raw batch
delivery goes outward and the actual processing outcome returns, correlated
with the pending delivery. Runtime communication is bidirectional while Cargo
dependencies remain one-way; defining this contract does not require the engine
to import the consuming SDK or know its application type `T`.

The shared CFP engine owns the common implementation of:

- Pending-delivery identity and its association with the lease, ownership
  generation, and candidate checkpoint.
- Processing-result acceptance, including stale and duplicate result handling.
- Per-lease admission and outstanding-work limits.
- Checkpoint eligibility, persistence, and retry/reconciliation decisions.

The public Rust API accesses this implementation through Rust interfaces, not
the C ABI. Its awaitable internal adapter decodes the raw batch, invokes the
application's `handleChanges` callback, and returns its actual completion/error
into the **same core-owned completion path** as a native host's result. There
must be no Rust-only success path that bypasses this mechanism. The engine
invokes the internal adapter, not the typed application callback directly.

The future `azure_data_cosmos_change_feed_processor_native` package exposes a
C-compatible binding to this same implementation and forwards host processing
results into that completion path. Other language SDKs own decoding and
application callback invocation. Bindings may differ in application decoding,
callback invocation, host-runtime dispatch, ABI marshalling, and handle lifetime
management; they must not independently implement CFP ownership validation,
progress admission, or checkpoint/recovery rules.

A future and a queue are complementary mechanisms: a queue can carry a request
while a future awaits its reply. They are not separate processing protocols.
No particular channel primitive or ABI signature is selected here.
Cross-language reuse does not require a producer-only engine, and package
separation does not imply another thread or runtime. `handleChanges` names the
application callback role, not an implemented native symbol.

The shared CFP engine owns **lockstep processing and checkpointing**:

> For each owned lease, an application batch is complete only after
> `handleChanges` succeeds and its corresponding conditional checkpoint is
> confirmed durable. That lease session must not read or deliver the next batch
> before both conditions hold.

Fetching creates a candidate continuation, not a committed checkpoint.
Receiving or scheduling the batch is not processing success: the consuming SDK
reports success only after the callback's actual work completes. Processing
failure or an absent reply cannot authorize checkpoint advancement; the engine
applies its retry/stop policy, and silence does not prove the callback never ran.
Before checkpointing, the engine validates delivery identity and current lease
ownership; stale outcomes cannot authorize progress.

After processing succeeds, checkpoint persistence failure leads to safe
checkpoint retry or reconciliation, not automatic re-invocation of the
successful callback. Lease renewal remains independent while the batch is
outstanding. This is lockstep coordination, not transactional atomicity: a
crash after application side effects but before checkpoint confirmation can
cause replay, so it does not provide exactly-once external effects. This is a
proposed package-level contract, not an implemented or repository-approved
native API.

### 4.2 Why the typed database SDK is not the shared baseline

`azure_data_cosmos` exposes typed
APIs such as `ContainerClient::query_change_feed<T>`. A host-language SHIM is the
Java/.NET/Go/Python adapter that receives native payloads and invokes its
language's API and serializers. Raw application payloads must reach that SHIM
without first materializing them into Rust application models.

Wrapping a typed Rust API with FFI is technically possible, but is rejected as
this baseline: it would introduce a Rust representation and conversion or
serialization step before host-language decoding. Choosing a generic Rust JSON
value does not remove that coupling. Use the database driver's `ResponseBody`
buffers and metadata instead.

**Raw means application-schema-agnostic, not text-only or byte-for-byte
untouched.** The database driver can perform wire decoding, schema-independent
envelope processing, and binary/text conversion. Each consuming Rust API/SHIM
owns application typing and receives the declared payload shape and encoding,
including supported change metadata and previous images.

## 5. Alternatives and Compatibility Consequences

| Alternative | Package tradeoff |
| --- | --- |
| Add CFP to `azure_data_cosmos`, or have the shared CFP engine depend on it. | Couples processor delivery to the typed SDK and places Rust document decoding below the native boundary. Rejected as the shared baseline. |
| Depend on `azure_data_cosmos` only from the public Rust CFP API to reuse event types. | Technically valid without affecting the shared engine or native path, but couples CFP's public types and releases to the primary SDK. Not selected for this draft. |
| One standalone CFP package. | Fewer artifacts, but combines the typed Rust API with reusable coordination and couples their public compatibility surfaces. |
| Two packages over `azure_data_cosmos_driver` (proposed). | Separates typing from coordination and reuses database capabilities; requires adapters and explicit compatible dependency ranges. |
| Route all public Rust API configuration/codec access through the shared CFP engine. | Hides the direct dependency but adds forwarding APIs unrelated to coordination. The proposal instead retains the direct database-driver edge. |

An internal dependency does not require re-exporting its public types. Exposing
a driver option, error, or model in the public Rust API makes that type and
its dependency version part of that API's compatibility contract. Use explicit
adapters by default; any intentional shared public types need a documented
compatibility decision. Package separation alone does not guarantee independent
versioning or compatibility with every version of the database SDK/driver.

The public Rust API is intended to provide a publishable Cargo source package. Publishing
it to crates.io with a normal dependency on the CFP driver requires a compatible
CFP-driver package available there, as well as its compatible database-driver
dependency. A local path dependency is not a distribution substitute. Publication
and release ordering therefore cannot be decided independently, even if support
policies and version cadence differ. A future native package has a separate
platform-library/header and ABI distribution contract.

### 5.1 Why not reuse the typed SDK's types in the public Rust API?

Depending on `azure_data_cosmos` solely from
`azure_data_cosmos_change_feed_processor` could reuse SDK event types and their
deserialization behavior without adding that dependency to the shared CFP
engine or native consumption path. This is a technically valid alternative,
distinct from making the shared engine depend on the typed SDK; FFI does not
make public-Rust-API-only type reuse impossible.

The author-selected direction for this draft instead uses explicit CFP-owned
public models to reduce coupling to the primary SDK's public API and release
lifecycle. Exposing SDK-owned types would make their compatibility part of CFP's
public API contract. Applications and CFP could resolve different SDK versions
or sources, yielding distinct Rust type identities and interoperability problems.
Ordinary diamond dependencies are supported by Cargo: repeated paths to the
same resolved package do not inherently duplicate types, clients, or runtime
instances. The concern is deliberate compatibility coupling, not an intrinsically
unsafe dependency diamond.

Cargo has no special "types-only dependency": using only model APIs still brings
the package's enabled dependency graph. `default-features = false` does not
turn `azure_data_cosmos` into a models-only package, and other dependency paths
may enable additional features.

The cost is maintaining the relevant event types and decoder semantics
identified in the duplication table in section 3. CFP-owned models have distinct
Rust type identities and are not automatically interchangeable with SDK types.
This duplication is a maintenance tradeoff, not a free solution or a guarantee
of complete versioning independence. It is neither repository approval nor a
universal rule for Rust libraries.

## 6. Integration Boundary

Reuse the database driver's feed/dataflow machinery and prepared account/container
bindings rather than copying SDK iterators, database execution, or metadata
caches into CFP. Preserve driver diagnostics data; the public Rust API adapts it for
application-facing diagnostics.

Feed and lease bindings accept independently configured endpoints and credentials.
The CFP retains the prepared driver/container bindings. Database authorization
stays in `azure_data_cosmos_driver`; provider-specific token acquisition and
lifecycle stay with the supplied credential provider. An application may
explicitly reuse a credential authorized for both accounts; there is no implicit
feed-to-lease credential fallback.

`ContainerClient` holds private driver-backed state, not the shared metadata-cache
implementation. Bypassing it does not automatically reuse an existing client's
state. Sharing must use supported, compatible driver/runtime contexts.
`CosmosDriverRuntime::create_driver` creates a fresh driver; compatible drivers
from the same runtime share runtime-owned resources. Retain the required
instances rather than assuming an account singleton.

## 7. Package Decisions Still Open

| Question | Public boundary or distribution consequence |
| --- | --- |
| Which driver types, if any, are exposed by the public Rust API instead of adapted? | Defines source compatibility and constraints on database/CFP-driver dependency upgrades. |
| What typed event and option contract does the public Rust API promise? | Determines which SDK-owned models, decoder semantics, and builder conveniences must be duplicated/adapted. |
| What are each package's publication, support, versioning, and compatible dependency-range policies? | Determines artifacts and coordinated release ordering; publishing the public Rust API requires its dependency packages to be available. |
| What is the placement and distribution of `azure_data_cosmos_change_feed_processor_native`? | Determines a separate package/ABI artifact boundary without making the public Rust API the native baseline; the package name is selected. |

The direct public-Rust-API-to-database-driver dependency is selected for this
proposal, not an unresolved alternative. Beyond the processing-completion
boundary in section 4.1, operational CFP details and FFI delivery mechanics are
intentionally excluded from this package decision.

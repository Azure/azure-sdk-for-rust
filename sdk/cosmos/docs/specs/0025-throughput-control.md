<!--
Copyright (c) Microsoft Corporation. All rights reserved.
Licensed under the MIT License.
-->

# Throughput control

**Status:** Partially implemented — server-side signalling only. This document
describes what exists today and names the gaps explicitly.
**Crates:** `azure_data_cosmos_driver` (canonical), `azure_data_cosmos` (re-export)
**Supersedes the deferral in:** [0001 — Configuration options](0001-configuration-options.md),
which deferred `priority` and `throughput_bucket` to "a throughput control
follow-up spec" (PR [#3803]).

---

## Table of Contents

1. [Scope and non-goals](#1-scope-and-non-goals)
2. [Concepts](#2-concepts)
3. [Configuration surface](#3-configuration-surface)
4. [Layering](#4-layering)
5. [Resolution](#5-resolution)
6. [Request and header mapping](#6-request-and-header-mapping)
7. [Validation, defaults, and errors](#7-validation-defaults-and-errors)
8. [Service-side responses](#8-service-side-responses)
9. [Boundaries and unknowns](#9-boundaries-and-unknowns)

---

## 1. Scope and non-goals

"Throughput control" in this driver means exactly one thing: **telling the
service how to treat a request when the container's provisioned throughput is
contended.** It is expressed by two request headers, `x-ms-cosmos-priority-level`
and `x-ms-cosmos-throughput-bucket`, plus the configuration machinery that
decides what values those headers carry.

**Non-goal: client-side rate limiting.** The driver has no RU accounting loop,
no target-throughput setting, no local token bucket, and no client-side
queueing or deferral. Nothing in this crate delays or rejects a request to stay
under an RU budget. Other Cosmos SDKs ship a client-side throughput-control
feature under a similar name; this is not that, and this spec does not propose
it. Enforcement is entirely server-side.

The type names (`ThroughputControlGroupOptions`, `ThroughputControlGroupName`,
`ThroughputControlGroupRegistry`) come from that other-SDK vocabulary. Read
"group" here as *a named, mutable bundle of header values scoped to a
container*, not as a client-side throttling group.

[#3803]: https://github.com/Azure/azure-sdk-for-rust/pull/3803

## 2. Concepts

### 2.1 Priority level

`PriorityLevel` is a closed enum with two variants, `High` (the `Default`) and
`Low`, whose wire representation is the exact string `"High"` / `"Low"`
(`Display` and `FromStr` round-trip). When the account has priority-based
execution enabled, the service throttles `Low` requests before `High` ones once
provisioned throughput is exhausted. When the account feature is off, the header
is accepted and ignored — the driver does not detect or validate account
capability.

### 2.2 Throughput bucket

`throughput_bucket` is an opaque `u32` identifying a server-side bucket
configured on the account. The driver treats it as an uninterpreted number: it
is stringified into the header and never range-checked, defaulted, or mapped to
a fraction of RU/s. Bucket semantics (how many buckets exist, what share each
gets) live entirely in the service configuration.

### 2.3 Groups

A `ThroughputControlGroupOptions` is a named binding of `(throughput_bucket,
priority_level)` to a `ContainerReference`:

```text
ThroughputControlGroupOptions
├── name: ThroughputControlGroupName     // immutable
├── container: ContainerReference        // immutable
├── is_default: bool                     // immutable
└── mutable: Arc<RwLock<{ throughput_bucket: Option<u32>,
                          priority_level:   Option<PriorityLevel> }>>
```

The identity fields are fixed at construction; only the two values are mutable,
via `set_throughput_bucket` / `set_priority_level`. The `Arc<RwLock<…>>` is the
point of the type: a caller can flip a whole family of requests from `High` to
`Low` at runtime by mutating one registered group, without rebuilding the
client. Reads take a snapshot (`ThroughputControlGroupSnapshot`) per operation.

## 3. Configuration surface

Two independent entry points, both driver-owned and both re-exported by the SDK
(`azure_data_cosmos::options`).

### 3.1 Per-request / per-layer options

`ThroughputControlOptions` is a nested option group on `OperationOptions`
(`#[option(nested)]`), following the same pattern as `ThrottlingRetryOptions`:

| Field | Type | Meaning |
| --- | --- | --- |
| `group_name` | `Option<ThroughputControlGroupName>` | Fallback source for the two values below |
| `throughput_bucket` | `Option<u32>` | Direct override for the bucket header |
| `priority_level` | `Option<PriorityLevel>` | Direct override for the priority header |

The direct fields exist so a caller can set the headers **without registering a
group at all** — a one-off `priority_level: Low` on a single background query
needs no registry entry. Registered groups are for shared, mutable values across
a family of operations.

None of these fields read environment variables. Throughput control is a
deliberate per-application policy, so there is no `AZURE_COSMOS_*` variable and
no `_OVERRIDE` kill switch for any of them.

### 3.2 Group registration

Groups are registered at build time:

- Driver: `DriverOptionsBuilder::register_throughput_control_group`
- SDK: `CosmosClientBuilder::register_throughput_control_group` (collects, then
  replays into the driver builder at `build()`)

The registry is **per driver** and immutable after driver creation — there is no
runtime-level registry, so two clients built from the same runtime have
independent group sets. Only the values inside a registered group can change
afterwards.

## 4. Layering

`ThroughputControlOptions` participates in the standard option layers
(`#[options(layers(runtime, account, operation))]`), with **per-field**
resolution — an unset inner field falls through independently of its siblings:

```text
operation  →  account (DriverOptions / CosmosClientOptions.operation)  →  runtime defaults
  (highest)                                                                 (lowest)
```

So a runtime default of `priority_level: Low` plus an operation-level
`throughput_bucket: 99` yields both values; the operation layer does not blank
out the runtime's priority. The environment layers that exist for other option
groups contribute nothing here (§3.1).

Registered groups are *not* a layer. They are a lookup consulted after layering,
only when the layered result left a field unset (§5).

## 5. Resolution

`CosmosDriver::effective_throughput_control` runs **once per operation**, before
the first attempt, and produces a `ResolvedThroughputControl { throughput_bucket,
priority_level }` that is reused for every retry, failover, and hedge of that
operation. Mutating a group mid-flight therefore affects the *next* operation,
not an in-progress one.

Per field, independently:

1. If the layered `ThroughputControlOptions` set the field, use it.
2. Else, if `group_name` resolves to a group registered for **this operation's
   container**, use the group's value for that field (if the group sets it).
3. Else, omit the header.

Short-circuit: when both fields are already set by layering, the registry is not
consulted at all — and consequently an unresolvable `group_name` is *not*
reported in that case.

Resolution is skipped entirely for operations that do not target a container
(account metadata, database-level work). Throughput-control options on such
operations are ignored rather than rejected, because they were most likely
inherited from a client- or runtime-level default.

## 6. Request and header mapping

`build_transport_request` emits, when present:

| Resolved value | Header | Wire format |
| --- | --- | --- |
| `priority_level` | `x-ms-cosmos-priority-level` | `"High"` / `"Low"` |
| `throughput_bucket` | `x-ms-cosmos-throughput-bucket` | decimal `u32` |

Both are omitted when unresolved — there is no "send the default" behavior. The
group *name* is never transmitted; it is a purely client-side lookup key,
despite what the `ThroughputControlGroupName` rustdoc currently claims (§9).

**Gateway 2.0 caveat.** The Gateway 2.0 wrap synthesizes a fresh header set
containing only what the thin-client proxy needs, and there is no RNTBD request
token for priority level or throughput bucket. Requests dispatched over
Gateway 2.0 therefore **lose both headers silently**. This is a real gap, not a
design choice (§9).

## 7. Validation, defaults, and errors

Deliberately thin. Everything the service can validate is left to the service.

| Situation | Where | Outcome |
| --- | --- | --- |
| Duplicate `(container, name)` group | registration | `CLIENT_THROUGHPUT_CONTROL_GROUP_REGISTRATION_FAILED` — 400 / 20151 |
| Second `is_default = true` group for one container | registration | same as above |
| `group_name` not registered for the operation's container | request build | `CLIENT_THROUGHPUT_CONTROL_GROUP_NOT_REGISTERED` — 400 / 20152, request never sent |
| Unparseable priority string via `FromStr` | parse | `CLIENT_UNKNOWN_PRIORITY_LEVEL` — 400 / 20111 |
| Bucket value out of the account's configured range | service | 429 with sub-status 3212 (see §8) |

Defaults: no group, no bucket, no priority, no headers. `PriorityLevel::default()`
is `High`, but that default is never applied implicitly — an unset priority means
"omit the header", not "send `High`".

Registration errors surface as soon as the collision is introduced on the driver
builder; the SDK builder defers to that same call at `build()` time, so the
error text and status code are identical from both crates.

## 8. Service-side responses

Throughput-control failures come back as ordinary throttles. The driver
recognizes the relevant sub-status codes by name — `THROUGHPUT_BUCKET_LIMIT_EXHAUSTED`
(3212) and `TOO_MANY_THROUGHPUT_BUCKET_UPDATES` (3213) — for diagnostics and
error reporting, but applies **no special handling**: they take the generic
429 path through the throttling retry policy, bounded by `ThrottlingRetryOptions`
(`max_retry_count`, `max_retry_wait_time`). Only 429/3092 is treated specially
elsewhere, as a failover-eligible system-resource signal.

Priority-based throttling likewise surfaces as a plain 429. There is no
client-side signal that a request was throttled *because* it was `Low`, and no
adaptive behavior keyed off priority.

## 9. Boundaries and unknowns

Known gaps, in rough order of how likely they are to bite:

1. **Gateway 2.0 drops both headers.** The wrap does not forward them and no
   RNTBD token carries them (§6). Any account on the thin-client path gets no
   throughput control today. Fixing this needs either a token definition or an
   explicit header passthrough in `wrap_request_for_gateway_v2`.
2. **`is_default` is validated but never consulted.** The registry enforces at
   most one default group per container and stores it, but resolution only
   reaches the registry through an explicit `group_name`;
   `get_default_for_container` has no production caller. Either resolution
   should fall back to the container's default group, or the flag should go.
3. **Registering a group is hard to reach from the SDK.** Registration is a
   *builder-time* API that requires a resolved `ContainerReference`, but
   `ContainerReference` can only be produced by `CosmosDriver::resolve_container`
   on an already-built driver — and the SDK's own accessor
   (`ContainerClient::container_reference`) is `pub(crate)`. In practice SDK
   users can only use the direct `throughput_bucket` / `priority_level`
   overrides. There are no tests registering a group through either public
   builder, which is consistent with the surface being effectively unreachable.
4. **`ThroughputControlGroupName` rustdoc is wrong.** It states the name "is
   serialized into request headers"; no header carries it (§6).
5. **No bucket validation.** Any `u32` is accepted and sent; misconfiguration is
   discovered only as a service-side 429/3212.
6. **No account-capability detection.** The driver does not know whether
   priority-based execution or throughput buckets are enabled on the account, so
   headers can be sent (and ignored) against accounts that do not support them.
7. **No native (C ABI) surface.** `azure_data_cosmos_driver_native` deliberately
   does not expose `register_throughput_control_group` yet; it needs its own
   flat `#[repr(C)]` options struct.
8. **No diagnostics.** Resolved throughput-control values are not recorded in
   `DiagnosticsContext`, so "which bucket/priority did this operation actually
   send?" is only answerable from the request headers.

Anything not listed above — client-side RU budgets, per-group throttling,
dynamic bucket assignment, priority-aware retry — is out of scope and must not
be inferred from the type names.

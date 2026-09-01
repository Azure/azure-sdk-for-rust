<!--
Copyright (c) Microsoft Corporation. All rights reserved.
Licensed under the MIT License.
-->

# Fault injection

**Status:** Implemented — this document describes the framework as it exists in
`azure_data_cosmos_driver` and `azure_data_cosmos` today.
**Crates:** `azure_data_cosmos_driver` (canonical), `azure_data_cosmos` (facade)
**Feature gate:** `fault_injection` (off by default in both crates)

---

## Table of Contents

1. [Scope and status](#1-scope-and-status)
2. [Ownership](#2-ownership)
3. [Rule model](#3-rule-model)
4. [Injection points](#4-injection-points)
5. [Matching](#5-matching)
6. [Injection behavior](#6-injection-behavior)
7. [Lifecycle and activation](#7-lifecycle-and-activation)
8. [Interaction with retries](#8-interaction-with-retries)
9. [Diagnostics](#9-diagnostics)
10. [Intended test use](#10-intended-test-use)
11. [Boundaries and known gaps](#11-boundaries-and-known-gaps)

---

## 1. Scope and status

Fault injection lets a test replace or perturb the transport-level outcome of
selected Cosmos requests: an HTTP status/sub-status pair, a synthetic response,
a connection failure, a response timeout, or an added delay. Faults are applied
**below the retry and failover machinery**, so the driver reacts to an injected
503 exactly as it reacts to a real one.

This document covers the rule model, where injection happens, how rules are
matched and applied, how faults surface to retries and diagnostics, and what the
framework deliberately does *not* do. It is not an API reference; the types are
documented in rustdoc under `azure_data_cosmos_driver::fault_injection`.

The framework is a **testing tool**. Both crates gate it behind the
`fault_injection` cargo feature, which is off in default builds and excluded
from the released feature set consumers are expected to enable. Everything
below only exists when that feature is on.

## 2. Ownership

**The driver is canonical.** `azure_data_cosmos_driver::fault_injection` owns
the rule types, the evaluation logic, and the `TransportClient` that performs
injection.

**The SDK is a facade.** `azure_data_cosmos::fault_injection` is a pure
re-export module: `FaultInjectionRule`, `FaultInjectionRuleBuilder`,
`FaultInjectionCondition(Builder)`, `FaultInjectionResult(Builder)`,
`CustomResponse(Builder)`, `FaultInjectionErrorType`, `FaultOperationType`, and
(from `driver::diagnostics`) `TransportKind`. `CosmosClientBuilder::with_fault_injection_rules`
takes the driver's `Vec<Arc<FaultInjectionRule>>` verbatim and forwards it into
`DriverOptionsBuilder::with_fault_injection_rules`.

This is deliberate. PR [#4426] deleted the SDK's parallel type hierarchy, its
translation layer (`driver_bridge::sdk_fi_rules_to_driver_fi_rules`), and a dead
SDK-side `FaultClient` — roughly a thousand lines that duplicated driver types
and created a dual-state problem, because SDK and driver rules shared mutable
state through `Arc` accessors. The rule for future work: **new fault-injection
capability lands in the driver; the SDK only re-exports it.**

The C ABI wrapper (`azure_data_cosmos_driver_native`) does not surface fault
injection today — `with_fault_injection_rules` needs its own flat `#[repr(C)]`
options struct, which has not been designed.

[#4426]: https://github.com/Azure/azure-sdk-for-rust/pull/4426

## 3. Rule model

A rule is `condition × result × activation state`, plus a caller-supplied `id`
used for duplicate detection and for every diagnostic evaluation record.

```text
FaultInjectionRule
├── id: String                     // unique per driver
├── condition: FaultInjectionCondition
│   ├── operation_type:  Option<FaultOperationType>
│   ├── region:          Option<Region>
│   ├── container_id:    Option<String>
│   └── transport_kind:  Option<TransportKind>
├── result: FaultInjectionResult
│   ├── error_type:      Option<FaultInjectionErrorType>
│   ├── custom_response: Option<CustomResponse>   // status + headers + body
│   ├── delay:           Option<Duration>
│   └── probability:     f32                      // default 1.0
├── start_time / end_time: Option<Instant>
├── hit_limit: Option<u32>
└── enabled + hit_count: Arc<AtomicBool> / Arc<AtomicU32>
```

### 3.1 Conditions

All set fields must match (AND). An unset field matches everything, so a
default condition matches every request the client sends.

### 3.2 Results

`FaultInjectionErrorType` maps to a fixed status/sub-status pair or to a
transport-level failure:

| Error type | Injected outcome |
| --- | --- |
| `InternalServerError` | HTTP 500 |
| `TooManyRequests` | HTTP 429 |
| `RetryWith` | HTTP 449 |
| `ReadSessionNotAvailable` | HTTP 404, sub-status 1002 |
| `Timeout` | HTTP 408 |
| `ServiceUnavailable` | HTTP 503 |
| `PartitionIsGone` | HTTP 410, sub-status 1002 |
| `WriteForbidden` | HTTP 403, sub-status 3 |
| `DatabaseAccountNotFound` | HTTP 403, sub-status 1008 |
| `ConnectionError` | `TransportError`, `TRANSPORT_CONNECTION_FAILED` (20010), `RequestSentStatus::NotSent` |
| `ResponseTimeout` | `TransportError`, `TRANSPORT_IO_FAILED` (20011), `RequestSentStatus::Unknown` |
| `ResponseTimeoutAfterService` | request is forwarded; a *successful* response is discarded and reported as `TRANSPORT_IO_FAILED` / `Unknown` |

`CustomResponse` (status code, headers, body) takes precedence over
`error_type` when both are set; it is how tests mock a service payload — for
example a synthetic `GetDatabaseAccount` document that describes a topology the
real account does not have.

A result with no `error_type`, no `custom_response`, and no `delay` matches but
does nothing (and, once it wins matching, prevents lower-priority rules from
applying to that request); the builder documents this as a misconfiguration.

### 3.3 Shared activation state

`FaultInjectionRuleBuilder::with_shared_state(enabled, hit_count)` lets several
rules share one `AtomicBool`/`AtomicU32` pair, so a test can enable, disable, or
budget a *family* of rules (for example one rule per region) as a single unit.

## 4. Injection points

Injection is a transport decorator, not a pipeline stage.

```text
operation pipeline → retry/failover → transport pipeline → FaultClient → real HTTP client
```

- `FaultInjectingHttpClientFactory` decorates the runtime's `HttpClientFactory`.
  It is installed **per driver** at driver-creation time when
  `DriverOptions::fault_injection_rules()` is non-empty; otherwise the driver
  shares the runtime factory `Arc` unchanged. There is no runtime-level rule
  registry, so two clients built from one runtime have independent rule sets.
- Every client the factory builds — data-plane (Gateway 1.x and Gateway 2.0)
  and metadata — is wrapped in a `FaultClient` carrying the rule list and the
  `TransportKind` of that client (`None` for metadata clients).
- **Bootstrap is never wrapped.** The initial account-metadata probe runs on the
  runtime's bootstrap transport, which is built before any driver exists.
  `MetadataReadDatabaseAccount` rules therefore fire only on post-bootstrap
  refreshes.
- Because injection sits under the retry layer, each retry attempt is evaluated
  independently: a `hit_limit: 1` rule fails the first attempt and lets the
  retry through.

### 4.1 Operation tagging

`FaultClient` sees an HTTP request, not a `CosmosOperation`, so operation-typed
matching relies on a synthetic header. `apply_fault_injection_operation_tag`
writes `x-ms-fault-injection-operation: <FaultOperationType>` in two places: the
data-plane operation pipeline, and the off-pipeline bootstrap account fetch.
The Gateway 2.0 wrap builds a fresh header set for the proxy and explicitly
forwards this header, so operation-typed rules match identically on both
transports.

`FaultClient` strips the header before delegating to the real client, so it
never reaches the service.

## 5. Matching

For each request, `FaultClient::send` walks the rules in registration order and
classifies each one:

1. **Applicability** — `enabled`, then `start_time`, then `end_time`, then a
   non-atomic `hit_count >= hit_limit` pre-check.
2. **Condition** — operation type (from the tag header), region, container,
   transport kind.
3. **First match wins.** The first rule passing both checks is selected; any
   later rule that also matches is recorded as `Superseded`.

Two matchers deserve their exact semantics stated:

- **Region** matches on the *first DNS label* of the request host: either the
  label equals the region slug (`eastus.emulator.local`) or it ends with
  `-<slug>` (`myaccount-eastus.documents.azure.com`). Substring matching on the
  full URL was rejected because `eastus` would match `eastus2`.
- **Container** is a plain substring test against the whole request URL. It is
  adequate for tests with distinct container ids, and it is *not* a resolved
  `ContainerReference` comparison.

Transport-kind conditions are checked against the `FaultClient`'s own kind, so a
rule scoped to `TransportKind::GatewayV2` never matches metadata clients (which
have no kind).

## 6. Injection behavior

Once a rule is selected, `apply_fault`:

1. **Probability.** If `probability < 1.0`, draws `rand::random::<f32>()` and
   skips (recording `ProbabilityMiss`) when the draw fails, when the value is
   non-finite, or when it is `<= 0.0`. No delay is applied on a miss.
2. **Hit reservation.** Reserves a slot atomically via `fetch_update`, so
   concurrent requests cannot exceed `hit_limit`. The reservation is released on
   drop unless committed — this matters for `ResponseTimeoutAfterService`, whose
   decision is deferred.
3. **Delay.** Applied *before* the fault for ordinary rules; a delay-only rule
   is therefore a latency injector that still forwards the request.
4. **Result.** `custom_response` first, then `error_type`.

### 6.1 HTTP-status faults are returned as responses, not errors

Status-code faults are returned as `Ok(HttpResponse)` carrying the injected
status, sub-status header, and a marker body. This is load-bearing: returning
them as `TransportError` would make the transport pipeline tag the outcome with
the synthetic `TRANSPORT_GENERATED_503`, masking the injected status and
defeating the point of injecting a specific status.

Connection-level faults (`ConnectionError`, `ResponseTimeout`) are the opposite
case: they must be `TransportError`s so the pipeline sees a delivery failure,
and they carry the `RequestSentStatus` that decides whether a non-idempotent
operation may be retried.

### 6.2 Ambiguous-outcome faults

`ResponseTimeoutAfterService` exists for one scenario: the mutation *did* commit
but the client never learned the outcome. The request is forwarded; success is
evaluated from the real response (for Gateway 2.0, from the unwrapped RNTBD
backend status, not the outer proxy status); only then are the hit committed,
the delay applied, and a `TRANSPORT_IO_FAILED` / `Unknown` error produced. If
the service itself failed, that failure is returned unchanged and the hit is
released. This is what makes exactly-once/patch-retry tests deterministic.

## 7. Lifecycle and activation

- **Registration** happens at builder time and is immutable afterwards. Rules
  are appended across calls on `DriverOptionsBuilder`; duplicate ids fail
  immediately with `CLIENT_DUPLICATE_FAULT_INJECTION_RULE_ID` (400/20150),
  because a silently dropped duplicate would surface much later as "my fault
  never fired". The SDK builder *replaces* its pending vector on each call and
  defers validation to the driver-options layer, where the single concatenation
  point can see all rules.
- **Activation windows** use `Instant`, not wall-clock time: `start_time` and
  `end_time` are compared against `Instant::now()` at evaluation.
- **Runtime toggling** is the only post-build mutation: `enable()` / `disable()`
  on a rule handle (or on a shared `AtomicBool`). Conditions, results, windows,
  and hit limits cannot be changed after `build()`.
- **Hit accounting** is observable through `hit_count()` and is the usual way a
  test asserts "the fault fired exactly N times".

## 8. Interaction with retries

Injected faults are indistinguishable from real ones to everything above the
transport client, which is the entire point:

- 503 / 408 / 410-1002 drive cross-region failover and endpoint-unavailability
  marking.
- 429 drives the throttling retry policy, bounded by
  `ThrottlingRetryOptions` (see [0025](0025-throughput-control.md) for the
  related throughput-control headers, and [0006](0006-error-codes-and-retries.md)
  for the retry classification).
- 449 drives the RetryWith policy.
- 404-1002 drives session retries; 403-3 drives write failover; 403-1008 drives
  account-topology refresh.
- `ConnectionError` / `ResponseTimeout` drive connectivity retries, including
  the same-endpoint shard retry, gated by `RequestSentStatus`.

Rules are re-evaluated per attempt, so `hit_limit` is the primary lever for
"fail the first N attempts, then succeed" scenarios, and `probability` is the
lever for statistical fault storms.

## 9. Diagnostics

Every rule evaluation is recorded, not just the winning one. `FaultInjectionEvaluation`
has one variant per outcome: `Applied`, `ProbabilityMiss`, `Disabled`,
`BeforeStartTime`, `AfterEndTime`, `HitLimitExhausted`, `OperationMismatch`,
`RegionMismatch`, `ContainerMismatch`, `TransportKindMismatch`, `Superseded`.

Plumbing: when the driver has rules, the transport pipeline attaches a shared
`EvaluationCollector` to the outgoing `HttpRequest`; `FaultClient` appends its
evaluations; the pipeline drains the collector after the attempt and stores the
result on that attempt's `RequestDiagnostics`. Evaluations are exposed as
`RequestDiagnostics::fault_injection_evaluations()` and serialized into the
diagnostics JSON under `fault_injection_evaluations` (both feature-gated). All
evaluations are also emitted at `TRACE` level.

This is what makes a failing fault-injection test diagnosable: the evaluation
list distinguishes "the rule never matched" from "the rule matched but its
budget was exhausted" from "another rule won".

## 10. Intended test use

Fault injection is the substrate for the driver's resilience test suites:

- **Hosted-emulator tests** (`driver_fault_injection.rs`,
  `cosmos_fault_injection.rs`) — status-code handling, probability and
  hit-limit semantics, operation- and container-scoped rules, rule ordering and
  activation windows, 429/449 retry budgets, and the Gateway 2.0 variants of
  each.
- **In-memory emulator tests** — `InMemoryEmulatorHttpClient::runtime_builder_with_fault_rules`
  composes the emulator factory with the fault factory, which is how hedging,
  throttling, endpoint failback, metadata hedging, and topology-refresh tests
  inject regional delays and errors without a network harness.
- **Multi-region / multi-write tests** — regional failover, 403-3 write
  failover, 403-1008 topology refresh, partition failover.
- **Patch and exactly-once tests** — `ResponseTimeoutAfterService` to force the
  ambiguous "committed but unacknowledged" outcome.
- **Diagnostics tests** — asserting retry history and the evaluation list.

Guidance for new tests: prefer a narrow condition (operation type plus region or
container) and an explicit `hit_limit` over `probability`, so assertions stay
deterministic; assert on `hit_count()` and on the diagnostics evaluation list
rather than on timing.

## 11. Boundaries and known gaps

Stated explicitly so the framework is not mistaken for something broader.

- **Not a production feature.** Feature-gated, no configuration file or
  environment-variable surface, no serialization format for rules.
- **No bootstrap injection.** Rules cannot affect the first account-metadata
  fetch (§4).
- **Operation coverage is partial.** `FaultOperationType::PatchItem` exists but
  `from_operation_and_resource` does not map `OperationType::Patch` — patch is a
  client-side read-modify-write, so tests target its `ReadItem` / `ReplaceItem`
  legs instead. `SqlQuery`, `Head`, `HeadFeed`, `Execute`, and the DTX operation
  types have no mapping either; requests carrying them are untagged and match
  only rules without an operation condition.
- **Matching is URL-shaped.** Container matching is a substring test and region
  matching is a host-label test; there is no matching on partition key, feed
  range, request body, headers, or item id.
- **No response mutation.** A rule either replaces the outcome entirely
  (`custom_response` / `error_type`) or delays it. There is no "pass through but
  rewrite a header/RU charge" mode.
- **Immutable after build**, except `enable`/`disable` and shared state (§7).
- **Rule ordering is registration order**, with no explicit priority field;
  `Superseded` records exist so ordering mistakes are visible.
- **No native (C ABI) surface** (§2).

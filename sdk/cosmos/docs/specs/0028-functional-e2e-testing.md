# Cosmos SDK Functional E2E Testing

**Status:** Draft
**Date:** 2026-09-11
**Crates:** `azure_data_cosmos`, `azure_data_cosmos_driver`,
`azure_data_cosmos_emulator`

## Purpose

This plan introduces a dedicated functional end-to-end test suite for the Rust
Cosmos DB SDK. The suite defines and validates behavior observable through the
public `azure_data_cosmos` API while exercising the real driver, routing,
retry, session, diagnostics, and transport paths underneath it.

The suite runs against three backends:

- the hosted in-memory emulator through Gateway V1;
- the hosted in-memory emulator through Gateway V2; and
- Azure Cosmos DB live accounts where service fidelity is required.

The hosted emulator provides deterministic setup, replication delay, fault, and
topology control. Live accounts remain the fidelity reference. Neither backend
replaces the other.

The work is delivered in five sequentially reviewed and merged pull requests.
Development of later pull requests may proceed on dependent branches while an
earlier pull request is under review, but each pull request is rebased onto the
updated `main` and reviewed independently after its predecessor merges.

## Goals

- Validate Rust SDK functional behavior through public SDK APIs only.
- Exercise driver behavior implicitly through the supported Rust SDK surface.
- Cover positive and negative operation behavior, configuration precedence,
  consistency, retries, fault injection, availability, topology transitions,
  split/merge, partition migration, and diagnostics.
- Run deterministic behavior against the hosted emulator through Gateway V1
  and Gateway V2.
- Validate service fidelity through targeted and eventually comprehensive live
  account execution.
- Keep scenario identities, requirements, setup profiles, backend
  applicability, and implementation references machine-readable and
  language-neutral.
- Keep executable behavior and assertions source-native, type checked, and easy
  for humans to review.
- Allow Java, .NET, Python, and other SDKs to implement the same semantic
  scenario IDs without prescribing language-specific APIs.
- Keep individual CI shards within a target of 15–20 minutes.

## Non-goals

- The JSON catalog is not a general-purpose test language or interpreter.
- Tests do not call the Rust driver directly to validate product behavior.
- The hosted emulator is not treated as proof of complete service fidelity.
- The suite does not duplicate every unit, component, or protocol conformance
  test already present in the SDK or driver.
- The suite does not standardize incidental timing, opaque IDs, exact request
  charge, error text, or serialized diagnostics.
- Cross-language reuse does not require identical source implementations.

## Test architecture

### Ownership boundary

The suite deliberately separates reusable configuration data from executable
behavior.

| Owner                          | Contents                                                                                                                                                  |
| ------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Scenario JSON                  | Stable ID, title, requirement, maturity, tags, precedents, applicable setup profiles, backend applicability, fidelity, and required backend capabilities. |
| Profile JSON                   | Account topology and consistency, replication behavior, runtime configuration, and client configuration.                                                  |
| SDK implementation map         | Scenario ID to source-native test implementation and implementation status.                                                                               |
| Rust source                    | Fixtures, operation-level options, generated cases, sequencing, retries, concurrency, state validation, diagnostics, and assertions.                      |
| Pipeline matrices              | Backend and setup-profile selection, scheduling, and sharding.                                                                                            |
| Hosted emulator management API | Deterministic external controls for emulator-only orchestration.                                                                                          |

This boundary prevents the catalog from becoming a second programming language.
Operation-specific dimensions such as `ReadConsistencyStrategy`, patch
strategy, availability strategy, continuation behavior, fault rules, and batch
options remain beside the test that uses them.

### Scenario catalog

The language-neutral catalog lives under `sdk/cosmos/e2e_tests`:

```text
e2e_tests/
├── schema/           # Scenario and profile JSON Schemas
├── profiles/         # Reusable account/runtime/client setup
├── scenarios/        # Concise semantic scenario metadata
├── implementations/  # SDK-specific implementation maps
└── README.md
```

Scenario IDs are permanent. A semantic change creates a new ID; established IDs
are deprecated rather than repurposed.

Each scenario declares backend applicability:

- `required`: the scenario must execute and pass on the backend;
- `supported`: expected to work, but not yet enforced in every pipeline;
- `simulated`: deterministic emulator coverage without a full service-fidelity
  claim; and
- `notApplicable`: intentionally unavailable, with a reason.

An unavailable required capability is a configuration failure, not a silent
skip.

### Setup profiles

A profile describes account, runtime, and client axes. Pipelines select a
profile and, when an axis has multiple definitions, one definition from each
axis through:

- `AZURE_COSMOS_E2E_PROFILE`;
- `AZURE_COSMOS_E2E_ACCOUNT`;
- `AZURE_COSMOS_E2E_RUNTIME`; and
- `AZURE_COSMOS_E2E_CLIENT`.

For hosted-emulator jobs, setup translates the selected account definition into
an emulator configuration. Rust fixtures apply the selected runtime and client
configuration through public builders.

For live jobs, a profile is matched to a fixed or provisioned account whose
actual consistency, regions, write mode, and feature configuration satisfy the
profile. Live setup must not silently pretend to apply an account property that
the selected account does not have.

### Source-native implementations

Rust implementations live under
`sdk/cosmos/azure_data_cosmos/tests/e2e_test_cases`. Each scenario has its own
module. A module should be optimized for human review in this order:

1. test entry point and selected profile expansion;
2. visible arrange, act, and assert lifecycle;
3. operation case definitions with descriptive behavioral names;
4. selected-profile translation;
5. retry or orchestration mechanics; and
6. focused unit tests for generated case matrices.

Helpers should remove mechanical noise without hiding what behavior is tested
or what is asserted.

### Public SDK boundary

Product operations use only `azure_data_cosmos` public APIs. The driver is
exercised transitively and must not become the assertion surface for Rust SDK
functional contracts.

Emulator-only orchestration may call the hosted emulator's external management
REST API. This keeps topology and fault control language-neutral without
exposing Rust internals to test implementations.

## Assertion policy

Tests assert stable, customer-observable behavior:

- HTTP status and Cosmos substatus;
- typed error classification;
- returned models and item identity;
- resource state and operation side effects;
- ordering, uniqueness, continuation, and no-loss/no-duplication invariants;
- transaction atomicity;
- operation-level configuration precedence;
- terminal retry and failover behavior; and
- selected structured diagnostics such as operation, effective status,
  attempts, and contacted regions.

Transient service states are permitted rather than required when live timing is
nondeterministic. Tests accept only explicitly listed transient status/substatus
combinations and retry until a terminal state or a bounded deadline. Immediate
terminal success is always valid when replication has already converged.

Exact first-attempt timing should be asserted only when a deterministic
emulator control makes it part of the test contract.

## Execution tiers

### Pull-request smoke

- Small, stable functional subset.
- Hosted emulator Gateway V1 and Gateway V2.
- Fast enough to block normal pull requests.
- Includes catalog/schema validation and implementation-map consistency.

### Scheduled hosted-emulator matrices

- Expanded account/runtime/client profiles.
- Consistency and configuration-precedence matrices.
- Fault and topology scenarios requiring deterministic control.
- Split into shards with a target maximum duration of 15–20 minutes.

### Live account validation

- Targeted differential baselines initially.
- Comprehensive eligible-scenario coverage before final promotion.
- Fixed accounts where stable topology is required; provisioned resources where
  isolation or specialized account configuration is required.
- Separate shards by account configuration and scenario cost.

## Delivery plan

### PR1 — Foundation and executable smoke

**Status:** Implemented; under review.

Scope:

- scenario and profile JSON Schemas;
- concise language-neutral scenario metadata;
- reusable account/runtime/client profiles;
- Rust implementation map and catalog validation;
- source-native Rust E2E test target using public SDK APIs;
- one source module per scenario;
- hosted emulator capability discovery;
- hosted Gateway V1 and Gateway V2 CI execution;
- initial consistency and layered runtime/client/operation default matrices;
- scheduled expanded emulator profiles; and
- initial scenarios:
  - client bootstrap;
  - item create/read/replace/delete lifecycle;
  - upsert create/update behavior;
  - duplicate create conflict;
  - missing ID and wrong partition key;
  - optimistic concurrency with ETags;
  - parameterized filtering and ordering;
  - invalid query syntax; and
  - critical success/error diagnostics.

PR1 establishes architecture and representative execution. An `azureLive`
applicability value of `supported` records scenarios intended for live
validation but does not yet mean that all such scenarios are enforced by live
CI.

### PR2 — Core operations and emulator fidelity

**Status:** Implemented for hosted-emulator validation; live differential
execution is deferred to the live-promotion phase.

Scope:

- database and container control-plane lifecycle;
- item CRUD breadth across partition-key variants;
- Hash V1, Hash V2, and hierarchical partition keys;
- string, numeric, Boolean, null, and undefined partition-key behavior where
  supported;
- item ID, partition-key, payload-size, and unique-key validation;
- query and read-feed pagination;
- feed ranges and continuation handling;
- change-feed modes and resume behavior;
- transactional batch success, failure, and atomicity;
- patch operation and strategy behavior;
- exact post-operation state assertions;
- expanded negative status/substatus coverage;
- emulator support needed for those scenarios.

PR2 should add emulator behavior only when required by a concrete SDK scenario.
Differences discovered against live accounts must be fixed, explicitly modeled
as simulated, or documented as not applicable.

The query executor's synthetic-response diagnostics fallback intentionally
retains each successful backend response context for the lifetime of one plan
execution. Do not impose an additional fixed source-count cap or fold those
contexts at the executor level: fan-out is already constrained by the physical
partition topology, and inefficient queries that consume many empty backend
pages are precisely the cases where preserving detailed per-request diagnostics
is most valuable.

The PR2 pipeline does not run a live-account job. Its `azureLive: supported`
metadata remains declarative until targeted differential scenarios are wired
and validated during live promotion; hosted-emulator results must not be
treated as evidence of live-service fidelity.

### PR3 — Configuration, consistency, and resilience

**Status:** Implemented for hosted-emulator validation. Entra ID remains on the
existing dedicated live AAD pipeline until PR5 promotes catalog-driven Azure
Live execution.

Scope:

- primary and backup bootstrap paths;
- supported authentication modes;
- runtime, client, account, and operation configuration precedence;
- Gateway V1/Gateway V2 selection;
- binary encoding and text-response combinations;
- connection-pool and preferred-region behavior;
- all account consistency levels across applicable read operations;
- `ReadConsistencyStrategy` beyond the initial lifecycle matrix;
- session capture, explicit tokens, disabled management, and cross-page token
  behavior;
- throttling, transport, service, and timeout retries;
- end-to-end operation deadlines;
- cross-region hedging;
- public fault-injection behavior;
- stable diagnostics for representative retry and failure paths; and
- representative OpenTelemetry metrics, spans, and logs.

PR3 keeps fault predicates and operation-specific configuration in source code.
Only reusable environment setup belongs in profile JSON.

The hosted deadline scenario intentionally retains its one-second deadline,
1.5-second injected delay, and two-second completion ceiling. The typed
`CLIENT_OPERATION_TIMEOUT`, applied-fault count, and retained diagnostics are the primary
contract; the wall-clock bounds are a practical smoke-test check rather than a precise scheduler
guarantee. Wider delay/ceiling separation may be added as optional hardening, but is not required
for PR3.

The hosted Gateway V2 binary audit deliberately checks the protocol-defined `CosmosBinary` flag
bit (`0x02`) independently of the production encoder's named constant. Sharing that constant would
couple the implementation and its wire oracle, allowing both to agree on an incorrect value. Keep
the independent literal and its explicit wire-value tests rather than centralizing it for style.

The `configurationResilience` profile provides a fixed two-region Session
account with deterministic replication delay. Source-native Rust scenarios own
ordered backup fallback, preferred-region and account-order routing, binary
encoding/text-response overrides, session-token management, bounded throttle
retry, partition-topology refresh and retry signaling, operation deadlines,
hedging, per-attempt diagnostics, diagnostics handlers, metrics, sampled logging,
and OpenTelemetry spans. The existing
scheduled consistency profile additionally runs feed-read contracts across all
five account consistency levels. Key authentication is exercised by these
hosted runs; Entra ID cannot be faithfully validated by the unauthenticated
hosted emulator and remains covered by the live AAD pipeline.

### PR4 — Dynamic topology and availability

Scope:

- TODO: before adding PR4 topology profiles, validate each profile's
  `client.routing` value against the routing strategy implemented by its test fixture so a profile
  change cannot silently exercise a different routing mode. Scenarios that intentionally cover
  multiple source-owned strategies, such as preferred-region plus account-order routing, must
  declare that override explicitly instead of weakening validation globally;
- TODO: generalize replication helpers before PR4 adds alternate region layouts or partition-key
  values. `wait_for_item_replication` currently reads partition key `"A"` while excluding East US,
  and current pause callers target West US. These are accepted PR3 constraints because every
  applicable profile and caller has that shape. PR4 fixtures must instead derive the write region,
  alternate read region, and partition key from the selected profile/scenario, or fail setup with
  an explicit unsupported-topology error;
- runtime region add, remove, offline, online, and recovery;
- write-region failover and failback;
- deterministic partition migration simulation;
- physical partition split and merge;
- manually controlled transition phases;
- operations issued before, during, and after each transition;
- stale routing and address-cache refresh;
- replication pause, resume, and convergence;
- per-partition automatic failover;
- per-partition circuit breaker behavior;
- hedging interaction with topology changes;
- query and change-feed continuation across topology changes; and
- no-loss, no-duplication, ordering, and bounded-recovery assertions.

Dynamic controls are invoked through the hosted emulator management API.
Equivalent live scenarios are added only where Azure can expose the transition
safely and repeatably.

### PR5 — Full matrix, live enforcement, and promotion

Scope:

- complete live-account setup profiles;
- fixed-account and provisioned-account E2E matrices;
- targeted live differential baselines that confirm emulator fidelity;
- Azure Live execution for every eligible scenario;
- promotion of validated `azureLive` applicability from `supported` to
  `required`;
- complete diagnostics and OpenTelemetry audit;
- TODO: define the semantic boundary of every hosted-emulator `/health` counter before treating
  the counters as a reporting contract: decoded request, completed emulator operation, or
  successfully emitted host response. Then place related Gateway V1/Gateway V2 increments on the
  chosen side of fallible conversion boundaries and add failure-path tests that pin the result.
  PR3 intentionally keeps the current placement because generated Gateway V2 responses are
  buffered and use validated headers, no reachable conversion failure or cross-counter equality
  contract has been demonstrated, and moving counters now would choose semantics implicitly. The
  current `defaultConsistencyRequests` name is also retained in PR3: it is used as the wire bucket
  for requests carrying no non-default read-consistency-strategy signal, not as a semantic count of
  Default reads. PR5 must decide whether to rename it to reflect that wire meaning or add separate
  semantic read counters before exposing these values in reports;
- generated pull-request and scheduled shard manifests;
- JUnit and scenario/profile/backend coverage reports;
- runtime measurement and shard calibration;
- 15–20 minute maximum shard target;
- retry and quarantine policy for environmental failures;
- documentation for local, hosted-emulator, and live execution; and
- cross-SDK portability review with Java, .NET, and Python implementations.

PR5 is the release-readiness gate for the suite rather than a place to add
large amounts of previously untested functionality.

## Pull-request workflow

The five pull requests are reviewed and merged sequentially:

1. merge the current pull request;
2. update local `main` from `origin/main`;
3. replay the next pull request's commits onto updated `main`;
4. verify its diff contains only that pull request's scope;
5. rerun its validation; and
6. open it for review.

Development may proceed in parallel on dependent local branches. The repository
does not depend on GitHub stacked-PR automation, and reviewers see one
independent pull request at a time.

## Quality gates

Every pull request must:

- validate all scenario and profile documents against their schemas;
- validate every scenario ID has exactly one Rust implementation-map entry;
- validate every active implementation maps to a discovered Rust test;
- validate pipeline matrices cover their declared profile axes;
- run `cargo fmt`;
- build affected crates;
- run Clippy with all features and targets;
- build Rust documentation without warnings;
- run relevant offline tests;
- run applicable hosted-emulator Gateway V1 and Gateway V2 tests;
- run required live tests for that phase;
- pass Markdown lint and spelling checks; and
- avoid silent skipping of required scenarios or capabilities.

## Completion criteria

The roadmap is complete when:

- all planned functional domains have stable scenario IDs and active Rust
  implementations;
- the pull-request smoke set is reliable and bounded;
- scheduled hosted-emulator matrices cover deterministic fault and topology
  behavior through Gateway V1 and Gateway V2;
- every eligible scenario runs against appropriate live account profiles;
- required backend coverage is enforced rather than informational;
- CI reports scenario/profile/backend coverage;
- no shard exceeds the 15–20 minute target under normal conditions; and
- the catalog is usable by peer SDK teams without requiring them to adopt Rust
  APIs or a shared behavioral interpreter.

## Related documents

- Hosted emulator: `specs/0027-hosted-emulator.md`
- In-memory emulator: `specs/0021-in-memory-emulator.md`
- Operation and transport pipelines:
  `specs/0005-operation-and-transport-pipelines.md`
- Error codes and retries: `specs/0006-error-codes-and-retries.md`
- Partition-level failover: `specs/0008-partition-level-failover.md`
- Cross-region hedging: `specs/0009-cross-region-hedging.md`
- Gateway V2: `specs/0011-gateway-v2.md`
- Feed operations and dataflow:
  `specs/0012-feed-operations-and-dataflow.md`
- Patch handler: `specs/0017-patch-handler.md`
- Diagnostics contract: `specs/0018-diagnostics-contract.md`
- Fault injection: `specs/0024-fault-injection.md`
- Session consistency: `specs/0026-session-consistency.md`

# Cosmos SDK E2E test catalog

This directory contains language-neutral metadata and reusable setup profiles
for Cosmos DB SDK E2E tests. Test behavior remains in each SDK's source code so
it is type checked, directly reviewable, and idiomatic for that language.

## Layout

- `schema/` contains JSON Schemas for scenario metadata and setup profiles.
- `profiles/` contains reusable account, runtime, and client configurations.
- `scenarios/` describes the behavior covered by each stable scenario ID.
- `implementations/` maps scenario IDs to SDK-specific test implementations.

Scenario IDs are permanent. If an established scenario changes meaning, add a
new scenario and deprecate the old one rather than reusing its ID.

## Scenario metadata

A scenario document intentionally stays concise. It records:

- a stable ID, title, and normative requirement;
- maturity, tags, and prior service or SDK test precedents;
- the setup profiles under which the implementation should run;
- backend applicability, fidelity, and required capabilities.

Scenario JSON does not describe executable steps, operation-level options,
fixtures, retries, diagnostics, or assertions. Those concerns vary by SDK and
scenario and belong in the source-native implementation. This prevents the
catalog from becoming a second programming language and lets compiler and IDE
tooling validate the test logic.

## Setup profiles

A profile defines three setup axes: `accounts`, `runtimes`, and `clients`.
Pipeline matrices select one profile and one value from each axis through:

- `AZURE_COSMOS_E2E_PROFILE`;
- `AZURE_COSMOS_E2E_ACCOUNT`;
- `AZURE_COSMOS_E2E_RUNTIME`;
- `AZURE_COSMOS_E2E_CLIENT`.

The hosted emulator setup converts the selected account definition into an
emulator configuration. SDK fixtures apply the selected runtime and client
configuration. A test runs only when its scenario metadata includes the
selected profile.

The item lifecycle implementation uses three profiles:

- `smokeTests` for the default PR smoke case on any supported backend;
- `lifecycleConsistencyMatrix` for five account consistency configurations;
- `readConsistencyOverrideMatrix` for runtime and client default precedence.

The `coreOperations` profile selects the broader PR2 contracts for resource
lifecycle, partition-key variants, pagination and continuations, feed ranges,
transactional batch, patch, and item validation. It runs independently through
Gateway V1 and Gateway V2 so this coverage does not expand every PR1 smoke or
consistency shard.

The `configurationResilience` profile selects PR3 configuration, consistency,
session, retry, deadline, hedging, diagnostics-handler, and OpenTelemetry
contracts. Its reusable setup is a two-region Session account with a fixed
replication delay. Operation defaults, fault rules, retry budgets, hedging
thresholds, diagnostics assertions, and telemetry exporters remain in the Rust
implementations rather than expanding the profile schema into a behavior DSL.

The `dynamicTopology` profile selects region lifecycle, write failover,
replication, partition-transition, PPAF/PPCB, hedging, and continuation
contracts. It runs a three-region single-write account with per-partition
failover enabled. External management calls control transition phases; all
product operations and state assertions continue to use the public Rust SDK.

Operation-level `ReadConsistencyStrategy` cases, session-token choices,
acceptable transient statuses, retry deadlines, and assertions are defined in
the Rust test source. Future operation-specific dimensions follow the same
pattern instead of expanding the profile schema.

The scheduled matrices in `e2e-consistency-matrix.json` and
`e2e-read-consistency-override-matrix.json` select every setup cell through
Gateway V1 and Gateway V2. Profile-driven jobs use the isolated `e2e` test
category.

The blocking `e2e-core-operations-matrix.json` selects the single
`coreOperations` setup cell through both hosted gateways.

The blocking `e2e-configuration-resilience-matrix.json` selects the single
`configurationResilience` setup cell through both hosted gateways. The
existing scheduled consistency matrix additionally runs the feed-consistency
scenario across all account consistency levels.

The blocking `e2e-dynamic-topology-matrix.json` selects the single
`dynamicTopology` setup cell through both hosted gateways. Fixture serialization
keeps account-wide topology controls isolated from other scenarios in the shard.

## Azure Live release gate

The catalog-driven live gate has fixed-account and provisioned-account legs:

- `e2e-live-fixed-matrix.json` runs `smokeTests` and `coreOperations` against a
  fixed single-region Session account;
- `e2e-live-configuration-matrix.json` runs `liveConfiguration` against a
  fixed two-region, single-write Session account; and
- `e2e-live-aad-matrix.json` provisions a short-lived account and reruns the
  smoke profile with Entra ID data-plane authentication.

The fixed-account resolver exports the actual consistency, write mode, and
region list. Test setup compares those values with the selected profile before
compiling the isolated `e2e` category. An incompatible or incompletely
described account fails setup; required live coverage cannot silently skip.
The provisioned shard declares equivalent metadata in its matrix and obtains
the endpoint and consistency from the ARM deployment outputs.

For a local fixed-account run, dot-source the resolver as described in the
pipeline README, set the four `AZURE_COSMOS_E2E_*` axes, and invoke the Cosmos
test setup before running the `e2e_tests` integration target. The selected
account must match the profile topology. Hosted runs instead set
`AZURE_COSMOS_EMULATOR_FLAVOR` to `inmemory-v1` or `inmemory-v2`; setup creates
the account and management endpoint automatically.

Dedicated live shards use nightly Rust. The standard test runner writes Cargo
JSON, the repository converter emits JUnit, and
`eng/scripts/Write-CosmosE2eReport.ps1` produces `coverage.json` and
`coverage.md`. Each record includes the scenario, profile, account, runtime,
client, backend, actual transport, status, duration, and attempt count.
Required tests that are missing, failed, or skipped fail the report gate. The
canonical pull-request, scheduled, and live shard inventory is
`shards.v1.json`; every shard has a 20-minute ceiling.

Pipeline infrastructure may retry an environmental job once. Source-native
product tests do not add broad retries or convert failures into skips. A
temporary quarantine belongs in `quarantine.json` and must name an owner,
public issue, reason, expiry date, and exact profile/backend cells. Catalog
validation rejects unknown or expired entries, while reports label an allowed
failure `quarantined` rather than `passed`.

## Portable capability and adapter contract

`scenario.v1.json` is the versioned capability vocabulary. Capabilities name
service or backend behavior (`item`, `query`, `gatewayV2`,
`partitionSplit`), never a Rust type or method. `implementation.v1.json`
defines the common per-SDK source-native map. A peer SDK may mark a scenario
`unsupported` with a reason; it must not emulate executable behavior in JSON.

Profiles describe semantic setup intent. Each SDK adapter projects a field as
follows:

| Profile concern | Rust | Java/.NET | Python |
| --- | --- | --- | --- |
| Account consistency, regions, write mode | Backend/account setup | Backend/account setup | Backend/account setup |
| Gateway V2 and PPCB | Public option when available; otherwise backend default | Public or test-only SDK option | Unsupported unless exposed publicly |
| Binary encoding | Public client/runtime option | Public or test-only SDK option | Unsupported unless exposed publicly |
| Preferred/account-order routing | Public client routing | Public client routing | Public equivalent when available |
| Read-consistency defaults | Public runtime/client/operation option | Public equivalent when available | Account/backend-driven or unsupported |
| Diagnostics, metrics, tracing | SDK-native handlers and OpenTelemetry | SDK-native diagnostics and OpenTelemetry | SDK-native diagnostics hooks |

An adapter must fail its cell as unsupported when it cannot faithfully project
a required setup axis. Internal or test-only switches are recorded in that
SDK's implementation documentation and are not promoted into scenario
requirements. This keeps scenario identity portable without requiring peer
SDKs to copy Rust APIs.

## Hosted health-counter contract

All Gateway V1 and Gateway V2 `/health` request, binary, and consistency
counters increment only after the host has constructed a complete HTTP
response. Decode, dispatch, buffering, and response-conversion failures are
not counted. Consistency fields use `wireDefaultConsistencyRequests`,
`wireEventualConsistencyRequests`, and corresponding `wire*` names. The
Default bucket means no non-default read-consistency wire header was observed;
it is not a claim about semantic consistency selected by the service.

## Precedents and implementations

The `precedents` array records prior evidence for expected behavior. A
precedent can point to a service specification or an existing Rust, Java,
.NET, or Python test. It is not an implementation registry.

SDK implementations are tracked under `implementations/`. The Rust mapping
links every scenario ID to an executable test in
`azure_data_cosmos/tests/e2e_test_cases/`. Catalog validation fails if an
active mapping names a missing test, a scenario references an unknown setup
profile, or a scenario has no Rust mapping. Future SDKs can add their own map
without changing scenario meaning.

## Source-native assertions

The Rust implementations use only the public `azure_data_cosmos` surface for
product operations. Emulator orchestration uses the external management
endpoint. Concrete tests own resource fixtures, operation sequencing, data
invariants, HTTP status and substatus checks, diagnostics, retries, and cleanup.

Error text, serialized diagnostics, opaque identifiers, exact request charge,
and incidental timing should not become stable E2E assertions.

Backend applicability values are:

- `required`: the scenario must execute and pass;
- `supported`: expected to work but not required in every pipeline;
- `simulated`: deterministic emulator behavior without a service-fidelity claim;
- `notApplicable`: intentionally unavailable, with a reason.

An unavailable required capability is a test configuration failure, never a
silent skip.

# Cosmos SDK E2E test scenarios

This directory contains language-neutral E2E test scenarios for Cosmos DB
SDKs. A scenario describes observable SDK behavior; it does not prescribe a
language-specific API shape or expose driver internals.

## Layout

- `schema/` contains the JSON Schemas for scenarios and profiles.
- `vocabulary/` contains controlled identifiers shared by SDK runners.
- `profiles/` contains reusable account, runtime, and client configurations.
- `scenarios/` contains one JSON document per semantic scenario.
- `implementations/` maps scenarios to SDK-specific test implementations.

Scenario IDs are permanent. If an established scenario changes meaning, add a
new scenario and deprecate the old one rather than reusing its ID.

## Precedents and implementations

The `precedents` array records prior evidence for the expected behavior. A
precedent can point to a service specification or an existing Rust, Java,
.NET, or Python test. It does not mean that the scenario is implemented only
for that SDK, and it is not the implementation registry.

SDK implementations are tracked separately under `implementations/`. The
initial `rust.json` mapping links every scenario ID to an executable Rust E2E
test. Catalog validation fails if an active Rust mapping names a missing test,
if a scenario has no Rust mapping, or if a scenario is mapped more than once.
Future SDKs add their own implementation map without changing the
language-neutral scenario or its precedents.

## Fixture variants

Every scenario declares a `fixtures` array. Each fixture is one cohesive test
setup containing:

- a container partition-key definition, including paths, kind, and version;
- named item definitions;
- each item's partition-key values and JSON document;
- whether the item is seeded before the scenario steps run.

The same semantic steps run for every fixture unless an SDK implementation
documents a backend limitation. This makes partitioning variants explicit
without duplicating the scenario. For example, the duplicate-create scenario
runs against Hash V1, Hash V2, and hierarchical MultiHash V2 containers.

Steps can reference a named fixture item through `itemRef`. Keeping the
partition-key values beside the corresponding document prevents setup data
from drifting away from its container definition.

## Account profiles and execution cases

Each reusable profile contains three arrays: `accounts`, `runtimes`, and
`clients`. Its configuration space is their Cartesian product. Every
definition has a stable ID, and an execution selects one account/runtime/client
cell by ID. Keeping several definitions in one focused profile avoids creating
one profile file for every combination.

Runtime and client definitions can specify
`defaultReadConsistencyStrategy`. The execution's
`readConsistencyStrategy` is the operation-level value; `Inherit` leaves it
unset. This directly tests the precedence order operation > client > runtime >
account default. In particular, an explicit operation-level `Default` clears a
lower-layer non-default strategy and restores account-default read behavior.

The item lifecycle scenario uses two bounded profiles rather than one giant
Cartesian product:

- `lifecycleConsistencyMatrix`: five account definitions × one runtime × one
  client × five operation strategies (25 executions per gateway);
- `readConsistencyOverrideMatrix`: one Session account × three runtime
  definitions × two client definitions × three operation values (`Inherit`,
  `Default`, and `Eventual`) (18 executions per gateway).

Non-Strong two-region account definitions inject a deterministic replication
delay. Each execution declares `acceptableInitialStatuses`, a `terminalStatus`,
and, when transient statuses are accepted, `maxWaitMs`. The runner accepts an
immediate terminal result. Otherwise, it retries only public results matching
an acceptable status/substatus pair until it reaches the terminal pair or the
time cap. An omitted substatus matches any substatus; a missing response
substatus is normalized to zero when an explicit substatus is expected.

SDK-internal retries can hide an acceptable transient status from the public
result. The runner includes matching internal diagnostic attempts in failure
evidence but does not require a transient attempt. This keeps the same scenario
valid for deterministic emulator delay and nondeterministic live replication.

`LatestCommitted` means the latest committed value available in the selected
read region. It is not a cross-region replication barrier, so a delayed
secondary can return a temporary plain 404. Only a completed Strong-account
write guarantees that an eligible secondary can immediately return the item.

The scheduled matrices in `e2e-consistency-matrix.json` and
`e2e-read-consistency-override-matrix.json` execute every lifecycle case
through Gateway V1 and Gateway V2. Profile-driven jobs use the isolated `e2e`
test category so they do not rerun unrelated emulator suites.

## Assertions

Scenarios standardize outcomes, HTTP status and substatus, data side effects,
and selected structured diagnostics. Error text, serialized diagnostics,
opaque identifiers, exact request charge, and incidental timing are not stable
E2E assertions.

Backend applicability is explicit:

- `required`: the scenario must execute and pass.
- `supported`: expected to work but not required in every pipeline.
- `simulated`: useful deterministic emulator behavior, not a service-fidelity claim.
- `notApplicable`: intentionally unavailable, with a reason.

An unavailable required capability is a test configuration failure, never a
silent skip.

## Initial execution

The Rust implementation is under
`azure_data_cosmos/tests/e2e_test_cases/`. Product operations use only the
public `azure_data_cosmos` surface. Emulator-only orchestration uses the
external management endpoint.

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

- `hostedEmulatorSmoke` for the default PR smoke case;
- `lifecycleConsistencyMatrix` for five account consistency configurations;
- `readConsistencyOverrideMatrix` for runtime and client default precedence.

Operation-level `ReadConsistencyStrategy` cases, session-token choices,
acceptable transient statuses, retry deadlines, and assertions are defined in
the Rust test source. Future operation-specific dimensions follow the same
pattern instead of expanding the profile schema.

The scheduled matrices in `e2e-consistency-matrix.json` and
`e2e-read-consistency-override-matrix.json` select every setup cell through
Gateway V1 and Gateway V2. Profile-driven jobs use the isolated `e2e` test
category.

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

# Cosmos live-test fixed accounts

This directory implements fixed, self-owned Cosmos DB accounts for the
`Cosmos_live_test` CI leg.

## Why

`Cosmos_live_test` previously deployed a fresh Cosmos account per CI run (via
`sdk/cosmos/test-resources.bicep`) into whatever Azure tenant the
`azure-sdk-tests-cosmos` service connection happened to be federated to. That
tenant is ephemeral and rotates periodically; rotating it requires recreating
the ADO service connection, which is an administrative action outside of what
a pipeline change can do on its own.

Since these are key-based tests (they don't need Entra ID / AAD), they don't
need to run against that tenant at all. Instead, `Cosmos_live_test` now runs
against a small set of **fixed accounts** that live permanently in a
separate, self-owned subscription/resource group (`sdk-ci`) that the SDK team
controls directly. Because that subscription never expires or rotates, the
account endpoints and keys - stored once in an ADO secret - keep working
indefinitely, with no per-run Azure authentication and no service-connection
dependency at all for this leg.

`Cosmos_live_test_aad` (the dedicated AAD/Entra ID leg) is **unaffected** by
this change: it continues to deploy fresh accounts per run via
`test-resources.bicep` against the existing service connection, exactly as
before. Thin-client/GatewayV2 legs are also unaffected/out of scope.

## How it fits together

1. `live-test-accounts.schema.json` defines
   the shape of a single JSON blob describing every fixed account
   (endpoint, key, consistency, region info, etc.), keyed by a logical
   **account selector**.
2. `live-test-accounts.sample.json` is a
   placeholder-filled example matching the current
   `sdk/cosmos/live-platform-matrix.json` selectors, used by the local
   resolver tests.
3. `account-provisioning/` contains the one-time /
   per-rotation script (`New-CosmosLiveTestAccounts.ps1`) that creates the
   accounts in the `sdk-ci` resource group and prints the JSON to store in
   the ADO secret. See its README for the full runbook.
4. That JSON is stored as the `rust-ci` **Key Vault secret**, which is linked
   (read-through) into the `Test Secrets for Cosmos Live Tests - user
   administered` ADO variable group (already wired into `sdk/cosmos/ci.yml`).
   Because the variable group is a KV-backed mapping, rotation happens on the
   Key Vault secret itself - editing the variable-group value in ADO has no
   effect.
5. `resolve-cosmos-test-account.ps1` is a
   cross-platform (pwsh) script that, given a selector and the JSON secret,
   resolves and exports (via Azure DevOps `##vso[task.setvariable]` logging
   commands):
   - `AZURE_COSMOS_CONNECTION_STRING` (secret - masked in logs)
   - `AZURE_COSMOS_SECONDARY_KEY` (secret, only if the account defines one)
   - `ACCOUNT_HOST`, `DATABASE_NAME`, `AZURE_COSMOS_DEFAULT_CONSISTENCY`, `COSMOS_RUSTFLAGS`
   - `AZURE_COSMOS_E2E_BACKEND`, `AZURE_COSMOS_ACCOUNT_WRITE_MODE`, `AZURE_COSMOS_ACCOUNT_MULTI_REGION`, `AZURE_COSMOS_ACCOUNT_REGIONS`, and `AZURE_COSMOS_ACCOUNT_FULL_FIDELITY_CHANGE_FEED`

   The first group matches the environment variables that
   `sdk/cosmos/test-resources.bicep` used to produce as ARM deployment
   outputs. The second group lets catalog-driven E2E setup validate that a
   selected fixed account satisfies its profile before any required scenario
   starts. The Rust client also compares the live account-properties response
   with the selected profile, so stale secret metadata cannot mask consistency,
   write-mode, or topology drift.
6. `resolve-test-account-steps.yml` is a
   reusable ADO step template that invokes the resolver script with the
   current job's `$(AccountSelector)` matrix variable.
7. `sdk/cosmos/live-platform-matrix.json` adds an `AccountSelector` string
   next to each leg's existing `ArmTemplateParameters` (left in place for
   documentation/reference, but no longer used to deploy anything for this
   leg).
8. `eng/pipelines/templates/jobs/live.tests.yml` gained a
   `DisableAzureResourceCreation` parameter that skips
   `build-test-resource-config.yml`/`deploy-test-resources.yml` (no ARM
   deployment, no tenant/service-connection auth at all) and a
   `PreTestRunSteps` hook that runs in their place.
   `eng/pipelines/templates/stages/archetype-sdk-client.yml` gained a
   `FixedAccountMatrixConfigs` parameter (a second, independent
   `LiveTestMatrixConfigs`-like list) so this only applies to the matrix
   configs that opt in - `Cosmos_live_test_aad` keeps deploying real ARM
   resources, unaffected.
9. `sdk/cosmos/ci.yml` wires `Cosmos_live_test` through
   `FixedAccountMatrixConfigs` with `PreTestRunSteps` pointing at
   `resolve-test-account-steps.yml`, while `Cosmos_live_test_aad` remains on
   `LiveTestMatrixConfigs` exactly as before.

## Local testing

Run the resolver's local test suite (no ADO, no Azure access required):

```powershell
pwsh sdk/cosmos/eng/pipelines/resolve-cosmos-test-account.tests.ps1
```

You can also invoke the resolver directly against the sample JSON. Dot-source
it (note the leading `.` followed by a space) so the resolved variables land directly in your
current shell instead of just being printed:

```powershell
$env:COSMOS_ACCOUNTS_LOCAL = 'true'
$env:COSMOS_ACCOUNT_SELECTOR = 'session-singlewrite'
$env:COSMOS_TEST_ACCOUNTS_JSON = Get-Content -Raw sdk/cosmos/eng/pipelines/live-test-accounts.sample.json
. ./sdk/cosmos/eng/pipelines/resolve-cosmos-test-account.ps1
```

### Live-only query tests

The SDK's `live` test target shares the integration-test framework but keeps
live-only DISTINCT coverage separate from emulator-compatible query tests.
It requires the `key_auth`, `control_plane`, and `fault_injection` features
and is ignored by default.

To run it locally, set `AZURE_COSMOS_CONNECTION_STRING` to a real account's
connection string, then run from the repository root:

```powershell
$env:AZURE_COSMOS_TEST_MODE = 'required'
$env:RUSTFLAGS = '--cfg=test_category="live"'
cargo test -p azure_data_cosmos --all-features --test live live_distinct_admission_and_per_query_options
```

The test creates and cleans up a unique database, so the account key must
permit database and container management. Explicit selection (including
`--ignored`) fails on missing/invalid connection strings, local emulator
endpoints, or `AZURE_COSMOS_TEST_MODE=skipped`; it never reports a skipped
live scenario as a pass.

The fixed-account resolver appends `test_category="live"` alongside each
account's existing category. Test setup propagates these flags before its
preconfigured-connection early return, so `Cosmos_live_test` runs this target
without changing the existing suites. Ordinary emulator, vnext, in-memory,
and codec-fuzz jobs do not use that resolver and do not enable the live gate.
Legacy ARM/AAD jobs retain their existing categories.

## vNext emulator conformance

`Cosmos_vnext_emulator` is a Build-only, blocking Linux matrix. It never
provisions an Azure account or uses live-account credentials. Its independent
axes are `AZURE_COSMOS_VNEXT_GATEWAY` (`v1`, `v2`) and
`AZURE_COSMOS_VNEXT_ENCODING` (`text`, `binary`).

The setup helper pins the EN20261008 manifest digest, uses HTTPS, and publishes
the gateway (8081), readiness (8080), and thin-client (10250) ports on loopback.
Readiness failure, conflicting credentials, and unowned or mismatched existing
containers fail setup. One owned container is reused across crates. The final
CI step prints its identity and logs and removes it even after test failure.
Local callers must finalize in the same PowerShell process that ran setup.

V1 cells select the existing `emulator_vnext` suite and explicitly disable V2.
V2 cells select `emulator_vnext_gateway_v2`, which runs focused conformance
instead of the live-account V2 suite. Binary cells additionally select
`emulator_vnext_binary`. Both test frameworks honor the selected encoding
default while preserving explicit test overrides. The original flat/HPK binary
write regression is active in binary cells.

The SDK `vnext` target requires a local HTTPS endpoint and required test mode.
It checks actual completed-request transport, point and query response encoding,
CRUD, patch, hierarchical keys, query results/continuations, and batch
atomicity. Missing configuration or V1 fallback in a V2 cell fails rather than
reporting a skipped test as passing. Other emulator, live, and scheduled fuzz
matrices are unchanged.

Run from the repository root in PowerShell 7:

```powershell
. ./sdk/cosmos/eng/scripts/CosmosVNext.ps1
try {
    Start-CosmosVNext -Gateway v1 -Encoding binary
    cargo test -p azure_data_cosmos --all-features --test vnext -- --test-threads=1
    if ($LASTEXITCODE) { throw "vNext conformance failed: $LASTEXITCODE" }
}
finally {
    Complete-CosmosVNext
}
```

Change the two arguments to select another cell. Use a fresh shell or clear
the helper's `RUSTFLAGS` before selecting another cell. For the V1 regression
suite, run `cargo test -p azure_data_cosmos_driver --all-features --test emulator`
and the equivalent `azure_data_cosmos` command before finalization. The helper
uses the standard public emulator key and emulator-only certificate handling;
do not supply cloud credentials.

Run the setup tests without Docker:

```powershell
Invoke-Pester -Path sdk/cosmos/eng/scripts/tests/CosmosVNext.Tests.ps1 -CI
```

### Release compatibility gate

The pinned image is a reproducible baseline, not a claim that these gates are
green. On ARM64, its HTTPS/HTTP2 thin-client listener returns 400 for an empty
`POST /connectivity-probe`, including when the `ConnectivityProbe` operation
header is supplied. Rust therefore correctly falls back to V1; the V2 tests
must remain red until a compatible image is available. Do not bypass the probe
or weaken the transport assertion.

The same image returns all four matching rows without a continuation for an
ordered query with a page-size hint of two. The separate ordered-pagination
gate exposes that missing coverage. Unordered continuation/resume, batch
atomicity (with content responses enabled), and point/query encoding checks
pass in both V1 encoding cells.

Before merging, update the pinned digest to a verified release if needed and
obtain green results for every cell on the CI platform. Keep limitations
documented instead of turning a required scenario into a silent skip.

## Adding or rotating an account

See `account-provisioning/README.md`.

## What this does *not* solve

This mechanism only covers **key-based** tests. AAD-specific behavior
requires a real Entra ID identity/tenant to authenticate against; fixed
accounts don't remove that requirement, they only remove it for the tests
that don't need AAD in the first place. `Cosmos_live_test_aad` still depends
on the ephemeral tenant and the `azure-sdk-tests-cosmos` service connection,
exactly as it did before this change.

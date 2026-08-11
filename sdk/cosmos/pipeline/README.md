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
   - `ACCOUNT_HOST`, `DATABASE_NAME`, `AZURE_COSMOS_DEFAULT_CONSISTENCY`,
     `COSMOS_RUSTFLAGS`

   These are exactly the environment variables that
   `sdk/cosmos/test-resources.bicep` used to produce as ARM deployment
   outputs, so the test frameworks
   (`azure_data_cosmos`/`azure_data_cosmos_driver`) need no changes.
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
pwsh sdk/cosmos/pipeline/resolve-cosmos-test-account.tests.ps1
```

You can also invoke the resolver directly against the sample JSON:

```powershell
$env:COSMOS_ACCOUNTS_LOCAL = 'true'
$env:COSMOS_ACCOUNT_SELECTOR = 'session-singlewrite'
$env:COSMOS_TEST_ACCOUNTS_JSON = Get-Content -Raw sdk/cosmos/pipeline/live-test-accounts.sample.json
./sdk/cosmos/pipeline/resolve-cosmos-test-account.ps1
```

## Adding or rotating an account

See `account-provisioning/README.md`.

## What this does *not* solve

This mechanism only covers **key-based** tests. AAD-specific behavior
requires a real Entra ID identity/tenant to authenticate against; fixed
accounts don't remove that requirement, they only remove it for the tests
that don't need AAD in the first place. `Cosmos_live_test_aad` still depends
on the ephemeral tenant and the `azure-sdk-tests-cosmos` service connection,
exactly as it did before this change.

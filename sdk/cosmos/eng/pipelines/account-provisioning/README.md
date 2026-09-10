# Cosmos live-test account provisioning

Creates and rotates the fixed, self-owned Cosmos DB accounts used by the
`Cosmos_live_test` leg (see `../README.md` for the overall
mechanism this supports).

## Prerequisites

- PowerShell 7+
- `Az.Accounts`, `Az.Resources`, `Az.CosmosDB` modules
  (`Install-Module Az.Accounts, Az.Resources, Az.CosmosDB -Scope CurrentUser`)
- Contributor access on a **permanent, self-owned subscription** - not the
  ephemeral test tenant used by `Cosmos_live_test_aad`. Using a permanent
  subscription is the entire point: it means these accounts, and the secret
  that references them, never need to be touched when the ephemeral tenant
  rotates.

## Usage

```powershell
# Dry run first - creates nothing, shows what would happen, keys are stubbed
./New-CosmosLiveTestAccounts.ps1 -SubscriptionId <sub-id> -WhatIf

# Preferred: create/refresh accounts and push the JSON straight to the Key
# Vault secret using your own `Connect-AzAccount` session (requires Key Vault
# Secrets Officer on the vault). The secret never touches a local file or the
# console.
./New-CosmosLiveTestAccounts.ps1 -SubscriptionId <sub-id> -KeyVaultName <kv-name>

# Fallback: create/refresh accounts and write the JSON to a local file to
# paste into the secret manually (see "Updating the secret" below)
./New-CosmosLiveTestAccounts.ps1 -SubscriptionId <sub-id> -OutputPath ./accounts.json
```

The script is idempotent: accounts that already exist are validated
all-or-nothing against the required capabilities (nothing is reconciled in
place - a drifted account fails the run with instructions to delete and
recreate it), but the shared test database is created if missing, and
endpoint/keys are always re-read so the emitted JSON is current.

## Updating the secret

Prefer `-KeyVaultName` (see Usage above): it signs in with your own `az`/`Az`
session and writes a new version of the `rust-ci` **Key Vault secret**
directly, so the secret spends the least possible time visible to a human.

If you can't use `-KeyVaultName`, copy the JSON the script prints (or the
contents of `-OutputPath`) into the `rust-ci` Key Vault secret manually
(portal, `az keyvault secret set`, or the KV REST API). Either way, the
secret backs the `Test Secrets for Cosmos Live Tests - user administered`
variable group; the ADO variable group only *links* to the Key Vault secret
(read-through mapping), so editing the value in the variable-group UI has no
effect. The next pipeline run picks up a new KV secret version automatically.

Treat the JSON as a secret at every step - it contains account keys.

## Adding a new account / rotating a key

1. Add (or edit) an entry in
   `cosmos-live-test-accounts.definition.json`.
   The `name` becomes the `AccountSelector` that
   `sdk/cosmos/live-platform-matrix.json` must reference.
2. Re-run the script with `-KeyVaultName` (see Usage above), or without it
   and update the secret manually (see "Updating the secret" above).
3. If you added a new selector, add a matching `AccountSelector` entry to the
   relevant leg in `sdk/cosmos/live-platform-matrix.json`.

To rotate a compromised or expiring key, use the Azure Portal or
`New-AzCosmosDBAccountKey` to regenerate it, then re-run this script with
`-KeyVaultName` (which re-reads the current keys and pushes the new secret
version directly).

## Cleaning up stale test databases

Live tests create per-run databases named `test-db-<run-id>` (see
`azure_data_cosmos_driver/tests/framework/test_client.rs`) and normally
delete them at the end of the run. A cancelled or crashed run skips that
in-process cleanup, so databases can accumulate on the fixed accounts over
time. Run `Remove-StaleCosmosTestDatabases.ps1` periodically (manually, or
from a scheduled job once a service connection for the `sdk-ci` subscription
exists) to delete `test-db-*` databases older than a threshold:

```powershell
# Dry run - lists what would be deleted, deletes nothing
./Remove-StaleCosmosTestDatabases.ps1 -SubscriptionId <sub-id> -WhatIf

# Delete test-db-* databases older than 6 hours (default) on every sdkci-* account
./Remove-StaleCosmosTestDatabases.ps1 -SubscriptionId <sub-id>
```

## Why this script exists

The previous live-test flow deployed a fresh Cosmos account per CI run via
`sdk/cosmos/test-resources.bicep`, against whatever tenant the
`azure-sdk-tests-cosmos` service connection was federated to at the time.
That tenant is ephemeral and rotates periodically, and rotating it requires
recreating the ADO service connection - an admin operation, not something
that can be done from a pipeline. Fixed, self-owned accounts in a permanent
subscription sidestep that dependency entirely for key-based tests.

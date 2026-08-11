# Cosmos live-test account provisioning

Creates and rotates the fixed, self-owned Cosmos DB accounts used by the
`Cosmos_live_test` leg (see [`../README.md`](../README.md) for the overall
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

# Create/refresh accounts and write the JSON to a local file
./New-CosmosLiveTestAccounts.ps1 -SubscriptionId <sub-id> -OutputPath ./accounts.json
```

The script is idempotent: accounts that already exist are left alone (their
configuration is not reconciled), but the shared test database is created if
missing, and endpoint/keys are always re-read so the emitted JSON is current.

## Updating the secret

The script does **not** write to any ADO variable group itself - copy the
JSON it prints (or the contents of `-OutputPath`) into the
`rust-ci` secret variable in the
`Test Secrets for Cosmos Live Tests - user administered` variable group (or
whichever group `sdk/cosmos/pipeline/resolve-test-account-steps.yml` is
pointed at). Treat the JSON as a secret at every step - it contains account
keys.

## Adding a new account / rotating a key

1. Add (or edit) an entry in
   [`cosmos-live-test-accounts.definition.json`](cosmos-live-test-accounts.definition.json).
   The `name` becomes the `AccountSelector` that
   `sdk/cosmos/live-platform-matrix.json` must reference.
2. Re-run the script (see Usage above).
3. Update the ADO secret with the new JSON.
4. If you added a new selector, add a matching `AccountSelector` entry to the
   relevant leg in `sdk/cosmos/live-platform-matrix.json`.

To rotate a compromised or expiring key, use the Azure Portal or
`New-AzCosmosDBAccountKey` to regenerate it, then re-run this script (which
re-reads the current keys) and update the secret.

## Why this script exists

The previous live-test flow deployed a fresh Cosmos account per CI run via
`sdk/cosmos/test-resources.bicep`, against whatever tenant the
`azure-sdk-tests-cosmos` service connection was federated to at the time.
That tenant is ephemeral and rotates periodically, and rotating it requires
recreating the ADO service connection - an admin operation, not something
that can be done from a pipeline. Fixed, self-owned accounts in a permanent
subscription sidestep that dependency entirely for key-based tests, mirroring
the approach taken in
[azure-sdk-for-java#49735](https://github.com/Azure/azure-sdk-for-java/pull/49735).

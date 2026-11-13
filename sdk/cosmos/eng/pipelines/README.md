# Cosmos test pipelines

## Fixed live-test accounts

`Cosmos_live_test` uses fixed, key-authenticated accounts rather than deploying
accounts per run. AAD and partition-merge jobs still require federated Azure
authentication and per-run ARM provisioning.

Account selectors in `sdk/cosmos/live-platform-matrix.json` map to entries in
the `rust-ci` Key Vault secret, exposed through the
`Test Secrets for Cosmos Live Tests - user administered` ADO variable group.
`resolve-test-account-steps.yml` invokes `resolve-cosmos-test-account.ps1` to
export credentials, test categories, and account metadata.
Rotate the Key Vault secret, not the variable-group value.

For account provisioning and rotation, see the
[provisioning runbook](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/eng/pipelines/account-provisioning/README.md).
The JSON schema and sample are in this directory.

## Local testing

Run commands from the repository root in PowerShell 7.
The resolver tests require no Azure access:

```powershell
pwsh sdk/cosmos/eng/pipelines/resolve-cosmos-test-account.tests.ps1
```

### Live-only query tests

Set `AZURE_COSMOS_CONNECTION_STRING` securely to a real account with permission
to create and delete databases and containers, then run:

```powershell
$env:AZURE_COSMOS_TEST_MODE = 'required'
$env:RUSTFLAGS = '--cfg=test_category="live"'
cargo test -p azure_data_cosmos --all-features --test live live_distinct_admission_and_per_query_options
```

The test cleans up its unique database. It is ignored without the `live`
category; explicit execution rejects missing configuration and emulator
endpoints. Fixed-account CI enables this category automatically.

## vNext emulator conformance

`Cosmos_vnext_emulator` is a blocking, Build-only Linux matrix:
`AZURE_COSMOS_VNEXT_GATEWAY` (`v1`/`v2`) ×
`AZURE_COSMOS_VNEXT_ENCODING` (`text`/`binary`).
Tests assert actual transport and encoding, not just successful operations.

Docker is required. Start in a fresh shell without cloud credentials,
`ACCOUNT_HOST`, non-key `AZURE_COSMOS_AUTH_MODE`, `CARGO_ENCODED_RUSTFLAGS`, or
existing test-category `RUSTFLAGS`. The helper configures local HTTPS and
emulator-only certificate handling:

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

Change the gateway and encoding arguments for another cell, using a fresh
shell each time. Finalize in the same PowerShell process as setup.
CI starts the emulator per crate; the crate cleanup hook collects logs and
removes it. Build or test failures can exit before that hook runs.
Setup and cleanup reject unowned containers.

For V1 regression coverage, also run the `emulator` target for
`azure_data_cosmos` and `azure_data_cosmos_driver` before finalization.

### Known gaps

- The helper pins EN20261008. Its V2 connectivity probe failed on ARM64;
  verify a compatible image and all enabled scenarios on CI before merging.
  Do not bypass the probe or weaken transport assertions.
- Ordered-pagination tests are ignored in both gateways:
  [Azure/azure-cosmos-db-emulator-docker#387](https://github.com/Azure/azure-cosmos-db-emulator-docker/issues/387).
- Batch tests are ignored only in V2 pending emulator JSON batch-read support;
  SDK batch atomicity tokens are also required.

Both ignores apply to text and binary. After fixes, run the regressions with
the selected cell active, then remove the corresponding ignores:

```powershell
cargo test -p azure_data_cosmos --all-features --test vnext -- --ignored --test-threads=1
```

## Gateway V2 multi-range prefix change feed

Use an existing Gateway V2 container with MultiHash V2 keys
`/tenant`, `/user`, `/device` and a tenant prefix spanning at least two physical
ranges. The test validates this topology; it does not provision or split it.
Set `AZURE_COSMOS_GW_V2_ENDPOINT` and `AZURE_COSMOS_GW_V2_KEY` securely, then run:

```powershell
$env:AZURE_COSMOS_GW_V2_PREFIX_DATABASE = '<existing-database>'
$env:AZURE_COSMOS_GW_V2_PREFIX_CONTAINER = '<existing-container>'
$env:AZURE_COSMOS_GW_V2_PREFIX_TENANT = '<tenant-spanning-physical-ranges>'
$env:RUSTFLAGS = '--cfg=test_category="gateway_v2_prefix"'
cargo test -p azure_data_cosmos --all-features --test gateway_v2 gateway_v2_prefix_change_feed_across_physical_ranges
```

Run serially with no other writers on that prefix. The test checks fan-out,
checkpoint/resume, exact membership, and V2 transport, then deletes only its
three items, not the fixture. It is ignored without the explicit category.
Split-during-read recovery belongs to the separate `split` suite.

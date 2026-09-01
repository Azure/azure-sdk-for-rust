---
name: emulator-tests
description: >
  Verify an existing Azure Cosmos DB Emulator and run the relevant Cosmos SDK
  integration tests against it.
disable-model-invocation: false
---

# Run Cosmos emulator tests

Use this skill for behavior that crosses the public SDK boundary and needs a
real Cosmos HTTP endpoint. Emulator tests are the gold-standard for evaluating
integration with a real Cosmos HTTP endpoint.

## Emulator prerequisite

Expect the user to have already installed and started an Azure Cosmos DB
Emulator that is reachable at `https://localhost:8081`. This may be a Windows
emulator reached from WSL2 or another emulator provided by the user.

Do not install, launch, restart, reconfigure, or manage an emulator or Docker
container. Never print `AZURE_COSMOS_CONNECTION_STRING`; it can contain a live
account key.

## Confirm emulator availability

Before configuring or running tests, run:

```bash
curl --insecure -v https://localhost:8081
```

The emulator uses a self-signed certificate, so `--insecure` is expected.
Confirm that:

- `curl` connects to `localhost` on port `8081`;
- TLS negotiation succeeds; and
- the server returns an HTTP response.

An unauthenticated `401 Unauthorized` response is expected and confirms that
the emulator gateway is listening. Emulator response headers such as
`x-ms-activity-id` or `x-ms-gatewayversion` provide additional confirmation.

If the command cannot connect, TLS negotiation fails, or no HTTP response is
returned, do not run the emulator tests. Report that the emulator tests could
not run because no emulator was available at `https://localhost:8081`, then ask
the user to make the emulator available or approve skipping those tests.

## Configure the tests

The test framework recognizes the literal `emulator` connection string and
expands it to the local HTTPS endpoint and well-known emulator test key.

On Bash:

```bash
case "${AZURE_COSMOS_CONNECTION_STRING:-}" in
    '') export AZURE_COSMOS_CONNECTION_STRING='emulator' ;;
    emulator) ;;
    *)
        echo 'Existing connection string is not the local emulator.' >&2
        exit 1
        ;;
esac
unset AZURE_COSMOS_EMULATOR_FLAVOR
export AZURE_COSMOS_TEST_MODE='required'
export AZURE_COSMOS_AUTH_MODE='key'
export RUST_TEST_THREADS='1'
export RUSTFLAGS="${RUSTFLAGS:-} --cfg=test_category=\"emulator\""
unset AZURE_COSMOS_ALLOW_INVALID_CERT
```

On PowerShell:

```powershell
if (-not $env:AZURE_COSMOS_CONNECTION_STRING) {
    $env:AZURE_COSMOS_CONNECTION_STRING = 'emulator'
}
elseif ($env:AZURE_COSMOS_CONNECTION_STRING -ne 'emulator') {
    throw 'Existing connection string is not the local emulator.'
}

$env:AZURE_COSMOS_EMULATOR_FLAVOR = $null
$env:AZURE_COSMOS_TEST_MODE = 'required'
$env:AZURE_COSMOS_AUTH_MODE = 'key'
$env:RUST_TEST_THREADS = '1'
$env:RUSTFLAGS = "$($env:RUSTFLAGS) --cfg=test_category=`"emulator`""
$env:AZURE_COSMOS_ALLOW_INVALID_CERT = $null
```

The well-known emulator key is test-only and is not a live credential.
`DATABASE_NAME` is optional; the framework otherwise uses `emulator-test-db`.

## Run the tests

Run the external emulator integration-test target:

```bash
cargo test -p azure_data_cosmos \
    --features key_auth,control_plane,fault_injection \
    --test emulator \
    -- --test-threads=1
```

Add the feature associated with the changed behavior when needed; for example,
add `preview_patch` for PATCH tests. Do not add the `multi_write` category:
those tests require a live multi-region account.

Start with the smallest matching module or test-name filter, then run the full
`emulator` target before completion:

```bash
cargo test -p azure_data_cosmos \
    --features key_auth,control_plane,fault_injection \
    --test emulator cosmos_query \
    -- --test-threads=1
```

Choose tests by the behavior changed:

- database, container, or throughput behavior:
  `cosmos_databases`, `cosmos_containers`, or `cosmos_offers`;
- item, batch, partition-key, or PATCH behavior:
  `cosmos_items`, `cosmos_batch`, `cosmos_partition_key_types`,
  `cosmos_hpk`, or `cosmos_patch`;
- query, feed-range, or change-feed behavior:
  `cosmos_query`, `cosmos_query_features`, `cosmos_feed_ranges`, or
  `cosmos_change_feed`;
- routing, metadata, authentication, proxy, or fault behavior:
  the corresponding `cosmos_*` module under
  `azure_data_cosmos/tests/emulator_tests/`.

When adding or changing a test, preserve its existing category gates. Use
`test_category = "emulator"` only when the available emulator supports the
asserted behavior, and keep an explicit ignore reason for known emulator
divergence.

## Sources of truth

- Test categories:
  `sdk/cosmos/azure_data_cosmos/build.rs`
- Required test features:
  `sdk/cosmos/azure_data_cosmos/Cargo.toml`
- Environment handling:
  `sdk/cosmos/azure_data_cosmos/tests/framework/test_client.rs`
- External test inventory:
  `sdk/cosmos/azure_data_cosmos/tests/emulator_tests/`

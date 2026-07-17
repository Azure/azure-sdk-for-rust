# azure_data_cosmos_emulator

Out-of-process host for the Azure Cosmos DB Rust SDK's in-memory test emulator
(`azure_data_cosmos_driver::in_memory_emulator`). It exposes the same
deterministic, memory-backed emulator used by the driver's own Rust unit
tests over real network ports, so any client — not just Rust — can exercise
it over the Cosmos DB wire protocol.

This is an **SDK engineering and test tool**, not a supported customer
product: it provides no service compatibility, durability, performance, or
support guarantees. See
[`docs/adr/001_build_memory_backed_sdk_test_emulator.md`](docs/adr/001_build_memory_backed_sdk_test_emulator.md)
for the full rationale and scope, and [`docs/plan.md`](docs/plan.md) for the
complete design (configuration schema, management REST API, Gateway 2.0
support, and CI integration).

`publish = false` — this crate is never published to crates.io.

## What it hosts

- **Gateway V1** (JSON REST) on every configured region — always on.
- **Gateway 2.0** (RNTBD over cleartext HTTP/2) per region, when the region's
  config sets `gateway20Port`.
- A **management REST API** for emulator-only control-plane actions with no
  Cosmos gateway equivalent: partition split/merge (as long-running
  operations), per-partition-failover toggling, and replication pause/resume.

## Quick start

Build and run against one of the sample configs checked into
[`config/`](config/):

```sh
cargo build -p azure_data_cosmos_emulator
./target/debug/azure_data_cosmos_emulator --config sdk/cosmos/azure_data_cosmos_emulator/config/ci-gateway-v1.json
```

The host writes one JSON `ready` record to stdout once every listener is
bound:

```json
{
  "event": "ready",
  "managementEndpoint": "http://127.0.0.1:49150/",
  "accountEndpoint": "http://127.0.0.1:49151/",
  "regions": [
    { "name": "East US", "gatewayEndpoint": "http://127.0.0.1:49151/" }
  ]
}
```

Logs go to stderr, so automation can parse stdout without filtering. Use the
resolved `accountEndpoint` as a normal Cosmos DB connection string endpoint,
and the `managementEndpoint` to drive control-plane actions, e.g.:

```sh
curl -X POST "http://127.0.0.1:49150/databases/testdb/containers/testcoll/partitions/0/split"
```

## Configuration

A single JSON file (`--config`) describes the account topology, the
databases/containers to create, and optional seed items — all applied on
startup. See [`docs/plan.md`](docs/plan.md#4-configuration-file) for the full
field reference; [`config/ci-gateway-v1.json`](config/ci-gateway-v1.json) and
[`config/ci-gateway-v2.json`](config/ci-gateway-v2.json) are minimal,
CI-oriented examples (both use port `0` for every listener, so the OS assigns
free ports and the resolved endpoints come from the `ready` record).

## Testing

```sh
cargo test -p azure_data_cosmos_emulator --all-features
```

The hosted emulator is also exercised end-to-end by
`azure_data_cosmos_driver`'s and `azure_data_cosmos`'s existing emulator test
suites, gated behind `test_category = "emulator_inmemory"` (and
`"emulator_inmemory_gateway_v2"` for Gateway 2.0-specific tests). See
[`sdk/cosmos/eng/scripts/Invoke-CosmosTestSetup.ps1`](../eng/scripts/Invoke-CosmosTestSetup.ps1)
for how CI builds, starts, and health-checks the host process before running
those suites.

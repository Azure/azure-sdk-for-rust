# azure_data_cosmos_emulator

Out-of-process host for the Azure Cosmos DB Rust SDK's in-memory test emulator
(`azure_data_cosmos_driver::in_memory_emulator`). It exposes the same
deterministic, memory-backed emulator used by the driver's own Rust unit
tests over real network ports, so any client — not just Rust — can exercise
it over the Cosmos DB wire protocol.

> This is an SDK engineering and test tool, not a supported Azure product. It
> provides no service compatibility, durability, performance, or support
> guarantees.

For the full design — rationale and scope, configuration schema, management
REST API, Gateway 2.0 support, and CI integration — see [`AGENTS.md`](AGENTS.md)
and the architecture decision records under [`docs/adr/`](docs/adr/).

## What it hosts

- **Gateway V1** (JSON REST) on every configured region — always on.
- **Gateway 2.0** (RNTBD over cleartext HTTP/2) per region, when the region's
  config sets `gateway20Port`.
- A **management REST API** for emulator-only control-plane actions with no
  Cosmos gateway equivalent: partition split/merge (as long-running
  operations), per-partition-failover toggling, and replication pause/resume.

## Quick start

Run against one of the sample configs checked into `config/`:

```sh
cargo run -p azure_data_cosmos_emulator -- --config sdk/cosmos/azure_data_cosmos_emulator/config/ci-gateway-v1.json
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
startup. See `AGENTS.md#4-configuration-file` for the full field
reference; `config/ci-gateway-v1.json` and
`config/ci-gateway-v2.json` are minimal,
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
`sdk/cosmos/eng/scripts/Invoke-CosmosTestSetup.ps1`
for how CI builds, starts, and health-checks the host process before running
those suites.

## Contributing

This project welcomes contributions and suggestions. Most contributions require
you to agree to a Contributor License Agreement (CLA) declaring that you have
the right to, and actually do, grant us the rights to use your contribution. For
details, visit [https://cla.microsoft.com](https://cla.microsoft.com).

When you submit a pull request, a CLA-bot will automatically determine whether
you need to provide a CLA and decorate the PR appropriately (e.g., label,
comment). Simply follow the instructions provided by the bot. You'll only need
to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information, see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/)
or contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any
additional questions or comments.

# cosspec

`cosspec` holds the high-level specification tests for the Azure Cosmos DB
Rust SDK. Spec tests are short, backend-agnostic end-to-end tests. Each one is
a human-verifiable specification of one behavior of the SDK. This crate
contains the spec-test DSL and runner, and the specs themselves in
`tests/spec`.

This crate is test infrastructure (`publish = false`); it is not part of the
supported SDK surface. See [spec 0030] for the design, the full DSL reference,
and the roadmap.

## Getting started

### Prerequisites

- The Rust toolchain pinned by this repository.

No Azure resources are needed. By default, specs run against the in-process
in-memory emulator. To run them on the [Azure Cosmos DB Emulator] or on live
accounts, see [Choose backends](#choose-backends).

### Run the specs

```sh
cargo test -p cosspec --test spec
```

Standard `libtest` filters and flags apply. For example, `-- split` runs only
specs whose name contains `split`, and `-- --include-ignored` also runs
skipped specs and specs with known issues.

| Variable | Effect |
| --- | --- |
| `AZURE_COSMOS_SPEC_PROFILES` | Comma-separated service profiles to run on every backend, such as `SingleRegion,MultiRead` |
| `AZURE_COSMOS_SPEC_STRICT` | `1` turns skips for missing capabilities into failures |
| `AZURE_COSMOS_SPEC_UPDATE_CATALOG` | `1` rewrites `CATALOG.md` and `catalog.json` |

Every spec that runs writes a JSON result file. The file records whether the
spec passed, any error or panic, the resources it created or borrowed from
the pool, and every HTTP request made for it. Results go to
`target/cosspec-results/<run-id>/` unless you pass a directory:

```sh
cargo test -p cosspec --test spec -- --results-dir ./spec-results
```

### Choose backends

Pass a TOML configuration file with `--config` to choose the backends and the
service profiles each one runs. Backends run in file order. Relative paths are
resolved from this directory. `configs/` has examples:

```sh
# A running Azure Cosmos DB Emulator; SingleRegion only.
cargo test -p cosspec --test spec -- --config ./configs/emulator.toml

# Live accounts, one per service profile.
cargo test -p cosspec --test spec -- --config ./configs/live.toml
```

| Backend (`kind`) | Profiles | Capabilities beyond the profile's |
| --- | --- | --- |
| `inmemory` (default) | All | `TopologyControl`, `SimpleSplitMerge`, `RegionControl`, `AccountIsolation` |
| `emulator` | `SingleRegion` only | None |
| `live` | One account per configured profile | None |

Specs that need a capability the backend lacks are skipped. Trial names
include the backend and profile, such as `[emulator.SingleRegion]`, so
`-- emulator.` runs only the emulator's trials.

To create live accounts, deploy `infra/main.bicep`. It creates one account
per service profile, in the regions the profiles expect, and outputs their
endpoints:

```sh
az deployment group create --resource-group <group> --template-file infra/main.bicep
```

Put the endpoints in a copy of `configs/live.toml`. Export each account's key
(`az cosmosdb keys list`) into the environment variable its `key_env` names,
or use `auth = { kind = "entra" }` to sign in with the Azure CLI. Specs create
`spec-<UUIDv7>` databases in these accounts and delete them afterward. Leftovers
older than two hours are deleted by later runs.

## Key concepts

- **Spec**: an `async fn` marked `#[spec_test]` whose doc comment states the
  behavior and whose body demonstrates it.
- **Service profile**: an account shape (`SingleRegion`, `MultiRead`, or
  `MultiWrite`). Each spec runs once per selected profile.
- **Backend**: where specs run: the in-memory emulator, the Azure Cosmos DB
  Emulator, or live accounts. A run can use several.
- **Capability**: something a backend can do, such as `SimpleSplitMerge`. A
  spec declares the capabilities it needs with `#[requires(..)]`. Specs that
  need a missing capability are skipped. A spec that uses a capability it
  didn't declare fails.
- **Injectables**: `SharedContainer`, `ExclusiveContainer`,
  `ExclusiveDatabase`, `ExclusiveAccount`, and `HttpTap` parameters are
  provisioned by the framework and cleaned up after the spec.
- **Catalog**: `CATALOG.md` and `catalog.json` list every spec. They are
  generated from the specs and checked by the `catalog::up_to_date` trial.

## Examples

### Write a spec

```rust ignore
use cosspec::{spec_test, testdata::Item, SharedContainer, SpecResult};

/// A point read returns the stored item unchanged.
#[spec_test]
pub async fn point_read_returns_stored_item(
    #[profile(basic_items)] container: SharedContainer,
) -> SpecResult {
    let expected = container.dataset().get("item-07");

    let response = container
        .client()
        .read_item(expected.partition_key(), &expected.id, None)
        .await?;

    assert_eq!(response.into_model::<Item>()?, expected);
    Ok(())
}
```

Add the spec to a module under `tests/spec`, then regenerate the catalog:

```sh
AZURE_COSMOS_SPEC_UPDATE_CATALOG=1 cargo test -p cosspec --test spec
```

## Troubleshooting

### General

- If `catalog::up_to_date` fails, regenerate the catalog as shown above and
  commit `CATALOG.md` and `catalog.json`.
- If a spec fails with `InvalidConfiguration`, it called a method that needs a
  capability it didn't declare. Add the capability to `#[requires(..)]`.
- If a spec fails after writing to a `SharedContainer`, use an
  `ExclusiveContainer` for specs that write.
- To see why a spec failed, open its result file. `error` is the spec body's
  failure and `teardown_error` is a cleanup failure. In `http`, requests
  labeled `spec` came from the spec's clients and requests labeled `harness`
  came from the framework's setup and teardown.
- If an `emulator` trial fails with `error sending request`, check that the
  emulator is running and reachable at the configured `endpoint`.
- If a `live` trial fails with `environment variable ... is not set`, export
  the key named by that profile's `key_env`.

## Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to these libraries.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit <https://opensource.microsoft.com/cla/>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

<!-- LINKS -->
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/
[Azure Cosmos DB Emulator]: https://learn.microsoft.com/azure/cosmos-db/emulator
[spec 0030]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/docs/specs/0030-cosmos-spec-tests.md

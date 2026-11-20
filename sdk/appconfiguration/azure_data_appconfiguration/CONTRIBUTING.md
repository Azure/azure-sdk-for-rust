# Contributing

See the [repository contribution guide](https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md)
for shared setup and contribution workflows.

## Prerequisites

Install the repository-pinned Rust toolchain, Node.js, and PowerShell. Playback
tests acquire Test Proxy automatically and don't require Azure resources or
credentials. Recording requires an existing App Configuration store and an
identity with the App Configuration Data Owner role on that store.

The crate uses local Azure Core and Azure Identity dependencies to match the
types used by the repository's `azure_core_test` infrastructure.

## Generation

`tsp-location.yaml` identifies the App Configuration TypeSpec project and its
base commit. The Rust emitter options, public-name customizations, and content-type
variant names used for this initial generation are local spec changes, not part
of that commit.
Update the pinned commit after those changes are committed upstream.

Rust-scoped `@@clientName` customizations in `client.tsp` align public names with
the .NET SDK, including `ConfigurationClient`, `ConfigurationSetting`,
`ConfigurationSnapshot`, and `SettingLabel`. Collection methods retain Rust's
`list_*` convention. The two lock operations are named `create_read_only_lock`
and `delete_read_only_lock`; renaming alone doesn't add .NET's combined
`SetReadOnly` convenience method. Wire names and other language SDKs are unchanged.

Until then, regenerate from a local spec project containing those changes.
Run these commands from the SDK repository root, replacing the placeholder with
the directory containing App Configuration's `tspconfig.yaml`:

```powershell
npm ci --prefix .\eng\common\tsp-client
npm exec --prefix .\eng\common\tsp-client --no -- tsp-client update `
    --output-dir .\sdk\appconfiguration\azure_data_appconfiguration `
    --local-spec-repo <local-AppConfiguration-TypeSpec-directory>
```

The SDK repository pins the emitter in `eng/emitter-package.json` and
`eng/emitter-package-lock.json`. Never edit files under `src/generated`.
Keep crate documentation, package metadata, and tests outside that directory.

## Build and offline checks

Run from the SDK repository root:

```powershell
cargo build -p azure_data_appconfiguration
cargo test -p azure_data_appconfiguration --all-features
cargo fmt -p azure_data_appconfiguration -- --check
cargo clippy -p azure_data_appconfiguration --all-features --all-targets
```

The unit tests use an in-memory HTTP transport to verify authentication,
request options, serialization, and pagination. Documentation examples compile
without calling Azure.

## Integration tests

The tests in `tests/app_configuration_client.rs` use `#[recorded::test]` and the
repository's [recording infrastructure](https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md).
The service-level `sdk/appconfiguration/assets.json` pins the published recordings.

The scenarios cover:

- Creating, retrieving, updating, and deleting labeled settings, including a
  not-found response after deletion.
- Create-only writes and conditional updates with current and stale ETags.
- Locking settings, rejecting writes while locked, and unlocking settings.
- Listing settings, keys, and labels with filters.
- Listing setting revisions.

Names use the recording's seeded random generator so parallel test runs don't
collide and playback reproduces the same names. The suite creates six settings
in total and deletes only its own settings. It doesn't create Azure resources or
modify role assignments. Use a dedicated test store; normal service charges and
revision retention still apply.

### Record against an existing store

Sign in with `az login`, then run from the SDK repository root:

```powershell
$env:AZURE_APPCONFIG_ENDPOINT = 'https://<test-store-name>.azconfig.io'
$env:AZURE_TEST_MODE = 'record'
cargo test -p azure_data_appconfiguration --test app_configuration_client --all-features
```

Default Test Proxy sanitizers protect credentials. The tests preserve synthetic
setting keys and names by removing only the `$.key` and `$..name` sanitizers.
Don't put secrets or personal data in test settings, labels, tags, or names.

### Verify playback and publish recordings

Run playback without an endpoint or live credentials:

```powershell
$env:AZURE_TEST_MODE = 'playback'
Remove-Item Env:\AZURE_APPCONFIG_ENDPOINT -ErrorAction SilentlyContinue
cargo test -p azure_data_appconfiguration --all-features
```

After both recording and playback pass, review the sanitized recordings and
publish them with an installed Test Proxy CLI:

```powershell
test-proxy push -a sdk/appconfiguration/assets.json
```

Commit the updated `assets.json` pointer with the tests. Recording files live in
the SDK assets repository, not in this crate's source tree.

### Snapshot testing limitation

Snapshot lifecycle tests are blocked by the generated `begin_create_snapshot`
poller. The service initially returns HTTP `201` with a snapshot whose status is
`provisioning` and an `Operation-Location` header. The generated code interprets
that initial body as an operation-status response and stops before defining its
final result. Awaiting the poller fails instead of following the operation URL.

The emitter needs to distinguish the initial resource response from subsequent
operation-status responses before snapshot lifecycle coverage can be added.
Don't edit `src/generated` to work around this limitation. Exploratory snapshot
tests must archive their snapshots after provisioning completes; App
Configuration doesn't support immediately deleting snapshots.

## Troubleshooting

If regeneration can't find Rust emitter options, make sure the local
spec contains the Rust changes rather than only the base commit in
`tsp-location.yaml`.

Conditional request options pass header values through unchanged. Quote ETags
from response models before assigning them to `if_match`; an unquoted value
doesn't reliably enforce the condition.

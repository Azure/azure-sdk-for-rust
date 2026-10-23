# Contributing to the Key Vault Resource Manager library

Follow the [repository contributing guide](../../../CONTRIBUTING.md) for shared
development, provisioning, and test-proxy workflows. This guide describes
crate-specific generation, tests, and resource requirements.

## Regenerating the library

This library targets stable API version `2026-05-15`. Its source specification
and commit are pinned in [tsp-location.yaml](tsp-location.yaml). The specification
does not yet include Rust emitter configuration.

From the repository root, install the pinned generator:

```sh
npm ci --prefix eng/common/tsp-client
```

Then, from this crate's directory, set `TSP_CLIENT_DIR` to the absolute path of
`eng/common/tsp-client` in your checkout and run:

```sh
npm exec --prefix "$TSP_CLIENT_DIR" --no -- tsp-client update \
  --emitter-options 'crate-name=azure_resourcemanager_keyvault;crate-version=0.1.0;api-version=2026-05-15;emitter-output-dir={output-dir}/sdk/keyvault/azure_resourcemanager_keyvault'
```

Generated constructors, polling methods, polling options, empty-result formats,
and key rotation action wire values currently contain prototype edits.
Regeneration overwrites those edits. [TODO.md](TODO.md) tracks the specification
and emitter changes needed to remove these patches.

After the upstream changes land, update `tsp-location.yaml` and the pinned
emitter as appropriate. Regenerate without overrides, verify that the manual
patches are no longer necessary, export the API, and run package analysis.
Remove obsolete prototype warnings once generation is reproducible.

The proposed upstream SKU enum renames will change Rust references from
`ManagedHsmSkuNameV2` to `ManagedHsmSkuName`, including managed HSM tests. Update
these references after regenerating against that specification change.

## Offline checks

Run these commands from the repository root:

```sh
cargo test -p azure_resourcemanager_keyvault --all-features --lib
cargo test -p azure_resourcemanager_keyvault --all-features --tests --no-run
cargo test -p azure_resourcemanager_keyvault --all-features --doc
```

`tests/readme.rs` also compiles both client-construction examples without running
their credentials or making network requests.

The library has 23 offline tests. Eleven enum-family tests cover all 29 generated
enum types and 155 known wire values, including serialization, deserialization,
unknown values, wrong-case behavior, and representative nested models. These
tests create no Azure resources.

Passing offline enum tests is not evidence of a service/specification mismatch
or a need for additional `client.tsp` customizations. They verify the declared
contract, with the documented rotation-action workaround. Recorded tests compare
actual service wire values with named variants so that extensible enums do not
hide a casing mismatch as `UnknownValue`. Optional fields absent from a response
are not claimed as live coverage.

The service's `CustomizedRecoverable+Purgeable` recovery level is not declared
in the current specification. The seven-day-retention key tests verify its exact
preserved `UnknownValue`, not named-enum compatibility. The name-availability test
remains ignored because the service returns undeclared reason `Invalid`.
[TODO.md](TODO.md) tracks both shared contract corrections.

## Recorded vault tests

Provision the shared Key Vault test resources and configure the test environment
using the repository guide before recording. The tests use the provisioned
subscription, resource group, location, tenant, and test principal values.
When enabled, name-availability coverage also reads `AZURE_KEYVAULT_URL`; it does
not modify that shared fixture.

Vault scenarios are in `tests/vaults.rs`. Each mutating test owns its vault;
multiple key cases within one test reuse that vault. Tests must not depend on
execution order or share mutable vaults.

| Test | Scenarios | New Standard vault names |
| --- | --- | ---: |
| `key_lifecycle` | RSA operations, list/get, and key versions | 1 |
| `ec_key_lifecycle` | Four curves, signing operations, and key versions | 1 |
| `key_rotation_policy` | Notification and rotation actions on the key-level GET; versioned key retrieval | 1 |
| `secret_lifecycle` | Create, list, update with value, and get | 1 |
| `vault_network_rules` | ACL actions and bypass options | 1 |
| `vault_access_policies` | Add/replace/remove; key, secret, certificate, and storage permissions | 1 |
| `vault_recovery` | Create, delete, recover the same name, and delete again | 1 |
| `vault_name_availability` (ignored) | Invalid and already-existing names; pending the specification correction | 0 |

The vault target creates seven unique Standard vaults, six keys, and one secret.
The recovery scenario uses one vault name through two provisioning cycles.
The separate `tests/readme.rs` target creates one more vault. A full live run
therefore creates eight unique Standard vaults with nine provisioning cycles.
Deletion is soft deletion, not purge. Premium vaults, real private endpoints,
multi-region deployments, and purge coverage are not included.

Run recording and playback explicitly:

```sh
AZURE_TEST_MODE=record cargo test -p azure_resourcemanager_keyvault --all-features --test vaults
AZURE_TEST_MODE=playback cargo test -p azure_resourcemanager_keyvault --all-features --test vaults
```

Recording creates resources; playback does not. Limit the test filter when
investigating one scenario, and inspect a new service failure before rerunning
or changing expected wire values.

Test setup removes `SANITIZE_BODY_NAME` because its recursive path also replaces
SKU enum values. This preserves the generated test resource names and exact SKU
spellings needed by playback assertions. Other default sanitizers remain active.

### Recording migration

Moving from `tests/lifecycle.rs` changes recording identifiers to `vaults/...`
and `managed_hsms/...`. Record and play back new or expanded scenarios before
replacing usable old recordings; do not fabricate recordings or assume the old
request sequence matches the expanded tests. After recording and playback
succeed, update the recording assets through the repository test-proxy workflow.

## Managed HSM tests

All three tests in `tests/managed_hsms.rs` remain ignored:

| Test | Prerequisites and resource use |
| --- | --- |
| `managed_hsm_lifecycle` | Creates one HSM; verifies SKU and returned lifecycle metadata, then deletes it |
| `managed_hsm_network_rules` | Creates one HSM; exercises ACL and public-network settings, then deletes it |
| `managed_hsm_key_lifecycle` | Requires a third, dedicated activated disposable HSM; creates two keys |

The first two tests require service availability, regional quota, and the
provisioned test principal. The key test additionally requires
`KEYVAULT_MANAGED_HSM_NAME` and `KEYVAULT_MANAGED_HSM_DISPOSABLE=true`.
Its provisioning owner must dispose of that fixture and its keys after use.
These management clients do not provide a key-delete operation; the test must
not run against a shared or user-owned HSM.

No security-domain activation, private networking, or multi-region provisioning
is performed by these tests. Their service behavior remains unverified until
the dedicated provisioning and recording requirements are met.

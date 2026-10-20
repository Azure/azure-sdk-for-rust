# Resource Manager generation follow-up

<!-- cspell:ignore tspconfig -->

Track changes needed in the specification and Rust emitter to regenerate
`azure_resourcemanager_keyvault` without manual generated-source patches.
Local development, test coverage, and recording instructions belong in
[CONTRIBUTING.md](CONTRIBUTING.md), not this checklist.

## Azure/azure-rest-api-specs

Repository: <https://github.com/Azure/azure-rest-api-specs>

TypeSpec folder:
[`specification/keyvault/resource-manager/Microsoft.KeyVault/KeyVault`](https://github.com/Azure/azure-rest-api-specs/tree/e35ce3d9e784cc6c298ae85608b9f79e7c833d28/specification/keyvault/resource-manager/Microsoft.KeyVault/KeyVault)

Current source commit: `e35ce3d9e784cc6c298ae85608b9f79e7c833d28`.
The Rust prototype targets stable API version `2026-05-15`.

### Emitter configuration

- [ ] Add Rust configuration to **`tspconfig.yaml`** (the existing filename is not
  `tspconfig.yml`). Retain the other languages' options and existing `emit` list;
  `tsp-client` selects the Rust emitter explicitly. Add the following options:

  ```yaml
  options:
    "@azure-tools/typespec-rust":
      crate-name: "azure_resourcemanager_keyvault"
      crate-version: "0.1.0"
      api-version: "2026-05-15"
      emitter-output-dir: "{output-dir}/{service-dir}/{crate-name}"
  ```

- [ ] Retain the existing `service-dir` default of `sdk/keyvault`, and verify
  `tsp-client init` and `tsp-client update` discover the Rust output directory
  without command-line emitter overrides. The prototype needed overrides because
  the pinned configuration has no Rust options.

### Existing client customizations to extend

- [ ] Add `rust` to the language list on **both** managed HSM SKU enum renames
  in `client.tsp`. This aligns Rust with the shared SDK names, not with Go's
  acronym spelling. Update the two existing declarations to:

  ```typespec
  @@clientName(
    ManagedHsmSkuNameV2,
    "ManagedHsmSkuName",
    "csharp,java,python,javascript,rust"
  );
  @@clientName(
    ManagedHsmSkuName,
    "ManagedHsmLegacySkuName",
    "csharp,java,python,javascript,rust"
  );
  ```

Apply the legacy enum rename together with the current enum rename to avoid
name collisions when both types are generated. These are naming improvements,
not fixes for the observed service failures.

The complete pinned `client.tsp` has no existing customization for rotation
action wire values, required secret PATCH fields, polling generation, or cloud
configuration. Adding `rust` to existing language lists cannot supply those fixes.

No other existing language-list extension is required for this prototype.
The `!csharp` scopes already include Rust. Other languages' compatibility
customizations do not need to be copied.

### Model contract corrections requiring new changes

- [ ] Add the observed `CustomizedRecoverable+Purgeable` value to the shared
  `DeletionRecoveryLevel` wire contract. Vaults configured with seven-day soft
  deletion return this value; the generated extensible enum currently preserves
  it as `UnknownValue`. Confirm any additional customized-retention values and
  managed HSM behavior before extending the enum further.
- [ ] Reconcile the vault name-availability `Reason` wire values with the
  service. An invalid name returned `Invalid`, while the specification declares
  `AccountNameInvalid`; the live test is ignored pending a contract correction.
  Confirm the existing-name reason and managed HSM behavior before changing the
  shared enum. Do not hide this mismatch with general casing normalization.
- [ ] Reconcile `KeyRotationPolicyActionType` in `models.tsp` with the service.
  The specification and its generated OpenAPI declare `notify` and `rotate`;
  a recorded successful key GET returned `Notify`. Confirm the service's accepted
  and returned casing for **both** actions, including managed HSM keys.
- [ ] Prefer fixing an incorrect shared wire contract in `models.tsp`. If a
  compatibility customization is necessary in `client.tsp`, make it affect the
  serialized values, not merely the generated member names. A `clientName`
  override alone cannot fix this mismatch. Decide whether lowercase values must
  remain accepted for compatibility before choosing the final representation.
  The local generated-code workaround uses `Notify` and `Rotate`; both have now
  been observed in recorded vault key policy responses. Managed HSM casing
  remains unverified.
- [ ] Verify the secret PATCH contract. A tags-only `SecretPatchParameters`
  request failed with `ParameterNotSpecified`: `properties` was missing.
  Including `properties.contentType` then failed because `value` was missing.
  Determine whether the specification should require both `properties` and
  `properties.value`, or whether the service should accept metadata-only updates.
  The local lifecycle test now sends the test value and content type alongside
  tags; this is a test correction, not a reason to silently inject properties
  or values into generated requests.
- [ ] Regenerate the affected OpenAPI artifacts and run the specification's
  validation after changing shared models or operation metadata.

## Azure/typespec-rust

Repository: <https://github.com/Azure/typespec-rust>

The prototype used `@azure-tools/typespec-rust` version `0.41.5`.

### Emitter configuration

- [ ] Document and test support for the common `api-version` option used by the
  specification configuration. The prototype generated `2026-05-15` with that
  option, but the Rust emitter's own schema does not explicitly declare it.
  Do not introduce an unimplemented Resource Manager mode or `flavor` option.

### Resource Manager polling

- [ ] Detect Resource Manager operations from TypeSpec/client metadata and emit
  `azure_core::http::resourcemanager::new_poller` instead of duplicating polling
  state machines or introducing a separate poller type. Keep returning the
  existing `Poller<OperationStatus>` and preserve lazy initial-request execution.
- [ ] Migrate every applicable operation, including these eight prototype cases:
  vault create/update and purge; managed HSM create/update, update, delete, and
  purge; vault private endpoint connection delete; and managed HSM private
  endpoint connection delete.
- [ ] Map operation metadata to `Option<FinalStateVia<_>>`. The prototype passes
  `Some(FinalStateVia::Location)` for six operations and `None` for managed HSM
  delete and purge. Preserve omission as method/response-based fallback, not an
  `Automatic` variant or an implicit Location selector.
- [ ] Support all helper selectors: Azure async operation, Location, original
  URI, and Operation-Location. For inline Operation-Location results, generate
  the extraction callback from result-path metadata. Preserve response status
  and headers and extract the original JSON bytes without serializing the
  complete model again. No Key Vault operation currently exercises this callback.
- [ ] Generate Resource Manager `PollerOptions` in long-running method options
  and preserve caller context and polling frequency. Leave ordinary method and
  pager options unchanged.
- [ ] Generate `StatusMonitor::Output = ()` with `StatusMonitor::Format = NoFormat`
  for empty final results. The prototype corrected managed HSM delete, managed
  HSM purge, and vault purge. Retain JSON formats for resource results.
- [ ] Delegate protocol selection, polling URLs, provisioning states, API-version
  handling, final GET decisions, and empty responses to the helper. Do not emit
  a competing implementation of those rules. The runtime already removes request
  bodies from polling GETs and retains the initial `Accept` header on every
  subsequent GET.
- [ ] Clean up imports after removing inline polling logic, but retain `json`
  and `RawResponse` wherever pagination still uses them.

### Client constructors, authentication, and package generation

- [ ] Generate Resource Manager constructors that read
  `options.client_options.cloud` and default to Public cloud when omitted.
  Public cloud uses service endpoint `https://management.azure.com/` and token
  scope `https://management.azure.com/.default`.
- [ ] Keep the identity authority, service endpoint, authentication audience, and
  token scope conceptually distinct. Never use the login authority as the
  Resource Manager endpoint, and never derive the endpoint from a token scope.
- [ ] Generate cloud resolution against available core APIs. Use a shared
  Resource Manager configuration helper when one becomes available rather than
  duplicating resolution across service crates. The current core cloud API has
  no separate custom service endpoint; do not emit calls to a nonexistent API.
- [ ] Honor caller-supplied custom-cloud values. The prototype's generated
  contract uses the crate's typed `Audience` as both service root and
  authentication resource, returns an explicit error when it is missing, parses
  the URL, and appends `/.default` without duplicating a trailing slash.
  Configure the credential's identity authority separately. Revisit this
  endpoint/audience coupling when the shared helper is designed.
- [ ] Preserve the current explicit limitation for Government, China, and future
  cloud variants: the prototype uses `todo!()` rather than guessing endpoints.
  Resolve and implement their correct endpoint/audience mappings separately
  before claiming support.
- [ ] Generate accurate constructor documentation for the default cloud,
  custom-cloud requirements, configuration errors, and unsupported-cloud panics.
  Use "Resource Manager" in public documentation; use `resourcemanager` for
  modules and features.
- [ ] Generate a manifest dependency enabling `azure_core`'s `resourcemanager`
  feature, preserve `default-features = false`, and forward default transport
  features consistently. Require a core version containing the helper.
  The prototype temporarily uses local `azure_core` path plus version
  `1.2.0-beta.2`; do not hard-code local paths into published service generation.

### Generation regression coverage

- [ ] Port the prototype's constructor and polling cases into emitter fixtures:
  omitted/Public/custom clouds, missing custom audience, trailing-slash scope
  handling, borrowed context, every long-running method, streamed monitors
  followed by awaiting, explicit final-state metadata, and failure propagation.
- [ ] Cover all polling protocols, repeated GETs without bodies preserving `Accept`,
  resource and empty final results, and Operation-Location inline extraction.
  Keep protocol semantics tested in `azure_core`; emitter tests should verify
  metadata selection and generated integration with that runtime.
- [ ] Add a generation fixture for key action wire-value fidelity and a secret
  PATCH containing tags, explicit properties, and a value. Keep service
  recordings as evidence, not as a source of private payloads in emitter tests.
  Verify that PascalCase Rust identifiers preserve specified wire values;
  do not fix the service mismatch with a global enum-casing transformation.

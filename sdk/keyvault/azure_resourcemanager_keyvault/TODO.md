# Resource Manager generation follow-up

<!-- cspell:ignore tspconfig -->

Track changes needed in the specification and Rust emitter to regenerate
`azure_resourcemanager_keyvault` without manual generated-source patches.
Local development, test coverage, and recording instructions belong in
[CONTRIBUTING.md](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/keyvault/azure_resourcemanager_keyvault/CONTRIBUTING.md), not this checklist.

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
  `azure_core::http::poller::resource_manager::new_poller` instead of duplicating polling
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
- [ ] Support all helper selectors: `AzureAsyncOperation`, `Location`, `OriginalUrl`,
  and `OperationLocation`. For inline Operation-Location results, generate
  the extraction callback from result-path metadata. Preserve response status
  and headers and extract the original JSON bytes without serializing the
  complete model again. No Key Vault operation currently exercises this callback.
- [ ] Generate Resource Manager `PollerOptions` in long-running method options
  from `azure_core::http::poller::resource_manager`, along with `FinalStateVia`
  in clients, and preserve caller context and polling frequency. Leave ordinary
  method and pager options unchanged.
- [ ] Generate `StatusMonitor::Output = ()` with `StatusMonitor::Format = NoFormat`
  for empty final results. The prototype corrected managed HSM delete, managed
  HSM purge, and vault purge. Retain JSON formats for resource results.
- [ ] Delegate protocol selection, polling URLs, provisioning states, API-version
  handling, final GET decisions, and empty responses to the helper. Do not emit
  a competing implementation of those rules. The runtime already removes request
  bodies from polling GETs and retains the initial `Accept` header on every
  subsequent GET. It uses only `properties.provisioningState` for resource
  responses, defaults a missing provisioning state to success, and reads
  top-level `status` only from operation-monitor responses. Polling and final
  GET requests always use the initial request's API version, replacing any
  service-supplied version and adding it when absent.
- [ ] Clean up imports after removing inline polling logic, but retain `json`
  and `RawResponse` wherever pagination still uses them.

### Client constructors, authentication, and package generation

- [ ] Generate Resource Manager constructors that read
  `options.client_options.cloud` and default to Public cloud when omitted.
  Emit the prototype's borrowed-default pattern:

  ```rust
  use azure_core::cloud::resource_manager::CloudConfigurationExt as _;

  let options = options.unwrap_or_default();
  let cloud = options.client_options.cloud.as_deref().unwrap_or_default();
  let endpoint = cloud
      .endpoint()
      .ok_or_else(|| Error::new(ErrorKind::Other, "missing Resource Manager endpoint"))?;
  let endpoint = Url::parse(endpoint)?;
  let scope = cloud.default_scope()?.into_owned();
  ```

  Keep `options.client_options` available to move into `Pipeline::new`.
  Import `azure_core::cloud::resource_manager::CloudConfigurationExt as _` in
  generated and hand-written constructors. Resolve the management endpoint with
  `cloud.endpoint()` and the token scope with `cloud.default_scope()`. These
  trait methods take no audience type parameter; core uses the shared
  `azure_core::cloud::resource_manager::Audience` marker internally.
- [ ] For every crate whose name starts with `azure_resourcemanager_`, do not
  define or re-export a per-crate `Audience`. Continue generating per-crate
  audience markers for data-plane crates without that prefix.
  Use `azure_core::cloud::resource_manager::Audience` to register custom audiences
  in examples and tests; constructors import `CloudConfigurationExt` instead.
  A single custom audience entry must work across Resource Manager crates.
- [ ] Parse the returned endpoint with `Url::parse`, return an explicit
  configuration error when `endpoint()` returns `None`, and propagate errors
  from `default_scope()`. Pass the scope's `into_owned()` result to
  `BearerTokenAuthorizationPolicy`; do not generate endpoint tables, cloud
  matches, or manual `/.default` concatenation.
- [ ] Keep the identity authority, service endpoint, authentication audience, and
  token scope conceptually distinct. Never use the login authority as the
  Resource Manager endpoint, and never derive the endpoint from a token scope.
- [ ] Honor `CustomConfiguration::endpoint` independently of the shared ARM
  `Audience`. A custom cloud must supply both an endpoint and an audience;
  they need not be the same URL. Configure the credential's identity authority
  separately. Replace the prototype's audience-as-endpoint coupling.
- [ ] Delegate Public, Government, and China cloud mappings to core. Remove the
  prototype's Government, China, and future-cloud `todo!()` branches rather than
  generating a competing list of supported clouds.
- [ ] Generate accurate constructor documentation for the default cloud,
  custom-cloud endpoint and audience requirements, and configuration/URL errors.
  Fully qualify the shared audience link as
  ``[`Audience`](azure_core::cloud::resource_manager::Audience)``; importing only
  `CloudConfigurationExt as _` does not bring `Audience` into rustdoc scope.
  Remove the obsolete unsupported-cloud panic documentation.
  Use "Resource Manager" in public documentation and `resource_manager` in
  module, field, function, method, and feature names. Retain the
  `azure_resourcemanager_*` crate prefix. Prefer nested `resource_manager`
  modules over `ResourceManager` type or variant prefixes when disambiguating
  from data-plane APIs; retain short names such as `PollerOptions` and `Audience`
  within those modules.
- [ ] Generate two client-construction examples in each Resource Manager README:
  the normal default-cloud case first, then a custom-cloud case using the shared
  `azure_core::cloud::resource_manager::Audience`. Configure
  `CustomConfiguration::authority_host` and `endpoint`, register the audience,
  and pass the same cloud to a cloud-aware credential and the service client.
  `DeveloperToolsCredential` uses the developer tools' cloud settings and has
  no cloud option; use a credential such as `ClientSecretCredential` for the
  explicit custom-cloud example. Keep endpoint and authentication audience URLs
  distinct and load secrets from a secure source, not generated literals.
  Caller examples do not need `CloudConfigurationExt`: configuring
  `CloudConfiguration`, `CustomConfiguration`, and `Audiences` is unchanged.
  Keep the shared `Audience` import for custom audience registration.
- [ ] Generate a manifest dependency enabling `azure_core`'s `resource_manager`
  feature, preserve `default-features = false`, and forward default transport
  features consistently. Require a core version containing both the polling
  helper and `cloud::resource_manager::CloudConfigurationExt` and `Audience`,
  which are all gated by that feature.
  The prototype temporarily uses local `azure_core` path plus version
  `1.2.0-beta.2`; do not hard-code local paths into published service generation.

### Generation regression coverage

- [ ] Port the prototype's constructor and polling cases into emitter fixtures:
  omitted/Public/Government/China/custom clouds, distinct custom endpoint and
  audience URLs, missing custom endpoint, missing custom audience, invalid
  endpoint URLs, borrowed context, every long-running method, streamed monitors
  followed by awaiting, explicit final-state metadata, and failure propagation.
- [ ] Verify generated constructors delegate endpoint and scope resolution to
  core and preserve its errors. Assert that fixtures emit
  `cloud.as_deref().unwrap_or_default()` rather than cloning the configured
  cloud, allocating a default `Arc`, or hard-coding `AzurePublic`.
  Verify that ARM crates do not emit an `Audience` declaration and import
  `azure_core::cloud::resource_manager::CloudConfigurationExt as _` in constructors.
  Verify that generated imports, rustdoc links, and manifest features use
  `resource_manager`, while the crate name remains `azure_resourcemanager_keyvault`.
  Call `endpoint()`, `audience()`, and `default_scope()` without audience type
  parameters rather than emitting inline cloud mappings, custom audience
  lookups, or scope construction.
  Keep cloud mappings and scope normalization
  semantics tested in `azure_core`, not reimplemented in emitter fixtures.
  The core API in `dfc31db6e1` appends `/.default` to custom audiences verbatim;
  do not retain the prototype's trailing-slash normalization as an emitter rule.
- [ ] Verify both README client-construction examples compile against generated
  APIs, including the credential's cloud options. Register custom audiences only
  with the core Resource Manager marker in fixtures so that a mistakenly emitted
  per-crate marker fails the custom-cloud test. Verify data-plane generation
  retains its per-crate audience marker. Verify caller examples configure clouds
  without importing `CloudConfigurationExt`; only constructor implementations
  need the trait.
- [ ] Cover all polling protocols, repeated GETs without bodies preserving `Accept`,
  resource and empty final results, and Operation-Location inline extraction.
  Use operation-monitor responses when testing streamed top-level `status`;
  Location resource responses use `properties.provisioningState` or HTTP status.
  Verify failed operations expose service error details through `ErrorResponse`
  even when the HTTP response is 200.
  Keep protocol semantics tested in `azure_core`; emitter tests should verify
  metadata selection and generated integration with that runtime.
- [ ] Add a generation fixture for key action wire-value fidelity and a secret
  PATCH containing tags, explicit properties, and a value. Keep service
  recordings as evidence, not as a source of private payloads in emitter tests.
  Verify that PascalCase Rust identifiers preserve specified wire values;
  do not fix the service mismatch with a global enum-casing transformation.

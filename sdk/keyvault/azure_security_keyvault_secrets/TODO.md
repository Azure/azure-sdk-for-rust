# Emitter changes for fake transports

This crate prototypes generated-output changes for a separate emitter update.
Regeneration will overwrite these changes until the emitter supports them.

## Generate fake modules alongside clients

Generate `src/generated/clients/fake/mod.rs` and
`src/generated/clients/fake/secret_client.rs`, mirroring the regular client module
layout. The fake module re-exports only its public server trait; the handwritten
`clients` module re-exports the generated fake module. Gate the module and
`with_fake()` on both `test` and `unstable_fake`.

Keep the transport struct and its constructor private to the generated fake client
module. Callers supply their server through `with_fake()` and do not need a
transport adapter API.

The generated fake file contains the server trait, transport-owned operation
routing, inherent constructor, and private per-operation argument containers.
Route with the original typed arguments captured in request context, not values
decoded from URLs or reconstructed from headers. Do not generate URL-decoding
helpers or endpoint-prefix storage for fake dispatch.

Do not generate HTTP method, route-literal, allowed-query, or required-api-version
assertions. Routing is selected by API name; it need not validate client correctness.
Construct normal core errors for missing operation arguments, API/pager metadata,
or unknown API names. Include the operation name in argument errors; do not
silently substitute defaults or fall back to request parsing. Use `api_name` consistently.
Fake servers can create HTTP error responses with `ResponseExt::fake_with_error()`.

Keep captured arguments private and capture helpers crate-private. Do not emit per-item
`cfg_attr(docsrs, doc(cfg(...)))` attributes; docs.rs handles feature availability.

Parse the endpoint URL, but do not validate its scheme or impose additional
endpoint restrictions in `with_fake()`. Fake servers must be able to simulate
service and connection errors. Continue rejecting an explicitly supplied transport.

Demonstrate service fakes in `examples/secret_client_fake.rs`, requiring both
`test` and `unstable_fake`.
Generic fake-feature and operation-metadata coverage belongs in
`azure_core/src/http/fake/tests.rs`, declared with `#[cfg(test)] mod tests;`, with
a minimal local test client and server independent of any service crate.
Test fixtures are handwritten tests, not emitter output. Do not generate
test-specific servers inside the fake implementation.

## Capture original operation arguments

Capture original path, query, and declared header arguments in the existing client
methods before encoding them into HTTP requests. Generate one private container
per operation with non-body inputs, preserving their Rust types instead of using
a string map. Body-only operations need no empty argument container.

The Secrets prototype captures names for named operations, optional versions and
content types for get/update operations, and page-size options for all list
operations. The version-list operation also captures its name. Restore forwards
only its body and method options.

The emitter must:

- Gate argument capture on `all(feature = "test", feature = "unstable_fake")`.
  Ordinary builds must not reference the fake module or allocate fake metadata.
- Capture required parameters and optional method-option properties without
  copying an options object containing context back into that context.
- Preserve optionality, empty values, enums, typed scalars, arrays, duplicate
  values, and collection order. Do not percent-decode, stringify, parse, or split
  original values, including values containing slashes or invalid percent sequences.
- Use generated Rust property names, adding `path_`, `query_`, or `header_`
  prefixes only to disambiguate transport-location collisions. Keep container
  types distinct across operations and clients.
- Own borrowed values stored in the type-erased context. Keep argument payloads
  out of debug output; do not derive an unrestricted `Debug` implementation.
- Capture pageable arguments after options become owned, before `Pager::new`,
  including when the initial options already contain a continuation.
- Read captured arguments for both initial and continuation requests. Opaque
  continuation URLs might contain neither the original path nor its query.
- Leave URL construction, tracing, requests, continuation processing, and pipeline
  behavior unchanged. Do not introduce a wrapper client or shared continuation cache.

## Forward opaque request bodies

Use the existing `From<Body>` implementation to forward request content, for
example `let content: RequestContent<T, F> = request.body().clone().into();`. The
inherent `RequestContent::from(Vec<u8>)` shadows the trait's `from` method, so use
the typed conversion rather than calling that inherent method. Retain the
operation's model/format marker or raw-body shape without introducing serde,
`DeserializeOwned`, or model-cloning bounds.

Forward the actual body after policies run, preserving all encoded body model
properties. Do not duplicate bodies in context, decode models, validate JSON,
buffer streams, reset readers, or serialize content again in the transport. Bytes,
non-JSON formats, binary content, streams, and empty bodies must all work.
Server implementations decide whether and how to decode content.

## Serialize read-only response fields for fakes

Make read-only-field `serde(skip_serializing)` conditional on
`not(all(feature = "test", feature = "unstable_fake"))`. With both features enabled,
serialize populated read-only fields and use `skip_serializing_if = "Option::is_none"`
for optional fields. Preserve wire names, defaults, base64url and timestamp
codecs, nested models, and other visibility rules.

Use the `test` feature, not `cfg(test)`: SDK dependencies in customer tests,
integration tests, examples, and documentation tests are not compiled with `cfg(test)`.
`unstable_fake` already enables the same-crate `test` feature.

This opt-in behavior also affects shared models serialized in outgoing requests
in the same build. Document that distinction; ordinary builds retain existing
request serialization. `fake_with_model()` remains a JSON/serde convenience;
byte and stream responses remain format-independent alternatives.

## Preserve public client paths and package metadata

Extend the public `SecretClient` re-export with `impl SecretClient` and import
`crate::SecretClient` in the fake implementation. Do not introduce a second
client or expose a private generated-module path in the API snapshot.

Retain `exclude.workspace = true` in generated package manifests. The workspace
excludes `TODO.md`, keeping emitter follow-up instructions out of published crates
without renaming files or replacing inherited exclusions.

## Reuse operation instrumentation

Existing `tracing::function` attributes already insert operation metadata without
requiring a tracer provider or listener. Do not emit duplicate operation-name
insertion or an `operation_name!` macro. Fake transports obtain names through
`azure_core::http::fake::get_api_name()`, keeping the storage choice inside core.

Preserve the exact tracing operation identifiers, such as `KeyVault.getSecrets`
and `KeyVault.getSecretVersions`, rather than deriving names from Rust methods.
If separate operation-name storage is needed later, its names must match those
identifiers exactly. Handwritten operations should supply equivalent instrumentation.

## Validation requirements

Keep opaque-continuation, direct-resume, interleaved-pager, and
no-provider/no-listener routing tests in core, using its local test client and
server. Cover exact argument preservation, query arrays and headers (including
same-name values in different locations), missing-context errors, and opaque
byte/stream bodies without serde.

Run the typed-model service example to verify list/get wiring, two returned
secrets, and a service error. Verify default, test-only, no-default-feature, and
`unstable_fake`-alone configurations, including no-default feature combinations.

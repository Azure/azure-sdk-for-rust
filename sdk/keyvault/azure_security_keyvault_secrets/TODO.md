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
routing, inherent constructor, and per-operation pager context capture. Use
`azure_core::http::fake::Arguments::from_request(request, strip_prefix)` for decoded
arguments, `path_segments()` for positional extraction, and `query().get()` for
iterating all values of a named query parameter. Keep scalar `.next()` and
collection `.collect()` bindings in generated output. Parse scalar values at the
binding site; unexpected parsing or missing-argument failures may use descriptive
`expect()` calls in fake code, but not in production client operations.

Do not generate HTTP method, route-literal, allowed-query, or required-api-version
assertions. Routing is selected by API name; it need not validate client correctness.
Retain optional endpoint prefix stripping at segment boundaries. Keep query names
case-sensitive and preserve repeated values in encounter order for each name.
Use `request_content()` for JSON bodies and construct normal core errors directly
for missing API/pager metadata or unknown API names. Use `api_name` consistently.
Fake servers can create HTTP error responses with `ResponseExt::fake_with_error()`.

Keep `Arguments` fields private, exposing queries through `Arguments::query()`.
Private fields make `#[non_exhaustive]` unnecessary. Do not emit per-item
`cfg_attr(docsrs, doc(cfg(...)))` attributes; docs.rs handles feature availability.

Parse the endpoint URL, but do not validate its scheme or impose additional
endpoint restrictions in `with_fake()`. Fake servers must be able to simulate
service and connection errors. Continue rejecting an explicitly supplied transport.

Demonstrate service fakes in `examples/secret_client_fake.rs`, requiring both
`test` and `unstable_fake`, rather than generating service fake test suites.
Generic fake-feature and operation-metadata coverage belongs in
`azure_core/src/http/fake/tests.rs`, declared with `#[cfg(test)] mod tests;`, with
a minimal local test client and server independent of any service crate.
Test fixtures are handwritten tests, not emitter output. Do not generate
test-specific servers inside the fake implementation.

## Capture original pageable-operation arguments

In `src/generated/clients/secret_client.rs`, the following operations capture
arguments in their owned method-options context before constructing `Pager`:

| Generated method | Captured arguments | Crate-private fake helper |
| --- | --- | --- |
| `list_deleted_secret_properties` | `maxresults` | `with_list_deleted_secret_properties_context` |
| `list_secret_properties` | `maxresults` | `with_list_secret_properties_context` |
| `list_secret_properties_versions` | `secret_name`, `maxresults` | `with_list_secret_properties_versions_context` |

The emitter must:

- Gate argument capture on `all(feature = "test", feature = "unstable_fake")`.
  Ordinary builds must not reference the fake module or allocate fake metadata.
- Capture arguments after options become owned, before `Pager::new`, even when
  the initial pager options already contain a continuation.
- Keep context payloads private to the fake module and call crate-private helpers.
  Own borrowed arguments needed after the operation returns.
- Preserve the captured arguments in every page's context. Opaque continuation
  URLs might contain neither the original path nor its query parameters.
- Leave URL construction, tracing, requests, continuation processing, and pipeline
  behavior unchanged. Do not introduce a wrapper client or shared continuation cache.

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
server. Run the service example to verify generated list/get wiring, two returned
secrets, and a service error. Verify default, test-only, no-default-feature, and
`unstable_fake`-alone builds.

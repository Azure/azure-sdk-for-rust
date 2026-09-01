# ADR-0006 — Keep HTTP transport internal

**Status:** Accepted
**Date:** 2026-08-31

## Context

Cosmos availability behavior depends on coordinated endpoint selection,
connection health, retries, request-sent tracking, and Gateway V2 framing.
Allowing callers to replace or wrap the transport could bypass those
invariants or interfere with the driver's pools and health state.

## Alternatives considered

- **Accept a public HTTP client or transport implementation.** Rejected because
  injected behavior could alter pooling, retries, timeouts, proxying, or
  request-sent semantics below the Cosmos pipelines.
- **Build on a generic, customer-extensible `azure_core` pipeline.** Rejected
  because generic policies cannot preserve Cosmos-specific ordering and
  availability ownership across transport attempts.
- **Publish the Gateway V2 RNTBD-over-HTTP codec separately.** Rejected because
  callers could send protocol frames without the routing, negotiation, and
  retry rules that make them valid.

## Decision

The driver exclusively owns HTTP clients, connection pools, middleware, and
the Gateway V2 RNTBD-over-HTTP codec. These are implementation details, not
public injection or extension points. Build-time selection among supported HTTP,
TLS, and cryptographic backends remains allowed through Cargo features.

Plaintext HTTP is accepted only for recognized Cosmos emulator hosts.
Production account and backup endpoints require TLS.

Future updates MAY permit extending or even replacing the transport layer, but
using these extension points will forfeit the SDK's support SLA. Best-effort
support through GitHub will still be available.

## Consequences and exceptions

The driver can enforce availability and protocol semantics end to end, and
consuming SDKs share one behavior. Customers cannot install arbitrary transport
middleware or reuse the codec independently. Test-only internal factories and
fault-injection hooks are permitted when they cannot become supported public
transport contracts.

## Authoritative references

- [Operation and transport pipelines specification](../specs/0005-operation-and-transport-pipelines.md)
- [Gateway V2 specification](../specs/0011-gateway-v2.md)
- [Emulator transport security specification](../specs/0023-emulator-transport-security-and-authentication.md)
- [`http_client_factory` implementation](../../azure_data_cosmos_driver/src/driver/transport/http_client_factory.rs)
- [`adaptive_transport` implementation](../../azure_data_cosmos_driver/src/driver/transport/adaptive_transport.rs)
- [Internal RNTBD codec](../../azure_data_cosmos_driver/src/driver/transport/rntbd/mod.rs)
- [Emulator endpoint enforcement](../../azure_data_cosmos_driver/src/driver/transport/emulator.rs)
- Merged PRs `Azure/azure-sdk-for-rust#3744`,
  `Azure/azure-sdk-for-rust#3829`, `Azure/azure-sdk-for-rust#3912`,
  `Azure/azure-sdk-for-rust#3957`, `Azure/azure-sdk-for-rust#4223`,
  `Azure/azure-sdk-for-rust#4319`, and `Azure/azure-sdk-for-rust#4763`

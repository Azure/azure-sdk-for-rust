# ADR-0007 — Categorize Cargo features by contract

**Status:** Accepted
**Date:** 2026-08-31

## Context

Cargo features affect dependency graphs, compiled capabilities, and public API
shape. Without category-specific naming, users cannot infer stability or
support commitments from a feature name.

## Alternatives considered

- **Put every unstable capability under one `experimental` prefix.** Rejected
  because preview, unsupported public API, and internal implementation details
  carry different contracts.
- **Use runtime switches for every capability.** Rejected because runtime gates
  cannot remove dependencies or public types from the compiled API.
- **Choose names feature by feature.** Rejected because inconsistent names hide
  compatibility and support expectations.

## Decision

Use these feature categories:

- Dependency or backend selection uses the backend name: `reqwest`, `rustls`,
  `native_tls`, `hmac_rust`, and `hmac_openssl`.
- Capability enablement uses an unprefixed capability name: `key_auth`,
  `control_plane`, and `fault_injection`.
- Preview public API uses `preview_<name>`.
- Approved future public API without an SLA or Microsoft support commitment
  uses `no_sla_<name>`. There is no current instance.
- Internal-only surfaces use `__internal_<name>` and carry no SemVer contract.

Feature-gated public API must use the category matching its contract rather
than relying only on a runtime check.

## Consequences and exceptions

Feature names communicate dependency, stability, and support intent. Renaming
a published feature may require compatibility handling. The current `__tls`
feature does not conform: it is implementation debt to remove or replace, not a
sanctioned additional naming category.

## Authoritative references

- [`azure_data_cosmos` features](../../azure_data_cosmos/Cargo.toml)
- [`azure_data_cosmos_driver` features](../../azure_data_cosmos_driver/Cargo.toml)
- Preview-feature precedent in merged PR
  `Azure/azure-sdk-for-rust#4702`

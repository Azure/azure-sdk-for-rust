# ADR-005 — Drive startup topology and seed data from a JSON config file

**Status:** Accepted
**Date:** 2026-07-14

## Context

The hosted emulator needs a way to declare account topology (regions, write mode, consistency,
replication), the databases and containers to create, and optional seed documents — applied on
startup, before any client connects. The driver's config types are not `serde`-friendly (some
fields hold closures and shared mutable state).

## Decision

Accept a single JSON file via `--config`. The host owns `serde` DTOs that mirror the config and
translate them into driver types (`VirtualAccountConfig`, `VirtualRegion`, `ContainerConfig`).
Seed documents are created through the normal write path — one synthesized create-item request per
item through `execute_request` — so EPK routing, RU accounting, and replication match
client-issued writes. The management REST API can further modify state at runtime. YAML is
deferred.

## Consequences

Startup provisioning is declarative and reproducible; runtime mutation stays available through the
control-plane API. Keeping the DTOs in the host crate leaves the driver's config types untouched.
JSON is supported first because `serde_json` is already a workspace dependency.

## Alternatives

- Adding `serde` derives directly to the driver config types was rejected: several fields are not
  serializable, and it would leak host concerns into the driver.
- Shipping YAML in the first PR was rejected: `serde_yaml` is unmaintained; a maintained crate can
  be added later if needed.
- Seeding items by writing store internals directly was rejected: routing through `execute_request`
  guarantees identical semantics to real writes.

## References

- Plan & summary: ../plan.md

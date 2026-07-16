# ADR-006 — Drive startup topology and seed data from a JSON config file

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
client-issued writes. The management REST API can further modify state at runtime.

Listener ports are optional configuration hints. Missing values and `0` request OS-assigned ports;
the runtime endpoint contract is the JSON `ready` record and management account response described
by ADR-003. Public configuration uses `gateway20` terminology. Literal `thinClient*` names are
reserved for Cosmos account-topology fields and wire headers.

JSON is the canonical configuration representation. Additional syntaxes may be introduced only as
parsers that map to the same host-owned configuration model; they must not create a second set of
configuration semantics.

## Consequences

Startup provisioning is declarative and reproducible; runtime mutation stays available through the
control-plane API. Keeping the DTOs in the host crate leaves the driver's config types untouched.
The configuration contract remains independent from the driver's internal structs and from any
particular parser implementation.

## Alternatives

- Adding `serde` derives directly to the driver config types was rejected: several fields are not
  serializable, and it would leak host concerns into the driver.
- Making YAML a second canonical contract was rejected because two independently evolving
  representations would make validation and automation ambiguous.
- Seeding items by writing store internals directly was rejected: routing through `execute_request`
  guarantees identical semantics to real writes.

## References

- Plan & summary: `sdk/cosmos/azure_data_cosmos_emulator/docs/plan.md`

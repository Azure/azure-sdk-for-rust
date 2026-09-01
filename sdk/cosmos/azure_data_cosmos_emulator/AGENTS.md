# Hosted Cosmos DB Emulator Contributor Guide

This file contains package-local contributor and delivery guidance. Durable
architecture belongs in
[`sdk/cosmos/docs/specs/0027-hosted-emulator.md`](../docs/specs/0027-hosted-emulator.md).
Do not restate or evolve the hosted-emulator contract here.

Also read:

- [`sdk/cosmos/AGENTS.md`](../AGENTS.md) for Cosmos-wide engineering rules.
- [The in-memory emulator specification](../docs/specs/0021-in-memory-emulator.md)
  for store and operation semantics.
- [The transport security and authentication specification](../docs/specs/0023-emulator-transport-security-and-authentication.md)
  for TLS and credential policy.

## Package role

`azure_data_cosmos_emulator` is an unpublished binary host for the in-memory
emulator implemented by `azure_data_cosmos_driver`. Keep this crate focused on:

- CLI and process lifecycle;
- host-owned JSON configuration DTOs and validation;
- Gateway V1 and Gateway 2.0 listeners;
- HTTP-to-driver request bridging;
- the management REST router;
- startup provisioning and seeding orchestration;
- ready-record publication and process logging; and
- host-boundary TLS and authentication.

Do not move store, EPK routing, replication, split/merge, session, or RNTBD codec
logic into this crate. Add the smallest feature-gated driver adapter needed by
the host instead.

The host enables the driver's `__internal_in_memory_emulator` feature through
its dependency declaration. That driver surface is intentionally outside the
SemVer contract; do not expose it as a stable host library API.

## Source ownership

```text
src/
├── main.rs        # CLI, startup ordering, listener tasks, shutdown
├── config.rs      # serde DTOs, validation, driver translation, seeding
├── data_plane.rs  # Gateway V1 HTTP bridge
├── gateway_v2.rs  # Gateway 2.0 listener and high-level driver adapter
├── management.rs  # Emulator-only management REST API
└── metrics.rs     # Host process metrics
```

Keep protocol mappings and frame/token internals in the driver's RNTBD module.
`gateway_v2.rs` owns network hosting and the connectivity probe, not a second
codec.

## Behavioral guardrails

- Use `gateway20` in public configuration. Reserve literal `thinClient*` names
  for Cosmos account-topology fields and wire headers.
- Treat listener ports as hints. Missing values and `0` request OS assignment,
  and all published topology must use the actual bound URLs.
- Write exactly one JSON `ready` record to stdout after validation,
  provisioning, seed replication, and listener binding complete.
- Write diagnostic logs to stderr. Do not mix logs into the machine-readable
  stdout contract.
- Seed documents through the normal create-item request path. Never insert
  startup documents directly into store internals.
- Keep gateway-native database, container, offer, item, partition-key-range,
  and account operations on the data-plane listeners. The management API owns
  only emulator-specific controls.
- Keep management split and merge phase behavior deterministic. Manual
  progression must not depend on timing.
- Do not advertise Gateway 2.0 for a region unless its listener is configured
  and ready.
- Reject unsupported protocol behavior explicitly rather than approximating it.

Any change to configuration fields, ready-record shape, endpoint ownership,
management routes, topology behavior, or Gateway 2.0 hosting must update
[specification 0027](../docs/specs/0027-hosted-emulator.md) in the same change.
Security-policy changes must also update
[specification 0023](../docs/specs/0023-emulator-transport-security-and-authentication.md).

## Delivery sequence

The hosted emulator is delivered in three implementation stages:

1. **Network host:** the binary crate, Gateway V1 and configuration-gated
   Gateway 2.0 h2c listeners, management API, startup provisioning, and hosted
   CI coverage.
2. **Dynamic topology controls:** region offline/online and runtime write-region
   failover backed by driver store primitives.
3. **Transport security:** optional HTTPS plus key and Microsoft Entra ID
   authentication at the host boundary.

Keep stage-specific code isolated where practical. Do not weaken the no-auth
loopback workflow while adding authenticated scenarios.

## Test and CI workflow

For host changes, run the smallest applicable checks first:

```bash
cargo fmt -p azure_data_cosmos_emulator
cargo build -p azure_data_cosmos_emulator
cargo clippy -p azure_data_cosmos_emulator --all-features --all-targets
cargo test -p azure_data_cosmos_emulator --all-features
```

When a change touches the feature-gated driver adapter, also format, build,
lint, and test `azure_data_cosmos_driver` with the relevant features.

End-to-end hosted validation must:

1. start the host from a JSON configuration;
2. parse the ready record from stdout;
3. wait for `GET /health` on the resolved management endpoint;
4. construct client configuration from the reported account endpoint; and
5. run the existing emulator suites through real network clients in both
   Gateway V1 and Gateway 2.0 modes.

Preserve intentional legacy-emulator exclusions when adding hosted test
categories. Peer-SDK compatibility claims require that SDK's own transport and
wire validation; Rust success alone is not sufficient.

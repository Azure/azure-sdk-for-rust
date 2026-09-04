# ADR-0013 — Keep basic control-plane operations in the SDK

**Status:** Accepted
**Date:** 2026-08-31

## Context

Applications and tests commonly need basic Database and Container CRUD through
the Cosmos data-plane endpoint, but these operations are not the full Azure
resource-management surface. Their current service APIs require key
authentication, while authentication support and API availability are
separate SDK capabilities.

## Alternatives considered

- **Remove all control-plane operations and require ARM.** Rejected because it
  would remove convenient, established Database and Container CRUD used with
  the data-plane client.
- **Make `control_plane` imply `key_auth`.** Rejected because it couples API
  availability to one credential mechanism and would obstruct a future
  supported authentication path.

## Decision

Keep basic Database and Container CRUD in `azure_data_cosmos`, behind the
non-default `control_plane` capability feature. The feature is
authentication-agnostic and must not enable or otherwise imply `key_auth`.

The currently exposed operations still require key authentication as a service
constraint. Documentation must direct complete account, governance,
provisioning, and other management scenarios to Azure Resource Manager rather
than expanding this feature into a replacement management SDK.

## Consequences and exceptions

Default builds retain a focused data-plane surface. Users opt into these APIs
and separately select the authentication capability they need. Until the
service supports another path, enabling `control_plane` without usable key
authentication may compile but cannot make these requests succeed.

## Authoritative references

- [`azure_data_cosmos` feature definitions](../../azure_data_cosmos/Cargo.toml)
- [`CosmosClient` Database operations](../../azure_data_cosmos/src/clients/cosmos_client.rs)
- [`DatabaseClient` CRUD operations](../../azure_data_cosmos/src/clients/database_client.rs)
- [`ContainerClient` CRUD operations](../../azure_data_cosmos/src/clients/container_client.rs)
- PRs [#1853](https://github.com/Azure/azure-sdk-for-rust/pull/1853),
  [#4854](https://github.com/Azure/azure-sdk-for-rust/pull/4854), and
  [#3500](https://github.com/Azure/azure-sdk-for-rust/pull/3500)
- Issue [#4196](https://github.com/Azure/azure-sdk-for-rust/issues/4196)

# ADR-0009 — Model behavioral environment variables as options

**Status:** Accepted
**Date:** 2026-08-31

## Context

Environment variables are useful for deployment defaults and incident
response, but direct reads create invisible configuration paths with no typed
API, validation contract, or consistent precedence.

## Alternatives considered

- **Allow magic environment-only behavior.** Rejected because callers cannot
  discover, validate, test, or override it through the option system.
- **Ban environment variables entirely.** Rejected because deploy-time
  configuration and operational kill switches are required.
- **Treat ecosystem proxy variables as Cosmos options.** Rejected because
  `HTTP_PROXY` and `HTTPS_PROXY` are standard transport conventions owned by
  the selected HTTP backend.

## Decision

Every Cosmos-specific environment variable that changes behavior corresponds
to a real option in the configuration system. Generated `{ENV}_OVERRIDE`
variables are allowed only as the highest-priority representation of an
existing option; they do not define independent behavior.

Standard ecosystem variables such as `HTTP_PROXY` and `HTTPS_PROXY` remain
distinct. A Cosmos option may control whether the HTTP backend honors them.

In rare cases, a direct environment variable read may be required, such as
when an option is required but there is no way to access the configuration
system. However, this is always an exception.

## Consequences and exceptions

Environment behavior becomes typed, discoverable, and subject to the layered
configuration contract. Existing direct reads are migration debt, specifically:

- `COSMOS_DISABLE_IMDS`
- `AZURE_COSMOS_ENDPOINT_UNAVAILABLE_TTL_MS`
- `AZURE_COSMOS_QUERYPLANINTEROP_DIR`
- `AZURE_COSMOS_EMULATOR_HOST`
- `AZURE_COSMOS_BACKTRACE_CAPTURES_PER_SECOND` and
  `AZURE_COSMOS_BACKTRACE_RESOLUTIONS_PER_SECOND`

Until migrated, these variables are documented nonconformance and migration
debt, not sanctioned exceptions or precedents for new direct reads.

## Authoritative references

- [Configuration options specification](../specs/0001-configuration-options.md)

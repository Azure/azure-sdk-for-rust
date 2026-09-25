<!--
Copyright (c) Microsoft Corporation. All rights reserved.
Licensed under the MIT License.
-->

# Throughput control

**Status:** Partially implemented — server-side signalling only.
**Crates:** `azure_data_cosmos_driver` (canonical), `azure_data_cosmos` (re-export)
**Supersedes the deferral in:** 0001 — Configuration options (`0001-configuration-options.md`),
which deferred `priority` and `throughput_bucket` to a follow-up ([#3803]).

## Scope

Throughput control tells the service how to treat a request when the
container's provisioned throughput is contended. The driver sends
`x-ms-cosmos-priority-level` and `x-ms-cosmos-throughput-bucket` when their
values are configured. It does not implement client-side RU accounting,
rate limiting, token buckets, or queueing; enforcement is server-side.

Applications set these values directly through
`OperationOptions::throughput_control`, which is available through
`azure_data_cosmos::options` as well as `azure_data_cosmos_driver::options`.
There are no throughput-control groups, name lookups, or registrations.

## Options and layering

`ThroughputControlOptions` is a nested option group on `OperationOptions`,
following the same pattern as `ThrottlingRetryOptions`:

| Field | Type | Wire header |
| --- | --- | --- |
| `throughput_bucket` | `Option<u32>` | `x-ms-cosmos-throughput-bucket` |
| `priority_level` | `Option<PriorityLevel>` | `x-ms-cosmos-priority-level` |

Each field resolves independently through operation → account → runtime
defaults. An unset field falls through without clearing its sibling: for
example, a runtime default `priority_level: Low` and an operation-level
`throughput_bucket: 99` produce both headers. Neither field reads environment
variables. The default for each is `None`, so the header is omitted when unset
at every layer.

`PriorityLevel` has `High` and `Low` variants. The header uses those exact
strings; when priority-based execution is enabled on the account, the service
throttles `Low` requests before `High` requests under contention. The
`PriorityLevel::default()` value is `High`, but it is not applied implicitly
to an unset option.

`throughput_bucket` is an opaque `u32` that identifies a server-side bucket
configured on the account. The driver sends its decimal value without
validating the account's bucket range. An out-of-range bucket can result in a
429/3212 response from the service.

## Request behavior

The driver resolves these values once per operation and reuses them for
retries, failover, and hedged attempts. Only container-targeted operations
use them; options inherited by account or database operations are ignored.
After recovery from a container recreation, the replacement operation uses
the same layered values. The standard gateway emits a header only when its
corresponding value is set.

The Gateway 2.0 wrapper currently synthesizes a fresh header set without
priority or bucket tokens, so requests sent through it lose both headers.
This gap is independent of the removed group-registration mechanism.

The driver applies its ordinary 429 throttling retry policy to
`THROUGHPUT_BUCKET_LIMIT_EXHAUSTED` (3212) and
`TOO_MANY_THROUGHPUT_BUCKET_UPDATES` (3213). It does not adapt retries based
on priority, track resolved values in diagnostics, or detect whether the
account has the corresponding service features enabled.

[#3803]: https://github.com/Azure/azure-sdk-for-rust/pull/3803

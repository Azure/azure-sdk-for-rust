# ADR 001: Select PATCH Execution Path

## Status

Accepted

## Context

Cosmos DB supports atomic server-side PATCH, but a request may contain no more
than 10 instructions. The driver also supports client-side PATCH by reading the
item, applying instructions locally, and replacing it under an ETag.

The paths have different availability and concurrency properties. Server-side
PATCH needs one request and preserves path-level conflict resolution. A
non-convergent server request cannot be resent after an ambiguous outcome
without risking duplicate application. Client-side PATCH needs at least two
requests and contends at document granularity, but its persisted tracking marker
provides bounded duplicate suppression and it is not subject to the service's
10-instruction limit.

## Decision

Expose `PatchStrategy` with three values:

- `Auto` is the default. It selects server-side PATCH only when the complete
  instruction list is retry-safe and contains at most 10 instructions. It uses
  client-side read-modify-write otherwise.
- `ClientSide` always uses the read-modify-write path and accepts more than 10
  instructions. It persists a tracking marker for non-retry-safe lists or when
  the caller supplies a tracking ID.
- `ServerSide` always sends one PATCH request. More than 10 instructions surface
  the service's HTTP 400 response rather than falling back. Unsafe lists disable
  retries after ambiguous outcomes.

The strategy participates in normal environment, runtime, account, and
operation option resolution. The Rust SDK also exposes a per-request strategy
on `PatchItemOptions` while PATCH remains a preview feature.

Both paths report `patch_item` as the logical OpenTelemetry operation. The
client-side network helpers report `patch_read_item` and
`patch_replace_item`; local tracking-marker work does not create a synthetic
network span.

## Consequences

`Auto` gives retry-safe, service-sized patches the lower latency and finer
conflict resolution of server execution. Unsafe requests retain marker-backed
client-side duplicate suppression; retry-safe over-limit requests use
client-side RMW without a marker. Explicit `ServerSide` lets callers avoid the
tracking property, but they accept at-most-once behavior for unsafe lists and a
hard failure above 10 instructions.

The driver and in-memory emulator must keep the service limit, wire content
type, Gateway 2.0 opcode, path-selection rules, and telemetry names covered by
regression tests.

## Alternatives

Always use server-side PATCH was rejected because it cannot support more than
10 instructions and cannot safely retry unsafe lists after ambiguous outcomes.

Always use client-side PATCH was rejected because it adds latency and loses
server path-level conflict resolution for requests the service can execute
safely.

Silently falling back from explicit `ServerSide` was rejected because an
explicit strategy must not change execution semantics without the caller's
knowledge.

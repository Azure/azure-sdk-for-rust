# ADR 0006 — Each language binding owns marshalling and buffer copy-out

**Status:** Accepted (proposed for review)

## Context
The driver core is schema-agnostic: the C-ABI passes request/response bodies as raw bytes (`const uint8_t*` + length), never parsed JSON (see `NATIVE_WRAPPER_SPEC.md`, introduced in #4461). Pushing serialization into the wrapper would re-introduce the parse/re-serialize/re-parse waste the old `azure_data_cosmos_native` crate had.

## Decision
- The ABI stays **bytes-in / bytes-out**; the wrapper does no JSON parsing.
- Each language binding **owns its own marshalling** (string encoding, structs) and **copies response buffers out of native memory** into host memory, then frees the native buffer.
- Rust owns the buffer until the host copies it out; ownership transfer is explicit per the ABI spec.

## Consequences
- The native binaries are identical for every language; differences live in each binding.
- No double-encoding cost across the FFI boundary.
- Each language must implement copy-out correctly (lifetime + free) — a per-binding responsibility.

## Alternatives considered
- Serialize to JSON strings in the wrapper — rejected: redundant parsing, hides driver concepts.
- Zero-copy borrow of native buffers into host memory — rejected for now: lifetime hazards across GC'd hosts; copy-out is the safe default.

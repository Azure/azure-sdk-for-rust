# Binary Encoding Round-Trip Findings

A running log of findings related to Cosmos binary JSON encoding round-trips.
Add new findings as additional numbered sections below.

---

## Finding 1: Integers beyond `2^53` do not round-trip through a live account

### Summary

Large unsigned integers beyond `2^53` do **not** round-trip losslessly through a
live Azure Cosmos DB account, regardless of whether the item body is encoded as
**Cosmos binary JSON** or as **text JSON**. This is a property of the service's
number model, not a defect in the SDK's binary codec.

### Background

The live Cosmos DB service normalizes every JSON number to an IEEE-754
double-precision float (`f64`) on ingestion. Doubles can represent integers
exactly only up to `2^53` (`9_007_199_254_740_992`). Any integer larger than
that is snapped to the nearest representable double when the service stores it.

Concretely, `u64::MAX` (`18446744073709551615`, i.e. `2^64 − 1`) is not exactly
representable as an `f64`. The nearest double is `2^64`
(`18446744073709551616`), which the service stores and echoes back. That value
no longer fits in a `u64` field (`u64` tops out at `2^64 − 1`).

### Observed failure

The live integration test
`binary_encoding_tests::cosmos_binary_encoding::binary_encoding_item_crud_round_trips`
failed in the pipeline (macOS `SessionSingleWriteBinaryEncodingRoundtripFuzz`
leg) with:

```text
CosmosError: 500/20020 (SerializationResponseBodyInvalid): failed to deserialize response body
Caused by:
  0: Custom("invalid type: floating point `18446744073709552000.0`, expected u64")
```

The test document set `huge: u64::MAX`. The service echoed the value back as the
double `18446744073709552000.0`, and deserialization into the `u64` field failed.

### Why this is not a binary-encoding regression

The failure happens **after** the service normalizes the number, so both wire
formats are affected identically:

- **Binary path:** the echoed number carries the `NUMBER_DOUBLE` marker. The
  decoder correctly maps it via `visit_f64`, and serde rejects the float for a
  `u64` field: *"invalid type: floating point … expected u64"*.
- **Text path:** the service echoes the value as a JSON number. `serde_json`
  attempts to fit `2^64` into the `u64` field and fails with an
  out-of-range/overflow error (`2^64` exceeds `u64::MAX`).

Either way the round-trip fails. The binary codec behaves correctly and
consistently with the text path — it faithfully decodes exactly what the service
returned.

### Why the in-memory emulator test passed

The analogous in-memory-emulator round-trip test uses `u64::MAX` and passes,
because the emulator preserves the exact `UInt64` wire form rather than
normalizing numbers to doubles. It therefore does not exercise the live
service's double-normalization behavior.

### Resolution

The defect was in the **test data**, not the codec. A value at or below `2^53`
round-trips losslessly. The live test now uses `2^53`
(`9_007_199_254_740_992`) — the largest integer exactly representable as an
`f64`, while still being a `u64` — to exercise the wide-unsigned encoder form
without hitting the service's precision ceiling.

### Guidance

- Do not assert exact round-trip equality for integers greater than `2^53` when
  testing against a live Cosmos DB account.
- If an application must preserve integers beyond `2^53`, store them as strings
  (or a struct with explicit high/low words); numeric fields will be subject to
  double precision on the service.
- This constraint is independent of the binary-encoding feature and applies to
  text JSON as well.

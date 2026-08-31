# [Cosmos] Wide-integer `Double` coercion: align text deserialization with binary, and close the `Value`/sequence/enum gaps

**Labels:** `Cosmos`, `binary-encoding`
**Follow-up from:** PR #4976 (review comments by @tvaron3 and the Copilot reviewer)
**Area:** `sdk/cosmos/azure_data_cosmos_driver` (`binary_json`), `sdk/cosmos/azure_data_cosmos` (response decode)

---

## Summary

PR #4976 taught the **binary** JSON deserializer to coerce a service-echoed integral `Double` back into an integer field (the service's `Number64` model has no unsigned/`>= 2^53` integer slot, so a wide integer is persisted and echoed as an IEEE-754 double). That fix is intentionally scoped to the native binary scalar path and leaves three related paths **inconsistent**:

1. **Text deserialization** (`serde_json::from_slice`) still **hard-fails** on the same value.
2. The **binary `Value`/exotic-form fallback** (`deserialize_via_value`) does not coerce, so a typed integer *sequence* from a service-only uniform `Float64` array (e.g. `Vec<u64>`) errors.
3. **Enum-variant** integer fields decoded via `serde_json::Value` do not coerce and error.

All three share **one root cause** and can be closed by **one fix**. This issue tracks that follow-up.

---

## Background

Cosmos stores **every JSON number as an IEEE-754 double**. Any integer at or beyond `2^53` (including every `u64 > i64::MAX`) therefore cannot be stored exactly and is echoed back as a `Double`. Reading that double into a typed integer field (`u64`/`i64`/…) must decide between erroring and coercing.

- Before #4976, **all** Rust read paths errored (`invalid type: floating point, expected u64` → `500 / SerializationResponseBodyInvalid`).
- #4976 made the **binary scalar** path coerce (signedness-routed, saturating cast, fractional doubles still error). See `sdk/cosmos/azure_data_cosmos_driver/docs/BINARY_ENCODING_U64_MAX_ANALYSIS.md`.

## The problem: text vs binary now diverge

After #4976, for the *same stored value* (a wide integer the service turned into a double):

| Read path | Dispatch | Behavior on a wide-integer field |
| --- | --- | --- |
| **Binary** | `is_binary(bytes)` (first byte `0x80`) → `binary_json::from_slice` → `deserialize_integer` | ✅ coerces (lossy) |
| **Text** | else → `serde_json::from_slice::<T>` (unchanged) | ❌ hard error |

A customer switching a container/operation between text and binary encoding would see a wide-integer field that **reads back under binary but errors under text**. (Note: text was **not** regressed by #4976 — it failed on wide `u64` before too; #4976 improved binary and thereby created the asymmetry.)

### .NET reference behavior (why this matters)

.NET does **not** have this split. Its text and binary readers (`JsonTextReader` / `JsonBinaryReader`) both produce a shared `Number64` (`"a double or a 64-bit long"`) **before** typing, so a single coercion policy (`Number64JsonConverter` + Newtonsoft) applies uniformly — both encodings coerce the echoed double into the integer field (silently, lossily) and neither hard-fails. Rust diverges only because it has **two independent serde deserializers** (`binary_json::from_slice` custom vs. `serde_json::from_slice` standard) that make independent typing decisions; #4976 only touched one of them.

> Provenance caveat: the .NET architecture claim (shared `Number64` → no text/binary split) is solid; the exact "never throws on read" guarantee is inferred from the Newtonsoft/`Number64JsonConverter` stack, not line-proven. Files to confirm: `JsonTextReader.cs`, `JsonBinaryReader.cs`, `Number64JsonConverter.cs` in `Azure/azure-cosmos-dotnet-v3`.

## Related gaps with the same root cause

Both are already documented as accepted limitations in #4976 (with `TODO(#4976)` markers and CHANGELOG notes), pinned by tests:

- **Uniform `Float64` array → typed integer sequence.** The binary `deserialize_via_value` fallback forwards a decoded `serde_json::Value` to `Value::deserialize_any`, which maps `Number(f64)` → `visit_f64`. A `Vec<u64>` of wide integers therefore errors. Pinned by an accepted-limitation note in `de.rs`.
- **Enum-variant integer fields.** `deserialize_enum` materializes the variant via `read_value()` → `serde_json::Value`, same non-coercing path. Pinned by `enum_variant_integer_from_double_is_not_yet_coerced` (asserts the current error; flip when fixed).

The common thread: **serde does not signal the target integer type to a `Value`/`serde_json`-driven deserialization**, so a per-visitor hint (as used for binary scalars) cannot reach these paths.

## Proposed fix

A **target-aware coercing `Value` pass** applied uniformly:

1. Add a reusable `coerce_integral_doubles(&mut Value)` (recursive) that rewrites every integral-valued finite `Double` to an integer `Number` (non-negative → `u64` saturating at `u64::MAX`; negative → `i64`; matching the scalar path's saturating semantics). Non-integral / non-finite doubles are left untouched.
2. **Text path** (`azure_data_cosmos` response decode): parse to `Value` → run the coercion pass → `T::deserialize(value)`, instead of a direct `serde_json::from_slice::<T>`.
3. **Binary `Value` fallback** (`deserialize_via_value`) and **enum path** (`deserialize_enum`): run the same pass on the decoded `Value` before delegating.

Because the transform happens on the **`Value` tree before typed deserialization**, it works uniformly for scalars, sequences (`Vec<u64>`), and enum variants — the cases individual visitor hints cannot reach. Result: **one coercion policy across text and binary**, matching .NET's `Number64` consistency.

### Trade-off requiring sign-off

Pre-coercing integral doubles (`2.0 → 2`) changes what a `T = serde_json::Value` (untyped) target observes on these paths. This is arguably moot because **Cosmos already destroys the int-vs-float distinction** (it stores `2.0` as `2`), so an untyped `Value` target cannot rely on getting `2.0` back regardless of encoding. Still, it is a behavior change and should be gated/reviewed:

- Option A — always-on (simplest; matches the binary scalar decision in #4976).
- Option B — gate behind an option/flag if untyped-`Value` fidelity must be preserved for some callers.

The `fuzz_tests.rs` `assert_decoders_agree` parity contract (binary `decode` vs `from_slice` at the `Value` level) must be re-evaluated under whichever option is chosen.

## Acceptance criteria

- [ ] A wide-integer field (e.g. `u64` ≥ `2^53` / `> i64::MAX`) round-trips (lossily, saturating) identically under **text** and **binary** encoding — no hard error on either.
- [ ] `Vec<u64>` decoded from a service-produced uniform `Float64` array coerces instead of erroring (binary).
- [ ] An integer field inside an enum variant coerces instead of erroring; flip `enum_variant_integer_from_double_is_not_yet_coerced` from `is_err()` to `is_ok()` + value check.
- [ ] Fractional doubles into an integer field still error on all paths (no silent truncation).
- [ ] The chosen `Value`-semantics option (A/B) is documented; `assert_decoders_agree` updated to match.
- [ ] CHANGELOG updated; the `TODO(#4976)` markers in `de.rs` replaced with this issue number.
- [ ] Analysis doc (`BINARY_ENCODING_U64_MAX_ANALYSIS.md`) updated to record text/binary alignment and the .NET `Number64` rationale.

## Out of scope

- The writer stays exact (`encode_u64` still emits `NumberUInt64`) — forward-compatible if the backend ever preserves `UInt64`. This issue is read-side only.
- No change to the fundamental lossiness: values ≥ `2^53` that are not exactly `f64`-representable remain lossy on round-trip (a `Number64`-model reality, identical to .NET). This issue is about *consistency and not-erroring*, not about preserving precision the service cannot store.

## References

- PR #4976 — Cosmos: Implement Binary Round Trip Fuzzer (introduced the binary coercion; approved by @analogrelay, @tvaron3).
- Review comment: PR #4976 `#discussion_r3738112395` (@tvaron3 — text/binary divergence).
- `sdk/cosmos/azure_data_cosmos_driver/docs/BINARY_ENCODING_U64_MAX_ANALYSIS.md`
- `sdk/cosmos/azure_data_cosmos_driver/src/binary_json/de.rs` (`deserialize_integer`, `deserialize_via_value`, `deserialize_enum`)

# Responses to Copilot review — PR #4976

Copy/paste each block as a reply to the corresponding Copilot comment. Fixed in commit `33f098a089`.

---

**1. Create-conflict masks a create-response decode failure (Medium)**

Fixed. On a 409 we now delete the deterministic stale item and retry `create` (bounded by `MAX_OP_ATTEMPTS`, treating a concurrent 404 as success), instead of reading the item back. A seed replay therefore re-exercises the create-response decode path rather than validating an unrelated read response.

---

**2. `WIDE_NUMBER_TAG` sentinel is not type-injective (Medium)**

Acknowledged and documented as a known blind spot at the constant. A genuine user object carrying this exact key and a matching value would canonicalize like the wide-number token; the sentinel is deliberately obscure to make that astronomically unlikely in generated documents. A fully injective fix (tagging normalized kinds out-of-band) is noted for follow-up.

---

**3. `arbitrary` `derive` feature is unused (Low)**

Fixed. The harness implements `Arbitrary` by hand and there is no `#[derive(Arbitrary)]` in the tree, so the workspace dependency is now `arbitrary = "1.4"` without the `derive` feature.

---

**4. CHANGELOG entry longer than the one-line policy (Low)**

Fixed. Condensed to a single-line summary that keeps the behavior and the lossy-coercion warning, and additionally notes the uniform-`Float64`-array / enum-variant gaps; implementation detail stays in the linked PR and design doc.

---

**5. Coercion bypassed for integer fields in enum variants (Medium)**

Confirmed — same root cause as the uniform-`Float64`-array limitation: `deserialize_enum` materializes the value via `read_value()` and delegates to `serde_json::Value`, which cannot see the target integer type. A correct fix is a target-aware coercing `Value` deserializer (serde does not signal the element type on the `Value` fallback path), which is larger than a local change. Tracked as a `TODO(#4976)` on `deserialize_enum` and disclosed in the CHANGELOG for now.

---

**6. Unchecked `as u32` narrowing on `max_depth` / `size_scale` (Medium)**

Fixed. Introduced a checked `env_u32(name, default, min, max)` that panics on values above the bound instead of silently wrapping (e.g. a multiple of `2^32` truncating to `0`). Added `MAX_DEPTH_LIMIT = 64`, `BREADTH_LIMIT = 1024`, and `SIZE_SCALE_LIMIT = 1024`, so an out-of-range CI value fails fast rather than stack-overflowing or OOMing. The regression test was updated accordingly.

---

**7. Design-doc pseudocode contradicts the implemented projection (Low)**

Fixed. The doc now describes `strip_system(R)` (a deny-list that removes only the service system fields and preserves any codec-invented extra field so it fails the assertion), matching the implementation instead of the weaker `project(R, keys(D))` allow-list.

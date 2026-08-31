# PR #4976 — Round 2 Review Remediation Plan

**Source:** deep review round 2 (`pr-4976-review-findings-v2.md`), verified against the working tree
on 2026-08-05. All findings below were independently confirmed in code except where noted.

**Verification note:** the review ran at head `62bcb1a0ed`; our local commits differ
(`531247cf83`, `51e121b595`, `c448ce53c2`, `8ef1aee1f0`). One consequence: **S8's `crates.txt`
half does not apply** (no `arbitrary` entries exist in `eng/dict/crates.txt` in our tree); the
`fsanitize` orphan in `.cspell.json` does apply.

---

## Decisions needed before starting

- **DR1 — B1 (blocking).** Fix the uniform-`Float64`-array coercion gap now, or accept + track?
  - *Recommendation:* **fix it.** It is small and local, and — given we already chose always-on
    coercion (D1=a) — accepting it creates an arbitrary `scalar coerces / Vec<u64> errors`
    asymmetry on the exact data shape the fix targets. Thread the `signed` hint into
    `deserialize_via_value` and apply the same `fract()==0` + range + saturating-cast coercion when
    walking a `Value::Number` into an integer visitor. Falls out naturally from S3 (shared helper).
  - *If deferred:* file a tracked issue, reference it in the `de.rs` comment (not the bare "Accepted
    limitation"), and surface it in the **CHANGELOG** so a user can discover it.
- **DR2 — R10.** Does the ".NET interop parity" trade-off have a real source? If yes, link it; if
  no, delete the sentence. Either way get an explicit ✅ from @analogrelay or @tvaron3 (it reverses a
  stated cross-SDK position).

---

## Phase 1 — Doc-only corrections (low risk, high clarity; do first)

- [ ] **R3** — `U64_MAX:96-97`: rewrite "oracle now flags … instead of masking" to describe the
      tagged-token behavior (lossy values compare **equal**). Fix the second instance too.
- [ ] **R4** — `de.rs:113` + `U64_MAX:183`: delete the false "genuine type error rather than a
      saturating coercion" sentence; add a comment that the `2^63`/`2^64` endpoints are
      **deliberately** inside the inclusive ranges (load-bearing saturation). Optionally add the
      signed analogue of Example B (`i64::MAX - 1`).
- [ ] **R10** — `U64_MAX:~161`: resolve DR2 (link or delete).
- [ ] **S10** — `ROUNDTRIP_FUZZER.md:581`: add a `## 9.` parent heading; demote `## 9.8` to `###`.

## Phase 2 — Close the coverage gaps that make the PR's claim true

- [ ] **R1 + R5 + B1(coverage)** — add a typed probe round-tripped each iteration:
      `struct WideProbe { id, pk, wide: u64, signed: i64, wides: Vec<u64> }`. This is the single
      highest-value change: it makes `deserialize_integer` actually reachable live, gives the oracle
      a value that cannot silently int↔float-collapse, and (via `wides`) exercises B1.
- [ ] **R7** — assert the negotiated encoding per config (first response byte `0x80` for binary, or
      `Content-Type`); fail loudly on silent degradation to text; print it in the banner.
- [ ] **R2** — set `AZURE_COSMOS_FUZZ_SHAPE_RATIO` (e.g. `50`) on the `binary_encoding` CI leg, or
      have 2–3 shape samplers draw one field from `gen_number`.

## Phase 3 — B1 production fix (if DR1 = fix)

- [ ] **B1 + S3** — extract a shared
      `deserialize_scalar_or_container<V>(visitor, integer: Option<Signedness>)` from the duplicated
      body of `deserialize_any`/`deserialize_integer`; thread `Option<Signedness>` through
      `deserialize_via_value` and the seq/map visitors so a `Value::Number` integral double coerces
      into an integer element. Add a `Vec<u64>`-from-uniform-`Float64`-array test. Update CHANGELOG.

## Phase 4 — Harness robustness

- [ ] **R6** — fold a short hash of the effective `FuzzConfig` into the item id
      (`fuzz-{seed:016x}-{cfghash:08x}-{iter}-{config}`); print the full effective config on the
      reproduce line.
- [ ] **R8** — set a container `DefaultTimeToLive` (a few hours) or delete items after their
      assertions; fix `Region::EAST_US` to match the provisioned region (East US 2) or derive it.
- [ ] **R9** — add `"ContinueOnError": true` to the `binary_encoding` matrix leg; lower
      `MAX_OP_ATTEMPTS` to 3 or add a wall-clock budget that aborts cleanly with a summary.
- [ ] **S4** — route the 409-recovery read through `with_transient_retry` (add a recovery
      predicate); drop the duplicated inline backoff.

## Phase 5 — Cleanup / polish

- [ ] **S1** — use `.clamp(1, u32::MAX as u64) as u32` for `size_scale` and `max_depth` (match
      `breadth`); extend the existing regression test.
- [ ] **S2** — clamp `max_depth ≤ 64`, `breadth ≤ 1024` (or panic with an explicit bound) so a
      typo'd CI value errors instead of stack-overflow/OOM.
- [ ] **S5** — drop the redundant SHA-256 comparison (or wire it to the durable-corpus use case);
      replace per-byte `format!` with `write!`.
- [ ] **S6** — delete the dead `gen_unicode_string_for` pass-through.
- [ ] **S7** — rename `project_to_sent_keys` → `strip_service_assigned_fields` (keep the `sent`
      safety-valve param); rename its test.
- [ ] **S8** — remove the orphaned `fsanitize` (and other unmatched) entries from `.cspell.json`;
      sort. *(crates.txt half is N/A in our tree.)*
- [ ] **S9** — drop `arbitrary`'s unused `derive` feature (confirm no other crate in the workspace
      relies on `#[derive(Arbitrary)]` first).

## Observations (process, not code)

- **O1** — close #4898 as superseded; annotate #4977 that `huge: u64::MAX` must **not** be lowered
  (it is the only live coverage of the `de.rs` fix until R1 lands).
- **O2** — reconcile the PR description with draft status once the above are triaged.

---

## Suggested sequencing

Phase 1 (docs, trivial) → Phase 2 (coverage — makes green mean something) → Phase 3 (B1 fix, if
DR1) → Phase 4 (robustness) → Phase 5 (cleanup). Post the 13-comment subset the review recommends
(B1, R1–R9, S1, S3, S8) if replying on GitHub.

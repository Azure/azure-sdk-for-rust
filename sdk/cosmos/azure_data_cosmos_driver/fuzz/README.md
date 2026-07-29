# Binary JSON codec fuzzing (`cargo-fuzz`)

Coverage-guided, **byte-level** fuzzing for the Cosmos binary JSON codec
(`azure_data_cosmos_driver::binary_json`). Where the live
[round-trip fuzzer](../../azure_data_cosmos_perf/tests/binary_roundtrip_fuzzer.rs)
generates random JSON *values* and only ever feeds the decoder **encoder-produced**
(well-formed) bytes, these targets feed **arbitrary and mutated bytes** straight
into the decoder — so they exercise the *format*/protocol itself: truncated
buffers, bad length prefixes, unknown or misused markers, reference/depth bombs,
non-UTF-8 string payloads, and trailing bytes.

This is a **separate crate** with its own empty `[workspace]` in `Cargo.toml`, so
it stays isolated from the stable repo workspace: cargo-fuzz builds it on nightly
with libFuzzer.

## Prerequisites

```bash
rustup toolchain install nightly
cargo install cargo-fuzz
```

## Targets

| Target | Entry point | What it checks |
| --- | --- | --- |
| `decode` | `binary_json::decode` | `Value` decode never panics/hangs/over-allocates on any bytes. |
| `from_slice` | `binary_json::from_slice::<Value>` | Native serde streaming decode honors the same no-crash contract. |
| `transcode_to_text` | `binary_json::transcode_to_text` | Driver-side binary→text response transcode never panics on a malformed body. |
| `decode_reencode_roundtrip` | `decode` + `encode` | **Differential**: any buffer the decoder accepts must satisfy `decode(encode(decode(x))) == decode(x)` — catches reader/writer disagreements. |

All four assert the **robustness oracle**: for *any* input the codec terminates
and returns `Ok`/`Err` — never panics, hangs, or allocates beyond the buffer.
The last one adds a **semantic** oracle on decoder-accepted inputs.

## Running

From this `fuzz/` directory (or the driver crate root):

```bash
# Explore one target (Ctrl-C to stop):
cargo +nightly fuzz run decode

# Time-boxed CI-style smoke run (60s), 4 workers:
cargo +nightly fuzz run decode -- -max_total_time=60 -workers=4

# Reproduce a crash from a saved artifact:
cargo +nightly fuzz run decode fuzz/artifacts/decode/crash-<hash>

# Minimize a crashing input:
cargo +nightly fuzz tmin decode fuzz/artifacts/decode/crash-<hash>
```

## Thorough manual run on a Linux VM

For deeper, longer fuzzing than the weekly CI budget (Option A caps each target
at ~1000 s), run it by hand on any Linux box (or WSL2). This is the same thing CI
does, without a wall-clock cap:

```bash
# 1. Toolchain (one-time)
rustup toolchain install nightly --component rust-src
cargo install cargo-fuzz --locked

# 2. Get the code and seed the corpus from the golden vectors (recommended —
#    lets libFuzzer mutate outward from real wire frames).
cd sdk/cosmos/azure_data_cosmos_driver
mkdir -p fuzz/corpus/decode
jq -r '.[] | "\(.name) \(.binary)"' testdata/binary_json_vectors.json |
while read -r name hex; do
  echo "$hex" | tr -d ' ' | xxd -r -p > "fuzz/corpus/decode/$name"
done

# 3a. Run one target for a fixed budget (e.g. 1 hour), 8 parallel workers:
cargo +nightly fuzz run decode -- -max_total_time=3600 -workers=8 -jobs=8 -print_final_stats=1

# 3b. Or run it open-ended until you Ctrl-C (a true soak):
cargo +nightly fuzz run decode -- -workers=8 -jobs=8

# 4. Repeat for the other targets (they share the same corpus format):
cargo +nightly fuzz run from_slice           -- -max_total_time=3600 -workers=8
cargo +nightly fuzz run transcode_to_text    -- -max_total_time=3600 -workers=8
cargo +nightly fuzz run decode_reencode_roundtrip -- -max_total_time=3600 -workers=8

# 5. Or drive all four with the CI helper (installs deps, seeds corpus, runs each):
pwsh ../eng/scripts/Run-BinaryJsonFuzz.ps1 -MaxTotalTimeSeconds 3600 -Workers 8
```

**If a crash is found**, libFuzzer writes the triggering input to
`fuzz/artifacts/<target>/crash-<hash>`. Reproduce and minimize it:

```bash
cargo +nightly fuzz run decode fuzz/artifacts/decode/crash-<hash>   # reproduce
cargo +nightly fuzz tmin decode fuzz/artifacts/decode/crash-<hash>  # minimize
```

Then add the minimized input as a golden vector / unit test in
`src/binary_json/` and fix the codec. The **corpus in `fuzz/corpus/<target>/`
persists across runs** — keep it (or copy it between machines) to accelerate
subsequent sessions.

Sizing guidance: `job time ≈ 1 min (compile) + N_targets × per-target budget`.
On an 8-vCPU VM, `-workers=8` roughly 2× the throughput seen in CI (~3.4K
exec/s/worker in the first run), so a 1-hour/target soak explores tens of
millions of inputs per target.

## Seeding the corpus from the golden vectors

Seeding libFuzzer with **valid** frames lets it mutate outward from real wire
shapes and reach the interesting error paths far faster than blind byte flips.
The [golden vectors](../testdata/binary_json_vectors.json) already contain every
marker family as space-separated hex. Materialize them into the `decode` corpus:

PowerShell:

```powershell
$dir = "fuzz/corpus/decode"; New-Item -ItemType Directory -Force $dir | Out-Null
(Get-Content ../testdata/binary_json_vectors.json | ConvertFrom-Json) | ForEach-Object {
  $bytes = $_.binary -split '\s+' | ForEach-Object { [Convert]::ToByte($_, 16) }
  [IO.File]::WriteAllBytes("$dir/$($_.name)", [byte[]]$bytes)
}
```

bash + jq + xxd:

```bash
mkdir -p fuzz/corpus/decode
jq -r '.[] | "\(.name) \(.binary)"' ../testdata/binary_json_vectors.json |
while read -r name hex; do
  echo "$hex" | tr -d ' ' | xxd -r -p > "fuzz/corpus/decode/$name"
done
```

The same corpus works for `from_slice`, `transcode_to_text`, and
`decode_reencode_roundtrip` (all consume raw binary buffers); copy or point
`--corpus` at `fuzz/corpus/decode`.

## Notes

- `corpus/`, `artifacts/`, and `target/` are git-ignored (regenerated locally / in CI).
- These targets are **offline** (no live account), so they are cheap enough to
  run in CI as a nightly job or a time-boxed smoke check on PRs touching
  `binary_json`.
- A reproducible crash should be reduced with `cargo fuzz tmin`, added as a
  golden vector / unit test in `src/binary_json/`, and fixed there.

## Windows

`cargo-fuzz` builds on **libFuzzer** (`-fsanitize=fuzzer`), which the Windows
MSVC target does not support — `cargo fuzz run` fails to link on Windows. Use
**WSL2** or a **Linux** box. On Windows, the always-on decoder robustness
coverage lives in `src/binary_json/fuzz_tests.rs` (random / truncated / corrupted
buffers into `decode`) and runs on stable via `cargo test -p
azure_data_cosmos_driver --lib fuzz`.

## CI

Fuzzing runs as a **non-blocking leg of the existing `sdk/cosmos/ci.yml`** — an
`AdditionalMatrixConfigs` entry ([`sdk/cosmos/fuzz-matrix.json`](../../fuzz-matrix.json))
that adds one **Linux + nightly** job (cargo-fuzz/libFuzzer is Linux-only). It
carries `ContinueOnError: "true"`, so a discovered crash reports "succeeded with
issues" instead of blocking merge. The job's test-setup hook
([`Invoke-CosmosTestSetup.ps1`](../../eng/scripts/Invoke-CosmosTestSetup.ps1),
gated on `AZURE_COSMOS_FUZZ=1`) calls
[`Run-BinaryJsonFuzz.ps1`](../../eng/scripts/Run-BinaryJsonFuzz.ps1), which
installs cargo-fuzz, seeds each corpus from the golden vectors, and runs every
target under a wall-clock budget that scales by build reason:

- **PR / CI smoke** — ~120 s/target (fast, non-blocking).
- **Weekly deep run** — ~30 min/target on the scheduled build.

Because it rides the already-registered `cosmos - ci` pipeline, the fuzz job
shows up automatically as a job on every cosmos PR — no separate pipeline to
register. Crash inputs are published as the `fuzz-crashes` build artifact so a
failure can be reproduced and minimized (`cargo fuzz tmin`).

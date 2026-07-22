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

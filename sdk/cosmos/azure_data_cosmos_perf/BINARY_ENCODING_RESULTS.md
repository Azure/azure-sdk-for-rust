# Binary encoding: measured impact

Measured against a live Cosmos DB account with
[`binary_payload_ab`](src/bin/binary_payload_ab.rs), which installs a custom
`TransportClient` and counts bytes at the wire boundary — request bodies *after*
binary transcoding, response bodies *before* it. Headers excluded, no content
encoding negotiated. Text and binary arms are interleaved round by round.

**Documents are real, not synthetic.** Both runs use `--profile corpus`,
sampling the 29 real-world JSON files under `testdata/` — the same corpus the
binary round-trip fuzzer draws its shapes from. 500 documents were seeded from a
pool of ~39,000 harvested objects:

| Statistic | Text JSON size |
| --- | ---: |
| min | 49 B |
| p50 | 310 B |
| p95 | 3369 B |
| max | 81566 B |
| total seeded | 797 KiB |

Two independent runs, each 500 docs, 25 iterations × 3 rounds, 75 operations per
workload per mode. Run 2 corrected two measurement artifacts found in run 1 (see
[Corrections](#corrections-applied-between-runs)); the payload figures agree to
within a point across both.

## The control arm

`binary+text_resp` sends binary requests but asks the service for text
responses. On queries it returns **0% binary** and byte counts identical to
plain text — so on the query workloads it is not a third configuration at all,
it is a **second copy of the text arm**.

That makes it a free noise control. Any latency delta it reports is measurement
noise by construction, because there is nothing different about the bytes.

| Workload | `binary+text_resp` resp bytes vs text | Its p50 vs text |
| --- | ---: | ---: |
| `SELECT *` | +0.0% | **+7.4%** |
| `ORDER BY` | +0.0% | **+18.4%** |
| `SELECT c.id, c.seq` | −0.1% | **+5.3%** |

**The query-latency noise floor for run 2 is therefore roughly ±18%.** Every
latency claim below is judged against it. This is the single most important
number in this document: without it, the −13% on `SELECT *` would look like a
result.

## 1. Payload size — the solid finding

Reproducible to within a point across both runs, and far outside any noise band.

| Workload | Run 1 | Run 2 |
| --- | ---: | ---: |
| `ORDER BY` response | −48.5% | **−48.6%** |
| `SELECT *` response | −46.8% | **−46.8%** |
| `SELECT c.id, c.seq` response | −40.3% | **−39.5%** |
| Point read response | −45.0% | −29.8% |
| Write request (create/replace/upsert) | −5.7% | **−7.8%** |

`SELECT *` reproduced to four significant figures (1255920 → 667530 in run 1;
1313763 → 698551 in run 2). This is not a noisy measurement.

Even the narrow projection — two fields per item, almost no structure left to
compress — still gives up ~40%.

**Point reads vary by document.** The −45% / −29.8% spread is not
inconsistency: run 2 strides across the whole seeded set rather than reading
only the first 25 documents, so it samples different corpus shapes. Savings are
document-dependent; **30–48% is the honest range**, not a single number.

## 2. Latency — mostly indistinguishable from noise

| Workload | text p50 | binary p50 | delta | vs ±18% floor |
| --- | ---: | ---: | ---: | --- |
| `SELECT c.id, c.seq` | 431.61 ms | 243.59 ms | **−43.6%** | **real** |
| `SELECT *` | 544.27 ms | 472.12 ms | −13.3% | within noise |
| `ORDER BY` | 551.49 ms | 510.95 ms | −7.4% | within noise |
| Point read | 18.25 ms | 18.38 ms | +0.7% | within noise |
| Point create | 24.10 ms | 23.59 ms | −2.1% | within noise |
| Point replace | 24.10 ms | 23.18 ms | −3.8% | within noise |
| Point upsert | 23.16 ms | 24.90 ms | +7.5% | within noise |
| Point delete | 22.85 ms | 22.76 ms | −0.4% | within noise |

Only the projection query clears the noise floor — and it does so in **both**
runs (−39.9%, then −43.6%), which is what makes it credible.

The defensible claim is therefore: **binary encoding costs no measurable
latency, and materially improves it on at least some query shapes.** Run 1's
across-the-board −20% was flattered by a slow text arm; it did not reproduce.

Point-operation deltas should be read as zero. `point_delete` has no body in
either direction and *cannot* be affected by encoding, yet it moves ±2% between
runs — that is the point-operation noise floor.

## 3. RU reduction on queries

| Workload | text RU/op | binary RU/op | delta (run 2) | delta (run 1) |
| --- | ---: | ---: | ---: | ---: |
| `SELECT *` | 54.45 | 46.33 | **−14.9%** | −14.7% |
| `ORDER BY` | 75.87 | 66.95 | **−11.8%** | −11.5% |
| `SELECT c.id, c.seq` | 41.49 | 41.34 | −0.4% | −0.3% |

Reproducible to within 0.3 points across runs. RU is server-reported and not
subject to client-side timing noise, which is why it is stable where latency is
not.

The saving tracks response size, so it appears only where the response is large.
The narrow projection returns little data and its RU is dominated by scan cost
rather than serialization, hence the flat result.

Writes show a consistent but negligible −0.1% to −0.2%. This is a direct billing
reduction, not just bandwidth.

## 4. `request_text_response = true` forfeits the benefit entirely

| Workload | text B/op | binary+text_resp B/op | delta |
| --- | ---: | ---: | ---: |
| `SELECT *` | 1313763 | 1313763 | +0.0% |
| `ORDER BY` | 1394495 | 1394495 | +0.0% |
| `SELECT c.id, c.seq` | 30746 | 30720 | −0.1% |

Byte-for-byte identical on two of three workloads, at **0% binary responses**.

**The flag is asymmetric.** Point reads in this same mode came back **100%
binary** and kept the full saving (530 B, identical to the `binary` arm). So the
option is honored for queries and silently ignored for point reads. A user who
enables binary encoding *and* sets this flag pays the setup cost, gets zero
query benefit, and gets binary responses anyway on the path where they
explicitly asked for text — with no diagnostic indicating either.

## 5. Write-side savings

| Workload | text | binary | delta |
| --- | ---: | ---: | ---: |
| Point create | 565 B | 521 B | **−7.8%** |
| Point replace | 565 B | 521 B | **−7.8%** |
| Point upsert | 565 B | 521 B | **−7.8%** |

Both binary arms produce byte-identical requests (521 B), exactly as they should
— which is what confirms run 1's apparent +2.4% penalty on `binary+text_resp`
was the id-length artifact described below, not a real effect.

Request-side savings are smaller than response-side because the request carries
one document while a query response carries a thousand, so per-request fixed
overhead dominates. Encoding never *costs* request bytes.

## Corrections applied between runs

Run 1 contained two artifacts, both now fixed in the harness:

1. **Write documents were unrepresentative.** Every write workload passed index
   `0` to the generator, so all writes sent one fixed ~120 B document rather
   than the seeded distribution (p50 310 B, p95 3369 B). Writes now stride
   across the whole seeded set via `document_index()`, which is why the measured
   request grew from 122 B to 565 B and the saving from −5.7% to −7.8%.
2. **Mode names leaked into request bytes.** Document ids embed the mode tag and
   travel inside the body, so `tmp-binary+text_resp-1-0` carried 12 bytes more
   than `tmp-text-1-0`. This inflated the `binary+text_resp` request count and
   produced a spurious +2.4% "regression". Mode tags are now fixed-width
   (`m0`/`m1`/`m2`) with zero-padded round and iteration numbers, so ids are the
   same length in every arm.

## Threats to validity

- **Latency noise dominates.** The control arm puts query-latency noise at ±18%
  in run 2. Only effects larger than that are reportable, and only the
  projection query qualifies.
- **No compression.** No content encoding was negotiated. Under gzip the
  relative advantage of binary encoding would shrink, since much of what binary
  removes — repeated property names — is exactly what a compressor handles well.
  **This is the most important untested dimension** and the first thing a
  reviewer will ask about.
- **Body bytes, not packet bytes.** Headers and TLS/HTTP framing are excluded.
  Real wire savings are slightly lower in percentage terms because header
  overhead is constant across modes.
- **Container accumulates across runs.** Items per query rose from 990 to 1065
  between runs as documents from earlier runs persisted. This is constant across
  arms within a run, so the A/B comparison holds, but absolute byte totals are
  not comparable between runs — only the percentages are.
- **Corpus is not a workload.** Real applications read a skewed subset of their
  data; this harness strides uniformly across the seeded set.
- **Single region, single account.** No cross-region or throttled-account data.

## Reproducing

```powershell
$env:AZURE_COSMOS_CONNECTION_STRING = "AccountEndpoint=...;AccountKey=...;"

cargo run --release -p azure_data_cosmos_perf --bin binary_payload_ab -- `
  --application-region "<region>" `
  --profile corpus `
  --docs 500 --iterations 25 --rounds 3 --include-text-response-mode
```

Always pass `--include-text-response-mode`: it costs one extra arm and buys the
noise control that makes the latency numbers interpretable.

### Document shapes

`--profile` selects what the documents under test look like. Binary encoding
compresses structure — property names, numbers, booleans — but not string
contents, so the shape is the single biggest factor in the measured savings.

| Profile | Shape | Why it is included |
| --- | --- | --- |
| `simple` | Small, flat, one number array | Best case for text JSON, so a conservative lower bound |
| `rich` | Nested business document, mixed types | Representative of typical application data |
| `huge` | Deep nesting, wide arrays, numeric edge cases | Upper bound; stresses the numeric encoder |
| `corpus` | Real documents sampled from `testdata/*.json` | Shapes not chosen by the harness author |

`corpus` is the profile to quote in review. It draws from 29 real-world JSON
files — airline delays, Bitcoin transactions, food facts, meteorite landings,
OpenAI embeddings, Reddit posts, volcanoes, log data, and more — harvesting
~39,000 individual documents (capped per file by `--corpus-per-file`). Cosmos
system properties (`_rid`, `_ts`, …) and any existing `id`/`partition_key` are
stripped, then the harness stamps its own.

Every profile is deterministic given `--seed`, so all modes see byte-identical
documents and a run reproduces exactly.

## Notes

- `binary%` on queries was 91–92% — roughly one non-binary request per query
  execution, which is the query-plan fetch (legitimately text). That 75 plan
  fetches occur for 75 executions of the *same* query string suggests the plan
  is not being cached; unrelated to binary encoding, but worth a separate look.
- `point_read` shows 0% binary in the text arm and 100% in both binary arms, as
  expected.
- Per-item and per-operation columns coincide because every mode returned
  exactly the same item count per query workload — container size is held
  constant across arms by deleting each round's created documents before the
  queries run.

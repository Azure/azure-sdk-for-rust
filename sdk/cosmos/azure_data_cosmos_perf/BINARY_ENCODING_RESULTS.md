# Binary encoding: measured impact

Measured against a live Cosmos DB account with
[`binary_payload_ab`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_perf/src/bin/binary_payload_ab.rs),
which installs a custom `TransportClient` and counts bytes at the wire boundary
— request bodies *after* binary transcoding, response bodies *before* it.
Headers excluded, no content encoding negotiated. Text and binary arms are
interleaved round by round.

**Runs 1–4 use real documents, not synthetic ones**: `--profile corpus` samples
the 29 real-world JSON files under `testdata/` — the same corpus the binary
round-trip fuzzer draws its shapes from. Runs 5 and 6 use the synthetic `simple`
and `huge` profiles, which generate near-uniform documents. That difference is a
confounder wherever the two are compared; it is called out where it bites.

Six runs, drawing on **two different corpus samples** plus two synthetic
profiles at opposite ends of the size range. That distinction turns out to
matter more than anything else in this document.

| Run | Sample | Config | Purpose |
| --- | --- | --- | --- |
| 1 | A | 500 docs, 25 × 3 | First measurement; contained two artifacts |
| 2 | A | 500 docs, 25 × 3 | Artifacts corrected |
| 3 | A | 500 docs, 25 × 3 | Regression check after the negotiation-derived encoding change |
| 4 | **B** | 800 docs, 30 × 4 | Different seed and 3× the per-file pool |
| 5 | `simple` | 500 docs, 40 × 5 | After `request_text_response` stopped forfeiting the wire encoding |
| 6 | `huge` | 300 docs, 40 × 5 | Large documents (p50 5347 B) — the opposite end of the size range from run 5 |

> **Runs 1–4 predate the `request_text_response` change.** In those runs the
> `binary+text_resp` arm returned text on the wire; from run 5 on it returns
> binary and transcodes client-side. Sections that depend on that arm are marked
> and carry both readings. Payload, RU, and latency figures for the `text` and
> `binary` arms are unaffected and remain comparable across all six runs.

Run 5 seeds a much narrower document distribution than either corpus sample (min
434 B, p50 464 B, p95 477 B, max 477 B — essentially uniform), so its absolute
percentages are not comparable to runs 1–4. It is included for the
`request_text_response` behavior and for the two `SkipTake` workloads, which
older runs did not exercise.

Run 6 is the same harness at the other extreme: 300 documents, min 4410 B, p50
5347 B, p95 5988 B, max 6184 B — a ~11× larger median than run 5, and also
near-uniform. Runs 5 and 6 are directly comparable to each other, which is what
makes them useful: they isolate document size while holding shape roughly
constant.

Sample A drew 500 documents from a pool of ~39,000 harvested objects; sample B
drew 800 from ~95,000 using a different `--seed`. The two samples are not
interchangeable:

| Statistic | Sample A | Sample B |
| --- | ---: | ---: |
| min | 49 B | 45 B |
| p50 | 310 B | 279 B |
| p95 | 3369 B | 3601 B |
| max | 81566 B | 68812 B |
| total seeded | 797 KiB | 794 KiB |
| **mean** | **1632 B** | **1016 B** |

Sample B packs nearly the same total bytes into 60% more documents, so its mean
document is ~38% smaller. Read that row before reading any percentage below.

## The control arm — valid for runs 1–4 only

In runs 1–4, `binary+text_resp` sent binary requests but asked the service for
text responses. On queries it returned **0% binary** and byte counts identical
to plain text — so on the query workloads it was not a third configuration at
all, it was a **second copy of the text arm**.

That made it a free noise control. Any latency delta it reported was measurement
noise by construction, because nothing about the bytes differed.

Its p50 versus text — noise by construction, in those runs:

| Workload | Run 2 | Run 3 | Run 4 |
| --- | ---: | ---: | ---: |
| `SELECT *` | +7.4% | −3.4% | +0.1% |
| `ORDER BY` | **+18.4%** | +7.5% | +1.5% |
| `SELECT c.id, c.seq` | +5.3% | +3.9% | −2.1% |
| **implied noise floor** | **±18%** | **±8%** | **±2%** |

The floor is itself unstable, varying nine-fold across runs of the same
workload. Latency claims below are judged against the **worst** observed floor,
not the most recent one — a quiet run does not retroactively license a noisy
one's numbers.

Without this control, run 2's −13% on `SELECT *` would have looked like a
result.

**This construct no longer exists.** From run 5 the arm negotiates binary on the
wire, so it is a genuine third configuration and its deltas are real, not noise.
The ±18% floor derived above is retained as the standing noise estimate because
it was measured honestly, but **no future run reproduces it for free**.
Re-establishing a noise floor now requires either a duplicated arm or repeated
runs of the same configuration — neither of which the harness currently does.
Treat latency claims made from run 5 onward as having *no* contemporaneous noise
control.

## 1. Payload size — direction is certain, magnitude is not

| Workload | Run 1 (A) | Run 2 (A) | Run 3 (A) | Run 4 (**B**) |
| --- | ---: | ---: | ---: | ---: |
| `ORDER BY` response | −48.5% | −48.6% | −48.6% | **−42.5%** |
| `SELECT *` response | −46.8% | −46.8% | −46.8% | **−40.1%** |
| `SELECT c.id, c.seq` response | −40.3% | −39.5% | −39.5% | −39.8% |
| Point read response | −45.0% | −29.8% | −32.2% | −26.8% |
| Write request | −5.7% | −7.8% | −7.8% | −6.3% |

Within sample A the numbers are essentially fixed: `SELECT *` landed on −46.8%
three times running, and run 3's `ORDER BY` text arm returned 1394495 bytes —
*identical to run 2's*. Byte counting is not the noisy part of this experiment.

**Sample B moved every figure down by 5–7 points**, well outside that. A
prediction of −46% to −49% was recorded before run 4 and it failed. The savings
are **dataset-dependent, and the earlier "−47%" was over-specific.**

**The honest band is −40% to −49% on large query responses**, −27% to −45% on
point reads, and **−6% to −26% on write requests** — see
[§5](#5-write-side-savings), where the synthetic profiles land far outside the
corpus range.

### Why the projection didn't move

One row is nearly constant across both samples: the projection, at −40.3% /
−39.5% / −39.5% / −39.8%.

That is exactly what should happen, and it is the best internal check available.
`SELECT c.id, c.seq` discards the source document and returns a fixed two-field
shape, so its savings cannot depend on what the documents looked like. The rows
that *do* depend on document shape — `SELECT *`, `ORDER BY`, point read — are
precisely the rows that moved.

So the variation is not measurement error. It is real sensitivity to document
composition, and across the two corpus samples it tracks the mean document size
(1632 B → 1016 B).

**The size mechanism does not survive runs 5 and 6.** The original explanation
was that binary's per-value type markers amortize worse over smaller documents,
which predicts smaller savings as documents shrink. Response savings are instead
flat across an ~11× median-size range: ~40–45% at 464 B (run 5), ~40–48% at
1.0–1.6 KB (corpus), ~39.5–44.5% at 5347 B (run 6). And the **request** saving
moves the *opposite* way to the prediction — −25.9% at 464 B versus −20.3% at
5347 B.

The caveat that keeps this from being a clean refutation: the corpus samples are
real and skewed while runs 5 and 6 are synthetic and near-uniform, so string
fraction is confounded with size. Size alone does not explain the corpus spread,
but a size-plus-string-fraction story is still open. Pinning it down needs
samples deliberately stratified by both.

## 2. Latency — one real win, the rest inside the noise

Binary p50 versus text, across all four runs:

| Workload | Run 1 | Run 2 | Run 3 | Run 4 | Verdict |
| --- | ---: | ---: | ---: | ---: | --- |
| `SELECT c.id, c.seq` | −39.9% | −43.6% | −44.3% | −44.1% | **real** |
| `SELECT *` | −20.1% | −13.3% | −22.3% | −10.4% | consistent sign |
| `ORDER BY` | −20.3% | −7.4% | −17.7% | −2.3% | consistent sign |
| Point ops | ±noise | ±noise | ±noise | ±noise | zero |

The projection query is the one unambiguous result: four independent runs within
4.4 points of each other, on a workload whose byte saving is also the most
stable. It clears even the ±18% floor.

`SELECT *` and `ORDER BY` are **negative in all four runs** — eight of eight
query-latency measurements favour binary, which is unlikely by chance — but the
magnitude swings by a factor of three and run 4's −2.3% on `ORDER BY` is
indistinguishable from zero. Report the direction, not the number.

The defensible claim: **binary encoding materially improves latency on
projection-style queries, and probably helps on large result sets by an amount
this harness cannot yet pin down.** It does not cost latency on any query
workload measured.

Point-operation deltas should be read as zero, *with one exception*.
`point_delete` has no body in either direction and *cannot* be affected by
encoding, yet it moved +3.6% in run 2, +6.9% in run 4, and −3.3% in run 6 — that
is the point-operation noise floor, measured directly.

### Point read is slower on binary, twice, unexplained

An earlier version of this document claimed binary "never costs latency". Runs 5
and 6 both contradict that on one workload:

| Run | binary p50 Δ vs text | `bin+text_resp` p50 Δ vs text | binary p95 | text p95 |
| --- | ---: | ---: | ---: | ---: |
| 5 (`simple`) | +1.4% | +7.4% | — | — |
| 6 (`huge`) | **+11.3%** | +3.2% | **89.66 ms** | **57.59 ms** |

Run 6's p50 is well outside that run's 3.3% floor, and the tail is worse: both
binary arms land near 89 ms p95 against text's 57.59 ms. Two independent arms
agreeing makes arm-position noise unlikely.

It is **not** the client-side transcode: client-side work (`p50` minus
`http p50`) is 0.21 ms on both binary arms, identical. But that comparison is
binary-against-binary, and the phenomenon is binary-against-**text** — binary
decode is itself client-side work, so these two arms cannot isolate client from
server. What can be said is that a 39.5% smaller response is arriving slower,
which is the opposite of what should happen, and **the cause is unknown**. Two
samples pointing the same way is suggestive, not conclusive — **this needs a
third, plus the text arm's own `p50 − http p50`, before any claim is made about
point-read latency in either direction.**

## 3. RU reduction on queries

| Workload | Run 1 | Run 2 | Run 3 | Run 4 |
| --- | ---: | ---: | ---: | ---: |
| `SELECT *` | −14.7% | −14.9% | −14.9% | **−11.6%** |
| `ORDER BY` | −11.5% | −11.8% | −11.8% | **−9.4%** |
| `SELECT c.id, c.seq` | −0.3% | −0.4% | −0.4% | −0.4% |

Runs 2 and 3 agree to the cent (54.45 → 46.33 RU both times). RU is
server-reported and immune to client-side timing noise, which is why it is
stable where latency is not.

Sample B again shifts the magnitude down, in lockstep with its smaller payload
saving — further evidence the two are the same underlying effect.

Runs 5 and 6 extend the range in both directions, and quoting a single band
across query shapes hides the more useful finding:

| Workload | Run 5 (`simple`) | Run 6 (`huge`) |
| --- | ---: | ---: |
| `SELECT *` | −12.5% | **−18.9%** |
| `ORDER BY` | −9.9% | **−15.5%** |
| `TOP` | −6.1% | −11.6% |
| `ORDER BY … OFFSET/LIMIT` | −5.2% | **−5.2%** |
| `SELECT c.id, c.seq` | −0.4% | **−0.4%** |

**The overall band is −0.4% to −18.9%, and it should be read by query shape, not
as one range.** Whole-document queries improve as documents grow — run 6 is the
best RU result of any run. But `OFFSET/LIMIT` and the projection reproduced *to
the decimal* between runs 5 and 6 despite an ~11× document-size change. Landing
on the same figure twice across independent runs means those are structural, not
spread: queries returning a small bounded or projected payload get no meaningful
RU benefit, reliably.

The saving tracks response size, so it appears only where the response is large.
The narrow projection returns little data and its RU is dominated by scan cost
rather than serialization, hence the flat result in every run.

`TOP` was previously called unstable. With the phantom third figure removed it is
not: −6.1% at 464 B and −11.6% at 5347 B is the same direction and comparable
magnitude as `SELECT *` (−12.5% → −18.9%) and `ORDER BY` (−9.9% → −15.5%), so it
fits the whole-document pattern above rather than contradicting it. It is a
bounded window over full documents, which is why it behaves like them and unlike
the projection.

Writes in runs 1–4 show a consistent but negligible −0.1% to −0.2%. Run 6 does
not reproduce even that, landing at +0.0% to +0.3% (see section 5). Treat write
RU as unchanged by encoding — the saving is request bandwidth only.

## 4. `request_text_response = true` — keeps the wire saving, costs a transcode

This is the behavior that changed. Previously the flag forfeited binary
negotiation on queries entirely; now it negotiates binary on the wire and
transcodes to text before handing items back.

### Before (runs 1–4)

| Workload | text B/op | binary+text_resp B/op | delta |
| --- | ---: | ---: | ---: |
| `SELECT *` | 1797666 | 1797666 | +0.0% |
| `ORDER BY` | 1920562 | 1920562 | +0.0% |
| `SELECT c.id, c.seq` | 46227 | 46227 | +0.0% |

Run 4 figures; byte-for-byte identical on all three workloads, at **0% binary
responses**, while point reads in the same mode came back 100% binary. The flag
was honored for queries and silently ignored for point reads.

### After (runs 5 and 6)

| Workload | binary% | resp B/op vs text | RU/op vs text |
| --- | ---: | ---: | ---: |
| `SELECT *` | **95%** | −43.4% / −43.3% | −12.5% / −18.9% |
| `ORDER BY` | **95%** | −45.3% / −44.5% | −9.9% / −15.5% |
| `SELECT c.id, c.seq` | **95%** | −40.0% / −40.1% | −0.4% / −0.4% |
| `ORDER BY … OFFSET/LIMIT` | **67%** | −32.2% / −32.2% | −5.2% / −5.2% |
| `TOP` | **50%** | −40.7% / −41.7% | −6.1% / −11.6% |
| Point read | **100%** | −44.4% / −39.5% | +0.0% / +0.0% |

Run 5 / run 6. `binary%` is identical in both runs. Every column matches the
plain `binary` arm to the last digit in both runs. RU is server-reported and
sensitive to response encoding, so identical RU is direct evidence the wire
genuinely went binary — not merely that the byte counter agreed. The asymmetry
between queries and point reads is gone.

### The cost: what the text hand-back actually buys and charges

Setting the flag now costs a client-side transcode instead of the wire saving.
Isolating that — `binary+text_resp` p50 against the `binary` arm, run 5:

| Workload | binary p50 | bin+text_resp p50 | run-5 delta |
| --- | ---: | ---: | ---: |
| Point read | 18.63 ms | 19.73 ms | +5.9% |
| `SELECT c.id, c.seq` | 416.68 ms | 431.51 ms | +3.6% |
| `TOP` | 58.01 ms | 59.15 ms | +2.0% |
| `ORDER BY … OFFSET/LIMIT` | 102.27 ms | 102.19 ms | −0.1% |
| `SELECT *` | 871.69 ms | 863.73 ms | −0.9% |
| `ORDER BY` | 907.64 ms | 888.79 ms | −2.1% |

The query rows scatter in **both directions** across a ±3.6% span, which is well
inside the ±8–18% floor measured in runs 2–4. On queries the transcode is not
measurable by this harness — it is swamped by the decode the caller would have
paid anyway.

Point read looked like the exception in run 5 — **+5.9% against binary** (and
+7.4% against text; see [§2](#point-read-is-slower-on-binary-twice-unexplained))
— on the theory that a single small document leaves no large parse for the
transcode to hide behind. **Run 6 does not reproduce it**: there
`binary+text_resp` point read is *faster* than binary (37.39 ms vs 40.34 ms).
The "consistently positive" reading was drawn from one run and does not hold. No
row now shows a reproducible transcode cost.

**Net:** on this harness the flag is close to free — no workload shows a
reproducible transcode cost — in exchange for keeping savings it previously threw
away: payload −32% to −45% and RU −0.4% to −18.9%, both by query shape (see §1
and §3; neither is a single number). That is a statement about what these runs
can resolve, not a claim that the transcode is free: it is real work, merely
smaller than the noise floor here. Its remaining caveat is behavioral rather than
performance: the transcode re-serializes, so property order and number spellings
(`1e20` → `1e+20`) are normalized rather than byte-preserved.

## 4b. Client-synthesized pages (`TOP`, `OFFSET`/`LIMIT`)

Runs 1–4 measured only workloads whose pages pass through from the service.
`TOP` and `OFFSET`/`LIMIT` are different: the driver synthesizes the page
client-side, so the **emitted** item encoding is chosen by the driver rather
than by the wire. Runs 5 and 6 add both.

| Workload | resp B/op | RU/op | binary% |
| --- | ---: | ---: | ---: |
| `ORDER BY … OFFSET 5 LIMIT 50` | −32.2% / −32.2% | −5.2% / −5.2% | 67% |
| `SELECT TOP 50 *` | −40.7% / −41.7% | −6.1% / −11.6% | 50% |

Run 5 / run 6. Both save less than the full-scan queries because they return a
bounded window — fewer items to amortize the fixed per-response overhead over.
`binary%` is lower for the same reason: the query-plan fetch is legitimately
text and it is a larger share of a small request count.

The `OFFSET/LIMIT` row is identical to the decimal across an ~11× change in
document size, which is a strong sign the bounded-window ceiling is structural.
`TOP`'s RU behavior tracks document size and is discussed in
[§3](#3-ru-reduction-on-queries).

Both arms agree with each other exactly in both runs, which is the point of
including them: these are the two paths where a bug in emitted-vs-wire encoding
selection would show up, and it does not.

## 5. Write-side savings

| Workload | text | binary | delta |
| --- | ---: | ---: | ---: |
| Point create | 514 B | 482 B | **−6.3%** |
| Point replace | 514 B | 482 B | **−6.3%** |
| Point upsert | 514 B | 482 B | **−6.3%** |

Run 4 figures; sample A gave −7.8% on all three. Both binary arms produce
byte-identical requests in every run, exactly as they should — which is what
confirms run 1's apparent +2.4% penalty on `binary+text_resp` was the id-length
artifact described below, not a real effect.

The synthetic profiles save far more than the corpus, but the three points do
**not** order by document size:

| Run | median document | write request delta |
| --- | ---: | ---: |
| 4 (corpus B) | 279 B | −6.3% |
| 5 (`simple`) | 464 B | **−25.9%** |
| 6 (`huge`) | 5347 B | **−20.3%** |

The saving rises then falls, so no monotonic trend in document size fits. The
corpus figure is a long way below both synthetic ones, which points at the
corpus/synthetic difference — real documents repeat fewer property names and
carry more incompressible string content — dominating whatever size effect
exists. Quote the shape of the corpus being written, not a single number.

Request-side savings are smaller than response-side because the request carries
one document while a query response carries a thousand, so per-request fixed
overhead dominates. Encoding never *costs* request bytes.

None of this reduces RU. Run 6's writes save 20.3% of request bytes and are
charged **+0.0% / +0.2% / +0.3%** RU against text — write RU is billed on the
decoded document, not the wire bytes. (An earlier note in this document put
point-operation RU at "+0.0% exactly"; run 6 shows +0.2% on replace and +0.3% on
upsert. Negligible, but not exact.)

## Regression check: negotiation-derived response encoding

Runs 3 and 4 were taken after the driver stopped inferring a query's response
encoding by sniffing each page for the binary preamble and began deriving it
from the negotiated operation, and after `normalize_integral_floats` was added
to the binary→text transcode.

Run 3 held the seed and config fixed against run 2, so any delta was
attributable to that change. There was none on the wire:

| Metric | Run 2 | Run 3 |
| --- | ---: | ---: |
| `ORDER BY` text response | 1394495 | 1394495 |
| `ORDER BY` binary response | 716131 | 716104 |
| `SELECT *` binary response | 698551 | 698499 |
| RU `SELECT *` | 54.45 → 46.33 | 54.45 → 46.33 |

This is the expected result: the harness counts bytes at the socket, upstream of
everything that changed, and normalization runs on already-received bytes.

One risk was worth checking. `normalize_integral_floats` walks the whole decoded
tree on every transcode — a new per-page CPU cost paid **only** by the binary
arm. It is not visible: binary's latency lead widened in run 3 (`SELECT *`
−13.3% → −22.3%). Whatever the walk costs is swamped by binary having less data
to parse in the first place.

## Corrections applied between runs 1 and 2

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

- **Magnitude is dataset-dependent.** Changing the corpus sample moved every
  payload and RU figure by 5–7 points. Quote the band, not a point estimate, and
  expect a given application's savings to depend on its document shapes. A
  prediction that −47% would hold across samples was recorded before run 4 and
  was wrong.
- **Two samples is not a distribution.** The size/savings relationship is
  inferred from two points. It is consistent with the projection query's
  invariance, but it is not established.
- **Latency noise dominates, and the surviving control is narrow.** The control
  arm put query-latency noise at ±18%, ±8%, and ±2% in three consecutive runs.
  Only the projection query's win survives the worst of those. Since run 5 the
  `binary+text_resp` arm is a real configuration rather than a duplicated text
  arm, so **no run from 5 onward measures its own query noise floor**. What
  remains is `point_delete`, whose arms send byte-identical HTTP in every run
  and which therefore is a live control — ±6% in run 5, ±3.3% in run 6. That
  bounds point-operation noise but says nothing about the much larger query
  timings. Restoring a query-side floor is the most valuable thing to fix —
  cheapest is a fourth arm that duplicates `text`.
- **Arm rotation does not balance at 5 rounds over 3 arms.** Summing arm
  positions across run 6's round log gives 10 / 9 / 11 for text / binary /
  `bin+text_resp`. Run 5 has the *same* imbalance yet the opposite sign in its
  `bin+text_resp` deltas, so the imbalance is not a usable explanation for any
  particular result — it is just unremoved bias. Six rounds would balance
  exactly.
- **Stray requests inflate some text baselines.** Run 3's text arm logged 76
  requests for 75 point reads, and 902 for 900 `SELECT *` operations. That lifts
  the text byte total slightly and flatters the corresponding saving; it is why
  run 3's point-read figure (−32.2%) should not be read as better than run 2's
  (−29.8%) when the binary arm returned exactly 530 B in both.
- **No compression.** No content encoding was negotiated. Under gzip the
  relative advantage of binary encoding would shrink, since much of what binary
  removes — repeated property names — is exactly what a compressor handles well.
  **This is the most important untested dimension** and the first thing a
  reviewer will ask about.
- **Body bytes, not packet bytes.** Headers and TLS/HTTP framing are excluded.
  Real wire savings are slightly lower in percentage terms because header
  overhead is constant across modes.
- **Container accumulates across runs.** Items per query rose across runs as
  documents from earlier runs persisted. This is constant across arms within a
  run, so the A/B comparison holds, but absolute byte totals are not comparable
  between runs — only the percentages are.
- **Corpus is not a workload.** Real applications read a skewed subset of their
  data; this harness strides uniformly across the seeded set.
- **Single region, single account.** No cross-region or throttled-account data.

## Reproducing

```powershell
$env:AZURE_COSMOS_CONNECTION_STRING = "AccountEndpoint=...;AccountKey=...;"

# Sample A (runs 1-3)
cargo run --release -p azure_data_cosmos_perf --features binary-ab --bin binary_payload_ab -- `
  --application-region "<region>" `
  --profile corpus `
  --docs 500 --iterations 25 --rounds 3 --include-text-response-mode

# Sample B (run 4) - different seed, 3x the per-file pool
cargo run --release -p azure_data_cosmos_perf --features binary-ab --bin binary_payload_ab -- `
  --application-region "<region>" `
  --profile corpus --corpus-per-file 6000 --seed 987654321987654321 `
  --docs 800 --partitions 40 --iterations 30 --rounds 4 `
  --include-text-response-mode

# Run 5 - small uniform documents, exercises TOP and OFFSET/LIMIT
cargo run --release -p azure_data_cosmos_perf --features binary-ab --bin binary_payload_ab -- `
  --application-region "<region>" `
  --profile simple --docs 500 --iterations 40 --rounds 5 --include-text-response-mode

# Run 6 - large uniform documents (p50 5347 B), the size counterpart to run 5
cargo run --release -p azure_data_cosmos_perf --features binary-ab --bin binary_payload_ab -- `
  --application-region "<region>" `
  --profile huge --docs 300 --rounds 5 --iterations 40 --include-text-response-mode
```

Always pass `--include-text-response-mode`. Note what it now buys: through run 4
it doubled as a noise control, but since `request_text_response` began
negotiating binary on the wire it measures a real third configuration instead.
It is still worth running — it is the only arm that exercises the transcode path
— but it no longer establishes a noise floor.

The `binary-ab` feature is required: it turns on the driver's
`__internal_mocking` surface, which the harness needs to install its
byte-counting transport. It is off by default so a plain workspace build does
not pull that unstable surface into every other crate through feature
unification.

Run **both** samples. A single sample understates the spread — the mistake runs
1–3 made.

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
39,000 to 95,000 individual documents depending on `--corpus-per-file`. Cosmos
system properties (`_rid`, `_ts`, …) and any existing `id`/`partition_key` are
stripped, then the harness stamps its own.

Every profile is deterministic given `--seed`, so all modes see byte-identical
documents and a run reproduces exactly.

## Notes

- `binary%` on the large query workloads was 91–95% — roughly one non-binary
  request per query execution, which is the query-plan fetch (legitimately
  text). That one plan fetch occurs per execution of the *same* query string
  suggests the plan is not being cached; unrelated to binary encoding, but worth
  a separate look. The bounded workloads (`TOP` 50%, `OFFSET`/`LIMIT` 67%) are
  lower only because that same single plan fetch is a larger share of a much
  smaller request count.
- **Request counts vary by ±9 between arms that issue identical requests.** Run
  5 logged 4000/4004/4007 on `ORDER BY` and 400/403/400 on `TOP`. An earlier
  reading of this document attributed the extra requests to the
  `binary+text_resp` arm specifically; run 5 shows them landing on the plain
  `binary` arm just as often, so it is retry/throttle jitter and not
  arm-correlated. Use the per-item columns, which are insensitive to it.
- `point_read` shows 0% binary in the text arm and 100% in both binary arms, as
  expected.
- Per-item and per-operation columns coincide because every mode returned
  exactly the same item count per query workload — container size is held
  constant across arms by deleting each round's created documents before the
  queries run.

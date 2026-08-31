# Plan: cross-encoding matrix for binary/text (queries + point ops)

Status: **designed, not implemented.** Awaiting validation of the part 1/2/3 A/B
results before writing code.

Target: `sdk/cosmos/azure_data_cosmos_perf/tests/binary_sampled_testdata.rs`

Goal: write a document in one encoding, read it back in the other, and look for
canonical differences in the result — and extend the existing arm coverage to
point operations, which today have none.

## What exists today

`build_client` enables binary at the client level (`:237`). `create_item`
(`:629`) and `read_item` (`:637`) pass no per-operation override, so both
inherit it. `write_options_with_content` (`:275`) sets only
`content_response_on_write`.

| Write | Read | Queries | Point ops |
| --- | --- | --- | --- |
| binary | binary | yes | yes |
| binary | text | yes — the `text` arm | no |
| binary | bin+txt_rsp | yes | no |
| text | any | no | no |

Point operations therefore have exactly **one** encoding configuration under
test, and nothing compares a point read across encodings. That matters because
the number-spelling bug this PR fixes was latent for untyped point reads on
`main`, and this test structurally could not have seen it.

## Structural facts (verified)

- `binary_encoding` is a per-operation override on `OperationOptions`
  (`options/operation_options.rs:153`).
- `ItemReadOptions::with_operation_options` (`options/item.rs:44`) and
  `ItemWriteOptions::with_operation_options` (`:89`) both exist. **No API work
  is needed.**
- **One client and one runtime is sufficient.** Separate SDK instances are not
  required; the per-operation override is the same mechanism the query arms
  already use (`:385`).
- `BinaryEncodingOptions::enabled` governs **both directions** within a single
  operation (`options/binary_encoding.rs:57`): it encodes the request body *and*
  advertises that binary responses are accepted. "Text request, binary response"
  is not expressible on one operation. The matrix is therefore *write-op
  encoding x read-op encoding*, not *request x response*.

## Steps

1. **Write-encoding dimension.** Add `WriteEncoding { Text, Binary }`. Extend
   `write_options_with_content` to set `operation.binary_encoding` alongside the
   existing `content_response_on_write`.
2. **Two seeded cohorts.** Seed each corpus document under both write modes,
   tagged with a `writeMode` field, preserving the existing `testRun` scoping.
   This roughly doubles seed time — part 1 ran 107s and was mostly seeding.
3. **Strict comparator.** Add a strict `Value`-equality path. Do **not** reuse
   `json_equivalent` (`:408`) / `numbers_equivalent` (`:434`): they compare
   integral numbers as `i128`, so `3` and `3.0` compare equal. That comparator
   is precisely why the number-spelling bug survived a passing corpus run, and
   reusing it would make this test blind to the differences it exists to find.
4. **Point-op arm loop.** Mirror `run_query` (`:376`) for point reads so
   `read_item` runs under all three read arms instead of inheriting binary, and
   compare the create/replace write echo the same way.

Cardinality: 2 write modes x 3 read arms x 4 query shapes = 24 query
executions, plus the point-op matrix.

## Design decisions to settle before coding

**`bin+txt_rsp` cannot use strict equality.** Its text is re-serialized by the
driver, not the service's bytes: object keys come out sorted and numbers use
Rust's shortest round-trip rendering, so `1e20` renders as `1e+20`
(`options/binary_encoding.rs:60-72`). Under strict comparison that arm will
diverge from `text` by design. It needs a key-order-insensitive comparator that
still checks number spelling. Decide this first or the run produces a wall of
false positives.

## Prediction, recorded before the run

I expect this to find nothing. The service parses on ingest and imposes its own
number spelling regardless of what was sent — the live fidelity measurement
showed `f64(3.0)` written over **text** reading back as `u64(3)`. If a
text-write/binary-read difference does appear, that is a genuine service-side
finding and more valuable than a pass.

This is an inference from a single measurement on the text-write path, not a
verified claim.

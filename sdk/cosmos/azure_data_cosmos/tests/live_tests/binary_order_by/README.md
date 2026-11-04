# Live binary ORDER BY validation

This opt-in test creates an isolated database with two containers, each provisioned
at **20,000 RU/s**. Obtain approval for the resulting **40,000 RU/s total** before
running it. Replicated regions add provisioned-throughput charges. Use a test
account, not production credentials.

## Running the test

Load the connection string without printing it. The throughput variable is an
explicit cost opt-in, not a configurable throughput setting; selecting live or
ignored tests alone does not authorize provisioning these containers.

```powershell
$env:AZURE_COSMOS_CONNECTION_STRING = (Get-Content -LiteralPath '<local-connection-string-file>' -Raw).Trim()
$env:AZURE_COSMOS_TEST_MODE = 'required'
$env:AZURE_COSMOS_AUTH_MODE = 'key'
$env:AZURE_COSMOS_ORDER_BY_LIVE_THROUGHPUT = '20000'
cargo test -p azure_data_cosmos --all-features --test live binary_order_by::live_binary_order_by_matches_text_across_physical_ranges -- --ignored --exact --nocapture --test-threads=1
```

By default, the test exercises both `GatewayOnly` and `LocalPreferred` planning.
To run them separately, set `AZURE_COSMOS_ORDER_BY_LIVE_PLAN_MODE` to one of those
values. `LocalPreferred` can fall back to gateway planning.

## Coverage

The typed SDK runs the full text/binary query matrix against identical fixtures
in containers with matching physical range boundaries and populated ranges.
It covers ordering, windows, DISTINCT, tied keys, and document/scalar/array/null
projections at page-size hints of 1, 7, and 64. Resumable cases checkpoint up to
three times using the same client and a fresh client/runtime, comparing against
uninterrupted ordering in the same container. At most three shapes run concurrently.

Focused driver checks cover standalone binary output and text-response mode for
document, windowed, DISTINCT, array, and null results, plus a text-only control.
An observation transport verifies that the service actually returned binary query
pages; re-encoding text fallback alone cannot satisfy that check. Deterministic
pipeline tests cover source-page ownership and materialization timing.

## Cleanup and limits

The framework deletes the database on completion, error, or the 30-minute timeout.
Charges continue until cleanup completes. An externally terminated process can
bypass cleanup: use the printed `auto-test-...` database name to identify and
remove only the test's resources, and verify that the database is absent.

This is a correctness test, not a latency benchmark. It neither forces a partition
split nor guarantees which compact binary markers the service emits.

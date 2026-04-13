# azure_data_cosmos_benchmarks

Criterion microbenchmarks for the Azure Cosmos DB Rust driver. All benchmarks
replace the reqwest transport with an in-memory mock, so they measure driver
overhead only — routing, signing, retry state, response parsing, and session
token management — with no network I/O.

## Running the benchmarks

### Standard latency benchmark

```sh
cargo bench -p azure_data_cosmos_benchmarks --bench point_read
```

Results are written to `target/criterion/point_read/`.

### CPU flamegraph (pprof)

Pass `--profile-time <seconds>` to enable pprof sampling:

```sh
cargo bench -p azure_data_cosmos_benchmarks --bench point_read -- --profile-time 30
```

The flamegraph SVG is written to
`target/criterion/point_read/profile/point_read.svg`.

For readable symbol names, build with debug symbols:

```sh
RUSTFLAGS="-C debuginfo=1" cargo bench -p azure_data_cosmos_benchmarks --bench point_read -- --profile-time 30
```

### Heap allocation trace (dhat)

```sh
cargo bench -p azure_data_cosmos_benchmarks --bench heap_profile
```

This traces every allocation made during a single warm `execute_operation`
call and writes `dhat-heap.json` to the current working directory. Load it at
<https://nnethercote.github.io/dh_view/dh_view.html> to explore the
allocation call graph.

For useful backtraces in the dhat output, build with debug symbols:

```sh
RUSTFLAGS="-C debuginfo=1" cargo bench -p azure_data_cosmos_benchmarks --bench heap_profile
```

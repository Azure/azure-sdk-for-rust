# azure_data_cosmos_benchmarks

Criterion benchmarks for the Azure Cosmos DB Rust driver. All benchmarks
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
cargo bench -p azure_data_cosmos_benchmarks --profile bench --bench point_read -- --profile-time 30
```



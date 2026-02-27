# Azure Cosmos DB Performance Testing Tool

A CLI tool for performance and scale testing the Azure Cosmos DB Rust SDK. It runs
point reads, single-partition queries, upserts, and creates concurrently and reports
latency statistics at configurable intervals.

## Prerequisites

- Rust toolchain (MSRV 1.88)
- An Azure Cosmos DB account
  - The database, container, and results container will be created automatically if they don't exist (with `/partition_key` as the partition key path)
- For key auth: a Cosmos DB account key
- For AAD auth: a managed identity available in the hosting environment (e.g., Azure VM, App Service)

## Building

From the repository root:

```bash
cargo build -p azure_data_cosmos_perf
```

## Usage

### Key Authentication

```bash
cargo run -p azure_data_cosmos_perf -- \
  --endpoint https://<account>.documents.azure.com:443/ \
  --auth key \
  --key "<your-account-key>"
```

Or use the `AZURE_COSMOS_KEY` environment variable:

```bash
export AZURE_COSMOS_KEY="<your-account-key>"
cargo run -p azure_data_cosmos_perf -- \
  --endpoint https://<account>.documents.azure.com:443/ \
  --auth key
```

### AAD (Entra ID) Authentication

Uses `ManagedIdentityCredential`, which requires the tool to run in an Azure
environment with a managed identity assigned (e.g., Azure VM, App Service, or
Container Instance).

```bash
cargo run -p azure_data_cosmos_perf -- \
  --endpoint https://<account>.documents.azure.com:443/ \
  --auth aad
```

### Options

| Flag | Default | Description |
|------|---------|-------------|
| `--endpoint` | *required* | Cosmos DB account endpoint URL |
| `--database` | `perfdb` | Database name |
| `--container` | `perfcontainer` | Container name (partition key path must be `/partition_key`) |
| `--auth` | *required* | Authentication method: `key` or `aad` |
| `--key` | — | Account key (or set `AZURE_COSMOS_KEY` env var) |
| `--preferred-regions` | — | Comma-separated preferred regions (e.g., `"West US,East US"`) |
| `--excluded-regions` | — | Comma-separated excluded regions |
| `--exclude-regions-for` | `both` | Scope for excluded regions: `reads`, `writes`, or `both` |
| `--concurrency` | `50` | Number of concurrent operations |
| `--duration` | indefinite | Run duration in seconds |
| `--seed-count` | `1000` | Number of items to pre-seed |
| `--throughput` | `100000` | Throughput (RU/s) when creating the container |
| `--default-ttl` | `3600` | Default TTL in seconds for items (0 to disable) |
| `--report-interval` | `300` | Stats reporting interval in seconds |
| `--results-container` | `perfresults` | Container for storing perf results and error documents |
| `--results-endpoint` | — | Cosmos DB endpoint for results (omit to use same account as `--endpoint`) |
| `--results-database` | `perfdb` | Database name on the results account |
| `--results-auth` | same as `--auth` | Authentication method for the results account: `key` or `aad` |
| `--results-key` | — | Account key for results account (or set `AZURE_COSMOS_RESULTS_KEY` env var) |
| `--workload-id` | random UUID | Unique identifier for this workload instance (for multi-VM correlation) |
| `--no-reads` | `false` | Disable point read operations |
| `--no-queries` | `false` | Disable query operations |
| `--no-upserts` | `false` | Disable upsert operations |
| `--no-creates` | `false` | Disable create operations |

### Examples

Run reads only with 100 concurrent operations for 60 seconds:

```bash
cargo run -p azure_data_cosmos_perf -- \
  --endpoint https://myaccount.documents.azure.com:443/ \
  --auth key --key "$AZURE_COSMOS_KEY" \
  --no-queries --no-upserts --no-creates \
  --concurrency 100 --duration 60 --report-interval 10
```

Run all operations with preferred regions and custom database:

```bash
cargo run -p azure_data_cosmos_perf -- \
  --endpoint https://myaccount.documents.azure.com:443/ \
  --database mydb --container mycontainer \
  --auth aad \
  --preferred-regions "West US,East US" \
  --concurrency 200 --seed-count 5000
```

## Output

The tool prints periodic latency summaries like:

```text
--- Interval Report ---
  Process: CPU 45.2%, Memory 128.3 MB
  System:  CPU 12.8%, Memory 5.2 GB/16.0 GB
  Operation         Count   Errors        Min        Max       Mean        P50        P90        P99
  -------------------------------------------------------------------------------------------------------
  CreateItem          280        0      4.0ms     55.2ms     16.8ms     12.5ms     35.0ms     50.1ms
  QueryItems          312        0      3.2ms     45.1ms     12.4ms      9.8ms     28.3ms     41.2ms
  ReadItem            298        2      1.8ms     38.7ms      8.2ms      6.1ms     19.5ms     35.4ms
  UpsertItem          325        0      4.5ms     52.3ms     15.1ms     11.2ms     32.1ms     48.7ms
```

### Results Container

Periodic summary documents and individual error documents are written to the
results container (`--results-container`, default `perfresults`).

- **Summary documents**: Upserted at each reporting interval with latency
  percentiles, process metrics (CPU/memory), system metrics (CPU/memory), and
  workload ID per operation.
- **Error documents**: Written for each individual operation failure with the
  operation name, error message, source error chain, workload ID, and timestamp.
  Errors during the perf run never stop the workload — they are captured and
  reported but execution continues.

If the tool cannot write a result or error document (e.g., the results container
is temporarily unavailable), a warning is printed to stderr and the workload
continues unaffected.

### Separate Results Account

By default, results are stored on the same account being tested. To avoid adding
noise to your workload, use `--results-endpoint` to direct result/error documents
to a different Cosmos DB account:

```bash
cargo run -p azure_data_cosmos_perf -- \
  --endpoint https://workload.documents.azure.com:443/ \
  --auth key --key "$AZURE_COSMOS_KEY" \
  --results-endpoint https://results.documents.azure.com:443/ \
  --results-auth key --results-key "$AZURE_COSMOS_RESULTS_KEY"
```

### TTL

Containers are created with a default TTL (`--default-ttl`, default 1 hour).
Items automatically expire after this duration, keeping the container from
growing unboundedly during long or repeated runs. Set `--default-ttl 0` to
disable TTL.

### Create Operation

When enabled (the default), the `CreateItem` operation generates new items with
unique IDs and partition keys. Successfully created items are added to the
shared item pool so they become targets for subsequent read, query, and upsert
operations.

### Multi-Process Launcher

The `run_perf.sh` script launches multiple OS processes of the perf tool in
parallel. This is useful for saturating a Cosmos DB account beyond what a single
process can achieve.

```bash
# Launch 4 parallel processes, each with 50 concurrent tasks
./run_perf.sh --processes 4 \
  --endpoint https://myaccount.documents.azure.com:443 \
  --auth key --key "$AZURE_COSMOS_KEY" \
  --concurrency 50 --duration 600

# All standard perf tool flags are passed through to each process
./run_perf.sh --processes 8 \
  --endpoint https://myaccount.documents.azure.com:443 \
  --auth aad --no-queries --no-creates
```

The script builds the crate in release mode, spawns the requested number of
processes, and forwards `Ctrl+C` to all children for graceful shutdown.

### Testing Against a Specific SDK Commit

Use `--cosmos-commit` to build and run against a specific version of
`azure_data_cosmos`. This is useful for A/B performance comparisons across
SDK changes.

```bash
# Test against a specific commit
./run_perf.sh --cosmos-commit abc123 --processes 4 \
  --endpoint https://myaccount.documents.azure.com:443 \
  --auth key --key "$AZURE_COSMOS_KEY"

# Test against a branch
./run_perf.sh --cosmos-commit upstream/main --processes 2 \
  --endpoint https://myaccount.documents.azure.com:443 \
  --auth aad
```

The script checks out the SDK source at the given ref before building, then
restores the original source after the build completes (or on Ctrl+C/error).

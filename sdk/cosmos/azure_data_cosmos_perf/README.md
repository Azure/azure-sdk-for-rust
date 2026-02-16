# Azure Cosmos DB Performance Testing Tool

A CLI tool for performance and scale testing the Azure Cosmos DB Rust SDK. It runs
point reads, single-partition queries, and upserts concurrently and reports latency
statistics at configurable intervals.

## Prerequisites

- Rust toolchain (MSRV 1.85)
- An Azure Cosmos DB account
  - The database, container, and results container will be created automatically if they don't exist (with `/partition_key` as the partition key path)
- For key auth: a Cosmos DB account key
- For AAD auth: Azure CLI logged in (`az login`)

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

```bash
az login
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
| `--concurrency` | `50` | Number of concurrent operations |
| `--duration` | indefinite | Run duration in seconds |
| `--seed-count` | `1000` | Number of items to pre-seed |
| `--throughput` | `100000` | Throughput (RU/s) when creating the container |
| `--report-interval` | `300` | Stats reporting interval in seconds |
| `--results-container` | `perfresults` | Container for storing perf results and error documents |
| `--workload-id` | random UUID | Unique identifier for this workload instance (for multi-VM correlation) |
| `--no-reads` | `false` | Disable point read operations |
| `--no-queries` | `false` | Disable query operations |
| `--no-upserts` | `false` | Disable upsert operations |

### Examples

Run reads only with 100 concurrent operations for 60 seconds:

```bash
cargo run -p azure_data_cosmos_perf -- \
  --endpoint https://myaccount.documents.azure.com:443/ \
  --auth key --key "$AZURE_COSMOS_KEY" \
  --no-queries --no-upserts \
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
  Operation         Count   Errors        Min        Max       Mean        P50        P90        P99
  -------------------------------------------------------------------------------------------------------
  QueryItems          312        0      3.2ms     45.1ms     12.4ms      9.8ms     28.3ms     41.2ms
  ReadItem            298        2      1.8ms     38.7ms      8.2ms      6.1ms     19.5ms     35.4ms
  UpsertItem          325        0      4.5ms     52.3ms     15.1ms     11.2ms     32.1ms     48.7ms
```

### Results Container

Periodic summary documents and individual error documents are written to the
results container (`--results-container`, default `perfresults`).

- **Summary documents**: Upserted at each reporting interval with latency
  percentiles, process metrics, and workload ID per operation.
- **Error documents**: Written for each individual operation failure with the
  operation name, error message, source error chain, workload ID, and timestamp.
  Errors during the perf run never stop the workload — they are captured and
  reported but execution continues.

If the tool cannot write a result or error document (e.g., the results container
is temporarily unavailable), a warning is printed to stderr and the workload
continues unaffected.

## Extending with New Operations

To add a new operation type:

1. Create a new file in `src/operations/` (e.g., `delete_item.rs`).
2. Implement the `Operation` trait:

    ```rust
    use async_trait::async_trait;
    use azure_data_cosmos::clients::ContainerClient;
    use crate::operations::Operation;

    pub struct DeleteItemOperation { /* ... */ }

    #[async_trait]
    impl Operation for DeleteItemOperation {
        fn name(&self) -> &'static str { "DeleteItem" }
        async fn execute(&self, container: &ContainerClient) -> azure_core::Result<()> {
            // implementation
            Ok(())
        }
    }
    ```

3. Register it in `src/operations/mod.rs` by adding it to `create_operations()`.
4. Add a CLI flag (e.g., `--no-deletes`) in `src/config.rs`.

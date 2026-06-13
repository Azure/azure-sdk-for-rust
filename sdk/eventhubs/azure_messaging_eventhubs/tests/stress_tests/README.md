# Event Hubs Stress Tests

This folder contains the custom (non-harness) stress tests for Azure Event Hubs. Two scenarios ship today, each exposed as a clap subcommand:

1. **basic_publish_read_test** – bounded publish/read throughput + validation.
2. **continuous_send_receive_stress** – long-running durability/consistency loop (72h default) inspired by the C# sample.

## Overview

Each test registers a `StressTestSpec` (name, description, `configure(Command)`, `run(ArgMatches)`). The entrypoint (`stress_tests.rs`, `harness = false`) builds the CLI from the registry and either:

- Runs a specific subcommand if provided.
- Runs all registered tests sequentially with their default arguments when no subcommand is given.

Both tests use `dotenvy` to load credentials and run under `tokio` with structured logging via `tracing_subscriber`.

## Running the Test

### Running (custom harness)

These tests disable the standard test harness (`harness = false`) and rely on clap/dotenvy. Use the stress test binary:

```bash
# Run all registered stress tests with their defaults
cargo test --test stress_tests

# Run a specific subcommand
cargo test --test stress_tests basic_publish_read_test
cargo test --test stress_tests continuous_send_receive_stress

# Pass custom args to a subcommand
cargo test --test stress_tests -- basic_publish_read_test --events 1000 --producers 2 --consumers 2
cargo test --test stress_tests -- continuous_send_receive_stress --duration-hours 4 --min-batch 10 --max-batch 50

# More logs
RUST_LOG=info cargo test --test stress_tests

# Discover options
cargo test --test stress_tests -- --help
cargo test --test stress_tests -- basic_publish_read_test --help
cargo test --test stress_tests -- continuous_send_receive_stress --help
```

#### Command Line Options

##### **Basic Publish/Read**

- `--events, -e` (default: 100): Number of events to publish and read
- `--producers, -p` (default: 1): Concurrent producer tasks
- `--consumers, -c` (default: 1): Concurrent consumer tasks
- `--timeout, -t` (default: 120s): End-to-end timeout
- `--event-size` (default: 512 bytes): Payload size
- `--batch-size` (default: 10): Events per batch

##### **Continuous Send/Receive**

- `--duration-hours` (default: 72): Duration of the run
- `--min-batch` (default: 20): Minimum batch size
- `--max-batch` (default: 100): Maximum batch size
- `--min-delay` (default: 1s): Minimum delay between batches
- `--max-delay` (default: 10s): Maximum delay between batches

## Environment Variables

The following environment variables are required (loaded via `dotenvy`, so a `.env` file works):

- `EVENTHUBS_HOST`: Event Hubs namespace hostname (e.g., "my-namespace.servicebus.windows.net")
- `EVENTHUB_NAME`: Event Hub name
- `RUST_LOG`: Log level (optional, defaults to warnings)

## Behavior & Metrics (high level)

- **basic_publish_read_test**: Publishes a bounded set, tracks per-task publish counts, measures publish/consume durations and throughput, and validates end-to-end completion/count-based results (collects errors on timeouts or task join failures). It does not currently compare consumed events against the originally published payloads.
- **continuous_send_receive_stress**: Long-running loop that emits random batches across partitions, periodically logs heartbeat metrics (sent/received, missing, corrupted body/properties, producer/consumer failures), and emits a final summary including lost events.

Both tests log via `tracing` and exit non-zero on failure to make CI-friendly verdicts.

<!-- cSpell:ignore otlp OTLP otlphttp Jaeger otelcol dbs healthcheck -->
# Azure Cosmos DB observability soak harness

A runnable soak/load **tool** for validating the Azure Cosmos DB Rust SDK's
client-side observability layer end-to-end. It registers every built-in
diagnostics handler on a real `CosmosClient`, wires up an OpenTelemetry exporter,
and drives a configurable read/write/query workload — optionally with fault
injection — for as long as you like.

The design goal it validates: **quiet at steady state, rich on error.** A fast,
successful point read produces no span and no sampled log line; a failure or a
threshold-breaching slow operation emits a full diagnostics record.

This crate is a developer tool (`publish = false`); it is not part of the
supported SDK surface. See the
[Cosmos SDK project documentation](../docs/README.md) for the diagnostics
architecture and specification.

## What it wires up

- Registers, via `CosmosClientBuilder::with_diagnostics_handler`, in order:
  - `CosmosMetricsHandler` — the stable `db.client.operation.duration` histogram
    (plus opt-in development-tier metrics). *(feature `metrics`)*
  - `CosmosTracingHandler` — tail-sampled, backdated OpenTelemetry span trees.
    *(feature `distributed_tracing`)*
  - `SamplingLogHandler` — tail-sampled compact diagnostics log lines emitted
    through `tracing` (always registered).
- Installs global OpenTelemetry `SdkMeterProvider` + `SdkTracerProvider` backed by
  a selectable exporter:
  - `stdout` (default) — prints metrics/spans to the console; no infrastructure.
  - `otlp` — OTLP/gRPC to a local collector *(requires building with `--features otlp`)*.
  - `none` — install no exporter; only the sampled `tracing` log lines are emitted.
- Optionally injects faults (throttling, 503, timeouts, slow responses, …) so the
  error/threshold telemetry path is exercised. *(feature `fault_injection`)*

## Cargo features

| Feature | Default | Effect |
| --- | --- | --- |
| `metrics` | ✅ | Enables `azure_data_cosmos/metrics` and registers `CosmosMetricsHandler`. |
| `distributed_tracing` | ✅ | Enables `azure_data_cosmos/distributed_tracing` and registers `CosmosTracingHandler`. |
| `fault_injection` | ✅ | Enables `azure_data_cosmos/fault_injection` and the `--fault-*` flags. |
| `otlp` | ❌ | Pulls `opentelemetry-otlp` (gRPC) so `--exporter otlp` works. Selects no TLS provider on its own. |
| `otlp_rustls` | ✅ | Adds rustls with the `aws-lc-rs` provider and bundled webpki roots to the OTLP transport, so `https://` collectors work with no OpenSSL. Inert unless `otlp` is also enabled, which is why it can default on without dragging the gRPC stack into every build. |

`otlp` and `otlp_rustls` are split so enabling the exporter does not lock the
build into a TLS stack. `--features otlp` gets rustls/`aws-lc-rs` for free; an app
that wants a different provider drops the default and names its own:

```bash
cargo build -p azure_data_cosmos_observability_harness \
  --no-default-features \
  --features "metrics,distributed_tracing,fault_injection,otlp,opentelemetry-otlp/tls-ring"
```

The feature names intentionally mirror the SDK's own feature names so the harness
compiles the exact code path it is validating. `key_auth` is always enabled so the
harness can talk to the emulator with the well-known key.

## Prerequisites

Run against either:

- The **Cosmos DB emulator** (recommended — real latency and RU flow). Start it
  with AAD enabled if you plan to use `--auth aad`; key auth works out of the box.
  See [how to develop with the emulator][emulator-docs].
- A **real account** (endpoint + key, or Entra ID).

## Quick start (emulator + stdout)

With the emulator listening on `https://localhost:8081`:

```sh
# From the repo root. Defaults target the emulator with its well-known key,
# auto-relaxing TLS validation for localhost.
cargo run -p azure_data_cosmos_observability_harness -- \
  --duration-secs 60 --concurrency 8 --rps 200
```

You'll see the metrics handler's `db.client.operation.duration` histogram printed
to stdout on each export interval, and — because steady-state reads succeed —
almost no spans or sampled logs. Inject some faults to see the rich path:

```sh
cargo run -p azure_data_cosmos_observability_harness -- \
  --duration-secs 120 --concurrency 8 --rps 200 \
  --fault-probability 0.1 --fault-error too-many-requests --fault-delay-ms 50
```

Now ~10% of matching operations fail/slow down. The sampling-log and tracing
handlers are rate-limited (a bounded number of emissions per interval — roughly
~100/min each by default), so you see a **bounded, sampled subset** of those
failures: each emitted failure surfaces a sampled diagnostics line (a `WARN` on
the `azure_data_cosmos::diagnostics::sampled` target) plus a backdated span tree,
while the rest are suppressed and summarized so an error storm stays bounded.

## Pointing at an OpenTelemetry collector (OTLP)

Build with the `otlp` feature and point at a collector's gRPC endpoint
(`4317` by default). The exporter uses rustls (no OpenSSL), so an `https://`
endpoint is also supported:

```sh
cargo run -p azure_data_cosmos_observability_harness --features otlp -- \
  --exporter otlp --otlp-endpoint http://localhost:4317 \
  --duration-secs 3600 --concurrency 16 --rps 500 \
  --fault-probability 0.02
```

A minimal local collector (`otel-collector-config.yaml`) that logs everything:

```yaml
receivers:
  otlp:
    protocols:
      grpc:
        endpoint: 0.0.0.0:4317
exporters:
  debug:
    verbosity: detailed
service:
  pipelines:
    metrics: { receivers: [otlp], exporters: [debug] }
    traces:  { receivers: [otlp], exporters: [debug] }
```

```sh
docker run --rm -p 4317:4317 \
  -v ${PWD}/otel-collector-config.yaml:/etc/otelcol/config.yaml \
  otel/opentelemetry-collector:latest
```

Swap the `debug` exporter for `prometheus`/`otlphttp`/Jaeger to forward the
signals to your backend of choice.

## Sampled logs

Sampled diagnostics lines and harness progress go through `tracing`. The default
filter stays quiet at steady state while still surfacing failures. Override it
with `RUST_LOG`, e.g. to see every sampled line and suppression notice:

```sh
RUST_LOG="azure_data_cosmos::diagnostics=debug,azure_data_cosmos_observability_harness=info" \
  cargo run -p azure_data_cosmos_observability_harness -- --fault-probability 0.5
```

## Configuration

Every flag has an environment fallback where noted. Run `--help` for the full,
authoritative list.

### Target account

| Flag | Env | Default | Description |
| --- | --- | --- | --- |
| `--endpoint` | `AZURE_COSMOS_ENDPOINT` | `https://localhost:8081` | Account endpoint. |
| `--key` | `AZURE_COSMOS_KEY` | emulator key | Account key (key auth). |
| `--connection-string` | `AZURE_COSMOS_CONNECTION_STRING` | — | `AccountEndpoint=...;AccountKey=...;`. The literal `emulator` expands to the local emulator. Overrides `--endpoint`/`--key`. |
| `--auth` | — | `key` | `key`, `aad` (Entra ID via the developer-tools credential chain), or `workload-identity` (Entra ID via a projected Kubernetes service account token). |
| `--region` | `AZURE_COSMOS_REGION` | `West US` | Application region for proximity routing. |
| `--emulator` | — | auto | Relax TLS validation; auto-enabled for `localhost`/`127.0.0.1`. For a custom (non-local) emulator host it exports `AZURE_COSMOS_EMULATOR_HOST` so the SDK relaxes validation for that host. |
| `--database` / `--container` | — | `observability_soak` / `items` | Created if missing. |
| `--throughput` | — | `400` | RU/s used when the container is created. |

### Workload

| Flag | Default | Description |
| --- | --- | --- |
| `--concurrency` | `8` | Number of concurrent worker tasks. |
| `--rps` | `0` (max) | Target aggregate requests/sec; `0` runs closed-loop. |
| `--duration-secs` | `0` (∞) | Run length; `0` runs until Ctrl+C. |
| `--seed-count` | `100` | Documents seeded before the loop starts. |
| `--read-weight` / `--write-weight` / `--query-weight` | `70` / `20` / `10` | Relative operation mix. |
| `--report-interval-secs` | `10` | Console progress interval. |

### Telemetry

| Flag | Default | Description |
| --- | --- | --- |
| `--exporter` | `stdout` | `stdout`, `otlp` (needs `--features otlp`), or `none`. |
| `--otlp-endpoint` | `http://localhost:4317` | OTLP/gRPC collector endpoint. |
| `--metric-export-interval-secs` | `15` | Metric export cadence. |
| `--extended-metrics` | off | Emit request-charge, returned-rows and active-instance metrics, plus the extended attribute set. |

### Fault injection *(requires the `fault_injection` feature)*

| Flag | Default | Description |
| --- | --- | --- |
| `--fault-probability` | `0.0` | Probability (0–1) a matching request is faulted; `0` disables. |
| `--fault-delay-ms` | `0` | Extra server-side delay applied to faulted requests (produces slow ops). |
| `--fault-error` | `service-unavailable` | `service-unavailable`, `too-many-requests`, `internal-server-error`, `timeout`, `retry-with`, `connection-error`. |
| `--fault-operation` | `all` | `all`, `read`, `write`, or `query`. |
| `--fault-start-secs` | `0` | Seconds from the start of the load loop before faults activate; `0` faults from the first op. Setup/seeding always run fault-free. Requires `--fault-probability > 0`. |
| `--fault-duration-secs` | `0` | How long the fault window stays active once it starts; `0` leaves faults active for the rest of the run. Requires `--fault-probability > 0`. |

## Running it continuously

Everything above drives the harness by hand, which is what you want while
developing or diagnosing. Running it *unattended* for months — so that latency,
RU and error-rate regressions show up as a trend line rather than being noticed
by a customer — needs infrastructure that has nothing to do with the SDK: a
Kubernetes cluster, a collector, a metrics store, a Grafana workspace, alert
rules and a real Cosmos account.

That lives in the Cosmos DB team's internal tooling repo rather than here, along
with a local `docker compose` stack and the Grafana dashboard, so this repo keeps
only the crate itself. The `workload-identity` auth method exists for that
deployment: it is what lets the harness authenticate from inside a container
without a key.

## Notes

- The exporters are flushed and shut down on exit (Ctrl+C, duration elapse, or
  error), so the final metric/span batch is emitted before the process ends.
- `cargo test --all-features` and
  `cargo clippy --all-features --all-targets` both work for this crate on
  Windows, and are worth running before pushing: CI's `Build Analyze` job runs
  the equivalent, and the extra features it turns on are enough to move
  size-sensitive lints such as `clippy::large_futures` past their thresholds.
  Keep those commands scoped with `-p azure_data_cosmos_observability_harness`
  though — a workspace-wide `--all-features` also enables
  `azure_data_cosmos/hmac_openssl`, which fails to build on Windows hosts
  without OpenSSL.

[emulator-docs]: https://learn.microsoft.com/azure/cosmos-db/how-to-develop-emulator

<!--
Copyright (c) Microsoft Corporation. All rights reserved.
Licensed under the MIT License.
-->

# Cosmos Rust SDK — client observability dashboards

`cosmos-observability.json` is a Grafana dashboard for the client-side
OpenTelemetry metrics emitted by the Azure Cosmos DB Rust SDK observability layer
(WS9 / [PR #4789](https://github.com/Azure/azure-sdk-for-rust/pull/4789)). It
targets a **Prometheus / OTLP-metrics** datasource and uses the real metric and
attribute names the SDK emits.

For the full walkthrough — running the soak harness, standing up a local
Collector + Prometheus + Grafana stack, and interpreting the panels — see the
[WS9 soak runbook](../docs/WS9_SOAK_RUNBOOK.md).

## Importing

- **Automatic (recommended):** `docker compose up -d` in
  [`../observability`](../observability) provisions the Prometheus datasource and
  auto-loads this dashboard (folder *Cosmos WS9*).
- **Manual:** in Grafana, *Dashboards -> New -> Import*, upload
  `cosmos-observability.json`, and pick your Prometheus datasource when prompted.

The dashboard has no hard-coded datasource: it exposes a `datasource` template
variable, so it binds to whichever Prometheus datasource you select.

## Template variables

- `datasource` — the Prometheus/OTLP datasource to query.
- `operation` — filter by `db.operation.name` (multi-select, defaults to All).
- `region` — filter by `azure.cosmosdb.operation.contacted_regions`. Populated
  only when the SDK is configured with extended metric attributes
  (`MetricsOptions::with_extended_attributes(true)`); leave on All otherwise.

## Metrics -> panels

The SDK emits OpenTelemetry instruments; the Collector's Prometheus exporter
normalizes their names (dots become `_`, the seconds histogram gains a `_seconds`
suffix, `{request_unit}` / `{row}` annotation units are dropped).

| SDK instrument (OTel) | Prometheus series | Stability | Panels |
| --- | --- | --- | --- |
| `db.client.operation.duration` (s) | `db_client_operation_duration_seconds_{bucket,sum,count}` | stable, always-on | Latency quantiles, throughput, error rate |
| `azure.cosmosdb.client.operation.request_charge` | `azure_cosmosdb_client_operation_request_charge_{bucket,sum,count}` | dev, opt-in | RU heatmap + RU quantiles + RU/sec |
| `db.client.response.returned_rows` | `db_client_response_returned_rows_{bucket,sum,count}` | dev, opt-in | Returned rows P95 |
| `azure.cosmosdb.client.active_instance.count` | `azure_cosmosdb_client_active_instance_count` | dev | Active client instances *(deferred — not emitted yet)* |

Always-on metric attributes (become Prometheus labels): `db_operation_name`,
`db_response_status_code`, `db_collection_name`, `db_namespace`, `error_type`,
`server_address`, `db_system_name`. Extended (opt-in) attributes:
`azure_cosmosdb_consistency_level`, `azure_cosmosdb_operation_contacted_regions`,
`azure_cosmosdb_response_sub_status_code`, `azure_cosmosdb_connection_mode`.

> The opt-in metrics/attributes are off by default (design decision D7), so a few
> panels read *No data* until you enable them on the SDK's `MetricsOptions`. The
> stable duration metric alone drives latency, throughput, and error-rate panels.

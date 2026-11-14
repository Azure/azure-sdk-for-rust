# Export Event Hubs telemetry

The [example](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/eventhubs/azure_messaging_eventhubs/examples/eventhubs_opentelemetry.rs) exports SDK traces, diagnostic logs,
and native metrics to an OpenTelemetry Collector over OTLP/HTTP. It configures
providers at application startup and shuts them down even when the SDK call fails.

From the repository root, start a collector with the supplied configuration. This
command uses the collector image used to validate the example:

```sh
docker run --rm --name eventhubs-otel -p 14318:4318 \
  -v "$PWD/sdk/eventhubs/azure_messaging_eventhubs/examples/opentelemetry/collector.yaml:/etc/otelcol/config.yaml:ro" \
  otel/opentelemetry-collector:0.109.0
```

In another terminal, run the no-Azure smoke mode:

```sh
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:14318 \
  cargo run -p azure_messaging_eventhubs --features metrics \
  --example eventhubs_opentelemetry
```

This mode attempts an SDK connection to local port zero, which must fail. It needs
no Azure credentials and exports a failed `EventHubs.connection.open` span,
structured AMQP lifecycle logs, and an `az.eventhubs.amqp.lifecycle` metric. It does
not send events or generate successful-send metrics. The collector's debug output
should show all three signals with `service.name=eventhubs-rust-example`.

To send an event to Azure instead, authenticate for `DeveloperToolsCredential`,
set `EVENTHUBS_HOST` to the fully qualified namespace and `EVENTHUB_NAME` to the hub,
and add `-- --live`:

```sh
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:14318 \
  cargo run -p azure_messaging_eventhubs --features metrics \
  --example eventhubs_opentelemetry -- --live
```

Live mode writes one event and requires Event Hubs send permission. It additionally
emits a logical producer span and operation, event, and duration metrics. Receivers
and event processors use the same telemetry setup through the consumer builder's
`with_tracing_provider` method. No message context is injected or extracted.

The sample filters bridged logs to the Event Hubs crate, preventing exporter logs
from recursively exporting themselves. Stop the collector with Ctrl+C after use.

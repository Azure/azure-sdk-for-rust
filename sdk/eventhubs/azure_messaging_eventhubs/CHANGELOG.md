# Release History

## 0.15.0 (Unreleased)

### Features Added

- Added connection-string authentication. `ProducerClientBuilder` and `ConsumerClientBuilder` now have an `open_with_connection_string` method that authenticates with a Shared Access Signature parsed from an Event Hubs connection string (`Endpoint=sb://...;SharedAccessKeyName=...;SharedAccessKey=...`, optionally with `EntityPath`, or a pre-formed `SharedAccessSignature`). The connection-string parser is exposed publicly as `ConnectionString`. This reaches parity with the other Azure SDKs for development and test scenarios; Microsoft Entra ID via `open` with a `TokenCredential` remains the recommended path for production. The parser rejects empty required values and empty Event Hub names up front, and a pre-formed `SharedAccessSignature` reports its own `se` as the token expiry (rather than a rolling client-side window); because such a token cannot be renewed, the connection's token refresher detects the non-advancing expiry and leaves the broker to enforce it. ([#3459](https://github.com/Azure/azure-sdk-for-rust/issues/3459))
- The `EventProcessor` now opens every partition receiver with AMQP epoch (owner level) `0` and surfaces broker-initiated displacement as the new `EventHubsError::ConsumerDisconnected` error kind. When a second `EventProcessor` instance claims a partition this instance is currently holding, the broker disconnects this instance's receiver and the consumer's `stream_events()` resolves with `ConsumerDisconnected`. This matches the behavior of `EventProcessorClient` in the .NET and Java Azure SDKs. Consumers should pattern-match on `ErrorKind::ConsumerDisconnected` to detect a stolen partition and re-acquire a client via `next_partition_client()`.
- Added `EventHubsError::ConsumerDisconnected(Option<AmqpDescribedError>)` error variant.

### Breaking Changes

- On the receive path, the `amqp:link:stolen` AMQP condition is no longer auto-retried. A receiver displaced by a higher-or-equal-epoch attacher now surfaces the error (translated to `EventHubsError::ConsumerDisconnected` by `EventReceiver::stream_events`) instead of silently re-attaching. Sender, CBS, and management operations retain the historical retry-on-stolen behavior.

### Bugs Fixed

- Increased `DEFAULT_PARTITION_EXPIRATION_DURATION` from 10 seconds to 60 seconds. The previous default was shorter than `DEFAULT_UPDATE_INTERVAL` (30 seconds), so ownership records expired between load-balancing cycles. The load balancer perpetually saw `current=0` for every consumer and continuously re-claimed partitions, causing widespread duplicate event processing. `EventProcessorBuilder::build` now rejects configurations where `partition_expiration_duration <= update_interval`. ([#3851](https://github.com/Azure/azure-sdk-for-rust/issues/3851))
- A partition stolen by a higher-or-equal-epoch attacher now surfaces as `ErrorKind::ConsumerDisconnected` when the broker reports `amqp:link:stolen` on a re-attach, not only on an in-flight receive. Other attach failures inside the receive loop now classify by their own kind. The wrapper reported all of them as a message error, which the retry decider treated as non-retryable.
- The `EventProcessor`'s load-balancer reconciliation now closes the underlying AMQP receiver for any partition that has been reassigned to another consumer, so the consumer's `stream_events()` resolves and the loop can terminate. Previously a stolen partition's client could continue to attempt receives until the broker tore down the link.

### Other Changes

- Reduced lock contention when a single `ProducerClient` or `ConsumerClient` is shared across threads. The per-path sender, session, and receiver caches no longer serialize on a connection-wide lock: each partition's link attach runs without holding the shared lock, so the partitions on a shared client set up and recover concurrently instead of one at a time, and steady-state sends no longer queue behind an unrelated partition's attach.
- Added `tracing` span instrumentation and structured-field logging across the connection, producer, consumer, event-processor, and checkpoint paths. Lifecycle events (connection open/close, reconnect outcome, link attach, partition ownership claim/revoke) and failure conditions (receive errors, link-stolen, send/batch rejections, unauthorized fast-fail, token-refresh failures) are now visible at the default `info`/`warn` levels, with diagnostic values attached as structured fields (`partition_id`, `connection_id`, `source_url`, and similar) following a documented level policy. Per-message hot paths stay at `trace`. Credentials are never logged, and event payloads are redacted by `SafeDebug` unless the `azure_core` `debug` feature is enabled. See the README for details and a subscriber example. ([#4592](https://github.com/Azure/azure-sdk-for-rust/issues/4592))

## 0.14.0 (Unreleased)

### Features Added

### Breaking Changes

### Bugs Fixed

### Other Changes

- Updated dependencies.

## 0.13.0 (2026-04-14)

### Other Changes

- Updated dependencies.

## 0.12.0 (2026-03-10)

### Breaking Changes

- Support for `wasm32-unknown-unknown` has been removed ([#3377](https://github.com/Azure/azure-sdk-for-rust/issues/3377))

### Other Changes

- Updated dependencies.

## 0.11.0 (2026-02-11)

### Breaking Changes

- Changed our minimum supported Rust version (MSRV) from 1.85 to 1.88.

### Other Changes

- Updated dependencies.

## 0.10.0 (2026-02-10)

### Other Changes

- Updated dependencies.

## 0.9.0 (2025-11-11)

### Breaking Changes

- All Event Hubs APIs now return an `azure_messaging_eventhubs::EventHubError` instead of an `azure_core::Error`.
- Several `azure_messaging_eventhubs::error::ErrorKind` enumerations have been removed because they are no longer needed.
- Modified several errors which previously used `azure_core::Error::with_message` to use `azure_messaging_eventhubs::EventHubsError::with_message`, changing their underlying type.

### Other Changes

- Added `azure_messaging_eventhubs::error::ErrorKind::AmqpError`, `azure_messaging_eventhubs::error::ErrorKind::AzureCore`, and `azure_messaging_eventhubs::error::ErrorKind::SimpleMessage` to describe AMQP originated messages, Azure Core originated messages, and messages which just have a string value respectively.
- Added `azure_messaging_eventhubs::Error::with_message` to enable simple error returns with a text message.

## 0.8.0 (2025-10-08)

### Breaking Changes

- `EventProcessor` now consumes its `ConsumerClient` parameter rather than accepting a clone of an `Arc`.

### Other Changes

- Internal refactoring to ensure that the `close()` method on various clients works as expected.

## 0.7.0 (2025-09-16)

### Features Added

- Reconnect support for EventHubs operations.
- Enable [Geo Replication](https://learn.microsoft.com/azure/event-hubs/geo-replication) support in Event Hubs consumers and producers.

### Breaking Changes

- `ProducerClient::send_batch` now consumes its `batch` argument.
- `RetryOptions::max_retries` is a `u32` not a `usize`.

## 0.6.0 (2025-08-05)

### Other Changes

- Updated dependencies.

## 0.5.0 (2025-07-11)

### Breaking Changes

- Minimum supported Rust version (MSRV) is now 1.85.
- Converted all `time::Duration` types to `azure_core::time::Duration`

## 0.4.0 (2025-06-10)

### Breaking Changes

- Event Message Properties is now an `AmqpSimpleValue` rather than an `AmqpValue` to more closely reflect the AMQP specification (AMQP ApplicationProperties cannot contain Map, List, or Array).

### Other Changes

- Added performance tests for some Event Hubs APIs
- Use `SafeDebug` attribute for AMQP messages
- Test infrastructure is now deployed using a `test-resources.bicep` file instead of a `test-resources.json` file.
- Restructured internal logic to prepare for connection/session recovery.

## 0.3.0 (2025-05-07)

### Features Added

- Added EventHubs token refresh.
- Added retries for failed EventHubs operations.

### Breaking Changes

- The `fully_qualified_domain` parameter to the `open` method on the `ProducerClient` and `ConsumerClient` builder now takes an `&str` instead of a `String` to better follow the Azure SDK for Rust API guidelines.

## 0.2.0 (2025-04-08)

### Features Added

- Added initial support for an EventHubs processor.

  Note that as currently implemented, the processor is not very functional, since it requires that the customer provide an instance of a checkpoint store.

  For people who wish to play with the checkpoint store, there is an `InMemoryCheckpointStore` created (under the "test" feature) which can be used to experiment with the EventHubs processor.

- Removed the requirement that streaming messages from the `stream_events` method on the `EventReceiver` use `pin_mut!()` on the provided stream.
- Removed direct dependencies on `tokio` package.
- Added `partition_id` option to `SendMessageOptions`.
- Significant modifications to API surface to improve conformance to Azure RUST guidelines e.g., APIs which take ownership of a string consume `String` parameter instead of borrowing a `&str` parameter.

### Breaking Changes

- The stream returned by the `stream_events` API needs to be declared as mutable.
- APIs which used to return `Option<String>`, and `Option<Vec<T>>` now return `Option<&str>`, and `Option<&[T]>`.
- APIs which take ownership of string parameters now take a `String` parameter instead of a `&str` parameter.

### Bugs Fixed

- If you call `send_event` or `send_message` with a specific target partition, the call now respects the desired target partition.

## 0.1.0 (2025-02-18)

### Features Added

- Initial supported release.

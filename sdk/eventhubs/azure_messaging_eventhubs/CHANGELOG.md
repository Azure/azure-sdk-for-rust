# Release History

## 0.15.0 (Unreleased)

### Features Added

- Added `BufferedProducerClient`, a producer that accepts single events and publishes them in the background. One worker owns each partition, so events keep their enqueued order and a slow partition does not stop the others. A successful enqueue means only that the local buffer accepted the event; the client reports each terminal outcome through the handlers that `with_on_send_succeeded` and `with_on_send_failed` register, and a handler for failed batches is required. The client sends a batch when the next event does not fit, when the batch reaches the configured event count, when the maximum wait time expires, on `flush`, and on `close`. The defaults are a 1 second maximum wait time and 1500 buffered events for each partition, matching the .NET, JavaScript, and Python clients. `flush` sets a barrier over the events accepted before the call, `close` sends the buffered events, and `abort` abandons them. The client reads the partition list one time, when it opens, so a partition that the service adds later stays unused until the application opens a new client. New public types: `BufferedProducerClient`, `BufferedProducerClientBuilder`, `EnqueueEventOptions`, `SendBatchSucceededContext`, and `SendBatchFailedContext`. ([#4873](https://github.com/Azure/azure-sdk-for-rust/pull/4873))
- Added the `ErrorKind::SendNotAccepted` error variant. The buffered producer reports an AMQP `Modified` or `Released` outcome as a delivery failure, because neither outcome means that the service durably stored the events. `ProducerClient::send_batch` keeps its historical behavior and treats both as success with a warning.
- Added connection-string authentication. `ProducerClientBuilder` and `ConsumerClientBuilder` now have an `open_with_connection_string` method that authenticates with a Shared Access Signature parsed from an Event Hubs connection string (`Endpoint=sb://...;SharedAccessKeyName=...;SharedAccessKey=...`, optionally with `EntityPath`, or a pre-formed `SharedAccessSignature`). The connection-string parser is exposed publicly as `ConnectionString`. This reaches parity with the other Azure SDKs for development and test scenarios; Microsoft Entra ID via `open` with a `TokenCredential` remains the recommended path for production. The parser rejects empty required values and empty Event Hub names up front, and a pre-formed `SharedAccessSignature` reports its own `se` as the token expiry (rather than a rolling client-side window); because such a token cannot be renewed, the connection's token refresher detects the non-advancing expiry and leaves the broker to enforce it. ([#3459](https://github.com/Azure/azure-sdk-for-rust/issues/3459))
- Added a `with_transport` builder method on `ProducerClient` and `ConsumerClient`, which takes the `AmqpTransport` of `azure_core_amqp` (re-exported as `models::AmqpTransport`). `AmqpTransport::WebSocket` tunnels AMQP over secure WebSockets (`wss://`, port 443), allowing clients to connect from networks that block the native AMQP ports (5671/5672). This matches the transport option offered by the .NET, Java, and Python Azure SDKs. The `EventProcessor` inherits the transport from the `ConsumerClient` passed to `build`, so it runs over WebSockets when that client selects them. ([#3601](https://github.com/Azure/azure-sdk-for-rust/issues/3601))
- Added the `fe2o3_amqp`, `fe2o3_amqp_rustls`, `fe2o3_amqp_ws`, and `fe2o3_amqp_ws_rustls` features, which forward the matching features of `azure_core_amqp`. The `default` feature selects the AMQP backend and the rustls stack with the aws-lc-rs provider for both the TCP and the WebSocket transport. That is the stack that the rest of `sdk/core` uses. See Breaking Changes for the effect on the TCP transport, which ran on native-tls before. To build on another stack, turn off the default features, name the base features, and take a direct dependency on `fe2o3-amqp` and `fe2o3-amqp-ws` with the stack you want; Cargo unifies the features. `samples/list_blobs_native_tls` shows the same pattern for `reqwest`.
- The `EventProcessor` now opens every partition receiver with AMQP epoch (owner level) `0` and surfaces broker-initiated displacement as the new `EventHubsError::ConsumerDisconnected` error kind. When a second `EventProcessor` instance claims a partition this instance is currently holding, the broker disconnects this instance's receiver and the consumer's `stream_events()` resolves with `ConsumerDisconnected`. This matches the behavior of `EventProcessorClient` in the .NET and Java Azure SDKs. Consumers should pattern-match on `ErrorKind::ConsumerDisconnected` to detect a stolen partition and re-acquire a client via `next_partition_client()`.
- Added `EventHubsError::ConsumerDisconnected(Option<AmqpDescribedError>)` error variant.
- Added the `ErrorKind::InvalidBatchSize { requested, max_allowed }` error variant. `create_batch` reports it when `EventDataBatchOptions::max_size_in_bytes` is zero or is larger than the maximum the sender link allows, so a caller can branch on the kind instead of the message. This matches the `ArgumentOutOfRangeException` that .NET raises and the typed error that Go returns for the same input.
- Added the `ErrorKind::MissingCheckpointMetadata { partition_id }` error variant. `PartitionClient::update_checkpoint` reports it when the event carries no offset and no sequence number, so a caller can branch on the kind instead of the message. This matches the `InvalidOperationException` that .NET raises for the same input.

### Breaking Changes

- The `default` feature now selects `fe2o3_amqp_rustls`, so AMQP framed directly on TCP (`amqps://`, port 5671) runs on rustls with the aws-lc-rs provider where it ran on native-tls. Both stacks read the trust store of the operating system, so a namespace behind a private or an enterprise certificate authority keeps working. The stacks read that store through different platform APIs, and a deployment that tunes native-tls directly, such as one that sets OpenSSL environment variables, can still see a difference. To keep native-tls, turn off the default features, name `fe2o3_amqp`, and take a direct dependency on `fe2o3-amqp` with its `native-tls` feature. ([#4189](https://github.com/Azure/azure-sdk-for-rust/issues/4189))
- On the receive path, the `amqp:link:stolen` AMQP condition is no longer auto-retried. A receiver displaced by a higher-or-equal-epoch attacher now surfaces the error (translated to `EventHubsError::ConsumerDisconnected` by `EventReceiver::stream_events`) instead of silently re-attaching. Sender, CBS, and management operations retain the historical retry-on-stolen behavior.
- `PartitionClient::update_checkpoint` now returns an error when the event carries no offset and no sequence number. Such a call returned `Ok(())` and recorded no position before.

### Bugs Fixed

- `ProducerClient::close` now stops the authorization refresh task, so repeated producer life cycles release task-held memory. ([#4595](https://github.com/Azure/azure-sdk-for-rust/issues/4595))
- `ConsumerClient::close` and `ProducerClient::close` now close the connection when another object still holds it, most often an `EventReceiver` that the caller has not dropped. Both methods used to report an error and leave the connection open. ([#4931](https://github.com/Azure/azure-sdk-for-rust/issues/4931))
- A handle that outlives the client it came from now reports that the client is closed on its next call. Such a handle opened a second connection to the service before. ([#4931](https://github.com/Azure/azure-sdk-for-rust/issues/4931))
- `EventProcessor::close` now continues past a partition client that the application still holds. It used to stop there, which left the partition clients behind it open and skipped the close of the consumer client. ([#4931](https://github.com/Azure/azure-sdk-for-rust/issues/4931))
- Claims-based-security authorizations for one connection now run in sequence. The service permits one `$cbs` link for each connection, so a client that attached more than one link at once could fail with `NotAllowed`.
- `EventDataBatchOptions::max_size_in_bytes` now takes effect. A batch keeps the requested size, and `create_batch` reports an error when the request is zero or is larger than the sender link allows.
- Increased `DEFAULT_PARTITION_EXPIRATION_DURATION` from 10 seconds to 60 seconds. The previous default was shorter than `DEFAULT_UPDATE_INTERVAL` (30 seconds), so ownership records expired between load-balancing cycles. The load balancer perpetually saw `current=0` for every consumer and continuously re-claimed partitions, causing widespread duplicate event processing. `EventProcessorBuilder::build` now rejects configurations where `partition_expiration_duration <= update_interval`. ([#3851](https://github.com/Azure/azure-sdk-for-rust/issues/3851))
- A partition stolen by a higher-or-equal-epoch attacher now surfaces as `ErrorKind::ConsumerDisconnected` when the broker reports `amqp:link:stolen` on a re-attach, not only on an in-flight receive. Other attach failures inside the receive loop now classify by their own kind. The wrapper reported all of them as a message error, which the retry decider treated as non-retryable.
- The `EventProcessor`'s load-balancer reconciliation now closes the underlying AMQP receiver for any partition that has been reassigned to another consumer, so the consumer's `stream_events()` resolves and the loop can terminate. Previously a stolen partition's client could continue to attempt receives until the broker tore down the link.
- Fixed a deadlock when a CBS failure during management-client creation started connection recovery. ([#4728](https://github.com/Azure/azure-sdk-for-rust/issues/4728))
- Closed a stale-resource window in connection recovery. A `ReconnectConnection` recovery that fired while a slow-path attach (authorize, session begin, or sender/receiver link attach) was in flight could cache a resource bound to the just-dropped connection; the next operation on that resource failed (unauthorized / detached / closed) and triggered a second, redundant recovery cycle. A recovery generation counter now tags each cached resource, and a slow path that completes across a recovery discards its result and re-attaches against the new connection instead of caching the stale one. The authorizer's token cache is mutable (a background task refreshes tokens) so it cannot use the same one-shot cell as the connection caches; both of its writers, `authorize_path` and the refresh task, instead re-check the generation under the same lock that recovery's clear takes, and a recovery brackets its invalidation with a generation bump on each side, which leaves the counter odd for as long as the recovery runs, so a slow path that overlaps a recovery at either end also discards rather than caching a resource bound to the connection that recovery is dropping. A token refresh pass that a recovery discards now applies the same backoff floor as a failed pass, so a recovery storm cannot turn the refresh loop into an uncapped stream of credential and CBS calls. The per-path / per-partition concurrency is preserved. ([#4454](https://github.com/Azure/azure-sdk-for-rust/issues/4454))
- The error that a receive timeout produces now carries its cause unboxed, so `downcast_ref::<std::io::Error>()` returns the `std::io::Error` with `ErrorKind::TimedOut`. The cause was boxed twice, which stored a `Box<std::io::Error>` and made every downcast to `std::io::Error` return `None`. ([#5098](https://github.com/Azure/azure-sdk-for-rust/issues/5098))
- `InMemoryCheckpointStore` now rotates the ETag and refreshes `last_modified_time` when an existing ownership is renewed, matching the create path and the production `BlobCheckpointStore`. Previously the renewal path reinserted the caller's record verbatim, leaving a stale ETag and timestamp; that divergence from the real store could mask bugs in code that relies on ETag rotation for optimistic concurrency. ([#4594](https://github.com/Azure/azure-sdk-for-rust/issues/4594))
- `PartitionClient::update_checkpoint` no longer reports success without writing a usable checkpoint. It wrote nothing when the event had no message annotations, and it wrote an empty checkpoint when the annotations held no position. An empty checkpoint suppressed the per-partition start position the caller configured. It also erased a good checkpoint in `BlobCheckpointStore`, because that store rewrites the whole blob metadata on each update. ([#5097](https://github.com/Azure/azure-sdk-for-rust/issues/5097))

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

# Release History

## 0.9.0 (Unreleased)

### Features Added

### Breaking Changes

- All Event Hubs APIs now return an `azure_messaging_eventhubs::EventHubError` instead of an `azure_core::Error`.
- Several `azure_messaging_eventhubs::error::ErrorKind` enumerations have been removed because they are no longer needed.
- Modified several errors which previously used `azure_core::Error::with_message` to use `azure_messaging_eventhubs::EventHubsError::with_message`, changing their underlying type.

### Bugs Fixed

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
